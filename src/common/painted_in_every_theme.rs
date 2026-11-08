//! Every test that paints a window for the table of pairs, Trindle's and
//! the toolkit's, and the checks that hold the list whole: each window has
//! one, and each is run again in every theme that comes with Trindle. The
//! toolkit runs its own three again in every theme in its own tests
//! (`theme::drawn`); here are Trindle's.

use crate::toolkit::theme::drawn::NAMED;

#[test]
fn the_themes_that_come_with_trindle_draw_only_pairs_the_table_holds_too() {
    const INNER: &str = "TRINDLE_PAIRS_FEW";
    if std::env::var_os(INNER).is_some() {
        return;
    }
    let themes = crate::common::source::root().join("share/themes");
    let mut names: Vec<std::path::PathBuf> = std::fs::read_dir(&themes).unwrap().flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "css")).collect();
    names.sort();
    assert!(names.len() >= 4, "{names:?}");
    for theme in names {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([NAMED, "--test-threads=1"]).env("TRINDLE_TEST_THEME", &theme).env(INNER, "1").output().unwrap();
        let said = String::from_utf8_lossy(&out.stdout);
        let ran = said.lines().filter(|l| l.starts_with("test ") && l.ends_with("... ok")).count();
        let all = every_painting_test();
        assert!(out.status.success() && ran >= all.len(), "in the theme {}: {ran} tests passed\n{}", theme.display(),
                said.lines().filter(|l| !l.ends_with("... ok")).take(120).collect::<Vec<_>>().join("\n"));

        for name in all {
            assert!(said.lines().any(|l| l.starts_with("test ") && l.ends_with(&format!("::{name} ... ok"))),
                    "in the theme {}, the test {name} did not run and pass", theme.display());
        }
    }
}

fn every_painting_test() -> Vec<&'static str> {
    let mut all: Vec<&str> = PAINTED.iter().flat_map(|(_, _, tests)| tests.iter().copied())
        .chain(ALSO.iter().filter(|(file, _)| !file.starts_with("toolkit/")).map(|(_, test)| *test)).collect();
    all.sort_unstable();
    all.dedup();
    all
}

const PAINTED: &[(&str, &str, &[&str])] = &[
    ("activity/mod.rs", "activity/tests.rs", &["activity_draws_only_pairs_the_table_holds"]),
    ("session/restart_ask.rs", "session/restart_ask.rs", &["the_restart_question_draws_only_pairs_the_table_holds"]),
    ("store/fence_window.rs", "store/fence_window.rs", &["the_requests_draw_only_pairs_the_table_holds"]),
    ("disks/app.rs", "disks/tests/stopping.rs", &["drives_draws_only_pairs_the_table_holds"]),
    ("greeter/mod.rs", "greeter/tests.rs", &["the_login_screen_draws_only_pairs_the_table_holds"]),
    ("installer/mod.rs", "installer/tests.rs", &["the_installer_draws_only_pairs_the_table_holds"]),
    ("installer/welcome.rs", "installer/welcome.rs", &["the_sticks_welcome_draws_only_pairs_the_table_holds"]),
    ("keyring/prompt.rs", "keyring/prompt.rs", &["the_password_box_draws_only_pairs_the_table_holds"]),
    ("plugins/pin.rs", "plugins/pin/tests.rs", &["the_pin_card_draws_only_pairs_the_table_holds"]),
    ("mcp/ask.rs", "mcp/ask.rs", &["the_assistants_question_draws_only_pairs_the_table_holds"]),
    ("polkit/mod.rs", "polkit/mod.rs", &["the_question_draws_only_pairs_the_table_holds"]),
    ("ring/battery_alert.rs", "ring/battery_alert.rs", &["the_batterys_card_draws_only_pairs_the_table_holds"]),
    ("ring/label_window.rs", "ring/label_window.rs", &["the_label_beside_a_tab_draws_only_pairs_the_table_holds"]),
    ("ring/notes/card.rs", "ring/notes/tests.rs", &["a_notification_circle_draws_only_pairs_the_table_holds"]),
    ("ring/magnify.rs", "ring/magnify.rs", &["the_magnifier_draws_only_pairs_the_table_holds"]),
    ("ring/notice.rs", "ring/notice.rs", &["the_notice_draws_only_pairs_the_table_holds"]),
    ("ring/view/app.rs", "ring/view/tests.rs", &["the_ring_draws_only_pairs_the_table_holds", "the_windows_ring_draws_only_pairs_the_table_holds"]),
    ("report/mod.rs", "report/tests_window.rs", &["the_window_draws_only_pairs_the_table_holds"]),
    ("screenshot/portal_print.rs","screenshot/portal_print.rs", &["the_print_window_draws_only_pairs_the_table_holds"]),
    ("screenshot/cast_pick.rs", "screenshot/cast_pick.rs", &["the_picker_of_what_to_share_draws_only_pairs_the_table_holds"]),
    ("screenshot/colours.rs", "screenshot/colours.rs", &["the_eye_droppers_list_draws_only_pairs_the_table_holds"]),
    ("screenshot/file_chooser/update.rs", "screenshot/file_chooser/tests/menu.rs", &["the_file_chooser_draws_only_pairs_the_table_holds"]),
    ("screenshot/mod.rs", "screenshot/mod.rs", &["the_screenshot_overlay_draws_only_pairs_the_table_holds"]),
    ("screenshot/pin.rs", "screenshot/pin.rs", &["a_pinned_screenshot_draws_only_pairs_the_table_holds"]),
    ("screenshot/portal_apps.rs", "screenshot/portal_apps.rs", &["the_chooser_of_an_app_draws_only_pairs_the_table_holds"]),
    ("screenshot/portal_ask.rs", "screenshot/portal_ask.rs", &["a_portals_question_draws_only_pairs_the_table_holds"]),
    ("session/starting.rs", "session/starting.rs", &["one_moment_please_draws_only_pairs_the_table_holds"]),
    ("settings/mod.rs", "settings/tests/trial.rs", &["settings_draws_only_pairs_the_table_holds"]),
    ("store/asking.rs", "store/asking.rs", &["what_a_program_may_use_draws_only_pairs_the_table_holds"]),
    ("store/deb.rs", "store/shots_test_pages.rs", &["apps_in_the_dark_look_draws_only_pairs_the_table_holds", "apps_in_the_light_look_draws_only_pairs_the_table_holds"]),
    ("store/draw.rs", "store/shots_test_pages.rs", &["apps_in_the_dark_look_draws_only_pairs_the_table_holds", "apps_in_the_light_look_draws_only_pairs_the_table_holds"]),
    ("text/app.rs", "text/tests.rs", &["text_draws_only_pairs_the_table_holds"]),
    ("thunder/ask.rs", "thunder/ask.rs", &["the_thunderbolt_question_draws_only_pairs_the_table_holds"]),
    ("welcome/mod.rs", "welcome/tests.rs", &["welcome_draws_only_pairs_the_table_holds"]),
];

const ALSO: &[(&str, &str)] = &[
    ("ring/circles.rs", "the_circles_draw_only_pairs_the_table_holds"),
    ("common/app_theme.rs", "what_is_written_for_other_programs_is_only_pairs_the_table_holds"),
    ("common/app_theme.rs", "the_colours_other_programs_write_words_in_are_only_pairs_the_table_holds"),
    ("ring/quick/tests.rs", "the_rings_pages_draw_only_pairs_the_table_holds"),
    ("session/lock/tests.rs", "the_lock_screen_draws_only_pairs_the_table_holds"),
    ("session/screensaver.rs", "the_screensavers_clock_draws_only_pairs_the_table_holds"),
    ("toolkit/gallery.rs", "the_gallery_draws_only_pairs_the_table_holds"),
    ("toolkit/ui/for_good_tests.rs", "a_red_main_action_draws_only_pairs_the_table_holds"),
    ("toolkit/ui.rs", "selected_words_draw_only_pairs_the_table_holds"),
];

#[test]
fn every_window_is_painted_for_its_pairs() {
    let (mut found, mut named) = (Vec::new(), Vec::new());
    for (name, text) in crate::common::source::every_file() {

        let own = name.starts_with("toolkit/") || name.starts_with("compose/");
        if !own && text.lines().any(|l| l.starts_with("impl App for ") || l.starts_with("impl Shows for ")) {
            found.push(name.clone());
        }
        for line in text.lines() {
            if let Some(test) = line.trim().strip_prefix("fn ").and_then(|l| l.strip_suffix("() {")).filter(|t| t.ends_with(NAMED)) {
                named.push((name.clone(), test.to_string()));
            }
        }
    }
    found.sort();
    assert!(found.len() >= 30, "{found:?}");
    for file in &found {
        assert!(PAINTED.iter().any(|(f, _, _)| f == file), "src/{file} holds a window: it needs a test that paints it for its pairs (`in_every_look`), named in PAINTED");
    }
    let listed: Vec<(&str, &str)> = PAINTED.iter().flat_map(|(_, by, tests)| tests.iter().map(move |t| (*by, *t))).chain(ALSO.iter().copied()).collect();
    for (file, by, tests) in PAINTED {
        assert!(found.iter().any(|f| f == file), "src/{file} holds no window now: take it out of PAINTED");
        assert!(!tests.is_empty(), "src/{file} names no test in src/{by}");
    }
    for (by, test) in &listed {
        assert!(test.ends_with(NAMED), "{test} must end {NAMED}, to be run again in every theme");
        let text = std::fs::read_to_string(crate::common::source::path_of(by)).unwrap();
        let Some(at) = text.find(&format!("fn {test}() {{")) else { panic!("src/{by} has no test named {test}") };
        let before: Vec<&str> = text[..at].lines().rev().take_while(|l| l.trim().starts_with('#') || l.trim().is_empty()).collect();
        assert!(before.iter().any(|l| l.trim() == "#[test]") && !before.iter().any(|l| l.contains("ignore")), "src/{by}: {test} is not a test that runs");

        let inset = text[..at].chars().rev().take_while(|c| *c == ' ').count();
        let body = &text[at..];
        let body = &body[..body.find(&format!("\n{}}}\n", " ".repeat(inset))).unwrap_or(body.len())];
        assert!(body.contains("in_every_look(") || body.contains("in_look(") || body.contains("drawn::highlights()"), "src/{by}: {test} holds nothing in every look");
    }

    for (file, test) in &named {
        if file == "toolkit/theme/drawn.rs" {
            continue;
        }
        assert!(listed.iter().any(|(by, t)| by == file && t == test), "src/{file}: {test} ends {NAMED} and is on neither list (PAINTED, ALSO)");
    }

    let mut names: Vec<&str> = named.iter().map(|(_, t)| t.as_str()).collect();
    names.sort_unstable();
    let all = names.len();
    names.dedup();
    assert_eq!(names.len(), all, "two painting tests share a name");
}

