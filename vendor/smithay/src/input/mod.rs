//! .

use std::{
    fmt,
    hash::Hash,
    sync::{Arc, Mutex, Weak},
};

use tracing::{info_span, instrument};

use self::touch::TouchTarget;
use self::{
    keyboard::{Error as KeyboardError, KeyboardHandle, KeyboardTarget, LedState},
    touch::TouchHandle,
};
use self::{
    pointer::{CursorImageStatus, PointerHandle, PointerTarget},
    touch::TouchGrab,
};
use crate::utils::{user_data::UserDataMap, Serial};

pub mod keyboard;
pub mod pointer;
pub mod touch;

pub trait SeatHandler: Sized {
    type KeyboardFocus: KeyboardTarget<Self> + 'static;
    type PointerFocus: PointerTarget<Self> + 'static;
    type TouchFocus: TouchTarget<Self> + 'static;

    fn seat_state(&mut self) -> &mut SeatState<Self>;

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&Self::KeyboardFocus>) {}

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}

    fn led_state_changed(&mut self, _seat: &Seat<Self>, _led_state: LedState) {}
}
pub struct SeatState<D: SeatHandler> {
    pub(crate) seats: Vec<Seat<D>>,
}

impl<D: SeatHandler> fmt::Debug for SeatState<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeatState").field("seats", &self.seats).finish()
    }
}

pub struct Seat<D: SeatHandler> {
    pub(crate) arc: Arc<SeatRc<D>>,
}

#[derive(Debug, Clone)]
pub struct WeakSeat<D: SeatHandler>(Weak<SeatRc<D>>);

impl<D: SeatHandler> WeakSeat<D> {
    pub fn upgrade(&self) -> Option<Seat<D>> {
        self.0.upgrade().map(|arc| Seat { arc })
    }

    pub fn is_alive(&self) -> bool {
        self.0.strong_count() != 0
    }
}

impl<D: SeatHandler> Seat<D> {
    pub fn downgrade(&self) -> WeakSeat<D> {
        WeakSeat(Arc::downgrade(&self.arc))
    }
}

impl<D: SeatHandler> fmt::Debug for Seat<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Seat").field("arc", &self.arc).finish()
    }
}

impl<D: SeatHandler> PartialEq for Seat<D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.arc, &other.arc)
    }
}
impl<D: SeatHandler> Eq for Seat<D> {}

impl<D: SeatHandler> Hash for Seat<D> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.arc).hash(state)
    }
}

pub(crate) struct Inner<D: SeatHandler> {
    pub(crate) pointer: Option<PointerHandle<D>>,
    pub(crate) keyboard: Option<KeyboardHandle<D>>,
    pub(crate) touch: Option<TouchHandle<D>>,

    #[cfg(feature = "wayland_frontend")]
    pub(crate) global: Option<wayland_server::backend::GlobalId>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) known_seats: Vec<wayland_server::Weak<wayland_server::protocol::wl_seat::WlSeat>>,
}

#[cfg(not(feature = "wayland_frontend"))]
impl<D: SeatHandler> fmt::Debug for Inner<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inner")
            .field("pointer", &self.pointer)
            .field("keyboard", &self.keyboard)
            .finish()
    }
}

#[cfg(feature = "wayland_frontend")]
impl<D: SeatHandler> fmt::Debug for Inner<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inner")
            .field("pointer", &self.pointer)
            .field("keyboard", &self.keyboard)
            .field("touch", &self.touch)
            .field("global", &self.global)
            .field("known_seats", &self.known_seats)
            .finish()
    }
}

pub(crate) struct SeatRc<D: SeatHandler> {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) inner: Mutex<Inner<D>>,
    span: tracing::Span,
    user_data_map: UserDataMap,
}

impl<D: SeatHandler> fmt::Debug for SeatRc<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeatRc")
            .field("name", &self.name)
            .field("inner", &self.inner)
            .field("user_data_map", &self.user_data_map)
            .finish()
    }
}

impl<D: SeatHandler> Clone for Seat<D> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}

impl<D: SeatHandler> Default for SeatState<D> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<D: SeatHandler> SeatState<D> {
    pub fn new() -> Self {
        Self { seats: Vec::new() }
    }

    pub fn new_seat<N>(&mut self, name: N) -> Seat<D>
    where
        N: Into<String>,
    {
        let name = name.into();
        let span = info_span!("input_seat", name);

        let arc = Arc::new(SeatRc {
            name,
            inner: Mutex::new(Inner {
                pointer: None,
                keyboard: None,
                touch: None,

                #[cfg(feature = "wayland_frontend")]
                global: None,
                #[cfg(feature = "wayland_frontend")]
                known_seats: Vec::new(),
            }),
            span,
            user_data_map: UserDataMap::new(),
        });
        self.seats.push(Seat { arc: arc.clone() });

        Seat { arc }
    }
}

impl<D: SeatHandler + 'static> Seat<D> {
    pub fn user_data(&self) -> &UserDataMap {
        &self.arc.user_data_map
    }

    #[instrument(parent = &self.arc.span, skip(self))]
    pub fn add_pointer(&mut self) -> PointerHandle<D> {
        let mut inner = self.arc.inner.lock().unwrap();
        let pointer = PointerHandle::new();
        if inner.pointer.is_some() {
            inner.pointer = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
        inner.pointer = Some(pointer.clone());
        #[cfg(feature = "wayland_frontend")]
        inner.send_all_caps();
        pointer
    }

    pub fn get_pointer(&self) -> Option<PointerHandle<D>> {
        self.arc.inner.lock().unwrap().pointer.clone()
    }

    #[instrument(parent = &self.arc.span, skip(self))]
    pub fn remove_pointer(&mut self) {
        let mut inner = self.arc.inner.lock().unwrap();
        if inner.pointer.is_some() {
            inner.pointer = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
    }

    #[instrument(parent = &self.arc.span, skip(self))]
    pub fn add_keyboard(
        &mut self,
        xkb_config: keyboard::XkbConfig<'_>,
        repeat_delay: i32,
        repeat_rate: i32,
    ) -> Result<KeyboardHandle<D>, KeyboardError> {
        let mut inner = self.arc.inner.lock().unwrap();
        let keyboard = self::keyboard::KeyboardHandle::new(xkb_config, repeat_delay, repeat_rate)?;
        if inner.keyboard.is_some() {
            inner.keyboard = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
        inner.keyboard = Some(keyboard.clone());
        #[cfg(feature = "wayland_frontend")]
        inner.send_all_caps();
        Ok(keyboard)
    }

    pub fn get_keyboard(&self) -> Option<KeyboardHandle<D>> {
        self.arc.inner.lock().unwrap().keyboard.clone()
    }

    #[instrument(parent = &self.arc.span, skip(self))]
    pub fn remove_keyboard(&mut self) {
        let mut inner = self.arc.inner.lock().unwrap();
        if inner.keyboard.is_some() {
            inner.keyboard = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
    }

    pub fn add_touch(&mut self) -> TouchHandle<D> {
        Self::add_touch_with_default_grab(self, || Box::new(touch::DefaultGrab))
    }

    pub fn add_touch_with_default_grab<F>(&mut self, defaut_grab: F) -> TouchHandle<D>
    where
        F: Fn() -> Box<dyn TouchGrab<D>> + Send + 'static,
    {
        let mut inner = self.arc.inner.lock().unwrap();
        let touch = TouchHandle::new(defaut_grab);
        if inner.touch.is_some() {
            inner.touch = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
        inner.touch = Some(touch.clone());
        #[cfg(feature = "wayland_frontend")]
        inner.send_all_caps();
        touch
    }

    pub fn get_touch(&self) -> Option<TouchHandle<D>> {
        self.arc.inner.lock().unwrap().touch.clone()
    }

    pub fn remove_touch(&mut self) {
        let mut inner = self.arc.inner.lock().unwrap();
        if inner.touch.is_some() {
            inner.touch = None;
            #[cfg(feature = "wayland_frontend")]
            inner.send_all_caps();
        }
    }

    pub fn name(&self) -> &str {
        &self.arc.name
    }
}

pub(super) enum GrabStatus<G: ?Sized> {
    None,
    Active(Serial, Box<G>),
    Borrowed,
}

impl<G: ?Sized> fmt::Debug for GrabStatus<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrabStatus::None => f.debug_tuple("GrabStatus::None").finish(),
            GrabStatus::Active(serial, _) => f.debug_tuple("GrabStatus::Active").field(&serial).finish(),
            GrabStatus::Borrowed => f.debug_tuple("GrabStatus::Borrowed").finish(),
        }
    }
}
