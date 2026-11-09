//! trindle-dns: the computer's own name server, run by systemd while
//! blocking is on. It answers on 127.0.2.53, blocks the sites on the
//! HaGeZi lists, and asks the chosen private server over TLS, falling back
//! to the network's own servers while that cannot be reached. Names on the
//! home network, on a VPN and on Tailscale go where they always went.

mod cache;
pub(crate) mod list;
mod network;
mod resolved;
mod tailnet;
mod upstream;
mod wire;

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use cache::Cache;
use list::List;
use network::Network;
use tailnet::Split;
use upstream::Private;
use wire::Question;

use crate::common::dns::{self as shared, Provider, Status};

const UDP_THREADS: usize = 12;
const TCP_MOST: usize = 32;
const STACK: usize = 1024 * 1024;
const TCP_WAIT: Duration = Duration::from_secs(10);
const RECENT: usize = 50;
const TAILSCALE: &str = "/sys/class/net/tailscale0";
const INTERFACES: &str = "/sys/class/net";
const QUAD100: IpAddr = IpAddr::V4(Ipv4Addr::new(100, 100, 100, 100));

const STALE: u64 = 86_400;
const CHECK: Duration = Duration::from_secs(600);
const RETRY: Duration = Duration::from_secs(120);

const BIND_AGAIN: Duration = Duration::from_secs(5);

const CANARY: &str = "use-application-dns.net";

const PORTAL_CHECKS: [&str; 9] = [
    "network-test.debian.org", "nmcheck.gnome.org", "connectivity-check.ubuntu.com", "networkcheck.kde.org",
    "detectportal.firefox.com", "connectivitycheck.gstatic.com", "clients3.google.com", "captive.apple.com",
    "www.msftconnecttest.com",
];

pub const LOCAL: [&str; 5] = ["lan", "home.arpa", "internal", "localdomain", "local"];

const SPLIT_AGAIN: Duration = Duration::from_secs(60);

static RELOAD: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
struct Stats {
    today: u64,
    day: i64,

    recent: VecDeque<String>,
}

type Prefix = ([u8; 16], u8);

struct Shared {
    list: RwLock<Option<Arc<List>>>,
    allow: RwLock<Vec<String>>,
    cache: Mutex<Cache>,
    private: RwLock<Arc<Private>>,
    network: RwLock<Arc<Network>>,

    vpn: RwLock<Vec<SocketAddr>>,

    search: RwLock<Vec<String>>,

    split: RwLock<Vec<Split>>,

    nat64: RwLock<Option<Prefix>>,

    moves: AtomicU64,
    stats: Mutex<Stats>,
    tailscale_up: AtomicBool,

    asked: AtomicBool,
    fetching: AtomicBool,
}

impl Shared {
    fn new(list: Option<Arc<List>>, allow: Vec<String>, provider: Provider) -> Shared {
        Shared {
            list: RwLock::new(list),
            allow: RwLock::new(allow),
            cache: Mutex::new(Cache::default()),
            private: RwLock::new(Arc::new(Private::new(provider))),
            network: RwLock::new(Arc::new(Network::default())),
            vpn: RwLock::new(Vec::new()),
            search: RwLock::new(Vec::new()),
            split: RwLock::new(Vec::new()),
            nat64: RwLock::new(None),
            moves: AtomicU64::new(0),
            stats: Mutex::new(Stats::default()),
            tailscale_up: AtomicBool::new(Path::new(TAILSCALE).exists()),
            asked: AtomicBool::new(false),
            fetching: AtomicBool::new(false),
        }
    }
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

fn read<T>(l: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    l.read().unwrap_or_else(|p| p.into_inner())
}

fn write<T>(l: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    l.write().unwrap_or_else(|p| p.into_inner())
}

pub fn main(args: Vec<String>) -> i32 {
    match args.get(1).map(String::as_str) {
        Some("--refresh") => {
            return match list::refresh(&list::folder()) {
                Ok(n) => {
                    println!("{n} sites listed");
                    0
                }
                Err(e) => {
                    eprintln!("trindle-dns: {e}");
                    1
                }
            };
        }
        Some("--started") => {
            started();
            return 0;
        }
        Some("--stopped") => {
            stopped();
            return 0;
        }
        _ => {}
    }
    let s = Arc::new(Shared::new(List::open(&list::folder().join(list::FILE)).map(Arc::new), shared::allowed(),
                                 shared::provider()));
    on_hangup();
    let at = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 2, 53)), 53);
    let (udp, tcp) = bind(at, Path::new(shared::RESOLV), Path::new("/"), BIND_AGAIN);
    for _ in 0..UDP_THREADS {
        let Ok(sock) = udp.try_clone() else { continue };
        let s = s.clone();
        let _ = thread::Builder::new().name("udp".into()).stack_size(STACK).spawn(move || serve_udp(&s, &sock));
    }
    {
        let s = s.clone();
        let _ = thread::Builder::new().name("tcp".into()).stack_size(STACK).spawn(move || serve_tcp(&s, &tcp));
    }
    {
        let s = s.clone();
        let _ = thread::Builder::new().name("lists".into()).stack_size(STACK).spawn(move || keep_lists(&s));
    }

    keep_status(&s);
    0
}

fn bind(at: SocketAddr, resolv: &Path, root: &Path, again: Duration) -> (UdpSocket, TcpListener) {
    let mut said = false;
    let mut written = String::new();
    loop {
        let udp = UdpSocket::bind(at);
        let tcp = TcpListener::bind(at);
        let e = match (udp, tcp) {
            (Ok(u), Ok(t)) => return (u, t),
            (Err(e), _) | (_, Err(e)) => e,
        };
        if !said {
            eprintln!("trindle-dns: cannot answer on {at}: {e}; the network's own servers are used until it can");
            said = true;
        }
        let text = network::fallback_text(&network::find(root, None));
        if text != written {
            replace(resolv, &text);
            written = text;
        }
        thread::sleep(again);
    }
}

fn started() {
    if !shared::resolved_running() {
        return;
    }
    let counted = std::fs::read_to_string(list::folder().join(resolved::TLDS_FILE)).unwrap_or_default();
    let text = resolved::drop_in(&counted, resolved::most_domains(resolved::systemd_version()));
    let path = Path::new(shared::RESOLVED_DROP_IN);
    if std::fs::read_to_string(path).is_ok_and(|t| t == text) {
        return;
    }
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    replace(path, &text);
    reload_resolved();
}

fn stopped() {
    let resolv = Path::new(shared::RESOLV);
    if resolv.parent().is_some_and(Path::exists) {
        replace(resolv, &network::fallback_text(&network::find(Path::new("/"), None)));
    }
    if std::fs::remove_file(shared::RESOLVED_DROP_IN).is_ok() {
        reload_resolved();
    }
}

fn reload_resolved() {
    let _ = std::process::Command::new("systemctl").args(["try-reload-or-restart", "systemd-resolved.service"])
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
}

fn serve_udp(s: &Shared, sock: &UdpSocket) {
    let mut buf = [0u8; 4096];
    loop {
        let Ok((n, from)) = sock.recv_from(&mut buf) else { continue };
        if let Some(r) = handle(s, &buf[..n], true) {
            let _ = sock.send_to(&r, from);
        }
    }
}

fn serve_tcp(s: &Arc<Shared>, listener: &TcpListener) {
    static OPEN: AtomicUsize = AtomicUsize::new(0);
    for conn in listener.incoming() {
        let Ok(conn) = conn else { continue };
        if OPEN.fetch_add(1, Ordering::SeqCst) >= TCP_MOST {
            OPEN.fetch_sub(1, Ordering::SeqCst);
            continue;
        }
        let s = s.clone();
        let spawned = thread::Builder::new().name("tcp-conn".into()).stack_size(STACK).spawn(move || {
            one_connection(&s, conn);
            OPEN.fetch_sub(1, Ordering::SeqCst);
        });
        if spawned.is_err() {
            OPEN.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

fn one_connection(s: &Shared, mut c: TcpStream) {
    let _ = c.set_read_timeout(Some(TCP_WAIT));
    let _ = c.set_write_timeout(Some(TCP_WAIT));
    loop {
        let mut len = [0u8; 2];
        if c.read_exact(&mut len).is_err() {
            return;
        }
        let mut msg = vec![0u8; u16::from_be_bytes(len) as usize];
        if c.read_exact(&mut msg).is_err() {
            return;
        }
        let Some(r) = handle(s, &msg, false) else { return };
        let mut out = Vec::with_capacity(r.len() + 2);
        out.extend_from_slice(&(r.len() as u16).to_be_bytes());
        out.extend_from_slice(&r);
        if c.write_all(&out).is_err() {
            return;
        }
    }
}

fn handle(s: &Shared, msg: &[u8], udp: bool) -> Option<Vec<u8>> {
    let Some(q) = wire::question(msg) else { return wire::formerr(msg) };

    if upstream::come_back(msg, &q) {
        return Some(wire::refused(msg, &q));
    }
    let r = answer(s, msg, &q);
    if udp && r.len() > wire::udp_size(msg, &q) {
        return Some(wire::truncated(msg, &q));
    }
    Some(r)
}

#[derive(Debug, PartialEq)]
enum Way {

    Tailscale(Vec<SocketAddr>),

    Local,

    Portal,

    Outside,
}

fn route(name: &str, tailscale: bool, search: &[String], split: &[Split]) -> Way {
    let ends = |suffix: &str| name.strip_suffix(suffix).is_some_and(|r| r.is_empty() || r.ends_with('.'));
    let quad100 = || Way::Tailscale(vec![SocketAddr::new(QUAD100, 53)]);
    if PORTAL_CHECKS.contains(&name) {
        return Way::Portal;
    }
    if tailscale {
        if let Some(r) = tailnet::find(name, split) {
            return if r.servers.is_empty() { quad100() } else { Way::Tailscale(r.servers.clone()) };
        }
    }
    if let Some(rest) = name.strip_suffix(".in-addr.arpa") {
        let octets: Vec<u8> = rest.split('.').rev().map_while(|o| o.parse().ok()).collect();
        return match octets.as_slice() {
            [100, b, ..] if tailscale && (64..=127).contains(b) => quad100(),
            [10, ..] | [127, ..] | [192, 168, ..] | [169, 254, ..] => Way::Local,
            [172, b, ..] if (16..=31).contains(b) => Way::Local,
            _ => Way::Outside,
        };
    }
    if let Some(rest) = name.strip_suffix(".ip6.arpa") {
        let nibbles: String = rest.split('.').rev().collect();
        return if tailscale && nibbles.starts_with("fd7a115ca1e0") {
            quad100()
        } else if nibbles.starts_with("fc") || nibbles.starts_with("fd")
            || ["fe8", "fe9", "fea", "feb"].iter().any(|p| nibbles.starts_with(p))
            || nibbles == "0".repeat(31) + "1" {
            Way::Local
        } else {
            Way::Outside
        };
    }
    if tailscale && ends("ts.net") {
        return quad100();
    }
    if !name.is_empty() && !name.contains('.') {
        return Way::Local;
    }
    if LOCAL.iter().any(|s| ends(s))
        || search.iter().any(|s| ends(s)) {
        return Way::Local;
    }
    Way::Outside
}

fn allowed(allow: &[String], name: &str) -> bool {
    allow.iter().any(|a| name.strip_suffix(a.as_str()).is_some_and(|r| r.is_empty() || r.ends_with('.')))
}

fn answer(s: &Shared, query: &[u8], q: &Question) -> Vec<u8> {
    if q.name == CANARY {
        return wire::nxdomain(query, q);
    }
    let network = read(&s.network).clone();
    let way = route(&q.name, s.tailscale_up.load(Ordering::Relaxed), &read(&s.search), &read(&s.split));
    let asked = match way {
        Way::Tailscale(servers) => upstream::plain(query, q, &servers),
        Way::Local => upstream::plain(query, q, &network.servers),
        Way::Portal => upstream::plain(query, q, &network.servers).or_else(|| fetch(s, query, q, &network)),
        Way::Outside => return outside(s, query, q, &network),
    };
    asked.unwrap_or_else(|| wire::servfail(query, q))
}

fn outside(s: &Shared, query: &[u8], q: &Question, network: &Network) -> Vec<u8> {
    let list = read(&s.list).clone();
    if !allowed(&read(&s.allow), &q.name) && list.is_some_and(|l| l.blocks(&q.name)) {
        let mut st = lock(&s.stats);
        st.today += 1;
        st.recent.retain(|n| n != &q.name);
        st.recent.push_front(q.name.clone());
        st.recent.truncate(RECENT);
        return wire::blocked(query, q);
    }
    if let Some(r) = lock(&s.cache).get(query, q, Instant::now()) {
        return r;
    }
    let Some(mut r) = fetch(s, query, q, network) else { return wire::servfail(query, q) };

    let checking = wire::flavour(query, q) & 6 == 6;
    let prefix = *read(&s.nat64);
    if let Some((prefix, len)) = prefix {
        if q.kind == wire::AAAA && q.class == 1 && !checking && wire::rcode(&r) == 0 && !wire::has(&r, q, wire::AAAA) {
            let four = wire::ask(&q.name, wire::A, [query[0], query[1]]);
            if let Some(fq) = wire::question(&four) {
                if let Some(made) = fetch(s, &four, &fq, network)
                    .and_then(|fr| wire::synthesise(query, q, &fr, &fq, &prefix, len)) {
                    r = made;
                }
            }
        }
    }
    lock(&s.cache).put(query, q, &r, Instant::now());
    r
}

fn fetch(s: &Shared, query: &[u8], q: &Question, network: &Network) -> Option<Vec<u8>> {
    let vpn = read(&s.vpn).clone();
    if !vpn.is_empty() {
        if let Some(r) = upstream::plain(query, q, &vpn).filter(|r| !matches!(wire::rcode(r), wire::SERVFAIL | wire::REFUSED)) {
            return Some(r);
        }
    }
    let private = read(&s.private).clone();
    if private.provider != Provider::Network {
        s.asked.store(true, Ordering::Relaxed);
        if let Some(r) = private.ask(query, q) {
            return Some(r);
        }
    }
    upstream::plain(query, q, &network.servers)
}

fn find_nat64(servers: &[SocketAddr]) -> Option<Prefix> {
    let query = wire::ask("ipv4only.arpa", wire::AAAA, [0x64, 0x64]);
    let q = wire::question(&query)?;
    let reply = upstream::plain(&query, &q, servers)?;
    wire::nat64_prefix(&reply, &q)
}

extern "C" fn hangup(_: libc::c_int) {
    RELOAD.store(true, Ordering::SeqCst);
}

fn on_hangup() {
    // SAFETY: the action is zeroed, then given a handler that only stores to

    unsafe {
        let mut act: libc::sigaction = std::mem::zeroed();
        act.sa_sigaction = hangup as extern "C" fn(libc::c_int) as usize;
        act.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut act.sa_mask);
        libc::sigaction(libc::SIGHUP, &act, std::ptr::null_mut());
    }
}

fn day() -> i64 {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs()) as libc::time_t;
    // SAFETY: localtime_r fills the zeroed tm given from the time given, and

    unsafe {
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&now, &mut tm).is_null() {
            return 0;
        }
        tm.tm_year as i64 * 1000 + tm.tm_yday as i64
    }
}

const OURS: &str = "# Written by trindle-dns";

fn resolv_text(search: &[String]) -> String {
    let mut t = format!("{OURS}, which blocks adverts and bad sites.\nnameserver {}\n", shared::ADDRESS);
    if !search.is_empty() {
        t.push_str("search ");
        t.push_str(&search.join(" "));
        t.push('\n');
    }
    t.push_str("options edns0 trust-ad\n");
    t
}

fn written_through(text: &str) -> Option<Network> {
    (!text.starts_with(OURS)).then(|| network::parse_resolv(text))
}

fn replace(path: &Path, text: &str) {
    let mut new = path.as_os_str().to_owned();
    new.push(".new");
    if std::fs::write(&new, text).is_ok() {
        let _ = std::fs::rename(&new, path);
    }
}

fn tailscale_suffix(ifs: &[(String, IpAddr)]) -> Option<String> {
    let [a, b, c, d] = ifs.iter().find_map(|(name, ip)| match ip {
        IpAddr::V4(v4) if name == "tailscale0" => Some(v4.octets()),
        _ => None,
    })?;
    let query = wire::ask(&format!("{d}.{c}.{b}.{a}.in-addr.arpa"), 12, [0, 0]);
    let q = wire::question(&query)?;
    let reply = upstream::plain(&query, &q, &[SocketAddr::new(QUAD100, 53)])?;
    let name = wire::first_ptr(&reply, &q)?;
    let (_, suffix) = name.split_once('.')?;
    suffix.ends_with("ts.net").then(|| suffix.to_string())
}

fn network_moved(s: &Arc<Shared>, net: Network) {
    let servers = net.servers.clone();
    *write(&s.network) = Arc::new(net);
    lock(&s.cache).clear();
    read(&s.private).fresh();
    let move_no = s.moves.fetch_add(1, Ordering::SeqCst) + 1;
    *write(&s.nat64) = None;
    let s = s.clone();
    let _ = thread::Builder::new().name("nat64".into()).stack_size(STACK).spawn(move || {
        let found = find_nat64(&servers);
        if s.moves.load(Ordering::SeqCst) == move_no {
            *write(&s.nat64) = found;
            lock(&s.cache).clear();
        }
    });
}

fn keep_status(s: &Arc<Shared>) {
    let mut keeper = Keeper::default();
    lock(&s.stats).day = day();
    loop {
        reload_on_hangup(s);
        count_days(s);
        keeper.take_in_resolv();
        let (net, ifs) = keeper.follow_network(s);
        keeper.follow_tailscale(s, &ifs);
        keeper.write_files(s, &net);
        thread::sleep(Duration::from_secs(1));
    }
}

#[derive(Default)]
struct Keeper {

    through: Option<(SystemTime, Network)>,
    seen_ifs: Option<Vec<(String, IpAddr)>>,
    suffix: Option<String>,
    suffix_tried: Option<Instant>,
    split_asked: Option<Instant>,

    resolv: String,
    status: String,
}

fn reload_on_hangup(s: &Arc<Shared>) {
    if RELOAD.swap(false, Ordering::SeqCst) {
        *write(&s.allow) = shared::allowed();
        let provider = shared::provider();
        if read(&s.private).provider != provider {
            *write(&s.private) = Arc::new(Private::new(provider));
            s.asked.store(false, Ordering::Relaxed);
        }
        lock(&s.cache).clear();
    }
}

fn count_days(s: &Shared) {
    let today = day();
    let mut st = lock(&s.stats);
    if st.day != today {
        st.day = today;
        st.today = 0;
    }
}

impl Keeper {

    fn take_in_resolv(&mut self) {
        if !self.resolv.is_empty() {
            let text = std::fs::read_to_string(shared::RESOLV).unwrap_or_default();
            if text != self.resolv {
                if let Some(n) = written_through(&text) {
                    self.through = (!n.servers.is_empty()).then(|| (SystemTime::now(), n));
                }
                self.resolv.clear();
            }
        }
    }

    fn follow_network(&mut self, s: &Arc<Shared>) -> (Arc<Network>, Vec<(String, IpAddr)>) {
        let net = network::find(Path::new("/"), self.through.as_ref());
        let mut ifs = network::interfaces();
        ifs.sort();
        if self.seen_ifs.as_ref() != Some(&ifs) || **read(&s.network) != net {
            network_moved(s, net);
            self.seen_ifs = Some(ifs.clone());
        }
        let net = read(&s.network).clone();
        let vpn = network::through_vpn(&net.servers, &ifs, Path::new(INTERFACES));
        if *read(&s.vpn) != vpn {
            *write(&s.vpn) = vpn;
            lock(&s.cache).clear();
        }
        (net, ifs)
    }

    fn follow_tailscale(&mut self, s: &Shared, ifs: &[(String, IpAddr)]) {
        let up = Path::new(TAILSCALE).exists();
        s.tailscale_up.store(up, Ordering::Relaxed);
        if !up {
            self.suffix = None;
            self.split_asked = None;
            if !read(&s.split).is_empty() {
                write(&s.split).clear();
            }
        } else {
            if self.suffix.is_none() && self.suffix_tried.is_none_or(|t| t.elapsed() >= Duration::from_secs(60)) {
                self.suffix_tried = Some(Instant::now());
                self.suffix = tailscale_suffix(ifs);
            }
            if self.split_asked.is_none_or(|t| t.elapsed() >= SPLIT_AGAIN) {
                self.split_asked = Some(Instant::now());

                if let Some(routes) = tailnet::routes() {
                    if *read(&s.split) != routes {
                        *write(&s.split) = routes;
                        lock(&s.cache).clear();
                    }
                }
            }
        }
    }

    fn write_files(&mut self, s: &Shared, net: &Network) {
        let mut search = net.search.clone();
        if let Some(t) = &self.suffix {
            if !search.contains(t) {
                search.push(t.clone());
            }
        }
        let mut local = search.clone();
        local.extend(net.routes.iter().filter(|r| !search.contains(r)).cloned());
        if *read(&s.search) != local {
            *write(&s.search) = local;
        }
        let text = resolv_text(&search);
        if text != self.resolv {
            replace(Path::new(shared::RESOLV), &text);
            self.resolv = text;
        }
        let text = current(s).write();
        if text != self.status {
            replace(Path::new(shared::STATUS), &text);
            self.status = text;
        }
    }
}

fn current(s: &Shared) -> Status {
    let list = read(&s.list).clone();
    let private = read(&s.private).clone();
    let vpn = !read(&s.vpn).is_empty();
    let st = lock(&s.stats);
    Status {
        listed: list.as_ref().map_or(0, |l| l.count),
        fetched: list.as_ref().map_or(0, |l| l.fetched),
        fetching: s.fetching.load(Ordering::Relaxed),
        today: st.today,
        private: if vpn {
            shared::Private::Vpn
        } else if private.provider == Provider::Network {
            shared::Private::Network
        } else if !s.asked.load(Ordering::Relaxed) {
            shared::Private::Starting
        } else if private.up() {
            shared::Private::Yes
        } else {
            shared::Private::Fallback
        },
        recent: st.recent.iter().cloned().collect(),
    }
}

fn keep_lists(s: &Shared) {
    loop {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let kept = read(&s.list).as_ref().map(|l| now.saturating_sub(l.fetched) < STALE);
        let fetch = crate::common::update_checks::may_fetch_catalogue(kept.is_some(), kept == Some(true), crate::common::update_checks::on_system_wide());
        if fetch {
            s.fetching.store(true, Ordering::Relaxed);
            let ok = std::process::Command::new("/proc/self/exe").arg0("trindle-dns").arg("--refresh")
                .stdout(std::process::Stdio::null()).status().is_ok_and(|st| st.success());
            if ok {
                if let Some(l) = List::open(&list::folder().join(list::FILE)) {
                    *write(&s.list) = Some(Arc::new(l));
                }
            }
            s.fetching.store(false, Ordering::Relaxed);
        }
        thread::sleep(if read(&s.list).is_none() { RETRY } else { CHECK });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wire::tests::{answer as a_answer, query, with};

    #[test]
    fn routes() {
        let search = vec!["corp.example".to_string()];
        let split = vec![Split { suffix: "wiki.corp.example".into(), servers: vec!["10.0.0.53:53".parse().unwrap()] },
                         Split { suffix: "records.example".into(), servers: Vec::new() }];
        let r = |n: &str, t: bool| route(n, t, &search, &split);
        let quad100 = Way::Tailscale(vec![SocketAddr::new(QUAD100, 53)]);
        assert_eq!(r("example.com", true), Way::Outside);
        assert_eq!(r("printer", true), Way::Local);
        assert_eq!(r("nas.lan", false), Way::Local);
        assert_eq!(r("printer.local", false), Way::Local);
        assert_eq!(r("router.home.arpa", false), Way::Local);
        assert_eq!(r("x.corp.example", false), Way::Local);
        assert_eq!(r("notcorp.example", false), Way::Outside);
        assert_eq!(r("laptop.tail1.ts.net", true), quad100);
        assert_eq!(r("laptop.tail1.ts.net", false), Way::Outside);
        assert_eq!(r("5.0.64.100.in-addr.arpa", true), quad100);
        assert_eq!(r("5.0.64.100.in-addr.arpa", false), Way::Outside);
        assert_eq!(r("1.1.168.192.in-addr.arpa", false), Way::Local);
        assert_eq!(r("1.0.20.172.in-addr.arpa", false), Way::Local);
        assert_eq!(r("1.0.40.172.in-addr.arpa", false), Way::Outside);
        assert_eq!(r("8.8.8.8.in-addr.arpa", false), Way::Outside);
        let v6 = |hex: &str| hex.chars().rev().map(|c| c.to_string()).collect::<Vec<_>>().join(".") + ".ip6.arpa";
        assert_eq!(r(&v6("fe800000000000000000000000000001"), false), Way::Local);
        assert_eq!(r(&v6("fd7a115ca1e000000000000000000001"), true), quad100);
        assert_eq!(r(&v6("26064700470000000000000000001111"), false), Way::Outside);
        assert_eq!(r("", false), Way::Outside);
        assert_eq!(r("detectportal.firefox.com", false), Way::Portal);
        assert_eq!(r("nmcheck.gnome.org", true), Way::Portal);
        assert_eq!(r("ads.nmcheck.gnome.org", false), Way::Outside);
        assert_eq!(r("network-test.debian.org", false), Way::Portal);

        assert_eq!(r("page.wiki.corp.example", true), Way::Tailscale(vec!["10.0.0.53:53".parse().unwrap()]));
        assert_eq!(r("page.wiki.corp.example", false), Way::Local);
        assert_eq!(r("mail.corp.example", true), Way::Local);
        assert_eq!(r("db.records.example", true), quad100);
        assert_eq!(r("db.records.example", false), Way::Outside);
    }

    #[test]
    fn allow_list() {
        let a = vec!["example.com".to_string()];
        assert!(allowed(&a, "example.com"));
        assert!(allowed(&a, "ads.example.com"));
        assert!(!allowed(&a, "badexample.com"));
    }

    #[test]
    fn resolv_files() {
        let search = vec!["home.lan".to_string(), "corp.example".to_string()];
        let t = resolv_text(&search);
        assert!(t.contains("nameserver 127.0.2.53\nsearch home.lan corp.example\n"));

        assert_eq!(written_through(&t), None);
        assert_eq!(written_through(&network::fallback_text(&Network::default())), None);
        let vpn = written_through("# Generated by resolvconf\nnameserver 10.8.0.1\nsearch corp.example\n").unwrap();
        assert_eq!(vpn.servers, ["10.8.0.1:53".parse::<SocketAddr>().unwrap()]);
        assert!(written_through("nameserver 127.0.2.53\n").unwrap().servers.is_empty());
    }

    fn fake(f: impl Fn(&[u8], &Question) -> Option<Vec<u8>> + Send + 'static) -> SocketAddr {
        let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
        let at = sock.local_addr().unwrap();
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok((n, from)) = sock.recv_from(&mut buf) {
                let Some(q) = wire::question(&buf[..n]) else { continue };
                if let Some(r) = f(&buf[..n], &q) {
                    let _ = sock.send_to(&r, from);
                }
            }
        });
        at
    }

    fn addresses(r: &[u8], q: &Question, kind: u16) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let mut i = q.end;
        for _ in 0..u16::from_be_bytes([r[6], r[7]]) {
            while r[i] != 0 && r[i] & 0xc0 != 0xc0 {
                i += 1 + r[i] as usize;
            }
            i += if r[i] == 0 { 1 } else { 2 };
            let len = u16::from_be_bytes([r[i + 8], r[i + 9]]) as usize;
            if u16::from_be_bytes([r[i], r[i + 1]]) == kind {
                out.push(r[i + 10..i + 10 + len].to_vec());
            }
            i += 10 + len;
        }
        out
    }

    #[test]
    fn answers_on_every_network() {

        let home = fake(|m, q| {
            let mut r = match (q.name.as_str(), q.kind) {
                ("ipv4only.arpa", wire::AAAA) => {
                    with(m, &[(wire::AAAA, &[0, 0x64, 0xff, 0x9b, 0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 170])])
                }
                (_, wire::A) => a_answer(m, &[300]),
                _ => a_answer(m, &[]),
            };
            r[3] |= 0x20;
            Some(r)
        });
        let dir = std::env::temp_dir().join(format!("trindle-dns-answers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(list::FILE);
        std::fs::write(&path, list::build(&["ads.example.com\n".to_string()], 1)).unwrap();
        let s = Shared::new(List::open(&path).map(Arc::new), Vec::new(), Provider::Network);
        *write(&s.network) = Arc::new(Network { servers: vec![home], ..Network::default() });
        let ask = |name: &str, kind: u16| {
            let m = query(name, kind, None);
            let q = wire::question(&m).unwrap();
            let r = answer(&s, &m, &q);
            assert!(wire::answers(&r, &m, &q), "{name}");
            (r, q)
        };

        let (r, q) = ask("x.ads.example.com", wire::A);
        assert_eq!(addresses(&r, &q, wire::A), [vec![0u8; 4]]);
        assert_eq!(wire::rcode(&ask(CANARY, wire::A).0), 3);
        let (r, q) = ask("example.org", wire::A);
        assert_eq!(addresses(&r, &q, wire::A), [vec![93u8, 184, 215, 14]]);
        assert_eq!(r[3] & 0x20, 0);
        let (r, q) = ask("detectportal.firefox.com", wire::A);
        assert_eq!(addresses(&r, &q, wire::A).len(), 1);

        let prefix = find_nat64(&[home]);
        assert_eq!(prefix.map(|p| p.1), Some(96));
        let (r, q) = ask("v4only.example", wire::AAAA);
        assert!(addresses(&r, &q, wire::AAAA).is_empty());
        *write(&s.nat64) = prefix;
        lock(&s.cache).clear();
        let (r, q) = ask("v4only.example", wire::AAAA);
        assert_eq!(addresses(&r, &q, wire::AAAA), [vec![0, 0x64, 0xff, 0x9b, 0, 0, 0, 0, 0, 0, 0, 0, 93, 184, 215, 14]]);

        let vpn = fake(|m, _| {
            let mut r = a_answer(m, &[60]);
            let n = r.len();
            r[n - 4..].copy_from_slice(&[10, 8, 0, 1]);
            Some(r)
        });
        *write(&s.vpn) = vec![vpn];
        lock(&s.cache).clear();
        let (r, q) = ask("intranet.corp.example", wire::A);
        assert_eq!(addresses(&r, &q, wire::A), [vec![10u8, 8, 0, 1]]);
        let (r, q) = ask("nas.lan", wire::A);
        assert_eq!(addresses(&r, &q, wire::A), [vec![93u8, 184, 215, 14]]);
        assert_eq!(current(&s).private, shared::Private::Vpn);

        *write(&s.vpn) = Vec::new();
        *write(&s.network) = Arc::new(Network::default());
        assert_eq!(wire::rcode(&ask("printer", wire::A).0), wire::SERVFAIL);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn forwarder(upstream: SocketAddr, lower: bool) -> SocketAddr {
        let sock = Arc::new(UdpSocket::bind("127.0.0.1:0").unwrap());
        let at = sock.local_addr().unwrap();
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok((n, from)) = sock.recv_from(&mut buf) {
                let mut m = buf[..n].to_vec();
                let sock = sock.clone();
                thread::spawn(move || {
                    let Some(q) = wire::question(&m) else { return };
                    let id = [m[0], m[1]];
                    if lower {
                        m[wire::HEADER..q.end].make_ascii_lowercase();
                    }
                    m[..2].copy_from_slice(&[0xab, 0xcd]);
                    let up = UdpSocket::bind("127.0.0.1:0").unwrap();
                    up.connect(upstream).unwrap();
                    up.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
                    up.send(&m).unwrap();
                    let mut b = [0u8; 4096];
                    let mut r = match up.recv(&mut b) {
                        Ok(n) => b[..n].to_vec(),
                        Err(_) => wire::servfail(&m, &q),
                    };
                    r[..2].copy_from_slice(&id);
                    let _ = sock.send_to(&r, from);
                });
            }
        });
        at
    }

    #[test]
    fn dnsmasq_reading_resolv_conf_never_loops() {
        let s = Arc::new(Shared::new(None, Vec::new(), Provider::Network));
        let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
        let at = sock.local_addr().unwrap();
        for _ in 0..4 {
            let (s, sock) = (s.clone(), sock.try_clone().unwrap());
            thread::spawn(move || serve_udp(&s, &sock));
        }
        let home = fake(|m, _| Some(a_answer(m, &[300])));
        let ask = |name: &str| {
            let c = UdpSocket::bind("127.0.0.1:0").unwrap();
            c.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            c.connect(at).unwrap();
            let m = query(name, wire::A, None);
            let t = Instant::now();
            c.send(&m).unwrap();
            let mut b = [0u8; 4096];
            let n = c.recv(&mut b).unwrap();
            let q = wire::question(&m).unwrap();

            assert_eq!(&b[wire::HEADER..q.end], &m[wire::HEADER..q.end]);
            (b[..n].to_vec(), q, t.elapsed())
        };
        for lower in [false, true] {
            let dnsmasq = forwarder(at, lower);

            *write(&s.network) = Arc::new(Network { servers: vec![dnsmasq, home], ..Network::default() });
            let name = format!("printer{}.lan", u8::from(lower));
            let (r, q, took) = ask(&name);
            assert_eq!(wire::rcode(&r), 0, "lower case: {lower}");
            assert_eq!(addresses(&r, &q, wire::A), [vec![93u8, 184, 215, 14]]);
            assert!(took < Duration::from_secs(1), "{took:?}");

            *write(&s.network) = Arc::new(Network { servers: vec![dnsmasq], ..Network::default() });
            let (r, _, took) = ask(&format!("nas{}.lan", u8::from(lower)));
            assert_eq!(wire::rcode(&r), wire::REFUSED);
            assert!(took < Duration::from_secs(1), "{took:?}");
        }

        let slow = fake(|m, _| {
            thread::sleep(Duration::from_millis(300));
            Some(a_answer(m, &[300]))
        });
        *write(&s.network) = Arc::new(Network { servers: vec![slow], ..Network::default() });
        let both: Vec<_> = (0..2).map(|_| thread::spawn(move || {
            let c = UdpSocket::bind("127.0.0.1:0").unwrap();
            c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            c.send_to(&query("twice.lan", wire::A, None), at).unwrap();
            let mut b = [0u8; 512];
            let n = c.recv(&mut b).unwrap();
            wire::rcode(&b[..n])
        })).collect();
        for t in both {
            assert_eq!(t.join().unwrap(), 0);
        }
    }

    #[test]
    fn port_held_by_another() {

        let dir = std::env::temp_dir().join(format!("trindle-dns-bind-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("run/NetworkManager")).unwrap();
        std::fs::write(dir.join("run/NetworkManager/resolv.conf"), "nameserver 192.0.2.1\n").unwrap();
        let other = UdpSocket::bind("127.0.0.1:0").unwrap();
        let at = other.local_addr().unwrap();
        let resolv = dir.join("resolv.conf");
        let (r, d) = (resolv.clone(), dir.clone());
        let took = thread::spawn(move || bind(at, &r, &d, Duration::from_millis(50)));
        let t = Instant::now();
        while !resolv.exists() && t.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }
        assert!(std::fs::read_to_string(&resolv).unwrap().contains("nameserver 192.0.2.1\n"));
        drop(other);
        let (udp, tcp) = took.join().unwrap();
        assert_eq!((udp.local_addr().unwrap(), tcp.local_addr().unwrap()), (at, at));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
