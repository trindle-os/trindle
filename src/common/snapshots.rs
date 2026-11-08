//! Btrfs system snapshots via snapper, and the GRUB submenu that boots them.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::common::run::text;

pub const CONFIG: &str = "root";

pub const READ_ONLY_MARKER: &str = "/etc/trindle/read-only-system";

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub number: u32,
    pub kind: String,
    pub date: String,
    pub description: String,
    pub important: bool,
}

impl Snapshot {
    pub fn what(&self) -> String {
        crate::common::snapshot_title::what_alone(&self.kind, &self.description)
    }
}

pub fn root_fs() -> String {
    Command::new("findmnt").args(["-no", "FSTYPE", "/"]).output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
}

pub fn root_subvolume() -> String {
    Command::new("findmnt").args(["-no", "FSROOT", "/"]).output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
}

pub fn home_separate() -> bool {
    Command::new("findmnt").args(["-no", "TARGET", "/home"]).output().ok()
        .is_some_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "/home")
}

pub fn available() -> bool {
    Path::new("/etc/snapper/configs").join(CONFIG).exists()
}

pub const SET_ASIDE: &str = "/var/lib/trindle/set-aside";

pub fn set_aside() -> Vec<String> {
    std::fs::read_to_string(SET_ASIDE).unwrap_or_default().lines()
        .map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

pub fn booted_snapshot() -> Option<u32> {
    let cmdline = std::fs::read_to_string("/proc/cmdline").unwrap_or_default();
    cmdline.split_whitespace().find_map(|w| w.strip_prefix("trindle.snapshot=")).and_then(|n| n.parse().ok())
}

pub fn parse_list(json: &str) -> Vec<Snapshot> {
    let v: Value = serde_json::from_str(json).unwrap_or(Value::Null);
    let mut out: Vec<Snapshot> = v[CONFIG].as_array().map(|a| a.iter().filter_map(|s| {
        let number = s["number"].as_u64()? as u32;
        (number != 0).then(|| Snapshot {
            number,
            kind: s["type"].as_str().unwrap_or("single").to_string(),
            date: s["date"].as_str().unwrap_or("").to_string(),
            description: s["description"].as_str().unwrap_or("").to_string(),
            important: s["userdata"]["important"].as_str() == Some("yes"),
        })
    }).collect()).unwrap_or_default();
    out.sort_by_key(|s| std::cmp::Reverse(s.number));
    out
}

pub(crate) fn xml_field(xml: &str, tag: &str) -> String {
    let open = format!("<{tag}>");
    xml.split(&open).nth(1).and_then(|r| r.split(&format!("</{tag}>")).next()).unwrap_or("")
        .replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'")
}

fn grub_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn kernel_in(boot: &Path) -> Option<(String, String)> {
    let mut kernels: Vec<String> = std::fs::read_dir(boot).ok()?.flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.starts_with("vmlinuz-"))
        .collect();
    kernels.sort_by(|a, b| version_order(a, b));
    let k = kernels.pop()?;
    let initrd = format!("initrd.img-{}", k.trim_start_matches("vmlinuz-"));
    boot.join(&initrd).exists().then_some((k, initrd))
}

fn version_order(a: &str, b: &str) -> std::cmp::Ordering {
    let key = |s: &str| s.split(|c: char| !c.is_ascii_digit()).filter_map(|p| p.parse::<u64>().ok()).collect::<Vec<_>>();
    key(a).cmp(&key(b))
}

pub fn grub_main() -> i32 {
    if !available() || root_fs() != "btrfs" {
        return 0;
    }
    let sub = root_subvolume();
    if sub.is_empty() || sub == "/" {
        return 0;
    }
    let uuid = text(&["findmnt", "-no", "UUID", "/"]);
    if uuid.is_empty() {
        return 0;
    }

    let boot_separate = text(&["findmnt", "-no", "TARGET", "/boot"]) == "/boot";
    let boot_uuid = if boot_separate { text(&["findmnt", "-no", "UUID", "/boot"]) } else { uuid.clone() };
    let mut numbers: Vec<u32> = std::fs::read_dir("/.snapshots").map(|d| d.flatten()
        .filter_map(|e| e.file_name().to_string_lossy().parse().ok()).collect()).unwrap_or_default();
    numbers.sort_unstable_by(|a, b| b.cmp(a));

    let own = format!("{} {}", std::env::var("GRUB_CMDLINE_LINUX").unwrap_or_default(),
                      std::env::var("GRUB_CMDLINE_LINUX_DEFAULT").unwrap_or_else(|_| "quiet".into()));
    let extra = format!("{} systemd.mask=systemd-remount-fs.service systemd.mask=snapper-boot.service",
                        own.split_whitespace().filter(|w| !w.starts_with("root=") && !w.starts_with("rootflags="))
                            .collect::<Vec<_>>().join(" "));
    let mut entries = Vec::new();
    for n in numbers.into_iter().take(12) {
        let base = PathBuf::from(format!("/.snapshots/{n}"));
        let info = std::fs::read_to_string(base.join("info.xml")).unwrap_or_default();
        if !base.join("snapshot").is_dir() {
            continue;
        }
        let boot_dir = if boot_separate { PathBuf::from("/boot") } else { base.join("snapshot/boot") };
        let Some((kernel, initrd)) = kernel_in(&boot_dir) else { continue };
        let snap_sub = format!("{}/.snapshots/{n}/snapshot", sub.trim_end_matches('/'));
        let prefix = if boot_separate { String::new() } else { format!("{snap_sub}/boot") };
        let desc = xml_field(&info, "description");
        let kind = xml_field(&info, "type");
        let title = crate::common::snapshot_title::title(n, &xml_field(&info, "date"), &kind, &desc);
        entries.push(format!(
            "    menuentry {} --class trindle {{\n\
             \x20       insmod btrfs\n\
             \x20       search --no-floppy --fs-uuid --set=root {boot_uuid}\n\
             \x20       echo 'Starting from snapshot {n}. Nothing done here is kept.'\n\
             \x20       linux {prefix}/{kernel} root=UUID={uuid} rootflags=subvol={} ro trindle.snapshot={n} {extra}\n\
             \x20       initrd {prefix}/{initrd}\n\
             \x20   }}\n",
            grub_quote(&title), snap_sub.trim_start_matches('/')));
    }
    if entries.is_empty() {
        return 0;
    }
    println!("submenu 'Trindle snapshots' {{");
    for e in entries {
        print!("{e}");
    }
    println!("}}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshots_are_named_by_the_clock_and_what_they_are() {
        use crate::common::snapshot_title::{local_label, what_it_is};
        assert!(local_label("2026-10-04 09:34:12").is_some());
        assert!(local_label("not a date").is_none());
        assert_eq!(what_it_is("single", "boot"), "when the computer started");
    }

    #[test]
    fn snapper_list_is_read() {
        let json = r#"{"root":[{"number":0,"type":"single","pre-number":null,"date":"","description":"current","userdata":null},
            {"number":7,"type":"pre","pre-number":null,"date":"2026-09-19 20:00:00","description":"apt","userdata":{"important":"yes"}},
            {"number":8,"type":"post","pre-number":7,"date":"2026-09-19 20:01:00","description":"apt","userdata":null}]}"#;
        let list = parse_list(json);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].number, 8);
        assert_eq!(list[1].what(), "Before a software change");
        assert!(list[1].important);
    }

    #[test]
    fn info_xml_fields() {
        let xml = "<snapshot>\n  <type>pre</type>\n  <num>7</num>\n  <date>2026-09-19 20:00:00</date>\n  <description>A &amp; B</description>\n</snapshot>";
        assert_eq!(xml_field(xml, "description"), "A & B");
        assert_eq!(xml_field(xml, "type"), "pre");
        assert_eq!(grub_quote("it's"), "'it'\\''s'");
    }
}

pub fn list_argv() -> [&'static str; 6] {
    ["snapper", "--utc", "--jsonout", "-c", CONFIG, "list"]
}

#[cfg(test)]
mod list_tests {
    #[test]
    fn the_list_asks_for_dates_in_utc_everywhere() {
        let a = super::list_argv();
        assert!(a.contains(&"--utc"), "snapper's list is read in UTC");
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for f in [
            "src/settings/panels/snapshots.rs", "src/disks/mod.rs", "src/disks/actions.rs", "src/disks/app.rs", "src/disks/cards.rs", "src/disks/job.rs",
            "src/disks/map.rs", "src/disks/model.rs", "src/disks/refresh.rs", "src/disks/rows.rs", "src/disks/steps.rs", "src/disks/udisks.rs",
            "src/disks/view.rs",
        ] {
            let text = std::fs::read_to_string(root.join(f)).unwrap();
            assert!(!text.contains("\"snapper\", \"--jsonout\""), "{f} builds its own snapper list command");
        }
    }
}
