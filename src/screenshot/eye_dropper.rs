//! `trindle-screenshot --eye-dropper`: the Eye dropper, one colour off the
//! screen for a program that asked the portal for it, as the Eyedropper
//! Flatpak does.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use tiny_skia::{FillRule, FilterQuality, Mask, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, Rect as Box, Stroke, Transform};

use super::area::{display_in_picture, Rect, View};
use crate::common::capture::{self, Picture};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::Loop;
use crate::toolkit::input::{Button, Cursor, Input, Keysym};
use crate::toolkit::text::{Fonts, Style};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel, Window};

pub const CHOSEN: i32 = 0;
pub const GAVE_UP: i32 = 1;
pub const FAILED: i32 = 2;

pub(crate) const ICON_FALLBACK: &str = "color-select-symbolic";

const LOUPE: f32 = 120.0;
const ZOOM: f32 = 10.0;
const OFFSET: f32 = 24.0;

pub(super) const PILL_HEIGHT: f32 = 24.0;

thread_local! {

    static LOOK: crate::toolkit::theme::Theme = crate::toolkit::theme::Theme::chosen();

    static SHADOWS: crate::toolkit::shadow::Shadows = crate::toolkit::shadow::Shadows::default();
}

pub(super) fn float_shadow(canvas: &mut PixmapMut, scale: f32, r: (f32, f32, f32, f32), radius: f32, fall: crate::toolkit::theme::Fall) {
    let falls = [crate::toolkit::theme::Fall { y: fall.y * scale, blur: fall.blur * scale, spread: fall.spread * scale, alpha: fall.alpha }];
    let rect = crate::toolkit::widget::Rect { x: r.0 * scale, y: r.1 * scale, w: r.2 * scale, h: r.3 * scale };
    let shadow = LOOK.with(|t| t.shadow);
    SHADOWS.with(|sh| sh.below(canvas, rect, radius * scale, &falls, shadow));
}

pub fn main() -> i32 {
    let picture = match capture::take(None, false) {
        Ok(p) => Rc::new(p),
        Err(e) => {
            eprintln!("trindle-screenshot: {e}");
            return FAILED;
        }
    };
    let Some(still) = still(&picture) else { return FAILED };
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else {
        eprintln!("trindle-screenshot: no Wayland display to open; run it inside a Trindle session");
        return FAILED;
    };
    let code = Rc::new(Cell::new(GAVE_UP));
    let size = (picture.width as f64, picture.height as f64);

    let shown = Rc::new(Cell::new(Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 }));
    let pointer: Rc<Cell<Option<(f32, f32)>>> = Rc::new(Cell::new(None));
    let Some(window) = open_window(&display, still, &picture, &shown, &pointer) else { return FAILED };
    let window = Rc::new(window);

    let finish = finisher(&events, &code);
    let colour_at = colour_finder(&picture, &window, &shown);
    let me = Rc::downgrade(&window);
    let display_ = display.clone();
    let done = RefCell::new(false);
    window.on_input(move |input| {
        let Some(w) = me.upgrade() else { return };
        if *done.borrow() {
            return;
        }
        follow_screen(&display_, &w, &shown, size);
        let pick = match input {
            Input::PointerIn { x, y } | Input::PointerMove { x, y } => {
                pointer.set(Some((x, y)));
                w.redraw();
                None
            }
            Input::Press { button: Button::Left, x, y } => Some(colour_at((x, y))),
            Input::Press { .. } => Some(None),
            Input::Key { sym: Keysym::Escape, .. } => Some(None),
            Input::Key { sym: Keysym::Return | Keysym::KP_Enter, .. } => pointer.get().map(&colour_at),
            _ => None,
        };
        if let Some(colour) = pick {
            *done.borrow_mut() = true;
            finish(colour);
        }
    });
    let ev = events.clone();
    window.on_close(move || ev.quit());
    events.run();
    code.get()
}

fn follow_screen(display: &Display, window: &Window, shown: &Cell<Rect>, size: (f64, f64)) {
    let places = display.places();
    if let Some(this) = window.place().or_else(|| (places.len() == 1).then(|| places[0])) {
        let rect = |p: [i32; 4]| Rect { x: p[0] as f64, y: p[1] as f64, w: p[2] as f64, h: p[3] as f64 };
        let all: Vec<Rect> = places.iter().map(|p| rect(*p)).collect();
        shown.set(display_in_picture(size, &all, rect(this)));
    }
}

fn open_window(display: &Rc<Display>, still: Pixmap, picture: &Rc<Picture>, shown: &Rc<Cell<Rect>>, pointer: &Rc<Cell<Option<(f32, f32)>>>) -> Option<Window> {
    let kind = Kind::Panel(Panel {
        layer: Layer::Overlay, edges: Edges::ALL, keep: -1, margins: [0; 4], keys: Keys::Always,
        name: "trindle-eye-dropper".into(), screen: None,
    });
    let (fonts, at, part, pic) = (Fonts::new(), pointer.clone(), shown.clone(), picture.clone());
    let Some(window) = display.open(kind, (0, 0), move |canvas, scale| {
        paint(canvas, scale, &still, &pic, part.get(), at.get(), &fonts);
    }) else {
        eprintln!("trindle-screenshot: this compositor does not offer the layer-shell protocol; run it inside a Trindle session");
        return None;
    };
    window.set_cursor(Cursor::Crosshair);
    Some(window)
}

fn finisher(events: &Loop, code: &Rc<Cell<i32>>) -> impl Fn(Option<[u8; 3]>) {
    let (events, code) = (events.clone(), code.clone());
    move |colour| {
        match colour {
            Some(rgb) => {
                println!("{}", format_colour(rgb));
                code.set(CHOSEN);
            }
            None => code.set(GAVE_UP),
        }
        events.quit();
    }
}

fn colour_finder(picture: &Rc<Picture>, window: &Rc<Window>, shown: &Rc<Cell<Rect>>) -> impl Fn((f32, f32)) -> Option<[u8; 3]> {
    let (picture, window, shown) = (picture.clone(), Rc::downgrade(window), shown.clone());
    move |p| {
        let w = window.upgrade()?;
        let (x, y) = view(&w.size(), shown.get()).in_picture((p.0 as f64, p.1 as f64));
        picture.rgba_at(x.floor() as i32, y.floor() as i32).map(|[r, g, b, _]| [r, g, b])
    }
}

pub(super) fn still(picture: &Picture) -> Option<Pixmap> {
    let (w, h) = (picture.width as usize, picture.height as usize);
    let row = w * 4;
    let data = if picture.stride == row {
        picture.data.clone()
    } else {
        picture.data.chunks(picture.stride).take(h).flat_map(|r| r[..row].iter().copied()).collect()
    };
    Pixmap::from_vec(data, tiny_skia::IntSize::from_wh(w as u32, h as u32)?)
}

pub(super) fn view(size: &(u32, u32), shown: Rect) -> View {
    View { shown, size: (size.0 as f64, size.1 as f64) }
}

fn paint(canvas: &mut PixmapMut, scale: f32, still: &Pixmap, picture: &Picture, shown: Rect, pointer: Option<(f32, f32)>, fonts: &Fonts) {
    let points = ((canvas.width() as f32 / scale) as u32, (canvas.height() as f32 / scale) as u32);
    let view = view(&points, shown);

    let k = scale / view.scale() as f32;
    let photo = Transform::from_scale(k, k).pre_translate(-shown.x as f32, -shown.y as f32);
    let good = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    canvas.draw_pixmap(0, 0, still.as_ref(), &good, photo, None);
    if let Some(p) = pointer {
        loupe(canvas, scale, still, picture, &view, p, fonts);
    }
}

fn loupe(canvas: &mut PixmapMut, scale: f32, still: &Pixmap, picture: &Picture, view: &View, p: (f32, f32), fonts: &Fonts) {
    let screen = (canvas.width() as f32 / scale, canvas.height() as f32 / scale);
    let r = LOUPE / 2.0;

    let mut c = (p.0 + OFFSET + r, p.1 + OFFSET + r);
    if c.0 + r + 40.0 > screen.0 {
        c.0 = p.0 - OFFSET - r;
    }
    if c.1 + r + 40.0 > screen.1 {
        c.1 = p.1 - OFFSET - r;
    }
    let (px, py) = view.in_picture((p.0 as f64, p.1 as f64));
    let (px, py) = (px.floor() as f32, py.floor() as f32);
    let to_px = Transform::from_scale(scale, scale);
    let paint = |colour: Colour| {
        let mut paint = Paint::default();
        paint.set_color(colour.to_skia());
        paint.anti_alias = true;
        paint
    };

    let Some(circle) = PathBuilder::from_circle(c.0, c.1, r) else { return };
    let Some(mut clip) = Mask::new(canvas.width(), canvas.height()) else { return };

    float_shadow(canvas, scale, (c.0 - r, c.1 - r, LOUPE, LOUPE), r, crate::toolkit::theme::sheet().label_shadow);
    let (dark, light) = LOOK.with(|t| (t.bg, t.fg));
    clip.fill_path(&circle, FillRule::Winding, true, to_px);
    canvas.fill_path(&circle, &paint(dark), FillRule::Winding, to_px, Some(&clip));

    let zoom = to_px.pre_translate(c.0 - (px + 0.5) * ZOOM, c.1 - (py + 0.5) * ZOOM).pre_scale(ZOOM, ZOOM);
    let sharp = PixmapPaint { quality: FilterQuality::Nearest, ..PixmapPaint::default() };
    canvas.draw_pixmap(0, 0, still.as_ref(), &sharp, zoom, Some(&clip));

    let half = ZOOM / 2.0;
    if let Some(square) = Box::from_xywh(c.0 - half, c.1 - half, ZOOM, ZOOM).map(PathBuilder::from_rect) {
        let line = |width: f32| Stroke { width, ..Stroke::default() };

        canvas.stroke_path(&square, &paint(dark), &line(2.0), to_px, None);
        canvas.stroke_path(&square, &paint(light), &line(1.0), to_px, None);
    }

    let Some([red, green, blue, _]) = picture.rgba_at(px as i32, py as i32) else { return };
    pill(canvas, scale, fonts, &format!("#{red:02X}{green:02X}{blue:02X}"), |w| (c.0 - w / 2.0, c.1 + r + 8.0));
}

pub(super) fn pill(canvas: &mut PixmapMut, scale: f32, fonts: &Fonts, words: &str, place: impl FnOnce(f32) -> (f32, f32)) {
    let to_px = Transform::from_scale(scale, scale);
    let k = crate::toolkit::theme::sheet();
    let text = fonts.layout(words, &Style { family: Some(crate::toolkit::text::FAMILY.into()), weight: k.label_weight,
                                            ..Style::new(k.label_text * scale) }, None);
    let pad = k.label_padding.0;
    let pill_w = text.width / scale + pad * 2.0;
    let (x, y) = place(pill_w);
    let mut pill = PathBuilder::new();
    let pr = PILL_HEIGHT / 2.0;
    pill.push_circle(x + pr, y + pr, pr);
    pill.push_circle(x + pill_w - pr, y + pr, pr);
    if let Some(middle) = Box::from_xywh(x + pr, y, (pill_w - PILL_HEIGHT).max(0.0), PILL_HEIGHT) {
        pill.push_rect(middle);
    }
    float_shadow(canvas, scale, (x, y, pill_w, PILL_HEIGHT), pr, k.label_shadow);
    if let Some(pill) = pill.finish() {
        let mut paint = Paint::default();
        paint.set_color(k.label_back.to_skia());
        paint.anti_alias = true;
        canvas.fill_path(&pill, &paint, FillRule::Winding, to_px, None);
    }
    fonts.draw(canvas, &text, (x + pad) * scale, (y + (PILL_HEIGHT - text.height / scale) / 2.0) * scale, k.label_ink);
}

pub fn format_colour([r, g, b]: [u8; 3]) -> String {
    let f = |v: u8| v as f64 / 255.0;
    format!("{} {} {}", f(r), f(g), f(b))
}

pub fn parse_colour(text: &str) -> Option<(f64, f64, f64)> {
    let mut parts = text.split_whitespace().map(|p| p.parse::<f64>().ok().filter(|v| (0.0..=1.0).contains(v)));
    let colour = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(colour)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_colour_goes_there_and_back() {
        let (r, g, b) = parse_colour(&format_colour([255, 0, 51])).unwrap();
        assert_eq!((r, g, b), (1.0, 0.0, 0.2));
        assert_eq!(parse_colour(""), None);
        assert_eq!(parse_colour("1 2 3"), None);
        assert_eq!(parse_colour("0.1 0.2"), None);
        assert_eq!(parse_colour("0.1 0.2 0.3 0.4"), None);
    }

    #[test]
    fn a_pixel_reads_straight_and_off_the_edge_is_none() {

        let data = vec![0x12, 0x34, 0x56, 0xFF, 0x80, 0x80, 0x80, 0x80];
        let p = Picture { width: 2, height: 1, stride: 8, data };
        assert_eq!(p.rgba_at(0, 0), Some([0x12, 0x34, 0x56, 0xFF]));
        assert_eq!(p.rgba_at(1, 0), Some([255, 255, 255, 0x80]));
        assert_eq!(p.rgba_at(2, 0), None);
        assert_eq!(p.rgba_at(0, -1), None);
    }
}
