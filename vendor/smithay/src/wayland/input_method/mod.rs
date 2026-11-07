//! Utilities for input method support.

use wayland_server::{
    backend::GlobalId, protocol::wl_surface::WlSurface, Client, DataInit, Dispatch, DisplayHandle,
    GlobalDispatch, New,
};

use wayland_protocols_misc::zwp_input_method_v2::server::{
    zwp_input_method_manager_v2::{self, ZwpInputMethodManagerV2},
    zwp_input_method_v2::ZwpInputMethodV2,
};

use crate::{
    input::{Seat, SeatHandler},
    utils::{Logical, Rectangle},
};

pub use input_method_handle::{InputMethodHandle, InputMethodUserData};
pub use input_method_keyboard_grab::{InputMethodKeyboardGrab, InputMethodKeyboardUserData};
pub use input_method_popup_surface::InputMethodPopupSurfaceUserData;

use super::text_input::TextInputHandle;

const MANAGER_VERSION: u32 = 1;

pub const INPUT_POPUP_SURFACE_ROLE: &str = "zwp_input_popup_surface_v2";

mod input_method_handle;
mod input_method_keyboard_grab;
mod input_method_popup_surface;
pub use input_method_popup_surface::{PopupParent, PopupSurface};

pub trait InputMethodHandler {
    fn new_popup(&mut self, surface: PopupSurface);

    fn dismiss_popup(&mut self, surface: PopupSurface);

    fn popup_repositioned(&mut self, surface: PopupSurface);

    fn parent_geometry(&self, parent: &WlSurface) -> Rectangle<i32, Logical>;
}

pub trait InputMethodSeat {
    fn input_method(&self) -> &InputMethodHandle;
}

impl<D: SeatHandler + 'static> InputMethodSeat for Seat<D> {
    fn input_method(&self) -> &InputMethodHandle {
        let user_data = self.user_data();
        user_data.insert_if_missing(InputMethodHandle::default);
        user_data.get::<InputMethodHandle>().unwrap()
    }
}

#[allow(missing_debug_implementations)]
pub struct InputMethodManagerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Debug)]
pub struct InputMethodManagerState {
    global: GlobalId,
}

impl InputMethodManagerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<ZwpInputMethodManagerV2, InputMethodManagerGlobalData>,
        D: Dispatch<ZwpInputMethodManagerV2, ()>,
        D: Dispatch<ZwpInputMethodV2, InputMethodUserData<D>>,
        D: SeatHandler,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = InputMethodManagerGlobalData {
            filter: Box::new(filter),
        };
        let global = display.create_global::<D, ZwpInputMethodManagerV2, _>(MANAGER_VERSION, data);

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<ZwpInputMethodManagerV2, InputMethodManagerGlobalData, D> for InputMethodManagerState
where
    D: GlobalDispatch<ZwpInputMethodManagerV2, InputMethodManagerGlobalData>,
    D: Dispatch<ZwpInputMethodManagerV2, ()>,
    D: Dispatch<ZwpInputMethodV2, InputMethodUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<ZwpInputMethodManagerV2>,
        _: &InputMethodManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &InputMethodManagerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<ZwpInputMethodManagerV2, (), D> for InputMethodManagerState
where
    D: Dispatch<ZwpInputMethodManagerV2, ()>,
    D: Dispatch<ZwpInputMethodV2, InputMethodUserData<D>>,
    D: SeatHandler + InputMethodHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _: &ZwpInputMethodManagerV2,
        request: zwp_input_method_manager_v2::Request,
        _: &(),
        _dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwp_input_method_manager_v2::Request::GetInputMethod { seat, input_method } => {
                let seat = Seat::<D>::from_resource(&seat).unwrap();

                let user_data = seat.user_data();
                user_data.insert_if_missing(TextInputHandle::default);
                user_data.insert_if_missing(InputMethodHandle::default);
                let handle = user_data.get::<InputMethodHandle>().unwrap();
                let text_input_handle = user_data.get::<TextInputHandle>().unwrap();
                text_input_handle.with_focused_text_input(|ti, surface| {
                    ti.enter(surface);
                });
                let keyboard_handle = seat.get_keyboard().unwrap();
                let instance = data_init.init(
                    input_method,
                    InputMethodUserData {
                        handle: handle.clone(),
                        text_input_handle: text_input_handle.clone(),
                        keyboard_handle,
                        popup_geometry_callback: D::parent_geometry,
                        popup_repositioned: D::popup_repositioned,
                        new_popup: D::new_popup,
                        dismiss_popup: D::dismiss_popup,
                    },
                );
                handle.add_instance(&instance);
            }
            zwp_input_method_manager_v2::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_input_method_manager {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_method_manager_v2::ZwpInputMethodManagerV2:
            $crate::wayland::input_method::InputMethodManagerGlobalData
        ] => $crate::wayland::input_method::InputMethodManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_method_manager_v2::ZwpInputMethodManagerV2: ()
        ] => $crate::wayland::input_method::InputMethodManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_method_v2::ZwpInputMethodV2:
            $crate::wayland::input_method::InputMethodUserData<Self>
        ] => $crate::wayland::input_method::InputMethodManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_method_keyboard_grab_v2::ZwpInputMethodKeyboardGrabV2:
            $crate::wayland::input_method::InputMethodKeyboardUserData<Self>
        ] => $crate::wayland::input_method::InputMethodManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_popup_surface_v2::ZwpInputPopupSurfaceV2:
            $crate::wayland::input_method::InputMethodPopupSurfaceUserData
        ] => $crate::wayland::input_method::InputMethodManagerState);
    };
}
