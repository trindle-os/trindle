//! Which page shows.

use crate::toolkit::motion::{tween, Ease};
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use super::catalogue::App;
use super::{browse, pages, plugin_page, reviews, update_page, Do, Store, View, FADE_MS, PAGE};

impl Store {
    pub(super) fn page_key(&self) -> String {
        match (&self.detail, &self.image, &self.view) {
            _ if self.awaiting() => "loading".into(),
            (Some(a), _, _) => format!("app:{}", a.id),
            (None, Some(f), _) => format!("image:{}", pages::image_tile::key_of(f)),
            (None, None, View::Search(_)) => "search".into(),
            (None, None, v) => v.nav_key(),
        }
    }

    fn needs_catalogue(&self) -> bool {
        self.detail.is_some() || matches!(self.view, View::Home | View::Category(_) | View::Search(_))
    }

    pub(super) fn awaiting(&self) -> bool {
        self.apps.is_empty() && self.needs_catalogue()
    }

    pub(super) fn turned(&mut self, was: &str) {
        if self.page_key() == was {
            return;
        }
        if let Some(t) = self.fading.take() {
            t.stop();
        }
        if !crate::common::graphics::moving() {
            self.fade = 1.0;
            return;
        }
        self.fade = 0.0;
        let asks = self.asks.clone();
        self.fading = Some(tween(&self.events, FADE_MS, Ease::Out, move |t| { asks.send(Do::Fade(t)); }, || {}));
    }

    pub(super) fn show_pending_app(&mut self) {
        if self.apps.is_empty() {
            return;
        }
        let Some(id) = self.pending_app.take() else { return };
        match self.apps.get(&id).cloned() {
            Some(a) => self.open_detail(a),
            None => self.top = Some((t!("store-not-in-catalogue", id = id), Tone::Problem)),
        }
    }

    fn hosted_key(&self, key: &str) -> Option<String> {
        let id = key.strip_prefix("set:").unwrap_or(key);
        let id = if id == "your-plugins" { "plugins" } else { id };
        if key == "plugins" {
            return None;
        }
        self.hosted.as_ref().filter(|h| h.has(id)).map(|_| id.to_string())
    }

    pub(super) fn select_nav(&mut self, key: &str) {
        let hosted = self.hosted_key(key);
        if matches!(self.view, View::Hosted(_)) {
            if let Some(h) = self.hosted.as_mut() {
                h.hide();
            }
        }
        self.search.clear();
        self.detail = None;
        self.image = None;
        self.headline = None;
        if let Some(id) = hosted {
            self.show_hosted(id);
            return;
        }
        self.shown = PAGE;
        if key != "updates" {
            self.pending_spot = None;
        }
        self.view = self.opened_view(key);
    }

    fn show_hosted(&mut self, id: String) {
        self.pending_spot = None;
        if let Some(h) = self.hosted.as_mut() {
            h.show(&id);
        }
        self.view = View::Hosted(id);
    }

    fn opened_view(&mut self, key: &str) -> View {
        match key {
            "home" => {
                self.want_listed();
                View::Home
            }
            "installed" => {
                pages::installed::installed_opened(self);
                plugin_page::opened_by_itself(self);
                View::Installed
            }
            "updates" => {
                update_page::check::opened(self);
                View::Updates
            }
            "plugins" => {
                plugin_page::opened(self);
                View::Plugins
            }
            "browse" => {
                browse::opened(self);
                View::Browse
            }
            "github" => {
                pages::github_search::github_opened(self);
                View::GitHub
            }
            "backup" => {
                pages::backup_page::backup_opened(self);
                View::Backup
            }
            "sources" => {
                pages::sources_page::sources_opened(self);
                View::Sources
            }
            k => {
                self.want_listed();
                View::Category(k.strip_prefix("cat:").unwrap_or(k).to_string())
            }
        }
    }

    pub(super) fn want_listed(&mut self) {
        if self.show.appimages() {
            browse::want_hub(self);
        }
    }

    pub(super) fn search_for(&mut self, words: &str) {
        self.typed(words.to_string());
    }

    pub(super) fn typed(&mut self, text: String) {
        self.search = text;
        self.detail = None;
        self.image = None;
        self.shown = PAGE;
        let words = self.search.trim().to_string();
        if !words.is_empty() {
            self.view = View::Search(words);
            browse::want_hub(self);
            browse::fetch_icons(self);
        } else if matches!(self.view, View::Search(_)) {
            self.view = View::Home;
        }
    }

    pub(super) fn open_detail(&mut self, a: App) {
        self.headline = None;
        self.image = None;
        if !self.reviews.contains_key(&a.id) {
            self.reviews.insert(a.id.clone(), None);
            let id = a.id.clone();
            reviews::ratings(&a.id, self.then(move |s: &mut Store, r| {
                s.reviews_heard.insert(id.clone());
                s.reviews.insert(id, r);
            }));
            let id = a.id.clone();
            reviews::reviews(&a.id, self.then(move |s: &mut Store, list| {
                s.said_about.insert(id, list);
            }));
        }
        pages::pictures::fetch_pictures(self, &a);
        pages::install::fetch_facts(self, &a);
        pages::github_search::look_up(self, &a.repo);
        self.shot = 0;
        self.detail = Some(a);
    }
}
