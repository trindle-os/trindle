//! The rows of a drive's card where a change is chosen: label, folder, startup, resize, room, check, format and delete.

use super::plan::size_text;
use crate::toolkit::t;
use crate::toolkit::widget::{Length, Node, Tone, button, choice, column, entry, label, main_word, note, slider, switch};
use super::{Act, Drives, GB, Msg, help, plan, room};
use super::model::Vol;
use super::model::Drive;
use super::model::Caps;
use super::actions::format_key;
use super::map::hidden_share;

impl Drives {
    pub(super) fn label_row(&self, d: &Drive, v: &Vol) -> Node<Msg> {
        let k = format!("{}:name", v.path);
        let rename = Msg::Act(Act::Rename { drive: d.path.clone(), vol: v.path.clone() });
        let kc = k.clone();
        let controls = vec![
            self.enter(entry(self.texts.get(&k).cloned().unwrap_or_else(|| v.label.clone()), t!("disks-name"), move |t| Msg::Text(kc.clone(), t))
                .key(k.clone()).width(Length::Fixed(180.0)), rename.clone()),
            button(super::SAVE_ICON, t!("disks-rename-tip")).on_press(rename).off_because(self.shut()),
        ];
        self.line(&d.path, &k, label(t!("disks-name")), t!("disks-name-note"), controls, None)
    }

    pub(super) fn folder_row(&self, d: &Drive, v: &Vol) -> Node<Msg> {
        let k = format!("{}:folder", v.path);
        let save = Msg::Act(Act::Folder { drive: d.path.clone(), vol: v.path.clone() });
        let own = v.fstab.as_ref().map(|f| plan::name_of(&f.dir)).unwrap_or_default();
        let kc = k.clone();
        let controls = vec![
            note(format!("{}/", plan::MNT)),
            self.enter(entry(self.texts.get(&k).cloned().unwrap_or(own), t!("disks-folder-name"), move |t| Msg::Text(kc.clone(), t))
                .key(k.clone()).width(Length::Fixed(160.0)), save.clone()),
            button(super::SAVE_ICON, t!("disks-use-folder")).on_press(save).off_because(self.shut()),
        ];
        self.line(&d.path, &k, label(t!("disks-folder")), t!("disks-folder-note"), controls, None)
    }

    pub(super) fn startup_row(&self, d: &Drive, v: &Vol) -> Node<Msg> {
        let k = format!("{}:startup", v.path);
        let on = v.fstab.as_ref().is_some_and(|f| plan::starts_at_boot(&f.opts));
        let (dp, vp) = (d.path.clone(), v.path.clone());
        let controls = vec![switch(on, t!("disks-mount-startup"), move |on| Msg::Startup { drive: dp.clone(), vol: vp.clone(), on }).key(k.clone())];
        self.line(&d.path, &k, label(t!("disks-mount-startup")),
            t!("disks-startup-note"), controls, None)
    }

    pub(super) fn resize_row(&self, d: &Drive, v: &Vol) -> Option<Node<Msg>> {
        let (min, max, used, mode) = self.resize_range(d, v)?;
        let k = format!("{}:size", v.path);
        let (least, most) = (size_text(min as f64), size_text(max as f64));
        let sub = if used.is_none() && mode & (plan::OFFLINE_SHRINK | plan::ONLINE_SHRINK) != 0 {
            t!("disks-resize-range-mount", least = least, most = most)
        } else {
            t!("disks-resize-range", least = least, most = most)
        };
        let gb = self.gbs.get(&k).copied().unwrap_or(v.size as f32 / GB as f32);
        let kc = k.clone();
        let controls = vec![
            slider(gb, min as f32 / GB as f32, max as f32 / GB as f32, 0.1, t!("disks-size-gb"), move |g| Msg::Gb(kc.clone(), g))
                .key(k.clone()).width(Length::Fixed(200.0)),
            label(plan::gb_text(gb)),
            button("view-fullscreen-symbolic", t!("disks-resize-tip")).on_press(Msg::Act(Act::Resize { drive: d.path.clone(), vol: v.path.clone() })).off_because(self.shut()),
        ];
        let (name, drive, from, to) = (v.name(), d.words(), size_text(v.size as f64), plan::gb_text(gb));
        let warning = if f64::from(gb) * GB < v.size as f64 {
            t!("disks-resize-warning-shrink", name = name, drive = drive, from = from, to = to)
        } else {
            t!("disks-resize-warning", name = name, drive = drive, from = from, to = to)
        };
        Some(self.line(&d.path, &k, label(t!("disks-size")), &sub, controls, Some((format!("resize:{}", v.path), warning))))
    }

    pub(super) fn room_row(&self, d: &Drive, v: &Vol) -> Option<Node<Msg>> {
        if d.optical || v.part.is_none() {
            return None;
        }
        let (facts, _) = self.room_facts(v, false);
        let k = format!("{}:room", v.path);
        let head = label(t!("disks-room-label"));
        match room::offer(&facts) {
            room::Offer::Nothing => None,
            room::Offer::Not(why) => Some(self.line(&d.path, &k, head, &why, vec![crate::common::more::button(Msg::More(help::ROOM))], None)),
            room::Offer::Make { least, most } => {
                let want = self.room_want(&k, &facts);
                let gb = want as f32 / GB as f32;
                let kc = k.clone();
                let controls = vec![
                    slider(gb, least as f32 / GB as f32, most as f32 / GB as f32, 0.1, t!("disks-room-gb"), move |g| Msg::Gb(kc.clone(), g))
                        .key(k.clone()).width(Length::Fixed(200.0)),
                    label(plan::gb_text(gb)),
                    button("view-fullscreen-symbolic", t!("disks-room-tip")).on_press(Msg::Act(Act::Room { drive: d.path.clone(), vol: v.path.clone() })).off_because(self.shut()),
                ];
                let warning = t!("disks-room-warning", name = v.name(), drive = d.words(), size = plan::gb_text(gb));
                Some(self.line(&d.path, &k, head, t!("disks-room-note"), controls, Some((format!("room:{}", v.path), warning))))
            }
        }
    }

    pub(super) fn check_row(&self, d: &Drive, v: &Vol, caps: Caps) -> Node<Msg> {
        let k = format!("{}:faults", v.path);
        let mut controls = Vec::new();
        if caps.check {
            controls.push(button("edit-find-symbolic", t!("disks-check-tip"))
                .on_press(Msg::Act(Act::Check { drive: d.path.clone(), vol: v.path.clone() })).off_because(self.shut()));
        }
        if caps.repair {
            controls.push(button("applications-engineering-symbolic", t!("disks-repair-tip"))
                .on_press(Msg::Act(Act::Repair { drive: d.path.clone(), vol: v.path.clone() })).off_because(self.shut()));
        }
        let warning = t!("disks-repair-warning", name = v.name(), drive = d.words());
        self.line(&d.path, &k, label(t!("disks-faults")),
            t!("disks-faults-note"),
            controls, Some((format!("repair:{}", v.path), warning)))
    }

    fn kind_note(&self, key: &str, kinds: &[(&'static str, String)]) -> String {
        let id = kinds.get(self.picks.get(key).copied().unwrap_or(0)).map(|k| k.0);
        plan::kinds().iter().find(|k| Some(k.id) == id).map(|k| t!("disks-kind-line", name = k.name, what = k.for_what).to_string()).unwrap_or_default()
    }

    pub(super) fn format_row(&self, d: &Drive, target: &str, whole: bool, kinds: &[(&'static str, String)]) -> Node<Msg> {
        let fk = format_key(&d.path, target, whole);
        let (head, sub) = if whole { (t!("disks-erase-whole"), t!("disks-format-whole")) } else { (t!("disks-format"), t!("disks-format-part")) };
        let words = match d.vols.iter().find(|v| v.path == target) {
            Some(v) if !whole && v.part.is_some() => t!("disks-part-where", name = v.name(), size = size_text(v.size as f64), drive = d.words()),
            _ => d.words(),
        };
        let (nk, kk) = (format!("{fk}:name"), format!("{fk}:kind"));
        let (nkc, kkc) = (nk.clone(), kk.clone());
        let controls = vec![
            entry(self.texts.get(&nk).cloned().unwrap_or_default(), t!("disks-name-optional"), move |t| Msg::Text(nkc.clone(), t))
                .key(nk.clone()).width(Length::Fixed(140.0)),
            choice(kinds.iter().map(|k| k.1.clone()).collect(), self.picks.get(&kk).copied().unwrap_or(0), t!("disks-kind"),
                move |i| Msg::Pick(kkc.clone(), i)).key(kk.clone()),
            main_word(t!("disks-format")).spoken(if whole { t!("disks-erase-whole") } else { t!("disks-format-tip") })
                .on_press(Msg::Act(Act::Format { drive: d.path.clone(), target: target.to_string(), whole })).off_because(self.shut()),
        ];
        let at = if whole { String::new() } else { fk.clone() };
        let warning = t!("disks-format-warning", words = &words);

        let shown = if whole { "\u{0}" } else { at.as_str() };
        let sub = format!("{sub} {}", self.kind_note(&kk, kinds));

        column(vec![
            note(t!("disks-format-erases", words = &words)).tone(Tone::Problem),
            self.line(&d.path, shown, label(head), sub.trim(), controls, Some((fk, warning))),
        ]).gap(8.0)
    }

    pub(super) fn delete_row(&self, d: &Drive, v: &Vol) -> Node<Msg> {

        let name = t!("disks-delete-name", name = v.name(), size = size_text(v.size as f64), drive = d.words());
        let controls = vec![crate::toolkit::widget::remove("user-trash-symbolic", name).key(format!("delete:{}", v.path))
            .on_press(Msg::Act(Act::Delete { drive: d.path.clone(), vol: v.path.clone() })).off_because(self.shut())];
        self.line(&d.path, "\u{0}", label(t!("disks-delete-label")),
            t!("disks-delete-note"),
            controls, None)
    }

    fn gap_able(&self, d: &Drive, kinds: &[(&'static str, String)]) -> bool {
        !d.system && !d.optical && !kinds.is_empty() && d.whole.is_some()
    }

    pub(super) fn gap_line(&self, d: &Drive, offset: u64, size: u64, kinds: &[(&'static str, String)]) -> Node<Msg> {
        let mut controls = Vec::new();
        if self.gap_able(d, kinds) {
            controls.push(self.toggle("list-add-symbolic", t!("disks-new-part-tip"), format!("{}:{offset}", d.path)));
        }
        let name = label(t!("disks-free-space"));
        let name = if self.drive_hidden(d) { name.opacity(hidden_share()) } else { name };
        self.line(&d.path, "\u{0}", name, &size_text(size as f64), controls, None)
    }

    pub(super) fn gap_extra(&self, d: &Drive, offset: u64, size: u64, kinds: &[(&'static str, String)]) -> Node<Msg> {
        let gk = format!("{}:{offset}", d.path);
        let gb_all = size as f32 / GB as f32;
        let (nk, kk, bk) = (format!("{gk}:name"), format!("{gk}:kind"), format!("{gk}:gb"));
        let gb = self.gbs.get(&bk).copied().unwrap_or(gb_all);
        let (nkc, kkc, bkc) = (nk.clone(), kk.clone(), bk.clone());
        let controls = vec![
            entry(self.texts.get(&nk).cloned().unwrap_or_default(), t!("disks-name-optional"), move |t| Msg::Text(nkc.clone(), t))
                .key(nk.clone()).width(Length::Fixed(140.0)),
            choice(kinds.iter().map(|k| k.1.clone()).collect(), self.picks.get(&kk).copied().unwrap_or(0), t!("disks-kind"),
                move |i| Msg::Pick(kkc.clone(), i)).key(kk.clone()),
            slider(gb, 0.1f32.min(gb_all), gb_all, 0.1, t!("disks-size-gb"), move |g| Msg::Gb(bkc.clone(), g))
                .key(bk.clone()).width(Length::Fixed(160.0)),
            label(plan::gb_text(gb)),
            button("list-add-symbolic", t!("disks-make-tip")).on_press(Msg::Act(Act::Make { drive: d.path.clone(), offset, size })).off_because(self.shut()),
        ];
        let warning = t!("disks-make-warning", size = plan::gb_text(gb), drive = d.words());
        self.line(&d.path, "\u{0}", label(t!("disks-new-part")), t!("disks-new-part-note"),
            controls, Some((format!("make:{gk}"), warning)))
    }
}
