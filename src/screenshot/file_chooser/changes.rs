//! Changes the chooser makes to the disk: paste, delete, rename and new folders.

use std::path::{Path, PathBuf};

use crate::common::kinds::shown_name;
use crate::toolkit::t;

use super::{lent, Answer, Chooser, Mode, Msg, Renaming, Verb, Where};

impl Chooser {

    pub(super) fn delete_sure(&mut self) {
        let Some((paths, asked)) = self.deleting.take() else { return };
        self.chosen = None;
        self.picked.clear();
        self.work(Verb::Deleted, move || lent::delete(paths, asked));
    }

    pub(super) fn pasted(&mut self, got: Option<(Vec<PathBuf>, bool)>, to: PathBuf) {
        let (items, cut) = match (got, &self.clip) {
            (Some((paths, said_cut)), own) => {
                let cut = lent::moves(&paths, said_cut, own.as_ref().map(|(own, cut)| (&own[..], *cut)));
                (paths, cut)
            }
            (None, Some(own)) => own.clone(),
            (None, None) => {
                self.said = t!("screenshot-chooser-nothing-cut").into();
                return;
            }
        };

        if self.post.is_some() && self.reply.is_some() {
            self.said = t!("screenshot-chooser-paste-busy").into();
            return;
        }
        if cut {
            self.clip = None;
        }
        let verb = if cut { Verb::Moved } else { Verb::Copied };
        let Some(post) = self.post.clone() else {

            return self.work(verb, move || lent::bring(items, &to, cut));
        };

        let (tx, rx) = std::sync::mpsc::channel();
        self.reply = Some(tx);
        self.aside(move || {
            let failed = lent::bring_asking(items, &to, cut, &mut |clash| {

                if !post.send(Msg::Asked(clash)) {
                    return (Answer::Skip, true);
                }
                rx.recv().unwrap_or((Answer::Skip, true))
            });
            Msg::Worked(failed, verb)
        });
    }

    pub(super) fn work(&mut self, verb: Verb, work: impl FnOnce() -> Vec<String> + Send + 'static) {
        self.aside(move || Msg::Worked(work(), verb));
    }

    pub(super) fn worked(&mut self, failed: Vec<String>, verb: Verb) {
        if matches!(verb, Verb::Moved | Verb::Copied) {
            self.reply = None;
            self.asking = None;
        }
        if !failed.is_empty() {
            let (n, names) = (failed.len(), failed.join(", "));
            self.failed = match verb {
                Verb::Moved => t!("screenshot-chooser-not-moved", n = n, names = names),
                Verb::Copied => t!("screenshot-chooser-not-copied", n = n, names = names),
                Verb::Trashed => t!("screenshot-chooser-not-trashed", n = n, names = names),
                Verb::Deleted => t!("screenshot-chooser-not-deleted", n = n, names = names),
            };
        }
        self.list();
    }

    pub(super) fn rename_end(&mut self, asked: bool) {
        let Some(r) = self.renaming.as_mut().filter(|r| !r.out) else { return };
        if r.text == shown_name(&r.path) {
            let into = (asked && r.fresh).then(|| r.path.clone());
            self.renaming = None;
            if let Some(path) = into {
                self.go(Where::Folder(path));
            }
            return;
        }
        r.out = true;
        let change = r.clone();
        self.aside(move || Msg::Renamed(lent::rename(&change.path, &change.text), change, asked));
    }

    pub(super) fn renamed(&mut self, done: Result<PathBuf, String>, mut change: Renaming, asked: bool) {

        let ours = self.renaming.as_ref().is_some_and(|r| r.out && r.path == change.path);
        if ours {
            self.renaming = None;
        }
        match done {
            Ok(new) if ours && asked && change.fresh && new.parent() == self.folder() => self.go(Where::Folder(new)),
            Ok(new) => {

                self.pick_next = Some(new);
                if matches!(self.ask.mode, Mode::Save) && !change.folder && self.name == shown_name(&change.path) {
                    self.name = change.text;
                    self.armed = None;
                }
                self.list();
            }
            Err(why) if asked && ours => {
                change.why = Some(why);
                change.out = false;
                self.renaming = Some(change);
            }
            Err(_) => {}
        }
    }

    pub(super) fn make_folder(&mut self) {
        self.said.clear();
        self.failed.clear();
        let Some(at) = self.folder().map(Path::to_path_buf) else { return };
        self.aside(move || Msg::Made(lent::new_folder(&at)));
    }

    pub(super) fn folder_was_made(&mut self, made: Result<PathBuf, String>) {
        match made {
            Ok(path) => {
                if path.parent() != self.folder() {
                    return;
                }
                self.pick_next = Some(path.clone());
                self.renaming = Some(Renaming { text: shown_name(&path), path, folder: true, fresh: true, why: None, out: false });
                self.asked_rename += 1;
                self.list();
            }
            Err(why) => self.failed = why,
        }
    }
}
