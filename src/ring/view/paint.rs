//! The ring's drawing: the dim, the pieces of All apps, the disc, the bands across the middle and the labels.

use std::time::Instant;

use tiny_skia::{Color, FillRule, Mask, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, Rect as SkRect, Stroke, Transform};

use crate::common::apps;
use crate::ring::notice;
use crate::ring::ctx::{msg, Msg};
use crate::ring::layout::{CENTRE, LABEL_INSET, Layout};
use crate::toolkit::t;
use crate::toolkit::colour::Colour;
use crate::toolkit::text::Style;
use crate::toolkit::theme::Theme;
use crate::toolkit::widget::{drawing, Node, Pen, Pointer};

use super::app_tile::LABEL_SIZE;
use super::middle::SEGMENT;

pub(in crate::ring) const PIECE_SHADOW: [(f32, f32); 2] = [(8.0, 0.14), (2.0, 0.22)];

const LETTER_GAP: f32 = 0.8;

pub(in crate::ring) const DISC_ROOM: f32 = 76.0;

pub(in crate::ring) fn disc(across: f32, fill: fn(&crate::toolkit::theme::Theme) -> Colour) -> Node<Msg> {
    let room = DISC_ROOM;
    let side = across + 2.0 * room;
    drawing((side, side), "", move |pm, pen| {
        let s = pen.scale;
        let t = Transform::from_scale(s, s);
        let (c, r) = (side / 2.0, across / 2.0);
        notice::float_shadow(pm, pen, crate::toolkit::widget::Rect { x: (c - r) * s, y: (c - r) * s, w: 2.0 * r * s, h: 2.0 * r * s }, r * s);
        if let Some(p) = circle(c, c, r) {
            let mut paint = Paint { anti_alias: true, ..Paint::default() };
            paint.set_color(colour(fill(pen.theme), 1.0));
            pm.fill_path(&p, &paint, FillRule::Winding, t, None);
        }
    })
    .on_pointer(move |p| match p {
        Pointer::Up(x, y) if ((x - side / 2.0).powi(2) + (y - side / 2.0).powi(2)).sqrt() > across / 2.0 => msg(move |f| {
            let (w, h) = (f.size.0 as f32, f.size.1 as f32);
            f.background_click((w - side) / 2.0 + x, (h - side) / 2.0 + y)
        }),
        _ => Msg::idle(),
    })
}

pub(in crate::ring) fn colour(c: Colour, a: f32) -> Color {
    let c = c.to_skia();
    Color::from_rgba(c.red(), c.green(), c.blue(), c.alpha() * a).unwrap_or(Color::BLACK)
}

pub(in crate::ring) fn fill_round(pm: &mut PixmapMut, r: crate::toolkit::widget::Rect, c: Colour, radius: f32, s: f32) {
    let k = radius.min(r.h / 2.0).min(r.w / 2.0).max(0.0);

    let q = k * (1.0 - 0.5523);
    let (x0, y0, x1, y1) = (r.x, r.y, r.x + r.w, r.y + r.h);
    let mut pb = PathBuilder::new();
    pb.move_to(x0 + k, y0);
    pb.line_to(x1 - k, y0);
    pb.cubic_to(x1 - q, y0, x1, y0 + q, x1, y0 + k);
    pb.line_to(x1, y1 - k);
    pb.cubic_to(x1, y1 - q, x1 - q, y1, x1 - k, y1);
    pb.line_to(x0 + k, y1);
    pb.cubic_to(x0 + q, y1, x0, y1 - q, x0, y1 - k);
    pb.line_to(x0, y0 + k);
    pb.cubic_to(x0, y0 + q, x0 + q, y0, x0 + k, y0);
    pb.close();
    let Some(path) = pb.finish() else { return };
    let mut paint = Paint::default();
    paint.set_color(colour(c, 1.0));
    paint.anti_alias = true;
    pm.fill_path(&path, &paint, FillRule::Winding, Transform::from_scale(s, s), None);
}

pub(in crate::ring) fn piece_path(cx: f32, cy: f32, inner: f32, outer: f32, start: f32, end: f32) -> Option<tiny_skia::Path> {
    let steps = (((end - start).abs() * 180.0 / std::f32::consts::PI) as usize).max(2);
    let mut pb = PathBuilder::new();
    for k in 0..=steps {
        let a = start + (end - start) * k as f32 / steps as f32;
        let (x, y) = (cx + outer * a.cos(), cy + outer * a.sin());
        if k == 0 { pb.move_to(x, y) } else { pb.line_to(x, y) }
    }
    for k in (0..=steps).rev() {
        let a = start + (end - start) * k as f32 / steps as f32;
        pb.line_to(cx + inner * a.cos(), cy + inner * a.sin());
    }
    pb.close();
    pb.finish()
}

pub(in crate::ring) fn circle(cx: f32, cy: f32, r: f32) -> Option<tiny_skia::Path> {
    PathBuilder::from_circle(cx, cy, r.max(0.1))
}

pub(super) struct Scene {
    pub(super) seg: bool,
    pub(super) top: Option<bool>,
    pub(super) bare: bool,
    pub(super) empty: bool,
    pub(super) all: bool,
    pub(super) dim: f32,
}

struct Brush<'a> {
    th: &'a Theme,
    t: Transform,
}

impl Brush<'_> {
    fn fill(&self, pm: &mut PixmapMut, p: &tiny_skia::Path, c: Color, mask: Option<&Mask>) {
        let mut paint = Paint::default();
        paint.set_color(c);
        paint.anti_alias = true;
        pm.fill_path(p, &paint, FillRule::Winding, self.t, mask);
    }

    fn line(&self, pm: &mut PixmapMut, p: &tiny_skia::Path, c: Color, width: f32) {
        let mut paint = Paint::default();
        paint.set_color(c);
        paint.anti_alias = true;
        pm.stroke_path(p, &paint, &Stroke { width, ..Stroke::default() }, self.t, None);
    }

    fn edge(&self, pm: &mut PixmapMut, p: &tiny_skia::Path) {
        let mut paint = Paint::default();
        paint.set_color(colour(crate::toolkit::theme::mix(self.th.well, self.th.ring_edge, crate::toolkit::theme::sheet().ring_edge_strength), 1.0));
        paint.anti_alias = true;
        pm.stroke_path(p, &paint, &Stroke { width: crate::toolkit::theme::sheet().ring_edge_width, ..Stroke::default() }, self.t, None);
    }
}

pub(super) fn paint_ring(pm: &mut PixmapMut, pen: &Pen, lay: &Layout<usize>, scene: &Scene) {
    let th = pen.theme;
    let s = pen.scale;
    let brush = Brush { th, t: Transform::from_scale(s, s) };
    let (w, h) = (pm.width() as f32 / s, pm.height() as f32 / s);
    let centre = (w / 2.0, h / 2.0);
    let dim = colour(th.shadow, scene.dim);
    pm.fill(dim);
    if scene.all {
        paint_pieces(pm, &brush, lay, centre);
        if let Some(clip) = lay.clip {
            cut_to(pm, clip, s, dim);
        }
    }
    paint_disc(pm, pen, &brush, lay, centre);
    paint_middle(pm, &brush, scene, centre);
    if scene.all {
        paint_labels(pm, pen, lay, centre);
    }
    if scene.empty {
        let m = CENTRE as f32 / 2.0;
        let l = pen.fonts.layout(t!("ring-empty-hint"), &Style::new(14.0 * s), Some(m * 1.6 * s));
        pen.fonts.draw(pm, &l, centre.0 * s - l.width / 2.0, (centre.1 - lay.inner_radius as f32 + 4.0) * s, th.muted);
    }
}

pub(super) fn cut_to(pm: &mut PixmapMut, (x, y, w, h): (f64, f64, f64, f64), s: f32, dim: Color) {
    let (pw, ph) = (pm.width() as f32, pm.height() as f32);
    let (l, t) = ((x as f32 * s).clamp(0.0, pw), (y as f32 * s).clamp(0.0, ph));
    let (r, b) = (((x + w) as f32 * s).clamp(l, pw), ((y + h) as f32 * s).clamp(t, ph));
    let mut paint = Paint::default();
    paint.set_color(dim);
    paint.blend_mode = tiny_skia::BlendMode::Source;
    for strip in [(0.0, 0.0, pw, t), (0.0, b, pw, ph - b), (0.0, t, l, b - t), (r, t, pw - r, b - t)] {
        if let Some(rect) = SkRect::from_xywh(strip.0, strip.1, strip.2, strip.3) {
            pm.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }
}

fn paint_pieces(pm: &mut PixmapMut, brush: &Brush, lay: &Layout<usize>, (cx, cy): (f32, f32)) {
    let th = brush.th;
    for piece in &lay.pieces {
        let Some(p) = piece_path(cx, cy, piece.inner as f32, piece.outer as f32, piece.start as f32, piece.end as f32) else { continue };
        for (dy, a) in PIECE_SHADOW {
            if let Some(sh) = p.clone().transform(Transform::from_translate(0.0, dy)) {
                brush.fill(pm, &sh, colour(th.shadow, a), None);
            }
        }
        brush.fill(pm, &p, colour(th.well, 1.0), None);
        brush.edge(pm, &p);
    }
}

fn paint_disc(pm: &mut PixmapMut, pen: &Pen, brush: &Brush, lay: &Layout<usize>, (cx, cy): (f32, f32)) {
    let s = pen.scale;
    let d = lay.disc_radius as f32;
    notice::float_shadow(pm, pen, crate::toolkit::widget::Rect { x: (cx - d) * s, y: (cy - d) * s, w: 2.0 * d * s, h: 2.0 * d * s }, d * s);
    if let Some(p) = circle(cx, cy, d) {
        brush.fill(pm, &p, colour(brush.th.well, 1.0), None);
        brush.edge(pm, &p);
    }
}

fn paint_middle(pm: &mut PixmapMut, brush: &Brush, scene: &Scene, (cx, cy): (f32, f32)) {
    let th = brush.th;
    let m = CENTRE as f32 / 2.0;
    let seg_h = SEGMENT as f32;

    let s = brush.t.sx;
    let (x0, y0) = (((cx - m - 1.0) * s).floor(), ((cy - m - 1.0) * s).floor());
    let side = ((2.0 * m + 2.0) * s).ceil() as u32 + 2;
    let (Some(mut small), Some(mut cut)) = (Pixmap::new(side, side), Mask::new(side, side)) else { return };
    let local = Brush { th, t: Transform::from_scale(s, s).post_translate(-x0, -y0) };
    let Some(round) = circle(cx, cy, m) else { return };
    let mut part = |top: f32, bottom: f32, c: Colour| {
        let Some(rect) = SkRect::from_ltrb(cx - m - 1.0, top, cx + m + 1.0, bottom) else { return };
        cut.clear();
        cut.fill_path(&PathBuilder::from_rect(rect), FillRule::Winding, true, local.t);
        local.fill(&mut small.as_mut(), &round, colour(c, 1.0), Some(&cut));
    };
    let lit = |on: bool| if on { th.accent } else { th.control };
    if scene.bare {
        part(cy - m - 1.0, cy + m + 1.0, th.bg);
    } else {
        let from = if scene.top.is_some() { cy - m + seg_h } else { cy - m - 1.0 };
        part(from, cy + m - seg_h, th.bg);
        part(cy + m - seg_h, cy + m + 1.0, lit(scene.seg));
        if let Some(on) = scene.top {
            part(cy - m - 1.0, cy - m + seg_h, lit(on));
        }
    }
    pm.draw_pixmap(x0 as i32, y0 as i32, small.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
}

const LISTENING_REACH: f32 = 6.0 + 10.0 + 3.0;

const PULSE_TURN: f32 = 1.0;

pub(super) fn pulse_at(since: Instant) -> f32 {
    (since.elapsed().as_secs_f32() / PULSE_TURN).fract()
}

pub(super) fn listening_rings(since: Instant) -> Node<Msg> {
    let side = CENTRE as f32 + 2.0 * LISTENING_REACH;
    let moving = !crate::toolkit::motion::reduced();
    let rings = drawing((side, side), "", move |pm, pen| {
        let s = pen.scale;
        let brush = Brush { th: pen.theme, t: Transform::from_scale(s, s) };
        paint_listening(pm, &brush, (side / 2.0, side / 2.0), if moving { pulse_at(since) } else { 0.0 });
    });
    if moving { rings.alive() } else { rings }
}

fn paint_listening(pm: &mut PixmapMut, brush: &Brush, (cx, cy): (f32, f32), pulse: f32) {
    let m = CENTRE as f32 / 2.0;
    let accent = brush.th.accent;
    if let Some(p) = circle(cx, cy, m + 6.0 + 10.0 * pulse) {
        brush.line(pm, &p, colour(accent, 0.9 * (1.0 - pulse)), 3.0);
    }
    if let Some(p) = circle(cx, cy, m + 2.0) {
        brush.line(pm, &p, colour(accent, 1.0), 3.0);
    }
}

pub(super) fn short_title(category: &str) -> Option<&'static str> {
    (category == "media").then(|| t!("ring-category-media-short"))
}

fn paint_labels(pm: &mut PixmapMut, pen: &Pen, lay: &Layout<usize>, (cx, cy): (f32, f32)) {
    let s = pen.scale;
    let ink = pen.theme.muted;
    let mut style = Style::new(LABEL_SIZE * s);
    style.family = Some(crate::toolkit::text::FAMILY.into());
    style.weight = 700;
    for piece in &lay.pieces {
        let words = apps::category_title(&piece.category).to_string();
        let r = (piece.label_radius - LABEL_INSET) as f32;
        let room = match &piece.straight {
            Some(st) => st.length as f32 * 0.9,
            None => (piece.end - piece.start) as f32 * 0.9 * r,
        } * s;
        let mut l = pen.fonts.layout(&words, &style, None).spaced(LETTER_GAP * s);
        if l.width > room {
            if let Some(short) = short_title(&piece.category) {
                l = pen.fonts.layout(short, &style, None).spaced(LETTER_GAP * s);
            }
        }
        if l.width > room {
            continue;
        }
        match &piece.straight {
            Some(st) => {
                let Some(mut tmp) = Pixmap::new(l.width.ceil() as u32 + 2, l.height.ceil() as u32 + 2) else { continue };
                pen.fonts.draw(&mut tmp.as_mut(), &l, 1.0, 1.0, ink);
                let at = Transform::from_translate(-(tmp.width() as f32) / 2.0, -(tmp.height() as f32) / 2.0)
                    .post_rotate((st.turn as f32).to_degrees())
                    .post_translate((cx + st.x as f32) * s, (cy + st.y as f32) * s);
                pm.draw_pixmap(0, 0, tmp.as_ref(), &PixmapPaint::default(), at, None);
            }
            None => {
                let mid = ((piece.start + piece.end) / 2.0) as f32;
                pen.fonts.draw_on_arc(pm, &l, (cx * s, cy * s), r * s, mid, ink);
            }
        }
    }
}
