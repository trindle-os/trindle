// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use crate::{focus::PointerFocusTarget, shell::FullscreenSurface, state::Backend, AnvilState};

use smithay::{
    desktop::{layer_map_for_output, LayerMap, LayerSurface, WindowSurfaceType},
    input::keyboard::KeyboardHandle, output::Output,
    utils::{Logical, Point, Rectangle, Serial},
    wayland::{input_method::InputMethodSeat, shell::wlr_layer::Layer as WlrLayer},
};

const ABOVE: [WlrLayer; 2] = [WlrLayer::Overlay, WlrLayer::Top];
const BELOW: [WlrLayer; 2] = [WlrLayer::Bottom, WlrLayer::Background];

type Under = (PointerFocusTarget, Point<i32, Logical>);

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(super) fn update_keyboard_focus(&mut self, location: Point<f64, Logical>, serial: Serial) {
        if self.is_locked() {
            self.focus_lock_at(location);
            return;
        }
        let keyboard = self.seat.get_keyboard().unwrap();
        let touch = self.seat.get_touch();
        let input_method = self.seat.input_method();

        if !self.pointer.is_grabbed()
            && (!keyboard.is_grabbed() || input_method.keyboard_grabbed())
            && !touch.map(|touch| touch.is_grabbed()).unwrap_or(false)
        {
            let output = self.space.output_under(location).next().cloned();
            if let Some(output) = output.as_ref() {
                if self.focus_above_windows(&keyboard, output, location, serial) {
                    return;
                }
            }

            if let Some((window, _)) = self.space.element_under(location).map(|(w, p)| (w.clone(), p)) {

                if crate::screen_edge::at_outer_edge(&self.screens(), location, self.config.gap) {
                    return;
                }

                self.focus_window(&window);
                return;
            }

            if let Some(output) = output.as_ref() {
                self.focus_layer_at(&keyboard, output, &BELOW, location, serial);
            };
        }
    }

    fn focus_above_windows(&mut self, keyboard: &KeyboardHandle<Self>, output: &Output, location: Point<f64, Logical>, serial: Serial) -> bool {
        let output_geo = self.space.output_geometry(output).unwrap();

        let full = output.user_data().get::<FullscreenSurface>().and_then(|f| f.get()).is_some();
        let trusted = {
            let layers = layer_map_for_output(output);
            let at = location - output_geo.loc.to_f64();
            crate::trusted::over_full_screen_under(&layers, at, full)
                .filter(|l| {
                    l.can_receive_keyboard_focus()
                        && l.surface_under(at - layers.layer_geometry(l).unwrap().loc.to_f64(), WindowSurfaceType::ALL)
                            .is_some()
                })
                .cloned()
        };
        if let Some(layer) = trusted {
            keyboard.set_focus(self, Some(layer.into()), serial);
            return true;
        }
        if let Some(window) = output
            .user_data()
            .get::<FullscreenSurface>()
            .and_then(|f| f.get())
        {
            if let Some((_, _)) =
                window.surface_under(location - output_geo.loc.to_f64(), WindowSurfaceType::ALL)
            {
                if let Some(surface) = window.0.x11_surface() {
                    self.raise_x11(surface);
                }
                keyboard.set_focus(self, Some(window.into()), serial);
                return true;
            }
        }
        self.focus_layer_at(keyboard, output, &ABOVE, location, serial)
    }

    fn focus_layer_at(&mut self, keyboard: &KeyboardHandle<Self>, output: &Output, which: &[WlrLayer], location: Point<f64, Logical>, serial: Serial) -> bool {
        let output_geo = self.space.output_geometry(output).unwrap();
        let layers = layer_map_for_output(output);
        let Some(layer) = crate::shell::layer_taking_input(&layers, which, location - output_geo.loc.to_f64(), |_| true) else { return false };
        if !layer.can_receive_keyboard_focus() {
            return false;
        }
        let at = location - output_geo.loc.to_f64() - layers.layer_geometry(layer).unwrap().loc.to_f64();
        if layer.surface_under(at, WindowSurfaceType::ALL).is_none() {
            return false;
        }
        keyboard.set_focus(self, Some(layer.clone().into()), serial);
        true
    }

    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        if self.is_locked() {
            return self.lock_surface_under(pos);
        }

        if let Some(found) = self.modal_under(pos) {
            return found.and_then(|(layer, origin)| {
                layer.surface_under(pos - origin, WindowSurfaceType::ALL)
                    .map(|(surface, loc)| (PointerFocusTarget::from(surface), (loc.to_f64() + origin)))
            });
        }

        let output = self.space.outputs().find(|o| {
            let geometry = self.space.output_geometry(o).unwrap();
            geometry.contains(pos.to_i32_floor())
        })?;
        self.surface_under_output(output, pos).map(|(s, l)| (s, l.to_f64()))
    }

    fn surface_under_output(&self, output: &Output, pos: Point<f64, Logical>) -> Option<Under> {
        let output_geo = self.space.output_geometry(output).unwrap();
        let layers = layer_map_for_output(output);

        let full = output.user_data().get::<FullscreenSurface>().and_then(|f| f.get()).is_some();
        if let Some(focus) =
            crate::trusted::over_full_screen_under(&layers, pos - output_geo.loc.to_f64(), full)
                .and_then(|layer| layer_focus(&layers, output_geo, pos, layer))
        {

            return Some(focus);
        }
        if let Some((surface, loc)) = output
            .user_data()
            .get::<FullscreenSurface>()
            .and_then(|f| f.get())
            .and_then(|w| w.surface_under(pos - output_geo.loc.to_f64(), WindowSurfaceType::ALL))
        {
            return Some((surface, loc + output_geo.loc));
        }
        if let Some(focus) = layer_on(&layers, &ABOVE, output_geo, pos) {
            return Some(focus);
        }
        if self.edge_under(pos).is_some() {

            return None;
        }
        if let Some(focus) = self.xdnd_element_under(pos).and_then(|(window, loc)| {
            window
                .surface_under(pos - loc.to_f64(), WindowSurfaceType::ALL)
                .map(|(surface, surf_loc)| (surface, surf_loc + loc))
        }) {
            return Some(focus);
        }
        layer_on(&layers, &BELOW, output_geo, pos)
    }
}

fn layer_focus(layers: &LayerMap, output_geo: Rectangle<i32, Logical>, pos: Point<f64, Logical>, layer: &LayerSurface) -> Option<Under> {
    let layer_loc = layers.layer_geometry(layer).unwrap().loc;
    layer
        .surface_under(pos - output_geo.loc.to_f64() - layer_loc.to_f64(), WindowSurfaceType::ALL)
        .map(|(surface, loc)| (PointerFocusTarget::from(surface), loc + layer_loc + output_geo.loc))
}

fn layer_on(layers: &LayerMap, which: &[WlrLayer], output_geo: Rectangle<i32, Logical>, pos: Point<f64, Logical>) -> Option<Under> {
    crate::shell::layer_taking_input(layers, which, pos - output_geo.loc.to_f64(), |_| true)
        .and_then(|layer| layer_focus(layers, output_geo, pos, layer))
}
