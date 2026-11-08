//! The page: the room and the cards beside it, the list of programs and the words under the i.

use crate::toolkit::{popup, t, theme};
use crate::toolkit::tiles::{empty_state, error_state, loading_state, tile_head};
use crate::toolkit::widget::{bento, button, card, column, column_heading, headline, label, Length, Node, note, page_title, Pen, row, scroll, space};

use super::{Activity, Msg, orbit, rings, STOP_ICON};
use super::reading::icon_picture;

const CARD_WIDTH: f32 = 400.0;

const FACT_NAME: f32 = 104.0;

const HELP_WIDE: f32 = 460.0;

const CARD_LEAST: f32 = 300.0;

const HEADLINE_ROOM: f32 = 12.0;

const ROOM_LEAST: f32 = 280.0;

const LIST_ROWS: usize = 8;

const LIST_LEAST: f32 = 240.0;

const FIGURE_WIDE: f32 = 72.0;

pub(super) fn help() -> [&'static str; 4] {
    [t!("activity-help-1"), t!("activity-help-2"), t!("activity-help-3"), t!("activity-help-4")]
}

impl Activity {

    pub(super) fn page(&self) -> Node<Msg> {
        let b = theme::bento();
        let info = button("dialog-information-symbolic", t!("activity-about-page")).size(theme::sheet().info)
            .on(self.help).on_press(Msg::Help);
        let mut all = vec![
            row(vec![page_title(t!("activity-title")), info, space(), note(self.forced.clone())]).gap(12.0).centred(),
            self.headline(),
            self.room(),
            self.programs(),
        ];
        all.extend(self.rings.view());
        scroll("activity", bento(CARD_LEAST, b.page_gap, all).padding(theme::sheet().page_padding)).height(Length::Fill)
    }

    fn room(&self) -> Node<Msg> {
        let picture = if self.failed && !self.looked {
            let retry = button("view-refresh-symbolic", t!("activity-try-again")).on_press(Msg::Retry);
            error_state(t!("activity-cannot-read"), retry)
        } else if !self.looked {
            loading_state(ROOM_LEAST)
        } else if self.infos.is_empty() {
            empty_state(t!("activity-no-programs"), None)
        } else {
            let facts = self.orbit.picked().map(|key| self.card(&key));
            self.orbit.view(facts, Msg::Pointer, Msg::Keyed, Msg::Press).height(Length::Fill)
        };
        let b = theme::bento();
        card(column(vec![
            tile_head("computer-symbolic", t!("activity-right-now"), Vec::new()),
            note(t!("activity-caption")),
            picture,
        ]).gap(16.0)).height(Length::Fill).beside().span_grows(b.picture.0, b.picture.1, b.words)
    }

    fn programs(&self) -> Node<Msg> {
        let b = theme::bento();
        let body = if !self.looked && self.failed {
            empty_state(t!("activity-nothing-to-list"), None)
        } else if !self.looked {
            loading_state(LIST_LEAST)
        } else if self.order.is_empty() {
            empty_state(t!("activity-no-programs"), None)
        } else {
            let picked = self.orbit.picked();
            let rows: Vec<Node<Msg>> = self.order.iter().filter_map(|key| self.infos.get(key).map(|i| (key, i)))
                .take(LIST_ROWS).enumerate().map(|(n, (key, i))| {
                    let line = row(vec![
                        icon_picture(i.icon.clone(), 24.0),
                        label(i.title.clone()).cut().width(Length::Fill),
                        note(i.holding.0.clone()).width(Length::Fixed(FIGURE_WIDE)),
                        note(i.work.0.clone()).width(Length::Fixed(FIGURE_WIDE)),
                    ]).gap(12.0).centred();
                    let shown = picked.as_deref() == Some(key.as_str());
                    let line = if shown {
                        card(line).snug()
                    } else if n % 2 == 1 {
                        line.banded()
                    } else {
                        card(line).snug().flat()
                    };
                    line.on_press(Msg::Pick(key.clone()))
                }).collect();

            let head = row(vec![
                space().width(Length::Fixed(24.0)),
                column_heading(t!("activity-column-program")).width(Length::Fill),
                column_heading(t!("activity-memory")).width(Length::Fixed(FIGURE_WIDE)),
                column_heading(t!("activity-processor")).width(Length::Fixed(FIGURE_WIDE)),
            ]).gap(12.0).centred();
            let mut all = vec![card(head).snug().flat()];
            all.extend(rows);
            column(all).gap(0.0)
        };
        card(column(vec![
            tile_head("view-list-symbolic", t!("activity-programs"), Vec::new()),
            note(t!("activity-list-caption")),
            body,
        ]).gap(16.0)).height(Length::Fill).beside().span_grows(b.card.0, b.card.0 + 1, b.words)
    }

    pub(super) fn help_panel(&self) -> Node<Msg> {
        let at = self.help_at;
        let words = column(help().iter().map(|p| note(*p).width(Length::Fill)).collect()).gap(12.0);
        let panel = card(words).floats().width(Length::Fixed(HELP_WIDE));
        popup::under_the_i(panel, at)
    }

    fn headline(&self) -> Node<Msg> {
        let (words, seen) = if self.failed && !self.looked {

            (&self.unread, 1.0)
        } else if self.fade < 0.5 {
            (&self.was, 1.0 - 2.0 * self.fade)
        } else {
            (&self.said, 2.0 * self.fade - 1.0)
        };
        column(vec![headline(words.clone()).opacity(seen).width(Length::Fill), space().height(Length::Fixed(HEADLINE_ROOM))])
            .gap(0.0)
    }

    fn card(&self, key: &str) -> Node<Msg> {
        let close = button("window-close-symbolic", t!("activity-close")).on_press(Msg::Close);
        let Some(i) = self.infos.get(key) else {
            let line = vec![label(t!("activity-closed")).width(Length::Fill), close];
            return card(row(line).gap(12.0)).width(Length::Fixed(CARD_WIDTH));
        };
        let picture = icon_picture(i.icon.clone(), 44.0);

        let heat = i.heat as f32;
        let working = i.working;
        let mut status = vec![note(i.effort)];
        if working {
            status.insert(0, rings::dot(10.0, move |_: &Pen| orbit::paint::heat_colour(heat)));
        }
        let mut head = vec![
            picture,
            column(vec![label(i.title.clone()), row(status).gap(8.0).centred()]).gap(2.0).width(Length::Fill),
        ];

        if i.can_stop {
            let tip = if self.armed { t!("activity-stop-again", title = &i.title) } else { t!("activity-stop", title = &i.title) };
            let stop = button(STOP_ICON, tip).key("activity:stop").on_press(Msg::Stop);
            head.push(if self.armed { stop.filled() } else { stop });
        }
        head.push(close);
        let parts = if i.parts == 1 { t!("activity-one-part").to_string() } else { t!("activity-separate-parts", n = i.parts) };
        let facts = [
            (t!("activity-processor"), t!("activity-two-parts", a = &i.work.0, b = &i.work.1)),
            (t!("activity-memory"), t!("activity-two-parts", a = &i.holding.0, b = &i.holding.1)),
            (t!("activity-running-for"), i.running.clone()),
            (t!("activity-made-of"), parts),
        ];
        let lines: Vec<Node<Msg>> = facts.into_iter().enumerate().map(|(n, (name, value))| {
            let line = row(vec![note(name).width(Length::Fixed(FACT_NAME)), label(value).width(Length::Fill)]).gap(12.0).centred();
            if n % 2 == 1 { line.banded() } else { card(line).snug().flat() }
        }).collect();
        card(column(vec![row(head).gap(12.0), column(lines).gap(0.0)]).gap(12.0)).width(Length::Fixed(CARD_WIDTH))
    }
}
