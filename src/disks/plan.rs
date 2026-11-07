//! The Drives window's decisions, kept apart so they can be tested.

use std::collections::HashSet;

use crate::common::refused::{self, Refused};
use crate::toolkit::t;

pub const MIB: u64 = 1 << 20;
const GIB: u64 = 1 << 30;

pub const SYSTEM_MOUNTS: &[&str] = &[
    "/", "/boot", "/boot/efi", "/efi", "/home", "/usr", "/var",
    "/run/live/medium", "/lib/live/mount/medium",
];

#[derive(Debug, PartialEq)]
pub struct Mounted {
    pub dir: String,
    pub major_minor: String,
    pub source: String,
}

pub fn system_mounts(mountinfo: &str) -> Vec<Mounted> {
    mountinfo
        .lines()
        .filter_map(|l| {
            let (left, right) = l.split_once(" - ")?;
            let f: Vec<&str> = left.split(' ').collect();
            let dir = unescape(f.get(4)?);
            let source = right.split(' ').nth(1).unwrap_or("").to_string();
            SYSTEM_MOUNTS.contains(&dir.as_str())
                .then(|| Mounted { dir, major_minor: f.get(2).unwrap_or(&"").to_string(), source })
        })
        .collect()
}

pub fn swap_devices(swaps: &str) -> Vec<String> {
    swaps
        .lines()
        .skip(1)
        .filter_map(|l| {
            let mut w = l.split_whitespace();
            let (dev, kind) = (w.next()?, w.next()?);
            (kind == "partition").then(|| unescape(dev))
        })
        .collect()
}

pub fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 3 < b.len() && b[i + 1..i + 4].iter().all(|c| (b'0'..=b'7').contains(c)) {
            out.push((b[i + 1] - b'0') * 64 + (b[i + 2] - b'0') * 8 + (b[i + 3] - b'0'));
            i += 4;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

pub fn underneath(seeds: &[String], below: impl Fn(&str) -> Vec<String>) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut todo: Vec<String> = seeds.to_vec();
    while let Some(d) = todo.pop() {
        if d.is_empty() || !seen.insert(d.clone()) {
            continue;
        }
        todo.extend(below(&d));
    }
    seen
}

pub struct OnDrive<'a> {
    pub dev: &'a str,
    pub drive: &'a str,
}

pub fn system_drives(blocks: &[OnDrive], system_devs: &HashSet<String>, root_traced: bool,
                     internal: &[&str]) -> HashSet<String> {
    let mut out: HashSet<String> = blocks
        .iter()
        .filter(|b| !b.drive.is_empty() && system_devs.contains(b.dev))
        .map(|b| b.drive.to_string())
        .collect();
    if !root_traced || out.is_empty() {
        out.extend(internal.iter().map(|d| d.to_string()));
    }
    out
}

pub const MNT: &str = "/mnt";
const NAME_MAX: usize = 32;

pub fn check_name(name: &str, taken: &HashSet<String>) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(t!("disks-name-empty").into());
    }
    if name.len() > NAME_MAX {
        return Err(t!("disks-name-too-long", max = NAME_MAX));
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(t!("disks-name-letters").into());
    }
    if name.starts_with('-') {
        return Err(t!("disks-name-start").into());
    }
    let dir = format!("{MNT}/{name}");
    if taken.contains(&dir) {
        return Err(t!("disks-name-taken", dir = &dir));
    }
    Ok(dir)
}

pub fn suggest_name(label: &str, uuid: &str, taken: &HashSet<String>) -> String {
    let mut base: String = label
        .trim()
        .chars()
        .map(|c| if c == ' ' { '-' } else { c.to_ascii_lowercase() })
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    base = base.trim_start_matches('-').chars().take(NAME_MAX - 3).collect();
    if base.is_empty() {
        let short: String = uuid.chars().filter(|c| c.is_ascii_alphanumeric()).take(8).collect();
        base = if short.is_empty() { "drive".into() } else { format!("drive-{}", short.to_lowercase()) };
    }
    let mut name = base.clone();
    let mut n = 2;
    while taken.contains(&format!("{MNT}/{name}")) {
        name = format!("{base}-{n}");
        n += 1;
    }
    name
}

pub fn fstab_dirs(fstab: &str) -> Vec<String> {
    fstab
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.split_whitespace().nth(1).map(unescape))
        .collect()
}

pub fn name_of(dir: &str) -> String {
    dir.strip_prefix("/mnt/").filter(|n| !n.contains('/')).unwrap_or("").to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fstab {
    pub fsname: String,
    pub dir: String,
    pub kind: String,
    pub opts: String,
    pub freq: i32,
    pub passno: i32,
}

fn needs_owner(fs_type: &str) -> bool {
    matches!(fs_type, "vfat" | "exfat" | "ntfs" | "ntfs3" | "msdos")
}

const ALWAYS: &[&str] = &["nofail", "x-systemd.device-timeout=5s"];

fn fstab_type(fs_type: &str) -> &str {
    match fs_type {
        "" => "auto",
        "ntfs" => "ntfs3",
        t => t,
    }
}

pub fn new_entry(uuid: &str, dir: &str, fs_type: &str, startup: bool, uid: u32, gid: u32) -> Fstab {
    let mut opts: Vec<String> = ALWAYS.iter().map(|s| s.to_string()).collect();
    opts.extend(["x-gvfs-show", "users", "exec"].map(String::from));
    if needs_owner(fs_type) {
        opts.push(format!("uid={uid}"));
        opts.push(format!("gid={gid}"));
    }
    if !startup {
        opts.push("noauto".into());
    }
    Fstab { fsname: format!("UUID={uuid}"), dir: dir.into(), kind: fstab_type(fs_type).into(), opts: opts.join(","), freq: 0, passno: 0 }
}

pub fn with_startup(opts: &str, startup: bool) -> String {
    let mut out: Vec<String> = opts
        .split(',')
        .map(str::trim)
        .filter(|o| !o.is_empty() && *o != "noauto" && *o != "auto" && *o != "defaults")
        .map(String::from)
        .collect();
    for a in ALWAYS {
        let key = a.split('=').next().unwrap_or(a);
        if !out.iter().any(|o| o.split('=').next() == Some(key)) {
            out.insert(0, a.to_string());
        }
    }
    if !startup {
        out.push("noauto".into());
    }
    out.join(",")
}

pub fn starts_at_boot(opts: &str) -> bool {
    !opts.split(',').any(|o| o.trim() == "noauto")
}

pub fn entry(existing: Option<&Fstab>, uuid: &str, fs_type: &str, dir: &str, startup: bool, uid: u32, gid: u32) -> Fstab {
    match existing {
        Some(e) => Fstab { dir: dir.into(), opts: with_startup(&e.opts, startup), ..e.clone() },
        None => new_entry(uuid, dir, fs_type, startup, uid, gid),
    }
}

pub struct Kind {
    pub id: &'static str,
    pub name: &'static str,
    pub for_what: &'static str,
}

pub fn kinds() -> [Kind; 4] {
    [
        Kind { id: "exfat", name: "exFAT", for_what: t!("disks-kind-exfat-for") },
        Kind { id: "ext4", name: "ext4", for_what: t!("disks-kind-ext4-for") },
        Kind { id: "ntfs", name: "NTFS", for_what: t!("disks-kind-ntfs-for") },
        Kind { id: "vfat", name: "FAT", for_what: t!("disks-kind-vfat-for") },
    ]
}

pub fn label_max(fs_type: &str) -> usize {
    match fs_type {
        "vfat" | "msdos" => 11,
        "xfs" => 12,
        "exfat" => 15,
        "ext2" | "ext3" | "ext4" => 16,
        "ntfs" | "ntfs3" => 32,
        "btrfs" => 255,
        _ => 32,
    }
}

pub fn check_label(fs_type: &str, label: &str) -> Result<(), String> {
    let max = label_max(fs_type);
    if label.len() > max {
        return Err(t!("disks-label-too-long", max = max));
    }
    if matches!(fs_type, "vfat" | "msdos") && label.chars().any(|c| "*?.,;:/\\|+=<>[]\"".contains(c)) {
        return Err(t!("disks-label-fat-marks").into());
    }
    Ok(())
}

pub fn table_for(size: u64, fs_type: &str) -> &'static str {
    if size > 2 * 1024 * GIB || fs_type == "ext4" { "gpt" } else { "dos" }
}

const MIN_GAP: u64 = 16 * MIB;

fn align_up(n: u64) -> u64 {
    n.div_ceil(MIB) * MIB
}

pub fn gaps(disk: u64, table: &str, parts: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let end = if table == "gpt" { disk.saturating_sub(MIB) } else { disk };
    let mut parts = parts.to_vec();
    parts.sort();
    let mut out = Vec::new();
    let mut at = MIB;
    for (o, s) in parts {
        let from = align_up(at);
        if o > from && o - from >= MIN_GAP {
            out.push((from, o - from));
        }
        at = at.max(o + s);
    }
    let from = align_up(at);
    if end > from && end - from >= MIN_GAP {
        out.push((from, end - from));
    }
    out
}

pub fn room_after(offset: u64, size: u64, gaps: &[(u64, u64)]) -> u64 {
    let end = offset + size;
    gaps.iter()
        .find(|(o, _)| *o >= end && *o <= align_up(end))
        .map(|(o, s)| o + s - end)
        .unwrap_or(0)
}

pub const OFFLINE_SHRINK: u64 = 1 << 1;
pub const OFFLINE_GROW: u64 = 1 << 2;
pub const ONLINE_SHRINK: u64 = 1 << 3;
pub const ONLINE_GROW: u64 = 1 << 4;

pub fn resize_needs_unmount(mode: u64, mounted: bool, grow: bool) -> Result<bool, String> {
    let (online, offline) = if grow { (ONLINE_GROW, OFFLINE_GROW) } else { (ONLINE_SHRINK, OFFLINE_SHRINK) };
    let way = if grow { "larger" } else { "smaller" };
    match (mounted, mode & online != 0, mode & offline != 0) {
        (true, true, _) => Ok(false),
        (true, false, true) => Ok(true),
        (false, _, true) => Ok(false),
        (false, true, false) => Err(t!("disks-resize-mount-first", way = way)),
        _ => Err(t!("disks-resize-cannot", way = way)),
    }
}

pub fn resize_range(size: u64, used: Option<u64>, room: u64, mode: u64) -> (u64, u64) {
    let min = match used {
        Some(u) if mode & (OFFLINE_SHRINK | ONLINE_SHRINK) != 0 => align_up(u + (u / 10).max(256 * MIB)).min(size),
        _ => size,
    };
    let max = if mode & (OFFLINE_GROW | ONLINE_GROW) != 0 { size + room } else { size };
    (min, max)
}

#[derive(Default)]
pub struct Health {
    pub failing: bool,
    pub troubles: Vec<String>,
    pub kelvin: f64,
    pub hours: u64,
}

pub fn nvme_trouble(code: &str) -> Option<&'static str> {
    Some(match code {
        "spare" => t!("disks-trouble-spare"),
        "temperature" => t!("disks-trouble-temperature"),
        "degraded" => t!("disks-trouble-wearing-out"),
        "readonly" => t!("disks-trouble-readonly"),
        "volatile_mem" => t!("disks-trouble-volatile"),
        "pmr_readonly" => t!("disks-trouble-pmr"),
        _ => return None,
    })
}

pub fn health_parts(h: &Health) -> (String, String, bool) {
    let mut facts = Vec::new();
    if h.kelvin > 200.0 {
        facts.push(t!("disks-health-temperature", degrees = format!("{:.0}", h.kelvin - 273.15)));
    }
    if h.hours > 0 {
        facts.push(t!("disks-health-hours", n = h.hours, hours = crate::common::thousands(h.hours)));
    }
    let facts = facts.join(", ");
    if h.failing {
        return (t!("disks-health-failing").into(), facts, false);
    }
    if !h.troubles.is_empty() {
        return (t!("disks-health-attention", troubles = h.troubles.join(", ")), facts, false);
    }
    (t!("disks-health-ok").into(), facts, true)
}

pub fn health_line(h: &Health) -> (String, bool) {
    let (state, facts, ok) = health_parts(h);
    (if facts.is_empty() { state } else { t!("disks-health-line", state = state, readings = facts) }, ok)
}

pub const FREE: [&str; 4] = ["Mount", "Unmount", "Eject", "PowerOff"];

pub fn no_password() -> &'static str {
    t!("disks-no-password")
}
pub fn one_at_a_time() -> &'static str {
    t!("disks-one-at-a-time")
}
pub fn change_going() -> &'static str {
    t!("disks-change-going")
}
pub fn not_sure() -> &'static str {
    t!("disks-not-sure")
}
pub fn standing() -> &'static str {
    t!("disks-standing")
}
pub fn taken_back() -> &'static str {
    t!("disks-taken-back")
}

pub fn refusal_words(r: Refused) -> String {
    match r {
        Refused::Cancelled => no_password().into(),
        other => t!("disks-nothing-changed", reason = other.sentence()),
    }
}

fn refusal_part_way(why: &str) -> Option<String> {
    if why == no_password() {
        return Some(t!("disks-no-password-part-way").into());
    }
    if why == service_not_asked() {
        return Some(t!("disks-part-way", reason = t!("disks-service-not-asked")));
    }
    refused::ALL.iter().find(|r| **r != Refused::Cancelled && refusal_words(**r) == why).map(|r| t!("disks-part-way", reason = r.sentence()))
}

fn service_not_asked() -> String {
    t!("disks-nothing-changed", reason = t!("disks-service-not-asked"))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Did {
    pub what: String,
    pub changes: bool,
    pub mounts: Option<(String, bool)>,
}

pub fn step_words(kind: &str, method: &str, dev: &str) -> String {
    match (kind, method) {
        (_, "Unmount") => t!("disks-step-unmount", dev = dev),
        (_, "Mount") => t!("disks-step-mount", dev = dev),
        (_, "Eject") => t!("disks-step-eject").into(),
        (_, "PowerOff") => t!("disks-step-power-off").into(),
        (_, "Format") => t!("disks-step-format", dev = dev),
        (_, "CreatePartitionAndFormat") => t!("disks-step-create").into(),
        (_, "Delete") => t!("disks-step-delete", dev = dev),
        ("Partition", "Resize") => t!("disks-step-resize-part", dev = dev),
        (_, "Resize") => t!("disks-step-resize", dev = dev),
        (_, "Repair") => t!("disks-step-repair", dev = dev),
        (_, "Check") => t!("disks-step-check", dev = dev),
        (_, "SetLabel") => t!("disks-step-rename", dev = dev),
        (_, "AddConfigurationItem" | "UpdateConfigurationItem" | "RemoveConfigurationItem") => t!("disks-step-fstab", dev = dev),
        (_, other) => t!("disks-step-other", method = other, dev = dev),
    }
}

pub fn run_stopped(steps: &[Did], done: usize, stopped: &Stopped) -> String {
    let done = done.min(steps.len());
    if !steps[..done].iter().any(|s| s.changes) {
        return stopped.why.clone();
    }
    let list = listed;
    let (why, failed, left) = if stopped.refused {
        let why = refusal_part_way(&stopped.why).unwrap_or_else(|| t!("disks-part-way", reason = stopped.why.trim_end_matches('.')));
        (why, &steps[done..done], &steps[done..])
    } else {
        let next = (done + 1).min(steps.len());
        (t!("disks-failed-part-way", why = sentence(&stopped.why)), &steps[done..next], &steps[next..])
    };
    told_in_order(why, [list(Listed::Done, &steps[..done]), list(Listed::Failed, failed), list(Listed::NotDone, left)])
}

#[derive(Clone, Copy)]
enum Listed {
    Done,
    Failed,
    NotDone,
    NotKnown,
}

fn listed(name: Listed, of: &[Did]) -> String {
    if of.is_empty() {
        return String::new();
    }
    let steps = of.iter().map(|s| s.what.as_str()).collect::<Vec<_>>().join(", ");
    match name {
        Listed::Done => t!("disks-list-done", steps = steps),
        Listed::Failed => t!("disks-list-failed", steps = steps),
        Listed::NotDone => t!("disks-list-not-done", steps = steps),
        Listed::NotKnown => t!("disks-list-not-known", steps = steps),
    }
}

fn told_in_order(first: String, lists: impl IntoIterator<Item = String>) -> String {
    let mut words = first;
    for list in lists.into_iter().filter(|l| !l.is_empty()) {
        words.push(' ');
        words.push_str(&list);
    }
    words
}

pub fn no_service() -> &'static str {
    t!("disks-no-service")
}
pub fn closed() -> &'static str {
    t!("disks-closed")
}
pub fn stays_open() -> &'static str {
    t!("disks-stays-open")
}

pub fn left_unmounted(steps: &[Did], done: usize) -> Vec<String> {
    let mut left: Vec<String> = Vec::new();
    for step in steps.iter().take(done) {
        match &step.mounts {
            Some((dev, false)) if !left.contains(dev) => left.push(dev.clone()),
            Some((dev, true)) => left.retain(|d| d != dev),
            _ => {}
        }
    }
    left
}

pub fn run_summed(steps: &[Did], done: usize, stopped: Option<&Stopped>, back: &[(String, Option<Stopped>)]) -> String {
    let done = done.min(steps.len());
    let mut words = match stopped {
        Some(stopped) => sentence(&run_stopped(steps, done, stopped)),
        None => {
            let next = (done + 1).min(steps.len());
            told_in_order(closed().to_string(), [listed(Listed::Done, &steps[..done]), listed(Listed::NotKnown, &steps[done..next]), listed(Listed::NotDone, &steps[next..])])
        }
    };
    let midway = stopped.is_none().then(|| steps.get(done)).flatten();
    let did_nothing = stopped.is_some_and(|s| s.refused);
    let changed = steps[..done].iter().any(|s| s.changes) || (!did_nothing && steps.get(done).is_some_and(|s| s.changes));
    for dev in left_unmounted(steps, done) {
        let state = match back.iter().find(|(tried, _)| *tried == dev) {
            Some((_, None)) => continue,
            Some((_, Some(why))) => not_put_back(&dev, why),
            None if changed => t!("disks-left-unmounted", dev = &dev),
            None => t!("disks-still-unmounted", dev = &dev),
        };
        words = format!("{words} {state}");
    }
    if let Some((dev, false)) = midway.and_then(|s| s.mounts.as_ref()) {
        words = format!("{words} {}", t!("disks-may-be-unmounted", dev = dev));
    }
    words
}

pub fn noted(drive: &str, row: &str, steps: &[Did], done: usize) -> String {
    let plain = |words: &str| words.replace(['\n', '\t'], " ");
    let mut text = format!("1\t{done}\n{}\n{}\n", plain(drive), plain(row));
    for step in steps {
        let mounts = match &step.mounts {
            None => "-".to_string(),
            Some((dev, mounts)) => format!("{}{}", if *mounts { "m" } else { "u" }, plain(dev)),
        };
        text.push_str(&format!("{}\t{mounts}\t{}\n", u8::from(step.changes), plain(&step.what)));
    }
    text
}

pub fn read_noted(text: &str) -> Option<(String, String, Vec<Did>, usize)> {
    let mut lines = text.lines();
    let done: usize = lines.next()?.strip_prefix("1\t")?.parse().ok()?;
    let (drive, row) = (lines.next()?.to_string(), lines.next()?.to_string());
    let mut steps = Vec::new();
    for line in lines {
        let mut parts = line.splitn(3, '\t');
        let (changes, mounts, what) = (parts.next()?, parts.next()?, parts.next()?);
        let mounts = match mounts.split_at_checked(1)? {
            ("-", "") => None,
            ("u", dev) if !dev.is_empty() => Some((dev.to_string(), false)),
            ("m", dev) if !dev.is_empty() => Some((dev.to_string(), true)),
            _ => return None,
        };
        steps.push(Did { what: what.to_string(), changes: match changes { "0" => false, "1" => true, _ => return None }, mounts });
    }
    (!steps.is_empty() && done <= steps.len()).then_some((drive, row, steps, done))
}

pub fn puts_back(steps: &[Did], done: usize, stopped: &Stopped) -> bool {
    stopped.refused && !steps.iter().take(done).any(|s| s.changes)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stopped {
    pub why: String,
    pub refused: bool,
}

pub fn stopped(name: &str, text: &str, asks: impl FnOnce() -> Option<bool>) -> Stopped {
    if never_begun(name) {
        return Stopped { why: service_not_asked(), refused: true };
    }
    match refused::heard_udisks(name, text).filter(|_| !busy(text)) {
        Some(heard) => Stopped { why: refusal_words(heard.settled(asks)), refused: true },
        None => Stopped { why: plain_error(if text.is_empty() { name } else { text }), refused: false },
    }
}

fn never_begun(name: &str) -> bool {
    let kind = name.strip_prefix("org.freedesktop.DBus.Error.").unwrap_or("");
    matches!(kind, "ServiceUnknown" | "NameHasNoOwner" | "UnknownMethod" | "UnknownObject" | "UnknownInterface" | "AccessDenied")
        || kind.starts_with("Spawn.")
}

pub fn standing_words(said: &str) -> String {
    format!("{} {}", sentence(said), standing())
}

pub fn taken_back_words(said: &str) -> String {
    format!("{} {}", sentence(said), taken_back())
}

pub fn sentence(words: &str) -> String {
    let words = words.trim();
    if words.ends_with(['.', '!', '?']) { words.to_string() } else { t!("disks-sentence", words = words) }
}

pub fn not_put_back(dev: &str, stopped: &Stopped) -> String {
    if stopped.refused {
        t!("disks-put-back-password", dev = dev)
    } else {
        t!("disks-put-back-failed", dev = dev, why = stopped.why.trim_end_matches('.'))
    }
}

pub fn folder_not_kept(dir: &str, why: &str) -> String {
    t!("disks-folder-not-kept", dir = dir, why = sentence(why))
}

fn busy(e: &str) -> bool {
    let l = e.to_lowercase();
    l.contains("target is busy") || l.contains("device is busy")
}

pub fn plain_error(e: &str) -> String {
    let l = e.to_lowercase();
    if busy(e) {
        t!("disks-busy").into()
    } else if let Some(heard) = refused::heard_udisks("", e) {
        refusal_words(heard.settled(|| None))
    } else if l.contains("no space left") || l.contains("too small") {
        t!("disks-no-room").into()
    } else {
        let e = e.trim();
        e.rsplit_once("exit code ").and_then(|(_, rest)| rest.split_once(": ")).map(|(_, why)| why.trim())
            .unwrap_or(e).to_string()
    }
}

pub fn label_online(fs_type: &str) -> bool {
    matches!(fs_type, "ext2" | "ext3" | "ext4" | "btrfs")
}

pub fn size_text(n: f64) -> String {
    trindle_toolkit::lang::format::size_exact(n.max(0.0) as u64)
}

pub fn gb_text(gb: impl Into<f64>) -> String {
    crate::toolkit::t!("kit-size-gb", size = trindle_toolkit::lang::format::decimal(gb.into(), 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(v: &[&str]) -> HashSet<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn names() {
        let taken = set(&["/mnt/usb"]);
        assert_eq!(check_name("backup", &taken), Ok("/mnt/backup".into()));
        assert_eq!(check_name(" Photos_2 ", &taken), Ok("/mnt/Photos_2".into()));
        assert!(check_name("", &taken).is_err());
        assert!(check_name("usb", &taken).is_err());
        assert!(check_name("my drive", &taken).is_err());
        assert!(check_name("../etc", &taken).is_err());
        assert!(check_name("a/b", &taken).is_err());
        assert!(check_name("-o", &taken).is_err());
        assert!(check_name("..", &taken).is_err());
        assert!(check_name("é", &taken).is_err());
        assert!(check_name(&"x".repeat(33), &taken).is_err());
        assert!(check_name(&"x".repeat(32), &taken).is_ok());
    }

    #[test]
    fn suggested_names() {
        let taken = set(&["/mnt/backup", "/mnt/backup-2"]);
        assert_eq!(suggest_name("Backup", "", &taken), "backup-3");
        assert_eq!(suggest_name("My Photos!", "", &taken), "my-photos");
        assert_eq!(suggest_name("", "1234-ABCD", &taken), "drive-1234abcd");
        assert_eq!(suggest_name("--", "", &taken), "drive");
        assert!(check_name(&suggest_name(&"é".repeat(40), "5", &taken), &taken).is_ok());
    }

    #[test]
    fn fstab_lines() {
        let e = new_entry("1234-ABCD", "/mnt/backup", "exfat", true, 1000, 1000);
        assert_eq!(e.fsname, "UUID=1234-ABCD");
        assert_eq!(e.kind, "exfat");
        assert_eq!(new_entry("u", "/mnt/w", "ntfs", true, 1000, 1000).kind, "ntfs3");
        assert_eq!(new_entry("u", "/mnt/w", "", true, 1000, 1000).kind, "auto");
        assert_eq!(e.opts, "nofail,x-systemd.device-timeout=5s,x-gvfs-show,users,exec,uid=1000,gid=1000");
        assert_eq!((e.freq, e.passno), (0, 0));
        let off = new_entry("u", "/mnt/b", "ext4", false, 1000, 1000);
        assert_eq!(off.opts, "nofail,x-systemd.device-timeout=5s,x-gvfs-show,users,exec,noauto");
        assert!(!starts_at_boot(&off.opts));
        assert!(starts_at_boot(&e.opts));

        let theirs = Fstab { fsname: "UUID=u".into(), dir: "/data".into(), kind: "ext4".into(),
                             opts: "defaults,noatime".into(), freq: 0, passno: 2 };
        let moved = entry(Some(&theirs), "u", "ext4", "/mnt/data", false, 1000, 1000);
        assert_eq!(moved.dir, "/mnt/data");
        assert_eq!(moved.kind, "ext4");
        assert_eq!(moved.passno, 2);
        assert_eq!(moved.opts, "x-systemd.device-timeout=5s,nofail,noatime,noauto");
        let back = entry(Some(&moved), "u", "ext4", "/mnt/data", true, 1000, 1000);
        assert_eq!(back.opts, "x-systemd.device-timeout=5s,nofail,noatime");
        assert!(starts_at_boot(&back.opts));
        let fresh = entry(None, "u", "ext4", "/mnt/x", true, 1000, 1000);
        assert_eq!(fresh, new_entry("u", "/mnt/x", "ext4", true, 1000, 1000));
    }

    #[test]
    fn fstab_folders() {
        let t = "# a comment\nUUID=1 / btrfs defaults 0 0\n\n/dev/sdb1 /mnt/my\\040drive auto noauto 0 0\n";
        assert_eq!(fstab_dirs(t), vec!["/", "/mnt/my drive"]);
        assert_eq!(name_of("/mnt/backup"), "backup");
        assert_eq!(name_of("/data"), "");
        assert_eq!(name_of("/mnt/a/b"), "");
    }

    #[test]
    fn system_is_found() {
        let info = "22 1 0:25 /@ / rw,relatime shared:1 - btrfs /dev/nvme0n1p2 rw\n\
                    23 22 259:1 / /boot/efi rw shared:2 - vfat /dev/nvme0n1p1 rw\n\
                    24 22 0:25 /@home /home rw shared:3 - btrfs /dev/nvme0n1p2 rw\n\
                    40 22 8:17 / /media/john/USB rw shared:9 - exfat /dev/sdb1 rw\n\
                    41 22 8:33 / /mnt/backup rw shared:9 - ext4 /dev/sdc1 rw\n";
        let m = system_mounts(info);
        assert_eq!(m.iter().map(|m| m.dir.as_str()).collect::<Vec<_>>(), vec!["/", "/boot/efi", "/home"]);
        assert_eq!(m[0].source, "/dev/nvme0n1p2");
        assert_eq!(m[1].major_minor, "259:1");
        assert_eq!(swap_devices("Filename Type Size Used Priority\n/swap/swapfile file 8 0 -2\n/dev/sda3 partition 8 0 -3\n"),
                   vec!["/dev/sda3"]);

        let below = |d: &str| match d {
            "dm-0" => vec!["nvme0n1p3".to_string()],
            _ => vec![],
        };
        let devs = underneath(&["dm-0".into(), "sda3".into()], below);
        assert_eq!(devs, set(&["dm-0", "nvme0n1p3", "sda3"]));
        let blocks = [
            OnDrive { dev: "dm-0", drive: "" },
            OnDrive { dev: "nvme0n1", drive: "/nvme" },
            OnDrive { dev: "nvme0n1p3", drive: "/nvme" },
            OnDrive { dev: "sda", drive: "/sata" },
            OnDrive { dev: "sda3", drive: "/sata" },
            OnDrive { dev: "sdb1", drive: "/usb" },
        ];
        assert_eq!(system_drives(&blocks, &devs, true, &["/nvme", "/sata", "/other"]), set(&["/nvme", "/sata"]));
        assert_eq!(system_drives(&blocks, &devs, false, &["/nvme", "/other"]), set(&["/nvme", "/sata", "/other"]));
        assert_eq!(system_drives(&blocks, &HashSet::new(), true, &["/other"]), set(&["/other"]));
    }

    #[test]
    fn labels_and_tables() {
        assert!(check_label("vfat", "BACKUP").is_ok());
        assert!(check_label("vfat", "TWELVE_CHARS").is_err());
        assert!(check_label("vfat", "A.B").is_err());
        assert!(check_label("exfat", "Fifteen letters").is_ok());
        assert!(check_label("ext4", "sixteen letters!").is_ok());
        assert!(check_label("ext4", "seventeen letters").is_err());
        assert_eq!(table_for(32_000_000_000, "exfat"), "dos");
        assert_eq!(table_for(32_000_000_000, "ext4"), "gpt");
        assert_eq!(table_for(4_000_000_000_000, "exfat"), "gpt");
    }

    #[test]
    fn free_space() {
        let disk = 4096 * MIB;
        let g = gaps(disk, "gpt", &[(1024 * MIB, 1024 * MIB)]);
        assert_eq!(g, vec![(MIB, 1023 * MIB), (2048 * MIB, 2047 * MIB)]);
        assert_eq!(room_after(1024 * MIB, 1024 * MIB, &g), 2047 * MIB);
        assert_eq!(room_after(MIB, 1023 * MIB, &g), 0);
        assert!(gaps(disk, "gpt", &[(MIB, 4094 * MIB)]).is_empty());
        assert!(gaps(disk, "dos", &[(MIB, 4094 * MIB)]).is_empty());
        assert_eq!(gaps(disk, "dos", &[(MIB, 4000 * MIB)]), vec![(4001 * MIB, 95 * MIB)]);
        assert!(gaps(disk, "gpt", &[(MIB, 1000 * MIB), (1001 * MIB + 4096, 3093 * MIB)]).is_empty());
        assert_eq!(gaps(disk, "dos", &[(MIB, 2048 * MIB), (2 * MIB, 100 * MIB)]), vec![(2049 * MIB, 2047 * MIB)]);
        assert_eq!(gaps(disk, "dos", &[]), vec![(MIB, 4095 * MIB)]);
    }

    #[test]
    fn resizing() {
        let ext4 = OFFLINE_SHRINK | OFFLINE_GROW | ONLINE_GROW;
        let btrfs = ONLINE_SHRINK | ONLINE_GROW;
        let ntfs = OFFLINE_SHRINK | OFFLINE_GROW;
        assert_eq!(resize_needs_unmount(ext4, true, true), Ok(false));
        assert_eq!(resize_needs_unmount(ext4, true, false), Ok(true));
        assert_eq!(resize_needs_unmount(ext4, false, false), Ok(false));
        assert_eq!(resize_needs_unmount(ntfs, true, true), Ok(true));
        assert!(resize_needs_unmount(btrfs, false, true).is_err());
        assert_eq!(resize_needs_unmount(btrfs, true, false), Ok(false));
        assert!(resize_needs_unmount(ONLINE_GROW, true, false).is_err());

        let gb = 1000 * MIB;
        let (min, max) = resize_range(10 * gb, Some(4 * gb), 5 * gb, ext4);
        assert!(min >= 4 * gb + 4 * gb / 10 && min < 5 * gb);
        assert_eq!(max, 15 * gb);
        assert_eq!(resize_range(10 * gb, Some(MIB), 0, ext4).0, align_up(257 * MIB));
        assert_eq!(resize_range(10 * gb, None, 5 * gb, ext4), (10 * gb, 15 * gb));
        assert_eq!(resize_range(10 * gb, Some(gb), 0, ONLINE_GROW), (10 * gb, 10 * gb));
        assert_eq!(resize_range(10 * gb, Some(10 * gb), 0, ext4).0, 10 * gb);
    }

    #[test]
    fn health() {
        let h = Health { kelvin: 311.15, hours: 2140, ..Default::default() };
        assert_eq!(health_line(&h), ("Healthy, 38 °C, used for 2,140 hours".into(), true));
        assert_eq!(health_line(&Health::default()), ("Healthy".into(), true));
        let bad = Health { failing: true, kelvin: 0.0, hours: 1, ..Default::default() };
        assert_eq!(health_line(&bad), ("Failing: copy anything you want to keep off it now, used for 1 hour".into(), false));
        let worn = Health { troubles: vec![nvme_trouble("spare").unwrap().into(), "too hot".into()], ..Default::default() };
        assert_eq!(health_line(&worn), ("Needs attention: running out of spare space, too hot".into(), false));
        assert_eq!(nvme_trouble("unknown"), None);
        assert_eq!(crate::common::thousands(1234567), "1,234,567");
        assert_eq!(crate::common::thousands(999), "999");
    }

    #[test]
    fn errors_in_words() {
        assert!(plain_error("Error unmounting /dev/sdb1: target is busy").starts_with("Something is still using it"));
        assert_eq!(plain_error("Not authorized to perform operation"), refusal_words(Refused::Unclear));
        assert_eq!(plain_error("The authentication dialog was dismissed"), no_password());
        assert_eq!(plain_error("Something odd\n"), "Something odd");
        assert_eq!(plain_error("Error setting label: Process reported exit code 1: Cannot make changes to a mounted device\n\n"),
                   "Cannot make changes to a mounted device");
    }

    #[test]
    fn a_closed_box_a_wrong_password_and_not_allowed_are_said_differently() {
        let udisks = "org.freedesktop.UDisks2.Error.";
        let unasked = || -> Option<bool> { panic!("polkit is asked only about \"Not authorized\"") };
        let closed = stopped(&format!("{udisks}NotAuthorizedDismissed"), "The authentication dialog was dismissed", unasked);
        assert_eq!(closed, Stopped { why: no_password().into(), refused: true });
        let refusal = (format!("{udisks}NotAuthorized"), "Not authorized to perform operation");
        let wrong = stopped(&refusal.0, refusal.1, || Some(true));
        assert_eq!(wrong, Stopped { why: "The administrator's password was not accepted, so nothing changed.".into(), refused: true });
        let not_allowed = stopped(&refusal.0, refusal.1, || Some(false));
        assert_eq!(not_allowed.why, "This login is not allowed to do that, whatever password is given, so nothing changed.");
        let unclear = stopped(&refusal.0, refusal.1, || None);
        assert_eq!(unclear.why, "The administrator's password was not accepted, or this login is not allowed to do that, so nothing changed.");
        let nothing_asked = stopped(&format!("{udisks}NotAuthorizedCanObtain"), "Not authorized to perform operation", unasked);
        assert_eq!(nothing_asked.why, "Nothing was there to ask for the administrator's password, so nothing changed.");
        let mut all = vec![closed.why, wrong.why, not_allowed.why, unclear.why, nothing_asked.why];
        for words in &all[1..] {
            assert!(!words.contains("No password was given"), "{words}");
        }
        for words in &all {
            assert!(words.ends_with(", so nothing changed.") && !words.contains('\u{2014}'), "{words}");
        }
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 5);
        for r in crate::common::refused::ALL {
            assert!(all.contains(&refusal_words(r)), "{r:?}");
        }
        let busy = stopped(&format!("{udisks}DeviceBusy"), "Error unmounting /dev/sdb1: target is busy", unasked);
        assert!(!busy.refused && busy.why.starts_with("Something is still using it"));
        assert_eq!(stopped(&format!("{udisks}Failed"), "", unasked), Stopped { why: format!("{udisks}Failed"), refused: false });
    }

    fn summed_runs() -> [Vec<Did>; 5] {
        let did = |what: &str, changes: bool| Did { what: what.into(), changes, mounts: None };
        [
            vec![did("formatting sdb1", true)],
            vec![did("erasing the whole drive", true), did("making the new part", true)],
            vec![did("unmounting sdb1", false), did("unmounting sdb2", false), did("erasing the whole drive", true), did("making the new part", true)],
            vec![did("unmounting sdb1", false), did("repairing sdb1", true), did("resizing the part sdb1", true),
                 did("resizing what is on sdb1", true), did("mounting sdb1 again", false)],
            vec![did("unmounting sdb1", false), did("powering the drive off", false)],
        ]
    }

    fn check_summed_case(steps: &[Did], at: usize, stop: &Stopped) {
        let words = run_stopped(steps, at, stop);
        let case = format!("{} steps, stopped at {at}: {words}", steps.len());
        let changed = steps[..at].iter().any(|s| s.changes);
        assert_eq!(puts_back(steps, at, stop), stop.refused && !changed, "{case}");
        assert!(!words.contains('\u{2014}'), "{case}");
        if !changed {
            assert_eq!(words, stop.why, "{case}");
            assert_eq!(words.contains("nothing changed"), stop.refused, "{case}");
            return;
        }
        assert!(!words.contains("nothing changed"), "{case}");
        assert!(words.contains("stopped part-way"), "{case}");
        let reason = stop.why.strip_suffix(", so nothing changed.").unwrap_or(stop.why.trim_end_matches('.'));
        assert!(words.starts_with(reason), "{case}");
        let under = |list: &str| -> Vec<String> {
            let Some((_, rest)) = words.split_once(&format!(" {list}: ")) else { return Vec::new() };
            let end = [" Done: ", " Failed: ", " Not done: "].iter().filter_map(|next| rest.find(next)).min().unwrap_or(rest.len());
            rest[..end].trim_end_matches('.').split(", ").map(String::from).collect()
        };
        let names = |of: &[Did]| -> Vec<String> { of.iter().map(|s| s.what.clone()).collect() };
        let failed = if stop.refused { at } else { at + 1 };
        assert_eq!(under("Done"), names(&steps[..at]), "{case}");
        assert_eq!(under("Failed"), names(&steps[at..failed]), "{case}");
        assert_eq!(under("Not done"), names(&steps[failed..]), "{case}");
        for step in steps {
            assert_eq!(words.matches(step.what.as_str()).count(), 1, "{case}");
        }
    }

    #[test]
    fn a_run_that_stops_is_summed_up_by_what_was_done_at_every_step() {
        let runs = summed_runs();
        let mut stops: Vec<Stopped> = crate::common::refused::ALL.iter().map(|r| Stopped { why: refusal_words(*r), refused: true }).collect();
        stops.push(Stopped { why: "Example reason".into(), refused: false });
        stops.push(Stopped { why: "Something is still using it. Close any windows or programs showing its files, then try again.".into(), refused: false });
        for steps in &runs {
            for at in 0..steps.len() {
                for stop in &stops {
                    check_summed_case(steps, at, stop);
                }
            }
        }
        check_the_wording(&runs, &stops);
    }

    fn unmounted_runs() -> [Vec<Did>; 6] {
        let did = |what: &str, changes: bool, mounts: Option<(&str, bool)>| Did { what: what.into(), changes, mounts: mounts.map(|(d, m)| (d.to_string(), m)) };
        [
            vec![did("unmounting sdb1", false, Some(("sdb1", false))), did("deleting sdb1", true, None)],
            vec![did("unmounting sdb1", false, Some(("sdb1", false))), did("renaming sdb1", true, None), did("mounting sdb1 again", false, Some(("sdb1", true)))],
            vec![did("unmounting sdb1", false, Some(("sdb1", false))), did("unmounting sdb2", false, Some(("sdb2", false))),
                 did("erasing the whole drive", true, None), did("making the new part", true, None)],
            vec![did("unmounting sdb1", false, Some(("sdb1", false))), did("repairing sdb1", true, None), did("resizing the part sdb1", true, None),
                 did("resizing what is on sdb1", true, None), did("mounting sdb1 again", false, Some(("sdb1", true)))],
            vec![did("unmounting sdb1", false, Some(("sdb1", false))), did("unmounting sdb2", false, Some(("sdb2", false))),
                 did("powering the drive off", false, None)],
            vec![did("formatting sdb1", true, None)],
        ]
    }

    fn check_the_wording(runs: &[Vec<Did>; 5], stops: &[Stopped]) {
        let erase = &runs[1];
        assert_eq!(run_stopped(erase, 1, &stops[0]),
                   "No password was given, so the work stopped part-way. Done: erasing the whole drive. Not done: making the new part.");
        assert_eq!(run_stopped(erase, 0, &stops[0]), no_password());
        assert_eq!(run_stopped(erase, 1, &stops[5]),
                   "Example reason. The work stopped part-way. Done: erasing the whole drive. Failed: making the new part.");
        for (kind, method) in [("Filesystem", "Unmount"), ("Filesystem", "Mount"), ("Drive", "Eject"), ("Drive", "PowerOff"), ("Block", "Format"),
                               ("PartitionTable", "CreatePartitionAndFormat"), ("Partition", "Delete"), ("Partition", "Resize"),
                               ("Filesystem", "Resize"), ("Filesystem", "Repair"), ("Filesystem", "Check"), ("Filesystem", "SetLabel"),
                               ("Block", "AddConfigurationItem"), ("Block", "UpdateConfigurationItem"), ("Block", "RemoveConfigurationItem")] {
            let words = step_words(kind, method, "sdb1");
            assert!(!words.contains(method) && !words.starts_with("the step"), "{method}: {words}");
        }
        assert_ne!(step_words("Partition", "Resize", "sdb1"), step_words("Filesystem", "Resize", "sdb1"));
        assert_eq!(step_words("Block", "SomethingNew", "sdb1"), "the step SomethingNew on sdb1", "a step with no words is still named");
    }

    fn check_back_words(steps: &[Did], at: usize, stop: &Option<Stopped>, left: &[&str], back: &[(String, Option<Stopped>)], cannot: &Stopped, seen: &mut i32) {
        let changed = steps[..at].iter().any(|s| s.changes);
        let words = run_summed(steps, at, stop.as_ref(), back);
        let case = format!("{} steps, stopped at {at} by {stop:?}: {words}", steps.len());
        *seen += 1;
        assert!(!words.contains('\u{2014}') && words.ends_with('.') && !words.contains(".."), "{case}");
        match stop {
            Some(stop) => assert!(words.starts_with(run_stopped(steps, at, stop).trim_end_matches('.')), "{case}"),
            None => assert!(words.starts_with(closed()) && !words.contains("nothing changed"), "{case}"),
        }
        for dev in ["sdb1", "sdb2"] {
            let said = words.matches(&format!("{dev} was unmounted for this")).count();
            let mounted_again = back.iter().any(|(d, why)| d == dev && why.is_none());
            let want = usize::from(left.contains(&dev) && !mounted_again);
            assert_eq!(said, want, "{case}: {dev} is named {said} times and is {}left unmounted", if want == 1 { "" } else { "not " });
            if back.iter().any(|(d, why)| d == dev && why.is_some()) {
                assert!(words.contains(&not_put_back(dev, cannot)), "{case}");
            } else if want == 1 {
                let may_have = stop.as_ref().is_none_or(|s| !s.refused) && steps[at].changes;
                let state = if changed || may_have { format!("{dev} was unmounted for this and is not mounted now.") }
                            else { format!("{dev} was unmounted for this and is still unmounted: mount it from its row.") };
                assert!(words.contains(&state), "{case}");
            }
        }
        if stop.is_none() {
            for (n, step) in steps.iter().enumerate() {
                let list = match n.cmp(&at) { std::cmp::Ordering::Less => " Done: ", std::cmp::Ordering::Equal => " Not known: ", std::cmp::Ordering::Greater => " Not done: " };
                let after = words.split_once(list).map_or("", |(_, rest)| rest);
                let end = after.find(". ").unwrap_or(after.len());
                assert!(after[..end].contains(step.what.as_str()), "{case}: {} is not under{list}", step.what);
                assert_eq!(words.matches(step.what.as_str()).count(), 1, "{case}");
            }
            let unmounting = matches!(&steps[at].mounts, Some((_, false)));
            assert_eq!(words.contains("may have been unmounted"), unmounting, "{case}");
        }
    }

    fn check_unmounted_case(steps: &[Did], at: usize, stop: &Option<Stopped>, left: &[&str], cannot: &Stopped, seen: &mut i32) {
        let changed = steps[..at].iter().any(|s| s.changes);
        let again = stop.as_ref().is_some_and(|s| puts_back(steps, at, s));
        assert_eq!(again, stop.as_ref().is_some_and(|s| s.refused) && !changed, "mounted again only when the stop did nothing and nothing changed");
        let all_back: Vec<(String, Option<Stopped>)> = if again { left.iter().map(|d| (d.to_string(), None)).collect() } else { Vec::new() };
        let mut one_not = all_back.clone();
        if let Some(first) = one_not.first_mut() {
            first.1 = Some(cannot.clone());
        }
        for back in [&all_back, &one_not] {
            check_back_words(steps, at, stop, left, back, cannot, seen);
        }
    }

    fn check_unmounted_tail(runs: &[Vec<Did>; 6], seen: i32) {
        assert!(seen > 300, "{seen}");
        let never = stopped("org.freedesktop.DBus.Error.UnknownMethod", "No such method", || None);
        assert!(never.refused && never.why.ends_with(", so nothing changed."), "{never:?}");
        assert_eq!(run_summed(&runs[2], 3, Some(&never), &[]),
                   "The drive service could not be asked, so the work stopped part-way. Done: unmounting sdb1, unmounting sdb2, erasing the whole drive. \
                    Not done: making the new part. sdb1 was unmounted for this and is not mounted now. sdb2 was unmounted for this and is not mounted now.");
        assert!(!stopped("org.freedesktop.UDisks2.Error.Failed", "Example reason", || None).refused);
        let failed = Stopped { why: "Example reason".into(), refused: false };
        assert_eq!(run_summed(&runs[1], 1, Some(&failed), &[]),
                   "Example reason. sdb1 was unmounted for this and is not mounted now.");
        let busy = Stopped { why: "Something is still using it.".into(), refused: false };
        assert_eq!(run_summed(&runs[2], 1, Some(&busy), &[]),
                   "Something is still using it. sdb1 was unmounted for this and is still unmounted: mount it from its row.");
        assert_eq!(run_summed(&runs[1], 1, None, &[]),
                   "Drives was closed while this change was being made. Done: unmounting sdb1. Not known: renaming sdb1. Not done: mounting sdb1 again. \
                    sdb1 was unmounted for this and is not mounted now.");
        for not in ["", "1\t0\n/d\n", "2\t0\n/d\n\n0\t-\tx\n", "1\t5\n/d\n\n0\t-\tx\n", "1\t0\n/d\n\n0\tq\tx\n", "1\tmany\n/d\n\n0\t-\tx\n"] {
            assert_eq!(read_noted(not), None, "{not:?}");
        }
    }

    #[test]
    fn every_run_that_stops_says_what_state_each_part_it_unmounted_is_left_in() {
        let runs = unmounted_runs();
        let mut stops: Vec<Option<Stopped>> = crate::common::refused::ALL.iter().map(|r| Some(Stopped { why: refusal_words(*r), refused: true })).collect();
        stops.push(Some(stopped("org.freedesktop.DBus.Error.ServiceUnknown", "The name is not there", || None)));
        stops.push(Some(Stopped { why: "Example reason".into(), refused: false }));
        stops.push(Some(Stopped { why: "Something is still using it. Close any windows or programs showing its files, then try again.".into(), refused: false }));
        stops.push(None);
        let cannot = Stopped { why: "Example trouble.".into(), refused: false };
        let mut seen = 0;
        for steps in &runs {
            for at in 0..steps.len() {
                let mut left: Vec<&str> = Vec::new();
                for step in &steps[..at] {
                    match &step.mounts {
                        Some((dev, false)) => left.push(dev.as_str()),
                        Some((dev, true)) => left.retain(|d| *d != dev.as_str()),
                        None => {}
                    }
                }
                assert_eq!(left_unmounted(steps, at), left);
                for stop in &stops {
                    check_unmounted_case(steps, at, stop, &left, &cannot, &mut seen);
                }
                assert_eq!(read_noted(&noted("/d/usb", "/b/Photos:name", steps, at)), Some(("/d/usb".to_string(), "/b/Photos:name".to_string(), steps.clone(), at)));
            }
        }
        check_unmounted_tail(&runs, seen);
    }

    #[test]
    fn what_is_left_undone_after_a_change_is_said_in_full() {
        assert_eq!(standing_words("Formatted"), format!("Formatted. {}", standing()));
        assert_eq!(standing_words(no_password()), format!("{} {}", no_password(), standing()));
        assert_eq!(taken_back_words("Formatted"), format!("Formatted. {}", taken_back()));
        assert!(standing().contains("could not be taken back") && standing().contains("for a few minutes") && standing().contains("tried again"));
        let needs = Stopped { why: refusal_words(Refused::Unasked), refused: true };
        assert_eq!(not_put_back("sdb1", &needs), "sdb1 was unmounted for this, and mounting it again needs a password of its own: mount it from its row.");
        let failed = Stopped { why: "Something odd.".into(), refused: false };
        assert_eq!(not_put_back("sdb1", &failed), "sdb1 was unmounted for this and could not be mounted again (Something odd): mount it from its row.");
        assert_eq!(folder_not_kept("/mnt/example", one_at_a_time()),
                   format!("Formatted, but it was not given its folder /mnt/example again. {} Choose the folder again from its row.", one_at_a_time()));
        for words in [standing(), taken_back(), &folder_not_kept("/mnt/example", "No reason"), &not_put_back("sdb1", &failed)] {
            assert!(!words.contains('\u{2014}') && words.ends_with('.'), "{words}");
        }
    }

    #[test]
    fn unescaping() {
        assert_eq!(unescape("/mnt/a\\040b"), "/mnt/a b");
        assert_eq!(unescape("/x\\"), "/x\\");
        assert_eq!(unescape("/x\\04"), "/x\\04");
    }

    #[test]
    fn exactly_one_byte_is_not_plural() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        assert_eq!((size_text(0.0), size_text(1.0), size_text(2.0)), ("0 bytes".to_string(), "1 byte".to_string(), "2 bytes".to_string()));
        let made_up = "kit-size-bytes =\n    { $n ->\n        [one] { $n } bajto\n       *[other] { $n } bajtoj\n    }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!((size_text(1.0), size_text(812.0)), ("1 bajto".to_string(), "812 bajtoj".to_string()));
        });
    }

    #[test]
    fn sizes_follow_the_languages_marks_and_units() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        assert_eq!((size_text(812.0), size_text(4_500.0), size_text(2e12), gb_text(12.5f32)), ("812 bytes".to_string(), "4.5 KB".to_string(), "2.0 TB".to_string(), "12.5 GB".to_string()));
        let made_up = "kit-number-decimal = ,\nkit-number-group = _\nkit-size-gb = GO { $size }\nkit-size-tb = TO { $size }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(size_text(499e9), "GO 499,0");
            assert_eq!(size_text(2e12), "TO 2,0");
            assert_eq!(gb_text(12.5f32), "GO 12,5");
            assert_eq!(crate::common::thousands(1_234_567), "1_234_567");
        });
    }
}
