//! One notification circle as a toolkit `App`: what it shows, the keys and pointer it takes and how it moves.

use std::rc::Rc;
use std::time::Duration;

use crate::ring::circles::{self, Act, Sizes, Spec};
use crate::toolkit::theme;
use crate::toolkit::input::{Button, Input, Keysym};
use crate::toolkit::ui::App;
use crate::toolkit::widget::{drawing, layers, Length, Node, placed};

use super::{Card, Content, Msg, Notes};
use super::icons::{FALLBACK_ICON, ICON_PX};

pub(super) const FRAME: u64 = 16;

pub(super) const WHEEL_LINES: i32 = 2;

const POINTS_A_LINE: f32 = 16.0;

pub(super) const COPIED_FOR: u64 = 2000;

pub(super) const PAGE_LINES: i32 = 4;

impl Content {

    pub(super) fn shown(&self) -> &str {
        if self.why.is_empty() { &self.body } else { &self.why }
    }

    pub(super) fn extra(&self) -> Option<Act> {
        if !self.drive.is_empty() {
            Some(Act::Drive)
        } else if !self.allow.is_empty() {
            Some(Act::Allow)
        } else {
            None
        }
    }

    pub(super) fn whole(&self) -> String {
        if self.body.is_empty() { self.title.clone() } else { format!("{}\n{}", self.title, self.body) }
    }

    pub(super) fn said(&self) -> String {
        let body = self.shown();
        if body.is_empty() { self.title.clone() } else { format!("{}. {}", self.title, body) }
    }
}

impl Card {

    fn later(&self, f: impl FnOnce(&Rc<Notes>, u32) + 'static) {
        self.later_after(Duration::ZERO, f);
    }

    fn later_after(&self, wait: Duration, f: impl FnOnce(&Rc<Notes>, u32) + 'static) {
        let (n, id) = (self.notes.clone(), self.content.id);
        self.lp.after(wait, move || {
            if let Some(n) = n.upgrade() {
                f(&n, id);
            }
        });
    }

    fn hold_begins(&self) {
        let token = self.press.get().wrapping_add(1);
        self.press.set(token);
        let press = self.press.clone();
        let wait = Duration::from_millis(theme::sheet().circle_hold as u64);
        self.later_after(wait, move |n, id| {
            if press.get() == token {
                n.pick_up(id);
            }
        });
    }

    fn hold_ends(&self) {
        self.press.set(self.press.get().wrapping_add(1));
    }

    pub(super) fn sizes(&self) -> Sizes {
        Sizes { across: self.look.across, ..Sizes::sheet() }
    }

    fn part_at(&self, x: f32, y: f32) -> Option<Act> {
        circles::pressed(self.look.face, self.look.buttons, self.sizes(), x, y).filter(|a| *a != Act::Open)
    }

    fn on_circle(&self, x: f32, y: f32) -> bool {
        let r = self.look.across / 2.0;
        (x - r).hypot(y - r) <= r
    }

    fn in_square(&self, x: f32, y: f32) -> bool {
        (0.0..=self.look.across).contains(&x) && (0.0..=self.look.across).contains(&y)
    }

    fn moved_to(&self, x: f32, y: f32) -> Option<Msg> {
        let inside = if self.inside.get() { self.in_square(x, y) } else { self.on_circle(x, y) };
        let hot = if inside { self.part_at(x, y) } else { None };

        let (was_inside, was_on) = (self.inside.replace(inside), self.on.replace(hot));
        if inside == was_inside && hot == was_on {
            return None;
        }
        Some(Msg::At { inside, hot })
    }
}

impl App for Card {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::At { inside, hot } => {
                self.look.hot = hot;
                self.later(move |n, id| n.point(id, inside));
            }
            Msg::Act(act) => self.later(move |n, id| n.act(id, act)),
            Msg::Scroll(lines) => self.later(move |n, id| n.scroll(id, lines)),
            Msg::Leave => self.later(|n, _| n.leave()),
            Msg::Drag { x, y } => self.later(move |n, id| n.drag(id, (x, y))),
            Msg::Drop => self.later(|n, id| n.drop_held(id)),
            Msg::PutBack => self.later(|n, id| n.put_back(id)),
            Msg::Nudge { right, down } => self.later(move |n, id| n.nudge(id, right, down)),
            Msg::Placed => self.later(|n, id| n.placed(id)),
        }
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::PointerIn { x, y } | Input::PointerMove { x, y } | Input::Press { x, y, .. } | Input::Release { x, y, .. } => self.last.set((*x, *y)),
            _ => {}
        }
        if matches!(input, Input::Placed) {
            return Some(Msg::Placed);
        }

        if self.look.moving {
            return match input {
                Input::PointerIn { x, y } | Input::PointerMove { x, y } => Some(Msg::Drag { x: *x, y: *y }),
                Input::Release { button: Button::Left, .. } => Some(Msg::Drop),
                Input::Press { .. } | Input::Release { .. } | Input::Scroll { .. } => Some(Msg::Drag { x: self.last.get().0, y: self.last.get().1 }),
                Input::Key { sym: Keysym::Escape, .. } => Some(Msg::PutBack),
                _ => None,
            };
        }
        match input {
            Input::Press { button: Button::Left, .. } => self.hold_begins(),
            Input::Press { .. } | Input::Release { .. } | Input::PointerOut => self.hold_ends(),
            _ => {}
        }
        match input {
            Input::Key { sym, mods, .. } if mods.alt && !mods.ctrl => {
                let (right, down) = match *sym {
                    Keysym::Left | Keysym::KP_Left => (-1, 0),
                    Keysym::Right | Keysym::KP_Right => (1, 0),
                    Keysym::Up | Keysym::KP_Up => (0, -1),
                    Keysym::Down | Keysym::KP_Down => (0, 1),
                    _ => return None,
                };
                Some(Msg::Nudge { right, down })
            }
            Input::PointerIn { x, y } | Input::PointerMove { x, y } => self.moved_to(*x, *y),
            Input::PointerOut => {
                self.on.set(None);
                self.inside.replace(false).then_some(Msg::At { inside: false, hot: None })
            }

            Input::Key { sym, mods, .. } => match *sym {
                Keysym::Down | Keysym::KP_Down => Some(Msg::Scroll(1)),
                Keysym::Up | Keysym::KP_Up => Some(Msg::Scroll(-1)),
                Keysym::Page_Down | Keysym::KP_Page_Down => Some(Msg::Scroll(PAGE_LINES)),
                Keysym::Page_Up | Keysym::KP_Page_Up => Some(Msg::Scroll(-PAGE_LINES)),
                Keysym::c | Keysym::C if mods.ctrl && !mods.alt => Some(Msg::Act(Act::Copy)),
                _ => None,
            },
            Input::Scroll { y, steps, .. } => {
                let lines = if *steps { if *y == 0.0 { 0 } else { y.signum() as i32 * WHEEL_LINES * (y.abs().round() as i32).max(1) } } else { (y / POINTS_A_LINE).round() as i32 };
                (lines != 0).then_some(Msg::Scroll(lines))
            }
            _ => None,
        }
    }

    fn view(&self) -> Node<Msg> {
        let (c, l) = (self.content.clone(), self.look.clone());
        let side = l.across;
        let (said, whole) = (c.said(), c.whole());
        let parts = circles::parts(l.face, l.buttons, self.sizes());
        let names: Vec<(Act, String)> = parts.iter().filter(|p| p.act != Act::Open)
            .map(|p| (p.act, p.act.name(l.pinned, l.copied))).collect();
        let painted = {
            let l = l.clone();
            move |canvas: &mut tiny_skia::PixmapMut, pen: &crate::toolkit::widget::Pen| {
                let px = (ICON_PX as f32 * pen.scale).round() as u32;
                let icon = c.icons.iter().map(String::as_str).find(|n| pen.icons.has(n, px, 1)).unwrap_or(FALLBACK_ICON);
                circles::paint_circle(canvas, pen, (0.0, 0.0), &Spec {
                    across: l.across, scroll: l.scroll, icon, title: &c.title, body: c.shown(), face: l.face, buttons: l.buttons,
                    hover: l.hover, pinned: l.pinned, hot: l.hot, copied: l.copied, moving: l.moving,
                });
            }
        };

        let mut items = vec![
            drawing((side, side), said, painted).width(Length::Fill).height(Length::Fill)
                .pressable(Msg::Act(Act::Open)).on_keys(|_| None).named(true).copies(whole).key("circle-message"),
        ];

        for (i, p) in parts.iter().filter(|p| p.act != Act::Open).enumerate() {
            let name = names.iter().find(|(a, _)| *a == p.act).map(|(_, n)| n.clone()).unwrap_or_default();
            let mut part = drawing((p.at.w, p.at.h), name, |_, _| {}).pressable(Msg::Act(p.act)).on_keys(|_| None).key(format!("circle-{:?}", p.act));
            if i == 0 && self.grab > 0 {
                part = part.grab(self.grab);
            }
            let (x, y) = (p.at.x, p.at.y);
            items.push(placed(part, move |_, _| (x, y)));
        }
        layers(items).on_escape(Msg::Leave)
    }
}
