//! What the main button and opening a row do.

use std::path::{Path, PathBuf};

use crate::common::kinds::{file_uri, shown_name};
use crate::toolkit::t;

use super::{Chooser, Entry, Found, Mode, Outcome, Where};
use super::entries::resolve;
use super::kept::keep_folder;

impl Chooser {
    fn choices(&self) -> Vec<(String, String)> {
        self.ask.choices.iter().map(|p| (p.id.clone(), p.value.clone())).collect()
    }

    pub(super) fn finish(&mut self, outcome: Outcome) {
        if let Some(done) = self.done.take() {
            done(outcome);
        }
    }

    fn chose(&mut self, paths: Vec<PathBuf>, kept: &Path) {
        keep_folder(&self.kept, &self.ask.app_id, kept);
        let filter = self.ask.filter.and_then(|i| self.ask.filters.get(i)).cloned();
        let outcome = Outcome::Chosen { uris: paths.iter().map(|p| file_uri(p)).collect(), choices: self.choices(), filter };
        self.finish(outcome);
    }

    pub(super) fn ready(&self) -> bool {
        match &self.ask.mode {
            Mode::Open { folder: false, .. } => self.picked_entries().iter().any(|e| !e.folder),
            Mode::Open { folder: true, .. } => self.folder().is_some() || self.picked_entries().iter().any(|e| e.folder),
            Mode::Save => !self.name.trim().is_empty() && (self.folder().is_some() || resolve(self.name.trim(), None, &self.home).is_some()),
            Mode::SaveMany(_) => self.folder().is_some() || self.picked_entries().iter().any(|e| e.folder),
        }
    }

    pub(super) fn picked_entries(&self) -> Vec<&Entry> {
        let mut at: Vec<usize> = self.picked.clone();
        if at.is_empty() {
            at.extend(self.chosen);
        }
        at.iter().filter_map(|&i| self.entries.get(i)).collect()
    }

    pub(super) fn taken(&self) -> Option<PathBuf> {
        let target = self.target()?;
        matches!(&self.seen, Some((seen, true)) if *seen == target).then_some(target)
    }

    pub(super) fn target(&self) -> Option<PathBuf> {
        if !matches!(self.ask.mode, Mode::Save) {
            return None;
        }
        resolve(self.name.trim(), self.folder(), &self.home)
    }

    pub(super) fn accept(&mut self, pressed: bool) {
        self.said.clear();
        match self.ask.mode.clone() {
            Mode::Open { folder: false, .. } => self.accept_files(),
            Mode::Open { folder: true, .. } => self.accept_folder(),
            Mode::SaveMany(names) => {
                let Some(into) = self.folder_picked() else { return };
                self.look_for_free_names(into, names);
            }
            Mode::Save => self.look_at_target(pressed),
        }
    }

    fn accept_files(&mut self) {
        let picked = self.picked_entries();
        if let [one] = picked.as_slice() {
            if one.folder {
                let to = one.path.clone();
                return self.go(Where::Folder(to));
            }
        }
        let files: Vec<PathBuf> = picked.iter().filter(|e| !e.folder).map(|e| e.path.clone()).collect();
        if let Some(first) = files.first() {
            let kept = first.parent().unwrap_or(Path::new("/")).to_path_buf();
            self.chose(files, &kept);
        }
    }

    fn accept_folder(&mut self) {
        if let Some(pick) = self.folder_picked() {
            let kept = pick.parent().unwrap_or(&pick).to_path_buf();
            self.chose(vec![pick], &kept);
        }
    }

    fn folder_picked(&self) -> Option<PathBuf> {
        self.picked_entries().iter().find(|e| e.folder).map(|e| e.path.clone())
            .or_else(|| self.folder().map(Path::to_path_buf))
    }

    pub(super) fn save_to(&mut self, found: Found, target: PathBuf, pressed: bool) {
        self.accepting.retain(|t| *t != target);
        self.late_save = false;
        if self.target().as_ref() != Some(&target) {
            return;
        }
        if found.folder {
            self.name.clear();
            return self.go(Where::Folder(target));
        }
        let Some(parent) = target.parent().filter(|_| found.parent).map(Path::to_path_buf) else {
            self.said = t!("screenshot-chooser-no-folder").into();
            return;
        };
        let sure = (pressed && self.offered.borrow().as_ref() == Some(&target)) || self.armed.as_ref() == Some(&target);
        if found.there && !sure {
            self.said = t!("screenshot-chooser-replace-ask", name = shown_name(&target));
            self.armed = Some(target);
            return;
        }
        self.chose(vec![target], &parent);
    }

    pub(super) fn named(&mut self, paths: Vec<PathBuf>, into: PathBuf) {
        self.accepting.retain(|t| *t != into);
        self.late_save = false;
        self.chose(paths, &into);
    }

    pub(super) fn open(&mut self, i: usize) {
        let Some(e) = self.entries.get(i).cloned() else { return };
        if e.folder {
            if self.saving() {
                self.name = if matches!(self.ask.mode, Mode::Save) { self.name.clone() } else { String::new() };
            }
            return self.go(Where::Folder(e.path));
        }
        self.chosen = Some(i);
        self.picked = vec![i];
        if matches!(self.ask.mode, Mode::Save) {
            self.name = e.name.clone();
            self.go_to_parent_of(&e.path);
        }
        self.accept(false);
    }

    fn go_to_parent_of(&mut self, path: &Path) {
        if self.at == Where::Recent {
            if let Some(p) = path.parent() {
                self.moved = true;
                self.at = Where::Folder(p.to_path_buf());
                self.again = false;
                self.list();
            }
        }
    }
}
