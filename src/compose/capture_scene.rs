// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Filling the screenshot and screen-sharing copies programs asked for from the frame just drawn, on any backend.

use std::time::Duration;

use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        renderer::{
            gles::GlesTexture,
            Bind, ExportMem, ImportAll, ImportMem, Offscreen, Renderer, Texture,
        },
    },
    desktop::space::Space,
    input::pointer::CursorImageStatus,
    output::Output,
    reexports::calloop::LoopHandle,
    utils::{Logical, Point, Rectangle, Scale},
};

use crate::{
    drawing::PointerElement,
    lock::SessionLock,
    protocols::image_capture::{CaptureTarget, ImageCaptureState},
    render::{output_elements, pointer_elements},
    shell::WindowElement,
    state::DndIcon,
};

pub type Windows = [(String, WindowElement, Rectangle<i32, Logical>, f64)];

pub struct Scene<'a> {
    pub space: &'a Space<WindowElement>,
    pub pointer_element: &'a PointerElement,
    pub cursor_status: &'a CursorImageStatus,
    pub dnd_icon: Option<&'a DndIcon>,
    pub session_lock: &'a SessionLock,
    pub pointer_location: Point<f64, Logical>,
    pub windows: &'a Windows,
    pub now: Duration,
}

impl Scene<'_> {
    pub fn fill_output<R, T>(
        &self,
        state: &mut ImageCaptureState,
        renderer: &mut R,
        output: &Output,
        handle: &LoopHandle<'static, T>,
    ) where
        R: Renderer + ImportAll + ImportMem + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + Clone + Send + 'static,
        T: 'static,
    {
        let Some(output_geometry) = self.space.output_geometry(output) else { return };
        let scale = Scale::from(output.current_scale().fractional_scale());
        state.service_output(
            renderer,
            output,
            |renderer, with_pointer| {
                let pointer = if with_pointer {
                    pointer_elements(renderer, output_geometry, scale, self.pointer_location,
                                     self.pointer_element, self.cursor_status, self.dnd_icon)
                } else {
                    Vec::new()
                };
                output_elements(output, self.space, pointer, renderer, self.session_lock)
            },
            self.now,
            handle,
        );
    }

    pub fn fill_toplevels<R, T>(
        &self,
        state: &mut ImageCaptureState,
        renderer: &mut R,
        handle: &LoopHandle<'static, T>,
    ) where
        R: Renderer + ImportAll + ImportMem + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + Clone + Send + 'static,
        T: 'static,
    {
        state.service_toplevels(
            renderer,
            |renderer, handle, with_pointer| {
                let id = handle.identifier();
                let (_, window, area, scale) = self.windows.iter().find(|(i, ..)| *i == id)?;
                if self.session_lock.is_active() {
                    return None;
                }
                let pointer = if with_pointer {
                    pointer_elements(renderer, *area, Scale::from(*scale), self.pointer_location,
                                     self.pointer_element, self.cursor_status, self.dnd_icon)
                } else {
                    Vec::new()
                };
                let size = crate::panel::capture_size(window, *scale);
                Some((crate::render::window_elements(renderer, window, *scale, pointer), size, *scale))
            },
            self.now,
            handle,
        );
    }

    pub fn fill_pointers<R, T>(
        &self,
        state: &mut ImageCaptureState,
        renderer: &mut R,
        handle: &LoopHandle<'static, T>,
    ) where
        R: Renderer + ImportAll + ImportMem + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + Clone + Send + 'static,
        T: 'static,
    {
        state.service_pointers(
            renderer,
            |renderer, target| {
                let (area, scale) = match target {
                    CaptureTarget::Output(o) => (self.space.output_geometry(o)?, o.current_scale().fractional_scale()),
                    CaptureTarget::Toplevel(h) => {
                        let id = h.identifier();
                        let (_, _, area, scale) = self.windows.iter().find(|(i, ..)| *i == id)?;
                        (*area, *scale)
                    }
                };
                Some(crate::render::pointer_view(renderer, area, scale, self.pointer_location, self.pointer_element, self.cursor_status))
            },
            self.now,
            handle,
        );
    }
}
