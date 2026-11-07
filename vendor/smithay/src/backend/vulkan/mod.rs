//! Types to initialize and use Vulkan.

#![warn(missing_debug_implementations)]
#![forbid(unsafe_op_in_unsafe_fn)]

use std::{
    env::{self, VarError},
    ffi::{CStr, CString},
    sync::{Arc, LazyLock},
};

use ash::{
    ext,
    prelude::VkResult,
    vk::{self, PhysicalDeviceDriverProperties, PhysicalDeviceDrmPropertiesEXT},
    Entry,
};
use libc::c_void;
use scopeguard::ScopeGuard;
use tracing::{error, info, info_span, instrument, trace, warn};

use crate::backend::vulkan::inner::DebugState;

use self::{inner::InstanceInner, version::Version};

#[cfg(feature = "backend_drm")]
use super::drm::DrmNode;

mod inner;
mod phd;

pub mod version;

static LIBRARY: LazyLock<Result<Entry, LoadError>> =
    LazyLock::new(|| unsafe { Entry::load().map_err(|_| LoadError) });

#[derive(Debug, thiserror::Error)]
#[error("Failed to load the Vulkan library")]
pub struct LoadError;

#[derive(Debug, thiserror::Error)]
pub enum InstanceError {
    #[error("Smithay requires at least Vulkan 1.1")]
    UnsupportedVersion,

    #[error(transparent)]
    Load(#[from] LoadError),

    #[error(transparent)]
    Vk(#[from] vk::Result),
}

#[derive(Debug, thiserror::Error)]
pub enum UnsupportedProperty {
    #[error("The following extensions are not available {0:?}")]
    Extensions(&'static [&'static CStr]),
}

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub version: Version,
}

#[derive(Debug, Clone)]
pub struct Instance(Arc<InstanceInner>);

impl Instance {
    pub fn new(max_version: Version, app_info: Option<AppInfo>) -> Result<Instance, InstanceError> {
        unsafe { Self::with_extensions(max_version, app_info, &[]) }
    }

    pub unsafe fn with_extensions(
        max_version: Version,
        app_info: Option<AppInfo>,
        extensions: &[&'static CStr],
    ) -> Result<Instance, InstanceError> {
        assert!(
            max_version >= Version::VERSION_1_1,
            "Smithay requires at least Vulkan 1.1"
        );
        let requested_max_version = get_env_or_max_version(max_version);

        let span = info_span!("backend_vulkan", version = tracing::field::Empty);
        let _guard = span.enter();

        let max_version = {
            unsafe {
                LIBRARY
                    .as_ref()
                    .or(Err(LoadError))?
                    .try_enumerate_instance_version()
            }
            .or(Err(LoadError))?
            .map(Version::from_raw)
            .unwrap_or(Version::VERSION_1_0)
        };

        if max_version == Version::VERSION_1_0 {
            error!("Vulkan does not support version 1.1");
            return Err(InstanceError::UnsupportedVersion);
        }

        let api_version = Version::from_raw(u32::min(max_version.to_raw(), requested_max_version.to_raw()));
        span.record("version", tracing::field::display(api_version));

        let available_layers = Self::enumerate_layers()?.collect::<Vec<_>>();
        let available_extensions = Self::enumerate_extensions()?.collect::<Vec<_>>();

        let mut layers = Vec::new();

        if cfg!(debug_assertions) {
            const VALIDATION: &CStr = c"VK_LAYER_KHRONOS_validation";

            if available_layers
                .iter()
                .any(|layer| layer.as_c_str() == VALIDATION)
            {
                layers.push(VALIDATION);
            } else {
                warn!("Validation layers not available. These can be installed through your package manager",);
            }
        }

        let mut enabled_extensions = Vec::<&'static CStr>::new();
        enabled_extensions.extend(extensions);

        let has_debug_utils = available_extensions
            .iter()
            .any(|name| name.as_c_str() == ext::debug_utils::NAME);

        if has_debug_utils {
            enabled_extensions.push(ext::debug_utils::NAME);
        }

        let extension_pointers = enabled_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();
        let layer_pointers = layers.iter().map(|name| name.as_ptr()).collect::<Vec<_>>();

        let app_version = app_info.as_ref().map(|info| info.version.to_raw());
        let app_name =
            app_info.map(|info| CString::new(info.name).expect("app name contains null terminator"));
        let mut app_info = vk::ApplicationInfo::default()
            .api_version(api_version.to_raw())
            // SAFETY: null terminated with no interior null bytes.
            .engine_name(c"Smithay")
            .engine_version(Version::SMITHAY.to_raw());

        if let Some(app_version) = app_version {
            app_info = app_info.application_version(app_version);
        }

        if let Some(app_name) = &app_name {
            app_info = app_info.application_name(app_name);
        }

        let library = LIBRARY.as_ref().map_err(|_| LoadError)?;
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layer_pointers)
            .enabled_extension_names(&extension_pointers);

        let instance = scopeguard::guard(
            unsafe { library.create_instance(&create_info, None) }?,
            |instance| unsafe {
                instance.destroy_instance(None);
            },
        );

        let debug_state = if has_debug_utils {
            let span = info_span!("backend_vulkan_debug");
            let debug_utils = ext::debug_utils::Instance::new(library, &instance);
            let span_ptr = scopeguard::guard(Box::into_raw(Box::new(span)), |ptr| unsafe {
                let _ = Box::from_raw(ptr);
            });

            let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
                )
                .pfn_user_callback(Some(vulkan_debug_utils_callback))
                .user_data(*span_ptr as *mut _);

            let debug_messenger = unsafe { debug_utils.create_debug_utils_messenger(&create_info, None) }?;

            let span_ptr = ScopeGuard::into_inner(span_ptr);

            Some(DebugState {
                debug_utils,
                debug_messenger,
                span_ptr,
            })
        } else {
            None
        };

        let instance = ScopeGuard::into_inner(instance);
        drop(_guard);
        let inner = InstanceInner {
            instance,
            version: api_version,
            debug_state,
            span,
            enabled_extensions,
        };

        info!("Created new instance");
        info!("Enabled instance extensions: {:?}", inner.enabled_extensions);

        #[allow(clippy::arc_with_non_send_sync)]
        Ok(Instance(Arc::new(inner)))
    }

    pub fn enumerate_extensions() -> Result<impl Iterator<Item = CString>, LoadError> {
        let library = LIBRARY.as_ref().or(Err(LoadError))?;

        let extensions = unsafe { library.enumerate_instance_extension_properties(None) }
            .or(Err(LoadError))?
            .into_iter()
            .map(|properties| {
                // SAFETY: Vulkan guarantees the string is null terminated.
                unsafe { CStr::from_ptr(&properties.extension_name as *const _) }.to_owned()
            })
            .collect::<Vec<_>>()
            .into_iter();

        Ok(extensions)
    }

    pub fn enabled_extensions(&self) -> impl Iterator<Item = &CStr> {
        self.0.enabled_extensions.iter().copied()
    }

    pub fn is_extension_enabled(&self, extension: &CStr) -> bool {
        self.enabled_extensions().any(|name| name == extension)
    }

    pub fn api_version(&self) -> Version {
        self.0.version
    }

    pub fn handle(&self) -> &ash::Instance {
        &self.0.instance
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    phd: vk::PhysicalDevice,
    info: PhdInfo,
    extensions: Vec<CString>,
    instance: Instance,
    span: tracing::Span,
}

impl PhysicalDevice {
    pub fn enumerate(instance: &Instance) -> VkResult<impl Iterator<Item = PhysicalDevice>> {
        let _span = instance.0.span.enter();

        let instance = instance.clone();
        let devices = unsafe { instance.handle().enumerate_physical_devices() }?;
        let devices = devices
            .into_iter()
            .flat_map(move |phd| unsafe { PhysicalDevice::from_phd(&instance, phd) })
            .flatten();

        Ok(devices)
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn api_version(&self) -> Version {
        self.info.api_version
    }

    pub fn ty(&self) -> vk::PhysicalDeviceType {
        self.info.properties.device_type
    }

    pub fn features(&self) -> vk::PhysicalDeviceFeatures {
        self.info.features
    }

    pub fn properties(&self) -> vk::PhysicalDeviceProperties {
        self.info.properties
    }

    pub fn properties_maintenance_3(&self) -> vk::PhysicalDeviceMaintenance3Properties<'_> {
        self.info.maintenance_3
    }

    pub fn id_properties(&self) -> vk::PhysicalDeviceIDProperties<'_> {
        self.info.id
    }

    pub fn limits(&self) -> vk::PhysicalDeviceLimits {
        self.info.properties.limits
    }

    pub fn driver(&self) -> Option<&DriverInfo> {
        self.info.driver.as_ref()
    }

    #[cfg(feature = "backend_drm")]
    #[instrument(level = "debug", parent = &self.span, skip(self))]
    pub fn primary_node(&self) -> Result<Option<DrmNode>, UnsupportedProperty> {
        let properties_drm = self.info.get_drm_properties()?;
        let node = Some(properties_drm)
            .filter(|props| props.has_primary == vk::TRUE)
            .and_then(|props| {
                DrmNode::from_dev_id(libc::makedev(props.primary_major as _, props.primary_minor as _)).ok()
            });

        Ok(node)
    }

    #[cfg(feature = "backend_drm")]
    #[instrument(level = "debug", parent = &self.span, skip(self))]
    pub fn render_node(&self) -> Result<Option<DrmNode>, UnsupportedProperty> {
        let properties_drm = self.info.get_drm_properties()?;
        let node = Some(properties_drm)
            .filter(|props| props.has_render == vk::TRUE)
            .and_then(|props| {
                DrmNode::from_dev_id(libc::makedev(props.render_major as _, props.render_minor as _)).ok()
            });

        Ok(node)
    }

    pub unsafe fn get_properties(&self, props: &mut vk::PhysicalDeviceProperties2<'_>) {
        let instance = self.instance().handle();
        // SAFETY: The caller has garunteed all valid usage requirements for vkGetPhysicalDeviceProperties2
        // are satisfied.
        unsafe { instance.get_physical_device_properties2(self.handle(), props) }
    }

    pub unsafe fn get_format_properties(&self, format: vk::Format, props: &mut vk::FormatProperties2<'_>) {
        let instance = self.instance().handle();
        // SAFETY: The caller has garunteed all valid usage requirements for vkGetPhysicalDeviceFormatProperties2
        // are satisfied.
        unsafe { instance.get_physical_device_format_properties2(self.handle(), format, props) }
    }

    #[instrument(level = "debug", parent = &self.span, skip(self))]
    pub fn get_format_modifier_properties(
        &self,
        format: vk::Format,
    ) -> Result<Vec<vk::DrmFormatModifierPropertiesEXT>, UnsupportedProperty> {
        if !self.has_device_extension(ext::image_drm_format_modifier::NAME) {
            const EXTENSIONS: &[&CStr] = &[ext::image_drm_format_modifier::NAME];
            return Err(UnsupportedProperty::Extensions(EXTENSIONS));
        }

        let count = unsafe {
            let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
            let mut format_properties2 = vk::FormatProperties2::default().push_next(&mut list);
            self.get_format_properties(format, &mut format_properties2);
            list.drm_format_modifier_count as usize
        };

        let mut data = Vec::with_capacity(count);

        unsafe {
            let mut list = vk::DrmFormatModifierPropertiesListEXT {
                p_drm_format_modifier_properties: data.as_mut_ptr(),
                drm_format_modifier_count: count as u32,
                ..Default::default()
            };

            let mut format_properties2 = vk::FormatProperties2::default().push_next(&mut list);
            self.get_format_properties(format, &mut format_properties2);
            // SAFETY: Vulkan just initialized the elements of the vector.
            data.set_len(list.drm_format_modifier_count as usize);
        }

        Ok(data)
    }

    pub fn device_extensions(&self) -> impl Iterator<Item = &CStr> {
        self.extensions.iter().map(CString::as_c_str)
    }

    pub fn has_device_extension(&self, extension: &CStr) -> bool {
        self.device_extensions().any(|name| name == extension)
    }

    pub fn handle(&self) -> vk::PhysicalDevice {
        self.phd
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl PartialEq for PhysicalDevice {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.phd == other.phd && self.instance().handle().handle() == other.instance().handle().handle()
    }
}

// SAFETY: The internal pointers in the PhysicalDevice*Properties are always null and only copies of the
// PhysicalDevice*Properties types are returned.
unsafe impl Send for PhysicalDevice {}
unsafe impl Sync for PhysicalDevice {}

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub id: vk::DriverId,

    pub name: String,

    pub info: String,

    pub conformance: vk::ConformanceVersion,
}

#[derive(Debug, Clone, Default)]
struct PhdInfo {
    api_version: Version,
    name: String,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    maintenance_3: vk::PhysicalDeviceMaintenance3Properties<'static>,
    id: vk::PhysicalDeviceIDProperties<'static>,
    properties_driver: Option<PhysicalDeviceDriverProperties<'static>>,
    #[cfg_attr(not(feature = "backend_drm"), allow(dead_code))]
    properties_drm: Option<PhysicalDeviceDrmPropertiesEXT<'static>>,
    driver: Option<DriverInfo>,
}

fn get_env_or_max_version(max_version: Version) -> Version {
    match env::var("SMITHAY_VK_VERSION") {
        Ok(version) => {
            let overriden_version = match &version[..] {
                "1.0" => {
                    warn!("Smithay does not support Vulkan 1.0, ignoring SMITHAY_VK_VERSION");
                    return max_version;
                }
                "1.1" => Some(Version::VERSION_1_1),
                "1.2" => Some(Version::VERSION_1_2),
                "1.3" => Some(Version::VERSION_1_3),
                _ => None,
            };

            if let Some(overridden_version) = overriden_version {
                if overridden_version > max_version {
                    warn!(
                        "Ignoring SMITHAY_VK_VERSION since the requested max version is higher than the maximum of {}.{}",
                        max_version.major,
                        max_version.minor
                    );
                    max_version
                } else {
                    overridden_version
                }
            } else {
                warn!("SMITHAY_VK_VERSION was set to an unknown Vulkan version");
                max_version
            }
        }

        Err(VarError::NotUnicode(_)) => {
            warn!("Value of SMITHAY_VK_VERSION is not valid Unicode, ignoring.");

            max_version
        }

        Err(VarError::NotPresent) => max_version,
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    span: *mut c_void,
) -> vk::Bool32 {
    let _ = std::panic::catch_unwind(|| {
        let _guard = unsafe { (span as *mut tracing::Span).as_ref() }.unwrap().enter();

        let message = unsafe { CStr::from_ptr((*p_callback_data).p_message) }.to_string_lossy();
        let ty = format!("{:?}", message_type).to_lowercase();

        match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
                trace!(ty, "{}", message)
            }
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => info!(ty, "{}", message),
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => warn!(ty, "{}", message),
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => error!(ty, "{}", message),
            _ => (),
        }
    });

    vk::FALSE
}

#[cfg(test)]
mod tests {
    use super::{Instance, PhysicalDevice};

    fn is_send_sync<T: Send + Sync>() {}

    #[test]
    fn send_sync() {
        is_send_sync::<Instance>();
        is_send_sync::<PhysicalDevice>();
    }
}
