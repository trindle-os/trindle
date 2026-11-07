//! One update's row, its Hold back button, and the key of a thing's row in the Held back card.

use crate::toolkit::t;
use crate::store::listing;
use crate::store::plugin_page;
use crate::store::{go, Msg, Store};
use crate::common::held::{Kind, Thing};
use crate::toolkit::theme;
use crate::toolkit::widget::{button, note, space, Length, Node, Tone};
use super::hold::{set_held};
use super::words::stopped_row;
use super::{HOLD_ICON, app_key, busy};

pub(super) enum Own {
    Button { key: String, problem: Option<String>, going: Option<f32>, press: Box<dyn Fn(&mut Store)>, hold: Option<Thing>, called: Option<String> },
    WithGroup,
}

impl Own {
    pub(super) fn button(s: &Store, key: String, going: Option<f32>, press: impl Fn(&mut Store) + 'static) -> Own {
        let problem = s.said.get(&key).filter(|(w, t)| !w.is_empty() && *t == Tone::Problem).map(|(w, _)| w.clone());
        Own::Button { key, problem, going, press: Box::new(press), hold: None, called: None }
    }

    pub(super) fn called(mut self, name: String) -> Own {
        if let Own::Button { called, .. } = &mut self {
            *called = Some(name);
        }
        self
    }

    pub(super) fn holding(mut self, thing: Option<Thing>) -> Own {
        if let Own::Button { hold, .. } = &mut self {
            *hold = thing;
        }
        self
    }
}

pub(crate) fn hold_key(key: &str) -> String {
    format!("hold:{key}")
}

pub(crate) fn hold_button(s: &Store, thing: &Thing, name: &str, key: &str) -> Node<Msg> {
    let held = s.held.has(thing);
    let tip = if held { t!("store-upd-release", name = name) } else { t!("store-upd-hold", name = name) };
    let (thing, row) = (thing.clone(), key.to_string());
    button(HOLD_ICON, tip).key(hold_key(key)).on(held).on_press(go(move |s| set_held(s, &thing, !held, &row))).off_because(crate::common::ui::wait_while(busy(s)))
}

pub(super) fn update_line(s: &Store, picture: Option<Node<Msg>>, name: String, detail: String, held_detail: String, size: String, own: Own) -> Node<Msg> {
    let room = || space().width(Length::Fixed(theme::icon_button()));
    let (figure, keep, end, under, held) = match own {
        Own::Button { key, problem, going, press, hold, called } => {
            let held = going.is_none() && hold.as_ref().is_some_and(|t| s.held.has(t));
            let called = called.unwrap_or_else(|| name.clone());
            let b = button(crate::settings::ui::UPDATE_ICON, t!("store-installed-update", name = &called)).key(key.clone());
            let b = match going {
                Some(share) => b.working(share, Tone::Accent),
                None => b.on_press(go(press)).off(held).off_because(crate::common::ui::wait_while(busy(s))),
            };
            let keep = match (&hold, s.upd.stop_button(&key)) {
                (_, Some(stop)) => stop,
                (Some(thing), None) => hold_button(s, thing, &called, &key).off(going.is_some()).off_because(crate::common::ui::wait_while(busy(s))),
                (None, None) => room(),
            };
            let said_stopped = s.upd.stopped.contains(&key);
            let figure = going.map(|share| format!("{:.0}%", (share.clamp(0.0, 1.0) * 100.0).floor()));
            let under = problem.map(|w| { let page = if key.starts_with("pl:") { plugin_page::page_of(&w) } else { s.page_of(&key) }; crate::store::pages::links::red_about(w, page) })
                .or_else(|| said_stopped.then(|| note(stopped_row())));
            (figure, keep, b, under, held)
        }
        Own::WithGroup => (None, room(), room(), None, false),
    };
    let line = listing::row_line(if held { held_detail } else { detail });
    let size = note(figure.unwrap_or(size)).off(held);
    listing::row_or_greyed(held, picture, name, line, under.into_iter().collect(), vec![size, keep, end])
}

pub(super) fn red_words(s: &Store, key: &str) -> Option<Node<Msg>> {
    s.said.get(key).filter(|(w, t)| !w.is_empty() && *t == Tone::Problem).map(|(w, _)| crate::store::help::red(w.clone(), s.more(key)))
}

pub(super) fn app_name(s: &Store, id: &str) -> String {
    s.apps.get(id).map(|a| a.name.clone())
        .or_else(|| crate::store::pages::installed::installed_name(s, id))
        .unwrap_or_else(|| id.to_string())
}

pub(crate) fn held_row_key(thing: &Thing) -> String {
    match thing.kind() {
        Kind::App => format!("held:{}", app_key(thing.id())),
        Kind::Part => format!("held:up:part:{}:{}", thing.id(), thing.branch()),
        Kind::Firmware => format!("held:fw:{}", thing.id()),
        Kind::Plugin => format!("held:pl:{}", thing.id()),
    }
}
