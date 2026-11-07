//! Tests for the Settings window.

use crate::common::info_popup_tests::{shuts_like_every_other, THE_I, UNDER_THE_I};

use std::cell::RefCell;
use std::time::Duration;
use super::*;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
mod sidebar;
mod pages;
mod moving;
mod colours;
mod whole;
mod trial;

fn settings_with_every_page() -> Settings {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut pages = panels::all();
    let plugins: Vec<Box<dyn Page>> = vec![
        Box::new(panels::crash_reports::CrashReports::default()),
        Box::new(panels::plugins::Plugins::default()),
    ];
    for p in plugins {
        if !pages.iter().any(|q| q.id() == p.id()) {
            pages.push(p);
        }
    }
    pages.retain(|p| !IN_APPS.contains(&p.id()));
    let listed: usize = page_groups().iter().map(|(_, ids)| ids.len()).sum();
    assert_eq!(pages.len(), listed, "every page is in a group");
    Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 0.0 }
}

const WIDE_BY_CHOICE: &str = "wallpaper";

fn by_choice(id: &str, mut found: crate::toolkit::widget::shape::Shape, across: f32) -> crate::toolkit::widget::shape::Shape {
    if id == WIDE_BY_CHOICE {
        found.wide = found.wide.saturating_sub(1);
        let wide = |bar: &String| bar.rsplit(' ').next().and_then(|size| size.split('x').next()).and_then(|w| w.parse::<f32>().ok())
            .is_some_and(|w| w > across / 2.0);
        if let Some(at) = found.bars.iter().position(wide) {
            found.bars.remove(at);
        }
    }
    found
}

fn flipped_pages() -> Vec<Vec<Box<dyn Page>>> {
    use panels::*;
    fn boxed<P: Page + 'static>(all: Vec<P>) -> Vec<Box<dyn Page>> {
        all.into_iter().map(|p| Box::new(p) as Box<dyn Page>).collect()
    }
    vec![
        boxed(blocking::Blocking::flipped()), boxed(bluetooth::Bluetooth::flipped()), boxed(tailscale::Tailscale::flipped()),
        boxed(wallpaper::Wallpaper::flipped()), boxed(display::Display::flipped()), boxed(startup::Startup::flipped()),
        boxed(printers::Printers::flipped()), boxed(network::Network::flipped()), boxed(power::Power::flipped()),
        boxed(help::Help::flipped()),
    ]
}

fn hues_in(pix: &tiny_skia::Pixmap, accent: crate::toolkit::colour::Colour, ignore: &[crate::toolkit::colour::Colour], offered: &[crate::toolkit::colour::Colour]) -> Vec<usize> {
    let bytes = |c: &crate::toolkit::colour::Colour| [c.r, c.g, c.b].map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8);
    let offered: Vec<[u8; 3]> = offered.iter().map(bytes).collect();
    let slice = |r: f32, g: f32, b: f32| -> Option<usize> {
        let (max, min) = (r.max(g).max(b), r.min(g).min(b));
        if max < 0.35 || max - min < 0.45 * max {
            return None;
        }
        let d = max - min;
        let h = if max == r { ((g - b) / d).rem_euclid(6.0) } else if max == g { (b - r) / d + 2.0 } else { (r - g) / d + 4.0 };
        Some(((h * 60.0) / 30.0) as usize % 12)
    };
    let mut skip: Vec<usize> = ignore.iter().chain(std::iter::once(&accent)).filter_map(|c| slice(c.r, c.g, c.b)).collect();
    if let Some(a) = slice(accent.r, accent.g, accent.b) {
        skip.extend([(a + 1) % 12, (a + 11) % 12]);
    }
    let mut counts = [0usize; 12];
    for p in pix.pixels().iter().step_by(2) {
        let c = p.demultiply();
        if offered.contains(&[c.red(), c.green(), c.blue()]) {
            continue;
        }
        if let Some(k) = slice(c.red() as f32 / 255.0, c.green() as f32 / 255.0, c.blue() as f32 / 255.0) {
            counts[k] += 1;
        }
    }
    (0..12).filter(|k| counts[*k] >= 300 && !skip.contains(k)).collect()
}

fn made_up_pages() -> Vec<Box<dyn Page>> {
    use panels::*;
    vec![
        Box::new(ring::Ring::made_up()), Box::new(display::Display::made_up()), Box::new(audio::Audio::made_up()),
        Box::new(sounds::Sounds::default()), Box::new(network::Network::made_up()),
        Box::new(tailscale::Tailscale::made_up()), Box::new(blocking::Blocking::made_up()),
        Box::new(bluetooth::Bluetooth::made_up()), Box::new(wallpaper::Wallpaper::made_up()),
        Box::new(appearance::Appearance::made_up()),
        Box::new(keyboard::Keyboard::made_up()), Box::new(power::Power::made_up()), Box::new(games::Games::made_up()),
        Box::new(default_apps::DefaultApps::made_up()), Box::new(printers::Printers::made_up()),
        Box::new(region::Region::made_up()), Box::new(datetime::Datetime::made_up()), Box::new(users::Users::made_up()),
        Box::new(snapshots::Snapshots::made_up()), Box::new(permissions::Permissions::made_up()), Box::new(ai_programs::AiPrograms::made_up()),
        Box::new(startup::Startup::made_up()), Box::new(about::About::made_up()),
        Box::new(thanks::Thanks::default()), Box::new(help::Help::default()),
        Box::new(crash_reports::CrashReports::made_up()), Box::new(plugins::Plugins::made_up()),
    ]
}

fn every_page() -> Vec<Box<dyn Page>> {
    let mut pages = panels::all();
    for p in [
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ] {
        if !pages.iter().any(|q| q.id() == p.id()) {
            pages.push(p);
        }
    }
    pages
}

const NARROWEST: u32 = 400;

fn showing(page: Box<dyn Page>, tall: f32) -> Settings {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events.clone(), asks, None, None);
    kit.current = page.id().to_string();
    kit.icon = page.icon();
    Settings { kit, pages: vec![page], current: 0, search: String::new(), tall }
}

fn on_trial(id: &str) -> (Box<dyn Page>, Box<dyn Page>) {
    use panels::{appearance::Appearance, display::Display};
    match id {
        "appearance" => (Box::new(Appearance::made_up()), Box::new(Appearance::on_trial())),
        _ => (Box::new(Display::made_up()), Box::new(Display::on_trial())),
    }
}
