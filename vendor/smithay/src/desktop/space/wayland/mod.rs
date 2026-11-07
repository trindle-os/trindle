use std::{cell::RefCell, collections::HashMap};

use tracing::instrument;
use wayland_server::protocol::wl_surface::WlSurface;

use crate::{
    backend::renderer::utils::RendererSurfaceStateUserData,
    output::{Output, WeakOutput},
    utils::{Logical, Point, Rectangle},
    wayland::compositor::{with_surface_tree_downward, TraversalAction},
};

mod layer;
mod window;
#[cfg(feature = "xwayland")]
mod x11;

#[instrument(level = "trace", skip(output), fields(output = output.name()))]
#[profiling::function]
pub fn output_update(output: &Output, output_overlap: Option<Rectangle<i32, Logical>>, surface: &WlSurface) {
    with_surface_tree_downward(
        surface,
        (Point::from((0, 0)), false),
        |_, states, (location, parent_unmapped)| {
            let mut location = *location;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if *parent_unmapped {
                TraversalAction::DoChildren((location, true))
            } else if let Some(surface_view) = data.and_then(|d| d.lock().unwrap().surface_view) {
                location += surface_view.offset;
                TraversalAction::DoChildren((location, false))
            } else {
                TraversalAction::DoChildren((location, true))
            }
        },
        |wl_surface, states, (location, parent_unmapped)| {
            let mut location = *location;

            if *parent_unmapped {
                output.leave(wl_surface);
                return;
            }

            let Some(output_overlap) = output_overlap else {
                output.leave(wl_surface);
                return;
            };

            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(surface_view) = data.and_then(|d| d.lock().unwrap().surface_view) {
                location += surface_view.offset;
                let surface_rectangle = Rectangle::new(location, surface_view.dst);
                if output_overlap.overlaps(surface_rectangle) {
                    output.enter(wl_surface);
                } else {
                    output.leave(wl_surface);
                }
            } else {
                output.leave(wl_surface);
            }
        },
        |_, _, _| true,
    );
}

#[derive(Debug, Default)]
struct WindowOutputState {
    output_overlap: HashMap<WeakOutput, Rectangle<i32, Logical>>,
}
type WindowOutputUserData = RefCell<WindowOutputState>;
