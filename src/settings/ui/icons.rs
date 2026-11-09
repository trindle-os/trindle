//! Icons and small pictures drawn for Settings pages: theme icons, app icons, logos and the in-use dots.

use crate::toolkit::colour::Colour;
use crate::toolkit::widget::{drawing, label,
                             row, Length, Node};

pub fn icon<M>(name: impl Into<String>, size: f32) -> Node<M> {
    let name = name.into();
    drawing((size, size), "", move |canvas, pen| {
        let (side, whole) = icon_px(size, pen.scale);
        pen.icons.draw(canvas, &name, 0, 0, side, whole, pen.theme.fg);
    })
}

pub fn icon_px(size: f32, scale: f32) -> (u32, u32) {
    let whole = (scale.ceil() as u32).max(1);
    ((((size * scale).round() as u32) / whole).max(1), whole)
}

pub fn icon_in<M>(name: impl Into<String>, size: f32, colour: impl Fn(&crate::toolkit::theme::Theme) -> Colour + 'static) -> Node<M> {
    let name = name.into();
    drawing((size, size), "", move |canvas, pen| {
        let (side, whole) = icon_px(size, pen.scale);
        pen.icons.draw(canvas, &name, 0, 0, side, whole, colour(pen.theme));
    })
}

pub fn app_icon<M>(names: Vec<String>, size: f32) -> Node<M> {
    drawing((size, size), "", move |canvas, pen| {
        let (side, s) = icon_px(size, pen.scale);
        let found = names.iter().any(|n| pen.icons.draw(canvas, n, 0, 0, side, s, pen.theme.fg).is_some());
        if !found {
            pen.icons.draw(canvas, crate::common::apps::GENERIC_ICONS[0], 0, 0, side, s, pen.theme.fg);
        }
    })
}

pub fn logo<M>(path: std::path::PathBuf, size: f32) -> Node<M> {
    if path.is_file() {
        icon(path.to_string_lossy().into_owned(), size)
    } else {
        icon(crate::common::apps::GENERIC_ICONS[0], size)
    }
}

pub const KEY_DOT: f32 = 16.0;

pub fn in_use_dot<M>(kind: crate::common::in_use::Kind) -> Node<M> {
    in_use_dot_sized(kind, KEY_DOT)
}

pub fn in_use_dot_sized<M>(kind: crate::common::in_use::Kind, size: f32) -> Node<M> {
    drawing((size, size), "", move |canvas, pen| {
        let r = canvas.width().min(canvas.height()) as f32 / 2.0;
        if let Some(path) = tiny_skia::PathBuilder::from_circle(r, r, r) {
            let (red, green, blue) = kind.colour_in(pen.theme.light);
            let mut paint = tiny_skia::Paint::default();
            paint.set_color(tiny_skia::Color::from_rgba(red as f32, green as f32, blue as f32, 1.0).unwrap_or(tiny_skia::Color::BLACK));
            paint.anti_alias = true;
            canvas.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
        }
    })
}

pub fn in_use_key<M: 'static>() -> Vec<Node<M>> {
    crate::common::in_use::Kind::ALL.into_iter().map(|kind| {
        row(vec![in_use_dot(kind), label(kind.spoken(&[])).width(Length::Fill)]).gap(10.0)
    }).collect()
}
