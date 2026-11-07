//! Common traits for input backends to receive input from.

use std::path::PathBuf;

pub use xkbcommon::xkb::Keycode;

mod tablet;

pub use tablet::{
    ProximityState, TabletToolAxisEvent, TabletToolButtonEvent, TabletToolCapabilities, TabletToolDescriptor,
    TabletToolEvent, TabletToolProximityEvent, TabletToolTipEvent, TabletToolTipState, TabletToolType,
};

#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_pointer;

use crate::utils::{Logical, Point, Raw, Size};

pub trait Device: PartialEq + Eq + std::hash::Hash {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn has_capability(&self, capability: DeviceCapability) -> bool;

    fn usb_id(&self) -> Option<(u32, u32)>;

    fn syspath(&self) -> Option<PathBuf>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DeviceCapability {
    Keyboard,
    Pointer,
    Touch,
    TabletTool,
    TabletPad,
    Gesture,
    Switch,
}

pub trait Event<B: InputBackend> {
    fn time_msec(&self) -> u32 {
        (self.time() / 1000) as u32
    }

    fn time(&self) -> u64;

    fn device(&self) -> B::Device;
}

#[derive(Debug)]
pub enum UnusedEvent {}

impl<B: InputBackend> Event<B> for UnusedEvent {
    fn time(&self) -> u64 {
        match *self {}
    }

    fn device(&self) -> B::Device {
        match *self {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyState {
    Released,
    Pressed,
}

pub trait KeyboardKeyEvent<B: InputBackend>: Event<B> {
    fn key_code(&self) -> Keycode;

    fn state(&self) -> KeyState;

    fn count(&self) -> u32;
}

impl<B: InputBackend> KeyboardKeyEvent<B> for UnusedEvent {
    fn key_code(&self) -> Keycode {
        match *self {}
    }

    fn state(&self) -> KeyState {
        match *self {}
    }

    fn count(&self) -> u32 {
        match *self {}
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ButtonState {
    Released,
    Pressed,
}

pub trait PointerButtonEvent<B: InputBackend>: Event<B> {
    fn button(&self) -> Option<MouseButton> {
        const BTN_LEFT: u32 = 0x110;
        const BTN_RIGHT: u32 = 0x111;
        const BTN_MIDDLE: u32 = 0x112;
        const BTN_SIDE: u32 = 0x113;
        const BTN_EXTRA: u32 = 0x114;
        const BTN_FORWARD: u32 = 0x115;
        const BTN_BACK: u32 = 0x116;

        match self.button_code() {
            BTN_LEFT => Some(MouseButton::Left),
            BTN_RIGHT => Some(MouseButton::Right),
            BTN_MIDDLE => Some(MouseButton::Middle),
            BTN_BACK | BTN_SIDE => Some(MouseButton::Back),
            BTN_FORWARD | BTN_EXTRA => Some(MouseButton::Forward),
            _ => None,
        }
    }

    fn button_code(&self) -> u32;

    fn state(&self) -> ButtonState;
}

impl<B: InputBackend> PointerButtonEvent<B> for UnusedEvent {
    fn button_code(&self) -> u32 {
        match *self {}
    }

    fn state(&self) -> ButtonState {
        match *self {}
    }
}
pub trait GestureBeginEvent<B: InputBackend>: Event<B> {
    fn fingers(&self) -> u32;
}

impl<B: InputBackend> GestureBeginEvent<B> for UnusedEvent {
    fn fingers(&self) -> u32 {
        match *self {}
    }
}

pub trait GestureEndEvent<B: InputBackend>: Event<B> {
    fn cancelled(&self) -> bool;
}

impl<B: InputBackend> GestureEndEvent<B> for UnusedEvent {
    fn cancelled(&self) -> bool {
        match *self {}
    }
}

pub trait GestureSwipeBeginEvent<B: InputBackend>: GestureBeginEvent<B> {}

impl<B: InputBackend> GestureSwipeBeginEvent<B> for UnusedEvent {}

pub trait GestureSwipeUpdateEvent<B: InputBackend>: Event<B> {
    fn delta_x(&self) -> f64;

    fn delta_y(&self) -> f64;

    fn delta(&self) -> Point<f64, Logical> {
        (self.delta_x(), self.delta_y()).into()
    }
}

impl<B: InputBackend> GestureSwipeUpdateEvent<B> for UnusedEvent {
    fn delta_x(&self) -> f64 {
        match *self {}
    }

    fn delta_y(&self) -> f64 {
        match *self {}
    }
}

pub trait GestureSwipeEndEvent<B: InputBackend>: GestureEndEvent<B> {}

impl<B: InputBackend> GestureSwipeEndEvent<B> for UnusedEvent {}

pub trait GesturePinchBeginEvent<B: InputBackend>: GestureBeginEvent<B> {}

impl<B: InputBackend> GesturePinchBeginEvent<B> for UnusedEvent {}

pub trait GesturePinchUpdateEvent<B: InputBackend>: Event<B> {
    fn delta_x(&self) -> f64;

    fn delta_y(&self) -> f64;

    fn scale(&self) -> f64;

    fn rotation(&self) -> f64;

    fn delta(&self) -> Point<f64, Logical> {
        (self.delta_x(), self.delta_y()).into()
    }
}

impl<B: InputBackend> GesturePinchUpdateEvent<B> for UnusedEvent {
    fn delta_x(&self) -> f64 {
        match *self {}
    }

    fn delta_y(&self) -> f64 {
        match *self {}
    }

    fn scale(&self) -> f64 {
        match *self {}
    }

    fn rotation(&self) -> f64 {
        match *self {}
    }
}

pub trait GesturePinchEndEvent<B: InputBackend>: GestureEndEvent<B> {}

impl<B: InputBackend> GesturePinchEndEvent<B> for UnusedEvent {}

pub trait GestureHoldBeginEvent<B: InputBackend>: GestureBeginEvent<B> {}

impl<B: InputBackend> GestureHoldBeginEvent<B> for UnusedEvent {}

pub trait GestureHoldEndEvent<B: InputBackend>: GestureEndEvent<B> {}

impl<B: InputBackend> GestureHoldEndEvent<B> for UnusedEvent {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Axis {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxisSource {
    Finger,
    Continuous,
    Wheel,
    WheelTilt,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxisRelativeDirection {
    Identical,
    Inverted,
}

#[cfg(feature = "wayland_frontend")]
impl From<AxisRelativeDirection> for wl_pointer::AxisRelativeDirection {
    #[inline]
    fn from(direction: AxisRelativeDirection) -> Self {
        match direction {
            AxisRelativeDirection::Identical => wl_pointer::AxisRelativeDirection::Identical,
            AxisRelativeDirection::Inverted => wl_pointer::AxisRelativeDirection::Inverted,
        }
    }
}

pub trait PointerAxisEvent<B: InputBackend>: Event<B> {
    fn amount(&self, axis: Axis) -> Option<f64>;

    fn amount_v120(&self, axis: Axis) -> Option<f64>;

    fn source(&self) -> AxisSource;

    fn relative_direction(&self, axis: Axis) -> AxisRelativeDirection;
}

impl<B: InputBackend> PointerAxisEvent<B> for UnusedEvent {
    fn amount(&self, _axis: Axis) -> Option<f64> {
        match *self {}
    }

    fn amount_v120(&self, _axis: Axis) -> Option<f64> {
        match *self {}
    }

    fn source(&self) -> AxisSource {
        match *self {}
    }

    fn relative_direction(&self, _axis: Axis) -> AxisRelativeDirection {
        match *self {}
    }
}

pub trait PointerMotionEvent<B: InputBackend>: Event<B> {
    fn delta(&self) -> Point<f64, Logical> {
        (self.delta_x(), self.delta_y()).into()
    }

    fn delta_unaccel(&self) -> Point<f64, Logical> {
        (self.delta_x_unaccel(), self.delta_y_unaccel()).into()
    }

    fn delta_x(&self) -> f64;

    fn delta_y(&self) -> f64;

    fn delta_x_unaccel(&self) -> f64;

    fn delta_y_unaccel(&self) -> f64;
}

impl<B: InputBackend> PointerMotionEvent<B> for UnusedEvent {
    fn delta_x(&self) -> f64 {
        match *self {}
    }

    fn delta_y(&self) -> f64 {
        match *self {}
    }

    fn delta_x_unaccel(&self) -> f64 {
        match *self {}
    }

    fn delta_y_unaccel(&self) -> f64 {
        match *self {}
    }
}

pub trait PointerMotionAbsoluteEvent<B: InputBackend>: AbsolutePositionEvent<B> {}
impl<B: InputBackend> PointerMotionAbsoluteEvent<B> for UnusedEvent {}

pub trait AbsolutePositionEvent<B: InputBackend>: Event<B> {
    fn position(&self) -> Point<f64, Raw> {
        (self.x(), self.y()).into()
    }

    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn position_transformed(&self, coordinate_space: Size<i32, Logical>) -> Point<f64, Logical> {
        (
            self.x_transformed(coordinate_space.w),
            self.y_transformed(coordinate_space.h),
        )
            .into()
    }

    fn x_transformed(&self, width: i32) -> f64;

    fn y_transformed(&self, height: i32) -> f64;
}

impl<B: InputBackend> AbsolutePositionEvent<B> for UnusedEvent {
    fn x(&self) -> f64 {
        match *self {}
    }

    fn y(&self) -> f64 {
        match *self {}
    }

    fn x_transformed(&self, _width: i32) -> f64 {
        match *self {}
    }

    fn y_transformed(&self, _height: i32) -> f64 {
        match *self {}
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchSlot {
    id: Option<u32>,
}

impl From<Option<u32>> for TouchSlot {
    #[inline]
    fn from(id: Option<u32>) -> Self {
        Self { id }
    }
}

impl From<TouchSlot> for i32 {
    #[inline]
    fn from(slot: TouchSlot) -> i32 {
        slot.id.map(|id| id as i32).unwrap_or(-1)
    }
}

pub trait TouchEvent<B: InputBackend>: Event<B> {
    fn slot(&self) -> TouchSlot;
}

impl<B: InputBackend> TouchEvent<B> for UnusedEvent {
    #[inline]
    fn slot(&self) -> TouchSlot {
        match *self {}
    }
}

pub trait TouchDownEvent<B: InputBackend>: TouchEvent<B> + AbsolutePositionEvent<B> {}
impl<B: InputBackend> TouchDownEvent<B> for UnusedEvent {}

pub trait TouchMotionEvent<B: InputBackend>: TouchEvent<B> + AbsolutePositionEvent<B> {}
impl<B: InputBackend> TouchMotionEvent<B> for UnusedEvent {}

pub trait TouchUpEvent<B: InputBackend>: TouchEvent<B> {}
impl<B: InputBackend> TouchUpEvent<B> for UnusedEvent {}

pub trait TouchCancelEvent<B: InputBackend>: TouchEvent<B> {}
impl<B: InputBackend> TouchCancelEvent<B> for UnusedEvent {}

pub trait TouchFrameEvent<B: InputBackend>: Event<B> {}
impl<B: InputBackend> TouchFrameEvent<B> for UnusedEvent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Switch {
    Lid,
    TabletMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Off,
    On,
}

pub trait SwitchToggleEvent<B: InputBackend>: Event<B> {
    fn switch(&self) -> Option<Switch>;
    fn state(&self) -> SwitchState;
}
impl<B: InputBackend> SwitchToggleEvent<B> for UnusedEvent {
    fn switch(&self) -> Option<Switch> {
        match *self {}
    }
    fn state(&self) -> SwitchState {
        match *self {}
    }
}

pub trait InputBackend: Sized {
    type Device: Device;
    type KeyboardKeyEvent: KeyboardKeyEvent<Self>;
    type PointerAxisEvent: PointerAxisEvent<Self>;
    type PointerButtonEvent: PointerButtonEvent<Self>;
    type PointerMotionEvent: PointerMotionEvent<Self>;
    type PointerMotionAbsoluteEvent: PointerMotionAbsoluteEvent<Self>;
    type GestureSwipeBeginEvent: GestureSwipeBeginEvent<Self>;
    type GestureSwipeUpdateEvent: GestureSwipeUpdateEvent<Self>;
    type GestureSwipeEndEvent: GestureSwipeEndEvent<Self>;
    type GesturePinchBeginEvent: GesturePinchBeginEvent<Self>;
    type GesturePinchUpdateEvent: GesturePinchUpdateEvent<Self>;
    type GesturePinchEndEvent: GesturePinchEndEvent<Self>;
    type GestureHoldBeginEvent: GestureHoldBeginEvent<Self>;
    type GestureHoldEndEvent: GestureHoldEndEvent<Self>;
    type TouchDownEvent: TouchDownEvent<Self>;
    type TouchUpEvent: TouchUpEvent<Self>;
    type TouchMotionEvent: TouchMotionEvent<Self>;
    type TouchCancelEvent: TouchCancelEvent<Self>;
    type TouchFrameEvent: TouchFrameEvent<Self>;
    type TabletToolAxisEvent: TabletToolAxisEvent<Self>;
    type TabletToolProximityEvent: TabletToolProximityEvent<Self>;
    type TabletToolTipEvent: TabletToolTipEvent<Self>;
    type TabletToolButtonEvent: TabletToolButtonEvent<Self>;
    type SwitchToggleEvent: SwitchToggleEvent<Self>;

    type SpecialEvent;
}

#[derive(Debug)]
pub enum InputEvent<B: InputBackend> {
    DeviceAdded {
        device: B::Device,
    },
    DeviceRemoved {
        device: B::Device,
    },
    Keyboard {
        event: B::KeyboardKeyEvent,
    },
    PointerMotion {
        event: B::PointerMotionEvent,
    },
    PointerMotionAbsolute {
        event: B::PointerMotionAbsoluteEvent,
    },
    PointerButton {
        event: B::PointerButtonEvent,
    },
    PointerAxis {
        event: B::PointerAxisEvent,
    },
    GestureSwipeBegin {
        event: B::GestureSwipeBeginEvent,
    },
    GestureSwipeUpdate {
        event: B::GestureSwipeUpdateEvent,
    },
    GestureSwipeEnd {
        event: B::GestureSwipeEndEvent,
    },
    GesturePinchBegin {
        event: B::GesturePinchBeginEvent,
    },
    GesturePinchUpdate {
        event: B::GesturePinchUpdateEvent,
    },
    GesturePinchEnd {
        event: B::GesturePinchEndEvent,
    },
    GestureHoldBegin {
        event: B::GestureHoldBeginEvent,
    },
    GestureHoldEnd {
        event: B::GestureHoldEndEvent,
    },
    TouchDown {
        event: B::TouchDownEvent,
    },
    TouchMotion {
        event: B::TouchMotionEvent,
    },
    TouchUp {
        event: B::TouchUpEvent,
    },
    TouchCancel {
        event: B::TouchCancelEvent,
    },
    TouchFrame {
        event: B::TouchFrameEvent,
    },

    TabletToolAxis {
        event: B::TabletToolAxisEvent,
    },

    TabletToolProximity {
        event: B::TabletToolProximityEvent,
    },

    TabletToolTip {
        event: B::TabletToolTipEvent,
    },

    TabletToolButton {
        event: B::TabletToolButtonEvent,
    },

    SwitchToggle {
        event: B::SwitchToggleEvent,
    },

    Special(B::SpecialEvent),
}

#[cfg(any(feature = "backend_winit", feature = "backend_x11"))]
pub(crate) fn xorg_mouse_to_libinput(xorg: u32) -> u32 {
    match xorg {
        0 => 0,
        1 => 0x110,
        2 => 0x112,
        3 => 0x111,
        _ => xorg - 8 + 0x113,
    }
}
