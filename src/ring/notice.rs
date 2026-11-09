//! The notice: a small solid pill in the box colour, one line of words, at
//! the bottom centre of the screen for three seconds, for what could go
//! unseen (a screenshot saved by a program with no window left to say so).
//! It floats, so it has the one soft shadow; it takes no clicks, and nothing
//! on the screen moves to make room for it.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

pub use crate::toolkit::pill::{float_shadow, paint_pill, BAND, FOOT, HEIGHT, ROOM};
use crate::toolkit::pill::WIDEST;
use crate::toolkit::theme;
use crate::toolkit::ui::{label_pill, tip_style, App, Shown};
use crate::toolkit::widget::{drawing, placed, Length, Node, Pen, Rect};
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};

use super::ctx::Ctx;

pub const SECONDS: u64 = 3;

pub const WORDS_BAND: f32 = 40.0;
const WORDS_LINE: f32 = 20.0;

const WORDS_TOP: f32 = 6.0;
const WORDS_SIDE: f32 = 16.0;

const WORDS_SHADOW: f32 = 12.0;

const PROBLEM_LINES: usize = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub words: String,
    pub problem: bool,
}

impl Line {
    pub fn new(words: impl Into<String>) -> Line {
        Line { words: words.into(), problem: false }
    }

    pub fn problem(words: impl Into<String>) -> Line {
        Line { words: words.into(), problem: true }
    }

    fn tall(&self) -> f32 {
        WORDS_BAND + if self.problem { (PROBLEM_LINES - 1) as f32 * WORDS_LINE } else { 0.0 }
    }
}

pub fn paint_words(canvas: &mut tiny_skia::PixmapMut, pen: &Pen, line: &Line) -> Rect {
    let (s, k) = (pen.scale, theme::sheet());
    let style = tip_style(s);
    let pad = (k.label_padding.0 * s).round();
    let across = canvas.width() as f32;
    let room = ((crate::common::style::ring_words_width() * s).min(across - 2.0 * WORDS_SIDE * s) - 2.0 * pad).max(1.0);
    let dots = pen.fonts.layout("\u{2026}", &style, None);
    let layout = if line.problem {
        pen.fonts.layout(&line.words, &style, Some(room)).cut_after(PROBLEM_LINES, room, &dots)
    } else {
        pen.fonts.layout(&line.words, &style, None).cut(room, &dots)
    };

    let red = [pen.theme.danger_text, pen.theme.danger].into_iter().find(|c| theme::contrast(*c, k.label_back) >= theme::AIM_WORDS);
    let ink = if line.problem { red.unwrap_or(k.label_ink) } else { k.label_ink };
    let gap = (k.label_gap * s).round();
    let above = Rect { x: 0.0, y: (WORDS_TOP * s).round() - gap, w: across, h: 0.0 };
    label_pill(canvas, pen.fonts, s, &layout, above, canvas.height() as f32, ink)
}

pub fn words_tops(lines: &[Line], tall: f32, clear: f32) -> Vec<f32> {
    let middle = tall / 2.0;
    let (mut below, mut above, mut over) = (middle + clear, middle - clear, middle + clear);
    lines.iter().map(|line| {
        let h = line.tall();
        if below + h <= tall {
            below += h;
            below - h
        } else if above - h >= 0.0 {
            above -= h;
            above
        } else {
            over = (over - h).max(0.0);
            over
        }
    }).collect()
}

pub fn words_outside<M: 'static>(lines: &[Line], (w, h): (f32, f32), clear: f32) -> Vec<Node<M>> {
    let wide = w.min(crate::common::style::ring_words_width() + 2.0 * (WORDS_SIDE + theme::sheet().label_padding.0));
    lines.iter().zip(words_tops(lines, h, clear)).map(|(line, top)| {
        let (tall, said) = (line.tall(), line.clone());
        let strip = drawing((wide, tall + WORDS_SHADOW), line.words.clone(), move |canvas, pen| {
            paint_words(canvas, pen, &said);
        })
        .width(Length::Fixed(wide)).height(Length::Fixed(tall + WORDS_SHADOW));
        placed(strip, move |_, (aw, _)| ((aw - wide) / 2.0, top))
    }).collect()
}
#[derive(Clone)]
enum Msg {}

struct Pill {
    words: String,
}

impl App for Pill {
    type Msg = Msg;

    fn update(&mut self, _: Msg) {}

    fn view(&self) -> Node<Msg> {
        let words = self.words.clone();
        drawing((WIDEST + 2.0 * ROOM, HEIGHT + 2.0 * ROOM), words.clone(), move |canvas, pen| {
            paint_words(canvas, pen, &Line::new(words.as_str()));
        })
            .width(Length::Fill)
            .height(Length::Fill)
            .spoken(self.words.clone())
    }
}

pub struct Notice {
    ctx: Rc<Ctx>,
    me: Weak<Notice>,
    win: RefCell<Option<Shown<Pill>>>,

    turn: Cell<u32>,
}

pub fn start(ctx: &Rc<Ctx>) -> Rc<Notice> {
    Rc::new_cyclic(|me| Notice { ctx: ctx.clone(), me: me.clone(), win: RefCell::new(None), turn: Cell::new(0) })
}

impl Notice {

    pub fn show(&self, words: &str) {
        let this = self.turn.get() + 1;
        self.turn.set(this);
        let size = ((WIDEST + 2.0 * ROOM).ceil() as u32, (HEIGHT + 2.0 * ROOM).ceil() as u32);
        let mut win = self.win.borrow_mut();
        match &*win {
            Some(w) => {
                w.change(|p| p.words = words.to_string());
                w.window.set_visible(true);
            }
            None => {
                let kind = Kind::Panel(Panel {
                    layer: Layer::Overlay, edges: Edges::BOTTOM, keep: -1, margins: [0, 0, (FOOT - ROOM) as i32, 0],
                    keys: Keys::Never, name: "trindle-notice".into(), screen: None,
                });
                let Some(shown) = Shown::open(&self.ctx.lp, &self.ctx.display, kind, size, Pill { words: words.to_string() }) else { return };
                shown.see_through();

                shown.window.set_input_region(Some(&[]));
                *win = Some(shown);
            }
        }
        let me = self.me.clone();
        self.ctx.lp.after(Duration::from_secs(SECONDS), move || {
            let Some(n) = me.upgrade() else { return };
            if n.turn.get() == this {
                if let Some(w) = &*n.win.borrow() {
                    w.window.set_visible(false);
                }
            }
        });
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
    use crate::toolkit::icon::Icons;
    use crate::toolkit::widget::Pen;
    use crate::toolkit::text::Fonts;
    use crate::toolkit::theme::Theme;

    #[test]
    fn the_notice_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        use crate::toolkit::ui::App;
        let size = ((WIDEST + 2.0 * ROOM).ceil() as u32, (HEIGHT + 2.0 * ROOM).ceil() as u32);
        drawn::none(drawn::in_every_look("the notice", 1, || {
            for words in ["Copied", "Copied a picture with a long name of its own, John Smith's notes [example.com], and a great deal more besides, on and on."] {
                drawn::window(words, Pill { words: words.into() }.view(), Theme::chosen(), size, true);
            }
        }));
    }

    #[test]
    fn words_outside_the_disc_are_in_a_pill_that_stays_on_the_screen() {
        let (t, fonts, icons) = (Theme::new(false, ""), Fonts::new(), Icons::new());
        let k = theme::sheet();
        let long = "1 item, 345 KB. Copied a picture with a long name of its own - John Smith's notes [example.com] and a great deal more besides, \
                    on and on past the edge of any screen, and then further still.";
        for line in [Line::new("No app or favourite is called that"), Line::new(long), Line::problem(long), Line::problem("The ring stopped.")] {
            for (across, scale) in [(360u32, 1.0f32), (560, 1.0), (1280, 1.0), (1280, 2.0), (2560, 1.5)] {
                let pen = Pen { theme: &t, fonts: &fonts, icons: &icons, scale };
                let mut pix = tiny_skia::Pixmap::new(across, (line.tall() * scale) as u32).unwrap();
                let pill = paint_words(&mut pix.as_mut(), &pen, &line);
                let what = format!("{:?} on {across} at {scale}", &line.words[..12]);
                assert!(pill.x >= WORDS_SIDE * scale - 1.0 && pill.x + pill.w <= across as f32 - WORDS_SIDE * scale + 1.0, "{what}: the pill is off the strip: {pill:?}");
                assert!(pill.y >= 0.0 && pill.y + pill.h <= pix.height() as f32, "{what}: the pill is taller than its strip: {pill:?}");
                let most = (crate::common::style::ring_words_width() + 2.0 * k.label_padding.0) * scale;
                assert!(pill.w <= most + 2.0, "{what}: wider than the most a line may be: {} against {most}", pill.w);

                let (want, got) = (k.label_back.to_skia().to_color_u8(), pix.pixel((pill.x + 3.0 * scale) as u32, (pill.y + pill.h / 2.0) as u32).unwrap());
                assert!((got.red() as i32 - want.red() as i32).abs() <= 3 && got.alpha() == 255, "{what}: no pill behind the words: {got:?} against {want:?}");
                let light = |p: &tiny_skia::PremultipliedColorU8| p.alpha() > 200 && p.red() as i32 + p.green() as i32 + p.blue() as i32 > 520;
                let stray = pix.pixels().iter().enumerate().filter(|(i, p)| {
                    let (x, y) = ((i % across as usize) as f32, (i / across as usize) as f32);
                    light(p) && !(x >= pill.x && x < pill.x + pill.w && y >= pill.y && y < pill.y + pill.h)
                }).count();
                assert_eq!(stray, 0, "{what}: words outside their pill");
                assert!(pix.pixels().iter().any(light) || line.problem, "{what}: the words are drawn");
            }
        }
    }

    #[test]
    fn lines_outside_the_disc_never_overlap_each_other_or_the_ring() {
        let three = [Line::new("No app or favourite is called that"), Line::new("Done"), Line::problem("The ring stopped.")];
        for tall in [600.0f32, 768.0, 800.0, 1080.0, 1440.0] {
            for clear in [178.0f32, 226.0, tall / 2.0 - 40.0 - WORDS_BAND, tall / 2.0 - 40.0] {
                for count in 1..=3 {
                    let lines = &three[..count];
                    let tops = words_tops(lines, tall, clear);
                    let spans: Vec<(f32, f32)> = tops.iter().zip(lines).map(|(t, l)| (*t, *t + l.tall())).collect();

                    let short = count <= 2 && lines.iter().all(|l| !l.problem);
                    let fits = short || tall / 2.0 + clear + lines.iter().map(Line::tall).sum::<f32>() <= tall;
                    for (i, (top, foot)) in spans.iter().enumerate() {
                        let what = format!("line {i} of {count} on a {tall} screen round a ring of {clear}: {spans:?}");
                        assert!(*top >= 0.0 && *foot <= tall, "off the screen: {what}");
                        assert!(spans.iter().enumerate().all(|(j, (a, b))| j == i || *b <= *top + 0.5 || *a >= *foot - 0.5), "on another line: {what}");
                        if fits {
                            assert!(*top >= tall / 2.0 + clear - 0.5 || *foot <= tall / 2.0 - clear + 0.5, "on the ring: {what}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_ring_shows_words_outside_its_disc_only_through_the_one_rule() {
        let view = crate::ring::view::tests::view_code();

        let body = |from: &str| {
            let at = view.find(from).unwrap_or_else(|| panic!("{from} is not in the ring's view"));
            let rest = &view[at..];
            &rest[..rest.find("\n    }\n").expect("the end of it") + 1]
        };

        let parts = ["    fn view(&self) -> Node<Msg> {", "    fn app_layers(&self", "    fn quick_layers(&self", "    fn menu_layers(&self", "    fn words_layers(&self"].map(body);
        for bare in ["placed(note(", "placed(label(", "paint_pill", "Tone::Problem"] {
            assert!(!parts.iter().any(|part| part.contains(bare)), "the ring's view places words outside the one rule: {bare}");
        }
        assert!(parts[4].contains("words_outside("));
    }

    #[test]
    fn the_pill_is_the_box_colour_with_its_words_in_it() {
        let (t, fonts, icons) = (Theme::new(false, ""), Fonts::new(), Icons::new());
        let pen = Pen { theme: &t, fonts: &fonts, icons: &icons, scale: 1.0 };
        let mut pix = tiny_skia::Pixmap::new(300, 120).unwrap();
        paint_pill(&mut pix.as_mut(), &pen, "Done");

        let (want, got) = (t.raised.to_skia().to_color_u8(), pix.pixel(124, 60).unwrap());
        assert!((got.red() as i32 - want.red() as i32).abs() <= 2 && got.alpha() == 255, "{got:?} against {want:?}");

        assert!(pix.pixels().iter().any(|p| p.red() > want.red() + 60), "the words are drawn");
    }
}
