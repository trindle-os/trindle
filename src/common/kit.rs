//! The colours that say how things are going, shared by every Trindle window and the compositor.

use super::sheet;

pub type Rgb = (f64, f64, f64);

fn named(name: &str) -> Rgb {
    let [r, g, b, _] = sheet::layers().colour(name, false).unwrap_or([0.5, 0.5, 0.5, 1.0]);
    (r as f64, g as f64, b as f64)
}

pub const MICROPHONE_NAME: &str = "trindle-busy";
pub const CAMERA_NAME: &str = "trindle-good";
pub const SCREEN_NAME: &str = "trindle-calm";
pub const INPUT_NAME: &str = "trindle-use-input";
pub const FILES_NAME: &str = "trindle-use-files";
pub const NETWORK_NAME: &str = "trindle-use-network";
pub const USB_NAME: &str = "trindle-use-usb";

pub fn named_in(name: &str, light: bool) -> Rgb {
    let [r, g, b, _] = sheet::layers().colour(name, light).unwrap_or([0.5, 0.5, 0.5, 1.0]);
    (r as f64, g as f64, b as f64)
}

pub fn microphone() -> Rgb {
    named(MICROPHONE_NAME)
}
pub fn camera() -> Rgb {
    named(CAMERA_NAME)
}
pub fn screen() -> Rgb {
    named(SCREEN_NAME)
}
pub fn input() -> Rgb {
    named(INPUT_NAME)
}
pub fn files() -> Rgb {
    named(FILES_NAME)
}
pub fn home_network() -> Rgb {
    named(NETWORK_NAME)
}
pub fn usb() -> Rgb {
    named(USB_NAME)
}
