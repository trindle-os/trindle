// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    input::{
        pointer::{
            AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent,
            GesturePinchBeginEvent, GesturePinchEndEvent, GesturePinchUpdateEvent,
            GestureSwipeBeginEvent, GestureSwipeEndEvent, GestureSwipeUpdateEvent,
            GrabStartData as PointerGrabStartData, MotionEvent, PointerGrab, PointerInnerHandle,
            RelativeMotionEvent,
        },
        touch::{GrabStartData as TouchGrabStartData, TouchGrab},
    },
    utils::{Logical, Point, Serial},
};
use smithay::utils::Rectangle;

use crate::shell::WindowElement;
use crate::{focus::PointerFocusTarget, state::{Compose, Backend}};

fn move_window<BackendData: Backend>(
    data: &mut Compose<BackendData>,
    window: &WindowElement,
    loc: Point<i32, Logical>,
) {
    data.space.map_element(window.clone(), loc, true);
    if let Some(x11) = window.0.x11_surface() {
        let _ = x11.configure(Rectangle::new(loc + window.deco_offset(), x11.geometry().size));
    }
}

const DRAG_START: f64 = 6.0;

fn restore_when_dragged<BackendData: Backend>(
    data: &mut Compose<BackendData>,
    window: &WindowElement,
    start: Point<f64, Logical>,
    delta: Point<f64, Logical>,
    pending: &mut bool,
    from: &mut Point<i32, Logical>,
) -> bool {
    if !*pending {
        return true;
    }
    if delta.x.abs().max(delta.y.abs()) < DRAG_START {
        return false;
    }
    *pending = false;
    if let Some(at) = data.restore_for_drag(window, start) {
        *from = at;
    }
    true
}

pub struct PointerMoveSurfaceGrab<BackendData: Backend + 'static> {
    pub start_data: PointerGrabStartData<Compose<BackendData>>,
    pub window: WindowElement,
    pub initial_window_location: Point<i32, Logical>,

    pub restore_pending: bool,
}

impl<BackendData: Backend> PointerGrab<Compose<BackendData>> for PointerMoveSurfaceGrab<BackendData> {
    fn motion(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        _focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {

        handle.motion(data, None, event);

        let delta = event.location - self.start_data.location;
        let start = self.start_data.location;
        let (pending, from) = (&mut self.restore_pending, &mut self.initial_window_location);
        if !restore_when_dragged(data, &self.window, start, delta, pending, from) {
            return;
        }
        let new_location = self.initial_window_location.to_f64() + delta;

        move_window(data, &self.window, new_location.to_i32_round());
    }

    fn relative_motion(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        handle.relative_motion(data, focus, event);
    }

    fn button(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &ButtonEvent,
    ) {
        handle.button(data, event);
        if handle.current_pressed().is_empty() {

            handle.unset_grab(self, data, event.serial, event.time, true);
        }
    }

    fn axis(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        details: AxisFrame,
    ) {
        handle.axis(data, details)
    }

    fn frame(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
    ) {
        handle.frame(data);
    }

    fn gesture_swipe_begin(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GestureSwipeBeginEvent,
    ) {
        handle.gesture_swipe_begin(data, event);
    }

    fn gesture_swipe_update(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GestureSwipeUpdateEvent,
    ) {
        handle.gesture_swipe_update(data, event);
    }

    fn gesture_swipe_end(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GestureSwipeEndEvent,
    ) {
        handle.gesture_swipe_end(data, event);
    }

    fn gesture_pinch_begin(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GesturePinchBeginEvent,
    ) {
        handle.gesture_pinch_begin(data, event);
    }

    fn gesture_pinch_update(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GesturePinchUpdateEvent,
    ) {
        handle.gesture_pinch_update(data, event);
    }

    fn gesture_pinch_end(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GesturePinchEndEvent,
    ) {
        handle.gesture_pinch_end(data, event);
    }

    fn gesture_hold_begin(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GestureHoldBeginEvent,
    ) {
        handle.gesture_hold_begin(data, event);
    }

    fn gesture_hold_end(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut PointerInnerHandle<'_, Compose<BackendData>>,
        event: &GestureHoldEndEvent,
    ) {
        handle.gesture_hold_end(data, event);
    }

    fn start_data(&self) -> &PointerGrabStartData<Compose<BackendData>> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut Compose<BackendData>) {}
}

pub struct TouchMoveSurfaceGrab<BackendData: Backend + 'static> {
    pub start_data: TouchGrabStartData<Compose<BackendData>>,
    pub window: WindowElement,
    pub initial_window_location: Point<i32, Logical>,

    pub restore_pending: bool,
}

impl<BackendData: Backend> TouchGrab<Compose<BackendData>> for TouchMoveSurfaceGrab<BackendData> {
    fn down(
        &mut self,
        _data: &mut Compose<BackendData>,
        _handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        _focus: Option<(
            <Compose<BackendData> as smithay::input::SeatHandler>::TouchFocus,
            Point<f64, Logical>,
        )>,
        _event: &smithay::input::touch::DownEvent,
        _seq: Serial,
    ) {
    }

    fn up(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        event: &smithay::input::touch::UpEvent,
        seq: Serial,
    ) {
        if event.slot != self.start_data.slot {
            return;
        }

        handle.up(data, event, seq);
        handle.unset_grab(self, data);
    }

    fn motion(
        &mut self,
        data: &mut Compose<BackendData>,
        _handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        _focus: Option<(
            <Compose<BackendData> as smithay::input::SeatHandler>::TouchFocus,
            Point<f64, Logical>,
        )>,
        event: &smithay::input::touch::MotionEvent,
        _seq: Serial,
    ) {
        if event.slot != self.start_data.slot {
            return;
        }

        let delta = event.location - self.start_data.location;
        let start = self.start_data.location;
        let (pending, from) = (&mut self.restore_pending, &mut self.initial_window_location);
        if !restore_when_dragged(data, &self.window, start, delta, pending, from) {
            return;
        }
        let new_location = self.initial_window_location.to_f64() + delta;
        move_window(data, &self.window, new_location.to_i32_round());
    }

    fn frame(
        &mut self,
        _data: &mut Compose<BackendData>,
        _handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        _seq: Serial,
    ) {
    }

    fn cancel(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        seq: Serial,
    ) {
        handle.cancel(data, seq);
        handle.unset_grab(self, data);
    }

    fn shape(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        event: &smithay::input::touch::ShapeEvent,
        seq: Serial,
    ) {
        handle.shape(data, event, seq);
    }

    fn orientation(
        &mut self,
        data: &mut Compose<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, Compose<BackendData>>,
        event: &smithay::input::touch::OrientationEvent,
        seq: Serial,
    ) {
        handle.orientation(data, event, seq);
    }

    fn start_data(&self) -> &smithay::input::touch::GrabStartData<Compose<BackendData>> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut Compose<BackendData>) {}
}
