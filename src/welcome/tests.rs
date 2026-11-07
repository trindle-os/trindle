//! Tests for the first-run welcome pages.

use crate::toolkit::input::Mods;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
use crate::toolkit::widget::What;

use super::*;
use crate::toolkit::widget::Tone;
use super::pages::{first_apps, more_apps, named};
use super::view::no_plugins_yet;

fn welcome() -> Welcome {
    let mut w = Welcome::new(None);
    w.moving = false;
    w
}

fn key(sym: Keysym) -> Input {
    Input::Key { sym, text: None, mods: Mods::default(), repeat: false }
}

fn at(page: Page) -> usize {
    PAGES.iter().position(|p| *p == page).expect("a page")
}

fn said(w: &Welcome) -> Vec<String> {
    w.view().said()
}

#[test]
fn the_pages_step_forward_and_back_and_stop_at_the_ends() {
    let mut w = welcome();
    w.update(Msg::Back);
    assert_eq!(w.page, 0, "nothing before the first page");
    for n in 1..PAGES.len() {
        w.update(Msg::Next);
        assert_eq!(w.page, n);
    }
    w.update(Msg::Next);
    assert_eq!(w.page, PAGES.len() - 1, "nothing after the last page");
    w.update(Msg::Back);
    assert_eq!(w.here(), Page::Words);
    assert!(w.asked.iter().any(|a| matches!(a, Ask::Say(s) if s.starts_with("Welcome to Trindle. Step 2 of 14."))));
}

#[test]
fn the_main_pill_moves_on_and_the_last_one_starts_trindle() {
    let mut w = welcome();
    assert_eq!(w.main_words(), "Next");
    for _ in 0..PAGES.len() - 1 {
        w.update(Msg::Main);
    }
    assert_eq!(w.here(), Page::Know);
    assert_eq!(w.main_words(), "Start using Trindle");
    w.update(Msg::Main);
    assert_eq!(w.asked.last(), Some(&Ask::Close));
}

#[test]
fn keys_move_through_the_pages_and_escape_closes() {
    let w = welcome();
    let free = Keyboard::default();
    let on_widget = Keyboard { keys: true, ..Keyboard::default() };
    let on_arrows = Keyboard { keys: true, arrows: true, ..Keyboard::default() };
    let got = |input: Input, kb: Keyboard| format!("{:?}", w.intercept_with(&input, kb));
    assert_eq!(got(key(Keysym::Right), free), "Some(Next)");
    assert_eq!(got(key(Keysym::Page_Down), on_widget), "Some(Next)");
    assert_eq!(got(key(Keysym::Left), free), "Some(Back)");
    assert_eq!(got(key(Keysym::Return), free), "Some(Main)");
    assert_eq!(got(key(Keysym::Escape), on_arrows), "Some(Close)");
    assert_eq!(got(key(Keysym::Return), on_widget), "None");
    assert_eq!(got(key(Keysym::Right), on_arrows), "None");
    let ctrl = Input::Key { sym: Keysym::Right, text: None, mods: Mods { ctrl: true, ..Mods::default() }, repeat: false };
    assert_eq!(got(ctrl, free), "None");
}

#[test]
fn apps_picked_on_both_pages_install_with_one_press() {
    let mut w = welcome();
    w.go_to(at(Page::Apps));
    assert_eq!(w.main_words(), "Next", "picking happens on two pages, so the first only moves on");
    w.update(Msg::Pick("org.mozilla.firefox".into()));
    w.update(Msg::Pick("org.signal.Signal".into()));
    w.update(Msg::Pick("org.signal.Signal".into()));
    assert_eq!(w.picked.iter().collect::<Vec<_>>(), ["org.mozilla.firefox"], "a second press unpicks");
    w.update(Msg::Main);
    assert_eq!(w.here(), Page::MoreApps);
    w.update(Msg::Pick("org.videolan.VLC".into()));
    assert_eq!(w.main_words(), "Install the chosen apps");
    w.update(Msg::Main);
    let ids = vec!["org.mozilla.firefox".to_string(), "org.videolan.VLC".to_string()];
    assert!(w.asked.contains(&Ask::Install(ids.clone())));
    assert_eq!(w.here(), Page::Plugins, "the tour carries on while they download");
    assert!(w.installing());
    w.update(Msg::Pick("org.gimp.GIMP".into()));
    assert!(w.picked.is_empty());
    w.progress(0.5, Some("about a minute left".into()));
    assert!(said(&w).contains(&"Installing Firefox and VLC, about a minute left.".to_string()));
    w.installed_now(true, String::new());
    assert!(w.installed.contains("org.videolan.VLC"));
    assert_eq!(w.glow, 1, "their tiles glow");
    assert!(said(&w).contains(&"Firefox and VLC are in the ring now.".to_string()));
}

#[test]
fn nothing_picked_means_nothing_to_install() {
    let mut w = welcome();
    w.go_to(at(Page::MoreApps));
    assert_eq!(w.main_words(), "Next");
    w.update(Msg::Main);
    assert!(!w.asked.iter().any(|a| matches!(a, Ask::Install(_))));
}

#[test]
fn offline_the_install_waits_and_says_why() {
    let mut w = welcome();
    w.online = false;
    w.go_to(at(Page::MoreApps));
    w.update(Msg::Pick("org.gimp.GIMP".into()));
    w.update(Msg::Main);
    assert!(!w.asked.iter().any(|a| matches!(a, Ask::Install(_))));
    assert_eq!(w.here(), Page::MoreApps);
    assert!(said(&w).contains(&"Connect to the internet to install apps.".to_string()));
    w.online = true;
    w.update(Msg::Main);
    assert!(w.asked.contains(&Ask::Install(vec!["org.gimp.GIMP".into()])));
}

#[test]
fn a_failed_install_is_said_in_red_and_the_apps_can_be_picked_again() {
    let mut w = welcome();
    w.go_to(at(Page::MoreApps));
    w.update(Msg::Pick("org.gimp.GIMP".into()));
    w.update(Msg::Main);
    w.installed_now(false, "No remote refs found for org.gimp.GIMP.".into());
    assert!(!w.installed.contains("org.gimp.GIMP"));
    let view = w.view();
    let mut red = Vec::new();
    fn find(n: &Node<Msg>, red: &mut Vec<String>) {
        if let What::Label { text, tone: Tone::Problem, .. } = &n.what {
            red.push(text.clone());
        }
        n.items().iter().for_each(|i| find(i, red));
    }
    find(&view, &mut red);
    assert_eq!(red, ["The apps could not be installed. No remote refs found for org.gimp.GIMP."]);
    w.go_to(at(Page::MoreApps));
    w.update(Msg::Pick("org.gimp.GIMP".into()));
    assert!(w.picked.contains("org.gimp.GIMP"));
}

#[test]
fn apps_already_installed_show_as_installed_and_cannot_be_picked() {
    let mut w = welcome();
    w.installed = ["org.mozilla.firefox".to_string()].into_iter().collect();
    w.go_to(at(Page::Apps));
    let words = said(&w);
    let firefox = words.iter().position(|s| s == "Firefox").expect("Firefox is offered");
    assert_eq!(words[firefox + 1], "Installed");
    assert!(!words.contains(&"button: Pick Firefox".to_string()));
    assert!(words.contains(&"button: Pick Brave".to_string()));
    w.update(Msg::Pick("org.mozilla.firefox".into()));
    assert!(w.picked.is_empty());
}

#[test]
fn only_apps_in_flathubs_catalogue_are_offered_once_it_is_read() {
    let mut w = welcome();
    w.go_to(at(Page::Apps));
    assert!(said(&w).contains(&"Zen".to_string()), "before the catalogue, every app shows");
    w.known = Some(every_app().map(|(id, _)| id.to_string()).filter(|id| id != "app.zen_browser.zen").collect());
    assert!(!said(&w).contains(&"Zen".to_string()));
    assert!(said(&w).contains(&"Brave".to_string()));
}

fn walk(n: &Node<Msg>, each: &mut impl FnMut(&Node<Msg>)) {
    each(n);
    n.items().iter().for_each(|i| walk(i, each));
}

#[test]
fn every_page_has_its_name_one_sentence_and_its_step() {
    let mut w = welcome();
    for (n, page) in PAGES.iter().enumerate() {
        w.go_to(n);
        let words = said(&w);
        assert_eq!(words[0], page.title());
        assert!(words.contains(&format!("Step {} of {}", n + 1, PAGES.len())), "{page:?} says its step");
        let sentence = page.sentence();
        assert!(words.contains(&sentence.to_string()), "{page:?} says its sentence");
        assert!(!sentence.trim_end_matches('.').contains(". "), "{page:?}: one sentence, not two");
        assert!(sentence.len() <= 72, "{page:?}: short enough to stay on one line");
        for tip in ["Close", "Back"] {
            assert!(!words.contains(&tip.to_string()), "{page:?} has a caption {tip}");
        }
    }
    assert!(said(&w).contains(&"button: Close".to_string()) && said(&w).contains(&"button: Back".to_string()));
}

#[test]
fn a_picked_app_is_shown_by_its_button_alone() {
    let mut w = welcome();
    w.go_to(at(Page::Apps));
    w.update(Msg::Pick("com.brave.Browser".into()));
    let mut filled = Vec::new();
    let mut lit = 0;
    walk(&w.view(), &mut |n| {
        if n.lit > 0.0 {
            lit += 1;
        }
        if let What::Button { tip, on: Some(true), .. } = &n.what {
            filled.push(tip.clone());
        }
    });
    assert_eq!(lit, 0, "no card has a lit edge");
    assert_eq!(filled, ["Brave, picked"]);
}

#[test]
fn each_kind_of_app_is_a_card_with_a_head() {
    let mut w = welcome();
    for (page, kinds) in [(Page::Apps, first_apps()), (Page::MoreApps, more_apps())] {
        w.go_to(at(page));
        let words = said(&w);
        for (kind, _, apps) in kinds {
            let head = words.iter().position(|s| s == kind).unwrap_or_else(|| panic!("{kind} has a head"));
            assert_eq!(words[head + 1], apps[0].1, "{kind}'s first app follows its head");
        }
    }
}

#[test]
fn install_progress_is_the_standard_ring_in_the_foot() {
    let mut w = welcome();
    w.go_to(at(Page::MoreApps));
    w.update(Msg::Pick("org.videolan.VLC".into()));
    w.update(Msg::Main);
    assert!(w.animating(), "the ring turns while the end is unknown");
    assert!(!said(&w).iter().any(|s| s.ends_with('%')));
    w.progress(0.4, None);
    assert!(!w.animating());
    assert!(said(&w).contains(&"40%".to_string()));
}

#[test]
fn a_failed_install_can_be_tried_again_with_the_same_apps() {
    let mut w = welcome();
    w.go_to(at(Page::MoreApps));
    w.update(Msg::Pick("org.gimp.GIMP".into()));
    w.update(Msg::Main);
    w.installed_now(false, "No remote refs found.".into());
    assert!(said(&w).contains(&"button: Try again".to_string()));
    w.asked.clear();
    w.update(Msg::Retry);
    assert!(w.asked.contains(&Ask::Install(vec!["org.gimp.GIMP".into()])));
    assert!(w.installing());
}

#[test]
fn every_picture_has_words() {
    fn check(n: &Node<Msg>, page: Page) {
        if let What::Drawing { size, tip, .. } = &n.what {
            assert!(size.0 < 100.0 || !tip.is_empty(), "a picture on {page:?} has no words");
        }
        n.items().iter().for_each(|i| check(i, page));
    }
    let mut w = welcome();
    for (n, page) in PAGES.iter().enumerate() {
        w.go_to(n);
        check(&w.view(), *page);
    }
}

#[test]
fn every_page_fits_without_scrolling() {
    let look = theme::welcome();
    let mut w = welcome();
    w.installed = ["org.mozilla.firefox".to_string()].into_iter().collect();
    w.picked = ["org.gimp.GIMP".to_string()].into_iter().collect();
    for (width, height) in [(look.least_width, look.least_height), (look.width, look.height), (1600.0, 1000.0)] {
        w.resized(width, height);
        for (n, page) in PAGES.iter().enumerate() {
            w.go_to(n);
            let ui = Ui::new(w.view(), Theme::new(false, "#fbc711"));
            let needs = ui.natural_height(width);
            assert!(needs <= height, "{:?} needs {needs} points at {width} by {height}", page);
        }
    }
}

#[test]
fn the_window_fits_the_screen() {
    assert_eq!(window_size((1280.0, 800.0)), (1040.0, 680.0));
    assert_eq!(window_size((2256.0 / 1.75, 1504.0 / 1.75)), (1040.0, 680.0));
    let (w, h) = window_size((1920.0 / 1.75, 1080.0 / 1.75));
    assert!(w <= 1920.0 / 1.75 && h + 32.0 <= 1080.0 / 1.75, "{w} by {h}");
    assert!(h >= theme::welcome().least_height);
}

#[test]
fn a_long_list_of_apps_ends_with_how_many_more() {
    let n = |v: &[&str]| named(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    assert_eq!(n(&["Firefox"]), "Firefox");
    assert_eq!(n(&["Firefox", "VLC", "GIMP"]), "Firefox, VLC and GIMP");
    assert_eq!(n(&["Firefox", "VLC", "GIMP", "Krita"]), "Firefox, VLC and 2 more");
}

#[test]
fn shown_once_then_from_all_apps() {
    assert!(config_path(DONE).ends_with("trindle/welcome-done"), "the same mark Apps' Get started page kept");
    let dir = std::env::temp_dir().join(format!("trindle-welcome-{}", std::process::id()));
    let mark = dir.join("trindle").join(DONE);
    mark_done_at(&mark);
    assert!(mark.is_file());
    let _ = std::fs::remove_dir_all(&dir);
    let entry = include_str!("../../share/applications/trindle-welcome.desktop");
    assert!(entry.contains("\nExec=trindle-welcome\n") && entry.contains("\nName=Welcome\n"));
    assert!(!entry.contains("NoDisplay"));
}

#[test]
#[ignore]
fn pictures_of_every_page() {
    let Some(dir) = std::env::var_os("WELCOME_SHOTS").map(PathBuf::from) else { return };
    let _ = std::fs::create_dir_all(&dir);
    for (name, screen, scale) in [("1280x800", (1280.0, 800.0), 1.0), ("2256x1504-175", (2256.0 / 1.75, 1504.0 / 1.75), 1.75),
                                  ("least", (908.0, 612.0), 1.0)] {
        let (ww, wh) = window_size(screen);
        let mut w = welcome();
        w.moving = std::env::var_os("WELCOME_MOVING").is_some();
        w.installed = ["org.mozilla.firefox".to_string()].into_iter().collect();
        w.picked = ["org.videolan.VLC".to_string(), "com.brave.Browser".to_string()].into_iter().collect();
        w.resized(ww, wh);
        let place = crate::common::files::home().join(".local/share/flatpak/appstream/flathub/x86_64/active");
        if let Ok(apps) = catalogue::parse(&place) {
            w.icons = every_app().filter_map(|(id, _)| apps.get(id)?.icon.clone().map(|p| (id.to_string(), p))).collect();
        }
        let shoot = |w: &Welcome, file: PathBuf| {
            let mut ui = Ui::new(w.view(), Theme::chosen());
            let (pw, ph) = ((ww * scale).round() as u32, (wh * scale).round() as u32);
            let mut px = tiny_skia::Pixmap::new(pw, ph).expect("pixmap");
            ui.paint(&mut px.as_mut(), scale);
            let rgba: Vec<u8> = px.pixels().iter().flat_map(|p| {
                let c = p.demultiply();
                [c.red(), c.green(), c.blue(), c.alpha()]
            }).collect();
            image::save_buffer_with_format(&file, &rgba, pw, ph, image::ExtendedColorType::Rgba8, image::ImageFormat::Png)
                .expect("saved");
        };
        for (n, page) in PAGES.iter().enumerate() {
            w.go_to(n);
            shoot(&w, dir.join(format!("welcome-{}-{:?}-{name}.png", n + 1, page).to_lowercase()));
            if page.moves() && scale == 1.0 && std::env::var_os("WELCOME_MOMENTS").is_some() {
                w.moving = true;
                for t in [0.8, 1.5, 1.7, 2.4, 3.2, 3.9, 4.6, 6.6] {
                    w.held = Some(t);
                    shoot(&w, dir.join(format!("moment-{:?}-{t}.png", page).to_lowercase()));
                }
                w.moving = false;
                w.held = None;
            }
        }
        w.go_to(at(Page::Plugins));
        w.picked.clear();
        w.install = Install::Running(vec!["com.brave.Browser".into(), "org.videolan.VLC".into()], 0.4,
                                     Some("about 2 minutes left".into()));
        shoot(&w, dir.join(format!("welcome-installing-{name}.png")));
        w.installed_now(true, String::new());
        w.go_to(at(Page::MoreApps));
        shoot(&w, dir.join(format!("welcome-installed-{name}.png")));
    }
}

#[test]
fn the_plugins_page_shows_what_the_catalogue_offers_and_names_none_itself() {
    use crate::plugins::catalogue::{Entry, Offer};
    let offer = |n: &str| Offer {
        entry: Entry { id: format!("org.example.{n}"), name: n.into(), version: "1.0".into(), summary: format!("{n} does a thing."),
                       permissions: Vec::new(), icon: None, icon_sha256: None, archive: String::new(), sha256: String::new() },
        icon: None,
    };
    let mut w = welcome();
    w.go_to(at(Page::Plugins));
    let before = said(&w).join("\n");
    assert!(before.contains(no_plugins_yet()), "{before}");
    w.plugins = offered(&["One", "Two", "Three", "Four"].map(offer));
    let after = said(&w).join("\n");
    for n in ["One", "Two", "Three"] {
        assert!(after.contains(&format!("{n} does a thing.")), "{after}");
    }
    assert!(!after.contains("Four") && !after.contains(no_plugins_yet()), "{after}");
}

#[test]
fn welcome_draws_only_pairs_the_table_holds() {
    use crate::plugins::catalogue::{Entry, Offer};
    use crate::toolkit::theme::drawn;
    let offer = |n: &str| Offer {
        entry: Entry { id: format!("org.example.{n}"), name: n.into(), version: "1.0".into(), summary: format!("{n} does a thing."),
                       permissions: Vec::new(), icon: None, icon_sha256: None, archive: String::new(), sha256: String::new() },
        icon: None,
    };
    let w = std::cell::RefCell::new(welcome());
    w.borrow_mut().plugins = offered(&["One", "Two", "Three"].map(offer));
    drawn::none(drawn::in_every_look("Welcome", 60, || {
        let mut w = w.borrow_mut();
        let mut pix = tiny_skia::Pixmap::new(1100, 760).unwrap();
        for page in 0..PAGES.len() {
            w.go_to(page);
            drawn::page(&format!("page {} of Welcome", page + 1));
            let mut ui = Ui::new(w.view(), Theme::for_page("welcome"));
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }));
}

#[test]
fn the_dots_page_shows_each_kind_with_its_words_and_where_dots_appear() {
    use crate::common::in_use::Kind;
    let mut w = welcome();
    w.go_to(at(Page::Dots));
    let words = said(&w);
    for kind in Kind::ALL {
        assert!(words.contains(&kind.spoken(&[])), "{kind:?} is said");
    }
    assert!(words.contains(&t!("common-in-use-key").to_string()));
    assert!(!pictures::dots_words().contains("favourite"), "the drawn example shows the tab and the corner only");
    assert_eq!(t!("welcome-dots-where-tab"), "On a window's tab");
    assert_eq!(t!("welcome-dots-where-corner"), "Top middle of the screen");
}

fn dots_sizes() -> Vec<((u32, u32), f32, &'static str)> {
    let look = theme::welcome();
    let size = |screen: (f32, f32)| {
        let w = window_size(screen);
        (w.0 as u32, w.1 as u32)
    };
    let least = (look.least_width as u32, look.least_height as u32);
    vec![(size((1280.0, 800.0)), 1.0, "1280x800"), (size((2256.0 / 1.75, 1504.0 / 1.75)), 1.75, "2256x1504-175"), (least, 1.0, "least")]
}

fn paint_dots(size: (u32, u32), scale: f32, light: bool) -> tiny_skia::Pixmap {
    let mut w = welcome();
    w.size = (size.0 as f32, size.1 as f32);
    w.go_to(at(Page::Dots));
    let mut ui = Ui::new(w.view(), Theme::new(light, &crate::common::style::accent()));
    let mut pix = tiny_skia::Pixmap::new((size.0 as f32 * scale) as u32, (size.1 as f32 * scale) as u32).unwrap();
    ui.paint(&mut pix.as_mut(), scale);
    pix
}

#[test]
fn the_dots_page_fills_its_card_at_every_size() {
    use crate::common::in_use::Kind;
    for light in [false, true] {
        for (size, scale, name) in dots_sizes() {
            let pix = paint_dots(size, scale, light);
            let (w, h) = (pix.width() as usize, pix.height() as usize);
            let (top, bottom) = (h * 6 / 10, h - (60.0 * scale) as usize);
            let mut colours = std::collections::HashSet::new();
            let mut kinds = 0;
            for y in top..bottom {
                for x in w / 20..w - w / 20 {
                    let p = pix.pixel(x as u32, y as u32).unwrap();
                    colours.insert((p.red() / 8, p.green() / 8, p.blue() / 8));
                }
            }
            for kind in [Kind::Microphone, Kind::Camera] {
                let (r, g, b) = kind.colour_in(light);
                let want = [(r * 255.0).round() as i32, (g * 255.0).round() as i32, (b * 255.0).round() as i32];
                let found = (top..bottom).any(|y| (0..w).any(|x| {
                    let p = pix.pixel(x as u32, y as u32).unwrap();
                    p.alpha() == 255 && [p.red() as i32, p.green() as i32, p.blue() as i32].iter().zip(want).all(|(a, b)| (a - b).abs() <= 3)
                }));
                kinds += found as usize;
            }
            assert!(colours.len() > 6, "{name} light {light}: the lower card is empty");
            assert_eq!(kinds, 2, "{name} light {light}: the example's dots are drawn in the table's colours");
        }
    }
}

#[test]
#[ignore]
fn shoot_the_dots() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-dots".into()));
    std::fs::create_dir_all(&out).unwrap();
    for (size, scale, name) in dots_sizes() {
        for (light, look) in [(false, "dark"), (true, "light")] {
            paint_dots(size, scale, light).save_png(out.join(format!("welcome-7-dots-{name}-{look}.png"))).unwrap();
        }
    }
}

#[test]
fn the_dots_are_said_in_the_spec_help_and_settings_from_the_one_table() {
    use crate::common::in_use::Kind;
    let Ok(spec) = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/spec/trindle.md")) else { return };
    assert!(spec.contains("the Permissions page has a card at its foot, \"What the dots mean\""));
    let help = crate::common::help_pages::page("in-use-dots");
    for kind in Kind::ALL {
        assert!(help.contains(&kind.spoken(&[])), "Help says {kind:?}");
    }
    assert!(help.contains("<!-- screenshot: in-use-dots-") || help.contains("img/in-use-dots-key.png"));
    assert!(!help.contains("Under a favourite"), "Help puts no dots under a favourite");
    assert_eq!(t!("common-in-use-key"), "What the dots mean");
    let settings = crate::settings::ui::in_use_key::<Msg>();
    let welcome_rows = crate::settings::ui::in_use_key::<Msg>();
    assert_eq!(settings.len(), Kind::ALL.len());
    assert_eq!(settings.len(), welcome_rows.len());
}

#[test]
fn a_dot_is_drawn_in_the_colour_of_its_kind_in_either_look() {
    use crate::common::in_use::Kind;
    for light in [false, true] {
        for kind in Kind::ALL {
            let mut ui = Ui::new(crate::settings::ui::in_use_dot::<Msg>(kind), Theme::new(light, &crate::common::style::accent()));
            let mut pix = tiny_skia::Pixmap::new(40, 40).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            let (r, g, b) = kind.colour_in(light);
            let want = [(r * 255.0).round() as i32, (g * 255.0).round() as i32, (b * 255.0).round() as i32];
            let found = pix.pixels().iter().any(|p| p.alpha() == 255
                && [p.red() as i32, p.green() as i32, p.blue() as i32].iter().zip(want).all(|(a, b)| (a - b).abs() <= 2));
            assert!(found, "{kind:?} light {light}");
        }
    }
}

#[test]
fn the_first_page_asks_how_much_you_know() {
    let w = welcome();
    assert_eq!(w.here(), Page::Level);
    let said = said(&w).join("|");
    for level in Level::ALL {
        assert!(said.contains(level.card().1), "{level:?} is offered by its words");
    }
}

#[test]
fn new_to_linux_sees_every_page() {
    let mut w = welcome();
    w.update(Msg::Level(Level::New));
    assert_eq!(w.pages(), PAGES.to_vec());
    assert_eq!(w.here(), Page::Hello, "the answer moves on");
}

#[test]
fn used_linux_skips_every_linux_basics_page_and_keeps_the_rest() {
    let mut w = welcome();
    w.update(Msg::Level(Level::Linux));
    let shown = w.pages();
    assert!(shown.iter().all(|p| p.mark() == Mark::Trindle));
    for p in PAGES.iter().filter(|p| p.mark() == Mark::Trindle) {
        assert!(shown.contains(p), "{p:?} is Trindle's own and always shown");
    }
    for own in [Page::Ring, Page::Dots, Page::Plugins, Page::Snapshots, Page::Words, Page::Know] {
        assert!(shown.contains(&own), "{own:?}");
    }
    assert_eq!(w.here(), Page::Hello);
    w.update(Msg::Next);
    assert!(said(&w).contains(&format!("Step 3 of {}", shown.len())), "the step count follows the pages shown");
    for _ in 0..shown.len() {
        w.update(Msg::Next);
    }
    assert_eq!(w.here(), Page::Know);
    assert_eq!(w.main_words(), "Start using Trindle");
}

#[test]
fn used_trindle_closes_the_tour_and_marks_it_seen() {
    let mut w = welcome();
    w.update(Msg::Level(Level::Trindle));
    assert_eq!(w.asked, vec![Ask::Seen, Ask::Close]);
    assert_eq!(w.page, 0, "it does not go on");
}

#[test]
fn every_page_is_listed_once_and_both_marks_are_used() {
    let mut seen = Vec::new();
    for p in PAGES.iter() {
        assert!(!seen.contains(&p), "{p:?} is listed twice");
        seen.push(p);
    }
    assert!(PAGES.iter().any(|p| p.mark() == Mark::LinuxBasics) && PAGES.iter().any(|p| p.mark() == Mark::Trindle));
}

#[test]
fn your_own_words_is_a_page_with_a_way_to_settings_and_a_help_page() {
    let mut w = welcome();
    w.go_to(at(Page::Words));
    let said = said(&w);
    assert!(said.contains(&"Your own words".to_string()));
    assert!(said.iter().any(|s| s == "button: Open Region and language"));
    w.update(Msg::Words);
    assert_eq!(w.asked.last(), Some(&Ask::Words));
    let help = crate::common::help_pages::page("own-words");
    assert!(help.lines().any(|l| l == "title: Your own words"), "Help has the own-words page");
    assert!(help.contains("Region and language"), "it says where in Settings");
    assert!(help.lines().any(|l| l.starts_with("## ")), "it has something to read");
}
