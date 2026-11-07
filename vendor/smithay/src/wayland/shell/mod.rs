//! Handler utilities for the various shell protocols.

use crate::{utils::Serial, wayland::compositor};
use thiserror::Error;
use wayland_server::protocol::wl_surface::WlSurface;
use xdg::XdgToplevelSurfaceData;

pub mod kde;
pub mod wlr_layer;
pub mod xdg;

#[derive(Debug, Error)]
pub enum PingError {
    #[error("the ping failed cause the underlying surface has been destroyed")]
    DeadSurface,
    #[error("there is already a ping pending `{0:?}`")]
    PingAlreadyPending(Serial),
}

pub fn is_toplevel_equivalent(surface: &WlSurface) -> bool {
    let role = compositor::get_role(surface);

    matches!(role, Some(xdg::XDG_TOPLEVEL_ROLE))
}

pub fn is_valid_parent(child: &WlSurface, parent: &WlSurface) -> bool {
    if !is_toplevel_equivalent(parent) {
        return false;
    }

    let mut next_parent = Some(parent.clone());
    while let Some(parent) = next_parent.clone() {
        if *child == parent {
            return false;
        }

        compositor::with_states(&parent, |states| {
            if let Some(data) = states.data_map.get::<XdgToplevelSurfaceData>() {
                let role = data.lock().unwrap();
                next_parent = role.parent.clone();
            } else {
                next_parent = None;
            }
        });
    }

    true
}
