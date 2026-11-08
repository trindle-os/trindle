//! Tests for the Appearance page.

use super::fonts_icons::{families, split_font};
use super::looks::{look_key, look_name, picture_key, refused_key, LOOKS_GAP, SAMPLE, THEMES_WAIT};
use super::*;
use crate::toolkit::colour::Colour;
use crate::toolkit::widget::Rect;

#[test]
fn the_highlight_swatches_are_the_real_palette() {
    let all = highlights();
    assert_eq!(all.len(), 20);
    assert_eq!(all[0].0, style::DEFAULT_ACCENT);
    let want = ["f34236", "e8207e", "9b26ae", "6639ab", "3d51b4", "2295f1", "03a8f3", "00bcd3", "019587", "4bae4f",
                "8ac248", "cbdc38", "ffea3a", "ffc006", "fe9700", "ff5721", "795649", "9e9d9e", "607c8b"];
    let got: Vec<String> = all[1..].iter().map(|(h, _)| h.trim_start_matches('#').to_lowercase()).collect();
    assert_eq!(got, want);
    let mut seen = got.clone();
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), 19, "every swatch is different");

}

#[test]
fn the_switch_covers_the_rest_of_a_batch_of_fonts() {
    use crate::settings::ui::stand::Stand;
    use crate::settings::ui::stand::made_up_taken;
    let mut s = Stand::new(Box::new(Appearance::default()));
    let left = |s: &mut Stand| s.page::<Appearance, _>(|p, _| p.taken_fonts.more() + usize::from(!p.taken_fonts.is_empty()));
    s.page::<Appearance, _>(|p, _| p.taken_fonts.add_batch(["A.ttf", "B.otf", "C.ttf"].map(made_up_taken)));
    s.page::<Appearance, _>(|p, _| p.skip_font());
    assert_eq!(left(&mut s), 2, "one answer with the switch off covered more than one font");
    s.page::<Appearance, _>(|p, _| {
        p.taken_fonts.set_same(true);
        p.skip_font();
    });
    assert_eq!(left(&mut s), 0, "the switch did not cover the rest of the batch");
}

#[test]
#[ignore]
fn shoot_the_font_question_with_the_switch() {
    use crate::settings::ui::stand::{made_up_taken, shoot_both};
    let mut page = Appearance::default();
    page.taken_fonts.add_batch(["Lora.ttf", "Inter.otf", "Mono.ttf", "Serif.ttf"].map(made_up_taken));
    shoot_both(Box::new(page), "376-fonts", (1600.0, 1700.0));
}

#[test]
#[ignore]
fn shoot_unread_appearance() {
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::ui::{App as _, Ui};
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1920x1080".into());
    let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<f32>().ok()?, b.parse::<f32>().ok()?))).unwrap_or((1920.0, 1080.0));
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events.clone(), asks, None, None);
    kit.current = "appearance".into();
    let s = Settings { kit, pages: vec![Box::new(Appearance::default())], current: 0, search: String::new(), tall: h };
    let mut ui = Ui::new(s.view(), crate::toolkit::theme::Theme::new(false, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    std::fs::create_dir_all(&out).unwrap();
    pix.save_png(out.join(format!("appearance-unread-{size}.png"))).unwrap();
}

#[test]
fn the_themes_are_read_off_the_window_and_a_ring_turns_until_they_come() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    static READ_ELSEWHERE: AtomicBool = AtomicBool::new(false);
    static ASKING: std::sync::Mutex<Option<std::thread::ThreadId>> = std::sync::Mutex::new(None);
    fn slow() -> themes::Seen {

        std::thread::sleep(std::time::Duration::from_millis(1500));
        let here = std::thread::current().id();
        READ_ELSEWHERE.store(ASKING.lock().unwrap().is_some_and(|asking| asking != here), Ordering::SeqCst);
        themes::Seen { listed: themes::Listed { themes: vec![themes::from_sheet("", style::CSS)], said: vec![themes::LIST_SLOW] }, ..themes::Seen::default() }
    }
    *ASKING.lock().unwrap() = Some(std::thread::current().id());
    let mut s = showing(Appearance { reader: Some(slow), accent: style::DEFAULT_ACCENT.into(), ..Appearance::default() }, 2400.0);
    let began = std::time::Instant::now();
    s.with::<Appearance>(|page, kit| page.read_themes(kit));
    assert!(began.elapsed() < std::time::Duration::from_millis(300), "asking held the window up for {:?}", began.elapsed());
    let keys = |s: &crate::settings::Settings| {
        let mut ui = Ui::new(s.view(), Theme::new(false, style::DEFAULT_ACCENT));
        ui.lay_at(1180, 2400);
        ui.boxes().iter().map(|b| b.0.clone()).collect::<Vec<String>>()
    };

    let waiting = keys(&s);
    assert!(waiting.iter().any(|k| k == THEMES_WAIT), "the ring turns: {waiting:?}");
    assert!(!waiting.iter().any(|k| k.starts_with("theme-trouble:") || k.starts_with("theme-picture:")), "{waiting:?}");

    while began.elapsed() < std::time::Duration::from_secs(20) && !READ_ELSEWHERE.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(READ_ELSEWHERE.load(Ordering::SeqCst), "the themes were read on a thread that is not the window's");
    s.with::<Appearance>(|page, _| page.took(slow_answer()));
    let after = keys(&s);
    assert!(!after.iter().any(|k| k == THEMES_WAIT), "the ring has gone: {after:?}");
    assert_eq!(after.iter().filter(|k| k.starts_with("theme-trouble:")).count(), 1, "{after:?}");
    assert_eq!(after.iter().filter(|k| k.starts_with("theme-picture:")).count(), 1, "Trindle's own look is listed: {after:?}");
    s.with::<Appearance>(|page, _| assert_eq!(page.troubles(), [themes::LIST_SLOW]));
    fn slow_answer() -> themes::Seen {
        themes::Seen { listed: themes::Listed { themes: vec![themes::from_sheet("", style::CSS)], said: vec![themes::LIST_SLOW] }, ..themes::Seen::default() }
    }
}

fn the_themes_that_come() -> Vec<themes::Theme> {
    let mut all = vec![themes::from_sheet("", style::CSS)];
    for (id, css) in [("bold", include_str!("../../../../share/themes/bold.css")), ("snug", include_str!("../../../../share/themes/snug.css")),
                      ("midnight", include_str!("../../../../share/themes/midnight.css")), ("paper", include_str!("../../../../share/themes/paper.css"))] {
        all.push(themes::from_sheet(id, css));
    }
    all
}

const HARBOUR: &str = ":root { --trindle-theme-name: \"Harbour\"; --trindle-theme-look: both; --trindle-page: #0e1620; \
                       --trindle-card: #182432; --trindle-box: #223244; }\n\
                       @media (prefers-color-scheme: light) { :root { --trindle-page: #e6ecf1; --trindle-card: #ffffff; --trindle-box: #eef2f6; } }";

fn showing(page: Appearance, tall: f32) -> crate::settings::Settings {
    use crate::settings::ui::Do;
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events.clone(), asks, None, None);
    kit.current = "appearance".into();
    crate::settings::Settings { kit, pages: vec![Box::new(page)], current: 0, search: String::new(), tall }
}

#[test]
fn every_themes_tile_offers_exactly_the_looks_it_has() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let mut all = the_themes_that_come();
    all.push(themes::from_sheet("harbour", HARBOUR));
    let looks: Vec<(String, usize)> = all.iter().map(|t| (t.id.clone(), t.looks().len())).collect();
    assert_eq!(looks, [("", 2), ("bold", 1), ("snug", 2), ("midnight", 1), ("paper", 1), ("harbour", 2)].map(|(id, n)| (id.to_string(), n)));
    for (w, h) in [(1180u32, 800u32), (700, 1100)] {
        for chosen in all.iter() {
            for saved in [false, true] {
                let what = format!("{} chosen, {} saved, {w} wide", if chosen.id.is_empty() { "trindle" } else { &chosen.id }, look_name(saved));
                let page = Appearance { themes: all.clone(), theme: chosen.id.clone(), light: saved, ..Appearance::made_up() };
                let s = showing(page, h as f32);
                let window = Theme::new(false, style::DEFAULT_ACCENT);
                let mut ui = Ui::new(s.view(), window.clone());
                ui.lay_at(w, h);
                let boxes = ui.boxes();
                let at = |key: &str| boxes.iter().find(|b| b.0 == key).map(|b| (b.1, b.2));
                let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                let is = |x: f32, y: f32, c: Colour| {
                    let p = pix.pixel(x as u32, y as u32).unwrap();
                    (p.red() as f32 - c.r * 255.0).abs() <= 1.5 && (p.green() as f32 - c.g * 255.0).abs() <= 1.5 && (p.blue() as f32 - c.b * 255.0).abs() <= 1.5
                };
                let mut widths = Vec::new();
                for t in &all {
                    widths.push(one_themes_tile(&what, t, (chosen, saved), h, (&at, &is), &window));
                }

                assert!(widths.iter().all(|x| *x == widths[0]), "{what}: {widths:?}");
                let pressed: Vec<String> = ui.pressables().into_iter().filter(|(id, _)| id.starts_with("theme-look:")).map(|(id, kind)| format!("{id} {kind}")).collect();
                assert_eq!(pressed, ["theme-look:default:dark button", "theme-look:default:light button", "theme-look:snug:dark button",
                                     "theme-look:snug:light button", "theme-look:harbour:dark button", "theme-look:harbour:light button"], "{what}");
            }
        }
    }
}

type Boxes<'a> = (&'a dyn Fn(&str) -> Option<(Rect, Option<Rect>)>, &'a dyn Fn(f32, f32, Colour) -> bool);

fn one_themes_tile(what: &str, t: &themes::Theme, (chosen, saved): (&themes::Theme, bool), h: u32, (at, is): Boxes, window: &crate::toolkit::theme::Theme) -> (f32, f32) {
    let (pic, _) = at(&picture_key(&t.id)).unwrap_or_else(|| panic!("{what}: {} has no picture", t.name));
    assert!(pic.w >= 96.0 && pic.h == SAMPLE.1, "{what}: {}'s picture, {pic:?}", t.name);

    let light = t.light(saved);
    let sample = crate::toolkit::theme::sample(&t.css, light, style::DEFAULT_ACCENT).theme;
    assert_eq!(sample.light, light, "{what}: {}", t.name);
    if pic.y + pic.h <= h as f32 {
        assert!(is(pic.x + pic.w / 2.0, pic.y + 3.0, sample.bg), "{what}: {}'s picture is drawn {}", t.name, look_name(light));
    }
    let buttons: Vec<(bool, Rect)> = [false, true].into_iter().filter_map(|l| at(&look_key(&t.id, l)).map(|b| (l, b.0))).collect();
    assert_eq!(buttons.iter().map(|b| b.0).collect::<Vec<_>>(), if t.looks().len() == 2 { t.looks().to_vec() } else { Vec::new() },
               "{what}: the looks {} offers", t.name);
    let mut wide = pic.w;
    if let [(_, dark), (_, lit)] = buttons[..] {
        assert!(dark.x >= pic.x + pic.w + LOOKS_GAP - 0.5, "{what}: {}'s looks are to the right of its picture", t.name);
        assert!(dark.x == lit.x && dark.w == lit.w && lit.y >= dark.y + dark.h + 4.0, "{what}: {}: Light is under Dark, {dark:?} {lit:?}", t.name);
        assert!(dark.y >= pic.y - 0.5 && lit.y + lit.h <= pic.y + pic.h + 0.5, "{what}: {}'s looks are beside its picture", t.name);
        assert!(dark.h >= 28.0 && dark.w >= 44.0 && lit.h >= 28.0, "{what}: {}'s looks can be pressed", t.name);
        wide = dark.x + dark.w - pic.x;

        for (l, b) in [(false, dark), (true, lit)] {
            if b.y + b.h > h as f32 {
                continue;
            }
            let on = t.id == chosen.id && light == l;
            assert_eq!(is(b.x + b.w / 2.0, b.y + 3.0, window.picked), on, "{what}: {} {} is {}marked", t.name, look_name(l), if on { "" } else { "not " });
        }
    }
    (wide, pic.h)
}

#[test]
fn a_theme_that_cannot_be_used_says_why_on_its_tile() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let mut all = the_themes_that_come();
    all.push(themes::Theme { refused: Some(themes::TOO_BIG), mine: true, ..themes::from_sheet("far-too-big", "") });
    all.push(themes::from_sheet("harbour", HARBOUR));
    for (w, h) in [(1180u32, 2400u32), (700, 3200)] {
        let page = Appearance { themes: all.clone(), ..Appearance::made_up() };
        let s = showing(page, h as f32);
        let window = Theme::new(false, style::DEFAULT_ACCENT);
        let mut ui = Ui::new(s.view(), window.clone());
        ui.lay_at(w, h);
        let boxes = ui.boxes();
        let at = |key: &str| boxes.iter().find(|b| b.0 == key).map(|b| b.1);
        let said = at(&refused_key("far-too-big")).unwrap_or_else(|| panic!("{w} wide: no reason on the tile"));
        assert!(said.w >= 96.0 && said.h >= 12.0 && said.x >= 0.0 && said.x + said.w <= w as f32, "{w} wide: {said:?}");
        assert_eq!(at(&picture_key("far-too-big")), None, "{w} wide: no picture of a sheet never read");
        for t in all.iter().filter(|t| t.refused.is_none()) {
            assert!(at(&picture_key(&t.id)).is_some() && at(&refused_key(&t.id)).is_none(), "{w} wide: {}", t.name);
        }

        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        let red = window.danger_text;
        let reds = (said.y as u32..(said.y + said.h) as u32)
            .flat_map(|y| (said.x as u32..(said.x + said.w) as u32).map(move |x| (x, y)))
            .filter(|(x, y)| {
                let p = pix.pixel(*x, *y).unwrap();
                (p.red() as f32 - red.r * 255.0).abs() <= 2.0 && (p.green() as f32 - red.g * 255.0).abs() <= 2.0 && (p.blue() as f32 - red.b * 255.0).abs() <= 2.0
            })
            .count();
        assert!(reds > 40, "{w} wide: {reds} red pixels in {said:?}");

        let pressed: Vec<String> = ui.pressables().into_iter().map(|(id, _)| id).filter(|id| id.contains("far-too-big")).collect();
        assert_eq!(pressed, ["twice:theme:far-too-big"], "{w} wide");
    }

    assert!(themes::TOO_BIG.contains(&format!("{} KB", crate::common::sheet::THEME_MOST / 1024)));
}

#[test]
fn a_theme_in_use_that_cannot_be_used_is_said_in_the_headline_and_in_red() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let mut all = the_themes_that_come();
    all.push(themes::Theme { refused: Some(themes::TOO_BIG), mine: true, ..themes::from_sheet("far-too-big", "") });
    let laid = |page: Appearance| {
        let s = showing(page, 2400.0);
        let mut ui = Ui::new(s.view(), Theme::new(false, style::DEFAULT_ACCENT));
        ui.lay_at(1180, 2400);
        let troubles: Vec<_> = ui.boxes().iter().filter(|b| b.0.starts_with("theme-trouble:")).map(|b| b.1).collect();
        (troubles, ui)
    };

    let fine = Appearance { themes: all.clone(), ..Appearance::made_up() };
    assert!(!fine.top_words().contains("cannot be used") && fine.troubles().is_empty());
    assert!(laid(fine).0.is_empty());

    let gone = Appearance { themes: all.clone(), theme: String::new(), unusable: Some(("Harbour".into(), themes::GONE)), ..Appearance::made_up() };
    assert_eq!(gone.top_words(), "Trindle's own look, because the Harbour theme cannot be used. Its file is no longer there.");
    assert_eq!(gone.troubles(), ["The Harbour theme cannot be used. Its file is no longer there."]);
    let (troubles, mut ui) = laid(gone);
    assert_eq!(troubles.len(), 1);
    let said = troubles[0];
    assert!(said.w >= 96.0 && said.h >= 12.0 && said.x >= 0.0 && said.x + said.w <= 1180.0, "{said:?}");
    let mut pix = tiny_skia::Pixmap::new(1180, 2400).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let red = Theme::new(false, style::DEFAULT_ACCENT).danger_text;
    let reds = (said.y as u32..(said.y + said.h) as u32)
        .flat_map(|y| (said.x as u32..(said.x + said.w) as u32).map(move |x| (x, y)))
        .filter(|(x, y)| {
            let p = pix.pixel(*x, *y).unwrap();
            (p.red() as f32 - red.r * 255.0).abs() <= 2.0 && (p.green() as f32 - red.g * 255.0).abs() <= 2.0 && (p.blue() as f32 - red.b * 255.0).abs() <= 2.0
        })
        .count();
    assert!(reds > 40, "{reds} red pixels in {said:?}");

    let big = Appearance { themes: all.clone(), theme: String::new(), unusable: Some(("Far too big".into(), themes::TOO_BIG)), ..Appearance::made_up() };
    assert!(big.top_words().starts_with("Trindle's own look, because the Far too big theme cannot be used. This file is too big"));
    assert!(big.troubles().is_empty());

    let more = Appearance { themes: all.clone(), said: vec![themes::NOT_ALL, themes::OWN_TOO_BIG], ..Appearance::made_up() };
    assert_eq!(more.troubles(), [themes::NOT_ALL, themes::OWN_TOO_BIG]);
    assert_eq!(laid(more).0.len(), 2);

    for said in [themes::LIST_SLOW, themes::NOT_ALL, themes::OWN_TOO_BIG, themes::OWN_NOT_A_SHEET, themes::OWN_TOO_SLOW, themes::OWN_NO_TARGET, themes::OWN_HARD_TO_READ] {
        let one = Appearance { themes: all.clone(), said: vec![said], ..Appearance::made_up() };
        assert_eq!(one.troubles(), [said]);
        let (troubles, _) = laid(one);
        assert_eq!(troubles.len(), 1, "{said}");
        assert!(troubles[0].w >= 96.0 && troubles[0].h >= 12.0 && troubles[0].x + troubles[0].w <= 1180.0, "{said}: {:?}", troubles[0]);
    }
}

#[test]
fn a_search_for_dark_light_or_look_leads_to_the_theme_card() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    for word in ["dark", "light", "look", "Dark", "dark mode", "light theme"] {
        let mut s = showing(Appearance::made_up(), 800.0);
        s.pages = crate::settings::panels::all();
        s.search = word.to_string();
        assert_eq!(s.first_found(), Some("appearance"), "{word}");
    }
    let page = Appearance { themes: the_themes_that_come(), ..Appearance::made_up() };
    let s = showing(page, 800.0);
    let mut ui = Ui::new(s.view(), Theme::new(false, style::DEFAULT_ACCENT));
    ui.lay_at(1180, 800);
    let boxes = ui.boxes();
    for look in [false, true] {
        let b = boxes.iter().find(|b| b.0 == look_key("", look)).map(|b| b.1).unwrap_or_else(|| panic!("no {}", look_name(look)));
        assert!(b.y > 0.0 && b.y + b.h <= 800.0, "{} is in sight before anything is scrolled: {b:?}", look_name(look));
    }

    let said = s.view().said();
    let at = |what: &str| said.iter().position(|w| w == what).unwrap_or_else(|| panic!("no {what}: {said:?}"));
    assert!(at("Theme") < at("button: Dark") && at("button: Dark") < at("button: Light") && at("button: Light") < at("Colour")
            && at("Colour") < at("Highlight"), "{said:?}");
    assert!(!said.iter().any(|w| w == "Look" || w.ends_with(": Look")), "the Look row has gone from the Colour card: {said:?}");
    assert!(!ui.pressables().iter().any(|(id, kind)| *kind == "dropdown" && id.contains("look")), "{:?}", ui.pressables());
}

#[test]
#[ignore]
fn shoot_the_theme_card() {
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::ui::{App as _, Ui};
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/theme-looks".into()));
    let name = std::env::var("TRINDLE_SHOT_NAME").unwrap_or_else(|_| "theme-card".into());
    let theme = std::env::var("TRINDLE_SHOT_THEME").unwrap_or_default();
    let mut all = the_themes_that_come();

    if let Ok(old) = std::env::var("TRINDLE_SHOT_OLD_NAME") {
        let css = include_str!("../../../../share/themes/snug.css").replace("\"Snug\"", &format!("\"{old}\""));
        all.retain(|t| t.id != "snug");
        all.push(themes::from_sheet("snug", &css));
    }
    let page = Appearance { themes: all, theme: theme.clone(), light: asked_light(), ..Appearance::made_up() };
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events.clone(), asks, None, None);
    kit.current = "appearance".into();
    kit.icon = "applications-graphics-symbolic";
    let mut s = Settings { kit, pages: vec![Box::new(page)], current: 0, search: String::new(), tall: 0.0 };
    std::fs::create_dir_all(&out).unwrap();
    for (w, h, tail) in [(1180u32, 800u32, ""), (700, 1100, "-narrow")] {
        s.tall = h as f32;
        let mut ui = Ui::new(s.view(), crate::toolkit::theme::Theme::chosen());
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{name}{tail}.png"))).unwrap();
    }
}

#[test]
fn the_page_is_drawn_in_hostile_themes_and_the_way_back_stays_readable() {
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::theme::{contrast, safe, sample, WORDS};
    use crate::toolkit::ui::{App as _, Ui};
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events.clone(), asks, None, None);
    kit.current = "appearance".into();
    let mut page = Appearance { trial: Trial::at(14), trials: 1, ..Appearance::made_up() };
    let sheets = crate::toolkit::theme::hostile_sheets_for_tests(40);

    for (n, css) in sheets.iter().enumerate().take(6) {
        page.themes.push(themes::Theme { id: format!("hostile-{n}"), name: format!("Hostile {n}"), about: "\u{202e}x".repeat(60), author: "John Smith".into(),
                                         version: "1".into(), mine: true, css: css.clone(), only: crate::common::sheet::one_look(css), refused: None });
    }

    page.themes.push(themes::Theme { refused: Some(themes::TOO_BIG), ..themes::from_sheet("too-big", "") });
    let s = Settings { kit, pages: vec![Box::new(page)], current: 0, search: String::new(), tall: 900.0 };
    let safe = safe();
    for (n, css) in sheets.iter().enumerate() {
        let look = sample(css, n % 2 == 1, ["#fbc711", "#000000", "#ffffff"][n % 3]).theme;
        let mut ui = Ui::new(s.view(), look);
        let mut pix = tiny_skia::Pixmap::new(1100, 900).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);

        let is = |p: &tiny_skia::PremultipliedColorU8, c: Colour| {
            (p.red() as i32 - (c.r * 255.0) as i32).abs() <= 1 && (p.green() as i32 - (c.g * 255.0) as i32).abs() <= 1
                && (p.blue() as i32 - (c.b * 255.0) as i32).abs() <= 1
        };
        let box_pixels = pix.pixels().iter().filter(|p| is(p, safe.raised)).count();
        let word_pixels = pix.pixels().iter().filter(|p| is(p, safe.fg)).count();
        let keep_pixels = pix.pixels().iter().filter(|p| is(p, safe.accent)).count();
        assert!(box_pixels > 5000 && word_pixels > 100 && keep_pixels > 1500, "sheet {n}: {box_pixels} {word_pixels} {keep_pixels}");
    }
    assert!(contrast(safe.fg, safe.raised) >= WORDS && contrast(safe.on_accent, safe.accent) >= WORDS);
}

#[test]
fn font_names_split() {
    assert_eq!(split_font("Cantarell 11"), ("Cantarell".into(), "11".into()));
    assert_eq!(split_font("DejaVu Sans Bold 10.5"), ("DejaVu Sans Bold".into(), "10.5".into()));
    assert_eq!(split_font("Sans"), ("Sans".into(), "11".into()));
}

#[test]
fn families_first_name_once() {
    let out = "DejaVu Sans,DejaVu Sans Light\nDejaVu Sans\nNoto\\-Mono\n\n";
    assert_eq!(families(out), vec!["DejaVu Sans".to_string(), "Noto-Mono".to_string()]);
}
