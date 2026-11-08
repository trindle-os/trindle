//! The answers to a search for devices.

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::time::{Duration, Instant};

use super::home_address;

pub const GROUPS: [(Ipv4Addr, u16); 2] = [(Ipv4Addr::new(224, 0, 0, 251), 5353), (Ipv4Addr::new(239, 255, 255, 250), 3702)];

pub const PAYLOAD_MAX: usize = 1472;

const ASKS_PER_SECOND: u32 = 50;
const PORTS_MOST: usize = 16;

const REMEMBER: Duration = Duration::from_secs(60);

const ASK: u8 = b'Q';
const ANSWER: u8 = b'A';

pub fn encode_ask(group: usize, port: u16, payload: &[u8]) -> Option<Vec<u8>> {
    if group >= GROUPS.len() || port == 0 || payload.is_empty() || payload.len() > PAYLOAD_MAX {
        return None;
    }
    let mut m = vec![ASK, group as u8];
    m.extend(port.to_be_bytes());
    m.extend(payload);
    Some(m)
}

pub fn decode_ask(m: &[u8]) -> Option<(usize, u16, &[u8])> {
    let (head, payload) = (m.get(..4)?, m.get(4..)?);
    let (group, port) = (usize::from(head[1]), u16::from_be_bytes([head[2], head[3]]));
    (head[0] == ASK && group < GROUPS.len() && port != 0 && !payload.is_empty() && payload.len() <= PAYLOAD_MAX).then_some((group, port, payload))
}

pub fn encode_answer(port: u16, from: SocketAddrV4, payload: &[u8]) -> Option<Vec<u8>> {
    if port == 0 || payload.is_empty() || payload.len() > PAYLOAD_MAX {
        return None;
    }
    let mut m = vec![ANSWER];
    m.extend(port.to_be_bytes());
    m.extend(from.ip().octets());
    m.extend(from.port().to_be_bytes());
    m.extend(payload);
    Some(m)
}

pub fn decode_answer(m: &[u8]) -> Option<(u16, SocketAddrV4, &[u8])> {
    let head = m.get(..9)?;
    let payload = m.get(9..)?;
    let port = u16::from_be_bytes([head[1], head[2]]);
    let from = SocketAddrV4::new(Ipv4Addr::new(head[3], head[4], head[5], head[6]), u16::from_be_bytes([head[7], head[8]]));
    (head[0] == ANSWER && port != 0 && !payload.is_empty() && payload.len() <= PAYLOAD_MAX).then_some((port, from, payload))
}

pub fn answer_ok(from: &SocketAddrV4) -> bool {
    let ip = from.ip();
    from.port() != 0 && !ip.is_multicast() && !ip.is_broadcast() && !ip.is_unspecified() && home_address(&std::net::IpAddr::V4(*ip))
}

pub struct Pace {
    began: Instant,
    seen: u32,
}

impl Pace {
    pub fn new() -> Pace {
        Pace { began: Instant::now(), seen: 0 }
    }

    pub fn allows(&mut self, now: Instant) -> bool {
        if now.duration_since(self.began) >= Duration::from_secs(1) {
            (self.began, self.seen) = (now, 0);
        }
        self.seen += 1;
        self.seen <= ASKS_PER_SECOND
    }
}

fn io(e: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

fn socket_option<T>(fd: RawFd, level: libc::c_int, name: libc::c_int, value: &T) -> std::io::Result<()> {
    // SAFETY: setsockopt reads `size_of::<T>()` bytes from a value that is alive for the call.
    let r = unsafe { libc::setsockopt(fd, level, name, (value as *const T).cast(), std::mem::size_of::<T>() as libc::socklen_t) };
    if r == 0 { Ok(()) } else { Err(std::io::Error::last_os_error()) }
}

pub fn udp_bound(to: SocketAddrV4) -> std::io::Result<UdpSocket> {
    // SAFETY: an ordinary datagram socket; the descriptor is wrapped at once.
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: just made, owned by nobody else.
    let owned = unsafe { OwnedFd::from_raw_fd(fd) };
    socket_option(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, &1i32)?;
    let sa = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: to.port().to_be(),
        sin_addr: libc::in_addr { s_addr: u32::from(*to.ip()).to_be() },
        sin_zero: [0; 8],
    };
    // SAFETY: bind reads one sockaddr_in.
    if unsafe { libc::bind(fd, (&sa as *const libc::sockaddr_in).cast(), std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(UdpSocket::from(owned))
}

fn wait_readable(fds: &[RawFd], most: Duration) -> Vec<bool> {
    let mut polls: Vec<libc::pollfd> = fds.iter().map(|fd| libc::pollfd { fd: *fd, events: libc::POLLIN, revents: 0 }).collect();
    // SAFETY: poll on descriptors that are open for the call.
    let n = unsafe { libc::poll(polls.as_mut_ptr(), polls.len() as libc::nfds_t, most.as_millis().min(i32::MAX as u128) as i32) };
    polls.iter().map(|p| n > 0 && p.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) != 0).collect()
}

fn receive(fd: RawFd, buf: &mut [u8]) -> Option<usize> {
    loop {
        // SAFETY: recv into a buffer that is alive for the call.
        let n = unsafe { libc::recv(fd, buf.as_mut_ptr().cast(), buf.len(), 0) };
        if n > 0 {
            return Some(n as usize);
        }
        if n < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
            continue;
        }
        return None;
    }
}

fn send(fd: RawFd, message: &[u8]) {
    // SAFETY: send from a slice that is alive for the call; a full or closed socket loses the message, which is all right.
    unsafe { libc::send(fd, message.as_ptr().cast(), message.len(), libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL) };
}

pub fn host(link: OwnedFd, interface: Option<&str>) {
    let mut ports: HashMap<u16, (UdpSocket, Instant)> = HashMap::new();
    let mut pace = Pace::new();
    let mut buf = vec![0u8; PAYLOAD_MAX + 16];
    let link_fd = link.as_raw_fd();
    loop {

        let order: Vec<u16> = ports.keys().copied().collect();
        let fds: Vec<RawFd> = std::iter::once(link_fd).chain(order.iter().filter_map(|p| ports.get(p)).map(|(s, _)| s.as_raw_fd())).collect();
        let ready = wait_readable(&fds, Duration::from_secs(1));
        let now = Instant::now();
        if ready[0] {
            let Some(n) = receive(link_fd, &mut buf) else { return };
            if let Some((group, port, payload)) = decode_ask(&buf[..n]) {
                if !pace.allows(now) || (!ports.contains_key(&port) && ports.len() >= PORTS_MOST) {
                    continue;
                }
                if let std::collections::hash_map::Entry::Vacant(e) = ports.entry(port) {
                    match asking_socket(interface) {
                        Ok(s) => {
                            e.insert((s, now));
                        }
                        Err(e) => {
                            eprintln!("trindle-boxed: no socket to ask the network with: {e}");
                            continue;
                        }
                    }
                }
                if let Some((s, last)) = ports.get_mut(&port) {
                    *last = now;
                    let (address, to) = GROUPS[group];
                    let _ = s.send_to(payload, SocketAddrV4::new(address, to));
                }
            }
        }
        for (port, was_ready) in order.iter().zip(ready.iter().skip(1)) {
            let Some((s, _)) = ports.get(port).filter(|_| *was_ready) else { continue };
            let mut got = vec![0u8; PAYLOAD_MAX + 1];
            while let Ok((n, SocketAddr::V4(from))) = s.recv_from(&mut got) {
                if n > 0 && n <= PAYLOAD_MAX && answer_ok(&from) {
                    if let Some(m) = encode_answer(*port, from, &got[..n]) {
                        send(link_fd, &m);
                    }
                }
            }
        }
        ports.retain(|_, (_, last)| now.duration_since(*last) < REMEMBER);
    }
}

fn asking_socket(interface: Option<&str>) -> std::io::Result<UdpSocket> {
    let s = UdpSocket::bind("0.0.0.0:0")?;
    s.set_nonblocking(true)?;
    s.set_multicast_ttl_v4(255)?;
    s.set_multicast_loop_v4(false)?;
    if let Some(name) = interface {
        let bytes = name.as_bytes();
        // SAFETY: setsockopt reads `len` bytes of the name.
        let r = unsafe { libc::setsockopt(s.as_raw_fd(), libc::SOL_SOCKET, libc::SO_BINDTODEVICE, bytes.as_ptr().cast(), bytes.len() as libc::socklen_t) };
        if r != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(s)
}

pub fn udp_packet(from: SocketAddrV4, to: SocketAddrV4, payload: &[u8]) -> Vec<u8> {
    let udp_len = 8 + payload.len();
    let total = 20 + udp_len;
    let mut p = Vec::with_capacity(total);
    p.extend([0x45, 0]);
    p.extend((total as u16).to_be_bytes());
    p.extend([0, 0, 0, 0, 64, libc::IPPROTO_UDP as u8, 0, 0]);
    p.extend(from.ip().octets());
    p.extend(to.ip().octets());
    let sum = checksum(&p[..20]);
    p[10..12].copy_from_slice(&sum.to_be_bytes());
    p.extend(from.port().to_be_bytes());
    p.extend(to.port().to_be_bytes());
    p.extend((udp_len as u16).to_be_bytes());
    p.extend([0, 0]);
    p.extend(payload);
    p
}

fn checksum(header: &[u8]) -> u16 {
    let mut sum: u32 = header.chunks(2).map(|c| u32::from(u16::from_be_bytes([c[0], *c.get(1).unwrap_or(&0)]))).sum();
    while sum > 0xffff {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

fn raw_sender() -> std::io::Result<OwnedFd> {
    // SAFETY: a raw socket; wrapped at once.
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW | libc::SOCK_CLOEXEC, libc::IPPROTO_RAW) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: just made, owned by nobody else.
    let owned = unsafe { OwnedFd::from_raw_fd(fd) };
    socket_option(fd, libc::IPPROTO_IP, libc::IP_HDRINCL, &1i32)?;
    Ok(owned)
}

fn send_raw(fd: RawFd, packet: &[u8], to: Ipv4Addr) {
    let sa = libc::sockaddr_in { sin_family: libc::AF_INET as libc::sa_family_t, sin_port: 0, sin_addr: libc::in_addr { s_addr: u32::from(to).to_be() }, sin_zero: [0; 8] };
    // SAFETY: sendto from a slice and one sockaddr_in, both alive for the call.
    unsafe {
        libc::sendto(fd, packet.as_ptr().cast(), packet.len(), libc::MSG_DONTWAIT, (&sa as *const libc::sockaddr_in).cast(), std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t)
    };
}

fn group_socket(group: Ipv4Addr, port: u16) -> std::io::Result<UdpSocket> {
    let s = udp_bound(SocketAddrV4::new(group, port))?;
    s.set_nonblocking(true)?;
    s.join_multicast_v4(&group, &Ipv4Addr::UNSPECIFIED)?;

    socket_option(s.as_raw_fd(), libc::IPPROTO_IP, libc::IP_MULTICAST_ALL, &0i32)?;
    Ok(s)
}

pub fn room(link: OwnedFd) -> std::io::Result<()> {
    let listening: Vec<UdpSocket> = GROUPS.iter().map(|(g, p)| group_socket(*g, *p)).collect::<std::io::Result<_>>().map_err(|e| io(format!("it cannot listen for questions: {e}")))?;
    let raw = raw_sender().map_err(|e| io(format!("it cannot answer as a device: {e}")))?;
    std::thread::spawn(move || {
        let mut asked_from: HashMap<u16, (Ipv4Addr, Instant)> = HashMap::new();
        let mut buf = vec![0u8; PAYLOAD_MAX + 16];
        let link_fd = link.as_raw_fd();
        loop {
            let fds: Vec<RawFd> = listening.iter().map(|s| s.as_raw_fd()).chain(std::iter::once(link_fd)).collect();
            let ready = wait_readable(&fds, Duration::from_secs(1));
            let now = Instant::now();
            asked_from.retain(|_, (_, at)| now.duration_since(*at) < REMEMBER);
            for (group, s) in listening.iter().enumerate() {
                if !ready[group] {
                    continue;
                }
                let mut got = vec![0u8; PAYLOAD_MAX + 1];
                while let Ok((n, SocketAddr::V4(from))) = s.recv_from(&mut got) {
                    if n == 0 || n > PAYLOAD_MAX || from.port() == 0 {
                        continue;
                    }
                    if !asked_from.contains_key(&from.port()) && asked_from.len() >= PORTS_MOST {
                        continue;
                    }
                    asked_from.insert(from.port(), (*from.ip(), now));
                    if let Some(m) = encode_ask(group, from.port(), &got[..n]) {
                        send(link_fd, &m);
                    }
                }
            }
            if ready[listening.len()] {
                let Some(n) = receive(link_fd, &mut buf) else { return };
                if let Some((port, from, payload)) = decode_answer(&buf[..n]) {
                    if let (true, Some((to, _))) = (answer_ok(&from), asked_from.get(&port)) {
                        send_raw(raw.as_raw_fd(), &udp_packet(from, SocketAddrV4::new(*to, port), payload), *to);
                    }
                }
            }
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests;
