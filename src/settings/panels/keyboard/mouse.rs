//! Keyboard.

use std::time::Duration;

use super::{CLICKER, Keyboard, said};
use crate::common::files::{trindle_conf, update_env};
use crate::common::pointer::{self, Pointer, Range};
use crate::settings::ui::{group, line, on, Kit, Msg};
use crate::toolkit::colour::Colour;
use crate::toolkit::colour::paint as brush;
use crate::toolkit::t;
use crate::toolkit::text::{Align, Style};
use crate::toolkit::theme;
use crate::toolkit::ui::arc_to;
use crate::toolkit::widget::{choice, column, drawing, slider, switch, Length, Node};
use tiny_skia::{LineCap, PathBuilder, PixmapMut, Stroke, Transform};

impl Keyboard {
    fn slid(&self, which: Slide) -> (&'static str, f64, &'static Range, Option<&'static [f64; 3]>) {
        let p = &self.pointer;
        match which {
            Slide::Speed => (pointer::SPEED, p.speed, &pointer::SPEEDS, None),
            Slide::Scroll => (pointer::SCROLL, p.scroll, &pointer::SCROLLS, Some(&pointer::WHEEL_PACES)),
            Slide::TouchpadScroll => (pointer::TOUCHPAD_SCROLL, p.touchpad_scroll, &pointer::TOUCHPAD_SCROLLS,
                                      Some(&pointer::TOUCHPAD_PACES)),
        }
    }

    fn set_value(&mut self, which: Slide, value: f64) {
        let p = &mut self.pointer;
        match which {
            Slide::Speed => p.speed = value,
            Slide::Scroll => p.scroll = value,
            Slide::TouchpadScroll => p.touchpad_scroll = value,
        }
    }

    fn slide(&mut self, kit: &mut Kit, which: Slide, value: f32) {
        let value = value as f64;
        self.set_value(which, value);
        self.rounds[which.place()] += 1;
        let round = self.rounds[which.place()];
        kit.after::<Keyboard>(Duration::from_millis(SETTLE_MS), move |me, k| {
            if me.rounds[which.place()] == round {
                me.keep(k, which, value);
            }
        });
    }

    fn pick_pace(&mut self, kit: &mut Kit, which: Slide, value: f64) {
        self.set_value(which, value);
        self.rounds[which.place()] += 1;
        self.keep(kit, which, value);
    }

    fn keep(&mut self, kit: &mut Kit, which: Slide, value: f64) {
        let key = self.slid(which).0;
        let (value, touchpad) = (format!("{value:.2}"), format!("{:.2}", self.pointer.touchpad_scroll));
        let mut changes = vec![(key, Some(value.as_str()))];
        if which == Slide::Scroll {
            changes.push((pointer::TOUCHPAD_SCROLL, Some(touchpad.as_str())));
        }
        if !said(kit, key, update_env(&trindle_conf(), &changes)) {
            self.pointer = Pointer::load();
        }
    }

    fn slider_line(&self, kit: &Kit, which: Slide, title: &str, sub: Option<&str>) -> Node<Msg> {
        let (key, value, range, paces) = self.slid(which);
        let share = ((value - range.least) / (range.most - range.least)) as f32;
        let figure = paces.map_or_else(|| along(value, range), |p| times(value, p));
        let bar = slider(value as f32, range.least as f32, range.most as f32, range.step as f32, title, move |v| {
            on::<Keyboard>(move |me, k| me.slide(k, which, v))
        });
        let below = match paces {
            None => bar,
            Some(paces) => {
                let mut words: Vec<String> = pace_words().iter().map(|w| w.to_string()).collect();
                let chosen = pace(value, paces).unwrap_or_else(|| {
                    words.push(figure.clone());
                    pace_words().len()
                });
                let pick = choice(words, chosen, title, move |i| {
                    let to = paces.get(i).copied();
                    on::<Keyboard>(move |me, k| {
                        if let Some(to) = to {
                            me.pick_pace(k, which, to);
                        }
                    })
                }).width(Length::Shrink);
                column(vec![pick, bar]).gap(12.0)
            }
        };
        line(title).sub(sub.unwrap_or_default()).key(key)
            .control(gauge(SLIDER_RING, share, figure))
            .below(below)
            .node(kit)
    }

    pub(super) fn mouse_cards(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let p = &self.pointer;
        let both = self.touchpad.then_some(t!("settings-keyboard-both"));
        let mouse = group(t!("settings-keyboard-mouse-title")).icon("input-mouse-symbolic")
            .about(t!("settings-keyboard-mouse-about"))
            .rows(vec![
                self.slider_line(kit, Slide::Speed, t!("settings-keyboard-pointer-speed"), both),
                self.slider_line(kit, Slide::Scroll, t!("settings-keyboard-scroll-speed"), Some(t!("settings-keyboard-scroll-wheel-about"))),
                Self::switch_line(kit, pointer::MOUSE_NATURAL, p.mouse_natural, t!("settings-keyboard-natural"),
                                  t!("settings-keyboard-natural-wheel-about")),
                Self::switch_line(kit, pointer::LEFT_HANDED, p.left_handed, t!("settings-keyboard-left-handed"),
                                  t!("settings-keyboard-left-handed-about")),
                line(t!("settings-keyboard-clicker")).sub(t!("settings-keyboard-clicker-about")).key(CLICKER)
                    .control(switch(self.clicker, t!("settings-keyboard-clicker"), |b| on::<Keyboard>(move |me, k| me.clicking_helper(k, b))))
                    .node(kit),
            ])
            .node(kit);
        let mut cards = vec![mouse];
        if self.touchpad {
            cards.push(group(t!("settings-keyboard-touchpad-title")).icon("input-touchpad-symbolic")
                .about(t!("settings-keyboard-touchpad-about"))
                .rows(vec![
                    self.slider_line(kit, Slide::TouchpadScroll, t!("settings-keyboard-scroll-speed"), Some(t!("settings-keyboard-scroll-touchpad-about"))),
                    Self::switch_line(kit, pointer::TAP, p.tap, t!("settings-keyboard-tap"), t!("settings-keyboard-tap-about")),
                    Self::switch_line(kit, pointer::TOUCHPAD_NATURAL, p.touchpad_natural, t!("settings-keyboard-natural"),
                                      t!("settings-keyboard-natural-touchpad-about")),
                    Self::switch_line(kit, pointer::QUIET_TYPING, p.quiet_typing, t!("settings-keyboard-quiet"),
                                      t!("settings-keyboard-quiet-about")),
                ])
                .node(kit));
        }
        cards
    }

}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Slide {
    Speed,
    Scroll,
    TouchpadScroll,
}

impl Slide {
    pub(super) fn place(self) -> usize {
        self as usize
    }
}

fn pace_words() -> [&'static str; 3] {
    [t!("settings-keyboard-pace-slower"), t!("settings-keyboard-pace-normal"), t!("settings-keyboard-pace-faster")]
}

fn along(value: f64, range: &Range) -> String {
    format!("{:.0}%", ((value - range.least) / (range.most - range.least)).clamp(0.0, 1.0) * 100.0)
}

pub(super) fn times(value: f64, paces: &[f64; 3]) -> String {
    let value = value / paces[1];
    let shown = trindle_toolkit::lang::format::decimal(value, 2);
    format!("{}\u{d7}", shown.trim_end_matches('0').trim_end_matches(t!("kit-number-decimal")))
}

fn pace(value: f64, paces: &[f64; 3]) -> Option<usize> {
    paces.iter().position(|p| (p - value).abs() < 0.001)
}

const SLIDER_RING: f32 = 60.0;

const SETTLE_MS: u64 = 300;

fn gauge<M>(size: f32, share: f32, text: String) -> Node<M> {
    drawing((size, size), "", move |canvas, pen| {
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let width = size / 8.4 * pen.scale;
        let (c, radius) = ((w / 2.0, h / 2.0), (w.min(h) - width) / 2.0);
        let stroke = |canvas: &mut PixmapMut, from: f32, to: f32, colour: Colour, cap: LineCap| {
            let mut pb = PathBuilder::new();
            arc_to(&mut pb, c, radius, from, to, true);
            if let Some(path) = pb.finish() {
                canvas.stroke_path(&path, &brush(colour), &Stroke { width, line_cap: cap, ..Stroke::default() },
                                   Transform::identity(), None);
            }
        };
        let top = -std::f32::consts::FRAC_PI_2;
        stroke(canvas, top, top + std::f32::consts::TAU, theme::alpha(pen.theme.fg, 0.10), LineCap::Butt);
        let sweep = share.clamp(0.0, 1.0) * std::f32::consts::TAU;
        if sweep > 0.001 {
            stroke(canvas, top, top + sweep, pen.theme.soft, LineCap::Round);
        }
        let size = (w * 0.21 / pen.scale).clamp(11.0, 40.0) * pen.scale;
        let style = Style { weight: 700, align: Align::Centre, ..Style::new(size) };
        let layout = pen.fonts.layout(&text, &style, Some(w));
        pen.fonts.draw(canvas, &layout, 0.0, (h - layout.height) / 2.0, pen.theme.fg);
    })
}
