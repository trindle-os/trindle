//! Seat global utilities.

pub(crate) mod keyboard;
pub(crate) mod pointer;
mod touch;

use std::{borrow::Cow, fmt, sync::Arc};

use crate::input::{Inner, Seat, SeatHandler, SeatRc, SeatState};

pub use self::{
    keyboard::KeyboardUserData,
    pointer::{PointerUserData, CURSOR_IMAGE_ROLE},
    touch::TouchUserData,
};

use wayland_server::{
    backend::{ClientId, GlobalId, ObjectId},
    protocol::{
        wl_keyboard::WlKeyboard,
        wl_pointer::WlPointer,
        wl_seat::{self, WlSeat},
        wl_surface,
        wl_touch::WlTouch,
    },
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

use super::compositor::CompositorHandler;

pub trait WaylandFocus {
    fn wl_surface(&self) -> Option<Cow<'_, wl_surface::WlSurface>>;
    fn same_client_as(&self, object_id: &ObjectId) -> bool {
        self.wl_surface()
            .map(|s| s.id().same_client_as(object_id))
            .unwrap_or(false)
    }
}

impl WaylandFocus for wl_surface::WlSurface {
    #[inline]
    fn wl_surface(&self) -> Option<Cow<'_, wl_surface::WlSurface>> {
        Some(Cow::Borrowed(self))
    }
}

impl<D: SeatHandler> Inner<D> {
    fn compute_caps(&self) -> wl_seat::Capability {
        let mut caps = wl_seat::Capability::empty();
        if self.pointer.is_some() {
            caps |= wl_seat::Capability::Pointer;
        }
        if self.keyboard.is_some() {
            caps |= wl_seat::Capability::Keyboard;
        }
        if self.touch.is_some() {
            caps |= wl_seat::Capability::Touch;
        }
        caps
    }

    pub(crate) fn send_all_caps(&self) {
        let capabilities = self.compute_caps();
        for seat in &self.known_seats {
            if let Ok(seat) = seat.upgrade() {
                seat.capabilities(capabilities);
            }
        }
    }
}

pub struct SeatGlobalData<D: SeatHandler> {
    arc: Arc<SeatRc<D>>,
}

impl<D: SeatHandler> fmt::Debug for SeatGlobalData<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeatGlobalData").field("arc", &self.arc).finish()
    }
}

impl<D: SeatHandler + 'static> SeatState<D> {
    pub fn new_wl_seat<N>(&mut self, display: &DisplayHandle, name: N) -> Seat<D>
    where
        D: GlobalDispatch<WlSeat, SeatGlobalData<D>> + SeatHandler + 'static,
        <D as SeatHandler>::PointerFocus: WaylandFocus,
        <D as SeatHandler>::KeyboardFocus: WaylandFocus,
        N: Into<String>,
    {
        let Seat { arc } = self.new_seat(name);

        let global_id = display.create_global::<D, _, _>(9, SeatGlobalData { arc: arc.clone() });
        arc.inner.lock().unwrap().global = Some(global_id);

        Seat { arc }
    }
}

impl<D: SeatHandler + 'static> Seat<D> {
    pub fn owns(&self, seat: &wl_seat::WlSeat) -> bool {
        let inner = self.arc.inner.lock().unwrap();
        inner.known_seats.iter().any(|s| s == seat)
    }

    pub fn from_resource(seat: &WlSeat) -> Option<Self> {
        seat.data::<SeatUserData<D>>()
            .map(|d| d.arc.clone())
            .map(|arc| Self { arc })
    }

    pub fn client_seats(&self, client: &Client) -> Vec<WlSeat> {
        self.arc
            .inner
            .lock()
            .unwrap()
            .known_seats
            .iter()
            .filter_map(|w| w.upgrade().ok())
            .filter(|s| s.client().is_some_and(|c| &c == client))
            .collect()
    }

    pub fn global(&self) -> Option<GlobalId> {
        self.arc.inner.lock().unwrap().global.as_ref().cloned()
    }
}

pub struct SeatUserData<D: SeatHandler> {
    arc: Arc<SeatRc<D>>,
}

impl<D: SeatHandler> fmt::Debug for SeatUserData<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeatUserData").field("arc", &self.arc).finish()
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_seat {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_seat::WlSeat: $crate::wayland::seat::SeatGlobalData<$ty>
        ] => $crate::input::SeatState<$ty>);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_seat::WlSeat: $crate::wayland::seat::SeatUserData<$ty>
        ] => $crate::input::SeatState<$ty>);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_pointer::WlPointer: $crate::wayland::seat::PointerUserData<$ty>
        ] => $crate::input::SeatState<$ty>);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard: $crate::wayland::seat::KeyboardUserData<$ty>
        ] => $crate::input::SeatState<$ty>);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?$ty: [
            $crate::reexports::wayland_server::protocol::wl_touch::WlTouch: $crate::wayland::seat::TouchUserData<$ty>
        ] => $crate::input::SeatState<$ty>);
    };
}

impl<D> Dispatch<WlSeat, SeatUserData<D>, D> for SeatState<D>
where
    D: Dispatch<WlSeat, SeatUserData<D>>,
    D: Dispatch<WlKeyboard, KeyboardUserData<D>>,
    D: Dispatch<WlPointer, PointerUserData<D>>,
    D: Dispatch<WlTouch, TouchUserData<D>>,
    D: SeatHandler,
    D: CompositorHandler,
    <D as SeatHandler>::KeyboardFocus: WaylandFocus,
    D: 'static,
{
    fn request(
        state: &mut D,
        client: &wayland_server::Client,
        _resource: &WlSeat,
        request: wl_seat::Request,
        data: &SeatUserData<D>,
        _dh: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wl_seat::Request::GetPointer { id } => {
                let inner = data.arc.inner.lock().unwrap();

                let client_scale = state.client_compositor_state(client).clone_client_scale();
                let pointer = data_init.init(
                    id,
                    PointerUserData {
                        handle: inner.pointer.clone(),
                        client_scale,
                    },
                );

                if let Some(ref ptr_handle) = inner.pointer {
                    ptr_handle.wl_pointer.new_pointer(pointer);
                } else {
                }
            }
            wl_seat::Request::GetKeyboard { id } => {
                let inner = data.arc.inner.lock().unwrap();

                let keyboard = data_init.init(
                    id,
                    KeyboardUserData {
                        handle: inner.keyboard.clone(),
                    },
                );

                if let Some(ref h) = inner.keyboard {
                    h.new_kbd(keyboard);
                } else {
                }
            }
            wl_seat::Request::GetTouch { id } => {
                let inner = data.arc.inner.lock().unwrap();

                let client_scale = state.client_compositor_state(client).clone_client_scale();
                let touch = data_init.init(
                    id,
                    TouchUserData {
                        handle: inner.touch.clone(),
                        client_scale,
                    },
                );

                if let Some(ref h) = inner.touch {
                    h.new_touch(touch);
                } else {
                }
            }
            wl_seat::Request::Release => {
            }
            _ => unreachable!(),
        }
    }

    fn destroyed(_state: &mut D, _: ClientId, seat: &WlSeat, data: &SeatUserData<D>) {
        data.arc
            .inner
            .lock()
            .unwrap()
            .known_seats
            .retain(|s| s.id() != seat.id());
    }
}

impl<D> GlobalDispatch<WlSeat, SeatGlobalData<D>, D> for SeatState<D>
where
    D: GlobalDispatch<WlSeat, SeatGlobalData<D>>,
    D: Dispatch<WlSeat, SeatUserData<D>>,
    D: Dispatch<WlKeyboard, KeyboardUserData<D>>,
    D: Dispatch<WlPointer, PointerUserData<D>>,
    D: Dispatch<WlTouch, TouchUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &wayland_server::Client,
        resource: New<WlSeat>,
        global_data: &SeatGlobalData<D>,
        data_init: &mut DataInit<'_, D>,
    ) {
        let data = SeatUserData {
            arc: global_data.arc.clone(),
        };

        let resource = data_init.init(resource, data);

        if resource.version() >= 2 {
            resource.name(global_data.arc.name.clone());
        }

        let mut inner = global_data.arc.inner.lock().unwrap();
        resource.capabilities(inner.compute_caps());
        inner.known_seats.push(resource.downgrade());
    }
}
