//! Notifications.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zbus::zvariant::{Structure, Value};

use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};

use super::ctx::Ctx;

use crate::common::files::{set_setting, setting};
use crate::toolkit::t;

const NAME: &str = "org.freedesktop.Notifications";
const PATH: &str = "/org/freedesktop/Notifications";

const KEEP: usize = 60;

pub fn how_long_choices() -> [(&'static str, String); 6] {
    [
        ("3", t!("ring-notify-seconds", n = 3)),
        ("5", t!("ring-notify-seconds", n = 5)),
        ("8", t!("ring-notify-seconds", n = 8)),
        ("10", t!("ring-notify-seconds", n = 10)),
        ("15", t!("ring-notify-seconds", n = 15)),
        ("0", t!("ring-notify-until-closed").to_string()),
    ]
}
const LONG_DEFAULT: i32 = 5;

const EXPIRED: u32 = 1;
const DISMISSED: u32 = 2;
const CLOSED_BY_CALL: u32 = 3;

const XML: &str = r#"
  <interface name="org.freedesktop.Notifications">
    <method name="GetCapabilities">
      <arg type="as" direction="out"/>
    </method>
    <method name="Notify">
      <arg type="s" direction="in"/>
      <arg type="u" direction="in"/>
      <arg type="s" direction="in"/>
      <arg type="s" direction="in"/>
      <arg type="s" direction="in"/>
      <arg type="as" direction="in"/>
      <arg type="a{sv}" direction="in"/>
      <arg type="i" direction="in"/>
      <arg type="u" direction="out"/>
    </method>
    <method name="CloseNotification">
      <arg type="u" direction="in"/>
    </method>
    <method name="GetServerInformation">
      <arg type="s" direction="out"/>
      <arg type="s" direction="out"/>
      <arg type="s" direction="out"/>
      <arg type="s" direction="out"/>
    </method>
    <signal name="NotificationClosed"><arg type="u"/><arg type="u"/></signal>
    <signal name="ActionInvoked"><arg type="u"/><arg type="s"/></signal>
    <signal name="ActivationToken"><arg type="u"/><arg type="s"/></signal>
  </interface>
"#;

#[derive(Clone)]
pub struct Note {
    pub id: u32,
    pub app: String,
    pub icon: String,
    pub desktop: String,
    pub summary: String,
    pub body: String,
    pub at: crate::common::clock::Local,
    pub urgent: bool,
    pub pinned: bool,
    pub handoff: String,
    pub drive: String,
    pub allow: String,
}

impl Note {
    pub fn when(&self) -> String {
        let days = crate::common::clock::Local::now().day_number() - self.at.day_number();
        let clock = self.at.line(crate::toolkit::lang::format::Pattern::Clock);
        match days {
            0 => clock,
            1 => t!("ring-notify-yesterday", clock = clock),
            _ => self.at.line(crate::toolkit::lang::format::Pattern::WeekdayClock),
        }
    }
}

#[derive(Default)]
struct State {
    notes: Vec<Note>,
    next: u32,
    bus: Option<Bus>,
    served: Option<Served>,
    listeners: Vec<Rc<dyn Fn(&Note)>>,
    gone: Vec<Rc<dyn Fn(u32)>>,
    pins: Vec<Rc<dyn Fn(u32, bool)>>,
    actions: HashMap<u32, Vec<String>>,
    processes: HashMap<u32, String>,
    lp: Option<crate::toolkit::event::Loop>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { next: 1, ..State::default() });
}

pub fn list() -> Vec<Note> {
    STATE.with(|s| s.borrow().notes.clone())
}

pub fn forget(id: u32) {
    STATE.with(|s| s.borrow_mut().notes.retain(|n| n.id != id));
    closed(id, DISMISSED);
}

pub fn set_pinned(id: u32, pinned: bool) {
    let changed = STATE.with(|s| {
        let mut st = s.borrow_mut();
        match st.notes.iter_mut().find(|n| n.id == id) {
            Some(n) if n.pinned != pinned => {
                n.pinned = pinned;
                true
            }
            _ => false,
        }
    });
    if changed {
        let told: Vec<Rc<dyn Fn(u32, bool)>> = STATE.with(|s| s.borrow().pins.clone());
        for f in told {
            f(id, pinned);
        }
    }
}

pub fn on_pinned(f: impl Fn(u32, bool) + 'static) {
    STATE.with(|s| s.borrow_mut().pins.push(Rc::new(f)));
}

pub fn clear() {
    let ids: Vec<u32> = STATE.with(|s| {
        let mut st = s.borrow_mut();
        let ids = st.notes.iter().filter(|n| !n.pinned).map(|n| n.id).collect();
        st.notes.retain(|n| n.pinned);
        ids
    });
    for id in ids {
        closed(id, DISMISSED);
    }
}

pub fn on_new(f: impl Fn(&Note) + 'static) {
    STATE.with(|s| s.borrow_mut().listeners.push(Rc::new(f)));
}

pub fn on_gone(f: impl Fn(u32) + 'static) {
    STATE.with(|s| s.borrow_mut().gone.push(Rc::new(f)));
}

pub fn showing() -> bool {
    setting("notifications") != "off"
}

pub fn set_showing(on: bool) -> std::io::Result<()> {
    set_setting("notifications", if on { "on" } else { "off" })
}

pub fn how_long(urgent: bool) -> i32 {
    if urgent {
        return 0;
    }
    let saved = setting("notification_seconds");
    match how_long_choices().iter().any(|(v, _)| *v == saved) {
        true => saved.parse().unwrap_or(LONG_DEFAULT),
        false => LONG_DEFAULT,
    }
}

pub fn how_long_setting() -> String {
    let saved = setting("notification_seconds");
    match how_long_choices().iter().any(|(v, _)| *v == saved) {
        true => saved,
        false => LONG_DEFAULT.to_string(),
    }
}

pub fn corner_choices() -> [(&'static str, &'static str); 4] {
    [
        ("bottom-right", t!("ring-notify-bottom-right")),
        ("bottom-left", t!("ring-notify-bottom-left")),
        ("top-left", t!("ring-notify-top-left")),
        ("top-right", t!("ring-notify-top-right")),
    ]
}

pub fn corner_setting() -> String {
    let saved = setting("notification_corner");
    match corner_choices().iter().any(|(v, _)| *v == saved) {
        true => saved,
        false => corner_choices()[0].0.to_string(),
    }
}

pub fn set_corner(value: &str) -> std::io::Result<()> {
    set_setting("notification_corner", value)
}

pub fn set_how_long(value: &str) -> std::io::Result<()> {
    set_setting("notification_seconds", value)
}

fn closed(id: u32, why: u32) {
    let bus = STATE.with(|s| {
        let mut st = s.borrow_mut();
        st.actions.remove(&id);
        st.processes.remove(&id);
        st.bus.clone()
    });
    if let Some(bus) = bus {
        let _ = bus.emit(PATH, NAME, "NotificationClosed", &(id, why));
    }
}

fn text(fields: &[Value], i: usize) -> String {
    match fields.get(i) {
        Some(Value::Str(s)) => s.as_str().to_string(),
        _ => String::new(),
    }
}

pub const SUMMARY_MOST: usize = 200;
pub const BODY_MOST: usize = 2000;

fn clipped(text: String, most: usize) -> String {
    match text.char_indices().nth(most) {
        Some((at, _)) => format!("{}\u{2026}", &text[..at]),
        None => text,
    }
}

fn arrived(fields: &[Value]) -> Note {
    let hint = |key: &str| -> Option<Value> {
        let Some(Value::Dict(d)) = fields.get(6) else { return None };
        d.iter().find(|(k, _)| matches!(k, Value::Str(s) if s.as_str() == key)).map(|(_, v)| match v {
            Value::Value(inner) => (**inner).clone(),
            v => v.clone(),
        })
    };
    let hint_string = |key: &str| match hint(key) {
        Some(Value::Str(s)) => s.as_str().to_string(),
        _ => String::new(),
    };
    let urgency = match hint("urgency") {
        Some(Value::U8(n)) => n,
        _ => 1,
    };
    let mut icon = text(fields, 2);
    if icon.is_empty() {
        icon = hint_string("image-path");
    }
    Note {
        id: 0,
        app: text(fields, 0),
        icon,
        desktop: hint_string("desktop-entry"),
        summary: clipped(text(fields, 3), SUMMARY_MOST),
        body: clipped(text(fields, 4), BODY_MOST),
        at: crate::common::clock::Local::now(),
        urgent: urgency >= 2,
        pinned: false,
        handoff: String::new(),
        drive: String::new(),
        allow: String::new(),
    }
}

fn keep(mut note: Note, replaces: u32) -> Note {
    let dropped = STATE.with(|s| {
        let mut st = s.borrow_mut();
        note.id = if replaces > 0 { replaces } else { st.next };
        if note.id >= st.next {
            st.next = note.id + 1;
        }
        note.pinned = st.notes.iter().any(|n| n.id == note.id && n.pinned);
        st.notes.retain(|n| n.id != note.id);
        st.notes.insert(0, note.clone());
        let mut dropped = Vec::new();
        while st.notes.len() > KEEP {
            match st.notes.iter().rposition(|n| !n.pinned) {
                Some(i) => dropped.push(st.notes.remove(i).id),
                None => break,
            }
        }
        dropped
    });
    for id in dropped {
        closed(id, EXPIRED);
    }
    note
}

pub fn restore(app: &str, icon: &str, desktop: &str, summary: &str, body: &str, pinned: bool) -> Note {
    let note = Note {
        id: 0, app: clipped(app.into(), SUMMARY_MOST), icon: icon.into(), desktop: desktop.into(),
        summary: clipped(summary.into(), SUMMARY_MOST), body: clipped(body.into(), BODY_MOST),
        at: crate::common::clock::Local::now(), urgent: false, pinned: false,
        handoff: String::new(), drive: String::new(), allow: String::new(),
    };
    let mut note = keep(note, 0);
    if pinned {
        set_pinned(note.id, true);
        note.pinned = true;
    }
    note
}

pub fn say_here(summary: &str, body: &str) {
    say_with(summary, body, crate::settings::ui::UPDATE_ICON, String::new());
}

pub fn say_crash(summary: &str, body: &str, handoff: String) {
    say_with(summary, body, "dialog-warning-symbolic", handoff);
}

pub fn say_as(app: &str, icon: &str, summary: &str, body: &str) {
    say_from(app, summary, body, icon, String::new());
}

fn say_with(summary: &str, body: &str, icon: &str, handoff: String) {
    say_from("Trindle", summary, body, icon, handoff);
}

fn say_from(app: &str, summary: &str, body: &str, icon: &str, handoff: String) {
    let note = Note {
        id: 0,
        app: app.into(),
        icon: icon.into(),
        desktop: String::new(),
        summary: summary.into(),
        body: body.into(),
        at: crate::common::clock::Local::now(),
        urgent: false,
        pinned: false,
        handoff,
        drive: String::new(),
        allow: String::new(),
    };
    let note = keep(note, 0);
    tell_everyone(&note);
}

pub fn say_allow(app: &str, icon: &str, summary: &str, body: &str, plugin: &str, replaces: u32) -> u32 {
    let note = Note {
        id: 0,
        app: app.into(),
        icon: icon.into(),
        desktop: String::new(),
        summary: summary.into(),
        body: body.into(),
        at: crate::common::clock::Local::now(),
        urgent: false,
        pinned: false,
        handoff: String::new(),
        drive: String::new(),
        allow: plugin.into(),
    };
    let note = keep(note, replaces);
    tell_everyone(&note);
    note.id
}

pub fn say_drive(summary: &str, drive: &str) -> u32 {
    let note = Note {
        id: 0,
        app: "Trindle".into(),
        icon: "drive-removable-media-symbolic".into(),
        desktop: String::new(),
        summary: summary.into(),
        body: String::new(),
        at: crate::common::clock::Local::now(),
        urgent: false,
        pinned: false,
        handoff: String::new(),
        drive: drive.into(),
        allow: String::new(),
    };
    let note = keep(note, 0);
    tell_everyone(&note);
    note.id
}

pub fn take_back(id: u32) {
    forget(id);
    let gone = STATE.with(|s| s.borrow().gone.clone());
    for g in gone {
        g(id);
    }
}

pub fn has_default(id: u32) -> bool {
    STATE.with(|s| s.borrow().actions.get(&id).is_some_and(|keys| keys.iter().any(|k| k == "default")))
}

pub fn process_of(id: u32) -> Option<String> {
    STATE.with(|s| s.borrow().processes.get(&id).cloned())
}

fn learn_process(id: u32, sender: &str) {
    let (bus, lp) = STATE.with(|s| {
        let st = s.borrow();
        (st.bus.clone(), st.lp.clone())
    });
    let (Some(bus), Some(lp)) = (bus, lp) else { return };
    if sender.is_empty() {
        return;
    }
    bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus")
        .method("GetConnectionUnixProcessID", &(sender,), false, None, &lp, move |r| {
            let Some(pid) = r.ok().and_then(|m| m.body().deserialize::<u32>().ok()) else { return };
            let Ok(name) = std::fs::read_to_string(format!("/proc/{pid}/comm")) else { return };
            let name = name.trim().to_string();
            STATE.with(|s| {
                let mut st = s.borrow_mut();
                if st.notes.iter().any(|n| n.id == id) {
                    st.processes.insert(id, name);
                }
            });
        });
}

pub fn clicked(id: u32) -> bool {
    let (bus, wanted) = STATE.with(|s| (s.borrow().bus.clone(), has_default(id)));
    let Some(bus) = bus.filter(|_| wanted) else { return false };
    crate::common::run::vouched(move |token| {
        for signal in signals(id, token) {
            let _ = match signal {
                Signal::Token(id, t) => bus.emit(PATH, NAME, "ActivationToken", &(id, t)),
                Signal::Action(id) => bus.emit(PATH, NAME, "ActionInvoked", &(id, "default")),
            };
        }
    });
    true
}

#[derive(Debug, PartialEq)]
enum Signal {
    Token(u32, String),
    Action(u32),
}

fn signals(id: u32, token: Option<String>) -> Vec<Signal> {
    token.into_iter().map(|t| Signal::Token(id, t)).chain(std::iter::once(Signal::Action(id))).collect()
}

fn tell_everyone(note: &Note) {
    let listeners = STATE.with(|s| s.borrow().listeners.clone());
    for l in listeners {
        l(note);
    }
}

pub fn start(ctx: &Rc<Ctx>) {
    let Some(bus) = ctx.session.clone() else { return };
    let offer = Offer {
        name: NAME.into(),
        xml: XML.trim().into(),
        call: Box::new(answer),
        props: Box::new(Props::new),
        set: None,
    };
    let Ok(served) = bus.serve(PATH, offer) else { return };
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        st.bus = Some(bus.clone());
        st.served = Some(served);
        st.lp = Some(ctx.lp.clone());
    });
    const ALLOW_REPLACEMENT_AND_REPLACE: u32 = 1 | 2;
    bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus")
        .method("RequestName", &(NAME, ALLOW_REPLACEMENT_AND_REPLACE), false, None, &ctx.lp, |r| {
            let got = r.ok().and_then(|m| m.body().deserialize::<u32>().ok());
            if !matches!(got, Some(1) | Some(4)) {
                eprintln!("trindle-ring: something else is already answering notifications");
            }
        });
}

fn answer(asked: Asked) {
    match asked.member().as_str() {
        "Notify" => {
            let note = {
                let body = asked.message().body();
                let fields = body.deserialize::<Structure>().map(|s| s.into_fields()).unwrap_or_default();
                let replaces = match fields.get(1) {
                    Some(Value::U32(n)) => *n,
                    _ => 0,
                };
                let note = keep(arrived(&fields), replaces);
                let keys: Vec<String> = match fields.get(5) {
                    Some(Value::Array(a)) => a.iter().step_by(2).filter_map(|v| match v {
                        Value::Str(s) => Some(s.as_str().to_string()),
                        _ => None,
                    }).collect(),
                    _ => Vec::new(),
                };
                STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    if keys.is_empty() {
                        st.actions.remove(&note.id);
                    } else {
                        st.actions.insert(note.id, keys);
                    }
                });
                note
            };
            learn_process(note.id, &asked.sender());
            asked.reply(&(note.id,));
            tell_everyone(&note);
        }
        "CloseNotification" => {
            let id = asked.message().body().deserialize::<u32>().unwrap_or(0);
            STATE.with(|s| s.borrow_mut().notes.retain(|n| n.id != id));
            asked.reply(&());
            closed(id, CLOSED_BY_CALL);
            let gone = STATE.with(|s| s.borrow().gone.clone());
            for g in gone {
                g(id);
            }
        }
        "GetCapabilities" => {
            let caps = vec!["body".to_string(), "icon-static".to_string(), "persistence".to_string()];
            asked.reply(&(caps,));
        }
        "GetServerInformation" => {
            asked.reply(&("Trindle", "Trindle", env!("CARGO_PKG_VERSION"), "1.2"));
        }
        _ => asked.reply(&()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zbus::zvariant::StructureBuilder;

    #[test]
    fn the_time_of_a_note_follows_the_language() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let mut note = Note { id: 1, app: String::new(), icon: String::new(), desktop: String::new(), summary: String::new(), body: String::new(),
            at: crate::common::clock::Local::now(), urgent: false, pinned: false, handoff: String::new(), drive: String::new(), allow: String::new() };
        let made_up = "kit-date-pattern-clock = %H.%M\nkit-date-pattern-weekday-clock = %H.%M Zorday\nring-notify-yesterday = Hierau { $clock }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(note.when(), note.at.format("%H.%M"));
            note.at = crate::common::clock::Local::at(note.at.unix - 3 * 86_400);
            assert!(note.when().ends_with(" Zorday"), "{}", note.when());
        });
    }

    #[test]
    fn a_click_hands_the_pass_to_come_forward_before_the_action() {
        assert_eq!(signals(7, Some("abc".into())), [Signal::Token(7, "abc".into()), Signal::Action(7)]);
        assert_eq!(signals(7, None), [Signal::Action(7)]);
    }

    #[test]
    fn a_huge_title_and_body_are_clipped_when_they_arrive() {
        let big = "w".repeat(1_000_000);
        let mut fields = sent(-1, 1);
        fields[3] = Value::new(big.clone());
        fields[4] = Value::new(big);
        let n = arrived(&fields);
        assert_eq!(n.summary.chars().count(), SUMMARY_MOST + 1);
        assert_eq!(n.body.chars().count(), BODY_MOST + 1);
        assert!(n.body.ends_with('\u{2026}'));
        assert_eq!(clipped("short".into(), 10), "short");
        assert_eq!(clipped("é".repeat(5), 5), "ééééé");
    }

    fn sent(timeout: i32, urgency: u8) -> Vec<Value<'static>> {
        let mut hints: HashMap<String, Value<'static>> = HashMap::new();
        hints.insert("urgency".into(), Value::new(urgency));
        hints.insert("desktop-entry".into(), Value::new("org.mozilla.firefox"));
        StructureBuilder::new()
            .add_field("Firefox".to_string())
            .add_field(0u32)
            .add_field("firefox".to_string())
            .add_field("One new message".to_string())
            .add_field("From Jo".to_string())
            .add_field(Vec::<String>::new())
            .add_field(hints)
            .add_field(timeout)
            .build()
            .expect("a notification")
            .into_fields()
    }

    #[test]
    fn the_interface_says_what_the_specification_says() {
        let shape = StructureBuilder::new().add_field("").add_field(0u32).add_field("").add_field("").add_field("")
            .add_field(Vec::<String>::new()).add_field(HashMap::<String, Value>::new()).add_field(0i32)
            .build().expect("a notification");
        assert_eq!(shape.signature().to_string(), "(susssasa{sv}i)");
        for m in ["Notify", "CloseNotification", "GetCapabilities", "GetServerInformation", "NotificationClosed"] {
            assert!(XML.contains(&format!("name=\"{m}\"")), "{m} is missing");
        }
    }

    #[test]
    fn a_pinned_one_is_never_cleared_and_never_falls_off_the_end() {
        let first = keep(arrived(&sent(-1, 1)), 0);
        set_pinned(first.id, true);
        for _ in 0..(KEEP + 20) {
            keep(arrived(&sent(-1, 1)), 0);
        }
        let held = list();
        assert_eq!(held.len(), KEEP);
        assert!(held.iter().any(|n| n.id == first.id), "the pinned one was pushed off the end");
        clear();
        let left = list();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].id, first.id);
        set_pinned(first.id, false);
        clear();
        assert!(list().is_empty());
    }

    #[test]
    fn rubbish_in_the_hints_is_not_a_reason_to_fall_over() {
        let odd = StructureBuilder::new().add_field("App".to_string()).add_field(0u32).add_field(String::new())
            .add_field("Hello".to_string()).add_field(String::new()).add_field(Vec::<String>::new())
            .add_field(7i32).add_field(-1i32).build().expect("a notification").into_fields();
        let n = arrived(&odd);
        assert_eq!(n.summary, "Hello");
        assert!(!n.urgent);
        assert_eq!(arrived(&[]).summary, "");
    }

    #[test]
    fn what_a_program_sends_becomes_a_note() {
        let n = arrived(&sent(6000, 1));
        assert_eq!(n.app, "Firefox");
        assert_eq!(n.summary, "One new message");
        assert_eq!(n.body, "From Jo");
        assert_eq!(n.desktop, "org.mozilla.firefox");
        assert_eq!(n.icon, "firefox");
        assert!(!n.urgent);
    }

    #[test]
    fn how_long_it_stays_is_the_persons_choice_not_the_programs() {
        for asked in [200, 600_000, -1, 0] {
            let n = arrived(&sent(asked, 1));
            assert!(!n.urgent);
            assert!(!n.pinned);
        }
        assert!(arrived(&sent(0, 2)).urgent);
        assert_eq!(how_long(true), 0);
        for (value, _) in how_long_choices() {
            assert!(value.parse::<i32>().is_ok(), "{value} is not a length");
        }
    }
}
