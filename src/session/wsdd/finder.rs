//! The finder: the computers found on each network, the socket gvfs asks over, and announcing this computer.

use std::thread;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{DISCOVERY_TO, GET, HOST_CHECK, HTTP_PORT, HTTP_WAIT, PROBE_KEEP, WSA, WSD};
use super::host::{lasting_urn, wants_us, Host};
use super::http::{http_reply, post, read_request, url_host};
use super::limits::{spawn_within, started, ANSWERING, DESCRIBING, MOST_ANSWERING, MOST_DESCRIBING, MOST_REPLYING, MOST_SHOUTING, REPLYING, SHOUTING};
use super::message::{clean, envelope, escape, local_time, new_urn, random_byte, same_type};
use super::net::{interfaces, listen_socket, send_socket, shout};
use super::xml::{metadata_of, Node};

#[derive(Default)]
struct Device {
    name: String,
    belongs_to: String,
    types: Vec<String>,

    addresses: BTreeMap<String, BTreeSet<String>>,
    last_seen: i64,
}

struct Link {
    name: String,
    addr: Ipv4Addr,
    send: Arc<UdpSocket>,
    alive: Arc<AtomicBool>,
}

#[derive(Default)]
struct State {
    devices: BTreeMap<String, Device>,
    probes: VecDeque<(String, Instant)>,
    seen: VecDeque<String>,
    links: Vec<Link>,

    greeted: BTreeSet<(String, Ipv4Addr)>,
}

struct Arrival<'a> {

    send: &'a Arc<UdpSocket>,

    net: &'a str,
    me: Ipv4Addr,

    from: SocketAddr,
}

pub(super) struct Finder {
    state: Mutex<State>,
    urn: String,
    host: Host,
}

impl Finder {
    pub(super) fn new() -> Self {
        let conf = std::fs::read_to_string(crate::common::sharing::SMB_CONF).unwrap_or_default();
        let host = Host::new(lasting_urn(), crate::common::sharing::host_name(), crate::common::sharing::workgroup(&conf));
        Finder { state: Mutex::new(State::default()), urn: new_urn(), host }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(super) fn serve(self: Arc<Self>, conn: UnixStream) {
        let Ok(mut out) = conn.try_clone() else { return };
        for line in BufReader::new(conn).lines() {
            let Ok(line) = line else { return };
            let mut words = line.split_whitespace();
            match words.next() {
                Some("probe") => self.clone().probe(words.next()),
                Some("clear") => self.lock().devices.clear(),
                Some("list") => {
                    let reply = self.list(words.next());
                    if out.write_all(reply.as_bytes()).is_err() {
                        return;
                    }
                }
                Some("quit") => return,
                _ => {}
            }
        }
    }

    fn list(&self, kind: Option<&str>) -> String {
        let state = self.lock();
        let mut reply = String::new();
        for (endpoint, dev) in &state.devices {
            if let Some(kind) = kind {
                if !dev.types.iter().any(|t| same_type(t, kind)) {
                    continue;
                }
            }
            let addresses: Vec<String> = dev
                .addresses
                .iter()
                .map(|(net, addrs)| {
                    let quoted: Vec<String> = addrs.iter().map(|a| format!("'{a}'")).collect();
                    format!("{net}, {{{}}}", quoted.join(", "))
                })
                .collect();
            reply.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\n",
                clean(endpoint),
                clean(&dev.name),
                clean(&dev.belongs_to),
                local_time(dev.last_seen),
                addresses.join(","),
                clean(&dev.types.join(",")),
            ));
        }
        reply.push_str(".\n");
        reply
    }

    fn probe(self: Arc<Self>, only: Option<&str>) {
        self.refresh_links();
        let id = new_urn();
        let body = "<wsd:Probe><wsd:Types>wsdp:Device</wsd:Types></wsd:Probe>";
        let msg = envelope(DISCOVERY_TO, &format!("{WSD}/Probe"), &id, "", body);
        let sends: Vec<Arc<UdpSocket>> = {
            let mut state = self.lock();
            state.probes.retain(|(_, at)| at.elapsed() < PROBE_KEEP);
            state.probes.push_back((id, Instant::now()));
            state
                .links
                .iter()
                .filter(|l| only.is_none_or(|n| n == l.name))
                .map(|l| l.send.clone())
                .collect()
        };
        for send in sends {
            let msg = msg.clone();
            spawn_within(&SHOUTING, MOST_SHOUTING, move || shout(&send, &msg));
        }
    }

    pub(super) fn keep_hosting(self: Arc<Self>) {
        loop {
            let wanted = Path::new(crate::common::sharing::OPEN).exists();
            match (wanted, self.host.on.load(Ordering::Relaxed)) {
                (true, false) => self.clone().start_hosting(),
                (false, true) => self.stop_hosting(),
                (true, true) => self.greet_new_links(),
                (false, false) => {}
            }
            thread::sleep(HOST_CHECK);
        }
    }

    fn start_hosting(self: Arc<Self>) {

        let Ok(listener) = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, HTTP_PORT)) else { return };
        if listener.set_nonblocking(true).is_err() {
            return;
        }
        self.host.on.store(true, Ordering::Relaxed);
        let me = self.clone();
        if !started(move || me.answer_http(listener)) {
            self.host.on.store(false, Ordering::Relaxed);
            return;
        }
        self.greet_new_links();
    }

    fn stop_hosting(&self) {
        self.host.on.store(false, Ordering::Relaxed);
        let bye = self.host.bye();
        let sends: Vec<Arc<UdpSocket>> = {
            let mut state = self.lock();
            let greeted = std::mem::take(&mut state.greeted);
            state.links.iter().filter(|l| greeted.contains(&(l.name.clone(), l.addr))).map(|l| l.send.clone()).collect()
        };
        for send in sends {
            shout(&send, &bye);
        }
    }

    fn greet_new_links(self: &Arc<Self>) {
        self.refresh_links();
        let fresh: Vec<(Arc<UdpSocket>, Ipv4Addr)> = {
            let mut state = self.lock();
            let state = &mut *state;
            state.greeted.retain(|(n, a)| state.links.iter().any(|l| l.name == *n && l.addr == *a));
            let mut out = Vec::new();
            for l in &state.links {
                if state.greeted.insert((l.name.clone(), l.addr)) {
                    out.push((l.send.clone(), l.addr));
                }
            }
            out
        };
        for (send, addr) in fresh {
            let hello = self.host.hello(addr);
            spawn_within(&SHOUTING, MOST_SHOUTING, move || shout(&send, &hello));
        }
    }

    fn answer_http(self: Arc<Self>, listener: TcpListener) {
        while self.host.on.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((conn, _)) => {
                    let me = self.clone();

                    spawn_within(&ANSWERING, MOST_ANSWERING, move || me.answer(conn));
                }
                Err(_) => thread::sleep(Duration::from_millis(200)),
            }
        }
    }

    fn answer(&self, mut conn: TcpStream) {
        let _ = conn.set_nonblocking(false);
        let _ = conn.set_read_timeout(Some(HTTP_WAIT));
        let _ = conn.set_write_timeout(Some(HTTP_WAIT));
        let reply = read_request(&mut conn).and_then(|(path, body)| self.host.reply_to(&path, &body));
        let _ = conn.write_all(http_reply(reply.as_deref()).as_bytes());
    }

    fn refresh_links(self: &Arc<Self>) {
        let now = interfaces();
        let mut state = self.lock();
        state.links.retain(|l| {
            let keep = now.iter().any(|i| i.name == l.name && i.addr == l.addr);
            if !keep {
                l.alive.store(false, Ordering::Relaxed);
            }
            keep
        });
        for iface in now {
            if state.links.iter().any(|l| l.name == iface.name && l.addr == iface.addr) {
                continue;
            }
            let (Ok(recv), Ok(send)) = (listen_socket(&iface), send_socket(&iface)) else {
                continue;
            };
            let alive = Arc::new(AtomicBool::new(true));
            let send = Arc::new(send);
            for sock in [Arc::new(recv), send.clone()] {
                let (me, alive, name, addr) = (self.clone(), alive.clone(), iface.name.clone(), iface.addr);
                let reply = send.clone();
                started(move || me.read(&sock, &reply, &name, addr, &alive));
            }
            state.links.push(Link { name: iface.name, addr: iface.addr, send, alive });
        }
    }

    fn read(self: Arc<Self>, sock: &UdpSocket, send: &Arc<UdpSocket>, net: &str, me: Ipv4Addr, alive: &AtomicBool) {
        let _ = sock.set_read_timeout(Some(Duration::from_secs(5)));
        let mut buf = vec![0u8; 65536];
        while alive.load(Ordering::Relaxed) {
            if let Ok((n, from)) = sock.recv_from(&mut buf) {
                self.clone().handle(&buf[..n], &Arrival { send, net, from, me });
            }
        }
    }

    fn handle(self: Arc<Self>, packet: &[u8], arrival: &Arrival) {
        let Some(root) = Node::parse(packet) else { return };
        let (Some(header), Some(body)) = (root.child("Header"), root.child("Body")) else {
            return;
        };
        let action = header.text_of("Action");
        let id = header.text_of("MessageID");
        if self.seen_before(&id) {
            return;
        }
        if self.answer_question(&action, &id, body, arrival) {
            return;
        }
        let Some(matches) = self.matches_in(&action, header, body) else { return };
        for (endpoint, xaddrs) in matches {
            match xaddrs.split_whitespace().next() {

                Some(xaddr) if url_host(xaddr).is_some_and(|h| h == arrival.from.ip().to_string()) => {
                    let (me, xaddr, net) = (self.clone(), xaddr.to_string(), arrival.net.to_string());
                    spawn_within(&DESCRIBING, MOST_DESCRIBING, move || me.describe(&endpoint, &xaddr, &net));
                }
                Some(_) => {}
                None => {
                    let body = format!(
                        "<wsd:Resolve><wsa:EndpointReference><wsa:Address>{}</wsa:Address>\
                         </wsa:EndpointReference></wsd:Resolve>",
                        escape(&endpoint)
                    );
                    let msg = envelope(DISCOVERY_TO, &format!("{WSD}/Resolve"), &new_urn(), "", &body);
                    let send = arrival.send.clone();
                    spawn_within(&SHOUTING, MOST_SHOUTING, move || shout(&send, &msg));
                }
            }
        }
    }

    fn seen_before(&self, id: &str) -> bool {
        let mut state = self.lock();
        if id.is_empty() {
            return false;
        }
        if state.seen.iter().any(|seen| seen == id) {
            return true;
        }
        state.seen.push_back(id.to_string());
        if state.seen.len() > 200 {
            state.seen.pop_front();
        }
        false
    }

    fn answer_question(&self, action: &str, id: &str, body: &Node, arrival: &Arrival) -> bool {
        let me = arrival.me;
        let answer = match action.strip_prefix(WSD).unwrap_or("") {
            "/Probe" => Some(wants_us(body).then(|| self.host.matches(me, id, true))),
            "/Resolve" => Some(body.path(&["Resolve", "EndpointReference", "Address"])
                .is_some_and(|a| a.text.trim() == self.host.urn).then(|| self.host.matches(me, id, false))),
            _ => None,
        };
        let Some(reply) = answer else { return false };
        if let Some(reply) = reply.filter(|_| self.host.on.load(Ordering::Relaxed)) {
            let (send, from) = (arrival.send.clone(), arrival.from);
            spawn_within(&REPLYING, MOST_REPLYING, move || {
                thread::sleep(Duration::from_millis(u64::from(random_byte()) * 2));
                let _ = send.send_to(reply.as_bytes(), from);
            });
        }
        true
    }

    fn matches_in(&self, action: &str, header: &Node, body: &Node) -> Option<Vec<(String, String)>> {
        let found = |path: &[&str]| -> Vec<(String, String)> {
            let mut out = Vec::new();
            let Some((last, first)) = path.split_last() else { return out };
            let Some(parent) = body.path(first) else { return out };
            for m in parent.kids.iter().filter(|k| k.name == *last) {
                let endpoint = m.path(&["EndpointReference", "Address"]).map(|n| n.text.trim().to_string());
                if let Some(endpoint) = endpoint.filter(|e| !e.is_empty()) {
                    out.push((endpoint, m.text_of("XAddrs")));
                }
            }
            out
        };
        match action.strip_prefix(WSD).unwrap_or("") {
            "/ProbeMatches" => {
                let relates = header.text_of("RelatesTo");
                if !self.lock().probes.iter().any(|(p, _)| *p == relates) {
                    return None;
                }
                Some(found(&["ProbeMatches", "ProbeMatch"]))
            }
            "/ResolveMatches" => Some(found(&["ResolveMatches", "ResolveMatch"])),
            "/Hello" => Some(found(&["Hello"])),
            "/Bye" => {
                for (endpoint, _) in found(&["Bye"]) {
                    self.lock().devices.remove(&endpoint);
                }
                None
            }
            _ => None,
        }
    }

    fn describe(&self, endpoint: &str, xaddr: &str, net: &str) {
        let Some(host) = url_host(xaddr) else { return };
        let extra = format!(
            "<wsa:ReplyTo><wsa:Address>{WSA}/role/anonymous</wsa:Address></wsa:ReplyTo>\
             <wsa:From><wsa:Address>{}</wsa:Address></wsa:From>",
            self.urn
        );
        let msg = envelope(&escape(endpoint), GET, &new_urn(), &extra, "");
        let Some(reply) = post(xaddr, &msg) else { return };
        let Some(root) = Node::parse(&reply) else { return };
        let Some((props, types)) = metadata_of(&root) else { return };

        let mut state = self.lock();
        let dev = state.devices.entry(endpoint.to_string()).or_default();
        if let Some(types) = types {
            dev.types = types;
        }
        dev.addresses.entry(net.to_string()).or_default().insert(host);
        dev.last_seen = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        match (props.get("DisplayName"), props.get("BelongsTo"), props.get("FriendlyName")) {
            (Some(name), Some(group), _) => {
                dev.name = name.clone();
                dev.belongs_to = group.clone();
            }
            (_, _, Some(name)) => dev.name = name.clone(),
            _ => {}
        }
    }
}
