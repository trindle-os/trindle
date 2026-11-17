//! The lock's wrong tries and when it locked, kept in the login's runtime
//! folder rather than in the lock screen, so one started again after a crash
//! carries on with the same count, the same PIN minutes and the same wait.

use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::common::pin;

const FREE: u32 = 3;

const FIRST_WAIT: Duration = Duration::from_secs(5);
const LONGEST_WAIT: Duration = Duration::from_secs(60);

const MOST_BYTES: u64 = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::session) struct Tries {

    locked_at: u64,
    pins: u32,
    wrongs: u32,
    last_wrong: u64,
}

pub(in crate::session) fn boot_now() -> Option<u64> {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: fills the local timespec; CLOCK_BOOTTIME exists on every Linux Trindle runs on.
    if unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut t) } != 0 {
        return None;
    }
    let secs = u64::try_from(t.tv_sec).ok()?;
    Some(secs.saturating_mul(1000).saturating_add(u64::try_from(t.tv_nsec).ok()? / 1_000_000))
}

pub(in crate::session) fn file() -> PathBuf {
    crate::common::files::scratch().join("trindle-lock.tries")
}

fn wait_after(wrongs: u32) -> Duration {
    if wrongs <= FREE {
        return Duration::ZERO;
    }
    FIRST_WAIT.checked_mul(1u32.checked_shl(wrongs - FREE - 1).unwrap_or(u32::MAX)).map_or(LONGEST_WAIT, |w| w.min(LONGEST_WAIT))
}

impl Tries {
    pub(in crate::session) fn fresh(now: u64) -> Self {
        Tries { locked_at: now, pins: 0, wrongs: 0, last_wrong: 0 }
    }

    fn spent(now: u64) -> Self {
        Tries { locked_at: 0, pins: pin::TRIES, wrongs: u32::MAX, last_wrong: now }
    }

    fn parse(text: &str) -> Option<Self> {
        let n: Vec<u64> = text.split_whitespace().map(str::parse).collect::<Result<_, _>>().ok()?;
        let [locked_at, pins, wrongs, last_wrong] = n[..] else { return None };
        Some(Tries { locked_at, pins: u32::try_from(pins).ok()?, wrongs: u32::try_from(wrongs).ok()?, last_wrong })
    }

    pub(in crate::session) fn load(path: &Path, now: u64) -> Self {
        let mut text = String::new();
        let read = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW).open(path)
            .and_then(|f| f.take(MOST_BYTES + 1).read_to_string(&mut text));
        match read {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Tries::fresh(now),
            Ok(n) if (n as u64) <= MOST_BYTES => Tries::parse(&text).unwrap_or(Tries::spent(now)),
            _ => Tries::spent(now),
        }
    }

    pub(in crate::session) fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600)
            .custom_flags(libc::O_NOFOLLOW).open(path)?;
        writeln!(f, "{} {} {} {}", self.locked_at, self.pins, self.wrongs, self.last_wrong)
    }

    pub(in crate::session) fn clear(path: &Path) -> std::io::Result<()> {
        match std::fs::remove_file(path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }

    pub(in crate::session) fn pin_allowed(&self, now: Option<u64>, minutes: u32) -> bool {
        let Some(now) = now else { return false };
        self.pins < pin::TRIES && now >= self.locked_at && now - self.locked_at < u64::from(minutes) * 60_000
    }

    pub(in crate::session) fn wait_left(&self, now: Option<u64>) -> Option<Duration> {
        let wait = wait_after(self.wrongs);
        let now = now?;
        if wait.is_zero() {
            return None;
        }

        let since = now.checked_sub(self.last_wrong).map_or(Duration::ZERO, Duration::from_millis);
        Some(wait.saturating_sub(since)).filter(|w| !w.is_zero())
    }

    pub(in crate::session) fn wrong(&mut self, now: Option<u64>, pin: bool) -> u32 {
        self.wrongs = self.wrongs.saturating_add(1);
        self.last_wrong = now.unwrap_or(0);
        if pin {
            self.pins = self.pins.saturating_add(1);
        }
        pin::TRIES.saturating_sub(self.pins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: u64 = 60_000;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-tries-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_lock_screen_started_again_carries_on_with_the_same_count() {
        let dir = scratch("again");
        let path = dir.join("tries");
        let mut first = Tries::load(&path, 1000);
        assert_eq!(first, Tries::fresh(1000));
        for _ in 0..4 {
            first.wrong(Some(2000), true);
        }
        first.save(&path).unwrap();
        let second = Tries::load(&path, 9 * MIN);
        assert_eq!(second, first, "the second lock screen has the first one's count");
        let mut second = second;
        assert_eq!(second.wrong(Some(3000), true), 0);
        assert!(!second.pin_allowed(Some(4000), 15), "five wrong PINs in all: the password only");
        Tries::clear(&path).unwrap();
        assert_eq!(Tries::load(&path, 5 * MIN), Tries::fresh(5 * MIN), "unlocked, the next lock starts afresh");
        Tries::clear(&path).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_file_that_is_not_ours_counts_as_every_try_used() {
        let dir = scratch("bad");
        let path = dir.join("tries");
        for text in ["", "1 2 3", "a b c d", "1 2 3 4 5", "1 99999999999 0 0", &"9".repeat(200)] {
            std::fs::write(&path, text).unwrap();
            let t = Tries::load(&path, 1000);
            assert!(!t.pin_allowed(Some(1000), 120), "{text:?}");
            assert_eq!(t.wait_left(Some(1000)), Some(LONGEST_WAIT), "{text:?}");
        }
        std::fs::remove_file(&path).unwrap();
        std::os::unix::fs::symlink(dir.join("elsewhere"), &path).unwrap();
        std::fs::write(dir.join("elsewhere"), "0 0 0 0").unwrap();
        assert_eq!(Tries::load(&path, 1000), Tries::spent(1000), "a link is not followed");
        assert!(Tries::fresh(1).save(&path).is_err(), "nor written through");
        assert_eq!(std::fs::read_to_string(dir.join("elsewhere")).unwrap(), "0 0 0 0");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_pin_runs_out_on_the_boot_clock() {
        let t = Tries::fresh(10 * MIN);
        assert!(t.pin_allowed(Some(10 * MIN), 15));
        assert!(t.pin_allowed(Some(25 * MIN - 1), 15));
        assert!(!t.pin_allowed(Some(25 * MIN), 15));
        assert!(!t.pin_allowed(Some(MIN), 15), "a clock behind the lock is no time");
        assert!(!t.pin_allowed(None, 15), "no clock, no PIN");
        assert!(boot_now().is_some_and(|n| n > 0));
    }

    #[test]
    fn wrong_tries_cost_a_growing_wait() {
        let mut t = Tries::fresh(0);
        for _ in 0..FREE {
            t.wrong(Some(1000), false);
            assert_eq!(t.wait_left(Some(1000)), None);
        }
        let mut last = Duration::ZERO;
        for _ in 0..8 {
            t.wrong(Some(1000), false);
            let w = t.wait_left(Some(1000)).unwrap();
            assert!(w >= last && w <= LONGEST_WAIT);
            last = w;
        }
        assert_eq!(last, LONGEST_WAIT);
        assert_eq!(t.wait_left(Some(1000 + 59_000)), Some(Duration::from_secs(1)));
        assert_eq!(t.wait_left(Some(1000 + 60_000)), None);
        assert_eq!(t.wait_left(Some(0)), Some(LONGEST_WAIT), "a last try still to come is the whole wait");
        assert_eq!(t.wait_left(None), None, "without a clock the password is never shut out for good");
        t.wrongs = u32::MAX;
        assert_eq!(wait_after(t.wrongs), LONGEST_WAIT);
    }
}
