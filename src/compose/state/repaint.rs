// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::{AnvilState, Backend, DndIcon};

use std::{collections::HashMap, time::Duration};

use smithay::{
    backend::renderer::element::{
        utils::select_dmabuf_feedback, RenderElementState, RenderElementStates,
    },
    desktop::{
        utils::{
            send_frames_surface_tree, surface_presentation_feedback_flags_from_states,
            surface_primary_scanout_output, update_surface_primary_scanout_output,
            with_surfaces_surface_tree, OutputPresentationFeedback,
        },
        Space,
    },
    input::pointer::CursorImageStatus, output::Output,
    reexports::wayland_server::{
        backend::ClientId, protocol::wl_surface::WlSurface, Client, Resource,
    },
    utils::{Monotonic, Time},
    wayland::{
        commit_timing::CommitTimerBarrierStateUserData,
        compositor::{CompositorHandler, SurfaceData},
        dmabuf::DmabufFeedback, fifo::FifoBarrierCachedState,
        fractional_scale::with_fractional_scale,
    },
};

use crate::shell::WindowElement;

pub fn prefers_next_output(current_area: usize, next_area: usize, current_refresh: i32, next_refresh: i32) -> bool {
    if next_area > current_area.saturating_mul(2) {
        return true;
    }
    if current_area > next_area.saturating_mul(2) {
        return false;
    }
    next_refresh > current_refresh
}

pub fn steady_primary_scanout_output_compare<'a>(
    current_output: &'a Output,
    current_state: &RenderElementState,
    next_output: &'a Output,
    next_state: &RenderElementState,
) -> &'a Output {
    let refresh = |o: &Output| o.current_mode().map_or(0, |m| m.refresh);
    if prefers_next_output(current_state.visible_area, next_state.visible_area, refresh(current_output), refresh(next_output)) {
        next_output
    } else {
        current_output
    }
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn refresh_screens(&mut self) {
        crate::protocols::screens::refresh(self);
    }

    pub fn pre_repaint(&mut self, output: &Output, frame_target: impl Into<Time<Monotonic>>) {
        let frame_target = frame_target.into();

        let mut clients = Waiting::default();
        self.space.elements().for_each(|window| {
            window.with_surfaces(|surface, states| signal_commit_timer(surface, states, frame_target, &mut clients));
        });

        let map = smithay::desktop::layer_map_for_output(output);
        for layer_surface in map.layers() {
            layer_surface.with_surfaces(|surface, states| signal_commit_timer(surface, states, frame_target, &mut clients));
        }

        std::mem::drop(map);

        if let CursorImageStatus::Surface(ref surface) = self.cursor_status {
            with_surfaces_surface_tree(surface, |surface, states| signal_commit_timer(surface, states, frame_target, &mut clients));
        }

        if let Some(surface) = self.dnd_icon.as_ref().map(|icon| &icon.surface) {
            with_surfaces_surface_tree(surface, |surface, states| signal_commit_timer(surface, states, frame_target, &mut clients));
        }

        self.clear_blockers(clients);
    }

    fn clear_blockers(&mut self, clients: Waiting) {
        let dh = self.display_handle.clone();
        for client in clients.into_clients() {
            self.client_compositor_state(&client).blocker_cleared(self, &dh);
        }
    }

    fn covered(&self, output: &Output) -> bool {
        const GAP: i32 = 16;
        let Some(screen) = self.space.output_geometry(output) else { return false };
        let windows = self.space.elements().filter_map(|w| self.space.element_geometry(w));
        screen.subtract_rects(windows).iter().all(|r| r.size.w.min(r.size.h) <= GAP)
    }

    pub fn post_repaint(
        &mut self,
        output: &Output,
        time: impl Into<Duration>,
        dmabuf_feedback: Option<SurfaceDmabufFeedback>,
        render_element_states: &RenderElementStates,
    ) {
        let shown = Shown { output, time: time.into(), dmabuf_feedback: dmabuf_feedback.as_ref(), render_element_states };

        let mut clients = Waiting::default();

        self.windows_shown(shown, &mut clients);
        let backdrop_hidden = self.is_locked() || self.covered(output);
        self.panels_shown(shown, backdrop_hidden, &mut clients);

        if let Some(surface) = self.session_lock.surface_on(output) {
            send_frames_surface_tree(surface, output, shown.time, THROTTLE, |_, _| Some(output.clone()));
        }

        if let CursorImageStatus::Surface(ref surface) = self.cursor_status {
            with_surfaces_surface_tree(surface, |surface, states| settle_surface(output, surface, states, &mut clients));
        }

        if let Some(surface) = self.dnd_icon.as_ref().map(|icon| &icon.surface) {
            with_surfaces_surface_tree(surface, |surface, states| settle_surface(output, surface, states, &mut clients));
        }

        self.clear_blockers(clients);
    }

    fn windows_shown(&self, shown: Shown, clients: &mut Waiting) {
        let Shown { output, time, dmabuf_feedback, render_element_states } = shown;
        self.space.elements().for_each(|window| {
            window.with_surfaces(|surface, states| settle_surface(output, surface, states, clients));

            if self.space.outputs_for_element(window).contains(output) {
                window.send_frame(output, time, THROTTLE, surface_primary_scanout_output);
                if let Some(dmabuf_feedback) = dmabuf_feedback {
                    window.send_dmabuf_feedback(output, surface_primary_scanout_output, |surface, _| {
                        select_dmabuf_feedback(
                            surface,
                            render_element_states,
                            &dmabuf_feedback.render_feedback,
                            &dmabuf_feedback.scanout_feedback,
                        )
                    });
                }
            }
        });
    }

    fn panels_shown(&self, shown: Shown, backdrop_hidden: bool, clients: &mut Waiting) {
        let Shown { output, time, dmabuf_feedback, render_element_states } = shown;
        let map = smithay::desktop::layer_map_for_output(output);
        for layer_surface in map.layers() {
            layer_surface.with_surfaces(|surface, states| settle_surface(output, surface, states, clients));

            let unseen = backdrop_hidden && layer_surface.layer() == smithay::wayland::shell::wlr_layer::Layer::Background;
            if !unseen {
                layer_surface.send_frame(output, time, THROTTLE, |_, _| Some(output.clone()));
            }
            if let Some(dmabuf_feedback) = dmabuf_feedback {
                layer_surface.send_dmabuf_feedback(output, surface_primary_scanout_output, |surface, _| {
                    select_dmabuf_feedback(
                        surface,
                        render_element_states,
                        &dmabuf_feedback.render_feedback,
                        &dmabuf_feedback.scanout_feedback,
                    )
                });
            }
        }

        std::mem::drop(map);
    }
}

#[allow(clippy::mutable_key_type)]
#[derive(Default)]
struct Waiting(HashMap<ClientId, Client>);

impl Waiting {
    fn add(&mut self, client: Client) {
        self.0.insert(client.id(), client);
    }

    fn into_clients(self) -> impl Iterator<Item = Client> {
        self.0.into_values()
    }
}

const THROTTLE: Option<Duration> = Some(Duration::from_secs(1));

#[derive(Clone, Copy)]
struct Shown<'a> {
    output: &'a Output,
    time: Duration,
    dmabuf_feedback: Option<&'a SurfaceDmabufFeedback>,
    render_element_states: &'a RenderElementStates,
}

fn signal_commit_timer(surface: &WlSurface, states: &SurfaceData, frame_target: Time<Monotonic>, clients: &mut Waiting) {
    if let Some(mut commit_timer_state) = states
        .data_map
        .get::<CommitTimerBarrierStateUserData>()
        .map(|commit_timer| commit_timer.lock().unwrap())
    {
        commit_timer_state.signal_until(frame_target);
        let client = surface.client().unwrap();
        clients.add(client);
    }
}

fn settle_surface(output: &Output, surface: &WlSurface, states: &SurfaceData, clients: &mut Waiting) {
    let primary_scanout_output = surface_primary_scanout_output(surface, states);

    if let Some(output) = primary_scanout_output.as_ref() {
        with_fractional_scale(states, |fraction_scale| {
            fraction_scale.set_preferred_scale(output.current_scale().fractional_scale());
        });
    }

    if primary_scanout_output
        .as_ref()
        .map(|o| o == output)
        .unwrap_or(true)
    {
        let fifo_barrier = states
            .cached_state
            .get::<FifoBarrierCachedState>()
            .current()
            .barrier
            .take();

        if let Some(fifo_barrier) = fifo_barrier {
            fifo_barrier.signal();
            let client = surface.client().unwrap();
            clients.add(client);
        }
    }
}

pub fn update_primary_scanout_output(
    space: &Space<WindowElement>,
    output: &Output,
    dnd_icon: &Option<DndIcon>,
    cursor_status: &CursorImageStatus,
    render_element_states: &RenderElementStates,
) {
    space.elements().for_each(|window| {
        window.with_surfaces(|surface, states| {
            update_surface_primary_scanout_output(
                surface,
                output,
                states,
                render_element_states,
                steady_primary_scanout_output_compare,
            );
        });
    });
    let map = smithay::desktop::layer_map_for_output(output);
    for layer_surface in map.layers() {
        layer_surface.with_surfaces(|surface, states| {
            update_surface_primary_scanout_output(
                surface,
                output,
                states,
                render_element_states,
                steady_primary_scanout_output_compare,
            );
        });
    }

    if let CursorImageStatus::Surface(ref surface) = cursor_status {
        with_surfaces_surface_tree(surface, |surface, states| {
            update_surface_primary_scanout_output(
                surface,
                output,
                states,
                render_element_states,
                steady_primary_scanout_output_compare,
            );
        });
    }

    if let Some(surface) = dnd_icon.as_ref().map(|icon| &icon.surface) {
        with_surfaces_surface_tree(surface, |surface, states| {
            update_surface_primary_scanout_output(
                surface,
                output,
                states,
                render_element_states,
                steady_primary_scanout_output_compare,
            );
        });
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceDmabufFeedback {
    pub render_feedback: DmabufFeedback,
    pub scanout_feedback: DmabufFeedback,
}

pub fn take_presentation_feedback(
    output: &Output,
    space: &Space<WindowElement>,
    render_element_states: &RenderElementStates,
) -> OutputPresentationFeedback {
    let mut output_presentation_feedback = OutputPresentationFeedback::new(output);

    space.elements().for_each(|window| {
        if space.outputs_for_element(window).contains(output) {
            window.take_presentation_feedback(
                &mut output_presentation_feedback,
                surface_primary_scanout_output,
                |surface, _| surface_presentation_feedback_flags_from_states(surface, render_element_states),
            );
        }
    });
    let map = smithay::desktop::layer_map_for_output(output);
    for layer_surface in map.layers() {
        layer_surface.take_presentation_feedback(
            &mut output_presentation_feedback,
            surface_primary_scanout_output,
            |surface, _| surface_presentation_feedback_flags_from_states(surface, render_element_states),
        );
    }

    output_presentation_feedback
}

#[cfg(test)]
mod steady_screen_tests {
    use super::prefers_next_output;

    #[test]
    fn a_window_on_two_screens_settles_on_one_whatever_the_areas_and_refresh_rates() {
        for (a_area, a_hz, b_area, b_hz) in [
            (3000, 59_999, 1000, 60_000), (1000, 59_999, 3000, 60_000), (2000, 60_000, 1000, 144_000),
            (1000, 60_000, 1000, 59_999), (1000, 59_999, 1000, 60_000), (500, 60_000, 4000, 30_000), (0, 60_000, 0, 60_000),
        ] {
            let mut on = 'a';
            let mut flips = 0;
            for _ in 0..6 {
                let next = if on == 'a' { 'b' } else { 'a' };
                let (ca, cr, na, nr) = if on == 'a' { (a_area, a_hz, b_area, b_hz) } else { (b_area, b_hz, a_area, a_hz) };
                if prefers_next_output(ca, na, cr, nr) {
                    on = next;
                    flips += 1;
                }
            }
            assert!(flips <= 1, "{a_area}/{a_hz} against {b_area}/{b_hz} flipped {flips} times");
        }
    }

    #[test]
    fn a_much_larger_view_wins_and_alike_views_go_to_the_faster_screen() {
        assert!(prefers_next_output(100, 300, 60_000, 30_000));
        assert!(!prefers_next_output(300, 100, 30_000, 144_000));
        assert!(prefers_next_output(100, 100, 60_000, 144_000));
        assert!(!prefers_next_output(100, 100, 144_000, 60_000));
    }
}
