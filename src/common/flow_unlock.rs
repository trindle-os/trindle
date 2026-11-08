//! Flow unlocking the lock screen with the PIN, and with nothing else
//! Flow alone may do this, and never with the password.

use std::path::{Path, PathBuf};
use std::time::Duration;

pub const TRIES: u32 = super::pin::TRIES;

const KEPT: usize = 200;

pub const NOT_LOCKED: &str = "Not locked.";
pub const NO_PIN: &str = "No PIN set.";
pub const LATE: &str = "PIN time is over.";
pub const TOO_MANY: &str = "Too many tries.";
pub const NOT_A_PIN: &str = "A PIN is 4 digits.";

pub fn wrong(left: u32) -> String {
    format!("Wrong PIN. {left} tries left.")
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Unlock,

    No(String, bool),
}

pub fn judge(locked: bool, set: bool, since: Duration, window: u32, used: u32, pin: &str, right: impl FnOnce(&str) -> bool) -> Verdict {
    if !locked {
        return Verdict::No(NOT_LOCKED.into(), false);
    }
    if !set {
        return Verdict::No(NO_PIN.into(), false);
    }
    if since.as_secs() >= u64::from(window) * 60 {
        return Verdict::No(LATE.into(), false);
    }
    if used >= TRIES {
        return Verdict::No(TOO_MANY.into(), false);
    }
    if !super::pin::valid(pin) {
        return Verdict::No(NOT_A_PIN.into(), false);
    }
    if right(pin) {
        Verdict::Unlock
    } else {
        Verdict::No(wrong(TRIES - used - 1), true)
    }
}

pub fn notes() -> PathBuf {
    super::files::home().join(".local/state/trindle/unlock-notes.log")
}

pub fn note(file: &Path, unix: i64) -> std::io::Result<()> {
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let old = super::files::read_small(file).unwrap_or_default();
    let mut lines: Vec<&str> = old.lines().collect();
    let line = format!("{} Flow unlocked with the PIN", super::clock::utc_iso(unix));
    lines.push(&line);
    let from = lines.len().saturating_sub(KEPT);
    let mut text = lines[from..].join("\n");
    text.push('\n');
    super::files::write_whole_private(file, text.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOON: Duration = Duration::from_secs(60);

    #[test]
    fn the_words_of_a_refused_unlock_are_short() {
        for words in [NOT_LOCKED, NO_PIN, LATE, TOO_MANY, NOT_A_PIN] {
            assert!(words.split_whitespace().count() <= 5 && !words.contains('\u{2014}'), "{words}");
        }
        assert_eq!(wrong(2), "Wrong PIN. 2 tries left.");
    }

    #[test]
    fn only_a_right_pin_in_time_with_tries_left_unlocks() {
        let go = |locked, set, since, used, pin: &str, ok: bool| judge(locked, set, since, 60, used, pin, move |_| ok);
        assert_eq!(go(true, true, SOON, 0, "1234", true), Verdict::Unlock);
        assert_eq!(go(true, true, SOON, 0, "1234", false), Verdict::No(wrong(4), true));
        assert_eq!(go(true, true, SOON, 4, "1234", false), Verdict::No(wrong(0), true));
        assert_eq!(go(true, true, SOON, 5, "1234", true), Verdict::No(TOO_MANY.into(), false));
        assert_eq!(go(false, true, SOON, 0, "1234", true), Verdict::No(NOT_LOCKED.into(), false));
        assert_eq!(go(true, false, SOON, 0, "1234", true), Verdict::No(NO_PIN.into(), false));
        assert_eq!(go(true, true, Duration::from_secs(3600), 0, "1234", true), Verdict::No(LATE.into(), false));

        let never = judge(true, true, SOON, 60, 0, "correct horse", |_| panic!("asked about a password"));
        assert_eq!(never, Verdict::No(NOT_A_PIN.into(), false));
    }

    #[test]
    fn each_unlock_is_noted_and_the_note_stays_short() {
        let dir = std::env::temp_dir().join(format!("trindle-flow-unlock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let file = dir.join("unlock-notes.log");
        for n in 0..KEPT as i64 + 5 {
            note(&file, 1_800_000_000 + n).unwrap();
        }
        let text = std::fs::read_to_string(&file).unwrap();
        assert_eq!(text.lines().count(), KEPT);
        assert!(text.lines().all(|l| l.ends_with("Z Flow unlocked with the PIN")));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
