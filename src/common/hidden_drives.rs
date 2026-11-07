//! The drives and parts a person has hidden.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::files::{config_path, read_small, write_text};
use crate::toolkit::bus::{self as dbus, Props};

pub const FILE: &str = "hidden-drives.conf";

pub const NOT_A_LOCK: &str = "Hiding does not lock or protect it. A twin of it, one that says exactly the same of itself, is hidden with it.";

pub fn file() -> PathBuf {
    config_path(FILE)
}

pub fn try_read_in(file: &Path) -> std::io::Result<BTreeSet<String>> {
    Ok(read_small(file)?.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from).collect())
}

pub fn read_in(file: &Path) -> BTreeSet<String> {
    try_read_in(file).unwrap_or_default()
}

pub fn set_in(file: &Path, key: &str, hidden: bool) -> std::io::Result<()> {
    if key.trim().is_empty() || !super::files::fits_a_line(key) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "this drive has no name of its own to remember it by"));
    }
    let key = key.trim();
    let was = read_small(file)
        .map_err(|e| std::io::Error::new(e.kind(), format!("the list of hidden drives was left as it is, because {e}")))?;
    let there = was.lines().any(|l| l.trim() == key);
    if there == hidden {
        return Ok(());
    }
    let mut text: String = was.lines().filter(|l| hidden || l.trim() != key).flat_map(|l| [l, "\n"]).collect();
    if hidden {
        text.push_str(key);
        text.push('\n');
    }
    write_text(file, &text)
}

pub fn drive_key(d: &Props) -> String {
    ["Id", "Serial", "WWN"].iter()
        .map(|k| clean(&dbus::get_str(d, k)))
        .find(|v| !v.is_empty())
        .map(|v| format!("drive:{v}"))
        .unwrap_or_default()
}

pub fn part_key(block: &Props, partition: Option<&Props>) -> String {
    let own = clean(&dbus::get_str(block, "IdUUID"));
    if !own.is_empty() {
        return format!("fs:{own}");
    }
    let table = partition.map(|p| clean(&dbus::get_str(p, "UUID"))).unwrap_or_default();
    if table.is_empty() { String::new() } else { format!("part:{table}") }
}

fn clean(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect::<String>().trim().to_string()
}

pub fn hides(hidden: &BTreeSet<String>, drive_key: &str, part_key: &str) -> bool {
    (!drive_key.is_empty() && hidden.contains(drive_key)) || (!part_key.is_empty() && hidden.contains(part_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::{OwnedValue, Value};

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-hidden-drives-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn props(items: &[(&str, &str)]) -> Props {
        items.iter().map(|(k, v)| (k.to_string(), OwnedValue::try_from(Value::from(*v)).unwrap())).collect()
    }

    #[test]
    fn with_no_file_nothing_is_hidden_and_hiding_is_kept_and_undone() {
        let dir = scratch("kept");
        let file = dir.join("deeper").join(FILE);
        assert!(read_in(&file).is_empty());
        set_in(&file, "drive:Example-123", false).unwrap();
        assert!(!file.exists());
        set_in(&file, "drive:Example-123", true).unwrap();
        set_in(&file, "fs:1234-ABCD", true).unwrap();
        set_in(&file, "fs:1234-ABCD", true).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "drive:Example-123\nfs:1234-ABCD\n");
        set_in(&file, "drive:Example-123", false).unwrap();
        assert_eq!(read_in(&file), BTreeSet::from(["fs:1234-ABCD".to_string()]));
        assert!(set_in(&file, "", true).is_err());
        assert!(set_in(&file, "fs:a\nfs:b", true).is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "fs:1234-ABCD\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_note_left_in_the_list_is_kept_through_a_change() {
        let file = scratch("note").join("hidden-drives");
        std::fs::write(&file, "# The spare stick, hidden on purpose.\nstick-one\n\n# and the old disk\ndisk-two\n").unwrap();
        set_in(&file, "disk-three", true).unwrap();
        set_in(&file, "stick-one", false).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "# The spare stick, hidden on purpose.\n\n# and the old disk\ndisk-two\ndisk-three\n");
        assert_eq!(read_in(&file), BTreeSet::from(["disk-two".to_string(), "disk-three".to_string()]));
        let before = std::fs::metadata(&file).unwrap().modified().unwrap();
        set_in(&file, "disk-two", true).unwrap();
        set_in(&file, "never-there", false).unwrap();
        assert_eq!(std::fs::metadata(&file).unwrap().modified().unwrap(), before);
        let _ = std::fs::remove_dir_all(file.parent().unwrap());
    }

    #[test]
    fn a_list_that_cannot_be_read_is_never_written_over() {
        let dir = scratch("honest");
        let file = dir.join(FILE);
        let was = b"drive:Example-123\nfs:1234-ABCD\n\xff\n".to_vec();
        std::fs::write(&file, &was).unwrap();
        assert!(try_read_in(&file).is_err());
        let e = set_in(&file, "drive:Other-456", true).unwrap_err();
        assert_eq!(e.to_string(), "the list of hidden drives was left as it is, because the file that keeps it is not text and cannot be read");
        assert!(set_in(&file, "drive:Example-123", false).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), was, "the drives hidden before are all still there");
        assert!(read_in(&file).is_empty());
        assert_eq!(try_read_in(&dir.join("none.conf")).unwrap(), BTreeSet::new());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn twins_that_cannot_be_told_apart_are_hidden_together_and_the_eye_says_so() {
        let one = props(&[("Id", "Example-Flash-Disk"), ("Serial", ""), ("Model", "Flash Disk")]);
        let other = props(&[("Id", "Example-Flash-Disk"), ("Serial", ""), ("Model", "Flash Disk")]);
        assert_eq!(drive_key(&one), drive_key(&other));
        let hidden = BTreeSet::from([drive_key(&one)]);
        assert!(hides(&hidden, &drive_key(&other), ""));
        let part = props(&[("IdUUID", "1234-ABCD")]);
        assert!(hides(&BTreeSet::from([part_key(&part, None)]), "drive:Another-0002", &part_key(&part, None)));
        assert_ne!(drive_key(&props(&[("Id", "Example-SSD-0001")])), drive_key(&props(&[("Id", "Example-SSD-0002")])));
        assert!(NOT_A_LOCK.starts_with("Hiding does not lock or protect it.") && NOT_A_LOCK.contains("twin") && NOT_A_LOCK.contains("is hidden with it"));
    }

    #[test]
    fn a_drive_and_a_part_are_known_by_what_they_say_of_themselves() {
        assert_eq!(drive_key(&props(&[("Id", "Example-SSD-0001"), ("Serial", "0001")])), "drive:Example-SSD-0001");
        assert_eq!(drive_key(&props(&[("Id", ""), ("Serial", "0001")])), "drive:0001");
        assert_eq!(drive_key(&props(&[("WWN", "0x5000000000000001")])), "drive:0x5000000000000001");
        assert_eq!(drive_key(&props(&[("Model", "Example")])), "", "a model alone is not one drive");
        let table = props(&[("UUID", "0a0a0a0a-01")]);
        assert_eq!(part_key(&props(&[("IdUUID", "1234-ABCD"), ("Device", "/dev/sdb1")]), Some(&table)), "fs:1234-ABCD");
        assert_eq!(part_key(&props(&[("IdUUID", "")]), Some(&table)), "part:0a0a0a0a-01");
        assert_eq!(part_key(&props(&[("Device", "/dev/sdb1")]), None), "", "never the device name");
        let hidden = BTreeSet::from(["drive:Example-SSD-0001".to_string(), "fs:1234-ABCD".to_string()]);
        assert!(hides(&hidden, "drive:Example-SSD-0001", "fs:other"));
        assert!(hides(&hidden, "drive:other", "fs:1234-ABCD"));
        assert!(!hides(&hidden, "drive:other", "fs:other"));
        assert!(!hides(&BTreeSet::from([String::new()]), "", ""));
    }
}
