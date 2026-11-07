//! Utilities for handling the `wp_presentation` protocol.

use std::time::Duration;

use wayland_protocols::wp::presentation_time::server::{wp_presentation, wp_presentation_feedback};
use wayland_server::{
    backend::GlobalId, protocol::wl_surface, Dispatch, DisplayHandle, GlobalDispatch, Resource, Weak,
};

use crate::output::Output;

use super::compositor::{with_states, Cacheable};

const EVT_PRESENTED_VARIABLE_SINCE: u32 = 2;

#[derive(Debug)]
pub struct PresentationState {
    global: GlobalId,
}

impl PresentationState {
    pub fn new<D>(display: &DisplayHandle, clk_id: u32) -> Self
    where
        D: GlobalDispatch<wp_presentation::WpPresentation, u32>
            + Dispatch<wp_presentation::WpPresentation, u32>
            + Dispatch<wp_presentation_feedback::WpPresentationFeedback, ()>
            + 'static,
    {
        PresentationState {
            global: display.create_global::<D, wp_presentation::WpPresentation, u32>(2, clk_id),
        }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<wp_presentation::WpPresentation, u32, D> for PresentationState
where
    D: GlobalDispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation_feedback::WpPresentationFeedback, ()>,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &wayland_server::Client,
        resource: wayland_server::New<wp_presentation::WpPresentation>,
        global_data: &u32,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        let interface = data_init.init(resource, *global_data);
        interface.clock_id(*global_data);
    }
}

impl<D> Dispatch<wp_presentation::WpPresentation, u32, D> for PresentationState
where
    D: GlobalDispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation_feedback::WpPresentationFeedback, ()>,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &wp_presentation::WpPresentation,
        request: <wp_presentation::WpPresentation as Resource>::Request,
        data: &u32,
        _dhandle: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_presentation::Request::Feedback { surface, callback } => {
                let callback = data_init.init(callback, ());

                with_states(&surface, |states| {
                    states
                        .cached_state
                        .get::<PresentationFeedbackCachedState>()
                        .pending()
                        .add_callback(&surface, *data, callback);
                });
            }
            wp_presentation::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<wp_presentation_feedback::WpPresentationFeedback, (), D> for PresentationFeedbackState
where
    D: GlobalDispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation::WpPresentation, u32>,
    D: Dispatch<wp_presentation_feedback::WpPresentationFeedback, ()>,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &wp_presentation_feedback::WpPresentationFeedback,
        _request: <wp_presentation_feedback::WpPresentationFeedback as Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
    }
}

#[derive(Debug)]
pub struct PresentationFeedbackState;

#[derive(Debug)]
pub struct PresentationFeedbackCallback {
    surface: Weak<wl_surface::WlSurface>,
    clk_id: u32,
    callback: wp_presentation_feedback::WpPresentationFeedback,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Refresh {
    Unknown,
    Variable(Duration),
    Fixed(Duration),
}

impl Refresh {
    pub fn variable(min: impl Into<Duration>) -> Self {
        Self::Variable(min.into())
    }

    pub fn fixed(rate: impl Into<Duration>) -> Self {
        Self::Fixed(rate.into())
    }
}

impl PresentationFeedbackCallback {
    pub fn clk_id(&self) -> u32 {
        self.clk_id
    }

    pub fn presented(
        self,
        output: &Output,
        time: impl Into<Duration>,
        refresh: Refresh,
        seq: u64,
        flags: wp_presentation_feedback::Kind,
    ) {
        let surface = match self.surface.upgrade() {
            Ok(surface) => surface,
            Err(_) => {
                self.discarded();
                return;
            }
        };

        let client = match surface.client() {
            Some(client) => client,
            None => return,
        };

        for output in output.client_outputs(&client) {
            self.callback.sync_output(&output);
        }

        let refresh = match refresh {
            Refresh::Fixed(duration) => duration,
            Refresh::Variable(duration) if self.callback.version() >= EVT_PRESENTED_VARIABLE_SINCE => {
                duration
            }
            _ => Duration::ZERO,
        };

        let time = time.into();
        let tv_sec_hi = (time.as_secs() >> 32) as u32;
        let tv_sec_lo = (time.as_secs() & 0xFFFFFFFF) as u32;
        let tv_nsec = time.subsec_nanos();
        let refresh = refresh.as_nanos() as u32;
        let seq_hi = (seq >> 32) as u32;
        let seq_lo = (seq & 0xFFFFFFFF) as u32;

        self.callback
            .presented(tv_sec_hi, tv_sec_lo, tv_nsec, refresh, seq_hi, seq_lo, flags);
    }

    pub fn discarded(self) {
        self.callback.discarded()
    }
}

#[derive(Debug, Default)]
pub struct PresentationFeedbackCachedState {
    pub callbacks: Vec<PresentationFeedbackCallback>,
}

impl PresentationFeedbackCachedState {
    fn add_callback(
        &mut self,
        surface: &wl_surface::WlSurface,
        clk_id: u32,
        callback: wp_presentation_feedback::WpPresentationFeedback,
    ) {
        self.callbacks.push(PresentationFeedbackCallback {
            surface: surface.downgrade(),
            clk_id,
            callback,
        });
    }
}

impl Cacheable for PresentationFeedbackCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        PresentationFeedbackCachedState {
            callbacks: std::mem::take(&mut self.callbacks),
        }
    }

    fn merge_into(mut self, into: &mut Self, _dh: &DisplayHandle) {
        if self.callbacks.is_empty() {
            return;
        }

        for callback in std::mem::replace(&mut into.callbacks, std::mem::take(&mut self.callbacks)) {
            callback.discarded();
        }
    }
}

impl Drop for PresentationFeedbackCachedState {
    fn drop(&mut self) {
        for callback in self.callbacks.drain(..) {
            callback.discarded();
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_presentation {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::presentation_time::server::wp_presentation::WpPresentation: u32
        ] => $crate::wayland::presentation::PresentationState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::presentation_time::server::wp_presentation::WpPresentation: u32
        ] => $crate::wayland::presentation::PresentationState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::presentation_time::server::wp_presentation_feedback::WpPresentationFeedback: ()
        ] => $crate::wayland::presentation::PresentationFeedbackState);
    };
}
