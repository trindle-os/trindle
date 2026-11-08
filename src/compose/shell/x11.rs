// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::os::unix::io::OwnedFd;

use smithay::{
    desktop::Window,
    input::pointer::Focus,
    utils::{Logical, Rectangle, SERIAL_COUNTER},
    wayland::{
        selection::{
            data_device::{
                clear_data_device_selection, current_data_device_selection_userdata,
                request_data_device_client_selection, set_data_device_selection,
            },
            primary_selection::{
                clear_primary_selection, current_primary_selection_userdata,
                request_primary_client_selection, set_primary_selection,
            },
            SelectionTarget,
        },
        xwayland_shell::{XWaylandShellHandler, XWaylandShellState},
    },
    xwayland::{
        xwm::{Reorder, ResizeEdge as X11ResizeEdge, WmWindowProperty, XwmId},
        X11Surface, X11Wm, XwmHandler,
    },
};
use tracing::{error, trace, warn};

use crate::{focus::KeyboardFocusTarget, state::Backend, AnvilState};

use super::placement::place_new_window;
use super::{
    FullscreenSurface, PointerMoveSurfaceGrab, TouchMoveSurfaceGrab, WindowElement,
};

impl<BackendData: Backend> XWaylandShellHandler for AnvilState<BackendData> {
    fn xwayland_shell_state(&mut self) -> &mut XWaylandShellState {
        &mut self.xwayland_shell_state
    }
}

impl<BackendData: Backend> XwmHandler for AnvilState<BackendData> {
    fn xwm_state(&mut self, xwm: XwmId) -> &mut X11Wm {

        self.rooms.wm(xwm).expect("an Xwayland window manager Trindle knows")
    }

    fn xdnd_event(&mut self, xwm: XwmId, event: smithay::xwayland::xwm::XdndEvent) {
        AnvilState::xdnd_event(self, xwm, event);
    }

    fn send_xdnd(&mut self, xwm: XwmId, mime_type: String, fd: OwnedFd) {
        self.xdnd_send_to_old(xwm, mime_type, fd);
    }

    fn disconnected(&mut self, xwm: XwmId) {
        self.fresh.retain(|(of, _), _| *of != Some(xwm));
        self.xdnd_lost(xwm);
        self.room_lost(xwm);
    }

    fn new_window(&mut self, _xwm: XwmId, _window: X11Surface) {}
    fn new_override_redirect_window(&mut self, _xwm: XwmId, _window: X11Surface) {}

    fn map_window_request(&mut self, _xwm: XwmId, window: X11Surface) {
        let _ = window.set_mapped(true);
        let decorated = window.is_decorated();
        let window = WindowElement(Window::new_x11_window(window));

        window.set_ssd(!decorated);
        place_new_window(&mut self.space, self.pointer.current_location(), &window, true);
        self.place_on_active_workspace(&window);
        let Some(xsurface) = window.0.x11_surface() else {
            unreachable!()
        };

        let Some(loc) = self.space.element_location(&window) else {
            let _ = xsurface.configure(None);
            return;
        };
        let _ = xsurface.configure(Some(Rectangle::new(loc + window.deco_offset(), xsurface.geometry().size)));
        self.fresh.insert((xsurface.xwm_id(), xsurface.window_id()), std::time::Instant::now());
        if !self.opens_behind_full_screen(&window) {
            self.focus_window(&window);
        }
    }

    fn map_window_notify(&mut self, _xwm: XwmId, window: X11Surface) {
        let _ = window.configure(None);
    }

    fn mapped_override_redirect_window(&mut self, _xwm: XwmId, window: X11Surface) {
        let location = window.geometry().loc;
        let window = WindowElement(Window::new_x11_window(window));
        self.space.map_element(window, location, true);
    }

    fn unmapped_window(&mut self, _xwm: XwmId, window: X11Surface) {
        let maybe = self
            .space
            .elements()
            .find(|e| matches!(e.0.x11_surface(), Some(w) if w == &window))
            .cloned();
        if let Some(elem) = maybe {
            crate::animate::closing(&elem);
            self.space.unmap_elem(&elem);

            let had_keyboard = self
                .seat
                .get_keyboard()
                .and_then(|k| k.current_focus())
                .is_some_and(|f| f == KeyboardFocusTarget::Window(elem.0.clone()));
            if had_keyboard {
                self.focus_topmost();
            }
        }

        self.minimized.retain(|(e, _)| e.0.x11_surface() != Some(&window));
        self.hidden.retain(|(e, _)| e.0.x11_surface() != Some(&window));
        self.fresh.remove(&(window.xwm_id(), window.window_id()));
        if !window.is_override_redirect() {
            let _ = window.set_mapped(false);
        }
    }

    fn destroyed_window(&mut self, _xwm: XwmId, _window: X11Surface) {}

    fn configure_request(
        &mut self,
        _xwm: XwmId,
        window: X11Surface,
        _x: Option<i32>,
        _y: Option<i32>,
        w: Option<u32>,
        h: Option<u32>,
        _reorder: Option<Reorder>,
    ) {

        let mut geo = window.geometry();
        if let Some(w) = w {
            geo.size.w = w as i32;
        }
        if let Some(h) = h {
            geo.size.h = h as i32;
        }
        let _ = window.configure(geo);
    }

    fn configure_notify(
        &mut self,
        _xwm: XwmId,
        window: X11Surface,
        geometry: Rectangle<i32, Logical>,
        _above: Option<u32>,
    ) {
        let Some(elem) = self
            .space
            .elements()
            .find(|e| matches!(e.0.x11_surface(), Some(w) if w == &window))
            .cloned()
        else {
            return;
        };

        let loc = geometry.loc - elem.deco_offset();
        super::place_keeping_order(&mut self.space, elem, loc);

    }

    fn maximize_request(&mut self, _xwm: XwmId, window: X11Surface) {
        self.maximize_request_x11(&window);
    }

    fn unmaximize_request(&mut self, _xwm: XwmId, window: X11Surface) {
        if let Some(elem) = self.x11_element(&window) {
            self.unmaximize(&elem);
        }
    }

    fn fullscreen_request(&mut self, _xwm: XwmId, window: X11Surface) {
        if let Some(elem) = self
            .space
            .elements()
            .find(|e| matches!(e.0.x11_surface(), Some(w) if w == &window))
        {
            let outputs_for_window = self.space.outputs_for_element(elem);

            let Some(output) = outputs_for_window.first().or_else(|| self.space.outputs().next()) else {
                return;
            };
            let Some(area) = self.space.output_geometry(output) else {
                return;
            };
            let geometry = full_screen_size(&window, area);

            let _ = window.set_fullscreen(true);
            elem.set_ssd(false);
            let _ = window.configure(geometry);
            output.user_data().insert_if_missing(FullscreenSurface::default);
            output
                .user_data()
                .get::<FullscreenSurface>()
                .unwrap()
                .set(elem.clone());
            trace!("Fullscreening: {:?}", elem);
        }
    }

    fn unfullscreen_request(&mut self, _xwm: XwmId, window: X11Surface) {
        if let Some(elem) = self
            .space
            .elements()
            .find(|e| matches!(e.0.x11_surface(), Some(w) if w == &window))
        {
            let _ = window.set_fullscreen(false);
            elem.set_ssd(!window.is_decorated());
            if let Some(output) = self.space.outputs().find(|o| {
                o.user_data()
                    .get::<FullscreenSurface>()
                    .and_then(|f| f.get())
                    .map(|w| &w == elem)
                    .unwrap_or(false)
            }) {
                trace!("Unfullscreening: {:?}", elem);
                output.user_data().get::<FullscreenSurface>().unwrap().clear();
                let _ = window.configure(self.space.element_bbox(elem));
                self.backend_data.reset_buffers(output);
            }
        }
    }

    fn property_notify(&mut self, _xwm: XwmId, window: X11Surface, property: WmWindowProperty) {
        if property != WmWindowProperty::EmulatedScreens || !window.is_fullscreen() {
            return;
        }
        let Some(elem) = self.x11_element(&window) else { return };
        let Some(output) = self.space.outputs_for_element(&elem).into_iter().next() else { return };
        if let Some(screen) = self.space.output_geometry(&output) {
            let _ = window.configure(full_screen_size(&window, screen));
        }
    }

    fn minimize_request(&mut self, _xwm: XwmId, window: X11Surface) {
        let Some(elem) = self.x11_element(&window) else {
            let _ = window.set_iconic(true);
            return;
        };

        if self.fresh.get(&(window.xwm_id(), window.window_id())).is_some_and(|at| at.elapsed() < self.fresh_for) {
            let _ = window.set_iconic(false);
            self.focus_window(&elem);
            return;
        }
        let _ = window.set_iconic(true);
        let has_keys = self.seat.get_keyboard().and_then(|k| k.current_focus()) == Some(KeyboardFocusTarget::Window(elem.0.clone()));
        if window.is_fullscreen() && !has_keys {
            return;
        }
        if let Some(surface) = elem.wl_surface() {
            self.set_window_minimized(&surface, true);
        }
    }

    fn unminimize_request(&mut self, _xwm: XwmId, window: X11Surface) {
        let _ = window.set_iconic(false);
        if let Some(surface) = window.wl_surface() {
            self.set_window_minimized(&surface, false);
        }
    }

    fn resize_request(&mut self, _xwm: XwmId, window: X11Surface, _button: u32, edges: X11ResizeEdge) {

        let Some(start_data) = self.pointer.grab_start_data() else { return };
        let Some(element) = self.x11_element(&window) else { return };
        self.start_pointer_resize(element, edges.into(), start_data, SERIAL_COUNTER.next_serial());
    }

    fn move_request(&mut self, _xwm: XwmId, window: X11Surface, _button: u32) {
        self.move_request_x11(&window)
    }

    fn allow_selection_access(&mut self, _xwm: XwmId, _selection: SelectionTarget) -> bool {
        true
    }

    fn send_selection(&mut self, xwm: XwmId, selection: SelectionTarget, mime_type: String, fd: OwnedFd) {

        let from = self.rooms.selection_from[crate::state::selection_slot(selection)].as_ref().map(|(id, _)| *id);
        if let Some(from) = from.filter(|f| *f != xwm) {
            let handle = self.handle.clone();
            if let Some(other) = self.rooms.wm(from) {
                if let Err(err) = other.send_selection(selection, mime_type, fd, handle) {
                    warn!(?err, "could not pass a copy between two Xwayland servers");
                }
            }
            return;
        }
        match selection {
            SelectionTarget::Clipboard => {
                if let Err(err) = request_data_device_client_selection(&self.seat, mime_type, fd) {
                    error!(?err, "Failed to request current wayland clipboard for Xwayland",);
                }
            }
            SelectionTarget::Primary => {
                if let Err(err) = request_primary_client_selection(&self.seat, mime_type, fd) {
                    error!(
                        ?err,
                        "Failed to request current wayland primary selection for Xwayland",
                    );
                }
            }
        }
    }

    fn new_selection(&mut self, xwm: XwmId, selection: SelectionTarget, mime_types: Vec<String>) {
        trace!(?selection, ?mime_types, "Got Selection from X11",);

        let slot = crate::state::selection_slot(selection);
        self.wayland_selection[slot] = None;
        self.rooms.selection_from[slot] = Some((xwm, mime_types.clone()));

        for other in self.rooms.running().filter(|w| w.id() != xwm) {
            if let Err(err) = other.new_selection(selection, Some(mime_types.clone())) {
                warn!(?err, ?selection, "Failed to set Xwayland selection");
            }
        }
        match selection {
            SelectionTarget::Clipboard => {
                set_data_device_selection(&self.display_handle, &self.seat, mime_types, ())
            }
            SelectionTarget::Primary => {
                set_primary_selection(&self.display_handle, &self.seat, mime_types, ())
            }
        }
    }

    fn cleared_selection(&mut self, xwm: XwmId, selection: SelectionTarget) {
        self.selection_cleared(xwm, selection);
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {

    pub fn selection_cleared(&mut self, xwm: XwmId, selection: SelectionTarget) {
        let slot = crate::state::selection_slot(selection);
        if self.rooms.selection_from[slot].as_ref().is_none_or(|(from, _)| *from != xwm) {
            return;
        }
        self.rooms.selection_from[slot] = None;
        for other in self.rooms.running().filter(|w| w.id() != xwm) {
            let _ = other.new_selection(selection, None);
        }
        match selection {
            SelectionTarget::Clipboard => {
                if current_data_device_selection_userdata(&self.seat).is_some() {
                    clear_data_device_selection(&self.display_handle, &self.seat)
                }
            }
            SelectionTarget::Primary => {
                if current_primary_selection_userdata(&self.seat).is_some() {
                    clear_primary_selection(&self.display_handle, &self.seat)
                }
            }
        }
    }

    pub(crate) fn x11_element(&self, window: &X11Surface) -> Option<WindowElement> {
        self.space
            .elements()
            .find(|e| matches!(e.0.x11_surface(), Some(w) if w == window))
            .cloned()
    }

    pub fn maximize_request_x11(&mut self, window: &X11Surface) {
        if let Some(elem) = self.x11_element(window) {
            self.maximize(&elem);
        }
    }

    pub fn move_request_x11(&mut self, window: &X11Surface) {
        if let Some(touch) = self.seat.get_touch() {
            if let Some(start_data) = touch.grab_start_data() {
                if let Some(element) = self.x11_element(window) {
                    let Some(initial_window_location) = self.space.element_location(&element) else {
                        return;
                    };

                    let grab = TouchMoveSurfaceGrab {
                        start_data,
                        window: element.clone(),
                        initial_window_location,
                        restore_pending: true,
                    };

                    touch.set_grab(self, grab, SERIAL_COUNTER.next_serial());
                    return;
                }
            }
        }

        let Some(start_data) = self.pointer.grab_start_data() else {
            return;
        };

        let Some(element) = self.x11_element(window) else { return };
        let Some(initial_window_location) = self.space.element_location(&element) else { return };

        let grab = PointerMoveSurfaceGrab {
            start_data,
            window: element.clone(),
            initial_window_location,
            restore_pending: true,
        };

        let pointer = self.pointer.clone();
        pointer.set_grab(self, grab, SERIAL_COUNTER.next_serial(), Focus::Clear);
    }
}

pub fn full_screen_size(window: &X11Surface, screen: Rectangle<i32, Logical>) -> Rectangle<i32, Logical> {
    window
        .emulated_screens()
        .into_iter()
        .find(|e| e.loc == screen.loc && e.size.w > 0 && e.size.h > 0)
        .map_or(screen, |e| Rectangle::new(screen.loc, e.size))
}
