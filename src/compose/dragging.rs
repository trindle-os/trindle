// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Things dragged from one program to another.
//!
//! Smithay passes what is dragged between the programs. Two things it
//! leaves to the compositor are done here. While something is dragged no
//! program has the pointer, so none can change its shape: the compositor
//! shows what letting go would do, from the way the program under the
//! pointer has said it would take it. And a drag whose program has gone,
//! or has taken back what it offered (as Escape does in Trindle's own
//! programs), is let go of at once, where it would otherwise stay on the
//! pointer until the button came up. A drag with a finger is let go of
//! the same way, and leaves the pointer's shape alone.

use smithay::input::pointer::{CursorIcon, CursorImageStatus};
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction;
use smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::{IsAlive, SERIAL_COUNTER};
use smithay::wayland::seat::WaylandFocus;

use crate::state::{AnvilState, Backend};

#[derive(Debug)]
pub struct Dragging {
    source: Option<WlDataSource>,
    action: DndAction,
    over: Option<ObjectId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Over {
    Nothing,
    Own,
    Other(DndAction),
}

fn shape(over: Over) -> CursorIcon {
    match over {
        Over::Own => CursorIcon::Grabbing,
        Over::Other(a) if a == DndAction::Copy => CursorIcon::Copy,
        Over::Other(a) if a == DndAction::Move => CursorIcon::Grabbing,
        Over::Other(a) if a == DndAction::Ask => CursorIcon::ContextMenu,
        Over::Other(_) | Over::Nothing => CursorIcon::NoDrop,
    }
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn drag_started(&mut self, source: Option<WlDataSource>) {
        self.dragging = Some(Dragging { source, action: DndAction::empty(), over: None });
    }

    pub fn drag_ended(&mut self) {
        if self.dragging.take().is_some() && matches!(self.cursor_status, CursorImageStatus::Named(_)) {
            self.cursor_status = CursorImageStatus::default_named();
        }
    }

    fn surface_under_pointer(&self) -> Option<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface> {
        let (focus, _) = self.surface_under(self.pointer.current_location())?;
        focus.wl_surface().map(|s| s.into_owned())
    }

    pub fn drag_chose(&mut self, action: DndAction) {
        let over = self.surface_under_pointer().map(|s| s.id());
        if let Some(d) = &mut self.dragging {
            d.action = action;
            d.over = over;
        }
    }

    pub fn show_drag(&mut self) {
        let Some(d) = &self.dragging else { return };
        let Some(source) = d.source.clone() else { return };
        let by_touch = !self.pointer.is_grabbed();
        if !source.alive() {
            if by_touch {
                if let Some(touch) = self.seat.get_touch() {
                    touch.unset_grab(self);
                }
            } else {
                let pointer = self.pointer.clone();
                pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), self.clock.now().as_millis());
            }
            self.drag_ended();
            return;
        }
        if by_touch {
            return;
        }
        if self.capture.away() {
            return;
        }
        let under = self.surface_under_pointer();
        let older = under.as_ref().and_then(|s| self.xdnd_shape_over(s));
        let Some(d) = &mut self.dragging else { return };
        let over = match &under {
            None => Over::Nothing,
            Some(_) if older.is_some() => Over::Other(older.unwrap_or(DndAction::empty())),
            Some(s) if s.id().same_client_as(&source.id()) => Over::Own,
            Some(s) if d.over.as_ref() == Some(&s.id()) => Over::Other(d.action),
            Some(_) => {
                d.action = DndAction::empty();
                Over::Other(DndAction::empty())
            }
        };
        self.cursor_status = CursorImageStatus::Named(shape(over));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pointer_says_what_letting_go_would_do() {
        assert_eq!(shape(Over::Other(DndAction::Copy)), CursorIcon::Copy);
        assert_eq!(shape(Over::Other(DndAction::Move)), CursorIcon::Grabbing);
        assert_eq!(shape(Over::Other(DndAction::Ask)), CursorIcon::ContextMenu);
        assert_eq!(shape(Over::Other(DndAction::empty())), CursorIcon::NoDrop);
        assert_eq!(shape(Over::Nothing), CursorIcon::NoDrop);
        assert_eq!(shape(Over::Own), CursorIcon::Grabbing);
    }
}
