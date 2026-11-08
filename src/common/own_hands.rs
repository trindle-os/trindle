//! The one plugin whose made-up presses are the person's own hands: Flow,
//! put in from Trindle's own signed catalogue.

use std::io::Read;
use std::path::Path;

pub const FLOW: &str = "org.trindle.Flow";

pub const SCREEN_READER: &str = "org.trindle.ScreenReader";

pub const MARK: &str = "catalogue";

const MOST: u64 = 256;

pub fn is_flow(id: &str) -> bool {
    is_flow_in(&super::files::data_home().join("trindle/plugins"), id)
}

pub fn is_flow_in(folders: &Path, id: &str) -> bool {
    id == FLOW && marked(&folders.join(id).join(MARK))
}

pub fn is_screen_reader_in(folders: &Path, id: &str) -> bool {
    id == SCREEN_READER && marked(&folders.join(id).join(MARK))
}

fn marked(file: &Path) -> bool {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let opened = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(file);
    let Ok(opened) = opened else { return false };
    let Ok(meta) = opened.metadata() else { return false };
    // SAFETY: asks the kernel for this process's own user id. It reads

    let me = unsafe { libc::getuid() };
    if !meta.is_file() || meta.uid() != me || meta.mode() & 0o022 != 0 || meta.len() > MOST {
        return false;
    }
    let mut text = String::new();
    if opened.take(MOST).read_to_string(&mut text).is_err() {
        return false;
    }
    is_sha256(text.trim_end_matches('\n'))
}

fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn mark(folders: &Path, id: &str, sha256: &str) -> std::io::Result<()> {
    if !is_sha256(sha256) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a SHA-256"));
    }
    let file = folders.join(id).join(MARK);
    if std::fs::symlink_metadata(&file).is_ok_and(|m| !m.is_file()) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a file"));
    }
    super::files::write_whole_private(&file, format!("{sha256}\n").as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUM: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn folders(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-own-hands-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(FLOW)).unwrap();
        dir
    }

    #[test]
    fn only_flow_with_the_catalogues_mark_is_flow() {
        use std::os::unix::fs::PermissionsExt;
        let all = folders("is");
        assert!(!is_flow_in(&all, FLOW), "Flow's id with no mark was taken for Flow");
        mark(&all, FLOW, SUM).unwrap();
        assert!(is_flow_in(&all, FLOW));
        let file = all.join(FLOW).join(MARK);
        assert_eq!(std::fs::metadata(&file).unwrap().permissions().mode() & 0o777, 0o600);

        for other in ["org.example.Flow".to_string(), FLOW.to_lowercase(), format!("{FLOW}2"), "Flow".into(), format!("../{FLOW}")] {
            let _ = std::fs::create_dir_all(all.join(&other));
            let _ = std::fs::copy(&file, all.join(&other).join(MARK));
            assert!(!is_flow_in(&all, &other), "{other} was taken for Flow");
        }

        for bad in ["", "yes\n", &SUM.to_uppercase(), &format!("{SUM}\n{SUM}\n"), &"a".repeat(300)] {
            std::fs::write(&file, bad).unwrap();
            assert!(!is_flow_in(&all, FLOW), "a mark of {bad:?} was taken for the catalogue's");
        }
        mark(&all, FLOW, SUM).unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o666)).unwrap();
        assert!(!is_flow_in(&all, FLOW), "a mark anyone may write was taken for the catalogue's");
        std::fs::remove_file(&file).unwrap();
        let real = all.join("elsewhere");
        std::fs::write(&real, format!("{SUM}\n")).unwrap();
        std::os::unix::fs::symlink(&real, &file).unwrap();
        assert!(!is_flow_in(&all, FLOW), "a link was taken for the catalogue's mark");
        assert!(mark(&all, FLOW, SUM).is_err(), "the mark was written through a link");
        assert!(mark(&all, FLOW, "not a sum").is_err());
        let _ = std::fs::remove_dir_all(&all);
    }

    #[test]
    fn only_the_screen_reader_with_the_catalogues_mark_is_the_screen_reader() {
        let all = folders("reader");
        std::fs::create_dir_all(all.join(SCREEN_READER)).unwrap();
        assert!(!is_screen_reader_in(&all, SCREEN_READER), "no mark");
        mark(&all, SCREEN_READER, SUM).unwrap();
        assert!(is_screen_reader_in(&all, SCREEN_READER));
        assert!(!is_flow_in(&all, SCREEN_READER) && !is_screen_reader_in(&all, FLOW), "one is not the other");
        mark(&all, FLOW, SUM).unwrap();
        for other in ["org.example.ScreenReader", "org.trindle.screenreader", &format!("{SCREEN_READER}2")] {
            std::fs::create_dir_all(all.join(other)).unwrap();
            std::fs::copy(all.join(SCREEN_READER).join(MARK), all.join(other).join(MARK)).unwrap();
            assert!(!is_screen_reader_in(&all, other), "{other}");
        }
        let _ = std::fs::remove_dir_all(&all);
    }
}
