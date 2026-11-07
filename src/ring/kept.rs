//! The ring Trindle's compositor started and keeps (compose/own_ring/), as against one started by name.

use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::common::own_press::Seen;
use crate::common::ring_pulse::{write, Wrote};
use crate::toolkit::bus::{Bus, Listen};
use crate::toolkit::event::{Flow, Loop};

const MARK: &str = "TRINDLE_RING_KEPT";

static KEPT: AtomicBool = AtomicBool::new(false);

thread_local! {
    static PULSE: RefCell<Option<std::fs::File>> = const { RefCell::new(None) };
    static HAND: RefCell<Hand> = const { RefCell::new(Hand::Unsaid) };
    static BEATING: Cell<bool> = const { Cell::new(false) };
    static HOLDERS: RefCell<Vec<Box<dyn Fn() -> bool>>> = const { RefCell::new(Vec::new()) };
    static REFUSED: Cell<bool> = const { Cell::new(false) };
}

enum Hand {
    Unsaid,
    Silent,
    Says(std::fs::File, Seen),
}

fn hand_from(named: Option<&str>, handed: bool) -> Hand {
    match named {
        None => Hand::Unsaid,
        Some(named) => match handed.then_some(named).and_then(pulse_numbered) {
            Some(pipe) => Hand::Says(pipe, Seen::default()),
            None => Hand::Silent,
        },
    }
}

pub fn take() {
    let named = std::env::var(MARK).ok();
    std::env::remove_var(MARK);
    let hand = std::env::var(crate::common::own_press::NAME).ok();
    std::env::remove_var(crate::common::own_press::NAME);
    let handed = std::env::var_os("WAYLAND_SOCKET").is_some();
    HAND.with(|h| *h.borrow_mut() = hand_from(hand.as_deref(), handed));
    let Some(pulse) = named.as_deref().and_then(pulse_numbered).filter(|_| handed) else { return };
    KEPT.store(true, Ordering::Relaxed);
    PULSE.with(|p| *p.borrow_mut() = Some(pulse));
}

pub fn hear_on(lp: &Loop) {
    use std::os::fd::AsRawFd;
    let fd = HAND.with(|h| match &*h.borrow() {
        Hand::Says(pipe, _) => Some(pipe.as_raw_fd()),
        _ => None,
    });
    let Some(fd) = fd else { return };
    lp.watch(fd, move |_| {
        hear_presses();
        Flow::Continue
    });
}

pub fn own_press() -> bool {
    crate::toolkit::window::asked_by_reader() || own(crate::toolkit::window::press_now())
}

pub fn clicked() -> bool {
    !crate::toolkit::window::key_now()
}

fn own(press: Option<u32>) -> bool {
    HAND.with(|h| match &mut *h.borrow_mut() {
        Hand::Unsaid => true,
        Hand::Silent => false,
        Hand::Says(pipe, seen) => {
            seen.read(pipe);
            press.is_some_and(|serial| seen.has(serial))
        }
    })
}

fn hear_presses() {
    HAND.with(|h| {
        if let Hand::Says(pipe, seen) = &mut *h.borrow_mut() {
            seen.read(pipe);
        }
    });
}

pub fn kept() -> bool {
    KEPT.load(Ordering::Relaxed)
}

fn pulse_numbered(named: &str) -> Option<std::fs::File> {
    use std::os::fd::FromRawFd;
    let fd: i32 = named.parse().ok()?;
    if fd < 3 {
        return None;
    }
    // SAFETY: fstat only writes into the stat it is handed, and fcntl only
    // sets flags; a number that is no open file fails them all.
    let pipe = unsafe {
        let mut st: libc::stat = std::mem::zeroed();
        libc::fstat(fd, &mut st) == 0 && (st.st_mode & libc::S_IFMT) == libc::S_IFIFO
            && libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == 0
            && libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) == 0
    };
    // SAFETY: the number is an open pipe handed to this program alone, and
    // nothing else here holds it.
    pipe.then(|| unsafe { std::fs::File::from_raw_fd(fd) })
}

pub fn pulse() -> bool {
    hear_presses();
    PULSE.with(|p| {
        let mut held = p.borrow_mut();
        let Some(pipe) = held.as_mut() else { return false };
        let wrote = write(pipe, crate::common::ring_pulse::now());
        if let Some(words) = said(wrote, REFUSED.replace(wrote == Wrote::Full)) {
            eprintln!("trindle-ring: {words}");
        }
        if wrote == Wrote::Gone {
            *held = None;
        }
        wrote != Wrote::Gone
    })
}

fn said(wrote: Wrote, refused_before: bool) -> Option<&'static str> {
    match (wrote, refused_before) {
        (Wrote::Full, false) => Some("the pipe the compositor reads the ring's pulse from is full, so it has not been read for hours: each pulse is tried again a second later"),
        (Wrote::Sent, true) => Some("the ring's pulse is being read again"),
        (Wrote::Gone, _) => Some("nothing reads the ring's pulse any more: no more is written"),
        _ => None,
    }
}

pub fn pulse_while_keys_held(lp: &Loop, display: &std::rc::Rc<crate::toolkit::window::Display>) {
    if !kept() {
        return;
    }
    let (lp, held) = (lp.clone(), std::rc::Rc::downgrade(display));
    display.on_keys_held(move || {
        let held = held.clone();
        beat_while(&lp, move || held.upgrade().is_some_and(|d| d.holds_keys()));
    });
}

fn beat_while(lp: &Loop, holds: impl Fn() -> bool + 'static) {
    HOLDERS.with(|h| h.borrow_mut().push(Box::new(holds)));
    pulse();
    if BEATING.replace(true) {
        return;
    }
    lp.every(std::time::Duration::from_millis(crate::common::ring_pulse::PULSE_EVERY), move || {
        let any = HOLDERS.with(|h| {
            let mut h = h.borrow_mut();
            h.retain(|holds| holds());
            !h.is_empty()
        });
        if !any || !pulse() {
            BEATING.set(false);
            HOLDERS.with(|h| h.borrow_mut().clear());
            return Flow::Stop;
        }
        Flow::Continue
    });
}

pub fn hold_name(bus: &Bus, lp: &Loop, name: &str, kept: bool, done: impl FnOnce(bool) + 'static) {
    if kept {
        bus.take_firmly(name, lp, done);
    } else {
        bus.own(name, false, lp, done);
    }
}

pub fn when_lost(bus: &Bus, name: &'static str, lost: impl Fn() + 'static) -> Option<Listen> {
    const DBUS: &str = "org.freedesktop.DBus";
    bus.listen(Some(DBUS), DBUS, Some("NameLost"), None, move |m| {
        if m.body().deserialize::<String>().is_ok_and(|n| n == name) {
            lost();
        }
    }).ok()
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;
    use crate::toolkit::bus::testing::{own_daemon, until};

    const NAME: &str = super::super::BUS_NAME;

    struct Ring {
        _bus: Bus,
        got: Rc<Cell<Option<bool>>>,
        lost: Rc<Cell<bool>>,
        _hearing: Option<Listen>,
    }

    fn ring(address: &str, lp: &Loop, kept: bool) -> Ring {
        let bus = Bus::at(lp, address).expect("the test's own bus");
        let (got, lost) = (Rc::new(Cell::new(None)), Rc::new(Cell::new(false)));
        let l = lost.clone();
        let hearing = when_lost(&bus, NAME, move || l.set(true));
        assert!(hearing.is_some());
        let g = got.clone();
        hold_name(&bus, lp, NAME, kept, move |ok| g.set(Some(ok)));
        let g = got.clone();
        assert!(until(lp, move || g.get().is_some()), "the bus never answered");
        Ring { _bus: bus, got, lost, _hearing: hearing }
    }

    fn own_pulse() -> std::fs::File {
        use std::os::fd::FromRawFd;
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC | libc::O_NONBLOCK) }, 0);
        PULSE.with(|p| *p.borrow_mut() = pulse_numbered(&ends[1].to_string()));
        // SAFETY: the number was just made by pipe2 and belongs to nothing else.
        unsafe { std::fs::File::from_raw_fd(ends[0]) }
    }

    fn written(pipe: &mut std::fs::File) -> usize {
        use std::io::Read;
        let (mut bytes, mut count) = ([0u8; 4096], 0);
        while let Ok(n) = pipe.read(&mut bytes) {
            if n == 0 {
                break;
            }
            count += n / 8;
        }
        count
    }

    fn turn(lp: &Loop, long: u64) {
        let quit = lp.clone();
        lp.after(std::time::Duration::from_millis(long), move || quit.quit());
        lp.run();
    }

    #[test]
    fn the_beat_goes_on_for_as_long_as_anything_holds_the_keyboard_and_no_longer() {
        use crate::common::ring_pulse::PULSE_EVERY;
        let mut pipe = own_pulse();
        let lp = Loop::new();
        let holds = Rc::new(Cell::new(true));
        let h = holds.clone();
        beat_while(&lp, move || h.get());
        assert_eq!(written(&mut pipe), 1, "nothing was written at once as the panel came out");
        turn(&lp, PULSE_EVERY * 3 + PULSE_EVERY / 2);
        assert_eq!(written(&mut pipe), 3, "the beat did not go on while the keyboard was held");
        let h = holds.clone();
        beat_while(&lp, move || h.get());
        assert_eq!(written(&mut pipe), 1);
        turn(&lp, PULSE_EVERY);
        assert_eq!(written(&mut pipe), 1, "a second panel doubled the beat");
        let (first, second) = (Rc::new(Cell::new(true)), Rc::new(Cell::new(true)));
        let (f, g) = (first.clone(), second.clone());
        HOLDERS.with(|h| h.borrow_mut().clear());
        beat_while(&lp, move || f.get());
        beat_while(&lp, move || g.get());
        written(&mut pipe);
        first.set(false);
        holds.set(false);
        turn(&lp, PULSE_EVERY * 2 + PULSE_EVERY / 2);
        assert!(written(&mut pipe) >= 2, "the beat stopped for the one still holding the keyboard when the first went quiet");
        second.set(false);
        turn(&lp, PULSE_EVERY * 2 + PULSE_EVERY / 2);
        assert_eq!(written(&mut pipe), 0, "the beat went on with nothing holding the keyboard");
        assert!(!BEATING.get());
        holds.set(true);
        let h = holds.clone();
        beat_while(&lp, move || h.get());
        turn(&lp, PULSE_EVERY + PULSE_EVERY / 2);
        assert_eq!(written(&mut pipe), 2, "the beat did not begin again for the next panel");
        holds.set(false);
        PULSE.with(|p| *p.borrow_mut() = None);
    }

    #[test]
    fn the_kept_ring_takes_the_name_from_a_ring_started_by_name_which_is_told_to_leave() {
        let daemon = own_daemon();
        let lp = Loop::new();
        let by_name = ring(&daemon.1, &lp, false);
        assert_eq!(by_name.got.get(), Some(true));
        let second = ring(&daemon.1, &lp, false);
        assert_eq!(second.got.get(), Some(false));
        assert!(!by_name.lost.get());
        let kept = ring(&daemon.1, &lp, true);
        assert_eq!(kept.got.get(), Some(true), "the kept ring stepped aside for a ring started by name");
        let l = by_name.lost.clone();
        assert!(until(&lp, move || l.get()), "the ring started by name was never told to leave");
        let later = ring(&daemon.1, &lp, false);
        assert_eq!(later.got.get(), Some(false));
        let pretender = Bus::at(&lp, &daemon.1).unwrap();
        let took = Rc::new(Cell::new(None));
        let t = took.clone();
        pretender.own(NAME, true, &lp, move |ok| t.set(Some(ok)));
        let t = took.clone();
        assert!(until(&lp, move || t.get().is_some()));
        assert_eq!(took.get(), Some(false));
        assert!(!kept.lost.get(), "the kept ring was told it lost its name");
    }

    #[test]
    fn only_a_pipe_is_a_pulse_and_a_ring_with_none_is_not_kept() {
        for bad in ["", "pulse", "-1", "0", "2", "1048575"] {
            assert!(pulse_numbered(bad).is_none(), "{bad:?}");
        }
        let mut ends = [0i32; 2];
        // SAFETY: pipe writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
        use std::io::Write;
        let mut pulse = pulse_numbered(&ends[1].to_string()).expect("a pipe is a pulse");
        pulse.write_all(&7u64.to_le_bytes()).unwrap();
        // SAFETY: the read end was just made by pipe and belongs to nothing else.
        let mut read = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[0]) };
        let mut got = [0u8; 8];
        std::io::Read::read_exact(&mut read, &mut got).unwrap();
        assert_eq!(u64::from_le_bytes(got), 7);
        assert!(!kept());
    }

    #[test]
    fn a_ring_told_of_a_pipe_and_handed_none_takes_no_press_as_the_persons() {
        use crate::toolkit::window::with_press;
        let file = crate::common::files::open_file(std::path::Path::new("/proc/self/stat")).unwrap();
        let not_a_pipe = std::os::fd::AsRawFd::as_raw_fd(&file).to_string();
        for named in [crate::common::own_press::NONE, "", "hand", "-1", "2", "1048575", not_a_pipe.as_str()] {
            for handed in [true, false] {
                assert!(matches!(hand_from(Some(named), handed), Hand::Silent), "{named:?}, handed {handed}");
            }
        }
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) }, 0);
        // SAFETY: both numbers were just made by pipe2 and belong to nothing else.
        let (read, _wrote) = unsafe { (<std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[0]), <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[1])) };
        assert!(matches!(hand_from(Some(&ends[0].to_string()), false), Hand::Silent), "a ring by name believed a pipe it was named");
        drop(read);
        assert!(matches!(hand_from(None, true), Hand::Unsaid) && matches!(hand_from(None, false), Hand::Unsaid));
        HAND.with(|h| *h.borrow_mut() = Hand::Silent);
        assert!(!with_press(Some(3), own_press), "a ring that cannot tell took a press as the person's");
        assert!(!with_press(None, own_press));
        assert!(!own(Some(0)) && !own(None));
        HAND.with(|h| *h.borrow_mut() = Hand::Unsaid);
        assert!(own(Some(3)) && own(None), "with no compositor saying, a press was refused");
        let take = crate::common::source::code(include_str!("kept.rs"));
        let take = take.split("pub fn take() {").nth(1).unwrap().split("\n}\n").next().unwrap();
        let (hand, pulse) = (take.find("HAND.with(").expect("take reads the hand"), take.find("else { return };").expect("take leaves with no pulse"));
        assert!(hand < pulse, "a ring with no pulse never read what it was told of its pipe");
    }

    #[test]
    fn the_persons_presses_are_read_as_they_are_written_so_the_pipe_never_fills() {
        use crate::toolkit::window::with_press;
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC | libc::O_NONBLOCK) }, 0);
        // SAFETY: the write end was just made by pipe2 and belongs to nothing else.
        let mut wrote = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[1]) };
        HAND.with(|h| *h.borrow_mut() = hand_from(Some(&ends[0].to_string()), true));
        assert!(HAND.with(|h| matches!(&*h.borrow(), Hand::Says(..))));
        let lp = Loop::new();
        hear_on(&lp);
        for serial in 0..40_000u32 {
            assert!(crate::common::own_press::write(&mut wrote, serial));
            if serial % 500 == 0 {
                let done = Rc::new(Cell::new(false));
                let d = done.clone();
                lp.after(std::time::Duration::from_millis(1), move || d.set(true));
                assert!(until(&lp, move || done.get()));
            }
        }
        assert!(crate::common::own_press::write(&mut wrote, 77_777));
        assert!(with_press(Some(77_777), own_press), "the number of a press was refused by a pipe nobody had read");
        HAND.with(|h| *h.borrow_mut() = Hand::Unsaid);
    }

    #[test]
    fn only_a_press_the_compositor_named_is_the_persons_own() {
        use crate::toolkit::window::with_press;
        assert!(own(Some(3)) && own(None), "with no compositor saying, a press was refused");
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) }, 0);
        // SAFETY: the write end was just made by pipe and belongs to nothing else.
        let mut wrote = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[1]) };
        HAND.with(|h| *h.borrow_mut() = hand_from(Some(&ends[0].to_string()), true));
        assert!(HAND.with(|h| matches!(&*h.borrow(), Hand::Says(..))));
        assert!(!with_press(Some(41), own_press), "a press the compositor never named counted as the person's");
        assert!(!with_press(None, own_press), "something no press brought about counted as the person's");
        assert!(!crate::toolkit::window::by_reader(own_press), "a press asked over the accessibility bus counted as the person's, with the screen reader not switched on by them");
        let dir = std::env::temp_dir().join(format!("trindle-ring-reader-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        crate::common::own_helpers::kept_at(&dir.join("own-helpers"), || {
            use crate::common::own_helpers::Helper;
            use crate::common::own_helpers_set::switch;
            switch(Helper::Reader, true, false).unwrap();
            assert!(!crate::toolkit::window::by_reader(own_press), "a screen reader switched on by a made-up press counted");
            switch(Helper::Reader, true, true).unwrap();
            assert!(crate::toolkit::window::by_reader(own_press), "the press of a screen reader the person switched on was refused by the ring");
        });
        let _ = std::fs::remove_dir_all(&dir);
        assert!(!own_press());
        assert!(crate::common::own_press::write(&mut wrote, 42));
        assert!(with_press(Some(42), own_press), "the person's own press was refused");
        assert!(!with_press(Some(41), own_press) && !with_press(Some(43), own_press));
        assert!(crate::common::own_press::write(&mut wrote, 44));
        hear_presses();
        drop(wrote);
        assert!(with_press(Some(44), own_press));
        HAND.with(|h| *h.borrow_mut() = Hand::Unsaid);
    }

    #[test]
    fn a_refused_pulse_is_said_once_and_a_pipe_nobody_reads_ends_the_pulsing() {
        assert_eq!(said(Wrote::Sent, false), None);
        assert!(said(Wrote::Full, false).is_some());
        assert_eq!(said(Wrote::Full, true), None);
        assert!(said(Wrote::Sent, true).is_some());
        assert!(said(Wrote::Gone, false).is_some() && said(Wrote::Gone, true).is_some());
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) }, 0);
        // SAFETY: the read end was just made by pipe and belongs to nothing else.
        let mut read = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[0]) };
        PULSE.with(|p| *p.borrow_mut() = pulse_numbered(&ends[1].to_string()));
        assert!(pulse(), "a pulse on a pipe that is read was not written");
        let since = std::time::Instant::now();
        for _ in 0..3 * 3600 {
            assert!(pulse(), "a full pipe ended the pulsing");
        }
        assert!(since.elapsed() < std::time::Duration::from_secs(5), "the ring waited on a full pipe");
        assert!(REFUSED.get(), "the pipe never filled");
        let mut all = Vec::new();
        // SAFETY: fcntl only sets a flag on a file this owns.
        assert_eq!(unsafe { libc::fcntl(std::os::fd::AsRawFd::as_raw_fd(&read), libc::F_SETFL, libc::O_NONBLOCK) }, 0);
        let _ = std::io::Read::read_to_end(&mut read, &mut all);
        assert!(pulse());
        assert!(!REFUSED.get());
        drop(read);
        let since = std::time::Instant::now();
        let mut ended = !pulse();
        while !ended && since.elapsed() < std::time::Duration::from_secs(5) {
            std::thread::sleep(std::time::Duration::from_millis(20));
            ended = !pulse();
        }
        assert!(ended, "a pipe nobody reads did not end the pulsing");
        assert!(PULSE.with(|p| p.borrow().is_none()));
        assert!(!pulse());
    }
}
