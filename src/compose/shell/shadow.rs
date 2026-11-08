//! The soft shadow an ordinary window casts, as the house style has
//! everything that floats over the page cast one. It is the style sheet's
//! float shadow, the one menus and the label on pointing cast, drawn in the
//! sheet's shadow colour below the window and its title bar. Nothing is
//! cast by a window that fills the screen or is maximised, by an X11 menu
//! or tooltip, by a program that draws a shadow of its own round its
//! window, or by panels and other layer-shell surfaces, which paint their
//! own.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::Id;
use smithay::backend::renderer::utils::CommitCounter;
use smithay::backend::renderer::{ErasedContextId, ImportMem, Renderer, Texture};
use smithay::utils::{Buffer, Physical, Point, Rectangle, Scale};

use super::element::frame_extra;
use super::picture::{whole_scale, Picture, Placer};
use super::{ssd, WindowElement};

struct Cast {

    pieces: [Id; 8],
    commit: CommitCounter,
    drawn: Option<Key>,
}

impl Default for Cast {
    fn default() -> Self {
        Cast { pieces: std::array::from_fn(|_| Id::new()), commit: CommitCounter::default(), drawn: None }
    }
}

type Key = (i32, i32, i32, u32, [[u32; 4]; 3], [u32; 4]);

thread_local! {

    static PICTURES: RefCell<HashMap<(ErasedContextId, Key), Box<dyn Any>>> = RefCell::new(HashMap::new());
}

fn wanted(window: &WindowElement) -> bool {
    if window.0.x11_surface().is_some_and(|x| x.is_override_redirect()) {
        return false;
    }
    if window.is_fullscreen() || window.is_maximized() {
        return false;
    }
    if window.has_ssd() {
        return true;
    }
    let geo = smithay::desktop::space::SpaceElement::geometry(&window.0);
    let bbox = smithay::desktop::space::SpaceElement::bbox(&window.0);
    !geo.is_empty() && bbox.loc.x >= geo.loc.x && bbox.loc.y >= geo.loc.y
        && bbox.loc.x + bbox.size.w <= geo.loc.x + geo.size.w && bbox.loc.y + bbox.size.h <= geo.loc.y + geo.size.h
}

fn reach(layers: &[[f32; 4]]) -> i32 {
    layers.iter()
        .filter(|l| l[3] > 0.0)
        .map(|&[y, blur, spread, _]| (y + blur + spread.max(0.0)).ceil() as i32 + 1)
        .max()
        .unwrap_or(0)
}

fn cover(x: f32, y: f32, lo: (f32, f32), hi: (f32, f32), radius: f32) -> f32 {
    let r = radius.min((hi.0 - lo.0) / 2.0).min((hi.1 - lo.1) / 2.0).max(0.0);
    let dx = (lo.0 + r - x).max(x - (hi.0 - r)).max(0.0);
    let dy = (lo.1 + r - y).max(y - (hi.1 - r)).max(0.0);
    let inside = if dx > 0.0 && dy > 0.0 {
        r - (dx * dx + dy * dy).sqrt()
    } else {
        (x - lo.0).min(hi.0 - x).min(y - lo.1).min(hi.1 - y)
    };
    (inside + 0.5).clamp(0.0, 1.0)
}

fn blur_line(v: &mut [f32], from: usize, step: usize, n: usize, r: usize, held: &mut Vec<f32>) {
    held.clear();
    held.extend((0..n).map(|i| v[from + i * step]));
    let width = (2 * r + 1) as f32;
    let mut sum: f32 = held.iter().take(r).sum();
    for i in 0..n {
        if i + r < n {
            sum += held[i + r];
        }
        v[from + i * step] = sum / width;
        if i >= r {
            sum -= held[i - r];
        }
    }
}

fn paint(layers: &[[f32; 4]], colour: [f32; 4], radius: i32, reach: i32, corner: i32, s: u32) -> Vec<u8> {
    let sf = s as f32;
    let side = ((2 * (reach + corner) + 1) as u32 * s) as usize;
    let lo = reach as f32 * sf;
    let hi = (reach + 2 * corner + 1) as f32 * sf;
    let mut clear = vec![1.0f32; side * side];
    let mut layer = vec![0.0f32; side * side];
    let mut held = Vec::with_capacity(side);
    for &[y, blur, spread, strength] in layers.iter().filter(|l| l[3] > 0.0) {
        let (down, grow) = (y * sf, spread * sf);
        for py in 0..side {
            for px in 0..side {
                let (cx, cy) = (px as f32 + 0.5, py as f32 + 0.5);
                layer[py * side + px] = cover(cx, cy, (lo - grow, lo - grow + down), (hi + grow, hi + grow + down), (radius as f32 * sf + grow).max(0.0));
            }
        }

        let r = (blur * sf / 2.0).round() as usize;
        if r > 0 {
            for _ in 0..3 {
                for row in 0..side {
                    blur_line(&mut layer, row * side, 1, side, r, &mut held);
                }
                for col in 0..side {
                    blur_line(&mut layer, col, side, side, r, &mut held);
                }
            }
        }
        for (c, l) in clear.iter_mut().zip(&layer) {
            *c *= 1.0 - l * strength * colour[3];
        }
    }
    let mut bytes = Vec::with_capacity(side * side * 4);
    for py in 0..side {
        for px in 0..side {
            let under = cover(px as f32 + 0.5, py as f32 + 0.5, (lo, lo), (hi, hi), radius as f32 * sf);
            let a = (1.0 - clear[py * side + px]) * (1.0 - under);
            let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            bytes.extend_from_slice(&[byte(colour[2] * a), byte(colour[1] * a), byte(colour[0] * a), byte(a)]);
        }
    }
    bytes
}

fn picture<R>(renderer: &mut R, key: Key, layers: &[[f32; 4]], colour: [f32; 4]) -> Option<R::TextureId>
where
    R: Renderer + ImportMem,
    R::TextureId: Clone + 'static,
{
    let (radius, reach, corner, s, _, _) = key;
    let id = (renderer.context_id().erased(), key);
    let found = PICTURES.with(|p| p.borrow().get(&id).and_then(|t| t.downcast_ref::<R::TextureId>()).cloned());
    if found.is_some() {
        return found;
    }
    let side = (2 * (reach + corner) + 1) * s as i32;
    let pixels = paint(layers, colour, radius, reach, corner, s);
    let texture = renderer.import_memory(&pixels, Fourcc::Argb8888, (side, side).into(), false).ok()?;
    PICTURES.with(|p| {
        let mut pictures = p.borrow_mut();

        pictures.retain(|(context, k), _| *context != id.0 || (k.4, k.5) == (key.4, key.5));
        pictures.insert(id, Box::new(texture.clone()));
    });
    Some(texture)
}

fn floating_box(window: &WindowElement) -> Option<(i32, i32, i32, i32)> {
    let (x0, y0, w, h) = if window.has_ssd() {
        let client = window.client_size();
        let extra = frame_extra(true);
        (0, 0, client.w + extra.w, client.h + extra.h)
    } else {
        let geo = smithay::desktop::space::SpaceElement::geometry(&window.0);
        (geo.loc.x - 1, geo.loc.y - 1, geo.size.w + 2, geo.size.h + 2)
    };
    (w > 0 && h > 0).then_some((x0, y0, w, h))
}

fn pieces(place: &Placer, floating: (i32, i32, i32, i32), reach: i32, corner: i32, s: u32) -> [(Rectangle<i32, Physical>, Rectangle<f64, Buffer>); 8] {
    let (x0, y0, w, h) = floating;

    let side = 2 * (reach + corner) + 1;
    let cw = (reach + corner).min(reach + w / 2);
    let ch = (reach + corner).min(reach + h / 2);
    let (ox, oy) = (x0 - reach, y0 - reach);
    let (ow, oh) = (w + 2 * reach, h + 2 * reach);
    let span = |x: i32, y: i32, pw: i32, ph: i32| place.span(x, y, pw, ph);
    let sf = s as f64;
    let src = |x: i32, y: i32, pw: i32, ph: i32| Rectangle::new((x as f64 * sf, y as f64 * sf).into(), (pw as f64 * sf, ph as f64 * sf).into());
    let mid = reach + corner;
    let (across, down) = (ow - 2 * cw, oh - 2 * ch);

    [
        (span(ox, oy, cw, ch), src(0, 0, cw, ch)),
        (span(ox + ow - cw, oy, cw, ch), src(side - cw, 0, cw, ch)),
        (span(ox, oy + oh - ch, cw, ch), src(0, side - ch, cw, ch)),
        (span(ox + ow - cw, oy + oh - ch, cw, ch), src(side - cw, side - ch, cw, ch)),
        (span(ox + cw, oy, across, reach), src(mid, 0, 1, reach)),
        (span(ox + cw, oy + oh - reach, across, reach), src(mid, side - reach, 1, reach)),
        (span(ox, oy + ch, reach, down), src(0, mid, reach, 1)),
        (span(ox + ow - reach, oy + ch, reach, down), src(side - reach, mid, reach, 1)),
    ]
}

pub fn elements<R>(window: &WindowElement, renderer: &mut R, location: Point<i32, Physical>, scale: Scale<f64>, alpha: f32) -> Vec<Picture<R>>
where
    R: Renderer + ImportMem,
    R::TextureId: Clone + Texture + 'static,
{
    if !wanted(window) {
        return Vec::new();
    }
    let (theme, radius) = ssd::theme();
    let layers = theme.shadow;
    let reach = reach(&layers);
    if reach == 0 {
        return Vec::new();
    }
    let Some(floating) = floating_box(window) else { return Vec::new() };
    let radius = radius.max(0);
    let corner = radius + reach;
    let s = whole_scale(scale);
    let key: Key = (radius, reach, corner, s, layers.map(|l| l.map(f32::to_bits)), theme.shadow_colour.map(f32::to_bits));
    let Some(texture) = picture(renderer, key, &layers, theme.shadow_colour) else { return Vec::new() };

    window.user_data().insert_if_missing(|| RefCell::new(Cast::default()));
    let mut cast = window.user_data().get::<RefCell<Cast>>().unwrap().borrow_mut();
    if cast.drawn != Some(key) {
        cast.drawn = Some(key);
        cast.commit.increment();
    }

    let pieces = pieces(&Placer { location, scale }, floating, reach, corner, s);
    pieces.into_iter()
        .zip(cast.pieces.iter())
        .filter(|((geo, _), _)| !geo.is_empty())
        .map(|((geo, src), id)| Picture::new(id.clone(), cast.commit, texture.clone(), src, geo, alpha))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOAT: [[f32; 4]; 2] = [[0.0, 0.0, 1.0, 0.5], [10.0, 28.0, 0.0, 0.55]];

    fn alpha_at(bytes: &[u8], side: usize, x: usize, y: usize) -> u8 {
        bytes[(y * side + x) * 4 + 3]
    }

    #[test]
    fn the_shadow_reaches_furthest_below() {
        assert_eq!(reach(&FLOAT), 39);
        assert_eq!(reach(&[[0.0, 0.0, 0.0, 0.0]]), 0, "a layer of no strength casts nothing");
    }

    #[test]
    fn the_shadow_is_soft_falls_below_and_is_clear_under_the_box() {
        let (radius, reach, corner) = (8, reach(&FLOAT), 8 + reach(&FLOAT));
        let side = (2 * (reach + corner) + 1) as usize;
        let bytes = paint(&FLOAT, [0.0, 0.0, 0.0, 1.0], radius, reach, corner, 1);
        assert_eq!(bytes.len(), side * side * 4);
        let mid = side / 2;
        let (top, bottom) = (reach as usize, side - reach as usize);

        assert_eq!(alpha_at(&bytes, side, mid, mid), 0);
        assert_eq!(alpha_at(&bytes, side, 0, 0), 0);

        assert!(alpha_at(&bytes, side, mid, top - 1) > 120);

        let below = alpha_at(&bytes, side, mid, bottom + 8);
        let above = alpha_at(&bytes, side, mid, top - 8);
        assert!(below > above, "{below} {above}");
        assert!(alpha_at(&bytes, side, mid, bottom + 30) < below);

        assert!(bytes.chunks(4).all(|p| p[0] <= p[3] && p[2] <= p[3]));
    }

    #[test]
    fn the_sides_are_even_along_the_middle() {

        let (reach, corner) = (reach(&FLOAT), 8 + reach(&FLOAT));
        let side = (2 * (reach + corner) + 1) as usize;
        let bytes = paint(&FLOAT, [0.0, 0.0, 0.0, 1.0], 8, reach, corner, 1);
        let mid = side / 2;
        for y in 0..reach as usize {
            let (a, b) = (alpha_at(&bytes, side, mid, y) as i32, alpha_at(&bytes, side, mid + 3, y) as i32);
            assert!((a - b).abs() <= 1, "row {y}: {a} {b}");
        }
    }

    #[test]
    fn a_box_of_a_given_shape_is_covered_inside_and_not_outside() {
        let (lo, hi) = ((0.0, 0.0), (20.0, 20.0));
        assert_eq!(cover(10.0, 10.0, lo, hi, 5.0), 1.0);
        assert_eq!(cover(25.0, 10.0, lo, hi, 5.0), 0.0);

        assert_eq!(cover(0.5, 0.5, lo, hi, 5.0), 0.0);
        assert_eq!(cover(0.5, 0.5, lo, hi, 0.0), 1.0);
    }
}
