//! An update replaces Trindle's program on disk, but one that runs all session long, such as the ring and its tab, carries on as the old one until it starts again.

use std::os::unix::fs::MetadataExt;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::t;

const LOOK_SECS: u64 = 30;
const SOON: i64 = 3 * LOOK_SECS as i64;
const REPLACED: &str = " (deleted)";

pub const INSTALLED: &str = "/usr/lib/trindle/bin/trindle";
const COMPOSE: &str = "/usr/lib/trindle/bin/trindle-compose";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    InPlace,
    HandsOver,
    Package,
    Brief,
    Stays,
}

pub struct Part {
    pub name: &'static str,
    pub rule: Rule,
    #[cfg(test)]
    pub why: &'static str,
}

impl Part {
    pub fn said(&self) -> &'static str {
        match self.name {
            "trindle-ring" => t!("common-renewed-ring"),
            "trindle-portal" => t!("common-renewed-portal"),
            "trindle-wallpaper" => t!("common-renewed-wallpaper"),
            "trindle-clipboard" => t!("common-renewed-clipboard"),
            "trindle-keyring" => t!("common-renewed-keyring"),
            "trindle-idle" => t!("common-renewed-idle"),
            "trindle-nightlight" => t!("common-renewed-nightlight"),
            "trindle-wsdd" => t!("common-renewed-wsdd"),
            "trindle-plugin" => t!("common-renewed-plugin"),
            "trindle-compose" => t!("common-renewed-compose"),
            "trindle-mcp" => t!("common-renewed-mcp"),
            _ => "",
        }
    }
}

const fn part(name: &'static str, rule: Rule, why: &'static str) -> Part {
    let _ = why;
    Part {
        name,
        rule,
        #[cfg(test)]
        why,
    }
}

const fn renews(name: &'static str, rule: Rule, why: &'static str) -> Part {
    let _ = why;
    Part {
        name,
        rule,
        #[cfg(test)]
        why,
    }
}

const WINDOW: &str = "A window the person opens and closes: the next one opened is the new program.";
const JOB: &str = "It does one job and ends: the next one started is the new program.";

pub const PARTS: &[Part] = &[
    renews("trindle-ring", Rule::InPlace, "Closed, with nothing open on it."),
    renews("trindle-portal", Rule::InPlace, "No window of its own up, no request half answered, nothing kept awake through it and no screen share running."),
    renews("trindle-wallpaper", Rule::HandsOver, "Any time: the old one leaves once the new one has drawn every screen."),
    renews("trindle-clipboard", Rule::InPlace, "The watcher, when no password manager's copy waits to be wiped and no copy is being written down. `copy` holds the one thing put back on the clipboard and ends at the next copy."),
    renews("trindle-keyring", Rule::HandsOver, "No password window up, no question waiting and no unlock under way."),
    renews("trindle-idle", Rule::InPlace, "Nothing is dimmed, dark, showing the clock or locked by it."),
    renews("trindle-nightlight", Rule::InPlace, "The screens are their daytime colour."),
    renews("trindle-wsdd", Rule::InPlace, "Nobody is being answered."),
    part("trindle-thunder", Rule::Package, "Restarted by the package as it is installed."),
    part("trindle-controller", Rule::Brief, "Started by its socket when the compositor connects and ended after 30 quiet seconds: the next one started is the new program, and none is cut off mid-game."),
    part("trindle-usb-retry", Rule::Package, "Restarted by the package as it is installed."),
    part("trindle-dns", Rule::Package, "Restarted by the package as it is installed."),
    part("trindle-compose", Rule::Stays, "It holds every window and the screen: starting it again ends the session."),
    part("trindle-session", Rule::Stays, "It is the session: it waits for the compositor and ends with it."),
    part("trindle-autostart", Rule::Stays, "It does its work once, at login. Where it stays on, under another compositor, it only starts the ring again by name, which is the new program."),
    part("trindle-lock", Rule::Stays, "It is the locked screen: one that ended would leave the screen locked with nobody to unlock it. The next lock is the new program."),
    part("trindle-mcp", Rule::Stays, "An assistant started it and talks to it down a pipe, mid-conversation. The assistant's next start is the new program."),
    renews("trindle-plugin", Rule::InPlace, "The keeper of a plugin's background part, only at the moment the plugin's part has stopped by itself and nothing of it is running: never while the part is busy. It becomes the new program in its own place, holding the plugin's lock all the way, and starts the part again. Every other use of `trindle-plugin` is a single job that ends."),
    part("trindle-boxed", Rule::Stays, "It is the outside of a running app's box and ends with the app."),
    part("trindle-greeter", Rule::Brief, "The login screen: each one shown is the new program."),
    part("trindle-greeter-session", Rule::Brief, "The login screen's session: each one is the new program."),
    part("trindle-starting", Rule::Brief, "Gone within a minute and a half of login."),
    part("trindle-screensaver", Rule::Brief, "Gone at the first movement."),
    part("trindle-settings", Rule::Brief, WINDOW),
    part("trindle-apps", Rule::Brief, WINDOW),
    part("trindle-store", Rule::Brief, WINDOW),
    part("trindle-text", Rule::Brief, WINDOW),
    part("trindle-drives", Rule::Brief, WINDOW),
    part("trindle-disks", Rule::Brief, WINDOW),
    part("trindle-activity", Rule::Brief, WINDOW),
    part("trindle-welcome", Rule::Brief, WINDOW),
    part("trindle-live-welcome", Rule::Brief, WINDOW),
    part("trindle-installer", Rule::Brief, WINDOW),
    part("trindle-screenshot", Rule::Brief, WINDOW),
    part("trindle-eye-dropper", Rule::Brief, WINDOW),
    part("trindle-report", Rule::Brief, WINDOW),
    part("trindle-voice-setup", Rule::Brief, WINDOW),
    part("trindle-terminal", Rule::Brief, "It starts the person's terminal and is gone."),
    part("trindle-icons", Rule::Brief, JOB),
    part("trindle-scale", Rule::Brief, JOB),
    part("trindle-brightness", Rule::Brief, JOB),
    part("trindle-admin", Rule::Brief, JOB),
    part("trindle-windows-grub", Rule::Brief, JOB),
];

pub fn part_named(name: &str) -> Option<&'static Part> {
    PARTS.iter().find(|p| p.name == name)
}

fn replaced() -> Option<PathBuf> {
    let link = std::fs::read_link("/proc/self/exe").ok()?;
    let text = link.to_str()?;
    text.strip_suffix(REPLACED).map(PathBuf::from)
}

fn may_start(program: &Path, owner: u32, mode: u32, ordinary: bool) -> bool {
    program == Path::new(INSTALLED) && ordinary && owner == 0 && mode & 0o022 == 0
}

fn trusted(program: &Path) -> bool {
    of_file(program).is_some_and(|(owner, mode, ordinary)| may_start(program, owner, mode, ordinary))
}

fn of_file(program: &Path) -> Option<(u32, u32, bool)> {
    std::fs::symlink_metadata(program).ok().map(|m| (m.uid(), m.mode(), m.file_type().is_file()))
}

fn installing() -> bool {
    let Ok(dir) = std::fs::read_dir("/proc") else { return false };
    dir.flatten().any(|p| {
        let comm = std::fs::read_to_string(p.path().join("comm")).unwrap_or_default();
        let cmdline = std::fs::read(p.path().join("cmdline")).unwrap_or_default();
        is_installer(comm.trim(), &String::from_utf8_lossy(&cmdline))
    })
}

fn is_installer(comm: &str, cmdline: &str) -> bool {
    match comm {
        "dpkg" | "apt" | "apt-get" => true,
        "unattended-upgr" => !cmdline.contains("unattended-upgrade-shutdown"),
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Renew {
    InPlace,
    InPlaceAs(&'static [&'static str]),
    Leave,
}

fn due(replaced: bool, installing: bool, resting: bool, there: bool) -> bool {
    replaced && !installing && resting && there
}

pub fn new_program() -> Option<PathBuf> {
    let program = replaced()?;
    due(true, installing(), true, trusted(&program)).then_some(program)
}

fn rule_of(name: &str, wanted: Rule) {
    debug_assert!(part_named(name).is_some_and(|p| p.rule == wanted), "{name} is not in the table of parts as {wanted:?}");
}

pub fn due_program(name: &'static str) -> Option<PathBuf> {
    rule_of(name, Rule::InPlace);
    new_program()
}

pub fn become_new(program: &Path, name: &'static str, args: &[String]) {
    rule_of(name, Rule::InPlace);
    let e = Command::new(program).arg0(name).args(args).exec();
    eprintln!("{name}: could not start the updated program: {e}");
}

fn renew(program: &Path, name: &str, how: Renew) {
    let mut cmd = Command::new(program);
    cmd.arg0(name);
    match how {
        Renew::Leave => {
            eprintln!("{name}: updated; leaving, to be started afresh as the new program");
            std::process::exit(0);
        }
        Renew::InPlace => cmd.args(std::env::args().skip(1)).env(IN_PLACE, "1"),
        Renew::InPlaceAs(args) => cmd.args(args).env(IN_PLACE, "1"),
    };
    let e = cmd.exec();
    eprintln!("{name}: could not start the updated program: {e}");
}

pub fn named(part: &str) {
    let Ok(name) = std::ffi::CString::new(&part.as_bytes()[..part.len().min(15)]) else { return };
    // SAFETY: PR_SET_NAME copies at most 16 bytes from the string it is
    // handed, which is ended by a zero and lives until the call returns.
    unsafe { libc::prctl(libc::PR_SET_NAME, name.as_ptr() as libc::c_ulong, 0, 0, 0) };
}

pub fn watch(events: &Loop, name: &'static str, how: impl Fn() -> Option<Renew> + 'static) {
    rule_of(name, Rule::InPlace);
    watch_with(events, move |program| {
        let Some(how) = how() else { return Flow::Continue };
        renew(program, name, how);
        Flow::Stop
    });
}

pub fn watch_apart(name: &'static str, resting: impl Fn() -> bool + Send + 'static) {
    rule_of(name, Rule::InPlace);
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(LOOK_SECS));
        let Some(program) = new_program() else { continue };
        if resting() {
            renew(&program, name, Renew::InPlace);
            return;
        }
    });
}

pub fn watch_with(events: &Loop, mut act: impl FnMut(&Path) -> Flow + 'static) {
    adopt_children();
    events.every(Duration::from_secs(LOOK_SECS), move || match new_program() {
        Some(program) => act(&program),
        None => Flow::Continue,
    });
}

fn a_unit_of_its_own(named: Option<&str>, pid: u32) -> bool {
    named.and_then(|p| p.parse::<u32>().ok()) == Some(pid)
}

pub const RENEWED: &str = "--renewed";

pub fn beside(program: &Path, name: &'static str) -> Option<(Child, UnixStream)> {
    rule_of(name, Rule::HandsOver);
    if a_unit_of_its_own(std::env::var("SYSTEMD_EXEC_PID").ok().as_deref(), std::process::id()) {
        return None;
    }
    let (mine, theirs) = UnixStream::pair().ok()?;
    let child = Command::new(program).arg0(name).arg(RENEWED)
        .stdin(Stdio::from(std::os::fd::OwnedFd::from(theirs))).spawn().ok()?;
    Some((child, mine))
}

pub fn from_old() -> Option<UnixStream> {
    use std::os::fd::AsFd;
    let stream = UnixStream::from(std::io::stdin().as_fd().try_clone_to_owned().ok()?);
    stream.peer_addr().is_ok().then_some(stream)
}

pub fn waiting() -> (Vec<&'static str>, Vec<&'static str>) {
    // SAFETY: getuid cannot fail.
    let me = unsafe { libc::getuid() };
    let (mut said, mut lagging) = (Vec::new(), Vec::new());
    let Ok(put) = std::fs::metadata(INSTALLED).map(|m| m.ctime()) else { return (said, lagging) };
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
    let settled = now - put >= SOON;
    let Ok(dir) = std::fs::read_dir("/proc") else { return (said, lagging) };
    for p in dir.flatten() {
        if p.metadata().map(|m| m.uid()).ok() != Some(me) {
            continue;
        }
        let Ok(file) = std::fs::read_link(p.path().join("exe")) else { continue };
        let file = file.to_string_lossy();
        let cmdline = std::fs::read(p.path().join("cmdline")).unwrap_or_default();
        let words: Vec<String> = cmdline.split(|b| *b == 0).map(|w| String::from_utf8_lossy(w).into_owned()).collect();
        if let Some(word) = lags(&words, &file).filter(|_| settled) {
            if !lagging.contains(&word) {
                lagging.push(word);
            }
        }
        let Some(word) = waits(&file, &words) else { continue };
        if !said.contains(&word) {
            said.push(word);
        }
    }
    for list in [&mut said, &mut lagging] {
        list.sort_by_key(|w| PARTS.iter().position(|p| p.said() == *w));
    }
    (said, lagging)
}

fn is_keeper(words: &[String]) -> bool {
    words.get(1).is_some_and(|w| w == "run") && words.get(3).is_some_and(|w| w == "background")
}

fn waits(file: &str, words: &[String]) -> Option<&'static str> {
    let old = file.strip_suffix(REPLACED)?;
    let name = words.first()?.rsplit('/').next()?;
    let part = part_named(name).filter(|p| !p.said().is_empty())?;
    if old != if name == "trindle-compose" { COMPOSE } else { INSTALLED } {
        return None;
    }
    let keeper = is_keeper(words);
    (part.rule == Rule::Stays && (name != "trindle-plugin" || keeper)).then_some(part.said())
}

fn lags(words: &[String], exe: &str) -> Option<&'static str> {
    let name = words.first()?.rsplit('/').next()?;
    let part = part_named(name).filter(|p| !p.said().is_empty() && matches!(p.rule, Rule::InPlace | Rule::HandsOver))?;
    let held = name == "trindle-clipboard" && words.get(1).is_some_and(|w| w == "copy");
    let job = name == "trindle-plugin" && !is_keeper(words);
    (exe.strip_suffix(REPLACED) == Some(INSTALLED) && !held && !job).then_some(part.said())
}

pub fn sentence(waiting: &[&str], lagging: &[&str]) -> Option<String> {
    let all: Vec<&str> = waiting.iter().chain(lagging).copied().collect();
    let list = match all.as_slice() {
        [] => return None,
        [one] => one.to_string(),
        [first, middle @ .., last] => {
            let most = middle.iter().fold(first.to_string(), |sofar, next| t!("common-renewed-join", first = &sofar, next = *next));
            t!("common-renewed-join-last", most = &most, last = *last)
        }
    };
    Some(if lagging.is_empty() { t!("common-renewed-sentence", list = &list) } else { t!("common-renewed-sentence-latest", list = &list) })
}

pub fn said() -> Option<String> {
    use std::sync::Mutex;
    use std::time::Instant;
    static LAST: Mutex<Option<(Instant, Option<String>)>> = Mutex::new(None);
    if cfg!(test) {
        return None;
    }
    let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, said)) = last.as_ref().filter(|(at, _)| at.elapsed() < Duration::from_secs(LOOK_SECS)) {
        return said.clone();
    }
    let (stays, lagging) = waiting();
    let said = sentence(&stays, &lagging);
    *last = Some((Instant::now(), said.clone()));
    said
}

pub fn not_traceable() -> bool {
    // SAFETY: prctl with PR_SET_DUMPABLE only sets a flag on this process.
    unsafe { libc::prctl(libc::PR_SET_DUMPABLE, 0, 0, 0, 0) == 0 }
}

pub const CLOSED: &[&str] = &["trindle-ring", "trindle-screenshot", "trindle-keyring", "trindle-lock"];

pub fn close(part: &str) -> bool {
    !CLOSED.contains(&part) || not_traceable()
}

fn adopt_children() {
    if std::env::var_os(IN_PLACE).is_some() {
        std::env::remove_var(IN_PLACE);
        adopt(|_| true);
    }
}

const IN_PLACE: &str = "TRINDLE_RENEWED_IN_PLACE";

fn adopt(which: impl Fn(libc::pid_t) -> bool) {
    let Ok(tasks) = std::fs::read_dir("/proc/self/task") else { return };
    for task in tasks.flatten() {
        let Ok(text) = std::fs::read_to_string(task.path().join("children")) else { continue };
        for pid in text.split_whitespace().filter_map(|p| p.parse().ok()).filter(|p| which(*p)) {
            let pid: libc::pid_t = pid;
            // SAFETY: waitpid with a null status pointer only reaps the child.
            std::thread::spawn(move || unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_shutdown_watcher_is_not_an_install() {
        let watcher = "/usr/bin/python3\0/usr/share/unattended-upgrades/unattended-upgrade-shutdown\0--wait-for-signal\0";
        assert!(!super::is_installer("unattended-upgr", watcher));
        assert!(super::is_installer("unattended-upgr", "/usr/bin/python3\0/usr/bin/unattended-upgrade\0"));
        assert!(super::is_installer("dpkg", ""));
        assert!(!super::is_installer("bash", ""));
    }

    #[test]
    fn every_part_renews_by_the_rule_or_says_why_it_stays() {
        use super::{part_named, Rule, PARTS};
        for name in crate::NAMES {
            assert!(part_named(name).is_some(), "{name} is started by name and is not in the table of parts: say how it renews or why it stays");
        }
        for (i, p) in PARTS.iter().enumerate() {
            assert!(!PARTS[..i].iter().any(|q| q.name == p.name), "{} is in the table twice", p.name);
            assert!(p.why.trim().len() > 20, "{} has neither a rule with its safe time nor a written reason", p.name);
        }
        let code: &[(&str, &str, &str)] = &[
            ("trindle-ring", crate::common::source::code(include_str!("../ring/mod.rs")), "renewed::watch(lp, \"trindle-ring\""),
            ("trindle-portal", crate::common::source::code(include_str!("../screenshot/portal.rs")), "renewed::watch(&events, \"trindle-portal\""),
            ("trindle-wallpaper", crate::common::source::code(include_str!("../session/wallpaper.rs")), "renewed::beside(program, \"trindle-wallpaper\""),
            ("trindle-clipboard", crate::common::source::code(include_str!("clipboard.rs")), "renewed::watch_apart(\"trindle-clipboard\""),
            ("trindle-keyring", crate::common::source::code(include_str!("../keyring/renew.rs")), "renewed::beside(program, \"trindle-keyring\""),
            ("trindle-idle", crate::common::source::code(include_str!("../session/idle.rs")), "renewed::watch(&events, \"trindle-idle\""),
            ("trindle-nightlight", crate::common::source::code(include_str!("../session/nightlight.rs")), "renewed::watch(&events, \"trindle-nightlight\""),
            ("trindle-wsdd", crate::common::source::code(include_str!("../session/wsdd.rs")), "renewed::watch_apart(\"trindle-wsdd\""),
            ("trindle-plugin", crate::common::source::code(include_str!("../plugins/background.rs")), "renewed::due_program(\"trindle-plugin\")"),
        ];
        for p in PARTS.iter().filter(|p| matches!(p.rule, Rule::InPlace | Rule::HandsOver)) {
            let Some((_, text, call)) = code.iter().find(|(name, _, _)| *name == p.name) else {
                panic!("{} is said to renew and no code of its own is named here", p.name);
            };
            assert!(text.contains(call), "{} is said to renew and its code never asks to: {call}", p.name);
        }
        let postinst = include_str!("../../tools/deb/postinst");
        for p in PARTS.iter().filter(|p| p.rule == Rule::Package) {
            assert!(postinst.contains(&format!("restart {}.service", p.name)), "the package never restarts {}", p.name);
        }
    }

    #[test]
    fn only_the_installed_program_that_is_roots_alone_is_ever_started() {
        use super::{may_start, INSTALLED};
        use std::path::Path;
        assert!(may_start(Path::new(INSTALLED), 0, 0o100755, true));
        assert!(!may_start(Path::new("/home/john/trindle/target/release/trindle"), 0, 0o100755, true));
        assert!(!may_start(Path::new("/usr/bin/trindle-portal"), 0, 0o100755, true));
        assert!(!may_start(Path::new(INSTALLED), 1000, 0o100755, true));
        assert!(!may_start(Path::new(INSTALLED), 0, 0o100775, true));
        assert!(!may_start(Path::new(INSTALLED), 0, 0o100757, true));
        assert!(!may_start(Path::new(INSTALLED), 0, 0o100755, false));
        assert!(!super::trusted(&std::env::current_exe().unwrap()));
    }

    #[test]
    fn whose_a_file_is_is_read_off_the_file_itself() {
        use super::{may_start, of_file, INSTALLED};
        use std::os::unix::fs::PermissionsExt;
        use std::path::Path;
        // SAFETY: getuid cannot fail.
        let me = unsafe { libc::getuid() };
        let folder = std::env::temp_dir().join(format!("trindle-renewed-whose-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let mine = folder.join("trindle");
        std::fs::write(&mine, b"#!/bin/sh\n").unwrap();
        std::fs::set_permissions(&mine, std::fs::Permissions::from_mode(0o755)).unwrap();
        let started = |file: &Path| of_file(file).is_some_and(|(owner, mode, ordinary)| may_start(Path::new(INSTALLED), owner, mode, ordinary));
        assert_eq!(of_file(&mine).map(|(owner, mode, ordinary)| (owner, mode & 0o777, ordinary)), Some((me, 0o755, true)));
        let roots = Path::new("/usr/bin/env");
        assert_eq!(of_file(roots).map(|(owner, mode, ordinary)| (owner, mode & 0o022, ordinary)), Some((0, 0, true)), "{roots:?} is not root's alone here");
        assert!(started(roots));
        assert_eq!(started(&mine), me == 0);
        let link = folder.join("link");
        std::os::unix::fs::symlink(roots, &link).unwrap();
        assert_eq!(of_file(&link).map(|(_, _, ordinary)| ordinary), Some(false));
        assert!(!started(&link) && !started(&folder) && !started(&folder.join("none")));
        let _ = std::fs::remove_dir_all(&folder);
    }

    #[test]
    fn every_part_that_is_closed_is_marked_as_the_program_starts() {
        const INNER: &str = "TRINDLE_TEST_CLOSED";
        if let Ok(part) = std::env::var(INNER) {
            assert!(super::close(&part));
            // SAFETY: prctl with PR_GET_DUMPABLE only reads a flag.
            println!("dumpable {}", unsafe { libc::prctl(libc::PR_GET_DUMPABLE, 0, 0, 0, 0) });
            return;
        }
        let started_as = |part: &str| {
            let out = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "common::renewed::tests::every_part_that_is_closed_is_marked_as_the_program_starts", "--nocapture", "--test-threads=1"])
                .env(INNER, part).output().unwrap();
            let said = String::from_utf8_lossy(&out.stdout).into_owned();
            assert!(out.status.success(), "{part}: {said}");
            said.contains("dumpable 0")
        };
        for part in ["trindle-keyring", "trindle-ring", "trindle-screenshot", "trindle-lock"] {
            assert!(super::CLOSED.contains(&part), "{part} is not on the list of closed parts");
            assert!(started_as(part), "{part} can be looked inside by the person's other programs");
        }
        for &part in super::CLOSED {
            assert!(super::part_named(part).is_some(), "{part} is not a part of Trindle");
            assert!(started_as(part), "{part} is on the list and is not marked");
        }
        for part in ["trindle-settings", "bash", ""] {
            assert!(!started_as(part), "{part} was marked");
        }
        let main = crate::common::source::code(include_str!("../main.rs"));
        let (call, first) = (main.find("common::renewed::close(&part)").expect("the program never marks its closed parts"), main.find("run_part(&part, args)").unwrap());
        assert!(call < first, "the parts are started before they are marked");
    }

    #[test]
    fn a_part_is_listed_under_its_own_name_however_it_was_started() {
        const INNER: &str = "TRINDLE_TEST_NAMED";
        if std::env::var_os(INNER).is_some() {
            super::named("trindle-nightlight");
            print!("{}", std::fs::read_to_string("/proc/thread-self/comm").unwrap());
            return;
        }
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "common::renewed::tests::a_part_is_listed_under_its_own_name_however_it_was_started", "--nocapture", "--test-threads=1"])
            .env(INNER, "1").output().unwrap();
        assert!(String::from_utf8_lossy(&out.stdout).contains("trindle-nightli\n"), "{}", String::from_utf8_lossy(&out.stdout));
    }

    #[test]
    fn a_part_systemd_runs_as_its_own_unit_does_not_hand_over() {
        use super::a_unit_of_its_own;
        assert!(a_unit_of_its_own(Some("4321"), 4321));
        assert!(!a_unit_of_its_own(Some("900"), 4321));
        assert!(!a_unit_of_its_own(None, 4321));
        assert!(!a_unit_of_its_own(Some(""), 4321));
    }

    #[test]
    fn what_waits_for_a_log_out_is_said_in_one_sentence_or_not_at_all() {
        use super::{lags, sentence, waits};
        assert_eq!(sentence(&[], &[]), None);
        assert_eq!(sentence(&["the desktop itself"], &[]).as_deref(),
            Some("This update is already in use, except the desktop itself, which will be the new version after the next log out."));
        assert_eq!(sentence(&["the desktop itself", "plugins running in the background"], &[]).as_deref(),
            Some("This update is already in use, except the desktop itself and plugins running in the background, which will be the new version after the next log out."));
        let all = sentence(&["a", "b", "c"], &[]).unwrap();
        assert!(all.contains("except a, b and c,"));
        assert!(!all.contains("reboot") && !all.contains("restart"));
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let (trindle, compose) = ("/usr/lib/trindle/bin/trindle (deleted)", "/usr/lib/trindle/bin/trindle-compose (deleted)");
        assert_eq!(waits(compose, &words(&["/usr/lib/trindle/bin/trindle-compose", "--start"])), Some("the desktop itself"));
        assert_eq!(waits(trindle, &words(&["trindle-plugin", "run", "org.example.flow", "background"])), None);
        assert_eq!(lags(&words(&["trindle-plugin", "run", "org.example.flow", "background"]), trindle), Some("plugins running in the background"));
        assert_eq!(waits(trindle, &words(&["trindle-mcp"])), Some("the link an AI assistant is using"));
        assert_eq!(waits(trindle, &words(&["trindle-plugin", "run", "org.example.flow", "shortcut"])), None);
        assert_eq!(waits(trindle, &words(&["trindle-portal"])), None);
        assert_eq!(waits(trindle, &words(&["trindle-session"])), None);
        assert_eq!(waits("/usr/bin/bash", &words(&["bash"])), None);
        assert_eq!(waits("/usr/lib/trindle/bin/trindle-compose", &words(&["trindle-compose", "--start"])), None);
        assert_eq!(waits("/usr/lib/trindle/bin/trindle", &words(&["trindle-mcp"])), None);
    }

    #[test]
    fn a_part_that_is_still_the_old_program_is_named_though_it_renews_by_itself() {
        use super::{lags, part_named, sentence, Rule, INSTALLED, PARTS, SOON};
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let old = format!("{INSTALLED} (deleted)");
        for p in PARTS.iter().filter(|p| matches!(p.rule, Rule::InPlace | Rule::HandsOver)) {
            assert!(!p.said().is_empty(), "{} renews by itself and has no name to be said by while it has not", p.name);
            let started: &[&str] = if p.name == "trindle-plugin" { &["trindle-plugin", "run", "org.example.One", "background"] } else { &[p.name] };
            assert_eq!(lags(&words(started), &old), Some(p.said()), "{}", p.name);
            assert_eq!(lags(&words(started), INSTALLED), None, "{}", p.name);
        }
        assert_eq!(lags(&words(&["/usr/lib/trindle/bin/trindle-keyring"]), &old), part_named("trindle-keyring").map(|p| p.said()));
        assert_eq!(lags(&words(&["trindle-keyring", "--renewed"]), &old), part_named("trindle-keyring").map(|p| p.said()));
        assert_eq!(lags(&words(&["trindle-keyring"]), "/home/john/trindle/target/release/trindle (deleted)"), None);
        assert_eq!(lags(&words(&["trindle-compose"]), "/usr/lib/trindle/bin/trindle-compose (deleted)"), None);
        assert_eq!(lags(&words(&["trindle-settings"]), &old), None);
        assert_eq!(lags(&words(&["trindle-clipboard", "copy"]), &old), None);
        assert_eq!(lags(&words(&["trindle-plugin", "run", "org.example.One", "ring"]), &old), None);
        assert_eq!(lags(&words(&["trindle-plugin", "list"]), &old), None);
        assert_eq!(lags(&words(&["trindle-plugin", "run", "org.example.One", "background", "--keeping=9"]), &old), part_named("trindle-plugin").map(|p| p.said()));
        assert_eq!(lags(&words(&["trindle-clipboard", "watch"]), &old), part_named("trindle-clipboard").map(|p| p.said()));
        assert_eq!(lags(&words(&["bash"]), &old), None);
        assert_eq!(lags(&[], &old), None);
        assert_eq!(SOON, 3 * super::LOOK_SECS as i64);
        assert_eq!(sentence(&["the desktop itself"], &["the keyring", "the wallpaper"]).as_deref(),
            Some("This update is already in use, except the desktop itself, the keyring and the wallpaper, which will be the new version after the next log out at the latest."));
        assert_eq!(sentence(&[], &["the keyring"]).as_deref(),
            Some("This update is already in use, except the keyring, which will be the new version after the next log out at the latest."));
    }

    #[test]
    fn a_program_that_only_calls_itself_a_part_of_trindle_is_not_taken_for_one() {
        use super::waits;
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let named = words(&["trindle-compose", "--start"]);
        for file in ["/home/john/bin/trindle-compose", "/home/john/bin/trindle-compose (deleted)", "/usr/bin/sleep", "/usr/bin/sleep (deleted)",
                     "/tmp/usr/lib/trindle/bin/trindle-compose (deleted)", "/usr/lib/trindle/bin/trindle-compose (deleted) (deleted)", " (deleted)", ""] {
            assert_eq!(waits(file, &named), None, "{file}");
        }
        assert_eq!(waits("/usr/lib/trindle/bin/trindle (deleted)", &named), None);
        assert_eq!(waits("/usr/lib/trindle/bin/trindle-compose (deleted)", &words(&["trindle-mcp"])), None);
        assert_eq!(waits("/home/john/trindle (deleted)", &words(&["trindle-plugin", "run", "org.example.flow", "background"])), None);
        assert_eq!(waits("/usr/lib/trindle/bin/trindle-compose (deleted)", &named), Some("the desktop itself"));
    }

    #[test]
    fn a_program_becomes_the_new_one_only_when_nothing_would_be_cut_short() {
        use super::due;
        assert!(due(true, false, true, true));
        assert!(!due(false, false, true, true));
        assert!(!due(true, true, true, true));
        assert!(!due(true, false, false, true));
        assert!(!due(true, false, true, false));
    }

    fn closed(pid: u32, what: &str) -> bool {
        let path = format!("/proc/{pid}/{what}");
        if what == "fd" {
            return std::fs::read_dir(&path).is_err();
        }
        std::fs::File::open(&path).is_err()
    }

    const INNER: &str = "TRINDLE_TEST_NOT_TRACEABLE";

    fn the_inner_program(how: &std::ffi::OsStr) {
        let dumpable = || {
            // SAFETY: prctl with PR_GET_DUMPABLE only reads a flag.
            unsafe { libc::prctl(libc::PR_GET_DUMPABLE, 0, 0, 0, 0) }
        };
        assert_eq!(dumpable(), 1, "unmarked, a program can be looked inside");
        if how == "marked" {
            assert!(super::not_traceable());
            assert_eq!(dumpable(), 0);
        }
        assert!(super::replaced().is_none());
        assert!(std::fs::read_link("/proc/self/exe").is_ok());
        assert!(std::fs::read_dir("/proc/self/task").is_ok());
        println!("ready");
        let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8; 1]);
    }

    fn plain_program_is_open(pid: u32, traced_freely: bool) {
        for open in ["mem", "environ", "maps", "fd"] {
            if open == "mem" && !traced_freely {
                continue;
            }
            assert!(!closed(pid, open), "/proc/{pid}/{open} of a program that is not marked counts as closed, so the check shows nothing");
        }
        let environment = std::fs::read(format!("/proc/{pid}/environ")).unwrap_or_default();
        assert!(String::from_utf8_lossy(&environment).contains(INNER), "an unmarked program's environment could not be read");
        assert!(std::fs::read_link(format!("/proc/{pid}/exe")).is_ok());
    }

    fn marked_program_is_closed(pid: u32, root: bool) {
        if !root {
            for what in ["mem", "environ", "maps", "fd"] {
                assert!(closed(pid, what), "/proc/{pid}/{what} can still be opened");
            }
            assert!(std::fs::read(format!("/proc/{pid}/environ")).is_err(), "a marked program's environment can still be read");
            assert!(std::fs::read_link(format!("/proc/{pid}/exe")).is_err());
        }
        assert!(std::fs::read_to_string(format!("/proc/{pid}/cmdline")).is_ok_and(|c| !c.is_empty()));
        assert!(std::fs::read_to_string(format!("/proc/{pid}/status")).is_ok_and(|s| s.contains("VmRSS:")));
        // SAFETY: getuid cannot fail.
        let me = unsafe { libc::getuid() };
        let whose = std::fs::metadata(format!("/proc/{pid}")).map(|m| std::os::unix::fs::MetadataExt::uid(&m)).ok();
        assert_eq!(whose, Some(me), "a marked program's folder is no longer the person's");
    }

    #[test]
    fn a_marked_program_cannot_be_looked_inside_by_the_same_person() {
        if let Some(how) = std::env::var_os(INNER) {
            the_inner_program(&how);
            return;
        }
        use std::io::{BufRead, Write};
        use std::process::{Command, Stdio};
        let start = |how: &str| {
            let mut inner = Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "common::renewed::tests::a_marked_program_cannot_be_looked_inside_by_the_same_person", "--nocapture", "--test-threads=1"])
                .env(INNER, how).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().unwrap();
            let mut lines = std::io::BufReader::new(inner.stdout.take().unwrap()).lines();
            assert!(lines.by_ref().any(|l| l.is_ok_and(|l| l.contains("ready"))), "the inner program ({how}) never got as far as waiting");
            (inner, lines)
        };
        let end = |(mut inner, lines): (std::process::Child, std::io::Lines<std::io::BufReader<std::process::ChildStdout>>)| {
            inner.stdin.take().unwrap().write_all(b"x").unwrap();
            lines.for_each(drop);
            assert!(inner.wait().unwrap().success());
        };
        // SAFETY: getuid cannot fail.
        let root = unsafe { libc::getuid() } == 0;
        let traced_freely = std::fs::read_to_string("/proc/sys/kernel/yama/ptrace_scope").map_or(true, |s| matches!(s.trim(), "0" | "1"));
        let plain = start("plain");
        let pid = plain.0.id();
        plain_program_is_open(pid, traced_freely);
        end(plain);
        let marked = start("marked");
        let pid = marked.0.id();
        marked_program_is_closed(pid, root);
        end(marked);
    }

    #[test]
    fn a_running_test_is_not_replaced() {
        assert!(super::replaced().is_none());
    }

    #[test]
    fn a_child_left_behind_is_cleared_up() {
        let pid = std::process::Command::new("true").spawn().unwrap().id();
        std::thread::sleep(std::time::Duration::from_millis(200));
        let stat = format!("/proc/{pid}/stat");
        assert!(std::fs::read_to_string(&stat).unwrap().contains(") Z "));
        super::adopt(|p| p as u32 == pid);
        for _ in 0..50 {
            if !std::path::Path::new(&stat).exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!std::path::Path::new(&stat).exists());
    }
}
