
use std::time::Instant;

use crate::toolkit::input::{Button, Input, Keysym};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
use crate::toolkit::widget::{Node, Rect};

const WINDOW: (u32, u32) = (1280, 800);

pub const UNDER_THE_I: (f32, f32) = (THE_I.x + 20.0, THE_I.y + THE_I.h + 22.0);

pub const THE_I: Rect = Rect { x: 400.0, y: 60.0, w: 28.0, h: 28.0 };

pub fn escape() -> Input {
    Input::Key { sym: Keysym::Escape, text: None, mods: Default::default(), repeat: false }
}

fn pressed<M: Clone + 'static>(view: Node<M>, at: (f32, f32)) -> Vec<M> {
    let mut ui = Ui::new(view, Theme::new(false, "#fbc711"));
    ui.lay_at(WINDOW.0, WINDOW.1);
    let now = Instant::now();
    ui.input(&Input::PointerIn { x: at.0, y: at.1 }, now);
    let sent = ui.input(&Input::Press { button: Button::Left, x: at.0, y: at.1 }, now).messages;
    ui.input(&Input::Release { button: Button::Left, x: at.0, y: at.1 }, now);
    sent
}

pub fn shuts_like_every_other<P, M: Clone + 'static>(
    name: &str,
    inside: (f32, f32),
    p: &mut P,
    open: impl Fn(&mut P),
    is_open: impl Fn(&P) -> bool,
    view: impl Fn(&P) -> Node<M>,
    update: impl Fn(&mut P, M),
    on_escape: impl Fn(&P, &Input) -> Option<M>,
) {
    let i = THE_I;
    let outside = [("the page, bottom left", (6.0, 794.0)), ("the middle of the page", (1000.0, 400.0)), ("the i itself", (i.x + i.w / 2.0, i.y + i.h / 2.0))];
    open(p);
    assert!(is_open(p), "{name}: the i would not open");
    for m in pressed(view(p), inside) {
        update(p, m);
    }
    assert!(is_open(p), "{name}: a press inside the words shut them");
    for (where_, at) in outside {
        open(p);
        for m in pressed(view(p), at) {
            update(p, m);
        }
        assert!(!is_open(p), "{name}: a press on {where_} did not shut the words");
    }
    open(p);
    let m = on_escape(p, &escape()).unwrap_or_else(|| panic!("{name}: Escape did nothing"));
    update(p, m);
    assert!(!is_open(p), "{name}: Escape did not shut the words");
}
