//! What a report reads from the computer, and nothing else.

use serde_json::Value;

use super::{HARDWARE_MOST, TRACE_MOST};

pub const BEFORE: i64 = 300;

#[derive(Clone, Debug, PartialEq)]
pub struct Crash {
    pub program: String,
    pub signal: String,
    pub trace: String,
}

pub fn newest(json: &str) -> Option<(u64, String, i64)> {
    let all: Value = serde_json::from_str(json).ok()?;
    all.as_array()?.iter().filter_map(|e| {
        let exe = e["exe"].as_str()?;
        let program = exe.rsplit('/').next()?.to_string();
        if !program.starts_with("trindle") {
            return None;
        }
        Some((e["pid"].as_u64()?, program, e["time"].as_i64()? / 1_000_000))
    }).max_by_key(|e| e.2)
}

pub fn newest_trindle() -> Option<(String, i64)> {
    let list = crate::common::run::text(&["coredumpctl", "--no-pager", "--json=short", "list"]);
    newest(&list).map(|(_, program, at)| (program, at))
}

pub fn parse_info(text: &str) -> (String, String) {
    let signal = text.lines().find_map(|l| l.trim().strip_prefix("Signal:")).map(|s| s.trim().to_string()).unwrap_or_default();
    let trace = match text.find("Stack trace of") {
        Some(at) => steady(text[at..].trim_end()),
        None => String::new(),
    };
    (signal, trace.chars().take(TRACE_MOST).collect())
}

fn steady(trace: &str) -> String {
    let mut out = String::with_capacity(trace.len());
    for line in trace.lines() {
        let body = line.trim_start();
        let kept = if body.starts_with("Stack trace of") {
            body.split_whitespace().take_while(|w| w.trim_end_matches(':').parse::<u64>().is_err()).collect::<Vec<_>>().join(" ") + ":"
        } else if let Some((frame, rest)) = body.split_once(char::is_whitespace).filter(|(f, _)| f.starts_with('#')) {
            let rest = rest.trim_start();
            let rest = match rest.split_once(char::is_whitespace) {
                Some((address, after)) if address.starts_with("0x") => after.trim_start(),
                _ => rest,
            };
            format!("{frame}  {rest}")
        } else {
            line.to_string()
        };
        out.push_str(&kept);
        out.push('\n');
    }
    out.trim_end().to_string()
}

pub fn plugin_program(info: &str) -> Option<String> {
    let line = info.lines().find_map(|l| l.trim().strip_prefix("Command Line:"))?;
    let mut words = line.split_whitespace();
    words.find(|w| w.ends_with("trindle-plugin"))?;
    let id = (words.next()? == "run").then(|| words.next()).flatten()?;
    (id.len() <= 80 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.')).then(|| format!("trindle-plugin {id}"))
}

pub fn coredump() -> Option<(Crash, i64)> {
    let list = crate::common::run::text(&["coredumpctl", "--no-pager", "--json=short", "list"]);
    let (pid, program, at) = newest(&list)?;
    let info = crate::common::run::text(&["coredumpctl", "--no-pager", "info", &pid.to_string()]);
    let (signal, trace) = parse_info(&info);
    let program = if program == "trindle-plugin" { plugin_program(&info).unwrap_or(program) } else { program };
    Some((Crash { program, signal, trace }, at))
}

fn uptime() -> i64 {
    std::fs::read_to_string("/proc/uptime").ok().and_then(|t| t.split('.').next().and_then(|s| s.parse().ok())).unwrap_or(0)
}

pub fn before_this_boot(at: i64, up: i64) -> bool {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    up > 0 && at < now - up
}

pub fn log_before(at: Option<i64>) -> String {
    let n = super::LOG_LINES.to_string();
    let mut argv = vec!["journalctl".to_string(), "--no-pager".into(), "-q".into(), "-o".into(), "short-iso".into(), "-n".into(), n];
    match at {
        Some(t) => {
            if before_this_boot(t, uptime()) {
                argv.extend(["-b".into(), "-1".into()]);
            }
            argv.extend(["--since".into(), format!("@{}", t - BEFORE), "--until".into(), format!("@{}", t + 5)]);
        }
        None => argv.extend(["--since".into(), format!("-{BEFORE}s")]),
    }
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    crate::common::run::text(&argv)
}

fn graphics() -> Vec<String> {
    let mut out = Vec::new();
    let Ok(dir) = std::fs::read_dir("/sys/class/drm") else { return out };
    let mut names: Vec<String> = dir.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    for n in names {
        if !n.starts_with("card") || n.contains('-') {
            continue;
        }
        let base = format!("/sys/class/drm/{n}/device");
        let get = |f: &str| std::fs::read_to_string(format!("{base}/{f}")).unwrap_or_default().trim().to_string();
        let line = graphics_line(&get("vendor"), &get("device"), &std::fs::read_link(format!("{base}/driver")).ok().and_then(|p| p.file_name().map(|f| f.to_string_lossy().into_owned())).unwrap_or_default());
        if !out.contains(&line) {
            out.push(line);
        }
    }
    out
}

pub fn graphics_line(vendor: &str, device: &str, driver: &str) -> String {
    let lower = vendor.to_ascii_lowercase();
    let maker = match lower.as_str() {
        "0x8086" => "Intel",
        "0x1002" => "AMD",
        "0x10de" => "NVIDIA",
        "0x1af4" => "Virtio",
        "0x15ad" => "VMware",
        "0x1234" => "QEMU",
        "" => "unknown maker",
        other => other,
    };
    let mut line = format!("Graphics: {maker}");
    if !device.is_empty() {
        line.push_str(&format!(" {device}"));
    }
    if !driver.is_empty() {
        line.push_str(&format!(" (driver {driver})"));
    }
    line
}

fn processor_and_memory() -> Vec<String> {
    let mut out = Vec::new();
    if let Some(cpu) = std::fs::read_to_string("/proc/cpuinfo").ok().and_then(|t| t.lines().find_map(|l| l.strip_prefix("model name").and_then(|r| r.split_once(':')).map(|(_, v)| v.trim().to_string()))) {
        out.push(format!("Processor: {cpu}"));
    }
    if let Some(kb) = std::fs::read_to_string("/proc/meminfo").ok().and_then(|t| t.lines().find_map(|l| l.strip_prefix("MemTotal:").and_then(|r| r.trim().split(' ').next().and_then(|n| n.parse::<u64>().ok())))) {
        out.push(format!("Memory: {} MB", kb / 1024));
    }
    out
}

pub fn hardware_summary() -> String {
    let mut lines = graphics();
    lines.extend(processor_and_memory());
    lines.extend(crate::common::no_driver::report_lines(&crate::common::no_driver::scan_sys()));
    lines.join("\n").chars().take(HARDWARE_MOST).collect()
}

pub fn hardware_text(model: &str, summary: Option<&str>) -> String {
    let summary = summary.filter(|s| !s.trim().is_empty());
    let field = |key: &str| summary.and_then(|s| s.lines().find_map(|l| l.strip_prefix(key)).map(str::trim));
    let mut first = vec![model.to_string()];
    first.extend(field("Processor:").map(str::to_string));
    first.extend(field("Memory:").and_then(|m| m.trim_end_matches(" MB").parse::<u64>().ok()).map(|mb| format!("{} GB", (mb + 512) / 1024)));
    first.extend(system_name());
    let first = first.join(", ");
    match summary {
        Some(p) => format!("{first}\n{p}"),
        None => first,
    }
}

fn system_name() -> Option<String> {
    let text = std::fs::read_to_string("/etc/os-release").ok()?;
    text.lines().find_map(|l| l.strip_prefix("PRETTY_NAME=")).map(|v| v.trim_matches('"').to_string())
}
