// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    input::touch::{GrabStartData as TouchGrabStartData, TouchGrab},
    utils::{IsAlive, Logical, Point, Serial, Size},
};

use super::resize::{ResizeEdge, Resizing};
use crate::shell::WindowElement;
use crate::state::{AnvilState, Backend};

pub struct TouchResizeSurfaceGrab<BackendData: Backend + 'static> {
    pub start_data: TouchGrabStartData<AnvilState<BackendData>>,
    pub window: WindowElement,
    pub edges: ResizeEdge,
    pub initial_window_location: Point<i32, Logical>,
    pub initial_window_size: Size<i32, Logical>,
    pub last_window_size: Size<i32, Logical>,
}

impl<BackendData: Backend> TouchResizeSurfaceGrab<BackendData> {
    fn resizing(&self) -> Resizing<'_> {
        Resizing {
            window: &self.window,
            edges: self.edges,
            initial_window_location: self.initial_window_location,
            initial_window_size: self.initial_window_size,
            last_window_size: self.last_window_size,
        }
    }
}

impl<BackendData: Backend> TouchGrab<AnvilState<BackendData>> for TouchResizeSurfaceGrab<BackendData> {
    fn down(
        &mut self,
        _data: &mut AnvilState<BackendData>,
        _handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        _focus: Option<(
            <AnvilState<BackendData> as smithay::input::SeatHandler>::TouchFocus,
            Point<f64, Logical>,
        )>,
        _event: &smithay::input::touch::DownEvent,
        _seq: Serial,
    ) {
    }

    fn up(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        event: &smithay::input::touch::UpEvent,
        _seq: Serial,
    ) {
        if event.slot != self.start_data.slot {
            return;
        }
        handle.unset_grab(self, data);

        if !self.window.alive() {
            return;
        }

        self.resizing().finish(data, event.serial);
    }

    fn motion(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        _focus: Option<(
            <AnvilState<BackendData> as smithay::input::SeatHandler>::TouchFocus,
            Point<f64, Logical>,
        )>,
        event: &smithay::input::touch::MotionEvent,
        _seq: Serial,
    ) {
        if event.slot != self.start_data.slot {
            return;
        }

        if !self.window.alive() {
            handle.unset_grab(self, data);
            return;
        }

        let size = self.resizing().size_for(event.location - self.start_data.location);
        self.last_window_size = size;
        self.resizing().ask(data, size);
    }

    fn frame(
        &mut self,
        _data: &mut AnvilState<BackendData>,
        _handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        _seq: Serial,
    ) {
    }

    fn cancel(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        seq: Serial,
    ) {
        handle.cancel(data, seq);
        handle.unset_grab(self, data);
    }

    fn shape(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        event: &smithay::input::touch::ShapeEvent,
        seq: Serial,
    ) {
        handle.shape(data, event, seq);
    }

    fn orientation(
        &mut self,
        data: &mut AnvilState<BackendData>,
        handle: &mut smithay::input::touch::TouchInnerHandle<'_, AnvilState<BackendData>>,
        event: &smithay::input::touch::OrientationEvent,
        seq: Serial,
    ) {
        handle.orientation(data, event, seq);
    }

    fn start_data(&self) -> &smithay::input::touch::GrabStartData<AnvilState<BackendData>> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut AnvilState<BackendData>) {}
}
