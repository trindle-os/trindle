//! The one rule for the address a backup restores apps from.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const HELP: &str = "apps#a-backup-link-is-not-encrypted";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {

    Encrypted,

    Home,

    Internet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {

    Nothing,

    Line,

    Card,
}

fn own_name() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname").unwrap_or_default().trim().to_ascii_lowercase()
}

fn http_host(url: &str) -> Option<String> {
    let rest = url.get(..7).filter(|s| s.eq_ignore_ascii_case("http://")).map(|_| &url[7..])?;
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = match host.strip_prefix('[') {
        Some(v6) => v6.split(']').next().unwrap_or(""),
        None => host.split(':').next().unwrap_or(""),
    };
    let host = host.split('%').next().unwrap_or("").trim_end_matches('.').to_ascii_lowercase();
    Some(host).filter(|h| !h.is_empty())
}

fn home_v4(a: Ipv4Addr) -> bool {
    a.is_loopback() || a.is_private() || a.is_link_local()
}

fn home_v6(a: Ipv6Addr) -> bool {
    if let Some(v4) = a.to_ipv4_mapped() {
        return home_v4(v4);
    }
    let first = a.segments()[0];
    a.is_loopback() || first & 0xfe00 == 0xfc00 || first & 0xffc0 == 0xfe80
}

fn home_host(host: &str, own: &str) -> bool {
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(a)) => home_v4(a),
        Ok(IpAddr::V6(a)) => home_v6(a),
        Err(_) => host == "localhost" || host.ends_with(".localhost") || host.ends_with(".local") || (!own.is_empty() && host == own),
    }
}

fn link_with(url: &str, own: &str) -> Link {
    match http_host(url) {
        None => Link::Encrypted,
        Some(h) if home_host(&h, own) => Link::Home,
        Some(_) => Link::Internet,
    }
}

pub fn link_of(url: &str) -> Link {
    link_with(url, &own_name())
}

fn rule_with<'a>(urls: impl IntoIterator<Item = &'a str>, own: &str) -> Rule {
    urls.into_iter().fold(Rule::Nothing, |worst, u| worst.max(match link_with(u, own) {
        Link::Encrypted => Rule::Nothing,
        Link::Home => Rule::Line,
        Link::Internet => Rule::Card,
    }))
}

pub fn rule<'a>(urls: impl IntoIterator<Item = &'a str>) -> Rule {
    rule_with(urls, &own_name())
}

pub fn allowed(url: &str, ticked: bool) -> bool {
    link_of(url) != Link::Internet || ticked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_of_host_is_told_apart() {
        let own = "mybox";
        for host in ["localhost", "127.0.0.1", "127.9.9.9", "[::1]", "10.0.0.5", "10.255.255.255", "172.16.0.1", "172.31.255.1", "192.168.1.20",
                     "169.254.3.4", "[fc00::1]", "[fd12:3456::9]", "[fe80::1]", "[fe80::1%eth0]", "[febf::1]", "printer.local", "Printer.LOCAL.",
                     "mybox", "MyBox:8080", "[::ffff:192.168.1.2]", "user:pw@192.168.0.9"] {
            assert_eq!(link_with(&format!("http://{host}/repo"), own), Link::Home, "{host}");
        }
        for host in ["example.com", "8.8.8.8", "172.15.0.1", "172.32.0.1", "11.0.0.1", "192.169.1.1", "169.255.0.1", "[2001:db8::1]", "[fec0::1]",
                     "[fbff::1]", "10.0.0.1.example.com", "192.168.1.1.evil.example", "mybox.example.com", "notlocal", "2130706433", "0x7f.1",
                     "[::ffff:8.8.8.8]", "localhost.example.com", "example.com@192.168.1.1.example.org"] {
            assert_eq!(link_with(&format!("http://{host}/repo"), own), Link::Internet, "{host}");
        }
    }

    #[test]
    fn https_and_non_addresses_are_untouched() {
        for u in ["https://example.com/repo", "https://192.168.1.2/repo", "HTTPS://example.com", "ftp://example.com", "", "http://"] {
            assert_eq!(link_with(u, "mybox"), Link::Encrypted, "{u}");
        }
        assert_eq!(link_with("HTTP://Example.com/x", "mybox"), Link::Internet);
    }

    #[test]
    fn the_worst_address_decides_what_is_shown() {
        let home = "http://192.168.1.2/r";
        let net = "http://example.com/r";
        let safe = "https://example.com/r";
        assert_eq!(rule_with([safe], "x"), Rule::Nothing);
        assert_eq!(rule_with([], "x"), Rule::Nothing);
        assert_eq!(rule_with([safe, home], "x"), Rule::Line);
        assert_eq!(rule_with([home, net, safe], "x"), Rule::Card);
        assert_eq!(rule_with([net], "x"), Rule::Card);
    }

    #[test]
    fn only_the_internet_waits_for_the_tick() {
        assert!(allowed("https://example.com/r", false));
        assert!(allowed("http://192.168.1.2/r", false));
        assert!(!allowed("http://example.com/r", false));
        assert!(allowed("http://example.com/r", true));
    }
}
