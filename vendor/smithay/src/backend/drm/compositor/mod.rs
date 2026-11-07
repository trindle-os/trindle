//! Composition for [`Element`]s using drm planes.

use std::{
    collections::HashMap,
    fmt::Debug,
    io::ErrorKind,
    os::unix::io::{AsFd, OwnedFd},
    str::FromStr,
    sync::Arc,
};

use drm::{
    control::{connector, crtc, framebuffer, plane, Device as _, Mode, PlaneType},
    Device, DriverCapability,
};
use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
use indexmap::{IndexMap, IndexSet};
use smallvec::SmallVec;
use tracing::{debug, error, info, info_span, instrument, trace, warn};
use wayland_server::{protocol::wl_buffer::WlBuffer, Resource};

#[cfg(feature = "renderer_pixman")]
use crate::backend::renderer::{
    pixman::{PixmanError, PixmanRenderer, PixmanTexture},
    Frame as _, ImportAll,
};
use crate::{
    backend::{
        allocator::{
            dmabuf::{AsDmabuf, Dmabuf},
            format::{get_opaque, has_alpha},
            gbm::{GbmAllocator, GbmBuffer, GbmBufferFlags, GbmDevice},
            Allocator, Buffer, Slot, Swapchain,
        },
        drm::{plane_has_property, DrmError, PlaneDamageClips},
        renderer::{
            buffer_y_inverted,
            damage::{Error as OutputDamageTrackerError, OutputDamageTracker},
            element::{
                Element, Id, Kind, RenderElement, RenderElementPresentationState, RenderElementState,
                RenderElementStates, RenderingReason, UnderlyingStorage,
            },
            sync::SyncPoint,
            utils::{CommitCounter, DamageBag},
            Bind, Color32F, DebugFlags, Renderer, RendererSuper, Texture,
        },
        SwapBuffersError,
    },
    output::OutputModeSource,
    utils::{Buffer as BufferCoords, DevPath, Physical, Point, Rectangle, Scale, Size, Transform},
    wayland::{shm, single_pixel_buffer},
};

use super::{
    error::AccessError,
    exporter::{gbm::GbmFramebufferExporter, ExportBuffer, ExportFramebuffer},
    surface::VrrSupport,
    DrmSurface, Framebuffer, PlaneClaim, PlaneInfo, Planes,
};

mod elements;
mod frame_result;

use elements::*;
pub use frame_result::*;

impl RenderElementState {
    pub(crate) fn zero_copy(visible_area: usize) -> Self {
        RenderElementState {
            visible_area,
            presentation_state: RenderElementPresentationState::ZeroCopy,
        }
    }

    pub(crate) fn rendering_with_reason(reason: RenderingReason) -> Self {
        RenderElementState {
            visible_area: 0,
            presentation_state: RenderElementPresentationState::Rendering { reason: Some(reason) },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ScanoutBuffer<B: Buffer> {
    Wayland(crate::backend::renderer::utils::Buffer),
    Swapchain(Arc<Slot<B>>),
    Cursor(Arc<GbmBuffer>),
}

impl<B: Buffer> Clone for ScanoutBuffer<B> {
    fn clone(&self) -> Self {
        match self {
            Self::Wayland(arg0) => Self::Wayland(arg0.clone()),
            Self::Swapchain(arg0) => Self::Swapchain(arg0.clone()),
            Self::Cursor(arg0) => Self::Cursor(arg0.clone()),
        }
    }
}

impl<B: Buffer> ScanoutBuffer<B> {
    fn acquire_point(
        &self,
        signaled_fence: Option<&Arc<OwnedFd>>,
    ) -> Option<(SyncPoint, Option<Arc<OwnedFd>>)> {
        if let Self::Wayland(buffer) = self {
            if buffer.acquire_point().is_some() {
                return Some((SyncPoint::signaled(), signaled_fence.cloned()));
            }
        }
        None
    }
}

impl<B: Buffer> ScanoutBuffer<B> {
    #[inline]
    fn from_underlying_storage(storage: UnderlyingStorage<'_>) -> Option<Self> {
        match storage {
            UnderlyingStorage::Wayland(buffer) => Some(Self::Wayland(buffer.clone())),
            UnderlyingStorage::Memory { .. } => None,
        }
    }
}

enum DrmFramebuffer<F: Framebuffer> {
    Exporter(F),
    Gbm(super::gbm::GbmFramebuffer),
}

impl<F> AsRef<framebuffer::Handle> for DrmFramebuffer<F>
where
    F: Framebuffer,
{
    #[inline]
    fn as_ref(&self) -> &framebuffer::Handle {
        match self {
            DrmFramebuffer::Exporter(e) => e.as_ref(),
            DrmFramebuffer::Gbm(g) => g.as_ref(),
        }
    }
}

impl<F> Framebuffer for DrmFramebuffer<F>
where
    F: Framebuffer,
{
    #[inline]
    fn format(&self) -> drm_fourcc::DrmFormat {
        match self {
            DrmFramebuffer::Exporter(e) => e.format(),
            DrmFramebuffer::Gbm(g) => g.format(),
        }
    }
}

impl<F> std::fmt::Debug for DrmFramebuffer<F>
where
    F: Framebuffer + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exporter(arg0) => f.debug_tuple("Exporter").field(arg0).finish(),
            Self::Gbm(arg0) => f.debug_tuple("Gbm").field(arg0).finish(),
        }
    }
}

struct DrmScanoutBuffer<B: Buffer, F: Framebuffer> {
    buffer: ScanoutBuffer<B>,
    fb: CachedDrmFramebuffer<F>,
}

impl<B: Buffer, F: Framebuffer> Clone for DrmScanoutBuffer<B, F> {
    fn clone(&self) -> Self {
        DrmScanoutBuffer {
            buffer: self.buffer.clone(),
            fb: self.fb.clone(),
        }
    }
}

impl<B, F> std::fmt::Debug for DrmScanoutBuffer<B, F>
where
    B: Buffer + std::fmt::Debug,
    F: Framebuffer + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DrmScanoutBuffer")
            .field("buffer", &self.buffer)
            .field("fb", &self.fb)
            .finish()
    }
}

impl<B: Buffer, F: Framebuffer> AsRef<framebuffer::Handle> for DrmScanoutBuffer<B, F> {
    #[inline]
    fn as_ref(&self) -> &drm::control::framebuffer::Handle {
        self.fb.as_ref()
    }
}

impl<B: Buffer, F: Framebuffer> Framebuffer for DrmScanoutBuffer<B, F> {
    #[inline]
    fn format(&self) -> drm_fourcc::DrmFormat {
        self.fb.format()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum ElementFramebufferCacheBuffer {
    Wayland(wayland_server::Weak<WlBuffer>),
}

impl ElementFramebufferCacheBuffer {
    #[inline]
    fn from_underlying_storage(storage: &UnderlyingStorage<'_>) -> Option<Self> {
        match storage {
            UnderlyingStorage::Wayland(buffer) => Some(Self::Wayland(buffer.downgrade())),
            UnderlyingStorage::Memory { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ElementFramebufferCacheKey {
    allow_opaque_fallback: bool,
    buffer: ElementFramebufferCacheBuffer,
}

impl ElementFramebufferCacheKey {
    #[inline]
    fn from_underlying_storage(storage: &UnderlyingStorage<'_>, allow_opaque_fallback: bool) -> Option<Self> {
        let buffer = ElementFramebufferCacheBuffer::from_underlying_storage(storage)?;
        Some(Self {
            allow_opaque_fallback,
            buffer,
        })
    }
}

impl ElementFramebufferCacheKey {
    #[inline]
    fn is_alive(&self) -> bool {
        match self.buffer {
            ElementFramebufferCacheBuffer::Wayland(ref buffer) => buffer.is_alive(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct PlanesSnapshot {
    primary: bool,
    cursor_bitmask: u32,
    overlay_bitmask: u32,
}

#[derive(Debug)]
struct ElementInstanceState {
    properties: PlaneProperties,
    active_planes: PlanesSnapshot,
    failed_planes: PlanesSnapshot,
}

#[derive(Debug)]
struct ElementState<B: Framebuffer> {
    instances: SmallVec<[ElementInstanceState; 1]>,
    fb_cache: ElementFramebufferCache<B>,
}

#[derive(Debug)]
struct ElementFramebufferCache<B>
where
    B: Framebuffer,
{
    fb_cache: SmallVec<
        [(
            ElementFramebufferCacheKey,
            Result<CachedDrmFramebuffer<B>, ExportBufferError>,
        ); 4],
    >,
}

impl<B> ElementFramebufferCache<B>
where
    B: Framebuffer,
{
    #[inline]
    fn get(
        &self,
        cache_key: &ElementFramebufferCacheKey,
    ) -> Option<Result<&CachedDrmFramebuffer<B>, ExportBufferError>> {
        self.fb_cache.iter().find_map(|(k, r)| {
            if k == cache_key {
                Some(r.as_ref().map_err(|err| *err))
            } else {
                None
            }
        })
    }

    #[inline]
    fn insert(
        &mut self,
        cache_key: ElementFramebufferCacheKey,
        fb: Result<CachedDrmFramebuffer<B>, ExportBufferError>,
    ) {
        self.fb_cache.push((cache_key, fb));
    }

    fn cleanup(&mut self) {
        self.fb_cache.retain(|(key, _)| key.is_alive());
    }
}

impl<B> Default for ElementFramebufferCache<B>
where
    B: Framebuffer,
{
    #[inline]
    fn default() -> Self {
        Self {
            fb_cache: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PlaneProperties {
    pub src: Rectangle<f64, BufferCoords>,
    pub dst: Rectangle<i32, Physical>,
    pub transform: Transform,
    pub alpha: f32,
    pub format: DrmFormat,
}

impl PlaneProperties {
    #[inline]
    fn is_compatible(&self, other: &PlaneProperties) -> bool {
        self.src == other.src
            && self.dst == other.dst
            && self.transform == other.transform
            && self.alpha == other.alpha
            && self.format == other.format
    }
}

struct ElementPlaneConfig<'a, B: Buffer, F: Framebuffer> {
    z_index: usize,
    geometry: Rectangle<i32, Physical>,
    properties: PlaneProperties,
    buffer: DrmScanoutBuffer<B, F>,
    failed_planes: &'a mut PlanesSnapshot,
}

#[derive(Debug)]
struct PlaneConfig<B: Buffer, F: Framebuffer> {
    pub properties: PlaneProperties,
    pub buffer: DrmScanoutBuffer<B, F>,
    pub damage_clips: Option<PlaneDamageClips>,
    pub plane_claim: PlaneClaim,
    pub sync: Option<(SyncPoint, Option<Arc<OwnedFd>>)>,
}

impl<B: Buffer, F: Framebuffer> PlaneConfig<B, F> {
    #[inline]
    pub fn is_compatible(&self, other: &PlaneConfig<B, F>) -> bool {
        self.properties.is_compatible(&other.properties)
    }
}

impl<B: Buffer, F: Framebuffer> Clone for PlaneConfig<B, F> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            properties: self.properties,
            buffer: self.buffer.clone(),
            damage_clips: self.damage_clips.clone(),
            plane_claim: self.plane_claim.clone(),
            sync: self.sync.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct PlaneElementState {
    id: Id,
    commit: CommitCounter,
    z_index: usize,
    cursor_size: Option<Size<i32, Physical>>,
}

#[derive(Debug)]
struct PlaneState<B: Buffer, F: Framebuffer> {
    skip: bool,
    needs_test: bool,
    element_state: Option<PlaneElementState>,
    config: Option<PlaneConfig<B, F>>,
}

impl<B: Buffer, F: Framebuffer> Default for PlaneState<B, F> {
    #[inline]
    fn default() -> Self {
        Self {
            skip: true,
            needs_test: false,
            element_state: Default::default(),
            config: Default::default(),
        }
    }
}

impl<B: Buffer, F: Framebuffer> PlaneState<B, F> {
    #[inline]
    fn buffer(&self) -> Option<&DrmScanoutBuffer<B, F>> {
        self.config.as_ref().map(|config| &config.buffer)
    }

    #[inline]
    fn is_compatible(&self, other: &Self) -> bool {
        match (self.config.as_ref(), other.config.as_ref()) {
            (Some(a), Some(b)) => a.is_compatible(b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<B: Buffer, F: Framebuffer> Clone for PlaneState<B, F> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            skip: self.skip,
            needs_test: self.needs_test,
            element_state: self.element_state.clone(),
            config: self.config.clone(),
        }
    }
}

#[derive(Debug)]
struct FrameState<B: Buffer, F: Framebuffer> {
    planes: SmallVec<[(plane::Handle, PlaneState<B, F>); 10]>,
}

impl<B: Buffer, F: Framebuffer> FrameState<B, F> {
    #[inline]
    fn is_assigned(&self, handle: plane::Handle) -> bool {
        self.planes
            .iter()
            .find_map(|(p, state)| {
                if *p == handle {
                    Some(state.config.is_some())
                } else {
                    None
                }
            })
            .unwrap_or(false)
    }

    #[inline]
    fn overlaps(&self, handle: plane::Handle, element_geometry: Rectangle<i32, Physical>) -> bool {
        self.planes
            .iter()
            .find(|(p, _)| *p == handle)
            .and_then(|(_, state)| {
                state
                    .config
                    .as_ref()
                    .map(|config| config.properties.dst.overlaps(element_geometry))
            })
            .unwrap_or(false)
    }

    #[inline]
    fn plane_state(&self, handle: plane::Handle) -> Option<&PlaneState<B, F>> {
        self.planes
            .iter()
            .find_map(|(p, state)| if *p == handle { Some(state) } else { None })
    }

    #[inline]
    fn plane_state_mut(&mut self, handle: plane::Handle) -> Option<&mut PlaneState<B, F>> {
        self.planes
            .iter_mut()
            .find_map(|(p, state)| if *p == handle { Some(state) } else { None })
    }

    #[inline]
    fn plane_properties(&self, handle: plane::Handle) -> Option<&PlaneProperties> {
        self.plane_state(handle)
            .and_then(|state| state.config.as_ref())
            .map(|config| &config.properties)
    }

    #[inline]
    fn plane_buffer(&self, handle: plane::Handle) -> Option<&DrmScanoutBuffer<B, F>> {
        self.plane_state(handle)
            .and_then(|state| state.config.as_ref().map(|config| &config.buffer))
    }
}

impl<B: Buffer, F: Framebuffer> FrameState<B, F> {
    fn from_planes(primary_plane: plane::Handle, planes: &Planes) -> Self {
        let mut tmp = SmallVec::with_capacity(planes.overlay.len() + planes.cursor.len() + 1);
        tmp.push((primary_plane, PlaneState::default()));
        tmp.extend(
            planes
                .cursor
                .iter()
                .map(|info| (info.handle, PlaneState::default())),
        );
        tmp.extend(
            planes
                .overlay
                .iter()
                .map(|info| (info.handle, PlaneState::default())),
        );

        FrameState { planes: tmp }
    }
}

impl<B: Buffer, F: Framebuffer> FrameState<B, F> {
    #[profiling::function]
    #[inline]
    fn set_state(&mut self, plane: plane::Handle, state: PlaneState<B, F>) {
        let current_config = match self.plane_state_mut(plane) {
            Some(config) => config,
            None => return,
        };
        *current_config = state;
    }

    #[profiling::function]
    fn test_state(
        &mut self,
        surface: &DrmSurface,
        supports_fencing: bool,
        plane: plane::Handle,
        state: PlaneState<B, F>,
        allow_modeset: bool,
    ) -> Result<(), DrmError> {
        let current_config = match self.plane_state_mut(plane) {
            Some(config) => config,
            None => return Ok(()),
        };
        let backup = current_config.clone();
        *current_config = state;

        let res = surface.test_state(self.build_planes(surface, supports_fencing, true), allow_modeset);

        if res.is_err() {
            *self.plane_state_mut(plane).unwrap() = backup;
        } else {
            self.planes
                .iter_mut()
                .for_each(|(_, state)| state.needs_test = false);
        }

        res
    }

    #[profiling::function]
    fn test_state_complete(
        &mut self,
        previous_frame: &Self,
        surface: &DrmSurface,
        supports_fencing: bool,
        allow_modeset: bool,
        allow_partial_update: bool,
    ) -> Result<(), DrmError> {
        let needs_test = self.planes.iter().any(|(_, state)| state.needs_test);
        let is_fully_compatible = self.planes.iter().all(|(handle, state)| {
            previous_frame
                .plane_state(*handle)
                .map(|other| state.is_compatible(other))
                .unwrap_or(false)
        });

        if allow_partial_update && (!needs_test || is_fully_compatible) {
            trace!("skipping fully compatible state test");
            self.planes
                .iter_mut()
                .for_each(|(_, state)| state.needs_test = false);
            return Ok(());
        }

        let res = surface.test_state(
            self.build_planes(surface, supports_fencing, allow_partial_update),
            allow_modeset,
        );

        if res.is_ok() {
            self.planes
                .iter_mut()
                .for_each(|(_, state)| state.needs_test = false);
        }

        res
    }

    #[profiling::function]
    fn commit(
        &mut self,
        surface: &DrmSurface,
        supports_fencing: bool,
        allow_partial_update: bool,
        event: bool,
    ) -> Result<(), crate::backend::drm::error::Error> {
        debug_assert!(!self.planes.iter().any(|(_, state)| state.needs_test));
        surface.commit(
            self.build_planes(surface, supports_fencing, allow_partial_update),
            event,
        )
    }

    #[profiling::function]
    fn page_flip(
        &mut self,
        surface: &DrmSurface,
        supports_fencing: bool,
        allow_partial_update: bool,
        event: bool,
    ) -> Result<(), crate::backend::drm::error::Error> {
        debug_assert!(!self.planes.iter().any(|(_, state)| state.needs_test));
        surface.page_flip(
            self.build_planes(surface, supports_fencing, allow_partial_update),
            event,
        )
    }

    #[profiling::function]
    fn build_planes<'a>(
        &'a mut self,
        surface: &'a DrmSurface,
        supports_fencing: bool,
        allow_partial_update: bool,
    ) -> impl IntoIterator<Item = super::PlaneState<'a>> {
        for (_, state) in self.planes.iter_mut().filter(|(_, state)| !state.skip) {
            if let Some(config) = state.config.as_mut() {
                if let Some((sync, fence)) = config.sync.as_mut() {
                    if supports_fencing && fence.is_none() {
                        *fence = sync.export().map(Arc::new);
                    }
                }
            }
        }

        self.planes
            .iter_mut()
            .filter(move |(handle, state)| {
                if allow_partial_update {
                    !state.skip || state.config.is_some()
                } else {
                    state.config.is_some() || surface.claim_plane(*handle).is_some()
                }
            })
            .map(move |(handle, state)| super::surface::PlaneState {
                handle: *handle,
                config: state.config.as_mut().map(|config| super::PlaneConfig {
                    src: config.properties.src,
                    dst: config.properties.dst,
                    alpha: config.properties.alpha,
                    transform: config.properties.transform,
                    damage_clips: config.damage_clips.as_ref().map(|d| d.blob()),
                    fb: *config.buffer.as_ref(),
                    fence: config
                        .sync
                        .as_ref()
                        .and_then(|(_, fence)| fence.as_ref().map(|fence| fence.as_fd())),
                }),
            })
    }
}

type CompositorFrameState<A, F> =
    FrameState<<A as Allocator>::Buffer, <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer>;

type FrameErrorType<A, F> = FrameError<
    <A as Allocator>::Error,
    <<A as Allocator>::Buffer as AsDmabuf>::Error,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error,
>;

pub(crate) type FrameResult<T, A, F> = Result<T, FrameErrorType<A, F>>;

pub(crate) type RenderFrameErrorType<A, F, R> = RenderFrameError<
    <A as Allocator>::Error,
    <<A as Allocator>::Buffer as AsDmabuf>::Error,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error,
    <R as RendererSuper>::Error,
>;

#[derive(Debug)]
struct CursorState<G: AsFd + 'static> {
    allocator: GbmAllocator<G>,
    framebuffer_exporter: GbmFramebufferExporter<G>,
    previous_output_transform: Option<Transform>,
    previous_output_scale: Option<Scale<f64>>,
    #[cfg(feature = "renderer_pixman")]
    pixman_renderer: Option<PixmanRenderer>,
}

#[derive(Debug, thiserror::Error, Copy, Clone)]
enum ExportBufferError {
    #[error("the buffer has no underlying storage")]
    NoUnderlyingStorage,
    #[error("exporting the framebuffer failed")]
    ExportFailed,
    #[error("no framebuffer could be exported")]
    Unsupported,
}

impl From<ExportBufferError> for Option<RenderingReason> {
    #[inline]
    fn from(err: ExportBufferError) -> Self {
        if matches!(err, ExportBufferError::ExportFailed) {
            Some(RenderingReason::ScanoutFailed)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct OverlayPlaneElementIds {
    plane_ids: Vec<(plane::Handle, Id, Id)>,
}

impl OverlayPlaneElementIds {
    fn from_planes(planes: &Planes) -> Self {
        let overlay_plane_count = planes.overlay.len();

        Self {
            plane_ids: Vec::with_capacity(overlay_plane_count),
        }
    }

    fn plane_id_for_element_id(&mut self, plane: &plane::Handle, element_id: &Id) -> Id {
        let existing = self.plane_ids.iter_mut().find(|(p, _, _)| p == plane);
        if let Some((_, plane_id, current_element_id)) = existing {
            if current_element_id != element_id {
                *plane_id = Id::new();
                *current_element_id = element_id.clone();
            }

            plane_id.clone()
        } else {
            let plane_id = Id::new();

            self.plane_ids
                .push((*plane, plane_id.clone(), element_id.clone()));

            plane_id
        }
    }

    fn contains_plane_id(&self, plane_id: &Id) -> bool {
        self.plane_ids.iter().any(|(_, p, _)| p == plane_id)
    }

    fn remove_plane(&mut self, plane: &plane::Handle) {
        self.plane_ids.retain(|(p, _, _)| p != plane);
    }
}

struct PlaneAssignment {
    handle: plane::Handle,
    type_: PlaneType,
}

impl From<&PlaneInfo> for PlaneAssignment {
    #[inline]
    fn from(value: &PlaneInfo) -> Self {
        PlaneAssignment {
            handle: value.handle,
            type_: value.type_,
        }
    }
}

struct PendingFrame<A: Allocator, F: ExportFramebuffer<<A as Allocator>::Buffer>, U> {
    frame: CompositorFrameState<A, F>,
    user_data: U,
}

impl<A, F, U> std::fmt::Debug for PendingFrame<A, F, U>
where
    A: Allocator,
    <A as Allocator>::Buffer: std::fmt::Debug,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingFrame")
            .field("frame", &self.frame)
            .field("user_data", &self.user_data)
            .finish()
    }
}

struct QueuedFrame<A: Allocator, F: ExportFramebuffer<<A as Allocator>::Buffer>, U> {
    prepared_frame: PreparedFrame<A, F>,
    user_data: U,
}

impl<A, F, U> std::fmt::Debug for QueuedFrame<A, F, U>
where
    A: Allocator,
    <A as Allocator>::Buffer: std::fmt::Debug,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueuedFrame")
            .field("prepared_frame", &self.prepared_frame)
            .field("user_data", &self.user_data)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PreparedFrameKind {
    Full,
    Partial,
}

struct PreparedFrame<A: Allocator, F: ExportFramebuffer<<A as Allocator>::Buffer>> {
    frame: CompositorFrameState<A, F>,
    kind: PreparedFrameKind,
}

impl<A: Allocator, F: ExportFramebuffer<<A as Allocator>::Buffer>> PreparedFrame<A, F> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.kind == PreparedFrameKind::Partial && self.frame.planes.iter().all(|p| p.1.skip)
    }
}

impl<A, F> std::fmt::Debug for PreparedFrame<A, F>
where
    A: Allocator,
    <A as Allocator>::Buffer: std::fmt::Debug,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedFrame")
            .field("frame", &self.frame)
            .field("kind", &self.kind)
            .finish()
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct FrameFlags: u32 {
        const ALLOW_PRIMARY_PLANE_SCANOUT = 1;
        const ALLOW_PRIMARY_PLANE_SCANOUT_ANY = 2;
        const ALLOW_OVERLAY_PLANE_SCANOUT = 4;
        const ALLOW_CURSOR_PLANE_SCANOUT = 8;
        const SKIP_CURSOR_ONLY_UPDATES = 16;
        const ALLOW_SCANOUT = Self::ALLOW_PRIMARY_PLANE_SCANOUT.bits() | Self::ALLOW_OVERLAY_PLANE_SCANOUT.bits() | Self::ALLOW_CURSOR_PLANE_SCANOUT.bits();
        const DEFAULT = Self::ALLOW_SCANOUT.bits();
    }
}

#[derive(Debug)]
pub struct DrmCompositor<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<A::Buffer>,
    <F as ExportFramebuffer<A::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    G: AsFd + 'static,
{
    output_mode_source: OutputModeSource,
    surface: Arc<DrmSurface>,
    planes: Planes,
    overlay_plane_element_ids: OverlayPlaneElementIds,
    damage_tracker: OutputDamageTracker,
    primary_is_opaque: bool,
    primary_plane_element_id: Id,
    primary_plane_damage_bag: DamageBag<i32, BufferCoords>,
    supports_fencing: bool,
    reset_pending: bool,
    signaled_fence: Option<Arc<OwnedFd>>,

    framebuffer_exporter: F,

    current_frame: CompositorFrameState<A, F>,
    pending_frame: Option<PendingFrame<A, F, U>>,
    queued_frame: Option<QueuedFrame<A, F, U>>,
    next_frame: Option<PreparedFrame<A, F>>,

    swapchain: Swapchain<A>,

    cursor_size: Size<i32, Physical>,
    cursor_state: Option<CursorState<G>>,

    element_states: IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
    previous_element_states: IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
    opaque_regions: Vec<Rectangle<i32, Physical>>,
    element_opaque_regions_workhouse: Vec<Rectangle<i32, Physical>>,

    debug_flags: DebugFlags,
    span: tracing::Span,
}

impl<A, F, U, G> DrmCompositor<A, F, U, G>
where
    A: Allocator,
    <A as Allocator>::Error: std::error::Error + Send + Sync,
    <A as Allocator>::Buffer: AsDmabuf,
    <A::Buffer as AsDmabuf>::Error: std::error::Error + Send + Sync + std::fmt::Debug,
    F: ExportFramebuffer<A::Buffer>,
    <F as ExportFramebuffer<A::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    <F as ExportFramebuffer<A::Buffer>>::Error: std::error::Error + Send + Sync,
    G: AsFd + Clone,
{
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip_all)]
    pub fn new(
        output_mode_source: impl Into<OutputModeSource> + Debug,
        surface: DrmSurface,
        planes: Option<Planes>,
        mut allocator: A,
        framebuffer_exporter: F,
        color_formats: impl IntoIterator<Item = DrmFourcc>,
        renderer_formats: impl IntoIterator<Item = DrmFormat>,
        cursor_size: Size<u32, BufferCoords>,
        gbm: Option<GbmDevice<G>>,
    ) -> FrameResult<Self, A, F> {
        let signaled_fence = match surface.create_syncobj(true) {
            Ok(signaled_syncobj) => match surface.syncobj_to_fd(signaled_syncobj, true) {
                Ok(signaled_fence) => {
                    let _ = surface.destroy_syncobj(signaled_syncobj);
                    Some(Arc::new(signaled_fence))
                }
                Err(err) => {
                    tracing::warn!(?err, "failed to export signaled syncobj");
                    let _ = surface.destroy_syncobj(signaled_syncobj);
                    None
                }
            },
            Err(err) => {
                tracing::warn!(?err, "failed to create signaled syncobj");
                None
            }
        };

        let span = info_span!(
            parent: None,
            "drm_compositor",
            device = ?surface.dev_path(),
            crtc = ?surface.crtc(),
        );

        let output_mode_source = output_mode_source.into();
        let renderer_formats = renderer_formats.into_iter().collect::<Vec<_>>();

        let mut error = None;
        let surface = Arc::new(surface);
        let mut planes = match planes {
            Some(planes) => planes,
            None => surface.planes().clone(),
        };

        if surface.is_legacy() {
            planes.cursor.clear();
            planes.overlay.clear();
        }

        planes
            .overlay
            .sort_by_key(|p| std::cmp::Reverse(p.zpos.unwrap_or_default()));

        let driver = surface.get_driver().map_err(|err| {
            FrameError::DrmError(DrmError::Access(AccessError {
                errmsg: "Failed to query drm driver",
                dev: surface.dev_path(),
                source: err,
            }))
        })?;
        let is_nvidia = driver.name().to_string_lossy().to_lowercase().contains("nvidia")
            || driver
                .description()
                .to_string_lossy()
                .to_lowercase()
                .contains("nvidia");

        let cursor_size = Size::from((cursor_size.w as i32, cursor_size.h as i32));
        let damage_tracker = OutputDamageTracker::from_mode_source(output_mode_source.clone());
        let supports_fencing = !surface.is_legacy()
            && surface
                .get_driver_capability(DriverCapability::SyncObj)
                .map(|val| val != 0)
                .map_err(|err| {
                    FrameError::DrmError(DrmError::Access(AccessError {
                        errmsg: "Failed to query driver capability",
                        dev: surface.dev_path(),
                        source: err,
                    }))
                })?
            && plane_has_property(&*surface, surface.plane(), "IN_FENCE_FD")?
            && !(is_nvidia && nvidia_drm_version().unwrap_or((0, 0, 0)) < (560, 35, 3));

        for format in color_formats {
            debug!("Testing color format: {}", format);
            match Self::find_supported_format(
                surface.clone(),
                supports_fencing,
                &planes,
                allocator,
                &framebuffer_exporter,
                renderer_formats.clone(),
                format,
            ) {
                Ok((swapchain, is_opaque)) => {
                    let cursor_state = gbm.map(|gbm| {
                        #[cfg(feature = "renderer_pixman")]
                        let pixman_renderer = match PixmanRenderer::new() {
                            Ok(pixman_renderer) => Some(pixman_renderer),
                            Err(err) => {
                                tracing::warn!(?err, "failed to initialize pixman renderer for cursor plane");
                                None
                            }
                        };

                        let cursor_allocator =
                            GbmAllocator::new(gbm.clone(), GbmBufferFlags::CURSOR | GbmBufferFlags::WRITE);
                        let framebuffer_exporter = GbmFramebufferExporter::new(gbm.clone(), None);
                        CursorState {
                            allocator: cursor_allocator,
                            framebuffer_exporter,
                            previous_output_scale: None,
                            previous_output_transform: None,
                            #[cfg(feature = "renderer_pixman")]
                            pixman_renderer,
                        }
                    });

                    let overlay_plane_element_ids = OverlayPlaneElementIds::from_planes(&planes);
                    let current_frame = FrameState::from_planes(surface.plane(), &planes);

                    let drm_renderer = DrmCompositor {
                        primary_plane_element_id: Id::new(),
                        primary_plane_damage_bag: DamageBag::new(4),
                        primary_is_opaque: is_opaque,
                        reset_pending: true,
                        signaled_fence,
                        current_frame,
                        pending_frame: None,
                        queued_frame: None,
                        next_frame: None,
                        swapchain,
                        framebuffer_exporter,
                        cursor_size,
                        cursor_state,
                        surface,
                        damage_tracker,
                        output_mode_source,
                        planes,
                        overlay_plane_element_ids,
                        element_states: IndexMap::new(),
                        previous_element_states: IndexMap::new(),
                        opaque_regions: Vec::new(),
                        element_opaque_regions_workhouse: Vec::new(),
                        supports_fencing,
                        debug_flags: DebugFlags::empty(),
                        span,
                    };

                    return Ok(drm_renderer);
                }
                Err((alloc, err)) => {
                    warn!("Preferred format {} not available: {:?}", format, err);
                    allocator = alloc;
                    error = Some(err);
                }
            }
        }
        Err(error.unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_format(
        output_mode_source: impl Into<OutputModeSource> + Debug,
        surface: DrmSurface,
        planes: Option<Planes>,
        allocator: A,
        framebuffer_exporter: F,
        code: DrmFourcc,
        modifiers: impl IntoIterator<Item = DrmModifier>,
        cursor_size: Size<u32, BufferCoords>,
        gbm: Option<GbmDevice<G>>,
    ) -> FrameResult<Self, A, F> {
        let signaled_fence = match surface.create_syncobj(true) {
            Ok(signaled_syncobj) => match surface.syncobj_to_fd(signaled_syncobj, true) {
                Ok(signaled_fence) => {
                    let _ = surface.destroy_syncobj(signaled_syncobj);
                    Some(Arc::new(signaled_fence))
                }
                Err(err) => {
                    tracing::warn!(?err, "failed to export signaled syncobj");
                    let _ = surface.destroy_syncobj(signaled_syncobj);
                    None
                }
            },
            Err(err) => {
                tracing::warn!(?err, "failed to create signaled syncobj");
                None
            }
        };

        let span = info_span!(
            parent: None,
            "drm_compositor",
            device = ?surface.dev_path(),
            crtc = ?surface.crtc(),
        );

        let output_mode_source = output_mode_source.into();

        let surface = Arc::new(surface);
        let mut planes = match planes {
            Some(planes) => planes,
            None => surface.planes().clone(),
        };

        if surface.is_legacy() {
            planes.cursor.clear();
            planes.overlay.clear();
        }

        planes
            .overlay
            .sort_by_key(|p| std::cmp::Reverse(p.zpos.unwrap_or_default()));

        let driver = surface.get_driver().map_err(|err| {
            FrameError::DrmError(DrmError::Access(AccessError {
                errmsg: "Failed to query drm driver",
                dev: surface.dev_path(),
                source: err,
            }))
        })?;
        let is_nvidia = driver.name().to_string_lossy().to_lowercase().contains("nvidia")
            || driver
                .description()
                .to_string_lossy()
                .to_lowercase()
                .contains("nvidia");

        let cursor_size = Size::from((cursor_size.w as i32, cursor_size.h as i32));
        let damage_tracker = OutputDamageTracker::from_mode_source(output_mode_source.clone());
        let supports_fencing = !surface.is_legacy()
            && surface
                .get_driver_capability(DriverCapability::SyncObj)
                .map(|val| val != 0)
                .map_err(|err| {
                    FrameError::DrmError(DrmError::Access(AccessError {
                        errmsg: "Failed to query driver capability",
                        dev: surface.dev_path(),
                        source: err,
                    }))
                })?
            && plane_has_property(&*surface, surface.plane(), "IN_FENCE_FD")?
            && !(is_nvidia && nvidia_drm_version().unwrap_or((0, 0, 0)) < (560, 35, 3));

        let (swapchain, is_opaque) = Self::test_format(
            &surface,
            supports_fencing,
            &planes,
            allocator,
            &framebuffer_exporter,
            code,
            modifiers,
        )
        .map_err(|(_, err)| err)?;

        let cursor_state = gbm.map(|gbm| {
            #[cfg(feature = "renderer_pixman")]
            let pixman_renderer = match PixmanRenderer::new() {
                Ok(pixman_renderer) => Some(pixman_renderer),
                Err(err) => {
                    tracing::warn!(?err, "failed to initialize pixman renderer for cursor plane");
                    None
                }
            };

            let cursor_allocator =
                GbmAllocator::new(gbm.clone(), GbmBufferFlags::CURSOR | GbmBufferFlags::WRITE);
            let framebuffer_exporter = GbmFramebufferExporter::new(gbm.clone(), None);
            CursorState {
                allocator: cursor_allocator,
                framebuffer_exporter,
                previous_output_scale: None,
                previous_output_transform: None,
                #[cfg(feature = "renderer_pixman")]
                pixman_renderer,
            }
        });

        let overlay_plane_element_ids = OverlayPlaneElementIds::from_planes(&planes);
        let current_frame = FrameState::from_planes(surface.plane(), &planes);

        let drm_renderer = DrmCompositor {
            primary_plane_element_id: Id::new(),
            primary_plane_damage_bag: DamageBag::new(4),
            primary_is_opaque: is_opaque,
            reset_pending: true,
            signaled_fence,
            current_frame,
            pending_frame: None,
            queued_frame: None,
            next_frame: None,
            swapchain,
            framebuffer_exporter,
            cursor_size,
            cursor_state,
            surface,
            damage_tracker,
            output_mode_source,
            planes,
            overlay_plane_element_ids,
            element_states: IndexMap::new(),
            previous_element_states: IndexMap::new(),
            opaque_regions: Vec::new(),
            element_opaque_regions_workhouse: Vec::new(),
            supports_fencing,
            debug_flags: DebugFlags::empty(),
            span,
        };

        Ok(drm_renderer)
    }

    fn test_format(
        drm: &DrmSurface,
        supports_fencing: bool,
        planes: &Planes,
        allocator: A,
        framebuffer_exporter: &F,
        code: DrmFourcc,
        modifiers: impl IntoIterator<Item = DrmModifier>,
    ) -> Result<(Swapchain<A>, bool), (A, FrameErrorType<A, F>)> {
        let modifiers = modifiers.into_iter().collect::<IndexSet<_>>();
        let mut plane_formats = drm.plane_info().formats.iter().copied().collect::<IndexSet<_>>();

        let opaque_code = get_opaque(code).unwrap_or(code);
        if !plane_formats
            .iter()
            .any(|fmt| fmt.code == code || fmt.code == opaque_code)
        {
            return Err((allocator, FrameError::NoSupportedPlaneFormat));
        }
        plane_formats.retain(|fmt| fmt.code == code || fmt.code == opaque_code);

        if plane_formats.is_empty() {
            return Err((allocator, FrameError::NoSupportedPlaneFormat));
        }

        let plane_modifiers = plane_formats
            .iter()
            .map(|fmt| fmt.modifier)
            .collect::<IndexSet<_>>();

        let swapchain_modifiers = plane_modifiers
            .intersection(&modifiers)
            .copied()
            .collect::<Vec<_>>();

        if swapchain_modifiers.is_empty() {
            return Err((allocator, FrameError::NoSupportedPlaneFormat));
        }

        let mode = drm.pending_mode();

        let mut swapchain: Swapchain<A> = Swapchain::new(
            allocator,
            mode.size().0 as u32,
            mode.size().1 as u32,
            code,
            swapchain_modifiers,
        );

        let buffer = match swapchain.acquire() {
            Ok(buffer) => buffer.unwrap(),
            Err(err) => return Err((swapchain.allocator, FrameError::Allocator(err))),
        };

        let dmabuf = match buffer.export() {
            Ok(dmabuf) => dmabuf,
            Err(err) => {
                return Err((swapchain.allocator, FrameError::AsDmabufError(err)));
            }
        };

        let use_opaque = !plane_formats.iter().any(|f| f.code == code);
        let fb_buffer = match framebuffer_exporter.add_framebuffer(
            drm.device_fd(),
            ExportBuffer::Allocator(&buffer),
            use_opaque,
        ) {
            Ok(Some(fb_buffer)) => fb_buffer,
            Ok(None) => return Err((swapchain.allocator, FrameError::NoFramebuffer)),
            Err(err) => return Err((swapchain.allocator, FrameError::FramebufferExport(err))),
        };
        buffer
            .userdata()
            .insert_if_missing(|| CachedDrmFramebuffer::new(DrmFramebuffer::Exporter(fb_buffer)));

        let mode = drm.pending_mode();
        let handle = buffer
            .userdata()
            .get::<CachedDrmFramebuffer<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>()
            .unwrap()
            .clone();

        let mode_size = Size::from((mode.size().0 as i32, mode.size().1 as i32));

        let mut current_frame_state = FrameState::from_planes(drm.plane(), planes);
        let plane_claim = match drm.claim_plane(drm.plane()) {
            Some(claim) => claim,
            None => {
                warn!("failed to claim primary plane",);
                return Err((swapchain.allocator, FrameError::PrimaryPlaneClaimFailed));
            }
        };

        let plane_state = PlaneState {
            skip: false,
            needs_test: true,
            element_state: None,
            config: Some(PlaneConfig {
                properties: PlaneProperties {
                    src: Rectangle::from_size(dmabuf.size()).to_f64(),
                    dst: Rectangle::from_size(mode_size),
                    transform: Transform::Normal,
                    alpha: 1.0,
                    format: buffer.format(),
                },
                buffer: DrmScanoutBuffer {
                    buffer: ScanoutBuffer::Swapchain(Arc::new(buffer)),
                    fb: handle,
                },
                damage_clips: None,
                plane_claim,
                sync: None,
            }),
        };

        match current_frame_state.test_state(drm, supports_fencing, drm.plane(), plane_state, true) {
            Ok(_) => Ok((swapchain, use_opaque)),
            Err(err) => {
                warn!(
                    "Mode-setting failed with buffer format {:?}: {}",
                    dmabuf.format(),
                    err
                );
                Err((swapchain.allocator, err.into()))
            }
        }
    }

    fn find_supported_format(
        drm: Arc<DrmSurface>,
        supports_fencing: bool,
        planes: &Planes,
        allocator: A,
        framebuffer_exporter: &F,
        mut renderer_formats: Vec<DrmFormat>,
        code: DrmFourcc,
    ) -> Result<(Swapchain<A>, bool), (A, FrameErrorType<A, F>)> {
        let mut plane_formats = drm.plane_info().formats.iter().copied().collect::<IndexSet<_>>();

        let opaque_code = get_opaque(code).unwrap_or(code);
        if !plane_formats
            .iter()
            .any(|fmt| fmt.code == code || fmt.code == opaque_code)
        {
            return Err((allocator, FrameError::NoSupportedPlaneFormat));
        }
        plane_formats.retain(|fmt| fmt.code == code || fmt.code == opaque_code);
        renderer_formats.retain(|fmt| fmt.code == code);

        trace!("Plane formats: {:?}", plane_formats);
        trace!("Renderer formats: {:?}", renderer_formats);

        let plane_modifiers = plane_formats
            .iter()
            .map(|fmt| fmt.modifier)
            .collect::<IndexSet<_>>();
        let renderer_modifiers = renderer_formats
            .iter()
            .map(|fmt| fmt.modifier)
            .collect::<IndexSet<_>>();
        debug!(
            "Remaining intersected modifiers: {:?}",
            plane_modifiers
                .intersection(&renderer_modifiers)
                .collect::<IndexSet<_>>()
        );

        if plane_formats.is_empty() {
            return Err((allocator, FrameError::NoSupportedPlaneFormat));
        } else if renderer_formats.is_empty() {
            return Err((allocator, FrameError::NoSupportedRendererFormat));
        }

        let formats = {
            if (plane_formats.len() == 1
                && plane_formats.iter().next().unwrap().modifier == DrmModifier::Invalid
                && renderer_formats
                    .iter()
                    .all(|x| x.modifier != DrmModifier::Invalid)
                && renderer_formats.iter().any(|x| x.modifier == DrmModifier::Linear))
                || (renderer_formats.len() == 1
                    && renderer_formats.first().unwrap().modifier == DrmModifier::Invalid
                    && plane_formats.iter().all(|x| x.modifier != DrmModifier::Invalid)
                    && plane_formats.iter().any(|x| x.modifier == DrmModifier::Linear))
            {
                vec![DrmFormat {
                    code,
                    modifier: DrmModifier::Invalid,
                }]
            } else {
                plane_modifiers
                    .intersection(&renderer_modifiers)
                    .cloned()
                    .map(|modifier| DrmFormat { code, modifier })
                    .collect::<Vec<_>>()
            }
        };

        debug!("Testing Formats: {:?}", formats);

        let modifiers = formats.iter().map(|x| x.modifier).collect::<Vec<_>>();

        let (swapchain, use_opaque) = Self::test_format(
            &drm,
            supports_fencing,
            planes,
            allocator,
            framebuffer_exporter,
            code,
            modifiers,
        )?;

        Ok((swapchain, use_opaque))
    }

    #[instrument(level = "trace", parent = &self.span, skip_all)]
    #[profiling::function]
    pub fn render_frame<'a, R, E>(
        &mut self,
        renderer: &mut R,
        elements: &'a [E],
        clear_color: impl Into<Color32F>,
        frame_flags: FrameFlags,
    ) -> Result<RenderFrameResult<'a, A::Buffer, F::Framebuffer, E>, RenderFrameErrorType<A, F, R>>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
    {
        let mut clear_color = clear_color.into();

        if !self.surface.is_active() {
            return Err(RenderFrameErrorType::<A, F, R>::PrepareFrame(
                FrameError::DrmError(DrmError::DeviceInactive),
            ));
        }

        std::mem::drop(self.next_frame.take());

        let allow_partial_update = !self.reset_pending && !self.surface.commit_pending();

        let (current_size, output_scale, output_transform) = (&self.output_mode_source)
            .try_into()
            .map_err(OutputDamageTrackerError::OutputNoMode)?;

        let output_transform = output_transform.invert();

        let output_geometry: Rectangle<_, Physical> =
            Rectangle::from_size(output_transform.transform_size(current_size));

        let primary_plane_buffer = self
            .swapchain
            .acquire()
            .map_err(FrameError::Allocator)?
            .ok_or(FrameError::NoFreeSlotsError)?;

        let dmabuf = primary_plane_buffer.export().map_err(FrameError::AsDmabufError)?;

        let maybe_buffer = primary_plane_buffer
            .userdata()
            .get::<CachedDrmFramebuffer<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>();
        if maybe_buffer.is_none() {
            let fb_buffer = self
                .framebuffer_exporter
                .add_framebuffer(
                    self.surface.device_fd(),
                    ExportBuffer::Allocator(&primary_plane_buffer),
                    self.primary_is_opaque,
                )
                .map_err(FrameError::FramebufferExport)?
                .ok_or(FrameError::NoFramebuffer)?;
            primary_plane_buffer
                .userdata()
                .insert_if_missing(|| CachedDrmFramebuffer::new(DrmFramebuffer::Exporter(fb_buffer)));
        }

        let fb = primary_plane_buffer
            .userdata()
            .get::<CachedDrmFramebuffer<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>()
            .unwrap()
            .clone();

        let mut opaque_regions: Vec<Rectangle<i32, Physical>> = std::mem::take(&mut self.opaque_regions);
        std::mem::swap(&mut self.previous_element_states, &mut self.element_states);
        let mut element_states = std::mem::take(&mut self.element_states);
        element_states.reserve(std::cmp::min(elements.len(), self.planes.overlay.len()));
        let mut render_element_states = RenderElementStates {
            states: HashMap::with_capacity(elements.len()),
        };

        let mut next_frame_state: FrameState<
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        > = {
            let previous_state = self
                .pending_frame
                .as_ref()
                .map(|pending| &pending.frame)
                .unwrap_or(&self.current_frame);

            let mut next_frame_state = FrameState::from_planes(self.surface.plane(), &self.planes);

            for (handle, plane_state) in next_frame_state.planes.iter_mut() {
                let reset_state = previous_state
                    .plane_state(*handle)
                    .map(|state| state.config.is_some())
                    .unwrap_or(false);

                if reset_state {
                    plane_state.skip = false;
                }
            }

            next_frame_state
        };

        let plane_claim = self.surface.claim_plane(self.surface.plane()).ok_or_else(|| {
            error!("failed to claim primary plane");
            FrameError::PrimaryPlaneClaimFailed
        })?;
        let primary_plane_state: PlaneState<
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        > = PlaneState {
            skip: false,
            needs_test: false,
            element_state: None,
            config: Some(PlaneConfig {
                properties: PlaneProperties {
                    src: Rectangle::from_size(dmabuf.size()).to_f64(),
                    dst: Rectangle::from_size(current_size),
                    transform: Transform::Normal,
                    alpha: 1.0,
                    format: primary_plane_buffer.format(),
                },
                buffer: DrmScanoutBuffer {
                    buffer: ScanoutBuffer::Swapchain(Arc::new(primary_plane_buffer)),
                    fb,
                },
                damage_clips: None,
                plane_claim,
                sync: None,
            }),
        };

        next_frame_state.set_state(self.surface.plane(), primary_plane_state.clone());

        let mut output_elements: Vec<(&'a E, Rectangle<i32, Physical>, usize, bool)> =
            Vec::with_capacity(elements.len());

        let mut element_opaque_regions_workhouse = std::mem::take(&mut self.element_opaque_regions_workhouse);
        for (index, element) in elements.iter().enumerate() {
            let element_id = element.id();
            let element_geometry = element.geometry(output_scale);
            let element_loc = element_geometry.loc;

            let element_output_geometry = match element_geometry.intersection(output_geometry) {
                Some(geo) => geo,
                None => continue,
            };

            element_opaque_regions_workhouse.clear();
            element_opaque_regions_workhouse.push(element_output_geometry);
            element_opaque_regions_workhouse = Rectangle::subtract_rects_many_in_place(
                element_opaque_regions_workhouse,
                opaque_regions.iter().copied(),
            );
            let element_visible_area = element_opaque_regions_workhouse
                .iter()
                .fold(0usize, |acc, item| acc + (item.size.w * item.size.h) as usize);

            if element_visible_area == 0 {
                trace!("skipping completely obscured element {:?}", element.id());

                if !render_element_states.states.contains_key(element_id) {
                    render_element_states
                        .states
                        .insert(element_id.clone(), RenderElementState::skipped());
                }
                continue;
            }

            let element_opaque_regions = element.opaque_regions(output_scale);
            element_opaque_regions_workhouse.clear();
            element_opaque_regions_workhouse.push(element_output_geometry);
            element_opaque_regions_workhouse = Rectangle::subtract_rects_many_in_place(
                element_opaque_regions_workhouse,
                element_opaque_regions.iter().copied(),
            );
            let element_is_opaque = element_opaque_regions_workhouse.is_empty();

            opaque_regions.extend(
                element_opaque_regions
                    .into_iter()
                    .map(|mut region| {
                        region.loc += element_loc;
                        region
                    })
                    .filter_map(|geo| geo.intersection(output_geometry)),
            );

            if element_is_opaque && element_output_geometry.contains_rect(output_geometry) {
                let element_color = element.underlying_storage(renderer).and_then(|storage| {
                    if let UnderlyingStorage::Wayland(buffer) = storage {
                        single_pixel_buffer::get_single_pixel_buffer(buffer)
                            .ok()
                            .map(|spb| Color32F::from(spb.rgba32f()))
                    } else {
                        None
                    }
                });

                if let Some(color) = element_color {
                    clear_color = color;

                    render_element_states
                        .states
                        .entry(element_id.clone())
                        .and_modify(|state| {
                            if matches!(state.presentation_state, RenderElementPresentationState::Skipped) {
                                *state = RenderElementState::rendered(element_visible_area);
                            } else {
                                state.visible_area += element_visible_area;
                            }
                        })
                        .or_insert_with(|| RenderElementState::rendered(element_visible_area));
                } else {
                    output_elements.push((
                        element,
                        element_geometry,
                        element_visible_area,
                        element_is_opaque,
                    ));
                }

                for element in elements.iter().skip(index + 1) {
                    let element_id = element.id();
                    if !render_element_states.states.contains_key(element_id) {
                        render_element_states
                            .states
                            .insert(element_id.clone(), RenderElementState::skipped());
                    }
                }
                break;
            }

            output_elements.push((element, element_geometry, element_visible_area, element_is_opaque));
        }
        self.element_opaque_regions_workhouse = element_opaque_regions_workhouse;

        let mut primary_plane_scanout_element: Option<&'a E> = None;
        let mut primary_plane_elements: Vec<&'a E> = Vec::with_capacity(elements.len());
        let mut overlay_plane_elements: IndexMap<plane::Handle, &'a E> =
            IndexMap::with_capacity(self.planes.overlay.len());
        let mut cursor_plane_element: Option<&'a E> = None;

        let output_elements_len = output_elements.len();
        for (index, (element, element_geometry, element_visible_area, element_is_opaque)) in
            output_elements.iter().enumerate()
        {
            let element_id = element.id();
            let element_geometry = *element_geometry;
            let remaining_elements = output_elements_len - index;
            let element_is_opaque = *element_is_opaque;

            let try_assign_primary_plane = if remaining_elements == 1 && primary_plane_elements.is_empty() {
                let crtc_background_matches_clear_color =
                    (clear_color.r() == 0f32 && clear_color.g() == 0f32 && clear_color.b() == 0f32)
                        || clear_color.a() == 0f32;
                let element_spans_complete_output = element_geometry.contains_rect(output_geometry);
                let overlaps_with_underlay = self
                    .planes
                    .overlay
                    .iter()
                    .filter(|p| {
                        p.zpos.unwrap_or_default() < self.surface.plane_info().zpos.unwrap_or_default()
                    })
                    .any(|p| next_frame_state.overlaps(p.handle, element_geometry));
                !overlaps_with_underlay
                    && (crtc_background_matches_clear_color
                        || (element_spans_complete_output && element_is_opaque))
            } else {
                false
            };

            match self.try_assign_element(
                renderer,
                *element,
                index,
                element_geometry,
                element_is_opaque,
                &mut element_states,
                &primary_plane_elements,
                output_scale,
                &mut next_frame_state,
                output_transform,
                output_geometry,
                try_assign_primary_plane,
                frame_flags,
            ) {
                Ok(direct_scan_out_plane) => {
                    match direct_scan_out_plane.type_ {
                        drm::control::PlaneType::Overlay => {
                            overlay_plane_elements.insert(direct_scan_out_plane.handle, element);
                        }
                        drm::control::PlaneType::Primary => primary_plane_scanout_element = Some(element),
                        drm::control::PlaneType::Cursor => cursor_plane_element = Some(element),
                    }

                    if let Some(state) = render_element_states.states.get_mut(element_id) {
                        state.presentation_state = RenderElementPresentationState::ZeroCopy;
                        state.visible_area += element_visible_area;
                    } else {
                        render_element_states.states.insert(
                            element_id.clone(),
                            RenderElementState::zero_copy(*element_visible_area),
                        );
                    }
                }
                Err(reason) => {
                    if let Some(reason) = reason {
                        if !render_element_states.states.contains_key(element_id) {
                            render_element_states.states.insert(
                                element_id.clone(),
                                RenderElementState::rendering_with_reason(reason),
                            );
                        }
                    }

                    primary_plane_elements.push(element);
                }
            }
        }

        for element_state in element_states.values_mut() {
            element_state.fb_cache.cleanup();
        }
        self.element_states = element_states;
        self.previous_element_states.clear();
        opaque_regions.clear();
        self.opaque_regions = opaque_regions;

        let previous_state = self
            .pending_frame
            .as_ref()
            .map(|pending| &pending.frame)
            .unwrap_or(&self.current_frame);

        if next_frame_state
            .test_state_complete(
                previous_state,
                &self.surface,
                self.supports_fencing,
                false,
                allow_partial_update,
            )
            .is_err()
        {
            trace!("atomic test failed for frame, resetting frame");

            let mut removed_overlay_elements: Vec<(usize, &E)> = Vec::with_capacity(
                next_frame_state
                    .planes
                    .iter()
                    .filter(|(_, state)| state.needs_test)
                    .count(),
            );
            for (plane, state) in next_frame_state.planes.iter_mut() {
                if !state.needs_test {
                    continue;
                }

                let element = if *plane == self.surface.plane() {
                    primary_plane_scanout_element.take()
                } else if self.planes.cursor.iter().any(|p| *plane == p.handle) {
                    cursor_plane_element.take()
                } else {
                    overlay_plane_elements.shift_remove(plane)
                };

                let Some(element) = element else {
                    continue;
                };

                state.config = None;
                state.skip = false;
                state.needs_test = false;
                let element_z_index = state.element_state.take().map(|s| s.z_index).unwrap_or_default();
                removed_overlay_elements.push((element_z_index, element));
                render_element_states.states.remove(element.id());
            }

            if !removed_overlay_elements.is_empty() {
                next_frame_state.set_state(self.surface.plane(), primary_plane_state);
            }

            removed_overlay_elements.sort_by_key(|(z_index, _)| *z_index);
            primary_plane_elements = removed_overlay_elements
                .into_iter()
                .map(|(_, element)| element)
                .chain(primary_plane_elements.into_iter())
                .collect();
        }

        for (handle, previous_plane_state) in previous_state.planes.iter() {
            if previous_plane_state.config.is_some()
                && next_frame_state
                    .plane_state(*handle)
                    .as_ref()
                    .and_then(|state| state.config.as_ref())
                    .is_none()
            {
                self.overlay_plane_element_ids.remove_plane(handle);
            }
        }

        let render = next_frame_state
            .plane_buffer(self.surface.plane())
            .map(|config| matches!(config.buffer, ScanoutBuffer::Swapchain(_)))
            .unwrap_or(false);

        if render {
            trace!(
                "rendering {} elements on the primary {:?}",
                primary_plane_elements.len(),
                self.surface.plane(),
            );
            let (mut dmabuf, age) = {
                let primary_plane_state = next_frame_state.plane_state(self.surface.plane()).unwrap();
                let config = primary_plane_state.config.as_ref().unwrap();
                let slot = match &config.buffer.buffer {
                    ScanoutBuffer::Swapchain(slot) => slot,
                    _ => unreachable!(),
                };

                let dmabuf = slot.export().map_err(FrameError::AsDmabufError)?;
                let age = slot.age().into();
                (dmabuf, age)
            };

            let renderer_debug_flags = renderer.debug_flags();
            renderer.set_debug_flags(self.debug_flags);

            let overlay_plane_elements = overlay_plane_elements.iter().filter_map(|(p, element)| {
                let id = self
                    .overlay_plane_element_ids
                    .plane_id_for_element_id(p, element.id());

                let plane_z_pos = self
                    .planes
                    .overlay
                    .iter()
                    .find_map(|info| {
                        if info.handle == *p {
                            Some(info.zpos.unwrap_or_default())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();
                let is_underlay = plane_z_pos < self.surface.plane_info().zpos.unwrap_or_default();
                if is_underlay {
                    Some(HolepunchRenderElement::from_render_element(id, element, output_scale).into())
                } else {
                    OverlayPlaneElement::from_render_element(id, *element, output_scale)
                        .map(DrmRenderElements::from)
                }
            });
            let elements = overlay_plane_elements
                .chain(
                    primary_plane_elements
                        .into_iter()
                        .map(|e| DrmRenderElements::Other(e)),
                )
                .collect::<Vec<_>>();

            let mut framebuffer = renderer
                .bind(&mut dmabuf)
                .map_err(|err| RenderFrameError::RenderFrame(OutputDamageTrackerError::Rendering(err)))?;
            let render_res =
                self.damage_tracker
                    .render_output(renderer, &mut framebuffer, age, &elements, clear_color);

            renderer.set_debug_flags(renderer_debug_flags);

            match render_res {
                Ok(render_output_result) => {
                    if render_output_result.damage.is_none() {
                        let _ = renderer.cleanup_texture_cache();
                    }

                    for (id, state) in render_output_result.states.states.into_iter() {
                        if self.overlay_plane_element_ids.contains_plane_id(&id) {
                            continue;
                        }

                        if let Some(existing_state) = render_element_states.states.get_mut(&id) {
                            if matches!(
                                existing_state.presentation_state,
                                RenderElementPresentationState::Skipped
                            ) {
                                *existing_state = state;
                            } else {
                                existing_state.visible_area += state.visible_area;
                            }
                        } else {
                            render_element_states.states.insert(id.clone(), state);
                        }
                    }

                    let had_direct_scan_out = previous_state
                        .plane_state(self.surface.plane())
                        .map(|state| state.element_state.is_some())
                        .unwrap_or(true);

                    let primary_plane_state = next_frame_state.plane_state_mut(self.surface.plane()).unwrap();
                    let config = primary_plane_state.config.as_mut().unwrap();

                    if !had_direct_scan_out {
                        if let Some(render_damage) = render_output_result.damage {
                            trace!("rendering damage: {:?}", render_damage);

                            self.primary_plane_damage_bag.add(render_damage.iter().map(|d| {
                                d.to_logical(1).to_buffer(
                                    1,
                                    Transform::Normal,
                                    &output_geometry.size.to_logical(1),
                                )
                            }));
                            config.damage_clips = PlaneDamageClips::from_damage(
                                self.surface.device_fd(),
                                config.properties.src,
                                config.properties.dst,
                                render_damage.iter().copied(),
                            )
                            .ok()
                            .flatten();
                            config.sync = Some((render_output_result.sync.clone(), None));
                        } else {
                            trace!("skipping primary plane, no damage");

                            primary_plane_state.skip = true;
                            *config = previous_state
                                .plane_state(self.surface.plane())
                                .and_then(|state| state.config.as_ref().cloned())
                                .unwrap_or_else(|| config.clone());
                        }
                    } else {
                        trace!(
                            "clearing previous direct scan-out on primary plane, damaging complete output"
                        );
                        self.primary_plane_damage_bag
                            .add([output_geometry.to_logical(1).to_buffer(
                                1,
                                Transform::Normal,
                                &output_geometry.size.to_logical(1),
                            )]);

                        config.sync = Some((render_output_result.sync.clone(), None));
                    }
                }
                Err(err) => {
                    self.swapchain.reset_buffers();
                    return Err(RenderFrameError::from(err));
                }
            }
        } else {
            let _ = renderer.cleanup_texture_cache();
        }

        let primary_plane_element = if render {
            let (slot, sync) = {
                let primary_plane_state = next_frame_state.plane_state(self.surface.plane()).unwrap();
                let config = primary_plane_state.config.as_ref().unwrap();
                (
                    config.buffer.clone(),
                    config
                        .sync
                        .as_ref()
                        .map(|(sync, _)| sync.clone())
                        .unwrap_or_default(),
                )
            };

            PrimaryPlaneElement::Swapchain(PrimarySwapchainElement {
                slot,
                transform: output_transform,
                damage: self.primary_plane_damage_bag.snapshot(),
                sync,
            })
        } else {
            PrimaryPlaneElement::Element(primary_plane_scanout_element.unwrap())
        };

        if frame_flags.contains(FrameFlags::SKIP_CURSOR_ONLY_UPDATES)
            && allow_partial_update
            && next_frame_state.planes.iter().all(|(plane, state)| {
                state.skip
                    || (self.planes.cursor.iter().any(|p| *plane == p.handle)
                        && state.buffer().map(|b| &b.fb)
                            == previous_state.plane_buffer(*plane).map(|b| &b.fb))
            })
        {
            for plane in self.planes.cursor.iter() {
                let Some(state) = next_frame_state.plane_state_mut(plane.handle) else {
                    continue;
                };
                state.skip = true;
            }
        }

        let next_frame = PreparedFrame {
            kind: if allow_partial_update {
                PreparedFrameKind::Partial
            } else {
                PreparedFrameKind::Full
            },
            frame: next_frame_state,
        };
        let frame_reference: RenderFrameResult<'a, A::Buffer, F::Framebuffer, E> = RenderFrameResult {
            is_empty: next_frame.is_empty(),
            primary_element: primary_plane_element,
            overlay_elements: overlay_plane_elements.into_values().collect(),
            cursor_element: cursor_plane_element,
            states: render_element_states,
            primary_plane_element_id: self.primary_plane_element_id.clone(),
            supports_fencing: self.supports_fencing,
        };

        if !next_frame.is_empty() {
            self.next_frame = Some(next_frame);
        }

        Ok(frame_reference)
    }

    #[profiling::function]
    pub fn queue_frame(&mut self, user_data: U) -> FrameResult<(), A, F> {
        if !self.surface.is_active() {
            return Err(FrameErrorType::<A, F>::DrmError(DrmError::DeviceInactive));
        }

        let prepared_frame = self.next_frame.take().ok_or(FrameErrorType::<A, F>::EmptyFrame)?;
        if prepared_frame.is_empty() {
            return Err(FrameErrorType::<A, F>::EmptyFrame);
        }

        if let Some(plane_state) = prepared_frame.frame.plane_state(self.surface.plane()) {
            if !plane_state.skip {
                let slot = plane_state.buffer().and_then(|config| match &config.buffer {
                    ScanoutBuffer::Swapchain(slot) => Some(slot),
                    _ => None,
                });

                if let Some(slot) = slot {
                    self.swapchain.submitted(slot);
                }
            }
        }

        self.queued_frame = Some(QueuedFrame {
            prepared_frame,
            user_data,
        });
        if self.pending_frame.is_none() {
            self.submit()?;
        }
        Ok(())
    }

    pub fn commit_frame(&mut self) -> FrameResult<(), A, F> {
        if !self.surface.is_active() {
            return Err(FrameErrorType::<A, F>::DrmError(DrmError::DeviceInactive));
        }

        let mut prepared_frame = self.next_frame.take().ok_or(FrameErrorType::<A, F>::EmptyFrame)?;
        if prepared_frame.is_empty() {
            return Err(FrameErrorType::<A, F>::EmptyFrame);
        }

        if let Some(plane_state) = prepared_frame.frame.plane_state(self.surface.plane()) {
            if !plane_state.skip {
                let slot = plane_state.buffer().and_then(|config| match &config.buffer {
                    ScanoutBuffer::Swapchain(slot) => Some(slot),
                    _ => None,
                });

                if let Some(slot) = slot {
                    self.swapchain.submitted(slot);
                }
            }
        }

        let flip = prepared_frame
            .frame
            .commit(&self.surface, self.supports_fencing, false, false);

        if flip.is_ok() {
            self.queued_frame = None;
            self.pending_frame = None;
        }

        self.handle_flip(prepared_frame, None, flip)
    }

    pub fn reset_state(&mut self) -> Result<(), DrmError> {
        self.surface.reset_state()?;
        self.reset_pending = true;
        Ok(())
    }

    #[profiling::function]
    fn submit(&mut self) -> FrameResult<(), A, F> {
        let QueuedFrame {
            mut prepared_frame,
            user_data,
        } = self.queued_frame.take().unwrap();

        let allow_partial_update = prepared_frame.kind == PreparedFrameKind::Partial;
        let flip = if self.surface.commit_pending() {
            prepared_frame
                .frame
                .commit(&self.surface, self.supports_fencing, allow_partial_update, true)
        } else {
            prepared_frame
                .frame
                .page_flip(&self.surface, self.supports_fencing, allow_partial_update, true)
        };

        self.handle_flip(prepared_frame, Some(user_data), flip)
    }

    fn handle_flip(
        &mut self,
        prepared_frame: PreparedFrame<A, F>,
        user_data: Option<U>,
        flip: Result<(), crate::backend::drm::error::Error>,
    ) -> FrameResult<(), A, F> {
        match flip {
            Ok(_) => {
                if prepared_frame.kind == PreparedFrameKind::Full {
                    self.reset_pending = false;
                }

                self.pending_frame = user_data.map(|user_data| PendingFrame {
                    frame: prepared_frame.frame,
                    user_data,
                });
            }
            Err(crate::backend::drm::error::Error::Access(ref access))
                if access.source.kind() == ErrorKind::InvalidInput =>
            {
                let primary_plane_element_state = prepared_frame
                    .frame
                    .plane_state(self.surface.plane())
                    .and_then(|plane_state| {
                        plane_state
                            .element_state
                            .as_ref()
                            .map(|element_state| &element_state.id)
                    })
                    .and_then(|primary_plane_element_id| {
                        self.element_states.get_mut(primary_plane_element_id)
                    });

                if let Some(primary_plane_element_state) = primary_plane_element_state {
                    for instance in primary_plane_element_state.instances.iter_mut() {
                        instance.failed_planes.primary = true;
                    }
                }
            }
            Err(_) => {}
        };

        flip.map_err(FrameError::DrmError)
    }

    #[profiling::function]
    pub fn frame_submitted(&mut self) -> FrameResult<Option<U>, A, F> {
        if let Some(PendingFrame { mut frame, user_data }) = self.pending_frame.take() {
            std::mem::swap(&mut frame, &mut self.current_frame);
            if self.queued_frame.is_some() {
                self.submit()?;
            }
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    }

    pub fn reset_buffers(&mut self) {
        self.swapchain.reset_buffers();
    }

    pub fn reset_buffer_ages(&mut self) {
        self.swapchain.reset_buffer_ages();
    }

    pub fn crtc(&self) -> crtc::Handle {
        self.surface.crtc()
    }

    pub fn plane(&self) -> plane::Handle {
        self.surface.plane()
    }

    pub fn current_connectors(&self) -> impl IntoIterator<Item = connector::Handle> {
        self.surface.current_connectors()
    }

    pub fn pending_connectors(&self) -> impl IntoIterator<Item = connector::Handle> {
        self.surface.pending_connectors()
    }

    pub fn add_connector(&self, connector: connector::Handle) -> FrameResult<(), A, F> {
        self.surface
            .add_connector(connector)
            .map_err(FrameError::DrmError)
    }

    pub fn remove_connector(&self, connector: connector::Handle) -> FrameResult<(), A, F> {
        self.surface
            .remove_connector(connector)
            .map_err(FrameError::DrmError)
    }

    pub fn set_connectors(&self, connectors: &[connector::Handle]) -> FrameResult<(), A, F> {
        self.surface
            .set_connectors(connectors)
            .map_err(FrameError::DrmError)
    }

    pub fn current_mode(&self) -> Mode {
        self.surface.current_mode()
    }

    pub fn pending_mode(&self) -> Mode {
        self.surface.pending_mode()
    }

    pub fn use_mode(&mut self, mode: Mode) -> FrameResult<(), A, F> {
        self.surface.use_mode(mode).map_err(FrameError::DrmError)?;
        let (w, h) = mode.size();
        self.swapchain.resize(w as _, h as _);
        Ok(())
    }

    pub fn vrr_supported(&self, conn: connector::Handle) -> FrameResult<VrrSupport, A, F> {
        self.surface.vrr_supported(conn).map_err(FrameError::DrmError)
    }

    pub fn vrr_enabled(&self) -> bool {
        self.surface.vrr_enabled()
    }

    pub fn use_vrr(&mut self, vrr: bool) -> FrameResult<(), A, F> {
        self.surface.use_vrr(vrr).map_err(FrameError::DrmError)
    }

    pub fn set_debug_flags(&mut self, flags: DebugFlags) {
        if self.debug_flags != flags {
            self.debug_flags = flags;
            self.swapchain.reset_buffers();
        }
    }

    pub fn debug_flags(&self) -> DebugFlags {
        self.debug_flags
    }

    pub fn surface(&self) -> &DrmSurface {
        &self.surface
    }

    pub fn format(&self) -> DrmFourcc {
        self.swapchain.format()
    }

    pub fn modifiers(&self) -> &[DrmModifier] {
        self.swapchain.modifiers()
    }

    pub fn set_format(
        &mut self,
        allocator: A,
        code: DrmFourcc,
        modifiers: impl IntoIterator<Item = DrmModifier>,
    ) -> Result<(), FrameErrorType<A, F>> {
        let (swapchain, is_oapque) = Self::test_format(
            &self.surface,
            self.supports_fencing,
            &self.planes,
            allocator,
            &self.framebuffer_exporter,
            code,
            modifiers,
        )
        .map_err(|(_, err)| err)?;

        self.swapchain = swapchain;
        self.primary_is_opaque = is_oapque;

        Ok(())
    }

    pub fn set_output_mode_source(&mut self, output_mode_source: OutputModeSource) {
        if output_mode_source == self.output_mode_source {
            return;
        }

        self.damage_tracker = OutputDamageTracker::from_mode_source(output_mode_source.clone());
        self.output_mode_source = output_mode_source;
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn try_assign_element<'a, R, E>(
        &mut self,
        renderer: &mut R,
        element: &'a E,
        element_zindex: usize,
        element_geometry: Rectangle<i32, Physical>,
        element_is_opaque: bool,
        element_states: &mut IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
        primary_plane_elements: &[&'a E],
        scale: Scale<f64>,
        frame_state: &mut CompositorFrameState<A, F>,
        output_transform: Transform,
        output_geometry: Rectangle<i32, Physical>,
        try_assign_primary_plane: bool,
        frame_flags: FrameFlags,
    ) -> Result<PlaneAssignment, Option<RenderingReason>>
    where
        R: Renderer + Bind<Dmabuf>,
        E: RenderElement<R>,
    {
        if !frame_flags.intersects(FrameFlags::ALLOW_SCANOUT) {
            trace!(
                "skipping direct scan-out for element {:?}, no free planes",
                element.id()
            );
            return Err(None);
        };

        let mut rendering_reason: Option<RenderingReason> = None;

        if try_assign_primary_plane {
            match self.try_assign_primary_plane(
                renderer,
                element,
                element_zindex,
                element_geometry,
                element_states,
                scale,
                frame_state,
                output_transform,
                output_geometry,
                frame_flags,
            ) {
                Ok(plane) => {
                    trace!(
                        "assigned element {:?} to primary {:?}",
                        element.id(),
                        self.surface.plane()
                    );
                    return Ok(plane);
                }
                Err(err) => rendering_reason = rendering_reason.or(err),
            };
        }

        if let Some(plane) = self.try_assign_cursor_plane(
            renderer,
            element,
            element_zindex,
            element_geometry,
            scale,
            frame_state,
            output_transform,
            output_geometry,
            frame_flags,
        ) {
            trace!("assigned element {:?} to cursor {:?}", element.id(), plane.handle);
            return Ok(plane);
        }

        match self.try_assign_overlay_plane(
            renderer,
            element,
            element_zindex,
            element_geometry,
            element_is_opaque,
            element_states,
            primary_plane_elements,
            scale,
            frame_state,
            output_transform,
            output_geometry,
            frame_flags,
        ) {
            Ok(plane) => {
                trace!(
                    "assigned element {:?} to overlay plane {:?}",
                    element.id(),
                    plane.handle
                );
                return Ok(plane);
            }
            Err(err) => rendering_reason = rendering_reason.or(err),
        }

        Err(rendering_reason)
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn try_assign_primary_plane<'a, R, E>(
        &mut self,
        renderer: &mut R,
        element: &'a E,
        element_zindex: usize,
        element_geometry: Rectangle<i32, Physical>,
        element_states: &mut IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
        scale: Scale<f64>,
        frame_state: &mut CompositorFrameState<A, F>,
        output_transform: Transform,
        output_geometry: Rectangle<i32, Physical>,
        frame_flags: FrameFlags,
    ) -> Result<PlaneAssignment, Option<RenderingReason>>
    where
        R: Renderer,
        E: RenderElement<R>,
    {
        if !frame_flags
            .intersects(FrameFlags::ALLOW_PRIMARY_PLANE_SCANOUT | FrameFlags::ALLOW_PRIMARY_PLANE_SCANOUT_ANY)
        {
            return Err(None);
        }

        if frame_state
            .plane_state(self.surface.plane())
            .map(|state| state.element_state.is_some())
            .unwrap_or(true)
        {
            return Err(None);
        }

        let element_config = self.element_config(
            renderer,
            element,
            element_zindex,
            element_geometry,
            element_states,
            frame_state,
            output_transform,
            output_geometry,
            true,
        )?;

        if let ScanoutBuffer::Swapchain(slot) = &frame_state
            .plane_buffer(self.surface.plane())
            .expect("We have a buffer for the primary plane")
            .buffer
        {
            if !frame_flags.contains(FrameFlags::ALLOW_PRIMARY_PLANE_SCANOUT_ANY)
                && slot.format() != element_config.properties.format
            {
                trace!(
                    "failed to assign element {:?} to primary {:?}, format doesn't match",
                    element.id(),
                    self.surface.plane()
                );
                return Err(None);
            }
        }

        let has_underlay = self
            .planes
            .overlay
            .iter()
            .filter(|plane| {
                self.surface.plane_info().zpos.unwrap_or_default() > plane.zpos.unwrap_or_default()
            })
            .any(|plane| frame_state.is_assigned(plane.handle));

        if has_underlay {
            trace!(
                "failed to assign element {:?} to primary {:?}, already has underlay",
                element.id(),
                self.surface.plane()
            );
            return Err(None);
        }

        if element_config.failed_planes.primary {
            return Err(Some(RenderingReason::ScanoutFailed));
        }

        let res = self.try_assign_plane(
            element,
            &element_config,
            self.surface.plane_info(),
            scale,
            frame_state,
        );

        if let Err(Some(RenderingReason::ScanoutFailed)) = res {
            element_config.failed_planes.primary = true;
        }

        res
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn try_assign_cursor_plane<R, E>(
        &mut self,
        renderer: &mut R,
        element: &E,
        element_zindex: usize,
        element_geometry: Rectangle<i32, Physical>,
        scale: Scale<f64>,
        frame_state: &mut CompositorFrameState<A, F>,
        output_transform: Transform,
        output_geometry: Rectangle<i32, Physical>,
        frame_flags: FrameFlags,
    ) -> Option<PlaneAssignment>
    where
        R: Renderer,
        E: RenderElement<R>,
    {
        if !frame_flags.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT) {
            return None;
        }

        let Some(cursor_state) = self.cursor_state.as_mut() else {
            trace!("no cursor state, skipping cursor rendering");
            return None;
        };

        if element.kind() != Kind::Cursor {
            trace!(
                "skipping element {:?} on cursor plane(s), element kind not cursor",
                element.id(),
            );
            return None;
        }

        let element_size = output_transform.transform_size(element_geometry.size);

        if element_size.w > self.cursor_size.w || element_size.h > self.cursor_size.h {
            trace!("element {:?} too big for cursor plane(s), skipping", element.id(),);
            return None;
        }

        if let Some(plane_info) = self
            .planes
            .cursor
            .iter()
            .find(|plane_info| frame_state.is_assigned(plane_info.handle))
        {
            trace!(
                "skipping element {:?} on cursor {:?}, plane already has element assigned",
                element.id(),
                plane_info.handle
            );
            return None;
        }

        let previous_state = self
            .pending_frame
            .as_ref()
            .map(|pending| &pending.frame)
            .unwrap_or(&self.current_frame);

        let Some((plane_info, plane_claim)) = self
            .planes
            .cursor
            .iter()
            .find_map(|plane_info: &PlaneInfo| {
                if previous_state.is_assigned(plane_info.handle) {
                    self.surface
                        .claim_plane(plane_info.handle)
                        .map(|claim| (plane_info, claim))
                } else {
                    None
                }
            })
            .or_else(|| {
                self.planes.cursor.iter().find_map(|plane_info| {
                    self.surface
                        .claim_plane(plane_info.handle)
                        .map(|claim| (plane_info, claim))
                })
            })
        else {
            trace!(
                "skipping element {:?} on cursor plane(s), no free plane found",
                element.id(),
            );
            return None;
        };

        let cursor_plane_size = if let Some(size_hints) = plane_info.size_hints.as_deref() {
            size_hints
                .iter()
                .find(|hint| hint.w as i32 >= element_size.w && hint.h as i32 >= element_size.h)
                .map(|hint| Size::<i32, Physical>::from((hint.w as i32, hint.h as i32)))
                .unwrap_or(self.cursor_size)
        } else {
            self.cursor_size
        };

        let cursor_plane_location = output_transform
            .transform_point_in(element.location(scale), &output_geometry.size)
            - output_transform.transform_point_in(Point::default(), &cursor_plane_size);

        let previous_state = self
            .pending_frame
            .as_ref()
            .map(|pending| &pending.frame)
            .unwrap_or(&self.current_frame);

        let previous_element_state = previous_state
            .plane_state(plane_info.handle)
            .and_then(|state| state.element_state.as_ref());

        let render = cursor_state
            .previous_output_transform
            .map(|t| t != output_transform)
            .unwrap_or(true)
            || cursor_state
                .previous_output_scale
                .map(|s| s != scale)
                .unwrap_or(true)
            || previous_element_state
                .map(|element_state| {
                    element_state.id != *element.id()
                        || element.current_commit() != element_state.commit
                        || element_state.cursor_size != Some(element_size)
                })
                .unwrap_or(true)
            || previous_state
                .plane_state(plane_info.handle)
                .and_then(|state| {
                    state
                        .config
                        .as_ref()
                        .map(|config| config.properties.dst.size != cursor_plane_size)
                })
                .unwrap_or(true);

        let reposition = previous_state
            .plane_state(plane_info.handle)
            .and_then(|state| {
                state
                    .config
                    .as_ref()
                    .map(|config| config.properties.dst.loc != cursor_plane_location)
            })
            .unwrap_or(true);

        if !render && !reposition {
            let mut plane_state = previous_state.plane_state(plane_info.handle).unwrap().clone();
            plane_state.skip = true;
            plane_state.needs_test = false;
            frame_state.set_state(plane_info.handle, plane_state);
            return Some(plane_info.into());
        }

        if !render && reposition {
            trace!("repositioning cursor plane");
            let mut plane_state = previous_state.plane_state(plane_info.handle).unwrap().clone();
            plane_state.skip = false;
            plane_state.needs_test = false;
            let config = plane_state.config.as_mut().unwrap();
            config.properties.dst.loc = cursor_plane_location;
            frame_state.set_state(plane_info.handle, plane_state);
            return Some(plane_info.into());
        }

        trace!(
            "trying to render element {:?} on cursor {:?}",
            element.id(),
            plane_info.handle
        );

        let mut cursor_buffer = match cursor_state.allocator.create_buffer(
            cursor_plane_size.w as u32,
            cursor_plane_size.h as u32,
            DrmFourcc::Argb8888,
            &[DrmModifier::Linear],
        ) {
            Ok(buffer) => buffer,
            Err(err) => {
                debug!("failed to create cursor buffer: {}", err);
                return None;
            }
        };

        let framebuffer = match cursor_state.framebuffer_exporter.add_framebuffer(
            self.surface.device_fd(),
            ExportBuffer::Allocator(&cursor_buffer),
            false,
        ) {
            Ok(Some(fb)) => fb,
            Ok(None) => {
                debug!(
                    "failed to export framebuffer for cursor {:?}: no framebuffer available",
                    plane_info.handle
                );
                return None;
            }
            Err(err) => {
                debug!(
                    "failed to export framebuffer for cursor {:?}: {}",
                    plane_info.handle, err
                );
                return None;
            }
        };

        let cursor_buffer_size = cursor_plane_size.to_logical(1).to_buffer(1, Transform::Normal);

        #[cfg(not(feature = "renderer_pixman"))]
        if !copy_element_to_cursor_bo(
            renderer,
            element,
            element_size,
            cursor_plane_size,
            output_transform,
            &mut cursor_buffer,
        ) {
            tracing::trace!("failed to copy element to cursor bo, skipping element on cursor plane");
            return None;
        }

        #[cfg(feature = "renderer_pixman")]
        if !copy_element_to_cursor_bo(
            renderer,
            element,
            element_size,
            cursor_plane_size,
            output_transform,
            &mut cursor_buffer,
        ) {
            profiling::scope!("render cursor plane");
            tracing::trace!("cursor fast-path copy failed, falling back to rendering using offscreen buffer");

            let Some(storage) = element.underlying_storage(renderer) else {
                trace!("Can't obtain cursor's underlying storage");
                return None;
            };

            let pixman_renderer = cursor_state.pixman_renderer.as_mut()?;

            let cursor_texture = match storage {
                UnderlyingStorage::Wayland(buffer) => pixman_renderer
                    .import_buffer(buffer, None, &[element.src().to_i32_up()])
                    .transpose()
                    .ok()
                    .flatten(),
                UnderlyingStorage::Memory(memory) => {
                    let format = memory.format();
                    let size = memory.size();
                    let Ok(pixman_format) = pixman::FormatCode::try_from(format) else {
                        debug!("No pixman format for {format}");
                        return None;
                    };
                    unsafe {
                        match pixman::Image::from_raw_mut(
                            pixman_format,
                            size.w as usize,
                            size.h as usize,
                            memory.as_ptr() as *mut u32,
                            memory.stride() as usize,
                            false,
                        ) {
                            Ok(image) => Some(PixmanTexture::from(image)),
                            Err(e) => {
                                debug!("pixman cursor: {e}");
                                None
                            }
                        }
                    }
                }
            }?;

            let ret = cursor_buffer
                .map_mut::<_, Result<_, PixmanError>>(
                    0,
                    0,
                    cursor_buffer_size.w as u32,
                    cursor_buffer_size.h as u32,
                    |mbo| {
                        let plane_pixman_format = pixman::FormatCode::try_from(DrmFourcc::Argb8888).unwrap();
                        let mut cursor_dst = unsafe {
                            pixman::Image::from_raw_mut(
                                plane_pixman_format,
                                mbo.width() as usize,
                                mbo.height() as usize,
                                mbo.buffer_mut().as_mut_ptr() as *mut u32,
                                mbo.stride() as usize,
                                false,
                            )
                        }
                        .map_err(|_| PixmanError::ImportFailed)?;
                        let mut framebuffer = pixman_renderer.bind(&mut cursor_dst)?;
                        let mut frame =
                            pixman_renderer.render(&mut framebuffer, cursor_plane_size, output_transform)?;
                        frame.clear(Color32F::TRANSPARENT, &[Rectangle::from_size(cursor_plane_size)])?;
                        let src = element.src();
                        let dst = Rectangle::from_size(element_geometry.size);
                        frame.render_texture_from_to(
                            &cursor_texture,
                            src,
                            dst,
                            &[dst],
                            &[],
                            element.transform(),
                            element.alpha(),
                        )?;
                        let _ = frame.finish()?.wait();
                        Ok(())
                    },
                )
                .expect("Lost track of cursor device");

            if let Err(err) = ret {
                debug!("{err}");
                return None;
            }
        };

        let src = Rectangle::from_size(cursor_buffer_size).to_f64();
        let dst = Rectangle::new(cursor_plane_location, cursor_plane_size);

        let config = PlaneConfig {
            properties: PlaneProperties {
                src,
                dst,
                alpha: 1.0,
                transform: Transform::Normal,
                format: framebuffer.format(),
            },
            buffer: DrmScanoutBuffer {
                buffer: ScanoutBuffer::Cursor(Arc::new(cursor_buffer)),
                fb: CachedDrmFramebuffer::new(DrmFramebuffer::Gbm(framebuffer)),
            },
            damage_clips: None,
            plane_claim,
            sync: None,
        };
        let is_compatible = previous_state
            .plane_state(plane_info.handle)
            .map(|state| {
                state
                    .config
                    .as_ref()
                    .map(|other| {
                        other.properties.src == config.properties.src
                            && other.properties.dst.size == config.properties.dst.size
                            && other.properties.alpha == config.properties.alpha
                            && other.properties.transform == config.properties.transform
                            && other.properties.format == config.properties.format
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let plane_state = PlaneState {
            skip: false,
            needs_test: !is_compatible,
            element_state: Some(PlaneElementState {
                id: element.id().clone(),
                commit: element.current_commit(),
                z_index: element_zindex,
                cursor_size: Some(element_size),
            }),
            config: Some(config),
        };

        let res = if is_compatible {
            frame_state.set_state(plane_info.handle, plane_state);
            true
        } else {
            frame_state
                .test_state(
                    &self.surface,
                    self.supports_fencing,
                    plane_info.handle,
                    plane_state,
                    false,
                )
                .is_ok()
        };

        if res {
            cursor_state.previous_output_scale = Some(scale);
            cursor_state.previous_output_transform = Some(output_transform);
            Some(plane_info.into())
        } else {
            info!("failed to test cursor {:?} state", plane_info.handle);
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn element_config<'a, R, E>(
        &mut self,
        renderer: &mut R,
        element: &E,
        element_zindex: usize,
        element_geometry: Rectangle<i32, Physical>,
        element_states: &'a mut IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
        frame_state: &mut CompositorFrameState<A, F>,
        output_transform: Transform,
        output_geometry: Rectangle<i32, Physical>,
        allow_opaque_fallback: bool,
    ) -> Result<
        ElementPlaneConfig<
            'a,
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        >,
        ExportBufferError,
    >
    where
        R: Renderer,
        E: RenderElement<R>,
    {
        let element_id = element.id();

        let underlying_storage = element
            .underlying_storage(renderer)
            .ok_or(ExportBufferError::NoUnderlyingStorage)?;

        let export_buffer = ExportBuffer::from_underlying_storage(&underlying_storage)
            .ok_or(ExportBufferError::Unsupported)?;

        if !self.framebuffer_exporter.can_add_framebuffer(&export_buffer) {
            return Err(ExportBufferError::Unsupported);
        }

        if !element_states.contains_key(element_id) {
            let previous_fb_cache = self
                .previous_element_states
                .get_mut(element_id)
                .map(|state| std::mem::take(&mut state.fb_cache))
                .unwrap_or_default();
            element_states.insert(
                element_id.clone(),
                ElementState {
                    instances: SmallVec::new(),
                    fb_cache: previous_fb_cache,
                },
            );
        }
        let element_fb_cache: &mut ElementFramebufferCache<
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        > = element_states
            .get_mut(element_id)
            .map(|state| &mut state.fb_cache)
            .unwrap();

        let element_cache_key =
            ElementFramebufferCacheKey::from_underlying_storage(&underlying_storage, allow_opaque_fallback)
                .ok_or(ExportBufferError::Unsupported)?;
        let cached_fb = element_fb_cache.get(&element_cache_key);

        if cached_fb.is_none() {
            trace!(
                "no cached fb, exporting new fb for element {:?} underlying storage {:?}",
                element_id,
                &underlying_storage
            );

            let fb = self
                .framebuffer_exporter
                .add_framebuffer(self.surface.device_fd(), export_buffer, allow_opaque_fallback)
                .map_err(|err| {
                    trace!("failed to add framebuffer: {:?}", err);
                    ExportBufferError::ExportFailed
                })
                .and_then(|fb| {
                    fb.map(|fb| CachedDrmFramebuffer::new(DrmFramebuffer::Exporter(fb)))
                        .ok_or(ExportBufferError::Unsupported)
                });

            if fb.is_err() {
                trace!(
                    "could not import framebuffer for element {:?} underlying storage {:?}",
                    element_id,
                    &underlying_storage
                );
            }

            element_fb_cache.insert(element_cache_key.clone(), fb);
        } else {
            trace!(
                "using cached fb for element {:?} underlying storage {:?}",
                element_id,
                &underlying_storage
            );
        }

        let fb: &CachedDrmFramebuffer<<F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer> =
            element_fb_cache.get(&element_cache_key).unwrap()?;

        let src = element.src();
        let dst = output_transform.transform_rect_in(element_geometry, &output_geometry.size);
        let transform = apply_output_transform(
            apply_underlying_storage_transform(element.transform(), &underlying_storage),
            output_transform.invert(),
        );
        let alpha = element.alpha();
        let properties = PlaneProperties {
            src,
            dst,
            alpha,
            transform,
            format: fb.format(),
        };
        let buffer: DrmScanoutBuffer<
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        > = ScanoutBuffer::from_underlying_storage(underlying_storage)
            .map(|buffer| DrmScanoutBuffer {
                fb: fb.clone(),
                buffer,
            })
            .ok_or(ExportBufferError::Unsupported)?;

        if !element_states
            .get(element_id)
            .unwrap()
            .instances
            .iter()
            .any(|i| i.properties == properties)
        {
            let overlay_bitmask =
                self.planes
                    .overlay
                    .iter()
                    .enumerate()
                    .fold(0u32, |mut acc, (index, plane)| {
                        if frame_state.is_assigned(plane.handle) {
                            acc |= 1 << index;
                        }
                        acc
                    });
            let cursor_bitmask =
                self.planes
                    .cursor
                    .iter()
                    .enumerate()
                    .fold(0u32, |mut acc, (index, plane)| {
                        if frame_state.is_assigned(plane.handle) {
                            acc |= 1 << index;
                        }
                        acc
                    });
            let current_plane_snapshot = PlanesSnapshot {
                primary: frame_state.is_assigned(self.surface.plane()),
                cursor_bitmask,
                overlay_bitmask,
            };

            let element_state = element_states.get_mut(element_id).unwrap();
            element_state.instances.push(ElementInstanceState {
                properties,
                active_planes: current_plane_snapshot,
                failed_planes: Default::default(),
            });

            if let Some(previous_state) = self.previous_element_states.get(element_id) {
                let matching_instance = previous_state
                    .instances
                    .iter()
                    .find(|i| i.properties == properties);

                if let Some(matching_instance) = matching_instance {
                    if current_plane_snapshot == matching_instance.active_planes {
                        let previous_frame_state = self
                            .pending_frame
                            .as_ref()
                            .map(|pending| &pending.frame)
                            .unwrap_or(&self.current_frame);

                        let primary_plane_changed = if current_plane_snapshot.primary {
                            frame_state.plane_properties(self.surface.plane())
                                != previous_frame_state.plane_properties(self.surface.plane())
                        } else {
                            false
                        };

                        let overlay_plane_changed =
                            self.planes.overlay.iter().enumerate().any(|(index, plane)| {
                                if current_plane_snapshot.overlay_bitmask & (1 << index) == 0 {
                                    return false;
                                }

                                frame_state.plane_properties(plane.handle)
                                    != previous_frame_state.plane_properties(plane.handle)
                            });

                        if !(primary_plane_changed || overlay_plane_changed) {
                            let instance_state = element_state
                                .instances
                                .iter_mut()
                                .find(|i| i.properties == properties)
                                .unwrap();
                            instance_state.failed_planes = matching_instance.failed_planes;
                        }
                    }
                }
            }
        }

        let failed_planes = element_states
            .get_mut(element_id)
            .unwrap()
            .instances
            .iter_mut()
            .find_map(|i| {
                if i.properties == properties {
                    Some(&mut i.failed_planes)
                } else {
                    None
                }
            })
            .unwrap();

        Ok(ElementPlaneConfig {
            properties,
            z_index: element_zindex,
            geometry: element_geometry,
            buffer,
            failed_planes,
        })
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn try_assign_overlay_plane<'a, R, E>(
        &mut self,
        renderer: &mut R,
        element: &'a E,
        element_zindex: usize,
        element_geometry: Rectangle<i32, Physical>,
        element_is_opaque: bool,
        element_states: &mut IndexMap<Id, ElementState<<F as ExportFramebuffer<A::Buffer>>::Framebuffer>>,
        primary_plane_elements: &[&'a E],
        scale: Scale<f64>,
        frame_state: &mut CompositorFrameState<A, F>,
        output_transform: Transform,
        output_geometry: Rectangle<i32, Physical>,
        frame_flags: FrameFlags,
    ) -> Result<PlaneAssignment, Option<RenderingReason>>
    where
        R: Renderer,
        E: RenderElement<R>,
    {
        if !frame_flags.contains(FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT) {
            return Err(None);
        }

        let element_id = element.id();

        if self
            .planes
            .overlay
            .iter()
            .all(|plane| frame_state.is_assigned(plane.handle))
        {
            trace!(
                "skipping overlay planes for element {:?}, no free planes",
                element_id
            );
            return Err(None);
        }

        let element_config = self.element_config(
            renderer,
            element,
            element_zindex,
            element_geometry,
            element_states,
            frame_state,
            output_transform,
            output_geometry,
            false,
        )?;

        let overlaps_with_primary_plane_element = primary_plane_elements.iter().any(|e| {
            let other_geometry = e.geometry(scale);
            other_geometry.overlaps(element_config.geometry)
        });

        let primary_plane_has_alpha = frame_state
            .plane_buffer(self.surface.plane())
            .map(|state| has_alpha(state.format().code))
            .unwrap_or(false);

        let previous_frame_state = self
            .pending_frame
            .as_ref()
            .map(|pending| &pending.frame)
            .unwrap_or(&self.current_frame);

        let is_plane_compatible = |plane: &&PlaneInfo| {
            previous_frame_state
                .plane_state(plane.handle)
                .map(|state| {
                    state
                        .element_state
                        .as_ref()
                        .map(|state| state.z_index <= element_config.z_index)
                        .unwrap_or(false)
                        && state
                            .config
                            .as_ref()
                            .map(|config| config.properties.is_compatible(&element_config.properties))
                            .unwrap_or(false)
                })
                .unwrap_or(false)
        };

        let mut test_overlay_plane = |plane: &PlaneInfo,
                                      element_config: &ElementPlaneConfig<
            '_,
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        >| {
            if frame_state.is_assigned(plane.handle) {
                trace!(
                    "skipping {:?} with zpos {:?} for element {:?}, already has element assigned, skipping",
                    plane.handle,
                    plane.zpos,
                    element_id,
                );
                return Err(None);
            }

            let is_underlay =
                self.surface.plane_info().zpos.unwrap_or_default() > plane.zpos.unwrap_or_default();

            if is_underlay && !(element_is_opaque && primary_plane_has_alpha) {
                trace!(
                    "skipping direct scan-out on underlay {:?} with zpos {:?}, element {:?} is not opaque or primary plane has no alpha channel",
                    plane.handle,
                    plane.zpos,
                    element_id
                );
                return Err(None);
            }

            if overlaps_with_primary_plane_element && !is_underlay {
                trace!(
                    "skipping direct scan-out on {:?} with zpos {:?}, element {:?} overlaps with element on primary plane", plane.handle, plane.zpos, element_id,
                );
                return Err(None);
            }

            let overlaps_with_plane_underneath = self
                .planes
                .overlay
                .iter()
                .filter(|info| {
                    info.handle != plane.handle
                        && info.zpos.unwrap_or_default() <= plane.zpos.unwrap_or_default()
                })
                .any(|overlapping_plane| {
                    frame_state.overlaps(overlapping_plane.handle, element_config.geometry)
                });

            if overlaps_with_plane_underneath {
                trace!(
                    "skipping direct scan-out on {:?} with zpos {:?}, element {:?} geometry {:?} overlaps with plane underneath", plane.handle, plane.zpos, element_id, element_config.geometry,
                );
                return Err(None);
            }

            self.try_assign_plane(element, element_config, plane, scale, frame_state)
        };

        for plane in self.planes.overlay.iter().filter(is_plane_compatible) {
            if let Ok(plane_assignment) = test_overlay_plane(plane, &element_config) {
                trace!(
                    "assigned element {:?} geometry {:?} to compatible {:?} with zpos {:?}",
                    element_id,
                    element_config.geometry,
                    plane.handle,
                    plane.zpos,
                );
                return Ok(plane_assignment);
            }
        }

        let mut rendering_reason: Option<RenderingReason> = None;
        for (index, plane) in self.planes.overlay.iter().enumerate() {
            if element_config.failed_planes.overlay_bitmask & (1 << index) != 0 {
                trace!(
                    "skipping direct scan-out on {:?} with zpos {:?}, element {:?} geometry {:?}, test already known to fail", plane.handle, plane.zpos, element_id, element_config.geometry,
                );
                rendering_reason = rendering_reason.or(Some(RenderingReason::ScanoutFailed));
                continue;
            }

            match test_overlay_plane(plane, &element_config) {
                Ok(plane) => return Ok(plane),
                Err(err) => {
                    if let Some(RenderingReason::ScanoutFailed) = err {
                        element_config.failed_planes.overlay_bitmask |= 1 << index;
                    }

                    rendering_reason = rendering_reason.or(err)
                }
            }
        }

        Err(rendering_reason)
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", skip_all)]
    #[profiling::function]
    fn try_assign_plane<R, E>(
        &self,
        element: &E,
        element_config: &ElementPlaneConfig<
            '_,
            <A as Allocator>::Buffer,
            <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer,
        >,
        plane: &PlaneInfo,
        scale: Scale<f64>,
        frame_state: &mut CompositorFrameState<A, F>,
    ) -> Result<PlaneAssignment, Option<RenderingReason>>
    where
        R: Renderer,
        E: RenderElement<R>,
    {
        let element_id = element.id();

        let plane_claim = match self.surface.claim_plane(plane.handle) {
            Some(claim) => claim,
            None => {
                trace!("failed to claim {:?} for element {:?}", plane.handle, element_id);
                return Err(None);
            }
        };

        trace!("testing direct scan-out for element {:?} on {:?} with zpos {:?}: fb: {:?}, element_geometry: {:?}", element_id, plane.handle, plane.zpos, &element_config.buffer.fb, element_config.geometry);

        if !plane.formats.contains(&element_config.properties.format) {
            trace!(
                "skipping direct scan-out on {:?} with zpos {:?} for element {:?}, format {:?} not supported",
                plane.handle,
                plane.zpos,
                element_id,
                element_config.properties.format,
            );
            return Err(Some(RenderingReason::FormatUnsupported));
        }

        let previous_state = self
            .pending_frame
            .as_ref()
            .map(|pending| &pending.frame)
            .unwrap_or(&self.current_frame);

        let previous_commit = previous_state.plane_state(plane.handle).and_then(|state| {
            state.element_state.as_ref().and_then(|state| {
                if state.id == *element_id {
                    Some(state.commit)
                } else {
                    None
                }
            })
        });

        let element_damage = element.damage_since(scale, previous_commit);
        let has_element_damage = !element_damage.is_empty();

        let damage_clips = if has_element_damage {
            PlaneDamageClips::from_damage(
                self.surface.device_fd(),
                element_config.properties.src,
                element_config.geometry,
                element_damage,
            )
            .ok()
            .flatten()
        } else {
            None
        };

        let config = PlaneConfig {
            properties: element_config.properties,
            buffer: element_config.buffer.clone(),
            damage_clips,
            plane_claim,
            sync: element_config
                .buffer
                .buffer
                .acquire_point(self.signaled_fence.as_ref()),
        };

        let is_compatible = previous_state
            .plane_state(plane.handle)
            .map(|state| {
                state
                    .config
                    .as_ref()
                    .map(|c| c.is_compatible(&config))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let skip = !has_element_damage
            && previous_state
                .plane_state(plane.handle)
                .map(|state| {
                    state
                        .config
                        .as_ref()
                        .map(|c| is_compatible && c.buffer.fb == config.buffer.fb)
                        .unwrap_or(false)
                })
                .unwrap_or(false);

        let plane_state = PlaneState {
            skip,
            needs_test: true,
            element_state: Some(PlaneElementState {
                id: element_id.clone(),
                commit: element.current_commit(),
                z_index: element_config.z_index,
                cursor_size: None,
            }),
            config: Some(config),
        };

        let res = if is_compatible {
            trace!(
                "skipping atomic test for compatible element {:?} on {:?} with zpos {:?}",
                element_id,
                plane.handle,
                plane.zpos,
            );
            frame_state.set_state(plane.handle, plane_state);
            true
        } else {
            frame_state
                .test_state(
                    &self.surface,
                    self.supports_fencing,
                    plane.handle,
                    plane_state,
                    false,
                )
                .is_ok()
        };

        if res {
            trace!(
                "successfully assigned element {:?} to {:?} with zpos {:?} for direct scan-out",
                element_id,
                plane.handle,
                plane.zpos,
            );

            Ok(plane.into())
        } else {
            trace!(
                "skipping direct scan-out on {:?} with zpos {:?} for element {:?}, test failed",
                plane.handle,
                plane.zpos,
                element_id
            );

            Err(Some(RenderingReason::ScanoutFailed))
        }
    }

    pub fn clear(&mut self) -> Result<(), DrmError> {
        self.surface.clear()?;

        self.current_frame
            .planes
            .iter_mut()
            .for_each(|(_, state)| *state = Default::default());
        self.pending_frame = None;
        self.queued_frame = None;
        self.next_frame = None;

        Ok(())
    }
}

#[inline]
fn apply_underlying_storage_transform(
    element_transform: Transform,
    storage: &UnderlyingStorage<'_>,
) -> Transform {
    match storage {
        UnderlyingStorage::Wayland(buffer) => {
            if buffer_y_inverted(buffer).unwrap_or(false) {
                match element_transform {
                    Transform::Normal => Transform::Flipped,
                    Transform::_90 => Transform::Flipped90,
                    Transform::_180 => Transform::Flipped180,
                    Transform::_270 => Transform::Flipped270,
                    Transform::Flipped => Transform::Normal,
                    Transform::Flipped90 => Transform::_90,
                    Transform::Flipped180 => Transform::_180,
                    Transform::Flipped270 => Transform::_270,
                }
            } else {
                element_transform
            }
        }
        UnderlyingStorage::Memory { .. } => element_transform,
    }
}

#[inline]
fn apply_output_transform(transform: Transform, output_transform: Transform) -> Transform {
    match (transform, output_transform) {
        (Transform::Normal, output_transform) => output_transform,

        (Transform::_90, Transform::Normal) => Transform::_270,
        (Transform::_90, Transform::_90) => Transform::Normal,
        (Transform::_90, Transform::_180) => Transform::_90,
        (Transform::_90, Transform::_270) => Transform::_180,
        (Transform::_90, Transform::Flipped) => Transform::Flipped270,
        (Transform::_90, Transform::Flipped90) => Transform::Flipped,
        (Transform::_90, Transform::Flipped180) => Transform::Flipped90,
        (Transform::_90, Transform::Flipped270) => Transform::Flipped180,

        (Transform::_180, Transform::Normal) => Transform::_180,
        (Transform::_180, Transform::_90) => Transform::_270,
        (Transform::_180, Transform::_180) => Transform::Normal,
        (Transform::_180, Transform::_270) => Transform::_90,
        (Transform::_180, Transform::Flipped) => Transform::Flipped180,
        (Transform::_180, Transform::Flipped90) => Transform::Flipped270,
        (Transform::_180, Transform::Flipped180) => Transform::Flipped,
        (Transform::_180, Transform::Flipped270) => Transform::Flipped90,

        (Transform::_270, Transform::Normal) => Transform::_90,
        (Transform::_270, Transform::_90) => Transform::_180,
        (Transform::_270, Transform::_180) => Transform::_270,
        (Transform::_270, Transform::_270) => Transform::Normal,
        (Transform::_270, Transform::Flipped) => Transform::Flipped90,
        (Transform::_270, Transform::Flipped90) => Transform::Flipped180,
        (Transform::_270, Transform::Flipped180) => Transform::Flipped270,
        (Transform::_270, Transform::Flipped270) => Transform::Flipped,

        (Transform::Flipped, Transform::Normal) => Transform::Flipped,
        (Transform::Flipped, Transform::_90) => Transform::Flipped90,
        (Transform::Flipped, Transform::_180) => Transform::Flipped180,
        (Transform::Flipped, Transform::_270) => Transform::Flipped270,
        (Transform::Flipped, Transform::Flipped) => Transform::Normal,
        (Transform::Flipped, Transform::Flipped90) => Transform::_90,
        (Transform::Flipped, Transform::Flipped180) => Transform::_180,
        (Transform::Flipped, Transform::Flipped270) => Transform::_270,

        (Transform::Flipped90, Transform::Normal) => Transform::Flipped270,
        (Transform::Flipped90, Transform::_90) => Transform::Flipped,
        (Transform::Flipped90, Transform::_180) => Transform::Flipped90,
        (Transform::Flipped90, Transform::_270) => Transform::Flipped180,
        (Transform::Flipped90, Transform::Flipped) => Transform::_270,
        (Transform::Flipped90, Transform::Flipped90) => Transform::Normal,
        (Transform::Flipped90, Transform::Flipped180) => Transform::_90,
        (Transform::Flipped90, Transform::Flipped270) => Transform::_180,

        (Transform::Flipped180, Transform::Normal) => Transform::Flipped180,
        (Transform::Flipped180, Transform::_90) => Transform::Flipped270,
        (Transform::Flipped180, Transform::_180) => Transform::Flipped,
        (Transform::Flipped180, Transform::_270) => Transform::Flipped90,
        (Transform::Flipped180, Transform::Flipped) => Transform::_180,
        (Transform::Flipped180, Transform::Flipped90) => Transform::_270,
        (Transform::Flipped180, Transform::Flipped180) => Transform::Normal,
        (Transform::Flipped180, Transform::Flipped270) => Transform::_90,

        (Transform::Flipped270, Transform::Normal) => Transform::Flipped90,
        (Transform::Flipped270, Transform::_90) => Transform::Flipped180,
        (Transform::Flipped270, Transform::_180) => Transform::Flipped270,
        (Transform::Flipped270, Transform::_270) => Transform::Flipped,
        (Transform::Flipped270, Transform::Flipped) => Transform::_90,
        (Transform::Flipped270, Transform::Flipped90) => Transform::_180,
        (Transform::Flipped270, Transform::Flipped180) => Transform::_270,
        (Transform::Flipped270, Transform::Flipped270) => Transform::Normal,
    }
}

#[profiling::function]
fn copy_element_to_cursor_bo<R, E>(
    renderer: &mut R,
    element: &E,
    element_size: Size<i32, Physical>,
    cursor_size: Size<i32, Physical>,
    output_transform: Transform,
    bo: &mut GbmBuffer,
) -> bool
where
    R: Renderer,
    E: RenderElement<R>,
{
    let Some(underlying_storage) = element.underlying_storage(renderer) else {
        return false;
    };

    let element_src = element.src();
    let element_scale = element_src.size / element_size.to_f64();

    if element_src.loc != Point::default()
        || element_scale != Scale::from(1f64)
        || element.transform() != Transform::Normal
        || output_transform != Transform::Normal
    {
        return false;
    }

    let bo_format = bo.format().code;
    let bo_stride = bo.stride();

    let mut copy_to_bo = |src, src_stride, src_height| {
        if src_stride == bo_stride as i32 {
            bo.write(src).is_ok()
        } else {
            let res = bo.map_mut(0, 0, cursor_size.w as u32, cursor_size.h as u32, |mbo| {
                let dst = mbo.buffer_mut();
                for row in 0..src_height {
                    let src_row_start = (row * src_stride) as usize;
                    let src_row_end = src_row_start + src_stride as usize;
                    let src_row = &src[src_row_start..src_row_end];
                    let dst_row_start = (row * bo_stride as i32) as usize;
                    let dst_row_end = dst_row_start + src_stride as usize;
                    let dst_row = &mut dst[dst_row_start..dst_row_end];
                    dst_row.copy_from_slice(src_row);
                }
            });
            res.is_ok()
        }
    };

    match underlying_storage {
        UnderlyingStorage::Wayland(buffer) => {
            shm::with_buffer_contents(buffer, |ptr, len, data| {
                let Some(format) = shm::shm_format_to_fourcc(data.format) else {
                    return false;
                };

                if format != bo_format {
                    return false;
                };

                let expected_len = (data.stride * data.height) as usize;
                if data.offset as usize + expected_len > len {
                    return false;
                };

                copy_to_bo(
                    unsafe { std::slice::from_raw_parts(ptr.offset(data.offset as isize), expected_len) },
                    data.stride,
                    data.height,
                )
            })
            .unwrap_or(false)
        }
        UnderlyingStorage::Memory(memory) => {
            if memory.format() != bo_format {
                return false;
            };

            copy_to_bo(memory, memory.stride(), memory.size().h)
        }
    }
}

struct CachedDrmFramebuffer<B: Framebuffer>(Arc<DrmFramebuffer<B>>);

impl<B: Framebuffer> PartialEq for CachedDrmFramebuffer<B> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        AsRef::<framebuffer::Handle>::as_ref(&self) == AsRef::<framebuffer::Handle>::as_ref(&other)
    }
}

impl<B: Framebuffer + std::fmt::Debug> std::fmt::Debug for CachedDrmFramebuffer<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CachedDrmFramebuffer").field(&self.0).finish()
    }
}

impl<B: Framebuffer> CachedDrmFramebuffer<B> {
    #[inline]
    fn new(buffer: DrmFramebuffer<B>) -> Self {
        CachedDrmFramebuffer(Arc::new(buffer))
    }
}

impl<B: Framebuffer> Clone for CachedDrmFramebuffer<B> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<B: Framebuffer> AsRef<framebuffer::Handle> for CachedDrmFramebuffer<B> {
    #[inline]
    fn as_ref(&self) -> &framebuffer::Handle {
        (*self.0).as_ref()
    }
}

impl<B: Framebuffer> Framebuffer for CachedDrmFramebuffer<B> {
    #[inline]
    fn format(&self) -> drm_fourcc::DrmFormat {
        (*self.0).format()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameError<
    A: std::error::Error + Send + Sync + 'static,
    B: std::error::Error + Send + Sync + 'static,
    F: std::error::Error + Send + Sync + 'static,
> {
    #[error("Failed to claim the primary plane")]
    PrimaryPlaneClaimFailed,
    #[error("No supported plane buffer format found")]
    NoSupportedPlaneFormat,
    #[error("No supported renderer buffer format found")]
    NoSupportedRendererFormat,
    #[error("Failed to allocate a new buffer")]
    NoFreeSlotsError,
    #[error("The underlying drm surface encountered an error: {0}")]
    DrmError(#[from] DrmError),
    #[error("The underlying allocator encountered an error: {0}")]
    Allocator(#[source] A),
    #[error("Failed to export the allocated buffer as dmabuf: {0}")]
    AsDmabufError(#[source] B),
    #[error("The framebuffer export encountered an error: {0}")]
    FramebufferExport(#[source] F),
    #[error("No framebuffer available")]
    NoFramebuffer,
    #[error("No frame has been prepared or it does not contain any changes")]
    EmptyFrame,
}

#[derive(thiserror::Error)]
pub enum RenderFrameError<
    A: std::error::Error + Send + Sync + 'static,
    B: std::error::Error + Send + Sync + 'static,
    F: std::error::Error + Send + Sync + 'static,
    R: std::error::Error,
> {
    #[error(transparent)]
    PrepareFrame(#[from] FrameError<A, B, F>),
    #[error(transparent)]
    RenderFrame(#[from] OutputDamageTrackerError<R>),
}

impl<A, B, F, R> std::fmt::Debug for RenderFrameError<A, B, F, R>
where
    A: std::error::Error + Send + Sync + 'static,
    B: std::error::Error + Send + Sync + 'static,
    F: std::error::Error + Send + Sync + 'static,
    R: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrepareFrame(arg0) => f.debug_tuple("PrepareFrame").field(arg0).finish(),
            Self::RenderFrame(arg0) => f.debug_tuple("RenderFrame").field(arg0).finish(),
        }
    }
}

impl<
        A: std::error::Error + Send + Sync + 'static,
        B: std::error::Error + Send + Sync + 'static,
        F: std::error::Error + Send + Sync + 'static,
    > From<FrameError<A, B, F>> for SwapBuffersError
{
    #[inline]
    fn from(err: FrameError<A, B, F>) -> SwapBuffersError {
        match err {
            x @ FrameError::NoSupportedPlaneFormat
            | x @ FrameError::NoSupportedRendererFormat
            | x @ FrameError::PrimaryPlaneClaimFailed
            | x @ FrameError::NoFramebuffer => SwapBuffersError::ContextLost(Box::new(x)),
            x @ FrameError::NoFreeSlotsError | x @ FrameError::EmptyFrame => {
                SwapBuffersError::TemporaryFailure(Box::new(x))
            }
            FrameError::DrmError(err) => err.into(),
            FrameError::Allocator(err) => SwapBuffersError::ContextLost(Box::new(err)),
            FrameError::AsDmabufError(err) => SwapBuffersError::ContextLost(Box::new(err)),
            FrameError::FramebufferExport(err) => SwapBuffersError::ContextLost(Box::new(err)),
        }
    }
}

fn nvidia_drm_version() -> Option<(u32, u32, u32)> {
    let ver = std::fs::read_to_string("/sys/module/nvidia_drm/version").ok()?;
    let mut components = ver.trim().split('.');
    let major = u32::from_str(components.next()?).ok()?;
    let minor = u32::from_str(components.next()?).ok()?;
    let patch = u32::from_str(components.next()?).ok()?;
    Some((major, minor, patch))
}

#[test]
fn drm_compositor_is_send() {
    use std::marker::PhantomData;

    use crate::backend::drm::DrmDeviceFd;

    fn is_send<T: Send>() {
        let _ = PhantomData::<T>;
    }

    is_send::<DrmCompositor<GbmAllocator<DrmDeviceFd>, GbmFramebufferExporter<DrmDeviceFd>, (), DrmDeviceFd>>(
    );
}
