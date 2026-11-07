//! Desktop management helpers.

pub mod space;
pub use self::space::Space;

#[cfg(feature = "wayland_frontend")]
pub use self::wayland::{
    layer::{layer_map_for_output, LayerMap, LayerSurface},
    popup::*,
    utils,
    window::*,
};
#[cfg(feature = "wayland_frontend")]
mod wayland {
    pub(crate) mod layer;
    pub mod popup;
    pub mod utils;
    pub mod window;
}
