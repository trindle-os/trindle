//! Utilities for manipulating the primary selection.

use std::{
    cell::{Ref, RefCell},
    os::unix::io::OwnedFd,
    sync::Arc,
};

use tracing::instrument;
use wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1 as PrimaryDeviceManager;
use wayland_server::{backend::GlobalId, Client, DisplayHandle, GlobalDispatch};

use crate::{
    input::{Seat, SeatHandler},
    wayland::selection::SelectionTarget,
};

mod device;
mod source;

pub use device::PrimaryDeviceUserData;
pub use source::{PrimarySourceUserData, SourceMetadata};

use super::source::CompositorSelectionProvider;
use super::SelectionHandler;
use super::{offer::OfferReplySource, seat_data::SeatData};

pub trait PrimarySelectionHandler: Sized + SeatHandler + SelectionHandler {
    fn primary_selection_state(&self) -> &PrimarySelectionState;
}

pub struct PrimarySelectionState {
    manager_global: GlobalId,
    pub(super) filter: Arc<Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>>,
}

impl std::fmt::Debug for PrimarySelectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimarySelectionState")
            .field("manager_global", &self.manager_global)
            .finish()
    }
}

impl PrimarySelectionState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<PrimaryDeviceManager, PrimaryDeviceManagerGlobalData> + 'static,
        D: PrimarySelectionHandler,
    {
        Self::new_with_filter::<D, _>(display, |_| true)
    }

    pub fn new_with_filter<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<PrimaryDeviceManager, PrimaryDeviceManagerGlobalData> + 'static,
        D: PrimarySelectionHandler,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = PrimaryDeviceManagerGlobalData {
            filter: Arc::new(Box::new(filter)),
        };
        let filter = Arc::clone(&data.filter);
        let manager_global = display.create_global::<D, PrimaryDeviceManager, _>(1, data);

        Self {
            manager_global,
            filter,
        }
    }

    pub fn global(&self) -> GlobalId {
        self.manager_global.clone()
    }
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct PrimaryDeviceManagerGlobalData {
    filter: Arc<Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>>,
}

#[instrument(name = "wayland_primary_selection", level = "debug", skip(dh, seat, client), fields(seat = seat.name(), client = ?client.as_ref().map(|c| c.id())))]
pub fn set_primary_focus<D>(dh: &DisplayHandle, seat: &Seat<D>, client: Option<Client>)
where
    D: SeatHandler + PrimarySelectionHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    seat_data.borrow_mut().set_primary_focus::<D>(dh, client);
}

#[instrument(name = "wayland_primary_selection", level = "debug", skip(dh, seat, user_data), fields(seat = seat.name()))]
pub fn set_primary_selection<D>(
    dh: &DisplayHandle,
    seat: &Seat<D>,
    mime_types: Vec<String>,
    user_data: D::SelectionUserData,
) where
    D: SeatHandler + PrimarySelectionHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();

    let selection = OfferReplySource::Compositor(CompositorSelectionProvider {
        ty: SelectionTarget::Primary,
        mime_types,
        user_data,
    });

    seat_data
        .borrow_mut()
        .set_primary_selection::<D>(dh, Some(selection));
}

#[derive(Debug, thiserror::Error)]
pub enum SelectionRequestError {
    #[error("Requested mime type is not available")]
    InvalidMimetype,
    #[error("Current selection is server-side")]
    ServerSideSelection,
    #[error("No active selection to query")]
    NoSelection,
}

pub fn request_primary_client_selection<D>(
    seat: &Seat<D>,
    mime_type: String,
    fd: OwnedFd,
) -> Result<(), SelectionRequestError>
where
    D: SeatHandler + PrimarySelectionHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    match seat_data.borrow().get_primary_selection() {
        None => Err(SelectionRequestError::NoSelection),
        Some(OfferReplySource::Client(source)) => {
            if !source.contains_mime_type(&mime_type) {
                Err(SelectionRequestError::InvalidMimetype)
            } else {
                source.send(mime_type, fd);
                Ok(())
            }
        }
        Some(OfferReplySource::Compositor(selection)) => {
            if !selection.mime_types.contains(&mime_type) {
                Err(SelectionRequestError::InvalidMimetype)
            } else {
                Err(SelectionRequestError::ServerSideSelection)
            }
        }
    }
}

#[instrument(name = "wayland_primary_selection", level = "debug", skip_all, fields(seat = seat.name()))]
pub fn current_primary_selection_userdata<D>(seat: &Seat<D>) -> Option<Ref<'_, D::SelectionUserData>>
where
    D: SeatHandler + PrimarySelectionHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    Ref::filter_map(seat_data.borrow(), |data| match data.get_primary_selection() {
        Some(OfferReplySource::Compositor(CompositorSelectionProvider { ref user_data, .. })) => {
            Some(user_data)
        }
        _ => None,
    })
    .ok()
}

#[instrument(name = "wayland_primary_selection", level = "debug", skip_all, fields(seat = seat.name()))]
pub fn clear_primary_selection<D>(dh: &DisplayHandle, seat: &Seat<D>)
where
    D: SeatHandler + PrimarySelectionHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    seat_data.borrow_mut().set_primary_selection::<D>(dh, None);
}

mod handlers {
    use std::cell::RefCell;

    use tracing::error;
    use wayland_protocols::wp::primary_selection::zv1::server::{
        zwp_primary_selection_device_manager_v1::{
            self as primary_device_manager, ZwpPrimarySelectionDeviceManagerV1 as PrimaryDeviceManager,
        },
        zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1 as PrimaryDevice,
        zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1 as PrimarySource,
    };
    use wayland_server::{Dispatch, DisplayHandle, GlobalDispatch};

    use crate::{
        input::{Seat, SeatHandler},
        wayland::selection::{device::SelectionDevice, seat_data::SeatData},
    };

    use super::{
        device::PrimaryDeviceUserData, source::PrimarySourceUserData, PrimaryDeviceManagerGlobalData,
    };
    use super::{PrimarySelectionHandler, PrimarySelectionState};

    impl<D> GlobalDispatch<PrimaryDeviceManager, PrimaryDeviceManagerGlobalData, D> for PrimarySelectionState
    where
        D: GlobalDispatch<PrimaryDeviceManager, PrimaryDeviceManagerGlobalData>,
        D: Dispatch<PrimaryDeviceManager, ()>,
        D: Dispatch<PrimarySource, PrimarySourceUserData>,
        D: Dispatch<PrimaryDevice, PrimaryDeviceUserData>,
        D: PrimarySelectionHandler,
        D: 'static,
    {
        fn bind(
            _state: &mut D,
            _handle: &DisplayHandle,
            _client: &wayland_server::Client,
            resource: wayland_server::New<PrimaryDeviceManager>,
            _global_data: &PrimaryDeviceManagerGlobalData,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            data_init.init(resource, ());
        }

        fn can_view(client: wayland_server::Client, global_data: &PrimaryDeviceManagerGlobalData) -> bool {
            (global_data.filter)(&client)
        }
    }

    impl<D> Dispatch<PrimaryDeviceManager, (), D> for PrimarySelectionState
    where
        D: Dispatch<PrimaryDeviceManager, ()>,
        D: Dispatch<PrimarySource, PrimarySourceUserData>,
        D: Dispatch<PrimaryDevice, PrimaryDeviceUserData>,
        D: PrimarySelectionHandler,
        D: SeatHandler,
        D: 'static,
    {
        fn request(
            _state: &mut D,
            client: &wayland_server::Client,
            _resource: &PrimaryDeviceManager,
            request: primary_device_manager::Request,
            _data: &(),
            _dhandle: &DisplayHandle,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            match request {
                primary_device_manager::Request::CreateSource { id } => {
                    data_init.init(id, PrimarySourceUserData::new());
                }
                primary_device_manager::Request::GetDevice { id, seat: wl_seat } => {
                    match Seat::<D>::from_resource(&wl_seat) {
                        Some(seat) => {
                            seat.user_data()
                                .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));

                            let device = SelectionDevice::Primary(
                                data_init.init(id, PrimaryDeviceUserData { wl_seat }),
                            );

                            let seat_data = seat
                                .user_data()
                                .get::<RefCell<SeatData<D::SelectionUserData>>>()
                                .unwrap();

                            seat_data.borrow_mut().add_device(device);
                        }
                        None => {
                            error!(
                                primary_selection_device = ?id,
                                client = ?client,
                                "Unmanaged seat given to a primary selection device."
                            );
                        }
                    }
                }
                primary_device_manager::Request::Destroy => {}
                _ => unreachable!(),
            }
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_primary_selection {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1: $crate::wayland::selection::primary_selection::PrimaryDeviceManagerGlobalData
        ] => $crate::wayland::selection::primary_selection::PrimarySelectionState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1: ()
        ] => $crate::wayland::selection::primary_selection::PrimarySelectionState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1: $crate::wayland::selection::primary_selection::PrimaryDeviceUserData
        ] => $crate::wayland::selection::primary_selection::PrimarySelectionState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1: $crate::wayland::selection::primary_selection::PrimarySourceUserData
        ] => $crate::wayland::selection::primary_selection::PrimarySelectionState);
    };
}
