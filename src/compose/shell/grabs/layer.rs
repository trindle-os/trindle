// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    input::pointer::{
        AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent,
        GesturePinchBeginEvent, GesturePinchEndEvent, GesturePinchUpdateEvent,
        GestureSwipeBeginEvent, GestureSwipeEndEvent, GestureSwipeUpdateEvent,
        GrabStartData as PointerGrabStartData, MotionEvent, PointerGrab, PointerInnerHandle,
        RelativeMotionEvent,
    },
    utils::{Logical, Point},
};

use crate::{focus::PointerFocusTarget, state::{AnvilState, Backend}};

pub struct LayerClickGrab<BackendData: Backend + 'static> {
    pub start_data: PointerGrabStartData<AnvilState<BackendData>>,
    pub layer: smithay::desktop::LayerSurface,
    pub output: smithay::output::Output,

    pub was_at: Point<i32, Logical>,
}

impl<BackendData: Backend> LayerClickGrab<BackendData> {
    fn focus_now(&self) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        let now = smithay::desktop::layer_map_for_output(&self.output)
            .layer_geometry(&self.layer)
            .map(|g| g.loc)
            .unwrap_or(self.was_at);
        let moved = (now - self.was_at).to_f64();
        self.start_data.focus.clone().map(|(f, at)| (f, at + moved))
    }
}

impl<BackendData: Backend> PointerGrab<AnvilState<BackendData>> for LayerClickGrab<BackendData> {
    fn motion(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        _focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        let focus = self.focus_now();
        handle.motion(data, focus, event);
    }

    fn relative_motion(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>,
        _focus: Option<(PointerFocusTarget, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        let focus = self.focus_now();
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
            handle.unset_grab(self, data, event.serial, event.time, false);
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

    fn frame(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>) {
        handle.frame(data);
    }

    fn gesture_swipe_begin(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GestureSwipeBeginEvent) {
        handle.gesture_swipe_begin(data, event);
    }

    fn gesture_swipe_update(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GestureSwipeUpdateEvent) {
        handle.gesture_swipe_update(data, event);
    }

    fn gesture_swipe_end(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GestureSwipeEndEvent) {
        handle.gesture_swipe_end(data, event);
    }

    fn gesture_pinch_begin(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GesturePinchBeginEvent) {
        handle.gesture_pinch_begin(data, event);
    }

    fn gesture_pinch_update(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GesturePinchUpdateEvent) {
        handle.gesture_pinch_update(data, event);
    }

    fn gesture_pinch_end(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GesturePinchEndEvent) {
        handle.gesture_pinch_end(data, event);
    }

    fn gesture_hold_begin(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GestureHoldBeginEvent) {
        handle.gesture_hold_begin(data, event);
    }

    fn gesture_hold_end(&mut self, data: &mut AnvilState<BackendData>, handle: &mut PointerInnerHandle<'_, AnvilState<BackendData>>, event: &GestureHoldEndEvent) {
        handle.gesture_hold_end(data, event);
    }

    fn start_data(&self) -> &PointerGrabStartData<AnvilState<BackendData>> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut AnvilState<BackendData>) {}
}
