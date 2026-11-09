// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::backend::input::{
    AbsolutePositionEvent, Axis, AxisRelativeDirection, AxisSource, ButtonState, Device, DeviceCapability, Event,
    InputBackend, KeyState, KeyboardKeyEvent, Keycode, PointerAxisEvent, PointerButtonEvent, PointerMotionAbsoluteEvent,
    PointerMotionEvent, UnusedEvent,
};
use smithay::reexports::wayland_server::Client;

use crate::protocols::virtual_pointer::PointerAction;
use crate::state::ClientState;

#[derive(Debug)]
pub struct Virtual;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VirtualDevice;

impl Device for VirtualDevice {
    fn id(&self) -> String {
        "virtual".into()
    }
    fn name(&self) -> String {
        "A made-up mouse and keyboard".into()
    }
    fn has_capability(&self, capability: DeviceCapability) -> bool {
        matches!(capability, DeviceCapability::Keyboard | DeviceCapability::Pointer)
    }
    fn usb_id(&self) -> Option<(u32, u32)> {
        None
    }
    fn syspath(&self) -> Option<std::path::PathBuf> {
        None
    }
}

fn micros(ms: u32) -> u64 {
    u64::from(ms) * 1000
}

#[derive(Debug)]
pub struct Key {
    pub time: u32,

    pub code: u32,
    pub pressed: bool,
}

#[derive(Debug)]
pub struct Motion {
    pub time: u32,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug)]
pub struct Absolute {
    pub time: u32,

    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Button {
    pub time: u32,
    pub button: u32,
    pub pressed: bool,
}

#[derive(Debug)]
pub struct Scroll {
    pub time: u32,
    pub source: AxisSource,
    pub amount: [f64; 2],
    pub v120: [Option<f64>; 2],
    pub stopped: [bool; 2],
}

macro_rules! timed {
    ($($t:ty),*) => {$(
        impl Event<Virtual> for $t {
            fn time(&self) -> u64 {
                micros(self.time)
            }
            fn device(&self) -> VirtualDevice {
                VirtualDevice
            }
        }
    )*};
}
timed!(Key, Motion, Absolute, Button, Scroll);

impl KeyboardKeyEvent<Virtual> for Key {
    fn key_code(&self) -> Keycode {

        Keycode::new(self.code + 8)
    }
    fn state(&self) -> KeyState {
        if self.pressed { KeyState::Pressed } else { KeyState::Released }
    }
    fn count(&self) -> u32 {
        u32::from(self.pressed)
    }
}

impl PointerMotionEvent<Virtual> for Motion {
    fn delta_x(&self) -> f64 {
        self.dx
    }
    fn delta_y(&self) -> f64 {
        self.dy
    }
    fn delta_x_unaccel(&self) -> f64 {
        self.dx
    }
    fn delta_y_unaccel(&self) -> f64 {
        self.dy
    }
}

impl AbsolutePositionEvent<Virtual> for Absolute {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn x_transformed(&self, width: i32) -> f64 {
        self.x * f64::from(width)
    }
    fn y_transformed(&self, height: i32) -> f64 {
        self.y * f64::from(height)
    }
}
impl PointerMotionAbsoluteEvent<Virtual> for Absolute {}

impl PointerButtonEvent<Virtual> for Button {
    fn button_code(&self) -> u32 {
        self.button
    }
    fn state(&self) -> ButtonState {
        if self.pressed { ButtonState::Pressed } else { ButtonState::Released }
    }
}

impl PointerAxisEvent<Virtual> for Scroll {
    fn amount(&self, axis: Axis) -> Option<f64> {
        let i = axis_index(axis);
        (self.amount[i] != 0.0 || self.stopped[i]).then_some(self.amount[i])
    }
    fn amount_v120(&self, axis: Axis) -> Option<f64> {
        self.v120[axis_index(axis)]
    }
    fn source(&self) -> AxisSource {
        self.source
    }
    fn relative_direction(&self, _axis: Axis) -> AxisRelativeDirection {
        AxisRelativeDirection::Identical
    }
}

fn axis_index(axis: Axis) -> usize {
    match axis {
        Axis::Horizontal => 0,
        Axis::Vertical => 1,
    }
}

impl InputBackend for Virtual {
    type Device = VirtualDevice;
    type KeyboardKeyEvent = Key;
    type PointerAxisEvent = Scroll;
    type PointerButtonEvent = Button;
    type PointerMotionEvent = Motion;
    type PointerMotionAbsoluteEvent = Absolute;
    type GestureSwipeBeginEvent = UnusedEvent;
    type GestureSwipeUpdateEvent = UnusedEvent;
    type GestureSwipeEndEvent = UnusedEvent;
    type GesturePinchBeginEvent = UnusedEvent;
    type GesturePinchUpdateEvent = UnusedEvent;
    type GesturePinchEndEvent = UnusedEvent;
    type GestureHoldBeginEvent = UnusedEvent;
    type GestureHoldEndEvent = UnusedEvent;
    type TouchDownEvent = UnusedEvent;
    type TouchUpEvent = UnusedEvent;
    type TouchMotionEvent = UnusedEvent;
    type TouchCancelEvent = UnusedEvent;
    type TouchFrameEvent = UnusedEvent;
    type TabletToolAxisEvent = UnusedEvent;
    type TabletToolProximityEvent = UnusedEvent;
    type TabletToolTipEvent = UnusedEvent;
    type TabletToolButtonEvent = UnusedEvent;
    type SwitchToggleEvent = UnusedEvent;
    type SpecialEvent = ();
}

pub enum Did {
    Key(Key),
    Motion(Motion),
    Absolute(Absolute),
    Button(Button),
    Scroll(Scroll),
}

impl Did {

    pub fn is_press(&self) -> bool {
        match self {
            Did::Key(k) => k.pressed,
            Did::Button(b) => b.pressed,
            _ => false,
        }
    }
}

impl Did {

    pub fn taken(&self) -> crate::capture::Taken {
        use crate::capture::Taken;
        match self {
            Did::Key(k) => Taken::Key { time: k.time, code: k.code, pressed: k.pressed },
            Did::Motion(m) => Taken::Motion { time: m.time, dx: m.dx, dy: m.dy },
            Did::Absolute(_) => Taken::Other,
            Did::Button(b) => Taken::Button { time: b.time, button: b.button, pressed: b.pressed },

            Did::Scroll(s) => Taken::Scroll {
                time: s.time,
                amount: [s.amount(Axis::Vertical), s.amount(Axis::Horizontal)],
                v120: [s.v120[1], s.v120[0]],
            },
        }
    }
}

impl From<PointerAction> for Did {
    fn from(action: PointerAction) -> Did {
        match action {
            PointerAction::Motion { time, dx, dy } => Did::Motion(Motion { time, dx, dy }),
            PointerAction::MotionAbsolute { time, x, y } => Did::Absolute(Absolute { time, x, y }),
            PointerAction::Button { time, button, pressed } => Did::Button(Button { time, button, pressed }),
            PointerAction::Axis { time, source, amount, v120, stopped } => Did::Scroll(Scroll { time, source, amount, v120, stopped }),
        }
    }
}

pub fn from_box(client: &Client) -> bool {
    client.get_data::<ClientState>().is_some_and(|s| s.security_context.is_some())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Maker {
    pub from_box: bool,

    pub plugin: Option<String>,
}

pub fn maker(client: &Client) -> Maker {
    Maker { from_box: from_box(client), plugin: may_control(client).then(|| app_id_of(client)).flatten() }
}

fn abilities(client: &Client) -> (bool, bool) {
    client.get_data::<ClientState>()
        .and_then(|s| s.security_context.as_ref())
        .and_then(|c| c.sandbox_engine.as_deref())
        .map_or((false, false), crate::sandbox::abilities)
}

pub fn app_id_of(client: &Client) -> Option<String> {
    client.get_data::<ClientState>()
        .and_then(|s| s.security_context.as_ref())
        .and_then(|c| c.app_id.clone())
}

pub fn may_control(client: &Client) -> bool {
    abilities(client).0
}

pub fn may_capture(client: &Client) -> bool {
    abilities(client).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_and_scrolls_read_as_a_real_device_would() {
        let k = Key { time: 5, code: 30, pressed: true };
        assert_eq!(k.key_code(), Keycode::new(38));
        assert_eq!(Event::<Virtual>::time_msec(&k), 5);
        let s = Scroll { time: 1, source: AxisSource::Wheel, amount: [0.0, 15.0], v120: [None, Some(120.0)], stopped: [false; 2] };
        assert_eq!(s.amount(Axis::Horizontal), None);
        assert_eq!((s.amount(Axis::Vertical), s.amount_v120(Axis::Vertical)), (Some(15.0), Some(120.0)));
        assert!(Did::Button(Button { time: 0, button: 0x110, pressed: true }).is_press());
        assert!(!Did::Motion(Motion { time: 0, dx: 1.0, dy: 0.0 }).is_press());
    }
}
