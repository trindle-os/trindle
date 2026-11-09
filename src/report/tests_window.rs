//! Tests for the window that shows a report.

use std::path::PathBuf;

use super::*;

fn who() -> Who {
    Who { home: "/home/jsmith".into(), names: vec!["jsmith".into()], ssids: Vec::new() }
}

fn made(mode: Mode, details: bool, gate: Gate) -> Reporter {
    let mut r = Reporter::new(who(), "Example Computer".into(), PathBuf::from("/nonexistent"));
    r.mode = mode;
    r.details = details;
    r.switch = true;
    r.gate = gate;
    r.log = Some(vec!["line one".into(); 6]);
    r
}

#[test]
fn every_state_of_the_window_fits() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    for mode in [Mode::Note, Mode::Crash] {
        for (details, gate) in [(false, Gate::Asking), (true, Gate::Shut), (true, Gate::Open)] {
            let mut r = made(mode, details, gate);
            r.waiting = Some("No connection.".into());
            r.sent = true;
            let ui = Ui::new(r.view(), Theme::chosen());
            assert!(ui.natural_height(SIZE.0 as f32) <= SIZE.1 as f32, "{mode:?}, details {details}");
        }
    }
}

#[test]
fn the_window_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    drawn::none(drawn::in_every_look("the report window", 8, || {
        for (state, mode, details, refusal) in [("note", Mode::Note, false, ""), ("crash details", Mode::Crash, true, ""),
                                                ("refused", Mode::Crash, false, "Refused.")] {
            let mut r = made(mode, details, Gate::Open);
            r.refusal = refusal.into();
            drawn::window(state, r.view(), Theme::chosen(), SIZE, false);
        }
        let mut waiting = made(Mode::Crash, false, Gate::Open);
        waiting.waiting = Some("No connection.".into());
        drawn::window("waiting", waiting.view(), Theme::chosen(), SIZE, false);
        drawn::window("shut", made(Mode::Note, false, Gate::Shut).view(), Theme::chosen(), SIZE, false);
    }));
}
