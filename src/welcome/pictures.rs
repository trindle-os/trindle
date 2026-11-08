//! The Welcome's pictures, drawn by hand in the desktop's flat greys: a
//! small screen, the ring opening on it, and the key that does the same. Nothing in them has an edge or a
//! line: every shape is a step of grey from the theme, the two picture
//! colours that go with the highlight are the apps' dots, and the highlight
//! itself marks the click and the key pressed. A picture that teaches a
//! movement plays it once a run and then rests; with movement turned off it
//! shows the finished moment, still.

use tiny_skia::{FillRule, PathBuilder, PixmapMut, Stroke, Transform};

use crate::toolkit::t;
use crate::toolkit::colour::{Colour, paint};
use crate::toolkit::motion::Ease;
use crate::toolkit::text::Style;
use crate::toolkit::theme::{self, Theme};
use crate::toolkit::ui::{fill, rounded};
use crate::toolkit::widget::{Pen, Rect};

pub fn hello_words() -> &'static str {
    t!("welcome-picture-hello")
}
pub fn plugins_words() -> &'static str {
    t!("welcome-picture-plugins")
}
pub fn ring_words() -> &'static str {
    t!("welcome-picture-ring")
}
pub fn super_words() -> &'static str {
    t!("welcome-picture-super")
}

pub fn dots_words() -> &'static str {
    t!("welcome-picture-dots")
}

fn circle(canvas: &mut PixmapMut, (x, y): (f32, f32), r: f32, c: Colour) {
    if let Some(path) = PathBuilder::from_circle(x, y, r.max(0.1)) {
        canvas.fill_path(&path, &paint(c), FillRule::Winding, Transform::identity(), None);
    }
}

fn with_alpha(c: Colour, a: f32) -> Colour {
    Colour { a: c.a * a.clamp(0.0, 1.0), ..c }
}

fn app_colour(i: usize) -> Colour {

    theme::pair()[i % 2]
}

fn sample_date() -> String {
    crate::toolkit::lang::format::date(&sample_moment(), crate::toolkit::lang::format::Pattern::ShortDay)
}

fn sample_time() -> String {
    crate::toolkit::lang::format::date(&sample_moment(), crate::toolkit::lang::format::Pattern::Clock)
}

fn sample_moment() -> crate::toolkit::lang::format::Moment {
    crate::toolkit::lang::format::Moment { year: 2026, month: 10, day: 10, weekday: 6, hour: 10, minute: 30, second: 0 }
}

fn words(canvas: &mut PixmapMut, pen: &Pen, text: &str, at: (f32, f32), size: f32, weight: u16, ink: Colour) {
    let style = Style { weight, ..Style::new(size) };
    let layout = pen.fonts.layout(text, &style, None);
    pen.fonts.draw(canvas, &layout, (at.0 - layout.width / 2.0).round(), (at.1 - layout.height / 2.0).round(), ink);
}

fn between(t: f32, from: f32, to: f32, ease: Ease) -> f32 {
    if to <= from {
        return if t >= to { 1.0 } else { 0.0 };
    }
    ease.at((t - from) / (to - from))
}

fn opened(t: f32, open: (f32, f32), close: (f32, f32)) -> f32 {
    between(t, open.0, open.1, Ease::Out) * (1.0 - between(t, close.0, close.1, Ease::Cubic))
}

fn lerp(a: (f32, f32), b: (f32, f32), k: f32) -> (f32, f32) {
    (a.0 + (b.0 - a.0) * k, a.1 + (b.1 - a.1) * k)
}

fn screen(canvas: &mut PixmapMut, t: &Theme, room: Rect, s: f32) -> Rect {
    let stand = room.h * 0.08;
    let (mut w, mut h) = (room.w, room.w * 10.0 / 16.0);
    if h > room.h - stand {
        h = room.h - stand;
        w = h * 16.0 / 10.0;
    }
    let glass = Rect { x: room.x + (room.w - w) / 2.0, y: room.y + (room.h - stand - h) / 2.0, w, h };
    let neck = Rect { x: glass.x + w / 2.0 - w * 0.05, y: glass.y + h, w: w * 0.1, h: stand * 0.7 };
    fill(canvas, neck, 0.0, t.raised);
    let foot = Rect { x: glass.x + w / 2.0 - w * 0.14, y: neck.y + neck.h - s, w: w * 0.28, h: stand * 0.35 };
    fill(canvas, foot, foot.h / 2.0, t.raised);
    let frame = Rect { x: glass.x - 4.0 * s, y: glass.y - 4.0 * s, w: w + 8.0 * s, h: h + 8.0 * s };
    fill(canvas, frame, 12.0 * s, t.raised);
    fill(canvas, glass, 8.0 * s, t.bg);
    wallpaper(canvas, t, glass, s);
    glass
}

fn wallpaper(canvas: &mut PixmapMut, t: &Theme, glass: Rect, s: f32) {
    canvas_clip(canvas, glass, |canvas| {
        for (x, y, w, h) in [(-0.08, 0.55, 0.5, 0.7), (0.62, -0.2, 0.55, 0.6), (0.3, 0.7, 0.45, 0.5)] {
            let r = Rect { x: glass.x + glass.w * x, y: glass.y + glass.h * y, w: glass.w * w, h: glass.h * h };
            fill(canvas, r, 40.0 * s, t.panel);
        }
    });
}

fn ring(canvas: &mut PixmapMut, pen: &Pen, glass: Rect, open: f32) {
    ring_with(canvas, pen, glass, open, &[]);
}

fn ring_with(canvas: &mut PixmapMut, pen: &Pen, glass: Rect, open: f32, tray: &[String]) {
    if open <= 0.001 {
        return;
    }
    let (t, s) = (pen.theme, pen.scale);

    fill(canvas, glass, 8.0 * s, with_alpha(t.bg, 0.55 * open));
    let centre = (glass.x + glass.w / 2.0, glass.y + glass.h / 2.0);
    let r = glass.h * 0.38 * (0.86 + 0.14 * open);
    circle(canvas, centre, r, with_alpha(t.raised, open));
    for i in 0..8 {
        let a = std::f32::consts::TAU * i as f32 / 8.0 - std::f32::consts::FRAC_PI_2;
        let at = (centre.0 + a.cos() * r * 0.73, centre.1 + a.sin() * r * 0.73);
        circle(canvas, at, r * 0.15, with_alpha(app_colour(i), open));
    }
    circle(canvas, centre, r * 0.47, with_alpha(t.panel, open));
    let weight = theme::sheet().heading.1;
    if tray.is_empty() {
        words(canvas, pen, &sample_time(), (centre.0, centre.1 - r * 0.08), r * 0.21, weight, with_alpha(t.fg, open));
        let small = (r * 0.12).max(9.0 * s);
        words(canvas, pen, &sample_date(), (centre.0, centre.1 + r * 0.14), small, 500, with_alpha(t.muted, open));
        return;
    }
    words(canvas, pen, &sample_time(), (centre.0, centre.1 - r * 0.16), r * 0.21, weight, with_alpha(t.fg, open));
    let size = (r * 0.17).round();
    let gap = size * 0.35;
    let whole = size * tray.len() as f32 + gap * (tray.len() - 1) as f32;
    let band = Rect { x: centre.0 - whole / 2.0 - gap, y: centre.1 + r * 0.05, w: whole + 2.0 * gap, h: size + gap };
    fill(canvas, band, band.h / 2.0, with_alpha(t.bg, open));
    for (i, icon) in tray.iter().enumerate() {
        let x = band.x + gap + i as f32 * (size + gap);
        let _ = pen.icons.draw(canvas, icon, x.round() as i32, (band.y + gap / 2.0).round() as i32, size as u32, 1, t.fg);
    }
}

pub fn plugins(canvas: &mut PixmapMut, pen: &Pen, icons: &[String]) {
    let s = pen.scale;
    let glass = screen(canvas, pen.theme, room(canvas, s), s);
    ring_with(canvas, pen, glass, 1.0, icons);
}

fn pointer(canvas: &mut PixmapMut, t: &Theme, at: (f32, f32), unit: f32) {
    let shape = [(0.0, 0.0), (0.0, 17.0), (4.2, 13.2), (7.2, 19.6), (9.6, 18.6), (6.7, 12.2), (12.2, 12.2)];
    let mut pb = PathBuilder::new();
    for (i, (x, y)) in shape.iter().enumerate() {
        let (px, py) = (at.0 + x * unit, at.1 + y * unit);
        if i == 0 { pb.move_to(px, py) } else { pb.line_to(px, py) }
    }
    pb.close();
    let Some(path) = pb.finish() else { return };
    canvas.fill_path(&path, &paint(t.fg), FillRule::Winding, Transform::identity(), None);
    let line = Stroke { width: (unit * 1.2).max(1.0), line_join: tiny_skia::LineJoin::Round, ..Stroke::default() };
    canvas.stroke_path(&path, &paint(t.bg), &line, Transform::identity(), None);
}

fn ripple(canvas: &mut PixmapMut, t: &Theme, at: (f32, f32), k: f32, unit: f32) {
    if k <= 0.0 || k >= 1.0 {
        return;
    }
    circle(canvas, at, unit * (4.0 + 14.0 * k), with_alpha(t.accent, 0.7 * (1.0 - k)));
}

fn key(canvas: &mut PixmapMut, pen: &Pen, r: Rect, label: &str, down: f32) {
    let (t, s) = (pen.theme, pen.scale);
    let face = theme::mix(t.raised, t.accent, down);
    fill(canvas, r, 10.0 * s, face);
    let ink = theme::mix(t.fg, t.on_accent, down);
    let size = (r.h * 0.34).min(20.0 * s);
    words(canvas, pen, label, (r.x + r.w / 2.0, r.y + r.h / 2.0), size, theme::sheet().heading.1, ink);
}

fn room(canvas: &PixmapMut, s: f32) -> Rect {
    let m = 12.0 * s;
    Rect { x: m, y: m, w: canvas.width() as f32 - 2.0 * m, h: canvas.height() as f32 - 2.0 * m }
}

pub fn hello(canvas: &mut PixmapMut, pen: &Pen) {
    let s = pen.scale;
    let glass = screen(canvas, pen.theme, room(canvas, s), s);
    ring(canvas, pen, glass, 1.0);
}

fn ring_moment(t: Option<f32>) -> (f32, f32, f32, f32) {
    match t {
        None => (1.0, 1.0, 0.3, 0.0),
        Some(t) => {
            let go = between(t, 0.3, 1.4, Ease::Cubic) * (1.0 - between(t, 3.0, 3.6, Ease::Cubic));
            let by_click = opened(t, (1.55, 2.1), (2.9, 3.3));
            let by_key = opened(t, (4.0, 4.5), (5.6, 6.0));
            let press = opened(t, (3.8, 3.95), (4.25, 4.45));
            let click = if (1.4..1.9).contains(&t) { (t - 1.4) / 0.5 } else { 0.0 };
            (go, by_click.max(by_key), click, press)
        }
    }
}

pub fn open_ring(canvas: &mut PixmapMut, pen: &Pen, t: Option<f32>) {
    let s = pen.scale;
    let glass = screen(canvas, pen.theme, room(canvas, s), s);
    let unit = (glass.h / 200.0).max(0.6 * s);
    let home = (glass.x + glass.w * 0.16, glass.y + glass.h * 0.8);
    let edge = (glass.x + glass.w * 0.7, glass.y + 1.0);
    let (go, open, click, _) = ring_moment(t);
    let at = lerp(home, edge, go);
    ring(canvas, pen, glass, open);
    canvas_clip(canvas, glass, |c| ripple(c, pen.theme, at, click, unit));
    pointer(canvas, pen.theme, at, unit);
}

pub fn super_key(canvas: &mut PixmapMut, pen: &Pen, t: Option<f32>) {
    let (_, _, _, press) = ring_moment(t);
    let s = pen.scale;
    let r = room(canvas, s);
    let h = (r.h * 0.5).min(r.w / 2.4).min(96.0 * s);
    let w = (h * 2.4).min(r.w);
    key(canvas, pen, Rect { x: r.x + (r.w - w) / 2.0, y: r.y + (r.h - h) / 2.0, w, h }, "Super", press);
}

fn canvas_clip(canvas: &mut PixmapMut, r: Rect, draw: impl FnOnce(&mut PixmapMut)) {
    let (w, h) = (canvas.width(), canvas.height());
    let Some(mut mask) = tiny_skia::Mask::new(w, h) else { return draw(canvas) };
    if let Some(path) = rounded(r, 0.0) {
        mask.fill_path(&path, FillRule::Winding, true, Transform::identity());
    }
    let Some(mut layer) = tiny_skia::Pixmap::new(w, h) else { return draw(canvas) };
    draw(&mut layer.as_mut());
    let lp = tiny_skia::PixmapPaint::default();
    canvas.draw_pixmap(0, 0, layer.as_ref(), &lp, Transform::identity(), Some(&mask));
}

pub fn letter(canvas: &mut PixmapMut, pen: &Pen, name: &str) {
    let (w, h) = (canvas.width() as f32, canvas.height() as f32);
    let n = name.bytes().fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(b.into()));
    let colour = app_colour(n as usize);
    let side = w.min(h);
    let r = Rect { x: (w - side) / 2.0, y: (h - side) / 2.0, w: side, h: side };
    fill(canvas, r, side * 0.28, colour);
    let first: String = name.chars().next().map(String::from).unwrap_or_default();
    words(canvas, pen, &first, (w / 2.0, h / 2.0), side * 0.5, theme::sheet().page_title.1, theme::on_as_written(colour));
}

fn fitted(canvas: &mut PixmapMut, pen: &Pen, text: &str, at: (f32, f32), size: f32, room: f32, ink: Colour) {
    let wide = pen.fonts.layout(text, &Style { weight: 500, ..Style::new(size) }, None).width;
    let size = if wide > room { (size * room / wide).max(8.0 * pen.scale) } else { size };
    words(canvas, pen, text, at, size, 500, ink);
}

fn dot(canvas: &mut PixmapMut, pen: &Pen, kind: crate::common::in_use::Kind, at: (f32, f32), r: f32) {
    let (red, green, blue) = kind.colour_in(pen.theme.light);
    circle(canvas, at, r, Colour { r: red as f32, g: green as f32, b: blue as f32, a: 1.0 });
}

pub fn dots_where(canvas: &mut PixmapMut, pen: &Pen) {
    use crate::common::in_use::Kind;
    let (t, s) = (pen.theme, pen.scale);
    let area = room(canvas, s);
    let gap = 16.0 * s;
    let cell = (area.w - gap) / 2.0;
    let caption = (area.h * 0.13).clamp(11.0 * s, 16.0 * s);
    let stage_h = (area.h - caption * 2.4).max(1.0);
    let unit = (cell / 190.0).min(stage_h / 120.0).max(0.2);
    let captions = [t!("welcome-dots-where-tab"), t!("welcome-dots-where-corner")];
    for (i, text) in captions.into_iter().enumerate() {
        let x = area.x + i as f32 * (cell + gap);
        let stage = Rect { x, y: area.y, w: cell, h: stage_h };
        fill(canvas, stage, 10.0 * s, t.bg);
        let mid = (stage.x + stage.w / 2.0, stage.y + stage.h / 2.0);
        let r = (6.5 * unit).max(4.0 * s);
        match i {
            0 => {

                let tab = Rect { x: stage.x + stage.w * 0.1, y: mid.1 - 17.0 * unit, w: stage.w * 0.8, h: 34.0 * unit };
                fill(canvas, tab, tab.h / 2.0, t.raised);
                circle(canvas, (tab.x + tab.h / 2.0, mid.1), tab.h * 0.28, app_colour(0));
                fill(canvas, Rect { x: tab.x + tab.h, y: mid.1 - 3.5 * unit, w: tab.w * 0.3, h: 7.0 * unit }, 3.5 * unit, t.muted);
                dot(canvas, pen, Kind::Camera, (tab.x + tab.w - tab.h * 0.45, mid.1), r);
                dot(canvas, pen, Kind::Microphone, (tab.x + tab.w - tab.h * 0.45 - r * 2.8, mid.1), r);
            }
            _ => {

                let glass = Rect { x: stage.x + stage.w * 0.08, y: stage.y + stage.h * 0.14, w: stage.w * 0.84, h: stage.h * 0.72 };
                fill(canvas, glass, 6.0 * s, t.panel);
                let pill = Rect { x: glass.x + (glass.w - r * 6.6) / 2.0, y: glass.y + 6.0 * unit, w: r * 6.6, h: r * 3.0 };
                fill(canvas, pill, pill.h / 2.0, t.raised);
                dot(canvas, pen, Kind::Microphone, (pill.x + pill.w * 0.3, pill.y + pill.h / 2.0), r);
                dot(canvas, pen, Kind::Camera, (pill.x + pill.w * 0.7, pill.y + pill.h / 2.0), r);
            }
        }
        fitted(canvas, pen, text, (stage.x + stage.w / 2.0, stage.y + stage.h + caption * 1.2), caption, cell, t.muted);
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

    #[test]
    fn the_sample_day_and_time_follow_the_language() {
        assert_eq!((sample_date().as_str(), sample_time().as_str()), ("Sat 10 Oct", "10:30"));
        let made_up = "kit-date-weekday-short-sat = Zor\nkit-date-month-short-oct = Blu\n\
                       kit-date-pattern-short-day = %b %-d, %a\nkit-date-pattern-clock = %H.%M\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!((sample_date().as_str(), sample_time().as_str()), ("Blu 10, Zor", "10.30"));
        });
    }
}
