use drm_fourcc::{DrmFourcc, DrmModifier};
use thiserror::Error;

use crate::backend::{
    allocator::dmabuf::{DmabufMappingFailed, DmabufSyncFailed},
    SwapBuffersError,
};

#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_shm;

#[derive(Debug, Error)]
pub enum PixmanError {
    #[error("Unsupported number of planes")]
    UnsupportedNumberOfPlanes,
    #[error("Unsupported pixel format: {0:?}")]
    UnsupportedPixelFormat(DrmFourcc),
    #[error("Unsupported modifier: {0:?}")]
    UnsupportedModifier(DrmModifier),
    #[error("Unsupported wl_shm format: {0:?}")]
    #[cfg(feature = "wayland_frontend")]
    UnsupportedWlPixelFormat(wl_shm::Format),
    #[error("Incomplete buffer {expected} < {actual}")]
    IncompleteBuffer {
        expected: usize,
        actual: usize,
    },
    #[error("Error accessing the buffer ({0:?})")]
    #[cfg(feature = "wayland_frontend")]
    BufferAccessError(#[from] crate::wayland::shm::BufferAccessError),
    #[error("Import failed")]
    ImportFailed,
    #[error("The underlying buffer has been destroyed")]
    BufferDestroyed,
    #[error("Mapping the buffer failed: {0}")]
    Map(#[from] DmabufMappingFailed),
    #[error("Synchronizing buffer failed: {0}")]
    Sync(#[from] DmabufSyncFailed),
    #[error("The requested operation failed")]
    Failed(#[from] pixman::OperationFailed),
    #[error("No target is currently bound")]
    NoTargetBound,
    #[error("The requested operation is not supported")]
    Unsupported,
    #[error("Blocking for a synchronization primitive got interrupted")]
    SyncInterrupted,
}

impl From<PixmanError> for SwapBuffersError {
    #[inline]
    fn from(value: PixmanError) -> Self {
        match value {
            x @ PixmanError::SyncInterrupted => SwapBuffersError::TemporaryFailure(Box::new(x)),
            x => SwapBuffersError::ContextLost(Box::new(x)),
        }
    }
}
