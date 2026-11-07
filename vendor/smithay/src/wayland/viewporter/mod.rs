//! Utilities for handling the `wp_viewporter` protocol.

use std::sync::Mutex;

use tracing::trace;
use wayland_protocols::wp::viewporter::server::{wp_viewport, wp_viewporter};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface, Dispatch, DisplayHandle, GlobalDispatch, Resource, Weak,
};

use crate::utils::{Client, Logical, Rectangle, Size};

use super::compositor::{self, with_states, Cacheable, CompositorHandler, SurfaceData};

#[derive(Debug)]
pub struct ViewporterState {
    global: GlobalId,
}

pub(crate) type ViewporterSurfaceState = Mutex<Option<ViewportMarker>>;

impl ViewporterState {
    pub fn new<D>(display: &DisplayHandle) -> ViewporterState
    where
        D: GlobalDispatch<wp_viewporter::WpViewporter, ()>
            + Dispatch<wp_viewporter::WpViewporter, ()>
            + Dispatch<wp_viewport::WpViewport, ViewportState>
            + 'static,
    {
        ViewporterState {
            global: display.create_global::<D, wp_viewporter::WpViewporter, ()>(1, ()),
        }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<wp_viewporter::WpViewporter, (), D> for ViewporterState
where
    D: GlobalDispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewport::WpViewport, ViewportState>,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &wayland_server::Client,
        resource: wayland_server::New<wp_viewporter::WpViewporter>,
        _global_data: &(),
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<wp_viewporter::WpViewporter, (), D> for ViewporterState
where
    D: GlobalDispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewport::WpViewport, ViewportState>,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &wp_viewporter::WpViewporter,
        request: <wp_viewporter::WpViewporter as wayland_server::Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_viewporter::Request::GetViewport { id, surface } => {
                let already_has_viewport = with_states(&surface, |states| {
                    states
                        .data_map
                        .get::<ViewporterSurfaceState>()
                        .map(|v| v.lock().unwrap().is_some())
                        .unwrap_or(false)
                });

                if already_has_viewport {
                    surface.post_error(
                        wp_viewporter::Error::ViewportExists as u32,
                        "the surface already has a viewport object associated".to_string(),
                    );
                    return;
                }

                let viewport = data_init.init(
                    id,
                    ViewportState {
                        surface: surface.downgrade(),
                    },
                );
                let initial = with_states(&surface, |states| {
                    let inserted = states
                        .data_map
                        .insert_if_missing_threadsafe::<ViewporterSurfaceState, _>(|| {
                            Mutex::new(Some(ViewportMarker(viewport.downgrade())))
                        });

                    if !inserted {
                        *states
                            .data_map
                            .get::<ViewporterSurfaceState>()
                            .unwrap()
                            .lock()
                            .unwrap() = Some(ViewportMarker(viewport.downgrade()));
                    }

                    inserted
                });

                if initial {
                    compositor::add_pre_commit_hook::<D, _>(&surface, viewport_pre_commit_hook);
                }
            }
            wp_viewporter::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<wp_viewport::WpViewport, ViewportState, D> for ViewportState
where
    D: GlobalDispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewporter::WpViewporter, ()>,
    D: Dispatch<wp_viewport::WpViewport, ViewportState>,
    D: CompositorHandler,
{
    fn request(
        state: &mut D,
        client: &wayland_server::Client,
        resource: &wp_viewport::WpViewport,
        request: <wp_viewport::WpViewport as wayland_server::Resource>::Request,
        data: &ViewportState,
        _dhandle: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_viewport::Request::Destroy => {
                if let Ok(surface) = data.surface.upgrade() {
                    with_states(&surface, |states| {
                        states
                            .data_map
                            .get::<ViewporterSurfaceState>()
                            .unwrap()
                            .lock()
                            .unwrap()
                            .take();
                        *states.cached_state.get::<ViewportCachedState>().pending() =
                            ViewportCachedState::default();
                    });
                }
            }
            wp_viewport::Request::SetSource { x, y, width, height } => {
                let is_unset = x == -1.0 && y == -1.0 && width == -1.0 && height == -1.0;
                let is_valid_src = x >= 0.0 && y >= 0.0 && width > 0.0 && height > 0.0;

                if !is_unset && !is_valid_src {
                    resource.post_error(
                        wp_viewport::Error::BadValue as u32,
                        "negative or zero values in width or height or negative values in x or y".to_string(),
                    );
                    return;
                }

                let Ok(surface) = data.surface.upgrade() else {
                    resource.post_error(
                        wp_viewport::Error::NoSurface as u32,
                        "the wl_surface was destroyed".to_string(),
                    );
                    return;
                };

                with_states(&surface, |states| {
                    let mut guard = states.cached_state.get::<ViewportCachedState>();
                    let viewport_state = guard.pending();
                    let src = if is_unset {
                        None
                    } else {
                        let src = Rectangle::new((x, y).into(), (width, height).into());
                        trace!(surface = ?surface, src = ?src, "setting surface viewport src");
                        Some(src)
                    };
                    viewport_state.src = src;
                });
            }
            wp_viewport::Request::SetDestination { width, height } => {
                let is_unset = width == -1 && height == -1;
                let is_valid_size = width > 0 && height > 0;

                if !is_unset && !is_valid_size {
                    resource.post_error(
                        wp_viewport::Error::BadValue as u32,
                        "negative or zero values in width or height".to_string(),
                    );
                    return;
                }

                let Ok(surface) = data.surface.upgrade() else {
                    resource.post_error(
                        wp_viewport::Error::NoSurface as u32,
                        "the wl_surface was destroyed".to_string(),
                    );
                    return;
                };

                with_states(&surface, |states| {
                    let mut guard = states.cached_state.get::<ViewportCachedState>();
                    let viewport_state = guard.pending();
                    let size = if is_unset {
                        None
                    } else {
                        let client_scale = state.client_compositor_state(client).client_scale();
                        let dst = Size::<_, Client>::from((width, height))
                            .to_f64()
                            .to_logical(client_scale)
                            .to_i32_round();
                        trace!(surface = ?surface, size = ?dst, "setting surface viewport destination size");
                        Some(dst)
                    };
                    viewport_state.dst = size;
                });
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct ViewportState {
    surface: Weak<wl_surface::WlSurface>,
}

pub(crate) struct ViewportMarker(Weak<wp_viewport::WpViewport>);

fn viewport_pre_commit_hook<D: 'static>(
    _state: &mut D,
    _dh: &DisplayHandle,
    surface: &wl_surface::WlSurface,
) {
    with_states(surface, |states| {
        states
            .data_map
            .insert_if_missing_threadsafe::<ViewporterSurfaceState, _>(|| Mutex::new(None));
        let viewport = states
            .data_map
            .get::<ViewporterSurfaceState>()
            .unwrap()
            .lock()
            .unwrap();
        if let Some(viewport) = &*viewport {
            let mut guard = states.cached_state.get::<ViewportCachedState>();
            let viewport_state = guard.pending();

            let src_size = viewport_state.src.map(|src| src.size);
            if viewport_state.dst.is_none()
                && src_size != src_size.map(|s| Size::from((s.w as i32, s.h as i32)).to_f64())
            {
                if let Ok(viewport) = viewport.0.upgrade() {
                    viewport.post_error(
                        wp_viewport::Error::BadSize as u32,
                        "destination size is not integer".to_string(),
                    );
                }
            }
        }
    });
}

pub fn ensure_viewport_valid(states: &SurfaceData, buffer_size: Size<i32, Logical>) -> bool {
    states
        .data_map
        .insert_if_missing_threadsafe::<ViewporterSurfaceState, _>(|| Mutex::new(None));
    let viewport = states
        .data_map
        .get::<ViewporterSurfaceState>()
        .unwrap()
        .lock()
        .unwrap();

    if let Some(viewport) = &*viewport {
        let mut guard = states.cached_state.get::<ViewportCachedState>();
        let state = guard.current();

        let buffer_rect = Rectangle::from_size(buffer_size.to_f64());
        let src = state.src.unwrap_or(buffer_rect);
        let valid = buffer_rect.contains_rect(src);
        if !valid {
            if let Ok(viewport) = viewport.0.upgrade() {
                viewport.post_error(
                    wp_viewport::Error::OutOfBuffer as u32,
                    format!(
                        "source rectangle x={},y={},w={},h={} extends outside of the content area x={},y={},w={},h={}",
                        src.loc.x, src.loc.y, src.size.w, src.size.h,
                        buffer_rect.loc.x, buffer_rect.loc.y, buffer_rect.size.w, buffer_rect.size.h),
                );
            }
        }
        valid
    } else {
        true
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ViewportCachedState {
    pub src: Option<Rectangle<f64, Logical>>,
    pub dst: Option<Size<i32, Logical>>,
}

impl ViewportCachedState {
    pub fn size(&self) -> Option<Size<i32, Logical>> {
        self.dst.or_else(|| {
            self.src
                .map(|src| Size::from((src.size.w as i32, src.size.h as i32)))
        })
    }
}

impl Cacheable for ViewportCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        ViewportCachedState {
            src: self.src,
            dst: self.dst,
        }
    }

    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        into.src = self.src;
        into.dst = self.dst;
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_viewporter {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::viewporter::server::wp_viewporter::WpViewporter: ()
        ] => $crate::wayland::viewporter::ViewporterState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::viewporter::server::wp_viewporter::WpViewporter: ()
        ] => $crate::wayland::viewporter::ViewporterState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::viewporter::server::wp_viewport::WpViewport: $crate::wayland::viewporter::ViewportState
        ] => $crate::wayland::viewporter::ViewportState);
    };
}
