//! This module represents abstraction on top the linux direct rendering manager api (drm).

#[cfg(all(feature = "wayland_frontend", feature = "backend_gbm"))]
pub mod compositor;
pub(crate) mod device;
#[cfg(feature = "backend_drm")]
pub mod dumb;
mod error;
pub mod exporter;
#[cfg(feature = "backend_gbm")]
pub mod gbm;
#[cfg(all(feature = "wayland_frontend", feature = "backend_gbm"))]
pub mod output;

mod surface;

use std::sync::Once;

use crate::utils::{DevPath, Physical, Size};
pub use device::{
    DrmDevice, DrmDeviceFd, DrmDeviceNotifier, DrmEvent, EventMetadata as DrmEventMetadata, PlaneClaim,
    Time as DrmEventTime, WeakDrmDeviceFd,
};
pub use drm::node::{CreateDrmNodeError, DrmNode, NodeType};
use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
pub use error::AccessError as DrmAccessError;
pub use error::Error as DrmError;
use indexmap::IndexSet;
#[cfg(feature = "backend_gbm")]
pub use surface::gbm::{Error as GbmBufferedSurfaceError, GbmBufferedSurface};
pub use surface::{DrmSurface, PlaneConfig, PlaneDamageClips, PlaneState, VrrSupport};

use drm::{
    control::{crtc, framebuffer, plane, Device as ControlDevice, PlaneType},
    DriverCapability,
};
use tracing::trace;

use self::error::AccessError;

use super::allocator::format::FormatSet;

fn warn_legacy_fb_export() {
    static WARN_LEGACY_FB_EXPORT: Once = Once::new();
    WARN_LEGACY_FB_EXPORT.call_once(|| {
        tracing::warn!("using legacy fbadd");
    });
}

pub trait Framebuffer: AsRef<framebuffer::Handle> {
    fn format(&self) -> drm_fourcc::DrmFormat;
}

#[derive(Debug, Clone)]
pub struct Planes {
    pub primary: Vec<PlaneInfo>,
    pub cursor: Vec<PlaneInfo>,
    pub overlay: Vec<PlaneInfo>,
}

#[derive(Debug, Clone)]
pub struct PlaneInfo {
    pub handle: plane::Handle,
    pub type_: PlaneType,
    pub zpos: Option<i32>,
    pub formats: FormatSet,
    pub size_hints: Option<Vec<Size<u16, Physical>>>,
}

static CURSOR_HOTSPOT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn set_cursor_hotspot(x: i32, y: i32) {
    CURSOR_HOTSPOT.store(((x.max(0) as u64) << 32) | y.max(0) as u64, std::sync::atomic::Ordering::Relaxed);
}

pub(crate) fn cursor_hotspot() -> (u64, u64) {
    let v = CURSOR_HOTSPOT.load(std::sync::atomic::Ordering::Relaxed);
    (v >> 32, v & 0xffff_ffff)
}

fn planes(
    dev: &(impl DevPath + ControlDevice),
    crtc: &crtc::Handle,
    has_universal_planes: bool,
) -> Result<Planes, DrmError> {
    let mut primary = Vec::with_capacity(1);
    let mut cursor = Vec::new();
    let mut overlay = Vec::new();

    let planes = dev.plane_handles().map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Error loading plane handles",
            dev: dev.dev_path(),
            source,
        })
    })?;

    let resources = dev.resource_handles().map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Error loading resource handles",
            dev: dev.dev_path(),
            source,
        })
    })?;

    for plane in planes {
        let info = dev.get_plane(plane).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to get plane information",
                dev: dev.dev_path(),
                source,
            })
        })?;
        let filter = info.possible_crtcs();
        if resources.filter_crtcs(filter).contains(crtc) {
            let zpos = plane_zpos(dev, plane).ok().flatten();
            let type_ = plane_type(dev, plane)?;
            let formats = plane_formats(dev, plane)?;
            let size_hints = plane_size_hints(dev, plane)?;
            let plane_info = PlaneInfo {
                handle: plane,
                type_,
                zpos,
                formats,
                size_hints,
            };
            match type_ {
                PlaneType::Primary => {
                    primary.push(plane_info);
                }
                PlaneType::Cursor => {
                    cursor.push(plane_info);
                }
                PlaneType::Overlay => {
                    overlay.push(plane_info);
                }
            };
        }
    }

    Ok(Planes {
        primary,
        cursor: if has_universal_planes { cursor } else { Vec::new() },
        overlay: if has_universal_planes { overlay } else { Vec::new() },
    })
}

fn plane_type(dev: &(impl ControlDevice + DevPath), plane: plane::Handle) -> Result<PlaneType, DrmError> {
    let props = dev.get_properties(plane).map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Failed to get properties of plane",
            dev: dev.dev_path(),
            source,
        })
    })?;
    let (ids, vals) = props.as_props_and_values();
    for (&id, &val) in ids.iter().zip(vals.iter()) {
        let info = dev.get_property(id).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to get property info",
                dev: dev.dev_path(),
                source,
            })
        })?;
        if info.name().to_str().map(|x| x == "type").unwrap_or(false) {
            return Ok(match val {
                x if x == (PlaneType::Primary as u64) => PlaneType::Primary,
                x if x == (PlaneType::Cursor as u64) => PlaneType::Cursor,
                _ => PlaneType::Overlay,
            });
        }
    }
    unreachable!()
}

fn plane_zpos(dev: &(impl ControlDevice + DevPath), plane: plane::Handle) -> Result<Option<i32>, DrmError> {
    let props = dev.get_properties(plane).map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Failed to get properties of plane",
            dev: dev.dev_path(),
            source,
        })
    })?;
    let (ids, vals) = props.as_props_and_values();
    for (&id, &val) in ids.iter().zip(vals.iter()) {
        let info = dev.get_property(id).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to get property info",
                dev: dev.dev_path(),
                source,
            })
        })?;
        if info.name().to_str().map(|x| x == "zpos").unwrap_or(false) {
            let plane_zpos = match info.value_type().convert_value(val) {
                drm::control::property::Value::UnsignedRange(u) => Some(u as i32),
                drm::control::property::Value::SignedRange(i) => Some(i as i32),
                drm::control::property::Value::Boolean(b) => Some(b.into()),
                _ => None,
            };
            return Ok(plane_zpos);
        }
    }
    Ok(None)
}

fn plane_formats(dev: &(impl ControlDevice + DevPath), plane: plane::Handle) -> Result<FormatSet, DrmError> {
    let plane_info = dev.get_plane(plane).map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Error loading plane info",
            dev: dev.dev_path(),
            source,
        })
    })?;
    let mut formats = IndexSet::new();
    for code in plane_info
        .formats()
        .iter()
        .flat_map(|x| DrmFourcc::try_from(*x).ok())
    {
        formats.insert(DrmFormat {
            code,
            modifier: DrmModifier::Invalid,
        });
    }

    if let Ok(1) = dev.get_driver_capability(DriverCapability::AddFB2Modifiers) {
        let set = dev.get_properties(plane).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to query properties",
                dev: dev.dev_path(),
                source,
            })
        })?;
        let (handles, _) = set.as_props_and_values();
        let prop = handles
            .iter()
            .find(|handle| {
                if let Ok(info) = dev.get_property(**handle) {
                    if info.name().to_str().map(|x| x == "IN_FORMATS").unwrap_or(false) {
                        return true;
                    }
                }
                false
            })
            .copied();
        if let Some(prop) = prop {
            let prop_info = dev.get_property(prop).map_err(|source| {
                DrmError::Access(AccessError {
                    errmsg: "Failed to query property",
                    dev: dev.dev_path(),
                    source,
                })
            })?;
            let (handles, raw_values) = set.as_props_and_values();
            let raw_value = raw_values[handles
                .iter()
                .enumerate()
                .find_map(|(i, handle)| if *handle == prop { Some(i) } else { None })
                .unwrap()];
            if let drm::control::property::Value::Blob(blob) = prop_info.value_type().convert_value(raw_value)
            {
                let data = dev.get_property_blob(blob).map_err(|source| {
                    DrmError::Access(AccessError {
                        errmsg: "Failed to query property blob data",
                        dev: dev.dev_path(),
                        source,
                    })
                })?;
                unsafe {
                    let fmt_mod_blob_ptr = data.as_ptr() as *const drm_ffi::drm_format_modifier_blob;
                    let fmt_mod_blob = &*fmt_mod_blob_ptr;

                    let formats_ptr: *const u32 = fmt_mod_blob_ptr
                        .cast::<u8>()
                        .offset(fmt_mod_blob.formats_offset as isize)
                        as *const _;
                    let modifiers_ptr: *const drm_ffi::drm_format_modifier = fmt_mod_blob_ptr
                        .cast::<u8>()
                        .offset(fmt_mod_blob.modifiers_offset as isize)
                        as *const _;
                    #[allow(clippy::unnecessary_cast)]
                    let formats_ptr = formats_ptr as *const u32;
                    #[allow(clippy::unnecessary_cast)]
                    let modifiers_ptr = modifiers_ptr as *const drm_ffi::drm_format_modifier;

                    for i in 0..fmt_mod_blob.count_modifiers {
                        let mod_info = modifiers_ptr.offset(i as isize).read_unaligned();
                        for j in 0..64 {
                            if mod_info.formats & (1u64 << j) != 0 {
                                let code = DrmFourcc::try_from(
                                    formats_ptr
                                        .offset((j + mod_info.offset) as isize)
                                        .read_unaligned(),
                                )
                                .ok();
                                let modifier = DrmModifier::from(mod_info.modifier);
                                if let Some(code) = code {
                                    formats.insert(DrmFormat { code, modifier });
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if plane_type(dev, plane)? == PlaneType::Cursor {
        for format in formats.clone() {
            formats.insert(DrmFormat {
                code: format.code,
                modifier: DrmModifier::Linear,
            });
        }
    }

    if formats.is_empty() {
        formats.insert(DrmFormat {
            code: DrmFourcc::Argb8888,
            modifier: DrmModifier::Invalid,
        });
    }

    trace!(
        "Supported scan-out formats for plane ({:?}): {:?}",
        plane,
        formats
    );

    Ok(FormatSet::from_formats(formats))
}

#[cfg(feature = "backend_gbm")]
fn plane_has_property(
    dev: &(impl drm::control::Device + DevPath),
    plane: plane::Handle,
    name: &str,
) -> Result<bool, DrmError> {
    let props = dev.get_properties(plane).map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Failed to get properties of plane",
            dev: dev.dev_path(),
            source,
        })
    })?;
    let (ids, _) = props.as_props_and_values();
    for &id in ids {
        let info = dev.get_property(id).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to get property info",
                dev: dev.dev_path(),
                source,
            })
        })?;
        if info.name().to_str().map(|x| x == name).unwrap_or(false) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[repr(C)]
struct drm_plane_size_hint {
    width: u16,
    height: u16,
}

fn plane_size_hints(
    dev: &(impl ControlDevice + DevPath),
    plane: plane::Handle,
) -> Result<Option<Vec<Size<u16, Physical>>>, DrmError> {
    let props = dev.get_properties(plane).map_err(|source| {
        DrmError::Access(AccessError {
            errmsg: "Failed to get properties of plane",
            dev: dev.dev_path(),
            source,
        })
    })?;
    let (ids, vals) = props.as_props_and_values();
    for (&id, &val) in ids.iter().zip(vals.iter()) {
        let info = dev.get_property(id).map_err(|source| {
            DrmError::Access(AccessError {
                errmsg: "Failed to get property info",
                dev: dev.dev_path(),
                source,
            })
        })?;
        if info.name().to_str().map(|x| x == "SIZE_HINTS").unwrap_or(false) {
            let size_hints = if let drm::control::property::Value::Blob(blob_id) =
                info.value_type().convert_value(val)
            {
                if blob_id == 0 {
                    return Ok(None);
                }

                let blob_info = drm_ffi::mode::get_property_blob(dev.as_fd(), blob_id as u32, None).map_err(
                    |source| {
                        DrmError::Access(AccessError {
                            errmsg: "Failed to get SIZE_HINTS blob info",
                            dev: dev.dev_path(),
                            source,
                        })
                    },
                )?;

                let mut data = Vec::with_capacity(blob_info.length as usize);
                drm_ffi::mode::get_property_blob(dev.as_fd(), blob_id as u32, Some(&mut data)).map_err(
                    |source| {
                        DrmError::Access(AccessError {
                            errmsg: "Failed to get SIZE_HINTS blob data",
                            dev: dev.dev_path(),
                            source,
                        })
                    },
                )?;

                let num_size_hints = data.len() / std::mem::size_of::<drm_plane_size_hint>();
                let size_hints = unsafe {
                    std::slice::from_raw_parts(data.as_ptr() as *const drm_plane_size_hint, num_size_hints)
                };
                Some(
                    size_hints
                        .iter()
                        .map(|size_hint| Size::from((size_hint.width, size_hint.height)))
                        .collect(),
                )
            } else {
                tracing::debug!(?plane, "SIZE_HINTS property has wrong value type");
                None
            };
            return Ok(size_hints);
        }
    }
    Ok(None)
}
