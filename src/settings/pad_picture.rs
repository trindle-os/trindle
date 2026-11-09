//! A game controller drawn flat, for the Games page: each button lights as it is pressed, the sticks lean the way they are pushed, the triggers fill as they are pulled, the button being changed has a ring, and a button with a change has a small dot.

use tiny_skia::{PathBuilder, PixmapMut, Stroke, Transform};

use crate::common::pads::{Family, From, Live};
use crate::toolkit::colour::{paint as ink, Colour};
use crate::toolkit::text::Style;
use crate::toolkit::theme::alpha;
use crate::toolkit::widget::Pen;

pub const WIDE: f32 = 424.0;
pub const HIGH: f32 = 356.0;
const FROM: (f32, f32) = (88.0, 6.0);

const DPAD: [(u16, f32, f32); 4] = [(0x220, 0.0, -1.0), (0x221, 0.0, 1.0), (0x222, -1.0, 0.0), (0x223, 1.0, 0.0)];

#[derive(Clone, Debug, Default)]
pub struct Shot {
    pub family: Option<Family>,
    pub down: Vec<u16>,
    pub sticks: [(f32, f32); 2],
    pub pulls: [f32; 2],
    pub picked: Option<From>,
    pub changed: Vec<From>,

    pub keys: Vec<(From, String)>,
}

impl Shot {
    pub fn of(live: &Live, family: Family, picked: Option<From>, changed: Vec<From>) -> Shot {
        let down = (0x130..=0x13e).chain(0x220..=0x223).filter(|b| live.is_down(*b)).collect();
        Shot {
            family: Some(family),
            down,
            sticks: [(live.moved(0), live.moved(1)), (live.moved(3), live.moved(4))],
            pulls: [live.pulled(2), live.pulled(5)],
            picked,
            changed,
            keys: Vec::new(),
        }
    }

    fn lit(&self, b: u16) -> bool {
        self.down.contains(&b)
    }

    fn marked(&self, from: From) -> bool {
        self.picked == Some(from)
    }

    fn printed(&self, from: From) -> Option<String> {
        let said = &self.keys.iter().find(|(f, _)| *f == from)?.1;
        Some(if said.chars().count() <= 5 { said.clone() } else { format!("{}.", said.chars().take(3).collect::<String>()) })
    }

    fn has_change(&self, from: From) -> bool {
        self.changed.contains(&from)
    }
}

struct Plan {
    left_stick: (f32, f32),
    right_stick: (f32, f32),
    dpad: (f32, f32),
    faces: (f32, f32),
}

fn plan(family: Family) -> Plan {
    match family {
        Family::PlayStation => Plan { left_stick: (252.0, 236.0), right_stick: (348.0, 236.0), dpad: (196.0, 150.0), faces: (404.0, 150.0) },
        _ => Plan { left_stick: (196.0, 150.0), right_stick: (348.0, 236.0), dpad: (252.0, 236.0), faces: (404.0, 150.0) },
    }
}

struct Ctx<'a, 'b> {
    canvas: &'a mut PixmapMut<'b>,
    pen: &'a Pen<'a>,
    shot: &'a Shot,
    k: f32,
    ox: f32,
    oy: f32,
}

impl Ctx<'_, '_> {
    fn x(&self, x: f32) -> f32 {
        self.ox + x * self.k
    }

    fn y(&self, y: f32) -> f32 {
        self.oy + y * self.k
    }

    fn disc(&mut self, x: f32, y: f32, r: f32, c: Colour) {
        if let Some(p) = PathBuilder::from_circle(self.x(x), self.y(y), r * self.k) {
            self.canvas.fill_path(&p, &ink(c), tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn ring(&mut self, x: f32, y: f32, r: f32, width: f32, c: Colour) {
        if let Some(p) = PathBuilder::from_circle(self.x(x), self.y(y), r * self.k) {
            let stroke = Stroke { width: width * self.k, ..Stroke::default() };
            self.canvas.stroke_path(&p, &ink(c), &stroke, Transform::identity(), None);
        }
    }

    fn capsule(&mut self, a: (f32, f32), b: (f32, f32), width: f32, c: Colour) {
        let mut pb = PathBuilder::new();
        pb.move_to(self.x(a.0), self.y(a.1));
        pb.line_to(self.x(b.0), self.y(b.1));
        if let Some(p) = pb.finish() {
            let stroke = Stroke { width: width * self.k, line_cap: tiny_skia::LineCap::Round, ..Stroke::default() };
            self.canvas.stroke_path(&p, &ink(c), &stroke, Transform::identity(), None);
        }
    }

    fn round_rect(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, c: Colour) {
        let (x0, y0, x1, y1, r) = (self.x(x), self.y(y), self.x(x + w), self.y(y + h), r * self.k);
        let mut pb = PathBuilder::new();
        pb.move_to(x0 + r, y0);
        pb.line_to(x1 - r, y0);
        pb.quad_to(x1, y0, x1, y0 + r);
        pb.line_to(x1, y1 - r);
        pb.quad_to(x1, y1, x1 - r, y1);
        pb.line_to(x0 + r, y1);
        pb.quad_to(x0, y1, x0, y1 - r);
        pb.line_to(x0, y0 + r);
        pb.quad_to(x0, y0, x0 + r, y0);
        pb.close();
        if let Some(p) = pb.finish() {
            self.canvas.fill_path(&p, &ink(c), tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn words(&mut self, text: &str, size: f32, x: f32, y: f32, c: Colour) {
        let layout = self.pen.fonts.layout(text, &Style { weight: 700, ..Style::new((size * self.k).round().max(8.0)) }, None);
        self.pen.fonts.draw(self.canvas, &layout, (self.x(x) - layout.width / 2.0).round(), (self.y(y) - layout.height / 2.0).round(), c);
    }

    fn tag(&mut self, from: From, x: f32, y: f32, r: f32) {
        let t = self.pen.theme;
        if self.shot.marked(from) {
            self.disc(x, y, r + 11.0, alpha(t.accent, 0.22));
            self.ring(x, y, r + 5.0, 3.0, t.accent);
        }
    }

    fn dot(&mut self, from: From, x: f32, y: f32) {
        if self.shot.has_change(from) {
            let t = self.pen.theme;
            self.disc(x, y, 6.5, t.panel);
            self.disc(x, y, 4.5, t.accent);
        }
    }

    fn glyph(&mut self, b: u16, x: f32, y: f32, c: Colour) {
        let (cx, cy, s) = (self.x(x), self.y(y), 7.0 * self.k);
        let stroke = Stroke { width: 2.2 * self.k, line_cap: tiny_skia::LineCap::Round, line_join: tiny_skia::LineJoin::Round, ..Stroke::default() };
        let mut pb = PathBuilder::new();
        let path = match b {
            0x130 => {
                pb.move_to(cx - s, cy - s);
                pb.line_to(cx + s, cy + s);
                pb.move_to(cx + s, cy - s);
                pb.line_to(cx - s, cy + s);
                pb.finish()
            }
            0x131 => PathBuilder::from_circle(cx, cy, s * 1.05),
            0x133 => {
                pb.move_to(cx, cy - s * 1.1);
                pb.line_to(cx + s * 1.1, cy + s * 0.85);
                pb.line_to(cx - s * 1.1, cy + s * 0.85);
                pb.close();
                pb.finish()
            }
            _ => {
                pb.move_to(cx - s * 0.9, cy - s * 0.9);
                pb.line_to(cx + s * 0.9, cy - s * 0.9);
                pb.line_to(cx + s * 0.9, cy + s * 0.9);
                pb.line_to(cx - s * 0.9, cy + s * 0.9);
                pb.close();
                pb.finish()
            }
        };
        if let Some(p) = path {
            self.canvas.stroke_path(&p, &ink(c), &stroke, Transform::identity(), None);
        }
    }

    fn button(&mut self, from: From, b: u16, x: f32, y: f32, r: f32, name: &str) {
        let t = self.pen.theme;
        let on = self.shot.lit(b);
        self.tag(from, x, y, r);
        self.disc(x, y, r, if on { t.accent } else { t.control });
        self.ring(x, y, r - 0.5, 1.5, if on { t.accent } else { t.control_edge });
        self.words(name, r * 0.95, x, y, if on { t.on_accent } else { t.fg });
        self.dot(from, x + r * 0.8, y - r * 0.8);
    }

    fn pill(&mut self, from: From, b: u16, x: f32, y: f32, w: f32, h: f32, name: &str, size: f32) {
        let t = self.pen.theme;
        let on = self.shot.lit(b);
        if self.shot.marked(from) {
            self.round_rect(x - w / 2.0 - 7.0, y - h / 2.0 - 7.0, w + 14.0, h + 14.0, h / 2.0 + 7.0, alpha(t.accent, 0.22));
        }
        self.round_rect(x - w / 2.0 - 1.5, y - h / 2.0 - 1.5, w + 3.0, h + 3.0, h / 2.0 + 1.5, t.edge);
        self.round_rect(x - w / 2.0, y - h / 2.0, w, h, h / 2.0, if on { t.accent } else { t.control });
        if !name.is_empty() {
            self.words(name, size, x, y, if on { t.on_accent } else { t.muted });
        }
        self.dot(from, x + w / 2.0 - 2.0, y - h / 2.0 + 1.0);
    }

    fn stick(&mut self, which: usize, at: (f32, f32), press: u16, from: From) {
        let t = self.pen.theme;
        let on = self.shot.lit(press);
        let (mx, my) = self.shot.sticks[which];
        self.disc(at.0, at.1, 36.0, t.well);
        self.ring(at.0, at.1, 36.0, 2.0, t.control_edge);
        let (kx, ky) = (at.0 + mx * 15.0, at.1 + my * 15.0);
        if self.shot.marked(from) {
            self.disc(kx, ky, 33.0, alpha(t.accent, 0.22));
            self.ring(kx, ky, 28.0, 3.0, t.accent);
        }
        self.disc(kx, ky, 23.0, if on { t.accent } else { t.control });
        self.ring(kx, ky, 22.5, 1.5, if on { t.accent } else { t.control_edge });
        self.disc(kx, ky, 11.0, if on { t.on_accent } else { t.well });

        let (across, down) = if which == 0 { (0, 1) } else { (3, 4) };
        for (axis, positive, dx, dy) in [(across, false, -1.0, 0.0), (across, true, 1.0, 0.0), (down, false, 0.0, -1.0), (down, true, 0.0, 1.0)] {
            if let Some(key) = self.shot.printed(From::Direction(axis, positive)) {
                self.words(&key, 10.0, at.0 + dx * 29.5, at.1 + dy * 29.5, t.fg);
            }
        }
        self.dot(from, at.0 + 30.0, at.1 - 30.0);
    }

    fn dpad(&mut self, at: (f32, f32)) {
        let t = self.pen.theme;
        self.disc(at.0, at.1, 38.0, t.well);
        for (b, dx, dy) in DPAD {
            let (cx, cy) = (at.0 + dx * 21.0, at.1 + dy * 21.0);
            let (w, h) = if dx == 0.0 { (24.0, 26.0) } else { (26.0, 24.0) };
            let on = self.shot.lit(b);
            let from = From::Button(b);
            if self.shot.marked(from) {
                self.round_rect(cx - w / 2.0 - 6.0, cy - h / 2.0 - 6.0, w + 12.0, h + 12.0, 10.0, alpha(t.accent, 0.22));
            }
            self.round_rect(cx - w / 2.0, cy - h / 2.0, w, h, 6.0, if on { t.accent } else { t.control });
            if let Some(key) = self.shot.printed(from) {
                self.words(&key, 10.0, cx, cy, if on { t.on_accent } else { t.fg });
            }
            self.dot(from, cx + dx * 9.0 + 8.0, cy + dy * 9.0 - 8.0);
        }
        self.disc(at.0, at.1, 7.0, t.control);
    }

    fn trigger(&mut self, which: usize, x: f32, from: From) {
        let t = self.pen.theme;
        let pull = self.shot.pulls[which];
        let (w, h, y) = (80.0, 34.0, 12.0);
        if self.shot.marked(from) {
            self.round_rect(x - w / 2.0 - 6.0, y - 6.0, w + 12.0, h + 12.0, 14.0, alpha(t.accent, 0.22));
        }
        self.round_rect(x - w / 2.0 - 1.5, y - 1.5, w + 3.0, h + 3.0, 13.5, t.edge);
        self.round_rect(x - w / 2.0, y, w, h, 12.0, t.control);
        if pull > 0.02 {
            let fill = h * pull;
            self.round_rect(x - w / 2.0, y + h - fill, w, fill.max(8.0), 6.0, t.accent);
        }
        if let Some(name) = self.pen_family_label(from) {
            self.words(name, 12.0, x, y + h / 2.0, if pull > 0.55 { t.on_accent } else { t.muted });
        }
        self.dot(from, x + w / 2.0 - 4.0, y + 4.0);
    }

    fn pen_family_label(&self, from: From) -> Option<&'static str> {
        self.shot.family.unwrap_or(Family::Xbox).label(from)
    }
}

pub fn paint(canvas: &mut PixmapMut, pen: &Pen, shot: &Shot) {
    let family = shot.family.unwrap_or(Family::Xbox);
    let t = pen.theme;
    let (w, h) = (canvas.width() as f32, canvas.height() as f32);
    let k = (w / WIDE).min(h / HIGH);
    let (ox, oy) = ((w - WIDE * k) / 2.0 - FROM.0 * k, (h - HIGH * k) / 2.0 - FROM.1 * k);
    let mut c = Ctx { canvas, pen, shot, k, ox, oy };
    let p = plan(family);
    let label = |from: From, fallback: &'static str| family.label(from).unwrap_or(fallback);

    for (grow, colour) in [(6.0, t.edge), (0.0, t.raised)] {
        c.capsule((205.0, 150.0), (395.0, 150.0), 146.0 + grow, colour);
        c.capsule((196.0, 216.0), (150.0, 290.0), 106.0 + grow, colour);
        c.capsule((404.0, 216.0), (450.0, 290.0), 106.0 + grow, colour);
    }

    c.trigger(0, 190.0, From::Trigger(2));
    c.trigger(1, 410.0, From::Trigger(5));
    for (b, x) in [(0x136u16, 190.0), (0x137, 410.0)] {
        let from = From::Button(b);
        c.pill(from, b, x, 62.0, 112.0, 22.0, label(from, ""), 12.0);
    }

    c.dpad(p.dpad);
    c.stick(0, p.left_stick, 0x13d, From::Button(0x13d));
    c.stick(1, p.right_stick, 0x13e, From::Button(0x13e));

    let (fx, fy) = p.faces;
    for (b, dx, dy) in [(0x130u16, 0.0, 1.0), (0x131, 1.0, 0.0), (0x133, -1.0, 0.0), (0x134, 0.0, -1.0)] {
        let from = From::Button(b);
        let (x, y) = (fx + dx * 31.0, fy + dy * 31.0);
        if family == Family::PlayStation {
            c.button(from, b, x, y, 17.0, "");
            let ink = if shot.lit(b) { t.on_accent } else { t.fg };
            c.glyph(b, x, y, ink);
        } else {
            c.button(from, b, x, y, 17.0, label(from, ""));
        }
    }

    let (select, start, home) = (From::Button(0x13a), From::Button(0x13b), From::Button(0x13c));
    c.pill(select, 0x13a, 268.0, 120.0, 36.0, 16.0, "", 10.0);
    c.pill(start, 0x13b, 332.0, 120.0, 36.0, 16.0, "", 10.0);
    c.button(home, 0x13c, 300.0, 168.0, 16.0, "");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::icon::Icons;
    use crate::toolkit::text::Fonts;
    use crate::toolkit::theme::Theme;

    fn render(light: bool, family: Family, shot: Shot) -> tiny_skia::Pixmap {
        let (fonts, icons) = (Fonts::new(), Icons::new());
        let theme = Theme::new(light, "#fbc711");
        let pen = Pen { theme: &theme, fonts: &fonts, icons: &icons, scale: 1.0 };
        let mut pix = tiny_skia::Pixmap::new(900, 560).unwrap();
        pix.fill(theme.panel.to_skia());
        paint(&mut pix.as_mut(), &pen, &Shot { family: Some(family), ..shot });
        pix
    }

    #[test]
    fn a_lit_button_is_painted_in_the_highlight_and_an_unlit_one_is_not() {
        let at = |pix: &tiny_skia::Pixmap, x: f32, y: f32| {
            let k = (900.0f32 / WIDE).min(560.0 / HIGH);
            let (ox, oy) = ((900.0 - WIDE * k) / 2.0 - FROM.0 * k, (560.0 - HIGH * k) / 2.0 - FROM.1 * k);
            pix.pixel((ox + x * k) as u32, (oy + y * k) as u32 - 2).unwrap()
        };
        let still = render(false, Family::Xbox, Shot::default());
        let pressed = render(false, Family::Xbox, Shot { down: vec![0x131], ..Shot::default() });
        assert_ne!(at(&still, 435.0 + 14.0, 150.0), at(&pressed, 435.0 + 14.0, 150.0));
        assert_eq!(at(&still, 300.0 - 9.0, 168.0), at(&pressed, 300.0 - 9.0, 168.0));
    }

    #[test]
    fn the_keys_a_stick_and_the_dpad_become_are_printed_on_them() {
        let bare = render(false, Family::Xbox, Shot::default());
        let keys = vec![(From::Direction(0, false), "A".to_string()), (From::Direction(1, false), "W".to_string()), (From::Button(0x220), "Up".to_string())];
        let printed = render(false, Family::Xbox, Shot { keys, ..Shot::default() });
        assert_ne!(bare.data(), printed.data(), "something was printed");
        let letter = Shot { keys: vec![(From::Direction(0, false), "Backspace".to_string())], ..Shot::default() };
        assert_eq!(letter.printed(From::Direction(0, false)).as_deref(), Some("Bac."), "a long key is cut to fit");
        assert_eq!(letter.printed(From::Direction(0, true)), None);
    }
}
