//! What every part of the ring shares: the loop, the screen, the buses,
//! the icons and the apps, and a way to ask the ring itself for something.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::apps::{self, Entry};
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::icon::Icons;
use crate::toolkit::window::Display;

use super::view::Face;

const WINDOW_IDS_MOST: usize = 512;

#[derive(Default)]
pub struct WindowApps(RefCell<HashMap<String, Option<usize>>>);

impl WindowApps {

    pub fn find<'a>(&self, all: &'a [Program], app_id: &str, look: impl FnOnce(&'a [Program], &str) -> Option<&'a Program>) -> Option<&'a Program> {
        let mut seen = self.0.borrow_mut();

        if seen.len() >= WINDOW_IDS_MOST {
            seen.clear();
        }
        let at = *seen.entry(app_id.to_string()).or_insert_with(|| {
            let found = look(all, app_id)?;
            all.iter().position(|e| std::ptr::eq(e, found))
        });
        at.and_then(|i| all.get(i))
    }

    pub fn forget(&self) {
        self.0.borrow_mut().clear();
    }
}

pub type Program = Entry;

pub struct Ctx {
    pub lp: Loop,
    pub display: Rc<Display>,
    pub session: Option<Bus>,
    pub system: Option<Bus>,
    pub icons: Rc<Icons>,
    apps: RefCell<Rc<Vec<Program>>>,

    window_apps: WindowApps,
    tell: Post<Do>,
}

impl Ctx {
    pub fn new(lp: Loop, display: Rc<Display>, session: Option<Bus>, system: Option<Bus>, tell: Post<Do>) -> Rc<Ctx> {
        Rc::new(Ctx {
            lp, display, session, system, tell,
            icons: Rc::new(Icons::new()),
            apps: RefCell::new(Rc::new(apps::entries())),
            window_apps: WindowApps::default(),
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
        self.window_apps.forget();
    }

    pub fn app_for_window<'a>(&self, all: &'a [Program], app_id: &str) -> Option<&'a Program> {
        self.window_apps.find(all, app_id, apps::find_for_window)
    }

    pub fn face(&self, f: impl FnOnce(&mut Face) + 'static) {
        self.tell(Do::Face(Box::new(f)));
    }

    pub fn face_if(&self, f: impl FnOnce(&mut Face) -> bool + 'static) {
        self.tell(Do::FaceIf(Box::new(f)));
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

    Notice(String),
    CloseRing,
    Redraw,

    ShowNote(u32),
    Face(Box<dyn FnOnce(&mut Face)>),
    FaceIf(Box<dyn FnOnce(&mut Face) -> bool>),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn app(id: &str) -> Program {
        Program { id: id.into(), name: id.into(), icon: None, exec: String::new(), try_exec: None, wm_class: String::new(), program: String::new(),
                  kinds: Vec::new(), path: Default::default(), category: "", keywords: Vec::new(), flatpak: None, appimage: None, terminal: false }
    }

    #[test]
    fn a_window_ids_app_is_worked_out_once_until_the_apps_are_read_again() {
        let (all, looked) = ([app("a.desktop"), app("b.desktop")], std::cell::Cell::new(0));
        let seen = WindowApps::default();
        let find = |id: &str| seen.find(&all, id, |all, id| { looked.set(looked.get() + 1); all.iter().find(|a| a.id == id) });
        assert_eq!(find("b.desktop").map(|a| a.id.as_str()), Some("b.desktop"));
        assert_eq!(find("b.desktop").map(|a| a.id.as_str()), Some("b.desktop"));
        assert!(find("nothing").is_none() && find("nothing").is_none());
        assert_eq!(looked.get(), 2, "one look for each id, found or not");
        seen.forget();
        find("b.desktop");
        assert_eq!(looked.get(), 3);
    }

    #[test]
    fn the_memory_of_window_ids_has_a_ceiling() {
        let (all, seen) = ([app("a.desktop")], WindowApps::default());
        for n in 0..WINDOW_IDS_MOST * 3 {
            seen.find(&all, &format!("w{n}"), |_, _| None);
        }
        assert!(seen.0.borrow().len() <= WINDOW_IDS_MOST);
    }
}
