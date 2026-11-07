// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! On-screen keyboards (zwp-virtual-keyboard-v1), handled by Smithay with one
//! repair on the way in. wvkbd gives its keymap's length without the closing
//! NUL the protocol counts, and Smithay, trusting the length, cuts off the
//! keymap's last character; the keymap then fails to load and the keyboard is
//! thrown off at its first key. A keymap that doesn't end in NUL is copied
//! with one added before Smithay reads it.
//!
//! Keys are dropped while a trusted window (trusted.rs) has the keyboard, so
//! no program can answer Pocketkey's question by pretending to type. And
//! while a question only the person may answer is out or has the keyboard,
//! or the screen is locked, they are dropped unless the person switched
//! the on-screen keyboard on themselves (`kept_from_a_made_up_keyboard`).
//!
//! A plugin allowed "control" is the one box that sees this global. Its keys
//! go through virtual_input instead, as a second keyboard's would.

use std::{
    fs::File,
    os::{fd::{FromRawFd, OwnedFd}, unix::fs::FileExt},
};

use smithay::{
    reexports::{
        wayland_protocols_misc::zwp_virtual_keyboard_v1::server::{
            zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
            zwp_virtual_keyboard_v1::{self, ZwpVirtualKeyboardV1},
        },
        wayland_server::{self, Client, DataInit, Dispatch, DisplayHandle},
    },
    wayland::seat::WaylandFocus,
    wayland::virtual_keyboard::{
        VirtualKeyboardManagerGlobalData, VirtualKeyboardManagerState, VirtualKeyboardUserData,
    },
};

use crate::state::{AnvilState, Backend};

wayland_server::delegate_global_dispatch!(@<BackendData: Backend + 'static> AnvilState<BackendData>: [
    ZwpVirtualKeyboardManagerV1: VirtualKeyboardManagerGlobalData
] => VirtualKeyboardManagerState);

wayland_server::delegate_dispatch!(@<BackendData: Backend + 'static> AnvilState<BackendData>: [
    ZwpVirtualKeyboardManagerV1: ()
] => VirtualKeyboardManagerState);

impl<BackendData: Backend + 'static> Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<Self>>
    for AnvilState<BackendData>
{
    fn request(
        state: &mut Self,
        client: &Client,
        keyboard: &ZwpVirtualKeyboardV1,
        request: zwp_virtual_keyboard_v1::Request,
        data: &VirtualKeyboardUserData<Self>,
        dh: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        let Some(request) = screened(state, client, request) else { return };
        if holds(&request) && kept_from_a_made_up_keyboard(crate::own_helpers::on(crate::own_helpers::Helper::Keyboard), state.is_locked(), state.only_the_person_answers()) {
            return;
        }
        let typed = matches!(request, zwp_virtual_keyboard_v1::Request::Key { .. }) && !crate::virtual_input::from_box(client);
        let named = if typed && crate::own_helpers::on(crate::own_helpers::Helper::Keyboard) { state.on_screen_key_coming() } else { None };
        <VirtualKeyboardManagerState as Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<Self>, Self>>::request(
            state, client, keyboard, request, data, dh, data_init,
        );
        state.on_screen_key_passed_on(named);
    }
}

fn screened<BackendData: Backend + 'static>(
    state: &mut AnvilState<BackendData>,
    client: &Client,
    request: zwp_virtual_keyboard_v1::Request,
) -> Option<zwp_virtual_keyboard_v1::Request> {
    match request {
        zwp_virtual_keyboard_v1::Request::Keymap { format, fd, size } => {
            let (fd, size) = with_ending(fd, size)?;
            Some(zwp_virtual_keyboard_v1::Request::Keymap { format, fd, size })
        }
        zwp_virtual_keyboard_v1::Request::Key { time, key, state: pressed } if crate::virtual_input::may_control(client) => {
            let key = crate::virtual_input::Key { time, code: key, pressed: pressed == 1 };
            state.virtual_input(crate::virtual_input::maker(client), crate::virtual_input::Did::Key(key));
            None
        }
        zwp_virtual_keyboard_v1::Request::Modifiers { .. } if crate::virtual_input::may_control(client) => None,
        zwp_virtual_keyboard_v1::Request::Key { time, key, state: pressed }
            if !crate::virtual_input::from_box(client) && (state.capture.is_active() || pressed == 0) =>
        {
            let did = crate::capture::Taken::Key { time, code: key, pressed: pressed == 1 };
            if state.captured(did) {
                return None;
            }
            Some(zwp_virtual_keyboard_v1::Request::Key { time, key, state: pressed })
        }
        zwp_virtual_keyboard_v1::Request::Key { .. } | zwp_virtual_keyboard_v1::Request::Modifiers { .. }
            if trusted_has_keyboard(state) =>
        {
            None
        }
        other => Some(other),
    }
}

fn holds(request: &zwp_virtual_keyboard_v1::Request) -> bool {
    match request {
        zwp_virtual_keyboard_v1::Request::Key { state: pressed, .. } => *pressed != 0,
        zwp_virtual_keyboard_v1::Request::Modifiers { mods_depressed, mods_latched, mods_locked, .. } => (mods_depressed | mods_latched | mods_locked) != 0,
        _ => false,
    }
}

pub(crate) fn kept_from_a_made_up_keyboard(switched_on: bool, locked: bool, question: bool) -> bool {
    !switched_on && (locked || question)
}

fn trusted_has_keyboard<BackendData: Backend + 'static>(state: &AnvilState<BackendData>) -> bool {
    state
        .seat
        .get_keyboard()
        .and_then(|k| k.current_focus())
        .is_some_and(|f| f.wl_surface().is_some_and(|s| crate::trusted::is_trusted(&s)))
}

fn with_ending(fd: OwnedFd, size: u32) -> Option<(OwnedFd, u32)> {
    let given = File::from(fd);
    const MOST: u32 = 4 << 20;
    if size == 0 || size > MOST {
        return None;
    }
    let mut text = vec![0u8; size as usize];
    given.read_exact_at(&mut text, 0).ok()?;
    if text.last() != Some(&0) {
        text.push(0);
    }
    // SAFETY: the name is a valid C string, and the flags are memfd_create's own.
    let made = unsafe { libc::memfd_create(c"trindle-keymap".as_ptr(), libc::MFD_CLOEXEC) };
    if made < 0 {
        return None;
    }
    // SAFETY: a fresh memfd, owned by nothing else, taken over by the File.
    let copy = unsafe { File::from_raw_fd(made) };
    copy.write_all_at(&text, 0).ok()?;
    Some((copy.into(), text.len() as u32))
}

#[cfg(test)]
mod tests {
    use std::{io::Write, os::unix::fs::FileExt};

    fn fd_of(bytes: &[u8]) -> std::os::fd::OwnedFd {
        let mut file = tempfile_in_memory();
        file.write_all(bytes).unwrap();
        file.into()
    }

    fn tempfile_in_memory() -> std::fs::File {
        // SAFETY: the name is a valid C string.
        let made = unsafe { libc::memfd_create(c"test".as_ptr(), 0) };
        assert!(made >= 0);
        // SAFETY: a fresh memfd, owned by nothing else.
        unsafe { std::os::fd::FromRawFd::from_raw_fd(made) }
    }

    fn read(fd: std::os::fd::OwnedFd, size: u32) -> Vec<u8> {
        let mut out = vec![0u8; size as usize];
        std::fs::File::from(fd).read_exact_at(&mut out, 0).unwrap();
        out
    }

    #[test]
    fn a_keymap_without_its_nul_gains_one() {
        let (fd, size) = super::with_ending(fd_of(b"xkb_keymap {};"), 14).unwrap();
        assert_eq!(size, 15);
        assert_eq!(read(fd, size), b"xkb_keymap {};\0");
    }

    #[test]
    fn a_length_too_big_to_be_a_keymap_is_refused() {
        assert!(super::with_ending(fd_of(b"xkb_keymap {};"), u32::MAX).is_none());
        assert!(super::with_ending(fd_of(b"xkb_keymap {};"), 0).is_none());
    }

    #[test]
    fn a_descriptor_holding_less_than_it_says_is_refused() {
        assert!(super::with_ending(fd_of(b""), 4096).is_none());
        assert!(super::with_ending(fd_of(b"xkb_keymap {};"), 4096).is_none());
    }

    #[test]
    fn a_keymap_with_its_nul_keeps_its_length_in_a_copy_of_its_own() {
        let (fd, size) = super::with_ending(fd_of(b"xkb_keymap {};\0"), 15).unwrap();
        assert_eq!(size, 15);
        assert_eq!(read(fd, size), b"xkb_keymap {};\0");
    }
}

#[cfg(test)]
mod keymap_nul_tests {
    use smithay::input::keyboard::{KeymapFile, xkb};

    fn parsed(sealed: bool) -> bool {
        use std::io::Read;
        let map = xkb::Keymap::new_from_names(&xkb::Context::new(xkb::CONTEXT_NO_FLAGS), "", "pc105", "gb", "", None, xkb::COMPILE_NO_FLAGS).unwrap();
        let mut ok = false;
        KeymapFile::new(&map)
            .with_fd(sealed, |fd, size| {
                let mut text = Vec::new();
                std::fs::File::from(fd.try_clone_to_owned().unwrap()).take(size as u64).read_to_end(&mut text).unwrap();
                assert_eq!(text.len(), size);
                assert_eq!(text.last(), Some(&0));
                let text = String::from_utf8(text).unwrap().trim_end_matches('\0').to_string();
                ok = xkb::Keymap::new_from_string(&xkb::Context::new(xkb::CONTEXT_NO_FLAGS), text, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS).is_some();
            })
            .unwrap();
        ok
    }

    #[test]
    fn keymap_ends_in_nul_for_old_and_new_clients() {
        assert!(parsed(true));
        assert!(parsed(false));
    }
}
