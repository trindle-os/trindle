//! What the ring's keeper does about each word it is told, and when it holds the ring back.

use std::time::{Duration, Instant};

use super::{Told, TRIES, WINDOW};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    Nothing,
    Start,
    Fresh,
    Look,
    End,
    Wait(Duration),
    Unended,
}

pub(super) fn step(told: Told, running: Option<u64>, asked: bool, starts: &[Instant], now: Instant) -> Step {
    let start = || {
        let recent: Vec<&Instant> = starts.iter().filter(|t| now.duration_since(**t) < WINDOW).collect();
        match recent.first() {
            Some(oldest) if recent.len() >= TRIES => Step::Wait(WINDOW.saturating_sub(now.duration_since(**oldest))),
            _ => Step::Start,
        }
    };
    match told {
        Told::Now if running.is_none() && !asked => start(),
        Told::Again if running.is_none() && asked => start(),
        Told::Wished if running.is_none() && asked => Step::Fresh,
        Told::LetGo(n) if running == Some(n) => Step::Look,
        Told::Lingers(n) if running == Some(n) => Step::End,
        Told::Ended(n) if running == Some(n) => start(),
        Told::Unended(n) if running == Some(n) => Step::Unended,
        _ => Step::Nothing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: u64 = 3;

    fn at(now: Instant, ago: &[u64]) -> Vec<Instant> {
        ago.iter().map(|s| now - Duration::from_secs(*s)).collect()
    }

    #[test]
    fn the_ring_starts_once_when_the_start_up_says_now() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::Now, None, false, &[], now), Step::Start);
        assert_eq!(step(Told::Now, Some(1), true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::Now, None, true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::Again, None, false, &[], now), Step::Nothing);
        assert_eq!(step(Told::Ended(N), None, false, &[], now), Step::Nothing);
    }

    #[test]
    fn a_ring_that_ends_is_started_again() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::Ended(N), Some(N), true, &at(now, &[50]), now), Step::Start);
    }

    #[test]
    fn a_ring_that_lets_go_of_its_connection_is_ended_and_its_ending_starts_the_next() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::LetGo(N), Some(N), true, &at(now, &[50]), now), Step::Look);
        assert_eq!(step(Told::Lingers(N), Some(N), true, &at(now, &[50]), now), Step::End);
        assert_eq!(step(Told::Ended(N), Some(N), true, &at(now, &[50]), now), Step::Start);
    }

    #[test]
    fn a_ring_that_was_ended_is_not_said_to_have_let_go() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::LetGo(N), Some(N), true, &at(now, &[50]), now), Step::Look);
        assert_eq!(step(Told::Ended(N), Some(N), true, &at(now, &[50]), now), Step::Start);
        assert_eq!(step(Told::Lingers(N), Some(N + 1), true, &at(now, &[50, 0]), now), Step::Nothing);
        assert_eq!(step(Told::Lingers(N), None, true, &at(now, &[50]), now), Step::Nothing);
    }

    #[test]
    fn word_of_an_earlier_ring_touches_no_later_one() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::LetGo(N - 1), Some(N), true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::Ended(N - 1), Some(N), true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::LetGo(N), None, true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::Lingers(N - 1), Some(N), true, &at(now, &[1]), now), Step::Nothing);
        assert_eq!(step(Told::Unended(N - 1), Some(N), true, &at(now, &[1]), now), Step::Nothing);
    }

    #[test]
    fn a_ring_that_could_not_be_ended_is_said_and_nothing_is_started_in_its_place() {
        let now = Instant::now() + WINDOW * 2;
        assert_eq!(step(Told::Unended(N), Some(N), true, &at(now, &[50]), now), Step::Unended);
        for told in [Told::Now, Told::Again, Told::Wished, Told::Unended(N)] {
            assert!(!matches!(step(told, Some(N), true, &at(now, &[50]), now), Step::Start | Step::Fresh), "{told:?}");
        }
        assert_eq!(step(Told::Unended(N), None, true, &at(now, &[50]), now), Step::Nothing);
        assert_eq!(step(Told::Unended(N), Some(N + 1), true, &at(now, &[50, 0]), now), Step::Nothing);
        assert_eq!(step(Told::Ended(N), Some(N), true, &at(now, &[50]), now), Step::Start);
    }

    #[test]
    fn a_ring_that_keeps_falling_over_waits_and_is_never_given_up() {
        let now = Instant::now() + WINDOW * 2;
        let four = at(now, &[40, 30, 20, 10]);
        assert_eq!(step(Told::Ended(N), Some(N), true, &four, now), Step::Start);
        let five = at(now, &[50, 40, 30, 20, 10]);
        assert_eq!(step(Told::Ended(N), Some(N), true, &five, now), Step::Wait(WINDOW - Duration::from_secs(50)));
        let later = now + WINDOW - Duration::from_secs(50);
        assert_eq!(step(Told::Again, None, true, &five, later), Step::Start);
        let old = at(now, &[900, 800, 700, 650, 610, 10]);
        assert_eq!(step(Told::Ended(N), Some(N), true, &old, now), Step::Start);
    }

    #[test]
    fn a_ring_held_back_starts_at_once_when_the_person_asks_for_it() {
        let now = Instant::now() + WINDOW * 2;
        let five = at(now, &[50, 40, 30, 20, 10]);
        assert_eq!(step(Told::Ended(N), Some(N), true, &five, now), Step::Wait(WINDOW - Duration::from_secs(50)));
        assert_eq!(step(Told::Wished, None, true, &five, now), Step::Fresh);
        assert_eq!(step(Told::Wished, Some(N), true, &five, now), Step::Nothing);
        assert_eq!(step(Told::Wished, None, false, &[], now), Step::Nothing);
        let mut starts = vec![now];
        for n in 1..TRIES as u64 {
            assert_eq!(step(Told::Ended(n), Some(n), true, &starts, now), Step::Start);
            starts.push(now);
        }
        assert_eq!(step(Told::Ended(N), Some(N), true, &starts, now), Step::Wait(WINDOW));
        assert_eq!(step(Told::Again, Some(N), true, &starts, now), Step::Nothing);
    }
}
