//! Utilities for relative pointer support.

use std::sync::{atomic::Ordering, Arc, Mutex};

use atomic_float::AtomicF64;
use wayland_protocols::wp::relative_pointer::zv1::server::{
    zwp_relative_pointer_manager_v1::{self, ZwpRelativePointerManagerV1},
    zwp_relative_pointer_v1::{self, ZwpRelativePointerV1},
};
use wayland_server::{
    backend::{ClientId, GlobalId},
    protocol::wl_surface::WlSurface,
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

use crate::{
    input::{
        pointer::{PointerHandle, RelativeMotionEvent},
        SeatHandler,
    },
    wayland::seat::PointerUserData,
};

const MANAGER_VERSION: u32 = 1;

#[derive(Debug, Default)]
pub(crate) struct WpRelativePointerHandle {
    known_relative_pointers: Mutex<Vec<ZwpRelativePointerV1>>,
}

impl WpRelativePointerHandle {
    fn new_relative_pointer(&self, pointer: ZwpRelativePointerV1) {
        self.known_relative_pointers.lock().unwrap().push(pointer);
    }

    pub(super) fn relative_motion<D: SeatHandler + 'static>(
        &self,
        surface: &WlSurface,
        event: &RelativeMotionEvent,
    ) {
        self.for_each_focused_pointer(surface, |ptr| {
            let client_scale = ptr
                .data::<RelativePointerUserData<D>>()
                .unwrap()
                .client_scale
                .load(Ordering::Acquire);
            let delta = event.delta.to_client(client_scale);
            let delta_unaccel = event.delta_unaccel.to_client(client_scale);

            let utime_hi = (event.utime >> 32) as u32;
            let utime_lo = (event.utime & 0xffffffff) as u32;
            ptr.relative_motion(
                utime_hi,
                utime_lo,
                delta.x,
                delta.y,
                delta_unaccel.x,
                delta_unaccel.y,
            );
        })
    }

    fn for_each_focused_pointer(&self, surface: &WlSurface, mut f: impl FnMut(ZwpRelativePointerV1)) {
        let inner = self.known_relative_pointers.lock().unwrap();
        for ptr in &*inner {
            if ptr.id().same_client_as(&surface.id()) {
                f(ptr.clone())
            }
        }
    }
}

#[derive(Debug)]
pub struct RelativePointerUserData<D: SeatHandler> {
    handle: Option<PointerHandle<D>>,
    client_scale: Arc<AtomicF64>,
}

#[derive(Debug)]
pub struct RelativePointerManagerState {
    global: GlobalId,
}

impl RelativePointerManagerState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<ZwpRelativePointerManagerV1, ()>,
        D: Dispatch<ZwpRelativePointerManagerV1, ()>,
        D: Dispatch<ZwpRelativePointerV1, RelativePointerUserData<D>>,
        D: SeatHandler,
        D: 'static,
    {
        let global = display.create_global::<D, ZwpRelativePointerManagerV1, _>(MANAGER_VERSION, ());

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> Dispatch<ZwpRelativePointerManagerV1, (), D> for RelativePointerManagerState
where
    D: Dispatch<ZwpRelativePointerManagerV1, ()>,
    D: Dispatch<ZwpRelativePointerV1, RelativePointerUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _relative_pointer_manager: &ZwpRelativePointerManagerV1,
        request: zwp_relative_pointer_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            zwp_relative_pointer_manager_v1::Request::GetRelativePointer { id, pointer } => {
                let data = pointer.data::<PointerUserData<D>>().unwrap();
                let user_data = RelativePointerUserData {
                    handle: data.handle.clone(),
                    client_scale: data.client_scale.clone(),
                };
                let pointer = data_init.init(id, user_data);
                if let Some(handle) = &data.handle {
                    handle.wp_relative.new_relative_pointer(pointer);
                }
            }
            zwp_relative_pointer_manager_v1::Request::Destroy => {}
            _ => unreachable!(),
        }
    }
}

impl<D> GlobalDispatch<ZwpRelativePointerManagerV1, (), D> for RelativePointerManagerState
where
    D: GlobalDispatch<ZwpRelativePointerManagerV1, ()>
        + Dispatch<ZwpRelativePointerManagerV1, ()>
        + SeatHandler
        + 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<ZwpRelativePointerManagerV1>,
        _global_data: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<ZwpRelativePointerV1, RelativePointerUserData<D>, D> for RelativePointerManagerState
where
    D: Dispatch<ZwpRelativePointerV1, RelativePointerUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _relative_pointer: &ZwpRelativePointerV1,
        request: zwp_relative_pointer_v1::Request,
        _data: &RelativePointerUserData<D>,
        _dh: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            zwp_relative_pointer_v1::Request::Destroy => {}
            _ => unreachable!(),
        }
    }

    fn destroyed(
        _state: &mut D,
        _: ClientId,
        object: &ZwpRelativePointerV1,
        data: &RelativePointerUserData<D>,
    ) {
        if let Some(ref handle) = data.handle {
            handle
                .wp_relative
                .known_relative_pointers
                .lock()
                .unwrap()
                .retain(|p| p.id() != object.id());
        }
    }
}

#[macro_export]
macro_rules! delegate_relative_pointer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::relative_pointer::zv1::server::zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1: ()
        ] => $crate::wayland::relative_pointer::RelativePointerManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::relative_pointer::zv1::server::zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1: ()
        ] => $crate::wayland::relative_pointer::RelativePointerManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::relative_pointer::zv1::server::zwp_relative_pointer_v1::ZwpRelativePointerV1: $crate::wayland::relative_pointer::RelativePointerUserData<Self>
        ] => $crate::wayland::relative_pointer::RelativePointerManagerState);
    };
}
