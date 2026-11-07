//! The ring of open windows, outside the favourites.

use std::f64::consts::PI;
use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::{Color, FillRule, Paint, Pixmap, PixmapMut, Transform};

use crate::toolkit::text::Style;
use crate::toolkit::theme::Sheet;
use crate::toolkit::widget::{drawing, Node, Pen, Rect};

use super::ctx::Msg;
use crate::common::in_use::Kind;
use crate::screenshot::cast_capture::Window;
use super::view::app_tile::{name_is_cut, paint_name, LABEL_SIZE, NAME_EDGE};
use super::view::paint::{circle, colour, piece_path, PIECE_SHADOW};

const ICON_LEAST: f64 = 8.0;
pub const BUTTON_PAD: f64 = 4.0;
const TITLE_LINE: f32 = 1.3;
const NAME_GAP: f64 = 2.0;
const TITLE_LINES: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes {
    pub width: f64,
    pub width_least: f64,
    pub gap: f64,
    pub part_gap: f64,
    pub icon_alone: f64,
    pub tab_pad: f64,
    pub name_width: f64,
}

impl Sizes {
    pub fn of(k: &Sheet) -> Sizes {
        Sizes { width: k.ring_windows_width as f64, width_least: k.ring_windows_width_least as f64, gap: k.ring_windows_gap as f64,
                part_gap: k.ring_windows_part_gap as f64, icon_alone: k.ring_windows_icon_alone as f64, tab_pad: k.ring_windows_tab_pad as f64, name_width: k.ring_windows_name_width as f64 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Part {
    pub start: f64,
    pub end: f64,
    pub mid: f64,
    pub icon_at: (f64, f64),
    pub icon: f64,
    pub name_top: f64,
    pub name_room: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    pub inner: f64,
    pub outer: f64,
    pub slot: f64,
    pub parts: Vec<Part>,
}

pub fn ring(n: usize, disc: f64, half: f64, k: &Sizes) -> Option<Ring> {
    let inner = disc + k.gap;
    let width = k.width.min(half - inner);
    if width < k.width_least {
        return None;
    }
    let outer = inner + width;
    let mid_r = inner + width / 2.0;
    let slot = tab_slot(n, mid_r, k);
    let inset = (k.part_gap / mid_r / 2.0).min(slot / 4.0);
    let mut parts = Vec::with_capacity(n);
    for i in 0..n {
        let mid = PI / 2.0 + ((n as f64 - 1.0) / 2.0 - i as f64) * slot;
        let (start, end) = (mid - slot / 2.0 + inset, mid + slot / 2.0 - inset);
        let arc = (end - start) * mid_r;
        let icon = k.icon_alone.min(width - 4.0).min(arc - 2.0).max(ICON_LEAST);
        let (mx, my) = (mid_r * mid.cos(), mid_r * mid.sin());
        let stack = icon + NAME_GAP + name_height();
        let (top, name_top) = (my - stack / 2.0, my - stack / 2.0 + icon + NAME_GAP);
        let mut room = k.name_width.min(arc - 2.0 * NAME_EDGE as f64);
        let holds = |room: f64| [mx - room / 2.0, mx + room / 2.0].iter().all(|&x| [name_top, name_top + name_height()].iter().all(|&y| holds(inner, outer, start, end, x, y)));
        while room > 0.0 && !holds(room) {
            room -= 2.0;
        }
        let least = icon + 2.0 * BUTTON_PAD - 2.0 * NAME_EDGE as f64;
        if room >= least && icon + NAME_GAP + name_height() + 2.0 * BUTTON_PAD <= width {
            parts.push(Part { start, end, mid, icon_at: (mx, top + icon / 2.0), icon, name_top, name_room: room });
        } else {
            parts.push(Part { start, end, mid, icon_at: (mx, my), icon, name_top: 0.0, name_room: 0.0 });
        }
    }
    Some(Ring { inner, outer, slot, parts })
}

fn holds(inner: f64, outer: f64, start: f64, end: f64, x: f64, y: f64) -> bool {
    let d = x.hypot(y);
    let mut a = y.atan2(x);
    while a < start - PI {
        a += 2.0 * PI;
    }
    while a > start + PI {
        a -= 2.0 * PI;
    }
    d >= inner - 1e-9 && d <= outer + 1e-9 && a >= start - 1e-9 && a <= end + 1e-9
}

fn name_height() -> f64 {
    (LABEL_SIZE * TITLE_LINE) as f64
}

fn tab_slot(n: usize, mid_r: f64, k: &Sizes) -> f64 {
    let fixed = (k.icon_alone.max(k.name_width) + 2.0 * k.tab_pad + k.part_gap) / mid_r;
    if n == 0 {
        fixed
    } else {
        fixed.min(2.0 * PI / n as f64)
    }
}

impl Ring {
    pub fn part_at(&self, dx: f64, dy: f64) -> Option<usize> {
        let r = dx.hypot(dy);
        let n = self.parts.len();
        if r < self.inner || r > self.outer || n == 0 {
            return None;
        }
        let from = self.parts[0].mid + self.slot / 2.0;
        let turned = (from - dy.atan2(dx)).rem_euclid(2.0 * PI);
        let i = (turned / self.slot) as usize;
        if i < n {
            Some(i)
        } else if n as f64 * self.slot >= 2.0 * PI - 1e-9 {
            Some(n - 1)
        } else {
            None
        }
    }

    pub fn button_of(&self, i: usize) -> [f64; 4] {
        let p = &self.parts[i];
        let (mut w, mut h) = (p.icon + 2.0 * BUTTON_PAD, p.icon + 2.0 * BUTTON_PAD);
        if p.name_room > 0.0 {
            w = w.max(p.name_room + 2.0 * NAME_EDGE as f64);
            h += NAME_GAP + name_height();
        }
        [p.icon_at.0 - w / 2.0, p.icon_at.1 - p.icon / 2.0 - BUTTON_PAD, w, h]
    }
}

pub fn grouped<T>(items: Vec<T>, program: impl Fn(&T) -> String) -> Vec<T> {
    let mut order: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut keyed: Vec<(usize, T)> = Vec::with_capacity(items.len());
    for it in items {
        let next = order.len();
        let at = *order.entry(program(&it)).or_insert(next);
        keyed.push((at, it));
    }
    keyed.sort_by_key(|(at, _)| *at);
    keyed.into_iter().map(|(_, it)| it).collect()
}

#[derive(Clone, Debug)]
pub struct Look {
    pub icons: Vec<String>,
    pub program: String,
    pub name: String,
    pub front: bool,
    pub hot: bool,
    pub uses: Vec<Kind>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UseDot {
    pub at: (f32, f32),
    pub across: f32,
    pub kind: Kind,
}

pub fn use_dots(ring: &Ring, i: usize, dot: f32, kinds: &[Kind]) -> Vec<UseDot> {
    let p = &ring.parts[i];
    let mid_r = (ring.inner + ring.outer) / 2.0;
    let edge = mid_r + p.icon / 2.0;
    let front = ring.outer - dot as f64;
    let r = (edge + front) / 2.0;
    let arc = (p.end - p.start) * r;
    let across = (dot as f64).min((front - edge - 1.0).max(2.0)).min(arc / (USE_DOTS_MOST as f64 + 1.0)).max(2.0);
    let step = across * USE_DOT_STEP;
    let n = kinds.len();
    kinds.iter().enumerate().map(|(k, kind)| {
        let a = p.mid + ((n as f64 - 1.0) / 2.0 - k as f64) * step / r;
        UseDot { at: ((r * a.cos()) as f32, (r * a.sin()) as f32), across: across as f32, kind: *kind }
    }).collect()
}

const USE_DOTS_MOST: usize = 3;
const USE_DOT_STEP: f64 = 1.5;

pub fn tab_words(title: &str, uses: &[crate::ring::using::Item]) -> String {
    let mut out = shown_title(title);
    for u in uses {
        out.push('\n');
        out.push_str(&shown_title(&u.words()));
    }
    out
}

const TITLE_LETTERS_MOST: usize = 120;

pub fn shown_title(title: &str) -> String {
    crate::common::plain::line(&title.chars().take(TITLE_LETTERS_MOST).collect::<String>())
}

const TITLE_SEPARATORS: [&str; 6] = [" - ", " \u{2013} ", " \u{2014} ", " | ", " \u{b7} ", " \u{2022} "];

pub fn without_program(title: &str, program: &str) -> String {
    let title = shown_title(title);
    let title = title.trim();
    let program = program.trim().to_lowercase();
    if program.is_empty() {
        return title.to_string();
    }
    let cut = TITLE_SEPARATORS.iter().filter_map(|sep| title.rfind(sep).map(|at| (at, sep.len()))).max_by_key(|(at, _)| *at);
    let Some((at, len)) = cut else { return title.to_string() };
    let (head, tail) = (title[..at].trim(), title[at + len..].trim().to_lowercase());
    let words = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    let (a, b) = (words(&tail), words(&program));
    let names = !a.is_empty() && !b.is_empty() && (a == b || (b.len() > a.len() && (b.starts_with(&a) || b.ends_with(&a))) || (a.len() > b.len() && a.ends_with(&b)));
    if !head.is_empty() && names {
        head.to_string()
    } else {
        title.to_string()
    }
}

pub const NO_ICON: &str = crate::common::apps::GENERIC_ICONS[0];

pub fn paint_icon(pm: &mut PixmapMut, pen: &Pen, icons: &[String], x: f32, y: f32, size: f32) {
    let s = pen.scale;
    let whole = s.round().max(1.0) as u32;
    let px = ((size * s) / whole as f32).round() as u32;
    let (x, y) = ((x * s).round() as i32, (y * s).round() as i32);
    let drawn = icons.iter().any(|n| pen.icons.draw(pm, n, x, y, px, whole, pen.theme.fg).is_some());
    if !drawn {
        pen.icons.draw(pm, NO_ICON, x, y, px, whole, pen.theme.fg);
    }
}

fn tab_fill(th: &crate::toolkit::theme::Theme, look: &Look) -> tiny_skia::Color {
    let sheet = crate::toolkit::theme::sheet();
    let c = if look.hot { th.hover } else if look.front { crate::toolkit::theme::mix(th.bg, th.fg, sheet.ring_windows_tab_front / 100.0) } else { crate::toolkit::theme::mix(th.bg, th.fg, sheet.ring_windows_tab_firm / 100.0) };
    colour(c, 1.0)
}

pub fn tab_node(ring: &Ring, i: usize, look: &Look) -> Node<Msg> {
    let b = ring.button_of(i);
    let (icons, icon, program, look) = (look.icons.clone(), ring.parts[i].icon as f32, look.program.clone(), look.clone());
    let shown = ring.parts[i].name_room > 0.0 && !program.is_empty();
    let cut = Rc::new(Cell::new(false));
    let was_cut = cut.clone();
    drawing((b[2] as f32, b[3] as f32), look.name.clone(), move |pm, pen| {
        let pad = BUTTON_PAD as f32;
        paint_icon(pm, pen, &icons, (pm.width() as f32 / pen.scale - icon) / 2.0, pad, icon);
        if shown {
            let fill = tab_fill(pen.theme, &look);
            let under = crate::toolkit::colour::Colour { r: fill.red(), g: fill.green(), b: fill.blue(), a: 1.0 };
            paint_name(pm, pen, &program, (pad + icon + NAME_GAP as f32) * pen.scale, pen.theme.words_on(&[under]));
            was_cut.set(name_is_cut(pen, &program, pm.width() as f32));
        }
    }).named(shown).tip_if_cut(cut)
}

pub fn paint(pm: &mut PixmapMut, pen: &Pen, ring: &Ring, looks: &[Look], centre: (f32, f32), dot: f32) {
    let th = pen.theme;
    let s = pen.scale;
    let t = Transform::from_scale(s, s);
    let sheet = crate::toolkit::theme::sheet();
    let (cx, cy) = centre;
    if let (Some(out), Some(inn)) = (circle(cx, cy, ring.outer as f32), circle(cx, cy, ring.inner as f32)) {
        let mut pb = tiny_skia::PathBuilder::new();
        pb.push_path(&out);
        pb.push_path(&inn);
        if let Some(track) = pb.finish() {
            let ink = crate::toolkit::theme::mix(th.bg, th.fg, crate::toolkit::theme::sheet().ring_windows_track / 100.0);
            let mut paint = Paint::default();
            paint.set_color(colour(ink, 1.0));
            paint.anti_alias = true;
            pm.fill_path(&track, &paint, FillRule::EvenOdd, t, None);
            paint.set_color(colour(crate::toolkit::theme::mix(ink, th.ring_edge, sheet.ring_edge_strength), 1.0));
            pm.stroke_path(&track, &paint, &tiny_skia::Stroke { width: crate::toolkit::theme::sheet().ring_edge_width, ..tiny_skia::Stroke::default() }, t, None);
        }
    }
    for (i, (part, look)) in ring.parts.iter().zip(looks).enumerate() {
        let Some(path) = piece_path(cx, cy, ring.inner as f32, ring.outer as f32, part.start as f32, part.end as f32) else { continue };
        for (dy, a) in PIECE_SHADOW {
            fill(pm, &path, colour(th.shadow, a), t.pre_translate(0.0, dy));
        }
        fill(pm, &path, tab_fill(th, look), t);
        for d in use_dots(ring, i, dot, &look.uses) {
            if let Some(c) = circle(cx + d.at.0, cy + d.at.1, d.across / 2.0) {
                let (r, g, b) = d.kind.colour();
                fill(pm, &c, Color::from_rgba(r as f32, g as f32, b as f32, 1.0).unwrap_or(Color::BLACK), t);
            }
        }
    }
}

fn fill(pm: &mut PixmapMut, p: &tiny_skia::Path, c: tiny_skia::Color, t: Transform) {
    let mut paint = Paint::default();
    paint.set_color(c);
    paint.anti_alias = true;
    pm.fill_path(p, &paint, FillRule::Winding, t, None);
}

pub fn capture_id(list: &[Window], app_id: &str, title: &str, nth: usize) -> Option<String> {
    list.iter().filter(|w| w.app_id == app_id && w.title == title).nth(nth).map(|w| w.id.clone())
}

pub fn picture_box(diameter: f32, margin: f32, corner: f32, w: f32, h: f32) -> (f32, f32) {
    let reach = (diameter / 2.0 - margin).max(1.0);
    if w <= 0.0 || h <= 0.0 {
        return (2.0 * reach, 2.0 * reach);
    }
    let furthest = |k: f32| {
        let (bw, bh) = (w * k, h * k);
        let c = corner.min(bw / 2.0).min(bh / 2.0).max(0.0);
        ((bw / 2.0 - c).hypot(bh / 2.0 - c) + c, bw, bh)
    };
    let (mut lo, mut hi) = (0.0f32, 4.0 * reach / w.min(h));
    for _ in 0..40 {
        let mid = (lo + hi) / 2.0;
        if furthest(mid).0 <= reach { lo = mid } else { hi = mid }
    }
    let (_, bw, bh) = furthest(lo);
    (bw.floor(), bh.floor())
}

#[derive(Clone)]
pub struct Middle {
    pub name: String,
    pub title: String,
    pub show_title: bool,
    pub icons: Vec<String>,
    pub picture: Option<Rc<Pixmap>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MiddleBoxes {
    pub picture: [f32; 4],
    pub title: [f32; 4],
}

fn title_room(size: f32) -> f32 {
    size * TITLE_LINE * TITLE_LINES as f32
}

#[derive(Clone, Copy)]
pub struct MiddleSpec {
    pub diameter: f32,
    pub margin: f32,
    pub corner: f32,
    pub gap: f32,
    pub room: f32,
    pub w: f32,
    pub h: f32,
    pub most: Option<f32>,
}

pub fn middle_boxes(spec: &MiddleSpec) -> MiddleBoxes {
    let MiddleSpec { diameter, margin, corner, gap, room, w, h, most } = *spec;
    let reach = (diameter / 2.0 - margin).max(1.0);
    let spare = reach - 1.0;
    let (mut bw, mut bh) = picture_box(diameter, margin, corner, w, h);
    if let Some(m) = most {
        let k = (m / bw.max(bh)).min(1.0);
        (bw, bh) = (bw * k, bh * k);
    }
    let stack = |k: f32| {
        let (pw, ph) = ((bw * k).floor(), (bh * k).floor());
        let total = ph + gap + room;
        let top = -total / 2.0;
        let c = corner.min(pw / 2.0).min(ph / 2.0).max(0.0);
        let up = (pw / 2.0 - c).hypot(top + c) + c;
        (up.max(total / 2.0), pw, ph)
    };
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    if stack(1.0).0 <= spare {
        lo = 1.0;
    } else {
        for _ in 0..40 {
            let mid = (lo + hi) / 2.0;
            if stack(mid).0 <= spare { lo = mid } else { hi = mid }
        }
    }
    let (_, pw, ph) = stack(lo);
    let total = ph + gap + room;
    let top = ((diameter - total) / 2.0).round();
    let bottom = total / 2.0;
    let across = (2.0 * (spare * spare - bottom * bottom).max(0.0).sqrt()).floor();
    MiddleBoxes {
        picture: [((diameter - pw) / 2.0).round(), top, pw, ph],
        title: [((diameter - across) / 2.0).round(), top + ph + gap, across, room],
    }
}

fn title_layout(fonts: &crate::toolkit::text::Fonts, title: &str, style: &Style, wide: f32) -> crate::toolkit::text::Layout {
    let whole = fonts.layout(title, style, Some(wide));
    if whole.lines.len() <= TITLE_LINES {
        return whole;
    }
    let words: Vec<&str> = title.split_whitespace().collect();
    let with = |n: usize| fonts.layout(&format!("{}\u{2026}", words[..n].join(" ")), style, Some(wide));
    let (mut lo, mut hi) = (0usize, words.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if with(mid).lines.len() <= TITLE_LINES { lo = mid } else { hi = mid - 1 }
    }
    if lo == 0 {
        let dots = fonts.layout("\u{2026}", style, None);
        return whole.cut_after(TITLE_LINES, wide, &dots);
    }
    with(lo)
}

pub fn middle(diameter: f32, m: Middle) -> Node<Msg> {
    let name = shown_title(&m.name);
    crate::toolkit::widget::drawing((diameter, diameter), name, move |pm, pen| {
        let s = pen.scale;
        let k = crate::toolkit::theme::sheet();
        let (w, h) = m.picture.as_ref().map(|p| (p.width() as f32, p.height() as f32)).unwrap_or((1.0, 1.0));
        let most = m.picture.is_none().then_some(k.ring_windows_picture_icon);
        let (gap, room) = if m.show_title { (k.ring_windows_title_gap, title_room(k.ring_windows_title)) } else { (0.0, 0.0) };
        let boxes = middle_boxes(&MiddleSpec { diameter, margin: k.ring_windows_picture_margin, corner: k.ring_windows_picture_corner, gap, room, w, h, most });
        let [px, py, pw, ph] = boxes.picture;
        match &m.picture {
            Some(p) => {
                let (bw, bh) = ((pw * s).round(), (ph * s).round());
                let (x, y) = ((px * s).round(), (py * s).round());
                if let Some(path) = crate::toolkit::ui::rounded(Rect { x, y, w: bw, h: bh }, k.ring_windows_picture_corner * s) {
                    let place = Transform::from_translate(x, y).pre_scale(bw / p.width() as f32, bh / p.height() as f32);
                    let shader = tiny_skia::Pattern::new((**p).as_ref(), tiny_skia::SpreadMode::Pad, tiny_skia::FilterQuality::Bilinear, 1.0, place);
                    let paint = Paint { shader, anti_alias: true, ..Paint::default() };
                    pm.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
                }
            }
            None => paint_icon(pm, pen, &m.icons, px, py, pw.min(ph)),
        }
        let [tx, ty, tw, _] = boxes.title;
        if m.show_title && !m.title.is_empty() && tw > 0.0 {
            let mut style = Style::new(k.ring_windows_title * s);
            style.align = crate::toolkit::text::Align::Centre;
            let l = title_layout(pen.fonts, &m.title, &style, tw * s);
            pen.fonts.draw(pm, &l, tx * s, ty * s, pen.theme.fg);
        }
    })
    .passes()
    .named(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes() -> Sizes {
        Sizes { width: 88.0, width_least: 56.0, gap: 8.0, part_gap: 4.0, icon_alone: 36.0, tab_pad: 6.0, name_width: 64.0 }
    }

    fn mid_r(r: &Ring) -> f64 {
        (r.inner + r.outer) / 2.0
    }

    const BOTTOM: f64 = PI / 2.0;

    fn names(list: &[(&str, &str)]) -> Vec<String> {
        grouped(list.to_vec(), |w| w.0.to_string()).iter().map(|w| format!("{}:{}", w.0, w.1)).collect()
    }

    #[test]
    fn windows_of_one_program_are_gathered_in_their_order() {
        assert!(names(&[]).is_empty());
        assert_eq!(names(&[("brave", "a")]), ["brave:a"]);
        assert_eq!(names(&[("brave", "a"), ("vlc", "b"), ("brave", "c")]), ["brave:a", "brave:c", "vlc:b"]);
        assert_eq!(names(&[("brave", "1"), ("vlc", "2"), ("brave", "3"), ("vlc", "4"), ("brave", "5")]),
                   ["brave:1", "brave:3", "brave:5", "vlc:2", "vlc:4"]);
        assert_eq!(names(&[("vlc", "1"), ("brave", "2"), ("vlc", "3")]), ["vlc:1", "vlc:3", "brave:2"]);
        assert_eq!(names(&[("a", "1"), ("b", "2"), ("a", "3"), ("c", "4"), ("a", "5")]), ["a:1", "a:3", "a:5", "b:2", "c:4"]);
        assert_eq!(names(&[("a", "1"), ("b", "2"), ("c", "3")]), ["a:1", "b:2", "c:3"]);
        assert_eq!(names(&[("a", "1"), ("a", "1")]).len(), 2);
    }

    #[test]
    fn a_full_ring_of_grouped_windows_keeps_every_tab() {
        let k = sizes();
        let list: Vec<(String, String)> = (0..60).map(|i| (format!("p{}", i % 4), format!("w{i}"))).collect();
        let g = grouped(list.clone(), |w| w.0.clone());
        assert_eq!(g.len(), 60);
        let mut seen: Vec<_> = g.iter().map(|w| w.1.clone()).collect();
        seen.sort();
        let mut want: Vec<_> = list.iter().map(|w| w.1.clone()).collect();
        want.sort();
        assert_eq!(seen, want);
        let r = ring(g.len(), 226.0, 500.0, &k).unwrap();
        assert_eq!(r.parts.len(), 60);
    }

    #[test]
    fn tabs_are_the_icon_plus_the_pad_each_side_and_pointing_finds_the_grouped_window() {
        let k = sizes();
        let list = [("brave", "one"), ("vlc", "two"), ("brave", "three"), ("gimp", "four"), ("brave", "five")];
        let g = grouped(list.to_vec(), |w| w.0.to_string());
        let r = ring(g.len(), 226.0, 500.0, &k).unwrap();
        let m = mid_r(&r);
        for (i, p) in r.parts.iter().enumerate() {
            let arc = (p.end - p.start) * m;
            assert!((arc - (k.icon_alone.max(k.name_width) + 2.0 * k.tab_pad)).abs() < 1e-9, "tab {i} is {arc} along");
            assert_eq!(r.part_at(m * p.mid.cos(), m * p.mid.sin()), Some(i));
            let near = |a: f64| r.part_at(m * a.cos(), m * a.sin());
            assert_eq!(near(p.start + 1e-6), Some(i));
            assert_eq!(near(p.end - 1e-6), Some(i));
        }
        let order: Vec<&str> = (0..g.len()).map(|i| g[i].1).collect();
        assert_eq!(order, ["one", "three", "five", "two", "four"]);
        assert_eq!(g[r.part_at(m * r.parts[1].mid.cos(), m * r.parts[1].mid.sin()).unwrap()].1, "three");
        assert!((r.parts[0].end - r.parts[0].start) * m > k.icon_alone + 2.0 * k.part_gap - 1e-9);
    }

    #[test]
    fn hundreds_of_windows_never_make_a_negative_arc() {
        for n in [100usize, 470, 800, 3000] {
            if let Some(r) = ring(n, 226.0, 500.0, &sizes()) {
                assert!(r.parts.iter().all(|p| p.end >= p.start), "{n} windows");
            }
        }
    }

    #[test]
    fn grouping_keeps_first_window_order_of_programs() {
        let g = grouped(vec![("a", 1), ("b", 1), ("a", 2), ("c", 1), ("b", 2)], |t| t.0.to_string());
        assert_eq!(g, vec![("a", 1), ("a", 2), ("b", 1), ("b", 2), ("c", 1)]);
    }

    #[test]
    fn nothing_open_is_the_bare_track() {
        let r = ring(0, 226.0, 400.0, &sizes()).unwrap();
        assert!(r.parts.is_empty());
        assert!((r.outer - r.inner - 88.0).abs() < 1e-9);
        for (dx, dy) in [(0.0, 300.0), (-300.0, 0.0), (0.0, -300.0), (300.0, 0.0)] {
            assert_eq!(r.part_at(dx, dy), None, "a click on the bare track is on no window");
        }
    }

    #[test]
    fn one_window_is_one_fixed_tab_at_six_oclock() {
        let k = sizes();
        let r = ring(1, 226.0, 500.0, &k).unwrap();
        assert_eq!(r.parts.len(), 1);
        let p = &r.parts[0];
        assert!((p.mid - BOTTOM).abs() < 1e-9, "the one tab is at the bottom");
        assert!(p.icon_at.0.abs() < 1e-9 && p.icon_at.1 > 0.0, "{:?}", p.icon_at);
        let arc = (p.end - p.start + k.part_gap / mid_r(&r)) * mid_r(&r);
        assert!((arc - (k.icon_alone.max(k.name_width) + 2.0 * k.tab_pad + k.part_gap)).abs() < 1e-9, "{arc}");
        assert!((p.icon - k.icon_alone).abs() < 1e-9);
        assert!(p.end - p.start < PI / 2.0);
        assert_eq!(r.part_at(0.0, mid_r(&r)), Some(0));
        assert_eq!(r.part_at(0.0, -mid_r(&r)), None, "twelve o'clock is bare track");
    }

    #[test]
    fn many_windows_are_centred_balanced_equal_and_grow_round_to_left_and_right() {
        let k = sizes();
        let mut width_before = 0.0;
        for n in 1usize..=20 {
            let r = ring(n, 226.0, 500.0, &k).unwrap();
            assert_eq!(r.parts.len(), n);
            let full = n as f64 * r.slot >= 2.0 * PI - 1e-9;
            let sizes_of: Vec<f64> = r.parts.iter().map(|p| p.end - p.start).collect();
            for w in &sizes_of {
                assert!((w - sizes_of[0]).abs() < 1e-9, "{n}: unequal tabs");
            }
            let fixed = (k.icon_alone.max(k.name_width) + 2.0 * k.tab_pad + k.part_gap) / mid_r(&r);
            if !full {
                assert!((r.slot - fixed).abs() < 1e-12, "{n}: the size moved before the ring was full");
            }
            let (first, last) = (&r.parts[0], &r.parts[n - 1]);
            assert!(((first.mid - BOTTOM) + (last.mid - BOTTOM)).abs() < 1e-9, "{n}: off centre");
            if n > 1 {
                if n as f64 * r.slot <= PI {
                    assert!(first.icon_at.0 < last.icon_at.0, "{n}: the order is not left to right");
                    for pair in r.parts.windows(2) {
                        assert!(pair[0].icon_at.0 <= pair[1].icon_at.0 + 1e-9, "{n}: out of order along the bottom");
                    }
                }
                for pair in r.parts.windows(2) {
                    assert!((pair[0].mid - pair[1].mid - r.slot).abs() < 1e-9, "{n}: tabs not evenly in a row");
                    assert!(pair[0].start >= pair[1].end, "{n}: tabs overlap");
                }
            }
            let group = n as f64 * r.slot;
            assert!(group + 1e-12 >= width_before, "{n}: the group shrank");
            width_before = group;
            for (i, p) in r.parts.iter().enumerate() {
                let m = mid_r(&r);
                assert_eq!(r.part_at(m * p.mid.cos(), m * p.mid.sin()), Some(i), "{n}: tab {i}");
            }
            if !full {
                assert_eq!(r.part_at(0.0, -mid_r(&r)), None, "{n}: the top is bare track");
            }
        }
    }

    #[test]
    fn each_new_window_widens_the_group_on_both_sides_in_turn() {
        let k = sizes();
        let ends = |n: usize| {
            let r = ring(n, 226.0, 500.0, &k).unwrap();
            let (a, b) = (r.parts[0].mid + r.slot / 2.0, r.parts[n - 1].mid - r.slot / 2.0);
            (a, b)
        };
        let (mut left, mut right) = ends(1);
        for n in 2usize..=12 {
            let (l, r) = ends(n);
            assert!(l > left && r < right, "{n}: the group did not grow both ways");
            assert!(((l - left) - (right - r)).abs() < 1e-9, "{n}: it grew more on one side");
            (left, right) = (l, r);
        }
    }

    #[test]
    fn when_the_ring_is_full_the_tabs_shrink_together_and_not_before() {
        let k = sizes();
        let r = ring(2, 226.0, 500.0, &k).unwrap();
        let most = (2.0 * PI / r.slot).floor() as usize;
        let at_most = ring(most, 226.0, 500.0, &k).unwrap();
        assert!((at_most.slot - r.slot).abs() < 1e-12, "full ring but not yet smaller");
        let over = ring(most + 1, 226.0, 500.0, &k).unwrap();
        assert!((over.slot - 2.0 * PI / (most + 1) as f64).abs() < 1e-12, "the tabs share the whole ring equally");
        assert!(over.slot < at_most.slot);
        for step in 0..360 {
            let a = step as f64 * PI / 180.0;
            let m = mid_r(&over);
            assert!(over.part_at(m * a.cos(), m * a.sin()).is_some(), "{step} degrees is bare when the ring is full");
        }
        for pair in over.parts.windows(2) {
            assert!(pair[0].start >= pair[1].end - 1e-12);
        }
        let many = ring(80, 226.0, 500.0, &k).unwrap();
        assert!(many.parts.iter().all(|p| p.icon >= ICON_LEAST));
    }

    #[test]
    fn icons_sit_in_the_middle_of_the_ring_and_never_touch_until_the_least_size() {
        for n in [1usize, 3, 6, 8, 9, 20, 60] {
            let r = ring(n, 226.0, 500.0, &sizes()).unwrap();
            let mid = mid_r(&r);
            for p in &r.parts {
                let (top, bottom) = (p.icon_at.1 - p.icon / 2.0, if p.name_room > 0.0 { p.name_top + name_height() } else { p.icon_at.1 + p.icon / 2.0 });
                assert!((p.icon_at.0.hypot((top + bottom) / 2.0) - mid).abs() < 1e-9);
                assert!(p.icon >= ICON_LEAST);
            }
            for pair in r.parts.windows(2) {
                if pair[0].icon > ICON_LEAST && pair[1].icon > ICON_LEAST {
                    let d = (pair[0].icon_at.0 - pair[1].icon_at.0).hypot(pair[0].icon_at.1 - pair[1].icon_at.1);
                    assert!(d >= pair[0].icon, "{n}: icons overlap");
                }
            }
        }
    }

    #[test]
    fn a_point_outside_the_ring_is_on_no_tab() {
        let r = ring(7, 226.0, 500.0, &sizes()).unwrap();
        assert_eq!(r.part_at(0.0, 0.0), None);
        assert_eq!(r.part_at(r.outer + 5.0, 0.0), None);
        assert_eq!(r.part_at(0.0, r.inner - 5.0), None);
    }

    #[test]
    fn it_gives_way_on_a_small_screen_and_goes_when_it_cannot_fit() {
        let wide = ring(3, 226.0, 400.0, &sizes()).unwrap();
        assert!((wide.outer - wide.inner - 88.0).abs() < 1e-9);
        let narrow = ring(3, 226.0, 226.0 + 8.0 + 70.0, &sizes()).unwrap();
        assert!((narrow.outer - narrow.inner - 70.0).abs() < 1e-9);
        assert!(narrow.outer <= 226.0 + 8.0 + 70.0 + 1e-9);
        assert!(ring(3, 226.0, 226.0 + 8.0 + 40.0, &sizes()).is_none());
    }

    #[test]
    fn a_window_is_found_for_its_picture_by_program_and_title_and_its_place_among_alike() {
        let w = |id: &str, title: &str, app: &str| Window { id: id.into(), title: title.into(), app_id: app.into() };
        let list = [w("a", "BBC", "firefox"), w("b", "Weather", "firefox"), w("c", "BBC", "firefox"), w("d", "BBC", "brave")];
        assert_eq!(capture_id(&list, "firefox", "BBC", 0).as_deref(), Some("a"));
        assert_eq!(capture_id(&list, "firefox", "BBC", 1).as_deref(), Some("c"));
        assert_eq!(capture_id(&list, "firefox", "Weather", 0).as_deref(), Some("b"));
        assert_eq!(capture_id(&list, "firefox", "BBC", 2), None);
        assert_eq!(capture_id(&list, "brave", "Other", 0), None);
    }

    #[test]
    fn the_program_name_at_the_end_of_a_title_is_dropped() {
        for (title, program, want) in [
            ("Holiday Photos: pictures kept in your albums - Brave", "Brave", "Holiday Photos: pictures kept in your albums"),
            ("Inbox \u{2014} Brave", "Brave", "Inbox"),
            ("Inbox | Brave", "brave", "Inbox"),
            ("Inbox \u{2013} BRAVE", "Brave", "Inbox"),
            ("BBC News - Home - Mozilla Firefox", "Firefox", "BBC News - Home"),
            ("BBC News - Home \u{2014} Mozilla Firefox", "Mozilla Firefox", "BBC News - Home"),
            ("Inbox - Brave", "Brave Web Browser", "Inbox"),
            ("Report.odt - LibreOffice Writer", "LibreOffice Writer", "Report.odt"),
            ("Report.odt - Writer", "LibreOffice Writer", "Report.odt"),
            ("notes.txt - Visual Studio Code", "Visual Studio Code", "notes.txt"),
            ("Poster.xcf - GIMP", "GIMP", "Poster.xcf"),
            ("Home - Files", "Files", "Home"),
            ("  Inbox  -  Brave  ", "Brave", "Inbox"),
            ("Brave - Brave", "Brave", "Brave"),
        ] {
            assert_eq!(without_program(title, program), want, "{title} / {program}");
        }
        for (title, program) in [
            ("Brave new world - Wikipedia", "Brave"),
            ("Brave", "Brave"),
            ("Why Firefox is fast", "Firefox"),
            ("Inbox - Bravery", "Brave"),
            ("Inbox-Brave", "Brave"),
            ("Inbox - Brave", "Chromium"),
            ("Inbox - Brave", ""),
            ("Brave Browser Nightly build - Notes", "Brave"),
            ("Notes - Brave tips", "Brave"),
            ("", "Brave"),
            ("", ""),
        ] {
            assert_eq!(without_program(title, program), title, "{title} / {program}");
        }
        assert_eq!(without_program("A - Brave - Brave", "Brave"), "A - Brave");
        assert!(!without_program("a\nb - Brave", "Brave").contains(char::is_control));
        assert!(without_program(&"w".repeat(100_000), "Brave").chars().count() <= TITLE_LETTERS_MOST);
    }

    fn inside(b: [f32; 4], corner: f32, diameter: f32, margin: f32) -> bool {
        let reach = diameter / 2.0 - margin + 0.01;
        let (c, m) = (corner.min(b[2] / 2.0).min(b[3] / 2.0), diameter / 2.0);
        let (hx, hy) = (b[2] / 2.0 - c, b[3] / 2.0 - c);
        let (cx, cy) = (b[0] + b[2] / 2.0 - m, b[1] + b[3] / 2.0 - m);
        [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)].iter().all(|(sx, sy)| (cx + sx * hx).hypot(cy + sy * hy) + c <= reach)
    }

    #[test]
    fn the_title_has_its_own_place_under_the_picture_inside_the_disc() {
        let k = crate::toolkit::theme::sheet();
        let room = title_room(k.ring_windows_title);
        for diameter in [340.0f32, 400.0, 504.0, 700.0] {
            for (w, h) in [(1280.0f32, 800.0f32), (1920.0, 1080.0), (800.0, 800.0), (600.0, 900.0), (300.0, 1200.0), (2400.0, 300.0), (10.0, 10.0), (1.0, 1.0)] {
                for most in [None, Some(k.ring_windows_picture_icon)] {
                    let b = middle_boxes(&MiddleSpec { diameter, margin: k.ring_windows_picture_margin, corner: k.ring_windows_picture_corner, gap: k.ring_windows_title_gap, room, w, h, most });
                    let at = format!("{diameter}: {w}x{h} {most:?}: {b:?}");
                    assert!(inside(b.picture, k.ring_windows_picture_corner, diameter, k.ring_windows_picture_margin), "{at}: the picture leaves the disc");
                    assert!(inside(b.title, 0.0, diameter, k.ring_windows_picture_margin), "{at}: the title leaves the disc");
                    assert!(b.picture[1] + b.picture[3] + k.ring_windows_title_gap <= b.title[1] + 0.51, "{at}: the title touches the picture");
                    assert!(b.title[3] >= room - 0.01 && b.title[2] > 0.0, "{at}: no room for the title");
                    assert!(((b.picture[0] + b.picture[2] / 2.0) - diameter / 2.0).abs() <= 1.0, "{at}: the picture is not centred");
                    assert!(((b.title[0] + b.title[2] / 2.0) - diameter / 2.0).abs() <= 1.0, "{at}: the title is not centred");
                    assert!(b.picture[2] >= 40.0 || w.min(h) < 20.0 || w / h > 3.0 || h / w > 3.0, "{at}: the picture is tiny");
                }
            }
        }
    }

    #[test]
    fn with_the_title_switched_off_the_picture_has_the_whole_middle() {
        let k = crate::toolkit::theme::sheet();
        for diameter in [340.0f32, 504.0] {
            for (w, h) in [(1280.0f32, 800.0f32), (600.0, 900.0)] {
                let (bw, bh) = picture_box(diameter, k.ring_windows_picture_margin, k.ring_windows_picture_corner, w, h);
                let off = middle_boxes(&MiddleSpec { diameter, margin: k.ring_windows_picture_margin, corner: k.ring_windows_picture_corner, gap: 0.0, room: 0.0, w, h, most: None });
                assert!((off.picture[2] - bw).abs() <= 3.0 && (off.picture[3] - bh).abs() <= 3.0, "{diameter}: {w}x{h}: {off:?} against {bw}x{bh}");
                assert!(((off.picture[1] + off.picture[3] / 2.0) - diameter / 2.0).abs() <= 1.0, "centred");
                let on = middle_boxes(&MiddleSpec { diameter, margin: k.ring_windows_picture_margin, corner: k.ring_windows_picture_corner, gap: k.ring_windows_title_gap, room: title_room(k.ring_windows_title), w, h, most: None });
                assert!(on.picture[3] <= off.picture[3], "the title takes room from the picture, not the other way");
            }
        }
    }

    #[test]
    fn a_long_title_is_two_lines_then_an_ellipsis_inside_its_room() {
        let fonts = crate::toolkit::text::Fonts::new();
        let k = crate::toolkit::theme::sheet();
        let room = title_room(k.ring_windows_title);
        for scale in [1.0f32, 1.5, 2.0] {
            for diameter in [340.0f32, 504.0] {
                let b = middle_boxes(&MiddleSpec { diameter, margin: k.ring_windows_picture_margin, corner: k.ring_windows_picture_corner, gap: k.ring_windows_title_gap, room, w: 1280.0, h: 800.0, most: None });
                let wide = b.title[2] * scale;
                let mut style = Style::new(k.ring_windows_title * scale);
                style.align = crate::toolkit::text::Align::Centre;
                for title in ["Holiday Photos: pictures kept in your albums", "Short", &"word ".repeat(40), &"w".repeat(200)] {
                    let l = title_layout(&fonts, title, &style, wide);
                    assert!(l.lines.len() <= TITLE_LINES, "{title}: {} lines", l.lines.len());
                    assert!(l.height <= room * scale + 0.5, "{title}: {} tall in {}", l.height, room * scale);
                    assert!(l.width <= wide + 0.5, "{title}: {} wide in {wide}", l.width);
                }
                let long = "Holiday Photos: pictures kept in your albums and more besides. ".repeat(3);
                let l = title_layout(&fonts, &long, &style, wide);
                let shown = l.whole().to_string();
                assert!(shown.ends_with('\u{2026}'), "a long title ends with an ellipsis: {shown}");
                let kept: Vec<&str> = shown.trim_end_matches('\u{2026}').split_whitespace().collect();
                assert!(!kept.is_empty() && long.split_whitespace().zip(&kept).all(|(a, b)| a == *b), "only whole words are kept: {shown}");
            }
        }
    }

    #[test]
    fn the_middle_picture_lets_the_pointer_through() {
        let code = crate::common::source::code(include_str!("open_windows.rs"));
        let body = code.split("pub fn middle(").nth(1).unwrap().split("\n}\n").next().unwrap();
        assert!(body.contains(".passes()"), "the picture over the ring takes the pointer from the part under it");
    }

    #[test]
    fn two_windows_of_one_program_are_two_parts_and_each_has_its_own_picture() {
        let w = |id: &str, title: &str, app: &str| Window { id: id.into(), title: title.into(), app_id: app.into() };
        let list = [w("a", "BBC News - Home", "firefox"), w("b", "Bank statement", "firefox")];
        let r = ring(list.len(), 226.0, 500.0, &sizes()).unwrap();
        assert_eq!(r.parts.len(), 2);
        let at = |i: usize| r.part_at(r.parts[i].icon_at.0, r.parts[i].icon_at.1);
        assert_eq!((at(0), at(1)), (Some(0), Some(1)));
        assert!(r.parts[0].icon_at.0 < 0.0 && r.parts[1].icon_at.0 > 0.0, "the first is left of the middle, the second right");
        assert_eq!(capture_id(&list, "firefox", "BBC News - Home", 0).as_deref(), Some("a"));
        assert_eq!(capture_id(&list, "firefox", "Bank statement", 0).as_deref(), Some("b"));
        let alike = [w("a", "Inbox", "firefox"), w("b", "Inbox", "firefox")];
        assert_eq!(capture_id(&alike, "firefox", "Inbox", 1).as_deref(), Some("b"));
    }

    #[test]
    fn a_hostile_title_is_one_short_line() {
        let long = "w".repeat(100_000);
        assert_eq!(shown_title(&long).chars().count(), TITLE_LETTERS_MOST);
        assert_eq!(shown_title("one\ntwo\u{7}\u{202e}x"), "one two  x");
        assert_eq!(tab_words("t", &[crate::ring::using::Item { kind: Kind::Camera, who: vec!["a\nb\u{202e}".into()], icons: vec![] }]), "t\nCamera in use by a b ");
        assert!(!shown_title("a\r\nb").contains(char::is_control));
    }

    #[test]
    fn use_dots_sit_outside_the_icon_inside_the_tab_for_every_count_and_crowd() {
        let kinds = [Kind::Microphone, Kind::Camera, Kind::Screen];
        for n in [1usize, 2, 3, 7, 12, 30, 60] {
            let r = ring(n, 226.0, 400.0, &sizes()).unwrap();
            for i in 0..n {
                let p = &r.parts[i];
                assert!(use_dots(&r, i, 6.0, &[]).is_empty(), "no use, no dot");
                for count in 1..=3 {
                    let dots = use_dots(&r, i, 6.0, &kinds[..count]);
                    assert_eq!(dots.len(), count);
                    for d in &dots {
                        let rad = (d.at.0 as f64).hypot(d.at.1 as f64);
                        assert!(rad - d.across as f64 / 2.0 >= mid_r(&r) + p.icon / 2.0 - 1e-3, "{n} windows: a dot touches the icon");
                        assert!(rad + d.across as f64 / 2.0 <= r.outer + 1e-3, "{n} windows: a dot leaves the tab");
                        assert!(d.across >= 2.0 && d.across <= 6.0);
                        let a = (d.at.1 as f64).atan2(d.at.0 as f64);
                        let off = (a - p.mid + PI).rem_euclid(2.0 * PI) - PI;
                        assert!(off.abs() <= (p.end - p.start) / 2.0 + 1e-6, "{n} windows: a dot is on the next tab");
                    }
                    for pair in dots.windows(2) {
                        let gap = ((pair[0].at.0 - pair[1].at.0) as f64).hypot((pair[0].at.1 - pair[1].at.1) as f64);
                        assert!(gap > pair[0].across as f64, "dots touch");
                    }
                }
            }
        }
    }

    #[test]
    fn use_dots_keep_the_corner_dots_colours_and_order_left_to_right_at_the_bottom() {
        let r = ring(1, 226.0, 400.0, &sizes()).unwrap();
        let dots = use_dots(&r, 0, 6.0, &[Kind::Microphone, Kind::Camera, Kind::Screen]);
        assert!(dots[0].at.0 < dots[1].at.0 && dots[1].at.0 < dots[2].at.0);
        for d in &dots {
            assert_eq!(d.kind.colour(), match d.kind { Kind::Microphone => crate::common::kit::microphone(), Kind::Camera => crate::common::kit::camera(), _ => crate::common::kit::screen() });
        }
    }

    #[test]
    fn the_tab_says_its_title_then_the_corner_labels_words() {
        let item = |kind, who: &str| crate::ring::using::Item { kind, who: vec![who.into()], icons: vec![] };
        assert_eq!(tab_words("BBC News", &[]), "BBC News");
        assert_eq!(tab_words("BBC News", &[item(Kind::Microphone, "Firefox")]), "BBC News\nMicrophone in use by Firefox");
        assert_eq!(tab_words("a\nb", &[item(Kind::Camera, "Firefox"), item(Kind::Screen, "Firefox")]), "a b\nCamera in use by Firefox\nScreen being recorded by Firefox");
    }
    const PROGRAMS: [&str; 4] = ["Brave", "VLC media player", "LibreOffice Writer", "Pneumonoultramicroscopicsilicovolcanoconiosis Viewer"];

    fn look(program: &str) -> Look {
        Look { icons: vec!["application-x-executable".into()], program: program.into(), name: format!("{program} window"), front: false, hot: false, uses: Vec::new() }
    }

    fn within(r: &Ring, p: &Part, x: f64, y: f64) -> bool {
        holds(r.inner, r.outer, p.start, p.end, x, y)
    }

    #[test]
    fn names_are_drawn_inside_their_segment_and_level() {
        let (mut named, mut crowded) = (0, 0);
        for disc in [260.0, 340.0, 436.0, 520.0, 700.0] {
            for n in [1usize, 2, 3, 5, 8, 12, 16, 20, 30, 60] {
                let r = ring(n, disc, 1000.0, &sizes()).unwrap();
                for (i, p) in r.parts.iter().enumerate() {
                    if p.name_room == 0.0 {
                        crowded += 1;
                        continue;
                    }
                    named += 1;
                    let b = r.button_of(i);
                    assert!((b[0] + b[2] / 2.0 - p.icon_at.0).abs() < 1e-9);
                    let (left, right) = (p.icon_at.0 - p.name_room / 2.0, p.icon_at.0 + p.name_room / 2.0);
                    assert!(p.name_top >= p.icon_at.1 + p.icon / 2.0, "the name is under the icon");
                    assert!(p.name_top + name_height() <= b[1] + b[3], "the name is inside its button");
                    for (x, y) in [(left, p.name_top), (right, p.name_top), (left, p.name_top + name_height()), (right, p.name_top + name_height())] {
                        assert!(within(&r, p, x, y), "{n} windows on a {disc} disc: the name of tab {i} reaches outside it at ({x:.1}, {y:.1})");
                    }
                    assert!(b[0] <= left + 1e-9 && b[0] + b[2] >= right - 1e-9 && b[1] <= p.icon_at.1 - p.icon / 2.0);
                }
            }
        }
        assert!(named > 40, "only {named} names were drawn");
        assert!(crowded > 0, "no crowded ring was tried");
    }

    #[test]
    fn a_segment_is_wide_enough_for_its_name() {
        let k = sizes();
        let r = ring(5, 436.0, 1000.0, &k).unwrap();
        for p in &r.parts {
            assert!((p.end - p.start) * mid_r(&r) >= k.name_width + 2.0 * k.tab_pad);
            assert!(p.name_room >= k.name_width - 1e-9);
        }
    }

    #[test]
    fn the_label_on_pointing_shows_only_when_the_name_is_cut() {
        use crate::toolkit::input::Input;
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::{tip_rest, Ui};
        let r = ring(PROGRAMS.len(), 436.0, 1000.0, &sizes()).unwrap();
        let mut seen: Vec<(String, bool)> = Vec::new();
        for (i, program) in PROGRAMS.iter().enumerate() {
            let l = look(program);
            let b = r.button_of(i);
            let mut ui = Ui::new(tab_node(&r, i, &l).pressable(super::super::ctx::msg(|_| {})).width(crate::toolkit::widget::Length::Fixed(b[2] as f32)), Theme::new(false, ""));
            let mut pix = tiny_skia::Pixmap::new(b[2].ceil() as u32 + 80, b[3].ceil() as u32 + 80).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            let now = std::time::Instant::now();
            ui.input(&Input::PointerIn { x: (b[2] / 2.0) as f32, y: (b[3] / 2.0) as f32 }, now);
            ui.tick(now + tip_rest());
            ui.paint(&mut pix.as_mut(), 1.0);
            seen.push((program.to_string(), ui.label_over((b[2] / 2.0) as f32, (b[3] / 2.0) as f32).is_some()));
        }
        assert_eq!(seen.iter().map(|s| s.1).collect::<Vec<_>>(), [false, false, false, true], "{seen:?}");
    }

    #[test]
    fn the_name_is_one_level_line_under_the_icon() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let r = ring(12, 436.0, 1000.0, &sizes()).unwrap();
        for i in [0usize, 5, 11] {
            let (b, p) = (r.button_of(i), &r.parts[i]);
            let mut ui = Ui::new(tab_node(&r, i, &look("Brave")).width(crate::toolkit::widget::Length::Fixed(b[2] as f32)), Theme::new(false, ""));
            ui.see_through = true;
            let mut pix = tiny_skia::Pixmap::new(b[2].ceil() as u32, b[3].ceil() as u32).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            let from = (p.name_top - b[1]).floor() as u32;
            let (mut top, mut bottom) = (u32::MAX, 0);
            for y in from..pix.height() {
                if (0..pix.width()).any(|x| pix.pixel(x, y).unwrap().alpha() > 0) {
                    top = top.min(y);
                    bottom = bottom.max(y);
                }
            }
            assert!(top != u32::MAX, "tab {i}: no name was drawn");
            assert!(bottom - top <= name_height().ceil() as u32, "tab {i}: the name is {} tall", bottom - top);
        }
    }

}
