// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! What the keyboard shortcuts and the window buttons ask for: start a
//! program, close, fill the screen, fill half of it, go full screen, and step
//! through the open windows.

use std::cell::RefCell;
use std::process::{Command, Stdio};

use smithay::desktop::{layer_map_for_output, space::SpaceElement};
use smithay::output::Output;
use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel;
use smithay::utils::{Logical, Point, Rectangle};

use crate::config::Action;
use crate::focus::KeyboardFocusTarget;
use crate::shell::WindowElement;
use crate::state::{AnvilState, Backend};

pub(crate) const READER_COMMAND: &str = "trindle-settings --screen-reader";

thread_local! {
    static PLUGIN_KEYS: RefCell<crate::limits::Rate> =
        RefCell::new(crate::limits::Rate::new(crate::limits::SHORTCUTS_PER_SECOND, std::time::Duration::from_secs(1)));
}

#[derive(Default)]
struct Before(RefCell<Option<Rectangle<i32, Logical>>>);

#[derive(Default)]
struct Squeezed(RefCell<Kept>);

type Kept = Option<(Rectangle<i32, Logical>, Point<i32, Logical>)>;

pub fn kept_here(kept: Kept, now_at: Point<i32, Logical>) -> Kept {
    kept.filter(|(_, left)| *left == now_at)
}

pub fn put_back(kept: Kept, room: Rectangle<i32, Logical>) -> Option<Rectangle<i32, Logical>> {
    kept.map(|(before, _)| before).filter(|b| b.loc.y + b.size.h <= room.loc.y + room.size.h)
}

#[derive(Default)]
struct Fitted(RefCell<Option<Rectangle<i32, Logical>>>);

pub fn squeezed(frame: Rectangle<i32, Logical>, room: Rectangle<i32, Logical>, screen: Rectangle<i32, Logical>) -> Option<Rectangle<i32, Logical>> {
    let foot = room.loc.y + room.size.h;
    if !screen.overlaps(frame) || frame.loc.y + frame.size.h <= foot {
        return None;
    }
    let y = (foot - frame.size.h).max(room.loc.y);
    let h = (foot - y).max(1);
    Some(Rectangle::new((frame.loc.x, y).into(), (frame.size.w, h).into()))
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn do_action(&mut self, action: Action) {
        let focused = self.focused_window();
        match action {
            Action::Execute(cmd) => match crate::level::kind_of(&cmd) {
                Some(kind) => {
                    if let Some(child) = self.start(&cmd) {
                        crate::level::show_after(child, kind);
                    }
                }
                None => self.spawn(&cmd),
            },
            Action::Plugin(cmd) => {
                let allowed = PLUGIN_KEYS.with(|r| r.borrow_mut().allow());
                if self.capture.from_box {
                    tracing::warn!("a plugin shortcut was typed by a plugin's made-up keyboard: left unrun");
                } else if allowed {
                    self.spawn(&cmd);
                } else {
                    tracing::warn!("a plugin shortcut was left unrun: too many in a second");
                }
            }
            Action::Close => focused.iter().for_each(|w| self.close_window(w)),
            Action::ToggleMaximize => {
                if let Some(w) = focused { self.toggle_maximize(&w) }
            }
            Action::ToggleFullscreen => {
                if let Some(w) = focused { self.toggle_fullscreen(&w) }
            }
            Action::SnapLeft | Action::SnapRight => {
                if let Some(w) = focused { self.snap(&w, action == Action::SnapLeft) }
            }
            Action::NextWindow => self.cycle_windows(true),
            Action::PreviousWindow => self.cycle_windows(false),
            Action::Iconify => {
                if let Some(surface) = focused.and_then(|w| w.wl_surface().map(|s| s.into_owned())) {
                    self.set_window_minimized(&surface, true);
                }
            }
            Action::RestartParts => {
                let env = self.socket_name.iter().map(|n| ("WAYLAND_DISPLAY".to_string(), n.clone()))
                    .chain(self.xdisplay.map(|x| ("DISPLAY".to_string(), format!(":{x}")))).collect();
                crate::restart::parts(env);
            }
            Action::RestartAll => self.spawn(crate::restart::ASK_COMMAND),
            Action::Exit => self.running.store(false, std::sync::atomic::Ordering::SeqCst),
            Action::Nothing => {}
        }
    }

    pub fn spawn(&mut self, command: &str) {
        if let Some(child) = self.start(command) {
            crate::session::let_go(child);
        }
    }

    fn start(&mut self, command: &str) -> Option<std::process::Child> {
        if self.ring_asked_for(command) || self.screenshot_asked_for(command) {
            return None;
        }
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg(command).stdin(Stdio::null());
        crate::session::usual_files(&mut cmd);
        cmd.env_remove(crate::own_helpers::OWN_KEY);
        if self.ring.by_hand && command.trim() == crate::wm::READER_COMMAND {
            cmd.env(crate::own_helpers::OWN_KEY, "1");
        }
        if let Some(name) = &self.socket_name {
            cmd.env("WAYLAND_DISPLAY", name);
        }
        if let Some(x) = self.xdisplay {
            cmd.env("DISPLAY", format!(":{x}"));
        }
        match cmd.spawn() {
            Ok(child) => Some(child),
            Err(e) => {
                tracing::warn!(command, "could not start: {e}");
                None
            }
        }
    }

    pub fn focused_window(&self) -> Option<WindowElement> {
        match self.seat.get_keyboard()?.current_focus()? {
            KeyboardFocusTarget::Window(w) => self.space.elements().find(|e| e.0 == w).cloned(),
            _ => None,
        }
    }

    pub fn close_window(&self, window: &WindowElement) {
        if let Some(toplevel) = window.0.toplevel() {
            toplevel.send_close();
        } else if let Some(x11) = window.0.x11_surface() {
            let _ = x11.close();
        }
    }

    fn usable_area(&self, window: &WindowElement) -> Option<(Output, Rectangle<i32, Logical>)> {
        let output = self.space.outputs_for_element(window).into_iter().next()
            .or_else(|| self.space.outputs().next().cloned())?;
        let geo = self.space.output_geometry(&output)?;
        let zone = layer_map_for_output(&output).non_exclusive_zone();
        Some((output, Rectangle::new(geo.loc + zone.loc, zone.size)))
    }

    fn is_maximized(window: &WindowElement) -> bool {
        match (window.0.toplevel(), window.0.x11_surface()) {
            (Some(t), _) => t.current_state().states.contains(xdg_toplevel::State::Maximized),
            (_, Some(x)) => x.is_maximized(),
            _ => false,
        }
    }

    fn place(&mut self, window: &WindowElement, area: Rectangle<i32, Logical>, maximized: bool) {
        let gap = self.config.gap;
        let area = Rectangle::new(
            (area.loc.x + gap, area.loc.y + gap).into(),
            ((area.size.w - 2 * gap).max(1), (area.size.h - 2 * gap).max(1)).into(),
        );
        let before = window.0.user_data().get_or_insert(Before::default);
        if before.0.borrow().is_none() {
            if let Some(loc) = self.space.element_location(window) {
                *before.0.borrow_mut() = Some(Rectangle::new(loc, window.client_size()));
            }
        }
        let extra = window.deco_extra();
        let content = Rectangle::new(
            area.loc + window.deco_offset(),
            ((area.size.w - extra.w).max(1), (area.size.h - extra.h).max(1)).into(),
        );
        if let Some(toplevel) = window.0.toplevel() {
            toplevel.with_pending_state(|state| {
                if maximized {
                    state.states.set(xdg_toplevel::State::Maximized);
                } else {
                    state.states.unset(xdg_toplevel::State::Maximized);
                }
                state.size = Some(content.size);
            });
            if toplevel.is_initial_configure_sent() {
                toplevel.send_pending_configure();
            }
        } else if let Some(x11) = window.0.x11_surface() {
            let _ = x11.set_maximized(maximized);
            let _ = x11.configure(content);
        }
        self.space.map_element(window.clone(), area.loc, true);
    }

    fn restore(&mut self, window: &WindowElement, at: Option<Point<i32, Logical>>) {
        let before = window.0.user_data().get_or_insert(Before::default);
        let was = before.0.borrow_mut().take();
        let loc = at.or(was.map(|r| r.loc));
        if let Some(toplevel) = window.0.toplevel() {
            toplevel.with_pending_state(|state| {
                state.states.unset(xdg_toplevel::State::Maximized);
                state.size = was.map(|r| r.size);
            });
            if toplevel.is_initial_configure_sent() {
                toplevel.send_pending_configure();
            }
        } else if let Some(x11) = window.0.x11_surface() {
            let _ = x11.set_maximized(false);
            if let (Some(r), Some(loc)) = (was, loc) {
                let _ = x11.configure(Rectangle::new(loc + window.deco_offset(), r.size));
            }
        }
        if let Some(loc) = loc {
            self.space.map_element(window.clone(), loc, true);
        }
    }

    fn is_placed(window: &WindowElement) -> bool {
        let snapped = window.0.user_data().get::<Before>().is_some_and(|b| b.0.borrow().is_some());
        Self::is_maximized(window) || snapped
    }

    pub fn toggle_maximize(&mut self, window: &WindowElement) {
        if Self::is_placed(window) {
            self.restore(window, None);
        } else {
            self.maximize(window);
        }
    }

    pub fn fit_windows_to_room(&mut self) {
        let outputs: Vec<Output> = self.space.outputs().cloned().collect();
        for output in outputs {
            let Some(screen) = self.space.output_geometry(&output) else { continue };
            let zone = layer_map_for_output(&output).non_exclusive_zone();
            let room = Rectangle::new(screen.loc + zone.loc, zone.size);
            let fitted = output.user_data().get_or_insert(Fitted::default);
            let was = fitted.0.replace(Some(room));
            if was.is_none_or(|w| w == room) {
                continue;
            }
            let windows: Vec<WindowElement> = self.space.elements()
                .filter(|w| self.space.outputs_for_element(w).contains(&output))
                .cloned().collect();
            for window in windows {
                self.fit_one(&window, room, screen);
            }
        }
    }

    pub fn fit_window_to_room(&mut self, window: &WindowElement) {
        let Some(output) = self.space.outputs_for_element(window).first().cloned() else { return };
        let Some(screen) = self.space.output_geometry(&output) else { return };
        let zone = layer_map_for_output(&output).non_exclusive_zone();
        self.fit_one(window, Rectangle::new(screen.loc + zone.loc, zone.size), screen);
    }

    fn fit_one(&mut self, window: &WindowElement, room: Rectangle<i32, Logical>, screen: Rectangle<i32, Logical>) {
        if window.is_fullscreen() {
            return;
        }
        if Self::is_maximized(window) {
            self.place(window, room, true);
            return;
        }
        let Some(loc) = self.space.element_location(window) else { return };
        let frame = Rectangle::new(loc, window.geometry().size);
        let kept = window.0.user_data().get_or_insert(Squeezed::default);
        let saved = kept_here(*kept.0.borrow(), loc);
        let to = match put_back(saved, room) {
            Some(before) => {
                kept.0.replace(None);
                Some(before)
            }
            None => {
                let to = squeezed(frame, room, screen);
                let next = match (to, saved) {
                    (Some(to), Some((before, _))) => Some((before, to.loc)),
                    (Some(to), None) => Some((frame, to.loc)),
                    (None, saved) => saved,
                };
                kept.0.replace(next);
                to
            }
        };
        if let Some(to) = to {
            self.set_frame(window, to);
        }
    }

    fn set_frame(&mut self, window: &WindowElement, frame: Rectangle<i32, Logical>) {
        let extra = window.deco_extra();
        let content = Rectangle::new(
            frame.loc + window.deco_offset(),
            ((frame.size.w - extra.w).max(1), (frame.size.h - extra.h).max(1)).into(),
        );
        if let Some(toplevel) = window.0.toplevel() {
            if window.client_size() != content.size {
                toplevel.with_pending_state(|state| state.size = Some(content.size));
                if toplevel.is_initial_configure_sent() {
                    toplevel.send_pending_configure();
                }
            }
        } else if let Some(x11) = window.0.x11_surface() {
            let _ = x11.configure(content);
        }
        self.space.map_element(window.clone(), frame.loc, false);
    }

    pub fn maximize(&mut self, window: &WindowElement) {
        if let Some((_, area)) = self.usable_area(window) {
            self.place(window, area, true);
        }
    }

    pub fn unmaximize(&mut self, window: &WindowElement) {
        if Self::is_placed(window) {
            self.restore(window, None);
        }
    }

    pub fn restore_for_drag(&mut self, window: &WindowElement, pointer: Point<f64, Logical>) -> Option<Point<i32, Logical>> {
        if !Self::is_placed(window) {
            return None;
        }
        let loc = self.space.element_location(window)?;
        let width = window.geometry().size.w.max(1) as f64;
        let across = ((pointer.x - loc.x as f64) / width).clamp(0.0, 1.0);
        let was = window.0.user_data().get::<Before>().and_then(|b| *b.0.borrow());
        let new_width = was.map_or(width, |r| (r.size.w + window.deco_extra().w) as f64);
        let at = Point::from(((pointer.x - across * new_width).round() as i32, loc.y));
        self.restore(window, Some(at));
        Some(at)
    }

    pub fn snap(&mut self, window: &WindowElement, left: bool) {
        if let Some((_, area)) = self.usable_area(window) {
            let half = area.size.w / 2;
            let x = if left { area.loc.x } else { area.loc.x + half };
            let w = if left { half } else { area.size.w - half };
            let rect = Rectangle::new((x, area.loc.y).into(), (w, area.size.h).into());
            self.place(window, rect, false);
        }
    }

    pub fn toggle_fullscreen(&mut self, window: &WindowElement) {
        if let Some(toplevel) = window.0.toplevel() {
            let full = toplevel.current_state().states.contains(xdg_toplevel::State::Fullscreen);
            if full {
                smithay::wayland::shell::xdg::XdgShellHandler::unfullscreen_request(self, toplevel.clone());
            } else {
                smithay::wayland::shell::xdg::XdgShellHandler::fullscreen_request(self, toplevel.clone(), None);
            }
        } else if let Some(x11) = window.0.x11_surface().cloned() {
            use smithay::xwayland::XwmHandler;
            let Some(xwm) = x11.xwm_id() else { return };
            if x11.is_fullscreen() {
                self.unfullscreen_request(xwm, x11);
            } else {
                self.fullscreen_request(xwm, x11);
            }
        }
    }

    pub fn cycle_windows(&mut self, forward: bool) {
        if forward {
            if let Some(i) = self.minimized.iter().position(|(w, _)| self.workspace_of(w) == self.active_workspace) {
                let (window, loc) = self.minimized.remove(i);
                self.bring_back(window.clone(), loc);
                self.focus_window(&window);
                return;
            }
        }
        let windows: Vec<WindowElement> = self.space.elements().filter(|w| crate::panel::is_listed(w)).cloned().collect();
        if windows.len() < 2 {
            if let Some(w) = windows.first() {
                self.focus_window(w);
            }
            return;
        }
        if forward {
            let back = windows[0].clone();
            self.focus_window(&back);
        } else {
            let front = windows[windows.len() - 1].clone();
            let rest: Vec<WindowElement> = windows[..windows.len() - 1].to_vec();
            self.space.raise_element(&front, false);
            for w in rest {
                self.space.raise_element(&w, false);
            }
            let top = windows[windows.len() - 2].clone();
            self.focus_window(&top);
        }
    }
}

#[cfg(test)]
mod room_tests {
    use super::*;

    fn r(x: i32, y: i32, w: i32, h: i32) -> Rectangle<i32, Logical> {
        Rectangle::new((x, y).into(), (w, h).into())
    }

    #[test]
    fn a_window_under_the_keyboard_moves_up_then_shrinks() {
        let screen = r(0, 0, 1000, 800);
        let room = r(0, 0, 1000, 500);
        assert_eq!(squeezed(r(10, 400, 300, 200), room, screen), Some(r(10, 300, 300, 200)), "moved up whole");
        assert_eq!(squeezed(r(10, 100, 300, 700), room, screen), Some(r(10, 0, 300, 500)), "too tall, so shorter");
        assert_eq!(squeezed(r(10, 50, 300, 200), room, screen), None, "already fits");
        assert_eq!(squeezed(r(1200, 400, 300, 300), room, screen), None, "on another screen");
        let top_bar = r(0, 30, 1000, 470);
        assert_eq!(squeezed(r(0, 0, 300, 900), top_bar, screen), Some(r(0, 30, 300, 470)), "never above the room's top");
    }

    #[test]
    fn a_squeezed_window_goes_back_unless_the_person_moved_it() {
        let before = r(10, 400, 300, 200);
        let kept = Some((before, (10, 300).into()));
        let big = r(0, 0, 1000, 800);
        let small = r(0, 0, 1000, 500);
        assert_eq!(put_back(kept_here(kept, (10, 300).into()), big), Some(before));
        assert_eq!(put_back(kept_here(kept, (10, 300).into()), small), None, "the room is still too small");
        assert_eq!(kept_here(kept, (200, 120).into()), None, "dragged elsewhere: left where the person put it");
        assert_eq!(put_back(None, big), None);
    }

    #[test]
    fn every_window_fits_above_the_keyboard_each_time_it_comes_out() {
        use crate::rig::{alone, Rig};
        alone("wm::room_tests::every_window_fits_above_the_keyboard_each_time_it_comes_out", |dir| {
            let mut rig = Rig::new(dir);
            let who = rig.join(Default::default());
            let surface = rig.window_first_drawn(who, false, (300, 600));
            let window = rig.state.space.elements().last().cloned().unwrap();
            let screen = rig.state.space.output_geometry(&rig.output).unwrap();
            let frame = |rig: &mut Rig| {
                rig.pump();
                let size = window.0.toplevel().unwrap().with_pending_state(|s| s.size)
                    .unwrap_or_else(|| window.client_size());
                rig.show(who, &surface, size.w, size.h);
                Rectangle::new(rig.state.space.element_location(&window).unwrap(), window.geometry().size)
            };
            let foot = |r: Rectangle<i32, Logical>| r.loc.y + r.size.h;
            let start = frame(&mut rig);
            assert!(foot(start) > 480, "the window reaches under where the keyboard goes: {start:?}");

            let keys = rig.keyboard_panel(who, 320);
            let room = 800 - 320;
            assert!(foot(frame(&mut rig)) <= room, "the keyboard's first time out");

            rig.put_away(&keys);
            assert_eq!(frame(&mut rig), start, "back where it was once the keyboard goes");

            rig.state.space.map_element(window.clone(), (start.loc.x, 0), true);
            rig.show(who, &surface, 300, 800);
            keys.commit();
            rig.pump();
            rig.show(who, &keys, screen.size.w, 320);
            assert!(foot(frame(&mut rig)) <= room, "the keyboard's second time out, after a resize");

            rig.state.maximize(&window);
            let full = frame(&mut rig);
            assert!(foot(full) <= room && full.size.h > room - 100, "filled the room above the keys: {full:?}");
            rig.put_away(&keys);
            let full = frame(&mut rig);
            assert!(foot(full) > room, "filled the whole screen once the keyboard went: {full:?}");
            keys.commit();
            rig.pump();
            rig.show(who, &keys, screen.size.w, 320);
            assert!(foot(frame(&mut rig)) <= room, "filled only the room again when it came back");
        });
    }
}
