//! Tests of the file chooser.

use crate::common::kinds::{parts, recent, uri_path};

use crate::toolkit::{popup, theme};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::ui::Keyboard;
use crate::toolkit::widget::Node;
use super::*;
use super::entries::{resolve, sort};
use super::kept::{folders_read, keep_folder};
use super::places::{bookmarks, drives, once_each, usual};
mod basics;
mod menu;
mod listing;

fn main_button(c: &Chooser) -> (String, bool) {
    let view = c.view();
    let words: Vec<String> = view.worded().into_iter().filter_map(|(_, words)| words).collect();
    assert_eq!(words.len(), 1, "one main action: {words:?}");
    let ready = view.buttons().into_iter().any(|(tip, ready)| tip == words[0] && ready);

    if matches!(c.ask.mode, Mode::Save) {
        let icons: Vec<_> = view.iconed().into_iter().filter(|(tip, _)| *tip == words[0]).collect();
        assert!(icons.is_empty(), "{} has an icon beside its word: {icons:?}", words[0]);
    }
    (words[0].clone(), ready)
}

fn menu_chooser(tag: &str, mode: Mode) -> (PathBuf, Chooser) {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-menu-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("in/Albums")).unwrap();
    std::fs::create_dir_all(dir.join("in/Letters")).unwrap();
    std::fs::write(dir.join("in/Letters/one.txt"), b"a letter").unwrap();
    std::fs::write(dir.join("in/notes.txt"), b"the notes").unwrap();
    std::fs::write(dir.join("in/list.txt"), b"the list").unwrap();
    let ask = Ask { title: "Made up".into(), accept: None, mode, filters: Vec::new(), filter: None, choices: Vec::new(),
                    folder: Some(dir.join("in")), name: String::new(), app_id: "org.example.Paint".into() };
    let mut c = Chooser::looked(ask, |_| {});
    c.kept = dir.join("folders");
    c.pins = dir.join("pins");
    let bin = lent::Bin::at(&dir.join("Trash"), None);
    bin.ready().unwrap();
    c.bin = Some(bin);
    (dir, c)
}

fn at(c: &Chooser, name: &str) -> usize {
    c.entries.iter().position(|e| e.name == name).unwrap_or_else(|| panic!("{name} is not listed"))
}

fn lines(c: &Chooser) -> Vec<String> {
    c.menu.as_ref().map(|(menu, _)| menu.rows.iter().map(|r| r.words.clone()).collect()).unwrap_or_default()
}

fn pick(c: &mut Chooser, words: &str) {
    let i = lines(c).iter().position(|w| w == words).unwrap_or_else(|| panic!("no line {words}"));
    c.update(Msg::Pop(PopupMsg::Pick(i)));
    assert!(c.menu.is_none(), "the menu closes");
}

const ALL_MODES: [fn() -> Mode; 4] = [|| Mode::Save, || Mode::Open { many: false, folder: false }, || Mode::Open { many: true, folder: false },
                                     || Mode::Open { many: false, folder: true }];

fn mime() -> Mime {
    let mut m = Mime::default();
    m.add("50:image/png:*.png\n50:text/x-csrc:*.c\n50:text/x-makefile:makefile\n10:application/x-tar:*.tar\n55:application/x-compressed-tar:*.tar.gz\n50:text/x-readme:README*:cs\n",
        "application/x-compressed-tar application/gzip\ntext/x-csrc text/plain\n", "image/x-png image/png\n");
    m
}

fn new_folders_are_named_in_the_list(c: &mut Chooser, dir: &Path) {
    c.update(Msg::NewFolder);
    let first = dir.join("New folder");
    assert!(first.is_dir(), "New folder made nothing");
    let r = c.renaming.clone().expect("the new folder's name is not open for typing");
    assert!(r.fresh && r.path == first && r.text == "New folder");
    assert_eq!(c.chosen.and_then(|i| c.entries.get(i)).map(|e| e.path.clone()), Some(first.clone()), "the new folder is not chosen");
    assert!(c.rename_box(26.0).is_some(), "the new folder's typing box is not over its row");
    assert!(!c.view().said().iter().any(|w| w == "entry: Folder name"), "a box of its own came with New folder");

    c.update(Msg::RenameStop);
    assert!(c.renaming.is_none() && first.is_dir() && c.at == Where::Folder(dir.to_path_buf()));

    c.update(Msg::NewFolder);
    let second = dir.join("New folder (2)");
    assert!(second.is_dir() && c.renaming.as_ref().is_some_and(|r| r.path == second));
    c.update(Msg::RenameText("a/b".into()));
    c.update(Msg::RenameDone);
    assert!(c.renaming.as_ref().is_some_and(|r| r.why.is_some()), "a name with a slash was let through");
    assert!(c.view().said().iter().any(|w| w.contains("slash")), "{:?}", c.view().said());

    c.update(Msg::RenameText("Bills".into()));
    c.update(Msg::RenameDone);
    assert!(dir.join("Bills").is_dir() && !second.exists());
    assert_eq!(c.at, Where::Folder(dir.join("Bills")));
    assert!(c.renaming.is_none());
}

impl Chooser {

    fn looked(ask: Ask, done: impl FnOnce(Outcome) + 'static) -> Chooser {
        let mut c = Chooser::new(ask, done);
        c.begin();
        c
    }
}
