//! What the compositor tells the lock screen.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Instant, SystemTime};

use wayland_client::{Connection, Dispatch, QueueHandle, WEnum};
use wayland_client::globals::GlobalListContents;
use wayland_client::protocol::{wl_buffer, wl_compositor, wl_output, wl_pointer, wl_registry, wl_seat, wl_shm, wl_shm_pool, wl_surface, wl_touch};
use wayland_protocols::ext::session_lock::v1::client::{ext_session_lock_surface_v1, ext_session_lock_v1};
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_v1::ExtSessionLockV1;

use crate::common::shm;

use super::{add_screen, State};
use super::input::{on_eye, on_phone, press, wake};
use super::picture::redraw;

const BTN_LEFT: u32 = 0x110;

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            if interface == "wl_output" {
                let o = registry.bind::<wl_output::WlOutput, _, _>(name, version.min(4), qh, ());
                add_screen(state, qh, o);
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for State {
    fn event(state: &mut Self, output: &wl_output::WlOutput, event: wl_output::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let wl_output::Event::Scale { factor } = event {
            if let Some(s) = state.screens.iter_mut().find(|s| &s.output == output) {
                s.scale = factor.max(1);
            }
        }
    }
}

impl Dispatch<ExtSessionLockV1, ()> for State {
    fn event(state: &mut Self, _: &ExtSessionLockV1, event: ext_session_lock_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            ext_session_lock_v1::Event::Locked => {
                state.locked = true;
                state.locked_at = SystemTime::now();
                state.reader.locked();
            }
            ext_session_lock_v1::Event::Finished => {
                if !state.locked {
                    state.refused = true;
                }
                state.done = true;
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtSessionLockSurfaceV1, usize> for State {
    fn event(state: &mut Self, ls: &ExtSessionLockSurfaceV1, event: ext_session_lock_surface_v1::Event, index: &usize,
             _: &Connection, qh: &QueueHandle<Self>) {
        if let ext_session_lock_surface_v1::Event::Configure { serial, width, height } = event {
            ls.ack_configure(serial);
            if let Some(s) = state.screens.get_mut(*index) {
                s.size = (width as i32, height as i32);
            }
            redraw(state, qh);
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(_: &mut Self, seat: &wl_seat::WlSeat, event: wl_seat::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_seat::Event::Capabilities { capabilities: WEnum::Value(caps) } = event {
            if caps.contains(wl_seat::Capability::Pointer) {
                seat.get_pointer(qh, ());
            }
            if caps.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }
            if caps.contains(wl_seat::Capability::Touch) {
                seat.get_touch(qh, ());
            }
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for State {
    fn event(state: &mut Self, pointer: &wl_pointer::WlPointer, event: wl_pointer::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        match event {
            wl_pointer::Event::Enter { serial, surface, surface_x, surface_y } => {
                state.pointer_on = Some(surface);
                state.pointer_at = (surface_x, surface_y);
                if let Some((c, _)) = &state.cursor {
                    pointer.set_cursor(serial, Some(c), 1, 1);
                }
            }
            wl_pointer::Event::Motion { surface_x, surface_y, .. } => {
                let was_on_phone = on_phone(state, state.pointer_at);
                let was_on_eye = on_eye(state, state.pointer_at);
                state.pointer_at = (surface_x, surface_y);
                if state.resting.is_none() && !was_on_eye && on_eye(state, state.pointer_at) {
                    state.eye.named = Some(Instant::now());
                    redraw(state, qh);
                }
                if state.resting.is_none() && !was_on_phone && on_phone(state, state.pointer_at) {
                    state.phone_hint = Some(Instant::now());
                    redraw(state, qh);
                }
                if state.resting.is_some() {
                    let first = *state.rest_pointer.get_or_insert((surface_x, surface_y));
                    if (surface_x - first.0).hypot(surface_y - first.1) > 12.0 {
                        wake(state, qh);
                    }
                }
            }
            wl_pointer::Event::Button { state: WEnum::Value(wl_pointer::ButtonState::Pressed), .. }
                if state.resting.is_some() => wake(state, qh),
            wl_pointer::Event::Button { button: BTN_LEFT, state: WEnum::Value(wl_pointer::ButtonState::Pressed), .. } => {
                let Some(on) = state.pointer_on.clone() else { return };
                let at = state.pointer_at;
                press(state, qh, &on, at);
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_touch::WlTouch, ()> for State {
    fn event(state: &mut Self, _: &wl_touch::WlTouch, event: wl_touch::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        if let wl_touch::Event::Down { surface, x, y, .. } = event {
            if state.resting.is_some() {
                return wake(state, qh);
            }
            press(state, qh, &surface, (x, y));
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, Arc<AtomicBool>> for State {
    fn event(_: &mut Self, _: &wl_buffer::WlBuffer, event: wl_buffer::Event, busy: &Arc<AtomicBool>,
             _: &Connection, _: &QueueHandle<Self>) {
        if let wl_buffer::Event::Release = event {
            busy.store(false, Ordering::Relaxed);
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, shm::Held> for State {
    fn event(_: &mut Self, buffer: &wl_buffer::WlBuffer, event: wl_buffer::Event, frame: &shm::Held,
             _: &Connection, _: &QueueHandle<Self>) {
        if let wl_buffer::Event::Release = event {
            shm::released(buffer, frame);
        }
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for State {
            fn event(_: &mut Self, _: &$t, _: <$t as wayland_client::Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
        }
    )*};
}
ignore!(wl_compositor::WlCompositor, wl_shm::WlShm, wl_shm_pool::WlShmPool, wl_surface::WlSurface, ExtSessionLockManagerV1);
