//! Utilities for handling the `wp-fifo` protocol.

use std::cell::RefCell;

use wayland_protocols::wp::fifo::v1::server::{
    wp_fifo_manager_v1::{self, WpFifoManagerV1},
    wp_fifo_v1::{self, WpFifoV1},
};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface::WlSurface, DataInit, Dispatch, DisplayHandle, GlobalDispatch,
    New, Resource, Weak,
};

use crate::wayland::compositor::{add_blocker, add_pre_commit_hook};

use super::compositor::{is_sync_subsurface, with_states, Barrier, Cacheable};

#[derive(Debug)]
pub struct FifoManagerState {
    global: GlobalId,
    is_managed: bool,
}

impl FifoManagerState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WpFifoManagerV1, bool>,
        D: Dispatch<WpFifoManagerV1, bool>,
        D: 'static,
    {
        Self::new_internal::<D>(display, true)
    }

    pub fn unmanaged<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WpFifoManagerV1, bool>,
        D: Dispatch<WpFifoManagerV1, bool>,
        D: 'static,
    {
        Self::new_internal::<D>(display, false)
    }

    fn new_internal<D>(display: &DisplayHandle, is_managed: bool) -> Self
    where
        D: GlobalDispatch<WpFifoManagerV1, bool>,
        D: Dispatch<WpFifoManagerV1, bool>,
        D: 'static,
    {
        let global = display.create_global::<D, WpFifoManagerV1, _>(1, is_managed);

        Self { global, is_managed }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }

    pub fn is_managed(&self) -> bool {
        self.is_managed
    }
}

impl<D> GlobalDispatch<WpFifoManagerV1, bool, D> for FifoManagerState
where
    D: GlobalDispatch<WpFifoManagerV1, bool>,
    D: Dispatch<WpFifoManagerV1, bool>,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &wayland_server::Client,
        resource: New<WpFifoManagerV1>,
        global_data: &bool,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, *global_data);
    }
}

impl<D> Dispatch<WpFifoManagerV1, bool, D> for FifoManagerState
where
    D: Dispatch<WpFifoManagerV1, bool>,
    D: Dispatch<WpFifoV1, Weak<WlSurface>>,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &WpFifoManagerV1,
        request: wp_fifo_manager_v1::Request,
        data: &bool,
        _dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        let is_managed = *data;

        match request {
            wp_fifo_manager_v1::Request::GetFifo { id, surface } => {
                let (is_initial, has_active_fifo) = with_states(&surface, |states| {
                    let marker = states.data_map.get::<RefCell<FifoMarker>>();
                    (
                        marker.is_none(),
                        marker.map(|m| m.borrow().0.is_some()).unwrap_or(false),
                    )
                });

                if has_active_fifo {
                    surface.post_error(
                        wp_fifo_manager_v1::Error::AlreadyExists,
                        "the surface has already a fifo object associated",
                    );
                    return;
                }

                if is_managed && is_initial {
                    add_pre_commit_hook::<D, _>(&surface, |_, _, surface| {
                        let fifo_barrier = with_states(surface, |states| {
                            let fifo_state = *states.cached_state.get::<FifoCachedState>().pending();

                            let fifo_barrier = fifo_state
                                .wait_barrier
                                .then(|| {
                                    states
                                        .cached_state
                                        .get::<FifoBarrierCachedState>()
                                        .pending()
                                        .barrier
                                        .take()
                                })
                                .flatten();

                            if fifo_state.set_barrier {
                                states
                                    .cached_state
                                    .get::<FifoBarrierCachedState>()
                                    .pending()
                                    .barrier = Some(Barrier::new(false));
                            }

                            fifo_barrier
                        });

                        if let Some(barrier) = fifo_barrier {
                            let skip = barrier.is_signaled() || is_sync_subsurface(surface);
                            if !skip {
                                add_blocker(surface, barrier);
                            }
                        }
                    });
                }

                let fifo: WpFifoV1 = data_init.init(id, surface.downgrade());

                with_states(&surface, |states| {
                    states
                        .data_map
                        .get_or_insert(|| RefCell::new(FifoMarker(None)))
                        .borrow_mut()
                        .0 = Some(fifo);
                });
            }
            wp_fifo_manager_v1::Request::Destroy => (),
            _ => unreachable!(),
        }
    }
}

struct FifoMarker(Option<WpFifoV1>);

impl<D> Dispatch<WpFifoV1, Weak<WlSurface>, D> for FifoManagerState
where
    D: Dispatch<WpFifoV1, Weak<WlSurface>>,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        resource: &WpFifoV1,
        request: wp_fifo_v1::Request,
        data: &Weak<WlSurface>,
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            wp_fifo_v1::Request::SetBarrier => {
                let Ok(surface) = data.upgrade() else {
                    resource.post_error(
                        wp_fifo_v1::Error::SurfaceDestroyed as u32,
                        "the surface associated with this fifo object has been destroyed".to_string(),
                    );
                    return;
                };
                with_states(&surface, move |states| {
                    states.cached_state.get::<FifoCachedState>().pending().set_barrier = true;
                });
            }
            wp_fifo_v1::Request::WaitBarrier => {
                let Ok(surface) = data.upgrade() else {
                    resource.post_error(
                        wp_fifo_v1::Error::SurfaceDestroyed as u32,
                        "the surface associated with this fifo object has been destroyed".to_string(),
                    );
                    return;
                };
                with_states(&surface, move |states| {
                    states
                        .cached_state
                        .get::<FifoCachedState>()
                        .pending()
                        .wait_barrier = true;
                });
            }
            wp_fifo_v1::Request::Destroy => {
                if let Ok(surface) = data.upgrade() {
                    with_states(&surface, |states| {
                        states
                            .data_map
                            .get::<RefCell<FifoMarker>>()
                            .unwrap()
                            .borrow_mut()
                            .0 = None;
                    });
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FifoCachedState {
    pub set_barrier: bool,

    pub wait_barrier: bool,
}

impl Cacheable for FifoCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        std::mem::take(self)
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

#[derive(Debug, Default)]
pub struct FifoBarrierCachedState {
    pub barrier: Option<Barrier>,
}

impl Cacheable for FifoBarrierCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        Self {
            barrier: self.barrier.clone(),
        }
    }

    fn merge_into(mut self, into: &mut Self, _dh: &DisplayHandle) {
        let Some(barrier) = self.barrier.take() else {
            return;
        };

        if into.barrier.as_ref() == Some(&barrier) || barrier.is_signaled() {
            return;
        }

        if let Some(barrier) = into.barrier.replace(barrier) {
            barrier.signal();
        }
    }
}

#[macro_export]
macro_rules! delegate_fifo {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fifo::v1::server::wp_fifo_manager_v1::WpFifoManagerV1: bool
        ] => $crate::wayland::fifo::FifoManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fifo::v1::server::wp_fifo_manager_v1::WpFifoManagerV1: bool
        ] => $crate::wayland::fifo::FifoManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fifo::v1::server::wp_fifo_v1::WpFifoV1: $crate::reexports::wayland_server::Weak<$crate::reexports::wayland_server::protocol::wl_surface::WlSurface>
        ] => $crate::wayland::fifo::FifoManagerState);
    };
}
