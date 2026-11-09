//! trindle-activity: what the computer is doing right now, in a window of
//! its own, drawn rather than listed, in the desktop style. A title and one
//! plain sentence say what is going on; under them the computer is a room
//! (see orbit/) where every program is a bubble as big as the memory it
//! holds, glowing as it works, with the processor's threads round the rim,
//! in a card whose i floats the longer words over the window; and under that
//! four cards of an amount each, a ring of a limit, give the figures (see
//! rings.rs). Every part of a program is added together, so Firefox is one
//! bubble and not twenty helpers.

mod reading;
mod headline;
mod page;
mod stop;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod language_tests;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use headline::Headline;
use procs::Member;

use crate::common::apps::Entry;
use crate::toolkit::{popup, t};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::motion::{Ease, tween};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{Node, Pointer, Rect};
use crate::toolkit::window::{Display, Kind as Window};

pub use reading::names;
use stop::FORCED_SHOWN_SECONDS;

mod orbit;
pub mod procs;
mod rings;

const EVERY_SECS: u32 = 2;

const HOLD_LOOKS: u32 = 3;

const STOP_ICON: &str = "media-playback-stop-symbolic";

const NAME: &str = "org.trindle.Activity";

const PATH: &str = "/org/trindle/Activity";

const XML: &str = r#"<node><interface name="org.trindle.Activity"><method name="Show"/></interface></node>"#;

const SIZE: (u32, u32) = (1280, 820);

#[derive(Clone)]
enum Msg {

    Idle,
    Pointer(Pointer),

    Keyed(bool),

    Press,
    Stop,
    Close,

    Pick(String),

    Help,

    Retry,
}

enum Do {
    Show,
    Tick,

    Look(Option<(procs::Look, (u64, u64))>),
    Disarm(u32),

    Force(Vec<Member>, String),

    Unforce(u32),

    Fade(f32),
}

const FADE_MS: u64 = 600;

struct Info {
    title: String,
    icon: Vec<String>,
    heat: f64,
    effort: &'static str,

    working: bool,

    work: (String, String),

    holding: (String, String),
    running: String,
    parts: usize,
    can_stop: bool,
}

struct Activity {
    events: Loop,
    asks: Post<Do>,
    apps: Vec<Entry>,
    names: Arc<procs::Names>,

    reader: Arc<Mutex<procs::Reader>>,
    looking: bool,

    members: HashMap<String, Vec<Member>>,
    infos: HashMap<String, Info>,

    order: Vec<String>,
    rings: rings::Rings,
    orbit: Rc<orbit::Orbit>,
    headline: Headline,

    said: String,
    was: String,

    unread: String,
    fade: f32,

    looked: bool,

    failed: bool,

    help: bool,
    help_at: Rect,
    pressed_at: Rect,

    armed: bool,

    arms: u32,

    forced: String,
    forces: u32,
}

impl Activity {
    fn new(events: &Loop, asks: Post<Do>) -> Activity {
        let apps = crate::common::apps::entries();
        let names = Arc::new(names(&apps));
        Activity {
            events: events.clone(), asks, apps, names, reader: Arc::new(Mutex::new(procs::Reader::default())),
            looking: false, members: HashMap::new(), infos: HashMap::new(), order: Vec::new(),
            rings: rings::Rings::new(),
            orbit: orbit::Orbit::new(events), headline: Headline::default(),
            said: t!("activity-looking").into(), was: String::new(),
            unread: t!("activity-unread").into(), fade: 1.0, looked: false, failed: false,
            help: false, help_at: Rect::default(), pressed_at: Rect::default(), armed: false, arms: 0, forced: String::new(), forces: 0,
        }
    }

    fn start(&mut self) {
        self.look();
        let a = self.asks.clone();
        self.events.after(Duration::from_secs(1), move || {
            a.send(Do::Tick);
        });
        let a = self.asks.clone();
        self.events.every(Duration::from_secs(EVERY_SECS.into()), move || {
            a.send(Do::Tick);
            Flow::Continue
        });
        self.orbit.wake();
    }

    fn look(&mut self) {
        if std::mem::replace(&mut self.looking, true) {
            return;
        }
        let (reader, names, asks) = (self.reader.clone(), self.names.clone(), self.asks.clone());
        let started = std::thread::Builder::new().spawn(move || {
            let look = std::panic::catch_unwind(AssertUnwindSafe(|| {
                let mut r = reader.lock().unwrap_or_else(PoisonError::into_inner);
                let look = r.look(&names);
                let m = rings::meminfo();
                (look, (m("MemTotal:").max(1), m("MemAvailable:")))
            })).ok();
            asks.send(Do::Look(look));
        });
        if started.is_err() {
            self.handle(Do::Look(None));
        }
    }

    fn handle(&mut self, ask: Do) {
        match ask {
            Do::Show => unreachable!("raised in window(), before this"),
            Do::Tick => self.look(),
            Do::Look(look) => {
                self.looking = false;
                self.failed = look.is_none();
                self.rings.failed(self.failed);
                if let Some((look, mem)) = look {
                    self.show(look, mem);
                }
            }
            Do::Disarm(n) => {
                if n == self.arms {
                    self.armed = false;
                }
            }
            Do::Force(members, title) => {
                if procs::force(&members) > 0 {
                    self.forced = t!("activity-forced", title = &title).to_string();
                    self.forces += 1;
                    let (n, asks) = (self.forces, self.asks.clone());
                    self.events.after(Duration::from_secs(FORCED_SHOWN_SECONDS), move || { asks.send(Do::Unforce(n)); });
                }
            }
            Do::Unforce(n) => {
                if n == self.forces {
                    self.forced.clear();
                }
            }
            Do::Fade(t) => self.fade = t,
        }
    }

    fn fade_in(&mut self) {
        if !crate::common::graphics::moving() {
            self.fade = 1.0;
            return;
        }
        self.fade = 0.0;
        let asks = self.asks.clone();
        tween(&self.events, FADE_MS, Ease::Out, move |t| { asks.send(Do::Fade(t)); }, || {});
    }
}

impl App for Activity {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Pointer(p) => {
                if self.orbit.pointer(p).is_some() {
                    self.armed = false;
                }
            }
            Msg::Keyed(changed) => {
                if changed {
                    self.armed = false;
                }
            }
            Msg::Press => {
                if self.orbit.press().is_some() {
                    self.armed = false;
                }
            }
            Msg::Stop => self.stop(),
            Msg::Close => {
                self.orbit.pick(None);
                self.armed = false;
            }
            Msg::Pick(key) => {
                let same = self.orbit.picked().as_deref() == Some(key.as_str());
                self.orbit.pick(if same { None } else { Some(&key) });
                self.armed = false;
            }
            Msg::Help => {
                self.help = !self.help;
                self.help_at = self.pressed_at;
            }
            Msg::Retry => self.look(),
            Msg::Idle => {}
        }
    }

    fn view(&self) -> Node<Msg> {
        let page = self.page();
        if self.help {
            popup::dismissing(page, self.help_panel(), Msg::Help, Msg::Idle)
        } else {
            page
        }
    }

    fn pressed(&mut self, place: Rect) {
        self.pressed_at = place;
    }

    fn intercept(&self, input: &Input, keys: bool) -> Option<Msg> {
        match input {
            Input::Key { sym: Keysym::Escape, .. } if self.help => Some(Msg::Help),
            Input::Key { sym: Keysym::Escape, .. } if keys && self.orbit.picked().is_some() => Some(Msg::Close),
            _ => None,
        }
    }
}

struct Running {
    _shown: Rc<Shown<Activity>>,
    _served: Option<Served>,
    _raise: Option<Served>,
}

pub fn main(_args: Vec<String>) -> i32 {
    let events = Loop::new();
    let Some(bus) = Bus::session(&events).ok() else { return open(&events) };
    let code = Rc::new(Cell::new(0));
    let code2 = code.clone();
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());

    let (ev2, b2) = (ev.clone(), b.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        match start(&ev, &b) {
            Some(running) => *k.borrow_mut() = Some(running),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        let (quit, c2) = (ev2.clone(), code2.clone());
        b2.call(NAME, PATH, NAME).method("Show", &(), false, Some(Duration::from_secs(2)), &ev2, move |reply| {
            if reply.is_err() {
                eprintln!("trindle-activity: the window already open did not answer");
                c2.set(1);
            }
            quit.quit();
        });
    });
    events.run();
    drop(kept);
    code.get()
}

fn start(events: &Loop, bus: &Bus) -> Option<Running> {
    let (shown, asks) = window(events)?;
    let served = bus.serve(PATH, Offer {
        name: NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            asks.send(Do::Show);
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).map_err(|e| eprintln!("trindle-activity: cannot be asked to show: {e:?}")).ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window).map_err(|e| eprintln!("trindle-activity: cannot be raised: {e:?}")).ok();
    Some(Running { _shown: shown, _served: served, _raise: raise })
}

fn open(events: &Loop) -> i32 {
    let Some(_running) = window(events) else { return 1 };
    events.run();
    0
}

fn window(events: &Loop) -> Option<(Rc<Shown<Activity>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Activity>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let asks = events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                shown.window.raise();
            }

            Do::Tick if !shown.window.seen() => {}
            ask => shown.change(|a| a.handle(ask)),
        }
    });
    let mut app = Activity::new(events, asks.clone());
    app.start();
    let orbit = app.orbit.clone();
    let kind = Window::Window { title: t!("activity-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);

    let s = Rc::downgrade(&shown);
    let ev = events.clone();
    orbit.on_redraw(move || {
        let Some(shown) = s.upgrade() else { return };
        if !shown.busy() {
            shown.window.redraw();
            return;
        }
        let w = Rc::downgrade(&shown.window);
        ev.after(Duration::ZERO, move || {
            if let Some(w) = w.upgrade() {
                w.redraw();
            }
        });
    });

    let a = asks.clone();
    shown.window.on_suspended(move |suspended| {
        if !suspended {
            a.send(Do::Tick);
        }
    });
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    Some((shown, asks))
}
