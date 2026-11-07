// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Origin: began as a file of anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld). About 38% of this file's lines still read as
//! they did there; the rest is Trindle's own.

use std::{sync::atomic::Ordering, time::Duration};

use smithay::backend::renderer::ImportEgl;

use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        drm::DrmNode,
        egl::EGLDevice,
        renderer::{
            damage::{Error as OutputDamageTrackerError, OutputDamageTracker},
            element::RenderElementStates,
            gles::GlesRenderer,
            ImportDma, ImportMemWl,
        },
        winit::{self, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
        SwapBuffersError,
    },
    delegate_dmabuf,
    input::{
        keyboard::LedState,
        pointer::{CursorIcon, CursorImageStatus},
    },
    output::{Mode, Output, PhysicalProperties, Scale as OutputScale, Subpixel},
    reexports::{
        calloop::EventLoop,
        wayland_protocols::wp::presentation_time::server::wp_presentation_feedback,
        wayland_server::{protocol::wl_surface, Display, DisplayHandle},
        winit::platform::pump_events::PumpStatus,
    },
    utils::{IsAlive, Logical, Monotonic, Physical, Point, Scale, Size, Time, Transform},
    wayland::{
        dmabuf::{
            DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState, ImportNotifier,
        },
        presentation::Refresh,
    },
};
use tracing::{error, info, warn};

use crate::capture_scene::Scene;
use crate::protocols::screens::{ScreenFacts, ScreenMode, ScreenPlan};
use crate::state::{take_presentation_feedback, update_primary_scanout_output, AnvilState, Backend};
use crate::{drawing::*, render::*};

pub const OUTPUT_NAME: &str = "winit";

const FULL_REDRAW_FRAMES: u8 = 4;

pub struct WinitData {
    backend: WinitGraphicsBackend<GlesRenderer>,
    damage_tracker: OutputDamageTracker,
    dmabuf_state: (DmabufState, DmabufGlobal, Option<DmabufFeedback>),
    full_redraw: u8,
}

impl DmabufHandler for AnvilState<WinitData> {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        &mut self.backend_data.dmabuf_state.0
    }

    fn dmabuf_imported(&mut self, _global: &DmabufGlobal, dmabuf: Dmabuf, notifier: ImportNotifier) {
        if self
            .backend_data
            .backend
            .renderer()
            .import_dmabuf(&dmabuf, None)
            .is_ok()
        {
            let _ = notifier.successful::<AnvilState<WinitData>>();
        } else {
            notifier.failed();
        }
    }
}
delegate_dmabuf!(AnvilState<WinitData>);

impl Backend for WinitData {
    fn seat_name(&self) -> String {
        String::from("winit")
    }
    fn reset_buffers(&mut self, _output: &Output) {
        self.full_redraw = FULL_REDRAW_FRAMES;
    }
    fn early_import(&mut self, _surface: &wl_surface::WlSurface) {}
    fn update_led_state(&mut self, _led_state: LedState) {}

    fn output_heads(state: &AnvilState<Self>) -> Vec<ScreenFacts> {
        let Some(output) = state.space.outputs().next() else {
            return Vec::new();
        };
        let physical = output.physical_properties();
        let mode = output.current_mode().unwrap_or(Mode {
            size: (0, 0).into(),
            refresh: ASSUMED_REFRESH,
        });
        let location = output.current_location();
        vec![ScreenFacts {
            name: output.name(),
            description: output.description(),
            make: physical.make,
            model: physical.model,
            serial: None,
            size_mm: None,
            modes: vec![ScreenMode {
                width: mode.size.w,
                height: mode.size.h,
                millihertz: mode.refresh,
                preferred: true,
            }],
            on: true,
            mode: Some(0),
            position: (location.x, location.y),
            turn: user_transform(output.current_transform()),
            scale: output.current_scale().fractional_scale(),
            varies_rate: false,
        }]
    }

    fn apply_output_config(
        state: &mut AnvilState<Self>,
        targets: &[ScreenPlan],
        test_only: bool,
    ) -> Result<(), String> {
        let Some(output) = state.space.outputs().next().cloned() else {
            return Err("there is no window to arrange".into());
        };
        let [target] = targets else {
            return Err("a window has exactly one screen".into());
        };
        if !target.on || target.mode != Some(0) || target.varies_rate {
            return Err("in a window only position, turn and scale can change".into());
        }
        if test_only {
            return Ok(());
        }
        let position = Point::from(target.position);
        output.change_current_state(
            None,
            Some(window_transform(target.turn)),
            Some(OutputScale::Fractional(target.scale)),
            Some(position),
        );
        state.space.map_output(&output, position);
        smithay::desktop::layer_map_for_output(&output).arrange();
        crate::shell::fixup_positions(&mut state.space, state.pointer.current_location());
        state.backend_data.full_redraw = FULL_REDRAW_FRAMES;
        Ok(())
    }
}

const WINDOW_FLIP: Transform = Transform::Flipped180;

const TRANSFORMS: [Transform; 8] = [
    Transform::Normal,
    Transform::_90,
    Transform::_180,
    Transform::_270,
    Transform::Flipped,
    Transform::Flipped90,
    Transform::Flipped180,
    Transform::Flipped270,
];

fn compose(first: Transform, then: Transform) -> Transform {
    let size: Size<i32, Logical> = (3, 5).into();
    let corners: [Point<i32, Logical>; 3] = [(0, 0).into(), (3, 0).into(), (0, 5).into()];
    TRANSFORMS
        .into_iter()
        .find(|both| {
            corners.iter().all(|&p| {
                let middle = first.transform_point_in(p, &size);
                then.transform_point_in(middle, &first.transform_size(size)) == both.transform_point_in(p, &size)
            })
        })
        .unwrap_or(then)
}

fn window_transform(chosen: Transform) -> Transform {
    compose(chosen, WINDOW_FLIP)
}

fn user_transform(carried: Transform) -> Transform {
    TRANSFORMS
        .into_iter()
        .find(|&t| window_transform(t) == carried)
        .unwrap_or(Transform::Normal)
}

pub fn run_winit() {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display = Display::new().unwrap();
    let mut display_handle = display.handle();

    let Some((mut backend, mut winit)) = open_window() else {
        return;
    };
    smithay::backend::renderer::utils::set_early_shm_release(true);
    let output = new_output(&display_handle, backend.window_size());

    let (dmabuf_state, card) = share_dmabuf(&display_handle, &mut backend);

    if backend.renderer().bind_wl_display(&display_handle).is_ok() {
        info!("EGL hardware-acceleration enabled");
    };

    let damage_tracker = OutputDamageTracker::from_output(&output);
    let data = WinitData {
        backend,
        damage_tracker,
        dmabuf_state,
        full_redraw: 0,
    };
    let mut state = AnvilState::init(display, event_loop.handle(), data, true);
    start_session(&mut state, &output, card);

    info!("Initialization completed, starting the main loop.");

    let mut pointer_element = PointerElement::default();
    let mut host_cursor = CursorIcon::Default;

    while state.running.load(Ordering::SeqCst) {
        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { size, .. } => resized(&mut state, size),
            WinitEvent::Input(event) => state.process_input_event_windowed(event, OUTPUT_NAME),
            _ => (),
        });

        if let PumpStatus::Exit(_) = status {
            state.running.store(false, Ordering::SeqCst);
            break;
        }

        let drew = draw(&mut state, &output, &mut pointer_element, &mut host_cursor);
        wait_for_events(&mut event_loop, &mut state, &mut display_handle, drew);
    }
}

fn open_window() -> Option<(WinitGraphicsBackend<GlesRenderer>, WinitEventLoop)> {
    let asked = std::env::var("TRINDLE_WINDOW_SIZE").ok().and_then(|s| {
        let (w, h) = s.split_once('x')?;
        Some((w.trim().parse::<f64>().ok()?, h.trim().parse::<f64>().ok()?))
    });
    let made = match asked {
        Some((w, h)) => winit::init_from_attributes::<GlesRenderer>(
            smithay::reexports::winit::window::Window::default_attributes()
                .with_inner_size(smithay::reexports::winit::dpi::LogicalSize::new(w, h))
                .with_title("Smithay")
                .with_visible(true),
        ),
        None => winit::init::<GlesRenderer>(),
    };
    match made {
        Ok(ret) => Some(ret),
        Err(err) => {
            error!("Failed to initialize Winit backend: {}", err);
            None
        }
    }
}

fn login_scale(size: Size<i32, Physical>) -> Option<f64> {
    crate::session::launch().greeter.then(|| {
        crate::screen_scale::for_login(&crate::screen_scale::Screen {
            connector: OUTPUT_NAME.into(),
            make: "Smithay".into(),
            model: "Winit".into(),
            serial: String::new(),
            pixels: (size.w, size.h),
            millimetres: (0, 0),
        })
    })
}

fn new_output(display_handle: &DisplayHandle, size: Size<i32, Physical>) -> Output {
    let mode = Mode {
        size,
        refresh: ASSUMED_REFRESH,
    };
    let output = Output::new(
        OUTPUT_NAME.to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "Winit".into(),
        },
    );
    let _global = output.create_global::<AnvilState<WinitData>>(display_handle);
    output.change_current_state(Some(mode), Some(WINDOW_FLIP), login_scale(size).map(OutputScale::Fractional), Some((0, 0).into()));
    output.set_preferred(mode);
    output
}

fn share_dmabuf(
    display_handle: &DisplayHandle,
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
) -> ((DmabufState, DmabufGlobal, Option<DmabufFeedback>), Option<DrmNode>) {
    let render_node = EGLDevice::device_for_display(backend.renderer().egl_context().display())
        .and_then(|device| device.try_get_render_node());

    let card = render_node.as_ref().ok().copied().flatten();
    let dmabuf_default_feedback = match render_node {
        Ok(Some(node)) => {
            let dmabuf_formats = backend.renderer().dmabuf_formats();
            let dmabuf_default_feedback = DmabufFeedbackBuilder::new(node.dev_id(), dmabuf_formats)
                .build()
                .unwrap();
            Some(dmabuf_default_feedback)
        }
        Ok(None) => {
            warn!("failed to query render node, dmabuf will use v3");
            None
        }
        Err(err) => {
            warn!(?err, "failed to egl device for display, dmabuf will use v3");
            None
        }
    };

    let dmabuf_state = if let Some(default_feedback) = dmabuf_default_feedback {
        let mut dmabuf_state = DmabufState::new();
        let dmabuf_global = dmabuf_state.create_global_with_default_feedback::<AnvilState<WinitData>>(
            display_handle,
            &default_feedback,
        );
        (dmabuf_state, dmabuf_global, Some(default_feedback))
    } else {
        let dmabuf_formats = backend.renderer().dmabuf_formats();
        let mut dmabuf_state = DmabufState::new();
        let dmabuf_global =
            dmabuf_state.create_global::<AnvilState<WinitData>>(display_handle, dmabuf_formats);
        (dmabuf_state, dmabuf_global, None)
    };
    (dmabuf_state, card)
}

fn start_session(state: &mut AnvilState<WinitData>, output: &Output, card: Option<DrmNode>) {
    if let Some(node) = card {
        let formats = state.backend_data.backend.renderer().egl_context().dmabuf_render_formats().clone();
        state
            .image_capture_state
            .set_dmabuf(crate::protocols::image_capture::dmabuf_target(node.dev_id(), formats.iter()));
    }
    state
        .shm_state
        .update_formats(state.backend_data.backend.renderer().shm_formats());
    state.space.map_output(output, (0, 0));
    state.refresh_screens();

    state.watch_settings();
    state.watch_in_use();
    if crate::session::launch().greeter {
        state.session_ready();
    } else {
        state.start_xwayland();
    }
}

fn resized(state: &mut AnvilState<WinitData>, size: Size<i32, Physical>) {
    let output = state.space.outputs().next().unwrap().clone();
    state.space.map_output(&output, output.current_location());
    let mode = Mode {
        size,
        refresh: ASSUMED_REFRESH,
    };
    output.change_current_state(Some(mode), None, login_scale(size).map(OutputScale::Fractional), None);
    output.set_preferred(mode);
    crate::shell::fixup_positions(&mut state.space, state.pointer.current_location());
    state.refit_login_screen();
    state.refresh_screens();
}

struct HostCursor {
    visible: bool,
    named: Option<CursorIcon>,
}

fn draw(
    state: &mut AnvilState<WinitData>,
    output: &Output,
    pointer_element: &mut PointerElement,
    host_cursor: &mut CursorIcon,
) -> bool {
    let now = state.clock.now();
    let frame_target = now
        + output
            .current_mode()
            .map(|mode| frame_duration(mode.refresh))
            .unwrap_or_default();
    state.pre_repaint(output, frame_target);
    state.show_drag();

    let cursor = settle_cursor(state);
    pointer_element.set_status(state.cursor_status.clone());

    let mut drew = false;
    match render_window(state, output, pointer_element) {
        Ok((has_rendered, states)) => {
            drew = has_rendered;
            show_cursor(state, &cursor, host_cursor);
            present(state, output, frame_target, has_rendered, &states);
        }
        Err(SwapBuffersError::ContextLost(err)) => {
            error!("Critical Rendering Error: {}", err);
            state.running.store(false, Ordering::SeqCst);
        }
        Err(err) => warn!("Rendering error: {}", err),
    }

    fill_captures(state, output, pointer_element);
    drew
}

fn settle_cursor(state: &mut AnvilState<WinitData>) -> HostCursor {
    let mut reset = false;
    if let CursorImageStatus::Surface(ref surface) = state.cursor_status {
        reset = !surface.alive();
    }
    if reset {
        state.cursor_status = CursorImageStatus::default_named();
    }
    let visible = !matches!(state.cursor_status, CursorImageStatus::Surface(_));
    let named = match state.cursor_status {
        CursorImageStatus::Named(icon) => Some(icon),
        _ => None,
    };
    HostCursor { visible, named }
}

fn render_window(
    state: &mut AnvilState<WinitData>,
    output: &Output,
    pointer_element: &PointerElement,
) -> Result<(bool, RenderElementStates), SwapBuffersError> {
    let backend = &mut state.backend_data.backend;
    let full_redraw = &mut state.backend_data.full_redraw;
    *full_redraw = full_redraw.saturating_sub(1);
    let output_geometry = state.space.output_geometry(output).unwrap_or_default();
    let space = &mut state.space;
    let damage_tracker = &mut state.backend_data.damage_tracker;
    let session_lock = &state.session_lock;

    let dnd_icon = state.dnd_icon.as_ref();
    let cursor_status = &state.cursor_status;

    let scale = Scale::from(output.current_scale().fractional_scale());
    let cursor_pos = state.pointer.current_location();

    let age = if *full_redraw > 0 {
        0
    } else {
        backend.buffer_age().unwrap_or(0)
    };
    let render_res = backend.bind().and_then(|(renderer, mut fb)| {
        let elements =
            pointer_elements(renderer, output_geometry, scale, cursor_pos, pointer_element, cursor_status, dnd_icon);

        render_output(output, space, elements, renderer, &mut fb, damage_tracker, age, session_lock)
            .map_err(|err| match err {
                OutputDamageTrackerError::Rendering(err) => err.into(),
                _ => unreachable!(),
            })
    });

    render_res.map(|render_output_result| {
        let has_rendered = render_output_result.damage.is_some();
        if let Some(damage) = render_output_result.damage {
            if let Err(err) = backend.submit(Some(damage)) {
                warn!("Failed to submit buffer: {}", err);
            }
        }
        (has_rendered, render_output_result.states)
    })
}

fn show_cursor(state: &mut AnvilState<WinitData>, cursor: &HostCursor, host_cursor: &mut CursorIcon) {
    let backend = &mut state.backend_data.backend;
    backend.window().set_cursor_visible(cursor.visible);
    if let Some(icon) = cursor.named.filter(|i| *i != *host_cursor) {
        backend.window().set_cursor(icon);
        *host_cursor = icon;
    }
}

fn present(
    state: &mut AnvilState<WinitData>,
    output: &Output,
    frame_target: Time<Monotonic>,
    has_rendered: bool,
    states: &RenderElementStates,
) {
    update_primary_scanout_output(
        &state.space,
        output,
        &state.dnd_icon,
        &state.cursor_status,
        states,
    );
    if has_rendered {
        let mut output_presentation_feedback =
            take_presentation_feedback(output, &state.space, states);
        output_presentation_feedback.presented(
            frame_target,
            output
                .current_mode()
                .map(|mode| {
                    Refresh::fixed(frame_duration(mode.refresh))
                })
                .unwrap_or(Refresh::Unknown),
            0,
            wp_presentation_feedback::Kind::Vsync,
        )
    }

    state.post_repaint(output, frame_target, None, states);
    state.lock_frame_shown(output);
}

fn fill_captures(state: &mut AnvilState<WinitData>, output: &Output, pointer_element: &PointerElement) {
    let toplevels = state.image_capture_state.has_toplevel_frames();
    let pointers = state.image_capture_state.has_pointers();
    if !(state.image_capture_state.has_output_frames(output) || toplevels || pointers) {
        return;
    }
    let now = Duration::from(state.clock.now());
    let windows = if toplevels || pointers { state.capture_windows() } else { Vec::new() };
    let scene = Scene {
        space: &state.space,
        pointer_element,
        cursor_status: &state.cursor_status,
        dnd_icon: state.dnd_icon.as_ref(),
        session_lock: &state.session_lock,
        pointer_location: state.pointer.current_location(),
        windows: &windows,
        now,
    };
    let renderer = state.backend_data.backend.renderer();
    scene.fill_output(&mut state.image_capture_state, renderer, output, &state.handle);
    scene.fill_toplevels(&mut state.image_capture_state, renderer, &state.handle);
    scene.fill_pointers(&mut state.image_capture_state, renderer, &state.handle);
}

fn wait_for_events(
    event_loop: &mut EventLoop<'static, AnvilState<WinitData>>,
    state: &mut AnvilState<WinitData>,
    display_handle: &mut DisplayHandle,
    drew: bool,
) {
    let nap = if drew { Duration::from_millis(1) } else { crate::rest::A_FRAME };
    let result = event_loop.dispatch(Some(nap), state);
    if result.is_err() {
        state.running.store(false, Ordering::SeqCst);
    } else {
        state.space.refresh();
        state.popups.cleanup();
        state.refresh_panel();
        state.refresh_session_lock();
        state.refresh_idle_inhibit();
        state.refresh_suspended();
        display_handle.flush_clients().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upright_in_a_window_is_the_flip_alone() {
        assert_eq!(window_transform(Transform::Normal), WINDOW_FLIP);
        assert_eq!(user_transform(WINDOW_FLIP), Transform::Normal);
    }

    #[test]
    fn every_turn_survives_the_round_trip() {
        for t in TRANSFORMS {
            assert_eq!(user_transform(window_transform(t)), t);
        }
    }

    #[test]
    fn composing_matches_doing_both_in_turn() {
        assert_eq!(compose(Transform::_90, Transform::_90), Transform::_180);
        assert_eq!(compose(Transform::Flipped, Transform::Flipped), Transform::Normal);
        assert_eq!(compose(Transform::Normal, Transform::_270), Transform::_270);
        assert_eq!(
            compose(Transform::_90, Transform::Flipped),
            compose(Transform::Flipped, Transform::_270)
        );
    }
}
