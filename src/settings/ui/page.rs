//! A whole Settings page.

use std::rc::Rc;

use super::{Kit, Msg, group, on_kit};
use crate::settings::Settings;
use crate::toolkit::t;
use crate::toolkit::widget::{bento, button, card as boxed, column, note, page_title,
                             row, scroll, space, split, Length, Node, Start};

const CARD_MIN: f32 = 340.0;
const QUIET_ROOM: u8 = 6;

fn toggle_info(key: String, words: String) -> Msg {
    on_kit(move |k| {
        if k.info.as_ref().is_some_and(|(open, _, _)| *open == key) {
            k.info = None;
        } else {
            let at = k.pressed_at.unwrap_or_default();
            k.info = Some((key.clone(), words.clone(), at));
        }
    })
}

pub(super) fn info(kit: &Kit, key: &str, tip: &str, words: &str) -> Node<Msg> {
    let open = kit.info.as_ref().is_some_and(|(k, _, _)| k == key);
    button("dialog-information-symbolic", tip).size(crate::toolkit::theme::sheet().info).on(open)
        .on_press(toggle_info(key.to_string(), words.to_string()))
}

pub(crate) fn with_info(kit: &Kit, page: Node<Msg>) -> Node<Msg> {
    const WIDE: f32 = 360.0;
    match kit.info.clone() {
        Some((_, words, at)) => {
            let panel = boxed(note(words).width(Length::Fill)).floats().width(Length::Fixed(WIDE));
            crate::toolkit::popup::dismissing(page, crate::toolkit::popup::under_the_i(panel, at), on_kit(|k| k.info = None), Msg(Rc::new(|_: &mut Settings| ())))
        }
        None => page,
    }
}

pub fn page(kit: &Kit, heading: &str, help: &str, items: Vec<Node<Msg>>) -> Node<Msg> {
    page_with(kit, heading, help, Vec::new(), items)
}

pub fn page_with(kit: &Kit, heading: &str, help: &str, end: Vec<Node<Msg>>, items: Vec<Node<Msg>>) -> Node<Msg> {
    let mut head = vec![page_title(heading)];
    if !help.is_empty() {
        head.push(info(kit, &format!("page:{heading}"), t!("settings-common-about-page"), help));
    }
    head.push(space());
    head.extend(end);
    let mut all = vec![row(head).gap(12.0)];
    let sizes = crate::toolkit::theme::bento();
    let alone = items.iter().filter(|i| i.beside).count() == 1
        && !items.iter().any(|i| i.beside && matches!(i.what, crate::toolkit::widget::What::Grid { .. }));
    for item in items {
        let card = item.beside;
        all.push(item);
        if alone && card {
            all.push(space().beside().span(QUIET_ROOM, QUIET_ROOM, sizes.words));
        }
    }
    scroll(format!("page:{heading}"), bento(CARD_MIN, sizes.page_gap, all).padding(crate::toolkit::theme::sheet().page_padding)).height(Length::Fill)
}

pub fn page_split(kit: &Kit, heading: &str, help: &str, top: Vec<Node<Msg>>, left: Vec<Node<Msg>>, right: Vec<Node<Msg>>) -> Node<Msg> {
    let mut head = vec![page_title(heading)];
    if !help.is_empty() {
        head.push(info(kit, &format!("page:{heading}"), t!("settings-common-about-page"), help));
    }
    head.push(space());
    let sizes = crate::toolkit::theme::bento();
    let mut over = vec![row(head).gap(12.0)];
    over.extend(top);
    let mut all = vec![scroll(format!("page:{heading}:head"), column(over).gap(sizes.page_gap)).height(Length::Shrink)];
    let stacked = right.into_iter().map(|card| card.height(Length::Shrink)).collect();
    let side = |key: String, items: Vec<Node<Msg>>| scroll(key, column(items).gap(sizes.page_gap)).width(Length::Fill);
    let columns = split(format!("page:{heading}:split"), side(format!("page:{heading}"), left), side(format!("page:{heading}:right"), stacked))
        .start(Start::Part(SPLIT_LIST));
    all.push(columns.height(Length::Fill));
    column(all).gap(sizes.page_gap).padding(crate::toolkit::theme::sheet().page_padding).height(Length::Fill)
}

const SPLIT_LIST: f32 = 0.4;

thread_local! {
    pub(super) static PAGE_ICON: std::cell::Cell<&'static str> = const { std::cell::Cell::new("preferences-system-symbolic") };
}

pub(crate) fn new_page(icon: &'static str) {
    PAGE_ICON.with(|i| i.set(icon));
}

pub fn card(heading: Option<&str>, rows: Vec<Node<Msg>>) -> Node<Msg> {
    group(heading.unwrap_or("")).rows(rows).build(None)
}
