//! Appearance.

use std::path::PathBuf;

use super::{Appearance, THEME, saved_accent};
use crate::common::files::setting;
use crate::common::{run, style, themes};
use crate::common::sheet::asked_light;
use crate::settings::ui::{confirm, group, help_link, line, on, Kit, Msg, Trial, not_saved, no_chooser};
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::widget::{self, button, column, drawing, label, note, row, Length, Node, Rect};
use tiny_skia::PixmapMut;

impl Appearance {
    fn pick_theme(&mut self, kit: &mut Kit, id: &str, look: Option<bool>) {
        let look = look.filter(|_| self.themes.iter().any(|t| t.id == id && t.looks().len() == 2));
        let saved = setting(crate::common::sheet::LOOK_KEY) == "light";
        let standing_in = id.is_empty() && self.unusable.is_some();
        if self.theme == id && !standing_in {
            let Some(light) = look.filter(|l| *l != self.light) else { return };
            if self.trial.left().is_none() {
                self.set_look(kit, light);
                return;
            }
        }
        if self.trial.left().is_some() && self.kept == id && look.is_none_or(|l| l == saved) {
            self.go_back(kit);
            return;
        }
        let (id, tried) = (id.to_string(), id.to_string());
        self.off(kit, move || themes::try_out(&tried, look).inspect(|_| crate::toolkit::theme::reread()), move |me, kit, tried| {
            if kit.settle_quietly(THEME, tried, not_saved()).is_none() {
                return;
            }
            me.theme = id;
            me.light = look.unwrap_or(saved);
            me.trials += 1;
            Trial::start(kit, me, TRIAL_FOR, |me: &mut Appearance| &mut me.trial, Appearance::go_back);
            kit.aloud(&t!("settings-appearance-aloud-trying", theme = me.theme_name(), seconds = TRIAL_FOR));
        });
    }

    fn theme_name(&self) -> String {
        self.themes.iter().find(|t| t.id == self.theme).map_or_else(|| t!("settings-appearance-trindle").to_string(), |t| t.name.clone())
    }

    pub(super) fn keep(&mut self, kit: &mut Kit) {
        if self.trial.left().is_none() {
            return;
        }
        self.trial.stop();
        let kept = || {
            let kept = themes::keep();
            if kept.is_err() {
                crate::toolkit::theme::reread();
            }
            kept
        };
        self.off(kit, kept, |me, kit, kept| {
            if kit.settle(THEME, kept, not_saved()).is_none() {
                return;
            }
            kit.aloud(&t!("settings-appearance-aloud-kept", theme = me.theme_name()));
            std::thread::spawn(style::apply_system_theme);
        });
    }

    pub(super) fn go_back(&mut self, kit: &mut Kit) {
        if self.trial.left().is_none() {
            return;
        }
        self.trial.stop();
        let back = || {
            let back = themes::go_back();
            crate::toolkit::theme::reread();
            back
        };
        self.off(kit, back, |me, kit, back| {
            kit.settle_quietly(THEME, back, t!("settings-appearance-back-failed"));
            me.light = asked_light();
            kit.aloud(t!("settings-appearance-aloud-back"));
        });
    }

    pub(super) fn off<T: Send + 'static>(&mut self, kit: &Kit, work: impl FnOnce() -> T + Send + 'static,
                              then: impl FnOnce(&mut Appearance, &mut Kit, T) + 'static) {
        #[cfg(test)]
        let read = self.reader.unwrap_or(themes::seen);
        #[cfg(not(test))]
        let read = themes::seen;
        let done = kit.then::<Appearance, (T, themes::Seen)>(move |me, kit, (got, seen)| {
            me.took(seen);
            then(me, kit, got);
        });
        run::on_thread(move || {
            let got = work();
            (got, read())
        }, done);
    }

    fn take_theme(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for(ADD_THEME) else { return };
        let ask = Ask { title: t!("settings-appearance-chooser-title").into(), filters: vec![(t!("settings-appearance-chooser-filter").into(), vec!["*.css".into()])], ..Ask::default() };
        let back = kit.then::<Appearance, Option<PathBuf>>(|me, k, path| {
            me.asking_theme = None;
            let Some(path) = path else { return k.unanswered(ADD_THEME) };
            me.off(k, move || themes::add(&path), |me, k, res| match res {
                Ok(id) => {
                    let name = me.themes.iter().find(|t| t.id == id).map_or(id, |t| t.name.clone());
                    k.say(ADD_THEME, &t!("settings-appearance-added", name = &name), true, false);
                }
                Err(e) => k.say(ADD_THEME, &e, false, false),
            });
        });
        self.asking_theme = chooser::open(&bus, &kit.events, ask, back);
        kit.started(ADD_THEME, self.asking_theme.is_some(), no_chooser());
    }

    fn remove_theme(&mut self, kit: &mut Kit, id: &str) {
        let (gone, in_use) = (id.to_string(), self.theme == id);
        let remove = move || {
            let removed = themes::remove(&gone);
            if removed.is_ok() && in_use {
                crate::toolkit::theme::reread();
            }
            removed
        };
        self.off(kit, remove, move |me, kit, removed| {
            if kit.settle(THEME, removed, t!("settings-appearance-not-removed")).is_none() {
                return;
            }
            if in_use {
                me.trial.stop();
                me.light = asked_light();
                std::thread::spawn(style::apply_system_theme);
            }
        });
    }

    pub(super) fn read_themes(&mut self, kit: &Kit) {
        self.listing = true;
        self.off(kit, || (), |_, _, ()| {});
    }

    pub(super) fn took(&mut self, seen: themes::Seen) {
        self.listing = false;
        self.themes = seen.listed.themes;
        self.said = seen.listed.said;
        self.said.extend(seen.own);
        if seen.hard {
            self.said.push(themes::OWN_HARD_TO_READ);
        }
        self.unusable = seen.unusable;
        self.theme = seen.in_use;
        self.kept = seen.kept;
    }

    pub(super) fn troubles(&self) -> Vec<String> {
        let mut all: Vec<String> = Vec::new();
        if let Some((name, why)) = &self.unusable {
            if !self.themes.iter().any(|t| t.refused.is_some() && t.name == *name) {
                all.push(t!("settings-appearance-cannot-use", theme = name, why = *why));
            }
        }
        all.extend(self.said.iter().map(|s| s.to_string()));
        all
    }

    fn tried_first(&self, kit: &Kit) -> Node<Msg> {
        line(t!("settings-appearance-tried-first"))
            .sub(t!("settings-appearance-tried-first-about", seconds = TRIAL_FOR))
            .node(kit)
    }

    pub(super) fn theme_card(&self, kit: &Kit) -> Node<Msg> {
        let accent = if self.accent.is_empty() { saved_accent() } else { self.accent.clone() };
        let cards = self.themes.iter().map(|t| self.theme_tile(kit, t, &accent)).collect::<Vec<_>>();
        let add = button("list-add-symbolic", t!("settings-appearance-add-theme")).on_press(on::<Appearance>(|me, k| me.take_theme(k)));
        let mut tiles = if self.listing && self.themes.is_empty() {
            vec![row(vec![widget::spinner().key(THEMES_WAIT).spoken(t!("settings-appearance-looking"))]).width(Length::Fill)]
        } else {
            vec![widget::even_grid(THEME_TILE, cards)]
        };
        tiles.extend(self.troubles().into_iter().enumerate().map(|(n, words)| {
            note(words).tone(widget::Tone::Problem).key(trouble_key(n)).width(Length::Fill)
        }));
        group(t!("settings-appearance-theme-card")).icon("applications-graphics-symbolic")
            .about(t!("settings-appearance-theme-about"))
            .rows(vec![
                self.tried_first(kit),
                line(t!("settings-appearance-theme-line")).sub(t!("settings-appearance-theme-line-about")).key(THEME).below(column(tiles).gap(8.0).width(Length::Fill)).node(kit),
                line(t!("settings-appearance-add-theme"))
                    .sub(t!("settings-appearance-add-theme-about"))
                    .key(ADD_THEME)
                    .controls(vec![add, help_link(t!("settings-appearance-theme-file-help"), "theme-files")])
                    .node(kit),
            ])
            .span(12, 12)
            .node(kit)
    }

    fn theme_tile(&self, kit: &Kit, t: &themes::Theme, accent: &str) -> Node<Msg> {
        if let Some(why) = t.refused {
            let said = note(why).tone(widget::Tone::Problem).key(refused_key(&t.id)).width(Length::Fill);
            return widget::card(column(vec![row(name_head(kit, t)).gap(8.0).width(Length::Fill), said]).gap(8.0));
        }
        let chosen = self.theme == t.id;
        let id = t.id.clone();
        let pick = move || {
            let id = id.clone();
            on::<Appearance>(move |me, k| me.pick_theme(k, &id, None))
        };
        let light = t.light(self.light);
        let s = crate::toolkit::theme::sample(&t.css, light, accent);
        let corrected = s.theme.corrected;
        let mut head = name_head(kit, t);
        head.push(crate::toolkit::widget::word_button(t!("settings-appearance-use-theme", name = &t.name)).on(chosen).on_press(pick()));
        let picture = theme_sample(&t.name, look_name_lower(light), s).key(picture_key(&t.id));
        let picture = if t.looks().len() == 2 { with_look_buttons(t, picture, chosen, light) } else { picture };
        let mut lines = vec![row(head).gap(8.0).width(Length::Fill), picture];
        if !t.about.is_empty() {
            lines.push(note(t.about.clone()));
        }
        if !t.author.is_empty() && t.mine {
            lines.push(note(t!("settings-appearance-by", author = &t.author)));
        }
        if corrected {
            lines.push(note(made_clearer()));
        }
        widget::card(column(lines).gap(8.0)).on_press(pick())
    }

}

fn name_head(kit: &Kit, t: &themes::Theme) -> Vec<Node<Msg>> {
    let mut head = vec![label(t.name.clone()).width(Length::Fill)];
    if t.mine {
        let id = t.id.clone();
        head.push(confirm::<Appearance>(kit, &format!("theme:{}", t.id), "user-trash-symbolic", t!("settings-appearance-remove-theme"),
                                        move |me, k| me.remove_theme(k, &id)));
    }
    head
}

fn with_look_buttons(t: &themes::Theme, picture: Node<Msg>, chosen: bool, light: bool) -> Node<Msg> {
    let tall = crate::toolkit::theme::sheet().word_button.1;
    let looks = t.looks().iter().map(|&look| {
        let id = t.id.clone();
        widget::word_button(look_name(look))
            .on(chosen && light == look)
            .spoken(t!("settings-appearance-theme-spoken", name = &t.name, look = look_name_lower(look)))
            .on_press(on::<Appearance>(move |me, k| me.pick_theme(k, &id, Some(look))))
            .key(look_key(&t.id, look))
            .width(Length::Fill)
    }).collect();
    let apart = (SAMPLE.1 - 2.0 * tall).clamp(4.0, 16.0);
    row(vec![picture, column(looks).gap(apart).width(Length::Shrink)]).gap(LOOKS_GAP).width(Length::Fill)
}

const ADD_THEME: &str = "appearance:add-theme";
pub(super) const THEMES_WAIT: &str = "appearance:themes-waiting";

pub(super) const SAMPLE: (f32, f32) = (184.0, 72.0);
const THEME_TILE: f32 = 216.0;
pub(super) const LOOKS_GAP: f32 = 8.0;

pub(super) fn look_name(light: bool) -> &'static str {
    if light { t!("settings-appearance-look-light") } else { t!("settings-appearance-look-dark") }
}

pub(super) fn look_name_lower(light: bool) -> &'static str {
    if light { t!("settings-appearance-look-light-lower") } else { t!("settings-appearance-look-dark-lower") }
}

fn look_id(light: bool) -> &'static str {
    if light { "light" } else { "dark" }
}

pub(super) fn picture_key(id: &str) -> String {
    format!("theme-picture:{}", if id.is_empty() { "default" } else { id })
}

pub(super) fn refused_key(id: &str) -> String {
    format!("theme-refused:{id}")
}

fn trouble_key(n: usize) -> String {
    format!("theme-trouble:{n}")
}

pub(super) fn look_key(id: &str, light: bool) -> String {
    format!("theme-look:{}:{}", if id.is_empty() { "default" } else { id }, look_id(light))
}

const TRIAL_FOR: i32 = crate::common::sheet::TRIAL_FOR as i32;

fn made_clearer() -> &'static str {
    t!("settings-appearance-corrected")
}

fn theme_sample(name: &str, look: &str, s: crate::toolkit::theme::Sample) -> Node<Msg> {
    use crate::toolkit::ui::fill;
    drawing(SAMPLE, t!("settings-appearance-sample-spoken", name = name, look = look), move |canvas, pen| {
        let (k, t) = (pen.scale, &s.theme);
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let f = h / 76.0;
        let wide = w / f;
        let at = |x: f32, y: f32, w: f32, h: f32| Rect { x: (x * f).round(), y: (y * f).round(), w: (w * f).round(), h: (h * f).round() };
        let narrow = wide < 150.0;
        let side = if narrow { 26.0 } else { 60.0 };
        let (sx, card) = (wide - 8.0 - side, (wide - 24.0 - side).max(24.0));
        let inner = card - 16.0;
        let edged = |canvas: &mut PixmapMut, r: Rect, radius: f32, c: Colour, edge: Colour| {
            if edge.a > 0.0 {
                fill(canvas, r.inset(-k.round().max(1.0)), radius + k, edge);
            }
            fill(canvas, r, radius, c);
        };
        let corner = (s.card_corner * 0.6).min(10.0) * k;
        let small = (s.box_corner * 0.6).min(7.0) * k;
        fill(canvas, Rect { x: 0.0, y: 0.0, w, h }, small.max(4.0 * k), t.bg);
        edged(canvas, at(8.0, 8.0, card, 60.0), corner, t.panel, t.edge);
        fill(canvas, at(16.0, 17.0, inner.min(46.0), 5.0), 2.5 * k, t.fg);
        fill(canvas, at(16.0, 28.0, inner.min(70.0), 4.0), 2.0 * k, t.muted);
        edged(canvas, at(16.0, 40.0, inner, 20.0), small, t.raised, t.card_edge);
        fill(canvas, at(22.0, 48.0, (inner - 12.0).min(34.0), 4.0), 2.0 * k, t.muted);
        fill(canvas, at(sx, 8.0, side, 16.0), 8.0 * k, t.accent);
        fill(canvas, at(sx + side * 0.23, 14.0, side * 0.54, 4.0), 2.0 * k, t.on_accent);
        edged(canvas, at(sx + 1.0, 33.0, side - 2.0, 14.0), small.min(7.0 * k), t.field, t.field_edge);
        if narrow {
            fill(canvas, at(sx + (side - 14.0) / 2.0, 54.0, 14.0, 14.0), 7.0 * k, t.control);
        } else {
            fill(canvas, at(sx, 54.0, 14.0, 14.0), 7.0 * k, t.control);
            fill(canvas, at(sx + 20.0, 54.0, 14.0, 14.0), 7.0 * k, t.control);
            fill(canvas, at(sx + 40.0, 57.0, 20.0, 8.0), 4.0 * k, t.muted);
        }
    })
    .width(Length::Fill)
}
