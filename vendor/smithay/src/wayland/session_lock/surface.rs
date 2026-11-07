//! ext-session-lock surface.

use std::sync::Mutex;

use crate::utils::{IsAlive, Logical, Serial, Size, SERIAL_COUNTER};
use crate::wayland::compositor;
use _session_lock::ext_session_lock_surface_v1::{Error, ExtSessionLockSurfaceV1, Request};
use wayland_protocols::ext::session_lock::v1::server::{self as _session_lock, ext_session_lock_surface_v1};
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, Resource, Weak};

use crate::wayland::session_lock::{SessionLockHandler, SessionLockManagerState};

#[derive(Debug)]
pub struct ExtLockSurfaceUserData {
    pub(crate) surface: Weak<WlSurface>,
}

impl<D> Dispatch<ExtSessionLockSurfaceV1, ExtLockSurfaceUserData, D> for SessionLockManagerState
where
    D: Dispatch<ExtSessionLockSurfaceV1, ExtLockSurfaceUserData>,
    D: SessionLockHandler,
    D: 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        lock_surface: &ExtSessionLockSurfaceV1,
        request: Request,
        data: &ExtLockSurfaceUserData,
        _display: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            Request::AckConfigure { serial } => {
                let Ok(surface) = data.surface.upgrade() else {
                    return;
                };

                let serial = Serial::from(serial);
                let configure = compositor::with_states(&surface, |states| {
                    let surface_data = states.data_map.get::<Mutex<LockSurfaceAttributes>>();
                    surface_data.unwrap().lock().unwrap().ack_configure(serial)
                });

                match configure {
                    Some(configure) => state.ack_configure(surface.clone(), configure),
                    None => lock_surface.post_error(
                        Error::InvalidSerial,
                        format!("wrong configure serial: {}", <u32>::from(serial)),
                    ),
                }
            }
            Request::Destroy => (),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct LockSurfaceAttributes {
    pub(crate) surface: ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,

    pub configure_serial: Option<Serial>,

    pub server_pending: Option<LockSurfaceState>,

    pub pending_configures: Vec<LockSurfaceConfigure>,

    pub last_acked: Option<LockSurfaceState>,

    pub current: LockSurfaceState,
}

impl LockSurfaceAttributes {
    pub(crate) fn new(surface: ext_session_lock_surface_v1::ExtSessionLockSurfaceV1) -> Self {
        Self {
            surface,
            configure_serial: None,
            server_pending: None,
            pending_configures: vec![],
            last_acked: None,
            current: Default::default(),
        }
    }

    fn ack_configure(&mut self, serial: Serial) -> Option<LockSurfaceConfigure> {
        let configure = self
            .pending_configures
            .iter()
            .find(|configure| configure.serial == serial)
            .cloned()?;

        self.pending_configures
            .retain(|configure| configure.serial > serial);
        self.last_acked = Some(configure.state);
        self.configure_serial = Some(serial);

        Some(configure)
    }
}

#[derive(Clone, Debug)]
pub struct LockSurface {
    shell_surface: ExtSessionLockSurfaceV1,
    surface: WlSurface,
}

impl PartialEq for LockSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.surface == other.surface
    }
}

impl LockSurface {
    pub(crate) fn new(surface: WlSurface, shell_surface: ExtSessionLockSurfaceV1) -> Self {
        Self {
            surface,
            shell_surface,
        }
    }

    #[inline]
    pub fn alive(&self) -> bool {
        self.surface.alive()
    }

    pub fn get_pending_state(&self, attributes: &mut LockSurfaceAttributes) -> Option<LockSurfaceState> {
        let server_pending = attributes.server_pending.take()?;

        let pending = attributes.pending_configures.last();
        let last_state = pending.map(|c| &c.state).or(attributes.last_acked.as_ref());

        match last_state {
            Some(state) if state == &server_pending => None,
            _ => Some(server_pending),
        }
    }

    pub fn send_configure(&self) {
        compositor::with_states(&self.surface, |states| {
            let attributes = states.data_map.get::<Mutex<LockSurfaceAttributes>>();
            let mut attributes = attributes.unwrap().lock().unwrap();

            let pending = match self.get_pending_state(&mut attributes) {
                Some(pending) => pending,
                None => return,
            };
            let configure = LockSurfaceConfigure::new(pending);

            let (width, height) = configure.state.size.unwrap_or_default().into();
            let serial = configure.serial;

            attributes.pending_configures.push(configure);

            self.shell_surface.configure(serial.into(), width, height);
        })
    }

    #[inline]
    pub fn wl_surface(&self) -> &WlSurface {
        &self.surface
    }

    pub fn with_pending_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut LockSurfaceState) -> T,
    {
        compositor::with_states(&self.surface, |states| {
            let attributes = states.data_map.get::<Mutex<LockSurfaceAttributes>>();
            let mut attributes = attributes.unwrap().lock().unwrap();

            let current = attributes.current;
            let server_pending = attributes.server_pending.get_or_insert(current);

            f(server_pending)
        })
    }

    #[allow(unused)]
    pub fn current_state(&self) -> LockSurfaceState {
        compositor::with_states(&self.surface, |states| {
            states
                .data_map
                .get::<Mutex<LockSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap()
                .current
        })
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct LockSurfaceState {
    pub size: Option<Size<u32, Logical>>,
}

#[derive(Debug, Copy, Clone)]
pub struct LockSurfaceConfigure {
    pub state: LockSurfaceState,

    pub serial: Serial,
}

impl LockSurfaceConfigure {
    fn new(state: LockSurfaceState) -> Self {
        Self {
            serial: SERIAL_COUNTER.next_serial(),
            state,
        }
    }
}
