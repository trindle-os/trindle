//! Switching to a text console works whatever holds the keyboard.

#[cfg(test)]
use crate::rig::*;

const KEY_LEFTCTRL: u32 = 29;
const KEY_LEFTALT: u32 = 56;
const KEY_F2: u32 = 60;

#[test]
fn a_console_key_is_never_handed_to_a_panel_holding_the_keyboard() {
    alone("rig::tests::console_keys::a_console_key_is_never_handed_to_a_panel_holding_the_keyboard", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.panel(who, "trindle-ring");
        assert_eq!(rig.keys_with().as_deref(), Some("trindle-ring"));
        rig.key_by_hand_only(KEY_LEFTCTRL, true);
        rig.key_by_hand_only(KEY_LEFTALT, true);
        rig.key_by_hand(KEY_F2);
        rig.key_by_hand_only(KEY_LEFTALT, false);
        rig.key_by_hand_only(KEY_LEFTCTRL, false);
        assert!(!rig.pressed(who).contains(&KEY_F2), "the ring was handed the console key: {:?}", rig.pressed(who));

        rig.key_by_hand(KEY_F2);
        assert!(rig.pressed(who).contains(&KEY_F2), "{:?}", rig.pressed(who));
    });
}
