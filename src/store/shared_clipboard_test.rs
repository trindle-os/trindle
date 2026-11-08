//! The clipboard shared between two computers, both ways, as a plugin
//! allowed "control" does it: against a running trindle-compose only (the
//! headless harness, never the real desktop), as secctx.rs's tests are.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use wayland_client::backend::ObjectId;
use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::{
    wl_buffer, wl_compositor, wl_data_device, wl_data_device_manager, wl_data_offer, wl_data_source, wl_keyboard,
    wl_pointer, wl_registry, wl_seat, wl_shm, wl_shm_pool, wl_surface,
};
use wayland_client::{delegate_noop, event_created_child, Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1, zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};
use wayland_protocols_wlr::data_control::v1::client::{
    zwlr_data_control_device_v1::{self, ZwlrDataControlDeviceV1},
    zwlr_data_control_manager_v1::ZwlrDataControlManagerV1,
    zwlr_data_control_offer_v1::{self, ZwlrDataControlOfferV1},
    zwlr_data_control_source_v1::{self, ZwlrDataControlSourceV1},
};

use wayland_protocols_wlr::virtual_pointer::v1::client::{
    zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1, zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1,
};

use super::secctx;
use crate::common::sandbox::{CONTROL, CONTROL_CAPTURE};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
mod capture {
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::backend as wayland_backend;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/input_capture.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("src/compose/protocols/input_capture.xml");
}
use capture::trindle_input_capture_manager_v1::{Side, TrindleInputCaptureManagerV1};
use capture::trindle_input_capture_v1::{self, TrindleInputCaptureV1};

const TEXT: &str = "text/plain;charset=utf-8";

const CTRL: u32 = 29;
const C: u32 = 46;

#[derive(Default)]
struct Told {

    configured: bool,
    has_keyboard: bool,

    enters: usize,
    keys: Vec<(u32, bool)>,
    mods: u32,
    pointer_in: bool,
    ctrl_down: bool,

    copies_asked: (usize, u32),
    data_manager: Option<wl_data_device_manager::WlDataDeviceManager>,
    data_device: Option<wl_data_device::WlDataDevice>,

    copy_next: Option<Vec<u8>>,
    kinds: HashMap<ObjectId, Vec<String>>,
    offered: Option<wl_data_offer::WlDataOffer>,

    offers: usize,

    control_kinds: HashMap<ObjectId, Vec<String>>,
    control_offered: Option<ZwlrDataControlOfferV1>,
    control_offers: usize,

    serving: Vec<u8>,
    cancelled: usize,

    capture: Vec<String>,

    capture_keys: Vec<(u32, bool)>,
}

struct Program {
    conn: Connection,
    queue: EventQueue<Told>,
    told: Told,
    globals: GlobalList,
}

impl Program {
    fn at(socket: &std::path::Path) -> Program {
        let stream = std::os::unix::net::UnixStream::connect(socket).expect("the harness's socket");
        let conn = Connection::from_socket(stream).unwrap();
        let (globals, queue) = registry_queue_init::<Told>(&conn).unwrap();
        Program { conn, queue, told: Told::default(), globals }
    }

    fn has(&self, name: &str) -> bool {
        self.globals.contents().clone_list().iter().any(|g| g.interface == name)
    }

    fn turn(&mut self) {
        self.queue.dispatch_pending(&mut self.told).unwrap();
        self.conn.flush().unwrap();
        let Some(guard) = self.conn.prepare_read() else { return };
        let mut fd = libc::pollfd { fd: guard.connection_fd().as_raw_fd(), events: libc::POLLIN, revents: 0 };
        // SAFETY: one pollfd, owned here, for a descriptor the guard keeps open.
        if unsafe { libc::poll(&mut fd, 1, 5) } > 0 {
            guard.read().unwrap();
        }
        self.queue.dispatch_pending(&mut self.told).unwrap();
        self.conn.flush().unwrap();
    }
}

struct Three {
    window: Program,
    boxed: Program,
    taker: Program,

    outer: Option<Program>,
}

impl Three {
    fn turn(&mut self) {
        self.window.turn();
        self.boxed.turn();
        self.taker.turn();
        if let Some(outer) = self.outer.as_mut() {
            outer.turn();
        }
    }

    fn until(&mut self, what: &str, done: impl Fn(&Three) -> bool) {
        let end = Instant::now() + Duration::from_secs(10);
        while !done(self) {
            assert!(Instant::now() < end, "never happened: {what}");
            self.turn();
        }
    }

    fn settle(&mut self) {
        let end = Instant::now() + Duration::from_millis(400);
        while Instant::now() < end {
            self.turn();
        }
    }

    fn read(&mut self, from: OwnedFd) -> Vec<u8> {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut data = Vec::new();
            let _ = std::fs::File::from(from).read_to_end(&mut data);
            let _ = tx.send(data);
        });
        let end = Instant::now() + Duration::from_secs(10);
        loop {
            if let Ok(data) = rx.try_recv() {
                return data;
            }
            assert!(Instant::now() < end, "what was copied never came");
            self.turn();
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for Told {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents, _: &Connection,
             _: &QueueHandle<Self>) {}
}
delegate_noop!(Told: ignore wl_compositor::WlCompositor);
delegate_noop!(Told: ignore wl_shm::WlShm);
delegate_noop!(Told: ignore wl_shm_pool::WlShmPool);
delegate_noop!(Told: ignore wl_buffer::WlBuffer);
delegate_noop!(Told: ignore wl_surface::WlSurface);
delegate_noop!(Told: ignore wl_seat::WlSeat);
delegate_noop!(Told: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(Told: ignore wl_data_device_manager::WlDataDeviceManager);
delegate_noop!(Told: ignore ZwlrDataControlManagerV1);
delegate_noop!(Told: ignore ZwpVirtualKeyboardManagerV1);
delegate_noop!(Told: ignore ZwpVirtualKeyboardV1);
delegate_noop!(Told: ignore TrindleInputCaptureManagerV1);
delegate_noop!(Told: ignore ZwlrVirtualPointerManagerV1);
delegate_noop!(Told: ignore ZwlrVirtualPointerV1);

impl Dispatch<wl_pointer::WlPointer, ()> for Told {
    fn event(told: &mut Self, _: &wl_pointer::WlPointer, event: wl_pointer::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        match event {
            wl_pointer::Event::Enter { .. } => told.pointer_in = true,
            wl_pointer::Event::Leave { .. } => told.pointer_in = false,
            _ => {}
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for Told {
    fn event(_: &mut Self, base: &xdg_wm_base::XdgWmBase, event: xdg_wm_base::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            base.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for Told {
    fn event(told: &mut Self, surface: &xdg_surface::XdgSurface, event: xdg_surface::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        if let xdg_surface::Event::Configure { serial } = event {
            surface.ack_configure(serial);
            told.configured = true;
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for Told {
    fn event(told: &mut Self, _: &wl_keyboard::WlKeyboard, event: wl_keyboard::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        match event {
            wl_keyboard::Event::Enter { .. } => {
                told.has_keyboard = true;
                told.enters += 1;
            }
            wl_keyboard::Event::Leave { .. } => told.has_keyboard = false,

            wl_keyboard::Event::Modifiers { mods_depressed, .. } => {
                told.ctrl_down = mods_depressed & 4 != 0;
                told.mods = mods_depressed;
            }
            wl_keyboard::Event::Key { serial, key, state, .. } => {
                let pressed = state == WEnum::Value(wl_keyboard::KeyState::Pressed);
                told.keys.push((key, pressed));
                if key != C || !pressed || !told.ctrl_down {
                    return;
                }
                told.copies_asked = (told.copies_asked.0 + 1, serial);

                let Some(data) = told.copy_next.take() else { return };
                let source = told.data_manager.as_ref().unwrap().create_data_source(qh, ());
                source.offer(TEXT.into());
                told.data_device.as_ref().unwrap().set_selection(Some(&source), serial);
                told.serving = data;
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_data_device::WlDataDevice, ()> for Told {
    fn event(told: &mut Self, _: &wl_data_device::WlDataDevice, event: wl_data_device::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        match event {
            wl_data_device::Event::DataOffer { id } => {
                told.kinds.insert(id.id(), Vec::new());
            }
            wl_data_device::Event::Selection { id } => {
                told.offered = id;
                told.offers += 1;
            }
            _ => {}
        }
    }

    event_created_child!(Told, wl_data_device::WlDataDevice, [
        wl_data_device::EVT_DATA_OFFER_OPCODE => (wl_data_offer::WlDataOffer, ())
    ]);
}

impl Dispatch<wl_data_offer::WlDataOffer, ()> for Told {
    fn event(told: &mut Self, offer: &wl_data_offer::WlDataOffer, event: wl_data_offer::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        if let wl_data_offer::Event::Offer { mime_type } = event {
            told.kinds.entry(offer.id()).or_default().push(mime_type);
        }
    }
}

impl Dispatch<wl_data_source::WlDataSource, ()> for Told {
    fn event(told: &mut Self, _: &wl_data_source::WlDataSource, event: wl_data_source::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        match event {
            wl_data_source::Event::Send { fd, .. } => {
                let _ = std::fs::File::from(fd).write_all(&told.serving);
            }
            wl_data_source::Event::Cancelled => told.cancelled += 1,
            _ => {}
        }
    }
}

impl Dispatch<ZwlrDataControlDeviceV1, ()> for Told {
    fn event(told: &mut Self, _: &ZwlrDataControlDeviceV1, event: zwlr_data_control_device_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwlr_data_control_device_v1::Event::DataOffer { id } => {
                told.control_kinds.insert(id.id(), Vec::new());
            }
            zwlr_data_control_device_v1::Event::Selection { id } => {
                told.control_offered = id;
                told.control_offers += 1;
            }
            _ => {}
        }
    }

    event_created_child!(Told, ZwlrDataControlDeviceV1, [
        zwlr_data_control_device_v1::EVT_DATA_OFFER_OPCODE => (ZwlrDataControlOfferV1, ())
    ]);
}

impl Dispatch<ZwlrDataControlOfferV1, ()> for Told {
    fn event(told: &mut Self, offer: &ZwlrDataControlOfferV1, event: zwlr_data_control_offer_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        if let zwlr_data_control_offer_v1::Event::Offer { mime_type } = event {
            told.control_kinds.entry(offer.id()).or_default().push(mime_type);
        }
    }
}

impl Dispatch<ZwlrDataControlSourceV1, ()> for Told {
    fn event(told: &mut Self, _: &ZwlrDataControlSourceV1, event: zwlr_data_control_source_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwlr_data_control_source_v1::Event::Send { fd, .. } => {
                let _ = std::fs::File::from(fd).write_all(&told.serving);
            }
            zwlr_data_control_source_v1::Event::Cancelled => told.cancelled += 1,
            _ => {}
        }
    }
}

impl Dispatch<TrindleInputCaptureV1, ()> for Told {
    fn event(told: &mut Self, _: &TrindleInputCaptureV1, event: trindle_input_capture_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        told.capture.push(match event {
            trindle_input_capture_v1::Event::Ready => "ready",
            trindle_input_capture_v1::Event::Activated { .. } => "activated",
            trindle_input_capture_v1::Event::Deactivated { .. } => "deactivated",
            trindle_input_capture_v1::Event::Unavailable { .. } => "unavailable",
            trindle_input_capture_v1::Event::Key { key, state, .. } => {
                told.capture_keys.push((key, state == 1));
                return;
            }
            _ => return,
        }.into());
    }
}

fn keymap(keyboard: &ZwpVirtualKeyboardV1) {
    use xkbcommon::xkb;
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let map = xkb::Keymap::new_from_names(&context, "", "pc105", "gb", "", None, xkb::KEYMAP_COMPILE_NO_FLAGS).unwrap();
    let text = map.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    let memory = crate::common::shm::Memory::new(c"test-keymap", text.len() + 1).unwrap();
    std::fs::File::from(memory.fd().try_clone_to_owned().unwrap()).write_all(text.as_bytes()).unwrap();
    keyboard.keymap(1, memory.fd(), text.len() as u32 + 1);
}

fn copied_by_made_up_keys_reaches_the_box(three: &mut Three, keys: &ZwpVirtualKeyboardV1, text: &str) {
    let (asked, told_before) = (three.window.told.copies_asked.0, three.boxed.told.control_offers);
    three.window.told.copy_next = Some(text.as_bytes().to_vec());
    for (time, (key, down)) in [(CTRL, 1), (C, 1), (C, 0), (CTRL, 0)].into_iter().enumerate() {
        keys.key(time as u32, key, down);
    }
    three.until("the window was sent Ctrl and C from the made-up keyboard", |t| t.window.told.copies_asked.0 > asked);
    three.until("the box was told of the new copy", |t| {
        t.boxed.told.control_offers > told_before && t.boxed.told.control_offered.is_some()
    });
    let offer = three.boxed.told.control_offered.clone().unwrap();
    assert_eq!(three.boxed.told.control_kinds[&offer.id()], [TEXT]);
    let (read, write) = crate::common::run::pipe().unwrap();
    offer.receive(TEXT.into(), write.as_fd());
    drop(write);
    assert_eq!(three.read(read), text.as_bytes(), "the box reads what the window copied");
}

fn shown_window(three: &mut Three) -> crate::common::shm::Memory {

    let qh = three.window.queue.handle();
    let globals = &three.window.globals;
    let compositor: wl_compositor::WlCompositor = globals.bind(&qh, 1..=4, ()).unwrap();
    let shm: wl_shm::WlShm = globals.bind(&qh, 1..=1, ()).unwrap();
    let base: xdg_wm_base::XdgWmBase = globals.bind(&qh, 1..=1, ()).unwrap();
    let window_seat: wl_seat::WlSeat = globals.bind(&qh, 1..=5, ()).unwrap();
    let data_manager: wl_data_device_manager::WlDataDeviceManager = globals.bind(&qh, 1..=3, ()).unwrap();
    let _keyboard = window_seat.get_keyboard(&qh, ());
    three.window.told.data_device = Some(data_manager.get_data_device(&window_seat, &qh, ()));
    three.window.told.data_manager = Some(data_manager);
    let surface = compositor.create_surface(&qh, ());
    let xdg = base.get_xdg_surface(&surface, &qh, ());
    let toplevel = xdg.get_toplevel(&qh, ());
    toplevel.set_title("Shared clipboard test".into());
    surface.commit();
    three.until("the window was given a size", |t| t.window.told.configured);
    let (w, h) = (64, 64);
    let memory = crate::common::shm::Memory::new(c"test-window", w * h * 4).unwrap();
    let pool = shm.create_pool(memory.fd(), (w * h * 4) as i32, &qh, ());
    let buffer = pool.create_buffer(0, w as i32, h as i32, (w * 4) as i32, wl_shm::Format::Argb8888, &qh, ());
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    three.until("the window was given the keyboard", |t| t.window.told.has_keyboard);
    memory
}

#[test]
#[ignore = "needs a real compositor, named by TRINDLE_SECCTX_TEST: run by the script that sets it up"]
fn a_clipboard_is_shared_both_ways_by_a_box_allowed_control() {
    let Ok(run) = std::env::var("TRINDLE_SECCTX_TEST") else { return };
    let run = PathBuf::from(run);
    let plain = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());

    let (close_read, close_write) = secctx::close_pipe().unwrap();
    let socket = run.join("shared-clipboard-test");
    secctx::listen(&socket, "shared-clipboard-test", CONTROL, &close_read).unwrap();

    let mut three = Three { window: Program::at(&plain), boxed: Program::at(&socket), taker: Program::at(&plain), outer: None };

    assert!(three.boxed.has("zwlr_data_control_manager_v1") && three.boxed.has("zwp_virtual_keyboard_manager_v1"));
    assert!(!three.boxed.has("trindle_input_capture_manager_v1") && !three.boxed.has("ext_image_copy_capture_manager_v1"));

    let qh = three.boxed.queue.handle();
    let seat: wl_seat::WlSeat = three.boxed.globals.bind(&qh, 1..=1, ()).unwrap();
    let control: ZwlrDataControlManagerV1 = three.boxed.globals.bind(&qh, 1..=2, ()).unwrap();
    let control_device = control.get_data_device(&seat, &qh, ());
    let keys = three.boxed.globals.bind::<ZwpVirtualKeyboardManagerV1, _, _>(&qh, 1..=1, ()).unwrap()
        .create_virtual_keyboard(&seat, &qh, ());
    keymap(&keys);

    let _memory = shown_window(&mut three);

    copied_by_made_up_keys_reaches_the_box(&mut three, &keys, "copied on the controlled computer");

    let qh = three.taker.queue.handle();
    let manager: TrindleInputCaptureManagerV1 = three.taker.globals.bind(&qh, 2..=2, ()).unwrap();
    let side = Side::Right;
    let taken = manager.get_capture(side, &qh, ());
    three.until("the side was armed", |t| t.taker.told.capture.iter().any(|c| c == "ready"));
    taken.take(side as u32);
    three.until("the mouse and keyboard were taken", |t| t.taker.told.capture.iter().any(|c| c == "activated"));
    three.until("the window lost the keyboard", |t| !t.window.told.has_keyboard);

    let arriving = "copied on the other computer";
    let window_told = three.window.told.offers;
    let box_told = three.boxed.told.control_offers;
    let qh = three.boxed.queue.handle();
    let source = control.create_data_source(&qh, ());
    source.offer(TEXT.into());
    control_device.set_selection(Some(&source));
    three.boxed.told.serving = arriving.as_bytes().to_vec();
    three.until("the box's own copy was taken", |t| t.boxed.told.control_offers > box_told);
    three.settle();
    assert_eq!(three.window.told.cancelled, 1, "the window's copy gave way to the box's");

    assert_eq!(three.window.told.offers, window_told, "a window without the keyboard was told of a copy");

    taken.release(0.5);
    three.until("the window was given the keyboard back", |t| t.window.told.has_keyboard);
    three.until("the window was told what is on the clipboard", |t| t.window.told.offers > window_told);
    let offer = three.window.told.offered.clone().expect("something is on the clipboard");
    assert_eq!(three.window.told.kinds[&offer.id()], [TEXT]);
    let (read, write) = crate::common::run::pipe().unwrap();
    offer.receive(TEXT.into(), write.as_fd());
    drop(write);
    assert_eq!(three.read(read), arriving.as_bytes(), "the window pastes what the box set");

    copied_by_made_up_keys_reaches_the_box(&mut three, &keys, "copied again on the controlled computer");
    assert_eq!(three.boxed.told.cancelled, 1, "the box's copy gave way to the window's");

    drop(close_write);
    let _ = std::fs::remove_file(&socket);
}

const A: u32 = 30;
const SHIFT: u32 = 42;
const LEFT_BUTTON: u32 = 0x110;
const SUPER: u32 = 125;
const ESCAPE: u32 = 1;

fn now_ms() -> u32 {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_millis() as u32 + 1000
}

struct Hands {

    keys: ZwpVirtualKeyboardV1,

    mouse: ZwlrVirtualPointerV1,
}

impl Hands {
    fn key(&self, key: u32, down: bool) {
        self.keys.key(now_ms(), key, u32::from(down));
    }

    fn place(&self, x: u32, y: u32) {
        self.mouse.motion_absolute(now_ms(), x, y, 1000, 1000);
        self.mouse.frame();
    }

    fn click(&self) {
        self.mouse.button(now_ms(), LEFT_BUTTON, wl_pointer::ButtonState::Pressed);
        self.mouse.frame();
        self.mouse.button(now_ms(), LEFT_BUTTON, wl_pointer::ButtonState::Released);
        self.mouse.frame();
    }
}

fn times(three: &Three, what: &str) -> usize {
    three.boxed.told.capture.iter().filter(|c| *c == what).count()
}

fn cross_by_push(three: &mut Three, hands: &Hands, left: bool) {
    let before = times(three, "activated");
    hands.place(500, 500);
    three.settle();
    let end = Instant::now() + Duration::from_secs(10);
    while times(three, "activated") == before {
        assert!(Instant::now() < end, "a push against the armed side never crossed");
        hands.mouse.motion(now_ms(), if left { -300.0 } else { 300.0 }, 0.0);
        hands.mouse.frame();
        three.turn();
        std::thread::sleep(Duration::from_millis(15));
    }
}

fn typed_reaches_window(three: &mut Three, hands: &Hands, key: u32) -> bool {
    let from = three.window.told.keys.len();
    hands.key(key, true);
    hands.key(key, false);
    let end = Instant::now() + Duration::from_secs(2);
    while Instant::now() < end {
        three.turn();
        if three.window.told.keys[from..] == [(key, true), (key, false)] {
            return true;
        }
    }
    println!("typed {key}, the window was sent {:?}", &three.window.told.keys[from..]);
    false
}

fn typed_reaches_capture(three: &mut Three, hands: &Hands, key: u32) {
    let (from, window) = (three.boxed.told.capture_keys.len(), three.window.told.keys.len());
    hands.key(key, true);
    hands.key(key, false);
    three.until("the capture was handed the key", |t| t.boxed.told.capture_keys[from..] == [(key, true), (key, false)]);
    assert_eq!(three.window.told.keys.len(), window, "a key reached the window while the keyboard was away");
}

#[test]
#[ignore = "needs a real compositor, named by TRINDLE_SECCTX_TEST: run by the script that sets it up"]
fn typing_works_after_the_pointer_comes_back() {
    let Ok(run) = std::env::var("TRINDLE_SECCTX_TEST") else { return };
    let Ok(outer) = std::env::var("TRINDLE_OUTER_DISPLAY") else { return };
    let run = PathBuf::from(run);
    let plain = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());
    let (close_read, close_write) = secctx::close_pipe().unwrap();
    let socket = run.join("typing-after-test");
    secctx::listen(&socket, "typing-after-test", CONTROL_CAPTURE, &close_read).unwrap();
    let mut three = Three {
        window: Program::at(&plain),
        boxed: Program::at(&socket),
        taker: Program::at(&plain),
        outer: Some(Program::at(&run.join(outer))),
    };

    let hands = typing_hands(&three);
    let left = true;
    let plugin = typing_plugin(&mut three, left);
    let _memory = typing_window(&mut three);

    let ring = std::env::var_os("TRINDLE_RING").is_some();
    if ring {

        tap(&mut three, &hands, ESCAPE);
        tap(&mut three, &hands, ESCAPE);
    }
    let over = find_the_window(&mut three, &hands);

    let mut wrong: Vec<String> = Vec::new();
    check(&mut wrong, typed_reaches_window(&mut three, &hands, A), "typing reaches the window before anything".into());
    let rig = Typing { hands, plugin, left, over };

    for round in 1..=4 {
        if ring {
            round_with_the_ring(&mut three, &rig, &mut wrong, round);
        }
        round_across_and_back(&mut three, &rig, &mut wrong, round);
        round_key_held_across(&mut three, &rig, &mut wrong, round);
        round_key_pressed_while_away(&mut three, &rig, &mut wrong, round);
        round_made_up_keys(&mut three, &rig, &mut wrong, round);
    }

    drop(close_write);
    let _ = std::fs::remove_file(&socket);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

struct Typing {
    hands: Hands,
    plugin: Plugin,
    left: bool,
    over: (u32, u32),
}

struct Plugin {
    made_up_keys: ZwpVirtualKeyboardV1,
    control: ZwlrDataControlManagerV1,
    control_device: ZwlrDataControlDeviceV1,
    capture: TrindleInputCaptureV1,
}

fn check(wrong: &mut Vec<String>, ok: bool, what: String) {
    println!("{} {what}", if ok { "ok   " } else { "WRONG" });
    if !ok {
        wrong.push(what);
    }
}

fn tap(three: &mut Three, hands: &Hands, key: u32) {
    hands.key(key, true);
    hands.key(key, false);
    let end = Instant::now() + Duration::from_millis(1200);
    while Instant::now() < end {
        three.turn();
    }
}

fn typing_hands(three: &Three) -> Hands {
    let program = three.outer.as_ref().unwrap();
    let qh = program.queue.handle();
    let outer_seat: wl_seat::WlSeat = program.globals.bind(&qh, 1..=1, ()).unwrap();
    let keys = program.globals.bind::<ZwpVirtualKeyboardManagerV1, _, _>(&qh, 1..=1, ()).unwrap()
        .create_virtual_keyboard(&outer_seat, &qh, ());
    keymap(&keys);
    let qh = three.taker.queue.handle();
    let mouse = three.taker.globals.bind::<ZwlrVirtualPointerManagerV1, _, _>(&qh, 1..=2, ()).unwrap()
        .create_virtual_pointer(None, &qh, ());
    Hands { keys, mouse }
}

fn typing_plugin(three: &mut Three, left: bool) -> Plugin {

    let qh = three.boxed.queue.handle();
    let seat: wl_seat::WlSeat = three.boxed.globals.bind(&qh, 1..=1, ()).unwrap();
    let made_up_keys = three.boxed.globals.bind::<ZwpVirtualKeyboardManagerV1, _, _>(&qh, 1..=1, ()).unwrap()
        .create_virtual_keyboard(&seat, &qh, ());
    keymap(&made_up_keys);
    let _made_up_mouse = three.boxed.globals.bind::<ZwlrVirtualPointerManagerV1, _, _>(&qh, 1..=2, ()).unwrap()
        .create_virtual_pointer(Some(&seat), &qh, ());
    let control: ZwlrDataControlManagerV1 = three.boxed.globals.bind(&qh, 1..=2, ()).unwrap();
    let control_device = control.get_data_device(&seat, &qh, ());
    let manager: TrindleInputCaptureManagerV1 = three.boxed.globals.bind(&qh, 2..=2, ()).unwrap();
    let capture = manager.get_capture(Side::Left, &qh, ());
    three.until("the side was armed", |t| t.boxed.told.capture.iter().any(|c| c == "ready"));
    println!("armed on the {}", if left { "left" } else { "right" });

    Plugin { made_up_keys, control, control_device, capture }
}

fn typing_window(three: &mut Three) -> crate::common::shm::Memory {

    let qh = three.window.queue.handle();
    let globals = &three.window.globals;
    let compositor: wl_compositor::WlCompositor = globals.bind(&qh, 1..=4, ()).unwrap();
    let shm: wl_shm::WlShm = globals.bind(&qh, 1..=1, ()).unwrap();
    let base: xdg_wm_base::XdgWmBase = globals.bind(&qh, 1..=1, ()).unwrap();
    let window_seat: wl_seat::WlSeat = globals.bind(&qh, 1..=5, ()).unwrap();
    let _keyboard = window_seat.get_keyboard(&qh, ());
    let _pointer = window_seat.get_pointer(&qh, ());
    let surface = compositor.create_surface(&qh, ());
    let xdg = base.get_xdg_surface(&surface, &qh, ());
    let toplevel = xdg.get_toplevel(&qh, ());
    toplevel.set_title("Typing after test".into());
    surface.commit();
    three.until("the window was given a size", |t| t.window.told.configured);
    let (w, h) = (400, 300);
    let memory = crate::common::shm::Memory::new(c"test-window", w * h * 4).unwrap();
    let pool = shm.create_pool(memory.fd(), (w * h * 4) as i32, &qh, ());
    let buffer = pool.create_buffer(0, w as i32, h as i32, (w * 4) as i32, wl_shm::Format::Argb8888, &qh, ());
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    three.until("the window was given the keyboard", |t| t.window.told.has_keyboard);
    memory
}

fn find_the_window(three: &mut Three, hands: &Hands) -> (u32, u32) {

    let mut over = None;
    'found: for y in (100..1000).step_by(100) {
        for x in (100..1000).step_by(100) {
            hands.place(x, y);
            three.settle();
            if three.window.told.pointer_in {
                over = Some((x, y));
                break 'found;
            }
        }
    }
    over.expect("the pointer found the window")
}

fn round_with_the_ring(three: &mut Three, rig: &Typing, wrong: &mut Vec<String>, round: i32) {

    tap(three, &rig.hands, SUPER);
    check(wrong, !three.window.told.has_keyboard, format!("round {round}: the open ring has the keyboard"));
    tap(three, &rig.hands, ESCAPE);
    tap(three, &rig.hands, ESCAPE);
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing after the ring was opened and closed"));

    tap(three, &rig.hands, SUPER);
    cross_by_push(three, &rig.hands, rig.left);
    typed_reaches_capture(three, &rig.hands, A);
    rig.plugin.capture.release(0.5);
    three.until("the capture was told it is over", |t| times(t, "deactivated") == times(t, "activated"));
    three.settle();
    tap(three, &rig.hands, ESCAPE);
    tap(three, &rig.hands, ESCAPE);
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing after crossing with the ring open"));
}

fn round_across_and_back(three: &mut Three, rig: &Typing, wrong: &mut Vec<String>, round: i32) {

    let enters = three.window.told.enters;
    cross_by_push(three, &rig.hands, rig.left);
    three.until("the window lost the keyboard", |t| !t.window.told.has_keyboard);
    typed_reaches_capture(three, &rig.hands, A);
    if round == 2 {

        let told = three.boxed.told.control_offers;
        let source = rig.plugin.control.create_data_source(&three.boxed.queue.handle(), ());
        source.offer(TEXT.into());
        rig.plugin.control_device.set_selection(Some(&source));
        three.until("the plugin's copy was taken", |t| t.boxed.told.control_offers > told);
    }
    rig.plugin.capture.release(0.5);
    three.until("the capture was told it is over", |t| times(t, "deactivated") == times(t, "activated"));
    three.settle();
    check(wrong, three.window.told.has_keyboard && three.window.told.enters == enters + 1,
          format!("round {round}: the window has the keyboard again at once"));
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing reaches the window with no click"));
    rig.hands.place(rig.over.0, rig.over.1);
    rig.hands.click();
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing reaches the window after a click"));
}

fn round_key_held_across(three: &mut Three, rig: &Typing, wrong: &mut Vec<String>, round: i32) {
    rig.hands.key(SHIFT, true);
    three.settle();
    cross_by_push(three, &rig.hands, rig.left);
    rig.hands.key(SHIFT, false);
    three.until("the capture was handed the letting go", |t| t.boxed.told.capture_keys.last() == Some(&(SHIFT, false)));
    typed_reaches_capture(three, &rig.hands, A);
    rig.plugin.capture.release(0.5);
    three.until("the capture was told it is over", |t| times(t, "deactivated") == times(t, "activated"));
    three.settle();
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing after a key held across the crossing"));
    check(wrong, three.window.told.mods == 0, format!("round {round}: no modifier is left held ({})", three.window.told.mods));
}

fn round_key_pressed_while_away(three: &mut Three, rig: &Typing, wrong: &mut Vec<String>, round: i32) {

    cross_by_push(three, &rig.hands, rig.left);
    rig.hands.key(A, true);
    three.until("the capture was handed the press", |t| t.boxed.told.capture_keys.last() == Some(&(A, true)));
    rig.plugin.capture.release(0.5);
    three.until("the capture was told it is over", |t| times(t, "deactivated") == times(t, "activated"));
    let sent = three.window.told.keys.len();
    rig.hands.key(A, false);
    three.settle();
    check(wrong, three.window.told.keys.len() == sent, format!("round {round}: a key pressed while away is not let go in the window"));
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing after a key held on the way back"));
}

fn round_made_up_keys(three: &mut Three, rig: &Typing, wrong: &mut Vec<String>, round: i32) {

    rig.plugin.made_up_keys.key(now_ms(), SHIFT, 1);
    rig.plugin.made_up_keys.key(now_ms(), SHIFT, 0);
    three.settle();
    check(wrong, typed_reaches_window(three, &rig.hands, A), format!("round {round}: typing after the plugin's made-up keys"));
}

