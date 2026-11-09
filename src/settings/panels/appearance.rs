//! Appearance: the theme and its dark or light look, the highlight, icons,
//! cursor, font and the screen reader.
//! GTK reads settings.ini at startup, so changes reach apps opened afterwards.

pub(crate) mod colours;
mod fonts_icons;
mod looks;
mod pointers;
mod reading;

use crate::common::files::{read_env, setting, write_text};
use crate::common::sheet::asked_light;
use crate::common::keyfile::KeyFile;
use crate::common::{dconf, look, screen_reader, style, themes, tip};
use crate::icons;
use crate::settings::session_env;
use crate::settings::ui::{on, page, Clashes, FootLook, Kit, Msg, Page, Trial};
use crate::toolkit::bus::Listen;
use crate::toolkit::widget::Node;
use crate::toolkit::t;
use colours::highlights;
use fonts_icons::{sample, SAMPLES};
use pointers::{cursor_themes, pointer_art, PointerArt};

fn icon_sets() -> [(&'static str, &'static str); 1] {
    [(icons::CIRCLES, t!("settings-appearance-icons-circles"))]
}

fn says() -> &'static str {
    t!("settings-appearance-reading-says")
}

const THEME: &str = "appearance:theme";

fn read_ini(key: &str, default: &str) -> String {
    KeyFile::load_or_empty(&look::gtk_files()[0]).string("Settings", key).unwrap_or_else(|| default.to_string())
}

fn write_ini(key: &str, value: &str) -> std::io::Result<()> {
    for path in look::gtk_files() {
        set_ini_in(&path, key, value)?;
    }
    dconf::copy_look();
    Ok(())
}

pub(crate) fn set_ini_in(path: &std::path::Path, key: &str, value: &str) -> std::io::Result<()> {
    let mut kf = KeyFile::load_kept(path)?;
    kf.set("Settings", key, value);
    write_text(path, &kf.text())
}

fn saved_accent() -> String {
    let saved = setting("accent");
    if saved.is_empty() { style::DEFAULT_ACCENT.to_string() } else { saved.to_lowercase() }
}

#[derive(Default)]
pub struct Appearance {

    themes: Vec<themes::Theme>,
    theme: String,

    kept: String,

    listing: bool,

    #[cfg(test)]
    reader: Option<fn() -> themes::Seen>,

    said: Vec<&'static str>,

    unusable: Option<(String, &'static str)>,

    trial: Trial,

    trials: u32,
    asking_theme: Option<Listen>,

    light: bool,
    accent: String,

    hex: String,
    reading: bool,

    labels_stay: bool,
    installed: bool,
    set: &'static str,

    samples: Vec<Vec<String>>,
    font: String,
    families: Vec<String>,
    cursors: Vec<String>,
    cursor: String,

    pointer_art: PointerArt,
    cursor_size: String,
    asking: Option<Listen>,

    taken_fonts: Clashes,
}

impl Page for Appearance {
    fn id(&self) -> &'static str {
        "appearance"
    }

    fn foot(&self, _kit: &Kit) -> Option<Node<Msg>> {
        self.trial.foot(FootLook { ask: t!("settings-appearance-foot-keep"), more: t!("settings-appearance-foot-escape"), safe: true, escape: true },
                        self.trials, Appearance::go_back, Appearance::keep)
    }
    fn title(&self) -> &'static str {
        t!("settings-appearance-title")
    }
    fn icon(&self) -> &'static str {
        "applications-graphics-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-appearance-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-appearance-help")
    }

    fn shown(&mut self, kit: &mut Kit) {
        self.read_themes(kit);

        match crate::common::sheet::trial_left() {
            Some(left) if self.trial.left().is_none() => {
                Trial::start(kit, self, left.as_secs() as i32, |me: &mut Appearance| &mut me.trial, Appearance::go_back);
            }
            None => self.trial.stop(),
            _ => {}
        }

        self.light = asked_light();
        self.accent = saved_accent();
        self.hex = if highlights().iter().any(|(h, _)| *h == self.accent) { String::new() } else { self.accent.clone() };
        self.installed = screen_reader::installed();
        self.labels_stay = setting(tip::KEY) == "on";
        self.reading = screen_reader::on() && self.installed;
        self.set = icons::chosen_set();
        self.samples = icon_sets().iter().map(|(set, _)| SAMPLES.iter().map(|s| sample(set, s)).collect()).collect();
        self.font = read_ini("gtk-font-name", "Sans 11");
        self.cursors = cursor_themes();
        let env = read_env(&session_env::path());
        self.cursor = env.get("XCURSOR_THEME").cloned().unwrap_or_else(|| "Adwaita".to_string());

        self.pointer_art = pointer_art(env.get("XCURSOR_THEME").map_or("default", String::as_str));
        self.cursor_size = env.get("XCURSOR_SIZE").cloned().unwrap_or_else(|| "24".to_string());
        self.load_families(kit);
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {

        let all = page(kit, self.title(), self.help(), vec![
            self.top(kit),
            self.theme_card(kit),
            self.highlight_card(kit),
            self.pointer_card(kit),
            self.icons_card(kit),
            self.text_card(kit),
            self.reading_card(kit),
        ]);

        if self.trial.left().is_some() { all.on_escape(on::<Appearance>(Appearance::go_back)) } else { all }
    }
}

#[cfg(test)]
impl Appearance {

    pub(crate) fn on_trial() -> Appearance {
        Appearance { trial: Trial::at(14), trials: 1, ..Appearance::made_up() }
    }

    pub(crate) fn made_up() -> Appearance {
        let made = |id: &str, css: &str| themes::Theme { id: id.into(), name: String::new(), about: String::new(), author: "John Smith".into(),
                                                       version: "1.0".into(), mine: true, css: css.into(),
                                                       only: crate::common::sheet::one_look(css), refused: None };
        let mut all = themes::all();
        all.truncate(1);
        all.push(themes::Theme { name: "Harbour".into(), about: "Cool greys with rounder corners.".into(),
                                 ..made("harbour", ":root { --trindle-page: #0e1620; --trindle-card: #182432; --trindle-box: #223244; --trindle-card-corner: 16; }") });
        all.push(themes::Theme { name: "Chalk".into(), about: "A pale page for bright rooms.".into(),
                                 ..made("chalk", ":root { --trindle-theme-look: light; --trindle-page: #ecebe6; --trindle-card: #fbfaf6; --trindle-box: #f0efe9; }") });

        let trial = if std::env::var_os("TRINDLE_SHOT_TRIAL").is_some() { Trial::at(14) } else { Trial::default() };
        Appearance { themes: all, trial, accent: style::DEFAULT_ACCENT.to_string(), installed: true, set: icon_sets()[0].0,
                     font: "Trindle Sans 11".into(),
                     families: ["DejaVu Sans", "Liberation Serif", "Noto Sans", "Noto Serif", "Trindle Sans"].map(String::from).to_vec(),
                     cursors: vec!["Adwaita".into(), "Big pointers".into()], cursor: "Adwaita".into(),
                     cursor_size: "24".into(), ..Appearance::default() }
    }
}

#[cfg(test)]
mod tests;
