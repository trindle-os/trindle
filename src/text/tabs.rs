//! Switching and closing tabs, and the question asked before one is closed.

use super::enc::Form;
use crate::toolkit::editing::doc::Doc;
use crate::toolkit::t;
use super::{Answer, Bar, Tab, Text, View, store};

impl Text {
    pub(super) fn switch_to(&mut self, i: usize) {
        if i >= self.tabs.len() {
            return;
        }
        let editing = self.tabs.get(self.active).map(|t| t.pane.borrow().editing).unwrap_or(false);
        if i != self.active {
            if let Some(old) = self.tabs.get(self.active) {
                let mut old = old.pane.borrow_mut();
                old.editing = false;
                old.marks.clear();
            }
        }
        self.active = i;
        self.view = View::Edit;
        if self.bar == Bar::Spell {
            self.bar = Bar::None;
        }
        if editing || self.bar == Bar::None {
            self.focus_editor();
        }
        self.mark_found();
    }

    pub(super) fn focus_editor(&mut self) {
        self.grab = self.grab.wrapping_add(1);
        self.grab_bar = false;
        if let Some(t) = self.tabs.get(self.active) {
            t.pane.borrow_mut().editing = true;
        }
    }

    pub(super) fn focus_bar(&mut self) {
        self.grab = self.grab.wrapping_add(1);
        self.grab_bar = true;
        if let Some(t) = self.tabs.get(self.active) {
            t.pane.borrow_mut().editing = false;
        }
    }

    pub(super) fn close_tab(&mut self, i: usize) {
        if i >= self.tabs.len() {
            return;
        }
        if self.tabs[i].unsaved() {
            self.closing = vec![self.tabs[i].id];
            self.quitting = false;
            self.switch_to(i);
            self.view = View::Closing;
            return;
        }
        self.drop_tab(i);
    }

    pub(super) fn drop_tab(&mut self, i: usize) {
        self.tabs.remove(i);
        if self.tabs.is_empty() {
            self.add_tab(None, Form::default(), Doc::new(String::new()));
        }
        if self.active > i || self.active >= self.tabs.len() {
            self.active = self.active.saturating_sub(1);
        }
        self.focus_editor();
        self.keep();
    }

    pub(super) fn quit(&mut self) {
        self.closing = self.tabs.iter().filter(|t| t.unsaved()).map(|t| t.id).collect();
        self.quitting = true;
        self.next_close();
    }

    pub(super) fn next_close(&mut self) {
        if let Some(&id) = self.closing.first() {
            if let Some(i) = self.tabs.iter().position(|t| t.id == id) {
                self.active = i;
                self.view = View::Closing;
                return;
            }
            self.closing.remove(0);
            return self.next_close();
        }
        self.view = View::Edit;
        if self.quitting {
            self.keep();
            self.events.quit();
        }
    }

    pub(super) fn to_keep(&self) -> (Vec<store::Entry>, usize) {
        self.to_keep_except(&[])
    }

    fn to_keep_except(&self, unkept: &[u64]) -> (Vec<store::Entry>, usize) {
        let stays = |t: &Tab| !(unkept.contains(&t.id) || (self.quitting && t.path.is_none() && t.pane.borrow().doc.is_empty()));
        let entries: Vec<store::Entry> = self.tabs.iter().filter(|t| stays(t)).map(Tab::entry).collect();
        let before = self.tabs.iter().take(self.active).filter(|t| stays(t)).count();
        let active = before.min(entries.len().saturating_sub(1));
        (entries, active)
    }

    pub(super) fn answer(&mut self, a: Answer) {
        if self.choosing {
            return;
        }
        let Some(&id) = self.closing.first() else { return };
        let Some(i) = self.tabs.iter().position(|t| t.id == id) else {
            self.closing.remove(0);
            return self.next_close();
        };
        match a {
            Answer::Keep => self.stop_closing(),
            Answer::Discard => {
                self.closing.remove(0);
                self.drop_tab(i);
                self.next_close();
            }
            Answer::Save => {
                if self.save(i, true) {
                    self.closing.remove(0);
                    self.drop_tab(i);
                    self.next_close();
                } else if self.tabs[i].path.is_some() {
                    if self.tabs[i].changed {
                        self.switch_to(i);
                    }
                    self.stop_closing();
                } else {
                    self.choosing = true;
                }
            }
        }
    }

    pub(super) fn stop_closing(&mut self) {
        self.closing.clear();
        self.choosing = false;
        self.quitting = false;
        self.view = View::Edit;
        self.focus_editor();
    }

    pub(super) fn keep(&mut self) {
        if self.settings_unsaved {
            self.save_settings();
        }
        let mut unkept: Vec<u64> = Vec::new();
        let mut failed = None;
        for t in &mut self.tabs {
            let unsaved = t.unsaved();
            let pane = t.pane.borrow();
            if unsaved && t.kept != pane.doc.version {
                match store::keep_text(&self.dir, t.id, pane.doc.text()) {
                    Ok(()) => t.kept = pane.doc.version,
                    Err(e) => {
                        unkept.push(t.id);
                        failed = Some(t!("text-work-not-kept", folder = self.dir.display().to_string(), reason = e.to_string()));
                    }
                }
            } else if !unsaved {
                t.kept = pane.doc.version;
            }
        }
        if let Some(words) = failed {
            self.fail(words);
        }
        let (entries, active) = self.to_keep_except(&unkept);
        if entries != self.written || active != self.written_active || entries.is_empty() {
            match store::write_list(&self.dir, &entries, active) {
                Ok(()) => {
                    self.written = entries;
                    self.written_active = active;
                }
                Err(e) => self.fail(t!("text-work-not-kept", folder = self.dir.display().to_string(), reason = e.to_string())),
            }
        }
    }
}
