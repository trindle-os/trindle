//! Timers that run only while something can be seen, and bursts of news gathered into one look.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::toolkit::event::{Flow, Loop};

#[derive(Default)]
pub struct Ticker {

    run: Rc<Cell<u32>>,
    made: Cell<u32>,
}

impl Ticker {

    fn begin(&self) -> Option<u32> {
        if self.run.get() != 0 {
            return None;
        }
        let id = self.made.get().checked_add(1).unwrap_or(1);
        self.made.set(id);
        self.run.set(id);
        Some(id)
    }

    pub fn start(&self, lp: &Loop, every: Duration, f: impl Fn() -> Flow + 'static) {
        let Some(id) = self.begin() else { return };
        let run = self.run.clone();
        lp.every(every, move || if run.get() == id { f() } else { Flow::Stop });
    }

    pub fn stop(&self) {
        self.run.set(0);
    }
}

#[derive(Default)]
pub struct Burst(Rc<Cell<bool>>);

impl Burst {

    fn arm(&self) -> bool {
        !self.0.replace(true)
    }

    pub fn poke(&self, lp: &Loop, wait: Duration, f: impl FnOnce() + 'static) {
        if self.arm() {
            let waiting = self.0.clone();
            lp.after(wait, move || {
                waiting.set(false);
                f();
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ticker_starts_once_and_a_stopped_run_ends_for_good() {
        let t = Ticker::default();
        let first = t.begin().expect("starts");
        assert!(t.begin().is_none(), "already going");
        t.stop();
        assert_ne!(t.run.get(), first, "the old run no longer matches");
        let second = t.begin().expect("starts again");
        assert_ne!(first, second);
        assert_eq!(t.run.get(), second);
    }

    #[test]
    fn a_burst_of_signals_is_one_call() {
        let b = Burst::default();
        assert!(b.arm());
        assert!(!b.arm() && !b.arm());
        b.0.set(false);
        assert!(b.arm());
    }
}
