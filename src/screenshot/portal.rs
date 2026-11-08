//! trindle-portal: screen photographs and the rest of what programs ask the desktop for.

use std::cell::Cell;
use std::collections::HashMap;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::{Duration, Instant};

use zbus::zvariant::{OwnedValue, Value};

use super::file_chooser::file_uri;
use super::portal_settings::Settings;
use crate::common::capture;
use crate::common::files::home;
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop};

const NAME: &str = "org.freedesktop.impl.portal.desktop.trindle";
pub(super) const PATH: &str = "/org/freedesktop/portal/desktop";
const INTERFACE: &str = "org.freedesktop.impl.portal.Screenshot";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Screenshot">
    <method name="Screenshot">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="PickColor">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <property name="version" type="u" access="read"/>
  </interface>"#;

pub(super) const DONE: u32 = 0;
pub(super) const CANCELLED: u32 = 1;
pub(super) const FAILED: u32 = 2;

const IDLE: Duration = Duration::from_secs(60);

const KEEP: usize = 1;

fn folder() -> PathBuf {
    home().join(".cache/trindle/portal-screenshots")
}

fn photograph() -> Result<String, String> {
    let dir = folder();
    {
        use std::os::unix::fs::DirBuilderExt;
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir).map_err(|e| e.to_string())?;
    }
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
    let file = dir.join(format!("screenshot-{stamp}.png"));
    capture::take(None, false)?.save_png(&file)?;
    tidy(&dir);
    Ok(file_uri(&file))
}

fn tidy(dir: &std::path::Path) {
    let Ok(read) = std::fs::read_dir(dir) else { return };
    let mut shots: Vec<PathBuf> = read.flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "png")).collect();
    shots.sort();

    let hour = Duration::from_secs(3600);
    let keep_from = shots.len().saturating_sub(KEEP);
    for (n, old) in shots.iter().enumerate() {
        let stale = old.metadata().and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|age| age > hour);
        if n < keep_from || stale {
            let _ = std::fs::remove_file(old);
        }
    }
}

pub(super) struct Stay {

    pub(super) last: Cell<Instant>,

    pub(super) busy: Cell<u32>,

    pub(super) settled: Cell<bool>,
}

fn pick(asked: Asked, events: &Loop, stay: Rc<Stay>) {
    let child = Command::new("trindle-screenshot").arg("--eye-dropper").stdout(Stdio::piped()).spawn();
    let (mut child, mut out) = match child.map(|mut c| (c.stdout.take(), c)) {
        Ok((Some(out), c)) => (c, out),
        Ok((None, mut c)) => {
            let _ = c.kill();
            let _ = c.wait();
            asked.reply(&answer(FAILED, HashMap::new()));
            return;
        }
        Err(e) => {
            eprintln!("trindle-portal: {e}");
            asked.reply(&answer(FAILED, HashMap::new()));
            return;
        }
    };
    let fd = out.as_raw_fd();
    // SAFETY: fd is the open pipe `out` owns; only its flags change.
    unsafe { libc::fcntl(fd, libc::F_SETFL, libc::fcntl(fd, libc::F_GETFL) | libc::O_NONBLOCK) };
    stay.busy.set(stay.busy.get() + 1);
    let (mut text, mut asked) = (Vec::new(), Some(asked));
    events.watch(fd, move |hung_up| {
        let mut ended = hung_up;
        let mut buf = [0u8; 256];
        while !ended {
            match out.read(&mut buf) {
                Ok(0) => ended = true,
                Ok(n) => text.extend_from_slice(&buf[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => ended = true,
            }
        }
        if !ended {
            return Flow::Continue;
        }
        stay.busy.set(stay.busy.get().saturating_sub(1));
        stay.last.set(Instant::now());
        let colour = super::eye_dropper::parse_colour(&String::from_utf8_lossy(&text));
        let code = child.wait().ok().and_then(|s| s.code());
        let reply = match (colour, code) {
            (Some(c), _) => answer(DONE, HashMap::from([("color", Value::from(c))])),
            (None, Some(super::eye_dropper::GAVE_UP)) => answer(CANCELLED, HashMap::new()),
            _ => answer(FAILED, HashMap::new()),
        };
        if let Some(asked) = asked.take() {
            asked.reply(&reply);
        }
        Flow::Stop
    });
}

pub(super) fn answer(response: u32, results: HashMap<&'static str, Value<'static>>) -> (u32, HashMap<&'static str, Value<'static>>) {
    (response, results)
}

fn shots(events: &Loop, stay: &Rc<Stay>) -> Offer {
    let (events, stay) = (events.clone(), stay.clone());
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            stay.last.set(Instant::now());
            let reply = match asked.member().as_str() {
                "Screenshot" => match photograph() {
                    Ok(uri) => answer(DONE, HashMap::from([("uri", Value::from(uri))])),
                    Err(e) => {
                        eprintln!("trindle-portal: {e}");
                        answer(FAILED, HashMap::new())
                    }
                },
                "PickColor" => return pick(asked, &events, stay.clone()),
                _ => answer(FAILED, HashMap::new()),
            };
            asked.reply(&reply);
        }),
        props: Box::new(|| Props::from([("version".to_string(), OwnedValue::from(2u32))])),
        set: None,
    }
}

const SETTLED: &str = "--settled";

fn renewal(busy: u32, settled: bool) -> Option<crate::common::renewed::Renew> {
    use crate::common::renewed::Renew;
    match (busy, settled) {
        (0, true) => Some(Renew::InPlaceAs(&[SETTLED])),
        (0, false) => Some(Renew::Leave),
        _ => None,
    }
}

fn events() -> Loop {
    let events = Loop::new();
    crate::common::run::hand_back_to(&events);
    events
}

pub fn main(args: Vec<String>) -> i32 {
    let events = events();
    let Ok(bus) = Bus::session(&events) else { return 1 };
    let stay = Rc::new(Stay { last: Cell::new(Instant::now()), busy: Cell::new(0), settled: Cell::new(args.iter().any(|a| a == SETTLED)) });
    let settings = Rc::new(Settings::new());
    settings.watch(&bus, &events, PATH);
    let Some(_served) = serve_all(&bus, &events, &stay, &settings) else { return 1 };
    let Some(_lost) = leave_if_name_lost(&bus, &events) else { return 1 };
    let s = stay.clone();
    crate::common::renewed::watch(&events, "trindle-portal", move || renewal(s.busy.get(), s.settled.get()));
    let quit = events.clone();
    events.every(Duration::from_secs(10), move || {
        if !stay.settled.get() && stay.busy.get() == 0 && stay.last.get().elapsed() >= IDLE {
            quit.quit();
            return Flow::Stop;
        }
        Flow::Continue
    });
    events.run();
    0
}

fn serve_all(bus: &Bus, events: &Loop, stay: &Rc<Stay>, settings: &Rc<Settings>) -> Option<Vec<Served>> {
    let s = stay.clone();
    let mut served = vec![
        bus.serve(PATH, shots(events, stay)).ok()?,
        bus.serve(PATH, settings.offer(move || s.settled.set(true))).ok()?,
        bus.serve(PATH, super::portal_files::offer(bus, events, stay)).ok()?,
        bus.serve(PATH, super::portal_lockdown::offer()).ok()?,
        bus.serve(PATH, super::portal_inhibit::offer(bus, events, stay, super::portal_inhibit::awake(), true)).ok()?,
    ];
    let kit = super::portal_ask::Kit::new(bus, events, stay);
    served.extend([
        bus.serve(PATH, super::portal_notify::offer(bus, events, stay)).ok()?,
        bus.serve(PATH, super::portal_access::offer_access(&kit)).ok()?,
        bus.serve(PATH, super::portal_access::offer_account(&kit)).ok()?,
        bus.serve(PATH, super::portal_email::offer(&kit)).ok()?,
        bus.serve(PATH, super::portal_wallpaper::offer(&kit)).ok()?,
        bus.serve(PATH, super::portal_launcher::offer(&kit)).ok()?,
        bus.serve(PATH, super::portal_apps::offer(&kit)).ok()?,
        bus.serve(PATH, super::portal_cast::offer(&kit)).ok()?,
        bus.serve(PATH, super::portal_print::offer(&kit)).ok()?,
    ]);
    Some(served)
}

fn leave_if_name_lost(bus: &Bus, events: &Loop) -> Option<Listen> {
    let lost = events.clone();
    let listening = bus.listen(Some("org.freedesktop.DBus"), "org.freedesktop.DBus", Some("NameLost"), None, move |m| {
        if m.body().deserialize::<String>().is_ok_and(|n| n == NAME) {
            lost.quit();
        }
    }).ok()?;
    let lost = events.clone();
    bus.own(NAME, false, events, move |ok| {
        if !ok {
            lost.quit();
        }
    });
    Some(listening)
}

#[cfg(test)]
mod tests {
    #[test]
    fn after_an_update_the_portal_goes_only_when_it_is_doing_nothing() {
        use crate::common::renewed::Renew;

        assert_eq!(super::renewal(1, false), None);
        assert_eq!(super::renewal(3, true), None);

        assert_eq!(super::renewal(0, false), Some(Renew::Leave));
        assert_eq!(super::renewal(0, true), Some(Renew::InPlaceAs(&[super::SETTLED])));
    }

    #[test]
    fn only_the_newest_few_are_kept() {
        let dir = std::env::temp_dir().join(format!("trindle-portal-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for n in 0..8 {
            std::fs::write(dir.join(format!("screenshot-{n}.png")), b"").unwrap();
        }
        super::tidy(&dir);
        let mut left: Vec<String> = std::fs::read_dir(&dir).unwrap().flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        left.sort();
        assert_eq!(left, vec!["screenshot-7.png".to_string()]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_address_keeps_only_safe_bytes() {
        assert_eq!(super::file_uri(std::path::Path::new("/home/john smith/shot-1.png")), "file:///home/john%20smith/shot-1.png");
        assert_eq!(super::file_uri(std::path::Path::new("/tmp/caf\u{e9}%")), "file:///tmp/caf%C3%A9%25");
    }

    #[test]
    fn work_on_a_thread_answers_with_no_window_shown() {
        use std::cell::Cell;
        use std::rc::Rc;
        let lp = super::events();
        let got = Rc::new(Cell::new(0));
        let g = got.clone();
        crate::common::run::on_thread(|| 7, move |v| g.set(v));
        let g = got.clone();
        assert!(crate::toolkit::bus::testing::until(&lp, move || g.get() == 7), "the answer never came back to the loop");
    }

    #[test]
    fn both_interfaces_answer_on_the_bus() {
        use crate::toolkit::bus::{get_u32, testing::{own_daemon, until}, Bus};
        use crate::toolkit::event::Loop;
        use std::cell::{Cell, RefCell};
        use std::rc::Rc;
        use std::time::Instant;

        let d = own_daemon();
        let lp = Loop::new();
        let (a, b) = (Bus::at(&lp, &d.1).expect("bus a"), Bus::at(&lp, &d.1).expect("bus b"));
        let stay = Rc::new(super::Stay { last: Cell::new(Instant::now()), busy: Cell::new(0), settled: Cell::new(false) });
        let settings = Rc::new(super::Settings::new());
        let s = stay.clone();
        let _shots = a.serve(super::PATH, super::shots(&lp, &stay)).expect("shots");
        let _asks = a.serve(super::PATH, settings.offer(move || s.settled.set(true))).expect("settings");
        let who = a.unique_name();
        let ask = |iface: &'static str| b.call(&who, super::PATH, iface);
        let got: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let wait = |lp: &Loop| {
            let g = got.clone();
            assert!(until(lp, move || g.borrow().is_some()));
            got.borrow_mut().take().unwrap_or_default()
        };

        for (iface, version) in [(super::INTERFACE, 2), (crate::screenshot::portal_settings::INTERFACE, 1)] {
            let g = got.clone();
            ask(iface).get_all(&lp, move |r| *g.borrow_mut() = Some(r.map(|p| get_u32(&p, "version").to_string()).unwrap_or_default()));
            assert_eq!(wait(&lp), version.to_string());
        }

        let g = got.clone();
        ask(crate::screenshot::portal_settings::INTERFACE).method("ReadAll", &(vec![String::new()],), false, None, &lp, move |r| {
            *g.borrow_mut() = Some(r.map(|m| m.body().signature().to_string()).unwrap_or_else(|f| f.name))
        });
        assert_eq!(wait(&lp), "a{sa{sv}}");
        assert!(stay.settled.get());

        let g = got.clone();
        ask(crate::screenshot::portal_settings::INTERFACE).method("Read", &("no.such", "thing"), false, None, &lp, move |r| {
            *g.borrow_mut() = Some(r.err().map(|f| f.name).unwrap_or_default())
        });
        assert_eq!(wait(&lp), "org.freedesktop.portal.Error.NotFound");

        let g = got.clone();
        ask("org.freedesktop.DBus.Introspectable").method("Introspect", &(), false, None, &lp, move |r| {
            *g.borrow_mut() = Some(r.ok().and_then(|m| m.body().deserialize::<String>().ok()).unwrap_or_default())
        });
        let xml = wait(&lp);
        assert!(xml.contains(super::INTERFACE) && xml.contains(crate::screenshot::portal_settings::INTERFACE), "{xml}");
    }
}
