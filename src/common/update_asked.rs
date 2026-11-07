//! The ring's update button asks Apps to put updates in.

use std::io::Write;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

const NAME: &str = "trindle-update-asked";
const FRESH: Duration = Duration::from_secs(20);

#[cfg(not(test))]
fn place() -> PathBuf {
    super::files::scratch().join(NAME)
}

#[cfg(test)]
fn place() -> PathBuf {
    let thread: String = format!("{:?}", std::thread::current().id()).chars().filter(char::is_ascii_digit).collect();
    std::env::temp_dir().join(format!("{NAME}-test-{}-{thread}", std::process::id()))
}

pub fn leave() -> bool {
    leave_at(&place(), SystemTime::now())
}

pub fn take() -> bool {
    take_at(&place(), SystemTime::now())
}

fn leave_at(path: &Path, now: SystemTime) -> bool {
    let _ = std::fs::remove_file(path);
    let stamp = now.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(path)
        .and_then(|mut f| f.write_all(stamp.to_string().as_bytes()))
        .is_ok()
}

fn take_at(path: &Path, now: SystemTime) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(path) else { return false };
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let _ = std::fs::remove_file(path);
    // SAFETY: geteuid takes nothing and cannot fail.
    let mine = meta.uid() == unsafe { libc::geteuid() };
    let stamp = text.trim().parse::<u64>().ok().map(|s| SystemTime::UNIX_EPOCH + Duration::from_secs(s));
    let fresh = stamp.is_some_and(|t| match now.duration_since(t) {
        Ok(age) => age <= FRESH,
        Err(early) => early.duration() <= FRESH,
    });
    meta.file_type().is_file() && meta.mode() & 0o077 == 0 && mine && fresh
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("trindle-asked-{name}-{}", std::process::id()))
    }

    fn secs(t: SystemTime) -> String {
        t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
    }

    #[test]
    fn a_note_works_once_and_only_when_fresh() {
        let p = path("once");
        let now = SystemTime::now();
        assert!(!take_at(&p, now), "no note, no start");
        assert!(leave_at(&p, now));
        assert!(take_at(&p, now));
        assert!(!take_at(&p, now), "used up");
        assert!(leave_at(&p, now));
        assert!(!take_at(&p, now + FRESH + Duration::from_secs(5)), "too old");
        assert!(!p.exists(), "and an old one is removed too");
    }

    #[test]
    fn a_note_that_is_not_the_rings_is_refused() {
        let p = path("forged");
        let now = SystemTime::now();
        std::fs::write(&p, "not a time").unwrap();
        assert!(!take_at(&p, now));
        let link = path("link");
        let target = path("target");
        std::fs::write(&target, secs(now)).unwrap();
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&target, &link).unwrap();
        assert!(!take_at(&link, now), "a link is not a note");
        let _ = std::fs::remove_file(&target);
        std::fs::write(&p, secs(now)).unwrap();
        assert!(!take_at(&p, now), "a note anyone could read is refused");
        let _ = std::fs::remove_file(&p);
    }
}
