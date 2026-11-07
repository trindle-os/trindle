//! Making room for a new part under a Btrfs part that is in use.

use std::rc::Rc;
use std::time::Duration;

use super::plan::{self, MIB};
use super::model::Vol;
use super::steps::{step, Step};
use super::udisks::{mount_points, num, parse_objects, used_space, Objects, Opts, BLOCK, FILESYSTEM, OBJECT_MANAGER, PARTITION, UD, UD_PATH};
use crate::toolkit::bus as dbus;
use crate::toolkit::bus::Failed;
use crate::toolkit::t;

const GIB: u64 = 1 << 30;
pub const ROOM_LEAST: u64 = GIB;
const SPARE_LEAST: u64 = 2 * GIB;
const HEADER_LEAST: u64 = MIB;
const HEADER_MOST: u64 = 64 * MIB;
const FS_SLACK: u64 = 4 * MIB;

pub const ENCRYPTED: &str = "org.freedesktop.UDisks2.Encrypted";
pub const KIND: &str = "btrfs";
pub fn no_snapshot() -> &'static str {
    t!("disks-room-no-snapshot")
}

fn align_up(n: u64) -> u64 {
    n.div_ceil(MIB) * MIB
}

fn align_down(n: u64) -> u64 {
    n / MIB * MIB
}

#[derive(Clone, Debug, Default)]
pub struct Facts {
    pub kind: String,
    pub in_use: bool,
    pub mounts: Vec<String>,
    pub encrypted: bool,
    pub opened: Option<u64>,
    pub part: Option<(u64, u64, bool)>,
    pub part_type: String,
    pub used: Option<u64>,
    pub fs_size: Option<u64>,
    pub devices: Option<usize>,
    pub read_only: Option<bool>,
    pub swap_file: Option<bool>,
    pub online_shrink: bool,
    pub snapshot: bool,
    pub busy: bool,
}

#[derive(Debug, PartialEq)]
pub enum Offer {
    Nothing,
    Not(String),
    Make { least: u64, most: u64 },
}

fn is_gpt(part_type: &str) -> bool {
    part_type.len() == 36 && part_type.matches('-').count() == 4
}

pub fn kind_words(kind: &str) -> String {
    match kind {
        "ext2" | "ext3" | "ext4" => t!("disks-room-ext-offline", kind = kind),
        "xfs" => t!("disks-room-xfs-fixed").into(),
        other => t!("disks-room-other-kind", kind = other),
    }
}

fn fit_to_shrink(f: &Facts, container: bool) -> Result<(), String> {
    if !f.online_shrink {
        return Err(t!("disks-room-no-online-shrink").into());
    }
    if container {
        return Err(t!("disks-room-holds-parts").into());
    }
    if !is_gpt(&f.part_type) {
        return Err(t!("disks-room-not-gpt").into());
    }
    if !f.in_use {
        return Err(t!("disks-room-mount-first").into());
    }
    if f.mounts.len() != 1 {
        return Err(t!("disks-room-mounts-not-one").into());
    }
    match f.read_only {
        Some(false) => {}
        Some(true) => return Err(t!("disks-room-read-only").into()),
        None => return Err(t!("disks-room-read-only-unknown").into()),
    }
    match f.devices {
        Some(1) => {}
        Some(_) => return Err(t!("disks-room-many-drives").into()),
        None => return Err(t!("disks-room-drives-unknown").into()),
    }
    match f.swap_file {
        Some(false) => {}
        Some(true) => return Err(t!("disks-room-swap-file").into()),
        None => return Err(t!("disks-room-swap-unknown").into()),
    }
    Ok(())
}

fn limits(f: &Facts) -> Result<(u64, u64, u64), String> {
    let Some((_, size, container)) = f.part else { return Err(t!("disks-room-not-in-table").into()) };
    fit_to_shrink(f, container)?;
    let Some(used) = f.used else { return Err(t!("disks-room-used-unknown").into()) };
    let Some(fs) = f.fs_size.filter(|n| *n > 0) else {
        return Err(t!("disks-room-area-unknown").into());
    };
    let (header, whole) = match (f.encrypted, f.opened) {
        (false, _) => (0, size),
        (true, None) => return Err(t!("disks-room-open-missing").into()),
        (true, Some(open)) => {
            let header = size.saturating_sub(open);
            if open >= size || !(HEADER_LEAST..=HEADER_MOST).contains(&header) {
                return Err(t!("disks-room-encryption-sizes").into());
            }
            (header, open)
        }
    };
    if fs > whole || used > fs {
        return Err(t!("disks-room-sizes").into());
    }
    if f.busy {
        return Err(plan::one_at_a_time().into());
    }
    if !f.snapshot {
        return Err(no_snapshot().into());
    }
    let keep_fs = align_up(used + (used / 10).max(SPARE_LEAST));
    let keep = align_up(keep_fs + header);
    let most = size.saturating_sub(keep);
    if most < ROOM_LEAST {
        return Err(t!("disks-room-too-little", used = plan::size_text(used as f64)));
    }
    Ok((ROOM_LEAST, align_down(most), header))
}

pub fn offer(f: &Facts) -> Offer {
    if f.part.is_none() || f.kind.is_empty() || matches!(f.kind.as_str(), "swap" | "crypto_LUKS") {
        return Offer::Nothing;
    }
    if f.kind != KIND {
        return if f.in_use { Offer::Not(kind_words(&f.kind)) } else { Offer::Nothing };
    }
    match limits(f) {
        Ok((least, most, _)) => Offer::Make { least, most },
        Err(why) => Offer::Not(why),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes {
    pub fs: u64,
    pub open: Option<u64>,
    pub part: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
    Fs(u64),
    Open(u64),
    Part(u64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub was: Sizes,
    pub moves: Vec<Move>,
    pub header: u64,
    pub used: u64,
    pub room: u64,
}

pub fn plan(f: &Facts, want: u64) -> Result<Plan, String> {
    let (least, most, header) = limits(f)?;
    let (Some((_, size, _)), Some(fs), Some(used)) = (f.part, f.fs_size, f.used) else { return Err(t!("disks-room-sizes-unread").into()) };
    if want < least || want > most {
        return Err(t!("disks-room-range", least = plan::size_text(least as f64), most = plan::size_text(most as f64)));
    }
    let keep = align_down(size - align_up(want));
    let target = keep - header;
    let mut moves = Vec::new();
    if fs > target {
        moves.push(Move::Fs(target));
    }
    if f.encrypted && f.opened.is_some_and(|o| o > target) {
        moves.push(Move::Open(target));
    }
    moves.push(Move::Part(keep));
    Ok(Plan { was: Sizes { fs, open: f.opened.filter(|_| f.encrypted), part: size }, moves, header, used, room: size - keep })
}

pub fn after(plan: &Plan, done: usize) -> Sizes {
    let mut s = plan.was;
    for m in plan.moves.iter().take(done) {
        match *m {
            Move::Fs(n) => s.fs = n,
            Move::Open(n) => s.open = Some(n),
            Move::Part(n) => s.part = n,
        }
    }
    s
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seen {
    pub fs: u64,
    pub open: Option<u64>,
    pub part: u64,
    pub mounted: bool,
}

pub fn verify(plan: &Plan, done: usize, seen: &Seen) -> Result<(), String> {
    let want = after(plan, done);
    let said = |what: String| Err(t!("disks-room-stopped", what = what));
    if !seen.mounted {
        return said(t!("disks-room-check-unmounted").into());
    }
    if seen.part != want.part {
        return said(t!("disks-room-check-part", is = plan::size_text(seen.part as f64), should = plan::size_text(want.part as f64)));
    }
    if seen.open != want.open {
        return said(t!("disks-room-check-open").into());
    }
    let moved = plan.moves.iter().take(done).any(|m| matches!(m, Move::Fs(_)));
    let within = if moved { seen.fs <= want.fs && seen.fs + FS_SLACK >= want.fs } else { seen.fs == want.fs };
    if !within {
        return said(t!("disks-room-check-area", is = plan::size_text(seen.fs as f64), should = plan::size_text(want.fs as f64)));
    }
    if seen.fs < plan.used {
        return said(t!("disks-room-check-area-small").into());
    }
    let holds = seen.open.unwrap_or(seen.part.saturating_sub(plan.header));
    if seen.fs > holds || holds + plan.header > seen.part {
        return said(t!("disks-room-check-fit").into());
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ids {
    pub part: String,
    pub open: Option<String>,
    pub fs: String,
}

pub fn seen(objects: &Objects, ids: &Ids) -> Result<Seen, String> {
    let missing = |what: &str| t!("disks-room-gone", what = what);
    let part = objects.get(&ids.part).and_then(|o| o.get(PARTITION)).ok_or_else(|| missing("part"))?;
    let open = match &ids.open {
        Some(p) => Some(num(objects.get(p).and_then(|o| o.get(BLOCK)).ok_or_else(|| missing("copy"))?, "Size")),
        None => None,
    };
    let fs = objects.get(&ids.fs).and_then(|o| o.get(FILESYSTEM)).ok_or_else(|| missing("files"))?;
    Ok(Seen { fs: num(fs, "Size"), open, part: num(part, "Size"), mounted: !mount_points(fs).is_empty() })
}

pub fn judge(objects: &Objects, ids: &Ids, plan: &Plan, done: usize) -> Result<(), String> {
    verify(plan, done, &seen(objects, ids)?)
}

pub struct World {
    pub mountinfo: String,
    pub swaps: String,
    pub devices: fn(&str) -> Option<usize>,
    pub used: fn(&str) -> Option<u64>,
    pub snapper: bool,
}

impl World {
    pub fn none() -> World {
        World { mountinfo: String::new(), swaps: String::new(), devices: |_| None, used: |_| None, snapper: false }
    }

    pub fn read() -> World {
        World {
            mountinfo: std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default(),
            swaps: std::fs::read_to_string("/proc/swaps").unwrap_or_default(),
            devices: btrfs_devices,
            used: |dir| used_space(dir),
            snapper: crate::common::snapshots::available(),
        }
    }
}

pub fn kind_of(objects: &Objects, v: &Vol) -> String {
    let open = (v.usage == "crypto").then(|| {
        objects.iter().find(|(_, o)| o.get(BLOCK).is_some_and(|b| dbus::get_path(b, "CryptoBackingDevice") == v.path)).map(|(p, _)| p.as_str())
    });
    let at = match open {
        Some(Some(p)) => p,
        Some(None) => return String::new(),
        None => v.path.as_str(),
    };
    objects.get(at).and_then(|o| o.get(BLOCK)).map(|b| dbus::get_str(b, "IdType")).unwrap_or_default()
}

fn btrfs_devices(uuid: &str) -> Option<usize> {
    if uuid.is_empty() || !uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        return None;
    }
    std::fs::read_dir(format!("/sys/fs/btrfs/{uuid}/devices")).ok().map(|d| d.flatten().count())
}

pub fn read_only(mountinfo: &str, dir: &str) -> Option<bool> {
    mountinfo.lines().find_map(|line| {
        let mut f = line.split(' ');
        let (at, options) = (f.nth(4)?, f.next()?);
        (plan::unescape(at) == dir).then(|| options.split(',').any(|o| o == "ro"))
    })
}

pub fn swap_file_under(swaps: &str, dir: &str) -> bool {
    let under = if dir == "/" { "/".to_string() } else { format!("{}/", dir.trim_end_matches('/')) };
    swaps.lines().skip(1).any(|l| {
        let mut f = l.split_whitespace();
        let (name, kind) = (f.next().unwrap_or(""), f.next().unwrap_or(""));
        kind == "file" && name.starts_with(&under)
    })
}

pub fn facts(objects: &Objects, v: &Vol, online_shrink: bool, busy: bool, world: &World) -> (Facts, Option<Ids>) {
    let encrypted = v.usage == "crypto";
    let open_path = encrypted.then(|| {
        objects.iter()
            .find(|(_, o)| o.get(BLOCK).is_some_and(|b| dbus::get_path(b, "CryptoBackingDevice") == v.path))
            .map(|(p, _)| p.clone())
    }).flatten();
    let fs_path = open_path.clone().unwrap_or_else(|| v.path.clone());
    let block = objects.get(&fs_path).and_then(|o| o.get(BLOCK));
    let fs = objects.get(&fs_path).and_then(|o| o.get(FILESYSTEM));
    let mounts = fs.map(mount_points).unwrap_or_default();
    let dir = mounts.first().cloned().unwrap_or_default();
    let uuid = block.map(|b| dbus::get_str(b, "IdUUID")).unwrap_or_default();
    let kind = block.map(|b| dbus::get_str(b, "IdType")).unwrap_or_default();
    let ids = fs.is_some().then(|| Ids { part: v.path.clone(), open: open_path.clone(), fs: fs_path.clone() });
    let facts = Facts {
        kind,
        in_use: !mounts.is_empty() || v.system,
        encrypted,
        opened: open_path.as_ref().and_then(|p| objects.get(p)).and_then(|o| o.get(BLOCK)).map(|b| num(b, "Size")),
        part: v.part,
        part_type: v.part_type.clone(),
        used: (!dir.is_empty()).then(|| (world.used)(&dir)).flatten(),
        fs_size: fs.map(|f| num(f, "Size")),
        devices: (world.devices)(&uuid),
        read_only: read_only(&world.mountinfo, &dir),
        swap_file: (!dir.is_empty()).then(|| swap_file_under(&world.swaps, &dir)),
        online_shrink,
        snapshot: world.snapper && mounts == ["/"],
        busy,
        mounts,
    };
    (facts, ids)
}

pub fn steps(plan: &Plan, ids: &Ids) -> Vec<Step> {
    let shared = Rc::new((plan.clone(), ids.clone()));
    let mut out = vec![check(&shared, 0)];
    for (i, m) in plan.moves.iter().enumerate() {
        out.push(match *m {
            Move::Fs(n) => step(&ids.fs, FILESYSTEM, "Resize", (n, Opts::new())).called(t!("disks-room-step-files")),
            Move::Open(n) => step(&ids.part, ENCRYPTED, "Resize", (n, Opts::new())).called(t!("disks-room-step-open")),
            Move::Part(n) => step(&ids.part, PARTITION, "Resize", (n, Opts::new())).called(t!("disks-room-step-part")),
        });
        out.push(check(&shared, i + 1));
    }
    out
}

fn check(shared: &Rc<(Plan, Ids)>, done: usize) -> Step {
    let shared = shared.clone();
    Step {
        method: "Verify",
        path: shared.1.part.clone(),
        what: t!("disks-room-step-check").into(),
        send: Box::new(move |bus, lp, sent| {
            bus.call(UD, UD_PATH, OBJECT_MANAGER).method("GetManagedObjects", &(), false, Some(Duration::from_secs(25)), lp, move |r| {
                let verdict = r.map_err(|e| e.to_string()).and_then(|m| {
                    let objects = parse_objects(&m)?;
                    judge(&objects, &shared.1, &shared.0, done).map(|()| m)
                });
                sent(verdict.map_err(|why| Failed::new("", &why)));
            });
        }),
    }
}

pub fn snapshot_words(dev: &str, at: u64) -> String {
    t!("disks-room-snapshot-note", dev = dev, at = at)
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: u64 = 1_000_000_000;
    const GPT: &str = "0fc63daf-8483-4772-8e79-3d69d8477de4";

    fn plain() -> Facts {
        Facts {
            kind: KIND.into(), in_use: true, mounts: vec!["/".into()], encrypted: false, opened: None,
            part: Some((MIB, 400 * G / MIB * MIB, false)), part_type: GPT.into(), used: Some(100 * G), fs_size: Some(400 * G / MIB * MIB),
            devices: Some(1), read_only: Some(false), swap_file: Some(false), online_shrink: true, snapshot: true, busy: false,
        }
    }

    fn encrypted() -> Facts {
        let mut f = plain();
        let size = 400 * G / MIB * MIB;
        f.encrypted = true;
        f.opened = Some(size - 16 * MIB);
        f.fs_size = Some(size - 16 * MIB);
        f
    }

    #[test]
    fn a_plain_part_is_shrunk_in_two_steps_and_an_encrypted_one_in_three() {
        let p = plan(&plain(), 100 * G).expect("a plan");
        assert_eq!(p.moves.len(), 2);
        assert!(matches!(p.moves[0], Move::Fs(_)) && matches!(p.moves[1], Move::Part(_)));
        let e = plan(&encrypted(), 100 * G).expect("a plan");
        assert!(matches!(e.moves[..], [Move::Fs(_), Move::Open(_), Move::Part(_)]), "{:?}", e.moves);
        let Move::Part(keep) = e.moves[2] else { unreachable!() };
        let Move::Fs(f) = e.moves[0] else { unreachable!() };
        let Move::Open(o) = e.moves[1] else { unreachable!() };
        assert_eq!((f, o), (keep - 16 * MIB, keep - 16 * MIB));
        assert!(e.room >= 100 * G && e.room < 100 * G + 2 * MIB);
    }

    #[test]
    fn the_limits_keep_the_files_a_tenth_and_two_gigabytes_more() {
        let (least, most, _) = limits(&plain()).expect("limits");
        assert_eq!(least, GIB);
        let size = plain().part.unwrap().1;
        assert!(size - most >= 100 * G + 10 * G, "a tenth more than the 100 GB used");
        let mut small = plain();
        small.used = Some(G);
        let (_, most, _) = limits(&small).unwrap();
        assert!(size - most >= 3 * G);
        assert!(plan(&plain(), 399 * G).is_err());
        assert!(plan(&plain(), GIB - 1).is_err());
    }

    #[test]
    fn every_doubt_is_refused_with_its_reason() {
        let cases: Vec<(&str, Box<dyn Fn(&mut Facts)>)> = vec![
            ("not a part", Box::new(|f| f.part = None)),
            ("cannot make Btrfs smaller", Box::new(|f| f.online_shrink = false)),
            ("holds other parts", Box::new(|f| f.part = Some((MIB, 400 * G, true)))),
            ("GPT", Box::new(|f| f.part_type = "0x83".into())),
            ("Mount it first", Box::new(|f| f.in_use = false)),
            ("exactly one place", Box::new(|f| f.mounts = vec!["/".into(), "/mnt/x".into()])),
            ("exactly one place", Box::new(|f| f.mounts = Vec::new())),
            ("read-only", Box::new(|f| f.read_only = Some(true))),
            ("whether it is read-only", Box::new(|f| f.read_only = None)),
            ("more than one drive", Box::new(|f| f.devices = Some(2))),
            ("how many drives", Box::new(|f| f.devices = None)),
            ("swap file", Box::new(|f| f.swap_file = Some(true))),
            ("whether a swap file", Box::new(|f| f.swap_file = None)),
            ("in use could not be read", Box::new(|f| f.used = None)),
            ("how big the files", Box::new(|f| f.fs_size = None)),
            ("how big the files", Box::new(|f| f.fs_size = Some(0))),
            ("do not agree", Box::new(|f| f.fs_size = Some(500 * G))),
            ("do not agree", Box::new(|f| f.used = Some(f.fs_size.unwrap() + 1))),
            ("Another change", Box::new(|f| f.busy = true)),
            ("snapshot", Box::new(|f| f.snapshot = false)),
            ("too little", Box::new(|f| f.used = Some(399 * G))),
        ];
        for (words, change) in &cases {
            let mut f = plain();
            change(&mut f);
            let why = limits(&f).expect_err(words);
            assert!(why.contains(words), "{words}: {why}");
            assert!(plan(&f, 50 * G).is_err());
            assert!(matches!(offer(&f), Offer::Not(_) | Offer::Nothing), "{words}");
        }
        let encrypted_cases: Vec<(&str, Box<dyn Fn(&mut Facts)>)> = vec![
            ("open copy of it could not be found", Box::new(|f| f.opened = None)),
            ("encryption do not agree", Box::new(|f| f.opened = Some(f.part.unwrap().1))),
            ("encryption do not agree", Box::new(|f| f.opened = Some(f.part.unwrap().1 - 100 * MIB))),
            ("encryption do not agree", Box::new(|f| f.opened = Some(f.part.unwrap().1 - 100))),
        ];
        for (words, change) in &encrypted_cases {
            let mut f = encrypted();
            change(&mut f);
            let why = limits(&f).expect_err(words);
            assert!(why.contains(words), "{words}: {why}");
        }
    }

    #[test]
    fn only_btrfs_is_offered_and_another_kind_in_use_says_why_not() {
        assert!(matches!(offer(&plain()), Offer::Make { .. }));
        let mut f = plain();
        f.kind = "ext4".into();
        let Offer::Not(why) = offer(&f) else { panic!("a label") };
        assert!(why.contains("ext4") && why.contains("not in use"), "{why}");
        f.kind = "xfs".into();
        assert_eq!(offer(&f), Offer::Not("This xfs part cannot be made smaller.".into()));
        f.kind = "ntfs".into();
        assert!(matches!(offer(&f), Offer::Not(w) if w.contains("ntfs")));
        f.in_use = false;
        assert_eq!(offer(&f), Offer::Nothing);
        for kind in ["", "swap", "crypto_LUKS"] {
            let mut f = plain();
            f.kind = kind.into();
            assert_eq!(offer(&f), Offer::Nothing, "{kind:?}");
        }
        for k in ["ext4", "xfs", "ntfs"] {
            assert!(!kind_words(k).contains('\u{2014}'));
        }
    }

    #[derive(Clone)]
    struct Disk {
        fs: u64,
        open: Option<u64>,
        part: u64,
        header: u64,
    }

    impl Disk {
        fn of(f: &Facts) -> Disk {
            let part = f.part.unwrap().1;
            Disk { fs: f.fs_size.unwrap(), open: f.opened.filter(|_| f.encrypted), part, header: f.opened.map(|o| part - o).filter(|_| f.encrypted).unwrap_or(0) }
        }
        fn seen(&self) -> Seen {
            Seen { fs: self.fs, open: self.open, part: self.part, mounted: true }
        }
        fn nested(&self) -> bool {
            let holds = self.open.unwrap_or(self.part - self.header);
            self.fs <= holds && holds + self.header <= self.part
        }
        fn apply(&mut self, m: Move) {
            match m {
                Move::Fs(n) => self.fs = n,
                Move::Open(n) => self.open = Some(n),
                Move::Part(n) => self.part = n,
            }
        }
    }

    #[test]
    fn a_run_stopped_after_any_step_leaves_every_layer_inside_what_holds_it() {
        for f in [plain(), encrypted()] {
            for want in [GIB, 10 * G, 100 * G, 200 * G] {
                let Ok(p) = plan(&f, want) else { continue };
                let mut disk = Disk::of(&f);
                assert!(verify(&p, 0, &disk.seen()).is_ok(), "the start is as planned");
                for (i, m) in p.moves.iter().enumerate() {
                    disk.apply(*m);
                    assert!(disk.nested(), "after {m:?} of {:?}", p.moves);
                    assert_eq!(verify(&p, i + 1, &disk.seen()), Ok(()), "after {m:?}");
                }
                assert_eq!(disk.part, p.was.part - p.room);
                for done in 0..p.moves.len() {
                    assert_eq!(after(&p, done).part, p.was.part);
                }
            }
        }
    }

    #[test]
    fn a_check_stops_the_run_when_anything_is_off() {
        let p = plan(&encrypted(), 100 * G).unwrap();
        let mut disk = Disk::of(&encrypted());
        let mut d = disk.clone();
        d.part -= MIB;
        assert!(verify(&p, 0, &d.seen()).is_err());
        let mut d = disk.clone();
        d.fs -= G;
        assert!(verify(&p, 0, &d.seen()).is_err(), "files' area not as planned");
        assert!(verify(&p, 1, &disk.seen()).is_err());
        disk.apply(p.moves[0]);
        assert!(verify(&p, 1, &disk.seen()).is_ok());
        assert!(verify(&p, 2, &disk.seen()).is_err());
        let mut d = disk.clone();
        d.open = Some(d.fs - G);
        assert!(verify(&p, 2, &d.seen()).is_err());
        disk.apply(p.moves[1]);
        assert!(verify(&p, 2, &disk.seen()).is_ok());
        assert!(verify(&p, 3, &disk.seen()).is_err());
        let mut d = disk.clone();
        d.apply(p.moves[2]);
        assert!(verify(&p, 3, &d.seen()).is_ok());
        d.apply(Move::Part(d.part - 100 * G));
        assert!(verify(&p, 3, &d.seen()).is_err(), "the part shrank by more than planned");
        let mut gone = d.seen();
        gone.mounted = false;
        assert!(verify(&p, 3, &gone).is_err());
        let mut small = Disk::of(&plain());
        let q = plan(&plain(), GIB).unwrap();
        small.fs = q.used - 1;
        assert!(verify(&q, 0, &small.seen()).is_err());
    }

    #[test]
    fn a_run_stopped_part_way_can_be_pressed_again() {
        let mut f = encrypted();
        let first = plan(&f, 100 * G).unwrap();
        let Move::Fs(target) = first.moves[0] else { panic!() };
        f.fs_size = Some(target);
        let second = plan(&f, 100 * G).unwrap();
        assert!(matches!(second.moves[..], [Move::Open(_), Move::Part(_)]), "{:?}", second.moves);
        f.opened = Some(target);
        let why = plan(&f, 100 * G).expect_err("refused");
        assert!(why.contains("encryption do not agree"), "{why}");
        let mut g = plain();
        let first = plan(&g, 100 * G).unwrap();
        let Move::Fs(t) = first.moves[0] else { panic!() };
        g.fs_size = Some(t);
        let again = plan(&g, 100 * G).unwrap();
        assert!(matches!(again.moves[..], [Move::Part(_)]), "{:?}", again.moves);
        let mut disk = Disk::of(&g);
        assert!(verify(&again, 0, &disk.seen()).is_ok());
        disk.apply(again.moves[0]);
        assert!(verify(&again, 1, &disk.seen()).is_ok() && disk.nested());
    }

    #[test]
    fn the_system_is_read_for_read_only_and_swap_files() {
        let info = "36 1 259:2 /@rootfs / rw,relatime shared:1 - btrfs /dev/nvme0n1p2 rw,ssd,subvol=/@rootfs\n\
                    40 36 259:3 / /mnt/a\\040b ro,relatime - btrfs /dev/sdb1 ro\n";
        assert_eq!(read_only(info, "/"), Some(false));
        assert_eq!(read_only(info, "/mnt/a b"), Some(true));
        assert_eq!(read_only(info, "/nowhere"), None);
        let swaps = "Filename\tType\tSize\tUsed\tPriority\n/dev/zram0 partition 100 0 100\n/swapfile file 100 0 -2\n";
        assert!(swap_file_under(swaps, "/"));
        assert!(!swap_file_under(swaps, "/mnt/x"));
        assert!(!swap_file_under("Filename\tType\tSize\tUsed\tPriority\n/dev/sda2 partition 100 0 -2\n", "/"));
        assert!(swap_file_under("Filename Type\n/mnt/x/swapfile file 1 0 -2\n", "/mnt/x"));
        assert!(!swap_file_under("Filename Type\n/mnt/xy/swapfile file 1 0 -2\n", "/mnt/x"));
    }
}
