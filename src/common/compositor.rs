//! Trindle's compositor, seen from the rest of Trindle.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const NAME: &str = "trindle-compose";

fn named(pid: u32) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/comm")).is_ok_and(|c| c.trim() == NAME)
}

pub fn desktop() -> Option<u32> {
    if let Some(pid) = std::env::var("TRINDLE_COMPOSE_PID").ok().and_then(|p| p.parse().ok()) {
        if named(pid) {
            return Some(pid);
        }
    }
    let me = std::fs::metadata("/proc/self").ok().map(|m| std::os::unix::fs::MetadataExt::uid(&m))?;
    std::fs::read_dir("/proc").ok()?.flatten().find_map(|e| {
        let pid: u32 = e.file_name().to_str()?.parse().ok()?;
        let owner = std::os::unix::fs::MetadataExt::uid(&e.metadata().ok()?);
        (owner == me && named(pid)).then_some(pid)
    })
}

pub fn end_session() {
    let pid = desktop();
    let session = pid.and_then(session_of).or_else(seat_session);
    end_programs();
    std::thread::spawn(move || {
        if shares_open() {
            within(SERVICES_WAIT, Command::new("pkexec").args([crate::common::run::admin_helper().as_str(), "smb-unmount-all"]));
        }
        within(SERVICES_WAIT, Command::new("systemctl").args(["--user", "stop", crate::session::start::SESSION_TARGET]));
        if let Some(pid) = pid {
            // SAFETY: kill only sends a signal.
            unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
            let gone = Instant::now() + FINISH_WAIT;
            while named(pid) && Instant::now() < gone {
                std::thread::sleep(Duration::from_millis(100));
            }
            if !named(pid) {
                return;
            }
            // SAFETY: as above.
            unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
        }
        if let Some(id) = session {
            within(SERVICES_WAIT, Command::new("loginctl").args(["terminate-session", &id]));
        }
    });
}

fn shares_open() -> bool {
    let mounts = std::fs::read_to_string("/proc/self/mounts").unwrap_or_default();
    let mine = crate::common::smb::user_folder(&crate::common::files::user());
    !crate::common::smb::mounted(&mounts, &mine).is_empty()
}

const SERVICES_WAIT: Duration = Duration::from_secs(8);
const FINISH_WAIT: Duration = Duration::from_secs(5);

fn within(limit: Duration, cmd: &mut Command) {
    let Ok(mut child) = cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() else { return };
    let end = Instant::now() + limit;
    while Instant::now() < end {
        if !matches!(child.try_wait(), Ok(None)) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn session_of(pid: u32) -> Option<String> {
    let id = std::fs::read_to_string(format!("/proc/{pid}/sessionid")).ok()?.trim().to_string();
    (!id.is_empty() && id != "4294967295").then_some(id)
}

fn seat_session() -> Option<String> {
    // SAFETY: getuid cannot fail.
    let uid = unsafe { libc::getuid() };
    let out = Command::new("loginctl").args(["show-user", &uid.to_string(), "-p", "Display", "--value"])
        .stdin(Stdio::null()).stderr(Stdio::null()).output().ok()?;
    let id = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!id.is_empty()).then_some(id)
}

pub fn end_programs() {
    let Ok(out) = Command::new("systemctl").args(["--user", "list-units", "--plain", "--no-legend", "app-*.scope"])
        .stdin(Stdio::null()).stderr(Stdio::null()).output() else { return };
    let own = own_scope();
    let scopes: Vec<String> = String::from_utf8_lossy(&out.stdout).lines()
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .filter(|u| Some(u) != own.as_ref())
        .collect();
    if scopes.is_empty() {
        return;
    }
    let _ = Command::new("systemctl").args(["--user", "stop", "--no-block"]).args(&scopes)
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
}

fn own_scope() -> Option<String> {
    let cgroup = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    cgroup.trim().rsplit('/').next().filter(|s| s.starts_with("app-") && s.ends_with(".scope")).map(str::to_string)
}
