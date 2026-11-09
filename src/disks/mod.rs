//! Drives, a window of its own, through UDisks2.

mod actions;
mod app;
mod cards;
pub mod help;
mod job;
mod map;
mod model;
pub(crate) mod plan;
mod refresh;
mod room;
mod rows;
mod steps;
mod udisks;
mod view;

use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::time::Duration;

use crate::common::hidden_drives;
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props, Served};
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::t;
use crate::toolkit::ui::Shown;
use crate::toolkit::widget::{
                             note, row, Length, Node, Rect, Tone};
use crate::toolkit::window::{Display, Kind};
pub(crate) use plan::size_text;
pub(crate) use udisks::{mount_points, num, text};
use crate::disks::udisks::UD;
use crate::disks::udisks::UD_PATH;
use crate::disks::udisks::OBJECT_MANAGER;
use crate::disks::udisks::Objects;
use crate::disks::udisks::take_snapshot;
use crate::disks::model::Drive;
use crate::disks::model::Caps;
use crate::disks::steps::Said;
use crate::disks::steps::Carried;
use crate::disks::steps::noted_folder;

const NAME: &str = "org.trindle.Drives";
const PATH: &str = "/org/trindle/Drives";
const XML: &str = r#"<interface name="org.trindle.Drives"><method name="Show"/></interface>"#;
const SIZE: (u32, u32) = (1280, 800);

const INTERACTIVE: Duration = Duration::from_secs(120);

const LONG: Duration = Duration::from_secs(3600);
const SAVE_ICON: &str = "media-floppy-symbolic";
const GB: f64 = 1e9;

#[derive(Clone)]
enum Msg {

    Idle,

    Toggle(String),

    Help,

    More(&'static str),

    Retry,
    Text(String, String),
    Pick(String, usize),
    Gb(String, f32),
    Startup { drive: String, vol: String, on: bool },

    Hide { drive: String, row: String, key: String, hide: bool },
    Act(Act),

    Weigh,
}

fn red(words: impl Into<String>, page: &'static str) -> Node<Msg> {
    row(vec![note(words.into()).tone(Tone::Problem).width(Length::Fill), crate::common::more::button(Msg::More(page))]).gap(8.0).centred()
}

#[derive(Clone)]
enum Act {
    Off { drive: String, method: &'static str },
    Mount { drive: String, vol: String, on: bool },
    Open(String),
    Rename { drive: String, vol: String },
    Folder { drive: String, vol: String },
    Resize { drive: String, vol: String },

    Room { drive: String, vol: String },
    Check { drive: String, vol: String },
    Repair { drive: String, vol: String },
    Format { drive: String, target: String, whole: bool },
    Delete { drive: String, vol: String },
    Make { drive: String, offset: u64, size: u64 },
}

enum Do {
    Objects(Result<Objects, String>),
    Cap { kind: String, method: &'static str, yes: Option<(bool, u64)> },

    Done { drive: String, row: String, res: Result<String, String>, standing: bool },

    TookBack { drive: String, row: String, said: String, left: u32, done: bool },

    Carry(Carried),

    Snapped { drive: String, vol: String, row: String, want: u64, desc: String, res: Result<(), String> },

    CarryKnown { carried: Carried, uuid: String },
    Soon(u32),

    Changed,
    Refresh,
    Gone,
    Disarm(u32),

    Redraw,
    Show,

    Close,
}

struct Ring {
    name: String,
    icon: &'static str,
    share: f64,
    free: u64,
    whole: u64,

    words: &'static str,
}

struct Drives {
    events: Loop,
    asks: Post<Do>,
    bus: Option<Bus>,
    objects: Objects,
    drives: Vec<Drive>,
    caps: HashMap<String, Caps>,
    headline: (String, Tone),
    rings: Vec<Ring>,

    health_said: String,

    missing: Option<(&'static str, &'static str)>,
    loaded: bool,

    open: Option<String>,

    help: bool,
    help_at: Rect,
    pressed_at: Rect,
    sheet_at: Rect,
    said: Vec<Said>,

    working: HashMap<String, String>,
    armed: Option<String>,
    arms: u32,
    texts: HashMap<String, String>,
    picks: HashMap<String, usize>,
    gbs: HashMap<String, f32>,
    busy: u32,
    soon: u32,
    asked_health: HashSet<String>,

    hidden: BTreeSet<String>,
    hidden_file: PathBuf,

    changing: Rc<Cell<bool>>,

    noted: PathBuf,

    stays: bool,

    session: Option<String>,

    snapping: bool,

    snap: fn(String, Box<dyn FnOnce(Result<(), String>)>),
    world: fn() -> room::World,
    #[cfg(test)]
    ran: Vec<Ran>,
    #[cfg(test)]
    snaps: Vec<String>,
}

fn hidden_file() -> PathBuf {
    if cfg!(test) {
        std::env::temp_dir().join(format!("trindle-drives-none-{}", std::process::id())).join(hidden_drives::FILE)
    } else {
        hidden_drives::file()
    }
}

fn spot(drive: &str, row: &str) -> String {
    format!("{drive}|{row}")
}

impl Drives {
    fn new(events: &Loop, asks: Post<Do>) -> Drives {
        Drives {
            events: events.clone(),
            asks,
            bus: None,
            objects: Objects::new(),
            drives: Vec::new(),
            caps: HashMap::new(),
            headline: (t!("disks-headline-loading").into(), Tone::Plain),
            rings: Vec::new(),
            health_said: String::new(),
            missing: None,
            loaded: false,
            open: None,
            help: false,
            help_at: Rect::default(),
            pressed_at: Rect::default(),
            sheet_at: Rect::default(),
            said: Vec::new(),
            working: HashMap::new(),
            armed: None,
            arms: 0,
            texts: HashMap::new(),
            picks: HashMap::new(),
            gbs: HashMap::new(),
            busy: 0,
            soon: 0,
            asked_health: HashSet::new(),
            hidden: BTreeSet::new(),
            hidden_file: hidden_file(),
            changing: Rc::new(Cell::new(false)),
            noted: noted_folder(),
            stays: false,
            session: None,
            snapping: false,
            snap: take_snapshot,
            world: room::World::read,
            #[cfg(test)]
            ran: Vec::new(),
            #[cfg(test)]
            snaps: Vec::new(),
        }
    }
}

struct Running {
    _shown: Rc<Shown<Drives>>,
    _hears: Vec<Listen>,
    _served: Option<Served>,
    _raise: Option<Served>,
}

pub fn main(_args: Vec<String>) -> i32 {
    let events = Loop::new();
    let Some(bus) = Bus::session(&events).ok() else { return open(&events) };
    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        match start(&ev, &b) {
            Some(running) => *k.borrow_mut() = Some(running),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        b2.call(NAME, PATH, NAME).method("Show", &(), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    code.get()
}

fn start(events: &Loop, bus: &Bus) -> Option<Running> {
    let (shown, asks, hears) = window(events)?;
    let served = bus.serve(PATH, Offer {
        name: NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            asks.send(Do::Show);
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).map_err(|e| eprintln!("trindle-disks: cannot be shown again from a second start: {e}")).ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window)
        .map_err(|e| eprintln!("trindle-disks: cannot be raised from a second start: {e}")).ok();
    Some(Running { _shown: shown, _hears: hears, _served: served, _raise: raise })
}

fn open(events: &Loop) -> i32 {
    let Some(_running) = window(events) else { return 1 };
    events.run();
    0
}

fn window(events: &Loop) -> Option<(Rc<Shown<Drives>>, Post<Do>, Vec<Listen>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Drives>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let asks = events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                shown.window.raise();
            }
            ask => shown.change(|d| d.handle(ask)),
        }
    });
    let mut app = Drives::new(events, asks.clone());
    let mut hears = Vec::new();
    match Bus::system(events) {
        Err(_) => app.missing = Some((t!("disks-missing-bus"), "dbus")),
        Ok(system) => {
            for member in ["InterfacesAdded", "InterfacesRemoved"] {
                let a = asks.clone();
                let heard = system.listen(Some(UD), OBJECT_MANAGER, Some(member), Some(UD_PATH), move |msg| {

                    if !crate::common::volumes::is_job(msg) {
                        a.send(Do::Changed);
                    }
                });
                hears.extend(heard.ok());
            }
            let (here, away) = (asks.clone(), asks.clone());
            hears.extend(system.watch(UD, events, move |_| {
                here.send(Do::Refresh);
            }, move || {
                away.send(Do::Gone);
            }).ok());
            app.bus = Some(system);
        }
    }

    app.tell_what_was_left();
    let kind = Kind::Window { title: t!("disks-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);

    let close = asks.clone();
    shown.window.on_close(move || {
        close.send(Do::Close);
    });
    Some((shown, asks, hears))
}

#[cfg(test)]
use steps::Ran;

#[cfg(test)]
mod tests;
