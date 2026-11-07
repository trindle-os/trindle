use std::{
    mem,
    sync::{mpsc::Receiver, Weak},
};

use drm_fourcc::DrmFourcc;
use tracing::instrument;
use x11rb::{connection::Connection, protocol::xproto::PixmapWrapper, rust_connection::RustConnection};

use crate::{
    backend::{
        allocator::{
            dmabuf::{AnyError, Dmabuf},
            Allocator, Slot, Swapchain,
        },
        x11::{buffer::PixmapWrapperExt, window_inner::WindowInner, AllocateBuffersError, Window},
    },
    utils::{Logical, Size},
};

use super::{WindowTemporary, X11Error};

#[derive(Debug, thiserror::Error)]
pub enum PresentError {
    #[error("The Dmabuf had too many planes")]
    TooManyPlanes,

    #[error("Duplicating the file descriptors for the dmabuf handles failed")]
    DupFailed(String),

    #[error("Buffer had incorrect format, expected: {0}")]
    IncorrectFormat(DrmFourcc),
}

#[derive(Debug)]
pub struct X11Surface {
    pub(crate) connection: Weak<RustConnection>,
    pub(crate) window: Weak<WindowInner>,
    pub(crate) resize: Receiver<Size<u16, Logical>>,
    pub(crate) swapchain: Swapchain<Box<dyn Allocator<Buffer = Dmabuf, Error = AnyError> + 'static>>,
    pub(crate) format: DrmFourcc,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) buffer: Option<Slot<Dmabuf>>,
    pub(crate) span: tracing::Span,
}

impl X11Surface {
    pub fn window(&self) -> Option<impl AsRef<Window> + '_> {
        self.window.upgrade().map(Window).map(WindowTemporary)
    }

    pub fn format(&self) -> DrmFourcc {
        self.format
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    #[profiling::function]
    pub fn buffer(&mut self) -> Result<(Dmabuf, u8), AllocateBuffersError> {
        if let Some(new_size) = self.resize.try_iter().last() {
            self.resize(new_size);
        }

        if self.buffer.is_none() {
            self.buffer = Some(
                self.swapchain
                    .acquire()
                    .map_err(Into::<AllocateBuffersError>::into)?
                    .ok_or(AllocateBuffersError::NoFreeSlots)?,
            );
        }

        let slot = self.buffer.as_ref().unwrap();
        let age = slot.age();
        Ok(((*slot).clone(), age))
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    #[profiling::function]
    pub fn submit(&mut self) -> Result<(), X11Error> {
        if let Some(connection) = self.connection.upgrade() {
            let mut next = self
                .swapchain
                .acquire()
                .map_err(Into::<AllocateBuffersError>::into)?
                .ok_or(AllocateBuffersError::NoFreeSlots)?;

            if let Some(current) = self.buffer.as_mut() {
                mem::swap(&mut next, current);
            }

            {
                let window = self.window().ok_or(AllocateBuffersError::WindowDestroyed)?;
                let pixmap = PixmapWrapper::with_dmabuf(&*connection, window.as_ref(), &next)?;

                let _ = pixmap.present(&*connection, window.as_ref())?;
            }
            self.swapchain.submitted(&next);

            let _ = connection.flush();
        }
        Ok(())
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    pub fn reset_buffers(&mut self) {
        self.swapchain.reset_buffers();
        self.buffer = None;
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    pub fn reset_buffer_ages(&mut self) {
        self.swapchain.reset_buffer_ages();
    }

    fn resize(&mut self, size: Size<u16, Logical>) {
        self.swapchain.resize(size.w as u32, size.h as u32);
        self.buffer = None;

        self.width = size.w;
        self.height = size.h;
    }
}
