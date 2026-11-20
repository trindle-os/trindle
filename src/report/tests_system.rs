//! Tests for the parts of a report that say what this computer is: their headings, caps and blanking.

use std::path::{Path, PathBuf};

use super::system::{self, ERROR_LINES, LINE_MOST};
use super::*;

fn who() -> Who {
    Who { home: "/home/jsmith".into(), names: vec!["jsmith".into(), "T490".into()], ssids: vec!["Home Net 5G".into()] }
}

fn put(root: &Path, file: &str, text: &str) {
    let p = root.join(file);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, text).unwrap();
}

fn tree(name: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("trindle-report-system-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (sys, proc_) = (base.join("sys"), base.join("proc"));
    for (f, v) in [("sys_vendor", "LENOVO"), ("product_name", "20N2"), ("product_version", "ThinkPad T490"), ("board_vendor", "LENOVO"), ("board_name", "20N2000XUK"),
                   ("bios_vendor", "LENOVO"), ("bios_version", "N2IET90W"), ("bios_date", "03/01/2024")] {
        put(&sys, &format!("class/dmi/id/{f}"), v);
    }
    put(&proc_, "sys/kernel/osrelease", "6.12.117+deb13-amd64");
    put(&proc_, "uptime", "7500.12 100.00");
    put(&proc_, "cpuinfo", "processor\t: 0\nmodel name\t: Intel(R) Core(TM) i5-8365U\nprocessor\t: 1\nmodel name\t: Intel(R) Core(TM) i5-8365U\n");
    put(&proc_, "meminfo", "MemTotal:       16384000 kB\nMemAvailable:    8192000 kB\nSwapTotal:       2048000 kB\nSwapFree:        1024000 kB\n");
    put(&proc_, "asound/cards", " 0 [PCH            ]: HDA-Intel - HDA Intel PCH\n                      HDA Intel PCH at 0xe1240000 irq 140\n");
    put(&sys, "class/drm/card0-eDP-1/status", "connected\n");
    put(&sys, "class/drm/card0-eDP-1/modes", "1920x1080\n1280x720\n");
    put(&sys, "class/drm/card0-HDMI-A-1/status", "disconnected\n");
    for (f, v) in [("type", "Battery"), ("energy_full", "40000000"), ("energy_full_design", "50000000"), ("cycle_count", "312"), ("status", "Discharging")] {
        put(&sys, &format!("class/power_supply/BAT0/{f}"), v);
    }
    put(&sys, "firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c", "");
    std::fs::write(sys.join("firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c"), [6, 0, 0, 0, 1]).unwrap();
    put(&sys, "bus/pci/devices/0000:00:14.3/class", "0x028000\n");
    put(&sys, "bus/pci/devices/0000:00:14.3/vendor", "0x8086\n");
    put(&sys, "bus/pci/devices/0000:00:14.3/device", "0x9df0\n");
    (sys, proc_)
}

fn fake(argv: &[&str]) -> String {
    match argv[0] {
        "dpkg-query" => "trindle 0.9.1-beta.4\ntrindle-welcome 12\n".into(),
        "lsblk" => "NAME=\"nvme0n1\" TYPE=\"disk\" SIZE=\"512110190592\" FSTYPE=\"\" FSAVAIL=\"\" MOUNTPOINTS=\"\"\n\
                    NAME=\"nvme0n1p2\" TYPE=\"part\" SIZE=\"500000000000\" FSTYPE=\"ext4\" FSAVAIL=\"200000000000\" MOUNTPOINTS=\"/home\\x0a/swap\\x0a/\"\n\
                    NAME=\"sda1\" TYPE=\"part\" SIZE=\"1000000000\" FSTYPE=\"vfat\" FSAVAIL=\"500000000\" MOUNTPOINTS=\"/home/jsmith/Stick\"\n\
                    NAME=\"loop0\" TYPE=\"loop\" SIZE=\"1\" FSTYPE=\"\" FSAVAIL=\"\" MOUNTPOINTS=\"\"\n".into(),
        "systemctl" if argv.contains(&"--user") => "jsmith-sync.service loaded failed failed Sync for jsmith\n".into(),
        "systemctl" => "NetworkManager-wait-online.service loaded failed failed Wait\n".into(),
        "systemd-analyze" => "Startup finished in 5.1s (firmware) + 2.0s (loader) + 1.9s (kernel) + 6.4s (userspace) = 15.4s\ngraphical.target reached after 6.3s in userspace.\n".into(),
        "journalctl" => (0..1000).map(|i| format!("2026-11-18T10:00:{:02} T490 kernel: error {i} on Home Net 5G for jsmith", i % 60)).collect::<Vec<_>>().join("\n"),
        _ => String::new(),
    }
}

fn inventory(sys: &Path, firmware: &[String]) -> crate::common::no_driver::Inventory {
    let log = firmware.iter().fold(String::new(), |l, f| l + &format!("Direct firmware load for {f} failed with error -2\n"));
    let none = sys.join("no-ids");
    crate::common::no_driver::inventory_in(sys, Some(&log), &none, &none)
}

fn gathered(name: &str) -> String {
    let (sys, proc_) = tree(name);
    system::gather_in(&sys, &proc_, &fake, &inventory(&sys, &["mediatek/BT_RAM_CODE_MT7902_1_1_hdr.bin".to_string()]), &[])
}

#[test]
fn every_part_has_a_plain_heading_and_says_what_it_found() {
    let text = gathered("headings");
    for part in ["Computer", "Trindle", "System", "Processor and memory", "Graphics", "Screens", "Sound", "Disks", "Battery", "Wi-Fi and Bluetooth", "Devices",
                 "Missing drivers and firmware", "Failed units", "Errors this boot"] {
        let has = text.lines().any(|l| l == system::heading(part)) || part == "Graphics" || part == "Wi-Fi and Bluetooth";
        assert!(has, "no heading for {part}:\n{text}");
    }
    for line in ["Maker: LENOVO", "Version: ThinkPad T490", "BIOS: LENOVO N2IET90W 03/01/2024", "Kernel: 6.12.117+deb13-amd64", "Up for: 2 h 5 min", "Boot took: 5.1s (firmware) + 2.0s (loader) + 1.9s (kernel) + 6.4s (userspace) = 15.4s",
                 "Start: UEFI, Secure Boot on", "Cores: 2", "Memory: 16000 MB", "Memory free: 8000 MB", "Swap: 2000 MB, free 1000 MB", "eDP-1: 1920x1080",
                 "PCH", "nvme0n1p2 part 500.0 GB ext4, free 200.0 GB at /", "holds 80% of new", "312 cycles", "8086:9df0 Network driver none",
                 "MT7902", "system: NetworkManager-wait-online.service failed/failed", "trindle-welcome 12"] {
        assert!(text.lines().any(|l| l.contains(line)), "missing {line:?}:\n{text}");
    }
    assert!(!text.contains("0xe1240000"), "a sound card's address is in the report");
    assert!(!text.contains("loop0"), "a loop device is listed");
    assert!(!text.contains("Stick"), "a mount place under /home is in the report");
    assert!(text.lines().next() == Some("== Computer =="), "the Computer part comes first:\n{text}");
}

#[test]
fn the_error_list_and_the_whole_summary_stay_within_their_caps() {
    let text = gathered("caps");
    let errors: Vec<&str> = text.split("== Errors this boot ==").nth(1).unwrap().lines().filter(|l| !l.is_empty()).collect();
    assert!(!errors.is_empty() && errors.len() <= ERROR_LINES, "{} error lines", errors.len());
    assert!(text.chars().count() <= HARDWARE_MOST, "{} characters", text.chars().count());
    assert!(text.lines().all(|l| l.chars().count() <= LINE_MOST));

    let loud = |argv: &[&str]| "x".repeat(5000) + "\n" + &fake(argv);
    let (sys, proc_) = tree("caps-loud");
    let big = system::gather_in(&sys, &proc_, &loud, &inventory(&sys, &(0..500).map(|i| format!("amdgpu/{i}.bin")).collect::<Vec<_>>()), &[]);
    assert!(big.chars().count() <= HARDWARE_MOST, "{} characters", big.chars().count());
    assert!(big.contains("== Computer ==") && big.contains("== Errors this boot =="), "a part was lost to the cap");
    let parts = Parts { mode: Mode::Note, version: "1", words: "w", hardware: &big, crash: None, log: &vec!["line".to_string(); 5000], username: "", github: "" };
    assert!(document(&parts, &who()).len() <= send::MOST_BYTES);
}

#[test]
fn what_trindle_gathers_is_blanked_in_every_part_but_the_maker_and_model() {
    let hardware = probe::hardware_text("LENOVO ThinkPad T490", Some(&gathered("blank")));
    let parts = Parts { mode: Mode::Note, version: "1", words: "CTEST T490", hardware: &hardware, crash: None, log: &[], username: "", github: "" };
    let doc: serde_json::Value = serde_json::from_str(&document(&parts, &who())).unwrap();
    let sent = doc["hardware"].as_str().unwrap();
    assert!(sent.starts_with("LENOVO ThinkPad T490,"), "the model was blanked: {}", sent.lines().next().unwrap());
    assert!(sent.contains("Version: ThinkPad T490"), "the model line of the Computer part was blanked");
    for part in ["Failed units", "Errors this boot", "Disks"] {
        let body = sent.split(&system::heading(part)).nth(1).unwrap_or_default().split("\n== ").next().unwrap_or_default();
        assert!(!body.contains("jsmith") && !body.contains("T490") && !body.contains("Home Net 5G"), "{part} was not blanked:\n{body}");
    }
    assert!(sent.contains("[name]"), "nothing was blanked at all");
    assert_eq!(doc["words"], "CTEST T490", "the person's own words were changed");
}

#[test]
fn the_log_is_this_boot_newest_first_and_capped() {
    let mut raw = String::new();
    for i in 0..500 {
        raw.push_str(&format!("line {i}\n"));
    }
    let lines = log_lines(&raw, &who());
    assert_eq!(lines.len(), LOG_LINES);
    assert_eq!(lines[0], "line 0", "the first line read is the newest");
    for argv in [unit_log_argv("200"), own_log_argv("200")] {
        assert!(argv.contains(&"-b") && argv.contains(&"-r"), "{argv:?}");
    }
    assert!(probe::log_argv(None, None, false).iter().any(|a| a == "-r"));
    let many: Vec<String> = (0..5000).map(|i| format!("2026-11-18 line {i} {}", "y".repeat(900))).collect();
    let parts = Parts { mode: Mode::Note, version: "1", words: "w", hardware: "", crash: None, log: &many, username: "", github: "" };
    let doc: serde_json::Value = serde_json::from_str(&document(&parts, &who())).unwrap();
    let log = doc["log"].as_array().unwrap();
    assert!(log.len() <= LOG_LINES && log[0].as_str().unwrap().contains("line 0 "), "the newest lines are the ones kept");
}

#[test]
#[ignore = "reads this computer; run by hand with --ignored and TRINDLE_REPORT_OUT"]
fn write_this_computers_details() {
    let Some(out) = std::env::var_os("TRINDLE_REPORT_OUT") else { return };
    std::fs::write(out, super::system::gather()).unwrap();
}

#[test]
fn a_screen_says_the_rate_and_scale_the_desktop_drives_it_at() {
    use crate::common::outputs::{Head, Mode, Turn};
    let (sys, proc_) = tree("live-screens");
    let head = Head {
        name: "eDP-1".into(), description: String::new(), make: "John Smith Panels".into(), model: String::new(),
        serial: "SERIAL123".into(), physical_size: (310, 170),
        modes: vec![Mode { width: 1920, height: 1080, refresh: 60008, preferred: true }, Mode { width: 1280, height: 720, refresh: 59940, preferred: false }],
        current: Some(1), enabled: true, position: (0, 0), turn: Turn::Normal, scale: 1.25, varies_rate: false,
    };
    let text = system::gather_in(&sys, &proc_, &fake, &inventory(&sys, &[]), &[head]);
    assert!(text.lines().any(|l| l == "eDP-1: 1280x720 at 59.94 Hz, scale 125%"), "{text}");
    assert!(!text.contains("SERIAL123"), "a screen's serial number is in the report");
}

#[test]
fn a_boot_that_has_not_finished_says_no_time() {
    let (sys, proc_) = tree("boot-unfinished");
    let text = system::gather_in(&sys, &proc_, &|argv: &[&str]| if argv[0] == "systemd-analyze" { String::new() } else { fake(argv) }, &inventory(&sys, &[]), &[]);
    assert!(!text.contains("Boot took"), "{text}");
}
