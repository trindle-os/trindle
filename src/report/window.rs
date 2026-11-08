//! Opening the report window: one at a time, a second press bringing the first forward, and the reading of what it will show.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

use super::probe::{self, Crash};
use super::scrub::Who;
use super::send::{self, Answer};
use super::{
    github, home, live_for, log_lines, model, offered, pending_dir, read_log, same_window, server, title_crash, title_note, username, Do,
    Mode, Reporter, NAME, PATH, SIZE, XML,
};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::ui::Shown;
use crate::toolkit::window::{Display, Kind};

type Slot = Rc<RefCell<Weak<Shown<Reporter>>>>;

type Kept = Option<(Rc<Shown<Reporter>>, Option<Served>, Option<Served>)>;

pub(super) fn share(events: &Loop, bus: &Bus, mode: Mode, program: String) -> i32 {
    let code = Rc::new(Cell::new(0));
    let kept: Rc<RefCell<Kept>> = Rc::new(RefCell::new(None));
    let (ev, b, k, c) = (events.clone(), bus.clone(), kept.clone(), code.clone());
    let (ev2, b2) = (events.clone(), bus.clone());
    let (k2, c2, program2) = (kept.clone(), code.clone(), program.clone());
    crate::toolkit::once::claim(bus, events, NAME, PATH, move || {
        match serve_first(&ev, &b, mode, program) {
            Some(first) => *k.borrow_mut() = Some(first),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        let (quit, ev3) = (ev2.clone(), ev2.clone());
        let ask = (mode == Mode::Crash, program2.clone());
        b2.call(NAME, PATH, NAME).method("Show", &ask, false, Some(Duration::from_secs(2)), &ev2, move |answer| {
            let taken = answer.ok().and_then(|m| m.body().deserialize::<bool>().ok()).unwrap_or(false);
            if taken {
                quit.quit();
                return;
            }

            match open(&ev3, mode, program2) {
                Some(shown) => *k2.borrow_mut() = Some((shown, None, None)),
                None => {
                    c2.set(1);
                    quit.quit();
                }
            }
        });
    });
    events.run();
    drop(kept);
    code.get()
}

fn serve_first(events: &Loop, bus: &Bus, mode: Mode, program: String) -> Option<(Rc<Shown<Reporter>>, Option<Served>, Option<Served>)> {
    let shown = open(events, mode, program)?;
    let served = crate::toolkit::once::serve_window(bus, PATH, &shown.window).ok();
    let shown_again = Rc::downgrade(&shown);
    let shows = bus.serve(PATH, Offer {
        name: NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {

            let (crash, program) = asked.message().body().deserialize::<(bool, String)>().unwrap_or((false, String::new()));
            let mut taken = false;
            if let Some(shown) = shown_again.upgrade() {
                shown.change(|r| taken = same_window(r.mode, &r.program, crash, &program));
                if taken {
                    shown.window.raise();
                }
            }
            asked.reply(&taken);
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok();
    Some((shown, served, shows))
}

pub(super) fn open(events: &Loop, mode: Mode, program: String) -> Option<Rc<Shown<Reporter>>> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let events = events.clone();
    let slot: Slot = Rc::new(RefCell::new(Weak::new()));
    let who = Who::here();
    let asks = answering(&events, &slot, &who);
    let mut app = Reporter::new(who, model(), home().join("Downloads"));
    app.mode = mode;
    app.program = program.clone();
    app.username = username();
    app.github = github();
    app.switch = offered();
    app.keeps_username = true;
    app.waiting_dir = pending_dir();
    app.events = Some(events.clone());
    app.asks = Some(asks.clone());
    let title = match mode {
        Mode::Crash => title_crash(),
        Mode::Note => title_note(),
    };
    let kind = Kind::Window { title: title.into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(&events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    read_in_background(&asks, mode, program);
    Some(shown)
}

fn answering(events: &Loop, slot: &Slot, who: &Who) -> Post<Do> {
    let (s, t) = (slot.clone(), slot.clone());
    let for_log = who.clone();
    let answer = events.post(move |result: Answer| {
        let Some(shown) = t.borrow().upgrade() else { return };
        shown.change(|r| r.answered(result));
    });
    events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Log(raw) => {
                let lines = log_lines(&raw, &for_log);
                shown.change(|r| r.log = Some(lines));
            }
            Do::Hardware(h) => shown.change(|r| r.hardware = h),
            Do::Crash(found, raw) => {
                let lines = log_lines(&raw, &for_log);
                shown.change(|r| {
                    r.crash = Some(found);
                    r.log = Some(lines);
                });
            }
            Do::Copy(text) => shown.window.copy(&text),
            Do::Post(body) => {
                let back = answer.clone();
                std::thread::spawn(move || {
                    back.send(send::post(&live_for(&server(), &send::slug_of(&body)), &body));
                });
            }
            Do::Status(open) => shown.change(|r| r.gate_is(open)),
        }
    })
}

fn read_in_background(asks: &Post<Do>, mode: Mode, program: String) {

    let asked = true;
    if asked && mode == Mode::Note {
        let ask = asks.clone();
        std::thread::spawn(move || {
            ask.send(Do::Status(send::accepting(&server())));
        });
    }
    let reads = asks.clone();
    std::thread::spawn(move || {
        match mode {
            Mode::Crash => {
                let found = probe::coredump();
                let raw = probe::log_before(found.as_ref().map(|f| f.1));
                let crash = found.map(|f| f.0).unwrap_or(Crash { program, signal: String::new(), trace: String::new() });
                let to = live_for(&server(), &send::slug_for(&crash.program));
                reads.send(Do::Crash(crash, raw));
                if asked {
                    reads.send(Do::Status(send::accepting(&to)));
                }
            }
            Mode::Note => {
                reads.send(Do::Log(read_log()));
            }
        }
        reads.send(Do::Hardware(Some(probe::hardware_summary())));
    });
}
