use std::sync::{Arc, Mutex};

use wayland_protocols_misc::zwp_input_method_v2::server::zwp_input_popup_surface_v2::{
    self, ZwpInputPopupSurfaceV2,
};
use wayland_server::{backend::ClientId, protocol::wl_surface::WlSurface, Dispatch, Resource};

use crate::utils::{
    alive_tracker::{AliveTracker, IsAlive},
    Logical, Point, Rectangle,
};

use super::InputMethodManagerState;

#[derive(Debug, Clone, Default)]
pub struct PopupHandle {
    pub surface: Option<PopupSurface>,
    pub rectangle: Rectangle<i32, Logical>,
}

#[derive(Debug, Clone)]
pub struct PopupSurface {
    pub surface_role: ZwpInputPopupSurfaceV2,
    surface: WlSurface,
    pub(crate) rectangle: Arc<Mutex<Rectangle<i32, Logical>>>,
    location: Arc<Mutex<Point<i32, Logical>>>,
    parent: Option<PopupParent>,
}

impl PopupSurface {
    pub(crate) fn new(
        surface_role: ZwpInputPopupSurfaceV2,
        surface: WlSurface,
        rectangle: Arc<Mutex<Rectangle<i32, Logical>>>,
        parent: Option<PopupParent>,
    ) -> Self {
        let location = Arc::new(Mutex::new(rectangle.lock().unwrap().loc));
        Self {
            surface_role,
            rectangle,
            location,
            surface,
            parent,
        }
    }

    #[inline]
    pub fn alive(&self) -> bool {
        let role_data: &InputMethodPopupSurfaceUserData = self.surface_role.data().unwrap();
        self.surface.alive() && role_data.alive_tracker.alive()
    }

    #[inline]
    pub fn wl_surface(&self) -> &WlSurface {
        &self.surface
    }

    pub fn get_parent(&self) -> Option<&PopupParent> {
        self.parent.as_ref()
    }

    pub fn set_parent(&mut self, parent: Option<PopupParent>) {
        self.parent = parent;
    }

    pub fn location(&self) -> Point<i32, Logical> {
        *self.location.lock().unwrap()
    }

    pub fn set_location(&self, location: Point<i32, Logical>) {
        *self.location.lock().unwrap() = location;
    }

    pub fn text_input_rectangle(&self) -> Rectangle<i32, Logical> {
        *self.rectangle.lock().unwrap()
    }

    pub fn set_text_input_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32) {
        *self.rectangle.lock().unwrap() = Rectangle::new((x, y).into(), (width, height).into());
        *self.location.lock().unwrap() = (x, y).into();
        self.surface_role.text_input_rectangle(x, y, width, height);
    }
}

impl std::cmp::PartialEq for PopupSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.surface_role == other.surface_role
    }
}

#[derive(Debug, Clone)]
pub struct PopupParent {
    pub surface: WlSurface,
    pub location: Rectangle<i32, Logical>,
}

#[derive(Debug)]
pub struct InputMethodPopupSurfaceUserData {
    pub(super) alive_tracker: AliveTracker,
}

impl<D> Dispatch<ZwpInputPopupSurfaceV2, InputMethodPopupSurfaceUserData, D> for InputMethodManagerState {
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &ZwpInputPopupSurfaceV2,
        request: zwp_input_popup_surface_v2::Request,
        _data: &InputMethodPopupSurfaceUserData,
        _dhandle: &wayland_server::DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            zwp_input_popup_surface_v2::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }

    fn destroyed(
        _state: &mut D,
        _client: ClientId,
        _object: &ZwpInputPopupSurfaceV2,
        data: &InputMethodPopupSurfaceUserData,
    ) {
        data.alive_tracker.destroy_notify();
    }
}
