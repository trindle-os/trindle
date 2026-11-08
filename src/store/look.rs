//! The small pieces Apps' Installed, Updates and app pages share.

use crate::toolkit::t;
use std::sync::OnceLock;
use std::time::Instant;

use tiny_skia::{FillRule, Paint, PathBuilder, Stroke, Transform};

use super::{go, Msg, Store};
use crate::toolkit::colour::Colour;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, drawing, entry, label, note, ring, row, Length, Node, Pen, Tone};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dot {

    UpToDate,

    Waiting,

    Filling(f32),

    Ready,

    Checking,

    Failed,

    Quiet,
}

impl Dot {
    fn pulses(self) -> bool {
        matches!(self, Dot::Waiting | Dot::Checking)
    }
}

pub fn dot_size() -> f32 {
    theme::sheet().state_dot
}

const PULSE_MS: u128 = 1000;

const PULSE_LOW: f32 = 0.35;

pub const PULSE_FRAME: std::time::Duration = std::time::Duration::from_millis(60);

fn born() -> Instant {
    static AT: OnceLock<Instant> = OnceLock::new();
    *AT.get_or_init(Instant::now)
}

fn pulse_now() -> f32 {
    if !crate::common::graphics::moving() {
        return 1.0;
    }
    let along = (born().elapsed().as_millis() % PULSE_MS) as f32 / PULSE_MS as f32;
    let k = 0.5 - 0.5 * (along * std::f32::consts::TAU).cos();
    1.0 - (1.0 - PULSE_LOW) * k
}

fn colour_of(dot: Dot, pen: &Pen) -> Colour {
    match dot {
        Dot::UpToDate => theme::quiet(),
        Dot::Waiting | Dot::Filling(_) => pen.theme.soft,
        Dot::Ready | Dot::Checking => theme::calm(),
        Dot::Failed => theme::problem(),
        Dot::Quiet => theme::quiet(),
    }
}

fn solid(c: Colour, alpha: f32) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color(theme::alpha(c, alpha).to_skia());
    p.anti_alias = true;
    p
}

pub fn dot(s: &Store, d: Dot) -> Node<Msg> {
    dot_marking(&s.pulsing, d)
}

pub fn dot_named(s: &Store, d: Dot, says: &str) -> Node<Msg> {
    made(&s.pulsing, d, says)
}

pub fn dot_marking(pulsing: &std::cell::Cell<bool>, d: Dot) -> Node<Msg> {
    made(pulsing, d, "")
}

fn made(pulsing: &std::cell::Cell<bool>, d: Dot, says: &str) -> Node<Msg> {
    if d.pulses() {
        pulsing.set(true);
    }
    #[cfg(test)]
    let named = !says.is_empty();
    let dot = drawing((dot_size(), dot_size()), says, move |canvas, pen| {
        let side = canvas.width().min(canvas.height()) as f32;
        let (c, r) = (side / 2.0, side / 2.0);
        let colour = colour_of(d, pen);
        #[cfg(test)]
        theme::drawn::dot(canvas, colour, named);
        match d {
            Dot::Filling(share) => {
                let line = (side / 6.0).max(2.0);
                let radius = r - line / 2.0;
                let stroke = Stroke { width: line, line_cap: tiny_skia::LineCap::Round, ..Stroke::default() };
                if let Some(track) = PathBuilder::from_circle(c, c, radius) {
                    canvas.stroke_path(&track, &solid(theme::quiet(), 0.45), &stroke, Transform::identity(), None);
                }
                let sweep = share.clamp(0.0, 1.0) * std::f32::consts::TAU;
                if sweep > 0.001 {
                    let mut pb = PathBuilder::new();
                    let top = -std::f32::consts::FRAC_PI_2;
                    crate::toolkit::ui::arc_to(&mut pb, (c, c), radius, top, top + sweep, true);
                    if let Some(arc) = pb.finish() {
                        canvas.stroke_path(&arc, &solid(colour, 1.0), &stroke, Transform::identity(), None);
                    }
                }
            }
            _ => {
                let strength = if d.pulses() { pulse_now() } else { 1.0 };
                if let Some(path) = PathBuilder::from_circle(c, c, r - 0.5) {
                    canvas.fill_path(&path, &solid(colour, strength), FillRule::Winding, Transform::identity(), None);
                }
            }
        }
    });

    if d.pulses() { dot.alive() } else { dot }
}

pub fn dot_line(s: &Store, d: Dot, words: impl Into<String>) -> Node<Msg> {
    row(vec![dot(s, d), note(words.into())]).gap(8.0).centred().width(Length::Shrink)
}

pub fn status_line(s: &Store, d: Dot, words: impl Into<String>, page: &'static str) -> Node<Msg> {
    let words = words.into();
    if d != Dot::Failed {
        return dot_line(s, d, words);
    }
    row(vec![dot(s, d), note(words), crate::common::more::button(super::pages::links::more_about(page))]).gap(8.0).centred().width(Length::Shrink)
}

pub fn tabs(names: &[(&str, usize)], chosen: usize, room: f32, pick: impl Fn(usize) -> Msg + 'static) -> Node<Msg> {
    let words: Vec<String> = names.iter().map(|(name, n)| t!("store-tab-count", name = *name, n = *n)).collect();
    if strip_room(&words) <= room {
        crate::toolkit::widget::choice(words, chosen, t!("store-tabs-show"), pick).width(Length::Shrink)
    } else {
        crate::toolkit::widget::dropdown(words, Some(chosen), t!("store-tabs-show"), pick)
    }
}

pub fn strip_room(words: &[String]) -> f32 {
    let k = theme::sheet();
    let wide = |c: char| if c.is_ascii_digit() { FIGURE } else { LETTER };
    let letters: f32 = words.iter().flat_map(|w| w.chars()).map(wide).sum();
    letters * k.text.0 + words.len() as f32 * 2.0 * k.entry_padding + 2.0 * k.tabs_padding
}

const LETTER: f32 = 0.5;
const FIGURE: f32 = 0.66;

pub fn search(text: &str, tip: &str, change: impl Fn(String) -> Msg + 'static) -> Node<Msg> {
    entry(text.to_string(), tip, change).width(Length::Fill)
}

pub fn back(to: &str, press: Msg) -> Node<Msg> {
    let words = to.to_string();
    card(row(vec![super::pages::pictures::icon("go-previous-symbolic".into(), 18.0), label(words)]).gap(8.0).centred())
        .flat().width(Length::Shrink).on_press(press)
}

pub fn ring_with_figure(share: f32, size: f32) -> Node<Msg> {
    let percent = (share.clamp(0.0, 1.0) * 100.0).floor();
    row(vec![ring(percent / 100.0, size).tone(Tone::Accent), note(format!("{percent:.0}%")).width(Length::Fixed(FIGURE_ROOM))])
        .gap(8.0).centred().width(Length::Shrink)
}

const FIGURE_ROOM: f32 = 40.0;

pub fn remove(_s: &Store, key: &str, name: &str, then: impl Fn(&mut Store) + 'static) -> Node<Msg> {
    crate::toolkit::widget::remove(REMOVE_ICON, name).key(key.to_string()).spoken(name.to_string()).on_press(go(move |s| then(s)))
}

pub const REMOVE_ICON: &str = "user-trash-symbolic";

pub fn link(words: &str, press: Msg) -> Node<Msg> {
    card(note(words.to_string())).flat().width(Length::Shrink).on_press(press)
}

pub fn about_button(tip: &str) -> Node<Msg> {
    button("dialog-information-symbolic", tip).face(|canvas, r, ink, _pen| {
        let paint = solid(ink, 1.0);
        let w = r.w;
        let stem = (w * 0.14).max(1.5);
        let cx = r.x + w / 2.0;
        if let Some(dot) = PathBuilder::from_circle(cx, r.y + w * 0.2, stem * 0.75) {
            canvas.fill_path(&dot, &paint, FillRule::Winding, Transform::identity(), None);
        }
        if let Some(bar) = tiny_skia::Rect::from_xywh(cx - stem / 2.0, r.y + w * 0.38, stem, w * 0.5) {
            canvas.fill_rect(bar, &paint, Transform::identity(), None);
        }
    })
}

pub fn open_button(tip: &str) -> Node<Msg> {
    button("media-playback-start-symbolic", tip).face(|canvas, r, ink, _pen| {
        let w = r.w;
        let line = (w * 0.1).max(1.5);
        let mut pb = PathBuilder::new();
        pb.move_to(r.x + w * 0.3, r.y + w * 0.2);
        pb.line_to(r.x + w * 0.8, r.y + w * 0.5);
        pb.line_to(r.x + w * 0.3, r.y + w * 0.8);
        pb.close();
        if let Some(path) = pb.finish() {
            let stroke = Stroke { width: line, line_join: tiny_skia::LineJoin::Round, ..Stroke::default() };
            canvas.stroke_path(&path, &solid(ink, 1.0), &stroke, Transform::identity(), None);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pulse_stays_between_its_quietest_and_full() {
        for _ in 0..5 {
            let p = pulse_now();
            assert!((PULSE_LOW - 0.001..=1.0).contains(&p), "{p}");
        }
    }

    #[test]
    fn only_waiting_and_checking_pulse() {
        assert!(Dot::Waiting.pulses());
        assert!(Dot::Checking.pulses());
        for d in [Dot::UpToDate, Dot::Filling(0.4), Dot::Ready, Dot::Failed, Dot::Quiet] {
            assert!(!d.pulses(), "{d:?}");
        }
    }
}
