//! Settings, Keyboard.

use super::{Keyboard, KEYS_PAUSE};
use crate::common::combos::{self, Combo};
use crate::common::files::{self, set_setting, setting};
use crate::common::restart::{ALL_KEYS, PARTS_KEYS};
use crate::plugins::shortcuts;
use crate::settings::ui::{group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, Length, Node};

const PARTS_LINE: &str = "restart-parts";
const ALL_LINE: &str = "restart-all";
const SHOT_LINE: &str = "screenshot-keys";
const LIST_LINE: &str = "shortcut-list-keys";
const REPORT_LINE: &str = "report-keys";

fn taken() -> Vec<Combo> {
    let mut all: Vec<Combo> = combos::BUILT_IN.iter().filter_map(|c| Combo::from_rc(c)).collect();
    for file in [files::system_desktop("shortcuts.xml"), files::trindle_config().join("shortcuts.xml")] {
        all.extend(combos::rc_combos(&files::read_text(&file)));
    }
    all.extend(shortcuts::bindings().into_iter().filter_map(|b| b.keys));
    all
}

fn clashes(key: &str, keys: &Combo, taken: &[Combo], setting: &dyn Fn(&str) -> String) -> bool {
    taken.contains(keys) || combos::SETTING_KEYS.iter().any(|o| *o != key && Combo::typed(&setting(o)).is_ok_and(|c| c == *keys))
}

pub fn choose(key: &str, typed: &str) -> Result<(), String> {
    let kept = if typed.trim().is_empty() {
        String::new()
    } else {
        let keys = Combo::typed(typed)?;
        if clashes(key, &keys, &taken(), &setting) {
            return Err(t!("settings-keyboard-restart-clash", keys = keys.shown()));
        }
        keys.shown()
    };
    set_setting(key, &kept).map_err(|e| t!("settings-keyboard-restart-not-saved", why = e.to_string()))
}

struct Shortcut {
    key: &'static str,
    id: &'static str,
    label: &'static str,
    about: &'static str,
    tip: &'static str,
    icon: &'static str,
}

impl Keyboard {
    fn keys_row(&self, kit: &Kit, Shortcut { key, id, label, about, tip, icon }: Shortcut, press: Msg) -> Node<Msg> {
        let shown = self.restart_typed.iter().find(|(k, _)| *k == key).map_or_else(|| setting(key), |(_, t)| t.clone());
        line(label).sub(about).key(id)
            .control(entry(shown, t!("settings-keyboard-restart-keys-name", label = label), move |text| {
                on::<Keyboard>(move |me, k| me.restart_typed(k, key, id, text.clone()))
            }).width(Length::Fixed(220.0)))
            .control(button(icon, tip).on_press(press))
            .node(kit)
    }

    pub(super) fn restart_card(&self, kit: &Kit) -> Node<Msg> {
        let about = t!("settings-keyboard-restart-about");
        group(t!("settings-keyboard-restart-title")).icon("system-reboot-symbolic").rows(vec![
            self.keys_row(kit, Shortcut { key: PARTS_KEYS, id: PARTS_LINE, label: t!("settings-keyboard-restart-parts"), about,
                                          tip: t!("settings-keyboard-restart-parts-tip"), icon: "view-refresh-symbolic" },
                on::<Keyboard>(|me, k| me.restart_parts(k))),
            self.keys_row(kit, Shortcut { key: ALL_KEYS, id: ALL_LINE, label: t!("settings-keyboard-restart-all"), about,
                                          tip: t!("settings-keyboard-restart-all-tip"), icon: "system-reboot-symbolic" },
                on::<Keyboard>(|_, _| {
                    crate::common::run::spawn(&crate::common::restart::ASK_COMMAND.split(' ').collect::<Vec<_>>());
                })),
        ]).node(kit)
    }

    pub(super) fn shortcuts_card(&self, kit: &Kit) -> Node<Msg> {
        let about = t!("settings-keyboard-shortcuts-about");
        group(t!("settings-keyboard-shortcuts-title")).icon("input-keyboard-symbolic").rows(vec![
            self.keys_row(kit, Shortcut { key: combos::SCREENSHOT_KEYS, id: SHOT_LINE, label: t!("settings-keyboard-shortcuts-screenshot"), about,
                                          tip: t!("settings-keyboard-shortcuts-screenshot-tip"), icon: "camera-photo-symbolic" },
                on::<Keyboard>(|_, _| { crate::common::run::spawn(&[combos::SCREENSHOT_COMMAND]); })),
            self.keys_row(kit, Shortcut { key: combos::LIST_KEYS, id: LIST_LINE, label: t!("settings-keyboard-shortcuts-list"), about,
                                          tip: t!("settings-keyboard-shortcuts-list-tip"), icon: "input-keyboard-symbolic" },
                on::<Keyboard>(|_, _| { crate::common::run::spawn(&combos::LIST_COMMAND.split(' ').collect::<Vec<_>>()); })),
            self.keys_row(kit, Shortcut { key: combos::REPORT_KEYS, id: REPORT_LINE, label: t!("settings-keyboard-shortcuts-report"), about,
                                          tip: t!("settings-keyboard-shortcuts-report-tip"), icon: "dialog-warning-symbolic" },
                on::<Keyboard>(|_, _| { crate::common::run::spawn(&[combos::REPORT_COMMAND]); })),
        ]).node(kit)
    }

    fn restart_typed(&mut self, kit: &mut Kit, key: &'static str, id: &'static str, text: String) {
        self.restart_typed.retain(|(k, _)| *k != key);
        self.restart_typed.push((key, text));
        self.keys_round = self.keys_round.wrapping_add(1);
        let mine = self.keys_round;
        kit.after::<Keyboard>(KEYS_PAUSE, move |me, k| {
            if me.keys_round != mine {
                return;
            }
            let text = me.restart_typed.iter().find(|(n, _)| *n == key).map(|(_, t)| t.clone()).unwrap_or_default();
            k.done(id, choose(key, &text).err().as_deref(), t!("settings-keyboard-plugin-saved"));
        });
    }

    fn restart_parts(&mut self, kit: &mut Kit) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::RESTART_PARTS) {
            return;
        }
        crate::common::restart::parts(Vec::new());
        kit.say(PARTS_LINE, t!("settings-keyboard-restart-parts-done"), true, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_default_is_different_and_no_standard_shortcut_has_them() {
        let (parts, all) = (Combo::typed(crate::common::restart::PARTS_DEFAULT).unwrap(), Combo::typed(crate::common::restart::ALL_DEFAULT).unwrap());
        assert_ne!(parts, all);
        let (shot, list) = (Combo::typed(files::default_setting(combos::SCREENSHOT_KEYS)).unwrap(), Combo::typed(files::default_setting(combos::LIST_KEYS)).unwrap());
        assert!(![&parts, &all].contains(&&shot) && ![&parts, &all, &shot].contains(&&list));
        let standard: Vec<Combo> = combos::BUILT_IN.iter().filter_map(|c| Combo::from_rc(c)).chain(
            combos::rc_combos(include_str!("../../../../config/desktop/shortcuts.xml"))).collect();
        assert!(![&parts, &all, &shot, &list].iter().any(|c| standard.contains(c)));
    }

    #[test]
    fn keys_another_shortcut_kept_in_settings_has_are_refused_and_its_own_are_not() {
        let now = |k: &str| files::default_setting(k).to_string();
        let list = Combo::typed(&now(combos::LIST_KEYS)).unwrap();
        assert!(clashes(combos::SCREENSHOT_KEYS, &list, &[], &now));
        assert!(!clashes(combos::LIST_KEYS, &list, &[], &now), "a shortcut's own keys typed again are not a clash");
        let shot = Combo::typed(&now(combos::SCREENSHOT_KEYS)).unwrap();
        assert!(clashes(combos::LIST_KEYS, &shot, &[], &now));
        let free = Combo::typed("Ctrl+Alt+F9").unwrap();
        assert!(!clashes(combos::LIST_KEYS, &free, &[], &now) && clashes(combos::LIST_KEYS, &free, std::slice::from_ref(&free), &now));
    }

    #[test]
    fn keys_that_are_not_shortcuts_are_refused_before_anything_is_kept() {
        for typed in ["Escape", "Ctrl+Shift+Nonsense", "Ctrl+"] {
            assert!(choose(PARTS_KEYS, typed).is_err(), "{typed}");
        }
    }
}
