//! Drag and drop between older (X11) programs and Wayland ones, against a running trindle-compose (the headless harness, as in drag_test.rs) whose Xwayland is the display named in TRINDLE_OLD_DISPLAY (":2").

use std::collections::HashMap;
use std::time::{Duration, Instant};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ClientMessageEvent, ConnectionExt, CreateWindowAux, EventMask, GrabMode, PropMode, Window,
    WindowClass, SELECTION_NOTIFY_EVENT,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::CURRENT_TIME;

use crate::toolkit::drag_test::{harness, Mouse, Plain, LEFT_BUTTON};

const OLD_SIZE: (u16, u16) = (80, 360);
const TEXT: &str = "UTF8_STRING";
const TEXT_MIME: &str = "text/plain;charset=utf-8";

struct Atoms {
    aware: Atom,
    enter: Atom,
    position: Atom,
    status: Atom,
    leave: Atom,
    drop: Atom,
    finished: Atom,
    selection: Atom,
    type_list: Atom,
    action_list: Atom,
    copy: Atom,
    move_: Atom,
    targets: Atom,
    incr: Atom,
    prop: Atom,
}

fn atom(conn: &RustConnection, name: &str) -> Atom {
    conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom
}

fn atom_name(conn: &RustConnection, atom: Atom) -> String {
    String::from_utf8(conn.get_atom_name(atom).unwrap().reply().unwrap().name).unwrap()
}

struct Drag {
    target: Option<Window>,
    accepted: bool,
}

pub struct Old {
    conn: RustConnection,
    root: Window,
    pub win: Window,
    a: Atoms,
    pub accept: bool,
    pub ask_for: Option<String>,
    pub entered: Option<Vec<String>>,
    pub positions: usize,
    pub dropped: bool,
    pub got: Option<Vec<u8>>,
    pub refused: bool,
    incr: Option<Vec<u8>>,
    from: Window,
    pub serving: HashMap<String, Vec<u8>>,
    pub drag_on_press: bool,
    drag: Option<Drag>,
    pub finished: Option<(bool, Atom)>,
    pub asked_for: Vec<String>,
}

fn intern_atoms(conn: &RustConnection) -> Atoms {
    Atoms {
        aware: atom(conn, "XdndAware"),
        enter: atom(conn, "XdndEnter"),
        position: atom(conn, "XdndPosition"),
        status: atom(conn, "XdndStatus"),
        leave: atom(conn, "XdndLeave"),
        drop: atom(conn, "XdndDrop"),
        finished: atom(conn, "XdndFinished"),
        selection: atom(conn, "XdndSelection"),
        type_list: atom(conn, "XdndTypeList"),
        action_list: atom(conn, "XdndActionList"),
        copy: atom(conn, "XdndActionCopy"),
        move_: atom(conn, "XdndActionMove"),
        targets: atom(conn, "TARGETS"),
        incr: atom(conn, "INCR"),
        prop: atom(conn, "TEST_DATA"),
    }
}

fn create_old_window(conn: &RustConnection, win: Window, root: Window, at: (i16, i16), size: (u16, u16)) {
    conn.create_window(
        x11rb::COPY_DEPTH_FROM_PARENT,
        win,
        root,
        at.0,
        at.1,
        size.0,
        size.1,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(0x00a0a0a0).event_mask(
            EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::PROPERTY_CHANGE
                | EventMask::EXPOSURE
                | EventMask::STRUCTURE_NOTIFY,
        ),
    )
    .unwrap();
}

impl Old {
    pub fn open(display: &str, at: (i16, i16), size: (u16, u16), aware: bool) -> Old {
        let (conn, screen) = RustConnection::connect(Some(display)).expect("the older programs' display");
        let root = conn.setup().roots[screen].root;
        let win = conn.generate_id().unwrap();
        let a = intern_atoms(&conn);
        create_old_window(&conn, win, root, at, size);
        conn.change_property8(PropMode::REPLACE, win, AtomEnum::WM_NAME, AtomEnum::STRING, b"An older program").unwrap();
        if aware {
            conn.change_property32(PropMode::REPLACE, win, a.aware, AtomEnum::ATOM, &[5]).unwrap();
        }
        conn.map_window(win).unwrap();
        conn.flush().unwrap();
        Old {
            conn,
            root,
            win,
            a,
            accept: true,
            ask_for: None,
            entered: None,
            positions: 0,
            dropped: false,
            got: None,
            refused: false,
            incr: None,
            from: 0,
            serving: HashMap::new(),
            drag_on_press: false,
            drag: None,
            finished: None,
            asked_for: Vec::new(),
        }
    }

    pub fn at(&self) -> (i32, i32) {
        let r = self.conn.translate_coordinates(self.win, self.root, 0, 0).unwrap().reply().unwrap();
        (i32::from(r.dst_x), i32::from(r.dst_y))
    }

    fn send(&self, to: Window, window: Window, type_: Atom, data: [u32; 5]) {
        self.conn.send_event(false, to, EventMask::NO_EVENT, ClientMessageEvent::new(32, window, type_, data)).unwrap();
        self.conn.flush().unwrap();
    }

    #[allow(clippy::let_and_return)]
    fn aware_version(&self, window: Window) -> Option<u32> {
        let r = self.conn.get_property(false, window, self.a.aware, AtomEnum::ATOM, 0, 1).ok()?.reply().ok()?;
        let first = r.value32()?.next();
        first
    }

    fn target_at(&self, x: i16, y: i16) -> Option<Window> {
        let mut w = self.root;
        loop {
            let r = self.conn.translate_coordinates(self.root, w, x, y).ok()?.reply().ok()?;
            if w != self.root && w != self.win && self.aware_version(w).is_some() {
                return Some(w);
            }
            if r.child == 0 {
                return None;
            }
            w = r.child;
        }
    }

    fn start_drag(&mut self) {
        let _ = self.conn.grab_pointer(
            true,
            self.win,
            EventMask::POINTER_MOTION | EventMask::BUTTON_RELEASE,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
            0u32,
            0u32,
            CURRENT_TIME,
        );
        self.conn.set_selection_owner(self.win, self.a.selection, CURRENT_TIME).unwrap();
        self.conn
            .change_property32(PropMode::REPLACE, self.win, self.a.action_list, AtomEnum::ATOM, &[self.a.copy, self.a.move_])
            .unwrap();
        self.conn.flush().unwrap();
        self.drag = Some(Drag { target: None, accepted: false });
        self.finished = None;
        self.asked_for.clear();
    }

    fn kinds_as_atoms(&self) -> Vec<Atom> {
        let mut names: Vec<&String> = self.serving.keys().collect();
        names.sort();
        names.into_iter().map(|n| atom(&self.conn, n)).collect()
    }

    fn enter(&self, target: Window) {
        let atoms = self.kinds_as_atoms();
        let more = atoms.len() > 3;
        if more {
            self.conn.change_property32(PropMode::REPLACE, self.win, self.a.type_list, AtomEnum::ATOM, &atoms).unwrap();
        }
        let t = |i: usize| if more { 0 } else { atoms.get(i).copied().unwrap_or(0) };
        self.send(target, target, self.a.enter, [self.win, (5 << 24) | u32::from(more), t(0), t(1), t(2)]);
    }

    fn moved(&mut self, x: i16, y: i16) {
        let target = self.target_at(x, y);
        let old = self.drag.as_ref().and_then(|d| d.target);
        if target != old {
            if let Some(o) = old {
                self.send(o, o, self.a.leave, [self.win, 0, 0, 0, 0]);
            }
            if let Some(t) = target {
                self.enter(t);
            }
            self.drag.as_mut().unwrap().accepted = false;
            self.drag.as_mut().unwrap().target = target;
        }
        if let Some(t) = target {
            let packed = ((x as u32) << 16) | (y as u32 & 0xffff);
            self.send(t, t, self.a.position, [self.win, 0, packed, CURRENT_TIME, self.a.copy]);
        }
    }

    fn released(&mut self) {
        let _ = self.conn.ungrab_pointer(CURRENT_TIME);
        let Some(d) = self.drag.take() else { return };
        if let Some(t) = d.target {
            if d.accepted {
                self.send(t, t, self.a.drop, [self.win, 0, CURRENT_TIME, 0, 0]);
            } else {
                self.send(t, t, self.a.leave, [self.win, 0, 0, 0, 0]);
            }
        }
        let _ = self.conn.flush();
    }

    fn finish_drop(&mut self, ok: bool) {
        self.send(self.from, self.from, self.a.finished, [self.win, u32::from(ok), self.a.copy, 0, 0]);
    }

    fn read_property(&mut self) -> Option<(Atom, Vec<u8>)> {
        let r = self.conn.get_property(true, self.win, self.a.prop, AtomEnum::ANY, 0, 0x1fff_ffff).ok()?.reply().ok()?;
        Some((r.type_, r.value))
    }

    pub fn pump(&mut self) {
        while let Some(event) = self.conn.poll_for_event().unwrap() {
            match event {
                Event::ClientMessage(m) => self.on_client_message(m),
                Event::SelectionNotify(n) => {
                    if n.property == 0 {
                        self.refused = true;
                    } else if let Some((type_, bytes)) = self.read_property() {
                        if type_ == self.a.incr {
                            self.incr = Some(Vec::new());
                        } else {
                            self.got = Some(bytes);
                            self.finish_drop(true);
                        }
                    }
                }
                Event::PropertyNotify(p) => {
                    if p.atom == self.a.prop && p.state == x11rb::protocol::xproto::Property::NEW_VALUE && self.incr.is_some() {
                        if let Some((_, bytes)) = self.read_property() {
                            if bytes.is_empty() {
                                self.got = self.incr.take();
                                self.finish_drop(true);
                            } else if let Some(buf) = &mut self.incr {
                                buf.extend(bytes);
                            }
                        }
                    }
                }
                Event::SelectionRequest(r) => self.on_selection_request(r),
                Event::ButtonPress(_) if self.drag_on_press => self.start_drag(),
                Event::MotionNotify(m) if self.drag.is_some() => self.moved(m.root_x, m.root_y),
                Event::ButtonRelease(_) if self.drag.is_some() => self.released(),
                Event::Expose(_) => {}
                _ => {}
            }
        }
    }

    fn on_client_message(&mut self, m: x11rb::protocol::xproto::ClientMessageEvent) {
            let d = m.data.as_data32();
            let t = m.type_;
            if t == self.a.enter {
                self.from = d[0];
                let atoms: Vec<Atom> = if d[1] & 1 != 0 {
                    let r = self
                        .conn
                        .get_property(false, d[0], self.a.type_list, AtomEnum::ATOM, 0, 64)
                        .unwrap()
                        .reply()
                        .unwrap();
                    r.value32().map(|v| v.collect()).unwrap_or_default()
                } else {
                    d[2..5].iter().copied().filter(|a| *a != 0).collect()
                };
                self.entered = Some(atoms.into_iter().map(|a| atom_name(&self.conn, a)).collect());
                self.dropped = false;
                self.got = None;
                self.refused = false;
            } else if t == self.a.position {
                self.positions += 1;
                let action = if self.accept { self.a.copy } else { 0 };
                self.send(d[0], d[0], self.a.status, [self.win, u32::from(self.accept), 0, 0, action]);
            } else if t == self.a.leave {
                self.entered = None;
            } else if t == self.a.drop {
                self.dropped = true;
                if let Some(kind) = self.ask_for.clone() {
                    let target = atom(&self.conn, &kind);
                    self.conn
                        .convert_selection(self.win, self.a.selection, target, self.a.prop, d[2])
                        .unwrap();
                    self.conn.flush().unwrap();
                }
            } else if t == self.a.status {
                if let Some(drag) = &mut self.drag {
                    drag.accepted = d[1] & 1 != 0;
                }
            } else if t == self.a.finished {
                self.finished = Some((d[1] & 1 != 0, d[2]));
            }
    }

    fn on_selection_request(&mut self, r: x11rb::protocol::xproto::SelectionRequestEvent) {
            let mut property = r.property;
            if r.target == self.a.targets {
                let atoms = self.kinds_as_atoms();
                self.conn
                    .change_property32(PropMode::REPLACE, r.requestor, r.property, AtomEnum::ATOM, &atoms)
                    .unwrap();
            } else {
                let name = atom_name(&self.conn, r.target);
                self.asked_for.push(name.clone());
                match self.serving.get(&name) {
                    Some(bytes) => {
                        self.conn
                            .change_property8(PropMode::REPLACE, r.requestor, r.property, r.target, bytes)
                            .unwrap();
                    }
                    None => property = 0,
                }
            }
            self.conn
                .send_event(
                    false,
                    r.requestor,
                    EventMask::NO_EVENT,
                    x11rb::protocol::xproto::SelectionNotifyEvent {
                        response_type: SELECTION_NOTIFY_EVENT,
                        sequence: 0,
                        time: r.time,
                        requestor: r.requestor,
                        selection: r.selection,
                        target: r.target,
                        property,
                    },
                )
                .unwrap();
            self.conn.flush().unwrap();
    }

    pub fn every_window(&self) -> Vec<Window> {
        let r = self.conn.query_tree(self.root).unwrap().reply().unwrap();
        r.children
    }

    pub fn pretend_to_drag(&mut self) {
        self.conn.set_selection_owner(self.win, self.a.selection, CURRENT_TIME).unwrap();
        let types = [atom(&self.conn, "text/plain;charset=utf-8")];
        for w in self.every_window() {
            if w == self.win {
                continue;
            }
            self.send(w, w, self.a.enter, [self.win, 5 << 24, types[0], 0, 0]);
            self.send(w, w, self.a.position, [self.win, 0, (200 << 16) | 200, CURRENT_TIME, self.a.copy]);
        }
    }

    pub fn ask_now(&mut self, kind: &str) {
        let target = atom(&self.conn, kind);
        self.refused = false;
        self.got = None;
        self.conn.convert_selection(self.win, self.a.selection, target, self.a.prop, CURRENT_TIME).unwrap();
        self.conn.flush().unwrap();
    }
}

fn settle(plain: &mut Plain, olds: &mut [&mut Old], ms: u64) {
    let end = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < end {
        plain.turn();
        for o in olds.iter_mut() {
            o.pump();
        }
    }
}

fn until(plain: &mut Plain, olds: &mut [&mut Old], what: &str, mut done: impl FnMut(&Plain, &[&mut Old]) -> bool) {
    let end = Instant::now() + Duration::from_secs(10);
    while !done(plain, olds) {
        assert!(Instant::now() < end, "never happened: {what}");
        settle(plain, olds, 10);
    }
}

fn glide(mouse: &mut Mouse, to: (f32, f32), plain: &mut Plain, olds: &mut [&mut Old]) {
    let from = mouse.at;
    let steps = (((to.0 - from.0).hypot(to.1 - from.1)) / 12.0).ceil().max(1.0) as usize;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        mouse.place(from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t);
        settle(plain, olds, 12);
    }
    settle(plain, olds, 200);
}

fn plain_middle(plain: &mut Plain, mouse: &mut Mouse, olds: &mut [&mut Old]) -> (f32, f32) {
    let (w, h) = (mouse.screen.0 as f32, mouse.screen.1 as f32);
    let mut tries = vec![(w / 2.0 + 250.0, h / 2.0), (w / 2.0 - 250.0, h / 2.0)];
    for gy in 1..6 {
        for gx in 1..8 {
            tries.push((w * gx as f32 / 8.0, h * gy as f32 / 6.0));
        }
    }
    for at in tries {
        for nudge in [3.0, 0.0] {
            mouse.place(at.0 + nudge, at.1 + nudge);
            settle(plain, olds, 60);
        }
        if plain.told.pointer.is_some() {
            return at;
        }
    }
    panic!("the pointer never found the plain window");
}

fn display() -> String {
    std::env::var("TRINDLE_OLD_DISPLAY").expect("TRINDLE_OLD_DISPLAY names the older programs' display, such as :2")
}

struct Rig {
    plain: Plain,
    mouse: Mouse,
    plain_at: (f32, f32),
}

fn set_up(aware: bool) -> Option<(Rig, Old)> {
    let run = harness()?;
    let socket = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());
    let mut plain = Plain::open(&socket, "A plain program", (700, 60));
    let mut mouse = plain.mouse();
    let mut old = Old::open(&display(), (40, 40), OLD_SIZE, aware);
    settle(&mut plain, &mut [&mut old], 1200);
    let plain_at = plain_middle(&mut plain, &mut mouse, &mut [&mut old]);
    mouse.place(1.0, 1.0);
    settle(&mut plain, &mut [&mut old], 100);
    Some((Rig { plain, mouse, plain_at }, old))
}

fn old_middle(old: &Old, _size: (u16, u16)) -> (f32, f32) {
    let at = old.at();
    (at.0 as f32 + 40.0, at.1 as f32 + 40.0)
}

#[test]
#[ignore]
fn older_programs_drag_to_a_wayland_program() {
    let Some((mut rig, mut old)) = set_up(false) else { return };
    old.drag_on_press = true;
    old.serving.insert(TEXT.into(), b"hello from an older program".to_vec());
    old.serving.insert("text/uri-list".into(), b"file:///tmp/example.txt\r\n".to_vec());
    rig.plain.told.answer = Some((TEXT_MIME.into(), 1, 1));
    let from = old_middle(&old, OLD_SIZE);
    assert_ne!(old.at(), (0, 0), "the older window was placed");
    rig.mouse.place(from.0, from.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 150);
    glide(&mut rig.mouse, (from.0 + 20.0, from.1 + 10.0), &mut rig.plain, &mut [&mut old]);
    glide(&mut rig.mouse, rig.plain_at, &mut rig.plain, &mut [&mut old]);
    until(&mut rig.plain, &mut [&mut old], "the plain program was told of the drag", |p, _| p.told.held.is_some());
    let kinds = rig.plain.kinds();
    assert!(kinds.contains(&TEXT_MIME.to_string()), "the text is named as Wayland names it: {kinds:?}");
    assert!(kinds.contains(&"text/uri-list".to_string()), "{kinds:?}");
    until(&mut rig.plain, &mut [&mut old], "the older program was told it would be taken", |_, o| o[0].drag.as_ref().is_some_and(|d| d.accepted));
    rig.mouse.button(LEFT_BUTTON, false);
    until(&mut rig.plain, &mut [&mut old], "the plain program was dropped on", |p, _| p.told.dropped.is_some());
    let data = rig.plain.read_while(TEXT_MIME, || old.pump());
    assert_eq!(data, b"hello from an older program");
    rig.plain.finish();
    until(&mut rig.plain, &mut [&mut old], "the older program was told it was taken", |_, o| o[0].finished.is_some());
    assert_eq!(old.finished.map(|f| f.0), Some(true));
    assert_eq!(old.asked_for, vec![TEXT.to_string()], "only what was asked for was read");
}

#[test]
#[ignore]
fn older_programs_are_dragged_to_by_a_wayland_program() {
    let Some((mut rig, mut old)) = set_up(true) else { return };
    old.ask_for = Some("text/uri-list".into());
    rig.plain.told.serving.insert("text/uri-list".into(), b"file:///tmp/example.txt\r\n".to_vec());
    rig.plain.told.serving.insert(TEXT_MIME.into(), b"some words".to_vec());
    let to = old_middle(&old, OLD_SIZE);
    rig.mouse.place(rig.plain_at.0, rig.plain_at.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 100);
    rig.plain.drag(1 | 2);
    settle(&mut rig.plain, &mut [&mut old], 100);
    glide(&mut rig.mouse, to, &mut rig.plain, &mut [&mut old]);
    until(&mut rig.plain, &mut [&mut old], "the older program was told of the drag", |_, o| o[0].entered.is_some() && o[0].positions > 0);
    let kinds = old.entered.clone().unwrap();
    assert!(kinds.contains(&"text/uri-list".to_string()), "{kinds:?}");
    old.ask_now("text/uri-list");
    settle(&mut rig.plain, &mut [&mut old], 300);
    assert!(old.refused && old.got.is_none(), "an older program is given nothing before the let go");
    assert!(rig.plain.told.asked.is_empty(), "the dragging program was not asked for anything yet");
    rig.mouse.button(LEFT_BUTTON, false);
    until(&mut rig.plain, &mut [&mut old], "the older program was given the data", |_, o| o[0].got.is_some());
    assert_eq!(old.got.as_deref(), Some(&b"file:///tmp/example.txt\r\n"[..]));
    until(&mut rig.plain, &mut [&mut old], "the dragging program was told it was finished", |p, _| p.told.finished == 1);
    assert_eq!(rig.plain.told.source_action, 1, "only a copy was ever settled on");
    assert!(rig.plain.told.performed);
}

#[test]
#[ignore]
fn older_programs_are_only_ever_offered_a_copy() {
    let Some((mut rig, mut old)) = set_up(true) else { return };
    old.ask_for = Some("text/uri-list".into());
    rig.plain.told.serving.insert("text/uri-list".into(), b"file:///tmp/example.txt\r\n".to_vec());
    let to = old_middle(&old, OLD_SIZE);
    rig.mouse.place(rig.plain_at.0, rig.plain_at.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 100);
    rig.plain.drag(2);
    settle(&mut rig.plain, &mut [&mut old], 100);
    glide(&mut rig.mouse, to, &mut rig.plain, &mut [&mut old]);
    rig.mouse.button(LEFT_BUTTON, false);
    settle(&mut rig.plain, &mut [&mut old], 600);
    assert!(old.entered.is_none() && !old.dropped, "a move is not offered to an older program");
    assert_eq!(rig.plain.told.cancelled, 1, "the dragging program was told it came to nothing");
    assert_eq!(rig.plain.told.finished, 0);
}

#[test]
#[ignore]
fn older_programs_cannot_start_a_drag_with_no_button_held() {
    let Some((mut rig, mut old)) = set_up(false) else { return };
    old.serving.insert(TEXT.into(), b"secret".to_vec());
    rig.plain.told.answer = Some((TEXT_MIME.into(), 1, 1));
    old.pretend_to_drag();
    rig.mouse.place(rig.plain_at.0, rig.plain_at.1);
    settle(&mut rig.plain, &mut [&mut old], 800);
    assert!(rig.plain.told.held.is_none(), "no drag was shown to the plain program");
    old.ask_now(TEXT);
    settle(&mut rig.plain, &mut [&mut old], 300);
}

#[test]
#[ignore]
fn an_older_programs_drag_let_go_on_a_program_that_refuses_comes_to_nothing() {
    let Some((mut rig, mut old)) = set_up(false) else { return };
    old.drag_on_press = true;
    old.serving.insert(TEXT.into(), b"hello".to_vec());
    rig.plain.told.answer = None;
    let from = old_middle(&old, OLD_SIZE);
    rig.mouse.place(from.0, from.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 150);
    glide(&mut rig.mouse, (from.0 + 20.0, from.1 + 10.0), &mut rig.plain, &mut [&mut old]);
    glide(&mut rig.mouse, rig.plain_at, &mut rig.plain, &mut [&mut old]);
    rig.mouse.button(LEFT_BUTTON, false);
    settle(&mut rig.plain, &mut [&mut old], 800);
    assert!(rig.plain.told.dropped.is_none(), "nothing was dropped on a program that turned it away");
    assert!(old.asked_for.is_empty(), "nothing was read");
}

impl Old {
    pub fn is_move(&self, action: Atom) -> bool {
        action == self.a.move_
    }

    pub fn is_copy(&self, action: Atom) -> bool {
        action == self.a.copy
    }
}

fn move_case(read_it: bool) {
    let Some((mut rig, mut old)) = set_up(false) else { return };
    old.drag_on_press = true;
    old.serving.insert(TEXT.into(), b"hello from an older program".to_vec());
    rig.plain.told.answer = Some((TEXT_MIME.into(), 1 | 2, 2));
    let from = old_middle(&old, OLD_SIZE);
    rig.mouse.place(from.0, from.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 150);
    glide(&mut rig.mouse, (from.0 + 20.0, from.1 + 10.0), &mut rig.plain, &mut [&mut old]);
    glide(&mut rig.mouse, rig.plain_at, &mut rig.plain, &mut [&mut old]);
    until(&mut rig.plain, &mut [&mut old], "the older program was told it would be taken", |_, o| o[0].drag.as_ref().is_some_and(|d| d.accepted));
    rig.mouse.button(LEFT_BUTTON, false);
    until(&mut rig.plain, &mut [&mut old], "the plain program was dropped on", |p, _| p.told.dropped.is_some());
    assert_eq!(rig.plain.told.action, 2, "the plain program settled on a move");
    if read_it {
        let data = rig.plain.read_while(TEXT_MIME, || old.pump());
        assert_eq!(data, b"hello from an older program");
    }
    rig.plain.finish();
    until(&mut rig.plain, &mut [&mut old], "the older program was told it was finished", |_, o| o[0].finished.is_some());
    let (taken, action) = old.finished.unwrap();
    assert!(taken);
    if read_it {
        assert!(old.is_move(action), "everything asked for was read, so it was moved");
    } else {
        assert!(old.is_copy(action), "nothing was read, so it was not told it was moved");
    }
}

#[test]
#[ignore]
fn older_programs_are_told_of_a_move_only_when_every_byte_was_read() {
    move_case(true);
}

#[test]
#[ignore]
fn older_programs_are_told_of_a_copy_when_nothing_was_read() {
    move_case(false);
}

#[test]
#[ignore]
fn nothing_is_read_from_an_older_program_before_the_let_go() {
    let Some((mut rig, mut old)) = set_up(false) else { return };
    old.drag_on_press = true;
    old.serving.insert(TEXT.into(), b"hello from an older program".to_vec());
    rig.plain.told.answer = Some((TEXT_MIME.into(), 1, 1));
    let from = old_middle(&old, OLD_SIZE);
    rig.mouse.place(from.0, from.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 150);
    glide(&mut rig.mouse, (from.0 + 20.0, from.1 + 10.0), &mut rig.plain, &mut [&mut old]);
    glide(&mut rig.mouse, rig.plain_at, &mut rig.plain, &mut [&mut old]);
    until(&mut rig.plain, &mut [&mut old], "the plain program was told of the drag", |p, _| p.told.held.is_some());
    let early = rig.plain.ask_early(TEXT_MIME);
    settle(&mut rig.plain, &mut [&mut old], 500);
    assert!(old.asked_for.is_empty(), "the older program was asked for nothing before the let go");
    assert!(early.try_recv().is_err(), "and the plain program has been given nothing");
    rig.mouse.button(LEFT_BUTTON, false);
    until(&mut rig.plain, &mut [&mut old], "the plain program was dropped on", |p, _| p.told.dropped.is_some());
    let data = rig.plain.wait_for(&early, || old.pump());
    assert_eq!(data, b"hello from an older program", "what was asked for early is given once it is let go");
}

#[test]
#[ignore]
fn a_large_drag_to_an_older_program_arrives_whole() {
    let Some((mut rig, mut old)) = set_up(true) else { return };
    old.ask_for = Some("text/uri-list".into());
    let big: Vec<u8> = (0..300_000u32).map(|i| b'a' + (i % 26) as u8).collect();
    rig.plain.told.serving.insert("text/uri-list".into(), big.clone());
    let to = old_middle(&old, OLD_SIZE);
    rig.mouse.place(rig.plain_at.0, rig.plain_at.1);
    settle(&mut rig.plain, &mut [&mut old], 200);
    rig.mouse.button(LEFT_BUTTON, true);
    settle(&mut rig.plain, &mut [&mut old], 100);
    rig.plain.drag(1);
    settle(&mut rig.plain, &mut [&mut old], 100);
    glide(&mut rig.mouse, to, &mut rig.plain, &mut [&mut old]);
    until(&mut rig.plain, &mut [&mut old], "the older program was told of the drag", |_, o| o[0].entered.is_some() && o[0].positions > 0);
    rig.mouse.button(LEFT_BUTTON, false);
    until(&mut rig.plain, &mut [&mut old], "the older program was given the data", |_, o| o[0].got.is_some());
    let got = old.got.clone().unwrap_or_default();
    let first_difference = got.iter().zip(&big).position(|(a, b)| a != b);
    assert!(got == big, "all 300000 bytes arrived, as sent: got {} bytes, first difference at {first_difference:?}", got.len());
}

#[test]
#[ignore]
fn a_drag_between_two_xwaylands_crosses_only_by_the_pointer() {
    use crate::common::x11_rooms::{ask, Answer};
    let Some(run) = harness() else { return };
    let socket = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());
    let mut plain = Plain::open(&socket, "A mouse", (100, 40));
    let mut mouse = plain.mouse();
    let (_kept, answer) = ask("xdnd-test-box").expect("the compositor's room socket");
    let Answer::Display(n) = answer else { panic!("no Xwayland for the box: {answer:?}") };
    let mut a = Old::open(&display(), (0, 0), (80, 360), false);
    let mut b = Old::open(&format!(":{n}"), (0, 0), (360, 80), true);
    settle(&mut plain, &mut [&mut a, &mut b], 1500);
    a.drag_on_press = true;
    a.serving.insert("text/uri-list".into(), b"file:///tmp/example.txt\r\n".to_vec());
    b.ask_for = Some("text/uri-list".into());
    let (pa, pb) = (a.at(), b.at());
    assert_ne!(pa, pb, "the two windows are in different places");
    let from = (pa.0 as f32 + 40.0, pa.1 as f32 + 20.0);
    let to = (pb.0 as f32 + 20.0, pb.1 as f32 + 40.0);
    mouse.place(from.0, from.1);
    settle(&mut plain, &mut [&mut a, &mut b], 200);
    mouse.button(LEFT_BUTTON, true);
    settle(&mut plain, &mut [&mut a, &mut b], 150);
    glide(&mut mouse, (from.0 + 10.0, from.1 + 10.0), &mut plain, &mut [&mut a, &mut b]);
    glide(&mut mouse, to, &mut plain, &mut [&mut a, &mut b]);
    until(&mut plain, &mut [&mut a, &mut b], "the other older program was told of the drag", |_, o| o[1].entered.is_some() && o[1].positions > 0);
    b.ask_now("text/uri-list");
    settle(&mut plain, &mut [&mut a, &mut b], 300);
    assert!(b.refused && b.got.is_none(), "nothing is given before the let go");
    assert!(a.asked_for.is_empty(), "the dragging older program was asked for nothing yet");
    mouse.button(LEFT_BUTTON, false);
    until(&mut plain, &mut [&mut a, &mut b], "the other older program was given the data", |_, o| o[1].got.is_some());
    assert_eq!(b.got.as_deref(), Some(&b"file:///tmp/example.txt\r\n"[..]));
    until(&mut plain, &mut [&mut a, &mut b], "the dragging older program was told it was taken", |_, o| o[0].finished.is_some());
    assert_eq!(a.finished.map(|f| f.0), Some(true));
    assert!(a.is_copy(a.finished.unwrap().1), "only ever a copy between two Xwaylands");
}
