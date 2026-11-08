//! After an update the keyring hands what it holds to the new program.

use std::cell::{Cell, RefCell};
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use zeroize::Zeroizing;

use super::service::{self, Held, Keyring, NAME};
use super::store;
use crate::common::renewed;
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Flow, Loop};

const DBUS: &str = "org.freedesktop.DBus";
const DBUS_PATH: &str = "/org/freedesktop/DBus";

const QUEUED: &str = "queued ";

const STAYS: &str = "stays\n";

const TAKEN: &[u8] = b"taken\n";

const LET_GO: &[u8] = b"let go\n";

const HELD: u8 = 1;

fn record(held: &Held) -> Zeroizing<Vec<u8>> {
    let packed = pack(held);
    let mut out = Zeroizing::new(Vec::with_capacity(5 + packed.len()));
    out.push(HELD);
    out.extend_from_slice(&(packed.len() as u32).to_le_bytes());
    out.extend_from_slice(&packed);
    out
}

const GRACE: Duration = Duration::from_millis(200);

const RESTS: u32 = 30;
const WORKS: u32 = 300;

const HEARS: Duration = Duration::from_secs(60);

const TAKES: Duration = Duration::from_secs(10);

const OWNS_TRIES: u32 = 20;
const OWNS_EVERY: Duration = Duration::from_millis(50);
const KEY: usize = 32;

fn pack(held: &Held) -> Zeroizing<Vec<u8>> {
    let texts: usize = held.sessions.iter().map(|(p, o)| 4 + p.len() + o.len()).sum();
    let mut out = Zeroizing::new(Vec::with_capacity(8 + 1 + KEY + 4 + texts));
    out.extend_from_slice(&held.counter.to_le_bytes());
    match &held.key {
        Some(key) => {
            out.push(1);
            out.extend_from_slice(&key[..]);
        }
        None => out.push(0),
    }
    out.extend_from_slice(&(held.sessions.len() as u32).to_le_bytes());
    for (path, owner) in &held.sessions {
        for text in [path, owner] {
            out.extend_from_slice(&(text.len().min(u16::MAX as usize) as u16).to_le_bytes());
            out.extend_from_slice(&text.as_bytes()[..text.len().min(u16::MAX as usize)]);
        }
    }
    out
}

fn unpack(mut bytes: &[u8]) -> Option<Held> {
    fn take<'a>(bytes: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
        if bytes.len() < n {
            return None;
        }
        let (head, rest) = bytes.split_at(n);
        *bytes = rest;
        Some(head)
    }
    let counter = u64::from_le_bytes(take(&mut bytes, 8)?.try_into().ok()?);
    let key = match take(&mut bytes, 1)?[0] {
        0 => None,
        1 => {
            let mut key: store::Key = Zeroizing::new([0u8; KEY]);
            key.copy_from_slice(take(&mut bytes, KEY)?);
            Some(key)
        }
        _ => return None,
    };
    let many = u32::from_le_bytes(take(&mut bytes, 4)?.try_into().ok()?);
    let mut sessions = Vec::new();
    for _ in 0..many {
        let mut text = || {
            let n = u16::from_le_bytes(take(&mut bytes, 2)?.try_into().ok()?) as usize;
            String::from_utf8(take(&mut bytes, n)?.to_vec()).ok()
        };
        sessions.push((text()?, text()?));
    }
    bytes.is_empty().then_some(Held { key, counter, sessions })
}

fn next_in_line(queue: &[String], me: &str, new: &str) -> bool {
    new.starts_with(':') && new != me && queue.len() == 2 && queue[0] == me && queue[1] == new
}

fn newest(mut bytes: &[u8]) -> Option<Held> {
    let mut last = None;
    while !bytes.is_empty() {
        if bytes.len() < 5 || bytes[0] != HELD {
            return None;
        }
        let long = u32::from_le_bytes(bytes[1..5].try_into().ok()?) as usize;
        let rest = &bytes[5..];
        if rest.len() < long {
            return None;
        }
        last = Some(unpack(&rest[..long])?);
        bytes = &rest[long..];
    }
    last
}

struct Hand {
    keyring: Rc<Keyring>,
    bus: Bus,
    events: Loop,
    socket: RefCell<Option<UnixStream>>,

    handing: Rc<Cell<bool>>,
    failed: RefCell<Option<Box<dyn FnOnce()>>>,

    stays: Cell<bool>,
}

impl Hand {

    fn fail(&self, why: &str) {
        eprintln!("trindle-keyring: not handed over to the updated keyring: {why}");
        self.keyring.when_changed(None);
        self.socket.borrow_mut().take();
        if let Some(failed) = self.failed.borrow_mut().take() {
            failed();
        }
    }

    fn ask(&self, method: &str, done: impl FnOnce(Option<zbus::Message>) + 'static) {
        self.bus.call(DBUS, DBUS_PATH, DBUS).method(method, &(NAME,), false, None, &self.events, move |r| done(r.ok()));
    }

    fn let_go(self: Rc<Self>, new: String, tries: u32) {
        if !self.keyring.resting() {
            if tries >= RESTS {
                return self.fail("something was under way for too long");
            }
            let (h, events) = (self.clone(), self.events.clone());
            return events.after(Duration::from_secs(1), move || h.let_go(new, tries + 1));
        }
        let (h, me) = (self.clone(), self.bus.unique_name());
        self.ask("ListQueuedOwners", move |m| {
            let queue: Vec<String> = m.and_then(|m| m.body().deserialize().ok()).unwrap_or_default();
            if !next_in_line(&queue, &me, &new) {
                return h.fail("the bus's queue for the name was not this keyring and then the new one");
            }
            if !h.keyring.resting() {
                return h.let_go(new, tries + 1);
            }

            if h.stays.get() {
                h.tell();
                let weak = Rc::downgrade(&h);
                h.keyring.when_changed(Some(Box::new(move || {
                    if let Some(hand) = weak.upgrade() {
                        hand.tell();
                    }
                })));
            }
            h.handing.set(true);
            let again = h.clone();
            h.ask("ReleaseName", move |m| {
                if m.and_then(|m| m.body().deserialize::<u32>().ok()) != Some(1) {
                    again.handing.set(false);
                    return again.fail("the bus did not let the name go");
                }
                if !again.stays.get() {
                    again.say(LET_GO);
                }
                let (h, events) = (again.clone(), again.events.clone());
                events.after(GRACE, move || h.finish(0));
            });
        });
    }

    fn say(&self, bytes: &[u8]) {
        if let Some(mut socket) = self.socket.borrow().as_ref() {
            let _ = socket.set_nonblocking(false);
            let _ = socket.write_all(bytes);
        }
    }

    fn tell(&self) {
        self.say(&record(&self.keyring.holding()));
    }

    fn finish(self: Rc<Self>, tries: u32) {
        if self.keyring.working() && tries < WORKS {
            let (h, events) = (self.clone(), self.events.clone());
            return events.after(Duration::from_millis(100), move || h.finish(tries + 1));
        }
        if !self.stays.get() {
            self.say(&pack(&self.keyring.held()));
            self.socket.borrow_mut().take();
            return self.events.quit();
        }
        let held = self.keyring.held();
        self.keyring.when_changed(None);
        self.say(&record(&held));
        drop(held);

        if let Some(socket) = self.socket.borrow().as_ref() {
            let _ = socket.shutdown(std::net::Shutdown::Write);
            let _ = socket.set_nonblocking(true);
        }
        self.hear_taken();
    }

    fn hear_taken(self: Rc<Self>) {
        let Some(fd) = self.socket.borrow().as_ref().map(|s| s.as_raw_fd()) else { return self.take_back() };
        let over = Rc::new(Cell::new(false));
        let (h, o) = (self.clone(), over.clone());
        let mut word = Vec::new();
        self.events.watch(fd, move |hung_up| {
            if o.get() {
                return Flow::Stop;
            }
            let ended = hung_up | h.read_into(&mut word);
            if word.starts_with(TAKEN) {
                o.set(true);
                h.socket.borrow_mut().take();
                h.events.quit();
            } else if ended {
                o.set(true);
                h.clone().take_back();
            } else {
                return Flow::Continue;
            }
            Flow::Stop
        });
        let (h, events) = (self.clone(), self.events.clone());
        events.after(TAKES, move || {
            if !over.replace(true) {
                eprintln!("trindle-keyring: the updated keyring has the name and has not said it took over: leaving all the same");
                h.socket.borrow_mut().take();
                h.events.quit();
            }
        });
    }

    fn read_into(&self, into: &mut Vec<u8>) -> bool {
        let mut buf = [0u8; 128];
        loop {
            let read = self.socket.borrow().as_ref().map(|mut s| s.read(&mut buf));
            match read {
                Some(Ok(0)) | None => return true,
                Some(Ok(n)) => into.extend_from_slice(&buf[..n]),
                Some(Err(e)) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Some(Err(e)) if e.kind() == std::io::ErrorKind::WouldBlock => return false,
                Some(Err(_)) => return true,
            }
        }
    }

    fn take_back(self: Rc<Self>) {
        self.socket.borrow_mut().take();
        let h = self.clone();
        self.bus.call(DBUS, DBUS_PATH, DBUS).method("RequestName", &(NAME, 4u32), false, None, &self.events, move |r| {
            if r.ok().and_then(|m| m.body().deserialize::<u32>().ok()) == Some(1) {
                eprintln!("trindle-keyring: the updated keyring went before it had taken over: this one has the name back and carries on as it was");
                h.handing.set(false);
                if let Some(failed) = h.failed.borrow_mut().take() {
                    failed();
                }
            } else {
                eprintln!("trindle-keyring: the updated keyring went before it had taken over, and another keyring has the name: leaving");
                h.events.quit();
            }
        });
    }
}

pub(super) fn hand_over(keyring: &Rc<Keyring>, bus: &Bus, events: &Loop, socket: UnixStream, handing: Rc<Cell<bool>>, failed: impl FnOnce() + 'static) {
    let _ = socket.set_nonblocking(true);
    let fd = socket.as_raw_fd();
    let hand = Rc::new(Hand { keyring: keyring.clone(), bus: bus.clone(), events: events.clone(), socket: RefCell::new(Some(socket)),
        handing, failed: RefCell::new(Some(Box::new(failed))), stays: Cell::new(false) });
    let mut line = Vec::new();
    events.watch(fd, move |hung_up| {
        let ended = hung_up | hand.read_into(&mut line);
        let end = line.iter().position(|b| *b == b'\n');
        let said = end.map(|end| String::from_utf8_lossy(&line[..end]).into_owned());

        hand.stays.set(end.is_some_and(|end| line[end + 1..].starts_with(STAYS.as_bytes())));
        match said.as_deref().map(|s| s.strip_prefix(QUEUED)) {
            Some(Some(new)) => hand.clone().let_go(new.to_string(), 0),
            Some(None) => hand.fail("the new keyring said something else"),
            None if ended => hand.fail("the new keyring went before it was queued"),
            None => return Flow::Continue,
        }
        Flow::Stop
    });
}

pub(super) fn watch(keyring: &Rc<Keyring>, bus: &Bus, events: &Loop) -> Rc<Cell<bool>> {
    let handing = Rc::new(Cell::new(false));
    let trying = Rc::new(Cell::new(false));
    let (weak, bus, ev, h) = (Rc::downgrade(keyring), bus.clone(), events.clone(), handing.clone());
    renewed::watch_with(events, move |program| {
        let Some(keyring) = weak.upgrade() else { return Flow::Stop };
        if trying.get() || !keyring.resting() {
            return Flow::Continue;
        }

        let Some((mut child, socket)) = renewed::beside(program, "trindle-keyring") else { return Flow::Stop };
        std::thread::spawn(move || child.wait());
        trying.set(true);
        let t = trying.clone();
        hand_over(&keyring, &bus, &ev, socket, h.clone(), move || t.set(false));
        Flow::Continue
    });
    handing
}

pub fn take_over() -> i32 {
    let Some(socket) = renewed::from_old() else {
        eprintln!("trindle-keyring: {} is said only by the keyring before this one", renewed::RENEWED);
        return 1;
    };
    let events = Loop::new();
    let Ok(bus) = Bus::session(&events) else { return 1 };
    take_over_on(&bus, &events, store::default_path(), socket, || {})
}

fn has_the_name(bus: &Bus, events: &Loop) -> bool {
    let (mine, me) = (Rc::new(Cell::new(false)), bus.unique_name());
    for _ in 0..OWNS_TRIES {
        let (m, ev, me) = (mine.clone(), events.clone(), me.clone());
        bus.call(DBUS, DBUS_PATH, DBUS).method("GetNameOwner", &(NAME,), false, None, events, move |r| {
            m.set(r.ok().and_then(|m| m.body().deserialize::<String>().ok()).as_deref() == Some(me.as_str()));
            ev.quit();
        });
        events.run();
        if mine.get() {
            return true;
        }
        std::thread::sleep(OWNS_EVERY);
    }
    false
}

fn take_over_on(bus: &Bus, events: &Loop, path: PathBuf, mut socket: UnixStream, taken: impl FnOnce()) -> i32 {

    let keyring = match Keyring::start_with(bus, events, path, || None) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("trindle-keyring: {e}");
            return 1;
        }
    };

    keyring.hold_calls();

    let queued = Rc::new(Cell::new(0u32));
    let (q, ev) = (queued.clone(), events.clone());
    bus.call(DBUS, DBUS_PATH, DBUS).method("RequestName", &(NAME, 0u32), false, None, events, move |r| {
        q.set(r.ok().and_then(|m| m.body().deserialize::<u32>().ok()).unwrap_or(0));
        ev.quit();
    });
    events.run();

    if !matches!(queued.get(), 1 | 2) || socket.write_all(format!("{QUEUED}{}\n{STAYS}", bus.unique_name()).as_bytes()).is_err() {
        return 1;
    }
    let _ = socket.set_read_timeout(Some(HEARS));
    let mut said = Zeroizing::new(Vec::with_capacity(1 << 16));
    let _ = socket.read_to_end(&mut said);
    if !has_the_name(bus, events) {
        eprintln!("trindle-keyring: the keyring before this one did not hand over");
        return 1;
    }

    let held = match said.strip_prefix(LET_GO) {
        Some(rest) => unpack(rest),
        None => newest(&said),
    };
    match held {
        Some(held) => {
            if let Err(e) = keyring.adopt(held) {
                eprintln!("trindle-keyring: what the keyring before this one held could not be taken up: {e}");
            }
        }
        None => eprintln!("trindle-keyring: the keyring before this one left the name and nothing whole of what it held: carrying on locked"),
    }
    keyring.answer_held();
    // The old one, if it waits to hear, may leave now.
    let _ = socket.write_all(TAKEN);
    drop(socket);
    taken();
    service::serve_until_over(&keyring, bus, events);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::bus::testing::{own_daemon, until};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc};
    use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

    #[test]
    fn what_is_held_crosses_whole_or_not_at_all() {
        let key: store::Key = Zeroizing::new([7u8; KEY]);
        let held = Held { key: Some(key), counter: 41, sessions: vec![("/org/freedesktop/secrets/session/s3".into(), ":1.42".into())] };
        let bytes = pack(&held);
        let back = unpack(&bytes).expect("read back");
        assert_eq!((back.counter, back.sessions.clone()), (41, held.sessions.clone()));
        assert_eq!(back.key.map(|k| *k), Some([7u8; KEY]));

        let locked = unpack(&pack(&Held { key: None, counter: 2, sessions: Vec::new() })).unwrap();
        assert!(locked.key.is_none() && locked.sessions.is_empty());

        for n in 0..bytes.len() {
            assert!(unpack(&bytes[..n]).is_none(), "{n} bytes were taken for the whole");
        }
        let mut more = bytes.to_vec();
        more.push(0);
        assert!(unpack(&more).is_none());
    }

    #[test]
    fn the_name_is_let_go_only_to_the_new_keyring_next_in_the_queue() {
        let q = |names: &[&str]| names.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        assert!(next_in_line(&q(&[":1.5", ":1.9"]), ":1.5", ":1.9"));

        assert!(!next_in_line(&q(&[":1.5", ":1.7", ":1.9"]), ":1.5", ":1.9"));
        assert!(!next_in_line(&q(&[":1.5", ":1.9", ":1.7"]), ":1.5", ":1.9"));
        assert!(!next_in_line(&q(&[":1.5"]), ":1.5", ":1.9"));
        assert!(!next_in_line(&q(&[":1.7", ":1.9"]), ":1.5", ":1.9"));
        assert!(!next_in_line(&q(&[":1.5", "org.example.Keys"]), ":1.5", "org.example.Keys"));
        assert!(!next_in_line(&q(&[":1.5", ":1.5"]), ":1.5", ":1.5"));
    }

    type Secret = (OwnedObjectPath, Vec<u8>, Vec<u8>, String);
    const SERVICE: &str = "org.freedesktop.Secret.Service";
    const COLLECTION: &str = "org.freedesktop.Secret.Collection";
    const ITEM: &str = "org.freedesktop.Secret.Item";
    const LOGIN: &str = "/org/freedesktop/secrets/collection/login";

    fn program_asking_until(addr: String, ready: Arc<AtomicBool>, gone: mpsc::Receiver<()>) {
        let conn = zbus::blocking::connection::Builder::address(addr.as_str()).unwrap()
            .method_timeout(Duration::from_secs(5)).build().unwrap();
        let (_, session): (OwnedValue, OwnedObjectPath) = conn.call_method(Some(NAME), "/org/freedesktop/secrets", Some(SERVICE),
            "OpenSession", &("plain", Value::from(""))).unwrap().body().deserialize().unwrap();
        let props = |label: &str| HashMap::from([
            ("org.freedesktop.Secret.Item.Label".to_string(), Value::from(label.to_string())),
            ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(HashMap::from([("user".to_string(), label.to_string())]))),
        ]);
        let secret = |v: &[u8]| -> Secret { (session.clone(), vec![], v.to_vec(), "text/plain".into()) };
        let (item, _): (OwnedObjectPath, OwnedObjectPath) = conn.call_method(Some(NAME), LOGIN, Some(COLLECTION), "CreateItem",
            &(props("before"), secret(b"kept"), true)).unwrap().body().deserialize().unwrap();
        let owner = || conn.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "GetNameOwner", &(NAME,)).unwrap()
            .body().deserialize::<String>().unwrap();
        let first = owner();
        ready.store(true, Ordering::SeqCst);

        let mut asked = 0;
        while gone.try_recv().is_err() {
            let found: Vec<OwnedObjectPath> = conn.call_method(Some(NAME), LOGIN, Some(COLLECTION), "SearchItems",
                &(HashMap::from([("user", "before")]),)).expect("a call made while the keyring changed hands failed")
                .body().deserialize().unwrap();
            assert_eq!(found, vec![item.clone()]);
            asked += 1;
        }
        assert!(asked > 0);
        assert_ne!(owner(), first, "the name is still the old keyring's");
        let locked: OwnedValue = conn.call_method(Some(NAME), LOGIN, Some("org.freedesktop.DBus.Properties"), "Get",
            &(COLLECTION, "Locked")).unwrap().body().deserialize().unwrap();
        assert!(!bool::try_from(locked).unwrap(), "the new keyring is locked");

        let (_, _, value, _): Secret = conn.call_method(Some(NAME), item.as_str(), Some(ITEM), "GetSecret", &(session.clone(),))
            .expect("the session did not survive").body().deserialize().unwrap();
        assert_eq!(value, b"kept");
        let _: (OwnedObjectPath, OwnedObjectPath) = conn.call_method(Some(NAME), LOGIN, Some(COLLECTION), "CreateItem",
            &(props("after"), secret(b"new"), true)).unwrap().body().deserialize().unwrap();
    }

    #[test]
    fn the_keyring_stays_unlocked_and_an_open_session_survives_the_hand_over() {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-renew-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keyring");
        let salt = store::Store::create(&path, b"test").unwrap().salt();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start_with(&bus, &lp, path.clone(), || Some(store::derive(b"test", &salt))).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get().is_some()));
        assert_eq!(owned.get(), Some(true));

        let (ready, is_ready) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)));
        let (r, addr) = (ready.clone(), d.1.clone());
        let (go, gone) = mpsc::channel::<()>();
        let client = std::thread::spawn(move || program_asking_until(addr, r, gone));
        let r = ready.clone();
        assert!(until(&lp, move || r.load(Ordering::SeqCst)), "the program never got its session");

        let (mine, theirs) = UnixStream::pair().unwrap();
        let (taken, addr, new_path) = (is_ready.clone(), d.1.clone(), path.clone());
        std::thread::spawn(move || {
            let lp = Loop::new();
            let bus = Bus::at(&lp, &addr).unwrap();
            take_over_on(&bus, &lp, new_path, theirs, move || taken.store(true, Ordering::SeqCst));
        });
        let (handing, failed) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(false)));
        let f = failed.clone();
        hand_over(&keyring, &bus, &lp, mine, handing.clone(), move || f.set(true));

        until(&lp, || false);
        assert!(!failed.get(), "the hand-over came to nothing");
        assert!(handing.get(), "the name was never let go");
        drop(keyring);
        drop(bus);
        for _ in 0..500 {
            if is_ready.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(is_ready.load(Ordering::SeqCst), "the new keyring never took over");
        std::thread::sleep(Duration::from_millis(100));
        go.send(()).unwrap();
        let joined = client.join();

        let mut on_disk = store::Store::load(&path).unwrap();
        on_disk.unlock(b"test").unwrap();
        let after = on_disk.search(None, &std::collections::BTreeMap::from([("user".to_string(), "after".to_string())]));
        let _ = std::fs::remove_dir_all(&dir);
        if let Err(e) = joined {
            std::panic::resume_unwind(e);
        }
        assert_eq!(after.len(), 1, "what was saved after the hand-over is not in the store");
    }

    #[test]
    fn a_stranger_in_the_queue_stops_the_hand_over_and_the_keyring_carries_on() {
        let d = own_daemon();
        let dir = std::env::temp_dir().join(format!("trindle-keyring-stranger-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keyring");
        store::Store::create(&path, b"test").unwrap();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start_with(&bus, &lp, path.clone(), || None).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get() == Some(true)));
        let stranger = zbus::blocking::connection::Builder::address(d.1.as_str()).unwrap().build().unwrap();
        let reply: u32 = stranger.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "RequestName", &(NAME, 0u32)).unwrap()
            .body().deserialize().unwrap();
        assert_eq!(reply, 2, "the stranger is not queued");
        let (mine, theirs) = UnixStream::pair().unwrap();
        let addr = d.1.clone();
        let new = std::thread::spawn(move || {
            let lp = Loop::new();
            let bus = Bus::at(&lp, &addr).unwrap();
            take_over_on(&bus, &lp, path, theirs, || panic!("the new keyring took over behind a stranger"))
        });
        let (handing, failed) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(false)));
        let f = failed.clone();
        hand_over(&keyring, &bus, &lp, mine, handing.clone(), move || f.set(true));
        let f = failed.clone();
        assert!(until(&lp, move || f.get()), "the hand-over went on with a stranger queued");
        assert!(!handing.get());
        assert_eq!(new.join().unwrap(), 1, "the new keyring did not end");
        let owner: String = stranger.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "GetNameOwner", &(NAME,)).unwrap()
            .body().deserialize().unwrap();
        assert_eq!(owner, bus.unique_name(), "the name left the keyring");
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn program(addr: &str) -> zbus::blocking::Connection {
        zbus::blocking::connection::Builder::address(addr).unwrap().method_timeout(Duration::from_secs(5)).build().unwrap()
    }

    fn owner(c: &zbus::blocking::Connection) -> Option<String> {
        c.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "GetNameOwner", &(NAME,)).ok()?.body().deserialize().ok()
    }

    fn locked(c: &zbus::blocking::Connection) -> bool {
        let locked: OwnedValue = c.call_method(Some(NAME), LOGIN, Some("org.freedesktop.DBus.Properties"), "Get", &(COLLECTION, "Locked"))
            .unwrap().body().deserialize().unwrap();
        bool::try_from(locked).unwrap()
    }

    const A_SESSION: &str = "/org/freedesktop/secrets/session/s7";

    #[derive(Clone, Copy, PartialEq)]
    enum Ends {

        Dies,

        LetsGo,

        Carries,
    }

    #[derive(Debug, PartialEq)]
    enum Became {
        Never,
        Took { locked: bool, session: bool },
    }

    fn queued_name(mine: &mut UnixStream) -> String {
        mine.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut said = Vec::new();
        let mut buf = [0u8; 128];
        while !said.ends_with(STAYS.as_bytes()) {
            let n = mine.read(&mut buf).expect("the new keyring never said it was queued and stays");
            assert!(n > 0, "the new keyring went before it was queued");
            said.extend_from_slice(&buf[..n]);
        }
        let said = String::from_utf8(said).unwrap();
        said.strip_prefix(QUEUED).and_then(|s| s.strip_suffix(STAYS)).and_then(|s| s.strip_suffix('\n')).expect("queued, its name, and that it stays").to_string()
    }

    fn the_new_keyring_after(tag: &str, sent: impl FnOnce(store::Key, String) -> Vec<u8>, ends: Ends) -> Became {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keyring");
        let salt = store::Store::create(&path, b"test").unwrap().salt();
        let (old, user) = (program(&d.1), program(&d.1));
        let reply: u32 = old.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "RequestName", &(NAME, 0u32)).unwrap().body().deserialize().unwrap();
        assert_eq!(reply, 1, "the stand-in for the old keyring has the name");
        let (mut mine, theirs) = UnixStream::pair().unwrap();
        let (taken, t, addr) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)), d.1.clone());
        let (is_taken, over) = (taken.clone(), t.clone());
        std::thread::spawn(move || {
            let lp = Loop::new();
            let bus = Bus::at(&lp, &addr).unwrap();
            take_over_on(&bus, &lp, path, theirs, move || is_taken.store(true, Ordering::SeqCst));
            over.store(true, Ordering::SeqCst);
        });
        let new = queued_name(&mut mine);
        let (user_name, old_name) = (user.unique_name().unwrap().to_string(), old.unique_name().map(|n| n.to_string()));
        let mut old = Some(old);
        mine.write_all(&sent(store::derive(b"test", &salt), user_name)).unwrap();
        match ends {
            Ends::Dies => {
                drop(mine);
                drop(old.take());
            }
            Ends::LetsGo => {
                let reply: u32 = old.as_ref().unwrap().call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "ReleaseName", &(NAME,)).unwrap().body().deserialize().unwrap();
                assert_eq!(reply, 1);
                drop(mine);
            }
            Ends::Carries => drop(mine),
        }
        for _ in 0..500 {
            if taken.load(Ordering::SeqCst) || t.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let became = if taken.load(Ordering::SeqCst) {
            assert_eq!(owner(&user), Some(new), "the new keyring answers and the name is not its own");
            let session = user.call_method(Some(NAME), A_SESSION, Some("org.freedesktop.Secret.Session"), "Close", &()).is_ok();
            Became::Took { locked: locked(&user), session }
        } else {
            assert!(t.load(Ordering::SeqCst), "the new keyring neither took over nor ended");
            if ends == Ends::Carries {
                assert_eq!(owner(&user), old_name, "the name left the keyring that carried on");
            }
            Became::Never
        };
        let _ = std::fs::remove_dir_all(&dir);
        became
    }

    #[test]
    fn a_program_asking_as_the_name_passes_is_answered_unlocked_with_no_password_box() {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-asks-in-between-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keyring");
        let salt = store::Store::create(&path, b"test").unwrap().salt();
        let (old, user) = (program(&d.1), program(&d.1));
        let reply: u32 = old.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "RequestName", &(NAME, 0u32)).unwrap().body().deserialize().unwrap();
        assert_eq!(reply, 1);
        let (mut mine, theirs) = UnixStream::pair().unwrap();
        let addr = d.1.clone();
        std::thread::spawn(move || {
            let lp = Loop::new();
            let bus = Bus::at(&lp, &addr).unwrap();
            take_over_on(&bus, &lp, path, theirs, || {});
        });
        mine.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut said = Vec::new();
        let mut buf = [0u8; 128];
        while !said.ends_with(STAYS.as_bytes()) {
            let n = mine.read(&mut buf).expect("the new keyring never said it was queued");
            assert!(n > 0);
            said.extend_from_slice(&buf[..n]);
        }
        let said = String::from_utf8(said).unwrap();
        let new = said.strip_prefix(QUEUED).and_then(|s| s.strip_suffix(STAYS)).and_then(|s| s.strip_suffix('\n')).unwrap().to_string();
        mine.write_all(&record(&with(Some(store::derive(b"test", &salt)), ":1.0"))).unwrap();

        drop(old);
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            while owner(&user).as_deref() != Some(new.as_str()) {
                std::thread::sleep(Duration::from_millis(10));
            }
            let answer: (Vec<OwnedObjectPath>, OwnedObjectPath) = user.call_method(Some(NAME), "/org/freedesktop/secrets", Some(SERVICE),
                "Unlock", &(vec![OwnedObjectPath::try_from(LOGIN).unwrap()],)).unwrap().body().deserialize().unwrap();
            let _ = tx.send(answer);
        });
        std::thread::sleep(Duration::from_millis(400));
        drop(mine);
        let (unlocked, prompt) = rx.recv_timeout(Duration::from_secs(10)).expect("the program was not answered");
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(prompt.as_str(), "/", "a password box was asked for though the keyring was handed over open");
        assert_eq!(unlocked.len(), 1);
    }

    fn with(key: Option<store::Key>, user: &str) -> Held {
        Held { key, counter: 7, sessions: vec![(A_SESSION.into(), user.into())] }
    }

    #[test]
    fn the_new_keyring_is_unlocked_with_its_sessions_though_the_old_one_dies_in_the_hand_over() {
        let became = the_new_keyring_after("old-dies", |key, user| record(&with(Some(key), &user)).to_vec(), Ends::Dies);
        assert_eq!(became, Became::Took { locked: false, session: true });
    }

    #[test]
    fn a_keyring_locked_in_the_hand_over_is_never_handed_on_unlocked() {
        let became = the_new_keyring_after("old-locks", |key, user| {
            [record(&with(Some(key), &user)).to_vec(), record(&with(None, &user)).to_vec()].concat()
        }, Ends::Dies);
        assert_eq!(became, Became::Took { locked: true, session: true });
        let became = the_new_keyring_after("old-cut", |key, user| {
            let whole = record(&with(Some(key), &user)).to_vec();
            let cut = record(&with(None, &user));
            [whole, cut[..cut.len() - 3].to_vec()].concat()
        }, Ends::Dies);
        assert_eq!(became, Became::Took { locked: true, session: false });
    }

    #[test]
    fn the_bus_says_whether_the_new_keyring_has_the_name_and_the_socket_never_does() {
        assert_eq!(the_new_keyring_after("old-silent", |_, _| Vec::new(), Ends::Dies), Became::Took { locked: true, session: false });
        assert_eq!(the_new_keyring_after("old-carries", |key, user| record(&with(Some(key), &user)).to_vec(), Ends::Carries), Became::Never);
        let let_go = |key, user: String| [LET_GO.to_vec(), pack(&with(Some(key), &user)).to_vec()].concat();
        assert_eq!(the_new_keyring_after("old-pretends", let_go, Ends::Carries), Became::Never);
    }

    #[test]
    fn a_keyring_from_before_hands_over_to_the_new_one_as_it_always_did() {
        let let_go = |key, user: String| [LET_GO.to_vec(), pack(&with(Some(key), &user)).to_vec()].concat();
        assert_eq!(the_new_keyring_after("old-before", let_go, Ends::LetsGo), Became::Took { locked: false, session: true });
    }

    fn an_unlocked_keyring(tag: &str) -> (crate::toolkit::bus::testing::Daemon, std::path::PathBuf, Loop, Bus, Rc<Keyring>) {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keyring");
        let salt = store::Store::create(&path, b"test").unwrap().salt();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start_with(&bus, &lp, path, || Some(store::derive(b"test", &salt))).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get() == Some(true)), "the test's keyring has its name");
        (d, dir, lp, bus, keyring)
    }

    #[test]
    fn the_old_keyring_takes_the_name_back_and_carries_on_when_the_new_one_dies_in_the_hand_over() {
        let (d, dir, lp, bus, keyring) = an_unlocked_keyring("new-dies");
        let (addr, old_name) = (d.1.clone(), bus.unique_name());
        let (opened, go, done) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)));
        let (o, g, dn) = (opened.clone(), go.clone(), done.clone());

        let user = std::thread::spawn(move || {
            let c = program(&addr);
            let (_, session): (OwnedValue, OwnedObjectPath) = c.call_method(Some(NAME), "/org/freedesktop/secrets", Some(SERVICE),
                "OpenSession", &("plain", Value::from(""))).unwrap().body().deserialize().unwrap();
            o.store(true, Ordering::SeqCst);
            while !g.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(10));
            }
            let found = (owner(&c), locked(&c), c.call_method(Some(NAME), session.as_str(), Some("org.freedesktop.Secret.Session"), "Close", &()).is_ok());
            dn.store(true, Ordering::SeqCst);
            found
        });
        assert!(until(&lp, move || opened.load(Ordering::SeqCst)), "the program never got its session");

        let (mine, mut theirs) = UnixStream::pair().unwrap();
        let addr = d.1.clone();
        let new = std::thread::spawn(move || {
            let c = program(&addr);
            let reply: u32 = c.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "RequestName", &(NAME, 0u32)).unwrap().body().deserialize().unwrap();
            assert_eq!(reply, 2, "the new keyring is queued");
            let me = c.unique_name().unwrap().to_string();
            theirs.write_all(format!("{QUEUED}{me}\n{STAYS}").as_bytes()).unwrap();
            for _ in 0..500 {
                if owner(&c).as_deref() == Some(me.as_str()) {
                    return true;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            false
        });
        let (handing, failed) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(false)));
        let f = failed.clone();
        hand_over(&keyring, &bus, &lp, mine, handing.clone(), move || f.set(true));
        let f = failed.clone();
        assert!(until(&lp, move || f.get()), "the old keyring left though the new one died with the name");
        assert!(new.join().unwrap(), "the new keyring was never handed the name");
        assert!(!handing.get(), "the old keyring still counts the name as let go");
        go.store(true, Ordering::SeqCst);
        assert!(until(&lp, move || done.load(Ordering::SeqCst)), "the program was not answered");
        let (owner, locked, session) = user.join().unwrap();
        assert_eq!(owner, Some(old_name), "the name is not back with the old keyring");
        assert!(!locked, "the keyring that carried on is locked");
        assert!(session, "the session opened before the hand-over was dropped");
        assert!(keyring.resting());
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_new_keyring_from_before_is_handed_over_to_as_it_always_was() {
        let (d, dir, lp, bus, keyring) = an_unlocked_keyring("new-before");
        let (mine, mut theirs) = UnixStream::pair().unwrap();
        let addr = d.1.clone();
        let new = std::thread::spawn(move || {
            let c = program(&addr);
            let _ = c.call_method(Some(DBUS), DBUS_PATH, Some(DBUS), "RequestName", &(NAME, 0u32)).unwrap();
            theirs.write_all(format!("{QUEUED}{}\n", c.unique_name().unwrap()).as_bytes()).unwrap();
            theirs.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            let mut said = Vec::new();
            let _ = theirs.read_to_end(&mut said);
            (said, owner(&c) == c.unique_name().map(|n| n.to_string()))
        });
        let (handing, failed) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(false)));
        let f = failed.clone();
        hand_over(&keyring, &bus, &lp, mine, handing.clone(), move || f.set(true));

        until(&lp, || false);
        assert!(handing.get() && !failed.get());
        drop(keyring);
        drop(bus);
        let (said, has_name) = new.join().unwrap();
        let held = said.strip_prefix(LET_GO).and_then(unpack).expect("the word that the name is let go, then what is held, whole");
        assert!(held.key.is_some() && has_name);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_change_of_lock_is_told_as_it_happens() {
        let (d, dir, lp, _bus, keyring) = an_unlocked_keyring("told");
        let told = Rc::new(Cell::new(0u32));
        let t = told.clone();
        keyring.when_changed(Some(Box::new(move || t.set(t.get() + 1))));
        let (addr, done) = (d.1.clone(), Arc::new(AtomicBool::new(false)));
        let dn = done.clone();
        let user = std::thread::spawn(move || {
            let c = program(&addr);
            c.call_method(Some(NAME), "/org/freedesktop/secrets", Some(SERVICE), "Lock", &(vec![OwnedObjectPath::try_from(LOGIN).unwrap()],)).unwrap();
            let is = locked(&c);
            dn.store(true, Ordering::SeqCst);
            is
        });
        assert!(until(&lp, move || done.load(Ordering::SeqCst)));
        assert!(user.join().unwrap(), "the keyring was not locked");
        assert_eq!(told.get(), 1, "the lock changed and it was not told, or told more than once");
        keyring.when_changed(None);
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
