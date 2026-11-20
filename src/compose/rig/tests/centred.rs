//! A new dialog or small window opens in the very middle of its screen's free room.

#[cfg(test)]
use crate::rig::*;
#[cfg(test)]
use smithay::{desktop::layer_map_for_output, utils::{Logical, Point, Rectangle}};

#[cfg(test)]
fn off_centre(rig: &Rig, who: usize, surface: &wayland_client::protocol::wl_surface::WlSurface, room: Rectangle<i32, Logical>) -> (i32, i32) {
    let window = rig.state.window_for_surface(&rig.server_side(who, surface)).expect("the window");
    let at = rig.state.space.element_location(&window).expect("placed");
    let size = smithay::desktop::space::SpaceElement::geometry(&window).size;
    ((at.x + size.w / 2) - (room.loc.x + room.size.w / 2), (at.y + size.h / 2) - (room.loc.y + room.size.h / 2))
}

#[cfg(test)]
fn free_room(rig: &Rig, output: &smithay::output::Output) -> Rectangle<i32, Logical> {
    let geo = rig.state.space.output_geometry(output).expect("on the space");
    let zone = layer_map_for_output(output).non_exclusive_zone();
    Rectangle::new(geo.loc + zone.loc, zone.size)
}

#[cfg(test)]
fn near(got: (i32, i32)) -> bool {
    got.0.abs() <= 1 && got.1.abs() <= 1
}

#[test]
fn small_windows_and_dialogs_open_in_the_middle() {
    alone("rig::tests::centred::small_windows_and_dialogs_open_in_the_middle", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let room = free_room(&rig, &rig.output.clone());
        let (page, page_top) = rig.window_and_top(who, (600, 400));
        assert!(near(off_centre(&rig, who, &page, room)), "a small new window is off the middle");
        let (other, _) = rig.window_and_top(who, (300, 200));
        assert!(near(off_centre(&rig, who, &other, room)), "a second small window stepped off the middle");
        let dialog = rig.dialog_of(who, &page_top, (200, 120));
        assert!(near(off_centre(&rig, who, &dialog, room)), "a dialog is off the middle: {:?}", off_centre(&rig, who, &dialog, room));
    });
}

#[test]
fn a_dialog_opens_in_the_middle_of_its_owners_screen() {
    alone("rig::tests::centred::a_dialog_opens_in_the_middle_of_its_owners_screen", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let second = rig.second_screen("second");
        let (page, page_top) = rig.window_and_top(who, (600, 400));
        let page_window = rig.state.window_for_surface(&rig.server_side(who, &page)).expect("the window");
        rig.state.space.map_element(page_window, (1300, 50), true);
        rig.pump();
        let dialog = rig.dialog_of(who, &page_top, (200, 120));
        let room = free_room(&rig, &second);
        assert!(near(off_centre(&rig, who, &dialog, room)), "a dialog left its owner's screen or the middle");
    });
}

#[test]
fn a_new_window_opens_on_the_screen_under_the_pointer() {
    alone("rig::tests::centred::a_new_window_opens_on_the_screen_under_the_pointer", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        let second = rig.second_screen("second");
        rig.state.pointer.set_location(Point::from((1900.0, 300.0)));
        let (page, _) = rig.window_and_top(who, (400, 300));
        let room = free_room(&rig, &second);
        assert!(near(off_centre(&rig, who, &page, room)), "a new window was not in the middle of the screen under the pointer");
    });
}
