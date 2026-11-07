// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Mouse and keyboard presses that programs made up, and whether they count as the person's own hands.

use super::KeyAction;
use crate::{state::Backend, AnvilState};

const HANDS_LOOKED_UP: std::time::Duration = std::time::Duration::from_millis(500);

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn virtual_input(&mut self, from: crate::virtual_input::Maker, did: crate::virtual_input::Did) {
        if from.from_box && self.is_locked() {
            return;
        }
        let moves = matches!(did, crate::virtual_input::Did::Motion(_) | crate::virtual_input::Did::Absolute(_));
        if self.persons_hands(&from, &did) {
            let was = std::mem::replace(&mut self.ring.maker, from.plugin.clone());
            self.by_hand(moves, |state| state.made_up(from.from_box, true, did));
            self.ring.maker = was;
            if moves {
                self.ring.made_at = None;
            }
            return;
        }
        let before = std::mem::replace(&mut self.ring.by_hand, false);
        let was = std::mem::replace(&mut self.ring.maker, from.plugin.clone());
        self.made_up(from.from_box, false, did);
        self.ring.by_hand = before;
        self.ring.maker = was;
        if moves {
            self.ring.made_at = from.plugin.map(|plugin| (plugin, self.pointer.current_location()));
        }
    }

    fn persons_hands(&mut self, from: &crate::virtual_input::Maker, did: &crate::virtual_input::Did) -> bool {
        use crate::virtual_input::Did;
        let pointer = !matches!(did, Did::Key(_));
        if !from.from_box && !pointer {
            return false;
        }
        let now = std::time::Instant::now();
        let who = (from.from_box, from.plugin.clone());
        if let Some((seen, said, at)) = &self.ring.hands_seen {
            if *seen == who && now.saturating_duration_since(*at) < HANDS_LOOKED_UP {
                return *said;
            }
        }
        let said = match &from.plugin {
            Some(id) => from.from_box && crate::own_hands::is_flow(id),
            None => !from.from_box && crate::own_helpers::on(crate::own_helpers::Helper::Clicker),
        };
        self.ring.hands_seen = Some((who, said, now));
        said
    }

    pub(super) fn made_up(&mut self, from_box: bool, hands: bool, did: crate::virtual_input::Did) {
        if self.capture.is_active() || !did.is_press() {
            if from_box {
                if self.capture.is_active() && did.is_press() {
                    return;
                }
            } else if self.captured(did.taken()) {
                return;
            }
        }
        if hands {
            if did.is_press() && self.roots_question() {
                return;
            }
        } else if self.not_for_a_made_up_press(&did) {
            self.say_refused(&did);
            return;
        }
        self.notify_activity();
        self.capture.from_box = from_box;
        self.virtual_did(did);
        self.capture.from_box = false;
    }

    fn virtual_did(&mut self, did: crate::virtual_input::Did) {
        use crate::virtual_input::{Did, Virtual};
        match did {
            Did::Key(event) => match self.keyboard_key_to_action::<Virtual>(event) {
                KeyAction::VtSwitch(_) => {}
                KeyAction::Bound(action) if action.is_restart() => tracing::warn!("a made-up key set off a restart: left unrun"),
                action => self.process_common_key_action(action),
            },
            Did::Motion(event) => self.on_pointer_move::<Virtual>(event),
            Did::Absolute(event) if self.space.outputs().next().is_some() => self.on_pointer_move_absolute::<Virtual>(event),
            Did::Absolute(_) => {}
            Did::Button(event) => self.on_pointer_button::<Virtual>(event),
            Did::Scroll(event) => self.on_pointer_axis::<Virtual>(event),
        }
    }

    pub(crate) fn changed_key(&mut self, key: crate::virtual_input::Key) -> Option<i32> {
        use crate::virtual_input::Virtual;
        match self.keyboard_key_to_action::<Virtual>(key) {
            KeyAction::VtSwitch(vt) => Some(vt),
            action => {
                self.process_common_key_action(action);
                None
            }
        }
    }

    pub(crate) fn changed_button(&mut self, button: crate::virtual_input::Button) {
        self.on_pointer_button::<crate::virtual_input::Virtual>(button);
    }

}
