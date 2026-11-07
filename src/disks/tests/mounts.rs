//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn every_part_shows_mounted_or_not() {
    let drives = made_up_drives("nvme1n1");
    assert_eq!(drives.len(), 4);
    let second = named(&drives, SECOND);
    assert!(!second.system && !second.removable);
    let words: Vec<(String, String, u64)> = second.vols.iter().map(|v| (v.name(), v.kind_words(), v.size)).collect();
    assert_eq!(words, vec![
        ("nvme1n1p1".to_string(), "FAT, EFI, the part a computer starts from".to_string(), 600_000_000),
        ("other_boot".to_string(), "ext4, a boot loader's own part".to_string(), 2 * G),
        ("other_system".to_string(), "btrfs".to_string(), THIRD),
    ], "all three parts, though UDisks marks the first two to be ignored");
    assert!(second.vols.iter().all(|v| v.unmounted() && !v.system));
    let stick = named(&drives, "Example Stick");
    assert_eq!(stick.vols.iter().map(Vol::kind_words).collect::<Vec<_>>(), vec!["Encrypted, locked", "No filesystem"]);
    assert_eq!(stick.gaps.len(), 1, "the room after its parts is free space: {:?}", stick.gaps);
    let blank = named(&drives, "Example Blank");
    assert!(blank.vols.is_empty() && blank.gaps.is_empty());

    let d = made_up_window("parts");
    let said = d.view().said();
    for want in [
        "FAT, EFI, the part a computer starts from, 600.0 MB, not mounted",
        "ext4, a boot loader's own part, 2.0 GB, not mounted",
        "btrfs, 2.0 TB, not mounted",
        "other_boot", "other_system", "nvme1n1p1",
        "Encrypted, locked, 32.0 GB",
        "No filesystem, 8.0 GB",
        "Free space",
        "No partition table: the whole drive is free.",
    ] {
        assert!(said.iter().any(|s| s == want), "the window says {want:?}: {said:?}");
    }
    assert!(said.iter().any(|s| s.starts_with("ext4, 499.0 GB, 399.0 GB free, in /")), "{said:?}");
    let rows = d.map_rows();
    let row = rows.iter().find(|r| r.name == SECOND).unwrap();
    assert_eq!(row.parts.iter().map(|p| (p.name.as_str(), p.words.as_str())).collect::<Vec<_>>(),
               vec![("nvme1n1p1", "Not mounted"), ("other_boot", "Not mounted"), ("other_system", "Not mounted")]);
    let stick = rows.iter().find(|r| r.name == "Example Stick").unwrap();
    assert_eq!(stick.parts.iter().map(|p| p.words.as_str()).collect::<Vec<_>>()[..2], ["Encrypted, locked", "No filesystem"]);
    assert!(stick.parts.last().is_some_and(|p| p.gap && p.name == "Free space"));
    let blank = rows.iter().find(|r| r.name == "Example Blank").unwrap();
    assert_eq!(blank.parts.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), vec!["Nothing on it yet"]);
    let mut open = made_up("nvme1n1");
    let mut copy = Fake::block("dm-7", "0003", "", "filesystem", "ext4", "uuid-inside", 32 * G, false);
    copy.insert("Drive".into(), path_val("/"));
    copy.insert("CryptoBackingDevice".into(), path_val(&format!("{B}sdx1")));
    open.insert(format!("{B}dm_2d7"), HashMap::from([(BLOCK.to_string(), copy)]));
    let drives = build(&open, &HashSet::new(), &HashSet::new(), |_| None);
    assert_eq!(named(&drives, "Example Stick").vols[0].kind_words(), "Encrypted, unlocked");
}

#[test]
fn a_hidden_drive_stays_greyed_and_comes_back() {
    let mut d = made_up_window("hide");
    let before = picture(&d);
    let said = d.view().said();
    let hide = format!("button: Hide this drive. {}", hidden_drives::NOT_A_LOCK);
    assert_eq!(said.iter().filter(|s| **s == hide).count(), 3, "every drive but the system's has an eye: {said:?}");
    assert!(!said.iter().any(|s| s == "button: Show this drive"));
    let second = named(&d.drives, SECOND);
    let (path, key) = (second.path.clone(), second.key.clone());
    assert_eq!(key, "drive:Example-NVMe-2TB-0002", "known by what it says of itself");
    assert!(!key.contains("nvme1n1"));

    d.update(Msg::Hide { drive: path.clone(), row: String::new(), key: key.clone(), hide: true });
    assert_eq!(std::fs::read_to_string(&d.hidden_file).unwrap(), format!("{key}\n"), "kept at once, in the person's own file");
    let said = d.view().said();
    for want in ["other_system", "other_boot", "button: Show this drive", "button: Mount", "button: Erase the whole drive"] {
        assert!(said.iter().any(|s| s == want), "{want:?} in {said:?}");
    }
    let line = format!("Hidden from Folders and the ring. {}", hidden_drives::NOT_A_LOCK);
    assert_eq!(said.iter().filter(|s| **s == line).count(), 1);
    assert_eq!(said.iter().filter(|s| **s == hide).count(), 2);
    let hidden = picture(&d);
    assert_ne!(hidden, before, "it is drawn quieter");
    let row = d.map_rows().into_iter().find(|r| r.name == SECOND).unwrap();
    assert!(row.quiet && row.parts.iter().all(|p| p.quiet) && row.words.ends_with(", hidden"));
    assert!(d.map_rows().iter().filter(|r| r.name != SECOND).all(|r| !r.quiet && r.parts.iter().all(|p| !p.quiet)));

    d.drives = made_up_drives("nvme0n1");
    d.hidden = hidden_drives::read_in(&d.hidden_file);
    assert!(named(&d.drives, SECOND).vols.iter().all(|v| v.dev.starts_with("nvme0n1")));
    assert!(d.drive_hidden(named(&d.drives, SECOND)));
    d.drives = made_up_drives("nvme1n1");

    d.update(Msg::Hide { drive: path.clone(), row: String::new(), key, hide: false });
    assert!(d.hidden.is_empty() && !d.drive_hidden(named(&d.drives, SECOND)));
    assert_eq!(picture(&d), before);

    let part = named(&d.drives, SECOND).vols[2].clone();
    assert_eq!(part.key, "fs:uuid-of-other_system");
    d.update(Msg::Hide { drive: path.clone(), row: part.path.clone(), key: part.key.clone(), hide: true });
    let row = d.map_rows().into_iter().find(|r| r.name == SECOND).unwrap();
    assert!(!row.quiet && row.parts[2].quiet && !row.parts[0].quiet);
    assert!(d.view().said().iter().any(|s| s == "button: Show this part"));
    d.update(Msg::Hide { drive: path.clone(), row: part.path.clone(), key: part.key, hide: false });
    assert_eq!(picture(&d), before);

    let system = d.drives.iter().find(|x| x.system).unwrap();
    let system_key = system.key.clone();
    d.hidden.insert(system_key);
    let system = d.drives.iter().find(|x| x.system).unwrap();
    assert!(!d.drive_hidden(system) && !d.part_hidden(system, &system.vols[1]));
    d.hidden.clear();

    std::fs::remove_dir_all(d.hidden_file.parent().unwrap()).unwrap();
    std::fs::write(d.hidden_file.parent().unwrap(), b"in the way").unwrap();
    d.update(Msg::Hide { drive: path.clone(), row: String::new(), key: "drive:Example".into(), hide: true });
    assert!(d.told(&path, "").is_some_and(|(w, t)| w.starts_with("That could not be kept: ") && t == Tone::Problem));
    assert!(d.hidden.is_empty());
    std::fs::remove_file(d.hidden_file.parent().unwrap()).unwrap();
}

#[test]
fn a_hidden_drive_still_reads_to_wcag_aa() {
    let least = theme::HIDDEN_LEAST;
    assert!(theme::sheet().hidden_drive >= least && theme::sheet().hidden_drive < 1.0);
    for dark in [true, false] {
        let t = Theme::new(dark, "#fbc711");
        for (what, under) in [("a card", t.raised), ("a band", t.raised_more), ("the page", t.bg)] {
            let shown = theme::over(theme::alpha(t.fg, least), under);
            let c = theme::contrast(shown, under);
            assert!(c >= theme::WORDS, "a name on {what}, dark {dark}: {c:.2} to 1");
        }
    }
}
