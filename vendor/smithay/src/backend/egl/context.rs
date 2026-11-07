//! EGL context related structs.

use std::{
    os::raw::c_int,
    sync::{atomic::Ordering, Arc},
};

use libc::c_void;

use super::{ffi, wrap_egl_call_bool, wrap_egl_call_ptr, EGLError, Error, MakeCurrentError};
use crate::{
    backend::{
        allocator::format::FormatSet,
        egl::{
            display::{EGLDisplay, PixelFormat},
            EGLSurface,
        },
    },
    utils::user_data::UserDataMap,
};

use tracing::{info, info_span, instrument, trace, warn};

#[derive(Debug)]
pub struct EGLContext {
    context: ffi::egl::types::EGLContext,
    display: EGLDisplay,
    config_id: ffi::egl::types::EGLConfig,
    pixel_format: Option<PixelFormat>,
    user_data: Arc<UserDataMap>,
    externally_managed: bool,
    pub(crate) span: tracing::Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextPriority {
    High,
    #[default]
    Medium,
    Low,
}

impl TryFrom<ffi::egl::types::EGLenum> for ContextPriority {
    type Error = EGLError;

    #[inline]
    fn try_from(value: ffi::egl::types::EGLenum) -> Result<Self, Self::Error> {
        let priority = match value {
            ffi::egl::CONTEXT_PRIORITY_HIGH_IMG => ContextPriority::High,
            ffi::egl::CONTEXT_PRIORITY_MEDIUM_IMG => ContextPriority::Medium,
            ffi::egl::CONTEXT_PRIORITY_LOW_IMG => ContextPriority::Low,
            _ => return Err(EGLError::BadParameter),
        };
        Ok(priority)
    }
}

impl From<ContextPriority> for ffi::egl::types::EGLenum {
    #[inline]
    fn from(value: ContextPriority) -> Self {
        match value {
            ContextPriority::High => ffi::egl::CONTEXT_PRIORITY_HIGH_IMG,
            ContextPriority::Medium => ffi::egl::CONTEXT_PRIORITY_MEDIUM_IMG,
            ContextPriority::Low => ffi::egl::CONTEXT_PRIORITY_LOW_IMG,
        }
    }
}

// SAFETY: A context can be sent to another thread when this context is not current.
//
// If the context is made current, the safety requirements for calling `EGLDisplay::make_current` must be
// upheld. This means a context which is made current must be unbound (using `unbind`) in order to send this
// context to another thread.
unsafe impl Send for EGLContext {}

impl EGLContext {
    pub unsafe fn from_raw(
        display: *const c_void,
        config_id: *const c_void,
        context: *const c_void,
    ) -> Result<EGLContext, Error> {
        assert!(!display.is_null(), "EGLDisplay pointer is null");
        assert!(!config_id.is_null(), "EGL configuration id pointer is null");
        assert!(!context.is_null(), "EGLContext pointer is null");

        let display = EGLDisplay::from_raw(display, config_id)?;
        let pixel_format = display.get_pixel_format(config_id)?;

        let span = info_span!(parent: &display.span, "egl_context", ptr = context as usize);

        Ok(EGLContext {
            context,
            display,
            config_id,
            pixel_format: Some(pixel_format),
            user_data: Arc::new(UserDataMap::default()),
            externally_managed: true,
            span,
        })
    }

    pub fn new(display: &EGLDisplay) -> Result<EGLContext, Error> {
        Self::new_internal(display, None, None, None)
    }

    pub fn new_with_priority(display: &EGLDisplay, priority: ContextPriority) -> Result<EGLContext, Error> {
        Self::new_internal(display, None, None, Some(priority))
    }

    pub fn new_with_config(
        display: &EGLDisplay,
        attributes: GlAttributes,
        reqs: PixelFormatRequirements,
    ) -> Result<EGLContext, Error> {
        Self::new_internal(display, None, Some((attributes, reqs)), None)
    }

    pub fn new_with_config_and_priority(
        display: &EGLDisplay,
        attributes: GlAttributes,
        reqs: PixelFormatRequirements,
        priority: ContextPriority,
    ) -> Result<EGLContext, Error> {
        Self::new_internal(display, None, Some((attributes, reqs)), Some(priority))
    }

    pub fn new_shared(display: &EGLDisplay, share: &EGLContext) -> Result<EGLContext, Error> {
        Self::new_internal(display, Some(share), None, None)
    }

    pub fn new_shared_with_priority(
        display: &EGLDisplay,
        share: &EGLContext,
        priority: ContextPriority,
    ) -> Result<EGLContext, Error> {
        Self::new_internal(display, Some(share), None, Some(priority))
    }

    pub fn new_shared_with_config(
        display: &EGLDisplay,
        share: &EGLContext,
        attributes: GlAttributes,
        reqs: PixelFormatRequirements,
    ) -> Result<EGLContext, Error> {
        Self::new_internal(display, Some(share), Some((attributes, reqs)), None)
    }

    pub fn new_shared_with_config_and_priority(
        display: &EGLDisplay,
        share: &EGLContext,
        attributes: GlAttributes,
        reqs: PixelFormatRequirements,
        priority: ContextPriority,
    ) -> Result<EGLContext, Error> {
        Self::new_internal(display, Some(share), Some((attributes, reqs)), Some(priority))
    }

    fn new_internal(
        display: &EGLDisplay,
        shared: Option<&EGLContext>,
        config: Option<(GlAttributes, PixelFormatRequirements)>,
        priority: Option<ContextPriority>,
    ) -> Result<EGLContext, Error> {
        let span = info_span!(parent: &display.span, "egl_context", ptr = tracing::field::Empty, shared = tracing::field::Empty);
        let _guard = span.enter();

        if let Some(shared) = shared {
            span.record("shared", shared.context as usize);
        }

        let (pixel_format, config_id) = match config {
            Some((attributes, reqs)) => {
                let (format, config_id) = display.choose_config(attributes, reqs)?;
                (Some(format), config_id)
            }
            None => {
                if !display
                    .extensions()
                    .iter()
                    .any(|x| x == "EGL_KHR_no_config_context")
                    && !display
                        .extensions()
                        .iter()
                        .any(|x| x == "EGL_MESA_configless_context")
                    && !display
                        .extensions()
                        .iter()
                        .any(|x| x == "EGL_KHR_surfaceless_context")
                {
                    return Err(Error::EglExtensionNotSupported(&[
                        "EGL_KHR_no_config_context",
                        "EGL_MESA_configless_context",
                        "EGL_KHR_surfaceless_context",
                    ]));
                }
                (None, ffi::egl::NO_CONFIG_KHR)
            }
        };

        let mut context_attributes = Vec::with_capacity(12);

        if let Some((attributes, _)) = config {
            let version = attributes.version;

            if display.get_egl_version() >= (1, 5)
                || display.extensions().iter().any(|s| s == "EGL_KHR_create_context")
            {
                trace!("Setting CONTEXT_MAJOR_VERSION to {}", version.0);
                context_attributes.push(ffi::egl::CONTEXT_MAJOR_VERSION as i32);
                context_attributes.push(version.0 as i32);
                trace!("Setting CONTEXT_MINOR_VERSION to {}", version.1);
                context_attributes.push(ffi::egl::CONTEXT_MINOR_VERSION as i32);
                context_attributes.push(version.1 as i32);

                if attributes.debug && display.get_egl_version() >= (1, 5) {
                    trace!("Setting CONTEXT_OPENGL_DEBUG to TRUE");
                    context_attributes.push(ffi::egl::CONTEXT_OPENGL_DEBUG as i32);
                    context_attributes.push(ffi::egl::TRUE as i32);
                }

                context_attributes.push(ffi::egl::CONTEXT_FLAGS_KHR as i32);
                context_attributes.push(0);
            } else if display.get_egl_version() >= (1, 3) {
                trace!("Setting CONTEXT_CLIENT_VERSION to {}", version.0);
                context_attributes.push(ffi::egl::CONTEXT_CLIENT_VERSION as i32);
                context_attributes.push(version.0 as i32);
            }
        } else {
            trace!("Setting CONTEXT_CLIENT_VERSION to 2");
            context_attributes.push(ffi::egl::CONTEXT_CLIENT_VERSION as i32);
            context_attributes.push(2);
        }

        let has_context_priority = display
            .extensions()
            .iter()
            .any(|x| x == "EGL_IMG_context_priority");
        if let Some(priority) = priority {
            if !has_context_priority {
                warn!(
                    ?priority,
                    "ignoring requested context priority, EGL_IMG_context_priority not supported"
                );
            } else {
                context_attributes.push(ffi::egl::CONTEXT_PRIORITY_LEVEL_IMG as i32);
                context_attributes.push(Into::<ffi::egl::types::EGLenum>::into(priority) as i32);
            }
        }

        context_attributes.push(ffi::egl::NONE as i32);

        trace!("Creating EGL context...");
        let context = wrap_egl_call_ptr(|| unsafe {
            ffi::egl::CreateContext(
                **display.get_display_handle(),
                config_id,
                shared
                    .map(|context| context.context)
                    .unwrap_or(ffi::egl::NO_CONTEXT),
                context_attributes.as_ptr(),
            )
        })
        .map_err(Error::CreationFailed)?;
        span.record("ptr", context as usize);

        let context_priority = if has_context_priority {
            let mut context_priority = 0;
            let res = wrap_egl_call_bool(|| unsafe {
                ffi::egl::QueryContext(
                    **display.get_display_handle(),
                    context,
                    ffi::egl::CONTEXT_PRIORITY_LEVEL_IMG as ffi::egl::types::EGLint,
                    &mut context_priority,
                )
            });

            if res.is_ok() {
                match ContextPriority::try_from(context_priority as ffi::egl::types::EGLenum) {
                    Ok(context_priority) => Some(context_priority),
                    Err(_) => {
                        warn!(context_priority, "failed to parse context priority");
                        None
                    }
                }
            } else {
                warn!("failed to query context priority");
                None
            }
        } else {
            None
        };

        if priority.is_some() && has_context_priority && priority != context_priority {
            warn!(requested = ?priority, got = ?context_priority, "failed to set context priority");
        }

        info!(priority = ?context_priority, "EGL context created");

        drop(_guard);
        Ok(EGLContext {
            context,
            display: display.clone(),
            config_id,
            pixel_format,
            user_data: if let Some(shared) = shared {
                shared.user_data.clone()
            } else {
                Arc::new(UserDataMap::default())
            },
            externally_managed: false,
            span,
        })
    }

    #[instrument(level = "trace", skip_all, parent = &self.span, err)]
    #[profiling::function]
    pub unsafe fn make_current(&self) -> Result<(), MakeCurrentError> {
        wrap_egl_call_bool(|| {
            ffi::egl::MakeCurrent(
                **self.display.get_display_handle(),
                ffi::egl::NO_SURFACE,
                ffi::egl::NO_SURFACE,
                self.context,
            )
        })
        .map(|_| ())
        .map_err(Into::into)
    }

    #[profiling::function]
    pub unsafe fn make_current_with_surface(&self, surface: &EGLSurface) -> Result<(), MakeCurrentError> {
        self.make_current_with_draw_and_read_surface(surface, surface)
    }

    #[instrument(level = "trace", skip_all, parent = &self.span, err)]
    #[profiling::function]
    pub unsafe fn make_current_with_draw_and_read_surface(
        &self,
        draw_surface: &EGLSurface,
        read_surface: &EGLSurface,
    ) -> Result<(), MakeCurrentError> {
        let draw_surface_ptr = draw_surface.surface.load(Ordering::SeqCst);
        let read_surface_ptr = read_surface.surface.load(Ordering::SeqCst);
        wrap_egl_call_bool(|| {
            ffi::egl::MakeCurrent(
                **self.display.get_display_handle(),
                draw_surface_ptr,
                read_surface_ptr,
                self.context,
            )
        })
        .map(|_| ())
        .map_err(Into::into)
    }

    pub fn is_current(&self) -> bool {
        unsafe { std::ptr::eq(ffi::egl::GetCurrentContext(), self.context as *const _) }
    }

    pub fn is_shared(&self) -> bool {
        self.externally_managed || Arc::strong_count(&self.user_data) > 1
    }

    pub fn config_id(&self) -> ffi::egl::types::EGLConfig {
        self.config_id
    }

    pub fn pixel_format(&self) -> Option<PixelFormat> {
        self.pixel_format
    }

    #[instrument(level = "trace", skip_all, parent = &self.span, err)]
    #[profiling::function]
    pub fn unbind(&self) -> Result<(), MakeCurrentError> {
        if self.is_current() {
            wrap_egl_call_bool(|| unsafe {
                ffi::egl::MakeCurrent(
                    **self.display.get_display_handle(),
                    ffi::egl::NO_SURFACE,
                    ffi::egl::NO_SURFACE,
                    ffi::egl::NO_CONTEXT,
                )
            })?;
        }
        Ok(())
    }

    pub fn display(&self) -> &EGLDisplay {
        &self.display
    }

    pub fn dmabuf_render_formats(&self) -> &FormatSet {
        self.display.dmabuf_render_formats()
    }

    pub fn dmabuf_texture_formats(&self) -> &FormatSet {
        self.display.dmabuf_texture_formats()
    }

    pub fn user_data(&self) -> &UserDataMap {
        &self.user_data
    }

    pub fn get_context_handle(&self) -> ffi::egl::types::EGLContext {
        self.context
    }
}

impl Drop for EGLContext {
    fn drop(&mut self) {
        if !self.externally_managed {
            let _guard = self.span.enter();
            unsafe {
                let _ = self.unbind();
                ffi::egl::DestroyContext(**self.display.get_display_handle(), self.context);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlAttributes {
    pub version: (u8, u8),
    pub profile: Option<GlProfile>,
    pub debug: bool,
    pub vsync: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlProfile {
    Compatibility,
    Core,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelFormatRequirements {
    pub hardware_accelerated: Option<bool>,
    pub color_bits: Option<u8>,
    pub float_color_buffer: bool,
    pub alpha_bits: Option<u8>,
    pub depth_bits: Option<u8>,
    pub stencil_bits: Option<u8>,
    pub multisampling: Option<u16>,
}

impl PixelFormatRequirements {
    pub fn _8_bit() -> Self {
        PixelFormatRequirements {
            hardware_accelerated: Some(true),
            color_bits: Some(24),
            float_color_buffer: false,
            alpha_bits: Some(8),
            depth_bits: Some(24),
            stencil_bits: Some(8),
            multisampling: None,
        }
    }

    pub fn _10_bit() -> Self {
        PixelFormatRequirements {
            hardware_accelerated: Some(true),
            color_bits: Some(30),
            float_color_buffer: false,
            alpha_bits: Some(2),
            depth_bits: Some(24),
            stencil_bits: Some(8),
            multisampling: None,
        }
    }

    pub fn _10f_bit() -> Self {
        PixelFormatRequirements {
            hardware_accelerated: Some(true),
            color_bits: Some(48),
            float_color_buffer: true,
            alpha_bits: Some(16),
            depth_bits: Some(24),
            stencil_bits: Some(8),
            multisampling: None,
        }
    }
}

impl PixelFormatRequirements {
    pub fn create_attributes(&self, out: &mut Vec<c_int>) {
        if let Some(hardware_accelerated) = self.hardware_accelerated {
            out.push(ffi::egl::CONFIG_CAVEAT as c_int);
            out.push(if hardware_accelerated {
                trace!("Setting CONFIG_CAVEAT to NONE");
                ffi::egl::NONE as c_int
            } else {
                trace!("Setting CONFIG_CAVEAT to SLOW_CONFIG");
                ffi::egl::SLOW_CONFIG as c_int
            });
        }

        if let Some(color) = self.color_bits {
            trace!("Setting RED_SIZE to {}", color / 3);
            out.push(ffi::egl::RED_SIZE as c_int);
            out.push((color / 3) as c_int);
            trace!("Setting GREEN_SIZE to {}", color / 3 + u8::from(color % 3 != 0));
            out.push(ffi::egl::GREEN_SIZE as c_int);
            out.push((color / 3 + u8::from(color % 3 != 0)) as c_int);
            trace!("Setting BLUE_SIZE to {}", color / 3 + u8::from(color % 3 == 2));
            out.push(ffi::egl::BLUE_SIZE as c_int);
            out.push((color / 3 + u8::from(color % 3 == 2)) as c_int);
        }

        if self.float_color_buffer {
            out.push(ffi::egl::COLOR_COMPONENT_TYPE_EXT as c_int);
            out.push(ffi::egl::COLOR_COMPONENT_TYPE_FLOAT_EXT as c_int);
        }

        if let Some(alpha) = self.alpha_bits {
            trace!("Setting ALPHA_SIZE to {}", alpha);
            out.push(ffi::egl::ALPHA_SIZE as c_int);
            out.push(alpha as c_int);
        }

        if let Some(depth) = self.depth_bits {
            trace!("Setting DEPTH_SIZE to {}", depth);
            out.push(ffi::egl::DEPTH_SIZE as c_int);
            out.push(depth as c_int);
        }

        if let Some(stencil) = self.stencil_bits {
            trace!("Setting STENCIL_SIZE to {}", stencil);
            out.push(ffi::egl::STENCIL_SIZE as c_int);
            out.push(stencil as c_int);
        }

        if let Some(multisampling) = self.multisampling {
            trace!("Setting SAMPLES to {}", multisampling);
            out.push(ffi::egl::SAMPLES as c_int);
            out.push(multisampling as c_int);
        }
    }
}
