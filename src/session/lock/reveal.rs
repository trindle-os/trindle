//! The eye beside what is typed, which shows it while it is on.

use std::time::{Duration, Instant};

use crate::common::eye;

use super::HINT;
use super::keyboard::Key;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Eye {
    pub(super) shown: bool,

    pub(super) on: bool,

    pub(super) named: Option<Instant>,

    pub(super) used: Option<Instant>,
}

impl Eye {

    pub(super) fn press(&mut self, now: Instant) {
        self.shown = !self.shown;
        self.used = Some(now);
    }

    pub(super) fn key(&mut self, key: Key, now: Instant) -> bool {
        self.used = Some(now);
        match key {
            Key::Tab => {
                self.on = !self.on;
                self.named = self.on.then_some(now);
                true
            }
            Key::Enter | Key::Char(' ') if self.on => {
                self.press(now);
                true
            }
            Key::Char(_) | Key::Erase if self.on => {
                self.on = false;
                self.named = None;
                false
            }
            _ => false,
        }
    }

    pub(super) fn shut(&mut self) {
        *self = Eye::default();
    }

    pub(super) fn rest(&mut self, now: Instant) -> bool {
        let left = self.shown && self.used.is_none_or(|at| eye::rested(at, now));
        if left {
            self.shown = false;
        }
        left
    }

    pub(super) fn wake(&self, now: Instant) -> Option<Duration> {
        let rest = self.used.filter(|_| self.shown).map(|at| (at + eye::REST).saturating_duration_since(now));
        let name = self.named.map(|at| (at + HINT).saturating_duration_since(now));
        [rest, name].into_iter().flatten().min()
    }

    pub(super) fn name_showing(&self, now: Instant) -> bool {
        self.named.is_some_and(|at| now.saturating_duration_since(at) < HINT)
    }
}
