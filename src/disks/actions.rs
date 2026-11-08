//! The changes a person can ask for: hide, rename, folder, resize, make room, check, format, delete.

use super::plan::{Fstab, MIB, size_text};
use zbus::zvariant::{OwnedObjectPath, Value};
use crate::common::eye::{HIDE_ICON, SHOW_ICON};
use crate::common::hidden_drives;
use crate::common::run::spawn;
use crate::toolkit::bus as dbus;
use crate::toolkit::t;
use crate::toolkit::widget::{Node, button};
use super::{Act, Do, Drives, GB, Msg, help, plan, room, spot};
use super::udisks::UD;
use super::udisks::DRIVE;
use super::udisks::BLOCK;
use super::udisks::FILESYSTEM;
use super::udisks::PARTITION;
use super::udisks::PARTITION_TABLE;
use super::udisks::Opts;
use super::udisks::opts;
use super::udisks::fstab_item;
use super::udisks::used_space;
use super::udisks::taken_dirs;
use super::udisks::me_ids;
use super::model::Vol;
use super::model::Drive;
use super::steps::Carried;
use super::steps::Step;
use super::steps::step;
use super::steps::unmount;
use super::steps::mount;

pub(super) fn fs_opts(fs: &str, name: &str, tear_down: bool) -> Opts {
    let mut o = opts(vec![("update-partition-type", Value::from(true))]);

    if !name.trim().is_empty() {
        o.insert("label", Value::from(name.trim().to_string()));
    }
    if fs == "ext4" {
        o.insert("take-ownership", Value::from(true));
    }
    if tear_down {
        o.insert("tear-down", Value::from(true));
    }
    o
}

fn tear_down() -> Opts {
    opts(vec![("tear-down", Value::from(true))])
}

pub(super) fn format_key(drive: &str, target: &str, whole: bool) -> String {
    if whole { format!("{drive}:wipe") } else { format!("{target}:format") }
}

impl Drives {

    pub(super) fn hide(&mut self, drive: &str, row: &str, key: &str, hide: bool) {
        match hidden_drives::set_in(&self.hidden_file, key, hide) {
            Ok(()) => self.hidden = hidden_drives::read_in(&self.hidden_file),
            Err(e) => self.say_on(drive, row, t!("disks-hide-failed", why = e.to_string()), false, help::HIDE),
        }
    }

    pub(super) fn drive_hidden(&self, d: &Drive) -> bool {
        !d.system && !d.key.is_empty() && self.hidden.contains(&d.key)
    }

    pub(super) fn part_hidden(&self, d: &Drive, v: &Vol) -> bool {
        !d.system && hidden_drives::hides(&self.hidden, &d.key, &v.key)
    }

    pub(super) fn eye(&self, drive: &str, row: &str, key: &str, hidden: bool, what: &str) -> Node<Msg> {
        let (icon, tip) = if hidden {
            (HIDE_ICON, t!("disks-eye-show", what = what))
        } else {
            (SHOW_ICON, t!("disks-eye-hide", what = what, lock = hidden_drives::NOT_A_LOCK))
        };
        button(icon, tip).on_press(Msg::Hide { drive: drive.into(), row: row.into(), key: key.into(), hide: !hidden })
    }

    fn text_of(&self, key: &str, or: &str) -> String {
        self.texts.get(key).map(|t| t.trim().to_string()).unwrap_or_else(|| or.to_string())
    }

    pub(super) fn act(&mut self, a: Act) {
        match a {
            Act::Off { drive, method } => self.drive_off(&drive, method),
            Act::Mount { drive, vol, on } => {
                let (s, doing, done) = if on { (mount(&vol), t!("disks-mounting"), t!("disks-mounted")) } else { (unmount(&vol), t!("disks-unmounting"), t!("disks-unmounted")) };
                self.job(&drive, &vol, doing, vec![s], move |r| r.map(|_| done.to_string()));
            }
            Act::Open(dir) => {
                spawn(&["xdg-open", &dir]);
            }
            Act::Rename { drive, vol } => self.rename(&drive, &vol),
            Act::Folder { drive, vol } => self.folder(&drive, &vol),
            Act::Resize { drive, vol } => self.resize(&drive, &vol),
            Act::Room { drive, vol } => self.room(&drive, &vol),
            Act::Check { drive, vol } => self.check(&drive, &vol, false),
            Act::Repair { drive, vol } => self.check(&drive, &vol, true),
            Act::Format { drive, target, whole } => self.format(&drive, &target, whole),
            Act::Delete { drive, vol } => self.delete(&drive, &vol),
            Act::Make { drive, offset, size } => self.make(&drive, offset, size),
        }
    }

    fn drive_off(&mut self, drive: &str, method: &'static str) {
        let Some(d) = self.drive(drive) else { return };
        let mut steps: Vec<Step> = d.mounted(&self.objects).iter().map(|p| unmount(p)).collect();
        steps.push(step(drive, DRIVE, method, (Opts::new(),)));
        let (doing, done) = if method == "Eject" { (t!("disks-closing-it"), t!("disks-safe-remove")) } else { (t!("disks-powering-off"), t!("disks-off-safe")) };
        self.job(drive, "", doing, steps, move |r| match r {
            Ok(_) => Ok(done.into()),

            Err(e) if method == "Eject" && e.contains("\u{201c}eject\u{201d}") && e.contains("No such file") => Ok(done.into()),
            Err(e) => Err(e),
        });
    }

    fn rename(&mut self, drive: &str, vol: &str) {
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let v = v.clone();
        let at = format!("{vol}:name");
        let label = self.text_of(&at, &v.label);
        if label == v.label {
            return;
        }
        if let Err(why) = plan::check_label(&v.fs_type, &label) {
            self.say_on(drive, &at, why, false, help::NAMES);
            return;
        }
        let s = step(&v.path, FILESYSTEM, "SetLabel", (label, Opts::new()));
        let steps = if !v.mounts.is_empty() && !plan::label_online(&v.fs_type) {
            vec![unmount(&v.path), s, mount(&v.path)]
        } else {
            vec![s]
        };
        self.job(drive, &at, t!("disks-renaming"), steps, |r| r.map(|_| t!("disks-renamed").into()));
    }

    pub(super) fn folder(&mut self, drive: &str, vol: &str) {
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let v = v.clone();
        let at = format!("{vol}:folder");
        let own = v.fstab.clone();
        let name = self.text_of(&at, &own.as_ref().map(|o| plan::name_of(&o.dir)).unwrap_or_default());
        if name.is_empty() {
            if own.is_some() {
                self.set_fstab(drive, &v, None, &at, t!("disks-folder-auto").into());
            }
            return;
        }
        let mut taken = taken_dirs();
        if let Some(o) = &own {
            taken.remove(&o.dir);
        }
        for m in &v.mounts {
            taken.remove(m);
        }
        match plan::check_name(&name, &taken) {
            Err(why) => self.say_on(drive, &at, why, false, help::NAMES),
            Ok(dir) => {
                if own.as_ref().is_some_and(|o| o.dir == dir) {
                    return;
                }
                let (uid, gid) = me_ids();
                let startup = own.as_ref().is_some_and(|o| plan::starts_at_boot(&o.opts));
                let n = plan::entry(own.as_ref(), &v.uuid, &v.fs_type, &dir, startup, uid, gid);
                self.set_fstab(drive, &v, Some(n), &at, t!("disks-folder-appears", dir = &dir));
            }
        }
    }

    pub(super) fn startup(&mut self, drive: &str, vol: &str, on: bool) {
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let v = v.clone();
        let dir = match &v.fstab {
            Some(f) => f.dir.clone(),
            None => format!("{}/{}", plan::MNT, plan::suggest_name(&v.label, &v.uuid, &taken_dirs())),
        };
        let (uid, gid) = me_ids();
        let n = plan::entry(v.fstab.as_ref(), &v.uuid, &v.fs_type, &dir, on, uid, gid);
        let words = if on { t!("disks-startup-on", dir = &dir) } else { t!("disks-startup-off").into() };
        self.set_fstab(drive, &v, Some(n), &format!("{vol}:startup"), words);
    }

    fn set_fstab(&mut self, drive: &str, v: &Vol, want: Option<Fstab>, at: &str, words: String) {
        let s = match (&v.fstab, &want) {
            (None, Some(n)) => step(&v.path, BLOCK, "AddConfigurationItem", (fstab_item(n), Opts::new())),
            (Some(o), Some(n)) => step(&v.path, BLOCK, "UpdateConfigurationItem", (fstab_item(o), fstab_item(n), Opts::new())),
            (Some(o), None) => step(&v.path, BLOCK, "RemoveConfigurationItem", (fstab_item(o), Opts::new())),
            (None, None) => return,
        };
        let moved = want.as_ref().is_some_and(|n| !v.mounts.is_empty() && !v.mounts.contains(&n.dir));
        let steps = if moved { vec![unmount(&v.path), s, mount(&v.path)] } else { vec![s] };
        self.job(drive, at, t!("disks-saving"), steps, move |r| r.map(|_| words));
    }

    pub(super) fn resize_range(&self, d: &Drive, v: &Vol) -> Option<(u64, u64, Option<u64>, u64)> {
        let (offset, size, _) = v.part?;
        let mode = self.caps.get(&v.fs_type).map(|c| c.resize).unwrap_or(0);
        let room = plan::room_after(offset, size, &d.gaps);
        let used = v.mounts.first().and_then(|m| used_space(m));
        let (min, max) = plan::resize_range(size, used, room, mode);
        (max >= min + 16 * MIB).then_some((min, max, used, mode))
    }

    fn resize(&mut self, drive: &str, vol: &str) {
        if !self.armed(&format!("resize:{vol}")) {
            return;
        }
        let Some((d, v)) = self.vol(drive, vol) else { return };
        let Some((min, max, _, mode)) = self.resize_range(d, v) else { return };
        let v = v.clone();
        let at = format!("{vol}:size");
        let gb = self.gbs.get(&at).copied().unwrap_or(v.size as f32 / GB as f32);
        let want = ((f64::from(gb) * GB) as u64 / MIB * MIB).clamp(min, max);
        if want.abs_diff(v.size) < MIB {
            return;
        }
        let grow = want > v.size;
        let mounted = !v.mounts.is_empty();
        let unmount_first = match plan::resize_needs_unmount(mode, mounted, grow) {
            Ok(u) => u,
            Err(why) => {
                self.say_on(drive, &at, why, false, help::NAMES);
                return;
            }
        };
        if !grow && v.fs_type == "ntfs" && crate::common::otheros::windows_asleep(&v.dev) {
            self.say_on(drive, &at, t!("disks-windows-asleep", name = v.name()), false, help::NAMES);
            return;
        }
        let p = v.path.as_str();
        let mut steps = Vec::new();
        if unmount_first {
            steps.push(unmount(p));
        }

        if v.fs_type.starts_with("ext") && (unmount_first || !mounted) {
            steps.push(step(p, FILESYSTEM, "Repair", (Opts::new(),)));
        }
        let part = step(p, PARTITION, "Resize", (want, Opts::new()));
        if grow {
            steps.push(part);
            steps.push(step(p, FILESYSTEM, "Resize", (0u64, Opts::new())));
        } else {
            steps.push(step(p, FILESYSTEM, "Resize", (want, Opts::new())));
            steps.push(part);
        }
        if v.fs_type == "ntfs" {
            steps.push(step(p, FILESYSTEM, "Repair", (Opts::new(),)));
        }
        if unmount_first {
            steps.push(mount(p));
        }
        self.job(drive, &at, t!("disks-resizing"), steps, move |r| r.map(|_| t!("disks-resized", size = size_text(want as f64))));
    }

    pub(super) fn room_facts(&self, v: &Vol, busy: bool) -> (room::Facts, Option<room::Ids>) {
        let online = self.caps.get(room::KIND).is_some_and(|c| c.resize & plan::ONLINE_SHRINK != 0);

        let world = if room::kind_of(&self.objects, v) == room::KIND { (self.world)() } else { room::World::none() };
        room::facts(&self.objects, v, online, busy, &world)
    }

    pub(super) fn room_want(&self, key: &str, facts: &room::Facts) -> u64 {
        let room::Offer::Make { least, most } = room::offer(facts) else { return 0 };
        let asked = self.gbs.get(key).map_or(least, |gb| (f64::from(*gb) * GB) as u64);
        asked.clamp(least, most)
    }

    fn room(&mut self, drive: &str, vol: &str) {
        if !self.armed(&format!("room:{vol}")) {
            return;
        }
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let v = v.clone();
        let at = format!("{vol}:room");
        let (facts, ids) = self.room_facts(&v, self.changing.get() || self.snapping);
        let want = self.room_want(&at, &facts);
        if let Err(why) = room::plan(&facts, want) {
            self.say_on(drive, &at, why, false, help::ROOM);
            return;
        }
        if ids.is_none() {
            self.say_on(drive, &at, t!("disks-room-files-missing").into(), false, help::ROOM);
            return;
        }
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let desc = room::snapshot_words(&v.dev, now);
        #[cfg(test)]
        self.snaps.push(desc.clone());
        self.snapping = true;
        self.busy += 1;
        self.working.insert(spot(drive, &at), t!("disks-snapshot-taking").into());
        let (asks, drive, vol) = (self.asks.clone(), drive.to_string(), vol.to_string());
        let told = desc.clone();
        (self.snap)(desc, Box::new(move |res| { asks.send(Do::Snapped { drive, vol, row: at, want, desc: told, res }); }));
    }

    pub(super) fn snapped(&mut self, drive: &str, vol: &str, row: &str, want: u64, _desc: &str, res: Result<(), String>) {
        self.snapping = false;
        self.working.remove(&spot(drive, row));
        if let Err(why) = res {
            self.say(drive, row, t!("disks-snapshot-failed", why = plan::sentence(&why)), false);
            return self.finish();
        }
        let Some((_, v)) = self.vol(drive, vol) else { return self.finish() };
        let v = v.clone();
        let (facts, ids) = self.room_facts(&v, self.changing.get());
        let (Some(ids), Ok(plan)) = (ids, room::plan(&facts, want)) else {
            self.say(drive, row, t!("disks-snapshot-changed").into(), false);
            return self.finish();
        };
        let made = plan.room;
        let steps = room::steps(&plan, &ids);
        self.busy = self.busy.saturating_sub(1);
        self.job(drive, row, t!("disks-making-room"), steps, move |r| r.map(|_| t!("disks-room-made", size = size_text(made as f64))));
    }

    fn check(&mut self, drive: &str, vol: &str, repair: bool) {
        if repair && !self.armed(&format!("repair:{vol}")) {
            return;
        }
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let (p, mounted) = (v.path.clone(), !v.mounts.is_empty());
        let at = format!("{vol}:faults");
        let mut steps = Vec::new();
        if mounted {
            steps.push(unmount(&p));
        }
        steps.push(step(&p, FILESYSTEM, if repair { "Repair" } else { "Check" }, (Opts::new(),)));
        if mounted {
            steps.push(mount(&p));
        }
        let n = usize::from(mounted);
        let (doing, good, bad) = if repair {
            (t!("disks-repairing"), t!("disks-repaired"), t!("disks-repair-failed"))
        } else {
            (t!("disks-checking"), t!("disks-no-faults"), t!("disks-faults-found"))
        };
        self.job(drive, &at, doing, steps, move |r| {
            let fine = r?.get(n).and_then(|m| m.body().deserialize::<(bool,)>().ok()).is_some_and(|(b,)| b);
            if fine { Ok(good.into()) } else { Err(bad.into()) }
        });
    }

    fn format(&mut self, drive: &str, target: &str, whole: bool) {
        let fk = format_key(drive, target, whole);
        if !self.armed(&fk) {
            return;
        }
        let kinds = self.kinds();
        let Some(fs) = kinds.get(self.picks.get(&format!("{fk}:kind")).copied().unwrap_or(0)).map(|k| k.0.to_string()) else { return };
        let name = self.text_of(&format!("{fk}:name"), "");
        let at = if whole { String::new() } else { fk.clone() };
        if let Err(why) = plan::check_label(&fs, &name) {
            self.say_on(drive, &at, why, false, help::NAMES);
            return;
        }
        let Some(d) = self.drive(drive) else { return };
        let objects = &self.objects;
        let mut steps: Vec<Step> = d.mounted(objects).into_iter()
            .filter(|p| whole || p == target || objects.get(p).and_then(|o| o.get(BLOCK)).is_some_and(|b| dbus::get_path(b, "CryptoBackingDevice") == target))
            .map(|p| unmount(&p))
            .collect();

        let keep = d.vols.iter()
            .filter(|v| whole || v.path == target)
            .find_map(|v| v.fstab.as_ref().map(|f| (f.dir.clone(), plan::starts_at_boot(&f.opts))));
        if whole {
            steps.push(step(target, BLOCK, "Format", (plan::table_for(d.size, &fs), tear_down())).called(t!("disks-erasing-whole")));
            steps.push(step(target, PARTITION_TABLE, "CreatePartitionAndFormat",
                (0u64, 0u64, "", "", Opts::new(), fs.clone(), fs_opts(&fs, &name, false))));
        } else {
            steps.push(step(target, BLOCK, "Format", (fs.clone(), fs_opts(&fs, &name, true))));
        }
        let (asks, target, on, at_row) = (self.asks.clone(), target.to_string(), drive.to_string(), at.clone());
        self.job(drive, &at, t!("disks-formatting"), steps, move |r| {
            let got = r?;
            let made = if whole {
                got.last().and_then(|m| m.body().deserialize::<(OwnedObjectPath,)>().ok()).map(|(p,)| p.to_string()).unwrap_or_default()
            } else {
                target
            };
            match keep {
                Some((dir, _)) if made.is_empty() => return Err(plan::folder_not_kept(&dir, t!("disks-new-part-missing"))),
                Some((dir, startup)) => {
                    asks.send(Do::Carry(Carried { drive: on, row: at_row, made, dir, startup, fs }));
                }
                None => {}
            }
            Ok(t!("disks-formatted").into())
        });
    }

    pub(super) fn carry_folder(&mut self, carried: Carried) {
        let Some(bus) = self.bus.clone() else {
            return self.not_carried(&carried, t!("disks-service-unreachable"));
        };
        self.busy += 1;
        let asks = self.asks.clone();
        bus.call(UD, &carried.made.clone(), BLOCK).get("IdUUID", &self.events, move |r| {
            let uuid = r.ok().and_then(|v| String::try_from(v).ok()).unwrap_or_default();
            asks.send(Do::CarryKnown { carried, uuid });
        });
    }

    pub(super) fn carry_known(&mut self, carried: Carried, uuid: &str) {
        if uuid.is_empty() {
            self.not_carried(&carried, t!("disks-uuid-unread"));
            return self.finish();
        }
        let (uid, gid) = me_ids();
        let n = plan::new_entry(uuid, &carried.dir, &carried.fs, carried.startup, uid, gid);
        let steps = vec![step(&carried.made, BLOCK, "AddConfigurationItem", (fstab_item(&n), Opts::new()))];
        let (asks, c) = (self.asks.clone(), carried.clone());
        let started = self.run(steps, Some((carried.drive.clone(), carried.row.clone())), Box::new(move |ended| {
            let res = match ended.res {
                Ok(_) => Ok(t!("disks-formatted").to_string()),
                Err(why) => Err(plan::folder_not_kept(&c.dir, &why)),
            };
            asks.send(Do::Done { drive: c.drive, row: c.row, res, standing: ended.standing.is_some() });
        }));
        match started {
            Ok(true) => {}

            Err(why) => {
                self.not_carried(&carried, &why);
                self.finish();
            }
            Ok(false) => {
                self.not_carried(&carried, t!("disks-service-unreachable"));
                self.finish();
            }
        }
    }

    fn not_carried(&mut self, carried: &Carried, why: &str) {
        self.say(&carried.drive, &carried.row, plan::folder_not_kept(&carried.dir, why), false);
    }

    fn delete(&mut self, drive: &str, vol: &str) {
        let Some((_, v)) = self.vol(drive, vol) else { return };
        let mut steps = Vec::new();
        if !v.mounts.is_empty() {
            steps.push(unmount(&v.path));
        }
        steps.push(step(&v.path, PARTITION, "Delete", (tear_down(),)));
        self.job(drive, "", t!("disks-deleting"), steps, |r| r.map(|_| t!("disks-deleted").into()));
    }

    fn make(&mut self, drive: &str, offset: u64, size: u64) {
        let gk = format!("{drive}:{offset}");
        if !self.armed(&format!("make:{gk}")) {
            return;
        }
        let kinds = self.kinds();
        let Some(fs) = kinds.get(self.picks.get(&format!("{gk}:kind")).copied().unwrap_or(0)).map(|k| k.0.to_string()) else { return };
        let name = self.text_of(&format!("{gk}:name"), "");
        if let Err(why) = plan::check_label(&fs, &name) {
            self.say_on(drive, "", why, false, help::NAMES);
            return;
        }
        let Some(whole) = self.drive(drive).and_then(|d| d.whole.clone()) else { return };
        let gb = self.gbs.get(&format!("{gk}:gb")).copied().unwrap_or(size as f32 / GB as f32);
        let mut want = ((f64::from(gb) * GB) as u64 / MIB * MIB).min(size);

        if size - want < 100 * MIB {
            want = size;
        }
        let s = step(&whole, PARTITION_TABLE, "CreatePartitionAndFormat",
            (offset, want, "", "", Opts::new(), fs.clone(), fs_opts(&fs, &name, false)));
        self.job(drive, "", t!("disks-making-it"), vec![s], |r| r.map(|_| t!("disks-made").into()));
    }
}
