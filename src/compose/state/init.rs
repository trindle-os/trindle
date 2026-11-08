// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::{remembered_num_lock, AnvilState, Backend, ClientState};

use std::sync::{atomic::AtomicBool, Arc};

use tracing::{info, warn};

use smithay::{
    desktop::{PopupManager, Space},
    input::{keyboard::XkbConfig, pointer::{CursorImageStatus, PointerHandle}, Seat, SeatState},
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction},
        wayland_server::{Client, Display, DisplayHandle},
    },
    utils::{Clock, Monotonic},
    wayland::{
        foreign_toplevel_list::ForeignToplevelListState,
        commit_timing::CommitTimingManagerState, compositor::CompositorState,
        fifo::FifoManagerState, fractional_scale::FractionalScaleManagerState,
        idle_inhibit::IdleInhibitManagerState, idle_notify::IdleNotifierState,
        input_method::InputMethodManagerState,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState, output::OutputManagerState,
        pointer_constraints::PointerConstraintsState, pointer_gestures::PointerGesturesState,
        presentation::PresentationState, relative_pointer::RelativePointerManagerState,
        security_context::SecurityContextState, session_lock::SessionLockManagerState,
        selection::{
            data_device::DataDeviceState, primary_selection::PrimarySelectionState,
            wlr_data_control::DataControlState,
        },
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{decoration::XdgDecorationState, XdgShellState},
        },
        shm::ShmState, single_pixel_buffer::SinglePixelBufferState,
        socket::ListeningSocketSource, cursor_shape::CursorShapeManagerState,
        tablet_manager::TabletManagerState, text_input::TextInputManagerState,
        viewporter::ViewporterState, virtual_keyboard::VirtualKeyboardManagerState,
        xdg_activation::XdgActivationState, xdg_foreign::XdgForeignState,
    },
};

use crate::{
    lock::SessionLock,
    protocols::{windows::WindowsState, workspaces::WorkspacesState},
    protocols::image_capture::ImageCaptureState,
    protocols::{input_capture::InputCaptureState, own_press::OwnPressState},
};
use crate::protocols::{
    virtual_pointer::VirtualPointerManagerState, night::NightState, screens::ScreensState,
    output_power::OutputPowerState,
};
use smithay::{wayland::xwayland_keyboard_grab::XWaylandKeyboardGrabState, wayland::xwayland_shell};

fn unsandboxed(client: &Client) -> bool {
    client
        .get_data::<ClientState>()
        .is_none_or(|client_state| client_state.security_context.is_none())
}

fn may_control(client: &Client) -> bool {
    unsandboxed(client) || crate::virtual_input::may_control(client)
}

fn may_capture(client: &Client) -> bool {
    unsandboxed(client) || crate::virtual_input::may_capture(client)
}

struct DesktopGlobals {
    compositor_state: CompositorState,
    data_device_state: DataDeviceState,
    layer_shell_state: WlrLayerShellState,
    output_manager_state: OutputManagerState,
    primary_selection_state: PrimarySelectionState,
    data_control_state: DataControlState,
    shm_state: ShmState,
    viewporter_state: ViewporterState,
    xdg_activation_state: XdgActivationState,
    xdg_decoration_state: XdgDecorationState,
    xdg_shell_state: XdgShellState,
    presentation_state: PresentationState,
    fractional_scale_manager_state: FractionalScaleManagerState,
    xdg_foreign_state: XdgForeignState,
    single_pixel_buffer_state: SinglePixelBufferState,
    fifo_manager_state: FifoManagerState,
    commit_timing_manager_state: CommitTimingManagerState,
    image_capture_state: ImageCaptureState,
    toplevel_list_state: ForeignToplevelListState,
}

struct InputGlobals {
    input_capture_state: InputCaptureState,
    own_press_state: OwnPressState,
}

struct ScreenGlobals<BackendData: Backend + 'static> {
    windows_state: WindowsState,
    workspaces_state: WorkspacesState,
    screens_state: ScreensState,
    output_power_state: OutputPowerState,
    night_state: NightState,
    session_lock_state: SessionLockManagerState,
    idle_notifier_state: IdleNotifierState<AnvilState<BackendData>>,
    idle_inhibit_state: IdleInhibitManagerState,
}

struct SeatParts<BackendData: Backend + 'static> {
    seat_name: String,
    seat: Seat<AnvilState<BackendData>>,
    pointer: PointerHandle<AnvilState<BackendData>>,
    config: crate::config::Config,
}

struct Parts<BackendData: Backend + 'static> {
    backend_data: BackendData,
    display_handle: DisplayHandle,
    socket_name: Option<String>,
    handle: LoopHandle<'static, AnvilState<BackendData>>,
    clock: Clock<Monotonic>,
    seat_state: SeatState<AnvilState<BackendData>>,
    keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    xwayland_shell_state: xwayland_shell::XWaylandShellState,
    desktop: DesktopGlobals,
    input: InputGlobals,
    screens: ScreenGlobals<BackendData>,
    seat: SeatParts<BackendData>,
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn init(
        display: Display<AnvilState<BackendData>>,
        handle: LoopHandle<'static, AnvilState<BackendData>>,
        backend_data: BackendData,
        listen_on_socket: bool,
    ) -> AnvilState<BackendData> {
        let dh = display.handle();

        dh.backend_handle().set_default_max_buffer_size(crate::limits::SLOW_CLIENT_ROOM);

        let clock = Clock::new();

        let socket_name = Self::listen_on_socket(&handle, listen_on_socket);
        Self::serve_display(&handle, display);

        let desktop = Self::desktop_globals(&dh, &clock);
        let input = Self::input_globals(&dh);
        let screens = Self::screen_globals(&dh, &handle);

        let mut seat_state = SeatState::new();
        let seat = Self::add_seat(&mut seat_state, &dh, backend_data.seat_name());

        let keyboard_shortcuts_inhibit_state = KeyboardShortcutsInhibitState::new::<Self>(&dh);

        let xwayland_shell_state = xwayland_shell::XWaylandShellState::new::<Self>(&dh.clone());

        XWaylandKeyboardGrabState::new::<Self>(&dh.clone());

        Self::assemble(Parts {
            backend_data,
            display_handle: dh,
            socket_name,
            handle,
            clock,
            seat_state,
            keyboard_shortcuts_inhibit_state,
            xwayland_shell_state,
            desktop,
            input,
            screens,
            seat,
        })
    }

    fn listen_on_socket(handle: &LoopHandle<'static, Self>, listen_on_socket: bool) -> Option<String> {
        if !listen_on_socket {
            return None;
        }
        let source = ListeningSocketSource::new_auto().unwrap();
        let socket_name = source.socket_name().to_string_lossy().into_owned();
        handle
            .insert_source(source, |client_stream, _, data| {
                let from_root = crate::trusted::from_root(&client_stream);
                if let Err(err) = data
                    .display_handle
                    .insert_client(client_stream, Arc::new(ClientState { from_root, ..ClientState::default() }))
                {
                    warn!("Error adding wayland client: {}", err);
                };
            })
            .expect("Failed to init wayland socket source");
        info!(name = socket_name, "Listening on wayland socket");
        Some(socket_name)
    }

    fn serve_display(handle: &LoopHandle<'static, Self>, display: Display<Self>) {
        handle
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |_, display, data| {
                    // Safety: we don't drop the display
                    unsafe {
                        display.get_mut().dispatch_clients(data).unwrap();
                    }
                    Ok(PostAction::Continue)
                },
            )
            .expect("Failed to init wayland server source");
    }

    fn desktop_globals(dh: &DisplayHandle, clock: &Clock<Monotonic>) -> DesktopGlobals {
        let compositor_state = CompositorState::new::<Self>(dh);
        let data_device_state = DataDeviceState::new::<Self>(dh);
        let layer_shell_state = WlrLayerShellState::new_with_filter::<Self, _>(dh, unsandboxed);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(dh);
        let primary_selection_state = PrimarySelectionState::new::<Self>(dh);
        let data_control_state =
            DataControlState::new::<Self, _>(dh, Some(&primary_selection_state), may_control);
        let shm_state = ShmState::new::<Self>(dh, vec![]);
        let viewporter_state = ViewporterState::new::<Self>(dh);
        let xdg_activation_state = XdgActivationState::new::<Self>(dh);
        let xdg_decoration_state = XdgDecorationState::new::<Self>(dh);
        let xdg_shell_state = XdgShellState::new::<Self>(dh);
        let presentation_state = PresentationState::new::<Self>(dh, clock.id() as u32);
        let fractional_scale_manager_state = FractionalScaleManagerState::new::<Self>(dh);
        let xdg_foreign_state = XdgForeignState::new::<Self>(dh);
        let single_pixel_buffer_state = SinglePixelBufferState::new::<Self>(dh);
        let fifo_manager_state = FifoManagerState::new::<Self>(dh);
        let commit_timing_manager_state = CommitTimingManagerState::new::<Self>(dh);
        let image_capture_state = ImageCaptureState::new::<Self, _>(dh, unsandboxed);
        let toplevel_list_state = ForeignToplevelListState::new_with_filter::<Self>(dh, unsandboxed);
        DesktopGlobals {
            compositor_state,
            data_device_state,
            layer_shell_state,
            output_manager_state,
            primary_selection_state,
            data_control_state,
            shm_state,
            viewporter_state,
            xdg_activation_state,
            xdg_decoration_state,
            xdg_shell_state,
            presentation_state,
            fractional_scale_manager_state,
            xdg_foreign_state,
            single_pixel_buffer_state,
            fifo_manager_state,
            commit_timing_manager_state,
            image_capture_state,
            toplevel_list_state,
        }
    }

    fn input_globals(dh: &DisplayHandle) -> InputGlobals {
        TextInputManagerState::new::<Self>(dh);
        InputMethodManagerState::new::<Self, _>(dh, unsandboxed);
        VirtualKeyboardManagerState::new::<Self, _>(dh, may_control);
        VirtualPointerManagerState::new::<Self, _>(dh, may_control);
        let input_capture_state = InputCaptureState::new::<Self, _>(dh, may_capture);

        let own_press_state = OwnPressState::new::<Self, _>(dh, unsandboxed);

        crate::protocols::unlock::UnlockState::new::<Self, _>(dh, crate::lock::is_flow_client);

        if BackendData::HAS_RELATIVE_MOTION {
            RelativePointerManagerState::new::<Self>(dh);
        }
        PointerConstraintsState::new::<Self>(dh);
        if BackendData::HAS_GESTURES {
            PointerGesturesState::new::<Self>(dh);
        }
        TabletManagerState::new::<Self>(dh);
        CursorShapeManagerState::new::<Self>(dh);
        SecurityContextState::new::<Self, _>(dh, unsandboxed);
        InputGlobals { input_capture_state, own_press_state }
    }

    fn screen_globals(dh: &DisplayHandle, handle: &LoopHandle<'static, Self>) -> ScreenGlobals<BackendData> {
        let windows_state = WindowsState::new::<Self, _>(dh, unsandboxed);
        let workspaces_state = WorkspacesState::new::<Self, _>(dh, unsandboxed);
        let screens_state = ScreensState::new::<Self, _>(dh, unsandboxed);
        let output_power_state = OutputPowerState::new::<Self, _>(dh, unsandboxed);
        let night_state = NightState::new::<Self, _>(dh, unsandboxed);

        let session_lock_state = SessionLockManagerState::new::<Self, _>(dh, unsandboxed);

        crate::locked::mark(false);
        let idle_notifier_state = IdleNotifierState::new(dh, handle.clone());
        let idle_inhibit_state = IdleInhibitManagerState::new::<Self>(dh);
        ScreenGlobals {
            windows_state,
            workspaces_state,
            screens_state,
            output_power_state,
            night_state,
            session_lock_state,
            idle_notifier_state,
            idle_inhibit_state,
        }
    }

    fn add_seat(seat_state: &mut SeatState<Self>, dh: &DisplayHandle, seat_name: String) -> SeatParts<BackendData> {
        let mut seat = seat_state.new_wl_seat(dh, seat_name.clone());

        let pointer = seat.add_pointer();
        let config = crate::config::Config::load();
        crate::shell::ssd::set_theme(config.theme, config.corner_radius);
        let (layout, variant, model, options) = crate::config::xkb_names();
        let xkb = XkbConfig { rules: "", model: &model, layout: &layout, variant: &variant, options };
        let keyboard = seat.add_keyboard(xkb, config.repeat_delay, config.repeat_rate)
            .or_else(|_| seat.add_keyboard(XkbConfig::default(), config.repeat_delay, config.repeat_rate))
            .expect("Failed to initialize the keyboard");

        if remembered_num_lock() {
            let mut mods = keyboard.modifier_state();
            mods.num_lock = true;
            keyboard.set_modifier_state(mods);
        }
        SeatParts { seat_name, seat, pointer, config }
    }

    fn assemble(parts: Parts<BackendData>) -> Self {
        let Parts {
            backend_data, display_handle, socket_name, handle, clock, seat_state,
            keyboard_shortcuts_inhibit_state, xwayland_shell_state, desktop, input, screens, seat,
        } = parts;
        let initial_workspaces = seat.config.workspaces.max(1);
        AnvilState {
            config: seat.config, release_key: None, released_bind: None, startup: crate::session::launch().startup, ring: Default::default(),
            desktop_tap: None, edge_cursor: false, backend_data, display_handle, socket_name, running: Arc::new(AtomicBool::new(true)), handle,
            space: Space::default(), popups: PopupManager::default(),
            compositor_state: desktop.compositor_state, data_device_state: desktop.data_device_state, layer_shell_state: desktop.layer_shell_state,
            output_manager_state: desktop.output_manager_state, primary_selection_state: desktop.primary_selection_state,
            data_control_state: desktop.data_control_state, seat_state, keyboard_shortcuts_inhibit_state, shm_state: desktop.shm_state,
            viewporter_state: desktop.viewporter_state, xdg_activation_state: desktop.xdg_activation_state,
            xdg_decoration_state: desktop.xdg_decoration_state, xdg_shell_state: desktop.xdg_shell_state,
            presentation_state: desktop.presentation_state, fractional_scale_manager_state: desktop.fractional_scale_manager_state,
            xdg_foreign_state: desktop.xdg_foreign_state, single_pixel_buffer_state: desktop.single_pixel_buffer_state,
            fifo_manager_state: desktop.fifo_manager_state, commit_timing_manager_state: desktop.commit_timing_manager_state,
            windows_state: screens.windows_state, workspaces_state: screens.workspaces_state,
            minimized: Vec::new(), hidden: Vec::new(), close_asked: Vec::new(), fresh: std::collections::HashMap::new(),
            fresh_for: std::time::Duration::from_secs(5), workspaces: initial_workspaces, active_workspace: 0,
            image_capture_state: desktop.image_capture_state, toplevel_list_state: desktop.toplevel_list_state, toplevel_list: Vec::new(),
            screens_state: screens.screens_state, output_power_state: screens.output_power_state, night_state: screens.night_state,
            session_lock_state: screens.session_lock_state, session_lock: SessionLock::default(), plugged: Default::default(),
            idle_notifier_state: screens.idle_notifier_state, idle_inhibit_state: screens.idle_inhibit_state, idle_inhibitors: Vec::new(),
            pads: Default::default(), dnd_icon: None, dragging: None, xdnd: Default::default(), suppressed_keys: Vec::new(),
            keyboard_layers: Vec::new(), window_before_panel: None, keys_to_panel: None, modal: None, modal_limit: crate::modal::LIMIT, keys_to_nothing: false,
            expander: Default::default(), reader_keys: Default::default(), input_capture_state: input.input_capture_state,
            own_press_state: input.own_press_state, capture: Default::default(), scroll_scaler: Default::default(),
            keys_changed: crate::keys_changed::KeysChanged::load(), xkb_names: crate::config::xkb_names(),
            cursor_status: CursorImageStatus::default_named(), seat_name: seat.seat_name, seat: seat.seat, pointer: seat.pointer, clock,
            xwayland_shell_state, rooms: crate::xrooms::Rooms::default(), xdisplay: None, wayland_selection: [None, None],
        }
    }
}
