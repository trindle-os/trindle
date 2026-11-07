//! Pointer-related types for smithay's input abstraction.

use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::{
    backend::input::{Axis, AxisRelativeDirection, AxisSource, ButtonState},
    input::{GrabStatus, Seat, SeatHandler},
    utils::Serial,
    utils::{Clock, IsAlive, Logical, Monotonic, Point},
};

mod cursor_image;
pub use cursor_icon::CursorIcon;
pub use cursor_image::{CursorImageAttributes, CursorImageStatus, CursorImageSurfaceData};

mod grab;
use grab::DefaultGrab;
pub use grab::{ClickGrab, GrabStartData, PointerGrab};
use tracing::{info_span, instrument};

pub struct PointerHandle<D: SeatHandler> {
    pub(crate) inner: Arc<Mutex<PointerInternal<D>>>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) wl_pointer: Arc<crate::wayland::seat::pointer::WlPointerHandle>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) wp_pointer_gestures: Arc<crate::wayland::pointer_gestures::WpPointerGesturePointerHandle>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) wp_relative: Arc<crate::wayland::relative_pointer::WpRelativePointerHandle>,
    pub(crate) span: tracing::Span,
}

#[cfg(not(feature = "wayland_frontend"))]
impl<D: SeatHandler> fmt::Debug for PointerHandle<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointerHandle")
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg(feature = "wayland_frontend")]
impl<D: SeatHandler> fmt::Debug for PointerHandle<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointerHandle")
            .field("inner", &self.inner)
            .field("wl_seat", &self.wl_pointer)
            .field("wp_pointer_gestures", &self.wp_pointer_gestures)
            .field("wp_relative", &self.wp_relative)
            .finish()
    }
}

impl<D: SeatHandler> Clone for PointerHandle<D> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            #[cfg(feature = "wayland_frontend")]
            wl_pointer: self.wl_pointer.clone(),
            #[cfg(feature = "wayland_frontend")]
            wp_pointer_gestures: self.wp_pointer_gestures.clone(),
            #[cfg(feature = "wayland_frontend")]
            wp_relative: self.wp_relative.clone(),
            span: self.span.clone(),
        }
    }
}

impl<D: SeatHandler> std::hash::Hash for PointerHandle<D> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state)
    }
}

impl<D: SeatHandler> std::cmp::PartialEq for PointerHandle<D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<D: SeatHandler> std::cmp::Eq for PointerHandle<D> {}

pub trait PointerTarget<D>: IsAlive + PartialEq + Clone + fmt::Debug + Send
where
    D: SeatHandler,
{
    fn enter(&self, seat: &Seat<D>, data: &mut D, event: &MotionEvent);
    fn motion(&self, seat: &Seat<D>, data: &mut D, event: &MotionEvent);
    fn relative_motion(&self, seat: &Seat<D>, data: &mut D, event: &RelativeMotionEvent);
    fn button(&self, seat: &Seat<D>, data: &mut D, event: &ButtonEvent);
    fn axis(&self, seat: &Seat<D>, data: &mut D, frame: AxisFrame);
    fn frame(&self, seat: &Seat<D>, data: &mut D);
    fn gesture_swipe_begin(&self, seat: &Seat<D>, data: &mut D, event: &GestureSwipeBeginEvent);
    fn gesture_swipe_update(&self, seat: &Seat<D>, data: &mut D, event: &GestureSwipeUpdateEvent);
    fn gesture_swipe_end(&self, seat: &Seat<D>, data: &mut D, event: &GestureSwipeEndEvent);
    fn gesture_pinch_begin(&self, seat: &Seat<D>, data: &mut D, event: &GesturePinchBeginEvent);
    fn gesture_pinch_update(&self, seat: &Seat<D>, data: &mut D, event: &GesturePinchUpdateEvent);
    fn gesture_pinch_end(&self, seat: &Seat<D>, data: &mut D, event: &GesturePinchEndEvent);
    fn gesture_hold_begin(&self, seat: &Seat<D>, data: &mut D, event: &GestureHoldBeginEvent);
    fn gesture_hold_end(&self, seat: &Seat<D>, data: &mut D, event: &GestureHoldEndEvent);
    fn leave(&self, seat: &Seat<D>, data: &mut D, serial: Serial, time: u32);
    fn replace(
        &self,
        replaced: <D as SeatHandler>::PointerFocus,
        seat: &Seat<D>,
        data: &mut D,
        event: &MotionEvent,
    ) {
        PointerTarget::<D>::leave(&replaced, seat, data, event.serial, event.time);
        data.cursor_image(seat, CursorImageStatus::default_named());
        PointerTarget::<D>::enter(self, seat, data, event);
    }
}

impl<D: SeatHandler + 'static> PointerHandle<D> {
    pub(crate) fn new() -> PointerHandle<D> {
        PointerHandle {
            inner: Arc::new(Mutex::new(PointerInternal::new())),
            #[cfg(feature = "wayland_frontend")]
            wl_pointer: Arc::new(crate::wayland::seat::pointer::WlPointerHandle::default()),
            #[cfg(feature = "wayland_frontend")]
            wp_pointer_gestures: Arc::new(
                crate::wayland::pointer_gestures::WpPointerGesturePointerHandle::default(),
            ),
            #[cfg(feature = "wayland_frontend")]
            wp_relative: Arc::new(crate::wayland::relative_pointer::WpRelativePointerHandle::default()),
            span: info_span!("input_pointer"),
        }
    }

    #[instrument(level = "debug", parent = &self.span, skip(self, data, grab))]
    pub fn set_grab<G: PointerGrab<D> + 'static>(&self, data: &mut D, grab: G, serial: Serial, focus: Focus) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .set_grab(data, &seat, serial, grab, focus);
    }

    #[instrument(level = "debug", parent = &self.span, skip(self, data))]
    pub fn unset_grab(&self, data: &mut D, serial: Serial, time: u32) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .unset_grab(data, &seat, serial, time, true);
    }

    pub fn has_grab(&self, serial: Serial) -> bool {
        let guard = self.inner.lock().unwrap();
        match guard.grab {
            GrabStatus::Active(s, _) => s == serial,
            _ => false,
        }
    }

    pub fn is_grabbed(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        !matches!(guard.grab, GrabStatus::None)
    }

    pub fn grab_start_data(&self) -> Option<GrabStartData<D>> {
        let guard = self.inner.lock().unwrap();
        match &guard.grab {
            GrabStatus::Active(_, g) => Some(g.start_data().clone()),
            _ => None,
        }
    }

    pub fn with_grab<T>(&self, f: impl FnOnce(Serial, &dyn PointerGrab<D>) -> T) -> Option<T> {
        let guard = self.inner.lock().unwrap();
        if let GrabStatus::Active(s, g) = &guard.grab {
            Some(f(*s, &**g))
        } else {
            None
        }
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data, focus), fields(focus = ?focus.as_ref().map(|(_, loc)| ("...", loc))))]
    pub fn motion(
        &self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        let mut inner = self.inner.lock().unwrap();
        inner.pending_focus.clone_from(&focus);
        let seat = self.get_seat(data);
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.motion(data, handle, focus, event);
        });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data, focus), fields(focus = ?focus.as_ref().map(|(_, loc)| ("...", loc))))]
    pub fn relative_motion(
        &self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        let mut inner = self.inner.lock().unwrap();
        inner.pending_focus.clone_from(&focus);
        let seat = self.get_seat(data);
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.relative_motion(data, handle, focus, event);
        });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn button(&self, data: &mut D, event: &ButtonEvent) {
        let mut inner = self.inner.lock().unwrap();
        match event.state {
            ButtonState::Pressed => {
                inner.pressed_buttons.push(event.button);
            }
            ButtonState::Released => {
                inner.pressed_buttons.retain(|b| *b != event.button);
            }
        }
        let seat = self.get_seat(data);
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.button(data, handle, event);
        });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn axis(&self, data: &mut D, details: AxisFrame) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.axis(data, handle, details);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn frame(&self, data: &mut D) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.frame(data, handle);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_swipe_begin(&self, data: &mut D, event: &GestureSwipeBeginEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_swipe_begin(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_swipe_update(&self, data: &mut D, event: &GestureSwipeUpdateEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_swipe_update(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_swipe_end(&self, data: &mut D, event: &GestureSwipeEndEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_swipe_end(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_pinch_begin(&self, data: &mut D, event: &GesturePinchBeginEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_pinch_begin(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_pinch_update(&self, data: &mut D, event: &GesturePinchUpdateEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_pinch_update(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_pinch_end(&self, data: &mut D, event: &GesturePinchEndEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_pinch_end(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_hold_begin(&self, data: &mut D, event: &GestureHoldBeginEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_hold_begin(data, handle, event);
            });
    }

    #[instrument(level = "trace", parent = &self.span, skip(self, data))]
    pub fn gesture_hold_end(&self, data: &mut D, event: &GestureHoldEndEvent) {
        let seat = self.get_seat(data);
        self.inner
            .lock()
            .unwrap()
            .with_grab(data, &seat, |data, handle, grab| {
                grab.gesture_hold_end(data, handle, event);
            });
    }

    pub fn current_location(&self) -> Point<f64, Logical> {
        self.inner.lock().unwrap().location
    }

    pub fn set_location(&self, location: Point<f64, Logical>) {
        self.inner.lock().unwrap().location = location;
    }

    #[cfg(feature = "wayland_frontend")]
    pub fn last_enter(&self) -> Option<Serial> {
        *self.wl_pointer.last_enter.lock().unwrap()
    }

    fn get_seat(&self, data: &mut D) -> Seat<D> {
        let seat_state = data.seat_state();
        seat_state
            .seats
            .iter()
            .find(|seat| seat.get_pointer().map(|h| &h == self).unwrap_or(false))
            .cloned()
            .unwrap()
    }
}

impl<D> PointerHandle<D>
where
    D: SeatHandler,
    <D as SeatHandler>::PointerFocus: Clone,
{
    pub fn current_focus(&self) -> Option<<D as SeatHandler>::PointerFocus> {
        self.inner.lock().unwrap().focus.clone().map(|(focus, _)| focus)
    }
}

pub struct PointerInnerHandle<'a, D: SeatHandler> {
    inner: &'a mut PointerInternal<D>,
    seat: &'a Seat<D>,
}

impl<D: SeatHandler> fmt::Debug for PointerInnerHandle<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointerInnerHandle")
            .field("inner", &self.inner)
            .field("seat", &self.seat.arc.name)
            .finish()
    }
}

impl<D: SeatHandler + 'static> PointerInnerHandle<'_, D> {
    pub fn set_grab<G: PointerGrab<D> + 'static>(
        &mut self,
        handler: &mut dyn PointerGrab<D>,
        data: &mut D,
        serial: Serial,
        focus: Focus,
        grab: G,
    ) {
        handler.unset(data);
        self.inner.set_grab(data, self.seat, serial, grab, focus);
    }

    pub fn unset_grab(
        &mut self,
        handler: &mut dyn PointerGrab<D>,
        data: &mut D,
        serial: Serial,
        time: u32,
        restore_focus: bool,
    ) {
        handler.unset(data);
        self.inner
            .unset_grab(data, self.seat, serial, time, restore_focus);
    }

    pub fn current_focus(&self) -> Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)> {
        self.inner.focus.clone()
    }

    pub fn current_location(&self) -> Point<f64, Logical> {
        self.inner.location
    }

    pub fn current_pressed(&self) -> &[u32] {
        &self.inner.pressed_buttons
    }

    pub fn motion(
        &mut self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        self.inner.motion(data, self.seat, focus, event);
    }

    pub fn relative_motion(
        &mut self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        self.inner.relative_motion(data, self.seat, focus, event);
    }

    pub fn button(&mut self, data: &mut D, event: &ButtonEvent) {
        if let Some((focused, _)) = self.inner.focus.as_mut() {
            focused.button(self.seat, data, event);
        }
    }

    pub fn axis(&mut self, data: &mut D, details: AxisFrame) {
        if let Some((focused, _)) = self.inner.focus.as_mut() {
            focused.axis(self.seat, data, details);
        }
    }

    pub fn frame(&mut self, data: &mut D) {
        if let Some((focused, _)) = self.inner.focus.as_mut() {
            focused.frame(self.seat, data);
        }
    }

    pub fn gesture_swipe_begin(&mut self, data: &mut D, event: &GestureSwipeBeginEvent) {
        self.inner.gesture_swipe_begin(data, self.seat, event);
    }

    pub fn gesture_swipe_update(&mut self, data: &mut D, event: &GestureSwipeUpdateEvent) {
        self.inner.gesture_swipe_update(data, self.seat, event);
    }

    pub fn gesture_swipe_end(&mut self, data: &mut D, event: &GestureSwipeEndEvent) {
        self.inner.gesture_swipe_end(data, self.seat, event);
    }

    pub fn gesture_pinch_begin(&mut self, data: &mut D, event: &GesturePinchBeginEvent) {
        self.inner.gesture_pinch_begin(data, self.seat, event);
    }

    pub fn gesture_pinch_update(&mut self, data: &mut D, event: &GesturePinchUpdateEvent) {
        self.inner.gesture_pinch_update(data, self.seat, event);
    }

    pub fn gesture_pinch_end(&mut self, data: &mut D, event: &GesturePinchEndEvent) {
        self.inner.gesture_pinch_end(data, self.seat, event);
    }

    pub fn gesture_hold_begin(&mut self, data: &mut D, event: &GestureHoldBeginEvent) {
        self.inner.gesture_hold_begin(data, self.seat, event);
    }

    pub fn gesture_hold_end(&mut self, data: &mut D, event: &GestureHoldEndEvent) {
        self.inner.gesture_hold_end(data, self.seat, event);
    }
}

pub(crate) struct PointerInternal<D: SeatHandler> {
    pub(crate) focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
    pending_focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
    location: Point<f64, Logical>,
    grab: GrabStatus<dyn PointerGrab<D>>,
    pressed_buttons: Vec<u32>,
}

impl<D: SeatHandler> fmt::Debug for PointerInternal<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointerInternal")
            .field("focus", &self.focus)
            .field("pending_focus", &self.pending_focus)
            .field("location", &self.location)
            .field("grab", &self.grab)
            .field("pressed_buttons", &self.pressed_buttons)
            .field("image_callback", &"...")
            .finish()
    }
}

impl<D: SeatHandler + 'static> PointerInternal<D> {
    fn new() -> Self {
        Self {
            focus: None,
            pending_focus: None,
            location: (0.0, 0.0).into(),
            grab: GrabStatus::None,
            pressed_buttons: Vec::new(),
        }
    }

    fn set_grab<G: PointerGrab<D> + 'static>(
        &mut self,
        data: &mut D,
        seat: &Seat<D>,
        serial: Serial,
        grab: G,
        focus: Focus,
    ) {
        if let GrabStatus::Active(_, handler) = &mut self.grab {
            handler.unset(data);
        }
        self.grab = GrabStatus::Active(serial, Box::new(grab));

        if matches!(focus, Focus::Clear) {
            let location = self.location;
            self.motion(
                data,
                seat,
                None,
                &MotionEvent {
                    location,
                    serial,
                    time: Clock::<Monotonic>::new().now().as_millis(),
                },
            );
        }
    }

    fn unset_grab(&mut self, data: &mut D, seat: &Seat<D>, serial: Serial, time: u32, restore_focus: bool) {
        if let GrabStatus::Active(_, handler) = &mut self.grab {
            handler.unset(data);
        }
        self.grab = GrabStatus::None;
        if restore_focus {
            let location = self.location;
            let focus = self.pending_focus.clone();
            self.motion(
                data,
                seat,
                focus,
                &MotionEvent {
                    location,
                    serial,
                    time,
                },
            );
        }
    }

    fn motion(
        &mut self,
        data: &mut D,
        seat: &Seat<D>,
        focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        self.location = event.location;
        if let Some((focus, loc)) = focus {
            let event = MotionEvent {
                location: event.location - loc,
                serial: event.serial,
                time: event.time,
            };
            let old_focus = self.focus.replace((focus.clone(), loc));
            match (focus, old_focus) {
                (focus, Some((old_focus, _))) if focus == old_focus => {
                    focus.motion(seat, data, &event);
                }
                (focus, Some((old_focus, _))) => {
                    focus.replace(old_focus, seat, data, &event);
                }
                (focus, None) => {
                    focus.enter(seat, data, &event);
                }
            };
        } else if let Some((old_focus, _)) = self.focus.take() {
            old_focus.leave(seat, data, event.serial, event.time);
            data.cursor_image(seat, CursorImageStatus::default_named());
        }
    }

    fn relative_motion(
        &mut self,
        data: &mut D,
        seat: &Seat<D>,
        _focus: Option<(<D as SeatHandler>::PointerFocus, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.relative_motion(seat, data, event);
        }
    }

    fn gesture_swipe_begin(&mut self, data: &mut D, seat: &Seat<D>, event: &GestureSwipeBeginEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_swipe_begin(seat, data, event);
        }
    }

    fn gesture_swipe_update(&mut self, data: &mut D, seat: &Seat<D>, event: &GestureSwipeUpdateEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_swipe_update(seat, data, event);
        }
    }

    fn gesture_swipe_end(&mut self, data: &mut D, seat: &Seat<D>, event: &GestureSwipeEndEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_swipe_end(seat, data, event);
        }
    }

    fn gesture_pinch_begin(&mut self, data: &mut D, seat: &Seat<D>, event: &GesturePinchBeginEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_pinch_begin(seat, data, event);
        }
    }

    fn gesture_pinch_update(&mut self, data: &mut D, seat: &Seat<D>, event: &GesturePinchUpdateEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_pinch_update(seat, data, event);
        }
    }

    fn gesture_pinch_end(&mut self, data: &mut D, seat: &Seat<D>, event: &GesturePinchEndEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_pinch_end(seat, data, event);
        }
    }

    fn gesture_hold_begin(&mut self, data: &mut D, seat: &Seat<D>, event: &GestureHoldBeginEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_hold_begin(seat, data, event);
        }
    }

    fn gesture_hold_end(&mut self, data: &mut D, seat: &Seat<D>, event: &GestureHoldEndEvent) {
        if let Some((focused, _)) = self.focus.as_mut() {
            focused.gesture_hold_end(seat, data, event);
        }
    }

    fn with_grab<F>(&mut self, data: &mut D, seat: &Seat<D>, f: F)
    where
        F: FnOnce(&mut D, &mut PointerInnerHandle<'_, D>, &mut dyn PointerGrab<D>),
    {
        let mut grab = std::mem::replace(&mut self.grab, GrabStatus::Borrowed);
        match grab {
            GrabStatus::Borrowed => panic!("Accessed a pointer grab from within a pointer grab access."),
            GrabStatus::Active(_, ref mut handler) => {
                if let Some((ref focus, _)) = handler.start_data().focus {
                    if !focus.alive() {
                        handler.unset(data);
                        self.grab = GrabStatus::None;
                        f(
                            data,
                            &mut PointerInnerHandle { inner: self, seat },
                            &mut DefaultGrab,
                        );
                        return;
                    }
                }
                f(
                    data,
                    &mut PointerInnerHandle { inner: self, seat },
                    &mut **handler,
                );
            }
            GrabStatus::None => {
                f(
                    data,
                    &mut PointerInnerHandle { inner: self, seat },
                    &mut DefaultGrab,
                );
            }
        }

        if let GrabStatus::Borrowed = self.grab {
            self.grab = grab;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Focus {
    Keep,
    Clear,
}

#[derive(Debug, Clone)]
pub struct MotionEvent {
    pub location: Point<f64, Logical>,
    pub serial: Serial,
    pub time: u32,
}

#[derive(Debug, Clone)]
pub struct RelativeMotionEvent {
    pub delta: Point<f64, Logical>,
    pub delta_unaccel: Point<f64, Logical>,
    pub utime: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonEvent {
    pub serial: Serial,
    pub time: u32,
    pub button: u32,
    pub state: ButtonState,
}

#[must_use = "AxisFrame uses a builder-like pattern, so its result must be used"]
#[derive(Copy, Clone, Debug)]
pub struct AxisFrame {
    pub source: Option<AxisSource>,
    pub relative_direction: (AxisRelativeDirection, AxisRelativeDirection),
    pub time: u32,
    pub axis: (f64, f64),
    pub v120: Option<(i32, i32)>,
    pub stop: (bool, bool),
}

#[derive(Debug, Clone)]
pub struct GestureSwipeBeginEvent {
    pub serial: Serial,
    pub time: u32,
    pub fingers: u32,
}

#[derive(Debug, Clone)]
pub struct GestureSwipeUpdateEvent {
    pub time: u32,
    pub delta: Point<f64, Logical>,
}

#[derive(Debug, Clone)]
pub struct GestureSwipeEndEvent {
    pub serial: Serial,
    pub time: u32,
    pub cancelled: bool,
}

#[derive(Debug, Clone)]
pub struct GesturePinchBeginEvent {
    pub serial: Serial,
    pub time: u32,
    pub fingers: u32,
}

#[derive(Debug, Clone)]
pub struct GesturePinchUpdateEvent {
    pub time: u32,
    pub delta: Point<f64, Logical>,
    pub scale: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone)]
pub struct GesturePinchEndEvent {
    pub serial: Serial,
    pub time: u32,
    pub cancelled: bool,
}

#[derive(Debug, Clone)]
pub struct GestureHoldBeginEvent {
    pub serial: Serial,
    pub time: u32,
    pub fingers: u32,
}

#[derive(Debug, Clone)]
pub struct GestureHoldEndEvent {
    pub serial: Serial,
    pub time: u32,
    pub cancelled: bool,
}

impl AxisFrame {
    pub fn new(time: u32) -> Self {
        AxisFrame {
            source: None,
            relative_direction: (AxisRelativeDirection::Identical, AxisRelativeDirection::Identical),
            time,
            axis: (0.0, 0.0),
            v120: None,
            stop: (false, false),
        }
    }

    pub fn source(mut self, source: AxisSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn relative_direction(mut self, axis: Axis, relative_direction: AxisRelativeDirection) -> Self {
        match axis {
            Axis::Horizontal => {
                self.relative_direction.0 = relative_direction;
            }
            Axis::Vertical => {
                self.relative_direction.1 = relative_direction;
            }
        };
        self
    }

    pub fn v120(mut self, axis: Axis, steps: i32) -> Self {
        let v120 = self.v120.get_or_insert_with(Default::default);
        match axis {
            Axis::Horizontal => {
                v120.0 = steps;
            }
            Axis::Vertical => {
                v120.1 = steps;
            }
        };
        self
    }

    pub fn value(mut self, axis: Axis, value: f64) -> Self {
        match axis {
            Axis::Horizontal => {
                self.axis.0 += value;
            }
            Axis::Vertical => {
                self.axis.1 += value;
            }
        };
        self
    }

    pub fn stop(mut self, axis: Axis) -> Self {
        match axis {
            Axis::Horizontal => {
                self.stop.0 = true;
            }
            Axis::Vertical => {
                self.stop.1 = true;
            }
        };
        self
    }
}
