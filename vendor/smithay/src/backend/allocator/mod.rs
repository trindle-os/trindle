//! Buffer allocation and management.

pub mod dmabuf;
#[cfg(feature = "backend_drm")]
pub mod dumb;
pub mod format;
#[cfg(feature = "backend_gbm")]
pub mod gbm;
#[cfg(feature = "backend_vulkan")]
pub mod vulkan;

mod swapchain;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::utils::{Buffer as BufferCoords, Size};
pub use swapchain::{Slot, Swapchain};

pub use drm_fourcc::{
    DrmFormat as Format, DrmFourcc as Fourcc, DrmModifier as Modifier, DrmVendor as Vendor,
    UnrecognizedFourcc, UnrecognizedVendor,
};

pub trait Buffer {
    fn width(&self) -> u32 {
        self.size().w as u32
    }
    fn height(&self) -> u32 {
        self.size().h as u32
    }
    fn size(&self) -> Size<i32, BufferCoords>;
    fn format(&self) -> Format;
}

pub trait Allocator {
    type Buffer: Buffer;
    type Error: std::error::Error;

    fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        fourcc: Fourcc,
        modifiers: &[Modifier],
    ) -> Result<Self::Buffer, Self::Error>;
}

impl<A: Allocator> Allocator for Arc<Mutex<A>> {
    type Buffer = A::Buffer;
    type Error = A::Error;

    fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        fourcc: Fourcc,
        modifiers: &[Modifier],
    ) -> Result<Self::Buffer, Self::Error> {
        let mut guard = self.lock().unwrap();
        guard.create_buffer(width, height, fourcc, modifiers)
    }
}

impl<A: Allocator> Allocator for Rc<RefCell<A>> {
    type Buffer = A::Buffer;
    type Error = A::Error;

    fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        fourcc: Fourcc,
        modifiers: &[Modifier],
    ) -> Result<Self::Buffer, Self::Error> {
        self.borrow_mut().create_buffer(width, height, fourcc, modifiers)
    }
}

impl<B: Buffer, E: std::error::Error> Allocator for Box<dyn Allocator<Buffer = B, Error = E> + 'static> {
    type Buffer = B;
    type Error = E;

    fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        fourcc: Fourcc,
        modifiers: &[Modifier],
    ) -> Result<B, E> {
        (**self).create_buffer(width, height, fourcc, modifiers)
    }
}
