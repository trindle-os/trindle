//! The service against a pretend kernel: hostile frames, refused keyboards and mice, limits, cleanup and the peer check.

use std::fmt::Write as _;
use super::*;
use crate::common::controller_wire::{answer, double, pair, parse, recv_frame, send_frame, Axis, Client};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Fake {
    live: Arc<AtomicUsize>,
    made: Arc<AtomicUsize>,
    sent: Arc<Mutex<Vec<Event>>>,
}

struct FakeDev(Arc<AtomicUsize>);

impl Drop for FakeDev {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

impl Kernel for Fake {
    type Made = FakeDev;
    fn make(&mut self, pad: &Pad) -> io::Result<FakeDev> {
        assert!(pad.check().is_ok(), "the kernel was asked for something not a pad");
        self.live.fetch_add(1, Ordering::SeqCst);
        self.made.fetch_add(1, Ordering::SeqCst);
        Ok(FakeDev(self.live.clone()))
    }
    fn emit(&mut self, _: &mut FakeDev, events: &[Event]) -> io::Result<()> {
        self.sent.lock().unwrap().extend_from_slice(events);
        Ok(())
    }
}

fn pad() -> Pad {
    Pad {
        bus: 3,
        vendor: 0x054c,
        product: 0x09cc,
        version: 1,
        keys: vec![0x130, 0x131, 0x133, 0x134, 0x13a, 0x13b, 0x220],
        axes: vec![Axis { code: 0, min: -32768, max: 32767, fuzz: 16, flat: 128 }, Axis { code: 0x10, min: -1, max: 1, fuzz: 0, flat: 0 }],
    }
}

fn make(slot: u8, pad: Pad) -> Vec<u8> {
    wire::encode(&Request::Make { slot, pad })
}

fn send(slot: u8, events: &[(u16, u16, i32)]) -> Vec<u8> {
    wire::encode(&Request::Send { slot, events: events.iter().map(|&(kind, code, value)| Event { kind, code, value }).collect() })
}

fn service(conns: usize) -> (Service<Fake>, Fake, Vec<OwnedFd>, Instant) {
    let fake = Fake::default();
    let mut s = Service::new(fake.clone());
    let now = Instant::now();
    let mut theirs = Vec::new();
    for _ in 0..conns {
        let (a, b) = pair().unwrap();
        s.add(a, now);
        theirs.push(b);
    }
    (s, fake, theirs, now)
}

fn refused(why: Refusal, slot: u8) -> Step {
    Step::Answer(answer(slot, Err(why)))
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn hostile_frames_never_panic_and_never_reach_the_kernel_as_anything_but_a_pad() {
    let mut rng = Rng(0x7269_6e64_6c65);
    let seeds = [make(0, pad()), send(0, &[(EV_KEY, 0x130, 1), (EV_SYN, 0, 0)]), wire::encode(&Request::Remove { slot: 0 })];
    let (mut s, fake, _theirs, now) = service(1);
    for round in 0..200_000u32 {
        let frame: Vec<u8> = if round % 2 == 0 {
            let len = (rng.next() % 700) as usize;
            let mut f: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
            if let Some(first) = f.first_mut() {
                *first = [b'M', b'S', b'R', rng.next() as u8][(rng.next() % 4) as usize];
            }
            f
        } else {
            let mut f = seeds[(rng.next() % 3) as usize].clone();
            for _ in 0..1 + rng.next() % 4 {
                match rng.next() % 3 {
                    0 if !f.is_empty() => {
                        let i = (rng.next() as usize) % f.len();
                        f[i] ^= 1 << (rng.next() % 8);
                    }
                    1 => f.truncate((rng.next() as usize) % (f.len() + 1)),
                    _ => f.push(rng.next() as u8),
                }
            }
            f
        };
        if let Ok(Request::Make { pad, .. }) = parse(&frame) {
            assert!(pad.check().is_ok());
        }
        if s.conns.is_empty() {
            let (a, _b) = pair().unwrap();
            s.add(a, now);
        }
        if s.handle(0, &frame, now) == Step::Close {
            s.conns.remove(0);
        }
        assert!(s.pads() <= SLOTS as usize);
    }
    assert!(fake.made.load(Ordering::SeqCst) > 0, "the valid seeds made some pads");
}

#[test]
fn every_kind_of_frame_reads_back_as_written() {
    let reqs = [
        Request::Make { slot: 7, pad: pad() },
        Request::Send { slot: 1, events: vec![Event { kind: EV_ABS, code: 0, value: -5 }] },
        Request::Remove { slot: 3 },
    ];
    for r in reqs {
        assert_eq!(parse(&wire::encode(&r)), Ok(r));
    }
    let mut long = make(0, pad());
    long.push(0);
    assert_eq!(parse(&long), Err(Refusal::Malformed));
    assert_eq!(parse(&make(0, pad())[..20]), Err(Refusal::Malformed));
    assert_eq!(parse(&[]), Err(Refusal::Malformed));
    assert_eq!(parse(b"M"), Err(Refusal::Malformed));
    assert_eq!(parse(&vec![b'S'; LONGEST + 1]), Err(Refusal::Malformed));
    let fullest = Pad { keys: (0x120..=0x13e).chain(0x220..=0x223).chain(0x2c0..=0x2e7).collect(), axes: (0..=0x0a).chain(0x10..=0x17).map(|code| Axis { code, min: 0, max: 1, fuzz: 0, flat: 0 }).collect(), ..pad() };
    assert!(matches!(parse(&make(0, fullest)), Ok(Request::Make { .. })), "every pad button and axis at once fits");
    assert!(matches!(parse(&send(0, &[(EV_SYN, 0, 0); wire::MOST_EVENTS])), Ok(Request::Send { .. })), "the most events at once fit");
    assert_eq!(parse(&[b'S', 0, 0, 0]), Err(Refusal::Malformed), "an empty send");
    assert_eq!(parse(&wire::encode(&Request::Remove { slot: SLOTS })), Err(Refusal::BadSlot));
}

#[test]
fn keyboards_mice_tablets_and_touch_are_refused() {
    let with_key = |k: u16| Pad { keys: vec![0x130, k], ..pad() };
    let with_axis = |code: u16| Pad { axes: vec![Axis { code, min: 0, max: 255, fuzz: 0, flat: 0 }], ..pad() };

    for k in [1, 30, 28, 0x160, 0x110, 0x140, 0x14a, 0x100, 0x2e8, 0x13f, 0x11f, 0x224] {
        assert_eq!(with_key(k).check(), Err(Refusal::NotAPadButton), "key {k:#x}");
    }

    for a in [0x18, 0x19, 0x1a, 0x28, 0x2f, 0x35, 0x36, 0x0b, 0x0f, 0x3f] {
        assert_eq!(with_axis(a).check(), Err(Refusal::NotAPadAxis), "axis {a:#x}");
    }
    assert_eq!(Pad { keys: vec![0x220, 0x2c0], ..pad() }.check(), Err(Refusal::NoPadButton), "d-pad alone is not filed as a joystick");
    assert_eq!(Pad { keys: vec![], ..pad() }.check(), Err(Refusal::NoPadButton));
    assert_eq!(Pad { keys: vec![0x130, 0x130], ..pad() }.check(), Err(Refusal::Twice));
    assert_eq!(Pad { bus: 0x11, ..pad() }.check(), Err(Refusal::BadBus), "i8042, a built-in keyboard's bus");
    let bad_range = Pad { axes: vec![Axis { code: 0, min: 5, max: 5, fuzz: 0, flat: 0 }], ..pad() };
    assert_eq!(bad_range.check(), Err(Refusal::BadRange));
    let bad_flat = Pad { axes: vec![Axis { code: 0, min: 0, max: 10, fuzz: 0, flat: 11 }], ..pad() };
    assert_eq!(bad_flat.check(), Err(Refusal::BadRange));
    let wide = Pad { axes: vec![Axis { code: 0, min: i32::MIN, max: i32::MAX, fuzz: i32::MAX, flat: 0 }], ..pad() };
    assert_eq!(wide.check(), Ok(()));

    let (mut s, fake, _t, now) = service(1);
    assert_eq!(s.handle(0, &make(0, with_key(30)), now), refused(Refusal::NotAPadButton, 0));
    assert_eq!(s.handle(0, &make(1, with_axis(0x35)), now), refused(Refusal::NotAPadAxis, 1));
    assert_eq!(fake.made.load(Ordering::SeqCst), 0, "the kernel never saw them");
}

#[test]
fn events_must_match_what_the_pad_was_made_with() {
    let (mut s, fake, _t, now) = service(1);
    assert_eq!(s.handle(0, &make(0, pad()), now), Step::Answer(answer(0, Ok(()))));
    assert_eq!(s.handle(0, &send(0, &[(EV_KEY, 0x130, 1), (EV_ABS, 0, 99_999), (EV_ABS, 0x10, -7), (EV_SYN, 0, 0)]), now), Step::Quiet);
    let sent = fake.sent.lock().unwrap().clone();
    assert_eq!(sent.iter().map(|e| e.value).collect::<Vec<_>>(), [1, 32767, -1, 0], "axes kept inside their range");
    assert_eq!(s.handle(0, &send(5, &[(EV_KEY, 0x130, 1)]), now), Step::Quiet, "nothing made in that slot");
    for hostile in [(EV_KEY, 30, 1), (EV_KEY, 0x132, 1), (EV_KEY, 0x130, 2), (2, 0, 5), (EV_ABS, 1, 0), (EV_SYN, 2, 0), (4, 4, 30), (0x15, 0, 1)] {
        let (mut s, fake, _t, now) = service(1);
        s.handle(0, &make(0, pad()), now);
        assert_eq!(s.handle(0, &send(0, &[(EV_KEY, 0x130, 1), hostile]), now), Step::Close, "{hostile:?}");
        assert!(fake.sent.lock().unwrap().is_empty(), "nothing of a refused batch goes out");
    }
}

#[test]
fn counts_are_limited() {
    let (mut s, fake, _t, now) = service(3);
    for slot in 0..SLOTS {
        assert_eq!(s.handle(0, &make(slot, pad()), now), Step::Answer(answer(slot, Ok(()))));
    }
    assert_eq!(s.handle(0, &make(0, pad()), now), refused(Refusal::SlotInUse, 0));
    assert_eq!(s.handle(0, &make(SLOTS, pad()), now), refused(Refusal::BadSlot, SLOTS));
    for slot in 0..SLOTS {
        s.handle(1, &make(slot, pad()), now);
    }
    assert_eq!(s.pads(), MOST_PADS);
    assert_eq!(s.handle(2, &make(0, pad()), now), refused(Refusal::TooMany, 0));
    assert_eq!(s.handle(0, &wire::encode(&Request::Remove { slot: 3 }), now), Step::Answer(answer(3, Ok(()))));
    assert_eq!(s.handle(0, &wire::encode(&Request::Remove { slot: 3 }), now), refused(Refusal::NotMade, 3));
    assert_eq!(fake.live.load(Ordering::SeqCst), MOST_PADS - 1);
    assert_eq!(s.handle(2, &make(0, pad()), now), Step::Answer(answer(0, Ok(()))));
}

#[test]
fn makes_and_events_are_rate_limited() {
    let (mut s, fake, _t, now) = service(1);
    for slot in 0..MAKES.0 as u8 {
        s.handle(0, &make(slot, pad()), now);
        s.handle(0, &wire::encode(&Request::Remove { slot }), now);
    }
    assert_eq!(s.handle(0, &make(0, pad()), now), refused(Refusal::TooFast, 0));
    let later = now + Duration::from_secs(1);
    assert_eq!(s.handle(0, &make(0, pad()), later), Step::Answer(answer(0, Ok(()))));

    let batch: Vec<_> = (0..64).map(|i| (EV_KEY, 0x130, i % 2)).collect();
    let frame = send(0, &batch);
    let fits = (EVENTS.0 as usize) / 64;
    for _ in 0..fits + 10 {
        assert_eq!(s.handle(0, &frame, later), Step::Quiet);
    }
    assert_eq!(fake.sent.lock().unwrap().len(), fits * 64);
    assert_eq!(s.conns[0].dropped, 10 * 64);
    s.handle(0, &frame, later + Duration::from_millis(10));

    assert_eq!(fake.sent.lock().unwrap().len(), (fits + 1) * 64 + 2, "a hundredth of a second later more go out");
}

#[test]
fn a_release_kept_back_by_the_rate_limit_is_sent_when_there_is_room() {
    let (mut s, fake, _t, now) = service(1);
    s.handle(0, &make(0, pad()), now);
    let burst: Vec<_> = (0..64).map(|_| (EV_KEY, 0x130, 1)).collect();
    for _ in 0..(EVENTS.0 as usize - 1) / 64 {
        s.handle(0, &send(0, &burst), now);
    }
    s.handle(0, &send(0, &burst[..(EVENTS.0 as usize - 1) % 64]), now);
    assert_eq!(s.handle(0, &send(0, &[(EV_KEY, 0x130, 0), (EV_SYN, 0, 0)]), now), Step::Quiet);
    let sent = |v: &Vec<Event>| v.iter().rev().find(|e| e.kind == EV_KEY).map(|e| e.value);
    assert_eq!(sent(&fake.sent.lock().unwrap()), Some(1), "the release was kept back");
    assert!(s.owing());
    s.settle(now + Duration::from_secs(1));
    assert!(!s.owing());
    assert_eq!(sent(&fake.sent.lock().unwrap()), Some(0), "the button is let go once there is room");
}

#[test]
fn a_request_to_make_a_controller_is_judged_again() {
    let (mut s, fake, theirs, _now) = service(1);
    send_frame(theirs[0].as_raw_fd(), &make(0, pad())).unwrap();
    s.turn(None, &|_| Err(Refusal::NotAllowed), 100).unwrap();
    assert_eq!(fake.made.load(Ordering::SeqCst), 0, "no controller for a person who is no longer at the screen");
    let mut buf = [0u8; LONGEST];
    let n = recv_frame(theirs[0].as_raw_fd(), &mut buf).unwrap().unwrap();
    assert_eq!(&buf[..n], &answer(0, Err(Refusal::NotAllowed))[..]);
    send_frame(theirs[0].as_raw_fd(), &make(0, pad())).unwrap();
    s.turn(None, &|_| Ok(()), 100).unwrap();
    assert_eq!(fake.made.load(Ordering::SeqCst), 1);
}

#[test]
fn a_controller_that_will_not_take_events_is_let_go() {
    #[derive(Default)]
    struct Deaf;
    impl Kernel for Deaf {
        type Made = ();
        fn make(&mut self, _: &Pad) -> io::Result<()> { Ok(()) }
        fn emit(&mut self, _: &mut (), _: &[Event]) -> io::Result<()> { Err(io::Error::other("gone")) }
    }
    let mut s = Service::new(Deaf);
    let (a, _b) = pair().unwrap();
    let now = Instant::now();
    s.add(a, now);
    s.handle(0, &make(0, pad()), now);
    assert_eq!(s.handle(0, &send(0, &[(EV_KEY, 0x130, 1), (EV_SYN, 0, 0)]), now), Step::Close);
}

#[test]
fn a_controller_goes_when_its_connection_does() {
    let (mut s, fake, mut theirs, now) = service(2);
    s.handle(0, &make(0, pad()), now);
    s.handle(0, &make(1, pad()), now);
    s.handle(1, &make(0, pad()), now);
    assert_eq!(fake.live.load(Ordering::SeqCst), 3);
    drop(theirs.remove(0));
    s.turn(None, &|_| Ok(()), 100).unwrap();
    assert_eq!(s.conns.len(), 1);
    assert_eq!(fake.live.load(Ordering::SeqCst), 1, "only the other connection's pad is left");

    send_frame(theirs[0].as_raw_fd(), &[b'Q', 0]).unwrap();
    s.turn(None, &|_| Ok(()), 100).unwrap();
    assert!(s.conns.is_empty());
    assert_eq!(fake.live.load(Ordering::SeqCst), 0);
}

#[test]
fn the_compositors_client_works_against_the_service() {
    let (ours, theirs) = pair().unwrap();
    let fake = Fake::default();
    let seen = fake.clone();
    let server = std::thread::spawn(move || {
        let mut s = Service::new(fake);
        s.add(theirs, Instant::now());
        while !s.conns.is_empty() {
            s.turn(None, &|_| Ok(()), 1000).unwrap();
        }
    });
    let client = Client::from_fd(ours).unwrap();
    assert_eq!(client.make(2, &pad()).unwrap(), Ok(()));
    assert_eq!(client.make(2, &pad()).unwrap(), Err(Refusal::SlotInUse));
    assert_eq!(client.make(3, &Pad { keys: vec![30], ..pad() }).unwrap(), Err(Refusal::NotAPadButton));
    let events: Vec<_> = (0..100).map(|_| Event { kind: EV_KEY, code: 0x131, value: 1 }).collect();
    client.send(2, &events).unwrap();
    assert_eq!(client.remove(2).unwrap(), Ok(()), "answered after the sends, so they arrived");
    assert_eq!(seen.sent.lock().unwrap().len(), 100);
    drop(client);
    server.join().unwrap();
    assert_eq!(seen.live.load(Ordering::SeqCst), 0);
}

#[test]
fn the_test_double_records_what_the_compositor_asked() {
    let (client, seen) = double().unwrap();
    assert_eq!(client.make(0, &pad()).unwrap(), Ok(()));
    client.send(0, &[Event { kind: EV_SYN, code: 0, value: 0 }]).unwrap();
    assert_eq!(client.remove(0).unwrap(), Ok(()));
    drop(client);
    let got = seen.join().unwrap();
    assert!(matches!(got.as_slice(), [Request::Make { slot: 0, .. }, Request::Send { slot: 0, .. }, Request::Remove { slot: 0 }]));
}

fn our_exe() -> Allowed {
    let path = std::env::current_exe().unwrap();
    let m = fs::metadata(&path).unwrap();
    Allowed { path: path.to_string_lossy().into_owned(), dev: m.dev(), ino: m.ino() }
}

fn seat(uid: u32) -> PathBuf {
    let dir = crate::common::files::private_temp("controller-seat").unwrap();
    let file = dir.join("seat0");
    fs::write(&file, format!("IS_SEAT0=1\nACTIVE=c2\nACTIVE_UID={uid}\n")).unwrap();
    file
}

#[test]
fn the_peer_is_read_from_the_kernel_and_proc() {
    let (a, b) = pair().unwrap();
    let (peer, pidfd) = peer_of(a.as_raw_fd()).unwrap();
    // SAFETY: no pointers.
    let (uid, gid) = unsafe { (libc::getuid(), libc::getgid()) };
    assert_eq!((peer.pid as u32, peer.uid, peer.gid), (std::process::id(), uid, gid));
    assert!(pidfd.is_some(), "this kernel gives a pidfd for the peer");
    let f = gather(&peer, pidfd.as_ref(), &seat(uid));
    let mine = our_exe();
    let exe = f.exe.clone().unwrap();
    assert_eq!((exe.dev, exe.ino), (mine.dev, mine.ino));
    assert!(f.alive);
    assert_eq!(f.at_seat, Some(uid));
    // SAFETY: no pointers; asks only for the current file ids back.
    assert_eq!(unsafe { libc::setfsuid(u32::MAX) } as u32, uid, "the file identity is put back");

    assert_eq!(judge(&peer, &f, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed));
    assert!(!f.maps_clean, "the test program's own code is not in a system place");
    let as_root = Facts { exe: Some(Exe { owner: 0, mode: 0o100755, ..exe.clone() }), env_clean: true, maps_clean: true, ..f.clone() };
    assert_eq!(judge(&peer, &as_root, std::slice::from_ref(&mine)), Ok(()));
    assert_eq!(judge(&peer, &as_root, &allowed(ALLOWED)), Err(Refusal::NotAllowed), "a different program");
    assert_eq!(judge(&peer, &Facts { at_seat: Some(uid + 1), ..as_root.clone() }, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed), "not the person at the screen");
    assert_eq!(judge(&peer, &Facts { at_seat: None, ..as_root.clone() }, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed), "nobody at the screen");
    assert_eq!(judge(&Peer { uid: uid + 1, ..peer }, &as_root, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed), "a wrong uid");
    assert_eq!(judge(&peer, &Facts { env_clean: false, ..as_root.clone() }, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed));
    assert_eq!(judge(&peer, &Facts { alive: false, ..as_root.clone() }, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed));
    let writable = Facts { exe: Some(Exe { mode: 0o100775, ..as_root.exe.clone().unwrap() }), ..as_root.clone() };
    assert_eq!(judge(&peer, &writable, std::slice::from_ref(&mine)), Err(Refusal::NotAllowed), "a file others may write");
    drop(b);
}

#[test]
fn a_compositor_whose_file_was_replaced_by_an_upgrade_is_still_let_in() {
    let peer = Peer { pid: 1, uid: 1000, gid: 1000 };
    let list = [Allowed { path: ALLOWED[0].into(), dev: 7, ino: 100 }];
    let old = Exe { dev: 7, ino: 99, owner: 0, mode: 0o100755, links: 0, link: format!("{} (deleted)", ALLOWED[0]) };
    let f = Facts { at_seat: Some(1000), exe: Some(old.clone()), env_clean: true, maps_clean: true, alive: true };
    assert_eq!(judge(&peer, &f, &list), Ok(()));
    let still_linked = Facts { exe: Some(Exe { links: 1, ..old.clone() }), ..f.clone() };
    assert_eq!(judge(&peer, &still_linked, &list), Err(Refusal::NotAllowed));
    let elsewhere = Facts { exe: Some(Exe { dev: 8, ..old.clone() }), ..f.clone() };
    assert_eq!(judge(&peer, &elsewhere, &list), Err(Refusal::NotAllowed));
    let other_name = Facts { exe: Some(Exe { link: "/usr/bin/python3 (deleted)".into(), ..old.clone() }), ..f.clone() };
    assert_eq!(judge(&peer, &other_name, &list), Err(Refusal::NotAllowed));
}

#[test]
fn a_peer_that_is_gone_is_refused() {
    let mut child = std::process::Command::new("true").spawn().unwrap();
    // SAFETY: no pointers; the result is checked before it is owned.
    let raw = unsafe { libc::syscall(libc::SYS_pidfd_open, child.id() as libc::pid_t, 0) } as libc::c_int;
    assert!(raw >= 0);
    // SAFETY: a fresh pidfd nothing else owns.
    let pidfd = unsafe { OwnedFd::from_raw_fd(raw) };
    child.wait().unwrap();
    // SAFETY: no pointers.
    let (uid, gid) = unsafe { (libc::getuid(), libc::getgid()) };
    let f = gather(&Peer { pid: child.id() as i32, uid, gid }, Some(&pidfd), &seat(uid));
    assert!(!f.alive);
}

#[test]
fn a_peer_that_is_there_is_found_there_without_signalling_it() {
    let mut child = std::process::Command::new("sleep").arg("5").spawn().unwrap();
    // SAFETY: no pointers; the result is checked before it is owned.
    let raw = unsafe { libc::syscall(libc::SYS_pidfd_open, child.id() as libc::pid_t, 0) } as libc::c_int;
    assert!(raw >= 0);
    // SAFETY: a fresh pidfd nothing else owns.
    let pidfd = unsafe { OwnedFd::from_raw_fd(raw) };
    // SAFETY: no pointers.
    let (uid, gid) = unsafe { (libc::getuid(), libc::getgid()) };
    let f = gather(&Peer { pid: child.id() as i32, uid, gid }, Some(&pidfd), &seat(uid));
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(f.alive);
}

#[test]
fn only_the_sessions_own_start_up_variables_pass() {
    assert!(env_clean(b"HOME=/home/a\0WAYLAND_DISPLAY=wayland-0\0LANG=en_GB.UTF-8\0LC_TIME=en_GB.UTF-8\0"));

    assert!(env_clean(b"DBUS_SESSION_BUS_ADDRESS=x\0GNOME_DESKTOP_SESSION_ID=x\0GTK_USE_PORTAL=1\0HOME=x\0KDE_SESSION_VERSION=5\0LANG=x\0LOGNAME=x\0MAIL=x\0MOTD_SHOWN=pam\0PATH=x\0PWD=x\0QT_QPA_PLATFORMTHEME=x\0RUST_BACKTRACE=1\0SHELL=x\0TERM=x\0USER=x\0XDG_CONFIG_DIRS=x\0XDG_CURRENT_DESKTOP=x\0XDG_DATA_DIRS=x\0XDG_RUNTIME_DIR=x\0XDG_SEAT=x\0XDG_SESSION_DESKTOP=x\0XDG_SESSION_ID=x\0XDG_SESSION_TYPE=x\0XDG_VTNR=x\0XKB_DEFAULT_LAYOUT=gb\0"));
    assert_eq!(env_refused(b"HOME=x\0LD_PRELOAD=/tmp/x.so\0"), ["LD_PRELOAD"]);
    assert!(env_clean(b""));
    for name in ALLOWED_ENV {
        assert!(env_clean(format!("{name}=x\0").as_bytes()), "{name}");
    }
    for bad in [&b"LD_PRELOAD=/tmp/x.so"[..], b"LD_LIBRARY_PATH=/tmp", b"LD_AUDIT=x", b"GCONV_PATH=/tmp", b"VK_ICD_FILENAMES=/tmp/x.json",
                b"__EGL_VENDOR_LIBRARY_DIRS=/tmp", b"LIBGL_DRIVERS_PATH=/tmp", b"LIBVA_DRIVERS_PATH=/tmp", b"GBM_BACKENDS_PATH=/tmp",
                b"GIO_EXTRA_MODULES=/tmp", b"GTK_MODULES=x", b"QT_PLUGIN_PATH=/tmp", b"SOMETHING_ELSE=1"] {
        let mut env = b"HOME=/home/a\0".to_vec();
        env.extend_from_slice(bad);
        env.push(0);
        assert!(!env_clean(&env), "{}", String::from_utf8_lossy(bad));
    }
    assert!(!env_clean(b"OLD_LD_PRELOAD=x\0"), "a name not on the list is refused");
    assert!(!env_clean(b"XLC_TIME=x\0"), "a prefix counts only at the start");
}

#[test]
fn the_listener_asks_the_gate_and_refuses_extra_clients() {

    let dir = crate::common::files::private_temp("ctl").unwrap();
    let path = dir.join("c.sock");
    let listener = listen_at(&path);
    let (mut s, _fake, _t, _now) = service(0);
    let refuse = |_| Err(Refusal::NotAllowed);
    let c = connect_at(&path);
    s.turn(Some(&listener), &refuse, 1000).unwrap();
    assert!(s.conns.is_empty());
    let mut buf = [0u8; 8];
    let n = recv_frame(c.as_raw_fd(), &mut buf).unwrap().unwrap();
    assert_eq!(&buf[..n], answer(WHOLE, Err(Refusal::NotAllowed)).as_slice());
    let mut keep = Vec::new();
    for _ in 0..MOST_CLIENTS + 1 {
        keep.push(connect_at(&path));
    }
    while s.conns.len() < MOST_CLIENTS {
        s.turn(Some(&listener), &|_| Ok(()), 1000).unwrap();
    }
    s.turn(Some(&listener), &|_| Ok(()), 200).unwrap();
    assert_eq!(s.conns.len(), MOST_CLIENTS);
    let n = recv_frame(keep[MOST_CLIENTS].as_raw_fd(), &mut buf).unwrap().unwrap();
    assert_eq!(&buf[..n], answer(WHOLE, Err(Refusal::TooMany)).as_slice());
}

fn addr(path: &Path) -> (libc::sockaddr_un, libc::socklen_t) {
    // SAFETY: sockaddr_un is plain numbers; all zero is a valid start.
    let mut a: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    a.sun_family = libc::AF_UNIX as libc::sa_family_t;
    let p = path.as_os_str().as_encoded_bytes();
    for (d, s) in a.sun_path.iter_mut().zip(p) {
        *d = *s as libc::c_char;
    }
    (a, (std::mem::size_of::<libc::sa_family_t>() + p.len() + 1) as libc::socklen_t)
}

fn seqpacket() -> OwnedFd {
    // SAFETY: no pointers; checked before it is owned.
    let fd = check(unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) }).unwrap();
    // SAFETY: fresh and owned by nothing else.
    unsafe { OwnedFd::from_raw_fd(fd) }
}

fn listen_at(path: &Path) -> OwnedFd {
    let fd = seqpacket();
    let (a, len) = addr(path);
    // SAFETY: a is a whole sockaddr_un and len lies within it.
    check(unsafe { libc::bind(fd.as_raw_fd(), (&a as *const libc::sockaddr_un).cast(), len) }).unwrap();
    // SAFETY: no pointers.
    check(unsafe { libc::listen(fd.as_raw_fd(), 16) }).unwrap();
    fd
}

fn connect_at(path: &Path) -> OwnedFd {
    let fd = seqpacket();
    let (a, len) = addr(path);
    // SAFETY: a is a whole sockaddr_un and len lies within it.
    check(unsafe { libc::connect(fd.as_raw_fd(), (&a as *const libc::sockaddr_un).cast(), len) }).unwrap();
    fd
}

#[test]
fn a_frame_from_a_process_that_was_not_let_in_ends_the_connection() {
    let (mut s, _fake, theirs, _now) = service(1);
    let fd = theirs[0].as_raw_fd();
    // SAFETY: the child only sends one frame and ends, calling nothing else.
    let child = unsafe { libc::fork() };
    if child == 0 {
        // SAFETY: send and _exit are async-signal-safe; the frame is one byte.
        unsafe {
            libc::send(fd, b"x".as_ptr().cast(), 1, 0);
            libc::_exit(0);
        }
    }
    assert!(child > 0);
    let mut status = 0;
    // SAFETY: waits for the child just made.
    unsafe { libc::waitpid(child, &mut status, 0) };
    s.turn(None, &|_| Ok(()), 1000).unwrap();
    assert!(s.conns.is_empty(), "the child inherited the descriptor and was heard");
}

#[test]
fn a_frame_from_the_process_that_connected_is_heard() {
    let (mut s, _fake, theirs, _now) = service(1);
    send_frame(theirs[0].as_raw_fd(), &make(0, pad())).unwrap();
    s.turn(None, &|_| Ok(()), 1000).unwrap();
    assert_eq!(s.conns.len(), 1);
    let mut buf = [0u8; 64];
    assert!(recv_frame(theirs[0].as_raw_fd(), &mut buf).unwrap().is_some(), "no answer came");
}

fn maps_line(perms: &str, ino: u64, path: &str) -> String {
    format!("7f0000000000-7f0000001000 {perms} 00000000 fe:01 {ino:<26} {path}\n")
}

fn pretend_stat(path: &Path) -> Option<Stat> {
    let folder = |owner: u32, mode: u32| Some(Stat { owner, mode: 0o040000 | mode, ino: 1 });
    match path.to_str()? {
        "/" | "/usr" | "/usr/lib" | "/opt" => folder(0, 0o755),
        "/home" | "/dev/shm" | "/run/user" => folder(0, 0),
        "/dev" | "/run" => folder(0, 0o755),
        "/tmp" => folder(0, 0o1777),
        "/usr/shared" => folder(1000, 0o755),
        "/usr/open" => folder(0, 0o775),
        "/usr/lib/good.so" | "/usr/shared/lib.so" | "/usr/open/lib.so" => Some(Stat { owner: 0, mode: 0o100644, ino: 50 }),
        "/usr/lib/theirs.so" => Some(Stat { owner: 1000, mode: 0o100644, ino: 51 }),
        "/usr/lib/open.so" => Some(Stat { owner: 0, mode: 0o100666, ino: 52 }),
        _ => None,
    }
}

#[test]
fn only_code_files_root_keeps_in_system_places_pass_the_maps_check() {
    let fine = [
        maps_line("r-xp", 50, "/usr/lib/good.so"),
        maps_line("r-xp", 77, "/usr/lib/replaced.so (deleted)"),
        maps_line("r-xp", 77, "/usr/lib/gone/old.so (deleted)"),
        maps_line("rw-p", 9, "/home/john/not-code.bin"),
        maps_line("r-xp", 0, ""),
        maps_line("r-xp", 0, "[vdso]"),
    ];
    assert_eq!(maps_refused(&fine.concat(), pretend_stat), None);
    for (ino, path) in [
        (9, "/home/john/evil.so"),
        (9, "/home/john/evil.so (deleted)"),
        (9, "/tmp/evil.so"),
        (9, "/tmp/evil.so (deleted)"),
        (9, "/dev/shm/evil.so (deleted)"),
        (9, "/run/user/1000/evil.so (deleted)"),
        (9, "/memfd:evil (deleted)"),
        (9, "/SYSV00000000 (deleted)"),
        (50, "/usr/shared/lib.so"),
        (50, "/usr/open/lib.so"),
        (51, "/usr/lib/theirs.so"),
        (52, "/usr/lib/open.so"),
        (99, "/usr/lib/missing.so"),

        (99, "/usr/lib/good.so"),
    ] {
        let maps = maps_line("r-xp", 50, "/usr/lib/good.so") + &maps_line("r-xp", ino, path);
        assert_eq!(maps_refused(&maps, pretend_stat).as_deref(), Some(path), "{path}");
    }
}

#[test]
fn a_compositor_with_outside_code_is_refused_and_maps_too_long_to_read_refuse() {
    let peer = Peer { pid: 1, uid: 1000, gid: 1000 };
    let list = [Allowed { path: ALLOWED[0].into(), dev: 7, ino: 100 }];
    let exe = Exe { dev: 7, ino: 100, owner: 0, mode: 0o100755, links: 1, link: ALLOWED[0].into() };
    let f = Facts { at_seat: Some(1000), exe: Some(exe), env_clean: true, maps_clean: true, alive: true };
    assert_eq!(judge(&peer, &f, &list), Ok(()));
    assert_eq!(judge(&peer, &Facts { maps_clean: false, ..f }, &list), Err(Refusal::NotAllowed));
    let dir = std::env::temp_dir().join(format!("trindle-maps-{}", std::process::id()));
    std::fs::write(&dir, [b'x'; 11]).unwrap();
    let path = dir.to_str().unwrap();
    assert_eq!(read_most(path, 11).map(|v| v.len()), Some(11));
    assert_eq!(read_most(path, 10), None);
    std::fs::remove_file(&dir).unwrap();
}

#[test]
fn this_machines_real_system_libraries_pass_the_maps_check() {
    let maps = std::fs::read_to_string("/proc/self/maps").unwrap();
    let system: String = maps.lines().filter(|l| !l.contains("/home/")).fold(String::new(), |mut s, l| { let _ = writeln!(s, "{l}"); s });
    assert!(system.contains("/usr/lib"), "{system}");
    assert_eq!(maps_refused(&system, stat_of), None);
}
