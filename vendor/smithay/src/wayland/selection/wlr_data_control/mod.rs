//! Automatic handling of the `wlr_data_control` protocol.

use std::sync::Arc;

use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1;
use wayland_server::backend::GlobalId;
use wayland_server::{Client, DisplayHandle, GlobalDispatch};

mod device;
mod source;

pub use device::DataControlDeviceUserData;
pub use source::DataControlSourceUserData;

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
        D: GlobalDispatch<ZwlrDataControlManagerV1, DataControlManagerGlobalData> + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = DataControlManagerGlobalData {
            primary_selection_filter: primary_selection
                .map(|x| Arc::clone(&x.filter))
                .unwrap_or_else(|| Arc::new(Box::new(|_| false))),
            filter: Box::new(filter),
        };
        let manager_global = display.create_global::<D, ZwlrDataControlManagerV1, _>(2, data);
        Self { manager_global }
    }

    pub fn global(&self) -> GlobalId {
        self.manager_global.clone()
    }
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct DataControlManagerGlobalData {
    primary_selection_filter: Arc<Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>>,

    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
#[derive(Clone)]
pub struct DataControlManagerUserData {
    primary_selection_filter: Arc<Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>>,
}

mod handlers {
    use std::cell::RefCell;
    use std::sync::Arc;

    use tracing::error;
    use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1;
    use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_manager_v1;
    use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1;
    use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_source_v1::ZwlrDataControlSourceV1;
    use wayland_server::{Client, Dispatch, DisplayHandle, GlobalDispatch};

    use crate::input::Seat;
    use crate::wayland::selection::device::SelectionDevice;
    use crate::wayland::selection::seat_data::SeatData;
    use crate::wayland::selection::SelectionTarget;

    use super::DataControlDeviceUserData;
    use super::DataControlHandler;
    use super::DataControlManagerGlobalData;
    use super::DataControlManagerUserData;
    use super::DataControlSourceUserData;
    use super::DataControlState;

    impl<D> GlobalDispatch<ZwlrDataControlManagerV1, DataControlManagerGlobalData, D> for DataControlState
    where
        D: GlobalDispatch<ZwlrDataControlManagerV1, DataControlManagerGlobalData>,
        D: Dispatch<ZwlrDataControlManagerV1, DataControlManagerUserData>,
        D: Dispatch<ZwlrDataControlDeviceV1, DataControlDeviceUserData>,
        D: Dispatch<ZwlrDataControlSourceV1, DataControlSourceUserData>,
        D: DataControlHandler,
        D: 'static,
    {
        fn bind(
            _state: &mut D,
            _handle: &DisplayHandle,
            _client: &wayland_server::Client,
            resource: wayland_server::New<ZwlrDataControlManagerV1>,
            global_data: &DataControlManagerGlobalData,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            data_init.init(
                resource,
                DataControlManagerUserData {
                    primary_selection_filter: Arc::clone(&global_data.primary_selection_filter),
                },
            );
        }

        fn can_view(client: Client, global_data: &DataControlManagerGlobalData) -> bool {
            (global_data.filter)(&client)
        }
    }

    impl<D> Dispatch<ZwlrDataControlManagerV1, DataControlManagerUserData, D> for DataControlState
    where
        D: Dispatch<ZwlrDataControlManagerV1, DataControlManagerUserData>,
        D: Dispatch<ZwlrDataControlDeviceV1, DataControlDeviceUserData>,
        D: Dispatch<ZwlrDataControlSourceV1, DataControlSourceUserData>,
        D: DataControlHandler,
        D: 'static,
    {
        fn request(
            _handler: &mut D,
            client: &wayland_server::Client,
            _resource: &ZwlrDataControlManagerV1,
            request: <ZwlrDataControlManagerV1 as wayland_server::Resource>::Request,
            data: &DataControlManagerUserData,
            dh: &DisplayHandle,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            match request {
                zwlr_data_control_manager_v1::Request::CreateDataSource { id } => {
                    data_init.init(id, DataControlSourceUserData::new());
                }
                zwlr_data_control_manager_v1::Request::GetDataDevice { id, seat: wl_seat } => {
                    match Seat::<D>::from_resource(&wl_seat) {
                        Some(seat) => {
                            seat.user_data()
                                .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));

                            let device = SelectionDevice::WlrDataControl(data_init.init(
                                id,
                                DataControlDeviceUserData {
                                    wl_seat,
                                    primary_selection_filter: Arc::clone(&data.primary_selection_filter),
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
                            if (*data.primary_selection_filter)(client) {
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
                zwlr_data_control_manager_v1::Request::Destroy => (),
                _ => unreachable!(),
            }
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_data_control {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1: $crate::wayland::selection::wlr_data_control::DataControlManagerGlobalData
        ] => $crate::wayland::selection::wlr_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1: $crate::wayland::selection::wlr_data_control::DataControlManagerUserData
        ] => $crate::wayland::selection::wlr_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1: $crate::wayland::selection::wlr_data_control::DataControlDeviceUserData
        ] => $crate::wayland::selection::wlr_data_control::DataControlState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_source_v1::ZwlrDataControlSourceV1: $crate::wayland::selection::wlr_data_control::DataControlSourceUserData
        ] => $crate::wayland::selection::wlr_data_control::DataControlState);
    };
}
