//! Dragging from a Wayland program onto an older (X11) program's window, through a real Xwayland.

#[cfg(test)]
use crate::rig::*;
use std::time::Duration;

use x11rb::connection::Connection as _;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ClientMessageEvent, ConnectionExt as _, CreateWindowAux, EventMask, PropMode, Window, WindowClass,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;

const KIND: &str = "text/plain;charset=utf-8";
const WORDS: &[u8] = b"Letter to John Smith";

struct Target {
    conn: RustConnection,
    window: Window,

    early: bool,
    entered: bool,
    positions: usize,
    got: Option<Vec<u8>>,
}

fn atom(conn: &RustConnection, name: &str) -> Atom {
    conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom
}

impl Target {
    fn open(display: u32, early: bool) -> Target {
        let (conn, screen) = x11rb::connect(Some(&format!(":{display}"))).unwrap();
        let root = conn.setup().roots[screen].root;
        let window = conn.generate_id().unwrap();
        conn.create_window(
            x11rb::COPY_DEPTH_FROM_PARENT, window, root, 0, 0, 300, 200, 0, WindowClass::INPUT_OUTPUT, 0,
            &CreateWindowAux::new().background_pixel(0x0000_00ff).event_mask(EventMask::PROPERTY_CHANGE),
        )
        .unwrap();
        conn.change_property32(PropMode::REPLACE, window, atom(&conn, "XdndAware"), AtomEnum::ATOM, &[5]).unwrap();
        conn.map_window(window).unwrap();
        conn.flush().unwrap();
        Target { conn, window, early, entered: false, positions: 0, got: None }
    }

    fn ask(&self, time: u32) {
        let (selection, kind) = (atom(&self.conn, "XdndSelection"), atom(&self.conn, KIND));
        self.conn.convert_selection(self.window, selection, kind, atom(&self.conn, "TRINDLE_DROP"), time).unwrap();
        self.conn.flush().unwrap();
    }

    fn tell(&self, to: Window, kind: &str, data: [u32; 5]) {
        let event = ClientMessageEvent::new(32, to, atom(&self.conn, kind), data);
        self.conn.send_event(false, to, EventMask::NO_EVENT, event).unwrap();
        self.conn.flush().unwrap();
    }

    fn hear(&mut self) {
        while let Some(event) = self.conn.poll_for_event().unwrap() {
            match event {
                Event::ClientMessage(m) => {
                    let d = m.data.as_data32();
                    let name = self.conn.get_atom_name(m.type_).unwrap().reply().unwrap().name;
                    match name.as_slice() {
                        b"XdndEnter" => {
                            self.entered = true;
                            if self.early {
                                self.ask(x11rb::CURRENT_TIME);
                            }
                        }
                        b"XdndPosition" => {
                            self.positions += 1;
                            let copy = atom(&self.conn, "XdndActionCopy");
                            self.tell(d[0], "XdndStatus", [self.window, 1, 0, 0, copy]);
                        }
                        b"XdndDrop" => self.ask(d[2]),
                        _ => {}
                    }
                }
                Event::SelectionNotify(n) if n.property != x11rb::NONE => {
                    let reply = self.conn.get_property(true, self.window, n.property, AtomEnum::ANY, 0, 1024).unwrap().reply().unwrap();
                    self.got = Some(reply.value);
                    let source = self.conn.get_selection_owner(atom(&self.conn, "XdndSelection")).unwrap().reply().unwrap().owner;
                    let copy = atom(&self.conn, "XdndActionCopy");
                    self.tell(source, "XdndFinished", [self.window, 1, copy, 0, 0]);
                }
                _ => {}
            }
        }
    }
}

fn drag_begun(rig: &mut Rig, dir: &std::path::Path, early: bool) -> (Target, usize) {
    let who = rig.join(ClientState::default());
    let surface = rig.window(who, false);
    let display = crate::games::older::start(rig, dir);
    let target = Target::open(display, early);
    let old = |r: &Rig| r.state.space.elements().find(|w| w.0.x11_surface().is_some() && w.wl_surface().is_some()).cloned();
    assert!(crate::games::until_drawn(rig, Duration::from_secs(10), |r| old(r).is_some()), "the older program's window never showed");
    let server = rig.server_side(who, &surface);
    let wayland = rig.state.window_for_surface(&server).expect("the Wayland window");
    rig.state.space.map_element(wayland, (0, 0), true);
    let old = old(rig).unwrap();
    rig.state.space.map_element(old, (700, 400), false);
    rig.pump();
    rig.move_by_hand(100.0 / 1280.0, 100.0 / 800.0);
    rig.button_by_hand_only(BTN_LEFT, true);
    rig.drag_from(who, &surface, KIND, WORDS);
    (target, who)
}

#[test]
fn a_drag_from_a_wayland_program_is_let_go_on_an_older_programs_window() {
    alone("rig::tests::older_drag::a_drag_from_a_wayland_program_is_let_go_on_an_older_programs_window", |dir| {
        let mut rig = Rig::new(dir);
        let (mut target, who) = drag_begun(&mut rig, dir, false);
        for step in 1..=8 {
            let f = step as f64 / 8.0;
            rig.move_by_hand((100.0 + f * 750.0) / 1280.0, (100.0 + f * 400.0) / 800.0);
            target.hear();
        }
        let told = rig.until(Duration::from_secs(5), |_| {
            target.hear();
            target.positions > 0
        });
        assert!(target.entered && told, "the older program was never offered the drag");
        assert!(target.got.is_none(), "the words were given before the let go");
        rig.button_by_hand_only(BTN_LEFT, false);
        let done = rig.until(Duration::from_secs(5), |r| {
            target.hear();
            r.dragged_said(who).contains(&"finished")
        });
        assert_eq!(target.got.as_deref(), Some(WORDS), "the older program was not given the words");
        assert!(done, "the dragging program never heard the drop was taken: {:?}", rig.dragged_said(who));
    });
}

#[test]
fn an_older_program_that_asks_before_the_let_go_is_given_nothing() {
    alone("rig::tests::older_drag::an_older_program_that_asks_before_the_let_go_is_given_nothing", |dir| {
        let mut rig = Rig::new(dir);
        let (mut target, who) = drag_begun(&mut rig, dir, true);
        rig.move_by_hand(850.0 / 1280.0, 500.0 / 800.0);
        let told = rig.until(Duration::from_secs(5), |_| {
            target.hear();
            target.positions > 0
        });
        assert!(target.entered && told, "the older program was never offered the drag");
        rig.until(Duration::from_millis(500), |_| {
            target.hear();
            false
        });
        assert!(target.got.is_none(), "an older program was given the words before the let go");
        assert!(!rig.dragged_said(who).contains(&"send"), "the dragging program was asked for its words early");
    });
}

struct Dragger {
    conn: RustConnection,
    window: Window,
    root: Window,
    over: Option<Window>,
    yes: Option<bool>,
    finished: Option<bool>,
}

impl Dragger {
    fn open(display: u32) -> Dragger {
        let (conn, screen) = x11rb::connect(Some(&format!(":{display}"))).unwrap();
        let root = conn.setup().roots[screen].root;
        let window = conn.generate_id().unwrap();
        conn.create_window(
            x11rb::COPY_DEPTH_FROM_PARENT, window, root, 0, 0, 300, 200, 0, WindowClass::INPUT_OUTPUT, 0,
            &CreateWindowAux::new().background_pixel(0x00ff_0000),
        )
        .unwrap();
        conn.map_window(window).unwrap();
        conn.flush().unwrap();
        Dragger { conn, window, root, over: None, yes: None, finished: None }
    }

    fn take(&self) {
        self.conn.set_selection_owner(self.window, atom(&self.conn, "XdndSelection"), x11rb::CURRENT_TIME).unwrap();
        self.conn.flush().unwrap();
    }

    fn tell(&self, to: Window, kind: &str, data: [u32; 5]) {
        let event = ClientMessageEvent::new(32, to, atom(&self.conn, kind), data);
        self.conn.send_event(false, to, EventMask::NO_EVENT, event).unwrap();
        self.conn.flush().unwrap();
    }

    fn move_over(&mut self) {
        let at = self.conn.query_pointer(self.root).unwrap().reply().unwrap();
        let aware = atom(&self.conn, "XdndAware");
        let takes = at.child != x11rb::NONE
            && self.conn.get_property(false, at.child, aware, AtomEnum::ATOM, 0, 1).unwrap().reply().is_ok_and(|r| r.value_len > 0);
        if !takes {
            return;
        }
        if self.over != Some(at.child) {
            self.tell(at.child, "XdndEnter", [self.window, 5 << 24, atom(&self.conn, KIND), 0, 0]);
            self.over = Some(at.child);
        }
        let place = ((at.root_x.max(0) as u32) << 16) | at.root_y.max(0) as u32;
        self.tell(at.child, "XdndPosition", [self.window, 0, place, 0, atom(&self.conn, "XdndActionCopy")]);
    }

    fn let_go(&self) {
        if let Some(to) = self.over {
            self.tell(to, "XdndDrop", [self.window, 0, x11rb::CURRENT_TIME, 0, 0]);
        }
    }

    fn hear(&mut self) {
        while let Some(event) = self.conn.poll_for_event().unwrap() {
            match event {
                Event::ClientMessage(m) => {
                    let d = m.data.as_data32();
                    match self.conn.get_atom_name(m.type_).unwrap().reply().unwrap().name.as_slice() {
                        b"XdndStatus" => self.yes = Some(d[1] & 1 != 0),
                        b"XdndFinished" => self.finished = Some(d[1] & 1 != 0),
                        _ => {}
                    }
                }
                Event::SelectionRequest(r) => {
                    self.conn.change_property8(PropMode::REPLACE, r.requestor, r.property, r.target, WORDS).unwrap();
                    let told = x11rb::protocol::xproto::SelectionNotifyEvent {
                        response_type: x11rb::protocol::xproto::SELECTION_NOTIFY_EVENT,
                        sequence: 0,
                        time: r.time,
                        requestor: r.requestor,
                        selection: r.selection,
                        target: r.target,
                        property: r.property,
                    };
                    self.conn.send_event(false, r.requestor, EventMask::NO_EVENT, told).unwrap();
                    self.conn.flush().unwrap();
                }
                _ => {}
            }
        }
    }
}

fn older_drags(rig: &mut Rig, dir: &std::path::Path) -> (Dragger, usize) {
    let who = rig.join(ClientState::default());
    let surface = rig.window(who, false);
    rig.take_drops(who, KIND);
    let display = crate::games::older::start(rig, dir);
    let dragger = Dragger::open(display);
    let old = |r: &Rig| r.state.space.elements().find(|w| w.0.x11_surface().is_some() && w.wl_surface().is_some()).cloned();
    assert!(crate::games::until_drawn(rig, Duration::from_secs(10), |r| old(r).is_some()), "the older program's window never showed");
    let server = rig.server_side(who, &surface);
    let wayland = rig.state.window_for_surface(&server).expect("the Wayland window");
    rig.state.space.map_element(wayland, (0, 0), false);
    let old = old(rig).unwrap();
    rig.state.space.map_element(old, (700, 400), true);
    rig.pump();
    rig.move_by_hand(850.0 / 1280.0, 500.0 / 800.0);
    (dragger, who)
}

fn carried_over(rig: &mut Rig, dragger: &mut Dragger) {
    for step in 1..=8 {
        let f = step as f64 / 8.0;
        rig.move_by_hand((850.0 - f * 700.0) / 1280.0, (500.0 - f * 400.0) / 800.0);
        rig.until(Duration::from_millis(30), |_| false);
        dragger.hear();
        dragger.move_over();
    }
}

#[test]
fn an_older_programs_drag_is_let_go_on_a_wayland_program() {
    alone("rig::tests::older_drag::an_older_programs_drag_is_let_go_on_a_wayland_program", |dir| {
        let mut rig = Rig::new(dir);
        let (mut dragger, who) = older_drags(&mut rig, dir);
        rig.button_by_hand_only(BTN_LEFT, true);
        dragger.take();
        rig.until(Duration::from_millis(300), |_| false);
        carried_over(&mut rig, &mut dragger);
        let yes = rig.until(Duration::from_secs(5), |_| {
            dragger.hear();
            dragger.yes.is_some()
        });
        assert!(yes && dragger.over.is_some(), "the older program never heard the Wayland program take the drag");
        assert_eq!(dragger.yes, Some(true), "the Wayland program's yes did not reach the older program");
        rig.button_by_hand_only(BTN_LEFT, false);
        dragger.let_go();
        let mut got = Vec::new();
        let read = rig.until(Duration::from_secs(5), |r| {
            dragger.hear();
            r.read_drop(who, &mut got)
        });
        assert!(read, "nothing was let go on the Wayland program");
        assert_eq!(got, WORDS, "the Wayland program was not given the words");
        let finished = rig.until(Duration::from_secs(5), |_| {
            dragger.hear();
            dragger.finished.is_some()
        });
        assert!(finished, "the older program never heard the drop was finished");
        assert_eq!(dragger.finished, Some(true), "the older program heard the drop came to nothing");
    });
}

#[test]
fn an_older_program_taking_the_drag_selection_with_no_button_held_drags_nothing() {
    alone("rig::tests::older_drag::an_older_program_taking_the_drag_selection_with_no_button_held_drags_nothing", |dir| {
        let mut rig = Rig::new(dir);
        let (mut dragger, who) = older_drags(&mut rig, dir);
        dragger.take();
        rig.until(Duration::from_millis(300), |_| false);
        carried_over(&mut rig, &mut dragger);
        rig.until(Duration::from_millis(300), |_| false);
        assert!(dragger.over.is_none(), "a window took a drag no button held");
        let mut got = Vec::new();
        assert!(!rig.read_drop(who, &mut got), "something was let go on the Wayland program");
    });
}
