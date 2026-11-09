//! A screen whose commits keep being refused is set up again in full.

use std::time::{Duration, Instant};

const STEPS: [Duration; 4] = [
    Duration::from_millis(250),
    Duration::from_secs(1),
    Duration::from_secs(2),
    Duration::from_secs(5),
];

pub const CAP: Duration = Duration::from_secs(15);

const LINE_AT_CAP_EVERY: u32 = 20;

pub fn wait_after(attempt: u32) -> Duration {
    STEPS.get(attempt.saturating_sub(1) as usize).copied().unwrap_or(CAP)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {

    Wait(Duration),

    SetUp { attempt: u32, say: bool, slow: bool },
}

#[derive(Debug, Default)]
pub struct Backoff {
    attempts: u32,
    next_at: Option<Instant>,
}

impl Backoff {

    pub fn refused(&mut self, now: Instant) -> Decision {
        if let Some(at) = self.next_at {
            if now < at {
                return Decision::Wait(at - now);
            }
        }
        self.attempts = self.attempts.saturating_add(1);
        let wait = wait_after(self.attempts);
        self.next_at = Some(now + wait);
        let slow = wait == CAP;
        let into_cap = self.attempts.saturating_sub(STEPS.len() as u32);
        let say = self.attempts <= 2 || (slow && (into_cap == 1 || into_cap % LINE_AT_CAP_EVERY == 0));
        Decision::SetUp { attempt: self.attempts, say, slow }
    }

    pub fn good(&mut self) -> bool {
        let was = self.attempts > 0;
        *self = Backoff::default();
        was
    }

    #[cfg(test)]
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Retaken {

    AsWas,

    StartedOver,
}

pub fn retake<E>(gentle: bool, mut activate: impl FnMut(bool) -> Result<(), E>) -> Result<Retaken, E> {
    if gentle && activate(false).is_ok() {
        return Ok(Retaken::AsWas);
    }
    activate(true).map(|()| Retaken::StartedOver)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCard {
        good_after: Option<u32>,
        full_setups: u32,
        gentle_calls: u32,
        commits: u32,
    }

    impl FakeCard {
        fn new(good_after: Option<u32>) -> Self {
            FakeCard { good_after, full_setups: 0, gentle_calls: 0, commits: 0 }
        }
        fn activate(&mut self, full: bool) -> Result<(), &'static str> {
            if full {
                self.full_setups += 1;
            } else {
                self.gentle_calls += 1;
            }
            Ok(())
        }
        fn commit(&mut self) -> Result<(), &'static str> {
            self.commits += 1;
            match self.good_after {
                Some(n) if self.full_setups >= n => Ok(()),
                _ => Err("EINVAL"),
            }
        }
    }

    fn run(card: &mut FakeCard, seconds: u64) -> (Vec<Duration>, Option<Duration>, Backoff) {
        let start = Instant::now();
        let mut backoff = Backoff::default();
        let mut setups = Vec::new();
        let frame = Duration::from_millis(66);
        let mut t = Duration::ZERO;
        let mut next_try = Duration::ZERO;
        let mut ended = None;
        while t < Duration::from_secs(seconds) {
            if t >= next_try {
                match card.commit() {
                    Ok(()) => {
                        if backoff.good() {
                            ended = Some(t);
                        }
                        break;
                    }
                    Err(_) => match backoff.refused(start + t) {
                        Decision::SetUp { .. } => {
                            retake(false, |full| card.activate(full)).unwrap();
                            setups.push(t);
                            next_try = t + frame;
                        }
                        Decision::Wait(w) => next_try = t + w,
                    },
                }
            }
            t += frame;
        }
        (setups, ended, backoff)
    }

    #[test]
    fn the_first_refusal_is_followed_by_a_full_modeset() {
        let mut card = FakeCard::new(Some(1));
        let (setups, ended, _) = run(&mut card, 5);
        assert_eq!(card.full_setups, 1);
        assert_eq!(setups, vec![Duration::ZERO], "straight after the first refusal");
        assert!(ended.is_some(), "the next commit is good");
    }

    #[test]
    fn a_good_commit_stops_the_recovery() {
        let mut card = FakeCard::new(Some(3));
        let (setups, ended, backoff) = run(&mut card, 30);
        assert_eq!(setups.len(), 3);
        assert!(ended.is_some());
        assert_eq!(backoff.attempts(), 0, "the run is cleared");
        let commits = card.commits;
        let (more, _, _) = run(&mut card, 30);
        assert!(more.is_empty(), "no more set-ups once it is good");
        assert_eq!(card.commits, commits + 1);
    }

    #[test]
    fn the_waits_grow_and_stop_at_the_cap() {
        let mut card = FakeCard::new(None);
        let (setups, ended, _) = run(&mut card, 120);
        assert!(ended.is_none());
        let gaps: Vec<Duration> = setups.windows(2).map(|w| w[1] - w[0]).collect();

        let slack = Duration::from_millis(70);
        for (i, gap) in gaps.iter().enumerate() {
            let want = wait_after(i as u32 + 1);
            assert!(*gap >= want && *gap <= want + slack, "gap {i}: {gap:?} for {want:?}");
        }
        assert!(gaps.windows(2).all(|g| g[1] + slack >= g[0]), "never shorter than before");
        assert!(*gaps.last().unwrap() >= CAP, "the last gaps are at the cap");
        assert_eq!(wait_after(1), Duration::from_millis(250));
        assert_eq!(wait_after(50), CAP);

        assert!(setups.len() < 20, "{} set-ups", setups.len());
        assert!(card.commits < 40, "{} commits", card.commits);
        assert!(setups.len() > 8, "it keeps trying at the cap");
    }

    #[test]
    fn it_says_so_at_the_cap_and_then_now_and_then() {
        let mut b = Backoff::default();
        let mut t = Instant::now();
        let mut said = Vec::new();
        for _ in 0..80 {
            match b.refused(t) {
                Decision::SetUp { attempt, say, slow } => {
                    if say {
                        said.push((attempt, slow));
                    }
                    t += wait_after(attempt);
                }
                Decision::Wait(_) => unreachable!(),
            }
        }
        assert_eq!(said[0], (1, false));
        assert_eq!(said[1], (2, false));
        assert_eq!(said[2], (5, true), "the first set-up at the cap");
        assert!(said.len() <= 7, "{said:?}");
    }

    #[test]
    fn a_wait_names_how_long_is_left() {
        let mut b = Backoff::default();
        let t = Instant::now();
        assert!(matches!(b.refused(t), Decision::SetUp { attempt: 1, .. }));
        assert_eq!(b.refused(t + Duration::from_millis(100)), Decision::Wait(Duration::from_millis(150)));
        assert!(matches!(b.refused(t + Duration::from_millis(250)), Decision::SetUp { attempt: 2, .. }));
    }

    #[test]
    fn resume_and_recovery_share_one_function() {

        let mut card = FakeCard::new(None);
        assert_eq!(retake(true, |f| card.activate(f)), Ok(Retaken::AsWas));
        assert_eq!((card.gentle_calls, card.full_setups), (1, 0));
        let mut calls = Vec::new();
        let r: Result<Retaken, &str> = retake(true, |full| {
            calls.push(full);
            if full { Ok(()) } else { Err("busy") }
        });
        assert_eq!(r, Ok(Retaken::StartedOver));
        assert_eq!(calls, vec![false, true]);

        let mut card = FakeCard::new(None);
        assert_eq!(retake(false, |f| card.activate(f)), Ok(Retaken::StartedOver));
        assert_eq!((card.gentle_calls, card.full_setups), (0, 1));

        let r: Result<Retaken, &str> = retake(false, |_| Err("no"));
        assert_eq!(r, Err("no"));
    }
}
