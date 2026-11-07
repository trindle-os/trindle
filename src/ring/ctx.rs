//! What every part of the ring shares.

use std::cell::RefCell;
use std::rc::Rc;

use crate::common::apps::{self, Entry};
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::icon::Icons;
use crate::toolkit::window::Display;

use super::view::Face;

pub type Program = Entry;

pub struct Ctx {
    pub lp: Loop,
    pub display: Rc<Display>,
    pub session: Option<Bus>,
    pub system: Option<Bus>,
    pub icons: Rc<Icons>,
    apps: RefCell<Rc<Vec<Program>>>,
    tell: Post<Do>,
}

impl Ctx {
    pub fn new(lp: Loop, display: Rc<Display>, session: Option<Bus>, system: Option<Bus>, tell: Post<Do>) -> Rc<Ctx> {
        Rc::new(Ctx {
            lp, display, session, system, tell,
            icons: Rc::new(Icons::new()),
            apps: RefCell::new(Rc::new(apps::entries())),
        })
    }

    pub fn tell(&self, d: Do) {
        self.tell.send(d);
    }

    pub fn apps(&self) -> Rc<Vec<Program>> {
        self.apps.borrow().clone()
    }

    pub fn reread_apps(&self) {
        *self.apps.borrow_mut() = Rc::new(apps::entries());
    }

    pub fn face(&self, f: impl FnOnce(&mut Face) + 'static) {
        self.tell(Do::Face(Box::new(f)));
    }
}

#[derive(Clone)]
pub struct Msg(pub Rc<dyn Fn(&mut Face)>, Option<Rc<dyn Fn(&Face) -> bool>>);

pub fn msg(f: impl Fn(&mut Face) + 'static) -> Msg {
    Msg(Rc::new(f), None)
}

impl Msg {
    pub fn idle() -> Msg {
        Msg(Rc::new(|_| {}), Some(Rc::new(|_| true)))
    }

    pub fn unless(f: impl Fn(&mut Face) + 'static, moot: impl Fn(&Face) -> bool + 'static) -> Msg {
        Msg(Rc::new(f), Some(Rc::new(moot)))
    }

    pub fn is_moot(&self, face: &Face) -> bool {
        self.1.as_ref().is_some_and(|m| m(face))
    }
}

pub enum Do {
    Command(Vec<String>),
    Tick,
    Notice(String),
    CloseRing,
    Redraw,
    ShowNote(u32),
    Face(Box<dyn FnOnce(&mut Face)>),
    AppsChanged,
    IconsMade(u64),
    IconsLate(u64),
    OpenDrive(String),
}

pub fn time_and_date() -> (String, String) {
    let fmt = crate::common::files::setting("clock_format");
    let clock_only = fmt.rsplit("  ").next().unwrap_or("%H:%M").to_string();
    let now = crate::common::clock::Local::now();
    let text = |f: &str| now.words(f);
    (text(&clock_only), now.line(crate::toolkit::lang::format::Pattern::ShortDay))
}
