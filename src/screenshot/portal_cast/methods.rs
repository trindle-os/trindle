//! The methods of the ScreenCast interface: a session opened, sources selected, sharing started.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use crate::toolkit::t;
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::ui::Shown;
use crate::toolkit::window::Kind;

use super::{cast_capture, cast_pick, sender_of, tell_in_use, Cast, Casting, EMBEDDED, Follow, HIDDEN, Kept, METADATA, MONITOR, Part, Pointer, RESTORE_VERSION, SESSION, SESSION_XML, START_WAIT, Session, VENDOR, WINDOW, Wants};
use super::desktop::{desktop_layout, kept, recall};
use super::super::cast_capture::{Capture, Source};
use super::super::cast_pick::{Choice, Picked, Picker};
use super::super::portal::{answer, CANCELLED, DONE, FAILED};
use super::super::portal_ask::{get, APP_ID, Answer, Options, REQUEST, REQUEST_XML};

fn app_label(app_id: &str, pid: Option<u32>) -> String {
    if !app_id.is_empty() {
        let name = super::super::portal_ask::app_name(app_id);
        if !name.is_empty() {
            return name;
        }
    }
    let comm = pid.and_then(|p| std::fs::read_to_string(format!("/proc/{p}/comm")).ok()).map(|c| c.trim().to_string());
    match comm {
        Some(c) if !c.is_empty() => {
            let apps = crate::common::apps::entries();
            crate::common::apps::find_for_window(&apps, &c).map(|e| e.name.clone()).unwrap_or(c)
        }
        _ => t!("screenshot-cast-an-app").into(),
    }
}

fn pid_of(bus: &Bus, events: &Loop, name: &str, done: impl FnOnce(Option<u32>) + 'static) {
    bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus")
        .method("GetConnectionUnixProcessID", &(name,), false, Some(Duration::from_secs(2)), events, move |r| {
            done(r.ok().and_then(|m| m.body().deserialize::<u32>().ok()))
        });
}

pub(super) fn create_session(casting: &Rc<Casting>, asked: Asked) {
    let Ok((_handle, session, app_id, _options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, OwnedObjectPath, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "CreateSession takes two handles, an app and options");
    };
    let path = session.as_str().to_string();
    let s = Rc::new(Session {
        path: path.clone(),
        app_name: app_label(&app_id, None),
        app_pid: Cell::new(None),
        wants: RefCell::new(Wants::default()),
        casts: RefCell::new(Vec::new()),
        served: RefCell::new(None),
        picker: RefCell::new(None),
        request: RefCell::new(None),
        started: Cell::new(false),
        closed: Cell::new(false),
    });
    let (me, weak) = (Rc::downgrade(casting), Rc::downgrade(&s));
    let served = casting.kit.bus.serve(&path, Offer {
        name: SESSION.into(),
        xml: SESSION_XML.into(),
        call: Box::new(move |asked: Asked| {
            asked.reply(&());
            if let (Some(me), Some(s)) = (me.upgrade(), weak.upgrade()) {
                me.end(&s, false);
            }
        }),
        props: Box::new(|| Props::from([("version".to_string(), OwnedValue::from(1u32))])),
        set: None,
    });
    let Ok(served) = served else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    *s.served.borrow_mut() = Some(served);
    casting.kit.hold();
    casting.sessions.borrow_mut().insert(path.clone(), s.clone());
    if let Some(sender) = sender_of(&path) {
        let weak = Rc::downgrade(&s);
        pid_of(&casting.kit.bus, &casting.kit.events, &sender, move |pid| {
            if let Some(s) = weak.upgrade() {
                s.app_pid.set(pid);
            }
        });
    }
    asked.reply(&answer(DONE, HashMap::new()));
}

pub(super) fn restore_of(options: &Options) -> Option<Vec<Kept>> {
    let Value::Structure(s) = &**options.get("restore_data")? else { return None };
    let [Value::Str(vendor), Value::U32(version), data] = s.fields() else { return None };
    if vendor.as_str() != VENDOR || *version != RESTORE_VERSION {
        return None;
    }
    let data = match data {
        Value::Value(inner) => &**inner,
        other => other,
    };
    let Value::Array(items) = data else { return None };
    items.iter().map(|item| {
        let Value::Structure(s) = item else { return None };
        let text = |v: &Value| match v {
            Value::Str(t) => Some(t.to_string()),
            _ => None,
        };
        match s.fields() {
            [a, b, c, d] => Some((text(a)?, text(b)?, text(c)?, text(d)?)),
            _ => None,
        }
    }).collect()
}

pub(super) fn select_sources(casting: &Rc<Casting>, asked: Asked) {
    let Ok((_handle, session, _app_id, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, OwnedObjectPath, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "SelectSources takes two handles, an app and options");
    };
    let Some(s) = casting.sessions.borrow().get(session.as_str()).cloned() else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    let mut wants = s.wants.borrow_mut();
    if let Some(types) = get::<u32>(&options, "types").filter(|t| *t != 0) {
        wants.types = types;
    }
    wants.multiple = get::<bool>(&options, "multiple").unwrap_or(false);
    if let Some(cursor) = get::<u32>(&options, "cursor_mode").filter(|c| matches!(*c, HIDDEN | EMBEDDED | METADATA)) {
        wants.cursor = cursor;
    }
    wants.persist = get::<u32>(&options, "persist_mode").unwrap_or(0).min(2);
    wants.restore = restore_of(&options);
    drop(wants);
    asked.reply(&answer(DONE, HashMap::new()));
}

pub(super) fn start(casting: &Rc<Casting>, asked: Asked) {
    let Ok((handle, session, _app_id, parent, _options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, OwnedObjectPath, String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "Start takes two handles, an app, a window and options");
    };
    let Some(s) = casting.sessions.borrow().get(session.as_str()).cloned() else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    if s.started.get() || s.closed.get() {
        return asked.reply(&answer(FAILED, HashMap::new()));
    }
    let Some(capture) = casting.capture() else {
        eprintln!("trindle-portal: the compositor cannot share the screen");
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    let wants = s.wants.borrow().clone();
    let screens = capture.screens();
    let windows = if wants.types & WINDOW != 0 && capture.has_windows() { capture.windows() } else { Vec::new() };
    if let Some(choices) = wants.restore.as_deref().and_then(|k| recall(k, &screens, &windows)) {
        return begin(casting, &s, asked, choices);
    }
    let choices = choices_to_offer(&wants, screens, windows);
    ask_which(casting, &s, asked, &capture, choices, (handle.as_str(), &parent));
}

fn choices_to_offer(wants: &Wants, screens: Vec<cast_capture::Screen>, windows: Vec<cast_capture::Window>) -> Vec<Choice> {
    let apps = crate::common::apps::entries();
    let mut choices = Vec::new();
    if wants.types & MONITOR != 0 {
        if screens.len() > 1 {
            choices.push(Choice::Desktop);
        }
        choices.extend(screens.into_iter().map(Choice::Screen));
    }
    choices.extend(windows.into_iter().map(|w| {
        let icon = crate::common::apps::find_for_window(&apps, &w.app_id).and_then(|e| e.icon.clone());
        Choice::Window(w, icon)
    }));
    choices
}

fn ask_which(casting: &Rc<Casting>, s: &Rc<Session>, asked: Asked, capture: &Rc<Capture>, choices: Vec<Choice>, (handle, parent): (&str, &str)) {

    let asked = Rc::new(RefCell::new(Some(asked)));
    let (me, weak, a) = (Rc::downgrade(casting), Rc::downgrade(s), asked.clone());
    let decide = move |picked: Picked| {
        let Some(asked) = a.borrow_mut().take() else { return };
        let (Some(me), Some(s)) = (me.upgrade(), weak.upgrade()) else { return };
        let picker = s.picker.borrow_mut().take();
        let request = s.request.borrow_mut().take();
        me.kit.events.after(Duration::ZERO, move || drop((picker, request)));
        match picked {
            Some(choices) => begin(&me, &s, asked, choices),
            None => asked.reply(&answer(CANCELLED, HashMap::new())),
        }
    };
    let decide: Rc<dyn Fn(Picked)> = Rc::new(decide);
    let wants = s.wants.borrow().clone();
    let picker = Picker::new(s.app_name.clone(), choices.clone(), wants.multiple, {
        let d = decide.clone();
        move |p| d(p)
    });
    let Some(display) = casting.kit.screen() else {
        if let Some(asked) = asked.borrow_mut().take() {
            asked.reply(&answer(FAILED, HashMap::new()));
        }
        return;
    };
    let kind = Kind::Window { title: t!("screenshot-cast-window").into(), app_id: APP_ID.into() };
    let Some(shown) = Shown::open(&casting.kit.events, &display, kind, cast_pick::SIZE, picker) else {
        decide(None);
        return;
    };
    let shown = Rc::new(shown);
    if let Some(handle) = parent.strip_prefix("wayland:") {
        shown.window.set_parent(handle);
    }
    {
        let d = decide.clone();
        shown.window.on_close(move || d(None));
    }
    let close = serve_request(casting, handle, decide);
    show_pictures(capture, &choices, &shown);

    *s.picker.borrow_mut() = Some(shown);
    *s.request.borrow_mut() = close;
}

fn serve_request(casting: &Rc<Casting>, handle: &str, decide: Rc<dyn Fn(Picked)>) -> Option<Served> {
    casting.kit.bus.serve(handle, Offer {
        name: REQUEST.into(),
        xml: REQUEST_XML.into(),
        call: Box::new(move |asked: Asked| {
            asked.reply(&());
            decide(None);
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok()
}

fn show_pictures(capture: &Rc<Capture>, choices: &[Choice], shown: &Rc<Shown<Picker>>) {
    for (i, choice) in choices.iter().enumerate() {
        let source = match choice {
            Choice::Desktop => continue,
            Choice::Screen(sc) => Source::Screen(sc.name.clone()),
            Choice::Window(w, _) => Source::Window(w.id.clone()),
        };
        let weak = Rc::downgrade(shown);
        capture.snapshot(&source, 400, move |picture| {
            let (Some(shown), Some(picture)) = (weak.upgrade(), picture) else { return };
            shown.change(|p| {
                if let Some(slot) = p.pictures.get_mut(i) {
                    *slot = Some(Rc::new(picture));
                }
            });
        });
    }
}

fn begin(casting: &Rc<Casting>, s: &Rc<Session>, asked: Asked, choices: Vec<Choice>) {
    let Some(capture) = casting.capture() else {
        return asked.reply(&answer(FAILED, HashMap::new()));
    };
    let wants = s.wants.borrow().clone();
    let screens = capture.screens();
    for choice in &choices {
        start_cast(casting, s, &capture, &wants, &screens, choice);
    }
    s.started.set(true);
    answer_when_ready(casting, s, asked);
}

fn start_cast(casting: &Rc<Casting>, s: &Rc<Session>, capture: &Rc<Capture>, wants: &Wants, screens: &[cast_capture::Screen], choice: &Choice) {
    let (sources, desktop, size, at): (Vec<Source>, bool, (u32, u32), Vec<(i32, i32, u32, u32)>) = match choice {
        Choice::Desktop => {
            let (size, at) = desktop_layout(screens);
            (screens.iter().map(|sc| Source::Screen(sc.name.clone())).collect(), true, size, at)
        }
        Choice::Screen(sc) => (vec![Source::Screen(sc.name.clone())], false, (0, 0), vec![(0, 0, 0, 0)]),
        Choice::Window(w, _) => (vec![Source::Window(w.id.clone())], false, (0, 0), vec![(0, 0, 0, 0)]),
    };
    let parts = sources.iter().zip(at).map(|(_, at)| Part {
        capture: None, size: (0, 0), at, memory: None, buffer: None, busy: false, fresh: Vec::new(),
    }).collect();
    let cast = Rc::new(Cast::new(choice.clone(), parts, desktop, size, (wants.cursor == METADATA).then(Pointer::default)));
    s.casts.borrow_mut().push(cast.clone());
    let pointer = wants.cursor == EMBEDDED;
    if wants.cursor == METADATA {
        for (index, source) in sources.iter().enumerate() {
            let (me, cw) = (Rc::downgrade(casting), Rc::downgrade(&cast));
            let id = capture.open_pointer(source, move |heard| {
                let (Some(me), Some(cast)) = (me.upgrade(), cw.upgrade()) else { return };
                me.pointer_heard(&cast, index, heard);
            });
            if let Some(p) = cast.pointer.borrow_mut().as_mut() {
                p.follows.push(Follow { id, ..Follow::default() });
            }
        }
    }
    for (index, source) in sources.iter().enumerate() {
        let (me, sw, cw) = (Rc::downgrade(casting), Rc::downgrade(s), Rc::downgrade(&cast));
        let id = capture.open(source, pointer, move |heard| {
            let (Some(me), Some(s), Some(cast)) = (me.upgrade(), sw.upgrade(), cw.upgrade()) else { return };
            me.heard(&s, &cast, index, heard);
        });
        match id {
            Some(id) => cast.parts.borrow_mut()[index].capture = Some(id),
            None => cast.failed.set(true),
        }
    }
}

fn answer_when_ready(casting: &Rc<Casting>, s: &Rc<Session>, asked: Asked) {
    let asked = Rc::new(RefCell::new(Some(asked)));
    let (me, sw, waited) = (Rc::downgrade(casting), Rc::downgrade(s), Rc::new(Cell::new(Duration::ZERO)));
    let step = Duration::from_millis(20);
    let check = move || -> Flow {
        let (Some(me), Some(s)) = (me.upgrade(), sw.upgrade()) else { return Flow::Stop };
        waited.set(waited.get() + step);
        let casts = s.casts.borrow().clone();
        let failed = s.closed.get() || casts.is_empty() || casts.iter().any(|c| c.failed.get());
        let ready = casts.iter().all(|c| c.node.get().is_some());
        if !failed && !ready && waited.get() < START_WAIT {
            return Flow::Continue;
        }
        let Some(asked) = asked.borrow_mut().take() else { return Flow::Stop };
        if failed || !ready {
            asked.reply(&answer(FAILED, HashMap::new()));
            me.end(&s, true);
            return Flow::Stop;
        }
        asked.reply(&started(&s, &casts, &screens_now(&me)));
        tell_in_use(&me);
        Flow::Stop
    };
    casting.kit.events.every(step, check);
}

fn screens_now(casting: &Casting) -> Vec<cast_capture::Screen> {
    casting.capture.borrow().as_ref().map(|c| c.screens()).unwrap_or_default()
}

pub(super) fn started(s: &Session, casts: &[Rc<Cast>], screens: &[cast_capture::Screen]) -> Answer {
    let streams: Vec<(u32, HashMap<String, OwnedValue>)> = casts.iter().map(|c| {
        let mut props: HashMap<String, OwnedValue> = HashMap::new();
        let (kind, place) = match &c.choice {
            Choice::Desktop => {
                let x0 = screens.iter().map(|s| s.place[0]).min().unwrap_or(0);
                let y0 = screens.iter().map(|s| s.place[1]).min().unwrap_or(0);
                let x1 = screens.iter().map(|s| s.place[0] + s.place[2]).max().unwrap_or(0);
                let y1 = screens.iter().map(|s| s.place[1] + s.place[3]).max().unwrap_or(0);
                (MONITOR, Some([x0, y0, x1 - x0, y1 - y0]))
            }
            Choice::Screen(sc) => (MONITOR, screens.iter().find(|x| x.name == sc.name).map(|x| x.place)),
            Choice::Window(..) => (WINDOW, None),
        };
        props.insert("source_type".into(), OwnedValue::from(kind));
        match place {
            Some(p) => {
                props.insert("position".into(), Value::from((p[0], p[1])).try_into().expect("plain value"));
                props.insert("size".into(), Value::from((p[2], p[3])).try_into().expect("plain value"));
            }
            None => {
                let (w, h) = c.size.get();
                props.insert("size".into(), Value::from((w as i32, h as i32)).try_into().expect("plain value"));
            }
        }
        (c.node.get().unwrap_or(0), props)
    }).collect();
    let mut results: HashMap<&'static str, Value<'static>> = HashMap::from([("streams", Value::from(streams))]);
    let persist = s.wants.borrow().persist;
    if persist > 0 {
        let keep: Vec<Kept> = casts.iter().map(|c| kept(&c.choice)).collect();
        results.insert("persist_mode", Value::from(persist));
        results.insert("restore_data", Value::from((VENDOR.to_string(), RESTORE_VERSION, Value::from(keep))));
    }
    answer(DONE, results)
}
