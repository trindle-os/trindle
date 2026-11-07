//! trindle_night_v1 (night.xml).

use smithay::output::Output;
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New,
};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/night.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/night.xml");
}

pub use server::trindle_night_manager_v1::{self as manager, TrindleNightManagerV1};
pub use server::trindle_night_v1::{self as tint, Reason, TrindleNightV1};

const VERSION: u32 = 1;

const LEAST_SHARE: u16 = u16::MAX / 10;

pub trait NightHandler {
    fn night_state(&mut self) -> &mut NightState;
    fn colour_table_size(&mut self, output: &Output) -> Option<u32>;
    fn set_colour_table(&mut self, output: &Output, table: Option<Vec<u16>>) -> bool;
}

pub fn table(size: u32, shares: [u16; 3]) -> Vec<u16> {
    let last = u64::from(size.saturating_sub(1).max(1));
    shares
        .iter()
        .flat_map(|&share| {
            let share = u64::from(share.max(LEAST_SHARE));
            (0..u64::from(size)).map(move |i| (share * i / last).min(u64::from(u16::MAX)) as u16)
        })
        .collect()
}

pub struct NightGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub struct TintData;

#[derive(Debug, Default)]
pub struct NightState {
    held: Vec<(Output, TrindleNightV1)>,
}

impl NightState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> NightState
    where
        D: GlobalDispatch<TrindleNightManagerV1, NightGlobalData>
            + Dispatch<TrindleNightManagerV1, ()>
            + Dispatch<TrindleNightV1, TintData>
            + NightHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleNightManagerV1, _>(VERSION, NightGlobalData { filter: Box::new(filter) });
        NightState::default()
    }

    pub fn output_removed(&mut self, output: &Output) {
        self.held.retain(|(o, _)| o != output);
    }

    fn holder_of(&self, tint: &TrindleNightV1) -> Option<&Output> {
        self.held.iter().find(|(_, t)| t == tint).map(|(o, _)| o)
    }
}

impl<D> GlobalDispatch<TrindleNightManagerV1, NightGlobalData, D> for NightState
where
    D: GlobalDispatch<TrindleNightManagerV1, NightGlobalData>
        + Dispatch<TrindleNightManagerV1, ()>
        + Dispatch<TrindleNightV1, TintData>
        + NightHandler
        + 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleNightManagerV1>,
        _: &NightGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, data: &NightGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleNightManagerV1, (), D> for NightState
where
    D: Dispatch<TrindleNightManagerV1, ()> + Dispatch<TrindleNightV1, TintData> + NightHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        _: &TrindleNightManagerV1,
        request: manager::Request,
        _: &(),
        _: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            manager::Request::Destroy => {}
            manager::Request::Tint { id, output } => {
                let output = Output::from_resource(&output);
                let made = data_init.init(id, TintData);
                let Some(output) = output else { return made.refused(Reason::Unsupported) };
                if state.night_state().held.iter().any(|(o, _)| *o == output) {
                    return made.refused(Reason::Taken);
                }
                if state.colour_table_size(&output).is_none_or(|n| n < 2) {
                    return made.refused(Reason::Unsupported);
                }
                state.night_state().held.push((output, made));
            }
        }
    }
}

impl<D> Dispatch<TrindleNightV1, TintData, D> for NightState
where
    D: Dispatch<TrindleNightV1, TintData> + NightHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        object: &TrindleNightV1,
        request: tint::Request,
        _: &TintData,
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            tint::Request::Destroy => {}
            tint::Request::Warm { red, green, blue } => {
                let Some(output) = state.night_state().holder_of(object).cloned() else { return };
                let Some(size) = state.colour_table_size(&output).filter(|&n| n >= 2) else {
                    return object.refused(Reason::Unsupported);
                };
                let share = |v: u32| v.min(u32::from(u16::MAX)) as u16;
                if !state.set_colour_table(&output, Some(table(size, [share(red), share(green), share(blue)]))) {
                    state.night_state().held.retain(|(_, t)| t != object);
                    object.refused(Reason::Unsupported);
                }
            }
        }
    }

    fn destroyed(state: &mut D, _: ClientId, object: &TrindleNightV1, _: &TintData) {
        let st = state.night_state();
        let held = st.holder_of(object).cloned();
        st.held.retain(|(_, t)| t != object);
        if let Some(output) = held {
            state.set_colour_table(&output, None);
        }
    }
}

#[macro_export]
macro_rules! delegate_night {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::night::TrindleNightManagerV1: $crate::protocols::night::NightGlobalData
        ] => $crate::protocols::night::NightState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::night::TrindleNightManagerV1: ()
        ] => $crate::protocols::night::NightState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::night::TrindleNightV1: $crate::protocols::night::TintData
        ] => $crate::protocols::night::NightState);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_full_share_gives_a_straight_ramp() {
        let t = table(256, [u16::MAX; 3]);
        assert_eq!(t.len(), 768);
        assert_eq!((t[0], t[255], t[256 + 255], t[512 + 255]), (0, 65535, 65535, 65535));
        assert_eq!(t[128], (65535u32 * 128 / 255) as u16);
        assert!(t.windows(2).take(255).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn a_smaller_share_lowers_only_its_own_colour() {
        let t = table(256, [u16::MAX, 40_000, 20_000]);
        assert_eq!((t[255], t[256 + 255], t[512 + 255]), (65535, 40_000, 20_000));
        assert!(t[512..768].windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(t[256], 0, "black stays black");
    }

    #[test]
    fn no_colour_can_be_tinted_away_to_nothing() {
        let t = table(256, [0, 1, 6000]);
        let floor = u16::MAX / 10;
        assert_eq!((t[255], t[256 + 255], t[512 + 255]), (floor, floor, floor));
    }

    #[test]
    fn any_table_size_from_two_up_is_filled_end_to_end() {
        for size in [2u32, 3, 17, 1024, 4096] {
            let t = table(size, [65535, 50000, 30000]);
            assert_eq!(t.len(), 3 * size as usize);
            let n = size as usize;
            assert_eq!((t[0], t[n - 1], t[2 * n - 1], t[3 * n - 1]), (0, 65535, 50000, 30000), "size {size}");
        }
    }
}
