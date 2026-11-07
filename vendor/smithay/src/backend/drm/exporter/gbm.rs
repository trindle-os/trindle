//! Implementation for ExportFramebuffer and related utilizes for gbm types.

use std::os::unix::io::AsFd;

use drm::node::DrmNode;

use super::{ExportBuffer, ExportFramebuffer};
#[cfg(feature = "wayland_frontend")]
use crate::backend::drm::gbm::framebuffer_from_wayland_buffer;
use crate::backend::{
    allocator::{
        dmabuf::AsDmabuf,
        gbm::{GbmBuffer, GbmConvertError},
    },
    drm::{
        gbm::{framebuffer_from_bo, framebuffer_from_dmabuf, Error as GbmError, GbmFramebuffer},
        DrmDeviceFd,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Dmabuf(#[from] GbmConvertError),
    #[error(transparent)]
    Gbm(#[from] GbmError),
}

#[derive(Debug, Clone)]
pub struct GbmFramebufferExporter<A: AsFd + 'static> {
    gbm: gbm::Device<A>,
    drm_node: Option<DrmNode>,
    #[cfg_attr(not(feature = "wayland_frontend"), allow(unused))]
    import_node: Option<DrmNode>,
}

impl<A: AsFd + 'static> GbmFramebufferExporter<A> {
    pub fn new(gbm: gbm::Device<A>, import_node: Option<DrmNode>) -> Self {
        let drm_node = DrmNode::from_file(gbm.as_fd()).ok();
        Self {
            gbm,
            drm_node,
            import_node,
        }
    }
}

impl<A: AsFd + 'static> ExportFramebuffer<GbmBuffer> for GbmFramebufferExporter<A> {
    type Framebuffer = GbmFramebuffer;
    type Error = Error;

    #[profiling::function]
    fn add_framebuffer(
        &self,
        drm: &DrmDeviceFd,
        buffer: ExportBuffer<'_, GbmBuffer>,
        use_opaque: bool,
    ) -> Result<Option<Self::Framebuffer>, Self::Error> {
        let framebuffer = match buffer {
            #[cfg(feature = "wayland_frontend")]
            ExportBuffer::Wayland(wl_buffer) => {
                framebuffer_from_wayland_buffer(drm, &self.gbm, wl_buffer, use_opaque)?
            }
            ExportBuffer::Allocator(buffer) => {
                let foreign = self.drm_node.is_none()
                    || buffer.device_node().is_none()
                    || self.drm_node != buffer.device_node();
                if foreign {
                    tracing::debug!("importing foreign buffer");
                    let dmabuf = buffer.export()?;
                    framebuffer_from_dmabuf(drm, &self.gbm, &dmabuf, use_opaque, true).map(Some)?
                } else {
                    framebuffer_from_bo(drm, buffer, use_opaque)
                        .map_err(GbmError::Drm)
                        .map(Some)?
                }
            }
        };
        Ok(framebuffer)
    }

    #[inline]
    #[cfg(feature = "wayland_frontend")]
    fn can_add_framebuffer(&self, buffer: &ExportBuffer<'_, GbmBuffer>) -> bool {
        match buffer {
            #[cfg(not(all(feature = "backend_egl", feature = "use_system_lib")))]
            ExportBuffer::Wayland(buffer) => crate::wayland::dmabuf::get_dmabuf(buffer)
                .ok()
                .is_some_and(|buf| buf.node().is_some_and(|node| Some(node) == self.import_node)),
            #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
            ExportBuffer::Wayland(buffer) => match crate::backend::renderer::buffer_type(buffer) {
                Some(crate::backend::renderer::BufferType::Dma) => crate::wayland::dmabuf::get_dmabuf(buffer)
                    .unwrap()
                    .node()
                    .is_some_and(|node| Some(node) == self.import_node),
                Some(crate::backend::renderer::BufferType::Egl) => true,
                _ => false,
            },
            ExportBuffer::Allocator(_) => true,
        }
    }

    #[inline]
    #[cfg(not(feature = "wayland_frontend"))]
    fn can_add_framebuffer(&self, buffer: &ExportBuffer<'_, GbmBuffer>) -> bool {
        match buffer {
            ExportBuffer::Allocator(_) => true,
        }
    }
}
