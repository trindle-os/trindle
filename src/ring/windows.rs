//! Open windows, from the compositor's list (trindle_windows_v1).

use std::cell::RefCell;
use std::os::fd::AsRawFd;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::toolkit::event::{Flow, Loop};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_registry;
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, QueueHandle};
use crate::common::wire::windows::{
    trindle_window_v1::{self, TrindleWindowV1},
    trindle_windows_manager_v1::{self, TrindleWindowsManagerV1},
};

pub type Handle = TrindleWindowV1;

const ACTIVE: u32 = 1;
const MINIMIZED: u32 = 2;
const MAXIMIZED: u32 = 4;

#[derive(Clone)]
pub struct Window {
    pub handle: TrindleWindowV1,
    pub title: String,
    pub app_id: String,
    pub active: bool,
    pub minimized: bool,
    pub maximized: bool,
}

#[derive(Default)]
struct State {
    windows: Vec<Window>,
    changed: bool,
    put_away: Vec<TrindleWindowV1>,
}

pub const FORCE_AFTER: Duration = Duration::from_secs(4);
pub const FORCE_UNTIL: Duration = Duration::from_secs(120);

pub fn force_now(ago: Duration, own: bool) -> bool {
    own && ago >= FORCE_AFTER
}

pub struct Windows {
    conn: Connection,
    queue: RefCell<EventQueue<State>>,
    state: RefCell<State>,
    listeners: RefCell<Vec<Box<dyn Fn()>>>,
    closing: RefCell<Vec<(TrindleWindowV1, Instant)>>,
}

impl Windows {
    pub fn start(lp: &Loop) -> Option<Rc<Windows>> {
        let me = Self::connect()?;
        let fd = me.conn.backend().poll_fd().as_raw_fd();
        let weak = Rc::downgrade(&me);
        lp.watch(fd, move |hup| {
            let Some(me) = weak.upgrade() else { return Flow::Stop };
            if hup {
                eprintln!("trindle-ring: the compositor closed the window-list connection");
                return Flow::Stop;
            }
            me.pump();
            Flow::Continue
        });
        Some(me)
    }

    pub fn connect() -> Option<Rc<Windows>> {
        let conn = Connection::connect_to_env().map_err(|e| eprintln!("trindle: no window list: {e}")).ok()?;
        let (globals, mut queue) = registry_queue_init::<State>(&conn).ok()?;
        let qh = queue.handle();
        let mut state = State::default();
        let _manager: Option<TrindleWindowsManagerV1> = globals.bind(&qh, 1..=1, ()).ok();
        queue.roundtrip(&mut state).ok()?;
        queue.roundtrip(&mut state).ok()?;

        let me = Rc::new(Windows {
            conn,
            queue: RefCell::new(queue),
            state: RefCell::new(state),
            listeners: RefCell::new(Vec::new()),
            closing: RefCell::new(Vec::new()),
        });
        Some(me)
    }

    pub fn settle(&self) {
        let mut q = self.queue.borrow_mut();
        let _ = q.roundtrip(&mut self.state.borrow_mut());
    }

    fn pump(&self) {
        if let Some(guard) = self.conn.prepare_read() {
            let _ = guard.read();
        }
        let changed = {
            let mut q = self.queue.borrow_mut();
            let mut s = self.state.borrow_mut();
            let _ = q.dispatch_pending(&mut s);
            std::mem::take(&mut s.changed)
        };
        let _ = self.conn.flush();
        if changed {
            for f in self.listeners.borrow().iter() {
                f();
            }
        }
    }

    pub fn on_change(&self, f: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    pub fn windows(&self) -> Vec<Window> {
        self.state.borrow().windows.clone()
    }

    pub fn activate(&self, win: &Window) {
        win.handle.raise();
        let _ = self.conn.flush();
    }

    pub fn show_desktop(&self) {
        let mut st = self.state.borrow_mut();
        let showing: Vec<TrindleWindowV1> =
            st.windows.iter().filter(|w| !w.minimized).map(|w| w.handle.clone()).collect();
        if showing.is_empty() {
            let back = std::mem::take(&mut st.put_away);
            let alive: Vec<_> = back.into_iter().filter(|h| st.windows.iter().any(|w| &w.handle == h)).collect();
            for h in &alive {
                h.change(0, MINIMIZED);
            }
            if let Some(last) = alive.first() {
                last.raise();
            }
        } else {
            for h in &showing {
                h.change(MINIMIZED, 0);
            }
            st.put_away = showing;
        }
        drop(st);
        let _ = self.conn.flush();
    }

    pub fn raise_newest(own: &crate::common::apps::Own) -> bool {
        let Some(list) = Windows::connect() else { return false };
        list.settle();
        let all = list.windows();
        let found = crate::common::apps::newest_of(&all, |w| w.app_id.as_str(), own);
        if let Some(w) = found {
            list.activate(w);
        }
        found.is_some()
    }

    pub fn raise_for(all: &[crate::common::apps::Entry], app: &crate::common::apps::Entry) -> bool {
        let Some(list) = Windows::connect() else { return false };
        list.settle();
        let found = list.windows().into_iter().rev().find(|w| crate::common::apps::owns(all, app, &w.app_id));
        if let Some(w) = &found {
            list.activate(w);
        }
        found.is_some()
    }

    pub fn open_ids() -> Vec<String> {
        let Some(list) = Windows::connect() else { return Vec::new() };
        list.settle();
        list.windows().into_iter().map(|w| w.app_id).collect()
    }

    pub fn raise_key(key: &str) -> bool {
        crate::common::apps::own_key(key).is_some_and(Windows::raise_newest)
    }

    pub fn minimize(&self, win: &Window) {
        win.handle.change(MINIMIZED, 0);
        let _ = self.conn.flush();
    }

    pub fn maximize(&self, win: &Window) {
        if win.maximized {
            win.handle.change(0, MAXIMIZED);
        } else {
            win.handle.change(MAXIMIZED, 0);
        }
        let _ = self.conn.flush();
    }

    pub fn close(&self, win: &Window) {
        win.handle.close();
        let _ = self.conn.flush();
    }

    pub fn close_asked(&self, win: &Window, own: bool) -> bool {
        let mut closing = self.closing.borrow_mut();
        if let Some(i) = closing.iter().position(|(h, _)| h == &win.handle) {
            if force_now(closing[i].1.elapsed(), own) {
                closing.remove(i);
                drop(closing);
                self.close(win);
            }
            return false;
        }
        closing.push((win.handle.clone(), Instant::now()));
        drop(closing);
        self.close(win);
        true
    }

    pub fn close_asked_at(&self, win: &Window) -> Option<Instant> {
        self.closing.borrow().iter().find(|(h, _)| h == &win.handle).map(|(_, t)| *t)
    }

    pub fn force_offered(&self, win: &Window) -> bool {
        self.close_asked_at(win).is_some_and(|t| t.elapsed() >= FORCE_AFTER)
    }

    pub fn forget_closes(&self) {
        let open = self.windows();
        self.closing.borrow_mut().retain(|(h, when)| when.elapsed() < FORCE_UNTIL && open.iter().any(|w| &w.handle == h));
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<TrindleWindowsManagerV1, ()> for State {
    fn event(state: &mut Self, _: &TrindleWindowsManagerV1, event: trindle_windows_manager_v1::Event,
             _: &(), _: &Connection, _: &QueueHandle<Self>) {
        let trindle_windows_manager_v1::Event::Window { window } = event;
        state.windows.push(Window { handle: window, title: String::new(), app_id: String::new(),
                                    active: false, minimized: false, maximized: false });
    }

    event_created_child!(State, TrindleWindowsManagerV1, [
        trindle_windows_manager_v1::EVT_WINDOW_OPCODE => (TrindleWindowV1, ()),
    ]);
}

impl Dispatch<TrindleWindowV1, ()> for State {
    fn event(state: &mut Self, handle: &TrindleWindowV1, event: trindle_window_v1::Event,
             _: &(), _: &Connection, _: &QueueHandle<Self>) {
        use trindle_window_v1::Event as E;
        let Some(i) = state.windows.iter().position(|w| &w.handle == handle) else { return };
        match event {
            E::Title { title } => state.windows[i].title = title,
            E::AppId { app_id } => state.windows[i].app_id = app_id,
            E::Flags { flags } => {
                state.windows[i].maximized = flags & MAXIMIZED != 0;
                state.windows[i].minimized = flags & MINIMIZED != 0;
                state.windows[i].active = flags & ACTIVE != 0;
            }
            E::Done => state.changed = true,
            E::Gone => {
                state.windows.remove(i);
                handle.destroy();
                state.changed = true;
            }
            _ => {}
        }
    }
}
