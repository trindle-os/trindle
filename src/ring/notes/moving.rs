//! Moving a pinned circle by hand: picking it up, dragging it, and dropping or putting it back.

use crate::toolkit::{t, theme};
use crate::toolkit::window::Keys;

use super::{Notes, now_ms};

impl Notes {

    pub(super) fn pick_up(&self, id: u32) {
        let win = self.wins.borrow().get(&id).cloned();
        let Some(w) = win else { return };
        let local = w.shown.with(|c| c.last.get());
        if !self.stack.borrow_mut().pick_up(id, local, self.corner.get(), self.screen(), now_ms()) {
            if !self.stack.borrow().pinned(id) {
                self.say_why(id, t!("ring-notes-pin-to-move"));
            }
            return;
        }
        w.shown.window.set_keys(Keys::Always);
        w.shown.announce(t!("ring-notes-moving"));
        self.sync();
    }

    pub(super) fn drag(&self, id: u32, local: (f32, f32)) {
        let Some(window) = self.wins.borrow().get(&id).map(|w| w.placed.get()) else { return };
        self.stack.borrow_mut().drag(id, window, local, self.screen(), now_ms());
        self.sync();
    }

    pub(super) fn drop_held(&self, id: u32) {
        self.stack.borrow_mut().drop_held(id);
        self.after_hold(id);
        if let Some(w) = self.wins.borrow().get(&id) {
            w.shown.announce(t!("ring-notes-moved"));
        }
        self.sync();
    }

    pub(super) fn put_back(&self, id: u32) {
        self.stack.borrow_mut().put_back(id);
        self.after_hold(id);
        if let Some(w) = self.wins.borrow().get(&id) {
            w.shown.announce(t!("ring-notes-put-back"));
        }
        self.sync();
    }

    pub(super) fn placed(&self, id: u32) {
        let Some(w) = self.wins.borrow().get(&id).cloned() else { return };
        if let Some((at, _)) = w.asked.take() {
            w.placed.set(at);
        }
        self.sync();
    }

    fn after_hold(&self, id: u32) {
        if self.focused.get() == Some(id) {
            return;
        }
        if let Some(w) = self.wins.borrow().get(&id) {
            w.shown.window.set_keys(Keys::Never);
        }
    }

    pub(super) fn nudge(&self, id: u32, right: i8, down: i8) {
        let step = theme::sheet().circle_step;
        let moved = self.stack.borrow_mut().nudge(id, (right as f32 * step, down as f32 * step), self.corner.get(), self.screen(), now_ms());
        if !moved {
            self.say_why(id, t!("ring-notes-pin-to-move"));
            return;
        }
        if let Some(w) = self.wins.borrow().get(&id) {
            w.shown.announce(t!("ring-notes-moved"));
        }
        self.sync();
    }
}
