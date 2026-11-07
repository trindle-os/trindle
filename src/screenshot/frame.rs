//! The part chosen on the photograph.

use tiny_skia::{FillRule, Mask, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, Stroke, Transform};

use super::area::{self, Handle, Rect, View};
use super::{eye_dropper, Doing, Shot, ASKING_DIM, CHOOSING_DIM};
use crate::toolkit::text::Fonts;
use crate::toolkit::widget::Pen;

pub(super) fn in_pixels(r: Rect, scale: f32) -> Option<tiny_skia::Rect> {
    tiny_skia::Rect::from_xywh(r.x as f32 * scale, r.y as f32 * scale, r.w as f32 * scale, r.h as f32 * scale)
}

pub(super) fn dim_outside(canvas: &mut PixmapMut, pen: &Pen, size: (f64, f64), part: Option<Rect>) {
    let all = Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 };
    let mut dim = PathBuilder::new();
    if let Some(r) = in_pixels(all, pen.scale) {
        dim.push_rect(r);
    }
    if let Some(r) = part.and_then(|p| in_pixels(p, pen.scale)) {
        dim.push_rect(r);
    }
    let mut black = Paint::default();
    black.set_color(crate::toolkit::theme::alpha(pen.theme.shadow, (if part.is_some() { CHOOSING_DIM } else { ASKING_DIM }) as f32).to_skia());
    if let Some(path) = dim.finish() {
        canvas.fill_path(&path, &black, FillRule::EvenOdd, Transform::identity(), None);
    }
}

pub(super) fn draw_marks(canvas: &mut PixmapMut, pen: &Pen, st: &Shot, photo: Transform, box_: tiny_skia::Rect) {
    let mut clip = Mask::new(canvas.width(), canvas.height());
    if let Some(m) = clip.as_mut() {
        m.fill_path(&PathBuilder::from_rect(box_), FillRule::Winding, false, Transform::identity());
    }
    if let Some(clip) = &clip {
        if let Some(mut layer) = Pixmap::new(canvas.width(), canvas.height()) {
            for m in st.marks.done().iter().chain(st.drawing.iter()) {
                m.draw(&mut layer.as_mut(), photo, pen.fonts, false);
            }
            if let Some(m) = &st.typing {
                m.draw(&mut layer.as_mut(), photo, pen.fonts, true);
            }
            canvas.draw_pixmap(0, 0, layer.as_ref(), &PixmapPaint::default(), Transform::identity(), Some(clip));
        }
    }
}

pub(super) fn draw_frame(canvas: &mut PixmapMut, pen: &Pen, (part, box_): (Rect, tiny_skia::Rect), doing: &Doing, view: &View) {
    let scale = pen.scale;
    let mut white = Paint::default();
    white.set_color(crate::toolkit::theme::alpha(pen.theme.fg, 0.9).to_skia());
    white.anti_alias = true;
    let line = Stroke { width: 1.5 * scale, ..Stroke::default() };
    canvas.stroke_path(&PathBuilder::from_rect(box_), &white, &line, Transform::identity(), None);
    if matches!(doing, Doing::Choosing(..)) {
        size_label(canvas, scale, pen.fonts, part, view);
        return;
    }
    let mut edge = Paint::default();
    edge.set_color(crate::toolkit::theme::alpha(pen.theme.shadow, 0.5).to_skia());
    edge.anti_alias = true;
    let thin = Stroke { width: scale, ..Stroke::default() };
    for h in Handle::ALL {
        let (x, y) = h.at(part);
        if let Some(dot) = PathBuilder::from_circle(x as f32 * scale, y as f32 * scale, 5.0 * scale) {
            canvas.fill_path(&dot, &white, FillRule::Winding, Transform::identity(), None);
            canvas.stroke_path(&dot, &edge, &thin, Transform::identity(), None);
        }
    }
    if matches!(doing, Doing::Resizing(..) | Doing::Moving(_)) {
        size_label(canvas, scale, pen.fonts, part, view);
    }
}

fn size_label(canvas: &mut PixmapMut, scale: f32, fonts: &Fonts, p: Rect, view: &View) {
    let (_, _, w, h) = area::to_pixels(p, view.size, view.shown);
    eye_dropper::pill(canvas, scale, fonts, &format!("{w} × {h}"), |pw| {
        let (x, y) = area::size_label_at(p, (pw as f64, eye_dropper::PILL_HEIGHT as f64), 8.0);
        (x as f32, y as f32)
    });
}
