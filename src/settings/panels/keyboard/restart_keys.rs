//! Settings, Keyboard: the keys kept in Settings (the two that restart the desktop, and the screenshot's, the shortcut list's and the problem report's), each in a box to type in, with a button beside it that does the same.

use super::{Keyboard, KEYS_PAUSE};
use crate::common::combos::{self, Combo};
use crate::common::files::{self, set_setting, setting};
use crate::common::restart::{ALL_KEYS, PARTS_KEYS};
use crate::plugins::shortcuts;
use crate::settings::ui::{group, line, on, why, Kit, Msg};
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
    set_setting(key, &kept).map_err(|e| not_kept(&e))
}

fn not_kept(e: &std::io::Error) -> String {
    t!("settings-keyboard-restart-not-saved", why = why(&e.to_string(), "").trim()).trim_end().to_string()
}

fn runs(line_key: &'static str, command: &'static str, name: &'static str) -> Msg {
    on::<Keyboard>(move |_, k| {
        let started = crate::common::run::spawn(&command.split(' ').collect::<Vec<_>>());
        k.started(line_key, started, &t!("settings-common-could-not-open", name = name));
    })
}

struct Shortcut {
    key: &'static str,
    id: &'static str,
    label: &'static str,
    tip: &'static str,
    icon: &'static str,
}

impl Keyboard {

    fn keys_row(&self, kit: &Kit, Shortcut { key, id, label, tip, icon }: Shortcut, press: Msg) -> Node<Msg> {
        let shown = self.restart_typed.iter().find(|(k, _)| *k == key).map_or_else(|| setting(key), |(_, t)| t.clone());
        line(label).key(id)
            .control(entry(shown, t!("settings-keyboard-restart-keys-name", label = label), move |text| {
                on::<Keyboard>(move |me, k| me.restart_typed(k, key, id, text.clone()))
            }).width(Length::Fixed(220.0)))
            .control(button(icon, tip).on_press(press))
            .node(kit)
    }

    pub(super) fn restart_card(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-keyboard-restart-title")).icon("system-reboot-symbolic").rows(vec![
            self.keys_row(kit, Shortcut { key: PARTS_KEYS, id: PARTS_LINE, label: t!("settings-keyboard-restart-parts"),
                                          tip: t!("settings-keyboard-restart-parts-tip"), icon: "view-refresh-symbolic" },
                on::<Keyboard>(|me, k| me.restart_parts(k))),
            self.keys_row(kit, Shortcut { key: ALL_KEYS, id: ALL_LINE, label: t!("settings-keyboard-restart-all"),
                                          tip: t!("settings-keyboard-restart-all-tip"), icon: "system-reboot-symbolic" },
                runs(ALL_LINE, crate::common::restart::ASK_COMMAND, t!("settings-keyboard-restart-all"))),
        ]).node(kit)
    }

    pub(super) fn shortcuts_card(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-keyboard-shortcuts-title")).icon("input-keyboard-symbolic").rows(vec![
            self.keys_row(kit, Shortcut { key: combos::SCREENSHOT_KEYS, id: SHOT_LINE, label: t!("settings-keyboard-shortcuts-screenshot"),
                                          tip: t!("settings-keyboard-shortcuts-screenshot-tip"), icon: "camera-photo-symbolic" },
                runs(SHOT_LINE, combos::SCREENSHOT_COMMAND, t!("settings-keyboard-shortcuts-screenshot"))),
            self.keys_row(kit, Shortcut { key: combos::LIST_KEYS, id: LIST_LINE, label: t!("settings-keyboard-shortcuts-list"),
                                          tip: t!("settings-keyboard-shortcuts-list-tip"), icon: "input-keyboard-symbolic" },
                runs(LIST_LINE, combos::LIST_COMMAND, t!("settings-keyboard-shortcuts-list"))),
            self.keys_row(kit, Shortcut { key: combos::REPORT_KEYS, id: REPORT_LINE, label: t!("settings-keyboard-shortcuts-report"),
                                          tip: t!("settings-keyboard-shortcuts-report-tip"), icon: "dialog-warning-symbolic" },
                runs(REPORT_LINE, combos::REPORT_COMMAND, t!("settings-keyboard-shortcuts-report"))),
        ]).node(kit)
    }

    fn restart_typed(&mut self, kit: &mut Kit, key: &'static str, id: &'static str, text: String) {
        self.restart_typed.retain(|(k, _)| *k != key);
        self.restart_typed.push((key, text));
        let mine = self.typing.next(id);
        kit.after::<Keyboard>(KEYS_PAUSE, move |me, k| {
            if !me.typing.latest(id, mine) {
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
    fn a_shortcut_not_kept_says_why_in_plain_words() {
        let full = not_kept(&std::io::Error::from_raw_os_error(28));
        assert!(full.contains(t!("settings-common-why-full")), "{full}");
        assert!(!full.contains("os error") && !full.contains("28"), "{full}");
        let odd = not_kept(&std::io::Error::other("EBADF stale handle"));
        assert_eq!(odd, t!("settings-keyboard-restart-not-saved", why = "").trim_end());
    }

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

    #[test]
    fn typing_in_a_second_box_does_not_cancel_the_first_boxs_save() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = Kit::new(events, asks, None, None);
        let mut page = Keyboard::default();
        page.restart_typed(&mut kit, PARTS_KEYS, PARTS_LINE, "Ctrl+Alt+F9".into());
        page.restart_typed(&mut kit, ALL_KEYS, ALL_LINE, "Ctrl+Alt+F10".into());
        assert!(page.typing.latest(PARTS_LINE, 1) && page.typing.latest(ALL_LINE, 1), "each box's last key is still the one its save waits for");
        page.restart_typed(&mut kit, PARTS_KEYS, PARTS_LINE, "Ctrl+Alt+F8".into());
        assert!(!page.typing.latest(PARTS_LINE, 1) && page.typing.latest(PARTS_LINE, 2), "another key in the same box replaces the wait");
    }

    #[test]
    fn changed_keys_are_not_saved_over_a_file_that_could_not_be_read() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = Kit::new(events, asks, None, None);
        let mut page = Keyboard { table_unread: Some("not text".into()), ..Keyboard::default() };
        page.keep_keys(&mut kit, "keys:test");
        assert_eq!(kit.said("keys:test").map(|(w, red)| (w.to_string(), red)), Some((t!("settings-keyboard-keys-not-saved-unread").to_string(), true)));
    }

    #[test]
    fn a_button_that_cannot_start_its_program_says_so_under_its_line() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut s = crate::settings::Settings { kit: Kit::new(events, asks, None, None), pages: vec![Box::new(Keyboard::default())],
                                                current: 0, search: String::new(), tall: 0.0 };
        (runs("test-line", "trindle-no-such-program-for-the-test", "Test").0)(&mut s);
        assert_eq!(s.kit.said("test-line").map(|(_, red)| red), Some(true));
    }
}
