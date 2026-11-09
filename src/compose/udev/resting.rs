// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use smithay::{
    output::Output,
    reexports::calloop::timer::{TimeoutAction, Timer},
};

use super::{AnvilState, UdevData};

const HOLD_FIRST_FRAME: Duration = Duration::from_secs(4);

impl AnvilState<UdevData> {

    pub(super) fn look_at_resting(&mut self) {
        let now = Instant::now();
        let mut due = Vec::new();
        for (node, device) in &mut self.backend_data.backends {
            for (crtc, surface) in &mut device.surfaces {
                let Some(looked) = surface.resting else { continue };
                let frame = surface
                    .output
                    .current_mode()
                    .map(|mode| crate::render::frame_duration(mode.refresh.max(1)))
                    .unwrap_or(crate::rest::A_FRAME);
                let wait = crate::rest::wait(frame, now.duration_since(looked));
                if wait.is_zero() {
                    due.push((*node, *crtc, None));
                } else if !std::mem::replace(&mut surface.rest_timer, true) {
                    due.push((*node, *crtc, Some(wait)));
                }
            }
        }
        for (node, crtc, wait) in due {
            let Some(wait) = wait else {
                self.render_surface(node, crtc, self.clock.now());
                continue;
            };
            self.handle
                .insert_source(Timer::from_duration(wait), move |_, _, data| {
                    if let Some(surface) = data.backend_data.backends.get_mut(&node).and_then(|d| d.surfaces.get_mut(&crtc)) {
                        surface.rest_timer = false;
                    }
                    TimeoutAction::Drop
                })
                .expect("failed to schedule frame timer");
        }
    }

    pub(super) fn anything_to_show(&mut self, output: &Output) -> bool {
        use std::sync::atomic::AtomicBool;
        use std::sync::OnceLock;
        static SHOWN: AtomicBool = AtomicBool::new(false);
        static SINCE: OnceLock<Instant> = OnceLock::new();
        if SHOWN.load(Ordering::Relaxed) {
            return true;
        }
        let since = *SINCE.get_or_init(Instant::now);
        let drawn = |w: &crate::shell::WindowElement| {
            w.wl_surface().is_some_and(|s| {
                smithay::backend::renderer::utils::with_renderer_surface_state(&s, |state| state.buffer().is_some())
                    .unwrap_or(false)
            })
        };
        let ready = self.space.elements().any(drawn)
            || (!crate::session::launch().greeter
                && smithay::desktop::layer_map_for_output(output).layers().next().is_some())
            || since.elapsed() >= HOLD_FIRST_FRAME;
        if ready {
            SHOWN.store(true, Ordering::Relaxed);
        }
        ready
    }
}
