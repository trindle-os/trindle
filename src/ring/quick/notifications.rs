//! The notifications panel: the list, a notification shown whole, and what is pinned or forgotten.

use std::time::Duration;

use crate::common::no_tick::SHOWN_SECONDS;
use crate::ring::outside;
use crate::ring::crashes::hand_off;
use crate::ring::ctx::{Do, Msg};
use crate::ring::notes::icons::note_icon;
use crate::ring::notify::{self, Note};
use crate::settings::ui::icon;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, label, Length, main_action, Node, remove, row, scroll, title};

use super::{banded, Body, BOLD, line_of, m, PAGE_GAP, Quick, ROW_GAP, Well};
use super::clip_panel::{forget_one, pin_button, TRASH_ICON};

const NOTE_ICON: f32 = 20.0;

const WHOLE_ICON: f32 = 24.0;

pub(super) fn newest_first(mut all: Vec<Note>) -> Vec<Note> {
    all.sort_by_key(|n| !n.pinned);
    all
}

fn mute_button() -> Node<Msg> {
    let on = notify::showing();
    let (icon_name, tip) = if on {
        ("preferences-system-notifications-symbolic", t!("ring-quick-notes-on"))
    } else {
        ("notifications-disabled-symbolic", t!("ring-quick-notes-off"))
    };

    button(icon_name, tip).on_press(m(|_| {
        if let Err(e) = notify::set_showing(!notify::showing()) {
            eprintln!("trindle-ring: could not keep the notifications choice: {e}");
        }
    })).well()
}

impl Quick {
    pub(super) fn notes(&mut self) {
        let apps = self.ctx().apps();
        self.body = Body::Notes(newest_first(notify::list()).into_iter().map(|n| {
            let icon_name = note_icon(&apps, &n);
            (n, icon_name)
        }).collect());
    }

    pub(super) fn notes_view(&self, notes: &[(Note, String)]) -> Vec<Node<Msg>> {
        let mut out = Vec::new();
        if notes.is_empty() {
            out.push(label(t!("ring-quick-notes-empty")));
        } else {
            out.push(self.scrolling(banded(notes.iter().map(|(n, icon_name)| {
                let top = if n.summary.is_empty() { n.app.clone() } else { n.summary.clone() };
                let under = if n.body.is_empty() { n.when() } else { format!("{}  {}", n.when(), n.body.replace('\n', " ")) };
                let id = n.id;
                row(vec![
                    icon(icon_name.clone(), NOTE_ICON),
                    card(column(vec![line_of(top.clone(), false, BOLD), line_of(under.clone(), true, 400)]).gap(0.0)).flat()
                        .width(Length::Fill).on_press(m(move |q| q.whole(id))).whole(format!("{top}\n{under}")),
                    pin_button(n.pinned, m(move |q| q.note_pin(id))),
                    remove(TRASH_ICON, forget_one()).key(format!("forget-note-{id}")).on_press(m(move |q| q.note_forget(id))).well(),
                ]).gap(ROW_GAP)
            }).collect())));
        }
        let mut foot = vec![mute_button()];
        if !notes.is_empty() {
            foot.push(self.forget_all_button(|q| {
                notify::clear();
                q.refresh_now();
            }));
        }
        out.push(row(foot).gap(ROW_GAP).width(Length::Shrink));
        out
    }

    fn note_pin(&mut self, id: u32) {
        match &mut self.body {
            Body::Notes(notes) => {
                if let Some((n, _)) = notes.iter_mut().find(|(n, _)| n.id == id) {
                    n.pinned = !n.pinned;
                    notify::set_pinned(id, n.pinned);
                }
            }
            Body::Whole(n, _) if n.id == id => {
                n.pinned = !n.pinned;
                notify::set_pinned(id, n.pinned);
            }
            _ => {}
        }
    }

    fn note_forget(&mut self, id: u32) {
        notify::forget(id);
        if let Body::Notes(notes) = &mut self.body {
            notes.retain(|(n, _)| n.id != id);
        }
    }

    pub(super) fn whole(&mut self, id: u32) {
        let Some(n) = notify::list().into_iter().find(|n| n.id == id) else { return };
        let icon_name = note_icon(&self.ctx().apps(), &n);
        self.say("");
        self.ticked = false;
        self.body = Body::Whole(n, icon_name);
    }

    pub(super) fn whole_view(&self, n: &Note, icon_name: &str) -> Vec<Node<Msg>> {
        let top = row(vec![icon(icon_name.to_string(), WHOLE_ICON), line_of(format!("{}  {}", n.app, n.when()), true, 400)])
            .gap(ROW_GAP);
        let mut page = Vec::new();
        if !n.summary.is_empty() {
            page.push(title(n.summary.clone()));
        }
        if !n.body.is_empty() {
            page.push(label(n.body.clone()));
        }
        let id = n.id;
        let mut foot = vec![button("go-previous-symbolic", t!("ring-quick-notes-back")).on_press(m(|q| q.refresh_now())).well()];
        if !n.drive.is_empty() {
            let drive = n.drive.clone();
            foot.push(main_action("folder-open-symbolic", t!("ring-open")).on_press(m(move |q| q.ctx().tell(Do::OpenDrive(drive.clone())))));
        }
        if !n.allow.is_empty() {
            let plugin = n.allow.clone();
            foot.push(main_action(crate::plugins::ALLOW_ICON, crate::plugins::allow_word())
                .on_press(m(move |_| {
                    outside::allow_pressed(&plugin);
                })));
        }
        if !n.handoff.is_empty() {
            let text = n.handoff.clone();

            let (icon_name, tip) = if self.ticked { ("edit-copy-symbolic", t!("ring-quick-copied")) } else { ("edit-copy-symbolic", t!("ring-quick-copy-assistant")) };
            foot.push(button(icon_name, tip).on(self.ticked).on_press(m(move |q| {
                hand_off(&text);
                q.tick();
            })).well());
        }
        foot.push(pin_button(n.pinned, m(move |q| q.note_pin(id))));
        foot.push(remove(TRASH_ICON, forget_one()).key(format!("forget-whole-{id}")).on_press(m(move |q| {
            notify::forget(id);
            q.refresh_now();
        })).well());
        vec![top, scroll(format!("note:{id}"), column(page).gap(PAGE_GAP)).height(Length::Fixed(self.list_height)),
             row(foot).gap(ROW_GAP).width(Length::Shrink)]
    }

    fn tick(&mut self) {
        self.ticked = true;
        self.tick_gen = self.tick_gen.wrapping_add(1);
        let (b, at) = (self.back(), self.tick_gen);
        self.ctx().lp.after(Duration::from_secs(SHOWN_SECONDS.into()), move || b.then(move |q| {
            if q.tick_gen == at {
                q.ticked = false;
            }
        }));
    }
}
