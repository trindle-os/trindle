// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

mod devices;
mod doors;
mod edges;
mod focus;
mod kept;
mod keys;
mod made_up;
mod pointer;

use smithay::backend::input::{Event, InputBackend, InputEvent};

use crate::{state::Backend, AnvilState};

const BTN_LEFT: u32 = 0x110;

const MOUSE_BUTTONS: std::ops::Range<u32> = BTN_LEFT..0x120;

impl<BackendData: Backend> AnvilState<BackendData> {
    fn process_common_key_action(&mut self, action: KeyAction) {
        match action {
            KeyAction::None => (),

            KeyAction::Bound(bound) => self.do_action(bound),
            KeyAction::Expand(swap, time) => self.type_swap(swap, time),
            _ => unreachable!(
                "Common key action handler encountered backend specific action {:?}",
                action
            ),
        }
    }
}

#[derive(Debug)]
enum KeyAction {

    Bound(crate::config::Action),

    VtSwitch(i32),

    Expand(crate::expander::Swap, u32),

    None,
}

pub(crate) fn sender<B: InputBackend>(event: &InputEvent<B>) -> Option<B::Device> {
    Some(match event {
        InputEvent::Keyboard { event } => event.device(),
        InputEvent::PointerMotion { event } => event.device(),
        InputEvent::PointerMotionAbsolute { event } => event.device(),
        InputEvent::PointerButton { event } => event.device(),
        InputEvent::PointerAxis { event } => event.device(),
        InputEvent::GestureSwipeBegin { event } => event.device(),
        InputEvent::GestureSwipeUpdate { event } => event.device(),
        InputEvent::GestureSwipeEnd { event } => event.device(),
        InputEvent::GesturePinchBegin { event } => event.device(),
        InputEvent::GesturePinchUpdate { event } => event.device(),
        InputEvent::GesturePinchEnd { event } => event.device(),
        InputEvent::GestureHoldBegin { event } => event.device(),
        InputEvent::GestureHoldEnd { event } => event.device(),
        InputEvent::TouchDown { event } => event.device(),
        InputEvent::TouchMotion { event } => event.device(),
        InputEvent::TouchUp { event } => event.device(),
        InputEvent::TouchCancel { event } => event.device(),
        InputEvent::TouchFrame { event } => event.device(),
        InputEvent::TabletToolAxis { event } => event.device(),
        InputEvent::TabletToolProximity { event } => event.device(),
        InputEvent::TabletToolTip { event } => event.device(),
        InputEvent::TabletToolButton { event } => event.device(),
        InputEvent::SwitchToggle { event } => event.device(),
        _ => return None,
    })
}

#[cfg(test)]
pub(crate) fn code() -> &'static str {
    let mut files: Vec<_> = std::fs::read_dir(crate::source::root().join("src/compose/input_handler"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    files.sort();
    let text: String = files.iter().map(|p| std::fs::read_to_string(p).unwrap()).collect();
    crate::source::code(Box::leak(text.into_boxed_str()))
}
