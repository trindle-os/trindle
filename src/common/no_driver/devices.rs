//! Every PCI and USB device under /sys, grouped by kind, with its name from
//! pci.ids and usb.ids and whether a driver works. Faults are the finds
//! `scan_in` makes, so the words are the ones the other pages say.

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use super::{bound, class_of_pci, hex, id_pair, id_text, kernel_log, known_usb, names_in, scan_in, words, Class};
use crate::toolkit::t;

const PCI_IDS: &str = "/usr/share/misc/pci.ids";
const USB_IDS: &str = "/usr/share/misc/usb.ids";

const IDS_MOST: u64 = 8 << 20;
const LINE_MOST: usize = 300;
const NAME_MOST: usize = 80;
const DEVICES_MOST: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Graphics,
    Sound,
    Network,
    Bluetooth,
    Storage,
    Input,
    Camera,
    Other,
}

pub const KINDS: [Kind; 8] = [Kind::Graphics, Kind::Sound, Kind::Network, Kind::Bluetooth, Kind::Storage, Kind::Input, Kind::Camera, Kind::Other];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Working,
    NoDriver,
    NoFirmware,

    Idle,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Device {
    pub kind: Kind,

    pub id: String,

    pub name: Option<String>,
    pub driver: Option<String>,
    pub state: State,

    pub problem: Option<String>,
}

pub struct Inventory {
    pub devices: Vec<Device>,

    pub loose: Vec<String>,
}

impl Inventory {

    pub fn problems(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for l in self.devices.iter().filter_map(|d| d.problem.as_ref()).chain(self.loose.iter()) {
            if !v.contains(l) {
                v.push(l.clone());
            }
        }
        v
    }

    pub fn of_kind(&self, kind: Kind) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.kind == kind).collect()
    }
}

fn kind_of_class(c: Class) -> Kind {
    match c {
        Class::Wifi => Kind::Network,
        Class::Bluetooth => Kind::Bluetooth,
        Class::Sound => Kind::Sound,
        Class::Graphics => Kind::Graphics,
    }
}

fn kind_of_pci(code: u32) -> Kind {
    if let Some(c) = class_of_pci(code) {
        return kind_of_class(c);
    }
    match code >> 16 {
        0x01 => Kind::Storage,
        0x02 => Kind::Network,
        0x09 => Kind::Input,
        _ => Kind::Other,
    }
}

fn kind_of_usb_interface(class: u32, sub: u32, protocol: u32) -> Kind {
    match (class, sub, protocol) {
        (0xe0, 1, 1) => Kind::Bluetooth,
        (0x01, _, _) => Kind::Sound,
        (0x03, _, _) => Kind::Input,
        (0x08, _, _) => Kind::Storage,
        (0x0e, _, _) => Kind::Camera,
        (0x02 | 0x0a | 0xe0, _, _) => Kind::Network,
        _ => Kind::Other,
    }
}

fn tidy(name: &str) -> String {
    name.chars().filter(|c| !c.is_control()).take(NAME_MOST).collect::<String>().trim().to_string()
}

fn names_from(text: &str, wanted: &[(u16, u16)]) -> HashMap<(u16, u16), String> {
    let mut out = HashMap::new();
    let mut vendor: Option<(u16, String)> = None;
    for l in text.lines() {
        if l.starts_with("C ") {
            break;
        }
        if l.len() > LINE_MOST || l.starts_with('#') || l.starts_with("\t\t") {
            continue;
        }
        let (is_device, rest) = match l.strip_prefix('\t') {
            Some(r) => (true, r),
            None => (false, l),
        };
        let Some((num, name)) = rest.split_once("  ") else { continue };
        let (4, Ok(n)) = (num.len(), u16::from_str_radix(num, 16)) else { continue };
        if !is_device {
            vendor = Some((n, tidy(name)));
        } else if let Some((v, vname)) = &vendor {
            if wanted.contains(&(*v, n)) {
                out.insert((*v, n), format!("{vname} {}", tidy(name)));
            }
        }
    }
    out
}

fn ids_text(path: &Path) -> String {
    let mut bytes = Vec::new();
    if let Ok(f) = std::fs::File::open(path) {
        let _ = f.take(IDS_MOST).read_to_end(&mut bytes);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn driver_name(dir: &Path) -> Option<String> {
    let target = std::fs::read_link(dir.join("driver")).ok()?;
    Some(tidy(&target.file_name()?.to_string_lossy())).filter(|n| !n.is_empty())
}

struct Raw {
    kind: Kind,
    id: (u16, u16),
    driver: Option<String>,
    bound: bool,

    expected: bool,
    usb: bool,
}

const NEEDS_NO_DRIVER: [u32; 2] = [0x05, 0x06];

fn pci_devices(sys: &Path) -> Vec<Raw> {
    let dir = sys.join("bus/pci/devices");
    let mut out = Vec::new();
    for n in names_in(&dir).into_iter().take(DEVICES_MOST) {
        let d = dir.join(&n);
        let (Some(code), Some(id)) = (hex(&d.join("class")), id_pair(&d, "vendor", "device")) else { continue };
        out.push(Raw { kind: kind_of_pci(code), id, driver: driver_name(&d), bound: bound(&d), expected: !NEEDS_NO_DRIVER.contains(&(code >> 16)), usb: false });
    }
    out
}

fn usb_devices(sys: &Path) -> Vec<Raw> {
    let dir = sys.join("bus/usb/devices");
    let all = names_in(&dir);
    let mut out = Vec::new();
    for n in all.iter().filter(|n| !n.contains(':') && !n.starts_with("usb")).take(DEVICES_MOST) {
        let d = dir.join(n);
        let Some(id) = id_pair(&d, "idVendor", "idProduct") else { continue };
        let faces: Vec<_> = all.iter().filter(|f| f.starts_with(&format!("{n}:"))).map(|f| dir.join(f)).collect();
        let by_face = || faces.iter().find_map(|f| {
            let k = kind_of_usb_interface(hex(&f.join("bInterfaceClass"))?, hex(&f.join("bInterfaceSubClass"))?, hex(&f.join("bInterfaceProtocol"))?);
            (k != Kind::Other).then_some(k)
        });
        let kind = known_usb(id).map(|k| kind_of_class(k.class)).or_else(by_face).unwrap_or(Kind::Other);
        let on = faces.iter().find(|f| bound(f));
        out.push(Raw { kind, id, driver: on.and_then(|f| driver_name(f)), bound: on.is_some(), expected: kind != Kind::Other, usb: true });
    }
    out
}

pub fn inventory_in(sys: &Path, log: Option<&str>, pci_ids: &Path, usb_ids: &Path) -> Inventory {
    let found = scan_in(sys, log);
    let (pci, usb) = (pci_devices(sys), usb_devices(sys));
    let pci_names = names_from(&ids_text(pci_ids), &pci.iter().map(|r| r.id).collect::<Vec<_>>());
    let usb_names = names_from(&ids_text(usb_ids), &usb.iter().map(|r| r.id).collect::<Vec<_>>());
    let mut used = vec![false; found.len()];
    let mut devices = Vec::new();
    for r in pci.into_iter().chain(usb) {
        let name = if r.usb { usb_names.get(&r.id) } else { pci_names.get(&r.id) }.cloned();
        let text = id_text(r.id);
        let at = if r.bound { None } else { found.iter().position(|f| f.id == text) };
        let (state, problem) = if let Some(i) = at {
            used[i] = true;
            (if found[i].firmware.is_some() { State::NoFirmware } else { State::NoDriver }, Some(words(&found[i])))
        } else if !r.bound && r.expected {
            (State::NoDriver, Some(t!("common-no-driver-device-none", name = name.as_deref().unwrap_or(&text))))
        } else if r.bound {
            (State::Working, None)
        } else {
            (State::Idle, None)
        };
        devices.push(Device { kind: r.kind, id: text, name, driver: r.driver, state, problem });
    }
    let mut loose = Vec::new();
    for f in found.iter().enumerate().filter(|(i, _)| !used[*i]).map(|(_, f)| f) {

        let owner = devices.iter_mut().find(|d| d.kind == kind_of_class(f.class) && d.state == State::Working);
        match owner.filter(|_| f.id.is_empty()) {
            Some(d) => {
                d.state = State::NoFirmware;
                d.problem = Some(words(f));
            }
            None => loose.push(words(f)),
        }
    }
    Inventory { devices, loose }
}

pub fn inventory() -> Inventory {
    inventory_in(Path::new("/sys"), kernel_log().as_deref(), Path::new(PCI_IDS), Path::new(USB_IDS))
}

#[cfg(test)]
mod tests;
