//! Every Settings page, in the order the sidebar shows them.

pub mod ring;
pub mod display;
pub mod audio;
pub mod sounds;
pub mod network;
pub mod tailscale;
pub mod blocking;
pub mod bluetooth;
pub mod wallpaper;
pub mod about;
pub mod appearance;
pub mod keyboard;
pub mod pads;
pub mod power;
pub mod default_apps;
pub mod printers;
pub mod region;
pub mod startup;
pub mod thanks;
pub mod datetime;
pub mod users;
pub mod snapshots;
pub mod help;
pub mod plugin_rows;
pub mod crash_reports;
pub mod plugins;
pub mod permissions;
mod outside_card;
pub mod ai_programs;
pub mod games;

use super::ui::Page;

pub fn all() -> Vec<Box<dyn Page>> {
    let mut all: Vec<Box<dyn Page>> = vec![
        Box::new(ring::Ring::default()),
        Box::new(display::Display::default()),
        Box::new(audio::Audio::default()),
        Box::new(sounds::Sounds::default()),
        Box::new(network::Network::default()),
        Box::new(tailscale::Tailscale::default()),
        Box::new(blocking::Blocking::default()),
        Box::new(bluetooth::Bluetooth::default()),
        Box::new(wallpaper::Wallpaper::default()),
        Box::new(appearance::Appearance::default()),
        Box::new(keyboard::Keyboard::default()),
        Box::new(power::Power::default()),
        Box::new(games::Games::default()),
        Box::new(default_apps::DefaultApps::default()),
        Box::new(printers::Printers::default()),
        Box::new(region::Region::default()),
        Box::new(datetime::Datetime::default()),
        Box::new(users::Users::default()),
        Box::new(snapshots::Snapshots::default()),
        Box::new(permissions::Permissions::default()),
        Box::new(ai_programs::AiPrograms::default()),
        Box::new(startup::Startup::default()),
        Box::new(about::About::default()),
        Box::new(thanks::Thanks::default()),
        Box::new(help::Help::default()),
    ];
    all.push(Box::new(crash_reports::CrashReports::default()));
    if plugins::here() {
        all.push(Box::new(plugins::Plugins::default()));
    }
    all
}
