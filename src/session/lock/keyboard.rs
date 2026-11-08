//! The keyboard: keys read through xkbcommon, and what the lock screen takes them to mean.

use std::ffi::{c_char, c_int, c_void, CStr};
use std::os::fd::AsRawFd;

use wayland_client::{Connection, Dispatch, QueueHandle, WEnum};
use wayland_client::protocol::wl_keyboard;

use super::State;
use super::input::typed;

#[link(name = "libxkbcommon.so.0", kind = "dylib", modifiers = "+verbatim")]
extern "C" {
    fn xkb_context_new(flags: c_int) -> *mut c_void;
    fn xkb_keymap_new_from_string(ctx: *mut c_void, text: *const c_char, format: c_int, flags: c_int) -> *mut c_void;
    fn xkb_keymap_unref(keymap: *mut c_void);
    fn xkb_state_new(keymap: *mut c_void) -> *mut c_void;
    fn xkb_state_unref(state: *mut c_void);
    fn xkb_state_update_mask(state: *mut c_void, dep: u32, lat: u32, lock: u32, dg: u32, lg: u32, lockg: u32) -> c_int;
    fn xkb_state_key_get_one_sym(state: *mut c_void, key: u32) -> u32;
    fn xkb_state_key_get_utf8(state: *mut c_void, key: u32, buf: *mut c_char, size: usize) -> c_int;
}

pub(super) fn new_context() -> *mut c_void {
    // SAFETY: makes a new context and hands over the only reference to it.
    unsafe { xkb_context_new(0) }
}

const KEY_RETURN: u32 = 0xff0d;
const KEY_KP_ENTER: u32 = 0xff8d;
const KEY_BACKSPACE: u32 = 0xff08;
const KEY_ESCAPE: u32 = 0xff1b;
const KEY_TAB: u32 = 0xff09;
const KEY_BACK_TAB: u32 = 0xfe20;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Key {
    Char(char),
    Erase,
    Enter,
    Back,
    ToPassword,
    Phone,

    Tab,
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(state: &mut Self, _: &wl_keyboard::WlKeyboard, event: wl_keyboard::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        match event {
            wl_keyboard::Event::Keymap { format: WEnum::Value(wl_keyboard::KeymapFormat::XkbV1), fd, size } => load_keymap(state, fd.as_raw_fd(), size as usize),
            wl_keyboard::Event::Modifiers { mods_depressed, mods_latched, mods_locked, group, .. } => {
                if !state.xkb_state.is_null() {
                    // SAFETY: a live xkb state, just checked not to be null,

                    unsafe { xkb_state_update_mask(state.xkb_state, mods_depressed, mods_latched, mods_locked, 0, 0, group) };
                }
            }
            wl_keyboard::Event::Key { key, state: WEnum::Value(wl_keyboard::KeyState::Pressed), .. } => {
                if let Some(k) = key_of(state, key + 8) {
                    typed(state, qh, k);
                }
            }
            _ => {}
        }
    }
}

fn load_keymap(state: &mut State, fd: c_int, size: usize) {
    // SAFETY: the compositor sends a descriptor to a keymap of `size`

    unsafe {
        let map = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ, libc::MAP_PRIVATE, fd, 0);
        if map == libc::MAP_FAILED {
            return;
        }

        let mut text = std::slice::from_raw_parts(map as *const u8, size).to_vec();
        libc::munmap(map, size);
        text.retain(|b| *b != 0);
        text.push(0);
        let keymap = xkb_keymap_new_from_string(state.xkb_ctx, text.as_ptr() as *const c_char, 1, 0);
        if keymap.is_null() {
            return;
        }
        if !state.xkb_state.is_null() {
            xkb_state_unref(state.xkb_state);
        }
        state.xkb_state = xkb_state_new(keymap);
        xkb_keymap_unref(keymap);
    }
}

fn key_of(state: &State, code: u32) -> Option<Key> {
    if state.xkb_state.is_null() {
        return None;
    }
    // SAFETY: a live xkb state, checked not to be null just above.
    let sym = unsafe { xkb_state_key_get_one_sym(state.xkb_state, code) };
    Some(match sym {
        KEY_RETURN | KEY_KP_ENTER => Key::Enter,
        KEY_BACKSPACE => Key::Erase,
        KEY_ESCAPE => Key::Back,
        KEY_TAB | KEY_BACK_TAB => Key::Tab,
        _ => {
            let mut buf = [0 as c_char; 16];
            // SAFETY: a live xkb state, and a local buffer whose

            let n = unsafe { xkb_state_key_get_utf8(state.xkb_state, code, buf.as_mut_ptr(), buf.len()) };
            if n <= 0 {
                return None;
            }
            // SAFETY: xkbcommon NUL-terminated the buffer above.
            let text = unsafe { CStr::from_ptr(buf.as_ptr()) }.to_string_lossy().into_owned();
            Key::Char(text.chars().next_back()?)
        }
    })
}
