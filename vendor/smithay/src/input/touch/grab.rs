use std::fmt;

use downcast_rs::{impl_downcast, Downcast};

use crate::{
    backend::input::TouchSlot,
    input::SeatHandler,
    utils::{Logical, Point, Serial},
};

use super::{DownEvent, MotionEvent, OrientationEvent, ShapeEvent, TouchInnerHandle, UpEvent};

pub trait TouchGrab<D: SeatHandler>: Send + Downcast {
    fn down(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
        seq: Serial,
    );
    fn up(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &UpEvent, seq: Serial);

    fn motion(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
        seq: Serial,
    );

    fn frame(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial);

    fn cancel(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial);

    fn shape(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &ShapeEvent, seq: Serial);

    fn orientation(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        event: &OrientationEvent,
        seq: Serial,
    );

    fn start_data(&self) -> &GrabStartData<D>;

    fn unset(&mut self, data: &mut D);
}

impl_downcast!(TouchGrab<D> where D: SeatHandler);

pub struct GrabStartData<D: SeatHandler> {
    pub focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
    pub slot: TouchSlot,
    pub location: Point<f64, Logical>,
}

impl<D: SeatHandler + 'static> fmt::Debug for GrabStartData<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GrabStartData")
            .field("focus", &self.focus.as_ref().map(|_| "..."))
            .field("slot", &self.slot)
            .field("location", &self.location)
            .finish()
    }
}

impl<D: SeatHandler + 'static> Clone for GrabStartData<D> {
    fn clone(&self) -> Self {
        GrabStartData {
            focus: self.focus.clone(),
            slot: self.slot,
            location: self.location,
        }
    }
}

#[derive(Debug)]
pub struct DefaultGrab;

impl<D: SeatHandler + 'static> TouchGrab<D> for DefaultGrab {
    fn down(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
        seq: Serial,
    ) {
        handle.down(data, focus.clone(), event, seq);
        handle.set_grab(
            self,
            data,
            event.serial,
            TouchDownGrab {
                start_data: GrabStartData {
                    focus,
                    slot: event.slot,
                    location: event.location,
                },
                touch_points: 1,
            },
        );
    }

    fn up(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &UpEvent, seq: Serial) {
        handle.up(data, event, seq)
    }

    fn motion(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
        seq: Serial,
    ) {
        handle.motion(data, focus, event, seq)
    }

    fn frame(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial) {
        handle.frame(data, seq)
    }

    fn cancel(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial) {
        handle.cancel(data, seq)
    }

    fn shape(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &ShapeEvent, seq: Serial) {
        handle.shape(data, event, seq)
    }

    fn orientation(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        event: &OrientationEvent,
        seq: Serial,
    ) {
        handle.orientation(data, event, seq)
    }

    fn start_data(&self) -> &GrabStartData<D> {
        unreachable!()
    }

    fn unset(&mut self, _data: &mut D) {}
}

pub struct TouchDownGrab<D: SeatHandler> {
    pub start_data: GrabStartData<D>,
    pub touch_points: usize,
}

impl<D: SeatHandler + 'static> fmt::Debug for TouchDownGrab<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TouchDownGrab")
            .field("start_data", &self.start_data)
            .field("touch_points", &self.touch_points)
            .finish()
    }
}

impl<D: SeatHandler + 'static> TouchGrab<D> for TouchDownGrab<D> {
    fn down(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        _focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &DownEvent,
        seq: Serial,
    ) {
        handle.down(data, self.start_data.focus.clone(), event, seq);
        self.touch_points += 1;
    }

    fn up(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &UpEvent, seq: Serial) {
        handle.up(data, event, seq);
        self.touch_points = self.touch_points.saturating_sub(1);
        if self.touch_points == 0 {
            handle.unset_grab(self, data);
        }
    }

    fn motion(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        _focus: Option<(<D as SeatHandler>::TouchFocus, Point<f64, Logical>)>,
        event: &MotionEvent,
        seq: Serial,
    ) {
        handle.motion(data, self.start_data.focus.clone(), event, seq)
    }

    fn frame(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial) {
        handle.frame(data, seq)
    }

    fn cancel(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, seq: Serial) {
        handle.cancel(data, seq);
        handle.unset_grab(self, data);
    }

    fn shape(&mut self, data: &mut D, handle: &mut TouchInnerHandle<'_, D>, event: &ShapeEvent, seq: Serial) {
        handle.shape(data, event, seq)
    }

    fn orientation(
        &mut self,
        data: &mut D,
        handle: &mut TouchInnerHandle<'_, D>,
        event: &OrientationEvent,
        seq: Serial,
    ) {
        handle.orientation(data, event, seq)
    }

    fn start_data(&self) -> &GrabStartData<D> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut D) {}
}
