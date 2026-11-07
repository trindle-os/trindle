//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn settings_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    let events = Loop::new();
    drawn::none(drawn::in_every_look("Settings", 400, || {
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: made_up_pages(), current: 0, search: String::new(), tall: 1500.0 };
        let mut pix = tiny_skia::Pixmap::new(1278, 1500).unwrap();
        for i in 0..s.pages.len() {
            s.current = i;
            s.kit.current = s.pages[i].id().to_string();
            s.kit.icon = s.pages[i].icon();
            drawn::page(s.pages[i].id());
            let mut ui = Ui::new(s.view(), Theme::chosen());
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }));
}

#[test]
#[ignore]
fn shoot_the_wallpaper_page() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let prefix = std::env::var("TRINDLE_SHOT_PREFIX").unwrap_or_else(|_| "wallpaper-".into());
    std::fs::create_dir_all(&out).unwrap();
    for (w, h, name) in [(1280u32, 900u32, "1280"), (1920, 1080, "1920"), (NARROWEST, 900, "narrowest")] {
        let s = showing(Box::new(panels::wallpaper::Wallpaper::made_up()), h as f32);
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{prefix}{name}.png"))).unwrap();
    }
}

#[test]
fn a_change_on_trial_is_asked_in_a_foot_that_never_scrolls_away() {
    use ui::{TRIAL_BACK, TRIAL_FOOT, TRIAL_KEEP};
    let (w, h) = (1100u32, 700u32);
    for id in ["appearance", "display"] {
        let (plain, trying) = on_trial(id);
        let (plain, trying) = (showing(plain, h as f32), showing(trying, h as f32));
        let name = format!("page:{}", plain.pages[0].title());
        let at = |ui: &Ui<Msg>, key: &str| ui.boxes().into_iter().find(|b| b.0 == key).map(|b| (b.1, b.2));
        let place = |ui: &Ui<Msg>| ui.places().into_iter().find(|p| p.0 == name).map(|p| (p.1, p.2)).unwrap();
        let mut ui = Ui::new(plain.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(w, h);
        assert!(at(&ui, TRIAL_FOOT).is_none() && at(&ui, TRIAL_BACK).is_none(), "{id}");
        assert_eq!(at(&ui, &name).unwrap().0.h, h as f32, "{id}");
        let most = place(&ui).1;
        assert!(most > 200.0, "{id}: the page is long enough to scroll, {most}");
        let foot = if id == "appearance" { crate::toolkit::theme::safe_sheet().trial_foot } else { crate::toolkit::theme::sheet().trial_foot };
        for (what, down) in [("at its top", 0.0), ("part way down", 150.0), ("at its end", most)] {
            ui.set_view(plain.view());
            ui.lay_at(w, h);
            ui.scroll_area(&name, down);
            assert_eq!(place(&ui).0, down, "{id} {what}");
            ui.set_view(trying.view());
            ui.lay_at(w, h);
            let (rect, clip) = at(&ui, TRIAL_FOOT).unwrap_or_else(|| panic!("{id} {what}: no foot"));
            assert_eq!((rect.x, rect.y, rect.w, rect.h), (SIDE_WIDTH, h as f32 - foot, w as f32 - SIDE_WIDTH, foot), "{id} {what}: the foot's place");
            assert!(clip.is_none(), "{id} {what}: nothing cuts the foot short");
            let page = at(&ui, &name).unwrap().0;
            assert_eq!((page.y, page.h), (0.0, h as f32 - foot), "{id} {what}: the page ends where the foot begins");
            assert_eq!(place(&ui), (down, most + foot), "{id} {what}: the page kept its place, and its end can still be reached");
            for key in [TRIAL_BACK, TRIAL_KEEP] {
                let (b, clip) = at(&ui, key).unwrap_or_else(|| panic!("{id} {what}: no {key}"));
                assert!(clip.is_none() && b.x >= rect.x && b.x + b.w <= rect.x + rect.w && b.y >= rect.y && b.y + b.h <= rect.y + rect.h,
                        "{id} {what}: {key} is whole and inside the foot, {b:?}");
                assert!(b.h >= 28.0 && b.w >= 44.0, "{id} {what}: {key} can be pressed");
            }
            if down == 0.0 {
                assert_eq!(ui.keyboard_on().as_deref(), Some(TRIAL_BACK), "{id} {what}");
            }
            ui.scroll_area(&name, f32::MAX);
            assert_eq!(place(&ui).0, most + foot, "{id} {what}");
            assert_eq!(at(&ui, TRIAL_FOOT).unwrap().0.y, h as f32 - foot, "{id} {what}: the foot stays where it is");
            ui.scroll_area(&name, down);
            ui.set_view(plain.view());
            ui.lay_at(w, h);
            assert!(at(&ui, TRIAL_FOOT).is_none() && at(&ui, TRIAL_KEEP).is_none(), "{id} {what}: the foot has gone");
            assert_eq!(at(&ui, &name).unwrap().0.h, h as f32, "{id} {what}");
            assert_eq!(place(&ui), (down, most), "{id} {what}: the page kept its place");
        }
        the_foot_is_painted_across(&mut ui, &trying, id, (w, h), foot);
    }
}

fn the_foot_is_painted_across(ui: &mut Ui<Msg>, trying: &Settings, id: &str, (w, h): (u32, u32), foot: f32) {
    ui.set_view(trying.view());
    let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let look = if id == "appearance" { crate::toolkit::theme::safe() } else { Theme::new(false, "#fbc711") };
    let is = |x: u32, y: u32, c: crate::toolkit::colour::Colour| {
        let p = pix.pixel(x, y).unwrap();
        (p.red(), p.green(), p.blue()) == ((c.r * 255.0).round() as u8, (c.g * 255.0).round() as u8, (c.b * 255.0).round() as u8)
    };
    for x in [SIDE_WIDTH as u32 + 2, w - 2] {
        assert!(is(x, h - 3, look.trial_foot), "{id}: the foot's surface reaches {x}");
        assert!(is(x, h - foot as u32, look.trial_foot_line), "{id}: the line along the foot's top reaches {x}");
    }
    assert!(!is(SIDE_WIDTH as u32 - 4, h - 3, look.trial_foot), "{id}: the foot is beside the side bar, not over it");
}

#[test]
#[ignore]
fn shoot_the_trial_foot() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    std::fs::create_dir_all(&out).unwrap();
    let (w, h) = (1278u32, 692u32);
    for (id, down, file) in [("appearance", 0.0, "trial-footer-top"), ("appearance", f32::MAX, "trial-footer-scrolled"), ("display", 300.0, "trial-footer-display")] {
        let s = showing(on_trial(id).1, h as f32);
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(w, h);
        ui.scroll_area(&format!("page:{}", s.pages[0].title()), down);
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{file}.png"))).unwrap();
    }
}

#[test]
fn no_page_has_a_loose_tile() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut pages = panels::all();
    pages.extend([
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ]);
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 0.0 };
    let fonts = crate::toolkit::text::Fonts::new();
    let mut loose = Vec::new();
    for i in 0..s.pages.len() {
        s.current = i;
        let tree = s.view();
        for w in [1280.0, 900.0, 400.0] {
            for (path, spare) in crate::toolkit::widget::loose_tiles(&fonts, &tree, w, 800.0) {
                loose.push(format!("{} at {w}: tile {path:?} has {spare} spare", s.pages[i].id()));
            }
        }
    }
    assert!(loose.is_empty(), "{}", loose.join("\n"));
}
