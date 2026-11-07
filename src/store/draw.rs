//! Drawing the window.

use std::rc::Rc;

use crate::toolkit::dnd::Way;
use crate::toolkit::event::Flow;
use crate::toolkit::ui::App as Shows;
use crate::toolkit::widget::{card, column, drawing, entry, layers, note, row, scroll, Length, Node, Pointer, Tone};
use crate::toolkit::t;
use crate::common::filling::Dial;
use super::catalogue::categories;
use super::side::{side_width, Place};
use super::{app_page, browse, go, help, look, pages, plugin_page, screen, update_page, Do, Msg, Store, View};

impl Store {
    pub(super) fn moving(&self) -> bool {
        self.loading || self.dials.values().any(Dial::running)
            || (matches!(self.view, View::Hosted(_)) && self.hosted.as_ref().is_some_and(|h| h.moving()))
    }

    fn page_name(&self) -> String {
        match (&self.detail, &self.image, &self.view) {
            (None, None, View::Search(words)) if !self.awaiting() => format!("search:{words}"),
            _ => self.page_key(),
        }
    }

    pub(super) fn grid_title(&self) -> String {
        match &self.view {
            View::Category(code) => categories().iter().find(|(c, _, _)| c == code).map(|(_, t, _)| *t).unwrap_or("").to_string(),
            View::Search(text) => t!("store-grid-results", text = text),
            _ => t!("store-grid-discover").into(),
        }
    }

    fn page(&self) -> Node<Msg> {
        screen::new_page(self.page_name());
        if self.view == View::Plugins && self.detail.is_none() {
            return plugin_page::view(self);
        }
        if let (View::Hosted(_), None, Some(h)) = (&self.view, &self.detail, &self.hosted) {
            let lift: Rc<dyn Fn(crate::settings::ui::Msg) -> Msg> = Rc::new(|m: crate::settings::ui::Msg| go(move |s| {
                if let Some(h) = s.hosted.as_mut() {
                    h.update(m.clone());
                }
            }));
            return h.view().map(lift);
        }
        if self.awaiting() {
            return pages::grid::waiting_for_flathub(&self.grid_title(), self.failed.as_deref(), self.online);
        }
        if let Some(a) = &self.detail {
            return app_page::laid(&app_page::gather(self, a));
        }
        if let Some(f) = &self.image {
            return pages::image_page::laid(&pages::image_page::gather(self, f));
        }
        match &self.view {
            View::Home | View::Category(_) | View::Search(_) => pages::grid::grid(self, &self.grid_title()),
            View::Installed => pages::installed::installed(self),
            View::Updates => update_page::view::view(self),
            View::Plugins => plugin_page::view(self),
            View::Browse => browse::page(self),
            View::GitHub => pages::github_page::github(self),
            View::Backup => pages::backup_page::backup(self),
            View::Sources => pages::sources_page::sources(self),
            View::Hosted(_) => pages::grid::grid(self, t!("store-grid-apps")),
        }
    }

    pub(super) fn whole(&self, settings: Vec<Place>) -> Node<Msg> {
        self.pulsing.set(false);
        let mut top = vec![entry(self.search.clone(), t!("store-search-placeholder"), |t| go(move |s| s.typed(t.clone())))
                           .width(Length::Fill)];
        if let Some((words, tone)) = &self.top {
            top.push(if *tone == Tone::Problem {
                pages::links::red_about(words.clone(), help::INSTALL)
            } else {
                note(words.clone()).tone(*tone)
            });
        }
        let main = column(vec![row(top).gap(12.0).padding(12.0), self.page().height(Length::Fill).opacity(self.fade)])
            .width(Length::Fill).height(Length::Fill);
        self.keep_pulsing();
        let window = row(vec![card(scroll("side", self.side(settings)).padding(crate::toolkit::theme::sheet().sidebar_room)).snug().side()
                     .width(Length::Fixed(side_width())).height(Length::Fill), main]).gap(crate::toolkit::theme::sheet().gap)
            .on_drop_from_outside(
                |held| (held.offered.files && held.offered.ways.has(Way::Copy)).then_some(Way::Copy),
                |_, given, _| go(move |s| s.dropped(given.clone())));
        let mut over = pages::appimage_warning::over(self);
        if let Some(panel) = pages::github_key::key_box(self) {
            let outside = drawing((0.0, 0.0), "", |_, _| {}).width(Length::Fill).height(Length::Fill)
                .on_pointer(|p| match p {
                    Pointer::Down(..) | Pointer::Menu(..) | Pointer::Middle(..) => go(pages::github_key::close_key_box),
                    _ => go(|_| {}),
                });
            over.extend([outside, panel]);
        }
        if over.is_empty() {
            return window;
        }
        over.insert(0, window);
        layers(over)
    }

    fn keep_pulsing(&self) {
        if !self.pulsing.get() || self.pulse_timer.get() {
            return;
        }
        self.pulse_timer.set(true);
        let (pulsing, running, asks) = (self.pulsing.clone(), self.pulse_timer.clone(), self.asks.clone());
        self.events.every(look::PULSE_FRAME, move || {
            if pulsing.get() && asks.send(Do::Pulse) {
                return Flow::Continue;
            }
            running.set(false);
            Flow::Stop
        });
    }
}

impl Shows for Store {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        let was = self.page_key();
        (msg.0)(self);
        self.turned(&was);
    }

    fn pressed(&mut self, place: crate::toolkit::widget::Rect) {
        self.pressed_at = Some(place);
        if let Some(h) = self.hosted.as_mut() {
            h.pressed(place);
        }
    }

    fn intercept(&self, input: &crate::toolkit::input::Input, _keys: bool) -> Option<Msg> {
        match input {
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. }
                if pages::appimage_warning::showing(self) => Some(go(pages::appimage_warning::cancel)),
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. }
                if pages::github_key::key_box_open(self) => Some(go(pages::github_key::close_key_box)),
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. }
                if matches!(self.view, View::Hosted(_)) => Some(go(|s| {
                    if let Some(h) = s.hosted.as_mut() {
                        h.escape();
                    }
                })),
            _ => None,
        }
    }

    fn resized(&mut self, w: f32, h: f32) -> bool {
        let changed = self.across != w;
        self.across = w;
        let taller = (self.tall - h).abs() >= 1.0;
        self.tall = h;
        (changed && (self.detail.is_some() || self.image.is_some() || self.view == View::Installed)) || taller
    }

    fn view(&self) -> Node<Msg> {
        self.whole(self.settings_places())
    }
}
