//! Appearance: reading the screen aloud, and the labels that stay beside the ring's tabs.

use super::{Appearance, says};
use crate::common::files::set_setting;
use crate::common::{run, screen_reader, tip};
use crate::settings::ui::{group, line, on, Kit, Msg, not_saved};
use crate::toolkit::t;
use crate::toolkit::widget::{note, row, switch, Length, Node};

impl Appearance {

    fn keep_labels(&mut self, kit: &mut Kit, yes: bool) {
        if kit.settle(LABELS, set_setting(tip::KEY, if yes { "on" } else { "off" }), not_saved()).is_none() {
            return;
        }
        self.labels_stay = yes;
        run::spawn(&["trindle-ring", "--restyle"]);
    }

    fn read_aloud(&mut self, kit: &mut Kit, on: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SCREEN_READER) {
            return;
        }
        if !screen_reader::installed() {
            kit.say(READER, screen_reader::NONE_YET, false, false);
            return;
        }
        if kit.settle(READER, screen_reader::set(on, true), not_saved()).is_some() {
            self.reading = on;
        }
    }

    fn reader_line(&self, kit: &Kit) -> Node<Msg> {
        let sub = if self.installed { t!("settings-appearance-reader-on") } else { screen_reader::NONE_YET };
        let mut controls = Vec::new();
        if let Some(d) = kit.dials.get(READER) {
            controls.push(row(vec![d.node(), note(d.words()).tone(d.tone())]).gap(8.0).width(Length::Shrink));
        }
        controls.push(switch(self.reading, t!("settings-appearance-reader"),
                             |yes| on::<Appearance>(move |me, k| me.read_aloud(k, yes))).off(!self.installed));
        line(t!("settings-appearance-reader")).sub(sub).key(READER).controls(controls).node(kit)
    }

    fn labels_line(&self, kit: &Kit) -> Node<Msg> {
        let stay = switch(self.labels_stay, t!("settings-appearance-labels"),
                          |yes| on::<Appearance>(move |me, k| me.keep_labels(k, yes)));
        line(t!("settings-appearance-labels"))
            .sub(t!("settings-appearance-labels-about"))
            .key(LABELS).control(stay).node(kit)
    }

    pub(super) fn reading_card(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-appearance-reading-title")).icon("preferences-desktop-accessibility-symbolic")
            .about(t!("settings-appearance-reading-about", says = says()))
            .rows(vec![self.reader_line(kit), self.labels_line(kit)])
            .node(kit)
    }

}

const READER: &str = "appearance:reader";
const LABELS: &str = "appearance:labels";
