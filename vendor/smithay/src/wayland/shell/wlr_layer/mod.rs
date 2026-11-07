//! Utilities for handling shell surfaces with the `wlr_layer_shell` protocol.

use std::sync::{Arc, Mutex};

use wayland_protocols_wlr::layer_shell::v1::server::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1,
};
use wayland_server::{
    backend::GlobalId,
    protocol::{wl_output::WlOutput, wl_surface},
    Client, DisplayHandle, GlobalDispatch, Resource,
};

use crate::{
    utils::{alive_tracker::IsAlive, Logical, Serial, Size, SERIAL_COUNTER},
    wayland::{
        compositor::{self, Cacheable},
        shell::xdg,
    },
};

mod handlers;
mod types;

pub use handlers::WlrLayerSurfaceUserData;
pub use types::{Anchor, ExclusiveZone, KeyboardInteractivity, Layer, Margins};

pub const LAYER_SURFACE_ROLE: &str = "zwlr_layer_surface_v1";

pub type LayerSurfaceData = Mutex<LayerSurfaceAttributes>;

#[derive(Debug)]
pub struct LayerSurfaceAttributes {
    surface: zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
    pub configured: bool,
    pub configure_serial: Option<Serial>,
    pub initial_configure_sent: bool,
    pending_configures: Vec<LayerSurfaceConfigure>,
    pub server_pending: Option<LayerSurfaceState>,
    pub last_acked: Option<LayerSurfaceState>,
    pub current: LayerSurfaceState,
}

impl LayerSurfaceAttributes {
    fn new(surface: zwlr_layer_surface_v1::ZwlrLayerSurfaceV1) -> Self {
        Self {
            surface,
            configured: false,
            configure_serial: None,
            initial_configure_sent: false,
            pending_configures: Vec::new(),
            server_pending: None,
            last_acked: None,
            current: Default::default(),
        }
    }

    fn ack_configure(&mut self, serial: Serial) -> Option<LayerSurfaceConfigure> {
        let configure = self
            .pending_configures
            .iter()
            .find(|configure| configure.serial == serial)
            .cloned()?;

        self.last_acked = Some(configure.state.clone());

        self.configured = true;
        self.configure_serial = Some(serial);
        self.pending_configures.retain(|c| c.serial > serial);
        Some(configure)
    }

    fn reset(&mut self) {
        self.configured = false;
        self.configure_serial = None;
        self.initial_configure_sent = false;
        self.pending_configures = Vec::new();
        self.server_pending = None;
        self.last_acked = None;
        self.current = Default::default();
    }

    fn current_server_state(&self) -> &LayerSurfaceState {
        self.pending_configures
            .last()
            .map(|c| &c.state)
            .or(self.last_acked.as_ref())
            .unwrap_or(&self.current)
    }

    fn has_pending_changes(&self) -> bool {
        self.server_pending
            .as_ref()
            .map(|s| s != self.current_server_state())
            .unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LayerSurfaceState {
    pub size: Option<Size<i32, Logical>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LayerSurfaceCachedState {
    pub size: Size<i32, Logical>,
    pub anchor: Anchor,
    pub exclusive_zone: ExclusiveZone,
    pub margin: Margins,
    pub keyboard_interactivity: KeyboardInteractivity,
    pub layer: Layer,
}

impl Cacheable for LayerSurfaceCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        *self
    }
    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

#[derive(Debug, Clone)]
pub struct WlrLayerShellState {
    known_layers: Arc<Mutex<Vec<LayerSurface>>>,
    shell_global: GlobalId,
}

#[allow(missing_debug_implementations)]
pub struct WlrLayerShellGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl WlrLayerShellState {
    pub fn new<D>(display: &DisplayHandle) -> WlrLayerShellState
    where
        D: GlobalDispatch<ZwlrLayerShellV1, WlrLayerShellGlobalData>,
        D: 'static,
    {
        Self::new_with_filter::<D, _>(display, |_| true)
    }

    pub fn new_with_filter<D, F>(display: &DisplayHandle, filter: F) -> WlrLayerShellState
    where
        D: GlobalDispatch<ZwlrLayerShellV1, WlrLayerShellGlobalData>,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let shell_global = display.create_global::<D, ZwlrLayerShellV1, WlrLayerShellGlobalData>(
            4,
            WlrLayerShellGlobalData {
                filter: Box::new(filter),
            },
        );

        WlrLayerShellState {
            known_layers: Default::default(),
            shell_global,
        }
    }

    pub fn shell_global(&self) -> GlobalId {
        self.shell_global.clone()
    }

    pub fn layer_surfaces(&self) -> impl DoubleEndedIterator<Item = LayerSurface> {
        self.known_layers.lock().unwrap().clone().into_iter()
    }
}

#[allow(unused_variables)]
pub trait WlrLayerShellHandler {
    fn shell_state(&mut self) -> &mut WlrLayerShellState;

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        output: Option<WlOutput>,
        layer: Layer,
        namespace: String,
    );

    fn new_popup(&mut self, parent: LayerSurface, popup: xdg::PopupSurface) {}

    fn ack_configure(&mut self, surface: wl_surface::WlSurface, configure: LayerSurfaceConfigure) {}

    fn layer_destroyed(&mut self, surface: LayerSurface) {}
}

#[derive(Debug, Clone)]
pub struct LayerSurface {
    wl_surface: wl_surface::WlSurface,
    shell_surface: zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
}

impl std::cmp::PartialEq for LayerSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.wl_surface == other.wl_surface
    }
}

impl LayerSurface {
    #[inline]
    pub fn alive(&self) -> bool {
        self.wl_surface.alive() && self.shell_surface.alive()
    }

    fn get_pending_state(&self, attributes: &mut LayerSurfaceAttributes) -> Option<LayerSurfaceState> {
        if !attributes.initial_configure_sent {
            return Some(
                attributes
                    .server_pending
                    .take()
                    .unwrap_or_else(|| attributes.current_server_state().clone()),
            );
        }

        if !attributes.has_pending_changes() {
            return None;
        }

        attributes.server_pending.take()
    }

    pub fn send_pending_configure(&self) -> Option<Serial> {
        if self.has_pending_changes() {
            Some(self.send_configure())
        } else {
            None
        }
    }

    pub fn send_configure(&self) -> Serial {
        let configure = compositor::with_states(&self.wl_surface, |states| {
            let mut attributes = states
                .data_map
                .get::<Mutex<LayerSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap();

            let state = self
                .get_pending_state(&mut attributes)
                .unwrap_or_else(|| attributes.current_server_state().clone());

            let configure = LayerSurfaceConfigure {
                serial: SERIAL_COUNTER.next_serial(),
                state,
            };

            attributes.pending_configures.push(configure.clone());
            attributes.initial_configure_sent = true;

            configure
        });

        let (width, height) = configure.state.size.unwrap_or_default().into();
        let serial = configure.serial;
        self.shell_surface
            .configure(serial.into(), width as u32, height as u32);
        serial
    }

    pub fn ensure_configured(&self) -> bool {
        let configured = compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<Mutex<LayerSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap()
                .configured
        });
        if !configured {
            self.shell_surface.post_error(
                zwlr_layer_shell_v1::Error::AlreadyConstructed,
                "layer_surface has never been configured",
            );
        }
        configured
    }

    pub fn send_close(&self) {
        self.shell_surface.closed()
    }

    #[inline]
    pub fn wl_surface(&self) -> &wl_surface::WlSurface {
        &self.wl_surface
    }

    pub fn with_pending_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut LayerSurfaceState) -> T,
    {
        compositor::with_states(&self.wl_surface, |states| {
            let mut attributes = states
                .data_map
                .get::<Mutex<LayerSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap();
            if attributes.server_pending.is_none() {
                attributes.server_pending = Some(attributes.current_server_state().clone());
            }

            let server_pending = attributes.server_pending.as_mut().unwrap();
            f(server_pending)
        })
    }

    pub fn has_pending_changes(&self) -> bool {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<Mutex<LayerSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap();

            !attributes.initial_configure_sent || attributes.has_pending_changes()
        })
    }

    pub fn current_state(&self) -> LayerSurfaceState {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<Mutex<LayerSurfaceAttributes>>()
                .unwrap()
                .lock()
                .unwrap();

            attributes.current.clone()
        })
    }

    pub fn shell_surface(&self) -> &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1 {
        &self.shell_surface
    }
}

#[derive(Debug, Clone)]
pub struct LayerSurfaceConfigure {
    pub state: LayerSurfaceState,

    pub serial: Serial,
}

#[macro_export]
macro_rules! delegate_layer_shell {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __ZwlrLayerShellV1 =
            $crate::reexports::wayland_protocols_wlr::layer_shell::v1::server::zwlr_layer_shell_v1::ZwlrLayerShellV1;
        type __ZwlrLayerShellSurfaceV1 =
            $crate::reexports::wayland_protocols_wlr::layer_shell::v1::server::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwlrLayerShellV1: ()
        ] => $crate::wayland::shell::wlr_layer::WlrLayerShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwlrLayerShellSurfaceV1: $crate::wayland::shell::wlr_layer::WlrLayerSurfaceUserData
        ] => $crate::wayland::shell::wlr_layer::WlrLayerShellState);

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwlrLayerShellV1: $crate::wayland::shell::wlr_layer::WlrLayerShellGlobalData
        ] => $crate::wayland::shell::wlr_layer::WlrLayerShellState);
    };
}
