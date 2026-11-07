//! The clipboard panel.

use crate::common::clipboard;
use crate::ring::ctx::{Msg, msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, entry, Length, Node, remove, row};

use super::{banded, Body, line_of, m, Quick, ROW_GAP, Well};

pub(super) const TRASH_ICON: &str = "user-trash-symbolic";

const PIN_ICON: &str = "view-pin-symbolic";

pub(super) fn pin_button(pinned: bool, toggle: Msg) -> Node<Msg> {
    button(PIN_ICON, if pinned { unpin() } else { pin_ask() }).on(pinned).on_press(toggle).well()
}

fn pin_ask() -> &'static str {
    t!("ring-quick-pin")
}

fn unpin() -> &'static str {
    t!("ring-quick-unpin")
}

pub(super) fn forget_one() -> &'static str {
    t!("ring-quick-forget-one")
}

fn forget_all() -> &'static str {
    t!("ring-quick-forget-all")
}

impl Quick {
    pub(super) fn clipboard(&mut self) {
        let items = clipboard::list();
        if items.is_empty() {
            let reset = crate::common::damaged::take_notes().join(" ");
            self.say(if reset.is_empty() { t!("ring-quick-clip-empty") } else { &reset });
            return;
        }
        self.body = Body::Clip(items, String::new());
    }

    pub(super) fn clip_view(&self, items: &[clipboard::Item], filter: &str) -> Vec<Node<Msg>> {
        let want = filter.to_lowercase();
        let rows: Vec<Node<Msg>> = items.iter().filter_map(|item| {
            let (words, dim) = match &item.picture {
                Some(_) => (t!("ring-quick-clip-picture").to_string(), true),
                None => (item.text.split_whitespace().collect::<Vec<_>>().join(" "), false),
            };
            if !words.to_lowercase().contains(&want) {
                return None;
            }
            let (id, pinned) = (item.id, item.pinned);
            let copy = |item: clipboard::Item| msg(move |face| {
                clipboard::copy(&item);
                face.close();
            });
            let line = card(line_of(words, dim, 400)).flat().width(Length::Fill).on_press(copy(item.clone()));
            let line = if item.picture.is_none() { line.whole(item.text.replace('\r', "")) } else { line };
            Some(row(vec![
                line,
                button("edit-copy-symbolic", t!("ring-quick-clip-again")).on_press(copy(item.clone())).well(),
                pin_button(pinned, m(move |q| q.clip_pin(id))),
                remove(TRASH_ICON, forget_one()).key(format!("forget-clip-{id}")).on_press(m(move |q| q.clip_forget(id))).well(),
            ]).gap(ROW_GAP))
        }).collect();
        let rows = banded(rows);
        vec![
            entry(filter, t!("ring-quick-clip-filter"), |t| m(move |q| q.clip_filter(&t))).grab(self.turn),
            self.scrolling(rows),
            row(vec![self.forget_all_button(|q| {
                clipboard::clear();
                q.refresh_now();
            })]).width(Length::Shrink),
        ]
    }

    fn clip_filter(&mut self, text: &str) {
        if let Body::Clip(_, filter) = &mut self.body {
            *filter = text.to_string();
        }
    }

    fn clip_pin(&mut self, id: u64) {
        if let Body::Clip(items, _) = &mut self.body {
            if let Some(i) = items.iter_mut().find(|i| i.id == id) {
                i.pinned = !i.pinned;
                clipboard::set_pinned(id, i.pinned);
            }
        }
    }

    fn clip_forget(&mut self, id: u64) {
        clipboard::remove(id);
        if let Body::Clip(items, _) = &mut self.body {
            items.retain(|i| i.id != id);
        }
    }

    pub(super) fn forget_all_button(&self, clear: impl Fn(&mut Quick) + 'static) -> Node<Msg> {
        remove(TRASH_ICON, forget_all()).key("forget-all").on_press(m(move |q| clear(q))).well()
    }
}
