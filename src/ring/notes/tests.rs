//! Tests of the notification circles.

use crate::toolkit::input::{Input, Keysym};

use super::*;
use crate::toolkit::ui::App;
use super::card::{PAGE_LINES, WHEEL_LINES};
use super::icons::{FALLBACK_ICON, named_icon};

pub(in crate::ring) fn notes_code() -> &'static str {
    static CODE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CODE.get_or_init(|| {
        let files = [include_str!("actions.rs"), include_str!("card.rs"), include_str!("icons.rs"), include_str!("keyboard.rs"), include_str!("moving.rs"), include_str!("mod.rs")];
        files.iter().map(|text| crate::common::source::code(text).split("\n#[cfg(test)]\nmod tests {").next().unwrap_or_default().replace("#[cfg(test)]\nmod tests;\n", "")).collect::<Vec<_>>().join("\n")
    })
}
use crate::toolkit::input::{Button, Mods};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
use std::time::Instant;

fn content(title: &str, body: &str) -> Content {
    Content {
        icons: vec!["drive-removable-media-symbolic".into()], title: title.into(), body: body.into(), id: 1,
        handoff: String::new(), drive: String::new(), allow: String::new(), why: String::new(),
    }
}

fn card(many: bool, hover: bool) -> Card {
    card_with(Buttons { many, extra: None }, hover, content("Backup finished", "Everything was copied."))
}

fn card_with(buttons: Buttons, hover: bool, content: Content) -> Card {
    let d = Sizes::sheet().across;
    Card {
        lp: crate::toolkit::event::Loop::new(), notes: Weak::new(), content,
        look: Look { across: d, scroll: 0, face: Face::Note, buttons, hover, pinned: false, hot: None, copied: false, moving: false },
        inside: Cell::new(false), on: Cell::new(None), grab: 0, press: Rc::new(Cell::new(0)), last: Cell::new((0.0, 0.0)),
    }
}

fn spot(c: &Card, act: Act) -> (f32, f32) {
    let p = circles::parts(c.look.face, c.look.buttons, c.sizes()).into_iter().find(|p| p.act == act).unwrap();
    (p.at.x + p.at.w / 2.0, p.at.y + p.at.h / 2.0)
}

#[test]
fn the_screen_reader_hears_the_words_of_the_circle() {
    let c = content("Stick is ready", "John Smith's photos");
    assert_eq!(c.said(), "Stick is ready. John Smith's photos");
    assert_eq!(content("Battery at 10%", "").said(), "Battery at 10%");
}

#[test]
fn a_click_that_cannot_go_anywhere_says_why_in_the_circle_itself() {
    let mut c = content("Message from John Smith", "Hello there");
    assert_eq!(c.said(), "Message from John Smith. Hello there");
    c.why = crate::ring::note_goes::no_program().into();
    assert_eq!(c.said(), "Message from John Smith. No program found");
    assert_eq!(c.shown(), "No program found");
    assert!(c.whole().contains("Hello there"), "Copy still gives what the program sent");
}

#[test]
fn a_circle_is_one_picture_that_fills_its_window() {
    let k = card(false, false);
    let d = k.look.across as u32;
    let mut ui = Ui::new(k.view(), Theme::new(false, ""));
    let mut pix = tiny_skia::Pixmap::new(d, d).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let r = d as f32 / 2.0;
    let centre = pix.pixel(r as u32, r as u32).unwrap();
    assert!(centre.alpha() > 0, "the middle of it is painted");
}

#[test]
fn pointing_on_the_circle_brings_the_buttons_and_the_corner_of_its_square_does_not() {
    let c = card(false, false);
    let d = c.look.across;
    assert_eq!(c.intercept(&Input::PointerIn { x: 2.0, y: 2.0 }, false), None, "a corner of the square is not the circle");
    let (x, y) = (d / 2.0, d / 2.0);
    assert_eq!(c.intercept(&Input::PointerMove { x, y }, false), Some(Msg::At { inside: true, hot: None }));
    assert_eq!(c.intercept(&Input::PointerMove { x: 2.0, y: 2.0 }, false), None);
    assert!(c.inside.get());
    assert_eq!(c.intercept(&Input::PointerOut, false), Some(Msg::At { inside: false, hot: None }));
    assert_eq!(c.intercept(&Input::PointerOut, false), None);
}

const WITH_EVERYTHING: Buttons = Buttons { many: true, extra: Some(Act::Drive) };

fn drive() -> Content {
    Content { drive: "/org/freedesktop/UDisks2/block_devices/sdb1".into(), ..content("Stick is ready", "John Smith's photos") }
}

fn tab() -> Input {
    key(Keysym::Tab, false)
}

fn key(sym: Keysym, shift: bool) -> Input {
    Input::Key { sym, text: None, mods: Mods { shift, ..Default::default() }, repeat: false }
}

fn click(button: Button, x: f32, y: f32) -> (Input, Input) {
    (Input::Press { button, x, y }, Input::Release { button, x, y })
}

fn laid(c: &Card) -> Ui<Msg> {
    let mut ui = Ui::new(c.view(), Theme::new(false, ""));
    ui.lay_at(c.look.across as u32, c.look.across as u32);
    ui
}

#[test]
fn a_press_and_release_on_a_button_is_the_buttons_own_and_the_circle_does_not_act_twice() {
    let c = card_with(WITH_EVERYTHING, true, drive());
    c.inside.set(true);
    let mut ui = laid(&c);
    let now = Instant::now();
    for act in [Act::Close, Act::Pin, Act::Snooze, Act::Copy, Act::Drive, Act::ClearAll] {
        let (x, y) = spot(&c, act);
        assert_eq!(c.intercept(&Input::PointerMove { x, y }, false), Some(Msg::At { inside: true, hot: Some(act) }));
        let (down, up) = click(Button::Left, x, y);
        assert_eq!(c.intercept(&down, false), None);
        assert_eq!(c.intercept(&up, false), None);
        ui.input(&Input::PointerMove { x, y }, now);
        assert!(ui.input(&down, now).messages.is_empty());
        assert_eq!(ui.input(&up, now).messages, vec![Msg::Act(act)], "{act:?}");
    }
    let ((px, py), (cx, cy)) = (spot(&c, Act::Pin), spot(&c, Act::Close));
    ui.input(&click(Button::Left, px, py).0, now);
    assert!(ui.input(&click(Button::Left, cx, cy).1, now).messages.is_empty());
    let r = c.look.across / 2.0;
    let (down, up) = click(Button::Left, r, r * 0.7);
    ui.input(&down, now);
    assert_eq!(ui.input(&up, now).messages, vec![Msg::Act(Act::Open)]);
}

#[test]
fn at_rest_a_circle_already_has_its_buttons_and_pointing_only_adds_labels() {
    let k = Sizes::sheet();
    let acts = |b: Buttons| -> Vec<Act> { circles::parts(Face::Note, b, k).iter().map(|p| p.act).filter(|a| *a != Act::Open).collect() };
    let many = Buttons { many: true, extra: None };
    assert_eq!(acts(many), vec![Act::Close, Act::Pin, Act::Snooze, Act::Copy, Act::ClearAll]);
    assert_eq!(acts(Buttons { many: false, extra: None }), vec![Act::Close, Act::Pin, Act::Snooze, Act::Copy]);
    assert_eq!(acts(Buttons { many: false, extra: Some(Act::Drive) }), vec![Act::Close, Act::Pin, Act::Snooze, Act::Copy, Act::Drive]);
    assert_eq!(acts(Buttons { many: true, extra: Some(Act::Allow) }), vec![Act::Close, Act::Pin, Act::Snooze, Act::Copy, Act::Allow, Act::ClearAll]);
    assert_eq!((drive().extra(), content("A", "b").extra()), (Some(Act::Drive), None));
    assert_eq!(Content { allow: "example-plugin".into(), ..content("A", "b") }.extra(), Some(Act::Allow));
}

#[test]
fn every_part_of_a_circle_is_named_for_the_screen_reader_and_reached_and_pressed_by_keys() {
    use accesskit::Role;
    let c = card_with(WITH_EVERYTHING, true, drive());
    let mut ui = laid(&c);
    let tree = ui.access("Notification");
    let names: Vec<String> = tree.nodes.iter().filter(|(_, n)| n.role() == Role::Button).map(|(_, n)| n.label().unwrap_or("").to_string()).collect();
    assert_eq!(names, vec!["Stick is ready. John Smith's photos", "Close", "Pin", "Snooze", "Copy the message", "Open", "Clear all"], "the message whole, then each button");
    let now = Instant::now();
    let order = [Act::Open, Act::Close, Act::Pin, Act::Snooze, Act::Copy, Act::Drive, Act::ClearAll];
    for (i, act) in order.iter().enumerate() {
        assert!(ui.input(&tab(), now).messages.is_empty());
        let press = if i % 2 == 0 { Keysym::space } else { Keysym::Return };
        assert_eq!(ui.input(&key(press, false), now).messages, vec![Msg::Act(*act)], "{act:?}");
    }
    ui.input(&tab(), now);
    assert_eq!(ui.input(&key(Keysym::space, false), now).messages, vec![Msg::Act(Act::Open)]);
    ui.input(&key(Keysym::ISO_Left_Tab, true), now);
    assert_eq!(ui.input(&key(Keysym::space, false), now).messages, vec![Msg::Act(Act::ClearAll)]);
    assert_eq!(ui.input(&key(Keysym::Escape, false), now).messages, vec![Msg::Leave]);
}

#[test]
fn the_keyboard_lands_on_the_first_button_when_a_circle_is_given_it() {
    let mut c = card_with(Buttons { many: false, extra: None }, true, content("Backup finished", "Done."));
    c.grab = 1;
    let mut ui = laid(&c);
    let now = Instant::now();
    assert_eq!(ui.input(&key(Keysym::space, false), now).messages, vec![Msg::Act(Act::Close)], "Close, with no Tab first");
    c.look.face = Face::Snooze;
    c.grab = 2;
    let mut ui = laid(&c);
    let tree = ui.access("Notification");
    let names: Vec<String> = tree.nodes.iter().filter(|(_, n)| n.role() == accesskit::Role::Button).map(|(_, n)| n.label().unwrap_or("").to_string()).collect();
    assert_eq!(names[1..], ["Snooze for 10 minutes", "Snooze for 1 hour", "Snooze for 6 hours", "Snooze for next morning", "Back to the message"]);
    assert_eq!(ui.input(&key(Keysym::space, false), now).messages, vec![Msg::Act(Act::Pick(circles::Snooze::Minutes10))]);
}

#[test]
fn pointing_on_a_button_names_it() {
    let c = card_with(WITH_EVERYTHING, true, drive());
    let mut ui = laid(&c);
    for (act, name) in [(Act::Close, "Close"), (Act::Pin, "Pin"), (Act::Snooze, "Snooze"), (Act::Copy, "Copy the message"), (Act::Drive, "Open"), (Act::ClearAll, "Clear all")] {
        assert_eq!(ui.label_at(&format!("circle-{act:?}")).as_deref(), Some(name));
    }
    assert_eq!(ui.label_at("circle-message"), None, "the message needs no label repeating it");
}

#[test]
fn a_circle_can_be_copied_by_its_button_by_ctrl_and_c_and_by_a_right_click() {
    let c = card(false, true);
    assert_eq!(c.content.whole(), "Backup finished\nEverything was copied.");
    assert_eq!(content("Code", "").whole(), "Code");
    let ctrl_c = Input::Key { sym: Keysym::c, text: None, mods: Mods { ctrl: true, ..Default::default() }, repeat: false };
    assert_eq!(c.intercept(&ctrl_c, true), Some(Msg::Act(Act::Copy)));
    let plain_c = Input::Key { sym: Keysym::c, text: None, mods: Default::default(), repeat: false };
    assert_eq!(c.intercept(&plain_c, true), None);
    let mut ui = laid(&c);
    let r = c.look.across / 2.0;
    let (down, up) = click(Button::Right, r, r * 0.7);
    let (a, b) = (ui.input(&down, Instant::now()), ui.input(&up, Instant::now()));
    assert!(a.redraw || b.redraw, "a right click on the message offers a menu");
    let name = |copied| Act::Copy.name(false, copied);
    assert_eq!((name(false).as_str(), name(true).as_str()), ("Copy the message", "Copied"));
}

#[test]
fn the_arrows_and_page_keys_scroll_a_long_message() {
    let c = card(false, true);
    for (sym, lines) in [(Keysym::Down, 1), (Keysym::Up, -1), (Keysym::Page_Down, PAGE_LINES), (Keysym::Page_Up, -PAGE_LINES)] {
        assert_eq!(c.intercept(&key(sym, false), true), Some(Msg::Scroll(lines)));
    }
}

#[test]
fn a_pin_made_on_the_circle_or_in_the_list_is_the_same_pin_in_both() {
    let stack = Rc::new(RefCell::new(Stack::new(Sizes::sheet(), 0)));
    notify::say_here("Backup finished", "Done.");
    let id = notify::list()[0].id;
    stack.borrow_mut().add(id, 0, 0);
    let there = stack.clone();
    notify::on_pinned(move |n, pinned| {
        there.borrow_mut().set_pin(n, pinned, 0);
    });
    let in_list = || notify::list().iter().find(|n| n.id == id).is_some_and(|n| n.pinned);
    let pinned = stack.borrow_mut().toggle_pin(id, 0);
    notify::set_pinned(id, pinned);
    assert!(stack.borrow().pinned(id) && in_list(), "pinned on the circle: pinned in the list");
    let pinned = stack.borrow_mut().toggle_pin(id, 0);
    notify::set_pinned(id, pinned);
    assert!(!stack.borrow().pinned(id) && !in_list());
    notify::set_pinned(id, true);
    assert!(stack.borrow().pinned(id) && in_list(), "pinned in the list: pinned on the circle");
    notify::set_pinned(id, true);
    assert!(stack.borrow().pinned(id), "the same pin again changes nothing");
    notify::set_pinned(id, false);
    assert!(!stack.borrow().pinned(id) && !in_list());
}

#[test]
fn the_wheel_scrolls_the_message() {
    let c = card(false, true);
    assert_eq!(c.intercept(&Input::Scroll { x: 10.0, y: 1.0, steps: true }, false), Some(Msg::Scroll(WHEEL_LINES)));
    assert_eq!(c.intercept(&Input::Scroll { x: 10.0, y: -1.0, steps: true }, false), Some(Msg::Scroll(-WHEEL_LINES)));
    assert_eq!(c.intercept(&Input::Scroll { x: 10.0, y: 32.0, steps: false }, false), Some(Msg::Scroll(2)));
    assert_eq!(c.intercept(&Input::Scroll { x: 10.0, y: 2.0, steps: false }, false), None);
}

fn pinned_card() -> Card {
    let mut c = card(false, true);
    c.look.pinned = true;
    c
}

fn moving_card() -> Card {
    let mut c = pinned_card();
    c.look.moving = true;
    c
}

fn alt(sym: Keysym) -> Input {
    Input::Key { sym, text: None, mods: Mods { alt: true, ..Default::default() }, repeat: false }
}

#[test]
fn holding_a_pinned_circle_shorter_than_the_time_does_nothing() {
    let c = pinned_card();
    let (down, up) = click(Button::Left, 40.0, 40.0);
    assert_eq!(c.intercept(&down, false), None);
    let token = c.press.get();
    assert!(c.press.get() == token, "still held");
    assert_eq!(c.intercept(&up, false), None);
    assert!(c.press.get() != token, "let go before the time: the hold finds it is not that press");
    c.intercept(&down, false);
    let token = c.press.get();
    c.intercept(&Input::PointerOut, false);
    assert!(c.press.get() != token, "the pointer leaving ends it too");
    c.intercept(&Input::Press { button: Button::Right, x: 5.0, y: 5.0 }, false);
    let after_right = c.press.get();
    c.intercept(&down, false);
    assert_eq!(c.press.get(), after_right.wrapping_add(1), "the left press starts one");
    let token = c.press.get();
    c.intercept(&Input::Press { button: Button::Right, x: 5.0, y: 5.0 }, false);
    assert!(c.press.get() != token, "any other press ends it");
}

#[test]
fn a_circle_that_is_not_pinned_starts_no_hold_and_its_move_keys_say_why() {
    let c = card(false, true);
    let before = c.press.get();
    c.intercept(&Input::Press { button: Button::Left, x: 40.0, y: 40.0 }, false);
    assert_eq!(c.press.get(), before.wrapping_add(1), "a press only ends: no hold is begun for it");
    assert_eq!(c.intercept(&alt(Keysym::Left), false), Some(Msg::Nudge { right: -1, down: 0 }));
}

#[test]
fn a_held_circle_follows_the_pointer_is_dropped_by_release_and_escape_puts_it_back() {
    let c = moving_card();
    assert_eq!(c.intercept(&Input::PointerMove { x: 90.0, y: 70.0 }, false), Some(Msg::Drag { x: 90.0, y: 70.0 }));
    assert_eq!(c.intercept(&Input::Release { button: Button::Left, x: 90.0, y: 70.0 }, false), Some(Msg::Drop));
    assert_eq!(c.intercept(&key(Keysym::Escape, false), false), Some(Msg::PutBack));
    let (x, y) = spot(&c, Act::Close);
    let (down, _) = click(Button::Left, x, y);
    assert!(matches!(c.intercept(&down, false), Some(Msg::Drag { .. })), "nothing reaches its buttons while held");
    assert_eq!(c.intercept(&key(Keysym::Tab, false), false), None);
    assert!(matches!(c.intercept(&Input::Scroll { x: 1.0, y: 1.0, steps: true }, false), Some(Msg::Drag { .. })));
}

#[test]
fn the_compositor_taking_up_a_move_is_heard_whether_or_not_the_circle_is_held() {
    assert_eq!(moving_card().intercept(&Input::Placed, false), Some(Msg::Placed));
    assert_eq!(pinned_card().intercept(&Input::Placed, false), Some(Msg::Placed));
}

#[test]
fn alt_and_the_arrows_move_a_circle_by_keys_and_the_arrows_alone_still_scroll() {
    let c = pinned_card();
    assert_eq!(c.intercept(&alt(Keysym::Left), false), Some(Msg::Nudge { right: -1, down: 0 }));
    assert_eq!(c.intercept(&alt(Keysym::Right), false), Some(Msg::Nudge { right: 1, down: 0 }));
    assert_eq!(c.intercept(&alt(Keysym::Up), false), Some(Msg::Nudge { right: 0, down: -1 }));
    assert_eq!(c.intercept(&alt(Keysym::Down), false), Some(Msg::Nudge { right: 0, down: 1 }));
    assert_eq!(c.intercept(&key(Keysym::Down, false), false), Some(Msg::Scroll(1)));
}

#[test]
fn the_hold_time_and_the_step_are_the_style_sheets_and_held_to_safe_limits() {
    let s = theme::sheet();
    assert!((1000.0..=4000.0).contains(&s.circle_hold) && (8.0..=96.0).contains(&s.circle_step));
    assert_eq!(s.circle_hold, 2000.0);
}

#[test]
fn a_window_is_held_to_the_two_edges_of_its_corner_and_slides_with_its_margins() {
    let screen = (1280.0f32, 800.0f32);
    let (x, y, d) = (1280.0 - 16.0 - 160.0, 800.0 - 16.0 - 160.0, 160.0);
    assert_eq!(Notes::margins(Corner::BottomRight, screen, (x, y, d)), [0, 16, 16, 0]);
    assert_eq!(Notes::margins(Corner::BottomLeft, screen, (16.0, y, d)), [0, 0, 16, 16]);
    assert_eq!(Notes::margins(Corner::TopLeft, screen, (16.0, 16.0, d)), [16, 0, 0, 16]);
    assert_eq!(Notes::margins(Corner::TopRight, screen, (x, 16.0, d)), [16, 16, 0, 0]);
    assert_eq!(Notes::margins(Corner::BottomRight, screen, (x, y - 172.0, d)), [0, 16, 188, 0]);
}

#[test]
fn a_notification_circle_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("a notification circle window", 2, || {
        let scenes: [(&str, Face, Buttons, bool, usize, bool); 6] = [
            ("at rest", Face::Note, WITH_EVERYTHING, false, 0, false),
            ("pointed", Face::Note, WITH_EVERYTHING, true, 0, false),
            ("ring on a button", Face::Note, WITH_EVERYTHING, true, 3, false),
            ("copied", Face::Note, Buttons { many: false, extra: None }, true, 0, true),
            ("snooze choices", Face::Snooze, WITH_EVERYTHING, true, 0, false),
            ("ring on a choice", Face::Snooze, WITH_EVERYTHING, true, 2, false),
        ];
        for (state, face, buttons, hover, tabs, copied) in scenes {
            let mut c = card_with(buttons, hover, drive());
            c.look.face = face;
            c.look.copied = copied;
            c.look.across = 200.0;
            let side = c.look.across as u32;
            let mut ui = Ui::new(c.view(), Theme::chosen());
            ui.see_through = true;
            let mut pix = tiny_skia::Pixmap::new(side, side).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            for _ in 0..tabs {
                ui.input(&tab(), Instant::now());
            }
            drawn::page(state);
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }));
}

#[test]
fn a_program_can_name_only_a_picture_as_its_icon() {
    let note = |icon: &str| Note { id: 1, app: String::new(), icon: icon.into(), desktop: String::new(), summary: String::new(), body: String::new(),
        at: crate::common::clock::Local::now(), urgent: false, pinned: false, handoff: String::new(), drive: String::new(), allow: String::new() };
    assert_eq!(named_icon(&note("file:///usr/share/pixmaps/My%20Pic.PNG")), "/usr/share/pixmaps/My Pic.PNG");
    assert_eq!(named_icon(&note("file:///home/john/.ssh/id_ed25519")), FALLBACK_ICON);
    assert_eq!(named_icon(&note("/etc/shadow")), FALLBACK_ICON);
    assert_eq!(named_icon(&note("/usr/share/pixmaps/a.svg")), "/usr/share/pixmaps/a.svg");
    assert_eq!(named_icon(&note("mail-unread")), "mail-unread");
    assert_eq!(named_icon(&note("")), FALLBACK_ICON);
}

#[test]
fn a_sideways_wheel_step_scrolls_nothing() {
    let c = card(false, true);
    let wheel = |y: f32| Input::Scroll { x: 1.0, y, steps: true };
    assert_eq!(c.intercept(&wheel(0.0), false), None);
    assert_eq!(c.intercept(&wheel(1.0), false), Some(Msg::Scroll(WHEEL_LINES)));
}
