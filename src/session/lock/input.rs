//! What the person's keys, clicks and touches do, and the checks that follow.

use std::ffi::c_void;
use std::time::{Duration, Instant};

use wayland_client::QueueHandle;
use wayland_client::protocol::wl_surface;
use zeroize::Zeroize;

use crate::common::{pin, pocketkey};
use crate::toolkit::t;

use super::{unlock, CHECKED, Mode, State};
use super::keyboard::Key;
use super::layout::{eye_button, keypad, phone_button, switch_button, EYE_REACH, KEY_R};
use super::pam::{password_ok, phone_ok};
use super::picture::redraw;

pub(super) fn wake(state: &mut State, qh: &QueueHandle<State>) {
    state.resting = None;
    state.eye.shut();
    (state.phone, state.due) = pocketkey::screen();
    state.phone_hint = Some(Instant::now());
    redraw(state, qh);
}

pub(super) fn typed(state: &mut State, qh: &QueueHandle<State>, key: Key) {
    handle_key(state, qh, key);
    if !state.done {
        state.reader.entry(state.entered.chars().count());
    }
}

fn handle_key(state: &mut State, qh: &QueueHandle<State>, key: Key) {

    if state.checking && key == Key::Back && !state.done {
        state.abandon_check();
        return redraw(state, qh);
    }
    if state.checking || state.done {
        return;
    }
    if state.resting.is_some() {
        return wake(state, qh);
    }

    if state.mode == Mode::Pin && !state.pin_allowed() {
        state.ask_for_password(t!("session-lock-pin-timed-out"));
    }

    if state.eye.key(key, Instant::now()) {
        return redraw(state, qh);
    }
    light_key(state, key);
    match (state.mode, key) {
        (Mode::Pin, Key::Char(c)) if c.is_ascii_digit() => {
            if pin_digit(state, c) {
                return;
            }
        }
        (Mode::Pin, Key::Erase) => {
            state.entered.pop();
        }
        (Mode::Pin, Key::ToPassword) => {
            state.ask_for_password("");
            state.phone_hint = Some(Instant::now());
        }

        (_, Key::Phone) | (_, Key::Enter) if state.phone && (key == Key::Phone || state.entered.is_empty()) => ask_phone(state),
        (Mode::Password, Key::Char(c)) if !c.is_control() => {
            state.entered.push(c);
            state.message.clear();
        }
        (Mode::Password, Key::Erase) => {
            state.entered.pop();
        }
        (Mode::Password, Key::Back) if state.pin_allowed() => {
            state.entered.zeroize();
            state.eye.shut();
            state.mode = Mode::Pin;
            state.message.clear();
            state.phone_hint = Some(Instant::now());
        }
        (Mode::Password, Key::Enter) if !state.entered.is_empty() => check_password(state),
        _ => {}
    }
    redraw(state, qh);
}

fn light_key(state: &mut State, key: Key) {
    match (state.mode, key) {
        (Mode::Pin, Key::Char(c)) if c.is_ascii_digit() => state.flash = Some((c, Instant::now())),
        (Mode::Pin, Key::Erase) => state.flash = Some(('<', Instant::now())),
        (_, Key::Phone) => state.flash = Some((PHONE_KEY, Instant::now())),
        _ => {}
    }
}

fn pin_digit(state: &mut State, c: char) -> bool {
    state.entered.push(c);
    state.message.clear();
    if state.entered.len() == pin::LENGTH {
        let good = pin::check(&state.entered);
        state.entered.zeroize();

        state.eye.shut();
        if good {
            unlock(state);
            return true;
        }
        state.tries += 1;
        if state.tries >= pin::TRIES {
            state.ask_for_password(t!("session-lock-too-many-pins"));
        } else {
            state.message = t!("session-lock-wrong-pin", n = pin::TRIES - state.tries);
            state.wrong = true;
            state.reader.wrong();
        }
    }
    false
}

fn ask_phone(state: &mut State) {
    state.entered.zeroize();
    state.eye.shut();
    state.wrong = false;
    state.message = t!("session-lock-asking-phone").into();
    let user = account_name();
    let turn = state.begin_check();
    answer_later(turn, move || if phone_ok(&user) { CHECK_OK } else { CHECK_PHONE_NO });
}

fn check_password(state: &mut State) {
    state.eye.shut();
    let password = std::mem::take(&mut state.entered);
    let user = account_name();
    let turn = state.begin_check();
    answer_later(turn, move || if password_ok(&user, &password) { CHECK_OK } else { CHECK_WRONG });
}

pub(super) fn answer_counts(got: isize, turn: u8, now_running: u8, checking: bool) -> bool {
    got == 2 && checking && turn == now_running
}

pub(super) const CHECK_LIMIT: Duration = Duration::from_secs(20);

pub(super) const CHECK_WRONG: u8 = 0;
pub(super) const CHECK_OK: u8 = 1;
pub(super) const CHECK_PHONE_NO: u8 = 2;

pub(super) const PHONE_KEY: char = 'p';

fn answer_later(turn: u8, check: impl FnOnce() -> u8 + Send + 'static) {
    let tx = CHECKED.with(|c| c.borrow().as_ref().map(|(_, w)| *w));
    let started = std::thread::Builder::new().spawn(move || {
        let byte = [check(), turn];
        if let Some(fd) = tx {
            // SAFETY: one byte from a local array to the writing end of the

            unsafe { libc::write(fd, byte.as_ptr() as *const c_void, 2) };
        }
    });
    if started.is_err() {

        if let Some(fd) = tx {
            let byte = [CHECK_WRONG, turn];
            // SAFETY: as above, two bytes from a local array.
            unsafe { libc::write(fd, byte.as_ptr() as *const c_void, 2) };
        }
    }
}

pub(super) fn account_name() -> String {
    // SAFETY: getpwuid returns null or a record valid until the next call; the name is copied out at once, on the main thread, which makes no other such call.
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        if pw.is_null() {
            return String::new();
        }
        std::ffi::CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
    }
}

pub(super) fn on_phone(state: &State, (px, py): (f64, f64)) -> bool {
    let Some(on) = &state.pointer_on else { return false };
    let Some(screen) = state.screens.iter().find(|s| s.surface.as_ref() == Some(on)) else { return false };
    let (cx, cy) = (screen.size.0 as f64 / 2.0, screen.size.1 as f64 / 2.0);
    let (x, y, r) = phone_button(state.mode, cx, cy, state.pin_allowed());
    state.phone && (px - x).hypot(py - y) <= r
}

pub(super) fn eye_holds(mode: Mode, (cx, cy): (f64, f64), (px, py): (f64, f64)) -> bool {
    let (x, y, r) = eye_button(mode, cx, cy);
    (px - x).hypot(py - y) <= r.max(EYE_REACH)
}

pub(super) fn on_eye(state: &State, at: (f64, f64)) -> bool {
    let Some(on) = &state.pointer_on else { return false };
    let Some(screen) = state.screens.iter().find(|s| s.surface.as_ref() == Some(on)) else { return false };
    eye_holds(state.mode, (screen.size.0 as f64 / 2.0, screen.size.1 as f64 / 2.0), at)
}

pub(super) fn press(state: &mut State, qh: &QueueHandle<State>, on: &wl_surface::WlSurface, (px, py): (f64, f64)) {
    let Some(screen) = state.screens.iter().find(|s| s.surface.as_ref() == Some(on)) else { return };
    let (cx, cy) = (screen.size.0 as f64 / 2.0, screen.size.1 as f64 / 2.0);

    if eye_holds(state.mode, (cx, cy), (px, py)) {
        if !state.checking && !state.done {
            state.eye.press(Instant::now());
            state.eye.on = false;
            redraw(state, qh);
        }
        return;
    }
    let pin_possible = state.pin_allowed();
    if state.phone {
        let (x, y, r) = phone_button(state.mode, cx, cy, pin_possible);
        if (px - x).hypot(py - y) <= r {
            return typed(state, qh, Key::Phone);
        }
    }
    let (bx, by, bw, bh) = switch_button(state.mode, cx, cy, state.phone);

    let shown = state.mode == Mode::Pin || pin_possible;
    if shown && px >= bx && px <= bx + bw && py >= by && py <= by + bh {
        let key = if state.mode == Mode::Pin { Key::ToPassword } else { Key::Back };
        return typed(state, qh, key);
    }
    if state.mode != Mode::Pin {
        return;
    }
    if let Some((k, _, _)) = keypad(cx, cy).into_iter().find(|(_, x, y)| (px - x).hypot(py - y) <= KEY_R) {
        let key = if k == '<' { Key::Erase } else { Key::Char(k) };
        typed(state, qh, key);
    }
}
