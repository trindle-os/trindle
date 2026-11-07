//! Helper functions to ease dealing with surface trees.

use crate::{
    backend::renderer::{
        element::{
            PrimaryScanoutOutput, RenderElementPresentationState, RenderElementState, RenderElementStates,
        },
        utils::{RendererSurfaceState, RendererSurfaceStateUserData},
    },
    desktop::WindowSurfaceType,
    output::{Output, WeakOutput},
    utils::{Logical, Point, Rectangle, Time},
    wayland::{
        compositor::{with_surface_tree_downward, SurfaceAttributes, SurfaceData, TraversalAction},
        dmabuf::{DmabufFeedback, SurfaceDmabufFeedbackState},
        presentation::{PresentationFeedbackCachedState, PresentationFeedbackCallback, Refresh},
    },
};
use std::{cell::RefCell, sync::Mutex, time::Duration};
use wayland_protocols::wp::presentation_time::server::wp_presentation_feedback;
use wayland_server::protocol::wl_surface;

pub use super::super::space::wayland::output_update;

impl RendererSurfaceState {
    fn contains_point<P: Into<Point<f64, Logical>>>(&self, attrs: &SurfaceAttributes, point: P) -> bool {
        let point = point.into();
        let size = match self.surface_view.map(|view| view.dst) {
            None => return false,
            Some(size) => size,
        };

        let rect = Rectangle::from_size(size).to_f64();

        if !rect.contains(point) {
            return false;
        }

        if attrs.input_region.is_none() {
            return true;
        }

        attrs
            .input_region
            .as_ref()
            .unwrap()
            .contains(point.to_i32_round())
    }
}

pub fn bbox_from_surface_tree<P>(surface: &wl_surface::WlSurface, location: P) -> Rectangle<i32, Logical>
where
    P: Into<Point<i32, Logical>>,
{
    let location = location.into();
    let mut bounding_box = Rectangle::new(location, (0, 0).into());
    with_surface_tree_downward(
        surface,
        location,
        |_, states, loc: &Point<i32, Logical>| {
            let mut loc = *loc;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(surface_view) = data.and_then(|d| d.lock().unwrap().surface_view) {
                loc += surface_view.offset;
                bounding_box = bounding_box.merge(Rectangle::new(loc, surface_view.dst));

                TraversalAction::DoChildren(loc)
            } else {
                TraversalAction::SkipChildren
            }
        },
        |_, _, _| {},
        |_, _, _| true,
    );
    bounding_box
}

pub fn under_from_surface_tree<P>(
    surface: &wl_surface::WlSurface,
    point: Point<f64, Logical>,
    location: P,
    surface_type: WindowSurfaceType,
) -> Option<(wl_surface::WlSurface, Point<i32, Logical>)>
where
    P: Into<Point<i32, Logical>>,
{
    let found = RefCell::new(None);
    with_surface_tree_downward(
        surface,
        location.into(),
        |_, states, location: &Point<i32, Logical>| {
            let mut location = *location;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(surface_view) = data.and_then(|d| d.lock().unwrap().surface_view) {
                location += surface_view.offset;
                if surface_type.contains(WindowSurfaceType::SUBSURFACE) {
                    TraversalAction::DoChildren(location)
                } else {
                    TraversalAction::SkipChildren
                }
            } else {
                TraversalAction::SkipChildren
            }
        },
        |wl_surface, states, location: &Point<i32, Logical>| {
            let mut location = *location;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(surface_view) = data.and_then(|d| d.lock().unwrap().surface_view) {
                location += surface_view.offset;

                let contains_the_point = data
                    .map(|data| {
                        data.lock()
                            .unwrap()
                            .contains_point(&*states.cached_state.get().current(), point - location.to_f64())
                    })
                    .unwrap_or(false);
                if contains_the_point {
                    *found.borrow_mut() = Some((wl_surface.clone(), location));
                }
            }
        },
        |_, _, _| {
            found.borrow().is_none()
        },
    );
    found.into_inner()
}

type SurfacePrimaryScanoutOutput = Mutex<PrimaryScanoutOutput>;

pub fn with_surfaces_surface_tree<F>(surface: &wl_surface::WlSurface, mut processor: F)
where
    F: FnMut(&wl_surface::WlSurface, &SurfaceData),
{
    with_surface_tree_downward(
        surface,
        (),
        |_, _, _| TraversalAction::DoChildren(()),
        |surface, states, _| processor(surface, states),
        |_, _, _| true,
    )
}

pub fn surface_primary_scanout_output(
    _surface: &wl_surface::WlSurface,
    states: &SurfaceData,
) -> Option<Output> {
    states
        .data_map
        .insert_if_missing_threadsafe(SurfacePrimaryScanoutOutput::default);
    let surface_primary_scanout_output = states.data_map.get::<SurfacePrimaryScanoutOutput>().unwrap();
    surface_primary_scanout_output.lock().unwrap().current_output()
}

pub fn update_surface_primary_scanout_output<F>(
    surface: &wl_surface::WlSurface,
    output: &Output,
    surface_data: &SurfaceData,
    states: &RenderElementStates,
    compare: F,
) -> Option<Output>
where
    F: for<'a> Fn(&'a Output, &'a RenderElementState, &'a Output, &'a RenderElementState) -> &'a Output,
{
    surface_data
        .data_map
        .insert_if_missing_threadsafe(SurfacePrimaryScanoutOutput::default);
    let surface_primary_scanout_output = surface_data
        .data_map
        .get::<SurfacePrimaryScanoutOutput>()
        .unwrap();
    surface_primary_scanout_output
        .lock()
        .unwrap()
        .update_from_render_element_states(surface, output, states, compare)
}

pub fn send_frames_surface_tree<T, F>(
    surface: &wl_surface::WlSurface,
    output: &Output,
    time: T,
    throttle: Option<Duration>,
    mut primary_scan_out_output: F,
) where
    T: Into<Duration>,
    F: FnMut(&wl_surface::WlSurface, &SurfaceData) -> Option<Output>,
{
    let time = time.into();

    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |surface, states, &()| {
            states
                .data_map
                .insert_if_missing_threadsafe(SurfaceFrameThrottlingState::default);
            let surface_frame_throttling_state =
                states.data_map.get::<SurfaceFrameThrottlingState>().unwrap();

            let on_primary_scanout_output = primary_scan_out_output(surface, states)
                .map(|preferred_output| preferred_output == *output)
                .unwrap_or(false);

            let frame_overdue = surface_frame_throttling_state.update(time, throttle);

            let send_frame_callback = on_primary_scanout_output || frame_overdue;

            if send_frame_callback {
                for callback in states
                    .cached_state
                    .get::<SurfaceAttributes>()
                    .current()
                    .frame_callbacks
                    .drain(..)
                {
                    callback.done(time.as_millis() as u32);
                }
            }
        },
        |_, _, &()| true,
    );
}

pub fn send_dmabuf_feedback_surface_tree<'a, P, F>(
    surface: &wl_surface::WlSurface,
    output: &Output,
    mut primary_scan_out_output: P,
    select_dmabuf_feedback: F,
) where
    P: FnMut(&wl_surface::WlSurface, &SurfaceData) -> Option<Output>,
    F: Fn(&wl_surface::WlSurface, &SurfaceData) -> &'a DmabufFeedback,
{
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |surface, states, &()| {
            let on_primary_scanout_output = primary_scan_out_output(surface, states)
                .map(|preferred_output| preferred_output == *output)
                .unwrap_or(false);

            if !on_primary_scanout_output {
                return;
            }

            let Some(surface_feedback) = SurfaceDmabufFeedbackState::from_states(states) else {
                return;
            };

            let feedback = select_dmabuf_feedback(surface, states);
            surface_feedback.set_feedback(feedback);
        },
        |_, _, &()| true,
    );
}

#[derive(Debug)]
pub struct SurfacePresentationFeedback {
    callbacks: Vec<PresentationFeedbackCallback>,
    flags: wp_presentation_feedback::Kind,
}

impl SurfacePresentationFeedback {
    pub fn from_states(states: &SurfaceData, flags: wp_presentation_feedback::Kind) -> Option<Self> {
        let mut guard = states.cached_state.get::<PresentationFeedbackCachedState>();
        let presentation_feedback_state = guard.current();
        if presentation_feedback_state.callbacks.is_empty() {
            return None;
        }

        let callbacks = std::mem::take(&mut presentation_feedback_state.callbacks);
        Some(SurfacePresentationFeedback { callbacks, flags })
    }

    pub fn presented(
        &mut self,
        output: &Output,
        clk_id: u32,
        time: impl Into<Duration>,
        refresh: Refresh,
        seq: u64,
        flags: wp_presentation_feedback::Kind,
    ) {
        let time = time.into();
        for callback in self.callbacks.drain(..) {
            if callback.clk_id() == clk_id {
                callback.presented(output, time, refresh, seq, flags | self.flags)
            } else {
                callback.discarded()
            }
        }
    }

    pub fn discarded(&mut self) {
        for callback in self.callbacks.drain(..) {
            callback.discarded()
        }
    }
}

impl Drop for SurfacePresentationFeedback {
    fn drop(&mut self) {
        self.discarded()
    }
}

#[derive(Debug)]
pub struct OutputPresentationFeedback {
    output: WeakOutput,
    callbacks: Vec<SurfacePresentationFeedback>,
}

impl OutputPresentationFeedback {
    pub fn new(output: &Output) -> Self {
        OutputPresentationFeedback {
            output: output.downgrade(),
            callbacks: Vec::new(),
        }
    }

    pub fn output(&self) -> Option<Output> {
        self.output.upgrade()
    }

    pub fn presented<T, Kind>(
        &mut self,
        time: T,
        refresh: Refresh,
        seq: u64,
        flags: wp_presentation_feedback::Kind,
    ) where
        T: Into<Time<Kind>>,
        Kind: crate::utils::NonNegativeClockSource,
    {
        let time = time.into();
        let clk_id = Kind::ID as u32;
        if let Some(output) = self.output.upgrade() {
            for mut callback in self.callbacks.drain(..) {
                callback.presented(&output, clk_id, time, refresh, seq, flags);
            }
        } else {
            self.discarded();
        }
    }

    pub fn discarded(&mut self) {
        for mut callback in self.callbacks.drain(..) {
            callback.discarded();
        }
    }
}

pub fn take_presentation_feedback_surface_tree<F1, F2>(
    surface: &wl_surface::WlSurface,
    output_feedback: &mut OutputPresentationFeedback,
    mut primary_scan_out_output: F1,
    mut presentation_feedback_flags: F2,
) where
    F1: FnMut(&wl_surface::WlSurface, &SurfaceData) -> Option<Output>,
    F2: FnMut(&wl_surface::WlSurface, &SurfaceData) -> wp_presentation_feedback::Kind,
{
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |surface, states, &()| {
            let on_primary_scanout_output = primary_scan_out_output(surface, states)
                .map(|preferred_output| preferred_output == output_feedback.output)
                .unwrap_or(false);

            if !on_primary_scanout_output {
                return;
            }

            let flags = presentation_feedback_flags(surface, states);
            if let Some(feedback) = SurfacePresentationFeedback::from_states(states, flags) {
                output_feedback.callbacks.push(feedback);
            }
        },
        |_, _, &()| true,
    );
}

pub fn surface_presentation_feedback_flags_from_states(
    surface: &wl_surface::WlSurface,
    states: &RenderElementStates,
) -> wp_presentation_feedback::Kind {
    let zero_copy = states
        .element_render_state(surface)
        .map(|state| state.presentation_state == RenderElementPresentationState::ZeroCopy)
        .unwrap_or(false);

    if zero_copy {
        wp_presentation_feedback::Kind::ZeroCopy
    } else {
        wp_presentation_feedback::Kind::empty()
    }
}

#[derive(Debug, Default)]
struct SurfaceFrameThrottlingState(Mutex<Option<Duration>>);

impl SurfaceFrameThrottlingState {
    pub fn update(&self, time: Duration, throttle: Option<Duration>) -> bool {
        if let Some(throttle) = throttle {
            let mut guard = self.0.lock().unwrap();
            let send_throttled_frame = guard
                .map(|last| time.saturating_sub(last) > throttle)
                .unwrap_or(true);
            if send_throttled_frame {
                *guard = Some(time);
            }
            send_throttled_frame
        } else {
            false
        }
    }
}
