//! Xwayland, started only when an older X11 program first asks for it.

use std::os::unix::process::CommandExt;

pub const WAITING_ROOM: &str = "/usr/lib/trindle/xwayland-later";
const WAITING_ROOM_ENV: &str = "TRINDLE_XWAYLAND_LATER";
const REAL: &str = "/usr/bin/Xwayland";
pub const STOP_ENV: &str = "TRINDLE_XWAYLAND_STOP";

pub fn run_if_asked() {
    let called = std::env::args_os().next().unwrap_or_default();
    if std::path::Path::new(&called).file_name().is_none_or(|n| n != "Xwayland") {
        return;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    let listening: Vec<i32> = args.windows(2)
        .filter(|w| w[0] == "-listenfd")
        .filter_map(|w| w[1].parse().ok())
        .collect();
    let wayland: Option<i32> = std::env::var("WAYLAND_SOCKET").ok().and_then(|v| v.parse().ok());
    loop {
        let mut fds: Vec<libc::pollfd> = listening.iter()
            .map(|&fd| libc::pollfd { fd, events: libc::POLLIN, revents: 0 })
            .collect();
        if let Some(fd) = wayland {
            fds.push(libc::pollfd { fd, events: 0, revents: 0 });
        }
        // SAFETY: a plain syscall on an array of pollfd built just above,
        // whose length is passed with it.
        let n = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, -1) };
        if n < 0 {
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            std::process::exit(1);
        }
        if wayland.is_some() && fds.last().is_some_and(|p| p.revents & (libc::POLLHUP | libc::POLLERR) != 0) {
            std::process::exit(0);
        }
        if fds.iter().take(listening.len()).any(|p| p.revents & libc::POLLIN != 0) {
            break;
        }
    }
    let stop = std::env::var(STOP_ENV).ok();
    std::env::remove_var(STOP_ENV);
    let e = std::process::Command::new(REAL).arg0("Xwayland").args(real_args(&args, stop.as_deref())).exec();
    eprintln!("trindle-compose: could not start {REAL}: {e}");
    std::process::exit(1);
}

pub fn real_args(args: &[String], stop: Option<&str>) -> Vec<String> {
    let stop = stop.filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()));
    let mut out = Vec::with_capacity(args.len() + 1);
    for a in args {
        if a == "-terminate" {
            if let Some(secs) = stop {
                out.push(a.clone());
                out.push(secs.to_string());
            }
            continue;
        }
        out.push(a.clone());
    }
    out
}

fn waiting_room() -> Option<std::path::PathBuf> {
    let at = std::env::var_os(WAITING_ROOM_ENV).map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(WAITING_ROOM));
    at.join("Xwayland").exists().then_some(at)
}

pub fn path() -> Option<String> {
    waiting_room().map(|at| format!("{}:{}", at.display(), std::env::var("PATH").unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_desktops_xwayland_stops_and_a_boxs_stays() {
        let smithay = v(&[":3", "-verbose", "-rootless", "-terminate", "-wm", "9", "-displayfd", "11", "-listenfd", "7"]);
        assert_eq!(real_args(&smithay, Some("10")),
                   v(&[":3", "-verbose", "-rootless", "-terminate", "10", "-wm", "9", "-displayfd", "11", "-listenfd", "7"]));
        assert_eq!(real_args(&smithay, None),
                   v(&[":3", "-verbose", "-rootless", "-wm", "9", "-displayfd", "11", "-listenfd", "7"]));
        assert!(!real_args(&smithay, Some("10 -ac")).iter().any(|a| a.contains("-ac") || a == "-terminate"));
        assert!(!real_args(&smithay, Some("")).iter().any(|a| a == "-terminate"));
    }
}
