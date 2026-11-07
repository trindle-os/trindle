//! The GitHub key.

use crate::store::{github, go, Msg, Store, View};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, entry, note, row, space, Length, Node, Tone};
use crate::toolkit::theme;
use crate::store::look::{self, Dot};
use super::links::{link_button, red_about};

const KEY_ICON: &str = "dialog-password-symbolic";

pub(super) fn key_button(s: &Store) -> Vec<Node<Msg>> {
    let g = &s.github;
    let stored = g.key_is == github::KeyIs::Stored;
    let name = if stored { t!("store-gh-key-stored") } else { t!("store-gh-key") };
    let open = button(KEY_ICON, name).key("gh:key-open").on(g.key_at.is_some())
        .on_press(go(|s| s.github.key_at = Some(s.pressed_at.unwrap_or_default())));
    if stored { vec![look::dot_named(s, Dot::UpToDate, &g.key_is.words()), open] } else { vec![open] }
}

pub(super) fn look_for_key(s: &mut Store) {
    github::key_is(s.then(|s: &mut Store, is: github::KeyIs| s.github.key_is = is));
}

fn remove_key(s: &mut Store) {
    github::set_key("", key_set(s, String::new(), true), key_late(s, true));
}

fn key_set(s: &Store, typed: String, removing: bool) -> impl FnOnce(github::Set, github::KeyIs) + 'static {
    let back = s.then(move |s: &mut Store, (how, is): (github::Set, github::KeyIs)| {
        s.github.key_is = is;
        match how {
            github::Set::Made => { s.said.remove("gh:key"); }
            github::Set::Not(e) => {
                if s.github.key.is_empty() {
                    s.github.key = typed;
                }
                s.say("gh:key", if removing { t!("store-gh-key-not-removed", why = e) } else { t!("store-gh-key-not-kept", why = e) }, Tone::Problem);
            }
            github::Set::Waiting => s.say("gh:key", github::waiting_words(removing), Tone::Plain),
        }
    });
    move |how, is| back((how, is))
}

fn key_late(s: &Store, removing: bool) -> impl FnOnce(Result<(), String>, github::KeyIs) + 'static {
    let back = s.then(move |s: &mut Store, (how, is): (Result<(), String>, github::KeyIs)| {
        s.github.key_is = is;
        let (words, ok) = github::late_words(removing, &how);
        s.say("gh:key", words, if ok { Tone::Plain } else { Tone::Problem });
    });
    move |how, is| back((how, is))
}

pub(crate) fn close_key_box(s: &mut Store) {
    s.github.key_at = None;
    s.github.key.clear();
}

pub(crate) fn key_box_open(s: &Store) -> bool {
    s.github.key_at.is_some() && s.view == View::GitHub && s.detail.is_none()
}

pub(crate) fn key_box(s: &Store) -> Option<Node<Msg>> {
    let at = s.github.key_at.filter(|_| key_box_open(s))?;
    let g = &s.github;
    let stored = !matches!(g.key_is, github::KeyIs::None | github::KeyIs::NoKeyring | github::KeyIs::Looking);
    let mut line = vec![
        entry(g.key.clone(), if stored { t!("store-gh-key-keep-empty") } else { t!("store-gh-key-paste") },
              |t| go(move |s| s.github.key = t.clone())).key("gh:key-box").secret_of(crate::toolkit::widget::Secret::Key).first()
            .on_submit(go(keep_key)).width(Length::Fill),
    ];
    if stored {
        line.push(crate::toolkit::widget::remove("user-trash-symbolic", t!("store-gh-key-remove")).key("gh:key-gone")
                  .on_press(go(remove_key)));
    }
    line.extend([
        link_button(t!("store-gh-key-make"), "https://github.com/settings/personal-access-tokens/new"),
        button(crate::settings::ui::SAVE_ICON, t!("store-gh-key-keep")).on_press(go(keep_key)),
    ]);
    let typed = line.remove(0);
    let mut buttons = vec![space()];
    buttons.extend(line);
    let standing = match &g.key_is {
        github::KeyIs::Stored => look::dot_line(s, Dot::UpToDate, g.key_is.words()),
        github::KeyIs::Plain(_) | github::KeyIs::Hidden(_) | github::KeyIs::Unsendable => red_about(g.key_is.words(), crate::store::help::GITHUB_KEY),
        _ => look::dot_line(s, Dot::Quiet, g.key_is.words()),
    };
    let mut parts = vec![
        crate::toolkit::tiles::tile_head(KEY_ICON, t!("store-gh-key"), Vec::new()),
        standing,
        note(t!("store-gh-key-words")).width(Length::Fill),
        note(t!("store-gh-key-how")).width(Length::Fill),
        typed,
        row(buttons).gap(theme::sheet().gap).centred(),
    ];
    parts.extend(s.said.get("gh:key").filter(|(w, _)| !w.is_empty()).map(|(w, t)| if *t == Tone::Problem { crate::store::help::red(w.clone(), s.more("gh:key")) } else { note(w.clone()).tone(*t).width(Length::Fill) }));
    let wide = theme::apps_list().card_least;
    let panel = card(column(parts).gap(theme::sheet().gap)).floats().width(Length::Fixed(wide));
    Some(crate::toolkit::widget::placed(panel, move |(w, h), (aw, ah)| {
        let x = (at.x + at.w - w).min(aw - w - 8.0).max(8.0);
        let below = at.y + at.h + 8.0;
        let y = if below + h > ah - 8.0 { (at.y - h - 8.0).max(8.0) } else { below };
        (x, y)
    }).stays("gh:key"))
}

fn keep_key(s: &mut Store) {
    let key = std::mem::take(&mut s.github.key);
    if key.trim().is_empty() {
        return;
    }
    github::set_key(key.trim(), key_set(s, key.clone(), false), key_late(s, false));
}
