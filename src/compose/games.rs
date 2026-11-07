//! Games: what the compositor owes a game, tested the way a game asks for it,
//! over Wayland and through a real Xwayland.
// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::os::fd::{AsFd, FromRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::utils::{Logical, Point};
use wayland_client::protocol::{wl_buffer, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat, wl_shm, wl_shm_pool, wl_surface};
use wayland_client::{delegate_noop, Connection, Dispatch, EventQueue, QueueHandle, WEnum};
use wayland_protocols::wp::idle_inhibit::zv1::client::{zwp_idle_inhibit_manager_v1::ZwpIdleInhibitManagerV1, zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1};
use wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::client::{
    zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1,
    zwp_keyboard_shortcuts_inhibitor_v1::{self, ZwpKeyboardShortcutsInhibitorV1},
};
use wayland_protocols::wp::pointer_constraints::zv1::client::{
    zwp_confined_pointer_v1::{self, ZwpConfinedPointerV1},
    zwp_locked_pointer_v1::{self, ZwpLockedPointerV1},
    zwp_pointer_constraints_v1::{Lifetime, ZwpPointerConstraintsV1},
};
use wayland_protocols::wp::relative_pointer::zv1::client::{
    zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1,
    zwp_relative_pointer_v1::{self, ZwpRelativePointerV1},
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use crate::rig::{alone, Rig};
use crate::state::ClientState;
use crate::virtual_input::{Did, Maker, Motion};

const ALT: u32 = 56;
const TAB: u32 = 15;

#[derive(Default)]
struct Player {
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,
    wm: Option<xdg_wm_base::XdgWmBase>,
    seat: Option<wl_seat::WlSeat>,
    pointer: Option<wl_pointer::WlPointer>,
    constraints: Option<ZwpPointerConstraintsV1>,
    relative: Option<ZwpRelativePointerManagerV1>,
    keep_keys: Option<ZwpKeyboardShortcutsInhibitManagerV1>,
    awake: Option<ZwpIdleInhibitManagerV1>,
    entered: Option<u32>,
    held: Option<bool>,
    keys_kept: Option<bool>,
    moved: Vec<(f64, f64)>,
    keys: Vec<u32>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for Player {
    fn event(p: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        let wl_registry::Event::Global { name, interface, version } = event else { return };
        match interface.as_str() {
            "wl_compositor" => p.compositor = Some(registry.bind(name, version.min(5), qh, ())),
            "wl_shm" => p.shm = Some(registry.bind(name, 1, qh, ())),
            "xdg_wm_base" => p.wm = Some(registry.bind(name, 1, qh, ())),
            "wl_seat" => p.seat = Some(registry.bind(name, version.min(7), qh, ())),
            "zwp_pointer_constraints_v1" => p.constraints = Some(registry.bind(name, 1, qh, ())),
            "zwp_relative_pointer_manager_v1" => p.relative = Some(registry.bind(name, 1, qh, ())),
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => p.keep_keys = Some(registry.bind(name, 1, qh, ())),
            "zwp_idle_inhibit_manager_v1" => p.awake = Some(registry.bind(name, 1, qh, ())),
            _ => {}
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for Player {
    fn event(p: &mut Self, seat: &wl_seat::WlSeat, event: wl_seat::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_seat::Event::Capabilities { capabilities: WEnum::Value(c) } = event {
            if c.contains(wl_seat::Capability::Pointer) && p.pointer.is_none() {
                p.pointer = Some(seat.get_pointer(qh, ()));
            }
            if c.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for Player {
    fn event(p: &mut Self, _: &wl_pointer::WlPointer, event: wl_pointer::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let wl_pointer::Event::Enter { serial, .. } = event {
            p.entered = Some(serial);
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for Player {
    fn event(p: &mut Self, _: &wl_keyboard::WlKeyboard, event: wl_keyboard::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let wl_keyboard::Event::Key { key, state: WEnum::Value(wl_keyboard::KeyState::Pressed), .. } = event {
            p.keys.push(key);
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for Player {
    fn event(_: &mut Self, wm: &xdg_wm_base::XdgWmBase, event: xdg_wm_base::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for Player {
    fn event(_: &mut Self, surface: &xdg_surface::XdgSurface, event: xdg_surface::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let xdg_surface::Event::Configure { serial } = event {
            surface.ack_configure(serial);
        }
    }
}

impl Dispatch<ZwpLockedPointerV1, ()> for Player {
    fn event(p: &mut Self, _: &ZwpLockedPointerV1, event: zwp_locked_pointer_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwp_locked_pointer_v1::Event::Locked => p.held = Some(true),
            zwp_locked_pointer_v1::Event::Unlocked => p.held = Some(false),
            _ => {}
        }
    }
}

impl Dispatch<ZwpConfinedPointerV1, ()> for Player {
    fn event(p: &mut Self, _: &ZwpConfinedPointerV1, event: zwp_confined_pointer_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwp_confined_pointer_v1::Event::Confined => p.held = Some(true),
            zwp_confined_pointer_v1::Event::Unconfined => p.held = Some(false),
            _ => {}
        }
    }
}

impl Dispatch<ZwpRelativePointerV1, ()> for Player {
    fn event(p: &mut Self, _: &ZwpRelativePointerV1, event: zwp_relative_pointer_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let zwp_relative_pointer_v1::Event::RelativeMotion { dx_unaccel, dy_unaccel, .. } = event {
            p.moved.push((dx_unaccel, dy_unaccel));
        }
    }
}

impl Dispatch<ZwpKeyboardShortcutsInhibitorV1, ()> for Player {
    fn event(p: &mut Self, _: &ZwpKeyboardShortcutsInhibitorV1, event: zwp_keyboard_shortcuts_inhibitor_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwp_keyboard_shortcuts_inhibitor_v1::Event::Active => p.keys_kept = Some(true),
            zwp_keyboard_shortcuts_inhibitor_v1::Event::Inactive => p.keys_kept = Some(false),
            _ => {}
        }
    }
}

delegate_noop!(Player: ignore wl_compositor::WlCompositor);
delegate_noop!(Player: ignore wl_surface::WlSurface);
delegate_noop!(Player: ignore wl_shm::WlShm);
delegate_noop!(Player: ignore wl_shm_pool::WlShmPool);
delegate_noop!(Player: ignore wl_buffer::WlBuffer);
delegate_noop!(Player: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(Player: ignore ZwpPointerConstraintsV1);
delegate_noop!(Player: ignore ZwpRelativePointerManagerV1);
delegate_noop!(Player: ignore ZwpKeyboardShortcutsInhibitManagerV1);
delegate_noop!(Player: ignore ZwpIdleInhibitManagerV1);
delegate_noop!(Player: ignore ZwpIdleInhibitorV1);

struct Game {
    conn: Connection,
    queue: EventQueue<Player>,
    p: Player,
    surface: Option<wl_surface::WlSurface>,
}

impl Game {
    fn join(rig: &mut Rig) -> Game {
        let (server_end, client_end) = UnixStream::pair().unwrap();
        rig.state.display_handle.insert_client(server_end, Arc::new(ClientState::default())).unwrap();
        let conn = Connection::from_socket(client_end).unwrap();
        let queue = conn.new_event_queue();
        conn.display().get_registry(&queue.handle(), ());
        let mut game = Game { conn, queue, p: Player::default(), surface: None };
        game.turn(rig);
        game.turn(rig);
        game
    }

    fn turn(&mut self, rig: &mut Rig) {
        for _ in 0..4 {
            let _ = self.conn.flush();
            rig.pump();
            if let Some(guard) = self.conn.prepare_read() {
                let _ = guard.read();
            }
            let _ = self.queue.dispatch_pending(&mut self.p);
        }
    }

    fn picture(&self, w: i32, h: i32) -> wl_buffer::WlBuffer {
        let qh = self.queue.handle();
        // SAFETY: memfd_create is handed a name that ends in NUL and gives
        // back a new file that belongs to nothing else, or -1.
        let fd = unsafe { libc::memfd_create(c"trindle-game".as_ptr(), libc::MFD_CLOEXEC) };
        assert!(fd >= 0);
        // SAFETY: the number was just made by memfd_create and is owned here alone.
        let file = std::fs::File::from(unsafe { OwnedFd::from_raw_fd(fd) });
        file.set_len((w * h * 4) as u64).unwrap();
        let pool = self.p.shm.as_ref().unwrap().create_pool(file.as_fd(), w * h * 4, &qh, ());
        pool.create_buffer(0, w, h, w * 4, wl_shm::Format::Argb8888, &qh, ())
    }

    fn full_screen(&mut self, rig: &mut Rig) -> wl_surface::WlSurface {
        let qh = self.queue.handle();
        let surface = self.p.compositor.as_ref().unwrap().create_surface(&qh, ());
        let xdg = self.p.wm.as_ref().unwrap().get_xdg_surface(&surface, &qh, ());
        let top = xdg.get_toplevel(&qh, ());
        top.set_fullscreen(None);
        surface.commit();
        self.turn(rig);
        surface.attach(Some(&self.picture(1280, 800)), 0, 0);
        surface.commit();
        self.turn(rig);
        self.surface = Some(surface.clone());
        surface
    }

    fn lock(&mut self, rig: &mut Rig) {
        let qh = self.queue.handle();
        let (surface, pointer) = (self.surface.clone().unwrap(), self.p.pointer.clone().unwrap());
        self.p.relative.as_ref().unwrap().get_relative_pointer(&pointer, &qh, ());
        self.p.constraints.as_ref().unwrap().lock_pointer(&surface, &pointer, None, Lifetime::Persistent, &qh, ());
        self.turn(rig);
    }

    fn hide_pointer(&mut self, rig: &mut Rig) {
        let serial = self.p.entered.expect("the pointer never entered the game");
        self.p.pointer.as_ref().unwrap().set_cursor(serial, None, 0, 0);
        self.turn(rig);
    }

    fn confine(&mut self, rig: &mut Rig) {
        let qh = self.queue.handle();
        let (surface, pointer) = (self.surface.clone().unwrap(), self.p.pointer.clone().unwrap());
        self.p.constraints.as_ref().unwrap().confine_pointer(&surface, &pointer, None, Lifetime::Persistent, &qh, ());
        self.turn(rig);
    }
}

fn nudge(rig: &mut Rig, game: &mut Game, dx: f64, dy: f64) {
    rig.state.virtual_input(Maker::default(), Did::Motion(Motion { time: 5000, dx, dy }));
    game.turn(rig);
}

fn at(rig: &Rig) -> Point<f64, Logical> {
    rig.state.pointer.current_location()
}

fn alt_tab(rig: &mut Rig, game: &mut Game) {
    rig.key_by_hand_only(ALT, true);
    rig.key_by_hand(TAB);
    rig.key_by_hand_only(ALT, false);
    game.turn(rig);
}

fn second_screen(rig: &mut Rig) -> Output {
    let output = Output::new(
        "rig-2".into(),
        PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
    );
    let mode = Mode { size: (1280, 800).into(), refresh: crate::render::ASSUMED_REFRESH };
    output.change_current_state(Some(mode), None, None, Some((1280, 0).into()));
    output.set_preferred(mode);
    output.create_global::<crate::state::AnvilState<crate::rig::RigData>>(&rig.state.display_handle);
    rig.state.space.map_output(&output, (1280, 0));
    output
}

fn until_drawn(rig: &mut Rig, most: Duration, mut done: impl FnMut(&mut Rig) -> bool) -> bool {
    rig.until(most, |r| {
        let output = r.output.clone();
        for w in r.state.space.elements() {
            w.0.send_frame(&output, Duration::ZERO, Some(Duration::ZERO), |_, _| Some(output.clone()));
        }
        done(r)
    })
}

fn drawn(window: &crate::shell::WindowElement) -> Option<smithay::utils::Size<i32, Logical>> {
    window.wl_surface().and_then(|s| smithay::backend::renderer::utils::with_renderer_surface_state(&s, |s| s.surface_size()).flatten())
}

fn filling(rig: &Rig) -> Option<crate::shell::WindowElement> {
    rig.output.user_data().get::<crate::shell::FullscreenSurface>().and_then(|f| f.get())
}

#[test]
fn a_game_holds_the_pointer_only_while_it_has_the_keyboard_and_gets_it_back_after_alt_tab() {
    alone("games::a_game_holds_the_pointer_only_while_it_has_the_keyboard_and_gets_it_back_after_alt_tab", |dir| {
        let mut rig = Rig::new(dir);
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        let mut game = Game::join(&mut rig);
        game.full_screen(&mut rig);
        rig.move_by_hand(0.5, 0.5);
        game.lock(&mut rig);
        assert_eq!(game.p.held, Some(true), "the lock was not given");

        let still = at(&rig);
        nudge(&mut rig, &mut game, 500.0, 300.0);
        assert_eq!(at(&rig), still, "a locked pointer moved");
        assert_eq!(game.p.moved.last(), Some(&(500.0, 300.0)), "the game was not told how far the mouse moved");
        rig.move_by_hand(0.9, 0.1);
        game.turn(&mut rig);
        assert_eq!(at(&rig), still, "a pointer placed by a tablet escaped the lock");

        alt_tab(&mut rig, &mut game);
        assert!(filling(&rig).is_none(), "the window brought in front is hidden behind the game");
        assert_eq!(game.p.held, Some(false), "the lock outlived the keyboard going to another window");
        nudge(&mut rig, &mut game, 40.0, 0.0);
        assert_ne!(at(&rig), still, "the pointer stayed locked with another window in front");

        alt_tab(&mut rig, &mut game);
        assert!(filling(&rig).is_some(), "the game did not fill its screen again");
        nudge(&mut rig, &mut game, 1.0, 0.0);
        assert_eq!(game.p.held, Some(true), "the lock did not come back with the keyboard");
    });
}

#[test]
fn the_ring_opened_over_a_game_that_holds_the_pointer_gets_the_pointer() {
    alone("games::the_ring_opened_over_a_game_that_holds_the_pointer_gets_the_pointer", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let mut game = Game::join(&mut rig);
        let surface = game.full_screen(&mut rig);
        let qh = game.queue.handle();
        game.p.keep_keys.as_ref().unwrap().inhibit_shortcuts(&surface, game.p.seat.as_ref().unwrap(), &qh, ());
        rig.move_by_hand(0.5, 0.5);
        game.lock(&mut rig);
        assert_eq!(game.p.held, Some(true), "the lock was not given");
        assert_eq!(game.p.keys_kept, Some(true), "the game was not given its shortcuts");
        nudge(&mut rig, &mut game, 1.0, 0.0);
        game.hide_pointer(&mut rig);
        assert!(matches!(rig.state.cursor_status, smithay::input::pointer::CursorImageStatus::Hidden), "the game's hidden pointer was not taken");
        let mut ring = rig.kept_ring();
        crate::rig::kept_ring::beat(&mut ring.pulse);
        rig.key_by_hand(crate::rig::KEY_F5);
        rig.panel(ring.who, "trindle-ring");
        game.turn(&mut rig);
        assert!(rig.has_keyboard(ring.who), "the ring was not given the keyboard");
        assert_eq!(game.p.held, Some(false), "the game kept the pointer with the ring over it");
        let still = at(&rig);
        nudge(&mut rig, &mut game, 40.0, 0.0);
        assert_ne!(at(&rig), still, "the pointer stayed locked with the ring over the game");
        assert!(!matches!(rig.state.cursor_status, smithay::input::pointer::CursorImageStatus::Hidden), "the game's hidden pointer stayed hidden over the ring");
    });
}

#[test]
fn a_held_pointer_reaches_the_edge_of_the_game_and_never_crosses_to_the_other_screen() {
    alone("games::a_held_pointer_reaches_the_edge_of_the_game_and_never_crosses_to_the_other_screen", |dir| {
        let mut rig = Rig::new(dir);
        let _right = second_screen(&mut rig);
        let mut game = Game::join(&mut rig);
        game.full_screen(&mut rig);
        rig.move_by_hand(0.5, 0.5);
        game.confine(&mut rig);
        assert_eq!(game.p.held, Some(true), "the confinement was not given");
        let start = at(&rig);
        nudge(&mut rig, &mut game, 5000.0, 0.0);
        assert_eq!(at(&rig), Point::from((1279.0, start.y)), "the pointer did not reach the right edge, or left the game");
        nudge(&mut rig, &mut game, -9000.0, -9000.0);
        assert_eq!(at(&rig), Point::from((0.0, 0.0)), "the pointer did not reach the top left corner");
        nudge(&mut rig, &mut game, 30.0, 0.0);
        assert_eq!(at(&rig), Point::from((30.0, 0.0)), "the pointer stuck at the edge");
    });
}

#[test]
fn a_locked_pointer_stays_put_on_two_screens() {
    alone("games::a_locked_pointer_stays_put_on_two_screens", |dir| {
        let mut rig = Rig::new(dir);
        let _right = second_screen(&mut rig);
        let mut game = Game::join(&mut rig);
        game.full_screen(&mut rig);
        rig.move_by_hand(0.9, 0.5);
        game.lock(&mut rig);
        assert_eq!(game.p.held, Some(true));
        let still = at(&rig);
        for _ in 0..5 {
            nudge(&mut rig, &mut game, 400.0, 0.0);
        }
        assert_eq!(at(&rig), still, "the locked pointer crossed to the other screen");
    });
}

#[test]
fn shortcuts_are_kept_back_only_for_the_window_with_the_keyboard() {
    alone("games::shortcuts_are_kept_back_only_for_the_window_with_the_keyboard", |dir| {
        let mut rig = Rig::new(dir);
        let mut game = Game::join(&mut rig);
        let surface = game.full_screen(&mut rig);
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        alt_tab(&mut rig, &mut game);
        let qh = game.queue.handle();
        game.p.keep_keys.as_ref().unwrap().inhibit_shortcuts(&surface, game.p.seat.as_ref().unwrap(), &qh, ());
        game.turn(&mut rig);
        assert_eq!(game.p.keys_kept, Some(true), "the game was not given its shortcuts");
        rig.move_by_hand(0.5, 0.5);

        let front = rig.state.space.elements().last().cloned();
        alt_tab(&mut rig, &mut game);
        assert!(game.p.keys.contains(&TAB), "Alt-Tab did not reach the game that keeps shortcuts");
        assert_eq!(rig.state.space.elements().last().cloned(), front, "Alt-Tab switched away from a game that keeps shortcuts");

        let game_window = front.unwrap();
        let other_window = rig.state.space.elements().find(|w| w != &&game_window).cloned().unwrap();
        rig.state.focus_window(&other_window);
        game.p.keys.clear();
        alt_tab(&mut rig, &mut game);
        assert!(!game.p.keys.contains(&TAB), "the game heard keys while another window had the keyboard");
        assert_eq!(rig.state.space.elements().last(), Some(&game_window), "Alt-Tab was kept back for a window without the keyboard");
    });
}

#[test]
fn a_game_asking_keeps_the_screen_awake_only_while_it_shows() {
    alone("games::a_game_asking_keeps_the_screen_awake_only_while_it_shows", |dir| {
        let mut rig = Rig::new(dir);
        let mut game = Game::join(&mut rig);
        let surface = game.full_screen(&mut rig);
        let qh = game.queue.handle();
        game.p.awake.as_ref().unwrap().create_inhibitor(&surface, &qh, ());
        game.turn(&mut rig);
        rig.state.refresh_idle_inhibit();
        assert!(rig.state.idle_notifier_state.is_inhibited(), "the screen may sleep under a game that asked it not to");
        let server = rig.state.space.elements().last().and_then(|w| w.wl_surface().map(|s| s.into_owned())).unwrap();
        rig.state.set_window_minimized(&server, true);
        rig.state.refresh_idle_inhibit();
        assert!(!rig.state.idle_notifier_state.is_inhibited(), "a minimised game kept the screen awake");
    });
}

mod older {

    use super::*;
    use x11rb::connection::Connection as _;
    use x11rb::protocol::randr::ConnectionExt as _;
    use x11rb::protocol::xproto::{
        AtomEnum, ClientMessageEvent, ConnectionExt as _, CreateWindowAux, EventMask, PropMode, Window, WindowClass,
    };
    use x11rb::rust_connection::RustConnection;
    use x11rb::wrapper::ConnectionExt as _;

    pub struct Older {
        pub conn: RustConnection,
        pub root: Window,
        pub window: Window,
    }

    fn atom(conn: &RustConnection, name: &str) -> u32 {
        conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom
    }

    pub fn start(rig: &mut Rig, dir: &Path) -> u32 {
        std::env::set_var("TRINDLE_XWAYLAND_LATER", dir.join("no-stand-in"));
        rig.state.start_xwayland();
        assert!(rig.until(Duration::from_secs(30), |r| r.state.xdisplay.is_some()), "Xwayland did not start");
        rig.state.xdisplay.unwrap()
    }

    impl Older {
        pub fn open(display: u32) -> Older {
            let (conn, screen) = x11rb::connect(Some(&format!(":{display}"))).unwrap();
            let root = conn.setup().roots[screen].root;
            let window = conn.generate_id().unwrap();
            conn.create_window(
                x11rb::COPY_DEPTH_FROM_PARENT, window, root, 0, 0, 640, 480, 0, WindowClass::INPUT_OUTPUT, 0,
                &CreateWindowAux::new().background_pixel(0x00ff_0000).event_mask(EventMask::STRUCTURE_NOTIFY | EventMask::PROPERTY_CHANGE),
            )
            .unwrap();
            let full = atom(&conn, "_NET_WM_STATE_FULLSCREEN");
            conn.change_property32(PropMode::REPLACE, window, atom(&conn, "_NET_WM_STATE"), AtomEnum::ATOM, &[full]).unwrap();
            conn.map_window(window).unwrap();
            conn.flush().unwrap();
            Older { conn, root, window }
        }

        pub fn size(&self) -> (u16, u16) {
            let g = self.conn.get_geometry(self.window).unwrap().reply().unwrap();
            (g.width, g.height)
        }

        pub fn wm_state(&self) -> Option<u32> {
            let name = atom(&self.conn, "WM_STATE");
            self.conn.get_property(false, self.window, name, name, 0, 2).unwrap().reply().ok()?.value32()?.next()
        }

        pub fn net_state(&self) -> Vec<u32> {
            let name = atom(&self.conn, "_NET_WM_STATE");
            let reply = self.conn.get_property(false, self.window, name, AtomEnum::ATOM, 0, 32).unwrap().reply().unwrap();
            reply.value32().map(|v| v.collect()).unwrap_or_default()
        }

        pub fn has(&self, state: &str) -> bool {
            self.net_state().contains(&atom(&self.conn, state))
        }

        pub fn helper(display: u32) -> Older {
            let (conn, screen) = x11rb::connect(Some(&format!(":{display}"))).unwrap();
            let root = conn.setup().roots[screen].root;
            let window = conn.generate_id().unwrap();
            conn.create_window(
                x11rb::COPY_DEPTH_FROM_PARENT, window, root, 0, 0, 10, 10, 0, WindowClass::INPUT_OUTPUT, 0,
                &CreateWindowAux::new().background_pixel(0x0000_ff00).override_redirect(1),
            )
            .unwrap();
            conn.map_window(window).unwrap();
            conn.flush().unwrap();
            Older { conn, root, window }
        }

        pub fn iconify(&self) {
            let event = ClientMessageEvent::new(32, self.window, atom(&self.conn, "WM_CHANGE_STATE"), [3u32, 0, 0, 0, 0]);
            self.conn
                .send_event(false, self.root, EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY, event)
                .unwrap();
            self.conn.flush().unwrap();
        }

        pub fn set_mode(&self, w: u16, h: u16) -> bool {
            let res = self.conn.randr_get_screen_resources(self.root).unwrap().reply().unwrap();
            let Some(mode) = res.modes.iter().find(|m| m.width == w && m.height == h) else { return false };
            let crtc = res.crtcs[0];
            let info = self.conn.randr_get_crtc_info(crtc, res.config_timestamp).unwrap().reply().unwrap();
            self.conn
                .randr_set_crtc_config(crtc, x11rb::CURRENT_TIME, res.config_timestamp, 0, 0, mode.id, info.rotation, &info.outputs)
                .unwrap()
                .reply()
                .is_ok_and(|r| u8::from(r.status) == 0)
        }
    }
}

#[test]
fn alt_tab_skips_helper_windows_and_brings_a_minimised_window_back() {
    alone("games::alt_tab_skips_helper_windows_and_brings_a_minimised_window_back", |dir| {
        let mut rig = Rig::new(dir);
        let display = older::start(&mut rig, dir);
        let _helper = older::Older::helper(display);
        assert!(
            until_drawn(&mut rig, Duration::from_secs(10), |r| r.state.space.elements().any(|w| w.0.x11_surface().is_some_and(|x| x.is_override_redirect()))),
            "the helper window never reached the compositor"
        );
        let mut game = Game::join(&mut rig);
        let first = rig.join(ClientState::default());
        rig.window(first, false);
        let back = rig.state.focused_window().expect("the first window has the keyboard");
        let second = rig.join(ClientState::default());
        rig.window(second, false);
        let front = rig.state.focused_window().expect("the second window has the keyboard");
        assert_ne!(back, front);

        alt_tab(&mut rig, &mut game);
        assert_eq!(rig.state.focused_window().as_ref(), Some(&back), "Alt-Tab did not bring the window at the back round");
        alt_tab(&mut rig, &mut game);
        assert_eq!(rig.state.focused_window().as_ref(), Some(&front));

        let surface = back.wl_surface().map(|s| s.into_owned()).unwrap();
        rig.state.set_window_minimized(&surface, true);
        rig.until(Duration::from_millis(100), |_| false);
        assert!(!rig.state.space.elements().any(|w| w == &back), "the minimised window is still on the screen");
        alt_tab(&mut rig, &mut game);
        assert!(rig.state.space.elements().any(|w| w == &back), "Alt-Tab left the minimised window off the screen");
        assert_eq!(rig.state.focused_window().as_ref(), Some(&back), "the window brought back did not get the keyboard");
    });
}

#[test]
fn an_older_game_that_minimises_itself_as_it_starts_is_brought_to_the_front() {
    alone("games::an_older_game_that_minimises_itself_as_it_starts_is_brought_to_the_front", |dir| {
        let mut rig = Rig::new(dir);
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        let display = older::start(&mut rig, dir);
        let old = older::Older::open(display);
        assert!(until_drawn(&mut rig, Duration::from_secs(10), |r| filling(r).is_some_and(|w| w.0.x11_surface().is_some())), "the game never filled the screen");
        let x11 = filling(&rig).unwrap();
        old.iconify();
        rig.until(Duration::from_secs(2), |_| false);
        assert_eq!(old.wm_state(), Some(1), "a game starting up was left minimised");
        assert!(!old.has("_NET_WM_STATE_HIDDEN"), "{:?}", old.net_state());
        assert_eq!(filling(&rig).as_ref(), Some(&x11), "the game does not fill its screen");
        assert_eq!(rig.state.focused_window().as_ref(), Some(&x11), "the game did not get the keyboard");
    });
}

#[test]
fn an_older_full_screen_game_stays_whole_through_alt_tab_a_lost_keyboard_and_a_resolution_change() {
    alone("games::an_older_full_screen_game_stays_whole_through_alt_tab_a_lost_keyboard_and_a_resolution_change", |dir| {
        let mut rig = Rig::new(dir);
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        let mut game = Game::join(&mut rig);
        let display = older::start(&mut rig, dir);
        let old = older::Older::open(display);
        let filled = |rig: &mut Rig, old: &older::Older| {
            until_drawn(rig, Duration::from_secs(10), |r| {
                filling(r).is_some_and(|w| w.0.x11_surface().is_some() && drawn(&w) == Some((1280, 800).into()))
            }) && old.size() == (1280, 800)
        };
        assert!(filled(&mut rig, &old), "the game that asked to open full screen does not fill the screen: {:?}", old.size());
        assert!(old.has("_NET_WM_STATE_FULLSCREEN"));
        rig.state.refresh_idle_inhibit();
        assert!(rig.state.idle_notifier_state.is_inhibited(), "the screen may sleep while an older game fills it");
        let x11 = filling(&rig).unwrap();

        let stranger = rig.join(ClientState::default());
        rig.window(stranger, false);
        assert_eq!(filling(&rig).as_ref(), Some(&x11), "a stranger's window ended the game's full screen");
        assert_eq!(rig.state.focused_window().as_ref(), Some(&x11), "a stranger's window took the game's keyboard");

        alt_tab(&mut rig, &mut game);
        assert!(filling(&rig).is_none(), "the window in front is hidden behind the game");
        assert!(rig.state.focused_window().is_some_and(|w| w != x11));
        assert!(rig.state.space.elements().any(|w| w == &x11), "the game was taken off the screen when it lost the keyboard");
        assert_eq!(old.wm_state(), Some(1));
        rig.state.refresh_idle_inhibit();
        assert!(!rig.state.idle_notifier_state.is_inhibited(), "a game behind another window kept the screen awake");

        rig.state.fresh_for = Duration::ZERO;
        old.iconify();
        assert!(until_drawn(&mut rig, Duration::from_secs(5), |_| old.wm_state() == Some(3)), "the game was not told it is minimised");
        assert!(rig.state.space.elements().any(|w| w == &x11), "a full-screen game was minimised on losing the keyboard");

        alt_tab(&mut rig, &mut game);
        assert!(until_drawn(&mut rig, Duration::from_secs(5), |_| old.wm_state() == Some(1)), "the game was not told it is back");
        assert!(old.has("_NET_WM_STATE_FULLSCREEN") && !old.has("_NET_WM_STATE_HIDDEN"), "{:?}", old.net_state());
        assert_eq!(filling(&rig).as_ref(), Some(&x11), "the game does not fill its screen after Alt-Tab");

        let surface = x11.wl_surface().map(|s| s.into_owned()).unwrap();
        rig.state.set_window_minimized(&surface, true);
        rig.state.set_window_minimized(&surface, false);
        assert!(filled(&mut rig, &old), "the game did not come back whole after Show desktop");
        rig.state.set_workspace_count(2);
        rig.state.go_to_workspace(1);
        assert!(filling(&rig).is_none());
        rig.state.go_to_workspace(0);
        rig.state.focus_window(&x11);
        assert!(filled(&mut rig, &old), "the game did not come back whole from another workspace");

        assert!(old.set_mode(1024, 768), "Xwayland refused a 1024 by 768 mode");
        assert!(until_drawn(&mut rig, Duration::from_secs(5), |_| old.size() == (1024, 768)), "the game was not given the resolution it chose: {:?}", old.size());
        let shown = until_drawn(&mut rig, Duration::from_secs(5), |_| drawn(&x11) == Some((1280, 800).into()));
        assert!(shown, "the game at its own resolution is not scaled to the screen: {:?}", drawn(&x11));
        assert_eq!(rig.output.current_mode().map(|m| m.size), Some((1280, 800).into()));

        drop(old);
        assert!(until_drawn(&mut rig, Duration::from_secs(5), |r| filling(r).is_none() && !r.state.space.elements().any(|w| w == &x11)), "the crashed game's window stayed");
        assert_eq!(rig.output.current_mode().map(|m| m.size), Some((1280, 800).into()));
        assert!(rig.state.focused_window().is_some(), "nothing had the keyboard after the game crashed");
    });
}

#[test]
fn the_x_server_names_a_windows_real_program_whatever_it_claims() {
    alone("games::the_x_server_names_a_windows_real_program_whatever_it_claims", |dir| {
        use x11rb::{connection::Connection, protocol::xproto::{AtomEnum, ConnectionExt, PropMode}, wrapper::ConnectionExt as _};
        let mut rig = Rig::new(dir);
        let display = older::start(&mut rig, dir);
        let old = older::Older::open(display);
        let pid_atom = old.conn.intern_atom(false, b"_NET_WM_PID").unwrap().reply().unwrap().atom;
        old.conn.change_property32(PropMode::REPLACE, old.window, pid_atom, AtomEnum::CARDINAL, &[1]).unwrap();
        old.conn.flush().unwrap();
        let (asker, _) = std::thread::scope(|s| {
            s.spawn(|| x11rb::connect(Some(&format!(":{display}"))).unwrap()).join().unwrap()
        });
        let window = old.window;
        let found = std::thread::scope(|s| {
            let asked = s.spawn(|| crate::panel::x11_program(&asker, window));
            while !asked.is_finished() {
                rig.pump();
            }
            asked.join().unwrap()
        });
        assert_eq!(found, Some(std::process::id()), "the server names who really connected, not the window's claim of pid 1");
        asker.kill_client(window).unwrap();
        asker.flush().unwrap();
        assert!(rig.until(Duration::from_secs(5), |_| old.conn.get_input_focus().ok().and_then(|c| c.reply().ok()).is_none()), "the program was not disconnected");
    });
}
