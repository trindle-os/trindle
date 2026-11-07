//! The window's page.

use super::plan::size_text;
use crate::toolkit::tiles::{empty_state, error_state, loading_state, tile_head};
use crate::toolkit::widget::{Length, Node, Tone, bento, button, card, column, heading, note, page_title, placed, row, scroll, space, title};
use crate::toolkit::{popup, t, theme};
use super::{Drives, Msg, help, plan, red};
use super::model::percent;
use super::map::map_caption;
use super::map::MAP_LEAST;
use super::map::amount_ring;
use super::map::MapPart;
use super::map::MapRow;
use super::map::map_picture;

fn help() -> [&'static str; 3] {
    [t!("disks-help-1"), t!("disks-help-2"), t!("disks-help-3")]
}
pub(super) const SHORT_LINE: usize = 56;

const HELP_WIDE: f32 = 460.0;
const SHEET_WIDE: f32 = 600.0;
const CARD_LEAST: f32 = 300.0;
const HEADLINE_ROOM: f32 = 12.0;
pub(super) const RING: f32 = 60.0;
const PLACE_WORDS: f32 = 240.0;

impl Drives {
    pub(super) fn line(&self, drive: &str, at: &str, head: Node<Msg>, sub: &str, controls: Vec<Node<Msg>>, arm: Option<(String, String)>) -> Node<Msg> {
        let mut words = vec![head];
        if !sub.is_empty() {
            words.push(note(sub));
        }
        let mut top = vec![column(words).gap(4.0).width(Length::Fill)];
        if !controls.is_empty() {
            top.push(row(controls).gap(8.0).width(Length::Shrink).wraps());
        }
        let mut out = vec![row(top).gap(8.0).centred().stacks()];
        if let Some((key, warning)) = arm {
            if self.armed.as_deref() == Some(key.as_str()) {
                out.push(note(warning).tone(Tone::Problem));
            }
        }
        if let Some((w, t)) = self.told(drive, at) {
            out.push(match self.told_page(drive, at) {
                Some(page) if t == Tone::Problem => red(w, page),
                _ => note(w).tone(t),
            });
        }
        column(out).gap(4.0)
    }

    pub(super) fn change_going(&self) -> bool {
        self.changing.get() || self.snapping
    }

    pub(super) fn shut(&self) -> Option<&'static str> {
        self.change_going().then(plan::change_going)
    }

    pub(super) fn enter(&self, box_: Node<Msg>, m: Msg) -> Node<Msg> {
        if self.shut().is_some() { box_ } else { box_.on_submit(m) }
    }

    pub(super) fn toggle(&self, icon: &str, tip: &str, key: String) -> Node<Msg> {
        button(icon, tip).on(self.open.as_deref() == Some(key.as_str())).on_press(Msg::Toggle(key))
    }

    pub(super) fn names(&self, key: &str) -> bool {
        self.drives.iter().any(|d| {
            key == format!("{}:wipe", d.path)
                || d.vols.iter().any(|v| v.path == key)
                || d.gaps.iter().any(|&(o, _)| key == format!("{}:{o}", d.path))
        })
    }

    pub(super) fn page(&self) -> Node<Msg> {
        let info = button("dialog-information-symbolic", t!("disks-about-tip")).size(theme::sheet().info)
            .on(self.help).on_press(Msg::Help);
        let head = row(vec![page_title(t!("disks-title")), info, space()]).gap(12.0).centred();
        let (words, tone) = if self.missing.is_some() {
            (t!("disks-cannot-read").to_string(), Tone::Problem)
        } else if self.headline.1 == Tone::Problem {
            (self.headline.0.clone(), Tone::Problem)
        } else {
            (self.headline.0.clone(), Tone::Plain)
        };
        let mut shown = crate::toolkit::widget::headline(words).tone(tone).width(Length::Fill);
        if tone == Tone::Problem {
            let page = if self.missing.is_some() {
                help::SERVICE
            } else if self.drives.iter().any(|d| d.health.as_ref().is_some_and(|h| h.failing)) {
                help::HEALTH
            } else {
                help::FULL
            };
            shown = row(vec![shown, crate::common::more::button(Msg::More(page))]).gap(8.0).centred();
        }
        let said = column(vec![
            shown,
            space().height(Length::Fixed(HEADLINE_ROOM)),
        ]).gap(0.0);
        let mut all = vec![head, said];
        if let Some((what, package)) = self.missing {
            let b = theme::bento();
            let retry = button("view-refresh-symbolic", t!("disks-try-again")).on_press(Msg::Retry);
            let words = t!("disks-missing", what = what, package = package);
            all.push(card(column(vec![
                tile_head("drive-harddisk-symbolic", t!("disks-title"), Vec::new()),
                error_state(words, retry),
            ]).gap(16.0)).beside().span(b.picture.0, b.picture.1, b.words));
        } else {
            all.push(self.overview());
            if self.loaded {
                all.extend(self.places());
                all.extend(self.drive_cards());
            }
        }
        let b = theme::bento();
        scroll("drives", bento(CARD_LEAST, b.page_gap, all).padding(theme::sheet().page_padding)).height(Length::Fill)
    }

    fn overview(&self) -> Node<Msg> {
        let b = theme::bento();
        let mut all = vec![tile_head("drive-harddisk-symbolic", t!("disks-your-drives"), Vec::new()), note(map_caption())];
        if !self.loaded {
            all.push(loading_state(MAP_LEAST));
        } else if self.drives.is_empty() {
            let again = button("view-refresh-symbolic", t!("disks-look-again")).on_press(Msg::Retry);
            all.push(empty_state(t!("disks-no-drives"), Some(again)));
        } else {
            let rows = self.map_rows();
            let tip = rows.iter().map(MapRow::spoken).collect::<Vec<_>>().join(" ");
            all.push(map_picture(rows, tip));
            all.push(note(self.together()));
        }
        card(column(all).gap(16.0)).height(Length::Fill).beside().span_grows(b.picture.0, b.picture.1, b.words)
    }

    pub(super) fn map_rows(&self) -> Vec<MapRow> {
        self.drives.iter().map(|d| {
            let mut parts: Vec<(u64, MapPart)> = Vec::new();
            let all_quiet = self.drive_hidden(d);
            for v in d.vols.iter().filter(|v| !v.container()) {
                let at = v.part.map(|p| p.0).unwrap_or(0);
                let (words, full) = match v.full {
                    Some((used, free)) => (t!("disks-free", size = size_text(free as f64)), Some(used as f64 / (used + free).max(1) as f64)),
                    None if v.unmounted() => (t!("disks-map-not-mounted").to_string(), None),
                    None => (v.kind_words(), None),
                };
                parts.push((at, MapPart { size: v.size.max(1), name: v.name(), words, full, gap: false, quiet: self.part_hidden(d, v) }));
            }
            for &(o, s) in &d.gaps {
                parts.push((o, MapPart { size: s.max(1), name: t!("disks-free-space").into(), words: size_text(s as f64), full: None, gap: true, quiet: all_quiet }));
            }
            parts.sort_by_key(|p| p.0);
            let mut parts: Vec<MapPart> = parts.into_iter().map(|p| p.1).collect();
            if parts.is_empty() {
                let words = if d.size == 0 { t!("disks-map-no-disc") } else { t!("disks-map-empty") };
                parts.push(MapPart { size: 1, name: words.into(), words: String::new(), full: None, gap: true, quiet: all_quiet });
            }
            let size = size_text(d.size as f64);
            let words = match (d.removable, all_quiet) {
                (true, false) => t!("disks-drive-removable", size = size),
                (false, false) => t!("disks-drive-internal", size = size),
                (true, true) => t!("disks-drive-removable-hidden", size = size),
                (false, true) => t!("disks-drive-internal-hidden", size = size),
            };
            MapRow { name: d.name.clone(), words, parts, quiet: all_quiet }
        }).collect()
    }

    fn places(&self) -> Vec<Node<Msg>> {
        let b = theme::bento();
        self.rings.iter().map(|r| {
            let tip = t!("disks-ring-tip", name = &r.name, full = percent(r.share), free = size_text(r.free as f64), whole = size_text(r.whole as f64));
            let sentence = t!("disks-ring-still-free", free = size_text(r.free as f64), place = r.words);
            let body = column(vec![
                note(sentence),
                row(vec![
                    title(percent(r.share)),
                    amount_ring(r.share, tip),
                    note(t!("disks-ring-of", whole = size_text(r.whole as f64))).width(Length::Fill),
                ]).gap(12.0).centred(),
            ]).gap(12.0);
            card(column(vec![tile_head(r.icon, r.name.clone(), Vec::new()), body]).gap(16.0))
                .height(Length::Fill).beside().span_grows(b.figure.0, b.figure.1, PLACE_WORDS)
        }).collect()
    }

    pub(super) fn together(&self) -> String {
        let total: u64 = self.drives.iter().map(|d| d.size).sum();
        t!("disks-together", n = self.drives.len(), total = size_text(total as f64), health = &self.health_said)
    }

    pub(super) fn sheet(&self) -> Option<Node<Msg>> {
        let key = self.open.clone()?;
        let kinds = self.kinds();
        let mut found = None;
        for d in &self.drives {
            if key == format!("{}:wipe", d.path) {
                if let Some(target) = d.whole.as_ref() {
                    found = Some((t!("disks-sheet-erase", name = &d.name), d.words(), vec![self.format_row(d, target, true, &kinds)]));
                }
            }
            for v in d.vols.iter().filter(|v| v.path == key) {
                found = Some((v.name(), t!("disks-sheet-part", size = size_text(v.size as f64), drive = d.words()), self.part_extras(d, v, &kinds)));
            }
            for &(o, s) in d.gaps.iter().filter(|&&(o, _)| key == format!("{}:{o}", d.path)) {
                found = Some((t!("disks-sheet-new").to_string(), t!("disks-sheet-gap", size = size_text(s as f64), drive = d.words()),
                              vec![self.gap_extra(d, o, s, &kinds)]));
            }
        }
        let (name, words, rows) = found?;
        let close = button("window-close-symbolic", t!("disks-close")).on_press(Msg::Toggle(key));
        let head = row(vec![heading(name), space(), close]).gap(12.0).centred();
        let body = scroll("sheet rows", column(rows).gap(16.0).width(Length::Fill));
        let panel = card(column(vec![head, note(words), body]).gap(16.0)).floats().width(Length::Fixed(SHEET_WIDE));
        let at = self.sheet_at;
        Some(placed(panel, move |(w, h), (aw, ah)| {
            let x = at.x.min(aw - w - 8.0).max(8.0);
            let below = at.y + at.h + 8.0;
            let y = if below + h > ah - 8.0 { (at.y - h - 8.0).max(8.0) } else { below };
            (x, y.min((ah - h - 8.0).max(8.0)))
        }).within().stays(format!("sheet:{}", self.open.as_deref().unwrap_or(""))))
    }

    pub(super) fn help_panel(&self) -> Node<Msg> {
        let at = self.help_at;
        let words = column(help().iter().map(|p| note(*p).width(Length::Fill)).collect()).gap(12.0);
        let panel = card(scroll("help words", words.width(Length::Fill))).floats().width(Length::Fixed(HELP_WIDE));
        popup::under_the_i(panel, at).within()
    }
}
