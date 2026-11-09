// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    backend::session::Session,
    input::keyboard::LedState,
    output::{Mode as WlMode, Output},
    reexports::wayland_server::protocol::wl_surface,
};
use tracing::warn;

use super::{
    configure_pointer,
    gamma::{apply_gamma, gamma_length, linear_ramp, read_gamma},
    output_config, AnvilState, UdevData, UdevOutputId,
};
use crate::{
    protocols::screens::{ScreenFacts, ScreenPlan},
    state::Backend,
};

impl Backend for UdevData {
    const HAS_RELATIVE_MOTION: bool = true;
    const HAS_GESTURES: bool = true;

    fn unlocked(&mut self) {
        self.newcomers.unlocked();
    }

    fn seat_name(&self) -> String {
        self.session.seat()
    }

    fn reset_buffers(&mut self, output: &Output) {
        if let Some(id) = output.user_data().get::<UdevOutputId>() {
            if let Some(gpu) = self.backends.get_mut(&id.device_id) {
                if let Some(surface) = gpu.surfaces.get_mut(&id.crtc) {
                    surface.drm_output.reset_buffers();
                }
            }
        }
    }

    fn early_import(&mut self, surface: &wl_surface::WlSurface) {
        if let Err(err) = self.gpus.early_import(self.primary_gpu, surface) {
            warn!("Early buffer import failed: {}", err);
        }
    }

    fn update_led_state(&mut self, led_state: LedState) {
        for keyboard in self.keyboards.iter_mut() {
            keyboard.led_update(led_state.into());
        }
    }

    fn apply_input_settings(&mut self, pointer: &crate::config::Pointer) {
        for device in self.pointers.iter_mut() {
            configure_pointer(device, pointer);
        }
    }

    fn reload_pointer(&mut self) {
        if self.pointer_image.stale() {
            self.pointer_image = crate::cursor::Cursor::load();

            self.pointer_images.clear();
        }
    }

    fn output_heads(state: &AnvilState<Self>) -> Vec<ScreenFacts> {
        let mut heads = Vec::new();
        for device in state.backend_data.backends.values() {
            for surface in device.surfaces.values() {
                let output = &surface.output;
                let physical = output.physical_properties();
                let (mode, adaptive_sync) = surface
                    .drm_output
                    .with_compositor(|c| (WlMode::from(c.pending_mode()), c.vrr_enabled()));
                let location = output.current_location();
                heads.push(ScreenFacts {
                    name: output.name(),
                    description: output.description(),
                    make: physical.make,
                    model: physical.model,
                    serial: surface.serial.clone(),
                    size_mm: Some((physical.size.w, physical.size.h)),
                    modes: surface.mode_infos.clone(),
                    on: surface.enabled,
                    mode: surface.mode_infos.iter().position(|m| {
                        (m.width, m.height, m.millihertz) == (mode.size.w, mode.size.h, mode.refresh)
                    }),
                    position: (location.x, location.y),
                    turn: output.current_transform(),
                    scale: output.current_scale().fractional_scale(),
                    varies_rate: adaptive_sync,
                });
            }
        }

        heads.sort_by(|a, b| a.name.cmp(&b.name));
        heads
    }

    fn apply_output_config(
        state: &mut AnvilState<Self>,
        targets: &[ScreenPlan],
        test_only: bool,
    ) -> Result<(), String> {
        output_config::apply(state, targets, test_only)
    }

    fn output_power(&self, output: &Output) -> Option<bool> {
        let surface = self.surface_of(output)?;
        surface.enabled.then_some(surface.powered)
    }

    fn set_output_power(state: &mut AnvilState<Self>, output: &Output, on: bool) -> bool {
        let Some(&UdevOutputId { device_id: node, crtc }) = output.user_data().get::<UdevOutputId>() else {
            return false;
        };
        let active = state.backend_data.session.is_active();
        let Some(surface) = state
            .backend_data
            .backends
            .get_mut(&node)
            .and_then(|device| device.surfaces.get_mut(&crtc))
        else {
            return false;
        };
        if !surface.enabled {
            return false;
        }
        if surface.powered == on {
            return true;
        }
        if on {

            surface.powered = true;
            surface.drm_output.reset_buffers();
            state
                .handle
                .insert_idle(move |state| state.render_surface(node, crtc, state.clock.now()));
        } else {

            if active {
                if let Err(err) = surface.drm_output.with_compositor(|c| c.clear()) {
                    warn!(?err, "could not switch {} off", output.name());
                    return false;
                }
            }
            surface.powered = false;
            if let Some(timer) = surface.vblank_throttle_timer.take() {
                state.handle.remove(timer);
            }
        }
        true
    }

    fn gamma_size(&self, output: &Output) -> Option<u32> {
        let id = output.user_data().get::<UdevOutputId>()?;
        let device = self.backends.get(&id.device_id)?;
        gamma_length(device.drm_output_manager.device(), id.crtc).map(|n| n as u32)
    }

    fn set_gamma(&mut self, output: &Output, ramp: Option<Vec<u16>>) -> bool {
        let active = self.session.is_active();
        let Some(&UdevOutputId { device_id, crtc }) = output.user_data().get::<UdevOutputId>() else {
            return false;
        };
        let Some(device) = self.backends.get_mut(&device_id) else {
            return false;
        };
        let drm = device.drm_output_manager.device();
        let Some(surface) = device.surfaces.get_mut(&crtc) else {
            return false;
        };
        let Some(size) = gamma_length(drm, crtc) else {
            return false;
        };
        if ramp.as_ref().is_some_and(|ramp| ramp.len() != size * 3) {
            return false;
        }
        if !active {

            surface.gamma = ramp;
            return true;
        }
        if ramp.is_some() && surface.original_gamma.is_none() {

            surface.original_gamma = Some(read_gamma(drm, crtc, size).unwrap_or_else(|_| linear_ramp(size)));
        }
        let Some(table) = ramp.as_ref().or(surface.original_gamma.as_ref()) else {
            return true;
        };
        match apply_gamma(drm, crtc, table) {
            Ok(()) => {
                surface.gamma = ramp;
                true
            }
            Err(err) => {
                warn!(?err, "could not set the colour ramp of {}", output.name());
                false
            }
        }
    }
}
