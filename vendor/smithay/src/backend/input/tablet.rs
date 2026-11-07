use super::{AbsolutePositionEvent, ButtonState, Event, InputBackend, UnusedEvent};
use crate::utils::{Logical, Point};
use bitflags::bitflags;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TabletToolDescriptor {
    pub tool_type: TabletToolType,
    pub hardware_serial: u64,
    pub hardware_id_wacom: u64,
    pub capabilities: TabletToolCapabilities,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum TabletToolType {
    Pen,
    Eraser,
    Brush,
    Pencil,
    Airbrush,
    Mouse,
    Lens,
    Totem,
    Unknown,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TabletToolCapabilities: u32 {
        const TILT = 1;
        const PRESSURE = 2;
        const DISTANCE = 4;
        const ROTATION = 16;
        const SLIDER = 32;
        const WHEEL = 64;
    }
}

pub trait TabletToolEvent<B: InputBackend>: AbsolutePositionEvent<B> {
    fn tool(&self) -> TabletToolDescriptor;

    fn delta(&self) -> Point<f64, Logical> {
        (self.delta_x(), self.delta_y()).into()
    }

    fn tilt(&self) -> (f64, f64) {
        (self.tilt_x(), self.tilt_y())
    }

    fn tilt_has_changed(&self) -> bool {
        self.tilt_x_has_changed() || self.tilt_y_has_changed()
    }

    fn delta_x(&self) -> f64;
    fn delta_y(&self) -> f64;

    fn distance(&self) -> f64;

    fn distance_has_changed(&self) -> bool;

    fn pressure(&self) -> f64;

    fn pressure_has_changed(&self) -> bool;

    fn slider_position(&self) -> f64;

    fn slider_has_changed(&self) -> bool;

    fn tilt_x(&self) -> f64;

    fn tilt_x_has_changed(&self) -> bool;

    fn tilt_y(&self) -> f64;

    fn tilt_y_has_changed(&self) -> bool;

    fn rotation(&self) -> f64;

    fn rotation_has_changed(&self) -> bool;

    fn wheel_delta(&self) -> f64;
    fn wheel_delta_discrete(&self) -> i32;
    fn wheel_has_changed(&self) -> bool;
}

impl<B: InputBackend> TabletToolEvent<B> for UnusedEvent {
    fn tool(&self) -> TabletToolDescriptor {
        match *self {}
    }
    fn delta_x(&self) -> f64 {
        match *self {}
    }
    fn delta_y(&self) -> f64 {
        match *self {}
    }
    fn distance(&self) -> f64 {
        match *self {}
    }
    fn distance_has_changed(&self) -> bool {
        match *self {}
    }
    fn pressure(&self) -> f64 {
        match *self {}
    }
    fn pressure_has_changed(&self) -> bool {
        match *self {}
    }
    fn slider_position(&self) -> f64 {
        match *self {}
    }
    fn slider_has_changed(&self) -> bool {
        match *self {}
    }
    fn tilt_x(&self) -> f64 {
        match *self {}
    }
    fn tilt_x_has_changed(&self) -> bool {
        match *self {}
    }
    fn tilt_y(&self) -> f64 {
        match *self {}
    }
    fn tilt_y_has_changed(&self) -> bool {
        match *self {}
    }
    fn rotation(&self) -> f64 {
        match *self {}
    }
    fn rotation_has_changed(&self) -> bool {
        match *self {}
    }
    fn wheel_delta(&self) -> f64 {
        match *self {}
    }
    fn wheel_delta_discrete(&self) -> i32 {
        match *self {}
    }
    fn wheel_has_changed(&self) -> bool {
        match *self {}
    }
}

pub trait TabletToolAxisEvent<B: InputBackend>: TabletToolEvent<B> + Event<B> {}

impl<B: InputBackend> TabletToolAxisEvent<B> for UnusedEvent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProximityState {
    Out,
    In,
}

pub trait TabletToolProximityEvent<B: InputBackend>: TabletToolEvent<B> + Event<B> {
    fn state(&self) -> ProximityState;
}

impl<B: InputBackend> TabletToolProximityEvent<B> for UnusedEvent {
    fn state(&self) -> ProximityState {
        match *self {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TabletToolTipState {
    Up,
    Down,
}

pub trait TabletToolTipEvent<B: InputBackend>: TabletToolEvent<B> + Event<B> {
    fn tip_state(&self) -> TabletToolTipState;
}

impl<B: InputBackend> TabletToolTipEvent<B> for UnusedEvent {
    fn tip_state(&self) -> TabletToolTipState {
        match *self {}
    }
}

pub trait TabletToolButtonEvent<B: InputBackend>: TabletToolEvent<B> + Event<B> {
    fn button(&self) -> u32;

    fn seat_button_count(&self) -> u32;

    fn button_state(&self) -> ButtonState;
}

impl<B: InputBackend> TabletToolButtonEvent<B> for UnusedEvent {
    fn button(&self) -> u32 {
        match *self {}
    }

    fn seat_button_count(&self) -> u32 {
        match *self {}
    }

    fn button_state(&self) -> ButtonState {
        match *self {}
    }
}
