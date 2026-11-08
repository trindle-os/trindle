//! How a plugin allowed "copied" is handed the files the person just copied.

use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use crate::common::copied::{self, Entry, Gate, Limits, Roots, Stop};
use crate::common::selection::{Held, Reader};

pub const BATCH: usize = 250;

const BODY_MAX: usize = 60_000;

const REQUEST_MAX: usize = 64;

const IDLE: Duration = Duration::from_secs(60);

const SEND_FOR: Duration = Duration::from_secs(30);

const BUSY: usize = 100;

const PARTING: Duration = Duration::from_secs(2);

const STALL: Duration = Duration::from_secs(120);

const PATIENCE: Duration = Duration::from_secs(60);

const CONNECTIONS: usize = 4;

const MAX_FDS: usize = 253;

fn cvt(n: isize) -> io::Result<usize> {
    if n < 0 { Err(io::Error::last_os_error()) } else { Ok(n as usize) }
}

fn owned(fd: RawFd) -> OwnedFd {
    // SAFETY: only called with descriptors the kernel has just made for this

    unsafe { OwnedFd::from_raw_fd(fd) }
}

pub fn send_message(sock: RawFd, body: &[u8], fds: &[RawFd]) -> io::Result<()> {
    if fds.len() > MAX_FDS {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many descriptors"));
    }
    let mut data = Vec::with_capacity(4 + body.len());
    data.extend_from_slice(&(body.len() as u32).to_be_bytes());
    data.extend_from_slice(body);
    let mut iov = libc::iovec { iov_base: data.as_ptr() as *mut libc::c_void, iov_len: data.len() };
    // SAFETY: CMSG_SPACE only works out a size.
    let space = unsafe { libc::CMSG_SPACE((fds.len() * 4) as u32) } as usize;
    let mut control = vec![0u64; space.div_ceil(8)];
    // SAFETY: an all-zero msghdr is a valid empty one.
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    if !fds.is_empty() {
        msg.msg_control = control.as_mut_ptr() as *mut libc::c_void;
        msg.msg_controllen = space as _;
        // SAFETY: `control` is as big as CMSG_SPACE says and aligned for a

        unsafe {
            let c = libc::CMSG_FIRSTHDR(&msg);
            (*c).cmsg_level = libc::SOL_SOCKET;
            (*c).cmsg_type = libc::SCM_RIGHTS;
            (*c).cmsg_len = libc::CMSG_LEN((fds.len() * 4) as u32) as _;
            std::ptr::copy_nonoverlapping(fds.as_ptr(), libc::CMSG_DATA(c) as *mut RawFd, fds.len());
        }
    }
    loop {
        // SAFETY: `msg` points at `iov` and `control`, which live through the

        let n = unsafe { libc::sendmsg(sock, &msg, libc::MSG_NOSIGNAL) };
        match cvt(n) {
            Ok(n) if n == data.len() => return Ok(()),
            Ok(_) => return Err(io::Error::new(io::ErrorKind::WriteZero, "a short message")),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

pub fn recv_message(sock: RawFd, max: usize) -> io::Result<Option<(Vec<u8>, Vec<OwnedFd>)>> {
    let mut buf = vec![0u8; 4 + max];
    let mut iov = libc::iovec { iov_base: buf.as_mut_ptr() as *mut libc::c_void, iov_len: buf.len() };
    // SAFETY: CMSG_SPACE only works out a size.
    let space = unsafe { libc::CMSG_SPACE((MAX_FDS * 4) as u32) } as usize;
    let mut control = vec![0u64; space.div_ceil(8)];
    // SAFETY: an all-zero msghdr is a valid empty one.
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = space as _;
    let n = loop {
        // SAFETY: `msg` points at `iov` and `control`, which live through the

        let n = unsafe { libc::recvmsg(sock, &mut msg, libc::MSG_CMSG_CLOEXEC) };
        match cvt(n) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            other => break other?,
        }
    };

    let mut fds: Vec<OwnedFd> = Vec::new();
    // SAFETY: the kernel filled `control` with well-formed headers, which

    unsafe {
        let mut c = libc::CMSG_FIRSTHDR(&msg);
        while !c.is_null() {
            if (*c).cmsg_level == libc::SOL_SOCKET && (*c).cmsg_type == libc::SCM_RIGHTS {
                let len = (*c).cmsg_len as usize - libc::CMSG_LEN(0) as usize;
                let data = libc::CMSG_DATA(c) as *const RawFd;
                for i in 0..len / 4 {
                    fds.push(owned(std::ptr::read_unaligned(data.add(i))));
                }
            }
            c = libc::CMSG_NXTHDR(&msg, c);
        }
    }
    if msg.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "too long"));
    }
    if n == 0 {
        return Ok(None);
    }
    if n < 4 || u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize != n - 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "the length does not match"));
    }
    buf.truncate(n);
    buf.drain(..4);
    Ok(Some((buf, fds)))
}

fn set_timeout(sock: RawFd, option: i32, d: Duration) {
    let tv = libc::timeval { tv_sec: d.as_secs() as _, tv_usec: d.subsec_micros() as _ };
    // SAFETY: a timeval of the size given, which lives through the call.
    unsafe {
        libc::setsockopt(sock, libc::SOL_SOCKET, option, &tv as *const _ as *const libc::c_void,
                         std::mem::size_of::<libc::timeval>() as u32);
    }
}

fn stream() -> io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [0 as RawFd; 2];
    // SAFETY: pipe2 writes two descriptors into the array, which has room.
    cvt(unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } as isize)?;
    let (read, write) = (owned(fds[0]), owned(fds[1]));
    // SAFETY: F_SETFL on a descriptor this function holds open.
    cvt(unsafe { libc::fcntl(write.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) } as isize)?;
    Ok((read, write))
}

struct Job {
    src: std::fs::File,
    dst: std::fs::File,
    left: u64,
}

struct Active {
    job: Job,
    pending: Vec<u8>,
    sent: usize,
    progress: Instant,
}

pub struct Pump {
    jobs: mpsc::Sender<Job>,
    live: Arc<AtomicUsize>,
}

impl Pump {
    pub fn new() -> Pump {
        let (jobs, rx) = mpsc::channel::<Job>();
        let live = Arc::new(AtomicUsize::new(0));
        let count = live.clone();
        std::thread::spawn(move || pump(rx, count));
        Pump { jobs, live }
    }

    fn submit(&self, src: OwnedFd, dst: OwnedFd, size: u64) {
        self.live.fetch_add(1, Ordering::SeqCst);
        let job = Job { src: src.into(), dst: dst.into(), left: size };
        if self.jobs.send(job).is_err() {
            self.live.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub fn busy(&self) -> usize {
        self.live.load(Ordering::SeqCst)
    }

    fn wait_below(&self, below: usize, within: Duration, dead: &AtomicBool) -> bool {
        let until = Instant::now() + within;
        while self.busy() >= below {
            if dead.load(Ordering::SeqCst) || Instant::now() >= until {
                return false;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        true
    }
}

impl Default for Pump {
    fn default() -> Pump {
        Pump::new()
    }
}

fn pump(rx: mpsc::Receiver<Job>, live: Arc<AtomicUsize>) {
    let mut active: Vec<Active> = Vec::new();
    let mut more = true;
    let mut chunk = vec![0u8; 32 * 1024];
    loop {
        if active.is_empty() {
            if !more {
                return;
            }
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(job) => active.push(Active { job, pending: Vec::new(), sent: 0, progress: Instant::now() }),
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        }
        while more {
            match rx.try_recv() {
                Ok(job) => active.push(Active { job, pending: Vec::new(), sent: 0, progress: Instant::now() }),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => more = false,
            }
        }
        let mut polls: Vec<libc::pollfd> =
            active.iter().map(|a| libc::pollfd { fd: a.job.dst.as_raw_fd(), events: libc::POLLOUT, revents: 0 }).collect();
        // SAFETY: `polls` is a run of pollfd of the length given; each names a

        unsafe { libc::poll(polls.as_mut_ptr(), polls.len() as libc::nfds_t, 100) };
        let mut done = vec![false; active.len()];
        for (i, a) in active.iter_mut().enumerate() {
            let ev = polls[i].revents;
            if ev & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL) != 0 {
                done[i] = true;
            } else if ev & libc::POLLOUT != 0 {
                let (finished, moved) = step(a, &mut chunk);
                done[i] = finished;
                if moved {
                    a.progress = Instant::now();
                }
            }
            if !done[i] && a.progress.elapsed() > STALL {
                done[i] = true;
            }
        }
        let mut i = 0;
        active.retain(|_| {
            let gone = done[i];
            i += 1;
            if gone {
                live.fetch_sub(1, Ordering::SeqCst);
            }
            !gone
        });
    }
}

fn step(a: &mut Active, chunk: &mut [u8]) -> (bool, bool) {
    if a.sent >= a.pending.len() {
        if a.job.left == 0 {
            return (true, false);
        }
        let want = chunk.len().min(a.job.left.min(usize::MAX as u64) as usize);
        match a.job.src.read(&mut chunk[..want]) {
            Ok(0) | Err(_) => return (true, false),
            Ok(n) => {
                a.pending.clear();
                a.pending.extend_from_slice(&chunk[..n]);
                a.sent = 0;
                a.job.left -= n as u64;
            }
        }
    }
    let mut moved = false;
    match a.job.dst.write(&a.pending[a.sent..]) {
        Ok(n) => {
            a.sent += n;
            moved = n > 0;
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {}
        Err(_) => return (true, false),
    }
    (a.sent >= a.pending.len() && a.job.left == 0, moved)
}

pub struct Serve {

    pub clipboard: Box<dyn Fn() -> Option<Held> + Send + Sync>,

    pub allowed: Box<dyn Fn() -> bool + Send + Sync>,

    pub note: Box<dyn Fn() + Send + Sync>,
    pub roots: Roots,
    pub limits: Limits,
    pub gate: Mutex<Gate>,
    pub pump: Pump,
    pub dead: AtomicBool,
}

enum Fail {
    Refuse(&'static str),
    Gone,
}

fn reply(sock: RawFd, text: &str) -> bool {
    send_message(sock, text.as_bytes(), &[]).is_ok()
}

fn part(sock: RawFd, text: &str) {
    if !reply(sock, text) {
        return;
    }
    // SAFETY: shutdown on a socket the caller holds open; it touches no memory.
    unsafe { libc::shutdown(sock, libc::SHUT_WR) };
    let until = Instant::now() + PARTING;
    loop {
        let left = until.saturating_duration_since(Instant::now());
        if left < Duration::from_millis(1) {
            return;
        }
        set_timeout(sock, libc::SO_RCVTIMEO, left);
        match recv_message(sock, REQUEST_MAX) {

            Ok(Some(_)) => {}

            Err(e) if e.kind() == io::ErrorKind::InvalidData => {}

            _ => return,
        }
    }
}

pub fn serve_connection(sock: OwnedFd, s: &Serve) {
    let raw = sock.as_raw_fd();
    set_timeout(raw, libc::SO_RCVTIMEO, Duration::from_secs(1));
    set_timeout(raw, libc::SO_SNDTIMEO, SEND_FOR);
    let mut quiet = Instant::now();
    loop {
        if s.dead.load(Ordering::SeqCst) || !(s.allowed)() {
            part(raw, "ERR\tnot_allowed");
            return;
        }
        match recv_message(raw, REQUEST_MAX) {
            Ok(Some((body, fds))) => {

                if s.dead.load(Ordering::SeqCst) || !(s.allowed)() {
                    part(raw, "ERR\tnot_allowed");
                    return;
                }
                if !fds.is_empty() {

                    drop(fds);
                    part(raw, "ERR\tbad_request");
                    return;
                }
                if body != b"LIST" {
                    part(raw, "ERR\tbad_request");
                    return;
                }
                quiet = Instant::now();
                match list(raw, s) {
                    Ok(()) => {}
                    Err(Fail::Refuse(why)) => {
                        if !reply(raw, &format!("ERR\t{why}")) {
                            return;
                        }
                    }
                    Err(Fail::Gone) => return,
                }
            }
            Ok(None) => return,
            Err(e) if matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut) => {
                if quiet.elapsed() > IDLE {
                    return;
                }
            }
            Err(_) => {
                part(raw, "ERR\tbad_request");
                return;
            }
        }
    }
}

fn list(sock: RawFd, s: &Serve) -> Result<(), Fail> {
    let plan = judged(s)?;
    (s.note)();

    copied::gather(&plan, &s.limits, &mut |_| Ok(())).map_err(|e| match e {
        Stop::TooBig => Fail::Refuse("too_big"),
        Stop::Gone => Fail::Gone,
    })?;

    let mut batch = Batch::default();
    let mut busy = false;
    let result = copied::gather(&plan, &s.limits, &mut |entry| send_entry(sock, s, entry, &mut batch, &mut busy));
    let tally = match result {
        Ok(t) => t,
        Err(Stop::TooBig) => return Err(Fail::Refuse("too_big")),
        Err(Stop::Gone) if busy => return Err(Fail::Refuse("busy")),
        Err(Stop::Gone) => return Err(Fail::Gone),
    };
    batch.send(sock).map_err(|_| Fail::Gone)?;
    let line = format!("DONE\t{}\t{}\t{}\t{}", if plan.cut { "cut" } else { "copy" }, tally.files, tally.bytes,
                       tally.skipped + plan.refused);
    if reply(sock, &line) { Ok(()) } else { Err(Fail::Gone) }
}

fn judged(s: &Serve) -> Result<copied::Plan, Fail> {
    let now = Instant::now();
    s.gate.lock().unwrap_or_else(|e| e.into_inner()).rate(now).map_err(Fail::Refuse)?;
    let held = (s.clipboard)().ok_or(Fail::Refuse("unavailable"))?;
    Gate::recent(Instant::now(), held.copied_at).map_err(Fail::Refuse)?;
    if held.kinds.iter().any(|(_, d)| d.len() as u64 > copied::LIST_MAX) {
        return Err(Fail::Refuse("too_big"));
    }
    let plan = copied::plan(&held.kinds, &s.roots);
    if plan.tops.is_empty() {
        return Err(Fail::Refuse("no_files"));
    }
    Ok(plan)
}

fn send_entry(sock: RawFd, s: &Serve, entry: Entry, batch: &mut Batch, busy: &mut bool) -> Result<(), Stop> {
    let line = match entry {
        Entry::Dir { rel } => format!("D\t0\t{rel}\n"),
        Entry::File { rel, size, fd } => {
            if s.pump.busy() >= BUSY {

                batch.send(sock).map_err(|_| Stop::Gone)?;
                if !s.pump.wait_below(BUSY, PATIENCE, &s.dead) {
                    *busy = true;
                    return Err(Stop::Gone);
                }
            }
            let (read, write) = stream().map_err(|_| Stop::Gone)?;
            if size == 0 {
                drop(write);
            } else {
                s.pump.submit(fd, write, size);
            }
            batch.fds.push(read);
            format!("F\t{size}\t{rel}\n")
        }
    };
    batch.text.push_str(&line);
    batch.lines += 1;
    if batch.lines >= BATCH || batch.text.len() >= BODY_MAX {
        batch.send(sock).map_err(|_| Stop::Gone)?;
    }
    Ok(())
}

#[derive(Default)]
struct Batch {
    text: String,
    lines: usize,
    fds: Vec<OwnedFd>,
}

impl Batch {
    fn send(&mut self, sock: RawFd) -> io::Result<()> {
        if self.lines == 0 {
            return Ok(());
        }
        let body = format!("FILES\n{}", self.text);
        let raw: Vec<RawFd> = self.fds.iter().map(|f| f.as_raw_fd()).collect();
        let sent = send_message(sock, body.as_bytes(), &raw);

        *self = Batch::default();
        sent
    }
}

pub struct Guard {
    socket: PathBuf,
    serve: Arc<Serve>,
    row: PathBuf,
}

impl Guard {

    pub fn socket(&self) -> String {
        self.socket.to_string_lossy().into_owned()
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.serve.dead.store(true, Ordering::SeqCst);
        let _ = std::fs::remove_file(&self.socket);
        let _ = std::fs::remove_file(&self.row);
    }
}

fn listener(path: &Path) -> io::Result<OwnedFd> {
    // SAFETY: socket only makes a descriptor.
    let raw = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) };
    cvt(raw as isize)?;
    let fd = owned(raw);

    let (dir, name) = (path.parent().unwrap_or(Path::new("/")), path.file_name().unwrap_or_default());
    let held = std::fs::File::open(dir)?;
    let by_fd = format!("/proc/self/fd/{}/{}", held.as_raw_fd(), name.to_string_lossy());
    let used = if path.as_os_str().len() < 100 { path.to_string_lossy().into_owned() } else { by_fd };
    // SAFETY: an all-zero sockaddr_un is a valid empty one.
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    addr.sun_family = libc::AF_UNIX as _;
    let bytes = used.as_bytes();
    if bytes.len() >= addr.sun_path.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "the socket's name is too long"));
    }
    for (i, b) in bytes.iter().enumerate() {
        addr.sun_path[i] = *b as _;
    }
    // SAFETY: `addr` is a filled sockaddr_un of the size given and `fd` is a

    cvt(unsafe { libc::bind(fd.as_raw_fd(), &addr as *const _ as *const libc::sockaddr, std::mem::size_of::<libc::sockaddr_un>() as u32) } as isize)?;
    // SAFETY: listen on the bound socket.
    cvt(unsafe { libc::listen(fd.as_raw_fd(), 4) } as isize)?;
    Ok(fd)
}

fn from_me(fd: RawFd) -> bool {
    let mut cred = libc::ucred { pid: 0, uid: 0, gid: 0 };
    let mut len = std::mem::size_of::<libc::ucred>() as u32;
    // SAFETY: a ucred and its size, which live through the call.
    let r = unsafe { libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_PEERCRED, &mut cred as *mut _ as *mut libc::c_void, &mut len) };
    // SAFETY: getuid only reads this process's own user number.
    r == 0 && cred.uid == unsafe { libc::getuid() }
}

pub fn start(run: &Path, app_id: &str, at: &Path) -> Option<Guard> {
    let socket = run.join("copied");
    let _ = std::fs::remove_file(&socket);
    let lis = match listener(&socket) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("trindle-boxed: {app_id}: no way to hand over copied files: {e}");
            return None;
        }
    };
    let reader = Reader::start();
    let row = crate::common::in_use::files_path(std::process::id());
    let last = Arc::new(Mutex::new(None::<Instant>));
    let pump = Pump::new();
    let keep = at.to_path_buf();
    let who = app_id.to_string();
    let noted = last.clone();
    let serve = Arc::new(Serve {
        clipboard: Box::new(move || reader.ask(&copied::KINDS, copied::LIST_MAX + 1)),
        allowed: Box::new(move || crate::store::boxed::allowed_at(&keep, &["copied"]).iter().any(|a| a == "copied")),
        note: Box::new(move || *noted.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now())),
        roots: Roots::for_person(),
        limits: Limits::default(),
        gate: Mutex::new(Gate::new()),
        pump,
        dead: AtomicBool::new(false),
    });
    raise_open_files();
    let (s, path) = (serve.clone(), row.clone());
    std::thread::spawn(move || keep_row(&s, &last, &who, &path));
    let s = serve.clone();
    std::thread::spawn(move || accept_loop(lis, &s));
    Some(Guard { socket, serve, row })
}

fn raise_open_files() {
    let mut lim = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    // SAFETY: a rlimit that lives through the call.
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut lim) } == 0 && lim.rlim_cur < lim.rlim_max {
        lim.rlim_cur = lim.rlim_max.min(8192);
        // SAFETY: a rlimit that lives through the call.
        unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &lim) };
    }
}

fn accept_loop(lis: OwnedFd, s: &Arc<Serve>) {
    let live = Arc::new(AtomicUsize::new(0));
    loop {
        if s.dead.load(Ordering::SeqCst) {
            return;
        }
        if !(s.allowed)() {

            s.dead.store(true, Ordering::SeqCst);
            return;
        }
        let mut p = libc::pollfd { fd: lis.as_raw_fd(), events: libc::POLLIN, revents: 0 };
        // SAFETY: one pollfd, for a descriptor held open here.
        if unsafe { libc::poll(&mut p, 1, 500) } <= 0 {
            continue;
        }
        // SAFETY: accept4 on the listening socket; the peer's name is not wanted.
        let c = unsafe { libc::accept4(lis.as_raw_fd(), std::ptr::null_mut(), std::ptr::null_mut(), libc::SOCK_CLOEXEC) };
        if c < 0 {
            continue;
        }
        let conn = owned(c);
        if !from_me(conn.as_raw_fd()) || live.load(Ordering::SeqCst) >= CONNECTIONS {
            continue;
        }
        live.fetch_add(1, Ordering::SeqCst);
        let (s, live) = (s.clone(), live.clone());
        std::thread::spawn(move || {
            serve_connection(conn, &s);
            live.fetch_sub(1, Ordering::SeqCst);
        });
    }
}

fn keep_row(s: &Serve, last: &Mutex<Option<Instant>>, who: &str, file: &Path) {
    let mut shown = false;
    while !s.dead.load(Ordering::SeqCst) {
        if s.pump.busy() > 0 {
            *last.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
        }
        let seen = *last.lock().unwrap_or_else(|e| e.into_inner());
        let on = seen.is_some_and(|t| t.elapsed() < crate::common::in_use::FILES_SHOWN - Duration::from_secs(1));
        if on {
            let until = crate::common::in_use::now_ms() + crate::common::in_use::FILES_SHOWN.as_millis() as u64 - 500;
            let tmp = file.with_extension("new");
            if std::fs::write(&tmp, crate::common::in_use::encode_files(until, who)).is_ok() {
                let _ = std::fs::rename(&tmp, file);
            }
            shown = true;
        } else if shown {
            let _ = std::fs::remove_file(file);
            shown = false;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    let _ = std::fs::remove_file(file);
}

#[cfg(test)]
mod tests;
