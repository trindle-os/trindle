//! The firewall.

use std::fmt::Write as _;
use std::fmt;
use std::path::Path;

pub const SHARING: &str = "/etc/trindle/firewall-sharing";
pub const PLUGINS: &str = "/etc/trindle/firewall-plugins";

pub const MAX_LISTEN: usize = 4;
pub const MAX_OPEN: usize = 16;
pub const LOWEST_PORT: u16 = 1024;
pub const RESERVED_PORTS: &[u16] = &[
    1433, 1900, 2049, 3306, 3389, 3702, 5060, 5353, 5357, 5432, 6379, 8080, 8443, 9100, 27017, 41641,
];
const RESERVED_RANGES: &[(u16, u16)] = &[(5900, 5999)];

const LOCAL4: &str = "{ 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 169.254.0.0/16 }";
const LOCAL6: &str = "{ fe80::/10, fc00::/7 }";

pub fn reserved(port: u16) -> bool {
    port < LOWEST_PORT || RESERVED_PORTS.contains(&port) || RESERVED_RANGES.iter().any(|(a, b)| (*a..=*b).contains(&port))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Proto {
    Tcp,
    Udp,
}

impl Proto {
    fn word(self) -> &'static str {
        match self {
            Proto::Tcp => "tcp",
            Proto::Udp => "udp",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Door {
    pub proto: Proto,
    pub port: u16,
}

impl fmt::Display for Door {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.proto.word(), self.port)
    }
}

impl Door {
    pub fn parse(text: &str) -> Result<Door, String> {
        let Some((proto, port)) = text.split_once(':') else {
            return Err(format!("{:?} is not written as protocol:port, such as tcp:24810.", text.chars().take(24).collect::<String>()));
        };
        let proto = match proto {
            "tcp" => Proto::Tcp,
            "udp" => Proto::Udp,
            other => return Err(format!("{:?} is not a protocol there is here: use tcp or udp, in small letters.", other.chars().take(12).collect::<String>())),
        };
        let plain = !port.is_empty() && port.len() <= 5 && port.bytes().all(|b| b.is_ascii_digit()) && !port.starts_with('0');
        let port = match port.parse::<u32>() {
            Ok(n) if plain && n >= u32::from(LOWEST_PORT) && n <= 65535 => n as u16,
            _ => return Err(format!("{:?} is not a port from {LOWEST_PORT} to 65535.", port.chars().take(12).collect::<String>())),
        };
        Ok(Door { proto, port })
    }

    pub fn words(&self) -> String {
        format!("{} {}", self.proto.word().to_uppercase(), self.port)
    }
}

pub fn ports_words(doors: &[Door]) -> String {
    let each: Vec<String> = doors.iter().map(Door::words).collect();
    let list = match each.split_last() {
        None => return String::new(),
        Some((last, [])) => last.clone(),
        Some((last, first)) => format!("{} and {last}", first.join(", ")),
    };
    format!("{} {list}", if doors.len() == 1 { "Port" } else { "Ports" })
}

pub fn vet(doors: &[Door]) -> Result<(), String> {
    if doors.len() > MAX_LISTEN {
        return Err(format!("A plugin may name at most {MAX_LISTEN} ports."));
    }
    for (i, d) in doors.iter().enumerate() {
        if doors[..i].contains(d) {
            return Err(format!("{} is named twice.", d.words()));
        }
        if d.port < LOWEST_PORT {
            return Err(format!("{} is below {LOWEST_PORT}, which the system keeps.", d.words()));
        }
        if reserved(d.port) {
            return Err(format!("Port {} is one Trindle or a common service uses, so a plugin cannot ask for it.", d.port));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Held {
    pub doors: Vec<Door>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entitled {
    pub uid: u32,
    pub id: String,
}

pub fn parse_entitled(text: &str) -> Vec<Entitled> {
    let mut all: Vec<Entitled> = Vec::new();
    for l in text.lines() {
        let mut f = l.split('\t');
        let (Some(uid), Some(id), None) = (f.next(), f.next(), f.next()) else { continue };
        let Ok(uid) = uid.parse() else { continue };
        let e = Entitled { uid, id: id.to_string() };
        if !id.is_empty() && !all.contains(&e) {
            all.push(e);
        }
    }
    all
}

pub fn entitled_text(all: &[Entitled]) -> String {
    all.iter().fold(String::new(), |mut s, e| { let _ = writeln!(s, "{}\t{}", e.uid, e.id); s })
}

pub fn entitled() -> Vec<Entitled> {
    parse_entitled(&std::fs::read_to_string(PLUGINS).unwrap_or_default())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Bad(String),
    Taken(Door, String),
    Full,
}

impl Refusal {
    pub fn words(&self, name: &dyn Fn(&str) -> String) -> String {
        match self {
            Refusal::Bad(why) => why.clone(),
            Refusal::Taken(d, id) => format!("{} is already used by {}, so it cannot be opened for this plugin too.", d.words(), name(id)),
            Refusal::Full => format!("The most ports that may be open for plugins, {MAX_OPEN}, are open already."),
        }
    }
}

pub fn derive(all: &[Entitled], read: &dyn Fn(&Entitled) -> Result<Vec<Door>, String>) -> Vec<(Entitled, Result<Vec<Door>, Refusal>)> {
    let mut taken: Vec<(Door, String)> = Vec::new();
    all.iter().map(|e| {
        let judged = (|| {
            let doors = read(e).map_err(Refusal::Bad)?;
            vet(&doors).map_err(Refusal::Bad)?;
            if let Some((d, other)) = doors.iter().find_map(|d| taken.iter().find(|(t, _)| t == d).map(|(t, o)| (*t, o.clone()))) {
                return Err(Refusal::Taken(d, other));
            }
            if taken.len() + doors.len() > MAX_OPEN {
                return Err(Refusal::Full);
            }
            Ok(doors)
        })();
        if let Ok(doors) = &judged {
            taken.extend(doors.iter().map(|d| (*d, e.id.clone())));
        }
        (e.clone(), judged)
    }).collect()
}

pub fn held_of(derived: &[(Entitled, Result<Vec<Door>, Refusal>)]) -> Vec<Held> {
    derived.iter().filter_map(|(_, r)| match r {
        Ok(d) if !d.is_empty() => Some(Held { doors: d.clone() }),
        _ => None,
    }).collect()
}

pub fn with(all: &[Entitled], uid: u32, id: &str) -> Vec<Entitled> {
    let mut v = all.to_vec();
    let e = Entitled { uid, id: id.to_string() };
    if !v.contains(&e) {
        v.push(e);
    }
    v
}

pub fn without(all: &[Entitled], uid: u32, id: &str) -> Vec<Entitled> {
    all.iter().filter(|e| !(e.uid == uid && e.id == id)).cloned().collect()
}

pub fn sharing() -> bool {
    Path::new(SHARING).exists()
}

pub fn ssh_wanted() -> bool {
    ["/etc/systemd/system/multi-user.target.wants/ssh.service",
     "/etc/systemd/system/sockets.target.wants/ssh.socket"]
        .iter().any(|p| Path::new(p).exists())
}

pub fn rules(sharing: bool, shared: bool, ssh: bool, plugins: &[Held]) -> String {
    let mut accept = always_let_in();
    if ssh {
        accept.push("tcp dport 22 accept".into());
    }
    if sharing {
        for (family, local) in [("ip", LOCAL4), ("ip6", LOCAL6)] {
            accept.push(format!("{family} saddr {local} tcp dport 631 accept"));
            accept.push(format!("{family} saddr {local} udp dport 631 accept"));
        }
    }
    if sharing && shared {
        for (family, local) in [("ip", LOCAL4), ("ip6", LOCAL6)] {
            accept.push(format!("{family} saddr {local} tcp dport {{ 445, 5357 }} accept"));
            accept.push(format!("{family} saddr {local} udp dport 3702 accept"));
        }
    }
    accept.extend(plugin_rules(plugins));
    let body = accept.iter().fold(String::new(), |b, r| b + "\t\t" + r + "\n");
    format!("table inet trindle {{}}\ndelete table inet trindle\ntable inet trindle {{\n\
             \tchain input {{\n\t\ttype filter hook input priority filter; policy drop;\n{body}\t}}\n}}\n")
}

fn always_let_in() -> Vec<String> {
    vec![
        "ct state established,related accept".to_string(),
        "ct state invalid drop".into(),
        "iif lo accept".into(),
        "meta l4proto ipv6-icmp accept".into(),
        "icmp type { echo-request, destination-unreachable, time-exceeded, parameter-problem } accept".into(),
        "udp sport 67 udp dport 68 accept".into(),
        format!("ip6 saddr {LOCAL6} udp dport 546 accept"),
        "udp dport 5353 accept".into(),
        format!("ip saddr {LOCAL4} udp sport {{ 1900, 3702 }} accept"),
        format!("ip6 saddr {LOCAL6} udp sport {{ 1900, 3702 }} accept"),
        "iifname \"tailscale0\" accept".into(),
        "udp dport 41641 accept".into(),
        "iifname { \"virbr*\", \"docker0\", \"br-*\", \"incusbr*\", \"lxdbr*\", \"podman*\" } accept".into(),
    ]
}

fn plugin_rules(plugins: &[Held]) -> Vec<String> {
    let mut accept = Vec::new();
    let mut doors: Vec<Door> = plugins.iter().flat_map(|h| h.doors.iter().copied())
        .filter(|d| d.port >= LOWEST_PORT && !reserved(d.port)).collect();
    doors.sort();
    doors.dedup();
    doors.truncate(MAX_OPEN);
    for (family, local) in [("ip", LOCAL4), ("ip6", LOCAL6)] {
        for proto in [Proto::Tcp, Proto::Udp] {
            let ports: Vec<String> = doors.iter().filter(|d| d.proto == proto).map(|d| d.port.to_string()).collect();
            match ports.as_slice() {
                [] => {}
                [one] => accept.push(format!("{family} saddr {local} {} dport {one} accept", proto.word())),
                many => accept.push(format!("{family} saddr {local} {} dport {{ {} }} accept", proto.word(), many.join(", "))),
            }
        }
    }
    accept
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(text: &str) -> Door {
        Door::parse(text).unwrap()
    }

    fn held(doors: &[&str]) -> Held {
        Held { doors: doors.iter().map(|t| d(t)).collect() }
    }

    fn lines_for(r: &str, word: &str) -> Vec<String> {
        r.lines().map(str::trim).filter(|l| l.contains(word)).map(String::from).collect()
    }

    #[test]
    fn keeps_out_what_was_not_asked_for() {
        let r = rules(false, false, false, &[]);
        assert!(r.contains("policy drop;"));
        assert!(r.contains("ct state established,related accept"));
        assert!(!r.contains("445"));
        assert!(!r.contains("dport 22"));
        assert!(r.starts_with("table inet trindle {}\ndelete table inet trindle\n"));
    }

    #[test]
    fn sharing_and_ssh_open_only_their_own_doors() {
        let r = rules(true, true, true, &[]);
        assert!(r.contains("tcp dport 631 accept"));
        assert!(r.contains("tcp dport 22 accept"));
        assert_eq!(r.matches("policy drop;").count(), 1);
        assert!(!r.contains("139") && !r.contains("137"));
    }

    #[test]
    fn smb_is_open_only_while_shared_and_allowed_and_only_at_home() {
        let smb = |r: &str| r.lines().map(str::trim).filter(|l| l.contains("445") || l.contains("dport 3702")).map(String::from).collect::<Vec<_>>();
        assert!(smb(&rules(true, false, false, &[])).is_empty());
        assert!(smb(&rules(false, true, true, &[])).is_empty());
        let lines = smb(&rules(true, true, false, &[]));
        assert_eq!(lines.len(), 4, "{lines:?}");
        assert!(lines.iter().all(|l| l.contains("saddr") && (l.contains("192.168.0.0/16") || l.contains("fe80::/10"))), "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("tcp dport { 445, 5357 } accept")));
    }

    #[test]
    fn doors_are_read_strictly() {
        assert_eq!(d("tcp:24810"), Door { proto: Proto::Tcp, port: 24810 });
        assert_eq!(d("udp:1024").port, 1024);
        assert_eq!(d("udp:65535").port, 65535);
        for bad in ["tcp", "tcp:", ":80", "icmp:2000", "TCP:2000", "tcp:1023", "tcp:80", "tcp:0", "udp:65536", "tcp:-1", "tcp:12x", "tcp:2000:1"] {
            assert!(Door::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_hostile_manifests_odd_ports_and_protocols_are_all_refused() {
        let bad = [
            "tcp:99999999999999999999999999999", "tcp:4294967296", "tcp:4294969320", "tcp:18446744073709551616", "tcp:65536",
            "tcp:-24810", "tcp:+24810", "tcp:024810", "tcp:0x60ea", "tcp:2.4e4", "tcp:24810.0", "tcp: 24810", "tcp:24 810", "tcp:1e4",
            "tcp:", "tcp:\u{0}", "tcp:24810\u{0}", "tcp:24810\n", "tcp:24810\ttcp:24811",
            "tcp:\u{662}\u{664}\u{668}\u{661}\u{660}", "tcp:\u{ff12}\u{ff14}\u{ff18}\u{ff11}\u{ff10}", "\u{ff54}\u{ff43}\u{ff50}:24810",
            "t\u{0441}p:24810", "tcp\u{200b}:24810", "\u{202e}tcp:24810", "\u{1f4a5}:24810", "tcp:\u{1f4a5}",
            "icmp:24810", "sctp:24810", "raw:24810", "any:24810", "*:24810", "TCP:24810", "Udp:24810", "tcp :24810", " :24810", ":", "",
            "tcp:24810:udp", "tcp::24810", "tcp;24810", "udp-lite:24810", "0.0.0.0:24810", "[::]:24810", "tcp:24810-24900", "tcp:1024-65535",
            "tcp:1", "udp:53", "tcp:80", "tcp:443", "tcp:1023",
        ];
        for b in bad {
            assert!(Door::parse(b).is_err(), "{b:?} was taken");
        }
        let long = format!("tcp:{}", "9".repeat(10_000));
        assert!(Door::parse(&long).unwrap_err().len() < 100);
        assert!(Door::parse(&"x".repeat(10_000)).unwrap_err().len() < 100);
    }

    #[test]
    fn ports_are_said_in_plain_words() {
        assert_eq!(ports_words(&[d("tcp:24810"), d("udp:24811")]), "Ports TCP 24810 and UDP 24811");
        assert_eq!(ports_words(&[d("udp:24811")]), "Port UDP 24811");
        assert_eq!(ports_words(&[d("tcp:2000"), d("udp:2001"), d("tcp:2002")]), "Ports TCP 2000, UDP 2001 and TCP 2002");
        assert_eq!(ports_words(&[]), "");
    }

    #[test]
    fn a_plugin_may_name_four_ports_none_twice_and_none_reserved() {
        assert!(vet(&[d("tcp:24810"), d("udp:24811")]).is_ok());
        assert!(vet(&[d("tcp:2000"), d("tcp:2001"), d("tcp:2002"), d("tcp:2003")]).is_ok());
        assert!(vet(&[d("tcp:2000"), d("tcp:2001"), d("tcp:2002"), d("tcp:2003"), d("tcp:2004")]).unwrap_err().contains("at most 4"));
        assert!(vet(&[d("tcp:2000"), d("tcp:2000")]).unwrap_err().contains("twice"));
        assert!(vet(&[d("tcp:2000"), d("udp:2000")]).is_ok());
        for r in RESERVED_PORTS {
            for proto in [Proto::Tcp, Proto::Udp] {
                assert!(vet(&[Door { proto, port: *r }]).unwrap_err().contains("common service"), "{r}");
            }
        }
        for r in [5900, 5901, 5950, 5999] {
            assert!(vet(&[Door { proto: Proto::Tcp, port: r }]).is_err(), "{r}");
        }
        assert!(vet(&[d("tcp:5899")]).is_ok() && vet(&[d("tcp:6000")]).is_ok());
    }

    #[test]
    fn the_well_known_service_ports_are_all_kept_back() {
        for port in [22u16, 53, 80, 443, 445, 631, 3389, 5900, 5353, 41641, 137, 138, 139, 515, 9100, 1900, 3702, 5357, 2049, 3306, 5432, 6379, 8080, 8443] {
            assert!(reserved(port), "{port}");
        }
        assert!(!reserved(24810) && !reserved(24811) && !reserved(1024) && !reserved(65535));
    }

    #[test]
    fn every_port_the_rules_use_above_the_lowest_is_kept_back_from_plugins() {
        let r = rules(true, true, true, &[]);
        let mut numbers = Vec::new();
        for l in r.lines() {
            for key in ["dport", "sport"] {
                if let Some(at) = l.find(key) {
                    let rest = &l[at + key.len()..];
                    let end = rest.find("accept").unwrap_or(rest.len());
                    numbers.extend(rest[..end].split(|c: char| !c.is_ascii_digit()).filter_map(|n| n.parse::<u16>().ok()));
                }
            }
        }
        assert!(!numbers.is_empty());
        for n in numbers {
            assert!(reserved(n), "{n} is used by the rules but a plugin could still ask for it");
        }
    }

    #[test]
    fn only_who_was_let_through_is_kept_never_ports() {
        let all = vec![Entitled { uid: 1000, id: "org.example.Sync".into() }, Entitled { uid: 1001, id: "b".into() }];
        let text = entitled_text(&all);
        assert_eq!(text, "1000\torg.example.Sync\n1001\tb\n");
        assert_eq!(parse_entitled(&text), all);
        assert_eq!(parse_entitled("x\ty\n\n1\n1\ta\textra\n\t\n"), []);
        assert_eq!(parse_entitled("1\ta\n1\ta\n").len(), 1);
        assert_eq!(with(&all, 1000, "org.example.Sync"), all);
        assert_eq!(with(&all, 5, "z").len(), 3);
        assert_eq!(without(&all, 1000, "org.example.Sync").len(), 1);
        assert_eq!(without(&all, 1000, "other"), all);
    }

    #[test]
    fn plugin_ports_open_from_the_home_network_only() {
        let r = rules(false, false, false, &[held(&["tcp:24810", "udp:24811"])]);
        let tcp = lines_for(&r, "tcp dport 24810");
        let udp = lines_for(&r, "udp dport 24811");
        assert_eq!(tcp.len(), 2, "{tcp:?}");
        assert_eq!(udp.len(), 2, "{udp:?}");
        assert!(tcp.iter().chain(&udp).all(|l| l.contains("saddr") && l.ends_with("accept")), "{r}");
        assert!(tcp.iter().any(|l| l.starts_with("ip saddr") && l.contains("192.168.0.0/16") && l.contains("10.0.0.0/8")
            && l.contains("172.16.0.0/12") && l.contains("169.254.0.0/16")));
        assert!(tcp.iter().any(|l| l.starts_with("ip6 saddr") && l.contains("fe80::/10") && l.contains("fc00::/7")));
        assert!(!r.lines().any(|l| l.trim() == "tcp dport 24810 accept" || l.trim() == "udp dport 24811 accept"));
        assert_eq!(r.matches("policy drop;").count(), 1);
        assert!(!r.contains("udp dport 24810") && !r.contains("tcp dport 24811"));
    }

    #[test]
    fn ports_of_one_kind_share_a_rule_and_the_order_is_fixed() {
        let r = rules(false, false, false, &[held(&["tcp:30001"]), held(&["tcp:30000", "udp:2000"])]);
        assert_eq!(lines_for(&r, "tcp dport { 30000, 30001 }").len(), 2, "{r}");
        assert_eq!(lines_for(&r, "udp dport 2000").len(), 2);
        let again = rules(false, false, false, &[held(&["udp:2000", "tcp:30000"]), held(&["tcp:30001"])]);
        assert_eq!(r, again);
    }

    fn reading(table: &'static [(&'static str, &'static [&'static str])]) -> impl Fn(&Entitled) -> Result<Vec<Door>, String> {
        move |e: &Entitled| {
            let row = table.iter().find(|(id, _)| *id == e.id).ok_or("Its manifest could not be read.".to_string())?;
            row.1.iter().map(|t| Door::parse(t)).collect()
        }
    }

    fn who(uid: u32, id: &str) -> Entitled {
        Entitled { uid, id: id.into() }
    }

    #[test]
    fn closing_takes_the_rules_away_and_nothing_else_changes() {
        let read = reading(&[("f", &["tcp:24810"])]);
        let open = held_of(&derive(&[who(1000, "f")], &read));
        assert_eq!(open, vec![held(&["tcp:24810"])]);
        let shut = held_of(&derive(&without(&[who(1000, "f")], 1000, "f"), &read));
        assert!(shut.is_empty());
        let before = rules(false, false, false, &[]);
        assert_eq!(rules(false, false, false, &shut), before);
        assert!(!before.contains("24810"));
        assert!(rules(false, false, false, &open).contains("24810"));
        assert!(without(&[], 5, "none").is_empty());
    }

    #[test]
    fn the_rules_never_hold_what_a_hand_written_list_might() {
        let r = rules(false, false, false, &[Held { doors: vec![d("tcp:2000"), Door { proto: Proto::Tcp, port: 22 },
            Door { proto: Proto::Udp, port: 5353 }, Door { proto: Proto::Tcp, port: 41641 }, Door { proto: Proto::Tcp, port: 5900 }] }]);
        assert!(!r.contains("dport 22 ") && !r.contains("tcp dport 41641") && !r.contains("5900"), "{r}");
        assert_eq!(lines_for(&r, "tcp dport 2000").len(), 2);
        assert_eq!(lines_for(&r, "udp dport 5353").len(), 1, "only Trindle's own");
        let many = Held { doors: (0..100u16).map(|n| Door { proto: Proto::Tcp, port: 20000 + n }).collect() };
        let r = rules(false, false, false, &[many]);
        let line = lines_for(&r, "tcp dport {").into_iter().find(|l| l.starts_with("ip saddr")).unwrap();
        let set = &line[line.find("dport {").unwrap()..];
        assert_eq!(set.matches(',').count() + 1, MAX_OPEN, "{r}");
    }

    #[test]
    fn a_port_in_use_by_another_plugin_is_refused() {
        let read = reading(&[("org.example.Sync", &["tcp:24810"]), ("org.example.Other", &["tcp:24810", "udp:3000"]),
                             ("org.example.Third", &["udp:24810"])]);
        let all = [who(1000, "org.example.Sync"), who(1000, "org.example.Other"), who(1001, "org.example.Third")];
        let got = derive(&all, &read);
        assert!(got[0].1.is_ok());
        let refused = got[1].1.clone().unwrap_err();
        assert_eq!(refused, Refusal::Taken(d("tcp:24810"), "org.example.Sync".into()));
        let says = refused.words(&|id| if id == "org.example.Sync" { "Sync".to_string() } else { id.to_string() });
        assert_eq!(says, "TCP 24810 is already used by Sync, so it cannot be opened for this plugin too.");
        assert_eq!(held_of(&got), vec![held(&["tcp:24810"]), held(&["udp:24810"])]);
        let swapped = derive(&[all[1].clone(), all[0].clone()], &read);
        assert!(swapped[0].1.is_ok() && swapped[1].1.is_err());
    }

    #[test]
    fn what_a_manifest_cannot_be_trusted_for_opens_nothing_and_holds_up_no_one() {
        let read = reading(&[("good", &["tcp:30000"])]);
        let all = [who(1, "gone"), who(1, "good")];
        let got = derive(&all, &read);
        assert!(matches!(&got[0].1, Err(Refusal::Bad(_))));
        assert_eq!(held_of(&got), vec![held(&["tcp:30000"])]);
        for (id, ports) in [("res", &["tcp:22"][..]), ("res", &["tcp:5900"]), ("res", &["tcp:30000", "udp:5353"]),
                            ("res", &["tcp:30001", "tcp:30001"]), ("res", &["tcp:30001", "tcp:30002", "tcp:30003", "tcp:30004", "tcp:30005"])] {
            let ports: Vec<Door> = ports.iter().filter_map(|p| Door::parse(p).ok()).collect();
            let read = move |_: &Entitled| Ok(ports.clone());
            let got = derive(&[who(1, id)], &read);
            assert!(held_of(&got).is_empty(), "{:?}", got);
        }
    }

    fn in_a_private_namespace(text: &str) -> String {
        use std::io::Write;
        use std::process::{Command, Stdio};
        crate::common::needs::programs(&["unshare", "nft"]);
        let mut child = Command::new("unshare").args(["-rn", "sh", "-c", "nft -f - && nft list chain inet trindle input"])
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("unshare could not be started");
        child.stdin.take().expect("unshare's input").write_all(text.as_bytes()).expect("the rules could not be handed to nft");
        let out = child.wait_with_output().expect("unshare could not be waited for");
        assert!(out.status.success(), "{}: {}\n{text}", NO_NAMESPACE, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    const NO_NAMESPACE: &str = "nft refused the rules, or this computer lets an ordinary person make no private \
                                network namespace (unshare -rn), and this test checks nothing without one";

    #[test]
    fn nft_accepts_the_rules_and_lists_the_plugins_ports() {
        let open = vec![held(&["tcp:24810", "udp:24811"]), held(&["tcp:30000", "tcp:30001"])];
        let listed = in_a_private_namespace(&rules(true, true, true, &open));
        assert!(listed.contains("policy drop"), "{listed}");
        assert!(listed.contains("tcp dport { 24810, 30000, 30001 } accept") && listed.contains("udp dport 24811 accept"), "{listed}");
        for l in listed.lines().filter(|l| l.contains("24810") || l.contains("24811")) {
            assert!(l.contains("saddr {") && (l.contains("192.168.0.0/16") || l.contains("fe80::/10")), "{l}");
        }
        let none = in_a_private_namespace(&rules(false, false, false, &[]));
        assert!(!none.contains("24810"));
    }

    #[test]
    fn a_rule_set_that_fails_to_load_leaves_the_old_one_whole() {
        let good = rules(false, false, false, &[held(&["tcp:24810"])]);
        let both = format!("{good}\nthis is not a rule\n");
        let after = in_two_loads(&good, &both);
        assert!(after.contains("tcp dport 24810 accept"), "{after}");
        assert!(after.contains("policy drop"), "{after}");
    }

    fn in_two_loads(first: &str, second: &str) -> String {
        use std::process::{Command, Stdio};
        crate::common::needs::programs(&["unshare", "nft"]);
        let dir = std::env::temp_dir().join(format!("trindle-nft-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.nft"), first).unwrap();
        std::fs::write(dir.join("b.nft"), second).unwrap();
        let cmd = format!("nft -f {0}/a.nft && ! nft -f {0}/b.nft 2>/dev/null && nft list chain inet trindle input", dir.display());
        let mut child = Command::new("unshare").args(["-rn", "sh", "-c", &cmd]).stdin(Stdio::piped()).stdout(Stdio::piped())
            .stderr(Stdio::piped()).spawn().expect("unshare could not be started");
        drop(child.stdin.take());
        let out = child.wait_with_output().expect("unshare could not be waited for");
        let _ = std::fs::remove_dir_all(&dir);
        if !out.status.success() {
            let why = String::from_utf8_lossy(&out.stderr);
            panic!("the second load was not refused, or the first failed, or: {NO_NAMESPACE}: {why}");
        }
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    #[test]
    fn no_more_than_sixteen_ports_are_open_for_plugins_in_all() {
        static ROWS: [(&str, &[&str]); 5] = [
            ("p0", &["tcp:20000", "tcp:20001", "tcp:20002", "tcp:20003"]), ("p1", &["tcp:20010", "tcp:20011", "tcp:20012", "tcp:20013"]),
            ("p2", &["tcp:20020", "tcp:20021", "tcp:20022", "tcp:20023"]), ("p3", &["tcp:20030", "tcp:20031", "tcp:20032", "tcp:20033"]),
            ("p9", &["tcp:30000"]),
        ];
        let read = reading(&ROWS);
        let all: Vec<Entitled> = ROWS.iter().map(|(id, _)| who(1, id)).collect();
        let got = derive(&all, &read);
        assert_eq!(held_of(&got).iter().map(|h| h.doors.len()).sum::<usize>(), MAX_OPEN);
        let last = got[4].1.clone().unwrap_err();
        assert_eq!(last, Refusal::Full);
        assert!(last.words(&|i| i.into()).contains("16"));
        let fewer = derive(&without(&all, 1, "p0"), &read);
        assert!(fewer.iter().all(|(_, r)| r.is_ok()));
    }
}
