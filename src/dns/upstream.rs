//! Asking upstream servers: private DNS over TLS or HTTPS, or the network's own.

use std::io::{self, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};

use super::wire::{self, Question};
use crate::common::dns::Provider;

const CONNECT: Duration = Duration::from_secs(2);
const WAIT: Duration = Duration::from_secs(3);
const UDP_WAIT: Duration = Duration::from_secs(2);

const IDLE: Duration = Duration::from_secs(15);
const KEEP: usize = 4;

const REST: Duration = Duration::from_secs(60);

const TRY_FOR: Duration = Duration::from_secs(4);

type Tls = StreamOwned<ClientConnection, TcpStream>;

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

fn exchange<S: Read + Write>(s: &mut S, query: &[u8]) -> io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(query.len() + 2);
    out.extend_from_slice(&(query.len() as u16).to_be_bytes());
    out.extend_from_slice(query);
    s.write_all(&out)?;
    s.flush()?;
    let mut len = [0u8; 2];
    s.read_exact(&mut len)?;
    let mut reply = vec![0u8; u16::from_be_bytes(len) as usize];
    s.read_exact(&mut reply)?;
    Ok(reply)
}

fn config(way: Way) -> Option<Arc<ClientConfig>> {
    static CONFIG: OnceLock<[Option<Arc<ClientConfig>>; 2]> = OnceLock::new();
    let both = CONFIG.get_or_init(|| {
        let mut roots = RootCertStore::empty();
        roots.add_parsable_certificates(rustls_native_certs::load_native_certs().certs);
        let make = |alpn: &[u8]| {
            let provider = Arc::new(rustls::crypto::ring::default_provider());
            let mut c = ClientConfig::builder_with_provider(provider).with_safe_default_protocol_versions().ok()?
                .with_root_certificates(roots.clone()).with_no_client_auth();
            c.alpn_protocols = vec![alpn.to_vec()];
            Some(Arc::new(c))
        };
        [make(b"dot"), make(b"h2")]
    });
    both[way as usize].clone()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Way {
    Tls = 0,
    Https = 1,
}

struct Conn {
    tls: Tls,
    way: Way,
    stream: u32,
    used: Instant,
}

pub struct Private {
    pub provider: Provider,
    host: &'static str,
    name: Option<ServerName<'static>>,
    addresses: Vec<IpAddr>,

    ports: [u16; 2],
    pool: Mutex<Vec<Conn>>,

    way: Mutex<Way>,
    down_until: Mutex<Option<Instant>>,

    trying: AtomicBool,
}

struct Trying<'a>(&'a AtomicBool);

impl Drop for Trying<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

impl Private {
    pub fn new(provider: Provider) -> Private {
        let (host, ips) = provider.servers();
        Private {
            provider,
            host,
            name: ServerName::try_from(host.to_string()).ok(),
            addresses: ips.iter().filter_map(|ip| ip.parse::<IpAddr>().ok()).collect(),
            ports: [853, 443],
            pool: Mutex::new(Vec::new()),
            way: Mutex::new(Way::Tls),
            down_until: Mutex::new(None),
            trying: AtomicBool::new(false),
        }
    }

    pub fn up(&self) -> bool {
        !self.addresses.is_empty() && lock(&self.down_until).is_none_or(|t| Instant::now() >= t)
    }

    fn connect(&self, ip: IpAddr, way: Way) -> Option<Conn> {
        let tcp = TcpStream::connect_timeout(&SocketAddr::new(ip, self.ports[way as usize]), CONNECT).ok()?;
        tcp.set_read_timeout(Some(WAIT)).ok()?;
        tcp.set_write_timeout(Some(WAIT)).ok()?;
        let _ = tcp.set_nodelay(true);
        let conn = ClientConnection::new(config(way)?, self.name.clone()?).ok()?;
        let mut tls = StreamOwned::new(conn, tcp);
        if way == Way::Https {
            h2::start(&mut tls).ok()?;
        }
        Some(Conn { tls, way, stream: 1, used: Instant::now() })
    }

    fn over(&self, c: &mut Conn, query: &[u8], q: &Question) -> Option<(Vec<u8>, bool)> {
        let (r, again) = match c.way {
            Way::Tls => (exchange(&mut c.tls, query).ok()?, true),
            Way::Https => {
                let r = h2::ask(&mut c.tls, self.host, c.stream, query).ok()?;
                c.stream += 2;
                (r.0, r.1 && c.stream < h2::LAST_STREAM)
            }
        };
        wire::answers(&r, query, q).then_some((r, again))
    }

    pub fn ask(&self, query: &[u8], q: &Question) -> Option<Vec<u8>> {
        if !self.up() {
            return None;
        }

        let _trying = if lock(&self.down_until).is_some() {
            if self.trying.swap(true, Ordering::SeqCst) {
                return None;
            }
            Some(Trying(&self.trying))
        } else {
            None
        };
        let kept = {
            let mut pool = lock(&self.pool);
            pool.retain(|c| c.used.elapsed() < IDLE);
            pool.pop()
        };

        if let Some(mut c) = kept {
            if let Some((r, again)) = self.over(&mut c, query, q) {
                if again {
                    self.keep(c);
                }
                return Some(r);
            }
        }
        let first = *lock(&self.way);
        let other = if first == Way::Tls { Way::Https } else { Way::Tls };
        let give_up = Instant::now() + TRY_FOR;

        for &ip in &self.addresses {
            for way in [first, other] {
                if Instant::now() >= give_up {
                    break;
                }
                let Some(mut c) = self.connect(ip, way) else { continue };
                if let Some((r, again)) = self.over(&mut c, query, q) {
                    *lock(&self.way) = way;
                    *lock(&self.down_until) = None;
                    if again {
                        self.keep(c);
                    }
                    return Some(r);
                }
            }
        }
        *lock(&self.down_until) = Some(Instant::now() + REST);
        lock(&self.pool).clear();
        None
    }

    pub fn fresh(&self) {
        *lock(&self.down_until) = None;
        lock(&self.pool).clear();
    }

    fn keep(&self, mut c: Conn) {
        c.used = Instant::now();
        let mut pool = lock(&self.pool);
        if pool.len() < KEEP {
            pool.push(c);
        }
    }
}

mod h2 {
    use std::io::{self, Read, Write};

    const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
    const DATA: u8 = 0;
    const HEADERS: u8 = 1;
    const RST_STREAM: u8 = 3;
    const SETTINGS: u8 = 4;
    const PING: u8 = 6;
    const GOAWAY: u8 = 7;
    const WINDOW_UPDATE: u8 = 8;
    const END_STREAM: u8 = 0x1;
    const ACK: u8 = 0x1;
    const END_HEADERS: u8 = 0x4;
    const PADDED: u8 = 0x8;

    const FRAME_MOST: usize = 16_384;

    pub const LAST_STREAM: u32 = 1 << 30;

    fn bad(why: &str) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, why.to_string())
    }

    fn frame(out: &mut Vec<u8>, kind: u8, flags: u8, stream: u32, body: &[u8]) {
        out.extend_from_slice(&(body.len() as u32).to_be_bytes()[1..]);
        out.push(kind);
        out.push(flags);
        out.extend_from_slice(&stream.to_be_bytes());
        out.extend_from_slice(body);
    }

    pub fn start<S: Write>(s: &mut S) -> io::Result<()> {
        let mut out = PREFACE.to_vec();
        frame(&mut out, SETTINGS, 0, 0, &[0, 2, 0, 0, 0, 0]);
        s.write_all(&out)?;
        s.flush()
    }

    fn header(block: &mut Vec<u8>, name: u8, value: &str) {
        if name < 15 {
            block.push(0x10 | name);
        } else {
            block.extend_from_slice(&[0x1f, name - 15]);
        }

        let v = &value.as_bytes()[..value.len().min(126)];
        block.push(v.len() as u8);
        block.extend_from_slice(v);
    }

    pub fn ask<S: Read + Write>(s: &mut S, host: &str, stream: u32, query: &[u8]) -> io::Result<(Vec<u8>, bool)> {

        let mut block = vec![0x83, 0x87];
        header(&mut block, 4, "/dns-query");
        header(&mut block, 1, host);
        header(&mut block, 31, "application/dns-message");
        header(&mut block, 19, "application/dns-message");
        header(&mut block, 28, &query.len().to_string());
        let mut out = Vec::new();
        frame(&mut out, HEADERS, END_HEADERS, stream, &block);
        frame(&mut out, DATA, END_STREAM, stream, query);
        s.write_all(&out)?;
        s.flush()?;
        let mut body = Vec::new();
        let mut again = true;
        loop {
            let mut head = [0u8; 9];
            s.read_exact(&mut head)?;
            let len = u32::from_be_bytes([0, head[0], head[1], head[2]]) as usize;
            let (kind, flags) = (head[3], head[4]);
            let id = u32::from_be_bytes([head[5], head[6], head[7], head[8]]) & 0x7fff_ffff;
            if len > FRAME_MOST {
                return Err(bad("frame too big"));
            }
            let mut payload = vec![0u8; len];
            s.read_exact(&mut payload)?;
            let mut reply = Vec::new();
            match kind {
                DATA if id == stream => {
                    let mut part = &payload[..];
                    if flags & PADDED != 0 {
                        let pad = *part.first().ok_or_else(|| bad("padding"))? as usize;
                        part = part.get(1..part.len().checked_sub(pad).ok_or_else(|| bad("padding"))?)
                            .ok_or_else(|| bad("padding"))?;
                    }
                    if body.len() + part.len() > 65_535 {
                        return Err(bad("too big"));
                    }
                    body.extend_from_slice(part);

                    if len > 0 {
                        frame(&mut reply, WINDOW_UPDATE, 0, 0, &(len as u32).to_be_bytes());
                    }
                }
                HEADERS if id == stream && flags & END_STREAM != 0 => return Err(bad("no answer")),
                RST_STREAM if id == stream => return Err(bad("stream refused")),
                SETTINGS if flags & ACK == 0 => frame(&mut reply, SETTINGS, ACK, 0, &[]),
                PING if flags & ACK == 0 => frame(&mut reply, PING, ACK, 0, &payload),
                GOAWAY => again = false,
                _ => {}
            }
            if !reply.is_empty() {
                s.write_all(&reply)?;
                s.flush()?;
            }
            if kind == DATA && id == stream && flags & END_STREAM != 0 {
                return Ok((body, again));
            }
        }
    }
}

fn random_id() -> [u8; 2] {
    let mut id = [0u8; 2];
    random(&mut id);
    id
}

fn random(out: &mut [u8]) {
    // SAFETY: fills exactly the bytes of the slice given, which lives until

    let n = unsafe { libc::getrandom(out.as_mut_ptr().cast(), out.len(), 0) };
    if n != out.len() as isize {
        let mut t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as u64);
        for b in out.iter_mut() {
            t = t.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            *b = (t >> 33) as u8;
        }
    }
}

struct Out {

    sent: Vec<u8>,
    name: String,
    kind: u16,
    id: u64,
}

static OUT: Mutex<Vec<Out>> = Mutex::new(Vec::new());
static OUT_IDS: AtomicU64 = AtomicU64::new(0);

struct Sent(u64);

impl Drop for Sent {
    fn drop(&mut self) {
        lock(&OUT).retain(|o| o.id != self.0);
    }
}

fn scrambled(query: &[u8], q: &Question) -> Vec<u8> {
    let mut out = query.to_vec();
    let letters: Vec<usize> = (wire::HEADER..q.end.saturating_sub(4)).filter(|&i| out[i].is_ascii_alphabetic()).collect();
    if letters.is_empty() {
        return out;
    }
    let mut bits = vec![0u8; letters.len()];
    random(&mut bits);
    for (&i, b) in letters.iter().zip(&bits) {
        out[i] = if b & 1 == 1 { out[i].to_ascii_uppercase() } else { out[i].to_ascii_lowercase() };
    }
    if out[wire::HEADER..q.end] == query[wire::HEADER..q.end] {
        out[letters[0]] ^= 0x20;
    }
    out
}

fn send_out(sent: &[u8], q: &Question) -> Sent {
    let id = OUT_IDS.fetch_add(1, Ordering::Relaxed);
    lock(&OUT).push(Out { sent: sent[wire::HEADER..q.end].to_vec(), name: q.name.clone(), kind: q.kind, id });
    Sent(id)
}

pub fn come_back(msg: &[u8], q: &Question) -> bool {
    let asked = &msg[wire::HEADER..q.end];
    let out = lock(&OUT);
    out.iter().any(|o| o.sent == asked) || out.iter().filter(|o| o.kind == q.kind && o.name == q.name).count() >= 2
}

fn udp(query: &[u8], q: &Question, to: SocketAddr) -> Option<Vec<u8>> {
    let any: SocketAddr = if to.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" }.parse().ok()?;
    let s = UdpSocket::bind(any).ok()?;

    s.connect(to).ok()?;
    s.set_read_timeout(Some(UDP_WAIT)).ok()?;
    s.send(query).ok()?;
    let end = Instant::now() + UDP_WAIT;
    let mut buf = vec![0u8; 65_535];
    while Instant::now() < end {
        let n = s.recv(&mut buf).ok()?;
        if wire::answers(&buf[..n], query, q) {
            buf.truncate(n);
            return Some(buf);
        }
    }
    None
}

fn tcp(query: &[u8], q: &Question, to: SocketAddr) -> Option<Vec<u8>> {
    let mut s = TcpStream::connect_timeout(&to, CONNECT).ok()?;
    s.set_read_timeout(Some(WAIT)).ok()?;
    s.set_write_timeout(Some(WAIT)).ok()?;
    exchange(&mut s, query).ok().filter(|r| wire::answers(r, query, q))
}

pub fn plain(query: &[u8], q: &Question, servers: &[SocketAddr]) -> Option<Vec<u8>> {
    if servers.is_empty() {
        return None;
    }
    let mut ours = scrambled(query, q);
    ours[..2].copy_from_slice(&random_id());
    let _out = send_out(&ours, q);
    let mut failed = None;
    for &to in servers {
        let mut reply = udp(&ours, q, to);
        if reply.as_deref().is_some_and(wire::is_truncated) {
            reply = tcp(&ours, q, to).or(reply);
        }
        let Some(mut r) = reply else { continue };
        r[..2].copy_from_slice(&query[..2]);
        r[wire::HEADER..q.end].copy_from_slice(&query[wire::HEADER..q.end]);

        wire::not_checked(&mut r);
        if matches!(wire::rcode(&r), wire::SERVFAIL | wire::REFUSED) {
            failed = Some(r);
            continue;
        }
        return Some(r);
    }
    failed
}

#[cfg(test)]
mod tests {
    use super::h2;
    use std::io::{self, Read, Write};

    struct Fake {
        reply: io::Cursor<Vec<u8>>,
        sent: Vec<u8>,
    }

    impl Read for Fake {
        fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
            self.reply.read(b)
        }
    }

    impl Write for Fake {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.sent.extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn frame(kind: u8, flags: u8, stream: u32, body: &[u8]) -> Vec<u8> {
        let mut f = (body.len() as u32).to_be_bytes()[1..].to_vec();
        f.extend_from_slice(&[kind, flags]);
        f.extend_from_slice(&stream.to_be_bytes());
        f.extend_from_slice(body);
        f
    }

    fn fake(frames: &[Vec<u8>]) -> Fake {
        Fake { reply: io::Cursor::new(frames.concat()), sent: Vec::new() }
    }

    #[test]
    fn over_http2() {
        let mut opening = Vec::new();
        h2::start(&mut opening).unwrap();
        assert!(opening.starts_with(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"));

        let mut s = fake(&[frame(4, 0, 0, &[0, 3, 0, 0, 0, 100]), frame(6, 0, 0, b"12345678"),
                           frame(1, 4, 3, &[0x88]), frame(0, 0, 3, b"ab"), frame(0, 9, 3, &[2, b'c', 0, 0])]);
        assert_eq!(h2::ask(&mut s, "dns.quad9.net", 3, b"xy").unwrap(), (b"abc".to_vec(), true));
        let sent = s.sent;

        assert_eq!(&sent[3..9], &[1, 4, 0, 0, 0, 3]);
        assert!(sent.windows(13).any(|w| w == b"dns.quad9.net"));
        let data = frame(0, 1, 3, b"xy");
        assert!(sent.windows(data.len()).any(|w| w == data));

        assert!(sent.windows(9).any(|w| w == frame(4, 1, 0, &[])));
        assert!(sent.windows(17).any(|w| w == frame(6, 1, 0, b"12345678")));
        assert!(sent.windows(13).any(|w| w == frame(8, 0, 0, &[0, 0, 0, 2])));

        assert!(h2::ask(&mut fake(&[frame(3, 0, 1, &[0, 0, 0, 7])]), "h", 1, b"q").is_err());
        assert!(h2::ask(&mut fake(&[frame(1, 5, 1, &[0x88])]), "h", 1, b"q").is_err());
        assert!(h2::ask(&mut fake(&[frame(0, 0, 1, b"ab")]), "h", 1, b"q").is_err());

        let mut s = fake(&[frame(7, 0, 0, &[0, 0, 0, 1, 0, 0, 0, 0]), frame(0, 1, 1, b"a")]);
        assert_eq!(h2::ask(&mut s, "h", 1, b"q").unwrap(), (b"a".to_vec(), false));
    }

    #[test]
    fn private_server_out_of_reach() {
        use super::{lock, Private};
        use crate::common::dns::Provider;
        use crate::dns::wire::{question, tests::query};
        use std::net::TcpListener;
        use std::sync::atomic::Ordering;
        use std::time::{Duration, Instant};
        let tls = TcpListener::bind("127.0.0.1:0").unwrap();
        let https = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut p = Private::new(Provider::Quad9);
        p.addresses = vec!["127.0.0.1".parse().unwrap(), "127.0.0.1".parse().unwrap()];
        p.ports = [tls.local_addr().unwrap().port(), https.local_addr().unwrap().port()];
        let m = query("example.org", 1, None);
        let q = question(&m).unwrap();
        let t = Instant::now();
        assert!(p.ask(&m, &q).is_none());
        assert!(t.elapsed() < Duration::from_secs(8), "{:?}", t.elapsed());

        tls.set_nonblocking(true).unwrap();
        https.set_nonblocking(true).unwrap();
        assert!(tls.accept().is_ok());
        assert!(https.accept().is_ok());
        assert!(!p.up());

        let t = Instant::now();
        assert!(p.ask(&m, &q).is_none());
        assert!(t.elapsed() < Duration::from_millis(100));

        *lock(&p.down_until) = Some(Instant::now());
        p.trying.store(true, Ordering::SeqCst);
        let t = Instant::now();
        assert!(p.up() && p.ask(&m, &q).is_none());
        assert!(t.elapsed() < Duration::from_millis(100));

        p.fresh();
        assert!(p.up() && lock(&p.down_until).is_none());
    }

    #[test]
    #[ignore = "needs a real network and real DNS servers: run by hand"]
    fn real_servers() {
        use super::{Private, Way};
        use crate::common::dns::Provider;
        use crate::dns::wire::{question, tests::query};
        let m = query("example.org", 1, None);
        let q = question(&m).unwrap();
        for provider in [Provider::Quad9, Provider::Mullvad, Provider::Cloudflare] {
            let p = Private::new(provider);
            for way in [Way::Tls, Way::Https] {
                let got = p.connect(p.addresses[0], way).map(|mut c| {
                    let one = p.over(&mut c, &m, &q).map(|r| r.0.len());

                    let two = p.over(&mut c, &m, &q).map(|r| r.0.len());
                    (one, two)
                });
                eprintln!("{provider:?} {way:?}: {got:?}");
            }
            assert!(p.ask(&m, &q).is_some());
        }
    }
}
