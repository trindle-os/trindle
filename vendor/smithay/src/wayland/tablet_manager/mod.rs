//! Utilities for graphics tablet support.

use crate::input::{Seat, SeatHandler};
use wayland_protocols::wp::tablet::zv2::server::{
    zwp_tablet_manager_v2::{self, ZwpTabletManagerV2},
    zwp_tablet_seat_v2::ZwpTabletSeatV2,
    zwp_tablet_tool_v2::ZwpTabletToolV2,
    zwp_tablet_v2::ZwpTabletV2,
};
use wayland_server::{backend::GlobalId, Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};

const MANAGER_VERSION: u32 = 1;

mod tablet;
mod tablet_seat;
pub(crate) mod tablet_tool;

pub use tablet::{TabletDescriptor, TabletHandle, TabletUserData};
pub use tablet_seat::{TabletSeatHandle, TabletSeatHandler, TabletSeatUserData};
pub use tablet_tool::{TabletToolHandle, TabletToolUserData};

use super::compositor::CompositorHandler;

pub trait TabletSeatTrait {
    fn tablet_seat(&self) -> TabletSeatHandle;
}

impl<D: SeatHandler + 'static> TabletSeatTrait for Seat<D> {
    fn tablet_seat(&self) -> TabletSeatHandle {
        let user_data = self.user_data();
        user_data.insert_if_missing(TabletSeatHandle::default);
        user_data.get::<TabletSeatHandle>().unwrap().clone()
    }
}

#[derive(Debug)]
pub struct TabletManagerState {
    global: GlobalId,
}

impl TabletManagerState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<ZwpTabletManagerV2, ()>,
        D: Dispatch<ZwpTabletManagerV2, ()>,
        D: Dispatch<ZwpTabletSeatV2, TabletSeatUserData>,
        D: Dispatch<ZwpTabletToolV2, TabletToolUserData>,
        D: 'static,
    {
        let global = display.create_global::<D, ZwpTabletManagerV2, _>(MANAGER_VERSION, ());

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<ZwpTabletManagerV2, (), D> for TabletManagerState
where
    D: GlobalDispatch<ZwpTabletManagerV2, ()>,
    D: Dispatch<ZwpTabletManagerV2, ()>,
    D: Dispatch<ZwpTabletSeatV2, TabletSeatUserData>,
    D: 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<ZwpTabletManagerV2>,
        _: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<ZwpTabletManagerV2, (), D> for TabletManagerState
where
    D: Dispatch<ZwpTabletManagerV2, ()>,
    D: Dispatch<ZwpTabletSeatV2, TabletSeatUserData>,
    D: Dispatch<ZwpTabletV2, TabletUserData>,
    D: Dispatch<ZwpTabletToolV2, TabletToolUserData>,
    D: SeatHandler + TabletSeatHandler + 'static,
    D: CompositorHandler,
{
    fn request(
        state: &mut D,
        client: &Client,
        _: &ZwpTabletManagerV2,
        request: zwp_tablet_manager_v2::Request,
        _: &(),
        dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwp_tablet_manager_v2::Request::GetTabletSeat { tablet_seat, seat } => {
                let seat = Seat::<D>::from_resource(&seat).unwrap();

                let user_data = seat.user_data();
                user_data.insert_if_missing(TabletSeatHandle::default);

                let handle = user_data.get::<TabletSeatHandle>().unwrap();
                let instance = data_init.init(
                    tablet_seat,
                    TabletSeatUserData {
                        handle: handle.clone(),
                    },
                );

                handle.add_instance::<D>(state, dh, &instance, client);
            }
            zwp_tablet_manager_v2::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_tablet_manager {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::tablet::zv2::server::zwp_tablet_manager_v2::ZwpTabletManagerV2: ()
        ] => $crate::wayland::tablet_manager::TabletManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::tablet::zv2::server::zwp_tablet_manager_v2::ZwpTabletManagerV2: ()
        ] => $crate::wayland::tablet_manager::TabletManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::tablet::zv2::server::zwp_tablet_seat_v2::ZwpTabletSeatV2: $crate::wayland::tablet_manager::TabletSeatUserData
        ] => $crate::wayland::tablet_manager::TabletManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::tablet::zv2::server::zwp_tablet_tool_v2::ZwpTabletToolV2: $crate::wayland::tablet_manager::TabletToolUserData
        ] => $crate::wayland::tablet_manager::TabletManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::tablet::zv2::server::zwp_tablet_v2::ZwpTabletV2: $crate::wayland::tablet_manager::TabletUserData
        ] => $crate::wayland::tablet_manager::TabletManagerState);
    };
}
