//! The computer drawn as a room. The big circle is the memory; each program
//! is a bubble in it, as big as the memory it holds, so the room fills up as
//! the memory does. A program that is working glows and gently beats, faster
//! and brighter the harder it works. Round the rim sits one small bar for
//! each of the processor's threads, growing outwards and warming from blue
//! to red with how busy that thread is, so sixteen threads all flat out are
//! seen at a glance, and the bubble glowing hardest says why.

mod motion;
pub(super) mod paint;
mod input;
#[cfg(test)]
mod tests;

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Instant;

use motion::{arrival, FILL, room, SMALLEST_PX};
use paint::{BAR_FULL, NOTE_LINE_GAP, NOTE_PAD};

use crate::toolkit::colour::Colour;
use crate::toolkit::event::Loop;
use crate::toolkit::input::Cursor;
use crate::toolkit::text::{Align, Style};
use crate::toolkit::ui::Labels;
use crate::toolkit::widget::{drawing, layers, Length, Node, Pen, placed, Pointer, Rect};

const BEAT_MS: u64 = 50;

const RIM_GAP: f64 = 8.0;

const NOTE_LEAST: f64 = 150.0;

const NOTE_MOST: f64 = 250.0;

const FREE_ROW: f64 = 26.0;

const SMALLEST: f32 = 280.0;

pub struct Note {
    pub key: String,
    pub heading: String,
    pub name: String,
    pub words: String,
}

pub struct Blob {
    pub key: String,
    pub title: String,

    pub icon: Vec<String>,

    pub memory: f64,

    pub figure: String,

    pub threads: f64,

    pub heat: f64,

    pub quiet: bool,
}

struct Bubble {
    key: String,
    title: String,
    icon: Vec<String>,
    figure: String,
    quiet: bool,

    x: f64,
    y: f64,

    r: f64,
    target: f64,
    heat: f64,
    heat_to: f64,
    threads: f64,

    phase: f64,
    leaving: bool,
}

#[derive(Clone, Copy)]
struct Room {
    cx: f64,
    cy: f64,
    radius: f64,
    rim: f64,
}

#[derive(Clone, PartialEq)]
enum Under {
    Thread(usize),
    Bubble(String),
    Note(usize),
}

pub struct Orbit {
    events: Loop,
    me: RefCell<Weak<Orbit>>,
    bubbles: RefCell<Vec<Bubble>>,

    threads: RefCell<Vec<(f64, f64)>>,
    notes: RefCell<Vec<Note>>,
    free: RefCell<String>,
    picked: RefCell<Option<String>>,

    keyed: RefCell<Option<String>>,

    card_at: Cell<Option<(f64, f64)>>,

    size: Cell<(f64, f64)>,
    last: Cell<Option<Instant>>,

    pace: Cell<u64>,

    run_no: Cell<u64>,
    moving: bool,
    redraw: RefCell<Option<Rc<dyn Fn()>>>,
    hover: RefCell<Option<Under>>,
    down: RefCell<Option<Under>>,

    tip: RefCell<Option<(String, Rect)>>,
    stamp: Cell<u64>,

    stays: bool,

    notes_at: RefCell<Vec<(Rect, usize)>>,
}

struct Placed {
    index: usize,

    rect: Rect,

    words: [(crate::toolkit::text::Layout, Colour); 3],

    line: (f64, f64, f64, f64),
}

impl Orbit {
    pub fn new(events: &Loop) -> Rc<Orbit> {
        let o = Rc::new(Orbit {
            events: events.clone(),
            me: RefCell::new(Weak::new()),
            bubbles: RefCell::new(Vec::new()),
            threads: RefCell::new(Vec::new()),
            notes: RefCell::new(Vec::new()),
            free: RefCell::new(String::new()),
            picked: RefCell::new(None),
            keyed: RefCell::new(None),
            card_at: Cell::new(None),
            size: Cell::new((0.0, 0.0)),
            last: Cell::new(None),
            pace: Cell::new(0),
            run_no: Cell::new(0),
            moving: crate::common::graphics::moving(),
            redraw: RefCell::new(None),
            hover: RefCell::new(None),
            down: RefCell::new(None),
            tip: RefCell::new(None),
            stamp: Cell::new(0),
            stays: Labels::chosen() != Labels::Brief,
            notes_at: RefCell::new(Vec::new()),
        });
        *o.me.borrow_mut() = Rc::downgrade(&o);
        o
    }

    pub fn on_redraw(&self, f: impl Fn() + 'static) {
        *self.redraw.borrow_mut() = Some(Rc::new(f));
    }

    pub fn redraw(&self) {
        let f = self.redraw.borrow().clone();
        if let Some(f) = f {
            f();
        }
    }

    pub fn picked(&self) -> Option<String> {
        self.picked.borrow().clone()
    }

    pub fn pick(&self, key: Option<&str>) {
        let key = key.map(str::to_string);
        if *self.picked.borrow() == key {
            return;
        }
        *self.picked.borrow_mut() = key;
        self.card_at.set(None);
        self.redraw();
    }

    pub fn set_notes(&self, notes: Vec<Note>, free: String) {
        *self.notes.borrow_mut() = notes;
        *self.free.borrow_mut() = free;
        self.redraw();
    }

    pub fn set_threads(&self, each: &[f64]) {
        {
            let mut t = self.threads.borrow_mut();
            t.resize(each.len(), (0.0, 0.0));
            for (s, b) in t.iter_mut().zip(each) {
                s.1 = *b;
            }
        }
        self.wake();
    }

    pub fn set_bubbles(&self, blobs: Vec<Blob>) {
        let first = self.bubbles.borrow().is_empty();
        {
            let mut bubbles = self.bubbles.borrow_mut();
            for b in bubbles.iter_mut() {
                b.leaving = !blobs.iter().any(|n| n.key == b.key);
            }
            for n in blobs {
                let target = (n.memory.max(0.0) * FILL).sqrt();
                match bubbles.iter_mut().find(|b| b.key == n.key) {
                    Some(b) => {
                        b.target = target;
                        b.heat_to = n.heat;
                        b.threads = n.threads;
                        b.title = n.title;
                        b.figure = n.figure;
                        b.icon = n.icon;
                        b.quiet = n.quiet;
                    }
                    None => {
                        let (x, y) = arrival(&n.key, target);
                        bubbles.push(Bubble {
                            key: n.key, title: n.title, icon: n.icon, figure: n.figure, quiet: n.quiet,
                            x, y, r: 0.0, target, heat: 0.0, heat_to: n.heat, threads: n.threads,
                            phase: 0.0, leaving: false,
                        });
                    }
                }
            }
        }

        if first || !self.moving {
            self.settle_now();
        }
        self.wake();
    }

    fn room(&self) -> Room {
        let (w, h) = self.size.get();
        room(w, h)
    }

    fn smallest(&self) -> f64 {
        SMALLEST_PX / self.room().radius.max(1.0)
    }

    pub fn view<M: Clone + 'static>(self: &Rc<Self>, card: Option<Node<M>>, on_pointer: impl Fn(Pointer) -> M + 'static,
                                    on_key: impl Fn(bool) -> M + 'static, press: M) -> Node<M> {
        let (paint, cursor, keys) = (Rc::clone(self), Rc::clone(self), Rc::clone(self));

        let picture = drawing((0.0, SMALLEST), self.spoken(), move |canvas, pen| paint.paint(canvas, pen))

            .own_labels()
            .on_keys(move |input| keys.key(input).map(&on_key))
            .pressable(press)
            .width(Length::Fill)
            .height(Length::Fill)
            .on_pointer(on_pointer)
            .on_hover()
            .cursor_with(move |x, y| match cursor.under(x as f64, y as f64) {
                Some(Under::Bubble(_) | Under::Note(_)) => Cursor::Hand,
                _ => Cursor::Arrow,
            });
        let mut items = vec![picture];
        if let Some(card) = card {
            let o = Rc::clone(self);
            items.push(placed(card.floats(), move |(cw, ch), (aw, ah)| {
                let (x, y) = o.card_place((cw as f64, ch as f64), (aw as f64, ah as f64));
                (x as f32, y as f32)
            }));
        }
        layers(items)
    }

    fn place_notes(&self, pen: &Pen, h: f64, g: Room) -> Vec<Placed> {
        let outside = g.rim + BAR_FULL + 16.0;
        let width = (g.cx - outside - 8.0).min(NOTE_MOST);
        let notes = self.notes.borrow();
        let bubbles = self.bubbles.borrow();
        let found: Vec<Option<&Bubble>> = notes.iter().take(2).map(|n| {
            bubbles.iter().find(|b| b.key == n.key && !b.leaving).filter(|_| width >= NOTE_LEAST)
        }).collect();

        let sides: Vec<bool> = match found.as_slice() {
            [Some(a), Some(b)] if (a.x >= 0.0) == (b.x >= 0.0) => {
                let a_out = a.x.abs() >= b.x.abs();
                vec![if a_out { a.x >= 0.0 } else { a.x < 0.0 }, if a_out { b.x < 0.0 } else { b.x >= 0.0 }]
            }
            _ => found.iter().map(|b| b.is_some_and(|b| b.x >= 0.0)).collect(),
        };
        let s = pen.scale;
        let inner = ((width - 2.0 * NOTE_PAD.0) as f32 * s).max(1.0);
        let mut out = Vec::new();
        for (i, (b, right)) in found.into_iter().zip(sides).enumerate() {
            let Some(b) = b else { continue };
            let n = &notes[i];
            let align = if right { Align::Start } else { Align::End };
            let style = |size: f32, weight: u16| Style { weight, align, ..Style::new(size * s) };
            let words = [
                (pen.fonts.layout(&n.heading, &style(12.0, 700), Some(inner)), pen.theme.muted),
                (pen.fonts.layout(&n.name, &style(18.0, 700), Some(inner)), pen.theme.fg),
                (pen.fonts.layout(&n.words, &style(13.0, 400), Some(inner)), pen.theme.fg),
            ];
            let tall: f32 = words.iter().map(|(l, _)| l.height).sum::<f32>() / s + 2.0 * NOTE_LINE_GAP as f32;
            let nh = tall as f64 + 2.0 * NOTE_PAD.1;
            let (px, py, rp) = (g.cx + b.x * g.radius, g.cy + b.y * g.radius, b.r * g.radius);
            let x = if right { g.cx + outside } else { g.cx - outside - width };
            let y = (py - nh / 2.0).clamp(8.0, (h - nh - 8.0).max(8.0));

            let (sx, sy) = (if right { x } else { x + width }, y + nh / 2.0);
            let (dx, dy) = (sx - px, sy - py);
            let d = dx.hypot(dy).max(1e-9);
            out.push(Placed {
                index: i,
                rect: Rect { x: x as f32, y: y as f32, w: width as f32, h: nh as f32 },
                words,
                line: (sx, sy, px + dx / d * rp, py + dy / d * rp),
            });
        }
        out
    }
}
