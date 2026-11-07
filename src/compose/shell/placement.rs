// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Where windows go: first placement, dialogs over their owners, and fitting them to the screens.

use super::{FullscreenSurface, WindowElement};

use smithay::{
    desktop::{layer_map_for_output, space::SpaceElement, Space},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point, Rectangle, Size},
};

use crate::state::{AnvilState, Backend};

const CASCADE: i32 = 32;

struct Placed;

fn parent_of(space: &Space<WindowElement>, window: &WindowElement) -> Option<WindowElement> {
    if let Some(toplevel) = window.0.toplevel() {
        let parent = toplevel.parent()?;
        return space.elements().find(|w| w.wl_surface().as_deref() == Some(&parent)).cloned();
    }
    if let Some(x11) = window.0.x11_surface() {
        let parent = x11.is_transient_for()?;
        return space
            .elements()
            .find(|w| w.0.x11_surface().is_some_and(|p| p.window_id() == parent))
            .cloned();
    }
    None
}

pub fn place_keeping_order(space: &mut Space<WindowElement>, window: WindowElement, loc: Point<i32, Logical>) {
    let order: Vec<WindowElement> = space.elements().cloned().collect();
    space.map_element(window, loc, false);
    for w in &order {
        space.raise_element(w, false);
    }
}

pub fn dialogs_of(space: &Space<WindowElement>, window: &WindowElement) -> Vec<WindowElement> {
    let mut found: Vec<WindowElement> = Vec::new();
    let mut owners = vec![window.clone()];
    while let Some(owner) = owners.pop() {
        for w in space.elements() {
            if parent_of(space, w).as_ref() == Some(&owner) && w != window && !found.contains(w) {
                found.push(w.clone());
                owners.push(w.clone());
            }
        }
    }
    found
}

pub(super) fn room_inside(area: Size<i32, Logical>, extra: Size<i32, Logical>) -> Size<i32, Logical> {
    ((area.w - extra.w).max(1), (area.h - extra.h).max(1)).into()
}

pub(super) fn place_new_window(
    space: &mut Space<WindowElement>,
    pointer_location: Point<f64, Logical>,
    window: &WindowElement,
    activate: bool,
) -> Rectangle<i32, Logical> {
    let area = free_area_under(space, pointer_location);

    #[allow(irrefutable_let_patterns)]
    if let Some(toplevel) = window.0.toplevel() {
        let room = room_inside(area.size, window.deco_extra());
        toplevel.with_pending_state(|state| {
            state.bounds = Some(room);
        });
    }

    let size = window.geometry().size;
    let parent = parent_of(space, window)
        .and_then(|p| Some(Rectangle::new(space.element_location(&p)?, p.geometry().size)));
    let mut loc = match parent {
        Some(over) => centred_in(over, size),
        None => stepped_clear_of_others(space, window, area),
    };
    loc.x = loc.x.min(area.loc.x + area.size.w - size.w).max(area.loc.x);
    loc.y = loc.y.min(area.loc.y + area.size.h - size.h).max(area.loc.y);

    if activate {
        space.map_element(window.clone(), loc, true);
    } else {
        place_keeping_order(space, window.clone(), loc);
    }
    area
}

fn free_area_under(space: &Space<WindowElement>, pointer_location: Point<f64, Logical>) -> Rectangle<i32, Logical> {
    let output = space
        .output_under(pointer_location)
        .next()
        .or_else(|| space.outputs().next())
        .cloned();
    output
        .and_then(|o| {
            let geo = space.output_geometry(&o)?;
            let map = layer_map_for_output(&o);
            let zone = map.non_exclusive_zone();
            Some(Rectangle::new(geo.loc + zone.loc, zone.size))
        })
        .unwrap_or_else(|| Rectangle::from_size((800, 800).into()))
}

fn centred_in(over: Rectangle<i32, Logical>, size: Size<i32, Logical>) -> Point<i32, Logical> {
    (over.loc.x + (over.size.w - size.w) / 2, over.loc.y + (over.size.h - size.h) / 2).into()
}

fn stepped_clear_of_others(space: &Space<WindowElement>, window: &WindowElement, area: Rectangle<i32, Logical>) -> Point<i32, Logical> {
    let size = window.geometry().size;
    let mut loc = centred_in(area, size);
    let taken = |at: Point<i32, Logical>| {
        space.elements().filter(|w| *w != window).any(|w| {
            space.element_location(w)
                .is_some_and(|l| (l.x - at.x).abs() < CASCADE / 4 && (l.y - at.y).abs() < CASCADE / 4)
        })
    };
    let mut steps = 0;
    while taken(loc) && steps < 32 {
        loc += Point::from((CASCADE, CASCADE));
        if loc.x + size.w > area.loc.x + area.size.w || loc.y + size.h > area.loc.y + area.size.h {
            loc = area.loc;
        }
        steps += 1;
    }
    loc
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub(super) fn place_when_ready(&mut self, window: &WindowElement) {
        if window.user_data().get::<Placed>().is_some() || window.0.geometry().is_empty() {
            return;
        }
        window.user_data().insert_if_missing(|| Placed);
        if !window.is_maximized() && !window.is_fullscreen() {
            let area = place_new_window(&mut self.space, self.pointer.current_location(), window, true);
            let gap = self.config.gap;
            let room = room_inside((area.size.w - 2 * gap, area.size.h - 2 * gap).into(), window.deco_extra());
            let have = window.client_size();
            if have.w > room.w || have.h > room.h {
                if let Some(toplevel) = window.0.toplevel() {
                    toplevel.with_pending_state(|state| state.size = Some((have.w.min(room.w), have.h.min(room.h)).into()));
                    toplevel.send_pending_configure();
                }
                self.space.map_element(window.clone(), area.loc + Point::from((gap, gap)), true);
            }
        }
        if window.wl_surface().is_some_and(|s| crate::trusted::window_kept_for_the_person(&s)) {
            self.leave_full_screen_for_a_question();
            self.drop_window_grabs();
            if let Some(keyboard) = self.seat.get_keyboard() {
                if keyboard.is_grabbed() {
                    keyboard.unset_grab(self);
                }
            }
        }
        if !self.opens_behind_full_screen(window) {
            self.focus_window(window);
        }
    }

    pub(crate) fn leave_full_screen_for_a_question(&mut self) {
        let filling: Vec<WlSurface> = self
            .space
            .outputs()
            .filter_map(|o| o.user_data().get::<FullscreenSurface>().and_then(|f| f.get()))
            .filter_map(|w| w.wl_surface().map(|s| s.into_owned()))
            .collect();
        for surface in filling {
            crate::protocols::windows::WindowsHandler::change_window(
                self,
                surface,
                crate::protocols::windows::Change::Fullscreen,
                false,
            );
        }
        for output in self.space.outputs() {
            if let Some(f) = output.user_data().get::<FullscreenSurface>() {
                f.clear();
            }
        }
    }
}

pub fn fixup_positions(space: &mut Space<WindowElement>, pointer_location: Point<f64, Logical>) {
    let mut offset = Point::<i32, Logical>::from((0, 0));
    for output in space.outputs().cloned().collect::<Vec<_>>().into_iter() {
        let size = space
            .output_geometry(&output)
            .map(|geo| geo.size)
            .unwrap_or_else(|| Size::from((0, 0)));
        space.map_output(&output, offset);
        layer_map_for_output(&output).arrange();
        offset.x += size.w;
    }

    let mut orphaned_windows = Vec::new();
    let outputs = space
        .outputs()
        .flat_map(|o| {
            let geo = space.output_geometry(o)?;
            let map = layer_map_for_output(o);
            let zone = map.non_exclusive_zone();
            Some(Rectangle::new(geo.loc + zone.loc, zone.size))
        })
        .collect::<Vec<_>>();
    for window in space.elements() {
        let window_location = match space.element_location(window) {
            Some(loc) => loc,
            None => continue,
        };
        let geo_loc = window.bbox().loc + window_location;

        if !outputs.iter().any(|o_geo| o_geo.contains(geo_loc)) {
            orphaned_windows.push(window.clone());
        }
    }
    for window in orphaned_windows.into_iter() {
        place_new_window(space, pointer_location, &window, false);
    }

    let mut moves = Vec::new();
    for window in space.elements() {
        let Some(at) = space.element_location(window) else { continue };
        let size = window.geometry().size;
        let Some(zone) = outputs.iter().find(|z| z.contains(at)) else { continue };
        let right = (zone.loc.x + zone.size.w - size.w).max(zone.loc.x);
        let bottom = (zone.loc.y + zone.size.h - size.h).max(zone.loc.y);
        let to = Point::from((at.x.min(right), at.y.min(bottom)));
        if to != at {
            moves.push((window.clone(), to));
        }
    }
    for (window, to) in moves {
        place_keeping_order(space, window, to);
    }
}

#[cfg(test)]
mod order_tests {
    #[test]
    fn nothing_but_the_helper_maps_a_shown_window_without_choosing_to_raise_it() {
        let mut found = Vec::new();
        let mut dirs = vec![std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/compose")];
        while let Some(dir) = dirs.pop() {
            for e in std::fs::read_dir(&dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    dirs.push(p);
                } else if p.extension().is_some_and(|x| x == "rs") {
                    let text = std::fs::read_to_string(&p).unwrap();
                    let code = text.split("#[cfg(test)]").next().unwrap();
                    for (n, l) in code.lines().enumerate() {
                        if l.contains(".map_element(") && l.trim_end().ends_with("false);") {
                            found.push(format!("{}:{}", p.display(), n + 1));
                        }
                    }
                }
            }
        }
        found.sort();
        let helper = found.iter().filter(|f| f.contains("/shell/placement.rs:")).count();
        assert_eq!(helper, 1, "the helper is the only such call in placement.rs: {found:?}");
        assert!(found.len() - helper <= 2, "moves that raise a window by accident: {found:?}");
    }
}
