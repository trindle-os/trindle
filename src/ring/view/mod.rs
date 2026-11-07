//! The ring.

mod pinned;
mod arrange;
mod launch;
mod power;
mod menu;
mod voice;
mod window_ring;
pub(super) mod middle;
pub(super) mod app_tile;
pub(super) mod paint;
mod app;
pub mod panel;
#[cfg(test)]
pub mod tests;
#[cfg(test)]
mod cover_tests;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant, SystemTime};

use menu::{FlatAsk, Gone};
use pinned::{Looking, Pinned};
use tiny_skia::Pixmap;

use crate::common::{apps, style};
use crate::common::removal::How;
use crate::ring::{store, using};
use crate::ring::ctx::{Ctx, Do, Msg, Program, time_and_date};
use crate::ring::layout::{layout, Layout};
use crate::ring::quick::Quick;
use crate::ring::tray::TrayRow;
use crate::ring::updates::Updates;
use crate::ring::windows::{Handle, Window, Windows};
use crate::screenshot::cast_capture::Capture;
use crate::toolkit::motion::Tween;
use crate::toolkit::widget::Node;

const FAVOURITES: u8 = 0;

const ALL: u8 = 1;

const FADE_MS: u64 = 200;

const REFUSED_SECONDS: u32 = 4;

pub struct Face {
    pub ctx: Rc<Ctx>,
    pub quick: Quick,
    pub tray: TrayRow,
    pub updates: Updates,
    windows: Option<Rc<Windows>>,
    size: (f64, f64),
    level: u8,
    level_turn: u64,
    level_k: f32,
    level_tween: Option<Tween>,
    dim: f32,
    typed: String,
    favourites: Vec<String>,
    fav_stamp: Option<SystemTime>,
    pinned: RefCell<Pinned>,
    image_stamps: Option<Vec<Option<SystemTime>>>,
    images_there: Vec<std::path::PathBuf>,
    looking: Rc<RefCell<Looking>>,
    opening: Rc<RefCell<Vec<(String, Instant)>>>,
    lay: Rc<Layout<usize>>,
    first_len: usize,
    best_match: Option<usize>,
    menu: Option<(usize, [f32; 4])>,
    beside: Option<(String, [f32; 4], Rc<dyn Fn() -> Node<Msg>>)>,
    pressed_at: [f32; 4],
    press: Option<(usize, f32, f32, bool)>,
    power: bool,
    armed: Option<String>,
    offered: bool,
    arm_turn: u32,
    can_hibernate: bool,
    words: Option<String>,
    words_turn: u32,
    said: Option<crate::common::ring_said::Said>,
    hover_app: Option<usize>,
    win_title: bool,
    list_keys: Option<crate::common::combos::Combo>,
    win_hover: Option<Handle>,
    win_press: Option<Handle>,
    win_menu: Option<(Handle, [f32; 4])>,
    watching: Option<Handle>,
    win_picture: Option<Rc<Pixmap>>,
    pic_turn: u64,
    capture: Option<Rc<Capture>>,
    capture_failed: bool,
    segment_hover: bool,
    top_hover: bool,
    clock: (String, String),
    using: Vec<crate::common::in_use::User>,
    using_hold: using::Hold,
    listening: bool,
    pulse: f32,
    listener: Option<Rc<crate::voice::Listener>>,
    listen_turn: u32,
    gone: Option<(usize, Gone)>,
    gone_turn: u32,
    wipe: bool,
    flat_ask: Option<FlatAsk>,
    how: Option<(usize, Option<How>)>,
}

impl Face {
    pub fn open_quick(&mut self, panel: &str) {
        self.quick.show(panel);
    }
    pub fn settings(&mut self, panel: Option<&str>) {
        match panel {
            Some("disks") => self.open_own("drives", &[]),
            Some(p) => self.open_own("settings", &["--panel", p]),
            None => self.open_own("settings", &[]),
        }
    }
    pub fn show_beside(&mut self, anchor: &str, card: Rc<dyn Fn() -> Node<Msg>>) {
        self.beside = Some((anchor.to_string(), self.pressed_at, card));
    }
    pub fn close_beside(&mut self) {
        self.beside = None;
    }
    pub(super) fn close_windows_named(&mut self, app_id: &str) -> bool {
        let Some(ws) = self.windows.clone() else { return false };
        let mine: Vec<_> = ws.windows().into_iter().filter(|w| apps::same_name(&w.app_id, app_id)).collect();
        mine.iter().for_each(|w| ws.close(w));
        !mine.is_empty()
    }
    pub fn say(&mut self, words: &str, seconds: u32) {
        self.words = (!words.is_empty()).then(|| words.to_string());
        self.words_turn += 1;
        if seconds > 0 {
            let turn = self.words_turn;
            self.ctx.lp.after(Duration::from_secs(seconds as u64), {
                let ctx = self.ctx.clone();
                move || ctx.face(move |f| if f.words_turn == turn { f.words = None })
            });
        }
    }
    pub fn close(&mut self) {
        self.ctx.tell(Do::CloseRing);
    }

    fn windows_of(&self, app: &Program) -> Vec<Window> {
        let Some(ws) = &self.windows else { return Vec::new() };
        let all = self.ctx.apps();
        ws.windows().into_iter().filter(|w| apps::owns(&all, app, &w.app_id)).collect()
    }

    fn reload(&mut self) {
        self.read_favourites(true);
        self.relayout();
    }

    fn read_favourites(&mut self, always: bool) {
        let stamp = store::favourites_changed_at();
        if always || stamp != self.fav_stamp {
            self.fav_stamp = stamp;
            self.favourites = store::favourites();
        }
    }

    fn opened(&mut self, size: (f64, f64)) {
        self.close_menu();
        self.win_title = crate::common::files::setting("ring_window_names") != "no";
        self.list_keys = crate::common::combos::Combo::typed(&crate::common::files::setting(crate::common::combos::LIST_KEYS)).ok();
        self.said = crate::common::ring_said::read();
        self.size = size;
        self.read_favourites(false);
        let ids = self.pinned_ids();
        self.look_aside(&ids);
        self.updates.notice();
        self.updates.look();
        self.tray.set_open(true);
        self.level = FAVOURITES;
        self.level_k = 1.0;
        self.dim = style::palette().dim as f32;
        self.typed.clear();
        self.relayout();
        self.ask_hibernate();
    }

    fn closed(&mut self) {
        if let Ok(dir) = std::env::var("XDG_RUNTIME_DIR") {
            let _ = std::fs::write(format!("{dir}/trindle-ring-closed"), "");
        }
        if let Some(said) = self.said.take() {
            crate::common::ring_said::shown(said);
        }
        self.close_menu();
        self.close_beside();
        self.show_power(false);
        self.quick.hide();
        self.tray.set_open(false);
        self.stop_listening();
        self.hover_app = None;
        self.win_hover = None;
        self.win_press = None;
        self.win_menu = None;
        self.stop_watching();
        self.capture = None;
        self.capture_failed = false;
    }

    fn fresh(ctx: &Rc<Ctx>, windows: Option<Rc<Windows>>) -> Face {
        Face {
            ctx: ctx.clone(), quick: Quick::new(ctx), tray: TrayRow::new(ctx), updates: Updates::new(ctx), windows: windows.clone(),
            size: (1280.0, 800.0),
            level: FAVOURITES, level_turn: 0, level_k: 1.0, level_tween: None,
            dim: style::palette().dim as f32,
            typed: String::new(),
            favourites: store::favourites(), fav_stamp: store::favourites_changed_at(),
            pinned: RefCell::default(), image_stamps: None, images_there: Vec::new(), looking: Rc::default(), opening: Rc::default(),
            lay: Rc::new(layout::<usize>(&[], &[], 1280.0, 800.0)), first_len: 0, best_match: None,
            menu: None, beside: None, pressed_at: [0.0; 4], press: None,
            power: false, armed: None, offered: false, arm_turn: 0, can_hibernate: false,
            words: None, words_turn: 0, said: None,
            hover_app: None,
            win_title: crate::common::files::setting("ring_window_names") != "no",
            list_keys: None,
            win_hover: None, win_press: None, win_menu: None, watching: None, win_picture: None, pic_turn: 0, capture: None, capture_failed: false,
            segment_hover: false, top_hover: false,
            clock: time_and_date(),
            using: crate::common::in_use::read_all(), using_hold: Default::default(),
            listening: false, pulse: 0.0, listener: None, listen_turn: 0,
            gone: None, gone_turn: 0, wipe: false, flat_ask: None, how: None,
        }
    }
}
