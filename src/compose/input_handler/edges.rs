// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Window edges: the shapes and clicks that resize a window, and the empty desktop under the pointer.

use super::BTN_LEFT;
use crate::{
    screen_edge,
    shell::{ssd::BORDER, FullscreenSurface, ResizeEdge, WindowElement},
    state::Backend, AnvilState,
};

use smithay::{
    desktop::{layer_map_for_output, space::SpaceElement},
    input::pointer::{CursorImageStatus, GrabStartData as PointerGrabStartData},
    utils::{Logical, Point, Rectangle, Serial, Size},
    wayland::{
        compositor::with_states,
        shell::{wlr_layer::Layer as WlrLayer, xdg::SurfaceCachedState},
    },
};

const EDGE_MARGIN: i32 = 8;
const EDGE_CORNER: i32 = 20;

fn resizable(window: &WindowElement) -> bool {
    if window.is_fullscreen() || window.is_maximized() {
        return false;
    }
    if let Some(x11) = window.0.x11_surface() {
        if x11.is_override_redirect() {
            return false;
        }
        return match (x11.min_size(), x11.max_size()) {
            (Some(min), Some(max)) => min != max,
            _ => true,
        };
    }
    let Some(surface) = window.wl_surface() else { return false };
    let (min, max) = with_states(&surface, |states| {
        let mut cached = states.cached_state.get::<SurfaceCachedState>();
        let current = cached.current();
        (current.min_size, current.max_size)
    });
    min.w == 0 || min != max
}

impl<BackendData: Backend> AnvilState<BackendData> {
    fn nothing_under(&self, location: Point<f64, Logical>, windows_ok: bool) -> bool {
        if !windows_ok && (self.space.element_under(location).is_some() || self.edge_under(location).is_some()) {
            return false;
        }
        let Some(output) = self.space.output_under(location).next() else { return true };
        let Some(geo) = self.space.output_geometry(output) else { return true };
        let layers = layer_map_for_output(output);
        let here = location - geo.loc.to_f64();
        crate::shell::layer_taking_input(&layers, &[WlrLayer::Overlay, WlrLayer::Top, WlrLayer::Bottom], here, |_| true)
            .is_none()
    }

    pub fn edge_under(&self, pos: Point<f64, Logical>) -> Option<(WindowElement, ResizeEdge)> {
        for window in self.space.elements().rev() {
            let Some(loc) = self.space.element_location(window) else { continue };
            let geo = Rectangle::new(loc, window.geometry().size).to_f64();
            let render_loc = (loc - window.geometry().loc).to_f64();
            if window.is_in_input_region(&(pos - render_loc)) {
                return None;
            }
            let outside = EDGE_MARGIN as f64;
            let grown = Rectangle::new(
                geo.loc - Point::from((outside, outside)),
                geo.size + Size::from((2.0 * outside, 2.0 * outside)),
            );
            if !grown.contains(pos) {
                continue;
            }
            if !resizable(window) {
                return None;
            }
            let border = if window.has_ssd() { BORDER as f64 } else { 0.0 };
            let (x0, x1) = (geo.loc.x + border, geo.loc.x + geo.size.w - border);
            let (y0, y1) = (geo.loc.y + border, geo.loc.y + geo.size.h - border);
            let mut edges = ResizeEdge::NONE;
            if pos.x < x0 {
                edges |= ResizeEdge::LEFT;
            } else if pos.x >= x1 {
                edges |= ResizeEdge::RIGHT;
            }
            if pos.y < y0 {
                edges |= ResizeEdge::TOP;
            } else if pos.y >= y1 {
                edges |= ResizeEdge::BOTTOM;
            }
            let corner = EDGE_CORNER as f64;
            if edges.intersects(ResizeEdge::LEFT | ResizeEdge::RIGHT) {
                if pos.y < geo.loc.y + corner {
                    edges |= ResizeEdge::TOP;
                } else if pos.y >= geo.loc.y + geo.size.h - corner {
                    edges |= ResizeEdge::BOTTOM;
                }
            }
            if edges.intersects(ResizeEdge::TOP | ResizeEdge::BOTTOM) {
                if pos.x < geo.loc.x + corner {
                    edges |= ResizeEdge::LEFT;
                } else if pos.x >= geo.loc.x + geo.size.w - corner {
                    edges |= ResizeEdge::RIGHT;
                }
            }
            return (edges != ResizeEdge::NONE).then(|| (window.clone(), edges));
        }
        None
    }

    pub(crate) fn screens(&self) -> Vec<Rectangle<i32, Logical>> {
        self.space.outputs().filter_map(|o| self.space.output_geometry(o)).collect()
    }

    pub(super) fn desktop_double(&mut self, pos: Point<f64, Logical>, time: u32, slop: f64, grabbed: bool) -> bool {
        let first = self.desktop_tap.take();
        if self.is_locked() || grabbed {
            return false;
        }
        let Some(output) = self.space.output_under(pos).next() else { return false };
        if output.user_data().get::<FullscreenSurface>().and_then(|f| f.get()).is_some() {
            return false;
        }
        let at_edge = screen_edge::at_outer_edge(&self.screens(), pos, self.config.gap);
        if !self.nothing_under(pos, at_edge) {
            return false;
        }
        let double = first.is_some_and(|(then, at)| {
            time.wrapping_sub(then) <= crate::pointer::DOUBLE_CLICK_MS && (pos - at).to_f64().x.hypot((pos - at).to_f64().y) <= slop
        });
        if !double {
            self.desktop_tap = Some((time, pos));
        }
        double
    }

    pub(crate) fn tap_desktop(&mut self, pos: Point<f64, Logical>, time: u32, grabbed: bool) {
        if self.desktop_double(pos, time, crate::pointer::DOUBLE_TAP_SLOP, grabbed) {
            if let Some(cmd) = self.config.desktop_double_click.clone() {
                self.spawn(&cmd);
            }
        }
    }

    pub(super) fn update_edge_cursor(&mut self, pos: Point<f64, Logical>) {
        if self.pointer.is_grabbed() {
            return;
        }
        let edge = if self.surface_under(pos).is_none() { self.edge_under(pos) } else { None };
        match edge {
            Some((_, edges)) => {
                self.cursor_status = CursorImageStatus::Named(edges.cursor());
                self.edge_cursor = true;
            }
            None if self.edge_cursor => {
                self.edge_cursor = false;
                if matches!(self.cursor_status, CursorImageStatus::Named(_)) {
                    self.cursor_status = CursorImageStatus::default_named();
                }
            }
            None => {}
        }
    }

    pub(super) fn press_on_edge(&mut self, button: u32, serial: Serial) -> bool {
        let pos = self.pointer.current_location();
        if button != BTN_LEFT || self.pointer.is_grabbed() || self.surface_under(pos).is_some() {
            return false;
        }
        if screen_edge::at_outer_edge(&self.screens(), pos, self.config.gap) {
            return false;
        }
        let Some((window, edges)) = self.edge_under(pos) else { return false };
        self.focus_window(&window);
        let start_data = PointerGrabStartData { focus: None, button, location: pos };
        self.start_pointer_resize(window, edges, start_data, serial);
        true
    }
}
