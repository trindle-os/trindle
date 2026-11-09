//! How often something a plugin can set off may happen. A plugin may be
//! written by anyone, so what it can start is counted: a limit says no past
//! so many in a stretch of time, and never queues the rest for later.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Rate {
    max: usize,
    per: Duration,
    hits: VecDeque<Instant>,
}

impl Rate {
    pub fn new(max: usize, per: Duration) -> Rate {
        Rate { max, per, hits: VecDeque::new() }
    }

    pub fn allow_at(&mut self, now: Instant) -> bool {
        while self.hits.front().is_some_and(|t| now.saturating_duration_since(*t) >= self.per) {
            self.hits.pop_front();
        }
        if self.hits.len() >= self.max {
            return false;
        }
        self.hits.push_back(now);
        true
    }

    pub fn allow(&mut self) -> bool {
        self.allow_at(Instant::now())
    }
}

pub const SHORTCUTS_PER_SECOND: usize = 8;

pub const TAKES_PER_SECOND: usize = 2;

pub const SLOW_CLIENT_ROOM: usize = 8 << 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn so_many_in_a_second_and_then_no_until_the_second_is_up() {
        let t0 = Instant::now();
        let mut r = Rate::new(2, Duration::from_secs(1));
        assert!(r.allow_at(t0));
        assert!(r.allow_at(t0 + Duration::from_millis(100)));
        assert!(!r.allow_at(t0 + Duration::from_millis(200)));
        assert!(!r.allow_at(t0 + Duration::from_millis(999)));

        assert!(r.allow_at(t0 + Duration::from_millis(1000)));
        assert!(!r.allow_at(t0 + Duration::from_millis(1050)));
        assert!(r.allow_at(t0 + Duration::from_millis(1100)));
    }

    #[test]
    fn a_flood_never_grows_what_is_kept() {
        let t0 = Instant::now();
        let mut r = Rate::new(8, Duration::from_secs(1));
        let granted = (0..100_000u64).filter(|i| r.allow_at(t0 + Duration::from_micros(*i))).count();
        assert_eq!(granted, 8);
        assert!(r.hits.len() <= 8);
    }
}
