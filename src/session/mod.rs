//! Background programs run by the session.

pub mod backdrop;
pub mod decoding;
pub mod brightness;
pub mod folders;
pub mod idle;
pub mod lock;
pub mod nightlight;
pub mod restart_ask;
pub mod scale;
pub mod moving;
pub mod start;
pub mod screensaver;
pub mod starting;
pub mod wallpaper;
pub mod wallpaper_limits;
mod wayland_loop;
pub mod wsdd;
