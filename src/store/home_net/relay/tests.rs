//! Tests of what the room's two helpers pass between them. The helpers
//! themselves are tried in the made-up house of `home_net/tests.rs`.

use super::*;

fn v4(s: &str) -> SocketAddrV4 {
    s.parse().unwrap()
}

#[test]
fn only_the_two_groups_can_be_asked() {
    assert_eq!(GROUPS.len(), 2);
    for (address, port) in GROUPS {
        assert!(address.is_multicast());
        assert!(home_address(&std::net::IpAddr::V4(address)), "{address} is a home address, or the rules would not let the question out");
        assert!(port == 5353 || port == 3702);
    }
    assert_eq!(GROUPS[0], (Ipv4Addr::new(224, 0, 0, 251), 5353));
    assert_eq!(GROUPS[1], (Ipv4Addr::new(239, 255, 255, 250), 3702));
}

#[test]
fn a_question_is_passed_whole_or_not_at_all() {
    let m = encode_ask(1, 40000, b"probe").unwrap();
    assert_eq!(decode_ask(&m), Some((1, 40000, &b"probe"[..])));
    let biggest = vec![7u8; PAYLOAD_MAX];
    assert_eq!(decode_ask(&encode_ask(0, 1, &biggest).unwrap()).map(|(g, p, d)| (g, p, d.len())), Some((0, 1, PAYLOAD_MAX)));

    assert!(encode_ask(2, 40000, b"x").is_none() && encode_ask(255, 40000, b"x").is_none());
    assert!(encode_ask(0, 0, b"x").is_none() && encode_ask(0, 40000, b"").is_none());
    assert!(encode_ask(0, 40000, &vec![0u8; PAYLOAD_MAX + 1]).is_none());

    for bad in [&b""[..], b"Q", b"Q\0\x9c", b"Q\x02\x9c\x40x", b"Q\0\0\0x", b"Q\0\x9c\x40", b"A\0\x9c\x40x", b"\0\0\x9c\x40x"] {
        assert!(decode_ask(bad).is_none(), "{bad:?}");
    }
    let mut big = vec![b'Q', 0, 0x9c, 0x40];
    big.extend(vec![1u8; PAYLOAD_MAX + 1]);
    assert!(decode_ask(&big).is_none());
}

#[test]
fn an_answer_is_passed_whole_or_not_at_all() {
    let from = v4("192.168.1.40:5353");
    let m = encode_answer(40000, from, b"here").unwrap();
    assert_eq!(decode_answer(&m), Some((40000, from, &b"here"[..])));
    assert!(encode_answer(0, from, b"x").is_none() && encode_answer(40000, from, b"").is_none());
    assert!(encode_answer(40000, from, &vec![0u8; PAYLOAD_MAX + 1]).is_none());
    for bad in [&b""[..], b"A", b"A\x9c\x40\x0a\x00\x00\x01\x14\xe9", b"A\0\0\x0a\0\0\x01\x14\xe9x", b"Q\x9c\x40\x0a\0\0\x01\x14\xe9x"] {
        assert!(decode_answer(bad).is_none(), "{bad:?}");
    }
}

#[test]
fn an_answer_is_given_only_from_a_device_at_home() {
    for yes in ["192.168.1.40:5353", "10.0.0.7:3702", "172.16.5.5:80", "169.254.3.3:1900"] {
        assert!(answer_ok(&v4(yes)), "{yes}");
    }
    for no in ["8.8.8.8:5353", "203.0.113.9:5353", "127.0.0.1:5353", "0.0.0.0:5353", "224.0.0.251:5353", "239.255.255.250:3702",
               "255.255.255.255:5353", "192.168.1.40:0", "100.64.0.1:5353", "172.32.0.1:5353"] {
        assert!(!answer_ok(&v4(no)), "{no}");
    }
}

#[test]
fn no_more_than_fifty_questions_a_second() {
    let mut pace = Pace::new();
    let began = Instant::now();
    let allowed = (0..200).filter(|_| pace.allows(began)).count();
    assert_eq!(allowed, ASKS_PER_SECOND as usize);
    assert!(pace.allows(began + Duration::from_millis(1001)), "a second on, it starts again");
}

#[test]
fn a_packet_made_for_the_room_is_a_whole_ipv4_udp_packet() {
    let p = udp_packet(v4("192.168.77.50:5353"), v4("192.168.77.2:40000"), b"answer");
    assert_eq!(p.len(), 20 + 8 + 6);
    assert_eq!((p[0], p[9]), (0x45, 17), "IPv4, header of 20 bytes, UDP");
    assert_eq!(u16::from_be_bytes([p[2], p[3]]) as usize, p.len());
    assert_eq!(&p[12..16], [192, 168, 77, 50]);
    assert_eq!(&p[16..20], [192, 168, 77, 2]);
    assert_eq!(u16::from_be_bytes([p[20], p[21]]), 5353);
    assert_eq!(u16::from_be_bytes([p[22], p[23]]), 40000);
    assert_eq!(u16::from_be_bytes([p[24], p[25]]) as usize, 8 + 6);
    assert_eq!(&p[28..], b"answer");

    let mut sum: u32 = p[..20].chunks(2).map(|c| u32::from(u16::from_be_bytes([c[0], c[1]]))).sum();
    while sum > 0xffff {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    assert_eq!(sum, 0xffff);
}

#[test]
fn a_question_goes_nowhere_but_the_two_groups_whatever_the_box_says() {

    let source = std::fs::read_to_string(crate::common::source::path_of("store/home_net/relay.rs")).unwrap();
    let code = source.split("#[cfg(test)]").next().unwrap();
    assert_eq!(code.matches(".send_to(").count(), 1, "one place puts a question to a group");
    assert!(code.contains("let (address, to) = GROUPS[group];"));
}
