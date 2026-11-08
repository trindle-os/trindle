//! The Inhibit portal: a program, such as a video player, asking that the
//! screen not dim, lock or sleep while it is busy, and one watching whether
//! the screensaver or lock screen is up.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use zbus::zvariant::{OwnedFd, OwnedObjectPath, OwnedValue, Value};

use super::portal::Stay;
use super::portal_ask::{get, Options, REQUEST, REQUEST_XML};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::t;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Inhibit";
const SESSION: &str = "org.freedesktop.impl.portal.Session";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Inhibit">
    <method name="Inhibit">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="window" direction="in"/>
      <arg type="u" name="flags" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
    </method>
    <method name="CreateMonitor">
      <arg type="o" name="handle" direction="in"/>
      <arg type="o" name="session_handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="window" direction="in"/>
      <arg type="u" name="response" direction="out"/>
    </method>
    <signal name="StateChanged">
      <arg type="o" name="session_handle" direction="out"/>
      <arg type="a{sv}" name="state" direction="out"/>
    </signal>
    <method name="QueryEndResponse">
      <arg type="o" name="session_handle" direction="in"/>
    </method>
  </interface>"#;
const SESSION_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Session">
    <method name="Close"/>
    <signal name="Closed"/>
    <property name="version" type="u" access="read"/>
  </interface>"#;

const SUSPEND: u32 = 4;
const IDLE: u32 = 8;

const RUNNING: u32 = 1;

const LOOK: Duration = Duration::from_secs(2);

pub(super) type Awake = Rc<dyn Fn(&str, bool)>;

pub(super) fn awake() -> Awake {
    Rc::new(|tag, on| if on { crate::session::idle::hold_awake(tag) } else { crate::session::idle::release_awake(tag) })
}

struct Hold {
    tag: String,
    awake: bool,

    sleep: Option<OwnedFd>,
    close: Option<Served>,
}

struct Watch {
    served: Option<Served>,
}

struct State {
    bus: Bus,
    events: Loop,
    stay: Rc<Stay>,
    awake: Awake,

    system: Option<RefCell<Option<Bus>>>,
    holds: RefCell<HashMap<String, Hold>>,
    watches: RefCell<HashMap<String, Watch>>,

    resting: Cell<bool>,
    looking: Cell<bool>,
    next: Cell<u32>,
}

impl State {
    fn hold(&self) {
        self.stay.busy.set(self.stay.busy.get() + 1);
    }

    fn release(&self) {
        self.stay.busy.set(self.stay.busy.get().saturating_sub(1));
        self.stay.last.set(Instant::now());
    }

    fn system(&self) -> Option<Bus> {
        let held = self.system.as_ref()?;
        if held.borrow().is_none() {
            *held.borrow_mut() = Bus::system(&self.events).ok();
        }
        held.borrow().clone()
    }
}

fn let_go(state: &Rc<State>, handle: &str) {
    let Some(mut hold) = state.holds.borrow_mut().remove(handle) else { return };
    if hold.awake {
        (state.awake)(&hold.tag, false);
    }
    hold.sleep = None;
    let close = hold.close.take();
    state.release();

    state.events.after(Duration::ZERO, move || drop(close));
}

fn inhibit(state: &Rc<State>, asked: Asked) {
    let Ok((handle, app_id, _window, flags, options)) =
        asked.message().body().deserialize::<(OwnedObjectPath, String, String, u32, Options)>()
    else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "Inhibit takes a handle, an app, a window, flags and options");
    };
    asked.reply(&());
    let handle = handle.as_str().to_string();
    if state.holds.borrow().contains_key(&handle) {
        return;
    }
    let n = state.next.get();
    state.next.set(n + 1);
    let tag = format!("portal{n}");
    let awake = flags & IDLE != 0;
    if awake {
        (state.awake)(&tag, true);
    }
    state.hold();
    let close = {
        let (weak, h) = (Rc::downgrade(state), handle.clone());
        state.bus.serve(&handle, Offer {
            name: REQUEST.into(),
            xml: REQUEST_XML.into(),
            call: Box::new(move |asked: Asked| {
                asked.reply(&());
                if let Some(state) = weak.upgrade() {
                    let_go(&state, &h);
                }
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok()
    };
    state.holds.borrow_mut().insert(handle.clone(), Hold { tag, awake, sleep: None, close });
    if flags & SUSPEND == 0 {
        return;
    }
    let Some(system) = state.system() else { return };
    let reason = get::<String>(&options, "reason").unwrap_or_default();
    let who = if app_id.is_empty() { t!("screenshot-inhibit-who").to_string() } else { app_id };
    let weak = Rc::downgrade(state);
    system.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
        .method("Inhibit", &("sleep", who.as_str(), reason.as_str(), "block"), false, Some(Duration::from_secs(5)), &state.events, move |r| {
            let Some(fd) = r.ok().and_then(|m| m.body().deserialize::<OwnedFd>().ok()) else { return };
            let Some(state) = weak.upgrade() else { return };

            let mut holds = state.holds.borrow_mut();
            if let Some(hold) = holds.get_mut(&handle) {
                hold.sleep = Some(fd);
            }
        });
}

fn resting() -> bool {
    let scratch = crate::common::files::scratch();
    let saver = std::fs::read_to_string(scratch.join("trindle-screensaver.pid")).ok()
        .and_then(|p| p.trim().parse::<u32>().ok())
        .is_some_and(|p| std::fs::read_to_string(format!("/proc/{p}/cmdline")).is_ok_and(|c| c.contains("screensaver")));
    saver || locked()
}

fn locked() -> bool {
    let me = std::fs::metadata("/proc/self").map(|m| std::os::unix::fs::MetadataExt::uid(&m)).ok();
    let Ok(dir) = std::fs::read_dir("/proc") else { return false };
    dir.flatten().any(|e| {
        e.file_name().to_string_lossy().bytes().all(|b| b.is_ascii_digit())
            && std::fs::read_to_string(e.path().join("comm")).is_ok_and(|c| c.trim() == "trindle-lock")
            && e.metadata().ok().map(|m| std::os::unix::fs::MetadataExt::uid(&m)) == me
    })
}

fn tell(state: &State, session: &str) {
    let mut s: HashMap<&str, Value> = HashMap::new();
    s.insert("screensaver-active", Value::from(state.resting.get()));
    s.insert("session-state", Value::from(RUNNING));
    let Ok(path) = OwnedObjectPath::try_from(session) else { return };
    let _ = state.bus.emit(super::portal::PATH, INTERFACE, "StateChanged", &(path, s));
}

fn look(state: &Rc<State>) {
    if state.looking.replace(true) {
        return;
    }
    let weak = Rc::downgrade(state);
    state.events.every(LOOK, move || {
        let Some(state) = weak.upgrade() else { return Flow::Stop };
        if state.watches.borrow().is_empty() {
            state.looking.set(false);
            return Flow::Stop;
        }
        let now = resting();
        if now != state.resting.replace(now) {
            let all: Vec<String> = state.watches.borrow().keys().cloned().collect();
            for session in all {
                tell(&state, &session);
            }
        }
        Flow::Continue
    });
}

fn end_watch(state: &Rc<State>, session: &str, told: bool) {
    let Some(mut watch) = state.watches.borrow_mut().remove(session) else { return };
    if told {
        let _ = state.bus.emit(session, SESSION, "Closed", &());
    }
    let served = watch.served.take();
    state.release();
    state.events.after(Duration::ZERO, move || drop(served));
}

fn monitor(state: &Rc<State>, asked: Asked) {
    let Ok((_handle, session, _app_id, _window)) =
        asked.message().body().deserialize::<(OwnedObjectPath, OwnedObjectPath, String, String)>()
    else {
        return asked.reply(&2u32);
    };
    let session = session.as_str().to_string();
    if state.watches.borrow().contains_key(&session) {
        return asked.reply(&2u32);
    }
    let served = {
        let (weak, s) = (Rc::downgrade(state), session.clone());
        state.bus.serve(&session, Offer {
            name: SESSION.into(),
            xml: SESSION_XML.into(),
            call: Box::new(move |asked: Asked| {
                if asked.member().as_str() != "Close" {
                    return asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Sessions only close");
                }
                asked.reply(&());
                if let Some(state) = weak.upgrade() {
                    end_watch(&state, &s, false);
                }
            }),
            props: Box::new(|| Props::from([("version".to_string(), OwnedValue::from(1u32))])),
            set: None,
        })
    };
    let Ok(served) = served else { return asked.reply(&2u32) };
    state.hold();
    if state.watches.borrow().is_empty() {
        state.resting.set(resting());
    }
    state.watches.borrow_mut().insert(session.clone(), Watch { served: Some(served) });
    asked.reply(&0u32);

    let weak = Rc::downgrade(state);
    state.events.after(Duration::ZERO, move || {
        if let Some(state) = weak.upgrade() {
            if state.watches.borrow().contains_key(&session) {
                tell(&state, &session);
            }
        }
    });
    look(state);
}

pub fn offer(bus: &Bus, events: &Loop, stay: &Rc<Stay>, awake: Awake, logind: bool) -> Offer {
    let state = Rc::new(State {
        bus: bus.clone(),
        events: events.clone(),
        stay: stay.clone(),
        awake,
        system: logind.then(|| RefCell::new(None)),
        holds: RefCell::new(HashMap::new()),
        watches: RefCell::new(HashMap::new()),
        resting: Cell::new(false),
        looking: Cell::new(false),
        next: Cell::new(1),
    });
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            state.stay.last.set(Instant::now());
            match asked.member().as_str() {
                "Inhibit" => inhibit(&state, asked),
                "CreateMonitor" => monitor(&state, asked),

                "QueryEndResponse" => asked.reply(&()),
                _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Inhibit"),
            }
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal::PATH;
    use super::super::portal_ask::testkit::{call, Rig};
    use crate::toolkit::bus::testing::until;

    #[test]
    fn holds_last_until_closed() {
        let rig = Rig::new();
        let said: Rc<RefCell<Vec<(String, bool)>>> = Rc::new(RefCell::new(Vec::new()));
        let s = said.clone();
        let awake: Awake = Rc::new(move |tag, on| s.borrow_mut().push((tag.to_string(), on)));
        let _i = rig.a.serve(PATH, offer(&rig.a, &rig.lp, &rig.stay, awake, false)).expect("serve");
        let handle = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/t1").expect("path");
        let options = HashMap::from([("reason", Value::from("Playing a film"))]);
        let r: Option<()> = call(&rig, INTERFACE, "Inhibit", &(&handle, "org.example.Player", "", IDLE | SUSPEND, &options));
        assert!(r.is_some());
        assert_eq!(*said.borrow(), vec![("portal1".to_string(), true)]);
        assert_eq!(rig.stay.busy.get(), 1);

        let other = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/t2").expect("path");
        let _: Option<()> = call(&rig, INTERFACE, "Inhibit", &(&other, "org.example.Editor", "", 1u32, &options));
        assert_eq!(said.borrow().len(), 1);
        assert_eq!(rig.stay.busy.get(), 2);

        let got: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let g = got.clone();
        rig.b.call(&rig.a.unique_name(), handle.as_str(), REQUEST).method("Close", &(), false, None, &rig.lp, move |r| g.set(r.is_ok()));
        let g = got.clone();
        assert!(until(&rig.lp, move || g.get()));
        assert_eq!(said.borrow().last().cloned(), Some(("portal1".to_string(), false)));
        assert_eq!(rig.stay.busy.get(), 1);
    }

    #[test]
    fn a_monitor_hears_the_state_at_once_and_closes() {
        let rig = Rig::new();
        let _i = rig.a.serve(PATH, offer(&rig.a, &rig.lp, &rig.stay, Rc::new(|_, _| {}), false)).expect("serve");
        let heard: Rc<RefCell<Vec<(String, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let h = heard.clone();
        let _l = rig.b.listen(None, INTERFACE, Some("StateChanged"), None, move |m| {
            if let Ok((s, state)) = m.body().deserialize::<(OwnedObjectPath, HashMap<String, OwnedValue>)>() {
                let st = state.get("session-state").and_then(|v| u32::try_from(v).ok()).unwrap_or(0);
                assert!(state.contains_key("screensaver-active"));
                h.borrow_mut().push((s.as_str().to_string(), st));
            }
        }).expect("listen");
        let handle = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/m1").expect("path");
        let session = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/session/1_1/m1").expect("path");
        let r: Option<u32> = call(&rig, INTERFACE, "CreateMonitor", &(&handle, &session, "org.example.Player", ""));
        assert_eq!(r, Some(0));
        let h = heard.clone();
        assert!(until(&rig.lp, move || !h.borrow().is_empty()));
        assert_eq!(heard.borrow()[0], (session.as_str().to_string(), RUNNING));
        assert_eq!(rig.stay.busy.get(), 1);

        let r: Option<()> = call(&rig, INTERFACE, "QueryEndResponse", &(&session,));
        assert!(r.is_some());
        let got: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let g = got.clone();
        rig.b.call(&rig.a.unique_name(), session.as_str(), SESSION).method("Close", &(), false, None, &rig.lp, move |r| g.set(r.is_ok()));
        let g = got.clone();
        assert!(until(&rig.lp, move || g.get()));
        assert_eq!(rig.stay.busy.get(), 0);
    }
}
