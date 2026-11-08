//! The Notification portal: a sandboxed program's notifications, handed to
//! Trindle's own notification server (ring::notify) as if the program had
//! sent them itself, and a press on one handed back to the program.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::Read;
use std::os::fd::AsFd;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use zbus::zvariant::{OwnedValue, Value};

use super::portal::{Stay, PATH};
use super::portal_ask::{app_name, get, Options};
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props};
use crate::toolkit::event::Loop;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Notification";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Notification">
    <method name="AddNotification">
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="id" direction="in"/>
      <arg type="a{sv}" name="notification" direction="in"/>
    </method>
    <method name="RemoveNotification">
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="id" direction="in"/>
    </method>
    <property name="SupportedOptions" type="a{sv}" access="read"/>
    <property name="version" type="u" access="read"/>
    <signal name="ActionInvoked">
      <arg type="s" name="app_id"/>
      <arg type="s" name="id"/>
      <arg type="s" name="action"/>
      <arg type="av" name="parameter"/>
    </signal>
  </interface>"#;

pub(super) const SERVER: &str = "org.freedesktop.Notifications";
const SERVER_PATH: &str = "/org/freedesktop/Notifications";

const ICON_MOST: u64 = 4 << 20;

type Key = (String, String);

type Action = (String, Option<OwnedValue>);

struct Sent {
    key: Key,
    default: Option<Action>,
    buttons: Vec<Action>,

    file: Option<PathBuf>,
}

impl Sent {
    fn wants_presses(&self) -> bool {
        self.default.is_some() || !self.buttons.is_empty()
    }
}

struct Slot {
    turn: u64,
    at: Option<u32>,
}

struct State {
    bus: Bus,
    events: Loop,
    stay: Rc<Stay>,
    slots: RefCell<HashMap<Key, Slot>>,
    shown: RefCell<HashMap<u32, Sent>>,
    turn: Cell<u64>,
    hearing: RefCell<Vec<Listen>>,
}

impl State {
    fn hold(&self) {
        self.stay.busy.set(self.stay.busy.get() + 1);
    }

    fn release(&self) {
        self.stay.busy.set(self.stay.busy.get().saturating_sub(1));
        self.stay.last.set(Instant::now());
    }

    fn forget(&self, number: u32) {
        let Some(sent) = self.shown.borrow_mut().remove(&number) else { return };
        let mut slots = self.slots.borrow_mut();
        if slots.get(&sent.key).is_some_and(|s| s.at == Some(number)) {
            slots.remove(&sent.key);
        }
        drop(slots);
        if let Some(f) = &sent.file {
            let _ = std::fs::remove_file(f);
        }

        if sent.wants_presses() {
            self.release();
        }
    }

    fn close_on_server(&self, number: u32) {
        self.bus.call(SERVER, SERVER_PATH, SERVER).never_start()
            .method("CloseNotification", &(number,), false, Some(Duration::from_secs(5)), &self.events, |_| {});
    }
}

enum Icon {
    Named(String),
    Bytes(Vec<u8>),
}

fn icon_of(v: &Value) -> Option<Icon> {

    if let Value::Value(inner) = v {
        return icon_of(inner);
    }
    let Value::Structure(s) = v else { return None };
    let [Value::Str(kind), inner] = s.fields() else { return None };
    let inner = match inner {
        Value::Value(b) => &**b,
        other => other,
    };
    match (kind.as_str(), inner) {
        ("themed", Value::Array(names)) => names.iter().find_map(|n| match n {
            Value::Str(s) if !s.is_empty() => Some(Icon::Named(s.to_string())),
            _ => None,
        }),
        ("bytes", Value::Array(bytes)) => {
            let all: Vec<u8> = bytes.iter().filter_map(|b| if let Value::U8(b) = b { Some(*b) } else { None }).collect();
            (!all.is_empty()).then_some(Icon::Bytes(all))
        }
        ("file", Value::Fd(fd)) => {
            let file = std::fs::File::from(fd.as_fd().try_clone_to_owned().ok()?);
            let mut all = Vec::new();
            file.take(ICON_MOST).read_to_end(&mut all).ok()?;
            (!all.is_empty()).then_some(Icon::Bytes(all))
        }
        _ => None,
    }
}

fn plain(markup: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in markup.chars() {
        match c {
            '<' => inside = true,
            '>' if inside => inside = false,
            c if !inside => out.push(c),
            _ => {}
        }
    }
    out.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

fn urgency(priority: &str) -> u8 {
    match priority {
        "low" => 0,
        "urgent" => 2,
        _ => 1,
    }
}

fn buttons_of(options: &Options) -> Vec<(String, Action)> {
    let Some(v) = options.get("buttons") else { return Vec::new() };
    let Value::Array(all) = &**v else { return Vec::new() };
    all.iter().filter_map(|b| {
        let Value::Dict(d) = b else { return None };
        let map: HashMap<String, OwnedValue> = HashMap::try_from(d.try_clone().ok()?).ok()?;
        let words = get::<String>(&map, "label")?;
        let action = get::<String>(&map, "action")?;
        Some((words, (action, map.get("target").and_then(|t| t.try_clone().ok()))))
    }).collect()
}

fn add(state: &Rc<State>, asked: Asked) {
    let Ok((app_id, id, options)) = asked.message().body().deserialize::<(String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "AddNotification takes an app, an id and the notification");
    };
    asked.reply(&());
    let key: Key = (app_id.clone(), id);
    let title = get::<String>(&options, "title").unwrap_or_default();
    let body = get::<String>(&options, "body")
        .or_else(|| get::<String>(&options, "markup-body").map(|m| plain(&m)))
        .unwrap_or_default();
    let (icon, file) = shown_icon(&options, state.turn.get() + 1);
    let default = get::<String>(&options, "default-action")
        .map(|a| (a, options.get("default-action-target").and_then(|t| t.try_clone().ok())));
    let buttons = buttons_of(&options);
    let actions = action_list(default.is_some(), &buttons);
    let hints = hints_of(&options, &app_id);

    let turn = state.turn.get() + 1;
    state.turn.set(turn);
    let replaces = {
        let mut slots = state.slots.borrow_mut();
        let replaces = slots.get(&key).and_then(|s| s.at).unwrap_or(0);
        slots.insert(key.clone(), Slot { turn, at: (replaces != 0).then_some(replaces) });
        replaces
    };
    let sent = Sent { key: key.clone(), default, buttons: buttons.into_iter().map(|(_, a)| a).collect(), file };
    let waiting = Waiting { key, turn, replaces, sent };
    let weak = Rc::downgrade(state);
    let body = (app_name(&app_id), replaces, icon, title, body, actions, hints, -1i32);
    state.bus.call(SERVER, SERVER_PATH, SERVER).never_start()
        .method("Notify", &body, false, Some(Duration::from_secs(10)), &state.events, move |r| {
            let Some(state) = weak.upgrade() else { return };
            let number = r.ok().and_then(|m| m.body().deserialize::<u32>().ok());
            state.numbered(number, waiting);
        });
}

fn shown_icon(options: &Options, turn: u64) -> (String, Option<PathBuf>) {
    match options.get("icon").and_then(|v| icon_of(v)) {
        Some(Icon::Named(name)) => (name, None),
        Some(Icon::Bytes(bytes)) => {
            let dir = crate::common::files::scratch().join("trindle-portal-notes");
            let path = dir.join(format!("{}-{}", std::process::id(), turn));
            if std::fs::create_dir_all(&dir).is_ok() && std::fs::write(&path, bytes).is_ok() {
                (path.to_string_lossy().into_owned(), Some(path))
            } else {
                (String::new(), None)
            }
        }
        None => (String::new(), None),
    }
}

fn action_list(has_default: bool, buttons: &[(String, Action)]) -> Vec<String> {
    let mut actions: Vec<String> = Vec::new();
    if has_default {
        actions.extend(["default".to_string(), String::new()]);
    }
    for (i, (words, _)) in buttons.iter().enumerate() {
        actions.extend([format!("button{i}"), words.clone()]);
    }
    actions
}

fn hints_of<'a>(options: &Options, app_id: &str) -> HashMap<&'a str, Value<'a>> {
    let mut hints: HashMap<&str, Value> = HashMap::new();
    hints.insert("urgency", Value::U8(urgency(&get::<String>(options, "priority").unwrap_or_default())));
    if !app_id.is_empty() {
        hints.insert("desktop-entry", Value::from(app_id.to_string()));
    }
    hints
}

struct Waiting {
    key: Key,
    turn: u64,
    replaces: u32,
    sent: Sent,
}

impl State {

    fn numbered(&self, number: Option<u32>, waiting: Waiting) {
        let Waiting { key, turn, replaces, sent } = waiting;
        let Some(number) = number else {
            if let Some(f) = &sent.file {
                let _ = std::fs::remove_file(f);
            }
            let mut slots = self.slots.borrow_mut();
            if slots.get(&key).is_some_and(|s| s.turn == turn) {
                slots.remove(&key);
            }
            return;
        };
        let current = self.slots.borrow().get(&key).is_some_and(|s| s.turn == turn);
        if replaces != 0 {
            self.forget(replaces);
        }
        if !current {

            self.close_on_server(number);
            if let Some(f) = &sent.file {
                let _ = std::fs::remove_file(f);
            }
            return;
        }
        self.forget(number);
        if sent.wants_presses() {
            self.hold();
        }
        self.slots.borrow_mut().insert(key, Slot { turn, at: Some(number) });
        self.shown.borrow_mut().insert(number, sent);
    }
}

fn remove(state: &Rc<State>, asked: Asked) {
    let Ok(key) = asked.message().body().deserialize::<Key>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "RemoveNotification takes an app and an id");
    };
    asked.reply(&());
    let Some(slot) = state.slots.borrow_mut().remove(&key) else { return };
    if let Some(number) = slot.at {
        state.close_on_server(number);
        state.forget(number);
    }
}

fn pressed(state: &State, number: u32, key: &str) {
    let shown = state.shown.borrow();
    let Some(sent) = shown.get(&number) else { return };
    let action = if key == "default" {
        sent.default.as_ref()
    } else {
        key.strip_prefix("button").and_then(|i| i.parse::<usize>().ok()).and_then(|i| sent.buttons.get(i))
    };
    let Some((name, target)) = action else { return };
    let parameter: Vec<Value> = target.iter().filter_map(|t| t.try_clone().ok()).map(Value::from).collect();
    let _ = state.bus.emit(PATH, INTERFACE, "ActionInvoked", &(sent.key.0.as_str(), sent.key.1.as_str(), name.as_str(), parameter));
}

pub fn offer(bus: &Bus, events: &Loop, stay: &Rc<Stay>) -> Offer {
    let state = Rc::new(State {
        bus: bus.clone(),
        events: events.clone(),
        stay: stay.clone(),
        slots: RefCell::new(HashMap::new()),
        shown: RefCell::new(HashMap::new()),
        turn: Cell::new(0),
        hearing: RefCell::new(Vec::new()),
    });
    let mut hearing = Vec::new();
    let weak = Rc::downgrade(&state);
    hearing.extend(bus.listen(Some(SERVER), SERVER, Some("ActionInvoked"), Some(SERVER_PATH), move |m| {
        let (Some(state), Ok((number, key))) = (weak.upgrade(), m.body().deserialize::<(u32, String)>()) else { return };
        pressed(&state, number, &key);
    }).ok());
    let weak = Rc::downgrade(&state);
    hearing.extend(bus.listen(Some(SERVER), SERVER, Some("NotificationClosed"), Some(SERVER_PATH), move |m| {
        let (Some(state), Ok((number, _why))) = (weak.upgrade(), m.body().deserialize::<(u32, u32)>()) else { return };
        state.forget(number);
    }).ok());

    let weak = Rc::downgrade(&state);
    hearing.extend(bus.watch(SERVER, events, |_| {}, move || {
        let Some(state) = weak.upgrade() else { return };
        let all: Vec<u32> = state.shown.borrow().keys().copied().collect();
        for n in all {
            state.forget(n);
        }
    }).ok());
    *state.hearing.borrow_mut() = hearing;
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            state.stay.last.set(Instant::now());
            match asked.member().as_str() {
                "AddNotification" => add(&state, asked),
                "RemoveNotification" => remove(&state, asked),
                _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Notification"),
            }
        }),
        props: Box::new(|| Props::from([
            ("SupportedOptions".to_string(), OwnedValue::from(HashMap::<String, OwnedValue>::new())),
            ("version".to_string(), OwnedValue::from(2u32)),
        ])),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal_ask::testkit::{call, props, Rig};
    use crate::toolkit::bus::{get_u32, testing::until};

    type Heard = (u32, String, String, String, Vec<String>, u8);

    struct Server {
        _bus: Bus,
        _served: crate::toolkit::bus::Served,
        heard: Rc<RefCell<Vec<Heard>>>,
        closed: Rc<RefCell<Vec<u32>>>,
    }

    fn server(rig: &Rig, addr: &str) -> Server {
        let bus = Bus::at(&rig.lp, addr).expect("bus");
        let heard: Rc<RefCell<Vec<Heard>>> = Rc::new(RefCell::new(Vec::new()));
        let closed: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let (h, c) = (heard.clone(), closed.clone());
        let next = Cell::new(40u32);
        let served = bus.serve(SERVER_PATH, Offer {
            name: SERVER.into(),
            xml: r#"<interface name="org.freedesktop.Notifications">
                <method name="Notify"><arg type="s"/><arg type="u"/><arg type="s"/><arg type="s"/><arg type="s"/><arg type="as"/><arg type="a{sv}"/><arg type="i"/><arg type="u" direction="out"/></method>
                <method name="CloseNotification"><arg type="u"/></method>
              </interface>"#.into(),
            call: Box::new(move |asked: Asked| match asked.member().as_str() {
                "Notify" => {
                    let (_app, replaces, icon, title, body, actions, hints, _t) = asked.message().body()
                        .deserialize::<(String, u32, String, String, String, Vec<String>, HashMap<String, OwnedValue>, i32)>().expect("notify");
                    let u = hints.get("urgency").and_then(|v| u8::try_from(v).ok()).unwrap_or(9);
                    h.borrow_mut().push((replaces, icon, title, body, actions, u));
                    let n = if replaces != 0 { replaces } else { next.set(next.get() + 1); next.get() };
                    asked.reply(&n);
                }
                _ => {
                    let n = asked.message().body().deserialize::<u32>().expect("close");
                    c.borrow_mut().push(n);
                    asked.reply(&());
                }
            }),
            props: Box::new(Props::new),
            set: None,
        }).expect("serve");
        let owned: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own(SERVER, false, &rig.lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&rig.lp, move || o.get().is_some()));
        assert_eq!(owned.get(), Some(true));
        Server { _bus: bus, _served: served, heard, closed }
    }

    #[test]
    fn notes_reach_the_server_and_presses_come_back() {
        let rig = Rig::new();
        let addr = rig._daemon.1.clone();
        let srv = server(&rig, &addr);
        let _n = rig.a.serve(PATH, offer(&rig.a, &rig.lp, &rig.stay)).expect("serve");
        let p = props(&rig, INTERFACE).expect("props");
        assert_eq!(get_u32(&p, "version"), 2);

        let pressed: Rc<RefCell<Vec<(String, String, String, usize)>>> = Rc::new(RefCell::new(Vec::new()));
        let pr = pressed.clone();
        let _l = rig.b.listen(None, INTERFACE, Some("ActionInvoked"), None, move |m| {
            let (a, i, act, p) = m.body().deserialize::<(String, String, String, Vec<OwnedValue>)>().expect("invoked");
            pr.borrow_mut().push((a, i, act, p.len()));
        }).expect("listen");

        let icon = Value::new(Value::from(("themed", Value::new(vec!["mail-unread", "mail"]))));
        let note: HashMap<&str, Value> = HashMap::from([
            ("title", Value::from("New mail")),
            ("markup-body", Value::from("<b>Hello</b> &amp; welcome")),
            ("icon", icon),
            ("priority", Value::from("urgent")),
            ("default-action", Value::from("app.open")),
            ("default-action-target", Value::new(Value::from(7u32))),
        ]);
        let r: Option<()> = call(&rig, INTERFACE, "AddNotification", &("org.example.Mail", "m1", &note));
        assert!(r.is_some());
        let h = srv.heard.clone();
        assert!(until(&rig.lp, move || !h.borrow().is_empty()));
        assert_eq!(srv.heard.borrow()[0], (0, "mail-unread".into(), "New mail".into(), "Hello & welcome".into(), vec!["default".into(), String::new()], 2));
        let s = rig.stay.clone();
        assert!(until(&rig.lp, move || s.busy.get() == 1));

        let r: Option<()> = call(&rig, INTERFACE, "AddNotification", &("org.example.Mail", "m1", &note));
        assert!(r.is_some());
        let h = srv.heard.clone();
        assert!(until(&rig.lp, move || h.borrow().len() == 2));
        assert_eq!(srv.heard.borrow()[1].0, 41);
        assert_eq!(rig.stay.busy.get(), 1);

        srv._bus.emit(SERVER_PATH, SERVER, "ActionInvoked", &(41u32, "default")).expect("emit");
        let p = pressed.clone();
        assert!(until(&rig.lp, move || !p.borrow().is_empty()));
        assert_eq!(pressed.borrow()[0], ("org.example.Mail".into(), "m1".into(), "app.open".into(), 1));

        let r: Option<()> = call(&rig, INTERFACE, "RemoveNotification", &("org.example.Mail", "m1"));
        assert!(r.is_some());
        let c = srv.closed.clone();
        assert!(until(&rig.lp, move || !c.borrow().is_empty()));
        assert_eq!(*srv.closed.borrow(), vec![41]);
        assert_eq!(rig.stay.busy.get(), 0);
    }

    #[test]
    fn markup_and_icons_are_read_plainly() {
        assert_eq!(plain("<i>a</i> &lt;b&gt; &quot;c&quot;"), "a <b> \"c\"");
        let v = Value::from(("bytes", Value::new(vec![1u8, 2, 3])));
        assert!(matches!(icon_of(&v), Some(Icon::Bytes(b)) if b == vec![1, 2, 3]));
        assert!(icon_of(&Value::from("mail")).is_none());
        assert_eq!((urgency("low"), urgency("high"), urgency("urgent")), (0, 1, 2));
    }
}
