//! The shape every Store page shares.

use super::Msg;
use crate::toolkit::theme;
use crate::toolkit::tiles::tile_head;
use crate::toolkit::widget::{bento, card, column, heading, headline, page_title, row, scroll, space, Length, Node};

const CARD_LEAST: f32 = 300.0;

pub(super) const CARD_GAP: f32 = 16.0;

const HEADLINE_ROOM: f32 = 12.0;

pub(super) const HEADLINE_LINES: u8 = 2;
const HEADLINE_MOST: u8 = 3;

pub(super) const HEADLINE: &str = "apps-headline";

thread_local! {

    static PAGE: std::cell::RefCell<(String, f32)> = const { std::cell::RefCell::new((String::new(), 0.0)) };

    static LINES: std::cell::RefCell<Option<(crate::common::widest::Stamp, u32, u8)>> = const { std::cell::RefCell::new(None) };
}

pub(crate) fn new_page(name: String, across: f32) {
    PAGE.with(|p| *p.borrow_mut() = (name, across));
}

pub(super) fn across() -> f32 {
    PAGE.with(|p| p.borrow().1)
}

pub(super) fn headline_lines() -> u8 {
    let room = super::app_page::room(across());
    let now = crate::common::widest::stamp();
    LINES.with(|kept| {
        if let Some((had, at, lines)) = &*kept.borrow() {
            if *had == now && *at == room.to_bits() {
                return *lines;
            }
        }
        let need = super::update_page::words::longest_headlines().iter()
            .map(|w| crate::common::widest::headline_lines(w, room)).max().unwrap_or(0);
        let lines = u8::try_from(need).unwrap_or(u8::MAX).clamp(HEADLINE_LINES, HEADLINE_MOST);
        *kept.borrow_mut() = Some((now, room.to_bits(), lines));
        lines
    })
}

pub(crate) fn page_area() -> String {
    PAGE.with(|p| format!("page:{}", p.borrow().0))
}

pub fn screen(head: Node<Msg>, words: impl Into<String>, tiles: Vec<Node<Msg>>) -> Node<Msg> {
    let line = column(vec![headline(words).room_for(headline_lines()).width(Length::Fill).key(HEADLINE), space().height(Length::Fixed(HEADLINE_ROOM))]).gap(0.0);
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

pub fn tile_far(icon: &str, name: &str, far: Vec<Node<Msg>>, body: Vec<Node<Msg>>, (least, most): (u8, u8)) -> Node<Msg> {
    let mut all = vec![far_row(icon, name, far)];
    all.extend(body);
    card(column(all).gap(CARD_GAP)).height(Length::Fill).beside().span(least, most, theme::bento().words)
}

fn far_row(icon: &str, name: &str, far: Vec<Node<Msg>>) -> Node<Msg> {
    if far.is_empty() {
        return head(icon, name, Vec::new());
    }
    let mut top = vec![head(icon, name, Vec::new()).width(Length::Fill)];
    top.extend(far);
    row(top).gap(theme::sheet().gap).centred()
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
    let mut all = vec![far_row(icon, name, far)];
    all.extend(body);
    card(column(all).gap(CARD_GAP))
}

pub fn small() -> (u8, u8) {
    theme::bento().card
}

