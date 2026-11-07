//! A screenshot pinned on the screen.

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::Duration;

use tiny_skia::{FillRule, FilterQuality, Mask, Pixmap, PixmapPaint, Transform};

use super::area::Pt;
use crate::toolkit::event::Loop;
use crate::toolkit::input::{Cursor, Input, Keysym};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::ui::{rounded, App, Shown};
use crate::toolkit::widget::{self, button, card, drawing, layers, placed, Length, Node, Pointer};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel, Window};

const FIRST_SHARE: f64 = 0.4;
const INSET: f64 = 32.0;
const STEP: f64 = 1.1;
const KEPT: f64 = 48.0;
const CLOSE_INSET: f32 = 8.0;
const ROOM: f32 = 16.0;

pub fn shown_size(picture: Pt, view: Pt, zoom: f64) -> Pt {
    let fit = (view.0 * FIRST_SHARE / picture.0).min(view.1 * FIRST_SHARE / picture.1).min(1.0);
    let most = (view.0 / picture.0).min(view.1 / picture.1);
    let least = 64.0 / picture.0.max(picture.1);
    let s = (fit * zoom).clamp(least.min(most), most);
    ((picture.0 * s).round(), (picture.1 * s).round())
}

#[derive(Clone, Debug)]
pub enum Msg {
    Pointer(Pointer),
    Zoom(bool),
    Close,
}

pub struct Pin {
    still: Rc<Pixmap>,
    picture: Pt,
    view: Pt,
    zoom: f64,
    place: Pt,
    start: Option<(f32, f32)>,
    events: Loop,
    win: Rc<RefCell<Weak<Window>>>,
    gone: Rc<dyn Fn()>,
}

impl Pin {
    fn later(&self, call: impl FnOnce(&Window) + 'static) {
        let win = self.win.borrow().clone();
        self.events.after(Duration::ZERO, move || {
            if let Some(w) = win.upgrade() {
                call(&w);
            }
        });
    }
}

impl App for Pin {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Pointer(Pointer::Down(x, y)) => self.start = Some((x, y)),
            Msg::Pointer(Pointer::Move(x, y)) => {
                let Some((x0, y0)) = self.start else { return };
                let (px, py) = self.place;
                self.place = ((px + (x - x0) as f64).clamp(0.0, (self.view.0 - KEPT).max(0.0)),
                              (py + (y - y0) as f64).clamp(0.0, (self.view.1 - KEPT).max(0.0)));
                let (x, y) = (self.place.0.round() as i32, self.place.1.round() as i32);
                self.later(move |w| w.set_margins([y, 0, 0, x]));
            }
            Msg::Pointer(Pointer::Up(..)) => self.start = None,
            Msg::Pointer(_) => {}
            Msg::Zoom(bigger) => {
                self.zoom = if bigger { self.zoom * STEP } else { self.zoom / STEP };
                let (w, h) = shown_size(self.picture, self.view, self.zoom);
                self.later(move |win| win.resize((w as u32, h as u32)));
            }
            Msg::Close => (self.gone)(),
        }
    }

    fn view(&self) -> Node<Msg> {
        let still = self.still.clone();
        let shown = drawing((0.0, 0.0), t!("screenshot-pinned"), move |canvas, pen| {
            let (cw, ch) = (canvas.width() as f32, canvas.height() as f32);
            let room = (ROOM * pen.scale).min(cw / 4.0).min(ch / 4.0);
            let (w, h) = ((cw - 2.0 * room).max(1.0), (ch - 2.0 * room).max(1.0));
            let radius = theme::sheet().corner_of(crate::toolkit::theme::Part::Button) * pen.scale;
            let at = widget::Rect { x: room, y: room, w, h };
            let k = pen.scale.max(0.01);
            super::eye_dropper::float_shadow(canvas, pen.scale, (at.x / k, at.y / k, at.w / k, at.h / k),
                                             radius / k, theme::sheet().label_shadow);
            let Some(edge) = rounded(at, radius) else { return };
            let Some(mut clip) = Mask::new(canvas.width(), canvas.height()) else { return };
            clip.fill_path(&edge, FillRule::Winding, true, Transform::identity());
            let fit = Transform::from_translate(room, room).pre_scale(w / still.width().max(1) as f32, h / still.height().max(1) as f32);
            let good = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
            canvas.draw_pixmap(0, 0, Pixmap::as_ref(&still), &good, fit, Some(&clip));
        })
        .width(Length::Fill).height(Length::Fill).on_pointer(Msg::Pointer).cursor_with(|_, _| Cursor::Move);
        let shut = placed(card(button("window-close-symbolic", t!("screenshot-close")).on_press(Msg::Close)).width(Length::Shrink).floats(),
                          |(w, _), (aw, _)| (aw - w - ROOM - CLOSE_INSET, ROOM + CLOSE_INSET));
        layers(vec![shown, shut])
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::Scroll { y, .. } if *y != 0.0 => Some(Msg::Zoom(*y < 0.0)),
            Input::Key { sym: Keysym::Escape, .. } => Some(Msg::Close),
            _ => None,
        }
    }
}

pub fn show(events: &Loop, display: &Rc<Display>, still: Pixmap, picture: Pt, view: Pt, gone: impl Fn() + 'static) -> Option<Shown<Pin>> {
    let (w, h) = shown_size(picture, view, 1.0);
    let place = ((view.0 - w - INSET).max(0.0), INSET);
    let gone: Rc<dyn Fn()> = Rc::new(gone);
    let win = Rc::new(RefCell::new(Weak::new()));
    let kind = Kind::Panel(Panel {
        layer: Layer::Top, edges: Edges::TOP | Edges::LEFT, keep: 0,
        margins: [place.1.round() as i32, 0, 0, place.0.round() as i32], keys: Keys::OnClick,
        name: "trindle-screenshot-pin".into(), screen: None,
    });
    let pin = Pin { still: Rc::new(still), picture, view, zoom: 1.0, place, start: None, events: events.clone(),
                    win: win.clone(), gone: gone.clone() };
    let shown = Shown::open(events, display, kind, (w as u32, h as u32), pin)?;
    *win.borrow_mut() = Rc::downgrade(&shown.window);
    shown.see_through();
    shown.window.on_close(move || gone());
    Some(shown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pinned_screenshot_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        use crate::toolkit::ui::App;
        let events = Loop::new();
        let stills: Vec<(&str, Rc<Pixmap>)> = ["dark", "light", "of many colours"].into_iter().map(|kind| {
            let mut still = Pixmap::new(400, 300).unwrap();
            for (i, px) in still.pixels_mut().iter_mut().enumerate() {
                let (x, y) = ((i % 400) as u32, (i / 400) as u32);
                let (r, g, b) = match kind {
                    "dark" => (8, 10, 12),
                    "light" => (250, 250, 246),
                    _ => ((x * 255 / 400) as u8, (y * 255 / 300) as u8, 140),
                };
                *px = tiny_skia::PremultipliedColorU8::from_rgba(r, g, b, 255).unwrap();
            }
            (kind, Rc::new(still))
        }).collect();
        drawn::none(drawn::in_every_look("a pinned screenshot", 1, || {
            for (kind, still) in &stills {
                let pin = Pin { still: still.clone(), picture: (400.0, 300.0), view: (1920.0, 1080.0), zoom: 1.0, place: (0.0, 0.0), start: None,
                                events: events.clone(), win: Rc::new(RefCell::new(Weak::new())), gone: Rc::new(|| {}) };
                drawn::window(kind, pin.view(), Theme::chosen(), (400, 300), true);
            }
        }));
    }

    #[test]
    fn a_big_picture_first_shows_small_and_a_small_one_as_it_is() {
        assert_eq!(shown_size((1920.0, 1080.0), (1920.0, 1080.0), 1.0), (768.0, 432.0));
        assert_eq!(shown_size((300.0, 200.0), (1920.0, 1080.0), 1.0), (300.0, 200.0));
    }

    #[test]
    fn it_grows_no_bigger_than_the_screen_and_no_smaller_than_a_thumbnail() {
        assert_eq!(shown_size((1920.0, 1080.0), (1920.0, 1080.0), 50.0), (1920.0, 1080.0));
        assert_eq!(shown_size((300.0, 200.0), (1920.0, 1080.0), 0.001), (64.0, 43.0));
    }
}
