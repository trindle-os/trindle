//! The finished look in the window: laid out once per version and width, scrolled and clicked.

use tiny_skia::PixmapMut;
use crate::toolkit::editing::editor::{Look, rect};
use crate::toolkit::input::Keysym;
use crate::toolkit::theme::alpha;
use crate::toolkit::theme;
use crate::toolkit::widget::{Pen, Pointer};
use super::parse;
use super::layout::Piece;
use super::layout::Laid;
use super::layout::Palette;
use super::layout::mono;
use super::layout::Lay;
use super::paint::paint_row;

const MARGIN: f32 = 24.0;
const WHEEL_LINES: f32 = 3.0;

pub struct Finished {
    laid: Option<Laid>,

    key: (u64, u32, u32, u32),

    scroll: f32,
    pending: f32,
    scale: f32,
    size: (f32, f32),
    mono: Option<Option<String>>,
    pub over: bool,
    hot: Option<usize>,
}

impl Default for Finished {
    fn default() -> Self {
        Finished { laid: None, key: (u64::MAX, 0, 0, 0), scroll: 0.0, pending: 0.0, scale: 1.0, size: (0.0, 0.0), mono: None, over: false, hot: None }
    }
}

impl Finished {
    pub fn paint(&mut self, canvas: &mut PixmapMut, pen: &Pen, text: &str, version: u64, look: &Look) {
        let s = pen.scale;
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        if (s - self.scale).abs() > f32::EPSILON {
            self.scroll *= s / self.scale;
        }
        self.scale = s;
        self.size = (w, h);
        let mono = self.mono.get_or_insert_with(|| mono(&pen.fonts.families())).clone();
        let margin = MARGIN * s;
        let width = (w - 2.0 * margin).max(look.size * s * 4.0);
        let key = (version, width.to_bits(), s.to_bits(), look.size.to_bits());
        if self.key != key || self.laid.is_none() {
            self.key = key;
            self.laid = Some(Lay::new(pen.fonts, look.size * s, mono).lay(&parse(text), width));
            self.hot = None;
        }
        let Some(laid) = &self.laid else { return };
        let ink = Palette::from_theme(pen.theme);
        let line = look.size * s * 1.35;
        self.scroll += self.pending * if self.pending.abs() < 50.0 { line * WHEEL_LINES } else { s };
        self.pending = 0.0;
        let most = (laid.height + 2.0 * margin - h).max(0.0);
        self.scroll = self.scroll.clamp(0.0, most);
        rect(canvas, 0.0, 0.0, w, h, ink.page);
        let mut y = margin - self.scroll;
        for row in &laid.rows {
            if y > h {
                break;
            }
            if y + row.height >= 0.0 {
                paint_row(canvas, pen.fonts, row, margin, y, &ink, self.hot);
            }
            y += row.height;
        }
        if most > 0.0 {
            let total = laid.height + 2.0 * margin;
            let len = (h * h / total).max(theme::SCROLLBAR_MIN * s).min(h);
            let at = self.scroll / most * (h - len);
            let bw = theme::sheet().scrollbar * s;
            rect(canvas, w - bw - theme::sheet().scrollbar_inset * s, at, bw, len, alpha(pen.theme.fg, 0.3));
        }
    }

    pub fn scroll(&mut self, y: f32, steps: bool) {
        if steps {
            self.pending += y;
        } else {
            self.scroll += y * self.scale;
        }
    }

    pub fn link_at(&self, x: f32, y: f32) -> Option<String> {
        let laid = self.laid.as_ref()?;
        let margin = MARGIN * self.scale;
        let (px, py) = (x * self.scale - margin, y * self.scale - margin + self.scroll);
        let mut top = 0.0;
        for row in &laid.rows {
            if py < top + row.height {
                return row.pieces.iter().find_map(|p| match p {
                    Piece::Words { layout, x, y, link: Some(l), .. }
                        if px >= *x && px <= x + layout.width && py - top >= *y && py - top <= y + layout.height =>
                    {
                        laid.links.get(*l).cloned()
                    }
                    _ => None,
                });
            }
            top += row.height;
        }
        None
    }

    fn link_index(&self, url: Option<&String>) -> Option<usize> {
        let laid = self.laid.as_ref()?;
        let url = url?;
        laid.links.iter().position(|l| l == url)
    }

    pub fn pointer(&mut self, p: Pointer) -> Option<String> {
        match p {
            Pointer::Hover(x, y) | Pointer::Move(x, y) => {
                self.over = true;
                let link = self.link_at(x, y);
                self.hot = self.link_index(link.as_ref());
                None
            }
            Pointer::Leave => {
                self.over = false;
                self.hot = None;
                None
            }
            Pointer::Down(x, y) => {
                self.over = true;
                self.link_at(x, y)
            }
            _ => None,
        }
    }

    pub fn key(&mut self, sym: Keysym) -> bool {
        let page = (self.size.1 - 2.0 * MARGIN * self.scale).max(self.scale * 40.0);
        let line = self.scale * 40.0;
        match sym {
            Keysym::Up => self.scroll -= line,
            Keysym::Down => self.scroll += line,
            Keysym::Page_Up => self.scroll -= page,
            Keysym::Page_Down | Keysym::space => self.scroll += page,
            Keysym::Home => self.scroll = 0.0,
            Keysym::End => self.scroll = f32::MAX / 4.0,
            _ => return false,
        }
        true
    }
}
