//! Tests of the WS-Discovery finder.

use std::thread;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use super::{clear_socket, DISCOVERY_TO, GET, WSD};
use super::host::{lasting_urn, wants_us, Host};
use super::http::{http_reply, read_request};
use super::limits::spawn_within;
use super::message::envelope;
use super::xml::{metadata_of, MOST_DEPTH, Node};

fn host() -> Host {
    Host::new("urn:uuid:11111111-2222-4333-8444-555555555555".into(), "laptop".into(), "HOME".into())
}

#[test]
fn only_a_socket_is_taken_away_from_the_listen_path() {
    let dir = std::env::temp_dir().join(format!("trindle-wsdd-listen-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("mine.txt");
    std::fs::write(&file, "the person's").unwrap();
    clear_socket(&file);
    assert_eq!(std::fs::read(&file).unwrap(), b"the person's", "a file of the person's was removed");
    let sock = dir.join("old.sock");
    let _old = std::os::unix::net::UnixListener::bind(&sock).unwrap();
    clear_socket(&sock);
    assert!(std::fs::symlink_metadata(&sock).is_err(), "an old socket was left");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_message_nested_past_the_most_is_refused() {
    let nest = |n: usize| format!("{}{}", "<a>".repeat(n), "</a>".repeat(n));
    assert!(Node::parse(nest(MOST_DEPTH).as_bytes()).is_some());
    assert!(Node::parse(nest(MOST_DEPTH + 1).as_bytes()).is_none());
    assert!(Node::parse(nest(9000).as_bytes()).is_none());
}

#[test]
fn hello_says_who_and_where() {
    let h = host();
    let hello = h.hello(Ipv4Addr::new(192, 168, 1, 5));
    let root = Node::parse(hello.as_bytes()).unwrap();
    assert_eq!(root.path(&["Header", "Action"]).unwrap().text, format!("{WSD}/Hello"));
    let about = root.path(&["Body", "Hello"]).unwrap();
    assert_eq!(about.path(&["EndpointReference", "Address"]).unwrap().text, h.urn);
    assert_eq!(about.text_of("XAddrs"), "http://192.168.1.5:5357/11111111-2222-4333-8444-555555555555");
    assert!(about.text_of("Types").contains("pub:Computer"));
    let bye = Node::parse(h.bye().as_bytes()).unwrap();
    assert_eq!(bye.path(&["Header", "Action"]).unwrap().text, format!("{WSD}/Bye"));

    assert!(h.bye().contains("MessageNumber=\"3\""));
}

#[test]
fn a_probe_for_a_computer_is_answered_and_others_are_not() {
    let probe = |types: &str| Node::parse(envelope(DISCOVERY_TO, &format!("{WSD}/Probe"), "urn:uuid:q", "",
        &format!("<wsd:Probe><wsd:Types>{types}</wsd:Types></wsd:Probe>")).as_bytes()).unwrap();
    assert!(wants_us(probe("wsdp:Device").child("Body").unwrap()));
    assert!(wants_us(probe("").child("Body").unwrap()));
    assert!(!wants_us(probe("wprt:PrintDeviceType").child("Body").unwrap()));
    let reply = host().matches(Ipv4Addr::new(10, 0, 0, 2), "urn:uuid:q", true);
    let root = Node::parse(reply.as_bytes()).unwrap();
    assert_eq!(root.path(&["Header", "RelatesTo"]).unwrap().text, "urn:uuid:q");
    assert_eq!(root.path(&["Body", "ProbeMatches", "ProbeMatch"]).unwrap().text_of("XAddrs"),
               "http://10.0.0.2:5357/11111111-2222-4333-8444-555555555555");
}

#[test]
fn its_description_reads_back_as_a_computer_in_its_workgroup() {
    let h = host();
    let get = envelope(&h.urn, GET, "urn:uuid:ask", "", "");
    let reply = h.reply_to("/11111111-2222-4333-8444-555555555555", get.as_bytes()).unwrap();
    let (props, types) = metadata_of(&Node::parse(reply.as_bytes()).unwrap()).unwrap();
    assert_eq!(props.get("DisplayName").map(String::as_str), Some("laptop"));
    assert_eq!(props.get("BelongsTo").map(String::as_str), Some("Workgroup:HOME"));
    assert_eq!(types, Some(vec!["pub:Computer".to_string()]));
    assert!(reply.contains("<wsa:RelatesTo>urn:uuid:ask</wsa:RelatesTo>"));

    assert!(h.reply_to("/someone-else", get.as_bytes()).is_none());
    assert!(h.reply_to("/11111111-2222-4333-8444-555555555555", envelope("urn:uuid:other", GET, "x", "", "").as_bytes()).is_none());
    assert!(h.reply_to("/11111111-2222-4333-8444-555555555555", b"not xml").is_none());
}

#[test]
fn requests_are_read_to_their_length() {
    let mut r: &[u8] = b"POST /abc HTTP/1.1\r\nHost: x\r\nContent-Length: 5\r\n\r\nhello and more";
    assert_eq!(read_request(&mut r), Some(("/abc".to_string(), b"hello".to_vec())));
    let mut get: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    assert_eq!(read_request(&mut get), None);
    let mut huge: &[u8] = b"POST / HTTP/1.1\r\nContent-Length: 99999999\r\n\r\n";
    assert_eq!(read_request(&mut huge), None);
    assert!(http_reply(None).starts_with("HTTP/1.1 400"));
    assert!(http_reply(Some("abc")).contains("Content-Length: 3\r\n"));
}

#[test]
fn the_lasting_address_stays_the_same() {
    assert_eq!(lasting_urn(), lasting_urn());
    assert!(lasting_urn().starts_with("urn:uuid:"));
}

#[test]
fn a_flood_starts_no_more_threads_than_the_most() {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let (go, wait) = std::sync::mpsc::channel::<()>();
    let wait = Arc::new(Mutex::new(wait));
    let started: usize = (0..50)
        .filter(|_| {
            let wait = wait.clone();
            spawn_within(&COUNT, 4, move || {
                let _ = wait.lock().map(|w| w.recv());
            })
        })
        .count();
    assert_eq!(started, 4);
    assert_eq!(COUNT.load(Ordering::SeqCst), 4);
    for _ in 0..4 {
        go.send(()).unwrap();
    }
    let until = std::time::Instant::now() + Duration::from_secs(10);
    while COUNT.load(Ordering::SeqCst) > 0 && std::time::Instant::now() < until {
        thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(COUNT.load(Ordering::SeqCst), 0);
}

#[test]
fn a_job_that_panics_gives_its_place_back() {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    for _ in 0..8 {
        assert!(spawn_within(&COUNT, 1, || panic!("a job that fails")));
        let until = std::time::Instant::now() + Duration::from_secs(10);
        while COUNT.load(Ordering::SeqCst) > 0 && std::time::Instant::now() < until {
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(COUNT.load(Ordering::SeqCst), 0);
    }
}
