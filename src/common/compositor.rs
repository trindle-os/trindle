//! Trindle's compositor, seen from the rest of Trindle: finding it, and
//! ending the session, which is the compositor finishing. Changed settings
//! need no telling: it notices their files change and takes them up.

use std::process::{Command, Stdio};

use super::run::parsed;
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
    std::thread::spawn(move || {
        end_programs();

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
    output_within(limit, cmd.stdout(Stdio::null()));
}

fn output_within(limit: Duration, cmd: &mut Command) -> Option<String> {
    use std::io::Read;
    let mut child = cmd.stdin(Stdio::null()).stderr(Stdio::null()).spawn().ok()?;
    let end = Instant::now() + limit;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < end => std::thread::sleep(Duration::from_millis(50)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    };
    let mut out = String::new();
    if let Some(mut pipe) = child.stdout.take() {
        let _ = pipe.by_ref().take(1 << 20).read_to_string(&mut out);
    }
    status.success().then_some(out)
}

fn session_of(pid: u32) -> Option<String> {
    let id = std::fs::read_to_string(format!("/proc/{pid}/sessionid")).ok()?.trim().to_string();
    (!id.is_empty() && id != "4294967295").then_some(id)
}

fn seat_session() -> Option<String> {
    // SAFETY: getuid cannot fail.
    let uid = unsafe { libc::getuid() };
    let out = parsed(Command::new("loginctl").args(["show-user", &uid.to_string(), "-p", "Display", "--value"])
        .stdin(Stdio::null()).stderr(Stdio::null())).output().ok()?;
    let id = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!id.is_empty()).then_some(id)
}

pub fn end_programs() {
    let Some(out) = output_within(SERVICES_WAIT, parsed(Command::new("systemctl")
        .args(["--user", "list-units", "--plain", "--no-legend", "app-*.scope"]).stdout(Stdio::piped()))) else { return };
    let own = own_scope();
    let scopes: Vec<String> = out.lines()
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .filter(|u| Some(u) != own.as_ref())
        .collect();
    if scopes.is_empty() {
        return;
    }
    within(SERVICES_WAIT, Command::new("systemctl").args(["--user", "stop", "--no-block"]).args(&scopes));
}

fn own_scope() -> Option<String> {
    let cgroup = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    cgroup.trim().rsplit('/').next().filter(|s| s.starts_with("app-") && s.ends_with(".scope")).map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_command_that_hangs_is_ended_at_its_limit_and_one_that_answers_is_heard() {
        let started = Instant::now();
        assert_eq!(output_within(Duration::from_millis(300), Command::new("sleep").arg("30").stdout(Stdio::piped())), None);
        assert!(started.elapsed() < Duration::from_secs(5), "it was waited for to its end");
        assert_eq!(output_within(SERVICES_WAIT, Command::new("echo").arg("hi").stdout(Stdio::piped())).as_deref(), Some("hi\n"));
        assert_eq!(output_within(SERVICES_WAIT, Command::new("false").stdout(Stdio::piped())), None);
    }
}
