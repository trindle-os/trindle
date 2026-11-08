//! The step taken just before a restart (not a power-off): external USB hubs
//! are let go of cleanly, so they are not left half-way through a transfer
//! or in a power-saving state when the computer's USB controller is reset.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MOST: usize = 24;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    pub file: PathBuf,
    pub value: &'static str,
}

fn device_name(n: &str) -> bool {
    n.split_once('-').is_some_and(|(b, r)| {
        !b.is_empty()
            && b.len() <= 4
            && b.bytes().all(|c| c.is_ascii_digit())
            && r.split('.').count() <= 7
            && r.split('.').all(|p| !p.is_empty() && p.len() <= 3 && p.bytes().all(|c| c.is_ascii_digit()))
    })
}

fn parent_port(name: &str) -> Option<(String, String)> {
    let (bus, path) = name.split_once('-')?;
    match path.rsplit_once('.') {
        None => Some((format!("{bus}-0:1.0"), format!("usb{bus}-port{path}"))),
        Some((up, n)) => {
            let parent = format!("{bus}-{up}");
            Some((format!("{parent}:1.0"), format!("{parent}-port{n}")))
        }
    }
}

pub fn plan(sys: &Path) -> Vec<Step> {
    let devices = sys.join("bus/usb/devices");
    let Ok(rd) = fs::read_dir(&devices) else { return Vec::new() };
    let mut hubs: Vec<(usize, String, PathBuf)> = Vec::new();
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !device_name(&name) {
            continue;
        }
        let class = fs::read_to_string(e.path().join("bDeviceClass")).unwrap_or_default();
        if class.trim() != "09" {
            continue;
        }
        let Some((iface, port)) = parent_port(&name) else { continue };
        let port_dir = devices.join(iface).join(port);
        let kind = fs::read_to_string(port_dir.join("connect_type")).unwrap_or_default();
        if kind.trim() == "hardwired" {
            continue;
        }
        hubs.push((name.matches('.').count(), name, port_dir));
    }

    hubs.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    hubs.truncate(MOST);
    let mut steps: Vec<Step> = hubs
        .iter()
        .map(|(_, name, _)| Step { file: devices.join(name).join("authorized"), value: "0" })
        .collect();
    for (depth, _, port_dir) in &hubs {
        let switch = port_dir.join("disable");
        if *depth == 0 && switch.is_file() {
            steps.push(Step { file: switch, value: "1" });
        }
    }
    steps
}

pub fn run(steps: &[Step], write: &mut dyn FnMut(&Path, &str) -> io::Result<()>) -> usize {
    steps.iter().filter(|s| write(&s.file, s.value).is_ok()).count()
}

pub fn main() -> i32 {
    let steps = plan(Path::new("/sys"));
    let done = run(&steps, &mut |p, v| fs::write(p, v));
    eprintln!("trindle-usb-retry: {done} of {} steps made before the restart", steps.len());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake(PathBuf);

    impl Fake {
        fn new() -> Fake {
            let root = crate::common::files::private_temp("usbquiet").unwrap();
            fs::create_dir_all(root.join("bus/usb/devices")).unwrap();
            Fake(root)
        }

        fn dev(&self, name: &str, class: &str) {
            let d = self.0.join("bus/usb/devices").join(name);
            fs::create_dir_all(&d).unwrap();
            fs::write(d.join("bDeviceClass"), format!("{class}\n")).unwrap();
            fs::write(d.join("authorized"), "1\n").unwrap();
        }

        fn port(&self, iface: &str, port: &str, kind: &str, switch: bool) {
            let d = self.0.join("bus/usb/devices").join(iface).join(port);
            fs::create_dir_all(&d).unwrap();
            fs::write(d.join("connect_type"), format!("{kind}\n")).unwrap();
            if switch {
                fs::write(d.join("disable"), "0\n").unwrap();
            }
        }
    }

    impl Drop for Fake {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn files(steps: &[Step], root: &Path) -> Vec<String> {
        steps
            .iter()
            .map(|s| format!("{} {}", s.file.strip_prefix(root.join("bus/usb/devices")).unwrap().display(), s.value))
            .collect()
    }

    #[test]
    fn lets_go_of_external_hubs_deepest_first_then_the_computers_port() {
        let f = Fake::new();

        f.dev("3-1", "09");
        f.dev("3-1.4", "09");
        f.dev("3-1.4.2", "00");
        f.dev("usb3", "09");
        f.port("3-0:1.0", "usb3-port1", "hotplug", true);
        f.port("3-1:1.0", "3-1-port4", "unknown", true);
        let got = files(&plan(&f.0), &f.0);
        assert_eq!(got, ["3-1.4/authorized 0", "3-1/authorized 0", "3-0:1.0/usb3-port1/disable 1"]);
    }

    #[test]
    fn built_in_hubs_and_other_devices_are_left_alone() {
        let f = Fake::new();
        f.dev("1-1", "09");
        f.port("1-0:1.0", "usb1-port1", "hardwired", true);
        f.dev("1-2", "03");
        f.port("1-0:1.0", "usb1-port2", "hotplug", true);
        f.dev("usb1", "09");
        assert!(plan(&f.0).is_empty());
    }

    #[test]
    fn a_port_with_no_switch_still_lets_the_hub_go() {
        let f = Fake::new();
        f.dev("2-3", "09");
        f.port("2-0:1.0", "usb2-port3", "unknown", false);
        assert_eq!(files(&plan(&f.0), &f.0), ["2-3/authorized 0"]);
    }

    #[test]
    fn odd_names_and_a_missing_bus_do_nothing() {
        let f = Fake::new();
        f.dev("3-1;x", "09");
        f.dev("../3-1", "09");
        f.dev("3-1:1.0", "09");
        assert!(plan(&f.0).is_empty());
        assert!(plan(&f.0.join("nowhere")).is_empty());
    }

    #[test]
    fn never_more_than_the_most_and_failures_are_counted_not_fatal() {
        let f = Fake::new();
        for n in 1..=(MOST + 6) {
            f.dev(&format!("3-{n}"), "09");
        }
        let steps = plan(&f.0);
        assert_eq!(steps.len(), MOST);
        let mut calls = 0;
        let worked = run(&steps, &mut |_, _| {
            calls += 1;
            if calls % 2 == 0 { Err(io::Error::other("no")) } else { Ok(()) }
        });
        assert_eq!(calls, MOST);
        assert_eq!(worked, MOST / 2);
    }

    #[test]
    fn the_step_runs_before_a_restart_only_and_is_quick() {
        let unit = include_str!("../../system/systemd/trindle-usb-quiet.service");
        let line = |key: &str| unit.lines().find_map(|l| l.strip_prefix(key)).unwrap_or_default().trim().to_string();
        assert_eq!(line("WantedBy="), "reboot.target kexec.target");
        assert_eq!(line("TimeoutStartSec="), "5");
        assert!(line("ExecStart=").ends_with("trindle-usb-retry --before-restart"));
        assert!(unit.contains("\nCapabilityBoundingSet=\n"), "no powers at all");
        assert_eq!(line("ProtectSystem="), "strict");
        assert_eq!(line("ReadWritePaths="), "/sys/devices", "the hubs' files are the one place it may write");
        assert!(!unit.contains("ProtectKernelTunables=yes"), "that would make /sys read-only");
        let owner_may_write = |mode: u32| mode & 0o200 != 0;
        assert!(owner_may_write(0o644) && !unit.contains("CAP_DAC"), "root owns the files, so no capability is needed to write them");
        let install = include_str!("../../tools/system-files.sh");
        assert!(install.contains("for t in reboot kexec; do"), "linked into the restart targets");
        assert!(!install.contains("poweroff.target.wants/trindle-usb-quiet") && !install.contains("halt.target.wants/trindle-usb-quiet"));
    }
}
