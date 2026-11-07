//! The chooser as it is drawn.

use crate::common::kinds::{icon_for, parts, shown_name, when_text};
use crate::toolkit::{t, theme};
use crate::toolkit::widget::{button, card, choice, column, drawing, entry, form_label, item, label, layers, list, main_action, main_word, note, one_of, placed, row, scroll, space, switch, typing_over, word_button, InPlace, Length, Node, Pointer, Rect, Tone};

use super::{lent, not_answering, Answer, Chooser, Clash, Entry, Mode, Msg, NAME_BOX, SIDEBAR};

const BUTTON_GAP: f32 = 8.0;

impl Chooser {
    pub(super) fn row(&self, e: &Entry) -> crate::toolkit::widget::Item {
        let kind = if e.folder { None } else { self.mime.of(&e.name) };
        let icon = if e.folder { "folder-symbolic" } else { icon_for(kind.as_deref()) };
        let detail = if e.folder { when_text(e.when) } else { format!("{}, {}", crate::disks::size_text(e.size as f64), when_text(e.when)) };
        let row = item(e.name.clone()).icon(icon).detail(detail);
        if e.folder { row.folder() } else { row }
    }

    pub(super) fn drawn(&self) -> Node<Msg> {
        let room = theme::compact_room();
        let places = self.places_panel(room.side_row);
        let work = self.work_panel(room.row, room.gap);
        let all = vec![self.top_row(room.gap), row(vec![places, work]).gap(theme::sheet().panel_gap).height(Length::Fill)];
        let all = column(all).gap(theme::sheet().panel_gap / 2.0).padding(theme::sheet().panel_gap).on_escape(Msg::Cancel);
        let all = match &self.menu {
            Some((menu, _)) => menu.over(all, Msg::Pop),
            None => all,
        };
        self.with_questions(all)
    }

    fn top_row(&self, gap: f32) -> Node<Msg> {
        let path_bar: Node<Msg> = match self.folder() {
            Some(f) => {
                let names: Vec<String> = parts(f, &self.home).into_iter().map(|(name, _)| name).collect();
                let last = names.len().saturating_sub(1);
                choice(names, last, t!("screenshot-chooser-folder"), Msg::Part)
            }
            None => choice(vec![t!("screenshot-chooser-recent").into()], 0, t!("screenshot-chooser-folder"), |_| Msg::Part(0)),
        };
        let mut top = vec![path_bar, space()];
        if self.folder().is_some() {
            top.push(button("folder-new-symbolic", t!("screenshot-chooser-new-folder")).on_press(Msg::NewFolder));
        }
        let (eye, eye_tip) = if self.hidden {
            ("view-conceal-symbolic", t!("screenshot-chooser-hide-hidden"))
        } else {
            ("view-reveal-symbolic", t!("screenshot-chooser-show-hidden"))
        };
        top.push(button(eye, eye_tip).on_press(Msg::Hidden));
        row(top).gap(2.0 * gap)
    }

    fn places_panel(&self, side_row: f32) -> Node<Msg> {
        let here = self.places.iter().position(|p| p.at == self.at);
        let places = list(self.places.iter().map(|p| if p.folder { item(p.label.clone()).icon(p.icon).folder() } else { item(p.label.clone()).icon(p.icon) }).collect(), here, Msg::Place).here().row_height(side_row);
        card(scroll("places", column(vec![card(places).snug()]).padding(theme::sheet().sidebar_room)))
            .snug().side().width(Length::Fixed(SIDEBAR)).height(Length::Fill)
    }

    fn work_panel(&self, row_height: f32, gap: f32) -> Node<Msg> {
        let mut work = vec![self.contents(row_height)];
        if matches!(self.ask.mode, Mode::Save) {
            let name = entry(self.name.clone(), t!("screenshot-portal-name"), Msg::Name).on_submit(Msg::Submit);
            work.push(name.first());
        }
        work.push(row(self.bottom_row()).gap(BUTTON_GAP));
        card(column(work).gap(2.0 * gap).height(Length::Fill)).width(Length::Fill).height(Length::Fill)
    }

    fn contents(&self, row_height: f32) -> Node<Msg> {
        if !self.listed {
            let mut waiting = Vec::new();
            if self.late_place {
                waiting.push(note(not_answering()));
            }
            waiting.push(self.empty_room(row_height));
            return column(waiting).width(Length::Fill).height(Length::Fill);
        }
        if self.entries.is_empty() {
            return column(vec![note(t!("screenshot-chooser-empty")), self.empty_room(row_height)]).width(Length::Fill).height(Length::Fill);
        }
        let rows = self.entries.iter().map(|e| self.row(e)).collect();
        let l = list(rows, self.chosen, Msg::Choose).row_height(row_height).on_open(Msg::Open).on_menu(Msg::Menu).width(Length::Fill);
        let l = if matches!(self.ask.mode, Mode::Open { many: true, .. }) { l.many(self.picked.clone(), Msg::Pick) } else { l };
        let rows = typing_over(l, self.rename_box(row_height));
        scroll(format!("folder:{:?}", self.at), column(vec![rows, self.empty_room(row_height)]).gap(0.0).height(Length::Fill)).width(Length::Fill).height(Length::Fill)
    }

    fn bottom_row(&self) -> Vec<Node<Msg>> {
        let mut bottom: Vec<Node<Msg>> = Vec::new();
        if self.ask.filters.len() > 1 && !self.folders_only() {
            let names = self.ask.filters.iter().map(|f| f.name.clone()).collect();
            bottom.push(one_of(names, Some(self.ask.filter.unwrap_or(0)), t!("screenshot-chooser-kind"), Msg::Filter));
        }
        for (i, p) in self.ask.choices.iter().enumerate() {
            if p.options.is_empty() {
                bottom.push(row(vec![label(p.label.clone()), switch(p.value == "true", p.label.clone(), move |on| Msg::Toggle(i, on))]).gap(BUTTON_GAP));
            } else {
                let at = p.options.iter().position(|(id, _)| *id == p.value).unwrap_or(0);
                let words = p.options.iter().map(|(_, w)| w.clone()).collect();
                bottom.push(one_of(words, Some(at), p.label.clone(), move |j| Msg::Choice(i, j)));
            }
        }
        let taken = self.taken();
        if !self.failed.is_empty() {
            bottom.push(note(self.failed.clone()).tone(Tone::Problem));
        } else if self.late_save {
            bottom.push(note(not_answering()));
        } else if !self.said.is_empty() {
            bottom.push(note(self.said.clone()));
        } else if let Some(t) = &taken {
            bottom.push(note(t!("screenshot-chooser-there", name = shown_name(t))));
        }
        *self.offered.borrow_mut() = taken;
        bottom.push(space());
        bottom.push(button("window-close-symbolic", t!("screenshot-portal-cancel")).on_press(Msg::Cancel));
        let accept = if matches!(self.ask.mode, Mode::Save) || self.taken().is_some() {
            main_word(self.accept_words())
        } else {
            main_action(crate::common::no_tick::GO_ICON, self.accept_words())
        };
        bottom.push(accept.on_press(Msg::Accept).off(!self.ready()));
        bottom
    }

    fn with_questions(&self, all: Node<Msg>) -> Node<Msg> {
        if self.deleting.is_some() {
            return over(all, self.delete_question(), "dialog:delete");
        }
        match &self.asking {
            Some(clash) => over(all, self.question(clash), "dialog:clash"),
            None => all,
        }
    }

    fn delete_question(&self) -> Node<Msg> {
        card(column(vec![
            form_label(t!("screenshot-chooser-delete-ask")),
            row(vec![
                space(),
                button("window-close-symbolic", t!("screenshot-cancel")).on_press(Msg::DeleteNo).grab(self.questions),
                main_action(lent::SHRED_ICON, t!("screenshot-chooser-delete")).cannot_be_undone().on_press(Msg::DeleteSure),
            ]).gap(BUTTON_GAP).width(Length::Fill),
        ]).gap(12.0).width(Length::Fill)).width(Length::Fixed(360.0)).on_escape(Msg::DeleteNo)
    }

    fn question(&self, clash: &Clash) -> Node<Msg> {
        let head = if clash.folders {
            t!("screenshot-chooser-clash-folder", name = &clash.name)
        } else {
            t!("screenshot-chooser-clash-file", name = &clash.name)
        };
        let mut quiet = vec![
            button("media-skip-forward-symbolic", t!("screenshot-chooser-skip")).on_press(Msg::Answer(Answer::Skip)).grab(self.questions),
            button("edit-copy-symbolic", t!("screenshot-chooser-both")).on_press(Msg::Answer(Answer::Both)),
        ];
        let main = if clash.folders {
            if clash.replace {
                quiet.push(word_button(t!("screenshot-chooser-replace")).spoken(t!("screenshot-chooser-replace-spoken")).on_press(Msg::Answer(Answer::Replace)));
            }
            main_action(lent::MERGE_ICON, t!("screenshot-chooser-merge")).on_press(Msg::Answer(Answer::Merge))
        } else {
            main_word(t!("screenshot-chooser-replace")).on_press(Msg::Answer(Answer::Replace))
        };
        let rest = if clash.folders { t!("screenshot-chooser-rest-folders") } else { t!("screenshot-chooser-rest-files") };
        let mut buttons = vec![space()];
        buttons.extend(quiet);
        buttons.push(main);
        card(column(vec![
            form_label(head),
            row(vec![label(rest).width(Length::Fill), switch(self.same, t!("screenshot-chooser-same"), Msg::SameForRest)]).gap(BUTTON_GAP).width(Length::Fill),
            row(buttons).gap(BUTTON_GAP).width(Length::Fill),
        ]).gap(12.0).width(Length::Fill)).width(Length::Fixed(360.0)).on_escape(Msg::AskStop)
    }

    pub(super) fn rename_box(&self, row: f32) -> Option<InPlace<Msg>> {
        let r = self.renaming.as_ref()?;
        let i = self.entries.iter().position(|e| e.path == r.path)?;
        let sheet = theme::sheet();
        let lead = sheet.entry_padding + theme::icon() + sheet.gap - sheet.snug_padding;
        let tall = sheet.entry_height.min(row);
        let old = shown_name(&r.path);
        let name = entry(r.text.clone(), t!("screenshot-chooser-new-name", old = &old), Msg::RenameText).key(format!("rename:{}", r.path.display()))
            .grab(self.asked_rename).selects(0..lent::stem_len(&old, r.folder))
            .on_submit(Msg::RenameDone).on_escape(Msg::RenameStop).on_leave(Msg::RenameLeft);
        Some(InPlace { place: Rect { x: lead, y: i as f32 * row + (row - tall) / 2.0, w: NAME_BOX, h: tall }, entry: name, problem: r.why.clone() })
    }

    fn empty_room(&self, row: f32) -> Node<Msg> {
        drawing((0.0, row), "", |_, _| {})
            .on_pointer(|p| if matches!(p, Pointer::Menu(..)) { Msg::RoomMenu(false) } else { Msg::Still })
            .width(Length::Fill).height(Length::Fill)
    }

    pub(super) fn accept_words(&self) -> String {
        if self.taken().is_some() {
            return t!("screenshot-chooser-replace").into();
        }
        if let Some(w) = self.ask.accept.as_deref().filter(|w| !w.is_empty()) {
            return w.replace("__", "\u{0}").replace('_', "").replace('\u{0}', "_");
        }
        match self.ask.mode {
            Mode::Open { folder: true, .. } | Mode::SaveMany(_) => t!("screenshot-chooser-choose-folder"),
            Mode::Open { .. } => t!("screenshot-portal-open"),
            Mode::Save => t!("screenshot-portal-save"),
        }.into()
    }
}

fn over(window: Node<Msg>, question: Node<Msg>, stays: &'static str) -> Node<Msg> {
    let centred = placed(question.floats(), |(w, h), (aw, ah)| (((aw - w) / 2.0).max(0.0), ((ah - h) / 2.0).max(0.0))).stays(stays);
    layers(vec![window.off(true), centred])
}
