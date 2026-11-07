use std::io;

use rustix::io::Errno;
use x11rb::rust_connection::{ConnectError, ConnectionError, ReplyError, ReplyOrIdError};

use crate::backend::{allocator::dmabuf::AnyError, drm::CreateDrmNodeError};

use super::PresentError;

#[derive(Debug, thiserror::Error)]
pub enum X11Error {
    #[error("Connecting to the X server failed")]
    ConnectionFailed(#[from] ConnectError),

    #[error("Connection to the X server was lost")]
    ConnectionLost,

    #[error("{0}")]
    MissingExtension(#[from] MissingExtensionError),

    #[error("Some protocol error occurred during setup")]
    Protocol(#[from] ReplyOrIdError),

    #[error("Creating the window failed")]
    CreateWindow(#[from] CreateWindowError),

    #[error("An X11 surface already exists for this window")]
    SurfaceExists,

    #[error("An invalid window was used to create an X11 surface")]
    InvalidWindow,

    #[error("The X server is not capable of direct rendering")]
    CannotDirectRender,

    #[error("Failed to allocate buffers needed to present to the window")]
    Allocation(#[from] AllocateBuffersError),

    #[error(transparent)]
    Present(#[from] PresentError),
}

impl From<ReplyError> for X11Error {
    #[inline]
    fn from(err: ReplyError) -> Self {
        Self::Protocol(err.into())
    }
}

impl From<ConnectionError> for X11Error {
    #[inline]
    fn from(err: ConnectionError) -> Self {
        Self::Protocol(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MissingExtensionError {
    #[error("Extension \"{name}\" version {major}.{minor} was not found.")]
    NotFound {
        name: &'static str,
        major: u32,
        minor: u32,
    },

    #[error("Extension \"{name}\" version {required_major}.{required_minor} is required but only version {available_major}.{available_minor} is available.")]
    WrongVersion {
        name: &'static str,
        required_major: u32,
        required_minor: u32,
        available_major: u32,
        available_minor: u32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateWindowError {
    #[error("No depth fulfilling the requirements was found")]
    NoDepth,

    #[error("No visual fulfilling the requirements was found")]
    NoVisual,
}

#[derive(Debug, thiserror::Error)]
pub enum AllocateBuffersError {
    #[error("Failed to open the DRM device to allocate buffers.")]
    OpenDevice(#[from] io::Error),

    #[error("The device used to allocate buffers is not the correct drm node type.")]
    UnsupportedDrmNode,

    #[error(transparent)]
    AllocationError(#[from] AnyError),

    #[error("No free slots in the swapchain")]
    NoFreeSlots,

    #[error("The window has been destroyed")]
    WindowDestroyed,
}

impl From<Errno> for AllocateBuffersError {
    #[inline]
    fn from(err: Errno) -> Self {
        Self::OpenDevice(err.into())
    }
}

impl From<CreateDrmNodeError> for AllocateBuffersError {
    #[inline]
    fn from(err: CreateDrmNodeError) -> Self {
        match err {
            CreateDrmNodeError::Io(err) => AllocateBuffersError::OpenDevice(err),
            CreateDrmNodeError::NotDrmNode => AllocateBuffersError::UnsupportedDrmNode,
        }
    }
}
