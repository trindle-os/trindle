// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::convert::TryInto;

use super::BTN_LEFT;
use crate::{focus::PointerFocusTarget, screen_edge, state::Backend, AnvilState};

use smithay::{
    backend::input::{
        self, AbsolutePositionEvent, Axis, AxisSource, Event, InputBackend, PointerAxisEvent,
        PointerButtonEvent, PointerMotionEvent,
    },
    desktop::{layer_map_for_output, WindowSurfaceType},
    input::pointer::{
        AxisFrame, ButtonEvent, GrabStartData as PointerGrabStartData, MotionEvent,
        PointerHandle, RelativeMotionEvent,
    },
    output::Output, reexports::wayland_server::protocol::wl_pointer,
    utils::{Logical, Point, Rectangle, Serial, SERIAL_COUNTER as SCOUNTER},
    wayland::{
        compositor::RegionAttributes,
        pointer_constraints::{with_pointer_constraint, PointerConstraint},
        seat::WaylandFocus,
    },
};
use tracing::info;

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(super) fn on_pointer_button<B: InputBackend>(&mut self, evt: B::PointerButtonEvent) {
        let serial = SCOUNTER.next_serial();
        let button = evt.button_code();
        let time = evt.time_msec();

        let state = wl_pointer::ButtonState::from(evt.state());

        if self.button_kept_back(button, state, serial, time) {
            return;
        }
        if wl_pointer::ButtonState::Pressed == state && self.press_taken_by_the_desktop(button, serial, time) {
            return;
        }
        if wl_pointer::ButtonState::Pressed == state && !self.is_locked() {
            self.hold_on_layer(button, serial);
        }
        let pointer = self.pointer.clone();

        let to = pointer.current_focus().and_then(|f| f.wl_surface().map(|s| s.into_owned()));
        let placed = self.pointer_put_by_hand();
        self.own_press(serial.into(), to.as_ref(), placed);
        pointer.button(
            self,
            &ButtonEvent {
                button,
                state: state.try_into().unwrap(),
                serial,
                time,
            },
        );
        pointer.frame(self);
        if wl_pointer::ButtonState::Released == state {
            self.update_edge_cursor(self.pointer.current_location());
        }
    }

    fn button_kept_back(&mut self, button: u32, state: wl_pointer::ButtonState, serial: Serial, time: u32) -> bool {

        if wl_pointer::ButtonState::Pressed == state {
            self.expander.clear();
        }
        if wl_pointer::ButtonState::Pressed == state && self.modal_keeps_back(self.pointer.current_location()) {
            return true;
        }

        if wl_pointer::ButtonState::Released == state {
            if let Some(i) = self.plugged.kept.iter().position(|b| *b == button) {
                self.plugged.kept.remove(i);
                return true;
            }
        } else if self.is_locked() {
            let targets = [self.surface_under(self.pointer.current_location()).map(|u| u.0), self.pointer.current_focus()];
            match self.press_on_question(&targets, true) {
                Some(false) => {
                    info!("a press on Pocketkey's unlock question was kept back");
                    self.plugged.kept.push(button);
                    return true;
                }
                Some(true) => {
                    let pointer = self.pointer.clone();
                    if pointer.is_grabbed() {
                        pointer.unset_grab(self, serial, time);
                    }
                    self.refresh_pointer_focus();
                }
                None => {}
            }
        }
        false
    }

    fn press_taken_by_the_desktop(&mut self, button: u32, serial: Serial, time: u32) -> bool {

        self.release_key = None;

        let at = self.pointer.current_location();

        if button == BTN_LEFT && !self.is_locked() && self.try_again_at(at) {
            return true;
        }
        if button == BTN_LEFT {

            if self.desktop_double(at, time, crate::pointer::DOUBLE_CLICK_SLOP, self.pointer.is_grabbed()) {
                if let Some(cmd) = self.config.desktop_double_click.clone() {
                    self.spawn_for_click(&cmd);
                }
            }
        } else {
            self.desktop_tap = None;
        }
        if self.is_locked() || !self.press_on_edge(button, serial) {
            self.update_keyboard_focus(at, serial);
        }
        false
    }

    fn hold_on_layer(&mut self, button: u32, serial: Serial) {
        if self.pointer.is_grabbed() {
            return;
        }
        let location = self.pointer.current_location();
        let Some(under) = self.surface_under(location) else { return };
        let PointerFocusTarget::WlSurface(surface) = &under.0 else { return };
        let mut root = surface.clone();
        while let Some(parent) = smithay::wayland::compositor::get_parent(&root) {
            root = parent;
        }
        let found = self.space.outputs().find_map(|o| {
            let map = layer_map_for_output(o);
            let layer = map.layer_for_surface(&root, WindowSurfaceType::ALL)?.clone();
            let at = map.layer_geometry(&layer)?.loc;
            Some((layer, o.clone(), at))
        });
        let Some((layer, output, was_at)) = found else { return };
        let grab = crate::shell::LayerClickGrab {
            start_data: PointerGrabStartData { focus: Some(under), button, location },
            layer,
            output,
            was_at,
        };
        let pointer = self.pointer.clone();
        pointer.set_grab(self, grab, serial, smithay::input::pointer::Focus::Keep);
    }

    pub(super) fn on_pointer_axis<B: InputBackend>(&mut self, evt: B::PointerAxisEvent) {
        self.expander.clear();

        let factor = crate::scroll::factor(&self.config.pointer, evt.source());
        let horizontal = self.scroll_scaler.scale(0, factor, evt.amount(input::Axis::Horizontal),
                                                  evt.amount_v120(input::Axis::Horizontal));
        let vertical = self.scroll_scaler.scale(1, factor, evt.amount(input::Axis::Vertical),
                                                evt.amount_v120(input::Axis::Vertical));
        let (horizontal_amount, horizontal_amount_discrete) = (horizontal.value, horizontal.v120);
        let (vertical_amount, vertical_amount_discrete) = (vertical.value, vertical.v120);

        {
            let mut frame = AxisFrame::new(evt.time_msec()).source(evt.source());
            if horizontal_amount != 0.0 {
                frame = frame.relative_direction(Axis::Horizontal, evt.relative_direction(Axis::Horizontal));
                frame = frame.value(Axis::Horizontal, horizontal_amount);
                if let Some(discrete) = horizontal_amount_discrete {
                    frame = frame.v120(Axis::Horizontal, discrete);
                }
            }
            if vertical_amount != 0.0 {
                frame = frame.relative_direction(Axis::Vertical, evt.relative_direction(Axis::Vertical));
                frame = frame.value(Axis::Vertical, vertical_amount);
                if let Some(discrete) = vertical_amount_discrete {
                    frame = frame.v120(Axis::Vertical, discrete);
                }
            }
            if evt.source() == AxisSource::Finger {
                if evt.amount(Axis::Horizontal) == Some(0.0) {
                    frame = frame.stop(Axis::Horizontal);
                }
                if evt.amount(Axis::Vertical) == Some(0.0) {
                    frame = frame.stop(Axis::Vertical);
                }
            }
            let pointer = self.pointer.clone();
            pointer.axis(self, frame);
            pointer.frame(self);
        }
    }

    pub(super) fn on_pointer_move_absolute_windowed<B: InputBackend>(
        &mut self,
        evt: B::PointerMotionAbsoluteEvent,
        output: &Output,
    ) {
        let output_geo = self.space.output_geometry(output).unwrap();

        let pos = evt.position_transformed(output_geo.size) + output_geo.loc.to_f64();
        let pos = self.clamp_coords(pos);
        if self.moved_while_held(pos, evt.time_msec()) {
            return;
        }
        let serial = SCOUNTER.next_serial();

        let pointer = self.pointer.clone();
        let under = self.surface_under(pos);
        pointer.motion(
            self,
            under,
            &MotionEvent {
                location: pos,
                serial,
                time: evt.time_msec(),
            },
        );
        pointer.frame(self);
        self.update_edge_cursor(pos);
    }

    pub fn refresh_pointer_focus(&mut self) {
        let pointer = self.pointer.clone();

        if pointer.is_grabbed() || self.capture.is_active() {
            return;
        }
        let pos = pointer.current_location();
        let under = self.surface_under(pos);
        if under.as_ref().map(|(f, _)| f) == pointer.current_focus().as_ref() {
            return;
        }
        pointer.motion(self, under, &MotionEvent { location: pos, serial: SCOUNTER.next_serial(), time: self.clock.now().as_millis() });
        pointer.frame(self);
    }

    pub(super) fn on_pointer_move<B: InputBackend>(&mut self, evt: B::PointerMotionEvent) {
        self.pointer_moved(evt.delta(), evt.delta_unaccel(), evt.time(), evt.time_msec());
    }

    fn pointer_moved(&mut self, delta: Point<f64, Logical>, delta_unaccel: Point<f64, Logical>, utime: u64, time: u32) {
        let mut pointer_location = self.pointer.current_location();
        let was = pointer_location;
        let serial = SCOUNTER.next_serial();

        let pointer = self.pointer.clone();
        let under = self.surface_under(pointer_location);

        let held = pointer_hold(&pointer, under.as_ref(), pointer_location);

        pointer.relative_motion(
            self,
            under.clone(),
            &RelativeMotionEvent { delta, delta_unaccel, utime },
        );

        if held.locked {
            pointer.frame(self);
            return;
        }

        pointer_location += delta;
        let tried = pointer_location;

        pointer_location = self.clamp_coords(pointer_location);

        if held.confined {
            match confined_move(under.as_ref(), held.region.as_ref(), was, pointer_location) {
                Some(at) => pointer_location = at,
                None => {
                    pointer.frame(self);
                    return;
                }
            }
        }

        let new_under = self.surface_under(pointer_location);

        if held.confined && leaves_surface(under.as_ref(), new_under.as_ref()) {
            pointer.frame(self);
            return;
        }

        let arrived = MotionEvent { location: pointer_location, serial, time };
        self.pointer_arrived(new_under, &arrived, tried, held.confined);
    }

    fn pointer_arrived(&mut self, new_under: Option<(PointerFocusTarget, Point<f64, Logical>)>, arrived: &MotionEvent, tried: Point<f64, Logical>, confined: bool) {
        let pointer = self.pointer.clone();

        pointer.motion(self, new_under.clone(), arrived);
        pointer.frame(self);
        self.update_edge_cursor(arrived.location);
        if !confined {
            self.cross_push(tried, arrived.location, arrived.time);
        }
        self.activate_constraint_under(new_under, arrived.location);
    }

    fn activate_constraint_under(&mut self, new_under: Option<(PointerFocusTarget, Point<f64, Logical>)>, pointer_location: Point<f64, Logical>) {
        let pointer = self.pointer.clone();
        if let Some((under, surface_location)) =
            new_under.and_then(|(target, loc)| Some((target.wl_surface()?.into_owned(), loc)))
        {
            let may = self.may_hold_pointer(&under);
            with_pointer_constraint(&under, &pointer, |constraint| match constraint {
                Some(constraint) if may && !constraint.is_active() => {
                    let point = (pointer_location - surface_location).to_i32_round();
                    if constraint.region().is_none_or(|region| region.contains(point)) {
                        constraint.activate();
                    }
                }
                _ => {}
            });
        }
    }

    pub(super) fn on_pointer_move_absolute<B: InputBackend>(&mut self, evt: B::PointerMotionAbsoluteEvent) {
        let serial = SCOUNTER.next_serial();

        let screens = self.screens();
        let Some(pointer_location) = screen_edge::absolute_to_screens(&screens, evt.x_transformed(1), evt.y_transformed(1)) else { return };
        if self.moved_while_held(pointer_location, evt.time_msec()) {
            return;
        }

        let pointer = self.pointer.clone();
        let under = self.surface_under(pointer_location);

        pointer.motion(
            self,
            under,
            &MotionEvent {
                location: pointer_location,
                serial,
                time: evt.time_msec(),
            },
        );
        pointer.frame(self);
        self.update_edge_cursor(pointer_location);
    }

    fn moved_while_held(&mut self, to: Point<f64, Logical>, time: u32) -> bool {
        let pointer = self.pointer.clone();
        let Some(focus) = pointer.current_focus().and_then(|f| f.wl_surface().map(|s| s.into_owned())) else { return false };
        if !with_pointer_constraint(&focus, &pointer, |c| c.is_some_and(|c| c.is_active())) {
            return false;
        }
        let delta = to - pointer.current_location();
        self.pointer_moved(delta, delta, u64::from(time) * 1000, time);
        true
    }

    pub(crate) fn clamp_coords(&self, pos: Point<f64, Logical>) -> Point<f64, Logical> {
        screen_edge::clamp_to_screens(&self.screens(), pos).unwrap_or(pos)
    }

}

#[derive(Default)]
struct Hold {
    locked: bool,
    confined: bool,
    region: Option<RegionAttributes>,
}

fn pointer_hold(
    pointer: &PointerHandle<AnvilState<impl Backend>>,
    under: Option<&(PointerFocusTarget, Point<f64, Logical>)>,
    pointer_location: Point<f64, Logical>,
) -> Hold {
    let mut held = Hold::default();
    let Some((surface, surface_loc)) = under.and_then(|(target, l)| Some((target.wl_surface()?, l))) else { return held };
    with_pointer_constraint(&surface, pointer, |constraint| match constraint {
        Some(constraint) if constraint.is_active() => {

            if !constraint.region().is_none_or(|x| {
                x.contains((pointer_location - *surface_loc).to_i32_round())
            }) {
                return;
            }
            match &*constraint {
                PointerConstraint::Locked(_locked) => {
                    held.locked = true;
                }
                PointerConstraint::Confined(confine) => {
                    held.confined = true;
                    held.region = confine.region().cloned();
                }
            }
        }
        _ => {}
    });
    held
}

fn confined_move(
    under: Option<&(PointerFocusTarget, Point<f64, Logical>)>,
    region: Option<&RegionAttributes>,
    was: Point<f64, Logical>,
    to: Point<f64, Logical>,
) -> Option<Point<f64, Logical>> {
    let Some((surface, surface_loc)) = under else { return Some(to) };
    surface.wl_surface().and_then(|s| held_inside(&s, *surface_loc, region, was, to))
}

fn leaves_surface(
    under: Option<&(PointerFocusTarget, Point<f64, Logical>)>,
    new_under: Option<&(PointerFocusTarget, Point<f64, Logical>)>,
) -> bool {
    let Some((surface, _)) = under else { return false };
    new_under.and_then(|(under, _)| under.wl_surface()) != surface.wl_surface()
}

fn held_inside(
    surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    origin: Point<f64, Logical>,
    region: Option<&smithay::wayland::compositor::RegionAttributes>,
    from: Point<f64, Logical>,
    to: Point<f64, Logical>,
) -> Option<Point<f64, Logical>> {
    use smithay::wayland::compositor::RectangleKind;
    let size = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.surface_size()).flatten()?;
    let mut area = Rectangle::new((0, 0).into(), size);
    if let Some(region) = region {
        let start = (from - origin).to_i32_round();
        let part = region.rects.iter().find(|(kind, r)| matches!(kind, RectangleKind::Add) && r.contains(start))?.1;
        area = area.intersection(part)?;
    }
    let want = to - origin;
    let at: Point<f64, Logical> = (
        want.x.clamp(f64::from(area.loc.x), f64::from(area.loc.x + area.size.w - 1)),
        want.y.clamp(f64::from(area.loc.y), f64::from(area.loc.y + area.size.h - 1)),
    )
        .into();
    if region.is_some_and(|r| !r.contains(at.to_i32_round())) {
        return None;
    }
    Some(origin + at)
}
