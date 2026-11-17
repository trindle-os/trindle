//! The filtered session bus the box of an app in Apps is given: xdg-dbus-proxy outside it, one socket in.

use std::os::fd::{AsRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub const NOTIFICATIONS: &str = "org.freedesktop.Notifications";

const NOTIFY_PATH: &str = "/org/freedesktop/Notifications";
const NOTIFY_CALLS: [&str; 4] = ["Notify", "CloseNotification", "GetCapabilities", "GetServerInformation"];

pub const WATCHER: &str = crate::ring::sni::WATCHER;

const ITEM_FAMILIES: [&str; 2] = ["org.kde.StatusNotifierItem", "org.freedesktop.StatusNotifierItem"];
const ITEM_NUMBERS: u32 = 4;

const BAND_FIRST: u32 = 64;
const BAND_PIDS: u32 = 64;
const BANDS: u32 = 15;

pub struct Band {
    n: u32,
    _lock: std::fs::File,
}

impl Band {

    pub fn take(dir: &Path) -> Option<Band> {
        use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir).ok()?;
        (0..BANDS).find_map(|n| {
            let lock = std::fs::OpenOptions::new().read(true).write(true).create(true).mode(0o600)
                .custom_flags(libc::O_NOFOLLOW).open(dir.join(n.to_string())).ok()?;
            // SAFETY: flock only takes or refuses the lock on the descriptor it is given.
            (unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0).then_some(Band { n, _lock: lock })
        })
    }

    pub fn pids(&self) -> std::ops::RangeInclusive<u32> {
        let first = BAND_FIRST + self.n * BAND_PIDS;
        first..=first + BAND_PIDS - 1
    }
}

const READY_MS: i32 = 5000;

pub struct Bus {
    child: Child,
    _alive: OwnedFd,
    socket: PathBuf,
}

impl Bus {

    pub fn start(run: &Path, band: Option<&Band>) -> Result<Bus, String> {
        Bus::start_at(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), run, band)
    }

    fn start_at(address: Option<&str>, run: &Path, band: Option<&Band>) -> Result<Bus, String> {
        let socket = run.join("bus");
        let (child, alive) = launch("xdg-dbus-proxy", &session_address(address)?, &socket, band.map(Band::pids))?;
        Ok(Bus { child, _alive: alive, socket })
    }

    pub fn socket(&self) -> String {
        self.socket.to_string_lossy().into_owned()
    }

    #[cfg(test)]
    pub fn pid(&self) -> u32 {
        self.child.id()
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn session_address(var: Option<&str>) -> Result<String, String> {
    let rest = var.and_then(|v| v.strip_prefix("unix:path=")).ok_or("the session's bus is not on a socket file")?;
    let path = rest.split(',').next().unwrap_or_default();
    if !path.starts_with('/') || path.contains(['%', ';']) {
        return Err("the session's bus address is not a plain path".into());
    }
    use std::os::unix::fs::FileTypeExt;
    if !std::fs::metadata(path).is_ok_and(|m| m.file_type().is_socket()) {
        return Err(format!("{path} is not a socket"));
    }
    Ok(format!("unix:path={path}"))
}

fn launch(program: &str, address: &str, socket: &Path, pids: Option<std::ops::RangeInclusive<u32>>) -> Result<(Child, OwnedFd), String> {
    let (alive, theirs) = crate::store::secctx::close_pipe().ok_or("no pipe could be made")?;
    crate::store::secctx::keep_across_exec(&theirs);
    let child = Command::new(program).args(proxy_args(address, socket, theirs.as_raw_fd(), pids))
        .stdin(Stdio::null()).stdout(Stdio::null()).spawn()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound { format!("{program} is not installed") } else { format!("{program}: {e}") })?;
    drop(theirs);
    let mut child = child;
    match ready(&alive) {
        Ok(()) => Ok((child, alive)),
        Err(why) => {
            let _ = child.kill();
            let _ = child.wait();
            Err(format!("{program} {why}"))
        }
    }
}

fn ready(fd: &OwnedFd) -> Result<(), &'static str> {
    let mut p = libc::pollfd { fd: fd.as_raw_fd(), events: libc::POLLIN, revents: 0 };
    // SAFETY: poll reads and writes the one pollfd it is given.
    if unsafe { libc::poll(&mut p, 1, READY_MS) } <= 0 {
        return Err("did not start in time");
    }
    let mut byte = 0u8;
    // SAFETY: read writes at most one byte into the one it is given.
    match unsafe { libc::read(fd.as_raw_fd(), (&mut byte as *mut u8).cast(), 1) } {
        1 => Ok(()),
        _ => Err("ended as it started"),
    }
}

pub fn proxy_args(address: &str, socket: &Path, fd: i32, pids: Option<std::ops::RangeInclusive<u32>>) -> Vec<String> {
    let mut a = vec![format!("--fd={fd}"), address.to_string(), socket.to_string_lossy().into_owned(), "--filter".into()];
    a.extend(NOTIFY_CALLS.iter().map(|m| format!("--call={NOTIFICATIONS}={NOTIFICATIONS}.{m}@{NOTIFY_PATH}")));
    a.push(format!("--broadcast={NOTIFICATIONS}={NOTIFICATIONS}.*@{NOTIFY_PATH}"));
    a.push(format!("--talk={WATCHER}"));
    for pid in pids.into_iter().flatten() {
        for family in ITEM_FAMILIES {
            a.extend((1..=ITEM_NUMBERS).map(|n| format!("--own={family}-{pid}-{n}")));
        }
    }
    a
}

#[path = "bus_tests.rs"]
#[cfg(test)]
mod bus_tests;
