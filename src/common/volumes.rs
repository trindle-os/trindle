//! Plugged-in drives as UDisks reports them, and which to offer.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::time::Duration;

use zbus::zvariant::{OwnedObjectPath, Value};
use zeroize::Zeroizing;

use super::hidden_drives;
use crate::disks::plan::{self, Mounted};
use crate::disks::{mount_points, num, text};
use crate::toolkit::bus::{self as dbus, Bus, Failed, Listen, Props};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::window::Display;
use crate::keyring::prompt;

pub const UD: &str = "org.freedesktop.UDisks2";
const UD_PATH: &str = "/org/freedesktop/UDisks2";
const OBJECT_MANAGER: &str = "org.freedesktop.DBus.ObjectManager";
const PROPERTIES: &str = "org.freedesktop.DBus.Properties";
const DRIVE: &str = "org.freedesktop.UDisks2.Drive";
pub const BLOCK: &str = "org.freedesktop.UDisks2.Block";
pub const FILESYSTEM: &str = "org.freedesktop.UDisks2.Filesystem";
pub const ENCRYPTED: &str = "org.freedesktop.UDisks2.Encrypted";
const PARTITION: &str = "org.freedesktop.UDisks2.Partition";
const SWAPSPACE: &str = "org.freedesktop.UDisks2.Swapspace";

const SETTLE: Duration = Duration::from_millis(300);
const INTERACTIVE: Duration = Duration::from_secs(120);

pub type Objects = BTreeMap<String, HashMap<String, Props>>;
type Opts = HashMap<&'static str, Value<'static>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Usb,
    Card,
    Disc,
    Other,
}

impl Kind {
    pub fn arrived(self) -> &'static str {
        match self {
            Kind::Usb => "USB drive connected",
            Kind::Card => "Memory card connected",
            Kind::Disc => "Disc inserted",
            Kind::Other => "Drive connected",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Volume {
    pub id: String,
    pub dev: String,
    pub label: String,
    pub size: u64,
    pub drive: String,
    pub drive_name: String,
    pub kind: Kind,
    pub mounts: Vec<String>,
    pub locked: bool,
    pub container: Option<String>,
    pub can_eject: bool,
    pub can_power_off: bool,
    pub fixed: bool,
    pub system: bool,
    pub key: String,
    pub drive_key: String,
}

impl Volume {
    pub fn name(&self) -> String {
        if self.label.trim().is_empty() { size_words(self.size) } else { self.label.trim().to_string() }
    }

    pub fn notice(&self) -> String {
        format!("{}: {}", self.kind.arrived(), self.name())
    }

    pub fn mounted(&self) -> Option<&str> {
        self.mounts.first().map(String::as_str)
    }

}

pub fn size_words(n: u64) -> String {
    trindle_toolkit::lang::format::size(n)
}

pub fn exact_size_words(bytes: u64) -> String {
    trindle_toolkit::lang::format::size_exact(bytes)
}

#[derive(Default, Debug, PartialEq)]
pub struct Boot {
    pub devs: HashSet<String>,
    pub drives: HashSet<String>,
}

fn basename(p: &str) -> String {
    p.rsplit('/').next().unwrap_or(p).to_string()
}

fn device_named(what: &str) -> Option<String> {
    if what.starts_with("/dev/") {
        return std::fs::canonicalize(what).ok().map(|p| basename(&p.to_string_lossy()));
    }
    if what.contains(':') && !what.starts_with("0:") {
        return std::fs::read_link(format!("/sys/dev/block/{what}")).ok().map(|p| basename(&p.to_string_lossy()));
    }
    None
}

pub fn slaves(dev: &str) -> Vec<String> {
    std::fs::read_dir(format!("/sys/class/block/{dev}/slaves"))
        .map(|d| d.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect())
        .unwrap_or_default()
}

pub fn boot_now(objects: &Objects) -> Boot {
    let info = std::fs::read_to_string("/proc/self/mountinfo").unwrap_or_default();
    let swaps = std::fs::read_to_string("/proc/swaps").unwrap_or_default();
    boot_of(objects, &info, &swaps, device_named, slaves)
}

pub fn boot_of(objects: &Objects, mountinfo: &str, swaps: &str, named: impl Fn(&str) -> Option<String>,
               below: impl Fn(&str) -> Vec<String>) -> Boot {
    let mut seeds: Vec<String> = Vec::new();
    for Mounted { major_minor, source, .. } in plan::system_mounts(mountinfo) {
        if let Some(d) = named(&source).or_else(|| named(&major_minor)) {
            seeds.push(d);
        }
    }
    seeds.extend(plan::swap_devices(swaps).iter().filter_map(|d| named(d)));
    let dev_of = |p: &str| objects.get(p).and_then(|o| o.get(BLOCK)).map(|b| basename(&text(b, "Device")));
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
    let devs = plan::underneath(&seeds, |d| {
        let mut v = below(d);
        v.extend(backing.get(d).cloned());
        v
    });
    let drives = objects.values()
        .filter_map(|o| o.get(BLOCK))
        .filter(|b| devs.contains(&basename(&text(b, "Device"))))
        .map(|b| dbus::get_path(b, "Drive"))
        .filter(|d| !d.is_empty() && d != "/")
        .collect();
    Boot { devs, drives }
}

fn removable(d: &Props) -> bool {
    dbus::get_bool(d, "Removable") || dbus::get_bool(d, "Ejectable") || dbus::get_bool(d, "MediaRemovable")
        || matches!(dbus::get_str(d, "ConnectionBus").as_str(), "usb" | "sdio" | "ieee1394")
}

fn kind_of(d: &Props) -> Kind {
    let media = dbus::get_str(d, "Media");
    if dbus::get_bool(d, "Optical") || media.starts_with("optical") {
        Kind::Disc
    } else if dbus::get_str(d, "ConnectionBus") == "sdio" || media.starts_with("flash_sd") || media.starts_with("flash_mmc")
        || media.starts_with("flash_ms")
    {
        Kind::Card
    } else if dbus::get_str(d, "ConnectionBus") == "usb" {
        Kind::Usb
    } else {
        Kind::Other
    }
}

pub fn volumes(objects: &Objects, boot: &Boot) -> Vec<Volume> {
    listed(objects, boot, false)
}

pub fn all(objects: &Objects, boot: &Boot) -> Vec<Volume> {
    listed(objects, boot, true)
}

const OWN_KINDS: [&str; 8] = ["c12a7328-f81f-11d2-ba4b-00a0c93ec93b", "21686148-6449-6e6f-744e-656564454649",
                              "bc13c2ff-59e6-4262-a352-b275fd6f7172", "e3c9e316-0b5c-4db8-817d-f92df00215ae",
                              "de94bba4-06d1-4d40-a16a-bfd50179d6ac", "0xef", "0x27", "0x12"];
const OWN_MOUNTS: [&str; 7] = ["/boot", "/boot/efi", "/efi", "/usr", "/var", "/run/live/medium", "/lib/live/mount/medium"];
const OWN_LABELS: [&str; 6] = ["recovery", "winre", "winre_drv", "system reserved", "esp", "efi"];

fn own_part(o: &HashMap<String, Props>, b: &Props, mounts: &[String]) -> bool {
    let kind = o.get(PARTITION).map(|p| dbus::get_str(p, "Type").to_lowercase()).unwrap_or_default();
    let label = dbus::get_str(b, "IdLabel").trim().to_lowercase();
    OWN_KINDS.contains(&kind.as_str()) || OWN_LABELS.contains(&label.as_str())
        || (!mounts.is_empty() && mounts.iter().all(|m| OWN_MOUNTS.contains(&m.as_str())))
}

fn listed(objects: &Objects, boot: &Boot, fixed_too: bool) -> Vec<Volume> {
    let mut out: Vec<Volume> = objects.iter().filter_map(|(path, o)| volume_of(objects, boot, fixed_too, path, o)).collect();
    out.sort_by(|a, b| (&a.drive_name, &a.drive, &a.dev).cmp(&(&b.drive_name, &b.drive, &b.dev)));
    out
}

struct Part<'a> {
    o: &'a HashMap<String, Props>,
    b: &'a Props,
    d: &'a Props,
    drive: String,
    drive_name: String,
    fixed: bool,
}

impl Part<'_> {
    fn volume(&self, id: &str, dev: String, from: &Props, fs: &Props, container: Option<String>, locked: bool) -> Volume {
        let (can_eject, can_power_off) = (dbus::get_bool(self.d, "Ejectable"), dbus::get_bool(self.d, "CanPowerOff"));
        Volume {
            id: id.to_string(),
            dev,
            label: dbus::get_str(from, "IdLabel"),
            size: num(from, "Size"),
            drive: self.drive.clone(),
            drive_name: if self.drive_name.is_empty() { "Drive".into() } else { self.drive_name.clone() },
            kind: kind_of(self.d),
            mounts: mount_points(fs),
            locked,
            container,
            can_eject: can_eject && !self.fixed,
            can_power_off: can_power_off && !self.fixed,
            fixed: self.fixed,
            system: false,
            key: hidden_drives::part_key(self.b, self.o.get(PARTITION)),
            drive_key: hidden_drives::drive_key(self.d),
        }
    }
}

fn volume_of(objects: &Objects, boot: &Boot, fixed_too: bool, path: &String, o: &HashMap<String, Props>) -> Option<Volume> {
    let b = o.get(BLOCK)?;
    if !dbus::get_path(b, "CryptoBackingDevice").trim_start_matches('/').is_empty() {
        return None;
    }
    if dbus::get_bool(b, "HintIgnore") {
        return None;
    }
    let drive = dbus::get_path(b, "Drive");
    let d = objects.get(&drive).and_then(|d| d.get(DRIVE))?;
    let dev = basename(&text(b, "Device"));
    let fixed = dbus::get_bool(b, "HintSystem") || boot.devs.contains(&dev) || boot.drives.contains(&drive) || !removable(d);
    if fixed && !fixed_too {
        return None;
    }
    let id_type = dbus::get_str(b, "IdType");
    if o.contains_key(SWAPSPACE) || id_type == "swap" || o.get(PARTITION).is_some_and(|p| dbus::get_bool(p, "IsContainer")) {
        return None;
    }
    let drive_name = [dbus::get_str(d, "Vendor"), dbus::get_str(d, "Model")]
        .iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ");
    let part = Part { o, b, d, drive, drive_name, fixed };
    let found = if o.contains_key(ENCRYPTED) || dbus::get_str(b, "IdUsage") == "crypto" {
        encrypted(objects, path, dev, &part)?
    } else {
        part.volume(path, dev, b, o.get(FILESYSTEM)?, None, false)
    };
    if fixed && own_part(o, b, &found.mounts) {
        return None;
    }
    let system = fixed && found.mounts.iter().any(|m| m == "/");
    Some(Volume { system, ..found })
}

fn encrypted(objects: &Objects, path: &String, dev: String, part: &Part) -> Option<Volume> {
    let b = part.b;
    let open = objects.iter().find(|(_, c)| c.get(BLOCK).is_some_and(|cb| dbus::get_path(cb, "CryptoBackingDevice") == *path));
    Some(match open {
        Some((cp, c)) => {
            let (Some(cb), Some(fs)) = (c.get(BLOCK), c.get(FILESYSTEM)) else { return None };
            let mut v = part.volume(cp, dev, cb, fs, Some(path.clone()), false);
            if v.label.is_empty() {
                v.label = dbus::get_str(b, "IdLabel");
            }
            v.size = num(b, "Size");
            v
        }
        None => part.volume(path, dev, b, &Props::new(), Some(path.clone()), true),
    })
}

pub fn unhidden(list: Vec<Volume>, hidden: &BTreeSet<String>) -> Vec<Volume> {
    if hidden.is_empty() {
        return list;
    }
    list.into_iter().filter(|v| v.system || !hidden_drives::hides(hidden, &v.drive_key, &v.key)).collect()
}

pub fn best<'a>(parts: &[&'a Volume]) -> Option<&'a Volume> {
    parts.iter().copied().max_by_key(|v| (!v.locked, v.size))
}

pub fn words(e: &Failed) -> String {
    let all = e.to_string();
    let l = all.to_lowercase();
    if l.contains("no key available") || l.contains("failed to activate") || l.contains("incorrect passphrase")
        || l.contains("bad passphrase") || l.contains("operation not permitted")
    {
        "That passphrase did not open it.".into()
    } else if l.contains("already mounted") {
        "It is already open.".into()
    } else {
        plan::plain_error(&all)
    }
}

pub fn mount(bus: &Bus, lp: &Loop, v: &Volume, done: impl FnOnce(Result<String, String>) + 'static) {
    match v.mounted() {
        Some(m) => done(Ok(m.to_string())),
        None if v.locked => done(Err("A passphrase is needed to open it.".into())),
        None => mount_at(bus, lp, &v.id, done),
    }
}

pub fn unlock(bus: &Bus, lp: &Loop, id: &str, passphrase: &str, done: impl FnOnce(Result<String, String>) + 'static) {
    bus.call(UD, id, ENCRYPTED).method("Unlock", &(passphrase, Opts::new()), true, Some(INTERACTIVE), lp, move |r| {
        done(r.map_err(|e| words(&e)).and_then(|m| {
            m.body().deserialize::<(OwnedObjectPath,)>().map(|b| b.0.to_string()).map_err(|e| e.to_string())
        }))
    });
}

pub fn ask_and_mount(bus: &Bus, lp: &Loop, display: &Rc<Display>, v: &Volume,
                     done: impl FnOnce(Result<String, String>) + 'static) {
    type Done = Box<dyn FnOnce(Result<String, String>)>;
    let finish: Rc<RefCell<Option<Done>>> = Rc::new(RefCell::new(Some(Box::new(done))));
    let opened: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let (b, l, id, at) = (bus.clone(), lp.clone(), v.id.clone(), opened.clone());
    let check = move |pass: Zeroizing<String>, answer: prompt::Answer| {
        let at = at.clone();
        unlock(&b, &l, &id, pass.as_str(), move |r| match r {
            Ok(open) => {
                *at.borrow_mut() = Some(open);
                answer(Ok(()));
            }
            Err(why) => answer(Err(why)),
        });
    };
    let (b, l, f) = (bus.clone(), lp.clone(), finish.clone());
    let closed = move |ok: bool| {
        let open = opened.borrow_mut().take();
        let Some(done) = f.borrow_mut().take() else { return };
        match (ok, open) {
            (true, Some(open)) => mount_at(&b, &l, &open, done),
            _ => done(Err(String::new())),
        }
    };
    let why = format!("Passphrase for {}", v.name());
    if prompt::ask(lp, display, "Unlock drive", &why, None, check, closed).is_none() {
        if let Some(done) = finish.borrow_mut().take() {
            done(Err("The passphrase window would not open.".into()));
        }
    }
}

pub fn mount_at(bus: &Bus, lp: &Loop, path: &str, done: impl FnOnce(Result<String, String>) + 'static) {
    bus.call(UD, path, FILESYSTEM).method("Mount", &(Opts::new(),), true, Some(INTERACTIVE), lp, move |r| {
        done(r.map_err(|e| words(&e)).and_then(|m| m.body().deserialize::<(String,)>().map(|b| b.0).map_err(|e| e.to_string())))
    });
}

struct State {
    bus: Bus,
    lp: Loop,
    last: RefCell<Option<Vec<Volume>>>,
    due: Cell<bool>,
    changed: RefCell<Box<dyn FnMut(&[Volume])>>,
    hears: RefCell<Vec<Listen>>,
    fixed_too: bool,
    hidden: PathBuf,
}

pub struct Watching {
    _state: Rc<State>,
}

impl Watching {
    #[cfg(test)]
    pub fn now(&self) -> Vec<Volume> {
        self._state.last.borrow().clone().unwrap_or_default()
    }
}

pub fn watch(bus: &Bus, lp: &Loop, changed: impl FnMut(&[Volume]) + 'static) -> Watching {
    watching(bus, lp, false, &hidden_file(), changed)
}

fn hidden_file() -> PathBuf {
    if cfg!(test) {
        std::env::temp_dir().join(format!("trindle-volumes-none-{}", std::process::id())).join(hidden_drives::FILE)
    } else {
        hidden_drives::file()
    }
}

fn watching(bus: &Bus, lp: &Loop, fixed_too: bool, hidden: &Path, changed: impl FnMut(&[Volume]) + 'static) -> Watching {
    let state = Rc::new(State {
        fixed_too,
        hidden: hidden.to_path_buf(),
        bus: bus.clone(),
        lp: lp.clone(),
        last: RefCell::new(None),
        due: Cell::new(false),
        changed: RefCell::new(Box::new(changed)),
        hears: RefCell::new(Vec::new()),
    });
    let mut hears = Vec::new();
    for member in ["InterfacesAdded", "InterfacesRemoved"] {
        let w = Rc::downgrade(&state);
        let heard = bus.listen(Some(UD), OBJECT_MANAGER, Some(member), Some(UD_PATH), move |msg| {
            if !is_job(msg) {
                State::soon(&w);
            }
        });
        hears.extend(heard.ok());
    }
    let w = Rc::downgrade(&state);
    let heard = bus.listen(Some(UD), PROPERTIES, Some("PropertiesChanged"), None, move |msg| {
        let body = msg.body();
        let Ok((iface, _, _)) = body.deserialize::<(String, Props, Vec<String>)>() else { return };
        if iface == FILESYSTEM || iface == BLOCK || iface == DRIVE || iface == ENCRYPTED {
            State::soon(&w);
        }
    });
    hears.extend(heard.ok());
    let w = Rc::downgrade(&state);
    hears.extend(bus.watch(UD, lp, move |_| State::soon(&w), || {}).ok());
    *state.hears.borrow_mut() = hears;
    watch_hidden(&state, lp);
    State::read(&state);
    Watching { _state: state }
}

fn watch_hidden(state: &Rc<State>, lp: &Loop) {
    let Some(folder) = state.hidden.parent().map(Path::to_path_buf) else { return };
    let _ = std::fs::create_dir_all(&folder);
    let Ok(mut watch) = crate::toolkit::watch::Watch::new() else { return };
    if watch.folder(&folder).is_err() {
        return;
    }
    let (w, file) = (Rc::downgrade(state), state.hidden.clone());
    lp.watch(watch.fd(), move |hung_up| {
        if hung_up || w.upgrade().is_none() {
            return Flow::Stop;
        }
        if watch.changed().contains(&file) {
            State::soon(&w);
        }
        Flow::Continue
    });
}

pub fn is_job(msg: &zbus::Message) -> bool {
    let body = msg.body();
    let path = body.deserialize::<(OwnedObjectPath, HashMap<String, Props>)>().ok().map(|b| b.0)
        .or_else(|| body.deserialize::<(OwnedObjectPath, Vec<String>)>().ok().map(|b| b.0));
    path.is_some_and(|p| p.as_str().contains("/jobs/"))
}

impl State {
    fn soon(w: &Weak<State>) {
        let Some(s) = w.upgrade() else { return };
        if !s.due.replace(true) {
            let w = w.clone();
            s.lp.after(SETTLE, move || {
                if let Some(s) = w.upgrade() {
                    s.due.set(false);
                    State::read(&s);
                }
            });
        }
    }

    fn read(s: &Rc<State>) {
        let w = Rc::downgrade(s);
        s.bus.call(UD, UD_PATH, OBJECT_MANAGER)
            .method("GetManagedObjects", &(), false, Some(Duration::from_secs(25)), &s.lp, move |r| {
                let Some(s) = w.upgrade() else { return };
                let objects = r.ok().and_then(|m| {
                    m.body().deserialize::<(HashMap<OwnedObjectPath, HashMap<String, Props>>,)>().ok()
                }).map(|(all,)| all.into_iter().map(|(p, o)| (p.to_string(), o)).collect::<Objects>());
                let list = objects.map(|o| {
                    let boot = boot_now(&o);
                    let list = if s.fixed_too { all(&o, &boot) } else { volumes(&o, &boot) };
                    unhidden(list, &hidden_drives::read_in(&s.hidden))
                }).unwrap_or_default();
                if s.last.borrow().as_ref() == Some(&list) {
                    return;
                }
                *s.last.borrow_mut() = Some(list.clone());
                (s.changed.borrow_mut())(&list);
            });
    }
}

#[cfg(test)]
mod tests;
