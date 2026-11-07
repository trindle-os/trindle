//! Blocking adverts and bad sites, with private DNS.

use std::path::Path;

pub const SERVICE: &str = "trindle-dns.service";
pub const ADDRESS: &str = "127.0.2.53";
pub const RESOLV: &str = "/run/trindle-dns/resolv.conf";
pub const STATUS: &str = "/run/trindle-dns/status";
pub const ETC_RESOLV: &str = "/etc/resolv.conf";
pub const NAME_MOST: usize = 253;
pub const ALLOW: &str = "/etc/trindle/dns-allow";
pub const PROVIDER: &str = "/etc/trindle/dns-provider";
pub const NM_DROP_IN: &str = "/etc/NetworkManager/conf.d/90-trindle-dns.conf";
pub const NM_CHECK: &str = "http://network-test.debian.org/nm";
pub const NM_SETTINGS: [&str; 5] = [
    "/usr/lib/NetworkManager/conf.d",
    "/run/NetworkManager/conf.d",
    "/etc/NetworkManager/conf.d",
    "/etc/NetworkManager/NetworkManager.conf",
    "/var/lib/NetworkManager/NetworkManager-intern.conf",
];
pub const NETWORK_FILES: [&str; 5] = [
    "/run/NetworkManager/resolv.conf",
    "/run/NetworkManager/no-stub-resolv.conf",
    "/run/systemd/resolve/resolv.conf",
    "/run/resolvconf/resolv.conf",
    "/run/connman/resolv.conf",
];
pub const RESOLV_BEFORE: &str = "/etc/trindle/resolv.conf.before";
pub const RESOLVED_MARK: &str = "/etc/trindle/dns-resolved";
pub const RESOLVED_DROP_IN: &str = "/run/systemd/resolved.conf.d/90-trindle-dns.conf";
pub const TAILSCALE_MARK: &str = "/etc/trindle/dns-tailscale";

pub fn on() -> bool {
    holds_resolv() || Path::new(RESOLVED_MARK).exists()
}

pub fn holds_resolv() -> bool {
    std::fs::read_link(ETC_RESOLV).is_ok_and(|t| t.starts_with("/run/trindle-dns"))
}

pub fn resolved_running() -> bool {
    Path::new("/run/systemd/resolve/io.systemd.Resolve").exists()
        || Path::new("/run/systemd/resolve/stub-resolv.conf").exists()
}

pub fn behind_resolved() -> bool {
    resolved_running() && std::fs::canonicalize(ETC_RESOLV).is_ok_and(|t| t.starts_with("/run/systemd/resolve"))
}

pub fn nm_check_chosen(root: &Path) -> bool {
    let ours = root.join(NM_DROP_IN.trim_start_matches('/'));
    let mut files = Vec::new();
    for place in NM_SETTINGS {
        let p = root.join(place.trim_start_matches('/'));
        match std::fs::read_dir(&p) {
            Ok(dir) => files.extend(dir.filter_map(|e| e.ok()).map(|e| e.path())
                .filter(|f| f.extension().is_some_and(|x| x == "conf") && *f != ours)),
            Err(_) => files.push(p),
        }
    }
    files.iter().filter_map(|f| std::fs::read_to_string(f).ok()).any(|text| {
        let mut inside = false;
        text.lines().map(str::trim).any(|l| {
            if l.starts_with('[') {
                inside = l == "[connectivity]";
                false
            } else {
                inside && !l.starts_with('#') && !l.starts_with(';') && l.contains('=')
            }
        })
    })
}

pub fn nm_drop_in(holds_resolv: bool, check: bool) -> Option<String> {
    if !holds_resolv && !check {
        return None;
    }
    let mut t = String::from("# Written by Trindle while blocking is on in Settings.\n");
    if holds_resolv {
        t.push_str("[main]\nrc-manager=unmanaged\n");
    }
    if check {
        t.push_str(&format!("# So it can say when a network wants a sign-in first.\n[connectivity]\nuri={NM_CHECK}\n"));
    }
    Some(t)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Quad9,
    Mullvad,
    Cloudflare,
    Network,
}

impl Provider {
    pub const ALL: [Provider; 4] = [Provider::Quad9, Provider::Mullvad, Provider::Cloudflare, Provider::Network];

    pub fn key(self) -> &'static str {
        match self {
            Provider::Quad9 => "quad9",
            Provider::Mullvad => "mullvad",
            Provider::Cloudflare => "cloudflare",
            Provider::Network => "network",
        }
    }

    pub fn from_key(key: &str) -> Option<Provider> {
        Provider::ALL.into_iter().find(|p| p.key() == key.trim())
    }

    pub fn label(self) -> &'static str {
        match self {
            Provider::Quad9 => "Quad9",
            Provider::Mullvad => "Mullvad",
            Provider::Cloudflare => "Cloudflare",
            Provider::Network => crate::toolkit::t!("settings-blocking-provider-network"),
        }
    }

    pub fn servers(self) -> (&'static str, &'static [&'static str]) {
        match self {
            Provider::Quad9 => ("dns.quad9.net", &["9.9.9.9", "149.112.112.112", "2620:fe::fe"]),
            Provider::Mullvad => ("dns.mullvad.net", &["194.242.2.2", "2a07:e340::2"]),
            Provider::Cloudflare => ("cloudflare-dns.com", &["1.1.1.1", "1.0.0.1", "2606:4700:4700::1111"]),
            Provider::Network => ("", &[]),
        }
    }
}

pub fn provider() -> Provider {
    std::fs::read_to_string(PROVIDER).ok().and_then(|k| Provider::from_key(&k)).unwrap_or(Provider::Quad9)
}

pub fn allowed() -> Vec<String> {
    allowed_in(&std::fs::read_to_string(ALLOW).unwrap_or_default())
}

pub fn allowed_in(text: &str) -> Vec<String> {
    text.lines().filter_map(site).collect()
}

pub fn site(typed: &str) -> Option<String> {
    let t = typed.trim().to_ascii_lowercase();
    let t = t.strip_prefix("https://").or_else(|| t.strip_prefix("http://")).unwrap_or(&t);
    let t = t.split(['/', '?', '#']).next().unwrap_or("");
    let t = t.strip_prefix("*.").unwrap_or(t).trim_end_matches('.');
    let fine = !t.is_empty() && t.len() <= NAME_MOST
        && t.split('.').all(|l| !l.is_empty() && l.len() <= 63 && !l.starts_with('-'))
        && t.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'));
    fine.then(|| t.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Private {
    Yes,
    Fallback,
    Network,
    Vpn,
    #[default]
    Starting,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Status {
    pub listed: u64,
    pub fetched: u64,
    pub fetching: bool,
    pub today: u64,
    pub private: Private,
    pub recent: Vec<String>,
}

impl Status {
    pub fn write(&self) -> String {
        let private = match self.private {
            Private::Yes => "yes",
            Private::Fallback => "fallback",
            Private::Network => "network",
            Private::Vpn => "vpn",
            Private::Starting => "starting",
        };
        let mut s = format!("listed {}\nfetched {}\nfetching {}\ntoday {}\nprivate {private}\n",
                            self.listed, self.fetched, if self.fetching { "yes" } else { "no" }, self.today);
        for name in &self.recent {
            s.push_str("blocked ");
            s.push_str(name);
            s.push('\n');
        }
        s
    }

    pub fn parse(text: &str) -> Status {
        let mut s = Status::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once(' ') else { continue };
            match key {
                "listed" => s.listed = value.parse().unwrap_or(0),
                "fetched" => s.fetched = value.parse().unwrap_or(0),
                "fetching" => s.fetching = value == "yes",
                "today" => s.today = value.parse().unwrap_or(0),
                "private" => s.private = match value {
                    "yes" => Private::Yes,
                    "fallback" => Private::Fallback,
                    "network" => Private::Network,
                    "vpn" => Private::Vpn,
                    _ => Private::Starting,
                },
                "blocked" => s.recent.push(value.to_string()),
                _ => {}
            }
        }
        s
    }
}

pub fn status() -> Option<Status> {
    if !Path::new(STATUS).exists() {
        return None;
    }
    std::fs::read_to_string(STATUS).ok().map(|t| Status::parse(&t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sites_as_typed() {
        assert_eq!(site("Example.com").as_deref(), Some("example.com"));
        assert_eq!(site(" https://Ads.Example.com/page?x=1 ").as_deref(), Some("ads.example.com"));
        assert_eq!(site("*.example.com.").as_deref(), Some("example.com"));
        assert_eq!(site("_dmarc.example.com").as_deref(), Some("_dmarc.example.com"));
        assert_eq!(site(""), None);
        assert_eq!(site("has space.com"), None);
        assert_eq!(site("a..b"), None);
        assert_eq!(site("-bad.com"), None);
        assert_eq!(site(&"a".repeat(64)), None);
        assert_eq!(allowed_in("one.com\n\nnot valid!\nTwo.org\n"), ["one.com", "two.org"]);
    }

    #[test]
    fn status_round_trip() {
        let s = Status { listed: 905_000, fetched: 1_791_100_000, fetching: true, today: 42,
                         private: Private::Fallback, recent: vec!["ads.example.com".into(), "track.example.org".into()] };
        assert_eq!(Status::parse(&s.write()), s);
        assert_eq!(Status::parse("odd\nlisted x\n"), Status::default());
    }

    #[test]
    fn network_manager_checks_for_sign_in() {
        let root = std::env::temp_dir().join(format!("trindle-dns-nm-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let put = |path: &str, text: &str| {
            let p = root.join(path.trim_start_matches('/'));
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        };
        put("/etc/NetworkManager/NetworkManager.conf", "[main]\nplugins=ifupdown,keyfile\n\n[ifupdown]\nmanaged=false\n");
        put("/usr/lib/NetworkManager/conf.d/no-mac-addr-change.conf", "[device]\nwifi.scan-rand-mac-address=no\n");
        assert!(!nm_check_chosen(&root));
        put(NM_DROP_IN, &nm_drop_in(true, true).unwrap());
        assert!(!nm_check_chosen(&root));
        put("/etc/NetworkManager/conf.d/50-mine.conf", "[connectivity]\n# uri=http://example.com/\n");
        assert!(!nm_check_chosen(&root));
        for (path, text) in [("/usr/lib/NetworkManager/conf.d/20-connectivity-debian.conf", "[connectivity]\nuri=http://network-test.debian.org/nm\n"),
                             ("/var/lib/NetworkManager/NetworkManager-intern.conf", "[connectivity]\n.set.enabled=false\n")] {
            put(path, text);
            assert!(nm_check_chosen(&root), "{path}");
            std::fs::remove_file(root.join(path.trim_start_matches('/'))).unwrap();
        }
        let t = nm_drop_in(true, true).unwrap();
        assert!(t.contains("[main]\nrc-manager=unmanaged\n") && t.contains("[connectivity]\nuri=http://network-test.debian.org/nm\n"));
        let t = nm_drop_in(false, true).unwrap();
        assert!(!t.contains("[main]") && t.contains("[connectivity]"));
        assert!(!nm_drop_in(true, false).unwrap().contains("[connectivity]"));
        assert_eq!(nm_drop_in(false, false), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn providers() {
        for p in Provider::ALL {
            assert_eq!(Provider::from_key(p.key()), Some(p));
        }
        assert_eq!(Provider::from_key("quad9\n"), Some(Provider::Quad9));
        assert_eq!(Provider::from_key("other"), None);
    }
}
