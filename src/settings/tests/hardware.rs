//! Tests: the Hardware page, from made-up devices.

use super::*;
use crate::common::no_driver::{Device, Inventory, Kind, State};

fn device(kind: Kind, id: &str, state: State, problem: Option<&str>) -> Device {
    Device { kind, id: id.into(), name: None, driver: None, state, problem: problem.map(Into::into) }
}

fn some_devices() -> Inventory {
    Inventory {
        devices: vec![
            device(Kind::Graphics, "8086:9a49", State::Working, None),
            device(Kind::Sound, "8086:51c8", State::NoFirmware, Some("Missing sound firmware: intel/sof/x.ri.")),
            device(Kind::Network, "14c3:7902", State::NoDriver, Some("No driver.")),
        ],
        loose: Vec::new(),
    }
}

fn cards_of(inv: Inventory) -> Vec<Option<String>> {
    cards_on(panels::hardware::Hardware::with(inv))
}

fn cards_on(page: panels::hardware::Hardware) -> Vec<Option<String>> {
    let s = showing(Box::new(page), 900.0);
    let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
    ui.lay_at(900, 900);
    ui.puzzle_cards_named().into_iter().map(|c| c.0).collect()
}

#[test]
fn the_page_has_a_card_for_each_kind_present_and_one_for_the_problems() {
    let cards = cards_of(some_devices());
    let has = |name: &str| cards.contains(&Some(crate::settings::ui::card_key(name)));
    assert!(has(t!("settings-hardware-kind-graphics")) && has(t!("settings-hardware-kind-sound")) && has(t!("settings-hardware-kind-network")), "{cards:?}");
    assert!(!has(t!("settings-hardware-kind-storage")) && !has(t!("settings-hardware-kind-camera")), "{cards:?}");
    assert!(cards.contains(&None), "no card for the problems: {cards:?}");
}

#[test]
fn a_computer_with_nothing_wrong_has_no_problems_card() {
    let inv = Inventory { devices: vec![device(Kind::Graphics, "8086:9a49", State::Working, None)], loose: Vec::new() };
    assert!(!cards_of(inv).contains(&None));
}

#[test]
fn hardware_is_listed_under_devices() {
    let group = page_groups().into_iter().find(|g| g.1.contains(&"hardware")).map(|g| g.0);
    assert_eq!(group, Some(t!("settings-window-group-devices")));
}

#[test]
fn the_chip_card_shows_only_after_the_chip_did_not_unlock_the_disk() {
    let chip = Some(crate::settings::ui::card_key(t!("settings-hardware-chip")));
    let inv = || Inventory { devices: vec![device(Kind::Graphics, "8086:9a49", State::Working, None)], loose: Vec::new() };
    assert!(!cards_on(panels::hardware::Hardware::with(inv())).contains(&chip));
    let refused = panels::hardware::Hardware::with(inv()).chip_refused("0b5f6a7e-1c2d-4e3f-8a9b-0c1d2e3f4a5b");
    assert!(cards_on(refused).contains(&chip));
}

#[test]
fn the_start_up_screen_names_the_button_settings_has() {
    let fix = trindle_toolkit::t!("unlock-chip-refused-fix");
    assert!(fix.contains(t!("settings-hardware-title")) && fix.contains(t!("settings-hardware-chip-again")), "{fix}");
}

#[test]
#[ignore = "writes pictures to TRINDLE_SHOTS and asserts nothing; run it by hand with --ignored"]
fn shoot_chip_card() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    std::fs::create_dir_all(&out).unwrap();
    for dark in [false, true] {
        for open in [false, true] {
            let mut page = panels::hardware::Hardware::with(some_devices()).chip_refused("0b5f6a7e-1c2d-4e3f-8a9b-0c1d2e3f4a5b");
            if open {
                page = page.chip_asking();
            }
            let s = showing(Box::new(page), 900.0);
            let mut ui = Ui::new(s.view(), Theme::new(dark, "#fbc711"));
            let mut pix = tiny_skia::Pixmap::new(1100, 900).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(out.join(format!("chip-card-{}-{}.png", if open { "open" } else { "closed" }, if dark { "dark" } else { "light" }))).unwrap();
        }
    }
}
