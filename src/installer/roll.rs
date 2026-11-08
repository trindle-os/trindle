//! The Trindle mark rolls along a line, one stop for each step of the install.

use crate::toolkit::colour::paint as ink;
use std::f32::consts::PI;

use tiny_skia::{FillRule, PathBuilder, PixmapMut, Stroke, Transform};

use crate::toolkit::colour::Colour;
use crate::toolkit::text::Style;
use crate::toolkit::widget::Pen;

pub const HEIGHT: f32 = 92.0;
const RADIUS: f32 = 26.0;
const EDGE: f32 = 44.0;

fn dot(canvas: &mut PixmapMut, at: Transform, x: f32, y: f32, r: f32, c: Colour) {
    if let Some(path) = PathBuilder::from_circle(x, y, r) {
        canvas.fill_path(&path, &ink(c), FillRule::Winding, at, None);
    }
}

fn line(canvas: &mut PixmapMut, from: f32, to: f32, y: f32, width: f32, c: Colour) {
    let mut pb = PathBuilder::new();
    pb.move_to(from, y);
    pb.line_to(to, y);
    if let Some(path) = pb.finish() {
        let stroke = Stroke { width, line_cap: tiny_skia::LineCap::Round, ..Stroke::default() };
        canvas.stroke_path(&path, &ink(c), &stroke, Transform::identity(), None);
    }
}

pub fn step(at: f32, to: f32) -> Option<f32> {
    let gap = to - at;
    (gap.abs() >= 0.002).then_some(at + gap * 0.14)
}

pub fn paint(canvas: &mut PixmapMut, pen: &Pen, names: &[&str], at: f32, accent: Colour) {
    let (t, s) = (pen.theme, pen.scale);
    let (w, h) = (canvas.width() as f32, canvas.height() as f32);
    let line_y = h / 2.0 + 14.0 * s;
    let left = EDGE * s;
    let right = (w - EDGE * s).max(left + 1.0);
    let step = (right - left) / (names.len().saturating_sub(1).max(1) as f32);
    let x = left + at * step;

    line(canvas, left, right, line_y, 4.0 * s, t.raised);
    if x > left {
        line(canvas, left, x, line_y, 4.0 * s, accent);
    }

    for (i, name) in names.iter().enumerate() {
        let sx = left + i as f32 * step;
        let done = i as f32 <= at + 0.01;
        dot(canvas, Transform::identity(), sx, line_y, 5.0 * s, if done { accent } else { t.raised_more });
        let style = Style { weight: if done { 700 } else { 400 }, ..Style::new(11.5 * s) };
        let layout = pen.fonts.layout(name, &style, None);
        let base = layout.lines.first().map_or(layout.height, |l| l.baseline);
        let colour = if done { t.fg } else { t.muted };
        pen.fonts.draw(canvas, &layout, (sx - layout.width / 2.0).round(), (line_y + 22.0 * s - base).round(), colour);
    }

    let r = RADIUS * s;
    let here = Transform::from_translate(x, line_y - r - 2.0 * s);
    let turned = here.pre_rotate((x - left) / r * 180.0 / PI);
    dot(canvas, turned, 0.0, 0.0, r, accent);
    let middle = t.bg;
    if let Some(path) = PathBuilder::from_circle(0.0, 0.0, r * 0.66) {
        canvas.stroke_path(&path, &ink(middle), &Stroke { width: r * 0.10, ..Stroke::default() }, turned, None);
    }
    for i in 0..8 {
        let a = i as f32 * PI / 4.0;
        dot(canvas, turned, a.cos() * r * 0.42, a.sin() * r * 0.42, r * 0.10, middle);
    }
}

#[cfg(test)]
mod tests {
    use super::step;

    #[test]
    fn rolls_in_and_stops() {
        let mut at = 0.0;
        let mut n = 0;
        while let Some(next) = step(at, 3.0) {
            at = next;
            n += 1;
            assert!(n < 200);
        }
        assert!((at - 3.0).abs() < 0.002);
    }
}
