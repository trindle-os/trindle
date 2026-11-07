//! Utilities for text input support.

use wayland_protocols::wp::text_input::zv3::server::{
    zwp_text_input_manager_v3::{self, ZwpTextInputManagerV3},
    zwp_text_input_v3::ZwpTextInputV3,
};
use wayland_server::{backend::GlobalId, Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};

use crate::input::{Seat, SeatHandler};

pub use text_input_handle::TextInputHandle;
pub use text_input_handle::TextInputUserData;

use super::input_method::InputMethodHandle;

const MANAGER_VERSION: u32 = 1;

mod text_input_handle;

pub trait TextInputSeat {
    fn text_input(&self) -> &TextInputHandle;
}

impl<D: SeatHandler + 'static> TextInputSeat for Seat<D> {
    fn text_input(&self) -> &TextInputHandle {
        let user_data = self.user_data();
        user_data.insert_if_missing(TextInputHandle::default);
        user_data.get::<TextInputHandle>().unwrap()
    }
}

#[derive(Debug)]
pub struct TextInputManagerState {
    global: GlobalId,
}

impl TextInputManagerState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<ZwpTextInputManagerV3, ()>,
        D: Dispatch<ZwpTextInputManagerV3, ()>,
        D: Dispatch<ZwpTextInputV3, TextInputUserData>,
        D: 'static,
    {
        let global = display.create_global::<D, ZwpTextInputManagerV3, _>(MANAGER_VERSION, ());

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<ZwpTextInputManagerV3, (), D> for TextInputManagerState
where
    D: GlobalDispatch<ZwpTextInputManagerV3, ()>,
    D: Dispatch<ZwpTextInputManagerV3, ()>,
    D: Dispatch<ZwpTextInputV3, TextInputUserData>,
    D: 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<ZwpTextInputManagerV3>,
        _: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<ZwpTextInputManagerV3, (), D> for TextInputManagerState
where
    D: Dispatch<ZwpTextInputManagerV3, ()>,
    D: Dispatch<ZwpTextInputV3, TextInputUserData>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &ZwpTextInputManagerV3,
        request: zwp_text_input_manager_v3::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwp_text_input_manager_v3::Request::GetTextInput { id, seat } => {
                let seat = Seat::<D>::from_resource(&seat).unwrap();

                let user_data = seat.user_data();
                user_data.insert_if_missing(TextInputHandle::default);
                user_data.insert_if_missing(InputMethodHandle::default);
                let handle = user_data.get::<TextInputHandle>().unwrap();
                let input_method_handle = user_data.get::<InputMethodHandle>().unwrap();
                let instance = data_init.init(
                    id,
                    TextInputUserData {
                        handle: handle.clone(),
                        input_method_handle: input_method_handle.clone(),
                    },
                );
                handle.add_instance(&instance);
                if input_method_handle.has_instance() {
                    handle.enter();
                }
            }
            zwp_text_input_manager_v3::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_text_input_manager {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::text_input::zv3::server::zwp_text_input_manager_v3::ZwpTextInputManagerV3: ()
        ] => $crate::wayland::text_input::TextInputManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::text_input::zv3::server::zwp_text_input_manager_v3::ZwpTextInputManagerV3: ()
        ] => $crate::wayland::text_input::TextInputManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::text_input::zv3::server::zwp_text_input_v3::ZwpTextInputV3:
            $crate::wayland::text_input::TextInputUserData
        ] => $crate::wayland::text_input::TextInputManagerState);
    };
}
