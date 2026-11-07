//! The words under a row.

use crate::toolkit::widget::Tone;
use super::{help, pages, refusal, Msg, Store};

impl Store {
    pub(super) fn say(&mut self, key: &str, words: impl Into<String>, tone: Tone) {
        let (words, behind) = refusal::apart(&words.into());
        self.said.insert(key.to_string(), (words, tone));
        self.said_on.remove(key);
        self.behind_open.remove(key);
        if behind.is_empty() {
            self.behind.remove(key);
        } else {
            self.behind.insert(key.to_string(), behind);
        }
    }

    pub(super) fn say_on(&mut self, key: &str, words: impl Into<String>, page: &'static str) {
        self.say(key, words, Tone::Problem);
        self.said_on.insert(key.to_string(), page);
    }

    pub(super) fn more(&self, key: &str) -> Msg {
        let mut said = self.said.get(key).map(|(w, _)| w.clone()).unwrap_or_default();
        for line in self.behind.get(key).into_iter().flatten() {
            said.push('\n');
            said.push_str(line);
        }
        pages::links::more_about_text(self.page_of(key), said)
    }

    pub(super) fn page_of(&self, key: &str) -> &'static str {
        self.said_on.get(key).copied().unwrap_or_else(|| help::of_key(key))
    }

    pub(super) fn toggle_behind(&mut self, key: &str) {
        if !self.behind_open.remove(key) {
            self.behind_open.insert(key.to_string());
        }
    }
}
