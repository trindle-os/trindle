//! The magnifier: a circle of magnified screen that follows your finger.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use tiny_skia::{FillRule, FilterQuality, Mask, Paint, PathBuilder, Pixmap, PixmapPaint, Transform};

use crate::common::capture;
use crate::common::run::on_thread;
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, drawing, layers, placed, Length, Node, Pen, Pointer, Rect};
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};

use super::ctx::Ctx;
use crate::toolkit::t;

const TIMES: f32 = 2.6;

const SIZE: f32 = 150.0;

const NOTE_SECONDS: u64 = 5;
fn note_words() -> &'static str {
    t!("ring-magnify-hint")
}

const RIM: f32 = 4.0;

const OFF_FOOT: f32 = super::notice::FOOT + super::notice::HEIGHT + 16.0;

const PLACE_WAIT: Duration = Duration::from_millis(16);
const PLACE_TRIES: u32 = 60;

thread_local! {
    static SHOWN: RefCell<Option<Rc<Shown<Magnifier>>>> = const { RefCell::new(None) };
}

pub fn on() -> bool {
    SHOWN.with(|s| s.borrow().is_some())
}

pub fn stop() {

    let gone = SHOWN.with(|s| s.borrow_mut().take());
    drop(gone);
}

#[derive(Clone)]
enum Msg {
    Pointer(Pointer),
    Off,
}

struct Magnifier {
    lp: Loop,

    picture: Option<Rc<Pixmap>>,

    held: Option<(f32, f32)>,
    note: bool,
    at: Option<(i32, i32, i32, i32)>,
    finished: Rc<dyn Fn()>,
}

impl App for Magnifier {
    type Msg = Msg;

    fn update(&mut self, m: Msg) {
        match m {
            Msg::Pointer(Pointer::Down(x, y)) => self.held = Some((x, y)),
            Msg::Pointer(Pointer::Move(x, y)) if self.held.is_some() => self.held = Some((x, y)),
            Msg::Pointer(Pointer::Up(..)) => self.held = None,
            Msg::Pointer(_) => {}

            Msg::Off => {
                let finished = self.finished.clone();
                self.lp.after(Duration::ZERO, move || {
                    stop();
                    finished();
                });
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let (picture, held) = (self.picture.clone(), self.held);
        let glass = drawing((0.0, 0.0), t!("ring-magnify-name"), move |canvas, pen| {
            let (Some(picture), Some((x, y))) = (picture.as_ref(), held) else { return };
            paint(canvas, pen, picture, (x * pen.scale, y * pen.scale));
        })
        .width(Length::Fill).height(Length::Fill).on_pointer(Msg::Pointer);

        let mut parts = vec![glass];

        if self.note {
            let pill = drawing((0.0, super::notice::BAND), note_words(), |pm, pen| super::notice::paint_pill(pm, pen, note_words()))
                .width(Length::Fill)
                .height(Length::Fixed(super::notice::BAND));
            let (foot, room, tall) = (super::notice::FOOT, super::notice::ROOM, super::notice::HEIGHT);
            parts.push(placed(pill, move |_, (_, ah)| (0.0, ah - foot - room - tall)));
        }

        let off = button("zoom-in-symbolic", t!("ring-magnify-stop")).on(true).on_press(Msg::Off);
        parts.push(match self.at {
            Some((x, y, w, h)) => {
                let off = off.width(Length::Fixed(w as f32)).height(Length::Fixed(h as f32));
                placed(off, move |_, _| (x as f32, y as f32))
            }

            None => placed(off, |(w, h), (aw, ah)| ((aw - w) / 2.0, ah - h - OFF_FOOT)),
        });
        layers(parts)
    }
}

fn paint(canvas: &mut tiny_skia::PixmapMut, pen: &Pen, picture: &Pixmap, (x, y): (f32, f32)) {
    let scale = pen.scale;
    let (glass, rim) = (SIZE * scale, (SIZE + RIM) * scale);
    let Some(circle) = PathBuilder::from_circle(x, y, glass) else { return };
    let whole = Rect { x: x - rim, y: y - rim, w: 2.0 * rim, h: 2.0 * rim };
    super::notice::float_shadow(canvas, pen, whole, rim);
    if let Some(frame) = PathBuilder::from_circle(x, y, rim) {
        let mut ink = Paint { anti_alias: true, ..Paint::default() };
        ink.set_color(pen.theme.raised.to_skia());
        canvas.fill_path(&frame, &ink, FillRule::Winding, Transform::identity(), None);
    }
    let Some(mut clip) = Mask::new(canvas.width(), canvas.height()) else { return };
    clip.fill_path(&circle, FillRule::Winding, true, Transform::identity());

    let k = (picture.width() as f32 / canvas.width().max(1) as f32).max(0.0001);
    let grow = TIMES / k;
    let t = Transform::from_translate(x, y).pre_scale(grow, grow).pre_translate(-x * k, -y * k);
    let good = PixmapPaint { quality: FilterQuality::Bilinear, ..PixmapPaint::default() };
    canvas.draw_pixmap(0, 0, picture.as_ref(), &good, t, Some(&clip));
}

fn picture(shot: &capture::Picture) -> Option<Pixmap> {
    let (w, h) = (shot.width.max(0) as u32, shot.height.max(0) as u32);
    let mut out = Pixmap::new(w, h)?;
    let row = w as usize * 4;
    for (i, line) in out.data_mut().chunks_exact_mut(row).enumerate() {
        let from = shot.data.get(i * shot.stride..i * shot.stride + row)?;
        line.copy_from_slice(from);
    }
    Some(out)
}

pub fn start(ctx: &Rc<Ctx>, at: Option<(i32, i32, i32, i32)>, finished: impl Fn() + 'static) {
    stop();
    let finished: Rc<dyn Fn()> = Rc::new(finished);
    let kind = Kind::Panel(Panel {
        layer: Layer::Overlay, edges: Edges::ALL, keep: -1, margins: [0; 4], keys: Keys::Never,
        name: "trindle-magnify".into(), screen: None,
    });
    let app = Magnifier { lp: ctx.lp.clone(), picture: None, held: None, note: true, at, finished: finished.clone() };
    let Some(shown) = Shown::open(&ctx.lp, &ctx.display, kind, (0, 0), app) else {
        eprintln!("trindle-ring: magnifier: no window");
        finished();
        return;
    };
    shown.see_through();
    let shown = Rc::new(shown);
    SHOWN.with(|s| *s.borrow_mut() = Some(shown.clone()));

    let note = Rc::downgrade(&shown);
    ctx.lp.after(Duration::from_secs(NOTE_SECONDS), move || {
        if let Some(s) = note.upgrade() {
            s.change(|m| m.note = false);
        }
    });

    let (me, display) = (Rc::downgrade(&shown), ctx.display.clone());
    let mut tries = 0;
    ctx.lp.every(PLACE_WAIT, move || {
        let Some(s) = me.upgrade() else { return Flow::Stop };
        tries += 1;
        let place = s.window.place().or_else(|| if tries >= PLACE_TRIES { display.places().first().copied() } else { None });
        if place.is_none() && tries < PLACE_TRIES {
            return Flow::Continue;
        }
        let area = place.map(|[x, y, width, height]| capture::Area { x, y, width, height });
        let (me, finished) = (Rc::downgrade(&s), finished.clone());
        on_thread(move || capture::take(area, false), move |shot| {
            let Some(s) = me.upgrade() else { return };
            match shot.map_err(|e| e.to_string()).and_then(|p| picture(&p).ok_or_else(|| "the picture was the wrong size".to_string())) {
                Ok(p) => s.change(|m| m.picture = Some(Rc::new(p))),
                Err(e) => {
                    eprintln!("trindle-ring: magnifier: {e}");

                    stop();
                    finished();
                }
            }
        });
        Flow::Stop
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_is_copied_row_by_row_past_its_padding() {

        let data: Vec<u8> = (0..24u8).collect();
        let shot = capture::Picture { width: 2, height: 2, stride: 12, data };
        let p = picture(&shot).unwrap();
        assert_eq!(p.data(), &[0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 15, 16, 17, 18, 19]);
    }

    #[test]
    fn the_magnifier_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let mut shot = Pixmap::new(800, 600).unwrap();
        for (i, px) in shot.pixels_mut().iter_mut().enumerate() {
            let (x, y) = ((i % 800) as u8, (i / 800) as u8);
            *px = tiny_skia::PremultipliedColorU8::from_rgba(x / 2 + 40, y / 2 + 40, 90, 255).unwrap();
        }
        let shot = Rc::new(shot);
        drawn::none(drawn::in_every_look("the magnifier", 2, || {
            for (state, held, note, at) in [("held, with its hint", Some((400.0, 300.0)), true, None), ("at rest", None, false, Some((380, 280, 40, 40)))] {
                let app = Magnifier { lp: Loop::new(), picture: Some(shot.clone()), held, note, at, finished: Rc::new(|| {}) };
                drawn::window(state, App::view(&app), Theme::chosen(), (800, 600), true);
            }
        }));
    }

    #[test]
    #[ignore]
    fn shoot_ring_magnifier() {
        let mut shot = Pixmap::new(800, 600).unwrap();
        for (i, px) in shot.pixels_mut().iter_mut().enumerate() {
            let (x, y) = ((i % 800) as u8, (i / 800) as u8);
            *px = tiny_skia::PremultipliedColorU8::from_rgba(x / 2 + 40, y / 2 + 40, 90, 255).unwrap();
        }
        let app = Magnifier {
            lp: Loop::new(), picture: Some(Rc::new(shot)), held: Some((400.0, 300.0)), note: true, at: None, finished: Rc::new(|| {}),
        };
        crate::ring::shots::save("magnifier", App::view(&app), (800, 600));
    }
}
