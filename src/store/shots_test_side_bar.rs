//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn the_side_bar_fits_without_scrolling_at_every_height() {
    for tall in [560u32, 600, 692, 720, 800, 1000, 1400] {
        let mut s = stocked();
        s.across = 1278.0;
        s.tall = tall as f32;
        let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
        ui.lay_at(1278, tall);
        let side = ui.places().into_iter().find(|p| p.0 == "side").expect("the side bar scrolls by name");
        assert_eq!(side.2, 0.0, "at {tall} high the side bar has {} points out of sight", side.2);
    }
    let room = crate::toolkit::theme::sheet().sidebar_room;
    assert_eq!(super::side::side_fit(1400.0, 24, 4, room), (super::side::SIDE_ROW, super::side::SIDE_GAP, super::side::SIDE_HEAD_GAP), "roomy: the usual sizes");
    let (line, gap, _) = super::side::side_fit(600.0, 24, 4, room);
    assert!((super::side::SIDE_ROW_LEAST..super::side::SIDE_ROW).contains(&line) && gap >= super::side::SIDE_GAP_LEAST, "{line} {gap}");
}

#[test]
#[ignore = "makes pictures"]
fn pictures_of_the_side_bar() {
    let name = std::env::var("TRINDLE_SHOTS_NAME").unwrap_or_else(|_| "apps-side".into());
    for (look, light) in [("dark", false), ("light", true)] {
        let mut s = stocked();
        s.across = 1278.0;
        s.tall = 900.0;
        let tree = crate::toolkit::widget::column(vec![crate::toolkit::widget::card(s.side(settings())).snug().width(Length::Fixed(SIDE + 24.0))]).padding(8.0);
        let mut ui = Ui::new(tree, Theme::new(light, "#fbc711"));
        let w = SIDE as u32 + 24 + 16;
        let h = ui.natural_height(w as f32).ceil() as u32;
        let mut pix = tiny_skia::Pixmap::new(w * 2, h * 2).unwrap();
        ui.paint(&mut pix.as_mut(), 2.0);
        pix.save_png(folder().join(format!("{name}-{look}.png"))).unwrap();
    }
}

#[test]
fn the_side_bar_has_a_line_between_every_pair_of_sections_in_every_look() {
    use crate::toolkit::theme::tests::{every_look, section_lines_hold};
    let shots = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from);
    for (name, look) in every_look() {
        let mut s = stocked();
        s.across = 1278.0;
        s.tall = 900.0;
        let tree = crate::toolkit::widget::column(vec![crate::toolkit::widget::card(s.side(settings())).snug().width(Length::Fixed(SIDE + 24.0))]).padding(8.0);
        let file = format!("386-apps-{}.png", name.replace([' ', ','], "-").replace("--", "-"));
        section_lines_hold(&format!("Apps, {name}"), tree, &look, SIDE as u32 + 24 + 16, 3, 0, shots.as_deref().map(|d| (d, file.as_str())));
    }
}
