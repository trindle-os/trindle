//! trindle-controller.

pub mod uinput;
#[cfg(test)]
pub fn env_clean(environ: &[u8]) -> bool {
    env_refused(environ).is_empty()
}

#[cfg(test)]
mod tests;

use crate::common::controller_wire::{self as wire, Event, Pad, Refusal, Request, EV_ABS, EV_KEY, EV_SYN, LONGEST, SLOTS, WHOLE};
use std::fs;
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{Duration, Instant};

pub const ALLOWED: &[&str] = &["/usr/lib/trindle/bin/trindle-compose"];
pub const SEAT: &str = "/run/systemd/seats/seat0";
pub const MOST_CLIENTS: usize = 4;
pub const MOST_PADS: usize = 16;
const MAKES: (f64, f64) = (8.0, 1.0);
const EVENTS: (f64, f64) = (4000.0, 16000.0);
const IDLE: Duration = Duration::from_secs(30);
const OWED_EVERY: i32 = 20;
const ALLOWED_ENV: &[&str] = &[
    "HOME", "USER", "LOGNAME", "SHELL", "PATH", "PWD", "SHLVL", "_", "TERM", "LANG", "LANGUAGE",
    "XDG_RUNTIME_DIR", "XDG_SESSION_ID", "XDG_SESSION_TYPE", "XDG_SESSION_CLASS", "XDG_SESSION_DESKTOP",
    "XDG_CURRENT_DESKTOP", "XDG_SEAT", "XDG_VTNR", "XDG_DATA_DIRS", "XDG_CONFIG_DIRS", "XDG_CONFIG_HOME",
    "XDG_DATA_HOME", "XDG_STATE_HOME", "XDG_CACHE_HOME",
    "DBUS_SESSION_BUS_ADDRESS", "WAYLAND_DISPLAY", "DISPLAY",
    "XKB_DEFAULT_LAYOUT", "XKB_DEFAULT_VARIANT", "XKB_DEFAULT_MODEL", "XKB_DEFAULT_OPTIONS", "XKB_DEFAULT_RULES",
    "XCURSOR_THEME", "XCURSOR_SIZE",
    "MOZ_ENABLE_WAYLAND", "GDK_BACKEND", "ELECTRON_OZONE_PLATFORM_HINT", "SDL_VIDEODRIVER",
    "GNOME_DESKTOP_SESSION_ID", "QT_QPA_PLATFORMTHEME", "KDE_SESSION_VERSION", "GSK_RENDERER",
    "GSETTINGS_BACKEND", "GIO_USE_VFS", "GTK_USE_PORTAL",
    "TRINDLE_DRAWING", "TRINDLE_DRAWN_BY_PROCESSOR", "TRINDLE_XWAYLAND_LATER", "TRINDLE_COMPOSE_PID",
    "MAIL", "MOTD_SHOWN", "TZ", "HOSTNAME", "EDITOR", "VISUAL", "PAGER", "COLORTERM", "OLDPWD", "LS_COLORS",
    "SSH_AUTH_SOCK", "SSH_AGENT_PID", "DESKTOP_SESSION", "XDG_SESSION_PATH", "XDG_SEAT_PATH", "XDG_GREETER_DATA_DIR",
    "INVOCATION_ID", "JOURNAL_STREAM", "MANAGERPID", "SYSTEMD_EXEC_PID", "NO_AT_BRIDGE", "DEBUGINFOD_URLS",
    "RUST_BACKTRACE", "RUST_LOG", "QT_SCALE_FACTOR", "QT_AUTO_SCREEN_SCALE_FACTOR", "GDK_SCALE", "GDK_DPI_SCALE",
    "ANVIL_DISABLE_DIRECT_SCANOUT", "ANVIL_DRM_DEVICE", "ANVIL_DISABLE_10BIT", "ANVIL_XWAYLAND_SCALE",
];
const ALLOWED_ENV_PREFIX: &[&str] = &["LC_"];
const MOST_ENVIRON: u64 = 1 << 20;
const MOST_MAPS: u64 = 4 << 20;
const SYSTEM_PLACES: [&str; 4] = ["/usr", "/opt", "/lib", "/lib64"];
const SO_PEERPIDFD: libc::c_int = 77;

pub trait Kernel {
    type Made;
    fn make(&mut self, pad: &Pad) -> io::Result<Self::Made>;
    fn emit(&mut self, made: &mut Self::Made, events: &[Event]) -> io::Result<()>;
}

struct Bucket {
    tokens: f64,
    cap: f64,
    per_sec: f64,
    last: Instant,
}

impl Bucket {
    fn new((cap, per_sec): (f64, f64), now: Instant) -> Bucket {
        Bucket { tokens: cap, cap, per_sec, last: now }
    }
    fn take(&mut self, n: f64, now: Instant) -> bool {
        let gained = now.saturating_duration_since(self.last).as_secs_f64() * self.per_sec;
        self.tokens = (self.tokens + gained).min(self.cap);
        self.last = now;
        let ok = self.tokens >= n;
        if ok {
            self.tokens -= n;
        }
        ok
    }
}

struct Made<M> {
    dev: M,
    keys: Vec<u16>,
    axes: Vec<(u16, i32, i32)>,
    owed: Vec<Event>,
}

pub struct Conn<M> {
    pub fd: OwnedFd,
    sender: Option<i32>,
    pads: Vec<Option<Made<M>>>,
    makes: Bucket,
    events: Bucket,
    pub dropped: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Answer(Vec<u8>),
    Quiet,
    Close,
}

pub struct Service<K: Kernel> {
    pub kernel: K,
    pub conns: Vec<Conn<K::Made>>,
}

impl<K: Kernel> Service<K> {
    pub fn new(kernel: K) -> Service<K> {
        Service { kernel, conns: Vec::new() }
    }

    pub fn pads(&self) -> usize {
        self.conns.iter().flat_map(|c| &c.pads).filter(|p| p.is_some()).count()
    }

    pub fn add(&mut self, fd: OwnedFd, now: Instant) {
        wire::pass_credentials(fd.as_raw_fd());
        let sender = peer_of(fd.as_raw_fd()).map(|(p, _)| p.pid);
        self.conns.push(Conn {
            fd,
            sender,
            pads: (0..SLOTS).map(|_| None).collect(),
            makes: Bucket::new(MAKES, now),
            events: Bucket::new(EVENTS, now),
            dropped: 0,
        });
    }

    pub fn handle(&mut self, i: usize, frame: &[u8], now: Instant) -> Step {
        let req = match wire::parse(frame) {
            Ok(r) => r,
            Err(Refusal::Malformed) => return Step::Close,
            Err(_) if frame.first() == Some(&b'S') => return Step::Close,
            Err(why) => return Step::Answer(wire::answer(frame.get(1).copied().unwrap_or(WHOLE), Err(why))),
        };
        let total = self.pads();
        match req {
            Request::Make { slot, pad } => self.make(i, total, slot, &pad, now),
            Request::Remove { slot } => {
                let r = self.conns[i].pads[slot as usize].take().map(drop).ok_or(Refusal::NotMade);
                Step::Answer(wire::answer(slot, r))
            }
            Request::Send { slot, events } => self.send(i, slot, events, now),
        }
    }

    fn make(&mut self, i: usize, total: usize, slot: u8, pad: &Pad, now: Instant) -> Step {
        let conn = &mut self.conns[i];
        let r = if conn.pads[slot as usize].is_some() {
            Err(Refusal::SlotInUse)
        } else if total >= MOST_PADS {
            Err(Refusal::TooMany)
        } else if !conn.makes.take(1.0, now) {
            Err(Refusal::TooFast)
        } else {
            match self.kernel.make(pad) {
                Ok(dev) => {
                    let axes = pad.axes.iter().map(|a| (a.code, a.min, a.max)).collect();
                    conn.pads[slot as usize] = Some(Made { dev, keys: pad.keys.clone(), axes, owed: Vec::new() });
                    Ok(())
                }
                Err(e) => {
                    eprintln!("trindle-controller: the kernel would not make a controller: {e}");
                    Err(Refusal::Failed)
                }
            }
        };
        Step::Answer(wire::answer(slot, r))
    }

    fn send(&mut self, i: usize, slot: u8, mut events: Vec<Event>, now: Instant) -> Step {
        let conn = &mut self.conns[i];
        let Some(made) = conn.pads[slot as usize].as_mut() else { return Step::Quiet };
        for e in &mut events {
            let fits = match e.kind {
                EV_SYN => e.code == 0 && e.value == 0,
                EV_KEY => made.keys.contains(&e.code) && matches!(e.value, 0 | 1),
                EV_ABS => match made.axes.iter().find(|a| a.0 == e.code) {
                    Some(&(_, min, max)) => {
                        e.value = e.value.clamp(min, max);
                        true
                    }
                    None => false,
                },
                _ => false,
            };
            if !fits {
                return Step::Close;
            }
        }
        let cost = events.len() + if made.owed.is_empty() { 0 } else { made.owed.len() + 1 };
        if !conn.events.take(cost as f64, now) {
            conn.dropped += events.len() as u64;
            for e in events.into_iter().filter(|e| e.kind != EV_SYN) {
                match made.owed.iter_mut().find(|o| (o.kind, o.code) == (e.kind, e.code)) {
                    Some(o) => o.value = e.value,
                    None => made.owed.push(e),
                }
            }
            return Step::Quiet;
        }
        let mut all = std::mem::take(&mut made.owed);
        if !all.is_empty() {
            all.push(Event { kind: EV_SYN, code: 0, value: 0 });
        }
        all.extend(events);
        match self.kernel.emit(&mut made.dev, &all) {
            Ok(()) => Step::Quiet,
            Err(e) => {
                eprintln!("trindle-controller: a controller would not take its events: {e}");
                Step::Close
            }
        }
    }

    fn settle(&mut self, now: Instant) {
        for conn in &mut self.conns {
            for made in conn.pads.iter_mut().flatten().filter(|m| !m.owed.is_empty()) {
                if conn.events.take((made.owed.len() + 1) as f64, now) {
                    let mut all = std::mem::take(&mut made.owed);
                    all.push(Event { kind: EV_SYN, code: 0, value: 0 });
                    if self.kernel.emit(&mut made.dev, &all).is_err() {
                        all.pop();
                        made.owed = all;
                    }
                }
            }
        }
    }

    fn owing(&self) -> bool {
        self.conns.iter().flat_map(|c| &c.pads).flatten().any(|m| !m.owed.is_empty())
    }

    fn handle_judged(&mut self, i: usize, frame: &[u8], now: Instant, gate: &dyn Fn(RawFd) -> Result<(), Refusal>) -> Step {
        if frame.first() == Some(&b'M') {
            if let Err(why) = gate(self.conns[i].fd.as_raw_fd()) {
                return Step::Answer(wire::answer(frame.get(1).copied().unwrap_or(WHOLE), Err(why)));
            }
        }
        self.handle(i, frame, now)
    }

    pub fn turn(&mut self, listener: Option<&OwnedFd>, gate: &dyn Fn(RawFd) -> Result<(), Refusal>, wait_ms: i32) -> io::Result<()> {
        let wait_ms = if self.owing() && !(0..=OWED_EVERY).contains(&wait_ms) { OWED_EVERY } else { wait_ms };
        let mut fds: Vec<libc::pollfd> = self.conns.iter().map(|c| libc::pollfd { fd: c.fd.as_raw_fd(), events: libc::POLLIN, revents: 0 }).collect();
        if let Some(l) = listener {
            fds.push(libc::pollfd { fd: l.as_raw_fd(), events: libc::POLLIN, revents: 0 });
        }
        // SAFETY: fds is a whole array of pollfd of the length given.
        let r = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, wait_ms) };
        if r < 0 {
            let e = io::Error::last_os_error();
            return if e.kind() == io::ErrorKind::Interrupted { Ok(()) } else { Err(e) };
        }
        let now = Instant::now();
        self.settle(now);
        let mut buf = [0u8; LONGEST];
        for i in (0..self.conns.len()).rev() {
            let re = fds[i].revents;
            if re == 0 {
                continue;
            }
            let keep = if re & libc::POLLIN != 0 {
                match wire::recv_frame_from(self.conns[i].fd.as_raw_fd(), &mut buf) {
                    Ok(Some((_, from))) if from.is_none() || from != self.conns[i].sender => {
                        eprintln!("trindle-controller: a frame from a process that was not let in: connection closed");
                        false
                    }
                    Ok(Some((n, _))) => match self.handle_judged(i, &buf[..n], now, gate) {
                        Step::Answer(a) => wire::send_frame(self.conns[i].fd.as_raw_fd(), &a).is_ok(),
                        Step::Quiet => true,
                        Step::Close => false,
                    },
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => true,
                    _ => false,
                }
            } else {
                false
            };
            if !keep {
                self.conns.remove(i);
            }
        }
        if let Some(l) = listener.filter(|_| fds.last().is_some_and(|p| p.revents & libc::POLLIN != 0)) {
            // SAFETY: no address is asked for; the result is checked before it is owned.
            if let Ok(fd) = check(unsafe { libc::accept4(l.as_raw_fd(), std::ptr::null_mut(), std::ptr::null_mut(), libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK) }) {
                // SAFETY: fd is fresh from accept4 and nothing else owns it.
                let fd = unsafe { OwnedFd::from_raw_fd(fd) };
                let judged = if self.conns.len() >= MOST_CLIENTS { Err(Refusal::TooMany) } else { gate(fd.as_raw_fd()) };
                match judged {
                    Ok(()) => self.add(fd, now),
                    Err(why) => {
                        eprintln!("trindle-controller: a connection was refused: {why:?}");
                        let _ = wire::send_frame(fd.as_raw_fd(), &wire::answer(WHOLE, Err(why)));
                    }
                }
            }
        }
        Ok(())
    }
}

fn check(r: libc::c_int) -> io::Result<libc::c_int> {
    if r < 0 { Err(io::Error::last_os_error()) } else { Ok(r) }
}

#[derive(Clone, Copy, Debug)]
pub struct Peer {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
}

#[derive(Clone, Debug)]
pub struct Exe {
    pub dev: u64,
    pub ino: u64,
    pub owner: u32,
    pub mode: u32,
    pub links: u64,
    pub link: String,
}

#[derive(Clone, Debug)]
pub struct Facts {
    pub at_seat: Option<u32>,
    pub exe: Option<Exe>,
    pub env_clean: bool,
    pub maps_clean: bool,
    pub alive: bool,
}

#[derive(Clone, Debug)]
pub struct Allowed {
    pub path: String,
    pub dev: u64,
    pub ino: u64,
}

pub fn allowed(paths: &[&str]) -> Vec<Allowed> {
    paths.iter().filter_map(|p| fs::metadata(p).ok().map(|m| Allowed { path: p.to_string(), dev: m.dev(), ino: m.ino() })).collect()
}

pub fn judge(peer: &Peer, f: &Facts, allowed: &[Allowed]) -> Result<(), Refusal> {
    let Some(exe) = f.exe.as_ref().filter(|_| f.alive && f.env_clean && f.maps_clean && f.at_seat == Some(peer.uid)) else {
        return Err(Refusal::NotAllowed);
    };
    if exe.owner != 0 || exe.mode & 0o022 != 0 {
        return Err(Refusal::NotAllowed);
    }
    let ours = |a: &Allowed| (a.dev, a.ino) == (exe.dev, exe.ino) || (exe.links == 0 && exe.dev == a.dev && exe.link == format!("{} (deleted)", a.path));
    if allowed.iter().any(ours) { Ok(()) } else { Err(Refusal::NotAllowed) }
}

fn first_word_about(pid: i32) -> bool {
    thread_local!(static TOLD: std::cell::RefCell<std::collections::VecDeque<i32>> = std::cell::RefCell::default());
    TOLD.with(|told| {
        let mut told = told.borrow_mut();
        if told.contains(&pid) {
            return false;
        }
        if told.len() == 8 {
            told.pop_front();
        }
        told.push_back(pid);
        true
    })
}

pub fn env_refused(environ: &[u8]) -> Vec<String> {
    environ.split(|b| *b == 0).filter(|v| !v.is_empty()).filter_map(|v| {
        let name = v.split(|b| *b == b'=').next().unwrap_or(v);
        let known = ALLOWED_ENV.iter().any(|n| name == n.as_bytes()) || ALLOWED_ENV_PREFIX.iter().any(|p| name.starts_with(p.as_bytes()));
        (!known).then(|| String::from_utf8_lossy(name).into_owned())
    }).collect()
}

#[derive(Clone, Copy, Debug)]
pub struct Stat {
    pub owner: u32,
    pub mode: u32,
    pub ino: u64,
}

pub fn maps_refused(maps: &str, stat: impl Fn(&Path) -> Option<Stat>) -> Option<String> {
    let safe = |s: &Stat| s.owner == 0 && s.mode & 0o022 == 0;
    maps.lines().find_map(|line| {
        let mut fields = line.splitn(6, ' ');
        let (perms, ino) = (fields.nth(1)?, fields.nth(2)?);
        let name = fields.next()?.trim_start();
        if perms.as_bytes().get(2) != Some(&b'x') || !name.starts_with('/') {
            return None;
        }
        let refused = Some(name.to_string());
        let (path, gone) = name.strip_suffix(" (deleted)").map_or((name, false), |p| (p, true));
        let path = Path::new(path);
        if !SYSTEM_PLACES.iter().any(|p| path.starts_with(p)) {
            return refused;
        }
        for folder in path.ancestors().skip(1) {
            match stat(folder) {
                Some(s) if safe(&s) => {}
                None if gone => {}
                _ => return refused,
            }
        }
        match stat(path) {
            Some(s) if safe(&s) && (gone || ino.parse() == Ok(s.ino)) => None,
            None if gone => None,
            _ => refused,
        }
    })
}

pub fn peer_of(fd: RawFd) -> Option<(Peer, Option<OwnedFd>)> {
    // SAFETY: ucred is plain numbers; all zero is a valid value.
    let mut cred: libc::ucred = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: cred and len are valid for writing and sized for SO_PEERCRED.
    check(unsafe { libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_PEERCRED, (&mut cred as *mut libc::ucred).cast(), &mut len) }).ok()?;
    let mut pidfd: libc::c_int = -1;
    let mut len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: pidfd and len are valid for writing and sized for an int.
    let got = check(unsafe { libc::getsockopt(fd, libc::SOL_SOCKET, SO_PEERPIDFD, (&mut pidfd as *mut libc::c_int).cast(), &mut len) }).is_ok() && pidfd >= 0;
    // SAFETY: a pidfd the kernel just gave us, owned by nothing else.
    let pidfd = got.then(|| unsafe { OwnedFd::from_raw_fd(pidfd) });
    Some((Peer { pid: cred.pid, uid: cred.uid, gid: cred.gid }, pidfd))
}

struct AsPeer;

impl AsPeer {
    fn new(peer: &Peer) -> Option<AsPeer> {
        // SAFETY: no pointers; each call is checked by reading the value back.
        unsafe {
            libc::setfsgid(peer.gid);
            libc::setfsuid(peer.uid);
            let ok = libc::setfsuid(u32::MAX) as u32 == peer.uid && libc::setfsgid(u32::MAX) as u32 == peer.gid;
            let back = AsPeer;
            ok.then_some(back)
        }
    }
}

impl Drop for AsPeer {
    fn drop(&mut self) {
        // SAFETY: no pointers; going back to the real ids needs no power.
        unsafe {
            libc::setfsuid(libc::getuid());
            libc::setfsgid(libc::getgid());
        }
    }
}

fn read_most(path: &str, most: u64) -> Option<Vec<u8>> {
    let mut all = Vec::new();
    let n = fs::File::open(path).and_then(|f| f.take(most + 1).read_to_end(&mut all)).ok()?;
    ((n as u64) <= most).then_some(all)
}

fn stat_of(path: &Path) -> Option<Stat> {
    fs::symlink_metadata(path).ok().map(|m| Stat { owner: m.uid(), mode: m.mode(), ino: m.ino() })
}

pub fn gather(peer: &Peer, pidfd: Option<&OwnedFd>, seat: &Path) -> Facts {
    let at_seat = crate::thunder::active_uid(seat);
    let (exe, env_clean, maps_clean) = match AsPeer::new(peer) {
        Some(_as) => {
            let base = format!("/proc/{}", peer.pid);
            let exe = fs::read_link(format!("{base}/exe")).ok().zip(fs::metadata(format!("{base}/exe")).ok()).map(|(link, m)| Exe {
                dev: m.dev(),
                ino: m.ino(),
                owner: m.uid(),
                mode: m.mode(),
                links: m.nlink(),
                link: link.to_string_lossy().into_owned(),
            });
            let environ = read_most(&format!("{base}/environ"), MOST_ENVIRON);
            let refused = env_refused(environ.as_deref().unwrap_or_default());
            let maps = read_most(&format!("{base}/maps"), MOST_MAPS);
            let bad_code = maps.as_deref().and_then(|m| maps_refused(&String::from_utf8_lossy(m), stat_of));
            if (!refused.is_empty() || bad_code.is_some()) && first_word_about(peer.pid) {
                eprintln!("trindle-controller: game controllers refused for pid {}: start-up variables not on the allowed list: {}; code from outside the system: {}", peer.pid, refused.join(", "), bad_code.as_deref().unwrap_or("none"));
            }
            (exe, environ.is_some() && refused.is_empty(), maps.is_some() && bad_code.is_none())
        }
        None => (None, false, false),
    };
    let alive = pidfd.is_some_and(|p| {
        let mut ask = libc::pollfd { fd: p.as_raw_fd(), events: libc::POLLIN, revents: 0 };
        // SAFETY: ask is one valid pollfd and the count says so; a zero wait only looks.
        unsafe { libc::poll(&mut ask, 1, 0) == 0 }
    });
    Facts { at_seat, exe, env_clean, maps_clean, alive }
}

pub fn gate(fd: RawFd, allowed: &[Allowed], seat: &Path) -> Result<(), Refusal> {
    let (peer, pidfd) = peer_of(fd).ok_or(Refusal::NotAllowed)?;
    judge(&peer, &gather(&peer, pidfd.as_ref(), seat), allowed)
}

fn listen_fd() -> Option<OwnedFd> {
    let mine = std::env::var("LISTEN_PID").ok()?.parse::<u32>().ok()? == std::process::id();
    if !mine || std::env::var("LISTEN_FDS").ok()? != "1" {
        return None;
    }
    // SAFETY: systemd passes the one listening socket as descriptor 3 and nothing else in this process owns it.
    let fd = unsafe { OwnedFd::from_raw_fd(3) };
    wire::pass_credentials(fd.as_raw_fd());
    // SAFETY: no pointers; flags on a descriptor we own.
    unsafe {
        let flags = libc::fcntl(fd.as_raw_fd(), libc::F_GETFL);
        libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK);
        libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC);
    }
    Some(fd)
}

pub fn main(_args: Vec<String>) -> i32 {
    let Some(listener) = listen_fd() else {
        eprintln!("trindle-controller: started by trindle-controller.socket only");
        return 2;
    };
    let allowed = allowed(ALLOWED);
    let judge_peer = |fd: RawFd| gate(fd, &allowed, Path::new(SEAT));
    let mut service = Service::new(uinput::Uinput);
    let mut quiet_since = Instant::now();
    loop {
        let wait = if service.conns.is_empty() {
            let left = IDLE.saturating_sub(quiet_since.elapsed());
            if left.is_zero() {
                return 0;
            }
            left.as_millis() as i32 + 1
        } else {
            -1
        };
        let busy = !service.conns.is_empty();
        if let Err(e) = service.turn(Some(&listener), &judge_peer, wait) {
            eprintln!("trindle-controller: {e}");
            return 1;
        }
        if busy && service.conns.is_empty() {
            quiet_since = Instant::now();
        }
    }
}
