//! Hardware that has no driver, or whose driver is missing a firmware file,
//! said in plain words. Wi-Fi, Bluetooth, sound and graphics
//! chips are looked for in the same way: the PCI and USB devices the kernel
//! lists in /sys are read, and one with no driver bound is found. The
//! kernel log is read the safe way the Report page reads it (no password,
//! nothing changed); a firmware file it says could not be loaded is found
//! too. When the log cannot be read, nothing is said about firmware.

use std::path::Path;
use std::sync::Mutex;

use crate::toolkit::t;

pub const HELP: &str = "hardware-no-driver";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    Wifi,
    Bluetooth,
    Sound,
    Graphics,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Gap {

    Driver { kernel: &'static str },

    Firmware { package: &'static str, version: &'static str },
}

#[derive(PartialEq, Debug)]
pub struct Known {
    pub chip: &'static str,
    pub class: Class,

    pub pci: Option<(u16, u16)>,

    pub usb: Option<(u16, u16)>,

    pub firmware: Option<&'static str>,
    pub gap: Gap,
}

pub const KNOWN: &[Known] = &[
    Known { chip: "MediaTek MT7902 Wi-Fi", class: Class::Wifi, pci: Some((0x14c3, 0x7902)), usb: None, firmware: None,
            gap: Gap::Driver { kernel: "7.1" } },
    Known { chip: "MediaTek MT7902 Bluetooth", class: Class::Bluetooth, pci: None, usb: None,
            firmware: Some("mediatek/BT_RAM_CODE_MT7902_1_1_hdr.bin"),
            gap: Gap::Firmware { package: "firmware-mediatek", version: "20260916" } },
];

const FIRMWARE_CLASS: &[(&str, Class)] = &[
    ("iwlwifi-", Class::Wifi), ("rtlwifi/", Class::Wifi), ("rtw88/", Class::Wifi), ("rtw89/", Class::Wifi),
    ("ath9k_htc/", Class::Wifi), ("ath10k/", Class::Wifi), ("ath11k/", Class::Wifi), ("ath12k/", Class::Wifi),
    ("brcm/brcmfmac", Class::Wifi), ("mediatek/mt79", Class::Wifi), ("mediatek/mt76", Class::Wifi),
    ("rtl_bt/", Class::Bluetooth), ("intel/ibt-", Class::Bluetooth), ("qca/", Class::Bluetooth),
    ("mediatek/BT_", Class::Bluetooth), ("brcm/BCM", Class::Bluetooth),
    ("amdgpu/", Class::Graphics), ("radeon/", Class::Graphics), ("i915/", Class::Graphics), ("nvidia/", Class::Graphics),
    ("intel/sof", Class::Sound), ("cirrus/", Class::Sound),
];

#[derive(Clone, PartialEq, Debug)]
pub struct Found {
    pub class: Class,

    pub known: Option<&'static Known>,

    pub id: String,

    pub firmware: Option<String>,
}

fn hex(path: &Path) -> Option<u32> {
    let s = std::fs::read_to_string(path).ok()?;
    u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok()
}

fn id_pair(dir: &Path, v: &str, d: &str) -> Option<(u16, u16)> {
    Some((hex(&dir.join(v))? as u16, hex(&dir.join(d))? as u16))
}

fn class_of_pci(code: u32) -> Option<Class> {
    match code >> 8 {
        0x0280 | 0x0d20 | 0x0d21 => Some(Class::Wifi),
        0x0d11 => Some(Class::Bluetooth),
        0x0401 | 0x0403 => Some(Class::Sound),
        c if c >> 8 == 0x03 => Some(Class::Graphics),
        _ => None,
    }
}

fn known_pci(id: (u16, u16)) -> Option<&'static Known> {
    KNOWN.iter().find(|k| k.pci == Some(id))
}

fn known_usb(id: (u16, u16)) -> Option<&'static Known> {
    KNOWN.iter().find(|k| k.usb == Some(id))
}

fn id_text(id: (u16, u16)) -> String {
    format!("{:04x}:{:04x}", id.0, id.1)
}

fn bound(dir: &Path) -> bool {
    dir.join("driver").exists()
}

fn names_in(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir).into_iter().flatten().flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    v.sort();
    v
}

fn unbound(sys: &Path) -> Vec<Found> {
    let mut out = Vec::new();
    let pci = sys.join("bus/pci/devices");
    for n in names_in(&pci) {
        let d = pci.join(&n);
        let (Some(code), Some(id)) = (hex(&d.join("class")), id_pair(&d, "vendor", "device")) else { continue };
        let known = known_pci(id);
        let Some(class) = known.map(|k| k.class).or_else(|| class_of_pci(code)) else { continue };
        if !bound(&d) {
            out.push(Found { class, known, id: id_text(id), firmware: None });
        }
    }
    let usb = sys.join("bus/usb/devices");
    for n in names_in(&usb) {
        let d = usb.join(&n);

        if n.contains(':') || n.starts_with("usb") {
            continue;
        }
        let Some(id) = id_pair(&d, "idVendor", "idProduct") else { continue };
        let faces: Vec<std::path::PathBuf> = names_in(&usb).into_iter()
            .filter(|f| f.starts_with(&format!("{n}:"))).map(|f| usb.join(f)).collect();
        if let Some(k) = known_usb(id) {
            if !faces.iter().any(|f| bound(f)) {
                out.push(Found { class: k.class, known: Some(k), id: id_text(id), firmware: None });
            }
            continue;
        }
        for f in faces.iter().filter(|f| !bound(f)) {
            let class = match (hex(&f.join("bInterfaceClass")), hex(&f.join("bInterfaceSubClass")), hex(&f.join("bInterfaceProtocol"))) {
                (Some(0xe0), Some(1), Some(1)) => Class::Bluetooth,
                (Some(1), _, _) => Class::Sound,
                _ => continue,
            };
            let found = Found { class, known: None, id: id_text(id), firmware: None };
            if !out.contains(&found) {
                out.push(found);
            }
        }
    }
    out
}

pub fn missing_firmware(log: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for l in log.lines() {
        let Some(at) = l.find("Direct firmware load for ") else { continue };
        let rest = &l[at + "Direct firmware load for ".len()..];
        let Some(end) = rest.find(" failed") else { continue };
        let file = rest[..end].trim().to_string();
        if !file.is_empty() && !out.contains(&file) {
            out.push(file);
        }
    }
    out
}

pub fn scan_in(sys: &Path, log: Option<&str>) -> Vec<Found> {
    let mut out = unbound(sys);
    let Some(log) = log else { return out };
    for file in missing_firmware(log) {
        if let Some(k) = KNOWN.iter().find(|k| k.firmware == Some(file.as_str())) {
            out.push(Found { class: k.class, known: Some(k), id: String::new(), firmware: Some(file) });
            continue;
        }
        let Some(&(_, class)) = FIRMWARE_CLASS.iter().find(|(p, _)| file.starts_with(p)) else { continue };

        let at = out.iter().position(|f| f.class == class && f.firmware.is_none());
        if let Some(at) = at {
            if out[at].known.is_none() {
                out[at].firmware = Some(file);
            }
        }
    }
    out
}

fn kernel_log() -> Option<String> {
    for argv in [&["journalctl", "-k", "-b", "--no-pager", "-q", "-o", "cat"][..], &["dmesg"][..]] {
        let t = crate::common::run::text(argv);
        if !t.is_empty() {
            return Some(t);
        }
    }
    None
}

pub fn scan() -> Vec<Found> {
    scan_in(Path::new("/sys"), kernel_log().as_deref())
}

pub fn scan_sys() -> Vec<Found> {
    scan_in(Path::new("/sys"), None)
}

fn class_words(c: Class) -> &'static str {
    match c {
        Class::Wifi => t!("common-no-driver-class-wifi"),
        Class::Bluetooth => t!("common-no-driver-class-bluetooth"),
        Class::Sound => t!("common-no-driver-class-sound"),
        Class::Graphics => t!("common-no-driver-class-graphics"),
    }
}

pub fn words(f: &Found) -> String {
    if let Some(k) = f.known {
        let line = match k.gap {
            Gap::Driver { kernel } => t!("common-no-driver-known-driver", chip = k.chip, kernel = kernel),
            Gap::Firmware { package, version } => t!("common-no-driver-known-firmware", chip = k.chip, package = package, version = version),
        };
        return if k.class == Class::Wifi { format!("{line} {}", t!("common-no-driver-stick")) } else { line };
    }
    let class = class_words(f.class);
    let line = match &f.firmware {
        Some(file) => t!("common-no-driver-generic-firmware", class = class, file = file.as_str()),
        None => t!("common-no-driver-generic", class = class, id = f.id.as_str()),
    };
    if f.class == Class::Wifi { format!("{line} {}", t!("common-no-driver-stick")) } else { line }
}

pub fn lines_for(found: &[Found], class: Class) -> Vec<String> {
    let mut v: Vec<String> = found.iter().filter(|f| f.class == class).map(words).collect();
    v.dedup();
    v
}

pub fn report_lines(found: &[Found]) -> Vec<String> {
    found.iter().map(words).collect()
}

static READY: Mutex<Option<Vec<Found>>> = Mutex::new(None);

pub fn start() {
    static STARTED: std::sync::Once = std::sync::Once::new();
    STARTED.call_once(|| {
        std::thread::spawn(|| {
            let found = scan();
            *READY.lock().unwrap_or_else(|e| e.into_inner()) = Some(found);
        });
    });
}

pub fn ready_lines(class: Class) -> Vec<String> {
    READY.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|f| lines_for(f, class)).unwrap_or_default()
}

#[cfg(test)]
mod tests;
