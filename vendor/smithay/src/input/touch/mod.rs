//! Touch-related types for smithay's input abstraction.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use tracing::{info_span, instrument};
#[cfg(feature = "wayland_frontend")]
use wayland_server::Weak;

use crate::backend::input::TouchSlot;
use crate::utils::{IsAlive, Logical, Point, Serial, SerialCounter};

pub use grab::{DefaultGrab, GrabStartData, TouchDownGrab, TouchGrab};

use super::{GrabStatus, Seat, SeatHandler};

mod grab;

pub struct TouchHandle<D: SeatHandler> {
    pub(crate) inner: Arc<Mutex<TouchInternal<D>>>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) known_instances:
        Arc<Mutex<Vec<(Weak<wayland_server::protocol::wl_touch::WlTouch>, Option<Serial>)>>>,
    pub(crate) span: tracing::Span,
}

#[cfg(not(feature = "wayland_frontend"))]
impl<D: SeatHandler> fmt::Debug for TouchHandle<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchHandle").field("inner", &self.inner).finish()
    }
}

#[cfg(feature = "wayland_frontend")]
impl<D: SeatHandler> fmt::Debug for TouchHandle<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchHandle")
            .field("inner", &self.inner)
            .field("known_instances", &self.known_instances)
            .finish()
    }
}

impl<D: SeatHandler> Clone for TouchHandle<D> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            #[cfg(feature = "wayland_frontend")]
            known_instances: self.known_instances.clone(),
            span: self.span.clone(),
        }
    }
}

impl<D: SeatHandler> std::hash::Hash for TouchHandle<D> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state)
    }
}

impl<D: SeatHandler> std::cmp::PartialEq for TouchHandle<D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<D: SeatHandler> std::cmp::Eq for TouchHandle<D> {}

pub(crate) struct TouchInternal<D: SeatHandler> {
    focus: HashMap<TouchSlot, TouchSlotState<D>>,
    seq_counter: SerialCounter,
    default_grab: Box<dyn Fn() -> Box<dyn TouchGrab<D>> + Send + 'static>,
    grab: GrabStatus<dyn TouchGrab<D>>,
}

struct TouchSlotState<D: SeatHandler> {
    focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
    frame_pending: Option<<D as SeatHandler>::TouchFocus>,
    pending: Serial,
    current: Option<Serial>,
}

impl<D: SeatHandler> fmt::Debug for TouchSlotState<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchSlotState")
            .field("focus", &self.focus)
            .field("frame_pending", &self.frame_pending)
            .field("pending", &self.pending)
            .field("current", &self.current)
            .finish()
    }
}

impl<D: SeatHandler> fmt::Debug for TouchInternal<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchInternal")
            .field("focus", &self.focus)
            .field("grab", &self.grab)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct DownEvent {
    pub slot: TouchSlot,
    pub location: Point<f64, Logical>,
    pub serial: Serial,
    pub time: u32,
}

#[derive(Debug, Clone)]
pub struct UpEvent {
    pub slot: TouchSlot,
    pub serial: Serial,
    pub time: u32,
}

#[derive(Debug, Clone)]
pub struct MotionEvent {
    pub slot: TouchSlot,
    pub location: Point<f64, Logical>,
    pub time: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeEvent {
    pub slot: TouchSlot,
    pub major: f64,
    pub minor: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct OrientationEvent {
    pub slot: TouchSlot,
    pub orientation: f64,
}

pub trait TouchTarget<D>: IsAlive + PartialEq + Clone + fmt::Debug + Send
where
    D: SeatHandler,
{
    fn down(&self, seat: &Seat<D>, data: &mut D, event: &DownEvent, seq: Serial);

    fn up(&self, seat: &Seat<D>, data: &mut D, event: &UpEvent, seq: Serial);

    fn motion(&self, seat: &Seat<D>, data: &mut D, event: &MotionEvent, seq: Serial);

    fn frame(&self, seat: &Seat<D>, data: &mut D, seq: Serial);

    fn cancel(&self, seat: &Seat<D>, data: &mut D, seq: Serial);

    fn shape(&self, seat: &Seat<D>, data: &mut D, event: &ShapeEvent, seq: Serial);

    fn orientation(&self, seat: &Seat<D>, data: &mut D, event: &OrientationEvent, seq: Serial);
}

impl<D: SeatHandler + 'static> TouchHandle<D> {
    pub(crate) fn new<F>(default_grab: F) -> TouchHandle<D>
    where
        F: Fn() -> Box<dyn TouchGrab<D>> + Send + 'static,
    {
        TouchHandle {
            inner: Arc::new(Mutex::new(TouchInternal::new(default_grab))),
            #[cfg(feature = "wayland_frontend")]
            known_instances: Arc::new(Mutex::new(Vec::new())),
            span: info_span!("input_touch"),
        }
    }

    #[instrument(level = "debug", parent = &self.span, skip(self, data, grab))]
    pub fn set_grab<G: TouchGrab<D> + 'static>(&self, data: &mut D, grab: G, serial: Serial) {
        let seat = self.get_seat(data);
        self.inner.lock().unwrap().set_grab(data, &seat, serial, grab);
    }

    #[instrument(level = "debug", parent = &self.span, skip(self, data))]
    pub fn unset_grab(&self, data: &mut D) {
        let seat = self.get_seat(data);
        self.inner.lock().unwrap().unset_grab(data, &seat);
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

    pub fn with_grab<T>(&self, f: impl FnOnce(Serial, &dyn TouchGrab<D>) -> T) -> Option<T> {
        let guard = self.inner.lock().unwrap();
        if let GrabStatus::Active(s, g) = &guard.grab {
            Some(f(*s, &**g))
        } else {
            None
        }
    }

    pub fn down(
        &self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
    ) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.down(data, handle, focus, event, seq);
        });
    }

    pub fn up(&self, data: &mut D, event: &UpEvent) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.up(data, handle, event, seq);
        });
    }

    pub fn motion(
        &self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.motion(data, handle, focus, event, seq);
        });
    }

    pub fn frame(&self, data: &mut D) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.frame(data, handle, seq);
        });
    }

    pub fn cancel(&self, data: &mut D) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.cancel(data, handle, seq);
        });
    }

    pub fn shape(&self, data: &mut D, event: &ShapeEvent) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.shape(data, handle, event, seq);
        });
    }

    pub fn orientation(&self, data: &mut D, event: &OrientationEvent) {
        let mut inner = self.inner.lock().unwrap();
        let seat = self.get_seat(data);
        let seq = inner.seq_counter.next_serial();
        inner.with_grab(data, &seat, |data, handle, grab| {
            grab.orientation(data, handle, event, seq);
        });
    }

    fn get_seat(&self, data: &mut D) -> Seat<D> {
        let seat_state = data.seat_state();
        seat_state
            .seats
            .iter()
            .find(|seat| seat.get_touch().map(|h| &h == self).unwrap_or(false))
            .cloned()
            .unwrap()
    }
}

pub struct TouchInnerHandle<'a, D: SeatHandler> {
    inner: &'a mut TouchInternal<D>,
    seat: &'a Seat<D>,
}

impl<D: SeatHandler> fmt::Debug for TouchInnerHandle<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchInnerHandle")
            .field("inner", &self.inner)
            .field("seat", &self.seat.arc.name)
            .finish()
    }
}

impl<D: SeatHandler + 'static> TouchInnerHandle<'_, D> {
    pub fn set_grab<G: TouchGrab<D> + 'static>(
        &mut self,
        handler: &mut dyn TouchGrab<D>,
        data: &mut D,
        serial: Serial,
        grab: G,
    ) {
        handler.unset(data);
        self.inner.set_grab(data, self.seat, serial, grab);
    }

    pub fn unset_grab(&mut self, handler: &mut dyn TouchGrab<D>, data: &mut D) {
        handler.unset(data);
        self.inner.unset_grab(data, self.seat);
    }

    pub fn down(
        &mut self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
        seq: Serial,
    ) {
        self.inner.down(data, self.seat, focus, event, seq)
    }

    pub fn up(&mut self, data: &mut D, event: &UpEvent, seq: Serial) {
        self.inner.up(data, self.seat, event, seq)
    }

    pub fn motion(
        &mut self,
        data: &mut D,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
        seq: Serial,
    ) {
        self.inner.motion(data, self.seat, focus, event, seq)
    }

    pub fn frame(&mut self, data: &mut D, seq: Serial) {
        self.inner.frame(data, self.seat, seq)
    }

    pub fn shape(&mut self, data: &mut D, event: &ShapeEvent, seq: Serial) {
        self.inner.shape(data, self.seat, event, seq)
    }

    pub fn orientation(&mut self, data: &mut D, event: &OrientationEvent, seq: Serial) {
        self.inner.orientation(data, self.seat, event, seq)
    }

    pub fn cancel(&mut self, data: &mut D, seq: Serial) {
        self.inner.cancel(data, self.seat, seq)
    }
}

impl<D: SeatHandler + 'static> TouchInternal<D> {
    fn new<F>(default_grab: F) -> Self
    where
        F: Fn() -> Box<dyn TouchGrab<D>> + Send + 'static,
    {
        Self {
            focus: Default::default(),
            seq_counter: SerialCounter::new(),
            default_grab: Box::new(default_grab),
            grab: GrabStatus::None,
        }
    }

    fn set_grab<G: TouchGrab<D> + 'static>(
        &mut self,
        data: &mut D,
        _seat: &Seat<D>,
        serial: Serial,
        grab: G,
    ) {
        if let GrabStatus::Active(_, handler) = &mut self.grab {
            handler.unset(data);
        }
        self.grab = GrabStatus::Active(serial, Box::new(grab));
    }

    fn unset_grab(&mut self, data: &mut D, _seat: &Seat<D>) {
        if let GrabStatus::Active(_, handler) = &mut self.grab {
            handler.unset(data);
        }
        self.grab = GrabStatus::None;
    }

    fn down(
        &mut self,
        data: &mut D,
        seat: &Seat<D>,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
        seq: Serial,
    ) {
        self.focus
            .entry(event.slot)
            .and_modify(|state| {
                state.pending = seq;
                state.frame_pending = None;
                state.focus.clone_from(&focus);
            })
            .or_insert_with(|| TouchSlotState {
                focus,
                frame_pending: None,
                pending: seq,
                current: None,
            });
        let state = self.focus.get(&event.slot).unwrap();
        if let Some((focus, loc)) = state.focus.as_ref() {
            let mut new_event = event.clone();
            new_event.location -= *loc;
            focus.down(seat, data, &new_event, seq);
        }
    }

    fn up(&mut self, data: &mut D, seat: &Seat<D>, event: &UpEvent, seq: Serial) {
        let Some(state) = self.focus.get_mut(&event.slot) else {
            return;
        };
        state.pending = seq;
        if let Some((focus, _)) = state.focus.take() {
            focus.up(seat, data, event, seq);

            state.frame_pending = Some(focus);
        }
    }

    fn motion(
        &mut self,
        data: &mut D,
        seat: &Seat<D>,
        _focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
        seq: Serial,
    ) {
        let Some(state) = self.focus.get_mut(&event.slot) else {
            return;
        };
        state.pending = seq;
        if let Some((focus, loc)) = state.focus.as_ref() {
            let mut new_event = event.clone();
            new_event.location -= *loc;
            focus.motion(seat, data, &new_event, seq);
        }
    }

    fn frame(&mut self, data: &mut D, seat: &Seat<D>, seq: Serial) {
        for state in self.focus.values_mut() {
            if state.current.map(|c| c >= state.pending).unwrap_or(false) {
                continue;
            }
            state.current = Some(seq);

            if let Some(focus) = state.frame_pending.take() {
                focus.frame(seat, data, seq);
            }

            if let Some((focus, _)) = state.focus.as_ref() {
                focus.frame(seat, data, seq);
            }
        }
    }

    fn cancel(&mut self, data: &mut D, seat: &Seat<D>, seq: Serial) {
        for state in self.focus.values_mut() {
            if state.current.map(|c| c >= state.pending).unwrap_or(false) {
                continue;
            }
            state.current = Some(seq);
            if let Some((focus, _)) = state.focus.take() {
                focus.cancel(seat, data, seq);
            }
        }
    }

    fn shape(&mut self, data: &mut D, seat: &Seat<D>, event: &ShapeEvent, seq: Serial) {
        let Some(state) = self.focus.get_mut(&event.slot) else {
            return;
        };
        state.pending = seq;
        if let Some((focus, _)) = state.focus.as_ref() {
            focus.shape(seat, data, event, seq);
        }
    }

    fn orientation(&mut self, data: &mut D, seat: &Seat<D>, event: &OrientationEvent, seq: Serial) {
        let Some(state) = self.focus.get_mut(&event.slot) else {
            return;
        };
        state.pending = seq;
        if let Some((focus, _)) = state.focus.as_ref() {
            focus.orientation(seat, data, event, seq);
        }
    }

    fn with_grab<F>(&mut self, data: &mut D, seat: &Seat<D>, f: F)
    where
        F: FnOnce(&mut D, &mut TouchInnerHandle<'_, D>, &mut dyn TouchGrab<D>),
    {
        let mut grab = std::mem::replace(&mut self.grab, GrabStatus::Borrowed);
        match grab {
            GrabStatus::Borrowed => panic!("Accessed a touch grab from within a touch grab access."),
            GrabStatus::Active(_, ref mut handler) => {
                if let Some((ref focus, _)) = handler.start_data().focus {
                    if !focus.alive() {
                        handler.unset(data);
                        self.grab = GrabStatus::None;
                        let mut default_grab = (self.default_grab)();
                        f(
                            data,
                            &mut TouchInnerHandle { inner: self, seat },
                            &mut *default_grab,
                        );
                        return;
                    }
                }
                f(data, &mut TouchInnerHandle { inner: self, seat }, &mut **handler);
            }
            GrabStatus::None => {
                let mut default_grab = (self.default_grab)();
                f(
                    data,
                    &mut TouchInnerHandle { inner: self, seat },
                    &mut *default_grab,
                );
            }
        }

        if let GrabStatus::Borrowed = self.grab {
            self.grab = grab;
        }
    }
}
