//! trindle-screensaver: after the computer has been left alone a while, the
//! screen goes dark with only the time on it, moving slowly about so nothing
//! is burned into the screen. Any key, click or real movement of the mouse
//! puts it away again. It does not lock: locking has its own time, and the
//! lock screen rests the same way until someone comes back.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use tiny_skia::{Pixmap, PixmapMut};

use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::input::Input;
use crate::toolkit::text::{Fonts, Style};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel, Window};

fn sheet(name: &str) -> Colour {
    crate::toolkit::theme::named(name, false)
}

const TIME_SIZE: f32 = 96.0;
const DATE_SIZE: f32 = 22.0;
const GAP: f32 = 4.0;

const NUDGE: f32 = 12.0;
const FADE: Duration = Duration::from_millis(200);
const FRAME: Duration = Duration::from_millis(16);

fn pid_file() -> std::path::PathBuf {
    crate::common::files::scratch().join("trindle-screensaver.pid")
}

pub fn began_file() -> std::path::PathBuf {
    crate::common::files::scratch().join("trindle-screensaver.began")
}

pub fn moving_for() -> Option<f64> {
    let began: f64 = std::fs::read_to_string(began_file()).ok()?.trim().parse().ok()?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?.as_secs_f64();
    (now >= began).then_some(now - began)
}

fn stop() -> i32 {
    let pid: Option<i32> = std::fs::read_to_string(pid_file()).ok().and_then(|p| p.trim().parse().ok());
    if let Some(pid) = pid.filter(|p| *p > 1) {
        let mine = std::fs::read_to_string(format!("/proc/{pid}/cmdline")).is_ok_and(|c| c.contains("screensaver"));
        if mine {
            // SAFETY: a signal to a process number just checked to be a

            unsafe { libc::kill(pid, libc::SIGTERM) };
        }
    }
    let _ = std::fs::remove_file(pid_file());
    let _ = std::fs::remove_file(began_file());
    0
}

pub fn main(args: Vec<String>) -> i32 {
    if args.iter().any(|a| a == "--stop") {
        return stop();
    }
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else {
        eprintln!("trindle-screensaver: no Wayland display to open; run it inside a Trindle session");
        return 1;
    };
    let _ = std::fs::write(pid_file(), format!("{}\n", std::process::id()));
    if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        let _ = std::fs::write(began_file(), format!("{:.3}\n", now.as_secs_f64()));
    }
    let code = match Resting::open(&events, &display) {
        Some(_resting) => {
            events.run();
            0
        }
        None => {
            eprintln!("trindle-screensaver: this compositor does not offer the layer-shell protocol; run it inside a Trindle session");
            1
        }
    };
    let _ = std::fs::remove_file(pid_file());
    let _ = std::fs::remove_file(began_file());
    code
}

struct Resting {
    events: Loop,
    window: Rc<Window>,

    shown: Rc<Cell<f32>>,

    leaving: Cell<bool>,

    first: Cell<Option<(f32, f32)>>,
}

impl Resting {
    fn open(events: &Loop, display: &Rc<Display>) -> Option<Rc<Resting>> {
        let moving = crate::common::graphics::moving();
        let kind = Kind::Panel(Panel {
            layer: Layer::Overlay, edges: Edges::ALL, keep: -1, margins: [0; 4], keys: Keys::Always,
            name: "trindle-screensaver".into(), screen: None,
        });
        let shown = Rc::new(Cell::new(if moving { 0.0 } else { 1.0 }));
        let began = Instant::now();
        let clock = Rc::new(Clock { fonts: Fonts::new(), face: RefCell::new(None), at: Cell::new(None) });
        let (c, seen) = (clock.clone(), shown.clone());
        let window = Rc::new(display.open(kind, (0, 0), move |canvas, scale| {
            c.paint(canvas, scale, began.elapsed().as_secs_f64(), seen.get());
        })?);

        let (c, seen) = (clock, shown.clone());
        window.patch_moves();
        window.on_patch(move |canvas, scale| c.step(canvas, scale, began.elapsed().as_secs_f64(), seen.get()));
        let me = Rc::new(Resting { events: events.clone(), window, shown, leaving: Cell::new(false), first: Cell::new(None) });

        let w = Rc::downgrade(&me.window);
        events.every(super::lock::rest::rest_step(), move || match w.upgrade() {
            Some(w) => {
                w.patch();
                Flow::Continue
            }
            None => Flow::Stop,
        });
        if moving {
            me.fade(true, || {});
        }
        me.leave_when_someone_is_back();
        let ev = events.clone();
        me.window.on_close(move || ev.quit());

        let ev = events.clone();
        events.on_signal(libc::SIGTERM, move || {
            ev.quit();
            Flow::Stop
        });
        Some(me)
    }

    fn leave_when_someone_is_back(self: &Rc<Self>) {
        let m = Rc::downgrade(self);
        self.window.on_input(move |input| {
            let Some(m) = m.upgrade() else { return };
            match input {
                Input::Key { .. } | Input::Press { .. } | Input::TouchDown { .. } => m.leave(),
                Input::PointerIn { x, y } | Input::PointerMove { x, y } => {
                    let from = m.first.get().unwrap_or((x, y));
                    m.first.set(Some(from));
                    if (x - from.0).hypot(y - from.1) > NUDGE {
                        m.leave();
                    }
                }
                _ => {}
            }
        });
    }

    fn leave(self: &Rc<Self>) {
        if self.leaving.replace(true) {
            return;
        }
        let ev = self.events.clone();
        if crate::common::graphics::moving() {
            self.fade(false, move || ev.quit());
        } else {
            ev.quit();
        }
    }

    fn fade(self: &Rc<Self>, inwards: bool, done: impl FnOnce() + 'static) {
        let (from, start, me) = (self.shown.get(), Instant::now(), Rc::downgrade(self));
        let mut done = Some(done);
        self.events.every(FRAME, move || {
            let Some(me) = me.upgrade() else { return Flow::Stop };

            if inwards && me.leaving.get() {
                return Flow::Stop;
            }
            let t = (start.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.0);
            me.shown.set(if inwards { from + (1.0 - from) * t } else { from * (1.0 - t) });
            me.window.redraw();
            if t < 1.0 {
                return Flow::Continue;
            }
            if let Some(done) = done.take() {
                done();
            }
            Flow::Stop
        });
    }
}

const ROOM: f32 = 6.0;

struct Face {

    says: String,
    scale: f32,
    img: Pixmap,
}

struct Clock {
    fonts: Fonts,

    face: RefCell<Option<Face>>,

    at: Cell<Option<[i32; 4]>>,
}

impl Clock {

    fn placed(&self, (width, height): (u32, u32), scale: f32, seconds: f64) -> Option<[i32; 4]> {
        let now = crate::common::clock::Local::now();
        let says = super::lock::rest::clock_says(&now);
        let (time_line, date_line) = super::lock::rest::clock_lines(&now);
        let mut face = self.face.borrow_mut();
        if !face.as_ref().is_some_and(|f| f.says == says && f.scale == scale) {
            let (ink, quiet) = (sheet("trindle-text"), sheet("trindle-text-quiet"));
            let time = self.fonts.layout(&time_line, &Style { weight: 300, ..Style::new(TIME_SIZE * scale) }, None);
            let date = self.fonts.layout(&date_line, &Style::new(DATE_SIZE * scale), None);
            let room = (ROOM * scale).ceil();
            let across = time.width.max(date.width).ceil() + 2.0 * room;
            let down = (time.height + GAP * scale + date.height).ceil() + 2.0 * room;
            let mut img = Pixmap::new(across as u32, down as u32)?;
            img.fill(sheet("trindle-well").to_skia());
            let mut canvas = img.as_mut();
            self.fonts.draw(&mut canvas, &time, (across - time.width) / 2.0, room, ink);
            self.fonts.draw(&mut canvas, &date, (across - date.width) / 2.0, room + time.height + GAP * scale, quiet);
            *face = Some(Face { says, scale, img });
        }
        let img = &face.as_ref()?.img;
        let s = f64::from(scale);
        let (x, y) = super::lock::rest::drift(seconds, f64::from(width) / s, f64::from(height) / s);
        let (w, h) = (img.width() as i32, img.height() as i32);
        Some([(x * s).round() as i32 - w / 2, (y * s).round() as i32 - h / 2, w, h])
    }

    fn paint(&self, canvas: &mut PixmapMut, scale: f32, seconds: f64, shown: f32) {
        canvas.fill(sheet("trindle-well").to_skia());
        let at = self.placed((canvas.width(), canvas.height()), scale, seconds);
        if let (Some(at), Some(face)) = (at, self.face.borrow().as_ref()) {
            canvas.draw_pixmap(at[0], at[1], face.img.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
        }
        self.at.set(at);
        if shown < 1.0 {

            let k = shown.clamp(0.0, 1.0);
            for b in canvas.data_mut() {
                *b = (f32::from(*b) * k).round() as u8;
            }
        }
    }

    fn step(&self, canvas: &mut PixmapMut, scale: f32, seconds: f64, shown: f32) -> Option<Vec<[i32; 4]>> {
        if shown < 1.0 {
            return None;
        }
        let was = self.at.get()?;
        let size = (canvas.width() as i32, canvas.height() as i32);
        let at = self.placed((canvas.width(), canvas.height()), scale, seconds)?;
        let face = self.face.borrow();
        crate::common::shm::fill(canvas.data_mut(), size, was, sheet("trindle-well").to_skia());
        crate::common::shm::put(canvas.data_mut(), size, face.as_ref()?.img.data(), at);
        self.at.set(Some(at));
        Some(vec![was, at])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_screensavers_clock_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        let clock = Clock { fonts: Fonts::new(), face: RefCell::new(None), at: Cell::new(None) };
        drawn::none(drawn::in_look("the screensaver's clock", false, 2, || {

            *clock.face.borrow_mut() = None;
            let mut pix = Pixmap::new(1280, 800).unwrap();
            clock.paint(&mut pix.as_mut(), 1.0, 30.0, 1.0);
            assert!(clock.at.get().is_some(), "the clock was not drawn");
        }));
    }
}
