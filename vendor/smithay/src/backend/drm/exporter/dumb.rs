//! Implementation for ExportFramebuffer and related utilizies for dumb buffers.

use thiserror::Error;

use super::{ExportBuffer, ExportFramebuffer};
use crate::backend::{
    allocator::dumb::DumbBuffer,
    drm::{
        dumb::{framebuffer_from_dumb_buffer, DumbFramebuffer},
        error::AccessError,
        DrmDeviceFd,
    },
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported buffer supplied")]
    Unsupported,
    #[error("failed to add a framebuffer for the dumb buffer")]
    Drm(AccessError),
}

impl ExportFramebuffer<DumbBuffer> for DrmDeviceFd {
    type Framebuffer = DumbFramebuffer;
    type Error = Error;

    #[profiling::function]
    fn add_framebuffer(
        &self,
        _drm: &DrmDeviceFd,
        buffer: ExportBuffer<'_, DumbBuffer>,
        use_opaque: bool,
    ) -> Result<Option<Self::Framebuffer>, Self::Error> {
        match buffer {
            #[cfg(feature = "wayland_frontend")]
            ExportBuffer::Wayland(_) => return Err(Error::Unsupported),
            ExportBuffer::Allocator(buffer) => framebuffer_from_dumb_buffer(self, buffer, use_opaque)
                .map_err(Error::Drm)
                .map(Some),
        }
    }

    #[inline]
    fn can_add_framebuffer(&self, buffer: &ExportBuffer<'_, DumbBuffer>) -> bool {
        match buffer {
            #[cfg(feature = "wayland_frontend")]
            ExportBuffer::Wayland(_) => false,
            ExportBuffer::Allocator(_) => true,
        }
    }
}
