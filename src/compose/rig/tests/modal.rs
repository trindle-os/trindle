// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

#[cfg(test)]
use crate::rig::*;

const KEY_ESC: u32 = 1;
const BTN: u32 = 0x110;

fn modal_scene(rig: &mut Rig, w: i32, h: i32) -> (usize, usize, wl_surface::WlSurface) {
    let window = rig.join(ClientState::default());
    rig.window(window, true);
    let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
    rig.move_by_hand(0.05, 0.05);
    rig.click_by_hand();
    let modal = rig.panel_sized(root, crate::trusted::MODAL, On::Overlay, w, h, KeyboardInteractivity::None);
    (window, root, modal)
}

#[test]
fn a_modal_holds_every_click_and_key() {
    alone("rig::tests::modal::a_modal_holds_every_click_and_key", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        rig.move_by_hand(0.05, 0.05);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN], "the window did not take a click before the modal");
        let root = rig.join(ClientState { from_root: true, ..ClientState::default() });

        let modal = rig.panel_sized(root, crate::trusted::MODAL, On::Overlay, 200, 40, KeyboardInteractivity::None);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN], "a click reached the window under a modal");
        assert!(rig.has_keyboard(root), "the modal did not take the keyboard though it did not ask");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [] as [u32; 0], "a key reached the window under a modal");
        assert_eq!(rig.pressed(root), [KEY_A]);
        rig.move_by_hand(0.9, 0.9);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN], "a click away from the modal reached the window");
        assert!(rig.refused(root) > 0, "the modal was not told a click was kept from the window");
        rig.move_by_hand(0.5, 0.5);
        rig.click_by_hand();
        assert_eq!(rig.clicked(root), [BTN], "the modal did not take its own click");
        assert!(rig.state.only_the_person_answers(), "the mouse and keyboard could be lent while a modal is out");
        rig.put_away(&modal);
        rig.move_by_hand(0.9, 0.9);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN, BTN], "the window took no click after the modal went");
    });
}

#[test]
fn a_one_point_panel_holds_nothing() {
    alone("rig::tests::modal::a_one_point_panel_holds_nothing", |dir| {
        let mut rig = Rig::new(dir);
        for (w, h) in [(1, 1), (200, 1), (1, 40), (15, 40)] {
            let (window, root, modal) = modal_scene(&mut rig, w, h);
            let other = rig.panel_sized(root, "anything", On::Overlay, w, h, KeyboardInteractivity::Exclusive);
            assert!(rig.state.the_modal().is_none(), "a {w} by {h} modal held the screen");
            assert!(!rig.state.only_the_person_answers(), "a {w} by {h} trusted panel blocked the mouse and keyboard being lent");
            assert!(!rig.has_keyboard(root), "a {w} by {h} trusted panel kept the keyboard");
            let before = rig.clicked(window).len();
            rig.move_by_hand(0.9, 0.9);
            rig.click_by_hand();
            assert_eq!(rig.clicked(window).len(), before + 1, "a {w} by {h} panel kept a click from the window");
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(root), [] as [u32; 0]);
            rig.put_away(&modal);
            rig.put_away(&other);
            rig.crash(root);
            rig.crash(window);
        }
    });
}

#[test]
fn a_modal_ends_when_its_owner_crashes() {
    alone("rig::tests::modal::a_modal_ends_when_its_owner_crashes", |dir| {
        let mut rig = Rig::new(dir);
        let (window, root, _modal) = modal_scene(&mut rig, 200, 40);
        assert!(rig.state.the_modal().is_some());
        rig.crash(root);
        assert!(rig.state.the_modal().is_none(), "the modal outlived its owner");
        assert!(!rig.state.only_the_person_answers());
        rig.move_by_hand(0.9, 0.9);
        rig.click_by_hand();
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.clicked(window), [BTN, BTN], "the window took no click after the owner crashed");
        assert_eq!(rig.pressed(window), [KEY_A]);
    });
}

#[test]
fn a_modal_ends_at_the_time_limit() {
    alone("rig::tests::modal::a_modal_ends_at_the_time_limit", |dir| {
        let mut rig = Rig::new(dir);
        rig.state.modal_limit = Duration::from_millis(400);
        let (window, root, _modal) = modal_scene(&mut rig, 200, 40);
        assert!(rig.state.the_modal().is_some());
        assert!(rig.until(Duration::from_secs(5), |r| r.closed(root) == 1), "the owner was not told the limit passed");
        assert!(rig.state.the_modal().is_none());
        rig.move_by_hand(0.9, 0.9);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN, BTN], "the window took no click after the limit");
    });
}

#[test]
fn escape_always_ends_a_modal() {
    alone("rig::tests::modal::escape_always_ends_a_modal", |dir| {
        let mut rig = Rig::new(dir);
        let (window, root, _modal) = modal_scene(&mut rig, 200, 40);
        rig.made_up_key(KEY_ESC);
        assert!(rig.state.the_modal().is_some(), "a made-up Escape ended the modal");
        rig.key_by_hand(KEY_ESC);
        assert!(rig.state.the_modal().is_none(), "Escape did not end the modal");
        assert_eq!(rig.closed(root), 1, "the owner was not told");
        assert!(!rig.state.only_the_person_answers());
        rig.move_by_hand(0.9, 0.9);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN, BTN]);
    });
}

#[test]
fn a_modal_asked_by_anyone_but_a_trusted_panel_is_refused() {
    alone("rig::tests::modal::a_modal_asked_by_anyone_but_a_trusted_panel_is_refused", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        let stranger = rig.join(ClientState::default());
        let boxed = rig.boxed();
        let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
        rig.panel_sized(stranger, crate::trusted::MODAL, On::Overlay, 1280, 800, KeyboardInteractivity::Exclusive);
        assert_eq!(rig.closed(stranger), 1, "a request from a program that is not trusted was not closed");
        assert!(rig.state.the_modal().is_none(), "a request from a program that is not trusted held the screen");

        assert!(rig.programs[boxed].program.shell.is_none(), "a program in a box was offered panels");
        assert!(rig.state.the_modal().is_none());

        rig.panel_sized(root, crate::trusted::MODAL, On::Top, 200, 40, KeyboardInteractivity::None);
        assert_eq!(rig.closed(root), 1);
        assert!(rig.state.the_modal().is_none());
        rig.move_by_hand(0.5, 0.5);
        rig.click_by_hand();
        assert_eq!(rig.clicked(window), [BTN], "a refused request still kept a click from the window");

        rig.state.lock_for_rig();
        rig.panel_sized(root, crate::trusted::MODAL, On::Overlay, 200, 40, KeyboardInteractivity::None);
        assert_eq!(rig.closed(root), 2, "a request was accepted under the lock screen");
    });
}

#[test]
fn a_second_modal_is_refused() {
    alone("rig::tests::modal::a_second_modal_is_refused", |dir| {
        let mut rig = Rig::new(dir);
        let (_window, root, modal) = modal_scene(&mut rig, 200, 40);
        let second = rig.join(ClientState { from_root: true, ..ClientState::default() });
        rig.panel_sized(second, crate::trusted::MODAL, On::Overlay, 300, 100, KeyboardInteractivity::Exclusive);
        assert_eq!(rig.closed(second), 1, "a second modal was not refused");
        assert_eq!(rig.closed(root), 0, "the first modal was closed by the second");
        assert!(rig.state.the_modal().is_some(), "the first modal no longer holds");

        rig.put_away(&modal);
        rig.crash(root);
        let third = rig.join(ClientState { from_root: true, ..ClientState::default() });
        rig.panel_sized(third, crate::trusted::MODAL, On::Overlay, 200, 40, KeyboardInteractivity::None);
        assert_eq!(rig.closed(third), 0, "a request after the first ended was refused");
    });
}
