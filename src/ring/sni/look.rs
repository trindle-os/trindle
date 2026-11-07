//! How a tray icon is drawn.

use std::cell::RefCell;
use std::rc::Rc;

use tiny_skia::{FilterQuality, Pixmap, PixmapMut, PixmapPaint, Transform};

use crate::toolkit::colour::Colour;
use crate::toolkit::icon::{self, Icons};
use crate::toolkit::widget::{Pen, Rect};

pub const SIZE: u32 = 18;

pub(super) const GLYPH: f32 = 16.0;

pub(in crate::ring) enum Look {
    Named(String),
    Shape { icons: Rc<Icons>, name: String, cache: RefCell<Option<(u32, Colour, Rc<Pixmap>)>> },
    Pixels { px: Pixmap, cache: RefCell<Option<(Colour, Rc<Pixmap>)>> },
}

impl Look {
    pub(in crate::ring) fn shape(icons: Rc<Icons>, name: String) -> Look {
        Look::Shape { icons, name, cache: RefCell::new(None) }
    }

    pub(in crate::ring) fn paint(&self, canvas: &mut PixmapMut, r: Rect, ink: Colour, pen: &Pen) {
        let size = r.w.max(1.0) as u32;
        match self {
            Look::Named(name) => {
                pen.icons.draw(canvas, name, r.x as i32, r.y as i32, size, 1, ink);
            }
            Look::Shape { icons, name, cache } => {
                let hit = cache.borrow().as_ref().filter(|(s, c, _)| *s == size && *c == ink).map(|(_, _, p)| p.clone());
                let px = hit.or_else(|| {
                    let src = icons.get(name, size, 1, ink)?;
                    let p = Rc::new(icon::silhouette(&src, ink, true));
                    *cache.borrow_mut() = Some((size, ink, p.clone()));
                    Some(p)
                });
                if let Some(px) = px {
                    fit(canvas, &px, r);
                }
            }
            Look::Pixels { px, cache } => {
                let hit = cache.borrow().as_ref().filter(|(c, _)| *c == ink).map(|(_, p)| p.clone());
                let flat = hit.unwrap_or_else(|| {
                    let p = Rc::new(icon::silhouette(px, ink, true));
                    *cache.borrow_mut() = Some((ink, p.clone()));
                    p
                });
                fit(canvas, &flat, r);
            }
        }
    }
}

fn fit(canvas: &mut PixmapMut, px: &Pixmap, r: Rect) {
    let (w, h) = (px.width() as f32, px.height() as f32);
    let k = (r.w / w).min(r.h / h);
    let (dx, dy) = (r.x + (r.w - w * k) / 2.0, r.y + (r.h - h * k) / 2.0);
    let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    canvas.draw_pixmap(0, 0, px.as_ref(), &paint, Transform::from_row(k, 0.0, 0.0, k, dx, dy), None);
}
