//! Keyboard-related types for smithay's input abstraction.

use crate::backend::input::KeyState;
use crate::utils::{IsAlive, Serial, SERIAL_COUNTER};
use downcast_rs::{impl_downcast, Downcast};
use std::collections::HashSet;
#[cfg(feature = "wayland_frontend")]
use std::sync::RwLock;
use std::{
    default::Default,
    fmt, io,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tracing::{debug, info, info_span, instrument, trace};

use xkbcommon::xkb::ffi::XKB_STATE_LAYOUT_EFFECTIVE;
pub use xkbcommon::xkb::{self, keysyms, Keycode, Keysym};

use super::{GrabStatus, Seat, SeatHandler};

#[cfg(feature = "wayland_frontend")]
use wayland_server::{Resource, Weak};
#[cfg(feature = "wayland_frontend")]
mod keymap_file;
#[cfg(feature = "wayland_frontend")]
pub use keymap_file::{KeymapFile, KeymapFileId};

mod modifiers_state;
pub use modifiers_state::{ModifiersState, SerializedMods};

mod xkb_config;
pub use xkb_config::XkbConfig;

pub trait KeyboardTarget<D>: IsAlive + PartialEq + Clone + fmt::Debug + Send
where
    D: SeatHandler,
{
    fn enter(&self, seat: &Seat<D>, data: &mut D, keys: Vec<KeysymHandle<'_>>, serial: Serial);
    fn leave(&self, seat: &Seat<D>, data: &mut D, serial: Serial);
    fn key(
        &self,
        seat: &Seat<D>,
        data: &mut D,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    );
    fn modifiers(&self, seat: &Seat<D>, data: &mut D, modifiers: ModifiersState, serial: Serial);
    fn replace(
        &self,
        replaced: <D as SeatHandler>::KeyboardFocus,
        seat: &Seat<D>,
        data: &mut D,
        keys: Vec<KeysymHandle<'_>>,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        KeyboardTarget::<D>::leave(&replaced, seat, data, serial);
        KeyboardTarget::<D>::enter(self, seat, data, keys, serial);
        KeyboardTarget::<D>::modifiers(self, seat, data, modifiers, serial);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedMapping {
    pub num: Option<xkb::LedIndex>,
    pub caps: Option<xkb::LedIndex>,
    pub scroll: Option<xkb::LedIndex>,
}

impl LedMapping {
    pub fn from_keymap(keymap: &xkb::Keymap) -> Self {
        Self {
            num: match keymap.led_get_index(xkb::LED_NAME_NUM) {
                xkb::LED_INVALID => None,
                index => Some(index),
            },
            caps: match keymap.led_get_index(xkb::LED_NAME_CAPS) {
                xkb::LED_INVALID => None,
                index => Some(index),
            },
            scroll: match keymap.led_get_index(xkb::LED_NAME_SCROLL) {
                xkb::LED_INVALID => None,
                index => Some(index),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct LedState {
    pub num: Option<bool>,
    pub caps: Option<bool>,
    pub scroll: Option<bool>,
}

impl LedState {
    pub fn update_with(&mut self, state: &xkb::State, mapping: &LedMapping) -> bool {
        let previous_state = *self;
        self.num = mapping.num.map(|idx| state.led_index_is_active(idx));
        self.caps = mapping.caps.map(|idx| state.led_index_is_active(idx));
        self.scroll = mapping.scroll.map(|idx| state.led_index_is_active(idx));
        *self != previous_state
    }

    pub fn from_state(state: &xkb::State, mapping: &LedMapping) -> Self {
        let mut led_state = LedState::default();
        led_state.update_with(state, mapping);
        led_state
    }
}

pub struct Xkb {
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
}

impl Xkb {
    pub unsafe fn context(&self) -> &xkb::Context {
        &self.context
    }

    pub unsafe fn keymap(&self) -> &xkb::Keymap {
        &self.keymap
    }

    pub unsafe fn state(&self) -> &xkb::State {
        &self.state
    }

    pub fn active_layout(&self) -> Layout {
        (0..self.keymap.num_layouts())
            .find(|&idx| self.state.layout_index_is_active(idx, XKB_STATE_LAYOUT_EFFECTIVE))
            .map(Layout)
            .unwrap_or_default()
    }

    pub fn layout_name(&self, layout: Layout) -> &str {
        self.keymap.layout_get_name(layout.0)
    }

    pub fn layouts(&self) -> impl Iterator<Item = Layout> {
        (0..self.keymap.num_layouts()).map(Layout)
    }

    pub fn raw_syms_for_key_in_layout(&self, keycode: Keycode, layout: Layout) -> &[Keysym] {
        self.keymap.key_get_syms_by_level(keycode, layout.0, 0)
    }
}

impl fmt::Debug for Xkb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Xkb")
            .field("context", &self.context.get_raw_ptr())
            .field("keymap", &self.keymap.get_raw_ptr())
            .field("state", &self.state.get_raw_ptr())
            .finish()
    }
}

unsafe impl Send for Xkb {}

pub(crate) struct KbdInternal<D: SeatHandler> {
    pub(crate) focus: Option<(<D as SeatHandler>::KeyboardFocus, Serial)>,
    pending_focus: Option<<D as SeatHandler>::KeyboardFocus>,
    pub(crate) pressed_keys: HashSet<Keycode>,
    pub(crate) forwarded_pressed_keys: HashSet<Keycode>,
    pub(crate) mods_state: ModifiersState,
    xkb: Arc<Mutex<Xkb>>,
    pub(crate) repeat_rate: i32,
    pub(crate) repeat_delay: i32,
    led_mapping: LedMapping,
    pub(crate) led_state: LedState,
    grab: GrabStatus<dyn KeyboardGrab<D>>,
}

impl<D: SeatHandler> fmt::Debug for KbdInternal<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KbdInternal")
            .field("focus", &self.focus)
            .field("pending_focus", &self.pending_focus)
            .field("pressed_keys", &self.pressed_keys)
            .field("forwarded_pressed_keys", &self.forwarded_pressed_keys)
            .field("mods_state", &self.mods_state)
            .field("xkb", &self.xkb)
            .field("repeat_rate", &self.repeat_rate)
            .field("repeat_delay", &self.repeat_delay)
            .finish()
    }
}

unsafe impl<D: SeatHandler> Send for KbdInternal<D> {}

impl<D: SeatHandler + 'static> KbdInternal<D> {
    fn new(xkb_config: XkbConfig<'_>, repeat_rate: i32, repeat_delay: i32) -> Result<KbdInternal<D>, ()> {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb_config.compile_keymap(&context)?;
        let state = xkb::State::new(&keymap);
        let led_mapping = LedMapping::from_keymap(&keymap);
        let led_state = LedState::from_state(&state, &led_mapping);
        Ok(KbdInternal {
            focus: None,
            pending_focus: None,
            pressed_keys: HashSet::new(),
            forwarded_pressed_keys: HashSet::new(),
            mods_state: ModifiersState::default(),
            xkb: Arc::new(Mutex::new(Xkb {
                context,
                keymap,
                state,
            })),
            repeat_rate,
            repeat_delay,
            led_mapping,
            led_state,
            grab: GrabStatus::None,
        })
    }

    fn key_input(&mut self, keycode: Keycode, state: KeyState) -> (bool, bool) {
        let direction = match state {
            KeyState::Pressed => {
                self.pressed_keys.insert(keycode);
                xkb::KeyDirection::Down
            }
            KeyState::Released => {
                self.pressed_keys.remove(&keycode);
                xkb::KeyDirection::Up
            }
        };

        let mut xkb = self.xkb.lock().unwrap();
        let state_components = xkb.state.update_key(keycode, direction);
        let modifiers_changed = state_components != 0;
        if modifiers_changed {
            self.mods_state.update_with(&xkb.state);
        }
        let leds_changed = self.led_state.update_with(&xkb.state, &self.led_mapping);
        (modifiers_changed, leds_changed)
    }

    fn with_grab<F>(&mut self, data: &mut D, seat: &Seat<D>, f: F)
    where
        F: FnOnce(&mut D, &mut KeyboardInnerHandle<'_, D>, &mut dyn KeyboardGrab<D>),
    {
        let mut grab = std::mem::replace(&mut self.grab, GrabStatus::Borrowed);
        match grab {
            GrabStatus::Borrowed => panic!("Accessed a keyboard grab from within a keyboard grab access."),
            GrabStatus::Active(_, ref mut handler) => {
                if let Some(ref surface) = handler.start_data().focus {
                    if !surface.alive() {
                        handler.unset(data);
                        self.grab = GrabStatus::None;
                        f(
                            data,
                            &mut KeyboardInnerHandle { inner: self, seat },
                            &mut DefaultGrab,
                        );
                        return;
                    }
                }
                f(
                    data,
                    &mut KeyboardInnerHandle { inner: self, seat },
                    &mut **handler,
                );
            }
            GrabStatus::None => {
                f(
                    data,
                    &mut KeyboardInnerHandle { inner: self, seat },
                    &mut DefaultGrab,
                );
            }
        }

        if let GrabStatus::Borrowed = self.grab {
            self.grab = grab;
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Libxkbcommon could not load the specified keymap")]
    BadKeymap,
    #[error("Failed to create tempfile to share the keymap: {0}")]
    IoError(io::Error),
}

pub(crate) struct KbdRc<D: SeatHandler> {
    pub(crate) internal: Mutex<KbdInternal<D>>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) keymap: Mutex<KeymapFile>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) known_kbds: Mutex<Vec<Weak<wayland_server::protocol::wl_keyboard::WlKeyboard>>>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) last_enter: Mutex<Option<Serial>>,
    pub(crate) span: tracing::Span,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) active_keymap: RwLock<KeymapFileId>,
}

#[cfg(not(feature = "wayland_frontend"))]
impl<D: SeatHandler> fmt::Debug for KbdRc<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KbdRc").field("internal", &self.internal).finish()
    }
}

#[cfg(feature = "wayland_frontend")]
impl<D: SeatHandler> fmt::Debug for KbdRc<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KbdRc")
            .field("internal", &self.internal)
            .field("keymap", &self.keymap)
            .field("known_kbds", &self.known_kbds)
            .field("last_enter", &self.last_enter)
            .finish()
    }
}

pub struct KeysymHandle<'a> {
    xkb: &'a Mutex<Xkb>,
    keycode: Keycode,
}

impl fmt::Debug for KeysymHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.keycode)
    }
}

impl<'a> KeysymHandle<'a> {
    pub fn xkb(&self) -> &Mutex<Xkb> {
        self.xkb
    }

    pub fn modified_sym(&self) -> Keysym {
        self.xkb.lock().unwrap().state.key_get_one_sym(self.keycode)
    }

    pub fn modified_syms(&self) -> Vec<Keysym> {
        self.xkb.lock().unwrap().state.key_get_syms(self.keycode).to_vec()
    }

    pub fn raw_syms(&self) -> Vec<Keysym> {
        let xkb = self.xkb.lock().unwrap();
        xkb.keymap
            .key_get_syms_by_level(self.keycode, xkb.state.key_get_layout(self.keycode), 0)
            .to_vec()
    }

    pub fn raw_latin_sym_or_raw_current_sym(&self) -> Option<Keysym> {
        let xkb = self.xkb.lock().unwrap();
        let effective_layout = Layout(xkb.state.key_get_layout(self.keycode));

        let raw_syms =
            xkb.keymap
                .key_get_syms_by_level(self.keycode, xkb.state.key_get_layout(self.keycode), 0);
        let base_sym = *raw_syms.first()?;

        if base_sym.key_char().map(|ch| ch.is_ascii()).unwrap_or(true) {
            return Some(base_sym);
        };

        for layout in xkb.layouts() {
            if layout == effective_layout {
                continue;
            }

            if let Some(keysym) = xkb.raw_syms_for_key_in_layout(self.keycode, layout).first() {
                if keysym
                    .key_char()
                    .map(|key| key.is_ascii() && !key.is_ascii_control())
                    .unwrap_or(false)
                {
                    return Some(*keysym);
                }
            }
        }

        Some(base_sym)
    }

    pub fn raw_code(&'a self) -> Keycode {
        self.keycode
    }
}

pub struct XkbContext<'a> {
    xkb: &'a Mutex<Xkb>,
    mods_state: &'a mut ModifiersState,
    mods_changed: &'a mut bool,
    leds_state: &'a mut LedState,
    leds_changed: &'a mut bool,
    leds_mapping: &'a LedMapping,
}

impl XkbContext<'_> {
    pub fn xkb(&self) -> &Mutex<Xkb> {
        self.xkb
    }

    pub fn set_layout(&mut self, layout: Layout) {
        let mut xkb = self.xkb.lock().unwrap();

        let state = xkb.state.update_mask(
            self.mods_state.serialized.depressed,
            self.mods_state.serialized.latched,
            self.mods_state.serialized.locked,
            0,
            0,
            layout.0,
        );

        if state != 0 {
            self.mods_state.update_with(&xkb.state);
            *self.mods_changed = true;
        }

        *self.leds_changed = self.leds_state.update_with(&xkb.state, self.leds_mapping);
    }

    pub fn cycle_next_layout(&mut self) {
        let xkb = self.xkb.lock().unwrap();
        let next_layout = (xkb.active_layout().0 + 1) % xkb.keymap.num_layouts();
        drop(xkb);
        self.set_layout(Layout(next_layout));
    }

    pub fn cycle_prev_layout(&mut self) {
        let xkb = self.xkb.lock().unwrap();
        let num_layouts = xkb.keymap.num_layouts();
        let next_layout = (num_layouts + xkb.active_layout().0 - 1) % num_layouts;
        drop(xkb);
        self.set_layout(Layout(next_layout));
    }
}

impl fmt::Debug for XkbContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XkbContext")
            .field("mods_state", &self.mods_state)
            .field("mods_changed", &self.mods_changed)
            .finish()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout(pub xkb::LayoutIndex);

#[derive(Debug)]
pub enum FilterResult<T> {
    Forward,
    Intercept(T),
}

pub struct GrabStartData<D: SeatHandler> {
    pub focus: Option<<D as SeatHandler>::KeyboardFocus>,
}

impl<D: SeatHandler + 'static> fmt::Debug for GrabStartData<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GrabStartData")
            .field("focus", &self.focus)
            .finish()
    }
}

impl<D: SeatHandler + 'static> Clone for GrabStartData<D> {
    fn clone(&self) -> Self {
        GrabStartData {
            focus: self.focus.clone(),
        }
    }
}

pub trait KeyboardGrab<D: SeatHandler>: Downcast {
    #[allow(clippy::too_many_arguments)]
    fn input(
        &mut self,
        data: &mut D,
        handle: &mut KeyboardInnerHandle<'_, D>,
        keycode: Keycode,
        state: KeyState,
        modifiers: Option<ModifiersState>,
        serial: Serial,
        time: u32,
    );

    fn set_focus(
        &mut self,
        data: &mut D,
        handle: &mut KeyboardInnerHandle<'_, D>,
        focus: Option<<D as SeatHandler>::KeyboardFocus>,
        serial: Serial,
    );

    fn start_data(&self) -> &GrabStartData<D>;

    fn unset(&mut self, data: &mut D);
}

impl_downcast!(KeyboardGrab<D> where D: SeatHandler);

pub struct KeyboardHandle<D: SeatHandler> {
    pub(crate) arc: Arc<KbdRc<D>>,
}

impl<D: SeatHandler> fmt::Debug for KeyboardHandle<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyboardHandle").field("arc", &self.arc).finish()
    }
}

impl<D: SeatHandler> Clone for KeyboardHandle<D> {
    #[inline]
    fn clone(&self) -> Self {
        KeyboardHandle {
            arc: self.arc.clone(),
        }
    }
}

impl<D: SeatHandler> ::std::cmp::PartialEq for KeyboardHandle<D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.arc, &other.arc)
    }
}

impl<D: SeatHandler + 'static> KeyboardHandle<D> {
    pub(crate) fn new(xkb_config: XkbConfig<'_>, repeat_delay: i32, repeat_rate: i32) -> Result<Self, Error> {
        let span = info_span!("input_keyboard");
        let _guard = span.enter();

        info!("Initializing a xkbcommon handler with keymap query");
        let internal = KbdInternal::new(xkb_config, repeat_rate, repeat_delay).map_err(|_| {
            debug!("Loading keymap failed");
            Error::BadKeymap
        })?;

        let xkb = internal.xkb.lock().unwrap();

        info!(name = xkb.keymap.layouts().next(), "Loaded Keymap");

        #[cfg(feature = "wayland_frontend")]
        let keymap_file = KeymapFile::new(&xkb.keymap);
        #[cfg(feature = "wayland_frontend")]
        let active_keymap = keymap_file.id();

        drop(xkb);
        drop(_guard);
        Ok(Self {
            arc: Arc::new(KbdRc {
                #[cfg(feature = "wayland_frontend")]
                keymap: Mutex::new(keymap_file),
                internal: Mutex::new(internal),
                #[cfg(feature = "wayland_frontend")]
                known_kbds: Mutex::new(Vec::new()),
                #[cfg(feature = "wayland_frontend")]
                last_enter: Mutex::new(None),
                #[cfg(feature = "wayland_frontend")]
                active_keymap: RwLock::new(active_keymap),
                span,
            }),
        })
    }

    #[cfg(feature = "wayland_frontend")]
    #[instrument(parent = &self.arc.span, skip(self, data, keymap))]
    pub(crate) fn change_keymap(
        &self,
        data: &mut D,
        focus: &Option<&mut <D as SeatHandler>::KeyboardFocus>,
        keymap: &xkb::Keymap,
        mods: ModifiersState,
    ) {
        let mut keymap_file = self.arc.keymap.lock().unwrap();
        keymap_file.change_keymap(keymap);

        self.send_keymap(data, focus, &keymap_file, mods);
    }

    #[cfg(feature = "wayland_frontend")]
    #[instrument(parent = &self.arc.span, skip(self, data, keymap_file))]
    pub(crate) fn send_keymap(
        &self,
        data: &mut D,
        focus: &Option<&mut <D as SeatHandler>::KeyboardFocus>,
        keymap_file: &KeymapFile,
        mods: ModifiersState,
    ) -> bool {
        use std::os::unix::io::AsFd;
        use tracing::warn;
        use wayland_server::{protocol::wl_keyboard::KeymapFormat, Resource};

        let new_id = keymap_file.id();
        if new_id == *self.arc.active_keymap.read().unwrap() {
            return false;
        }
        *self.arc.active_keymap.write().unwrap() = new_id;

        let known_kbds = &self.arc.known_kbds;
        for kbd in &*known_kbds.lock().unwrap() {
            let Ok(kbd) = kbd.upgrade() else {
                continue;
            };

            let res = keymap_file.with_fd(kbd.version() >= 7, |fd, size| {
                kbd.keymap(KeymapFormat::XkbV1, fd.as_fd(), size as u32)
            });
            if let Err(e) = res {
                warn!(
                    err = ?e,
                    "Failed to send keymap to client"
                );
            }
        }

        let seat = self.get_seat(data);
        if let Some(focus) = focus {
            focus.modifiers(&seat, data, mods, SERIAL_COUNTER.next_serial());
        }

        true
    }

    fn update_xkb_state(&self, data: &mut D, keymap: xkb::Keymap) {
        let mut internal = self.arc.internal.lock().unwrap();

        let mut state = xkb::State::new(&keymap);
        for key in &internal.pressed_keys {
            state.update_key(*key, xkb::KeyDirection::Down);
        }

        let led_mapping = LedMapping::from_keymap(&keymap);
        internal.led_mapping = led_mapping;
        internal.mods_state.update_with(&state);
        let leds_changed = internal.led_state.update_with(&state, &led_mapping);
        let mut xkb = internal.xkb.lock().unwrap();
        xkb.keymap = keymap.clone();
        xkb.state = state;
        drop(xkb);

        let mods = internal.mods_state;
        let focus = internal.focus.as_mut().map(|(focus, _)| focus);

        #[cfg(not(feature = "wayland_frontend"))]
        if let Some(focus) = focus.as_ref() {
            let seat = self.get_seat(data);
            focus.modifiers(&seat, data, mods, SERIAL_COUNTER.next_serial());
        };

        #[cfg(feature = "wayland_frontend")]
        self.change_keymap(data, &focus, &keymap, mods);

        if leds_changed {
            let led_state = internal.led_state;
            std::mem::drop(internal);
            let seat = self.get_seat(data);
            data.led_state_changed(&seat, led_state);
        }
    }

    pub fn set_keymap_from_string(&self, data: &mut D, keymap: String) -> Result<(), Error> {
        let keymap = xkb::Keymap::new_from_string(
            &self.arc.internal.lock().unwrap().xkb.lock().unwrap().context,
            keymap,
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .ok_or_else(|| {
            debug!("Loading keymap from string failed");
            Error::BadKeymap
        })?;
        self.update_xkb_state(data, keymap);
        Ok(())
    }

    pub fn set_xkb_config(&self, data: &mut D, xkb_config: XkbConfig<'_>) -> Result<(), Error> {
        let keymap = xkb_config
            .compile_keymap(&self.arc.internal.lock().unwrap().xkb.lock().unwrap().context)
            .map_err(|_| {
                debug!("Loading keymap from XkbConfig failed");
                Error::BadKeymap
            })?;
        self.update_xkb_state(data, keymap);
        Ok(())
    }

    pub fn with_xkb_state<F, T>(&self, data: &mut D, mut callback: F) -> T
    where
        F: FnMut(XkbContext<'_>) -> T,
    {
        let (result, new_led_state) = {
            let internal = &mut *self.arc.internal.lock().unwrap();
            let mut mods_changed = false;
            let mut leds_changed = false;
            let state = XkbContext {
                mods_state: &mut internal.mods_state,
                xkb: &mut internal.xkb,
                mods_changed: &mut mods_changed,
                leds_state: &mut internal.led_state,
                leds_changed: &mut leds_changed,
                leds_mapping: &internal.led_mapping,
            };

            let result = callback(state);

            if mods_changed {
                if let Some((focus, _)) = internal.focus.as_mut() {
                    let seat = self.get_seat(data);
                    focus.modifiers(&seat, data, internal.mods_state, SERIAL_COUNTER.next_serial());
                };
            }

            (result, leds_changed.then_some(internal.led_state))
        };

        if let Some(led_state) = new_led_state {
            let seat = self.get_seat(data);
            data.led_state_changed(&seat, led_state)
        }

        result
    }

    pub fn set_grab<G: KeyboardGrab<D> + 'static>(&self, data: &mut D, grab: G, serial: Serial) {
        let mut inner = self.arc.internal.lock().unwrap();
        if let GrabStatus::Active(_, handler) = &mut inner.grab {
            handler.unset(data);
        }
        inner.grab = GrabStatus::Active(serial, Box::new(grab));
    }

    pub fn unset_grab(&self, data: &mut D) {
        let mut inner = self.arc.internal.lock().unwrap();
        if let GrabStatus::Active(_, handler) = &mut inner.grab {
            handler.unset(data);
        }
        inner.grab = GrabStatus::None;
    }

    pub fn has_grab(&self, serial: Serial) -> bool {
        let guard = self.arc.internal.lock().unwrap();
        match guard.grab {
            GrabStatus::Active(s, _) => s == serial,
            _ => false,
        }
    }

    pub fn is_grabbed(&self) -> bool {
        let guard = self.arc.internal.lock().unwrap();
        !matches!(guard.grab, GrabStatus::None)
    }

    pub fn grab_start_data(&self) -> Option<GrabStartData<D>> {
        let guard = self.arc.internal.lock().unwrap();
        match &guard.grab {
            GrabStatus::Active(_, g) => Some(g.start_data().clone()),
            _ => None,
        }
    }

    pub fn with_grab<T>(&self, f: impl FnOnce(Serial, &dyn KeyboardGrab<D>) -> T) -> Option<T> {
        let guard = self.arc.internal.lock().unwrap();
        if let GrabStatus::Active(s, g) = &guard.grab {
            Some(f(*s, &**g))
        } else {
            None
        }
    }

    #[instrument(level = "trace", parent = &self.arc.span, skip(self, data, filter))]
    pub fn input<T, F>(
        &self,
        data: &mut D,
        keycode: Keycode,
        state: KeyState,
        serial: Serial,
        time: u32,
        filter: F,
    ) -> Option<T>
    where
        F: FnOnce(&mut D, &ModifiersState, KeysymHandle<'_>) -> FilterResult<T>,
    {
        let (filter_result, mods_changed) = self.input_intercept(data, keycode, state, filter);
        if let FilterResult::Intercept(val) = filter_result {
            trace!("Input was intercepted by filter");
            return Some(val);
        }

        self.input_forward(data, keycode, state, serial, time, mods_changed);
        None
    }

    pub fn input_intercept<T, F>(
        &self,
        data: &mut D,
        keycode: Keycode,
        state: KeyState,
        filter: F,
    ) -> (T, bool)
    where
        F: FnOnce(&mut D, &ModifiersState, KeysymHandle<'_>) -> T,
    {
        trace!("Handling keystroke");

        let mut guard = self.arc.internal.lock().unwrap();
        let (mods_changed, leds_changed) = guard.key_input(keycode, state);
        let led_state = guard.led_state;
        let mods_state = guard.mods_state;
        let xkb = guard.xkb.clone();
        std::mem::drop(guard);

        let key_handle = KeysymHandle { xkb: &xkb, keycode };

        trace!(mods_state = ?mods_state, sym = xkb::keysym_get_name(key_handle.modified_sym()), "Calling input filter");
        let filter_result = filter(data, &mods_state, key_handle);

        if leds_changed {
            let seat = self.get_seat(data);
            data.led_state_changed(&seat, led_state);
        }

        (filter_result, mods_changed)
    }

    pub fn input_forward(
        &self,
        data: &mut D,
        keycode: Keycode,
        state: KeyState,
        serial: Serial,
        time: u32,
        mods_changed: bool,
    ) {
        let mut guard = self.arc.internal.lock().unwrap();
        match state {
            KeyState::Pressed => {
                guard.forwarded_pressed_keys.insert(keycode);
            }
            KeyState::Released => {
                guard.forwarded_pressed_keys.remove(&keycode);
            }
        };

        let seat = self.get_seat(data);
        let modifiers = mods_changed.then_some(guard.mods_state);
        guard.with_grab(data, &seat, |data, handle, grab| {
            grab.input(data, handle, keycode, state, modifiers, serial, time);
        });
        if guard.focus.is_some() {
            trace!("Input forwarded to client");
        } else {
            trace!("No client currently focused");
        }
    }

    #[instrument(level = "debug", parent = &self.arc.span, skip(self, data, focus), fields(focus = focus.is_some()))]
    pub fn set_focus(&self, data: &mut D, focus: Option<<D as SeatHandler>::KeyboardFocus>, serial: Serial) {
        let mut guard = self.arc.internal.lock().unwrap();
        guard.pending_focus.clone_from(&focus);
        let seat = self.get_seat(data);
        guard.with_grab(data, &seat, |data, handle, grab| {
            grab.set_focus(data, handle, focus, serial);
        });
    }

    pub fn pressed_keys(&self) -> HashSet<Keycode> {
        let guard = self.arc.internal.lock().unwrap();
        guard.pressed_keys.clone()
    }

    pub fn with_pressed_keysyms<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Vec<KeysymHandle<'_>>) -> R,
        R: 'static,
    {
        let guard = self.arc.internal.lock().unwrap();
        {
            let handles = guard
                .pressed_keys
                .iter()
                .map(|keycode| KeysymHandle {
                    xkb: &guard.xkb,
                    keycode: *keycode,
                })
                .collect::<Vec<_>>();
            f(handles)
        }
    }

    pub fn modifier_state(&self) -> ModifiersState {
        self.arc.internal.lock().unwrap().mods_state
    }

    pub fn set_modifier_state(&self, mods_state: ModifiersState) -> u32 {
        let internal = &mut self.arc.internal.lock().unwrap();

        let (leds_changed, led_state, modifiers_changed, serialized) = {
            let state = &mut internal.xkb.lock().unwrap().state;

            let serialized = mods_state.serialize_back(state);

            let modifiers_changed = state.update_mask(
                serialized.depressed,
                serialized.latched,
                serialized.locked,
                serialized.layout_effective & xkb::STATE_LAYOUT_DEPRESSED,
                serialized.layout_effective & xkb::STATE_LAYOUT_LATCHED,
                serialized.layout_effective & xkb::STATE_LAYOUT_LOCKED,
            );

            if modifiers_changed == 0 {
                return 0;
            }

            let led_mapping = &internal.led_mapping;
            let mut led_state = internal.led_state;
            let leds_changed = led_state.update_with(state, led_mapping);

            (leds_changed, led_state, modifiers_changed, serialized)
        };

        internal.mods_state = mods_state;
        internal.mods_state.serialized = serialized;

        if leds_changed {
            internal.led_state = led_state;
        }

        modifiers_changed
    }

    pub fn led_state(&self) -> LedState {
        self.arc.internal.lock().unwrap().led_state
    }

    pub fn is_focused(&self) -> bool {
        self.arc.internal.lock().unwrap().focus.is_some()
    }

    #[instrument(parent = &self.arc.span, skip(self))]
    pub fn change_repeat_info(&self, rate: i32, delay: i32) {
        let mut guard = self.arc.internal.lock().unwrap();
        guard.repeat_delay = delay;
        guard.repeat_rate = rate;
        #[cfg(feature = "wayland_frontend")]
        for kbd in &*self.arc.known_kbds.lock().unwrap() {
            let Ok(kbd) = kbd.upgrade() else {
                continue;
            };
            if kbd.version() >= 4 {
                kbd.repeat_info(rate, delay);
            }
        }
    }

    #[cfg(feature = "wayland_frontend")]
    pub fn last_enter(&self) -> Option<Serial> {
        *self.arc.last_enter.lock().unwrap()
    }

    fn get_seat(&self, data: &mut D) -> Seat<D> {
        let seat_state = data.seat_state();
        seat_state
            .seats
            .iter()
            .find(|seat| seat.get_keyboard().map(|h| &h == self).unwrap_or(false))
            .cloned()
            .unwrap()
    }
}

impl<D> KeyboardHandle<D>
where
    D: SeatHandler,
    <D as SeatHandler>::KeyboardFocus: Clone,
{
    pub fn current_focus(&self) -> Option<<D as SeatHandler>::KeyboardFocus> {
        self.arc
            .internal
            .lock()
            .unwrap()
            .focus
            .clone()
            .map(|(focus, _)| focus)
    }
}

pub struct KeyboardInnerHandle<'a, D: SeatHandler> {
    inner: &'a mut KbdInternal<D>,
    seat: &'a Seat<D>,
}

impl<D: SeatHandler> fmt::Debug for KeyboardInnerHandle<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyboardInnerHandle")
            .field("inner", &self.inner)
            .field("seat", &self.seat.arc.name)
            .finish()
    }
}

impl<D: SeatHandler + 'static> KeyboardInnerHandle<'_, D> {
    pub fn set_grab<G: KeyboardGrab<D> + 'static>(
        &mut self,
        handler: &mut dyn KeyboardGrab<D>,
        data: &mut D,
        serial: Serial,
        grab: G,
    ) {
        handler.unset(data);
        self.inner.grab = GrabStatus::Active(serial, Box::new(grab));
    }

    pub fn unset_grab(
        &mut self,
        handler: &mut dyn KeyboardGrab<D>,
        data: &mut D,
        serial: Serial,
        restore_focus: bool,
    ) {
        handler.unset(data);
        self.inner.grab = GrabStatus::None;
        if restore_focus {
            let focus = self.inner.pending_focus.clone();
            self.set_focus(data, focus, serial);
        }
    }

    pub fn current_focus(&self) -> Option<&<D as SeatHandler>::KeyboardFocus> {
        self.inner.focus.as_ref().map(|f| &f.0)
    }

    pub fn keysym_handle(&self, keycode: Keycode) -> KeysymHandle<'_> {
        KeysymHandle {
            keycode,
            xkb: &self.inner.xkb,
        }
    }

    pub fn modifier_state(&self) -> ModifiersState {
        self.inner.mods_state
    }

    pub fn input(
        &mut self,
        data: &mut D,
        keycode: Keycode,
        key_state: KeyState,
        modifiers: Option<ModifiersState>,
        serial: Serial,
        time: u32,
    ) {
        let (focus, _) = match self.inner.focus.as_mut() {
            Some(focus) => focus,
            None => return,
        };

        #[cfg(feature = "wayland_frontend")]
        if let Some(keyboard_handle) = self.seat.get_keyboard() {
            let keymap_file = keyboard_handle.arc.keymap.lock().unwrap();
            let mods = self.inner.mods_state;
            keyboard_handle.send_keymap(data, &Some(focus), &keymap_file, mods);
        }

        let key = KeysymHandle {
            xkb: &self.inner.xkb,
            keycode,
        };

        focus.key(self.seat, data, key, key_state, serial, time);
        if let Some(mods) = modifiers {
            focus.modifiers(self.seat, data, mods, serial);
        }
    }

    pub fn with_pressed_keysyms<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Vec<KeysymHandle<'_>>) -> R,
        R: 'static,
    {
        let handles = self
            .inner
            .pressed_keys
            .iter()
            .map(|code| self.keysym_handle(*code))
            .collect();
        f(handles)
    }

    pub fn set_focus(
        &mut self,
        data: &mut D,
        focus: Option<<D as SeatHandler>::KeyboardFocus>,
        serial: Serial,
    ) {
        if let Some(focus) = focus {
            let old_focus = self.inner.focus.replace((focus.clone(), serial));
            match (focus, old_focus) {
                (focus, Some((old_focus, _))) if focus == old_focus => {
                    trace!("Focus unchanged");
                }
                (focus, Some((old_focus, _))) => {
                    trace!("Focus set to new surface");
                    let keys = self
                        .inner
                        .forwarded_pressed_keys
                        .iter()
                        .map(|keycode| KeysymHandle {
                            xkb: &self.inner.xkb,
                            keycode: *keycode,
                        })
                        .collect();

                    focus.replace(old_focus, self.seat, data, keys, self.inner.mods_state, serial);
                    data.focus_changed(self.seat, Some(&focus));
                }
                (focus, None) => {
                    let keys = self
                        .inner
                        .forwarded_pressed_keys
                        .iter()
                        .map(|keycode| KeysymHandle {
                            xkb: &self.inner.xkb,
                            keycode: *keycode,
                        })
                        .collect();

                    focus.enter(self.seat, data, keys, serial);
                    focus.modifiers(self.seat, data, self.inner.mods_state, serial);
                    data.focus_changed(self.seat, Some(&focus));
                }
            }
        } else if let Some((old_focus, _)) = self.inner.focus.take() {
            trace!("Focus unset");
            old_focus.leave(self.seat, data, serial);
        }
    }
}

struct DefaultGrab;

impl<D: SeatHandler + 'static> KeyboardGrab<D> for DefaultGrab {
    fn input(
        &mut self,
        data: &mut D,
        handle: &mut KeyboardInnerHandle<'_, D>,
        keycode: Keycode,
        state: KeyState,
        modifiers: Option<ModifiersState>,
        serial: Serial,
        time: u32,
    ) {
        handle.input(data, keycode, state, modifiers, serial, time)
    }

    fn set_focus(
        &mut self,
        data: &mut D,
        handle: &mut KeyboardInnerHandle<'_, D>,
        focus: Option<<D as SeatHandler>::KeyboardFocus>,
        serial: Serial,
    ) {
        handle.set_focus(data, focus, serial)
    }

    fn start_data(&self) -> &GrabStartData<D> {
        unreachable!()
    }

    fn unset(&mut self, _data: &mut D) {}
}
