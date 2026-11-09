//! The label on pointing for a window with no room for one.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::toolkit::theme;
use crate::toolkit::ui::{label_layout, label_pill, App, Outside, Shown};
use crate::toolkit::widget::{drawing, Length, Node, Rect};
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};

use super::ctx::Ctx;

pub const NAME: &str = "trindle-label";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Beside {

    pub right: bool,

    pub across: i32,

    pub middle: i32,
}

fn room() -> i32 {
    let fall = theme::sheet().label_shadow;
    (fall.y + fall.blur).ceil() as i32 + 2
}

pub fn placed(size: (f32, f32), at: Beside, tall: i32) -> ((u32, u32), [i32; 4]) {
    let k = room();
    let gap = theme::sheet().label_gap.round() as i32;
    let (w, h) = (size.0.ceil() as i32 + 2 * k, size.1.ceil() as i32 + 2 * k);
    let top = (at.middle - h / 2).clamp(0, (tall - h).max(0));
    let side = (at.across + gap - k).max(0);
    ((w.max(1) as u32, h.max(1) as u32), if at.right { [top, side, 0, 0] } else { [top, 0, 0, side] })
}

#[derive(Clone)]
enum Msg {}

struct Words {
    words: String,
}

impl App for Words {
    type Msg = Msg;

    fn update(&mut self, _: Msg) {}

    fn view(&self) -> Node<Msg> {
        let words = self.words.clone();
        drawing((1.0, 1.0), "", move |canvas, pen| {
            let layout = label_layout(pen.fonts, &words, pen.scale, f32::MAX);
            let gap = (theme::sheet().label_gap * pen.scale).round();
            let edge = (room() as f32 * pen.scale).round();

            let under = Rect { x: 0.0, y: edge - gap, w: canvas.width() as f32, h: 0.0 };
            label_pill(canvas, pen.fonts, pen.scale, &layout, under, canvas.height() as f32, theme::sheet().label_ink);
        })
        .width(Length::Fill)
        .height(Length::Fill)
    }
}

pub struct LabelWindow {
    ctx: Rc<Ctx>,
    win: RefCell<Option<Shown<Words>>>,

    right: Cell<bool>,
}

impl LabelWindow {
    pub fn new(ctx: &Rc<Ctx>) -> LabelWindow {
        LabelWindow { ctx: ctx.clone(), win: RefCell::new(None), right: Cell::new(false) }
    }

    pub fn show(&self, out: Option<Outside>, at: impl FnOnce(Rect) -> Option<Beside>) {
        let Some(out) = out else { return self.hide() };
        let Some(at) = at(out.at) else { return self.hide() };
        let tall = self.ctx.display.places().first().map_or(800, |p| p[3]);
        let (size, margins) = placed(out.size, at, tall);
        let mut win = self.win.borrow_mut();
        if self.right.get() != at.right {
            *win = None;
        }
        match &*win {
            Some(w) => {
                w.change(|l| l.words = out.words.clone());
                w.window.resize(size);
                w.window.set_margins(margins);
                w.window.set_visible(true);
            }
            None => {
                let edges = Edges::TOP | if at.right { Edges::RIGHT } else { Edges::LEFT };
                let kind = Kind::Panel(Panel {
                    layer: Layer::Overlay, edges, keep: -1, margins, keys: Keys::Never, name: NAME.into(), screen: None,
                });
                let Some(shown) = Shown::open(&self.ctx.lp, &self.ctx.display, kind, size, Words { words: out.words }) else { return };
                shown.see_through();

                shown.window.set_input_region(Some(&[]));
                self.right.set(at.right);
                *win = Some(shown);
            }
        }
    }

    pub fn hide(&self) {
        if let Some(w) = &*self.win.borrow() {
            w.window.set_visible(false);
        }
    }

    pub fn restyle(&self) {
        if let Some(w) = &*self.win.borrow() {
            w.restyle();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_label_beside_a_tab_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("the label beside a tab", 1, || {
            for words in ["Letters", "Backup finished\nThe backup of Documents to the drive in the hall cupboard finished at 04:12 with no errors and 3 warnings"] {
                let size = (260, 90);
                drawn::window(words, Words { words: words.into() }.view(), Theme::chosen(), size, true);
            }
        }));
    }

    #[test]
    fn a_label_sits_beside_the_tab_on_the_side_away_from_the_edge_and_stays_on_the_screen() {
        let at = Beside { right: false, across: 170, middle: 300 };
        let ((w, h), [top, right, bottom, left]) = placed((120.0, 30.0), at, 800);
        assert_eq!((right, bottom), (0, 0));

        assert_eq!(left, 170 + theme::sheet().label_gap.round() as i32 - room());
        assert!(w as i32 >= 120 + 2 * room() && h as i32 >= 30 + 2 * room());

        assert_eq!(top + h as i32 / 2, 300);

        let (_, m) = placed((120.0, 30.0), Beside { right: true, ..at }, 800);
        assert_eq!((m[1], m[3]), (left, 0));

        let ((_, h), m) = placed((120.0, 30.0), Beside { middle: 2, ..at }, 800);
        assert_eq!(m[0], 0);
        let (_, m) = placed((120.0, 30.0), Beside { middle: 799, ..at }, 800);
        assert_eq!(m[0] + h as i32, 800);
    }

    #[test]
    #[ignore]
    fn shoot_the_label_window() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-label".into()));
        std::fs::create_dir_all(&out).unwrap();
        let cases = [
            ("notification", "Backup finished\nThe backup of Documents to the drive in the hall cupboard finished at 04:12 with no errors and 3 warnings about files that were open"),
            ("window-name", "Quarterly accounts 2026 final (copy 3) - Spreadsheet"),
        ];
        for (name, words) in cases {
            for scale in [1.0f32, 2.0] {
                let mut ui = Ui::new(Words { words: words.to_string() }.view(), Theme::new(false, "#fbc711"));
                ui.see_through = true;
                let fonts = crate::toolkit::text::Fonts::new();
                let layout = label_layout(&fonts, words, scale, f32::MAX);
                let wide = layout.lines.iter().map(|l| l.width).fold(0.0, f32::max);
                let (w, h) = ((wide.ceil() / scale + 2.0 * theme::sheet().label_padding.0.round()).ceil(), (layout.height.ceil() / scale + 2.0 * theme::sheet().label_padding.1.round()).ceil());
                let ((cw, ch), _) = placed((w, h), Beside { right: false, across: 170, middle: 100 }, 800);
                let mut label = tiny_skia::Pixmap::new((cw as f32 * scale) as u32, (ch as f32 * scale) as u32).unwrap();
                ui.paint(&mut label.as_mut(), scale);

                let k = scale;
                let (sw, sh) = (((cw as f32 + 200.0) * k) as u32, ((ch as f32 + 100.0) * k) as u32);
                let mut screen = tiny_skia::Pixmap::new(sw, sh).unwrap();
                screen.fill(tiny_skia::Color::from_rgba8(0x3a, 0x3d, 0x44, 0xff));
                let mut tab = tiny_skia::Paint::default();
                tab.set_color_rgba8(0x24, 0x26, 0x2b, 0xff);
                let rect = tiny_skia::Rect::from_xywh(0.0, 20.0 * k, 170.0 * k, 60.0 * k).unwrap();
                screen.fill_rect(rect, &tab, tiny_skia::Transform::identity(), None);
                let top = (20.0 * k + 30.0 * k - label.height() as f32 / 2.0).max(0.0);
                screen.draw_pixmap((170.0 * k) as i32, top as i32, label.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
                screen.save_png(out.join(format!("label-beside-{name}-{}x.png", scale as u32))).unwrap();
            }
        }
    }
}
