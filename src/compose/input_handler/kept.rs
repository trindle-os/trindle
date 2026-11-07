// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! What only the person may press: questions on a panel and the windows and menus kept for them.

use crate::{focus::PointerFocusTarget, state::Backend, AnvilState};

use smithay::{desktop::{layer_map_for_output, WindowSurfaceType}, wayland::seat::WaylandFocus};

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(crate) fn only_the_person_answers(&self) -> bool {
        self.question_on_a_panel() || self.keys_kept_for_the_person()
    }

    pub(super) fn roots_question(&self) -> bool {
        let shown = self.space.outputs().any(|o| layer_map_for_output(o).layers().any(crate::trusted::counts));
        let focused = self.seat.get_keyboard().and_then(|k| k.current_focus())
            .is_some_and(|held| held.wl_surface().is_some_and(|s| crate::trusted::is_trusted(&s)));
        shown || focused
    }

    pub(super) fn say_refused(&mut self, did: &crate::virtual_input::Did) {
        use crate::virtual_input::Did;
        let question = self.space.outputs().find_map(|o| {
            layer_map_for_output(o).layers().rev().find(|l| crate::trusted::asks(l)).map(|l| l.wl_surface().clone())
        });
        let to = question.or_else(|| match did {
            Did::Key(_) => self.seat.get_keyboard().and_then(|k| k.current_focus()).and_then(|f| f.wl_surface().map(|s| s.into_owned())),
            _ => match self.surface_under(self.pointer.current_location()).map(|under| under.0).or_else(|| self.pointer.current_focus()) {
                Some(PointerFocusTarget::WlSurface(surface)) => Some(surface),
                _ => None,
            },
        });
        if let Some(to) = to {
            self.own_press_state.refused(&to, std::time::Instant::now());
        }
    }

    pub(super) fn question_on_a_panel(&self) -> bool {
        let shown = self.space.outputs().any(|o| layer_map_for_output(o).layers().any(crate::trusted::asks));
        let focused = self.seat.get_keyboard().and_then(|k| k.current_focus())
            .is_some_and(|held| held.wl_surface().is_some_and(|s| crate::trusted::is_trusted(&s)));
        shown || focused
    }

    fn keys_kept_for_the_person(&self) -> bool {
        let held = self.seat.get_keyboard().and_then(|k| k.current_focus())
            .is_some_and(|held| held.wl_surface().is_some_and(|s| self.kept_for_the_person(&s)));
        held || self.whole_keyboard_panels().iter().any(crate::trusted::panel_kept_for_the_person)
    }

    fn click_kept_for_the_person(&self) -> bool {
        let of = |target: Option<PointerFocusTarget>| match target {
            Some(PointerFocusTarget::WlSurface(surface)) => self.kept_for_the_person(&surface),
            _ => false,
        };
        of(self.surface_under(self.pointer.current_location()).map(|under| under.0)) || of(self.pointer.current_focus())
    }

    pub(crate) fn not_for_a_made_up_press(&self, did: &crate::virtual_input::Did) -> bool {
        use crate::virtual_input::Did;
        did.is_press()
            && (self.question_on_a_panel()
                || match did {
                    Did::Key(_) => self.keys_kept_for_the_person(),
                    Did::Button(_) => self.click_kept_for_the_person(),
                    _ => false,
                })
    }

    pub(crate) fn kept_for_the_person(&self, surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface) -> bool {
        let top = |of: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface| {
            let mut root = of.clone();
            while let Some(parent) = smithay::wayland::compositor::get_parent(&root) {
                root = parent;
            }
            root
        };
        let mut root = top(surface);
        if let Some(menu) = self.popups.find_popup(&root) {
            match smithay::desktop::find_popup_root_surface(&menu) {
                Ok(opened_from) => root = top(&opened_from),
                Err(_) => return crate::trusted::outside_every_box(&root),
            }
        }
        let panel = self.space.outputs().find_map(|o| layer_map_for_output(o).layer_for_surface(&root, WindowSurfaceType::TOPLEVEL).cloned());
        match panel {
            Some(panel) => crate::trusted::panel_kept_for_the_person(&panel),
            None => crate::trusted::window_kept_for_the_person(&root),
        }
    }

}
