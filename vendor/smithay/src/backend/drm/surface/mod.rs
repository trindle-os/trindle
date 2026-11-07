use std::io;
use std::os::unix::io::{AsFd, BorrowedFd};
use std::sync::atomic::Ordering;
use std::sync::Arc;

use drm::control::{connector, crtc, framebuffer, plane, Device as ControlDevice, Mode};
use drm::Device as BasicDevice;

use libc::dev_t;

pub(super) mod atomic;
#[cfg(feature = "backend_gbm")]
pub(super) mod gbm;
pub(super) mod legacy;
use super::{
    device::PlaneClaimStorage, error::Error, plane_type, DrmDeviceFd, PlaneClaim, PlaneInfo, PlaneType,
    Planes,
};
use crate::utils::DevPath;
use crate::utils::{Buffer, Physical, Point, Rectangle, Transform};
use atomic::AtomicDrmSurface;
use legacy::LegacyDrmSurface;

#[derive(Debug)]
pub struct DrmSurface {
    #[allow(dead_code)]
    pub(super) dev_id: dev_t,
    pub(super) crtc: crtc::Handle,
    pub(super) planes: Planes,
    pub(super) internal: Arc<DrmSurfaceInternal>,
    pub(super) plane_claim_storage: PlaneClaimStorage,
    pub(super) primary_plane: (PlaneInfo, PlaneClaim),
}

#[derive(Debug)]
struct PlaneDamageInner {
    drm: DrmDeviceFd,
    blob: Option<drm::control::property::Value<'static>>,
}

impl Drop for PlaneDamageInner {
    fn drop(&mut self) {
        if let Some(drm::control::property::Value::Blob(id)) = self.blob.take() {
            let _ = self.drm.destroy_property_blob(id);
        }
    }
}

#[derive(Debug)]
pub struct PlaneDamageClips {
    inner: Arc<PlaneDamageInner>,
}

impl PlaneDamageClips {
    pub fn blob(&self) -> drm::control::property::Value<'_> {
        self.inner.blob.unwrap()
    }
}

impl PlaneDamageClips {
    #[profiling::function]
    pub fn from_damage(
        device: &DrmDeviceFd,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: impl IntoIterator<Item = Rectangle<i32, Physical>>,
    ) -> io::Result<Option<Self>> {
        let scale = src.size / dst.size.to_logical(1).to_buffer(1, Transform::Normal).to_f64();

        let mut rects = damage
            .into_iter()
            .map(|rect| {
                let mut rect = rect
                    .to_f64()
                    .to_logical(1f64)
                    .to_buffer(
                        1f64,
                        Transform::Normal,
                        &src.size.to_logical(1f64, Transform::Normal),
                    )
                    .upscale(scale);
                rect.loc += src.loc;
                let rect = rect.to_i32_up();

                drm_ffi::drm_mode_rect {
                    x1: rect.loc.x,
                    y1: rect.loc.y,
                    x2: rect.loc.x.saturating_add(rect.size.w),
                    y2: rect.loc.y.saturating_add(rect.size.h),
                }
            })
            .collect::<Vec<_>>();

        if rects.is_empty() {
            return Ok(None);
        }

        let data = unsafe {
            std::slice::from_raw_parts_mut(
                rects.as_mut_ptr() as *mut u8,
                std::mem::size_of::<drm_ffi::drm_mode_rect>() * rects.len(),
            )
        };

        let blob = drm_ffi::mode::create_property_blob(device.as_fd(), data)?;

        Ok(Some(PlaneDamageClips {
            inner: Arc::new(PlaneDamageInner {
                drm: device.clone(),
                blob: Some(drm::control::property::Value::Blob(blob.blob_id as u64)),
            }),
        }))
    }
}

impl Clone for PlaneDamageClips {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaneState<'a> {
    pub handle: plane::Handle,
    pub config: Option<PlaneConfig<'a>>,
}

#[derive(Debug, Clone)]
pub struct PlaneConfig<'a> {
    pub src: Rectangle<f64, Buffer>,
    pub dst: Rectangle<i32, Physical>,
    pub transform: Transform,
    pub alpha: f32,
    pub damage_clips: Option<drm::control::property::Value<'a>>,
    pub fb: framebuffer::Handle,
    pub fence: Option<BorrowedFd<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VrrSupport {
    NotSupported,
    RequiresModeset,
    Supported,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum DrmSurfaceInternal {
    Atomic(AtomicDrmSurface),
    Legacy(LegacyDrmSurface),
}

impl AsFd for DrmSurface {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.fd.as_fd(),
            DrmSurfaceInternal::Legacy(surf) => surf.fd.as_fd(),
        }
    }
}
impl BasicDevice for DrmSurface {}
impl ControlDevice for DrmSurface {}

impl DrmSurface {
    pub fn device_fd(&self) -> &DrmDeviceFd {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.device_fd(),
            DrmSurfaceInternal::Legacy(surf) => surf.device_fd(),
        }
    }

    pub fn is_legacy(&self) -> bool {
        matches!(&*self.internal, DrmSurfaceInternal::Legacy(_))
    }

    pub fn crtc(&self) -> crtc::Handle {
        self.crtc
    }

    pub fn plane(&self) -> plane::Handle {
        self.primary_plane.0.handle
    }

    pub fn plane_info(&self) -> &PlaneInfo {
        &self.primary_plane.0
    }

    pub fn current_connectors(&self) -> impl IntoIterator<Item = connector::Handle> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.current_connectors(),
            DrmSurfaceInternal::Legacy(surf) => surf.current_connectors(),
        }
    }

    pub fn pending_connectors(&self) -> impl IntoIterator<Item = connector::Handle> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.pending_connectors(),
            DrmSurfaceInternal::Legacy(surf) => surf.pending_connectors(),
        }
    }

    pub fn add_connector(&self, connector: connector::Handle) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.add_connector(connector),
            DrmSurfaceInternal::Legacy(surf) => surf.add_connector(connector),
        }
    }

    pub fn remove_connector(&self, connector: connector::Handle) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.remove_connector(connector),
            DrmSurfaceInternal::Legacy(surf) => surf.remove_connector(connector),
        }
    }

    pub fn set_connectors(&self, connectors: &[connector::Handle]) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.set_connectors(connectors),
            DrmSurfaceInternal::Legacy(surf) => surf.set_connectors(connectors),
        }
    }

    pub fn current_mode(&self) -> Mode {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.current_mode(),
            DrmSurfaceInternal::Legacy(surf) => surf.current_mode(),
        }
    }

    pub fn pending_mode(&self) -> Mode {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.pending_mode(),
            DrmSurfaceInternal::Legacy(surf) => surf.pending_mode(),
        }
    }

    pub fn use_mode(&self, mode: Mode) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.use_mode(mode),
            DrmSurfaceInternal::Legacy(surf) => surf.use_mode(mode),
        }
    }

    pub fn vrr_supported(&self, conn: connector::Handle) -> Result<VrrSupport, Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.vrr_supported(conn),
            DrmSurfaceInternal::Legacy(_) => Ok(VrrSupport::NotSupported),
        }
    }

    pub fn vrr_enabled(&self) -> bool {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.vrr_enabled(),
            DrmSurfaceInternal::Legacy(_) => false,
        }
    }

    pub fn use_vrr(&self, vrr: bool) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.use_vrr(vrr),
            DrmSurfaceInternal::Legacy(_) => Err(Error::UnknownProperty {
                handle: self.crtc.into(),
                name: "VRR_ENABLED",
            }),
        }
    }

    pub fn clear_plane(&self, plane: plane::Handle) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.clear_plane(plane),
            DrmSurfaceInternal::Legacy(_) => Err(Error::NonPrimaryPlane(plane)),
        }
    }

    pub fn commit_pending(&self) -> bool {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.commit_pending(),
            DrmSurfaceInternal::Legacy(surf) => surf.commit_pending(),
        }
    }

    #[profiling::function]
    pub fn test_state<'a>(
        &self,
        planes: impl IntoIterator<Item = PlaneState<'a>>,
        allow_modeset: bool,
    ) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.test_state(planes, allow_modeset),
            DrmSurfaceInternal::Legacy(surf) => {
                let fb = ensure_legacy_planes(self, planes)?;

                if allow_modeset {
                    surf.test_buffer(fb, &self.pending_mode())
                } else {
                    Ok(())
                }
            }
        }
    }

    #[profiling::function]
    pub fn commit<'a>(
        &self,
        planes: impl IntoIterator<Item = PlaneState<'a>>,
        event: bool,
    ) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.commit(planes, event),
            DrmSurfaceInternal::Legacy(surf) => {
                let fb = ensure_legacy_planes(self, planes)?;
                surf.commit(fb, event)
            }
        }
    }

    #[profiling::function]
    pub fn page_flip<'a>(
        &self,
        planes: impl IntoIterator<Item = PlaneState<'a>>,
        event: bool,
    ) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.page_flip(planes, event),
            DrmSurfaceInternal::Legacy(surf) => {
                let fb = ensure_legacy_planes(self, planes)?;
                surf.page_flip(fb, event)
            }
        }
    }

    pub fn planes(&self) -> &Planes {
        &self.planes
    }

    pub fn claim_plane(&self, plane: plane::Handle) -> Option<PlaneClaim> {
        if self.planes.primary.iter().any(|p| p.handle == plane)
            || self.planes.cursor.iter().any(|p| p.handle == plane)
            || self.planes.overlay.iter().any(|p| p.handle == plane)
        {
            self.plane_claim_storage.claim(plane, self.crtc)
        } else {
            None
        }
    }

    pub fn reset_state(&self) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.reset_state::<Self>(None),
            DrmSurfaceInternal::Legacy(surf) => surf.reset_state::<Self>(None),
        }
    }

    pub fn is_active(&self) -> bool {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.active.load(Ordering::SeqCst),
            DrmSurfaceInternal::Legacy(surf) => surf.active.load(Ordering::SeqCst),
        }
    }

    #[cfg(feature = "backend_gbm")]
    pub(super) fn span(&self) -> &tracing::Span {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => &surf.span,
            DrmSurfaceInternal::Legacy(surf) => &surf.span,
        }
    }

    pub fn clear(&self) -> Result<(), Error> {
        match &*self.internal {
            DrmSurfaceInternal::Atomic(surf) => surf.clear(),
            DrmSurfaceInternal::Legacy(surf) => surf.clear(),
        }
    }
}

fn ensure_legacy_planes<'a>(
    dev: &(impl ControlDevice + DevPath),
    planes: impl IntoIterator<Item = PlaneState<'a>>,
) -> Result<framebuffer::Handle, Error> {
    let state = planes.into_iter().next().ok_or(Error::NoPlane)?;

    if plane_type(dev, state.handle)? != PlaneType::Primary {
        return Err(Error::NonPrimaryPlane(state.handle));
    }

    let Some(config) = state.config else {
        return Err(Error::NoFramebuffer(state.handle));
    };

    if config.dst.loc != Point::default() {
        return Err(Error::UnsupportedPlaneConfiguration(state.handle));
    }

    if config.src.loc != Point::default()
        || config
            .src
            .size
            .to_logical(1.0, Transform::Normal)
            .to_physical(1.0)
            != config.dst.size.to_f64()
    {
        return Err(Error::UnsupportedPlaneConfiguration(state.handle));
    }

    if config.transform != Transform::Normal {
        return Err(Error::UnsupportedPlaneConfiguration(state.handle));
    }

    Ok(config.fb)
}
