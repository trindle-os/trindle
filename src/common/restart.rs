//! Restarting the desktop from the keyboard: the parts that are ended and started again, the two keys, and the flag a full restart leaves for the session.

use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub const PARTS_KEYS: &str = "restart-parts-keys";
pub const ALL_KEYS: &str = "restart-all-keys";

pub const PARTS_DEFAULT: &str = "Ctrl+Alt+Escape";
pub const ALL_DEFAULT: &str = "Ctrl+Alt+Shift+Escape";

pub const ASK_COMMAND: &str = "trindle-boxed --restart-ask";

pub struct Part {
    pub name: &'static str,

    pub start: Option<&'static [&'static str]>,
}

pub const PARTS: &[Part] = &[
    Part { name: "trindle-ring", start: None },
    Part { name: "trindle-wallpaper", start: Some(&[]) },
    Part { name: "trindle-clipboard", start: Some(&["watch"]) },
    Part { name: "trindle-keyring", start: Some(&[]) },
    Part { name: "trindle-idle", start: Some(&[]) },
    Part { name: "trindle-wsdd", start: Some(&[]) },
    Part { name: "trindle-nightlight", start: Some(&[]) },
];

const LEAVE_WAIT: Duration = Duration::from_secs(4);

const FLAG_FRESH: Duration = Duration::from_secs(60);

static UNDER_WAY: AtomicBool = AtomicBool::new(false);

fn running(part: &Part) -> Vec<OwnedFd> {
    // SAFETY: getuid cannot fail.
    let me = unsafe { libc::getuid() };
    let Ok(dir) = std::fs::read_dir("/proc") else { return Vec::new() };
    let mut found = Vec::new();
    for p in dir.flatten() {
        let Some(pid) = p.file_name().to_str().and_then(|n| n.parse::<u32>().ok()) else { continue };
        if pid == std::process::id() {
            continue;
        }
        // SAFETY: pidfd_open takes two numbers and gives back a descriptor or -1, checked here.
        let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::pid_t, 0) };
        if fd < 0 {
            continue;
        }
        // SAFETY: a descriptor just made, owned by nothing else.
        let fd = unsafe { OwnedFd::from_raw_fd(fd as libc::c_int) };
        if p.metadata().map(|m| m.uid()).ok() != Some(me) {
            continue;
        }
        let cmdline = std::fs::read(p.path().join("cmdline")).unwrap_or_default();
        let words: Vec<&[u8]> = cmdline.split(|b| *b == 0).filter(|w| !w.is_empty()).collect();
        let name = words.first().map(|w| String::from_utf8_lossy(w).rsplit('/').next().unwrap_or_default().to_string());
        if name.as_deref() != Some(part.name) {
            continue;
        }
        if let Some(start) = part.start {
            if !start.iter().zip(&words[1..]).all(|(a, b)| a.as_bytes() == *b) || words.len() - 1 < start.len() {
                continue;
            }
        }
        let exe = std::fs::read_link(p.path().join("exe")).unwrap_or_default();
        let exe = exe.to_string_lossy();
        if exe.trim_end_matches(" (deleted)").rsplit('/').next().is_some_and(|f| f.starts_with("trindle")) {
            found.push(fd);
        }
    }
    found
}

fn alive(fd: &OwnedFd) -> bool {
    let mut poll = libc::pollfd { fd: fd.as_raw_fd(), events: libc::POLLIN, revents: 0 };
    // SAFETY: one pollfd, owned here, and no waiting.
    unsafe { libc::poll(&mut poll, 1, 0) == 0 }
}

fn signal(fd: &OwnedFd, signal: libc::c_int) {
    // SAFETY: the descriptor is a process handle held open; no signal info is given.
    unsafe { libc::syscall(libc::SYS_pidfd_send_signal, fd.as_raw_fd(), signal, std::ptr::null::<libc::siginfo_t>(), 0) };
}

pub fn end(parts: &[Part]) -> Vec<(&Part, Vec<OwnedFd>)> {
    parts.iter().map(|part| {
        let held = running(part);
        for fd in &held {
            signal(fd, libc::SIGTERM);
        }
        (part, held)
    }).filter(|(_, held)| !held.is_empty()).collect()
}

pub fn restart_with(parts: &[Part], launch: impl Fn(&Part)) {
    let ended = end(parts);
    let by = Instant::now() + LEAVE_WAIT;
    while ended.iter().any(|(_, held)| held.iter().any(alive)) && Instant::now() < by {
        std::thread::sleep(Duration::from_millis(50));
    }

    for fd in ended.iter().flat_map(|(_, held)| held).filter(|fd| alive(fd)) {
        signal(fd, libc::SIGKILL);
    }
    let again = Instant::now() + Duration::from_secs(1);
    while ended.iter().any(|(_, held)| held.iter().any(alive)) && Instant::now() < again {
        std::thread::sleep(Duration::from_millis(50));
    }
    for (part, held) in &ended {
        if part.start.is_some() && !held.iter().any(alive) {
            launch(part);
        }
    }
}

fn start(part: &Part, env: &[(String, String)]) {
    let Some(words) = part.start else { return };
    let child = Command::new(part.name).args(words).envs(env.iter().map(|(k, v)| (k, v))).stdin(Stdio::null())
        .stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    if let Ok(mut child) = child {
        std::thread::spawn(move || { let _ = child.wait(); });
    }
}

pub fn parts(env: Vec<(String, String)>) {
    if UNDER_WAY.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(move || {
        parts_with(&env);
        UNDER_WAY.store(false, Ordering::SeqCst);
    });
}

pub fn parts_now() {
    let env: Vec<(String, String)> = ["WAYLAND_DISPLAY", "DISPLAY"].iter().filter_map(|k| std::env::var(k).ok().map(|v| (k.to_string(), v))).collect();
    parts_with(&env);
}

fn parts_with(env: &[(String, String)]) {
    if let Some(flag) = restarted_flag() {
        let _ = std::fs::write(flag, "");
    }
    restart_with(PARTS, |part| start(part, env));
}

fn restarted_flag() -> Option<PathBuf> {
    flag().map(|f| f.with_extension("parts"))
}

pub fn restarted_just_now() -> bool {
    restarted_flag().is_some_and(|f| taken(&f))
}

pub fn flag() -> Option<PathBuf> {
    std::env::var_os("XDG_RUNTIME_DIR").filter(|d| !d.is_empty()).map(|d| PathBuf::from(d).join("trindle-restart-all"))
}

pub fn all_asked() -> bool {
    flag().is_some_and(|f| taken(&f))
}

fn taken(flag: &std::path::Path) -> bool {
    let fresh = std::fs::metadata(flag).ok().and_then(|m| m.modified().ok())
        .is_some_and(|t| t.elapsed().is_ok_and(|e| e < FLAG_FRESH));
    let _ = std::fs::remove_file(flag);
    fresh
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::CommandExt;
    use std::process::Child;
    use std::sync::Mutex;

    fn fake(dir: &std::path::Path, name: &str) -> Child {
        let at = dir.join(name);
        if !at.exists() {
            std::fs::copy("/bin/sleep", &at).unwrap();
        }
        Command::new(&at).arg0(name).arg("60").spawn().unwrap()
    }

    fn gone(child: &mut Child) -> bool {
        std::thread::sleep(Duration::from_millis(200));
        child.try_wait().unwrap().is_some()
    }

    #[test]
    fn restarting_the_parts_leaves_every_window_s_process_alive() {
        let dir = std::env::temp_dir().join(format!("trindle-restart-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (part_name, window_name): (&'static str, &'static str) = (
            Box::leak(format!("trindle-fakepart-{}", std::process::id()).into_boxed_str()),
            Box::leak(format!("trindle-fakewindow-{}", std::process::id()).into_boxed_str()),
        );
        let parts: &[Part] = &[Part { name: part_name, start: Some(&["60"]) }];
        let mut part = fake(&dir, part_name);
        let mut window = fake(&dir, window_name);
        let started: Mutex<Vec<Child>> = Mutex::new(Vec::new());
        restart_with(parts, |p| started.lock().unwrap().push(fake(&dir, p.name)));
        assert!(gone(&mut part), "the part was not ended");
        assert!(!gone(&mut window), "an open window's program was ended");
        let mut again = started.into_inner().unwrap();
        assert_eq!(again.len(), 1, "the part was not started afresh exactly once");
        assert!(!gone(&mut again[0]));
        let _ = window.kill();
        let _ = again[0].kill();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_part_that_was_not_running_is_not_started() {
        let parts: &[Part] = &[Part { name: "trindle-fakenever-running", start: Some(&[]) }];
        let started = Mutex::new(0);
        restart_with(parts, |_| *started.lock().unwrap() += 1);
        assert_eq!(*started.lock().unwrap(), 0);
    }

    #[test]
    fn the_defaults_files_keeps_are_the_ones_named_here() {
        assert_eq!(crate::common::files::default_setting(PARTS_KEYS), PARTS_DEFAULT);
        assert_eq!(crate::common::files::default_setting(ALL_KEYS), ALL_DEFAULT);
    }

    #[test]
    fn the_flag_counts_once_and_only_when_fresh() {
        let dir = std::env::temp_dir().join(format!("trindle-flag-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();

        let flag = dir.join("trindle-restart-all");
        assert!(!taken(&flag));
        std::fs::write(&flag, "").unwrap();
        assert!(taken(&flag));
        assert!(!taken(&flag), "the flag was not taken away");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
