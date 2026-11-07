//! The resting clock.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tiny_skia::{Pixmap, PixmapMut};
use wayland_client::QueueHandle;
use wayland_client::protocol::{wl_shm, wl_surface};

use crate::common::shm;
use crate::toolkit::colour::Colour;
use crate::toolkit::text::Fonts;
use crate::toolkit::theme::Theme;

use super::{Kept, Screen, State};
use super::pen::{look, Pen};

pub(in crate::session) fn drift(seconds: f64, w: f64, h: f64) -> (f64, f64) {
    let (mx, my) = (w * 0.18, h * 0.16);
    let bounce = |t: f64, from: f64, to: f64| {
        let span = (to - from).max(1.0);
        let p = (t % (2.0 * span)).abs();
        from + if p <= span { p } else { 2.0 * span - p }
    };
    (bounce(seconds * DRIFT_SPEED, mx, w - mx), bounce(seconds * DRIFT_SPEED * 0.73 + h * 0.3, my, h - my))
}

const DRIFT_SPEED: f64 = 18.0;

#[cfg(test)]
pub(super) fn draw_resting(pen: &mut Pen, w: f64, h: f64, seconds: f64) {
    let (x, y) = drift(seconds, w, h);
    draw_clock(pen, x, y);
}

pub(in crate::session) fn clock_lines(now: &crate::common::clock::Local) -> (String, String) {
    (now.line(trindle_toolkit::lang::format::Pattern::Clock), now.line(trindle_toolkit::lang::format::Pattern::DayMonthPadded))
}

pub(in crate::session) fn clock_says(now: &crate::common::clock::Local) -> String {
    let (time, date) = clock_lines(now);
    format!("{time} {date}")
}

fn draw_clock(pen: &mut Pen, x: f64, y: f64) {
    let (well, words, quiet) = (pen.t.well, pen.t.fg, pen.t.muted);
    pen.px.fill(well.to_skia());
    let now = crate::common::clock::Local::now();
    let (time, date) = clock_lines(&now);
    pen.centred(&time, x, y, 96.0, false, words);
    pen.centred(&date, x, y + 48.0, 22.0, false, quiet);
}

const CLOCK_ABOVE: f64 = 104.0;
const CLOCK_BELOW: f64 = 64.0;
const CLOCK_BESIDE: f64 = 12.0;

pub(in crate::session) fn rest_step() -> Duration {
    if crate::common::graphics::moving() { Duration::from_millis(200) } else { Duration::from_secs(1) }
}

pub(super) fn rest_frame(state: &mut State, qh: &QueueHandle<State>, seconds: f64) {
    let theme = look();
    let (shm, fonts, faces) = (&state.shm, &state.fonts, &mut state.faces);
    for screen in &mut state.screens {
        let (w, h, scale) = (screen.size.0, screen.size.1, screen.scale.max(1));
        let Some(surface) = screen.surface.clone() else { continue };
        if w <= 0 || h <= 0 {
            continue;
        }
        let (pw, ph) = (w * scale, h * scale);
        let (fits, gone): (Vec<Kept>, Vec<Kept>) = std::mem::take(&mut screen.kept).into_iter().partition(|k| k.size == (pw, ph));
        screen.kept = fits;
        state.old.extend(gone);
        let Some(at) = free_picture(screen, shm, qh, (pw, ph), theme.well) else { continue };
        let kept = &mut screen.kept[at];

        let at = drift(seconds, f64::from(w), f64::from(h));
        let Some(face) = current_face(faces, scale, fonts, &theme) else { continue };
        put_clock(kept, &surface, face, at, scale, theme.well);
    }
}

fn free_picture(screen: &mut Screen, shm: &wl_shm::WlShm, qh: &QueueHandle<State>, size: (i32, i32), dark: Colour) -> Option<usize> {
    let free = screen.kept.iter().position(|k| !k.busy.load(Ordering::Relaxed));
    match free {
        Some(at) => Some(at),
        None if screen.kept.len() < 2 => {
            let new = new_kept(shm, qh, size, dark)?;
            screen.kept.push(new);
            Some(screen.kept.len() - 1)
        }
        None => None,
    }
}

fn current_face<'a>(faces: &'a mut Vec<Face>, scale: i32, fonts: &Fonts, theme: &Theme) -> Option<&'a Face> {
    let says = clock_says(&crate::common::clock::Local::now());
    faces.retain(|f| f.says == says);
    if !faces.iter().any(|f| f.scale == scale) {
        let face = Face::draw(says, scale, fonts, theme)?;
        faces.push(face);
    }
    faces.iter().find(|f| f.scale == scale)
}

fn put_clock(kept: &mut Kept, surface: &wl_surface::WlSurface, face: &Face, (x, y): (f64, f64), scale: i32, dark: Colour) {
    let s = f64::from(scale);
    let (pw, ph) = kept.size;
    let (across, img) = (face.across, &face.img);
    let (bw, bh) = (img.width() as i32, img.height() as i32);
    let place = [((x - across / 2.0) * s).round() as i32, ((y - CLOCK_ABOVE) * s).round() as i32, bw, bh];
    let was = kept.clock.replace(place);
    let bytes = kept.memory.bytes_mut();
    if let Some(was) = was {
        shm::fill(bytes, (pw, ph), was, dark.to_skia());
    }
    shm::put(bytes, (pw, ph), img.data(), place);

    surface.set_buffer_scale(scale);
    surface.attach(Some(&kept.buffer), 0, 0);
    match was {
        None => surface.damage_buffer(0, 0, pw, ph),
        Some(was) => {
            for part in [was, place] {
                surface.damage_buffer(part[0], part[1], part[2], part[3]);
            }
        }
    }
    surface.commit();
    kept.busy.store(true, Ordering::Relaxed);
}

pub(super) struct Face {
    says: String,
    scale: i32,
    across: f64,
    img: Pixmap,
}

impl Face {
    fn draw(says: String, scale: i32, fonts: &Fonts, theme: &Theme) -> Option<Face> {
        let now = crate::common::clock::Local::now();
        let wide = {
            let mut none = [0u8; 4];
            let probe = Pen { px: PixmapMut::from_bytes(&mut none, 1, 1)?, scale: scale as f32, fonts, t: theme };
            let (time, date) = clock_lines(&now);
            [(time, 96.0), (date, 22.0)].iter()
                .map(|(text, size)| probe.width(&probe.layout(text, *size, false))).fold(0.0, f64::max)
        };
        let across = wide + 2.0 * CLOCK_BESIDE;
        let s = f64::from(scale);
        let mut img = Pixmap::new(((across * s).ceil() as u32).max(1), (((CLOCK_ABOVE + CLOCK_BELOW) * s).ceil() as u32).max(1))?;
        let mut pen = Pen { px: img.as_mut(), scale: scale as f32, fonts, t: theme };
        draw_clock(&mut pen, across / 2.0, CLOCK_ABOVE);
        Some(Face { says, scale, across, img })
    }
}

fn new_kept(shm: &wl_shm::WlShm, qh: &QueueHandle<State>, size: (i32, i32), dark: Colour) -> Option<Kept> {
    let (w, h) = size;
    let mut memory = shm::Memory::new(c"trindle-lock", (w * h * 4) as usize)?;
    shm::fill(memory.bytes_mut(), size, [0, 0, w, h], dark.to_skia());
    let pool = shm.create_pool(memory.fd(), memory.len() as i32, qh, ());
    let busy = Arc::new(AtomicBool::new(false));
    let buffer = pool.create_buffer(0, w, h, w * 4, wl_shm::Format::Argb8888, qh, busy.clone());
    pool.destroy();
    Some(Kept { memory, buffer, size, busy, clock: None })
}
