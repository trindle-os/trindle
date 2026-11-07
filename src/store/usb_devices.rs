//! `usb-devices`.

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Roots {
    pub sys: PathBuf,
    pub dev: PathBuf,
    pub udev: PathBuf,
}

impl Default for Roots {
    fn default() -> Roots {
        Roots { sys: "/sys".into(), dev: "/dev".into(), udev: "/run/udev/data".into() }
    }
}

const CLASS_IMAGING: u8 = 0x06;
const CLASS_PRINTER: u8 = 0x07;
const CLASS_PER_INTERFACE: u8 = 0x00;
const CLASS_MISC: u8 = 0xef;
const CLASS_VENDOR: u8 = 0xff;

const DEVICE_CLASSES: [u8; 4] = [CLASS_PER_INTERFACE, CLASS_IMAGING, CLASS_MISC, CLASS_VENDOR];
const INTERFACE_CLASSES: [u8; 3] = [CLASS_IMAGING, CLASS_PRINTER, CLASS_VENDOR];

pub const MOST: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub name: String,
    pub bus: u32,
    pub number: u32,
    pub folder: PathBuf,
    pub dev: Option<String>,
}

fn hex_byte(path: &Path) -> Option<u8> {
    u8::from_str_radix(std::fs::read_to_string(path).ok()?.trim(), 16).ok()
}

fn number(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

fn sane_knows(roots: &Roots, dev: Option<&str>) -> bool {
    let Some(dev) = dev.filter(|d| d.chars().all(|c| c.is_ascii_digit() || c == ':')) else { return false };
    std::fs::read_to_string(roots.udev.join(format!("c{dev}"))).is_ok_and(|t| t.lines().any(|l| l.trim() == "E:libsane_matched=yes"))
}

fn classes(folder: &Path) -> Option<(u8, Vec<u8>)> {
    let device = hex_byte(&folder.join("bDeviceClass"))?;
    let mut interfaces = Vec::new();
    let prefix = format!("{}:", folder.file_name()?.to_string_lossy());
    for e in std::fs::read_dir(folder).ok()?.flatten() {
        if e.file_name().to_string_lossy().starts_with(&prefix) {
            interfaces.push(hex_byte(&e.path().join("bInterfaceClass"))?);
        }
    }
    Some((device, interfaces))
}

pub fn allowed_kind(device: u8, interfaces: &[u8], sane: bool) -> bool {
    if !DEVICE_CLASSES.contains(&device) || interfaces.is_empty() || !interfaces.iter().all(|c| INTERFACE_CLASSES.contains(c)) {
        return false;
    }
    device == CLASS_IMAGING || interfaces.contains(&CLASS_IMAGING) || sane
}

pub fn scanners(roots: &Roots) -> Vec<Device> {
    let listing = roots.sys.join("bus/usb/devices");
    let mut names: Vec<String> = std::fs::read_dir(&listing).into_iter().flatten().flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| !n.contains(':') && !n.starts_with("usb") && n.bytes().all(|b| b.is_ascii_digit() || b == b'-' || b == b'.'))
        .collect();
    names.sort();
    let mut found = Vec::new();
    for name in names {
        let Ok(folder) = std::fs::canonicalize(listing.join(&name)) else { continue };
        let Some((device, interfaces)) = classes(&folder) else { continue };
        let (Some(bus), Some(number)) = (number(&folder.join("busnum")), number(&folder.join("devnum"))) else { continue };
        let dev = std::fs::read_to_string(folder.join("dev")).ok().map(|d| d.trim().to_string());
        if !allowed_kind(device, &interfaces, sane_knows(roots, dev.as_deref())) {
            continue;
        }
        if bus == 0 || bus > 999 || number == 0 || number > 999 {
            continue;
        }
        found.push(Device { name, bus, number, folder, dev });
        if found.len() == MOST {
            break;
        }
    }
    found
}

pub fn node(d: &Device) -> String {
    format!("/dev/bus/usb/{:03}/{:03}", d.bus, d.number)
}

pub fn args_for(roots: &Roots, devices: &[Device]) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();
    let mut add = |list: &[&str]| a.extend(list.iter().map(|s| s.to_string()));
    for d in devices {
        let shown = node(d);
        let source = roots.dev.join(shown.trim_start_matches("/dev/"));
        add(&["--dev-bind", &source.to_string_lossy(), &shown]);
        let inside = Path::new("/sys").join(d.folder.strip_prefix(&roots.sys).unwrap_or(&d.folder));
        let (real, inside) = (d.folder.to_string_lossy().into_owned(), inside.to_string_lossy().into_owned());
        add(&["--ro-bind", &real, &inside, "--symlink", &inside, &format!("/sys/bus/usb/devices/{}", d.name)]);
        if let Some(dev) = &d.dev {
            add(&["--symlink", &inside, &format!("/sys/dev/char/{dev}")]);
            for record in [format!("c{dev}"), format!("+usb:{}", d.name)] {
                let source = roots.udev.join(&record);
                if source.is_file() {
                    add(&["--ro-bind", &source.to_string_lossy(), &format!("/run/udev/data/{record}")]);
                }
            }
        }
    }
    a
}

pub fn args() -> Vec<String> {
    let roots = Roots::default();
    args_for(&roots, &scanners(&roots))
}

#[cfg(test)]
mod tests;
