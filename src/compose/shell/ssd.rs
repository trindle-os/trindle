//! Title bars the compositor draws itself, for programs that leave it to
//! the desktop: Qt programs, the terminal, and X11 programs. GTK draws its own.

use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont, VariableFont};
use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{solid::SolidColorBuffer, Id},
            utils::CommitCounter,
            ErasedContextId, ImportMem, Renderer,
        },
    },
    utils::{Buffer, Logical, Point, Size},
};

use std::cell::{RefCell, RefMut};

use crate::config::{Theme, TitleColours};

use super::WindowElement;

pub const TITLE_HEIGHT: i32 = 32;

pub const BORDER: i32 = 2;

pub const HEADER_BAR_HEIGHT: i32 = BORDER + TITLE_HEIGHT;
const BUTTON: i32 = 32;

const MAX_TITLE_WIDTH: i32 = 16_384;
const BUTTON_GAP: i32 = 6;

pub(crate) const FONT_PX: f32 = 10.0 * 96.0 / 72.0;
const FONT: &[u8] = include_bytes!("../../../share/fonts/trindle-sans/TrindleSans-Variable.ttf");

use crate::pointer::DOUBLE_CLICK_MS;

static LOOK: Mutex<Option<(Theme, i32)>> = Mutex::new(None);

pub fn set_theme(theme: Theme, radius: i32) {
    if let Ok(mut look) = LOOK.lock() {
        *look = Some((theme, radius));
    }
}

pub(crate) fn theme() -> (Theme, i32) {
    LOOK.lock().ok().and_then(|l| *l).unwrap_or((Theme::default(), 10))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Title,
    Minimize,
    Maximize,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Move,
    Minimize,
    ToggleMaximize,
    Close,
}

fn button_x(width: i32, part: Part) -> i32 {
    let right = width - BORDER;
    match part {
        Part::Close => right - BUTTON,
        Part::Maximize => right - 2 * BUTTON - BUTTON_GAP,
        Part::Minimize => right - 3 * BUTTON - 2 * BUTTON_GAP,
        Part::Title => BORDER,
    }
}

pub fn part_at(width: i32, loc: Point<f64, Logical>) -> Option<Part> {
    let (x, y) = (loc.x, loc.y);
    if y < BORDER as f64 || y >= HEADER_BAR_HEIGHT as f64 || x < BORDER as f64 || x >= (width - BORDER) as f64 {
        return None;
    }
    for part in [Part::Close, Part::Maximize, Part::Minimize] {
        let left = button_x(width, part) as f64;
        if x >= left && x < left + BUTTON as f64 {
            return Some(part);
        }
    }
    Some(Part::Title)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Look {

    pub width: i32,

    pub scale: i32,
    pub title: String,
    pub active: bool,
    pub maximized: bool,
    pub hover: Option<Part>,
    pub pressed: Option<Part>,
}

pub struct WindowState {
    pub is_ssd: bool,
    pub header_bar: HeaderBar,
}

struct Drawn {
    look: Look,
    theme: Theme,
    radius: i32,

    pixels: Vec<u8>,
    textures: HashMap<ErasedContextId, Box<dyn Any>>,
}

pub struct HeaderBar {

    pub pointer_loc: Option<Point<f64, Logical>>,

    pressed: Option<Part>,
    last_click: Option<u32>,
    drawn: Option<Drawn>,

    pub id: Id,
    pub commit: CommitCounter,

    pub sides: [SolidColorBuffer; 3],
}

impl Default for HeaderBar {
    fn default() -> Self {
        HeaderBar {
            pointer_loc: None,
            pressed: None,
            last_click: None,
            drawn: None,
            id: Id::new(),
            commit: CommitCounter::default(),
            sides: Default::default(),
        }
    }
}

impl HeaderBar {
    pub fn pointer_enter(&mut self, loc: Point<f64, Logical>) {
        self.pointer_loc = Some(loc);
    }

    pub fn pointer_leave(&mut self) {
        self.pointer_loc = None;
        self.pressed = None;
    }

    pub fn hover(&self, width: i32) -> Option<Part> {
        self.pointer_loc.and_then(|l| part_at(width, l)).filter(|p| *p != Part::Title)
    }

    pub fn pressed(&self) -> Option<Part> {
        self.pressed
    }

    pub fn press(&mut self, width: i32, time: u32) -> Option<Action> {
        match self.pointer_loc.and_then(|l| part_at(width, l))? {
            Part::Title => {
                let double = self.last_click.is_some_and(|t| time.wrapping_sub(t) <= DOUBLE_CLICK_MS);
                self.last_click = if double { None } else { Some(time) };
                Some(if double { Action::ToggleMaximize } else { Action::Move })
            }
            part => {
                self.pressed = Some(part);
                None
            }
        }
    }

    pub fn release(&mut self, width: i32) -> Option<Action> {
        let pressed = self.pressed.take()?;
        if self.pointer_loc.and_then(|l| part_at(width, l)) != Some(pressed) {
            return None;
        }
        match pressed {
            Part::Minimize => Some(Action::Minimize),
            Part::Maximize => Some(Action::ToggleMaximize),
            Part::Close => Some(Action::Close),
            Part::Title => None,
        }
    }

    pub fn picture<R>(&mut self, renderer: &mut R, look: Look) -> (Option<(R::TextureId, Size<i32, Buffer>)>, TitleColours)
    where
        R: Renderer + ImportMem,
        R::TextureId: Clone + 'static,
    {
        let (theme, radius) = theme();
        let colours = if look.active { theme.active } else { theme.inactive };

        let look = Look { width: look.width.clamp(1, MAX_TITLE_WIDTH), scale: look.scale.clamp(1, 4), ..look };
        let fresh = self.drawn.as_ref().is_some_and(|d| d.look == look && d.theme == theme && d.radius == radius);
        if !fresh {
            let pixels = paint(&look, &theme, radius);
            self.drawn = Some(Drawn { look, theme, radius, pixels, textures: HashMap::new() });
            self.commit.increment();
        }
        let Some(drawn) = self.drawn.as_mut() else { return (None, colours) };
        let size: Size<i32, Buffer> = (drawn.look.width * drawn.look.scale, HEADER_BAR_HEIGHT * drawn.look.scale).into();
        let context = renderer.context_id().erased();
        if let Some(texture) = drawn.textures.get(&context).and_then(|t| t.downcast_ref::<R::TextureId>()) {
            return (Some((texture.clone(), size)), colours);
        }
        if drawn.pixels.is_empty() {
            drawn.pixels = paint(&drawn.look, &drawn.theme, drawn.radius);
        }
        let texture = renderer.import_memory(&drawn.pixels, Fourcc::Argb8888, size, false).ok();
        if let Some(texture) = &texture {
            drawn.textures.insert(context, Box::new(texture.clone()));
            drawn.pixels = Vec::new();
        }
        (texture.map(|t| (t, size)), colours)
    }

    pub fn update_sides(&mut self, client: Size<i32, Logical>, colour: [f32; 4]) {
        let colour = premultiplied(colour);
        let side: Size<i32, Logical> = (BORDER, client.h).into();
        self.sides[0].update(side, colour);
        self.sides[1].update(side, colour);
        self.sides[2].update((client.w + 2 * BORDER, BORDER), colour);
    }
}

pub(crate) fn premultiplied(c: [f32; 4]) -> [f32; 4] {
    [c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3]]
}

#[derive(Clone, Copy)]
pub(crate) struct Paint(pub(crate) [f32; 4]);

impl Paint {
    pub(crate) const CLEAR: Paint = Paint([0.0; 4]);

    pub(crate) fn of(c: [f32; 4], coverage: f32) -> Paint {
        let a = c[3] * coverage.clamp(0.0, 1.0);
        Paint([c[0] * a, c[1] * a, c[2] * a, a])
    }

    pub(crate) fn over(self, below: Paint) -> Paint {
        let k = 1.0 - self.0[3];
        Paint(std::array::from_fn(|i| self.0[i] + below.0[i] * k))
    }
}

pub(crate) fn to_bytes(px: &[Paint]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(px.len() * 4);
    for p in px {
        let [red, green, blue, alpha] = p.0.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8);
        bytes.extend_from_slice(&[blue, green, red, alpha]);
    }
    bytes
}

fn rounded_top(x: f32, y: f32, w: f32, r: f32) -> f32 {
    if y < r && (x < r || x > w - r) {
        let cx = if x < r { r } else { w - r };
        return ((x - cx).powi(2) + (y - r).powi(2)).sqrt() - r;
    }
    (-x).max(x - w).max(-y)
}

fn to_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((p.0 - a.0 - t * dx).powi(2) + (p.1 - a.1 - t * dy).powi(2)).sqrt()
}

fn to_square(p: (f32, f32), c: (f32, f32), half: f32, r: f32) -> f32 {
    let q = ((p.0 - c.0).abs() - (half - r), (p.1 - c.1).abs() - (half - r));
    let outside = (q.0.max(0.0).powi(2) + q.1.max(0.0).powi(2)).sqrt();
    (outside + q.0.max(q.1).min(0.0) - r).abs()
}

fn in_square(p: (f32, f32), c: (f32, f32), half: f32, r: f32) -> f32 {
    let r = r.clamp(0.0, half);
    let q = ((p.0 - c.0).abs() - (half - r), (p.1 - c.1).abs() - (half - r));
    (q.0.max(0.0).powi(2) + q.1.max(0.0).powi(2)).sqrt() + q.0.max(q.1).min(0.0) - r
}

fn glyph_distance(part: Part, maximized: bool, p: (f32, f32)) -> f32 {
    match part {
        Part::Minimize => to_segment(p, (8.0, 14.5), (16.0, 14.5)),
        Part::Maximize if maximized => {

            let back = if p.0 < 9.5 || p.1 > 14.5 { f32::MAX } else { to_square(p, (13.25, 10.75), 3.75, 1.5) };
            to_square(p, (10.75, 13.25), 3.75, 1.5).min(back)
        }
        Part::Maximize => to_square(p, (12.0, 12.0), 4.25, 1.75),
        Part::Close => to_segment(p, (8.25, 8.25), (15.75, 15.75)).min(to_segment(p, (15.75, 8.25), (8.25, 15.75))),
        Part::Title => f32::MAX,
    }
}

fn paint(look: &Look, theme: &Theme, radius: i32) -> Vec<u8> {
    let s = look.scale.max(1) as f32;
    let colours = if look.active { theme.active } else { theme.inactive };
    let (pw, ph) = ((look.width * look.scale) as usize, (HEADER_BAR_HEIGHT * look.scale) as usize);
    let (w, b) = (look.width as f32, BORDER as f32);
    let r = (radius as f32).clamp(b, TITLE_HEIGHT as f32);
    let mut px = vec![Paint([0.0; 4]); pw * ph];

    for y in 0..ph {
        for x in 0..pw {
            let (lx, ly) = ((x as f32 + 0.5) / s, (y as f32 + 0.5) / s);
            let outer = 0.5 - rounded_top(lx, ly, w, r) * s;
            let inner = 0.5 - rounded_top(lx - b, ly - b, w - 2.0 * b, r - b) * s;
            px[y * pw + x] = Paint::of(colours.background, inner).over(Paint::of(colours.border, outer));
        }
    }

    let mut leftmost = w - b;
    for part in [Part::Minimize, Part::Maximize, Part::Close] {
        let left = button_x(look.width, part) as f32;
        leftmost = leftmost.min(left);
        let hovered = look.hover == Some(part);
        let colour = if hovered { theme.active.button } else { colours.button };
        let stroke = if look.active { 1.8 } else { 1.6 };
        let backing = if look.pressed == Some(part) { 0.26 } else if hovered { 0.18 } else { 0.09 };
        let (cx, cy) = (left + BUTTON as f32 / 2.0, b + TITLE_HEIGHT as f32 / 2.0);
        let x0 = (left * s) as usize;
        let y0 = (b * s) as usize;
        for y in y0..((b + BUTTON as f32) * s) as usize {
            for x in x0..(((left + BUTTON as f32) * s) as usize).min(pw) {
                let (lx, ly) = ((x as f32 + 0.5) / s, (y as f32 + 0.5) / s);
                let mut here = px[y * pw + x];
                if backing > 0.0 {
                    let d = in_square((lx, ly), (cx, cy), BUTTON as f32 / 2.0 - 2.0, theme.button_corner);
                    let mut c = colour;
                    c[3] *= backing;
                    here = Paint::of(c, 0.5 - d * s).over(here);
                }
                let d = glyph_distance(part, look.maximized, (lx - cx + 12.0, ly - cy + 12.0));
                here = Paint::of(colour, (stroke / 2.0 - d) * s + 0.5).over(here);
                px[y * pw + x] = here;
            }
        }
    }

    write_title(&mut px, pw, ph, look, colours.text, (b + 8.0) * s, (leftmost - 8.0) * s);

    to_bytes(&px)
}

fn write_title(px: &mut [Paint], pw: usize, ph: usize, look: &Look, colour: [f32; 4], left: f32, right: f32) {
    let Some(font) = face(if look.active { 700.0 } else { 400.0 }) else { return };
    let size = PxScale::from(FONT_PX * look.scale as f32);
    let room = right - left;
    let mut text: Vec<char> = look.title.trim().chars().collect();
    if text.is_empty() || room <= 0.0 {
        return;
    }
    if chars_width(&font, size, &text) > room {
        while !text.is_empty() && chars_width(&font, size, &text) + chars_width(&font, size, &['…']) > room {
            text.pop();
        }
        while text.last().is_some_and(|c| c.is_whitespace()) {
            text.pop();
        }
        text.push('…');
    }
    let tw = chars_width(&font, size, &text);
    let pen = ((pw as f32 - tw) / 2.0).min(right - tw).max(left);

    let middle = (BORDER as f32 + TITLE_HEIGHT as f32 / 2.0) * look.scale as f32;
    let baseline = (middle + 0.36 * size.y).round();
    draw_chars(px, pw, ph, &font, size, &text, colour, pen, baseline);
}

pub(crate) fn face(weight: f32) -> Option<FontRef<'static>> {
    let mut font = FontRef::try_from_slice(FONT).ok()?;
    font.set_variation(b"wght", weight);
    Some(font)
}

pub(crate) fn chars_width(font: &FontRef, size: PxScale, text: &[char]) -> f32 {
    let scaled = font.as_scaled(size);
    let mut total = 0.0;
    let mut last = None;
    for c in text {
        let id = scaled.glyph_id(*c);
        if let Some(prev) = last {
            total += scaled.kern(prev, id);
        }
        total += scaled.h_advance(id);
        last = Some(id);
    }
    total
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_chars(px: &mut [Paint], pw: usize, ph: usize, font: &FontRef, size: PxScale, text: &[char], colour: [f32; 4], x: f32, baseline: f32) {
    let scaled = font.as_scaled(size);
    let mut pen = x;
    let mut last = None;
    for &c in text {
        let id = scaled.glyph_id(c);
        if let Some(prev) = last {
            pen += scaled.kern(prev, id);
        }
        last = Some(id);
        let glyph = id.with_scale_and_position(size, ab_glyph::point(pen, baseline));
        pen += scaled.h_advance(id);
        let Some(drawn) = font.outline_glyph(glyph) else { continue };
        let bounds = drawn.px_bounds();
        drawn.draw(|gx, gy, on| {
            let x = bounds.min.x as i32 + gx as i32;
            let y = bounds.min.y as i32 + gy as i32;
            if x >= 0 && y >= 0 && (x as usize) < pw && (y as usize) < ph {
                let i = y as usize * pw + x as usize;
                px[i] = Paint::of(colour, on).over(px[i]);
            }
        });
    }
}

impl WindowElement {
    pub fn decoration_state(&self) -> RefMut<'_, WindowState> {
        self.user_data().insert_if_missing(|| {
            RefCell::new(WindowState {
                is_ssd: false,
                header_bar: HeaderBar::default(),
            })
        });

        self.user_data()
            .get::<RefCell<WindowState>>()
            .unwrap()
            .borrow_mut()
    }

    pub fn set_ssd(&self, ssd: bool) {
        self.decoration_state().is_ssd = ssd;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look(hover: Option<Part>) -> Look {
        Look { width: 300, scale: 1, title: "Terminal".into(), active: true, maximized: false, hover, pressed: None }
    }

    fn pixel(bytes: &[u8], width: i32, x: i32, y: i32) -> [u8; 4] {
        let i = ((y * width + x) * 4) as usize;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    }

    #[test]
    fn the_buttons_are_where_a_click_finds_them() {
        let w = 300;
        let y = (BORDER + TITLE_HEIGHT / 2) as f64;
        assert_eq!(part_at(w, (w as f64 - 10.0, y).into()), Some(Part::Close));
        assert_eq!(part_at(w, ((button_x(w, Part::Maximize) + 1) as f64, y).into()), Some(Part::Maximize));
        assert_eq!(part_at(w, ((button_x(w, Part::Minimize) + 31) as f64, y).into()), Some(Part::Minimize));
        assert_eq!(part_at(w, (100.0, y).into()), Some(Part::Title));

        assert_eq!(part_at(w, (100.0, 0.5).into()), None);
        assert_eq!(part_at(w, (0.5, y).into()), None);
    }

    #[test]
    fn the_borders_flat_pieces_are_laid_over_not_added() {
        let mut borders = vec![Theme::of(false).active.border, Theme::of(false).inactive.border, Theme::of(true).active.border, Theme::of(true).inactive.border,
                               Theme::of(false).edge, Theme::of(true).edge];
        borders.extend([[0.745, 0.843, 1.0, 0.09], [0.745, 0.843, 1.0, 0.06], [1.0, 1.0, 1.0, 0.55], [0.0, 0.0, 0.0, 0.16], [1.0, 1.0, 1.0, 0.02]]);
        for colour in borders {
            let mut bar = HeaderBar::default();
            bar.update_sides((300, 200).into(), colour);
            for side in &bar.sides {
                let c = side.color().components();
                assert_eq!(c[3], colour[3], "{colour:?}: alpha changed");
                for i in 0..3 {
                    assert!(c[i] <= c[3] + 1e-6, "{colour:?}: channel {i} is {} with alpha {}, so it is added rather than laid over", c[i], c[3]);

                    let (over_black, over_white) = (c[i], c[i] + (1.0 - c[3]));
                    assert!((over_black - colour[i] * colour[3]).abs() < 1e-5 && over_white <= 1.0 + 1e-6);
                }
            }
        }
    }

    #[test]
    fn corners_are_round_and_the_middle_is_the_title_colour() {
        let theme = Theme::default();
        let bytes = paint(&look(None), &theme, 10);
        assert_eq!(bytes.len(), (300 * HEADER_BAR_HEIGHT * 4) as usize);

        assert_eq!(pixel(&bytes, 300, 0, 0)[3], 0);

        let bg = theme.active.background.map(|v| (v * 255.0).round() as u8);
        assert_eq!(pixel(&bytes, 300, 20, HEADER_BAR_HEIGHT - 1), [bg[2], bg[1], bg[0], 255]);
    }

    #[test]
    fn a_hovered_button_gets_a_backing() {
        let theme = Theme::default();
        let x = button_x(300, Part::Close) + 2;
        let y = BORDER + TITLE_HEIGHT / 2;
        let plain = paint(&look(None), &theme, 10);
        let hovered = paint(&look(Some(Part::Close)), &theme, 10);
        assert_ne!(pixel(&plain, 300, x, y), pixel(&hovered, 300, x, y));
    }

    #[test]
    fn every_button_sits_on_a_backing() {
        let theme = Theme::default();
        let bytes = paint(&look(None), &theme, 10);
        let bg = theme.active.background.map(|v| (v * 255.0).round() as u8);
        for part in [Part::Minimize, Part::Maximize, Part::Close] {

            let (x, y) = (button_x(300, part) + 6, BORDER + TITLE_HEIGHT / 2);
            assert_ne!(pixel(&bytes, 300, x, y), [bg[2], bg[1], bg[0], 255], "{part:?}");
        }
    }

    #[test]
    fn a_backings_corner_is_the_sheets() {
        let at = |corner: f32| {
            let theme = Theme { button_corner: corner, ..Theme::default() };
            let bytes = paint(&look(None), &theme, 10);
            pixel(&bytes, 300, button_x(300, Part::Close) + 3, BORDER + 3)
        };
        assert_ne!(at(4.0), at(999.0));
        assert_eq!(at(999.0), at(14.0));
    }

    #[test]
    fn the_shapes_are_a_line_a_square_and_a_cross() {

        assert!(super::glyph_distance(Part::Minimize, false, (12.0, 14.5)) < 0.1);
        assert!(super::glyph_distance(Part::Maximize, false, (7.75, 12.0)) < 0.1);
        assert!(super::glyph_distance(Part::Maximize, false, (12.0, 12.0)) > 3.0);
        assert!(super::glyph_distance(Part::Close, false, (12.0, 12.0)) < 0.1);
        assert!(super::glyph_distance(Part::Close, false, (12.0, 8.25)) > 2.0);
    }

    #[test]
    fn a_double_click_on_the_title_fills_the_screen() {
        let mut bar = HeaderBar::default();
        bar.pointer_enter((100.0, 20.0).into());
        assert_eq!(bar.press(300, 1000), Some(Action::Move));
        assert_eq!(bar.press(300, 1200), Some(Action::ToggleMaximize));
        assert_eq!(bar.press(300, 5000), Some(Action::Move));

        bar.pointer_enter((295.0, 20.0).into());
        assert_eq!(bar.press(300, 6000), None);
        assert_eq!(bar.release(300), Some(Action::Close));
    }
}
