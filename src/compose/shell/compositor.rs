// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::{
    element, layers::configure_layer,
    placement::{place_keeping_order, room_inside},
    ResizeState, SurfaceData, WindowElement,
};
use std::cell::RefCell;

use smithay::xwayland::XWaylandClientData;

use smithay::wayland::drm_syncobj::DrmSyncobjCachedState;

use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    desktop::{layer_map_for_output, PopupKind, PopupManager, Space, WindowSurfaceType},
    input::pointer::{CursorImageStatus, CursorImageSurfaceData},
    reexports::{
        calloop::Interest,
        wayland_protocols::xdg::{
            decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
            shell::server::xdg_toplevel,
        },
        wayland_server::{protocol::wl_surface::WlSurface, Client, Resource},
    },
    wayland::{
        compositor::{
            add_blocker, add_pre_commit_hook, get_parent, is_sync_subsurface, with_states,
            with_surface_tree_upward, BufferAssignment, CompositorClientState,
            CompositorHandler, CompositorState, SurfaceAttributes, TraversalAction,
        },
        dmabuf::get_dmabuf,
        shell::{wlr_layer::LayerSurfaceCachedState, xdg::XdgToplevelSurfaceData},
    },
};

use crate::{state::{AnvilState, Backend}, ClientState};

impl<BackendData: Backend> CompositorHandler for AnvilState<BackendData> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        if let Some(state) = client.get_data::<XWaylandClientData>() {
            return &state.compositor_state;
        }
        if let Some(state) = client.get_data::<ClientState>() {
            return &state.compositor_state;
        }
        panic!("Unknown client data type")
    }

    fn new_surface(&mut self, surface: &WlSurface) {

        add_pre_commit_hook::<Self, _>(surface, |_state, _dh, surface| give_layer_a_size(surface));
        add_pre_commit_hook::<Self, _>(surface, move |state, _dh, surface| wait_for_dmabuf(state, surface));
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
        self.backend_data.early_import(surface);
        self.modal_committed(surface);
        self.ring_panel_committed(surface);
        self.window_committed(surface);
        self.popups.commit(surface);
        self.cursor_committed(surface);
        self.drag_icon_committed(surface);

        self.keyboard_to_new_layer(surface);
        ensure_initial_configure(surface, &self.space, &mut self.popups);
        self.refit_login_screen();
        self.fit_windows_to_room();
        self.refresh_pointer_focus();
    }
}

fn give_layer_a_size(surface: &WlSurface) {
    with_states(surface, |states| {
        if states.role != Some(smithay::wayland::shell::wlr_layer::LAYER_SURFACE_ROLE) {
            return;
        }
        let mut cached = states.cached_state.get::<LayerSurfaceCachedState>();
        let pending = cached.pending();
        if pending.size.w == 0 && !pending.anchor.anchored_horizontally() {
            pending.size.w = 1;
        }
        if pending.size.h == 0 && !pending.anchor.anchored_vertically() {
            pending.size.h = 1;
        }
    });
}

fn wait_for_dmabuf<BackendData: Backend>(state: &mut AnvilState<BackendData>, surface: &WlSurface) {
    let mut acquire_point = None;
    let maybe_dmabuf = with_states(surface, |surface_data| {
        acquire_point.clone_from(
            &surface_data
                .cached_state
                .get::<DrmSyncobjCachedState>()
                .pending()
                .acquire_point,
        );
        surface_data
            .cached_state
            .get::<SurfaceAttributes>()
            .pending()
            .buffer
            .as_ref()
            .and_then(|assignment| match assignment {
                BufferAssignment::NewBuffer(buffer) => get_dmabuf(buffer).cloned().ok(),
                _ => None,
            })
    });
    if let Some(dmabuf) = maybe_dmabuf {
        if let Some(acquire_point) = acquire_point {
            if let Ok((blocker, source)) = acquire_point.generate_blocker() {
                let client = surface.client().unwrap();
                let res = state.handle.insert_source(source, move |_, _, data| {
                    let dh = data.display_handle.clone();
                    data.client_compositor_state(&client).blocker_cleared(data, &dh);
                    Ok(())
                });
                if res.is_ok() {
                    add_blocker(surface, blocker);
                    return;
                }
            }
        }
        if let Ok((blocker, source)) = dmabuf.generate_blocker(Interest::READ) {
            if let Some(client) = surface.client() {
                let res = state.handle.insert_source(source, move |_, _, data| {
                    let dh = data.display_handle.clone();
                    data.client_compositor_state(&client).blocker_cleared(data, &dh);
                    Ok(())
                });
                if res.is_ok() {
                    add_blocker(surface, blocker);
                }
            }
        }
    }
}

impl<BackendData: Backend> AnvilState<BackendData> {

    fn ring_panel_committed(&mut self, surface: &WlSurface) {
        if crate::trusted::is_ours(surface) {
            let shows = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| s.buffer().is_some()).unwrap_or(false);
            let changed = crate::trusted::ours_committed(surface, shows, self.ring.wished, std::time::Instant::now());

            if changed && self.panel_of(surface).is_some_and(|p| crate::trusted::is_ring_panel(&p)) {
                self.ring.wished = None;
                self.ring.expecting = None;
            }
        }
    }

    fn window_committed(&mut self, surface: &WlSurface) {
        if is_sync_subsurface(surface) {
            return;
        }
        let mut root = surface.clone();
        while let Some(parent) = get_parent(&root) {
            root = parent;
        }
        if let Some(window) = self.window_for_surface(&root) {
            window.0.on_commit();

            if &root == surface && window.is_wayland() {
                self.place_when_ready(&window);
            }

            if &root == surface {
                let buffer_offset = with_states(surface, |states| {
                    states
                        .cached_state
                        .get::<SurfaceAttributes>()
                        .current()
                        .buffer_delta
                        .take()
                });

                if let Some(buffer_offset) = buffer_offset {
                    let current_loc = self.space.element_location(&window).unwrap();
                    place_keeping_order(&mut self.space, window, current_loc + buffer_offset);
                }
            }
        }
    }

    fn cursor_committed(&mut self, surface: &WlSurface) {
        if matches!(&self.cursor_status, CursorImageStatus::Surface(cursor_surface) if cursor_surface == surface)
        {
            with_states(surface, |states| {
                let cursor_image_attributes = states.data_map.get::<CursorImageSurfaceData>();

                if let Some(mut cursor_image_attributes) =
                    cursor_image_attributes.map(|attrs| attrs.lock().unwrap())
                {
                    let buffer_delta = states
                        .cached_state
                        .get::<SurfaceAttributes>()
                        .current()
                        .buffer_delta
                        .take();
                    if let Some(buffer_delta) = buffer_delta {
                        tracing::trace!(hotspot = ?cursor_image_attributes.hotspot, ?buffer_delta, "decrementing cursor hotspot");
                        cursor_image_attributes.hotspot -= buffer_delta;
                    }
                }
            });
        }
    }

    fn drag_icon_committed(&mut self, surface: &WlSurface) {
        if matches!(&self.dnd_icon, Some(icon) if &icon.surface == surface) {
            let dnd_icon = self.dnd_icon.as_mut().unwrap();
            with_states(&dnd_icon.surface, |states| {
                let buffer_delta = states
                    .cached_state
                    .get::<SurfaceAttributes>()
                    .current()
                    .buffer_delta
                    .take()
                    .unwrap_or_default();
                tracing::trace!(offset = ?dnd_icon.offset, ?buffer_delta, "moving dnd offset");
                dnd_icon.offset += buffer_delta;
            });
        }
    }
}

pub(super) fn ensure_initial_configure(surface: &WlSurface, space: &Space<WindowElement>, popups: &mut PopupManager) {
    with_surface_tree_upward(
        surface,
        (),
        |_, _, _| TraversalAction::DoChildren(()),
        |_, states, _| {
            states
                .data_map
                .insert_if_missing(|| RefCell::new(SurfaceData::default()));
        },
        |_, _, _| true,
    );

    if let Some(window) = space
        .elements()
        .find(|window| window.wl_surface().map(|s| &*s == surface).unwrap_or(false))
        .cloned()
    {
        configure_window(surface, &window);
        return;
    }

    if let Some(popup) = popups.find_popup(surface) {
        let popup = match popup {
            PopupKind::Xdg(ref popup) => popup,

            PopupKind::InputMethod(ref _input_popup) => {
                return;
            }
        };

        if !popup.is_initial_configure_sent() {

            popup.send_configure().expect("initial configure failed");
        }

        return;
    };

    if let Some(output) = space.outputs().find(|o| {
        let map = layer_map_for_output(o);
        map.layer_for_surface(surface, WindowSurfaceType::TOPLEVEL)
            .is_some()
    }) {
        configure_layer(surface, output);
    };
}

fn configure_window(surface: &WlSurface, window: &WindowElement) {

    if let Some(toplevel) = window.0.toplevel() {
        let initial_configure_sent = with_states(surface, |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        });
        if !initial_configure_sent {

            toplevel.with_pending_state(|state| {
                let ssd = state.decoration_mode == Some(DecorationMode::ServerSide)
                    && !state.states.contains(xdg_toplevel::State::Fullscreen);
                if let Some(room) = state.bounds {
                    state.bounds = Some(room_inside(room, element::frame_extra(ssd)));
                }
            });
            toplevel.send_configure();
        }
    }

    with_states(surface, |states| {
        let mut data = states
            .data_map
            .get::<RefCell<SurfaceData>>()
            .unwrap()
            .borrow_mut();

        if let ResizeState::WaitingForCommit(_) = data.resize_state {
            data.resize_state = ResizeState::NotResizing;
        }
    });
}
