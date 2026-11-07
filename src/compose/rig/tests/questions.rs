// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Questions only the person may answer, and what may be typed into them.
// The source guards read what comes before this attribute as the live code.

#[cfg(test)]
use crate::rig::kept_ring::{a_while, beat};
use crate::rig::*;

#[test]
fn the_password_question_is_typed_into_slowly_after_the_ring_was_opened_and_nothing_is_ended() {
    alone("rig::tests::questions::the_password_question_is_typed_into_slowly_after_the_ring_was_opened_and_nothing_is_ended", |dir| {
        let question = crate::trusted::QUESTION;
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        let alive = (ring.number, ring.pid);
        ring.turn(&["trindle-ring"]);
        let panel = rig.panel(ring.who, "trindle-ring");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A]);
        rig.put_away(&panel);
        ring.turn(&[]);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        ring.turn(&[]);
        ring.turn(&[question]);
        rig.panel(ring.who, question);
        assert_eq!(rig.keys_with().as_deref(), Some(question));
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.state.ring.running_now(), Some(alive), "the first key typed into the question ended the ring");
        assert_eq!(rig.pressed(ring.who), [KEY_A, KEY_A], "the first key did not reach the question");
        a_while(&mut rig, &mut ring, &[question]);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.state.ring.running_now(), Some(alive), "a key typed slowly ended the ring");
        assert_eq!(rig.pressed(ring.who), [KEY_A, KEY_A, KEY_A]);
        a_while(&mut rig, &mut ring, &[question]);
        rig.key_by_hand(KEY_F5);
        assert_eq!(rig.state.ring.running_now(), Some(alive), "the ring's key ended the ring while the question was showing");
        assert_eq!(rig.pressed(ring.who), [KEY_A, KEY_A, KEY_A, KEY_F5], "the ring's key did not go to the question");
        assert_eq!(rig.ran(), 0, "the ring's command was run over the question");
        assert!(rig.state.ring.wished.is_none() && rig.state.ring.expecting.is_none(), "the ring's key counted as asking for the ring while the question held the keyboard");
        assert_eq!(rig.keys_with().as_deref(), Some(question));
        assert_eq!(rig.pressed(window), [] as [u32; 0]);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A], "a ring that had stopped with the question out kept the key");
        let ended = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != ring.number));
        assert!(ended, "a ring that had stopped with the question out was not replaced");
    });
}

#[test]
fn the_rings_key_with_the_password_question_showing_ends_nothing_and_what_is_typed_is_the_password() {
    alone("rig::tests::questions::the_rings_key_with_the_password_question_showing_ends_nothing_and_what_is_typed_is_the_password", |dir| {
        let question = crate::trusted::QUESTION;
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let mut ring = rig.kept_ring();
        let alive = (ring.number, ring.pid);
        ring.turn(&["trindle-ring"]);
        let panel = rig.panel(ring.who, "trindle-ring");
        rig.put_away(&panel);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        ring.turn(&[question]);
        let asked = rig.panel(ring.who, question);
        a_while(&mut rig, &mut ring, &[question]);
        rig.key_by_hand(KEY_F5);
        assert_eq!(rig.state.ring.running_now(), Some(alive), "the ring's key ended the ring and the question with it");
        assert_eq!(rig.pressed(ring.who), [KEY_F5]);
        assert_eq!(rig.ran(), 0);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_F5, KEY_A], "what was typed after the ring's key did not reach the question");
        assert_eq!(rig.keys_with().as_deref(), Some(question));
        ring.turn(&[question, "trindle-ring"]);
        rig.show(ring.who, &panel, 200, 40);
        a_while(&mut rig, &mut ring, &[question, "trindle-ring"]);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some(question), "the ring opened under the question took what was typed");
        assert_eq!(rig.state.ring.running_now(), Some(alive));
        rig.put_away(&asked);
        a_while(&mut rig, &mut ring, &["trindle-ring"]);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some("trindle-ring"));
        assert_eq!(rig.state.ring.running_now(), Some(alive));
    });
}

#[test]
fn a_made_up_key_never_ends_the_ring_while_a_question_only_the_person_may_answer_is_showing() {
    alone("rig::tests::questions::a_made_up_key_never_ends_the_ring_while_a_question_only_the_person_may_answer_is_showing", |dir| {
        let question = crate::trusted::QUESTION;
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        let alive = (ring.number, ring.pid);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        beat(&mut ring.pulse);
        rig.panel(ring.who, question);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        rig.made_key(&plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.state.ring.running_now(), Some(alive), "a made-up key ended the ring, and the question with it");
        assert_eq!(rig.keys_with().as_deref(), Some(question), "a made-up key took the keyboard from the question");
        assert_eq!(rig.pressed(ring.who), [] as [u32; 0], "a made-up key was typed into the question");
        assert_eq!(rig.pressed(window), [] as [u32; 0]);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A]);
        let ended = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != ring.number));
        assert!(ended, "the person's own key did not end a ring that had stopped with the question out");
    });
}

#[test]
fn typing_into_the_keyrings_box_or_at_the_lock_screen_never_ends_the_ring() {
    alone("rig::tests::questions::typing_into_the_keyrings_box_or_at_the_lock_screen_never_ends_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        let mut ring = rig.kept_ring();
        let alive = (ring.number, ring.pid);
        ring.turn(&["trindle-ring"]);
        let panel = rig.panel(ring.who, "trindle-ring");
        let keyring = rig.join(ClientState::default());
        rig.window_called(keyring, crate::person_only::KEYRING);
        rig.put_away(&panel);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        for typed in 1..=3 {
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(keyring).len(), typed, "a key typed into the keyring's box did not reach it");
            assert_eq!(rig.state.ring.running_now(), Some(alive), "a key typed into the keyring's box ended the ring");
        }
        rig.show(ring.who, &panel, 200, 40);
        rig.state.lock_for_rig();
        for _ in 0..3 {
            rig.key_by_hand(KEY_A);
            rig.key_by_hand(KEY_F5);
        }
        assert_eq!(rig.state.ring.running_now(), Some(alive), "a key typed at the lock screen ended the ring");
        assert_eq!(rig.pressed(ring.who), [] as [u32; 0], "a key typed at the lock screen reached a panel");
        assert_eq!(rig.pressed(keyring).len(), 3, "a key typed at the lock screen reached a window");
    });
}

#[test]
fn no_made_up_press_passes_while_the_administrators_question_is_showing() {
    alone("rig::tests::questions::no_made_up_press_passes_while_the_administrators_question_is_showing", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        let stranger = rig.join(ClientState::default());
        let fake = rig.panel(stranger, crate::trusted::QUESTION);
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.pressed(stranger), [KEY_A], "a stranger's panel by the question's name kept a made-up key back");
        rig.put_away(&fake);
        let question = rig.panel(ring.who, crate::trusted::QUESTION);
        assert_eq!(rig.keys_with().as_deref(), Some(crate::trusted::QUESTION));
        rig.made_key(&plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.pressed(ring.who), [] as [u32; 0], "a made-up key was typed into the administrator's question");
        rig.move_by_hand(0.5, 0.5);
        rig.numbered(ring.who);
        rig.made_click(&plugin);
        rig.made_up_click();
        assert_eq!(rig.numbered(ring.who).len(), 2, "a made-up click was passed to the administrator's question");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "the person's own key did not reach the question");
        rig.put_away(&question);
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A], "a made-up key was still kept back after the question had gone");
    });
}

#[test]
fn every_question_only_the_person_may_answer_is_under_the_one_rule() {
    alone("rig::tests::questions::every_question_only_the_person_may_answer_is_under_the_one_rule", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        assert!(!rig.state.only_the_person_answers());
        for name in [crate::trusted::QUESTION, "trindle-thunder"] {
            let question = rig.panel(ring.who, name);
            assert!(rig.state.only_the_person_answers(), "{name} is not a question only the person may answer");
            rig.made_key(&plugin, KEY_A);
            assert_eq!(rig.pressed(ring.who), [] as [u32; 0], "a made-up key was typed into {name}");
            rig.put_away(&question);
            assert!(!rig.state.only_the_person_answers(), "{name} was still asking once put away");
            let stranger = rig.join(ClientState::default());
            let fake = rig.panel(stranger, name);
            assert!(!rig.state.only_the_person_answers(), "a stranger's panel called {name} was taken for the question");
            rig.put_away(&fake);
        }
        let panel = rig.panel(ring.who, "trindle-ring");
        assert!(!rig.state.only_the_person_answers());
        rig.put_away(&panel);
        let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
        let asked = rig.panel(root, "anything");
        assert!(rig.state.only_the_person_answers());
        rig.put_away(&asked);
        let asker = rig.join(ClientState::default());
        rig.panel(asker, "trindle-permissions");
        assert!(rig.state.only_the_person_answers(), "the permission window is not a question only the person may answer");
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.pressed(asker), [] as [u32; 0], "a made-up key was typed into the permission window");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(asker), [KEY_A]);
        let capture = crate::source::code(include_str!("../../capture.rs"));
        let capture = capture.split("\nmod tests {").next().unwrap();
        assert_eq!(capture.matches("self.only_the_person_answers()").count(), 3, "a push, a take and a capture under way each ask");
        let input = crate::input_handler::code();
        assert!(input.contains("if self.not_for_a_made_up_press(&did) {"));
        assert_eq!(input.matches("self.not_for_a_made_up_press(").count(), 1, "made-up presses are judged in one place");
        for (file, code) in [("capture.rs", capture), ("input_handler.rs", input), ("state.rs", crate::state::code()),
                             ("virtual_keyboard.rs", crate::source::code(include_str!("../../virtual_keyboard.rs")))] {
            for other in ["trusted_showing(", "question_showing(", "trusted::overlays(&layer_map_for_output(o)).next().is_some()"] {
                assert!(!code.contains(other), "{file} judges a question by {other} and not by the one rule");
            }
        }
    });
}

fn tried(rig: &mut Rig, on_screen: usize, mark: &Path, asker: usize, name: &str) {
    use crate::own_helpers::Helper;
    use crate::own_helpers_set::switch;
    let before = rig.pressed(asker).len();
    rig.on_screen_key(on_screen, KEY_A);
    assert_eq!(rig.pressed(asker).len(), before, "{name}: a made-up keyboard typed into it with the on-screen keyboard switched off");
    crate::own_helpers::kept_at(mark, || {
        switch(Helper::Keyboard, true, false).unwrap();
        switch(Helper::Reader, true, true).unwrap();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.pressed(asker).len(), before, "{name}: a made-up keyboard typed into it with the on-screen keyboard switched on by a made-up press");
        switch(Helper::Keyboard, true, true).unwrap();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.pressed(asker).len(), before + 1, "{name}: the on-screen keyboard the person switched on could not type into it");
        switch(Helper::Keyboard, false, false).unwrap();
        switch(Helper::Reader, false, false).unwrap();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.pressed(asker).len(), before + 1, "{name}: a made-up keyboard typed into it after the on-screen keyboard was switched off");
    });
    rig.key_by_hand(KEY_A);
    assert_eq!(rig.pressed(asker).len(), before + 2, "{name}: the person's own key did not reach it");
}

#[test]
fn a_made_up_keyboard_types_into_no_question_until_the_person_switched_it_on() {
    alone("rig::tests::questions::a_made_up_keyboard_types_into_no_question_until_the_person_switched_it_on", |dir| {
        use crate::person_only as only;
        let mark = dir.join("home/.local/state/trindle/own-helpers");
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        let on_screen = rig.join(ClientState::default());
        for name in only::ASKED_BY_TRINDLE {
            beat(&mut ring.pulse);
            let question = rig.panel(ring.who, name);
            tried(&mut rig, on_screen, &mark, ring.who, name);
            rig.put_away(&question);
        }
        for name in only::WINDOWS {
            let asker = rig.join(ClientState::default());
            rig.window_called(asker, name);
            rig.onto_the_window_in_front();
            assert!(rig.has_keyboard(asker), "{name}: the question was not given the keyboard");
            tried(&mut rig, on_screen, &mark, asker, name);
        }
        let plain = rig.join(ClientState::default());
        rig.window_called(plain, "org.example.Notes");
        rig.onto_the_window_in_front();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.pressed(plain), [KEY_A], "a made-up keyboard no longer types into an ordinary window");
        for name in only::ASKED_BY_NAME {
            let asker = rig.join(ClientState::default());
            let question = rig.panel(asker, name);
            tried(&mut rig, on_screen, &mark, asker, name);
            rig.put_away(&question);
        }
        use crate::virtual_keyboard::kept_from_a_made_up_keyboard as kept;
        assert!(kept(false, true, false) && kept(false, false, true), "a made-up keyboard may type at a locked screen or a question with the switch off");
        assert!(!kept(true, true, true), "the on-screen keyboard the person switched on is kept from the lock screen or a question");
        assert!(!kept(false, false, false), "a made-up keyboard is kept from ordinary windows");
        let before = rig.pressed(plain).len();
        rig.state.lock_for_rig();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.pressed(plain).len(), before, "a made-up keyboard typed while the screen was locked");
        let code = crate::source::code(include_str!("../../virtual_keyboard.rs"));
        assert!(code.contains("state.is_locked(), state.only_the_person_answers()"), "the made-up keyboard no longer asks the one rule which windows are questions");
    });
}

#[test]
fn a_menu_that_names_no_opener_is_not_simply_let_pass() {
    alone("rig::tests::questions::a_menu_that_names_no_opener_is_not_simply_let_pass", |dir| {
        let mut rig = Rig::new(dir);
        let outside = rig.join(ClientState::default());
        let menu = rig.menu_without_opener(outside);
        let seen = rig.server_side(outside, &menu);
        assert!(rig.state.popups.find_popup(&seen).is_some_and(|m| smithay::desktop::find_popup_root_surface(&m).is_err()), "the menu names an opener after all");
        assert!(rig.state.kept_for_the_person(&seen), "a menu of no opener from outside every box was let pass");
        let boxed = rig.boxed();
        let menu = rig.menu_without_opener(boxed);
        let seen = rig.server_side(boxed, &menu);
        assert!(rig.state.popups.find_popup(&seen).is_some_and(|m| smithay::desktop::find_popup_root_surface(&m).is_err()));
        assert!(!rig.state.kept_for_the_person(&seen), "a menu of no opener from a box was kept");
    });
}

#[test]
fn the_password_question_stands_above_the_ring_and_has_the_keys_whichever_opened_first() {
    alone("rig::tests::questions::the_password_question_stands_above_the_ring_and_has_the_keys_whichever_opened_first", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let panel = rig.panel(ring.who, "trindle-ring");
        let question = rig.panel(ring.who, crate::trusted::QUESTION);
        assert_eq!(rig.standing(), ["trindle-ring", crate::trusted::QUESTION]);
        assert_eq!(rig.keys_with().as_deref(), Some(crate::trusted::QUESTION));
        rig.put_away(&panel);
        rig.show(ring.who, &panel, 200, 40);
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some(crate::trusted::QUESTION), "the ring opened after the question took what was typed");
        rig.put_away(&panel);
        rig.put_away(&question);
        let question = rig.panel(ring.who, crate::trusted::QUESTION);
        let again = rig.panel(ring.who, "trindle-ring");
        let standing = rig.standing();
        let place = |name: &str| standing.iter().rposition(|n| n == name).unwrap();
        assert!(place(crate::trusted::QUESTION) > place("trindle-ring"), "the ring opened after the question stands above it: {standing:?}");
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some(crate::trusted::QUESTION), "the ring opened after the question took what was typed");
        let news = rig.panel(ring.who, "trindle-notes");
        assert_eq!(rig.standing().last().map(String::as_str), Some("trindle-notes"));
        rig.put_away(&news);
        rig.put_away(&question);
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some("trindle-ring"));
        rig.put_away(&again);
        let other = rig.join(ClientState::default());
        rig.panel(other, crate::trusted::QUESTION);
        rig.panel(ring.who, "trindle-ring");
        let standing = rig.standing();
        let ring_at = standing.iter().rposition(|n| n == "trindle-ring").unwrap();
        let others = standing.iter().position(|n| n == crate::trusted::QUESTION).unwrap();
        assert!(ring_at > others, "a panel only called the question stands above the ring: {standing:?}");
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.keys_with().as_deref(), Some("trindle-ring"), "a panel only called the question took the keys from the ring opened after it");
    });
}

#[test]
fn a_window_opened_under_the_ring_has_the_keyboard_the_moment_the_ring_is_put_away() {
    alone("rig::tests::questions::a_window_opened_under_the_ring_has_the_keyboard_the_moment_the_ring_is_put_away", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let panel = rig.panel(ring.who, "trindle-ring");
        let keyring = rig.join(ClientState::default());
        rig.window(keyring, false);
        beat(&mut ring.pulse);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A]);
        assert_eq!(rig.pressed(keyring), [] as [u32; 0]);
        rig.put_away(&panel);
        assert!(rig.has_keyboard(keyring), "the window opened under the ring was not given the keyboard as the ring was put away");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(keyring), [KEY_A]);
        assert_eq!(rig.pressed(window), [] as [u32; 0]);
    });
}

#[test]
fn a_password_box_opened_over_a_pointed_circle_is_typed_into() {
    alone("rig::tests::questions::a_password_box_opened_over_a_pointed_circle_is_typed_into", |dir| {
        let mut rig = Rig::new(dir);
        let ring = rig.kept_ring();
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        rig.panel(ring.who, "trindle-notes");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "the circle was not given the keys before any window opened");
        let keyring = rig.join(ClientState::default());
        rig.window_called(keyring, crate::person_only::KEYRING);
        for typed in 1..=3 {
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(keyring).len(), typed, "key {typed} went to a pointed circle, not the keyring's box");
        }
        assert_eq!(rig.pressed(ring.who), [KEY_A], "the circle went on taking keys under the keyring's box");
        assert_eq!(rig.keys_with(), None);
        assert!(rig.has_keyboard(keyring));
    });
}

struct StaleGrab(smithay::input::keyboard::GrabStartData<AnvilState<RigData>>);

impl smithay::input::keyboard::KeyboardGrab<AnvilState<RigData>> for StaleGrab {
    fn input(&mut self, data: &mut AnvilState<RigData>, handle: &mut smithay::input::keyboard::KeyboardInnerHandle<'_, AnvilState<RigData>>,
             keycode: smithay::input::keyboard::Keycode, state: smithay::backend::input::KeyState,
             modifiers: Option<smithay::input::keyboard::ModifiersState>, serial: smithay::utils::Serial, time: u32) {
        handle.input(data, keycode, state, modifiers, serial, time);
    }
    fn set_focus(&mut self, _: &mut AnvilState<RigData>, _: &mut smithay::input::keyboard::KeyboardInnerHandle<'_, AnvilState<RigData>>,
                 _: Option<crate::focus::KeyboardFocusTarget>, _: smithay::utils::Serial) {}
    fn start_data(&self) -> &smithay::input::keyboard::GrabStartData<AnvilState<RigData>> {
        &self.0
    }
    fn unset(&mut self, _: &mut AnvilState<RigData>) {}
}

#[test]
fn a_password_box_opened_while_a_stale_grab_holds_the_keyboard_is_typed_into() {
    alone("rig::tests::questions::a_password_box_opened_while_a_stale_grab_holds_the_keyboard_is_typed_into", |dir| {
        let mut rig = Rig::new(dir);
        let other = rig.join(ClientState::default());
        rig.window(other, false);
        let keyboard = rig.state.seat.get_keyboard().unwrap();
        let start = smithay::input::keyboard::GrabStartData { focus: keyboard.current_focus() };
        keyboard.set_grab(&mut rig.state, StaleGrab(start), smithay::utils::SERIAL_COUNTER.next_serial());
        let keyring = rig.join(ClientState::default());
        rig.window_called(keyring, crate::person_only::KEYRING);
        for typed in 1..=3 {
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(keyring).len(), typed, "key {typed} went to the window holding a stale grab, not the keyring's box");
        }
        assert_eq!(rig.pressed(other), [] as [u32; 0]);
    });
}
