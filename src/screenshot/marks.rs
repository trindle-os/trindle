//! What is drawn on a screenshot.

use tiny_skia::{FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, Rect as Box,
                StrokeDash, Stroke, Transform};

use super::area::{between, Pt, Rect};
use crate::common::kit::Rgb;
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::text::{Fonts, Style};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    Pen,
    Line,
    Arrow,
    Box,
    Ring,
    Marker,
    Text,
    Number,
    Blur,
}

impl Tool {
    pub const ALL: [Tool; 9] = [Tool::Pen, Tool::Line, Tool::Arrow, Tool::Box, Tool::Ring,
                                Tool::Marker, Tool::Text, Tool::Number, Tool::Blur];

    pub fn name(self) -> &'static str {
        match self {
            Tool::Pen => t!("screenshot-tool-pen"),
            Tool::Line => t!("screenshot-tool-line"),
            Tool::Arrow => t!("screenshot-tool-arrow"),
            Tool::Box => t!("screenshot-tool-box"),
            Tool::Ring => t!("screenshot-tool-circle"),
            Tool::Marker => t!("screenshot-tool-highlighter"),
            Tool::Text => t!("screenshot-tool-words"),
            Tool::Number => t!("screenshot-tool-numbers"),
            Tool::Blur => t!("screenshot-tool-blur"),
        }
    }
}

pub const RED: Rgb = (0.953, 0.259, 0.212);
pub const BLUE: Rgb = (0.129, 0.588, 0.953);
pub const GREEN: Rgb = (0.294, 0.682, 0.310);
pub const ORANGE: Rgb = (0.996, 0.592, 0.0);
pub const PURPLE: Rgb = (0.608, 0.149, 0.682);
pub const WHITE: Rgb = (1.0, 1.0, 1.0);
pub const BLACK: Rgb = (0.0, 0.0, 0.0);

pub const COLOURS: [(Rgb, &str); 7] = [(RED, "Red"), (BLUE, "Blue"), (GREEN, "Green"),
                                       (ORANGE, "Orange"), (PURPLE, "Purple"), (WHITE, "White"),
                                       (BLACK, "Black")];

pub fn colour_names() -> [&'static str; 7] {
    [t!("screenshot-colour-red"), t!("screenshot-colour-blue"), t!("screenshot-colour-green"), t!("screenshot-colour-orange"),
     t!("screenshot-colour-purple"), t!("screenshot-colour-white"), t!("screenshot-colour-black")]
}

pub const THICKNESS: [(f64, &str); 3] = [(3.0, "Thin"), (6.0, "Medium"), (10.0, "Thick")];

pub fn thickness_names() -> [&'static str; 3] {
    [t!("screenshot-thickness-thin"), t!("screenshot-thickness-medium"), t!("screenshot-thickness-thick")]
}

pub fn against(c: Rgb) -> Rgb {
    use crate::toolkit::theme::contrast;
    let on = colour_of(c);
    let ink = if contrast(colour_of(BLACK), on) >= contrast(colour_of(WHITE), on) { BLACK } else { WHITE };
    #[cfg(test)]
    crate::toolkit::theme::drawn::vouch(colour_of(ink), &[on]);
    ink
}

#[derive(Clone, Debug)]
pub enum Shape {
    Pen(Vec<Pt>),
    Line(Pt, Pt),
    Arrow(Pt, Pt),
    Box(Pt, Pt),
    Ring(Pt, Pt),
    Marker(Vec<Pt>),
    Text(Pt, String),
    Number(Pt, u32),
    Blur(Pt, Pt, Option<Pixmap>),
}

#[derive(Clone, Debug)]
pub struct Mark {
    pub shape: Shape,
    pub colour: Rgb,
    pub thick: f64,
    pub unit: f64,
}

impl Mark {
    pub fn begin(tool: Tool, p: Pt, colour: Rgb, thick: f64, unit: f64, number: u32) -> Mark {
        let shape = match tool {
            Tool::Pen => Shape::Pen(vec![p]),
            Tool::Line => Shape::Line(p, p),
            Tool::Arrow => Shape::Arrow(p, p),
            Tool::Box => Shape::Box(p, p),
            Tool::Ring => Shape::Ring(p, p),
            Tool::Marker => Shape::Marker(vec![p]),
            Tool::Text => Shape::Text(p, String::new()),
            Tool::Number => Shape::Number(p, number),
            Tool::Blur => Shape::Blur(p, p, None),
        };
        Mark { shape, colour, thick, unit }
    }

    pub fn drag(&mut self, p: Pt) {
        match &mut self.shape {
            Shape::Pen(points) | Shape::Marker(points) => {
                if points.last().is_none_or(|l| (l.0 - p.0).hypot(l.1 - p.1) >= self.unit) {
                    points.push(p);
                }
            }
            Shape::Line(_, b) | Shape::Arrow(_, b) | Shape::Box(_, b) | Shape::Ring(_, b) => *b = p,
            Shape::Blur(_, b, made) => {
                *b = p;
                *made = None;
            }
            Shape::Text(at, _) | Shape::Number(at, _) => *at = p,
        }
    }

    pub fn worth_keeping(&self) -> bool {
        let long = |a: &Pt, b: &Pt| (a.0 - b.0).hypot(a.1 - b.1) >= 3.0 * self.unit;
        match &self.shape {
            Shape::Pen(_) | Shape::Marker(_) | Shape::Number(..) => true,
            Shape::Line(a, b) | Shape::Arrow(a, b) => long(a, b),
            Shape::Box(a, b) | Shape::Ring(a, b) | Shape::Blur(a, b, _) => {
                let r = between(*a, *b);
                r.w >= 3.0 * self.unit && r.h >= 3.0 * self.unit
            }
            Shape::Text(_, words) => !words.trim().is_empty(),
        }
    }

    fn width(&self) -> f64 {
        self.thick * self.unit
    }

    fn block(&self) -> usize {
        ((6.0 + 1.6 * self.thick) * self.unit).round().max(2.0) as usize
    }

    pub fn text_size(&self) -> f64 {
        (14.0 + 2.4 * self.thick) * self.unit
    }

    pub fn caret(&self, fonts: &Fonts) -> Option<(Pt, f64)> {
        let Shape::Text(p, words) = &self.shape else { return None };
        let size = self.text_size();
        let end = p.0 + fonts.layout(words, &style(size as f32), None).width as f64;
        Some(((end + 2.0 * self.unit, p.1 - size * 0.6), size * 1.15))
    }

    pub fn make_blur(&mut self, still: &Pixmap) {
        let block = self.block();
        if let Shape::Blur(a, b, made @ None) = &mut self.shape {
            *made = blurred(still, between(*a, *b), block);
        }
    }

    pub fn draw(&self, canvas: &mut PixmapMut, at: Transform, fonts: &Fonts, caret: bool) {
        let w = self.width() as f32;
        let ink = paint(self.colour, 1.0);
        let round = line(w, LineCap::Round, LineJoin::Round);
        match &self.shape {
            Shape::Pen(points) => trail(canvas, points, &ink, &round, at),
            Shape::Marker(points) => {
                let wide = ((12.0 + 2.5 * self.thick) * self.unit) as f32;
                trail(canvas, points, &paint(self.colour, 0.42), &line(wide, LineCap::Square, LineJoin::Round), at);
            }
            Shape::Line(a, b) => {
                if let Some(p) = segment(*a, *b) {
                    canvas.stroke_path(&p, &ink, &round, at, None);
                }
            }
            Shape::Arrow(a, b) => arrow(canvas, *a, *b, w as f64, self.unit, &ink, &round, at),
            Shape::Box(a, b) => {
                if let Some(p) = outline(between(*a, *b)) {
                    canvas.stroke_path(&p, &ink, &line(w, LineCap::Butt, LineJoin::Miter), at, None);
                }
            }
            Shape::Ring(a, b) => {
                let r = between(*a, *b);
                if r.w > 0.0 && r.h > 0.0 {
                    if let Some(p) = Box::from_xywh(r.x as f32, r.y as f32, r.w as f32, r.h as f32).and_then(PathBuilder::from_oval) {
                        canvas.stroke_path(&p, &ink, &round, at, None);
                    }
                }
            }
            Shape::Text(p, words) => {
                let size = self.text_size();
                let end = words_at(canvas, at, fonts, *p, words, size, self.colour);
                if caret {
                    let x = end + 2.0 * self.unit;
                    if let Some(bar) = segment((x, p.1 - size * 0.6), (x, p.1 + size * 0.55)) {
                        canvas.stroke_path(&bar, &ink, &line((2.0 * self.unit) as f32, LineCap::Round, LineJoin::Round), at, None);
                    }
                }
            }
            Shape::Number(p, n) => number(canvas, at, fonts, *p, *n, self.colour, self.thick, self.unit),
            Shape::Blur(a, b, made) => {
                let r = between(*a, *b);
                match made {
                    Some(blocks) => {
                        canvas.draw_pixmap(r.x.round() as i32, r.y.round() as i32, blocks.as_ref(),
                            &PixmapPaint::default(), at, None);
                    }
                    None => {
                        let u = self.unit as f32;
                        let dashed = Stroke { dash: StrokeDash::new(vec![6.0 * u, 4.0 * u], 0.0),
                                              ..line(1.5 * u, LineCap::Butt, LineJoin::Miter) };
                        if let Some(p) = outline(r) {
                            canvas.stroke_path(&p, &paint(WHITE, 0.8), &dashed, at, None);
                        }
                    }
                }
            }
        }
    }
}

fn paint(c: Rgb, alpha: f32) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color(Colour { r: c.0 as f32, g: c.1 as f32, b: c.2 as f32, a: alpha }.to_skia());
    p.anti_alias = true;
    p
}

fn line(width: f32, line_cap: LineCap, line_join: LineJoin) -> Stroke {
    Stroke { width, line_cap, line_join, ..Stroke::default() }
}

fn segment(a: Pt, b: Pt) -> Option<tiny_skia::Path> {
    let mut p = PathBuilder::new();
    p.move_to(a.0 as f32, a.1 as f32);
    p.line_to(b.0 as f32, b.1 as f32);
    p.finish()
}

fn outline(r: Rect) -> Option<tiny_skia::Path> {
    let (x0, y0, x1, y1) = (r.x as f32, r.y as f32, (r.x + r.w) as f32, (r.y + r.h) as f32);
    let mut p = PathBuilder::new();
    p.move_to(x0, y0);
    p.line_to(x1, y0);
    p.line_to(x1, y1);
    p.line_to(x0, y1);
    p.close();
    p.finish()
}

fn trail(canvas: &mut PixmapMut, points: &[Pt], ink: &Paint, stroke: &Stroke, at: Transform) {
    match points {
        [] => {}
        [p] => {
            if let Some(dot) = PathBuilder::from_circle(p.0 as f32, p.1 as f32, stroke.width / 2.0) {
                canvas.fill_path(&dot, ink, FillRule::Winding, at, None);
            }
        }
        [first, rest @ ..] => {
            let mut path = PathBuilder::new();
            path.move_to(first.0 as f32, first.1 as f32);
            for p in rest {
                path.line_to(p.0 as f32, p.1 as f32);
            }
            if let Some(path) = path.finish() {
                canvas.stroke_path(&path, ink, stroke, at, None);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn arrow(canvas: &mut PixmapMut, a: Pt, b: Pt, width: f64, unit: f64, ink: &Paint, stroke: &Stroke, at: Transform) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = dx.hypot(dy);
    if len < 1.0 {
        return;
    }
    let (ux, uy) = (dx / len, dy / len);
    let head = (width * 3.6).max(14.0 * unit).min(len);
    let half = head * 0.55;
    if let Some(p) = segment(a, (b.0 - ux * head * 0.7, b.1 - uy * head * 0.7)) {
        canvas.stroke_path(&p, ink, stroke, at, None);
    }
    let base = (b.0 - ux * head, b.1 - uy * head);
    let mut tip = PathBuilder::new();
    tip.move_to(b.0 as f32, b.1 as f32);
    tip.line_to((base.0 - uy * half) as f32, (base.1 + ux * half) as f32);
    tip.line_to((base.0 + uy * half) as f32, (base.1 - ux * half) as f32);
    tip.close();
    if let Some(tip) = tip.finish() {
        canvas.fill_path(&tip, ink, FillRule::Winding, at, None);
    }
}

fn style(size: f32) -> Style {
    Style { family: Some(crate::toolkit::text::FAMILY.into()), weight: 700, ..Style::new(size.max(1.0)) }
}

fn place(at: Transform, p: Pt) -> (f32, f32) {
    let mut q = tiny_skia::Point::from_xy(p.0 as f32, p.1 as f32);
    at.map_point(&mut q);
    (q.x, q.y)
}

fn words_at(canvas: &mut PixmapMut, at: Transform, fonts: &Fonts, p: Pt, words: &str, size: f64, colour: Rgb) -> f64 {
    let k = at.sx.max(f32::EPSILON);
    let laid = fonts.layout(words, &style(size as f32 * k), None);
    let (x, y) = place(at, (p.0, p.1 - size * 0.6));
    fonts.draw(canvas, &laid, x, y, colour_of(colour));
    p.0 + (laid.width / k) as f64
}

fn colour_of(c: Rgb) -> Colour {
    Colour::rgb(c.0 as f32, c.1 as f32, c.2 as f32)
}

fn spot_radius(thick: f64, unit: f64) -> f64 {
    (10.0 + 1.6 * thick) * unit
}

#[allow(clippy::too_many_arguments)]
fn number(canvas: &mut PixmapMut, at: Transform, fonts: &Fonts, p: Pt, n: u32, colour: Rgb, thick: f64, unit: f64) {
    number_in(canvas, at, fonts, p, n, colour, thick, unit, against(colour));
}

#[allow(clippy::too_many_arguments)]
fn number_in(canvas: &mut PixmapMut, at: Transform, fonts: &Fonts, p: Pt, n: u32, colour: Rgb, thick: f64, unit: f64, edge: Rgb) {
    let r = spot_radius(thick, unit);
    let Some(spot) = PathBuilder::from_circle(p.0 as f32, p.1 as f32, r as f32) else { return };
    canvas.fill_path(&spot, &paint(colour, 1.0), FillRule::Winding, at, None);
    canvas.stroke_path(&spot, &paint(edge, 0.55), &line((1.5 * unit) as f32, LineCap::Butt, LineJoin::Round), at, None);
    let k = at.sx.max(f32::EPSILON);
    let size = r as f32 * 1.15 * k;
    let laid = fonts.layout(&n.to_string(), &style(size), None);
    let baseline = laid.lines.first().map_or(size * 0.8, |l| l.baseline);
    let (cx, cy) = place(at, p);
    fonts.draw(canvas, &laid, cx - laid.width / 2.0, cy + size * 0.36 - baseline, colour_of(edge));
}

pub fn pixelate(data: &mut [u8], stride: usize, width: usize, height: usize, block: usize) {
    let block = block.max(1);
    for by in (0..height).step_by(block) {
        for bx in (0..width).step_by(block) {
            let (x1, y1) = ((bx + block).min(width), (by + block).min(height));
            let mut sum = [0u64; 4];
            for y in by..y1 {
                for x in bx..x1 {
                    let i = y * stride + x * 4;
                    for (c, s) in sum.iter_mut().enumerate() {
                        *s += data[i + c] as u64;
                    }
                }
            }
            let n = ((x1 - bx) * (y1 - by)) as u64;
            let avg = sum.map(|s| ((s + n / 2) / n) as u8);
            for y in by..y1 {
                for x in bx..x1 {
                    let i = y * stride + x * 4;
                    data[i..i + 4].copy_from_slice(&avg);
                }
            }
        }
    }
}

fn blurred(still: &Pixmap, r: Rect, block: usize) -> Option<Pixmap> {
    let x = r.x.round().max(0.0) as i32;
    let y = r.y.round().max(0.0) as i32;
    let w = ((r.x + r.w).round() as i32).min(still.width() as i32) - x;
    let h = ((r.y + r.h).round() as i32).min(still.height() as i32) - y;
    if w < 1 || h < 1 {
        return None;
    }
    let mut patch = still.clone_rect(tiny_skia::IntRect::from_xywh(x, y, w as u32, h as u32)?)?;
    pixelate(patch.data_mut(), w as usize * 4, w as usize, h as usize, block);
    Some(patch)
}

pub fn render<'a>(still: &Pixmap, marks: impl Iterator<Item = &'a Mark>, part: (i32, i32, i32, i32),
                  fonts: &Fonts) -> Option<Pixmap> {
    let (x, y, w, h) = part;
    let mut out = Pixmap::new(w.max(1) as u32, h.max(1) as u32)?;
    let at = Transform::from_translate(-x as f32, -y as f32);
    {
        let mut canvas = out.as_mut();
        canvas.draw_pixmap(0, 0, still.as_ref(), &PixmapPaint::default(), at, None);
        for m in marks {
            m.draw(&mut canvas, at, fonts, false);
        }
    }
    Some(out)
}

pub fn png(picture: &Pixmap) -> Option<Vec<u8>> {
    use image::ImageEncoder;
    let rgba: Vec<u8> = picture.pixels().iter().flat_map(|p| {
        let c = p.demultiply();
        [c.red(), c.green(), c.blue(), c.alpha()]
    }).collect();
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out)
        .write_image(&rgba, picture.width(), picture.height(), image::ExtendedColorType::Rgba8).ok()?;
    Some(out)
}

#[derive(Debug)]
pub struct History<T> {
    done: Vec<T>,
    undone: Vec<T>,
}

impl<T> Default for History<T> {
    fn default() -> Self {
        History { done: Vec::new(), undone: Vec::new() }
    }
}

impl<T> History<T> {
    pub fn push(&mut self, t: T) {
        self.done.push(t);
        self.undone.clear();
    }

    pub fn undo(&mut self) -> bool {
        match self.done.pop() {
            Some(t) => {
                self.undone.push(t);
                true
            }
            None => false,
        }
    }

    pub fn redo(&mut self) -> bool {
        match self.undone.pop() {
            Some(t) => {
                self.done.push(t);
                true
            }
            None => false,
        }
    }

    pub fn done(&self) -> &[T] {
        &self.done
    }

    pub fn can_undo(&self) -> bool {
        !self.done.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.undone.is_empty()
    }
}

pub fn next_number(marks: &[Mark]) -> u32 {
    marks.iter().filter(|m| matches!(m.shape, Shape::Number(..))).count() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_and_redo_walk_back_and_forth() {
        let mut h = History::default();
        assert!(!h.undo());
        h.push(1);
        h.push(2);
        assert!(h.undo());
        assert_eq!(h.done(), &[1]);
        assert!(h.can_redo());
        assert!(h.redo());
        assert_eq!(h.done(), &[1, 2]);
        assert!(!h.redo());
    }

    #[test]
    fn something_new_forgets_what_was_undone() {
        let mut h = History::default();
        h.push(1);
        h.push(2);
        h.undo();
        h.push(3);
        assert_eq!(h.done(), &[1, 3]);
        assert!(!h.can_redo());
    }

    #[test]
    fn blocks_are_the_average_of_what_was_there() {
        let mut data = vec![
            0, 0, 0, 255, 100, 200, 50, 255, 10, 10, 10, 255, 30, 30, 30, 255, 7, 7,
        ];
        pixelate(&mut data, 18, 4, 1, 2);
        assert_eq!(&data[0..8], &[50, 100, 25, 255, 50, 100, 25, 255]);
        assert_eq!(&data[8..16], &[20, 20, 20, 255, 20, 20, 20, 255]);
        assert_eq!(&data[16..], &[7, 7]);
    }

    #[test]
    fn a_ragged_last_block_averages_only_what_it_covers() {
        let mut data: Vec<u8> = (0..3).flat_map(|i| [i * 30, 0, 0, 255]).collect();
        pixelate(&mut data, 12, 3, 1, 2);
        assert_eq!(data[0], 15);
        assert_eq!(data[4], 15);
        assert_eq!(data[8], 60);
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_numbers_on_their_spots() {
        use crate::toolkit::theme::contrast;
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from) else { return };
        std::fs::create_dir_all(&dir).unwrap();
        let fonts = Fonts::new();
        let was = |c: Rgb| if 0.2126 * c.0 + 0.7152 * c.1 + 0.0722 * c.2 > 0.6 { BLACK } else { WHITE };
        let rules: [(&str, &dyn Fn(Rgb) -> Rgb); 2] = [("before", &was), ("after", &against)];
        for (when, ink) in rules {
            let mut pix = Pixmap::new(90 * COLOURS.len() as u32, 250).unwrap();
            pix.fill(tiny_skia::Color::from_rgba8(0x60, 0x68, 0x74, 0xff));
            for (i, (colour, _)) in COLOURS.iter().enumerate() {
                let x = 45.0 + 90.0 * i as f64;
                for (row, (thick, _)) in THICKNESS.iter().enumerate() {
                    number_in(&mut pix.as_mut(), Transform::identity(), &fonts, (x, 40.0 + 65.0 * row as f64), i as u32 + 1, *colour, *thick, 1.0, ink(*colour));
                }
                let stands = format!("{:.2}", contrast(colour_of(ink(*colour)), colour_of(*colour)));
                let laid = fonts.layout(&stands, &style(14.0), None);
                fonts.draw(&mut pix.as_mut(), &laid, x as f32 - laid.width / 2.0, 222.0, colour_of(WHITE));
            }
            pix.save_png(dir.join(format!("screenshot-numbers-{when}.png"))).unwrap();
        }
    }

    #[test]
    fn numbers_count_on_from_those_showing() {
        let spot = |n| Mark::begin(Tool::Number, (0.0, 0.0), WHITE, 6.0, 1.0, n);
        let line = Mark::begin(Tool::Line, (0.0, 0.0), WHITE, 6.0, 1.0, 0);
        assert_eq!(next_number(&[]), 1);
        assert_eq!(next_number(&[spot(1), line, spot(2)]), 3);
    }

    #[test]
    fn a_click_that_slipped_is_not_kept() {
        let mut line = Mark::begin(Tool::Line, (10.0, 10.0), WHITE, 6.0, 2.0, 0);
        line.drag((13.0, 12.0));
        assert!(!line.worth_keeping());
        line.drag((30.0, 12.0));
        assert!(line.worth_keeping());
        let words = Mark::begin(Tool::Text, (0.0, 0.0), WHITE, 6.0, 1.0, 0);
        assert!(!words.worth_keeping());
    }

    #[test]
    fn a_number_can_be_read_on_a_spot_of_any_colour() {
        use crate::toolkit::theme::{contrast, WORDS};
        assert_eq!(against(WHITE), BLACK);
        assert_eq!(against(BLACK), WHITE);
        assert_eq!(against(PURPLE), WHITE);
        let any = (0..=20).flat_map(|r| (0..=20).flat_map(move |g| (0..=20).map(move |b| (r as f64 / 20.0, g as f64 / 20.0, b as f64 / 20.0))));
        for c in COLOURS.iter().map(|(c, _)| *c).chain(any) {
            let stands = contrast(colour_of(against(c)), colour_of(c));
            assert!(stands >= WORDS, "the number on a spot of {c:?} stands {stands:.2} to one");
        }
    }

    #[test]
    fn the_saved_picture_carries_what_was_drawn() {
        let mut still = Pixmap::new(40, 20).unwrap();
        still.fill(tiny_skia::Color::BLACK);
        let mut m = Mark::begin(Tool::Box, (10.0, 2.0), WHITE, 4.0, 1.0, 0);
        m.drag((30.0, 18.0));
        let out = render(&still, [m].iter(), (10, 0, 30, 20), &Fonts::new()).unwrap();
        assert_eq!((out.width(), out.height()), (30, 20));
        let stride = out.width() as usize * 4;
        let data = out.data();
        let px = |x: usize, y: usize| &data[y * stride + x * 4..y * stride + x * 4 + 3];
        assert_eq!(px(0, 10), &[255, 255, 255]);
        assert_eq!(px(10, 10), &[0, 0, 0]);
        assert!(png(&out).is_some_and(|b| b.starts_with(b"\x89PNG")));
    }
}
