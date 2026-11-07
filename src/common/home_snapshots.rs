//! Snapshots of the home folder, so that an earlier version of a file can be opened or put back.

use std::path::{Path, PathBuf};

use crate::common::snapshots::xml_field;

pub const CONFIG: &str = "home";
pub const HOME: &str = "/home";
pub const SNAPSHOTS: &str = "/home/.snapshots";
pub const SETTING: &str = "/etc/trindle/home-snapshots";
pub const STATE: &str = "/var/lib/trindle/home-snapshots";
pub const SNAPPER_CONFIG: &str = "/etc/snapper/configs/home";
pub const DESCRIPTION: &str = "trindle-home";
pub const TIMER: &str = "trindle-home-snapshots.timer";

pub const KEEP_CHOICES: [u32; 4] = [3, 7, 14, 30];
pub const KEEP_DEFAULT: u32 = 7;

pub fn keep_words(days: u32) -> String {
    match days {
        7 => "a week".into(),
        14 => "two weeks".into(),
        30 => "a month".into(),
        n => format!("{n} days"),
    }
}

pub const GAP: i64 = 20 * 3600;

pub const FLOOR_BYTES: u64 = 2 * 1024 * 1024 * 1024;
pub const FLOOR_SHARE: u64 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setting {
    pub on: bool,
    pub days: u32,
}

impl Default for Setting {
    fn default() -> Setting {
        Setting { on: false, days: KEEP_DEFAULT }
    }
}

impl Setting {
    pub fn parse(text: &str) -> Setting {
        let mut words = text.split_whitespace();
        let on = words.next() == Some("on");
        let days = words.next().and_then(|d| d.parse().ok()).filter(|d| KEEP_CHOICES.contains(d)).unwrap_or(KEEP_DEFAULT);
        Setting { on, days }
    }

    pub fn text(&self) -> String {
        format!("{} {}\n", if self.on { "on" } else { "off" }, self.days)
    }

    pub fn read() -> Setting {
        Setting::parse(&std::fs::read_to_string(SETTING).unwrap_or_default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mount {
    pub point: String,
    pub root: String,
    pub fs: String,
    pub read_only: bool,
}

fn unescaped(word: &str) -> String {
    let bytes = word.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let octal = bytes.get(i + 1..i + 4).filter(|d| bytes[i] == b'\\' && d.iter().all(|c| (b'0'..=b'7').contains(c)));
        match octal {
            Some(d) => {
                out.push(((d[0] - b'0') << 6) | ((d[1] - b'0') << 3) | (d[2] - b'0'));
                i += 4;
            }
            None => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

pub fn mounts(mountinfo: &str) -> Vec<Mount> {
    mountinfo.lines().filter_map(|line| {
        let (before, after) = line.split_once(" - ")?;
        let mut b = before.split(' ');
        let (root, point, opts) = (b.nth(3)?, b.next()?, b.next()?);
        let fs = after.split(' ').next()?;
        Some(Mount { point: unescaped(point), root: unescaped(root), fs: fs.to_string(),
                     read_only: opts.split(',').any(|o| o == "ro") })
    }).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotHere {
    NoSnapper,
    FromSnapshot,
    PartOfSystem,
    NotBtrfs(String),
    NotWhole,
    ReadOnly,
    LockedFiles,
    Elsewhere,
    Apart,
    OwnPart,
}

impl NotHere {
    pub fn words(&self) -> String {
        match self {
            NotHere::NoSnapper => "Earlier versions need snapper, which is not installed on this computer.".into(),
            NotHere::FromSnapshot => "Not while the computer is started from a snapshot.".into(),
            NotHere::PartOfSystem =>
                "The home folder is part of the system on this computer, not a part of the disk of its own, so \
                 earlier versions cannot be kept apart from the system.".into(),
            NotHere::NotBtrfs(fs) if is_network(fs) =>
                "The home folder is kept on another computer, so earlier versions cannot be kept here.".into(),
            NotHere::NotBtrfs(fs) => format!("Earlier versions need the home folder on Btrfs, and here it is on {fs}."),
            NotHere::NotWhole =>
                "The home folder is a folder inside another part of the disk, not a part of its own, so earlier \
                 versions cannot be kept for it.".into(),
            NotHere::ReadOnly => "The home folder cannot be written to at the moment, so nothing can be kept.".into(),
            NotHere::LockedFiles =>
                "A home folder here is kept as one locked file, and a copy of it each day would fill the disk.".into(),
            NotHere::Elsewhere => "Your home folder is not inside /home, where the copies are taken.".into(),
            NotHere::Apart =>
                "Your home folder is locked or kept apart from the others, so the copies cannot hold it.".into(),
            NotHere::OwnPart =>
                "Your home folder is a part of the disk of its own, which the copies of /home do not go into.".into(),
        }
    }
}

fn is_network(fs: &str) -> bool {
    matches!(fs, "nfs" | "nfs4" | "cifs" | "smb3" | "autofs" | "ceph" | "glusterfs" | "9p" | "afs")
        || fs.starts_with("fuse.sshfs")
}

pub const SUBVOLUME_INODE: u64 = 256;

pub struct Seen<'a> {
    pub mountinfo: &'a str,
    pub home_inode: u64,
    pub snapper: bool,
    pub from_snapshot: bool,
    pub locked_files: bool,
}

pub fn possible(seen: &Seen) -> Result<(), NotHere> {
    if !seen.snapper {
        return Err(NotHere::NoSnapper);
    }
    if seen.from_snapshot {
        return Err(NotHere::FromSnapshot);
    }
    let all = mounts(seen.mountinfo);
    let Some(home) = all.iter().rev().find(|m| m.point == HOME) else { return Err(NotHere::PartOfSystem) };
    if home.fs != "btrfs" {
        return Err(NotHere::NotBtrfs(home.fs.clone()));
    }
    if seen.home_inode != SUBVOLUME_INODE {
        return Err(NotHere::NotWhole);
    }
    if home.read_only {
        return Err(NotHere::ReadOnly);
    }
    if seen.locked_files {
        return Err(NotHere::LockedFiles);
    }
    Ok(())
}

pub fn reaches(mountinfo: &str, person: &Path, person_inode: u64) -> Result<(), NotHere> {
    let Ok(inside) = person.strip_prefix(HOME) else { return Err(NotHere::Elsewhere) };
    if inside.as_os_str().is_empty() {
        return Err(NotHere::Elsewhere);
    }
    let all = mounts(mountinfo);
    let hidden = all.iter().any(|m| {
        let p = Path::new(&m.point);
        p != Path::new(HOME) && p.starts_with(HOME) && person.starts_with(p)
    });
    if hidden {
        return Err(NotHere::Apart);
    }
    if person_inode == SUBVOLUME_INODE {
        return Err(NotHere::OwnPart);
    }
    Ok(())
}

fn inode(path: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).map(|m| m.ino()).unwrap_or(0)
}

fn locked_files_in(home: &Path) -> bool {
    std::fs::read_dir(home).is_ok_and(|d| d.flatten().any(|e| {
        e.file_name().to_string_lossy().ends_with(".home") && e.file_type().is_ok_and(|t| t.is_file())
    }))
}

pub fn possible_here() -> Result<(), NotHere> {
    let mountinfo = std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default();
    possible(&Seen {
        mountinfo: &mountinfo,
        home_inode: inode(Path::new(HOME)),
        snapper: crate::common::run::have("snapper"),
        from_snapshot: crate::common::snapshots::booted_snapshot().is_some(),
        locked_files: locked_files_in(Path::new(HOME)),
    })
}

pub fn possible_for_me() -> Result<(), NotHere> {
    possible_here()?;
    let mountinfo = std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default();
    let me = crate::common::files::home();
    let me = std::fs::canonicalize(&me).unwrap_or(me);
    reaches(&mountinfo, &me, inode(&me))
}

#[derive(Clone, Copy, Debug)]
pub struct Room {
    pub total: u64,
    pub free: u64,
}

pub fn nearly_full(room: Room) -> bool {
    room.free < FLOOR_BYTES || room.free.saturating_mul(FLOOR_SHARE) < room.total
}

pub fn room_on(path: &Path) -> Option<Room> {
    use std::os::unix::ffi::OsStrExt;
    let c = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    // SAFETY: statvfs is plain numbers, so all-zeroes is a valid starting
    // value, and the kernel fills it in.
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: a NUL-terminated path that outlives the call, and a local
    // struct the kernel fills in.
    (unsafe { libc::statvfs(c.as_ptr(), &mut st) } == 0).then(|| Room {
        total: st.f_blocks.saturating_mul(st.f_frsize),
        free: st.f_bavail.saturating_mul(st.f_frsize),
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kept {
    pub number: u32,
    pub taken: i64,
    pub ours: bool,
}

pub fn most(days: u32) -> u32 {
    days + 2
}

pub fn due(kept: &[Kept], now: i64) -> bool {
    !kept.iter().any(|k| k.ours && now - k.taken < GAP)
}

pub fn to_delete(kept: &[Kept], now: i64, days: u32) -> Vec<u32> {
    let mut ours: Vec<&Kept> = kept.iter().filter(|k| k.ours).collect();
    ours.sort_by_key(|k| std::cmp::Reverse((k.taken, k.number)));
    let limit = i64::from(days) * 86_400;
    let mut out: Vec<u32> = ours.iter().enumerate()
        .filter(|(i, k)| now - k.taken > limit || *i >= most(days) as usize)
        .map(|(_, k)| k.number).collect();
    out.sort_unstable();
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub delete: Vec<u32>,
    pub take: bool,
    pub skipped_full: bool,
}

pub fn plan(setting: Setting, kept: &[Kept], now: i64, room: Option<Room>) -> Plan {
    let wanted = setting.on && due(kept, now);
    let full = room.is_some_and(nearly_full);
    Plan { delete: to_delete(kept, now, setting.days), take: wanted && room.is_some() && !full, skipped_full: wanted && full }
}

pub fn unix_of(date: &str) -> Option<i64> {
    let num = |r: std::ops::Range<usize>| date.get(r)?.parse::<i64>().ok();
    let (y, m, d, h, mi, s) = (num(0..4)?, num(5..7)?, num(8..10)?, num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || h > 23 || mi > 59 || s > 60 {
        return None;
    }
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146_097 + doe - 719_468) * 86_400 + h * 3600 + mi * 60 + s)
}

pub fn kept_in(dir: &Path) -> Vec<Kept> {
    let mut out: Vec<Kept> = std::fs::read_dir(dir).map(|d| d.flatten().filter_map(|e| {
        let number: u32 = e.file_name().to_str()?.parse().ok().filter(|n| *n > 0)?;
        let base = e.path();
        if !std::fs::symlink_metadata(base.join("snapshot")).is_ok_and(|m| m.is_dir()) {
            return None;
        }
        let info = std::fs::read_to_string(base.join("info.xml")).unwrap_or_default();
        Some(Kept { number, taken: unix_of(&xml_field(&info, "date")).unwrap_or(0),
                    ours: xml_field(&info, "description") == DESCRIPTION })
    }).collect()).unwrap_or_default();
    out.sort_by_key(|k| k.number);
    out
}

pub const SNAPPER_TEMPLATE: &str = "/usr/share/snapper/config-templates/default";
pub const SNAPPER_LIST: &str = "/etc/default/snapper";

fn words(argv: &[&str]) -> Vec<String> {
    argv.iter().map(|w| w.to_string()).collect()
}

const SNAPPER: [&str; 4] = ["snapper", "--no-dbus", "-c", CONFIG];

pub fn config_values(days: u32) -> Vec<String> {
    vec!["TIMELINE_CREATE=no".into(), "TIMELINE_CLEANUP=no".into(), "NUMBER_CLEANUP=yes".into(),
         format!("NUMBER_LIMIT={}", most(days)), "SYNC_ACL=no".into()]
}

pub fn setup_commands(set_up: bool, days: u32) -> Vec<Vec<String>> {
    let mut all = Vec::new();
    if !set_up {
        let mut make = words(&SNAPPER);
        make.extend(words(&["create-config", HOME]));
        all.push(make);
    }
    let mut set = words(&SNAPPER);
    set.push("set-config".into());
    set.extend(config_values(days));
    all.push(set);
    all
}

pub fn take_command() -> Vec<String> {
    let mut take = words(&SNAPPER);
    take.extend(words(&["create", "--description", DESCRIPTION, "--cleanup-algorithm", "number"]));
    take
}

pub fn delete_command(numbers: &[u32], at_once: bool) -> Option<Vec<String>> {
    if numbers.is_empty() {
        return None;
    }
    let mut delete = words(&SNAPPER);
    delete.push("delete".into());
    if at_once {
        delete.push("--sync".into());
    }
    delete.extend(numbers.iter().map(u32::to_string));
    Some(delete)
}

pub fn adopted_config(template: &str, days: u32) -> String {
    let mut values: Vec<(String, String)> = vec![("SUBVOLUME".into(), HOME.into()), ("FSTYPE".into(), "btrfs".into())];
    values.extend(config_values(days).iter().filter_map(|v| v.split_once('=')).map(|(k, v)| (k.to_string(), v.to_string())));
    let mut out = String::new();
    for line in template.lines() {
        let key = line.split_once('=').map(|(k, _)| k.trim());
        match values.iter().position(|(k, _)| Some(k.as_str()) == key) {
            Some(i) => {
                let (k, v) = values.remove(i);
                out.push_str(&format!("{k}=\"{v}\"\n"));
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    for (k, v) in values {
        out.push_str(&format!("{k}=\"{v}\"\n"));
    }
    out
}

pub fn listed(list: &str) -> Option<String> {
    let is_line = |l: &str| l.trim_start().starts_with("SNAPPER_CONFIGS=");
    let now: Vec<String> = list.lines().find(|l| is_line(l)).and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim().trim_matches('"').split_whitespace().map(String::from).collect()).unwrap_or_default();
    if now.iter().any(|c| c == CONFIG) {
        return None;
    }
    let mut all = now;
    all.push(CONFIG.into());
    let line = format!("SNAPPER_CONFIGS=\"{}\"", all.join(" "));
    let mut out: Vec<String> = Vec::new();
    let mut put = false;
    for l in list.lines() {
        if is_line(l) {
            if !put {
                out.push(line.clone());
                put = true;
            }
        } else {
            out.push(l.to_string());
        }
    }
    if !put {
        out.push(line);
    }
    Some(out.join("\n") + "\n")
}

pub const FOLDER_MODE: u32 = 0o755;

#[cfg(test)]
pub fn others_may_enter(mode: u32) -> bool {
    mode & 0o001 != 0
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub skipped_full: Option<i64>,
    pub room: Option<(u64, i64)>,
}

impl State {
    pub fn parse(text: &str) -> State {
        let mut s = State::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else { continue };
            let mut nums = value.split_whitespace().filter_map(|n| n.parse::<i64>().ok());
            match key.trim() {
                "skipped-full" => s.skipped_full = nums.next(),
                "room" => s.room = nums.next().zip(nums.next()).map(|(bytes, at)| (bytes.max(0) as u64, at)),
                _ => {}
            }
        }
        s
    }

    pub fn text(&self) -> String {
        let mut out = String::new();
        if let Some(at) = self.skipped_full {
            out.push_str(&format!("skipped-full={at}\n"));
        }
        if let Some((bytes, at)) = self.room {
            out.push_str(&format!("room={bytes} {at}\n"));
        }
        out
    }

    pub fn read() -> State {
        State::parse(&std::fs::read_to_string(STATE).unwrap_or_default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Standing {
    pub possible: Result<(), NotHere>,
    pub setting: Setting,
    pub kept: Vec<Kept>,
    pub state: State,
}

impl Default for Standing {
    fn default() -> Standing {
        Standing { possible: Ok(()), setting: Setting::default(), kept: Vec::new(), state: State::default() }
    }
}

pub fn standing() -> Standing {
    let set_up = Path::new(SNAPPER_CONFIG).exists();
    Standing {
        possible: possible_for_me(),
        setting: Setting { on: set_up && Setting::read().on, ..Setting::read() },
        kept: kept_in(Path::new(SNAPSHOTS)),
        state: State::read(),
    }
}

pub fn tree(snapshots: &Path, number: u32) -> PathBuf {
    snapshots.join(number.to_string()).join("snapshot")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) const OURS: &str = "\
30 1 0:26 /@rootfs / rw,noatime shared:1 - btrfs /dev/nvme0n1p3 rw,compress=zstd:1,subvolid=256,subvol=/@rootfs\n\
48 30 0:26 /@home /home rw,noatime shared:126 - btrfs /dev/nvme0n1p3 rw,compress=zstd:1,subvolid=257,subvol=/@home\n\
56 30 0:48 / /tmp rw,nosuid,nodev shared:130 - tmpfs tmpfs rw,size=16077924k\n\
487 28 0:73 / /run/user/1000 rw,nosuid,nodev,relatime shared:320 - tmpfs tmpfs rw,mode=700,uid=1000\n";

    fn seen(mountinfo: &str) -> Seen<'_> {
        Seen { mountinfo, home_inode: SUBVOLUME_INODE, snapper: true, from_snapshot: false, locked_files: false }
    }

    fn with_home(line: &str) -> String {
        format!("30 1 0:26 /@rootfs / rw,noatime shared:1 - btrfs /dev/sda2 rw,subvol=/@rootfs\n{line}\n")
    }

    #[test]
    fn the_mount_table_is_read() {
        let all = mounts(OURS);
        assert_eq!(all.len(), 4);
        assert_eq!(all[1], Mount { point: "/home".into(), root: "/@home".into(), fs: "btrfs".into(), read_only: false });
        let odd = mounts("70 30 8:1 / /media/My\\040Disk ro,relatime - ext4 /dev/sdb1 rw\n");
        assert_eq!(odd[0].point, "/media/My Disk");
        assert!(odd[0].read_only);
        assert!(mounts("rubbish\n\n").is_empty());
    }

    #[test]
    fn trindles_own_layout_can_have_them() {
        assert_eq!(possible(&seen(OURS)), Ok(()));
    }

    #[test]
    fn a_whole_btrfs_disk_for_home_can_have_them() {
        let other = with_home("50 30 0:40 / /home rw,relatime shared:9 - btrfs /dev/sdb1 rw,subvolid=5,subvol=/");
        assert_eq!(possible(&seen(&other)), Ok(()));
        let locked = with_home("50 30 0:41 /@home /home rw - btrfs /dev/mapper/luks-1234 rw,subvol=/@home");
        assert_eq!(possible(&seen(&locked)), Ok(()));
    }

    #[test]
    fn every_other_layout_says_why_not() {
        let part = "30 1 0:26 /@rootfs / rw - btrfs /dev/sda2 rw\n";
        assert_eq!(possible(&seen(part)), Err(NotHere::PartOfSystem));
        assert_eq!(possible(&seen("30 1 8:2 / / rw - ext4 /dev/sda2 rw\n")), Err(NotHere::PartOfSystem));
        assert_eq!(possible(&seen("")), Err(NotHere::PartOfSystem));
        let ext4 = with_home("50 30 8:17 / /home rw,relatime shared:9 - ext4 /dev/sdb1 rw");
        assert_eq!(possible(&seen(&ext4)), Err(NotHere::NotBtrfs("ext4".into())));
        assert_eq!(NotHere::NotBtrfs("ext4".into()).words(),
                   "Earlier versions need the home folder on Btrfs, and here it is on ext4.");
        let nfs = with_home("50 30 0:55 /export/home /home rw,relatime shared:9 - nfs4 server:/export/home rw");
        assert_eq!(possible(&seen(&nfs)), Err(NotHere::NotBtrfs("nfs4".into())));
        assert!(NotHere::NotBtrfs("nfs4".into()).words().contains("another computer"));
        let auto = with_home("50 30 0:56 / /home rw,relatime shared:9 - autofs /etc/auto.home rw");
        assert!(possible(&seen(&auto)).unwrap_err().words().contains("another computer"));
        let bound = with_home("50 30 0:26 /@data/home /home rw - btrfs /dev/sda2 rw,subvol=/@data");
        assert_eq!(possible(&Seen { home_inode: 4711, ..seen(&bound) }), Err(NotHere::NotWhole));
        let ro = with_home("50 30 0:26 /@home /home ro,noatime - btrfs /dev/sda2 rw,subvol=/@home");
        assert_eq!(possible(&seen(&ro)), Err(NotHere::ReadOnly));
        let over = format!("{OURS}90 30 8:33 / /home rw - ext4 /dev/sdc1 rw\n");
        assert_eq!(possible(&seen(&over)), Err(NotHere::NotBtrfs("ext4".into())));
    }

    #[test]
    fn the_computer_itself_can_rule_them_out() {
        assert_eq!(possible(&Seen { snapper: false, ..seen(OURS) }), Err(NotHere::NoSnapper));
        assert_eq!(possible(&Seen { from_snapshot: true, ..seen(OURS) }), Err(NotHere::FromSnapshot));
        assert_eq!(possible(&Seen { locked_files: true, ..seen(OURS) }), Err(NotHere::LockedFiles));
    }

    #[test]
    fn a_home_the_copies_cannot_hold_says_so() {
        let john = Path::new("/home/john");
        assert_eq!(reaches(OURS, john, 257), Ok(()));
        assert_eq!(reaches(OURS, Path::new("/home/staff/john"), 300), Ok(()));
        let locked = format!("{OURS}300 48 0:60 / /home/john rw,nosuid,nodev - ecryptfs /home/.ecryptfs/john/.Private rw\n");
        assert_eq!(reaches(&locked, john, 2), Err(NotHere::Apart));
        assert_eq!(reaches(&locked, Path::new("/home/jane"), 258), Ok(()));
        let homed = format!("{OURS}301 48 253:2 / /home/john rw,nosuid,nodev - btrfs /dev/mapper/home-john rw\n");
        assert_eq!(reaches(&homed, john, 256), Err(NotHere::Apart));
        let between = format!("{OURS}302 48 8:49 / /home/staff rw - ext4 /dev/sdd1 rw\n");
        assert_eq!(reaches(&between, Path::new("/home/staff/john"), 12), Err(NotHere::Apart));
        let inside = format!("{OURS}303 48 0:83 / /home/john/.cache/doc rw - fuse.portal portal rw\n");
        assert_eq!(reaches(&inside, john, 257), Ok(()));
        assert_eq!(reaches(OURS, john, SUBVOLUME_INODE), Err(NotHere::OwnPart));
        assert_eq!(reaches(OURS, Path::new("/var/lib/john"), 9), Err(NotHere::Elsewhere));
        assert_eq!(reaches(OURS, Path::new("/home"), 256), Err(NotHere::Elsewhere));
        assert_eq!(reaches(OURS, Path::new("/homes/john"), 9), Err(NotHere::Elsewhere));
    }

    #[test]
    fn every_reason_is_one_plain_sentence() {
        for why in [NotHere::NoSnapper, NotHere::FromSnapshot, NotHere::PartOfSystem, NotHere::NotBtrfs("xfs".into()),
                    NotHere::NotBtrfs("cifs".into()), NotHere::NotWhole, NotHere::ReadOnly, NotHere::LockedFiles,
                    NotHere::Elsewhere, NotHere::Apart, NotHere::OwnPart] {
            let words = why.words();
            assert!(words.ends_with('.') && words.matches(". ").count() == 0, "{words}");
            assert!(!words.contains('\u{2014}'), "{words}");
        }
    }

    #[test]
    fn the_setting_is_off_unless_it_plainly_says_on() {
        assert_eq!(Setting::parse(""), Setting { on: false, days: 7 });
        assert_eq!(Setting::parse("on 14\n"), Setting { on: true, days: 14 });
        assert_eq!(Setting::parse("off 3"), Setting { on: false, days: 3 });
        assert_eq!(Setting::parse("on 3650"), Setting { on: true, days: 7 });
        assert_eq!(Setting::parse("on forever"), Setting { on: true, days: 7 });
        assert_eq!(Setting::parse("yes"), Setting { on: false, days: 7 });
        for days in KEEP_CHOICES {
            let s = Setting { on: true, days };
            assert_eq!(Setting::parse(&s.text()), s);
        }
        assert_eq!(KEEP_CHOICES.map(keep_words), ["3 days", "a week", "two weeks", "a month"]);
    }

    const GB: u64 = 1024 * 1024 * 1024;

    #[test]
    fn no_copy_is_taken_on_a_nearly_full_disk() {
        assert!(nearly_full(Room { total: 20 * GB, free: 2 * GB - 1 }));
        assert!(!nearly_full(Room { total: 20 * GB, free: 2 * GB }));
        assert!(nearly_full(Room { total: 1000 * GB, free: 49 * GB }));
        assert!(!nearly_full(Room { total: 1000 * GB, free: 50 * GB }));
        assert!(nearly_full(Room { total: 0, free: 0 }));
    }

    const DAY: i64 = 86_400;

    fn ours(number: u32, taken: i64) -> Kept {
        Kept { number, taken, ours: true }
    }

    #[test]
    fn one_copy_a_day_at_most() {
        let now = 100 * DAY;
        assert!(due(&[], now));
        assert!(!due(&[ours(1, now - 3600)], now));
        assert!(!due(&[ours(1, now - GAP + 1)], now));
        assert!(due(&[ours(1, now - GAP)], now));
        assert!(due(&[Kept { number: 1, taken: now - 60, ours: false }], now));
    }

    #[test]
    fn copies_go_when_their_time_is_up_and_never_pile_up() {
        let now = 100 * DAY;
        let week: Vec<Kept> = (0..10).map(|i| ours(i + 1, now - (9 - i64::from(i)) * DAY - 60)).collect();
        assert_eq!(to_delete(&week, now, 7), [1, 2, 3]);
        assert_eq!(to_delete(&week, now, 3), [1, 2, 3, 4, 5, 6, 7]);
        assert!(to_delete(&week, now, 14).is_empty());
        let burst: Vec<Kept> = (1..=30).map(|i| ours(i, now - 600 + i64::from(i))).collect();
        let gone = to_delete(&burst, now, 7);
        assert_eq!(gone.len(), 21);
        assert_eq!(gone, (1..=21).collect::<Vec<u32>>());
        for days in KEEP_CHOICES {
            assert_eq!(burst.len() - to_delete(&burst, now, days).len(), burst.len().min(most(days) as usize));
        }
        let hand = [Kept { number: 1, taken: 0, ours: false }, ours(2, now - 30 * DAY)];
        assert_eq!(to_delete(&hand, now, 7), [2]);
    }

    #[test]
    fn one_look_tidies_then_takes_only_when_it_should() {
        let now = 100 * DAY;
        let roomy = Some(Room { total: 500 * GB, free: 200 * GB });
        let tight = Some(Room { total: 500 * GB, free: 10 * GB });
        let on = Setting { on: true, days: 7 };
        let off = Setting { on: false, days: 7 };
        let old = [ours(1, now - 9 * DAY), ours(2, now - 2 * DAY)];
        assert_eq!(plan(on, &old, now, roomy), Plan { delete: vec![1], take: true, skipped_full: false });
        assert_eq!(plan(on, &old, now, tight), Plan { delete: vec![1], take: false, skipped_full: true });
        let today = [ours(3, now - 3600)];
        assert_eq!(plan(on, &today, now, tight), Plan { delete: vec![], take: false, skipped_full: false });
        assert_eq!(plan(off, &old, now, roomy), Plan { delete: vec![1], take: false, skipped_full: false });
        assert_eq!(plan(on, &[], now, None), Plan { delete: vec![], take: false, skipped_full: false });
    }

    #[test]
    fn snappers_dates_are_read() {
        assert_eq!(unix_of("1970-01-01 00:00:00"), Some(0));
        assert_eq!(unix_of("2026-10-04 09:34:12"), Some(1_791_106_452));
        assert_eq!(unix_of("2024-02-29 12:00:00"), Some(1_709_208_000));
        assert_eq!(unix_of("2026-13-04 09:34:12"), None);
        assert_eq!(unix_of("yesterday"), None);
        assert_eq!(unix_of(""), None);
    }

    pub(crate) fn made_up_snapshots(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-home-snapshots-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    pub(crate) fn add_snapshot(dir: &Path, number: u32, date: &str, description: &str) -> PathBuf {
        let base = dir.join(number.to_string());
        std::fs::create_dir_all(base.join("snapshot")).unwrap();
        std::fs::write(base.join("info.xml"), format!(
            "<?xml version=\"1.0\"?>\n<snapshot>\n  <type>single</type>\n  <num>{number}</num>\n  \
             <date>{date}</date>\n  <description>{description}</description>\n  <cleanup>number</cleanup>\n</snapshot>\n")).unwrap();
        base.join("snapshot")
    }

    #[test]
    fn the_snapshots_are_read_from_their_folder() {
        let dir = made_up_snapshots("read");
        add_snapshot(&dir, 2, "2026-10-04 09:34:12", DESCRIPTION);
        add_snapshot(&dir, 10, "2026-10-05 09:34:12", "by hand");
        add_snapshot(&dir, 1, "2026-10-03 09:34:12", DESCRIPTION);
        std::fs::create_dir_all(dir.join("7")).unwrap();
        std::fs::create_dir_all(dir.join("notes/snapshot")).unwrap();
        std::fs::create_dir_all(dir.join("0/snapshot")).unwrap();
        let kept = kept_in(&dir);
        assert_eq!(kept.iter().map(|k| k.number).collect::<Vec<_>>(), [1, 2, 10]);
        assert_eq!(kept[1], Kept { number: 2, taken: 1_791_106_452, ours: true });
        assert!(!kept[2].ours);
        assert!(kept_in(&dir.join("none")).is_empty());
        assert_eq!(tree(&dir, 2), dir.join("2/snapshot"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn line(c: &[String]) -> String {
        c.join(" ")
    }

    #[test]
    fn the_commands_root_runs_are_exactly_these() {
        let fresh: Vec<String> = setup_commands(false, 7).iter().map(|c| line(c)).collect();
        assert_eq!(fresh, [
            "snapper --no-dbus -c home create-config /home",
            "snapper --no-dbus -c home set-config TIMELINE_CREATE=no TIMELINE_CLEANUP=no NUMBER_CLEANUP=yes NUMBER_LIMIT=9 SYNC_ACL=no",
        ]);
        let again: Vec<String> = setup_commands(true, 30).iter().map(|c| line(c)).collect();
        assert_eq!(again, ["snapper --no-dbus -c home set-config TIMELINE_CREATE=no TIMELINE_CLEANUP=no NUMBER_CLEANUP=yes NUMBER_LIMIT=32 SYNC_ACL=no"]);
        assert_eq!(line(&take_command()),
                   "snapper --no-dbus -c home create --description trindle-home --cleanup-algorithm number");
        assert_eq!(delete_command(&[], false), None);
        assert_eq!(line(&delete_command(&[3, 4], false).unwrap()), "snapper --no-dbus -c home delete 3 4");
        assert_eq!(line(&delete_command(&[3, 4, 9], true).unwrap()), "snapper --no-dbus -c home delete --sync 3 4 9");
        for c in setup_commands(false, 7).iter().chain([&take_command(), &delete_command(&[1], true).unwrap()]) {
            assert_eq!(c[..4], ["snapper", "--no-dbus", "-c", "home"]);
        }
    }

    const TEMPLATE: &str = "\n# subvolume to snapshot\nSUBVOLUME=\"/\"\n\n# filesystem type\nFSTYPE=\"btrfs\"\n\
        ALLOW_USERS=\"\"\nALLOW_GROUPS=\"\"\nSYNC_ACL=\"no\"\nNUMBER_CLEANUP=\"yes\"\nNUMBER_LIMIT=\"50\"\n\
        TIMELINE_CREATE=\"yes\"\nTIMELINE_CLEANUP=\"yes\"\nTIMELINE_LIMIT_DAILY=\"10\"\n";

    #[test]
    fn snapshots_left_with_no_config_get_one_that_takes_no_timeline() {
        let made = adopted_config(TEMPLATE, 7);
        for want in ["SUBVOLUME=\"/home\"\n", "FSTYPE=\"btrfs\"\n", "TIMELINE_CREATE=\"no\"\n", "TIMELINE_CLEANUP=\"no\"\n",
                     "NUMBER_CLEANUP=\"yes\"\n", "NUMBER_LIMIT=\"9\"\n", "SYNC_ACL=\"no\"\n", "ALLOW_USERS=\"\"\n",
                     "ALLOW_GROUPS=\"\"\n", "# subvolume to snapshot\n", "TIMELINE_LIMIT_DAILY=\"10\"\n"] {
            assert_eq!(made.matches(want).count(), 1, "{want} in {made}");
        }
        assert!(!made.contains("SUBVOLUME=\"/\"\n") && !made.contains("TIMELINE_CREATE=\"yes\""));
        let bare = adopted_config("", 3);
        assert_eq!(bare, "SUBVOLUME=\"/home\"\nFSTYPE=\"btrfs\"\nTIMELINE_CREATE=\"no\"\nTIMELINE_CLEANUP=\"no\"\n\
                          NUMBER_CLEANUP=\"yes\"\nNUMBER_LIMIT=\"5\"\nSYNC_ACL=\"no\"\n");
    }

    #[test]
    fn home_joins_snappers_list_once() {
        let debian = "# List of snapper configurations.\nSNAPPER_CONFIGS=\"root\"\n\nDISABLE_APT_SNAPSHOT=\"no\"\n";
        assert_eq!(listed(debian).unwrap(),
                   "# List of snapper configurations.\nSNAPPER_CONFIGS=\"root home\"\n\nDISABLE_APT_SNAPSHOT=\"no\"\n");
        assert_eq!(listed("SNAPPER_CONFIGS=\"root home\"\n"), None);
        assert_eq!(listed("SNAPPER_CONFIGS=\"\"\n").unwrap(), "SNAPPER_CONFIGS=\"home\"\n");
        assert_eq!(listed("").unwrap(), "SNAPPER_CONFIGS=\"home\"\n");
        assert_eq!(listed("SNAPPER_CONFIGS=\"root homework\"\n").unwrap(), "SNAPPER_CONFIGS=\"root homework home\"\n");
    }

    #[test]
    fn a_person_reaches_only_what_permissions_already_allow() {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(FOLDER_MODE, 0o755);
        assert!(others_may_enter(FOLDER_MODE));
        assert_eq!(FOLDER_MODE & 0o004, 0o004);
        assert_eq!(FOLDER_MODE & 0o022, 0);
        let dir = made_up_snapshots("access");
        let tree = add_snapshot(&dir, 1, "2026-10-04 09:34:12", DESCRIPTION);
        let mode = |p: &Path| std::fs::symlink_metadata(p).unwrap().permissions().mode() & 0o777;
        let set = |p: &Path, m: u32| std::fs::set_permissions(p, std::fs::Permissions::from_mode(m)).unwrap();
        for (name, home_mode) in [("john", 0o700), ("jane", 0o755), ("staff", 0o750)] {
            std::fs::create_dir_all(tree.join(name)).unwrap();
            std::fs::write(tree.join(name).join("diary.txt"), "private").unwrap();
            set(&tree.join(name).join("diary.txt"), 0o644);
            set(&tree.join(name), home_mode);
        }
        set(&dir, FOLDER_MODE);
        set(&dir.join("1"), 0o755);
        set(&tree, 0o755);
        for above in [dir.as_path(), &dir.join("1"), &tree] {
            assert!(others_may_enter(mode(above)), "{above:?}");
        }
        assert!(!others_may_enter(mode(&tree.join("john"))));
        assert!(!others_may_enter(mode(&tree.join("staff"))));
        assert!(others_may_enter(mode(&tree.join("jane"))));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_state_is_written_and_read_back() {
        let s = State { skipped_full: Some(1_791_106_452), room: Some((3_200_000_000, 1_791_100_000)) };
        assert_eq!(State::parse(&s.text()), s);
        assert_eq!(State::parse(""), State::default());
        assert_eq!(State::parse("room=12\nnonsense\nskipped-full=soon\n"), State::default());
    }
}
