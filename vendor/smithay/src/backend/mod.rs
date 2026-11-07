//! Backend (rendering/input) helpers.

pub mod allocator;
pub mod input;
pub mod renderer;

#[cfg(feature = "backend_drm")]
pub mod drm;
#[cfg(feature = "backend_egl")]
pub mod egl;
#[cfg(feature = "backend_libinput")]
pub mod libinput;
#[cfg(feature = "backend_session")]
pub mod session;
#[cfg(feature = "backend_udev")]
pub mod udev;

#[cfg(feature = "backend_vulkan")]
pub mod vulkan;

#[cfg(feature = "backend_winit")]
pub mod winit;

#[cfg(feature = "backend_x11")]
pub mod x11;

#[derive(Debug, thiserror::Error)]
pub enum SwapBuffersError {
    #[error("Buffers are already swapped, swap_buffers was called too many times")]
    AlreadySwapped,
    #[error("The context has been lost, it needs to be recreated: {0}")]
    ContextLost(Box<dyn std::error::Error + Send + Sync>),
    #[error("A temporary condition caused the page flip to fail: {0}")]
    TemporaryFailure(Box<dyn std::error::Error + Send + Sync>),
}
