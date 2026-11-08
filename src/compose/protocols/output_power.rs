//! Screens off and on for wlopm (wlr-output-power-management-unstable-v1).

use smithay::output::Output;
use smithay::reexports::wayland_protocols_wlr::output_power_management::v1::server::{
    zwlr_output_power_manager_v1::{self, ZwlrOutputPowerManagerV1},
    zwlr_output_power_v1::{self, Mode, ZwlrOutputPowerV1},
};
use smithay::reexports::wayland_server::{
    backend::ClientId, Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, WEnum,
};

const VERSION: u32 = 1;

#[derive(Debug)]
pub struct OutputPowerState {

    controls: Vec<(ZwlrOutputPowerV1, Output)>,
}

pub struct OutputPowerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub trait OutputPowerHandler {
    fn output_power_state(&mut self) -> &mut OutputPowerState;

    fn output_power(&mut self, output: &Output) -> Option<bool>;

    fn set_output_power(&mut self, output: &Output, on: bool) -> bool;
}

fn mode(on: bool) -> Mode {
    if on {
        Mode::On
    } else {
        Mode::Off
    }
}

impl OutputPowerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<ZwlrOutputPowerManagerV1, OutputPowerGlobalData> + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global_data = OutputPowerGlobalData {
            filter: Box::new(filter),
        };
        display.create_global::<D, ZwlrOutputPowerManagerV1, _>(VERSION, global_data);
        Self { controls: Vec::new() }
    }

    pub fn mode_changed(&self, output: &Output, on: bool) {
        for (control, _) in self.controls.iter().filter(|(_, o)| o == output) {
            control.mode(mode(on));
        }
    }

    pub fn output_removed(&mut self, output: &Output) {
        self.controls.retain(|(control, o)| {
            if o == output {
                control.failed();
            }
            o != output
        });
    }
}

impl<D> GlobalDispatch<ZwlrOutputPowerManagerV1, OutputPowerGlobalData, D> for OutputPowerState
where
    D: GlobalDispatch<ZwlrOutputPowerManagerV1, OutputPowerGlobalData>
        + Dispatch<ZwlrOutputPowerManagerV1, ()>
        + 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<ZwlrOutputPowerManagerV1>,
        _global_data: &OutputPowerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &OutputPowerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<ZwlrOutputPowerManagerV1, (), D> for OutputPowerState
where
    D: Dispatch<ZwlrOutputPowerManagerV1, ()> + Dispatch<ZwlrOutputPowerV1, ()> + OutputPowerHandler + 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        _manager: &ZwlrOutputPowerManagerV1,
        request: zwlr_output_power_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        if let zwlr_output_power_manager_v1::Request::GetOutputPower { id, output } = request {
            let control = data_init.init(id, ());
            let known = Output::from_resource(&output)
                .and_then(|output| state.output_power(&output).map(|on| (output, on)));
            match known {
                Some((output, on)) => {
                    control.mode(mode(on));
                    state.output_power_state().controls.push((control, output));
                }

                None => control.failed(),
            }
        }
    }
}

impl<D> Dispatch<ZwlrOutputPowerV1, (), D> for OutputPowerState
where
    D: Dispatch<ZwlrOutputPowerV1, ()> + OutputPowerHandler + 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        control: &ZwlrOutputPowerV1,
        request: zwlr_output_power_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        let zwlr_output_power_v1::Request::SetMode { mode: wanted } = request else {
            return;
        };
        let on = match wanted {
            WEnum::Value(Mode::On) => true,
            WEnum::Value(Mode::Off) => false,
            _ => {
                control.post_error(zwlr_output_power_v1::Error::InvalidMode, "no such power mode");
                return;
            }
        };
        let found = state
            .output_power_state()
            .controls
            .iter()
            .find(|(c, _)| c == control)
            .map(|(_, output)| output.clone());

        let Some(output) = found else {
            return;
        };
        let was = state.output_power(&output);
        if state.set_output_power(&output, on) {
            if was != Some(on) {
                state.output_power_state().mode_changed(&output, on);
            }
        } else {
            control.failed();
            state.output_power_state().controls.retain(|(c, _)| c != control);
        }
    }

    fn destroyed(state: &mut D, _client: ClientId, control: &ZwlrOutputPowerV1, _data: &()) {
        state.output_power_state().controls.retain(|(c, _)| c != control);
    }
}

#[macro_export]
macro_rules! delegate_output_power {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::output_power_management::v1::server::zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1: $crate::protocols::output_power::OutputPowerGlobalData
        ] => $crate::protocols::output_power::OutputPowerState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::output_power_management::v1::server::zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1: ()
        ] => $crate::protocols::output_power::OutputPowerState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::output_power_management::v1::server::zwlr_output_power_v1::ZwlrOutputPowerV1: ()
        ] => $crate::protocols::output_power::OutputPowerState);
    };
}
