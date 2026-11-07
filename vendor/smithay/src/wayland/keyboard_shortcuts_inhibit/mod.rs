//! Protocol for inhibiting the compositor keyboard shortcuts.

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::atomic};
use wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::server::{
    zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1,
    zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1,
};
use wayland_server::{
    backend::{GlobalId, ObjectId},
    protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    Dispatch, DisplayHandle, GlobalDispatch, Resource,
};

mod dispatch;
pub use dispatch::KeyboardShortcutsInhibitorUserData;

use crate::input::{Seat, SeatHandler};

type SeatId = ObjectId;

#[derive(Debug, Default)]
struct SeatInhibitors(Vec<KeyboardShortcutsInhibitor>);

impl SeatInhibitors {
    fn push(&mut self, value: KeyboardShortcutsInhibitor) {
        self.0.push(value)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn is_inhibited(&self) -> bool {
        self.0.iter().any(|i| i.is_active())
    }

    fn remove(&mut self, id: ObjectId) -> Option<KeyboardShortcutsInhibitor> {
        self.0
            .iter()
            .position(|i| i.inhibitor.id() == id)
            .map(|id| self.0.remove(id))
    }

    fn surface_has_inhibitor(&self, surface: &WlSurface) -> bool {
        self.inhibitor_for_surface(surface).is_some()
    }

    fn inhibitor_for_surface(&self, surface: &WlSurface) -> Option<&KeyboardShortcutsInhibitor> {
        self.0.iter().find(|i| i.wl_surface() == surface)
    }
}

#[derive(Debug)]
pub struct KeyboardShortcutsInhibitState {
    manager_global: GlobalId,
    inhibitors: HashMap<SeatId, Rc<RefCell<SeatInhibitors>>>,
}

impl KeyboardShortcutsInhibitState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<ZwpKeyboardShortcutsInhibitManagerV1, ()>,
        D: Dispatch<ZwpKeyboardShortcutsInhibitManagerV1, ()>,
        D: Dispatch<ZwpKeyboardShortcutsInhibitorV1, KeyboardShortcutsInhibitorUserData>,
        D: 'static,
    {
        let manager_global = display.create_global::<D, ZwpKeyboardShortcutsInhibitManagerV1, _>(1, ());
        Self {
            manager_global,
            inhibitors: HashMap::new(),
        }
    }

    pub fn global(&self) -> GlobalId {
        self.manager_global.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardShortcutsInhibitor {
    inhibitor: ZwpKeyboardShortcutsInhibitorV1,
}

impl KeyboardShortcutsInhibitor {
    fn data(&self) -> &KeyboardShortcutsInhibitorUserData {
        self.inhibitor.data().unwrap()
    }

    fn set_is_active(&self, v: bool) {
        self.data().is_active.store(v, atomic::Ordering::Release);
    }

    fn seat_id(&self) -> &SeatId {
        &self.data().seat
    }

    pub fn seat(&self, dh: &DisplayHandle) -> Option<WlSeat> {
        WlSeat::from_id(dh, self.seat_id().clone()).ok()
    }

    #[inline]
    pub fn wl_surface(&self) -> &WlSurface {
        &self.data().surface
    }

    pub fn is_active(&self) -> bool {
        self.data().is_active.load(atomic::Ordering::Acquire)
    }

    pub fn activate(&self) {
        self.inhibitor.active();
        self.set_is_active(true);
    }

    pub fn inactivate(&self) {
        self.inhibitor.inactive();
        self.set_is_active(false);
    }
}

#[derive(Debug, Default)]
struct SeatData {
    inhibitors: Rc<RefCell<SeatInhibitors>>,
}

impl SeatData {
    fn get<D>(seat: &Seat<D>) -> &RefCell<Self>
    where
        D: SeatHandler,
        D: 'static,
    {
        seat.user_data()
            .insert_if_missing(|| RefCell::new(SeatData::default()));
        seat.user_data().get().unwrap()
    }

    fn is_inhibited(&self) -> bool {
        self.inhibitors.borrow().is_inhibited()
    }

    fn inhibitor_for_surface(&self, surface: &WlSurface) -> Option<KeyboardShortcutsInhibitor> {
        self.inhibitors.borrow().inhibitor_for_surface(surface).cloned()
    }
}

pub trait KeyboardShortcutsInhibitorSeat {
    fn keyboard_shortcuts_inhibited(&self) -> bool;

    fn keyboard_shortcuts_inhibitor_for_surface(
        &self,
        surface: &WlSurface,
    ) -> Option<KeyboardShortcutsInhibitor>;
}

impl<D> KeyboardShortcutsInhibitorSeat for Seat<D>
where
    D: SeatHandler,
    D: 'static,
{
    fn keyboard_shortcuts_inhibited(&self) -> bool {
        SeatData::get(self).borrow().is_inhibited()
    }

    fn keyboard_shortcuts_inhibitor_for_surface(
        &self,
        surface: &WlSurface,
    ) -> Option<KeyboardShortcutsInhibitor> {
        SeatData::get(self).borrow().inhibitor_for_surface(surface)
    }
}

#[allow(unused_variables)]
pub trait KeyboardShortcutsInhibitHandler {
    fn keyboard_shortcuts_inhibit_state(&mut self) -> &mut KeyboardShortcutsInhibitState;

    fn new_inhibitor(&mut self, inhibitor: KeyboardShortcutsInhibitor) {}

    fn inhibitor_destroyed(&mut self, inhibitor: KeyboardShortcutsInhibitor) {}
}

#[macro_export]
macro_rules! delegate_keyboard_shortcuts_inhibit {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::server::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1: ()
        ] => $crate::wayland::keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::server::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1: ()
        ] => $crate::wayland::keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::server::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1: $crate::wayland::keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitorUserData
        ] => $crate::wayland::keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState);
    };
}
