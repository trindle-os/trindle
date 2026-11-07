//! Drawing the whole lock screen for its state, and putting it on each screen.

use std::time::Instant;

use tiny_skia::Pixmap;
use wayland_client::QueueHandle;
use wayland_client::protocol::{wl_buffer, wl_surface};

use crate::common::{pin, pocketkey, shm};
use crate::toolkit::t;
use crate::toolkit::widget::Secret;

use super::{Mode, State};
use super::input::PHONE_KEY;
use super::layout::{card_left, eye_button, keypad, password_box, phone_button, switch_button, typing_room, BUTTON_H, DOT_GAP, KEY_R, PAD, PIN_ROW};
use super::pen::{GLYPH_LINE, circle, lines, look, rounded, Pen};
use super::rest::{clock_lines, rest_frame};
use super::reveal::Eye;
use super::widgets::{card_head, draw_card, draw_eye, draw_name, draw_phone, draw_ring, draw_switch};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Phone {
    Absent,
    Here { hint: bool },
}

pub(super) fn shown_message(message: &str, due: Option<u32>, checking: bool) -> String {
    match due {
        Some(secs) if message.is_empty() && !checking => pocketkey::due_words(secs),
        _ => message.to_string(),
    }
}

fn message_line(mode: Mode, cy: f64) -> f64 {
    match mode {
        Mode::Pin => cy - 72.0,
        Mode::Password => cy - 14.0,
    }
}

pub(super) struct View<'a> {
    pub(super) mode: Mode,
    pub(super) typed: &'a str,
    pub(super) eye: Eye,
    pub(super) eye_named: bool,
    pub(super) message: &'a str,
    pub(super) wrong: bool,
    pub(super) checking: bool,
    pub(super) pin_possible: bool,
    pub(super) flash: Option<char>,
    pub(super) phone: Phone,
}

const CARD_ABOVE: f64 = 182.0;

pub(super) fn draw(pen: &mut Pen, w: f64, h: f64, view: &View) {
    let (cx, cy) = (w / 2.0, h / 2.0);
    draw_time(pen, cx, cy);
    match view.mode {
        Mode::Pin => draw_pin(pen, view, cx, cy),
        Mode::Password => draw_password(pen, view, cx, cy),
    }
    if let Phone::Here { hint } = view.phone {
        let (x, y, r) = phone_button(view.mode, cx, cy, view.pin_possible);
        draw_phone(pen, (x, y, r), view.flash == Some(PHONE_KEY));
        if hint {
            draw_name(pen, t!("session-lock-use-phone"), x, y + r + 6.0);
        }
    }
    draw_note(pen, view, cx, cy);
    if view.eye_named {
        let (x, y, r) = eye_button(view.mode, cx, cy);
        let what = if view.mode == Mode::Pin { Secret::Pin } else { Secret::Password };
        draw_name(pen, if view.eye.shown { what.hide() } else { what.show() }, x, y + r + 8.0);
    }
}

fn draw_time(pen: &mut Pen, cx: f64, cy: f64) {
    let (page, words, quiet) = (pen.t.bg, pen.t.fg, pen.t.muted);
    pen.px.fill(page.to_skia());
    let now = crate::common::clock::Local::now();
    let (time, date) = clock_lines(&now);
    pen.centred(&time, cx, cy - 250.0, 72.0, false, words);
    pen.centred(&date, cx, cy - 212.0, 18.0, false, quiet);
}

fn draw_pin(pen: &mut Pen, view: &View, cx: f64, cy: f64) {
    let (page, accent) = (pen.t.bg, pen.t.accent);
    let has_phone = view.phone != Phone::Absent;
    let top = cy - CARD_ABOVE;
    let (_, sy, _, sh) = switch_button(Mode::Pin, cx, cy, has_phone);
    draw_card(pen, cx, top, sy + sh + PAD);
    card_head(pen, t!("session-lock-enter-pin"), cx, top + PAD + BUTTON_H / 2.0);
    let entered = view.typed.chars().count();
    for i in 0..pin::LENGTH {
        let x = cx + (i as f64 - 1.5) * 24.0;
        match view.typed.chars().nth(i).filter(|_| view.eye.shown) {
            Some(digit) => pen.centred(&digit.to_string(), x, cy + PIN_ROW + 6.0, 17.0, true, pen.t.accent_text),
            None => pen.fill(circle(x, cy + PIN_ROW, 6.0), if i < entered { accent } else { page }),
        }
    }
    draw_eye(pen, eye_button(Mode::Pin, cx, cy), view.eye, page);
    draw_keypad(pen, view.flash, cx, cy);
    draw_switch(pen, switch_button(Mode::Pin, cx, cy, has_phone), t!("session-lock-use-password"));
}

fn draw_keypad(pen: &mut Pen, flash: Option<char>, cx: f64, cy: f64) {
    let (page, words, accent, on_accent) = (pen.t.bg, pen.t.fg, pen.t.accent, pen.t.on_accent);
    for (k, x, y) in keypad(cx, cy) {
        let lit = flash == Some(k);
        pen.fill(circle(x, y, KEY_R), if lit { accent } else { page });
        let ink = if lit { on_accent } else { words };
        match k {
            '<' => {
                let erase: &[&[(f64, f64)]] = &[
                    &[(x - 10.0, y), (x - 4.0, y - 7.0), (x + 9.0, y - 7.0), (x + 9.0, y + 7.0), (x - 4.0, y + 7.0)],
                    &[(x - 1.0, y - 3.5), (x + 5.5, y + 3.5)],
                    &[(x + 5.5, y - 3.5), (x - 1.0, y + 3.5)],
                ];
                pen.stroke(lines(erase, true), ink, GLYPH_LINE, true);
            }
            _ => pen.centred(&k.to_string(), x, y + 8.0, 22.0, false, ink),
        }
    }
}

fn draw_password(pen: &mut Pen, view: &View, cx: f64, cy: f64) {
    let (quiet, accent, typing) = (pen.t.muted, pen.t.accent, pen.t.field);
    let has_phone = view.phone != Phone::Absent;
    let top = cy - CARD_ABOVE;
    let bottom = if has_phone { 96.0 } else if view.pin_possible { 70.0 } else { 24.0 };
    draw_card(pen, cx, top, cy + bottom);
    card_head(pen, t!("session-lock-locked"), cx, top + PAD + BUTTON_H / 2.0);
    let hint = pen.layout(t!("session-lock-type-password"), 14.0, false);
    pen.text_at(&hint, card_left(cx), cy - 100.0, quiet);
    let (bx, by, bw, bh) = password_box(cx, cy);
    pen.edged(rounded(bx, by, bw, bh, f64::from(crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Box, bh as f32, 1.0))), typing, accent, GLYPH_LINE);
    draw_typed(pen, view, cx, cy);
    let rest = pen.t.control;
    draw_eye(pen, eye_button(Mode::Password, cx, cy), view.eye, rest);
    if view.pin_possible {
        draw_switch(pen, switch_button(Mode::Password, cx, cy, has_phone), t!("session-lock-use-pin"));
    }
}

fn draw_typed(pen: &mut Pen, view: &View, cx: f64, cy: f64) {
    let words = pen.t.fg;
    let (_, by, _, bh) = password_box(cx, cy);
    let (mid, room) = typing_room(cx, cy);
    if view.eye.shown {
        let typed = view.typed;
        let mut from = 0;
        let mut l = pen.layout(typed, 16.0, false);
        while pen.width(&l) > room && from < typed.len() {
            from += typed[from..].chars().next().map_or(1, char::len_utf8);
            l = pen.layout(&typed[from..], 16.0, false);
        }
        let lw = pen.width(&l);
        pen.text_at(&l, mid - lw / 2.0, by + bh / 2.0 + 5.5, words);
    } else {
        let shown = view.typed.chars().count().min((room / DOT_GAP) as usize);
        for i in 0..shown {
            let x = mid + (i as f64 - (shown as f64 - 1.0) / 2.0) * DOT_GAP;
            pen.fill(circle(x, by + bh / 2.0, 4.0), words);
        }
    }
}

fn draw_note(pen: &mut Pen, view: &View, cx: f64, cy: f64) {
    let (quiet, bad) = (pen.t.muted, pen.t.danger_text);
    let left = card_left(cx);
    let note = if view.checking && view.message.is_empty() { t!("session-lock-checking") } else { view.message };
    let y = message_line(view.mode, cy);
    if view.checking {
        let l = pen.layout(note, 14.0, false);
        draw_ring(pen, left + 8.0, y - 4.0, 8.0);
        pen.text_at(&l, left + 24.0, y, quiet);
    } else if !note.is_empty() {
        let l = pen.layout(note, 14.0, false);
        pen.text_at(&l, left, y, if view.wrong { bad } else { quiet });
    }
}

pub(super) fn redraw(state: &mut State, qh: &QueueHandle<State>) {
    state.moved = Instant::now();
    if let Some(since) = state.resting {
        return rest_frame(state, qh, since.elapsed().as_secs_f64());
    }
    for screen in &mut state.screens {
        state.old.append(&mut screen.kept);
    }
    let pin_possible = state.pin_allowed();
    let phone = if state.phone { Phone::Here { hint: state.hint_showing() } } else { Phone::Absent };
    let message = shown_message(&state.message, state.due, state.checking);
    let theme = look();
    for i in 0..state.screens.len() {
        let (w, h, scale) = (state.screens[i].size.0, state.screens[i].size.1, state.screens[i].scale.max(1));
        let Some(surface) = state.screens[i].surface.clone() else { continue };
        if w <= 0 || h <= 0 {
            continue;
        }
        let (pw, ph) = (w * scale, h * scale);
        let Some(mut img) = Pixmap::new(pw as u32, ph as u32) else { continue };
        {
            let mut pen = Pen { px: img.as_mut(), scale: scale as f32, fonts: &state.fonts, t: &theme };
            let view = View {
                mode: state.mode, typed: &state.entered, eye: state.eye, eye_named: state.eye.name_showing(Instant::now()),
                message: &message, wrong: state.wrong, checking: state.checking, pin_possible, flash: state.flash.map(|(k, _)| k), phone,
            };
            draw(&mut pen, w as f64, h as f64, &view);
        }
        let Some(buffer) = shm::pixmap(&state.shm, qh, &img, c"trindle-lock") else { continue };
        surface.set_buffer_scale(scale);
        surface.attach(Some(&buffer), 0, 0);
        surface.damage_buffer(0, 0, pw, ph);
        surface.commit();
    }
}

pub(super) fn make_cursor(state: &State, qh: &QueueHandle<State>) -> Option<(wl_surface::WlSurface, wl_buffer::WlBuffer)> {
    let mut img = Pixmap::new(24, 24)?;
    {
        let theme = look();
        let mut pen = Pen { px: img.as_mut(), scale: 1.0, fonts: &state.fonts, t: &theme };
        let arrow = || lines(&[&[(2.0, 2.0), (2.0, 19.0), (7.0, 14.5), (10.5, 21.0), (13.0, 20.0), (9.8, 13.5), (16.0, 13.5)]], true);
        pen.edged(arrow(), theme.fg, theme.bg, 1.2);
    }
    let buffer = shm::pixmap(&state.shm, qh, &img, c"trindle-lock")?;
    let surface = state.compositor.create_surface(qh, ());
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    Some((surface, buffer))
}
