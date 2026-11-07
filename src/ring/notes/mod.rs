//! Notifications as they arrive.

mod card;
pub(super) mod icons;
mod keyboard;
mod actions;
mod moving;
mod keeps;
#[cfg(test)]
pub mod tests;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use card::FRAME;
use icons::note_icons;

use crate::ring::circles::{self, Act, Buttons, Corner, Face, Sizes};
use crate::ring::ctx::Ctx;
use crate::ring::label_window::{Beside, LabelWindow};
use crate::ring::notify::{self, Note};
use crate::ring::stack::Stack;
use crate::toolkit::{t, theme};
use crate::toolkit::text::Fonts;
use crate::toolkit::ui::Shown;
use crate::toolkit::widget::Rect;
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};

pub const NAME: &str = "trindle-notes";

const PLACED_WITHIN: u64 = 250;

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

#[derive(Clone)]
struct Content {
    icons: Vec<String>,
    title: String,
    body: String,
    id: u32,
    handoff: String,
    drive: String,
    allow: String,
    why: String,
}

#[derive(Clone)]
struct Look {
    across: f32,
    scroll: usize,
    face: Face,
    buttons: Buttons,
    hover: bool,
    pinned: bool,
    hot: Option<Act>,
    copied: bool,
    moving: bool,
}

struct Card {
    lp: crate::toolkit::event::Loop,
    notes: Weak<Notes>,
    content: Content,
    look: Look,
    inside: Cell<bool>,
    on: Cell<Option<Act>>,
    grab: u32,
    press: Rc<Cell<u32>>,
    last: Cell<(f32, f32)>,
}

#[derive(Clone, Debug, PartialEq)]
enum Msg {
    At { inside: bool, hot: Option<Act> },
    Act(Act),
    Scroll(i32),
    Leave,
    Drag { x: f32, y: f32 },
    Drop,
    PutBack,
    Nudge { right: i8, down: i8 },
    Placed,
}

struct Win {
    shown: Shown<Card>,
    last: Cell<(u32, [i32; 4])>,
    placed: Cell<(f32, f32)>,
    asked: Cell<Option<((f32, f32), u64)>>,
}

pub struct Notes {
    ctx: Rc<Ctx>,
    me: Weak<Notes>,
    stack: RefCell<Stack>,
    content: RefCell<HashMap<u32, Content>>,
    wins: RefCell<HashMap<u32, Rc<Win>>>,
    corner: Cell<Corner>,
    wake: Cell<u32>,
    framing: Cell<bool>,
    fonts: RefCell<Option<Fonts>>,
    focused: Cell<Option<u32>>,
    turn: Cell<u32>,
    copied: Cell<Option<(u32, u64)>>,
    label: LabelWindow,
    saved: RefCell<String>,
}

pub fn start(ctx: &Rc<Ctx>) -> Rc<Notes> {
    Rc::new_cyclic(|me| Notes {
        ctx: ctx.clone(), me: me.clone(), stack: RefCell::new(Stack::new(Sizes::sheet(), slide())),
        content: RefCell::new(HashMap::new()), wins: RefCell::new(HashMap::new()), corner: Cell::new(Corner::chosen()),
        wake: Cell::new(0), framing: Cell::new(false), fonts: RefCell::new(None),
        focused: Cell::new(None), turn: Cell::new(0), copied: Cell::new(None), label: LabelWindow::new(ctx),
        saved: RefCell::new(String::new()),
    })
}

fn slide() -> u64 {
    theme::sheet().note_slide.max(0.0) as u64
}

impl Notes {
    pub fn announce(&self, note: &Note) {
        if !notify::showing() {
            return;
        }
        let content = self.content_for(note);
        let fresh = !self.stack.borrow().has(note.id);
        let said = content.said();
        let rest = self.rest_of(&content);
        self.content.borrow_mut().insert(note.id, content.clone());
        let win = self.wins.borrow().get(&note.id).cloned();
        if let Some(w) = &win {
            w.shown.change(|c| c.content = content);
            w.shown.announce(&said);
        }
        self.stack.borrow_mut().add_sized(note.id, notify::how_long(note.urgent), rest, now_ms());
        if fresh {
            crate::common::sounds::play("notification");
        }
        self.sync();
        if fresh {
            if let Some(w) = self.wins.borrow().get(&note.id) {
                w.shown.announce(&said);
            }
        }
    }

    fn content_for(&self, note: &Note) -> Content {
        let title = if note.summary.is_empty() { note.app.clone() } else { note.summary.clone() };
        Content {
            icons: note_icons(&self.ctx.apps(), note), title, body: note.body.clone(),
            id: note.id, handoff: note.handoff.clone(), drive: note.drive.clone(), allow: note.allow.clone(), why: String::new(),
        }
    }

    pub fn bring_back(&self) {
        if !notify::showing() {
            return;
        }
        let all = keeps::load();
        *self.saved.borrow_mut() = keeps::render(&all);
        for k in all {
            let note = notify::restore(&k.app, &k.icon, &k.desktop, &k.summary, &k.body, k.wake.is_none());
            let content = self.content_for(&note);
            let rest = self.rest_of(&content);
            self.content.borrow_mut().insert(note.id, content);
            self.stack.borrow_mut().restore(note.id, rest, k.put, k.wake, now_ms());
        }
        self.sync();
    }

    fn to_keep(&self) -> Vec<keeps::Kept> {
        let s = self.stack.borrow();
        let notes = notify::list();
        let one = |id: u32, put, wake| {
            let n = notes.iter().find(|n| n.id == id)?;
            Some(keeps::Kept { app: n.app.clone(), icon: n.icon.clone(), desktop: n.desktop.clone(), summary: n.summary.clone(), body: n.body.clone(), put, wake })
        };
        let pinned = s.pinned_places().into_iter().filter_map(|(id, put)| one(id, put, None));
        let asleep = s.sleepers().into_iter().filter_map(|(id, wake)| one(id, None, Some(wake)));
        pinned.chain(asleep).collect()
    }

    fn keep(&self) {
        if self.stack.borrow().pinned_places().iter().any(|(id, _)| self.stack.borrow().holding(*id)) {
            return;
        }
        let text = keeps::render(&self.to_keep());
        if *self.saved.borrow() != text {
            keeps::save(&text);
            *self.saved.borrow_mut() = text;
        }
    }

    pub fn withdraw(&self, id: u32) {
        self.shut(id);
        self.sync();
        self.keep_keyboard();
    }

    pub fn pinned_elsewhere(&self, id: u32, pinned: bool) {
        if self.stack.borrow_mut().set_pin(id, pinned, now_ms()) {
            self.sync();
        }
    }

    fn ids(&self) -> Vec<u32> {
        self.stack.borrow().places(self.corner.get(), self.screen(), now_ms()).into_iter().map(|p| p.0).collect()
    }

    pub fn restyle(&self) {
        self.stack.borrow_mut().restyle(Sizes::sheet(), slide(), now_ms());
        let all: Vec<(u32, Content)> = self.content.borrow().iter().map(|(id, c)| (*id, c.clone())).collect();
        for (id, c) in all {
            let rest = self.rest_of(&c);
            self.stack.borrow_mut().set_rest(id, rest, now_ms());
        }
        let wins: Vec<Rc<Win>> = self.wins.borrow().values().cloned().collect();
        for w in &wins {
            w.shown.restyle();
        }
        self.label.restyle();
        self.sync();
    }

    fn shut(&self, id: u32) {
        self.stack.borrow_mut().close(id);
        self.content.borrow_mut().remove(&id);
        self.label.hide();
    }

    fn buttons(&self, id: u32) -> Buttons {
        let many = self.stack.borrow().many();
        self.buttons_with(id, many)
    }

    fn buttons_with(&self, id: u32, many: bool) -> Buttons {
        Buttons { many, extra: self.content.borrow().get(&id).and_then(Content::extra) }
    }

    fn beside(&self, id: u32, at: Rect) -> Option<Beside> {
        let (corner, screen) = (self.corner.get(), self.screen());
        let (x, y, d) = self.stack.borrow().places(corner, screen, now_ms()).into_iter().find(|p| p.0 == id).map(|p| (p.1, p.2, p.3))?;
        let across = if corner.right() { screen.0 - x } else { x + d };
        Some(Beside { right: corner.right(), across: across.round() as i32, middle: (y + at.y + at.h / 2.0).round() as i32 })
    }

    fn point(&self, id: u32, on: bool) {
        if !on && self.focused.get() == Some(id) {
            return;
        }
        let grown = if on { self.grown(id) } else { self.stack.borrow().rest(id) };
        self.stack.borrow_mut().point(id, on, grown, now_ms());
        self.sync();
    }

    fn rest_of(&self, c: &Content) -> f32 {
        let mut fonts = self.fonts.borrow_mut();
        let fonts = fonts.get_or_insert_with(Fonts::new);
        circles::rest(fonts, Sizes::sheet(), self.screen().1, &c.title, &c.body)
    }

    fn grown(&self, id: u32) -> f32 {
        let Some(c) = self.content.borrow().get(&id).cloned() else { return Sizes::sheet().across };
        let tall = self.screen().1;
        let rest = self.stack.borrow().rest(id);
        let mut fonts = self.fonts.borrow_mut();
        let fonts = fonts.get_or_insert_with(Fonts::new);
        circles::grown(fonts, Sizes::sheet(), rest, circles::shown_buttons(self.buttons(id)), &c.title, &c.body, tall)
    }

    fn scroll(&self, id: u32, lines: i32) {
        let Some(c) = self.content.borrow().get(&id).cloned() else { return };
        let now = now_ms();
        let across = self.stack.borrow().across(id, now).round();
        let most = {
            let mut fonts = self.fonts.borrow_mut();
            let fonts = fonts.get_or_insert_with(Fonts::new);
            circles::most_scroll(fonts, Sizes { across, ..Sizes::sheet() }, circles::shown_buttons(self.buttons(id)), &c.title, &c.body)
        };
        self.stack.borrow_mut().scroll_by(id, lines, most);
        self.sync();
    }

    fn screen(&self) -> (f32, f32) {
        self.ctx.display.places().first().map_or((1280.0, 800.0), |p| (p[2] as f32, p[3] as f32))
    }

    fn margins(corner: Corner, screen: (f32, f32), (x, y, d): (f32, f32, f32)) -> [i32; 4] {
        let (v, h) = (if corner.bottom() { screen.1 - y - d } else { y }.round() as i32, if corner.right() { screen.0 - x - d } else { x }.round() as i32);
        match (corner.bottom(), corner.right()) {
            (true, true) => [0, h, v, 0],
            (true, false) => [0, 0, v, h],
            (false, true) => [v, h, 0, 0],
            (false, false) => [v, 0, 0, h],
        }
    }

    fn open_window(&self, id: u32, corner: Corner, d: u32, margins: [i32; 4], at: (f32, f32)) -> Option<Rc<Win>> {
        let content = self.content.borrow().get(&id).cloned()?;
        let said = content.said();
        let card = Card {
            lp: self.ctx.lp.clone(), notes: self.me.clone(), content,
            look: Look { across: d as f32, scroll: 0, face: Face::Note, buttons: Buttons { many: false, extra: None }, hover: false, pinned: false, hot: None, copied: false, moving: false },
            inside: Cell::new(false), on: Cell::new(None), grab: 0, press: Rc::new(Cell::new(0)), last: Cell::new((0.0, 0.0)),
        };
        let edges = (if corner.bottom() { Edges::BOTTOM } else { Edges::TOP }) | if corner.right() { Edges::RIGHT } else { Edges::LEFT };
        let kind = Kind::Panel(Panel { layer: Layer::Overlay, edges, keep: -1, margins, keys: Keys::Never, name: NAME.into(), screen: None });
        let shown = Shown::open(&self.ctx.lp, &self.ctx.display, kind, (d, d), card)?;
        shown.see_through();
        shown.spoken(t!("ring-notes-name"));
        shown.announce(&said);
        let me = self.me.clone();
        shown.labels_outside(move |out| {
            if let Some(n) = me.upgrade() {
                n.label.show(out, |at| n.beside(id, at));
            }
        });
        shown.window.set_visible(true);
        Some(Rc::new(Win { shown, last: Cell::new((d, margins)), placed: Cell::new(at), asked: Cell::new(None) }))
    }

    fn sync(&self) {
        let now = now_ms();
        let corner = Corner::chosen();
        if corner != self.corner.get() {
            self.wins.borrow_mut().clear();
            self.corner.set(corner);
        }
        let screen = self.screen();
        let (placed, many) = {
            let s = self.stack.borrow();
            (s.places(corner, screen, now), s.many())
        };
        let live: Vec<u32> = placed.iter().map(|p| p.0).collect();
        self.wins.borrow_mut().retain(|id, _| live.contains(id));
        for (id, x, y, d) in placed {
            let (side, margins) = (d as u32, Self::margins(corner, screen, (x, y, d)));
            let have = self.wins.borrow().get(&id).cloned();
            let win = match have {
                Some(w) => w,
                None => {
                    let Some(w) = self.open_window(id, corner, side, margins, (x, y)) else { continue };
                    self.wins.borrow_mut().insert(id, w.clone());
                    w
                }
            };
            let waiting = win.asked.get().is_some_and(|(_, at)| now < at + PLACED_WITHIN);
            if win.last.get() != (side, margins) && !waiting {
                if let Some((at, _)) = win.asked.take() {
                    win.placed.set(at);
                }
                win.last.set((side, margins));
                win.asked.set(Some(((x, y), now)));
                win.shown.window.resize((side, side));
                win.shown.window.set_margins_placed(margins);
            }
            let look = {
                let s = self.stack.borrow();
                let copied = self.copied.get().is_some_and(|(c, until)| c == id && until > now);
                Look { across: d, scroll: s.scroll(id), face: s.face(id), buttons: self.buttons_with(id, many), hover: s.hovering(id), pinned: s.pinned(id), hot: None, copied, moving: s.holding(id) }
            };
            win.shown.change(|c| c.look = Look { hot: if look.hover { c.look.hot } else { None }, ..look });
        }
        self.keep();
        self.schedule(now);
    }

    fn schedule(&self, now: u64) {
        if self.stack.borrow().moving(now) && !self.framing.replace(true) {
            let me = self.me.clone();
            self.ctx.lp.after(Duration::from_millis(FRAME), move || {
                let Some(n) = me.upgrade() else { return };
                n.framing.set(false);
                n.sync();
            });
        }
        let this = self.wake.get().wrapping_add(1);
        self.wake.set(this);
        let Some(due) = self.stack.borrow().next_wake() else { return };
        let me = self.me.clone();
        self.ctx.lp.after(Duration::from_millis(due.saturating_sub(now)), move || {
            let Some(n) = me.upgrade() else { return };
            if n.wake.get() != this {
                return;
            }
            let gone = n.stack.borrow_mut().tick(now_ms());
            let mut content = n.content.borrow_mut();
            for id in &gone.closed {
                content.remove(id);
            }
            drop(content);
            n.sync();
        });
    }
}
