// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::{ops::Not, sync::Once, time::Duration};

use smithay::{
    backend::{
        drm::{DrmError, DrmEventMetadata, DrmEventTime, DrmNode},
        SwapBuffersError,
    },
    desktop::space::Space,
    output::Output,
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            LoopHandle,
        },
        drm::control::crtc,
        wayland_protocols::wp::presentation_time::server::wp_presentation_feedback,
    },
    utils::{Monotonic, Time},
    wayland::presentation::Refresh,
};
use tracing::{error, trace, warn};

use super::{recovery::try_again, AnvilState, SurfaceData, UdevData, UdevOutputId};
use crate::{render::frame_duration, shell::WindowElement};

struct Vblank {

    tp: Option<Duration>,
    sequence: u32,
    clock: Time<Monotonic>,
    flags: wp_presentation_feedback::Kind,
}

impl Vblank {
    fn of(metadata: Option<&DrmEventMetadata>, now: impl FnOnce() -> Time<Monotonic>) -> Self {
        let tp = metadata.and_then(|metadata| match metadata.time {
            DrmEventTime::Monotonic(tp) => tp.is_zero().not().then_some(tp),
            DrmEventTime::Realtime(_) => None,
        });

        let sequence = metadata.map(|metadata| metadata.sequence).unwrap_or(0);

        let (clock, flags) = if let Some(tp) = tp {
            (
                tp.into(),
                wp_presentation_feedback::Kind::Vsync
                    | wp_presentation_feedback::Kind::HwClock
                    | wp_presentation_feedback::Kind::HwCompletion,
            )
        } else {
            (now(), wp_presentation_feedback::Kind::Vsync)
        };
        Vblank { tp, sequence, clock, flags }
    }
}

impl AnvilState<UdevData> {
    pub(super) fn frame_finish(&mut self, dev_id: DrmNode, crtc: crtc::Handle, metadata: &mut Option<DrmEventMetadata>) {
        let Some(surface) = vblank_surface(&mut self.backend_data, dev_id, crtc) else {
            return;
        };

        if let Some(timer_token) = surface.vblank_throttle_timer.take() {
            self.handle.remove(timer_token);
        }

        if !(surface.enabled && surface.powered) {
            return;
        }

        let Some(output) = output_for(&self.space, UdevOutputId { device_id: surface.device_id, crtc }) else {

            return;
        };

        let Some(frame_duration) = output
            .current_mode()
            .map(|mode| frame_duration(mode.refresh))
        else {
            return;
        };

        let vblank = Vblank::of(metadata.as_ref(), || self.clock.now());
        if throttled(&self.handle, surface, dev_id, crtc, frame_duration, &vblank) {
            return;
        }
        surface.last_presentation_time = Some(vblank.clock);

        let (schedule_render, locked_frame_shown) = submit_frame(surface, frame_duration, &vblank);

        if schedule_render {
            let copies = surface
                .render_node
                .map(|render_node| render_node != self.backend_data.primary_gpu)
                .unwrap_or(true);
            self.schedule_repaint(dev_id, crtc, vblank.clock + frame_duration, frame_duration, copies);
        }

        if locked_frame_shown {
            self.lock_frame_shown(&output);
        }
    }

    fn schedule_repaint(
        &mut self,
        dev_id: DrmNode,
        crtc: crtc::Handle,
        next_frame_target: Time<Monotonic>,
        frame_duration: Duration,
        copies: bool,
    ) {

        let repaint_delay = Duration::from_secs_f64(frame_duration.as_secs_f64() * 0.6f64);

        let timer = if copies {

            trace!("scheduling repaint timer immediately on {:?}", crtc);
            Timer::immediate()
        } else {
            trace!(
                "scheduling repaint timer with delay {:?} on {:?}",
                repaint_delay,
                crtc
            );
            Timer::from_duration(repaint_delay)
        };

        self.render_when(timer, dev_id, crtc, next_frame_target);
    }
}

fn vblank_surface(data: &mut UdevData, dev_id: DrmNode, crtc: crtc::Handle) -> Option<&mut SurfaceData> {
    let device_backend = match data.backends.get_mut(&dev_id) {
        Some(backend) => backend,
        None => {
            error!("Trying to finish frame on non-existent backend {}", dev_id);
            return None;
        }
    };

    match device_backend.surfaces.get_mut(&crtc) {
        Some(surface) => Some(surface),
        None => {
            error!("Trying to finish frame on non-existent crtc {:?}", crtc);
            None
        }
    }
}

pub(super) fn output_for(space: &Space<WindowElement>, id: UdevOutputId) -> Option<Output> {
    space
        .outputs()
        .find(|o| o.user_data().get::<UdevOutputId>() == Some(&id))
        .cloned()
}

fn throttled(
    handle: &LoopHandle<'static, AnvilState<UdevData>>,
    surface: &mut SurfaceData,
    dev_id: DrmNode,
    crtc: crtc::Handle,
    frame_duration: Duration,
    vblank: &Vblank,
) -> bool {
    let vblank_remaining_time = surface.last_presentation_time.map(|last_presentation_time| {
        frame_duration.saturating_sub(Time::elapsed(&last_presentation_time, vblank.clock))
    });

    let Some(vblank_remaining_time) = vblank_remaining_time else { return false };
    if vblank_remaining_time <= frame_duration / 2 {
        return false;
    }
    static WARN_ONCE: Once = Once::new();
    WARN_ONCE.call_once(|| {
        warn!("display running faster than expected, throttling vblanks and disabling HwClock")
    });
    let throttled_time = vblank
        .tp
        .map(|tp| tp.saturating_add(vblank_remaining_time))
        .unwrap_or(Duration::ZERO);
    let throttled_metadata = DrmEventMetadata {
        sequence: vblank.sequence,
        time: DrmEventTime::Monotonic(throttled_time),
    };
    let timer_token = handle
        .insert_source(Timer::from_duration(vblank_remaining_time), move |_, _, data| {
            data.frame_finish(dev_id, crtc, &mut Some(throttled_metadata));
            TimeoutAction::Drop
        })
        .expect("failed to register vblank throttle timer");
    surface.vblank_throttle_timer = Some(timer_token);
    true
}

fn submit_frame(surface: &mut SurfaceData, frame_duration: Duration, vblank: &Vblank) -> (bool, bool) {
    let submit_result = surface
        .drm_output
        .frame_submitted()
        .map_err(Into::<SwapBuffersError>::into);

    let locked_frame_shown = submit_result.is_ok() && std::mem::take(&mut surface.locked_frame_queued);
    let schedule_render = match submit_result {
        Ok(user_data) => {
            if let Some(mut feedback) = user_data.flatten() {

                let refresh = if surface.drm_output.with_compositor(|c| c.vrr_enabled()) {
                    Refresh::variable(frame_duration)
                } else {
                    Refresh::fixed(frame_duration)
                };
                feedback.presented(vblank.clock, refresh, vblank.sequence as u64, vblank.flags);
            }

            true
        }
        Err(err) => draw_again_after(err),
    };
    (schedule_render, locked_frame_shown)
}

fn draw_again_after(err: SwapBuffersError) -> bool {
    warn!("Error during rendering: {:?}", err);
    match err {
        SwapBuffersError::AlreadySwapped => true,

        SwapBuffersError::TemporaryFailure(err)
            if matches!(err.downcast_ref::<DrmError>(), Some(&DrmError::DeviceInactive)) =>
        {
            false
        }
        SwapBuffersError::TemporaryFailure(err) => {
            err.downcast_ref::<DrmError>().is_some_and(try_again)
        }

        SwapBuffersError::ContextLost(err) => {
            warn!("Frame lost on its way to the screen: {}", err);
            true
        }
    }
}
