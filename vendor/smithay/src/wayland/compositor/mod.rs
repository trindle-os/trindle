//! Utilities for handling surfaces, subsurfaces and regions.

mod cache;
mod handlers;
mod transaction;
mod tree;

use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{any::Any, sync::Mutex};

pub use self::cache::{Cacheable, CachedState, MultiCache};
pub use self::handlers::{RegionUserData, SubsurfaceCachedState, SubsurfaceUserData, SurfaceUserData};
use self::transaction::TransactionQueue;
pub use self::transaction::{Barrier, Blocker, BlockerState};
pub use self::tree::{AlreadyHasRole, TraversalAction};
use self::tree::{PrivateSurfaceData, SuggestedSurfaceState};
pub use crate::utils::hook::HookId;
use crate::utils::Transform;
use crate::utils::{user_data::UserDataMap, Buffer, Logical, Point, Rectangle};
use atomic_float::AtomicF64;
use wayland_server::backend::GlobalId;
use wayland_server::protocol::wl_compositor::WlCompositor;
use wayland_server::protocol::wl_subcompositor::WlSubcompositor;
use wayland_server::protocol::{wl_buffer, wl_callback, wl_output, wl_region, wl_surface::WlSurface};
use wayland_server::{Client, DisplayHandle, GlobalDispatch, Resource};

pub const SUBSURFACE_ROLE: &str = "subsurface";

#[derive(Debug, PartialEq, Eq)]
pub enum Damage {
    Surface(Rectangle<i32, Logical>),
    Buffer(Rectangle<i32, Buffer>),
}

#[derive(Debug)]
pub struct SurfaceData {
    pub role: Option<&'static str>,
    pub data_map: UserDataMap,
    pub cached_state: MultiCache,
}

#[derive(Debug)]
pub enum BufferAssignment {
    Removed,
    NewBuffer(wl_buffer::WlBuffer),
}

#[derive(Debug)]
pub struct SurfaceAttributes {
    pub buffer: Option<BufferAssignment>,

    pub buffer_delta: Option<Point<i32, Logical>>,

    pub buffer_scale: i32,
    pub buffer_transform: wl_output::Transform,
    pub opaque_region: Option<RegionAttributes>,
    pub input_region: Option<RegionAttributes>,
    pub damage: Vec<Damage>,
    pub frame_callbacks: Vec<wl_callback::WlCallback>,

    pub(crate) client_scale: f64,
}

impl Default for SurfaceAttributes {
    fn default() -> SurfaceAttributes {
        SurfaceAttributes {
            buffer: None,
            buffer_delta: None,
            buffer_scale: 1,
            buffer_transform: wl_output::Transform::Normal,
            opaque_region: None,
            input_region: None,
            damage: Vec::new(),
            frame_callbacks: Vec::new(),
            client_scale: 1.,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RectangleKind {
    Add,
    Subtract,
}

#[derive(Clone, Debug, Default)]
pub struct RegionAttributes {
    pub rects: Vec<(RectangleKind, Rectangle<i32, Logical>)>,
}

impl RegionAttributes {
    pub fn contains<P: Into<Point<i32, Logical>>>(&self, point: P) -> bool {
        let point: Point<i32, Logical> = point.into();
        let mut contains = false;
        for (kind, rect) in &self.rects {
            if rect.contains(point) {
                match kind {
                    RectangleKind::Add => contains = true,
                    RectangleKind::Subtract => contains = false,
                }
            }
        }
        contains
    }
}

pub fn with_surface_tree_upward<F1, F2, F3, T>(
    surface: &WlSurface,
    initial: T,
    filter: F1,
    processor: F2,
    post_filter: F3,
) where
    F1: FnMut(&WlSurface, &SurfaceData, &T) -> TraversalAction<T>,
    F2: FnMut(&WlSurface, &SurfaceData, &T),
    F3: FnMut(&WlSurface, &SurfaceData, &T) -> bool,
{
    PrivateSurfaceData::map_tree(surface, &initial, filter, processor, post_filter, false);
}

pub fn with_surface_tree_downward<F1, F2, F3, T>(
    surface: &WlSurface,
    initial: T,
    filter: F1,
    processor: F2,
    post_filter: F3,
) where
    F1: FnMut(&WlSurface, &SurfaceData, &T) -> TraversalAction<T>,
    F2: FnMut(&WlSurface, &SurfaceData, &T),
    F3: FnMut(&WlSurface, &SurfaceData, &T) -> bool,
{
    PrivateSurfaceData::map_tree(surface, &initial, filter, processor, post_filter, true);
}

pub fn get_parent(surface: &WlSurface) -> Option<WlSurface> {
    PrivateSurfaceData::get_parent(surface)
}

pub fn get_children(surface: &WlSurface) -> Vec<WlSurface> {
    PrivateSurfaceData::get_children(surface)
}

pub fn is_sync_subsurface(surface: &WlSurface) -> bool {
    self::handlers::is_effectively_sync(surface)
}

pub fn get_role(surface: &WlSurface) -> Option<&'static str> {
    PrivateSurfaceData::get_role(surface)
}

pub fn give_role(surface: &WlSurface, role: &'static str) -> Result<(), AlreadyHasRole> {
    PrivateSurfaceData::set_role(surface, role)
}

pub fn with_states<F, T>(surface: &WlSurface, f: F) -> T
where
    F: FnOnce(&SurfaceData) -> T,
{
    PrivateSurfaceData::with_states(surface, f)
}

pub fn send_surface_state(surface: &WlSurface, data: &SurfaceData, scale: i32, transform: Transform) {
    if surface.version() < 6 {
        return;
    }

    let mut storage = data
        .data_map
        .get_or_insert(|| RefCell::new(SuggestedSurfaceState::default()))
        .borrow_mut();

    if storage.scale != scale {
        surface.preferred_buffer_scale(scale);
        storage.scale = scale;
    }

    let transform = transform.into();
    if storage.transform != transform {
        surface.preferred_buffer_transform(transform);
        storage.transform = transform;
    }
}

pub fn get_region_attributes(region: &wl_region::WlRegion) -> RegionAttributes {
    match region.data::<RegionUserData>() {
        Some(data) => data.inner.lock().unwrap().clone(),
        None => panic!("Accessing the data of foreign regions is not supported."),
    }
}

pub fn add_pre_commit_hook<D, F>(surface: &WlSurface, hook: F) -> HookId
where
    F: Fn(&mut D, &DisplayHandle, &WlSurface) + Send + Sync + 'static,
    D: 'static,
{
    let (user_state_type_id, user_state_type) = surface.data::<SurfaceUserData>().unwrap().user_state_type;
    assert_eq!(
        std::any::TypeId::of::<D>(),
        user_state_type_id,
        "D has to equal D used in CompositorState::new<D>(), {} != {}",
        std::any::type_name::<D>(),
        user_state_type,
    );

    let hook = move |state: &mut dyn Any, dh: &DisplayHandle, surface: &WlSurface| {
        let state = state.downcast_mut::<D>().unwrap();
        hook(state, dh, surface);
    };
    PrivateSurfaceData::add_pre_commit_hook(surface, hook)
}

pub fn add_post_commit_hook<D, F>(surface: &WlSurface, hook: F) -> HookId
where
    F: Fn(&mut D, &DisplayHandle, &WlSurface) + Send + Sync + 'static,
    D: 'static,
{
    let (user_state_type_id, user_state_type) = surface.data::<SurfaceUserData>().unwrap().user_state_type;
    assert_eq!(
        std::any::TypeId::of::<D>(),
        user_state_type_id,
        "D has to equal D used in CompositorState::new<D>(), {} != {}",
        std::any::type_name::<D>(),
        user_state_type,
    );

    let hook = move |state: &mut dyn Any, dh: &DisplayHandle, surface: &WlSurface| {
        let state = state.downcast_mut::<D>().unwrap();
        hook(state, dh, surface);
    };
    PrivateSurfaceData::add_post_commit_hook(surface, hook)
}

pub fn add_destruction_hook<D, F>(surface: &WlSurface, hook: F) -> HookId
where
    F: Fn(&mut D, &WlSurface) + Send + Sync + 'static,
    D: 'static,
{
    let (user_state_type_id, user_state_type) = surface.data::<SurfaceUserData>().unwrap().user_state_type;
    assert_eq!(
        std::any::TypeId::of::<D>(),
        user_state_type_id,
        "D has to equal D used in CompositorState::new<D>(), {} != {}",
        std::any::type_name::<D>(),
        user_state_type,
    );

    let hook = move |state: &mut dyn Any, surface: &WlSurface| {
        let state = state.downcast_mut::<D>().unwrap();
        hook(state, surface);
    };
    PrivateSurfaceData::add_destruction_hook(surface, hook)
}

pub fn remove_pre_commit_hook(surface: &WlSurface, hook_id: HookId) {
    PrivateSurfaceData::remove_pre_commit_hook(surface, hook_id)
}

pub fn remove_post_commit_hook(surface: &WlSurface, hook_id: HookId) {
    PrivateSurfaceData::remove_post_commit_hook(surface, hook_id)
}

pub fn remove_destruction_hook(surface: &WlSurface, hook_id: HookId) {
    PrivateSurfaceData::remove_destruction_hook(surface, hook_id)
}

pub fn add_blocker(surface: &WlSurface, blocker: impl Blocker + Send + 'static) {
    PrivateSurfaceData::add_blocker(surface, blocker)
}

pub trait CompositorHandler {
    fn compositor_state(&mut self) -> &mut CompositorState;
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState;

    fn new_surface(&mut self, surface: &WlSurface) {
        let _ = surface;
    }

    fn new_subsurface(&mut self, surface: &WlSurface, parent: &WlSurface) {
        let _ = surface;
        let _ = parent;
    }

    fn commit(&mut self, surface: &WlSurface);

    fn destroyed(&mut self, _surface: &WlSurface) {}
}

#[derive(Debug)]
pub struct CompositorState {
    compositor: GlobalId,
    subcompositor: GlobalId,
    surfaces: Vec<WlSurface>,
}

#[derive(Debug)]
pub struct CompositorClientState {
    queue: Mutex<Option<TransactionQueue>>,
    scale_override: Arc<AtomicF64>,
}

impl Default for CompositorClientState {
    fn default() -> Self {
        CompositorClientState {
            queue: Mutex::new(None),
            scale_override: Arc::new(AtomicF64::new(1.)),
        }
    }
}

impl CompositorClientState {
    pub fn blocker_cleared<D: CompositorHandler + 'static>(&self, state: &mut D, dh: &DisplayHandle) {
        let transactions = if let Some(queue) = self.queue.lock().unwrap().as_mut() {
            queue.take_ready()
        } else {
            Vec::new()
        };

        for transaction in transactions {
            transaction.apply(dh, state)
        }
    }

    pub fn set_client_scale(&self, new_scale: f64) {
        self.scale_override.store(new_scale, Ordering::Release);
    }

    pub fn client_scale(&self) -> f64 {
        self.scale_override.load(Ordering::Acquire)
    }

    pub(crate) fn clone_client_scale(&self) -> Arc<AtomicF64> {
        self.scale_override.clone()
    }
}

impl CompositorState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WlCompositor, ()> + GlobalDispatch<WlSubcompositor, ()> + 'static,
    {
        Self::new_with_version::<D>(display, 5)
    }

    pub fn new_v6<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<WlCompositor, ()> + GlobalDispatch<WlSubcompositor, ()> + 'static,
    {
        Self::new_with_version::<D>(display, 6)
    }

    fn new_with_version<D>(display: &DisplayHandle, version: u32) -> Self
    where
        D: GlobalDispatch<WlCompositor, ()> + GlobalDispatch<WlSubcompositor, ()> + 'static,
    {
        let compositor = display.create_global::<D, WlCompositor, ()>(version, ());
        let subcompositor = display.create_global::<D, WlSubcompositor, ()>(1, ());

        CompositorState {
            compositor,
            subcompositor,
            surfaces: Vec::new(),
        }
    }

    pub fn compositor_global(&self) -> GlobalId {
        self.compositor.clone()
    }

    pub fn subcompositor_global(&self) -> GlobalId {
        self.subcompositor.clone()
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_compositor {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_compositor::WlCompositor: ()
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor: ()
        ] => $crate::wayland::compositor::CompositorState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_compositor::WlCompositor: ()
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_surface::WlSurface: $crate::wayland::compositor::SurfaceUserData
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_region::WlRegion: $crate::wayland::compositor::RegionUserData
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_callback::WlCallback: ()
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor: ()
        ] => $crate::wayland::compositor::CompositorState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface: $crate::wayland::compositor::SubsurfaceUserData
        ] => $crate::wayland::compositor::CompositorState);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_attributes_empty() {
        let region = RegionAttributes { rects: vec![] };
        assert!(!region.contains((0, 0)));
    }

    #[test]
    fn region_attributes_add() {
        let region = RegionAttributes {
            rects: vec![(RectangleKind::Add, Rectangle::from_size((10, 10).into()))],
        };

        assert!(region.contains((0, 0)));
    }

    #[test]
    fn region_attributes_add_subtract() {
        let region = RegionAttributes {
            rects: vec![
                (RectangleKind::Add, Rectangle::from_size((10, 10).into())),
                (RectangleKind::Subtract, Rectangle::from_size((5, 5).into())),
            ],
        };

        assert!(!region.contains((0, 0)));
        assert!(region.contains((5, 5)));
    }

    #[test]
    fn region_attributes_add_subtract_add() {
        let region = RegionAttributes {
            rects: vec![
                (RectangleKind::Add, Rectangle::from_size((10, 10).into())),
                (RectangleKind::Subtract, Rectangle::from_size((5, 5).into())),
                (RectangleKind::Add, Rectangle::new((2, 2).into(), (2, 2).into())),
            ],
        };

        assert!(!region.contains((0, 0)));
        assert!(region.contains((5, 5)));
        assert!(region.contains((2, 2)));
    }
}
