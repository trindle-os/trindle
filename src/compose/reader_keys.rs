//! The screen reader's keys.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use smithay::input::keyboard::{xkb, Keysym, ModifiersState};

pub const FOLDER: &str = "trindle-screen-reader";
pub const SOCKET: &str = "keys.sock";
const LOOK_EVERY: Duration = Duration::from_secs(1);
const MOST_PER_LOOK: usize = 4096;
const LEARN_ANSWER: Duration = Duration::from_secs(3);

pub fn is_reader_key(sym: Keysym) -> bool {
    matches!(sym, Keysym::Caps_Lock | Keysym::Insert)
}

fn socket() -> Option<PathBuf> {
    std::env::var_os("XDG_RUNTIME_DIR").map(|d| PathBuf::from(d).join(FOLDER).join(SOCKET))
}

pub struct Press<'a> {
    pub code: u32,
    pub name: &'a str,
    pub reader_key: bool,
    pub down: bool,
    pub mods: ModifiersState,
    pub real: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fate {
    Taken(Vec<String>),
    Pass,
}

#[derive(Debug, Default)]
pub struct Held {
    reader_down: Vec<u32>,
    taken: Vec<u32>,
    alone: bool,
}

fn line(p: &Press, reader: bool) -> String {
    let m = p.mods;
    let words: Vec<&str> = [(m.shift, "shift"), (m.ctrl, "ctrl"), (m.alt, "alt"), (m.logo, "super"), (reader, "reader")]
        .iter().filter(|(on, _)| *on).map(|(_, w)| *w).collect();
    let mods = if words.is_empty() { "-".to_string() } else { words.join(",") };
    format!("key\t{}\t{}\t{}\t{}\n", p.name, if p.down { "down" } else { "up" }, mods, if p.real { "real" } else { "made" })
}

impl Held {
    pub fn fate(&mut self, p: &Press, linked: bool, learn: bool) -> Fate {
        if p.down {
            if p.name == "Insert" && (p.mods.shift || p.mods.ctrl) && self.reader_down.is_empty() {
                return Fate::Pass;
            }
            if p.name == "Escape" && self.reader_down.is_empty() {
                if !(linked && learn && p.real) {
                    return Fate::Pass;
                }
                if !self.taken.contains(&p.code) {
                    self.taken.push(p.code);
                }
                return Fate::Taken(vec![line(p, false)]);
            }
            if p.reader_key && self.reader_down.is_empty() && !(linked && p.real) {
                return Fate::Pass;
            }
            if !p.reader_key && self.reader_down.is_empty() {
                return Fate::Pass;
            }
            if p.reader_key {
                self.reader_down.push(p.code);
                self.alone = self.taken.is_empty();
            } else {
                self.alone = false;
            }
            if !self.taken.contains(&p.code) {
                self.taken.push(p.code);
            }
            return Fate::Taken(vec![line(p, true)]);
        }
        if !self.taken.contains(&p.code) {
            return Fate::Pass;
        }
        self.taken.retain(|c| *c != p.code);
        let was_reader = self.reader_down.contains(&p.code);
        self.reader_down.retain(|c| *c != p.code);
        let mut out = vec![line(p, !self.reader_down.is_empty())];
        if was_reader && self.alone && self.taken.is_empty() {
            out.push(format!("tap\t{}\t{}\n", p.name, if p.real { "real" } else { "made" }));
        }
        if was_reader {
            self.alone = false;
        }
        Fate::Taken(out)
    }

    fn idle(&self) -> bool {
        self.reader_down.is_empty() && self.taken.is_empty()
    }

    fn undo(&mut self, code: u32) {
        self.reader_down.retain(|c| *c != code);
        self.taken.retain(|c| *c != code);
    }
}

#[derive(Debug, Default)]
pub struct ReaderKeys {
    link: Option<UnixStream>,
    looked: Option<Instant>,
    held: Held,
    learn: bool,
    browse: bool,
    browsed: Vec<u32>,
    heard: Vec<u8>,
    watched: bool,
    escape_sent: Option<Instant>,
    switch_checked: Option<Instant>,
    caps_told: Option<bool>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ask {
    CapsToggle,
    Pass(String, Vec<&'static str>),
}

const LONGEST_LINE: usize = 256;

impl ReaderKeys {
    fn link(&mut self) -> bool {
        if !crate::own_helpers::on(crate::own_helpers::Helper::Reader) {
            self.drop_link();
            return false;
        }
        if self.link.is_some() {
            return true;
        }
        if self.looked.is_some_and(|t| t.elapsed() < LOOK_EVERY) {
            return false;
        }
        self.looked = Some(Instant::now());
        self.switch_checked = Some(Instant::now());
        self.link = socket().and_then(own_socket).and_then(|path| connect_without_waiting(&path))
            .filter(|s| peer_is_us(s) && peer_is_reader(s));
        if self.link.is_some() && !self.send(&["state\tplainkeys\tavailable\n".to_string()]) {
            return false;
        }
        self.link.is_some()
    }

    fn drop_link(&mut self) {
        self.link = None;
        self.learn = false;
        self.browse = false;
        self.browsed.clear();
        self.escape_sent = None;
        self.heard.clear();
    }

    fn still_on(&mut self) -> bool {
        if self.link.is_none() || self.switch_checked.is_some_and(|t| t.elapsed() < LOOK_EVERY) {
            return true;
        }
        self.switch_checked = Some(Instant::now());
        if crate::own_helpers::on(crate::own_helpers::Helper::Reader) {
            return true;
        }
        self.drop_link();
        false
    }

    fn lapse(&mut self) {
        if self.escape_sent.is_some_and(|t| t.elapsed() >= LEARN_ANSWER) {
            self.learn = false;
            self.escape_sent = None;
        }
    }

    pub fn start_watching(&mut self) -> bool {
        let start = self.link.is_some() && !self.watched;
        self.watched |= start;
        start
    }

    pub fn keep_watching(&mut self) -> bool {
        self.watched = self.link.is_some();
        self.watched
    }

    pub fn wants(&self, reader_key: bool, escape: bool) -> bool {
        reader_key || (escape && self.learn) || self.browse || !self.held.idle()
    }

    pub fn release(&mut self) {
        if !self.held.idle() {
            self.held = Held::default();
            self.drop_link();
        }
    }

    pub fn hear(&mut self) -> Vec<Ask> {
        let mut asks = vec![];
        if self.link.is_none() || !self.still_on() {
            return asks;
        }
        self.lapse();
        let mut buf = [0u8; 512];
        let mut read = 0;
        while read < MOST_PER_LOOK {
            let Some(link) = self.link.as_mut() else { break };
            match link.read(&mut buf) {
                Ok(0) => self.drop_link(),
                Ok(n) => {
                    read += n;
                    self.heard.extend_from_slice(&buf[..n]);
                    self.take_lines(&mut asks);
                    if self.heard.len() > LONGEST_LINE {
                        self.drop_link();
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => self.drop_link(),
            }
        }
        asks
    }

    fn take_lines(&mut self, asks: &mut Vec<Ask>) {
        while let Some(end) = self.heard.iter().position(|b| *b == b'\n') {
            let said: Vec<u8> = self.heard.drain(..=end).collect();
            match String::from_utf8_lossy(&said).trim_end_matches(['\n', '\r']).split('\t').collect::<Vec<_>>().as_slice() {
                ["caps", "toggle"] => asks.push(Ask::CapsToggle),
                ["pass", name, mods] if self.link.is_some() => {
                    if let Some(mods) = pass_mods(mods) {
                        asks.push(Ask::Pass((*name).to_string(), mods));
                    }
                }
                ["state", "learn", on @ ("on" | "off")] => {
                    self.learn = *on == "on";
                    self.escape_sent = None;
                }
                ["state", "browse", on @ ("on" | "off")] => {
                    self.browse = *on == "on";
                    if !self.browse {
                        self.browsed.clear();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn tell_caps(&mut self, on: bool) {
        self.caps_told = Some(on);
        if self.link.is_some() {
            self.send(&[format!("state\tcaps\t{}\n", if on { "on" } else { "off" })]);
        }
    }

    pub fn caps_changed(&mut self, on: bool) {
        if self.caps_told != Some(on) {
            self.tell_caps(on);
        }
    }

    fn send(&mut self, lines: &[String]) -> bool {
        let Some(link) = self.link.as_mut() else { return false };
        for l in lines {
            match link.write(l.as_bytes()) {
                Ok(n) if n == l.len() => {}
                _ => {
                    self.drop_link();
                    return false;
                }
            }
        }
        true
    }

    fn browsing(&mut self, p: &Press) -> Option<bool> {
        if p.down && (p.mods.ctrl || p.mods.alt || p.mods.logo) {
            return None;
        }
        if !p.down && !self.browsed.contains(&p.code) {
            return None;
        }
        if p.down && !self.browsed.contains(&p.code) {
            self.browsed.push(p.code);
        }
        if !p.down {
            self.browsed.retain(|c| *c != p.code);
        }
        Some(self.send(&[line(p, false)]))
    }

    pub fn take(&mut self, p: &Press) -> bool {
        let starting = p.down && p.reader_key && self.held.reader_down.is_empty();
        let linked = if starting { self.link() } else { self.still_on() && self.link.is_some() };
        self.lapse();
        if linked && self.browse && !p.reader_key && self.held.reader_down.is_empty() {
            if let Some(taken) = self.browsing(p) {
                return taken;
            }
        }
        let learn = self.learn;
        match self.held.fate(p, linked, learn) {
            Fate::Pass => false,
            Fate::Taken(lines) => {
                if p.down && p.name == "Escape" && learn {
                    self.escape_sent.get_or_insert_with(Instant::now);
                }
                if !self.send(&lines) && starting {
                    self.looked = None;
                    if !(self.link() && self.send(&lines)) {
                        self.held.undo(p.code);
                        return false;
                    }
                }
                true
            }
        }
    }
}

fn own_socket(path: PathBuf) -> Option<PathBuf> {
    use std::os::unix::fs::{FileTypeExt, MetadataExt};
    let uid = my_uid();
    let folder = std::fs::symlink_metadata(path.parent()?).ok()?;
    let sock = std::fs::symlink_metadata(&path).ok()?;
    (folder.is_dir() && folder.uid() == uid && folder.mode() & 0o022 == 0 && sock.file_type().is_socket() && sock.uid() == uid)
        .then_some(path)
}

fn peer_is_us(s: &UnixStream) -> bool {
    use std::os::fd::AsRawFd;
    let mut cred = libc::ucred { pid: 0, uid: u32::MAX, gid: 0 };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: `cred` and `len` are live and sized for SO_PEERCRED; the fd is open for the call.
    let got = unsafe { libc::getsockopt(s.as_raw_fd(), libc::SOL_SOCKET, libc::SO_PEERCRED, (&mut cred as *mut libc::ucred).cast(), &mut len) };
    got == 0 && cred.uid == my_uid()
}

#[cfg(not(test))]
const READER_EXE: &str = "trindle/plugins/org.trindle.ScreenReader/bin/trindle-screen-reader";
#[cfg(not(test))]
const READER_SYSTEM_EXE: &str = "/usr/lib/trindle/plugins/org.trindle.ScreenReader/bin/trindle-screen-reader";

fn reader_exe() -> Option<PathBuf> {
    #[cfg(test)]
    if let Some(p) = TEST_EXE.with(|t| t.borrow().clone()) {
        return Some(p);
    }
    #[cfg(test)]
    return std::env::current_exe().ok();
    #[cfg(not(test))]
    std::fs::canonicalize(READER_SYSTEM_EXE).or_else(|_| std::fs::canonicalize(crate::files::data_home().join(READER_EXE))).ok()
}

#[cfg(test)]
thread_local! {
    static TEST_EXE: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

fn exe_is_reader(exe: &Path, expected: &Path) -> bool {
    exe == expected
}

const SO_PEERPIDFD: libc::c_int = 77;

fn peer_is_reader(s: &UnixStream) -> bool {
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    let mut cred = libc::ucred { pid: 0, uid: u32::MAX, gid: 0 };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: `cred` and `len` are live and sized for SO_PEERCRED; the fd is open for the call.
    let got = unsafe { libc::getsockopt(s.as_raw_fd(), libc::SOL_SOCKET, libc::SO_PEERCRED, (&mut cred as *mut libc::ucred).cast(), &mut len) };
    if got != 0 || cred.pid <= 0 {
        return false;
    }
    let mut pidfd: libc::c_int = -1;
    let mut len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: `pidfd` and `len` are live and sized for an int.
    let got = unsafe { libc::getsockopt(s.as_raw_fd(), libc::SOL_SOCKET, SO_PEERPIDFD, (&mut pidfd as *mut libc::c_int).cast(), &mut len) };
    if got != 0 || pidfd < 0 {
        return false;
    }
    // SAFETY: a pidfd the kernel just gave us, owned by nothing else.
    let pidfd = unsafe { OwnedFd::from_raw_fd(pidfd) };
    let Some(expected) = reader_exe() else {
        return false;
    };
    reader_is(cred.pid, &pidfd, &expected)
}

fn reader_is(pid: i32, pidfd: &std::os::fd::OwnedFd, expected: &Path) -> bool {
    use std::os::fd::AsRawFd;
    let Some(exe) = exe_of(pid) else {
        return false;
    };
    // SAFETY: no pointers; signal 0 only asks whether the process is there.
    let alive = unsafe { libc::syscall(libc::SYS_pidfd_send_signal, pidfd.as_raw_fd(), 0, std::ptr::null::<libc::siginfo_t>(), 0) } == 0;
    alive && exe_is_reader(&exe, expected)
}

fn exe_of(pid: i32) -> Option<PathBuf> {
    std::fs::read_link(format!("/proc/{pid}/exe")).ok()
}

fn my_uid() -> u32 {
    // SAFETY: getuid takes nothing and cannot fail.
    unsafe { libc::getuid() }
}

fn connect_without_waiting(path: &Path) -> Option<UnixStream> {
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    // SAFETY: sockaddr_un is plain integers and bytes, valid all zero.
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    let bytes = path.as_os_str().as_bytes();
    if bytes.len() >= addr.sun_path.len() {
        return None;
    }
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
    for (to, from) in addr.sun_path.iter_mut().zip(bytes) {
        *to = *from as libc::c_char;
    }
    // SAFETY: a plain system call; the result is checked.
    let fd = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_STREAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return None;
    }
    // SAFETY: `fd` is a new socket nothing else owns; the stream closes it, on the early return too.
    let stream = unsafe { UnixStream::from_raw_fd(fd) };
    // SAFETY: `addr` is fully set and its size is passed.
    let done = unsafe { libc::connect(fd, (&addr as *const libc::sockaddr_un).cast(), std::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t) };
    (done == 0).then_some(stream)
}

pub(crate) fn may_be_passed_a_key(locked: bool, on_the_lock_surface: bool, on: Option<crate::trusted::Kind>) -> bool {
    !locked && !on_the_lock_surface && on != Some(crate::trusted::Kind::Question)
}

fn pass_mods(mods: &str) -> Option<Vec<&'static str>> {
    if mods == "-" {
        return Some(vec![]);
    }
    mods.split(',').map(|m| match m {
        "shift" => Some("Shift_L"),
        "ctrl" => Some("Control_L"),
        "alt" => Some("Alt_L"),
        _ => None,
    }).collect()
}

pub fn code_of(keymap: &xkb::Keymap, layout: u32, name: &str) -> Option<xkb::Keycode> {
    let sym = xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS);
    if sym.raw() == 0 {
        return None;
    }
    (keymap.min_keycode().raw()..=keymap.max_keycode().raw())
        .map(xkb::Keycode::new)
        .find(|c| keymap.key_get_syms_by_level(*c, layout, 0).contains(&sym))
}

pub fn name(keymap: &xkb::Keymap, layout: u32, code: xkb::Keycode) -> Option<String> {
    let first = *keymap.key_get_syms_by_level(code, layout, 0).first()?;
    let pad = keymap.key_get_syms_by_level(code, layout, 1).first().copied()
        .filter(|s| (Keysym::KP_0.raw()..=Keysym::KP_9.raw()).contains(&s.raw()) || *s == Keysym::KP_Decimal);
    Some(xkb::keysym_get_name(pad.unwrap_or(first)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(code: u32, name: &str, down: bool) -> Press<'_> {
        Press { code, name, reader_key: is_reader_key(xkb::keysym_from_name(name, 0)), down, mods: ModifiersState::default(), real: true }
    }

    fn switched_on<R>(name: &str, during: impl FnOnce() -> R) -> R {
        let dir = std::env::temp_dir().join(format!("trd-ro-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        use crate::own_helpers::Helper;
        use crate::own_helpers_set::switch;
        let out = crate::own_helpers::kept_at(&dir.join("own-helpers"), || {
            switch(Helper::Reader, true, true).unwrap();
            during()
        });
        let _ = std::fs::remove_dir_all(&dir);
        out
    }

    fn taken(f: Fate) -> Vec<String> {
        match f {
            Fate::Taken(l) => l,
            Fate::Pass => panic!("passed"),
        }
    }

    #[test]
    fn keys_pressed_with_the_reader_key_go_to_the_reader_as_lines() {
        let mut h = Held::default();
        assert_eq!(h.fate(&p(30, "a", true), true, false), Fate::Pass, "a key alone is the window's");
        assert_eq!(taken(h.fate(&p(58, "Caps_Lock", true), true, false)), ["key\tCaps_Lock\tdown\treader\treal\n"]);
        let mut shifted = p(19, "r", true);
        shifted.mods.shift = true;
        assert_eq!(taken(h.fate(&shifted, true, false)), ["key\tr\tdown\tshift,reader\treal\n"]);
        assert_eq!(taken(h.fate(&p(58, "Caps_Lock", false), true, false)), ["key\tCaps_Lock\tup\t-\treal\n"], "no tap after another key");
        let mut made = p(19, "r", false);
        made.real = false;
        assert_eq!(taken(h.fate(&made, true, false)), ["key\tr\tup\t-\tmade\n"]);
        assert_eq!(h.fate(&p(19, "r", true), true, false), Fate::Pass);
    }

    #[test]
    fn a_reader_key_alone_is_a_tap_and_with_no_reader_it_is_the_persons() {
        let mut h = Held::default();
        assert_eq!(h.fate(&p(58, "Caps_Lock", true), false, false), Fate::Pass, "no reader: Caps Lock is Caps Lock");
        assert_eq!(h.fate(&p(58, "Caps_Lock", false), false, false), Fate::Pass);
        taken(h.fate(&p(110, "Insert", true), true, false));
        assert_eq!(taken(h.fate(&p(110, "Insert", false), true, false)), ["key\tInsert\tup\t-\treal\n", "tap\tInsert\treal\n"]);
        taken(h.fate(&p(110, "Insert", true), true, false));
        h.undo(110);
        assert_eq!(h.fate(&p(110, "Insert", false), true, false), Fate::Pass, "a reader key whose line could not go is given back");
    }

    #[test]
    fn shift_or_ctrl_insert_is_the_windows_and_escape_is_the_readers_only_in_learn_mode() {
        let mut h = Held::default();
        let mut shifted = p(110, "Insert", true);
        shifted.mods.shift = true;
        assert_eq!(h.fate(&shifted, true, false), Fate::Pass);
        let mut ctrled = p(110, "Insert", true);
        ctrled.mods.ctrl = true;
        assert_eq!(h.fate(&ctrled, true, false), Fate::Pass);
        assert_eq!(h.fate(&p(110, "Insert", false), true, false), Fate::Pass, "its release goes to the window too");
        assert_eq!(h.fate(&p(1, "Escape", true), true, false), Fate::Pass, "not in learn mode");
        assert_eq!(h.fate(&p(1, "Escape", false), true, false), Fate::Pass);
        assert_eq!(h.fate(&p(1, "Escape", true), false, true), Fate::Pass, "no reader");
        assert_eq!(taken(h.fate(&p(1, "Escape", true), true, true)), ["key\tEscape\tdown\t-\treal\n"]);
        assert_eq!(taken(h.fate(&p(1, "Escape", false), true, false)), ["key\tEscape\tup\t-\treal\n"], "no tap for Escape");
    }

    #[test]
    fn what_the_reader_says_is_heard_a_line_at_a_time() {
        switched_on("heard", what_the_reader_says);
    }

    fn what_the_reader_says() {
        let (a, mut b) = UnixStream::pair().unwrap();
        a.set_nonblocking(true).unwrap();
        let mut k = ReaderKeys { link: Some(a), ..Default::default() };
        b.write_all(b"caps\ttoggle\nstate\tlearn\ton\nstate\tcap").unwrap();
        assert_eq!(k.hear(), [Ask::CapsToggle]);
        assert!(k.learn);
        b.write_all(b"s\ton\nnonsense\nstate\tlearn\toff\n").unwrap();
        assert_eq!(k.hear(), []);
        assert!(!k.learn);
        k.tell_caps(true);
        k.caps_changed(true);
        k.caps_changed(false);
        let mut got = vec![0; 32];
        let n = (&b).read(&mut got).unwrap();
        assert_eq!(String::from_utf8_lossy(&got[..n]), "state\tcaps\ton\nstate\tcaps\toff\n");
        k.learn = true;
        b.shutdown(std::net::Shutdown::Both).unwrap();
        drop(b);
        assert_eq!(k.hear(), []);
        assert!(k.link.is_none() && !k.learn, "a reader that went takes learn mode with it");
    }

    #[test]
    fn a_pass_line_asks_for_the_key_with_its_modifiers_and_a_bad_one_is_dropped() {
        switched_on("passed", || {
            let (a, mut b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), ..Default::default() };
            b.write_all(b"pass\tq\t-\npass\tDown\tshift,ctrl\npass\tx\treader\npass\tx\n").unwrap();
            assert_eq!(k.hear(), [Ask::Pass("q".into(), vec![]), Ask::Pass("Down".into(), vec!["Shift_L", "Control_L"])]);
        });
        assert_eq!(pass_mods("alt"), Some(vec!["Alt_L"]));
        assert_eq!(pass_mods("super"), None);
    }

    #[test]
    fn a_passed_key_never_reaches_the_lock_screen_or_the_administrators_password_question() {
        use super::may_be_passed_a_key as may;
        use crate::trusted::Kind;
        assert!(may(false, false, None) && may(false, false, Some(Kind::Ordinary)));
        assert!(!may(true, false, None), "typed over a locked screen");
        assert!(!may(false, true, None), "typed into the lock surface");
        assert!(!may(false, false, Some(Kind::Question)), "typed into the password question");
    }

    #[test]
    fn a_made_up_reader_key_or_escape_never_starts_reader_mode() {
        let mut h = Held::default();
        for name in ["Caps_Lock", "Insert"] {
            let mut made = p(58, name, true);
            made.real = false;
            assert_eq!(h.fate(&made, true, false), Fate::Pass, "{name}");
            assert!(h.idle(), "a made-up {name} changed what is held");
        }
        let mut esc = p(1, "Escape", true);
        esc.real = false;
        assert_eq!(h.fate(&esc, true, true), Fate::Pass, "a made-up Escape was taken in learn mode");
        assert!(h.idle());
        taken(h.fate(&p(58, "Caps_Lock", true), true, false));
        let mut made = p(19, "r", true);
        made.real = false;
        assert_eq!(taken(h.fate(&made, true, false)), ["key\tr\tdown\treader\tmade\n"]);
    }

    #[test]
    fn a_reader_that_writes_without_end_is_cut_off_and_never_read_for_long() {
        switched_on("flood", || {
            let (a, mut b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            b.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), ..Default::default() };
            let _ = b.write(&vec![b'x'; 100_000]);
            assert_eq!(k.hear(), []);
            assert!(k.link.is_none(), "a reader writing no line end kept its link");
            assert!(k.heard.is_empty());
            let (a, mut b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            b.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), ..Default::default() };
            let sent = b.write(&vec![b'\n'; 150_000]).unwrap();
            assert!(sent > 3 * MOST_PER_LOOK);
            assert_eq!(k.hear(), []);
            let mut left = vec![0u8; 400_000];
            let rest = k.link.as_ref().map(|mut l| l.read(&mut left).unwrap_or(0));
            assert!(rest.is_some_and(|n| n >= sent - 2 * MOST_PER_LOOK), "one look read more than it may, or dropped the reader: left {rest:?} of {sent}");
            assert!(k.heard.len() <= LONGEST_LINE);
        });
    }

    #[test]
    fn a_line_that_cannot_be_written_drops_the_link_so_the_reader_starts_clean() {
        let (a, _b) = UnixStream::pair().unwrap();
        a.set_nonblocking(true).unwrap();
        let mut k = ReaderKeys { link: Some(a), ..Default::default() };
        k.learn = true;
        let mut sent = 0;
        while k.link.is_some() && sent < 1_000_000 {
            k.tell_caps(sent % 2 == 0);
            sent += 1;
        }
        assert!(k.link.is_none(), "a reader that reads nothing kept its link after {sent} lines");
        assert!(!k.learn);
    }

    #[test]
    fn switched_off_in_settings_the_link_goes_and_learn_mode_lapses_without_an_answer() {
        let (a, _b) = UnixStream::pair().unwrap();
        a.set_nonblocking(true).unwrap();
        let mut k = ReaderKeys { link: Some(a), learn: true, ..Default::default() };
        assert_eq!(k.hear(), []);
        assert!(k.link.is_none() && !k.learn, "a reader switched off kept its link");
        switched_on("lapse", || {
            let (a, mut b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), learn: true, ..Default::default() };
            assert!(k.take(&p(1, "Escape", true)), "Escape is the reader's in learn mode");
            assert!(k.escape_sent.is_some());
            k.take(&p(1, "Escape", false));
            b.write_all(b"state\tlearn\ton\n").unwrap();
            k.hear();
            assert!(k.learn && k.escape_sent.is_none());
            assert!(k.take(&p(1, "Escape", true)));
            k.take(&p(1, "Escape", false));
            k.escape_sent = Some(Instant::now() - LEARN_ANSWER);
            assert!(!k.take(&p(1, "Escape", true)), "a reader that hung kept Escape from the window");
            assert!(!k.learn);
        });
    }

    #[test]
    fn while_browse_is_open_plain_keys_go_to_the_reader_and_to_no_window() {
        switched_on("browse", || {
            let (a, mut b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), ..Default::default() };
            assert!(!k.take(&p(30, "d", true)), "no browse yet: the window's");
            b.write_all(b"state\tbrowse\ton\n").unwrap();
            k.hear();
            assert!(k.browse && k.wants(false, false));
            assert!(k.take(&p(30, "d", true)) && k.take(&p(30, "d", false)));
            assert!(k.take(&p(1, "Escape", true)), "Escape is the reader's too");
            let mut ctrl = p(31, "s", true);
            ctrl.mods.ctrl = true;
            assert!(!k.take(&ctrl), "Ctrl keys are the window's");
            assert!(!k.take(&p(31, "s", false)), "and so is the release of one it never took");
            b.write_all(b"state\tbrowse\toff\n").unwrap();
            k.hear();
            assert!(!k.take(&p(30, "d", true)));
            b.write_all(b"state\tbrowse\ton\n").unwrap();
            k.hear();
            k.drop_link();
            assert!(!k.browse, "a link that dropped ended browse");
        });
    }

    #[test]
    fn what_is_held_is_forgotten_when_the_screen_locks_or_the_keys_are_let_go() {
        switched_on("release", || {
            let (a, _b) = UnixStream::pair().unwrap();
            a.set_nonblocking(true).unwrap();
            let mut k = ReaderKeys { link: Some(a), ..Default::default() };
            assert!(k.take(&p(58, "Caps_Lock", true)));
            assert!(!k.held.idle() && k.wants(false, false), "a key is held and every key is to be asked about");
            k.release();
            assert!(k.held.idle() && k.link.is_none());
            assert!(!k.wants(false, false), "idle, an ordinary key is not asked about");
            assert!(k.wants(true, false) && !k.wants(false, true));
            k.learn = true;
            assert!(k.wants(false, true), "Escape in learn mode is asked about");
            assert!(!k.take(&p(30, "a", true)), "after a release an ordinary key is the window's");
        });
    }

    #[test]
    fn the_watching_timer_runs_only_while_a_reader_is_linked() {
        let mut k = ReaderKeys::default();
        assert!(!k.start_watching(), "a timer was started with no reader");
        let (a, _b) = UnixStream::pair().unwrap();
        k.link = Some(a);
        assert!(k.start_watching() && !k.start_watching(), "one timer for a link");
        assert!(k.keep_watching());
        k.drop_link();
        assert!(!k.keep_watching(), "the timer went on with no reader");
        let (a, _b) = UnixStream::pair().unwrap();
        k.link = Some(a);
        assert!(k.start_watching(), "a new link did not start a new timer");
    }

    #[test]
    fn number_pad_keys_are_named_by_their_numbers() {
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let map = xkb::Keymap::new_from_names(&ctx, "", "", "us", "", None, xkb::KEYMAP_COMPILE_NO_FLAGS).unwrap();
        let at = |evdev: u32| name(&map, 0, (evdev + 8).into()).unwrap();
        assert_eq!((at(76), at(19), at(51), at(58), at(110)), ("KP_5".into(), "r".into(), "comma".into(), "Caps_Lock".into(), "Insert".into()));
    }
    #[test]
    fn only_the_real_reader_program_is_accepted() {
        let want = Path::new("/home/x/.local/share/trindle/plugins/org.trindle.ScreenReader/bin/trindle-screen-reader");
        assert!(exe_is_reader(want, want));
        assert!(!exe_is_reader(Path::new("/usr/bin/python3"), want));
        assert!(!exe_is_reader(Path::new("/home/x/.local/share/trindle/plugins/org.trindle.ScreenReader/bin/trindle-screen-reader (deleted)"), want));
        assert!(!exe_is_reader(Path::new("/home/x/.local/share/trindle/plugins/com.example.Other/bin/trindle-screen-reader"), want));
    }

    #[test]
    fn a_socketpair_peer_is_checked_by_its_program() {
        let (a, _b) = UnixStream::pair().unwrap();
        let me = std::env::current_exe().unwrap();
        TEST_EXE.with(|t| *t.borrow_mut() = Some(me));
        assert!(peer_is_reader(&a), "the program itself was refused");
        TEST_EXE.with(|t| *t.borrow_mut() = Some(PathBuf::from("/usr/bin/not-the-reader")));
        assert!(!peer_is_reader(&a), "another program was accepted");
        TEST_EXE.with(|t| *t.borrow_mut() = Some(PathBuf::from("/usr/bin/not-the-reader (deleted)")));
        assert!(!peer_is_reader(&a));
        TEST_EXE.with(|t| *t.borrow_mut() = None);
    }

    #[test]
    fn a_pid_given_to_another_program_is_no_reader() {
        use std::os::fd::{FromRawFd, OwnedFd};
        let pidfd = |pid: u32| {
            // SAFETY: a plain system call with no pointers; the result is checked before use.
            let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::c_int, 0) };
            assert!(fd >= 0, "pidfd_open({pid}) failed: {}", std::io::Error::last_os_error());
            // SAFETY: a pidfd just made and checked above, owned by nothing else.
            unsafe { OwnedFd::from_raw_fd(fd as libc::c_int) }
        };
        let me = std::env::current_exe().unwrap();
        let mut child = std::process::Command::new("sleep").arg("5").spawn().unwrap();
        let gone = pidfd(child.id());
        assert!(reader_is(std::process::id() as i32, &pidfd(std::process::id()), &me),
                "a live process running the reader's file was refused");
        child.kill().unwrap();
        child.wait().unwrap();
        assert!(!reader_is(std::process::id() as i32, &gone, &me), "a pidfd of a process that ended passed for a pid that lives");
    }

    #[test]
    fn a_pid_that_has_gone_has_no_program() {
        let mut child = std::process::Command::new("true").spawn().unwrap();
        let pid = child.id() as i32;
        child.wait().unwrap();
        assert!(exe_of(pid).is_none(), "a pid that exited still named a program");
        assert!(exe_of(std::process::id() as i32).is_some());
    }

    #[test]
    fn the_reader_is_sent_its_keys_and_the_window_none_of_them() {
        use crate::own_helpers::Helper;
        use crate::own_helpers_set::switch;
        use crate::rig::{alone, Rig};
        use crate::state::ClientState;
        use std::io::Read;
        alone("reader_keys::tests::the_reader_is_sent_its_keys_and_the_window_none_of_them", |dir| {
            let mark = dir.join("home/.local/state/trindle/own-helpers");
            std::fs::create_dir_all(mark.parent().unwrap()).unwrap();
            crate::own_helpers::kept_at(&mark, || {
                const CAPS: u32 = 58;
                const INSERT: u32 = 110;
                const R: u32 = 19;
                const A: u32 = 30;
                let mut rig = Rig::new(dir);
                let run = std::path::PathBuf::from(format!("/tmp/trd-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&run);
                std::env::set_var("XDG_RUNTIME_DIR", &run);
                let folder = run.join(FOLDER);
                std::fs::create_dir_all(&folder).unwrap();
                std::fs::set_permissions(&folder, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
                let listener = std::os::unix::net::UnixListener::bind(folder.join(SOCKET)).unwrap();
                listener.set_nonblocking(true).unwrap();
                let app = rig.join(ClientState::default());
                rig.window(app, false);
                rig.onto_the_window_in_front();
                rig.key_by_hand(CAPS);
                rig.key_by_hand(CAPS);
                assert_eq!(rig.pressed(app), [CAPS, CAPS]);
                assert!(listener.accept().is_err(), "the reader was reached with its switch off");
                switch(Helper::Reader, true, true).unwrap();
                rig.key_by_hand_only(CAPS, true);
                rig.key_by_hand(R);
                rig.key_by_hand_only(CAPS, false);
                rig.key_by_hand(INSERT);
                assert_eq!(rig.pressed(app), [CAPS, CAPS], "a reader key or a key pressed with one reached the window");
                let (mut link, _) = listener.accept().expect("the compositor did not reach the reader");
                link.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                let want = "state\tplainkeys\tavailable\nkey\tCaps_Lock\tdown\treader\treal\nkey\tr\tdown\treader\treal\nkey\tr\tup\treader\treal\n\
                            key\tCaps_Lock\tup\t-\treal\nkey\tInsert\tdown\treader\treal\nkey\tInsert\tup\t-\treal\ntap\tInsert\treal\n";
                let mut got = vec![0; want.len()];
                link.read_exact(&mut got).unwrap();
                assert_eq!(String::from_utf8_lossy(&got), want);
                rig.key_by_hand(A);
                assert_eq!(rig.pressed(app), [CAPS, CAPS, A]);
                rig.made_up_key(INSERT);
                assert_eq!(rig.pressed(app), [CAPS, CAPS, A, INSERT], "a made-up reader key was kept from the window");
                link.set_nonblocking(true).unwrap();
                assert!(link.read(&mut [0u8; 8]).is_err(), "the reader was told of a made-up reader key");
                link.set_nonblocking(false).unwrap();
                drop(link);
                drop(listener);
                std::fs::remove_file(folder.join(SOCKET)).unwrap();
                rig.key_by_hand(CAPS);
                assert_eq!(rig.pressed(app), [CAPS, CAPS, A, INSERT, CAPS], "with no reader, Caps Lock was kept from the window");
                let _ = std::fs::remove_dir_all(&run);
            });
        });
    }

    fn insert_with_shift_or_ctrl_is_the_windows(rig: &mut crate::rig::Rig, app: usize) {
        const INSERT: u32 = 110;
        const SHIFT: u32 = 42;
        const CTRL: u32 = 29;
        for held in [SHIFT, CTRL] {
            let before = rig.pressed(app).len();
            rig.key_by_hand_only(held, true);
            rig.key_by_hand(INSERT);
            rig.key_by_hand_only(held, false);
            assert_eq!(rig.pressed(app)[before..], [held, INSERT], "Insert with {held} did not reach the window");
        }
    }

    fn the_reader_toggles_caps_lock_and_is_answered(rig: &mut crate::rig::Rig, mut link: &std::os::unix::net::UnixStream) {
        use std::io::Read;
        let caps = |rig: &crate::rig::Rig| rig.state.seat.get_keyboard().unwrap().modifier_state().caps_lock;
        assert!(!caps(rig));
        link.write_all(b"caps\ttoggle\n").unwrap();
        std::thread::sleep(Duration::from_millis(50));
        rig.state.reader_listen();
        assert!(caps(rig), "Caps Lock was not toggled");
        let mut got = vec![0; "state\tcaps\ton\n".len()];
        link.read_exact(&mut got).unwrap();
        assert_eq!(String::from_utf8_lossy(&got), "state\tcaps\ton\n");
        link.write_all(b"caps\ttoggle\n").unwrap();
        std::thread::sleep(Duration::from_millis(50));
        rig.state.reader_listen();
        assert!(!caps(rig));
        let mut got = vec![0; "state\tcaps\toff\n".len()];
        link.read_exact(&mut got).unwrap();
        assert_eq!(String::from_utf8_lossy(&got), "state\tcaps\toff\n");
    }

    fn escape_goes_to_the_reader_only_in_learn_mode(rig: &mut crate::rig::Rig, app: usize, mut link: &std::os::unix::net::UnixStream) {
        use std::io::Read;
        const ESC: u32 = 1;
        let before = rig.pressed(app).len();
        link.write_all(b"state\tlearn\ton\n").unwrap();
        std::thread::sleep(Duration::from_millis(50));
        rig.state.reader_listen();
        rig.key_by_hand(ESC);
        assert_eq!(rig.pressed(app).len(), before, "Escape reached the window in learn mode");
        let want = "key\tEscape\tdown\t-\treal\nkey\tEscape\tup\t-\treal\n";
        let mut got = vec![0; want.len()];
        link.read_exact(&mut got).unwrap();
        assert_eq!(String::from_utf8_lossy(&got), want);
        link.write_all(b"state\tlearn\toff\n").unwrap();
        std::thread::sleep(Duration::from_millis(50));
        rig.key_by_hand(ESC);
        assert_eq!(rig.pressed(app)[before..], [ESC], "Escape was kept from the window with learn mode off");
    }

    #[test]
    fn the_reader_can_toggle_caps_lock_and_hears_escape_in_learn_mode() {
        use crate::own_helpers::Helper;
        use crate::own_helpers_set::switch;
        use crate::rig::{alone, Rig};
        use crate::state::ClientState;
        use std::io::Read;
        alone("reader_keys::tests::the_reader_can_toggle_caps_lock_and_hears_escape_in_learn_mode", |dir| {
            let mark = dir.join("home/.local/state/trindle/own-helpers");
            std::fs::create_dir_all(mark.parent().unwrap()).unwrap();
            crate::own_helpers::kept_at(&mark, || {
                const CAPS: u32 = 58;
                let mut rig = Rig::new(dir);
                let run = std::path::PathBuf::from(format!("/tmp/trd2-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&run);
                std::env::set_var("XDG_RUNTIME_DIR", &run);
                let folder = run.join(FOLDER);
                std::fs::create_dir_all(&folder).unwrap();
                std::fs::set_permissions(&folder, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
                let listener = std::os::unix::net::UnixListener::bind(folder.join(SOCKET)).unwrap();
                listener.set_nonblocking(true).unwrap();
                let app = rig.join(ClientState::default());
                rig.window(app, false);
                rig.onto_the_window_in_front();
                switch(Helper::Reader, true, true).unwrap();
                rig.key_by_hand(CAPS);
                let (mut link, _) = listener.accept().expect("the compositor did not reach the reader");
                link.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                let want = "state\tplainkeys\tavailable\nkey\tCaps_Lock\tdown\treader\treal\nkey\tCaps_Lock\tup\t-\treal\ntap\tCaps_Lock\treal\n";
                let mut got = vec![0; want.len()];
                link.read_exact(&mut got).unwrap();
                assert_eq!(String::from_utf8_lossy(&got), want);
                insert_with_shift_or_ctrl_is_the_windows(&mut rig, app);
                the_reader_toggles_caps_lock_and_is_answered(&mut rig, &link);
                escape_goes_to_the_reader_only_in_learn_mode(&mut rig, app, &link);
                let before = rig.pressed(app).len();
                link.write_all(b"pass\tq\tshift\n").unwrap();
                std::thread::sleep(Duration::from_millis(50));
                rig.state.reader_listen();
                rig.pump();
                assert_eq!(rig.pressed(app)[before..], [42, 16], "a passed Shift+q did not reach the window");
                rig.state.lock_for_rig();
                rig.state.pass_key("q", &[]);
                rig.pump();
                rig.state.unlock_for_rig();
                assert_eq!(rig.pressed(app).len(), before + 2, "a key was passed over the lock screen");
                let _ = std::fs::remove_dir_all(&run);
            });
        });
    }

    #[test]
    fn a_reader_key_let_go_over_the_lock_screen_does_not_leave_the_keyboard_dead() {
        use crate::own_helpers::Helper;
        use crate::own_helpers_set::switch;
        use crate::rig::{alone, Rig};
        use crate::state::ClientState;
        alone("reader_keys::tests::a_reader_key_let_go_over_the_lock_screen_does_not_leave_the_keyboard_dead", |dir| {
            let mark = dir.join("home/.local/state/trindle/own-helpers");
            std::fs::create_dir_all(mark.parent().unwrap()).unwrap();
            crate::own_helpers::kept_at(&mark, || {
                const CAPS: u32 = 58;
                const A: u32 = 30;
                let mut rig = Rig::new(dir);
                let run = std::path::PathBuf::from(format!("/tmp/trd3-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&run);
                std::env::set_var("XDG_RUNTIME_DIR", &run);
                let folder = run.join(FOLDER);
                std::fs::create_dir_all(&folder).unwrap();
                std::fs::set_permissions(&folder, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
                let listener = std::os::unix::net::UnixListener::bind(folder.join(SOCKET)).unwrap();
                listener.set_nonblocking(true).unwrap();
                let app = rig.join(ClientState::default());
                rig.window(app, false);
                rig.onto_the_window_in_front();
                switch(Helper::Reader, true, true).unwrap();
                rig.key_by_hand_only(CAPS, true);
                rig.key_by_hand(A);
                assert_eq!(rig.pressed(app), [] as [u32; 0], "a key with the reader key held reached the window");
                rig.state.lock_for_rig();
                rig.key_by_hand_only(CAPS, false);
                rig.state.unlock_for_rig();
                rig.onto_the_window_in_front();
                rig.key_by_hand(A);
                assert_eq!(rig.pressed(app), [A], "the keyboard stayed the reader's after the lock");
                let _ = std::fs::remove_dir_all(&run);
            });
        });
    }
}
