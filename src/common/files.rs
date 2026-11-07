//! Trindle's own settings and desktop files.

pub use trindle_toolkit::common::files::*;

use std::path::{Path, PathBuf};

use super::pointer;

pub const SYSTEM_DESKTOP: &str = "/usr/share/trindle/desktop";

pub fn system_desktop(name: &str) -> PathBuf {
    let installed = Path::new(SYSTEM_DESKTOP).join(name);
    if installed.is_file() {
        return installed;
    }
    PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default()).join("config/desktop").join(name)
}

pub fn user_environment() -> PathBuf {
    trindle_config().join("environment")
}

pub fn default_setting(key: &str) -> &'static str {
    match key {
        "restart-parts-keys" => "Ctrl+Alt+Escape",
        "restart-all-keys" => "Ctrl+Alt+Shift+Escape",
        "screenshot-keys" => "Ctrl+Alt+Shift+S",
        "shortcut-list-keys" => "Super+K",
        "report-keys" => "Ctrl+Alt+R",
        "report_offer" => "on",
        "clock_format" => "%H:%M",
        "wallpaper_color" => "#09090b",
        "dim_after" => "5",
        "lock_after" => "10",
        "lock_on_suspend" => "yes",
        "ring_window_names" => "yes",
        "check_updates_by_itself" => "yes",
        "security_updates" => "yes",
        "lock" => "off",
        "blank_after" => "10",
        "suspend_after" => "30",
        "dim_after_battery" => "2",
        "screensaver_after" => "5",
        "lid_close" => "suspend",
        "lid_close_battery" => "suspend",
        "screensaver_after_battery" => "3",
        "blank_after_battery" => "5",
        "suspend_after_battery" => "15",
        "wallpaper_every" => "300",
        "wallpaper_order" => "shuffle",
        "wallpaper_fit" => "fill",
        "icon_source" => "Trindle-Circle",
        "older_programs" => "yes",
        "outside_network" | "outside_files" | "downloads_boxed" => "yes",
        pointer::SPEED => "0",
        pointer::SCROLL => "4",
        pointer::TOUCHPAD_SCROLL => "1.5",
        pointer::MOUSE_NATURAL => "no",
        pointer::LEFT_HANDED => "no",
        pointer::TAP => "yes",
        pointer::TOUCHPAD_NATURAL => "yes",
        pointer::QUIET_TYPING => "yes",
        _ => "",
    }
}

pub fn setting(key: &str) -> String {
    read_env(&trindle_conf())
        .get(key)
        .cloned()
        .unwrap_or_else(|| default_setting(key).to_string())
}

pub fn set_setting(key: &str, value: &str) -> std::io::Result<()> {
    update_env(&trindle_conf(), &[(key, Some(value))])
}
