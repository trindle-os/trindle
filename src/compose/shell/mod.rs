// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::cell::RefCell;

use tracing::trace;

use smithay::{
    desktop::Space, output::Output,
    reexports::wayland_server::protocol::{
        wl_buffer::WlBuffer, wl_output, wl_surface::WlSurface,
    },
    utils::{IsAlive, Logical, Rectangle},
    wayland::buffer::BufferHandler,
};

use crate::state::{AnvilState, Backend};

mod compositor;
mod element;
mod grabs;
mod layers;
#[cfg(test)]
mod layer_tests;
mod outline;
mod picture;
mod placement;
mod shadow;
pub(crate) mod ssd;
mod x11;
mod xdg;

pub use self::element::*;
pub use self::grabs::*;
pub use self::layers::layer_taking_input;
pub use self::placement::{dialogs_of, fixup_positions, place_keeping_order};

fn fullscreen_output_geometry(
    wl_surface: &WlSurface,
    wl_output: Option<&wl_output::WlOutput>,
    space: &mut Space<WindowElement>,
) -> Option<(Output, Rectangle<i32, Logical>)> {
    let window = space
        .elements()
        .find(|window| window.wl_surface().map(|s| &*s == wl_surface).unwrap_or(false));
    let output = wl_output
        .and_then(Output::from_resource)
        .or_else(|| window.and_then(|w| space.outputs_for_element(w).first().cloned()))
        .or_else(|| {
            let at = space.element_location(window?)?.to_f64();
            space.output_under(at).next().cloned()
        })
        .or_else(|| space.outputs().next().cloned())?;
    let geometry = space.output_geometry(&output)?;
    Some((output, geometry))
}

#[derive(Default)]
pub struct FullscreenSurface(RefCell<Option<WindowElement>>);

impl FullscreenSurface {
    pub fn set(&self, window: WindowElement) {
        *self.0.borrow_mut() = Some(window);
    }

    pub fn get(&self) -> Option<WindowElement> {
        let mut window = self.0.borrow_mut();
        if window.as_ref().map(|w| !w.alive()).unwrap_or(false) {
            *window = None;
        }
        window.clone()
    }

    pub fn clear(&self) -> Option<WindowElement> {
        self.0.borrow_mut().take()
    }

    pub fn clear_if_held_by(&self, window: &WindowElement) -> bool {
        take_if_same(&self.0, window)
    }
}

fn take_if_same<T: PartialEq>(slot: &RefCell<Option<T>>, held: &T) -> bool {
    let mut slot = slot.borrow_mut();
    let same = slot.as_ref() == Some(held);
    if same {
        *slot = None;
    }
    same
}

#[cfg(test)]
mod slot_tests {
    use super::take_if_same;
    use std::cell::RefCell;

    #[test]
    fn leaving_full_screen_empties_only_the_slot_the_window_holds() {
        let slot = RefCell::new(Some(1));
        assert!(!take_if_same(&slot, &2));
        assert_eq!(*slot.borrow(), Some(1));
        assert!(take_if_same(&slot, &1));
        assert_eq!(*slot.borrow(), None);
    }
}

impl<BackendData: Backend> BufferHandler for AnvilState<BackendData> {
    fn buffer_destroyed(&mut self, _buffer: &WlBuffer) {}
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn window_for_surface(&self, surface: &WlSurface) -> Option<WindowElement> {
        self.space
            .elements()
            .find(|window| window.wl_surface().map(|s| &*s == surface).unwrap_or(false))
            .cloned()
    }

    pub fn leave_fullscreen_slots(&mut self, window: &WindowElement, keep: Option<&Output>) {
        let outputs: Vec<Output> = self.space.outputs().filter(|o| Some(*o) != keep).cloned().collect();
        for output in outputs {
            if output.user_data().get::<FullscreenSurface>().is_some_and(|f| f.clear_if_held_by(window)) {
                trace!("Unfullscreening: {:?}", window);
                self.backend_data.reset_buffers(&output);
            }
        }
    }
}

#[derive(Default)]
pub struct SurfaceData {
    pub resize_state: ResizeState,
}
