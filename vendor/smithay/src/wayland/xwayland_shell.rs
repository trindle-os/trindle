//! Helpers for handling the xwayland shell protocol.

use std::collections::HashMap;

use tracing::{debug, warn};
use wayland_protocols::xwayland::shell::v1::server::{
    xwayland_shell_v1::{self, XwaylandShellV1},
    xwayland_surface_v1::{self, XwaylandSurfaceV1},
};
use wayland_server::{
    backend::GlobalId,
    protocol::wl_surface::{self, WlSurface},
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

use crate::{
    wayland::compositor,
    xwayland::{xwm::XwmId, X11Surface, XWaylandClientData, XwmHandler},
};

pub const XWAYLAND_SHELL_ROLE: &str = "xwayland_shell";

const VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct XWaylandShellState {
    global: GlobalId,
    by_serial: HashMap<u64, WlSurface>,
}

impl XWaylandShellState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<XwaylandShellV1, ()>,
        D: Dispatch<XwaylandShellV1, ()>,
        D: Dispatch<XwaylandSurfaceV1, XWaylandSurfaceUserData>,
        D: 'static,
    {
        let global = display.create_global::<D, XwaylandShellV1, _>(VERSION, ());
        Self {
            global,
            by_serial: HashMap::new(),
        }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }

    pub fn surface_for_serial(&self, serial: u64) -> Option<WlSurface> {
        self.by_serial.get(&serial).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct XWaylandSurfaceUserData {
    pub(crate) wl_surface: wl_surface::WlSurface,
}

pub trait XWaylandShellHandler {
    fn xwayland_shell_state(&mut self) -> &mut XWaylandShellState;

    fn surface_associated(&mut self, xwm: XwmId, wl_surface: wl_surface::WlSurface, surface: X11Surface) {
        let _ = (xwm, wl_surface, surface);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct XWaylandShellCachedState {
    pub serial: Option<u64>,
}

impl compositor::Cacheable for XWaylandShellCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        *self
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

impl<D> GlobalDispatch<XwaylandShellV1, (), D> for XWaylandShellState
where
    D: GlobalDispatch<XwaylandShellV1, ()>,
    D: Dispatch<XwaylandShellV1, ()>,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<XwaylandShellV1>,
        _global_data: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, _global_data: &()) -> bool {
        client.get_data::<XWaylandClientData>().is_some()
    }
}

impl<D> Dispatch<XwaylandShellV1, (), D> for XWaylandShellState
where
    D: Dispatch<XwaylandShellV1, ()>,
    D: Dispatch<XwaylandSurfaceV1, XWaylandSurfaceUserData>,
    D: XWaylandShellHandler + XwmHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        resource: &XwaylandShellV1,
        request: <XwaylandShellV1 as Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            xwayland_shell_v1::Request::GetXwaylandSurface { id, surface } => {
                if compositor::give_role(&surface, XWAYLAND_SHELL_ROLE).is_err() {
                    resource.post_error(xwayland_shell_v1::Error::Role, "Surface already has a role.");
                    return;
                }

                compositor::add_pre_commit_hook::<D, _>(&surface, serial_commit_hook);

                data_init.init(id, XWaylandSurfaceUserData { wl_surface: surface });
            }
            xwayland_shell_v1::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<XwaylandSurfaceV1, XWaylandSurfaceUserData, D> for XWaylandShellState
where
    D: Dispatch<XwaylandSurfaceV1, XWaylandSurfaceUserData>,
    D: XWaylandShellHandler,
    D: 'static,
    D: XwmHandler,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &XwaylandSurfaceV1,
        request: <XwaylandSurfaceV1 as Resource>::Request,
        data: &XWaylandSurfaceUserData,
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            xwayland_surface_v1::Request::SetSerial { serial_lo, serial_hi } => {
                let serial = u64::from(serial_lo) | (u64::from(serial_hi) << 32);

                compositor::with_states(&data.wl_surface, |states| {
                    states
                        .cached_state
                        .get::<XWaylandShellCachedState>()
                        .pending()
                        .serial = Some(serial);
                });
            }
            xwayland_surface_v1::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

fn serial_commit_hook<D: XWaylandShellHandler + XwmHandler + 'static>(
    state: &mut D,
    _dh: &DisplayHandle,
    surface: &WlSurface,
) {
    if let Some(serial) = compositor::with_states(surface, |states| {
        states
            .cached_state
            .get::<XWaylandShellCachedState>()
            .pending()
            .serial
    }) {
        if let Some(client) = surface.client() {
            if let Some(xwm_id) = client
                .get_data::<XWaylandClientData>()
                .and_then(|data| data.user_data().get::<XwmId>())
            {
                let xwm = XwmHandler::xwm_state(state, *xwm_id);

                if let Some(window) = xwm.unpaired_surfaces.remove(&serial) {
                    if let Some(xsurface) = xwm
                        .windows
                        .iter()
                        .find(|x| x.window_id() == window || x.mapped_window_id() == Some(window))
                        .cloned()
                    {
                        debug!(
                            window = xsurface.window_id(),
                            wl_surface = ?surface.id().protocol_id(),
                            "associated X11 window to wl_surface in commit hook",
                        );

                        xsurface.state.lock().unwrap().wl_surface = Some(surface.clone());

                        XWaylandShellHandler::surface_associated(state, *xwm_id, surface.clone(), xsurface);
                    } else {
                        warn!(
                            window,
                            wl_surface = ?surface.id().protocol_id(),
                            "Unknown X11 window associated to wl_surface in commit hook"
                        )
                    }
                } else {
                    XWaylandShellHandler::xwayland_shell_state(state)
                        .by_serial
                        .insert(serial, surface.clone());
                }
            }
        }
    }
}

#[macro_export]
macro_rules! delegate_xwayland_shell {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xwayland::shell::v1::server::xwayland_shell_v1::XwaylandShellV1: ()
        ] => $crate::wayland::xwayland_shell::XWaylandShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xwayland::shell::v1::server::xwayland_shell_v1::XwaylandShellV1: ()
        ] => $crate::wayland::xwayland_shell::XWaylandShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xwayland::shell::v1::server::xwayland_surface_v1::XwaylandSurfaceV1: $crate::wayland::xwayland_shell::XWaylandSurfaceUserData
        ] => $crate::wayland::xwayland_shell::XWaylandShellState);
    };
}
