// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

#[cfg(test)]
use crate::rig::*;

#[test]
fn a_made_up_click_or_key_never_counts_as_the_person_asking_for_the_ring() {
    alone("rig::tests::ring_opening::a_made_up_click_or_key_never_counts_as_the_person_asking_for_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        let ring = rig.kept_ring();

        rig.made_up_move(0.5, 0.0);
        rig.made_up_click();
        assert_eq!(rig.ran(), 0, "an edge click over a full-screen window ran the ring's command");

        rig.made_up_key(KEY_F5);
        assert_eq!(rig.ran(), 1);
        assert!(rig.state.ring.wished.is_none(), "a made-up key counted as the person asking for the ring");

        let panel = rig.panel(ring.who, "trindle-ring");
        assert_eq!(rig.hidden("trindle-ring"), [true], "a made-up key put the ring over a full-screen window");
        assert!(!rig.has_keyboard(ring.who));
        rig.put_away(&panel);

        rig.move_by_hand(0.5, 0.5);
        rig.move_by_hand(0.5, 0.0);
        rig.click_by_hand();
        assert_eq!(rig.ran(), 1);
        assert!(rig.state.ring.wished.is_none(), "a click over a full-screen window counted as the person asking for the ring");

        rig.key_by_hand(KEY_F5);
        assert_eq!(rig.ran(), 2);
        assert!(rig.state.ring.wished.is_some(), "the person's own key did not count");

        rig.show(ring.who, &panel, 200, 40);
        assert_eq!(rig.hidden("trindle-ring"), [false]);
        assert!(rig.has_keyboard(ring.who));
    });
}

#[test]
fn a_made_up_double_click_on_the_desktop_never_counts_as_the_person_asking_for_the_ring() {
    alone("rig::tests::ring_opening::a_made_up_double_click_on_the_desktop_never_counts_as_the_person_asking_for_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        rig.kept_ring();
        rig.made_up_move(0.5, 0.0);
        rig.made_up_click();
        assert_eq!(rig.ran(), 0, "one click ran the ring's command");
        rig.made_up_click();
        assert_eq!(rig.ran(), 1, "the double click did not run the ring's command");
        assert!(rig.state.ring.wished.is_none(), "a made-up click counted as the person asking for the ring");

        rig.time += 1000;
        rig.move_by_hand(0.5, 0.5);
        rig.made_up_move(0.5, 0.0);
        rig.click_by_hand();
        rig.click_by_hand();
        assert_eq!(rig.ran(), 2);
        assert!(rig.state.ring.wished.is_none(), "a click where a made-up mouse put the pointer counted as the person's");

        rig.time += 1000;
        rig.move_by_hand(0.5, 0.5);
        rig.move_by_hand(0.5, 0.0);
        rig.click_by_hand();
        rig.click_by_hand();
        assert_eq!(rig.ran(), 3);
        assert!(rig.state.ring.wished.take().is_some(), "the person's own edge double click did not count");
    });
}

#[test]
fn only_a_double_click_opens_the_ring() {
    alone("rig::tests::ring_opening::only_a_double_click_opens_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        rig.kept_ring();
        let spots = [(0.5, 0.5), (0.2, 0.5), (0.5, 0.0), (0.5, 0.004), (0.5, 0.9999), (0.9999, 0.5), (0.0, 0.5), (0.0, 0.0)];
        for (x, y) in spots {
            rig.time += 1000;
            rig.move_by_hand(0.5, 0.5);
            rig.move_by_hand(x, y);
            rig.state.ring.wished = None;
            rig.click_by_hand();
            assert_eq!(rig.ran(), 0, "one click at {x},{y} ran the ring's command");
            assert!(rig.state.ring.wished.is_none(), "one click at {x},{y} asked for the ring");
        }
        let mut ran = 0;
        for (x, y) in spots {
            rig.time += 1000;
            rig.move_by_hand(0.5, 0.5);
            rig.move_by_hand(x, y);
            rig.state.ring.wished = None;
            rig.click_by_hand();
            rig.click_by_hand();
            ran += 1;
            assert_eq!(rig.ran(), ran, "a double click at {x},{y} on the empty desktop did not run the ring's command");
            assert!(rig.state.ring.wished.is_some(), "the person's own double click at {x},{y} did not count");
        }
    });
}

#[test]
fn no_double_click_over_a_full_screen_window_opens_the_ring() {
    alone("rig::tests::ring_opening::no_double_click_over_a_full_screen_window_opens_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let window = rig.join(ClientState::default());
        rig.window(window, true);
        rig.kept_ring();
        for (x, y) in [(0.5, 0.5), (0.2, 0.5), (0.5, 0.0), (0.5, 0.004), (0.5, 0.9999), (0.9999, 0.5), (0.0, 0.5), (0.0, 0.0), (0.9999, 0.9999)] {
            rig.time += 1000;
            rig.move_by_hand(0.5, 0.5);
            rig.move_by_hand(x, y);
            rig.state.ring.wished = None;
            rig.click_by_hand();
            rig.click_by_hand();
            assert_eq!(rig.ran(), 0, "a double click at {x},{y} over a full-screen window ran the ring's command");
            assert!(rig.state.ring.wished.is_none(), "a double click at {x},{y} over a full-screen window asked for the ring");
        }

        rig.key_by_hand(KEY_F5);
        assert_eq!(rig.ran(), 1);
        assert!(rig.state.ring.wished.is_some());
    });
}

#[test]
fn no_window_hides_the_edge_and_a_double_click_there_works() {
    alone("rig::tests::ring_opening::no_window_hides_the_edge_and_a_double_click_there_works", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        rig.kept_ring();
        let who = rig.join(ClientState::default());
        let surface = rig.window(who, false);
        for (w, h) in [(1280, 800), (1000, 1000), (1400, 900)] {
            rig.show(who, &surface, w, h);
            let window = rig.state.space.elements().next().cloned().expect("the window");
            rig.state.space.map_element(window, (0, 0), false);
            let before = rig.ran();
            rig.time += 1000;
            rig.move_by_hand(0.5, 0.5);
            rig.move_by_hand(0.5, 0.9);
            rig.click_by_hand();
            rig.click_by_hand();
            assert_eq!(rig.ran(), before, "a double click inside a {w}x{h} window ran the ring's command");
            rig.time += 1000;
            rig.move_by_hand(0.5, 0.5);
            rig.move_by_hand(0.5, 0.9999);
            rig.state.ring.wished = None;
            rig.click_by_hand();
            assert_eq!(rig.ran(), before, "one click at the bottom edge ran the ring's command");
            rig.click_by_hand();
            assert!(rig.ran() > before, "a double click at the bottom edge over a {w}x{h} window did not run the ring's command");
            assert!(rig.state.ring.wished.is_some());
        }
    });
}

fn at(rig: &mut Rig, x: f64, y: f64) {
    rig.move_by_hand(0.5, 0.5);
    rig.move_by_hand(x / 1280.0, y / 800.0);
}

fn double(rig: &mut Rig, x: f64, y: f64) {
    at(rig, x, y);
    rig.click_by_hand();
    rig.click_by_hand();
}

#[test]
fn a_double_click_on_the_empty_desktop_opens_the_ring() {
    alone("rig::tests::ring_opening::a_double_click_on_the_empty_desktop_opens_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        rig.kept_ring();
        at(&mut rig, 640.0, 400.0);
        rig.click_by_hand();
        assert_eq!(rig.ran(), 0, "a single click ran the ring's command");
        rig.click_by_hand();
        assert_eq!(rig.ran(), 1, "a double click did not run the ring's command");
        assert!(rig.state.ring.wished.is_some(), "the person's own double click did not count");
        rig.click_by_hand();
        assert_eq!(rig.ran(), 1, "the click after a pair counted as the first of a pair and the second");
        rig.click_by_hand();
        assert_eq!(rig.ran(), 2, "a second pair did not run it");
        for (x, y) in [(10.0, 400.0), (1270.0, 790.0), (200.0, 100.0)] {
            double(&mut rig, x, y);
            let n = rig.ran();
            assert!(n >= 3, "a double click at {x},{y} did not run it");
            rig.time += 1000;
            rig.click_by_hand();
            rig.time += 1000;
            rig.click_by_hand();
            assert_eq!(rig.ran(), n, "a slow double click at {x},{y} ran it");
        }
        let n = rig.ran();
        at(&mut rig, 300.0, 300.0);
        rig.click_by_hand();
        at(&mut rig, 600.0, 300.0);
        rig.click_by_hand();
        assert_eq!(rig.ran(), n, "two clicks far apart ran it");
        rig.state.config.desktop_double_click = None;
        at(&mut rig, 300.0, 300.0);
        rig.time += 1000;
        rig.click_by_hand();
        rig.click_by_hand();
        assert_eq!(rig.ran(), n, "a double click ran something nobody bound it to");
    });
}

#[test]
fn a_double_click_or_tap_works_at_every_edge_and_corner_of_one_and_of_two_screens() {
    alone("rig::tests::ring_opening::a_double_click_or_tap_works_at_every_edge_and_corner_of_one_and_of_two_screens", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        rig.kept_ring();
        let mut names = vec![SCREEN.to_string()];
        for two in [false, true] {
            if two {
                let second = Output::new(
                    "rig2".into(),
                    PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
                );
                let mode = Mode { size: (1280, 800).into(), refresh: 60_000 };
                second.change_current_state(Some(mode), None, None, Some((1280, 0).into()));
                second.set_preferred(mode);
                rig.state.space.map_output(&second, (1280, 0));
                names.push("rig2".into());
            }
            {
                for name in &names {
                    for fx in [0.0, 0.5, 1.0] {
                        for fy in [0.0, 0.5, 1.0] {
                            let step = |rig: &mut Rig, x: f64, y: f64| {
                                let event = Absolute { time: rig.next_time(), x, y };
                                rig.state.process_input_event_windowed(InputEvent::<Virtual>::PointerMotionAbsolute { event }, name);
                                rig.pump();
                            };
                            rig.time += 1000;
                            step(&mut rig, 0.5, 0.5);
                            step(&mut rig, fx, fy);

                            rig.state.ring.wished = None;
                            rig.click_by_hand();
                            assert!(rig.state.ring.wished.is_none(), "one click at {name} {fx},{fy} asked for the ring");
                            rig.click_by_hand();
                            assert!(rig.state.ring.wished.is_some(), "{} screens: a double click at {name} {fx},{fy} did not ask for the ring", names.len());

                            rig.time += 1000;
                            let spot = rig.state.pointer.current_location();
                            let t = rig.time;
                            rig.state.ring.wished = None;
                            rig.state.ring.by_hand = true;
                            rig.state.tap_desktop(spot, t, false);
                            assert!(rig.state.ring.wished.is_none(), "one tap at {name} {fx},{fy} asked for the ring");
                            rig.state.tap_desktop(spot, t + 100, false);
                            rig.state.ring.by_hand = false;
                            assert!(rig.state.ring.wished.is_some(), "{} screens: a double tap at {name} {fx},{fy} did not ask for the ring", names.len());
                        }
                    }
                }
            }
        }
    });
}

#[test]
fn a_double_click_on_a_window_or_its_border_does_not_open_the_ring() {
    alone("rig::tests::ring_opening::a_double_click_on_a_window_or_its_border_does_not_open_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        rig.kept_ring();
        let w = rig.state.space.elements().last().cloned().expect("a window is open");
        let g = rig.state.space.element_geometry(&w).expect("the window has a place");
        let (cx, cy) = ((g.loc.x + g.size.w / 2) as f64, (g.loc.y + g.size.h / 2) as f64);
        for (x, y) in [(cx, cy), (g.loc.x as f64 - 1.0, cy), (cx, g.loc.y as f64 - 1.0),
                       ((g.loc.x + g.size.w) as f64 + 1.0, cy), (cx, (g.loc.y + g.size.h) as f64 + 1.0)] {
            rig.time += 1000;
            double(&mut rig, x, y);
            assert_eq!(rig.ran(), 0, "a double click at {x},{y} on a window or its border ran the ring's command");
        }

        at(&mut rig, 1000.0, 600.0);
        rig.click_by_hand();
        at(&mut rig, cx, cy);
        rig.click_by_hand();
        at(&mut rig, 1000.0, 600.0);
        rig.click_by_hand();
        assert_eq!(rig.ran(), 0, "a click on a window did not break the pair");
    });
}

#[test]
fn a_click_always_brings_the_window_behind_to_the_front() {
    alone("rig::tests::ring_opening::a_click_always_brings_the_window_behind_to_the_front", |dir| {
        use crate::shell::place_keeping_order;
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        rig.window(who, false);
        let back = rig.state.space.elements().next().cloned().expect("a window");
        let front = rig.state.space.elements().last().cloned().expect("a window");
        assert_ne!(back, front);
        place_keeping_order(&mut rig.state.space, back.clone(), (20, 60).into());
        place_keeping_order(&mut rig.state.space, front.clone(), (400, 300).into());
        assert_eq!(rig.state.space.elements().last(), Some(&front), "moving a window changed what was in front");

        place_keeping_order(&mut rig.state.space, back.clone(), (24, 64).into());
        assert_eq!(rig.state.space.elements().last(), Some(&front), "a window behind moved itself to the front");
        let g = rig.state.space.element_geometry(&back).expect("a place");
        at(&mut rig, (g.loc.x + 5) as f64, (g.loc.y + 5) as f64);
        rig.click_by_hand();
        assert_eq!(rig.state.space.elements().last(), Some(&back), "the click did not bring the window behind to the front");
    });
}

#[test]
fn a_click_at_the_very_edge_raises_no_window() {
    alone("rig::tests::ring_opening::a_click_at_the_very_edge_raises_no_window", |dir| {
        use crate::shell::place_keeping_order;
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let big = rig.window(who, false);
        rig.show(who, &big, 1280, 800);
        let small = rig.window(who, false);
        let back = rig.state.space.elements().next().cloned().expect("a window");
        let front = rig.state.space.elements().last().cloned().expect("a window");
        assert_ne!(back, front);
        let _ = small;
        place_keeping_order(&mut rig.state.space, back.clone(), (0, 0).into());
        place_keeping_order(&mut rig.state.space, front.clone(), (400, 300).into());
        at(&mut rig, 100.0, 799.5);
        assert!(rig.state.space.element_under(smithay::utils::Point::<f64, smithay::utils::Logical>::from((100.0, 799.5))).is_some_and(|(w, _)| *w == back), "the big window is not under the edge");
        rig.click_by_hand();
        assert_eq!(rig.state.space.elements().last(), Some(&front), "a click at the very edge brought the window under it to the front");
    });
}

#[test]
fn a_full_screen_window_behind_stays_behind_at_the_edge_and_comes_up_on_a_click() {
    alone("rig::tests::ring_opening::a_full_screen_window_behind_stays_behind_at_the_edge_and_comes_up_on_a_click", |dir| {
        use crate::shell::place_keeping_order;
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.window(who, true);
        let small = rig.window(who, false);
        let _ = small;
        let full = rig.state.space.elements().next().cloned().expect("a window");
        let front = rig.state.space.elements().last().cloned().expect("a window");
        assert_ne!(full, front);
        place_keeping_order(&mut rig.state.space, full.clone(), (0, 0).into());
        place_keeping_order(&mut rig.state.space, front.clone(), (400, 300).into());
        rig.state.focus_window(&front);
        at(&mut rig, 100.0, 799.5);
        rig.click_by_hand();
        rig.click_by_hand();
        assert_eq!(rig.state.space.elements().last(), Some(&front), "a double click at the edge brought the full-screen window behind to the front");
        at(&mut rig, 50.0, 50.0);
        rig.click_by_hand();
        assert_eq!(rig.state.space.elements().last(), Some(&full), "a click did not bring the full-screen window behind to the front");
    });
}

#[test]
fn a_press_on_every_side_raises_no_maximised_window() {
    alone("rig::tests::ring_opening::a_press_on_every_side_raises_no_maximised_window", |dir| {
        use crate::shell::place_keeping_order;
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let big = rig.window(who, false);
        rig.show(who, &big, 1280, 800);
        rig.window(who, false);
        let back = rig.state.space.elements().next().cloned().expect("a window");
        let front = rig.state.space.elements().last().cloned().expect("a window");
        assert_ne!(back, front);
        place_keeping_order(&mut rig.state.space, back.clone(), (0, 0).into());
        place_keeping_order(&mut rig.state.space, front.clone(), (400, 300).into());
        for (x, y) in [(0.0, 400.0), (0.5, 400.0), (1279.5, 400.0), (640.0, 0.0), (640.0, 0.5), (640.0, 799.5), (0.0, 0.0), (1279.5, 799.5)] {
            at(&mut rig, x, y);
            rig.click_by_hand();
            assert_eq!(rig.state.space.elements().last(), Some(&front), "a press at {x},{y} brought the window behind to the front");
        }
    });
}

#[test]
fn a_window_redrawing_after_losing_the_keyboard_stays_behind() {
    alone("rig::tests::ring_opening::a_window_redrawing_after_losing_the_keyboard_stays_behind", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let s1 = rig.window(who, false);
        let s2 = rig.window(who, false);
        let first = rig.state.space.elements().next().cloned().expect("a window");
        let g = rig.state.space.element_geometry(&first).expect("a place");
        at(&mut rig, (g.loc.x + 5) as f64, (g.loc.y + 5) as f64);
        rig.click_by_hand();
        assert_eq!(rig.state.space.elements().last(), Some(&first), "the click did not raise the window");
        for (w, h) in [(310, 210), (290, 190), (300, 200), (640, 480)] {
            rig.show(who, &s2, w, h);
            assert_eq!(rig.state.space.elements().last(), Some(&first), "a window behind came to the front when it redrew at {w}x{h}");
        }
        let _ = s1;
    });
}

#[test]
fn a_double_click_over_a_full_screen_window_does_not_open_the_ring() {
    alone("rig::tests::ring_opening::a_double_click_over_a_full_screen_window_does_not_open_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let who = rig.join(ClientState::default());
        rig.window(who, true);
        rig.kept_ring();
        for (x, y) in [(640.0, 400.0), (2.0, 2.0), (1278.0, 798.0), (5.0, 400.0)] {
            rig.time += 1000;
            double(&mut rig, x, y);
            assert_eq!(rig.ran(), 0, "a double click at {x},{y} over a full-screen window ran the ring's command");
        }
    });
}

#[test]
fn a_double_tap_on_the_empty_desktop_opens_the_ring() {
    alone("rig::tests::ring_opening::a_double_tap_on_the_empty_desktop_opens_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        rig.kept_ring();
        let w = rig.state.space.elements().last().cloned().expect("a window is open");
        let g = rig.state.space.element_geometry(&w).expect("the window has a place");
        let on_window = ((g.loc.x + g.size.w / 2) as f64, (g.loc.y + g.size.h / 2) as f64);

        at(&mut rig, 5.0, 5.0);
        let empty = (5.0, 5.0).into();
        let t = rig.time;
        rig.state.tap_desktop(empty, t + 10, false);
        assert_eq!(rig.ran(), 0, "one tap ran the ring's command");
        rig.state.tap_desktop((12.0, 14.0).into(), t + 200, false);
        assert_eq!(rig.ran(), 1, "a double tap did not run the ring's command");
        rig.state.tap_desktop(empty, t + 2000, false);
        rig.state.tap_desktop(empty, t + 4000, false);
        assert_eq!(rig.ran(), 1, "a slow double tap ran it");
        rig.state.tap_desktop(empty, t + 6000, false);
        rig.state.tap_desktop((300.0, 5.0).into(), t + 6100, false);
        assert_eq!(rig.ran(), 1, "two taps far apart ran it");
        rig.state.tap_desktop(on_window.into(), t + 8000, false);
        rig.state.tap_desktop(on_window.into(), t + 8100, false);
        assert_eq!(rig.ran(), 1, "a double tap on a window ran it");
        rig.state.tap_desktop(empty, t + 10000, true);
        rig.state.tap_desktop(empty, t + 10100, true);
        assert_eq!(rig.ran(), 1, "a double tap held by a grab ran it");
    });
}

#[test]
fn a_double_tap_over_a_full_screen_window_does_not_open_the_ring() {
    alone("rig::tests::ring_opening::a_double_tap_over_a_full_screen_window_does_not_open_the_ring", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let who = rig.join(ClientState::default());
        rig.window(who, true);
        rig.kept_ring();
        at(&mut rig, 5.0, 5.0);
        let t = rig.time;
        rig.state.tap_desktop((5.0, 5.0).into(), t + 10, false);
        rig.state.tap_desktop((5.0, 5.0).into(), t + 100, false);
        assert_eq!(rig.ran(), 0, "a double tap over a full-screen window ran the ring's command");
    });
}

#[test]
fn opening_and_closing_the_ring_leaves_the_windows_and_the_keyboard_as_they_were() {
    alone("rig::tests::ring_opening::opening_and_closing_the_ring_leaves_the_windows_and_the_keyboard_as_they_were", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.state.config.desktop_double_click = Some("trindle-ring".into());
        let ring = rig.kept_ring();
        let (a, b) = (rig.join(ClientState::default()), rig.join(ClientState::default()));
        rig.window(a, false);
        rig.window(b, false);
        let order = |rig: &Rig| rig.state.space.elements().cloned().collect::<Vec<_>>();
        let low = order(&rig).into_iter().next().expect("the lower window");
        let serial = smithay::utils::SERIAL_COUNTER.next_serial();
        let keyboard = rig.state.seat.get_keyboard().unwrap();
        keyboard.set_focus(&mut rig.state, Some(low.clone().into()), serial);
        rig.pump();
        let stacking = order(&rig);
        assert!(rig.has_keyboard(a) && !rig.has_keyboard(b), "the lower window should hold the keyboard to begin with");
        let panel = rig.panel(ring.who, "trindle-ring");
        rig.time += 1000;
        rig.move_by_hand(0.5, 0.5);
        rig.move_by_hand(0.9999, 0.9999);
        rig.click_by_hand();
        rig.click_by_hand();
        rig.show(ring.who, &panel, 200, 40);
        assert!(rig.has_keyboard(ring.who), "the ring did not take the keyboard");
        rig.put_away(&panel);
        assert!(order(&rig) == stacking, "opening the ring changed which window is in front");
        assert!(rig.has_keyboard(a) && !rig.has_keyboard(b), "the keyboard did not go back to the window that had it");
    });
}
