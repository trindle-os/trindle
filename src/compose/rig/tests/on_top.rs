//! A browser's picture-in-picture window stays over the window the person clicks.

#[cfg(test)]
use crate::rig::*;

#[test]
fn picture_in_picture_stays_over_a_clicked_window() {
    alone("rig::tests::on_top::picture_in_picture_stays_over_a_clicked_window", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let (page, _) = rig.window_and_top(who, (600, 400));
        let (_, video) = rig.window_and_top(who, (300, 200));
        video.set_title("Picture in picture".into());
        video.set_app_id("firefox".into());
        rig.pump();
        let page = rig.server_side(who, &page);
        let page = rig.state.window_for_surface(&page).expect("the page");
        rig.state.focus_window(&page);
        let top = rig.state.space.elements().last().cloned().expect("a window");
        assert!(top.stays_on_top(), "the clicked page went over the video");
        assert!(page.is_active(), "the clicked page did not get the keyboard");

        let (_, other) = rig.window_and_top(who, (300, 200));
        other.set_title("Notes".into());
        rig.pump();
        let top = rig.state.space.elements().last().cloned().expect("a window");
        assert!(top.stays_on_top(), "a new window went over the video");
    });
}

#[test]
fn only_a_browser_can_float_a_window_over_the_rest() {
    alone("rig::tests::on_top::only_a_browser_can_float_a_window_over_the_rest", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let (page, _) = rig.window_and_top(who, (600, 400));
        let (_, fake) = rig.window_and_top(who, (300, 200));
        fake.set_title("Picture in picture".into());
        fake.set_app_id("org.example.Notes".into());
        rig.pump();
        let page = rig.server_side(who, &page);
        let page = rig.state.window_for_surface(&page).expect("the page");
        rig.state.focus_window(&page);
        let top = rig.state.space.elements().last().cloned().expect("a window");
        assert!(!top.stays_on_top(), "a program that is not a browser floated over the clicked window");
        assert!(page.is_active());
    });
}
