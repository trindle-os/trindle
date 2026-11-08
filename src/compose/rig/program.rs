// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use wayland_client::protocol::{wl_buffer, wl_compositor, wl_keyboard, wl_output, wl_pointer, wl_region, wl_registry, wl_seat, wl_shm, wl_shm_pool, wl_surface};
use wayland_client::{delegate_noop, Connection, Dispatch, QueueHandle, WEnum};
use wayland_protocols::xdg::shell::client::{xdg_popup, xdg_positioner, xdg_surface, xdg_toplevel, xdg_wm_base};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::ZwlrLayerShellV1;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1};

#[derive(Default)]
pub struct Program {
    pub(super) compositor: Option<wl_compositor::WlCompositor>,
    pub(super) shm: Option<wl_shm::WlShm>,
    pub(super) shell: Option<ZwlrLayerShellV1>,
    pub(super) wm: Option<xdg_wm_base::XdgWmBase>,
    pub(super) keyboard: Option<wl_keyboard::WlKeyboard>,
    pub(super) pointer: Option<wl_pointer::WlPointer>,

    pub pressed: Vec<u32>,

    pub numbered: Vec<u32>,
    pub(super) seat: Option<wl_seat::WlSeat>,
    pub(super) typer: Option<ZwpVirtualKeyboardManagerV1>,

    pub(super) named: Option<TrindleOwnPressV1>,

    pub(super) own: Vec<u32>,

    pub(super) was: Vec<(u32, u32)>,

    pub pressed_was: Vec<Option<u32>>,

    pub clicked: Vec<u32>,
    pub clicked_was: Vec<Option<u32>>,

    pub named_first: Vec<bool>,

    pub refused: usize,

    pub(super) closed: usize,

    pub(super) unlock: Option<TrindleUnlockV1>,

    pub(super) said: Vec<Result<(), String>>,
    pub(super) lock_manager: Option<ExtSessionLockManagerV1>,
    pub(super) lock_object: Option<ExtSessionLockV1>,

    pub(super) held: bool,
    pub(super) finished: bool,

    pub(super) keys_state: Option<smithay::input::keyboard::xkb::State>,
    pub typed: String,
    pub(super) texts: Option<ZwpTextInputManagerV3>,

    pub(super) outputs: Vec<wl_output::WlOutput>,
}

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
mod unlocking {
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::backend as wayland_backend;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/unlock.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("src/compose/protocols/unlock.xml");
}
use unlocking::trindle_unlock_v1::{self, TrindleUnlockV1};
use wayland_protocols::ext::session_lock::v1::client::{
    ext_session_lock_manager_v1::ExtSessionLockManagerV1,
    ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,
    ext_session_lock_v1::{self, ExtSessionLockV1},
};

impl Dispatch<TrindleUnlockV1, ()> for Program {
    fn event(p: &mut Self, _: &TrindleUnlockV1, event: trindle_unlock_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            trindle_unlock_v1::Event::Done => p.said.push(Ok(())),
            trindle_unlock_v1::Event::Refused { reason } => p.said.push(Err(reason)),
        }
    }
}

impl Dispatch<ExtSessionLockV1, ()> for Program {
    fn event(p: &mut Self, _: &ExtSessionLockV1, event: ext_session_lock_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            ext_session_lock_v1::Event::Locked => p.held = true,
            ext_session_lock_v1::Event::Finished => p.finished = true,
            _ => {}
        }
    }
}
delegate_noop!(Program: ignore ExtSessionLockManagerV1);
delegate_noop!(Program: ignore ExtSessionLockSurfaceV1);
delegate_noop!(Program: ignore wl_output::WlOutput);
delegate_noop!(Program: ignore wl_region::WlRegion);

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
mod named {
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::backend as wayland_backend;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/own_press.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("src/compose/protocols/own_press.xml");
}
use named::trindle_own_press_v1::{self, TrindleOwnPressV1};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1, zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

impl Dispatch<TrindleOwnPressV1, ()> for Program {
    fn event(p: &mut Self, _: &TrindleOwnPressV1, event: trindle_own_press_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            trindle_own_press_v1::Event::Own { serial } => p.own.push(serial),
            trindle_own_press_v1::Event::Refused { .. } => p.refused += 1,
            trindle_own_press_v1::Event::Was { serial, code } => p.was.push((serial, code)),
        }
    }
}
delegate_noop!(Program: ignore ZwpVirtualKeyboardManagerV1);
delegate_noop!(Program: ignore ZwpVirtualKeyboardV1);
delegate_noop!(Program: ignore ZwpTextInputManagerV3);
delegate_noop!(Program: ignore ZwpTextInputV3);
use wayland_protocols::wp::text_input::zv3::client::{zwp_text_input_manager_v3::ZwpTextInputManagerV3, zwp_text_input_v3::ZwpTextInputV3};

impl Dispatch<wl_registry::WlRegistry, ()> for Program {
    fn event(p: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            match interface.as_str() {
                "wl_compositor" => p.compositor = Some(registry.bind(name, version.min(4), qh, ())),
                "wl_shm" => p.shm = Some(registry.bind(name, 1, qh, ())),
                "zwlr_layer_shell_v1" => p.shell = Some(registry.bind(name, version.min(4), qh, ())),
                "xdg_wm_base" => p.wm = Some(registry.bind(name, version.min(2), qh, ())),
                "wl_seat" => {
                    let seat: wl_seat::WlSeat = registry.bind(name, version.min(5), qh, ());
                    p.keyboard = Some(seat.get_keyboard(qh, ()));
                    p.pointer = Some(seat.get_pointer(qh, ()));
                    p.seat = Some(seat);
                }
                "trindle_unlock_v1" => p.unlock = Some(registry.bind(name, 1, qh, ())),
                "ext_session_lock_manager_v1" => p.lock_manager = Some(registry.bind(name, 1, qh, ())),
                "trindle_own_press_v1" => p.named = Some(registry.bind(name, version.min(3), qh, ())),
                "zwp_virtual_keyboard_manager_v1" => p.typer = Some(registry.bind(name, 1, qh, ())),
                "zwp_text_input_manager_v3" => p.texts = Some(registry.bind(name, 1, qh, ())),
                "wl_output" => p.outputs.push(registry.bind(name, version.min(4), qh, ())),
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for Program {
    fn event(p: &mut Self, _: &wl_keyboard::WlKeyboard, event: wl_keyboard::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        use smithay::input::keyboard::xkb;
        match &event {
            wl_keyboard::Event::Keymap { fd, size, .. } => {
                let mut text = vec![0; *size as usize];
                let file = std::fs::File::from(fd.try_clone().unwrap());
                std::os::unix::fs::FileExt::read_exact_at(&file, &mut text, 0).unwrap();
                let text = String::from_utf8_lossy(&text).trim_end_matches('\0').to_string();
                let map = xkb::Keymap::new_from_string(&xkb::Context::new(xkb::CONTEXT_NO_FLAGS), text, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS);
                p.keys_state = map.map(|m| xkb::State::new(&m));
            }
            wl_keyboard::Event::Modifiers { mods_depressed, mods_latched, mods_locked, group, .. } => {
                if let Some(st) = p.keys_state.as_mut() {
                    st.update_mask(*mods_depressed, *mods_latched, *mods_locked, 0, 0, *group);
                }
            }
            wl_keyboard::Event::Key { key, state: WEnum::Value(wl_keyboard::KeyState::Pressed), .. } => {
                if let Some(st) = p.keys_state.as_ref() {
                    let sym = st.key_get_one_sym((key + 8).into());
                    if sym == xkb::Keysym::BackSpace {
                        p.typed.pop();
                    } else if let Some(c) = sym.key_char() {
                        p.typed.push(c);
                    }
                }
            }
            _ => {}
        }
        if let wl_keyboard::Event::Key { serial, key, state, .. } = event {
            p.named_first.push(p.own.contains(&serial));
            p.numbered.push(serial);
            if state == WEnum::Value(wl_keyboard::KeyState::Pressed) {
                p.pressed.push(key);
                p.pressed_was.push(p.was.iter().find(|w| w.0 == serial).map(|w| w.1));
            }
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for Program {
    fn event(p: &mut Self, _: &wl_pointer::WlPointer, event: wl_pointer::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let wl_pointer::Event::Button { serial, button, state, .. } = event {
            p.named_first.push(p.own.contains(&serial));
            p.numbered.push(serial);
            if state == WEnum::Value(wl_pointer::ButtonState::Pressed) {
                p.clicked.push(button);
                p.clicked_was.push(p.was.iter().find(|w| w.0 == serial).map(|w| w.1));
            }
        }
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for Program {
    fn event(p: &mut Self, panel: &ZwlrLayerSurfaceV1, event: zwlr_layer_surface_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, .. } => panel.ack_configure(serial),
            zwlr_layer_surface_v1::Event::Closed => p.closed += 1,
            _ => {}
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for Program {
    fn event(_: &mut Self, surface: &xdg_surface::XdgSurface, event: xdg_surface::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let xdg_surface::Event::Configure { serial } = event {
            surface.ack_configure(serial);
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for Program {
    fn event(_: &mut Self, wm: &xdg_wm_base::XdgWmBase, event: xdg_wm_base::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm.pong(serial);
        }
    }
}

delegate_noop!(Program: ignore wl_compositor::WlCompositor);
delegate_noop!(Program: ignore wl_surface::WlSurface);
delegate_noop!(Program: ignore wl_shm::WlShm);
delegate_noop!(Program: ignore wl_shm_pool::WlShmPool);
delegate_noop!(Program: ignore wl_buffer::WlBuffer);
delegate_noop!(Program: ignore wl_seat::WlSeat);
delegate_noop!(Program: ignore ZwlrLayerShellV1);
delegate_noop!(Program: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(Program: ignore xdg_popup::XdgPopup);
delegate_noop!(Program: ignore xdg_positioner::XdgPositioner);
