//! Tests how the compositor treats older X11 programs as their window manager.

use std::time::{Duration, Instant};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ConfigureNotifyEvent, ConfigureWindowAux, ConnectionExt, CreateWindowAux, EventMask, PropMode,
    Window, WindowClass,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::CURRENT_TIME;

use crate::toolkit::drag_test::{harness, Plain, TEXT};

fn display() -> String {
    std::env::var("TRINDLE_OLD_DISPLAY").expect("TRINDLE_OLD_DISPLAY names the older programs' display, such as :2")
}

fn atom(conn: &RustConnection, name: &str) -> Atom {
    conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Normal,
    Dialog,
    OverrideRedirect,
}

const KINDS: [Kind; 3] = [Kind::Normal, Kind::Dialog, Kind::OverrideRedirect];

#[derive(Clone, Copy, Debug)]
struct Notice {
    synthetic: bool,
    after_map: bool,
    x: i16,
    y: i16,
    w: u16,
    h: u16,
}

struct Old {
    conn: RustConnection,
    root: Window,
    win: Window,
    kind: Kind,
    notices: Vec<Notice>,
    mapped: bool,
    pasted: Option<Option<Vec<u8>>>,
    prop: Atom,
}

impl Old {
    fn open(kind: Kind, at: (i16, i16), size: (u16, u16), parent: Option<Window>, ask_first: bool) -> Old {
        let mut old = Old::create(kind, at, size, parent);
        if ask_first {
            old.ask_same();
        }
        old.show();
        old
    }

    fn show(&mut self) {
        self.conn.map_window(self.win).unwrap();
        self.conn.flush().unwrap();
    }

    fn create(kind: Kind, at: (i16, i16), size: (u16, u16), parent: Option<Window>) -> Old {
        let (conn, screen) = RustConnection::connect(Some(&display())).expect("the older programs' display");
        let root = conn.setup().roots[screen].root;
        let win = conn.generate_id().unwrap();
        let mut aux = CreateWindowAux::new().background_pixel(0x00a0a0a0).event_mask(
            EventMask::STRUCTURE_NOTIFY | EventMask::EXPOSURE | EventMask::PROPERTY_CHANGE,
        );
        if kind == Kind::OverrideRedirect {
            aux = aux.override_redirect(1);
        }
        conn.create_window(x11rb::COPY_DEPTH_FROM_PARENT, win, root, at.0, at.1, size.0, size.1, 0,
                           WindowClass::INPUT_OUTPUT, 0, &aux).unwrap();
        let title = format!("An older {kind:?} window");
        conn.change_property8(PropMode::REPLACE, win, AtomEnum::WM_NAME, AtomEnum::STRING, title.as_bytes()).unwrap();
        if kind == Kind::Dialog {
            if let Some(parent) = parent {
                conn.change_property32(PropMode::REPLACE, win, AtomEnum::WM_TRANSIENT_FOR, AtomEnum::WINDOW, &[parent]).unwrap();
            }
            let ty = atom(&conn, "_NET_WM_WINDOW_TYPE");
            let dialog = atom(&conn, "_NET_WM_WINDOW_TYPE_DIALOG");
            conn.change_property32(PropMode::REPLACE, win, ty, AtomEnum::ATOM, &[dialog]).unwrap();
        }
        let prop = atom(&conn, "TEST_PASTE");
        Old { conn, root, win, kind, notices: Vec::new(), mapped: false, pasted: None, prop }
    }

    fn real(&self) -> (i16, i16, u16, u16) {
        let g = self.conn.get_geometry(self.win).unwrap().reply().unwrap();
        let at = self.conn.translate_coordinates(self.win, self.root, 0, 0).unwrap().reply().unwrap();
        (at.dst_x, at.dst_y, g.width, g.height)
    }

    fn ask_same(&mut self) {
        let (x, y, w, h) = self.real();
        let g = self.conn.get_geometry(self.win).unwrap().reply().unwrap();
        let (x, y) = if self.mapped || self.kind == Kind::OverrideRedirect { (x, y) } else { (g.x, g.y) };
        self.conn.configure_window(self.win, &ConfigureWindowAux::new().x(x as i32).y(y as i32)
            .width(w as u32).height(h as u32)).unwrap();
        self.conn.flush().unwrap();
    }

    fn pump(&mut self) {
        while let Some(event) = self.conn.poll_for_event().unwrap() {
            match event {
                Event::ConfigureNotify(ConfigureNotifyEvent { window, response_type, x, y, width, height, .. })
                    if window == self.win =>
                {
                    self.notices.push(Notice { synthetic: response_type & 0x80 != 0, after_map: self.mapped, x, y, w: width, h: height });
                }
                Event::MapNotify(m) if m.window == self.win => self.mapped = true,
                Event::SelectionNotify(n) => {
                    self.pasted = Some(if n.property == 0u32 {
                        None
                    } else {
                        let r = self.conn.get_property(true, self.win, n.property, AtomEnum::ANY, 0, 1 << 16)
                            .unwrap().reply().unwrap();
                        Some(r.value)
                    });
                }
                _ => {}
            }
        }
    }

    fn paste(&mut self, plain: &mut Plain, selection: &str, target: &str) -> Option<Vec<u8>> {
        self.pasted = None;
        let (s, t) = (atom(&self.conn, selection), atom(&self.conn, target));
        self.conn.convert_selection(self.win, s, t, self.prop, CURRENT_TIME).unwrap();
        self.conn.flush().unwrap();
        let end = Instant::now() + Duration::from_secs(10);
        loop {
            plain.turn();
            self.pump();
            if let Some(answer) = self.pasted.take() {
                return answer;
            }
            assert!(Instant::now() < end, "no answer to a paste of {target} from {selection}");
        }
    }

    fn owner(&self, selection: &str) -> Window {
        let s = atom(&self.conn, selection);
        self.conn.get_selection_owner(s).unwrap().reply().unwrap().owner
    }
}

fn settle(plain: &mut Plain, olds: &mut [&mut Old], ms: u64) {
    let end = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < end {
        plain.turn();
        for old in olds.iter_mut() {
            old.pump();
        }
    }
}

fn until(plain: &mut Plain, old: &mut Old, what: &str, done: impl Fn(&Old) -> bool) {
    let end = Instant::now() + Duration::from_secs(10);
    while !done(old) {
        assert!(Instant::now() < end, "never happened: {what}");
        settle(plain, &mut [old], 20);
    }
}

fn told_the_truth(old: &Old, since: usize) -> Result<(), String> {
    let real = old.real();
    match old.notices[since..].iter().rev().find(|n| n.synthetic) {
        Some(n) if since == 0 && old.mapped && !old.notices.iter().any(|n| n.synthetic && n.after_map) => {
            Err(format!("{:?}: told {n:?} only before its window was shown", old.kind))
        }
        None => Err(format!("{:?}: no synthetic ConfigureNotify (got {:?}); real {real:?}", old.kind, &old.notices[since..])),
        Some(n) if (n.x, n.y, n.w, n.h) != real => {
            Err(format!("{:?}: told {n:?}, but the window is really at {real:?}", old.kind))
        }
        Some(_) => Ok(()),
    }
}

fn open_plain() -> Option<Plain> {
    let run = harness()?;
    let socket = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());
    Some(Plain::open(&socket, "A plain program", (300, 60)))
}

fn parent_window() -> Old {
    Old::open(Kind::Normal, (30, 30), (200, 120), None, false)
}

#[test]
#[ignore]
fn older_window_is_told_where_it_is_when_it_is_shown() {
    let Some(mut plain) = open_plain() else { return };
    let mut parent = parent_window();
    settle(&mut plain, &mut [&mut parent], 1500);
    for ask_first in [false, true] {
        for kind in [Kind::Normal, Kind::Dialog] {
            let mut old = Old::open(kind, (120, 90), (260, 140), Some(parent.win), ask_first);
            until(&mut plain, &mut old, "the window was shown", |o| o.mapped);
            settle(&mut plain, &mut [&mut old, &mut parent], 500);
            if let Err(e) = told_the_truth(&old, 0) {
                panic!("asked first: {ask_first}: {e}");
            }
        }
    }
}

#[test]
#[ignore]
fn older_window_asking_before_it_is_shown_is_told_where_it_is() {
    let Some(mut plain) = open_plain() else { return };
    let mut parent = parent_window();
    settle(&mut plain, &mut [&mut parent], 1500);
    if let Err(e) = told_the_truth(&parent, 0) {
        panic!("the first window: {e}");
    }
    for kind in [Kind::Normal, Kind::Dialog] {
        let mut old = Old::create(kind, (150, 110), (240, 150), Some(parent.win));
        old.ask_same();
        settle(&mut plain, &mut [&mut old, &mut parent], 500);
        assert!(!old.mapped);
        if let Err(e) = told_the_truth(&old, 0) {
            panic!("before it was shown: {e}");
        }
        old.show();
        until(&mut plain, &mut old, "the window was shown", |o| o.mapped);
        settle(&mut plain, &mut [&mut old, &mut parent], 500);
        let n = old.notices.len();
        let shown = old.notices.iter().rposition(|n| n.synthetic && n.after_map);
        assert!(shown.is_some(), "{kind:?}: not told once shown ({n} notices)");
    }
}

#[test]
#[ignore]
fn older_window_that_overrides_the_manager_stays_where_it_asked() {
    let Some(mut plain) = open_plain() else { return };
    let mut old = Old::open(Kind::OverrideRedirect, (333, 222), (150, 90), None, false);
    until(&mut plain, &mut old, "the window was shown", |o| o.mapped);
    settle(&mut plain, &mut [&mut old], 500);
    assert_eq!(old.real(), (333, 222, 150, 90));
    old.conn.configure_window(old.win, &ConfigureWindowAux::new().x(300).y(200).width(170).height(100)).unwrap();
    old.conn.flush().unwrap();
    settle(&mut plain, &mut [&mut old], 500);
    assert_eq!(old.real(), (300, 200, 170, 100));
    let last = old.notices.last().expect("told of its own move");
    assert_eq!((last.x, last.y, last.w, last.h), (300, 200, 170, 100));
}

#[test]
#[ignore]
fn older_window_asking_for_what_it_has_is_told_so() {
    let Some(mut plain) = open_plain() else { return };
    let mut parent = parent_window();
    settle(&mut plain, &mut [&mut parent], 1500);
    for kind in [Kind::Normal, Kind::Dialog] {
        let mut old = Old::open(kind, (140, 100), (250, 130), Some(parent.win), false);
        until(&mut plain, &mut old, "the window was shown", |o| o.mapped);
        settle(&mut plain, &mut [&mut old, &mut parent], 400);
        for round in 0..3 {
            let since = old.notices.len();
            old.ask_same();
            settle(&mut plain, &mut [&mut old, &mut parent], 400);
            if let Err(e) = told_the_truth(&old, since) {
                panic!("request {round}: {e}");
            }
        }
        let since = old.notices.len();
        old.conn.configure_window(old.win, &ConfigureWindowAux::new().width(310).height(170)).unwrap();
        old.conn.flush().unwrap();
        settle(&mut plain, &mut [&mut old, &mut parent], 500);
        assert_eq!(old.real().2, 310);
        assert_eq!(old.real().3, 170);
        if let Err(e) = told_the_truth(&old, since) {
            panic!("resized: {e}");
        }
    }
}

fn text_kinds(plain: &mut Plain, text: &str) {
    plain.told.serving.clear();
    for kind in [TEXT, "text/plain", "UTF8_STRING", "STRING"] {
        plain.told.serving.insert(kind.into(), text.as_bytes().to_vec());
    }
}

#[test]
#[ignore]
fn older_window_pastes_what_a_wayland_program_copied() {
    if harness().is_none() {
        return;
    }
    let mut parent = parent_window();
    for kind in KINDS {
        let mut old = Old::open(kind, (160, 120), (120, 80), Some(parent.win), false);
        let mut plain = open_plain().unwrap();
        until(&mut plain, &mut old, "the window was shown", |o| o.mapped);
        settle(&mut plain, &mut [&mut parent], 300);
        for (selection, primary) in [("CLIPBOARD", false), ("PRIMARY", true)] {
            assert_eq!(old.owner(selection), x11rb::NONE, "{kind:?}: {selection} owned before any copy");
            let text = format!("copied on Wayland for {selection}");
            text_kinds(&mut plain, &text);
            if primary { plain.copy_primary(true) } else { plain.copy(true) }
            settle(&mut plain, &mut [&mut old], 400);
            assert_ne!(old.owner(selection), x11rb::NONE, "{kind:?}: {selection} not offered to the older programs");
            for target in ["UTF8_STRING", "STRING", "text/plain", "TEXT"] {
                let got = old.paste(&mut plain, selection, target);
                assert_eq!(got.as_deref(), Some(text.as_bytes()), "{kind:?}: {selection} as {target}");
            }
            let targets = old.paste(&mut plain, selection, "TARGETS").expect("a list of what can be pasted");
            let names: Vec<String> = targets.chunks(4)
                .map(|c| u32::from_ne_bytes([c[0], c[1], c[2], c[3]]))
                .map(|a| String::from_utf8(old.conn.get_atom_name(a).unwrap().reply().unwrap().name).unwrap())
                .collect();
            assert!(names.iter().any(|n| n == "UTF8_STRING"), "{kind:?}: {selection} lists {names:?}");
            let again = format!("copied again for {selection}");
            text_kinds(&mut plain, &again);
            if primary { plain.copy_primary(true) } else { plain.copy(true) }
            settle(&mut plain, &mut [&mut old], 300);
            assert_eq!(old.paste(&mut plain, selection, "UTF8_STRING").as_deref(), Some(again.as_bytes()));
            if primary { plain.copy_primary(false) } else { plain.copy(false) }
            settle(&mut plain, &mut [&mut old], 400);
            assert_eq!(old.owner(selection), x11rb::NONE, "{kind:?}: {selection} still offered after it was cleared");
            assert_eq!(old.paste(&mut plain, selection, "UTF8_STRING"), None, "{kind:?}: {selection} pasted after clearing");
        }
    }
}
