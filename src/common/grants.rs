//! The person's say-so for a plugin to take this computer's mouse and
//! keyboard. A plugin that was allowed
//! "capture" may ask the compositor to take them at once only right after
//! the person did something that runs that plugin: pressed its shortcut,
//! chosen one of its ring items or pressed its ring icon. Whatever starts
//! the plugin (`trindle-plugin run`, outside every box) leaves a mark, once;
//! the compositor, when the plugin asks, takes the mark if it is fresh, and
//! a plugin that asks with none is refused.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub const VALID: Duration = Duration::from_secs(5);

fn folder() -> PathBuf {
    super::files::scratch().join("trindle-take")
}

fn good(id: &str) -> bool {
    !id.is_empty() && id.len() <= 100 && !id.starts_with(['.', '-']) && !id.contains("..")
        && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

pub fn give(id: &str) -> std::io::Result<()> {
    give_in(&folder(), id)
}

pub fn take(id: &str) -> bool {
    take_in(&folder(), id, SystemTime::now())
}

pub fn give_in(folder: &Path, id: &str) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
    if !good(id) {
        return Err(std::io::Error::other("not a plugin's id"));
    }
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(folder)?;
    let path = folder.join(id);

    let _ = std::fs::remove_file(&path);
    std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC).open(&path).map(drop)
}
pub fn take_in(folder: &Path, id: &str, now: SystemTime) -> bool {
    if !good(id) {
        return false;
    }
    let path = folder.join(id);
    let made = match std::fs::symlink_metadata(&path) {
        Ok(m) if m.is_file() => m.modified().ok(),
        _ => return false,
    };

    if std::fs::remove_file(&path).is_err() {
        return false;
    }
    made.and_then(|t| now.duration_since(t).ok()).is_some_and(|age| age <= VALID)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-grants-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn a_mark_is_good_once_and_for_five_seconds() {
        let dir = temp("once");
        give_in(&dir, "org.example.One").unwrap();
        assert!(take_in(&dir, "org.example.One", SystemTime::now()));
        assert!(!take_in(&dir, "org.example.One", SystemTime::now()), "used up");

        give_in(&dir, "org.example.One").unwrap();
        assert!(!take_in(&dir, "org.example.One", SystemTime::now() + VALID + Duration::from_secs(1)));
        assert!(!take_in(&dir, "org.example.One", SystemTime::now()));

        give_in(&dir, "org.example.One").unwrap();
        assert!(take_in(&dir, "org.example.One", SystemTime::now() + VALID - Duration::from_millis(100)));

        give_in(&dir, "org.example.One").unwrap();
        assert!(!take_in(&dir, "org.example.One", SystemTime::now() - Duration::from_secs(60)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_mark_is_for_the_plugin_that_was_run_and_no_other() {
        let dir = temp("who");
        give_in(&dir, "org.a.Alpha").unwrap();
        assert!(!take_in(&dir, "org.b.Beta", SystemTime::now()));
        assert!(!take_in(&dir, "", SystemTime::now()));
        assert!(take_in(&dir, "org.a.Alpha", SystemTime::now()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn names_that_are_not_a_plugins_id_never_reach_a_path() {
        let dir = temp("names");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("secret"), "").unwrap();
        for bad in ["../secret", "a/b", "..", ".", "-x", ".hidden", "a b", "", "a\0b", &"x".repeat(101)] {
            assert!(give_in(&dir, bad).is_err(), "{bad:?}");
            assert!(!take_in(&dir, bad, SystemTime::now()), "{bad:?}");
        }
        assert!(dir.join("secret").exists(), "nothing outside was touched");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_planted_link_or_folder_is_not_a_mark() {
        let dir = temp("planted");
        std::fs::create_dir_all(&dir).unwrap();
        let real = dir.join("real");
        std::fs::write(&real, "").unwrap();
        std::os::unix::fs::symlink(&real, dir.join("org.x.Linked")).unwrap();
        assert!(!take_in(&dir, "org.x.Linked", SystemTime::now()), "a link is not a mark");
        assert!(real.exists(), "and what it led to is left alone");
        std::fs::create_dir(dir.join("org.x.Folder")).unwrap();
        assert!(!take_in(&dir, "org.x.Folder", SystemTime::now()));

        let _ = std::fs::remove_file(dir.join("org.x.Linked"));
        std::os::unix::fs::symlink(&real, dir.join("org.x.Linked")).unwrap();
        give_in(&dir, "org.x.Linked").unwrap();
        assert!(std::fs::symlink_metadata(dir.join("org.x.Linked")).unwrap().is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_folder_is_for_this_account_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = temp("mode");
        give_in(&dir, "org.a.Alpha").unwrap();
        assert_eq!(std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777, 0o700);
        assert_eq!(std::fs::metadata(dir.join("org.a.Alpha")).unwrap().permissions().mode() & 0o077, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
