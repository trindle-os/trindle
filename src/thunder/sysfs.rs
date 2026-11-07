//! The kernel's Thunderbolt and USB4 ports and the devices plugged into them, as /sys/bus/thunderbolt/devices shows them.

use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    None,
    User,
    Secure,
    DpOnly,
    UsbOnly,
    NoPcie,
    Unknown,
}

impl Level {
    pub fn parse(s: &str) -> Level {
        match s.trim() {
            "none" => Level::None,
            "user" => Level::User,
            "secure" => Level::Secure,
            "dponly" => Level::DpOnly,
            "usbonly" => Level::UsbOnly,
            "nopcie" => Level::NoPcie,
            _ => Level::Unknown,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Level::None => "none",
            Level::User => "user",
            Level::Secure => "secure",
            Level::DpOnly => "dponly",
            Level::UsbOnly => "usbonly",
            Level::NoPcie => "nopcie",
            Level::Unknown => "unknown",
        }
    }

    pub fn asks(self) -> bool {
        matches!(self, Level::User | Level::Secure | Level::Unknown)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Domain {
    pub number: String,
    pub level: Level,
    pub iommu: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub kernel: String,
    pub dir: PathBuf,
    pub unique_id: String,
    pub vendor: String,
    pub name: String,
    pub authorized: u8,
    pub domain: String,
    pub reachable: bool,
    pub depth: usize,
    pub has_key: bool,
}

pub fn bus(sys: &Path) -> PathBuf {
    sys.join("bus/thunderbolt/devices")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).map(|s| s.trim().to_string()).unwrap_or_default()
}

pub fn clean(s: &str) -> String {
    let words: Vec<&str> = s.split(|c: char| c.is_control() || c.is_whitespace()).filter(|w| !w.is_empty()).collect();
    words.join(" ").chars().take(60).collect()
}

pub fn sane_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

fn device_name(name: &str) -> Option<(&str, &str)> {
    let (domain, route) = name.split_once('-')?;
    let figures = |s: &str, hex: bool| !s.is_empty() && s.chars().all(|c| if hex { c.is_ascii_hexdigit() } else { c.is_ascii_digit() });
    (figures(domain, false) && figures(route, true) && !route.trim_start_matches('0').is_empty()).then_some((domain, route))
}

pub fn domains(sys: &Path) -> Vec<Domain> {
    let Ok(dir) = fs::read_dir(bus(sys)) else { return Vec::new() };
    let mut found: Vec<Domain> = dir.flatten().filter_map(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        let number = name.strip_prefix("domain")?.to_string();
        let path = e.path();
        Some(Domain {
            number,
            level: Level::parse(&read(&path.join("security"))),
            iommu: read(&path.join("iommu_dma_protection")) == "1",
        })
    }).collect();
    found.sort_by(|a, b| a.number.cmp(&b.number));
    found
}

pub fn devices(sys: &Path) -> Vec<Device> {
    let Ok(dir) = fs::read_dir(bus(sys)) else { return Vec::new() };
    let mut found: Vec<Device> = dir.flatten().filter_map(|e| {
        let kernel = e.file_name().to_string_lossy().to_string();
        let (domain, _) = device_name(&kernel)?;
        let dir = e.path();
        let authorized = read(&dir.join("authorized"));
        if !dir.join("authorized").is_file() {
            return None;
        }
        let real = fs::canonicalize(&dir).unwrap_or_else(|_| dir.clone());
        let parent = real.parent().map(Path::to_path_buf).unwrap_or_default();
        let parent_is_device = parent.file_name().and_then(|n| n.to_str()).and_then(device_name).is_some();
        let reachable = !parent_is_device || read(&parent.join("authorized")).parse::<u8>().is_ok_and(|a| a > 0);
        let depth = real.ancestors().filter(|a| a.file_name().and_then(|n| n.to_str()).and_then(device_name).is_some()).count();
        Some(Device {
            unique_id: read(&dir.join("unique_id")),
            vendor: clean(&read(&dir.join("vendor_name"))),
            name: clean(&read(&dir.join("device_name"))),
            authorized: authorized.parse().unwrap_or(0),
            domain: domain.to_string(),
            has_key: dir.join("key").is_file(),
            reachable,
            depth,
            dir,
            kernel,
        })
    }).collect();
    found.sort_by(|a, b| a.depth.cmp(&b.depth).then(a.kernel.cmp(&b.kernel)));
    found
}

#[derive(Clone, Debug, PartialEq)]
pub enum How {
    Plain,
    NewKey(String),
    Key(String),
}

fn put(path: &Path, value: &str) -> io::Result<()> {
    fs::OpenOptions::new().write(true).truncate(true).open(path)?.write_all(value.as_bytes())
}

pub fn authorize(dev: &Device, how: &How) -> io::Result<()> {
    match how {
        How::Plain => put(&dev.dir.join("authorized"), "1"),
        How::NewKey(key) => {
            put(&dev.dir.join("key"), key)?;
            put(&dev.dir.join("authorized"), "1")
        }
        How::Key(key) => {
            put(&dev.dir.join("key"), key)?;
            put(&dev.dir.join("authorized"), "2")
        }
    }
}

pub fn new_key() -> io::Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| io::Error::other(e.to_string()))?;
    Ok(bytes.iter().fold(String::new(), |mut s, b| { let _ = write!(s, "{b:02x}"); s }))
}

#[cfg(test)]
pub mod fake {
    use std::fs;
    use std::path::{Path, PathBuf};

    pub fn domain(sys: &Path, n: u32, level: &str, iommu: bool) {
        let real = sys.join(format!("devices/pci0000:00/domain{n}"));
        fs::create_dir_all(real.join(format!("{n}-0"))).unwrap();
        fs::write(real.join("security"), format!("{level}\n")).unwrap();
        fs::write(real.join("iommu_dma_protection"), if iommu { "1\n" } else { "0\n" }).unwrap();
        fs::write(real.join(format!("{n}-0/authorized")), "1\n").unwrap();
        let bus = super::bus(sys);
        fs::create_dir_all(&bus).unwrap();
        let _ = std::os::unix::fs::symlink(&real, bus.join(format!("domain{n}")));
        let _ = std::os::unix::fs::symlink(real.join(format!("{n}-0")), bus.join(format!("{n}-0")));
    }

    pub fn plug(sys: &Path, under: &str, kernel: &str, id: &str, vendor: &str, name: &str, secure: bool) -> PathBuf {
        let bus = super::bus(sys);
        let parent = fs::canonicalize(bus.join(under)).unwrap();
        let real = parent.join(kernel);
        fs::create_dir_all(&real).unwrap();
        fs::write(real.join("authorized"), "0\n").unwrap();
        fs::write(real.join("unique_id"), format!("{id}\n")).unwrap();
        fs::write(real.join("vendor_name"), format!("{vendor}\n")).unwrap();
        fs::write(real.join("device_name"), format!("{name}\n")).unwrap();
        if secure {
            fs::write(real.join("key"), "").unwrap();
        }
        std::os::unix::fs::symlink(&real, bus.join(kernel)).unwrap();
        real
    }

    pub fn unplug(sys: &Path, kernel: &str) {
        let bus = super::bus(sys);
        let real = fs::canonicalize(bus.join(kernel)).unwrap();
        for e in fs::read_dir(&bus).unwrap().flatten() {
            if fs::read_link(e.path()).is_ok_and(|to| to.starts_with(&real)) {
                fs::remove_file(e.path()).unwrap();
            }
        }
        fs::remove_dir_all(real).unwrap();
    }

    pub fn authorized(sys: &Path, kernel: &str) -> String {
        fs::read_to_string(super::bus(sys).join(kernel).join("authorized")).unwrap().trim().to_string()
    }

    pub fn key(sys: &Path, kernel: &str) -> String {
        fs::read_to_string(super::bus(sys).join(kernel).join("key")).unwrap().trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_read_and_only_two_ask() {
        for (word, asks) in [("none", false), ("user", true), ("secure", true), ("dponly", false), ("usbonly", false),
                             ("nopcie", false)] {
            let l = Level::parse(word);
            assert_eq!(l.word(), word);
            assert_eq!(l.asks(), asks, "{word}");
        }
        assert!(Level::parse("something new").asks());
    }

    #[test]
    fn devices_are_found_nearest_first() {
        let t = crate::common::files::private_temp("thunder-sysfs").unwrap();
        fake::domain(&t, 0, "user", true);
        fake::plug(&t, "0-0", "0-1", "aaaa-1", "Dock Makers", "Big Dock", false);
        fake::plug(&t, "0-1", "0-301", "bbbb-2", "Disk Co", "Fast\tDisk\n\u{7}", false);
        let d = domains(&t);
        assert_eq!(d, vec![Domain { number: "0".into(), level: Level::User, iommu: true }]);
        let all = devices(&t);
        assert_eq!(all.iter().map(|d| d.kernel.as_str()).collect::<Vec<_>>(), ["0-1", "0-301"]);
        assert!(all[0].reachable && !all[1].reachable, "behind a dock that waits");
        assert_eq!(all[1].name, "Fast Disk");
        assert_eq!((all[0].depth, all[1].depth), (1, 2));
        authorize(&all[0], &How::Plain).unwrap();
        assert!(devices(&t)[1].reachable);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn names_from_a_device_are_cleaned() {
        assert_eq!(clean("  A\u{0}B \t C\r\n"), "A B C");
        assert_eq!(clean(&"x".repeat(200)).len(), 60);
        assert!(sane_id("82593804-e0ed-2235-ffff-ffffffffffff"));
        assert!(!sane_id("a\tb") && !sane_id(""));
        assert_eq!(new_key().unwrap().len(), 64);
    }
}
