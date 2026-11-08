use super::*;
use crate::plugins::fixtures::*;

#[test]
fn a_plugin_may_name_the_ports_it_listens_on() {
    let p = listening("listen", &with_listen("network;listen", LISTENING)).unwrap();
    assert_eq!(p.listen.iter().map(|d| d.to_string()).collect::<Vec<_>>(), ["tcp:24810", "udp:24811"]);
    assert_eq!(p.asks, ["network", "listen"]);

    assert!(listening("nolisten", GOOD).unwrap().listen.is_empty());

    let four = with_listen("network;listen", "\n[Network]\nListen=tcp:2000;tcp:2001;udp:2000;udp:2001\n");
    assert_eq!(listening("four", &four).unwrap().listen.len(), 4);
}

#[test]
fn the_validator_refuses_ports_it_should() {
    let problems = |perms: &str, listen: &str| -> String {
        listening("badlisten", &with_listen(perms, &format!("\n[Network]\nListen={listen}\n"))).unwrap_err().join("\n")
    };
    assert!(problems("network;listen", "icmp:2000").contains("not a protocol"));
    assert!(problems("network;listen", "TCP:2000").contains("small letters"));
    assert!(problems("network;listen", "tcp").contains("protocol:port"));
    assert!(problems("network;listen", "tcp:80").contains("1024 to 65535"));
    assert!(problems("network;listen", "tcp:70000").contains("1024 to 65535"));
    assert!(problems("network;listen", "udp:abc").contains("1024 to 65535"));
    assert!(problems("network;listen", "tcp:2000;tcp:2001;tcp:2002;tcp:2003;tcp:2004").contains("at most 4"));
    assert!(problems("network;listen", "tcp:2000;tcp:2000").contains("twice"));
    assert!(problems("network;listen", "udp:5353").contains("common service"));
    assert!(problems("network;listen", "tcp:5357").contains("common service"));

    assert!(problems("listen", "tcp:2000").contains("network permission"));
    assert!(problems("network", "tcp:2000").contains("listen permission"));
    let none = listening("noports", &with_listen("network;listen", "")).unwrap_err().join("\n");
    assert!(none.contains("needs [Network] Listen"), "{none}");
}

#[test]
fn a_hostile_manifest_cannot_get_a_port_it_should_not() {
    for bad in [
        "tcp:99999999999999999999999999999", "tcp:4294969320", "tcp:65536", "tcp:-1", "tcp:+24810", "tcp:024810", "tcp:0x60ea",
        "udp:\u{662}\u{664}\u{668}\u{661}\u{660}", "t\u{0441}p:24810", "\u{ff54}\u{ff43}\u{ff50}:24810", "tcp:24810\u{200b}",
        "TCP:24810", "icmp:24810", "tcp", ":24810", "tcp:24810;tcp:24810", "tcp:24810;udp:2000;tcp:3000;udp:3000;tcp:4000",
        "tcp:22", "tcp:80", "tcp:443", "tcp:445", "tcp:631", "tcp:53", "udp:53", "tcp:3389", "tcp:5900", "tcp:5901", "udp:5353",
        "tcp:41641", "udp:41641", "tcp:5357", "udp:3702", "udp:1900", "tcp:9100", "tcp:1023", "tcp:0",
    ] {
        assert!(declared_ports(&manifest(bad)).is_err(), "{bad:?} was taken");

        assert!(listening("hostile", &with_listen("network;listen", &format!("\n[Network]\nListen={bad}\n"))).is_err(), "{bad:?} passed the check");
    }

    let twice = format!("{}\n[Network]\nListen=tcp:22\n", manifest("tcp:24810"));
    assert!(declared_ports(&twice).map_or(true, |d| d.iter().all(|d| d.port >= 1024)));
}
