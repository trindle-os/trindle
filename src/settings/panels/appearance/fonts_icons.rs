//! Appearance: the icon set and the fonts, with a sample of each and the way to add fonts of the person's own.

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::{Appearance, icon_sets, write_ini};
use crate::common::files::{data_home, set_setting};
use crate::common::{packs, run};
use crate::settings::ui::{dropdown, group, help_link, icon, line, on, ran, replace_rows, Kit, Msg, Page, Raw, not_saved, no_chooser};
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::t;
use crate::toolkit::widget::{self, button, column, label, note, row, Length, Node};

impl Appearance {
    pub(super) fn load_families(&self, kit: &Kit) {
        let back = kit.then::<Appearance, Vec<String>>(|me, _, found| me.families = found);
        run::capture(&["fc-list", ":", "family"], None, move |_, out, _| back(families(&out)));
    }

    pub(super) fn ini(&self, kit: &Kit, key: &'static str, name: &'static str, value: String) {
        let back = kit.settles::<Appearance, (), std::io::Error>(key, not_saved(), |me, k, done| {

            if done.is_none() {
                me.shown(k);
            }
        });
        run::on_thread(move || write_ini(name, &value), back);
    }

    fn icon_set(&mut self, kit: &mut Kit, set: &'static str) {
        if self.set == set || kit.working(ICONS) {
            return;
        }
        if kit.settle_quietly(ICONS, set_setting("icon_source", set), not_saved()).is_none() {
            return;
        }
        self.set = set;
        kit.hush(ICONS);
        kit.start(ICONS, t!("settings-appearance-icons-working")).expect(5.0);
        let back = kit.then::<Appearance, Result<(), Raw>>(|_, k, made| {
            k.finish(ICONS, made.is_ok(), if made.is_ok() { t!("settings-appearance-icons-changed") } else { "" });
            k.settle(ICONS, made, no_icons());

            run::spawn(&["trindle-ring", "--restyle"]);
        });
        run::capture(&["trindle-icons", "--quiet"], None, move |ok, _, err| back(ran(ok, &err)));
    }

    fn set_font(&mut self, kit: &mut Kit, family: Option<String>, size: Option<String>) {
        let (f, s) = split_font(&self.font);
        self.font = format!("{} {}", family.unwrap_or(f), size.unwrap_or(s));
        self.ini(kit, FONT, "gtk-font-name", self.font.clone());
    }

    pub(super) fn skip_font(&mut self) {
        for taken in self.taken_fonts.answer() {
            packs::forget_waiting(&taken);
        }
    }

    fn replace_font(&mut self, kit: &mut Kit) {
        let covered = self.taken_fonts.answer();
        if covered.is_empty() {
            return;
        }
        let back = kit.then::<Appearance, Vec<(String, std::io::Result<()>)>>(|me, k, done| {
            let total = done.len();
            let mut mended = Vec::new();
            for (name, result) in done {
                if k.settle_quietly(REPLACE_FONT, result, &t!("settings-appearance-font-not-replaced", name = &name)).is_some() {
                    mended.push(name);
                }
            }
            if let [one] = mended.as_slice() {
                k.say(REPLACE_FONT, &t!("settings-appearance-font-replaced-one", name = one), true, false);
            } else if !mended.is_empty() {
                k.say(REPLACE_FONT, &t!("settings-appearance-font-replaced-some", n = mended.len(), total = total), true, false);
            }
            me.load_families(k);
        });
        run::on_thread(move || covered.iter().map(|t| (t.name.clone(), packs::replace_font(t))).collect(), back);
    }

    fn take_fonts(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for(ADD_FONTS) else { return };
        let ask = Ask { title: t!("settings-appearance-chooser-fonts").into(), ..Ask::default() };
        let back = kit.then::<Appearance, Option<PathBuf>>(|me, k, path| {
            me.asking = None;
            let Some(path) = path else { return k.unanswered(ADD_FONTS) };
            let done = k.then::<Appearance, Result<packs::Added, String>>(|me, k, res| match res {
                Ok(added) => {
                    let n = added.made;
                    if n > 0 {
                        k.say(ADD_FONTS, &t!("settings-appearance-fonts-added", n = n), true, false);
                        me.load_families(k);
                    } else if added.taken.is_empty() {
                        k.say(ADD_FONTS, t!("settings-appearance-fonts-in-list"), true, false);
                    }
                    me.taken_fonts.add_batch(added.taken);
                }
                Err(e) => k.say(ADD_FONTS, &e, false, false),
            });
            run::on_thread(move || packs::add_fonts(&path), done);
        });
        self.asking = chooser::open(&bus, &kit.events, ask, back);
        kit.started(ADD_FONTS, self.asking.is_some(), no_chooser());
    }

    pub(super) fn icons_card(&self, kit: &Kit) -> Node<Msg> {
        let cards = icon_sets().into_iter().enumerate().map(|(i, (set, name))| {
            let set: &'static str = set;
            let chosen = self.set == set;
            let pick = || on::<Appearance>(move |me, k| me.icon_set(k, set));

            let pictures = match self.samples.get(i) {
                Some(s) => s.iter().map(|p| icon(p.clone(), SAMPLE_SIZE)).collect(),
                None => SAMPLES.iter().map(|p| icon(*p, SAMPLE_SIZE)).collect(),
            };
            let head = row(vec![label(name).width(Length::Fill),
                                crate::toolkit::widget::word_button(t!("settings-appearance-icons-set", name = name)).on(chosen).on_press(pick())])
                .gap(8.0).width(Length::Fill);

            widget::card(column(vec![head, row(pictures).gap(8.0)]).gap(8.0)).on_press(pick())
                .off_because(crate::common::ui::wait_while(kit.working(ICONS)))
        }).collect::<Vec<_>>();
        let mut controls = Vec::new();
        if let Some(d) = kit.dials.get(ICONS) {
            controls.push(row(vec![d.node(), note(d.words()).tone(d.tone())]).gap(8.0).width(Length::Shrink));
        }
        let shape = line(t!("settings-appearance-shape"))
            .sub(t!("settings-appearance-shape-about"))
            .key(ICONS)
            .controls(controls)
            .below(column(cards).gap(8.0))
            .node(kit);
        group(t!("settings-appearance-icons-title")).icon("view-app-grid-symbolic")
            .about(t!("settings-appearance-icons-about"))
            .rows(vec![shape])
            .node(kit)
    }

    fn font_line(&self, kit: &Kit) -> Node<Msg> {
        let (family, size) = split_font(&self.font);
        let mut fams = self.families.clone();
        if !fams.contains(&family) {
            fams.push(family.clone());
            fams.sort_by_key(|s| s.to_lowercase());
        }
        let at = fams.iter().position(|f| *f == family);
        let mut sizes: Vec<String> = FONT_SIZES.iter().map(|s| s.to_string()).collect();
        if !sizes.contains(&size) {
            sizes.push(size.clone());
            sizes.sort_by(|a, b| a.parse::<f32>().unwrap_or(0.0).total_cmp(&b.parse::<f32>().unwrap_or(0.0)));
        }
        let size_at = sizes.iter().position(|s| *s == size);
        let names = fams.clone();

        let shown: Vec<String> = fams.iter().map(|f| if f.is_empty() { t!("settings-appearance-font-default").to_string() } else { f.clone() }).collect();
        let pick_family = dropdown(kit, FONT, shown, at, t!("settings-appearance-font"), move |i| {
            let f = names[i].clone();
            on::<Appearance>(move |me, k| me.set_font(k, Some(f.clone()), None))
        });
        let all = sizes.clone();
        let pick_size = dropdown(kit, FONT_SIZE, sizes, size_at, t!("settings-appearance-font-size"), move |i| {
            let s = all[i].clone();
            on::<Appearance>(move |me, k| me.set_font(k, None, Some(s.clone())))
        });
        line(t!("settings-appearance-font")).sub(t!("settings-appearance-font-about")).key(FONT).controls(vec![pick_family, pick_size]).node(kit)
    }

    pub(super) fn text_card(&self, kit: &Kit) -> Node<Msg> {
        let add = button("list-add-symbolic", t!("settings-appearance-add-fonts")).on_press(on::<Appearance>(|me, k| me.take_fonts(k)));
        let mut rows = vec![self.font_line(kit)];
        rows.extend(replace_rows(kit, REPLACE_FONT, &self.taken_fonts, Vec::new(), Vec::new(),
                                 on::<Appearance>(|me, _| me.skip_font()), on::<Appearance>(|me, k| me.replace_font(k)),
                                 |all| on::<Appearance>(move |me, _| me.taken_fonts.set_same(all))));
        rows.push(line(t!("settings-appearance-add-fonts"))
            .sub(t!("settings-appearance-add-fonts-about"))
            .key(ADD_FONTS)
            .controls(vec![add, help_link(t!("settings-appearance-fonts-help"), "themes")])
            .node(kit));
        group(t!("settings-appearance-text-title")).icon("font-x-generic-symbolic")
            .about(t!("settings-appearance-text-about"))
            .rows(rows)
            .node(kit)
    }

}

pub(super) const SAMPLES: &[&str] = &["firefox", "thunderbird", "org.gnome.Calculator", "vlc", "gimp"];
const SAMPLE_SIZE: f32 = 28.0;

const FONT_SIZES: &[&str] = &["8", "9", "10", "11", "12", "13", "14", "16", "18", "20", "24"];

const ICONS: &str = "appearance:icons";
const FONT: &str = "appearance:font";
const FONT_SIZE: &str = "appearance:font-size";
const ADD_FONTS: &str = "appearance:add-fonts";

const REPLACE_FONT: &str = "appearance:replace-font";

fn no_icons() -> &'static str {
    t!("settings-appearance-no-icons")
}

pub(super) fn sample(set: &str, name: &str) -> String {
    let system = std::env::var("XDG_DATA_DIRS").ok().filter(|d| !d.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".to_string());
    std::iter::once(data_home()).chain(system.split(':').map(PathBuf::from))
        .map(|d| d.join("icons").join(set).join("128x128/apps").join(format!("{name}.svg")))
        .find(|f| crate::common::files::own_place(f) && f.exists())
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| name.to_string())
}

pub(super) fn families(out: &str) -> Vec<String> {
    let found: BTreeSet<String> = out.lines()
        .filter_map(|l| l.split(',').next())
        .map(|f| f.replace("\\-", "-").trim().to_string())
        .filter(|f| !f.is_empty())
        .collect();
    let mut all: Vec<String> = found.into_iter().collect();
    all.sort_by_key(|s| s.to_lowercase());
    all
}

pub(super) fn split_font(name: &str) -> (String, String) {
    let name = name.trim();
    match name.rsplit_once(' ') {
        Some((family, size)) if size.parse::<f32>().is_ok() => (family.trim().to_string(), size.to_string()),
        _ => (name.to_string(), "11".to_string()),
    }
}
