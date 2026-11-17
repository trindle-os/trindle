//! Buttons, headlines and links that pages share: the button pressed twice, jobs, dropdowns and the ways out to a browser.

use std::rc::Rc;

use super::{Kit, Msg, Page, card, line, on_kit};
use crate::settings::Settings;
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, even_grid, note,
                             row, space, Length, Node};

pub fn confirm<P: Page + 'static>(key: &str, icon: &str, name: &'static str,
                                  f: impl Fn(&mut P, &mut Kit) + 'static) -> Node<Msg> {
    let f = Rc::new(f);
    crate::toolkit::widget::remove(icon, name).key(format!("twice:{key}")).on_press(Msg(Rc::new(move |s: &mut Settings| {
        let f = f.clone();
        s.with::<P>(move |p, kit| f(p, kit));
    })))
}

pub fn confirm_by_hand<P: Page + 'static>(key: &str, icon: &str, name: &'static str, act: &'static str,
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

pub fn dropdown(key: &str, options: Vec<String>, chosen: Option<usize>, tip: &str,
                choose: impl Fn(usize) -> Msg + 'static) -> Node<Msg> {
    dropdown_beside(key, options, chosen, tip, 0.0, choose)
}

pub fn dropdown_beside(key: &str, options: Vec<String>, chosen: Option<usize>, tip: &str, beside: f32,
                       choose: impl Fn(usize) -> Msg + 'static) -> Node<Msg> {
    let chosen = chosen.filter(|&i| i < options.len());
    let strip = chosen.is_some() && crate::toolkit::widget::is_short(&options) && strip_width(&options) <= STRIP_MOST - beside;
    let node = match chosen {
        Some(at) if strip => crate::toolkit::widget::choice(options, at, tip, choose),
        _ => crate::toolkit::widget::dropdown(options, chosen, tip, choose),
    };
    node.key(format!("dropdown:{key}"))
}

pub const STRIP_MOST: f32 = 260.0;

pub const LINE_MOST: f32 = 276.0;

pub fn strip_width(options: &[String]) -> f32 {
    crate::common::widest::strip_kept(options)
}

pub fn place_or_extra(names: &mut Vec<String>, found: Option<usize>, saved: &str) -> usize {
    found.unwrap_or_else(|| {
        names.push(saved.to_string());
        names.len() - 1
    })
}

pub fn missing(kit: &Kit, what: &str, package: &str) -> Node<Msg> {
    card(None, vec![line(t!("settings-common-not-available", what = what)).sub(t!("settings-common-install-with", package = package)).node(kit)])
}

pub fn headline<M>(words: impl Into<String>) -> Node<M> {
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

pub fn in_words_or(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => t!("settings-common-or", rest = rest.join(t!("settings-common-list-comma")), last = last),
    }
}

#[cfg(test)]
mod place_tests {
    use super::{in_words_or, place_or_extra};

    #[test]
    fn a_list_of_choices_is_joined_with_or() {
        let names = |l: &[&str]| l.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(in_words_or(&names(&["Web"])), "Web");
        assert_eq!(in_words_or(&names(&["Web", "Mail"])), "Web or Mail");
        assert_eq!(in_words_or(&names(&["Web", "Mail", "Music and video"])), "Web, Mail or Music and video");
    }

    #[test]
    fn a_saved_value_that_is_not_listed_is_shown_as_an_extra_choice_not_as_the_first() {
        let mut names = vec!["Never".to_string(), "Sleep".to_string()];
        assert_eq!(place_or_extra(&mut names, Some(1), "sleep"), 1);
        assert_eq!(names.len(), 2, "a listed value added nothing");
        assert_eq!(place_or_extra(&mut names, None, "hibernate"), 2);
        assert_eq!(names, ["Never", "Sleep", "hibernate"]);
    }
}
