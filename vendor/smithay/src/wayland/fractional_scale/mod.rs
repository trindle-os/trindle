//! Utilities for handling the `wp_fractional_scale` protocol.

use std::cell::RefCell;

use wayland_protocols::wp::fractional_scale::v1::server::{
    wp_fractional_scale_manager_v1, wp_fractional_scale_v1,
};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface, Dispatch, DisplayHandle, GlobalDispatch, Resource, Weak,
};

use super::compositor::{with_states, SurfaceData};

#[derive(Debug)]
pub struct FractionalScaleManagerState {
    global: GlobalId,
}

impl FractionalScaleManagerState {
    pub fn new<D>(display: &DisplayHandle) -> FractionalScaleManagerState
    where
        D: GlobalDispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
            + Dispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
            + Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, Weak<wl_surface::WlSurface>>
            + 'static,
        D: FractionalScaleHandler,
    {
        FractionalScaleManagerState {
            global: display
                .create_global::<D, wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>(1, ()),
        }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, (), D>
    for FractionalScaleManagerState
where
    D: GlobalDispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, Weak<wl_surface::WlSurface>>,
    D: FractionalScaleHandler,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &wayland_server::Client,
        resource: wayland_server::New<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
        _global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, (), D>
    for FractionalScaleManagerState
where
    D: GlobalDispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, Weak<wl_surface::WlSurface>>,
    D: FractionalScaleHandler,
{
    fn request(
        state: &mut D,
        _client: &wayland_server::Client,
        _resource: &wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1,
        request: <wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1 as Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_fractional_scale_manager_v1::Request::Destroy => {
            }
            wp_fractional_scale_manager_v1::Request::GetFractionalScale { id, surface } => {
                let already_has_fractional_scale = with_states(&surface, |states| {
                    states
                        .data_map
                        .get::<FractionalScaleStateUserData>()
                        .map(|v| v.borrow().fractional_scale.is_some())
                        .unwrap_or(false)
                });

                if already_has_fractional_scale {
                    surface.post_error(
                        wp_fractional_scale_manager_v1::Error::FractionalScaleExists as u32,
                        "the surface already has a fractional_scale object associated".to_string(),
                    );
                    return;
                }

                let fractional_scale: wp_fractional_scale_v1::WpFractionalScaleV1 =
                    data_init.init(id, surface.downgrade());

                with_states(&surface, move |states| {
                    with_fractional_scale(states, move |data| {
                        if let Some(scale) = data.preferred_scale {
                            fractional_scale.preferred_scale(f64::round(scale * 120.0) as u32);
                        }
                        data.fractional_scale = Some(fractional_scale);
                    });
                });
                state.new_fractional_scale(surface);
            }
            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, Weak<wl_surface::WlSurface>, D>
    for FractionalScaleManagerState
where
    D: GlobalDispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, ()>
        + Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, Weak<wl_surface::WlSurface>>,
    D: FractionalScaleHandler,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &wp_fractional_scale_v1::WpFractionalScaleV1,
        request: <wp_fractional_scale_v1::WpFractionalScaleV1 as Resource>::Request,
        data: &Weak<wl_surface::WlSurface>,
        _dhandle: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_fractional_scale_v1::Request::Destroy => {
                if let Ok(surface) = data.upgrade() {
                    with_states(&surface, |states| {
                        states
                            .data_map
                            .get::<FractionalScaleStateUserData>()
                            .and_then(|v| v.borrow_mut().fractional_scale.take());
                    })
                }
            }
            _ => unreachable!(),
        }
    }
}

pub trait FractionalScaleHandler {
    fn new_fractional_scale(&mut self, _surface: wl_surface::WlSurface) {}
}

pub type FractionalScaleStateUserData = RefCell<FractionalScaleState>;

#[derive(Debug, Default)]
pub struct FractionalScaleState {
    fractional_scale: Option<wp_fractional_scale_v1::WpFractionalScaleV1>,
    preferred_scale: Option<f64>,
}

impl FractionalScaleState {
    pub fn set_preferred_scale(&mut self, scale: f64) {
        if self.preferred_scale != Some(scale) {
            if let Some(obj) = &self.fractional_scale {
                obj.preferred_scale(f64::round(scale * 120.0) as u32);
            }
            self.preferred_scale = Some(scale);
        }
    }

    pub fn preferred_scale(&self) -> Option<f64> {
        self.preferred_scale
    }
}

pub fn with_fractional_scale<F, T>(states: &SurfaceData, f: F) -> T
where
    F: FnOnce(&mut FractionalScaleState) -> T,
{
    let mut fractional_scale = states
        .data_map
        .get_or_insert(FractionalScaleStateUserData::default)
        .borrow_mut();

    f(&mut fractional_scale)
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_fractional_scale {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fractional_scale::v1::server::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1: ()
        ] => $crate::wayland::fractional_scale::FractionalScaleManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fractional_scale::v1::server::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1: ()
        ] => $crate::wayland::fractional_scale::FractionalScaleManagerState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::fractional_scale::v1::server::wp_fractional_scale_v1::WpFractionalScaleV1: $crate::reexports::wayland_server::Weak<$crate::reexports::wayland_server::protocol::wl_surface::WlSurface>
        ] => $crate::wayland::fractional_scale::FractionalScaleManagerState);
    };
}
