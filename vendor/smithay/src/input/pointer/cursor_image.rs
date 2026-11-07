#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_surface::WlSurface;

pub use cursor_icon::CursorIcon;

use crate::utils::{Logical, Point};
use std::sync::Mutex;

#[derive(Debug, Default, Copy, Clone)]
pub struct CursorImageAttributes {
    pub hotspot: Point<i32, Logical>,
}

pub type CursorImageSurfaceData = Mutex<CursorImageAttributes>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorImageStatus {
    Hidden,
    Named(CursorIcon),
    #[cfg(feature = "wayland_frontend")]
    Surface(WlSurface),
}

impl CursorImageStatus {
    pub fn default_named() -> Self {
        Self::Named(CursorIcon::Default)
    }
}
