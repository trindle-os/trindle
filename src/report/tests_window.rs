//! Tests for the window that shows a report.

use std::path::PathBuf;

use super::*;

fn who() -> Who {
    Who {
        home: "/home/jsmith".into(),
        names: vec!["jsmith".into()],
        ssids: Vec::new(),
    }
}

fn made(mode: Mode, details: bool, gate: Gate) -> Reporter {
    let mut r = Reporter::new(
        who(),
        "Example Computer".into(),
        PathBuf::from("/nonexistent"),
    );
    r.mode = mode;
    r.details = details;
    r.switch = true;
    r.gate = gate;
    r.log = Some(vec!["line one".into(); 6]);
    r
}

#[test]
fn every_state_of_the_window_fits() {
    for mode in [Mode::Note, Mode::Crash] {
        for (details, gate) in [
            (false, Gate::Asking),
            (true, Gate::Shut),
            (true, Gate::Open),
        ] {
            let mut r = made(mode, details, gate);
            r.waiting = Some("No connection.".into());
            r.sent = true;
            assert!(needed(&r) <= SIZE.1 as f32, "{mode:?}, details {details}");
        }
    }
}

#[test]
fn the_what_happened_box_is_ten_lines_and_stays_that_tall_whatever_is_typed() {
    const { assert!(WRITING_LINES >= 10) };
    assert!(
        include_str!("mod.rs").contains("t!(\"report-what-doing\"), Msg::Typed).lines(self.lines)"),
        "the box is as tall as the setting"
    );
    assert_eq!(
        made(Mode::Note, false, Gate::Open).lines,
        WRITING_LINES,
        "it starts at the constant"
    );
    for details in [false, true] {
        let height = |typed: &str| {
            let mut r = made(Mode::Note, details, Gate::Open);
            r.fit_to(None);
            r.typed = typed.into();
            (needed(&r), r)
        };
        let (empty, r) = height("");
        assert_eq!((r.lines, r.tight), (WRITING_LINES, false));
        assert_eq!(
            empty,
            height(&["a line"; WRITING_LINES].join("\n")).0,
            "ten lines fit, details {details}"
        );
        assert_eq!(
            empty,
            height(&["a line"; WRITING_LINES + 20].join("\n")).0,
            "more lines scroll, they do not grow it, details {details}"
        );
        assert_eq!(
            empty,
            height(&"word ".repeat(5000)).0,
            "a long wrapped text does not grow it, details {details}"
        );
        assert!(empty <= SIZE.1 as f32, "fits, details {details}");
    }
}

#[test]
fn the_box_height_follows_its_lines() {
    let high = |lines: usize| {
        let mut r = made(Mode::Note, false, Gate::Open);
        r.set_box(lines, false);
        r.box_high.unwrap_or_default()
    };
    let line = high(WRITING_LINES) - high(WRITING_LINES - 1);
    assert!(line > 0.0);
    assert!(
        (high(WRITING_LINES)
            - high(WRITING_LINES_LEAST)
            - (WRITING_LINES - WRITING_LINES_LEAST) as f32 * line)
            .abs()
            < 1.0
    );
}

#[test]
fn on_a_short_screen_the_details_pane_shrinks_so_the_open_window_fits_too() {
    for room in [Some(768.0 / 1.5), Some(768.0 / 1.5 - 48.0)] {
        let room = room.unwrap_or_default();
        for typed in ["", "words\n".repeat(200).as_str()] {
            let mut r = made(Mode::Note, true, Gate::Open);
            r.typed = typed.into();
            r.fit_to(Some(room));
            assert!(needed(&r) <= room + 0.5, "needs {} of {room}", needed(&r));
            assert!(
                r.details_high < DETAILS_HIGH && r.details_high >= DETAILS_LEAST,
                "{}",
                r.details_high
            );
        }
    }
    let mut r = made(Mode::Note, true, Gate::Open);
    r.fit_to(Some(1.0));
    assert_eq!(
        (r.details_high, r.lines),
        (DETAILS_LEAST, WRITING_LINES_DETAILS_LEAST),
        "never below their least"
    );
    r.fit_to(None);
    assert_eq!(r.details_high, DETAILS_HIGH, "a normal screen puts it back");
    for over in [0.0, -5.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(
            (DETAILS_LEAST..=DETAILS_HIGH).contains(&super::room::details_for(over)),
            "{over}"
        );
    }
}

fn pick_by_name(r: &mut Reporter, name: &str) {
    let at = r
        .picks()
        .iter()
        .position(|p| p.name == name)
        .unwrap_or_else(|| panic!("no choice called {name}"));
    r.update(Msg::App(at));
}

fn said(r: &Reporter) -> (String, String) {
    let v: serde_json::Value = serde_json::from_str(&r.text()).unwrap();
    (
        v["project"].as_str().unwrap().into(),
        v["words"].as_str().unwrap_or_default().into(),
    )
}

#[test]
fn the_app_picked_decides_the_project_and_starts_the_words() {
    let mut r = made(Mode::Note, false, Gate::Open);
    let own = ["org.", "trindle", "."].concat();
    r.plugins = vec![
        (format!("{own}Flow"), "Flow".into()),
        ("org.example.Other".into(), "Other".into()),
    ];
    r.typed = "It stopped.".into();
    assert_eq!(
        said(&r),
        ("trindle".into(), "It stopped.".into()),
        "not sure is the start, and says nothing of an app"
    );
    pick_by_name(&mut r, "Folders");
    assert_eq!(
        said(&r),
        ("trindle-folders".into(), "App: Folders. It stopped.".into())
    );
    pick_by_name(&mut r, "Flow");
    assert_eq!(
        said(&r),
        ("trindle-flow".into(), "App: Flow. It stopped.".into())
    );
    pick_by_name(&mut r, "Settings");
    assert_eq!(
        said(&r),
        ("trindle".into(), "App: Settings. It stopped.".into())
    );
    pick_by_name(&mut r, "Another program");
    assert_eq!(
        said(&r),
        ("trindle".into(), "App: Another program. It stopped.".into())
    );
    pick_by_name(&mut r, "Not sure");
    assert_eq!(said(&r).0, "trindle");
    assert!(
        r.picks().iter().all(|p| p.name != "Other"),
        "a plugin with no project of its own is not offered"
    );
}

#[test]
fn a_crash_preselects_the_program_that_crashed() {
    let mut r = made(Mode::Crash, false, Gate::Open);
    r.program = send::FOLDERS_PROGRAM.into();
    assert_eq!(said(&r), ("trindle-folders".into(), "App: Folders.".into()));
    assert_eq!(
        r.picks()[which::place(&r.picks(), &r.app_program())].name,
        "Folders"
    );
    pick_by_name(&mut r, "Not sure");
    assert_eq!(
        said(&r).0,
        "trindle",
        "the person's pick wins over the crash"
    );
    let mut ring = made(Mode::Crash, false, Gate::Open);
    ring.program = "trindle-ring".into();
    ring.crash = Some(Crash {
        program: "trindle-store".into(),
        signal: String::new(),
        trace: String::new(),
    });
    assert_eq!(
        said(&ring),
        ("trindle".into(), "App: Apps.".into()),
        "the crash read from the core dump names the program, under either of its names"
    );
    let mut odd = made(Mode::Crash, false, Gate::Open);
    odd.program = "trindle-lock".into();
    assert!(
        odd.picks().iter().any(|p| p.name == "trindle-lock"),
        "a program not in the list is offered as it is"
    );
    assert_eq!(said(&odd).1, "App: trindle-lock.");
    let plain = made(Mode::Crash, false, Gate::Open);
    assert_eq!(
        said(&plain).1,
        "",
        "a crash with no program named starts at not sure"
    );
}

#[test]
fn the_window_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    drawn::none(drawn::in_every_look("the report window", 8, || {
        for (state, mode, details, refusal) in [
            ("note", Mode::Note, false, ""),
            ("crash details", Mode::Crash, true, ""),
            ("refused", Mode::Crash, false, "Refused."),
        ] {
            let mut r = made(mode, details, Gate::Open);
            r.refusal = refusal.into();
            drawn::window(state, r.view(), Theme::chosen(), SIZE, false);
        }
        let mut waiting = made(Mode::Crash, false, Gate::Open);
        waiting.waiting = Some("No connection.".into());
        drawn::window("waiting", waiting.view(), Theme::chosen(), SIZE, false);
        let mut pictured = made(Mode::Note, false, Gate::Open);
        let mut tiny = Vec::new();
        image::RgbaImage::new(16, 9).write_to(&mut std::io::Cursor::new(&mut tiny), image::ImageFormat::Png).unwrap();
        pictured.picture_is(super::picture::load(&tiny));
        pictured.picture_fault = Some("Damaged picture.".into());
        drawn::window("picture", pictured.view(), Theme::chosen(), SIZE, false);
        drawn::window(
            "shut",
            made(Mode::Note, false, Gate::Shut).view(),
            Theme::chosen(),
            SIZE,
            false,
        );
    }));
}

#[test]
fn the_window_is_fitted_to_what_it_shows_with_no_room_spare() {
    let window = include_str!("window.rs");
    let opened = window
        .split("fn open(")
        .nth(1)
        .and_then(|o| o.split("\n}\n").next())
        .unwrap_or_default();
    assert!(opened.contains("refit(&shown);"), "fitted once open");
    let answering = window
        .split("fn answering(")
        .nth(1)
        .and_then(|o| o.split("\n}\n").next())
        .unwrap_or_default();
    assert_eq!(
        answering.matches("refit(&shown);").count(),
        2,
        "fitted after every answer and every change from the threads"
    );
    let details = include_str!("mod.rs")
        .split("Msg::Details =>")
        .nth(1)
        .unwrap_or_default();
    assert!(
        details
            .split("Msg::Send")
            .next()
            .unwrap_or_default()
            .contains("Do::Fit"),
        "showing or hiding the details fits the window again"
    );
}

#[test]
fn the_shown_text_is_kept_until_something_it_is_made_from_changes() {
    let mut r = made(Mode::Note, true, Gate::Open);
    r.typed = "it froze".into();
    let first = r.shown_text();
    assert_eq!(first, r.text());
    r.shown.borrow_mut().as_mut().unwrap().1 = "kept".into();
    assert_eq!(
        r.shown_text(),
        "kept",
        "nothing changed, so it is not made again"
    );
    r.typed = "it froze twice".into();
    assert_eq!(r.shown_text(), r.text(), "a change makes it again");
}

fn needed(r: &Reporter) -> f32 {
    super::room::natural(r.view())
}

#[test]
fn on_a_short_screen_at_a_large_text_scale_the_whole_window_is_in_view() {
    for details in [false, true] {
        for room in [Some(768.0 / 1.5), Some(768.0 / 1.5 - 48.0)] {
            let mut r = made(Mode::Note, details, Gate::Open);
            r.fit_to(room);
            let room = room.unwrap_or_default();
            assert!(
                needed(&r) <= room + 0.5,
                "needs {} of {room}, details {details}",
                needed(&r)
            );
            assert!(
                r.lines
                    >= if details {
                        WRITING_LINES_DETAILS_LEAST
                    } else {
                        WRITING_LINES_LEAST
                    }
            );
        }
    }
}

#[test]
fn on_a_normal_screen_nothing_changes() {
    for room in [
        Some(SIZE.1 as f32),
        Some(1080.0 / 1.0),
        Some(2160.0 / 2.0),
        None,
    ] {
        let mut r = made(Mode::Note, false, Gate::Open);
        r.fit_to(room);
        assert_eq!((r.lines, r.tight), (WRITING_LINES, false), "{room:?}");
    }
}

#[test]
fn a_tiny_or_nonsense_room_never_takes_the_box_below_its_least_and_never_panics() {
    for room in [
        Some(1.0),
        Some(0.5),
        Some(768.0 / 1_000_000.0),
        Some(1.0 / 1.0),
    ] {
        for (details, least) in [
            (false, WRITING_LINES_LEAST),
            (true, WRITING_LINES_DETAILS_LEAST),
        ] {
            let mut r = made(Mode::Note, details, Gate::Open);
            r.fit_to(room);
            assert_eq!(
                (r.lines, r.tight),
                (least, true),
                "{room:?}, details {details}"
            );
            let _ = needed(&r);
        }
    }
    for scale in [0.0, -1.5, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut r = made(Mode::Note, false, Gate::Open);
        r.fit_to(Some(768.0 / scale));
        assert!(
            (WRITING_LINES_LEAST..=WRITING_LINES).contains(&r.lines),
            "scale {scale}"
        );
    }
    for room in [Some(f32::NAN), Some(f32::INFINITY), Some(-5.0), Some(0.0)] {
        let mut r = made(Mode::Note, false, Gate::Open);
        r.fit_to(room);
        assert_eq!((r.lines, r.tight), (WRITING_LINES, false), "{room:?}");
    }
}

#[test]
fn the_squeeze_prefers_fewer_lines_before_tighter_gaps() {
    use super::room::pick;
    let tall = |n: usize, tight: bool| n as f32 * 20.0 + if tight { 100.0 } else { 150.0 };
    assert_eq!(
        pick(Some(tall(7, false)), WRITING_LINES_LEAST, tall),
        (7, false)
    );
    assert_eq!(
        pick(
            Some(tall(WRITING_LINES_LEAST, false) - 1.0),
            WRITING_LINES_LEAST,
            tall
        ),
        (5, true)
    );
    assert_eq!(
        pick(Some(tall(1, true)), WRITING_LINES_DETAILS_LEAST, tall),
        (1, true),
        "a lower least is reached when asked"
    );
}

#[test]
fn with_the_details_open_on_a_short_screen_the_box_gets_the_most_lines_that_fit() {
    let mut checked = 0;
    for room in (150..700).step_by(7) {
        let room = room as f32;
        let mut r = made(Mode::Note, true, Gate::Open);
        r.fit_to(Some(room));
        if needed(&r) > room + 0.5 || r.lines >= WRITING_LINES {
            continue;
        }
        checked += 1;
        r.set_box(r.lines + 1, r.tight);
        assert!(
            needed(&r) > room + 0.5,
            "room {room}: one more line would still fit"
        );
    }
    assert!(checked > 20, "{checked}");
}

#[test]
fn every_gap_is_named() {
    for source in [include_str!("mod.rs"), include_str!("window.rs")] {
        let code = crate::common::source::code(source);
        assert!(!code.split(".gap(").skip(1).any(|rest| rest.starts_with(|c: char| c.is_ascii_digit())), "a gap is written as a bare number");
    }
}

#[test]
fn on_a_720_screen_the_whole_window_is_in_view_in_every_language() {
    use crate::common::shipped_languages::{in_language, tags};
    let room = 720.0 - 48.0;
    for tag in tags() {
        in_language(&tag, || {
            for mode in [Mode::Note, Mode::Crash] {
                for (details, typed) in [(false, ""), (false, "It froze twice when I opened it."), (true, "It froze twice when I opened it.")] {
                    let mut r = made(mode, details, Gate::Open);
                    r.typed = typed.into();
                    r.fit_to(Some(room));
                    let send = r.view().iconed().into_iter().any(|(tip, _)| tip == t!("report-send"));
                    assert!(send || typed.is_empty(), "{tag}: no Send");
                    assert!(needed(&r) <= room + 0.5, "{tag} {mode:?}, details {details}, {typed:?}: needs {} of {room}", needed(&r));
                }
            }
        });
    }
}

fn press_on(ui: &mut crate::toolkit::ui::Ui<Msg>, name: &str) -> Vec<Msg> {
    use crate::toolkit::input::{Button, Input};
    let now = std::time::Instant::now();
    let tree = ui.access("");
    let b = tree.nodes.iter().find(|(_, p)| p.label() == Some(name) && p.role() != accesskit::Role::Window)
        .and_then(|(_, p)| p.bounds()).unwrap_or_else(|| panic!("no {name}"));
    let (x, y) = (((b.x0 + b.x1) / 2.0) as f32, ((b.y0 + b.y1) / 2.0) as f32);
    let mut got = ui.input(&Input::PointerMove { x, y }, now).messages;
    got.extend(ui.input(&Input::Press { button: Button::Left, x, y }, now).messages);
    got.extend(ui.input(&Input::Release { button: Button::Left, x, y }, now).messages);
    got
}

#[test]
fn close_after_sending_closes_at_the_first_press() {
    use crate::toolkit::{theme::Theme, ui::Ui};
    let mut r = made(Mode::Note, false, Gate::Open);
    r.typed = "The panel went blank after the lid closed".into();
    let mut ui = Ui::new(r.view(), Theme::chosen());
    ui.lay_at(SIZE.0, SIZE.1);

    r.sent = true;
    r.sending = true;
    ui.set_view(r.view());
    ui.lay_at(SIZE.0, SIZE.1);
    r.sending = false;
    r.delivered = true;
    ui.set_view(r.view());
    ui.lay_at(SIZE.0, SIZE.1);
    let got = press_on(&mut ui, t!("screenshot-close"));
    assert!(got.iter().any(|m| matches!(m, Msg::Cancel)), "the first press on Close did nothing");
}
