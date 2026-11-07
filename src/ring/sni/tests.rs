//! Tests of the tray icons.

use crate::toolkit::colour::Colour;
use crate::toolkit::widget::{Pen, Rect};

use super::*;
use super::menu::{may_close, menu_frame, menu_gap, menu_line, menu_words, Part};

pub(in crate::ring) fn sni_code() -> &'static str {
    static CODE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CODE.get_or_init(|| {
        let files = [include_str!("look.rs"), include_str!("menu.rs"), include_str!("outside.rs"), include_str!("mod.rs")];
        files.iter().map(|text| crate::common::source::code(text).split("\n#[cfg(test)]\nmod tests {").next().unwrap_or_default().replace("#[cfg(test)]\nmod tests;\n", "")).collect::<Vec<_>>().join("\n")
    })
}

#[test]
fn a_tray_icons_close_never_reaches_init_this_program_or_an_unknown_process() {
    assert!(!may_close(0) && !may_close(1) && !may_close(std::process::id()));
    assert!(may_close(std::process::id() + 1));
}

fn lines(words: &[&str]) -> Vec<Part> {
    words.iter().map(|w| menu_line(w.to_string(), msg(|_| {}))).collect()
}

#[test]
fn a_menu_says_its_lines_in_order_with_its_groups_apart() {
    let mut parts = lines(&["Open", "Pause", "Mute"]);
    parts.push(menu_gap());
    parts.extend(lines(&["Quit"]));
    assert_eq!(menu_frame(parts).said(), ["Open", "Pause", "Mute", "Quit"]);
}

#[test]
fn an_icon_outside_the_set_is_told_from_its_names_alone() {
    let want = |name: &str, folder: &str| Some(Want { name: name.into(), folder: folder.into() });
    assert_eq!(wanted_outside("player", "/mnt/share/icons", false, false), want("player", "/mnt/share/icons"));
    assert_eq!(wanted_outside("player", "/mnt/share/icons", false, true), want("player", "/mnt/share/icons"));
    assert_eq!(wanted_outside("player", "/mnt/share/icons", true, false), None);
    assert_eq!(wanted_outside("player", "", false, false), None);
    assert_eq!(wanted_outside("", "/mnt/share/icons", false, false), None);
    assert_eq!(wanted_outside("/mnt/share/player.png", "/mnt/share/icons", false, false), want("/mnt/share/player.png", ""));
    assert_eq!(wanted_outside("/mnt/share/player.png", "", false, true), None);
}

#[test]
fn one_look_for_an_icon_is_out_at_a_time_and_an_old_answer_is_not_shown() {
    let want = |name: &str| Some(Want { name: name.into(), folder: "/mnt/share/icons".into() });
    let mut outside = Outside::default();
    assert_eq!(outside.asks(None), Step::Nothing);
    assert_eq!(outside.asks(want("a")), Step::Look(want("a").unwrap()));
    assert_eq!(outside.asks(want("a")), Step::Nothing);
    assert_eq!(outside.asks(want("b")), Step::Nothing);
    assert_eq!(outside.back(), (false, Step::Look(want("b").unwrap())));
    assert_eq!(outside.back(), (true, Step::Nothing));
    assert_eq!(outside.asks(want("b")), Step::Look(want("b").unwrap()));
    assert_eq!(outside.asks(None), Step::Nothing);
    assert_eq!(outside.back(), (false, Step::Nothing));
    let stand_in = || Rc::new(Look::Named("trindle-generic-01".into()));
    let mut outside = Outside::default();
    assert!(!outside.keeps(&want("a"), stand_in()), "nothing was on show to keep");
    assert_eq!(outside.asks(want("a")), Step::Look(want("a").unwrap()));
    assert_eq!(outside.back(), (true, Step::Nothing));
    assert!(matches!(outside.shows(Some(Look::Named("/mnt/share/icons/a-symbolic.svg".into()))).as_deref(), Some(Look::Named(n)) if n.starts_with("/mnt")));
    assert!(outside.keeps(&want("a"), stand_in()), "the icon blinked to its stand-in while it was looked for again");
    assert!(!outside.keeps(&want("b"), stand_in()) && !outside.keeps(&want("a"), stand_in()));
    outside.asks(want("a"));
    outside.back();
    assert!(outside.shows(Some(Look::Named("/mnt/share/a.svg".into()))).is_some());
    assert!(outside.keeps(&want("a"), stand_in()));
    assert!(matches!(outside.shows(None).as_deref(), Some(Look::Named(n)) if n == "trindle-generic-01"));
    assert!(outside.shows(None).is_none());
}

#[test]
#[ignore]
fn shoot_ring_tray_menu() {
    let mut parts = vec![menu_words("Playing now".into())];
    parts.extend(lines(&["Open", "Pause", "Mute", "Share", "Settings"]));
    parts.push(menu_gap());
    parts.extend(lines(&["Quit"]));
    crate::ring::shots::save("tray-menu", widget::card(menu_frame(parts)).floats(), (300, 340));
}
#[test]
fn a_tray_icon_with_none_is_drawn_in_the_ink_alone() {
    let (t, fonts, icons) = (crate::toolkit::theme::Theme::new(false, ""), crate::toolkit::text::Fonts::new(), Icons::new());
    let pen = Pen { theme: &t, fonts: &fonts, icons: &icons, scale: 1.0 };
    let look = Look::shape(Rc::new(Icons::new()), crate::common::apps::generic_icon_for("org.example.Flow", "Flow"));
    let ink = Colour::rgb(1.0, 1.0, 1.0);
    let mut pix = tiny_skia::Pixmap::new(24, 24).unwrap();
    look.paint(&mut pix.as_mut(), Rect { x: 0.0, y: 0.0, w: 24.0, h: 24.0 }, ink, &pen);
    let drawn: Vec<_> = pix.pixels().iter().filter(|p| p.alpha() > 0).collect();
    assert!(!drawn.is_empty(), "something is drawn");
    assert!(drawn.iter().all(|p| p.red() == p.green() && p.green() == p.blue()), "only the ink's own colour");
}
