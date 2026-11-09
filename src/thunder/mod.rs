//! trindle-thunder: asks before a Thunderbolt or USB4 device connects.

pub mod allowed;
pub mod ask;
pub mod sysfs;

use crate::toolkit::t;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use allowed::Allowed;
use sysfs::{Device, Domain, How, Level};

pub const SOCKET: &str = "/run/trindle-thunder/socket";

const CLIENTS: usize = 16;

const LINE: usize = 512;

#[derive(Clone, Debug)]
pub struct Places {
    pub sys: PathBuf,
    pub state: PathBuf,
    pub socket: PathBuf,

    pub seat: PathBuf,

    pub run_user: PathBuf,
}

impl Places {
    pub fn system() -> Places {
        Places {
            sys: "/sys".into(),
            state: "/var/lib/trindle-thunder".into(),
            socket: SOCKET.into(),
            seat: "/run/systemd/seats/seat0".into(),
            run_user: "/run/user".into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Always,
    Once,
    Never,
}

impl Choice {
    pub fn parse(s: &str) -> Option<Choice> {
        match s {
            "always" => Some(Choice::Always),
            "once" => Some(Choice::Once),
            "never" => Some(Choice::Never),
            _ => None,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Choice::Always => "always",
            Choice::Once => "once",
            Choice::Never => "never",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Question {
    pub id: String,
    pub vendor: String,
    pub name: String,
    pub level: Level,
    pub iommu: bool,

    pub doubt: bool,
}

impl Question {
    pub fn line(&self) -> String {
        format!("ask\t{}\t{}\t{}\t{}\t{}\t{}", self.id, self.vendor, self.name, self.level.word(), u8::from(self.iommu),
                u8::from(self.doubt))
    }

    pub fn parse(line: &str) -> Option<Question> {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() != 7 || f[0] != "ask" || !sysfs::sane_id(f[1]) {
            return None;
        }
        Some(Question {
            id: f[1].into(), vendor: sysfs::clean(f[2]), name: sysfs::clean(f[3]), level: Level::parse(f[4]),
            iommu: f[5] == "1", doubt: f[6] == "1",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Ok,

    Locked,

    Unknown,
    Failed(String),
}

impl Outcome {
    fn words(&self) -> String {
        match self {
            Outcome::Ok => "ok".into(),
            Outcome::Locked => "locked".into(),
            Outcome::Unknown => "unknown".into(),
            Outcome::Failed(w) => format!("failed\t{}", sysfs::clean(w)),
        }
    }
}

pub struct Guard {
    places: Places,
    allowed: Vec<Allowed>,
    waiting: Vec<(Question, Device)>,

    refused: HashSet<String>,

    doubted: HashSet<String>,

    pub news: Vec<String>,
}

impl Guard {
    pub fn new(places: Places) -> Guard {
        let allowed = allowed::load(&places.state);
        Guard { places, allowed, waiting: Vec::new(), refused: HashSet::new(), doubted: HashSet::new(), news: Vec::new() }
    }

    #[cfg(test)]
    pub fn waiting(&self) -> Vec<Question> {
        self.waiting.iter().map(|(q, _)| q.clone()).collect()
    }

    #[cfg(test)]
    pub fn allowed(&self) -> &[Allowed] {
        &self.allowed
    }

    pub fn rescan(&mut self) {
        for _ in 0..8 {
            if !self.pass() {
                break;
            }
        }
    }

    fn pass(&mut self) -> bool {
        let domains = sysfs::domains(&self.places.sys);
        let devices = sysfs::devices(&self.places.sys);
        let present: HashSet<&str> = devices.iter().map(|d| d.unique_id.as_str()).collect();
        self.refused.retain(|id| present.contains(id.as_str()));
        self.doubted.retain(|id| present.contains(id.as_str()));

        let mut gone = Vec::new();
        self.waiting.retain(|(q, dev)| {
            let still = devices.iter().any(|d| d.unique_id == q.id && d.kernel == dev.kernel && d.authorized == 0);
            if !still {
                gone.push(q.id.clone());
            }
            still
        });
        self.news.extend(gone.into_iter().map(|id| format!("gone\t{id}")));

        let mut let_in = false;
        for dev in devices {
            if dev.authorized != 0 || !dev.reachable || !sysfs::sane_id(&dev.unique_id) {
                continue;
            }
            let (level, iommu) = domains.iter().find(|d| d.number == dev.domain).map_or((Level::Unknown, false), |d| (d.level, d.iommu));
            if !level.asks() || self.refused.contains(&dev.unique_id) || self.waiting.iter().any(|(q, _)| q.id == dev.unique_id) {
                continue;
            }
            let known = self.allowed.iter().position(|a| a.id == dev.unique_id);
            if let (Some(i), false) = (known, self.doubted.contains(&dev.unique_id)) {
                match self.let_in_known(i, &dev) {
                    Ok(()) => {
                        eprintln!("trindle-thunder: let in {} ({}), allowed always", dev.kernel, dev.unique_id);
                        let_in = true;
                        continue;
                    }
                    Err(e) => {
                        eprintln!("trindle-thunder: {} ({}) was allowed always, but: {e}", dev.kernel, dev.unique_id);
                        self.doubted.insert(dev.unique_id.clone());
                    }
                }
            }
            let q = Question {
                id: dev.unique_id.clone(), vendor: dev.vendor.clone(), name: dev.name.clone(), level, iommu,
                doubt: self.doubted.contains(&dev.unique_id),
            };
            eprintln!("trindle-thunder: {} ({}) waits for an answer", dev.kernel, dev.unique_id);
            self.news.push(q.line());
            self.waiting.push((q, dev));
        }
        let_in
    }

    fn let_in_known(&mut self, i: usize, dev: &Device) -> io::Result<()> {
        let how = match (&self.allowed[i].key, dev.has_key) {
            (Some(k), true) => How::Key(k.clone()),
            (None, true) => How::NewKey(sysfs::new_key()?),
            (_, false) => How::Plain,
        };
        sysfs::authorize(dev, &how)?;
        if let How::NewKey(k) = how {
            self.allowed[i].key = Some(k);
            self.keep();
        }
        Ok(())
    }

    fn keep(&self) {
        if let Err(e) = allowed::save(&self.places.state, &self.allowed) {
            eprintln!("trindle-thunder: the allowed list could not be saved: {e}");
        }
    }

    pub fn answer(&mut self, id: &str, choice: Choice, locked: bool) -> Outcome {
        let Some(pos) = self.waiting.iter().position(|(q, _)| q.id == id) else { return Outcome::Unknown };
        if locked && choice != Choice::Never {
            return Outcome::Locked;
        }
        let (q, dev) = self.waiting[pos].clone();
        let done = match choice {
            Choice::Never => {
                self.refused.insert(q.id.clone());
                Ok(())
            }
            Choice::Once => sysfs::authorize(&dev, &How::Plain),
            Choice::Always => {
                let how = if dev.has_key { sysfs::new_key().map(How::NewKey) } else { Ok(How::Plain) };
                how.and_then(|how| sysfs::authorize(&dev, &how).map(|()| how)).map(|how| {
                    self.allowed.retain(|a| a.id != q.id);
                    let key = match how {
                        How::NewKey(k) => Some(k),
                        _ => None,
                    };
                    self.allowed.push(Allowed { id: q.id.clone(), key, vendor: q.vendor.clone(), name: q.name.clone() });
                    self.doubted.remove(&q.id);
                    self.keep();
                })
            }
        };
        if let Err(e) = done {
            eprintln!("trindle-thunder: {} ({}) could not be let in: {e}", dev.kernel, q.id);
            return Outcome::Failed(t!("thunder-unplug").into());
        }
        eprintln!("trindle-thunder: {} ({}) answered {}", dev.kernel, q.id, choice.word());
        self.waiting.remove(pos);
        self.news.push(format!("gone\t{}", q.id));
        self.rescan();
        Outcome::Ok
    }

    pub fn forget(&mut self, id: &str) -> bool {
        let before = self.allowed.len();
        self.allowed.retain(|a| a.id != id);
        let gone = self.allowed.len() != before;
        if gone {
            self.keep();
        }
        gone
    }

    pub fn list(&self) -> Vec<String> {
        let mut out: Vec<String> = sysfs::domains(&self.places.sys).iter()
            .map(|d| format!("level\t{}\t{}\t{}", d.number, d.level.word(), u8::from(d.iommu))).collect();
        out.extend(self.allowed.iter().map(|a| format!("device\t{}\t{}\t{}", a.id, a.vendor, a.name)));
        out.push("end".into());
        out
    }

    pub fn locked(&self) -> bool {
        match active_uid(&self.places.seat) {
            Some(uid) => crate::common::locked::of(&self.places.run_user, uid),
            None => true,
        }
    }

    fn may(&self, uid: Option<u32>) -> bool {
        match uid {
            Some(0) => true,
            Some(u) => active_uid(&self.places.seat) == Some(u),
            None => false,
        }
    }

    fn request(&mut self, uid: Option<u32>, line: &str) -> (Vec<String>, bool) {
        if !self.may(uid) {
            return (vec!["denied".into()], false);
        }
        let f: Vec<&str> = line.split('\t').collect();
        match f.as_slice() {
            ["watch"] => (self.waiting.iter().map(|(q, _)| q.line()).collect(), true),
            ["answer", id, choice] => match Choice::parse(choice) {
                Some(c) => {
                    let locked = self.locked();
                    let out = self.answer(id, c, locked);
                    (vec![format!("answered\t{id}\t{}", out.words())], false)
                }
                None => (vec!["what".into()], false),
            },
            ["list"] => (self.list(), false),
            ["forget", id] => (vec![if self.forget(id) { "ok" } else { "unknown" }.into()], false),
            _ => (vec!["what".into()], false),
        }
    }
}

pub fn active_uid(seat: &Path) -> Option<u32> {
    fs::read_to_string(seat).ok()?.lines().find_map(|l| l.strip_prefix("ACTIVE_UID=")).and_then(|u| u.trim().parse().ok())
}

fn peer_uid(fd: RawFd) -> Option<u32> {
    // SAFETY: ucred is plain numbers; all zero is a valid value.
    let mut cred: libc::ucred = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: cred and len are valid for writing and sized for SO_PEERCRED.
    let r = unsafe {
        libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_PEERCRED, (&mut cred as *mut libc::ucred).cast(), &mut len)
    };
    (r == 0).then_some(cred.uid)
}

fn uevents() -> io::Result<OwnedFd> {
    // SAFETY: no pointers; the result is checked before it is owned.
    let fd = unsafe {
        libc::socket(libc::AF_NETLINK, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK, libc::NETLINK_KOBJECT_UEVENT)
    };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: fd is a fresh descriptor nothing else owns.
    let fd = unsafe { OwnedFd::from_raw_fd(fd) };
    // SAFETY: sockaddr_nl is plain numbers; all zero is a valid start.
    let mut addr: libc::sockaddr_nl = unsafe { std::mem::zeroed() };
    addr.nl_family = libc::AF_NETLINK as libc::sa_family_t;

    addr.nl_groups = 1;
    // SAFETY: addr is a whole sockaddr_nl that lives through the call.
    let r = unsafe {
        libc::bind(fd.as_raw_fd(), (&addr as *const libc::sockaddr_nl).cast(), std::mem::size_of::<libc::sockaddr_nl>() as libc::socklen_t)
    };
    if r < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

fn heard_thunderbolt(fd: RawFd) -> bool {
    let mut any = false;
    let mut buf = [0u8; 8192];
    loop {
        // SAFETY: buf is writable for its whole length.
        let n = unsafe { libc::recv(fd, buf.as_mut_ptr().cast(), buf.len(), 0) };
        if n <= 0 {
            break;
        }
        let msg = &buf[..n as usize];
        any |= msg.windows(21).any(|w| w == b"SUBSYSTEM=thunderbolt");
    }
    any
}

struct Client {
    stream: UnixStream,
    uid: Option<u32>,
    got: Vec<u8>,
    watching: bool,
}

impl Client {
    fn send(&mut self, lines: &[String]) -> bool {
        let mut text = String::new();
        for l in lines {
            text.push_str(l);
            text.push('\n');
        }
        self.stream.write_all(text.as_bytes()).is_ok()
    }
}

pub struct Opts {

    pub uevents: bool,

    pub every: Option<Duration>,

    pub stop: Option<Arc<AtomicBool>>,
}

pub fn serve(places: Places, opts: Opts) -> io::Result<()> {
    let listener = bind(&places)?;
    let news = if opts.uevents {
        uevents().map_err(|e| eprintln!("trindle-thunder: cannot hear the kernel, so looking every two seconds: {e}")).ok()
    } else {
        None
    };
    let every = match (&news, opts.every) {
        (None, None) => Some(Duration::from_secs(2)),
        (_, e) => e,
    };
    let wake = match (every, &opts.stop) {
        (Some(e), Some(_)) => Some(e.min(Duration::from_millis(100))),
        (None, Some(_)) => Some(Duration::from_millis(100)),
        (e, None) => e,
    };
    let mut guard = Guard::new(places);
    guard.rescan();
    let mut clients: Vec<Client> = Vec::new();
    let mut last = std::time::Instant::now();
    loop {
        if opts.stop.as_ref().is_some_and(|s| s.load(Ordering::Relaxed)) {
            return Ok(());
        }
        if !guard.news.is_empty() {
            let lines: Vec<String> = std::mem::take(&mut guard.news);
            clients.retain_mut(|c| !c.watching || c.send(&lines));
        }
        let Some((fds, first_client)) = wait(&listener, news.as_ref(), &clients, wake)? else { continue };
        if every.is_some_and(|e| last.elapsed() >= e) {
            last = std::time::Instant::now();
            guard.rescan();
        }
        if let Some(n) = &news {
            if fds[1].revents != 0 && heard_thunderbolt(n.as_raw_fd()) {
                guard.rescan();
            }
        }
        talk(&mut guard, &mut clients, &fds, first_client);
        if fds[0].revents != 0 {
            accept(&listener, &mut clients);
        }
    }
}

fn bind(places: &Places) -> io::Result<UnixListener> {
    if let Some(dir) = places.socket.parent() {
        fs::create_dir_all(dir)?;
    }
    let _ = fs::remove_file(&places.socket);
    let listener = UnixListener::bind(&places.socket)?;

    fs::set_permissions(&places.socket, fs::Permissions::from_mode(0o666))?;
    listener.set_nonblocking(true)?;
    Ok(listener)
}

fn wait(listener: &UnixListener, news: Option<&OwnedFd>, clients: &[Client], wake: Option<Duration>) -> io::Result<Option<(Vec<libc::pollfd>, usize)>> {
    let mut fds = vec![libc::pollfd { fd: listener.as_raw_fd(), events: libc::POLLIN, revents: 0 }];
    if let Some(n) = news {
        fds.push(libc::pollfd { fd: n.as_raw_fd(), events: libc::POLLIN, revents: 0 });
    }
    let first_client = fds.len();
    fds.extend(clients.iter().map(|c| libc::pollfd { fd: c.stream.as_raw_fd(), events: libc::POLLIN, revents: 0 }));
    let timeout = wake.map_or(-1, |w| w.as_millis().min(i32::MAX as u128) as i32);
    // SAFETY: fds is a live array of pollfd of the length given.
    let r = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, timeout) };
    if r < 0 {
        let e = io::Error::last_os_error();
        if e.kind() == io::ErrorKind::Interrupted {
            return Ok(None);
        }
        return Err(e);
    }
    Ok(Some((fds, first_client)))
}

fn talk(guard: &mut Guard, clients: &mut Vec<Client>, fds: &[libc::pollfd], first_client: usize) {
    let mut drop = vec![false; clients.len()];
    for (i, c) in clients.iter_mut().enumerate() {
        let revents = fds[first_client + i].revents;
        if revents == 0 {
            continue;
        }
        let mut buf = [0u8; 1024];
        match c.stream.read(&mut buf) {
            Ok(0) => drop[i] = true,
            Ok(n) => c.got.extend_from_slice(&buf[..n]),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => drop[i] = true,
        }
        while let Some(end) = c.got.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = c.got.drain(..=end).collect();
            let line = String::from_utf8_lossy(&line[..end]).to_string();
            let (reply, watch) = guard.request(c.uid, &line);
            c.watching |= watch;
            if !c.send(&reply) {
                drop[i] = true;
                break;
            }
        }
        if c.got.len() > LINE {
            drop[i] = true;
        }
    }
    let mut i = 0;
    clients.retain(|_| {
        i += 1;
        !drop[i - 1]
    });
}

fn accept(listener: &UnixListener, clients: &mut Vec<Client>) {
    while let Ok((stream, _)) = listener.accept() {
        if clients.len() >= CLIENTS || stream.set_nonblocking(true).is_err() {
            continue;
        }
        let uid = peer_uid(stream.as_raw_fd());
        clients.push(Client { stream, uid, got: Vec::new(), watching: false });
    }
}

pub fn socket_path() -> PathBuf {
    std::env::var_os("TRINDLE_THUNDER_SOCKET").filter(|s| !s.is_empty()).map(PathBuf::from).unwrap_or_else(|| SOCKET.into())
}

pub fn ask_service(line: &str) -> io::Result<Vec<String>> {
    let mut s = UnixStream::connect(socket_path())?;
    s.set_read_timeout(Some(Duration::from_secs(20)))?;
    s.write_all(format!("{line}\n").as_bytes())?;
    let many = line == "list";
    let mut out = Vec::new();
    for l in BufReader::new(s).lines() {
        let l = l?;
        if l == "denied" {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "only the person at the screen may ask"));
        }
        if !many || l == "end" {
            if !many {
                out.push(l);
            }
            return Ok(out);
        }
        out.push(l);
    }
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "the service stopped answering"))
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Listing {
    pub domains: Vec<Domain>,

    pub devices: Vec<(String, String, String)>,
}

pub fn parse_list(lines: &[String]) -> Listing {
    let mut l = Listing::default();
    for line in lines {
        let f: Vec<&str> = line.split('\t').collect();
        match f.as_slice() {
            ["level", n, level, iommu] => l.domains.push(Domain { number: n.to_string(), level: Level::parse(level), iommu: *iommu == "1" }),
            ["device", id, vendor, name] => l.devices.push((id.to_string(), sysfs::clean(vendor), sysfs::clean(name))),
            _ => {}
        }
    }
    l
}

pub fn listing() -> (Listing, Option<String>) {
    match ask_service("list") {
        Ok(lines) => (parse_list(&lines), None),
        Err(e) => {
            let domains = sysfs::domains(Path::new("/sys"));
            (Listing { domains, devices: Vec::new() }, Some(e.to_string()))
        }
    }
}

pub fn forget(id: &str) -> io::Result<()> {
    match ask_service(&format!("forget\t{id}"))?.first().map(String::as_str) {
        Some("ok") | Some("unknown") => Ok(()),
        other => Err(io::Error::other(format!("the service said {other:?}"))),
    }
}

pub fn main(args: Vec<String>) -> i32 {
    let mut places = Places::system();
    let mut opts = Opts { uevents: true, every: None, stop: None };
    let mut it = args.iter().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--ask" => return ask::main(),
            "--list" => {
                return match ask_service("list") {
                    Ok(lines) => {
                        lines.iter().for_each(|l| println!("{l}"));
                        0
                    }
                    Err(e) => {
                        eprintln!("trindle-thunder: {e}");
                        1
                    }
                };
            }
            "--sys" | "--state" | "--socket" | "--seat" | "--run-user" => {
                let Some(v) = it.next() else {
                    eprintln!("trindle-thunder: {a} needs a path");
                    return 2;
                };
                let p = PathBuf::from(v);
                match a.as_str() {
                    "--sys" => places.sys = p,
                    "--state" => places.state = p,
                    "--socket" => places.socket = p,
                    "--seat" => places.seat = p,
                    _ => places.run_user = p,
                }
            }
            "--no-uevents" => opts.uevents = false,
            _ => {
                eprintln!("trindle-thunder: [--ask | --list | --sys DIR --state DIR --socket PATH --seat FILE --run-user DIR --no-uevents]");
                return 2;
            }
        }
    }
    match serve(places, opts) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-thunder: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod language_tests {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    use crate::toolkit::t;

    #[test]
    fn the_thunderbolt_words_follow_the_language_and_fall_back_word_by_word() {
        let french = "thunder-no-name = Un appareil\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(t!("thunder-no-name"), "Un appareil");
            assert!(t!("thunder-why").starts_with("Thunderbolt"));
        });
    }
}
