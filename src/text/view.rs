//! Drawing the window: the tab row, the bars, the editor, the status row and the pages.

use crate::toolkit::input::{Cursor, Input, Keysym, Mods};
use crate::toolkit::widget::{Length, MenuRow, Node, Tone, button, card, choice, column, drawing, entry, even_grid, grid, item, label, list, main_word, note, one_of, row, scroll, slider, space, switch, title, word_button};
use crate::toolkit::{t, theme};
use super::{Answer, Bar, Msg, SIZE, TAB_NAME, TAB_WIDTH, Tab, Text, View, cut, dictionaries, dropped, enc, spell, store, takes};

impl Text {
    pub(super) fn tabs_row(&self) -> Node<Msg> {
        let mut tabs = self.tab_cards();
        let tools = self.tool_row();

        let width = match self.pane().borrow().width() {
            w if w > 0.0 => w,
            _ => SIZE.0 as f32 - 2.0 * theme::sheet().gap,
        };
        let cols = (((width + theme::sheet().gap) / (TAB_WIDTH + theme::sheet().gap)).floor() as usize).max(1);
        while tabs.len() % cols != 0 {
            tabs.push(space());
        }
        column(vec![tools, self.changed_line(), even_grid(TAB_WIDTH, tabs)]).gap(8.0)
    }

    fn tab_cards(&self) -> Vec<Node<Msg>> {
        self.tabs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let mut name = cut(&t.name(), TAB_NAME);
                if t.unsaved() {
                    name.push_str(" *");
                }
                let close = button("window-close-symbolic", t!("text-close-tab", name = t.name())).flat().on_press(Msg::CloseTab(i));
                let tab = card(row(vec![label(name), space(), close]).gap(4.0)).on_press(Msg::Tab(i));
                if i == self.active { tab } else { tab.flat() }
            })
            .collect()
    }

    fn tool_row(&self) -> Node<Msg> {
        let pane = self.pane().borrow();
        let tab = &self.tabs[self.active];
        let mut tools = vec![
            button("document-new-symbolic", t!("text-new-key")).on_press(Msg::New),
            button("document-open-symbolic", t!("text-open-key")).on_press(Msg::Open),
            button("document-open-recent-symbolic", t!("text-recent-files")).on_press(Msg::Show(View::Recent)).on(self.view == View::Recent),

            word_button(t!("text-save")).spoken(t!("text-save-key")).on_press(Msg::Save),
            word_button(t!("text-save-as")).spoken(t!("text-save-as-key")).on_press(Msg::SaveAs),
            button("edit-undo-symbolic", t!("text-undo-key")).on_press(Msg::Undo).off(!pane.doc.can_undo()),
            button("edit-redo-symbolic", t!("text-redo-key")).on_press(Msg::Redo).off(!pane.doc.can_redo()),
            button("edit-find-symbolic", t!("text-find-key")).on_press(Msg::Bar(Bar::Find)).on(self.view == View::Edit && self.bar == Bar::Find),
            button("edit-find-replace-symbolic", t!("text-find-replace-key")).on_press(Msg::Bar(Bar::Replace)).on(self.view == View::Edit && self.bar == Bar::Replace),
            button("go-jump-symbolic", t!("text-go-to-line-key")).on_press(Msg::Bar(Bar::Goto)).on(self.view == View::Edit && self.bar == Bar::Goto),
        ];
        if tab.markdown {
            tools.push(button("view-reveal-symbolic", t!("text-finished-key")).on_press(Msg::Finished).on(self.view == View::Edit && tab.shown));
            tools.push(button("text-html-symbolic", t!("text-export-html")).on_press(Msg::Export(false)));
            tools.push(button("document-export-symbolic", t!("text-export-pdf")).on_press(Msg::Export(true)));
            if let Some(t) = self.noted_at("export") {
                tools.push(t);
            }
        }
        tools.extend([
            space(),
            button("view-wrapped-symbolic", t!("text-wrap")).on_press(Msg::Wrap(!self.settings.wrap)).on(self.settings.wrap),
            button("view-list-ordered-symbolic", t!("text-line-numbers")).on_press(Msg::Numbers(!self.settings.numbers)).on(self.settings.numbers),
            button("emblem-system-symbolic", t!("text-settings")).on_press(Msg::Show(View::Settings)).on(self.view == View::Settings),
        ]);
        row(tools).gap(4.0)
    }

    fn changed_line(&self) -> Node<Msg> {
        if self.tabs[self.active].changed {
            row(vec![
                label(t!("text-changed-elsewhere")),
                word_button(t!("text-save-anyway")).on_press(Msg::SaveAnyway),
                word_button(t!("text-reload")).on_press(Msg::Reload),
            ])
            .gap(8.0)
        } else {
            row(vec![space()])
        }
        .height(Length::Fixed(theme::sheet().word_button.1))
    }

    pub(super) fn bar_row(&self) -> Option<Node<Msg>> {
        let close = button("window-close-symbolic", t!("text-close-bar")).on_press(Msg::Back);
        let bar = match self.bar {
            Bar::None => return None,
            Bar::Find => {
                let mut items = self.finding();
                items.push(close);
                row(items)
            }
            Bar::Replace => {
                let mut items = self.finding();
                items.push(entry(self.with.clone(), t!("text-replace-with"), Msg::WithText).key("with").on_submit(Msg::ReplaceOne));
                items.push(word_button(t!("text-replace")).spoken(t!("text-replace-spoken")).on_press(Msg::ReplaceOne));
                items.push(word_button(t!("text-replace-all")).spoken(t!("text-replace-all-spoken")).on_press(Msg::ReplaceAll));
                if let Some(t) = self.noted_at("bar") {
                    items.push(t);
                }
                items.push(close);
                row(items)
            }
            Bar::Spell => self.spell_row(close)?,
            Bar::Goto => {
                let lines = self.pane().borrow().doc.lines();
                row(vec![
                    label(t!("text-go-to-line")),
                    self.grabbing(entry(self.goto.clone(), t!("text-goto-placeholder", lines = lines), Msg::GotoText).key("goto").on_submit(Msg::Goto)),
                    note(self.found.clone().unwrap_or_else(|| t!("text-of-lines", lines = lines))),
                    button("go-jump-symbolic", t!("text-go")).on_press(Msg::Goto),
                    close,
                ])
            }
        };
        Some(bar.gap(8.0))
    }

    fn grabbing(&self, n: Node<Msg>) -> Node<Msg> {
        if self.grab_bar { n.grab(self.grab) } else { n }
    }

    fn found_note(&self) -> Node<Msg> {
        let words = match &self.found {
            Some(w) => w.clone(),
            None if self.find.is_empty() => String::new(),
            None => {
                let pane = self.pane().borrow();
                let sel = pane.doc.selection();
                match (pane.marks.len(), pane.marks.iter().position(|m| *m == sel)) {
                    (0, _) => t!("text-none-found").into(),
                    (n, Some(at)) => t!("text-of-found", at = at + 1, n = n),
                    (n, None) => t!("text-found", n = n),
                }
            }
        };
        note(words)
    }

    fn finding(&self) -> Vec<Node<Msg>> {
        vec![
            self.grabbing(entry(self.find.clone(), t!("text-find"), Msg::FindText).key("find").on_submit(Msg::Next(true))),
            label(t!("text-match-case")),
            switch(self.how.case, t!("text-match-case"), Msg::Case),
            label(t!("text-whole-words")),
            switch(self.how.whole, t!("text-whole-words"), Msg::Whole),
            self.found_note(),
            button("go-up-symbolic", t!("text-previous-key")).on_press(Msg::Next(false)),
            button("go-down-symbolic", t!("text-next-key")).on_press(Msg::Next(true)),
        ]
    }

    fn spell_row(&self, close: Node<Msg>) -> Option<Node<Msg>> {
        let s = self.spelling.as_ref()?;
        let said = if s.added {
            t!("text-spell-added")
        } else if s.offers.is_empty() {
            t!("text-spell-none")
        } else {
            t!("text-spell-offers")
        };
        let mut items = vec![label(format!("\u{2018}{}\u{2019}", s.word)), note(said)];
        if let Some(t) = self.noted_at("spell") {
            items.push(t);
        }
        items.push(space());
        if !s.added && !s.offers.is_empty() {
            items.push(button("tools-check-spelling-symbolic", t!("text-use-spelling")).on_press(Msg::UseWord));
        }
        items.push(button("list-add-symbolic", t!("text-add-word")).on_press(Msg::AddWord).off(s.added));
        items.push(close);

        Some(if s.added || s.offers.is_empty() {
            row(items)
        } else {
            column(vec![row(items), self.grabbing(choice(s.offers.clone(), s.pick, t!("text-suggestions"), Msg::Pick).key("offers"))])
        })
    }

    pub(super) fn editor(&self) -> Node<Msg> {
        let tab = &self.tabs[self.active];
        if tab.shown {
            return self.finished_look(tab);
        }
        let pane = self.pane().clone();
        let paint = pane.clone();
        let place = pane.clone();
        let menu = pane.clone();
        let node = drawing((0.0, 0.0), t!("text-title"), move |canvas, pen| paint.borrow_mut().paint(canvas, pen))
            .key(format!("editor-{}", self.tabs[self.active].id))
            .menu(move |x, y| {

                let key = |sym| Msg::Key(Input::Key { sym, text: None, mods: Mods { ctrl: true, ..Mods::default() }, repeat: false });
                let mut rows = vec![
                    MenuRow::send("edit-cut-symbolic", t!("text-cut"), key(Keysym::x)).keys("Ctrl+X"),
                    MenuRow::send("edit-copy-symbolic", t!("text-copy"), key(Keysym::c)).keys("Ctrl+C"),
                    MenuRow::send("edit-paste-symbolic", t!("text-paste"), key(Keysym::v)).keys("Ctrl+V"),
                    MenuRow::send("edit-select-all-symbolic", t!("text-select-all"), key(Keysym::a)).keys("Ctrl+A"),
                ];
                let pane = menu.borrow();
                if pane.misspelt_at(pane.hit(x, y)).is_some() {
                    rows.push(MenuRow::send("tools-check-spelling-symbolic", t!("text-check-spelling"), Msg::SpellHere).rule());
                }
                rows
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .on_pointer(Msg::Pointer)
            .on_hover()
            .on_drop_from_outside(|held| takes(held, true), dropped)
            .on_keys(|input| match input {
                Input::Key { sym, mods, .. } => {
                    let leave = *sym == Keysym::Escape
                        || *sym == Keysym::ISO_Left_Tab
                        || (*sym == Keysym::Tab && (mods.ctrl || mods.alt));
                    (!leave).then(|| Msg::Key(input.clone()))
                }
                Input::Composed { .. } | Input::Focus(_) => Some(Msg::Key(input.clone())),
                _ => None,
            })
            .cursor_with(move |x, _| if x < place.borrow().text_left() || x > place.borrow().width() - 14.0 { Cursor::Arrow } else { Cursor::Text });
        if self.grab_bar { node } else { node.grab(self.grab) }
    }

    fn finished_look(&self, tab: &Tab) -> Node<Msg> {
        let (paint, place, pane) = (tab.finished.clone(), tab.finished.clone(), tab.pane.clone());
        let node = drawing((0.0, 0.0), t!("text-finished-look"), move |canvas, pen| {
            let pane = pane.borrow();
            paint.borrow_mut().paint(canvas, pen, pane.doc.text(), pane.doc.version, &pane.look);
        })
        .key(format!("finished-{}", tab.id))
        .width(Length::Fill)
        .height(Length::Fill)
        .on_pointer(Msg::Pointer)
        .on_hover()
        .on_drop_from_outside(|held| takes(held, false), dropped)
        .menu({
            let all = tab.pane.clone();
            move |_, _| {
                let text = all.borrow().doc.text().to_string();
                vec![MenuRow::copy(text).words(t!("text-copy-all"))]
            }
        })
        .on_keys(|input| match input {
            Input::Key { sym, mods, .. } if !mods.ctrl && !mods.alt && !mods.logo && !matches!(*sym, Keysym::Tab | Keysym::ISO_Left_Tab | Keysym::Escape) => {
                Some(Msg::Key(input.clone()))
            }
            Input::Key { sym: Keysym::c | Keysym::C, mods, .. } if mods.ctrl => Some(Msg::CopyAll),
            _ => None,
        })
        .cursor_with(move |x, y| if place.borrow().link_at(x, y).is_some() { Cursor::Hand } else { Cursor::Arrow });
        node.grab(self.grab)
    }

    pub(super) fn status_row(&self) -> Node<Msg> {
        let tab = &self.tabs[self.active];
        let pane = tab.pane.borrow();
        let (line, col) = pane.doc.line_col(pane.doc.caret);
        let sel = pane.doc.selection();
        let counts = {
            let mut cache = self.counted.borrow_mut();
            match &*cache {
                Some((id, v, r, c)) if *id == tab.id && *v == pane.doc.version && *r == sel => *c,
                _ => {
                    let range = if sel.is_empty() { 0..pane.doc.len() } else { sel.clone() };
                    let c = pane.doc.counts(range);
                    *cache = Some((tab.id, pane.doc.version, sel.clone(), c));
                    c
                }
            }
        };
        let mut items = vec![
            label(t!("text-line-column", line = line, col = col)),
            note(if sel.is_empty() { t!("text-counts", words = counts.words, letters = counts.letters) } else { t!("text-counts-selected", words = counts.words, letters = counts.letters) }),
            space(),
        ];
        if let Some(said) = &self.said {
            items.push(label(said.clone()).tone(Tone::Problem));
        }
        if let Some(t) = self.noted_at("status") {
            items.push(t);
        }
        if tab.form.crlf {
            items.push(note(t!("text-windows-endings")));
        }
        let enc = card(label(tab.form.enc.name())).chip().on_press(Msg::Show(View::Encodings)).width(Length::Shrink);
        items.push(enc);
        row(items).gap(12.0)
    }

    pub(super) fn recent_view(&self) -> Node<Msg> {
        let rows: Vec<_> = self
            .recent
            .iter()
            .map(|p| {
                let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let folder = p.parent().map(|f| f.display().to_string()).unwrap_or_default();
                item(name).icon("text-x-generic-symbolic").detail(folder)
            })
            .collect();
        let body = if rows.is_empty() { note(t!("text-files-here")) } else { list(rows, None, Msg::Recent).height(Length::Fill) };
        column(vec![
            row(vec![
                title(t!("text-recent-files")),
                space(),
                crate::toolkit::widget::remove("edit-clear-all-symbolic", t!("text-clear-list")).key("forget-recent").on_press(Msg::ForgetRecent)
                    .off(self.recent.is_empty()),
                button("go-previous-symbolic", t!("text-back")).on_press(Msg::Back),
            ])
            .gap(4.0),
            body,
        ])
        .gap(12.0)
        .height(Length::Fill)
    }

    pub(super) fn settings_view(&self) -> Node<Msg> {
        let s = &self.settings;
        let families = self.pane().borrow().families.clone();
        let mut fonts = vec![item(t!("text-own-font"))];
        fonts.extend(families.iter().map(|f| item(f.clone())));
        let chosen = if s.font.is_empty() { Some(0) } else { families.iter().position(|f| *f == s.font).map(|i| i + 1) };
        let langs = dictionaries();
        let lang = langs.iter().position(|l| *l == s.lang).map_or(0, |i| i + 1);
        let auto = spell::pick(&langs, "", spell::system_lang().as_deref()).unwrap_or_default();
        let mut lang_items = vec![t!("text-automatic-language", lang = auto)];
        lang_items.extend(langs.iter().cloned());
        let mut head = vec![title(t!("text-settings-title")), space()];
        if let Some(t) = self.noted_at("settings") {
            head.push(t);
        }
        head.push(button("go-previous-symbolic", t!("text-back")).on_press(Msg::Back));
        let line = |words: &str, control: Node<Msg>| row(vec![label(words.to_string()), space(), control]).gap(12.0);
        let starting = card(
            column(vec![
                title(t!("text-starting")),
                line(
                    t!("text-when-opens"),
                    choice(vec![t!("text-reopen").into(), t!("text-start-blank").into()], if s.reopen { 0 } else { 1 }, t!("text-when-opens"), Msg::Reopen),
                ),
                note(t!("text-keep-note")),
            ])
            .gap(10.0),
        );
        let showing = card(
            column(vec![
                title(t!("text-showing")),
                line(t!("text-wrap"), switch(s.wrap, t!("text-wrap"), Msg::Wrap)),
                line(t!("text-line-numbers"), switch(s.numbers, t!("text-line-numbers"), Msg::Numbers)),
                row(vec![label(t!("text-size")), slider(s.size, store::SIZE_MIN, store::SIZE_MAX, 1.0, t!("text-text-size"), Msg::Size), note(format!("{}", s.size))]).gap(12.0),
            ])
            .gap(10.0),
        );
        let spelling = card(
            column(vec![
                title(t!("text-spelling")),
                line(t!("text-check-spelling"), switch(s.spell, t!("text-check-spelling"), Msg::Spell)),
                if langs.is_empty() {
                    note(t!("text-no-hunspell"))
                } else {
                    line(t!("text-language"), one_of(lang_items, Some(lang), t!("text-spelling-language"), Msg::Lang).off(!s.spell))
                },
            ])
            .gap(10.0),
        );
        let font = card(
            column(vec![title(t!("text-font")), scroll("fonts", list(fonts, chosen, Msg::Font)).height(Length::Fixed(300.0))]).gap(10.0),
        );
        column(vec![row(head).gap(4.0), scroll("settings", column(vec![grid(320.0, vec![starting, showing, spelling, font]).width(Length::Fill)]))])
            .gap(12.0)
            .height(Length::Fill)
    }

    pub(super) fn encodings_view(&self) -> Node<Msg> {
        let tab = &self.tabs[self.active];
        let names = || enc::OFFERED.iter().map(|n| item(n.to_string())).collect::<Vec<_>>();
        let now = enc::OFFERED.iter().position(|n| enc::by_name(n) == Some(tab.form.enc));
        let read = card(
            column(vec![
                title(t!("text-read-as")),
                if tab.path.is_some() {
                    list(names(), now, Msg::ReadAs).height(Length::Fill)
                } else {
                    note(t!("text-no-file"))
                },
            ])
            .gap(10.0)
            .height(Length::Fill),
        );
        let save = card(
            column(vec![
                title(t!("text-save-in")),
                note(t!("text-next-save")),
                list(names(), now, Msg::SaveIn).height(Length::Fill),
            ])
            .gap(10.0)
            .height(Length::Fill),
        );
        column(vec![
            row(vec![title(t!("text-encoding", name = tab.form.enc.name())), space(), button("go-previous-symbolic", t!("text-back")).on_press(Msg::Back)]).gap(4.0),
            row(vec![read, save]).gap(12.0).height(Length::Fill),
        ])
        .gap(12.0)
        .height(Length::Fill)
    }

    pub(super) fn closing_view(&self) -> Node<Msg> {
        let tab = &self.tabs[self.active];
        let what = match &tab.path {
            Some(p) => t!("text-changes-not-saved", name = p.display().to_string()),
            None => t!("text-not-saved").into(),
        };
        let left = self.closing.len().saturating_sub(1);
        let mut words = vec![title(t!("text-save-named", name = tab.name())), label(what)];
        if left > 0 {
            words.push(note(t!("text-more-after", left = left)));
        }
        column(vec![
            card(
                column(vec![
                    column(words).gap(8.0),
                    row(vec![

                        main_word(t!("text-save")).on_press(Msg::Answer(Answer::Save)),
                        button("window-close-symbolic", t!("text-close-no-save")).on_press(Msg::Answer(Answer::Discard)),
                        button("go-previous-symbolic", t!("text-keep-open")).on_press(Msg::Answer(Answer::Keep)).grab(self.grab),
                    ])
                    .gap(8.0),
                ])
                .gap(16.0),
            )
            .width(Length::Fill),
            space(),
        ])
        .height(Length::Fill)
    }
}
