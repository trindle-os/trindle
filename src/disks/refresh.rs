//! Loading the drives.

use std::collections::{HashMap, HashSet};
use std::time::Duration;
use zbus::zvariant::Value;
use crate::common::hidden_drives;
use crate::toolkit::bus as dbus;
use crate::toolkit::t;
use super::{Do, Drives, Ring, plan};
use super::udisks::UD;
use super::udisks::UD_PATH;
use super::udisks::MANAGER_PATH;
use super::udisks::MANAGER;
use super::udisks::DRIVE;
use super::udisks::ATA;
use super::udisks::NVME;
use super::udisks::BLOCK;
use super::udisks::FILESYSTEM;
use super::udisks::SWAPSPACE;
use super::udisks::OBJECT_MANAGER;
use super::udisks::Objects;
use super::udisks::Opts;
use super::udisks::text;
use super::udisks::mount_points;
use super::udisks::num;
use super::udisks::opts;
use super::udisks::fullness;
use super::udisks::basename;
use super::udisks::dev_name;
use super::udisks::comes_out;
use super::udisks::parse_objects;
use super::model::Drive;
use super::model::Caps;
use super::model::build;
use super::model::headline;
use super::steps::Said;

impl Drives {
    fn model(&self) -> Vec<Drive> {
        let objects = &self.objects;
        let dev_of = |p: &str| objects.get(p).and_then(|o| o.get(BLOCK)).map(|b| basename(&text(b, "Device")));

        let info = std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default();
        let mut seeds = Vec::new();
        let mut root_traced = false;
        for m in plan::system_mounts(&info) {
            let dev = if m.source.starts_with("/dev/") { dev_name(&m.source) } else { None }
                .or_else(|| {
                    (!m.major_minor.starts_with("0:")).then(|| {
                        std::fs::read_link(format!("/sys/dev/block/{}", m.major_minor)).ok()
                            .map(|p| basename(&p.to_string_lossy()))
                    }).flatten()
                });
            if let Some(d) = dev {
                if m.dir == "/" || m.dir.contains("live") {
                    root_traced = true;
                }
                seeds.push(d);
            }
        }
        let swaps = std::fs::read_to_string("/proc/swaps").unwrap_or_default();
        seeds.extend(plan::swap_devices(&swaps).iter().filter_map(|d| dev_name(d)));
        let mut backing: HashMap<String, String> = HashMap::new();
        for o in objects.values() {
            let Some(b) = o.get(BLOCK) else { continue };
            let dev = basename(&text(b, "Device"));
            if o.get(SWAPSPACE).is_some_and(|s| dbus::get_bool(s, "Active")) {
                seeds.push(dev.clone());
            }
            if o.get(FILESYSTEM).is_some_and(|f| mount_points(f).iter().any(|m| plan::SYSTEM_MOUNTS.contains(&m.as_str()))) {
                seeds.push(dev.clone());
            }
            if let Some(under) = dev_of(&dbus::get_path(b, "CryptoBackingDevice")) {
                backing.insert(dev, under);
            }
        }
        let system_devs = plan::underneath(&seeds, |d| {
            let mut v = crate::common::volumes::slaves(d);
            v.extend(backing.get(d).cloned());
            v
        });

        let blocks: Vec<(String, String)> = objects.values()
            .filter_map(|o| o.get(BLOCK).map(|b| (basename(&text(b, "Device")), dbus::get_path(b, "Drive"))))
            .collect();
        let on: Vec<plan::OnDrive> = blocks.iter()
            .map(|(d, dr)| plan::OnDrive { dev: d, drive: if dr == "/" { "" } else { dr } })
            .collect();
        let internal: Vec<&str> = objects.iter()
            .filter(|(_, o)| o.get(DRIVE).is_some_and(|d| !comes_out(d)))
            .map(|(p, _)| p.as_str())
            .collect();
        let system_drives = plan::system_drives(&on, &system_devs, root_traced, &internal);
        build(objects, &system_devs, &system_drives, fullness)
    }

    pub(super) fn kinds(&self) -> Vec<(&'static str, String)> {
        plan::kinds().iter()
            .filter(|k| self.caps.get(k.id).is_some_and(|c| c.format))
            .map(|k| (k.id, k.name.to_string()))
            .collect()
    }

    pub(super) fn refresh(&self) {
        let Some(bus) = self.bus.clone() else { return };
        let asks = self.asks.clone();
        bus.call(UD, UD_PATH, OBJECT_MANAGER).method("GetManagedObjects", &(), false, Some(Duration::from_secs(25)), &self.events, move |r| {
            asks.send(Do::Objects(r.map_err(|e| e.to_string()).and_then(|m| parse_objects(&m))));
        });
    }

    pub(super) fn got(&mut self, objects: Result<Objects, String>) {
        let Ok(objects) = objects else {
            self.missing = Some((t!("disks-missing-service"), "udisks2"));
            return;
        };
        self.missing = None;
        let mut kinds: HashSet<String> = plan::kinds().iter().map(|k| k.id.to_string()).collect();
        for o in objects.values() {
            if let (Some(b), Some(_)) = (o.get(BLOCK), o.get(FILESYSTEM)) {
                kinds.insert(dbus::get_str(b, "IdType"));
            }
        }
        let missing: Vec<String> = kinds.into_iter().filter(|k| !k.is_empty() && !self.caps.contains_key(k)).collect();
        self.objects = objects;
        if missing.is_empty() {
            self.draw();
        } else {
            self.ask_caps(missing);
        }
    }

    fn ask_caps(&mut self, kinds: Vec<String>) {
        let Some(bus) = self.bus.clone() else { return };
        for k in kinds {
            self.caps.insert(k.clone(), Caps::default());
            for method in ["CanFormat", "CanResize", "CanCheck", "CanRepair"] {
                let (asks, kind) = (self.asks.clone(), k.clone());
                bus.call(UD, MANAGER_PATH, MANAGER).method(method, &(k.as_str(),), false, Some(Duration::from_secs(25)), &self.events, move |r| {
                    let yes = r.ok().and_then(|m| {
                        if method == "CanResize" {
                            m.body().deserialize::<((bool, u64, String),)>().ok().map(|((y, f, _),)| (y, f))
                        } else {
                            m.body().deserialize::<((bool, String),)>().ok().map(|((y, _),)| (y, 0))
                        }
                    });
                    asks.send(Do::Cap { kind, method, yes });
                });
            }
        }
    }

    pub(super) fn cap(&mut self, kind: &str, method: &str, yes: Option<(bool, u64)>) {
        let c = self.caps.entry(kind.to_string()).or_default();
        let (y, flags) = yes.unwrap_or((false, 0));
        match method {
            "CanFormat" => c.format = y,
            "CanResize" => c.resize = if y { flags } else { 0 },
            "CanCheck" => c.check = y,
            _ => c.repair = y,
        }
        self.draw();
    }

    fn ask_health(&mut self, path: &str) {
        let Some(o) = self.objects.get(path) else { return };
        let (iface, o) = if let Some(a) = o.get(ATA) {
            if !dbus::get_bool(a, "SmartSupported") || !dbus::get_bool(a, "SmartEnabled") || num(a, "SmartUpdated") != 0 {
                return;
            }
            (ATA, opts(vec![("nowakeup", Value::from(true))]))
        } else if let Some(n) = o.get(NVME) {
            if num(n, "SmartUpdated") != 0 {
                return;
            }
            (NVME, Opts::new())
        } else {
            return;
        };
        if !self.asked_health.insert(path.to_string()) {
            return;
        }
        let Some(bus) = self.bus.clone() else { return };
        let (asks, gen) = (self.asks.clone(), self.soon);
        bus.call(UD, path, iface).method("SmartUpdate", &(o,), false, Some(Duration::from_secs(30)), &self.events, move |r| {
            if r.is_ok() {
                asks.send(Do::Soon(gen));
            }
        });
    }

    fn draw(&mut self) {
        self.loaded = true;
        self.said.retain(Said::fresh);
        self.hidden = hidden_drives::read_in(&self.hidden_file);
        self.drives = self.model();
        if self.open.as_deref().is_some_and(|k| !self.names(k)) {
            self.open = None;
        }
        self.sum_up();
        let paths: Vec<String> = self.drives.iter().map(|d| d.path.clone()).collect();
        for p in paths {
            self.ask_health(&p);
        }
    }

    pub(super) fn weigh(&mut self) {
        for d in &mut self.drives {
            for v in &mut d.vols {
                v.full = v.mounts.first().and_then(|m| fullness(m));
            }
        }
        self.sum_up();
    }

    pub(super) fn sum_up(&mut self) {
        let root = fullness("/");
        let home = fullness("/home").filter(|h| Some(*h) != root);
        let mut rings = Vec::new();
        let mut add = |name: &str, icon: &'static str, (used, free): (u64, u64), place: &'static str| {
            rings.push(Ring { name: name.into(), icon, share: used as f64 / (used + free) as f64, free, whole: used + free, words: place });
        };
        if let Some(r) = root {
            add(t!("disks-ring-system"), "computer-symbolic", r, "system");
        }
        if let Some(h) = home {
            add(t!("disks-ring-files"), "folder-symbolic", h, "files");
        }
        self.rings = rings;
        let failing = self.drives.iter().filter(|d| d.health.as_ref().is_some_and(|h| h.failing || !h.troubles.is_empty())).count();
        let read = self.drives.iter().filter(|d| d.health.is_some()).count();
        self.health_said = match (failing, read) {
            (0, 0) => t!("disks-health-none").to_string(),
            (0, 1) => t!("disks-health-one").to_string(),
            (0, n) => t!("disks-health-all", n = n),
            (n, _) => t!("disks-health-look", n = n),
        };
        self.headline = headline(&self.drives, home.or(root));
    }
}
