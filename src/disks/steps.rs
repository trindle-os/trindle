//! The steps of a change, the notes kept while one is made and the leave taken back around it.

use std::cell::Cell;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};
use serde::Serialize;
use zbus::Message;
use zbus::zvariant::DynamicType;
use crate::common::drive_leave;
use crate::toolkit::bus::{Bus, Failed};
use crate::toolkit::event::Loop;
use super::{INTERACTIVE, LONG, plan};
use super::udisks::UD;
use super::udisks::FILESYSTEM;
use super::udisks::Opts;

pub(super) struct Said {
    pub(super) drive: String,
    pub(super) row: String,
    pub(super) words: String,
    pub(super) ok: bool,

    pub(super) page: &'static str,
    pub(super) at: Instant,
}

impl Said {

    pub(super) fn fresh(&self) -> bool {
        self.at.elapsed() < Duration::from_secs(if self.ok { 3 } else { 60 })
    }
}

#[derive(Clone)]
pub(super) struct Carried {
    pub(super) drive: String,
    pub(super) row: String,
    pub(super) made: String,
    pub(super) dir: String,
    pub(super) startup: bool,
    pub(super) fs: String,
}

type Sent = Box<dyn FnOnce(Result<Message, Failed>)>;

pub(super) struct Step {
    pub(super) method: &'static str,
    pub(super) path: String,

    pub(super) what: String,
    pub(super) send: Box<dyn FnOnce(&Bus, &Loop, Sent)>,
}

impl Step {

    pub(super) fn called(mut self, what: &str) -> Step {
        self.what = what.to_string();
        self
    }
}

pub(super) fn step<B: Serialize + DynamicType + 'static>(path: &str, iface: &'static str, method: &'static str, body: B) -> Step {
    let at = path.to_string();
    let wait = if matches!(method, "Unmount" | "Mount") { INTERACTIVE } else { LONG };
    let what = plan::step_words(iface.rsplit('.').next().unwrap_or(iface), method, path.rsplit('/').next().unwrap_or(path));
    Step { method, path: path.to_string(), what, send: Box::new(move |bus, lp, done| bus.call(UD, &at, iface).method(method, &body, true, Some(wait), lp, done)) }
}

pub(super) fn told_of(steps: &[Step]) -> Vec<plan::Did> {
    steps.iter().map(|s| {
        let dev = part_name(&s.path).to_string();
        let mounts = match s.method {
            "Unmount" => Some((dev, false)),
            "Mount" => Some((dev, true)),
            _ => None,
        };
        plan::Did { what: s.what.clone(), changes: !plan::FREE.contains(&s.method), mounts }
    }).collect()
}

pub(super) fn part_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

pub(super) struct Noted {
    file: PathBuf,
    drive: String,
    row: String,
    steps: Vec<plan::Did>,
}

impl Noted {

    pub(super) fn new(notes: &std::path::Path, drive: String, row: String, steps: Vec<plan::Did>) -> Noted {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let me = std::process::id();
        Noted { file: notes.join(format!("{me}-{}-{n}", began(me).unwrap_or(0))), drive, row, steps }
    }

    pub(super) fn write(&self, done: usize) {
        if let Some(notes) = self.file.parent() {
            let _ = std::fs::create_dir_all(notes);
        }
        let _ = crate::common::files::write_text(&self.file, &plan::noted(&self.drive, &self.row, &self.steps, done));
    }

    pub(super) fn clear(&self) {
        let _ = std::fs::remove_file(&self.file);
    }
}

pub(super) fn began(pid: u32) -> Option<u64> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    stat.rsplit_once(')')?.1.split_whitespace().nth(19)?.parse().ok()
}

pub(super) fn in_the_making(name: &str) -> bool {
    let mut parts = name.split('-');
    let (Some(pid), Some(at)) = (parts.next().and_then(|p| p.parse::<u32>().ok()), parts.next().and_then(|a| a.parse::<u64>().ok())) else { return false };
    pid != std::process::id() && began(pid) == Some(at)
}

pub(super) const NOTES_MOST: usize = 64;
pub(super) const NOTE_MOST: u64 = 64 * 1024;

pub(super) fn noted_folder() -> PathBuf {
    if cfg!(test) {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::env::temp_dir().join(format!("trindle-drives-runs-{}-{n}", std::process::id()))
    } else {
        crate::common::files::cache_home().join("trindle").join("drives-runs")
    }
}

pub(super) fn changes(steps: &[Step]) -> bool {
    told_of(steps).iter().any(|s| s.changes)
}

#[cfg(test)]
pub(super) struct Ran {
    pub(super) methods: Vec<&'static str>,
    pub(super) asked: bool,
}

pub(super) fn unmount(path: &str) -> Step {
    step(path, FILESYSTEM, "Unmount", (Opts::new(),))
}

pub(super) fn mount(path: &str) -> Step {
    step(path, FILESYSTEM, "Mount", (Opts::new(),))
}

pub(super) struct Ended {
    pub(super) res: Result<Vec<Message>, String>,
    pub(super) standing: Option<String>,
}

pub(super) type Done = Box<dyn FnOnce(Ended)>;

pub(super) const TAKE_BACK_TRIES: u32 = 3;
const TAKE_BACK_GAP: Duration = Duration::from_millis(if cfg!(test) { 20 } else { 1500 });

pub(super) const STANDING_GAP: Duration = Duration::from_millis(if cfg!(test) { 30 } else { 15_000 });
pub(super) const STANDING_TRIES: u32 = 21;

pub(super) fn take_back(bus: &Bus, lp: &Loop, session: Option<String>, done: impl FnOnce(Result<(), String>) + 'static) {
    match session {
        Some(sid) => drive_leave::take_back_for(bus, lp, &sid, done),
        None => drive_leave::take_back(bus, lp, done),
    }
}

pub(super) fn take_back_firmly(bus: Bus, lp: Loop, session: Option<String>, tries: u32, done: Box<dyn FnOnce(Result<(), String>)>) {
    let (b, l, s) = (bus.clone(), lp.clone(), session.clone());
    take_back(&bus, &lp, session, move |sure| match sure {
        Err(_) if tries > 1 => {
            let again = l.clone();
            again.after(TAKE_BACK_GAP, move || take_back_firmly(b, l, s, tries - 1, done));
        }
        sure => done(sure),
    });
}

pub(super) fn take_back_between(bus: Bus, lp: Loop, session: Option<String>, changing: Rc<Cell<bool>>, done: Box<dyn FnOnce(Result<(), String>)>) {
    if changing.get() {
        let again = lp.clone();
        again.after(STANDING_GAP, move || take_back_between(bus, lp, session, changing, done));
        return;
    }
    take_back(&bus, &lp, session, done);
}

type Chained = Box<dyn FnOnce(Vec<Message>, Option<plan::Stopped>)>;

pub(super) fn chain(bus: Bus, lp: Loop, mut steps: VecDeque<Step>, mut got: Vec<Message>, asks: fn() -> Option<bool>, noted: Option<Rc<Noted>>, done: Chained) {
    let Some(s) = steps.pop_front() else { return done(got, None) };
    let (b, l) = (bus.clone(), lp.clone());
    (s.send)(&bus, &lp, Box::new(move |r| match r {
        Err(e) => done(got, Some(plan::stopped(&e.name, &e.text, asks))),
        Ok(m) => {
            got.push(m);
            if let Some(noted) = &noted {
                noted.write(got.len());
            }
            chain(b, l, steps, got, asks, noted, done)
        }
    }));
}

type Back = Vec<(String, Option<plan::Stopped>)>;

pub(super) fn put_back(bus: Bus, lp: Loop, mut paths: VecDeque<String>, mut back: Back, done: Box<dyn FnOnce(Back)>) {
    let Some(path) = paths.pop_front() else { return done(back) };
    let (b, l) = (bus.clone(), lp.clone());
    bus.call(UD, &path.clone(), FILESYSTEM).method("Mount", &(Opts::new(),), false, Some(INTERACTIVE), &lp, move |r| {
        back.push((part_name(&path).to_string(), r.err().map(|e| plan::stopped(&e.name, &e.text, || None))));
        put_back(b, l, paths, back, done)
    });
}
