//! A small moving picture of the ring, for the welcome window: the edge
//! of the screen lights, the ring turns slowly, and the time sits in the
//! middle. Flat greys from the theme, the picture colours that go with the
//! highlight for the apps, and the highlight itself for the first app and
//! the edge.

use crate::toolkit::colour::paint as ink;
use std::f32::consts::PI;

use tiny_skia::{PathBuilder, PixmapMut, Stroke, Transform};

use crate::toolkit::colour::Colour;
use crate::toolkit::text::Style;
use crate::toolkit::theme::{alpha, pair};
use crate::toolkit::widget::Pen;

pub const SPEED: f32 = 0.0875;
const APPS: usize = 10;

fn disc(canvas: &mut PixmapMut, x: f32, y: f32, r: f32, c: Colour) {
    if let Some(path) = PathBuilder::from_circle(x, y, r) {
        canvas.fill_path(&path, &ink(c), tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
}

pub fn paint(canvas: &mut PixmapMut, pen: &Pen, accent: Colour, turn: f32) {
    let (t, s) = (pen.theme, pen.scale);
    let (w, h) = (canvas.width() as f32, canvas.height() as f32);
    let (cx, cy) = (w / 2.0, h / 2.0);
    let r = (w.min(h) / 2.0 - 26.0 * s).max(40.0 * s);

    let (edge, half) = (8.0 * s, 28.0 * s);
    let mut pb = PathBuilder::new();
    pb.move_to(cx - half, edge);
    pb.line_to(cx + half, edge);
    if let Some(path) = pb.finish() {
        let stroke = Stroke { width: 4.0 * s, line_cap: tiny_skia::LineCap::Round, ..Stroke::default() };
        canvas.stroke_path(&path, &ink(accent), &stroke, Transform::identity(), None);
    }

    let colours = pair();
    disc(canvas, cx, cy, r + 18.0 * s, t.raised);
    for i in 0..APPS {
        let a = turn + i as f32 * 2.0 * PI / APPS as f32 - PI / 2.0;
        let (x, y) = (cx + a.cos() * r, cy + a.sin() * r);
        if i == 0 {
            disc(canvas, x, y, 15.0 * s, accent);
        } else {
            disc(canvas, x, y, 12.0 * s, colours[i % 2]);
        }
    }

    disc(canvas, cx, cy, r * 0.62, t.bg);
    let time = crate::common::clock::Local::now().line(trindle_toolkit::lang::format::Pattern::Clock);
    let layout = pen.fonts.layout(&time, &Style { weight: 700, ..Style::new((r * 0.36).round()) }, None);
    pen.fonts.draw(canvas, &layout, (cx - layout.width / 2.0).round(), (cy - layout.height / 2.0).round(), t.fg);

    for i in 0..4 {
        let x = cx + (i as f32 - 1.5) * 18.0 * s;
        disc(canvas, x, cy + r * 0.34, 5.0 * s, alpha(t.fg, 0.22));
    }
}
