//! Tests of the home network room.

use super::*;
use crate::store::boxed::walls::{walls, Gates};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

#[test]
fn the_home_network_is_the_private_ranges_and_the_local_multicast_only() {
    for yes in ["10.0.0.1", "10.255.255.255", "172.16.0.1", "172.31.255.254", "192.168.0.1", "192.168.255.254", "169.254.1.1",
                "224.0.0.251", "224.0.0.1", "239.255.255.250", "255.255.255.255", "fc00::1", "fd12:3456::1", "fe80::1", "febf::1", "ff02::fb", "ff02::c"] {
        assert!(home_address(&ip(yes)), "{yes}");
    }
    for no in ["8.8.8.8", "1.1.1.1", "127.0.0.1", "127.0.0.53", "0.0.0.0", "172.15.255.255", "172.32.0.1", "100.64.0.1", "100.100.100.100",
               "192.167.255.255", "192.169.0.1", "11.0.0.1", "9.255.255.255", "169.253.255.255", "169.255.0.1", "224.0.1.1", "239.255.255.251", "239.255.255.249",
               "192.0.0.1", "198.18.0.1", "203.0.113.9", "::1", "::", "2001:db8::1", "2606:4700::1", "::ffff:10.0.0.1", "::ffff:192.168.1.1",
               "ff05::1", "ff0e::1", "ff01::1", "fec0::1", "fbff::1", "fe00::1", "fec0::fb"] {
        assert!(!home_address(&ip(no)), "{no}");
    }
}

#[test]
fn the_rules_are_made_from_that_one_list_and_drop_everything_else() {
    let r = ruleset();
    for (a, b) in HOME_V4.iter().chain(HOME_V6) {
        assert!(r.contains(&format!("{a}/{b}")), "{a}/{b} is in the rules: {r}");
    }
    assert_eq!(r.matches("policy drop;").count(), 2, "{r}");
    assert!(r.contains("hook output") && r.contains("hook input"), "{r}");
    assert!(r.contains("th dport { 53, 853 } drop"), "{r}");
    let names = r.find("th dport").unwrap();
    assert!(names < r.find("ip daddr @home4 accept").unwrap(), "names are refused before any address is let through: {r}");
    let accepts: Vec<&str> = r.lines().map(str::trim).filter(|l| l.ends_with("accept")).collect();
    assert_eq!(accepts, ["oifname \"lo\" accept", "ip daddr @home4 accept", "ip6 daddr @home6 accept",
                         "iifname \"lo\" accept", "ip saddr @home4 accept", "ip6 saddr @home6 accept"]);
}

#[test]
fn pasta_is_told_to_forward_nothing_and_to_keep_to_one_interface() {
    let a = pasta_args(77, Path::new("/run/box/pasta.pid"), Some("enp3s0"), Some("enp3s0")).join(" ");
    for must in ["--no-map-gw", "-t none", "-T none", "-u none", "-U none", "--no-dhcp", "--no-dhcpv6", "--no-ra", "--config-net",
                 "--outbound-if4 enp3s0", "--outbound-if6 enp3s0", "-P /run/box/pasta.pid 77"] {
        assert!(a.contains(must), "{must}: {a}");
    }
    assert!(!a.contains("--ipv4-only") && !a.contains("--ipv6-only") && !a.contains("--map-host-loopback") && !a.contains("--dns"), "{a}");
    let v4 = pasta_args(1, Path::new("/p"), Some("eth0"), None).join(" ");
    assert!(v4.contains("--ipv4-only") && !v4.contains("--outbound-if6"), "{v4}");
    let v6 = pasta_args(1, Path::new("/p"), None, Some("eth0")).join(" ");
    assert!(v6.contains("--ipv6-only") && !v6.contains("--outbound-if4"), "{v6}");
    assert_eq!(pasta_args(4242, Path::new("/p"), Some("eth0"), None).last().unwrap(), "4242");
}

#[test]
fn the_default_route_is_read_from_the_kernels_tables() {
    let v4 = "Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT\n\
              wlp2s0\t00000000\t0100A8C0\t0003\t0\t0\t600\t00000000\t0\t0\t0\n\
              enp3s0\t00000000\t0100A8C0\t0003\t0\t0\t100\t00000000\t0\t0\t0\n\
              enp3s0\t0000A8C0\t00000000\t0001\t0\t0\t100\t0000FFFF\t0\t0\t0\n\
              down0\t00000000\t0100A8C0\t0002\t0\t0\t1\t00000000\t0\t0\t0\n";
    assert_eq!(default_interface_v4(v4).as_deref(), Some("enp3s0"), "the lowest metric among routes that are up");
    assert_eq!(default_interface_v4("Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT\n"), None);
    let z = "0".repeat(32);
    let v6 = format!("{z} 00 {z} 00 {z} 00000400 00000001 00000000 00000003 enp3s0\n\
                      {z} 00 {z} 00 {z} 00000200 00000001 00000000 00000203 reject0\n\
                      {z} 00 {z} 00 {z} 00000001 00000001 00000000 00000003 lo\n\
                      fe800000000000000000000000000000 40 {z} 00 {z} 00000100 00000001 00000000 00000001 wlp2s0\n");
    assert_eq!(default_interface_v6(&v6).as_deref(), Some("enp3s0"));
    assert_eq!(default_interface_v6(""), None);
    assert!(plain_interface("enp3s0") && plain_interface("wlp2s0.12") && plain_interface("br-lan"));
    for no in ["", "a b", "a/b", "--help", "-t", "an-interface-name-that-is-too-long", "x\ny"] {
        assert!(!plain_interface(no), "{no:?}");
    }
}

fn gates_for_room() -> Gates {
    Gates { home_room: Some((1000, 1000)), ..Gates::default() }
}

#[test]
fn the_box_in_the_room_is_a_user_namespace_of_its_own_with_no_powers() {
    let at = PathBuf::from("/var/tmp/trindle-room-args");
    let cmd = vec!["x".to_string()];
    let line = |allowed: &[&str], gates: &Gates| {
        let allowed: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
        walls(&at, &allowed, gates, &cmd).join(" ")
    };
    let shut = "--share-net --unshare-user --disable-userns --uid 1000 --gid 1000 --cap-drop ALL";
    assert!(line(&["home-network"], &gates_for_room()).contains(shut));
    for (allowed, gates) in [(&[][..], gates_for_room()), (&["usb-devices"][..], gates_for_room()), (&["home-network"][..], Gates::default())] {
        let l = line(allowed, &gates);
        assert!(!l.contains("--share-net") && !l.contains("--cap-drop") && !l.contains("--uid"), "{allowed:?}: {l}");
    }
    let l = line(&["network", "home-network"], &gates_for_room());
    assert!(l.contains("--share-net") && !l.contains("--cap-drop") && !l.contains("--uid"), "{l}");
}

const OUT: &str = "TRINDLE_ROOM_OUT";
const LAN: &str = "TRINDLE_ROOM_LAN";
const KEEPER_ENV: &str = "TRINDLE_ROOM_KEEPER";
const PROBE: &str = "TRINDLE_ROOM_PROBE";
const OUTER_ENTRY: &str = "store::home_net::tests::the_room_outer_entry";
const LAN_ENTRY: &str = "store::home_net::tests::the_room_lan_entry";
const KEEPER_ENTRY: &str = "store::home_net::tests::the_room_keeper_entry";
const PROBE_ENTRY: &str = "store::home_net::tests::the_room_probe_entry";

const THIS_COMPUTER: &str = "192.168.77.2";
const ROUTER: &str = "192.168.77.1";
const SCANNER: &str = "192.168.77.50";
const ON_THE_INTERNET: &str = "203.0.113.9";
const DOCKER_BRIDGE: &str = "172.17.0.1";
const P_SCANNER: u16 = 38081;
const P_INTERNET: u16 = 38082;
const P_ROUTER: u16 = 38083;
const P_LOCALHOST: u16 = 38084;
const P_EVERYWHERE: u16 = 38085;

fn run_ok(program: &str, args: &[&str]) {
    let out = Command::new(program).args(args).output().unwrap_or_else(|e| panic!("{program}: {e}"));
    assert!(out.status.success(), "{program} {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

fn serve(addr: &str, port: u16, who: &'static str) {
    let l = TcpListener::bind((addr, port)).unwrap_or_else(|e| panic!("{addr}:{port}: {e}"));
    std::thread::spawn(move || {
        for mut c in l.incoming().flatten() {
            let _ = c.write_all(format!("hello from {who}\n").as_bytes());
        }
    });
}

fn answer_questions(group: Ipv4Addr, port: u16, who: &'static str) {
    let listener = relay::udp_bound(SocketAddrV4::new(group, port)).unwrap();
    listener.join_multicast_v4(&group, &Ipv4Addr::new(192, 168, 77, 1)).unwrap();
    let device = UdpSocket::bind((SCANNER, port)).unwrap();
    let liar = UdpSocket::bind((ON_THE_INTERNET, port)).unwrap();
    std::thread::spawn(move || {
        let mut buf = [0u8; 2000];
        while let Ok((n, from)) = listener.recv_from(&mut buf) {
            let text = String::from_utf8_lossy(&buf[..n]).into_owned();
            let _ = device.send_to(format!("{who}-answer:{text}").as_bytes(), from);
            let _ = liar.send_to(format!("{who}-spoof:{text}").as_bytes(), from);
        }
    });
}

fn wait_for(path: &Path) {
    for _ in 0..400 {
        if path.exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("{} did not appear", path.display());
}

#[test]
fn the_room_lan_entry() {
    let Ok(dir) = std::env::var(LAN) else { return };
    let dir = PathBuf::from(dir);
    std::fs::write(dir.join("lan-alive"), "").unwrap();
    for _ in 0..400 {
        if Command::new("ip").args(["link", "show", "l0"]).output().is_ok_and(|o| o.status.success()) {
            break;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    for args in [&["link", "set", "lo", "up"][..], &["link", "set", "l0", "up"], &["addr", "add", "192.168.77.1/24", "dev", "l0"],
                 &["addr", "add", "192.168.77.50/32", "dev", "l0"], &["addr", "add", "203.0.113.9/32", "dev", "lo"]] {
        run_ok("ip", args);
    }
    serve(SCANNER, P_SCANNER, "the scanner");
    serve(ROUTER, P_ROUTER, "the router");
    serve(ON_THE_INTERNET, P_INTERNET, "the internet");
    serve(SCANNER, 53, "a name server");
    for (group, port, who) in [(relay::GROUPS[0].0, relay::GROUPS[0].1, "mdns"), (relay::GROUPS[1].0, relay::GROUPS[1].1, "wsd")] {
        answer_questions(group, port, who);
    }
    std::fs::write(dir.join("lan-ready"), "").unwrap();
    std::thread::sleep(Duration::from_secs(120));
}

#[test]
fn the_room_keeper_entry() {
    if std::env::var_os(KEEPER_ENV).is_none() {
        return;
    }
    let args: Vec<String> = std::env::args().collect();
    let mut given = vec!["trindle".to_string()];
    given.extend(args[args.len() - 3..].iter().cloned());
    std::process::exit(keeper_main(&given));
}

fn reach(addr: &str, port: u16) -> String {
    let sa: SocketAddr = format!("{addr}:{port}").parse().unwrap();
    match TcpStream::connect_timeout(&sa, Duration::from_millis(1500)) {
        Ok(mut s) => {
            let _ = s.set_read_timeout(Some(Duration::from_millis(1500)));
            let mut said = String::new();
            let _ = s.read_to_string(&mut said);
            if said.starts_with("hello") { "reached".into() } else { "silent".into() }
        }
        Err(e) => match e.raw_os_error() {
            Some(libc::EPERM) | Some(libc::EACCES) => "blocked".into(),
            Some(libc::ECONNREFUSED) => "refused".into(),
            Some(libc::ENETUNREACH) | Some(libc::EHOSTUNREACH) => "unreachable".into(),
            _ if e.kind() == std::io::ErrorKind::TimedOut => "timeout".into(),
            _ => format!("error {e}"),
        },
    }
}

fn ask(group: Ipv4Addr, port: u16, question: &str) -> String {
    let s = UdpSocket::bind("0.0.0.0:0").unwrap();
    s.set_multicast_ttl_v4(255).unwrap();
    s.set_read_timeout(Some(Duration::from_millis(600))).unwrap();
    let began = Instant::now();
    let mut heard = Vec::new();
    let _ = s.send_to(question.as_bytes(), (group, port));
    let mut buf = [0u8; 2000];
    while began.elapsed() < Duration::from_secs(3) {
        if let Ok((n, from)) = s.recv_from(&mut buf) {
            heard.push(format!("{from}:{}", String::from_utf8_lossy(&buf[..n])));
        }
    }
    heard.sort();
    heard.join(" ")
}

fn send(addr: &str, port: u16) -> String {
    let s = UdpSocket::bind("0.0.0.0:0").unwrap();
    match s.send_to(b"ping", (addr, port)) {
        Ok(_) => "sent".into(),
        Err(e) if matches!(e.raw_os_error(), Some(libc::EPERM) | Some(libc::EACCES)) => "blocked".into(),
        Err(e) => format!("error {e}"),
    }
}

#[test]
fn the_room_probe_entry() {
    let Ok(out) = std::env::var(PROBE) else { return };
    let mut r: Vec<String> = Vec::new();
    for (name, addr, port) in [("scanner", SCANNER, P_SCANNER), ("router", ROUTER, P_ROUTER), ("internet", ON_THE_INTERNET, P_INTERNET),
                               ("localhost", "127.0.0.1", P_LOCALHOST), ("this_computer", THIS_COMPUTER, P_EVERYWHERE),
                               ("docker_bridge", DOCKER_BRIDGE, P_EVERYWHERE), ("name_server", SCANNER, 53)] {
        r.push(format!("{name}={}", reach(addr, port)));
    }
    r.push(format!("udp_name_server={}", send(SCANNER, 53)));
    r.push(format!("udp_name_server_dot={}", send(SCANNER, 853)));
    r.push(format!("udp_internet={}", send(ON_THE_INTERNET, 5353)));
    r.push(format!("udp_mdns={}", send("224.0.0.251", 5353)));
    r.push(format!("udp_wsd={}", send("239.255.255.250", 3702)));
    r.push(format!("udp_scanner={}", send(SCANNER, 5353)));
    r.push(format!("heard_mdns={}", ask(relay::GROUPS[0].0, relay::GROUPS[0].1, "who-is-there")));
    r.push(format!("heard_wsd={}", ask(relay::GROUPS[1].0, relay::GROUPS[1].1, "probe-please")));
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let caps = status.lines().find_map(|l| l.strip_prefix("CapEff:")).map(|c| c.trim().to_string()).unwrap_or_default();
    r.push(format!("capabilities={caps}"));
    // SAFETY: both calls are refused or change only this box.
    let new_net = unsafe { libc::unshare(libc::CLONE_NEWNET) } == 0;
    // SAFETY: as above, refused or changes only this box.
    let new_user = unsafe { libc::unshare(libc::CLONE_NEWUSER) } == 0;
    r.push(format!("new_network={}", if new_net { "made" } else { "refused" }));
    r.push(format!("new_user_namespace={}", if new_user { "made" } else { "refused" }));
    r.push(format!("change_loopback={}", change_loopback()));
    r.push(format!("rules={}", rules_visible()));
    std::fs::write(out, r.join("\n")).unwrap();
    std::process::exit(0);
}

fn change_loopback() -> &'static str {
    // SAFETY: a datagram socket and one ioctl on a zeroed ifreq; refused or not it touches only this box's room.
    unsafe {
        let s = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        let mut ifr: libc::ifreq = std::mem::zeroed();
        ifr.ifr_name[0] = b'l' as libc::c_char;
        ifr.ifr_name[1] = b'o' as libc::c_char;
        let sin = libc::sockaddr_in { sin_family: libc::AF_INET as u16, sin_port: 0, sin_addr: libc::in_addr { s_addr: u32::from_be_bytes([10, 9, 8, 7]).to_be() }, sin_zero: [0; 8] };
        std::ptr::copy_nonoverlapping(&sin as *const _ as *const u8, &mut ifr.ifr_ifru as *mut _ as *mut u8, std::mem::size_of::<libc::sockaddr_in>());
        let done = libc::ioctl(s, libc::SIOCSIFADDR, &ifr) == 0;
        libc::close(s);
        if done { "changed" } else { "refused" }
    }
}

fn rules_visible() -> &'static str {
    let flushed = Command::new("/usr/sbin/nft").args(["flush", "ruleset"]).output().is_ok_and(|o| o.status.success());
    if flushed { "flushed" } else { "refused" }
}

fn results(text: &str) -> std::collections::HashMap<String, String> {
    text.lines().filter_map(|l| l.split_once('=')).map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

fn try_in_house(label: &str, allowed: &[&str], exe: &Path, tools: Option<&Tools>) -> (std::collections::HashMap<String, String>, bool) {
    let base = PathBuf::from(format!("/var/tmp/trindle-room-{}-{label}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (at, shared, run) = (base.join("plugin"), base.join("out"), base.join("run"));
    for d in [at.join("app"), at.join("home"), shared.clone(), run.clone()] {
        std::fs::create_dir_all(d).unwrap();
    }
    let mut gates = Gates {
        extra: ["--ro-bind", &exe.to_string_lossy(), "/probe", "--bind", &shared.to_string_lossy(), "/out"].map(String::from).into(),
        ..Gates::default()
    };
    let allowed: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
    let command: Vec<String> = ["/probe", "--exact", PROBE_ENTRY, "--nocapture", "--test-threads=1"].map(String::from).into();
    std::env::set_var(PROBE, "/out/result");
    let room = tools.map(|t| Room::prepare_on(t, &run, Some("h0"), None).unwrap_or_else(|e| panic!("{label}: {e:?}")));
    let mut pasta_pid = None;
    if let Some(room) = &room {
        gates.home_room = Some(room.ids());
        pasta_pid = std::fs::read_to_string(run.join("pasta.pid")).ok().and_then(|t| t.trim().parse::<i32>().ok());
    }
    let (code, started) = match room {
        Some(room) => {
            let mut argv = vec!["bwrap".to_string()];
            argv.extend(walls(&at, &allowed, &gates, &command));
            let out = room.run(&argv);
            (out.code, out.started)
        }
        None => {
            let code = Command::new("bwrap").args(walls(&at, &allowed, &gates, &command)).status().unwrap().code().unwrap_or(-1);
            (code, true)
        }
    };
    assert_eq!(code, 0, "{label}: the box ran and ended well");
    let said = std::fs::read_to_string(shared.join("result")).unwrap_or_else(|e| panic!("{label}: nothing was written: {e}"));
    if let Some(pid) = pasta_pid {
        for _ in 0..120 {
            if !Path::new(&format!("/proc/{pid}")).exists() {
                break;
            }
            std::thread::sleep(Duration::from_millis(25));
        }
        assert!(!Path::new(&format!("/proc/{pid}")).exists(), "{label}: pasta ended with the room");
    }
    let _ = std::fs::remove_dir_all(&base);
    (results(&said), started)
}

#[test]
fn the_room_outer_entry() {
    let Ok(out) = std::env::var(OUT) else { return };
    let dir = PathBuf::from(&out).parent().unwrap().to_path_buf();
    let exe = std::env::current_exe().unwrap();
    std::env::set_var(KEEPER_ENV, "1");
    run_ok("ip", &["link", "set", "lo", "up"]);
    let mut house = Command::new("unshare").args(["--net", "--"]).arg(&exe).args(["--exact", LAN_ENTRY, "--nocapture", "--test-threads=1"])
        .env(LAN, &dir).spawn().unwrap();
    wait_for(&dir.join("lan-alive"));
    run_ok("ip", &["link", "add", "h0", "type", "veth", "peer", "name", "l0", "netns", &house.id().to_string()]);
    for args in [&["addr", "add", "192.168.77.2/24", "dev", "h0"][..], &["link", "set", "h0", "up"], &["route", "add", "default", "via", ROUTER, "dev", "h0"],
                 &["link", "add", "d1", "type", "dummy"], &["addr", "add", "172.17.0.1/16", "dev", "d1"], &["link", "set", "d1", "up"]] {
        run_ok("ip", args);
    }
    wait_for(&dir.join("lan-ready"));
    serve("127.0.0.1", P_LOCALHOST, "this computer's localhost");
    serve("0.0.0.0", P_EVERYWHERE, "a service on every address of this computer");
    let tools = Tools {
        unshare: program("unshare").unwrap(),
        pasta: program("pasta").unwrap(),
        keeper: [exe.to_string_lossy().as_ref(), "--exact", KEEPER_ENTRY, "--nocapture", "--test-threads=1"].map(String::from).into(),
    };
    let mut all: Vec<String> = Vec::new();
    for (label, allowed, room) in [("room", &["home-network"][..], true), ("none", &[][..], false), ("network", &["network"][..], false)] {
        let (r, started) = try_in_house(label, allowed, &exe, room.then_some(&tools));
        assert!(started, "{label}: the keeper started the box");
        all.extend(r.into_iter().map(|(k, v)| format!("{label}.{k}={v}")));
    }
    let missing = Tools { pasta: PathBuf::from("/nonexistent/pasta"), unshare: tools.unshare.clone(), keeper: tools.keeper.clone() };
    let run = dir.join("missing-run");
    match Room::prepare_on(&missing, &run, Some("h0"), None) {
        Err(Refused::Because(why)) => all.push(format!("missing.refused={}", why == "pasta is not installed")),
        Err(other) => all.push(format!("missing.refused={other:?}")),
        Ok(_) => all.push("missing.refused=made a room".into()),
    }
    let _ = house.kill();
    std::fs::write(&out, all.join("\n")).unwrap();
    std::process::exit(0);
}

#[test]
fn a_box_allowed_the_home_network_reaches_the_house_and_nothing_else() {
    crate::common::needs::programs(&["unshare", "pasta", "nft", "bwrap", "ip"]);
    let dir = PathBuf::from(format!("/var/tmp/trindle-room-main-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("results");
    let me = std::env::current_exe().unwrap();
    let ran = Command::new("unshare").args(["-Urn", "--"]).arg(&me).args(["--exact", OUTER_ENTRY, "--nocapture", "--test-threads=1"])
        .env(OUT, &out).output().unwrap();
    let text = std::fs::read_to_string(&out).unwrap_or_default();
    assert!(ran.status.success() && !text.is_empty(), "the house ran: {}{}", String::from_utf8_lossy(&ran.stdout), String::from_utf8_lossy(&ran.stderr));
    let r = results(&text);
    let mut wrong: Vec<String> = Vec::new();
    let shut = |v: &str| matches!(v, "blocked" | "timeout");
    let mut is = |k: &str, ok: &dyn Fn(&str) -> bool, wanted: &str| {
        let v = r.get(k).map(String::as_str).unwrap_or("not reported");
        if !ok(v) {
            wrong.push(format!("{k} is {v:?}, and should be {wanted}"));
        }
    };
    for what in ["scanner", "router", "internet", "localhost", "this_computer", "docker_bridge"] {
        is(&format!("network.{what}"), &|v| v == "reached", "reached");
    }
    for what in ["scanner", "router", "internet", "localhost", "this_computer", "docker_bridge", "name_server"] {
        is(&format!("none.{what}"), &|v| v != "reached", "not reached");
    }
    is("room.scanner", &|v| v == "reached", "reached");
    is("room.router", &|v| v == "reached", "reached: a home range includes the router");
    is("room.internet", &shut, "dropped");
    is("room.localhost", &|v| v == "refused", "refused: the room's own loopback holds nothing of this computer's");
    is("room.this_computer", &|v| v == "refused", "refused: the computer's own address is the room's own");
    is("room.docker_bridge", &|v| v != "reached", "not reached: an address this computer has on another interface");
    is("room.name_server", &shut, "dropped");
    for k in ["udp_name_server", "udp_name_server_dot", "udp_internet"] {
        is(&format!("room.{k}"), &|v| v == "blocked", "blocked");
    }
    for k in ["udp_mdns", "udp_wsd", "udp_scanner"] {
        is(&format!("room.{k}"), &|v| v == "sent", "sent");
    }
    is("room.heard_mdns", &|v| v == "192.168.77.50:5353:mdns-answer:who-is-there", "the scanner's one answer");
    is("room.heard_wsd", &|v| v == "192.168.77.50:3702:wsd-answer:probe-please", "the scanner's one answer");
    is("room.capabilities", &|v| v == "0000000000000000", "no capabilities");
    is("room.new_network", &|v| v == "refused", "refused");
    is("room.new_user_namespace", &|v| v == "refused", "refused");
    is("room.change_loopback", &|v| v == "refused", "refused");
    is("room.rules", &|v| v == "refused", "refused");
    is("missing.refused", &|v| v == "true", "true");
    assert!(wrong.is_empty(), "{wrong:#?}\nall of it:\n{text}");
    let _ = std::fs::remove_dir_all(&dir);
}
