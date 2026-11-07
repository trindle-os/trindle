use std::{ffi::CStr, os::raw::c_void, path::PathBuf, ptr};

use super::{
    ffi::{self, egl::types::EGLDeviceEXT},
    wrap_egl_call_bool, wrap_egl_call_ptr, EGLDisplay, EGLError, Error,
};

#[cfg(feature = "backend_drm")]
use crate::backend::drm::{DrmNode, NodeType};

#[derive(Debug, Clone)]
pub struct EGLDevice {
    pub(super) inner: EGLDeviceEXT,
    device_extensions: Vec<String>,
}

unsafe impl Send for EGLDevice {}

impl EGLDevice {
    pub fn enumerate() -> Result<impl Iterator<Item = EGLDevice>, Error> {
        let extensions = ffi::make_sure_egl_is_loaded()?;

        if !extensions.iter().any(|s| s == "EGL_EXT_device_base") {
            return Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_base"]));
        }

        if !extensions.iter().any(|s| s == "EGL_EXT_device_enumeration") {
            return Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_enumeration"]));
        }

        if !extensions.iter().any(|s| s == "EGL_EXT_device_query") {
            return Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_query"]));
        }

        let mut device_amount = 0;
        wrap_egl_call_bool(|| unsafe {
            ffi::egl::QueryDevicesEXT(0, ptr::null_mut(), &mut device_amount)
        })
        .map_err(Error::QueryDevices)?;

        let mut devices = Vec::with_capacity(device_amount as usize);

        wrap_egl_call_bool(|| unsafe {
            // SAFETY:
            // - Vector used as pointer is correct size.
            // - Device amount will accommodate all available devices because we have checked the size earlier.
            ffi::egl::QueryDevicesEXT(device_amount, devices.as_mut_ptr(), &mut device_amount)
        })
        .map_err(Error::QueryDevices)?;

        // SAFETY:
        // 1) the vector is pre-allocated to the same size as the amount of returned devices.
        // 2) EGL has initialized every value in the vector.
        unsafe { devices.set_len(device_amount as usize) };

        Ok(devices
            .into_iter()
            .map(|device| {
                // SAFETY: We have queried that the extensions are valid and the device pointer is valid.
                let device_extensions = unsafe { device_extensions(device) }?;
                Ok(EGLDevice {
                    inner: device,
                    device_extensions,
                })
            })
            .collect::<Result<Vec<_>, EGLError>>()
            .map_err(Error::QueryDevices)?
            .into_iter())
    }

    pub fn device_for_display(display: &EGLDisplay) -> Result<EGLDevice, Error> {
        let extensions = ffi::make_sure_egl_is_loaded()?;

        if !extensions.iter().any(|s| s == "EGL_EXT_device_base") {
            return Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_base"]));
        }

        if !extensions.iter().any(|s| s == "EGL_EXT_device_query") {
            return Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_query"]));
        }

        let mut device: ffi::egl::types::EGLAttrib = 0;
        if unsafe {
            ffi::egl::QueryDisplayAttribEXT(
                display.get_display_handle().handle,
                ffi::egl::DEVICE_EXT as i32,
                &mut device as *mut _,
            )
        } != ffi::egl::TRUE
        {
            return Err(Error::DisplayNotSupported);
        }

        let device = device as EGLDeviceEXT;

        if device == ffi::egl::NO_DEVICE_EXT {
            return Err(Error::DisplayNotSupported);
        }

        // SAFETY: We have queried that the extensions are valid and the device pointer is valid.
        let device_extensions = unsafe { device_extensions(device) }.map_err(Error::QueryDevices)?;
        Ok(EGLDevice {
            inner: device,
            device_extensions,
        })
    }

    pub fn extensions(&self) -> Vec<String> {
        self.device_extensions.clone()
    }

    pub fn drm_device_path(&self) -> Result<PathBuf, Error> {
        if !self.extensions().contains(&"EGL_EXT_device_drm".to_owned()) {
            Err(Error::EglExtensionNotSupported(&["EGL_EXT_device_drm"]))
        } else {
            let raw_path = wrap_egl_call_ptr(|| unsafe {
                ffi::egl::QueryDeviceStringEXT(
                    self.inner,
                    ffi::egl::DRM_DEVICE_FILE_EXT as ffi::egl::types::EGLint,
                )
            })
            .map_err(Error::QueryDeviceProperty)?;
            if raw_path.is_null() {
                return Err(Error::EmptyDeviceProperty);
            }

            let device_path = unsafe { CStr::from_ptr(raw_path) }
                .to_str()
                .expect("Non-UTF8 device path name");

            Ok(PathBuf::from(device_path))
        }
    }

    pub fn render_device_path(&self) -> Result<PathBuf, Error> {
        if !self
            .extensions()
            .contains(&"EGL_EXT_device_drm_render_node".to_owned())
        {
            Err(Error::EglExtensionNotSupported(&[
                "EGL_EXT_device_drm_render_node",
            ]))
        } else {
            let raw_path = wrap_egl_call_ptr(|| unsafe {
                ffi::egl::QueryDeviceStringEXT(
                    self.inner,
                    ffi::egl::DRM_RENDER_NODE_FILE_EXT as ffi::egl::types::EGLint,
                )
            })
            .map_err(Error::QueryDeviceProperty)?;
            if raw_path.is_null() {
                return Err(Error::EmptyDeviceProperty);
            }

            let device_path = unsafe { CStr::from_ptr(raw_path) }
                .to_str()
                .expect("Non-UTF8 device path name");

            Ok(PathBuf::from(device_path))
        }
    }

    #[cfg(feature = "backend_drm")]
    pub fn try_get_render_node(&self) -> Result<Option<DrmNode>, Error> {
        match self
            .render_device_path()
            .ok()
            .and_then(|path| DrmNode::from_path(path).ok())
        {
            Some(node) => Ok(Some(node)),
            None => {
                let path = self.drm_device_path()?;
                let node = DrmNode::from_path(path).ok();
                Ok(node.map(|node| {
                    node.node_with_type(NodeType::Render)
                        .and_then(Result::ok)
                        .unwrap_or(node)
                }))
            }
        }
    }

    pub fn is_software(&self) -> bool {
        self.extensions().contains(&"EGL_MESA_device_software".to_owned())
    }

    pub fn get_device_handle(&self) -> *const c_void {
        self.inner
    }
}

unsafe fn device_extensions(device: EGLDeviceEXT) -> Result<Vec<String>, EGLError> {
    let raw_extensions = wrap_egl_call_ptr(|| {
        ffi::egl::QueryDeviceStringEXT(device, ffi::egl::EXTENSIONS as ffi::egl::types::EGLint)
    })?;

    // SAFETY:
    // 1) The string returned by `eglQueryDeviceStringEXT` is string which will exist as long
    //    as the EGLDisplay is valid. Safety requirements for the function ensure this.
    // 2) The string returned by EGL is null terminated.
    let c_extensions = CStr::from_ptr(raw_extensions);

    Ok(c_extensions
        .to_str()
        .expect("Non-UTF8 device extension name")
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>())
}
