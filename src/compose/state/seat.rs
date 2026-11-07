// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The seat: cursor, tablets, text input, shortcut inhibiting and pointer constraints.

use super::{remember_num_lock, AnvilState, Backend};

use tracing::warn;

use smithay::{
    backend::input::TabletToolDescriptor, delegate_cursor_shape, delegate_input_method_manager,
    delegate_keyboard_shortcuts_inhibit, delegate_pointer_constraints,
    delegate_pointer_gestures, delegate_relative_pointer, delegate_seat,
    delegate_tablet_manager, delegate_text_input_manager, delegate_viewporter,
    desktop::{space::SpaceElement, PopupKind, PopupManager},
    input::{
        keyboard::LedState,
        pointer::{CursorImageStatus, PointerHandle},
        Seat, SeatHandler, SeatState,
    },
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    utils::{Logical, Point, Rectangle},
    wayland::{
        compositor::get_parent,
        input_method::{InputMethodHandler, PopupSurface},
        keyboard_shortcuts_inhibit::{
            KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState,
            KeyboardShortcutsInhibitor,
        },
        pointer_constraints::{with_pointer_constraint, PointerConstraintsHandler},
        seat::WaylandFocus,
        selection::{data_device::set_data_device_focus, primary_selection::set_primary_focus},
        tablet_manager::TabletSeatHandler,
    },
};

use crate::focus::{KeyboardFocusTarget, PointerFocusTarget};

impl<BackendData: Backend> SeatHandler for AnvilState<BackendData> {
    type KeyboardFocus = KeyboardFocusTarget;
    type PointerFocus = PointerFocusTarget;
    type TouchFocus = PointerFocusTarget;

    fn seat_state(&mut self) -> &mut SeatState<AnvilState<BackendData>> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, seat: &Seat<Self>, target: Option<&KeyboardFocusTarget>) {
        let dh = &self.display_handle;

        self.expander.clear();
        let wl_surface = target.and_then(WaylandFocus::wl_surface);

        let focus = wl_surface.as_ref().and_then(|s| dh.get_client(s.id()).ok());
        set_data_device_focus(dh, seat, focus.clone());
        set_primary_focus(dh, seat, focus);
        self.handle.insert_idle(|state| {
            let keys = state.seat.get_keyboard().and_then(|k| k.current_focus()).and_then(|f| f.wl_surface().map(|s| s.into_owned()));
            state.pointer_hold_follows_keyboard(keys.as_ref());
        });
    }
    fn cursor_image(&mut self, _seat: &Seat<Self>, image: CursorImageStatus) {
        if self.capture.away() {
            return;
        }
        self.cursor_status = image;
    }

    fn led_state_changed(&mut self, _seat: &Seat<Self>, led_state: LedState) {
        self.backend_data.update_led_state(led_state);
        if let Some(on) = led_state.num {
            remember_num_lock(on);
        }
        if let Some(on) = led_state.caps {
            self.reader_keys.caps_changed(on);
        }
    }
}
delegate_seat!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn give_keyboard(&mut self, target: Option<KeyboardFocusTarget>, serial: smithay::utils::Serial) {
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        let to_nothing = target.is_none();
        keyboard.set_focus(self, target, serial);
        if to_nothing {
            let seat = self.seat.clone();
            self.focus_changed(&seat, None);
        }
    }
}

impl<BackendData: Backend> TabletSeatHandler for AnvilState<BackendData> {
    fn tablet_tool_image(&mut self, _tool: &TabletToolDescriptor, image: CursorImageStatus) {
        if self.capture.away() {
            return;
        }
        self.cursor_status = image;
    }
}
delegate_tablet_manager!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
delegate_cursor_shape!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_text_input_manager!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> InputMethodHandler for AnvilState<BackendData> {
    fn new_popup(&mut self, surface: PopupSurface) {
        if let Err(err) = self.popups.track_popup(PopupKind::from(surface)) {
            warn!("Failed to track popup: {}", err);
        }
    }

    fn popup_repositioned(&mut self, _: PopupSurface) {}

    fn dismiss_popup(&mut self, surface: PopupSurface) {
        if let Some(parent) = surface.get_parent().map(|parent| parent.surface.clone()) {
            let _ = PopupManager::dismiss_popup(&parent, &PopupKind::from(surface));
        }
    }

    fn parent_geometry(&self, parent: &WlSurface) -> Rectangle<i32, smithay::utils::Logical> {
        self.space
            .elements()
            .find_map(|window| (window.wl_surface().as_deref() == Some(parent)).then(|| window.geometry()))
            .unwrap_or_default()
    }
}

delegate_input_method_manager!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> KeyboardShortcutsInhibitHandler for AnvilState<BackendData> {
    fn keyboard_shortcuts_inhibit_state(&mut self) -> &mut KeyboardShortcutsInhibitState {
        &mut self.keyboard_shortcuts_inhibit_state
    }

    fn new_inhibitor(&mut self, inhibitor: KeyboardShortcutsInhibitor) {
        inhibitor.activate();
    }
}

delegate_keyboard_shortcuts_inhibit!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_pointer_gestures!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_relative_pointer!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> PointerConstraintsHandler for AnvilState<BackendData> {
    fn new_constraint(&mut self, surface: &WlSurface, pointer: &PointerHandle<Self>) {
        let Some(current_focus) = pointer.current_focus() else {
            return;
        };
        if current_focus.wl_surface().as_deref() == Some(surface) && self.may_hold_pointer(surface) {
            with_pointer_constraint(surface, pointer, |constraint| {
                constraint.unwrap().activate();
            });
        }
    }

    fn cursor_position_hint(
        &mut self,
        surface: &WlSurface,
        pointer: &PointerHandle<Self>,
        location: Point<f64, Logical>,
    ) {
        if with_pointer_constraint(surface, pointer, |constraint| {
            constraint.is_some_and(|c| c.is_active())
        }) {
            let Some((under, origin)) = self.surface_under(pointer.current_location()) else { return };
            if under.wl_surface().as_deref() == Some(surface) {
                pointer.set_location(origin + location);
            }
        }
    }
}

fn root_of(surface: &WlSurface) -> WlSurface {
    let mut root = surface.clone();
    while let Some(parent) = get_parent(&root) {
        root = parent;
    }
    root
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(crate) fn may_hold_pointer(&self, surface: &WlSurface) -> bool {
        self.seat
            .get_keyboard()
            .and_then(|k| k.current_focus())
            .and_then(|f| f.wl_surface().map(|s| root_of(&s)))
            .is_some_and(|k| k == root_of(surface))
    }

    fn pointer_hold_follows_keyboard(&mut self, keys: Option<&WlSurface>) {
        let pointer = self.pointer.clone();
        let Some(held) = pointer.current_focus().and_then(|f| f.wl_surface().map(|s| s.into_owned())) else { return };
        let ours = keys.is_some_and(|k| root_of(k) == root_of(&held));
        let location = pointer.current_location();
        let at = self
            .surface_under(location)
            .filter(|(under, _)| under.wl_surface().as_deref() == Some(&held))
            .map(|(_, origin)| (location - origin).to_i32_round());
        with_pointer_constraint(&held, &pointer, |constraint| match constraint {
            Some(c) if c.is_active() && !ours => c.deactivate(),
            Some(c) if !c.is_active() && ours && at.is_some_and(|at| c.region().is_none_or(|r| r.contains(at))) => c.activate(),
            _ => {}
        });
        if ours {
            return;
        }
        let keys_root = keys.map(root_of);
        let others: Vec<WlSurface> = self.space.elements().filter_map(|w| w.wl_surface().map(|s| s.into_owned()))
            .filter(|s| Some(root_of(s)) != keys_root).collect();
        let mut surfaces = Vec::new();
        for root in others {
            smithay::wayland::compositor::with_surface_tree_downward(&root, (), |_, _, _| smithay::wayland::compositor::TraversalAction::DoChildren(()),
                |surface, _, _| surfaces.push(surface.clone()), |_, _, _| true);
        }
        for surface in surfaces {
            with_pointer_constraint(&surface, &pointer, |constraint| {
                if let Some(c) = constraint.filter(|c| c.is_active()) {
                    c.deactivate();
                }
            });
        }
        let under = self.surface_under(location);
        if under.as_ref().and_then(|(t, _)| t.wl_surface().map(|s| s.into_owned())).as_ref() != Some(&held) {
            let serial = smithay::utils::SERIAL_COUNTER.next_serial();
            let time = self.clock.now().as_millis();
            pointer.motion(self, under, &smithay::input::pointer::MotionEvent { location, serial, time });
            pointer.frame(self);
        }
    }
}
delegate_pointer_constraints!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_viewporter!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
