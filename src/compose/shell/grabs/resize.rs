// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::cell::RefCell;

use smithay::{
    desktop::WindowSurface,
    input::pointer::{
        AxisFrame, ButtonEvent, CursorIcon, CursorImageStatus, Focus, GestureHoldBeginEvent,
        GestureHoldEndEvent, GesturePinchBeginEvent, GesturePinchEndEvent,
        GesturePinchUpdateEvent, GestureSwipeBeginEvent, GestureSwipeEndEvent,
        GestureSwipeUpdateEvent, GrabStartData as PointerGrabStartData, MotionEvent,
        PointerGrab, PointerInnerHandle, RelativeMotionEvent,
    },
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    utils::{IsAlive, Logical, Point, Serial, Size},
    wayland::{compositor::with_states, shell::xdg::SurfaceCachedState},
};
use smithay::{utils::Rectangle, xwayland::xwm::ResizeEdge as X11ResizeEdge};

use crate::shell::{SurfaceData, WindowElement};
use crate::{focus::PointerFocusTarget, state::{AnvilState, Backend}};

impl<BackendData: Backend> AnvilState<BackendData> {

    pub fn start_pointer_resize(
        &mut self,
        window: WindowElement,
        edges: ResizeEdge,
        start_data: PointerGrabStartData<AnvilState<BackendData>>,
        serial: Serial,
    ) {
        let Some(initial_window_location) = self.space.element_location(&window) else { return };

        let initial_window_size = window.client_size();
        let Some(surface) = window.wl_surface() else { return };
        with_states(&surface, |states| {
            states
                .data_map
                .insert_if_missing(|| RefCell::new(SurfaceData::default()));
            states.data_map.get::<RefCell<SurfaceData>>().unwrap().borrow_mut().resize_state =
                ResizeState::Resizing(ResizeData { edges, initial_window_location, initial_window_size });
        });
        let grab = PointerResizeSurfaceGrab {
            start_data,
            window,
            edges,
            initial_window_location,
            initial_window_size,
            last_window_size: initial_window_size,
        };
        let pointer = self.pointer.clone();
        pointer.set_grab(self, grab, serial, Focus::Clear);

        self.cursor_status = CursorImageStatus::Named(edges.cursor());
    }
}

impl ResizeEdge {

    pub fn cursor(self) -> CursorIcon {
        match self {
            ResizeEdge::TOP => CursorIcon::NResize,
            ResizeEdge::BOTTOM => CursorIcon::SResize,
            ResizeEdge::LEFT => CursorIcon::WResize,
            ResizeEdge::RIGHT => CursorIcon::EResize,
            ResizeEdge::TOP_LEFT => CursorIcon::NwResize,
            ResizeEdge::TOP_RIGHT => CursorIcon::NeResize,
            ResizeEdge::BOTTOM_LEFT => CursorIcon::SwResize,
            ResizeEdge::BOTTOM_RIGHT => CursorIcon::SeResize,
            _ => CursorIcon::Default,
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ResizeEdge: u32 {
        const NONE = 0;
        const TOP = 1;
        const BOTTOM = 2;
        const LEFT = 4;
        const TOP_LEFT = 5;
        const BOTTOM_LEFT = 6;
        const RIGHT = 8;
        const TOP_RIGHT = 9;
        const BOTTOM_RIGHT = 10;
    }
}

impl From<xdg_toplevel::ResizeEdge> for ResizeEdge {
    #[inline]
    fn from(x: xdg_toplevel::ResizeEdge) -> Self {
        Self::from_bits(x as u32).unwrap()
    }
}

impl From<ResizeEdge> for xdg_toplevel::ResizeEdge {
    #[inline]
    fn from(x: ResizeEdge) -> Self {
        Self::try_from(x.bits()).unwrap()
    }
}

impl From<X11ResizeEdge> for ResizeEdge {
    #[inline]
    fn from(edge: X11ResizeEdge) -> Self {
        match edge {
            X11ResizeEdge::Bottom => ResizeEdge::BOTTOM,
            X11ResizeEdge::BottomLeft => ResizeEdge::BOTTOM_LEFT,
            X11ResizeEdge::BottomRight => ResizeEdge::BOTTOM_RIGHT,
            X11ResizeEdge::Left => ResizeEdge::LEFT,
            X11ResizeEdge::Right => ResizeEdge::RIGHT,
            X11ResizeEdge::Top => ResizeEdge::TOP,
            X11ResizeEdge::TopLeft => ResizeEdge::TOP_LEFT,
            X11ResizeEdge::TopRight => ResizeEdge::TOP_RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ResizeData {

    pub edges: ResizeEdge,

    pub initial_window_location: Point<i32, Logical>,

    pub initial_window_size: Size<i32, Logical>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum ResizeState {

    #[default]
    NotResizing,

    Resizing(ResizeData),

    WaitingForFinalAck(ResizeData, Serial),

    WaitingForCommit(ResizeData),
}

pub struct PointerResizeSurfaceGrab<BackendData: Backend + 'static> {
    pub start_data: PointerGrabStartData<AnvilState<BackendData>>,
    pub window: WindowElement,
    pub edges: ResizeEdge,
    pub initial_window_location: Point<i32, Logical>,
    pub initial_window_size: Size<i32, Logical>,
    pub last_window_size: Size<i32, Logical>,
}

impl<BackendData: Backend> PointerResizeSurfaceGrab<BackendData> {
    fn resizing(&self) -> Resizing<'_> {
        Resizing {
            window: &self.window,
            edges: self.edges,
            initial_window_location: self.initial_window_location,
            initial_window_size: self.initial_window_size,
            last_window_size: self.last_window_size,
        }
    }
}

pub(super) struct Resizing<'a> {
    pub(super) window: &'a WindowElement,
    pub(super) edges: ResizeEdge,
    pub(super) initial_window_location: Point<i32, Logical>,
    pub(super) initial_window_size: Size<i32, Logical>,
    pub(super) last_window_size: Size<i32, Logical>,
}

impl Resizing<'_> {

    pub(super) fn size_for(&self, delta: Point<f64, Logical>) -> Size<i32, Logical> {
        let (mut dx, mut dy): (f64, f64) = delta.into();

        let mut new_window_width = self.initial_window_size.w;
        let mut new_window_height = self.initial_window_size.h;

        let left_right = ResizeEdge::LEFT | ResizeEdge::RIGHT;
        let top_bottom = ResizeEdge::TOP | ResizeEdge::BOTTOM;

        if self.edges.intersects(left_right) {
            if self.edges.intersects(ResizeEdge::LEFT) {
                dx = -dx;
            }

            new_window_width = (self.initial_window_size.w as f64 + dx) as i32;
        }

        if self.edges.intersects(top_bottom) {
            if self.edges.intersects(ResizeEdge::TOP) {
                dy = -dy;
            }

            new_window_height = (self.initial_window_size.h as f64 + dy) as i32;
        }

        let (min_size, max_size) = if let Some(surface) = self.window.wl_surface() {
            with_states(&surface, |states| {
                let mut guard = states.cached_state.get::<SurfaceCachedState>();
                let data = guard.current();
                (data.min_size, data.max_size)
            })
        } else {
            ((0, 0).into(), (0, 0).into())
        };

        let min_width = min_size.w.max(1);
        let min_height = min_size.h.max(1);
        let max_width = if max_size.w == 0 { i32::MAX } else { max_size.w };
        let max_height = if max_size.h == 0 { i32::MAX } else { max_size.h };

        new_window_width = new_window_width.max(min_width).min(max_width);
        new_window_height = new_window_height.max(min_height).min(max_height);

        (new_window_width, new_window_height).into()
    }

    pub(super) fn ask<BackendData: Backend>(&self, data: &mut AnvilState<BackendData>, size: Size<i32, Logical>) {
        match &self.window.0.underlying_surface() {
            WindowSurface::Wayland(xdg) => {
                xdg.with_pending_state(|state| {
                    state.states.set(xdg_toplevel::State::Resizing);
                    state.size = Some(size);
                });
                xdg.send_pending_configure();
            }
            WindowSurface::X11(x11) => {
                let location = data.space.element_location(self.window).unwrap_or_default();
                let _ = x11.configure(Rectangle::new(location + self.window.deco_offset(), size));
            }
        }
    }

    pub(super) fn finish<BackendData: Backend>(&self, data: &mut AnvilState<BackendData>, serial: Serial) {
        match &self.window.0.underlying_surface() {
            WindowSurface::Wayland(xdg) => {
                xdg.with_pending_state(|state| {
                    state.states.unset(xdg_toplevel::State::Resizing);
                    state.size = Some(self.last_window_size);
                });
                xdg.send_pending_configure();
                self.anchor_far_edges(data);

                let Some(live) = self.window.wl_surface() else {
                    return;
                };
                Self::next_state(&live, |resize_data| ResizeState::WaitingForFinalAck(resize_data, serial));
            }
            WindowSurface::X11(x11) => {
                let location = self.anchor_far_edges(data);
                let _ = x11.configure(Rectangle::new(location + self.window.deco_offset(), self.last_window_size));

                let Some(surface) = self.window.wl_surface() else {

                    return;
                };
                Self::next_state(&surface, ResizeState::WaitingForCommit);
            }
        }
    }

    fn anchor_far_edges<BackendData: Backend>(&self, data: &mut AnvilState<BackendData>) -> Point<i32, Logical> {
        let mut location = data.space.element_location(self.window).unwrap_or_default();
        if self.edges.intersects(ResizeEdge::TOP_LEFT) {
            let geometry = smithay::utils::Rectangle::<i32, Logical>::from_size(self.window.client_size());

            if self.edges.intersects(ResizeEdge::LEFT) {
                location.x = self.initial_window_location.x
                    + (self.initial_window_size.w - geometry.size.w);
            }
            if self.edges.intersects(ResizeEdge::TOP) {
                location.y = self.initial_window_location.y
                    + (self.initial_window_size.h - geometry.size.h);
            }

            data.space.map_element(self.window.clone(), location, true);
        }
        location
    }

    fn next_state(surface: &WlSurface, next: impl FnOnce(ResizeData) -> ResizeState) {
        with_states(surface, |states| {
            let mut data = states
                .data_map
                .get::<RefCell<SurfaceData>>()
                .unwrap()
                .borrow_mut();
            if let ResizeState::Resizing(resize_data) = data.resize_state {
                data.resize_state = next(resize_data);
            }
        });
    }
}

impl<BackendData: Backend> PointerGrab<AnvilState<BackendData>> for PointerResizeSurfaceGrab<BackendData> {
    fn motion(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        _focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {

        handle.motion(data, None, event);

        if !self.window.alive() {
            handle.unset_grab(self, data, event.serial, event.time, true);
            return;
        }

        let size = self.resizing().size_for(event.location - self.start_data.location);
        self.last_window_size = size;
        self.resizing().ask(data, size);
    }

    fn relative_motion(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        handle.relative_motion(data, focus, event);
    }

    fn button(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &ButtonEvent,
    ) {
        handle.button(data, event);
        if handle.current_pressed().is_empty() {

            handle.unset_grab(self, data, event.serial, event.time, true);

            if !self.window.alive() {
                return;
            }

            self.resizing().finish(data, event.serial);
        }
    }

    fn axis(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        details: AxisFrame,
    ) {
        handle.axis(data, details)
    }

    fn frame(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
    ) {
        handle.frame(data);
    }

    fn gesture_swipe_begin(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GestureSwipeBeginEvent,
    ) {
        handle.gesture_swipe_begin(data, event);
    }

    fn gesture_swipe_update(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GestureSwipeUpdateEvent,
    ) {
        handle.gesture_swipe_update(data, event);
    }

    fn gesture_swipe_end(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GestureSwipeEndEvent,
    ) {
        handle.gesture_swipe_end(data, event);
    }

    fn gesture_pinch_begin(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GesturePinchBeginEvent,
    ) {
        handle.gesture_pinch_begin(data, event);
    }

    fn gesture_pinch_update(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GesturePinchUpdateEvent,
    ) {
        handle.gesture_pinch_update(data, event);
    }

    fn gesture_pinch_end(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GesturePinchEndEvent,
    ) {
        handle.gesture_pinch_end(data, event);
    }

    fn gesture_hold_begin(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GestureHoldBeginEvent,
    ) {
        handle.gesture_hold_begin(data, event);
    }

    fn gesture_hold_end(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        event: &GestureHoldEndEvent,
    ) {
        handle.gesture_hold_end(data, event);
    }

    fn start_data(&self) -> &PointerGrabStartData<AnvilState<BackendData>> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut AnvilState<BackendData>) {}
}
