//! The Games page's system steps.

use std::path::Path;

use super::{and_undone, at, gone, kept, reading, replace, run, Outcome};
use crate::common::games::{self, DRIVE_DIR};
use crate::toolkit::t;

pub(super) fn split_lock(args: &[String]) -> Outcome {
    let slow = match args {
        [w] if w == "on" => true,
        [w] if w == "off" => false,
        _ => return Err("usage: trindle-admin split-lock-slowing on|off".into()),
    };
    if !at(games::SPLIT_LOCK).exists() {
        return Err(t!("admin-split-lock-none").into());
    }
    if slow {
        gone(&at(games::SPLIT_LOCK_FILE))?;
    } else {
        let file = at(games::SPLIT_LOCK_FILE);
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        replace(&file, games::SPLIT_LOCK_TEXT.as_bytes(), 0o644)?;
    }
    run(&["sysctl", "-q", "-w", &format!("kernel.split_lock_mitigate={}", u8::from(slow))], None)
}

fn mounted_at(uuid: &str) -> Vec<String> {
    reading(&["findmnt", "-n", "-o", "TARGET", "-S", &format!("UUID={uuid}")])
        .lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

fn games_mounted() -> bool {
    !reading(&["findmnt", "-n", "-o", "SOURCE", DRIVE_DIR]).trim().is_empty()
}

pub(super) fn drive(args: &[String]) -> Outcome {
    let [uuid] = args else { return Err("usage: trindle-admin games-drive UUID".into()) };
    if !games::valid_uuid(uuid) {
        return Err("usage: trindle-admin games-drive UUID".into());
    }
    let person = super::shares::caller()?;
    drive_for(uuid, person.uid, person.gid)
}

fn drive_for(uuid: &str, uid: u32, gid: u32) -> Outcome {
    let all = games::drives(&reading(&games::LSBLK));
    let found = all.iter().find(|d| d.uuid == uuid).ok_or(t!("admin-games-drive-not-found"))?;
    if let Some(why) = games::refused(&found.fs) {
        return Err(why.into());
    }
    for place in mounted_at(uuid).iter().filter(|p| *p != DRIVE_DIR) {
        run(&["umount", place], None).map_err(|e| t!("admin-games-drive-busy", why = e))?;
    }
    if games_mounted() && !found.games {
        run(&["umount", DRIVE_DIR], None).map_err(|e| t!("admin-games-drive-busy", why = e))?;
    }
    let fstab = at("/etc/fstab");
    let before = kept(&fstab)?;
    replace(&fstab, games::fstab_with(&before, Some(&games::fstab_line(uuid, &found.fs))).as_bytes(), 0o644)?;
    let mounted = (|| {
        std::fs::create_dir_all(at(DRIVE_DIR)).map_err(|e| e.to_string())?;
        run(&["systemctl", "daemon-reload"], None)?;
        if !found.games {
            run(&["mount", DRIVE_DIR], None)?;
        }
        Ok(())
    })();
    if let Err(e) = mounted {
        return Err(and_undone(e, replace(&fstab, before.as_bytes(), 0o644)));
    }
    given(&at(DRIVE_DIR), uid, gid)
}

fn given(top: &Path, uid: u32, gid: u32) -> Outcome {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::metadata(top).map_err(|e| e.to_string())?;
    if meta.uid() != 0 || games::is_a_systems_root(top) {
        return Ok(());
    }
    std::os::unix::fs::chown(top, Some(uid), Some(gid)).map_err(|e| e.to_string())
}

pub(super) fn forget(_args: &[String]) -> Outcome {
    let fstab = at("/etc/fstab");
    let before = kept(&fstab)?;
    replace(&fstab, games::fstab_with(&before, None).as_bytes(), 0o644)?;
    run(&["systemctl", "daemon-reload"], None)?;
    if games_mounted() {
        run(&["umount", DRIVE_DIR], None).map_err(|e| t!("admin-games-drive-busy", why = e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::stand_in::StandIn;
    use crate::common::games;

    #[test]
    fn split_lock_slowing_off_is_kept_for_every_start_and_on_takes_it_away() {
        let stand = StandIn::all_well("split-lock");
        stand.write(games::SPLIT_LOCK, b"1\n");
        assert_eq!(super::split_lock(&["off".into()]), Ok(()));
        assert_eq!(std::fs::read_to_string(stand.path(games::SPLIT_LOCK_FILE)).unwrap(), games::SPLIT_LOCK_TEXT);
        assert!(stand.ran().iter().any(|c| c == "sysctl -q -w kernel.split_lock_mitigate=0"));
        assert_eq!(super::split_lock(&["on".into()]), Ok(()));
        assert!(!stand.path(games::SPLIT_LOCK_FILE).exists());
        assert!(stand.ran().iter().any(|c| c == "sysctl -q -w kernel.split_lock_mitigate=1"));
        assert!(super::split_lock(&["maybe".into()]).is_err());
    }

    #[test]
    fn split_lock_is_refused_where_the_kernel_has_no_such_slowing() {
        let stand = StandIn::all_well("split-lock-none");
        assert!(super::split_lock(&["off".into()]).is_err());
        assert!(stand.ran().is_empty());
    }

    const OTHER: &str = r#"{"blockdevices":[{"uuid":"1111-root","fstype":"btrfs","label":null,"size":1,"mountpoints":["/"]},
        {"uuid":"2222-aaaa","fstype":"ext4","label":"Games","size":1,"mountpoints":["/home/john/old-games"]},
        {"uuid":"3333-bbbb","fstype":"ntfs","label":"Windows","size":1,"mountpoints":[null]}]}"#;
    const BEFORE: &str = "UUID=1111-root / btrfs defaults 0 0\n";

    #[test]
    fn a_part_of_the_computers_own_drive_a_windows_one_or_a_bad_id_is_never_taken() {
        let stand = StandIn::all_well("games-drive-refused");
        stand.write("/etc/fstab", BEFORE.as_bytes());
        stand.prints("lsblk", OTHER);
        assert!(super::drive(&["../etc".into()]).is_err());
        for uuid in ["1111-root", "3333-bbbb", "9999-none"] {
            assert!(super::drive_for(uuid, 1000, 1000).is_err(), "{uuid}");
        }
        assert_eq!(std::fs::read_to_string(stand.path("/etc/fstab")).unwrap(), BEFORE);
        assert!(stand.ran().is_empty(), "{:?}", stand.ran());
    }

    #[test]
    fn a_games_drive_is_closed_where_it_was_then_mounted_at_every_start() {
        let stand = StandIn::all_well("games-drive");
        stand.write("/etc/fstab", BEFORE.as_bytes());
        stand.prints("lsblk", OTHER);
        stand.prints("findmnt -n -o TARGET", "/home/john/old-games\n");
        assert_eq!(super::drive_for("2222-aaaa", 1000, 1000), Ok(()));
        let fstab = std::fs::read_to_string(stand.path("/etc/fstab")).unwrap();
        assert_eq!(fstab, games::fstab_with(BEFORE, Some(&games::fstab_line("2222-aaaa", "ext4"))));
        assert_eq!(stand.ran(), ["umount /home/john/old-games", "systemctl daemon-reload", "mount /mnt/games"]);
        assert_eq!(super::forget(&[]), Ok(()));
        assert_eq!(std::fs::read_to_string(stand.path("/etc/fstab")).unwrap(), BEFORE);
    }

    #[test]
    fn a_failed_mount_puts_the_fstab_back() {
        let stand = StandIn::failing("games-drive-mount", "mount", "no such device");
        stand.write("/etc/fstab", BEFORE.as_bytes());
        stand.prints("lsblk", OTHER);
        assert!(super::drive_for("2222-aaaa", 1000, 1000).is_err());
        assert_eq!(std::fs::read_to_string(stand.path("/etc/fstab")).unwrap(), BEFORE);
    }
}
