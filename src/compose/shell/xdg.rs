// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::cell::RefCell;

use smithay::{
    desktop::{
        find_popup_root_surface, get_popup_toplevel_coords, layer_map_for_output,
        PopupKeyboardGrab, PopupKind, PopupPointerGrab, PopupUngrabStrategy, Space, Window,
        WindowSurfaceType,
    },
    input::{pointer::Focus, touch::TouchHandle, Seat},
    reexports::{
        wayland_protocols::xdg::{decoration as xdg_decoration, shell::server::xdg_toplevel},
        wayland_server::{
            protocol::{wl_output, wl_seat, wl_surface::WlSurface},
            Resource,
        },
    },
    utils::{Logical, Point, Rectangle, Serial},
    wayland::{
        compositor::{self, with_states},
        seat::WaylandFocus,
        shell::xdg::{
            Configure, PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
    },
};
use tracing::{trace, warn};

use crate::{
    focus::KeyboardFocusTarget,
    shell::{TouchMoveSurfaceGrab, TouchResizeSurfaceGrab},
    state::{AnvilState, Backend},
};

use super::placement::place_new_window;
use super::{
    fullscreen_output_geometry, FullscreenSurface, PointerMoveSurfaceGrab,
    ResizeData, ResizeEdge, ResizeState, SurfaceData, WindowElement,
};

impl<BackendData: Backend> XdgShellHandler for AnvilState<BackendData> {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        if let Some(window) = self.window_for_surface(surface.wl_surface()) {
            crate::animate::closing(&window);
        }
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {

        let window = WindowElement(Window::new_wayland_window(surface.clone()));

        let greeter = crate::session::launch().greeter;
        let screen = self.space.outputs().next().and_then(|o| self.space.output_geometry(o));
        match (greeter, screen) {
            (true, Some(screen)) => {
                surface.with_pending_state(|state| {
                    state.states.set(xdg_toplevel::State::Fullscreen);
                    state.size = Some(screen.size);
                });
                self.space.map_element(window.clone(), screen.loc, true);
            }
            _ => { place_new_window(&mut self.space, self.pointer.current_location(), &window, true); }
        }
        self.place_on_active_workspace(&window);

        compositor::add_post_commit_hook(surface.wl_surface(), |state: &mut Self, _, surface| {
            handle_toplevel_commit(&mut state.space, surface);
        });
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {

        self.unconstrain_popup(&surface);

        if let Err(err) = self.popups.track_popup(PopupKind::from(surface)) {
            warn!("Failed to track popup: {}", err);
        }
    }

    fn reposition_request(&mut self, surface: PopupSurface, positioner: PositionerState, token: u32) {
        surface.with_pending_state(|state| {
            let geometry = positioner.get_geometry();
            state.geometry = geometry;
            state.positioner = positioner;
        });
        self.unconstrain_popup(&surface);
        surface.send_repositioned(token);
    }

    fn move_request(&mut self, surface: ToplevelSurface, seat: wl_seat::WlSeat, serial: Serial) {
        let Some(seat) = known_seat(Seat::<AnvilState<BackendData>>::from_resource(&seat)) else {
            return;
        };
        self.move_request_xdg(&surface, &seat, serial)
    }

    fn resize_request(
        &mut self,
        surface: ToplevelSurface,
        seat: wl_seat::WlSeat,
        serial: Serial,
        edges: xdg_toplevel::ResizeEdge,
    ) {
        let Some(seat) = known_seat(Seat::<AnvilState<BackendData>>::from_resource(&seat)) else {
            return;
        };

        if let Some(touch) = seat.get_touch() {
            if self.resize_by_touch(&touch, &surface, serial, edges) {
                return;
            }
        }

        let pointer = seat.get_pointer().unwrap();

        if !pointer.has_grab(serial) {
            return;
        }

        let start_data = pointer.grab_start_data().unwrap();

        let Some(window) = self.window_for_surface(surface.wl_surface()) else { return };

        if start_data.focus.is_none()
            || !start_data
                .focus
                .as_ref()
                .unwrap()
                .0
                .same_client_as(&surface.wl_surface().id())
        {
            return;
        }

        self.start_pointer_resize(window, edges.into(), start_data, serial);
    }

    fn ack_configure(&mut self, surface: WlSurface, configure: Configure) {
        if let Configure::Toplevel(configure) = configure {
            resize_acknowledged(&surface, configure.serial);

            let window = self
                .space
                .elements()
                .find(|element| element.wl_surface().as_deref() == Some(&surface));
            if let Some(window) = window {
                use xdg_decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;
                let is_ssd = configure
                    .state
                    .decoration_mode
                    .map(|mode| mode == Mode::ServerSide)
                    .unwrap_or(false);
                window.set_ssd(is_ssd);
            }
        }
    }

    fn fullscreen_request(&mut self, surface: ToplevelSurface, mut wl_output: Option<wl_output::WlOutput>) {
        if surface
            .current_state()
            .capabilities
            .contains(xdg_toplevel::WmCapabilities::Fullscreen)
        {

            let wl_surface = surface.wl_surface();

            let output_geometry = fullscreen_output_geometry(wl_surface, wl_output.as_ref(), &mut self.space);

            if let Some((output, geometry)) = output_geometry {
                let client = match self.display_handle.get_client(wl_surface.id()) {
                    Ok(client) => client,
                    Err(_) => return,
                };
                for output in output.client_outputs(&client) {
                    wl_output = Some(output);
                }
                let Some(window) = self
                    .space
                    .elements()
                    .find(|window| window.wl_surface().map(|s| &*s == wl_surface).unwrap_or(false))
                    .cloned()
                else {
                    return;
                };

                surface.with_pending_state(|state| {
                    state.states.set(xdg_toplevel::State::Fullscreen);
                    state.size = Some(geometry.size);
                    state.fullscreen_output = wl_output;
                });
                self.leave_fullscreen_slots(&window, Some(&output));
                output.user_data().insert_if_missing(FullscreenSurface::default);
                output
                    .user_data()
                    .get::<FullscreenSurface>()
                    .unwrap()
                    .set(window.clone());
                trace!("Fullscreening: {:?}", window);
            }
        }

        if surface.is_initial_configure_sent() {
            surface.send_configure();
        }
    }

    fn unfullscreen_request(&mut self, surface: ToplevelSurface) {
        if !surface
            .current_state()
            .states
            .contains(xdg_toplevel::State::Fullscreen)
        {
            return;
        }

        surface.with_pending_state(|state| {
            state.states.unset(xdg_toplevel::State::Fullscreen);
            state.size = None;
            state.fullscreen_output.take();
        });
        if let Some(window) = self.window_for_surface(surface.wl_surface()) {
            self.leave_fullscreen_slots(&window, None);
        }

        surface.send_pending_configure();
    }

    fn maximize_request(&mut self, surface: ToplevelSurface) {

        if surface
            .current_state()
            .capabilities
            .contains(xdg_toplevel::WmCapabilities::Maximize)
        {
            if let Some(window) = self.window_for_surface(surface.wl_surface()) {

                let order: Vec<_> = self.space.elements().cloned().collect();
                self.maximize(&window);
                for w in &order {
                    self.space.raise_element(w, false);
                }
            }
        }

        if surface.is_initial_configure_sent() {
            surface.send_configure();
        }
    }

    fn unmaximize_request(&mut self, surface: ToplevelSurface) {
        if let Some(window) = self.window_for_surface(surface.wl_surface()) {
            let order: Vec<_> = self.space.elements().cloned().collect();
            self.unmaximize(&window);
            for w in &order {
                self.space.raise_element(w, false);
            }
        }
    }

    fn minimize_request(&mut self, surface: ToplevelSurface) {
        self.set_window_minimized(surface.wl_surface(), true);
    }

    fn grab(&mut self, surface: PopupSurface, seat: wl_seat::WlSeat, serial: Serial) {
        let Some(seat) = known_seat(Seat::<AnvilState<BackendData>>::from_resource(&seat)) else {
            return;
        };
        let kind = PopupKind::Xdg(surface);
        if let Some(root) = find_popup_root_surface(&kind).ok().and_then(|root| {
            self.space
                .elements()
                .find(|w| w.wl_surface().map(|s| *s == root).unwrap_or(false))
                .cloned()
                .map(KeyboardFocusTarget::from)
                .or_else(|| {
                    self.space
                        .outputs()
                        .find_map(|o| {
                            let map = layer_map_for_output(o);
                            map.layer_for_surface(&root, WindowSurfaceType::TOPLEVEL).cloned()
                        })
                        .map(KeyboardFocusTarget::LayerSurface)
                })
        }) {
            let ret = self.popups.grab_popup(root, kind, &seat, serial);

            if let Ok(mut grab) = ret {
                if let Some(keyboard) = seat.get_keyboard() {
                    if keyboard.is_grabbed()
                        && !(keyboard.has_grab(serial)
                            || keyboard.has_grab(grab.previous_serial().unwrap_or(serial)))
                    {
                        grab.ungrab(PopupUngrabStrategy::All);
                        return;
                    }
                    keyboard.set_focus(self, grab.current_grab(), serial);
                    keyboard.set_grab(self, PopupKeyboardGrab::new(&grab), serial);
                }
                if let Some(pointer) = seat.get_pointer() {
                    if pointer.is_grabbed()
                        && !(pointer.has_grab(serial)
                            || pointer.has_grab(grab.previous_serial().unwrap_or_else(|| grab.serial())))
                    {
                        grab.ungrab(PopupUngrabStrategy::All);
                        return;
                    }
                    pointer.set_grab(self, PopupPointerGrab::new(&grab), serial, Focus::Keep);
                }
            }
        }
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {

    pub fn refit_login_screen(&mut self) {
        if !crate::session::launch().greeter {
            return;
        }
        let Some(output) = self.space.outputs().next().cloned() else { return };
        let Some(screen) = self.space.output_geometry(&output) else { return };
        let zone = smithay::desktop::layer_map_for_output(&output).non_exclusive_zone();
        let room = Rectangle::new(screen.loc + zone.loc, zone.size);
        for window in self.space.elements().cloned().collect::<Vec<_>>() {
            if let Some(toplevel) = window.0.toplevel() {
                if toplevel.with_pending_state(|state| state.size) != Some(room.size) {
                    toplevel.with_pending_state(|state| state.size = Some(room.size));
                    toplevel.send_pending_configure();
                }
            }
            if self.space.element_location(&window) != Some(room.loc) {
                super::place_keeping_order(&mut self.space, window, room.loc);
            }
        }
    }

    pub fn move_request_xdg(&mut self, surface: &ToplevelSurface, seat: &Seat<Self>, serial: Serial) {
        if let Some(touch) = seat.get_touch() {
            if self.move_by_touch(&touch, surface, serial) {
                return;
            }
        }

        let pointer = seat.get_pointer().unwrap();

        if !pointer.has_grab(serial) {
            return;
        }

        let start_data = pointer.grab_start_data().unwrap();

        let Some(window) = self.window_for_surface(surface.wl_surface()) else {
            return;
        };

        if start_data.focus.is_none()
            || !start_data
                .focus
                .as_ref()
                .unwrap()
                .0
                .same_client_as(&surface.wl_surface().id())
        {
            return;
        }

        let initial_window_location = self.space.element_location(&window).unwrap();

        let grab = PointerMoveSurfaceGrab {
            start_data,
            window,
            initial_window_location,
            restore_pending: true,
        };

        pointer.set_grab(self, grab, serial, Focus::Clear);
    }

    fn move_by_touch(&mut self, touch: &TouchHandle<Self>, surface: &ToplevelSurface, serial: Serial) -> bool {
        if !touch.has_grab(serial) {
            return false;
        }
        let start_data = touch.grab_start_data().unwrap();

        let Some(window) = self.window_for_surface(surface.wl_surface()) else {
            return true;
        };

        if start_data.focus.is_none()
            || !start_data
                .focus
                .as_ref()
                .unwrap()
                .0
                .same_client_as(&surface.wl_surface().id())
        {
            return true;
        }

        let initial_window_location = self.space.element_location(&window).unwrap();

        let grab = TouchMoveSurfaceGrab {
            start_data,
            window,
            initial_window_location,
            restore_pending: true,
        };

        touch.set_grab(self, grab, serial);
        true
    }

    fn resize_by_touch(&mut self, touch: &TouchHandle<Self>, surface: &ToplevelSurface, serial: Serial, edges: xdg_toplevel::ResizeEdge) -> bool {
        if !touch.has_grab(serial) {
            return false;
        }
        let start_data = touch.grab_start_data().unwrap();
        tracing::info!(?start_data);

        let Some(window) = self.window_for_surface(surface.wl_surface()) else {
            tracing::info!("no window");
            return true;
        };

        if start_data.focus.is_none()
            || !start_data
                .focus
                .as_ref()
                .unwrap()
                .0
                .same_client_as(&surface.wl_surface().id())
        {
            tracing::info!("different surface");
            return true;
        }
        let loc = self.space.element_location(&window).unwrap();
        let (initial_window_location, initial_window_size) = (loc, window.client_size());

        with_states(surface.wl_surface(), move |states| {
            states
                .data_map
                .get::<RefCell<SurfaceData>>()
                .unwrap()
                .borrow_mut()
                .resize_state = ResizeState::Resizing(ResizeData {
                edges: edges.into(),
                initial_window_location,
                initial_window_size,
            });
        });

        let grab = TouchResizeSurfaceGrab {
            start_data,
            window,
            edges: edges.into(),
            initial_window_location,
            initial_window_size,
            last_window_size: initial_window_size,
        };

        touch.set_grab(self, grab, serial);
        true
    }

    pub(crate) fn unconstrain_popup(&self, popup: &PopupSurface) {
        let Ok(root) = find_popup_root_surface(&PopupKind::Xdg(popup.clone())) else {
            return;
        };
        let Some(window) = self.window_for_surface(&root) else {

            return self.unconstrain_layer_popup(popup, &root);
        };

        let mut outputs_for_window = self.space.outputs_for_element(&window);
        if outputs_for_window.is_empty() {
            return;
        }

        let mut outputs_geo = self
            .space
            .output_geometry(&outputs_for_window.pop().unwrap())
            .unwrap();
        for output in outputs_for_window {
            outputs_geo = outputs_geo.merge(self.space.output_geometry(&output).unwrap());
        }

        let window_geo = self.space.element_geometry(&window).unwrap();

        let mut target = outputs_geo;
        target.loc -= get_popup_toplevel_coords(&PopupKind::Xdg(popup.clone()));
        target.loc -= window_geo.loc;

        popup.with_pending_state(|state| {
            state.geometry = state.positioner.get_unconstrained_geometry(target);
        });
    }

    fn unconstrain_layer_popup(&self, popup: &PopupSurface, root: &WlSurface) {
        let Some((output_size, layer_loc)) = self.space.outputs().find_map(|o| {
            let map = smithay::desktop::layer_map_for_output(o);
            let layer = map.layer_for_surface(root, smithay::desktop::WindowSurfaceType::TOPLEVEL)?;
            Some((self.space.output_geometry(o)?.size, map.layer_geometry(layer)?.loc))
        }) else {
            return;
        };
        let mut target = Rectangle::from_size(output_size);
        target.loc -= get_popup_toplevel_coords(&PopupKind::Xdg(popup.clone()));
        target.loc -= layer_loc;
        popup.with_pending_state(|state| {
            state.geometry = state.positioner.get_unconstrained_geometry(target);
        });
    }
}

fn handle_toplevel_commit(space: &mut Space<WindowElement>, surface: &WlSurface) -> Option<()> {
    let window = space
        .elements()
        .find(|w| w.wl_surface().as_deref() == Some(surface))
        .cloned()?;

    let mut window_loc = space.element_location(&window)?;
    let geometry = Rectangle::<i32, Logical>::from_size(window.client_size());

    let new_loc: Point<Option<i32>, Logical> = with_states(window.wl_surface().as_deref()?, |states| {
        let data = states.data_map.get::<RefCell<SurfaceData>>()?.borrow_mut();

        if let ResizeState::Resizing(resize_data) = data.resize_state {
            let edges = resize_data.edges;
            let loc = resize_data.initial_window_location;
            let size = resize_data.initial_window_size;

            edges.intersects(ResizeEdge::TOP_LEFT).then(|| {
                let new_x = edges
                    .intersects(ResizeEdge::LEFT)
                    .then_some(loc.x + (size.w - geometry.size.w));

                let new_y = edges
                    .intersects(ResizeEdge::TOP)
                    .then_some(loc.y + (size.h - geometry.size.h));

                (new_x, new_y).into()
            })
        } else {
            None
        }
    })?;

    if let Some(new_x) = new_loc.x {
        window_loc.x = new_x;
    }
    if let Some(new_y) = new_loc.y {
        window_loc.y = new_y;
    }

    if new_loc.x.is_some() || new_loc.y.is_some() {

        super::place_keeping_order(space, window, window_loc);
    }

    Some(())
}

fn resize_acknowledged(surface: &WlSurface, acked: Serial) {
        if let Some(serial) = with_states(surface, |states| {
            if let Some(data) = states.data_map.get::<RefCell<SurfaceData>>() {
                if let ResizeState::WaitingForFinalAck(_, serial) = data.borrow().resize_state {
                    return Some(serial);
                }
            }

            None
        }) {

            let is_resizing = with_states(surface, |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .current
                    .states
                    .contains(xdg_toplevel::State::Resizing)
            });

            if acked >= serial && is_resizing {
                with_states(surface, |states| {
                    let mut data = states
                        .data_map
                        .get::<RefCell<SurfaceData>>()
                        .unwrap()
                        .borrow_mut();
                    if let ResizeState::WaitingForFinalAck(resize_data, _) = data.resize_state {
                        data.resize_state = ResizeState::WaitingForCommit(resize_data);
                    } else {
                        unreachable!()
                    }
                });
            }
        }
}

fn known_seat<S>(found: Option<S>) -> Option<S> {
    if found.is_none() {
        tracing::debug!("request names a seat the compositor does not know; ignored");
    }
    found
}

#[cfg(test)]
mod seat_tests {
    use super::known_seat;

    #[test]
    fn a_request_naming_an_unknown_seat_is_ignored() {
        assert_eq!(known_seat::<u8>(None), None);
        assert_eq!(known_seat(Some(3u8)), Some(3));
    }

    #[test]
    fn no_client_named_seat_or_output_is_unwrapped() {
        let text = include_str!("xdg.rs");
        let code = text.split("#[cfg(test)]").next().unwrap();
        assert!(!code.contains("Seat::from_resource(&seat).unwrap()"));
        assert!(!code.contains("Output::from_resource(&output).unwrap()"));
    }
}
