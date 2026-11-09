//! Each graphics card's screens start from a clean, known state.

use std::time::Duration;

pub const RETRY_AFTER: Duration = Duration::from_millis(500);

const LINE_EVERY: u32 = 600;
const FIRST_LINES: u32 = 5;

#[derive(Debug, Default)]
pub struct CleanStart {
    done: bool,
    retried: bool,

    waiting: bool,

    told: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refused {

    CleanAgain,

    Ladder { first: bool },
}

impl CleanStart {

    pub fn begin(&mut self) -> bool {
        let begin = !self.done;
        self.done = true;
        begin
    }

    pub fn waiting(&self) -> bool {
        self.waiting
    }

    pub fn refused(&mut self, lit: bool) -> Refused {
        if lit || !self.done {
            return Refused::Ladder { first: false };
        }
        if !self.retried {
            self.retried = true;
            self.waiting = true;
            return Refused::CleanAgain;
        }
        if self.waiting {

            return Refused::Ladder { first: false };
        }
        Refused::Ladder { first: !std::mem::replace(&mut self.told, true) }
    }

    pub fn due(&mut self) -> bool {
        std::mem::take(&mut self.waiting)
    }
}

pub fn worth_a_line(n: u32) -> bool {
    n <= FIRST_LINES || n % LINE_EVERY == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_card_starts_cleanly_once() {
        let mut c = CleanStart::default();
        assert!(c.begin());
        assert!(!c.begin());
        assert!(!c.begin());
    }

    #[test]
    fn a_refused_first_frame_gets_one_more_clean_start_only() {
        let mut c = CleanStart::default();
        c.begin();
        assert_eq!(c.refused(false), Refused::CleanAgain);
        assert!(c.waiting(), "nothing is drawn while it waits");
        assert!(c.due());
        assert!(!c.waiting());
        assert!(!c.due(), "the timer runs it once");

        assert_eq!(c.refused(false), Refused::Ladder { first: true });
        assert_eq!(c.refused(false), Refused::Ladder { first: false });
        assert!(!c.waiting());
    }

    #[test]
    fn a_lit_screen_keeps_the_ladder() {
        let mut c = CleanStart::default();
        c.begin();
        assert_eq!(c.refused(true), Refused::Ladder { first: false });
        assert!(!c.waiting());

        assert_eq!(c.refused(false), Refused::CleanAgain);
    }

    #[test]
    fn a_second_screen_rides_on_the_waiting_start() {
        let mut c = CleanStart::default();
        c.begin();
        assert_eq!(c.refused(false), Refused::CleanAgain);
        assert_eq!(c.refused(false), Refused::Ladder { first: false });
        assert!(c.waiting());
    }

    #[test]
    fn nothing_before_the_clean_start() {
        let mut c = CleanStart::default();
        assert_eq!(c.refused(false), Refused::Ladder { first: false });
        assert!(c.begin());
    }

    #[test]
    fn a_long_fault_writes_little() {
        let lines = (1..=6000).filter(|n| worth_a_line(*n)).count();
        assert_eq!(lines, 5 + 10);
    }
}
