// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

#[cfg(test)]
use crate::rig::*;

fn media_keys_run(rig: &mut Rig) -> (usize, usize) {
    for code in [115, 114, 113, 248, 224, 225] {
        rig.key_by_hand(code);
    }
    let sound = rig.ran_lines("sound").into_iter().filter(|l| !l.starts_with("sound get-")).count();
    (sound, rig.ran_lines("light").len())
}

fn with_shipped_shortcuts(rig: &mut Rig) {
    let mut shipped = crate::config::Config::default();
    shipped.read_rc(include_str!("../../../../config/desktop/shortcuts.xml"));
    rig.state.config.keybinds = shipped.keybinds;
}

#[test]
fn media_keys_act_with_nothing_focused_and_nothing_clicked() {
    alone("rig::tests::media_keys::media_keys_act_with_nothing_focused_and_nothing_clicked", |dir| {
        let mut rig = Rig::new(dir);
        with_shipped_shortcuts(&mut rig);
        assert!(rig.state.seat.get_keyboard().unwrap().current_focus().is_none());
        assert_eq!(media_keys_run(&mut rig), (4, 2), "no window, no click");
    });
}

#[test]
fn media_keys_act_after_the_last_window_closes() {
    alone("rig::tests::media_keys::media_keys_act_after_the_last_window_closes", |dir| {
        let mut rig = Rig::new(dir);
        with_shipped_shortcuts(&mut rig);
        let who = rig.join(ClientState::default());
        let window = rig.window(who, false);
        rig.put_away(&window);
        assert_eq!(media_keys_run(&mut rig), (4, 2), "after the last window closed");
    });
}

#[test]
fn media_keys_act_with_the_greetings_panel_put_away() {
    alone("rig::tests::media_keys::media_keys_act_with_the_greetings_panel_put_away", |dir| {
        let mut rig = Rig::new(dir);
        with_shipped_shortcuts(&mut rig);
        let who = rig.join(ClientState::default());
        let panel = rig.panel(who, "trindle-ring");
        rig.put_away(&panel);
        assert_eq!(media_keys_run(&mut rig), (4, 2), "after the ring's panel was put away");
    });
}

#[test]
fn media_keys_act_while_a_panel_holds_the_whole_keyboard() {
    alone("rig::tests::media_keys::media_keys_act_while_a_panel_holds_the_whole_keyboard", |dir| {
        let mut rig = Rig::new(dir);
        with_shipped_shortcuts(&mut rig);
        let who = rig.join(ClientState::default());
        rig.panel(who, "trindle-ring");
        assert_eq!(media_keys_run(&mut rig), (4, 2), "with the ring's panel up");
    });
}

#[test]
fn the_volume_and_brightness_keys_run_their_commands() {
    alone("rig::tests::media_keys::the_volume_and_brightness_keys_run_their_commands", |dir| {
        let mut rig = Rig::new(dir);
        rig.state.config.keybinds.clear();
        let mut shipped = crate::config::Config::default();
        shipped.read_rc(include_str!("../../../../config/desktop/shortcuts.xml"));
        rig.state.config.keybinds = shipped.keybinds;
        let window = rig.join(ClientState::default());
        rig.window(window, false);

        for code in [115, 114, 113, 248, 224, 225] {
            rig.key_by_hand(code);
        }

        let sound: Vec<String> = rig.ran_lines("sound").into_iter().filter(|l| !l.starts_with("sound get-")).collect();
        let light = rig.ran_lines("light");
        assert_eq!(sound.len(), 4, "the sound keys ran {sound:?}");
        assert_eq!(light, ["light down", "light up"], "the brightness keys ran {light:?}");
    });
}
