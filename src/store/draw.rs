//! Drawing the window: the side bar beside the page, the page for each view, and the toolkit's hooks.

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
        self.dials.values().any(Dial::running) || self.hosted_moving()
    }

    fn hosted_moving(&self) -> bool {
        matches!(self.view, View::Hosted(_)) && self.hosted.as_ref().is_some_and(|h| h.moving())
    }

    pub(super) fn turn(&self) -> bool {
        let moved = self.dials.values().fold(false, |m, d| d.turn() | m);
        update_page::words::overall_ring(self);
        let hosted = self.hosted_moving() && self.hosted.as_ref().is_some_and(|h| h.turn());
        self.turning.turned(moved) | hosted
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
        screen::new_page(self.page_name(), self.size.across);
        if self.view == View::Plugins && self.detail.is_none() {
            return plugin_page::view(self);
        }
        if let (View::Hosted(_), None, Some(h)) = (&self.view, &self.detail, &self.hosted) {
            return h.view().map(lift_hosted());
        }
        if let Some(page) = pages::flatpak_page::view(self) {
            return page;
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
        self.pulse.on.set(false);
        let mut top = vec![entry(self.search.clone(), t!("store-search-placeholder"), |t| go(move |s| s.typed(t.clone())))
                           .width(Length::Fill)];
        if let Some((words, tone)) = self.top.as_ref().filter(|(w, _)| !crate::common::refused::closed_box(w)) {
            top.push(if *tone == Tone::Problem {
                pages::links::red_about(words.clone(), help::INSTALL)
            } else {
                note(words.clone()).tone(*tone)
            });
        }
        let main = column(vec![row(top).gap(12.0).padding(12.0), self.page().height(Length::Fill).opacity(self.fade_in.shown)])
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

        if let (View::Hosted(_), None, Some((panel, close))) = (&self.view, &self.detail, self.hosted.as_ref().and_then(|h| h.info_box())) {
            let lift = lift_hosted();
            let shut = lift(close);
            let outside = drawing((0.0, 0.0), "", |_, _| {}).width(Length::Fill).height(Length::Fill)
                .on_pointer(move |p| match p {
                    Pointer::Down(..) | Pointer::Menu(..) | Pointer::Middle(..) => shut.clone(),
                    _ => go(|_| {}),
                });
            over.extend([outside, panel.map(lift)]);
        }
        if over.is_empty() {
            return window;
        }
        over.insert(0, window);
        layers(over)
    }

    fn keep_pulsing(&self) {
        if !self.pulse.on.get() || self.pulse.timer.get() {
            return;
        }
        self.pulse.timer.set(true);
        let (pulsing, running, asks) = (self.pulse.on.clone(), self.pulse.timer.clone(), self.asks.clone());
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
                if pages::flatpak_page::showing(self) => Some(go(pages::flatpak_page::cancel_first)),
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. }
                if pages::github_key::key_box_open(self) => Some(go(pages::github_key::close_key_box)),
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. }
                if matches!(self.view, View::Hosted(_)) => Some(go(|s| {
                    if let Some(h) = s.hosted.as_mut() {
                        h.escape();
                    }
                })),

            crate::toolkit::input::Input::Focus(false)
                if self.hosted.as_ref().is_some_and(|h| h.info_box().is_some()) => Some(go(|s| {
                    if let Some(h) = s.hosted.as_mut() {
                        h.escape();
                    }
                })),
            _ => None,
        }
    }

    fn resized(&mut self, w: f32, h: f32) -> bool {
        let changed = self.size.across != w;
        self.size.across = w;

        let taller = (self.size.tall - h).abs() >= 1.0;
        self.size.tall = h;

        (changed && (self.detail.is_some() || self.image.is_some() || self.view == View::Installed)) || taller
    }

    fn view(&self) -> Node<Msg> {
        self.whole(self.settings_places())
    }
}

fn lift_hosted() -> Rc<dyn Fn(crate::settings::ui::Msg) -> Msg> {
    Rc::new(|m: crate::settings::ui::Msg| go(move |s| {
        if let Some(h) = s.hosted.as_mut() {
            h.update(m.clone());
        }
    }))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::store::catalogue::App;

    fn store() -> Store {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: Do| {});
        Store::new(&events, asks, None)
    }

    #[test]
    fn a_hosted_pages_i_box_sits_under_its_i_and_shuts_on_leaving() {
        let mut s = store();
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut h = crate::settings::Hosted::new(&events, asks, None, None);
        h.show("startup");
        let at = crate::toolkit::widget::Rect { x: 600.0, y: 300.0, w: 30.0, h: 30.0 };
        h.open_info(at);
        s.hosted = Some(h);
        s.view = View::Hosted("startup".into());
        let mut ui = crate::toolkit::ui::Ui::new(Shows::view(&s), crate::toolkit::theme::Theme::new(true, "#fbc711"));
        ui.lay_at(1400, 900);
        let gap = crate::toolkit::theme::gaps().medium;
        let found = ui.boxes().into_iter().any(|(_, r, _)| (r.w - 360.0).abs() < 1.0 && (r.x + r.w / 2.0 - 615.0).abs() < 1.0 && (r.y - (330.0 + gap)).abs() < 1.0);
        assert!(found, "no 360 wide box centred a gap under the i: {:?}", ui.boxes().into_iter().filter(|(_, r, _)| (r.w - 360.0).abs() < 1.0).collect::<Vec<_>>());
        assert!(Shows::intercept(&s, &crate::toolkit::input::Input::Focus(false), false).is_some(), "leaving the window does not shut it");
        s.hosted.as_mut().unwrap().show("default-apps");
        assert!(s.hosted.as_ref().unwrap().info_box().is_none(), "going to another page leaves it open");
    }

    #[test]
    fn a_turning_dial_asks_for_the_page_at_most_once_a_second() {
        let mut s = store();
        let app = App { id: "org.example.Paint".into(), ..Default::default() };
        let mut d = Dial::default();
        d.expect(1.0);
        s.dials.insert(app.id.clone(), d);
        let app_page::State::Fetching(ring) = app_page::gather(&s, &app).state else { panic!("the app is being fetched") };
        let was = ring.get();
        let mut built = 0;
        for _ in 0..12 {
            std::thread::sleep(Duration::from_millis(40));
            built += usize::from(s.moving() && s.turn());
        }
        assert!(ring.get() > was, "the page's ring followed the dial");
        assert!((1..=3).contains(&built), "the view was asked for {built} times in 12 turns");
        s.dials.get_mut(&app.id).unwrap().finish(true, "");
        assert!(!s.moving(), "a finished job turns nothing");
    }

    #[test]
    fn loading_alone_keeps_no_clock() {
        let mut s = store();
        s.loading = true;
        assert!(!s.moving());
    }
}
