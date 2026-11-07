//! DRM syncobj protocol.

use std::{
    cell::RefCell,
    os::unix::io::AsFd,
    sync::{Arc, Weak},
};
use tracing::warn;
use wayland_protocols::wp::linux_drm_syncobj::v1::server::{
    wp_linux_drm_syncobj_manager_v1::{self, WpLinuxDrmSyncobjManagerV1},
    wp_linux_drm_syncobj_surface_v1::{self, WpLinuxDrmSyncobjSurfaceV1},
    wp_linux_drm_syncobj_timeline_v1::{self, WpLinuxDrmSyncobjTimelineV1},
};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface::WlSurface, Client, DataInit, Dispatch, DisplayHandle,
    GlobalDispatch, New, Resource, Weak as WlWeak,
};

use super::{
    compositor::{self, with_states, BufferAssignment, Cacheable, HookId, SurfaceAttributes},
    dmabuf::get_dmabuf,
};
use crate::backend::drm::DrmDeviceFd;

mod sync_point;
pub use sync_point::*;

pub fn supports_syncobj_eventfd(device: &DrmDeviceFd) -> bool {
    match drm_ffi::syncobj::eventfd(device.as_fd(), 0, 0, device.as_fd(), false) {
        Ok(_) => unreachable!(),
        Err(err) => err.kind() == std::io::ErrorKind::NotFound,
    }
}

pub trait DrmSyncobjHandler {
    fn drm_syncobj_state(&mut self) -> Option<&mut DrmSyncobjState>;
}

#[allow(missing_debug_implementations)]
pub struct DrmSyncobjGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Debug, Default)]
pub struct DrmSyncobjCachedState {
    pub acquire_point: Option<DrmSyncPoint>,
    pub release_point: Option<DrmSyncPoint>,
}

impl Cacheable for DrmSyncobjCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        Self {
            acquire_point: self.acquire_point.take(),
            release_point: self.release_point.take(),
        }
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        if self.acquire_point.is_some() && self.release_point.is_some() {
            if let Some(release_point) = &into.release_point {
                if let Err(err) = release_point.signal() {
                    tracing::error!("Failed to signal syncobj release point: {}", err);
                }
            }
            into.acquire_point = self.acquire_point;
            into.release_point = self.release_point;
        }
    }
}

#[derive(Debug)]
pub struct DrmSyncobjState {
    global: GlobalId,
    import_device: DrmDeviceFd,
    known_timelines: Vec<Weak<DrmTimelineInner>>,
}

impl DrmSyncobjState {
    pub fn new<D>(display: &DisplayHandle, import_device: DrmDeviceFd) -> Self
    where
        D: GlobalDispatch<WpLinuxDrmSyncobjManagerV1, DrmSyncobjGlobalData>,
        D: 'static,
    {
        Self::new_with_filter::<D, _>(display, import_device, |_| true)
    }

    pub fn new_with_filter<D, F>(display: &DisplayHandle, import_device: DrmDeviceFd, filter: F) -> Self
    where
        D: GlobalDispatch<WpLinuxDrmSyncobjManagerV1, DrmSyncobjGlobalData>,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global = display.create_global::<D, WpLinuxDrmSyncobjManagerV1, DrmSyncobjGlobalData>(
            1,
            DrmSyncobjGlobalData {
                filter: Box::new(filter),
            },
        );

        Self {
            global,
            import_device,
            known_timelines: Vec::new(),
        }
    }

    pub fn update_device(&mut self, import_device: DrmDeviceFd) {
        for timeline in self.known_timelines.iter().filter_map(Weak::upgrade) {
            if let Err(err) = timeline.update_device(&import_device) {
                warn!(?err, "Failed to update existing timeline");
            }
        }
        self.import_device = import_device;
    }

    pub fn into_global(self) -> GlobalId {
        for timeline in self.known_timelines.iter().filter_map(Weak::upgrade) {
            timeline.invalidate();
        }
        self.global
    }
}

impl<D> GlobalDispatch<WpLinuxDrmSyncobjManagerV1, DrmSyncobjGlobalData, D> for DrmSyncobjState
where
    D: Dispatch<WpLinuxDrmSyncobjManagerV1, ()>,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<WpLinuxDrmSyncobjManagerV1>,
        _global_data: &DrmSyncobjGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init::<_, _>(resource, ());
    }

    fn can_view(client: Client, global_data: &DrmSyncobjGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

fn commit_hook<D: DrmSyncobjHandler>(_data: &mut D, _dh: &DisplayHandle, surface: &WlSurface) {
    compositor::with_states(surface, |states| {
        let mut cached = states.cached_state.get::<SurfaceAttributes>();
        let pending = cached.pending();
        let new_buffer = pending.buffer.as_ref().and_then(|buffer| match buffer {
            BufferAssignment::NewBuffer(buffer) => Some(buffer),
            _ => None,
        });
        if let Some(data) = states
            .data_map
            .get::<RefCell<Option<WpLinuxDrmSyncobjSurfaceV1>>>()
        {
            if let Some(syncobj_surface) = data.borrow().as_ref() {
                let mut cached = states.cached_state.get::<DrmSyncobjCachedState>();
                let pending = cached.pending();
                if pending.acquire_point.is_some() && new_buffer.is_none() {
                    syncobj_surface.post_error(
                        wp_linux_drm_syncobj_surface_v1::Error::NoBuffer as u32,
                        "acquire point without buffer".to_string(),
                    );
                } else if pending.acquire_point.is_some() && pending.release_point.is_none() {
                    syncobj_surface.post_error(
                        wp_linux_drm_syncobj_surface_v1::Error::NoReleasePoint as u32,
                        "acquire point without release point".to_string(),
                    );
                } else if pending.acquire_point.is_none() && pending.release_point.is_some() {
                    syncobj_surface.post_error(
                        wp_linux_drm_syncobj_surface_v1::Error::NoAcquirePoint as u32,
                        "release point without acquire point".to_string(),
                    );
                } else if let (Some(acquire), Some(release)) =
                    (pending.acquire_point.as_ref(), pending.release_point.as_ref())
                {
                    if acquire.timeline == release.timeline && release.point <= acquire.point {
                        syncobj_surface.post_error(
                            wp_linux_drm_syncobj_surface_v1::Error::ConflictingPoints as u32,
                            format!(
                                "release point {} is not greater than acquire point {}",
                                release.point, acquire.point
                            ),
                        );
                    }
                    if let Some(buffer) = new_buffer {
                        if get_dmabuf(buffer).is_err() {
                            syncobj_surface.post_error(
                                wp_linux_drm_syncobj_surface_v1::Error::UnsupportedBuffer as u32,
                                "sync points with non-dmabuf buffer".to_string(),
                            );
                        }
                    }
                }
            }
        }
    });
}

fn destruction_hook<D: DrmSyncobjHandler>(_data: &mut D, surface: &WlSurface) {
    compositor::with_states(surface, |states| {
        let mut cached = states.cached_state.get::<DrmSyncobjCachedState>();
        if let Some(release_point) = &cached.pending().release_point {
            if let Err(err) = release_point.signal() {
                tracing::error!("Failed to signal syncobj release point: {}", err);
            }
        }
        if let Some(release_point) = &cached.current().release_point {
            if let Err(err) = release_point.signal() {
                tracing::error!("Failed to signal syncobj release point: {}", err);
            }
        }
    });
}

impl<D> Dispatch<WpLinuxDrmSyncobjManagerV1, (), D> for DrmSyncobjState
where
    D: Dispatch<WpLinuxDrmSyncobjSurfaceV1, DrmSyncobjSurfaceData>,
    D: Dispatch<WpLinuxDrmSyncobjTimelineV1, DrmSyncobjTimelineData>,
    D: DrmSyncobjHandler,
{
    fn request(
        state: &mut D,
        _client: &Client,
        resource: &WpLinuxDrmSyncobjManagerV1,
        request: wp_linux_drm_syncobj_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            wp_linux_drm_syncobj_manager_v1::Request::GetSurface { id, surface } => {
                let already_exists = with_states(&surface, |states| {
                    states
                        .data_map
                        .get::<RefCell<Option<WpLinuxDrmSyncobjSurfaceV1>>>()
                        .map(|v| v.borrow().is_some())
                        .unwrap_or(false)
                });
                if already_exists {
                    resource.post_error(
                        wp_linux_drm_syncobj_manager_v1::Error::SurfaceExists as u32,
                        "the surface already has a syncobj_surface object associated".to_string(),
                    );
                    return;
                }
                let commit_hook_id = compositor::add_pre_commit_hook::<D, _>(&surface, commit_hook);
                let destruction_hook_id =
                    compositor::add_destruction_hook::<D, _>(&surface, destruction_hook);
                let syncobj_surface = data_init.init::<_, _>(
                    id,
                    DrmSyncobjSurfaceData {
                        surface: surface.downgrade(),
                        commit_hook_id,
                        destruction_hook_id,
                    },
                );
                with_states(&surface, |states| {
                    states
                        .data_map
                        .insert_if_missing(|| RefCell::new(Some(syncobj_surface)))
                });
            }
            wp_linux_drm_syncobj_manager_v1::Request::ImportTimeline { id, fd } => {
                if let Some(state) = state.drm_syncobj_state() {
                    match DrmTimeline::new(&state.import_device, fd) {
                        Ok(timeline) => {
                            state.known_timelines.push(Arc::downgrade(&timeline.0));
                            data_init.init::<_, _>(id, DrmSyncobjTimelineData { timeline });
                        }
                        Err(err) => {
                            resource.post_error(
                                wp_linux_drm_syncobj_manager_v1::Error::InvalidTimeline as u32,
                                format!("failed to import syncobj timeline: {}", err),
                            );
                        }
                    }
                } else {
                    resource.post_error(
                        wp_linux_drm_syncobj_manager_v1::Error::InvalidTimeline as u32,
                        "global orphaned",
                    )
                }
            }
            wp_linux_drm_syncobj_manager_v1::Request::Destroy => {}
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct DrmSyncobjSurfaceData {
    surface: WlWeak<WlSurface>,
    commit_hook_id: HookId,
    destruction_hook_id: HookId,
}

impl<D> Dispatch<WpLinuxDrmSyncobjSurfaceV1, DrmSyncobjSurfaceData, D> for DrmSyncobjState
where
    D: DrmSyncobjHandler,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        resource: &WpLinuxDrmSyncobjSurfaceV1,
        request: wp_linux_drm_syncobj_surface_v1::Request,
        data: &DrmSyncobjSurfaceData,
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            wp_linux_drm_syncobj_surface_v1::Request::Destroy => {
                if let Ok(surface) = data.surface.upgrade() {
                    compositor::remove_pre_commit_hook(&surface, data.commit_hook_id.clone());
                    compositor::remove_destruction_hook(&surface, data.destruction_hook_id.clone());
                    with_states(&surface, |states| {
                        *states
                            .data_map
                            .get::<RefCell<Option<WpLinuxDrmSyncobjSurfaceV1>>>()
                            .unwrap()
                            .borrow_mut() = None;
                        let mut cached = states.cached_state.get::<DrmSyncobjCachedState>();
                        cached.pending().acquire_point = None;
                        if let Some(release_point) = cached.pending().release_point.take() {
                            if let Err(err) = release_point.signal() {
                                tracing::error!("Failed to signal syncobj release point: {}", err);
                            }
                        }
                    });
                }
            }
            wp_linux_drm_syncobj_surface_v1::Request::SetAcquirePoint {
                timeline,
                point_hi,
                point_lo,
            } => {
                let Ok(surface) = data.surface.upgrade() else {
                    resource.post_error(
                        wp_linux_drm_syncobj_surface_v1::Error::NoSurface,
                        "Set acquire point for destroyed surface.",
                    );
                    return;
                };

                let sync_point = DrmSyncPoint {
                    timeline: timeline
                        .data::<DrmSyncobjTimelineData>()
                        .unwrap()
                        .timeline
                        .clone(),
                    point: ((point_hi as u64) << 32) + (point_lo as u64),
                };
                with_states(&surface, |states| {
                    let mut cached = states.cached_state.get::<DrmSyncobjCachedState>();
                    let cached_state = cached.pending();
                    cached_state.acquire_point = Some(sync_point);
                });
            }
            wp_linux_drm_syncobj_surface_v1::Request::SetReleasePoint {
                timeline,
                point_hi,
                point_lo,
            } => {
                let Ok(surface) = data.surface.upgrade() else {
                    resource.post_error(
                        wp_linux_drm_syncobj_surface_v1::Error::NoSurface,
                        "Set release point for destroyed surface.",
                    );
                    return;
                };

                let sync_point = DrmSyncPoint {
                    timeline: timeline
                        .data::<DrmSyncobjTimelineData>()
                        .unwrap()
                        .timeline
                        .clone(),
                    point: ((point_hi as u64) << 32) + (point_lo as u64),
                };
                with_states(&surface, |states| {
                    let mut cached = states.cached_state.get::<DrmSyncobjCachedState>();
                    let cached_state = cached.pending();
                    cached_state.release_point = Some(sync_point);
                });
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct DrmSyncobjTimelineData {
    timeline: DrmTimeline,
}

impl<D: DrmSyncobjHandler> Dispatch<WpLinuxDrmSyncobjTimelineV1, DrmSyncobjTimelineData, D>
    for DrmSyncobjState
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &WpLinuxDrmSyncobjTimelineV1,
        request: wp_linux_drm_syncobj_timeline_v1::Request,
        _data: &DrmSyncobjTimelineData,
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            wp_linux_drm_syncobj_timeline_v1::Request::Destroy => {}
            _ => unreachable!(),
        }
    }

    fn destroyed(
        state: &mut D,
        _client: wayland_server::backend::ClientId,
        _resource: &WpLinuxDrmSyncobjTimelineV1,
        data: &DrmSyncobjTimelineData,
    ) {
        if let Some(state) = state.drm_syncobj_state() {
            state
                .known_timelines
                .retain(|t| t.upgrade().is_some_and(|t| !Arc::ptr_eq(&t, &data.timeline.0)))
        }
    }
}

#[macro_export]
macro_rules! delegate_drm_syncobj {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::linux_drm_syncobj::v1::server::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1: $crate::wayland::drm_syncobj::DrmSyncobjGlobalData
        ] => $crate::wayland::drm_syncobj::DrmSyncobjState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::linux_drm_syncobj::v1::server::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1: ()
        ] => $crate::wayland::drm_syncobj::DrmSyncobjState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::linux_drm_syncobj::v1::server::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1: $crate::wayland::drm_syncobj::DrmSyncobjSurfaceData
        ] => $crate::wayland::drm_syncobj::DrmSyncobjState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::linux_drm_syncobj::v1::server::wp_linux_drm_syncobj_timeline_v1::WpLinuxDrmSyncobjTimelineV1: $crate::wayland::drm_syncobj::DrmSyncobjTimelineData
        ] => $crate::wayland::drm_syncobj::DrmSyncobjState);
    }
}
