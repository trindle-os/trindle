//! Programs and screens that ask for what the compositor never expects: it carries on.

#[cfg(test)]
use crate::rig::*;

#[test]
fn a_panel_with_no_screen_to_go_on_is_closed() {
    alone("rig::tests::bad_input::a_panel_with_no_screen_to_go_on_is_closed", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let output = rig.output.clone();
        rig.state.space.unmap_output(&output);
        let p = &rig.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, None, On::Top, "a-panel".into(), &qh, ());
        surface.commit();
        rig.pump();
        assert_eq!(rig.closed(who), 1, "the panel with no screen was not closed");
        rig.state.space.map_output(&output, (0, 0));
        rig.panel(who, "another-panel");
        assert_eq!(rig.closed(who), 1, "a panel with a screen to go on was closed");
    });
}

#[test]
fn a_screen_with_no_mode_is_passed_over_by_the_pointer() {
    alone("rig::tests::bad_input::a_screen_with_no_mode_is_passed_over_by_the_pointer", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        let first = rig.output.clone();
        rig.state.space.unmap_output(&first);
        let blank = Output::new(
            "blank".into(),
            PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
        );
        rig.state.space.map_output(&blank, (0, 0));
        rig.state.space.map_output(&first, (0, 0));
        rig.onto_the_window_in_front();
        rig.click_by_hand();
        assert!(rig.has_keyboard(who), "the click did not reach the window");
    });
}

#[test]
fn real_input_switches_the_screens_back_on() {
    alone("rig::tests::bad_input::real_input_switches_the_screens_back_on", |dir| {
        let mut rig = Rig::new(dir);
        rig.state.backend_data.lit = false;
        rig.move_by_hand(0.4, 0.4);
        assert!(rig.state.backend_data.lit, "the person's move left the screens black");
    });
}

#[test]
fn a_press_on_one_programs_window_moves_no_other_programs_window() {
    alone("rig::tests::bad_input::a_press_on_one_programs_window_moves_no_other_programs_window", |dir| {
        use crate::focus::PointerFocusTarget;
        let mut rig = Rig::new(dir);
        let (a, b) = (rig.join(ClientState::default()), rig.join(ClientState::default()));
        let (wa, wa2, wb) = (rig.window(a, false), rig.window(a, false), rig.window(b, false));
        let (sa, sa2, sb) = (rig.server_side(a, &wa), rig.server_side(a, &wa2), rig.server_side(b, &wb));
        let press = (PointerFocusTarget::WlSurface(sa.clone()), (1.0, 1.0).into());
        assert!(crate::shell::press_of_its_own(Some(&press), &sa2));
        assert!(!crate::shell::press_of_its_own(Some(&press), &sb), "another program's press moved a window");
        assert!(!crate::shell::press_of_its_own::<PointerFocusTarget>(None, &sa));
        assert!(crate::shell::press_began_on(Some(&press), Some(&sa)));
        assert!(!crate::shell::press_began_on(Some(&press), Some(&sa2)), "a press on one window moved another");
        assert!(!crate::shell::press_began_on(Some(&press), None));
    });
}

#[test]
fn a_screen_copy_asked_for_wrongly_cuts_off_only_that_program() {
    alone("rig::tests::bad_input::a_screen_copy_asked_for_wrongly_cuts_off_only_that_program", |dir| {
        use wayland_protocols::ext::image_copy_capture::v1::client::ext_image_copy_capture_manager_v1::Options;
        let mut rig = Rig::new(dir);
        let bystander = rig.join(ClientState::default());
        type Misuse = fn(&wayland_protocols::ext::image_copy_capture::v1::client::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1, &QueueHandle<Program>);
        let misuses: [(&str, Misuse); 3] = [
            ("a second frame while one is open", |s, qh| { s.create_frame(qh, ()); s.create_frame(qh, ()); }),
            ("a copy with no buffer", |s, qh| s.create_frame(qh, ()).capture()),
            ("damage of no size", |s, qh| s.create_frame(qh, ()).damage_buffer(0, 0, 0, -5)),
        ];
        for (what, misuse) in misuses {
            let who = rig.join(ClientState::default());
            let (program, qh) = rig.program(who);
            let source = program.screen_sources.as_ref().expect("screen copies are offered").create_source(&program.outputs[0], &qh, ());
            let session = program.copier.as_ref().expect("screen copies are offered").create_session(&source, Options::empty(), &qh, ());
            rig.pump();
            misuse(&session, &qh);
            rig.pump();
            assert!(rig.cut_off(who), "{what} was taken");
            assert!(!rig.cut_off(bystander), "{what} cut off another program");
        }
        rig.window(bystander, false);
        rig.onto_the_window_in_front();
        assert!(rig.has_keyboard(bystander), "the desktop stopped after a program misused a screen copy");
    });
}
