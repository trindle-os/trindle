//! The screens and windows screen sharing can send, and their pictures, asked of Trindle's compositor over ext-image-copy-capture.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd};
use std::rc::Rc;

use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_output::{self, WlOutput};
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::{wl_buffer, wl_registry, wl_shm, wl_shm_pool};
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1::{self, ExtForeignToplevelHandleV1},
    ext_foreign_toplevel_list_v1::{self, ExtForeignToplevelListV1},
};
use wayland_protocols::ext::image_capture_source::v1::client::{
    ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1,
    ext_image_capture_source_v1::ExtImageCaptureSourceV1,
    ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1,
};
use wayland_protocols::ext::image_copy_capture::v1::client::{
    ext_image_copy_capture_cursor_session_v1::{self, ExtImageCopyCaptureCursorSessionV1},
    ext_image_copy_capture_frame_v1::{self, ExtImageCopyCaptureFrameV1},
    ext_image_copy_capture_manager_v1::{self, ExtImageCopyCaptureManagerV1},
    ext_image_copy_capture_session_v1::{self, ExtImageCopyCaptureSessionV1},
};
use wayland_protocols::wp::linux_dmabuf::zv1::client::{
    zwp_linux_buffer_params_v1::{self, ZwpLinuxBufferParamsV1},
    zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1,
};
use wayland_protocols::xdg::xdg_output::zv1::client::{
    zxdg_output_manager_v1::ZxdgOutputManagerV1,
    zxdg_output_v1::{self, ZxdgOutputV1},
};

use crate::common::shm::Memory;
use crate::toolkit::event::{Flow, Loop};

#[derive(Clone, Debug, PartialEq)]
pub struct Screen {
    pub name: String,
    pub description: String,
    pub place: [i32; 4],
    pub pixels: (i32, i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    Screen(String),
    Window(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Heard {
    Size(Constraints),
    Ready(Vec<(i32, i32, u32, u32)>),
    Failed,
    Stopped,
    Pointer(PointerHeard),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointerHeard {
    Over,
    Gone,
    At(i32, i32),
    Hotspot(i32, i32),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Constraints {
    pub w: u32,
    pub h: u32,
    pub device: Option<u64>,
    pub modifiers: Vec<u64>,
}

struct ScreenState {
    global: u32,
    output: WlOutput,
    _xdg: Option<ZxdgOutputV1>,
    screen: Screen,
}

struct WindowState {
    handle: ExtForeignToplevelHandleV1,
    window: Window,
    shown: bool,
}

struct SessionState {
    proxy: ExtImageCopyCaptureSessionV1,
    _source: ExtImageCaptureSourceV1,
    cursor: Option<ExtImageCopyCaptureCursorSessionV1>,
    size: (u32, u32),
    told: Constraints,
    frame: Option<ExtImageCopyCaptureFrameV1>,
    damage: Vec<(i32, i32, u32, u32)>,
}

struct State {
    shm: Option<wl_shm::WlShm>,
    manager: Option<ExtImageCopyCaptureManagerV1>,
    outputs: Option<ExtOutputImageCaptureSourceManagerV1>,
    toplevels: Option<ExtForeignToplevelImageCaptureSourceManagerV1>,
    xdg: Option<ZxdgOutputManagerV1>,
    dmabuf: Option<ZwpLinuxDmabufV1>,
    pointer: Option<WlPointer>,
    screens: Vec<ScreenState>,
    windows: Vec<WindowState>,
    sessions: HashMap<u64, SessionState>,
    heard: Vec<(u64, Heard)>,
}

pub struct Capture {
    conn: Connection,
    queue: RefCell<EventQueue<State>>,
    state: RefCell<State>,
    handlers: RefCell<HashMap<u64, Rc<dyn Fn(Heard)>>>,
    next: Cell<u64>,
}

impl Capture {
    pub fn connect(events: &Loop) -> Option<Rc<Capture>> {
        let conn = Connection::connect_to_env().ok()?;
        let (globals, mut queue) = registry_queue_init::<State>(&conn).ok()?;
        let qh = queue.handle();
        let mut state = State {
            shm: globals.bind(&qh, 1..=1, ()).ok(),
            manager: globals.bind(&qh, 1..=1, ()).ok(),
            outputs: globals.bind(&qh, 1..=1, ()).ok(),
            toplevels: globals.bind(&qh, 1..=1, ()).ok(),
            xdg: globals.bind(&qh, 2..=3, ()).ok(),
            dmabuf: globals.bind(&qh, 3..=4, ()).ok(),
            pointer: None,
            screens: Vec::new(),
            windows: Vec::new(),
            sessions: HashMap::new(),
            heard: Vec::new(),
        };
        if state.shm.is_none() || state.manager.is_none() || state.outputs.is_none() {
            return None;
        }
        let _list: Option<ExtForeignToplevelListV1> = globals.bind(&qh, 1..=1, ()).ok();
        let seat: Option<WlSeat> = globals.bind(&qh, 1..=5, ()).ok();
        state.pointer = seat.map(|s| s.get_pointer(&qh, ()));
        let outputs: Vec<(u32, u32)> = globals.contents().clone_list().into_iter()
            .filter(|g| g.interface == WlOutput::interface().name)
            .map(|g| (g.name, g.version))
            .collect();
        for (name, version) in outputs {
            state.add_screen(globals.registry(), name, version, &qh);
        }
        queue.roundtrip(&mut state).ok()?;
        queue.roundtrip(&mut state).ok()?;
        let me = Rc::new(Capture {
            conn,
            queue: RefCell::new(queue),
            state: RefCell::new(state),
            handlers: RefCell::new(HashMap::new()),
            next: Cell::new(1),
        });
        let fd = me.conn.backend().poll_fd().as_raw_fd();
        let weak = Rc::downgrade(&me);
        events.watch(fd, move |hung_up| {
            let Some(me) = weak.upgrade() else { return Flow::Stop };
            if hung_up || !me.pump() {
                me.lost();
                return Flow::Stop;
            }
            Flow::Continue
        });
        Some(me)
    }

    pub fn has_windows(&self) -> bool {
        self.state.borrow().toplevels.is_some()
    }

    pub fn screens(&self) -> Vec<Screen> {
        self.state.borrow().screens.iter().filter(|s| s.screen.pixels.0 > 0).map(|s| s.screen.clone()).collect()
    }

    pub fn windows(&self) -> Vec<Window> {
        self.state.borrow().windows.iter()
            .filter(|w| w.shown && w.window.app_id != super::portal_ask::APP_ID)
            .map(|w| w.window.clone())
            .collect()
    }

    fn pump(&self) -> bool {
        if let Some(guard) = self.conn.prepare_read() {
            if guard.read().is_err() && self.conn.flush().is_err() {
                return false;
            }
        }
        if self.queue.borrow_mut().dispatch_pending(&mut self.state.borrow_mut()).is_err() {
            return false;
        }
        self.deliver();
        self.conn.flush().is_ok()
    }

    fn deliver(&self) {
        let heard = std::mem::take(&mut self.state.borrow_mut().heard);
        for (id, what) in heard {
            let handler = self.handlers.borrow().get(&id).cloned();
            if let Some(handler) = handler {
                handler(what);
            }
        }
    }

    fn lost(&self) {
        let ids: Vec<u64> = self.handlers.borrow().keys().copied().collect();
        for id in ids {
            let handler = self.handlers.borrow_mut().remove(&id);
            if let Some(handler) = handler {
                handler(Heard::Stopped);
            }
        }
    }

    pub fn open(&self, source: &Source, pointer: bool, handler: impl Fn(Heard) + 'static) -> Option<u64> {
        self.open_as(source, pointer, false, handler)
    }

    pub fn open_pointer(&self, source: &Source, handler: impl Fn(Heard) + 'static) -> Option<u64> {
        self.open_as(source, false, true, handler)
    }

    fn open_as(&self, source: &Source, paint: bool, follow: bool, handler: impl Fn(Heard) + 'static) -> Option<u64> {
        let qh = self.queue.borrow().handle();
        let id = self.next.replace(self.next.get() + 1);
        {
            let mut state = self.state.borrow_mut();
            let made = match source {
                Source::Screen(name) => {
                    let output = state.screens.iter().find(|s| s.screen.name == *name)?.output.clone();
                    state.outputs.as_ref()?.create_source(&output, &qh, ())
                }
                Source::Window(wid) => {
                    let handle = state.windows.iter().find(|w| w.window.id == *wid)?.handle.clone();
                    state.toplevels.as_ref()?.create_source(&handle, &qh, ())
                }
            };
            let manager = state.manager.clone()?;
            let (proxy, cursor) = if follow {
                let cursor = manager.create_pointer_cursor_session(&made, state.pointer.as_ref()?, &qh, id);
                (cursor.get_capture_session(&qh, id), Some(cursor))
            } else {
                let options = if paint {
                    ext_image_copy_capture_manager_v1::Options::PaintCursors
                } else {
                    ext_image_copy_capture_manager_v1::Options::empty()
                };
                (manager.create_session(&made, options, &qh, id), None)
            };
            state.sessions.insert(id, SessionState {
                proxy, _source: made, cursor, size: (0, 0), told: Constraints::default(), frame: None, damage: Vec::new(),
            });
        }
        self.handlers.borrow_mut().insert(id, Rc::new(handler));
        let _ = self.conn.flush();
        Some(id)
    }

    pub fn close(&self, id: u64) {
        self.handlers.borrow_mut().remove(&id);
        if let Some(s) = self.state.borrow_mut().sessions.remove(&id) {
            if let Some(frame) = s.frame {
                frame.destroy();
            }
            s.proxy.destroy();
            if let Some(cursor) = s.cursor {
                cursor.destroy();
            }
        }
        let _ = self.conn.flush();
    }

    pub fn buffer(&self, fd: BorrowedFd<'_>, len: usize, w: u32, h: u32, stride: u32) -> Option<Buffer> {
        self.shm_buffer(fd, len, w, h, stride, wl_shm::Format::Xrgb8888)
    }

    pub fn see_through_buffer(&self, fd: BorrowedFd<'_>, len: usize, w: u32, h: u32) -> Option<Buffer> {
        self.shm_buffer(fd, len, w, h, w * 4, wl_shm::Format::Argb8888)
    }

    fn shm_buffer(&self, fd: BorrowedFd<'_>, len: usize, w: u32, h: u32, stride: u32, format: wl_shm::Format) -> Option<Buffer> {
        let qh = self.queue.borrow().handle();
        let state = self.state.borrow();
        let shm = state.shm.as_ref()?;
        let pool = shm.create_pool(fd, len as i32, &qh, ());
        let buffer = pool.create_buffer(0, w as i32, h as i32, stride as i32, format, &qh, ());
        pool.destroy();
        Some(Buffer(buffer))
    }

    pub fn dmabuf_buffer(&self, bo: &super::gbm::Bo, w: u32, h: u32) -> Option<Buffer> {
        let qh = self.queue.borrow().handle();
        let state = self.state.borrow();
        let params = state.dmabuf.as_ref()?.create_params(&qh, ());
        for (i, plane) in bo.planes.iter().enumerate() {
            params.add(plane.fd.as_fd(), i as u32, plane.offset, plane.stride, (bo.modifier >> 32) as u32, bo.modifier as u32);
        }
        let buffer = params.create_immed(w as i32, h as i32, super::gbm::XRGB8888,
                                         zwp_linux_buffer_params_v1::Flags::empty(), &qh, ());
        params.destroy();
        Some(Buffer(buffer))
    }

    pub fn capture(&self, id: u64, buffer: &Buffer) -> bool {
        let qh = self.queue.borrow().handle();
        let mut state = self.state.borrow_mut();
        let Some(session) = state.sessions.get_mut(&id) else { return false };
        if session.frame.is_some() || session.size.0 == 0 {
            return false;
        }
        let frame = session.proxy.create_frame(&qh, id);
        frame.attach_buffer(&buffer.0);
        frame.damage_buffer(0, 0, session.size.0 as i32, session.size.1 as i32);
        frame.capture();
        session.frame = Some(frame);
        drop(state);
        let _ = self.conn.flush();
        true
    }

    pub fn snapshot(self: &Rc<Self>, source: &Source, px: u32, done: impl FnOnce(Option<tiny_skia::Pixmap>) + 'static) {
        let shot = Rc::new(RefCell::new(Shot { memory: None, buffer: None, size: (0, 0), done: Some(Box::new(done)) }));
        let id = Rc::new(Cell::new(0u64));
        let (me, s, i) = (Rc::downgrade(self), shot.clone(), id.clone());
        let finish = move |picture: Option<tiny_skia::Pixmap>| {
            let done = s.borrow_mut().done.take();
            if let Some(me) = me.upgrade() {
                me.close(i.get());
            }
            if let Some(done) = done {
                done(picture);
            }
        };
        let me = Rc::downgrade(self);
        let (s, i) = (shot.clone(), id.clone());
        let opened = self.open(source, false, move |heard| match heard {
            Heard::Size(Constraints { w, h, .. }) => {
                let Some(me) = me.upgrade() else { return };
                let len = (w * h * 4) as usize;
                let Some(memory) = Memory::new(c"trindle-cast-picture", len) else { return finish(None) };
                let Some(buffer) = me.buffer(memory.fd(), len, w, h, w * 4) else { return finish(None) };
                s.borrow_mut().keep((w, h), memory, buffer);
                let shot = s.borrow();
                if let Some(b) = &shot.buffer {
                    me.capture(i.get(), b);
                }
            }
            Heard::Ready(_) => {
                let picture = s.borrow_mut().small(px);
                finish(picture)
            }
            Heard::Pointer(_) => {}
            Heard::Failed | Heard::Stopped => {
                s.borrow_mut().let_go();
                finish(None)
            }
        });
        match opened {
            Some(n) => id.set(n),
            None => {
                let done = shot.borrow_mut().done.take();
                if let Some(done) = done {
                    done(None);
                }
            }
        }
    }
}

struct Shot {
    memory: Option<Memory>,
    buffer: Option<Buffer>,
    size: (u32, u32),
    done: Option<Box<dyn FnOnce(Option<tiny_skia::Pixmap>)>>,
}

impl Shot {
    fn keep(&mut self, size: (u32, u32), memory: Memory, buffer: Buffer) {
        self.let_go();
        self.size = size;
        self.memory = Some(memory);
        self.buffer = Some(buffer);
    }

    fn small(&mut self, px: u32) -> Option<tiny_skia::Pixmap> {
        let (w, h) = self.size;
        let picture = self.memory.as_ref().and_then(|m| small_picture(m.bytes(), w, h, px));
        self.let_go();
        self.memory = None;
        picture
    }

    fn let_go(&mut self) {
        if let Some(b) = self.buffer.take() {
            b.0.destroy();
        }
    }
}

pub struct Buffer(wl_buffer::WlBuffer);

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.0.is_alive() {
            self.0.destroy();
        }
    }
}

pub fn small_picture(bytes: &[u8], w: u32, h: u32, px: u32) -> Option<tiny_skia::Pixmap> {
    if w == 0 || h == 0 || bytes.len() < (w * h * 4) as usize {
        return None;
    }
    let k = (px as f32 / w as f32).min(px as f32 / h as f32).min(1.0);
    let (tw, th) = (((w as f32 * k).round() as u32).max(1), ((h as f32 * k).round() as u32).max(1));
    let mut out = tiny_skia::Pixmap::new(tw, th)?;
    let data = out.data_mut();
    for ty in 0..th {
        let (y0, y1) = (ty * h / th, ((ty + 1) * h / th).max(ty * h / th + 1).min(h));
        for tx in 0..tw {
            let (x0, x1) = (tx * w / tw, ((tx + 1) * w / tw).max(tx * w / tw + 1).min(w));
            let (sy, sx) = (((y1 - y0) / 4).max(1), ((x1 - x0) / 4).max(1));
            let (mut sum, mut n) = ([0u32; 3], 0u32);
            let mut y = y0;
            while y < y1 {
                let mut x = x0;
                while x < x1 {
                    let at = ((y * w + x) * 4) as usize;
                    sum[0] += bytes[at + 2] as u32;
                    sum[1] += bytes[at + 1] as u32;
                    sum[2] += bytes[at] as u32;
                    n += 1;
                    x += sx;
                }
                y += sy;
            }
            let at = ((ty * tw + tx) * 4) as usize;
            let n = n.max(1);
            data[at] = (sum[0] / n) as u8;
            data[at + 1] = (sum[1] / n) as u8;
            data[at + 2] = (sum[2] / n) as u8;
            data[at + 3] = 255;
        }
    }
    Some(out)
}

impl State {
    fn add_screen(&mut self, registry: &wl_registry::WlRegistry, global: u32, version: u32, qh: &QueueHandle<Self>) {
        let output: WlOutput = registry.bind(global, version.min(4), qh, global);
        let xdg = self.xdg.as_ref().map(|m| m.get_xdg_output(&output, qh, global));
        let name = format!("screen-{global}");
        self.screens.push(ScreenState {
            global,
            output,
            _xdg: xdg,
            screen: Screen { name, description: String::new(), place: [0; 4], pixels: (0, 0) },
        });
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, qh: &QueueHandle<Self>) {
        match event {
            wl_registry::Event::Global { name, interface, version } if interface == WlOutput::interface().name => {
                state.add_screen(registry, name, version, qh);
            }
            wl_registry::Event::GlobalRemove { name } => {
                state.screens.retain(|s| s.global != name);
            }
            _ => {}
        }
    }
}

impl Dispatch<WlOutput, u32> for State {
    fn event(state: &mut Self, _: &WlOutput, event: wl_output::Event, global: &u32,
             _: &Connection, _: &QueueHandle<Self>) {
        let Some(s) = state.screens.iter_mut().find(|s| s.global == *global) else { return };
        match event {
            wl_output::Event::Name { name } => s.screen.name = name,
            wl_output::Event::Description { description } => s.screen.description = description,
            wl_output::Event::Mode { flags: WEnum::Value(f), width, height, .. } if f.contains(wl_output::Mode::Current) => {
                s.screen.pixels = (width, height);
            }
            wl_output::Event::Geometry { transform: WEnum::Value(t), .. }
                if matches!(t, wl_output::Transform::_90 | wl_output::Transform::_270
                    | wl_output::Transform::Flipped90 | wl_output::Transform::Flipped270) && s.screen.pixels.0 > s.screen.pixels.1 => {
                    s.screen.pixels = (s.screen.pixels.1, s.screen.pixels.0);
                }
            _ => {}
        }
    }
}

impl Dispatch<ZxdgOutputV1, u32> for State {
    fn event(state: &mut Self, _: &ZxdgOutputV1, event: zxdg_output_v1::Event, global: &u32,
             _: &Connection, _: &QueueHandle<Self>) {
        let Some(s) = state.screens.iter_mut().find(|s| s.global == *global) else { return };
        match event {
            zxdg_output_v1::Event::LogicalPosition { x, y } => {
                s.screen.place[0] = x;
                s.screen.place[1] = y;
            }
            zxdg_output_v1::Event::LogicalSize { width, height } => {
                s.screen.place[2] = width;
                s.screen.place[3] = height;
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtForeignToplevelListV1, ()> for State {
    fn event(state: &mut Self, _: &ExtForeignToplevelListV1, event: ext_foreign_toplevel_list_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        if let ext_foreign_toplevel_list_v1::Event::Toplevel { toplevel } = event {
            state.windows.push(WindowState {
                handle: toplevel,
                window: Window { id: String::new(), title: String::new(), app_id: String::new() },
                shown: false,
            });
        }
    }

    event_created_child!(State, ExtForeignToplevelListV1, [
        ext_foreign_toplevel_list_v1::EVT_TOPLEVEL_OPCODE => (ExtForeignToplevelHandleV1, ()),
    ]);
}

impl Dispatch<ExtForeignToplevelHandleV1, ()> for State {
    fn event(state: &mut Self, handle: &ExtForeignToplevelHandleV1, event: ext_foreign_toplevel_handle_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        use ext_foreign_toplevel_handle_v1::Event;
        if let Event::Closed = event {
            state.windows.retain(|w| w.handle != *handle);
            handle.destroy();
            return;
        }
        let Some(w) = state.windows.iter_mut().find(|w| w.handle == *handle) else { return };
        match event {
            Event::Identifier { identifier } => w.window.id = identifier,
            Event::Title { title } => w.window.title = title,
            Event::AppId { app_id } => w.window.app_id = app_id,
            Event::Done => w.shown = true,
            _ => {}
        }
    }
}

impl Dispatch<ExtImageCopyCaptureSessionV1, u64> for State {
    fn event(state: &mut Self, _: &ExtImageCopyCaptureSessionV1, event: ext_image_copy_capture_session_v1::Event, id: &u64,
             _: &Connection, _: &QueueHandle<Self>) {
        use ext_image_copy_capture_session_v1::Event;
        let Some(session) = state.sessions.get_mut(id) else { return };
        match event {
            Event::BufferSize { width, height } => {
                session.told.w = width;
                session.told.h = height;
            }
            Event::DmabufDevice { device } => {
                session.told.device = device.get(..8).and_then(|b| b.try_into().ok()).map(u64::from_ne_bytes);
            }
            Event::DmabufFormat { format, modifiers } if format == super::gbm::XRGB8888 => {
                session.told.modifiers = modifiers.chunks_exact(8)
                    .filter_map(|c| c.try_into().ok().map(u64::from_ne_bytes))
                    .filter(|&m| m != super::gbm::MOD_INVALID)
                    .collect();
            }
            Event::Done => {
                let told = std::mem::take(&mut session.told);
                session.size = (told.w, told.h);
                state.heard.push((*id, Heard::Size(told)));
            }
            Event::Stopped => state.heard.push((*id, Heard::Stopped)),
            _ => {}
        }
    }
}

impl Dispatch<ExtImageCopyCaptureFrameV1, u64> for State {
    fn event(state: &mut Self, frame: &ExtImageCopyCaptureFrameV1, event: ext_image_copy_capture_frame_v1::Event, id: &u64,
             _: &Connection, _: &QueueHandle<Self>) {
        use ext_image_copy_capture_frame_v1::Event;
        let Some(session) = state.sessions.get_mut(id) else {
            if matches!(event, Event::Ready | Event::Failed { .. }) {
                frame.destroy();
            }
            return;
        };
        match event {
            Event::Damage { x, y, width, height } => {
                session.damage.push((x.max(0), y.max(0), width.max(0) as u32, height.max(0) as u32));
            }
            Event::Ready => {
                frame.destroy();
                session.frame = None;
                let damage = std::mem::take(&mut session.damage);
                state.heard.push((*id, Heard::Ready(damage)));
            }
            Event::Failed { .. } => {
                frame.destroy();
                session.frame = None;
                session.damage.clear();
                state.heard.push((*id, Heard::Failed));
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtImageCopyCaptureCursorSessionV1, u64> for State {
    fn event(state: &mut Self, _: &ExtImageCopyCaptureCursorSessionV1, event: ext_image_copy_capture_cursor_session_v1::Event,
             id: &u64, _: &Connection, _: &QueueHandle<Self>) {
        use ext_image_copy_capture_cursor_session_v1::Event;
        let heard = match event {
            Event::Enter => PointerHeard::Over,
            Event::Leave => PointerHeard::Gone,
            Event::Position { x, y } => PointerHeard::At(x, y),
            Event::Hotspot { x, y } => PointerHeard::Hotspot(x, y),
            _ => return,
        };
        state.heard.push((*id, Heard::Pointer(heard)));
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for State {
            fn event(_: &mut Self, _: &$t, _: <$t as Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
        }
    )*};
}

ignore!(wl_shm::WlShm, wl_shm_pool::WlShmPool, wl_buffer::WlBuffer, ExtImageCopyCaptureManagerV1,
        ExtOutputImageCaptureSourceManagerV1, ExtForeignToplevelImageCaptureSourceManagerV1,
        ExtImageCaptureSourceV1, ZxdgOutputManagerV1, ZwpLinuxDmabufV1, ZwpLinuxBufferParamsV1, WlSeat, WlPointer);

#[cfg(test)]
mod tests {
    #[test]
    fn a_small_picture_keeps_the_shape_and_the_colours() {
        let (w, h) = (40u32, 20u32);
        let mut bytes = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let at = ((y * w + x) * 4) as usize;
                if x < w / 2 { bytes[at + 2] = 255 } else { bytes[at] = 255 }
            }
        }
        let px = super::small_picture(&bytes, w, h, 10).expect("picture");
        assert_eq!((px.width(), px.height()), (10, 5));
        let p = px.pixel(0, 0).unwrap();
        assert_eq!((p.red(), p.blue()), (255, 0));
        let p = px.pixel(9, 4).unwrap();
        assert_eq!((p.red(), p.blue()), (0, 255));
        assert!(super::small_picture(&bytes, w, h + 1, 10).is_none(), "too few bytes");
    }
}
