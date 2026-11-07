// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The doors input events come in by on the windowed compositor, and the helpers every door shares.

use super::{KeyAction, MOUSE_BUTTONS};
use crate::{state::Backend, AnvilState};

use smithay::backend::input::{InputBackend, InputEvent};

pub(super) fn moves_pointer<B: InputBackend>(event: &InputEvent<B>) -> bool {
    matches!(event, InputEvent::PointerMotion { .. } | InputEvent::PointerMotionAbsolute { .. } | InputEvent::TabletToolAxis { .. })
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(super) fn by_hand(&mut self, moves: bool, handle: impl FnOnce(&mut Self)) {
        let before = std::mem::replace(&mut self.ring.by_hand, true);
        handle(self);
        self.ring.by_hand = before;
        if moves {
            self.ring.hand_at = Some(self.pointer.current_location());
        }
    }

    pub(crate) fn pad_press(&mut self, code: u32, pressed: bool) {
        use crate::virtual_input::{Button, Did, Key};
        let time = self.clock.now().as_millis();
        let did = if MOUSE_BUTTONS.contains(&code) { Did::Button(Button { time, button: code, pressed }) } else { Did::Key(Key { time, code, pressed }) };
        self.by_hand(false, |state| state.made_up(false, true, did));
    }

    pub fn process_input_event_windowed<B: InputBackend>(&mut self, event: InputEvent<B>, output_name: &str) {
        let moves = moves_pointer(&event);
        self.through_door(true, moves, |state| state.by_hand(moves, |state| state.windowed_event(event, output_name)));
    }

    pub(crate) fn windowed_event<B: InputBackend>(&mut self, event: InputEvent<B>, output_name: &str) {
        if is_activity(&event) {
            self.notify_activity();
        }
        if crate::capture::Taken::from_event(&event).is_some_and(|did| self.captured(did)) {
            return;
        }
        self.keys_device(&event);
        if self.kept_from_a_program(&event) {
            return;
        }
        if self.through_table(&event).is_some() {
            return;
        }
        if let InputEvent::PointerAxis { .. } = event {
            self.keys_before_scroll();
        }
        match event {
            InputEvent::Keyboard { event } => match self.keyboard_key_to_action::<B>(event) {
                action @ (KeyAction::None | KeyAction::Bound(_) | KeyAction::Expand(..)) => self.process_common_key_action(action),
                action => tracing::warn!(?action, output_name, "Key action unsupported on on output backend."),
            },

            InputEvent::PointerMotionAbsolute { event } => {
                let output = self
                    .space
                    .outputs()
                    .find(|o| o.name() == output_name)
                    .unwrap()
                    .clone();
                self.on_pointer_move_absolute_windowed::<B>(event, &output)
            }
            InputEvent::PointerButton { event } => self.on_pointer_button::<B>(event),
            InputEvent::PointerAxis { event } => self.on_pointer_axis::<B>(event),
            _ => (),
        }
    }
}

pub(super) fn is_activity<B: InputBackend>(event: &InputEvent<B>) -> bool {
    !matches!(event, InputEvent::DeviceAdded { .. } | InputEvent::DeviceRemoved { .. })
}
