//! What a circle's buttons do.

use std::time::Duration;

use crate::common::apps;
use crate::ring::{crashes, note_goes, notify, outside, windows};
use crate::ring::circles::Act;
use crate::ring::ctx::Do;
use crate::toolkit::t;

use super::{Content, Notes, now_ms};
use super::card::COPIED_FOR;

impl Notes {
    pub(super) fn act(&self, id: u32, act: Act) {
        let now = now_ms();
        match act {
            Act::Open | Act::Drive | Act::Allow => {
                let Some(c) = self.content.borrow().get(&id).cloned() else { return };
                if let Some(why) = self.open(&c) {
                    self.say_why(id, why);
                    return;
                }
                self.shut(id);
                self.sync();
                self.keep_keyboard();
                return;
            }
            Act::Copy => {
                self.copy(id, now);
                return;
            }
            Act::Close => self.shut(id),
            Act::Pin => {
                let pinned = self.stack.borrow_mut().toggle_pin(id, now);
                notify::set_pinned(id, pinned);
            }
            Act::Snooze => self.stack.borrow_mut().open_snooze(id, now),
            Act::Back => self.stack.borrow_mut().back(id, now),
            Act::Pick(choice) => {
                let t = crate::common::clock::Local::now();
                self.stack.borrow_mut().snooze(id, choice, (t.hour(), t.minute(), t.second()), now);
            }
            Act::ClearAll => {
                let gone = self.stack.borrow_mut().clear_all();
                let mut content = self.content.borrow_mut();
                for id in gone {
                    content.remove(&id);
                }
            }
        }
        self.sync();
        self.keep_keyboard();
        if matches!(act, Act::Snooze | Act::Back) && self.focused.get() == Some(id) {
            self.turn.set(self.turn.get().wrapping_add(1).max(1));
            let turn = self.turn.get();
            if let Some(w) = self.wins.borrow().get(&id) {
                w.shown.change(|c| c.grab = turn);
            }
        }
    }

    fn copy(&self, id: u32, now: u64) {
        let Some(c) = self.content.borrow().get(&id).cloned() else { return };
        let win = self.wins.borrow().get(&id).cloned();
        if let Some(w) = win {
            w.shown.window.copy(&c.whole());
            w.shown.announce(&t!("ring-copied-said", said = &c.said()));
        }
        self.copied.set(Some((id, now + COPIED_FOR)));
        self.sync();
        let me = self.me.clone();
        self.ctx.lp.after(Duration::from_millis(COPIED_FOR), move || {
            let Some(n) = me.upgrade() else { return };
            if n.copied.get().is_some_and(|(_, until)| until <= now_ms()) {
                n.copied.set(None);
                n.sync();
            }
        });
    }

    fn open(&self, c: &Content) -> Option<&'static str> {
        if !c.drive.is_empty() {
            self.ctx.tell(Do::OpenDrive(c.drive.clone()));
            return None;
        }
        if !c.allow.is_empty() {
            outside::allow_pressed(&c.allow);
            return None;
        }
        if !c.handoff.is_empty() {
            crashes::hand_off(&c.handoff);
            self.ctx.tell(Do::Notice(t!("ring-notes-copied").into()));
            if !notify::clicked(c.id) {
                self.ctx.tell(Do::ShowNote(c.id));
            }
            return None;
        }
        self.goes(c.id)
    }

    fn goes(&self, id: u32) -> Option<&'static str> {
        use note_goes::{plan, Goes, Sender, no_program, not_started};
        let note = notify::list().into_iter().find(|n| n.id == id)?;
        let (process, has_default) = (notify::process_of(id), notify::has_default(id));
        let all = self.ctx.apps();
        let open = if has_default { Vec::new() } else { windows::Windows::open_ids() };
        let sender = Sender { app: &note.app, desktop: &note.desktop, has_default, process: process.as_deref() };
        let entry = |key: &str| all.iter().find(|e| e.id == key);
        match plan(&all, &sender, &open) {
            Goes::Default => {
                if !notify::clicked(id) {
                    self.ctx.tell(Do::ShowNote(id));
                }
                None
            }
            Goes::Raise(key) => {
                let Some(e) = entry(&key) else { return Some(no_program()) };
                if windows::Windows::raise_for(&all, e) || apps::launch(e, &[]) { None } else { Some(not_started()) }
            }
            Goes::Start(key) => {
                let Some(e) = entry(&key) else { return Some(no_program()) };
                if apps::launch(e, &[]) { None } else { Some(not_started()) }
            }
            Goes::Why(why) => Some(why),
        }
    }

    pub(super) fn say_why(&self, id: u32, why: &str) {
        let Some(c) = self.content.borrow_mut().get_mut(&id).map(|c| { c.why = why.to_string(); c.clone() }) else { return };
        if let Some(w) = self.wins.borrow().get(&id).cloned() {
            let said = c.said();
            w.shown.change(|card| card.content = c);
            w.shown.announce(&said);
        }
        self.sync();
        self.keep_keyboard();
    }
}
