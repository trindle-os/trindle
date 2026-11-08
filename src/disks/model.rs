//! The drives and their parts as the window knows them, built from what UDisks says.

use std::collections::{HashMap, HashSet};
use super::plan::{Fstab, Health, size_text};
use crate::common::hidden_drives;
use crate::toolkit::bus::Props;
use crate::toolkit::bus as dbus;
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use super::{GB, plan};
use super::udisks::DRIVE;
use super::udisks::ATA;
use super::udisks::NVME;
use super::udisks::BLOCK;
use super::udisks::FILESYSTEM;
use super::udisks::PARTITION;
use super::udisks::PARTITION_TABLE;
use super::udisks::Objects;
use super::udisks::text;
use super::udisks::mount_points;
use super::udisks::num;
use super::udisks::real;
use super::udisks::strings;
use super::udisks::fstab_of;
use super::udisks::basename;
use super::udisks::comes_out;

#[derive(Clone)]
pub(super) struct Vol {
    pub(super) path: String,
    pub(super) dev: String,
    pub(super) label: String,
    pub(super) fs_type: String,
    pub(super) usage: String,
    pub(super) uuid: String,
    pub(super) size: u64,
    pub(super) has_fs: bool,
    pub(super) mounts: Vec<String>,

    pub(super) full: Option<(u64, u64)>,

    pub(super) part: Option<(u64, u64, bool)>,
    pub(super) fstab: Option<Fstab>,

    pub(super) system: bool,

    pub(super) part_type: String,

    pub(super) unlocked: bool,

    pub(super) key: String,
}

impl Vol {
    pub(super) fn name(&self) -> String {
        if !self.label.is_empty() {
            self.label.clone()
        } else if !self.dev.is_empty() {
            self.dev.clone()
        } else {
            t!("disks-part-name").into()
        }
    }

    pub(super) fn kind_words(&self) -> String {
        let kind = match (self.fs_type.as_str(), self.usage.as_str()) {
            ("swap", _) => t!("disks-kind-swap").into(),
            (_, "crypto") if self.unlocked => t!("disks-kind-crypto-open").into(),
            (_, "crypto") => t!("disks-kind-crypto-locked").into(),
            ("", _) if self.part.is_some_and(|p| p.2) => t!("disks-kind-container").into(),
            ("", _) => t!("disks-kind-none").into(),

            (other, _) => plan::kinds().iter().find(|k| k.id == other).map_or_else(|| other.to_string(), |k| k.name.to_string()),
        };
        match self.role() {
            Some(role) => t!("disks-kind-role", kind = &kind, role = role),
            None => kind,
        }
    }

    fn role(&self) -> Option<&'static str> {
        match self.part_type.to_lowercase().as_str() {
            "c12a7328-f81f-11d2-ba4b-00a0c93ec93b" | "0xef" => Some("efi"),
            "21686148-6449-6e6f-744e-656564454649" => Some("legacy"),
            "bc13c2ff-59e6-4262-a352-b275fd6f7172" => Some("loader"),
            _ => None,
        }
    }

    pub(super) fn unmounted(&self) -> bool {
        self.has_fs && self.mounts.is_empty() && self.fs_type != "swap"
    }

    pub(super) fn container(&self) -> bool {
        self.part.is_some_and(|p| p.2)
    }

    pub(super) fn nameable(&self) -> bool {
        self.has_fs && !self.uuid.is_empty() && !self.system
    }
}

pub(super) struct Drive {
    pub(super) path: String,
    pub(super) name: String,
    pub(super) size: u64,
    pub(super) removable: bool,
    pub(super) can_power_off: bool,
    pub(super) optical: bool,
    pub(super) whole: Option<String>,
    pub(super) vols: Vec<Vol>,
    pub(super) gaps: Vec<(u64, u64)>,
    pub(super) health: Option<Health>,
    pub(super) system: bool,

    pub(super) key: String,
}

impl Drive {
    pub(super) fn words(&self) -> String {
        t!("disks-drive-words", name = &self.name, size = size_text(self.size as f64))
    }

    pub(super) fn mounted(&self, objects: &Objects) -> Vec<String> {
        let mut mine: Vec<&str> = self.vols.iter().map(|v| v.path.as_str()).collect();
        mine.extend(self.whole.as_deref());
        objects.iter()
            .filter(|(p, o)| {
                o.get(BLOCK).is_some_and(|b| mine.contains(&p.as_str()) || mine.contains(&dbus::get_path(b, "CryptoBackingDevice").as_str()))
                    && o.get(FILESYSTEM).is_some_and(|f| !mount_points(f).is_empty())
            })
            .map(|(p, _)| p.clone())
            .collect()
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct Caps {
    pub(super) format: bool,
    pub(super) resize: u64,
    pub(super) check: bool,
    pub(super) repair: bool,
}

pub(super) fn build(objects: &Objects, system_devs: &HashSet<String>, system_drives: &HashSet<String>,
         full: impl Fn(&str) -> Option<(u64, u64)>) -> Vec<Drive> {
    let mut drives = Vec::new();
    for (path, o) in objects.iter() {
        let Some(d) = o.get(DRIVE) else { continue };
        let name = [dbus::get_str(d, "Vendor"), dbus::get_str(d, "Model")]
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let mine: Vec<(&String, &HashMap<String, Props>)> = objects.iter()
            .filter(|(_, o)| o.get(BLOCK).is_some_and(|b| dbus::get_path(b, "Drive") == *path))
            .collect();
        let whole = mine.iter().find(|(_, o)| !o.contains_key(PARTITION)).map(|(p, _)| p.to_string());
        let whole_props = whole.as_ref().and_then(|w| objects.get(w));
        let table = whole_props.and_then(|o| o.get(PARTITION_TABLE)).map(|t| dbus::get_str(t, "Type")).unwrap_or_default();

        let size = match num(d, "Size") {
            0 => mine.iter().filter_map(|(_, o)| o.get(BLOCK)).map(|b| num(b, "Size")).max().unwrap_or(0),
            n => n,
        };
        let mut vols = volumes_of(objects, &mine, system_devs, &full);
        vols.sort_by_key(|v| v.part.map(|p| p.0).unwrap_or(0));
        let gaps = if table.is_empty() {
            Vec::new()
        } else {
            let parts: Vec<(u64, u64)> = vols.iter().filter_map(|v| v.part.map(|p| (p.0, p.1))).collect();
            plan::gaps(size, &table, &parts)
        };
        drives.push(Drive {
            path: path.clone(),
            name: if name.is_empty() { t!("disks-drive-name").into() } else { name },
            size,
            removable: comes_out(d),
            can_power_off: dbus::get_bool(d, "CanPowerOff"),
            optical: dbus::get_bool(d, "Optical") || strings(d, "MediaCompatibility").iter().any(|m| m.starts_with("optical")),
            whole,
            vols,
            gaps,
            health: health(o),
            system: system_drives.contains(path),
            key: hidden_drives::drive_key(d),
        });
    }
    drives.sort_by(|a, b| (a.removable, &a.name).cmp(&(b.removable, &b.name)));
    drives
}

fn volumes_of(objects: &Objects, mine: &[(&String, &HashMap<String, Props>)], system_devs: &HashSet<String>,
              full: &impl Fn(&str) -> Option<(u64, u64)>) -> Vec<Vol> {
    let mut vols = Vec::new();
    for (p, o) in mine {
        let Some(b) = o.get(BLOCK) else { continue };
        let part = o.get(PARTITION).map(|pt| (num(pt, "Offset"), num(pt, "Size"), dbus::get_bool(pt, "IsContainer")));
        let fs = o.get(FILESYSTEM);
        let usage = dbus::get_str(b, "IdUsage");

        if part.is_none() && fs.is_none() && usage != "crypto" {
            continue;
        }
        let dev = basename(&text(b, "Device"));
        let mounts = fs.map(mount_points).unwrap_or_default();

        vols.push(Vol {
            path: p.to_string(),
            label: dbus::get_str(b, "IdLabel"),
            fs_type: dbus::get_str(b, "IdType"),
            uuid: dbus::get_str(b, "IdUUID"),
            size: num(b, "Size"),
            has_fs: fs.is_some(),
            full: mounts.first().and_then(|m| full(m)),
            mounts,
            part,
            fstab: fstab_of(b),
            system: system_devs.contains(&dev),
            part_type: o.get(PARTITION).map(|pt| dbus::get_str(pt, "Type")).unwrap_or_default(),
            unlocked: usage == "crypto" && objects.values()
                .any(|c| c.get(BLOCK).is_some_and(|cb| dbus::get_path(cb, "CryptoBackingDevice") == **p)),
            key: hidden_drives::part_key(b, o.get(PARTITION)),
            usage,
            dev,
        });
    }
    vols
}

fn health(o: &HashMap<String, Props>) -> Option<Health> {
    if let Some(a) = o.get(ATA) {
        if !dbus::get_bool(a, "SmartSupported") || !dbus::get_bool(a, "SmartEnabled") || num(a, "SmartUpdated") == 0 {
            return None;
        }
        let mut troubles = Vec::new();
        let bad = num(a, "SmartNumBadSectors");
        if bad > 0 {
            troubles.push(t!("disks-health-damaged", n = bad));
        }
        if num(a, "SmartNumAttributesFailing") > 0 {
            troubles.push(t!("disks-trouble-wearing-out").into());
        }
        return Some(Health {
            failing: dbus::get_bool(a, "SmartFailing"),
            troubles,
            kelvin: real(a, "SmartTemperature"),
            hours: num(a, "SmartPowerOnSeconds") / 3600,
        });
    }
    let n = o.get(NVME)?;
    if num(n, "SmartUpdated") == 0 {
        return None;
    }
    let mut troubles: Vec<String> = strings(n, "SmartCriticalWarning").iter()
        .filter_map(|w| plan::nvme_trouble(w))
        .map(String::from)
        .collect();
    if num(n, "SmartPercentUsed") >= 100 {
        troubles.push(t!("disks-trouble-worn").into());
    }
    Some(Health { failing: false, troubles, kelvin: real(n, "SmartTemperature"), hours: num(n, "SmartPowerOnHours") })
}

pub(super) fn headline(drives: &[Drive], home: Option<(u64, u64)>) -> (String, Tone) {
    for d in drives {
        if d.health.as_ref().is_some_and(|h| h.failing) {
            return (t!("disks-headline-failing", name = &d.name), Tone::Problem);
        }
    }
    for d in drives {
        if let Some(h) = d.health.as_ref().filter(|h| !h.troubles.is_empty()) {
            return (t!("disks-headline-look", name = &d.name, troubles = h.troubles.join(", ")), Tone::Warn);
        }
    }
    let plugged = drives.iter().filter(|d| d.removable && !d.system).count();
    let Some((used, free)) = home else {
        return (t!("disks-headline-calm", plugged = plugged), Tone::Calm);
    };
    let share = used as f64 / (used + free) as f64;
    let left = size_text(free as f64);

    if share >= 0.95 || (free as f64) < GB / 2.0 {
        (t!("disks-headline-full", free = &left, plugged = plugged), Tone::Problem)
    } else if share >= 0.85 || (free as f64) < 5.0 * GB {
        (t!("disks-headline-getting-full", free = &left, plugged = plugged), Tone::Warn)
    } else {
        (t!("disks-headline-plenty", free = &left, plugged = plugged), Tone::Plain)
    }
}

pub(super) fn percent(share: f64) -> String {
    format!("{:.0}%", share.clamp(0.0, 1.0) * 100.0)
}
