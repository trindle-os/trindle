//! Tests of what a box allowed `usb-devices` is given, against made-up trees of /sys, /dev and udev's database in a folder of their own.

use super::*;
use crate::store::boxed::walls::{walls, Gates};

struct Tree {
    root: PathBuf,
}

impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

impl Tree {
    fn new(tag: &str) -> Tree {
        let root = PathBuf::from(format!("/var/tmp/trindle-usb-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sys/bus/usb/devices")).unwrap();
        std::fs::create_dir_all(root.join("sys/devices/pci0000:00/0000:00:14.0/usb1")).unwrap();
        std::fs::create_dir_all(root.join("dev/bus/usb/001")).unwrap();
        std::fs::create_dir_all(root.join("udev")).unwrap();
        Tree { root }
    }

    fn roots(&self) -> Roots {
        Roots { sys: self.root.join("sys"), dev: self.root.join("dev"), udev: self.root.join("udev") }
    }

    fn add(&self, name: &str, number: u32, class: u8, interfaces: &[u8], sane: bool) {
        let folder = self.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1").join(name);
        std::fs::create_dir_all(&folder).unwrap();
        let minor = number - 1;
        for (file, text) in [("bDeviceClass", format!("{class:02x}\n")), ("busnum", "1\n".into()), ("devnum", format!("{number}\n")), ("dev", format!("189:{minor}\n"))] {
            std::fs::write(folder.join(file), text).unwrap();
        }
        for (i, c) in interfaces.iter().enumerate() {
            let i = folder.join(format!("{name}:1.{i}"));
            std::fs::create_dir_all(&i).unwrap();
            std::fs::write(i.join("bInterfaceClass"), format!("{c:02x}\n")).unwrap();
        }
        std::os::unix::fs::symlink(format!("../../../devices/pci0000:00/0000:00:14.0/usb1/{name}"), self.root.join("sys/bus/usb/devices").join(name)).unwrap();
        std::fs::write(self.root.join(format!("dev/bus/usb/001/{number:03}")), "").unwrap();
        if sane {
            std::fs::write(self.root.join(format!("udev/c189:{minor}")), "S:bus/usb/001/x\nE:libsane_matched=yes\n").unwrap();
        } else {
            std::fs::write(self.root.join(format!("udev/c189:{minor}")), "S:bus/usb/001/x\nE:ID_VENDOR=Example\n").unwrap();
        }
    }
}

fn desk(tag: &str) -> Tree {
    let t = Tree::new(tag);
    t.add("1-1", 2, 0xff, &[0xff], true);
    t.add("1-2", 3, 0x00, &[0x06], false);
    t.add("1-6", 7, 0xff, &[0xff, 0x07], true);
    t.add("1-3", 4, 0x00, &[0x03], false);
    t.add("1-4", 5, 0x00, &[0x08], false);
    t.add("1-5", 6, 0xff, &[0xff, 0x08], true);
    t.add("1-7", 8, 0xef, &[0x0e, 0x0e, 0x01], false);
    t.add("1-8", 9, 0xff, &[0xff], false);
    t.add("1-9", 10, 0x06, &[0x06, 0x03], false);
    t.add("1-10", 11, 0x09, &[0x09], false);
    t.add("1-11", 12, 0xe0, &[0xe0], false);
    t.add("1-12", 13, 0x00, &[], true);
    t.add("1-13", 14, 0xff, &[0xff, 0x02, 0x0a], true);
    t.add("1-4.2", 15, 0x00, &[0x06], false);
    std::os::unix::fs::symlink("../../../devices/pci0000:00/0000:00:14.0/usb1", t.root.join("sys/bus/usb/devices/usb1")).unwrap();
    t
}

#[test]
fn only_scanners_and_image_devices_are_let_in() {
    let t = desk("only");
    let names: Vec<String> = scanners(&t.roots()).into_iter().map(|d| d.name).collect();
    assert_eq!(names, ["1-1", "1-2", "1-4.2", "1-6"]);
}

#[test]
fn the_rule_lets_in_an_interface_class_only_when_it_is_imaging_a_printer_or_vendor_specific() {
    for class in 0..=255u8 {
        let may = matches!(class, 0x06 | 0x07 | 0xff);
        assert_eq!(allowed_kind(0x00, &[class], true), may, "interface class {class:02x} with SANE's word");
        assert_eq!(allowed_kind(0x00, &[class], false), class == 0x06, "interface class {class:02x} without");
        assert_eq!(allowed_kind(0x00, &[0x06, class], false), may, "imaging beside {class:02x}");
        assert_eq!(allowed_kind(0xff, &[0xff, class], true), may, "vendor beside {class:02x}");
        assert_eq!(allowed_kind(class, &[0x06], false), matches!(class, 0x00 | 0x06 | 0xef | 0xff), "device class {class:02x}");
    }
    assert!(!allowed_kind(0x00, &[], true), "a device with no interface says nothing it is");
}

#[test]
fn a_hostile_device_cannot_make_a_path_of_its_own() {
    let t = Tree::new("hostile");
    t.add("1-1", 2, 0x00, &[0x06], false);
    std::fs::write(t.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1/1-1/devnum"), "../../../etc\n").unwrap();
    assert!(scanners(&t.roots()).is_empty());
    std::fs::write(t.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1/1-1/devnum"), "0\n").unwrap();
    assert!(scanners(&t.roots()).is_empty());
    std::fs::write(t.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1/1-1/devnum"), "1000\n").unwrap();
    assert!(scanners(&t.roots()).is_empty());
    std::os::unix::fs::symlink("../../../devices/pci0000:00/0000:00:14.0/usb1/1-1", t.root.join("sys/bus/usb/devices/..evil")).unwrap();
    std::fs::write(t.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1/1-1/devnum"), "2\n").unwrap();
    let names: Vec<String> = scanners(&t.roots()).into_iter().map(|d| d.name).collect();
    assert_eq!(names, ["1-1"]);
    std::fs::write(t.root.join("sys/devices/pci0000:00/0000:00:14.0/usb1/1-1/dev"), "../x\n").unwrap();
    assert!(!sane_knows(&t.roots(), Some("../x")));
}

#[test]
fn no_more_than_the_most_are_let_in() {
    let t = Tree::new("many");
    for n in 0..20u32 {
        t.add(&format!("1-{}", n + 1), n + 2, 0x00, &[0x06], false);
    }
    assert_eq!(scanners(&t.roots()).len(), MOST);
}

#[test]
fn the_arguments_name_each_device_and_nothing_around_it() {
    let t = desk("args");
    let roots = t.roots();
    let a = args_for(&roots, &scanners(&roots));
    let text = a.join(" ");
    let root = t.root.display();
    for number in [2, 3, 7, 15] {
        let node = format!("/dev/bus/usb/001/{number:03}");
        assert!(text.contains(&format!("--dev-bind {root}/dev{} {node}", node.trim_start_matches("/dev"))), "{node}: {text}");
    }
    for number in [4, 5, 6, 8, 9, 10, 11, 12, 13, 14] {
        assert!(!text.contains(&format!("/dev/bus/usb/001/{number:03}")), "{number}: {text}");
    }
    assert!(text.contains("--ro-bind") && text.contains("--symlink /sys/devices/pci0000:00/0000:00:14.0/usb1/1-1 /sys/bus/usb/devices/1-1"), "{text}");
    for whole in ["--dev-bind /dev /dev", "--ro-bind /sys /sys", "--bind /sys", "--dev-bind /dev/bus", "--ro-bind /run/udev /run/udev"] {
        assert!(!text.contains(whole), "{whole}");
    }
    assert!(text.contains("/run/udev/data/c189:1"), "{text}");
    assert!(a.iter().filter(|x| *x == "--dev-bind").count() == 4, "{text}");
}

fn folder(tag: &str) -> PathBuf {
    let at = PathBuf::from(format!("/var/tmp/trindle-usb-box-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(at.join("app")).unwrap();
    std::fs::create_dir_all(at.join("home")).unwrap();
    at
}

fn in_a_box(at: &Path, allowed: &[&str], usb: Vec<String>, command: &[&str]) -> (bool, String) {
    crate::common::needs::program("bwrap");
    let gates = Gates { usb, ..Gates::default() };
    let allowed: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
    let command: Vec<String> = command.iter().map(|s| s.to_string()).collect();
    let out = std::process::Command::new("bwrap").args(walls(at, &allowed, &gates, &command)).output().unwrap();
    (out.status.success(), String::from_utf8_lossy(&out.stdout).into_owned())
}

#[test]
fn a_real_box_sees_those_devices_only_and_a_box_without_the_permission_sees_none() {
    let t = desk("box");
    let roots = t.roots();
    let usb = args_for(&roots, &scanners(&roots));
    let at = folder("real");
    let look = ["/usr/bin/find", "/dev/bus", "/sys/bus/usb/devices", "/run/udev/data"];
    let (ok, seen) = in_a_box(&at, &["usb-devices"], usb.clone(), &look);
    assert!(ok || !seen.is_empty(), "the box ran: {seen}");
    for number in [2, 3, 7, 15] {
        assert!(seen.lines().any(|l| l == format!("/dev/bus/usb/001/{number:03}")), "{number}: {seen}");
    }
    for number in [4, 5, 6, 8, 9, 10, 11, 12, 13, 14] {
        assert!(!seen.contains(&format!("/dev/bus/usb/001/{number:03}")), "{number}: {seen}");
    }
    for name in ["1-1", "1-2", "1-4.2", "1-6"] {
        assert!(seen.lines().any(|l| l == format!("/sys/bus/usb/devices/{name}")), "{name}: {seen}");
    }
    for name in ["1-3", "1-4", "1-5", "1-7", "1-8", "1-9", "1-10", "1-11", "1-12", "1-13", "usb1"] {
        assert!(!seen.lines().any(|l| l == format!("/sys/bus/usb/devices/{name}")), "{name}: {seen}");
    }
    let (_, read) = in_a_box(&at, &["usb-devices"], usb.clone(), &["/usr/bin/cat", "/sys/bus/usb/devices/1-2/bDeviceClass", "/sys/bus/usb/devices/1-3/bDeviceClass"]);
    assert_eq!(read, "00\n");
    let (_, none) = in_a_box(&at, &[], usb.clone(), &look);
    assert!(!none.contains("/dev/bus") && !none.contains("1-1"), "{none}");
    let (_, other) = in_a_box(&at, &["network", "files", "sound"], usb, &look);
    assert!(!other.contains("/dev/bus") && !other.contains("1-1"), "{other}");
    let _ = std::fs::remove_dir_all(&at);
}
