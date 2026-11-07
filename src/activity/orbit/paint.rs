//! How the room, its rim bars, the bubbles and the notes are drawn.

use std::f64::consts::{FRAC_PI_2, TAU};

use tiny_skia::{FillRule, GradientStop, Paint, PathBuilder, PixmapMut, Point, RadialGradient, SpreadMode, Stroke, Transform};

use crate::toolkit::theme;
use crate::toolkit::colour::{Colour, paint as brush};
use crate::toolkit::text::Style;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{arc_to, label_layout, rounded, tip_box};
use crate::toolkit::widget::{Pen, Rect};

use super::{Bubble, FREE_ROW, Orbit, Placed, Room, Under};
use super::motion::room;

const BAR_REST: f64 = 4.0;

pub(super) const BAR_FULL: f64 = 24.0;

const STILL: f64 = 0.05;

pub(super) const BEAT: f64 = 0.5;

const NAMED_PX: f64 = 44.0;

const FIGURED_PX: f64 = 56.0;

pub(super) const NOTE_PAD: (f64, f64) = (10.0, 8.0);

pub(super) const NOTE_LINE_GAP: f64 = 1.0;

const NOTE_RADIUS: f32 = 14.0;

pub(in crate::activity) fn heat_colour(heat: f32) -> Colour {
    let calm = theme::pair()[0];
    let stops: [(f32, Colour); 5] =
        [(0.0, calm), (0.5, calm), (0.6, theme::warn()), (0.75, theme::warn()), (0.92, theme::problem())];
    let h = heat.clamp(0.0, 1.0);
    for pair in stops.windows(2) {
        let ((a, ca), (b, cb)) = (pair[0], pair[1]);
        if h <= b {
            return theme::mix(ca, cb, if b > a { (h - a) / (b - a) } else { 1.0 });
        }
    }
    theme::problem()
}

pub(super) fn beat(heat: f64) -> f64 {
    0.35 + 1.25 * heat
}

struct Pencil {
    at: Transform,
}

impl Pencil {
    fn fill(&self, canvas: &mut PixmapMut, path: &tiny_skia::Path, c: Colour) {
        canvas.fill_path(path, &brush(c), FillRule::Winding, self.at, None);
    }

    fn line(&self, canvas: &mut PixmapMut, path: &tiny_skia::Path, c: Colour, width: f32) {
        canvas.stroke_path(path, &brush(c), &Stroke { width, ..Stroke::default() }, self.at, None);
    }
}

fn circle(x: f64, y: f64, r: f64) -> Option<tiny_skia::Path> {
    PathBuilder::from_circle(x as f32, y as f32, r as f32)
}

impl Orbit {
    pub(super) fn paint(&self, canvas: &mut PixmapMut, pen: &Pen) {
        let s = pen.scale;
        let (w, h) = (canvas.width() as f64 / s as f64, canvas.height() as f64 / s as f64);
        let resized = self.size.replace((w, h)) != (w, h);
        if resized && !self.moving {
            self.settle_now();
        }
        let t = pen.theme;
        let pencil = Pencil { at: Transform::from_scale(s, s) };
        let g = room(w, h);
        self.paint_room(canvas, pen, &pencil, g, h);
        self.paint_rim(canvas, &pencil, t, g);

        let placed_notes = self.place_notes(pen, h, g);
        for p in &placed_notes {
            let (sx, sy, ex, ey) = p.line;
            let mut pb = PathBuilder::new();
            pb.move_to(sx as f32, sy as f32);
            pb.line_to(ex as f32, ey as f32);
            if let Some(path) = pb.finish() {
                pencil.line(canvas, &path, theme::alpha(t.fg, 0.45), 1.5);
            }
        }

        let bubbles = self.bubbles.borrow();
        self.paint_glows(canvas, &pencil, &bubbles, g);
        self.paint_bubbles(canvas, pen, &pencil, &bubbles, g);
        drop(bubbles);
        self.paint_notes(canvas, pen, &pencil, &placed_notes);

        if let Some((text, r)) = self.tip.borrow().as_ref() {
            let layout = label_layout(pen.fonts, text, s, canvas.width() as f32);
            let r = Rect { x: r.x * s, y: r.y * s, w: r.w * s, h: r.h * s };
            let ch = canvas.height() as f32;
            tip_box(canvas, t, pen.fonts, s, &layout, r, ch);
        }
    }

    fn paint_room(&self, canvas: &mut PixmapMut, pen: &Pen, pencil: &Pencil, g: Room, h: f64) {
        let (t, s) = (pen.theme, pen.scale);
        if let Some(path) = circle(g.cx, g.cy, g.radius) {
            pencil.fill(canvas, &path, t.bg);
        }
        let free = self.free.borrow();
        if !free.is_empty() {
            let style = Style { weight: 600, ..Style::new(14.0 * s) };
            let l = pen.fonts.layout(&free, &style, None);
            let middle = (h - FREE_ROW / 2.0) as f32 * s;
            pen.fonts.draw(canvas, &l, (g.cx as f32 * s - l.width / 2.0).round(), (middle - l.height / 2.0).round(), t.muted);
        }
    }

    fn paint_rim(&self, canvas: &mut PixmapMut, pencil: &Pencil, t: &Theme, g: Room) {
        let threads = self.threads.borrow();
        let n = threads.len().max(1) as f64;
        let each = TAU / n;
        let gap = (each * 0.22).min(0.05);
        let band = |a0: f64, a1: f64, out: f64| {
            let mut pb = PathBuilder::new();
            let c = (g.cx as f32, g.cy as f32);
            arc_to(&mut pb, c, (g.rim + out) as f32, a0 as f32, a1 as f32, true);
            arc_to(&mut pb, c, g.rim as f32, a1 as f32, a0 as f32, false);
            pb.close();
            pb.finish()
        };
        for (i, (shown, _)) in threads.iter().enumerate() {
            let a0 = -FRAC_PI_2 + i as f64 * each + gap / 2.0;
            let a1 = a0 + each - gap;
            if let Some(path) = band(a0, a1, BAR_FULL) {
                pencil.fill(canvas, &path, theme::alpha(t.fg, 0.07));
            }
            let b = shown.clamp(0.0, 1.0);
            if let Some(path) = band(a0, a1, BAR_REST + (BAR_FULL - BAR_REST) * b) {
                pencil.fill(canvas, &path, theme::alpha(heat_colour(b as f32), 0.55 + 0.45 * b as f32));
            }
        }
    }

    fn beat_of(&self, b: &Bubble) -> f64 {
        if self.moving && b.threads >= BEAT { b.phase.sin() } else { 0.0 }
    }

    fn paint_glows(&self, canvas: &mut PixmapMut, pencil: &Pencil, bubbles: &[Bubble], g: Room) {
        for b in bubbles.iter().filter(|b| b.threads >= STILL) {
            let (px, py, rp) = (g.cx + b.x * g.radius, g.cy + b.y * g.radius, b.r * g.radius);
            if rp < 1.0 {
                continue;
            }
            let sb = self.beat_of(b);
            let c = heat_colour(b.heat as f32);
            let reach = (rp * (0.16 + 0.42 * b.heat)).max(6.0 + 30.0 * b.heat) * (0.85 + 0.15 * sb) + 3.0;
            let strength = ((0.30 + 0.55 * b.heat) * (0.78 + 0.22 * sb)) as f32;
            let (inner, outer) = (rp * 0.9, rp + reach);
            let centre = Point::from_xy(px as f32, py as f32);
            let stops = vec![
                GradientStop::new(0.0, theme::alpha(c, strength).to_skia()),
                GradientStop::new(1.0, theme::alpha(c, 0.0).to_skia()),
            ];
            let Some(shader) = RadialGradient::new(centre, inner as f32, centre, outer as f32, stops, SpreadMode::Pad, Transform::identity())
            else { continue };
            let paint = Paint { shader, anti_alias: true, ..Paint::default() };
            if let Some(path) = circle(px, py, outer) {
                canvas.fill_path(&path, &paint, FillRule::Winding, pencil.at, None);
            }
        }
    }

    fn paint_bubbles(&self, canvas: &mut PixmapMut, pen: &Pen, pencil: &Pencil, bubbles: &[Bubble], g: Room) {
        let t = pen.theme;
        let solid = if t.light { t.panel } else { t.raised };
        let picked = self.picked();
        let keyed = self.keyed.borrow().clone();
        for b in bubbles.iter() {
            let (px, py) = (g.cx + b.x * g.radius, g.cy + b.y * g.radius);
            let rp = b.r * g.radius * (1.0 + (0.012 + 0.03 * b.heat) * self.beat_of(b));
            if rp < 0.5 {
                continue;
            }
            let Some(path) = circle(px, py, rp) else { continue };
            pencil.fill(canvas, &path, if b.quiet { theme::alpha(solid, 0.55) } else { solid });
            if b.threads >= STILL {
                pencil.line(canvas, &path, theme::alpha(heat_colour(b.heat as f32), 0.9), 2.0);
            }
            if picked.as_deref() == Some(b.key.as_str()) {
                if let Some(ring) = circle(px, py, rp + 5.0) {
                    pencil.line(canvas, &ring, theme::alpha(t.fg, 0.9), 3.0);
                }
            }
            if keyed.as_deref() == Some(b.key.as_str()) {
                let r = rp + 5.0 + 1.5 + (theme::sheet().focus_offset + theme::sheet().focus_width / 2.0) as f64;
                if let Some(ring) = circle(px, py, r) {
                    pencil.line(canvas, &ring, t.accent, theme::sheet().focus_width);
                }
            }
            paint_bubble_face(canvas, pen, b, (px, py), rp);
        }
    }

    fn paint_notes(&self, canvas: &mut PixmapMut, pen: &Pen, pencil: &Pencil, placed_notes: &[Placed]) {
        let (s, fg) = (pen.scale, pen.theme.fg);
        for p in placed_notes {
            if let Some(path) = circle(p.line.2, p.line.3, 3.5) {
                pencil.fill(canvas, &path, theme::alpha(fg, 0.8));
            }
        }
        let hover = self.hover.borrow().clone();
        for p in placed_notes {
            if hover == Some(Under::Note(p.index)) {
                if let Some(path) = rounded(p.rect, NOTE_RADIUS) {
                    pencil.fill(canvas, &path, theme::alpha(fg, 0.06));
                }
            }
            let mut y = (p.rect.y + NOTE_PAD.1 as f32) * s;
            let x = (p.rect.x + NOTE_PAD.0 as f32) * s;
            for (l, colour) in &p.words {
                pen.fonts.draw(canvas, l, x, y, *colour);
                y += l.height + NOTE_LINE_GAP as f32 * s;
            }
        }
        *self.notes_at.borrow_mut() = placed_notes.iter().map(|p| (p.rect, p.index)).collect();
    }
}

fn paint_bubble_face(canvas: &mut PixmapMut, pen: &Pen, b: &Bubble, (px, py): (f64, f64), rp: f64) {
    let (s, t) = (pen.scale, pen.theme);
    let dots = |size: f32, weight: u16| pen.fonts.layout("…", &Style { weight, ..Style::new(size) }, None);
    let icon = ((rp * 0.62 / 4.0).round() * 4.0).clamp(16.0, 64.0) as f32;
    let room_px = ((rp - rp * 0.28) * 2.0) as f32 * s;
    let name = (rp >= NAMED_PX).then(|| {
        let size = 13.0 * s;
        pen.fonts.layout(&b.title, &Style { weight: 700, ..Style::new(size) }, None).cut(room_px, &dots(size, 700))
    });
    let figure = (rp >= FIGURED_PX).then(|| {
        let size = 12.0 * s;
        pen.fonts.layout(&b.figure, &Style { weight: 600, ..Style::new(size) }, None).cut(room_px, &dots(size, 600))
    });
    let tall = icon * s + name.as_ref().map_or(0.0, |l| l.height) + figure.as_ref().map_or(0.0, |l| l.height);
    let (cx, mut y) = (px as f32 * s, py as f32 * s - tall / 2.0);
    let isize = (icon * s).round();
    for name in &b.icon {
        if pen.icons.draw(canvas, name, (cx - isize / 2.0).round() as i32, y.round() as i32, isize as u32, 1, t.fg).is_some() {
            break;
        }
    }
    y += isize;
    for (l, colour) in [(name, t.fg), (figure, t.muted)] {
        if let Some(l) = l {
            pen.fonts.draw(canvas, &l, cx - l.width / 2.0, y, colour);
            y += l.height;
        }
    }
}
