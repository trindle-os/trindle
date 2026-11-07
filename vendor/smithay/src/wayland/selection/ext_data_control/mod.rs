//! Automatic handling of the `ext_data_control` protocol.

use wayland_protocols::ext::data_control::v1::server::ext_data_control_manager_v1::ExtDataControlManagerV1;
use wayland_server::backend::GlobalId;
use wayland_server::{Client, DisplayHandle, GlobalDispatch};

mod device;
mod source;

pub use device::ExtDataControlDeviceUserData;
pub use source::ExtDataControlSourceUserData;

use super::primary_selection::PrimarySelectionState;
use super::SelectionHandler;

pub trait DataControlHandler: Sized + SelectionHandler {
    fn data_control_state(&self) -> &DataControlState;
}

#[derive(Debug)]
pub struct DataControlState {
    manager_global: GlobalId,
}

impl DataControlState {
    pub fn new<D, F>(
        display: &DisplayHandle,
        primary_selection: Option<&PrimarySelectionState>,
        filter: F,
    ) -> Self
    where
        D: GlobalDispatch<ExtDataControlManagerV1, ExtDataControlManagerGlobalData> + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = ExtDataControlManagerGlobalData {
            primary: primary_selection.is_some(),
            filter: Box::new(filter),
        };
        let manager_global = display.create_global::<D, ExtDataControlManagerV1, _>(1, data);
        Self { manager_global }
    }

    pub fn global(&self) -> GlobalId {
        self.manager_global.clone()
    }
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct ExtDataControlManagerGlobalData {
    primary: bool,

    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ExtDataControlManagerUserData {
    primary: bool,
}

mod handlers {
    use std::cell::RefCell;

    use tracing::error;
    use wayland_protocols::ext::data_control::v1::server::{
        ext_data_control_device_v1::ExtDataControlDeviceV1,
        ext_data_control_manager_v1::{self, ExtDataControlManagerV1},
        ext_data_control_source_v1::ExtDataControlSourceV1,
    };
    use wayland_server::{Client, Dispatch, DisplayHandle, GlobalDispatch};

    use crate::input::Seat;
    use crate::wayland::selection::device::SelectionDevice;
    use crate::wayland::selection::seat_data::SeatData;
    use crate::wayland::selection::SelectionTarget;

    use super::DataControlHandler;
    use super::DataControlState;
    use super::ExtDataControlDeviceUserData;
    use super::ExtDataControlManagerGlobalData;
    use super::ExtDataControlManagerUserData;
    use super::ExtDataControlSourceUserData;

    impl<D> GlobalDispatch<ExtDataControlManagerV1, ExtDataControlManagerGlobalData, D> for DataControlState
    where
        D: GlobalDispatch<ExtDataControlManagerV1, ExtDataControlManagerGlobalData>,
        D: Dispatch<ExtDataControlManagerV1, ExtDataControlManagerUserData>,
        D: Dispatch<ExtDataControlDeviceV1, ExtDataControlDeviceUserData>,
        D: Dispatch<ExtDataControlSourceV1, ExtDataControlSourceUserData>,
        D: DataControlHandler,
        D: 'static,
    {
        fn bind(
            _state: &mut D,
            _handle: &DisplayHandle,
            _client: &wayland_server::Client,
            resource: wayland_server::New<ExtDataControlManagerV1>,
            global_data: &ExtDataControlManagerGlobalData,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            data_init.init(
                resource,
                ExtDataControlManagerUserData {
                    primary: global_data.primary,
                },
            );
        }

        fn can_view(client: Client, global_data: &ExtDataControlManagerGlobalData) -> bool {
            (global_data.filter)(&client)
        }
    }

    impl<D> Dispatch<ExtDataControlManagerV1, ExtDataControlManagerUserData, D> for DataControlState
    where
        D: Dispatch<ExtDataControlManagerV1, ExtDataControlManagerUserData>,
        D: Dispatch<ExtDataControlDeviceV1, ExtDataControlDeviceUserData>,
        D: Dispatch<ExtDataControlSourceV1, ExtDataControlSourceUserData>,
        D: DataControlHandler,
        D: 'static,
    {
        fn request(
            _handler: &mut D,
            client: &wayland_server::Client,
            _resource: &ExtDataControlManagerV1,
            request: <ExtDataControlManagerV1 as wayland_server::Resource>::Request,
            data: &ExtDataControlManagerUserData,
            dh: &DisplayHandle,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            match request {
                ext_data_control_manager_v1::Request::CreateDataSource { id } => {
                    data_init.init(id, ExtDataControlSourceUserData::new());
                }
                ext_data_control_manager_v1::Request::GetDataDevice { id, seat: wl_seat } => {
                    match Seat::<D>::from_resource(&wl_seat) {
                        Some(seat) => {
                            seat.user_data()
                                .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));

                            let device = SelectionDevice::ExtDataControl(data_init.init(
                                id,
                                ExtDataControlDeviceUserData {
                                    wl_seat,
                                    primary: data.primary,
                                },
                            ));

                            let mut seat_data = seat
                                .user_data()
                                .get::<RefCell<SeatData<D::SelectionUserData>>>()
                                .unwrap()
                                .borrow_mut();

                            seat_data.add_device(device.clone());

                            let device = Some(&device);
                            seat_data.send_selection::<D>(dh, SelectionTarget::Clipboard, device, true);
                            if data.primary {
                                seat_data.send_selection::<D>(dh, SelectionTarget::Primary, device, true);
                            }
                        }
                        None => {
                            error!(
                                data_control_device = ?id,
                                client = ?client,
                                "Unmanaged seat given to a primary selection device."
                            );
                        }
                    }
                }
                ext_data_control_manager_v1::Request::Destroy => (),
                _ => unreachable!(),
            }
        }
    }
}

#[macro_export]
macro_rules! delegate_ext_data_control {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::data_control::v1::server::ext_data_control_manager_v1::ExtDataControlManagerV1: $crate::wayland::selection::ext_data_control::ExtDataControlManagerGlobalData
        ] => $crate::wayland::selection::ext_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::data_control::v1::server::ext_data_control_manager_v1::ExtDataControlManagerV1: $crate::wayland::selection::ext_data_control::ExtDataControlManagerUserData
        ] => $crate::wayland::selection::ext_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::data_control::v1::server::ext_data_control_device_v1::ExtDataControlDeviceV1: $crate::wayland::selection::ext_data_control::ExtDataControlDeviceUserData
        ] => $crate::wayland::selection::ext_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::data_control::v1::server::ext_data_control_source_v1::ExtDataControlSourceV1: $crate::wayland::selection::ext_data_control::ExtDataControlSourceUserData
        ] => $crate::wayland::selection::ext_data_control::DataControlState);
    };
}
