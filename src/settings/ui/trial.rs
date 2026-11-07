//! A change on trial.

use std::time::Duration;

use super::{Kit, Msg, Page, on};
use crate::toolkit::t;
use crate::toolkit::colour::Colour;
use crate::toolkit::widget::{drawing,
                             row, Length, Node, Rect};

pub struct FootLook<'a> {
    pub ask: &'a str,
    pub more: &'a str,
    pub safe: bool,
    pub escape: bool,
}

#[derive(Default)]
pub struct Trial {
    left: Option<i32>,
    whole: i32,
    round: u32,
}

impl Trial {
    #[cfg(test)]
    pub(crate) fn at(left: i32) -> Trial {
        Trial { left: Some(left), whole: left * 2, round: 0 }
    }

    pub fn left(&self) -> Option<i32> {
        self.left
    }

    pub fn stop(&mut self) {
        self.round += 1;
        self.left = None;
    }

    pub fn start<P: Page + 'static>(kit: &mut Kit, page: &mut P, secs: i32, mine: fn(&mut P) -> &mut Trial, over: fn(&mut P, &mut Kit)) {
        let t = mine(page);
        t.round += 1;
        t.left = Some(secs);
        t.whole = secs;
        let round = t.round;
        Trial::count(kit, round, mine, over);
    }

    pub fn foot<P: Page + 'static>(&self, look: FootLook, turn: u32, back: fn(&mut P, &mut Kit), keep: fn(&mut P, &mut Kit)) -> Option<Node<Msg>> {
        let FootLook { ask, more, safe, escape } = look;
        use crate::toolkit::input::{Input, Keysym};
        use crate::toolkit::theme::{self, Theme};
        use crate::toolkit::widget::{layers, Pointer};
        let left = self.left?;
        let own = safe.then(theme::safe);
        let sizes = if safe { theme::safe_sheet() } else { theme::sheet() };
        let (tall, room) = (sizes.trial_foot, sizes.trial_foot_padding);
        let part = (tall - 2.0 * room).max(TRIAL_PART_LEAST);
        let under = if more.is_empty() { t!("settings-common-goes-back", left = left) } else { t!("settings-common-goes-back-more", left = left, more = more) };
        let look = move |t: &Theme| own.clone().unwrap_or_else(|| t.clone());
        let surface = {
            let look = look.clone();
            drawing((TRIAL_WORDS_LEAST, tall), "", move |canvas, pen| {
                let t = look(pen.theme);
                let (w, h) = (canvas.width() as f32, canvas.height() as f32);
                crate::toolkit::ui::fill(canvas, Rect { x: 0.0, y: 0.0, w, h }, 0.0, t.trial_foot);
                crate::toolkit::ui::fill(canvas, Rect { x: 0.0, y: 0.0, w, h: pen.scale.round().max(1.0) }, 0.0, t.trial_foot_line);
            }).key(TRIAL_FOOT).width(Length::Fill).height(Length::Fixed(tall))
        };
        let words = {
            let look = look.clone();
            trial_part((TRIAL_WORDS_LEAST, part), format!("{ask} {under}"), ask.to_string(), under, false, false, move |t| {
                let t = look(t);
                (t.trial_foot, t.fg, t.muted, Colour { a: 0.0, ..t.fg })
            }).width(Length::Fill)
        };
        let button = |name: &str, wide: f32, main: bool, press: fn(&mut P, &mut Kit)| {
            let look = look.clone();
            trial_part((wide, part), name.to_string(), name.to_string(), String::new(), true, main, move |t| {
                let t = look(t);
                if main { (t.accent, t.on_accent, t.on_accent, Colour { a: 0.0, ..t.fg }) } else { (t.bg, t.fg, t.fg, t.field_edge) }
            })
                .pressable(on::<P>(press))
                .on_pointer(move |p| match p {
                    Pointer::Up(..) => on::<P>(press),
                    _ => on::<P>(|_, _| {}),
                })
                .on_keys(move |input| match input {
                    Input::Key { sym: Keysym::Return | Keysym::space, .. } => Some(on::<P>(press)),
                    _ => None,
                })
        };
        let go_back = button(t!("settings-common-go-back-now"), TRIAL_BACK_WIDTH, false, back).key(TRIAL_BACK).grab(turn);
        let keep = button(t!("settings-common-keep"), TRIAL_KEEP_WIDTH, true, keep).key(TRIAL_KEEP);
        let gone = 1.0 - left as f32 / self.whole.max(1) as f32;
        let pie = crate::toolkit::tiles::time_pie(gone, part.min(TRIAL_PIE), move |t| {
            let t = look(t);
            (t.bg, t.accent)
        });
        let asked = row(vec![pie, words, go_back, keep]).gap(room).padding(room).width(Length::Fill);
        let foot = layers(vec![surface, asked]).height(Length::Fixed(tall));
        Some(if escape { foot.on_escape(on::<P>(back)) } else { foot })
    }

    fn count<P: Page + 'static>(kit: &Kit, round: u32, mine: fn(&mut P) -> &mut Trial, over: fn(&mut P, &mut Kit)) {
        kit.after::<P>(Duration::from_secs(1), move |me, k| {
            let t = mine(me);
            if t.round != round {
                return;
            }
            let Some(left) = t.left else { return };
            if left - 1 < 0 {
                over(me, k);
                return;
            }
            t.left = Some(left - 1);
            Trial::count(k, round, mine, over);
        });
    }
}

pub const TRIAL_FOOT: &str = "trial:foot";
pub const TRIAL_BACK: &str = "trial:go-back";
pub const TRIAL_KEEP: &str = "trial:keep";
const TRIAL_WORDS_LEAST: f32 = 240.0;
const TRIAL_BACK_WIDTH: f32 = 148.0;
const TRIAL_KEEP_WIDTH: f32 = 96.0;
const TRIAL_PART_LEAST: f32 = 32.0;
const TRIAL_PIE: f32 = 36.0;

fn trial_part(size: (f32, f32), tip: String, words: String, under: String, centred: bool, bold: bool,
              colours: impl Fn(&crate::toolkit::theme::Theme) -> (Colour, Colour, Colour, Colour) + 'static) -> Node<Msg> {
    use crate::toolkit::text::Style;
    drawing(size, tip, move |canvas, pen| {
        let k = pen.scale;
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let (fill, ink, quiet, edge) = colours(pen.theme);
        let radius = if centred { h / 2.0 } else { 0.0 };
        if edge.a > 0.0 {
            crate::toolkit::ui::fill(canvas, Rect { x: 0.0, y: 0.0, w, h }, radius, edge);
        }
        let line = if edge.a > 0.0 { k.round().max(1.0) } else { 0.0 };
        crate::toolkit::ui::fill(canvas, Rect { x: line, y: line, w: w - 2.0 * line, h: h - 2.0 * line }, radius - line, fill);
        let first = pen.fonts.layout(&words, &Style { weight: if bold { 700 } else { 600 }, ..Style::new(16.0 * k) }, None);
        let dots = pen.fonts.layout("\u{2026}", &Style::new(14.0 * k), None);
        let second = (!under.is_empty()).then(|| pen.fonts.layout(&under, &Style::new(14.0 * k), None).cut(w.max(0.0), &dots));
        let tall = first.height + second.as_ref().map_or(0.0, |s| s.height);
        let x = if centred { ((w - first.width) / 2.0).round() } else { 0.0 };
        let y = ((h - tall) / 2.0).round();
        pen.fonts.draw(canvas, &first, x, y, ink);
        if let Some(second) = second {
            pen.fonts.draw(canvas, &second, x, y + first.height, quiet);
        }
    })
}
