// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! When a frame fails: counting failures, giving up planes, and starting a card's screens over.

use std::{io, time::Instant};

use smithay::{
    backend::{
        drm::{compositor::FrameFlags, DrmAccessError, DrmError, DrmNode},
        SwapBuffersError,
    },
    output::Output,
    reexports::{calloop::timer::{TimeoutAction, Timer}, drm::control::crtc},
};
use tracing::{debug, info, warn};

use super::{paint::AfterFrame, AnvilState, SurfaceData, UdevData};
use crate::{
    clean_start::{self, CleanStart},
    render::{frames_in, ASSUMED_REFRESH},
};

pub(super) fn failed_frame(
    clean_start: &mut CleanStart,
    surface: &mut SurfaceData,
    output: &Output,
    crtc: crtc::Handle,
    err: SwapBuffersError,
) -> AfterFrame {
    surface.failed_frames = surface.failed_frames.saturating_add(1);
    let mut failed = AfterFrame {
        reschedule: false,
        frames: surface.failed_frames.clamp(1, 4) as u64,
        rests: false,
        clean_again: false,
        set_up_again: false,
    };
    surface.still_since = None;
    let refusal = drm_error(&err).filter(|e| !matches!(e, DrmError::DeviceInactive));
    let refused = refusal.is_some();
    let next = match refusal {
        Some(_) => clean_start.refused(surface.lit),
        None => clean_start::Refused::Ladder { first: false },
    };
    if next == clean_start::Refused::CleanAgain {
        say_first_try_failed(surface, output, crtc, refusal, true);
        failed.clean_again = true;
        return failed;
    }
    if next == (clean_start::Refused::Ladder { first: true }) {
        say_first_try_failed(surface, output, crtc, refusal, false);
    }
    if clean_start::worth_a_line(surface.failed_frames) {
        warn!("Error during rendering ({} in a row): {:#?}", surface.failed_frames, err);
    }
    let (again, want_setup) = retry_after(surface, &err, refused);
    failed.reschedule = again;
    if want_setup {
        space_out_setup(surface, output, &mut failed);
    }
    failed
}

fn say_first_try_failed(
    surface: &SurfaceData,
    output: &Output,
    crtc: crtc::Handle,
    refusal: Option<&DrmError>,
    again: bool,
) {
    let mode = surface.drm_output.with_compositor(|c| c.pending_mode());
    let (w, h) = mode.size();
    warn!(
        "{} refused its first frame after {} clean start ({}): mode {w}x{h} at {} Hz, pipe {:?}, {}; {}",
        output.name(),
        if again { "a" } else { "a second" },
        refusal.map(|e| e.to_string()).unwrap_or_default(),
        mode.vrefresh(),
        crtc,
        planes_named(surface.frame_flags),
        if again {
            "starting the card cleanly once more in half a second"
        } else {
            "trying with fewer planes and starting over now and then"
        },
    );
}

fn retry_after(surface: &mut SurfaceData, err: &SwapBuffersError, refused: bool) -> (bool, bool) {
    match err {
        SwapBuffersError::AlreadySwapped => (true, false),
        SwapBuffersError::TemporaryFailure(err) => {
            let want_setup = refused && !surface.lit;
            if let Some(e) = err.downcast_ref::<DrmError>() {
                if !matches!(e, DrmError::DeviceInactive)
                    && !try_again(e)
                    && clean_start::worth_a_line(surface.failed_frames)
                {
                    warn!("Drawing failed, trying again shortly: {}", e);
                }
            }
            (true, want_setup)
        }
        SwapBuffersError::ContextLost(err) => (true, flip_refused(surface, &**err)),
    }
}

fn flip_refused(surface: &mut SurfaceData, err: &(dyn std::error::Error + Send + Sync + 'static)) -> bool {
    surface.refused_flips += 1;
    let test_failed = matches!(err.downcast_ref::<DrmError>(), Some(DrmError::TestFailed(_)));
    let fewer = fewer_planes(surface.frame_flags);
    if surface.lit && !test_failed && fewer != surface.frame_flags {
        warn!("Flip refused, drawing with fewer planes from now on: {}", err);
        surface.frame_flags = fewer;
        false
    } else {
        if clean_start::worth_a_line(surface.refused_flips) {
            warn!("Flip refused ({} in a row): {}", surface.refused_flips, err);
        }
        true
    }
}

fn space_out_setup(surface: &mut SurfaceData, output: &Output, failed: &mut AfterFrame) {
    match surface.recovery.refused(Instant::now()) {
        crate::modeset_recovery::Decision::Wait(wait) => {
            let refresh = output.current_mode().map_or(ASSUMED_REFRESH, |m| m.refresh.max(1)) as u64;
            failed.frames = failed.frames.max(frames_in(wait, refresh));
        }
        crate::modeset_recovery::Decision::SetUp { attempt, say, slow } => {
            failed.set_up_again = true;
            if say {
                warn!(
                    "{} keeps refusing its frames, setting the screens up in full again (try {}{})",
                    output.name(),
                    attempt,
                    if slow { ", now every 15 s for as long as it takes" } else { "" },
                );
            }
        }
    }
}

impl AnvilState<UdevData> {
    pub(super) fn start_cleanly(&mut self, node: DrmNode) {
        let Some(device) = self.backend_data.backends.get_mut(&node) else { return };
        if let Err(err) = device.drm_output_manager.device_mut().reset_state() {
            warn!("could not switch the screens on {} off for a clean start: {}", node, err);
        }
        for surface in device.surfaces.values_mut() {
            if let Err(err) = surface.drm_output.with_compositor(|c| c.reset_state()) {
                debug!(?err, "could not read what {} shows again", surface.output.name());
            }
        }
    }

    pub(super) fn set_screens_up_again(&mut self, node: DrmNode) {
        let Some(device) = self.backend_data.backends.get_mut(&node) else { return };
        let manager = &mut device.drm_output_manager;
        if let Err(err) = crate::modeset_recovery::retake(false, |full| manager.activate(full)) {
            warn!("could not set the screens on {} up again: {}", node, err);
        }
    }

    pub(super) fn clean_again_soon(&mut self, node: DrmNode) {
        self.handle
            .insert_source(Timer::from_duration(clean_start::RETRY_AFTER), move |_, _, data| {
                data.clean_again(node);
                TimeoutAction::Drop
            })
            .expect("failed to schedule a clean start");
    }

    pub(super) fn clean_again(&mut self, node: DrmNode) {
        let due = self
            .backend_data
            .backends
            .get_mut(&node)
            .is_some_and(|device| device.clean_start.due());
        if due {
            info!("starting the screens on {} cleanly once more", node);
            self.start_cleanly(node);
            self.render(node, None, self.clock.now());
        }
    }
}

fn planes_named(flags: FrameFlags) -> &'static str {
    let overlay = flags.contains(FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT);
    let cursor = flags.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT);
    match (cursor, overlay) {
        (true, true) => "main, pointer and overlay planes",
        (true, false) => "main and pointer planes",
        (false, true) => "main and overlay planes",
        (false, false) => "main plane only",
    }
}

fn drm_error(err: &SwapBuffersError) -> Option<&DrmError> {
    match err {
        SwapBuffersError::TemporaryFailure(err) | SwapBuffersError::ContextLost(err) => err.downcast_ref::<DrmError>(),
        SwapBuffersError::AlreadySwapped => None,
    }
}

fn fewer_planes(flags: FrameFlags) -> FrameFlags {
    if flags.contains(FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT) {
        flags - FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT
    } else if flags.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT) {
        flags - FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT
    } else {
        FrameFlags::empty()
    }
}

pub(super) fn try_again(err: &DrmError) -> bool {
    match err {
        DrmError::Access(DrmAccessError { source, .. }) => matches!(
            source.kind(),
            io::ErrorKind::PermissionDenied
                | io::ErrorKind::ResourceBusy
                | io::ErrorKind::WouldBlock
                | io::ErrorKind::Interrupted
        ),
        _ => false,
    }
}

pub(super) fn quiet_planes(flags: FrameFlags, still: bool) -> FrameFlags {
    if still { flags } else { flags - FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_card_refusing_counts_as_a_refusal() {
        let away = SwapBuffersError::from(DrmError::DeviceInactive);
        assert!(matches!(away, SwapBuffersError::TemporaryFailure(_)));
        assert!(matches!(drm_error(&away), Some(DrmError::DeviceInactive)));
        let drawing = SwapBuffersError::ContextLost(Box::new(io::Error::other("lost the drawing context")));
        assert!(drm_error(&drawing).is_none());
        assert!(drm_error(&SwapBuffersError::AlreadySwapped).is_none());
    }

    #[test]
    fn planes_go_one_kind_at_a_time() {
        let once = fewer_planes(FrameFlags::DEFAULT);
        assert!(!once.contains(FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT));
        assert!(once.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT));
        let twice = fewer_planes(once);
        assert!(!twice.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT));
        assert_eq!(fewer_planes(twice), FrameFlags::empty());
        assert_eq!(fewer_planes(FrameFlags::empty()), FrameFlags::empty());
        assert_eq!(planes_named(FrameFlags::DEFAULT), "main, pointer and overlay planes");
        assert_eq!(planes_named(once), "main and pointer planes");
        assert_eq!(planes_named(FrameFlags::empty()), "main plane only");
        assert_eq!(quiet_planes(FrameFlags::DEFAULT, true), FrameFlags::DEFAULT);
        let busy = quiet_planes(FrameFlags::DEFAULT, false);
        assert!(!busy.contains(FrameFlags::ALLOW_OVERLAY_PLANE_SCANOUT));
        assert!(busy.contains(FrameFlags::ALLOW_PRIMARY_PLANE_SCANOUT));
        assert!(busy.contains(FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT));
    }
}
