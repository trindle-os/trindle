//! Tests of the ring's view.

use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::{PixmapPaint, Transform};

use crate::ring::open_windows;
use crate::ring::ctx::{msg, Msg, Program};
use crate::ring::layout::{button_size, CENTRE, layout, Layout, MARGIN};
use crate::ring::open_windows::Look;
use crate::toolkit::colour::Colour;
use crate::toolkit::text::Style;
use crate::toolkit::widget::{button, Length, row};

include!("../site_shots_test.rs");

use super::Face;
use super::app::look;
use super::app_tile::{app_button, paint_app};
use super::arrange::{found, less_word, rank_found};
use super::middle::{ALL_ICON, BATTERY_TEXT, centre_of, core, SEGMENT, strip_of, time_and_date_drawn, tool_rows};
use super::paint::{paint_ring, Scene};
use super::pinned::Ink;
use super::power::{Ends, ends_session, escape_ends, OFFER_SECONDS, only_a_click_confirms, Power, power_step, spoken_step};

fn method<'a>(code: &'a str, head: &str) -> &'a str {
    let rest = &code[code.find(head).unwrap_or_else(|| panic!("{head} is not in the ring's view"))..];
    &rest[..rest.find("\n    }\n").expect("the end of it")]
}

pub(in crate::ring) fn view_code() -> &'static str {
    static CODE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CODE.get_or_init(|| {
        let files = [include_str!("app.rs"), include_str!("app_tile.rs"), include_str!("arrange.rs"), include_str!("launch.rs"), include_str!("menu.rs"),
                     include_str!("middle.rs"), include_str!("paint.rs"), include_str!("panel.rs"), include_str!("pinned.rs"), include_str!("power.rs"), include_str!("voice.rs"),
                     include_str!("window_ring.rs"), include_str!("mod.rs")];
        files.iter().map(|text| crate::common::source::code(text).split("\n#[cfg(test)]\nmod tests {").next().unwrap_or_default().replace("#[cfg(test)]\nmod tests;\n", "")).collect::<Vec<_>>().join("\n")
    })
}

#[test]
fn one_escape_on_the_power_page_goes_straight_back() {
    for (win, menu, quick, typed) in (0..16).map(|n| (n & 1 != 0, n & 2 != 0, n & 4 != 0, n & 8 != 0)) {
        assert_eq!(escape_ends(true, win, menu, quick, typed), Ends::Power, "{win} {menu} {quick} {typed}");
    }
    assert_eq!(escape_ends(false, true, true, true, true), Ends::WindowMenu);
    assert_eq!(escape_ends(false, false, true, true, true), Ends::Menu);
    assert_eq!(escape_ends(false, false, false, true, true), Ends::Quick);
    assert_eq!(escape_ends(false, false, false, false, true), Ends::Typed);
    assert_eq!(escape_ends(false, false, false, false, false), Ends::Ring);

    let view = view_code();
    let before_tests = view;
    let power = before_tests.split("Ends::Power => {").nth(1).unwrap().split("Ends::WindowMenu").next().unwrap();
    assert!(power.contains("self.win_menu = None;") && power.contains("self.show_power(false);"));
}

#[test]
fn no_way_out_of_the_session_is_taken_or_armed_without_the_persons_own_press() {
    for what in ["Suspend", "Hibernate", "logout", "Reboot", "PowerOff"] {

        assert_eq!(power_step(what, None, false), Power::Refuse, "{what} was armed by a press that was not the person's");
        assert_eq!(power_step(what, Some(what), false), Power::Refuse, "{what} was run by a press that was not the person's");

        assert_eq!(power_step(what, None, true), Power::Arm);
        assert_eq!(power_step(what, Some("lock"), true), Power::Arm);
        assert_eq!(power_step(what, Some(what), true), Power::Run);
    }
    for what in ["lock", "screensaver"] {
        assert_eq!(power_step(what, None, false), Power::Run);
        assert_eq!(power_step(what, None, true), Power::Run);
    }
}

#[test]
fn a_choice_a_command_offered_is_confirmed_only_by_a_click() {
    use crate::toolkit::window::{key_now, note_press_code, with_press};
    for what in ["Suspend", "Hibernate", "logout", "Reboot", "PowerOff"] {
        let second = power_step(what, Some(what), true);
        assert_eq!(second, Power::Run);
        assert_eq!(spoken_step(second, true, false), Power::NeedsClick, "{what}: a key confirmed what a command offered");
        assert_eq!(spoken_step(second, true, true), Power::Run, "{what}: a click did not confirm what a command offered");

        assert_eq!(spoken_step(second, false, false), Power::Run);

        assert_eq!(spoken_step(power_step(what, Some(what), false), true, true), Power::Refuse);
        assert_eq!(spoken_step(power_step(what, None, true), true, false), Power::Arm);
    }

    note_press_code(8801, 28);
    note_press_code(8802, 0x110);
    assert!(with_press(Some(8801), key_now));
    assert!(!with_press(Some(8802), key_now));
    assert!(!with_press(None, key_now), "what no press brought about is not a key");

    let code = view_code();
    let confirm = method(code, "fn confirm(");
    assert!(confirm.contains("self.arm(what, OFFER_SECONDS);") && confirm.contains("self.offered = true;"));
    assert_eq!(code.matches("self.offered = true;").count(), 1);
    assert!((3..=10).contains(&OFFER_SECONDS));
    let choice = method(code, "fn power_choice");
    assert!(choice.contains("spoken_step(step, self.offered, kept::clicked())") && choice.contains("self.say(only_a_click_confirms()"));
    assert!(only_a_click_confirms().len() < 60 && !only_a_click_confirms().contains('\u{2014}'));
}

#[test]
fn a_refused_power_press_says_why_and_a_made_up_key_closes_the_page() {
    let view = view_code();
    let before_tests = view;
    let choice = method(before_tests, "fn power_choice");
    assert!(choice.contains("Power::Refuse => {") && choice.contains("self.say(only_your_own_power(), REFUSED_SECONDS);"), "a refused power choice says nothing");
    let intercept = method(before_tests, "fn intercept(&self");
    let closes = intercept.find("if self.power && matches!(input, Input::Key { .. }) && !kept::own_press() {").expect("the power page does not close on a made-up key");
    assert!(intercept[closes..].contains("f.show_power(false);") && intercept[closes..].contains("f.say(power_closed(), REFUSED_SECONDS);"));
    assert!(closes < intercept.find("if self.quick.is_open() || self.power").unwrap(), "the made-up key is looked at after the power page took it");
    for words in [super::power::only_your_own_power(), super::power::power_closed()] {
        assert!(words.len() < 60 && !words.contains('\u{2014}'), "{words}");
    }
}

#[test]
fn every_way_out_of_the_session_goes_through_the_one_rule() {
    let view = view_code();
    let before_tests = view;
    assert_eq!(before_tests.matches("session_end(&self.ctx").count(), 1, "the session is ended from a second place in the ring's view");
    let choice = method(before_tests, "fn power_choice");
    assert!(choice.contains("power_step(") && choice.contains("kept::own_press()") && choice.contains("session_end(&self.ctx"));
    use crate::common::source::code;
    for (name, code) in [("lid.rs", code(include_str!("../lid.rs"))), ("quick", crate::ring::quick::tests::quick_code()), ("mod.rs", code(include_str!("../mod.rs"))),
                         ("notes", crate::ring::notes::tests::notes_code()),
                         ("plugins.rs", code(include_str!("../plugins.rs")))] {

        let most = if name == "lid.rs" { 1 } else { 0 };
        assert!(code.matches("session_end(").count() <= most, "{name} ends the session by a way of its own");
    }
}

#[test]
fn nothing_on_the_way_to_the_rings_panel_waits_on_a_folder() {
    let code = view_code();
    let body = |from: &str| {
        let at = code.find(from).unwrap_or_else(|| panic!("{from} is not in the ring's view"));
        let rest = &code[at..];
        &rest[..rest.find("\n    }\n").expect("the end of it")]
    };

    let waits = ["crate::common::looks::looks", "found_outside(", "path_kind(", "appimages::remove(", ".exists()", ".is_dir()", ".is_file()", "metadata(", "read_dir("];

    for part in ["fn opened(&mut self", "fn all(&self)", "fn open_fresh(&self)", "fn relayout(&mut self)",
                 "fn open_looked(&mut self", "fn looked(&mut self", "fn look_afresh(&mut self", "fn look_overdue(&mut self",
                 "fn images(&self)", "fn menu_card(&self"] {
        let body = body(part);
        for wait in waits {
            assert!(!body.contains(wait), "{}: {wait} may wait on a folder that has stopped answering, on the ring's own loop", part.trim());
        }
    }

    for (part, handed) in [("fn look_aside(&self", "on_thread(move || found_outside("), ("fn open_path(&mut self", "on_thread(move || path_kind("),
                           ("fn start_removal(", "on_thread(move || on_disk(crate::icons::appimages::remove("), ("fn uninstall(&mut self", "Self::start_removal("),
                           ("fn open_menu(&mut self", "on_thread(move || removal::how(")] {
        assert!(body(part).contains(handed), "{} no longer hands its look to a thread", part.trim());
    }
    for pure in ["fn image_of(", "fn stamps_kept("] {
        let at = code.find(pure).unwrap_or_else(|| panic!("{pure} is not in the ring's view"));
        let body = &code[at..][..code[at..].find("\n}\n").expect("the end of it")];
        for wait in waits {
            assert!(!body.contains(wait), "{pure}: {wait} may wait on a folder that has stopped answering, on the ring's own loop");
        }
    }
}

#[test]
fn ctrl_backspace_takes_off_the_last_word() {
    assert_eq!(less_word("visual studio"), "visual ");
    assert_eq!(less_word("visual studio  "), "visual ");
    assert_eq!(less_word("firefox"), "");
    assert_eq!(less_word(""), "");
    assert_eq!(less_word("café crème"), "café ");
    assert_eq!(less_word("visual\u{a0}studio"), "visual\u{a0}");
    assert_eq!(less_word("日本\u{3000}語"), "日本\u{3000}");
    assert_eq!(less_word("\u{3000}"), "");
    assert_eq!(less_word("é\u{2003}ü\u{2003}"), "é\u{2003}");
}

#[test]
fn only_the_ways_out_ask_twice() {
    for what in ["Suspend", "Hibernate", "logout", "Reboot", "PowerOff"] {
        assert!(ends_session(what), "{what}");
    }
    assert!(ends_session("Anything") && ends_session(""), "a choice not on the safe list ran with no press of the person's own");
    assert!(!ends_session("lock"));
    assert!(!ends_session("screensaver"));
}

#[test]
fn start_at_login_asks_twice_to_add_and_once_to_take_off() {
    use crate::common::{autostart, scratch_home};
    use crate::toolkit::testing::Harness;
    use crate::toolkit::ui::App;
    struct Probe { launcher: std::path::PathBuf, delivered: u32 }
    impl App for Probe {
        type Msg = Msg;
        fn update(&mut self, _: Msg) { self.delivered += 1 }
        fn view(&self) -> crate::toolkit::widget::Node<Msg> { super::menu::startup_button(&self.launcher) }
    }
    let Some(home) = scratch_home::inside() else { return scratch_home::rerun(module_path!(), "start_at_login_asks_twice_to_add_and_once_to_take_off") };
    let launcher = home.join("notes.desktop");
    std::fs::write(&launcher, "[Desktop Entry]\nName=Notes\nExec=notes %U\nType=Application\n").unwrap();
    let mut off = Harness::new(Probe { launcher: launcher.clone(), delivered: 0 }, (200, 80));
    assert!(off.click("Start at login"));
    assert_eq!(off.app().delivered, 0, "the first press only asks");
    let again = off.text().into_iter().find(|w| w.to_lowercase().contains("again")).unwrap_or_else(|| panic!("{:?}", off.text()));
    assert!(off.click(&again));
    assert_eq!(off.app().delivered, 1, "the second press goes through");

    assert!(autostart::list().is_empty());
    assert!(autostart::toggle_app(&launcher).unwrap());
    let listed = autostart::list();
    assert_eq!((listed.len(), listed[0].name.as_str(), listed[0].exec.as_str(), listed[0].enabled), (1, "Notes", "notes %U", true));
    let mut on = Harness::new(Probe { launcher: launcher.clone(), delivered: 0 }, (200, 80));
    assert!(!on.click("Start at login"), "it is on, so it is offered off");
    assert!(on.click("Stop starting at login"));
    assert_eq!(on.app().delivered, 1, "taking it off is one press");
    assert!(!autostart::toggle_app(&launcher).unwrap());
    assert!(autostart::list().is_empty());
    let code = view_code();
    assert!(code.contains("if !favourites::is_path(&app.id) {\n            tools.push(startup_button(&app.path));"), "an app's menu");
    assert!(code.contains("tools.push(startup_button(&app.path).size(size))"), "an open window's menu");
}

fn names(found: Vec<usize>, apps: &[&str]) -> Vec<String> {
    found.into_iter().map(|i| apps[i].to_string()).collect()
}

#[test]
fn typed_letters_find_only_apps_whose_names_start_with_them() {
    let apps = ["Firefox", "LibreOffice", "Calibre", "Lantern", "Lollypop", "Slim Editor", "Audacity"];
    assert_eq!(names(rank_found("L", &apps), &apps), ["Lantern", "LibreOffice", "Lollypop"]);
    assert_eq!(names(rank_found("li", &apps), &apps), ["LibreOffice"]);
    assert_eq!(names(rank_found("  fire ", &apps), &apps), ["Firefox"]);
    assert!(rank_found("editor", &apps).is_empty(), "a word later in a name is not a match");
    assert!(rank_found("zzz", &apps).is_empty());
    assert!(rank_found("   ", &apps).is_empty());
}

#[test]
fn the_report_button_is_always_in_the_lower_row_in_the_beta() {
    let lower: Vec<_> = super::middle::lower_tools().iter().map(|t| t.0).collect();
    assert_eq!(lower.last(), Some(&"report"), "{lower:?}");
}

#[test]
fn the_eye_dropper_is_in_the_lower_row_after_the_magnifier() {
    let lower: Vec<_> = tool_rows()[1].iter().map(|(k, i, t)| (*k, *i, *t)).collect();
    assert_eq!(lower.last(), Some(&("report", "dialog-warning-symbolic", "Report a problem")));
    assert_eq!(lower[lower.len() - 2], ("eye-dropper", "color-select-symbolic", "Eye dropper"));
    assert_eq!(lower[lower.len() - 3].0, "magnifier");
}

#[test]
fn the_ring_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("the ring", 20, || {
        drawn::page("open");
        let _ = ring_open();
    }));
}

fn ring_open() -> tiny_skia::Pixmap {
    let apps = ["firefox", "thunderbird", "org.gnome.Calculator", "vlc", "gimp", "libreoffice-writer", "org.gnome.Nautilus", "inkscape",
                "audacity", "org.gnome.TextEditor"];
    let names = ["Firefox", "Thunderbird", "Calculator", "VLC", "GIMP", "Writer", "Folders", "Inkscape", "Audacity", "Text"];
    ring_open_at((1280.0, 800.0), 1.0, &apps, &names, 3, 0)
}

const LAPTOP_APPS: [&str; 6] = ["system-file-manager", "drive-harddisk", "com.brave.Browser", "com.anthropic.Claude", "lantern", "md.obsidian.Obsidian"];
const LAPTOP_NAMES: [&str; 6] = ["Folders", "Drives", "Brave", "Claude", "Lantern", "Obsidian"];

const SCREENS: [(f32, f32, f32); 7] = [(1289.0, 859.0, 1.75), (1920.0, 1080.0, 1.0), (1536.0, 864.0, 1.25), (1366.0, 768.0, 1.0),
                                       (1706.0, 1066.0, 1.5), (1280.0, 800.0, 2.0), (1920.0, 1080.0, 2.0)];

fn ring_tree(size: (f32, f32), apps: &[&'static str], names: &[&'static str], status: usize, tray: usize) -> (crate::toolkit::widget::Node<Msg>, std::rc::Rc<Layout<usize>>) {
    ring_tree_with(size, apps, names, status, tray, &[])
}

fn ring_tree_with(size: (f32, f32), apps: &[&'static str], names: &[&'static str], status: usize, tray: usize,
                  wins: &[(&'static str, &'static str, bool, bool)])
                  -> (crate::toolkit::widget::Node<Msg>, std::rc::Rc<Layout<usize>>) {
    ring_tree_found(size, apps, names, status, tray, wins, false)
}

thread_local! {

    static TAB_USES: std::cell::RefCell<Vec<(usize, Vec<crate::common::in_use::Kind>)>> = const { std::cell::RefCell::new(Vec::new()) };

    static POINTED: std::cell::RefCell<Option<(usize, Option<std::rc::Rc<tiny_skia::Pixmap>>)>> = const { std::cell::RefCell::new(None) };

    static SHOW_TITLE: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
}

#[allow(clippy::too_many_arguments)]
fn ring_tree_found((w, h): (f32, f32), apps: &[&'static str], names: &[&'static str], status: usize, tray: usize,
                  wins: &[(&'static str, &'static str, bool, bool)], searching: bool)
                  -> (crate::toolkit::widget::Node<Msg>, std::rc::Rc<Layout<usize>>) {
    use crate::toolkit::widget::{drawing, layers, placed, Node};
    let (apps, names) = (apps.to_vec(), names.to_vec());
    let wins = open_windows::grouped(wins.to_vec(), |w| if w.0.contains("brave") { "brave".to_string() } else { w.0.to_string() });
    let wins = &wins[..];
    let first: Vec<usize> = (0..apps.len()).collect();
    let k = open_windows::Sizes::of(crate::toolkit::theme::sheet());
    let lay = Rc::new(if searching {
        crate::ring::layout::search_ring(&first, w as f64, h as f64, k.gap + k.width)
    } else {
        layout::<usize>(&first, &[], w as f64, h as f64)
    });
    let (cx, cy) = (w / 2.0, h / 2.0);
    let back = lay.clone();
    let dim = crate::common::style::palette().dim as f32;
    let mut all: Vec<Node<Msg>> = vec![drawing((w, h), "", move |pm, pen| paint_ring(pm, pen, &back, &Scene { seg: searching, top: Some(false), bare: POINTED.with(|p| p.borrow().is_some()), listening: false, pulse: 0.0, empty: false, all: searching, dim }))];
    push_windows_ring(&mut all, wins, &lay, (w, h));
    let (upper, status, lower, foot) = tool_columns(status, tray);
    let clock = time_and_date_drawn("14:32".into(), "Wed 14 Oct".into()).key("clock");
    let pointed = POINTED.with(|p| p.borrow().clone());
    match &pointed {
        Some((i, picture)) => {
            let icons = vec![format!("/usr/share/icons/Trindle-Circle/48x48/apps/{}.svg", wins[*i].0)];
            let m = open_windows::middle(CENTRE as f32, open_windows::Middle { name: wins[*i].1.into(), show_title: SHOW_TITLE.with(|s| s.get()), title: open_windows::without_program(wins[*i].1, if wins[*i].0.contains("brave") { "Brave" } else { wins[*i].0 }), icons, picture: picture.clone() });
            all.push(placed(m, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0)));
        }
        None => all.push(placed(centre_of(upper, status, clock, lower, foot), |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0))),
    }
    push_favourites(&mut all, &lay, &apps, &names);
    let m = CENTRE / 2.0;
    let half = (m * m - (m - SEGMENT).powi(2)).sqrt() as f32;
    let (top, foot) = (cy - m as f32, cy + m as f32 - SEGMENT as f32);
    if pointed.is_none() {
        all.push(placed(band_icon(half, "battery-level-80-symbolic", Some("82%")).key("top-band"), move |_, _| (cx - half, top)));
        all.push(placed(band_icon(half, "view-app-grid-symbolic", None).key("foot-band"), move |_, _| (cx - half, foot)));
    }
    (layers(all).width(Length::Fixed(w)).height(Length::Fixed(h)), lay)
}

fn push_windows_ring(all: &mut Vec<crate::toolkit::widget::Node<Msg>>, wins: &[(&'static str, &'static str, bool, bool)], lay: &Layout<usize>, (w, h): (f32, f32)) {
    use crate::toolkit::widget::{drawing, placed, Node};
    let (cx, cy) = (w / 2.0, h / 2.0);
    if let Some(rg) = open_windows::ring(wins.len(), lay.disc_radius, (w.min(h) / 2.0 - MARGIN as f32) as f64, &open_windows::Sizes::of(crate::toolkit::theme::sheet())) {
        let looks: Vec<Look> = wins.iter().map(|(icon, title, front, hot)| Look {
            program: { let last = icon.rsplit('.').next().unwrap_or(icon); let mut c = last.chars(); c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default() }, icons: vec![format!("/usr/share/icons/Trindle-Circle/48x48/apps/{icon}.svg")], name: title.to_string(), front: *front, hot: *hot, uses: Vec::new() }).enumerate().map(|(i, mut l)| { l.uses = TAB_USES.with(|u| u.borrow().iter().find(|(n, _)| *n == i).map(|(_, k)| k.clone()).unwrap_or_default()); l }).collect();
        let (r, l) = (rg.clone(), looks.clone());
        let dot = crate::toolkit::theme::sheet().ring_windows_dot;
        all.push(drawing((w, h), "Open windows", move |pm, pen| open_windows::paint(pm, pen, &r, &l, (w / 2.0, h / 2.0), dot)));
        for (i, look) in looks.iter().enumerate() {
            let b = rg.button_of(i);
            let node: Node<Msg> = open_windows::tab_node(&rg, i, look).key(format!("win-{i}"));
            let at = (cx + b[0] as f32, cy + b[1] as f32);
            all.push(placed(node, move |_, _| at));
        }
    }
}

fn tool_columns(status: usize, tray: usize) -> (Vec<crate::toolkit::widget::Node<Msg>>, Vec<crate::toolkit::widget::Node<Msg>>, Vec<crate::toolkit::widget::Node<Msg>>, Option<crate::toolkit::widget::Node<Msg>>) {
    use crate::toolkit::widget::Node;
    let tool = |band: &str, n: usize, icon: &str, tip: &str, job: crate::ring::Job| button(icon, tip).size(core()).tinted(job.share()).on_press(msg(|_| {})).key(format!("tool-{band}-{n}"));
    let upper: Vec<Node<Msg>> = tool_rows()[0].iter().map(|(_, i, t)| (*i, *t))
        .chain([("software-update-available-symbolic", "Updates")])
        .enumerate().map(|(n, (i, t))| tool("upper", n, i, t, crate::ring::Job::Programs)).collect();
    let shown = [("audio-volume-high-symbolic", "Sound"), ("audio-input-microphone-symbolic", "Microphone"), ("bluetooth-active-symbolic", "Bluetooth"),
                 ("network-wireless-signal-good-symbolic", "Wi-Fi"), ("display-brightness-symbolic", "Screen brightness"),
                 ("preferences-desktop-screensaver-symbolic", "Keep awake")];
    let status: Vec<Node<Msg>> = std::iter::once(&("system-shutdown-symbolic", "Power")).chain(shown.iter().take(status))
        .enumerate().map(|(n, (i, t))| tool("status", n, i, t, crate::ring::Job::Computer)).collect();
    let lower: Vec<Node<Msg>> = tool_rows()[1].iter().map(|(_, i, t)| (*i, *t)).chain([("input-keyboard-symbolic", "Keyboard")])
        .enumerate().map(|(n, (i, t))| tool("lower", n, i, t, crate::ring::Job::Tools)).collect();

    let foot = (tray > 0).then(|| {
        let gap = crate::ring::sni::row_gap();

        let icons: Vec<Node<Msg>> = (0..tray).map(|n| {
            let look = crate::ring::sni::look::Look::shape(std::rc::Rc::new(crate::toolkit::icon::Icons::new()), crate::common::apps::generic_icon_for("org.example.Flow", "Flow"));
            tool("tray", n, "", "A program", crate::ring::Job::Tray).face(move |c, r, ink, pen| look.paint(c, r, ink, pen))
        }).collect();
        strip_of(row(icons).gap(gap).width(Length::Shrink), tray as f32 * core() + (tray as f32 - 1.0) * gap)
    });
    (upper, status, lower, foot)
}

fn push_favourites(all: &mut Vec<crate::toolkit::widget::Node<Msg>>, lay: &Layout<usize>, apps: &[&'static str], names: &[&'static str]) {
    use crate::toolkit::widget::{drawing, placed, Node};
    for p in lay.placed.iter() {
        let (bw, bh) = button_size(lay.icon, lay.names);
        let at = [p.x as f32 - bw as f32 / 2.0, p.y as f32 - bh as f32 / 2.0, bw as f32, bh as f32];
        let icons = vec![format!("/usr/share/icons/Trindle-Circle/48x48/apps/{}.svg", apps[p.item])];
        let (icon, shows, name) = (lay.icon, lay.names, names[p.item]);
        let (best, lit) = (p.item == 0, p.item == 3);
        let node: Node<Msg> = drawing((at[2], at[3]), name, move |pm, pen| paint_app(pm, pen, &icons, name, icon, shows, best, lit, Ink::Usual))
            .key(format!("fav-{}", p.item));
        all.push(placed(node, move |_, _| (at[0], at[1])));
    }
}

fn band_icon(half: f32, icon: &'static str, words: Option<&'static str>) -> crate::toolkit::widget::Node<Msg> {
    use crate::toolkit::widget::drawing;
    drawing((2.0 * half, SEGMENT as f32), "", move |canvas, pen| {
        let s = pen.scale;
        let size = (ALL_ICON * s).round();
        let said = words.map(|t| pen.fonts.layout(t, &Style::new(BATTERY_TEXT * s), None));
        let gap = (4.0 * s).round();
        let wide = size + said.as_ref().map_or(0.0, |l| gap + l.width);
        let x = ((canvas.width() as f32 - wide) / 2.0).round();
        let y = ((canvas.height() as f32 - size) / 2.0).round();
        pen.icons.draw(canvas, icon, x as i32, y as i32, size as u32, 1, pen.theme.fg);
        if let Some(l) = said {
            pen.fonts.draw(canvas, &l, x + size + gap, (canvas.height() as f32 - l.height) / 2.0, pen.theme.fg);
        }
    })
}

fn ring_open_at((w, h): (f32, f32), scale: f32, apps: &[&'static str], names: &[&'static str], status: usize, tray: usize) -> tiny_skia::Pixmap {
    ring_open_with_at((w, h), scale, apps, names, status, tray, &[])
}

#[allow(clippy::too_many_arguments)]
fn ring_open_with_at(size: (f32, f32), scale: f32, apps: &[&'static str], names: &[&'static str], status: usize, tray: usize,
                     wins: &[(&'static str, &'static str, bool, bool)]) -> tiny_skia::Pixmap {
    ring_open_found(size, scale, apps, names, status, tray, wins, false)
}

#[allow(clippy::too_many_arguments)]
fn ring_open_found((w, h): (f32, f32), scale: f32, apps: &[&'static str], names: &[&'static str], status: usize, tray: usize,
                   wins: &[(&'static str, &'static str, bool, bool)], searching: bool) -> tiny_skia::Pixmap {
    use crate::toolkit::ui::Ui;
    let (tree, _) = ring_tree_found((w, h), apps, names, status, tray, wins, searching);
    let mut ui = Ui::new(tree, look());
    ui.see_through = true;

    let (pw, ph) = ((w * scale).ceil() as u32, (h * scale).ceil() as u32);
    let mut pix = Pixmap::new(pw, ph).unwrap();
    for (i, px) in pix.pixels_mut().iter_mut().enumerate() {
        let (x, y) = ((i as u32 % pw) as f32 / pw as f32, (i as u32 / pw) as f32 / ph as f32);
        let hill = 0.62 + 0.08 * (x * 7.0).sin() + 0.04 * (x * 17.0 + 1.0).cos();
        let c = if y < hill { [70.0 + 90.0 * y, 120.0 + 70.0 * y, 190.0 - 20.0 * y] } else { [52.0 - 20.0 * (y - hill), 96.0 - 60.0 * (y - hill), 74.0] };
        *px = tiny_skia::PremultipliedColorU8::from_rgba(c[0] as u8, c[1] as u8, c[2] as u8, 255).unwrap();
    }
    let mut over = Pixmap::new(pw, ph).unwrap();

    let laid = crate::toolkit::theme::laid_over(&pix.as_mut(), (0, 0, pw, ph), 1.0);
    ui.paint(&mut over.as_mut(), scale);
    drop(laid);
    pix.draw_pixmap(0, 0, over.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    pix
}

#[test]
fn the_ring_keeps_its_shape_at_every_size_and_scale() {
    let mut out = Vec::new();
    for (w, h, s) in SCREENS {
        for status in 3..=6 {
            for tray in [0, 1, 3, 5] {
                let at = format!("{w}x{h} points at {s}, {status} status, {tray} tray");
                its_shape_at(&at, (w, h, s), (status, tray), &mut out);
            }
        }
    }
    assert!(out.is_empty(), "{} faults:\n{}", out.len(), out.join("\n"));
}

fn its_shape_at(at: &str, (w, h, s): (f32, f32, f32), (status, tray): (usize, usize), out: &mut Vec<String>) {
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Rect;
    let apart = |a: &Rect, b: &Rect| a.x + a.w <= b.x + 0.01 || b.x + b.w <= a.x + 0.01 || a.y + a.h <= b.y + 0.01 || b.y + b.h <= a.y + 0.01;
    let (tree, lay) = ring_tree((w, h), &LAPTOP_APPS, &LAPTOP_NAMES, status, tray);
    let mut ui = Ui::new(tree, look());
    let places = ui.places_at_scale(((w * s).ceil() as u32, (h * s).ceil() as u32), s);
    let find = |p: &str| -> Vec<(String, Rect)> { places.iter().filter(|(id, _)| id.starts_with(p)).cloned().collect() };
    let one = |id: &str| places.iter().find(|(i, _)| i == id).map(|p| p.1);
    let (cx, cy) = (w * s / 2.0, h * s / 2.0);
    let from_middle = |r: &Rect| ((r.x + r.w / 2.0 - cx).powi(2) + (r.y + r.h / 2.0 - cy).powi(2)).sqrt();
    let tools = find("tool-");
    let (Some(clock), Some(top), Some(foot)) = (one("clock"), one("top-band"), one("foot-band")) else {
        out.push(format!("{at}: the time or a band is missing"));
        return;
    };
    assert_eq!(tools.len(), tool_rows()[0].len() + 1 + 1 + status + tool_rows()[1].len() + 1 + tray, "{at}");
    for band in ["upper", "status", "lower", "tray"] {
        let ys: Vec<f32> = tools.iter().filter(|(id, _)| id.starts_with(&format!("tool-{band}-"))).map(|t| t.1.y).collect();
        if ys.iter().any(|y| (y - ys[0]).abs() > 0.5) {
            out.push(format!("{at}: the {band} band is on more than one row: {ys:?}"));
        }
    }
    let disc = CENTRE as f32 / 2.0 * s;
    for (i, (id, r)) in tools.iter().enumerate() {
        let others = tools.iter().skip(i + 1).map(|(a, b)| (a.as_str(), *b)).chain([("clock", clock), ("top-band", top), ("foot-band", foot)]);
        for (other, o) in others {
            if !apart(r, &o) {
                out.push(format!("{at}: {id} {r:?} is on {other} {o:?}"));
            }
        }
        if from_middle(r) + r.w.max(r.h) / 2.0 > disc + 0.5 {
            out.push(format!("{at}: {id} {r:?} leaves the middle's disc"));
        }
    }
    let favs = find("fav-");
    assert_eq!(favs.len(), LAPTOP_APPS.len(), "{at}");
    let ring = lay.disc_radius as f32 * s;
    for (i, (id, r)) in favs.iter().enumerate() {
        for (other, o) in favs.iter().skip(i + 1) {
            if !apart(r, o) {
                out.push(format!("{at}: {id} {r:?} is on {other} {o:?}"));
            }
        }
        let d = from_middle(r);
        if d + r.w.max(r.h) / 2.0 > ring + 0.5 || d - r.w.max(r.h) / 2.0 < disc - 0.5 {
            out.push(format!("{at}: {id} {r:?} leaves the ring between {disc} and {ring}"));
        }
        if r.x < 0.0 || r.y < 0.0 || r.x + r.w > w * s + 0.5 || r.y + r.h > h * s + 0.5 {
            out.push(format!("{at}: {id} {r:?} leaves the screen"));
        }
    }
}

#[test]
fn the_windows_ring_stays_whole_beside_search_results() {
    for (w, h, _) in SCREENS {
        let found: Vec<usize> = (0..400).collect();
        let k = crate::ring::open_windows::Sizes::of(crate::toolkit::theme::sheet());
        let lay = crate::ring::layout::search_ring(&found, w as f64, h as f64, k.gap + k.width);
        let ring = crate::ring::open_windows::ring(7, lay.disc_radius, (w.min(h) / 2.0 - crate::ring::layout::MARGIN as f32) as f64, &k);
        let ring = ring.unwrap_or_else(|| panic!("{w}x{h}: the windows ring is gone beside the results"));
        assert!((ring.outer - ring.inner - k.width).abs() < 1e-6, "{w}x{h}: the windows ring is narrower beside the results");
        assert!(lay.placed.len() >= 8, "{w}x{h}: only {} results show", lay.placed.len());
    }
}

#[test]
fn tab_dots_show_only_for_the_program_using_and_move_nothing() {
    use crate::common::in_use::Kind::{Camera as C, Microphone as M};
    let wins: Vec<(&'static str, &'static str, bool, bool)> = WINDOWS[..4].to_vec();
    let shot = |uses: Vec<(usize, Vec<crate::common::in_use::Kind>)>| {
        TAB_USES.with(|u| *u.borrow_mut() = uses);
        let p = ring_open_with_at((1280.0, 800.0), 1.0, &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &wins);
        TAB_USES.with(|u| u.borrow_mut().clear());
        p
    };
    let none = shot(vec![]);
    let some = shot(vec![(1, vec![M, C])]);
    assert_ne!(none.data(), some.data(), "a program using the microphone and camera shows no dots");
    let places = |uses: Vec<(usize, Vec<crate::common::in_use::Kind>)>| {
        TAB_USES.with(|u| *u.borrow_mut() = uses);
        let (tree, _) = ring_tree_found((1280.0, 800.0), &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &wins, false);
        TAB_USES.with(|u| u.borrow_mut().clear());
        crate::toolkit::ui::Ui::new(tree, look()).places_at_scale((1280, 800), 1.0)
    };
    let (a, b) = (places(vec![]), places(vec![(1, vec![M, C])]));
    assert_eq!(a.len(), b.len());
    for ((i, x), (j, y)) in a.iter().zip(&b) {
        assert_eq!((i, x.x, x.y, x.w, x.h), (j, y.x, y.y, y.w, y.h), "{i} moved when a dot came");
    }
}

const WINDOWS: [(&str, &str, bool, bool); 12] = [
    ("firefox", "BBC News - Home", true, false), ("brave", "Trindle releases", false, false), ("system-file-manager", "Documents", false, false),
    ("com.brave.Browser", "Signal", false, true), ("md.obsidian.Obsidian", "Garden plans", false, false), ("vlc", "holiday 2026.mp4", false, false),
    ("firefox", "Bank statement - October", false, false), ("gimp", "Poster", false, false), ("inkscape", "Logo", false, false),
    ("audacity", "Interview", false, false), ("thunderbird", "Inbox", false, false), ("lantern", "Lantern", false, false)];

fn windows_ring_faults((w, h, s): (f32, f32, f32), n: usize, out: &mut Vec<String>) {
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Rect;
    let at = format!("{w}x{h} points at {s}, {n} windows");
    let (tree, lay) = ring_tree_with((w, h), &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..n]);
    let k = crate::toolkit::theme::sheet();
    let Some(rg) = open_windows::ring(n, lay.disc_radius, (w.min(h) / 2.0 - MARGIN as f32) as f64, &open_windows::Sizes::of(k)) else {
        out.push(format!("{at}: there is no ring of windows"));
        return;
    };
    let half = (w.min(h) / 2.0) as f64;
    if rg.inner < lay.disc_radius + k.ring_windows_gap as f64 - 0.01 {
        out.push(format!("{at}: it is on the favourites' disc"));
    }
    if rg.outer > half - MARGIN + 0.01 {
        out.push(format!("{at}: it reaches {} of the {} the screen leaves", rg.outer, half - MARGIN));
    }
    let mut ui = Ui::new(tree, look());
    let places = ui.places_at_scale(((w * s).ceil() as u32, (h * s).ceil() as u32), s);
    let icons: Vec<Rect> = (0..n).filter_map(|i| places.iter().find(|(id, _)| id == &format!("win-{i}")).map(|p| p.1)).collect();
    if icons.len() != n {
        out.push(format!("{at}: {} of the {n} icons are drawn", icons.len()));
        return;
    }
    let (cx, cy) = (w * s / 2.0, h * s / 2.0);
    for (i, r) in icons.iter().enumerate() {
        let d = ((r.x + r.w / 2.0 - cx).powi(2) + (r.y + r.h / 2.0 - cy).powi(2)).sqrt();

        let icon = r.w.max(r.h) / 2.0 - open_windows::BUTTON_PAD as f32 * s;
        if d - icon < rg.inner as f32 * s - 0.5 || d + icon > rg.outer as f32 * s + 0.5 {
            out.push(format!("{at}: icon {i} {r:?} leaves the ring between {} and {}", rg.inner as f32 * s, rg.outer as f32 * s));
        }
        if r.x < 0.0 || r.y < 0.0 || r.x + r.w > w * s + 0.5 || r.y + r.h > h * s + 0.5 {
            out.push(format!("{at}: icon {i} {r:?} leaves the screen"));
        }
        for (j, o) in icons.iter().enumerate().skip(i + 1) {

            let (ox, oy) = (o.x + o.w / 2.0, o.y + o.h / 2.0);
            let (rx, ry) = (r.x + r.w / 2.0, r.y + r.h / 2.0);
            let other = o.w.max(o.h) / 2.0 - open_windows::BUTTON_PAD as f32 * s;
            if (ox - rx).hypot(oy - ry) < icon + other - 0.05 * s {
                out.push(format!("{at}: icon {i} is on icon {j}"));
            }
        }
    }
}

#[test]
fn the_windows_ring_fits_the_screen_at_every_size_and_scale() {
    let mut out = Vec::new();
    for (w, h, s) in SCREENS {
        for n in [1usize, 2, 3, 7, 8, 9, 12] {
            windows_ring_faults((w, h, s), n, &mut out);
        }
    }
    assert!(out.is_empty(), "{} faults:\n{}", out.len(), out.join("\n"));

    let apps: Vec<usize> = (0..crate::common::favourites::MAX).collect();
    for (w, h, s) in SCREENS {
        let lay = layout::<usize>(&apps, &[], w as f64, h as f64);
        let half = (w.min(h) / 2.0 - MARGIN as f32) as f64;
        if let Some(rg) = open_windows::ring(5, lay.disc_radius, half, &open_windows::Sizes::of(crate::toolkit::theme::sheet())) {
            assert!(rg.outer <= half + 0.01, "{w}x{h} at {s}: with {} favourites it leaves the screen", apps.len());
        }
    }
}

#[test]
fn the_windows_ring_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("the ring of windows", 20, || {
        drawn::page("open");
        let _ = ring_open_with_at((1280.0, 800.0), 1.0, &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..3]);
        let _ = ring_open_with_at((1280.0, 800.0), 1.0, &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..10]);

        for pic in [Some(std::rc::Rc::new(made_up_window(640, 400, [70, 110, 190]))), None] {
            POINTED.with(|p| *p.borrow_mut() = Some((0, pic)));
            let _ = ring_open_with_at((1280.0, 800.0), 1.0, &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..4]);
            POINTED.with(|p| *p.borrow_mut() = None);
        }
    }));
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_windows_ring_names() {
    let Some(dir) = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&dir).unwrap();
    for (n, scale) in [(3usize, 1.0_f32), (8, 1.0), (12, 1.0), (12, 2.0)] {
        ring_open_with_at((1280.0, 800.0), scale, &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..n]).save_png(dir.join(format!("outer-ring-{n}-windows-at-{scale}.png"))).unwrap();
    }
}

#[test]
fn the_ring_and_its_disc_are_solid_grey_in_every_look() {
    use crate::toolkit::ui::Ui;
    let (w, h) = (1280.0_f32, 800.0_f32);
    for light in [false, true] {
        crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((light, crate::common::style::DEFAULT_ACCENT.to_string())));
        {
            let (tree, lay) = ring_tree_found((w, h), &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..3], false);
            let mut ui = Ui::new(tree, look());
            ui.see_through = true;
            let mut pix = Pixmap::new(w as u32, h as u32).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            let sheet = crate::toolkit::theme::sheet();
            let top = |r: f32| pix.pixel((w / 2.0) as u32, (h / 2.0 - r) as u32).unwrap();
            let d = lay.disc_radius as f32;
            for r in [d * 0.5, d * 0.8, d - 3.0] {
                assert_eq!(top(r).alpha(), 255, "light {light}: the disc is solid at {r} out");
            }
            let mid = d + sheet.ring_windows_gap + sheet.ring_windows_width / 2.0;
            let got = top(mid);
            assert_eq!(got.alpha(), 255, "light {light}: the window ring's track is solid");
            let want = crate::toolkit::theme::mix(look().bg, look().fg, sheet.ring_windows_track / 100.0).to_skia();
            assert!((got.red() as i32 - (want.red() * 255.0) as i32).abs() <= 3, "light {light}: the track is the sheet's grey {got:?}");
        }
        crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
    }
}

#[test]
fn pointing_at_a_window_moves_no_icon_and_only_the_middle_has_the_trimmed_title() {
    use crate::toolkit::ui::Ui;
    let places = |pointed: Option<usize>| {
        POINTED.with(|p| *p.borrow_mut() = pointed.map(|i| (i, None)));
        let (tree, _) = ring_tree_with((1280.0, 800.0), &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..6]);
        let got = Ui::new(tree, look()).places_at_scale((1280, 800), 1.0);
        POINTED.with(|p| *p.borrow_mut() = None);
        got
    };
    let rest = places(None);
    for at in [0usize, 3, 5] {
        let on = places(Some(at));
        let mut seen = 0;
        for (id, r) in rest.iter().filter(|(id, _)| id.starts_with("win-") || id.starts_with("fav-")) {
            let now = on.iter().find(|(i, _)| i == id).unwrap_or_else(|| panic!("{id} went when window {at} was pointed at")).1;
            assert_eq!((now.x, now.y, now.w, now.h), (r.x, r.y, r.w, r.h), "{id} moved when window {at} was pointed at");
            seen += 1;
        }
        assert!(seen >= 6 + 6, "the windows' icons and the favourites were found: {seen}");
    }
    let view = view_code();
    let before_tests = view;
    assert_eq!(before_tests.matches("without_program(").count(), 1, "only the middle's title is trimmed");
    assert!(before_tests.contains("name: open_windows::tab_words(&if title.is_empty() { program } else { title }, &used)"), "the icon's name is the whole title");
    assert!(before_tests.contains("let name = if title.is_empty() { program } else { format!(\"{program}, {title}\") };"), "the middle's name for a screen reader is the whole title");
}

#[test]
fn the_title_switch_is_on_to_begin_with_and_changes_no_name_or_place() {
    use crate::toolkit::ui::Ui;
    assert_eq!(crate::common::files::default_setting("ring_window_names"), "yes");
    let places = |show: bool| {
        SHOW_TITLE.with(|s| s.set(show));
        POINTED.with(|p| *p.borrow_mut() = Some((0, None)));
        let (tree, _) = ring_tree_with((1280.0, 800.0), &LAPTOP_APPS, &LAPTOP_NAMES, 5, 1, &WINDOWS[..6]);
        let got = Ui::new(tree, look()).places_at_scale((1280, 800), 1.0);
        POINTED.with(|p| *p.borrow_mut() = None);
        SHOW_TITLE.with(|s| s.set(true));
        got
    };
    let (on, off) = (places(true), places(false));
    assert_eq!(on.len(), off.len());
    for (id, r) in &on {
        let now = off.iter().find(|(i, _)| i == id).unwrap().1;
        assert_eq!((now.x, now.y, now.w, now.h), (r.x, r.y, r.w, r.h), "{id} moved with the switch");
    }
    let view = view_code();
    let before_tests = view;
    assert_eq!(before_tests.matches("win_title").count(), 4, "the switch is read at opening, kept, and only handed to the middle");
    assert!(before_tests.contains("let name = if title.is_empty() { program } else { format!(\"{program}, {title}\") };"), "the name for a screen reader is the whole title with the switch either way");
}

fn made_up_window(w: u32, h: u32, tint: [u8; 3]) -> tiny_skia::Pixmap {
    use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};
    let mut pm = Pixmap::new(w, h).unwrap();
    pm.fill(Color::from_rgba8(246, 246, 244, 255));
    let mut put = |x: f32, y: f32, rw: f32, rh: f32, c: [u8; 3]| {
        let mut paint = Paint::default();
        paint.set_color_rgba8(c[0], c[1], c[2], 255);
        let path = PathBuilder::from_rect(Rect::from_xywh(x, y, rw, rh).unwrap());
        pm.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    };
    let (wf, hf) = (w as f32, h as f32);
    put(0.0, 0.0, wf, hf * 0.09, tint);
    put(0.0, hf * 0.09, wf * 0.22, hf * 0.91, [228, 228, 224]);
    for i in 0..9 {
        let y = hf * (0.18 + i as f32 * 0.085);
        put(wf * 0.28, y, wf * (0.3 + 0.4 * ((i * 5 % 7) as f32 / 7.0)), hf * 0.03, [150, 150, 146]);
    }
    put(wf * 0.28, hf * 0.12, wf * 0.2, hf * 0.04, tint);
    pm
}

#[test]
fn a_favourite_has_no_in_use_dots_under_its_name() {
    use crate::common::in_use::Kind;
    use crate::toolkit::ui::Ui;
    let mut app = crate::common::favourites::path_entry(std::path::Path::new("/usr/share/applications/flow.desktop"), Some("system-file-manager".into()));
    app.id = "flow.desktop".into();
    app.name = "Flow".into();
    let at = [4.0, 4.0, 88.0, 78.0];
    let tree = crate::toolkit::widget::placed(app_button(&app, 0, 48, true, false, true, at, Ink::Usual), move |_, _| (at[0], at[1]));
    let mut ui = Ui::new(tree, crate::toolkit::theme::Theme::new(false, &crate::common::style::accent()));
    let mut pix = tiny_skia::Pixmap::new(96, 90).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    for kind in Kind::ALL {
        let (r, g, b) = kind.colour();
        let want = [(r * 255.0).round() as i32, (g * 255.0).round() as i32, (b * 255.0).round() as i32];
        let found = pix.pixels().iter().enumerate().any(|(n, p)| {
            (n / 96) as f32 >= 4.0 + 48.0 + 14.0 && p.alpha() == 255
                && [p.red() as i32, p.green() as i32, p.blue() as i32].iter().zip(want).all(|(a, b)| (a - b).abs() <= 2)
        });
        assert!(!found, "{kind:?} has no dot under a favourite's name");
    }
}

#[test]
fn the_middle_picture_fits_the_disc_whatever_the_window() {
    let k = crate::toolkit::theme::sheet();
    use crate::ring::open_windows;
    let disc = CENTRE as f32;
    for (w, h) in [(1280.0f32, 800.0f32), (1920.0, 1080.0), (800.0, 800.0), (600.0, 900.0), (300.0, 1200.0), (2400.0, 300.0), (10.0, 10.0)] {
        let (bw, bh) = open_windows::picture_box(disc, k.ring_windows_picture_margin, k.ring_windows_picture_corner, w, h);
        assert!(((bw / bh) / (w / h) - 1.0).abs() < 0.03 || bw.min(bh) < 40.0, "{w}x{h}: proportions lost: {bw}x{bh}");
        let c = k.ring_windows_picture_corner.min(bw / 2.0).min(bh / 2.0);
        let reach = (bw / 2.0 - c).hypot(bh / 2.0 - c) + c;
        assert!(reach <= disc / 2.0 - k.ring_windows_picture_margin + 0.01, "{w}x{h}: the corners leave the disc: {reach}");
        assert!(bw.max(bh) >= disc / 2.0 - k.ring_windows_picture_margin - 2.0 || w / h > 3.0 || h / w > 3.0, "{w}x{h}: too small: {bw}x{bh}");
    }
}

#[test]
fn the_middle_gives_way_to_the_picture_only_while_one_is_pointed_at() {
    use super::window_ring::pointing_shows_picture as shows;
    for n in 0..16u8 {
        let (pointing, menu, power, quick) = (n & 1 != 0, n & 2 != 0, n & 4 != 0, n & 8 != 0);
        assert_eq!(shows(pointing, menu, power, quick), pointing && !menu && !power && !quick);
    }

    let view = view_code();
    let before_tests = view;
    assert!(before_tests.contains("let hiding = pointed.is_some();") && before_tests.contains("if !hiding {") && before_tests.contains("self.backdrop(hiding)"));
    assert!(!before_tests.contains("fn window_card"), "the square card on pointing is back");
}

#[test]
fn no_hover_in_the_ring_rebuilds_the_view_on_each_move() {
    for (name, code) in [("view", view_code()), ("plugins.rs", crate::common::source::code(include_str!("../plugins.rs"))), ("sni", crate::ring::sni::tests::sni_code()),
                         ("open_windows.rs", crate::common::source::code(include_str!("../open_windows.rs"))), ("mic.rs", crate::common::source::code(include_str!("../mic.rs"))), ("quick", crate::ring::quick::tests::quick_code()),
                         ("notes", crate::ring::notes::tests::notes_code())] {
        let code = code.split("\nmod tests {").next().unwrap();
        assert!(!code.contains("_ => msg(|_| {}),"), "{name} sends a message that does nothing on a pointer move: use Msg::idle()");
        if let Some(line) = code.lines().find(|l| l.contains("Pointer::Hover(") && l.contains("=> msg(")) {
            panic!("{name}: a hover says something with a bare message, so every move rebuilds: {line}");
        }
        if let Some(line) = code.lines().find(|l| l.contains("Pointer::Leave") && l.contains("=> msg(")) {
            panic!("{name}: a leave says something with a bare message: {line}");
        }
    }
}

fn an_app(id: &str, name: &str, category: &'static str) -> Program {
    let mut app = crate::common::favourites::path_entry(std::path::Path::new("/usr/share/applications").join(id).as_path(), None);
    (app.id, app.name, app.category) = (id.to_string(), name.to_string(), category);
    app
}

#[test]
fn a_windows_menu_offers_make_or_remove_favourite_for_every_program_with_a_launcher() {
    let kind = crate::common::apps::CATEGORIES[0];
    let all: Vec<Program> = ["a", "b", "c", "d"].iter().map(|n| an_app(&format!("{n}.desktop"), n, kind)).collect();
    let favs = vec!["b.desktop".to_string(), "d.desktop".to_string()];
    for (n, want) in [("a", false), ("b", true), ("c", false), ("d", true)] {
        assert_eq!(Face::window_favourite(&all, n, &favs), Some(want), "{n}");
    }
    assert_eq!(Face::window_favourite(&all, "unknown-program", &favs), None);
    let code = view_code();
    let src = code;
    assert!(src.contains("tools.push(press(star, WinAct::Star))") && src.contains("WinAct::Star => {"), "the star is the menu's fourth button");
    assert!(src.contains("self.set_favourite(&id, on)"), "it uses the one favourites store");
}

#[test]
fn a_favourite_is_in_the_middle_ring_and_in_no_piece() {
    use super::arrange::pieces_of;
    let kind = crate::common::apps::CATEGORIES[0];
    let mut all = vec![an_app("firefox.desktop", "Firefox", kind), an_app("org.example.Notes.desktop", "Notes", kind),
                       an_app("org.example.Fire.desktop", "Fire Maker", kind)];

    all.push(crate::common::favourites::path_entry(std::path::Path::new("/home/john/Fireworks"), None));
    all.push(crate::common::favourites::path_entry(std::path::Path::new("/home/john/notes.txt"), None));
    let favourites: Vec<String> = ["firefox.desktop", "/home/john/Fireworks", "/home/john/notes.txt"].map(String::from).to_vec();
    let first = vec![0usize, 3, 4];
    let in_pieces = |groups: &[(String, Vec<usize>)]| -> Vec<usize> { groups.iter().flat_map(|g| g.1.clone()).collect() };

    assert!(pieces_of(&all, &favourites, &first, false).is_empty());

    let mut shown = in_pieces(&pieces_of(&all, &favourites, &first, true));
    shown.sort();
    assert_eq!(shown, vec![1, 2]);

    let fewer: Vec<String> = favourites[1..].to_vec();
    let mut shown = in_pieces(&pieces_of(&all, &fewer, &[3, 4], true));
    shown.sort();
    assert_eq!(shown, vec![0, 1, 2]);

    let groups = pieces_of(&all, &["firefox-esr.desktop".to_string()], &[0], true);
    assert!(!in_pieces(&groups).contains(&0), "a favourite kept under another name is drawn twice");

    assert_eq!(found(&all, "fire"), vec![0, 3, 2]);
    assert_eq!(found(&all, "firew"), vec![3]);
    assert!(found(&all, "zzz").is_empty(), "nothing matches: the ring says so");
}

#[test]
fn ring_button_icons_are_white_or_dark_and_readable_on_every_shade() {
    use crate::ring::Job;
    use crate::toolkit::theme::{contrast, Theme};
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Node;
    let size = 40u32;
    for light in [false, true] {
        for accent in ["#fbc711", "#ff3b9a", "#e91e63", "#2f6fed", "#33cc66", "#9b59b6"] {
            for job in [Job::Programs, Job::Computer, Job::Tools, Job::Tray] {
                for pressable in [false, true] {
                    let seen: Rc<Cell<Option<Colour>>> = Rc::new(Cell::new(None));
                    let keep = seen.clone();
                    let mut node: Node<()> = button("", "Test").size(size as f32).tinted(job.share())
                        .face(move |_, _, ink, _| keep.set(Some(ink)));
                    if pressable {
                        node = node.on_press(());
                    }
                    let mut ui = Ui::new(node, Theme::new(light, accent));
                    let mut pix = Pixmap::new(size, size).unwrap();
                    ui.paint(&mut pix.as_mut(), 1.0);
                    let ink = seen.get().expect("the icon is drawn");
                    let back = pix.pixel(size / 2, 3).unwrap();
                    let back = Colour::rgb(back.red() as f32 / 255.0, back.green() as f32 / 255.0, back.blue() as f32 / 255.0);
                    let at = format!("light {light}, {accent}, {job:?}, pressable {pressable}");
                    assert!((ink.r - ink.g).abs() < 0.005 && (ink.g - ink.b).abs() < 0.005, "{at}: the icon is tinted {ink:?}");
                    assert!(contrast(ink, back) >= crate::toolkit::theme::WORDS, "{at}: the icon stands {} to one", contrast(ink, back));
                }
            }
        }
    }
}

#[test]
fn each_ring_job_has_its_own_shade() {
    use crate::ring::Job;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Node;
    let size = 40u32;
    for light in [false, true] {
        for accent in ["#fbc711", "#ff3b9a", "#2f6fed", "#9b59b6"] {
            let backs: Vec<(Job, (u8, u8, u8))> = [Job::Programs, Job::Computer, Job::Tools, Job::Tray].into_iter().map(|job| {
                let node: Node<()> = button("", "Test").size(size as f32).tinted(job.share());
                let mut ui = Ui::new(node, Theme::new(light, accent));
                let mut pix = Pixmap::new(size, size).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                let p = pix.pixel(size / 2, 3).unwrap();
                (job, (p.red(), p.green(), p.blue()))
            }).collect();
            for (n, (a, x)) in backs.iter().enumerate() {
                for (b, y) in &backs[n + 1..] {
                    assert_ne!(x, y, "light {light}, {accent}: {a:?} and {b:?} have the same shade");
                }
            }
        }
    }
}

#[test]
fn a_long_name_under_an_app_is_cut_at_its_end_only() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::{drawing, Node};
    use tiny_skia::Pixmap;
    let across = 60u32;
    let painted = |name: &'static str| {
        let node: Node<()> = drawing((across as f32, 80.0), name, move |pm, pen| {
            paint_app(pm, pen, &[], name, 32, true, false, false, super::pinned::Ink::Usual);
        });
        let mut ui = Ui::new(node, Theme::new(true, "#fbc711"));
        let mut pix = Pixmap::new(across, 80).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix
    };
    let bare = painted("");

    let inked = |name: &'static str| {
        let with = painted(name);
        let columns: Vec<u32> = (0..across).filter(|&x| (0..80).any(|y| with.pixel(x, y) != bare.pixel(x, y))).collect();
        (*columns.first().expect("the name is drawn"), *columns.last().unwrap())
    };
    for long in ["Example Terminal Emulator", "ImageMagick (color depth=q16)", "WWWWWWWWWWWWWW", "MMMMMMMMM"] {
        let (first, last) = inked(long);
        assert!(first >= 1 && last <= across - 2, "{long} is drawn from column {first} to {last} of a tile {across} across: cut at its start, or run off its end");

        assert!(last - first > across / 2, "{long} was cut to less than half the tile");
    }
    let (first, last) = inked("Text");
    assert!(first > 10 && last < across - 10, "a short name is no longer in the middle of its tile");
}

#[test]
fn a_disc_has_room_for_its_whole_float_shadow() {
    for light in [true, false] {
        let theme = crate::toolkit::theme::Theme::new(light, "#fbc711");
        for f in theme.float.iter() {
            assert!(f.y + f.blur + f.spread <= super::paint::DISC_ROOM, "{f:?}");
        }
    }
}
