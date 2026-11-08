//! Made-up /sys trees and made-up logs; the real /sys is never read.

use super::*;
use std::path::PathBuf;

fn tree(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("trindle-no-driver-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("bus/pci/devices")).unwrap();
    std::fs::create_dir_all(d.join("bus/usb/devices")).unwrap();
    d
}

fn pci(sys: &Path, addr: &str, class: &str, vendor: &str, device: &str, driver: bool) {
    let d = sys.join("bus/pci/devices").join(addr);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("class"), format!("{class}\n")).unwrap();
    std::fs::write(d.join("vendor"), format!("{vendor}\n")).unwrap();
    std::fs::write(d.join("device"), format!("{device}\n")).unwrap();
    if driver {
        std::fs::create_dir_all(sys.join("drivers/x")).unwrap();
        std::os::unix::fs::symlink(sys.join("drivers/x"), d.join("driver")).unwrap();
    }
}

fn usb_face(sys: &Path, dev: &str, face: &str, id: (&str, &str), class: (&str, &str, &str), driver: bool) {
    let root = sys.join("bus/usb/devices");
    let d = root.join(dev);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("idVendor"), format!("{}\n", id.0)).unwrap();
    std::fs::write(d.join("idProduct"), format!("{}\n", id.1)).unwrap();
    let f = root.join(face);
    std::fs::create_dir_all(&f).unwrap();
    std::fs::write(f.join("bInterfaceClass"), format!("{}\n", class.0)).unwrap();
    std::fs::write(f.join("bInterfaceSubClass"), format!("{}\n", class.1)).unwrap();
    std::fs::write(f.join("bInterfaceProtocol"), format!("{}\n", class.2)).unwrap();
    if driver {
        std::fs::create_dir_all(sys.join("drivers/u")).unwrap();
        std::os::unix::fs::symlink(sys.join("drivers/u"), f.join("driver")).unwrap();
    }
}

#[test]
fn a_known_chip_with_no_driver_is_found_and_named() {
    let sys = tree("known");
    pci(&sys, "0000:03:00.0", "0x028000", "0x14c3", "0x7902", false);
    let found = scan_in(&sys, None);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].class, Class::Wifi);
    let said = words(&found[0]);
    assert!(said.contains("MediaTek MT7902") && said.contains("7.1") && said.contains("USB Wi-Fi stick"), "{said}");
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_chip_with_a_driver_bound_says_nothing() {
    let sys = tree("bound");
    pci(&sys, "0000:03:00.0", "0x028000", "0x14c3", "0x7902", true);
    pci(&sys, "0000:00:02.0", "0x030000", "0x8086", "0x9a49", true);
    pci(&sys, "0000:00:1f.3", "0x040300", "0x8086", "0x51c8", true);
    usb_face(&sys, "1-3", "1-3:1.0", ("8087", "0033"), ("e0", "01", "01"), true);
    assert!(scan_in(&sys, Some("")).is_empty());
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn an_unknown_chip_with_no_driver_still_gets_a_plain_line() {
    let sys = tree("unknown");
    pci(&sys, "0000:00:02.0", "0x030000", "0x1234", "0xabcd", false);
    pci(&sys, "0000:00:1f.3", "0x040300", "0x1111", "0x2222", false);
    pci(&sys, "0000:00:14.0", "0x0c0330", "0x8086", "0x0001", false);
    usb_face(&sys, "1-4", "1-4:1.0", ("0a12", "0001"), ("e0", "01", "01"), false);
    let found = scan_in(&sys, None);
    assert_eq!(found.len(), 3, "{found:?}");
    let all: Vec<String> = found.iter().map(words).collect();
    assert!(all.iter().any(|w| w.contains("1234:abcd")), "{all:?}");
    assert!(all.iter().any(|w| w.contains("1111:2222")), "{all:?}");
    assert!(all.iter().any(|w| w.contains("0a12:0001")), "{all:?}");
    assert!(all.iter().all(|w| !w.contains('\u{2014}')));
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_known_usb_chip_with_no_driver_is_found_by_its_numbers_and_a_bound_one_is_not() {

    let sys = tree("usb");
    usb_face(&sys, "1-1", "1-1:1.0", ("0d8c", "0014"), ("01", "01", "00"), false);
    usb_face(&sys, "1-2", "1-2:1.0", ("0d8c", "0015"), ("01", "01", "00"), true);
    let found = scan_in(&sys, None);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].class, Class::Sound);
    assert_eq!(found[0].id, "0d8c:0014");
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_missing_firmware_file_in_the_log_is_found_for_a_known_chip() {
    let sys = tree("firmware");
    let log = "Bluetooth: hci1: Direct firmware load for mediatek/BT_RAM_CODE_MT7902_1_1_hdr.bin failed with error -2\n\
               Bluetooth: hci1: Failed to load firmware file\n";
    let found = scan_in(&sys, Some(log));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].class, Class::Bluetooth);
    let said = words(&found[0]);
    assert!(said.contains("MediaTek MT7902") && said.contains("firmware-mediatek") && said.contains("20260916"), "{said}");
    assert!(!said.contains("USB Wi-Fi stick"));
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_failed_try_beside_a_working_driver_is_not_a_fault() {
    let sys = tree("tries");
    pci(&sys, "0000:03:00.0", "0x028000", "0x8086", "0x2725", true);
    let log = "iwlwifi 0000:03:00.0: Direct firmware load for iwlwifi-ty-a0-gf-a0-89.ucode failed with error -2\n";
    assert!(scan_in(&sys, Some(log)).is_empty());
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn an_unknown_firmware_file_is_said_beside_a_chip_with_no_driver() {
    let sys = tree("generic-fw");
    pci(&sys, "0000:03:00.0", "0x028000", "0x8086", "0x2725", false);
    let log = "iwlwifi 0000:03:00.0: Direct firmware load for iwlwifi-ty-a0-gf-a0-89.ucode failed with error -2\n";
    let found = scan_in(&sys, Some(log));
    assert_eq!(found.len(), 1);
    let said = words(&found[0]);
    assert!(said.contains("iwlwifi-ty-a0-gf-a0-89.ucode") && said.contains("USB Wi-Fi stick"), "{said}");
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn an_unreadable_log_says_nothing_about_firmware() {
    let sys = tree("no-log");
    assert!(scan_in(&sys, None).is_empty());
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn lines_are_kept_per_class_and_the_words_are_in_the_word_file() {
    let sys = tree("classes");
    pci(&sys, "0000:03:00.0", "0x028000", "0x14c3", "0x7902", false);
    pci(&sys, "0000:00:02.0", "0x030000", "0x1234", "0xabcd", false);
    let found = scan_in(&sys, None);
    assert_eq!(lines_for(&found, Class::Wifi).len(), 1);
    assert_eq!(lines_for(&found, Class::Graphics).len(), 1);
    assert!(lines_for(&found, Class::Sound).is_empty() && lines_for(&found, Class::Bluetooth).is_empty());
    assert_eq!(report_lines(&found).len(), 2);
    assert_eq!(HELP, "hardware-no-driver");
    let ftl = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/share/lang/en/common.ftl")).unwrap();
    for key in ["known-driver", "known-firmware", "generic", "generic-firmware", "stick", "more", "class-wifi"] {
        assert!(ftl.contains(&format!("common-no-driver-{key} =")), "{key}");
    }
    std::fs::remove_dir_all(&sys).unwrap();
}
