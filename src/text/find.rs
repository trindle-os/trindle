//! The find, replace and go-to-line bars.

use crate::toolkit::editing::doc;
use crate::toolkit::t;
use super::{Bar, Text, View};

impl Text {
    pub(super) fn open_bar(&mut self, bar: Bar) {
        self.raw();
        self.bar = bar;
        self.view = View::Edit;
        self.found = None;
        if bar == Bar::Find || bar == Bar::Replace {
            let picked = self.pane().borrow().doc.selected().to_string();
            if !picked.is_empty() && !picked.contains('\n') {
                self.find = picked;
            }
        }
        if bar == Bar::Goto {
            self.goto.clear();
        }
        self.focus_bar();
        self.mark_found();
    }

    pub(super) fn close_bar(&mut self) {
        self.bar = Bar::None;
        self.found = None;
        self.pane().borrow_mut().marks.clear();
        self.focus_editor();
    }

    pub(super) fn mark_found(&mut self) {
        let Some(tab) = self.tabs.get(self.active) else { return };
        let mut pane = tab.pane.borrow_mut();
        pane.marks = if matches!(self.bar, Bar::Find | Bar::Replace) && !self.find.is_empty() {
            pane.doc.find(&self.find, self.how)
        } else {
            Vec::new()
        };
    }

    pub(super) fn step(&mut self, forward: bool) {
        self.mark_found();
        let mut pane = self.pane().borrow_mut();
        if pane.marks.is_empty() {
            return;
        }
        let sel = pane.doc.selection();
        let next = if forward {
            pane.marks.iter().find(|m| m.start >= sel.end && *m != &sel).or(pane.marks.first()).cloned()
        } else {
            pane.marks.iter().rev().find(|m| m.end <= sel.start).or(pane.marks.last()).cloned()
        };
        if let Some(r) = next {
            pane.reveal(r);
        }
    }

    pub(super) fn search_from_here(&mut self) {
        self.mark_found();
        let mut pane = self.pane().borrow_mut();
        let from = pane.doc.selection().start;
        let hit = pane.marks.iter().find(|m| m.start >= from).or(pane.marks.first()).cloned();
        if let Some(r) = hit {
            pane.reveal(r);
        }
    }

    pub(super) fn replace_one(&mut self) {
        self.mark_found();
        {
            let mut pane = self.pane().borrow_mut();
            let sel = pane.doc.selection();
            if pane.marks.contains(&sel) {
                pane.doc.put(&self.with, doc::Kind::Other);
            }
        }
        self.step(true);
    }

    pub(super) fn replace_all(&mut self) {
        let n = self.pane().borrow_mut().doc.replace_all(&self.find, &self.with, self.how);
        self.pane().borrow_mut().show_caret();
        self.found = Some(match n {
            0 => t!("text-none-found").into(),
            n => t!("text-replaced", n = n),
        });
        if n > 0 {
            self.noted_to("bar");
        }
        self.mark_found();
    }

    pub(super) fn go_to_line(&mut self) {
        let Ok(n) = self.goto.trim().parse::<usize>() else {
            self.found = Some(t!("text-type-line").into());
            return;
        };
        let mut pane = self.pane().borrow_mut();
        if let Some(at) = pane.doc.line_start(n) {
            pane.reveal(at..at);
        }
        drop(pane);
        self.bar = Bar::None;
        self.found = None;
        self.focus_editor();
    }
}
