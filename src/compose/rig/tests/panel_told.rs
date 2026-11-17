//! The panel's window list is looked at only when a window changes, and not on every turn of the loop.

#[cfg(test)]
use crate::rig::*;

#[test]
fn a_turn_that_changes_no_window_looks_at_no_window() {
    alone("rig::tests::panel_told::a_turn_that_changes_no_window_looks_at_no_window", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        rig.state.refresh_panel();
        assert_eq!(rig.state.panel_told.len(), 1, "the first window was not listed");

        rig.state.panel_told.clear();
        rig.state.toplevel_list.clear();
        for _ in 0..3 {
            rig.pump();
            rig.state.refresh_panel();
        }
        assert!(rig.state.panel_told.is_empty(), "a turn with no change looked at the windows");
        assert!(rig.state.toplevel_list.is_empty(), "a turn with no change rebuilt the list");
        rig.window(who, false);
        rig.state.refresh_panel();
        assert_eq!(rig.state.panel_told.len(), 2, "a new window was not looked at");
        assert_eq!(rig.state.toplevel_list.len(), 2, "a new window did not rebuild the list");
    });
}

#[test]
fn every_change_to_a_window_reaches_the_panel() {
    alone("rig::tests::panel_told::every_change_to_a_window_reaches_the_panel", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let (surface, top) = rig.window_and_top(who, (300, 200));
        let told = |rig: &mut Rig| {
            rig.pump();
            rig.state.refresh_panel();
            rig.state.panel_told.clone()
        };
        assert_eq!(told(&mut rig).len(), 1);

        top.set_title("Letter to John Smith".into());
        assert_eq!(told(&mut rig)[0].title, "Letter to John Smith", "a new title");
        top.set_app_id("org.example.Letters".into());
        assert_eq!(told(&mut rig)[0].app_id, "org.example.Letters", "a new program id");

        top.set_maximized();
        rig.pump();
        rig.show(who, &surface, 1280, 800);
        assert!(told(&mut rig)[0].maximized, "maximised");
        top.unset_maximized();
        rig.pump();
        rig.show(who, &surface, 300, 200);
        assert!(!told(&mut rig)[0].maximized, "maximised no more");

        let (_, second) = rig.window_and_top(who, (300, 200));
        let windows = told(&mut rig);
        assert_eq!(windows.len(), 2);
        assert!(windows[1].active && !windows[0].active, "the newer window is the active one");
        let first = rig.server_side(who, &surface);
        let element = rig.state.window_for_surface(&first).expect("the first window");
        rig.state.space.raise_element(&element, true);
        let windows = told(&mut rig);
        assert!(windows[1].active && !windows[0].active, "the raised window is the active one");
        assert_eq!(windows[1].surface, first);

        rig.state.set_window_minimized(&first, true);
        assert!(told(&mut rig).iter().any(|w| w.surface == first && w.minimized), "minimised");
        rig.state.set_window_minimized(&first, false);
        assert!(told(&mut rig).iter().any(|w| w.surface == first && !w.minimized), "restored");

        let screen = rig.second_screen("RIG-2");
        rig.state.space.map_element(element, (1400, 100), false);
        assert!(told(&mut rig).iter().any(|w| w.surface == first && w.outputs.contains(&screen)), "moved to the second screen");

        second.destroy();
        assert_eq!(told(&mut rig).len(), 1, "a closed window stayed listed");
    });
}
