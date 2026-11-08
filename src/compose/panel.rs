// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::cell::Cell;

use smithay::{
    desktop::WindowSurface,
    output::Output,
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    utils::{IsAlive, Logical, Point, SERIAL_COUNTER},
    wayland::{
        compositor::with_states,
        shell::xdg::{XdgShellHandler, XdgToplevelSurfaceData},
    },
};
use smithay::xwayland::XwmHandler;
use tracing::{debug, warn};

use crate::{
    focus::KeyboardFocusTarget,
    protocols::{
        workspaces::{self, WorkspacesHandler, WorkspacesState},
        windows::{self, Change, WindowInfo, WindowsHandler, WindowsState},
    },
    shell::{FullscreenSurface, WindowElement},
    state::{AnvilState, Backend},
};

const FORCE_AFTER: std::time::Duration = std::time::Duration::from_secs(4);

const FORCE_UNTIL: std::time::Duration = std::time::Duration::from_secs(120);

fn ours(pid: u32) -> bool {
    use std::os::unix::fs::MetadataExt;
    // SAFETY: getuid cannot fail.
    let me = unsafe { libc::getuid() };
    pid > 1
        && pid != std::process::id()
        && std::fs::metadata(format!("/proc/{pid}")).is_ok_and(|m| m.uid() == me)
}

pub(crate) fn x11_program(conn: &impl x11rb::connection::Connection, window: u32) -> Option<u32> {
    use x11rb::protocol::res::{ClientIdMask, ClientIdSpec, ConnectionExt};
    let spec = ClientIdSpec { client: window, mask: ClientIdMask::LOCAL_CLIENT_PID };
    let ids = conn.res_query_client_ids(&[spec]).ok()?.reply().ok()?.ids;
    ids.iter().find(|id| id.spec.mask == ClientIdMask::LOCAL_CLIENT_PID).and_then(|id| id.value.first().copied())
}

fn end_x11_program(number: u32, window: u32) {
    use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};
    std::thread::spawn(move || {
        let Ok((conn, _)) = x11rb::connect(Some(&format!(":{number}"))) else {
            warn!("force close: could not reach display :{number}");
            return;
        };
        let pid = x11_program(&conn, window);
        if pid.is_some_and(|p| !ours(p)) {
            warn!("force close: no program of this person's to end");
            return;
        }
        let _ = conn.kill_client(window);
        let _ = conn.flush();
        if let Some(pid) = pid {
            // SAFETY: kill only sends a signal.
            unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
        }
    });
}

pub type PutAway = (WindowElement, Point<i32, Logical>);

struct OnWorkspace(Cell<usize>);

pub(crate) fn is_listed(window: &WindowElement) -> bool {
    if let Some(x11) = window.0.x11_surface() {
        return !x11.is_override_redirect();
    }
    let _ = window;
    true
}

fn is_fullscreen(window: &WindowElement) -> bool {
    match window.0.underlying_surface() {
        WindowSurface::Wayland(t) => t.current_state().states.contains(xdg_toplevel::State::Fullscreen),
        WindowSurface::X11(x11) => x11.is_fullscreen(),
    }
}

fn describe(window: &WindowElement, active: bool, minimized: bool, outputs: Vec<Output>) -> Option<WindowInfo> {

    let surface = window.wl_surface()?.into_owned();
    let (title, app_id, maximized, fullscreen, parent) = match window.0.underlying_surface() {
        WindowSurface::Wayland(t) => {
            let (title, app_id) = with_states(t.wl_surface(), |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .map(|data| {
                        let data = data.lock().unwrap();
                        (data.title.clone().unwrap_or_default(), data.app_id.clone().unwrap_or_default())
                    })
                    .unwrap_or_default()
            });
            let states = t.current_state().states;
            (
                title,
                app_id,
                states.contains(xdg_toplevel::State::Maximized),
                states.contains(xdg_toplevel::State::Fullscreen),
                t.parent(),
            )
        }
        WindowSurface::X11(x11) => {

            let class = Some(x11.class()).filter(|c| !c.is_empty()).unwrap_or_else(|| x11.instance());
            (x11.title(), class, x11.is_maximized(), x11.is_fullscreen(), None)
        }
    };
    Some(WindowInfo { surface, title, app_id, active, minimized, maximized, fullscreen, outputs, parent })
}

pub fn capture_size(window: &WindowElement, scale: f64) -> smithay::utils::Size<i32, smithay::utils::Physical> {
    use smithay::desktop::space::SpaceElement;
    let size = SpaceElement::geometry(window).size.to_f64().to_physical(scale).to_i32_round::<i32>();
    (size.w.max(1), size.h.max(1)).into()
}

impl<BackendData: Backend> AnvilState<BackendData> {

    fn force_close(&mut self, window: &WindowElement) {
        use smithay::reexports::wayland_server::Resource;
        let pid = match window.0.underlying_surface() {
            WindowSurface::Wayland(t) => {
                let Some(client) = t.wl_surface().client() else { return };
                if client.get_data::<crate::state::ClientState>().is_some_and(|c| c.from_root) {
                    return;
                }
                let pid = client.get_credentials(&self.display_handle).ok().map(|c| c.pid as u32);

                self.display_handle.backend_handle().kill_client(client.id(),
                    smithay::reexports::wayland_server::backend::DisconnectReason::ConnectionClosed);
                pid
            }
            WindowSurface::X11(x11) => {
                match x11.xwm_id().and_then(|id| self.rooms.display_of(id)) {
                    Some(display) => end_x11_program(display, x11.window_id()),
                    None => warn!("force close: this window's Xwayland has stopped"),
                }
                return;
            }
        };
        let Some(pid) = pid.filter(|&p| ours(p)) else {
            warn!("force close: no program of this person's to end");
            return;
        };
        // SAFETY: kill only sends a signal.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
    }

    pub fn refresh_panel(&mut self) {

        self.minimized.retain(|(w, _)| w.alive());
        self.hidden.retain(|(w, _)| w.alive());
        self.refocus_if_lost();

        let windows = self.toplevel_infos();
        self.refresh_toplevel_list(&windows);
        windows::refresh(self, windows);
        self.image_capture_state.refresh();

        let (count, showing) = (self.workspaces, self.active_workspace);
        workspaces::refresh(self, count, showing);
    }

    fn toplevel_infos(&self) -> Vec<WindowInfo> {
        let showing: Vec<&WindowElement> = self.space.elements().filter(|w| is_listed(w)).collect();

        let top = showing.last().copied();
        let mut infos: Vec<WindowInfo> = showing
            .iter()
            .filter_map(|w| {
                describe(w, Some(*w) == top, false, self.space.outputs_for_element(w))
            })
            .collect();
        let put_away = self.minimized.iter().map(|p| (p, true)).chain(self.hidden.iter().map(|p| (p, false)));
        for ((window, loc), minimized) in put_away {
            let outputs = self.output_at(*loc).into_iter().collect();
            infos.extend(describe(window, false, minimized, outputs));
        }
        infos
    }

    fn refresh_toplevel_list(&mut self, windows: &[WindowInfo]) {
        let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut self.toplevel_list)
            .into_iter()
            .partition(|(surface, _)| !windows.iter().any(|w| w.surface == *surface));
        for (_, handle) in gone {
            self.toplevel_list_state.remove_toplevel(&handle);
        }
        self.toplevel_list = kept;
        for info in windows {
            match self.toplevel_list.iter().find(|(s, _)| *s == info.surface) {
                Some((_, handle)) => {
                    let mut changed = false;
                    if handle.title() != info.title {
                        handle.send_title(&info.title);
                        changed = true;
                    }
                    if handle.app_id() != info.app_id {
                        handle.send_app_id(&info.app_id);
                        changed = true;
                    }
                    if changed {
                        handle.send_done();
                    }
                }
                None => {
                    let handle = self.toplevel_list_state.new_toplevel::<Self>(info.title.clone(), info.app_id.clone());
                    self.toplevel_list.push((info.surface.clone(), handle));
                }
            }
        }
    }

    pub fn window_of(
        &self,
        handle: &smithay::wayland::foreign_toplevel_list::ForeignToplevelHandle,
    ) -> Option<WindowElement> {
        let id = handle.identifier();
        let surface = &self.toplevel_list.iter().find(|(_, h)| h.identifier() == id)?.0;
        let is = |w: &&WindowElement| w.wl_surface().is_some_and(|s| *s == *surface);
        self.space
            .elements()
            .find(is)
            .or_else(|| self.minimized.iter().chain(self.hidden.iter()).map(|(w, _)| w).find(is))
            .cloned()
    }

    pub fn window_place(&self, window: &WindowElement) -> Point<i32, Logical> {
        self.space
            .element_location(window)
            .or_else(|| {
                self.minimized.iter().chain(self.hidden.iter()).find(|(w, _)| w == window).map(|(_, at)| *at)
            })
            .unwrap_or_default()
    }

    pub fn capture_windows(&self) -> Vec<(String, WindowElement, smithay::utils::Rectangle<i32, Logical>, f64)> {
        use smithay::desktop::space::SpaceElement;
        self.toplevel_list
            .iter()
            .filter_map(|(_, handle)| {
                let window = self.window_of(handle)?;
                let area = smithay::utils::Rectangle::new(self.window_place(&window), SpaceElement::geometry(&window).size);
                let scale = self.window_scale(&window);
                Some((handle.identifier(), window, area, scale))
            })
            .collect()
    }

    pub fn window_scale(&self, window: &WindowElement) -> f64 {
        self.space
            .outputs_for_element(window)
            .first()
            .cloned()
            .or_else(|| self.output_at(self.window_place(window)))
            .map(|o| o.current_scale().fractional_scale())
            .unwrap_or(1.0)
    }

    fn output_at(&self, loc: Point<i32, Logical>) -> Option<Output> {
        self.space
            .output_under(loc.to_f64())
            .next()
            .or_else(|| self.space.outputs().next())
            .cloned()
    }

    pub fn window_owners(&self) -> Vec<(u32, String)> {
        use smithay::reexports::wayland_server::Resource;
        let shown = self.space.elements().filter(|w| is_listed(w));
        let away = self.minimized.iter().chain(self.hidden.iter()).map(|(w, _)| w);
        shown
            .chain(away)
            .filter_map(|w| {
                let info = describe(w, false, false, Vec::new())?;
                let pid = match w.0.underlying_surface() {
                    WindowSurface::Wayland(t) => {
                        t.wl_surface().client()?.get_credentials(&self.display_handle).ok().map(|c| c.pid as u32)
                    }
                    WindowSurface::X11(x11) => x11.pid(),
                }?;
                Some((pid, info.app_id))
            })
            .collect()
    }

    pub fn place_on_active_workspace(&self, window: &WindowElement) {
        let active = self.active_workspace;
        window.user_data().insert_if_missing(|| OnWorkspace(Cell::new(active)));
    }

    pub(crate) fn workspace_of(&self, window: &WindowElement) -> usize {
        self.place_on_active_workspace(window);
        window.user_data().get::<OnWorkspace>().map_or(0, |w| w.0.get())
    }

    fn find_window(&self, surface: &WlSurface) -> Option<WindowElement> {
        self.window_for_surface(surface).or_else(|| {
            self.minimized
                .iter()
                .chain(&self.hidden)
                .map(|(w, _)| w)
                .find(|w| w.wl_surface().as_deref() == Some(surface))
                .cloned()
        })
    }

    fn put_away(&mut self, window: &WindowElement, refocus: bool) -> Option<Point<i32, Logical>> {
        let loc = self.space.element_location(window)?;
        for output in self.space.outputs().cloned().collect::<Vec<_>>() {
            if let Some(slot) = output.user_data().get::<FullscreenSurface>() {
                if slot.get().as_ref() == Some(window) {
                    slot.clear();
                    self.backend_data.reset_buffers(&output);
                }
            }
        }
        self.space.unmap_elem(window);

        match window.0.underlying_surface() {
            WindowSurface::Wayland(t) => {
                t.with_pending_state(|s| s.states.unset(xdg_toplevel::State::Activated));
                if t.is_initial_configure_sent() {
                    t.send_pending_configure();
                }
            }
            WindowSurface::X11(x11) => {
                let _ = x11.set_activated(false);
            }
        }

        let had_keyboard = self
            .seat
            .get_keyboard()
            .and_then(|k| k.current_focus())
            .is_some_and(|f| f == KeyboardFocusTarget::Window(window.0.clone()));
        if refocus && had_keyboard {
            self.focus_topmost();
        }
        Some(loc)
    }

    pub(crate) fn bring_back(&mut self, window: WindowElement, loc: Point<i32, Logical>) {
        self.space.map_element(window.clone(), loc, false);
        if is_fullscreen(&window) {
            if let Some(output) = self.output_at(loc) {
                output.user_data().insert_if_missing(FullscreenSurface::default);
                output.user_data().get::<FullscreenSurface>().unwrap().set(window);
            }
        } else {
            self.fit_window_to_room(&window);
        }
    }

    pub fn focus_window(&mut self, window: &WindowElement) {
        if self.is_locked() {
            self.focus_after_unlock(window.clone().into());
            return;
        }

        if self.the_modal().is_some() {
            return;
        }
        self.window_before_panel = None;
        self.space.raise_element(window, true);
        if let Some(x11) = window.0.x11_surface() {
            self.raise_x11(x11);
            if x11.is_minimized() {
                let _ = x11.set_iconic(false);
            }
        }
        self.fill_follows_front(window);

        for dialog in crate::shell::dialogs_of(&self.space, window) {
            self.space.raise_element(&dialog, false);
            if let Some(x11) = dialog.0.x11_surface() {
                self.raise_x11(x11);
            }
        }

        for w in self.space.elements() {
            if let WindowSurface::Wayland(t) = w.0.underlying_surface() {
                if t.is_initial_configure_sent() {
                    t.send_pending_configure();
                }
            }
        }
        if let Some(keyboard) = self.seat.get_keyboard() {
            keyboard.set_focus(self, Some(window.clone().into()), SERIAL_COUNTER.next_serial());
        }
    }

    fn fill_follows_front(&mut self, window: &WindowElement) {
        let on = self.space.outputs_for_element(window);
        for output in &on {
            let Some(slot) = output.user_data().get::<FullscreenSurface>() else { continue };
            if slot.get().is_some_and(|f| &f != window) {
                slot.clear();
                self.backend_data.reset_buffers(output);
            }
        }
        if is_fullscreen(window) {
            if let Some(output) = on.first() {
                output.user_data().insert_if_missing(FullscreenSurface::default);
                output.user_data().get::<FullscreenSurface>().unwrap().set(window.clone());
            }
        }
    }

    pub(crate) fn opens_behind_full_screen(&self, window: &WindowElement) -> bool {
        let program = |w: &WindowElement| match w.0.underlying_surface() {
            WindowSurface::Wayland(t) => {
                self.display_handle.get_client(smithay::reexports::wayland_server::Resource::id(t.wl_surface())).ok().map(|c| c.id())
            }
            WindowSurface::X11(_) => None,
        };
        self.space.outputs_for_element(window).iter().any(|o| {
            o.user_data()
                .get::<FullscreenSurface>()
                .and_then(|f| f.get())
                .is_some_and(|f| &f != window && program(&f) != program(window))
        })
    }

    pub fn focus_topmost(&mut self) {
        let top = self.space.elements().rev().find(|w| is_listed(w)).cloned();
        match top {
            Some(window) => self.focus_window(&window),
            None if self.is_locked() => {}
            None => self.give_keyboard(None, SERIAL_COUNTER.next_serial()),
        }
    }

    fn tab_for(&self, window: &WindowElement) -> Option<Point<f64, Logical>> {
        let output = self.space.outputs_for_element(window).into_iter().next()
            .or_else(|| self.space.outputs().next().cloned())?;
        let geo = self.space.output_geometry(&output)?;
        Some(crate::animate::tab_point(geo))
    }

    fn refocus_if_lost(&mut self) {
        let lost = self
            .seat
            .get_keyboard()
            .and_then(|k| k.current_focus())
            .is_some_and(|f| !f.alive());
        if lost {
            self.focus_topmost();
        }
    }

    pub fn set_window_minimized(&mut self, surface: &WlSurface, minimized: bool) {
        let Some(window) = self.find_window(surface) else { return };
        if minimized {
            if self.minimized.iter().any(|(w, _)| w == &window) {
                return;
            }
            let tab = self.tab_for(&window);
            if let Some(loc) = self.put_away(&window, true) {
                if let Some(tab) = tab {
                    crate::animate::minimizing(&window, tab);
                }
                self.minimized.push((window, loc));
            } else if let Some(i) = self.hidden.iter().position(|(w, _)| w == &window) {
                let entry = self.hidden.remove(i);
                self.minimized.push(entry);
            }
        } else if let Some(i) = self.minimized.iter().position(|(w, _)| w == &window) {
            let (window, loc) = self.minimized.remove(i);
            if self.workspace_of(&window) == self.active_workspace {
                self.bring_back(window.clone(), loc);
                if let Some(tab) = self.tab_for(&window) {
                    crate::animate::restoring(&window, tab);
                }
                self.focus_window(&window);
            } else {
                self.hidden.push((window, loc));
            }
        }
    }

    pub fn go_to_workspace(&mut self, index: usize) {
        if index >= self.workspaces || index == self.active_workspace {
            return;
        }

        let leaving: Vec<WindowElement> = self
            .space
            .elements()
            .filter(|w| is_listed(w) && self.workspace_of(w) != index)
            .cloned()
            .collect();
        for window in leaving {
            if let Some(loc) = self.put_away(&window, false) {
                self.hidden.push((window, loc));
            }
        }

        self.active_workspace = index;
        let (back, others): (Vec<PutAway>, Vec<PutAway>) =
            std::mem::take(&mut self.hidden).into_iter().partition(|(w, _)| self.workspace_of(w) == index);
        self.hidden = others;
        for (window, loc) in back {
            self.bring_back(window, loc);
        }
        self.focus_topmost();
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {

    pub fn set_workspace_count(&mut self, count: usize) {
        let count = count.max(1);
        if count == self.workspaces {
            return;
        }
        let last = count - 1;
        if self.active_workspace > last {
            self.go_to_workspace(last);
        }
        let everyone: Vec<WindowElement> = self.space.elements().cloned()
            .chain(self.minimized.iter().map(|(w, _)| w.clone()))
            .chain(self.hidden.iter().map(|(w, _)| w.clone()))
            .collect();
        for w in everyone {
            if let Some(on) = w.user_data().get::<OnWorkspace>() {
                if on.0.get() > last {
                    on.0.set(last);
                }
            }
        }
        self.workspaces = count;
        let active = self.active_workspace;
        let (back, others): (Vec<PutAway>, Vec<PutAway>) =
            std::mem::take(&mut self.hidden).into_iter().partition(|(w, _)| self.workspace_of(w) == active);
        self.hidden = others;
        for (window, loc) in back {
            self.bring_back(window, loc);
        }
    }
}

impl<BackendData: Backend> WindowsHandler for AnvilState<BackendData> {
    fn windows_state(&mut self) -> &mut WindowsState {
        &mut self.windows_state
    }

    fn raise_window(&mut self, wl_surface: WlSurface) {
        let Some(window) = self.find_window(&wl_surface) else { return };
        let workspace = self.workspace_of(&window);
        self.go_to_workspace(workspace);
        if let Some(i) = self.minimized.iter().position(|(w, _)| w == &window) {
            let (window, loc) = self.minimized.remove(i);
            self.bring_back(window, loc);
        }
        self.focus_window(&window);
    }

    fn close_window(&mut self, wl_surface: WlSurface) {
        let Some(window) = self.find_window(&wl_surface) else { return };
        self.close_asked.retain(|(w, _)| w.alive());
        self.close_asked.retain(|(_, at)| at.elapsed() < FORCE_UNTIL);
        if let Some(i) = self.close_asked.iter().position(|(w, _)| w == &window) {
            if self.close_asked[i].1.elapsed() >= FORCE_AFTER {
                self.close_asked.remove(i);
                self.force_close(&window);
            }
            return;
        }
        self.close_asked.push((window.clone(), std::time::Instant::now()));
        match window.0.underlying_surface() {
            WindowSurface::Wayland(t) => t.send_close(),
            WindowSurface::X11(x11) => {
                if let Err(err) = x11.close() {
                    warn!(?err, "could not ask an X11 window to close");
                }
            }
        }
    }

    fn change_window(&mut self, wl_surface: WlSurface, change: Change, on: bool) {
        match change {
            Change::Minimized => self.set_window_minimized(&wl_surface, on),
            Change::Maximized => self.maximize_window(&wl_surface, on),
            Change::Fullscreen => self.fullscreen_window(&wl_surface, on),
        }
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {

    fn maximize_window(&mut self, wl_surface: &WlSurface, maximized: bool) {
        let Some(window) = self.window_for_surface(wl_surface) else {
            debug!("maximise asked for a window that is not showing; ignored");
            return;
        };
        match window.0.underlying_surface() {
            WindowSurface::Wayland(t) if maximized => XdgShellHandler::maximize_request(self, t.clone()),
            WindowSurface::Wayland(t) => XdgShellHandler::unmaximize_request(self, t.clone()),
            WindowSurface::X11(x11) => {
                if maximized {
                    self.maximize_request_x11(x11);
                } else if let Some(xwm) = x11.xwm_id() {
                    XwmHandler::unmaximize_request(self, xwm, x11.clone());
                }
            }
        }
    }

    fn fullscreen_window(&mut self, wl_surface: &WlSurface, fullscreen: bool) {
        let Some(window) = self.window_for_surface(wl_surface) else {
            debug!("full screen asked for a window that is not showing; ignored");
            return;
        };
        match window.0.underlying_surface() {
            WindowSurface::Wayland(t) if fullscreen => XdgShellHandler::fullscreen_request(self, t.clone(), None),
            WindowSurface::Wayland(t) => XdgShellHandler::unfullscreen_request(self, t.clone()),

            WindowSurface::X11(x11) => {
                if let Some(xwm) = x11.xwm_id() {
                    if fullscreen {
                        XwmHandler::fullscreen_request(self, xwm, x11.clone());
                    } else {
                        XwmHandler::unfullscreen_request(self, xwm, x11.clone());
                    }
                }
            }
        }
    }
}

impl<BackendData: Backend> WorkspacesHandler for AnvilState<BackendData> {
    fn workspaces_state(&mut self) -> &mut WorkspacesState {
        &mut self.workspaces_state
    }

    fn workspace_asked(&mut self, index: usize) {
        if index < self.workspaces {
            self.go_to_workspace(index);
        }
    }
}

crate::delegate_windows!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
crate::delegate_workspaces!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
