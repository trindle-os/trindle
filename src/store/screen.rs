//! The shape every Store page shares.

use super::Msg;
use crate::toolkit::theme;
use crate::toolkit::tiles::tile_head;
use crate::toolkit::widget::{bento, card, column, heading, headline, page_title, row, scroll, space, Length, Node};

const CARD_LEAST: f32 = 300.0;
pub(super) const CARD_GAP: f32 = 16.0;
const HEADLINE_ROOM: f32 = 12.0;
pub(super) const HEADLINE_LINES: u8 = 2;
pub(super) const HEADLINE: &str = "apps-headline";

thread_local! {
    static PAGE: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

pub(crate) fn new_page(name: String) {
    PAGE.with(|p| *p.borrow_mut() = name);
}

pub(crate) fn page_area() -> String {
    PAGE.with(|p| format!("page:{}", p.borrow()))
}

pub fn screen(head: Node<Msg>, words: impl Into<String>, tiles: Vec<Node<Msg>>) -> Node<Msg> {
    let line = column(vec![headline(words).room_for(HEADLINE_LINES).width(Length::Fill).key(HEADLINE), space().height(Length::Fixed(HEADLINE_ROOM))]).gap(0.0);
    let mut all = vec![head, line];
    all.extend(tiles);
    scroll(page_area(), bento(CARD_LEAST, theme::bento().page_gap, all).padding(theme::sheet().page_padding)).height(Length::Fill)
}

pub fn title_row(name: impl Into<String>, end: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = vec![page_title(name), space()];
    all.extend(end);
    row(all).gap(12.0).centred()
}

fn head(icon: &str, name: &str, end: Vec<Node<Msg>>) -> Node<Msg> {
    let k = theme::sheet();
    tile_head(icon, name, end).height(Length::Fixed(k.badge.max(k.spinner)))
}

pub fn words_card(name: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = vec![heading(name)];
    all.extend(body);
    card(column(all).gap(CARD_GAP))
}

pub fn tile(icon: &str, name: &str, end: Vec<Node<Msg>>, body: Vec<Node<Msg>>, (least, most): (u8, u8)) -> Node<Msg> {
    let mut all = vec![head(icon, name, end)];
    all.extend(body);
    card(column(all).gap(CARD_GAP)).height(Length::Fill).beside().span(least, most, theme::bento().words)
}

pub fn rows_tile(icon: &str, name: &str, end: Vec<Node<Msg>>, body: Vec<Node<Msg>>, (least, most): (u8, u8)) -> Node<Msg> {
    let mut all = vec![head(icon, name, end)];
    all.extend(body);
    card(column(all).gap(CARD_GAP)).height(Length::Fill).beside().span(least, most, theme::apps_list().card_least)
}

pub fn list_card(icon: &str, name: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    list_card_with(icon, name, Vec::new(), body)
}

pub fn list_card_with(icon: &str, name: &str, far: Vec<Node<Msg>>, body: Vec<Node<Msg>>) -> Node<Msg> {
    let top = if far.is_empty() {
        head(icon, name, Vec::new())
    } else {
        let mut top = vec![head(icon, name, Vec::new()).width(Length::Fill)];
        top.extend(far);
        row(top).gap(theme::sheet().gap).centred()
    };
    let mut all = vec![top];
    all.extend(body);
    card(column(all).gap(CARD_GAP))
}

pub fn small() -> (u8, u8) {
    theme::bento().card
}
