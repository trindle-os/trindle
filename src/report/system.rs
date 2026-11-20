//! What a report says of this computer, part by part under plain headings: read from /sys and /proc, and from a few programs that only look (no password, nothing changed).

use std::path::Path;

use super::HARDWARE_MOST;
use crate::common::no_driver;
use crate::common::outputs::Head;

pub const OPEN_PART: &str = "Computer";

const PACKAGE_LINES: usize = 60;
const UNIT_LINES: usize = 40;
const DEVICE_LINES: usize = 150;
const DISK_LINES: usize = 40;

pub const ERROR_LINES: usize = 100;

pub const LINE_MOST: usize = 200;

const SPARE: usize = 1500;
const SECURE_BOOT: &str = "firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c";

pub type Run<'a> = &'a dyn Fn(&[&str]) -> String;

pub fn heading(title: &str) -> String {
    format!("== {title} ==")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default().trim().to_string()
}

fn names(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    v.sort();
    v
}

fn cut(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(LINE_MOST).collect()
}

fn computer(sys: &Path) -> Vec<String> {
    let dmi = sys.join("class/dmi/id");
    let get = |f: &str| read(&dmi.join(f));
    let joined = |a: &str, b: &str| format!("{} {}", get(a), get(b)).trim().to_string();
    [("Maker", get("sys_vendor")), ("Model", get("product_name")), ("Version", get("product_version")), ("Board", joined("board_vendor", "board_name")),
     ("BIOS", format!("{} {} {}", get("bios_vendor"), get("bios_version"), get("bios_date")).trim().to_string())]
        .into_iter().filter(|(_, v)| !v.is_empty()).map(|(k, v)| cut(&format!("{k}: {v}"))).collect()
}

fn trindle(run: Run) -> Vec<String> {
    let mut out = vec![format!("Version: {}", env!("CARGO_PKG_VERSION"))];
    let list = run(&["dpkg-query", "-W", "-f=${Package} ${Version}\\n", "trindle*"]);
    out.extend(list.lines().filter(|l| !l.trim().is_empty()).take(PACKAGE_LINES).map(cut));
    out
}

fn up_for(seconds: u64) -> String {
    format!("{} h {} min", seconds / 3600, seconds % 3600 / 60)
}

fn system(sys: &Path, proc_: &Path, run: Run) -> Vec<String> {
    let mut out = Vec::new();
    let kernel = read(&proc_.join("sys/kernel/osrelease"));
    if !kernel.is_empty() {
        out.push(format!("Kernel: {kernel}"));
    }
    if let Some(up) = read(&proc_.join("uptime")).split('.').next().and_then(|s| s.parse::<u64>().ok()) {
        out.push(format!("Up for: {}", up_for(up)));
    }

    if let Some(took) = run(&["systemd-analyze", "time"]).lines().next().and_then(|l| l.strip_prefix("Startup finished in ")) {
        out.push(cut(&format!("Boot took: {took}")));
    }
    out.push(if !sys.join("firmware/efi").exists() {
        "Start: BIOS, not UEFI".to_string()
    } else {
        match std::fs::read(sys.join(SECURE_BOOT)).ok().and_then(|b| b.get(4).copied()) {
            Some(1) => "Start: UEFI, Secure Boot on".into(),
            Some(_) => "Start: UEFI, Secure Boot off".into(),
            None => "Start: UEFI, Secure Boot unknown".into(),
        }
    });
    out
}

fn kb(text: &str, key: &str) -> Option<u64> {
    text.lines().find_map(|l| l.strip_prefix(key)?.trim().split(' ').next()?.parse().ok())
}

fn processor_and_memory(proc_: &Path) -> Vec<String> {
    let cpu = std::fs::read_to_string(proc_.join("cpuinfo")).unwrap_or_default();
    let mem = std::fs::read_to_string(proc_.join("meminfo")).unwrap_or_default();
    let mut out = Vec::new();
    if let Some(model) = cpu.lines().find_map(|l| l.strip_prefix("model name").and_then(|r| r.split_once(':')).map(|(_, v)| v.trim().to_string())) {
        out.push(cut(&format!("Processor: {model}")));
    }
    let cores = cpu.lines().filter(|l| l.starts_with("processor")).count();
    if cores > 0 {
        out.push(format!("Cores: {cores}"));
    }
    if let Some(total) = kb(&mem, "MemTotal:") {
        out.push(format!("Memory: {} MB", total / 1024));
    }
    if let Some(free) = kb(&mem, "MemAvailable:") {
        out.push(format!("Memory free: {} MB", free / 1024));
    }
    if let (Some(total), Some(free)) = (kb(&mem, "SwapTotal:"), kb(&mem, "SwapFree:")) {
        out.push(format!("Swap: {} MB, free {} MB", total / 1024, free / 1024));
    }
    out
}

fn driver_opt(dir: &Path) -> Option<String> {
    std::fs::read_link(dir.join("driver")).ok().and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
}

fn driver_of(dir: &Path) -> String {
    driver_opt(dir).unwrap_or_else(|| "none".into())
}

fn graphics(sys: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for n in names(&sys.join("class/drm")).iter().filter(|n| n.starts_with("card") && !n.contains('-')) {
        let base = sys.join("class/drm").join(n).join("device");
        let line = super::probe::graphics_line(&read(&base.join("vendor")), &read(&base.join("device")), &driver_opt(&base).unwrap_or_default());
        if !out.contains(&line) {
            out.push(line);
        }
    }
    out
}

fn screens(sys: &Path, heads: &[Head]) -> Vec<String> {
    let drm = sys.join("class/drm");
    names(&drm).iter().filter_map(|n| {
        let (_, connector) = n.strip_prefix("card")?.split_once('-')?;
        let dir = drm.join(n);
        (read(&dir.join("status")) == "connected").then(|| {
            let live = heads.iter().find(|h| h.name == connector).and_then(|h| Some((h, h.current_mode()?)));
            let mode = match live {
                Some((h, m)) => {
                    let hz = m.hz().map_or_else(|| "unknown rate".to_string(), |hz| format!("{hz:.2} Hz"));
                    let on = if h.enabled { "" } else { ", off" };
                    format!("{}x{} at {hz}, scale {}%{on}", m.width, m.height, (h.scale * 100.0).round())
                }
                None => read(&dir.join("modes")).lines().next().unwrap_or("unknown size").to_string(),
            };
            cut(&format!("{connector}: {mode}"))
        })
    }).collect()
}

fn sound(proc_: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(proc_.join("asound/cards")).unwrap_or_default();
    text.lines().filter(|l| l.trim_start().starts_with(|c: char| c.is_ascii_digit())).map(cut).take(12).collect()
}

fn pairs(line: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = line.trim();
    while let Some((key, after)) = rest.split_once("=\"") {
        let Some((value, next)) = after.split_once('"') else { break };
        out.push((key.trim().to_string(), value.to_string()));
        rest = next.trim_start();
    }
    out
}

fn gigabytes(bytes: &str) -> String {
    bytes.parse::<u64>().map_or_else(|_| "?".into(), |b| format!("{:.1} GB", b as f64 / 1e9))
}

fn disks(run: Run) -> Vec<String> {
    let text = run(&["lsblk", "-b", "-P", "-o", "NAME,TYPE,SIZE,FSTYPE,FSAVAIL,MOUNTPOINTS"]);
    text.lines().filter_map(|l| {
        let get = |k: &str| pairs(l).into_iter().find(|p| p.0 == k).map(|p| p.1).unwrap_or_default();
        let kind = get("TYPE");
        if kind.is_empty() || matches!(kind.as_str(), "loop" | "rom") {
            return None;
        }
        let mut line = format!("{} {kind} {}", get("NAME"), gigabytes(&get("SIZE")));
        let fs = get("FSTYPE");
        if !fs.is_empty() {
            line.push_str(&format!(" {fs}, free {}", gigabytes(&get("FSAVAIL"))));
        }

        let mounts = get("MOUNTPOINTS");
        let sayable: Vec<&str> = mounts.split("\\x0a").filter(|m| !m.is_empty() && !m.starts_with("/home") && !m.contains("/media")).collect();
        if let Some(mount) = sayable.iter().find(|m| **m == "/").or(sayable.first()) {
            line.push_str(&format!(" at {mount}"));
        }
        Some(cut(&line))
    }).take(DISK_LINES).collect()
}

fn number(dir: &Path, file: &str) -> Option<f64> {
    read(&dir.join(file)).parse().ok()
}

fn batteries(sys: &Path) -> Vec<String> {
    let root = sys.join("class/power_supply");
    names(&root).iter().filter_map(|n| {
        let dir = root.join(n);
        if read(&dir.join("type")) != "Battery" {
            return None;
        }
        let (full, design) = ["energy", "charge"].iter().find_map(|k| Some((number(&dir, &format!("{k}_full"))?, number(&dir, &format!("{k}_full_design"))?)))?;
        let unit = if dir.join("energy_full").exists() { "Wh" } else { "Ah" };
        let mut line = format!("{n}: holds {:.0}% of new ({:.1} of {:.1} {unit})", 100.0 * full / design.max(1.0), full / 1e6, design / 1e6);
        if let Some(c) = number(&dir, "cycle_count").filter(|c| *c > 0.0) {
            line.push_str(&format!(", {c:.0} cycles"));
        }
        let state = read(&dir.join("status"));
        if !state.is_empty() {
            line.push_str(&format!(", {state}"));
        }
        Some(cut(&line))
    }).collect()
}

fn adapters(sys: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for n in names(&sys.join("class/net")) {
        let dir = sys.join("class/net").join(&n);
        if dir.join("wireless").exists() {
            let ids = format!("{}:{}", read(&dir.join("device/vendor")).trim_start_matches("0x"), read(&dir.join("device/device")).trim_start_matches("0x"));
            out.push(format!("Wi-Fi: driver {} ({ids})", driver_of(&dir.join("device"))));
        }
    }
    for n in names(&sys.join("class/bluetooth")).iter().filter(|n| !n.contains(':')) {
        let dir = sys.join("class/bluetooth").join(n);
        let own = driver_of(&dir.join("device"));
        let driver = if own == "none" { driver_of(&dir.join("device/..")) } else { own };
        out.push(format!("Bluetooth: driver {driver}"));
    }
    out
}

fn devices(inventory: &no_driver::Inventory) -> Vec<String> {
    inventory.devices.iter().take(DEVICE_LINES)
        .map(|d| cut(&format!("{} {:?} {} driver {}", d.id, d.kind, d.name.as_deref().unwrap_or(""), d.driver.as_deref().unwrap_or("none")))).collect()
}

fn failed_units(run: Run) -> Vec<String> {
    let mut out = Vec::new();
    for (scope, argv) in [("system", ["systemctl", "--failed", "--no-legend", "--plain", "--no-pager"]), ("user", ["systemctl", "--user", "--failed", "--no-legend", "--plain"])] {
        for l in run(&argv).lines() {
            let f: Vec<&str> = l.split_whitespace().collect();
            if let (Some(unit), Some(active), Some(sub)) = (f.first(), f.get(2), f.get(3)) {
                out.push(cut(&format!("{scope}: {unit} {active}/{sub}")));
            }
        }
    }
    out.truncate(UNIT_LINES);
    out
}

fn errors(run: Run) -> Vec<String> {
    let n = ERROR_LINES.to_string();
    run(&["journalctl", "-b", "-p", "err", "--no-pager", "-q", "-o", "short-iso", "-r", "-n", &n]).lines().filter(|l| !l.trim().is_empty()).map(cut).take(ERROR_LINES).collect()
}

pub fn gather_in(sys: &Path, proc_: &Path, run: Run, inventory: &no_driver::Inventory, heads: &[Head]) -> String {
    let none = |v: Vec<String>| if v.is_empty() { vec!["None".to_string()] } else { v };
    let parts: Vec<(&str, Vec<String>)> = vec![
        (OPEN_PART, computer(sys)),
        ("Trindle", trindle(run)),
        ("System", system(sys, proc_, run)),
        ("Processor and memory", processor_and_memory(proc_)),
        ("Graphics", graphics(sys)),
        ("Screens", screens(sys, heads)),
        ("Sound", sound(proc_)),
        ("Disks", disks(run)),
        ("Battery", batteries(sys)),
        ("Wi-Fi and Bluetooth", adapters(sys)),
        ("Devices", devices(inventory)),
        ("Missing drivers and firmware", none(inventory.problems().iter().map(|l| cut(l)).collect())),
        ("Failed units", none(failed_units(run))),
        ("Errors this boot", none(errors(run))),
    ];
    let room = HARDWARE_MOST.saturating_sub(SPARE);
    let mut text = String::new();
    for (title, lines) in parts.into_iter().filter(|(_, l)| !l.is_empty()) {
        let mut part = format!("{}\n", heading(title));
        for l in lines {
            if text.chars().count() + part.chars().count() + l.chars().count() + 2 > room {
                break;
            }
            part.push_str(&l);
            part.push('\n');
        }
        text.push_str(&part);
        text.push('\n');
    }
    text.trim_end().to_string()
}

pub fn gather() -> String {

    let heads = crate::common::outputs::heads().unwrap_or_default();
    gather_in(Path::new("/sys"), Path::new("/proc"), &|argv| crate::common::run::text(argv), &no_driver::inventory(), &heads)
}

pub fn blanked(text: &str, blank: impl Fn(&str) -> String) -> String {
    let mut open = false;
    text.lines().map(|l| {
        if let Some(title) = l.strip_prefix("== ").and_then(|t| t.strip_suffix(" ==")) {
            open = title == OPEN_PART;
            l.to_string()
        } else if open {
            l.to_string()
        } else {
            blank(l)
        }
    }).collect::<Vec<_>>().join("\n")
}
