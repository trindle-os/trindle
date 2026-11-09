//! `home-network`: a plugin allowed it reaches the devices of the person's
//! own network and nothing else. Not the internet, not this computer's own
//! services (its "localhost" and every address it owns), not the name
//! servers of the network.

pub mod relay;

use std::net::IpAddr;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use crate::toolkit::t;

pub const KEEPER: &str = "trindle-net-keeper";

pub const TABLE: &str = "trindle_box";

pub const HOME_V4: &[(&str, u8)] = &[
    ("10.0.0.0", 8),
    ("172.16.0.0", 12),
    ("192.168.0.0", 16),
    ("169.254.0.0", 16),

    ("224.0.0.0", 24),
    ("239.255.255.250", 32),

    ("255.255.255.255", 32),
];
pub const HOME_V6: &[(&str, u8)] = &[("fc00::", 7), ("fe80::", 10), ("ff02::", 16)];

pub const NO_NAMES: [u16; 2] = [53, 853];

fn in_range(ip: &IpAddr, (base, bits): (&str, u8)) -> bool {
    match (ip, base.parse::<IpAddr>()) {
        (IpAddr::V4(a), Ok(IpAddr::V4(b))) => {
            let mask = if bits == 0 { 0 } else { u32::MAX << (32 - u32::from(bits)) };
            u32::from(*a) & mask == u32::from(b) & mask
        }
        (IpAddr::V6(a), Ok(IpAddr::V6(b))) => {
            let mask = if bits == 0 { 0 } else { u128::MAX << (128 - u32::from(bits)) };
            u128::from(*a) & mask == u128::from(b) & mask
        }
        _ => false,
    }
}

pub fn home_address(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(_) => HOME_V4.iter().any(|r| in_range(ip, *r)),
        IpAddr::V6(_) => HOME_V6.iter().any(|r| in_range(ip, *r)),
    }
}

pub fn ruleset() -> String {
    let list = |r: &[(&str, u8)]| r.iter().map(|(a, b)| format!("{a}/{b}")).collect::<Vec<_>>().join(", ");
    let names = NO_NAMES.iter().map(u16::to_string).collect::<Vec<_>>().join(", ");
    format!(
        "table inet {TABLE} {{\n\
         \x20   set home4 {{ type ipv4_addr; flags interval; elements = {{ {v4} }} }}\n\
         \x20   set home6 {{ type ipv6_addr; flags interval; elements = {{ {v6} }} }}\n\
         \x20   chain out {{\n\
         \x20       type filter hook output priority 0; policy drop;\n\
         \x20       oifname \"lo\" accept\n\
         \x20       meta l4proto {{ tcp, udp }} th dport {{ {names} }} drop\n\
         \x20       ip daddr @home4 accept\n\
         \x20       ip6 daddr @home6 accept\n\
         \x20   }}\n\
         \x20   chain in {{\n\
         \x20       type filter hook input priority 0; policy drop;\n\
         \x20       iifname \"lo\" accept\n\
         \x20       ip saddr @home4 accept\n\
         \x20       ip6 saddr @home6 accept\n\
         \x20   }}\n\
         }}\n",
        v4 = list(HOME_V4),
        v6 = list(HOME_V6),
    )
}

pub fn default_interface_v4(text: &str) -> Option<String> {
    text.lines().skip(1).filter_map(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        let flags = u32::from_str_radix(f.get(3)?, 16).ok()?;
        let metric: u32 = f.get(6)?.parse().ok()?;
        (*f.get(1)? == "00000000" && *f.get(7)? == "00000000" && flags & 1 != 0).then(|| (metric, f[0].to_string()))
    }).min().map(|(_, i)| i)
}

pub fn default_interface_v6(text: &str) -> Option<String> {
    text.lines().filter_map(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        if f.len() < 10 {
            return None;
        }
        let flags = u32::from_str_radix(f[8], 16).ok()?;
        let metric = u32::from_str_radix(f[5], 16).ok()?;

        (f[0] == "0".repeat(32) && f[1] == "00" && flags & 1 != 0 && flags & 0x200 == 0 && f[9] != "lo").then(|| (metric, f[9].to_string()))
    }).min().map(|(_, i)| i)
}

fn plain_interface(n: &str) -> bool {
    !n.is_empty() && n.len() <= 15 && !n.starts_with('-') && n.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.' | b':'))
}

pub fn default_interfaces() -> (Option<String>, Option<String>) {
    let read = |p: &str| std::fs::read_to_string(p).unwrap_or_default();
    (
        default_interface_v4(&read("/proc/net/route")).filter(|i| plain_interface(i)),
        default_interface_v6(&read("/proc/net/ipv6_route")).filter(|i| plain_interface(i)),
    )
}

pub fn pasta_args(pid: u32, pid_file: &Path, if4: Option<&str>, if6: Option<&str>) -> Vec<String> {
    let mut a: Vec<String> = [
        "--quiet", "--config-net",

        "--no-map-gw",

        "-t", "none", "-T", "none", "-u", "none", "-U", "none",
        "--no-dhcp", "--no-dhcpv6", "--no-ra",
    ].map(String::from).into();
    match (if4, if6) {
        (Some(_), None) => a.push("--ipv4-only".into()),
        (None, Some(_)) => a.push("--ipv6-only".into()),
        _ => {}
    }

    if let Some(i) = if4 {
        a.extend(["--outbound-if4".into(), i.into()]);
    }
    if let Some(i) = if6 {
        a.extend(["--outbound-if6".into(), i.into()]);
    }
    a.extend(["-P".into(), pid_file.to_string_lossy().into_owned(), pid.to_string()]);
    a
}

pub struct Tools {
    pub unshare: PathBuf,
    pub pasta: PathBuf,

    pub keeper: Vec<String>,
}

fn program(name: &str) -> Option<PathBuf> {
    ["/usr/bin", "/usr/sbin", "/bin", "/sbin"].iter().map(|d| Path::new(d).join(name)).find(|p| p.is_file())
        .or_else(|| crate::common::run::find(name))
}

impl Tools {

    pub fn installed() -> Result<Tools, String> {

        let exe = crate::common::boxed_self::this_program().map_err(|_| t!("store-boxed-home-why-self").to_string())?;
        Ok(Tools {
            unshare: program("unshare").ok_or_else(|| t!("store-boxed-home-why-unshare").to_string())?,
            pasta: program("pasta").ok_or_else(|| t!("store-boxed-home-why-pasta").to_string())?,
            keeper: vec![exe.to_string_lossy().into_owned(), "net-keeper".into()],
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refused {

    Offline,
    Because(String),
}

fn pipe() -> std::io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [0 as RawFd; 2];
    // SAFETY: pipe2 writes two descriptors into the array it is given.
    if unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: both descriptors were just made and are owned by nobody else.
    Ok(unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) })
}

fn link() -> std::io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [0 as RawFd; 2];
    // SAFETY: socketpair writes two descriptors into the array it is given.
    if unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0, fds.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: both descriptors were just made and are owned by nobody else.
    Ok(unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) })
}

fn byte(fd: RawFd, most: Duration) -> Option<u8> {
    let mut p = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
    // SAFETY: poll on one descriptor we own.
    if unsafe { libc::poll(&mut p, 1, most.as_millis().min(i32::MAX as u128) as i32) } <= 0 {
        return None;
    }
    let mut b = 0u8;
    // SAFETY: reads at most one byte into b.
    (unsafe { libc::read(fd, (&mut b as *mut u8).cast(), 1) } == 1).then_some(b)
}

fn put(fd: RawFd, bytes: &[u8]) -> bool {
    let mut at = 0;
    while at < bytes.len() {
        // SAFETY: writes from a slice that is alive for the call.
        let n = unsafe { libc::write(fd, bytes[at..].as_ptr().cast(), bytes.len() - at) };
        if n < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
            continue;
        }
        if n <= 0 {
            return false;
        }
        at += n as usize;
    }
    true
}

const READY: u8 = b'R';
const STARTED: u8 = b'S';

const MAKE: Duration = Duration::from_secs(10);

const COMMAND_MAX: u64 = 1 << 20;

fn shares_ours(pid: u32) -> bool {
    ["user", "net"].iter().any(|k| {
        let ours = std::fs::read_link(format!("/proc/self/ns/{k}"));
        let theirs = std::fs::read_link(format!("/proc/{pid}/ns/{k}"));
        !matches!((ours, theirs), (Ok(a), Ok(b)) if a != b)
    })
}

pub struct Room {
    keeper: Child,
    commands: Option<OwnedFd>,
    status: OwnedFd,
    pasta_pid: PathBuf,

    ids: (u32, u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    pub code: i32,

    pub started: bool,
}

impl Room {

    pub fn prepare(tools: &Tools, run: &Path) -> Result<Room, Refused> {
        let (if4, if6) = default_interfaces();
        if if4.is_none() && if6.is_none() {
            return Err(Refused::Offline);
        }
        Room::prepare_on(tools, run, if4.as_deref(), if6.as_deref())
    }

    pub fn prepare_on(tools: &Tools, run: &Path, if4: Option<&str>, if6: Option<&str>) -> Result<Room, Refused> {
        if !tools.unshare.is_file() {
            return Err(Refused::Because(t!("store-boxed-home-why-unshare").to_string()));
        }
        if !tools.pasta.is_file() {
            return Err(Refused::Because(t!("store-boxed-home-why-pasta").to_string()));
        }
        let Started { keeper, command_w, status_r, relay_here } = start_keeper(tools)?;
        let ready = byte(status_r.as_raw_fd(), MAKE) == Some(READY);
        let fail = |mut keeper: Child, why: String| {
            let _ = keeper.kill();
            let _ = keeper.wait();
            Err(Refused::Because(why))
        };
        if !ready {
            return fail(keeper, t!("store-boxed-home-why-not-made").to_string());
        }

        if shares_ours(keeper.id()) {
            return fail(keeper, t!("store-boxed-home-why-not-new").to_string());
        }
        let pid_file = match join_pasta(tools, run, keeper.id(), if4, if6) {
            Ok(file) => file,
            Err(why) => return fail(keeper, why),
        };

        match if4 {
            Some(i) => {
                let interface = i.to_string();
                std::thread::spawn(move || relay::host(relay_here, Some(&interface)));
            }
            None => drop(relay_here),
        }
        // SAFETY: getuid and getgid only read.
        let ids = unsafe { (libc::getuid(), libc::getgid()) };
        Ok(Room { keeper, commands: Some(command_w), status: status_r, pasta_pid: pid_file, ids })
    }

    pub fn ids(&self) -> (u32, u32) {
        self.ids
    }

    pub fn run(mut self, argv: &[String]) -> Outcome {
        let frame = argv.join("\0");
        let sent = self.commands.take().is_some_and(|w| put(w.as_raw_fd(), frame.as_bytes()));
        if !sent {
            return Outcome { code: 1, started: false };
        }
        let code = match self.keeper.wait() {
            Ok(s) => s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0)),
            Err(_) => 1,
        };

        let mut started = false;
        while let Some(b) = byte(self.status.as_raw_fd(), Duration::ZERO) {
            started |= b == STARTED;
        }
        Outcome { code, started }
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        let _ = self.keeper.kill();
        let _ = self.keeper.wait();

        if let Some(pid) = std::fs::read_to_string(&self.pasta_pid).ok().and_then(|t| t.trim().parse::<i32>().ok()).filter(|p| *p > 1) {
            let comm = std::fs::read_to_string(format!("/proc/{pid}/comm")).unwrap_or_default();
            if comm.starts_with("pasta") {
                // SAFETY: a signal to a process just checked to be pasta.
                unsafe { libc::kill(pid, libc::SIGTERM) };
            }
        }
        let _ = std::fs::remove_file(&self.pasta_pid);
    }
}

fn loopback_up() -> std::io::Result<()> {
    // SAFETY: an ordinary datagram socket, closed below.
    let s = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if s < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: ifreq is plain data; all zero is a valid empty one.
    let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };
    ifr.ifr_name[0] = b'l' as libc::c_char;
    ifr.ifr_name[1] = b'o' as libc::c_char;
    // SAFETY: the two ioctls read and write the one ifreq given.
    let result = unsafe {
        if libc::ioctl(s, libc::SIOCGIFFLAGS, &mut ifr) != 0 {
            Err(std::io::Error::last_os_error())
        } else {
            ifr.ifr_ifru.ifru_flags |= libc::IFF_UP as libc::c_short;
            if libc::ioctl(s, libc::SIOCSIFFLAGS, &ifr) != 0 { Err(std::io::Error::last_os_error()) } else { Ok(()) }
        }
    };
    // SAFETY: closes the socket made above.
    unsafe { libc::close(s) };
    result
}

fn load_rules() -> Result<(), String> {
    use std::io::Write;
    let nft = program("nft").ok_or("nft is not installed")?;
    let mut c = Command::new(nft).args(["-f", "-"]).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped()).spawn()
        .map_err(|e| format!("nft would not start: {e}"))?;
    c.stdin.take().ok_or("nft took nothing")?.write_all(ruleset().as_bytes()).map_err(|e| e.to_string())?;
    let out = c.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).lines().next().unwrap_or("nft refused the rules").to_string())
    }
}

struct Started {
    keeper: Child,
    command_w: OwnedFd,
    status_r: OwnedFd,
    relay_here: OwnedFd,
}

fn start_keeper(tools: &Tools) -> Result<Started, Refused> {
    let sorry = |e: &str| Refused::Because(e.to_string());
    let (command_r, command_w) = pipe().map_err(|e| sorry(&e.to_string()))?;
    let (status_r, status_w) = pipe().map_err(|e| sorry(&e.to_string()))?;
    let (relay_here, relay_there) = link().map_err(|e| sorry(&e.to_string()))?;
    let (cr, sw, rl) = (command_r.as_raw_fd(), status_w.as_raw_fd(), relay_there.as_raw_fd());
    let mut c = Command::new(&tools.unshare);
    c.args(["--user", "--map-root-user", "--net", "--"]).args(&tools.keeper).arg(cr.to_string()).arg(sw.to_string()).arg(rl.to_string());
    // SAFETY: only fcntl runs between fork and exec; it is async-signal-safe.
    unsafe {
        c.pre_exec(move || {
            for fd in [cr, sw, rl] {
                if libc::fcntl(fd, libc::F_SETFD, 0) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
    let keeper = c.spawn().map_err(|e| Refused::Because(t!("store-boxed-home-why-would-not-start", program = tools.unshare.display().to_string(), why = e.to_string())))?;
    drop(command_r);
    drop(status_w);
    drop(relay_there);
    Ok(Started { keeper, command_w, status_r, relay_here })
}

fn join_pasta(tools: &Tools, run: &Path, pid: u32, if4: Option<&str>, if6: Option<&str>) -> Result<PathBuf, String> {
    std::fs::create_dir_all(run).ok();
    let pid_file = run.join("pasta.pid");
    let _ = std::fs::remove_file(&pid_file);
    let mut p = Command::new(&tools.pasta);
    p.args(pasta_args(pid, &pid_file, if4, if6)).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped());
    // SAFETY: close_range is one system call, async-signal-safe; pasta is given none of this program's other descriptors.
    unsafe {
        p.pre_exec(|| {
            libc::syscall(libc::SYS_close_range, 3u32, u32::MAX, 0u32);
            Ok(())
        });
    }
    let mut pasta = p.spawn().map_err(|e| t!("store-boxed-home-why-would-not-start", program = tools.pasta.display().to_string(), why = e.to_string()))?;
    let began = Instant::now();
    let said = loop {
        match pasta.try_wait() {
            Ok(Some(s)) => break Some(s),
            Ok(None) if began.elapsed() < MAKE => std::thread::sleep(Duration::from_millis(5)),
            _ => break None,
        }
    };
    match said {
        Some(s) if s.success() => Ok(pid_file),
        other => {
            let mut why = String::new();
            if other.is_none() {
                let _ = pasta.kill();
                let _ = pasta.wait();
            }
            if let Some(mut e) = pasta.stderr.take() {
                use std::io::Read;
                let _ = e.by_ref().take(400).read_to_string(&mut why);
            }
            let why = why.lines().next().unwrap_or("").trim().to_string();
            Err(if why.is_empty() { t!("store-boxed-home-why-pasta-no-join").to_string() } else { t!("store-boxed-home-why-pasta-said", why = why) })
        }
    }
}

fn read_command(commands: RawFd) -> Option<Vec<String>> {
    use std::io::Read;
    // SAFETY: the descriptor was handed to this program for this and is not used elsewhere.
    let mut source = std::fs::File::from(unsafe { OwnedFd::from_raw_fd(commands) });
    let mut frame = Vec::new();
    if source.by_ref().take(COMMAND_MAX).read_to_end(&mut frame).is_err() || frame.is_empty() {
        return None;
    }
    drop(source);
    let argv: Vec<String> = String::from_utf8_lossy(&frame).split('\0').map(String::from).collect();
    if argv.first().is_none_or(|a| a.is_empty()) {
        return None;
    }
    Some(argv)
}

pub fn keeper_main(args: &[String]) -> i32 {
    let fd = |i: usize| args.get(i).and_then(|a| a.parse::<RawFd>().ok()).filter(|f| *f > 2);
    let (Some(commands), Some(status), link_fd) = (fd(1), fd(2), fd(3)) else {
        eprintln!("trindle net-keeper: it is started by Trindle, with its descriptors");
        return 125;
    };
    // SAFETY: the box dies with its keeper; this only sets that, and closes the keeper's own descriptors on exec.
    unsafe {
        libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL);
        libc::fcntl(commands, libc::F_SETFD, libc::FD_CLOEXEC);
        libc::fcntl(status, libc::F_SETFD, libc::FD_CLOEXEC);
        if let Some(r) = link_fd {
            libc::fcntl(r, libc::F_SETFD, libc::FD_CLOEXEC);
        }
    }
    // SAFETY: getuid only reads.
    if unsafe { libc::getuid() } != 0 {
        eprintln!("trindle net-keeper: it runs only in a user namespace of its own");
        return 125;
    }
    if !put(status, &[READY]) {
        return 125;
    }
    let Some(argv) = read_command(commands) else { return 125 };
    if let Err(e) = loopback_up() {
        eprintln!("trindle net-keeper: the room's loopback would not come up: {e}");
        return 125;
    }
    if let Err(e) = load_rules() {
        eprintln!("trindle net-keeper: the room's rules were not loaded: {e}");
        return 125;
    }

    if let Some(r) = link_fd {
        // SAFETY: the descriptor was handed to this program for this and is not used elsewhere.
        if let Err(e) = relay::room(unsafe { OwnedFd::from_raw_fd(r) }) {
            eprintln!("trindle net-keeper: answers to a search for devices are off: {e}");
        }
    }
    let mut child = Command::new(&argv[0]);
    child.args(&argv[1..]);
    put(status, &[STARTED]);
    // SAFETY: the status descriptor is ours to close.
    unsafe { libc::close(status) };
    match child.spawn().and_then(|mut c| c.wait()) {
        Ok(s) => s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0)),
        Err(e) => {
            eprintln!("trindle net-keeper: could not start {}: {e}", argv[0]);
            127
        }
    }
}

#[cfg(test)]
mod tests;
