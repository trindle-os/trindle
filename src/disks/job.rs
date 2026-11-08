//! Making a change: sending the steps one at a time, taking the leave back and saying how it ended.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::{Duration, Instant};
use zbus::Message;
use crate::toolkit::bus::Bus;
use crate::toolkit::event::Loop;
use crate::common::drive_leave;
use crate::common::ui::DISARM_SECONDS;
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use super::{Do, Drives, help, plan, spot};
use super::udisks::unknown;
use super::model::Vol;
use super::model::Drive;
use super::steps::Said;
use super::steps::Step;
use super::steps::told_of;
use super::steps::part_name;
use super::steps::Noted;
use super::steps::in_the_making;
use super::steps::NOTES_MOST;
use super::steps::NOTE_MOST;
use super::steps::changes;
use super::steps::Ended;
use super::steps::Done;
use super::steps::TAKE_BACK_TRIES;
use super::steps::STANDING_GAP;
use super::steps::take_back;
use super::steps::take_back_firmly;
use super::steps::take_back_between;
use super::steps::chain;
use super::steps::put_back;

impl Drives {
    pub(super) fn drive(&self, path: &str) -> Option<&Drive> {
        self.drives.iter().find(|d| d.path == path)
    }

    pub(super) fn vol(&self, drive: &str, path: &str) -> Option<(&Drive, &Vol)> {
        let d = self.drive(drive)?;
        Some((d, d.vols.iter().find(|v| v.path == path)?))
    }

    pub(super) fn told_page(&self, drive: &str, row: &str) -> Option<&'static str> {
        self.said.iter().find(|s| s.drive == drive && s.row == row && s.fresh() && !s.ok).map(|s| s.page)
    }

    pub(super) fn told(&self, drive: &str, row: &str) -> Option<(String, Tone)> {
        if let Some(w) = self.working.get(&spot(drive, row)) {

            let words = if self.stays { t!("disks-working-stays", doing = w, closing = plan::stays_open()) } else { t!("disks-working", doing = w) };
            return Some((words, Tone::Plain));
        }
        self.said.iter()
            .find(|s| s.drive == drive && s.row == row && s.fresh())
            .map(|s| (s.words.clone(), if s.ok { Tone::Plain } else { Tone::Problem }))
    }

    pub(super) fn say(&mut self, drive: &str, row: &str, words: String, ok: bool) {
        let page = help::of_outcome(row, &words);
        self.say_on(drive, row, words, ok, page);
    }

    pub(super) fn say_on(&mut self, drive: &str, row: &str, words: String, ok: bool, page: &'static str) {
        self.said.retain(|s| !(s.drive == drive && s.row == row));
        self.said.push(Said { drive: drive.into(), row: row.into(), words, ok, page, at: Instant::now() });
        let asks = self.asks.clone();
        self.events.after(Duration::from_secs(if ok { 3 } else { 60 }) + Duration::from_millis(100), move || {
            asks.send(Do::Redraw);
        });
    }

    pub(super) fn job(&mut self, drive: &str, row: &str, doing: &str, steps: Vec<Step>,
           after: impl FnOnce(Result<Vec<Message>, String>) -> Result<String, String> + 'static) {
        let (asks, d, r) = (self.asks.clone(), drive.to_string(), row.to_string());
        let done: Done = Box::new(move |ended| {
            asks.send(Do::Done { drive: d, row: r, res: after(ended.res), standing: ended.standing.is_some() });
        });
        match self.run(steps, Some((drive.to_string(), row.to_string())), done) {
            Err(why) => self.say(drive, row, why, false),

            Ok(false) => self.say(drive, row, plan::no_service().into(), false),
            Ok(true) => {
                self.working.insert(spot(drive, row), doing.to_string());
                self.busy += 1;
            }
        }
    }

    pub(super) fn run(&mut self, steps: Vec<Step>, on: Option<(String, String)>, done: Done) -> Result<bool, String> {
        let asked = changes(&steps);
        #[cfg(test)]
        self.ran.push(Ran { methods: steps.iter().map(|s| s.method).collect(), asked });
        if asked && self.change_going() {
            return Err(plan::one_at_a_time().into());
        }
        let Some(bus) = self.bus.clone() else { return Ok(false) };
        let lp = self.events.clone();

        let told = told_of(&steps);
        let noted = on.map(|(drive, row)| Rc::new(Noted::new(&self.noted, drive, row, told.clone())));
        if let Some(noted) = &noted {
            noted.write(0);
        }
        let going = Going { steps, told, noted, session: self.session.clone(), changing: self.changing.clone() };
        if !asked {
            run_unasked(bus, lp, going, done);
            return Ok(true);
        }

        drive_leave::prime();
        self.changing.set(true);
        run_asked(bus, lp, going, done);
        Ok(true)
    }

    pub(super) fn keep_taking_back(&mut self, drive: String, row: String, said: String, left: u32) {
        let Some(bus) = self.bus.clone() else { return };
        let (asks, lp, session, changing) = (self.asks.clone(), self.events.clone(), self.session.clone(), self.changing.clone());
        self.events.after(STANDING_GAP, move || {
            take_back_between(bus, lp, session, changing, Box::new(move |sure| {
                asks.send(Do::TookBack { drive, row, said, left, done: sure.is_ok() });
            }));
        });
    }

    pub(super) fn took_back(&mut self, drive: String, row: String, said: String, left: u32, done: bool) {
        if done {
            return self.say(&drive, &row, plan::taken_back_words(&said), true);
        }

        self.say(&drive, &row, plan::standing_words(&said), false);
        if left > 1 {
            self.keep_taking_back(drive, row, said, left - 1);
        }
    }

    pub(super) fn finish(&mut self) {
        self.busy = self.busy.saturating_sub(1);
        if self.busy == 0 {
            self.stays = false;
            self.refresh();
        }
    }

    pub(super) fn close(&mut self) {
        if self.may_close() {
            self.events.quit();
        }
    }

    pub(super) fn may_close(&mut self) -> bool {
        self.stays = self.busy > 0 || self.changing.get();
        !self.stays
    }

    pub(super) fn tell_what_was_left(&mut self) {
        let Ok(notes) = std::fs::read_dir(&self.noted) else { return };
        for note in notes.flatten().take(NOTES_MOST) {
            if in_the_making(&note.file_name().to_string_lossy()) {
                continue;
            }
            let file = note.path();

            let plain = std::fs::symlink_metadata(&file).is_ok_and(|m| m.is_file() && m.len() <= NOTE_MOST);
            let text = if plain { std::fs::read_to_string(&file).ok() } else { None };
            let _ = std::fs::remove_file(&file);
            if let Some((drive, row, steps, done)) = text.as_deref().and_then(plan::read_noted) {
                self.say(&drive, &row, plan::run_summed(&steps, done, None, &[]), false);
            }
        }
    }

    pub(super) fn soon(&mut self) {
        self.soon += 1;
        let (asks, gen) = (self.asks.clone(), self.soon);
        self.events.after(Duration::from_millis(500), move || {
            asks.send(Do::Soon(gen));
        });
    }

    pub(super) fn armed(&mut self, key: &str) -> bool {
        if self.armed.as_deref() == Some(key) {
            self.armed = None;
            return true;
        }
        self.armed = Some(key.to_string());
        self.arms += 1;
        let (asks, n) = (self.asks.clone(), self.arms);
        self.events.after(Duration::from_secs(DISARM_SECONDS.into()), move || {
            asks.send(Do::Disarm(n));
        });
        false
    }
}

struct Going {
    steps: Vec<Step>,
    told: Vec<plan::Did>,
    noted: Option<Rc<Noted>>,
    session: Option<String>,
    changing: Rc<Cell<bool>>,
}

fn run_unasked(bus: Bus, lp: Loop, going: Going, done: Done) {
    let Going { steps, told, noted, .. } = going;
    let ended = noted.clone();
    chain(bus, lp, steps.into(), Vec::new(), unknown, noted, Box::new(move |got, failed| {
        if let Some(noted) = &ended {
            noted.clear();
        }
        done(Ended {
            res: match failed {
                None => Ok(got),
                Some(stopped) => Err(plan::run_summed(&told, got.len(), Some(&stopped), &[])),
            },
            standing: None,
        })
    }));
}

fn run_asked(bus: Bus, lp: Loop, going: Going, done: Done) {
    let Going { steps, told, noted, session, changing } = going;
    let sent: Vec<(&'static str, String)> = steps.iter().map(|s| (s.method, s.path.clone())).collect();
    let (b, l) = (bus.clone(), lp.clone());
    take_back(&bus, &lp, session.clone(), move |sure| {
        if let Err(why) = sure {
            changing.set(false);
            if let Some(noted) = &noted {
                noted.clear();
            }
            return done(Ended { res: Err(t!("disks-not-sure-why", words = plan::not_sure(), why = &why)), standing: None });
        }
        let (b2, l2, ended) = (b.clone(), l.clone(), noted.clone());
        chain(b, l, steps.into(), Vec::new(), drive_leave::would_ask, noted, Box::new(move |got, failed| {

            let unmounted = match failed.as_ref().is_some_and(|f| plan::puts_back(&told, got.len(), f)) {
                true => parts_to_mount_again(&told, &sent, got.len()),
                false => VecDeque::new(),
            };
            let (b3, l3) = (b2.clone(), l2.clone());
            put_back(b2, l2, unmounted, Vec::new(), Box::new(move |back| {
                let res = match failed {
                    None => Ok(got),
                    Some(stopped) => Err(plan::run_summed(&told, got.len(), Some(&stopped), &back)),
                };
                take_back_firmly(b3, l3, session, TAKE_BACK_TRIES, Box::new(move |sure| {
                    changing.set(false);
                    if let Some(noted) = &ended {
                        noted.clear();
                    }
                    done(Ended { res, standing: sure.err() });
                }));
            }));
        }));
    });
}

fn parts_to_mount_again(told: &[plan::Did], sent: &[(&'static str, String)], done: usize) -> VecDeque<String> {
    let left = plan::left_unmounted(told, done);
    let mut paths: VecDeque<String> = VecDeque::new();
    for (method, path) in sent.iter().take(done) {
        if *method == "Unmount" && left.iter().any(|dev| dev == part_name(path)) && !paths.contains(path) {
            paths.push_back(path.clone());
        }
    }
    paths
}

#[cfg(test)]
use super::steps::Ran;
