// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use super::KeyAction;

use crate::{focus::KeyboardFocusTarget, shell::FullscreenSurface, state::Backend, AnvilState};

use smithay::{
    backend::input::{Event, InputBackend, KeyState, KeyboardKeyEvent},
    desktop::{layer_map_for_output, LayerSurface},
    input::keyboard::{
        keysyms as xkb, FilterResult, KeyboardHandle, Keycode, Keysym, ModifiersState,
    },
    utils::{Serial, SERIAL_COUNTER as SCOUNTER},
    wayland::{keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitorSeat, seat::WaylandFocus},
};
use tracing::{debug, info};

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(super) fn keyboard_key_to_action<B: InputBackend>(&mut self, evt: B::KeyboardKeyEvent) -> KeyAction {
        let keycode = evt.key_code();
        let state = evt.state();
        debug!(?keycode, ?state, "key");
        let serial = SCOUNTER.next_serial();
        let time = Event::time_msec(&evt);
        let press = Press { keycode, state, serial, time };
        let mut suppressed_keys = self.suppressed_keys.clone();
        let keyboard = self.seat.get_keyboard().unwrap();

        if self.is_locked() {
            return self.locked_key(&keyboard, press, suppressed_keys);
        }

        self.reader_listen();
        if self.reader_key(&keyboard, keycode, state) {
            self.expander.clear();
            return KeyAction::None;
        }

        if let Some(action) = self.key_to_a_panel(&keyboard, press, &mut suppressed_keys) {
            return action;
        }
        self.key_to_a_window(&keyboard, press, suppressed_keys)
    }

    fn locked_key(&mut self, keyboard: &KeyboardHandle<Self>, press: Press, mut suppressed_keys: Vec<Keysym>) -> KeyAction {
        let Press { keycode, state, serial, time } = press;
        self.expander.clear();

        self.reader_keys.release();

        if keyboard.is_grabbed() {
            keyboard.unset_grab(self);
        }
        if !matches!(keyboard.current_focus(), Some(KeyboardFocusTarget::LockSurface(_))) {
            self.focus_lock_at(self.pointer.current_location());
        }
        let action = keyboard
            .input(self, keycode, state, serial, time, |_, _, handle| {
                let keysym = handle.modified_sym();
                if let KeyState::Pressed = state {
                    if crate::lock::works_while_locked(keysym) {
                        suppressed_keys.push(keysym);
                        return FilterResult::Intercept(vt_switch(keysym));
                    }
                } else if suppressed_keys.contains(&keysym) {

                    suppressed_keys.retain(|k| *k != keysym);
                    return FilterResult::Intercept(KeyAction::None);
                }
                FilterResult::Forward
            })
            .unwrap_or(KeyAction::None);
        self.suppressed_keys = suppressed_keys;
        self.release_key = None;
        self.released_bind = None;
        action
    }

    fn key_to_a_panel(&mut self, keyboard: &KeyboardHandle<Self>, press: Press, suppressed_keys: &mut Vec<Keysym>) -> Option<KeyAction> {

        let judged = self.ring.by_hand || !self.question_on_a_panel();

        if let Some(KeyboardFocusTarget::LayerSurface(held)) = keyboard.current_focus() {
            if self.whole_keyboard_panel(held.wl_surface()).is_none() && self.question_window_in_front() {
                self.focus_topmost();
            }
        }
        for surface in self.whole_keyboard_panels() {
            if self.panel_passed_over(keyboard, &surface, judged, press.serial) {
                continue;
            }
            return Some(self.key_into_panel(keyboard, surface, press, suppressed_keys));
        }
        None
    }

    fn panel_passed_over(&mut self, keyboard: &KeyboardHandle<Self>, surface: &LayerSurface, judged: bool, serial: Serial) -> bool {
        let gone = smithay::reexports::wayland_server::Resource::client(surface.wl_surface()).is_none();
        if !(gone || (crate::trusted::is_ours(surface.wl_surface()) && self.ring_silent(surface.wl_surface(), judged)) || self.shot_silent(surface.wl_surface(), judged)) {
            return false;
        }
        if matches!(keyboard.current_focus(), Some(KeyboardFocusTarget::LayerSurface(held)) if held == *surface) {
            keyboard.set_focus(self, Option::<KeyboardFocusTarget>::None, serial);
            self.focus_topmost();
        }
        true
    }

    fn key_into_panel(&mut self, keyboard: &KeyboardHandle<Self>, surface: LayerSurface, press: Press, suppressed_keys: &mut Vec<Keysym>) -> KeyAction {
        let Press { keycode, state, serial, time } = press;
        let modal_surface = surface.wl_surface().clone();
        let mut escaped = false;
        let name = surface.namespace().to_string();
        if self.keys_to_panel.as_deref() != Some(name.as_str()) {
            info!(target: crate::capture::LOG, panel = %name, layer = ?surface.layer(), "keys go to a panel holding the whole keyboard, not to a window");
            self.keys_to_panel = Some(name);
        }
        self.expander.clear();
        self.drop_window_grabs();
        self.drop_keyboard_grab_for(surface.wl_surface());

        self.own_press(serial.into(), Some(surface.wl_surface()), true);
        keyboard.set_focus(self, Some(surface.into()), serial);

        let action = keyboard
            .input(self, keycode, state, serial, time, |st, modifiers, handle| {
                let keysym = handle.modified_sym();
                if state == KeyState::Pressed {
                    if keysym.raw() == xkb::KEY_Escape && st.is_modal_panel(&modal_surface) {
                        suppressed_keys.push(keysym);
                        escaped = true;
                        return FilterResult::Intercept(KeyAction::None);
                    }
                    if is_media_key(keysym) {
                        let mods = own_modifier_released(crate::config::Mods::held(modifiers), keysym);
                        if let Some(b) = st.config.keybind(mods, keysym).cloned().filter(|b| !b.on_release) {
                            suppressed_keys.push(keysym);
                            return FilterResult::Intercept(KeyAction::Bound(b.action));
                        }
                    }
                } else if suppressed_keys.contains(&keysym) {
                    suppressed_keys.retain(|k| *k != keysym);
                    return FilterResult::Intercept(KeyAction::None);
                }
                FilterResult::Forward
            })
            .unwrap_or(KeyAction::None);
        self.suppressed_keys = std::mem::take(suppressed_keys);
        if escaped {
            self.modal_escape();
        }
        action
    }

    fn key_to_a_window(&mut self, keyboard: &KeyboardHandle<Self>, press: Press, mut suppressed_keys: Vec<Keysym>) -> KeyAction {
        let Press { keycode, state, serial, time } = press;
        self.hand_keyboard_to_full_screen(keyboard, serial);
        if self.keys_to_panel.take().is_some() {
            info!(target: crate::capture::LOG, "keys go to the window with the keyboard again");
        }
        self.say_keys_go_nowhere(keyboard, state);

        let inhibited = keyboard
            .current_focus()
            .and_then(|f| f.wl_surface().map(|s| s.into_owned()))
            .and_then(|surface| self.seat.keyboard_shortcuts_inhibitor_for_surface(&surface))
            .is_some_and(|inhibitor| inhibitor.is_active());

        let to = keyboard.current_focus().and_then(|f| f.wl_surface().map(|s| s.into_owned()));
        self.own_press(serial.into(), to.as_ref(), true);

        let watch = self.ring.by_hand && self.may_watch_typing();
        if watch {
            self.expander.refresh(&crate::files::data_home().join("trindle/plugins"));
        }
        let keys = WindowKeys { state, time, inhibited, watch };
        let action = keyboard
            .input(self, keycode, state, serial, time, |st, modifiers, handle| {
                keys.filter(st, modifiers, handle.modified_sym(), &mut suppressed_keys)
            })
            .unwrap_or(KeyAction::None);

        self.suppressed_keys = suppressed_keys;
        match (action, self.released_bind.take()) {
            (KeyAction::None, Some(bound)) => KeyAction::Bound(bound),
            (action, _) => action,
        }
    }

    fn hand_keyboard_to_full_screen(&mut self, keyboard: &KeyboardHandle<Self>, serial: Serial) {
        let Some(KeyboardFocusTarget::LayerSurface(held)) = keyboard.current_focus() else { return };
        let window = self.space.outputs().find_map(|o| {
            let on = layer_map_for_output(o).layers().any(|l| l == &held);
            if !on || !crate::trusted::hidden_by_full_screen(o, &held) {
                return None;
            }
            o.user_data().get::<FullscreenSurface>().and_then(|f| f.get())
        });
        if let Some(window) = window {
            keyboard.set_focus(self, Some(window.into()), serial);
        }
    }

    fn say_keys_go_nowhere(&mut self, keyboard: &KeyboardHandle<Self>, state: KeyState) {
        let nothing = keyboard.current_focus().is_none();
        if nothing && state == KeyState::Pressed && !self.keys_to_nothing {
            info!(target: crate::capture::LOG, keyboard_grabbed = keyboard.is_grabbed(), pointer_grabbed = self.pointer.is_grabbed(),
                  windows = self.space.elements().count(), "a key was pressed with nothing holding the keyboard");
        }
        self.keys_to_nothing = nothing;
    }

    fn may_watch_typing(&self) -> bool {
        use smithay::reexports::wayland_protocols::wp::text_input::zv3::server::zwp_text_input_v3::ContentPurpose;
        use smithay::wayland::text_input::TextInputSeat;
        let window = matches!(self.seat.get_keyboard().and_then(|k| k.current_focus()), Some(KeyboardFocusTarget::Window(_)));
        window && !self.is_locked() && !self.only_the_person_answers()
            && !matches!(self.seat.text_input().active_purpose(), Some(ContentPurpose::Password | ContentPurpose::Pin))
            && !self.seat.text_input().active_sensitive()
    }

    pub(super) fn type_swap(&mut self, swap: crate::expander::Swap, time: u32) {
        if !self.may_watch_typing() {
            return;
        }
        let seat = self.seat.clone();
        for (map, keys) in crate::expander::keymaps(&swap) {
            smithay::wayland::virtual_keyboard::type_with_keymap(&seat, self, &map, &keys, time);
        }
    }

    pub(crate) fn reader_listen(&mut self) {
        for ask in self.reader_keys.hear() {
            match ask {
                crate::reader_keys::Ask::CapsToggle => self.toggle_caps_lock(),
                crate::reader_keys::Ask::Pass(name, mods) => self.pass_key(&name, &mods),
            }
        }
    }

    pub(crate) fn pass_key(&mut self, name: &str, mods: &[&str]) {
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        let on = match keyboard.current_focus() {
            Some(KeyboardFocusTarget::LayerSurface(l)) => Some(crate::trusted::kind_of(&l)),
            _ => None,
        };
        if !crate::reader_keys::may_be_passed_a_key(self.is_locked(), matches!(keyboard.current_focus(), Some(KeyboardFocusTarget::LockSurface(_))), on) {
            return;
        }
        let codes = keyboard.with_xkb_state(self, |ctx| {
            let xkb = ctx.xkb().lock().unwrap();
            let layout = xkb.active_layout().0;
            // SAFETY: the keymap is only read here, while the lock is held.
            let map = unsafe { xkb.keymap() };
            let key = crate::reader_keys::code_of(map, layout, name)?;
            let held: Option<Vec<_>> = mods.iter().map(|m| crate::reader_keys::code_of(map, layout, m)).collect();
            Some((held?, key))
        });
        let Some((held, key)) = codes else { return };
        let time = std::time::Duration::from(self.clock.now()).as_millis() as u32;
        let steps = held.iter().map(|c| (*c, KeyState::Pressed))
            .chain([(key, KeyState::Pressed), (key, KeyState::Released)])
            .chain(held.iter().rev().map(|c| (*c, KeyState::Released)));
        for (code, state) in steps {
            keyboard.input(self, code, state, SCOUNTER.next_serial(), time, |_, _, _| FilterResult::Forward::<bool>);
        }
    }

    fn toggle_caps_lock(&mut self) {
        let Some(keyboard) = self.seat.get_keyboard() else { return };
        let mut mods = keyboard.modifier_state();
        mods.caps_lock = !mods.caps_lock;
        if keyboard.set_modifier_state(mods) != 0 {
            self.backend_data.update_led_state(keyboard.led_state());

            if let Some(focus) = keyboard.current_focus() {
                let seat = self.seat.clone();
                smithay::input::keyboard::KeyboardTarget::modifiers(&focus, &seat, self, keyboard.modifier_state(), smithay::utils::SERIAL_COUNTER.next_serial());
            }
        }
        self.reader_keys.tell_caps(keyboard.modifier_state().caps_lock);
    }

    fn reader_key(&mut self, keyboard: &smithay::input::keyboard::KeyboardHandle<Self>, code: smithay::input::keyboard::Keycode, state: KeyState) -> bool {
        let sym = keyboard.with_xkb_state(self, |ctx| {
            let xkb = ctx.xkb().lock().unwrap();
            let layout = xkb.active_layout();
            xkb.raw_syms_for_key_in_layout(code, layout).first().copied()
        });
        let Some(sym) = sym else { return false };

        if !self.reader_keys.wants(crate::reader_keys::is_reader_key(sym), sym == smithay::input::keyboard::Keysym::Escape) {
            return false;
        }
        let name = keyboard.with_xkb_state(self, |ctx| {
            let xkb = ctx.xkb().lock().unwrap();
            let layout = xkb.active_layout();
            // SAFETY: the keymap is only read here, while the lock is held.
            crate::reader_keys::name(unsafe { xkb.keymap() }, layout.0, code)
        });
        let Some(name) = name else { return false };
        let press = crate::reader_keys::Press {
            code: code.raw(),
            name: &name,
            reader_key: crate::reader_keys::is_reader_key(sym),
            down: state == KeyState::Pressed,
            mods: keyboard.modifier_state(),
            real: self.ring.by_hand,
        };
        let taken = self.reader_keys.take(&press);
        self.watch_reader();
        taken
    }

    pub fn release_all_keys(&mut self) {
        self.reader_keys.release();
        let keyboard = self.seat.get_keyboard().unwrap();
        for keycode in keyboard.pressed_keys() {
            keyboard.input(
                self,
                keycode,
                KeyState::Released,
                SCOUNTER.next_serial(),
                0,
                |_, _, _| FilterResult::Forward::<bool>,
            );
        }
    }
}

#[derive(Clone, Copy)]
struct Press {
    keycode: Keycode,
    state: KeyState,
    serial: Serial,
    time: u32,
}

struct WindowKeys {
    state: KeyState,
    time: u32,

    inhibited: bool,

    watch: bool,
}

impl WindowKeys {

    fn filter<BackendData: Backend>(
        &self,
        st: &mut AnvilState<BackendData>,
        modifiers: &ModifiersState,
        keysym: Keysym,
        suppressed_keys: &mut Vec<Keysym>,
    ) -> FilterResult<KeyAction> {
        debug!(
            state = ?self.state,
            mods = ?modifiers,
            keysym = ::xkbcommon::xkb::keysym_get_name(keysym),
            "keysym"
        );

        let mods = own_modifier_released(crate::config::Mods::held(modifiers), keysym);
        if let KeyState::Pressed = self.state {
            self.pressed(st, modifiers, mods, keysym, suppressed_keys)
        } else {
            Self::released(st, mods, keysym, suppressed_keys)
        }
    }

    fn pressed<BackendData: Backend>(
        &self,
        st: &mut AnvilState<BackendData>,
        modifiers: &ModifiersState,
        mods: crate::config::Mods,
        keysym: Keysym,
        suppressed_keys: &mut Vec<Keysym>,
    ) -> FilterResult<KeyAction> {

        if (xkb::KEY_XF86Switch_VT_1..=xkb::KEY_XF86Switch_VT_12).contains(&keysym.raw()) {
            suppressed_keys.push(keysym);
            return FilterResult::Intercept(vt_switch(keysym));
        }
        let bind = st.config.keybind(mods, keysym).cloned();

        let ring_key = bind.as_ref().is_some_and(|b| matches!(&b.action, crate::config::Action::Execute(c) if crate::own_ring::opens_ring(c)));
        if self.inhibited && !ring_key {
            st.expander.clear();
            return FilterResult::Forward;
        }
        match bind {

            Some(b) if b.on_release => {
                st.expander.clear();
                st.release_key = Some(keysym);
                FilterResult::Forward
            }
            Some(b) => {
                st.expander.clear();
                st.release_key = None;
                suppressed_keys.push(keysym);
                FilterResult::Intercept(KeyAction::Bound(b.action))
            }
            None => {
                st.release_key = None;
                self.typed(st, modifiers, keysym, suppressed_keys)
            }
        }
    }

    fn typed<BackendData: Backend>(
        &self,
        st: &mut AnvilState<BackendData>,
        modifiers: &ModifiersState,
        keysym: Keysym,
        suppressed_keys: &mut Vec<Keysym>,
    ) -> FilterResult<KeyAction> {
        if !self.watch {
            st.expander.clear();
            return FilterResult::Forward;
        }
        let held = modifiers.ctrl || modifiers.alt || modifiers.logo;
        match st.expander.key(crate::expander::typed(keysym, held)) {

            Some(swap) => {
                suppressed_keys.push(keysym);
                FilterResult::Intercept(KeyAction::Expand(swap, self.time))
            }
            None => FilterResult::Forward,
        }
    }

    fn released<BackendData: Backend>(
        st: &mut AnvilState<BackendData>,
        mods: crate::config::Mods,
        keysym: Keysym,
        suppressed_keys: &mut Vec<Keysym>,
    ) -> FilterResult<KeyAction> {
        if suppressed_keys.contains(&keysym) {
            suppressed_keys.retain(|k| *k != keysym);
            FilterResult::Intercept(KeyAction::None)
        } else if st.release_key.take() == Some(keysym) {

            st.released_bind = st.config.keybind(mods, keysym)
                .filter(|b| b.on_release).map(|b| b.action.clone());
            FilterResult::Forward
        } else {
            FilterResult::Forward
        }
    }
}

fn vt_switch(keysym: Keysym) -> KeyAction {
    KeyAction::VtSwitch((keysym.raw() - xkb::KEY_XF86Switch_VT_1 + 1) as i32)
}

pub(crate) fn is_media_key(key: Keysym) -> bool {
    [
        xkb::KEY_XF86AudioRaiseVolume, xkb::KEY_XF86AudioLowerVolume, xkb::KEY_XF86AudioMute, xkb::KEY_XF86AudioMicMute,
        xkb::KEY_XF86MonBrightnessUp, xkb::KEY_XF86MonBrightnessDown, xkb::KEY_XF86AudioPlay, xkb::KEY_XF86AudioPause,
        xkb::KEY_XF86AudioStop, xkb::KEY_XF86AudioNext, xkb::KEY_XF86AudioPrev,
    ]
    .contains(&key.raw())
}

fn own_modifier_released(mut mods: crate::config::Mods, key: Keysym) -> crate::config::Mods {
    match key {
        Keysym::Super_L | Keysym::Super_R | Keysym::Hyper_L | Keysym::Hyper_R => mods.logo = false,
        Keysym::Alt_L | Keysym::Alt_R | Keysym::Meta_L | Keysym::Meta_R => mods.alt = false,
        Keysym::Control_L | Keysym::Control_R => mods.ctrl = false,
        Keysym::Shift_L | Keysym::Shift_R => mods.shift = false,
        _ => {}
    }
    mods
}

