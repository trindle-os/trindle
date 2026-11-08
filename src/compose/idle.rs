// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    delegate_idle_inhibit, delegate_idle_notify,
    desktop::{find_popup_root_surface, layer_map_for_output, WindowSurfaceType},
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    utils::IsAlive,
    wayland::{
        compositor::get_parent,
        idle_inhibit::IdleInhibitHandler,
        idle_notify::{IdleNotifierHandler, IdleNotifierState},
    },
};

use crate::state::{AnvilState, Backend};

impl<BackendData: Backend> AnvilState<BackendData> {

    pub fn notify_activity(&mut self) {
        self.idle_notifier_state.notify_activity(&self.seat);
    }

    pub fn refresh_idle_inhibit(&mut self) {
        self.idle_inhibitors.retain(|s| s.alive());
        let inhibited = self.idle_inhibitors.iter().any(|s| self.is_showing(s)) || self.older_game_playing();
        self.idle_notifier_state.set_is_inhibited(inhibited);
    }

    fn older_game_playing(&self) -> bool {
        if self.is_locked() {
            return false;
        }
        let Some(crate::focus::KeyboardFocusTarget::Window(w)) = self.seat.get_keyboard().and_then(|k| k.current_focus()) else {
            return false;
        };
        w.x11_surface().is_some_and(|x| x.is_fullscreen())
            && self.space.outputs().any(|o| {
                o.user_data().get::<crate::shell::FullscreenSurface>().and_then(|f| f.get()).is_some_and(|f| f.0 == w)
            })
    }

    pub fn refresh_suspended(&mut self) {
        let toplevels = self.xdg_shell_state.toplevel_surfaces().to_vec();
        for toplevel in toplevels {
            let surface = toplevel.wl_surface();
            let drawn = smithay::backend::renderer::utils::with_renderer_surface_state(surface, |s| {
                s.buffer().is_some()
            })
            .unwrap_or(false);
            let suspended = toplevel.with_pending_state(|s| s.states.contains(xdg_toplevel::State::Suspended));
            let change = suspend_change(
                toplevel.is_initial_configure_sent(),
                drawn,
                self.is_showing(surface),
                suspended,
            );
            if let Some(now) = change {
                toplevel.with_pending_state(|s| {
                    if now {
                        s.states.set(xdg_toplevel::State::Suspended);
                    } else {
                        s.states.unset(xdg_toplevel::State::Suspended);
                    }
                });
                toplevel.send_pending_configure();
            }
        }
    }

    fn is_showing(&self, surface: &WlSurface) -> bool {
        let mut root = surface.clone();
        loop {
            if let Some(parent) = get_parent(&root) {
                root = parent;
            } else if let Some(popup) = self.popups.find_popup(&root) {
                match find_popup_root_surface(&popup) {
                    Ok(parent) if parent != root => root = parent,
                    _ => break,
                }
            } else {
                break;
            }
        }
        let root = &root;
        if self.is_locked() {
            return self.space.outputs().any(|o| self.session_lock.surface_on(o) == Some(root));
        }
        let in_window = self.space.elements().any(|w| {
            w.wl_surface().as_deref() == Some(root) && !self.space.outputs_for_element(w).is_empty()
        });
        in_window
            || self.space.outputs().any(|o| {
                layer_map_for_output(o)
                    .layer_for_surface(root, WindowSurfaceType::TOPLEVEL)
                    .is_some()
            })
    }
}

fn suspend_change(configured: bool, drawn: bool, showing: bool, suspended: bool) -> Option<bool> {
    if !configured {
        None
    } else if showing && suspended {
        Some(false)
    } else if !showing && !suspended && drawn {
        Some(true)
    } else {
        None
    }
}

impl<BackendData: Backend + 'static> IdleNotifierHandler for AnvilState<BackendData> {
    fn idle_notifier_state(&mut self) -> &mut IdleNotifierState<Self> {
        &mut self.idle_notifier_state
    }
}
delegate_idle_notify!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> IdleInhibitHandler for AnvilState<BackendData> {
    fn inhibit(&mut self, surface: WlSurface) {
        self.idle_inhibitors.push(surface);
        self.refresh_idle_inhibit();
    }

    fn uninhibit(&mut self, surface: WlSurface) {

        if let Some(i) = self.idle_inhibitors.iter().position(|s| s == &surface) {
            self.idle_inhibitors.remove(i);
        }
        self.refresh_idle_inhibit();
    }
}
delegate_idle_inhibit!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

#[cfg(test)]
mod tests {
    use super::suspend_change;

    #[test]
    fn a_window_out_of_sight_is_suspended() {
        assert_eq!(suspend_change(true, true, false, false), Some(true));
    }

    #[test]
    fn a_window_back_in_sight_wakes() {
        assert_eq!(suspend_change(true, true, true, true), Some(false));

        assert_eq!(suspend_change(true, false, true, true), Some(false));
    }

    #[test]
    fn nothing_changes_when_nothing_moved() {
        assert_eq!(suspend_change(true, true, true, false), None);
        assert_eq!(suspend_change(true, true, false, true), None);
    }

    #[test]
    fn a_new_window_is_not_suspended_before_it_draws() {
        assert_eq!(suspend_change(false, false, false, false), None);
        assert_eq!(suspend_change(false, true, false, false), None);
        assert_eq!(suspend_change(true, false, false, false), None);
    }
}
