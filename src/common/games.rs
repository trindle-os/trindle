//! The Games page's choices that are not Steam's own: a larger sound buffer
//! for games that crackle, the kernel's slowing of split-lock programs, and
//! a second drive for games. Settings reads them here and the
//! administrator's helper writes the system ones, so both name the same
//! files.

use std::path::PathBuf;

use serde_json::Value;

use crate::toolkit::t;

const SOUND_FILE: &str = "pipewire/pipewire.conf.d/90-trindle-games.conf";

pub const QUANTUM: u32 = 1024;

pub fn sound_file() -> PathBuf {
    crate::common::files::config_home().join(SOUND_FILE)
}

pub fn sound_fix_on() -> bool {
    sound_file().is_file()
}

pub fn sound_text() -> String {
    format!("# Settings, Games: a larger sound buffer for games that crackle.\ncontext.properties = {{\n    default.clock.min-quantum = {QUANTUM}\n}}\n")
}

pub fn sound_now(on: bool) -> Vec<String> {
    let q = if on { QUANTUM } else { 0 };
    ["pw-metadata", "-n", "settings", "0", "clock.min-quantum", &q.to_string()].map(String::from).to_vec()
}

pub fn set_sound_fix(on: bool) -> Result<(), String> {
    let path = sound_file();
    if on {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        crate::common::files::write_whole(&path, sound_text().as_bytes()).map_err(|e| e.to_string())
    } else {
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub const SPLIT_LOCK: &str = "/proc/sys/kernel/split_lock_mitigate";

pub const SPLIT_LOCK_FILE: &str = "/etc/sysctl.d/91-trindle-split-lock.conf";
pub const SPLIT_LOCK_TEXT: &str = "kernel.split_lock_mitigate = 0\n";

pub fn split_lock_off() -> Option<bool> {
    std::fs::read_to_string(SPLIT_LOCK).ok().map(|v| v.trim() == "0")
}

pub const DRIVE_DIR: &str = "/mnt/games";

pub const STEAM: &str = "com.valvesoftware.Steam";

pub fn steam_reach() -> Vec<String> {
    ["flatpak", "override", "--user", &format!("--filesystem={DRIVE_DIR}"), STEAM].map(String::from).to_vec()
}

const LINUX: [&str; 5] = ["ext4", "btrfs", "xfs", "f2fs", "bcachefs"];

pub fn refused(fs: &str) -> Option<&'static str> {
    match fs {
        f if LINUX.contains(&f) => None,
        "ntfs" | "ntfs3" | "vfat" | "exfat" | "msdos" => Some(t!("settings-games-drive-windows")),
        "crypto_LUKS" => Some(t!("settings-games-drive-locked")),
        "swap" | "LVM2_member" | "linux_raid_member" => Some(t!("settings-games-drive-not-files")),
        _ => Some(t!("settings-games-drive-unknown")),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Drive {
    pub uuid: String,
    pub fs: String,
    pub label: String,
    pub bytes: u64,

    pub games: bool,
}

const SYSTEM: [&str; 6] = ["/", "/boot", "/boot/efi", "/home", "/usr", "/var"];

const SYSTEM_PARTS: [&str; 8] = [
    "c12a7328-f81f-11d2-ba4b-00a0c93ec93b",
    "bc13c2ff-59e6-4262-a352-b275fd6f7172",
    "4f68bce3-e8cd-4db1-96e7-fbcaf984b709",
    "b921b045-1df0-41c3-af44-4c6f280d3fae",
    "8484680c-9521-48c6-9c11-b0720656f69e",
    "b0e01050-ee5f-4390-949a-9101b17104e9",
    "44479540-f297-41b2-9af7-d131d5f0458a",
    "0xef",
];

pub const LSBLK: [&str; 5] = ["lsblk", "-J", "-b", "-o", "NAME,UUID,FSTYPE,LABEL,SIZE,MOUNTPOINTS,PARTTYPE"];

pub fn drives(lsblk: &str) -> Vec<Drive> {
    let Ok(v) = serde_json::from_str::<Value>(lsblk) else { return Vec::new() };
    let mut out = Vec::new();
    for disk in v["blockdevices"].as_array().into_iter().flatten() {
        if !holds_system(disk) {
            gather(disk, &mut out);
        }
    }
    out
}

fn mounts(dev: &Value) -> impl Iterator<Item = &str> {
    dev["mountpoints"].as_array().into_iter().flatten().filter_map(Value::as_str)
}

fn holds_system(dev: &Value) -> bool {
    mounts(dev).any(|m| SYSTEM.contains(&m) || m == "[SWAP]")
        || dev["parttype"].as_str().is_some_and(|t| SYSTEM_PARTS.contains(&t.to_ascii_lowercase().as_str()))
        || dev["children"].as_array().into_iter().flatten().any(holds_system)
}

pub fn is_a_systems_root(top: &std::path::Path) -> bool {
    ["etc", "usr"].iter().all(|d| top.join(d).is_dir())
}

fn gather(dev: &Value, out: &mut Vec<Drive>) {
    let text = |k: &str| dev[k].as_str().unwrap_or_default().to_string();
    let (uuid, fs) = (text("uuid"), text("fstype"));
    if !uuid.is_empty() && !fs.is_empty() {
        let bytes = dev["size"].as_u64().or_else(|| dev["size"].as_str().and_then(|s| s.parse().ok())).unwrap_or(0);
        out.push(Drive { uuid, fs, label: text("label"), bytes, games: mounts(dev).any(|m| m == DRIVE_DIR) });
    }
    for child in dev["children"].as_array().into_iter().flatten() {
        gather(child, out);
    }
}

pub fn valid_uuid(uuid: &str) -> bool {
    (4..=64).contains(&uuid.len()) && uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

pub fn fstab_with(fstab: &str, line: Option<&str>) -> String {
    let mut out: String = fstab.lines()
        .filter(|l| l.trim_start().starts_with('#') || l.split_whitespace().nth(1) != Some(DRIVE_DIR))
        .flat_map(|l| [l, "\n"])
        .collect();
    if let Some(line) = line {
        out.push_str(line);
        out.push('\n');
    }
    out
}

pub fn fstab_line(uuid: &str, fs: &str) -> String {
    format!("UUID={uuid} {DRIVE_DIR} {fs} nofail,nosuid,nodev,x-systemd.device-timeout=5s,x-gvfs-show 0 0")
}

#[cfg(test)]
mod tests {
    use super::*;

    const LSBLK: &str = r#"{"blockdevices":[
      {"uuid":null,"fstype":null,"label":null,"size":512110190592,"mountpoints":[null],"children":[
        {"uuid":"AB12-CD34","fstype":"vfat","label":null,"size":536870912,"mountpoints":["/boot/efi"]},
        {"uuid":"1111-root","fstype":"btrfs","label":"trindle","size":511000000000,"mountpoints":["/","/home"]}]},
      {"uuid":null,"fstype":null,"label":null,"size":2000398934016,"mountpoints":[null],"children":[
        {"uuid":"2222-aaaa","fstype":"ext4","label":"Games","size":1000000000000,"mountpoints":["/mnt/games"]},
        {"uuid":"3333-bbbb","fstype":"ntfs","label":"Windows","size":1000000000000,"mountpoints":[null]}]},
      {"uuid":"4444-cccc","fstype":"xfs","label":"Spare","size":"256000000000","mountpoints":[null]},
      {"uuid":null,"fstype":null,"label":null,"size":2000398934016,"mountpoints":[null],"children":[
        {"uuid":"717C-300C","fstype":"vfat","label":null,"size":629145600,"mountpoints":[null],"parttype":"c12a7328-f81f-11d2-ba4b-00a0c93ec93b"},
        {"uuid":"5555-boot","fstype":"ext4","label":"other_xboot","size":2147483648,"mountpoints":[null],"parttype":"0fc63daf-8483-4772-8e79-3d69d8477de4"},
        {"uuid":"6666-root","fstype":"btrfs","label":"other_root","size":1900000000000,"mountpoints":[null],"parttype":"0fc63daf-8483-4772-8e79-3d69d8477de4"}]}]}"#;

    #[test]
    fn the_computers_own_drive_and_another_systems_are_never_offered_and_every_other_part_is() {
        let found = drives(LSBLK);
        let ids: Vec<&str> = found.iter().map(|d| d.uuid.as_str()).collect();
        assert_eq!(ids, ["2222-aaaa", "3333-bbbb", "4444-cccc"], "a drive with another system's EFI part was offered");
        assert!(found[0].games && !found[1].games);
        assert_eq!(found[2].bytes, 256_000_000_000, "a size lsblk gives as words is read too");
        assert!(drives("not json").is_empty());
        let root = std::env::temp_dir().join(format!("trindle-games-root-{}", std::process::id()));
        std::fs::create_dir_all(root.join("etc")).unwrap();
        std::fs::create_dir_all(root.join("usr")).unwrap();
        assert!(is_a_systems_root(&root), "a folder with etc and usr is a system's root");
        std::fs::remove_dir_all(root.join("usr")).unwrap();
        assert!(!is_a_systems_root(&root), "a games folder that happens to have an etc folder is not");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn lsblk_is_asked_for_names_or_it_prints_no_tree() {
        assert!(super::LSBLK.last().unwrap().split(',').any(|c| c == "NAME"));
    }

    #[test]
    fn only_a_linux_filesystem_will_do_for_games() {
        for fs in ["ext4", "btrfs", "xfs"] {
            assert_eq!(refused(fs), None);
        }
        for fs in ["ntfs", "exfat", "vfat", "crypto_LUKS", "swap", "iso9660"] {
            assert!(refused(fs).is_some(), "{fs}");
        }
    }

    #[test]
    fn the_games_line_replaces_its_old_self_and_touches_nothing_else() {
        let fstab = "# /etc/fstab\nUUID=1111-root / btrfs defaults 0 0\nUUID=old /mnt/games ext4 nofail 0 0\n";
        let new = fstab_with(fstab, Some(&fstab_line("2222-aaaa", "ext4")));
        assert_eq!(new, "# /etc/fstab\nUUID=1111-root / btrfs defaults 0 0\nUUID=2222-aaaa /mnt/games ext4 nofail,nosuid,nodev,x-systemd.device-timeout=5s,x-gvfs-show 0 0\n");
        assert_eq!(fstab_with(&new, None), "# /etc/fstab\nUUID=1111-root / btrfs defaults 0 0\n");
        assert!(valid_uuid("2222-aaaa") && valid_uuid("AB12-CD34"));
        assert!(!valid_uuid("x y") && !valid_uuid("../etc") && !valid_uuid(""));
    }

    #[test]
    fn the_sound_fix_raises_the_smallest_buffer_and_off_hands_it_back() {
        assert!(sound_text().contains("default.clock.min-quantum = 1024"));
        assert_eq!(sound_now(true).last().map(String::as_str), Some("1024"));
        assert_eq!(sound_now(false).last().map(String::as_str), Some("0"));
        assert_eq!(steam_reach()[3], "--filesystem=/mnt/games");
    }

    #[test]
    fn the_package_gives_an_old_games_line_the_new_options() {
        let postinst = include_str!("../../tools/deb/postinst");
        let command = postinst.lines().find(|l| l.starts_with("sed -i -E") && l.contains("/mnt/games")).expect("an upgrade step for the games line");
        let dir = std::env::temp_dir().join(format!("trindle-games-fstab-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("fstab");
        let old = "UUID=1111-root / btrfs defaults 0 0\nUUID=2222-aaaa /mnt/games ext4 nofail,x-systemd.device-timeout=5s,x-gvfs-show 0 0\n";
        std::fs::write(&file, old).unwrap();
        let run = || {
            let script = command.replace("/etc/fstab", &file.to_string_lossy()).replace(" 2>/dev/null || true", "");
            assert!(std::process::Command::new("sh").arg("-c").arg(script).status().unwrap().success());
            std::fs::read_to_string(&file).unwrap()
        };
        let new = run();
        assert_eq!(new, "UUID=1111-root / btrfs defaults 0 0\n".to_string() + &fstab_line("2222-aaaa", "ext4") + "\n");
        assert_eq!(run(), new, "a line that is already right is left alone");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
