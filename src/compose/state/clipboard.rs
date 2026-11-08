// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::{selection_slot, AnvilState, Backend, DndIcon};

use std::os::unix::io::OwnedFd;

use tracing::warn;

use smithay::{
    delegate_data_control, delegate_data_device, delegate_primary_selection, delegate_shm,
    input::{pointer::{CursorImageStatus, CursorImageSurfaceData}, Seat},
    reexports::wayland_server::protocol::{
        wl_data_device_manager::DndAction, wl_data_source::WlDataSource, wl_surface::WlSurface,
    },
    utils::{Logical, Point},
    wayland::{
        compositor::with_states,
        selection::{
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
            },
            primary_selection::{PrimarySelectionHandler, PrimarySelectionState},
            wlr_data_control::{DataControlHandler, DataControlState},
            SelectionHandler,
        },
        shm::{ShmHandler, ShmState},
    },
};

use smithay::wayland::selection::{SelectionSource, SelectionTarget};

impl<BackendData: Backend> DataDeviceHandler for AnvilState<BackendData> {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }

    fn action_choice(&mut self, available: DndAction, preferred: DndAction) -> DndAction {
        let chosen = smithay::wayland::selection::data_device::default_action_chooser(available, preferred);
        self.drag_chose(chosen);
        chosen
    }

    fn foreign_target(&mut self, surface: &WlSurface) -> bool {
        self.xdnd_foreign_target(surface)
    }

    fn foreign_enter(&mut self, surface: &WlSurface, location: Point<f64, Logical>) {
        self.xdnd_foreign_enter(surface, location)
    }

    fn foreign_motion(&mut self, location: Point<f64, Logical>, _time: u32) {
        self.xdnd_foreign_motion(location)
    }

    fn foreign_leave(&mut self) {
        self.xdnd_foreign_leave()
    }

    fn foreign_drop(&mut self) -> bool {
        self.xdnd_foreign_drop()
    }
}

impl<BackendData: Backend> ClientDndGrabHandler for AnvilState<BackendData> {
    fn started(&mut self, source: Option<WlDataSource>, icon: Option<WlSurface>, _seat: Seat<Self>) {
        self.xdnd_wayland_drag(source.clone());
        self.drag_started(source);
        let offset = if let CursorImageStatus::Surface(ref surface) = self.cursor_status {
            with_states(surface, |states| {
                let hotspot = states
                    .data_map
                    .get::<CursorImageSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .hotspot;
                Point::from((-hotspot.x, -hotspot.y))
            })
        } else {
            (0, 0).into()
        };
        self.dnd_icon = icon.map(|surface| DndIcon { surface, offset });
    }
    fn dropped(&mut self, _target: Option<WlSurface>, _validated: bool, _seat: Seat<Self>) {
        self.dnd_icon = None;
        self.xdnd_wayland_drag(None);
        self.drag_ended();
    }
}

impl<BackendData: Backend> ServerDndGrabHandler for AnvilState<BackendData> {
    fn accept(&mut self, mime_type: Option<String>, _seat: Seat<Self>) {
        self.xdnd_server_accept(mime_type);
    }
    fn action(&mut self, action: DndAction, _seat: Seat<Self>) {
        self.xdnd_server_action(action);
    }
    fn send(&mut self, mime_type: String, fd: OwnedFd, _seat: Seat<Self>) {
        self.xdnd_server_send(mime_type, fd);
    }
    fn finished(&mut self, _seat: Seat<Self>) {
        self.xdnd_server_finished();
    }
    fn cancelled(&mut self, _seat: Seat<Self>) {
        self.xdnd_server_cancelled();
    }
    fn drop_allowed(&mut self, _seat: Seat<Self>) -> bool {
        self.xdnd_drop_allowed()
    }
    fn released(&mut self, _seat: Seat<Self>) {
        self.xdnd_released();
    }
}
delegate_data_device!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> SelectionHandler for AnvilState<BackendData> {
    type SelectionUserData = ();

    fn new_selection(&mut self, ty: SelectionTarget, source: Option<SelectionSource>, _seat: Seat<Self>) {
        let mime_types = source.map(|source| source.mime_types());
        self.wayland_selection[selection_slot(ty)] = mime_types.clone();
        self.rooms.selection_from[selection_slot(ty)] = None;
        for xwm in self.rooms.running() {
            if let Err(err) = xwm.new_selection(ty, mime_types.clone()) {
                warn!(?err, ?ty, "Failed to set Xwayland selection");
            }
        }
    }

    fn send_selection(
        &mut self,
        ty: SelectionTarget,
        mime_type: String,
        fd: OwnedFd,
        _seat: Seat<Self>,
        _user_data: &(),
    ) {

        let Some(from) = self.rooms.selection_from[selection_slot(ty)].as_ref().map(|(id, _)| *id) else { return };
        let handle = self.handle.clone();
        if let Some(xwm) = self.rooms.wm(from) {
            if let Err(err) = xwm.send_selection(ty, mime_type, fd, handle) {
                warn!(?err, "Failed to send primary (X11 -> Wayland)");
            }
        }
    }
}

impl<BackendData: Backend> PrimarySelectionHandler for AnvilState<BackendData> {
    fn primary_selection_state(&self) -> &PrimarySelectionState {
        &self.primary_selection_state
    }
}
delegate_primary_selection!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> DataControlHandler for AnvilState<BackendData> {
    fn data_control_state(&self) -> &DataControlState {
        &self.data_control_state
    }
}

delegate_data_control!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend> ShmHandler for AnvilState<BackendData> {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
delegate_shm!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
