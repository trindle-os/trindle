//! trindle-lock: ext-session-lock-v1 lock screen; PIN for a limited time, else PAM password.
//! The compositor holds the lock, so a crash leaves the screen locked. Returns once
//! locked, so a suspend never happens first.

mod events;
mod input;
mod keyboard;
mod layout;
mod pam;
mod pen;
mod picture;
mod reader_events;
mod reveal;
pub(super) mod rest;
pub(super) mod run;
mod widgets;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::ffi::{c_int, c_void};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use wayland_client::protocol::{wl_buffer, wl_compositor, wl_output, wl_shm, wl_surface};
use wayland_client::QueueHandle;
use wayland_protocols::ext::session_lock::v1::client::{
    ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,
    ext_session_lock_v1::ExtSessionLockV1,
};
use zeroize::{Zeroize, Zeroizing};

use crate::common::{pin, shm};
use crate::toolkit::t;
use crate::toolkit::text::Fonts;

pub(crate) use pam::password_ok;
use reveal::Eye;
use rest::{rest_step, Face};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mode {
    Pin,
    Password,
}

struct Screen {
    output: wl_output::WlOutput,
    scale: i32,
    surface: Option<wl_surface::WlSurface>,
    lock_surface: Option<ExtSessionLockSurfaceV1>,
    size: (i32, i32),

    kept: Vec<Kept>,
}

struct Kept {
    memory: shm::Memory,
    buffer: wl_buffer::WlBuffer,

    size: (i32, i32),

    busy: Arc<AtomicBool>,

    clock: Option<[i32; 4]>,
}

const HINT: Duration = Duration::from_secs(3);

const TURN: Duration = Duration::from_millis(30);

struct State {
    compositor: wl_compositor::WlCompositor,
    shm: wl_shm::WlShm,
    lock: Option<ExtSessionLockV1>,
    screens: Vec<Screen>,
    locked: bool,
    refused: bool,
    mode: Mode,
    entered: Zeroizing<String>,
    reader: reader_events::Reader,
    eye: Eye,
    message: String,
    wrong: bool,
    tries: u32,
    locked_at: SystemTime,
    checking: bool,

    check_since: Option<Instant>,
    check_turn: u8,
    done: bool,
    pointer_on: Option<wl_surface::WlSurface>,
    pointer_at: (f64, f64),

    flash: Option<(char, Instant)>,
    cursor: Option<(wl_surface::WlSurface, wl_buffer::WlBuffer)>,
    xkb_ctx: *mut c_void,
    xkb_state: *mut c_void,
    minute: i32,

    resting: Option<Instant>,

    rest_pointer: Option<(f64, f64)>,

    phone: bool,

    due: Option<u32>,

    phone_hint: Option<Instant>,
    fonts: Fonts,

    moved: Instant,

    faces: Vec<Face>,

    old: Vec<Kept>,
}

impl State {

    fn begin_check(&mut self) -> u8 {
        self.checking = true;
        self.check_since = Some(Instant::now());
        self.check_turn = self.check_turn.wrapping_add(1);
        self.check_turn
    }

    fn abandon_check(&mut self) {
        self.checking = false;
        self.check_since = None;
        self.check_turn = self.check_turn.wrapping_add(1);
        self.message = t!("session-lock-check-abandoned").into();
        self.wrong = true;
    }

    fn moves_every(&self) -> Option<Duration> {
        if self.checking {
            Some(TURN)
        } else {
            self.resting.map(|_| rest_step())
        }
    }

    fn pin_allowed(&self) -> bool {
        pin::is_set() && self.tries < pin::TRIES
            && self.locked_at.elapsed().map(|e| e.as_secs() < pin::minutes() as u64 * 60).unwrap_or(false)
    }

    fn hint_showing(&self) -> bool {
        self.phone && self.phone_hint.is_some_and(|at| at.elapsed() < HINT)
    }

    fn ask_for_password(&mut self, why: &str) {
        self.mode = Mode::Password;
        self.entered.zeroize();
        self.eye.shut();
        self.message = why.to_string();
        self.wrong = false;
    }
}

fn unlock(state: &mut State) {
    if let Some(lock) = state.lock.take() {
        lock.unlock_and_destroy();
    }
    state.reader.unlocked();
    state.done = true;
}

thread_local! {

    static CHECKED: RefCell<Option<(c_int, c_int)>> = const { RefCell::new(None) };
}

fn add_screen(state: &mut State, qh: &QueueHandle<State>, output: wl_output::WlOutput) {
    let index = state.screens.len();
    state.screens.push(Screen { output, scale: 1, surface: None, lock_surface: None, size: (0, 0), kept: Vec::new() });
    if state.lock.is_some() {
        give_surface(state, qh, index);
    }
}

fn give_surface(state: &mut State, qh: &QueueHandle<State>, index: usize) {
    let Some(lock) = state.lock.clone() else { return };
    let surface = state.compositor.create_surface(qh, ());
    let ls = lock.get_lock_surface(&surface, &state.screens[index].output, qh, index);
    state.screens[index].surface = Some(surface);
    state.screens[index].lock_surface = Some(ls);
}
