//! Implementation of wp_content_type protocol.

use std::sync::{
    atomic::{self, AtomicBool},
    Mutex,
};

use wayland_protocols::wp::content_type::v1::server::{
    wp_content_type_manager_v1::WpContentTypeManagerV1,
    wp_content_type_v1::{self, WpContentTypeV1},
};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface::WlSurface, Dispatch, DisplayHandle, GlobalDispatch, Resource,
    Weak,
};

use super::compositor::Cacheable;

mod dispatch;

#[derive(Debug, Clone, Copy)]
pub struct ContentTypeSurfaceCachedState {
    content_type: wp_content_type_v1::Type,
}

impl ContentTypeSurfaceCachedState {
    pub fn content_type(&self) -> &wp_content_type_v1::Type {
        &self.content_type
    }
}

impl Default for ContentTypeSurfaceCachedState {
    fn default() -> Self {
        Self {
            content_type: wp_content_type_v1::Type::None,
        }
    }
}

impl Cacheable for ContentTypeSurfaceCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        *self
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

#[derive(Debug)]
struct ContentTypeSurfaceData {
    is_resource_attached: AtomicBool,
}

impl ContentTypeSurfaceData {
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
pub struct ContentTypeUserData(Mutex<Weak<WlSurface>>);

impl ContentTypeUserData {
    fn new(surface: WlSurface) -> Self {
        Self(Mutex::new(surface.downgrade()))
    }

    #[inline]
    fn wl_surface(&self) -> Option<WlSurface> {
        self.0.lock().unwrap().upgrade().ok()
    }
}

#[derive(Debug)]
pub struct ContentTypeState {
    global: GlobalId,
}

impl ContentTypeState {
    pub fn new<D>(display: &DisplayHandle) -> ContentTypeState
    where
        D: GlobalDispatch<WpContentTypeManagerV1, ()>
            + Dispatch<WpContentTypeManagerV1, ()>
            + Dispatch<WpContentTypeV1, ContentTypeUserData>
            + 'static,
    {
        let global = display.create_global::<D, WpContentTypeManagerV1, _>(1, ());

        ContentTypeState { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

#[macro_export]
macro_rules! delegate_content_type {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __WpContentTypeManagerV1 =
            $crate::reexports::wayland_protocols::wp::content_type::v1::server::wp_content_type_manager_v1::WpContentTypeManagerV1;
        type __WpContentTypeV1 =
            $crate::reexports::wayland_protocols::wp::content_type::v1::server::wp_content_type_v1::WpContentTypeV1;

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpContentTypeManagerV1: ()
            ] => $crate::wayland::content_type::ContentTypeState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpContentTypeManagerV1: ()
            ] => $crate::wayland::content_type::ContentTypeState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __WpContentTypeV1: $crate::wayland::content_type::ContentTypeUserData
            ] => $crate::wayland::content_type::ContentTypeState
        );
    };
}
