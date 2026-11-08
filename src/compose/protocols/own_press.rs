//! trindle_own_press_v1 (own_press.xml): a program is told which of the
//! presses it is passed the person made themselves. The ring the
//! compositor keeps hears the same through a pipe (common/own_press.rs);
//! this is that for every other window of Trindle's, which the compositor
//! did not start and can hand nothing: Activity's Stop ends a program, and
//! could not tell a plugin's made-up click from the person's.

use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/own_press.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/own_press.xml");
}

pub use server::trindle_own_press_v1::{self as own, TrindleOwnPressV1};

const VERSION: u32 = 3;

const REFUSED_EVERY: std::time::Duration = std::time::Duration::from_secs(1);

pub trait OwnPressHandler {
    fn own_press_state(&mut self) -> &mut OwnPressState;
}

#[derive(Debug, Default)]
pub struct OwnPressState {
    told: Vec<TrindleOwnPressV1>,

    refused: Vec<(smithay::reexports::wayland_server::backend::ClientId, std::time::Instant)>,
}

pub struct OwnPressGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl OwnPressState {

    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> OwnPressState
    where
        D: GlobalDispatch<TrindleOwnPressV1, OwnPressGlobalData> + Dispatch<TrindleOwnPressV1, ()> + OwnPressHandler + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleOwnPressV1, _>(VERSION, OwnPressGlobalData { filter: Box::new(filter) });
        OwnPressState::default()
    }

    pub fn tell(&mut self, to: &WlSurface, serial: u32) {
        let Some(whose) = to.client().map(|c| c.id()) else { return };
        self.told.retain(|t| t.is_alive());
        for told in self.told.iter().filter(|t| t.client().is_some_and(|c| c.id() == whose)) {
            told.own(serial);
        }
    }

    pub fn refused(&mut self, to: &WlSurface, now: std::time::Instant) -> bool {
        let Some(whose) = to.client().map(|c| c.id()) else { return false };
        self.told.retain(|t| t.is_alive());
        self.refused.retain(|(_, at)| now.saturating_duration_since(*at) < REFUSED_EVERY);
        if self.refused.iter().any(|(who, _)| *who == whose) {
            return false;
        }
        let mut said = false;
        for told in self.told.iter().filter(|t| t.version() >= 2 && t.client().is_some_and(|c| c.id() == whose)) {
            told.refused(to);
            said = true;
        }
        if said {
            self.refused.push((whose, now));
        }
        said
    }
}

impl OwnPressState {

    pub fn tell_was(&mut self, to: &WlSurface, serial: u32, code: u32) {
        let Some(whose) = to.client().map(|c| c.id()) else { return };
        self.told.retain(|t| t.is_alive());
        for told in self.told.iter().filter(|t| t.version() >= 3 && t.client().is_some_and(|c| c.id() == whose)) {
            told.was(serial, code);
        }
    }
}

impl<D> GlobalDispatch<TrindleOwnPressV1, OwnPressGlobalData, D> for OwnPressState
where
    D: GlobalDispatch<TrindleOwnPressV1, OwnPressGlobalData> + Dispatch<TrindleOwnPressV1, ()> + OwnPressHandler + 'static,
{
    fn bind(
        state: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleOwnPressV1>,
        _: &OwnPressGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        let told = data_init.init(resource, ());
        state.own_press_state().told.push(told);
    }

    fn can_view(client: Client, global_data: &OwnPressGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleOwnPressV1, (), D> for OwnPressState
where
    D: Dispatch<TrindleOwnPressV1, ()> + OwnPressHandler + 'static,
{
    fn request(_: &mut D, _: &Client, _: &TrindleOwnPressV1, request: own::Request, _: &(), _: &DisplayHandle, _: &mut DataInit<'_, D>) {
        match request {
            own::Request::Destroy => {}
        }
    }

    fn destroyed(state: &mut D, _: smithay::reexports::wayland_server::backend::ClientId, gone: &TrindleOwnPressV1, _: &()) {
        state.own_press_state().told.retain(|t| t != gone);
    }
}

#[macro_export]
macro_rules! delegate_own_press {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::own_press::TrindleOwnPressV1: $crate::protocols::own_press::OwnPressGlobalData
        ] => $crate::protocols::own_press::OwnPressState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::own_press::TrindleOwnPressV1: ()
        ] => $crate::protocols::own_press::OwnPressState);
    };
}
