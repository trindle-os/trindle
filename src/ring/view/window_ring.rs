//! The ring of open windows round the middle.

use std::rc::Rc;
use std::time::Duration;

use crate::common::apps;
use crate::ring::{kept, menu_button, store, using};
use crate::ring::ctx::{msg, Msg, Program};
use crate::ring::layout::{CENTRE, MARGIN};
use crate::ring::open_windows::{self, Look};
use crate::ring::windows::{FORCE_AFTER, Handle, Window};
use crate::screenshot::cast_capture::{Capture, Source};
use crate::toolkit::t;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::widget::{button, card, column, drawing, Length, Node, note, placed, Pointer, row, space, title};

use super::{Face, FAVOURITES};
use super::app_tile::opens_menu;
use super::menu::{MENU_TITLE_CHARS, startup_button};

const WINDOW_PICTURE_EVERY: Duration = Duration::from_secs(1);

const WINDOW_PICTURE_KEPT: Duration = Duration::from_millis(150);

#[derive(Clone, Copy)]
enum WinAct {
    Minimise,
    Maximise,
    Close,
    Star,
}

pub(super) fn pointing_shows_picture(pointing: bool, menu: bool, power: bool, quick: bool) -> bool {
    pointing && !menu && !power && !quick
}

impl Face {
    pub(super) fn open_ring(&self) -> Option<(open_windows::Ring, Vec<Window>)> {
        if self.level != FAVOURITES && !self.searching() {
            return None;
        }
        let ws = self.windows.as_ref()?;
        let wins: Vec<Window> = ws.windows().into_iter().filter(|w| !(w.title.is_empty() && w.app_id.is_empty())).collect();
        let all = self.ctx.apps();
        let wins = open_windows::grouped(wins, |w| apps::find_for_window(&all, &w.app_id).map(|a| a.name.clone()).unwrap_or_else(|| w.app_id.to_lowercase()));
        let half = self.size.0.min(self.size.1) / 2.0 - MARGIN;
        let ring = open_windows::ring(wins.len(), self.lay.disc_radius, half, &open_windows::Sizes::of(crate::toolkit::theme::sheet()))?;
        Some((ring, wins))
    }

    fn window_words(apps: &[Program], w: &Window) -> (String, String) {
        let program = apps::find_for_window(apps, &w.app_id).map(|a| a.name.clone()).unwrap_or_else(|| w.app_id.clone());
        (program, w.title.clone())
    }

    pub(super) fn windows_ring(&self, ring: &open_windows::Ring, wins: &[Window]) -> Vec<Node<Msg>> {
        let (w, h) = (self.size.0 as f32, self.size.1 as f32);
        let centre = (w / 2.0, h / 2.0);
        let looks = self.window_looks(wins);
        let handles: Vec<Handle> = wins.iter().map(|w| w.handle.clone()).collect();
        let mut out = vec![Self::ring_drawing(ring, &looks, handles, (w, h))];
        for (i, win) in wins.iter().enumerate() {
            let b = ring.button_of(i);
            let at = [centre.0 + b[0] as f32, centre.1 + b[1] as f32, b[2] as f32, b[3] as f32];
            let node = Self::window_button(ring, i, &looks[i], win.handle.clone(), at);
            out.push(placed(node, move |_, _| (at[0], at[1])));
        }
        out
    }

    fn window_looks(&self, wins: &[Window]) -> Vec<Look> {
        let all = self.ctx.apps();
        let mut looks = Vec::with_capacity(wins.len());
        for win in wins {
            let (program, title) = Self::window_words(&all, win);
            let app = apps::find_for_window(&all, &win.app_id);
            let used = using::window_uses(&self.using, &all, using::known_plugin, &win.app_id);
            looks.push(Look {
                program: program.clone(),
                icons: app.map(|a| a.icons()).unwrap_or_else(|| vec![apps::generic_icon_for(&win.app_id, &win.app_id)]),
                name: open_windows::tab_words(&if title.is_empty() { program } else { title }, &used),
                front: win.active && !win.minimized,
                hot: self.win_hover.as_ref() == Some(&win.handle),
                uses: used.iter().map(|u| u.kind).collect(),
            });
        }
        looks
    }

    fn ring_drawing(ring: &open_windows::Ring, looks: &[Look], handles: Vec<Handle>, (w, h): (f32, f32)) -> Node<Msg> {
        let centre = (w / 2.0, h / 2.0);
        let dot = crate::toolkit::theme::sheet().ring_windows_dot;
        let drawn = Rc::new(ring.clone());
        let (r, l) = (drawn.clone(), looks.to_vec());
        let (r2, hs) = (drawn.clone(), handles);
        drawing((w, h), t!("ring-open-windows"), move |pm, pen| open_windows::paint(pm, pen, &r, &l, centre, dot))
            .width(Length::Fill)
            .height(Length::Fill)
            .on_hover()
            .on_pointer(move |p| {
                let (r, hs) = (r2.clone(), hs.clone());
                let at = |x: f32, y: f32| r.part_at((x - centre.0) as f64, (y - centre.1) as f64).map(|i| hs[i].clone());
                match p {
                    Pointer::Hover(x, y) => Face::hovering(at(x, y)),
                    Pointer::Leave => Face::hovering(None),
                    _ => msg(move |f| f.pointer_on_ring(&r, &hs, centre, p)),
                }
            })
    }

    fn window_button(ring: &open_windows::Ring, i: usize, look: &Look, handle: Handle, at: [f32; 4]) -> Node<Msg> {
        let (on_click, on_key, on_point) = (handle.clone(), handle.clone(), handle);
        open_windows::tab_node(ring, i, look)
            .pressable(msg(move |f| f.window_click(&on_click)))
            .on_hover()
            .on_keys(move |input| {
                let h = on_key.clone();
                match input {
                    Input::Focus(true) => Some(Face::hovering(Some(h))),
                    Input::Focus(false) => Some(Face::left_focus(h)),
                    Input::Key { sym: Keysym::Return | Keysym::space, .. } => Some(msg(move |f| f.window_click(&h))),
                    Input::Key { sym, mods, .. } if opens_menu(*sym, mods.shift) => Some(msg(move |f| f.win_menu = Some((h.clone(), at)))),
                    _ => None,
                }
            })
            .on_pointer(move |p| {
                let h = on_point.clone();
                match p {
                    Pointer::Hover(..) => Face::hovering(Some(h)),
                    Pointer::Leave => Face::hovering(None),
                    _ => msg(move |f| f.pointer_on_window(&h, at, p)),
                }
            })
    }

    fn pointer_on_ring(&mut self, ring: &open_windows::Ring, hs: &[Handle], centre: (f32, f32), p: Pointer) {
        let at = |x: f32, y: f32| ring.part_at((x - centre.0) as f64, (y - centre.1) as f64);
        match p {
            Pointer::Hover(x, y) => self.hover_window(at(x, y).map(|i| hs[i].clone())),
            Pointer::Leave => self.hover_window(None),
            Pointer::Down(x, y) => self.win_press = at(x, y).map(|i| hs[i].clone()),
            Pointer::Up(x, y) => match at(x, y) {
                Some(i) => {
                    if self.win_press.take().as_ref() == Some(&hs[i]) {
                        self.window_click(&hs[i]);
                    }
                }
                None => {
                    self.win_press = None;
                    self.background_click(x, y);
                }
            },
            Pointer::Menu(x, y) => {
                if let Some(i) = at(x, y) {
                    let b = ring.button_of(i);
                    self.open_window_menu(&hs[i], [centre.0 + b[0] as f32, centre.1 + b[1] as f32, b[2] as f32, b[3] as f32]);
                }
            }
            _ => {}
        }
    }

    fn pointer_on_window(&mut self, h: &Handle, at: [f32; 4], p: Pointer) {
        match p {
            Pointer::Down(_, _) => self.win_press = Some(h.clone()),
            Pointer::Up(_, _) => {
                if self.win_press.take().as_ref() == Some(h) {
                    self.window_click(h);
                }
            }
            Pointer::Menu(_, _) => self.open_window_menu(h, at),
            Pointer::Hover(_, _) => self.hover_window(Some(h.clone())),
            Pointer::Leave => self.hover_window(None),
            _ => {}
        }
    }

    fn window_click(&mut self, h: &Handle) {
        let Some(ws) = self.windows.clone() else { return };
        if let Some(w) = ws.windows().into_iter().find(|w| &w.handle == h) {
            ws.activate(&w);
            self.close();
        }
    }

    fn open_window_menu(&mut self, h: &Handle, at: [f32; 4]) {
        self.press = None;
        self.win_press = None;
        self.close_menu();
        self.win_menu = Some((h.clone(), at));
    }

    fn hovering(h: Option<Handle>) -> Msg {
        let now = h.clone();
        Msg::unless(move |f| f.hover_window(h.clone()), move |f| f.win_hover == now)
    }

    fn left_focus(h: Handle) -> Msg {
        Msg::unless(move |f| f.hover_window(None), move |f| f.win_hover.as_ref() != Some(&h))
    }

    fn hover_window(&mut self, h: Option<Handle>) {
        if self.win_hover == h {
            return;
        }
        self.win_hover = h.clone();
        match h {
            Some(h) => {
                if self.watching.as_ref() != Some(&h) {
                    self.start_watching(h);
                }
            }
            None => {
                let (ctx, turn) = (self.ctx.clone(), self.pic_turn);
                self.ctx.lp.after(WINDOW_PICTURE_KEPT, move || {
                    ctx.face(move |f| if f.win_hover.is_none() && f.pic_turn == turn { f.stop_watching() })
                });
            }
        }
    }

    fn start_watching(&mut self, h: Handle) {
        self.pic_turn += 1;
        self.watching = Some(h);
        self.win_picture = None;
        let turn = self.pic_turn;
        self.snap(turn);
    }

    pub(super) fn stop_watching(&mut self) {
        self.pic_turn += 1;
        self.watching = None;
        self.win_picture = None;
    }

    fn snap(&mut self, turn: u64) {
        let (Some(h), Some(ws)) = (self.watching.clone(), self.windows.clone()) else { return };
        let wins = ws.windows();
        let Some(at) = wins.iter().position(|w| w.handle == h) else { return self.stop_watching() };
        let win = &wins[at];
        let nth = wins[..at].iter().filter(|w| w.app_id == win.app_id && w.title == win.title).count();
        if self.capture.is_none() && !self.capture_failed {
            self.capture = Capture::connect(&self.ctx.lp);
            self.capture_failed = self.capture.is_none();
        }
        if let Some(cap) = self.capture.clone() {
            if let Some(id) = open_windows::capture_id(&cap.windows(), &win.app_id, &win.title, nth) {
                let px = (crate::toolkit::theme::sheet().ring_windows_picture * 2.0).ceil() as u32;
                let ctx = self.ctx.clone();
                cap.snapshot(&Source::Window(id), px, move |picture| {
                    if let Some(p) = picture {
                        ctx.face(move |f| if f.pic_turn == turn { f.win_picture = Some(Rc::new(p)) });
                    }
                });
            }
        }
        let ctx = self.ctx.clone();
        self.ctx.lp.after(WINDOW_PICTURE_EVERY, move || ctx.face(move |f| if f.pic_turn == turn { f.snap(turn) }));
    }

    pub(super) fn middle_picture(&self, wins: Option<&[Window]>) -> Option<Node<Msg>> {
        if !pointing_shows_picture(self.win_hover.is_some(), self.win_menu.is_some(), self.power, self.quick.is_open()) {
            return None;
        }
        let hover = self.win_hover.as_ref()?;
        let win = wins?.iter().find(|w| &w.handle == hover)?;
        let all = self.ctx.apps();
        let (program, title) = Self::window_words(&all, win);
        let icons = apps::find_for_window(&all, &win.app_id).map(|a| a.icons()).unwrap_or_else(|| vec![apps::generic_icon_for(&win.app_id, &win.app_id)]);
        let picture = if self.watching.as_ref() == Some(hover) { self.win_picture.clone() } else { None };
        let shown = open_windows::without_program(&title, &program);
        let name = if title.is_empty() { program } else { format!("{program}, {title}") };
        Some(open_windows::middle(CENTRE as f32, open_windows::Middle { name, title: shown, show_title: self.win_title, icons, picture }))
    }

    pub(super) fn window_menu_card(&self, h: &Handle) -> Node<Msg> {
        let Some(ws) = &self.windows else { return space() };
        let Some(win) = ws.windows().into_iter().find(|w| &w.handle == h) else { return space() };
        let first = if win.minimized { button("window-restore-symbolic", t!("ring-window-restore")) } else { button("window-minimize-symbolic", t!("ring-window-minimise")) };
        let second = if win.maximized { button("window-restore-symbolic", t!("ring-window-restore-size")) } else { button("window-maximize-symbolic", t!("ring-window-maximise")) };
        let third = if ws.force_offered(&win) {
            button("process-stop-symbolic", t!("ring-window-force-close")).face(|canvas, r, _, pen| {
                pen.icons.draw(canvas, "process-stop-symbolic", r.x as i32, r.y as i32, r.w as u32, 1, pen.theme.danger);
            })
        } else {
            button("window-close-symbolic", t!("ring-close"))
        };
        let size = menu_button();
        let press = |b: Node<Msg>, which: WinAct| {
            let h = h.clone();
            b.size(size).on_press(msg(move |f| f.window_do(&h, which)))
        };
        let all = self.ctx.apps();
        let (program, what) = Self::window_words(&all, &win);
        let name = if what.is_empty() { program } else { what };
        let mut tools = vec![press(first, WinAct::Minimise), press(second, WinAct::Maximise), press(third, WinAct::Close)];
        let mut items = vec![title(open_windows::shown_title(&name).chars().take(MENU_TITLE_CHARS).collect::<String>())];
        let mut full = false;
        if let Some(fav) = Self::window_favourite(&all, &win.app_id, &self.favourites) {
            full = !fav && self.favourites.len() >= store::MAX_FAVOURITES;
            let words = if fav { t!("ring-star-remove") } else { t!("ring-star-add") };
            let star = button(if fav { "starred-symbolic" } else { "non-starred-symbolic" }, words).on(fav).off(full);
            tools.push(press(star, WinAct::Star));
        }
        if let Some(app) = apps::find_for_window(&all, &win.app_id) {
            tools.push(startup_button(&app.path).size(size));
        }
        items.push(row(tools).gap(8.0));
        if full {
            items.push(note(t!("ring-middle-full", n = store::MAX_FAVOURITES)));
        }
        card(column(items).gap(8.0).width(Length::Shrink)).width(Length::Shrink).on_escape(msg(|f| f.win_menu = None))
    }

    pub(super) fn window_favourite(all: &[Program], app_id: &str, favourites: &[String]) -> Option<bool> {
        let app = apps::find_for_window(all, app_id)?;
        Some(favourites.contains(&app.id))
    }

    fn window_do(&mut self, h: &Handle, which: WinAct) {
        let Some(ws) = self.windows.clone() else { return };
        let Some(w) = ws.windows().into_iter().find(|w| &w.handle == h) else { return self.win_menu = None };
        match which {
            WinAct::Minimise if w.minimized => ws.activate(&w),
            WinAct::Minimise => ws.minimize(&w),
            WinAct::Maximise => ws.maximize(&w),
            WinAct::Star => {
                let all = self.ctx.apps();
                let Some(app) = apps::find_for_window(&all, &w.app_id) else { return };
                let on = !self.favourites.contains(&app.id);
                if on && self.favourites.len() >= store::MAX_FAVOURITES {
                    return;
                }
                let id = app.id.clone();
                self.set_favourite(&id, on);
            }
            WinAct::Close => {
                let own = kept::own_press();
                if !own && ws.force_offered(&w) {
                    crate::toolkit::window::refuse();
                }
                if ws.close_asked(&w, own) {
                    let (ctx, h) = (self.ctx.clone(), h.clone());
                    self.ctx.lp.after(FORCE_AFTER + Duration::from_millis(200), move || {
                        ctx.face(move |f| {
                            let (Some(ws), Some((open, at))) = (f.windows.clone(), f.win_menu.clone()) else { return };
                            if open == h && ws.windows().iter().any(|w| w.handle == h) {
                                f.win_menu = Some((h, at));
                            }
                        })
                    });
                }
                return;
            }
        }
        self.win_menu = None;
    }
}
