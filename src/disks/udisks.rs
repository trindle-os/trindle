//! UDisks2 on the system bus.

use std::collections::{BTreeMap, HashMap, HashSet};
use super::plan::Fstab;
use zbus::Message;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use crate::toolkit::bus::Props;
use crate::toolkit::bus as dbus;
use crate::toolkit::t;
use super::plan;

pub(super) const UD: &str = "org.freedesktop.UDisks2";
pub(super) const UD_PATH: &str = "/org/freedesktop/UDisks2";
pub(super) const MANAGER_PATH: &str = "/org/freedesktop/UDisks2/Manager";
pub(super) const MANAGER: &str = "org.freedesktop.UDisks2.Manager";
pub(super) const DRIVE: &str = "org.freedesktop.UDisks2.Drive";
pub(super) const ATA: &str = "org.freedesktop.UDisks2.Drive.Ata";
pub(super) const NVME: &str = "org.freedesktop.UDisks2.NVMe.Controller";
pub(super) const BLOCK: &str = "org.freedesktop.UDisks2.Block";
pub(super) const FILESYSTEM: &str = "org.freedesktop.UDisks2.Filesystem";
pub(super) const PARTITION: &str = "org.freedesktop.UDisks2.Partition";
pub(super) const PARTITION_TABLE: &str = "org.freedesktop.UDisks2.PartitionTable";
pub(super) const SWAPSPACE: &str = "org.freedesktop.UDisks2.Swapspace";
pub(super) const OBJECT_MANAGER: &str = "org.freedesktop.DBus.ObjectManager";

pub(super) type Objects = BTreeMap<String, HashMap<String, Props>>;
pub(super) type Opts = HashMap<&'static str, Value<'static>>;
type Item = (String, HashMap<String, Value<'static>>);

fn trim_nul(mut b: Vec<u8>) -> String {
    while b.last() == Some(&0) {
        b.pop();
    }
    String::from_utf8_lossy(&b).to_string()
}

pub(crate) fn text(p: &Props, k: &str) -> String {
    trim_nul(dbus::get_bytes(p, k))
}

pub(crate) fn mount_points(fs: &Props) -> Vec<String> {
    fs.get("MountPoints")
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| Vec::<Vec<u8>>::try_from(v).ok())
        .unwrap_or_default()
        .into_iter()
        .map(trim_nul)
        .filter(|m| !m.is_empty())
        .collect()
}

pub(crate) fn num(p: &Props, k: &str) -> u64 {
    let Some(v) = p.get(k) else { return 0 };
    u64::try_from(v).ok()
        .or_else(|| i64::try_from(v).ok().map(|n| n.max(0) as u64))
        .or_else(|| u32::try_from(v).ok().map(u64::from))
        .or_else(|| i32::try_from(v).ok().map(|n| n.max(0) as u64))
        .or_else(|| u16::try_from(v).ok().map(u64::from))
        .or_else(|| u8::try_from(v).ok().map(u64::from))
        .unwrap_or(0)
}

pub(super) fn real(p: &Props, k: &str) -> f64 {
    p.get(k).and_then(|v| f64::try_from(v).ok()).unwrap_or_else(|| num(p, k) as f64)
}

pub(super) fn strings(p: &Props, k: &str) -> Vec<String> {
    p.get(k).and_then(|v| v.try_clone().ok()).and_then(|v| Vec::<String>::try_from(v).ok()).unwrap_or_default()
}

pub(super) fn opts(pairs: Vec<(&'static str, Value<'static>)>) -> Opts {
    pairs.into_iter().collect()
}

fn bytes(s: &str) -> Value<'static> {
    let mut b = s.as_bytes().to_vec();
    b.push(0);
    Value::from(b)
}

pub(super) fn fstab_item(e: &Fstab) -> Item {
    let mut d = HashMap::new();
    d.insert("fsname".to_string(), bytes(&e.fsname));
    d.insert("dir".to_string(), bytes(&e.dir));
    d.insert("type".to_string(), bytes(&e.kind));
    d.insert("opts".to_string(), bytes(&e.opts));
    d.insert("freq".to_string(), Value::from(e.freq));
    d.insert("passno".to_string(), Value::from(e.passno));
    ("fstab".to_string(), d)
}

pub(super) fn fstab_of(block: &Props) -> Option<Fstab> {
    let all = block.get("Configuration")?.try_clone().ok()?;
    let all = Vec::<(String, HashMap<String, OwnedValue>)>::try_from(all).ok()?;
    all.into_iter().find(|(kind, _)| kind == "fstab").map(|(_, d)| {
        let num = |k: &str| d.get(k).and_then(|v| i32::try_from(v).ok()).unwrap_or(0);
        Fstab {
            fsname: text(&d, "fsname"),
            dir: text(&d, "dir"),
            kind: text(&d, "type"),
            opts: text(&d, "opts"),
            freq: num("freq"),
            passno: num("passno"),
        }
    })
}

pub(super) fn used_space(path: &str) -> Option<u64> {
    fullness(path).map(|(used, _)| used)
}

pub(super) fn fullness(path: &str) -> Option<(u64, u64)> {
    crate::common::files::how_full(std::path::Path::new(path))
}

pub(super) fn basename(p: &str) -> String {
    p.rsplit('/').next().unwrap_or(p).to_string()
}

pub(super) fn dev_name(path: &str) -> Option<String> {
    std::fs::canonicalize(path).ok().map(|p| basename(&p.to_string_lossy()))
}

pub(super) fn taken_dirs() -> HashSet<String> {
    let mut out: HashSet<String> = plan::fstab_dirs(&std::fs::read_to_string("/etc/fstab").unwrap_or_default())
        .into_iter()
        .collect();
    for l in std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default().lines() {
        if let Some(dir) = l.split(' ').nth(4) {
            out.insert(dir.replace("\\040", " "));
        }
    }
    if let Ok(d) = std::fs::read_dir(plan::MNT) {
        for e in d.flatten() {
            let p = e.path();
            let empty_dir = p.is_dir() && std::fs::read_dir(&p).map(|mut i| i.next().is_none()).unwrap_or(false);
            if !empty_dir {
                out.insert(p.to_string_lossy().to_string());
            }
        }
    }
    out
}

pub(super) fn me_ids() -> (u32, u32) {
    // SAFETY: getuid and getgid cannot fail.
    unsafe { (libc::getuid(), libc::getgid()) }
}

pub(super) fn unknown() -> Option<bool> {
    None
}

pub(super) fn comes_out(d: &Props) -> bool {
    dbus::get_bool(d, "Removable") || dbus::get_bool(d, "Ejectable")
        || dbus::get_bool(d, "MediaRemovable") || dbus::get_str(d, "ConnectionBus") == "usb"
}

pub(super) fn parse_objects(m: &Message) -> Result<Objects, String> {
    let (all,) = m.body()
        .deserialize::<(HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>,)>()
        .map_err(|e| e.to_string())?;
    Ok(all.into_iter().map(|(p, o)| (p.to_string(), o)).collect())
}

pub(super) fn take_snapshot(desc: String, done: Box<dyn FnOnce(Result<(), String>)>) {
    use crate::common::run::{admin_error, admin_helper, capture};
    use crate::common::snapshots;
    if cfg!(test) {
        return;
    }
    let listed = desc.clone();
    capture(&["pkexec", &admin_helper(), "snapshot-create", &desc], None, move |ok, _, err| {
        if !ok {
            return done(Err(admin_error(&err)));
        }
        capture(&snapshots::list_argv(), None, move |ok, out, _| {
            let found = ok && snapshots::parse_list(&out).iter().any(|s| s.description == listed);
            done(if found { Ok(()) } else { Err(t!("disks-snapshot-missing").into()) });
        });
    });
}
