//! Text as a Toolkit app: its messages, its drawing and its keys.

use super::enc::Form;
use crate::toolkit::dnd::Given;
use crate::toolkit::editing::doc::Doc;
use crate::toolkit::editing::editor::Did;
use crate::toolkit::input::{Input, Typing};
use crate::toolkit::t;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{Length, Node, Pointer, column};
use super::{Answer, Bar, Msg, Text, View, dictionaries, markdown, store};

impl App for Text {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Key(_) | Msg::Pointer(_) | Msg::Scroll(..) if self.shown() => self.finished_look_input(msg),
            Msg::Finished | Msg::Show(_) | Msg::Back => self.page_msg(msg),
            Msg::Key(_) | Msg::Pointer(_) | Msg::Scroll(..) | Msg::Dropped(..) | Msg::Undo | Msg::Redo | Msg::CopyAll => {
                self.editor_msg(msg)
            }
            Msg::New | Msg::Open | Msg::Save | Msg::SaveAs | Msg::SaveAnyway | Msg::Reload | Msg::Tab(_) | Msg::CloseTab(_)
            | Msg::NextTab(_) | Msg::Quit | Msg::Recent(_) | Msg::ForgetRecent | Msg::ReadAs(_) | Msg::SaveIn(_) | Msg::Answer(_)
            | Msg::Export(_) => self.file_msg(msg),
            Msg::Bar(_) | Msg::FindText(_) | Msg::WithText(_) | Msg::GotoText(_) | Msg::Case(_) | Msg::Whole(_) | Msg::Next(_)
            | Msg::ReplaceOne | Msg::ReplaceAll | Msg::Goto => self.find_msg(msg),
            Msg::Reopen(_) | Msg::Wrap(_) | Msg::Numbers(_) | Msg::Font(_) | Msg::Size(_) | Msg::Zoom(_) => self.look_msg(msg),
            Msg::Spell(_) | Msg::Lang(_) | Msg::SpellHere | Msg::Pick(_) | Msg::UseWord | Msg::AddWord => self.spelling_msg(msg),
        }
    }

    fn view(&self) -> Node<Msg> {
        let main = match self.view {
            View::Edit => self.editor(),
            View::Recent => self.recent_view(),
            View::Settings => self.settings_view(),
            View::Encodings => self.encodings_view(),
            View::Closing => self.closing_view(),
        };
        let mut parts = vec![self.tabs_row()];
        if self.view == View::Edit {
            if let Some(bar) = self.bar_row() {
                parts.push(bar);
            }
        }
        parts.push(main);
        parts.push(self.status_row());
        column(parts).gap(8.0).padding(12.0).height(Length::Fill)
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::PointerIn { x, y } | Input::PointerMove { x, y } | Input::Press { x, y, .. } => {
                self.at.set((*x, *y));
                None
            }
            Input::Scroll { x, y, steps } if self.view == View::Edit && self.over() => Some(Msg::Scroll(*x, *y, *steps)),
            Input::Key { sym, mods, .. } => self.shortcut(*sym, *mods),
            _ => None,
        }
    }

    fn typing(&self) -> Option<Typing> {
        if self.view != View::Edit || self.shown() {
            return None;
        }
        let pane = self.pane().borrow();
        if !pane.editing {
            return None;
        }
        let doc = &pane.doc;
        let line = doc.line_range(doc.line_of(doc.caret));
        let anchor = doc.anchor.clamp(line.start, line.end);
        let (x, y, w, h) = pane.caret_box;
        let (ox, oy) = pane.origin;
        Some(Typing {
            id: "editor".into(),
            text: doc.text()[line.clone()].to_string(),
            caret: doc.caret - line.start,
            anchor: anchor - line.start,
            area: ((ox + x) as i32, (oy + y) as i32, w.ceil() as i32, h.ceil() as i32),
            secret: false, purpose: Default::default(),
        })
    }
}

impl Text {

    fn finished_look_input(&mut self, msg: Msg) {
        match msg {
            Msg::Key(Input::Key { sym, .. }) => {
                self.tabs[self.active].finished.borrow_mut().key(sym);
            }
            Msg::Pointer(p) => {
                let link = self.tabs[self.active].finished.borrow_mut().pointer(p);
                if let Some(url) = link.filter(|u| markdown::html::safe_url(u)) {
                    crate::settings::ui::open_link(&url);
                }
            }
            Msg::Scroll(_, y, steps) => self.tabs[self.active].finished.borrow_mut().scroll(y, steps),
            _ => {}
        }
    }

    fn page_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Finished => {
                let tab = &mut self.tabs[self.active];
                if !tab.markdown {
                    return;
                }
                tab.shown = !(tab.shown && self.view == View::Edit);
                if tab.shown {
                    self.bar = Bar::None;
                    self.found = None;
                    self.view = View::Edit;
                    self.focus_editor();
                    let mut pane = self.pane().borrow_mut();
                    pane.marks.clear();
                    pane.editing = false;
                } else {
                    self.view = View::Edit;
                    self.focus_editor();
                }
            }
            Msg::Show(view) => {
                self.view = if self.view == view { View::Edit } else { view };
                if self.view == View::Edit {
                    self.focus_editor();
                } else {
                    self.pane().borrow_mut().editing = false;
                }
            }
            Msg::Back => {
                if self.view == View::Closing {
                    self.answer(Answer::Keep);
                } else if self.view != View::Edit {
                    self.view = View::Edit;
                    self.focus_editor();
                } else {
                    self.close_bar();
                }
            }
            _ => {}
        }
    }

    fn editor_msg(&mut self, msg: Msg) {
        match msg {
            Msg::CopyAll => {
                if let Some(w) = self.window.upgrade() {
                    w.copy(self.pane().borrow().doc.text());
                }
            }
            Msg::Key(input) => {
                let did = self.pane().borrow_mut().key(&input);
                if let Some(did) = did {
                    self.edited(did);
                }
            }
            Msg::Pointer(p) => {
                if let Pointer::Down(x, y) | Pointer::Move(x, y) | Pointer::Hover(x, y) | Pointer::Up(x, y) = p {
                    let at = self.at.get();
                    self.pane().borrow_mut().origin = (at.0 - x, at.1 - y);
                }
                if matches!(p, Pointer::Down(..)) && self.grab_bar {
                    self.grab_bar = false;
                }
                let did = self.pane().borrow_mut().pointer(p);
                self.edited(did);
            }

            Msg::Dropped(_, _, Given::Files(files)) => {
                for file in files {
                    self.open_path(&file);
                }
            }
            Msg::Dropped(x, y, Given::Text(text)) => {
                let mut pane = self.pane().borrow_mut();
                if pane.editing {
                    let at = pane.hit(x, y);
                    pane.doc.select(at, at);
                    pane.paste(&text);
                }
                drop(pane);
                self.edited(Did::Edited);
            }
            Msg::Dropped(_, _, Given::Nothing(why)) => self.fail(why.words().to_string()),
            Msg::Dropped(_, _, Given::Typed(..)) => {}
            Msg::Scroll(x, y, steps) => self.pane().borrow_mut().scroll(x, y, steps),
            Msg::Undo | Msg::Redo => {
                let mut pane = self.pane().borrow_mut();
                if matches!(msg, Msg::Undo) { pane.doc.undo() } else { pane.doc.redo() };
                pane.show_caret();
                drop(pane);
                self.edited(Did::Edited);
            }
            _ => {}
        }
    }

    fn file_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Export(pdf) => self.export(pdf),
            Msg::New => {
                let at = self.add_tab(None, Form::default(), Doc::new(String::new()));
                self.switch_to(at);
            }
            Msg::Open => self.choose_open(),
            Msg::Save => {
                self.save(self.active, false);
            }
            Msg::SaveAs => self.save_as(self.active, false),
            Msg::SaveAnyway => self.save_anyway(self.active),
            Msg::Reload => self.reload(self.active),
            Msg::Tab(i) => self.switch_to(i),
            Msg::CloseTab(i) => self.close_tab(i),
            Msg::NextTab(forward) => {
                let n = self.tabs.len();
                let i = if forward { (self.active + 1) % n } else { (self.active + n - 1) % n };
                self.switch_to(i);
            }
            Msg::Quit => self.quit(),
            Msg::Recent(i) => {
                if let Some(p) = self.recent.get(i).cloned() {
                    self.open_path(&p);
                }
            }
            Msg::ForgetRecent => {
                self.recent.clear();
                if let Err(e) = store::write_recent(&self.dir, &[]) {
                    self.fail(t!("text-list-not-cleared", reason = e.to_string()));
                }
            }
            Msg::ReadAs(i) => self.read_as(i),
            Msg::SaveIn(i) => self.save_in(i),
            Msg::Answer(a) => self.answer(a),
            _ => {}
        }
    }

    fn find_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Bar(bar) => {
                if self.bar == bar {
                    self.close_bar();
                } else {
                    self.open_bar(bar);
                }
            }
            Msg::FindText(t) => {
                self.find = t;
                self.found = None;
                self.search_from_here();
            }
            Msg::WithText(t) => self.with = t,
            Msg::GotoText(t) => {
                self.goto = t;
                self.found = None;
            }
            Msg::Case(on) => {
                self.how.case = on;
                self.search_from_here();
            }
            Msg::Whole(on) => {
                self.how.whole = on;
                self.search_from_here();
            }
            Msg::Next(forward) => {
                self.raw();
                if self.bar == Bar::None || self.bar == Bar::Goto {
                    if self.find.is_empty() {
                        self.open_bar(Bar::Find);
                        return;
                    }
                    self.bar = Bar::Find;
                }
                self.found = None;
                self.step(forward);
            }
            Msg::ReplaceOne => {
                self.found = None;
                self.replace_one();
            }
            Msg::ReplaceAll => self.replace_all(),
            Msg::Goto => self.go_to_line(),
            _ => {}
        }
    }

    fn look_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Reopen(i) => self.set(|s| s.reopen = i == 0),
            Msg::Wrap(on) => self.set(|s| s.wrap = on),
            Msg::Numbers(on) => self.set(|s| s.numbers = on),
            Msg::Font(i) => {
                let font = if i == 0 { String::new() } else { self.pane().borrow().families.get(i - 1).cloned().unwrap_or_default() };
                self.set(|s| s.font = font);
            }
            Msg::Size(size) => self.set_resting(|s| s.size = size.round().clamp(store::SIZE_MIN, store::SIZE_MAX)),
            Msg::Zoom(by) => {
                let size = if by == 0 { store::Settings::default().size } else { self.settings.size + by as f32 };
                self.set(|s| s.size = size.clamp(store::SIZE_MIN, store::SIZE_MAX));
            }
            _ => {}
        }
    }

    fn spelling_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Spell(on) => {
                self.set(|s| s.spell = on);
                self.spell_setup();
            }
            Msg::Lang(i) => {
                let lang = if i == 0 { Some(String::new()) } else { dictionaries().get(i - 1).cloned() };
                if let Some(lang) = lang {
                    self.set(|s| s.lang = lang);
                    self.spell_setup();
                }
            }
            Msg::SpellHere => {
                let pane = self.pane().borrow();
                let word = pane.misspelt_at(pane.doc.selection().end);
                drop(pane);
                match word {
                    Some(w) => {
                        self.pane().borrow_mut().doc.select(w.start, w.end);
                        self.spell_bar(w);
                    }
                    None if self.bar == Bar::Spell => self.close_bar(),
                    None => {}
                }
            }
            Msg::Pick(i) => {
                if let Some(s) = self.spelling.as_mut() {
                    s.pick = i;
                }
            }
            Msg::UseWord => self.use_word(),
            Msg::AddWord => self.add_word(),
            _ => {}
        }
    }
}
