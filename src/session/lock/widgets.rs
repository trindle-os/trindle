//! The pieces the lock screen is drawn from: card, buttons, eye, name and ring.

use std::time::SystemTime;

use tiny_skia::PathBuilder;

use crate::toolkit::colour::Colour;

use super::layout::{BUTTON_H, CARD_W, PAD};
use super::pen::{GLYPH_LINE, circle, lines, rounded, Pen};
use super::reveal::Eye;

pub(super) fn draw_card(pen: &mut Pen, cx: f64, top: f64, bottom: f64) {
    let (x, h) = (cx - CARD_W / 2.0, bottom - top);
    let corner = f64::from(crate::toolkit::theme::sheet().corner_of(crate::toolkit::theme::Part::Card));
    let card = pen.t.panel;
    pen.fill(rounded(x, top, CARD_W, h, corner), card);
}

pub(super) fn card_head(pen: &mut Pen, text: &str, cx: f64, y: f64) {
    let left = cx - CARD_W / 2.0 + PAD;
    let r = BUTTON_H / 2.0;
    let (disc, ink) = (pen.t.bg, pen.t.fg);
    pen.fill(circle(left + r, y, r), disc);

    let (x, by) = (left + r, y + 2.0);
    pen.stroke(rounded(x - 5.5, by - 1.0, 11.0, 8.0, 2.0), ink, GLYPH_LINE, true);
    let mut p = PathBuilder::new();
    p.move_to((x - 3.5) as f32, (by - 1.0) as f32);
    p.line_to((x - 3.5) as f32, (by - 4.0) as f32);
    p.cubic_to((x - 3.5) as f32, (by - 9.5) as f32, (x + 3.5) as f32, (by - 9.5) as f32, (x + 3.5) as f32, (by - 4.0) as f32);
    p.line_to((x + 3.5) as f32, (by - 1.0) as f32);
    pen.stroke(p.finish(), ink, GLYPH_LINE, true);
    let l = pen.layout(text, 16.0, true);
    pen.text_at(&l, left + BUTTON_H + 12.0, y + 5.5, ink);
}

pub(super) fn draw_phone(pen: &mut Pen, (x, y, r): (f64, f64, f64), lit: bool) {
    let fill = if lit { pen.t.accent } else { pen.t.bg };
    pen.fill(circle(x, y, r), fill);
    let (w, h) = (10.0, 18.0);
    let ink = if lit { pen.t.on_accent } else { pen.t.fg };
    pen.stroke(rounded(x - w / 2.0, y - h / 2.0, w, h, 2.5), ink, GLYPH_LINE, true);
    pen.stroke(lines(&[&[(x - 2.0, y + h / 2.0 - 3.5), (x + 2.0, y + h / 2.0 - 3.5)]], false), ink, GLYPH_LINE, true);
}

pub(super) fn draw_eye(pen: &mut Pen, (x, y, r): (f64, f64, f64), eye: Eye, rest: Colour) {
    let fill = if eye.shown { pen.t.accent } else { rest };
    pen.fill(circle(x, y, r), fill);
    let ink = if eye.shown { pen.t.on_accent } else { pen.t.fg };
    let (a, b) = (r * 0.58, r * 0.62);
    let mut almond = PathBuilder::new();
    almond.move_to((x - a) as f32, y as f32);
    almond.quad_to(x as f32, (y - b) as f32, (x + a) as f32, y as f32);
    almond.quad_to(x as f32, (y + b) as f32, (x - a) as f32, y as f32);
    almond.close();
    pen.stroke(almond.finish(), ink, 1.4, true);
    pen.fill(circle(x, y, r * 0.17), ink);
    if eye.shown {
        pen.stroke(lines(&[&[(x - a * 0.8, y - b * 0.75), (x + a * 0.8, y + b * 0.75)]], false), ink, 1.4, true);
    }
    if eye.on {
        let accent = pen.t.accent;
        pen.stroke(circle(x, y, r + 2.0), accent, GLYPH_LINE, false);
    }
}

pub(super) fn draw_name(pen: &mut Pen, text: &str, x: f64, top: f64) {
    let l = pen.layout(text, 12.0, true);
    let (lw, lh) = (pen.width(&l) + 20.0, 24.0);
    let (lx, ly) = (x - lw / 2.0, top);
    let tip_r = f64::from(crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Tooltip, lh as f32, 1.0));
    let shadow = pen.t.shadow;
    for (grow, a) in [(4.0, 0.06), (2.5, 0.1), (1.0, 0.16)] {
        pen.fill(rounded(lx - grow, ly + 3.0 - grow, lw + 2.0 * grow, lh + 2.0 * grow, tip_r + grow),
                 crate::toolkit::theme::alpha(shadow, a));
    }
    let (fill, words) = (pen.t.raised, pen.t.fg);
    pen.fill(rounded(lx, ly, lw, lh, tip_r), fill);
    pen.text_at(&l, lx + 10.0, ly + lh / 2.0 + 4.0, words);
}

pub(super) fn draw_switch(pen: &mut Pen, (x, y, w, h): (f64, f64, f64, f64), text: &str) {
    let r = f64::from(crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Button, h as f32, 1.0));
    let fill = pen.t.bg;
    pen.fill(rounded(x, y, w, h, r), fill);
    let l = pen.layout(text, 14.0, true);
    let tw = pen.width(&l);

    let left = x + (w - (17.0 + 8.0 + tw)) / 2.0;

    let (kx, ky) = (left + 4.0, y + r);
    let ink = pen.t.fg;
    pen.stroke(circle(kx, ky, 4.0), ink, GLYPH_LINE, true);
    let shaft: &[&[(f64, f64)]] = &[
        &[(kx + 4.0, ky), (kx + 13.0, ky)],
        &[(kx + 10.0, ky), (kx + 10.0, ky + 3.5)],
        &[(kx + 13.0, ky), (kx + 13.0, ky + 3.5)],
    ];
    pen.stroke(lines(shaft, false), ink, GLYPH_LINE, true);
    pen.text_at(&l, left + 17.0 + 8.0, y + r + 5.0, ink);
}

pub(super) fn draw_ring(pen: &mut Pen, x: f64, y: f64, r: f64) {
    let (track, accent) = (pen.t.bg, pen.t.accent);
    let width = (r * 0.44) as f32;
    pen.stroke(circle(x, y, r), track, width, false);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let from = (now.as_millis() % 1200) as f32 / 1200.0;
    let mut p = PathBuilder::new();
    for i in 0..=24 {
        let a = std::f32::consts::TAU * (from + 0.3 * i as f32 / 24.0) - std::f32::consts::FRAC_PI_2;
        let (px, py) = (x as f32 + r as f32 * a.cos(), y as f32 + r as f32 * a.sin());
        if i == 0 {
            p.move_to(px, py);
        } else {
            p.line_to(px, py);
        }
    }
    pen.stroke(p.finish(), accent, width, true);
}
