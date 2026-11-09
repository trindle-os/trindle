//! Tests: layout.

#[cfg(test)]
use super::*;

#[test]
fn the_i_shuts_on_a_press_anywhere_outside_its_words() {
    let mut d = sample();
    shuts_like_every_other("Drives", UNDER_THE_I, &mut d,
        |d| { d.help = true; d.help_at = THE_I; },
        |d| d.help, |d| d.view(), |d, m| d.update(m), |d, i| d.intercept(i, false));
}

#[test]
fn room_is_judged_by_what_is_left_as_well() {
    let g = GB as u64;
    assert!(headline(&[], Some((g, g * 4 / 10))).0.starts_with("Nearly full"));
    assert!(headline(&[], Some((3 * g, 2 * g))).0.starts_with("Getting full"));
    assert!(headline(&[], Some((100 * g, 400 * g))).0.starts_with("Plenty of room"));
    assert!(headline(&[], Some((960 * g, 40 * g))).0.starts_with("Nearly full"));
}

#[test]
fn drives_follows_the_language_and_falls_back_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let french = "disks-map-caption = Chaque partie de chaque disque, remplie selon son usage.\n\
                  disks-formatted = Format\u{e9}\n\
                  disks-health-hours =\n    { $n ->\n        [one] utilis\u{e9} pendant { $hours } heure\n       *[other] utilis\u{e9} pendant { $hours } heures\n    }\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        assert_eq!(map_caption(), "Chaque partie de chaque disque, remplie selon son usage.");
        assert_eq!(t!("disks-formatted"), "Format\u{e9}");

        let h = Health { kelvin: 311.15, hours: 2140, ..Default::default() };
        assert_eq!(plan::health_line(&h), ("Healthy, 38 \u{b0}C, utilis\u{e9} pendant 2,140 heures".to_string(), true));
        assert_eq!(plan::kinds()[1].for_what, "Linux only");
    });
    assert_eq!(map_caption(), "Each part of each drive, filled as far as it is used.");
}

#[test]
fn the_empty_window_fits() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let d = Drives::new(&events, asks);
    let h = Ui::new(d.view(), Theme::chosen()).natural_height(SIZE.0 as f32);
    assert!(h < SIZE.1 as f32, "{h}");
}

#[test]
fn the_boxes_of_changes_never_leave_their_window() {
    let mut wrong = Vec::new();
    let mut d = sample();
    d.sheet_at = Rect { x: 900.0, y: 300.0, w: 30.0, h: 30.0 };
    for open in ["/b/Photos", "/d/usb:wipe", "/d/nvme:437000000000"] {
        d.open = Some(open.into());
        for scale in [1.0f32, 1.75, 2.0] {
            for size in [(1280u32, 800u32), (800, 600), (480, 400), (360, 300)] {
                let (places, sheet, _) = sheet_places(&d, size, scale);
                let win = Rect { x: 0.0, y: 0.0, w: size.0 as f32 * scale, h: size.1 as f32 * scale };
                let at = format!("{open} {size:?} at {scale}");
                let inside = |r: Rect, o: Rect| r.x >= o.x - 0.5 && r.y >= o.y - 0.5 && r.x + r.w <= o.x + o.w + 0.5 && r.y + r.h <= o.y + o.h + 0.5;
                if !inside(sheet, win) {
                    wrong.push(format!("{at}: the box {sheet:?} is not inside the window"));
                }

                let scrollers: Vec<Rect> = places.iter().filter(|(k, _)| k == "sheet rows").map(|p| p.1).collect();
                for (k, r) in places.iter().filter(|(k, _)| !k.starts_with("sheet")) {
                    if r.x < sheet.x - 0.5 || r.x + r.w > sheet.x + sheet.w + 0.5 {
                        wrong.push(format!("{at}: {k} {r:?} is across the edge of the box {sheet:?}"));
                    }
                    let seen = scrollers.iter().any(|s| r.y >= s.y - 0.5 && r.y + r.h <= s.y + s.h + 0.5);
                    let reachable = !scrollers.is_empty() && scrollers.iter().any(|s| r.x >= s.x - 0.5 && r.x + r.w <= s.x + s.w + 0.5);
                    if !inside(*r, sheet) && (!reachable || seen) {
                        wrong.push(format!("{at}: {k} {r:?} is outside the box {sheet:?}"));
                    }
                }

                if size.0 >= 800 && open != "/b/Photos" {
                    for (k, r) in places.iter().filter(|(k, _)| !k.starts_with("sheet")) {
                        if !inside(*r, sheet) {
                            wrong.push(format!("{at}: {k} {r:?} is below or beside the box {sheet:?}"));
                        }
                    }
                }
            }
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_format_card_keeps_its_choices_and_its_button_inside() {
    let mut d = sample();
    d.sheet_at = Rect { x: 900.0, y: 300.0, w: 30.0, h: 30.0 };
    d.open = Some("/d/usb:wipe".into());
    for scale in [1.0f32, 1.75, 2.0] {
        for size in [(1280u32, 800u32), (640, 480), (360, 400)] {
            let (places, sheet, view) = sheet_places(&d, size, scale);
            let mut ui = Ui::new(view, Theme::chosen());
            ui.places_at_scale(((size.0 as f32 * scale) as u32, (size.1 as f32 * scale) as u32), scale);
            let words = ui.worded_buttons();
            let (_, pill, _) = words.iter().find(|(w, ..)| w == "Format").expect("Format is a button with the word on it");
            assert!(pill.x >= sheet.x && pill.x + pill.w <= sheet.x + sheet.w + 0.5, "{size:?} at {scale}: {pill:?} in {sheet:?}");
            assert!(pill.y >= sheet.y && pill.y + pill.h <= sheet.y + sheet.h + 0.5, "{size:?} at {scale}: {pill:?} in {sheet:?}");
            let kind = places.iter().find(|(k, _)| k.ends_with(":wipe:kind")).expect("the kinds are laid out").1;
            assert!(kind.x >= sheet.x && kind.x + kind.w <= sheet.x + sheet.w + 0.5, "{size:?} at {scale}: {kind:?} in {sheet:?}");
        }
    }
}

#[test]
fn a_blank_name_asks_for_no_label() {
    for fs in ["exfat", "vfat", "ext4", "ntfs"] {
        for name in ["", "   "] {
            assert!(!fs_opts(fs, name, true).contains_key("label"), "{fs} {name:?}");
            assert!(plan::check_label(fs, name).is_ok(), "{fs} {name:?}");
        }
        assert!(fs_opts(fs, "Photos", false).contains_key("label"));
    }
}

#[test]
fn the_buttons_that_make_a_change_are_greyed_out_while_one_is_made() {
    let free = [t!("disks-mount"), t!("disks-unmount"), t!("disks-open"), t!("disks-remove-safely"), t!("disks-power-off-tip")];
    let mut shut_somewhere = 0;
    for how in ["changing", "snapping"] {
        let mut d = sample();
        let keys: Vec<String> = d.drives.iter().flat_map(|dr| {
            let mut k = vec![format!("{}:wipe", dr.path)];
            k.extend(dr.vols.iter().map(|v| v.path.clone()));
            k.extend(dr.gaps.iter().map(|&(o, _)| format!("{}:{o}", dr.path)));
            k
        }).collect();
        for key in keys {
            d.sheet_at = Rect { x: 900.0, y: 300.0, w: 30.0, h: 30.0 };
            d.open = Some(key.clone());
            d.changing.set(false);
            d.snapping = false;
            let idle = d.view().shut();
            match how {
                "changing" => d.changing.set(true),
                _ => d.snapping = true,
            }
            for (tip, off, why) in d.view().shut() {
                let was = idle.iter().find(|(t, ..)| *t == tip).is_some_and(|(_, off, _)| *off);
                assert!(!free.contains(&tip.as_str()) || !off, "{how} {key}: {tip} is free of it and stays usable");
                if off && !was {
                    assert_eq!(why.as_deref(), Some(plan::change_going()), "{how} {key}: {tip} is greyed out and says why");
                    shut_somewhere += 1;
                }
            }
            let view = d.view();
            for (tip, off, m) in view.presses() {
                assert!(off || !is_a_change(m), "{how} {key}: {tip} makes a change and looks usable");
            }
            for m in view.submits() {
                assert!(!is_a_change(m), "{how} {key}: Enter in a box makes a change while one is going");
            }
        }
    }
    assert!(shut_somewhere > 0, "some button was shut");

    let mut d = sample();
    d.open = Some("/b/Data".into());
    let before = d.view().shut();
    d.changing.set(true);
    d.changing.set(false);
    assert_eq!(d.view().shut(), before);
}

#[test]
fn the_format_card_is_drawn_at_each_scale() {
    let mut d = sample();
    d.sheet_at = Rect { x: 900.0, y: 300.0, w: 30.0, h: 30.0 };
    d.open = Some("/d/usb:wipe".into());
    for scale in [1.0f32, 1.75, 2.0] {
        let (w, h) = ((1280.0 * scale) as u32, (800.0 * scale) as u32);
        let mut ui = Ui::new(d.view(), Theme::chosen());
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), scale);
        if let Ok(out) = std::env::var("H468_SHOTS") {
            std::fs::create_dir_all(&out).unwrap();
            pix.save_png(std::path::Path::new(&out).join(format!("format-card-{}.png", (scale * 100.0) as u32))).unwrap();
        }
    }
}

#[test]
fn the_window_is_a_puzzle_with_no_loose_tile() {
    let d = sample();
    let fonts = crate::toolkit::text::Fonts::new();
    let mut wrong = Vec::new();
    for w in [1280.0, 900.0, 400.0] {
        for (path, spare) in crate::toolkit::widget::loose_tiles(&fonts, &d.view(), w, 800.0) {
            wrong.push(format!("at {w}: tile {path:?} has {spare} spare"));
        }
    }
    for w in [1280.0, 1920.0] {
        let found = crate::toolkit::widget::shape::check(&fonts, &d.view(), w, 800.0);
        if found.wide > 1 {
            wrong.push(format!("at {w}: {} cards wider than three quarters", found.wide));
        }
        wrong.extend(found.bars.iter().map(|b| format!("at {w}: thinner than 3:1: {b}")));
        wrong.extend(found.long.iter().map(|l| format!("a long line: {l}")));
        if !found.puzzle {
            wrong.push(format!("at {w}: not a puzzle"));
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_map_draws_every_part_as_wide_as_it_is_big() {
    let d = sample();
    let rows = d.map_rows();
    assert_eq!(rows.len(), 3);

    assert_eq!(rows[0].parts.len(), 5);
    assert!(rows[0].parts[4].gap);
    assert_eq!(rows[0].parts[2].words, "180.0 GB free");
    assert_eq!(rows[2].parts[0].words, "Not mounted");
    let segs = segments(&rows, 800.0);
    let first: Vec<_> = segs.iter().filter(|s| s.2 == 0).collect();

    for pair in first.windows(2) {
        assert!(pair[1].0 >= pair[0].0 + pair[0].1, "{pair:?}");
    }
    let end = first.last().map(|s| s.0 + s.1).unwrap_or(0.0);
    assert!((end - 800.0).abs() < 0.5, "{end}");
    assert!(first.iter().all(|s| s.1 >= SEG_LEAST));

    assert!(first[2].1 > 4.0 * first[0].1);
    assert!(rows[0].spoken().contains("Data, 40% full, 180.0 GB free"), "{}", rows[0].spoken());
    assert_eq!(d.together(), "3 drives, 2.5 TB in all. 1 needs a look, see its card.");
}

#[test]
fn a_lone_drive_fills_a_tall_card_and_many_drives_do_not_grow() {

    let need = 3.0 * MAP_ROW - MAP_GAP;
    assert_eq!(map_spacing(need, 3), (MAP_ROW, MAP_BAR, 0.0));

    assert_eq!(map_spacing(100.0, 3), (MAP_ROW, MAP_BAR, 0.0));

    let (pitch, bar, top) = map_spacing(400.0, 1);
    assert!(pitch > MAP_ROW && bar > MAP_BAR && top > 0.0);
    assert!(top + pitch - MAP_GAP + top <= 400.0 + 0.5, "{top} {pitch}");
}

#[test]
fn a_full_part_takes_a_worry_colour() {
    assert!(worry(0.5).is_none());
    assert_eq!(worry(0.9), Some(theme::warn()));
    assert_eq!(worry(0.97), Some(theme::problem()));
}

#[test]
fn the_box_of_changes_names_only_what_is_there() {
    let d = sample();
    assert!(!d.names("/b/nothing"));
    assert!(d.names("/b/Data"));
    assert!(d.names("/d/nvme:wipe"));
    assert!(d.names(&format!("/d/nvme:{}", 437_000_000_000u64)));
}

#[test]
fn the_i_and_the_box_of_changes_leave_the_page_where_it_was() {
    let mut d = sample();
    let mut ui = Ui::new(d.view(), Theme::new(true, "#fbc711"));
    ui.lay_at(900, 420);
    ui.to_the_ends();
    assert!(ui.places().iter().any(|p| p.0 == "drives" && p.1 > 0.0), "{:?}", ui.places());
    for m in [Msg::Help, Msg::Help, Msg::Toggle("/b/Data".into()), Msg::Toggle("/b/System".into()), Msg::Toggle("/b/System".into())] {
        let what = format!("{:?}", std::mem::discriminant(&m));
        d.update(m);

        let moved: Vec<String> = ui.moved_by(d.view()).into_iter().filter(|m| !m.starts_with("sheet rows") && !m.starts_with("help words")).collect();
        assert_eq!(moved, Vec::<String>::new(), "{what}");
    }
}

#[test]
fn the_box_of_changes_floats_and_closes() {
    let mut d = sample();
    assert!(d.sheet().is_none());
    d.update(Msg::Toggle("/b/Data".into()));
    assert!(d.sheet().is_some());

    d.update(Msg::Toggle("/b/System".into()));
    assert_eq!(d.open.as_deref(), Some("/b/System"));
    d.update(Msg::Toggle("/b/System".into()));
    assert!(d.sheet().is_none());

    d.update(Msg::Toggle("/b/Data".into()));
    d.update(Msg::Help);
    assert!(d.help && d.open.is_none());

    let esc = Input::Key { sym: Keysym::Escape, text: None, mods: Default::default(), repeat: false };
    assert!(matches!(d.intercept(&esc, false), Some(Msg::Help)));
}

#[test]
fn nothing_that_erases_is_offered_for_the_system_drive() {
    let d = sample();
    let kinds = d.kinds();
    let nvme = &d.drives[0];
    let system = d.part_extras(nvme, &nvme.vols[1], &kinds);
    assert!(system.is_empty(), "the system part has {} changes", system.len());
    let usb = &d.drives[1];
    assert!(d.part_extras(usb, &usb.vols[0], &kinds).len() >= 4);
}
