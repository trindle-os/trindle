//! Flowing the spans of a block into wrapped rows.

use crate::toolkit::text::{Fonts, Layout};
use crate::text::markdown::Span;
use crate::text::markdown::layout::Row;
use crate::text::markdown::layout::Fmt;
use crate::text::markdown::layout::Bit;
use crate::text::markdown::layout::Tok;
use crate::text::markdown::layout::Lay;
use crate::text::markdown::paint::finish_line;

struct Lines {
    rows: Vec<Row>,
    line: Vec<(Layout, f32, Fmt)>,
    x: f32,
    size: f32,
    quoted: bool,
}

impl Lines {
    fn end_line(&mut self, fonts: &Fonts) {
        self.rows.push(finish_line(std::mem::take(&mut self.line), self.size, self.quoted, fonts));
        self.x = 0.0;
    }
}

impl Lay<'_> {
    fn toks(&mut self, spans: &[Span], fmt: Fmt, toks: &mut Vec<Tok>, joined: &mut bool) {
        for span in spans {
            match span {
                Span::Text(t) => self.words(t, fmt, toks, joined),
                Span::Code(t) => self.words(t, Fmt { code: true, ..fmt }, toks, joined),
                Span::Bold(x) => self.toks(x, Fmt { bold: true, ..fmt }, toks, joined),
                Span::Italic(x) => self.toks(x, Fmt { italic: true, ..fmt }, toks, joined),
                Span::Strike(x) => self.toks(x, Fmt { strike: true, ..fmt }, toks, joined),
                Span::Link(x, url) => {
                    self.links.push(url.clone());
                    let link = Some(self.links.len() - 1);
                    self.toks(x, Fmt { link, ..fmt }, toks, joined);
                }
                Span::Image(alt, _) => {
                    let alt = if alt.trim().is_empty() { "picture" } else { alt };
                    self.words(alt, Fmt { italic: true, ..fmt }, toks, joined);
                }
                Span::Break => {
                    toks.push(Tok::Break);
                    *joined = false;
                }
            }
        }
    }

    fn words(&self, text: &str, fmt: Fmt, toks: &mut Vec<Tok>, joined: &mut bool) {
        for (k, word) in text.split(' ').enumerate() {
            if k > 0 {
                *joined = false;
            }
            if word.is_empty() {
                continue;
            }
            let bit = Bit { text: word.to_string(), fmt };
            match toks.last_mut() {
                Some(Tok::Word(bits)) if *joined => bits.push(bit),
                _ => toks.push(Tok::Word(vec![bit])),
            }
            *joined = true;
        }
        if text.ends_with(' ') {
            *joined = false;
        }
    }

    pub(super) fn flow(&mut self, spans: &[Span], size: f32, fmt: Fmt, width: f32) -> Vec<Row> {
        let mut toks = Vec::new();
        let mut joined = false;
        self.toks(spans, fmt, &mut toks, &mut joined);
        let gap = {
            let st = self.style(size, Fmt::default());
            self.fonts.layout("a a", &st, None).width - self.fonts.layout("aa", &st, None).width
        };
        let mut lines = Lines { rows: Vec::new(), line: Vec::new(), x: 0.0, size, quoted: self.quoted > 0 };
        for tok in toks {
            match tok {
                Tok::Break => lines.end_line(self.fonts),
                Tok::Word(bits) => self.place_word(&mut lines, &bits, gap, width),
            }
        }
        if !lines.line.is_empty() {
            lines.end_line(self.fonts);
        }
        lines.rows
    }

    fn place_word(&self, lines: &mut Lines, bits: &[Bit], gap: f32, width: f32) {
        let size = lines.size;
        let mut laid: Vec<(Layout, Fmt)> = bits.iter().map(|b| (self.fonts.layout(&b.text, &self.style(size, b.fmt), None), b.fmt)).collect();
        let w: f32 = laid.iter().map(|l| l.0.width).sum();
        let space = if lines.line.is_empty() { 0.0 } else { gap };
        if lines.x + space + w > width && !lines.line.is_empty() {
            lines.end_line(self.fonts);
        }
        if w > width {
            laid = self.break_word(lines, bits, width);
        }
        let mut at = lines.x + if lines.line.is_empty() { 0.0 } else { gap };
        for (layout, fmt) in laid {
            let lw = layout.width;
            lines.line.push((layout, at, fmt));
            at += lw;
        }
        lines.x = at;
    }

    fn break_word(&self, lines: &mut Lines, bits: &[Bit], width: f32) -> Vec<(Layout, Fmt)> {
        let mut laid: Vec<(Layout, Fmt)> = Vec::new();
        for b in bits {
            let style = self.style(lines.size, b.fmt);
            let mut piece = String::new();
            for ch in b.text.chars() {
                let mut next = piece.clone();
                next.push(ch);
                let used: f32 = laid.iter().map(|l: &(Layout, Fmt)| l.0.width).sum::<f32>();
                let room = if lines.line.is_empty() { width - used } else { width - lines.x };
                if !piece.is_empty() && self.fonts.layout(&next, &style, None).width > room {
                    let l = self.fonts.layout(&piece, &style, None);
                    for (layout, fmt) in laid.drain(..).chain(std::iter::once((l, b.fmt))) {
                        let at = lines.x;
                        lines.x += layout.width;
                        lines.line.push((layout, at, fmt));
                    }
                    lines.end_line(self.fonts);
                    piece = ch.to_string();
                } else {
                    piece = next;
                }
            }
            if !piece.is_empty() {
                laid.push((self.fonts.layout(&piece, &style, None), b.fmt));
            }
        }
        laid
    }
}
