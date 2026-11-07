// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Panels: the layer shell, who holds the whole keyboard, and keeping the panels in order.

use smithay::utils::SERIAL_COUNTER;
use smithay::wayland::seat::WaylandFocus;

use smithay::{
    desktop::{layer_map_for_output, LayerSurface, PopupManager, WindowSurfaceType},
    output::Output,
    reexports::wayland_server::{
        protocol::{wl_output, wl_surface::WlSurface},
        Resource,
    },
    utils::{Logical, Point},
    wayland::{
        compositor::with_states,
        shell::wlr_layer::{
            ExclusiveZone, KeyboardInteractivity, Layer, LayerSurface as WlrLayerSurface,
            LayerSurfaceCachedState, LayerSurfaceData, WlrLayerShellHandler, WlrLayerShellState,
        },
    },
};

use crate::state::{AnvilState, Backend};

impl<BackendData: Backend> WlrLayerShellHandler for AnvilState<BackendData> {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: WlrLayerSurface,
        wl_output: Option<wl_output::WlOutput>,
        layer_kind: Layer,
        namespace: String,
    ) {
        if namespace == crate::trusted::MODAL && !self.modal_asked(&surface, layer_kind) {
            return;
        }
        let output = wl_output
            .as_ref()
            .and_then(Output::from_resource)
            .unwrap_or_else(|| self.space.outputs().next().unwrap().clone());
        let mut map = layer_map_for_output(&output);
        let question = crate::person_only::ASKED_BY_NAME.contains(&namespace.as_str()) && crate::trusted::outside_every_box(surface.wl_surface());
        map.map_layer(&LayerSurface::new(surface, namespace)).unwrap();
        crate::trusted::keep_on_top(&mut map);
        drop(map);
        if question {
            self.leave_full_screen_for_a_question();
        }
    }

    fn new_popup(&mut self, _parent: WlrLayerSurface, popup: smithay::wayland::shell::xdg::PopupSurface) {
        self.unconstrain_popup(&popup);
    }

    fn layer_destroyed(&mut self, surface: WlrLayerSurface) {
        if let Some((mut map, layer)) = self.space.outputs().find_map(|o| {
            let map = layer_map_for_output(o);
            let layer = map
                .layers()
                .find(|&layer| layer.layer_surface() == &surface)
                .cloned();
            layer.map(|layer| (map, layer))
        }) {
            map.unmap_layer(&layer);
        }
        self.modal_gone(surface.wl_surface());
        self.refit_login_screen();
        self.fit_windows_to_room();
        self.refresh_pointer_focus();
        let wl = surface.wl_surface().clone();
        if self.keyboard_layers.contains(&wl) {
            self.keyboard_layers.retain(|s| s != &wl);
            self.keyboard_after_panel();
        }
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn panel_of(&self, surface: &WlSurface) -> Option<LayerSurface> {
        self.space.outputs().find_map(|o| layer_map_for_output(o).layer_for_surface(surface, WindowSurfaceType::TOPLEVEL).cloned())
    }

    pub fn whole_keyboard_panel(&self, surface: &WlSurface) -> Option<LayerSurface> {
        let showing = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.buffer().is_some())
            .unwrap_or(false);
        let modal = self.the_modal().is_some_and(|m| m.wl_surface() == surface);
        let wants = modal || with_states(surface, |states| {
            let data = *states.cached_state.get::<LayerSurfaceCachedState>().current();
            data.keyboard_interactivity == KeyboardInteractivity::Exclusive
                && matches!(data.layer, Layer::Top | Layer::Overlay)
        });
        if !showing || !wants {
            return None;
        }
        if self.question_window_in_front() && self.panel_of(surface).is_some_and(|l| crate::trusted::standing(&l) == crate::trusted::Kind::News) {
            return None;
        }
        self.space.outputs().find_map(|o| {
            let layer = layer_map_for_output(o).layer_for_surface(surface, WindowSurfaceType::TOPLEVEL).cloned();
            layer.filter(|l| !crate::trusted::hidden_by_full_screen(o, l) && !crate::trusted::too_small(l))
        })
    }

    pub fn question_window_in_front(&self) -> bool {
        self.space.elements().last().and_then(|w| w.wl_surface()).is_some_and(|s| crate::trusted::window_kept_for_the_person(&s))
    }

    pub fn whole_keyboard_panels(&self) -> Vec<LayerSurface> {
        let mut panels: Vec<LayerSurface> =
            self.layer_shell_state.layer_surfaces().rev().filter_map(|l| self.whole_keyboard_panel(l.wl_surface())).collect();
        panels.sort_by_key(|l| {
            std::cmp::Reverse((crate::trusted::standing(l), self.keyboard_layers.iter().position(|s| s == l.wl_surface())))
        });
        panels
    }

    pub(crate) fn keyboard_after_panel(&mut self) {
        if self.is_locked() {
            return;
        }
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        let panel = self.whole_keyboard_panels().into_iter().find(|p| p.wl_surface().client().is_some());
        let next: Option<crate::focus::KeyboardFocusTarget> = match panel {
            Some(panel) => Some(panel.into()),
            None => self.space.elements().last().cloned().map(Into::into),
        };
        if let Some(next) = next {
            keyboard.set_focus(self, Some(next), SERIAL_COUNTER.next_serial());
        }
    }

    pub(super) fn keyboard_to_new_layer(&mut self, surface: &WlSurface) {
        let showing = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.buffer().is_some())
            .unwrap_or(false);
        if !showing && self.keyboard_layers.contains(surface) {
            self.keyboard_layers.retain(|s| s != surface);
            self.keyboard_after_panel();
        }
        if self.keyboard_layers.contains(surface) || self.is_locked() {
            return;
        }
        let Some(layer) = self.whole_keyboard_panel(surface) else { return };
        let above = self.whole_keyboard_panels().iter().any(|p| crate::trusted::standing(p) > crate::trusted::standing(&layer));
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        self.keyboard_layers.push(surface.clone());
        if above {
            return;
        }
        self.drop_window_grabs();
        self.drop_keyboard_grab_for(surface);
        keyboard.set_focus(self, Some(layer.into()), SERIAL_COUNTER.next_serial());
    }

    pub fn drop_keyboard_grab_for(&mut self, surface: &WlSurface) {
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        let Some(start) = keyboard.grab_start_data() else { return };
        let own = start.focus.as_ref().and_then(|f| f.wl_surface().and_then(|s| s.client()))
            .is_some_and(|c| Some(c) == surface.client());
        if !own {
            keyboard.unset_grab(self);
        }
    }

    pub fn drop_window_grabs(&mut self) {
        let roots: Vec<_> = self.space.elements().filter_map(|w| w.wl_surface().map(|s| (*s).clone())).collect();
        let mut closed = false;
        for root in roots {
            let popups: Vec<_> = PopupManager::popups_for_surface(&root).map(|(p, _)| p).collect();
            for popup in popups {
                closed |= PopupManager::dismiss_popup(&root, &popup).is_ok();
            }
        }
        if !closed {
            return;
        }
        if let Some(keyboard) = self.seat.get_keyboard() {
            if keyboard.is_grabbed() {
                keyboard.unset_grab(self);
            }
        }
        let pointer = self.pointer.clone();
        if pointer.is_grabbed() {
            pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), self.clock.now().as_millis());
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum LayerShown {
    Shown,
    PutAway,
}

fn layer_comes_back(surface: &WlSurface) -> bool {
    let has_buffer = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.buffer().is_some())
        .unwrap_or(false);
    with_states(surface, |states| {
        states.data_map.insert_if_missing(|| std::cell::Cell::new(None::<LayerShown>));
        let cell = states.data_map.get::<std::cell::Cell<Option<LayerShown>>>().unwrap();
        match (cell.get(), has_buffer) {
            (_, true) => {
                cell.set(Some(LayerShown::Shown));
                false
            }
            (Some(LayerShown::Shown), false) => {
                cell.set(Some(LayerShown::PutAway));
                false
            }
            (Some(LayerShown::PutAway), false) => {
                cell.set(None);
                true
            }
            (None, false) => false,
        }
    })
}

fn put_away_keeps_no_room(surface: &WlSurface) {
    let shows = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.buffer().is_some())
        .unwrap_or(false);
    if shows {
        return;
    }
    with_states(surface, |states| {
        let mut cached = states.cached_state.get::<LayerSurfaceCachedState>();
        let now = cached.current();
        if matches!(now.exclusive_zone, ExclusiveZone::Exclusive(_)) {
            now.exclusive_zone = ExclusiveZone::Neutral;
        }
    });
}

fn to_top(map: &mut smithay::desktop::LayerMap, surface: &WlSurface) {
    let Some(layer) = map.layer_for_surface(surface, WindowSurfaceType::TOPLEVEL).cloned() else { return };
    map.unmap_layer(&layer);
    let _ = map.map_layer(&layer);
    crate::trusted::keep_on_top(map);
}

pub fn takes_input(map: &smithay::desktop::LayerMap, layer: &LayerSurface, at: Point<f64, Logical>) -> bool {
    map.layer_geometry(layer)
        .is_some_and(|g| layer.surface_under(at - g.loc.to_f64(), WindowSurfaceType::ALL).is_some())
}

pub fn layer_taking_input<'a>(
    map: &'a smithay::desktop::LayerMap,
    layers: &[Layer],
    at: Point<f64, Logical>,
    keep: impl Fn(&LayerSurface) -> bool,
) -> Option<&'a LayerSurface> {
    layers.iter().find_map(|&on| map.layers_on(on).rev().find(|l| keep(l) && takes_input(map, l, at)))
}

fn room_takers_first(map: &mut smithay::desktop::LayerMap) {
    let takes = |l: &LayerSurface| matches!(l.cached_state().exclusive_zone, ExclusiveZone::Exclusive(_));
    let layers: Vec<LayerSurface> = map.layers().cloned().collect();
    let Some(last_taker) = layers.iter().rposition(takes) else { return };
    let behind: Vec<&LayerSurface> = layers[..last_taker].iter().filter(|l| !takes(l)).collect();
    for layer in &behind {
        map.unmap_layer(layer);
    }
    for layer in behind {
        let _ = map.map_layer(layer);
    }
}

pub(super) fn configure_layer(surface: &WlSurface, output: &Output) {
    let initial_configure_sent = with_states(surface, |states| {
        states
            .data_map
            .get::<LayerSurfaceData>()
            .unwrap()
            .lock()
            .unwrap()
            .initial_configure_sent
    });

    let back = layer_comes_back(surface);
    put_away_keeps_no_room(surface);
    let mut map = layer_map_for_output(output);
    if back {
        to_top(&mut map, surface);
    }
    room_takers_first(&mut map);

    map.arrange();
    if !initial_configure_sent || back {
        let layer = map
            .layer_for_surface(surface, WindowSurfaceType::TOPLEVEL)
            .unwrap();

        layer.layer_surface().send_configure();
    }
}
