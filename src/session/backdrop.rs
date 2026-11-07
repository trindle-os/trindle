//! The wallpaper itself.

use std::cell::{Cell, RefCell};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use tiny_skia::{FilterQuality, Pattern, Pixmap, Rect, SpreadMode, Transform};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::{
    wl_buffer, wl_callback, wl_compositor, wl_output, wl_region, wl_registry, wl_shm, wl_shm_pool, wl_subcompositor,
    wl_subsurface, wl_surface,
};
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use wayland_protocols::wp::fractional_scale::v1::client::{
    wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1,
    wp_fractional_scale_v1::{self, WpFractionalScaleV1},
};
use wayland_protocols::wp::viewporter::client::{wp_viewport::WpViewport, wp_viewporter::WpViewporter};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
};

use super::moving;
use crate::common::{charge, graphics, shm};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::picture;

#[derive(Clone, Default)]
pub struct Look {
    pub picture: Option<PathBuf>,
    pub fit: String,
    pub color: String,
}

struct Screen {
    name: u32,
    output: wl_output::WlOutput,
    surface: wl_surface::WlSurface,
    layer: ZwlrLayerSurfaceV1,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    size: (i32, i32),
    scale: i32,
    scale120: u32,
    configured: bool,
    film: Option<Film>,
}

struct Film {
    surface: wl_surface::WlSurface,
    sub: wl_subsurface::WlSubsurface,
    viewport: WpViewport,
    waiting: bool,
}

struct Movie {
    path: PathBuf,
    player: moving::Player,
    size: (i32, i32),
    memory: shm::Memory,
    buffers: [wl_buffer::WlBuffer; 2],
    busy: [bool; 2],
    shown: Option<usize>,
    every: Duration,
    due: Instant,
    on_battery: bool,
}

struct Slot(usize);

const REST_AFTER: Duration = Duration::from_secs(5);

const AGAIN_AFTER: Duration = Duration::from_secs(1);

fn asked_again(over: bool, now: Instant) -> Option<Instant> {
    (!over).then(|| now + AGAIN_AFTER)
}

fn stands_still(on_battery: bool, shown: bool) -> bool {
    on_battery && shown
}

struct State {
    compositor: wl_compositor::WlCompositor,
    shm: wl_shm::WlShm,
    layers: ZwlrLayerShellV1,
    viewporter: Option<WpViewporter>,
    fractions: Option<WpFractionalScaleManagerV1>,
    subsurfaces: Option<wl_subcompositor::WlSubcompositor>,
    screens: Vec<Screen>,
    look: Look,
    movie: Option<Movie>,
}

pub struct Backdrop {
    conn: Connection,
    queue: RefCell<EventQueue<State>>,
    state: RefCell<State>,
    events: Loop,
    timer: Cell<bool>,
}

impl Backdrop {
    pub fn start(look: Look, events: &Loop, gone: impl Fn() + 'static) -> Option<Rc<Backdrop>> {
        let conn = Connection::connect_to_env().map_err(|e| eprintln!("trindle-wallpaper: no compositor: {e}")).ok()?;
        let (globals, queue) = registry_queue_init::<State>(&conn).ok()?;
        let qh = queue.handle();
        let bound = (globals.bind(&qh, 1..=4, ()), globals.bind(&qh, 1..=1, ()), globals.bind(&qh, 1..=4, ()));
        let (Ok(compositor), Ok(shm), Ok(layers)) = bound else {
            eprintln!("trindle-wallpaper: the compositor cannot show a background");
            return None;
        };
        let mut state = State {
            compositor,
            shm,
            layers,
            viewporter: globals.bind(&qh, 1..=1, ()).ok(),
            fractions: globals.bind(&qh, 1..=1, ()).ok(),
            subsurfaces: globals.bind(&qh, 1..=1, ()).ok(),
            screens: Vec::new(),
            look,
            movie: None,
        };
        state.movie = start_movie(&state, &qh);
        let outputs: Vec<(u32, u32)> = globals.contents().clone_list().into_iter()
            .filter(|g| g.interface == "wl_output")
            .map(|g| (g.name, g.version))
            .collect();
        for (name, version) in outputs {
            add_screen(&mut state, &qh, globals.registry(), name, version);
        }
        let me = Rc::new(Backdrop { conn, queue: RefCell::new(queue), state: RefCell::new(state), events: events.clone(),
            timer: Cell::new(false) });
        me.pump();
        me.tick();
        let fd = me.conn.backend().poll_fd().as_raw_fd();
        let weak = Rc::downgrade(&me);
        events.watch(fd, move |hung_up| {
            let Some(me) = weak.upgrade() else { return Flow::Stop };
            if hung_up || !me.pump() {
                gone();
                return Flow::Stop;
            }
            me.tick();
            Flow::Continue
        });
        let weak = Rc::downgrade(&me);
        events.every(Duration::from_secs(5), move || {
            let Some(me) = weak.upgrade() else { return Flow::Stop };
            me.check_battery();
            Flow::Continue
        });
        Some(me)
    }

    fn pump(&self) -> bool {
        if let Some(guard) = self.conn.prepare_read() {
            if guard.read().is_err() && self.conn.flush().is_err() {
                return false;
            }
        }
        let ok = self.queue.borrow_mut().dispatch_pending(&mut self.state.borrow_mut()).is_ok();
        ok && self.conn.flush().is_ok()
    }

    pub fn drawn(&self) -> bool {
        self.state.borrow().screens.iter().all(|s| s.configured)
    }

    pub fn show(self: &Rc<Self>, look: Look) {
        let qh = self.queue.borrow().handle();
        let mut state = self.state.borrow_mut();
        let same = state.movie.as_ref().is_some_and(|m| Some(&m.path) == look.picture.as_ref());
        state.look = look;
        if !same {
            stop_movie(&mut state);
            state.movie = start_movie(&state, &qh);
        }
        for i in 0..state.screens.len() {
            draw(&mut state, &qh, i);
        }
        drop(state);
        self.tick();
        let _ = self.conn.flush();
    }

    fn tick(self: &Rc<Self>) {
        let qh = self.queue.borrow().handle();
        let mut state = self.state.borrow_mut();
        let State { movie, screens, .. } = &mut *state;
        let Some(movie) = movie.as_mut() else { return };
        if stands_still(movie.on_battery, movie.shown.is_some()) || !screens.iter().any(|s| s.film.as_ref().is_some_and(|f| !f.waiting)) {
            return;
        }
        let now = Instant::now();
        if now < movie.due {
            self.tick_in(movie.due - now);
            return;
        }
        let free = |i: usize| !movie.busy[i];
        let Some(slot) = (0..2).find(|&i| free(i) && Some(i) != movie.shown).or_else(|| (0..2).find(|&i| free(i))) else {
            return;
        };
        let (w, h) = movie.size;
        let len = (w * h * 4) as usize;
        let frame = &mut movie.memory.bytes_mut()[slot * len..(slot + 1) * len];
        if movie.player.next(frame, w as usize * 4).is_none() {
            if let Some(due) = asked_again(movie.player.over(), now) {
                movie.due = due;
                self.tick_in(due - now);
            }
            return;
        }
        movie.shown = Some(slot);
        movie.due += movie.every;
        if movie.due < now {
            movie.due = now + movie.every;
        }
        for s in screens.iter_mut() {
            if let Some(f) = s.film.as_mut().filter(|f| !f.waiting) {
                put_up(f, movie, slot, &qh, s.name);
            }
        }
        drop(state);
        let _ = self.conn.flush();
    }

    fn tick_in(self: &Rc<Self>, wait: Duration) {
        if self.timer.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        self.events.after(wait, move || {
            if let Some(me) = weak.upgrade() {
                me.timer.set(false);
                me.tick();
            }
        });
    }

    fn check_battery(self: &Rc<Self>) {
        let mut state = self.state.borrow_mut();
        let Some(movie) = state.movie.as_mut() else { return };
        let was = std::mem::replace(&mut movie.on_battery, charge::on_battery());
        if movie.on_battery || Instant::now() > movie.due + REST_AFTER {
            movie.player.rest();
        }
        drop(state);
        if was {
            self.tick();
        }
    }
}

fn start_movie(state: &State, qh: &QueueHandle<State>) -> Option<Movie> {
    let path = state.look.picture.as_deref().filter(|p| moving::is_moving(p))?;
    if !graphics::moving() || state.viewporter.is_none() || state.subsurfaces.is_none() {
        return None;
    }
    let clip = crate::common::files::gently(path, super::wallpaper::PICTURE_WAIT, moving::Clip::open).flatten()?;
    let player = moving::Player::of(clip)?;
    let info = player.info();
    let (w, h) = (info.width as i32, info.height as i32);
    let len = (w * h * 4) as usize;
    let memory = shm::Memory::new(c"trindle-wallpaper", len * 2)?;
    let pool = state.shm.create_pool(memory.fd(), memory.len() as i32, qh, ());
    let buffers = [0, 1].map(|i| pool.create_buffer((i * len) as i32, w, h, w * 4, wl_shm::Format::Xrgb8888, qh, Slot(i)));
    pool.destroy();
    Some(Movie {
        path: path.to_path_buf(),
        player,
        size: (w, h),
        memory,
        buffers,
        busy: [false; 2],
        shown: None,
        every: Duration::from_secs_f64(1.0 / info.fps),
        due: Instant::now(),
        on_battery: charge::on_battery(),
    })
}

fn stop_movie(state: &mut State) {
    for s in &mut state.screens {
        if let Some(f) = s.film.take() {
            drop_film(f);
        }
    }
    if let Some(m) = state.movie.take() {
        for b in &m.buffers {
            b.destroy();
        }
        drop(m);
        // SAFETY: as in `draw`; the player's memory is handed back.
        unsafe { libc::malloc_trim(0) };
    }
}

fn drop_film(f: Film) {
    f.viewport.destroy();
    f.sub.destroy();
    f.surface.destroy();
}

fn put_up(f: &mut Film, movie: &mut Movie, slot: usize, qh: &QueueHandle<State>, name: u32) {
    let (w, h) = movie.size;
    f.surface.attach(Some(&movie.buffers[slot]), 0, 0);
    f.surface.damage_buffer(0, 0, w, h);
    f.surface.frame(qh, name);
    f.surface.commit();
    f.waiting = true;
    movie.busy[slot] = true;
}

fn placement(fit: &str, (vw, vh): (f64, f64), (w, h): (f64, f64), scale: f64) -> ((f64, f64, f64, f64), (i32, i32, i32, i32)) {
    let (kx, ky) = match fit {
        "stretch" => (w / vw, h / vh),
        "fit" => { let k = (w / vw).min(h / vh); (k, k) }
        "center" | "tile" => (1.0 / scale, 1.0 / scale),
        _ => { let k = (w / vw).max(h / vh); (k, k) }
    };
    let (ox, oy) = ((w - vw * kx) / 2.0, (h - vh * ky) / 2.0);
    let (x0, y0) = (ox.max(0.0), oy.max(0.0));
    let (x1, y1) = ((ox + vw * kx).min(w), (oy + vh * ky).min(h));
    let sx = ((x0 - ox) / kx).clamp(0.0, vw);
    let sy = ((y0 - oy) / ky).clamp(0.0, vh);
    let sw = ((x1 - x0) / kx).min(vw - sx);
    let sh = ((y1 - y0) / ky).min(vh - sy);
    let (x0, y0, x1, y1) = (x0.round(), y0.round(), x1.round(), y1.round());
    ((sx, sy, sw, sh), (x0 as i32, y0 as i32, (x1 - x0) as i32, (y1 - y0) as i32))
}

fn draw_movie(state: &mut State, qh: &QueueHandle<State>, i: usize) {
    let (Some(movie), Some(subs), Some(viewporter)) = (state.movie.as_mut(), &state.subsurfaces, &state.viewporter) else { return };
    let s = &mut state.screens[i];
    let (w, h) = s.size;
    let Some(parent) = &s.viewport else { return };
    let plain = Look { picture: None, fit: String::new(), color: state.look.color.clone() };
    let Some(img) = paint(&plain, 1, 1) else { return };
    let Some(buffer) = shm::pixmap(&state.shm, qh, &img, c"trindle-wallpaper") else { return };
    s.surface.set_buffer_scale(1);
    parent.set_destination(w, h);
    s.surface.attach(Some(&buffer), 0, 0);
    s.surface.damage_buffer(0, 0, 1, 1);
    let scale = if s.scale120 > 0 { s.scale120 as f64 / 120.0 } else { s.scale.max(1) as f64 };
    let (vw, vh) = movie.size;
    let ((sx, sy, sw, sh), (x, y, dw, dh)) = placement(&state.look.fit, (vw as f64, vh as f64), (w as f64, h as f64), scale);
    if dw <= 0 || dh <= 0 || sw <= 0.0 || sh <= 0.0 {
        if let Some(f) = s.film.take() {
            drop_film(f);
        }
        s.surface.commit();
        return;
    }
    let fresh = s.film.is_none();
    let f = s.film.get_or_insert_with(|| {
        let surface = state.compositor.create_surface(qh, ());
        let region = state.compositor.create_region(qh, ());
        surface.set_input_region(Some(&region));
        region.destroy();
        let sub = subs.get_subsurface(&surface, &s.surface, qh, ());
        sub.set_desync();
        let viewport = viewporter.get_viewport(&surface, qh, ());
        Film { surface, sub, viewport, waiting: false }
    });
    f.viewport.set_source(sx, sy, sw, sh);
    f.viewport.set_destination(dw, dh);
    match movie.shown.filter(|_| fresh) {
        Some(slot) => put_up(f, movie, slot, qh, s.name),
        None => f.surface.commit(),
    }
    f.sub.set_position(x, y);
    s.surface.commit();
}

fn add_screen(state: &mut State, qh: &QueueHandle<State>, registry: &wl_registry::WlRegistry, name: u32, version: u32) {
    let output = registry.bind::<wl_output::WlOutput, _, _>(name, version.min(4), qh, name);
    let surface = state.compositor.create_surface(qh, ());
    let region = state.compositor.create_region(qh, ());
    surface.set_input_region(Some(&region));
    region.destroy();
    let layer = state.layers.get_layer_surface(&surface, Some(&output), zwlr_layer_shell_v1::Layer::Background,
                                               "wallpaper".into(), qh, name);
    layer.set_anchor(Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right);
    layer.set_exclusive_zone(-1);
    layer.set_size(0, 0);
    let (viewport, fractional) = match (&state.viewporter, &state.fractions) {
        (Some(v), Some(f)) => (Some(v.get_viewport(&surface, qh, ())), Some(f.get_fractional_scale(&surface, qh, name))),
        _ => (None, None),
    };
    surface.commit();
    state.screens.push(Screen {
        name, output, surface, layer, viewport, fractional, size: (0, 0), scale: 1, scale120: 0, configured: false, film: None,
    });
}

fn remove_screen(state: &mut State, name: u32) {
    if let Some(i) = state.screens.iter().position(|s| s.name == name) {
        let s = state.screens.remove(i);
        if let Some(f) = s.fractional {
            f.destroy();
        }
        if let Some(v) = s.viewport {
            v.destroy();
        }
        s.layer.destroy();
        s.surface.destroy();
        if s.output.version() >= 3 {
            s.output.release();
        }
        if let Some(f) = s.film {
            drop_film(f);
        }
    }
}

fn draw(state: &mut State, qh: &QueueHandle<State>, i: usize) {
    let s = &state.screens[i];
    let (w, h) = s.size;
    if !s.configured || w <= 0 || h <= 0 {
        return;
    }
    if state.movie.is_some() {
        return draw_movie(state, qh, i);
    }
    let fine = s.viewport.is_some() && s.scale120 > 0;
    let (pw, ph) = if fine {
        (((w as u32 * s.scale120 + 60) / 120) as i32, ((h as u32 * s.scale120 + 60) / 120) as i32)
    } else {
        (w * s.scale.max(1), h * s.scale.max(1))
    };
    let Some(img) = paint(&state.look, pw, ph) else { return };
    let Some(buffer) = shm::pixmap(&state.shm, qh, &img, c"trindle-wallpaper") else { return };
    drop(img);
    let s = &state.screens[i];
    match (&s.viewport, fine) {
        (Some(v), true) => {
            s.surface.set_buffer_scale(1);
            v.set_destination(w, h);
        }
        (v, _) => {
            if let Some(v) = v {
                v.set_destination(-1, -1);
            }
            s.surface.set_buffer_scale(s.scale.max(1));
        }
    }
    let opaque = state.compositor.create_region(qh, ());
    opaque.add(0, 0, w, h);
    s.surface.set_opaque_region(Some(&opaque));
    opaque.destroy();
    s.surface.attach(Some(&buffer), 0, 0);
    s.surface.damage_buffer(0, 0, pw, ph);
    s.surface.commit();
    // SAFETY: a plain call into the C library's allocator, which it makes
    // safe to call at any time; no memory of ours is touched.
    unsafe { libc::malloc_trim(0) };
}

const FALLBACK: Colour = Colour::rgb(0.035, 0.035, 0.043);

fn colour(text: &str) -> Colour {
    Colour::parse(text).unwrap_or(FALLBACK)
}

fn load(path: &Path) -> Option<image::DynamicImage> {
    crate::common::files::gently(path, super::wallpaper::PICTURE_WAIT, load_here).flatten()
}

fn load_here(path: &Path) -> Option<image::DynamicImage> {
    if moving::is_moving(path) {
        let (fw, fh, rgb) = moving::first_frame(path, 1)?;
        return picture::from_rgb(fw, fh, rgb);
    }
    picture::load(path)
}

fn paint(look: &Look, w: i32, h: i32) -> Option<Pixmap> {
    let mut canvas = Pixmap::new(w as u32, h as u32)?;
    canvas.fill(colour(&look.color).to_skia());
    let Some(img) = look.picture.as_deref().and_then(load) else { return Some(canvas) };
    let (iw, ih) = (img.width() as f64, img.height() as f64);
    let (sw, sh) = (w as f64, h as f64);
    let centred = |pw: f64, ph: f64| (((sw - pw) / 2.0).round() as i32, ((sh - ph) / 2.0).round() as i32);
    match look.fit.as_str() {
        "stretch" => picture::draw(&mut canvas.as_mut(), &img, 0, 0, w as u32, h as u32)?,
        "fit" | "fill" => {
            let k = if look.fit == "fit" { (sw / iw).min(sh / ih) } else { (sw / iw).max(sh / ih) };
            let (pw, ph) = ((iw * k).round().max(1.0), (ih * k).round().max(1.0));
            let (x, y) = centred(pw, ph);
            picture::draw(&mut canvas.as_mut(), &img, x, y, pw as u32, ph as u32)?
        }
        "tile" => {
            let src = picture::to_pixmap(&img)?;
            drop(img);
            let tiles = Pattern::new(src.as_ref(), SpreadMode::Repeat, FilterQuality::Nearest, 1.0, Transform::identity());
            let all = Rect::from_xywh(0.0, 0.0, sw as f32, sh as f32)?;
            canvas.fill_rect(all, &tiny_skia::Paint { shader: tiles, ..Default::default() }, Transform::identity(), None);
        }
        "center" => {
            let (x, y) = centred(iw, ih);
            picture::draw(&mut canvas.as_mut(), &img, x, y, img.width(), img.height())?
        }
        _ => picture::draw(&mut canvas.as_mut(), &img, 0, 0, img.width(), img.height())?,
    }
    Some(canvas)
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, qh: &QueueHandle<Self>) {
        match event {
            wl_registry::Event::Global { name, interface, version } if interface == "wl_output" => {
                add_screen(state, qh, registry, name, version);
            }
            wl_registry::Event::GlobalRemove { name } => remove_screen(state, name),
            _ => {}
        }
    }
}

impl Dispatch<wl_output::WlOutput, u32> for State {
    fn event(state: &mut Self, _: &wl_output::WlOutput, event: wl_output::Event, name: &u32, _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_output::Event::Scale { factor } = event {
            if let Some(i) = state.screens.iter().position(|s| s.name == *name) {
                if state.screens[i].scale != factor.max(1) {
                    state.screens[i].scale = factor.max(1);
                    draw(state, qh, i);
                }
            }
        }
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, u32> for State {
    fn event(state: &mut Self, layer: &ZwlrLayerSurfaceV1, event: zwlr_layer_surface_v1::Event, name: &u32,
             _: &Connection, qh: &QueueHandle<Self>) {
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, width, height } => {
                layer.ack_configure(serial);
                if let Some(i) = state.screens.iter().position(|s| s.name == *name) {
                    let size = (width as i32, height as i32);
                    let s = &mut state.screens[i];
                    if s.configured && s.size == size {
                        s.surface.commit();
                    } else {
                        s.size = size;
                        s.configured = true;
                        draw(state, qh, i);
                    }
                }
            }
            zwlr_layer_surface_v1::Event::Closed => remove_screen(state, *name),
            _ => {}
        }
    }
}

impl Dispatch<WpFractionalScaleV1, u32> for State {
    fn event(state: &mut Self, _: &WpFractionalScaleV1, event: wp_fractional_scale_v1::Event, name: &u32,
             _: &Connection, qh: &QueueHandle<Self>) {
        if let wp_fractional_scale_v1::Event::PreferredScale { scale } = event {
            if let Some(i) = state.screens.iter().position(|s| s.name == *name) {
                if state.screens[i].scale120 != scale {
                    state.screens[i].scale120 = scale;
                    draw(state, qh, i);
                }
            }
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, shm::Held> for State {
    fn event(_: &mut Self, buffer: &wl_buffer::WlBuffer, event: wl_buffer::Event, frame: &shm::Held,
             _: &Connection, _: &QueueHandle<Self>) {
        if let wl_buffer::Event::Release = event {
            shm::released(buffer, frame);
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, Slot> for State {
    fn event(state: &mut Self, _: &wl_buffer::WlBuffer, event: wl_buffer::Event, slot: &Slot,
             _: &Connection, _: &QueueHandle<Self>) {
        if let (wl_buffer::Event::Release, Some(m)) = (event, state.movie.as_mut()) {
            m.busy[slot.0] = false;
        }
    }
}

impl Dispatch<wl_callback::WlCallback, u32> for State {
    fn event(state: &mut Self, _: &wl_callback::WlCallback, event: wl_callback::Event, name: &u32,
             _: &Connection, _: &QueueHandle<Self>) {
        if let wl_callback::Event::Done { .. } = event {
            if let Some(f) = state.screens.iter_mut().find(|s| s.name == *name).and_then(|s| s.film.as_mut()) {
                f.waiting = false;
            }
        }
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for State {
            fn event(_: &mut Self, _: &$t, _: <$t as Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
        }
    )*};
}
ignore!(wl_compositor::WlCompositor, wl_shm::WlShm, wl_shm_pool::WlShmPool, wl_surface::WlSurface, wl_region::WlRegion,
        ZwlrLayerShellV1, WpViewporter, WpViewport, WpFractionalScaleManagerV1, wl_subcompositor::WlSubcompositor,
        wl_subsurface::WlSubsurface);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_colour_fills_the_screen() {
        let look = Look { picture: None, fit: "fill".into(), color: "#102030".into() };
        let img = paint(&look, 4, 3).expect("drawn");
        assert_eq!(&img.data()[0..4], &[0x10, 0x20, 0x30, 0xff]);
    }

    #[test]
    fn a_moving_wallpaper_is_placed_by_the_fit() {
        let video = (1920.0, 1080.0);
        let (src, dst) = placement("fill", video, (1000.0, 1000.0), 1.0);
        assert_eq!(dst, (0, 0, 1000, 1000));
        assert!((src.0 - 420.0).abs() < 1.0 && (src.2 - 1080.0).abs() < 1.0 && src.3 == 1080.0);
        let (src, dst) = placement("fit", video, (1000.0, 1000.0), 1.0);
        assert_eq!(dst.0, 0);
        assert_eq!(dst.2, 1000);
        assert!(dst.1 > 0 && dst.3 < 1000 && near(src, (0.0, 0.0, 1920.0, 1080.0)));
        let (src, dst) = placement("center", video, (800.0, 400.0), 2.0);
        assert_eq!(dst, (0, 0, 800, 400));
        assert!(near(src, (160.0, 140.0, 1600.0, 800.0)));
    }

    fn near(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        [a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3].iter().all(|d| d.abs() < 0.01)
    }

    #[test]
    fn a_film_that_gave_no_frame_is_asked_again_unless_its_decoder_is_over() {
        let now = Instant::now();
        assert_eq!(asked_again(false, now), Some(now + AGAIN_AFTER));
        assert_eq!(asked_again(true, now), None);
        assert!(AGAIN_AFTER >= Duration::from_millis(200) && AGAIN_AFTER < REST_AFTER, "asked again before the film is taken to be at rest");
        let code = crate::common::source::code(include_str!("backdrop.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        let tick = &code[code.find("    fn tick(").expect("tick is gone")..];
        let tick = &tick[..tick.find("\n    }\n").expect("the end of tick")];
        let none = &tick[tick.find(".next(frame").expect("the asking for a frame")..];
        let none = &none[..none.find("return;").expect("where no frame ends")];
        assert!(none.contains("asked_again(movie.player.over(), now)") && none.contains("self.tick_in("), "no frame, and no time set to ask again");
        assert_eq!(code.matches("self.events.after(").count(), 1, "a wait for the film is set from a second place");
    }

    #[test]
    fn a_film_on_battery_shows_its_first_frame_and_stands_still_there() {
        assert!(!stands_still(true, false), "a film started on battery was given no frame at all");
        assert!(stands_still(true, true));
        assert!(!stands_still(false, false) && !stands_still(false, true));
        let code = crate::common::source::code(include_str!("backdrop.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        let tick = &code[code.find("    fn tick(").expect("tick is gone")..];
        let tick = &tick[..tick.find("\n    }\n").expect("the end of tick")];
        assert!(tick.contains("stands_still(movie.on_battery, movie.shown.is_some())") && !tick.contains("if movie.on_battery"), "{tick}");
    }

    #[test]
    fn a_colour_that_cannot_be_read_falls_back_to_the_dark_one() {
        assert_eq!(colour("not a colour"), FALLBACK);
    }
}
