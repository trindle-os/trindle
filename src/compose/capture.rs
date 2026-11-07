// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Sharing this computer's mouse and keyboard with another. A plugin allowed it names the
//! side of the screens the other computer is on (protocols::input_capture).
//! When the pointer is pushed hard against that side, and never by a click,
//! the pointer is hidden where it left, the windows are told the pointer and keyboard have gone, and every
//! movement, button, scroll and key goes to the program until it lets go.
//!
//! The mouse and keyboard always come back: when the program lets go, when
//! it or its box ends, when the screen locks, and when Ctrl, Alt and Shift
//! are held together for a second, whatever the program does.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use smithay::backend::input::{
    Axis, ButtonState, Event, InputBackend, InputEvent, KeyState, KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent,
    PointerMotionEvent,
};
use smithay::input::keyboard::KeyboardHandle;
use smithay::input::pointer::{CursorImageStatus, MotionEvent};
use smithay::reexports::calloop::{
    timer::{TimeoutAction, Timer},
    LoopHandle,
};
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::{IsAlive, Logical, Point, Rectangle, SERIAL_COUNTER};

use crate::focus::KeyboardFocusTarget;
use tracing::info;
use crate::protocols::input_capture::{capture, InputCaptureHandler, Side, TrindleInputCaptureV1};
use crate::limits::{Rate, TAKES_PER_SECOND};
use crate::screen_edge::{self, EdgePush, CORNER_GUARD, CROSS_PUSH};
use crate::state::{AnvilState, Backend};

pub const SAFETY_HOLD: Duration = Duration::from_secs(1);
const BACK_PAST_RING: f64 = 2.0;

const CTRL: [u32; 2] = [29, 97];
const ALT: [u32; 2] = [56, 100];
const SHIFT: [u32; 2] = [42, 54];

#[derive(Debug)]
pub struct Slots<C> {
    armed: Vec<(C, Side)>,
    replaced: Vec<C>,
}

impl<C> Default for Slots<C> {
    fn default() -> Slots<C> {
        Slots { armed: Vec::new(), replaced: Vec::new() }
    }
}

pub const MOST_HELD: usize = 8;

impl<C: PartialEq + Clone> Slots<C> {
    pub fn ask(&mut self, capture: C, side: Side) -> Option<C> {
        let old = self.armed.iter().position(|(_, s)| *s == side).map(|i| self.armed.remove(i).0);
        if let Some(old) = &old {
            self.replaced.push(old.clone());
        }
        self.armed.push((capture, side));
        old
    }

    pub fn at(&self, side: Side) -> Option<&C> {
        self.armed.iter().find(|(_, s)| *s == side).map(|(c, _)| c)
    }

    pub fn sides(&self, keep: impl Fn(&C) -> bool) -> Vec<Side> {
        self.armed.iter().filter(|(c, _)| keep(c)).map(|(_, s)| *s).collect()
    }

    pub fn is_replaced(&self, capture: &C) -> bool {
        self.replaced.contains(capture)
    }

    pub fn forget(&mut self, capture: &C) {
        self.armed.retain(|(c, _)| c != capture);
        self.replaced.retain(|c| c != capture);
    }

    pub fn held(&self, keep: impl Fn(&C) -> bool) -> usize {
        self.all().filter(|c| keep(c)).count()
    }

    pub fn all(&self) -> impl Iterator<Item = &C> {
        self.armed.iter().map(|(c, _)| c).chain(&self.replaced)
    }
}

pub fn pushed(sides: &[Side], here: &[Side], tried: Point<f64, Logical>, now: Point<f64, Logical>)
    -> (Option<(Side, f64)>, f64) {
    let hardest = sides.iter().map(|s| (*s, s.beyond(tried, now).0)).filter(|(_, out)| *out > 0.0)
        .max_by(|a, b| a.1.total_cmp(&b.1));
    let measured = hardest.map(|(s, _)| s).or_else(|| sides.iter().copied().find(|s| here.contains(s)))
        .or_else(|| sides.first().copied());
    (hardest, measured.map_or(0.0, |s| s.beyond(tried, now).1))
}

#[derive(Debug, PartialEq)]
pub enum Permit {
    Yes,
    TooOften,
    NotAsked,
}

pub fn permit(takes: &mut Rate, app_id: Option<&str>, grant: &dyn Fn(&str) -> bool) -> Permit {
    if !takes.allow() {
        return Permit::TooOften;
    }
    match app_id {
        Some(id) if !grant(id) => Permit::NotAsked,
        _ => Permit::Yes,
    }
}

impl Capture {
    pub fn away(&self) -> bool {
        self.active.is_some()
    }
}

pub struct Capture {
    takes: Rate,
    slots: Slots<TrindleInputCaptureV1>,
    active: Option<Active>,
    push: EdgePush,
    manual: HashSet<ClientId>,
    swallow_keys: Vec<u32>,
    swallow_buttons: Vec<u32>,
    pub from_box: bool,
}

impl Default for Capture {
    fn default() -> Capture {
        Capture {
            takes: Rate::new(TAKES_PER_SECOND, Duration::from_secs(1)),
            slots: Slots::default(),
            active: None,
            push: EdgePush::needing(CROSS_PUSH),
            manual: HashSet::new(),
            swallow_keys: Vec::new(),
            swallow_buttons: Vec::new(),
            from_box: false,
        }
    }
}

impl std::fmt::Debug for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capture")
            .field("armed", &self.slots.sides(|_| true))
            .field("active", &self.active.is_some())
            .finish()
    }
}

impl Capture {
    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    fn is_manual(&self, capture: &TrindleInputCaptureV1) -> bool {
        client_of(capture).is_some_and(|id| self.manual.contains(&id))
    }
}

pub const LOG: &str = "trindle_compose::capture";

fn kind_of(focus: Option<&KeyboardFocusTarget>) -> &'static str {
    match focus {
        None => "nothing",
        Some(KeyboardFocusTarget::Window(_)) => "a window",
        Some(KeyboardFocusTarget::LayerSurface(_)) => "a panel",
        Some(_) => "something else",
    }
}

fn client_of(capture: &TrindleInputCaptureV1) -> Option<ClientId> {
    capture.client().map(|c| c.id())
}

fn same_program(a: &TrindleInputCaptureV1, b: &TrindleInputCaptureV1) -> bool {
    a == b || client_of(a).is_some_and(|x| client_of(b) == Some(x))
}

struct Active {
    capture: TrindleInputCaptureV1,
    side: Side,
    left_at: Point<f64, Logical>,
    screen: Rectangle<i32, Logical>,
    focus: Option<KeyboardFocusTarget>,
    keys: Vec<u32>,
    buttons: Vec<u32>,
    safety_since: Option<Instant>,
    pre_held: Vec<u32>,
}

pub fn along_of(screen: Rectangle<i32, Logical>, at: Point<f64, Logical>, side: Side) -> f64 {
    let (from, length, pos) = match side {
        Side::Left | Side::Right => (screen.loc.y, screen.size.h, at.y),
        Side::Top | Side::Bottom => (screen.loc.x, screen.size.w, at.x),
    };
    ((pos - f64::from(from)) / f64::from((length - 1).max(1))).clamp(0.0, 1.0)
}

pub fn back_at(screen: Rectangle<i32, Logical>, side: Side, along: f64, inside: f64) -> Point<f64, Logical> {
    let (x0, y0) = (f64::from(screen.loc.x), f64::from(screen.loc.y));
    let (w, h) = (f64::from(screen.size.w), f64::from(screen.size.h));
    let along = along.clamp(0.0, 1.0);
    match side {
        Side::Left => (x0 + inside, y0 + along * (h - 1.0)),
        Side::Right => (x0 + w - 1.0 - inside, y0 + along * (h - 1.0)),
        Side::Top => (x0 + along * (w - 1.0), y0 + inside),
        Side::Bottom => (x0 + along * (w - 1.0), y0 + h - 1.0 - inside),
    }
    .into()
}

pub fn safety_held(keys: &[u32]) -> bool {
    [CTRL, ALT, SHIFT].iter().all(|pair| pair.iter().any(|k| keys.contains(k)))
}

fn held(list: &mut Vec<u32>, code: u32, down: bool) {
    list.retain(|c| *c != code);
    if down {
        list.push(code);
    }
}

pub enum Taken {
    Key { time: u32, code: u32, pressed: bool },
    Motion { time: u32, dx: f64, dy: f64 },
    Button { time: u32, button: u32, pressed: bool },
    Scroll { time: u32, amount: [Option<f64>; 2], v120: [Option<f64>; 2] },
    Other,
}

impl Taken {
    pub fn from_event<B: InputBackend>(event: &InputEvent<B>) -> Option<Taken> {
        Some(match event {
            InputEvent::Keyboard { event } => Taken::Key {
                time: event.time_msec(),
                code: event.key_code().raw().saturating_sub(8),
                pressed: event.state() == KeyState::Pressed,
            },
            InputEvent::PointerMotion { event } => {
                let d = event.delta();
                Taken::Motion { time: event.time_msec(), dx: d.x, dy: d.y }
            }
            InputEvent::PointerButton { event } => Taken::Button {
                time: event.time_msec(),
                button: event.button_code(),
                pressed: event.state() == ButtonState::Pressed,
            },
            InputEvent::PointerAxis { event } => Taken::Scroll {
                time: event.time_msec(),
                amount: [event.amount(Axis::Vertical), event.amount(Axis::Horizontal)],
                v120: [event.amount_v120(Axis::Vertical), event.amount_v120(Axis::Horizontal)],
            },
            InputEvent::DeviceAdded { .. } | InputEvent::DeviceRemoved { .. } | InputEvent::Special(_) => return None,
            _ => Taken::Other,
        })
    }
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn cross_push(&mut self, tried: Point<f64, Logical>, now: Point<f64, Logical>, time_ms: u32) {
        if self.capture.from_box || self.capture.active.is_some() {
            return;
        }
        let sides = self.capture.slots.sides(|c| !self.capture.is_manual(c));
        if sides.is_empty() {
            return;
        }
        let screens = self.screens();
        let here = screen_edge::outer_sides(&screens, now, 1);
        let at_edge = sides.iter().any(|s| here.contains(s));
        let (beyond, along) = pushed(&sides, &here, tried, now);
        let Some(side) = self.capture.push.feed(beyond, along, at_edge, time_ms) else { return };
        if !here.contains(&side) || self.is_locked() || self.pointer.is_grabbed() || self.only_the_person_answers() {
            return;
        }
        if screen_edge::near_corner(&screens, now, CORNER_GUARD) {
            return;
        }
        self.start_capture(side, now);
    }

    fn start_capture(&mut self, side: Side, at: Point<f64, Logical>) {
        let Some(capture) = self.capture.slots.at(side).cloned() else { return };
        if !capture.is_alive() {
            self.capture.slots.forget(&capture);
            return;
        }
        self.begin_capture(capture, side, at, Vec::new());
    }

    pub fn keep_pointer_away(&mut self) {
        if self.capture.active.is_some() && !matches!(self.cursor_status, CursorImageStatus::Hidden) {
            self.cursor_status = CursorImageStatus::Hidden;
        }
    }

    fn begin_capture(&mut self, capture: TrindleInputCaptureV1, side: Side, at: Point<f64, Logical>, pre_held: Vec<u32>) {
        let screen = self.screens().into_iter().find(|s| s.to_f64().contains(at)).unwrap_or_default();
        let serial = SERIAL_COUNTER.next_serial();
        let keyboard: KeyboardHandle<Self> = self.seat.get_keyboard().unwrap();
        let focus = keyboard.current_focus();
        self.give_keyboard(None, serial);
        self.release_all_keys();
        let pointer = self.pointer.clone();
        pointer.motion(self, None, &MotionEvent { location: at, serial, time: self.clock.now().as_millis() });
        pointer.frame(self);
        self.cursor_status = CursorImageStatus::Hidden;
        capture.activated(along_of(screen, at, side));
        info!(?side, had_keyboard = %kind_of(focus.as_ref()), "the mouse and keyboard are lent to another computer");
        let who = capture.client().and_then(|c| crate::virtual_input::app_id_of(&c)).unwrap_or_else(|| trindle_toolkit::t!("compose-capture-unknown-program").into());
        crate::indicators::capture_changed(Some(who));
        self.capture.active = Some(Active {
            capture,
            side,
            left_at: at,
            screen,
            focus,
            keys: Vec::new(),
            buttons: Vec::new(),
            safety_since: None,
            pre_held,
        });
    }

    fn take_now(&mut self, capture: &TrindleInputCaptureV1, side: Side) -> bool {
        if self.capture.active.is_some() || self.is_locked() || self.pointer.is_grabbed() || self.only_the_person_answers() {
            return false;
        }
        let at = self.pointer.current_location();
        if !self.screens().iter().any(|s| s.to_f64().contains(at)) {
            return false;
        }
        let keyboard: KeyboardHandle<Self> = self.seat.get_keyboard().unwrap();
        let down: Vec<u32> = keyboard.pressed_keys().into_iter().map(|k| k.raw().saturating_sub(8)).collect();
        let on = self.capture.slots.at(side).filter(|c| same_program(c, capture)).unwrap_or(capture).clone();
        self.begin_capture(on, side, at, down);
        true
    }

    pub fn end_capture(&mut self, reason: Option<capture::DeactivatedReason>, along: Option<f64>) {
        let Some(active) = self.capture.active.take() else { return };
        crate::indicators::capture_changed(None);
        if let (Some(reason), true) = (reason, active.capture.is_alive()) {
            active.capture.deactivated(reason);
        }
        self.capture.swallow_keys.extend(active.keys);
        self.capture.swallow_buttons.extend(active.buttons);
        self.capture.push = EdgePush::needing(CROSS_PUSH);
        let inside = f64::from(self.config.gap.max(1)) + BACK_PAST_RING;
        let along = along.unwrap_or_else(|| along_of(active.screen, active.left_at, active.side));
        let place = back_at(active.screen, active.side, along, inside);
        let place = self.clamp_coords(place);
        self.cursor_status = CursorImageStatus::default_named();
        let serial = SERIAL_COUNTER.next_serial();
        let pointer = self.pointer.clone();
        let under = self.surface_under(place);
        pointer.motion(self, under, &MotionEvent { location: place, serial, time: self.clock.now().as_millis() });
        pointer.frame(self);
        if self.is_locked() {
            return;
        }
        let question = self.question_window_in_front();
        let back = active.focus.filter(|f| match f {
            KeyboardFocusTarget::Window(w) => !question && f.alive() && self.space.elements().any(|e| e.0 == *w),
            _ => false,
        });
        info!(?reason, keyboard_to = back.as_ref().map_or("the window on top", |_| "the window that had it"), keys_still_held = self.capture.swallow_keys.len(),
              "the mouse and keyboard are back");
        match back {
            Some(focus) => {
                let keyboard: KeyboardHandle<Self> = self.seat.get_keyboard().unwrap();
                keyboard.set_focus(self, Some(focus), serial);
            }
            None => self.focus_topmost(),
        }
    }

    pub fn captured(&mut self, did: Taken) -> bool {
        if self.capture.active.is_some() && self.is_locked() {
            self.end_capture(Some(capture::DeactivatedReason::Locked), None);
        }
        if self.capture.active.is_some() && self.only_the_person_answers() {
            self.end_capture(Some(capture::DeactivatedReason::Question), None);
        }
        let Some(active) = self.capture.active.as_mut() else {
            return self.capture.keep_back_letting_go(did);
        };
        let to = active.capture.clone();
        if let Taken::Key { code, pressed: false, .. } = did {
            if active.pre_held.contains(&code) {
                active.pre_held.retain(|c| *c != code);
                return true;
            }
        }
        match did {
            Taken::Key { time, code, pressed } => {
                held(&mut active.keys, code, pressed);
                to.key(time, code, u32::from(pressed));
                watch_safety_keys(active, &self.handle);
            }
            Taken::Motion { time, dx, dy } => to.motion(time, dx, dy),
            Taken::Button { time, button, pressed } => {
                held(&mut active.buttons, button, pressed);
                to.button(time, button, u32::from(pressed));
            }
            Taken::Scroll { time, amount, v120 } => {
                for axis in 0..2 {
                    if let Some(value) = amount[axis].filter(|v| *v != 0.0) {
                        to.axis(time, axis as u32, value, v120[axis].map_or(0, |v| v.round() as i32));
                    }
                }
            }
            Taken::Other => {}
        }
        true
    }

    fn safety_check(&mut self) {
        let due = self.capture.active.as_ref().and_then(|a| a.safety_since)
            .is_some_and(|t| t.elapsed() + Duration::from_millis(20) >= SAFETY_HOLD);
        if due {
            self.end_capture(Some(capture::DeactivatedReason::SafetyKeys), None);
        }
    }
}

impl Capture {
    fn keep_back_letting_go(&mut self, did: Taken) -> bool {
        match did {
            Taken::Key { code, pressed: false, .. } if self.swallow_keys.contains(&code) => {
                self.swallow_keys.retain(|c| *c != code);
                info!("the letting go of a key pressed while the keyboard was away is kept back");
                true
            }
            Taken::Button { button, pressed: false, .. } if self.swallow_buttons.contains(&button) => {
                self.swallow_buttons.retain(|b| *b != button);
                true
            }
            _ => false,
        }
    }
}

fn watch_safety_keys<BackendData: Backend + 'static>(
    active: &mut Active,
    handle: &LoopHandle<'static, AnvilState<BackendData>>,
) {
    let all = safety_held(&active.keys);
    match (all, active.safety_since) {
        (true, None) => {
            active.safety_since = Some(Instant::now());
            let _ = handle.insert_source(Timer::from_duration(SAFETY_HOLD), |_, _, state| {
                state.safety_check();
                TimeoutAction::Drop
            });
        }
        (false, _) => active.safety_since = None,
        (true, Some(_)) => {}
    }
}

impl<BackendData: Backend + 'static> InputCaptureHandler for AnvilState<BackendData> {
    fn capture_asked(&mut self, capture: TrindleInputCaptureV1, side: Side) {
        if self.capture.slots.held(|c| same_program(c, &capture)) >= MOST_HELD {
            capture.post_error(0u32, "too many captures held at once");
            return;
        }
        if let Some(old) = self.capture.slots.ask(capture.clone(), side) {
            if self.capture.active.as_ref().is_some_and(|a| a.capture == old) {
                self.end_capture(Some(capture::DeactivatedReason::Released), None);
            }
            if old.is_alive() {
                old.unavailable(capture::UnavailableReason::Taken);
            }
        }
        capture.ready();
        self.capture.push = EdgePush::needing(CROSS_PUSH);
    }

    fn capture_take(&mut self, capture: &TrindleInputCaptureV1, side: Side) {
        if self.capture.slots.is_replaced(capture) {
            capture.unavailable(capture::UnavailableReason::Taken);
            return;
        }
        let app = capture.client().and_then(|c| crate::virtual_input::app_id_of(&c));
        match permit(&mut self.capture.takes, app.as_deref(), &crate::grants::take) {
            Permit::Yes => {}
            Permit::TooOften => return capture.unavailable(capture::UnavailableReason::NotNow),
            Permit::NotAsked => return capture.unavailable(capture::UnavailableReason::NotAsked),
        }
        if !self.take_now(capture, side) {
            capture.unavailable(capture::UnavailableReason::NotNow);
        }
    }

    fn capture_mode(&mut self, capture: &TrindleInputCaptureV1, manual: bool) {
        let Some(id) = client_of(capture) else { return };
        if manual {
            self.capture.manual.insert(id);
        } else {
            self.capture.manual.remove(&id);
        }
    }

    fn capture_release(&mut self, capture: &TrindleInputCaptureV1, along: f64) {
        if self.capture.active.as_ref().is_some_and(|a| same_program(&a.capture, capture)) {
            self.end_capture(Some(capture::DeactivatedReason::Released), Some(along));
        }
    }

    fn capture_gone(&mut self, capture: &TrindleInputCaptureV1) {
        if self.capture.active.as_ref().is_some_and(|a| a.capture == *capture) {
            self.end_capture(None, None);
        }
        self.capture.slots.forget(capture);
        #[allow(clippy::mutable_key_type)]
        let live: HashSet<ClientId> = self.capture.slots.all().filter_map(client_of).collect();
        self.capture.manual.retain(|id| live.contains(id));
    }
}
crate::delegate_input_capture!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

#[cfg(test)]
mod tests {
    use super::*;

    fn screen() -> Rectangle<i32, Logical> {
        Rectangle::new((1280, 0).into(), (1920, 1080).into())
    }

    #[test]
    fn where_along_a_side_the_pointer_left_and_comes_back() {
        let s = screen();
        assert_eq!(along_of(s, (3199.4, 0.0).into(), Side::Right), 0.0);
        assert_eq!(along_of(s, (3199.4, 1079.0).into(), Side::Right), 1.0);
        assert!((along_of(s, (1280.0, 539.5).into(), Side::Left) - 0.5).abs() < 1e-9);
        assert!((along_of(s, (2239.5, 0.0).into(), Side::Top) - 0.5).abs() < 1e-9);
        assert_eq!(back_at(s, Side::Right, 0.5, 8.0), Point::from((3191.0, 539.5)));
        assert_eq!(back_at(s, Side::Left, 0.0, 8.0), Point::from((1288.0, 0.0)));
        assert_eq!(back_at(s, Side::Top, 1.0, 8.0), Point::from((3199.0, 8.0)));
        assert_eq!(back_at(s, Side::Bottom, 2.0, 8.0), Point::from((3199.0, 1071.0)));
        let at: Point<f64, Logical> = (3199.4, 300.0).into();
        let back = back_at(s, Side::Right, along_of(s, at, Side::Right), 8.0);
        assert!((back.y - at.y).abs() < 1e-9);
    }

    #[test]
    fn the_safety_hold_needs_ctrl_alt_and_shift_together() {
        assert!(safety_held(&[29, 56, 42]));
        assert!(safety_held(&[97, 100, 54, 30]));
        assert!(!safety_held(&[29, 56]));
        assert!(!safety_held(&[29, 42, 30]));
        assert!(!safety_held(&[1, 1, 1]));
        let mut keys = Vec::new();
        held(&mut keys, 29, true);
        held(&mut keys, 56, true);
        held(&mut keys, 42, true);
        assert!(safety_held(&keys));
        held(&mut keys, 56, false);
        assert!(!safety_held(&keys));
    }

    fn at(x: f64, y: f64) -> Point<f64, Logical> {
        (x, y).into()
    }

    #[test]
    fn a_plugin_in_a_box_may_take_only_when_the_person_just_ran_it() {
        let mut takes = Rate::new(2, Duration::from_secs(1));
        let marked = std::cell::RefCell::new(vec!["org.a.Alpha".to_string()]);
        let grant = |id: &str| {
            let mut m = marked.borrow_mut();
            let had = m.iter().any(|x| x == id);
            m.retain(|x| x != id);
            had
        };
        assert_eq!(permit(&mut takes, Some("org.b.Beta"), &grant), Permit::NotAsked);
        assert_eq!(permit(&mut takes, Some("org.a.Alpha"), &grant), Permit::Yes);
        let mut takes = Rate::new(2, Duration::from_secs(1));
        marked.borrow_mut().extend(["org.a.Alpha".to_string(), "org.a.Alpha".to_string()]);
        assert_eq!(permit(&mut takes, Some("org.a.Alpha"), &grant), Permit::Yes);
        assert_eq!(permit(&mut takes, Some("org.a.Alpha"), &grant), Permit::NotAsked);
        assert_eq!(permit(&mut takes, Some("org.a.Alpha"), &grant), Permit::TooOften);
        assert_eq!(permit(&mut takes, None, &grant), Permit::TooOften);
        let mut takes = Rate::new(2, Duration::from_secs(1));
        assert_eq!(permit(&mut takes, None, &|_| false), Permit::Yes);
    }

    #[test]
    fn two_sides_are_armed_at_once_and_a_newer_one_replaces_its_own_side_only() {
        let mut slots: Slots<u32> = Slots::default();
        assert_eq!(slots.ask(1, Side::Left), None);
        assert_eq!(slots.ask(2, Side::Right), None);
        assert_eq!((slots.at(Side::Left), slots.at(Side::Right)), (Some(&1), Some(&2)));
        assert_eq!(slots.at(Side::Bottom), None);
        assert_eq!(slots.ask(3, Side::Left), Some(1));
        assert_eq!((slots.at(Side::Left), slots.at(Side::Right)), (Some(&3), Some(&2)));
        assert!(slots.is_replaced(&1) && !slots.is_replaced(&2) && !slots.is_replaced(&3));
        assert_eq!(slots.ask(4, Side::Top), None);
        assert_eq!(slots.ask(5, Side::Bottom), None);
        let mut sides = slots.sides(|_| true);
        sides.sort_by_key(|s| *s as u8);
        assert_eq!(sides, [Side::Left, Side::Right, Side::Top, Side::Bottom]);
        slots.forget(&1);
        slots.forget(&3);
        assert_eq!(slots.at(Side::Left), None);
        assert!(!slots.all().any(|c| *c == 1 || *c == 3));
    }

    #[test]
    fn what_is_kept_for_a_program_asking_again_and_again_is_counted() {
        let mut slots: Slots<u32> = Slots::default();
        for n in 0..20u32 {
            slots.ask(n, if n % 2 == 0 { Side::Left } else { Side::Top });
        }
        assert_eq!(slots.held(|_| true), 20);
        assert!(slots.held(|c| c % 2 == 0) >= MOST_HELD, "the handler refuses the ninth");
        for n in 0..20u32 {
            slots.forget(&n);
        }
        assert_eq!(slots.held(|_| true), 0);
        assert_eq!(MOST_HELD, 8);
    }

    #[test]
    fn a_program_in_manual_mode_has_no_side_a_push_may_cross_at() {
        let mut slots: Slots<u32> = Slots::default();
        slots.ask(1, Side::Left);
        slots.ask(2, Side::Right);
        assert_eq!(slots.sides(|c| *c != 2), [Side::Left]);
        assert_eq!(slots.sides(|c| *c != 1 && *c != 2), []);
    }

    #[test]
    fn the_side_pushed_hardest_is_the_one_that_crosses() {
        let sides = [Side::Left, Side::Right];
        let (beyond, along) = pushed(&sides, &[Side::Right], at(1310.0, 301.0), at(1279.0, 300.0));
        assert_eq!(beyond, Some((Side::Right, 31.0)));
        assert_eq!(along, 1.0);
        let (beyond, along) = pushed(&sides, &[Side::Left], at(0.0, 340.0), at(0.0, 300.0));
        assert_eq!((beyond, along), (None, 40.0));
        let corner = [Side::Left, Side::Top];
        let (beyond, _) = pushed(&corner, &[Side::Left, Side::Top], at(-10.0, -30.0), at(0.0, 0.0));
        assert_eq!(beyond, Some((Side::Top, 30.0)));
        let (beyond, _) = pushed(&[Side::Right], &[Side::Left], at(-50.0, 300.0), at(0.0, 300.0));
        assert_eq!(beyond, None);
    }

    #[test]
    fn cross_left_then_let_go_then_cross_right() {
        let mut slots: Slots<u32> = Slots::default();
        slots.ask(1, Side::Left);
        slots.ask(2, Side::Right);
        let sides = slots.sides(|_| true);
        let mut push = EdgePush::needing(CROSS_PUSH);
        let mut crossed = None;
        for (i, t) in [990u32, 1000, 1010].into_iter().enumerate() {
            let (beyond, along) = pushed(&sides, &[Side::Left], at(-30.0, 300.0), at(0.0, 300.0));
            let got = push.feed(beyond, along, true, t);
            assert_eq!(got.is_some(), i == 2, "push {i}");
            crossed = got.or(crossed);
        }
        assert_eq!(crossed.and_then(|s| slots.at(s)), Some(&1));
        let mut push = EdgePush::needing(CROSS_PUSH);
        let mut crossed = None;
        for t in [1990u32, 2000, 2010] {
            let (beyond, along) = pushed(&sides, &[Side::Right], at(1310.0, 300.0), at(1279.0, 300.0));
            crossed = push.feed(beyond, along, true, t).or(crossed);
        }
        assert_eq!(crossed.and_then(|s| slots.at(s)), Some(&2));
    }

    #[test]
    fn locking_the_screen_always_hands_the_mouse_and_keyboard_back() {
        let lock = include_str!("lock.rs");
        let ends = lock.find("self.end_capture(Some(crate::protocols::input_capture::capture::DeactivatedReason::Locked), None)").expect("lock ends the capture");
        let first_change = lock.find("keyboard.unset_grab(self)").expect("lock lets go of grabs");
        assert!(ends < first_change, "the capture must end before the lock changes anything else");
        let code = include_str!("capture.rs").split("\n#[cfg(test)]").next().unwrap();
        assert!(code.contains("self.capture.active.is_some() && self.is_locked()"), "input while locked must end a capture");
    }
}
