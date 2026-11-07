//! Reading and writing files.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use super::enc::Form;
use super::store::LARGEST;
use crate::common::files;
use crate::toolkit::chooser::Ask;
use crate::toolkit::editing::doc::Doc;
use crate::toolkit::editing::doc;
use crate::toolkit::editing::editor::Pane;
use crate::toolkit::widget::{Node, note};
use crate::toolkit::{chooser, t};
use super::{Do, NOTE_FOR, Purpose, Tab, Text, View, absolute, enc, look, markdown, pdf, same_file, store};

impl Text {
    pub(super) fn open_args(&mut self, args: &[String]) {
        for path in args.iter().filter_map(|a| absolute(a)) {
            self.open_path(&path);
        }
    }

    pub(super) fn open_path(&mut self, path: &Path) {
        if let Some(i) = self.tabs.iter().position(|t| t.path.as_deref().is_some_and(|p| same_file(p, path))) {
            self.switch_to(i);
            return;
        }
        let shown = path.display().to_string();
        match std::fs::metadata(path) {
            Ok(m) if m.is_dir() => return self.fail(t!("text-is-folder", name = &shown)),
            Ok(m) if m.len() > LARGEST => return self.fail(t!("text-too-big", name = &shown)),
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return self.fail(t!("text-cannot-open", name = &shown, reason = e.to_string())),
            _ => {}
        }
        let stamp = store::stamp(path);
        let (text, form) = match store::read_document(path) {
            Ok(bytes) => enc::read(&bytes),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (String::new(), Form::default()),
            Err(e) => return self.fail(t!("text-cannot-open", name = &shown, reason = e.to_string())),
        };
        let at = if self.tabs.get(self.active).is_some_and(Tab::blank) {
            let id = self.tabs[self.active].id;
            let mut pane = Pane::new(Doc::new(text), look(&self.settings));
            pane.checker = self.checker();
            let kept = pane.doc.version;
            self.tabs[self.active] = Tab {
                id,
                path: Some(path.to_path_buf()),
                form,
                pane: Rc::new(RefCell::new(pane)),
                markdown: false,
                shown: false,
                finished: Rc::default(),
                kept,
                stamp,
                changed: false,
            };
            *self.counted.borrow_mut() = None;
            self.active
        } else {
            let at = self.add_tab(Some(path.to_path_buf()), form, Doc::new(text));
            self.tabs[at].stamp = stamp;
            at
        };
        self.tabs[at].markdown = is_markdown(path);
        self.switch_to(at);
        self.remember(path);
        self.said = None;
    }

    fn remember(&mut self, path: &Path) {
        if let Ok(list) = store::add_recent(&self.dir, path) {
            self.recent = list;
        }
    }

    pub(super) fn choose_open(&mut self) {
        let folder = self.tabs.get(self.active).and_then(|t| t.path.as_ref()).and_then(|p| p.parent()).map(Path::to_path_buf);
        let ask = Ask { title: t!("text-open").into(), folder, ..Ask::default() };
        self.choose(Purpose::Open, ask);
    }

    fn choose(&mut self, purpose: Purpose, ask: Ask) {
        let Some(bus) = self.bus.clone() else {
            return self.fail(t!("text-no-bus").into());
        };
        let asks = self.asks.clone();
        self.listen = match purpose {
            Purpose::Open => chooser::open_many(&bus, &self.events, ask, move |paths| {
                asks.send(Do::Chosen(purpose, paths));
            }),
            Purpose::SaveAs { .. } | Purpose::Export { .. } => chooser::save(&bus, &self.events, ask, move |path| {
                asks.send(Do::Chosen(purpose, path.into_iter().collect()));
            }),
        };
        if self.listen.is_none() {
            self.fail(t!("text-chooser-failed").into());
        }
    }

    pub(super) fn chosen(&mut self, purpose: Purpose, paths: Vec<PathBuf>) {
        self.listen = None;
        self.choosing = false;
        if paths.is_empty() && chooser::failed() {
            if matches!(purpose, Purpose::SaveAs { closing: true, .. }) {
                self.stop_closing();
            }
            return self.fail(chooser::did_not_answer().into());
        }
        match purpose {
            Purpose::Open => {
                for p in paths {
                    self.open_path(&p);
                }
            }
            Purpose::SaveAs { id, closing } => self.save_as_chosen(id, closing, paths),
            Purpose::Export { id, pdf } => self.export_chosen(id, pdf, paths),
        }
    }

    fn save_as_chosen(&mut self, id: u64, closing: bool, paths: Vec<PathBuf>) {
        let Some(i) = self.tabs.iter().position(|t| t.id == id) else { return };
        let Some(path) = paths.into_iter().next() else {
            if closing {
                self.stop_closing();
            }
            return;
        };
        let before = (self.tabs[i].path.replace(path.clone()), self.tabs[i].markdown);
        if is_markdown(&path) {
            self.tabs[i].markdown = true;
        }
        if self.write(i) {
            if closing {
                self.closing.retain(|c| *c != id);
                self.drop_tab(i);
                self.next_close();
            }
        } else {
            (self.tabs[i].path, self.tabs[i].markdown) = before;
            if closing {
                self.stop_closing();
            }
        }
    }

    fn export_chosen(&mut self, id: u64, pdf: bool, paths: Vec<PathBuf>) {
        let Some(tab) = self.tabs.iter().find(|t| t.id == id) else { return };
        let Some(path) = paths.into_iter().next() else { return };
        let text = tab.pane.borrow().doc.text().to_string();
        let name = tab.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| t!("text-untitled").into());
        if pdf {
            let asks = self.asks.clone();
            let state = self.dir.clone();
            std::thread::spawn(move || {
                asks.send(Do::Exported(export_pdf(&text, &name, &path, &state)));
            });
        } else {
            let blocks = markdown::parse(&text);
            let title = markdown::title(&blocks).unwrap_or(name);
            let done = store::write_document(&path, markdown::html::to_html(&blocks, &title).as_bytes(), &self.dir).map_err(|e| t!("text-cannot-write", name = path.display().to_string(), reason = e.to_string()));
            self.exported(done);
        }
    }

    pub(super) fn export(&mut self, pdf: bool) {
        let tab = &self.tabs[self.active];
        if !tab.markdown {
            return;
        }
        let folder = tab.path.as_ref().and_then(|p| p.parent()).map(Path::to_path_buf).or_else(|| Some(files::home()));
        let stem = tab.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| t!("text-untitled").into());
        let name = Some(format!("{stem}.{}", if pdf { "pdf" } else { "html" }));
        let title = if pdf { t!("text-export-pdf") } else { t!("text-export-html") };
        let ask = Ask { title: title.into(), folder, name, ..Ask::default() };
        self.choose(Purpose::Export { id: tab.id, pdf }, ask);
    }

    pub(super) fn exported(&mut self, done: Result<(), String>) {
        match done {
            Ok(()) => {
                self.said = None;
                self.noted_to("export");
            }
            Err(e) => self.fail(e),
        }
    }

    pub(super) fn raw(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.active) {
            tab.shown = false;
        }
    }

    pub(super) fn shown(&self) -> bool {
        self.tabs.get(self.active).is_some_and(|t| t.shown)
    }

    pub(super) fn over(&self) -> bool {
        let tab = &self.tabs[self.active];
        if tab.shown { tab.finished.borrow().over } else { tab.pane.borrow().over }
    }

    pub(super) fn save(&mut self, i: usize, closing: bool) -> bool {
        if self.tabs[i].path.is_none() {
            self.save_as(i, closing);
            return false;
        }
        self.tabs[i].changed = false;
        let path = self.tabs[i].path.clone();
        if let Some(now) = path.as_deref().and_then(store::stamp) {
            if self.tabs[i].stamp != Some(now) {
                self.tabs[i].changed = true;
                return false;
            }
        }
        self.write(i)
    }

    pub(super) fn save_anyway(&mut self, i: usize) {
        self.tabs[i].changed = false;
        self.write(i);
    }

    pub(super) fn reload(&mut self, i: usize) {
        let Some(path) = self.tabs[i].path.clone() else { return };
        let stamp = store::stamp(&path);
        let bytes = match store::read_document(&path) {
            Ok(b) => b,
            Err(e) => return self.fail(t!("text-cannot-read", name = path.display().to_string(), reason = e.to_string())),
        };
        let (text, form) = enc::read(&bytes);
        let tab = &mut self.tabs[i];
        tab.form = form;
        tab.stamp = stamp;
        tab.changed = false;
        {
            let mut pane = tab.pane.borrow_mut();
            let len = pane.doc.len();
            pane.doc.select(0, len);
            pane.doc.put(&text, doc::Kind::Other);
            pane.doc.go(0, false);
            pane.doc.mark_saved();
            pane.show_caret();
        }
        self.said = None;
        self.focus_editor();
        self.keep();
    }

    pub(super) fn save_as(&mut self, i: usize, closing: bool) {
        let tab = &self.tabs[i];
        let folder = tab.path.as_ref().and_then(|p| p.parent()).map(Path::to_path_buf).or_else(|| Some(files::home()));
        let name = Some(tab.path.as_ref().map(|_| tab.name()).unwrap_or_else(|| t!("text-untitled-file").into()));
        let ask = Ask { title: t!("text-save-as").into(), folder, name, ..Ask::default() };
        self.choose(Purpose::SaveAs { id: tab.id, closing }, ask);
    }

    fn write(&mut self, i: usize) -> bool {
        let Some(path) = self.tabs[i].path.clone() else { return false };
        let form = self.tabs[i].form;
        let bytes = {
            let pane = self.tabs[i].pane.borrow();
            enc::write(pane.doc.text(), &form)
        };
        let bytes = match bytes {
            Ok(b) => b,
            Err(c) => {
                self.fail(t!("text-encoding-cannot-write", c = c.to_string(), encoding = form.enc.name()));
                return false;
            }
        };
        if let Err(e) = store::write_document(&path, &bytes, &self.dir) {
            self.fail(t!("text-cannot-save", name = path.display().to_string(), reason = e.to_string()));
            return false;
        }
        self.tabs[i].pane.borrow_mut().doc.mark_saved();
        self.tabs[i].stamp = store::stamp(&path);
        self.tabs[i].changed = false;
        self.remember(&path);
        self.said = None;
        self.noted_to("status");
        self.keep();
        true
    }

    pub(super) fn fail(&mut self, words: String) {
        self.said = Some(words);
    }

    pub(super) fn noted_to(&mut self, at: &'static str) {
        self.turn = self.turn.wrapping_add(1);
        self.noted = Some((at, self.turn));
        let (asks, turn) = (self.asks.clone(), self.turn);
        self.events.after(NOTE_FOR, move || {
            asks.send(Do::NoteGone(turn));
        });
    }

    pub(super) fn noted_at<M: 'static>(&self, at: &str) -> Option<Node<M>> {
        self.noted.filter(|t| t.0 == at && matches!(at, "status" | "export" | "settings")).map(|_| note(t!("text-done")))
    }

    pub(super) fn read_as(&mut self, i: usize) {
        let Some(enc) = enc::OFFERED.get(i).and_then(|n| enc::by_name(n)) else { return };
        let tab = &mut self.tabs[self.active];
        let Some(path) = tab.path.clone() else { return };
        let bytes = match store::read_document(&path) {
            Ok(b) => b,
            Err(e) => return self.fail(t!("text-cannot-read", name = path.display().to_string(), reason = e.to_string())),
        };
        let (text, form) = enc::read_as(&bytes, enc);
        tab.form = form;
        let mut pane = tab.pane.borrow_mut();
        let was = pane.doc.modified();
        let len = pane.doc.len();
        pane.doc.select(0, len);
        pane.doc.put(&text, doc::Kind::Other);
        pane.doc.go(0, false);
        if !was {
            pane.doc.mark_saved();
        }
        pane.show_caret();
        drop(pane);
        self.view = View::Edit;
        self.focus_editor();
        self.noted_to("status");
    }

    pub(super) fn save_in(&mut self, i: usize) {
        let Some(enc) = enc::OFFERED.get(i).and_then(|n| enc::by_name(n)) else { return };
        let tab = &mut self.tabs[self.active];
        if tab.form.enc != enc {
            let utf16 = enc == encoding_rs::UTF_16LE || enc == encoding_rs::UTF_16BE;
            tab.form = Form { enc, bom: utf16, crlf: tab.form.crlf };
            tab.pane.borrow_mut().doc.mark_unsaved();
        }
        self.view = View::Edit;
        self.focus_editor();
        self.noted_to("status");
    }
}

fn export_pdf(text: &str, name: &str, path: &Path, state: &Path) -> Result<(), String> {
    let blocks = markdown::parse(text);
    let title = markdown::title(&blocks).unwrap_or_else(|| name.to_string());
    let fonts = crate::toolkit::text::Fonts::new();
    let bytes = pdf::render(&fonts, &blocks, &title);
    store::write_document(path, &bytes, state).map_err(|e| t!("text-cannot-write", name = path.display().to_string(), reason = e.to_string()))
}

pub(super) fn is_markdown(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "md" | "markdown" | "mdown" | "mkd"))
}
