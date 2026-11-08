//! Tests: sidebar.

#[cfg(test)]
use super::*;

#[test]
fn the_i_shuts_on_a_press_anywhere_outside_its_words() {
    let open = |s: &mut Settings| s.kit.info = Some(("page".to_string(), "What this page is for.".to_string(), THE_I));
    let mut s = showing(Box::new(panels::wallpaper::Wallpaper::made_up()), 900.0);
    shuts_like_every_other("Settings", UNDER_THE_I, &mut s, open, |s| s.kit.info.is_some(), |s| s.view(), |s, m| s.update(m), |s, i| s.intercept(i, false));
    let mut hosted = Hosted { s: showing(Box::new(panels::wallpaper::Wallpaper::made_up()), 900.0) };
    shuts_like_every_other("Settings in the corner", UNDER_THE_I, &mut hosted, |h| open(&mut h.s), |h| h.s.kit.info.is_some(), |h| h.view(),
        |h, m| h.update(m), |_, _| Some(Msg(Rc::new(|s: &mut Settings| { s.kit.info.take(); }))));
}

#[test]
fn the_sidebar_fits_720_points() {
    let s = settings_with_every_page();
    let ui = Ui::new(s.side_list(), Theme::new(false, "#fbc711"));

    let edge = theme::sheet().snug_padding;
    let h = ui.natural_height(SIDE_WIDTH - 2.0 * edge) + 2.0 * edge;
    assert!(h <= 720.0, "the sidebar is {h} points high");
}

#[test]
fn the_side_bar_has_a_line_between_every_pair_of_sections_in_every_look() {
    use crate::toolkit::theme::tests::{every_look, section_lines_hold};
    let shots = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from);
    for (name, look) in every_look() {
        let mut s = settings_with_every_page();
        s.tall = 720.0;
        let tree = column(vec![card(s.side_list()).snug().width(Length::Fixed(SIDE_WIDTH))]).padding(8.0);

        let sheet = theme::sheet();
        let search_box_ends = (8.0 + sheet.snug_padding + sheet.sidebar_room + sheet.entry_height).ceil() as u32 + 3;
        let file = format!("386-settings-{}.png", name.replace([' ', ','], "-").replace("--", "-"));
        section_lines_hold(&format!("Settings, {name}"), tree, &look, SIDE_WIDTH as u32 + 16, page_groups().len() - 1, search_box_ends, shots.as_deref().map(|d| (d, file.as_str())));
    }
}

#[test]
#[ignore = "makes pictures"]
fn pictures_of_the_side_bar() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap());
    let name = std::env::var("TRINDLE_SHOTS_NAME").unwrap_or_else(|_| "settings-side".into());
    for (look, light) in [("dark", false), ("light", true)] {
        let mut s = settings_with_every_page();
        s.tall = 720.0;
        let tree = column(vec![card(s.side_list()).snug().width(Length::Fixed(SIDE_WIDTH))]).padding(8.0);
        let mut ui = Ui::new(tree, Theme::new(light, "#fbc711"));
        let w = SIDE_WIDTH as u32 + 16;
        let h = ui.natural_height(w as f32).ceil() as u32;
        let mut pix = tiny_skia::Pixmap::new(w * 2, h * 2).unwrap();
        ui.paint(&mut pix.as_mut(), 2.0);
        pix.save_png(out.join(format!("{name}-{look}.png"))).unwrap();
    }
}

#[test]
fn the_sidebar_never_needs_its_scroll_bar_at_any_height() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut pages = panels::all();
    pages.retain(|p| !IN_APPS.contains(&p.id()));
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 0.0 };
    let edge = theme::sheet().snug_padding;
    for tall in (720..=1440).step_by(8) {
        s.tall = tall as f32;
        let ui = Ui::new(s.side_list(), Theme::new(false, "#fbc711"));
        let h = ui.natural_height(SIDE_WIDTH - 2.0 * edge) + 2.0 * edge;
        assert!(h <= s.tall, "in a window {tall} points high the sidebar is {h}");
    }
}

#[test]
fn a_second_with_nothing_new_asks_for_no_painting() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: panels::all(), current: 0, search: String::new(), tall: 0.0 };

    let follow = ["datetime", "plugins", "blocking", "power", "tailscale", "snapshots"];
    let mut quiet = 0;
    for i in 0..s.pages.len() {
        if follow.contains(&s.pages[i].id()) {
            continue;
        }
        s.current = i;
        assert!(!s.second(), "{} found something new with nothing to follow", s.pages[i].id());
        quiet += 1;
    }
    assert!(quiet >= 15, "only {quiet} pages were tried");
    s.current = 0;
    s.kit.want = Some("help".into());
    assert!(s.second(), "going to another page is something to show");
    assert_eq!(s.pages[s.current].id(), "help");
}
