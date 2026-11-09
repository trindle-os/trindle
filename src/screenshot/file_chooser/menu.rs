//! The right-click menu of the chooser and the acts its lines do.

use std::path::{Path, PathBuf};

use crate::common::kinds::shown_name;
use crate::toolkit::{popup, t};
use crate::toolkit::popup::Popup;
use crate::toolkit::ui::App;

use super::{lent, Act, Chooser, Msg, On, Renaming, Verb};

impl Act {

    const ITEM: [Act; 9] = [Act::Rename, Act::Copy, Act::CopyPath, Act::Cut, Act::Paste, Act::NewFolder, Act::Pin, Act::Trash, Act::Delete];
    const ROOM: [Act; 2] = [Act::Paste, Act::NewFolder];

    fn line(self, pinned: bool) -> popup::Row {
        match self {
            Act::Rename => popup::row("document-edit-symbolic", t!("screenshot-chooser-rename")).keys("F2"),
            Act::Copy => popup::row("edit-copy-symbolic", t!("screenshot-copy")),
            Act::Cut => popup::row("edit-cut-symbolic", t!("screenshot-chooser-cut")),
            Act::Paste => popup::row("edit-paste-symbolic", t!("screenshot-chooser-paste")),
            Act::NewFolder => popup::row("folder-new-symbolic", t!("screenshot-chooser-new-folder")),
            Act::Trash => popup::row("user-trash-symbolic", t!("screenshot-chooser-trash")),
            Act::Delete => popup::row(lent::SHRED_ICON, t!("screenshot-chooser-delete")).keys("Shift+Delete"),
            Act::CopyPath => popup::row("edit-copy-symbolic", t!("screenshot-chooser-copy-location")),
            Act::Pin => popup::row("view-pin-symbolic", if pinned { t!("screenshot-chooser-unpin") } else { t!("screenshot-chooser-pin") }),
        }
    }
}

impl Chooser {

    pub(super) fn acts(&self, on: Option<usize>) -> Vec<Act> {
        match (self.folder().is_some(), on) {
            (true, Some(i)) => Act::ITEM.iter().copied().filter(|a| *a != Act::Pin || self.entries.get(i).is_some_and(|e| e.folder)).collect(),
            (true, None) => Act::ROOM.to_vec(),
            (false, Some(_)) => vec![Act::Copy],
            (false, None) => Vec::new(),
        }
    }

    pub(super) fn open_menu(&mut self, on: Option<usize>, at: (f32, f32), keys: bool) {
        if on.is_none() {

            self.chosen = None;
            self.picked.clear();
        }
        let acts = self.acts(on);
        if acts.is_empty() {
            return;
        }
        self.menu_on = on.and_then(|i| self.entries.get(i)).map(|e| e.path.clone());
        let pinned = self.menu_on.as_ref().is_some_and(|p| lent::is_pinned(&self.pins, p));
        let menu = Popup::new(acts.iter().map(|a| a.line(pinned)).collect(), popup::Place::At(at.0, at.1));
        self.menu = Some((if keys { menu.lit() } else { menu }, acts));
    }

    fn picked_paths(&self) -> Vec<PathBuf> {
        self.picked_entries().iter().map(|e| e.path.clone()).collect()
    }

    pub(super) fn act(&mut self, act: Act, on: On) {
        self.said.clear();
        self.failed.clear();
        match act {
            Act::Rename => self.start_rename(on),
            Act::Copy | Act::Cut => self.copy_or_cut(act == Act::Cut),
            Act::Paste => self.start_paste(),
            Act::NewFolder => self.update(Msg::NewFolder),
            Act::Delete => self.ask_to_delete(),
            Act::CopyPath => {
                let paths = self.picked_paths();
                if !paths.is_empty() && self.post.is_some() {
                    lent::offer_locations(&paths);
                }
            }
            Act::Pin => self.toggle_pin(on),
            Act::Trash => self.trash_picked(),
        }
    }

    fn start_rename(&mut self, on: On) {
        let row = match on {
            On::Menu(Some(path)) => self.entries.iter().position(|e| e.path == path),
            On::Menu(None) => None,
            On::Keys => self.chosen,
        };
        let Some(e) = row.and_then(|i| self.entries.get(i)).filter(|_| self.folder().is_some()) else { return };
        self.renaming = Some(Renaming { path: e.path.clone(), text: e.name.clone(), folder: e.folder, fresh: false, why: None, out: false });
        self.asked_rename += 1;
    }

    fn copy_or_cut(&mut self, cut: bool) {
        let paths = self.picked_paths();
        if paths.is_empty() {
            return;
        }
        self.said = if cut {
            t!("screenshot-chooser-was-cut", what = what(&paths))
        } else {
            t!("screenshot-chooser-was-copied", what = what(&paths))
        };
        if self.post.is_some() {
            lent::offer(&paths, cut);
        }
        self.clip = Some((paths, cut));
    }

    fn start_paste(&mut self) {
        let Some(to) = self.folder().map(Path::to_path_buf) else { return };
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("chooser-paste".into()).spawn(move || {
                    post.send(Msg::Pasted(lent::on_clipboard(), to));
                });
            }
            None => self.pasted(None, to),
        }
    }

    fn ask_to_delete(&mut self) {
        let paths = self.picked_paths();
        if !paths.is_empty() {
            let asked = lent::ask_about(&paths);
            self.deleting = Some((paths, asked));
            self.questions += 1;
        }
    }

    fn toggle_pin(&mut self, on: On) {
        let folder = match on {
            On::Menu(Some(path)) => path,
            _ => return,
        };
        if lent::toggle_pin(&self.pins, &folder).is_err() {
            self.failed = t!("screenshot-chooser-pin-not-kept").into();
        }
    }

    fn trash_picked(&mut self) {
        let paths = self.picked_paths();
        if paths.is_empty() {
            return;
        }
        self.chosen = None;
        self.picked.clear();
        let bin = self.bin.clone();
        self.work(Verb::Trashed, move || lent::to_trash(paths, bin));
    }
}

fn what(paths: &[PathBuf]) -> String {
    match paths {
        [one] => shown_name(one),
        many => t!("screenshot-chooser-items", n = many.len()),
    }
}
