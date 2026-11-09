//! The file chooser's right-click acts, done by the toolkit's file work as Folders does them.

use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use crate::toolkit::common::file_work::{clip, proof, reach, work};
use crate::toolkit::t;

pub(super) use crate::toolkit::common::file_work::proof::Asked;
pub(super) use crate::toolkit::common::file_work::trash::Bin;
pub(super) use work::{Answer, Clash};
use work::Kind;

type Plan = work::Plan;
type Ctl = work::Ctl;
type News = work::News;

const CLIP_LIMIT: u64 = 1 << 20;

pub(super) fn stem_len(name: &str, folder: bool) -> usize {
    match name.rfind('.') {
        Some(i) if i > 0 && !folder => i,
        _ => name.len(),
    }
}

pub(super) fn rename(path: &Path, name: &str) -> Result<PathBuf, String> {
    if let Some(why) = bad_name(name) {
        return Err(why.to_string());
    }
    let new = path.with_file_name(name);
    if std::fs::symlink_metadata(&new).is_ok() {
        return Err(t!("chosen-inplace-taken", kind = if new.is_dir() { "folder" } else { "file" }, name = name));
    }

    crate::common::files::rename_new(path, &new).map_err(|e| t!("chosen-inplace-not-renamed", name = work::name_on_disk(path), why = e.to_string()))?;
    Ok(new)
}

fn bad_name(name: &str) -> Option<&'static str> {
    if name.trim().is_empty() {
        Some(t!("chosen-act-name-empty"))
    } else if name.contains('/') {
        Some(t!("chosen-act-name-slash"))
    } else if name == "." || name == ".." {
        Some(t!("chosen-act-name-kept"))
    } else if name.len() > work::NAME_MOST {
        Some(t!("chosen-act-name-long"))
    } else {
        None
    }
}

pub(super) fn new_folder(dir: &Path) -> Result<PathBuf, String> {
    let base = t!("chosen-act-new-folder");
    let plain = dir.join(base);
    if !reach::answers(dir) {
        return Err(reach::not_answering().into());
    }
    let path = if std::fs::symlink_metadata(&plain).is_err() { plain } else { work::free_name(dir, base.as_ref(), true) };
    std::fs::create_dir(&path).map(|()| path).map_err(|e| t!("chosen-act-make-failed", base = base, why = &e.to_string()))
}

pub(super) fn offer(paths: &[PathBuf], cut: bool) {
    let kinds = clip::kinds_for(paths, cut);
    let _ = std::thread::Builder::new().name("chooser-clip".into()).spawn(move || {
        crate::common::selection::offer_kinds(kinds, || {});
    });
}

pub(super) fn on_clipboard() -> Option<(Vec<PathBuf>, bool)> {
    let read = crate::common::selection::paste_kinds(CLIP_LIMIT, &clip::READ);
    Some(clip::parse(&read)).filter(|got| !got.paths.is_empty()).map(|got| (got.paths, got.cut))
}

pub(super) fn moves(named: &[PathBuf], said_cut: bool, own: Option<(&[PathBuf], bool)>) -> bool {
    clip::moves(named, said_cut, own, None)
}

pub(super) fn bring(items: Vec<PathBuf>, to: &Path, cut: bool) -> Vec<String> {
    let kind = if cut { Kind::Move } else { Kind::Copy };
    let ctl = Ctl { fixed: Some(Answer::Both), ..Ctl::default() };
    work::run(Plan { kind, items, to: to.to_path_buf(), key: None }, &ctl, &|_| {}).troubles()
}

pub(super) const MERGE_ICON: &str = "folder-drag-accept-symbolic";

pub(super) fn bring_asking(items: Vec<PathBuf>, to: &Path, cut: bool, ask: &mut dyn FnMut(Clash) -> (Answer, bool)) -> Vec<String> {
    let kind = if cut { Kind::Move } else { Kind::Copy };
    let (tx, rx) = std::sync::mpsc::channel();
    let ctl = Ctl { answers: Some(rx), ..Ctl::default() };
    let ask = std::cell::RefCell::new(ask);
    let tell = |news: News| {
        if let News::Clash(clash) = news {
            let mut guard = ask.borrow_mut();
            let answer = (*guard)(clash);
            let _ = tx.send(answer);
        }
    };
    work::run(Plan { kind, items, to: to.to_path_buf(), key: None }, &ctl, &tell).troubles()
}

pub(super) fn to_trash(items: Vec<PathBuf>, bin: Option<Bin>) -> Vec<String> {
    let ctl = Ctl { bin, ..Ctl::default() };
    work::run(Plan { kind: Kind::Trash, items, to: PathBuf::new(), key: None }, &ctl, &|_| {}).troubles()
}

pub(super) const SHRED_ICON: &str = "edit-delete-symbolic";

pub(super) fn ask_about(items: &[PathBuf]) -> Vec<Asked> {
    proof::asked(items)
}

pub(super) fn delete(items: Vec<PathBuf>, asked: Vec<Asked>) -> Vec<String> {
    let ctl = Ctl { asked, ..Ctl::default() };
    work::run(Plan { kind: Kind::Delete, items, to: PathBuf::new(), key: None }, &ctl, &|_| {}).troubles()
}

const TEXT_TYPES: [&str; 3] = ["text/plain;charset=utf-8", "text/plain", "UTF8_STRING"];

pub(super) fn offer_locations(paths: &[PathBuf]) {
    let text = paths.iter().map(|t| t.to_string_lossy().into_owned()).collect::<Vec<_>>().join("\n");
    let _ = std::thread::Builder::new().name("chooser-clip".into()).spawn(move || {
        crate::common::selection::offer(text.into_bytes(), &TEXT_TYPES, || {});
    });
}

pub(super) fn is_pinned(file: &Path, folder: &Path) -> bool {
    read_pins(file).iter().any(|p| p == folder)
}

pub(super) fn toggle_pin(file: &Path, folder: &Path) -> Result<bool, ()> {
    let mut pins = read_pins(file);
    let now = match pins.iter().position(|p| p == folder) {
        Some(i) => { pins.remove(i); false }
        None => { pins.push(folder.to_path_buf()); true }
    };
    write_pins(file, &pins).map(|()| now).map_err(|_| ())
}

pub(super) fn pins_file() -> PathBuf {
    crate::common::files::config_path("folders-pinned")
}

fn read_pins(file: &Path) -> Vec<PathBuf> {
    let bytes = crate::common::files::read_file(file).unwrap_or_default();
    bytes.split(|&b| b == b'\n').filter(|l| !l.iter().all(u8::is_ascii_whitespace))
        .map(|l| PathBuf::from(std::ffi::OsString::from_vec(l.to_vec()))).collect()
}

fn write_pins(file: &Path, pins: &[PathBuf]) -> std::io::Result<()> {
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }

    crate::common::files::may_rewrite(file)?;
    let mut out = Vec::new();

    for p in pins.iter().map(|p| p.as_os_str().as_bytes()).filter(|b| !b.contains(&b'\n')) {
        out.extend_from_slice(p);
        out.push(b'\n');
    }
    crate::common::files::write_whole(file, &out)
}
