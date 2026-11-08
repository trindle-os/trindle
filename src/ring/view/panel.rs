//! The ring's panel: opening it, fading it in and out and putting it away, and ending the session.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use crate::common::style;
use crate::ring::kept;
use crate::ring::ctx::{Ctx, time_and_date};
use crate::ring::windows::Windows;
use crate::toolkit::event::Flow;
use crate::toolkit::motion::{Ease, tween, Tween};
use crate::toolkit::ui::Shown;
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};

use super::{ALL, Face, FADE_MS};
use super::app::look;

pub struct Ring {
    ctx: Rc<Ctx>,
    shown: RefCell<Option<Shown<Face>>>,
    spare: RefCell<Option<Face>>,
    open: Cell<bool>,
    windows: Option<Rc<Windows>>,
    fade: RefCell<Option<Tween>>,
    me: std::rc::Weak<Ring>,
}

impl Ring {
    pub fn start(ctx: &Rc<Ctx>, windows: Option<Rc<Windows>>) -> Rc<Ring> {
        let face = Face::fresh(ctx, windows.clone());
        let ring = Rc::new_cyclic(|me| Ring { ctx: ctx.clone(), shown: RefCell::new(None), spare: RefCell::new(Some(face)), open: Cell::new(false), windows, fade: RefCell::new(None), me: me.clone() });
        let weak = Rc::downgrade(&ring);
        ctx.lp.every(Duration::from_secs(1), move || {
            let Some(r) = weak.upgrade() else { return Flow::Stop };
            if r.is_open() {
                let now = time_and_date();
                let using = crate::common::in_use::read_all();

                r.change_if(Box::new(move |f| {
                    let new_time = f.clock != now;
                    f.clock = now;
                    let held = f.using_hold.settle(std::time::Instant::now(), using);
                    let new_use = f.using != held;
                    f.using = held;

                    f.updates.notice() || new_time || new_use
                }));
            }
            Flow::Continue
        });
        if let Some(ws) = ring.windows() {
            let weak = Rc::downgrade(&ring);
            ws.on_change(move || {
                if let Some(r) = weak.upgrade() {
                    r.redraw();
                }
            });
        }
        ring
    }
    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    fn size(&self) -> (f64, f64) {
        if let Some(s) = self.shown.borrow().as_ref() {
            let (w, h) = s.window.size();
            if w > 0 && h > 0 {
                return (w as f64, h as f64);
            }
        }
        self.ctx.display.places().first().map(|p| (p[2] as f64, p[3] as f64)).unwrap_or((1280.0, 800.0))
    }

    pub fn open_fresh(&self) {

        kept::pulse();
        if self.shown.borrow().is_none() {
            let face = self.spare.borrow_mut().take().unwrap_or_else(|| Face::fresh(&self.ctx, self.windows.clone()));
            let (w, h) = self.size();

            let kind = Kind::Panel(Panel { layer: Layer::Overlay, edges: Edges::ALL, keep: 0, margins: [0; 4], keys: Keys::Always,
                                           name: "trindle-ring".into(), screen: None });
            match Shown::open(&self.ctx.lp, &self.ctx.display, kind, (w as u32, h as u32), face) {
                Some(s) => {
                    s.see_through();
                    s.spoken("trindle-ring");
                    *self.shown.borrow_mut() = Some(s);
                }
                None => {
                    eprintln!("trindle-ring: the ring panel could not open");

                    crate::common::ring_said::leave(crate::common::ring_said::Said::NotStarted);
                    return;
                }
            }
        }
        let size = self.size();
        self.open.set(true);
        if let Some(s) = self.shown.borrow().as_ref() {
            s.change(move |f| f.opened(size));
            s.set_look(look);
            s.window.set_visible(true);
        }
        self.fade_to(true);
    }

    fn fade_to(&self, show: bool) {
        if let Some(t) = self.fade.borrow_mut().take() {
            t.stop();
        }
        let (step, done) = (self.me.clone(), self.me.clone());
        let t = tween(&self.ctx.lp, FADE_MS, Ease::Out, move |k| {
            if let Some(r) = step.upgrade() {
                if let Some(s) = r.shown.borrow().as_ref() {
                    s.set_shown(if show { k } else { 1.0 - k });
                }
            }
        }, move || {
            if show {
                return;
            }
            if let Some(r) = done.upgrade() {
                if let Some(s) = r.shown.borrow().as_ref() {
                    s.window.set_visible(false);
                }
            }
        });
        *self.fade.borrow_mut() = Some(t);
    }

    pub fn close(&self) {
        if !self.open.replace(false) {
            return;
        }
        if let Some(s) = self.shown.borrow().as_ref() {
            s.change(|f| f.closed());
        }
        self.fade_to(false);
    }
    pub fn redraw(&self) {

        if !self.is_open() {
            return;
        }
        let size = self.size();
        self.change(Box::new(move |f| {
            f.size = size;
            f.read_favourites(false);
            f.relayout();
        }));
    }
    pub fn restyle(&self) {
        if let Some(s) = self.shown.borrow().as_ref() {
            s.set_look(look);
            s.change(|f| f.dim = style::palette().dim as f32);

            s.reload_icons();
        }
    }
    pub fn reload_icons(&self) {
        if let Some(s) = self.shown.borrow().as_ref() {
            s.reload_icons();
        }
    }
    pub fn listen_now(&self, seconds: u32) {
        self.change(Box::new(move |f| f.listen(seconds)));
    }
    pub fn heard_words(&self, words: &str) {
        let words = words.to_string();
        self.change(Box::new(move |f| f.heard(&words)));
    }
    pub fn show_power_choice(&self) {
        if !self.is_open() {
            self.open_fresh();
        }
        self.change(Box::new(|f| f.show_power(true)));
    }

    pub fn toggle_shortcuts(&self) {
        let was_open = self.is_open();
        if !was_open {
            self.open_fresh();
        }
        self.change(Box::new(move |f| f.toggle_shortcuts(was_open)));
    }
    pub fn show_all(&self) {
        if !self.is_open() {
            self.open_fresh();
        }
        self.change(Box::new(|f| f.set_level(ALL)));
    }
    pub fn show_note(&self, id: u32) {
        if !self.is_open() {
            self.open_fresh();
        }
        self.change(Box::new(move |f| {
            f.show_power(false);
            f.quick.show_note(id);
        }));
    }

    pub fn resting(&self) -> bool {
        !self.is_open()
    }
    pub fn windows(&self) -> Option<Rc<Windows>> {
        self.windows.clone()
    }
    pub fn change(&self, f: Box<dyn FnOnce(&mut Face)>) {
        if let Some(s) = self.shown.borrow().as_ref() {
            return s.change(f);
        }
        if let Some(face) = self.spare.borrow_mut().as_mut() {
            f(face);
        }
    }

    pub fn change_if(&self, f: Box<dyn FnOnce(&mut Face) -> bool>) {
        if let Some(s) = self.shown.borrow().as_ref() {
            return s.change_if(f);
        }
        if let Some(face) = self.spare.borrow_mut().as_mut() {
            f(face);
        }
    }
}

pub fn session_end(ctx: &Rc<Ctx>, what: &str) {
    let (ctx, what) = (ctx.clone(), what.to_string());
    crate::common::sounds::play_then("logout", move || session_end_now(&ctx, &what));
}

fn session_end_now(ctx: &Rc<Ctx>, what: &str) {
    if what == "logout" {
        return crate::common::compositor::end_session();
    }

    let known = matches!(what, "Suspend" | "Hibernate" | "Reboot" | "PowerOff");
    let Some(bus) = ctx.system.as_ref().filter(|_| known) else {
        eprintln!("trindle-ring: {what} cannot be asked for");
        return crate::common::ring_said::leave(crate::common::ring_said::Said::NotDone);
    };
    let what = what.to_string();
    bus.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
        .method(&what.clone(), &(true,), true, Some(Duration::from_secs(10)), &ctx.lp, move |r| {
            if let Err(e) = r {
                eprintln!("trindle-ring: {what} refused: {}", e.text);
                crate::common::ring_said::leave(crate::common::ring_said::Said::NotDone);
            }
        });
}
