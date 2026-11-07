//! WlOutput advertising capabilities.

mod handlers;
pub(crate) mod xdg;

use std::sync::{atomic::Ordering, Arc};

use crate::{
    output::{Inner, Mode, Output, Scale, Subpixel, WeakOutput},
    utils::iter::new_locked_obj_iter,
};

use atomic_float::AtomicF64;
use tracing::info;
use wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1;
use wayland_server::{
    backend::{ClientId, GlobalId},
    protocol::{
        wl_output::{Mode as WMode, Subpixel as WlSubpixel, Transform, WlOutput},
        wl_surface,
    },
    Client, DisplayHandle, GlobalDispatch, Resource,
};

use crate::utils::{Logical, Point};

pub use self::handlers::XdgOutputUserData;

#[derive(Debug, Default)]
pub struct OutputManagerState {
    xdg_output_manager: Option<GlobalId>,
}

#[derive(Debug)]
pub struct WlOutputData {
    output: Output,
}

pub trait OutputHandler {
    fn output_bound(&mut self, _output: Output, _wl_output: WlOutput) {}
}

impl OutputManagerState {
    pub fn new() -> Self {
        Self {
            xdg_output_manager: None,
        }
    }

    pub fn new_with_xdg_output<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WlOutput, WlOutputData>,
        D: GlobalDispatch<ZxdgOutputManagerV1, ()>,
        D: 'static,
    {
        let xdg_output_manager = display.create_global::<D, ZxdgOutputManagerV1, _>(3, ());

        Self {
            xdg_output_manager: Some(xdg_output_manager),
        }
    }

    pub fn xdg_output_manager_global(&self) -> Option<GlobalId> {
        self.xdg_output_manager.clone()
    }
}

#[derive(Debug)]
pub struct OutputUserData {
    pub(crate) output: WeakOutput,
    last_client_scale: AtomicF64,
    client_scale: Arc<AtomicF64>,
}

impl Clone for OutputUserData {
    fn clone(&self) -> Self {
        OutputUserData {
            output: self.output.clone(),
            last_client_scale: AtomicF64::new(self.last_client_scale.load(Ordering::Acquire)),
            client_scale: self.client_scale.clone(),
        }
    }
}

impl Inner {
    fn send_geometry_to(&self, output: &WlOutput) {
        output.geometry(
            self.location.x,
            self.location.y,
            self.physical.size.w,
            self.physical.size.h,
            self.physical.subpixel.into(),
            self.physical.make.clone(),
            self.physical.model.clone(),
            self.transform.into(),
        );
    }
}

impl From<Subpixel> for WlSubpixel {
    #[inline]
    fn from(s: Subpixel) -> Self {
        match s {
            Subpixel::HorizontalBgr => WlSubpixel::HorizontalBgr,
            Subpixel::HorizontalRgb => WlSubpixel::HorizontalRgb,
            Subpixel::None => WlSubpixel::None,
            Subpixel::Unknown => WlSubpixel::Unknown,
            Subpixel::VerticalBgr => WlSubpixel::VerticalBgr,
            Subpixel::VerticalRgb => WlSubpixel::VerticalRgb,
        }
    }
}

impl Output {
    pub fn create_global<D>(&self, display: &DisplayHandle) -> GlobalId
    where
        D: GlobalDispatch<WlOutput, WlOutputData>,
        D: 'static,
    {
        info!(output = self.name(), "Creating new wl_output");
        self.inner.0.lock().unwrap().handle = Some(display.backend_handle().downgrade());
        display.create_global::<D, WlOutput, _>(4, WlOutputData { output: self.clone() })
    }

    pub fn from_resource(output: &WlOutput) -> Option<Output> {
        output.data::<OutputUserData>().and_then(|ud| ud.output.upgrade())
    }

    pub(crate) fn wl_change_current_state(
        &self,
        new_mode: Option<Mode>,
        new_transform: Option<Transform>,
        new_scale: Option<Scale>,
        new_location: Option<Point<i32, Logical>>,
    ) {
        let inner = self.inner.0.lock().unwrap();
        if let Some(xdg_output) = inner.xdg_output.as_ref() {
            xdg_output.change_current_state(new_mode, new_scale, new_location, new_transform);
        }

        let mut flags = WMode::Current;
        if inner.preferred_mode == new_mode {
            flags |= WMode::Preferred;
        }

        for output in &inner.instances {
            let Ok(output) = output.upgrade() else {
                continue;
            };

            let data = output.data::<OutputUserData>().unwrap();
            let client_scale = data.client_scale.load(Ordering::Acquire);
            let scale_changed = client_scale != data.last_client_scale.swap(client_scale, Ordering::AcqRel);

            if let Some(mode) = new_mode {
                output.mode(flags, mode.size.w, mode.size.h, mode.refresh);
            }
            if new_transform.is_some() || new_location.is_some() {
                inner.send_geometry_to(&output);
            }
            if (new_scale.is_some() || scale_changed) && output.version() >= 2 {
                let scale = (inner.scale.integer_scale() as f64 / client_scale).max(1.).ceil() as i32;
                output.scale(scale);
            }
            if output.version() >= 2 {
                output.done();
            }
        }
    }

    pub fn owns(&self, output: &WlOutput) -> bool {
        self.inner
            .0
            .lock()
            .unwrap()
            .instances
            .iter()
            .any(|o| o.id() == output.id())
    }

    pub fn client_outputs<'a>(&'a self, client: &Client) -> impl Iterator<Item = WlOutput> + 'a {
        self.client_outputs_internal(client.id())
    }

    fn client_outputs_internal(&self, client: ClientId) -> impl Iterator<Item = WlOutput> + '_ {
        let guard = self.inner.0.lock().unwrap();

        new_locked_obj_iter(guard, client, |inner| inner.instances.iter())
    }

    #[profiling::function]
    pub fn enter(&self, surface: &wl_surface::WlSurface) {
        let mut inner = self.inner.0.lock().unwrap();
        if inner.surfaces.insert(surface.downgrade()) {
            let client = inner
                .handle
                .as_ref()
                .and_then(|handle| handle.upgrade())
                .and_then(|handle| handle.get_client(surface.id()).ok());
            drop(inner);

            if let Some(client) = client {
                for output in self.client_outputs_internal(client) {
                    surface.enter(&output);
                }
            }
        }
    }

    #[profiling::function]
    pub fn leave(&self, surface: &wl_surface::WlSurface) {
        let mut inner = self.inner.0.lock().unwrap();
        if inner.surfaces.remove(&surface.downgrade()) {
            let client = inner
                .handle
                .as_ref()
                .and_then(|handle| handle.upgrade())
                .and_then(|handle| handle.get_client(surface.id()).ok());
            drop(inner);

            if let Some(client) = client {
                for output in self.client_outputs_internal(client) {
                    surface.leave(&output);
                }
            }
        }
    }

    pub(crate) fn cleanup_surfaces(&self) {
        let mut inner = self.inner.0.lock().unwrap();
        inner.surfaces.retain(|s| s.is_alive());
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_output {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_output::WlOutput: $crate::wayland::output::WlOutputData
        ] => $crate::wayland::output::OutputManagerState);
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1: ()
        ] => $crate::wayland::output::OutputManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_output::WlOutput: $crate::wayland::output::OutputUserData
        ] => $crate::wayland::output::OutputManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1: $crate::wayland::output::XdgOutputUserData
        ] => $crate::wayland::output::OutputManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1: ()
        ] => $crate::wayland::output::OutputManagerState);
    };
}
