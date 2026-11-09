//! A drive's card and the line for each of its parts.

use super::plan::size_text;
use crate::common::hidden_drives;
use crate::toolkit::widget::{Length, Node, Tone, button, card, column, heading, icon_circle, label, note, row, space};
use crate::toolkit::{t, theme};
use super::{Act, Drives, Msg, help, plan, red};
use super::model::Vol;
use super::model::Drive;
use super::view::SHORT_LINE;
use super::map::bands;
use super::map::hidden_share;

impl Drives {

    pub(super) fn drive_cards(&self) -> Vec<Node<Msg>> {
        if self.drives.is_empty() {
            return Vec::new();
        }
        let kinds = self.kinds();
        self.drives.iter().map(|d| self.drive_card(d, &kinds)).collect()
    }

    fn drive_card(&self, d: &Drive, kinds: &[(&'static str, String)]) -> Node<Msg> {
        let b = theme::bento();
        let hidden = self.drive_hidden(d);
        let mut items = self.drive_head(d, kinds, hidden);
        items.extend(self.drive_status(d));
        let rows = self.part_lines(d, kinds);
        if !rows.is_empty() {
            items.push(bands(rows));
        }
        card(column(items).gap(12.0)).height(Length::Fill).beside().span(b.card.0, b.card.1, b.words)
    }

    fn drive_head(&self, d: &Drive, kinds: &[(&'static str, String)], hidden: bool) -> Vec<Node<Msg>> {
        let mut size = size_text(d.size as f64);
        let mut trouble = None;
        if d.size == 0 && !d.optical {
            size = t!("disks-no-answer").into();
            trouble = Some(t!("disks-no-answer-help"));
        }

        let dim = |n: Node<Msg>, yes: bool| if yes { n.opacity(hidden_share()) } else { n };
        let icon = if d.optical {
            "media-optical-symbolic"
        } else if d.removable {
            "media-removable-symbolic"
        } else {
            "drive-harddisk-symbolic"
        };
        let mut head = vec![dim(icon_circle(icon), hidden), dim(heading(d.name.clone()), hidden), space()];
        head.extend(self.drive_controls(d, kinds, hidden));
        let mut items = vec![row(head).gap(12.0).centred(), note(if d.removable { t!("disks-drive-removable", size = &size) } else { t!("disks-drive-internal", size = &size) })];
        if hidden {
            items.push(note(t!("disks-hidden-note", lock = hidden_drives::NOT_A_LOCK)));
        }
        if d.system {
            items.push(note(t!("disks-system-note")));
        }
        if let Some(words) = trouble {
            items.push(red(words, help::HEALTH));
        }
        items
    }

    fn drive_controls(&self, d: &Drive, kinds: &[(&'static str, String)], hidden: bool) -> Vec<Node<Msg>> {
        let mut controls = Vec::new();
        if d.removable && !d.system {
            controls.push(button("media-eject-symbolic", t!("disks-remove-safely"))
                .on_press(Msg::Act(Act::Off { drive: d.path.clone(), method: "Eject" })));
        }
        if d.can_power_off && !d.system {
            controls.push(button("system-shutdown-symbolic", t!("disks-power-off-tip"))
                .on_press(Msg::Act(Act::Off { drive: d.path.clone(), method: "PowerOff" })));
        }
        let wipe = (!d.system && !d.optical && d.size > 0 && !kinds.is_empty()).then_some(d.whole.as_ref()).flatten();
        if wipe.is_some() {
            controls.push(self.toggle("edit-clear-symbolic", t!("disks-erase-whole"), format!("{}:wipe", d.path)));
        }

        if !d.system && !d.key.is_empty() {
            controls.push(self.eye(&d.path, "", &d.key, hidden, "drive"));
        }
        controls
    }

    fn drive_status(&self, d: &Drive) -> Vec<Node<Msg>> {
        let mut items = Vec::new();
        if let Some((w, t)) = self.told(&d.path, "") {
            items.push(match self.told_page(&d.path, "") {
                Some(page) if t == Tone::Problem => red(w, page),
                _ => note(w).tone(t),
            });
        }
        if let Some(h) = &d.health {

            let (line, ok) = plan::health_line(h);
            let tone = if ok { Tone::Plain } else { Tone::Problem };
            let said = |words: String| if ok { note(words).tone(tone) } else { red(words, help::HEALTH) };
            if line.chars().count() <= SHORT_LINE {
                items.push(said(line));
            } else {
                let (state, facts, _) = plan::health_parts(h);
                items.push(said(state));
                items.push(note(facts));
            }
        }

        if d.vols.is_empty() && d.gaps.is_empty() && d.size > 0 && !d.optical {
            items.push(note(t!("disks-no-table")));
        }
        items
    }

    fn part_lines(&self, d: &Drive, kinds: &[(&'static str, String)]) -> Vec<Node<Msg>> {
        let mut rows = Vec::new();
        let mut gaps = d.gaps.iter().peekable();
        for v in &d.vols {
            let at = v.part.map(|p| p.0).unwrap_or(0);
            while let Some(&&(o, s)) = gaps.peek() {
                if o >= at {
                    break;
                }
                rows.push(self.gap_line(d, o, s, kinds));
                gaps.next();
            }
            rows.push(self.part_line(d, v, kinds));
        }
        for &(o, s) in gaps {
            rows.push(self.gap_line(d, o, s, kinds));
        }
        rows
    }

    fn part_line(&self, d: &Drive, v: &Vol, kinds: &[(&'static str, String)]) -> Node<Msg> {
        let mut sub = vec![v.kind_words(), size_text(v.size as f64)];
        if let Some((_, free)) = v.full {
            sub.push(t!("disks-free", size = size_text(free as f64)));
        }
        if v.unmounted() {
            sub.push(t!("disks-part-not-mounted").into());
        }
        match &v.fstab {
            Some(f) if !v.system => sub.push(if plan::starts_at_boot(&f.opts) {
                t!("disks-part-startup", dir = &f.dir)
            } else {
                t!("disks-part-appears", dir = &f.dir)
            }),
            _ => {
                if let Some(m) = v.mounts.first() {
                    sub.push(t!("disks-part-in", dir = m));
                }
            }
        }
        let (dp, vp) = (d.path.clone(), v.path.clone());
        let mut controls = Vec::new();
        if let Some(m) = v.mounts.first() {
            controls.push(button("folder-open-symbolic", t!("disks-open")).on_press(Msg::Act(Act::Open(m.clone()))));
        }
        if v.has_fs && !v.system {
            let on = v.mounts.is_empty();
            let (icon, tip) = if on { ("media-playback-start-symbolic", t!("disks-mount")) } else { ("media-playback-stop-symbolic", t!("disks-unmount")) };
            controls.push(button(icon, tip).on_press(Msg::Act(Act::Mount { drive: dp, vol: vp, on })));
        }
        if !self.part_extras(d, v, kinds).is_empty() {
            controls.push(self.toggle("document-edit-symbolic", t!("disks-change-it"), v.path.clone()));
        }

        if !d.system && !v.key.is_empty() && !self.drive_hidden(d) {
            controls.push(self.eye(&d.path, &v.path, &v.key, self.part_hidden(d, v), "part"));
        }

        let name = label(v.name());
        let name = if self.part_hidden(d, v) { name.opacity(hidden_share()) } else { name };
        self.line(&d.path, &v.path, name, &sub.join(", "), controls, None)
    }

    pub(super) fn part_extras(&self, d: &Drive, v: &Vol, kinds: &[(&'static str, String)]) -> Vec<Node<Msg>> {
        let caps = self.caps.get(&v.fs_type).copied().unwrap_or_default();
        let mut extras = Vec::new();
        if v.has_fs && !v.system && v.fs_type != "swap" {
            extras.push(self.label_row(d, v));
        }
        if v.nameable() {
            extras.push(self.folder_row(d, v));
            extras.push(self.startup_row(d, v));
        }
        if v.part.is_some_and(|p| !p.2) && !d.system && v.has_fs && caps.resize != 0 {
            extras.extend(self.resize_row(d, v));
        }
        extras.extend(self.room_row(d, v));
        if v.has_fs && !v.system && (caps.check || caps.repair) {
            extras.push(self.check_row(d, v, caps));
        }
        if !d.system && !d.optical && !kinds.is_empty() && !v.container() {
            extras.push(self.format_row(d, &v.path, false, kinds));
        }
        if v.part.is_some() && !d.system {
            extras.push(self.delete_row(d, v));
        }
        extras
    }
}
