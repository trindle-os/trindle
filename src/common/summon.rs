//! Whether the person has allowed a plugin "summon": its made-up keys and
//! clicks may bring the ring out over a full-screen window, as their own
//! do. The compositor asks here each time
//! such a key or click runs the ring's command, so taking the permission
//! away takes effect at the next key, with nothing to restart.

use std::path::Path;

pub const WORD: &str = "summon";

const CONTROL: &str = "control";

const RECORD: &str = "allowed";

const MOST: u64 = 4096;

pub fn good(id: &str) -> bool {
    !id.is_empty() && id.len() <= 100 && !id.starts_with(['.', '-']) && !id.contains("..")
        && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

pub fn allowed(id: &str) -> bool {
    allowed_in(&super::files::data_home().join("trindle/plugins"), id)
}

pub fn allowed_in(folders: &Path, id: &str) -> bool {
    record_says(folders, id, &[WORD, CONTROL])
}

pub fn record_says(folders: &Path, id: &str, words: &[&str]) -> bool {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    if !good(id) {
        return false;
    }
    let opened = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(folders.join(id).join(RECORD));
    let Ok(file) = opened else { return false };
    if !file.metadata().is_ok_and(|m| m.is_file() && m.len() <= MOST) {
        return false;
    }
    let mut text = String::new();
    if file.take(MOST).read_to_string(&mut text).is_err() {
        return false;
    }
    words.iter().all(|word| text.lines().any(|l| l.trim() == *word))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_record_that_says_summon_and_control_allows_it() {
        let dir = std::env::temp_dir().join(format!("trindle-summon-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let record = |id: &str, text: &str| {
            std::fs::create_dir_all(dir.join(id)).unwrap();
            std::fs::write(dir.join(id).join(RECORD), text).unwrap();
        };
        record("org.example.steer", "network\ncontrol\nsummon\n");
        assert!(allowed_in(&dir, "org.example.steer"));

        assert!(!allowed_in(&dir, "org.example.none"));
        record("org.example.empty", "");
        record("org.example.control", "control\n");
        assert!(!allowed_in(&dir, "org.example.empty") && !allowed_in(&dir, "org.example.control"));

        record("org.example.alone", "summon\n");
        assert!(!allowed_in(&dir, "org.example.alone"));

        record("org.example.near", "control\nsummons\nnot summon\n");
        assert!(!allowed_in(&dir, "org.example.near"));

        record("org.example.steer", "network\ncontrol\n");
        assert!(!allowed_in(&dir, "org.example.steer"));

        record("org.example.steer", "control\nsummon\n");
        for odd in ["", ".", "..", "../org.example.steer", "org.example.steer/", "-x", "a/b"] {
            assert!(!allowed_in(&dir, odd), "{odd:?}");
        }
        std::fs::create_dir_all(dir.join("org.example.link")).unwrap();
        std::os::unix::fs::symlink(dir.join("org.example.steer").join(RECORD), dir.join("org.example.link").join(RECORD)).unwrap();
        assert!(!allowed_in(&dir, "org.example.link"));
        std::fs::create_dir_all(dir.join("org.example.folder").join(RECORD)).unwrap();
        assert!(!allowed_in(&dir, "org.example.folder"));
        record("org.example.long", &format!("control\nsummon\n{}", "x\n".repeat(4096)));
        assert!(!allowed_in(&dir, "org.example.long"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
