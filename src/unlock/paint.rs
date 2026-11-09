//! What the unlock screen looks like: near black, the Trindle mark, and one
//! round box with the dots of what has been typed.

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

use super::fb::Screen;
use super::logo::Logo;
use trindle_toolkit::t;

const SHEET: &[u8] = include_bytes!("../../share/style.css");

const fn sheet(name: &[u8]) -> (u8, u8, u8) {
    const fn digit(b: u8) -> u8 {
        match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            _ => panic!("the sheet must give this colour as #rrggbb, in small letters"),
        }
    }
    const fn pair(at: usize) -> u8 {
        digit(SHEET[at]) * 16 + digit(SHEET[at + 1])
    }
    let mut i = 0;
    while i + name.len() + 8 < SHEET.len() {
        let mut same = 0;
        while same < name.len() && SHEET[i + same] == name[same] {
            same += 1;
        }
        if same == name.len() {
            let mut at = i + same;
            while SHEET[at] == b' ' {
                at += 1;
            }
            assert!(SHEET[at] == b'#', "the sheet must give this colour as #rrggbb");
            return (pair(at + 1), pair(at + 3), pair(at + 5));
        }
        i += 1;
    }
    panic!("the sheet has no such name")
}

const BACK: (u8, u8, u8) = (0x09, 0x09, 0x0b);

const CARD: (u8, u8, u8) = sheet(b"--trindle-card:");

const BOX: (u8, u8, u8) = sheet(b"--trindle-typing-box:");
const TEXT: (u8, u8, u8) = sheet(b"--trindle-text:");

const WRONG: (u8, u8, u8) = sheet(b"--trindle-shade-red:");

const ACCENT: (u8, u8, u8) = (0xfb, 0xc7, 0x11);

const NAVY: (u8, u8, u8) = (0x11, 0x18, 0x27);

const FONT: &[u8] = include_bytes!("../../share/fonts/trindle-sans/TrindleSans-Variable.ttf");

#[derive(Clone, Copy)]
pub struct Spot {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl Spot {
    pub fn holds(&self, x: f32, y: f32) -> bool {
        (self.x0..=self.x1).contains(&x) && (self.y0..=self.y1).contains(&y)
    }
}

fn unit(screen: &Screen) -> f32 {
    (screen.height as f32 / 900.0).clamp(0.6, 2.5)
}

struct Frame {
    w: usize,
    h: usize,
    unit: f32,
}

impl Frame {
    fn middle(&self) -> usize {
        self.w / 2
    }

    fn offset_from_middle(&self, n: f32) -> usize {
        (self.h as f32 / 2.0 + n * self.unit).max(0.0) as usize
    }
}

pub fn draw(screen: &mut Screen, prompt: &str, typed: &str, show: bool, empty: bool,
            pointer: Option<(f32, f32)>) -> Spot {
    let font = FontRef::try_from_slice(FONT).ok();
    screen.fill(BACK);
    let f = Frame { w: screen.width, h: screen.height, unit: unit(screen) };
    mark(screen, f.middle() as f32, (f.h / 2) as f32 - 232.0 * f.unit, 44.0 * f.unit, 0.0);
    let card_w = draw_card(screen, font.as_ref(), &f, prompt, show, empty);
    let eye = draw_box(screen, font.as_ref(), &f, typed, show, card_w);
    screen.keep();
    if let Some((px, py)) = pointer {
        arrow(screen, px, py, f.unit);
    }
    screen.show();
    eye
}

fn draw_card(screen: &mut Screen, font: Option<&FontRef>, f: &Frame, prompt: &str, show: bool, empty: bool) -> usize {
    let (middle, unit) = (f.middle(), f.unit);
    let card_w = ((684.0 * unit) as usize).min(f.w.saturating_sub(40));
    let card_top = f.offset_from_middle(-150.0);
    rounded(screen, middle - card_w / 2, card_top, card_w, f.offset_from_middle(136.0) - card_top, (8.0 * unit) as usize, CARD);
    if let Some(font) = font {
        write(screen, font, prompt, middle, f.offset_from_middle(-92.0), 30.0 * unit, TEXT);

        keys_line(screen, font, &[("Tab", if show { t!("unlock-hide") } else { t!("unlock-show") }), ("F1", t!("unlock-restart")), ("F2", t!("unlock-turn-off"))],
                  middle, f.offset_from_middle(86.0), unit);
        if empty {
            write(screen, font, t!("unlock-type-first"), middle, f.offset_from_middle(30.0), 22.0 * unit, WRONG);
        }
    }
    card_w
}

fn draw_box(screen: &mut Screen, font: Option<&FontRef>, f: &Frame, typed: &str, show: bool, card_w: usize) -> Spot {
    let (middle, unit) = (f.middle(), f.unit);

    let pad = (32.0 * unit) as usize;
    let box_w = card_w.saturating_sub(2 * pad);
    let box_h = (64.0 * unit) as usize;
    let x = middle - box_w / 2;
    let y = f.offset_from_middle(-66.0);
    let rad = trindle_toolkit::theme::corner_px(trindle_toolkit::theme::Part::Box, box_h as f32, 1.0) as usize;
    rounded(screen, x, y, box_w, box_h, rad, BOX);
    ring(screen, x, y, box_w, box_h, rad, ACCENT, 1.5 * unit);

    let eye_x = (x + box_w) as f32 - box_h as f32 * 0.62;
    let centre_y = (y + box_h / 2) as f32;
    eye(screen, eye_x, centre_y, box_h as f32 * 0.24, show, unit);

    let clear = box_h as f32 * 0.62;
    let middle_of_text = middle - (clear / 2.0) as usize;
    let room = box_w.saturating_sub(clear as usize + box_h / 2);

    if show {
        if let Some(font) = font {
            write(screen, font, typed, middle_of_text, y + box_h / 2 + (9.0 * unit) as usize, 26.0 * unit, TEXT);
        }
    } else {
        dots(screen, middle_of_text, y + box_h / 2, typed.chars().count(), 7.0 * unit, 24.0 * unit, room);
    }

    let reach = box_h as f32 * 0.55;
    Spot { x0: eye_x - reach, y0: y as f32, x1: (x + box_w) as f32 + reach * 0.3, y1: (y + box_h) as f32 }
}

fn arrow_size(unit: f32) -> f32 {
    32.0 * unit
}

pub fn move_pointer(screen: &mut Screen, from: Option<(f32, f32)>, to: (f32, f32)) {
    let unit = unit(screen);
    let reach = arrow_size(unit) + 3.0 * unit;
    let rows = |at: (f32, f32)| ((at.1 - 2.0).max(0.0) as usize, (at.1 + reach) as usize + 1);
    let (mut top, mut bottom) = rows(to);
    if let Some(old) = from {
        let (t, b) = rows(old);
        screen.restore_rows(t, b);
        top = top.min(t);
        bottom = bottom.max(b);
    }
    arrow(screen, to.0, to.1, unit);
    screen.show_rows(top, bottom.min(screen.height));
}

fn arrow(screen: &mut Screen, x: f32, y: f32, unit: f32) {
    let s = arrow_size(unit) / 20.0;

    let shape: [(f32, f32); 7] = [(0.0, 0.0), (0.0, 17.0), (4.2, 13.2), (7.2, 19.8), (10.0, 18.6),
                                  (7.1, 12.1), (12.3, 12.1)];
    let points: Vec<(f32, f32)> = shape.iter().map(|&(a, b)| (x + a * s, y + b * s)).collect();
    let edge = (1.3 * unit).max(1.0);
    let (x0, x1) = ((x - edge - 1.0).max(0.0) as usize, (x + 13.0 * s + edge) as usize + 1);
    let (y0, y1) = ((y - edge - 1.0).max(0.0) as usize, (y + 21.0 * s + edge) as usize + 1);
    for py in y0..=y1 {
        for px in x0..=x1 {
            let (fx, fy) = (px as f32 + 0.5, py as f32 + 0.5);
            let inside = inside(&points, fx, fy);
            let near = distance_to_outline(&points, fx, fy);
            if inside && near > edge {
                screen.blend(px, py, TEXT, 1.0);
            } else if inside || near <= edge {
                screen.blend(px, py, BACK, 1.0);
            }
        }
    }
}

fn inside(points: &[(f32, f32)], x: f32, y: f32) -> bool {
    let mut crossed = false;
    for i in 0..points.len() {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            crossed = !crossed;
        }
    }
    crossed
}

fn distance_to_outline(points: &[(f32, f32)], x: f32, y: f32) -> f32 {
    (0..points.len()).map(|i| {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((x - a.0) * dx + (y - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        ((x - a.0 - t * dx).powi(2) + (y - a.1 - t * dy).powi(2)).sqrt()
    }).fold(f32::MAX, f32::min)
}

fn keycap(screen: &mut Screen, font: &FontRef, label: &str, cx: f32, cy: f32, w: f32, h: f32, _unit: f32) {
    let (x, y) = ((cx - w / 2.0).max(0.0) as usize, (cy - h / 2.0).max(0.0) as usize);
    rounded(screen, x, y, w as usize, h as usize, trindle_toolkit::theme::corner_px(trindle_toolkit::theme::Part::Button, h, 1.0) as usize, BOX);
    let size = h * 0.52;
    write(screen, font, label, cx as usize, (cy + size * 0.36) as usize, size, TEXT);
}

fn keys_line(screen: &mut Screen, font: &FontRef, items: &[(&str, &str)], middle: usize, cy: usize, unit: f32) {
    let size = 24.0 * unit;
    let cap_h = size * 1.45;
    let cap_w = cap_h * 1.5;
    let gap = size * 0.45;
    let between = size * 1.6;
    let scaled = font.as_scaled(PxScale::from(size));
    let width_of = |t: &str| -> f32 { t.chars().map(|c| scaled.h_advance(font.glyph_id(c))).sum() };
    let total: f32 = items.iter().map(|(_, word)| cap_w + gap + width_of(word)).sum::<f32>()
        + between * (items.len().saturating_sub(1)) as f32;
    let mut pen = middle as f32 - total / 2.0;
    for (key, word) in items {
        keycap(screen, font, key, pen + cap_w / 2.0, cy as f32, cap_w, cap_h, unit);
        pen += cap_w + gap;
        let wide = width_of(word);
        write(screen, font, word, (pen + wide / 2.0) as usize, (cy as f32 + size * 0.36) as usize, size, TEXT);
        pen += wide + between;
    }
}

fn eye(screen: &mut Screen, cx: f32, cy: f32, size: f32, crossed: bool, unit: f32) {
    let thick = (2.2 * unit).max(1.5);

    let r = size * 1.55;
    let drop = r - size * 0.62;
    let (x0, x1) = ((cx - size * 1.3) as usize, (cx + size * 1.3) as usize + 1);
    let (y0, y1) = ((cy - size) as usize, (cy + size) as usize + 1);
    for py in y0..=y1 {
        for px in x0..=x1 {
            let (fx, fy) = (px as f32, py as f32);
            let upper = ((fx - cx).powi(2) + (fy - (cy + drop)).powi(2)).sqrt();
            let lower = ((fx - cx).powi(2) + (fy - (cy - drop)).powi(2)).sqrt();

            let on_upper = (thick / 2.0 - (upper - r).abs() + 0.5).clamp(0.0, 1.0) * (lower <= r + 0.5) as u8 as f32;
            let on_lower = (thick / 2.0 - (lower - r).abs() + 0.5).clamp(0.0, 1.0) * (upper <= r + 0.5) as u8 as f32;
            let on = on_upper.max(on_lower);
            if on > 0.0 {
                screen.blend(px, py, TEXT, on);
            }
        }
    }
    disc(screen, cx, cy, size * 0.40, TEXT, 1.0);
    disc(screen, cx, cy, size * 0.17, BOX, 1.0);
    if crossed {

        line(screen, cx - size * 1.05, cy - size * 0.85, cx + size * 1.05, cy + size * 0.85, thick * 3.2, BOX);
        line(screen, cx - size * 1.05, cy - size * 0.85, cx + size * 1.05, cy + size * 0.85, thick, TEXT);
    }
}

fn line(screen: &mut Screen, ax: f32, ay: f32, bx: f32, by: f32, thick: f32, colour: (u8, u8, u8)) {
    let pad = thick;
    let (x0, x1) = ((ax.min(bx) - pad).max(0.0) as usize, (ax.max(bx) + pad) as usize);
    let (y0, y1) = ((ay.min(by) - pad).max(0.0) as usize, (ay.max(by) + pad) as usize);
    let (dx, dy) = (bx - ax, by - ay);
    let len2 = (dx * dx + dy * dy).max(0.0001);
    for py in y0..=y1 {
        for px in x0..=x1 {
            let (fx, fy) = (px as f32, py as f32);
            let t = (((fx - ax) * dx + (fy - ay) * dy) / len2).clamp(0.0, 1.0);
            let d = ((fx - (ax + t * dx)).powi(2) + (fy - (ay + t * dy)).powi(2)).sqrt();
            let on = (thick / 2.0 - d + 0.5).clamp(0.0, 1.0);
            if on > 0.0 {
                screen.blend(px, py, colour, on);
            }
        }
    }
}

pub fn goodbye(screen: &mut Screen) {
    screen.fill(BACK);
    screen.show();
}

fn dots(screen: &mut Screen, middle: usize, y: usize, count: usize, r: f32, gap: f32, width: usize) {
    if count == 0 {
        return;
    }
    let most = ((width as f32 - 80.0) / gap) as usize;
    let shown = count.min(most.max(1));
    let span = (shown as f32 - 1.0) * gap;
    let mut x = middle as f32 - span / 2.0;
    for _ in 0..shown {
        disc(screen, x, y as f32, r, TEXT, 1.0);
        x += gap;
    }
}

fn mark(screen: &mut Screen, x: f32, y: f32, r: f32, turn: f32) {
    let u = r / 31.0;
    disc(screen, x, y, r, ACCENT, 1.0);
    circle(screen, x, y, 21.5 * u, NAVY, 4.5 * u);
    for i in 0..8 {
        let a = turn + i as f32 * std::f32::consts::PI / 4.0;
        disc(screen, x + a.cos() * 13.5 * u, y + a.sin() * 13.5 * u, 2.6 * u, NAVY, 1.0);
    }
}

pub fn turning(screen: &mut Screen, turn: f32, whole: bool, logo: Option<&Logo>) {
    let unit = unit(screen);
    let r = 44.0 * unit;
    let placed = logo.and_then(|l| Some((l, l.place(screen.width, screen.height)?)));
    let (x, y) = match placed {

        Some(_) => (screen.width as f32 / 2.0, screen.height as f32 * 0.75),
        None => (screen.width as f32 / 2.0, screen.height as f32 / 2.0),
    };
    let back = if placed.is_some() { (0, 0, 0) } else { BACK };

    let (top, bottom) = ((y - r - 2.0).max(0.0) as usize, ((y + r + 2.0) as usize).min(screen.height));
    if whole {
        screen.fill(back);
        if let Some((logo, (lx, ly))) = placed {
            for py in 0..logo.height {
                for px in 0..logo.width {
                    screen.blend(lx + px, ly + py, logo.pixel(px, py), 1.0);
                }
            }
        }
    } else {
        let left = (x - r - 2.0).max(0.0) as usize;
        let right = ((x + r + 2.0) as usize).min(screen.width);
        for py in top..bottom {
            for px in left..right {
                screen.blend(px, py, back, 1.0);
            }
        }
    }
    mark(screen, x, y, r, turn);
    if whole {
        screen.show();
    } else {
        screen.show_rows(top, bottom);
    }
}

fn disc(screen: &mut Screen, cx: f32, cy: f32, r: f32, colour: (u8, u8, u8), alpha: f32) {
    let (x0, x1) = ((cx - r - 1.0).max(0.0) as usize, (cx + r + 1.0) as usize);
    let (y0, y1) = ((cy - r - 1.0).max(0.0) as usize, (cy + r + 1.0) as usize);
    for y in y0..=y1 {
        for x in x0..=x1 {
            let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
            let edge = (r - d + 0.5).clamp(0.0, 1.0);
            if edge > 0.0 {
                screen.blend(x, y, colour, edge * alpha);
            }
        }
    }
}

fn circle(screen: &mut Screen, cx: f32, cy: f32, r: f32, colour: (u8, u8, u8), thick: f32) {
    let outer = r + thick / 2.0;
    let (x0, x1) = ((cx - outer - 1.0).max(0.0) as usize, (cx + outer + 1.0) as usize);
    let (y0, y1) = ((cy - outer - 1.0).max(0.0) as usize, (cy + outer + 1.0) as usize);
    for y in y0..=y1 {
        for x in x0..=x1 {
            let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
            let on = (thick / 2.0 - (d - r).abs() + 0.5).clamp(0.0, 1.0);
            if on > 0.0 {
                screen.blend(x, y, colour, on);
            }
        }
    }
}

fn rounded(screen: &mut Screen, x: usize, y: usize, w: usize, h: usize, r: usize, colour: (u8, u8, u8)) {
    let r = r.min(h / 2).min(w / 2) as f32;
    for yy in y..y + h {
        for xx in x..x + w {
            let dx = ((x as f32 + r) - xx as f32).max(xx as f32 - (x + w) as f32 + r).max(0.0);
            let dy = ((y as f32 + r) - yy as f32).max(yy as f32 - (y + h) as f32 + r).max(0.0);
            let out = (dx * dx + dy * dy).sqrt() - r;
            let on = (0.5 - out).clamp(0.0, 1.0);
            if on > 0.0 {
                screen.blend(xx, yy, colour, on);
            }
        }
    }
}

fn ring(screen: &mut Screen, x: usize, y: usize, w: usize, h: usize, r: usize, colour: (u8, u8, u8), thick: f32) {
    outline(screen, x, y, w, h, r, colour, thick, 1.0);
}

fn outline(screen: &mut Screen, x: usize, y: usize, w: usize, h: usize, r: usize, colour: (u8, u8, u8),
           thick: f32, alpha: f32) {
    let r = r.min(h / 2).min(w / 2) as f32;
    for yy in y.saturating_sub(2)..y + h + 2 {
        for xx in x.saturating_sub(2)..x + w + 2 {
            let dx = ((x as f32 + r) - xx as f32).max(xx as f32 - (x + w) as f32 + r).max(0.0);
            let dy = ((y as f32 + r) - yy as f32).max(yy as f32 - (y + h) as f32 + r).max(0.0);
            let out = (dx * dx + dy * dy).sqrt() - r;
            let on = (thick / 2.0 - out.abs() + 0.5).clamp(0.0, 1.0);
            if on > 0.0 {
                screen.blend(xx, yy, colour, on * alpha);
            }
        }
    }
}

fn write(screen: &mut Screen, font: &FontRef, text: &str, middle: usize, baseline: usize,
         size: f32, colour: (u8, u8, u8)) {
    if text.is_empty() {
        return;
    }
    let scaled = font.as_scaled(PxScale::from(size));
    let width: f32 = text.chars().map(|c| scaled.h_advance(font.glyph_id(c))).sum();
    let mut pen = middle as f32 - width / 2.0;
    for c in text.chars() {
        let id = font.glyph_id(c);
        let glyph = id.with_scale_and_position(PxScale::from(size), ab_glyph::point(pen, baseline as f32));
        if let Some(drawn) = font.outline_glyph(glyph) {
            let bounds = drawn.px_bounds();
            drawn.draw(|gx, gy, on| {
                let x = bounds.min.x as i32 + gx as i32;
                let y = bounds.min.y as i32 + gy as i32;
                if x >= 0 && y >= 0 {
                    screen.blend(x as usize, y as usize, colour, on);
                }
            });
        }
        pen += scaled.h_advance(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_turning_mark_is_drawn() {
        let mut screen = Screen::in_memory(1280, 800);
        turning(&mut screen, 0.6, true, None);
        let rgb = screen.rgb();
        if let Some(dir) = std::env::var_os("UNLOCK_PREVIEW") {
            let mut ppm = format!("P6 {} {} 255\n", screen.width, screen.height).into_bytes();
            ppm.extend_from_slice(&rgb);
            std::fs::write(std::path::Path::new(&dir).join("turning.ppm"), ppm).unwrap();
        }
        assert!(rgb.chunks(3).any(|p| p == [ACCENT.0, ACCENT.1, ACCENT.2]), "no mark drawn");
    }

    #[test]
    fn the_eye_is_there_and_crossed_only_when_the_password_shows() {
        for (show, name) in [(false, "hidden"), (true, "shown")] {
            let mut screen = Screen::in_memory(1280, 800);
            let eye = draw(&mut screen, "Enter encryption password", "correct horse #battery", show, false,
                           show.then_some((930.0, 420.0)));
            assert!(eye.holds(880.0, 370.0), "a click on the eye would miss it");
            assert!(!eye.holds(640.0, 370.0), "a click in the middle of the box would count as the eye");
            let rgb = screen.rgb();
            let accent = rgb.chunks(3).filter(|p| *p == [ACCENT.0, ACCENT.1, ACCENT.2]).count();
            if let Some(dir) = std::env::var_os("UNLOCK_PREVIEW") {
                let mut ppm = format!("P6 {} {} 255\n", screen.width, screen.height).into_bytes();
                ppm.extend_from_slice(&rgb);
                std::fs::write(std::path::Path::new(&dir).join(format!("unlock-{name}.ppm")), ppm).unwrap();
            }
            assert!(accent > 0, "nothing drawn in the accent colour");
        }
    }

    #[test]
    fn the_card_fits_big_and_small_screens() {
        for (w, h) in [(1920, 1080), (1024, 768), (800, 600)] {
            let mut screen = Screen::in_memory(w, h);
            draw(&mut screen, "Enter encryption password", "secret", false, true, None);
            let rgb = screen.rgb();
            let at = |x: usize, y: usize| {
                let i = (y * w + x) * 3;
                (rgb[i], rgb[i + 1], rgb[i + 2])
            };
            assert_eq!(at(2, 2), BACK, "the screen behind at {w}x{h}");

            let unit = unit(&screen);
            let card_w = ((684.0 * unit) as usize).min(w - 40);
            assert_eq!(at(w / 2 - card_w / 2 + 3, h / 2 - (66.0 * unit) as usize), CARD, "the card at {w}x{h}");
            if let Some(dir) = std::env::var_os("UNLOCK_PREVIEW") {
                let mut ppm = format!("P6 {w} {h} 255\n").into_bytes();
                ppm.extend_from_slice(&rgb);
                std::fs::write(std::path::Path::new(&dir).join(format!("unlock-{w}.ppm")), ppm).unwrap();
            }
        }
    }
}

