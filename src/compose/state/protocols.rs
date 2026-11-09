// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::{AnvilState, Backend, ClientState};

use std::sync::Arc;

use tracing::warn;

use smithay::{
    delegate_fractional_scale, delegate_layer_shell, delegate_output, delegate_presentation,
    delegate_security_context, delegate_xdg_activation, delegate_xdg_decoration,
    delegate_xdg_shell, desktop::utils::surface_primary_scanout_output, input::Seat,
    output::Output,
    reexports::{
        wayland_protocols::xdg::decoration::{
            self as xdg_decoration,
            zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
        },
        wayland_server::{protocol::wl_surface::WlSurface, Client, Resource},
    },
    wayland::{
        compositor::{get_parent, with_states},
        fractional_scale::{with_fractional_scale, FractionalScaleHandler},
        output::OutputHandler,
        security_context::{
            SecurityContext, SecurityContextHandler, SecurityContextListenerSource,
        },
        shell::xdg::{decoration::XdgDecorationHandler, ToplevelSurface},
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken,
            XdgActivationTokenData,
        },
        xdg_foreign::{XdgForeignHandler, XdgForeignState},
    },
};

use crate::{
    focus::KeyboardFocusTarget,
    protocols::image_capture::{delegate_image_capture, ImageCaptureHandler, ImageCaptureState},
};
use crate::protocols::{
    virtual_pointer::{PointerAction, VirtualPointerHandler},
    night::{NightHandler, NightState},
    screens::{ScreenFacts, ScreenPlan, ScreensHandler, ScreensState},
    output_power::{OutputPowerHandler, OutputPowerState},
};
use smithay::{
    delegate_xwayland_keyboard_grab, delegate_xwayland_shell, utils::Size,
    wayland::xwayland_keyboard_grab::XWaylandKeyboardGrabHandler,
};

impl<BackendData: Backend> OutputHandler for AnvilState<BackendData> {}
delegate_output!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> XdgActivationHandler for AnvilState<BackendData> {
    fn activation_state(&mut self) -> &mut XdgActivationState {
        &mut self.xdg_activation_state
    }

    fn token_created(&mut self, _token: XdgActivationToken, data: XdgActivationTokenData) -> bool {
        let last_enter = self.seat.get_keyboard().and_then(|k| k.last_enter());
        match data.serial {
            Some((serial, seat)) => Seat::from_resource(&seat) == Some(self.seat.clone()) && activation_allowed(true, Some(serial), last_enter),
            None => false,
        }
    }

    fn request_activation(
        &mut self,
        _token: XdgActivationToken,
        token_data: XdgActivationTokenData,
        surface: WlSurface,
    ) {

        let own = token_data.client_id.is_some() && token_data.client_id == self.display_handle.get_client(surface.id()).ok().map(|c| c.id());
        let last_enter = self.seat.get_keyboard().and_then(|k| k.last_enter());
        let current = activation_allowed(own, token_data.serial.map(|(serial, _)| serial), last_enter);
        if current && token_data.timestamp.elapsed().as_secs() < 10 {

            crate::protocols::windows::WindowsHandler::raise_window(self, surface);
        }
    }
}

fn activation_allowed(own: bool, serial: Option<smithay::utils::Serial>, last_enter: Option<smithay::utils::Serial>) -> bool {
    !own || serial.zip(last_enter).is_some_and(|(serial, enter)| serial.is_no_older_than(&enter))
}

#[cfg(test)]
mod activation_tests {
    use super::activation_allowed;
    use smithay::utils::Serial;

    #[test]
    fn a_program_cannot_bring_itself_back_with_a_token_made_before_the_keyboard_moved() {
        let (old, new) = (Serial::from(5), Serial::from(9));
        assert!(!activation_allowed(true, Some(old), Some(new)), "a stale own token raised a window");
        assert!(!activation_allowed(true, None, Some(new)));
        assert!(activation_allowed(true, Some(new), Some(new)));
        assert!(activation_allowed(false, Some(old), Some(new)), "a token from the ring or another program was refused");
    }
}

delegate_xdg_activation!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> XdgDecorationHandler for AnvilState<BackendData> {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        use xdg_decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });
    }
    fn request_mode(&mut self, toplevel: ToplevelSurface, mode: DecorationMode) {
        use xdg_decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(match mode {
                DecorationMode::ServerSide => Mode::ServerSide,
                _ => Mode::ClientSide,
            });
        });

        if toplevel.is_initial_configure_sent() {
            toplevel.send_pending_configure();
        }
    }
    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        use xdg_decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });

        if toplevel.is_initial_configure_sent() {
            toplevel.send_pending_configure();
        }
    }
}
delegate_xdg_decoration!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_xdg_shell!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
delegate_layer_shell!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
delegate_presentation!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> FractionalScaleHandler for AnvilState<BackendData> {
    fn new_fractional_scale(
        &mut self,
        surface: smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {

        #[allow(clippy::redundant_clone)]
        let mut root = surface.clone();
        while let Some(parent) = get_parent(&root) {
            root = parent;
        }

        with_states(&surface, |states| {
            let primary_scanout_output = surface_primary_scanout_output(&surface, states)
                .or_else(|| {
                    if root != surface {
                        with_states(&root, |states| {
                            surface_primary_scanout_output(&root, states).or_else(|| {
                                self.window_for_surface(&root).and_then(|window| {
                                    self.space.outputs_for_element(&window).first().cloned()
                                })
                            })
                        })
                    } else {
                        self.window_for_surface(&root)
                            .and_then(|window| self.space.outputs_for_element(&window).first().cloned())
                    }
                })
                .or_else(|| self.space.outputs().next().cloned());
            if let Some(output) = primary_scanout_output {
                with_fractional_scale(states, |fractional_scale| {
                    fractional_scale.set_preferred_scale(output.current_scale().fractional_scale());
                });
            }
        });
    }
}
delegate_fractional_scale!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> SecurityContextHandler for AnvilState<BackendData> {
    fn context_created(&mut self, source: SecurityContextListenerSource, security_context: SecurityContext) {
        self.handle
            .insert_source(source, move |client_stream, _, data| {
                let client_state = ClientState {
                    security_context: Some(security_context.clone()),
                    ..ClientState::default()
                };
                if let Err(err) = data
                    .display_handle
                    .insert_client(client_stream, Arc::new(client_state))
                {
                    warn!("Error adding wayland client: {}", err);
                };
            })
            .expect("Failed to init wayland socket source");
    }
}
delegate_security_context!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> XWaylandKeyboardGrabHandler for AnvilState<BackendData> {
    fn keyboard_focus_for_xsurface(&self, surface: &WlSurface) -> Option<KeyboardFocusTarget> {
        let elem = self
            .space
            .elements()
            .find(|elem| elem.wl_surface().as_deref() == Some(surface))?;
        Some(KeyboardFocusTarget::Window(elem.0.clone()))
    }
}
delegate_xwayland_keyboard_grab!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

delegate_xwayland_shell!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> XdgForeignHandler for AnvilState<BackendData> {
    fn xdg_foreign_state(&mut self) -> &mut XdgForeignState {
        &mut self.xdg_foreign_state
    }
}
smithay::delegate_xdg_foreign!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

smithay::delegate_single_pixel_buffer!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

smithay::delegate_fifo!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

smithay::delegate_commit_timing!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> ImageCaptureHandler for AnvilState<BackendData> {
    fn image_capture_state(&mut self) -> &mut ImageCaptureState {
        &mut self.image_capture_state
    }
    fn toplevel_capture_size(
        &self,
        handle: &smithay::wayland::foreign_toplevel_list::ForeignToplevelHandle,
    ) -> Option<Size<i32, smithay::utils::Physical>> {
        let window = self.window_of(handle)?;
        Some(crate::panel::capture_size(&window, self.window_scale(&window)))
    }
}
delegate_image_capture!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> smithay::wayland::foreign_toplevel_list::ForeignToplevelListHandler for AnvilState<BackendData> {
    fn foreign_toplevel_list_state(&mut self) -> &mut smithay::wayland::foreign_toplevel_list::ForeignToplevelListState {
        &mut self.toplevel_list_state
    }
}
smithay::delegate_foreign_toplevel_list!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> ScreensHandler for AnvilState<BackendData> {
    fn screens_state(&mut self) -> &mut ScreensState {
        &mut self.screens_state
    }
    fn screens_now(&mut self) -> Vec<ScreenFacts> {
        BackendData::output_heads(self)
    }
    fn arrange(&mut self, plan: &[ScreenPlan], test_only: bool) -> Result<(), String> {
        BackendData::apply_output_config(self, plan, test_only)
    }
}
crate::delegate_screens!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> OutputPowerHandler for AnvilState<BackendData> {
    fn output_power_state(&mut self) -> &mut OutputPowerState {
        &mut self.output_power_state
    }
    fn output_power(&mut self, output: &Output) -> Option<bool> {
        self.backend_data.output_power(output)
    }
    fn set_output_power(&mut self, output: &Output, on: bool) -> bool {
        BackendData::set_output_power(self, output, on)
    }
}
crate::delegate_output_power!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> NightHandler for AnvilState<BackendData> {
    fn night_state(&mut self) -> &mut NightState {
        &mut self.night_state
    }
    fn colour_table_size(&mut self, output: &Output) -> Option<u32> {
        self.backend_data.gamma_size(output)
    }
    fn set_colour_table(&mut self, output: &Output, table: Option<Vec<u16>>) -> bool {
        self.backend_data.set_gamma(output, table)
    }
}
crate::delegate_night!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> VirtualPointerHandler for AnvilState<BackendData> {
    fn virtual_pointer(&mut self, client: &Client, action: PointerAction) {
        self.virtual_input(crate::virtual_input::maker(client), action.into());
    }
}
crate::delegate_virtual_pointer!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
