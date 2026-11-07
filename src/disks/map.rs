//! The map of a drive's parts, drawn as wide as each is big.

use tiny_skia::PixmapMut;
use crate::toolkit::text::Style;
use crate::toolkit::tiles::{Sketch, worry};
use crate::toolkit::widget::{Length, Node, Pen, card, column, drawing};
use crate::toolkit::{t, theme};
use super::Msg;
use super::model::percent;
use super::view::RING;

pub(super) fn map_caption() -> &'static str {
    t!("disks-map-caption")
}

pub(super) const MAP_ROW: f32 = 98.0;
pub(super) const MAP_GAP: f32 = 16.0;
const MAP_TITLE: f32 = 26.0;
pub(super) const MAP_BAR: f32 = 20.0;
const MAP_ROUND: f32 = 6.0;
const MAP_GROW_MOST: f32 = 40.0;
const SEG_GAP: f32 = 4.0;
pub(super) const SEG_LEAST: f32 = 12.0;
const SEG_LABELLED: f32 = 64.0;
pub(super) const MAP_LEAST: f32 = 120.0;

pub(super) fn bands(rows: Vec<Node<Msg>>) -> Node<Msg> {
    column(rows.into_iter().enumerate().map(|(n, r)| {
        let r = column(vec![r]).padding(8.0);
        if n % 2 == 1 { r.banded() } else { card(r).snug().flat() }
    }).collect()).gap(0.0)
}

pub(super) fn amount_ring(share: f64, tip: String) -> Node<Msg> {
    drawing((RING, RING), tip, move |canvas, pen| {
        let mut s = Sketch::fit(canvas, pen, (84.0, 84.0));
        let track = s.track();
        s.ring((42.0, 42.0), 34.0, 15.0, track);
        let colour = worry(share).unwrap_or_else(|| s.highlight());
        s.arc((42.0, 42.0), 34.0, 15.0, 0.0, share.clamp(0.0, 1.0) as f32, colour);
    })
}

pub(super) struct MapPart {
    pub(super) size: u64,
    pub(super) name: String,
    pub(super) words: String,
    pub(super) full: Option<f64>,
    pub(super) gap: bool,
    pub(super) quiet: bool,
}

pub(super) struct MapRow {
    pub(super) name: String,
    pub(super) words: String,
    pub(super) parts: Vec<MapPart>,
    pub(super) quiet: bool,
}

impl MapRow {
    pub(super) fn spoken(&self) -> String {
        let parts: Vec<String> = self.parts.iter().map(|p| {
            match p.full {
                Some(f) => t!("disks-spoken-part-full", name = &p.name, full = percent(f), words = &p.words),
                None if p.words.is_empty() => p.name.clone(),
                None => t!("disks-spoken-part", name = &p.name, words = &p.words),
            }
        }).collect();
        t!("disks-spoken-drive", name = &self.name, words = &self.words, parts = parts.join("; "))
    }
}

pub(super) fn segments(rows: &[MapRow], wide: f32) -> Vec<(f32, f32, usize, usize)> {
    let mut out = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        let n = row.parts.len().max(1);
        let total: f64 = row.parts.iter().map(|p| p.size as f64).sum::<f64>().max(1.0);
        let room = (wide - SEG_GAP * (n - 1) as f32).max(n as f32 * SEG_LEAST);
        let spare = room - n as f32 * SEG_LEAST;
        let mut x = 0.0;
        for (i, p) in row.parts.iter().enumerate() {
            let w = SEG_LEAST + spare * (p.size as f64 / total) as f32;
            out.push((x, w, r, i));
            x += w + SEG_GAP;
        }
    }
    out
}

pub(super) fn hidden_share() -> f32 {
    theme::Theme::chosen().dim(theme::sheet().hidden_drive, theme::AIM_WORDS)
}

pub(super) fn map_picture(rows: Vec<MapRow>, tip: String) -> Node<Msg> {
    let tall = rows.len().max(1) as f32 * MAP_ROW - MAP_GAP;
    let count = rows.len();
    drawing((0.0, tall), tip, move |canvas, pen| {
        let s = pen.scale;
        let quiet = pen.theme.dim(theme::sheet().hidden_drive, theme::AIM_WORDS);
        let (wide, high) = (canvas.width() as f32 / s, canvas.height() as f32 / s);
        let (pitch, bar, top) = map_spacing(high, count);
        let map = Drawn { rows: &rows, segs: segments(&rows, wide), pitch, bar, top, quiet, wide, high };
        map.paint_bars(canvas, pen);
        map.paint_names(canvas, pen);
    }).width(Length::Fill).height(Length::Fill)
}

struct Drawn<'a> {
    rows: &'a [MapRow],
    segs: Vec<(f32, f32, usize, usize)>,
    pitch: f32,
    bar: f32,
    top: f32,
    quiet: f32,
    wide: f32,
    high: f32,
}

impl Drawn<'_> {
    fn paint_bars(&self, canvas: &mut PixmapMut, pen: &Pen) {
        let (rows, bar, top, pitch, quiet) = (self.rows, self.bar, self.top, self.pitch, self.quiet);
        let mut sk = Sketch::strip(canvas, pen, self.high);
        let (track, boxed) = (sk.track(), sk.boxed());
        let first = theme::pair()[0];
        for &(x, w, r, i) in &self.segs {
            let y = top + r as f32 * pitch + MAP_TITLE;
            let p = &rows[r].parts[i];
            let shown = |c| if p.quiet { theme::alpha(c, quiet) } else { c };
            match (p.full, p.gap) {
                (Some(share), _) => {
                    sk.rect((x, y, w, bar), MAP_ROUND, shown(track));
                    if share > 0.0 {
                        let across = (w * share as f32).max(2.0 * MAP_ROUND).min(w);
                        sk.rect((x, y, across, bar), MAP_ROUND, shown(worry(share).unwrap_or(first)));
                    }
                }
                (None, true) => sk.rect((x, y, w, bar), MAP_ROUND, shown(track)),
                (None, false) => sk.rect((x, y, w, bar), MAP_ROUND, shown(boxed)),
            }
        }
    }

    fn paint_names(&self, canvas: &mut PixmapMut, pen: &Pen) {
        let (rows, bar, top, pitch, quiet, wide) = (self.rows, self.bar, self.top, self.pitch, self.quiet, self.wide);
        let s = pen.scale;
        let style = |size: f32, weight: u16| Style { weight, ..Style::new(size * s) };
        let dots = |size: f32, weight: u16| pen.fonts.layout("\u{2026}", &style(size, weight), None);
        let t = pen.theme;
        for (r, row) in rows.iter().enumerate() {
            let y = top + r as f32 * pitch;
            let name = pen.fonts.layout(&row.name, &style(14.0, 700), None).cut(wide * 0.6 * s, &dots(14.0, 700));
            let ink = |c| if row.quiet { theme::alpha(c, quiet) } else { c };
            pen.fonts.draw(canvas, &name, 0.0, (y * s).round(), ink(t.fg));
            let left = wide * s - name.width - 8.0 * s;
            if left > 40.0 * s {
                let said = pen.fonts.layout(&row.words, &style(13.0, 400), None).cut(left, &dots(13.0, 400));
                pen.fonts.draw(canvas, &said, (name.width + 8.0 * s).round(), (y * s + (name.height - said.height) / 2.0).round(), t.muted);
            }
        }
        for &(x, w, r, i) in &self.segs {
            if w < SEG_LABELLED {
                continue;
            }
            let p = &rows[r].parts[i];
            let y = (top + r as f32 * pitch + MAP_TITLE + bar + 6.0) * s;
            let name = pen.fonts.layout(&p.name, &style(12.0, 600), None).cut(w * s, &dots(12.0, 600));
            let ink = |c| if p.quiet { theme::alpha(c, quiet) } else { c };
            pen.fonts.draw(canvas, &name, (x * s).round(), y.round(), ink(t.fg));
            if !p.words.is_empty() {
                let words = pen.fonts.layout(&p.words, &style(12.0, 400), None).cut(w * s, &dots(12.0, 400));
                pen.fonts.draw(canvas, &words, (x * s).round(), (y + name.height).round(), t.muted);
            }
        }
    }
}

pub(super) fn map_spacing(high: f32, count: usize) -> (f32, f32, f32) {
    let n = count.max(1) as f32;
    let need = n * MAP_ROW - MAP_GAP;
    let extra = (high - need).max(0.0);
    let grow = (extra / n).min(MAP_GROW_MOST);
    let top = ((extra - grow * n) / 2.0).max(0.0);
    (MAP_ROW + grow, MAP_BAR + grow * 0.5, top)
}
