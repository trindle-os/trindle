// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The compositor's state: the one value every handler works on, and its clients.
//!
//! Origin: began as a file of anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld). About 53% of this file's lines still read as
//! they did there; the rest is Trindle's own.

mod clipboard;
mod init;
mod protocols;
mod repaint;
mod seat;

pub use repaint::{take_presentation_feedback, update_primary_scanout_output, SurfaceDmabufFeedback};

use std::sync::{atomic::AtomicBool, Arc};

use smithay::{
    delegate_compositor,
    desktop::{PopupManager, Space},
    input::{
        keyboard::{Keysym, LedState},
        pointer::{CursorImageStatus, PointerHandle},
        Seat, SeatState,
    },
    output::Output,
    reexports::{
        calloop::LoopHandle,
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface, DisplayHandle,
        },
    },
    utils::{Clock, Logical, Monotonic, Point},
    wayland::{
        commit_timing::CommitTimingManagerState,
        compositor::{CompositorClientState, CompositorState},
        fifo::FifoManagerState, fractional_scale::FractionalScaleManagerState,
        idle_inhibit::IdleInhibitManagerState, idle_notify::IdleNotifierState,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState, output::OutputManagerState,
        presentation::PresentationState, security_context::SecurityContext,
        session_lock::SessionLockManagerState,
        selection::{
            data_device::DataDeviceState, primary_selection::PrimarySelectionState,
            wlr_data_control::DataControlState,
        },
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{decoration::XdgDecorationState, XdgShellState},
        },
        shm::ShmState, single_pixel_buffer::SinglePixelBufferState, viewporter::ViewporterState,
        xdg_activation::XdgActivationState, xdg_foreign::XdgForeignState,
    },
};

use crate::{
    lock::SessionLock, panel::PutAway,
    protocols::{windows::WindowsState, workspaces::WorkspacesState},
    protocols::image_capture::ImageCaptureState, shell::WindowElement,
};
use crate::protocols::{
    night::NightState,
    screens::{ScreenFacts, ScreenPlan, ScreensState},
    output_power::OutputPowerState,
};
use smithay::{wayland::selection::SelectionTarget, wayland::xwayland_shell};

#[derive(Debug, Default)]

pub struct ClientState {
    pub compositor_state: CompositorClientState,
    pub security_context: Option<SecurityContext>,
    pub from_root: bool,
    pub ours: bool,
    pub asked: bool,
    pub left: Option<(smithay::reexports::calloop::channel::Sender<crate::own_ring::Told>, u64)>,
}
impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        if let Some((keeper, number)) = &self.left {
            let _ = keeper.send(crate::own_ring::Told::LetGo(*number));
        }
    }
}

#[derive(Debug)]
pub struct AnvilState<BackendData: Backend + 'static> {
    pub backend_data: BackendData,
    pub socket_name: Option<String>,
    pub display_handle: DisplayHandle,
    pub running: Arc<AtomicBool>,
    pub handle: LoopHandle<'static, AnvilState<BackendData>>,

    pub space: Space<WindowElement>,
    pub popups: PopupManager,

    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub layer_shell_state: WlrLayerShellState,
    pub output_manager_state: OutputManagerState,
    pub primary_selection_state: PrimarySelectionState,
    pub data_control_state: DataControlState,
    pub seat_state: SeatState<AnvilState<BackendData>>,
    pub keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    pub shm_state: ShmState,
    pub viewporter_state: ViewporterState,
    pub xdg_activation_state: XdgActivationState,
    pub xdg_decoration_state: XdgDecorationState,
    pub xdg_shell_state: XdgShellState,
    pub presentation_state: PresentationState,
    pub fractional_scale_manager_state: FractionalScaleManagerState,
    pub xdg_foreign_state: XdgForeignState,
    pub xwayland_shell_state: xwayland_shell::XWaylandShellState,
    pub single_pixel_buffer_state: SinglePixelBufferState,
    pub fifo_manager_state: FifoManagerState,
    pub commit_timing_manager_state: CommitTimingManagerState,
    pub windows_state: WindowsState,
    pub workspaces_state: WorkspacesState,

    pub minimized: Vec<PutAway>,
    pub hidden: Vec<PutAway>,
    pub close_asked: Vec<(WindowElement, std::time::Instant)>,
    pub fresh: std::collections::HashMap<(Option<smithay::xwayland::xwm::XwmId>, u32), std::time::Instant>,
    pub fresh_for: std::time::Duration,
    pub workspaces: usize,
    pub active_workspace: usize,
    pub image_capture_state: ImageCaptureState,
    pub toplevel_list_state: smithay::wayland::foreign_toplevel_list::ForeignToplevelListState,
    pub toplevel_list: Vec<(WlSurface, smithay::wayland::foreign_toplevel_list::ForeignToplevelHandle)>,
    pub screens_state: ScreensState,
    pub output_power_state: OutputPowerState,
    pub night_state: NightState,
    pub session_lock_state: SessionLockManagerState,
    pub session_lock: SessionLock,
    pub plugged: crate::lock::Plugged,
    pub idle_notifier_state: IdleNotifierState<AnvilState<BackendData>>,
    pub idle_inhibit_state: IdleInhibitManagerState,
    pub idle_inhibitors: Vec<WlSurface>,
    pub pads: crate::game_pads::Pads,

    pub dnd_icon: Option<DndIcon>,
    pub dragging: Option<crate::dragging::Dragging>,
    pub xdnd: crate::xdnd::Xdnd,

    pub suppressed_keys: Vec<Keysym>,
    pub keyboard_layers: Vec<WlSurface>,
    pub keys_to_panel: Option<String>,
    pub modal: Option<crate::modal::Held>,
    pub modal_limit: std::time::Duration,
    pub keys_to_nothing: bool,
    pub expander: crate::expander::Expander,
    pub reader_keys: crate::reader_keys::ReaderKeys,
    pub input_capture_state: crate::protocols::input_capture::InputCaptureState,
    pub own_press_state: crate::protocols::own_press::OwnPressState,
    pub capture: crate::capture::Capture,
    pub scroll_scaler: crate::scroll::Scaler,
    pub keys_changed: crate::keys_changed::KeysChanged,
    pub xkb_names: (String, String, String, Option<String>),
    pub cursor_status: CursorImageStatus,
    pub seat_name: String,
    pub seat: Seat<AnvilState<BackendData>>,
    pub clock: Clock<Monotonic>,
    pub pointer: PointerHandle<AnvilState<BackendData>>,

    pub rooms: crate::xrooms::Rooms,
    pub xdisplay: Option<u32>,
    pub wayland_selection: [Option<Vec<String>>; 2],

    pub ring: crate::own_ring::Kept,

    pub desktop_tap: Option<(u32, Point<f64, Logical>)>,

    pub config: crate::config::Config,
    pub release_key: Option<Keysym>,
    pub released_bind: Option<crate::config::Action>,
    pub startup: Option<String>,
    pub edge_cursor: bool,
}

#[derive(Debug)]
pub struct DndIcon {
    pub surface: WlSurface,
    pub offset: Point<i32, Logical>,
}

delegate_compositor!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

pub fn selection_slot(ty: SelectionTarget) -> usize {
    match ty {
        SelectionTarget::Clipboard => 0,
        SelectionTarget::Primary => 1,
    }
}

pub trait Backend {
    const HAS_RELATIVE_MOTION: bool = false;
    const HAS_GESTURES: bool = false;
    fn seat_name(&self) -> String;
    fn reset_buffers(&mut self, output: &Output);
    fn early_import(&mut self, surface: &WlSurface);
    fn update_led_state(&mut self, led_state: LedState);

    fn output_heads(state: &AnvilState<Self>) -> Vec<ScreenFacts>
    where
        Self: Sized + 'static;
    fn apply_output_config(
        state: &mut AnvilState<Self>,
        targets: &[ScreenPlan],
        test_only: bool,
    ) -> Result<(), String>
    where
        Self: Sized + 'static;
    fn output_power(&self, _output: &Output) -> Option<bool> {
        None
    }
    fn set_output_power(_state: &mut AnvilState<Self>, _output: &Output, _on: bool) -> bool
    where
        Self: Sized + 'static,
    {
        false
    }
    fn gamma_size(&self, _output: &Output) -> Option<u32> {
        None
    }
    fn set_gamma(&mut self, _output: &Output, _ramp: Option<Vec<u16>>) -> bool {
        false
    }
    fn apply_input_settings(&mut self, _pointer: &crate::config::Pointer) {}
    fn reload_pointer(&mut self) {}
    fn unlocked(&mut self) {}
}

fn num_lock_file() -> std::path::PathBuf {
    crate::files::home().join(".local/state/trindle/num-lock")
}

fn remembered_num_lock() -> bool {
    std::fs::read_to_string(num_lock_file()).is_ok_and(|t| t.trim() == "on")
}

fn remember_num_lock(on: bool) {
    if remembered_num_lock() == on {
        return;
    }
    let file = num_lock_file();
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = crate::files::write_text(&file, if on { "on\n" } else { "off\n" });
}

#[cfg(test)]
pub(crate) fn code() -> &'static str {
    let mut files: Vec<_> = std::fs::read_dir(crate::source::root().join("src/compose/state"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    files.sort();
    let text: String = files.iter().map(|p| std::fs::read_to_string(p).unwrap()).collect();
    crate::source::code(Box::leak(text.into_boxed_str()))
}
