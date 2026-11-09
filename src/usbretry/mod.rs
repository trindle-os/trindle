//! trindle-usb-retry: tries a USB port that failed to start once more.

use crate::toolkit::t;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub mod quiet;

pub const LOG: &str = "/run/trindle-usb-retry/log";
const KMSG: &str = "/dev/kmsg";

pub const MOST: usize = 6;

const FAILED: &str = "unable to enumerate USB device";

const SETTLE: Duration = Duration::from_secs(3);

const OFF: Duration = Duration::from_secs(1);

const WAIT: u32 = 8;

const CONTROLLER_WAIT: u32 = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {

    Worked,

    WorkedAfterController,

    Nothing,

    NothingAfterController,
}

impl Outcome {
    fn word(self) -> &'static str {
        match self {
            Outcome::Worked => "worked",
            Outcome::WorkedAfterController => "controller",
            Outcome::Nothing => "nothing",
            Outcome::NothingAfterController => "controller-nothing",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub port: String,
    pub worked: bool,

    pub controller: bool,
}

impl Entry {

    pub fn words(&self) -> String {
        match (self.worked, self.controller) {
            (true, false) => t!("usbretry-worked", port = &self.port),
            (true, true) => t!("usbretry-worked-controller", port = &self.port),
            (false, true) => t!("usbretry-nothing-controller", port = &self.port),
            (false, false) => t!("usbretry-nothing", port = &self.port),
        }
    }
}

pub fn read_log(path: &Path) -> Vec<Entry> {
    let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
    text.lines()
        .filter_map(|l| {
            let mut it = l.split('\t');
            let port = it.next()?;
            let result = it.next()?;
            valid_port(port).then(|| Entry {
                port: port.to_string(),
                worked: result == "worked" || result == "controller",
                controller: result == "controller" || result == "controller-nothing",
            })
        })
        .take(MOST)
        .collect()
}

pub fn valid_port(s: &str) -> bool {
    let Some((hub, n)) = s.rsplit_once("-port") else { return false };
    let digits = |p: &str, most: usize| !p.is_empty() && p.len() <= most && p.bytes().all(|b| b.is_ascii_digit());
    if !digits(n, 3) {
        return false;
    }
    if let Some(bus) = hub.strip_prefix("usb") {
        return digits(bus, 4);
    }
    let Some((bus, path)) = hub.split_once('-') else { return false };
    digits(bus, 4) && path.split('.').count() <= 7 && path.split('.').all(|p| digits(p, 3))
}

pub fn failed_port(record: &str) -> Option<String> {
    let message = record.split_once(';').map_or(record, |(_, m)| m);
    let message = message.lines().next()?;
    let (head, tail) = message.split_once(FAILED)?;
    if !tail.trim().is_empty() {
        return None;
    }
    let token = head.trim_end().trim_end_matches(':').rsplit(' ').next()?;
    valid_port(token).then(|| token.to_string())
}

#[derive(Clone, Debug)]
pub struct Places {
    pub sys: PathBuf,
    pub log: PathBuf,
}

impl Places {
    pub fn system() -> Places {
        Places { sys: "/sys".into(), log: LOG.into() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Skip {
    Done,
    TooMany,
    NotFound,
    Busy,
    Overcurrent,
    NoSwitch,
}

type Write1 = Box<dyn FnMut(&Path, &str) -> io::Result<()>>;
type Nap = Box<dyn FnMut(Duration)>;

pub struct Retrier {
    places: Places,
    tried: HashSet<String>,

    write: Write1,
    sleep: Nap,

    restarted: HashSet<String>,
}

impl Retrier {
    pub fn new(places: Places) -> Retrier {
        Retrier::with_io(places, Box::new(|p, v| fs::write(p, v)), Box::new(std::thread::sleep))
    }

    pub fn with_io(places: Places, write: Write1, sleep: Nap) -> Retrier {

        let tried = read_log(&places.log).into_iter().map(|e| e.port).collect();
        Retrier { places, tried, write, sleep, restarted: HashSet::new() }
    }

    fn port_dir(&self, port: &str) -> Option<PathBuf> {
        let devices = self.places.sys.join("bus/usb/devices");
        for e in fs::read_dir(devices).ok()?.flatten() {
            if !e.file_name().to_string_lossy().contains(':') {
                continue;
            }
            let dir = e.path().join(port);
            if dir.is_dir() {
                return Some(dir);
            }
        }
        None
    }

    fn occupied(&self, port: &str) -> bool {
        let Some((hub, n)) = port.rsplit_once("-port") else { return false };
        let name = match hub.strip_prefix("usb") {
            Some(bus) => format!("{bus}-{n}"),
            None => format!("{hub}.{n}"),
        };
        self.places.sys.join("bus/usb/devices").join(name).exists()
    }

    pub fn consider(&mut self, port: &str) -> Result<Outcome, Skip> {
        if !valid_port(port) || self.tried.contains(port) {
            return Err(Skip::Done);
        }
        if self.tried.len() >= MOST {
            return Err(Skip::TooMany);
        }
        let dir = self.port_dir(port).ok_or(Skip::NotFound)?;

        (self.sleep)(SETTLE);
        if self.occupied(port) {
            return Err(Skip::Busy);
        }
        let over = fs::read_to_string(dir.join("over_current_count"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        if over > 0 {
            return Err(Skip::Overcurrent);
        }
        let switch = dir.join("disable");
        if !switch.is_file() {
            return Err(Skip::NoSwitch);
        }

        self.tried.insert(port.to_string());
        let off = (self.write)(&switch, "1");
        (self.sleep)(OFF);

        let on = (self.write)(&switch, "0");
        let mut outcome = Outcome::Nothing;
        if off.is_ok() && on.is_ok() {
            for _ in 0..WAIT {
                (self.sleep)(Duration::from_secs(1));
                if self.occupied(port) {
                    outcome = Outcome::Worked;
                    break;
                }
            }
        }
        if outcome == Outcome::Nothing && port.starts_with("usb") {
            match self.restart_controller(&dir, port) {
                Some(true) => outcome = Outcome::WorkedAfterController,
                Some(false) => outcome = Outcome::NothingAfterController,
                None => {}
            }
        }
        self.note(port, outcome);
        Ok(outcome)
    }

    fn restart_controller(&mut self, port_dir: &Path, port: &str) -> Option<bool> {
        let pci = controller_of(port_dir)?;
        let addr = pci.file_name().map(|n| n.to_string_lossy().into_owned())?;
        if !valid_pci(&addr) || !self.restarted.insert(addr.clone()) || !controller_safe(&pci) {
            return None;
        }
        let driver = self.places.sys.join("bus/pci/drivers/xhci_hcd");
        let unbind = (self.write)(&driver.join("unbind"), &addr);
        (self.sleep)(OFF);

        let bind = (self.write)(&driver.join("bind"), &addr);
        if unbind.is_err() && bind.is_err() {
            return None;
        }
        for _ in 0..CONTROLLER_WAIT {
            (self.sleep)(Duration::from_secs(1));
            if self.occupied(port) {
                return Some(true);
            }
        }
        Some(false)
    }

    fn note(&self, port: &str, outcome: Outcome) {
        if let Some(dir) = self.places.log.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&self.places.log) {
            let _ = writeln!(f, "{port}\t{}", outcome.word());
        }
    }
}

fn controller_of(port_dir: &Path) -> Option<PathBuf> {
    let real = fs::canonicalize(port_dir).ok()?;
    let bus = real.ancestors().find(|a| {
        a.file_name().is_some_and(|n| {
            let n = n.to_string_lossy();
            n.strip_prefix("usb").is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
        })
    })?;
    let pci = bus.parent()?;
    let driver = fs::read_link(pci.join("driver")).ok()?;
    (driver.file_name()? == "xhci_hcd").then(|| pci.to_path_buf())
}

fn valid_pci(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 12
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b':',
            10 => *c == b'.',
            11 => (b'0'..=b'7').contains(c),
            _ => c.is_ascii_digit() || (b'a'..=b'f').contains(c),
        })
}

fn controller_safe(pci: &Path) -> bool {
    fn holds_disk(dir: &Path, depth: u32) -> bool {
        if depth == 0 {
            return true;
        }
        if fs::read_to_string(dir.join("bInterfaceClass")).is_ok_and(|c| c.trim() == "08") {
            return true;
        }
        if fs::read_link(dir.join("driver"))
            .ok()
            .and_then(|d| d.file_name().map(|n| n.to_string_lossy().into_owned()))
            .is_some_and(|n| n == "usb-storage" || n == "uas")
        {
            return true;
        }
        let Ok(rd) = fs::read_dir(dir) else { return true };
        rd.flatten().any(|e| {
            let name = e.file_name().to_string_lossy().into_owned();

            let is_dir = e.file_type().is_ok_and(|t| t.is_dir());
            is_dir && (name == "block" || ((name.starts_with("usb") || name.contains(':') || name.contains('-') || name.starts_with("host") || name.starts_with("target")) && holds_disk(&e.path(), depth - 1)))
        })
    }
    !holds_disk(pci, 14)
}

fn watch(retrier: &mut Retrier, kmsg: &Path) -> io::Result<()> {
    let mut file = fs::File::open(kmsg)?;
    let mut buf = vec![0u8; 8192];
    loop {
        match file.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                let record = String::from_utf8_lossy(&buf[..n]);
                if let Some(port) = failed_port(&record) {
                    match retrier.consider(&port) {
                        Ok(o) => eprintln!("trindle-usb-retry: {port} switched off and on: {}", o.word()),
                        Err(s) => eprintln!("trindle-usb-retry: {port} left alone: {s:?}"),
                    }
                }
            }

            Err(e) if e.raw_os_error() == Some(32) || e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
}

pub fn main(args: Vec<String>) -> i32 {
    if args.iter().skip(1).any(|a| a == "--before-restart") {
        return quiet::main();
    }
    if args.iter().skip(1).any(|a| a == "--list") {
        read_log(Path::new(LOG)).iter().for_each(|e| println!("{}", e.words()));
        return 0;
    }
    let mut retrier = Retrier::new(Places::system());
    match watch(&mut retrier, Path::new(KMSG)) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-usb-retry: cannot read the kernel log: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod language_tests {
    use super::Entry;
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

    #[test]
    fn the_usb_words_follow_the_language_and_fall_back_word_by_word() {
        let french = "usbretry-nothing = Le port { $port } ne revient pas.\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            let gone = Entry { port: "1-1".into(), worked: false, controller: false };
            assert_eq!(gone.words(), "Le port 1-1 ne revient pas.");
            let back = Entry { port: "1-1".into(), worked: true, controller: true };
            assert!(back.words().starts_with("USB port 1-1 did not start"), "{}", back.words());
        });
    }
}
