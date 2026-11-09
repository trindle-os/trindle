//! A notification as a circle, stacking from a corner of the screen. This holds the part that is the same
//! whatever shows them: where each circle goes for the chosen corner, the
//! message written in it, the round buttons that appear on pointing, how
//! far it grows on pointing to hold the whole message (and how the ones
//! beyond it slide along to make room), the snooze choices, how it is
//! painted, and what pinning and Clear all mean. Every size is a named value in the style sheet.

use crate::toolkit::colour::paint;
use crate::toolkit::text::{Align, Fonts, Layout, Style};
use crate::toolkit::theme;
use crate::toolkit::ui::rounded;
use crate::toolkit::widget::{Pen, Rect};

use super::notify;
use crate::toolkit::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    BottomRight,
    BottomLeft,
    TopLeft,
    TopRight,
}

impl Corner {

    pub fn from_setting(value: &str) -> Corner {
        match value {
            "bottom-left" => Corner::BottomLeft,
            "top-left" => Corner::TopLeft,
            "top-right" => Corner::TopRight,
            _ => Corner::BottomRight,
        }
    }

    pub fn chosen() -> Corner {
        Corner::from_setting(&notify::corner_setting())
    }

    pub fn right(self) -> bool {
        matches!(self, Corner::BottomRight | Corner::TopRight)
    }

    pub fn bottom(self) -> bool {
        matches!(self, Corner::BottomRight | Corner::BottomLeft)
    }
}

pub const MIN_TARGET: f32 = 24.0;

pub const EDGE_SHARE: f32 = 0.15;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes {

    pub across: f32,

    pub least: f32,
    pub gap: f32,
    pub margin: f32,

    pub button: f32,
}

impl Sizes {
    pub fn sheet() -> Sizes {
        let s = theme::sheet();
        Sizes {
            across: s.circle.round(),
            least: s.circle_rest_least.round().min(s.circle.round()),
            gap: s.circle_gap.round(),
            margin: s.circle_margin.round(),
            button: (s.ring_button * s.circle_button).round().max(MIN_TARGET),
        }
    }
}

pub fn places(corner: Corner, (w, h): (f32, f32), sizes: &[f32], k: Sizes) -> Vec<(f32, f32, f32)> {
    let shift = (h * EDGE_SHARE).round();
    let mut away = shift;
    let mut out = Vec::new();
    for d in sizes {
        let x = if corner.right() { w - k.margin - d } else { k.margin };
        let y = if corner.bottom() { h - k.margin - away - d } else { k.margin + away };
        if y < k.margin || y + d > h - k.margin {
            break;
        }
        out.push((x, y, *d));
        away += d + k.gap;
    }
    out
}

pub fn size_at(from: f32, to: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    from + (to - from) * (1.0 - (1.0 - t) * (1.0 - t))
}

pub const POINTED_LEAST: f32 = 144.0;

pub fn rest(fonts: &Fonts, k: Sizes, tall: f32, title: &str, body: &str) -> f32 {
    let most = (theme::sheet().circle_most * tall).round().max(k.across);
    let mut d = k.least.max(POINTED_LEAST).min(k.across);
    while d < most {
        let at = Sizes { across: d, ..k };
        if message(fonts, 1.0, at, REST_BUTTONS, false, title, body, 0).fits {
            return d;
        }
        d += 2.0;
    }
    most
}

pub fn grown(fonts: &Fonts, k: Sizes, rest: f32, buttons: usize, title: &str, body: &str, tall: f32) -> f32 {
    let from = rest.max(POINTED_LEAST.min(k.across));
    let most = (theme::sheet().circle_pointed_most * tall).round().max(from);
    let mut d = from;
    while d < most {
        if message(fonts, 1.0, Sizes { across: d, ..k }, buttons, true, title, body, 0).fits {
            return d;
        }
        d += 4.0;
    }
    most
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Note,
    Snooze,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snooze {
    Minutes10,
    Hour1,
    Hours6,

    Morning,
}

pub const SNOOZES: [Snooze; 4] = [Snooze::Minutes10, Snooze::Hour1, Snooze::Hours6, Snooze::Morning];

pub const MORNING_HOUR: i32 = 8;

impl Snooze {
    pub fn words(self) -> &'static str {
        match self {
            Snooze::Minutes10 => t!("ring-snooze-10-minutes"),
            Snooze::Hour1 => t!("ring-snooze-1-hour"),
            Snooze::Hours6 => t!("ring-snooze-6-hours"),
            Snooze::Morning => t!("ring-snooze-next-morning"),
        }
    }

    pub fn tip(self) -> &'static str {
        match self {
            Snooze::Minutes10 => t!("ring-snooze-for-10-minutes"),
            Snooze::Hour1 => t!("ring-snooze-for-1-hour"),
            Snooze::Hours6 => t!("ring-snooze-for-6-hours"),
            Snooze::Morning => t!("ring-snooze-for-next-morning"),
        }
    }

    pub fn seconds(self, hour: i32, minute: i32, second: i32) -> i64 {
        match self {
            Snooze::Minutes10 => 10 * 60,
            Snooze::Hour1 => 3600,
            Snooze::Hours6 => 6 * 3600,
            Snooze::Morning => {
                let now = (hour * 3600 + minute * 60 + second) as i64;
                let at = MORNING_HOUR as i64 * 3600;
                if now < at { at - now } else { 24 * 3600 - now + at }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {

    Open,
    Close,

    Pin,
    Snooze,

    ClearAll,

    Copy,

    Drive,

    Allow,
    Pick(Snooze),

    Back,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub act: Act,
    pub at: Rect,
    pub round: bool,
}

const ROW: f32 = MIN_TARGET;
const ROWS_FROM: f32 = -54.0;

const EDGE_ROOM: f32 = 2.0;
const BUTTON_GAP: f32 = 8.0;

pub fn edge_room() -> f32 {
    let s = theme::sheet();
    (s.focus_offset + s.focus_width).ceil().max(EDGE_ROOM)
}

const TEXT: f32 = 14.0;

pub const TITLE_LINES: usize = 3;
const ICON: f32 = 16.0;
const TEXT_PAD: f32 = 6.0;

fn arc(n: usize, k: Sizes) -> Vec<Rect> {
    let (r, b) = (k.across / 2.0, k.button);
    let on = r - b / 2.0 - edge_room();
    let step = 2.0 * ((b + BUTTON_GAP) / 2.0 / on).clamp(-1.0, 1.0).asin();
    (0..n).map(|i| {
        let a = (i as f32 - (n as f32 - 1.0) / 2.0) * step;
        Rect { x: r + on * a.sin() - b / 2.0, y: r + on * a.cos() - b / 2.0, w: b, h: b }
    }).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buttons {
    pub many: bool,
    pub extra: Option<Act>,
}

pub const REST_BUTTONS: usize = 4;

impl Buttons {
    pub fn list(self) -> Vec<Act> {
        let mut out = vec![Act::Close, Act::Pin, Act::Snooze, Act::Copy];
        out.extend(self.extra.filter(|a| matches!(a, Act::Drive | Act::Allow)));
        if self.many {
            out.push(Act::ClearAll);
        }
        out
    }

    pub fn count(self) -> usize {
        self.list().len()
    }
}

impl Act {

    pub fn name(self, pinned: bool, copied: bool) -> String {
        match self {
            Act::Open => t!("ring-open").into(),
            Act::Close => t!("ring-close").into(),
            Act::Back => t!("ring-act-back").into(),
            Act::Pin if pinned => t!("ring-act-unpin").into(),
            Act::Pin => t!("ring-act-pin").into(),
            Act::Snooze => t!("ring-act-snooze").into(),
            Act::ClearAll => t!("ring-act-clear-all").into(),
            Act::Copy if copied => t!("ring-act-copied").into(),
            Act::Copy => t!("ring-act-copy").into(),
            Act::Drive => t!("ring-open").into(),
            Act::Allow => crate::plugins::allow_word().into(),
            Act::Pick(s) => s.tip().into(),
        }
    }
}

pub fn text_box(k: Sizes, buttons: usize) -> Rect {
    let (d, r) = (k.across, k.across / 2.0);
    let top = icon_at(k) + ICON + 4.0;
    let foot = arc(buttons.max(REST_BUTTONS), k).iter().map(|b| b.y).fold(f32::MAX, f32::min) - 6.0;
    let far = (top - r).abs().max((foot - r).abs());
    let w = 2.0 * (r * r - far * far).max(0.0).sqrt() - 2.0 * TEXT_PAD;
    Rect { x: (d - w) / 2.0, y: top, w, h: foot - top }
}

fn icon_at(k: Sizes) -> f32 {
    (k.across * 0.11).round()
}

pub struct Message {
    pub title: Layout,
    pub body: Layout,
    pub fits: bool,

    pub lines: usize,
    pub room: usize,

    pub drop: f32,

    pub area: Rect,
    pub icon_y: f32,
}

pub fn words(pointed: bool) -> f32 {
    if pointed { theme::sheet().text.0 } else { TEXT }
}

fn style(s: f32, size: f32, heavy: bool) -> Style {
    let mut st = Style::new((size * s).round());
    st.align = Align::Centre;
    if heavy {
        st.weight = theme::sheet().note_title_weight;
    }
    st
}

pub fn message(fonts: &Fonts, s: f32, k: Sizes, buttons: usize, pointed: bool, title: &str, body: &str, scroll: usize) -> Message {
    let size = words(pointed);
    let area = text_box(k, buttons);
    let (icon_y, high) = (icon_at(k), area.h * s);
    let room = (area.w * s).floor();
    let head = style(s, size, true);
    let plain = style(s, size, false);
    let dots = fonts.layout("\u{2026}", &plain, None);
    let cap = TITLE_LINES;
    let whole = fonts.layout(title, &head, Some(room));
    let title_fits = whole.lines.len() <= cap;
    let shown = if title_fits {
        whole
    } else {
        whole.cut_after(cap, room, &fonts.layout("\u{2026}", &head, None))
    };
    let line = fonts.layout("Ag", &plain, None).height.max(1.0);
    let lines = ((high - shown.height) / line).floor().max(0.0) as usize;
    let all = fonts.layout(body, &plain, Some(room));
    let total = all.lines.len();
    let body_fits = body.is_empty() || total <= lines;
    let from = scroll.min(total.saturating_sub(1));
    let rest = if from > 0 { fonts.layout(&body[all.lines[from].range.start..], &plain, Some(room)) } else { all };
    let mut shown_body = if rest.lines.len() <= lines { rest } else { rest.cut_after(lines, room, &dots) };

    if shown_body.cut {
        if let Some(last) = shown_body.lines.last_mut() {
            let shift = ((room - last.width) / 2.0).max(0.0) - last.x;
            last.x += shift;
            for g in &mut last.glyphs {
                g.x += shift;
            }
        }
    }
    let drop = ((high - shown.height - shown_body.height) / 2.0).max(0.0).floor();
    Message { title: shown, body: shown_body, fits: title_fits && body_fits, lines: total, room: lines, drop, area, icon_y }
}

pub fn most_scroll(fonts: &Fonts, k: Sizes, buttons: usize, title: &str, body: &str) -> usize {
    let m = message(fonts, 1.0, k, buttons, true, title, body, 0);
    m.lines.saturating_sub(m.room)
}

pub fn parts(face: Face, buttons: Buttons, k: Sizes) -> Vec<Part> {
    let (d, r, b) = (k.across, k.across / 2.0, k.button);
    let mut out = Vec::new();
    match face {
        Face::Note => {
            let list = buttons.list();
            for (act, at) in list.iter().zip(arc(list.len(), k)) {
                out.push(Part { act: *act, at, round: true });
            }

            out.push(Part { act: Act::Open, at: Rect { x: 0.0, y: 0.0, w: d, h: d }, round: false });
        }
        Face::Snooze => {
            for (i, s) in SNOOZES.iter().enumerate() {
                let mid = r + ROWS_FROM + (i as f32 + 0.5) * ROW;

                let far = (mid + ROW / 2.0 - r).abs().max((mid - ROW / 2.0 - r).abs());
                let half = (r * r - far * far).max(0.0).sqrt() - edge_room();
                out.push(Part { act: Act::Pick(*s), at: Rect { x: r - half, y: mid - ROW / 2.0, w: 2.0 * half, h: ROW }, round: false });
            }
            out.push(Part { act: Act::Back, at: Rect { x: r - b / 2.0, y: r + (r - b / 2.0 - edge_room()) - b / 2.0, w: b, h: b }, round: true });
        }
    }
    out
}

pub fn pressed(face: Face, buttons: Buttons, k: Sizes, x: f32, y: f32) -> Option<Act> {
    let r = k.across / 2.0;
    if (x - r).hypot(y - r) > r {
        return None;
    }
    let all = parts(face, buttons, k);

    all.iter().filter(|p| p.act != Act::Open).chain(all.iter().filter(|p| p.act == Act::Open))
        .find(|p| p.at.contains(x, y)).map(|p| p.act)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Held {
    pub id: u32,
    pub pinned: bool,

    pub moved: bool,
}

pub fn cleared(all: &[Held]) -> Vec<u32> {
    all.iter().filter(|h| !h.pinned && !h.moved).map(|h| h.id).collect()
}

pub struct Spec<'a> {

    pub across: f32,

    pub scroll: usize,
    pub icon: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub face: Face,
    pub buttons: Buttons,

    pub hover: bool,

    pub copied: bool,
    pub pinned: bool,

    pub hot: Option<Act>,

    pub moving: bool,
}

const MOVING_EDGE: f32 = 3.0;

pub fn shown_buttons(buttons: Buttons) -> usize {
    buttons.count().max(REST_BUTTONS)
}

fn button_icon(act: Act) -> &'static str {
    match act {
        Act::Close | Act::Back => "window-close-symbolic",
        Act::Pin => "view-pin-symbolic",
        Act::Snooze => "alarm-symbolic",
        Act::Copy => "edit-copy-symbolic",
        Act::Drive => "folder-open-symbolic",
        Act::Allow => crate::plugins::ALLOW_ICON,
        Act::ClearAll => "edit-clear-all-symbolic",

        Act::Open | Act::Pick(_) => "edit-clear-all-symbolic",
    }
}

fn fill(canvas: &mut tiny_skia::PixmapMut, r: Rect, c: crate::toolkit::colour::Colour) {
    fill_with(canvas, r, c, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Button, r.w.min(r.h), 1.0));
}

fn fill_with(canvas: &mut tiny_skia::PixmapMut, r: Rect, c: crate::toolkit::colour::Colour, corner: f32) {
    if let Some(path) = rounded(r, corner) {
        let mut p = paint(c);
        p.anti_alias = true;
        canvas.fill_path(&path, &p, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
    }
}

pub fn paint_circle(canvas: &mut tiny_skia::PixmapMut, pen: &Pen, (x, y): (f32, f32), spec: &Spec) {
    let (s, t) = (pen.scale, pen.theme);
    let k = Sizes { across: spec.across, ..Sizes::sheet() };
    let whole = Rect { x, y, w: k.across * s, h: k.across * s };

    let edge = if spec.moving { t.accent } else { t.card_edge };
    let rim = if spec.moving { MOVING_EDGE * s } else { s };

    fill_with(canvas, whole, edge, whole.w / 2.0);
    let inner = Rect { x: x + rim, y: y + rim, w: whole.w - 2.0 * rim, h: whole.h - 2.0 * rim };
    fill_with(canvas, inner, t.raised, inner.w / 2.0);
    let place = |r: Rect| Rect { x: x + r.x * s, y: y + r.y * s, w: r.w * s, h: r.h * s };
    if spec.face == Face::Note {
        let icon = (ICON * s).round() as u32;
        let pin = spec.pinned && !spec.hover;
        let n = shown_buttons(spec.buttons);
        let m = message(pen.fonts, s, k, n, spec.hover, spec.title, spec.body, spec.scroll);
        let (cx, iy) = (x + (whole.w - icon as f32) / 2.0 - if pin { (ICON * 0.75 * s).round() } else { 0.0 }, y + m.icon_y * s);
        if pen.icons.draw(canvas, spec.icon, cx.round() as i32, iy.round() as i32, icon, 1, t.fg).is_none() {
            pen.icons.draw(canvas, "preferences-system-notifications-symbolic", cx.round() as i32, iy.round() as i32, icon, 1, t.fg);
        }
        if spec.pinned {
            pen.icons.draw(canvas, "view-pin-symbolic", (cx + (ICON * 1.5 * s).round()).round() as i32, iy.round() as i32, icon, 1, t.fg);
        }
        let b = m.area;
        pen.fonts.draw(canvas, &m.title, x + b.x * s, y + b.y * s + m.drop, t.fg);
        pen.fonts.draw(canvas, &m.body, x + b.x * s, y + b.y * s + m.drop + m.title.height, t.muted);
    }
    for p in parts(spec.face, spec.buttons, k) {
        match p.act {
            Act::Open => {}
            Act::Pick(choice) => {
                let r = place(p.at);
                if spec.hot == Some(p.act) {
                    fill(canvas, Rect { x: r.x + 2.0 * s, y: r.y + 2.0 * s, w: r.w - 4.0 * s, h: r.h - 4.0 * s }, t.hover);
                }
                let mut st = style(s, TEXT, false);
                st.weight = theme::sheet().note_title_weight;
                let layout = pen.fonts.layout(choice.words(), &st, None);
                pen.fonts.draw(canvas, &layout, r.x + ((r.w - layout.width) / 2.0).round(), r.y + ((r.h - layout.height) / 2.0).round(), t.fg);
            }
            act => {
                let r = place(p.at);
                let on = (act == Act::Pin && spec.pinned) || (act == Act::Copy && spec.copied);
                fill(canvas, r, if on { t.picked } else if spec.hot == Some(act) { t.hover } else { t.control });
                let side = (r.w * 0.58).round() as u32;
                let icon = button_icon(act);
                pen.icons.draw(canvas, icon, (r.x + (r.w - side as f32) / 2.0).round() as i32, (r.y + (r.h - side as f32) / 2.0).round() as i32,
                               side, 1, if on { t.on_picked } else { t.fg });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;

    fn kit() -> Sizes {
        Sizes { across: 160.0, least: 96.0, gap: 12.0, margin: 24.0, button: 24.0 }
    }

    const SHORT: (&str, &str) = ("Copy finished", "3 files copied.");
    const LONG: (&str, &str) = ("Mail from John Smith", "Lunch on Thursday? I have booked the table for one o'clock, and Jane Brown is bringing the folder with the figures from the spring, and her sister is coming too, so please say if you can still make it by Wednesday evening.");

    fn endless() -> String {
        (1..=60).map(|i| format!("Line {i} of a very long message.")).collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn circles_stack_away_from_the_chosen_corner_and_never_leave_the_screen() {
        let k = kit();
        for corner in [Corner::BottomRight, Corner::BottomLeft, Corner::TopLeft, Corner::TopRight] {
            for screen in [(1280.0f32, 720.0f32), (1920.0, 1080.0), (800.0, 600.0)] {
                let all: Vec<(f32, f32)> = places(corner, screen, &[k.across; 40], k).into_iter().map(|(x, y, _)| (x, y)).collect();
                assert!(!all.is_empty());
                for (i, (x, y)) in all.iter().enumerate() {
                    assert!(*x >= k.margin && x + k.across <= screen.0 - k.margin, "{corner:?} {i} is not clear of the sides of a {screen:?} screen");
                    let shift = (screen.1 * EDGE_SHARE).round();
                    assert!(*y >= k.margin && y + k.across <= screen.1 - k.margin, "{corner:?} {i} is off a {screen:?} screen");
                    assert!(if corner.bottom() { screen.1 - y - k.across >= k.margin + shift } else { *y >= k.margin + shift }, "{corner:?} {i} is not a share of the height in");
                    if i > 0 {
                        let (px, py) = all[i - 1];
                        assert_eq!(*x, px, "they line up the side of the screen");
                        let step = if corner.bottom() { py - y } else { y - py };
                        assert_eq!(step, k.across + k.gap, "{corner:?}: one daylight apart, away from the corner");
                    }
                }
                let (x, y) = all[0];
                assert_eq!(if corner.right() { screen.0 - x - k.across } else { x }, k.margin);
                let shift = (screen.1 * EDGE_SHARE).round();
                assert_eq!(if corner.bottom() { screen.1 - y - k.across } else { y }, k.margin + shift);
                assert_eq!(all.len(), ((screen.1 - 2.0 * k.margin - shift + k.gap) / (k.across + k.gap)).floor() as usize);
            }
        }
    }

    #[test]
    fn a_grown_circle_grows_towards_the_middle_and_its_neighbours_slide_along() {
        let k = kit();
        for corner in [Corner::BottomRight, Corner::BottomLeft, Corner::TopLeft, Corner::TopRight] {
            let screen = (1280.0f32, 800.0f32);
            let calm = places(corner, screen, &[160.0; 4], k);
            for grown in 0..4 {
                let mut sizes = [160.0f32; 4];
                sizes[grown] = 260.0;
                let all = places(corner, screen, &sizes, k);
                for (i, (x, y, d)) in all.iter().enumerate() {
                    assert!(*x >= k.margin - 0.01 && x + d <= screen.0 - k.margin + 0.01 && *y >= 0.0 && y + d <= screen.1, "{corner:?} {i}: off the screen");

                    assert_eq!(if corner.right() { x + d } else { *x }, if corner.right() { screen.0 - k.margin } else { k.margin });

                    if i < grown {
                        assert_eq!((*x, *y), (calm[i].0, calm[i].1));
                    }
                    if i == grown {
                        assert_eq!(if corner.bottom() { y + d } else { *y }, if corner.bottom() { calm[i].1 + 160.0 } else { calm[i].1 });
                    }
                    if i > 0 {
                        let (_, py, pd) = all[i - 1];
                        let apart = if corner.bottom() { py - (y + d) } else { y - (py + pd) };
                        assert!((apart - k.gap).abs() < 0.01, "{corner:?} {i}: not a gap from the one before: {apart}");
                    }
                }
            }
        }

        let all = places(Corner::BottomRight, (1280.0, 700.0), &[300.0; 4], k);
        assert!(all.len() < 4 && all.iter().all(|(_, y, d)| *y >= 0.0 && y + d <= 700.0));
    }

    #[test]
    fn it_grows_smoothly_from_its_size_to_the_one_that_holds_the_message() {
        assert_eq!(size_at(160.0, 240.0, 0.0), 160.0);
        assert_eq!(size_at(160.0, 240.0, 1.0), 240.0);
        let steps: Vec<f32> = (0..=10).map(|i| size_at(160.0, 240.0, i as f32 / 10.0)).collect();
        assert!(steps.windows(2).all(|w| w[1] >= w[0] && w[1] - w[0] <= 24.0), "{steps:?}");

        assert_eq!(size_at(240.0, 160.0, 1.0), 160.0);
    }

    #[test]
    fn a_line_cut_short_stays_centred() {
        let (fonts, k) = (Fonts::new(), Sizes::sheet());
        let m = message(&fonts, 1.0, k, 0, false, LONG.0, LONG.1, 0);
        assert!(m.body.cut);
        let (room, last) = (text_box(k, 0).w.floor(), m.body.lines.last().unwrap());
        assert!((last.x + last.width / 2.0 - room / 2.0).abs() <= 1.0, "{} {} {room}", last.x, last.width);
    }

    fn grown_for(fonts: &Fonts, k: Sizes, buttons: usize, title: &str, body: &str, tall: f32) -> f32 {
        grown(fonts, k, rest(fonts, k, tall, title, body), buttons, title, body, tall)
    }

    #[test]
    fn the_circle_grows_just_enough_and_no_more_than_the_most() {
        let (fonts, k, tall) = (Fonts::new(), Sizes::sheet(), 800.0f32);
        let most = (theme::sheet().circle_pointed_most * tall).round();
        assert!(most > k.across, "the most is bigger than the usual circle");
        assert!((POINTED_LEAST..=k.across).contains(&grown_for(&fonts, k, REST_BUTTONS, SHORT.0, SHORT.1, tall)));
        assert_eq!(grown_for(&fonts, k, REST_BUTTONS, "Done", "", tall), POINTED_LEAST);
        let d = grown_for(&fonts, k, REST_BUTTONS, LONG.0, LONG.1, tall);
        assert!(d > k.across && d <= most, "{d}");
        assert!(message(&fonts, 1.0, Sizes { across: d, ..k }, REST_BUTTONS, true, LONG.0, LONG.1, 0).fits, "holds the whole of it");
        assert!(!message(&fonts, 1.0, Sizes { across: d - 4.0, ..k }, REST_BUTTONS, true, LONG.0, LONG.1, 0).fits, "and just enough");

        assert!(grown_for(&fonts, k, REST_BUTTONS, LONG.0, LONG.1, 400.0) <= (theme::sheet().circle_pointed_most * 400.0).round().max(k.across));

        let long = endless();
        let at_most = grown_for(&fonts, k, REST_BUTTONS, "Log", &long, tall);
        assert_eq!(at_most, most);
        let big = Sizes { across: at_most, ..k };
        let m = message(&fonts, 1.0, big, REST_BUTTONS, true, "Log", &long, 0);
        assert!(!m.fits && m.body.cut);
        let scrolled = message(&fonts, 1.0, big, REST_BUTTONS, true, "Log", &long, 5);
        assert!(scrolled.body.lines[0].range.start > 0 || scrolled.body.cut, "the words have moved up");
        assert!(most_scroll(&fonts, big, REST_BUTTONS, "Log", &long) > 5);
        assert_eq!(most_scroll(&fonts, big, REST_BUTTONS, SHORT.0, SHORT.1), 0);

        let rest = message(&fonts, 1.0, k, REST_BUTTONS, false, LONG.0, LONG.1, 0);
        assert!(rest.body.cut && rest.body.lines.len() >= 2, "most of the message is written in the circle");
    }

    #[test]
    fn at_rest_a_circle_is_the_smallest_that_holds_its_words_above_its_buttons() {
        let (fonts, k) = (Fonts::new(), Sizes::sheet());
        let medium = ("Mail from Jane", "Lunch is at one on Thursday.");
        let all = [("Done", ""), SHORT, medium, LONG];
        let sizes: Vec<f32> = all.iter().map(|m| rest(&fonts, k, 800.0, m.0, m.1)).collect();
        assert_eq!(sizes[0], POINTED_LEAST, "a one-word message is the least: {sizes:?}");
        assert!(sizes[0] <= sizes[2] && sizes[2] <= sizes[3], "{sizes:?}");
        assert!(sizes.iter().all(|d| *d >= POINTED_LEAST && *d <= (theme::sheet().circle_most * 800.0).round()), "{sizes:?}");
        let n = shown_buttons(Buttons { many: false, extra: None });
        for (title, body) in all {
            let d = rest(&fonts, k, 800.0, title, body);
            let m = message(&fonts, 1.0, Sizes { across: d, ..k }, n, false, title, body, 0);
            assert!(m.fits, "{title}: whole message holds at {d}");
            assert_eq!(m.icon_y, icon_at(Sizes { across: d, ..k }));
            if d > POINTED_LEAST.max(k.least) {
                let less = message(&fonts, 1.0, Sizes { across: d - 2.0, ..k }, n, false, title, body, 0);
                assert!(!less.fits, "{title}: smaller would do");
            }
            for g in m.title.lines.iter().chain(m.body.lines.iter()) {
                assert!(g.width <= m.area.w.floor() + 1.0);
            }

            let up = grown(&fonts, k, d, n, title, body, 800.0);
            assert!(up >= d, "{title}: {up}");
            assert!(message(&fonts, 1.0, Sizes { across: up, ..k }, n, true, title, body, 0).fits, "{title}: whole at {up}");
        }
    }

    #[test]
    fn at_rest_every_kind_of_notification_shows_the_whole_of_a_long_message() {
        let (fonts, k) = (Fonts::new(), Sizes::sheet());
        let long = "Please look at the backup of John Smith's folders, which stopped at the third of five disks because the drive was removed; put it back in and press Open to carry on, or press Allow to let the plugin try again later today.";
        let kinds = [
            ("A program", "Mail from John Smith", long),
            ("A crash", "Trindle Folders stopped", long),
            ("A drive", "Backup disk 2 was added and is ready to open now", long),
            ("A plugin", "Weather wants to use the internet and your folders", long),
        ];
        let cap = (theme::sheet().circle_most * 800.0).round();
        for (kind, title, body) in kinds {
            let d = rest(&fonts, k, 800.0, title, body);
            assert!(d <= cap, "{kind}: {d} over the cap");
            let m = message(&fonts, 1.0, Sizes { across: d, ..k }, REST_BUTTONS, false, title, body, 0);
            assert!(m.fits && !m.title.cut && !m.body.cut, "{kind}: cut at rest at {d}");
            let pointed = grown(&fonts, k, d, shown_buttons(Buttons { many: true, extra: None }), title, body, 800.0);
            assert!(pointed >= d, "{kind}: pointing shrank it");
        }
    }

    #[test]
    fn at_rest_every_face_draws_its_buttons_clear_of_the_words() {
        let (fonts, k) = (Fonts::new(), Sizes::sheet());
        for (title, body) in [("Done", ""), SHORT, FAULT] {
            let d = rest(&fonts, k, 800.0, title, body);
            assert!(d >= POINTED_LEAST, "{title}: {d} leaves no room for 24 point buttons");
            let at = Sizes { across: d, ..k };
            for face in [Face::Note, Face::Snooze] {
                for b in EVERY {
                    let all = parts(face, b, at);
                    let shown: Vec<&Part> = all.iter().filter(|p| p.act != Act::Open).collect();
                    assert!(!shown.is_empty(), "{face:?}: no buttons at rest");
                    assert!(shown.iter().all(|p| p.at.w >= MIN_TARGET && p.at.h >= MIN_TARGET));
                    if face == Face::Note {
                        assert_eq!(shown.len(), b.count(), "every button is there at rest");
                        let m = message(&fonts, 1.0, at, shown_buttons(b), false, title, body, 0);
                        let top = shown.iter().map(|p| p.at.y).fold(f32::MAX, f32::min);
                        assert!(m.area.y + m.area.h <= top, "{title}: the words reach the buttons");
                    }
                }
            }
        }
    }

    const FAULT: (&str, &str) = ("Holiday Photo Manager", "In Gallery, attach holiday.jpg from the Summer-photos-2026-10-20-095744 folder to each draft, confirm the captions, then schedule all three for Thursday morning.");

    #[test]
    fn a_pointed_circle_writes_whole_titles_in_regular_size_words_in_the_middle_of_its_room() {
        let (fonts, k) = (Fonts::new(), Sizes::sheet());
        let regular = theme::sheet().text.0;
        assert_eq!(words(true), regular, "the sheet's ordinary words");
        for tall in [768.0f32, 1080.0, 1440.0, 2000.0] {
            let n = shown_buttons(Buttons { many: true, extra: None });
            let d = grown_for(&fonts, k, n, FAULT.0, FAULT.1, tall);
            let big = Sizes { across: d, ..k };
            let m = message(&fonts, 1.0, big, n, true, FAULT.0, FAULT.1, 0);
            assert!(m.fits && !m.title.cut && !m.body.cut, "{tall}: all of it written at {d} with lines {} {}", m.title.lines.len(), m.body.lines.len());
            assert!((1..=TITLE_LINES).contains(&m.title.lines.len()), "{tall}: the title takes as many lines as it needs, at most {TITLE_LINES}: {}", m.title.lines.len());
            assert!(d <= (theme::sheet().circle_pointed_most * tall).round(), "{tall}: {d} fits the screen");
            let room = text_box(big, n);
            let left = room.h - m.title.height - m.body.height;
            assert!(left >= 0.0 && (m.drop - left / 2.0).abs() <= 1.0, "centred: {} of {left}", m.drop);
            let words_high = m.title.height + m.body.height;
            assert!(room.h >= words_high && m.drop + words_high <= room.h, "inside the room");

            assert!(!message(&fonts, 1.0, Sizes { across: d - 4.0, ..k }, n, true, FAULT.0, FAULT.1, 0).fits || d == k.across || d == POINTED_LEAST);
            for g in m.title.lines.iter().chain(m.body.lines.iter()) {
                assert!(g.width <= room.w.floor() + 1.0);
            }
        }

        let rest = message(&fonts, 1.0, k, 0, false, FAULT.0, FAULT.1, 0);
        assert_eq!(words(false), TEXT);
        assert!(rest.title.lines.len() <= TITLE_LINES);
    }

    #[test]
    fn a_button_is_two_thirds_of_the_ring_centres_and_never_under_24() {
        let k = Sizes::sheet();
        let ring = theme::sheet().ring_button;
        assert!(k.button >= MIN_TARGET && (k.button == MIN_TARGET || (k.button - ring * 2.0 / 3.0).abs() <= 1.0), "{} against {ring}", k.button);
        assert!(k.across >= 144.0, "the circle holds its 24 point targets");
    }

    const EVERY: [Buttons; 6] = [
        Buttons { many: false, extra: None },
        Buttons { many: true, extra: None },
        Buttons { many: false, extra: Some(Act::Drive) },
        Buttons { many: true, extra: Some(Act::Drive) },
        Buttons { many: false, extra: Some(Act::Allow) },
        Buttons { many: true, extra: Some(Act::Allow) },
    ];

    #[test]
    fn every_part_stays_inside_the_circle_apart_and_big_enough() {
        for k in [kit(), Sizes::sheet(), Sizes { across: 176.0, least: 96.0, gap: 4.0, margin: 16.0, button: 28.0 }, Sizes { across: 144.0, least: 96.0, gap: 4.0, margin: 16.0, button: 24.0 }, Sizes { across: 260.0, least: 96.0, gap: 4.0, margin: 16.0, button: 24.0 }] {
            let r = k.across / 2.0;
            for face in [Face::Note, Face::Snooze] {
                for b in EVERY {
                    {
                        let all = parts(face, b, k);
                        let has = |a: Act| all.iter().any(|p| p.act == a);
                        let buttons = face == Face::Note;
                        assert_eq!((has(Act::Close), has(Act::Pin), has(Act::Snooze), has(Act::Copy)), (buttons, buttons, buttons, buttons));
                        assert_eq!(has(Act::ClearAll), buttons && b.many);
                        assert_eq!((has(Act::Drive), has(Act::Allow)), (buttons && b.extra == Some(Act::Drive), buttons && b.extra == Some(Act::Allow)));
                        assert_eq!(has(Act::Back), face == Face::Snooze);
                        let shown: Vec<&Part> = all.iter().filter(|p| p.act != Act::Open).collect();
                        for p in &shown {
                            let a = p.at;
                            assert!(a.w >= MIN_TARGET && a.h >= MIN_TARGET, "{:?} is under 24 points: {a:?}", p.act);
                            let far = if p.round {
                                (a.x + a.w / 2.0 - r).hypot(a.y + a.h / 2.0 - r) + a.w / 2.0
                            } else {
                                [(a.x, a.y), (a.x + a.w, a.y), (a.x, a.y + a.h), (a.x + a.w, a.y + a.h)].iter().map(|(cx, cy)| (cx - r).hypot(cy - r)).fold(0.0, f32::max)
                            };
                            assert!(far <= r + 0.01, "{:?} sticks out of a {} circle: {a:?}", p.act, k.across);
                            assert!(!p.round || far + edge_room() <= r + 0.01, "{:?}: the keyboard's ring does not fit in a {} circle", p.act, k.across);
                        }
                        for (i, p) in shown.iter().enumerate() {
                            for q in &shown[i + 1..] {
                                let (a, b) = (p.at, q.at);
                                let apart = if p.round && q.round {
                                    (a.x + a.w / 2.0 - b.x - b.w / 2.0).hypot(a.y + a.h / 2.0 - b.y - b.h / 2.0) >= a.w / 2.0 + b.w / 2.0 + BUTTON_GAP - 0.01
                                } else {
                                    a.x + a.w <= b.x + 0.01 || b.x + b.w <= a.x + 0.01 || a.y + a.h <= b.y + 0.01 || b.y + b.h <= a.y + 0.01
                                };
                                assert!(apart, "{:?} is too near {:?}", p.act, q.act);
                            }
                        }

                        if buttons {
                            let words = text_box(k, b.count());
                            for p in &shown {
                                assert!(words.y + words.h <= p.at.y, "{:?} is over the message", p.act);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn a_press_finds_its_part_and_the_corners_of_the_square_are_nothing() {
        let k = kit();
        let alone = Buttons { many: false, extra: None };
        assert_eq!(pressed(Face::Note, Buttons { many: true, ..alone }, k, 1.0, 1.0), None);
        assert_eq!(pressed(Face::Note, alone, k, 80.0, 60.0), Some(Act::Open));
        for face in [Face::Note, Face::Snooze] {
            for b in EVERY {
                for p in parts(face, b, k).iter().filter(|p| p.act != Act::Open) {
                    let (x, y) = (p.at.x + p.at.w / 2.0, p.at.y + p.at.h / 2.0);
                    assert_eq!(pressed(face, b, k, x, y), Some(p.act));
                }
            }
        }
    }

    #[test]
    fn clear_all_leaves_pinned_and_moved_circles() {
        let all = [Held { id: 1, pinned: false, moved: false }, Held { id: 2, pinned: true, moved: false }, Held { id: 3, pinned: false, moved: true }, Held { id: 4, pinned: false, moved: false }];
        assert_eq!(cleared(&all), vec![1, 4]);
    }

    #[test]
    fn a_snooze_comes_back_when_it_says_and_the_morning_is_eight() {
        assert_eq!(Snooze::Minutes10.seconds(3, 0, 0), 600);
        assert_eq!(Snooze::Hour1.seconds(3, 0, 0), 3600);
        assert_eq!(Snooze::Hours6.seconds(23, 0, 0), 21600);
        assert_eq!(Snooze::Morning.seconds(6, 30, 0), 90 * 60);
        assert_eq!(Snooze::Morning.seconds(8, 0, 0), 24 * 3600);
        assert_eq!(Snooze::Morning.seconds(22, 0, 0), 10 * 3600);
        assert_eq!(SNOOZES.map(Snooze::words), ["10 minutes", "1 hour", "6 hours", "Next morning"]);
    }

    #[test]
    fn the_corner_setting_round_trips_and_defaults_to_bottom_right() {
        for ((value, _), corner) in notify::corner_choices().iter().zip([Corner::BottomRight, Corner::BottomLeft, Corner::TopLeft, Corner::TopRight]) {
            assert_eq!(Corner::from_setting(value), corner);
        }
        assert_eq!(Corner::from_setting("somewhere else"), Corner::BottomRight);
        assert_eq!(notify::corner_choices()[0].0, "bottom-right");
    }

    #[test]
    fn a_circle_is_round_whatever_the_sheets_corners() {
        use crate::toolkit::icon::Icons;
        let (fonts, icons) = (Fonts::new(), Icons::new());
        let theme = Theme::new(false, "#fbc711");
        let k = Sizes::sheet();
        assert!(crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Button, k.across, 1.0) < k.across / 4.0, "the default sheet has small corners, so this test means something");
        let pen = Pen { theme: &theme, fonts: &fonts, icons: &icons, scale: 1.0 };
        let side = k.across as u32;
        let mut pix = tiny_skia::Pixmap::new(side, side).unwrap();
        paint_circle(&mut pix.as_mut(), &pen, (0.0, 0.0), &Spec {
            across: k.across, scroll: 0, icon: "mail-unread-symbolic", title: "", body: "", face: Face::Note,
            buttons: Buttons { many: false, extra: None }, hover: false, pinned: false, hot: None, copied: false, moving: false,
        });
        let colour = |x: u32, y: u32| { let p = pix.pixel(x, y).unwrap().demultiply(); (p.red(), p.green(), p.blue(), p.alpha()) };
        let c = |c: crate::toolkit::colour::Colour| { let u = c.to_skia().to_color_u8(); (u.red(), u.green(), u.blue(), 255) };
        let (raised, edge) = (c(theme.raised), c(theme.card_edge));
        let near = side / 10;
        for (x, y) in [(near, near), (side - 1 - near, near), (near, side - 1 - near), (side - 1 - near, side - 1 - near)] {
            let got = colour(x, y);
            assert!(got != raised && got != edge, "the corner at {x},{y} is painted as the circle: {got:?}");
        }
        assert_eq!(colour(side / 2, side / 2), raised, "its middle is");
    }

    #[test]
    fn nothing_is_painted_outside_the_circle() {
        use crate::toolkit::icon::Icons;
        let (fonts, icons) = (Fonts::new(), Icons::new());
        let theme = Theme::new(false, "#fbc711");
        let k = Sizes::sheet();
        let pen = Pen { theme: &theme, fonts: &fonts, icons: &icons, scale: 1.0 };
        let side = k.across as u32;
        let mut pix = tiny_skia::Pixmap::new(side, side).unwrap();
        paint_circle(&mut pix.as_mut(), &pen, (0.0, 0.0), &Spec {
            across: k.across, scroll: 0, icon: "mail-unread-symbolic", title: "", body: "", face: Face::Note,
            buttons: Buttons { many: false, extra: None }, hover: false, pinned: false, hot: None, copied: false, moving: false,
        });
        let near = side / 10;
        for (x, y) in [(near, near), (side - 1 - near, near), (near, side - 1 - near), (side - 1 - near, side - 1 - near)] {
            assert_eq!(pix.pixel(x, y).unwrap().alpha(), 0, "the corner at {x},{y} is painted");
        }
    }

    #[test]
    fn a_held_circle_has_an_accent_edge_and_a_resting_one_does_not() {
        use crate::toolkit::icon::Icons;
        let (fonts, icons) = (Fonts::new(), Icons::new());
        let theme = Theme::new(false, "#fbc711");
        let k = Sizes::sheet();
        let pen = Pen { theme: &theme, fonts: &fonts, icons: &icons, scale: 1.0 };
        let edge = |moving: bool| {
            let side = k.across as u32;
            let mut pix = tiny_skia::Pixmap::new(side, side).unwrap();
            paint_circle(&mut pix.as_mut(), &pen, (0.0, 0.0), &Spec {
                across: k.across, scroll: 0, icon: "mail-unread-symbolic", title: "", body: "", face: Face::Note,
                buttons: Buttons { many: false, extra: None }, hover: false, pinned: true, hot: None, copied: false, moving,
            });
            let p = pix.pixel(side / 2, 1).unwrap();
            (p.red(), p.green(), p.blue())
        };
        let want = theme.accent.to_skia().to_color_u8();
        let held = edge(true);
        assert_eq!(held, (want.red(), want.green(), want.blue()), "the edge is the accent");
        assert_ne!(edge(false), held);
    }

    #[test]
    fn tight_sizes_never_make_a_nan_button_place_and_copy_keeps_its_icon() {
        let tight = Sizes { across: 40.0, least: 30.0, gap: 4.0, margin: 4.0, button: 30.0 };
        for r in arc(3, tight) {
            assert!(r.x.is_finite() && r.y.is_finite(), "{r:?}");
        }
        assert_eq!(button_icon(Act::Copy), "edit-copy-symbolic");
    }

    #[test]
    fn icons_in_a_circle_grow_with_the_scale_and_no_more() {
        use crate::toolkit::icon::Icons;
        let (fonts, icons) = (Fonts::new(), Icons::new());
        let theme = Theme::new(false, "#fbc711");
        let k = Sizes::sheet();
        let ink = |scale: f32, face: Face, hover: bool| -> (usize, usize) {
            let side = (k.across * scale).ceil() as u32;
            let mut pix = tiny_skia::Pixmap::new(side, side).unwrap();
            let pen = Pen { theme: &theme, fonts: &fonts, icons: &icons, scale };
            paint_circle(&mut pix.as_mut(), &pen, (0.0, 0.0), &Spec {
                across: k.across, scroll: 0, icon: "mail-unread-symbolic", title: "", body: "", face, buttons: Buttons { many: false, extra: None },
                hover, pinned: false, hot: None, copied: false, moving: false,
            });

            let at = message(&fonts, scale, k, shown_buttons(Buttons { many: false, extra: None }), hover, "", "", 0).icon_y;
            let (top, foot) = ((at * scale) as u32, ((at + ICON) * scale) as u32);
            let mut count = 0;
            let (mut left, mut right) = (side as usize, 0usize);
            for y in top..foot {
                for x in 0..side {
                    let p = pix.pixel(x, y).unwrap();
                    if (i32::from(p.red()) - i32::from(theme.raised.to_skia().to_color_u8().red())).abs() > 80 {
                        count += 1;
                        left = left.min(x as usize);
                        right = right.max(x as usize);
                    }
                }
            }
            (count, right.saturating_sub(left))
        };
        let (one, two) = (ink(1.0, Face::Note, false), ink(2.0, Face::Note, false));
        assert!(one.0 > 0 && one.1 > 0, "the program's icon is drawn");
        let (wide, area) = (two.1 as f32 / one.1 as f32, two.0 as f32 / one.0 as f32);
        assert!((1.7..=2.3).contains(&wide), "twice the scale is twice the width, not {wide} times");
        assert!((3.0..=5.0).contains(&area), "and four times the area, not {area} times");
    }

    #[test]
    fn the_circles_draw_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        use crate::toolkit::widget::{drawing, Length, Node};
        drawn::none(drawn::in_every_look("a notification circle", 2, || {
            let (alone, many, drive) = (Buttons { many: false, extra: None }, Buttons { many: true, extra: None }, Buttons { many: true, extra: Some(Act::Drive) });
            for (face, buttons, hover, pinned, msg, across) in [
                (Face::Note, alone, false, false, SHORT, 160.0f32),
                (Face::Note, many, true, true, LONG, 232.0),
                (Face::Note, many, true, false, SHORT, 160.0),
                (Face::Note, drive, true, false, SHORT, 200.0),
                (Face::Snooze, many, true, false, SHORT, 160.0),
            ] {
                let view: Node<()> = drawing((260.0, 260.0), "a notification circle", move |canvas, pen| {
                    let at = (10.0 * pen.scale, 10.0 * pen.scale);
                    paint_circle(canvas, pen, at, &Spec { across, scroll: 0, icon: "mail-unread-symbolic", title: msg.0, body: msg.1, face, buttons, hover, pinned, hot: None, copied: false, moving: false });
                }).width(Length::Fill).height(Length::Fill);
                drawn::window(&format!("{face:?} {hover} {pinned}"), view, Theme::chosen(), (260, 260), true);
            }
        }));
    }

}
