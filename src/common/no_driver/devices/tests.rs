//! Made-up /sys trees and made-up ids files; the real ones are never read.

use super::*;
use crate::common::no_driver::tests::{pci, tree, usb_face};
use std::path::PathBuf;

const PCI_TEXT: &str = "# list\n8086  Intel Corporation\n\t9a49  Iris Xe Graphics\n\t\t1028 0a5f  Subsystem\n14c3  MediaTek Inc.\n\t7902  MT7902 Wi-Fi\nC 03  Display controller\n";
const USB_TEXT: &str = "8087  Intel Corp.\n\t0033  AX211 Bluetooth\n";

fn ids(sys: &Path, pci: &str, usb: &str) -> (PathBuf, PathBuf) {
    let (p, u) = (sys.join("pci.ids"), sys.join("usb.ids"));
    std::fs::write(&p, pci).unwrap();
    std::fs::write(&u, usb).unwrap();
    (p, u)
}

fn by_id<'a>(inv: &'a Inventory, id: &str) -> &'a Device {
    inv.devices.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("no {id}"))
}

#[test]
fn a_bound_an_unbound_and_a_missing_firmware_device_are_listed_right() {
    let sys = tree("devices");
    pci(&sys, "0000:00:02.0", "0x030000", "0x8086", "0x9a49", true);
    pci(&sys, "0000:03:00.0", "0x028000", "0x14c3", "0x7902", false);
    pci(&sys, "0000:04:00.0", "0x040300", "0x8086", "0x51c8", false);
    pci(&sys, "0000:00:1c.0", "0x060400", "0x8086", "0x1111", false);
    usb_face(&sys, "1-4", "1-4:1.0", ("8087", "0033"), ("0xe0", "0x01", "0x01"), true);
    let (p, u) = ids(&sys, PCI_TEXT, USB_TEXT);
    let log = "bt: Direct firmware load for mediatek/BT_RAM_CODE_MT7902_1_1_hdr.bin failed with error -2\n\
               snd: Direct firmware load for intel/sof/sof-tgl.ri failed with error -2";
    let inv = inventory_in(&sys, Some(log), &p, &u);

    let gpu = by_id(&inv, "8086:9a49");
    assert_eq!((gpu.kind, gpu.state), (Kind::Graphics, State::Working));
    assert_eq!(gpu.name.as_deref(), Some("Intel Corporation Iris Xe Graphics"));
    assert_eq!(gpu.driver.as_deref(), Some("x"));

    let wifi = by_id(&inv, "14c3:7902");
    assert_eq!((wifi.kind, wifi.state), (Kind::Network, State::NoDriver));
    assert!(wifi.problem.as_deref().unwrap().contains("MediaTek MT7902"));

    let sound = by_id(&inv, "8086:51c8");
    assert_eq!(sound.state, State::NoFirmware);
    assert!(sound.problem.as_deref().unwrap().contains("intel/sof/sof-tgl.ri"));
    assert_eq!(by_id(&inv, "8086:1111").state, State::Idle);

    let bt = by_id(&inv, "8087:0033");
    assert_eq!((bt.kind, bt.state), (Kind::Bluetooth, State::NoFirmware));
    assert_eq!(bt.name.as_deref(), Some("Intel Corp. AX211 Bluetooth"));
    assert_eq!(inv.problems().len(), 3);
    assert_eq!(inv.of_kind(Kind::Network).len(), 1);
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_device_the_ids_files_do_not_know_has_no_name_and_still_a_line() {
    let sys = tree("unnamed");
    pci(&sys, "0000:05:00.0", "0x020000", "0x10ec", "0x8168", false);
    let (p, u) = ids(&sys, "", "");
    let inv = inventory_in(&sys, None, &p, &u);
    assert_eq!(inv.devices[0].name, None);
    assert_eq!(inv.devices[0].kind, Kind::Network);
    assert!(inv.problems()[0].contains("10ec:8168"), "{:?}", inv.problems());
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_memory_controller_or_bridge_with_no_driver_is_not_a_fault() {
    let sys = tree("pmc");
    pci(&sys, "0000:00:1f.2", "0x058000", "0x8086", "0x9d21", false);
    pci(&sys, "0000:00:1f.0", "0x060100", "0x8086", "0x9d4e", false);
    let (p, u) = ids(&sys, "", "");
    let inv = inventory_in(&sys, None, &p, &u);
    assert!(inv.problems().is_empty(), "{:?}", inv.problems());
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn missing_garbage_and_huge_ids_files_do_not_crash_or_hang() {
    let sys = tree("hostile");
    pci(&sys, "0000:00:02.0", "0x030000", "0x8086", "0x9a49", true);
    let huge = format!("8086  Intel\n\t9a49  {}\n{}", "x".repeat(10_000_000), "\t9a49  again\n".repeat(100_000));
    for text in [huge.as_str(), "\u{0}\u{1}\t\t\t  \n8086\n\t  \nzzzz  Bad\n\tzzzz  Bad\n", "8086  Intel\n\t9a49  ev\u{7}il\u{1b}[31m name\n"] {
        let (p, u) = ids(&sys, text, text);
        let inv = inventory_in(&sys, None, &p, &u);
        let name = inv.devices[0].name.clone().unwrap_or_default();
        assert!(name.chars().all(|c| !c.is_control()) && name.chars().count() <= NAME_MOST + 20, "{name}");
    }
    let nowhere = sys.join("nothing");
    assert_eq!(inventory_in(&sys, None, &nowhere, &nowhere).devices[0].name, None);
    std::fs::remove_dir_all(&sys).unwrap();
}

#[test]
fn a_garbage_class_file_and_a_crowd_of_devices_are_kept_in_bounds() {
    let sys = tree("crowd");
    for i in 0..(DEVICES_MOST + 50) {
        pci(&sys, &format!("0000:{:02x}:{:02x}.0", i / 32, i % 32), "0x030000", "0x8086", "0x0001", true);
    }
    let d = sys.join("bus/pci/devices/0000:00:00.0");
    std::fs::write(d.join("class"), "x".repeat(1_000_000)).unwrap();
    let (p, u) = ids(&sys, "", "");
    let inv = inventory_in(&sys, None, &p, &u);
    assert!(inv.devices.len() <= DEVICES_MOST);
    std::fs::remove_dir_all(&sys).unwrap();
}
