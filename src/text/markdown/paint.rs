//! Painting laid-out rows onto a canvas.

use tiny_skia::PixmapMut;
use crate::toolkit::editing::editor::rect;
use crate::toolkit::text::{Fonts, Layout, Style};
use super::layout::Ink;
use super::layout::Fill;
use super::layout::Piece;
use super::layout::Row;
use super::layout::Palette;
use super::layout::Fmt;

pub(super) fn finish_line(line: Vec<(Layout, f32, Fmt)>, size: f32, quoted: bool, fonts: &Fonts) -> Row {
    let metrics = |l: &Layout| l.lines.first().map(|f| (f.baseline - f.top, f.height - (f.baseline - f.top))).unwrap_or((l.height * 0.8, l.height * 0.2));
    let blank = fonts.layout("Ag", &Style::new(size), None);
    let (mut up, mut down) = metrics(&blank);
    for (l, _, _) in &line {
        let (a, d) = metrics(l);
        up = up.max(a);
        down = down.max(d);
    }
    let height = (up + down) * 1.25;
    let lead = (height - up - down) / 2.0;
    let base = lead + up;
    let mut pieces = Vec::new();
    let mut code: Option<(f32, f32)> = None;
    let mut words = Vec::new();
    let mut last: Option<(f32, Fmt)> = None;
    for (layout, x, fmt) in line {
        let (a, _) = metrics(&layout);
        let right = x + layout.width;
        if fmt.code {
            code = match code {
                Some((start, _)) if last.is_some_and(|(_, f)| f.code) => Some((start, right)),
                Some((start, end)) => {
                    pieces.push(code_rect(start, end, lead, up + down, size));
                    Some((x, right))
                }
                None => Some((x, right)),
            };
        }
        if let Some((end, f)) = last {
            if f.link.is_some() && f.link == fmt.link && x > end {
                words.push(Piece::Rect { x: end, y: base + size * 0.12, w: x - end, h: (size / 15.0).max(1.0), fill: Fill::Rule });
            }
        }
        last = Some((right, fmt));
        let ink = if fmt.link.is_some() {
            Ink::Link
        } else if quoted {
            Ink::Muted
        } else {
            Ink::Text
        };
        words.push(Piece::Words { layout, x, y: base - a, base: a, ink, under: fmt.link.is_some(), strike: fmt.strike, link: fmt.link });
    }
    if let Some((start, end)) = code {
        pieces.push(code_rect(start, end, lead, up + down, size));
    }
    for w in &mut words {
        if let Piece::Rect { fill, .. } = w {
            *fill = Fill::Rule;
        }
    }
    pieces.extend(words);
    Row { height, pieces }
}

fn code_rect(start: f32, end: f32, top: f32, h: f32, size: f32) -> Piece {
    let pad = size * 0.2;
    Piece::Rect { x: start - pad, y: top, w: end - start + 2.0 * pad, h, fill: Fill::Code }
}

pub fn paint_row(canvas: &mut PixmapMut, fonts: &Fonts, row: &Row, x: f32, y: f32, ink: &Palette, links: Option<usize>) {
    let line = |size: f32| (size / 15.0).max(1.0);
    for p in &row.pieces {
        match p {
            Piece::Rect { x: px, y: py, w, h, fill } => {
                let (rx, ry) = (x + px, y + py);
                match fill {
                    Fill::Code => rect(canvas, rx, ry, *w, *h, ink.code),
                    Fill::Bar => rect(canvas, rx, ry, *w, *h, ink.edge),
                    Fill::Rule => rect(canvas, rx, ry, *w, *h, ink.edge),
                    Fill::Edge => rect(canvas, rx, ry, *w, *h, ink.edge),
                    Fill::Head => rect(canvas, rx, ry, *w, *h, ink.head),
                    Fill::Ticked => rect(canvas, rx, ry, *w, *h, ink.tick),
                    Fill::Box => {
                        let t = line(*h * 1.2);
                        rect(canvas, rx, ry, *w, t, ink.muted);
                        rect(canvas, rx, ry + h - t, *w, t, ink.muted);
                        rect(canvas, rx, ry, t, *h, ink.muted);
                        rect(canvas, rx + w - t, ry, t, *h, ink.muted);
                    }
                }
            }
            Piece::Words { layout, x: px, y: py, base, ink: role, under, strike, link } => {
                let colour = match role {
                    Ink::Text => ink.text,
                    Ink::Muted => ink.muted,
                    Ink::Link => ink.link,
                };
                let (wx, wy) = (x + px, y + py);
                fonts.draw(canvas, layout, wx, wy, colour);
                let size = base / 0.8;
                if *under {
                    let hot = links.is_some() && links == *link;
                    rect(canvas, wx, wy + base + size * 0.12, layout.width, line(size) * if hot { 2.0 } else { 1.0 }, colour);
                }
                if *strike {
                    rect(canvas, wx, wy + base - size * 0.3, layout.width, line(size), colour);
                }
            }
        }
    }
}
