//! The guard against a pretend /sys in a scratch folder.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use super::sysfs::fake;
use super::*;

const DOCK: &str = "d0c0-0001";
const DISK: &str = "d15c-0002";

struct Scratch {
    root: PathBuf,
    places: Places,
}

impl Scratch {
    fn new(level: &str, iommu: bool) -> Scratch {
        let root = crate::common::files::private_temp("thunder").unwrap();
        let places = Places {
            sys: root.join("sys"),
            state: root.join("state"),
            socket: root.join("run/socket"),
            seat: root.join("seat0"),
            run_user: root.join("user"),
        };
        fake::domain(&places.sys, 0, level, iommu);
        // SAFETY: getuid cannot fail and touches no memory.
        let me = unsafe { libc::getuid() };
        fs::write(&places.seat, format!("IS_SEAT0=1\nACTIVE_UID={me}\n")).unwrap();
        fs::create_dir_all(places.run_user.join(me.to_string())).unwrap();
        Scratch { root, places }
    }

    fn dock(&self, secure: bool) {
        fake::plug(&self.places.sys, "0-0", "0-1", DOCK, "Dock Makers", "Big Dock", secure);
    }

    fn unplug_dock(&self) {
        fake::unplug(&self.places.sys, "0-1");
    }

    fn lock(&self, on: bool) {
        // SAFETY: as above.
        let me = unsafe { libc::getuid() };
        let f = self.places.run_user.join(me.to_string()).join(crate::common::locked::NAME);
        if on { fs::write(f, "").unwrap() } else { let _ = fs::remove_file(f); }
    }

    fn authorized(&self, kernel: &str) -> String {
        fake::authorized(&self.places.sys, kernel)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Ok(dir) = fs::read_dir(sysfs::bus(&self.places.sys)) {
            for e in dir.flatten() {
                let _ = fs::set_permissions(e.path().join("authorized"), fs::Permissions::from_mode(0o644));
            }
        }
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn asked(g: &mut Guard) -> Vec<String> {
    g.news.drain(..).filter(|l| l.starts_with("ask\t")).collect()
}

#[test]
fn a_new_device_waits_and_is_asked_about() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    g.rescan();
    assert!(g.waiting().is_empty());
    s.dock(false);
    g.rescan();
    let q = g.waiting();
    assert_eq!(q.len(), 1);
    assert_eq!((q[0].id.as_str(), q[0].vendor.as_str(), q[0].name.as_str()), (DOCK, "Dock Makers", "Big Dock"));
    assert_eq!(asked(&mut g), vec![q[0].line()]);
    assert_eq!(Question::parse(&q[0].line()).as_ref(), Some(&q[0]));
    assert_eq!(s.authorized("0-1"), "0", "nothing connects before the answer");
    g.rescan();
    assert!(asked(&mut g).is_empty());
}

#[test]
fn allowed_always_connects_now_and_every_time_after() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    assert_eq!(g.answer(DOCK, Choice::Always, false), Outcome::Ok);
    assert_eq!(s.authorized("0-1"), "1");
    assert!(g.news.contains(&format!("gone\t{DOCK}")));
    assert_eq!(g.allowed().len(), 1);
    s.unplug_dock();
    s.dock(false);
    let mut again = Guard::new(s.places.clone());
    again.rescan();
    assert_eq!(s.authorized("0-1"), "1");
    assert!(again.waiting().is_empty() && asked(&mut again).is_empty());
}

#[test]
fn allowed_this_time_is_not_remembered() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    assert_eq!(g.answer(DOCK, Choice::Once, false), Outcome::Ok);
    assert_eq!(s.authorized("0-1"), "1");
    assert!(g.allowed().is_empty());
    s.unplug_dock();
    g.rescan();
    s.dock(false);
    g.rescan();
    assert_eq!(s.authorized("0-1"), "0");
    assert_eq!(g.waiting().len(), 1, "asked again next time");
}

#[test]
fn refused_stays_out_until_plugged_in_again() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    asked(&mut g);
    assert_eq!(g.answer(DOCK, Choice::Never, false), Outcome::Ok);
    assert_eq!(s.authorized("0-1"), "0");
    g.rescan();
    assert!(g.waiting().is_empty() && asked(&mut g).is_empty(), "not asked again while it stays plugged in");
    s.unplug_dock();
    g.rescan();
    s.dock(false);
    g.rescan();
    assert_eq!(asked(&mut g).len(), 1);
    assert_eq!(g.answer("ffff-0000", Choice::Always, false), Outcome::Unknown);
}

#[test]
fn a_forgotten_device_is_asked_about_again() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    g.answer(DOCK, Choice::Always, false);
    assert!(g.forget(DOCK));
    assert!(!g.forget(DOCK));
    assert!(allowed::load(&s.places.state).is_empty());
    s.unplug_dock();
    g.rescan();
    s.dock(false);
    g.rescan();
    assert_eq!(s.authorized("0-1"), "0");
    assert_eq!(g.waiting().len(), 1);
}

#[test]
fn while_locked_nothing_new_gets_in_but_known_devices_do() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.lock(true);
    assert!(g.locked());
    s.dock(false);
    g.rescan();
    assert_eq!(g.answer(DOCK, Choice::Always, g.locked()), Outcome::Locked);
    assert_eq!(g.answer(DOCK, Choice::Once, g.locked()), Outcome::Locked);
    assert_eq!(s.authorized("0-1"), "0");
    assert_eq!(g.waiting().len(), 1, "the question waits for the unlock");
    s.lock(false);
    assert!(!g.locked());
    assert_eq!(g.answer(DOCK, Choice::Always, g.locked()), Outcome::Ok);
    s.lock(true);
    s.unplug_dock();
    g.rescan();
    s.dock(false);
    g.rescan();
    assert_eq!(s.authorized("0-1"), "1");
    fs::write(&s.places.seat, "IS_SEAT0=1\n").unwrap();
    assert!(g.locked());
}

#[test]
fn levels_that_let_everything_in_ask_nothing() {
    for level in ["none", "dponly", "usbonly", "nopcie"] {
        let s = Scratch::new(level, false);
        let mut g = Guard::new(s.places.clone());
        s.dock(false);
        g.rescan();
        assert!(g.waiting().is_empty(), "{level}");
        assert_eq!(s.authorized("0-1"), "0", "{level}: left to the kernel");
    }
    let s = Scratch::new("user", true);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    let q = g.waiting();
    assert!(q[0].iommu && q[0].level == Level::User, "asked once even with the IOMMU on");
    assert!(g.list().contains(&"level\t0\tuser\t1".to_string()));
}

#[test]
fn the_secure_level_puts_a_key_on_and_checks_it() {
    let s = Scratch::new("secure", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(true);
    g.rescan();
    assert_eq!(g.answer(DOCK, Choice::Always, false), Outcome::Ok);
    assert_eq!(s.authorized("0-1"), "1", "first time: the key goes on, then 1");
    let key = fake::key(&s.places.sys, "0-1");
    assert_eq!(key.len(), 64);
    assert_eq!(g.allowed()[0].key.as_deref(), Some(key.as_str()));
    s.unplug_dock();
    g.rescan();
    s.dock(true);
    g.rescan();
    assert_eq!(s.authorized("0-1"), "2");
    assert_eq!(fake::key(&s.places.sys, "0-1"), key);
    s.unplug_dock();
    g.rescan();
    s.dock(true);
    fs::set_permissions(sysfs::bus(&s.places.sys).join("0-1/authorized"), fs::Permissions::from_mode(0o444)).unwrap();
    g.rescan();
    let q = g.waiting();
    assert!(q.len() == 1 && q[0].doubt, "{q:?}");
    let s = Scratch::new("secure", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(true);
    g.rescan();
    assert_eq!(g.answer(DOCK, Choice::Once, false), Outcome::Ok);
    assert_eq!(s.authorized("0-1"), "1");
    assert!(g.allowed().is_empty());
}

#[test]
fn a_failed_connection_is_said_and_still_waits() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    g.rescan();
    fs::set_permissions(sysfs::bus(&s.places.sys).join("0-1/authorized"), fs::Permissions::from_mode(0o444)).unwrap();
    assert!(matches!(g.answer(DOCK, Choice::Always, false), Outcome::Failed(_)));
    assert!(g.allowed().is_empty());
    assert_eq!(g.waiting().len(), 1);
}

#[test]
fn what_is_behind_a_dock_is_asked_about_once_the_dock_is_in() {
    let s = Scratch::new("user", false);
    let mut g = Guard::new(s.places.clone());
    s.dock(false);
    fake::plug(&s.places.sys, "0-1", "0-301", DISK, "Disk Co", "Fast Disk", false);
    g.rescan();
    assert_eq!(g.waiting().iter().map(|q| q.id.as_str()).collect::<Vec<_>>(), [DOCK]);
    g.answer(DOCK, Choice::Always, false);
    assert_eq!(g.waiting().iter().map(|q| q.id.as_str()).collect::<Vec<_>>(), [DISK]);
    g.answer(DISK, Choice::Always, false);
    fake::unplug(&s.places.sys, "0-1");
    g.rescan();
    s.dock(false);
    fake::plug(&s.places.sys, "0-1", "0-301", DISK, "Disk Co", "Fast Disk", false);
    g.rescan();
    assert_eq!((s.authorized("0-1"), s.authorized("0-301")), ("1".into(), "1".into()));
}

fn line(r: &mut BufReader<UnixStream>) -> String {
    let mut l = String::new();
    r.read_line(&mut l).unwrap();
    l.trim_end().to_string()
}

fn connect(socket: &Path) -> (UnixStream, BufReader<UnixStream>) {
    for _ in 0..100 {
        if let Ok(s) = UnixStream::connect(socket) {
            s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let r = BufReader::new(s.try_clone().unwrap());
            return (s, r);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("the service never opened its socket");
}

#[test]
fn the_socket_asks_the_person_at_the_screen_and_nobody_else() {
    let s = Scratch::new("user", true);
    let stop = Arc::new(AtomicBool::new(false));
    let places = s.places.clone();
    let st = stop.clone();
    let service = std::thread::spawn(move || {
        serve(places, Opts { uevents: false, every: Some(Duration::from_millis(30)), stop: Some(st) }).unwrap();
    });
    let (mut w, mut r) = connect(&s.places.socket);
    let mode = fs::metadata(&s.places.socket);
    assert_eq!(mode.map(|m| m.permissions().mode() & 0o777).ok(), Some(0o666));
    w.write_all(b"watch\n").unwrap();
    s.dock(false);
    let ask = line(&mut r);
    let q = Question::parse(&ask).expect(&ask);
    assert_eq!((q.id.as_str(), q.iommu), (DOCK, true));
    w.write_all(format!("answer\t{DOCK}\talways\n").as_bytes()).unwrap();
    let mut heard = vec![line(&mut r), line(&mut r)];
    heard.sort();
    assert_eq!(heard, vec![format!("answered\t{DOCK}\tok"), format!("gone\t{DOCK}")]);
    assert_eq!(s.authorized("0-1"), "1");

    let (mut w2, mut r2) = connect(&s.places.socket);
    w2.write_all(b"list\n").unwrap();
    let mut listed = Vec::new();
    loop {
        let l = line(&mut r2);
        if l == "end" {
            break;
        }
        listed.push(l);
    }
    let l = parse_list(&listed);
    assert_eq!(l.devices, vec![(DOCK.to_string(), "Dock Makers".to_string(), "Big Dock".to_string())]);
    assert_eq!(l.domains.len(), 1);
    w2.write_all(format!("forget\t{DOCK}\n").as_bytes()).unwrap();
    assert_eq!(line(&mut r2), "ok");

    fs::write(&s.places.seat, "IS_SEAT0=1\nACTIVE_UID=4242\n").unwrap();
    let (mut w3, mut r3) = connect(&s.places.socket);
    w3.write_all(b"list\n").unwrap();
    assert_eq!(line(&mut r3), "denied");
    stop.store(true, Ordering::Relaxed);
    service.join().unwrap();
}
