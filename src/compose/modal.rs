// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The modal request: a trusted question says "nothing else gets clicks or keys until I am answered".
//!
//! The request is a layer panel named `trindle-modal` (trusted::MODAL) on the overlay layer, from a
//! program that connected as root. Nothing else can make one: any other program that tries is
//! closed before anything is drawn. While it is out, drawn and at least 16 points each way:
//! - the pointer is over it or over nothing, so no click, scroll or touch reaches another window;
//! - the keyboard is its own whatever it asked for, so no key reaches a window or a shortcut;
//! - no window is given the keyboard or raised by a program's request;
//! - the mouse and keyboard are not lent to another computer (Flow's crossing included), as for
//!   every question only the person may answer (trusted::asks).
//!
//! It can never trap the person: Escape on a real keyboard ends it, so does the time limit, so does
//! its owner going or crashing (it is a panel of the owner's connection), and a panel under 16
//! points is no modal at all. The lock screen and the login screen keep priority: no request is
//! accepted under either, and an accepted one gives way when the screen locks. Only one at a time.

use std::time::{Duration, Instant};

use smithay::{
    reexports::calloop::timer::{TimeoutAction, Timer},
    desktop::{layer_map_for_output, LayerSurface},
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    utils::{Logical, Point},
    wayland::shell::wlr_layer::{Layer, LayerSurface as WlrLayerSurface},
};
use tracing::warn;

use crate::state::{AnvilState, Backend};

pub const LIMIT: Duration = Duration::from_secs(120);

#[derive(Debug)]
pub struct Held {
    surface: WlSurface,
    since: Instant,
    pointer_taken: bool,
}

impl<BackendData: Backend> AnvilState<BackendData> {
    fn modal_refusal(&self, surface: &WlrLayerSurface, layer: Layer) -> Option<&'static str> {
        if !crate::trusted::is_trusted(surface.wl_surface()) {
            Some("only a trusted panel may ask")
        } else if layer != Layer::Overlay {
            Some("it must be on the overlay layer")
        } else if self.is_locked() || crate::session::launch().greeter {
            Some("the lock or login screen comes first")
        } else if self.modal_taken() {
            Some("another is already out")
        } else {
            None
        }
    }

    pub(crate) fn modal_asked(&mut self, surface: &WlrLayerSurface, layer: Layer) -> bool {
        self.modal_asked_with(surface, layer, |state, wl| {
            state.handle.insert_source(Timer::from_duration(state.modal_limit), move |_, _, state| {
                if state.modal.as_ref().is_some_and(|m| m.surface == wl) {
                    state.end_modal("the time limit passed");
                }
                TimeoutAction::Drop
            }).map(|_| ()).map_err(|e| e.to_string())
        })
    }

    fn modal_asked_with(&mut self, surface: &WlrLayerSurface, layer: Layer, arm: impl FnOnce(&mut Self, WlSurface) -> Result<(), String>) -> bool {
        if let Some(why) = self.modal_refusal(surface, layer) {
            warn!(why, "a modal request was refused");
            surface.send_close();
            return false;
        }
        let wl = surface.wl_surface().clone();
        if let Err(why) = arm(self, wl.clone()) {
            warn!(why, "a modal request was refused: its time limit could not be set");
            surface.send_close();
            return false;
        }
        self.modal = Some(Held { surface: wl, since: Instant::now(), pointer_taken: false });
        true
    }

    fn modal_taken(&self) -> bool {
        self.modal.as_ref().is_some_and(|m| m.surface.is_alive() && m.since.elapsed() < self.modal_limit && self.panel_of(&m.surface).is_some())
    }

    pub(crate) fn the_modal(&self) -> Option<LayerSurface> {
        let held = self.modal.as_ref()?;
        if self.is_locked() || held.since.elapsed() >= self.modal_limit {
            return None;
        }
        let drawn = smithay::backend::renderer::utils::with_renderer_surface_state(&held.surface, |s| s.buffer().is_some()).unwrap_or(false);
        let layer = self.panel_of(&held.surface)?;
        (drawn && crate::trusted::counts(&layer)).then_some(layer)
    }

    pub(crate) fn end_modal(&mut self, why: &str) {
        let Some(held) = self.modal.take() else { return };
        warn!(why, "a modal ended");
        let Some(layer) = self.panel_of(&held.surface) else { return };
        layer.layer_surface().send_close();
        for o in self.space.outputs().cloned().collect::<Vec<_>>() {
            layer_map_for_output(&o).unmap_layer(&layer);
        }
        self.keyboard_layers.retain(|s| s != &held.surface);
        self.keyboard_after_panel();
        self.refresh_pointer_focus();
    }

    pub(crate) fn modal_gone(&mut self, surface: &WlSurface) {
        if self.modal.as_ref().is_some_and(|m| &m.surface == surface) {
            self.modal = None;
        }
    }

    pub(crate) fn modal_committed(&mut self, surface: &WlSurface) -> bool {
        if self.modal.as_ref().is_some_and(|m| &m.surface == surface && !m.pointer_taken) && self.the_modal().is_some() {
            if let Some(m) = self.modal.as_mut() {
                m.pointer_taken = true;
            }
            self.drop_window_grabs();
            self.refresh_pointer_focus();
            return true;
        }
        false
    }

    pub(crate) fn modal_under(&self, pos: Point<f64, Logical>) -> Option<Option<(smithay::desktop::LayerSurface, Point<f64, Logical>)>> {
        let layer = self.the_modal()?;
        let found = self.space.outputs().find_map(|o| {
            let map = layer_map_for_output(o);
            let at = map.layer_geometry(&layer)?.loc;
            let out = self.space.output_geometry(o)?;
            Some((layer.clone(), at.to_f64() + out.loc.to_f64()))
        });
        Some(found.filter(|(l, origin)| {
            l.surface_under(pos - *origin, smithay::desktop::WindowSurfaceType::ALL).is_some()
        }))
    }

    pub(crate) fn modal_keeps_back(&mut self, at: Point<f64, Logical>) -> bool {
        let Some(under) = self.modal_under(at) else { return false };
        if under.is_some() {
            return false;
        }
        if let Some(layer) = self.the_modal() {
            self.own_press_state.refused(layer.wl_surface(), Instant::now());
        }
        true
    }

    pub(crate) fn is_modal_panel(&self, surface: &WlSurface) -> bool {
        self.the_modal().is_some_and(|m| m.wl_surface() == surface)
    }

    pub(crate) fn modal_escape(&mut self) {
        self.end_modal("escape was pressed");
    }
}

#[cfg(test)]
mod tests {
    use crate::rig::{alone, Rig};
    use crate::state::ClientState;
    use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::Layer as On;
    use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::KeyboardInteractivity;

    #[test]
    fn the_pointer_is_taken_once_not_at_every_frame() {
        alone("modal::tests::the_pointer_is_taken_once_not_at_every_frame", |dir| {
            let mut rig = Rig::new(dir);
            let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
            let modal = rig.panel_sized(root, crate::trusted::MODAL, On::Overlay, 200, 40, KeyboardInteractivity::None);
            let seen = rig.server_side(root, &modal);
            assert!(rig.state.modal.as_ref().is_some_and(|m| m.pointer_taken), "the first picture did not take the pointer");
            assert!(!rig.state.modal_committed(&seen), "a later picture took the pointer again");
            rig.show(root, &modal, 200, 40);
            assert!(!rig.state.modal_committed(&seen), "an animated modal took the pointer at every frame");
            if let Some(m) = rig.state.modal.as_mut() {
                m.pointer_taken = false;
            }
            assert!(rig.state.modal_committed(&seen));
            assert!(!rig.state.modal_committed(&seen));
        });
    }

    #[test]
    fn a_modal_whose_time_limit_cannot_be_set_is_closed_at_once() {
        alone("modal::tests::a_modal_whose_time_limit_cannot_be_set_is_closed_at_once", |dir| {
            let mut rig = Rig::new(dir);
            let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
            let panel = rig.panel_sized(root, "org.example.question", On::Overlay, 200, 40, KeyboardInteractivity::None);
            let seen = rig.server_side(root, &panel);
            let layer = rig.state.panel_of(&seen).expect("the panel is out");
            let kept = rig.state.modal_asked_with(layer.layer_surface(), smithay::wayland::shell::wlr_layer::Layer::Overlay, |_, _| Err("no timer".into()));
            rig.pump();
            assert!(!kept, "a modal with no time limit was accepted");
            assert!(rig.state.modal.is_none());
            assert_eq!(rig.closed(root), 1, "the owner was not told it was refused");
        });
    }
}
