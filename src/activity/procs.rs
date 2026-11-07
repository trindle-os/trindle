//! What every program on the computer is doing, read straight from /proc and added up by program.

use std::collections::HashMap;
use std::sync::Arc;

const KERNEL_THREAD: u64 = 0x0020_0000;

pub const TRINDLE_APPS: &[&str] =
    &["trindle-settings", "trindle-apps", "trindle-store", "trindle-activity", "trindle-drives", "trindle-disks", "trindle-screenshot",
      "trindle-welcome", "trindle-live-welcome", "trindle-installer", "trindle-text", "trindle-help"];

const INTERPRETERS: &[&str] = &["python", "perl", "ruby", "node", "nodejs", "bash", "sh", "dash", "gjs", "lua", "php"];
pub use crate::common::files::read_text as read;

#[derive(Debug, PartialEq)]
pub struct Stat {
    pub comm: String,
    pub kernel: bool,
    pub ticks: u64,
    pub start: u64,
}

pub fn parse_stat(line: &str) -> Option<Stat> {
    let open = line.find('(')?;
    let close = line.rfind(')')?;
    let comm = line.get(open + 1..close)?.to_string();
    let rest: Vec<&str> = line.get(close + 1..)?.split_whitespace().collect();
    let field = |n: usize| rest.get(n - 3).and_then(|v| v.parse::<u64>().ok());
    let flags = field(9)?;
    Some(Stat { comm, kernel: flags & KERNEL_THREAD != 0, ticks: field(14)? + field(15)?, start: field(22)? })
}

pub fn field_kb(text: &str, key: &str) -> u64 {
    text.lines().find_map(|l| l.strip_prefix(key))
        .and_then(|v| v.split_whitespace().next()?.parse().ok()).unwrap_or(0)
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cpu {
    pub busy: u64,
    pub all: u64,
    pub threads: usize,
    pub each: Vec<(u64, u64)>,
}

pub fn parse_cpu(text: &str) -> Cpu {
    let figures = |parts: std::str::SplitWhitespace| {
        let n: Vec<u64> = parts.filter_map(|v| v.parse().ok()).collect();
        let all: u64 = n.iter().sum();
        let idle = n.get(3).copied().unwrap_or(0) + n.get(4).copied().unwrap_or(0);
        (all.saturating_sub(idle), all)
    };
    let mut cpu = Cpu::default();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("cpu") => (cpu.busy, cpu.all) = figures(parts),
            Some(name) if name.starts_with("cpu") => cpu.each.push(figures(parts)),
            _ => {}
        }
    }
    cpu.threads = cpu.each.len().max(1);
    cpu
}

pub fn each_busy(was: &Cpu, now: &Cpu) -> Vec<f64> {
    now.each.iter().enumerate().map(|(i, (busy, all))| {
        let (b0, a0) = was.each.get(i).copied().unwrap_or((*busy, *all));
        let all = all.saturating_sub(a0);
        if all == 0 { 0.0 } else { (busy.saturating_sub(b0) as f64 / all as f64).clamp(0.0, 1.0) }
    }).collect()
}

pub struct Clock {
    up: u64,
    hz: u64,
}

impl Clock {
    pub fn now() -> Clock {
        // SAFETY: sysconf takes a constant and only returns a number.
        let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) }.max(1) as u64;
        let up: f64 = read("/proc/uptime").split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        Clock { up: up as u64, hz }
    }

    pub fn running(&self, start: u64) -> u64 {
        self.up.saturating_sub(start / self.hz)
    }
}

pub fn flatpak_id(cgroup: &str) -> Option<String> {
    let last = cgroup.lines().last()?.rsplit('/').next()?;
    let id = last.strip_prefix("app-flatpak-")?.strip_suffix(".scope")?;
    let (id, n) = id.rsplit_once('-')?;
    (n.chars().all(|c| c.is_ascii_digit()) && !id.is_empty()).then(|| id.to_string())
}

fn base(path: &str) -> &str {
    let path = path.strip_suffix(" (deleted)").unwrap_or(path);
    path.rsplit('/').next().unwrap_or(path)
}

fn is_interpreter(name: &str) -> bool {
    INTERPRETERS.contains(&name.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.'))
}

fn script(args: &[&str]) -> Option<String> {
    let mut rest = args.iter().skip(1);
    while let Some(a) = rest.next() {
        if *a == "-m" {
            return rest.next().map(|m| m.to_string());
        }
        if a.starts_with('-') {
            continue;
        }
        let name = base(a);
        let name = [".py", ".pl", ".rb", ".mjs", ".js", ".sh", ".lua", ".php"].iter()
            .find_map(|x| name.strip_suffix(x)).unwrap_or(name);
        return (!name.is_empty()).then(|| name.to_string());
    }
    None
}

pub fn program_name(exe: Option<&str>, args: &[&str], comm: &str) -> String {
    let first = args.first().copied().unwrap_or("");
    let mut name = match exe {
        Some(e) if !e.is_empty() => base(e).to_string(),
        _ if !first.is_empty() && !first.contains([':', ' ']) => base(first).to_string(),
        _ => comm.to_string(),
    };
    if name == "trindle" {
        name = match (base(first), args.get(1)) {
            ("trindle", Some(part)) if !part.starts_with('-') => format!("trindle-{part}"),
            ("", _) => comm.to_string(),
            (called, _) => called.to_string(),
        };
    } else if is_interpreter(&name) {
        if let Some(s) = script(args) {
            name = s;
        }
    }
    name
}

pub fn exec_program(commandline: &str) -> Option<String> {
    let mut owned: Vec<String> = vec![String::new()];
    let mut quoted = false;
    for c in commandline.chars() {
        match c {
            '"' | '\'' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => owned.push(String::new()),
            c => {
                if let Some(w) = owned.last_mut() {
                    w.push(c);
                }
            }
        }
    }
    let words: Vec<&str> = owned.iter().map(String::as_str).filter(|w| !w.is_empty()).collect();
    let mut i = 0;
    while let Some(w) = words.get(i) {
        if base(w) == "env" || w.contains('=') || (i > 0 && w.starts_with('-') && base(words[0]) == "env") {
            i += 1;
        } else {
            break;
        }
    }
    let program = base(words.get(i)?);
    if program == "flatpak" || program.is_empty() {
        return None;
    }
    if is_interpreter(program) {
        if let Some(s) = script(&words[i..]) {
            return Some(s.to_lowercase());
        }
    }
    Some(program.to_lowercase())
}

pub struct AppNames {
    pub key: String,
    pub exec: Option<String>,
    pub try_exec: Option<String>,
    pub wm_class: String,
}

#[derive(Default)]
pub struct Names {
    by_name: HashMap<String, usize>,
    by_key: HashMap<String, usize>,
}

impl Names {
    pub fn new(apps: &[AppNames]) -> Names {
        let mut n = Names::default();
        let mut claim = |name: String, i: usize| {
            if !name.is_empty() {
                n.by_name.entry(name).or_insert(i);
            }
        };
        for (i, a) in apps.iter().enumerate() {
            claim(a.key.to_lowercase(), i);
        }
        for (i, a) in apps.iter().enumerate() {
            if let Some(e) = &a.exec {
                claim(e.clone(), i);
            }
            if let Some(t) = &a.try_exec {
                claim(base(t).to_lowercase(), i);
            }
        }
        for (i, a) in apps.iter().enumerate() {
            claim(a.key.to_lowercase().rsplit('.').next().unwrap_or("").to_string(), i);
            claim(a.wm_class.to_lowercase(), i);
        }
        for (i, a) in apps.iter().enumerate() {
            n.by_key.entry(a.key.to_lowercase()).or_insert(i);
        }
        n
    }

    pub fn app(&self, name: &str) -> Option<usize> {
        let name = name.to_lowercase();
        let bare = ["-bin", ".bin", "-wrapped", "-real"].iter().find_map(|x| name.strip_suffix(x));
        self.by_name.get(&name).or_else(|| bare.and_then(|b| self.by_name.get(b))).copied()
    }

    pub fn flatpak(&self, id: &str) -> Option<usize> {
        self.by_key.get(&id.to_lowercase()).copied()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub kernel: bool,
    pub mine: bool,
    pub flatpak: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Linux,
    Trindle,
    App,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Owner {
    pub key: String,
    pub kind: Kind,
    pub app: Option<usize>,
    pub name: String,
}

pub fn is_desktop(name: &str) -> bool {
    (name.starts_with("trindle") && !TRINDLE_APPS.contains(&name)) || name == "Xwayland"
}

pub fn owner(id: &Ident, names: &Names) -> Owner {
    let other = |kind, key: String, app| Owner { key, kind, app, name: id.name.clone() };
    if id.kernel {
        return other(Kind::Linux, "linux".into(), None);
    }
    if let Some(fp) = &id.flatpak {
        return match names.flatpak(fp) {
            Some(i) => other(Kind::App, format!("app:{i}"), Some(i)),
            None => Owner { key: format!("flatpak:{fp}"), kind: Kind::App, app: None,
                            name: fp.rsplit('.').next().unwrap_or(fp).to_string() },
        };
    }
    if id.mine && is_desktop(&id.name) {
        return other(Kind::Trindle, "trindle".into(), None);
    }
    match names.app(&id.name) {
        Some(i) => other(Kind::App, format!("app:{i}"), Some(i)),
        None => other(Kind::Other, format!("program:{}", id.name), None),
    }
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub pid: u32,
    pub start: u64,
    pub ticks: u64,
    pub kb: u64,
    pub ident: Arc<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub pid: u32,
    pub start: u64,
    pub mine: bool,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub owner: Owner,
    pub ticks: u64,
    pub kb: u64,
    pub members: Vec<Member>,
}

pub fn gather(samples: &[Sample], before: Option<&HashMap<(u32, u64), u64>>, names: &Names) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut at: HashMap<String, usize> = HashMap::new();
    for s in samples {
        let owner = owner(&s.ident, names);
        let i = *at.entry(owner.key.clone()).or_insert_with(|| {
            groups.push(Group { owner, ticks: 0, kb: 0, members: Vec::new() });
            groups.len() - 1
        });
        let g = &mut groups[i];
        g.ticks += match before {
            Some(b) => s.ticks.saturating_sub(b.get(&(s.pid, s.start)).copied().unwrap_or(0)),
            None => 0,
        };
        g.kb += s.kb;
        g.members.push(Member { pid: s.pid, start: s.start, mine: s.ident.mine });
    }
    groups
}

pub fn me() -> u32 {
    // SAFETY: getuid takes nothing, cannot fail and only returns a number.
    unsafe { libc::getuid() }
}

fn owner_of(pid: u32) -> Option<u32> {
    owner_in(&read(format!("/proc/{pid}/status")))
}

fn owner_in(status: &str) -> Option<u32> {
    status.lines().find_map(|l| l.strip_prefix("Uid:"))?.split_whitespace().next()?.parse().ok()
}

const FAIR_EVERY: u64 = 15;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Kept {
    plain: u64,
    fair: u64,
    at: u64,
}

fn fair_due(kept: Option<&Kept>, plain: u64, looks: u64) -> bool {
    match kept {
        None => true,
        Some(k) => plain.abs_diff(k.plain) > k.plain / 20 || looks.saturating_sub(k.at) >= FAIR_EVERY,
    }
}

pub struct Look {
    pub busy: u64,
    pub all: u64,
    pub threads: usize,
    pub each: Vec<f64>,
    pub groups: Vec<Group>,
}

#[derive(Default)]
pub struct Reader {
    known: HashMap<(u32, u64), (String, Arc<Ident>)>,
    before: Option<HashMap<(u32, u64), u64>>,
    memory: HashMap<(u32, u64), Kept>,
    cpu: Option<Cpu>,
    looks: u64,
}

fn memory_kb(pid: u32, mine: bool, kept: Option<&Kept>, looks: u64) -> Kept {
    let s = read(format!("/proc/{pid}/status"));
    let plain = field_kb(&s, "VmRSS:") + field_kb(&s, "VmSwap:");
    if !mine {
        return Kept { plain, fair: plain, at: looks };
    }
    match kept {
        Some(k) if !fair_due(Some(k), plain, looks) => *k,
        _ => {
            let r = read(format!("/proc/{pid}/smaps_rollup"));
            let at = if kept.is_none() { looks.saturating_sub(pid as u64 % FAIR_EVERY) } else { looks };
            let fair = if r.is_empty() { plain } else { field_kb(&r, "Pss:") + field_kb(&r, "SwapPss:") };
            Kept { plain, fair, at }
        }
    }
}

impl Reader {
    fn ident(&self, pid: u32, stat: &Stat, me: u32) -> Arc<Ident> {
        let raw = std::fs::read(format!("/proc/{pid}/cmdline")).unwrap_or_default();
        let raw = String::from_utf8_lossy(&raw);
        let args: Vec<&str> = raw.split('\0').filter(|a| !a.is_empty()).collect();
        let exe = std::fs::read_link(format!("/proc/{pid}/exe")).ok().map(|p| p.to_string_lossy().to_string());
        Arc::new(Ident {
            name: program_name(exe.as_deref(), &args, &stat.comm),
            kernel: stat.kernel,
            mine: owner_of(pid) == Some(me),
            flatpak: flatpak_id(&read(format!("/proc/{pid}/cgroup"))),
        })
    }

    pub fn look(&mut self, names: &Names) -> Look {
        let me = me();
        let cpu = parse_cpu(&read("/proc/stat"));
        let was = self.cpu.replace(cpu.clone()).unwrap_or_else(|| cpu.clone());
        self.looks += 1;
        let mut samples = Vec::new();
        let mut known = HashMap::new();
        let mut memory = HashMap::new();
        let dir = std::fs::read_dir("/proc");
        for e in dir.into_iter().flatten().flatten() {
            let Ok(pid) = e.file_name().to_string_lossy().parse::<u32>() else { continue };
            let Some(stat) = parse_stat(&read(format!("/proc/{pid}/stat"))) else { continue };
            let ident = match self.known.remove(&(pid, stat.start)) {
                Some((comm, id)) if comm == stat.comm => id,
                _ => self.ident(pid, &stat, me),
            };
            let key = (pid, stat.start);
            let kb = if stat.kernel {
                0
            } else {
                let kept = memory_kb(pid, ident.mine, self.memory.get(&key), self.looks);
                memory.insert(key, kept);
                kept.fair
            };
            samples.push(Sample { pid, start: stat.start, ticks: stat.ticks, kb, ident: ident.clone() });
            known.insert(key, (stat.comm, ident));
        }
        self.known = known;
        self.memory = memory;
        let groups = gather(&samples, self.before.as_ref(), names);
        self.before = Some(samples.iter().map(|s| ((s.pid, s.start), s.ticks)).collect());
        Look {
            busy: cpu.busy.saturating_sub(was.busy), all: cpu.all.saturating_sub(was.all), threads: cpu.threads,
            each: each_busy(&was, &cpu), groups,
        }
    }
}

pub fn same_live_process(stat: &str, status: &str, start: u64, me: u32) -> bool {
    let alive = stat.rfind(')').and_then(|i| stat.get(i + 1..)).and_then(|r| r.split_whitespace().next())
        .is_some_and(|state| !matches!(state, "Z" | "X"));
    alive && parse_stat(stat).is_some_and(|s| s.start == start) && owner_in(status) == Some(me)
}

fn signal_same(m: &Member, me: u32, signal: libc::c_int) -> bool {
    // SAFETY: pidfd_open takes two numbers and gives back a descriptor, closed below.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, m.pid as libc::pid_t, 0) };
    if fd < 0 {
        return false;
    }
    let fd = fd as libc::c_int;
    let sent = same_live_process(&read(format!("/proc/{}/stat", m.pid)), &read(format!("/proc/{}/status", m.pid)), m.start, me)
        // SAFETY: the descriptor is the process checked just now; no signal info is given.
        && unsafe { libc::syscall(libc::SYS_pidfd_send_signal, fd, signal, std::ptr::null::<libc::siginfo_t>(), 0) } == 0;
    // SAFETY: closing the descriptor opened above.
    unsafe { libc::close(fd) };
    sent
}

pub fn stop(members: &[Member]) -> usize {
    let me = me();
    members.iter().filter(|m| m.mine && signal_same(m, me, libc::SIGTERM)).count()
}

pub fn force(members: &[Member]) -> usize {
    let me = me();
    members.iter().filter(|m| m.mine && signal_same(m, me, libc::SIGKILL)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn whose_a_process_is_comes_from_its_status_not_its_folder() {
        assert_eq!(owner_in("Name:\ttrindle-ring\nUid:\t1000\t1000\t1000\t1000\nGid:\t1000\t1000\t1000\t1000\n"), Some(1000));
        assert_eq!(owner_in("Name:\tsystemd\nUid:\t0\t0\t0\t0\n"), Some(0));
        assert_eq!(owner_in(""), None);
        assert_eq!(owner_of(std::process::id()), Some(me()));
    }

    #[test]
    fn stat_lines_are_read_from_the_last_bracket() {
        let line = "4242 (Web Content (2)) S 4000 4000 4000 0 -1 4194560 100 0 0 0 250 50 0 0 20 0 30 0 987654 1000 200 18446744073709551615";
        let s = parse_stat(line).unwrap();
        assert_eq!(s.comm, "Web Content (2)");
        assert_eq!(s.ticks, 300);
        assert_eq!(s.start, 987654);
        assert!(!s.kernel);
        let k = "2 (kthreadd) S 0 0 0 0 -1 2129984 0 0 0 0 0 7 0 0 20 0 1 0 5 0 0";
        assert!(parse_stat(k).unwrap().kernel);
        assert_eq!(parse_stat("nonsense"), None);
    }

    #[test]
    fn status_and_rollup_figures() {
        let status = "Name:\tfirefox\nVmRSS:\t  204800 kB\nVmSwap:\t 1024 kB\n";
        assert_eq!(field_kb(status, "VmRSS:"), 204800);
        assert_eq!(field_kb(status, "VmSwap:"), 1024);
        let rollup = "55e0-7ff [rollup]\nRss: 9000 kB\nPss: 4500 kB\nSwapPss: 12 kB\n";
        assert_eq!(field_kb(rollup, "Pss:"), 4500);
        assert_eq!(field_kb(rollup, "SwapPss:"), 12);
        assert_eq!(field_kb(rollup, "Missing:"), 0);
    }

    #[test]
    fn the_fair_figure_is_read_again_only_when_needed() {
        let k = Kept { plain: 100_000, fair: 60_000, at: 10 };
        assert!(fair_due(None, 5, 1));
        assert!(!fair_due(Some(&k), 104_000, 11));
        assert!(fair_due(Some(&k), 106_000, 11));
        assert!(fair_due(Some(&k), 90_000, 11));
        assert!(fair_due(Some(&k), 100_000, 10 + FAIR_EVERY));
    }

    #[test]
    fn a_later_process_with_the_same_number_is_not_the_one_seen() {
        let stat = "4242 (sleep) S 4000 4000 4000 0 -1 4194560 100 0 0 0 250 50 0 0 20 0 30 0 987654 1000 200 18446744073709551615";
        let status = "Name:\tsleep\nUid:\t1000\t1000\t1000\t1000\n";
        assert!(same_live_process(stat, status, 987654, 1000));
        assert!(!same_live_process(stat, status, 987655, 1000), "another start time is another process");
        assert!(!same_live_process(stat, status, 987654, 1001), "someone else's");
        assert!(!same_live_process(&stat.replace(") S ", ") Z "), status, 987654, 1000), "already dead");
        assert!(!same_live_process("", "", 0, 0));
    }

    #[test]
    fn a_program_that_ignores_stop_is_ended_by_force_and_only_the_one_seen() {
        let mut child = std::process::Command::new("sh").args(["-c", "trap '' TERM; while :; do sleep 0.2; done"]).spawn().unwrap();
        let pid = child.id();
        std::thread::sleep(Duration::from_millis(300));
        let start = parse_stat(&read(format!("/proc/{pid}/stat"))).unwrap().start;
        let seen = Member { pid, start, mine: true };
        assert_eq!(stop(std::slice::from_ref(&seen)), 1);
        std::thread::sleep(Duration::from_millis(300));
        assert!(child.try_wait().unwrap().is_none(), "it ignored the request");
        assert_eq!(force(&[Member { pid, start: start + 1, mine: true }]), 0);
        assert_eq!(force(&[Member { pid, start, mine: false }]), 0);
        assert!(child.try_wait().unwrap().is_none());
        assert_eq!(force(&[seen]), 1);
        let status = child.wait().unwrap();
        assert_eq!(std::os::unix::process::ExitStatusExt::signal(&status), Some(libc::SIGKILL));
        assert_eq!(force(&[Member { pid, start, mine: true }]), 0);
    }

    #[test]
    fn stopping_asks_only_the_process_that_was_seen() {
        let mut child = std::process::Command::new("sleep").arg("30").spawn().unwrap();
        let pid = child.id();
        let start = parse_stat(&read(format!("/proc/{pid}/stat"))).unwrap().start;
        assert_eq!(stop(&[Member { pid, start: start + 1, mine: true }]), 0);
        assert_eq!(stop(&[Member { pid, start, mine: false }]), 0);
        assert_eq!(stop(&[Member { pid, start, mine: true }]), 1);
        let status = child.wait().unwrap();
        assert_eq!(std::os::unix::process::ExitStatusExt::signal(&status), Some(libc::SIGTERM));
    }

    #[test]
    fn the_whole_processor() {
        let text = "cpu  100 0 50 800 50 0 0 0 0 0\ncpu0 50 0 25 400 25 0 0 0 0 0\ncpu1 50 0 25 400 25 0 0 0 0 0\nintr 1\n";
        let was = parse_cpu(text);
        assert_eq!(was, Cpu { busy: 150, all: 1000, threads: 2, each: vec![(75, 500), (75, 500)] });
        let now = "cpu  200 0 50 900 50 0 0 0 0 0\ncpu0 150 0 25 400 25 0 0 0 0 0\ncpu1 50 0 25 500 25 0 0 0 0 0\n";
        assert_eq!(each_busy(&was, &parse_cpu(now)), vec![1.0, 0.0]);
    }

    #[test]
    fn how_long_a_process_has_run() {
        let clock = Clock { up: 5400, hz: 100 };
        assert_eq!(clock.running(180_000), 3600);
        assert_eq!(clock.running(900_000), 0);
        assert!(Clock::now().up > 0);
    }

    #[test]
    fn flatpak_apps_are_found_by_their_box() {
        let cg = "0::/user.slice/user-1000.slice/user@1000.service/app.slice/app-flatpak-org.signal.Signal-206368514.scope";
        assert_eq!(flatpak_id(cg).as_deref(), Some("org.signal.Signal"));
        assert_eq!(flatpak_id("0::/user.slice/user-1000.slice/session-2.scope"), None);
    }

    #[test]
    fn processes_go_by_the_program_they_run() {
        assert_eq!(program_name(Some("/usr/lib/firefox-esr/firefox-esr"), &["/usr/lib/firefox-esr/firefox-esr", "-contentproc"], "Web Content"), "firefox-esr");
        assert_eq!(program_name(Some("/usr/lib/firefox-esr/firefox-esr (deleted)"), &[], "Isolated Web Co"), "firefox-esr");
        assert_eq!(program_name(Some("/usr/lib/trindle/bin/trindle"), &["trindle-ring"], "trindle-ring"), "trindle-ring");
        assert_eq!(program_name(Some("/usr/bin/trindle"), &["/usr/bin/trindle-settings", "--panel", "about"], "trindle-setting"), "trindle-settings");
        assert_eq!(program_name(Some("/x/trindle"), &["trindle", "activity"], "trindle"), "trindle-activity");
        assert_eq!(program_name(Some("/usr/bin/python3.13"), &["/usr/bin/python3", "/usr/bin/unattended-upgrade"], "unattended-upgr"), "unattended-upgrade");
        assert_eq!(program_name(Some("/usr/bin/python3.13"), &["python3", "-u", "-m", "http.server"], "python3"), "http.server");
        assert_eq!(program_name(Some("/usr/bin/bash"), &["-bash"], "bash"), "bash");
        assert_eq!(program_name(None, &["/usr/sbin/NetworkManager", "--no-daemon"], "NetworkManager"), "NetworkManager");
        assert_eq!(program_name(None, &["sshd: john [priv]"], "sshd"), "sshd");
        assert_eq!(program_name(None, &[], "kworker/0:1"), "kworker/0:1");
    }

    #[test]
    fn exec_lines_name_their_program() {
        assert_eq!(exec_program("/usr/lib/firefox-esr/firefox-esr %u").as_deref(), Some("firefox-esr"));
        assert_eq!(exec_program("env GDK_BACKEND=x11 /opt/app/App --flag").as_deref(), Some("app"));
        assert_eq!(exec_program("/usr/bin/flatpak run --branch=stable org.signal.Signal"), None);
        assert_eq!(exec_program("python3 /usr/share/foo/foo.py").as_deref(), Some("foo"));
        assert_eq!(exec_program("\"/opt/My App/app\"").as_deref(), Some("app"));
    }

    fn names() -> Names {
        let app = |key: &str, exec: &str| AppNames { key: key.into(), exec: exec_program(exec), try_exec: None, wm_class: String::new() };
        Names::new(&[
            app("network-shortcut", "trindle-settings --panel network"),
            app("firefox-esr", "/usr/lib/firefox-esr/firefox-esr %u"),
            app("org.signal.Signal", "/usr/bin/flatpak run org.signal.Signal"),
            app("trindle-settings", "trindle-settings"),
            app("org.gnome.Nautilus", "nautilus --new-window"),
        ])
    }

    fn sample(pid: u32, name: &str, ticks: u64, kb: u64, mine: bool, flatpak: Option<&str>) -> Sample {
        Sample { pid, start: pid as u64 * 10, ticks, kb,
                 ident: Arc::new(Ident { name: name.into(), kernel: false, mine, flatpak: flatpak.map(String::from) }) }
    }

    #[test]
    fn processes_are_matched_to_apps() {
        let n = names();
        let id = |name: &str, mine| Ident { name: name.into(), kernel: false, mine, flatpak: None };
        assert_eq!(owner(&id("trindle-settings", true), &n).app, Some(3));
        assert_eq!(owner(&id("nautilus", true), &n).app, Some(4));
        assert_eq!(owner(&id("firefox-esr-bin", true), &n).app, Some(1));
        assert_eq!(owner(&id("trindle-ring", true), &n).kind, Kind::Trindle);
        assert_eq!(owner(&id("Xwayland", true), &n).kind, Kind::Trindle);
        assert_eq!(owner(&id("trindle-ring", false), &n).kind, Kind::Other);
        assert_eq!(owner(&id("rustc", true), &n).key, "program:rustc");
        let fp = Ident { name: "signal-desktop".into(), kernel: false, mine: true, flatpak: Some("org.signal.Signal".into()) };
        assert_eq!(owner(&fp, &n).app, Some(2));
        let unknown = Ident { flatpak: Some("com.example.Thing".into()), ..fp };
        let o = owner(&unknown, &n);
        assert_eq!((o.kind, o.name.as_str()), (Kind::App, "Thing"));
    }

    #[test]
    fn a_programs_parts_are_added_together() {
        let n = names();
        let samples = vec![
            sample(10, "firefox-esr", 500, 300_000, true, None),
            sample(11, "firefox-esr", 900, 200_000, true, None),
            sample(12, "firefox-esr", 100, 100_000, true, None),
            sample(20, "signal-desktop", 40, 150_000, true, Some("org.signal.Signal")),
            sample(21, "bwrap", 0, 1_000, true, Some("org.signal.Signal")),
            sample(30, "trindle-ring", 10, 20_000, true, None),
            sample(31, "trindle-compose", 30, 60_000, true, None),
            sample(40, "rustc", 1000, 500_000, true, None),
        ];
        let first = gather(&samples, None, &n);
        assert_eq!(first.len(), 4);
        assert!(first.iter().all(|g| g.ticks == 0));
        let fox = first.iter().find(|g| g.owner.app == Some(1)).unwrap();
        assert_eq!((fox.kb, fox.members.len()), (600_000, 3));
        let signal = first.iter().find(|g| g.owner.app == Some(2)).unwrap();
        assert_eq!(signal.kb, 151_000);
        let trindle = first.iter().find(|g| g.owner.kind == Kind::Trindle).unwrap();
        assert_eq!(trindle.kb, 80_000);

        let before: HashMap<(u32, u64), u64> = samples.iter().map(|s| ((s.pid, s.start), s.ticks)).collect();
        let mut later = samples.clone();
        later[0].ticks += 50;
        later[1].ticks += 70;
        later.push(sample(13, "firefox-esr", 25, 1_000, true, None));
        let groups = gather(&later, Some(&before), &n);
        let fox = groups.iter().find(|g| g.owner.app == Some(1)).unwrap();
        assert_eq!(fox.ticks, 145);
        assert_eq!(groups.iter().find(|g| g.owner.key == "program:rustc").unwrap().ticks, 0);
    }

    #[test]
    fn this_computer_can_be_read() {
        let mut r = Reader::default();
        let n = Names::default();
        let first = std::time::Instant::now();
        r.look(&n);
        let first = first.elapsed();
        let later = std::time::Instant::now();
        let look = r.look(&n);
        eprintln!("A first look took {first:?}, the next {:?}", later.elapsed());
        assert!(look.threads >= 1);
        let here = std::process::id();
        assert!(look.groups.iter().any(|g| g.members.iter().any(|m| m.pid == here && m.mine)));
    }
}
