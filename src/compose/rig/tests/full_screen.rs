// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Questions and the ring over a window that fills the screen.
// The source guards read what comes before this attribute as the live code.

#[cfg(test)]
use crate::rig::kept_ring::{beat, KeptRing};
use crate::rig::*;

fn filled(rig: &Rig) -> bool {
    rig.state.space.outputs().any(|o| o.user_data().get::<crate::shell::FullscreenSurface>().and_then(|f| f.get()).is_some())
}

#[test]
fn a_question_that_comes_out_over_a_full_screen_window_ends_the_full_screen() {
    alone("rig::tests::full_screen::a_question_that_comes_out_over_a_full_screen_window_ends_the_full_screen", |dir| {
        let mut rig = Rig::new(dir);
        let game = rig.join(ClientState::default());
        let asker = rig.join(ClientState::default());
        rig.window(game, true);
        assert!(filled(&rig));
        rig.panel(asker, "anything");
        rig.window_called(asker, "org.example.Other");
        assert!(filled(&rig), "a stranger's panel or window ended the full screen");
        rig.panel(asker, crate::person_only::PERMISSIONS);
        assert!(!filled(&rig), "the permission window stayed hidden behind a full-screen window");
        assert_eq!(rig.hidden(crate::person_only::PERMISSIONS), [false]);
        for name in crate::person_only::WINDOWS {
            let game = rig.join(ClientState::default());
            rig.window(game, true);
            assert!(filled(&rig));
            rig.window_called(asker, name);
            assert!(!filled(&rig), "{name} stayed hidden behind a full-screen window");
        }
    });
}

#[test]
fn one_asking_for_the_ring_lifts_it_over_a_full_screen_window_once() {
    alone("rig::tests::full_screen::one_asking_for_the_ring_lifts_it_over_a_full_screen_window_once", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_F5);
        let panel = rig.panel(ring.who, "trindle-ring");
        assert_eq!(rig.hidden("trindle-ring"), [false]);
        assert!(rig.has_keyboard(ring.who));
        assert!(rig.state.ring.wished.is_none(), "the asking was not used up as the ring came out for it");
        rig.put_away(&panel);
        rig.show(ring.who, &panel, 200, 40);
        assert_eq!(rig.hidden("trindle-ring"), [true], "one asking lifted the ring over a full-screen window a second time");
        assert!(!rig.has_keyboard(ring.who));
        rig.put_away(&panel);
        rig.key_by_hand(KEY_F5);
        rig.show(ring.who, &panel, 200, 40);
        assert_eq!(rig.hidden("trindle-ring"), [false]);
        rig.state.ring.wished = Some(Instant::now());
        rig.put_away(&panel);
        assert!(rig.state.ring.wished.is_none(), "the key that put the ring away still stands as an asking");
        rig.show(ring.who, &panel, 200, 40);
        assert_eq!(rig.hidden("trindle-ring"), [true], "the key that put the ring away lifted the next ring over a full-screen window");
        rig.put_away(&panel);
        rig.key_by_hand(KEY_F5);
        rig.panel(ring.who, "trindle-notes");
        rig.show(ring.who, &panel, 200, 40);
        assert_eq!(rig.hidden("trindle-ring"), [false], "a notification used up the person's asking for the ring");
    });
}

#[test]
fn a_ring_held_back_is_started_by_its_key_and_only_the_persons_own_lifts_it() {
    alone("rig::tests::full_screen::a_ring_held_back_is_started_by_its_key_and_only_the_persons_own_lifts_it", |dir| {
        let (program, leave) = (dir.join("trindle"), dir.join("leave"));
        std::fs::write(&program, format!("#!/bin/sh\nwhile kill -0 $PPID 2>/dev/null; do if rm {} 2>/dev/null; then exit 0; fi; sleep 0.05; done\n", leave.display())).unwrap();
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        rig.kept_ring();
        let held_back = |rig: &mut Rig| {
            for _ in 0..20 {
                std::fs::write(&leave, "").unwrap();
                let before = rig.state.ring.running_now().map(|r| r.0);
                rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().map(|r| r.0) != before);
                if rig.state.ring.running_now().is_none() {
                    let _ = std::fs::remove_file(&leave);
                    return;
                }
            }
            panic!("the ring was never held back");
        };
        held_back(&mut rig);
        assert_eq!(rig.state.ring.counted(), 5);
        rig.made_up_key(KEY_F5);
        assert!(rig.state.ring.running_now().is_some(), "a made-up key did not start the ring that was held back");
        assert_eq!(rig.state.ring.counted(), 6, "the count was begun again for a made-up key");
        assert_eq!(rig.ran(), 0, "the ring's command was run by name while the kept ring was held back");
        assert!(rig.state.ring.wished.is_none(), "a made-up key counted as the person asking for the ring");
        held_back(&mut rig);
        assert_eq!(rig.state.ring.counted(), 6, "a ring started by a made-up key was given starts of its own");
        for _ in 0..3 {
            rig.made_up_key(KEY_F5);
            assert!(rig.state.ring.running_now().is_none(), "a made-up key started a held-back ring a second time");
        }
        assert_eq!(rig.state.ring.counted(), 6);
        assert_eq!(rig.ran(), 0, "a made-up key refused its start ran the ring by name");
        rig.key_by_hand(KEY_F5);
        assert!(rig.state.ring.running_now().is_some(), "the person's key did not start the ring that was held back");
        assert_eq!(rig.state.ring.counted(), 1, "the count was not begun again for the person's own key");
        assert_eq!(rig.ran(), 0);
        assert!(rig.state.ring.wished.is_some(), "the person's own key did not count as asking for the ring");
        held_back(&mut rig);
        assert_eq!(rig.state.ring.counted(), 5);
        rig.made_up_key(KEY_F5);
        assert!(rig.state.ring.running_now().is_some(), "a person steering from another computer was left without a ring");
    });
}

#[test]
fn only_a_plugin_allowed_summon_brings_the_ring_out_over_a_full_screen_window_with_its_made_up_keys() {
    alone("rig::tests::full_screen::only_a_plugin_allowed_summon_brings_the_ring_out_over_a_full_screen_window_with_its_made_up_keys", |dir| {
        use crate::sandbox::{BOXED, CAPTURE, CONTROL, CONTROL_CAPTURE};
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let steer = rig.maker(Some(CONTROL), Some(STEER));
        let other = rig.maker(Some(CONTROL_CAPTURE), Some(OTHER));
        assert_eq!(steer, Maker { from_box: true, plugin: Some(STEER.into()) });
        assert_eq!(other, Maker { from_box: true, plugin: Some(OTHER.into()) });
        let plain = rig.maker(Some(BOXED), Some(STEER));
        let capture = rig.maker(Some(CAPTURE), Some(STEER));
        let outside = rig.maker(None, None);
        assert_eq!((plain.plugin.clone(), capture.plugin.clone()), (None, None));
        assert_eq!(outside, Maker::default());
        let panel = rig.panel(ring.who, "trindle-ring");
        rig.put_away(&panel);
        let pretenders = [("a plain box", &plain), ("a box allowed only capture", &capture), ("a program outside every box", &outside)];
        summon_is_read_off_the_record_at_each_key(&mut rig, &mut ring, &panel, &steer);
        no_one_else_gains_from_the_allowed_plugin(&mut rig, &mut ring, &panel, &steer, &other, &pretenders);
        clicks_never_open_the_ring_and_the_permission_can_be_taken_away(&mut rig, &mut ring, &panel, &steer);
    });
}

const STEER: &str = "org.example.steer";
const OTHER: &str = "org.example.other";

fn key(rig: &mut Rig, ring: &mut KeptRing, panel: &wl_surface::WlSurface, by: &Maker) -> (bool, bool) {
    rig.state.ring.wished = None;
    rig.made_key(by, KEY_F5);
    let asked = rig.state.ring.wished.is_some();
    beat(&mut ring.pulse);
    rig.show(ring.who, panel, 200, 40);
    let over = rig.hidden("trindle-ring") == [false];
    assert_eq!(over, rig.has_keyboard(ring.who), "the ring's keyboard and its showing disagree");
    rig.put_away(panel);
    (asked, over)
}

fn summon_is_read_off_the_record_at_each_key(rig: &mut Rig, ring: &mut KeptRing, panel: &wl_surface::WlSurface, steer: &Maker) {
    rig.allow(STEER, &["network", "control"]);
    rig.allow(OTHER, &["control", "capture"]);
    assert_eq!(key(rig, ring, panel, steer), (false, false), "a plugin never allowed summon brought the ring out over a full-screen window");
    rig.allow(STEER, &["network", "control", "summon"]);
    assert_eq!(key(rig, ring, panel, steer), (true, true), "a plugin allowed summon did not bring the ring out over a full-screen window");
    rig.made_key(steer, KEY_F5);
    rig.show(ring.who, panel, 200, 40);
    rig.put_away(panel);
    rig.show(ring.who, panel, 200, 40);
    assert_eq!(rig.hidden("trindle-ring"), [true], "one made-up key lifted the ring twice");
    rig.put_away(panel);
}

fn no_one_else_gains_from_the_allowed_plugin(rig: &mut Rig, ring: &mut KeptRing, panel: &wl_surface::WlSurface, steer: &Maker, other: &Maker, pretenders: &[(&str, &Maker)]) {
    assert_eq!(key(rig, ring, panel, other), (false, false), "a second plugin gained from the first one's permission");
    for (who, pretender) in pretenders {
        assert_eq!(key(rig, ring, panel, pretender), (false, false), "{who} brought the ring out as a plugin allowed summon");
    }
    rig.allow(OTHER, &["summon"]);
    assert_eq!(key(rig, ring, panel, other), (false, false), "summon counted without control");
    let shots = rig.shots_by_command();
    rig.made_key(steer, KEY_F6);
    assert_eq!(rig.shots_by_command(), shots + 1, "a plugin allowed summon had the screenshot tool started as the person's own");
}

fn clicks_never_open_the_ring_and_the_permission_can_be_taken_away(rig: &mut Rig, ring: &mut KeptRing, panel: &wl_surface::WlSurface, steer: &Maker) {
    let click = |rig: &mut Rig, by: &Maker| {
        rig.state.ring.wished = None;
        rig.made_click(by);
        rig.state.ring.wished.is_some()
    };
    rig.made_move(steer, 0.5, 0.0);
    assert!(!click(rig, steer), "a plugin allowed summon opened the ring over a full-screen window with a click");
    rig.move_by_hand(0.5, 0.5);
    rig.move_by_hand(0.5, 0.0);
    assert!(!click(rig, steer), "a click opened the ring over a full-screen window");
    rig.allow(STEER, &["network", "control"]);
    assert_eq!(key(rig, ring, panel, steer), (false, false), "the permission taken away still counted");
    rig.state.ring.wished = None;
    rig.key_by_hand(KEY_F5);
    assert!(rig.state.ring.wished.is_some());
}

#[test]
fn a_ring_out_unseen_behind_a_full_screen_window_is_brought_out_by_the_persons_own_key() {
    alone("rig::tests::full_screen::a_ring_out_unseen_behind_a_full_screen_window_is_brought_out_by_the_persons_own_key", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        rig.panel(ring.who, "trindle-ring");
        assert_eq!(rig.hidden("trindle-ring"), [true]);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A], "an unseen ring took what was typed at the window");
        rig.made_up_key(KEY_F5);
        assert_eq!(rig.hidden("trindle-ring"), [true], "a made-up key brought the unseen ring out over the window");
        assert_eq!(rig.ran(), 1);
        rig.key_by_hand(KEY_F5);
        assert_eq!(rig.hidden("trindle-ring"), [false], "the person's key did not bring the unseen ring out");
        assert_eq!(rig.ran(), 1, "the ring's command was run, which only puts an unseen ring away");
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "the ring brought out was not given the keyboard");
        assert_eq!(rig.pressed(window), [KEY_A]);
    });
}

#[test]
fn super_and_k_lists_the_shortcuts_over_a_full_screen_window_and_never_over_the_lock() {
    alone("rig::tests::full_screen::super_and_k_lists_the_shortcuts_over_a_full_screen_window_and_never_over_the_lock", |dir| {
        use crate::config::{Action, Keybind, Mods};
        const KEY_LEFTMETA: u32 = 125;
        const KEY_K: u32 = 37;
        let mut rig = Rig::new(dir);
        rig.state.config.keybinds.push(Keybind {
            mods: Mods { logo: true, ..Mods::default() },
            key: smithay::input::keyboard::Keysym::k,
            on_release: false,
            action: Action::Execute(crate::common::combos::LIST_COMMAND.into()),
        });
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        assert!(filled(&rig));
        let chord = |rig: &mut Rig| {
            rig.key_by_hand_only(KEY_LEFTMETA, true);
            rig.key_by_hand(KEY_K);
            rig.key_by_hand_only(KEY_LEFTMETA, false);
        };
        chord(&mut rig);
        assert_eq!(rig.ran_lines("ring"), ["ring --shortcuts"], "Super and K did not ask for the shortcut list over the window");
        assert!(filled(&rig), "the shortcut list moved the full-screen window");
        rig.state.lock_for_rig();
        chord(&mut rig);
        assert_eq!(rig.ran_lines("ring").len(), 1, "Super and K asked for the shortcut list at the lock screen");
    });
}
