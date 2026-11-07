//! Utilities for manipulating the data devices.

use std::{
    cell::{Ref, RefCell},
    os::unix::io::OwnedFd,
};

use tracing::instrument;
use wayland_server::{
    backend::GlobalId,
    protocol::{
        wl_data_device_manager::{DndAction, WlDataDeviceManager},
        wl_data_source::WlDataSource,
        wl_surface::WlSurface,
    },
    Client, DisplayHandle, GlobalDispatch,
};

use crate::{
    input::{
        pointer::{Focus, GrabStartData as PointerGrabStartData},
        touch::GrabStartData as TouchGrabStartData,
        Seat, SeatHandler,
    },
    utils::{Logical, Point, Serial},
    wayland::seat::WaylandFocus,
};

mod device;
mod dnd_grab;
mod server_dnd_grab;
mod source;

pub use device::{DataDeviceUserData, DND_ICON_ROLE};
pub use dnd_grab::DnDGrab;
pub use server_dnd_grab::ServerDnDGrab;
pub use source::{with_source_metadata, DataSourceUserData, SourceMetadata};

use super::{
    offer::OfferReplySource, seat_data::SeatData, source::CompositorSelectionProvider, SelectionHandler,
    SelectionTarget,
};

#[allow(unused_variables)]
pub trait DataDeviceHandler: Sized + SelectionHandler + ClientDndGrabHandler + ServerDndGrabHandler {
    fn data_device_state(&self) -> &DataDeviceState;

    fn action_choice(&mut self, available: DndAction, preferred: DndAction) -> DndAction {
        default_action_chooser(available, preferred)
    }

    fn foreign_target(&mut self, surface: &WlSurface) -> bool {
        false
    }

    fn foreign_enter(&mut self, surface: &WlSurface, location: Point<f64, Logical>) {}

    fn foreign_motion(&mut self, location: Point<f64, Logical>, time: u32) {}

    fn foreign_leave(&mut self) {}

    fn foreign_drop(&mut self) -> bool {
        false
    }
}

#[allow(unused_variables)]
pub trait ClientDndGrabHandler: SeatHandler + Sized {
    fn started(&mut self, source: Option<WlDataSource>, icon: Option<WlSurface>, seat: Seat<Self>) {}

    fn dropped(&mut self, target: Option<WlSurface>, validated: bool, seat: Seat<Self>) {}
}

#[allow(unused_variables)]
pub trait ServerDndGrabHandler: SeatHandler {
    fn accept(&mut self, mime_type: Option<String>, seat: Seat<Self>) {}

    fn action(&mut self, action: DndAction, seat: Seat<Self>) {}

    fn dropped(&mut self, seat: Seat<Self>) {}

    fn cancelled(&mut self, seat: Seat<Self>) {}

    fn send(&mut self, mime_type: String, fd: OwnedFd, seat: Seat<Self>) {}

    fn finished(&mut self, seat: Seat<Self>) {}

    fn drop_allowed(&mut self, seat: Seat<Self>) -> bool {
        true
    }

    fn released(&mut self, seat: Seat<Self>) {}
}

#[derive(Debug)]
pub struct DataDeviceState {
    manager_global: GlobalId,
}

impl DataDeviceState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WlDataDeviceManager, ()> + 'static,
        D: DataDeviceHandler,
    {
        let manager_global = display.create_global::<D, WlDataDeviceManager, _>(3, ());

        Self { manager_global }
    }

    pub fn global(&self) -> GlobalId {
        self.manager_global.clone()
    }
}

pub fn default_action_chooser(available: DndAction, preferred: DndAction) -> DndAction {
    if [DndAction::Move, DndAction::Copy, DndAction::Ask].contains(&preferred)
        && available.contains(preferred)
    {
        preferred
    } else if available.contains(DndAction::Ask) {
        DndAction::Ask
    } else if available.contains(DndAction::Copy) {
        DndAction::Copy
    } else if available.contains(DndAction::Move) {
        DndAction::Move
    } else {
        DndAction::empty()
    }
}

#[instrument(name = "wayland_data_device", level = "debug", skip(dh, seat, client), fields(seat = seat.name(), client = ?client.as_ref().map(|c| c.id())))]
pub fn set_data_device_focus<D>(dh: &DisplayHandle, seat: &Seat<D>, client: Option<Client>)
where
    D: SeatHandler + DataDeviceHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    seat_data.borrow_mut().set_clipboard_focus::<D>(dh, client);
}

#[instrument(name = "wayland_data_device", level = "debug", skip(dh, seat, user_data), fields(seat = seat.name()))]
pub fn set_data_device_selection<D>(
    dh: &DisplayHandle,
    seat: &Seat<D>,
    mime_types: Vec<String>,
    user_data: D::SelectionUserData,
) where
    D: SeatHandler + DataDeviceHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));

    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();

    let selection = OfferReplySource::Compositor(CompositorSelectionProvider {
        ty: SelectionTarget::Clipboard,
        mime_types,
        user_data,
    });

    seat_data
        .borrow_mut()
        .set_clipboard_selection::<D>(dh, Some(selection));
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

pub fn request_data_device_client_selection<D>(
    seat: &Seat<D>,
    mime_type: String,
    fd: OwnedFd,
) -> Result<(), SelectionRequestError>
where
    D: SeatHandler + DataDeviceHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    match seat_data.borrow().get_clipboard_selection() {
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

#[instrument(name = "wayland_data_device", level = "debug", skip_all, fields(seat = seat.name()))]
pub fn current_data_device_selection_userdata<D>(seat: &Seat<D>) -> Option<Ref<'_, D::SelectionUserData>>
where
    D: SeatHandler + DataDeviceHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    Ref::filter_map(seat_data.borrow(), |data| match data.get_clipboard_selection() {
        Some(OfferReplySource::Compositor(CompositorSelectionProvider { ref user_data, .. })) => {
            Some(user_data)
        }
        _ => None,
    })
    .ok()
}

#[instrument(name = "wayland_data_device", level = "debug", skip_all, fields(seat = seat.name()))]
pub fn clear_data_device_selection<D>(dh: &DisplayHandle, seat: &Seat<D>)
where
    D: SeatHandler + DataDeviceHandler + 'static,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    let seat_data = seat
        .user_data()
        .get::<RefCell<SeatData<D::SelectionUserData>>>()
        .unwrap();
    seat_data.borrow_mut().set_clipboard_selection::<D>(dh, None);
}

#[instrument(name = "wayland_data_device", level = "debug", skip(dh, seat, data), fields(seat = seat.name()))]
pub fn start_dnd<D>(
    dh: &DisplayHandle,
    seat: &Seat<D>,
    data: &mut D,
    serial: Serial,
    pointer_start_data: Option<PointerGrabStartData<D>>,
    touch_start_data: Option<TouchGrabStartData<D>>,
    metadata: SourceMetadata,
) where
    D: SeatHandler + DataDeviceHandler + 'static,
    <D as SeatHandler>::PointerFocus: WaylandFocus,
    <D as SeatHandler>::TouchFocus: WaylandFocus,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    if let (Some(pointer_start_data), Some(pointer)) = (pointer_start_data, seat.get_pointer()) {
        pointer.set_grab(
            data,
            server_dnd_grab::ServerDnDGrab::new_pointer(dh, pointer_start_data, metadata, seat.clone()),
            serial,
            Focus::Keep,
        );
    } else if let (Some(touch_start_data), Some(touch)) = (touch_start_data, seat.get_touch()) {
        touch.set_grab(
            data,
            server_dnd_grab::ServerDnDGrab::new_touch(dh, touch_start_data, metadata, seat.clone()),
            serial,
        );
    }
}

#[instrument(name = "wayland_data_device", level = "debug", skip(dh, seat, data), fields(seat = seat.name()))]
pub fn start_dnd_following<D>(
    dh: &DisplayHandle,
    seat: &Seat<D>,
    data: &mut D,
    serial: Serial,
    pointer_start_data: PointerGrabStartData<D>,
    metadata: SourceMetadata,
) where
    D: SeatHandler + DataDeviceHandler + 'static,
    <D as SeatHandler>::PointerFocus: WaylandFocus,
{
    seat.user_data()
        .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));
    if let Some(pointer) = seat.get_pointer() {
        let mut grab = server_dnd_grab::ServerDnDGrab::new_pointer(dh, pointer_start_data, metadata, seat.clone());
        grab.follow();
        pointer.set_grab(data, grab, serial, Focus::Keep);
    }
}

mod handlers {
    use std::cell::RefCell;

    use tracing::error;
    use wayland_server::{
        protocol::{
            wl_data_device::WlDataDevice,
            wl_data_device_manager::{self, WlDataDeviceManager},
            wl_data_source::WlDataSource,
        },
        Dispatch, DisplayHandle, GlobalDispatch,
    };

    use crate::{
        input::Seat,
        wayland::selection::{device::SelectionDevice, seat_data::SeatData},
    };

    use super::{device::DataDeviceUserData, source::DataSourceUserData};
    use super::{DataDeviceHandler, DataDeviceState};

    impl<D> GlobalDispatch<WlDataDeviceManager, (), D> for DataDeviceState
    where
        D: GlobalDispatch<WlDataDeviceManager, ()>,
        D: Dispatch<WlDataDeviceManager, ()>,
        D: Dispatch<WlDataSource, DataSourceUserData>,
        D: Dispatch<WlDataDevice, DataDeviceUserData>,
        D: DataDeviceHandler,
        D: 'static,
    {
        fn bind(
            _state: &mut D,
            _handle: &DisplayHandle,
            _client: &wayland_server::Client,
            resource: wayland_server::New<WlDataDeviceManager>,
            _global_data: &(),
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            data_init.init(resource, ());
        }
    }

    impl<D> Dispatch<WlDataDeviceManager, (), D> for DataDeviceState
    where
        D: Dispatch<WlDataDeviceManager, ()>,
        D: Dispatch<WlDataSource, DataSourceUserData>,
        D: Dispatch<WlDataDevice, DataDeviceUserData>,
        D: DataDeviceHandler,
        D: 'static,
    {
        fn request(
            _state: &mut D,
            client: &wayland_server::Client,
            _resource: &WlDataDeviceManager,
            request: wl_data_device_manager::Request,
            _data: &(),
            _dhandle: &DisplayHandle,
            data_init: &mut wayland_server::DataInit<'_, D>,
        ) {
            match request {
                wl_data_device_manager::Request::CreateDataSource { id } => {
                    data_init.init(id, DataSourceUserData::new());
                }
                wl_data_device_manager::Request::GetDataDevice { id, seat: wl_seat } => {
                    match Seat::<D>::from_resource(&wl_seat) {
                        Some(seat) => {
                            seat.user_data()
                                .insert_if_missing(|| RefCell::new(SeatData::<D::SelectionUserData>::new()));

                            let device = SelectionDevice::DataDevice(
                                data_init.init(id, DataDeviceUserData { wl_seat }),
                            );

                            let seat_data = seat
                                .user_data()
                                .get::<RefCell<SeatData<D::SelectionUserData>>>()
                                .unwrap();
                            seat_data.borrow_mut().add_device(device);
                        }
                        None => {
                            error!(client = ?client, data_device = ?id, "Unmanaged seat given to a data device.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_data_device {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager: ()
        ] => $crate::wayland::selection::data_device::DataDeviceState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager: ()
        ] => $crate::wayland::selection::data_device::DataDeviceState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_data_device::WlDataDevice: $crate::wayland::selection::data_device::DataDeviceUserData
        ] => $crate::wayland::selection::data_device::DataDeviceState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_data_source::WlDataSource: $crate::wayland::selection::data_device::DataSourceUserData
        ] => $crate::wayland::selection::data_device::DataDeviceState);
    };
}
