//! Implementation of `wp_alpha_modifier` protocol.

use std::sync::{
    atomic::{self, AtomicBool},
    Mutex,
};

use wayland_protocols::wp::alpha_modifier::v1::server::{
    wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1, wp_alpha_modifier_v1::WpAlphaModifierV1,
};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface::WlSurface, Dispatch, DisplayHandle, GlobalDispatch, Resource,
    Weak,
};

use super::compositor::Cacheable;

mod dispatch;

#[derive(Debug, Clone, Copy, Default)]
pub struct AlphaModifierSurfaceCachedState {
    multiplier: Option<u32>,
}

impl AlphaModifierSurfaceCachedState {
    pub fn multiplier(&self) -> Option<u32> {
        self.multiplier
    }

    pub fn multiplier_f32(&self) -> Option<f32> {
        self.multiplier
            .map(|multiplier| multiplier as f32 / u32::MAX as f32)
    }

    pub fn multiplier_f64(&self) -> Option<f64> {
        self.multiplier
            .map(|multiplier| multiplier as f64 / u32::MAX as f64)
    }
}

impl Cacheable for AlphaModifierSurfaceCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        *self
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

#[derive(Debug)]
struct AlphaModifierSurfaceData {
    is_resource_attached: AtomicBool,
}

impl AlphaModifierSurfaceData {
    fn new() -> Self {
        Self {
            is_resource_attached: AtomicBool::new(false),
        }
    }

    fn set_is_resource_attached(&self, is_attached: bool) {
        self.is_resource_attached
            .store(is_attached, atomic::Ordering::Release)
    }

    fn is_resource_attached(&self) -> bool {
        self.is_resource_attached.load(atomic::Ordering::Acquire)
    }
}

#[derive(Debug)]
pub struct AlphaModifierSurfaceUserData(Mutex<Weak<WlSurface>>);

impl AlphaModifierSurfaceUserData {
    fn new(surface: WlSurface) -> Self {
        Self(Mutex::new(surface.downgrade()))
    }

    fn wl_surface(&self) -> Option<WlSurface> {
        self.0.lock().unwrap().upgrade().ok()
    }
}

#[derive(Debug)]
pub struct AlphaModifierState {
    global: GlobalId,
}

impl AlphaModifierState {
    pub fn new<D>(display: &DisplayHandle) -> AlphaModifierState
    where
        D: GlobalDispatch<WpAlphaModifierV1, ()>
            + Dispatch<WpAlphaModifierV1, ()>
            + Dispatch<WpAlphaModifierSurfaceV1, AlphaModifierSurfaceUserData>
            + 'static,
    {
        let global = display.create_global::<D, WpAlphaModifierV1, _>(1, ());

        AlphaModifierState { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

#[macro_export]
macro_rules! delegate_alpha_modifier {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __WpAlphaModifierV1 =
            $crate::reexports::wayland_protocols::wp::alpha_modifier::v1::server::wp_alpha_modifier_v1::WpAlphaModifierV1;
        type __WpAlphaModifierSurfaceV1 =
            $crate::reexports::wayland_protocols::wp::alpha_modifier::v1::server::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1;

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpAlphaModifierV1: ()
            ] => $crate::wayland::alpha_modifier::AlphaModifierState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpAlphaModifierV1: ()
            ] => $crate::wayland::alpha_modifier::AlphaModifierState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpAlphaModifierSurfaceV1: $crate::wayland::alpha_modifier::AlphaModifierSurfaceUserData
            ] => $crate::wayland::alpha_modifier::AlphaModifierState
        );
    };
}
