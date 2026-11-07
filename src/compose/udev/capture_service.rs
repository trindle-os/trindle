// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Screenshots and screen sharing: filling the copies programs have asked for from what was just drawn.

use std::time::Duration;

use smithay::output::Output;
use tracing::warn;

use super::{AnvilState, UdevData};
use crate::capture_scene::Scene;

impl AnvilState<UdevData> {
    pub(super) fn service_image_capture(&mut self, output: &Output) {
        self.keep_pointer_away();
        let outputs = self.image_capture_state.has_output_frames(output);
        let toplevels = self.image_capture_state.has_toplevel_frames();
        let pointers = self.image_capture_state.has_pointers();
        if !outputs && !toplevels && !pointers {
            return;
        }
        let pointer_location = self.pointer.current_location();
        let now = Duration::from(self.clock.now());
        let windows = if toplevels || pointers { self.capture_windows() } else { Vec::new() };
        let primary_gpu = self.backend_data.primary_gpu;
        let mut renderer = match self.backend_data.gpus.single_renderer(&primary_gpu) {
            Ok(renderer) => renderer,
            Err(err) => {
                warn!("image capture: no renderer on the primary GPU: {err}");
                return;
            }
        };
        let scene = Scene {
            space: &self.space,
            pointer_element: &self.backend_data.pointer_element,
            cursor_status: &self.cursor_status,
            dnd_icon: self.dnd_icon.as_ref(),
            session_lock: &self.session_lock,
            pointer_location,
            windows: &windows,
            now,
        };
        let state = &mut self.image_capture_state;
        if outputs {
            scene.fill_output(state, &mut renderer, output, &self.handle);
        }
        if toplevels {
            scene.fill_toplevels(state, &mut renderer, &self.handle);
        }
        if pointers {
            scene.fill_pointers(state, &mut renderer, &self.handle);
        }
    }
}
