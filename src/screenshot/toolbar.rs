//! The small card of round buttons floating beside the chosen part: the
//! drawing tools, undo and redo on the top row; colours, thicknesses and
//! what to do with the picture below. The tools have no icons in the icon
//! set, so each is drawn here, in the button's own colour, so it lights up
//! with the button like any icon would.

use tiny_skia::{FillRule, LineCap, LineJoin, Paint, PathBuilder, Path, PixmapMut, Rect as Area, Stroke, Transform};

use super::marks::{colour_names, thickness_names, Tool, COLOURS, THICKNESS};
use crate::common::kit::Rgb;
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::text::Style;
use crate::toolkit::widget::{button, card, column, row, space, word_button, Length, Node, Pen, Rect};

const SPACING: f32 = 4.0;

const GROUP: f32 = 12.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Act {
    Tool(Tool),
    Colour(usize),
    Thick(usize),
    Undo,
    Redo,
    Copy,
    Save,
    SaveAs,
    Pin,
    Cancel,
}

pub struct Now {
    pub tool: Option<Tool>,
    pub colour: Rgb,
    pub thick: f64,
    pub undo: bool,
    pub redo: bool,
}

pub fn card_of<M: Clone + 'static>(now: &Now, wrap: fn(Act) -> M) -> Node<M> {
    let gap = || space().width(Length::Fixed(GROUP));
    let mut top: Vec<Node<M>> = Tool::ALL.iter()
        .map(|t| {
            let t = *t;
            button("", t.name()).face(move |c, r, ink, pen| glyph(t, c, r, ink, pen))
                .on(now.tool == Some(t)).on_press(wrap(Act::Tool(t)))
        })
        .collect();
    top.push(space());
    top.push(gap());
    let mut undo = button("edit-undo-symbolic", t!("screenshot-undo"));
    if now.undo {
        undo = undo.on_press(wrap(Act::Undo));
    }
    let mut redo = button("edit-redo-symbolic", t!("screenshot-redo"));
    if now.redo {
        redo = redo.on_press(wrap(Act::Redo));
    }
    top.push(undo);
    top.push(redo);

    let colour_names = colour_names();
    let mut foot: Vec<Node<M>> = COLOURS.iter().enumerate()
        .map(|(i, (c, _))| {
            let c = *c;
            button("", colour_names[i]).face(move |canvas, r, ink, _| swatch(canvas, r, c, ink))
                .on(c == now.colour).on_press(wrap(Act::Colour(i)))
        })
        .collect();
    foot.push(gap());
    let thickness_names = thickness_names();
    foot.extend(THICKNESS.iter().enumerate().map(|(i, (t, _))| {
        let t = *t;
        button("", thickness_names[i]).face(move |canvas, r, ink, _| dot(canvas, r, t, ink))
            .on(t == now.thick).on_press(wrap(Act::Thick(i)))
    }));
    foot.push(gap());
    foot.push(button("edit-copy-symbolic", t!("screenshot-copy")).on_press(wrap(Act::Copy)));

    foot.push(word_button(t!("screenshot-save")).on_press(wrap(Act::Save)));
    foot.push(word_button(t!("screenshot-save-as")).on_press(wrap(Act::SaveAs)));
    foot.push(button("view-pin-symbolic", t!("screenshot-pin")).on_press(wrap(Act::Pin)));
    foot.push(gap());
    foot.push(button("window-close-symbolic", t!("screenshot-cancel")).on_press(wrap(Act::Cancel)));

    let rows = vec![row(top).gap(SPACING).width(Length::Fill), row(foot).gap(SPACING).width(Length::Shrink)];
    card(column(rows).gap(SPACING * 2.0).width(Length::Shrink)).width(Length::Shrink)
}

fn ink_paint(c: Colour, alpha: f32) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color(Colour { a: c.a * alpha, ..c }.to_skia());
    p.anti_alias = true;
    p
}

fn stroke(width: f32, cap: LineCap, join: LineJoin) -> Stroke {
    Stroke { width, line_cap: cap, line_join: join, ..Stroke::default() }
}

fn path(build: impl FnOnce(&mut PathBuilder)) -> Option<Path> {
    let mut b = PathBuilder::new();
    build(&mut b);
    b.finish()
}

struct Brush {
    at: Transform,
    ink: Paint<'static>,
}

impl Brush {
    fn line(&self, canvas: &mut PixmapMut, s: &Stroke, build: &dyn Fn(&mut PathBuilder)) {
        if let Some(p) = path(|b| build(b)) {
            canvas.stroke_path(&p, &self.ink, s, self.at, None);
        }
    }

    fn fill(&self, canvas: &mut PixmapMut, paint: &Paint, build: &dyn Fn(&mut PathBuilder)) {
        if let Some(p) = path(|b| build(b)) {
            canvas.fill_path(&p, paint, FillRule::Winding, self.at, None);
        }
    }
}

fn glyph(tool: Tool, canvas: &mut PixmapMut, r: Rect, c: Colour, pen: &Pen) {
    let u = r.w / 20.0;
    let brush = Brush { at: Transform::from_translate(r.x, r.y).pre_scale(u, u), ink: ink_paint(c, 1.0) };
    let round = stroke(2.0, LineCap::Round, LineJoin::Round);
    match tool {
        Tool::Pen => pen_glyph(&brush, canvas),
        Tool::Line => brush.line(canvas, &round, &|b| {
            b.move_to(4.0, 16.0);
            b.line_to(16.0, 4.0);
        }),
        Tool::Arrow => arrow_glyph(&brush, canvas, &round),
        Tool::Box => brush.line(canvas, &stroke(2.0, LineCap::Butt, LineJoin::Miter), &|b| {
            b.push_rect(Area::from_xywh(3.5, 5.5, 13.0, 9.0).unwrap_or(Area::from_xywh(0.0, 0.0, 1.0, 1.0).unwrap()));
        }),
        Tool::Ring => brush.line(canvas, &round, &|b| {
            if let Some(o) = Area::from_xywh(3.0, 4.2, 14.0, 11.6) {
                b.push_oval(o);
            }
        }),
        Tool::Marker => marker_glyph(&brush, canvas, c),
        Tool::Text => brush.line(canvas, &stroke(2.4, LineCap::Round, LineJoin::Round), &|b| {
            b.move_to(4.5, 4.5);
            b.line_to(15.5, 4.5);
            b.move_to(10.0, 4.5);
            b.line_to(10.0, 16.0);
        }),
        Tool::Number => number_glyph(&brush, canvas, (r, c), pen),
        Tool::Blur => blur_glyph(&brush, canvas, c),
    }
}

fn pen_glyph(brush: &Brush, canvas: &mut PixmapMut) {
    brush.line(canvas, &stroke(3.4, LineCap::Butt, LineJoin::Round), &|b| {
        b.move_to(7.4, 12.6);
        b.line_to(15.2, 4.8);
    });
    brush.fill(canvas, &brush.ink, &|b| {
        b.move_to(3.5, 16.5);
        b.line_to(4.6, 12.3);
        b.line_to(7.7, 15.4);
        b.close();
    });
}

fn arrow_glyph(brush: &Brush, canvas: &mut PixmapMut, round: &Stroke) {
    brush.line(canvas, round, &|b| {
        b.move_to(4.0, 16.0);
        b.line_to(12.5, 7.5);
    });
    brush.fill(canvas, &brush.ink, &|b| {
        b.move_to(16.5, 3.5);
        b.line_to(9.2, 5.6);
        b.line_to(14.4, 10.8);
        b.close();
    });
}

fn marker_glyph(brush: &Brush, canvas: &mut PixmapMut, c: Colour) {
    brush.fill(canvas, &ink_paint(c, 0.45), &|b| {
        if let Some(r) = Area::from_xywh(2.5, 12.5, 15.0, 5.0) {
            b.push_rect(r);
        }
    });
    brush.fill(canvas, &brush.ink, &|b| {
        b.move_to(8.0, 14.0);
        b.line_to(10.5, 9.0);
        b.line_to(16.0, 3.5);
        b.line_to(17.5, 5.0);
        b.line_to(12.0, 10.5);
        b.close();
    });
}

fn number_glyph(brush: &Brush, canvas: &mut PixmapMut, (r, c): (Rect, Colour), pen: &Pen) {
    let u = r.w / 20.0;
    brush.line(canvas, &stroke(1.8, LineCap::Round, LineJoin::Round), &|b| b.push_circle(10.0, 10.0, 7.6));
    let style = Style { family: Some(crate::toolkit::text::FAMILY.into()), weight: 700, ..Style::new(10.5 * u) };
    let text = pen.fonts.layout("1", &style, None);
    pen.fonts.draw(canvas, &text, r.x + 10.0 * u - text.width / 2.0, r.y + 10.0 * u - text.height / 2.0, c);
}

fn blur_glyph(brush: &Brush, canvas: &mut PixmapMut, c: Colour) {
    const SHADE: [f32; 9] = [1.0, 0.35, 0.7, 0.35, 0.7, 0.35, 0.7, 0.35, 1.0];
    for (i, a) in SHADE.iter().enumerate() {
        let (col, row) = ((i % 3) as f32, (i / 3) as f32);
        brush.fill(canvas, &ink_paint(c, *a), &|b| {
            if let Some(r) = Area::from_xywh(3.0 + col * 4.8, 3.0 + row * 4.8, 4.4, 4.4) {
                b.push_rect(r);
            }
        });
    }
}

fn swatch(canvas: &mut PixmapMut, r: Rect, c: Rgb, _fg: Colour) {
    let Some(disc) = path(|b| b.push_circle(r.x + r.w / 2.0, r.y + r.h / 2.0, r.w / 2.0 - r.w * 0.2)) else { return };
    let colour = Colour::rgb(c.0 as f32, c.1 as f32, c.2 as f32);
    canvas.fill_path(&disc, &ink_paint(colour, 1.0), FillRule::Winding, Transform::identity(), None);
}

fn dot(canvas: &mut PixmapMut, r: Rect, thick: f64, fg: Colour) {
    let u = r.w / 18.0;
    let radius = (1.2 + thick as f32 * 0.55) * u;
    if let Some(d) = path(|b| b.push_circle(r.x + r.w / 2.0, r.y + r.h / 2.0, radius.min(r.w / 2.0))) {
        canvas.fill_path(&d, &ink_paint(fg, 1.0), FillRule::Winding, Transform::identity(), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;

    #[test]
    fn save_and_save_as_are_words_and_the_rows_end_together() {
        let now = Now { tool: Some(Tool::Pen), colour: COLOURS[1].0, thick: THICKNESS[1].0, undo: true, redo: true };
        let card: Node<Act> = card_of(&now, |a| a);
        let worded: Vec<(String, Option<String>)> = card.worded().into_iter().filter(|(tip, _)| tip.starts_with("Save")).collect();
        assert_eq!(worded, [("Save".to_string(), Some("Save".to_string())), ("Save as".to_string(), Some("Save as".to_string()))]);
        assert!(card.iconed().iter().all(|(tip, _)| !tip.starts_with("Save")), "a Save button has an icon");
        let node = crate::toolkit::widget::layers(vec![crate::toolkit::widget::space(), crate::toolkit::widget::placed(card.floats(), |_, _| (24.0, 24.0))]);
        let mut ui = Ui::new(node, Theme::new(false, crate::common::style::DEFAULT_ACCENT));
        let places = ui.places_at((900, 200));
        let wrong = ui.squeezed_or_touching();
        assert!(wrong.is_empty(), "{wrong:?}");
        let ends: Vec<f32> = ui.round_and_worded().into_iter().filter(|(tip, _)| tip == "Redo" || tip == "Cancel").map(|(_, r)| r.x + r.w).collect();
        assert_eq!(ends.len(), 2, "{places:?}");
        assert!((ends[0] - ends[1]).abs() <= 1.0, "Redo ends at {} and Cancel, under it, at {}", ends[0], ends[1]);
    }

    #[test]
    fn the_screenshot_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "screenshot-tool-pen = Crayon\nscreenshot-undo = Annuler\nscreenshot-eye-taken =\n    { $n ->\n        [one] Une couleur prise : { $newest }.\n       *[other] { $n } couleurs prises ; la dernière est { $newest }.\n    }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(Tool::Pen.name(), "Crayon");
            assert_eq!(Tool::Line.name(), "Line", "a word French lacks is English");
            let now = Now { tool: None, colour: COLOURS[0].0, thick: THICKNESS[0].0, undo: true, redo: true };
            let card: Node<Act> = card_of(&now, |a| a);
            assert!(card.iconed().iter().any(|(tip, _)| tip == "Annuler"));
            assert_eq!(t!("screenshot-eye-taken", n = 1, newest = "#FF0000"), "Une couleur prise : #FF0000.");
            assert_eq!(t!("screenshot-eye-taken", n = 3, newest = "#FF0000"), "3 couleurs prises ; la dernière est #FF0000.");
        });
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_toolbar() {
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let now = Now { tool: Some(Tool::Pen), colour: COLOURS[1].0, thick: THICKNESS[1].0, undo: true, redo: false };
        let card: Node<Act> = card_of(&now, |a| a);
        let node = crate::toolkit::widget::layers(vec![
            crate::toolkit::widget::space(),
            crate::toolkit::widget::placed(card.floats(), |_, _| (24.0, 24.0)),
        ]);
        let mut ui = Ui::new(node, Theme::new(false, crate::common::style::DEFAULT_ACCENT));
        let mut pix = tiny_skia::Pixmap::new(760, 160).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(std::path::Path::new(&dir).join("toolbar.png")).unwrap();
    }
}
