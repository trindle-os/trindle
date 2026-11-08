//! Tests of the lock screen.

use std::time::{Duration, Instant};

use tiny_skia::Pixmap;

use crate::common::{eye, pocketkey};
use crate::toolkit::colour::Colour;
use crate::toolkit::text::Fonts;
use crate::toolkit::theme::Theme;

use super::Mode;
use super::input::{account_name, answer_counts, eye_holds};
use super::keyboard::Key;
use super::layout::{eye_button, keypad, password_box, phone_button, switch_button, typing_room, CARD_W, EYE_REACH, KEY_GAP, KEY_R, KEY_TOP, PAD, PIN_ROW};
use super::pen::Pen;
use super::picture::{draw, shown_message, Phone, View};
use super::rest::{clock_lines, clock_says, draw_resting};
use super::reveal::Eye;

#[test]
fn only_the_answer_to_the_check_now_running_counts() {
    assert!(answer_counts(2, 5, 5, true));
    assert!(!answer_counts(2, 4, 5, true), "the late answer to a check given up on unlocked or failed a newer one");
    assert!(!answer_counts(2, 5, 5, false), "an answer with no check running");
    assert!(!answer_counts(1, 5, 5, true), "a short read");
    assert!(!answer_counts(-1, 5, 5, true));
}

#[test]
fn the_account_is_the_user_id_s_not_the_environment_s() {
    let name = account_name();
    assert!(!name.is_empty());
    std::env::set_var("USER", "someone-else-entirely");
    assert_eq!(account_name(), name);
}

#[test]
fn the_resting_clock_is_written_in_the_persons_language() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

    let now = crate::common::clock::Local::utc(1_790_924_589);
    assert_eq!(clock_lines(&now), ("07:03".to_string(), "Friday 02 October".to_string()));
    assert_eq!(clock_says(&now), "07:03 Friday 02 October");
    let made_up = "kit-date-weekday-fri = Frodag\nkit-date-month-oct = Blumar\n\
                   kit-date-pattern-clock = %H.%M\nkit-date-pattern-day-month-padded = %A, %B %d\n";
    with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
        assert_eq!(clock_lines(&now), ("07.03".to_string(), "Frodag, Blumar 02".to_string()));
        assert_eq!(clock_says(&now), "07.03 Frodag, Blumar 02");
    });
}

#[test]
fn keypad_keys_do_not_touch() {
    let keys = keypad(500.0, 500.0);
    assert_eq!(keys.len(), 11);
    for (i, a) in keys.iter().enumerate() {
        for b in &keys[i + 1..] {
            assert!((a.1 - b.1).hypot(a.2 - b.2) > 2.0 * KEY_R);
        }
    }
}

fn clear_of((x, y, r): (f64, f64, f64), (bx, by, bw, bh): (f64, f64, f64, f64)) -> bool {
    x + r < bx || x - r > bx + bw || y + r < by || y - r > by + bh
}

#[test]
fn phone_button_touches_nothing() {
    let (cx, cy) = (500.0, 500.0);
    let phone = phone_button(Mode::Pin, cx, cy, true);
    for k in keypad(cx, cy) {
        assert!((phone.0 - k.1).hypot(phone.1 - k.2) > 2.0 * KEY_R, "on key {:?}", k.0);
    }

    let switch = switch_button(Mode::Pin, cx, cy, true);
    assert!(clear_of(phone, switch));
    assert!(phone.1 + phone.2 + 16.0 < switch.1);
    for pin in [true, false] {
        let phone = phone_button(Mode::Password, cx, cy, pin);

        if pin {
            assert!(clear_of(phone, switch_button(Mode::Password, cx, cy, true)));
        }

        assert!(phone.1 - phone.2 > cy - 40.0);
    }

    assert_eq!(switch_button(Mode::Pin, cx, cy, false).1, cy + KEY_TOP + 3.0 * KEY_GAP + KEY_R + 16.0);
    assert_eq!(switch_button(Mode::Password, cx, cy, false).0, cx - 88.0);
}

fn drawn(w: i32, h: i32, mode: Mode, entered: usize, message: &str, wrong: bool, checking: bool, flash: Option<char>,
         phone: Phone) -> Pixmap {
    let typed: String = "1234567890".chars().cycle().take(entered).collect();
    drawn_with(w, h, mode, &typed, Eye::default(), false, message, wrong, checking, flash, phone)
}

fn drawn_with(w: i32, h: i32, mode: Mode, typed: &str, eye: Eye, eye_named: bool, message: &str, wrong: bool, checking: bool,
              flash: Option<char>, phone: Phone) -> Pixmap {
    let fonts = Fonts::new();
    let theme = Theme::new(false, crate::common::style::DEFAULT_ACCENT);
    let mut img = Pixmap::new(w as u32, h as u32).unwrap();
    let mut pen = Pen { px: img.as_mut(), scale: 1.0, fonts: &fonts, t: &theme };
    let view = View { mode, typed, eye, eye_named, message, wrong, checking, pin_possible: true, flash, phone };
    draw(&mut pen, f64::from(w), f64::from(h), &view);
    img
}

#[test]
fn the_eye_is_reached_with_tab_and_pressed_with_space_or_enter() {
    let t = Instant::now();
    let mut eye = Eye::default();

    assert!(!eye.key(Key::Char(' '), t) && !eye.key(Key::Enter, t) && !eye.key(Key::Char('a'), t));
    assert!(eye.key(Key::Tab, t) && eye.on && eye.name_showing(t));
    assert!(eye.key(Key::Char(' '), t) && eye.shown && eye.on);
    assert!(eye.key(Key::Enter, t) && !eye.shown && eye.on);
    assert!(eye.key(Key::Enter, t) && eye.shown);

    assert!(!eye.key(Key::Char('a'), t) && !eye.on && eye.shown);
    assert!(eye.key(Key::Tab, t) && eye.on);
    assert!(!eye.key(Key::Erase, t) && !eye.on);

    assert!(eye.key(Key::Tab, t) && eye.on);
    assert!(eye.key(Key::Tab, t) && !eye.on && !eye.name_showing(t));

    eye.key(Key::Tab, t);
    assert!(!eye.key(Key::Back, t) && !eye.key(Key::Phone, t));
}

#[test]
fn a_shown_password_hides_after_each_try_and_after_half_a_minute_with_no_key() {
    let t = Instant::now();
    let mut eye = Eye::default();
    eye.press(t);
    assert!(eye.shown);
    assert_eq!(eye.wake(t + Duration::from_secs(10)), Some(Duration::from_secs(20)));
    assert!(!eye.rest(t + Duration::from_secs(29)) && eye.shown);

    assert!(!eye.key(Key::Char('a'), t + Duration::from_secs(29)));
    assert!(!eye.rest(t + Duration::from_secs(40)) && eye.shown);
    assert!(eye.rest(t + Duration::from_secs(29) + eye::REST) && !eye.shown);
    assert!(!eye.rest(t + Duration::from_secs(200)) && eye.wake(t).is_none());

    eye.key(Key::Tab, t);
    eye.press(t);
    eye.shut();
    assert_eq!(eye, Eye::default());

    eye.press(t);
    eye.press(t);
    assert!(!eye.shown);
}

#[test]
fn the_eye_sits_inside_the_password_box_and_touches_nothing() {
    let (cx, cy) = (640.0, 400.0);
    let (bx, by, bw, bh) = password_box(cx, cy);
    let (x, y, r) = eye_button(Mode::Password, cx, cy);
    assert!(x - r > bx && x + r < bx + bw && y - r > by && y + r < by + bh, "the eye is not inside the box");
    assert_eq!(bx + bw - (x + r), y - r - by, "as far from the end as from the top");

    let (mid, room) = typing_room(cx, cy);
    assert!(mid + room / 2.0 < x - r && mid - room / 2.0 > bx && room > 200.0);
    assert!(eye_holds(Mode::Password, (cx, cy), (x, y)) && eye_holds(Mode::Password, (cx, cy), (x + 8.0, by + 4.0)));
    assert!(!eye_holds(Mode::Password, (cx, cy), (cx, y)), "a press on the words counts as the eye");

    let (x, y, r) = eye_button(Mode::Pin, cx, cy);
    assert_eq!(y, cy + PIN_ROW);
    assert!(x - r.max(EYE_REACH) > cx + 1.5 * 24.0 + 6.0 && x + r <= cx + CARD_W / 2.0 - PAD);
    for (k, kx, ky) in keypad(cx, cy) {
        assert!((x - kx).hypot(y - ky) > KEY_R + r.max(EYE_REACH), "on key {k}");
    }
    for phone in [true, false] {
        assert!(clear_of((x, y, r), switch_button(Mode::Pin, cx, cy, phone)));
    }
    assert!(clear_of(phone_button(Mode::Pin, cx, cy, true), (x - r, y - r, 2.0 * r, 2.0 * r)));
}

#[test]
fn showing_the_password_moves_nothing() {
    let theme = Theme::new(false, crate::common::style::DEFAULT_ACCENT);
    let rgb = |c: Colour| ((c.r * 255.0).round() as u8, (c.g * 255.0).round() as u8, (c.b * 255.0).round() as u8);
    let (w, h) = (1280, 800);
    let (cx, cy) = (640.0, 400.0);
    let on = Eye { shown: true, ..Eye::default() };
    for (mode, typed) in [(Mode::Password, "john-smith"), (Mode::Pin, "12")] {
        let hidden = drawn_with(w, h, mode, typed, Eye::default(), false, "That password is not right.", true, false, None, Phone::Here { hint: false });
        let shown = drawn_with(w, h, mode, typed, on, false, "That password is not right.", true, false, None, Phone::Here { hint: false });

        let (ex, ey, er) = eye_button(mode, cx, cy);
        let (x0, y0, x1, y1) = match mode {
            Mode::Password => {
                let (bx, by, bw, bh) = password_box(cx, cy);
                (bx, by, bx + bw, by + bh)
            }
            Mode::Pin => (cx - 60.0, ey - er - 2.0, ex + er + 2.0, ey + er + 2.0),
        };
        let mut changed = 0;
        for y in 0..h as u32 {
            for x in 0..w as u32 {
                let inside = f64::from(x) >= x0 && f64::from(x) <= x1 && f64::from(y) >= y0 && f64::from(y) <= y1;
                if hidden.pixel(x, y) != shown.pixel(x, y) {
                    assert!(inside, "{mode:?}: showing changed the picture at {x}, {y}, outside what is typed");
                    changed += 1;
                }
            }
        }
        assert!(changed > 50, "{mode:?}: showing changed nothing");

        let at_eye = ((ex - er + 3.0) as u32, ey as u32);
        let rest = if mode == Mode::Pin { theme.bg } else { theme.control };
        assert_eq!(at(&hidden, at_eye.0, at_eye.1), rgb(rest), "{mode:?}");
        assert_eq!(at(&shown, at_eye.0, at_eye.1), rgb(theme.accent), "{mode:?}");
    }

    let long = "correct horse battery staple and then some more words";
    let (ex, _, er) = eye_button(Mode::Password, cx, cy);
    let (_, by, _, bh) = password_box(cx, cy);
    for eye in [Eye::default(), on] {
        let img = drawn_with(w, h, Mode::Password, long, eye, false, "", false, false, None, Phone::Absent);
        for y in by as u32 + 3..(by + bh) as u32 - 3 {
            for x in (ex - er - 7.0) as u32..(ex - er - 1.0) as u32 {
                assert_eq!(at(&img, x, y), rgb(theme.field), "something typed runs under the eye at {x}, {y}");
            }
        }
    }

    let named = drawn_with(w, h, Mode::Password, "john", Eye::default(), true, "", false, false, None, Phone::Absent);
    let plain = drawn_with(w, h, Mode::Password, "john", Eye::default(), false, "", false, false, None, Phone::Absent);
    let (ex, ey, er) = eye_button(Mode::Password, cx, cy);
    assert_ne!(at(&named, ex as u32, (ey + er + 20.0) as u32), at(&plain, ex as u32, (ey + er + 20.0) as u32));
}

fn at(img: &Pixmap, x: u32, y: u32) -> (u8, u8, u8) {
    let p = img.pixel(x, y).unwrap();
    (p.red(), p.green(), p.blue())
}

#[test]
fn the_lock_screen_is_flat_greys_in_both_modes() {
    let theme = Theme::new(false, crate::common::style::DEFAULT_ACCENT);
    let rgb = |c: Colour| ((c.r * 255.0).round() as u8, (c.g * 255.0).round() as u8, (c.b * 255.0).round() as u8);
    for mode in [Mode::Pin, Mode::Password] {
        let img = drawn(1280, 800, mode, 2, "", false, false, None, Phone::Absent);
        assert_eq!(at(&img, 4, 4), rgb(theme.bg), "page");

        let card_x = 1280 / 2 - CARD_W as u32 / 2 + 8;
        assert_eq!(at(&img, card_x, 400 - 182 + 90), rgb(theme.panel), "card");
    }
    let img = drawn(1280, 800, Mode::Pin, 0, "", false, false, None, Phone::Absent);
    let (_, kx, ky) = keypad(640.0, 400.0)[4];
    assert_eq!(at(&img, kx as u32 - 12, ky as u32 - 12), rgb(theme.bg), "a key is the page's grey");
    let lit = drawn(1280, 800, Mode::Pin, 0, "", false, false, Some('5'), Phone::Absent);
    assert_eq!(at(&lit, kx as u32 - 12, ky as u32 - 12), rgb(theme.accent), "the pressed key lights in the highlight");
}

#[test]
fn the_lock_screen_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("the lock screen", 60, || {
        let on = Eye { shown: true, ..Eye::default() };
        drawn::page("typed");
        let _ = drawn_with(1280, 800, Mode::Password, "john-smith-17", Eye::default(), true, "", false, false, None, Phone::Absent);
        let _ = drawn_with(1280, 800, Mode::Password, "john-smith-17", Eye { on: true, ..on }, true, "", false, false, None, Phone::Absent);
        let _ = drawn_with(1280, 800, Mode::Pin, "12", on, true, "", false, false, None, Phone::Absent);
        let states: [(&str, Mode, usize, &str, bool, bool, Option<char>, Phone); 5] = [
            ("pin", Mode::Pin, 2, "", false, false, Some('7'), Phone::Here { hint: true }),
            ("pin-wrong", Mode::Pin, 0, "Wrong PIN. 2 tries left.", true, false, None, Phone::Absent),
            ("password", Mode::Password, 9, "", false, false, None, Phone::Here { hint: true }),
            ("password-wrong", Mode::Password, 0, "That password is not right.", true, false, None, Phone::Absent),
            ("password-checking", Mode::Password, 0, "", false, true, None, Phone::Absent),
        ];
        for (name, mode, entered, message, wrong, checking, flash, phone) in states {
            drawn::page(name);
            let _ = drawn(1280, 800, mode, entered, message, wrong, checking, flash, phone);
        }
        drawn::page("resting");
        let fonts = Fonts::new();
        let theme = Theme::new(false, crate::common::style::DEFAULT_ACCENT);
        let mut img = Pixmap::new(1280, 800).unwrap();
        let mut pen = Pen { px: img.as_mut(), scale: 1.0, fonts: &fonts, t: &theme };
        draw_resting(&mut pen, 1280.0, 800.0, 30.0);
    }));
}

#[test]
#[ignore = "writes pictures when LOCK_SHOTS names a folder"]
fn shoot_the_lock_screen() {
    let Some(dir) = std::env::var_os("LOCK_SHOTS") else { return };
    for (w, h) in [(1280, 800), (1920, 1080)] {
        let on = Eye { shown: true, ..Eye::default() };
        for (name, mode, typed, eye, named) in [
            ("eye-password-hidden", Mode::Password, "john-smith-17", Eye::default(), true),
            ("eye-password-shown", Mode::Password, "john-smith-17", on, false),
            ("eye-password-keyboard", Mode::Password, "john-smith-17", Eye { on: true, ..on }, true),
            ("eye-pin-hidden", Mode::Pin, "12", Eye::default(), false),
            ("eye-pin-shown", Mode::Pin, "12", on, true),
        ] {
            let img = drawn_with(w, h, mode, typed, eye, named, "", false, false, None, Phone::Absent);
            img.save_png(std::path::Path::new(&dir).join(format!("lock-{name}-{w}.png"))).unwrap();
        }
        let states: [(&str, Mode, usize, &str, bool, bool, Option<char>, Phone); 5] = [
            ("pin", Mode::Pin, 2, "", false, false, Some('7'), Phone::Here { hint: true }),
            ("pin-wrong", Mode::Pin, 0, "Wrong PIN. 2 tries left.", true, false, None, Phone::Absent),
            ("password", Mode::Password, 9, "", false, false, None, Phone::Here { hint: true }),
            ("password-wrong", Mode::Password, 0, "That password is not right.", true, false, None, Phone::Absent),
            ("password-checking", Mode::Password, 0, "", false, true, None, Phone::Absent),
        ];
        for (name, mode, entered, message, wrong, checking, flash, phone) in states {
            let img = drawn(w, h, mode, entered, message, wrong, checking, flash, phone);
            img.save_png(std::path::Path::new(&dir).join(format!("lock-{name}-{w}.png"))).unwrap();
        }
        let fonts = Fonts::new();
        let theme = Theme::new(false, crate::common::style::DEFAULT_ACCENT);
        let mut img = Pixmap::new(w as u32, h as u32).unwrap();
        let mut pen = Pen { px: img.as_mut(), scale: 1.0, fonts: &fonts, t: &theme };
        draw_resting(&mut pen, f64::from(w), f64::from(h), 30.0);
        img.save_png(std::path::Path::new(&dir).join(format!("lock-resting-{w}.png"))).unwrap();
    }
}

#[test]
fn the_time_is_up_label_shows_under_the_clock_for_every_choice() {
    for secs in pocketkey::REAUTH.into_iter().filter(|s| *s != 0) {
        let label = pocketkey::due_words(secs);
        assert_eq!(shown_message("", Some(secs), false), label);
        assert_eq!(shown_message("Wrong password.", Some(secs), false), "Wrong password.", "its own message comes first");
        assert_eq!(shown_message("", Some(secs), true), "", "checking says its own words");
    }
    assert_eq!(shown_message("", None, false), "");
    for secs in pocketkey::REAUTH.into_iter().filter(|s| *s != 0) {
        let (phone, due) = pocketkey::gate(true, Some(secs));
        assert!(!phone && due == Some(secs));
        drawn(1280, 800, Mode::Password, 0, &shown_message("", due, false), false, false, None, Phone::Absent);
    }
}
