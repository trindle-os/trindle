//! Where everything sits on the lock screen, and how big it is.

use super::Mode;

pub(super) fn keypad(cx: f64, cy: f64) -> Vec<(char, f64, f64)> {
    let keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '<', '0'];
    keys.iter().enumerate().map(|(i, k)| {
        let (col, row) = ((i % 3) as f64 - 1.0, (i / 3) as f64);
        (*k, cx + col * KEY_GAP, cy + KEY_TOP + row * KEY_GAP)
    }).collect()
}

pub(super) fn switch_button(mode: Mode, cx: f64, cy: f64, phone: bool) -> (f64, f64, f64, f64) {
    let (w, h) = (176.0, BUTTON_H);
    match mode {
        Mode::Pin => (cx - w / 2.0, cy + KEY_TOP + 3.0 * KEY_GAP + KEY_R + if phone { 40.0 } else { 16.0 }, w, h),
        Mode::Password if phone => (cx - (w + PHONE_GAP + 2.0 * PHONE_R) / 2.0, cy + 12.0, w, h),
        Mode::Password => (cx - w / 2.0, cy + 12.0, w, h),
    }
}

pub(super) fn phone_button(mode: Mode, cx: f64, cy: f64, pin_possible: bool) -> (f64, f64, f64) {
    match mode {
        Mode::Pin => (cx + KEY_GAP, cy + KEY_TOP + 3.0 * KEY_GAP, KEY_R),
        Mode::Password if pin_possible => {
            let (x, y, w, h) = switch_button(Mode::Password, cx, cy, true);
            (x + w + PHONE_GAP + PHONE_R, y + h / 2.0, PHONE_R)
        }
        Mode::Password => (cx, cy + 12.0 + BUTTON_H / 2.0, PHONE_R),
    }
}

pub(super) fn card_left(cx: f64) -> f64 {
    cx - CARD_W / 2.0 + PAD
}

pub(super) fn password_box(cx: f64, cy: f64) -> (f64, f64, f64, f64) {
    let (w, h) = (CARD_W - 2.0 * PAD, 44.0);
    (cx - w / 2.0, cy - 84.0, w, h)
}

pub(super) fn eye_button(mode: Mode, cx: f64, cy: f64) -> (f64, f64, f64) {
    let r = f64::from(crate::toolkit::theme::sheet().entry_eye) / 2.0;
    match mode {
        Mode::Pin => (cx + CARD_W / 2.0 - PAD - r, PIN_ROW + cy, r),
        Mode::Password => {
            let (x, y, w, h) = password_box(cx, cy);
            (x + w - h / 2.0, y + h / 2.0, r)
        }
    }
}

pub(super) const EYE_REACH: f64 = 22.0;

pub(super) const PIN_ROW: f64 = -102.0;

pub(super) const DOT_GAP: f64 = 13.0;

pub(super) fn typing_room(cx: f64, cy: f64) -> (f64, f64) {
    let (x, _, _, h) = password_box(cx, cy);
    let (ex, _, er) = eye_button(Mode::Password, cx, cy);
    let (left, right) = (x + h / 2.0, ex - er - 8.0);
    ((left + right) / 2.0, right - left)
}

const PHONE_R: f64 = KEY_R;
const PHONE_GAP: f64 = 12.0;

pub(super) const CARD_W: f64 = 392.0;
pub(super) const PAD: f64 = 24.0;

pub(super) const BUTTON_H: f64 = 34.0;

pub(super) const KEY_R: f64 = 22.0;

pub(super) const KEY_GAP: f64 = 56.0;

pub(super) const KEY_TOP: f64 = -36.0;
