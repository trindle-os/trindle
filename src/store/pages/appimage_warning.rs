//! The card that warns before any AppImage is installed, and the tick that stops it coming again.

use crate::store::{formats, go, Msg, Store};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{card, column, drawing, form_label, main_word, note, placed, row, space, tick, word_button, Length, Node, Tone};
use super::install::pressed_install;

pub(crate) struct Warn {
    install: Box<dyn FnOnce(&mut Store)>,
    ticked: bool,
}

pub(crate) fn before(s: &mut Store, install: impl FnOnce(&mut Store) + 'static) {
    if s.warning_off {
        return install(s);
    }
    match s.warn.as_mut() {
        Some(w) => {
            let first = std::mem::replace(&mut w.install, Box::new(|_| {}));
            w.install = Box::new(move |s| {
                first(s);
                install(s);
            });
        }
        None => s.warn = Some(Warn { install: Box::new(install), ticked: false }),
    }
}

pub(crate) fn showing(s: &Store) -> bool {
    s.warn.is_some()
}

fn ok(s: &mut Store) {
    pressed_install(s, |s| {
        let Some(w) = s.warn.take() else { return };
        if w.ticked {
            s.warning_off = true;
            if let Err(e) = formats::set_warning_off_in(&s.conf) {
                s.top = Some((e.to_string(), Tone::Problem));
            }
        }
        (w.install)(s);
    });
}

pub(crate) fn cancel(s: &mut Store) {
    s.warn = None;
}

pub(crate) fn ticked(s: &mut Store, on: bool) {
    if let Some(w) = s.warn.as_mut() {
        w.ticked = on;
    }
}

pub(crate) fn over(s: &Store) -> Vec<Node<Msg>> {
    let Some(w) = &s.warn else { return Vec::new() };
    let hold = drawing((0.0, 0.0), "", |_, _| {}).width(Length::Fill).height(Length::Fill).on_pointer(|_| go(|_| {}));
    let wide = (theme::apps_list().card_least * 1.5).min(s.across - 32.0).max(260.0);
    let panel = placed(warning_card(w).width(Length::Fixed(wide)), |(w, h), (aw, ah)| ((aw - w) / 2.0, ((ah - h) / 2.0).max(8.0)));
    vec![hold, panel]
}

fn warning_card(w: &Warn) -> Node<Msg> {
    let gap = theme::sheet().gap;
    let words = column(vec![form_label(t!("store-appimage-warn-head")), note(t!("store-appimage-warn-box")).width(Length::Fill)]).gap(8.0);
    let answer = row(vec![
        tick(w.ticked, t!("store-appimage-warn-hide"), |on| go(move |s| ticked(s, on))).key("appimage-warn-tick"),
        note(t!("store-appimage-warn-hide")),
        space().width(Length::Fill),
        word_button(t!("store-appimage-warn-cancel")).on_press(go(cancel)).key("appimage-warn-cancel").first(),
        main_word(t!("store-appimage-warn-ok")).on_press(go(ok)).key("appimage-warn-ok"),
    ]).gap(gap).centred().wraps();
    card(column(vec![words, answer]).gap(gap).width(Length::Fill)).floats().on_escape(go(cancel)).key("appimage-warn")
}
