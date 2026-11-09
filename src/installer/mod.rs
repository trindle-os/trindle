//! trindle-installer: puts Trindle on a disk. The window runs as the live
//! user; `--run` is the same program as root, reading the plan on stdin.

mod actions;
mod brand;
mod look;
mod pages;
mod plan;
mod poster;
mod roll;
mod steps;
mod tour;
pub mod welcome;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::mpsc::Sender;
use std::time::Duration;

use zeroize::Zeroizing;

use crate::common::run;
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::t;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{column, drawing, scroll, Length, Node};
use crate::toolkit::window::{Display, Kind};
use actions::plain;
use plan::{Disk, Lock, Plan, Where};

pub(crate) const SIZE: (u32, u32) = (920, 600);

pub fn main(args: Vec<String>) -> i32 {
    match args.get(1).map(String::as_str) {
        Some("--look") => return look::main(),
        Some("--run") => return run_plan(),
        _ => {}
    }
    let brand = brand::brand();
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };

    let slot: Slot = Rc::new(RefCell::new(Weak::new()));
    let (asks, wake) = posts(&events, &slot);
    let (r, g, b) = brand.accent;
    let app = Installer::new(brand.name.clone(), Colour::rgb(r as f32, g as f32, b as f32), this_machine(),
                             events.clone(), asks, wake);

    let height = display.places().first().map_or(SIZE.1 as i32, |p| (SIZE.1 as i32).min((p[3] - 90).max(360)));
    let kind = Kind::Window { title: t!("installer-window-title", name = &brand.name), app_id: "org.trindle.Installer".into() };
    let Some(shown) = Shown::open(&events, &display, kind, (SIZE.0, height as u32), app) else { return 1 };
    let shown = Rc::new(shown);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    events.run();
    0
}

fn run_plan() -> i32 {
    let mut json = String::new();
    if std::io::stdin().read_line(&mut json).is_err() {
        eprintln!("{}", t!("installer-plan-unreadable"));
        return 1;
    }
    let plan: Plan = match serde_json::from_str(&json) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", t!("installer-plan-unreadable-why", why = e.to_string()));
            return 1;
        }
    };
    let (answers, wait) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(n) if n > 0 => {
                let _ = answers.send(line);
            }
            _ => {
                steps::stop();
                return;
            }
        }
    });
    let outcome = steps::install(&plan, &wait);
    if steps::stopped() {
        return if steps::let_go() { steps::STOPPED } else { steps::STUCK };
    }
    match outcome {
        Ok(()) => 0,
        Err(e) => {
            steps::let_go();
            eprintln!("{e}");
            1
        }
    }
}

type Slot = Rc<RefCell<Weak<Shown<Installer>>>>;

fn posts(events: &Loop, slot: &Slot) -> (Post<Msg>, Post<()>) {
    let s = slot.clone();
    let asks = events.post(move |m: Msg| {
        if let Some(shown) = s.borrow().upgrade() {
            shown.change(|a| a.update(m));
        }
    });
    let s = slot.clone();
    let wake = events.post(move |()| {
        if let Some(shown) = s.borrow().upgrade() {
            shown.window.redraw();
        }
    });
    (asks, wake)
}

fn this_machine() -> Machine {
    let zones = plan::timezones();
    Machine {
        locales: plan::locales(),
        keyboards: plan::keyboards().into_iter().map(|(l, v, name)| (format!("{l}\t{v}"), name)).collect(),
        zones: zones.iter().map(|z| (z.clone(), z.replace('_', " "))).collect(),
        disks: plan::disks(),
        chip: steps::has_chip(),
        secure_boot: plan::secure_boot(),
        memory: plan::memory_gib(),
        short_of_memory: plan::short_of_memory(),
        uefi: plan::uefi(),
    }
}

fn pages() -> [&'static str; PAGE_COUNT] {
    [t!("installer-stop-welcome"), t!("installer-stop-disk"), t!("installer-stop-privacy"), t!("installer-stop-password"),
     t!("installer-stop-ready"), t!("installer-stop-you"), t!("installer-stop-installing"), t!("installer-stop-done")]
}
const PAGE_COUNT: usize = 8;

const DISK_PASSWORD: usize = 3;
const READY: usize = 4;

const ABOUT_YOU: usize = 5;
const RUNNING: usize = 6;
const DONE: usize = 7;

const BESIDE_WIDTH: f32 = 304.0;

const SWAP_WIDTH: f32 = 300.0;

const FRAME: Duration = Duration::from_millis(16);

const SLIDE: Duration = Duration::from_secs(11);

const MARKS: usize = 5;
const MARK: (f32, f32) = (34.0, 6.0);
const MARK_GAP: f32 = 5.0;

struct Machine {
    locales: Vec<(String, String)>,

    keyboards: Vec<(String, String)>,
    zones: Vec<(String, String)>,
    disks: Vec<Disk>,
    chip: bool,
    secure_boot: bool,
    memory: u64,

    short_of_memory: bool,

    uefi: bool,
}

#[derive(Clone)]
enum Msg {
    Back,
    Next,
    Restart,
    Install,

    Terms(bool),

    ReadTerms(bool),
    Disarm(u32),

    Find(usize, String),

    Pick(usize, usize),
    Try(String),
    Erase(String),
    Beside(String),
    Leave(String, f32),
    Swap(bool),
    Lock(Lock),
    DiskOne(String),
    DiskTwo(String),
    Suggest,
    Full(String),
    User(String),
    Host(String),
    One(String),
    Two(String),

    Line(String),
    Finished(Result<(), String>),

    Stop,

    Stopped(bool),
    NextSlide,
}

struct Installer {
    name: String,
    accent: Colour,
    plan: Plan,
    page: usize,
    machine: Machine,
    find: [String; 3],
    tried: String,
    disk_one: Zeroizing<String>,
    disk_two: Zeroizing<String>,

    suggested: u32,
    full: String,
    user: String,
    host: String,
    one: Zeroizing<String>,
    two: Zeroizing<String>,

    user_theirs: bool,

    leave: HashMap<String, u64>,

    doing: String,
    share: f32,
    failed: bool,

    problems: Vec<String>,

    helper: Option<Sender<Zeroizing<String>>>,
    stopping: bool,

    stuck: bool,

    armed: bool,
    arms: u32,

    terms: bool,

    reading_terms: bool,
    roll_at: Rc<Cell<f32>>,
    roll_to: Rc<Cell<f32>>,
    rolling: Rc<Cell<bool>>,
    slide: usize,
    touring: Rc<Cell<bool>>,
    cache: Rc<tour::Cache>,
    events: Loop,
    asks: Post<Msg>,
    wake: Post<()>,
}

impl Installer {
    fn tick_terms(&mut self, on: bool) {
        self.terms = on;
        self.plan.terms = on;
        self.armed &= on;
    }

    fn disarm(&mut self, n: u32) {
        if n == self.arms {
            self.armed = false;
        }
    }

    fn set_find(&mut self, at: usize, t: String) {
        if let Some(f) = self.find.get_mut(at) {
            *f = t;
        }
    }
}

impl App for Installer {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Back => self.go(-1),
            Msg::Next => self.go(1),
            Msg::Restart => {
                let _ = run::spawn(&["systemctl", "reboot"]);
            }
            Msg::Install => self.install(),
            Msg::Terms(on) => self.tick_terms(on),
            Msg::ReadTerms(open) => self.reading_terms = open,
            Msg::Disarm(n) => self.disarm(n),
            Msg::Find(at, t) => self.set_find(at, t),
            Msg::Pick(at, i) => self.pick(at, i),
            Msg::Try(t) => self.tried = t,
            Msg::Erase(disk) => self.plan.target = Some(Where::Erase { disk }),
            Msg::Beside(part) => self.beside(part),
            Msg::Leave(part, v) => {
                self.leave.insert(part.clone(), v.max(0.0) as u64);
                self.beside(part);
            }
            Msg::Swap(on) => self.plan.swap = on,
            Msg::Lock(lock) => self.plan.lock = lock,
            Msg::DiskOne(t) => {
                self.disk_one = Zeroizing::new(t);
                self.disk_typed();
            }
            Msg::DiskTwo(t) => {
                self.disk_two = Zeroizing::new(t);
                self.disk_typed();
            }
            Msg::Suggest => self.suggest(),
            Msg::Full(t) => self.full_name(t),
            Msg::User(t) => self.sign_in_name(t),
            Msg::Host(t) => {
                self.host = t;
                self.you_typed();
            }
            Msg::One(t) => {
                self.one = Zeroizing::new(t);
                self.you_typed();
            }
            Msg::Two(t) => {
                self.two = Zeroizing::new(t);
                self.you_typed();
            }
            Msg::Line(line) => self.progress(&line),
            Msg::Finished(Ok(())) => {
                self.helper = None;
                self.show(DONE);
            }
            Msg::Finished(Err(e)) => self.fail(&plain(&e)),
            Msg::Stop => self.stop(),
            Msg::Stopped(free) => self.stopped(free),
            Msg::NextSlide => self.slide = (self.slide + 1) % tour::COUNT,
        }
    }

    fn view(&self) -> Node<Msg> {
        let (at, accent) = (self.roll_at.clone(), self.accent);
        let names = pages();
        let strip = drawing((400.0, roll::HEIGHT), names[self.page], move |canvas, pen| {
            roll::paint(canvas, pen, &names, at.get(), accent)
        }).width(Length::Fill);
        column(vec![
            strip,
            scroll(format!("page-{}", self.page), self.body().padding(24.0)),
            self.foot(),
        ]).gap(0.0)
    }
}

#[cfg(test)]
mod tests;
