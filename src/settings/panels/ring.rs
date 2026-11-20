//! Ring: how it opens, its notifications, clipboard history and voice; it only counts the favourites.

use crate::common::clipboard;
use crate::common::files::set_setting;
use crate::common::run::{self, spawn};
use crate::ring::notify;
use crate::settings::pictures;
use crate::settings::ui::{confirm, dropdown, group, headline, line, on, page, why, Kit, Msg, Page, INSTALL_ICON, not_saved};
use crate::store::updates::admin_watching;
use crate::toolkit::widget::{button, card, note, row, switch, Length, Node};
use crate::toolkit::t;

const VOICE: &str = "voice";
const INSTALL: &str = "voice-install";
const REMOVE: &str = "voice-remove";
const TOOLS: &str = "voice-tools-remove";

#[derive(Default)]
pub struct Ring {

    window_names: bool,
    showing: bool,
    long: String,
    corner: String,
    forget: String,
    voice_installed: bool,
    voice_on: bool,

    tools_left: bool,

    working: Option<&'static str>,

    favourites: usize,
    copied: (usize, usize),
}

const RING_ICON: &str = "view-app-grid-symbolic";

impl Ring {

    fn read(&mut self, kit: &Kit) {
        self.window_names = crate::common::files::setting("ring_window_names") != "no";
        self.showing = notify::showing();
        self.long = notify::how_long_setting();
        self.corner = notify::corner_setting();
        self.forget = clipboard::forget_setting();
        self.voice_installed = crate::voice::installed();
        self.voice_on = crate::voice::enabled();
        self.tools_left = self.voice_installed && !crate::voice::setup::tools_left().is_empty();
        self.favourites = crate::ring::store::favourites().len();
        let counted = kit.then_if::<Ring, (usize, usize)>(|me, _, copied| std::mem::replace(&mut me.copied, copied) != copied);
        run::on_thread(|| {
            let kept = clipboard::list();
            (kept.len(), kept.iter().filter(|i| i.pinned).count())
        }, counted);
    }

    fn sum_up(&self) -> String {
        t!("settings-ring-headline").to_string()
    }

    fn opening(&self, kit: &Kit) -> Node<Msg> {
        let names = line(t!("settings-ring-names"))
            .key("ring_window_names")
            .control(switch(self.window_names, t!("settings-ring-names"), |now| on::<Ring>(move |me, k| {
                if k.settle("ring_window_names", set_setting("ring_window_names", if now { "yes" } else { "no" }), not_saved()).is_some() {
                    me.window_names = now;
                }
            })))
            .node(kit);

        let n = self.favourites;
        let favourites = line(t!("settings-ring-favourites", n = n)).node(kit);
        group(t!("settings-ring-opening")).icon(RING_ICON)
            .about([t!("settings-ring-opening-about"), t!("settings-ring-names-about")].join(" "))
            .rows(vec![names, favourites])
            .node(kit)
    }

    fn notifications(&self, kit: &Kit) -> Node<Msg> {
        let show = line(t!("settings-ring-notify-show"))
            .key("notifications")
            .control(switch(self.showing, t!("settings-ring-notify-show"), |now| on::<Ring>(move |me, k| {
                if k.settle("notifications", notify::set_showing(now), not_saved()).is_some() {
                    me.showing = now;
                }
            })))
            .node(kit);
        let chosen = notify::how_long_choices().iter().position(|(v, _)| *v == self.long);
        let pick = dropdown("notification_seconds", notify::how_long_choices().iter().map(|(_, w)| w.to_string()).collect(),
                            chosen, t!("settings-ring-notify-long"), |i| {
            let value = notify::how_long_choices()[i].0;
            on::<Ring>(move |me, k| {
                if k.settle("notification_seconds", notify::set_how_long(value), not_saved()).is_some() {
                    me.long = value.to_string();
                }
            })
        });
        let long = line(t!("settings-ring-notify-long"))
            .key("notification_seconds")
            .control(pick)
            .node(kit);
        let corner = notify::corner_choices().iter().position(|(v, _)| *v == self.corner);
        let where_ = dropdown("notification_corner", notify::corner_choices().iter().map(|(_, w)| w.to_string()).collect(),
                              corner, t!("settings-ring-notify-where"), |i| {
            let value = notify::corner_choices()[i].0;
            on::<Ring>(move |me, k| {
                if k.settle("notification_corner", notify::set_corner(value), not_saved()).is_some() {
                    me.corner = value.to_string();
                }
            })
        });
        let place = line(t!("settings-ring-notify-where"))
            .key("notification_corner")
            .control(where_)
            .node(kit);
        group(t!("settings-ring-notify-title")).icon("preferences-system-notifications-symbolic")
            .about([t!("settings-ring-notify-about"), t!("settings-ring-notify-long-about"), t!("settings-ring-notify-where-about")].join(" "))
            .rows(vec![show, long, place])
            .node(kit)
    }

    fn clipboard(&self, kit: &Kit) -> Node<Msg> {
        let chosen = clipboard::forget_choices().iter().position(|(v, _)| *v == self.forget);
        let pick = dropdown("clipboard_forget", clipboard::forget_choices().iter().map(|(_, w)| w.to_string()).collect(),
                            chosen, t!("settings-ring-clip-forget"), |i| {
            let value = clipboard::forget_choices()[i].0;
            on::<Ring>(move |me, k| {
                if k.settle("clipboard_forget", set_setting("clipboard_forget", value), not_saved()).is_some() {
                    me.forget = value.to_string();
                }
            })
        });
        let forget = line(t!("settings-ring-clip-forget"))
            .key("clipboard_forget")
            .control(pick)
            .node(kit);
        let (n, pinned) = self.copied;
        let kept = line(t!("settings-ring-clip-now")).sub(t!("settings-ring-clip-count", n = n, pinned = pinned)).node(kit);
        group(t!("settings-ring-clip-title")).icon("edit-paste-symbolic")
            .about(t!("settings-ring-clip-about"))
            .rows(vec![forget, kept])
            .node(kit)
    }

    fn voice(&self, kit: &Kit) -> Node<Msg> {
        if let Some(d) = kit.dials.get(VOICE) {
            let adding = self.working == Some(INSTALL);
            let sub = if adding { t!("settings-ring-voice-adding") } else if self.working == Some(TOOLS) { t!("settings-ring-voice-removing-tools") } else { t!("settings-ring-voice-removing") };
            let ring = row(vec![d.node(), note(d.words()).tone(d.tone())]).gap(8.0).width(Length::Shrink);
            return self.voice_card(kit, vec![line(t!("settings-ring-voice-control")).sub(sub).key(VOICE).control(ring).node(kit)], "");
        }
        let rows = if self.voice_installed {
            let toggle = switch(self.voice_on, t!("settings-ring-voice-control"), |now| on::<Ring>(move |me, k| {
                if save_and_restart(k, "voice-switch", "voice", if now { "on" } else { "off" }, if now { t!("settings-ring-voice-onoff-on") } else { t!("settings-ring-voice-onoff-off") }) {
                    me.voice_on = now;
                }
            }));
            let control = line(t!("settings-ring-voice-control"))
                .key("voice-switch")
                .control(toggle)
                .node(kit);
            let remove = line(t!("settings-ring-voice-remove"))
                .key(VOICE)
                .control(confirm::<Ring>(VOICE, "user-trash-symbolic", t!("settings-ring-voice-remove"), |me, k| me.voice_change(k, REMOVE)))
                .node(kit);
            let mut rows = vec![control, remove];
            if self.tools_left {
                rows.push(line(t!("settings-ring-voice-tools-left"))
                    .key(TOOLS)
                    .control(confirm::<Ring>(TOOLS, "user-trash-symbolic", t!("settings-ring-voice-tools-remove"), |me, k| me.voice_change(k, TOOLS)))
                    .node(kit));
            }
            rows
        } else {
            let add = button(INSTALL_ICON, t!("settings-ring-voice-add"))
                .on_press(on::<Ring>(|me, k| me.voice_change(k, INSTALL)));
            vec![line(t!("settings-ring-voice-control"))
                .key(VOICE)
                .control(add)
                .node(kit)]
        };
        let cost = if self.voice_installed { t!("settings-ring-voice-remove-about") } else { t!("settings-ring-voice-optional") };
        self.voice_card(kit, rows, cost)
    }

    fn voice_card(&self, kit: &Kit, rows: Vec<Node<Msg>>, cost: &str) -> Node<Msg> {
        group(t!("settings-ring-voice-title")).icon("audio-input-microphone-symbolic")
            .about(format!("{} {cost}", t!("settings-ring-voice-card-about")).trim_end().to_string())
            .rows(rows)
            .node(kit)
    }

    fn voice_change(&mut self, kit: &mut Kit, verb: &'static str) {
        if kit.working(VOICE) {
            return;
        }
        self.working = Some(verb);
        kit.hush(VOICE);
        let dial = kit.start(VOICE, if verb == INSTALL { t!("settings-ring-voice-working-add") } else if verb == TOOLS { t!("settings-ring-voice-working-tools") } else { t!("settings-ring-voice-working-remove") });
        if verb == INSTALL {
            dial.by_steps();
        } else {
            dial.expect(3.0);
        }
        let heard = kit.heard_if::<Ring>(|_, k, l| k.dial(VOICE).is_some_and(|d| d.hear(l)));
        let done = kit.then::<Ring, Result<(), String>>(move |me, k, res| {
            me.working = None;
            match res {

                Ok(()) => {
                    k.finish(VOICE, true, "");
                    spawn(&["trindle-ring", "--corners"]);
                }
                Err(err) => {
                    k.finish(VOICE, false, "");
                    k.say(VOICE, &voice_failed(verb, &err), false, true);
                }
            }
            me.read(k);
        });
        admin_watching(&[verb], heard, done);
    }
}

fn voice_failed(verb: &str, err: &str) -> String {
    let could_not = match verb {
        INSTALL => t!("settings-ring-voice-not-added"),
        TOOLS => t!("settings-ring-voice-tools-not-removed"),
        _ => t!("settings-ring-voice-not-removed"),
    };
    why(err, could_not)
}

fn ring_picture() -> Node<Msg> {
    let b = crate::toolkit::theme::bento();
    card(pictures::ring().height(Length::Fill)).height(Length::Fill).beside().span_grows(b.picture.0, b.picture.1, b.words)
}

#[cfg(test)]
impl Ring {

    pub(crate) fn made_up() -> Ring {
        Ring { window_names: true, showing: true, long: notify::how_long_choices()[1].0.to_string(), corner: notify::corner_choices()[0].0.to_string(), forget: clipboard::forget_choices()[1].0.to_string(),
               voice_installed: true, voice_on: true, tools_left: false, working: None, favourites: 6, copied: (12, 2) }
    }
}

fn save_and_restart(kit: &mut Kit, key: &str, name: &str, value: &str, success: &str) -> bool {
    if kit.settle(key, set_setting(name, value), not_saved()).is_none() {
        return false;
    }
    spawn(&["trindle-ring", "--corners"]);
    kit.say(key, success, true, false);
    true
}

impl Page for Ring {
    fn id(&self) -> &'static str {
        "ring"
    }
    fn title(&self) -> &'static str {
        t!("settings-ring-title")
    }
    fn icon(&self) -> &'static str {
        "view-app-grid-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-ring-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-ring-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.read(kit);
    }
    fn conf_changed(&mut self, kit: &mut Kit) {
        self.read(kit);
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        page(kit, self.title(), self.help(), vec![
            headline(self.sum_up()),
            self.opening(kit),
            self.notifications(kit),
            self.voice(kit),

            self.clipboard(kit),
            ring_picture(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_voice_change_that_failed_is_said_in_the_pages_own_words() {
        assert_eq!(voice_failed(INSTALL, "trindle-voice: whisper.cpp build exited 2"), t!("settings-ring-voice-not-added"));
        assert_eq!(voice_failed(TOOLS, "trindle-admin: apt-get busy"), t!("settings-ring-voice-tools-not-removed"));
        assert_eq!(voice_failed("voice-remove", "x"), t!("settings-ring-voice-not-removed"));
    }
}
