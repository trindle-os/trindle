//! The FileChooser interface of trindle-portal.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use super::file_chooser::{Ask, Chooser, Filter, Mode, Outcome, Pick, SIZE};
use super::portal::{answer, Stay, CANCELLED, DONE, FAILED};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::ui::Shown;
use crate::toolkit::window::{Display, Kind};

pub const INTERFACE: &str = "org.freedesktop.impl.portal.FileChooser";
const VERSION: u32 = 4;
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.FileChooser">
    <method name="OpenFile">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="SaveFile">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="SaveFiles">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <property name="version" type="u" access="read"/>
  </interface>"#;
const REQUEST: &str = "org.freedesktop.impl.portal.Request";
const FOLDERS: &str = if cfg!(test) { "trindle-folders-not-in-tests" } else { "trindle-folders" };
const REQUEST_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Request">
    <method name="Close"/>
  </interface>"#;

type Options = HashMap<String, OwnedValue>;
type Rules = Vec<(u32, String)>;

fn get<T: TryFrom<OwnedValue>>(options: &Options, key: &str) -> Option<T> {
    T::try_from(options.get(key)?.try_clone().ok()?).ok()
}

fn bytes_path(mut b: Vec<u8>) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    while b.last() == Some(&0) {
        b.pop();
    }
    (!b.is_empty()).then(|| PathBuf::from(std::ffi::OsString::from_vec(b)))
}

pub fn ask_from(member: &str, app_id: &str, title: &str, options: &Options) -> Ask {
    let flag = |k: &str| get::<bool>(options, k).unwrap_or(false);
    let mut folder = get::<Vec<u8>>(options, "current_folder").and_then(bytes_path);
    let mut name = get::<String>(options, "current_name").unwrap_or_default();
    let mode = match member {
        "SaveFile" => {
            if let Some(file) = get::<Vec<u8>>(options, "current_file").and_then(bytes_path) {
                folder = file.parent().map(PathBuf::from).or(folder);
                if let Some(n) = file.file_name() {
                    name = n.to_string_lossy().into_owned();
                }
            }
            Mode::Save
        }
        "SaveFiles" => {
            let files = get::<Vec<Vec<u8>>>(options, "files").unwrap_or_default();
            Mode::SaveMany(files.into_iter().filter_map(bytes_path)
                .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).collect())
        }
        _ => Mode::Open { many: flag("multiple"), folder: flag("directory") },
    };
    let mut filters: Vec<Filter> = get::<Vec<(String, Rules)>>(options, "filters").unwrap_or_default()
        .into_iter().map(|(name, rules)| Filter { name, rules }).collect();
    let filter = get::<(String, Rules)>(options, "current_filter").map(|(name, rules)| {
        let f = Filter { name, rules };
        filters.iter().position(|g| *g == f).unwrap_or_else(|| {
            filters.push(f);
            filters.len() - 1
        })
    });
    let choices = super::portal_ask::choices_of(options).into_iter()
        .map(|(id, label, options, value)| Pick { id, label, options, value }).collect();
    Ask {
        title: title.to_string(),
        accept: get::<String>(options, "accept_label"),
        mode, filters, filter, choices, folder, name,
        app_id: app_id.to_string(),
    }
}

fn results(outcome: Outcome) -> (u32, HashMap<&'static str, Value<'static>>) {
    match outcome {
        Outcome::Cancelled => answer(CANCELLED, HashMap::new()),
        Outcome::Chosen { uris, choices, filter } => {
            let mut out = HashMap::from([("uris", Value::from(uris))]);
            if !choices.is_empty() {
                out.insert("choices", Value::from(choices));
            }
            if let Some(f) = filter {
                out.insert("current_filter", Value::from((f.name, f.rules)));
            }
            answer(DONE, out)
        }
    }
}

struct Request {
    asked: Option<Asked>,
    shown: Option<Rc<Shown<Chooser>>>,
    close: Option<Served>,
}

fn choose_in_folders(program: &str, ask: &Ask, pid: &std::sync::atomic::AtomicI32) -> Option<Outcome> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let request = serde_json::to_vec(ask).ok()?;
    let mut child = Command::new(program).arg("--choose")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    pid.store(child.id() as i32, std::sync::atomic::Ordering::SeqCst);
    let written = child.stdin.take().is_some_and(|mut i| i.write_all(&request).is_ok());
    let line = child.stdout.take().and_then(|out| {
        std::io::BufRead::lines(std::io::BufReader::new(out)).map_while(Result::ok).find(|l| !l.trim().is_empty())
    });
    pid.store(0, std::sync::atomic::Ordering::SeqCst);
    std::thread::spawn(move || child.wait());
    serde_json::from_str(&line.filter(|_| written)?).ok()
}

fn finish(request: &Rc<RefCell<Request>>, events: &Loop, stay: &Rc<Stay>, outcome: Outcome) {
    let Some(asked) = request.borrow_mut().asked.take() else { return };
    asked.reply(&results(outcome));
    stay.busy.set(stay.busy.get().saturating_sub(1));
    stay.last.set(Instant::now());
    let request = request.clone();
    events.after(Duration::ZERO, move || {
        let mut r = request.borrow_mut();
        r.shown = None;
        r.close = None;
    });
}

type Screen = Rc<RefCell<Option<Rc<Display>>>>;

fn screen(events: &Loop, held: &Screen) -> Option<Rc<Display>> {
    if let Some(d) = held.borrow().as_ref() {
        return Some(d.clone());
    }
    let gone = held.clone();
    let display = Display::connect(events, move || {
        gone.borrow_mut().take();
    })?;
    *held.borrow_mut() = Some(display.clone());
    Some(display)
}

fn open(asked: Asked, bus: &Bus, events: &Loop, stay: &Rc<Stay>, held: &Screen) {
    let Ok((handle, app_id, parent, title, options)) =
        asked.message().body().deserialize::<(OwnedObjectPath, String, String, String, Options)>()
    else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    let ask = ask_from(&asked.member(), &app_id, &title, &options);
    if crate::common::run::have(FOLDERS) {
        return through_folders(asked, handle, parent, ask, bus, events, stay, held);
    }
    show_own(asked, handle, parent, ask, bus, events, stay, held);
}

#[allow(clippy::too_many_arguments)]
fn through_folders(asked: Asked, handle: OwnedObjectPath, parent: String, ask: Ask, bus: &Bus, events: &Loop, stay: &Rc<Stay>, held: &Screen) {
    use std::sync::atomic::{AtomicI32, Ordering};
    stay.busy.set(stay.busy.get() + 1);
    let request = Rc::new(RefCell::new(Request { asked: Some(asked), shown: None, close: None }));
    let pid = std::sync::Arc::new(AtomicI32::new(0));
    let close = {
        let (request, events, stay, pid) = (request.clone(), events.clone(), stay.clone(), pid.clone());
        bus.serve(handle.as_str(), Offer {
            name: REQUEST.into(),
            xml: REQUEST_XML.into(),
            call: Box::new(move |asked: Asked| {
                asked.reply(&());
                let child = pid.load(Ordering::SeqCst);
                if child > 0 {
                    // SAFETY: a plain signal to the chooser this portal started.
                    unsafe { libc::kill(child, libc::SIGTERM) };
                }
                finish(&request, &events, &stay, Outcome::Cancelled);
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok()
    };
    request.borrow_mut().close = close;
    let (bus, events, stay, held) = (bus.clone(), events.clone(), stay.clone(), held.clone());
    let asking = ask.clone();
    crate::common::run::on_thread(move || choose_in_folders(FOLDERS, &asking, &pid), move |outcome| match outcome {
        Some(outcome) => finish(&request, &events, &stay, outcome),
        None => {
            let mut r = request.borrow_mut();
            r.close = None;
            let Some(asked) = r.asked.take() else { return };
            drop(r);
            stay.busy.set(stay.busy.get().saturating_sub(1));
            show_own(asked, handle, parent, ask, &bus, &events, &stay, &held);
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn show_own(asked: Asked, handle: OwnedObjectPath, parent: String, ask: Ask, bus: &Bus, events: &Loop, stay: &Rc<Stay>, held: &Screen) {
    let Some(display) = screen(events, held) else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    stay.busy.set(stay.busy.get() + 1);
    let request = Rc::new(RefCell::new(Request { asked: Some(asked), shown: None, close: None }));
    let done = {
        let (request, events, stay) = (request.clone(), events.clone(), stay.clone());
        move |outcome: Outcome| finish(&request, &events, &stay, outcome)
    };
    let words = match ask.mode {
        Mode::Save => t!("screenshot-portal-save"),
        Mode::SaveMany(_) => t!("screenshot-files-save-many"),
        Mode::Open { folder: true, .. } => t!("screenshot-files-folder"),
        Mode::Open { .. } => t!("screenshot-portal-open"),
    };
    let kind = Kind::Window {
        title: if ask.title.is_empty() { words.into() } else { ask.title.clone() },
        app_id: "org.trindle.FileChooser".into(),
    };
    let Some(shown) = Shown::open(events, &display, kind, SIZE, Chooser::new(ask, done)) else {
        return finish(&request, events, stay, Outcome::Cancelled);
    };
    let shown = Rc::new(shown);
    if let Some(handle) = parent.strip_prefix("wayland:") {
        shown.window.set_parent(handle);
    }
    {
        let (request, events, stay) = (request.clone(), events.clone(), stay.clone());
        shown.window.on_close(move || finish(&request, &events, &stay, Outcome::Cancelled));
    }
    super::file_chooser::wire(events, &shown);
    let close = {
        let (request, events2, stay) = (request.clone(), events.clone(), stay.clone());
        bus.serve(handle.as_str(), Offer {
            name: REQUEST.into(),
            xml: REQUEST_XML.into(),
            call: Box::new(move |asked: Asked| {
                asked.reply(&());
                finish(&request, &events2, &stay, Outcome::Cancelled);
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok()
    };
    let mut r = request.borrow_mut();
    if r.asked.is_some() {
        r.shown = Some(shown);
        r.close = close;
    }
}

pub fn offer(bus: &Bus, events: &Loop, stay: &Rc<Stay>) -> Offer {
    let (bus, events, stay) = (bus.clone(), events.clone(), stay.clone());
    let held: Screen = Rc::new(RefCell::new(None));
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            stay.last.set(Instant::now());
            match asked.member().as_str() {
                "OpenFile" | "SaveFile" | "SaveFiles" => open(asked, &bus, &events, &stay, &held),
                _ => asked.reply(&answer(FAILED, HashMap::new())),
            }
        }),
        props: Box::new(|| Props::from([("version".to_string(), OwnedValue::from(VERSION))])),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned<'a>(v: impl Into<Value<'a>>) -> OwnedValue {
        OwnedValue::try_from(v.into()).expect("owned")
    }

    #[test]
    fn folders_reads_the_request_and_its_answer_is_read() {
        let ask = Ask { title: "Open".into(), accept: None, mode: Mode::Open { many: true, folder: false },
                        filters: vec![Filter { name: "Pictures".into(), rules: vec![(0, "*.png".into()), (1, "image/png".into())] }],
                        filter: Some(0), choices: vec![Pick { id: "q".into(), label: "Quality".into(), options: vec![("hi".into(), "High".into())], value: "hi".into() }],
                        folder: Some(PathBuf::from("/home/john")), name: String::new(), app_id: "org.example.App".into() };
        let json: serde_json::Value = serde_json::to_value(&ask).unwrap();
        assert_eq!(json["mode"], serde_json::json!({"Open": {"many": true, "folder": false}}));
        assert_eq!(json["filters"][0]["rules"], serde_json::json!([[0, "*.png"], [1, "image/png"]]));
        assert_eq!(json["choices"][0]["options"], serde_json::json!([["hi", "High"]]));
        assert_eq!(json["folder"], "/home/john");
        assert_eq!(serde_json::to_value(Mode::SaveMany(vec!["a.txt".into()])).unwrap(), serde_json::json!({"SaveMany": ["a.txt"]}));
        let chosen: Outcome = serde_json::from_str(r#"{"Chosen":{"uris":["file:///home/john/a.png"],"choices":[["q","hi"]],"filter":{"name":"Pictures","rules":[[0,"*.png"]]}}}"#).unwrap();
        assert_eq!(chosen, Outcome::Chosen { uris: vec!["file:///home/john/a.png".into()], choices: vec![("q".into(), "hi".into())],
                                             filter: Some(Filter { name: "Pictures".into(), rules: vec![(0, "*.png".into())] }) });
        assert_eq!(serde_json::from_str::<Outcome>(r#""Cancelled""#).unwrap(), Outcome::Cancelled);
        assert!(serde_json::from_str::<Outcome>(r#"{"Chosen":{"uris":[],"choices":[],"filter":null}}"#).is_ok());
    }

    #[test]
    fn folders_is_given_the_request_and_its_answer_or_failure_is_taken() {
        use std::sync::atomic::{AtomicI32, Ordering};
        let dir = std::env::temp_dir().join(format!("trindle-choose-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let script = |name: &str, body: &str| {
            let file = dir.join(name);
            std::fs::write(&file, format!("#!/bin/sh\n{body}\n")).unwrap();
            std::fs::set_permissions(&file, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
            file.to_string_lossy().into_owned()
        };
        let ask = ask_from("SaveFile", "org.example.App", "Save", &HashMap::new());
        let pid = AtomicI32::new(0);
        let saw = dir.join("saw");
        let answers = script("answers", &format!("[ \"$1\" = --choose ] || exit 3\ncat > {}\necho\necho '{{\"Chosen\":{{\"uris\":[\"file:///home/john/a.txt\"],\"choices\":[],\"filter\":null}}}}'", saw.display()));
        assert_eq!(choose_in_folders(&answers, &ask, &pid), Some(Outcome::Chosen { uris: vec!["file:///home/john/a.txt".into()], choices: vec![], filter: None }));
        let sent: serde_json::Value = serde_json::from_slice(&std::fs::read(&saw).unwrap()).unwrap();
        assert_eq!((sent["mode"].as_str(), sent["app_id"].as_str()), (Some("Save"), Some("org.example.App")));
        assert_eq!(pid.load(Ordering::SeqCst), 0, "the finished chooser's number was kept");
        assert_eq!(choose_in_folders(&script("cancels", "cat >/dev/null; echo '\"Cancelled\"'"), &ask, &pid), Some(Outcome::Cancelled));
        let since = std::time::Instant::now();
        assert_eq!(choose_in_folders(&script("lingers", "cat >/dev/null; echo '\"Cancelled\"'; exec sleep 5"), &ask, &pid), Some(Outcome::Cancelled));
        assert!(since.elapsed() < std::time::Duration::from_secs(3), "the program waited for the chooser to end");
        assert_eq!(choose_in_folders(&script("unreadable", "cat >/dev/null; exit 2"), &ask, &pid), None);
        assert_eq!(choose_in_folders(&script("garbled", "cat >/dev/null; echo nonsense"), &ask, &pid), None);
        assert_eq!(choose_in_folders(&dir.join("absent").to_string_lossy(), &ask, &pid), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn options_become_what_is_asked() {
        let rules = vec![(0u32, "*.png".to_string()), (1, "image/jpeg".to_string())];
        let options: Options = HashMap::from([
            ("accept_label".to_string(), owned("_Open")),
            ("multiple".to_string(), owned(true)),
            ("filters".to_string(), owned(vec![("Pictures".to_string(), rules.clone())])),
            ("current_filter".to_string(), owned(("Text".to_string(), vec![(0u32, "*.txt".to_string())]))),
            ("choices".to_string(), owned(vec![
                ("enc".to_string(), "Encoding".to_string(), vec![("u8".to_string(), "UTF-8".to_string())], "nope".to_string()),
                ("ro".to_string(), "Read only".to_string(), Vec::<(String, String)>::new(), "true".to_string()),
            ])),
            ("current_folder".to_string(), owned(b"/home/john\0".to_vec())),
        ]);
        let ask = ask_from("OpenFile", "org.example.App", "Pick", &options);
        assert_eq!(ask.mode, Mode::Open { many: true, folder: false });
        assert_eq!(ask.accept.as_deref(), Some("_Open"));
        assert_eq!(ask.filters.len(), 2);
        assert_eq!(ask.filters[0], Filter { name: "Pictures".into(), rules });
        assert_eq!(ask.filter, Some(1));
        assert_eq!(ask.choices[0].value, "u8");
        assert_eq!(ask.choices[1].value, "true");
        assert_eq!(ask.folder, Some(PathBuf::from("/home/john")));

        let options: Options = HashMap::from([("current_file".to_string(), owned(b"/tmp/notes.txt\0".to_vec()))]);
        let ask = ask_from("SaveFile", "x", "", &options);
        assert_eq!((ask.mode, ask.folder, ask.name.as_str()), (Mode::Save, Some(PathBuf::from("/tmp")), "notes.txt"));

        let options: Options = HashMap::from([("files".to_string(), owned(vec![b"/a/one.png\0".to_vec(), b"two.png\0".to_vec()]))]);
        assert_eq!(ask_from("SaveFiles", "x", "", &options).mode, Mode::SaveMany(vec!["one.png".into(), "two.png".into()]));
    }

    #[test]
    fn outcomes_become_results() {
        let (code, out) = results(Outcome::Chosen { uris: vec!["file:///tmp/a".into()], choices: vec![], filter: None });
        assert_eq!(code, DONE);
        assert!(out.contains_key("uris") && !out.contains_key("choices"));
        assert_eq!(results(Outcome::Cancelled).0, CANCELLED);
    }

    #[test]
    fn the_chooser_answers_on_the_bus() {
        use crate::toolkit::bus::{get_u32, testing::{own_daemon, until}};
        use std::cell::Cell;

        let d = own_daemon();
        std::env::remove_var("WAYLAND_DISPLAY");
        let lp = Loop::new();
        let (a, b) = (Bus::at(&lp, &d.1).expect("bus a"), Bus::at(&lp, &d.1).expect("bus b"));
        let stay = Rc::new(Stay { last: Cell::new(Instant::now()), busy: Cell::new(0), settled: Cell::new(false) });
        let _files = a.serve(super::super::portal::PATH, offer(&a, &lp, &stay)).expect("files");
        let who = a.unique_name();
        let ask = |iface: &'static str| b.call(&who, super::super::portal::PATH, iface);
        let got: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let wait = |lp: &Loop| {
            let g = got.clone();
            assert!(until(lp, move || g.borrow().is_some()));
            got.borrow_mut().take().unwrap_or_default()
        };

        let g = got.clone();
        ask(INTERFACE).get_all(&lp, move |r| *g.borrow_mut() = Some(r.map(|p| get_u32(&p, "version").to_string()).unwrap_or_default()));
        assert_eq!(wait(&lp), VERSION.to_string());

        let g = got.clone();
        ask("org.freedesktop.DBus.Introspectable").method("Introspect", &(), false, None, &lp, move |r| {
            *g.borrow_mut() = Some(r.ok().and_then(|m| m.body().deserialize::<String>().ok()).unwrap_or_default())
        });
        assert!(wait(&lp).contains(INTERFACE));

        let g = got.clone();
        let handle = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/t1").expect("path");
        let body = (handle, "org.example.App", "", "Open", HashMap::<String, Value>::new());
        ask(INTERFACE).method("OpenFile", &body, false, None, &lp, move |r| {
            *g.borrow_mut() = Some(r.ok().and_then(|m| m.body().deserialize::<(u32, HashMap<String, OwnedValue>)>().ok()).map_or("none".into(), |x| x.0.to_string()))
        });
        assert_eq!(wait(&lp), FAILED.to_string());
        assert_eq!(stay.busy.get(), 0);
    }
}
