use super::*;
use crate::backend::SwapBuffersError;

#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_shm;

#[derive(thiserror::Error, Debug)]
pub enum GlesError {
    #[error("Failed to compile Shader")]
    ShaderCompileError,
    #[error("Failed to link Program")]
    ProgramLinkError,
    #[error("Failed to bind Framebuffer")]
    FramebufferBindingError,
    #[error("Failed to load GL functions from EGL")]
    GLFunctionLoaderError,
    #[error("None of the following GL extensions is supported by the underlying GL implementation, at least one is required: {0:?}")]
    GLExtensionNotSupported(&'static [&'static str]),
    #[error("None of the following EGL extensions is supported by the underlying implementation, at least one is required: {0:?}")]
    EGLExtensionNotSupported(&'static [&'static str]),
    #[error(
        "The OpenGL ES version of the underlying GL implementation is too low, at least required: {0:?}"
    )]
    GLVersionNotSupported(version::GlVersion),
    #[error("Failed to active egl context")]
    ContextActivationError(#[from] crate::backend::egl::MakeCurrentError),
    #[error("Failed to convert between dmabuf and EGLImage")]
    BindBufferEGLError(#[source] crate::backend::egl::Error),
    #[error("Unknown pixel format")]
    UnknownPixelFormat,
    #[error("Unsupported pixel format: {0:?}")]
    UnsupportedPixelFormat(Fourcc),
    #[error("Unsupported pixel layout")]
    UnsupportedPixelLayout,
    #[error("Unsupported wl_shm format: {0:?}")]
    #[cfg(feature = "wayland_frontend")]
    UnsupportedWlPixelFormat(wl_shm::Format),
    #[error("Error accessing the buffer ({0:?})")]
    #[cfg(feature = "wayland_frontend")]
    BufferAccessError(crate::wayland::shm::BufferAccessError),
    #[error("Error accessing the buffer ({0:?})")]
    #[cfg(feature = "wayland_frontend")]
    EGLBufferAccessError(crate::backend::egl::BufferAccessError),
    #[error("Error mapping the buffer")]
    MappingError,
    #[error("Error reading buffer, size is too small for the given dimensions")]
    UnexpectedSize,
    #[error("Error determining the size of the provided framebuffer")]
    UnknownSize,
    #[error("Error blitting between framebuffers")]
    BlitError,
    #[error("An error occured while creating the shader object.")]
    CreateShaderObject,
    #[error("Uniform {0:?} was not declared when compiling the provided shader")]
    UnknownUniform(String),
    #[error("Uniform with different type (got {provided:?}, expected: {declared:?})")]
    UniformTypeMismatch {
        provided: UniformType,
        declared: UniformType,
    },
    #[error("Blocking for a synchronization primitive got interrupted")]
    SyncInterrupted,
}

impl From<GlesError> for SwapBuffersError {
    #[cfg(feature = "wayland_frontend")]
    #[inline]
    fn from(err: GlesError) -> SwapBuffersError {
        match err {
            x @ GlesError::ShaderCompileError
            | x @ GlesError::ProgramLinkError
            | x @ GlesError::GLFunctionLoaderError
            | x @ GlesError::GLExtensionNotSupported(_)
            | x @ GlesError::EGLExtensionNotSupported(_)
            | x @ GlesError::GLVersionNotSupported(_) => SwapBuffersError::ContextLost(Box::new(x)),
            GlesError::ContextActivationError(err) => err.into(),
            x @ GlesError::FramebufferBindingError
            | x @ GlesError::BindBufferEGLError(_)
            | x @ GlesError::UnknownPixelFormat
            | x @ GlesError::UnsupportedPixelFormat(_)
            | x @ GlesError::UnsupportedWlPixelFormat(_)
            | x @ GlesError::UnsupportedPixelLayout
            | x @ GlesError::BufferAccessError(_)
            | x @ GlesError::MappingError
            | x @ GlesError::UnexpectedSize
            | x @ GlesError::UnknownSize
            | x @ GlesError::BlitError
            | x @ GlesError::CreateShaderObject
            | x @ GlesError::UniformTypeMismatch { .. }
            | x @ GlesError::UnknownUniform(_)
            | x @ GlesError::EGLBufferAccessError(_)
            | x @ GlesError::SyncInterrupted => SwapBuffersError::TemporaryFailure(Box::new(x)),
        }
    }
    #[cfg(not(feature = "wayland_frontend"))]
    #[inline]
    fn from(err: GlesError) -> SwapBuffersError {
        match err {
            x @ GlesError::ShaderCompileError
            | x @ GlesError::ProgramLinkError
            | x @ GlesError::GLFunctionLoaderError
            | x @ GlesError::GLExtensionNotSupported(_)
            | x @ GlesError::EGLExtensionNotSupported(_)
            | x @ GlesError::GLVersionNotSupported(_) => SwapBuffersError::ContextLost(Box::new(x)),
            GlesError::ContextActivationError(err) => err.into(),
            x @ GlesError::FramebufferBindingError
            | x @ GlesError::MappingError
            | x @ GlesError::UnknownPixelFormat
            | x @ GlesError::UnsupportedPixelFormat(_)
            | x @ GlesError::UnsupportedPixelLayout
            | x @ GlesError::UnexpectedSize
            | x @ GlesError::UnknownSize
            | x @ GlesError::BlitError
            | x @ GlesError::CreateShaderObject
            | x @ GlesError::UniformTypeMismatch { .. }
            | x @ GlesError::UnknownUniform(_)
            | x @ GlesError::BindBufferEGLError(_)
            | x @ GlesError::SyncInterrupted => SwapBuffersError::TemporaryFailure(Box::new(x)),
        }
    }
}
