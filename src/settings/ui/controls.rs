//! Buttons, headlines and links that pages share.

use std::rc::Rc;

use super::{Kit, Msg, Page, card, line, on_kit};
use crate::settings::Settings;
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, even_grid, note,
                             row, space, Length, Node};

pub fn confirm<P: Page + 'static>(_kit: &Kit, key: &str, icon: &str, name: &'static str,
                                  f: impl Fn(&mut P, &mut Kit) + 'static) -> Node<Msg> {
    let f = Rc::new(f);
    crate::toolkit::widget::remove(icon, name).key(format!("twice:{key}")).on_press(Msg(Rc::new(move |s: &mut Settings| {
        let f = f.clone();
        s.with::<P>(move |p, kit| f(p, kit));
    })))
}

pub fn confirm_by_hand<P: Page + 'static>(_kit: &Kit, key: &str, icon: &str, name: &'static str, act: &'static str,
                                          f: impl Fn(&mut P, &mut Kit) + 'static) -> Node<Msg> {
    let f = Rc::new(f);
    crate::toolkit::widget::remove_by_hand(icon, name, act).key(format!("twice:{key}")).on_press(Msg(Rc::new(move |s: &mut Settings| {
        let f = f.clone();
        s.with::<P>(move |p, kit| f(p, kit));
    })))
}

pub fn job(kit: &Kit, key: &str, icon: &str, tip: &str, press: Msg) -> Node<Msg> {
    match kit.dials.get(key) {
        Some(d) => row(vec![d.node(), note(d.words()).tone(d.tone())]).gap(8.0).width(Length::Shrink),
        None => button(icon, tip).on_press(press),
    }
}

pub fn dropdown(_kit: &Kit, key: &str, options: Vec<String>, chosen: Option<usize>, tip: &str,
                choose: impl Fn(usize) -> Msg + 'static) -> Node<Msg> {
    crate::toolkit::widget::one_of(options, chosen, tip, choose).key(format!("dropdown:{key}"))
}

pub fn missing(kit: &Kit, what: &str, package: &str) -> Node<Msg> {
    card(None, vec![line(t!("settings-common-not-available", what = what)).sub(t!("settings-common-install-with", package = package)).node(kit)])
}

pub fn headline<M>(_kit: &Kit, words: impl Into<String>) -> Node<M> {
    let all = vec![crate::toolkit::widget::headline(words), space()];
    column(vec![row(all).gap(12.0).width(Length::Fill), space().height(Length::Fixed(HEADLINE_ROOM))]).gap(0.0)
}

const HEADLINE_ROOM: f32 = 12.0;

pub fn loading<M>() -> Node<M> {
    row(vec![space(), crate::toolkit::widget::spinner(), space()])
}

pub fn tiles<M>(min: f32, items: Vec<Node<M>>) -> Node<M> {
    even_grid(min, items).grows().height(Length::Fill)
}

pub fn help_link(words: &str, topic: &'static str) -> Node<Msg> {
    button("help-browser-symbolic", words).on_press(on_kit(move |k| k.go_to_help(topic)))
}

pub fn no_driver_card(kit: &Kit, class: crate::common::no_driver::Class) -> Option<Node<Msg>> {
    let lines = crate::common::no_driver::ready_lines(class);
    if lines.is_empty() {
        return None;
    }
    let rows = lines.into_iter()
        .map(|l| line(l).control(help_link(t!("common-no-driver-more"), crate::common::no_driver::HELP)).node(kit))
        .collect();
    Some(card(None, rows))
}

pub fn add_no_driver(kit: &Kit, items: &mut Vec<Node<Msg>>, class: crate::common::no_driver::Class) {
    if let Some(c) = no_driver_card(kit, class) {
        let at = items.len().min(1);
        items.insert(at, c);
    }
}

pub fn open_link(url: &str) -> bool {
    crate::toolkit::link::is_web(url) && crate::common::run::spawn(&["xdg-open", url])
}

pub fn no_browser() -> &'static str {
    t!("settings-common-no-browser")
}

pub fn link(key: impl Into<String>, url: impl Into<String>) -> Msg {
    let (key, url) = (key.into(), url.into());
    on_kit(move |k| {
        let opened = open_link(&url);
        k.started(&key, opened, no_browser());
    })
}

pub fn opens(key: impl Into<String>, program: &'static str, name: &'static str) -> Msg {
    let key = key.into();
    on_kit(move |k| {
        let started = crate::common::run::spawn(&[program]);
        k.started(&key, started, &t!("settings-common-could-not-open", name = name));
    })
}

pub fn in_words(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => t!("settings-common-and", rest = rest.join(t!("settings-common-list-comma")), last = last),
    }
}
