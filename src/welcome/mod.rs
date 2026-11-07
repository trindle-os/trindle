//! trindle-welcome.

mod pages;
mod view;
mod install;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use install::install;
use pages::{every_app, Level, Mark, offered, Page, PAGES, title};
use view::PART_GAP;

use crate::common::files::config_path;
use crate::common::run::{self, spawn};
use crate::store::catalogue;
use crate::toolkit::{t, theme};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::ui::{App, Keyboard, Shown};
use crate::toolkit::widget::{column, headline, Length, Node, note, page_title, row};
use crate::toolkit::window::{Display, Kind};

pub(crate) mod pictures;

const NAME: &str = "org.trindle.Welcome";

const PATH: &str = "/org/trindle/Welcome";

const XML: &str = r#"<interface name="org.trindle.Welcome"><method name="Show"/></interface>"#;

const DONE: &str = "welcome-done";

pub fn first_time() -> bool {
    !config_path(DONE).exists()
}

pub fn mark_done() {
    mark_done_at(&config_path(DONE));
}

fn mark_done_at(f: &std::path::Path) {
    if let Some(dir) = f.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(f, format!("{}\n", t!("welcome-done-file-note")));
}

#[derive(Clone, Debug, PartialEq)]
enum Install {
    Idle,
    Running(Vec<String>, f32, Option<String>),
    Done(Vec<String>),
    Failed(Vec<String>, String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MainAct {
    Install,
    Start,
    Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ask {
    Install(Vec<String>),
    Close,
    Plugins,
    Reports(bool),
    Say(String),
    Seen,
    Words,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Next,
    Back,
    Main,
    Close,
    Pick(String),
    Plugins,
    Reports(bool),
    Retry,
    Level(Level),
    Words,
}

#[derive(Clone, Debug, PartialEq)]
struct Offered {
    icon: Option<PathBuf>,
    name: String,
    line: String,
}

pub struct Welcome {
    page: usize,
    level: Level,
    picked: BTreeSet<String>,
    installed: HashSet<String>,
    known: Option<HashSet<String>>,
    icons: HashMap<String, PathBuf>,
    plugins: Vec<Offered>,
    install: Install,
    reports: bool,
    glow: u32,
    online: bool,
    began: Instant,
    held: Option<f32>,
    moving: bool,
    size: (f32, f32),
    post: Option<Post<Ask>>,
    asked: Vec<Ask>,
}

impl Welcome {
    fn new(post: Option<Post<Ask>>) -> Welcome {
        let look = theme::welcome();
        Welcome {
            page: 0,
            level: Level::New,
            picked: BTreeSet::new(),
            installed: HashSet::new(),
            known: None,
            icons: HashMap::new(),
            plugins: Vec::new(),
            install: Install::Idle,
            reports: false,
            glow: 0,
            online: true,
            began: Instant::now(),
            held: None,
            moving: crate::common::graphics::moving(),
            size: (look.width, look.height),
            post,
            asked: Vec::new(),
        }
    }

    fn ask(&mut self, a: Ask) {
        match &self.post {
            Some(p) => {
                p.send(a);
            }
            None => self.asked.push(a),
        }
    }

    fn pages(&self) -> Vec<Page> {
        PAGES.into_iter().filter(|p| self.level == Level::New || p.mark() == Mark::Trindle).collect()
    }

    fn here(&self) -> Page {
        self.pages()[self.page]
    }

    fn know(&mut self, level: Level) {
        self.level = level;
        if level == Level::Trindle {
            self.ask(Ask::Seen);
            self.ask(Ask::Close);
        } else {
            self.go_to(self.page + 1);
        }
    }

    fn go_to(&mut self, page: usize) {
        let total = self.pages().len();
        let page = page.min(total - 1);
        if page == self.page {
            return;
        }
        self.page = page;
        self.began = Instant::now();
        self.ask(Ask::Say(self.here().spoken(page + 1, total)));
    }

    fn offered(&self, apps: &'static [(&'static str, &'static str)]) -> Vec<(&'static str, &'static str)> {
        apps.iter().copied().filter(|(id, _)| self.known.as_ref().is_none_or(|k| k.contains(*id))).collect()
    }

    fn main_words(&self) -> &'static str {
        match self.main_act() {
            MainAct::Install => t!("welcome-install"),
            MainAct::Start => t!("welcome-start"),
            MainAct::Next => t!("welcome-next"),
        }
    }

    fn main_act(&self) -> MainAct {
        match self.here() {
            Page::MoreApps if !self.picked.is_empty() && !self.installing() => MainAct::Install,
            Page::Know => MainAct::Start,
            _ => MainAct::Next,
        }
    }

    fn main(&mut self) {
        match self.main_act() {
            MainAct::Install => {
                if !self.online {
                    return;
                }
                let ids: Vec<String> = self.picked.iter().cloned().collect();
                self.picked.clear();
                self.begin(ids);
                self.go_to(self.page + 1);
            }
            MainAct::Start => self.ask(Ask::Close),
            MainAct::Next => self.go_to(self.page + 1),
        }
    }
}

impl App for Welcome {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Next => self.go_to(self.page + 1),
            Msg::Back => self.go_to(self.page.saturating_sub(1)),
            Msg::Main => self.main(),
            Msg::Close => self.ask(Ask::Close),
            Msg::Pick(id) => self.pick(&id),
            Msg::Plugins => self.ask(Ask::Plugins),
            Msg::Level(l) => self.know(l),
            Msg::Words => self.ask(Ask::Words),
            Msg::Reports(yes) => {
                self.reports = yes;
                self.ask(Ask::Reports(yes));
            }
            Msg::Retry => self.retry(),
        }
    }

    fn view(&self) -> Node<Msg> {
        let page = self.here();
        let step = t!("welcome-step", n = self.page + 1, total = self.pages().len());
        let head = column(vec![
            row(vec![page_title(page.title()).width(Length::Fill), note(step)]).gap(theme::sheet().gap),
            headline(page.sentence()).width(Length::Fill),
        ]).gap(4.0);
        column(vec![head, self.view_page().height(Length::Fill), self.foot()])
            .gap(PART_GAP).padding(theme::sheet().page_padding)
    }

    fn intercept_with(&self, input: &Input, at: Keyboard) -> Option<Msg> {
        let Input::Key { sym, mods, .. } = input else { return None };
        if mods.ctrl || mods.alt || mods.logo {
            return None;
        }
        match *sym {
            Keysym::Escape => Some(Msg::Close),
            Keysym::Right | Keysym::Page_Down if !at.arrows => Some(Msg::Next),
            Keysym::Left | Keysym::Page_Up if !at.arrows => Some(Msg::Back),
            Keysym::Return | Keysym::KP_Enter if !at.keys => Some(Msg::Main),
            _ => None,
        }
    }

    fn resized(&mut self, w: f32, h: f32) -> bool {
        let before = self.tile();
        self.size = (w, h);
        self.tile() != before
    }
}

pub fn window_size(screen: (f32, f32)) -> (f32, f32) {
    let look = theme::welcome();
    let w = look.width.min(screen.0 - 48.0).max(look.least_width.min(screen.0));
    let h = look.height.min(screen.1 - 72.0).max(look.least_height.min(screen.1 - 40.0));
    (w.floor(), h.floor())
}

fn start(events: &Loop, first: bool) -> Option<Rc<Shown<Welcome>>> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    run::hand_back_to(events);
    if first {
        mark_done();
    }
    let slot: Rc<RefCell<Weak<Shown<Welcome>>>> = Rc::new(RefCell::new(Weak::new()));
    let closed = Rc::new(std::cell::Cell::new(false));
    let (s, ev, c) = (slot.clone(), events.clone(), closed.clone());
    let post = events.post(move |a: Ask| {
        let Some(shown) = s.borrow().upgrade() else { return };
        answer(a, &shown, &ev, first, &c);
    });
    let app = Welcome::new(Some(post.clone()));
    let screen = display.places().first().map(|p| (p[2] as f32, p[3] as f32)).unwrap_or((1280.0, 800.0));
    let (w, h) = window_size(screen);
    let kind = Kind::Window { title: title().into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, (w as u32, h as u32), app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let p = post.clone();
    shown.window.on_close(move || {
        p.send(Ask::Close);
    });
    read_what_is_there(&shown);
    keep_looking(&shown, events);
    shown.change(|s| s.online = crate::common::online());
    let p = shown.with(|s| s.here());
    shown.announce(&p.spoken(1, PAGES.len()));
    Some(shown)
}

fn answer(a: Ask, shown: &Rc<Shown<Welcome>>, events: &Loop, first: bool, closed: &std::cell::Cell<bool>) {
    match a {
        Ask::Install(ids) => install(shown, ids, events),
        Ask::Plugins => {
            spawn(&["trindle-apps", "--page", "plugins"]);
        }
        Ask::Seen => mark_done(),
        Ask::Words => {
            spawn(&["trindle-settings", "--panel", "region"]);
        }
        Ask::Reports(yes) => {
            if crate::common::files::set_setting(crate::report::OFFER_KEY, if yes { "on" } else { "off" }).is_ok() && !yes {
                crate::report::forget_pending(&crate::report::pending_dir());
            }
        }
        Ask::Say(words) => shown.announce(&words),
        Ask::Close => {
            if closed.replace(true) {
                return;
            }
            if first {
                spawn(&["trindle-ring"]);
            }
            shown.window.set_visible(false);
            events.quit();
        }
    }
}

fn read_what_is_there(shown: &Rc<Shown<Welcome>>) {
    let w = Rc::downgrade(shown);
    catalogue::installed(move |list| {
        if let Some(shown) = w.upgrade() {
            shown.change(|s| s.installed = list.into_keys().collect());
        }
    });
    let w = Rc::downgrade(shown);
    catalogue::load(move |apps, _| {
        let (Some(shown), Some(apps)) = (w.upgrade(), apps) else { return };
        shown.change(|s| {
            s.icons = every_app().filter_map(|(id, _)| apps.get(id)?.icon.clone().map(|p| (id.to_string(), p))).collect();
            s.known = Some(apps.into_keys().collect());
        });
    });
    let w = Rc::downgrade(shown);
    crate::common::run::on_thread(
        || crate::plugins::catalogue::gather().0,
        move |offers| {
            if let Some(shown) = w.upgrade() {
                shown.change(|s| s.plugins = offered(&offers));
            }
        });
}

fn keep_looking(shown: &Rc<Shown<Welcome>>, events: &Loop) {
    let w = Rc::downgrade(shown);
    events.every(Duration::from_secs(3), move || match w.upgrade() {
        Some(shown) => {
            let online = crate::common::online();
            if shown.with(|s| s.online != online) {
                shown.change(|s| s.online = online);
            }
            Flow::Continue
        }
        None => Flow::Stop,
    });
    let w = Rc::downgrade(shown);
    events.every(FRAME, move || match w.upgrade() {
        Some(shown) => {
            if shown.with(|s| s.animating()) && shown.window.seen() {
                shown.window.redraw();
            }
            Flow::Continue
        }
        None => Flow::Stop,
    });
}

const FRAME: Duration = Duration::from_millis(33);

pub fn main(args: Vec<String>) -> i32 {
    let first = args.iter().any(|a| a == "--first");
    let events = Loop::new();
    let Ok(bus) = crate::toolkit::bus::Bus::session(&events) else {
        let Some(_shown) = start(&events, first) else { return 1 };
        events.run();
        return 0;
    };
    let kept = Rc::new(RefCell::new(None));
    let (ev, b, k) = (events.clone(), bus.clone(), kept.clone());
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        let Some(shown) = start(&ev, first) else {
            ev.quit();
            return;
        };
        let w = Rc::downgrade(&shown);
        let served = b.serve(PATH, crate::toolkit::bus::Offer {
            name: NAME.into(),
            xml: XML.into(),
            call: Box::new(move |asked| {
                if let Some(shown) = w.upgrade() {
                    shown.window.raise();
                }
                asked.reply(&());
            }),
            props: Box::new(crate::toolkit::bus::Props::new),
            set: None,
        }).ok();
        let raise = crate::toolkit::once::serve_window(&b, PATH, &shown.window).ok();
        *k.borrow_mut() = Some((shown, served, raise));
    }, move || {
        b2.call(NAME, PATH, NAME).method("Show", &(), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    0
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_reports;
