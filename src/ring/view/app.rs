//! The ring as a toolkit `App`: its messages, the keys it takes and the view it builds.

use crate::ring::{kept, notice};
use crate::ring::ctx::{msg, Msg};
use crate::ring::layout::{button_size, CENTRE};
use crate::ring::notice::Line;
use crate::toolkit::t;
use crate::toolkit::input::{Input, Keysym, Typing};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{card, layers, Length, Node, placed, Rect};

use super::{ALL, Face, REFUSED_SECONDS};
use super::app_tile::app_button;
use super::arrange::{less_word, quick_side};
use super::middle::FINDING;
use super::paint::disc;
use super::power::power_closed;

fn nothing_found() -> &'static str {
    t!("ring-find-nothing")
}

pub(super) fn look() -> Theme {
    Theme::chosen()
}

impl App for Face {
    type Msg = Msg;

    fn update(&mut self, m: Msg) {
        (m.0)(self);
    }

    fn unchanged(&self, m: &Msg) -> bool {
        m.is_moot(self)
    }

    fn pressed(&mut self, p: Rect) {
        self.pressed_at = [p.x, p.y, p.w, p.h];
    }

    fn blend(&self) -> Option<(u64, f32)> {
        Some((self.level_turn, self.level_k))
    }

    fn resized(&mut self, w: f32, h: f32) -> bool {
        let size = (w as f64, h as f64);
        if size == self.size || w < 1.0 || h < 1.0 {
            return false;
        }
        self.size = size;
        self.relayout();
        true
    }

    fn intercept(&self, input: &Input, keys: bool) -> Option<Msg> {
        if matches!(input, Input::Key { sym: Keysym::Escape, .. }) {
            return Some(msg(|f| f.escape()));
        }

        if let Input::Key { sym, mods, repeat: false, .. } = input {
            if self.list_keys.as_ref().is_some_and(|c| crate::common::key_list::pressed(c, *sym, *mods)) {
                return Some(msg(|f| f.toggle_shortcuts(true)));
            }
        }

        if self.power && matches!(input, Input::Key { .. }) && !kept::own_press() {
            return Some(msg(|f| {
                f.show_power(false);
                f.say(power_closed(), REFUSED_SECONDS);
            }));
        }
        if self.quick.is_open() || self.power || self.menu.is_some() {
            return None;
        }
        self.typed_key(input, keys)
    }

    fn typing(&self) -> Option<Typing> {
        if self.level != ALL || self.quick.is_open() || self.power || self.menu.is_some() {
            return None;
        }
        let (w, h) = (self.size.0 as i32, self.size.1 as i32);
        let (fw, fh) = (FINDING.0 as i32, FINDING.1 as i32);
        Some(Typing { id: "ring-find".into(), text: self.typed.clone(), caret: self.typed.len(), anchor: self.typed.len(),
                      area: ((w - fw) / 2, (h - fh) / 2, fw, fh), secret: false, purpose: Default::default() })
    }

    fn view(&self) -> Node<Msg> {
        let (w, h) = (self.size.0 as f32, self.size.1 as f32);

        if let Some(ws) = &self.windows {
            ws.forget_closes();
        }
        let open = self.open_ring();
        let pointed = self.middle_picture(open.as_ref().map(|o| o.1.as_slice()));
        let hiding = pointed.is_some();
        let mut all_layers = vec![self.backdrop(hiding)];
        if let Some((ring, wins)) = &open {
            all_layers.extend(self.windows_ring(ring, wins));
        }
        let (cover, mid) = (self.covered(), (w / 2.0, h / 2.0));
        let mut under = vec![(mid, in_middle(pointed.unwrap_or_else(|| self.centre())))];
        self.app_layers(&mut under);
        if !hiding {
            under.push((mid, self.segment(mid.0, mid.1)));
            under.extend(self.top_segment(mid.0, mid.1).map(|n| (mid, n)));
        }
        all_layers.extend(uncovered(cover, mid, under));
        self.quick_layers(&mut all_layers);
        self.menu_layers(&mut all_layers);
        if self.power {
            all_layers.push(in_middle(disc(MIDDLE_COVER, |t| t.raised)));
            all_layers.push(in_middle(self.power_cover()));
        }
        all_layers.extend(self.words_layers());
        layers(all_layers).width(Length::Fixed(w)).height(Length::Fixed(h)).spoken("trindle-ring")
    }
}

pub(super) const MIDDLE_COVER: f32 = (CENTRE + 2.0) as f32;

pub(super) fn uncovered<N>(cover: Option<f32>, mid: (f32, f32), under: Vec<((f32, f32), N)>) -> impl Iterator<Item = N> {
    under.into_iter().filter(move |((x, y), _)| cover.is_none_or(|r| (x - mid.0).hypot(y - mid.1) >= r)).map(|(_, n)| n)
}

fn in_middle(n: Node<Msg>) -> Node<Msg> {
    placed(n, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0))
}

fn beside(n: Node<Msg>, r: [f32; 4]) -> Node<Msg> {
    placed(n, move |(mw, mh), (aw, ah)| ((r[0] + r[2] + 8.0).min(aw - mw - 8.0), r[1].min(ah - mh - 8.0)))
}

impl Face {

    fn typed_key(&self, input: &Input, keys: bool) -> Option<Msg> {
        let finding = self.level == ALL;
        match input {
            Input::Composed { commit, .. } if finding && !commit.is_empty() => {
                let more = commit.clone();
                Some(msg(move |f| f.type_more(&more)))
            }
            Input::Key { sym, text, mods, .. } => {
                if mods.alt || mods.logo {
                    return None;
                }
                if !finding {
                    if mods.ctrl {
                        return None;
                    }
                    let c = text.as_ref().and_then(|t| t.chars().next()).filter(|c| c.is_alphanumeric())?;
                    return Some(msg(move |f| {
                        f.set_level(ALL);
                        f.set_typed(c.to_string());
                    }));
                }
                match *sym {
                    Keysym::BackSpace if mods.ctrl => Some(msg(|f| f.set_typed(less_word(&f.typed)))),
                    Keysym::BackSpace => Some(msg(|f| {
                        let mut t = f.typed.clone();
                        t.pop();
                        f.set_typed(t);
                    })),

                    Keysym::Return | Keysym::KP_Enter | Keysym::space if keys => None,
                    Keysym::Return | Keysym::KP_Enter => Some(msg(|f| if let Some(i) = f.best_match { f.activate(i) })),
                    Keysym::space if self.typed.is_empty() => None,
                    _ if mods.ctrl => None,
                    _ => {
                        let t = text.as_ref().filter(|t| !t.is_empty() && !t.chars().any(char::is_control))?.clone();
                        Some(msg(move |f| f.type_more(&t)))
                    }
                }
            }
            _ => None,
        }
    }

    fn app_layers(&self, out: &mut Vec<((f32, f32), Node<Msg>)>) {
        let lay = &self.lay;
        let names = self.all();
        let best = self.best_match;
        for p in lay.placed.iter() {
            let (bw, bh) = button_size(lay.icon, lay.names);
            let at = [p.x as f32 - bw as f32 / 2.0, p.y as f32 - bh as f32 / 2.0, bw as f32, bh as f32];
            let Some(app) = names.get(p.item) else { continue };
            let node = app_button(app, p.item, lay.icon, lay.names, Some(p.item) == best, Some(p.item) == self.hover_app, at, self.ink_of(&app.id));
            out.push(((p.x as f32, p.y as f32), placed(node, move |_, _| (at[0], at[1]))));
        }
    }

    pub(super) fn covered(&self) -> Option<f32> {
        if self.power || self.quick.is_tray() {
            Some(MIDDLE_COVER / 2.0)
        } else if self.quick.is_open() {
            Some(quick_side(self.lay.disc_radius) as f32 / 2.0)
        } else {
            None
        }
    }

    fn quick_layers(&self, out: &mut Vec<Node<Msg>>) {
        if self.quick.is_tray() {
            out.push(in_middle(disc(MIDDLE_COVER, |t| t.raised)));
            out.push(in_middle(super::middle::tray_cover()));
        } else if self.quick.is_open() {
            let q = quick_side(self.lay.disc_radius) as f32;
            out.push(in_middle(disc(q, |t| t.raised)));
            if self.quick.asking() {
                out.push(in_middle(disc(self.quick.pop_size(q as i32) as f32, |t| t.raised_more)));
            }
            out.push(in_middle(self.quick.view()));
        }
    }

    fn menu_layers(&self, out: &mut Vec<Node<Msg>>) {
        if let Some((_, r, make)) = &self.beside {
            out.push(beside(card(make()).width(Length::Shrink).floats(), *r));
        }
        if let Some((h, r)) = self.win_menu.clone() {
            out.push(beside(self.window_menu_card(&h).floats(), r));
        }
        if let Some((i, r)) = self.menu {
            out.push(beside(self.menu_card(i).floats(), r));
        }
    }

    fn words_layers(&self) -> Vec<Node<Msg>> {
        let mut lines = Vec::new();
        if self.level == ALL && !self.typed.trim().is_empty() && self.best_match.is_none() {
            lines.push(Line::new(nothing_found()));
        }
        if let Some(said) = self.said {
            lines.push(Line::problem(said.words()));
        }
        if let Some(words) = self.words.clone() {
            lines.push(Line::new(words));
        }
        notice::words_outside(&lines, (self.size.0 as f32, self.size.1 as f32), self.lay.outer_radius as f32)
    }
}
