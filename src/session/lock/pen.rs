//! The pen the lock screen draws with, and the look it wears.

use tiny_skia::{FillRule, LineCap, LineJoin, Path, PathBuilder, PixmapMut, Stroke, Transform};

use crate::toolkit::ui;
use crate::toolkit::colour::{paint, Colour};
use crate::toolkit::text::{Fonts, Layout, Style};
use crate::toolkit::theme::Theme;
use crate::toolkit::widget::Rect;

pub(super) fn look() -> Theme {
    Theme::new(false, &crate::common::style::accent())
}

pub(super) const GLYPH_LINE: f32 = 1.5;

pub(super) struct Pen<'a> {
    pub(super) px: PixmapMut<'a>,
    pub(super) scale: f32,
    pub(super) fonts: &'a Fonts,
    pub(super) t: &'a Theme,
}

impl Pen<'_> {
    pub(super) fn fill(&mut self, path: Option<Path>, c: Colour) {
        let Some(path) = path else { return };
        self.px.fill_path(&path, &paint(c), FillRule::Winding, Transform::from_scale(self.scale, self.scale), None);
    }

    pub(super) fn stroke(&mut self, path: Option<Path>, c: Colour, width: f32, round: bool) {
        let Some(path) = path else { return };
        let mut s = Stroke { width, ..Stroke::default() };
        if round {
            s.line_cap = LineCap::Round;
            s.line_join = LineJoin::Round;
        }
        self.px.stroke_path(&path, &paint(c), &s, Transform::from_scale(self.scale, self.scale), None);
    }

    pub(super) fn edged(&mut self, path: Option<Path>, fill: Colour, edge: Colour, width: f32) {
        self.fill(path.clone(), fill);
        self.stroke(path, edge, width, false);
    }

    pub(super) fn layout(&self, text: &str, size: f64, bold: bool) -> Layout {
        let style = Style { family: Some(crate::toolkit::text::FAMILY.into()), weight: if bold { 700 } else { 400 }, ..Style::new(size as f32 * self.scale) };
        self.fonts.layout(text, &style, None)
    }

    pub(super) fn text_at(&mut self, l: &Layout, x: f64, y: f64, c: Colour) {
        let baseline = l.lines.first().map_or(0.0, |line| line.baseline);
        self.fonts.draw(&mut self.px, l, x as f32 * self.scale, y as f32 * self.scale - baseline, c);
    }

    pub(super) fn width(&self, l: &Layout) -> f64 {
        f64::from(l.width / self.scale)
    }

    pub(super) fn centred(&mut self, text: &str, x: f64, y: f64, size: f64, bold: bool, c: Colour) {
        let l = self.layout(text, size, bold);
        let w = self.width(&l);
        self.text_at(&l, x - w / 2.0, y, c);
    }
}

pub(super) fn circle(x: f64, y: f64, r: f64) -> Option<Path> {
    PathBuilder::from_circle(x as f32, y as f32, r as f32)
}

pub(super) fn rounded(x: f64, y: f64, w: f64, h: f64, r: f64) -> Option<Path> {
    ui::rounded(Rect { x: x as f32, y: y as f32, w: w as f32, h: h as f32 }, r as f32)
}

pub(super) fn lines(runs: &[&[(f64, f64)]], close_first: bool) -> Option<Path> {
    let mut p = PathBuilder::new();
    for (n, run) in runs.iter().enumerate() {
        for (i, &(x, y)) in run.iter().enumerate() {
            if i == 0 {
                p.move_to(x as f32, y as f32);
            } else {
                p.line_to(x as f32, y as f32);
            }
        }
        if n == 0 && close_first {
            p.close();
        }
    }
    p.finish()
}
