//! The blocks laid out in rows of pieces at a width, for the window and for PDF pages.

use crate::toolkit::colour::Colour;
use crate::toolkit::text::{Fonts, Layout, Style};
use crate::toolkit::theme::{alpha, mix};
use crate::toolkit::theme;
use super::{Align, Block, Item};
use super::Span;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ink {
    Text,
    Muted,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fill {
    Code,

    Bar,
    Rule,

    Edge,

    Head,

    Box,
    Ticked,
}

pub enum Piece {
    Words {
        layout: Layout,
        x: f32,
        y: f32,

        base: f32,
        ink: Ink,
        under: bool,
        strike: bool,
        link: Option<usize>,
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: Fill,
    },
}

impl Piece {
    fn shift(&mut self, dx: f32, dy: f32) {
        match self {
            Piece::Words { x, y, .. } | Piece::Rect { x, y, .. } => {
                *x += dx;
                *y += dy;
            }
        }
    }

    pub(super) fn right(&self) -> f32 {
        match self {
            Piece::Words { layout, x, .. } => x + layout.width,
            Piece::Rect { x, w, .. } => x + w,
        }
    }
}

#[derive(Default)]
pub struct Row {
    pub height: f32,
    pub pieces: Vec<Piece>,
}

pub struct Laid {
    pub rows: Vec<Row>,
    pub links: Vec<String>,
    pub height: f32,
}

pub struct Palette {
    pub page: Colour,
    pub text: Colour,
    pub muted: Colour,
    pub link: Colour,
    pub code: Colour,
    pub edge: Colour,
    pub head: Colour,
    pub tick: Colour,
}

impl Palette {

    pub fn from_theme(t: &theme::Theme) -> Palette {
        let (code, head) = (mix(t.field, t.fg, 0.07), alpha(t.fg, 0.05));
        let page = theme::over(t.field, theme::over(t.panel, t.bg));
        let grounds = [page, theme::over(code, page), theme::over(head, page)];
        Palette {
            page: t.field,
            text: t.words_on(&grounds),
            muted: t.quiet_on(&grounds),
            link: t.link_on(&grounds),
            code,
            edge: alpha(t.fg, 0.22),
            head,
            tick: t.accent,
        }
    }

    pub fn print() -> Palette {
        let grey = |v: f32| Colour::rgb(v, v, v);
        Palette {
            page: grey(1.0),
            text: grey(0.1),
            muted: grey(0.36),
            link: Colour::rgb(0.1, 0.34, 0.69),
            code: grey(0.95),
            edge: grey(0.8),
            head: grey(0.97),
            tick: Colour::rgb(0.1, 0.34, 0.69),
        }
    }
}

pub fn mono(families: &[String]) -> Option<String> {
    const LIKED: [&str; 5] = ["DejaVu Sans Mono", "Noto Sans Mono", "Liberation Mono", "Fira Mono", "Source Code Pro"];
    LIKED
        .iter()
        .find(|l| families.iter().any(|f| f == *l))
        .map(|s| s.to_string())
        .or_else(|| families.iter().find(|f| f.contains("Mono")).cloned())
}

#[derive(Clone, Copy, Default)]
pub(super) struct Fmt {
    bold: bool,
    italic: bool,
    pub(super) strike: bool,
    pub(super) code: bool,
    pub(super) link: Option<usize>,
}

struct Bit {
    text: String,
    fmt: Fmt,
}

enum Tok {
    Word(Vec<Bit>),
    Break,
}

pub struct Lay<'a> {
    fonts: &'a Fonts,

    size: f32,
    mono: Option<String>,
    rows: Vec<Row>,
    links: Vec<String>,

    quoted: usize,

    listed: usize,
}

impl<'a> Lay<'a> {
    pub fn new(fonts: &'a Fonts, size: f32, mono: Option<String>) -> Lay<'a> {
        Lay { fonts, size, mono, rows: Vec::new(), links: Vec::new(), quoted: 0, listed: 0 }
    }

    pub fn lay(mut self, blocks: &[Block], width: f32) -> Laid {
        self.blocks(blocks, 0.0, width);
        while self.rows.last().is_some_and(|r| r.pieces.is_empty()) {
            self.rows.pop();
        }
        let height = self.rows.iter().map(|r| r.height).sum();
        Laid { rows: self.rows, links: self.links, height }
    }

    fn gap(&mut self, h: f32) {
        if self.rows.is_empty() {
            return;
        }
        match self.rows.last_mut() {
            Some(r) if r.pieces.is_empty() => r.height = r.height.max(h),
            _ => self.rows.push(Row { height: h, pieces: Vec::new() }),
        }
    }

    fn para_gap(&self) -> f32 {
        if self.listed > 0 {
            self.size * 0.25
        } else {
            self.size * 0.75
        }
    }

    fn blocks(&mut self, blocks: &[Block], left: f32, right: f32) {
        for b in blocks {
            self.block(b, left, right);
        }
    }

    fn style(&self, size: f32, fmt: Fmt) -> Style {
        Style {
            family: if fmt.code { self.mono.clone() } else { None },
            weight: if fmt.bold { 700 } else { 400 },
            italic: fmt.italic,
            ..Style::new(if fmt.code { size * 0.9 } else { size })
        }
    }

    fn block(&mut self, b: &Block, left: f32, right: f32) {
        match b {
            Block::Heading(level, spans) => self.heading(*level, spans, left, right),
            Block::Para(spans) => {
                let rows = self.flow(spans, self.size, Fmt::default(), right - left);
                self.push(rows, left);
                self.gap(self.para_gap());
            }
            Block::Quote(inner) => self.quote(inner, left, right),
            Block::List(start, items) => self.list(*start, items, left, right),
            Block::Code(_, code) => self.code_block(code, left, right),
            Block::Rule => self.rule(left, right),
            Block::Table(align, head, rows) => self.table(align, head, rows, left, right),
        }
    }

    fn heading(&mut self, level: u8, spans: &[Span], left: f32, right: f32) {
        let s = self.size;
        let scale = [2.0, 1.6, 1.3, 1.1, 1.0, 0.9][(level as usize).clamp(1, 6) - 1];
        self.gap(s * 0.9);
        let rows = self.flow(spans, s * scale, Fmt { bold: true, ..Fmt::default() }, right - left);
        self.push(rows, left);
        if level <= 2 {
            let line = (s / 15.0).max(1.0);
            self.rows.push(Row {
                height: s * 0.4,
                pieces: vec![Piece::Rect { x: left, y: s * 0.2, w: right - left, h: line, fill: Fill::Rule }],
            });
        }
        self.gap(s * 0.5);
    }

    fn quote(&mut self, inner: &[Block], left: f32, right: f32) {
        let s = self.size;
        let first = self.rows.len();
        self.quoted += 1;
        self.blocks(inner, left + s * 1.2, right);
        self.quoted -= 1;
        while self.rows.len() > first && self.rows.last().is_some_and(|r| r.pieces.is_empty()) {
            self.rows.pop();
        }
        let bar = (s / 5.0).max(2.0);
        for row in &mut self.rows[first..] {
            row.pieces.push(Piece::Rect { x: left, y: 0.0, w: bar, h: row.height, fill: Fill::Bar });
        }
        self.gap(self.para_gap());
    }

    fn list(&mut self, start: Option<u64>, items: &[Item], left: f32, right: f32) {
        let s = self.size;
        self.listed += 1;
        let indent = s * 1.6;
        for (k, item) in items.iter().enumerate() {
            let first = self.rows.len();
            self.blocks(&item.blocks, left + indent, right);
            if self.rows.len() == first {
                let h = self.line_height(s);
                self.rows.push(Row { height: h, pieces: Vec::new() });
            }
            let mark = self.list_mark(item.task, start.map(|n| n + k as u64), first, left + indent);
            self.rows[first].pieces.push(mark);
        }
        self.listed -= 1;
        self.gap(self.para_gap());
    }

    fn list_mark(&self, task: Option<bool>, number: Option<u64>, first: usize, text_left: f32) -> Piece {
        let s = self.size;
        let (top, base) = self.rows[first]
            .pieces
            .iter()
            .find_map(|p| match p {
                Piece::Words { y, base, .. } => Some((*y, *base)),
                _ => None,
            })
            .unwrap_or((0.0, s));
        match task {
            Some(done) => {
                let size = s * 0.8;
                let (x, y) = (text_left - size - s * 0.4, top + base - size * 0.9);
                Piece::Rect { x, y, w: size, h: size, fill: if done { Fill::Ticked } else { Fill::Box } }
            }
            None => {
                let mark = match number {
                    Some(n) => format!("{n}."),
                    None => ["\u{2022}", "\u{25E6}", "\u{25AA}"][(self.listed - 1) % 3].to_string(),
                };
                let style = self.style(s, Fmt::default());
                let layout = self.fonts.layout(&mark, &style, None);
                let mbase = layout.lines.first().map(|l| l.baseline - l.top).unwrap_or(s);
                let x = text_left - layout.width - s * 0.4;
                let ink = if self.quoted > 0 { Ink::Muted } else { Ink::Text };
                Piece::Words { layout, x, y: top + base - mbase, base: mbase, ink, under: false, strike: false, link: None }
            }
        }
    }

    fn code_block(&mut self, code: &str, left: f32, right: f32) {
        let s = self.size;
        let pad = s * 0.6;
        let style = self.style(s, Fmt { code: true, ..Fmt::default() });
        let lines: Vec<&str> = code.split('\n').collect();
        let last = lines.len() - 1;
        for (k, line) in lines.iter().enumerate() {
            let text = if line.is_empty() { " " } else { line };
            let layout = self.fonts.layout(text, &style, Some((right - left - 2.0 * pad).max(s)));
            let top = if k == 0 { pad } else { 0.0 };
            let height = top + layout.height.max(s) + if k == last { pad } else { 0.0 };
            let base = layout.lines.first().map(|l| l.baseline - l.top).unwrap_or(s);
            let ink = if self.quoted > 0 { Ink::Muted } else { Ink::Text };
            self.rows.push(Row {
                height,
                pieces: vec![
                    Piece::Rect { x: left, y: 0.0, w: right - left, h: height, fill: Fill::Code },
                    Piece::Words { layout, x: left + pad, y: top, base, ink, under: false, strike: false, link: None },
                ],
            });
        }
        self.gap(self.para_gap());
    }

    fn rule(&mut self, left: f32, right: f32) {
        let s = self.size;
        self.gap(s * 0.5);
        let line = (s / 15.0).max(1.0);
        self.rows.push(Row { height: s, pieces: vec![Piece::Rect { x: left, y: s * 0.5, w: right - left, h: line, fill: Fill::Rule }] });
        self.gap(s * 0.5);
    }

    fn table(&mut self, align: &[Align], head: &[Vec<Span>], rows: &[Vec<Vec<Span>>], left: f32, right: f32) {
        let line = (self.size / 15.0).max(1.0);
        let room = right - left;
        let widths = self.column_widths(align.len().max(1), head, rows, room);
        let all = std::iter::once(head).chain(rows.iter().map(Vec::as_slice));
        let count = rows.len() + 1;
        for (r, cells) in all.enumerate() {
            let (pieces, height) = self.table_cells(cells, r == 0, align, &widths, left);
            let mut lines = vec![Piece::Rect { x: left, y: 0.0, w: room, h: line, fill: Fill::Edge }];
            if r == 0 {
                lines.insert(0, Piece::Rect { x: left, y: 0.0, w: room, h: height, fill: Fill::Head });
            }
            if r + 1 == count {
                lines.push(Piece::Rect { x: left, y: height - line, w: room, h: line, fill: Fill::Edge });
            }
            let mut x = left;
            for w in widths.iter() {
                lines.push(Piece::Rect { x, y: 0.0, w: line, h: height, fill: Fill::Edge });
                x += w;
            }
            lines.push(Piece::Rect { x: right - line, y: 0.0, w: line, h: height, fill: Fill::Edge });
            lines.extend(pieces);
            self.rows.push(Row { height, pieces: lines });
        }
        self.gap(self.para_gap());
    }

    fn column_widths(&mut self, n: usize, head: &[Vec<Span>], rows: &[Vec<Vec<Span>>], room: f32) -> Vec<f32> {
        let s = self.size;
        let pad = s * 0.5;
        let bold = Fmt { bold: true, ..Fmt::default() };
        let mut natural = vec![s * 2.0; n];
        for (r, row) in std::iter::once(head).chain(rows.iter().map(Vec::as_slice)).enumerate() {
            for (c, cell) in row.iter().enumerate().take(n) {
                let laid = self.flow(cell, s, if r == 0 { bold } else { Fmt::default() }, f32::INFINITY);
                let w = laid.iter().flat_map(|r| r.pieces.iter().map(Piece::right)).fold(0.0, f32::max);
                natural[c] = natural[c].max(w + 2.0 * pad);
            }
        }
        let total: f32 = natural.iter().sum();
        natural.iter().map(|w| w * room / total).collect()
    }

    fn table_cells(&mut self, cells: &[Vec<Span>], heading: bool, align: &[Align], widths: &[f32], left: f32) -> (Vec<Piece>, f32) {
        let s = self.size;
        let pad = s * 0.5;
        let fmt = if heading { Fmt { bold: true, ..Fmt::default() } } else { Fmt::default() };
        let mut pieces = Vec::new();
        let mut height: f32 = self.line_height(s);
        let mut x = left;
        for (c, w) in widths.iter().enumerate() {
            let empty = Vec::new();
            let cell = cells.get(c).unwrap_or(&empty);
            let laid = self.flow(cell, s, fmt, w - 2.0 * pad);
            let mut y = pad;
            for mut row in laid {
                let used = row.pieces.iter().map(Piece::right).fold(0.0, f32::max);
                let dx = match align[c] {
                    Align::Right => w - 2.0 * pad - used,
                    Align::Centre => (w - 2.0 * pad - used) / 2.0,
                    _ => 0.0,
                };
                for p in &mut row.pieces {
                    p.shift(x + pad + dx.max(0.0), y);
                }
                y += row.height;
                pieces.extend(row.pieces);
            }
            height = height.max(y - pad);
            x += w;
        }
        (pieces, height + 2.0 * pad)
    }

    fn line_height(&self, size: f32) -> f32 {
        let l = self.fonts.layout("Ag", &Style::new(size), None);
        l.height * 1.25
    }

    fn push(&mut self, rows: Vec<Row>, left: f32) {
        for mut row in rows {
            for p in &mut row.pieces {
                p.shift(left, 0.0);
            }
            self.rows.push(row);
        }
    }
}

mod flow;
