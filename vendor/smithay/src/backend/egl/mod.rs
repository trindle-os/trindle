//! Common traits and types for egl rendering.

use std::ffi::c_void;
use std::fmt;

pub mod context;
pub use self::context::EGLContext;
mod device;
mod error;
pub use self::error::*;
use crate::backend::SwapBuffersError as GraphicsSwapBuffersError;
#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
use crate::utils::{Buffer, Size};

#[allow(non_camel_case_types, dead_code, unused_mut, non_upper_case_globals)]
pub mod ffi;
use self::display::EGLDisplayHandle;
#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
use self::ffi::egl::types::EGLImage;

pub mod display;
pub mod fence;
pub mod native;
pub mod surface;
pub use self::device::EGLDevice;
pub use self::display::EGLDisplay;
pub use self::surface::EGLSurface;

use std::ffi::CString;
#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EglExtensionNotSupportedError(pub &'static [&'static str]);

impl fmt::Display for EglExtensionNotSupportedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        write!(
            formatter,
            "None of the following EGL extensions is supported by the underlying EGL implementation,
                     at least one is required: {:?}",
            self.0
        )
    }
}

impl ::std::error::Error for EglExtensionNotSupportedError {}

pub unsafe fn get_proc_address(symbol: &str) -> *const c_void {
    let addr = CString::new(symbol.as_bytes()).unwrap();
    let addr = addr.as_ptr();
    ffi::egl::GetProcAddress(addr) as *const _
}

#[cfg(feature = "wayland_frontend")]
#[derive(thiserror::Error)]
pub enum BufferAccessError {
    #[error("The corresponding context was lost")]
    ContextLost,
    #[error("This buffer is not managed by EGL. Err: {0:}")]
    NotManaged(#[source] EGLError),
    #[error("Failed to create EGLImages from the buffer. Err: {0:}")]
    EGLImageCreationFailed(#[source] EGLError),
    #[error("{0}")]
    EglExtensionNotSupported(#[from] EglExtensionNotSupportedError),
    #[error("Multi-planar formats (like {0:?}) are unsupported")]
    UnsupportedMultiPlanarFormat(Format),
    #[error("This buffer has been destroyed")]
    Destroyed,
}

#[cfg(feature = "wayland_frontend")]
impl fmt::Debug for BufferAccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            BufferAccessError::ContextLost => write!(formatter, "BufferAccessError::ContextLost"),
            BufferAccessError::NotManaged(_) => write!(formatter, "BufferAccessError::NotManaged"),
            BufferAccessError::EGLImageCreationFailed(_) => {
                write!(formatter, "BufferAccessError::EGLImageCreationFailed")
            }
            BufferAccessError::EglExtensionNotSupported(ref err) => write!(formatter, "{:?}", err),
            BufferAccessError::UnsupportedMultiPlanarFormat(ref fmt) => write!(
                formatter,
                "BufferAccessError::UnsupportedMultiPlanerFormat({:?})",
                fmt
            ),
            BufferAccessError::Destroyed => write!(formatter, "BufferAccessError::Destroyed"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SwapBuffersError {
    #[error("{0:}")]
    EGLSwapBuffers(#[source] EGLError),
    #[error("{0:}")]
    EGLCreateSurface(#[source] EGLError),
}

impl std::convert::From<SwapBuffersError> for GraphicsSwapBuffersError {
    #[inline]
    fn from(value: SwapBuffersError) -> Self {
        match value {
            x @ SwapBuffersError::EGLSwapBuffers(EGLError::BadSurface) => {
                GraphicsSwapBuffersError::TemporaryFailure(Box::new(x))
            }
            x @ SwapBuffersError::EGLSwapBuffers(_) => GraphicsSwapBuffersError::ContextLost(Box::new(x)),
            x @ SwapBuffersError::EGLCreateSurface(_) => GraphicsSwapBuffersError::ContextLost(Box::new(x)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("`eglMakeCurrent` failed: {0}")]
pub struct MakeCurrentError(#[from] EGLError);

impl From<MakeCurrentError> for GraphicsSwapBuffersError {
    #[inline]
    fn from(err: MakeCurrentError) -> GraphicsSwapBuffersError {
        match err {
            x @ MakeCurrentError(EGLError::BadAccess) => {
                GraphicsSwapBuffersError::TemporaryFailure(Box::new(x))
            }
            x @ MakeCurrentError(EGLError::BadSurface) => {
                GraphicsSwapBuffersError::TemporaryFailure(Box::new(x))
            }
            x @ MakeCurrentError(EGLError::BadCurrentSurface) => {
                GraphicsSwapBuffersError::TemporaryFailure(Box::new(x))
            }
            x => GraphicsSwapBuffersError::ContextLost(Box::new(x)),
        }
    }
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    RGB = ffi::egl::TEXTURE_RGB as i32,
    RGBA = ffi::egl::TEXTURE_RGBA as i32,
    External = ffi::egl::TEXTURE_EXTERNAL_WL,
    Y_UV = ffi::egl::TEXTURE_Y_UV_WL,
    Y_U_V = ffi::egl::TEXTURE_Y_U_V_WL,
    Y_XUXV = ffi::egl::TEXTURE_Y_XUXV_WL,
}

impl Format {
    #[inline]
    pub fn num_planes(&self) -> usize {
        match *self {
            Format::RGB | Format::RGBA | Format::External => 1,
            Format::Y_UV | Format::Y_XUXV => 2,
            Format::Y_U_V => 3,
        }
    }
}

#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
#[derive(Debug)]
pub struct EGLBuffer {
    display: Arc<EGLDisplayHandle>,
    pub size: Size<i32, Buffer>,
    pub y_inverted: bool,
    pub format: Format,
    images: Vec<EGLImage>,
}

#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
impl EGLBuffer {
    #[inline]
    pub fn num_planes(&self) -> usize {
        self.format.num_planes()
    }

    pub fn image(&self, plane: usize) -> Option<EGLImage> {
        if plane > self.format.num_planes() {
            None
        } else {
            Some(self.images[plane])
        }
    }

    pub fn into_images(mut self) -> Vec<EGLImage> {
        self.images.drain(..).collect()
    }
}

#[cfg(all(feature = "wayland_frontend", feature = "use_system_lib"))]
impl Drop for EGLBuffer {
    fn drop(&mut self) {
        for image in self.images.drain(..) {
            unsafe {
                ffi::egl::DestroyImageKHR(**self.display, image);
            }
        }
    }
}
