// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::{
    collections::hash_map::HashMap,
    time::{Duration, Instant},
};

use smithay::{
    backend::{
        allocator::Fourcc,
        drm::{
            compositor::PrimaryPlaneElement,
            DrmDeviceFd, DrmNode,
        },
        renderer::{
            damage::Error as OutputDamageTrackerError,
            element::{memory::MemoryRenderBuffer, RenderElementStates},
            gles::GlesRenderer,
            multigpu::{gbm::GbmGlesBackend, GpuManager},
        },
        session::Session,
        SwapBuffersError,
    },
    desktop::space::Space,
    input::pointer::{CursorIcon, CursorImageStatus},
    output::Output,
    reexports::{
        calloop::timer::{TimeoutAction, Timer},
        drm::control::crtc,
    },
    utils::{IsAlive, Logical, Monotonic, Point, Scale, Time, Transform},
};
use tracing::{error, info, trace};

use super::{
    frame::output_for,
    recovery::{failed_frame, quiet_planes},
    AnvilState, BackendData, SurfaceData, UdevData, UdevOutputId, UdevRenderer,
};
use crate::{
    clean_start::CleanStart,
    drawing::*,
    lock::SessionLock,
    render::*,
    shell::WindowElement,
    state::{take_presentation_feedback, update_primary_scanout_output, DndIcon},
};

pub(super) struct AfterFrame {

    pub reschedule: bool,
    pub frames: u64,

    pub rests: bool,

    pub clean_again: bool,

    pub set_up_again: bool,
}

impl AfterFrame {
    fn drawn(rests: bool, has_rendered: bool) -> Self {
        AfterFrame { reschedule: !has_rendered, frames: 1, rests, clean_again: false, set_up_again: false }
    }
}

impl AnvilState<UdevData> {

    pub(super) fn render(&mut self, node: DrmNode, crtc: Option<crtc::Handle>, frame_target: Time<Monotonic>) {
        let device_backend = match self.backend_data.backends.get_mut(&node) {
            Some(backend) => backend,
            None => {
                error!("Trying to render on non-existent backend {}", node);
                return;
            }
        };

        if let Some(crtc) = crtc {
            self.render_surface(node, crtc, frame_target);
        } else {
            let crtcs: Vec<_> = device_backend.surfaces.keys().copied().collect();
            for crtc in crtcs {
                self.render_surface(node, crtc, frame_target);
            }
        };
    }

    pub(super) fn render_surface(&mut self, node: DrmNode, crtc: crtc::Handle, frame_target: Time<Monotonic>) {
        self.show_drag();
        self.keep_pointer_away();

        let Some(was_resting) = self.take_frame_slot(node, crtc) else {
            return;
        };

        let Some(output) = output_for(&self.space, UdevOutputId { device_id: node, crtc }) else {

            return;
        };

        if !self.anything_to_show(&output) {
            let wait = Duration::from_millis(50);
            self.render_when(Timer::from_duration(wait), node, crtc, frame_target + wait);
            return;
        }

        self.pre_repaint(&output, frame_target);
        self.begin_clean_start(node);
        self.draw_frame(node, crtc, &output, frame_target, was_resting);
    }

    fn draw_frame(
        &mut self,
        node: DrmNode,
        crtc: crtc::Handle,
        output: &Output,
        frame_target: Time<Monotonic>,
        was_resting: bool,
    ) {
        let Some((surface, clean_start)) = screen_of(&mut self.backend_data.backends, node, crtc) else {
            return;
        };

        let start = Instant::now();

        let (pointer_image, hotspot) = pointer_picture(
            &mut self.backend_data.pointer_image,
            &mut self.backend_data.pointer_images,
            &self.cursor_status,
            output.current_scale().fractional_scale(),
            self.clock.now(),
        );

        let primary_gpu = self.backend_data.primary_gpu;
        let mut renderer = frame_renderer(&mut self.backend_data.gpus, primary_gpu, surface);

        let result = render_surface(
            surface, &mut renderer, &self.space, output, self.pointer.current_location(),
            (&pointer_image, hotspot), &mut self.backend_data.pointer_element,
            &self.dnd_icon, &mut self.cursor_status, &self.session_lock,
        );
        let locked = self.session_lock.is_active();
        let after = match result {
            Ok((has_rendered, states)) => {
                let rests = note_drawn(surface, output, start, was_resting, has_rendered, locked);
                let dmabuf_feedback = surface.dmabuf_feedback.clone();
                self.post_repaint(output, frame_target, dmabuf_feedback, &states);
                if locked && !has_rendered {
                    self.lock_frame_shown(output);
                }
                AfterFrame::drawn(rests, has_rendered)
            }
            Err(err) => failed_frame(clean_start, surface, output, crtc, err),
        };
        self.follow_up(node, crtc, output, frame_target, start, &after);
    }

    fn follow_up(
        &mut self,
        node: DrmNode,
        crtc: crtc::Handle,
        output: &Output,
        frame_target: Time<Monotonic>,
        start: Instant,
        after: &AfterFrame,
    ) {
        if after.set_up_again {
            self.set_screens_up_again(node);
        }

        if after.clean_again {
            self.clean_again_soon(node);
        }

        self.service_image_capture(output);

        self.ask_for_next_frame(node, crtc, output, frame_target, start, after);
    }

    pub(super) fn render_when(&mut self, timer: Timer, node: DrmNode, crtc: crtc::Handle, target: Time<Monotonic>) {
        self.handle
            .insert_source(timer, move |_, _, data| {
                data.render(node, Some(crtc), target);
                TimeoutAction::Drop
            })
            .expect("failed to schedule frame timer");
    }

    fn take_frame_slot(&mut self, node: DrmNode, crtc: crtc::Handle) -> Option<bool> {
        let mut was_resting = false;
        let (lit, holding) = self
            .backend_data
            .backends
            .get_mut(&node)
            .and_then(|device| Some((device.surfaces.get_mut(&crtc)?, device.clean_start.waiting())))
            .map_or((false, false), |(surface, waiting)| {
                was_resting = surface.resting.take().is_some();
                (surface.enabled && surface.powered, waiting)
            });

        (lit && !holding).then_some(was_resting)
    }

    fn begin_clean_start(&mut self, node: DrmNode) {
        let begin = self.backend_data.session.is_active()
            && self
                .backend_data
                .backends
                .get_mut(&node)
                .is_some_and(|device| device.clean_start.begin());
        if begin {
            self.start_cleanly(node);
        }
    }

    fn ask_for_next_frame(
        &mut self,
        node: DrmNode,
        crtc: crtc::Handle,
        output: &Output,
        frame_target: Time<Monotonic>,
        start: Instant,
        after: &AfterFrame,
    ) {
        if after.reschedule && after.rests {
            if let Some(surface) = self.backend_data.backends.get_mut(&node).and_then(|d| d.surfaces.get_mut(&crtc)) {
                surface.resting = Some(Instant::now());
            }
        } else if after.reschedule {
            let output_refresh = match output.current_mode() {
                Some(mode) => mode.refresh,
                None => return,
            };

            let next_frame_target = frame_target + Duration::from_millis(frames_to_millis(after.frames, output_refresh as u64));
            let reschedule_timeout =
                Duration::from(next_frame_target).saturating_sub(self.clock.now().into());
            trace!(
                "reschedule repaint timer with delay {:?} on {:?}",
                reschedule_timeout,
                crtc,
            );
            self.render_when(Timer::from_duration(reschedule_timeout), node, crtc, next_frame_target);
        } else {
            let elapsed = start.elapsed();
            tracing::trace!(?elapsed, "rendered surface");
        }
    }
}

fn note_rest(surface: &mut SurfaceData, output: &Output, start: Instant, was_resting: bool, has_rendered: bool) -> bool {
    let mut rests = false;
    if has_rendered {
        surface.still_since = None;
    } else if crate::rest::allowed() {
        rests = crate::rest::rests(surface.still_since.get_or_insert(start).elapsed());
    }
    if crate::rest::told() {
        if was_resting {
            surface.rest_looks.0 += 1;
            surface.rest_looks.1 += has_rendered as u32;
        }

        let changed = (rests && !was_resting) || (was_resting && has_rendered);
        if changed && surface.rest_told.is_none_or(|t| t.elapsed() >= Duration::from_secs(5)) {
            info!(
                target: crate::rest::LOG,
                "{} {}: {} looks at rest since the last line, {} found something to draw",
                output.name(),
                if rests { "rests" } else { "wakes" },
                surface.rest_looks.0,
                surface.rest_looks.1,
            );
            surface.rest_looks = (0, 0);
            surface.rest_told = Some(Instant::now());
        }
    }
    rests
}

fn note_drawn(
    surface: &mut SurfaceData,
    output: &Output,
    start: Instant,
    was_resting: bool,
    has_rendered: bool,
    locked: bool,
) -> bool {
    let rests = note_rest(surface, output, start, was_resting, has_rendered);

    if has_rendered {
        if surface.failed_frames > 1 {
            info!("{} draws again after {} failed frames", output.name(), surface.failed_frames);
        }
        surface.refused_flips = 0;
        surface.failed_frames = 0;
        surface.lit = true;
        if surface.recovery.good() {
            info!("{} shows our frames again", output.name());
        }
    }

    surface.locked_frame_queued = locked && has_rendered;
    rests
}

fn pointer_picture(
    cursor: &mut crate::cursor::Cursor,
    cache: &mut Vec<(xcursor::parser::Image, u32, MemoryRenderBuffer)>,
    cursor_status: &CursorImageStatus,
    screen_scale: f64,
    now: Time<Monotonic>,
) -> (MemoryRenderBuffer, Point<i32, Logical>) {

    let picture_scale = crate::cursor::picture_scale(screen_scale);

    let icon = match cursor_status {
        CursorImageStatus::Named(icon) => *icon,
        _ => CursorIcon::Default,
    };
    let frame = cursor.get_named(icon, picture_scale, now.into());
    let hotspot = crate::cursor::hotspot_points(frame.xhot, frame.yhot, picture_scale);

    let pointer_image = cache
        .iter()
        .find_map(|(image, scale, texture)| {
            if image == &frame && *scale == picture_scale {
                Some(texture.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            let buffer = MemoryRenderBuffer::from_slice(
                &frame.pixels_rgba,
                Fourcc::Argb8888,
                (frame.width as i32, frame.height as i32),
                picture_scale as i32,
                Transform::Normal,
                None,
            );
            cache.push((frame, picture_scale, buffer.clone()));
            buffer
        });
    (pointer_image, hotspot)
}

fn screen_of(
    backends: &mut HashMap<DrmNode, BackendData>,
    node: DrmNode,
    crtc: crtc::Handle,
) -> Option<(&mut SurfaceData, &mut CleanStart)> {
    let device = backends.get_mut(&node)?;
    let surface = device.surfaces.get_mut(&crtc)?;
    Some((surface, &mut device.clean_start))
}

fn frame_renderer<'a>(
    gpus: &'a mut GpuManager<GbmGlesBackend<GlesRenderer, DrmDeviceFd>>,
    primary_gpu: DrmNode,
    surface: &SurfaceData,
) -> UdevRenderer<'a> {
    let render_node = surface.render_node.unwrap_or(primary_gpu);
    if primary_gpu == render_node {
        gpus.single_renderer(&render_node)
    } else {
        let format = surface.drm_output.format();
        gpus.renderer(&primary_gpu, &render_node, format)
    }
    .unwrap()
}

fn render_surface<'a>(
    surface: &'a mut SurfaceData,
    renderer: &mut UdevRenderer<'a>,
    space: &Space<WindowElement>,
    output: &Output,
    pointer_location: Point<f64, Logical>,
    (pointer_image, hotspot): (&MemoryRenderBuffer, Point<i32, Logical>),
    pointer_element: &mut PointerElement,
    dnd_icon: &Option<DndIcon>,
    cursor_status: &mut CursorImageStatus,
    session_lock: &SessionLock,
) -> Result<(bool, RenderElementStates), SwapBuffersError> {
    let output_geometry = space.output_geometry(output).unwrap();
    let scale = Scale::from(output.current_scale().fractional_scale());

    set_pointer(pointer_element, cursor_status, pointer_image, hotspot, scale);

    let custom_elements = pointer_elements(renderer, output_geometry, scale, pointer_location,
                                           pointer_element, cursor_status, dnd_icon.as_ref());

    let (elements, clear_color) =
        output_elements(output, space, custom_elements, renderer, session_lock);

    let still = !crate::animate::moving_now()
        && !space.elements_for_output(output).any(|w| w.is_fullscreen());
    let frame_mode = quiet_planes(surface.frame_flags, still);
    let (rendered, states) = surface
        .drm_output
        .render_frame(renderer, &elements, clear_color, frame_mode)
        .map(|render_frame_result| {

            if render_frame_result.needs_sync() {
                if let PrimaryPlaneElement::Swapchain(element) = &render_frame_result.primary_element {
                    let _ = element.sync.wait();
                }
            }
            (!render_frame_result.is_empty, render_frame_result.states)
        })
        .map_err(|err| match err {
            smithay::backend::drm::compositor::RenderFrameError::PrepareFrame(err) => {
                SwapBuffersError::from(err)
            }
            smithay::backend::drm::compositor::RenderFrameError::RenderFrame(
                OutputDamageTrackerError::Rendering(err),
            ) => SwapBuffersError::from(err),
            _ => unreachable!(),
        })?;

    update_primary_scanout_output(space, output, dnd_icon, cursor_status, &states);

    if rendered {
        queue_frame(surface, space, output, &states)?;
    }

    Ok((rendered, states))
}

fn queue_frame(
    surface: &mut SurfaceData,
    space: &Space<WindowElement>,
    output: &Output,
    states: &RenderElementStates,
) -> Result<(), SwapBuffersError> {
    let output_presentation_feedback = take_presentation_feedback(output, space, states);
    surface
        .drm_output
        .queue_frame(Some(output_presentation_feedback))
        .map_err(Into::<SwapBuffersError>::into)
}

fn set_pointer(
    pointer_element: &mut PointerElement,
    cursor_status: &mut CursorImageStatus,
    pointer_image: &MemoryRenderBuffer,
    hotspot: Point<i32, Logical>,
    scale: Scale<f64>,
) {
    pointer_element.set_buffer(pointer_image.clone(), hotspot);

    if let CursorImageStatus::Surface(ref surface) = *cursor_status {
        if !surface.alive() {
            *cursor_status = CursorImageStatus::default_named();
        }
    }
    pointer_element.set_status(cursor_status.clone());
    let spot = pointer_hotspot(cursor_status, pointer_element).to_physical_precise_round(scale);
    smithay::backend::drm::set_cursor_hotspot(spot.x, spot.y);
}
