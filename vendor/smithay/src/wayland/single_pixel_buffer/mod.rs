//! Utilities for handling the `wp_single_pixel_buffer` protocol.

use wayland_protocols::wp::single_pixel_buffer::v1::server::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1;
use wayland_server::{
    backend::GlobalId, protocol::wl_buffer::WlBuffer, Dispatch, DisplayHandle, GlobalDispatch, Resource,
};

mod handlers;

#[derive(Debug)]
pub struct SinglePixelBufferState {
    global: GlobalId,
}

impl SinglePixelBufferState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WpSinglePixelBufferManagerV1, ()>,
        D: Dispatch<WpSinglePixelBufferManagerV1, ()>,
        D: 'static,
    {
        let global = display.create_global::<D, WpSinglePixelBufferManagerV1, _>(1, ());

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

#[derive(Debug)]
pub struct SinglePixelBufferUserData {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl SinglePixelBufferUserData {
    pub fn has_alpha(&self) -> bool {
        self.a != u32::MAX
    }

    pub fn rgba8888(&self) -> [u8; 4] {
        let divisor = u32::MAX / 255;

        [
            (self.r / divisor) as u8,
            (self.g / divisor) as u8,
            (self.b / divisor) as u8,
            (self.a / divisor) as u8,
        ]
    }

    pub fn rgba32f(&self) -> [f32; 4] {
        let divisor = u32::MAX as f32;

        [
            self.r as f32 / divisor,
            self.g as f32 / divisor,
            self.b as f32 / divisor,
            self.a as f32 / divisor,
        ]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BufferAccessError {
    #[error("non-single-pixel buffer")]
    NotManaged,
}

pub fn get_single_pixel_buffer(buffer: &WlBuffer) -> Result<&SinglePixelBufferUserData, BufferAccessError> {
    buffer
        .data::<SinglePixelBufferUserData>()
        .ok_or(BufferAccessError::NotManaged)
}

#[macro_export]
macro_rules! delegate_single_pixel_buffer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::single_pixel_buffer::v1::server::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1: ()
        ] => $crate::wayland::single_pixel_buffer::SinglePixelBufferState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::single_pixel_buffer::v1::server::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1: ()
        ] => $crate::wayland::single_pixel_buffer::SinglePixelBufferState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_buffer::WlBuffer: $crate::wayland::single_pixel_buffer::SinglePixelBufferUserData
        ] => $crate::wayland::single_pixel_buffer::SinglePixelBufferState);
    };
}
