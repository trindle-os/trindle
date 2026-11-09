//! What the pointer and the keyboard do in the room: pointing at a bubble or note, picking one, and reading it out.

use std::f64::consts::{FRAC_PI_2, TAU};
use std::rc::Rc;

use crate::activity::reading::busy_words;
use crate::toolkit::t;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::ui::{tip_rest, tip_stays};
use crate::toolkit::widget::{Pointer, Rect};

use super::{Orbit, Under};
use super::motion::{room, thread_at};
use super::paint::BAR_FULL;

const CARD_GAP: f64 = 14.0;

impl Orbit {

    pub(super) fn under(&self, x: f64, y: f64) -> Option<Under> {
        if let Some((_, i)) = self.notes_at.borrow().iter().find(|(r, _)| r.contains(x as f32, y as f32)) {
            return Some(Under::Note(*i));
        }
        let g = self.room();
        let bubbles = self.bubbles.borrow();
        if let Some(b) = bubbles.iter().rev().filter(|b| !b.leaving).find(|b| {
            (x - (g.cx + b.x * g.radius)).hypot(y - (g.cy + b.y * g.radius)) <= b.r * g.radius
        }) {
            return Some(Under::Bubble(b.key.clone()));
        }
        thread_at(&g, x, y, self.threads.borrow().len()).map(Under::Thread)
    }

    pub fn pointer(&self, p: Pointer) -> Option<Option<String>> {
        match p {
            Pointer::Down(x, y) => {

                if self.keyed.borrow_mut().take().is_some() {
                    self.redraw();
                }
                *self.down.borrow_mut() = self.under(x as f64, y as f64);
                None
            }
            Pointer::Up(x, y) => {
                let down = self.down.borrow_mut().take();
                let here = self.under(x as f64, y as f64);
                if down != here {
                    return None;
                }
                let key = match here {
                    Some(Under::Bubble(k)) => (self.picked().as_deref() != Some(k.as_str())).then_some(k),
                    Some(Under::Note(i)) => self.notes.borrow().get(i).map(|n| n.key.clone()),
                    _ => None,
                };
                if self.picked() == key {
                    return None;
                }
                self.pick(key.as_deref());
                Some(key)
            }
            Pointer::Hover(x, y) => {
                let here = self.under(x as f64, y as f64);
                if *self.hover.borrow() != here {
                    self.name(here.clone());
                    *self.hover.borrow_mut() = here;
                    self.redraw();
                }
                None
            }
            Pointer::Leave => {
                *self.hover.borrow_mut() = None;
                *self.down.borrow_mut() = None;
                *self.tip.borrow_mut() = None;

                self.stamp.set(self.stamp.get() + 1);
                self.redraw();
                None
            }
            Pointer::Move(..) | Pointer::Menu(..) | Pointer::Middle(..) | Pointer::RightDown(..) | Pointer::RightMove(..) | Pointer::RightUp(..) => None,
        }
    }

    fn keyable(&self) -> Vec<String> {
        self.bubbles.borrow().iter().filter(|b| !b.leaving).map(|b| b.key.clone()).collect()
    }

    pub fn key(&self, input: &Input) -> Option<bool> {
        let keys = self.keyable();
        let at = self.keyed.borrow().as_ref().and_then(|k| keys.iter().position(|x| x == k));
        let go = |to: Option<usize>| -> Option<bool> {
            let key = to.and_then(|i| keys.get(i)).cloned();
            *self.keyed.borrow_mut() = key.clone();
            if let Some(k) = key {
                self.name(Some(Under::Bubble(k)));
            }
            self.redraw();
            Some(false)
        };
        match input {
            Input::Focus(true) => {
                let picked = self.picked().filter(|p| keys.contains(p));
                let start = picked.and_then(|p| keys.iter().position(|x| *x == p)).or((!keys.is_empty()).then_some(0));

                *self.keyed.borrow_mut() = start.and_then(|i| keys.get(i)).cloned();
                self.redraw();
                Some(false)
            }
            Input::Focus(false) => {
                *self.keyed.borrow_mut() = None;
                *self.tip.borrow_mut() = None;
                self.redraw();
                Some(false)
            }
            Input::Key { sym, mods, .. } => {
                let back = *sym == Keysym::ISO_Left_Tab || (*sym == Keysym::Tab && mods.shift);
                let next = *sym == Keysym::Tab && !back;
                match *sym {
                    _ if next || matches!(*sym, Keysym::Right | Keysym::Down) => {
                        let to = at.map_or(0, |i| i + 1);
                        if to >= keys.len() {
                            return if next { None } else { Some(false) };
                        }
                        go(Some(to))
                    }
                    _ if back || matches!(*sym, Keysym::Left | Keysym::Up) => match at {
                        Some(i) if i > 0 => go(Some(i - 1)),
                        None if !back && !keys.is_empty() => go(Some(0)),
                        _ if back => None,
                        _ => Some(false),
                    },
                    Keysym::Return | Keysym::KP_Enter | Keysym::space | Keysym::KP_Space => self.press(),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn press(&self) -> Option<bool> {
        let keyed = self.keyed.borrow().clone().or_else(|| self.keyable().into_iter().next())?;
        let to = (self.picked().as_deref() != Some(keyed.as_str())).then_some(keyed);
        self.pick(to.as_deref());
        Some(true)
    }

    pub fn spoken(&self) -> String {
        let keyed = self.keyed.borrow().clone();
        let bubbles = self.bubbles.borrow();
        match keyed.and_then(|k| bubbles.iter().find(|b| b.key == k)) {
            Some(b) if self.picked().as_deref() == Some(b.key.as_str()) => t!("activity-spoken-open", title = &b.title, figure = &b.figure),
            Some(b) => t!("activity-spoken", title = &b.title, figure = &b.figure),
            None => t!("activity-room-spoken").into(),
        }
    }

    fn name(&self, under: Option<Under>) {
        let g = self.room();
        let tip = match under {
            Some(Under::Thread(i)) => {
                let n = self.threads.borrow().len().max(1) as f64;
                let b = self.threads.borrow().get(i).map_or(0.0, |t| t.1);
                let a = -FRAC_PI_2 + (i as f64 + 0.5) * TAU / n;
                let d = g.rim + BAR_FULL / 2.0;
                let (x, y) = (g.cx + a.cos() * d, g.cy + a.sin() * d);
                let half = BAR_FULL / 2.0;
                Some((t!("activity-thread-tip", n = i + 1, busy = busy_words(b)),
                      Rect { x: (x - half) as f32, y: (y - half) as f32, w: BAR_FULL as f32, h: BAR_FULL as f32 }))
            }
            Some(Under::Bubble(k)) => self.bubbles.borrow().iter().find(|b| b.key == k).map(|b| {
                let (px, py, rp) = (g.cx + b.x * g.radius, g.cy + b.y * g.radius, b.r * g.radius);
                (b.title.clone(), Rect { x: (px - rp) as f32, y: (py - rp) as f32, w: (2.0 * rp) as f32, h: (2.0 * rp) as f32 })
            }),
            Some(Under::Note(i)) => {
                let at = self.notes_at.borrow().iter().find(|(_, n)| *n == i).map(|(r, _)| *r);
                let name = self.notes.borrow().get(i).map(|n| n.name.clone());
                at.zip(name).map(|(r, name)| (t!("activity-more-about", name = &name), r))
            }
            None => None,
        };

        *self.tip.borrow_mut() = None;
        let stamp = self.stamp.get() + 1;
        self.stamp.set(stamp);
        let Some(tip) = tip else { return };
        let weak = self.me.borrow().clone();
        self.events.after(tip_rest(), move || {
            let Some(o) = weak.upgrade() else { return };
            if o.stamp.get() != stamp {
                return;
            }
            *o.tip.borrow_mut() = Some(tip);
            o.redraw();
            if o.stays {
                return;
            }
            let weak = Rc::downgrade(&o);
            o.events.after(tip_stays(), move || {
                if let Some(o) = weak.upgrade() {
                    if o.stamp.get() == stamp && o.tip.borrow_mut().take().is_some() {
                        o.redraw();
                    }
                }
            });
        });
    }

    pub(super) fn card_place(&self, (cw, ch): (f64, f64), (w, h): (f64, f64)) -> (f64, f64) {
        let g = room(w, h);
        let picked = self.picked();
        let bubbles = self.bubbles.borrow();
        let Some(b) = bubbles.iter().find(|b| Some(&b.key) == picked.as_ref()) else { return (8.0, 8.0) };
        let (px, py, rp) = (g.cx + b.x * g.radius, g.cy + b.y * g.radius, b.r * g.radius);
        if let Some((x, y)) = self.card_at.get() {
            let inside = x >= 8.0 && x + cw <= w - 8.0 && y >= 8.0 && y + ch <= h - 8.0;
            let (nx, ny) = (px.clamp(x, x + cw), py.clamp(y, y + ch));
            if inside && (px - nx).hypot(py - ny) >= rp + CARD_GAP / 2.0 {
                return (x, y);
            }
        }
        let fits = |x: f64| x >= 8.0 && x + cw <= w - 8.0;
        let (right, left) = (px + rp + CARD_GAP, px - rp - CARD_GAP - cw);
        let (x, y) = if fits(right) || fits(left) {
            let x = if fits(right) { right } else { left };
            (x, (py - ch / 2.0).clamp(8.0, (h - ch - 8.0).max(8.0)))
        } else {
            let above = py - rp - CARD_GAP - ch;
            let y = if above >= 8.0 { above } else { py + rp + CARD_GAP };
            ((px - cw / 2.0).clamp(8.0, (w - cw - 8.0).max(8.0)), y.min((h - ch - 8.0).max(8.0)))
        };
        let at = (x.round(), y.round());
        self.card_at.set(Some(at));
        at
    }
}
