//! A plugin's background part: starting it outside the session, keeping one copy running and stopping it.

use std::os::fd::{AsRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::common::files::read_text;
use crate::store::boxed;
use crate::toolkit::t;

use super::{log, shortcuts};
use super::manifest::folder;
use super::permissions::{allowed, find, installed, my_uid};

pub fn start_background() {

    // there at once.
    let _ = shortcuts::refresh();
    for p in installed() {
        if p.background.is_some() && allowed(&p).iter().any(|a| a == "background") {

            stop_background(&p.id);
            start_keeper(&p.id);
        }
    }
}

fn keeper_command(id: &str, manager: bool) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    if manager {
        words.extend(["systemd-run", "--user", "--scope", "--quiet", "--collect", "--"].map(String::from));
    }
    words.extend(["trindle-plugin", "run", id, "background"].map(String::from));
    words
}

fn start_keeper(id: &str) -> bool {
    let manager = crate::store::pulse_guard::runtime_dir().join("systemd/private").exists()
        && crate::common::run::find("systemd-run").is_some();
    if !manager {
        eprintln!("trindle: {id}: there is no service manager to start its background part under, so it is started in this session and ends with it");
    }
    let words = keeper_command(id, manager);
    crate::common::run::spawn(&words.iter().map(String::as_str).collect::<Vec<_>>())
}

const SOON: std::time::Duration = std::time::Duration::from_secs(30);
const SOON_MOST: u32 = 5;
const WAIT_MOST: u64 = 60;

fn wait_before(soon: u32) -> std::time::Duration {
    std::time::Duration::from_secs((2u64 << soon.min(10)).min(WAIT_MOST))
}

fn screen_there() -> bool {
    let Some(name) = std::env::var_os("WAYLAND_DISPLAY").filter(|w| !w.is_empty()) else { return false };
    let path = if Path::new(&name).is_absolute() { PathBuf::from(name) } else { crate::store::pulse_guard::runtime_dir().join(name) };
    std::os::unix::net::UnixStream::connect(path).is_ok()
}

fn screen_to_go_on_with() -> bool {
    if screen_there() {
        return true;
    }
    match living_screen(&crate::store::pulse_guard::runtime_dir(), None) {
        Some(other) => {
            eprintln!("trindle: the session this background part was started in has ended; it goes on in the one at {other}");
            std::env::set_var("WAYLAND_DISPLAY", other);
            true
        }
        None => false,
    }
}

pub(super) enum Ending {
    Code(i32),
    Renew(PathBuf),
}

pub(super) fn keep_running(id: &str, allowed: &[String], argv: &[String]) -> Ending {
    let mut soon = 0;
    loop {
        let began = std::time::Instant::now();
        let code = boxed::run::run_boxed(&folder(id), id, allowed, argv);
        if !screen_to_go_on_with() {
            return Ending::Code(code);
        }
        if let Some(program) = crate::common::renewed::due_program("trindle-plugin") {
            return Ending::Renew(program);
        }
        soon = if began.elapsed() < SOON { soon + 1 } else { 0 };
        if soon >= SOON_MOST {
            eprintln!("trindle: {id} stopped {SOON_MOST} times just after starting; it is left stopped until the next login");
            return Ending::Code(code);
        }
        eprintln!("trindle: {id} stopped (code {code}); starting it again");
        std::thread::sleep(wait_before(soon));
        if !screen_to_go_on_with() {
            return Ending::Code(code);
        }
    }
}

const KEEPING: &str = "--keeping=";

pub(super) fn become_new_keeper(id: &str, program: &Path, lock: i32, log: Option<log::Kept>) -> i32 {
    eprintln!("trindle: {id} stopped by itself after an update; going on as the updated program");
    if let Some(log) = log {
        log.finish();
    }
    // SAFETY: open and dup2 only point descriptors 1 and 2 at /dev/null,

    unsafe {
        let null = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR);
        if null >= 0 {
            libc::dup2(null, 1);
            libc::dup2(null, 2);
            if null > 2 {
                libc::close(null);
            }
        }
    }
    let words = ["run".to_string(), id.to_string(), "background".to_string(), format!("{KEEPING}{lock}")];
    crate::common::renewed::become_new(program, "trindle-plugin", &words);
    1
}

pub(super) fn lock_handed_on(at: &Path, rest: &[String]) -> Option<i32> {
    use std::os::unix::fs::MetadataExt;
    let fd = rest.first()?.strip_prefix(KEEPING)?.parse::<i32>().ok().filter(|n| *n > 2)?;
    let held = std::fs::metadata(format!("/proc/self/fd/{fd}")).ok()?;
    let lock = std::fs::metadata(at.join(LOCK)).ok()?;
    if !held.is_file() || (held.dev(), held.ino()) != (lock.dev(), lock.ino()) {
        return None;
    }
    // SAFETY: flock on a descriptor of this process only takes or keeps a lock on its file.
    let kept = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) } == 0;
    // SAFETY: fcntl only clears the close-on-exec flag of a descriptor this process owns.
    (kept && unsafe { libc::fcntl(fd, libc::F_SETFD, 0) } == 0).then_some(fd)
}

const LOCK: &str = "background.lock";

const PID: &str = "background.pid";

const STOP_WAIT: Duration = Duration::from_secs(3);
const BOX_WAIT: Duration = Duration::from_secs(5);
const LOCK_WAIT: Duration = Duration::from_secs(10);
const START_WAIT: Duration = Duration::from_secs(15);
const LOOK_EVERY: Duration = Duration::from_millis(100);

pub(super) fn take_lock(at: &Path) -> Option<OwnedFd> {
    let fd = OwnedFd::from(std::fs::OpenOptions::new().write(true).create(true).truncate(false).open(at.join(LOCK)).ok()?);
    // SAFETY: fd is an open file this function owns; flock only takes or

    (unsafe { libc::flock(fd.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0).then_some(fd)
}

pub(super) fn lock_free(at: &Path) -> bool {
    !at.join(LOCK).exists() || take_lock(at).is_some()
}

pub(super) fn is_background_of(pid: i32, id: &str) -> bool {
    let words: Vec<String> = std::fs::read(format!("/proc/{pid}/cmdline")).unwrap_or_default()
        .split(|b| *b == 0).map(|w| String::from_utf8_lossy(w).into_owned()).collect();
    words.len() >= 4 && words[0].ends_with("trindle-plugin") && words[1] == "run" && words[2] == id && words[3] == "background"
}

fn keepers_of(id: &str) -> Vec<i32> {
    use std::os::unix::fs::MetadataExt;
    let (me, mine) = (std::process::id() as i32, my_uid());
    std::fs::read_dir("/proc").into_iter().flatten().flatten()
        .filter(|e| e.metadata().is_ok_and(|m| m.uid() == mine))
        .filter_map(|e| e.file_name().to_str().and_then(|n| n.parse::<i32>().ok()))
        .filter(|pid| *pid > 1 && *pid != me && is_background_of(*pid, id))
        .collect()
}

fn only_copy_at(at: &Path, id: &str, wait: Duration) -> Option<OwnedFd> {
    let began = Instant::now();
    let got = loop {
        if let Some(fd) = take_lock(at) {
            break Some(fd);
        }
        if !keepers_of(id).is_empty() {
            return None;
        }
        if began.elapsed() >= wait {
            break None;
        }
        std::thread::sleep(LOOK_EVERY);
    };
    let fd = got.or_else(|| {
        eprintln!("trindle: {id}: its lock is held by nothing that keeps it; a new one is begun");
        let _ = std::fs::remove_file(at.join(LOCK));
        take_lock(at)
    })?;
    // A note for the ring; without it the plugin still runs, and the lock says so.
    let _ = crate::common::files::write_whole(&at.join(PID), std::process::id().to_string().as_bytes());
    Some(fd)
}

pub(super) fn only_copy(id: &str) -> Option<i32> {
    use std::os::fd::IntoRawFd;
    let fd = only_copy_at(&folder(id), id, LOCK_WAIT)?.into_raw_fd();
    // SAFETY: fd is the lock just taken, kept open on purpose until this

    (unsafe { libc::fcntl(fd, libc::F_SETFD, 0) } == 0).then_some(fd)
}

fn keeper_in(at: &Path, id: &str) -> Option<i32> {
    read_text(at.join(PID)).trim().parse::<i32>().ok().filter(|p| *p > 1 && is_background_of(*p, id))
}

pub(super) fn stop_background(id: &str) {
    stop_background_at(&folder(id), id);
}

fn stop_background_at(at: &Path, id: &str) {
    let mut keepers = keepers_of(id);
    keepers.extend(keeper_in(at, id).filter(|p| !keepers.contains(p)));
    let signal = |sig: libc::c_int| for pid in keepers.iter().filter(|p| is_background_of(**p, id)) {
        // SAFETY: kill only sends a signal, to a process checked just above

        unsafe { libc::kill(*pid, sig) };
    };
    let wait = |most: Duration, gone: &dyn Fn() -> bool| {
        let began = Instant::now();
        while !gone() && began.elapsed() < most {
            std::thread::sleep(LOOK_EVERY);
        }
    };
    if !keepers.is_empty() {
        let ended = || !keepers.iter().any(|p| is_background_of(*p, id));
        signal(libc::SIGTERM);
        wait(STOP_WAIT, &ended);
        if !ended() {
            signal(libc::SIGKILL);
            wait(STOP_WAIT, &ended);
        }
    }
    wait(BOX_WAIT, &|| lock_free(at));
    for pid in &keepers {
        let _ = std::fs::remove_dir_all(crate::store::pulse_guard::runtime_dir().join("trindle-boxed").join(format!("{id}-{pid}")));
    }
    let _ = std::fs::remove_file(at.join(PID));
}

pub(super) fn back_on(id: &str) -> Option<String> {
    let p = find(id)?;
    if p.background.is_none() || !allowed(&p).iter().any(|a| a == "background") {
        return None;
    }
    begin_background(id).err()
}

fn living_screen(dir: &Path, own: Option<&str>) -> Option<String> {
    let answers = |name: &str| std::os::unix::net::UnixStream::connect(dir.join(name)).is_ok();
    own.filter(|own| answers(own)).map(str::to_string).or_else(|| screens_in(dir).into_iter().find(|s| answers(s)))
}

pub(super) fn begin_background(id: &str) -> Result<(), String> {
    reach_the_screen()?;
    if !start_keeper(id) {
        return Err(t!("plugins-bg-no-program").into());
    }
    let (at, began) = (folder(id), Instant::now());
    while began.elapsed() < START_WAIT {
        if keeper_in(&at, id).is_some() {
            return Ok(());
        }
        std::thread::sleep(LOOK_EVERY);
    }
    Err(t!("plugins-bg-slow", n = START_WAIT.as_secs()))
}

fn screens_in(dir: &Path) -> Vec<String> {
    use std::os::unix::fs::FileTypeExt;
    let mut found: Vec<String> = std::fs::read_dir(dir).into_iter().flatten().flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_socket()))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.strip_prefix("wayland-").is_some_and(|rest| !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())))
        .collect();
    found.sort();
    found
}

pub(super) fn reach_the_screen() -> Result<(), String> {
    if screen_there() {
        return Ok(());
    }
    let found = screens_in(&crate::store::pulse_guard::runtime_dir());
    match found.as_slice() {
        [one] => {
            std::env::set_var("WAYLAND_DISPLAY", one);
            Ok(())
        }
        [] => Err(t!("plugins-bg-no-session").into()),
        many => Err(t!("plugins-bg-which-session", sessions = many.join(", "))),
    }
}

pub fn restart_background(id: &str) -> i32 {
    let Some(p) = find(id) else {
        eprintln!("trindle-plugin: {id} is not an installed plugin that conforms");
        return 1;
    };

    let starts = p.background.is_some() && allowed(&p).iter().any(|a| a == "background");
    if starts {
        if let Err(why) = reach_the_screen() {
            eprintln!("trindle-plugin: {id}: {why}");
            return 1;
        }
    }
    stop_background(id);
    if starts {
        if let Err(why) = begin_background(id) {
            eprintln!("trindle-plugin: {id}: its background part is not running: {why}");
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests;
