//! A thin line round every window that draws its own title bar (GTK, libadwaita, Firefox and the like), so a dark window stands apart from a dark window or wallpaper behind it.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::solid::{SolidColorBuffer, SolidColorRenderElement};
use smithay::backend::renderer::element::{Id, Kind};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::backend::renderer::{ErasedContextId, ImportMem, Renderer, Texture};
use smithay::utils::{Logical, Physical, Point, Rectangle, Scale, Size};

use super::picture::{whole_scale, Picture, Placer};
use super::{ssd, WindowElement};

struct Outline {
    sides: [SolidColorBuffer; 4],
    corners: [Id; 4],
    commit: CommitCounter,
    drawn: Option<(u32, i32, [u32; 4])>,
}

fn wanted(window: &WindowElement) -> bool {
    if window.0.x11_surface().is_some_and(|x| x.is_override_redirect()) {
        return false;
    }
    !window.decoration_state().is_ssd && !window.is_fullscreen() && !window.is_maximized()
}

impl Default for Outline {
    fn default() -> Self {
        Outline {
            sides: Default::default(),
            corners: std::array::from_fn(|_| Id::new()),
            commit: CommitCounter::default(),
            drawn: None,
        }
    }
}

thread_local! {
    static RINGS: RefCell<HashMap<(ErasedContextId, i32, u32, [u32; 4]), Box<dyn Any>>> = RefCell::new(HashMap::new());
}

fn paint_ring(radius: i32, s: u32, colour: [f32; 4]) -> Vec<u8> {
    let outer = (radius + 1) as f32;
    let side = (2 * (radius + 1)) as usize * s as usize;
    let sf = s as f32;
    let mut bytes = Vec::with_capacity(side * side * 4);
    for y in 0..side {
        for x in 0..side {
            let (lx, ly) = ((x as f32 + 0.5) / sf, (y as f32 + 0.5) / sf);
            let d = ((lx - outer).powi(2) + (ly - outer).powi(2)).sqrt();
            let within = |r: f32| (0.5 - (d - r) * sf).clamp(0.0, 1.0);
            let a = colour[3] * (within(outer) - within(outer - 1.0));
            let [r, g, b] = [colour[0] * a, colour[1] * a, colour[2] * a];
            let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            bytes.extend_from_slice(&[byte(b), byte(g), byte(r), byte(a)]);
        }
    }
    bytes
}

fn ring<R>(renderer: &mut R, radius: i32, s: u32, colour: [f32; 4]) -> Option<R::TextureId>
where
    R: Renderer + ImportMem,
    R::TextureId: Clone + 'static,
{
    let key = (renderer.context_id().erased(), radius, s, colour.map(f32::to_bits));
    let found = RINGS.with(|r| r.borrow().get(&key).and_then(|t| t.downcast_ref::<R::TextureId>()).cloned());
    if found.is_some() {
        return found;
    }
    let side = 2 * (radius + 1) * s as i32;
    let pixels = paint_ring(radius, s, colour);
    let texture = renderer.import_memory(&pixels, Fourcc::Argb8888, (side, side).into(), false).ok()?;
    RINGS.with(|r| {
        let mut rings = r.borrow_mut();
        rings.retain(|(context, _, _, c), _| *context != key.0 || *c == key.3);
        rings.insert(key, Box::new(texture.clone()));
    });
    Some(texture)
}

fn corner_pictures<R>(outline: &Outline, texture: &R::TextureId, place: &Placer, outer: (i32, i32, i32, i32), k: i32, s: u32, alpha: f32) -> Vec<Picture<R>>
where
    R: Renderer + ImportMem,
    R::TextureId: Clone + Texture + 'static,
{
    let (x0, y0, w, h) = outer;
    let quarter = (k * s as i32) as f64;
    let places = [(x0, y0, 0.0, 0.0), (x0 + w - k, y0, quarter, 0.0), (x0, y0 + h - k, 0.0, quarter),
                  (x0 + w - k, y0 + h - k, quarter, quarter)];
    places.iter().zip(outline.corners.iter()).map(|(&(x, y, sx, sy), id)| {
        let src = Rectangle::new((sx, sy).into(), (quarter, quarter).into());
        Picture::new(id.clone(), outline.commit, texture.clone(), src, place.span(x, y, k, k), alpha)
    }).collect()
}

fn side_pieces(outline: &mut Outline, place: &Placer, outer: (i32, i32, i32, i32), k: i32, colour: [f32; 4], alpha: f32) -> Vec<SolidColorRenderElement> {
    let (x0, y0, w, h) = outer;
    let across: Size<i32, Logical> = ((w - 2 * k).max(0), 1).into();
    let down: Size<i32, Logical> = (1, (h - 2 * k).max(0)).into();
    let sides = [(across, x0 + k, y0), (across, x0 + k, y0 + h - 1), (down, x0, y0 + k), (down, x0 + w - 1, y0 + k)];
    let mut solid = Vec::with_capacity(4);
    for (buffer, (size, x, y)) in outline.sides.iter_mut().zip(sides) {
        buffer.update(size, colour);
        solid.push(SolidColorRenderElement::from_buffer(buffer, place.at(x, y), place.scale, alpha, Kind::Unspecified));
    }
    solid
}

pub fn elements<R>(
    window: &WindowElement,
    renderer: &mut R,
    location: Point<i32, Physical>,
    scale: Scale<f64>,
    alpha: f32,
) -> (Vec<Picture<R>>, Vec<SolidColorRenderElement>)
where
    R: Renderer + ImportMem,
    R::TextureId: Clone + Texture + 'static,
{
    let none = (Vec::new(), Vec::new());
    if !wanted(window) {
        return none;
    }
    let geo = smithay::desktop::space::SpaceElement::geometry(&window.0);
    if geo.is_empty() {
        return none;
    }
    let (theme, radius) = ssd::theme();
    let colour = ssd::premultiplied(theme.edge);
    let radius = radius.min(geo.size.w / 2).min(geo.size.h / 2).max(0);
    let s = whole_scale(scale);
    let Some(texture) = ring(renderer, radius, s, theme.edge) else { return none };

    window.user_data().insert_if_missing(|| RefCell::new(Outline::default()));
    let mut outline = window.user_data().get::<RefCell<Outline>>().unwrap().borrow_mut();
    let drawn = Some((s, radius, theme.edge.map(f32::to_bits)));
    if outline.drawn != drawn {
        outline.drawn = drawn;
        outline.commit.increment();
    }

    let outer = (geo.loc.x - 1, geo.loc.y - 1, geo.size.w + 2, geo.size.h + 2);
    let place = Placer { location, scale };
    let corners = corner_pictures(&outline, &texture, &place, outer, radius + 1, s, alpha);
    let solid = side_pieces(&mut outline, &place, outer, radius + 1, colour, alpha);
    (corners, solid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alpha_at(bytes: &[u8], side: usize, x: usize, y: usize) -> u8 {
        bytes[(y * side + x) * 4 + 3]
    }

    #[test]
    fn the_ring_is_a_thin_line_with_nothing_inside_or_out() {
        let bytes = paint_ring(10, 1, [1.0, 1.0, 1.0, 1.0]);
        let side = 22;
        assert_eq!(bytes.len(), side * side * 4);
        assert!(alpha_at(&bytes, side, 11, 0) > 200);
        assert!(alpha_at(&bytes, side, 0, 11) > 200);
        assert_eq!(alpha_at(&bytes, side, 0, 0), 0);
        assert_eq!(alpha_at(&bytes, side, 11, 11), 0);
        assert_eq!(alpha_at(&bytes, side, 11, 2), 0);
    }

    #[test]
    fn a_faint_colour_stays_faint() {
        let bytes = paint_ring(10, 2, [1.0, 1.0, 1.0, 0.18]);
        let peak = bytes.chunks(4).map(|p| p[3]).max().unwrap();
        assert!(peak > 40 && peak <= 46, "{peak}");
    }
}
