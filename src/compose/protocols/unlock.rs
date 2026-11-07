//! trindle_unlock_v1 (unlock.xml).

use smithay::reexports::wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/unlock.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/unlock.xml");
}

pub use server::trindle_unlock_v1::{self as unlock, TrindleUnlockV1};

pub trait UnlockHandler {
    fn unlock_with_pin(&mut self, client: &Client, pin: &str) -> Result<(), String>;
}

pub struct UnlockGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub struct UnlockState;

impl UnlockState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> UnlockState
    where
        D: GlobalDispatch<TrindleUnlockV1, UnlockGlobalData> + Dispatch<TrindleUnlockV1, ()> + UnlockHandler + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleUnlockV1, _>(1, UnlockGlobalData { filter: Box::new(filter) });
        UnlockState
    }
}

impl<D> GlobalDispatch<TrindleUnlockV1, UnlockGlobalData, D> for UnlockState
where
    D: GlobalDispatch<TrindleUnlockV1, UnlockGlobalData> + Dispatch<TrindleUnlockV1, ()> + UnlockHandler + 'static,
{
    fn bind(_: &mut D, _: &DisplayHandle, _: &Client, resource: New<TrindleUnlockV1>, _: &UnlockGlobalData, data_init: &mut DataInit<'_, D>) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &UnlockGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleUnlockV1, (), D> for UnlockState
where
    D: Dispatch<TrindleUnlockV1, ()> + UnlockHandler + 'static,
{
    fn request(state: &mut D, client: &Client, object: &TrindleUnlockV1, request: unlock::Request, _: &(), _: &DisplayHandle, _: &mut DataInit<'_, D>) {
        match request {
            unlock::Request::Destroy => {}
            unlock::Request::WithPin { pin } => match state.unlock_with_pin(client, &pin) {
                Ok(()) => object.done(),
                Err(why) => object.refused(why),
            },
        }
    }
}

#[macro_export]
macro_rules! delegate_unlock {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::unlock::TrindleUnlockV1: $crate::protocols::unlock::UnlockGlobalData
        ] => $crate::protocols::unlock::UnlockState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::unlock::TrindleUnlockV1: ()
        ] => $crate::protocols::unlock::UnlockState);
    };
}
