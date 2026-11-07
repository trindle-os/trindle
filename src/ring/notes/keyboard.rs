//! When a notification circle holds the keyboard.

use crate::toolkit::window::Keys;

use super::{Notes, now_ms};

impl Notes {
    pub fn take_keyboard(&self) {
        let ids = self.ids();
        let next = match self.focused.get().and_then(|f| ids.iter().position(|i| *i == f)) {
            Some(at) => ids.get(at + 1).copied(),
            None => ids.first().copied(),
        };
        match next {
            Some(id) => self.focus_on(id),
            None => self.leave(),
        }
    }

    fn focus_on(&self, id: u32) {
        if let Some(old) = self.focused.replace(Some(id)).filter(|old| *old != id) {
            self.release(old);
        }
        let grown = self.grown(id);
        self.stack.borrow_mut().point(id, true, grown, now_ms());
        self.sync();
        self.turn.set(self.turn.get().wrapping_add(1).max(1));
        let turn = self.turn.get();
        let win = self.wins.borrow().get(&id).cloned();
        if let Some(w) = win {
            w.shown.window.set_keys(Keys::Always);
            w.shown.change(|c| c.grab = turn);
        }
    }

    fn release(&self, id: u32) {
        let rest = self.stack.borrow().rest(id);
        self.stack.borrow_mut().point(id, false, rest, now_ms());
        let win = self.wins.borrow().get(&id).cloned();
        if let Some(w) = win {
            w.shown.window.set_keys(Keys::Never);
            w.shown.change(|c| c.grab = 0);
        }
    }

    pub(super) fn leave(&self) {
        if let Some(id) = self.focused.take() {
            self.release(id);
            self.label.hide();
            self.sync();
        }
    }

    pub(super) fn keep_keyboard(&self) {
        let Some(id) = self.focused.get() else { return };
        if self.stack.borrow().has(id) {
            return;
        }
        self.focused.set(None);
        match self.ids().first().copied() {
            Some(next) => self.focus_on(next),
            None => self.label.hide(),
        }
    }
}
