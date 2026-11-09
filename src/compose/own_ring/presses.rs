// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

use super::start_of;
use crate::state::{AnvilState, Backend};

impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn own_press(&mut self, serial: u32, to: Option<&WlSurface>, placed: bool) {
        if let (Some(to), Some(real)) = (to, self.keys_changed.was) {
            self.own_press_state.tell_was(to, serial, real);
        }
        if !self.ring.by_hand || !placed {
            return;
        }
        if let Some(to) = to {
            self.own_press_state.tell(to, serial);
        }
        self.tell_ring(serial, to);
    }

    fn tell_ring(&mut self, serial: u32, to: Option<&WlSurface>) {
        let running = self.ring.running.as_ref().map(|r| r.0);
        if running.is_none() || to.and_then(start_of) != running {
            return;
        }
        if self.ring.hand.as_mut().is_some_and(|pipe| !crate::own_press::write(pipe, serial)) {
            self.ring.hand = None;
        }
    }

    pub fn on_screen_key_coming(&mut self) -> Option<(u32, u32)> {
        use smithay::utils::SERIAL_COUNTER;
        let to = self.seat.get_keyboard().and_then(|k| k.current_focus()).and_then(|f| {
            use smithay::wayland::seat::WaylandFocus;
            f.wl_surface().map(|s| s.into_owned())
        });
        let to = to?;
        let before: u32 = SERIAL_COUNTER.next_serial().into();

        let after = |n: u32| match n.wrapping_add(1) {
            0 => 1,
            next => next,
        };
        let (first, second) = (after(before), after(after(before)));
        self.own_press_state.tell(&to, first);
        self.own_press_state.tell(&to, second);

        self.tell_ring(first, Some(&to));
        self.tell_ring(second, Some(&to));
        Some((before, second))
    }

    pub fn on_screen_key_passed_on(&mut self, named: Option<(u32, u32)>) {
        let Some((before, last)) = named else { return };

        for _ in 0..4 {
            let taken: u32 = smithay::utils::SERIAL_COUNTER.next_serial().into();
            if taken >= last || taken < before {
                break;
            }
        }
    }

    pub fn pointer_put_by_hand(&self) -> bool {
        self.ring.hand_at == Some(self.pointer.current_location())
    }
}

impl<BackendData: Backend + 'static> crate::protocols::own_press::OwnPressHandler for AnvilState<BackendData> {
    fn own_press_state(&mut self) -> &mut crate::protocols::own_press::OwnPressState {
        &mut self.own_press_state
    }
}
crate::delegate_own_press!(@<BackendData: Backend + 'static> AnvilState<BackendData>);
