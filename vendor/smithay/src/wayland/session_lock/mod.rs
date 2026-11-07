//! Utilities for handling the `ext-session-lock` protocol.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use _session_lock::ext_session_lock_manager_v1::{ExtSessionLockManagerV1, Request};
use _session_lock::ext_session_lock_v1::ExtSessionLockV1;
use wayland_protocols::ext::session_lock::v1::server as _session_lock;
use wayland_server::protocol::wl_output::WlOutput;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};

use crate::wayland::session_lock::surface::LockSurfaceConfigure;

mod lock;
mod surface;

pub use lock::SessionLockState;
pub use surface::{ExtLockSurfaceUserData, LockSurface, LockSurfaceState};

const MANAGER_VERSION: u32 = 1;

#[derive(Debug)]
pub struct SessionLockManagerState {
    pub(crate) locked_outputs: Vec<WlOutput>,
}

impl SessionLockManagerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<ExtSessionLockManagerV1, SessionLockManagerGlobalData>,
        D: Dispatch<ExtSessionLockManagerV1, ()>,
        D: Dispatch<ExtSessionLockV1, SessionLockState>,
        D: SessionLockHandler,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = SessionLockManagerGlobalData {
            filter: Box::new(filter),
        };
        display.create_global::<D, ExtSessionLockManagerV1, _>(MANAGER_VERSION, data);

        Self {
            locked_outputs: Vec::new(),
        }
    }
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct SessionLockManagerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl<D> GlobalDispatch<ExtSessionLockManagerV1, SessionLockManagerGlobalData, D> for SessionLockManagerState
where
    D: GlobalDispatch<ExtSessionLockManagerV1, SessionLockManagerGlobalData>,
    D: Dispatch<ExtSessionLockManagerV1, ()>,
    D: Dispatch<ExtSessionLockV1, SessionLockState>,
    D: SessionLockHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _display: &DisplayHandle,
        _client: &Client,
        manager: New<ExtSessionLockManagerV1>,
        _global_data: &SessionLockManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, global_data: &SessionLockManagerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<ExtSessionLockManagerV1, (), D> for SessionLockManagerState
where
    D: GlobalDispatch<ExtSessionLockManagerV1, SessionLockManagerGlobalData>,
    D: Dispatch<ExtSessionLockManagerV1, ()>,
    D: Dispatch<ExtSessionLockV1, SessionLockState>,
    D: SessionLockHandler,
    D: 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        _manager: &ExtSessionLockManagerV1,
        request: Request,
        _data: &(),
        _display: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            Request::Lock { id } => {
                let lock_state = SessionLockState::new();
                let lock_status = lock_state.lock_status.clone();
                let lock = data_init.init(id, lock_state);
                state.lock(SessionLocker::new(lock, lock_status));
            }
            Request::Destroy => (),
            _ => unreachable!(),
        }
    }
}

pub trait SessionLockHandler {
    fn lock_state(&mut self) -> &mut SessionLockManagerState;

    fn lock(&mut self, confirmation: SessionLocker);

    fn unlock(&mut self);

    fn new_surface(&mut self, surface: LockSurface, output: WlOutput);

    fn ack_configure(&mut self, _surface: WlSurface, _configure: LockSurfaceConfigure) {}
}

#[derive(Debug)]
pub struct SessionLocker {
    lock: Option<ExtSessionLockV1>,
    lock_status: Arc<AtomicBool>,
}

impl Drop for SessionLocker {
    fn drop(&mut self) {
        if let Some(lock) = self.lock.take() {
            lock.finished();
        }
    }
}

impl SessionLocker {
    fn new(lock: ExtSessionLockV1, lock_status: Arc<AtomicBool>) -> Self {
        Self {
            lock: Some(lock),
            lock_status,
        }
    }

    pub fn ext_session_lock(&self) -> &ExtSessionLockV1 {
        self.lock.as_ref().unwrap()
    }

    pub fn lock(mut self) {
        if let Some(lock) = self.lock.take() {
            self.lock_status.store(true, Ordering::Relaxed);
            lock.locked();
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_session_lock {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::session_lock::v1::server::ext_session_lock_manager_v1::ExtSessionLockManagerV1: $crate::wayland::session_lock::SessionLockManagerGlobalData
        ] => $crate::wayland::session_lock::SessionLockManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::session_lock::v1::server::ext_session_lock_manager_v1::ExtSessionLockManagerV1: ()
        ] => $crate::wayland::session_lock::SessionLockManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::session_lock::v1::server::ext_session_lock_v1::ExtSessionLockV1: $crate::wayland::session_lock::SessionLockState
        ] => $crate::wayland::session_lock::SessionLockManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::ext::session_lock::v1::server::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1: $crate::wayland::session_lock::ExtLockSurfaceUserData
        ] => $crate::wayland::session_lock::SessionLockManagerState);
    };
}
