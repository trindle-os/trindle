//! The person's picture above the boxes.

use std::rc::Rc;

use image::DynamicImage;
use tiny_skia::{FilterQuality, Paint, Pattern, Pixmap, SpreadMode, Transform};

use crate::toolkit::colour::Colour;
use crate::toolkit::text::Style;
use crate::toolkit::theme::Palette;
use crate::toolkit::widget::{drawing, Length, Node, Rect};

use super::{Greeter, Msg, PANEL};

const FACE: f32 = 72.0;

const FACE_SHADOW: f32 = 6.0;

const INITIALS: [Palette; 14] = [Palette::Pink, Palette::Purple, Palette::DeepPurple, Palette::Indigo, Palette::Blue,
                                 Palette::LightBlue, Palette::Cyan, Palette::Teal, Palette::Yellow, Palette::Amber,
                                 Palette::Orange, Palette::DeepOrange, Palette::Brown, Palette::BlueGrey];

pub(super) fn initials_colour(name: &str) -> Colour {
    let n = name.bytes().fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(b.into()));
    INITIALS[n as usize % INITIALS.len()].swatch()
}

fn fill_circle(canvas: &mut tiny_skia::PixmapMut, at: Rect, colour: Colour, scale: f32) {
    let Some(circle) = crate::toolkit::ui::rounded(at, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, at.w, scale)) else { return };
    let mut paint = Paint::default();
    paint.set_color(colour.to_skia());
    paint.anti_alias = true;
    canvas.fill_path(&circle, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
}

fn paint_face(canvas: &mut tiny_skia::PixmapMut, at: Rect, face: &DynamicImage, scale: f32) {
    let size = at.w.round() as u32;
    let Some(mut picture) = Pixmap::new(size, size) else { return };
    if crate::toolkit::picture::draw(&mut picture.as_mut(), face, 0, 0, size, size).is_none() {
        return;
    }
    let Some(circle) = crate::toolkit::ui::rounded(at, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, at.w, scale)) else { return };
    let paint = Paint {
        shader: Pattern::new(picture.as_ref(), SpreadMode::Pad, FilterQuality::Bilinear, 1.0, Transform::from_translate(at.x, at.y)),
        anti_alias: true,
        ..Paint::default()
    };
    canvas.fill_path(&circle, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
}

impl Greeter {
    pub(super) fn picture(&self) -> Node<Msg> {
        enum Shows {
            Photo(Rc<DynamicImage>),
            Letter(String, Colour),
            Mark(String),
        }
        let name = self.user.trim();
        let shows = match (&self.face, name.chars().next()) {
            (Some(face), _) => Shows::Photo(face.clone()),
            (None, Some(first)) => Shows::Letter(first.to_uppercase().collect(), initials_colour(name)),
            (None, None) => Shows::Mark(crate::common::files::share("icons").join("trindle.svg").to_string_lossy().into_owned()),
        };
        let icons = self.icons.clone();
        drawing((PANEL, FACE + FACE_SHADOW), "", move |canvas, pen| {
            let size = (FACE * pen.scale).round();
            let at = Rect { x: ((canvas.width() as f32 - size) / 2.0).round(), y: (2.0 * pen.scale).round(), w: size, h: size };
            crate::toolkit::shadow::edge_round(canvas, pen.theme, at, pen.scale);
            match &shows {
                Shows::Photo(face) => paint_face(canvas, at, face, pen.scale),
                Shows::Letter(letter, colour) => {
                    fill_circle(canvas, at, *colour, pen.scale);
                    let style = Style { family: Some(crate::toolkit::text::FAMILY.into()), weight: 600, ..Style::new(FACE * 0.45 * pen.scale) };
                    let layout = pen.fonts.layout(letter, &style, None);
                    let x = (at.x + (size - layout.width) / 2.0).round();
                    let y = (at.y + (size - layout.height) / 2.0).round();
                    pen.fonts.draw(canvas, &layout, x, y, crate::toolkit::theme::on(*colour));
                }
                Shows::Mark(file) => {
                    fill_circle(canvas, at, pen.theme.raised, pen.scale);
                    let mark = (FACE * 0.6).round();
                    let px = (mark * pen.scale).round();
                    let (x, y) = ((at.x + (size - px) / 2.0) as i32, (at.y + (size - px) / 2.0) as i32);
                    let (side, whole) = crate::settings::ui::icon_px(mark, pen.scale);
                    icons.draw(canvas, file, x, y, side, whole, pen.theme.fg);
                }
            }
        }).width(Length::Fill)
    }
}
