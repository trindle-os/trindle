//! Why the box is up on a disk the TPM chip is meant to open: read from the
//! disk's own LUKS2 header and the kernel's list of chips, never guessed.

use trindle_shared::luks_chip::{chip_seen, has_chip_key, read_header, valid_uuid, REFUSED};
use trindle_toolkit::t;

pub fn note(device: Option<&str>) -> Option<String> {
    let header = read_header(std::fs::File::open(device?).ok()?)?;
    let seen = chip_seen();
    let line = why(has_chip_key(&header.json), seen)?;
    if seen && valid_uuid(&header.uuid) {
        leave_note(std::path::Path::new(REFUSED), &header.uuid);
    }
    Some(line)
}

pub fn why(chip_key: bool, chip_seen: bool) -> Option<String> {
    let (why, fix) = match (chip_key, chip_seen) {
        (false, _) => return None,
        (true, false) => (t!("unlock-chip-missing"), t!("unlock-chip-missing-fix")),
        (true, true) => (t!("unlock-chip-refused"), t!("unlock-chip-refused-fix")),
    };
    Some(format!("{why}\n{fix}"))
}

fn leave_note(path: &std::path::Path, uuid: &str) {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let _ = std::fs::remove_file(path);
    if let Ok(mut f) = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o644)
        .custom_flags(libc::O_NOFOLLOW).open(path)
    {
        let _ = f.write_all(format!("{uuid}\n").as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_line_says_why_only_where_the_chip_should_have_opened_it() {
        assert_eq!(why(false, true), None);
        assert_eq!(why(false, false), None);
        let refused = why(true, true).unwrap();
        assert!(refused.starts_with("The security chip didn't unlock the disk"), "{refused}");
        assert!(refused.lines().nth(1).is_some_and(|fix| fix.contains("Settings") && fix.contains("Use the chip again")), "{refused}");
        let missing = why(true, false).unwrap();
        assert!(missing.starts_with("The security chip wasn't found"), "{missing}");
        assert!(missing.lines().nth(1).is_some_and(|fix| fix.contains("BIOS")), "{missing}");
        assert_eq!(note(None), None, "no disk named");
        assert_eq!(note(Some("/nonexistent/disk")), None);
    }

    #[test]
    fn the_note_replaces_a_link_rather_than_writing_through_it() {
        let dir = std::env::temp_dir().join(format!("trindle-unlock-note-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let victim = dir.join("victim");
        std::fs::write(&victim, "keep").unwrap();
        let note = dir.join("note");
        std::os::unix::fs::symlink(&victim, &note).unwrap();
        let uuid = "0b5f6a7e-1c2d-4e3f-8a9b-0c1d2e3f4a5b";
        leave_note(&note, uuid);
        assert_eq!(std::fs::read_to_string(&victim).unwrap(), "keep");
        assert_eq!(std::fs::read_to_string(&note).unwrap(), format!("{uuid}\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
