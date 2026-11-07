//! Finds the network's own DNS servers on any computer.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV6, UdpSocket};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::common::dns as shared;

const NETWORKD_LINKS: &str = "/run/systemd/netif/links";
const MOST_WRITTEN: usize = 3;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Network {
    pub servers: Vec<SocketAddr>,
    pub written: Vec<String>,
    pub search: Vec<String>,
    pub routes: Vec<String>,
}

fn ours(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_unspecified() || [[127, 0, 2, 53], [127, 0, 0, 53], [127, 0, 0, 54]].contains(&v4.octets()),
        IpAddr::V6(v6) => v6.is_unspecified(),
    }
}

fn interface_index(name: &str) -> Option<u32> {
    if let Ok(n) = name.parse::<u32>() {
        return (n > 0).then_some(n);
    }
    let c = std::ffi::CString::new(name).ok()?;
    // SAFETY: if_nametoindex only reads the NUL-terminated name given, which
    // lives until the call returns.
    let n = unsafe { libc::if_nametoindex(c.as_ptr()) };
    (n > 0).then_some(n)
}

fn server(word: &str, link: Option<u32>) -> Option<SocketAddr> {
    let word = word.split('#').next()?;
    let (host, port) = if let Some(rest) = word.strip_prefix('[') {
        let (h, p) = rest.split_once(']')?;
        (h, p.strip_prefix(':').map_or(Some(53), |p| p.parse().ok())?)
    } else if word.matches(':').count() == 1 {
        let (h, p) = word.split_once(':')?;
        (h, p.parse().ok()?)
    } else {
        (word, 53)
    };
    let (ip, scope) = match host.split_once('%') {
        Some((ip, ifname)) => (ip.parse::<IpAddr>().ok()?, Some(interface_index(ifname)?)),
        None => (host.parse::<IpAddr>().ok()?, None),
    };
    if ours(ip) {
        return None;
    }
    match ip {
        IpAddr::V6(v6) if v6.segments()[0] & 0xffc0 == 0xfe80 => {
            Some(SocketAddr::V6(SocketAddrV6::new(v6, port, 0, scope.or(link)?)))
        }
        ip => Some(SocketAddr::new(ip, port)),
    }
}

fn add_names(to: &mut Vec<String>, words: &[&str]) {
    for w in words {
        if let Some(d) = shared::site(w.trim_start_matches('~')) {
            if !to.contains(&d) {
                to.push(d);
            }
        }
    }
}

fn add_server(n: &mut Network, word: &str, link: Option<u32>) {
    if let Some(sa) = server(word, link) {
        if !n.servers.contains(&sa) {
            n.servers.push(sa);
            n.written.push(word.split('#').next().unwrap_or(word).to_string());
        }
    }
}

pub fn parse_resolv(text: &str) -> Network {
    let mut n = Network::default();
    for line in text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.first().copied() {
            Some("nameserver") => {
                if let Some(w) = words.get(1) {
                    add_server(&mut n, w, None);
                }
            }
            Some("search") | Some("domain") => add_names(&mut n.search, &words[1..]),
            _ => {}
        }
    }
    n
}

fn parse_networkd(text: &str, link: Option<u32>, into: &mut Network) {
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else { continue };
        let words: Vec<&str> = value.split_whitespace().collect();
        match key {
            "DNS" => {
                for w in &words {
                    add_server(into, w, link);
                }
            }
            "DOMAINS" => add_names(&mut into.search, &words),
            "ROUTE_DOMAINS" => add_names(&mut into.routes, &words),
            _ => {}
        }
    }
}

fn under(root: &Path, path: &str) -> PathBuf {
    root.join(path.trim_start_matches('/'))
}

fn modified(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

fn networkd(root: &Path) -> Option<(SystemTime, Network)> {
    let mut entries: Vec<(u32, PathBuf)> = std::fs::read_dir(under(root, NETWORKD_LINKS)).ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| Some((e.file_name().to_str()?.parse().ok()?, e.path())))
        .collect();
    entries.sort();
    let mut n = Network::default();
    let mut newest = None;
    for (link, path) in entries {
        let t = modified(&path);
        newest = newest.max(t);
        parse_networkd(&std::fs::read_to_string(&path).unwrap_or_default(), Some(link), &mut n);
    }
    Some((newest?, n))
}

fn before(root: &Path) -> Network {
    let kept = under(root, shared::RESOLV_BEFORE);
    let path = match std::fs::read_link(&kept) {
        Ok(t) if t.is_relative() => under(root, "/etc").join(t),
        Ok(t) => under(root, &t.to_string_lossy()),
        Err(_) => kept,
    };
    parse_resolv(&std::fs::read_to_string(path).unwrap_or_default())
}

pub fn find(root: &Path, through: Option<&(SystemTime, Network)>) -> Network {
    let mut best: Option<(SystemTime, Network)> = None;
    let mut offer = |t: Option<SystemTime>, n: Network| {
        let Some(t) = t else { return };
        if !n.servers.is_empty() && best.as_ref().is_none_or(|(bt, _)| t > *bt) {
            best = Some((t, n));
        }
    };
    for f in shared::NETWORK_FILES {
        let path = under(root, f);
        if let Ok(text) = std::fs::read_to_string(&path) {
            offer(modified(&path), parse_resolv(&text));
        }
    }
    if let Some((t, n)) = networkd(root) {
        offer(Some(t), n);
    }
    if let Some((t, n)) = through {
        offer(Some(*t), n.clone());
    }
    best.map_or_else(|| before(root), |(_, n)| n)
}

pub fn fallback_text(n: &Network) -> String {
    let mut t = String::from("# Written by trindle-dns, which cannot answer just now, so lookups go\n\
                              # straight to the network's own servers.\n");
    if n.written.is_empty() {
        t.push_str(&format!("nameserver {}\n", shared::ADDRESS));
    }
    for w in n.written.iter().filter(|w| !w.contains('[') && w.matches(':').count() != 1).take(MOST_WRITTEN) {
        t.push_str(&format!("nameserver {w}\n"));
    }
    if !n.search.is_empty() {
        t.push_str(&format!("search {}\n", n.search.join(" ")));
    }
    t
}

pub fn interfaces() -> Vec<(String, IpAddr)> {
    let mut found = Vec::new();
    // SAFETY: getifaddrs gives a list we only read, freed once with
    // freeifaddrs; each address is read only after its family is checked.
    unsafe {
        let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut head) != 0 {
            return found;
        }
        let mut at = head;
        while !at.is_null() {
            let i = &*at;
            if !i.ifa_addr.is_null() && !i.ifa_name.is_null() {
                let name = std::ffi::CStr::from_ptr(i.ifa_name).to_string_lossy().into_owned();
                match (*i.ifa_addr).sa_family as i32 {
                    libc::AF_INET => {
                        let sin = &*(i.ifa_addr as *const libc::sockaddr_in);
                        found.push((name, IpAddr::V4(Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr)))));
                    }
                    libc::AF_INET6 => {
                        let sin6 = &*(i.ifa_addr as *const libc::sockaddr_in6);
                        found.push((name, IpAddr::from(sin6.sin6_addr.s6_addr)));
                    }
                    _ => {}
                }
            }
            at = i.ifa_next;
        }
        libc::freeifaddrs(head);
    }
    found
}

fn interface_for(to: SocketAddr, ifs: &[(String, IpAddr)]) -> Option<String> {
    let any: SocketAddr = if to.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" }.parse().ok()?;
    let s = UdpSocket::bind(any).ok()?;
    s.connect(to).ok()?;
    let from = s.local_addr().ok()?.ip();
    ifs.iter().find(|(_, ip)| *ip == from).map(|(name, _)| name.clone())
}

pub fn is_vpn(name: &str, sys: &Path) -> bool {
    if name == "tailscale0" || name == "lo" {
        return false;
    }
    let dir = sys.join(name);
    dir.join("tun_flags").exists()
        || std::fs::read_to_string(dir.join("type")).is_ok_and(|t| t.trim() == "65534")
}

pub fn through_vpn(servers: &[SocketAddr], ifs: &[(String, IpAddr)], sys: &Path) -> Vec<SocketAddr> {
    servers.iter().copied().filter(|s| !s.ip().is_loopback())
        .filter(|s| interface_for(*s, ifs).is_some_and(|i| is_vpn(&i, sys))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn ip(s: &str) -> SocketAddr {
        SocketAddr::new(s.parse().unwrap(), 53)
    }

    #[test]
    fn resolv_files() {
        let n = parse_resolv("# NM\nsearch Home.lan corp.example\nnameserver 192.168.1.1\nnameserver 127.0.0.53\n\
                              nameserver 127.0.2.53\nnameserver fe80::1%lo\nnameserver fe80::2\nnameserver 127.0.0.1\n\
                              nameserver 2001:db8::53\nnameserver 192.168.1.1\n");
        assert_eq!(n.servers[0], ip("192.168.1.1"));
        assert!(matches!(n.servers[1], SocketAddr::V6(v6) if v6.scope_id() == 1 && v6.port() == 53));
        assert_eq!(&n.servers[2..], [ip("127.0.0.1"), ip("2001:db8::53")]);
        assert_eq!(n.written, ["192.168.1.1", "fe80::1%lo", "127.0.0.1", "2001:db8::53"]);
        assert_eq!(n.search, ["home.lan", "corp.example"]);
        assert_eq!(server("[2001:db8::1]:5353#dns.example", None), Some("[2001:db8::1]:5353".parse().unwrap()));
        assert_eq!(server("192.0.2.1:5353", None), Some("192.0.2.1:5353".parse().unwrap()));
        assert_eq!(server("fe80::1%no-such-interface", None), None);
        assert_eq!(server("0.0.0.0", None), None);
    }

    #[test]
    fn networkd_links() {
        let mut n = Network::default();
        parse_networkd("# This is private data.\nADMIN_STATE=configured\nDNS=192.0.2.53 fe80::1\n\
                        DOMAINS=office.example\nROUTE_DOMAINS=~corp.example ~.\n", Some(7), &mut n);
        assert_eq!(n.servers[0], ip("192.0.2.53"));
        assert!(matches!(n.servers[1], SocketAddr::V6(v6) if v6.scope_id() == 7));
        assert_eq!((n.search.as_slice(), n.routes.as_slice()), (&["office.example".to_string()][..], &["corp.example".to_string()][..]));
    }

    fn root(name: &str) -> PathBuf {
        let r = std::env::temp_dir().join(format!("trindle-dns-net-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&r);
        std::fs::create_dir_all(&r).unwrap();
        r
    }

    fn put(root: &Path, path: &str, text: &str, age: u64) {
        let p = under(root, path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, text).unwrap();
        let t = SystemTime::now() - Duration::from_secs(age);
        std::fs::File::options().write(true).open(&p).unwrap().set_modified(t).unwrap();
    }

    #[test]
    fn every_kind_of_computer() {
        let r = root("none");
        assert_eq!(find(&r, None), Network::default());
        put(&r, "/etc/static-resolv.conf", "nameserver 192.0.2.1\n", 100);
        std::fs::create_dir_all(under(&r, "/etc/trindle")).unwrap();
        std::os::unix::fs::symlink("static-resolv.conf", under(&r, shared::RESOLV_BEFORE)).unwrap();
        assert_eq!(find(&r, None).servers, [ip("192.0.2.1")]);
        put(&r, "/run/NetworkManager/resolv.conf", "nameserver 127.0.0.53\nsearch lan\n", 50);
        put(&r, "/run/NetworkManager/no-stub-resolv.conf", "nameserver 192.168.1.1\nsearch lan\n", 50);
        assert_eq!(find(&r, None).servers, [ip("192.168.1.1")]);
        put(&r, "/run/systemd/resolve/resolv.conf", "nameserver 127.0.2.53\nnameserver 192.168.1.2\n", 40);
        assert_eq!(find(&r, None).servers, [ip("192.168.1.2")]);
        put(&r, "/run/resolvconf/resolv.conf", "nameserver 10.64.0.1\n", 30);
        assert_eq!(find(&r, None).servers, [ip("10.64.0.1")]);
        put(&r, "/run/connman/resolv.conf", "nameserver 127.0.0.1\n", 20);
        assert_eq!(find(&r, None).servers, [ip("127.0.0.1")]);
        put(&r, "/run/systemd/netif/links/3", "DNS=192.0.2.9\nROUTE_DOMAINS=~corp.example\n", 10);
        let n = find(&r, None);
        assert_eq!((n.servers.as_slice(), n.routes.as_slice()), (&[ip("192.0.2.9")][..], &["corp.example".to_string()][..]));
        let vpn = (SystemTime::now(), parse_resolv("nameserver 10.8.0.1\nsearch corp.example\n"));
        assert_eq!(find(&r, Some(&vpn)).servers, [ip("10.8.0.1")]);
        put(&r, "/run/NetworkManager/resolv.conf", "nameserver 172.16.0.1\n", 0);
        let later = (SystemTime::now() - Duration::from_secs(5), vpn.1);
        assert_eq!(find(&r, Some(&later)).servers, [ip("172.16.0.1")]);
        let _ = std::fs::remove_dir_all(&r);
    }

    #[test]
    fn written_while_down() {
        let mut n = parse_resolv("nameserver 192.168.1.1\nnameserver fe80::1%lo\nnameserver 10.0.0.1\nnameserver 10.0.0.2\nsearch lan\n");
        let t = fallback_text(&n);
        assert!(t.starts_with("# Written by trindle-dns"));
        assert!(t.contains("nameserver 192.168.1.1\nnameserver fe80::1%lo\nnameserver 10.0.0.1\nsearch lan\n"));
        assert!(!t.contains("10.0.0.2"));
        n = Network::default();
        assert!(fallback_text(&n).contains("nameserver 127.0.2.53\n"));
    }

    #[test]
    fn vpns() {
        let r = root("sys");
        for (name, kind, tun) in [("wg0", "65534", false), ("tun0", "65534", true), ("tap0", "1", true),
                                  ("wlan0", "1", false), ("ppp0", "512", false), ("tailscale0", "65534", true)] {
            put(&r, &format!("/{name}/type"), &format!("{kind}\n"), 0);
            if tun {
                put(&r, &format!("/{name}/tun_flags"), "0x1001\n", 0);
            }
        }
        let vpn: Vec<&str> = ["wg0", "tun0", "tap0", "wlan0", "ppp0", "tailscale0", "missing"].into_iter()
            .filter(|n| is_vpn(n, &r)).collect();
        assert_eq!(vpn, ["wg0", "tun0", "tap0"]);
        assert_eq!(interface_for(ip("127.0.0.1"), &interfaces()).as_deref(), Some("lo"));
        assert!(through_vpn(&[ip("127.0.0.1")], &interfaces(), &r).is_empty());
        let _ = std::fs::remove_dir_all(&r);
    }
}
