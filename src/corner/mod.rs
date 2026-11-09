//! A finger's spot in each top corner on a touch screen: a tap there opens
//! the ring, as a double click at the very edge of the screen does with a pointer
//! (desktop_double in compose/input_handler/). The pointer reaching a corner
//! opens nothing. The spots are at 1% opacity, as they always were, so they
//! are never taken for empty glass. They lie on the Top layer, under a
//! full-screen window, so no tap over one opens the ring (17
//! October 2026: full screen never opens the ring from a click).

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use tiny_skia::{FillRule, Paint, Transform};

use crate::common::run::spawn;
use crate::ring::ctx::Ctx;
use crate::toolkit::colour::Colour;
use crate::toolkit::input::Input;
use crate::toolkit::theme::{self, Theme};
use crate::toolkit::ui::rounded;
use crate::toolkit::widget::Rect;
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel, Window};

const SIZE: i32 = 8;

const REACH: i32 = 120;

const CORNERS: [&str; 2] = ["top-left", "top-right"];

const TOUCH_SIZE: i32 = 44;
const DEBOUNCE_MS: u128 = 1000;

const FAINT: Colour = Colour { r: 0.0, g: 0.0, b: 0.0, a: 0.01 };

const MARK_STRENGTH: f32 = 0.10;

const MARK_RADIUS: f32 = 22.0;

fn ring_just_closed() -> bool {
    std::env::var_os("XDG_RUNTIME_DIR")
        .and_then(|d| std::fs::metadata(std::path::Path::new(&d).join("trindle-ring-closed")).ok())
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.elapsed().ok())
        .is_some_and(|e| e.as_millis() < 1500)
}

fn edges(place: &str) -> Edges {
    match place {
        "top-right" => Edges::TOP | Edges::RIGHT,
        "bottom-left" => Edges::BOTTOM | Edges::LEFT,
        "bottom-right" => Edges::BOTTOM | Edges::RIGHT,
        _ => Edges::TOP | Edges::LEFT,
    }
}

fn the_edges(place: &str, w: i32, h: i32, thick: i32) -> [[i32; 4]; 2] {
    let (right, bottom) = (place.contains("right"), place.contains("bottom"));
    let down = [if right { w - thick } else { 0 }, 0, thick, h];
    let across = [0, if bottom { h - thick } else { 0 }, w, thick];
    [down, across]
}

fn corner(ctx: &Rc<Ctx>, place: &'static str, namespace: &str, command: &'static [&'static str]) -> Option<Window> {
    let touch = ctx.display.has_touch();
    let thick = if touch { TOUCH_SIZE } else { SIZE };
    let kind = Kind::Panel(Panel {
        layer: Layer::Top,
        edges: edges(place),
        keep: -1,
        margins: [0; 4],
        keys: Keys::Never,
        name: namespace.into(),
        screen: None,
    });
    let win = ctx.display.open(kind, (REACH as u32, REACH as u32), move |canvas, scale| {
        canvas.fill(FAINT.to_skia());
        if !touch {
            return;
        }

        let (right, bottom) = (place.contains("right"), place.contains("bottom"));
        let (t, reach, r) = (thick as f32, REACH as f32, MARK_RADIUS);
        let x = if right { reach - t } else { -r };
        let y = if bottom { reach - t } else { -r };
        let Some(path) = rounded(Rect { x, y, w: t + r, h: t + r }, r) else { return };
        let mut paint = Paint::default();
        paint.set_color(theme::alpha(Theme::chosen().accent, MARK_STRENGTH).to_skia());
        paint.anti_alias = true;
        canvas.fill_path(&path, &paint, FillRule::Winding, Transform::from_scale(scale, scale), None);
    })?;
    win.set_input_region(Some(&the_edges(place, REACH, REACH, thick)));
    let last: Cell<Option<Instant>> = Cell::new(None);

    win.on_input(move |input| {
        if !matches!(input, Input::TouchUp { .. }) {
            return;
        }
        if last.get().is_some_and(|t| t.elapsed().as_millis() < DEBOUNCE_MS) || ring_just_closed() {
            return;
        }
        last.set(Some(Instant::now()));
        spawn(command);
    });
    Some(win)
}

thread_local! {

    static MADE: RefCell<Vec<Window>> = const { RefCell::new(Vec::new()) };

    static HEARING: Cell<bool> = const { Cell::new(false) };
}

pub fn start(ctx: &Rc<Ctx>) {
    if !HEARING.with(|h| h.replace(true)) {
        let c = ctx.clone();
        ctx.display.on_screens(move || build(&c));
    }
    build(ctx);
}

fn build(ctx: &Rc<Ctx>) {

    MADE.with(|m| m.borrow_mut().clear());
    let mut made = Vec::new();

    if ctx.display.has_touch() {
        for place in CORNERS {
            made.extend(corner(ctx, place, "trindle-corner", &["trindle-ring"]));
        }
    }
    MADE.with(|m| *m.borrow_mut() = made);
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_the_edges_take_the_finger() {
        assert_eq!(super::the_edges("top-left", 120, 120, 44), [[0, 0, 44, 120], [0, 0, 120, 44]]);
        assert_eq!(super::the_edges("bottom-right", 120, 120, 44), [[76, 0, 44, 120], [0, 76, 120, 44]]);
    }

    #[test]
    fn no_corner_spot_lies_over_a_full_screen_window() {

        let code = crate::common::source::code(include_str!("mod.rs"));
        assert!(!code.contains(concat!("Layer::", "Overlay")), "a corner spot over a full-screen window would open the ring from a tap");
    }
}
