//! Spell checking: which dictionary is wanted, the words marked and the words added.

use std::cell::RefCell;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use crate::toolkit::editing::editor::Did;
use crate::toolkit::editing::{doc, editor};
use crate::toolkit::t;
use super::{Bar, Do, Spelling, Text, View, dictionaries, spell, store};

impl Text {

    fn wanted_lang(&self) -> Option<String> {
        if !self.settings.spell {
            return None;
        }
        spell::pick(&dictionaries(), &self.settings.lang, spell::system_lang().as_deref())
    }

    pub(super) fn spell_setup(&mut self) {
        let Some(lang) = self.wanted_lang() else {
            self.loading = None;
            if self.speller.take().is_some() {
                self.recheck();
            }
            return;
        };
        if self.speller.as_ref().is_some_and(|s| s.borrow().lang == lang) {
            self.loading = None;
            return;
        }
        if self.loading.as_deref() == Some(lang.as_str()) {
            return;
        }
        self.loading = Some(lang.clone());
        let asks = self.asks.clone();
        std::thread::spawn(move || {
            let read = spell::Speller::load(Path::new(spell::FOLDER), &lang, &store::words_file()).map(Box::new);
            asks.send(Do::Speller(lang, read));
        });
    }

    pub(super) fn speller_read(&mut self, lang: String, read: Result<Box<spell::Speller>, String>) {
        if self.loading.as_deref() != Some(lang.as_str()) {
            return;
        }
        self.loading = None;
        match read {
            Ok(speller) => {
                self.speller = Some(Rc::new(RefCell::new(*speller)));
                self.recheck();
            }
            Err(e) => self.fail(e),
        }
    }

    pub(super) fn checker(&self) -> Option<editor::Checker> {
        let speller = self.speller.clone()?;
        Some(Rc::new(move |word: &str| speller.borrow().right(word)))
    }

    fn recheck(&mut self) {
        for t in &self.tabs {
            let mut pane = t.pane.borrow_mut();
            pane.checker = self.checker();
            pane.checked += 1;
        }
    }

    pub(super) fn spell_bar(&mut self, range: Range<usize>) {
        let Some(speller) = self.speller.clone() else { return };
        let (word, version) = {
            let pane = self.pane().borrow();
            (pane.doc.text()[range.clone()].to_string(), pane.doc.version)
        };
        let offers = speller.borrow().suggest(&word);
        self.spelling = Some(Spelling { tab: self.tabs[self.active].id, range, word, offers, pick: 0, version, added: false });
        self.bar = Bar::Spell;
        self.view = View::Edit;
        self.found = None;
        self.focus_bar();
    }

    fn spelt(&self) -> Option<&Spelling> {
        let s = self.spelling.as_ref().filter(|s| self.bar == Bar::Spell && s.tab == self.tabs[self.active].id)?;
        let pane = self.pane().borrow();
        (pane.doc.version == s.version && pane.doc.text().get(s.range.clone()) == Some(s.word.as_str())).then_some(s)
    }

    pub(super) fn use_word(&mut self) {
        let Some((range, with)) = self.spelt().and_then(|s| Some((s.range.clone(), s.offers.get(s.pick)?.clone()))) else {
            return self.close_bar();
        };
        {
            let mut pane = self.pane().borrow_mut();
            pane.doc.replace(range, &with, doc::Kind::Other);
            pane.show_caret();
        }
        self.edited(Did::Edited);
        self.close_bar();
    }

    pub(super) fn add_word(&mut self) {
        let (Some(word), Some(speller)) = (self.spelling.as_ref().map(|s| s.word.clone()), self.speller.clone()) else { return };
        let added = speller.borrow_mut().add(&word, &store::words_file());
        match added {
            Ok(()) => {
                if let Some(s) = self.spelling.as_mut() {
                    s.added = true;
                }
                self.recheck();
                self.noted_to("spell");
            }
            Err(e) => self.fail(t!("text-word-not-added", word = word, reason = e.to_string())),
        }
    }
}
