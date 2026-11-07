//! The shortcut list.

use quick_xml::events::Event;
use quick_xml::Reader;

use super::combos::{self, Combo};
use super::files;
use crate::toolkit::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Trindle,
    Plugin,
    Yours,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub group: Group,
    pub words: String,
    pub keys: String,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub keys: Combo,
    pub group: Group,
    pub words: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Bind {
    pub key: String,
    pub list: Option<String>,
    pub action: String,
    pub command: String,
}

const FIXED: &[(&str, &str)] = &[("console", "console-keys"), ("reader-keys", "reader-keys-keys")];

pub fn words_of(id: &str) -> Option<&'static str> {
    Some(match id {
        "ring" => t!("ring-shortcuts-ring"),
        "terminal" => t!("ring-shortcuts-terminal"),
        "settings" => t!("ring-shortcuts-settings"),
        "close" => t!("ring-shortcuts-close"),
        "lock" => t!("ring-shortcuts-lock"),
        "desktop" => t!("ring-shortcuts-desktop"),
        "notes" => t!("ring-shortcuts-notes"),
        "activity" => t!("ring-shortcuts-activity"),
        "reader" => t!("ring-shortcuts-reader"),
        "maximise" => t!("ring-shortcuts-maximise"),
        "minimise" => t!("ring-shortcuts-minimise"),
        "fullscreen" => t!("ring-shortcuts-fullscreen"),
        "next-window" => t!("ring-shortcuts-next-window"),
        "previous-window" => t!("ring-shortcuts-previous-window"),
        "snap-left" => t!("ring-shortcuts-snap-left"),
        "snap-right" => t!("ring-shortcuts-snap-right"),
        "screenshot" => t!("ring-shortcuts-screenshot"),
        "report" => t!("ring-shortcuts-report"),
        "screenshot-whole" => t!("ring-shortcuts-screenshot-whole"),
        "volume-up" => t!("ring-shortcuts-volume-up"),
        "volume-down" => t!("ring-shortcuts-volume-down"),
        "mute" => t!("ring-shortcuts-mute"),
        "mic-mute" => t!("ring-shortcuts-mic-mute"),
        "brightness-up" => t!("ring-shortcuts-brightness-up"),
        "brightness-down" => t!("ring-shortcuts-brightness-down"),
        "restart-parts" => t!("ring-shortcuts-restart-parts"),
        "restart-all" => t!("ring-shortcuts-restart-all"),
        "shortcuts" => t!("ring-shortcuts-list"),
        "console" => t!("ring-shortcuts-console"),
        "reader-keys" => t!("ring-shortcuts-reader-keys"),
        "other" => t!("ring-shortcuts-other"),
        _ => return None,
    })
}

pub fn standard_id(rc: &str) -> Option<&'static str> {
    Some(match rc {
        "A-Tab" => "next-window",
        "A-S-Tab" => "previous-window",
        "A-F4" => "close",
        "W-a" | "W-Up" => "maximise",
        "W-Left" => "snap-left",
        "W-Right" => "snap-right",
        "W-Down" => "minimise",
        "F11" => "fullscreen",
        _ => return None,
    })
}

pub fn setting_id(setting: &str) -> Option<&'static str> {
    Some(match setting {
        "restart-parts-keys" => "restart-parts",
        "restart-all-keys" => "restart-all",
        combos::SCREENSHOT_KEYS => "screenshot",
        combos::LIST_KEYS => "shortcuts",
        combos::REPORT_KEYS => "report",
        _ => return None,
    })
}

fn described(b: &Bind) -> String {
    let id = match b.action.as_str() {
        "Execute" if !b.command.is_empty() => return t!("ring-shortcuts-runs", command = b.command.as_str()),
        "Close" => "close",
        "ToggleMaximize" => "maximise",
        "ToggleFullscreen" => "fullscreen",
        "Iconify" => "minimise",
        "NextWindow" => "next-window",
        "PreviousWindow" => "previous-window",
        "SnapLeft" => "snap-left",
        "SnapRight" => "snap-right",
        _ => "other",
    };
    words_of(id).unwrap_or_default().to_string()
}

pub fn binds(text: &str) -> Vec<Bind> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().trim_text(true);
    let mut found: Vec<Bind> = Vec::new();
    let mut open: Option<Bind> = None;
    let attr = |e: &quick_xml::events::BytesStart, name: &str| -> Option<String> {
        e.attributes().flatten().find(|a| a.key.as_ref() == name.as_bytes())
            .and_then(|a| a.normalized_value(quick_xml::XmlVersion::Implicit1_0).ok().map(|v| v.to_string()))
    };
    loop {
        match reader.read_event() {
            Ok(Event::Eof) | Err(_) => break,
            Ok(Event::Start(e)) if e.name().as_ref() == b"keybind" => {
                open = attr(&e, "key").map(|key| Bind { key, list: attr(&e, "list"), ..Bind::default() });
            }
            Ok(Event::Start(e) | Event::Empty(e)) if e.name().as_ref() == b"action" => {
                if let Some(b) = open.as_mut().filter(|b| b.action.is_empty()) {
                    b.action = attr(&e, "name").unwrap_or_default();
                    b.command = attr(&e, "command").unwrap_or_default();
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"keybind" => found.extend(open.take()),
            _ => {}
        }
    }
    found
}

pub fn shown(c: &Combo) -> String {
    let alone = !(c.logo || c.alt || c.ctrl || c.shift);
    match c.key.as_str() {
        "Super_L" | "Super_R" if alone => t!("ring-shortcuts-key-super").into(),
        "XF86AudioRaiseVolume" => t!("ring-shortcuts-key-volume-up").into(),
        "XF86AudioLowerVolume" => t!("ring-shortcuts-key-volume-down").into(),
        "XF86AudioMute" => t!("ring-shortcuts-key-mute").into(),
        "XF86AudioMicMute" => t!("ring-shortcuts-key-mic-mute").into(),
        "XF86MonBrightnessUp" => t!("ring-shortcuts-key-brightness-up").into(),
        "XF86MonBrightnessDown" => t!("ring-shortcuts-key-brightness-down").into(),
        _ => c.shown(),
    }
}

pub fn pressed(keys: &Combo, sym: crate::toolkit::input::Keysym, mods: crate::toolkit::input::Mods) -> bool {
    (keys.logo, keys.alt, keys.ctrl, keys.shift) == (mods.logo, mods.alt, mods.ctrl, mods.shift)
        && xkbcommon::xkb::keysym_get_name(sym).eq_ignore_ascii_case(&keys.key)
}

pub struct Sources<'a> {
    pub system_rc: &'a str,
    pub user_rc: &'a str,
    pub plugins: &'a [(String, Combo)],
    pub setting: &'a dyn Fn(&str) -> String,
}

pub fn entries(src: &Sources) -> Vec<Entry> {
    let mut all: Vec<Entry> = Vec::new();
    let mut put = |keys: Combo, group: Group, words: String| {
        all.retain(|e| e.keys != keys);
        all.push(Entry { keys, group, words });
    };
    for rc in combos::BUILT_IN {
        if let (Some(keys), Some(id)) = (Combo::from_rc(rc), standard_id(rc)) {
            put(keys, Group::Trindle, words_of(id).unwrap_or_default().to_string());
        }
    }
    for b in binds(src.system_rc) {
        if let Some(keys) = Combo::from_rc(&b.key) {
            let words = b.list.as_deref().and_then(words_of).map_or_else(|| described(&b), str::to_string);
            put(keys, Group::Trindle, words);
        }
    }
    for (words, keys) in src.plugins {
        put(keys.clone(), Group::Plugin, words.clone());
    }
    for b in binds(src.user_rc) {
        if let Some(keys) = Combo::from_rc(&b.key) {
            put(keys, Group::Yours, described(&b));
        }
    }
    for setting in combos::SETTING_KEYS {
        if let (Ok(keys), Some(id)) = (Combo::typed(&(src.setting)(setting)), setting_id(setting)) {
            put(keys, Group::Trindle, words_of(id).unwrap_or_default().to_string());
        }
    }
    all
}

pub fn rows_of(src: &Sources) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for e in entries(src) {
        let keys = shown(&e.keys);
        match rows.iter_mut().find(|r| r.group == e.group && r.words == e.words) {
            Some(r) => r.keys = t!("ring-shortcuts-and", first = r.keys.as_str(), next = keys.as_str()),
            None => rows.push(Row { group: e.group, words: e.words, keys }),
        }
    }
    for (id, keys) in FIXED {
        if let (Some(words), Some(keys)) = (words_of(id), fixed_keys(keys)) {
            rows.push(Row { group: Group::Trindle, words: words.to_string(), keys: keys.to_string() });
        }
    }
    rows.sort_by_key(|r| r.group as u8);
    rows
}

fn fixed_keys(id: &str) -> Option<&'static str> {
    Some(match id {
        "console-keys" => t!("ring-shortcuts-console-keys"),
        "reader-keys-keys" => t!("ring-shortcuts-reader-keys-keys"),
        _ => return None,
    })
}

pub fn rows() -> Vec<Row> {
    let plugins: Vec<(String, Combo)> = crate::plugins::shortcuts::bindings().into_iter().filter_map(|b| {
        let keys = b.keys?;
        Some((t!("ring-shortcuts-plugin", plugin = b.plugin_name.as_str(), label = b.label.as_str()), keys))
    }).collect();
    let system = files::read_text(files::system_desktop("shortcuts.xml"));
    let user = files::read_text(files::trindle_config().join("shortcuts.xml"));
    rows_of(&Sources { system_rc: &system, user_rc: &user, plugins: &plugins, setting: &files::setting })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn own() -> String {
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/desktop/shortcuts.xml")).unwrap()
    }

    fn defaults(key: &str) -> String {
        files::default_setting(key).to_string()
    }

    fn list(user: &str, plugins: &[(String, Combo)], setting: &dyn Fn(&str) -> String) -> Vec<Row> {
        rows_of(&Sources { system_rc: &own(), user_rc: user, plugins, setting })
    }

    fn listed(rows: &[Row], keys: &Combo, words: &str) -> bool {
        let said = shown(keys);
        rows.iter().any(|r| r.words == words && r.keys.split(", ").any(|k| k == said))
    }

    #[test]
    fn every_keybind_trindle_ships_has_words_and_is_on_the_list() {
        let rows = list("", &[], &defaults);
        let shipped = binds(&own());
        assert!(shipped.len() > 15, "the shipped keybinds were not read");
        for b in shipped {
            let id = b.list.as_deref().unwrap_or_else(|| panic!("{} has no list name in shortcuts.xml", b.key));
            let words = words_of(id).unwrap_or_else(|| panic!("{} names {id}, which has no words", b.key));
            let keys = Combo::from_rc(&b.key).unwrap_or_else(|| panic!("{} is not a key", b.key));
            assert!(listed(&rows, &keys, words), "{} is bound and not on the list", b.key);
        }
        assert!(own().contains("<default />"), "the standard keys are not asked for, so the list should not show them");
    }

    #[test]
    fn every_standard_key_and_every_key_kept_in_settings_is_on_the_list() {
        let rows = list("", &[], &defaults);
        for rc in combos::BUILT_IN {
            let id = standard_id(rc).unwrap_or_else(|| panic!("{rc} is a standard key with no words"));
            assert!(listed(&rows, &Combo::from_rc(rc).unwrap(), words_of(id).unwrap()), "{rc}");
        }
        for setting in combos::SETTING_KEYS {
            let id = setting_id(setting).unwrap_or_else(|| panic!("{setting} is kept in Settings with no words"));
            let keys = Combo::typed(&defaults(setting)).unwrap_or_else(|e| panic!("{setting} has no valid default: {e}"));
            assert!(listed(&rows, &keys, words_of(id).unwrap()), "{setting}");
        }
        for (id, keys) in FIXED {
            assert!(words_of(id).is_some() && fixed_keys(keys).is_some(), "{id}");
            assert!(rows.iter().any(|r| r.words == words_of(id).unwrap()), "{id}");
        }
    }

    #[test]
    fn the_lists_own_key_and_the_screenshot_key_are_on_it() {
        let rows = list("", &[], &defaults);
        assert!(listed(&rows, &Combo::typed(&defaults(combos::LIST_KEYS)).unwrap(), words_of("shortcuts").unwrap()));
        assert!(listed(&rows, &Combo::typed(&defaults(combos::SCREENSHOT_KEYS)).unwrap(), words_of("screenshot").unwrap()));
        let shot: Vec<&Row> = rows.iter().filter(|r| r.words == words_of("screenshot").unwrap()).collect();
        assert_eq!(shot.len(), 1);
        assert_eq!(shot[0].keys.split(", ").count(), 3, "{}", shot[0].keys);
    }

    #[test]
    fn a_shortcut_the_person_set_up_is_on_the_list_as_theirs() {
        let theirs = r#"<trindle_desktop><keyboard>
            <keybind key="C-A-g"><action name="Execute" command="foot --title Notes" /></keybind>
            <keybind key="C-A-m"><action name="ToggleMaximize" /></keybind>
        </keyboard></trindle_desktop>"#;
        let rows = list(theirs, &[], &defaults);
        let row = rows.iter().find(|r| r.keys == "Ctrl+Alt+G").expect("the person's shortcut is not on the list");
        assert_eq!((row.group, row.words.as_str()), (Group::Yours, "Runs foot --title Notes"));
        assert!(rows.iter().any(|r| r.group == Group::Yours && r.keys == "Ctrl+Alt+M" && r.words == words_of("maximise").unwrap()));
        assert!(rows.iter().position(|r| r.group == Group::Yours) > rows.iter().rposition(|r| r.group == Group::Trindle));
    }

    #[test]
    fn a_person_s_key_over_one_of_trindle_s_shows_once_as_theirs() {
        let theirs = r#"<trindle_desktop><keyboard><keybind key="W-l"><action name="Execute" command="my-lock" /></keybind></keyboard></trindle_desktop>"#;
        let rows = list(theirs, &[], &defaults);
        let lock = Combo::typed("Super+L").unwrap();
        let on_lock: Vec<&Row> = rows.iter().filter(|r| r.keys.split(", ").any(|k| k == lock.shown())).collect();
        assert!(!on_lock.is_empty() && on_lock.iter().all(|r| r.group == Group::Yours && r.words == "Runs my-lock"), "{on_lock:?}");
        assert!(!listed(&rows, &lock, words_of("lock").unwrap()));
    }

    #[test]
    fn keys_changed_in_settings_show_as_changed_and_off_ones_not_at_all() {
        let changed = |k: &str| match k {
            combos::SCREENSHOT_KEYS => "Ctrl+Alt+P".to_string(),
            combos::LIST_KEYS => String::new(),
            other => defaults(other),
        };
        let rows = list("", &[], &changed);
        assert!(listed(&rows, &Combo::typed("Ctrl+Alt+P").unwrap(), words_of("screenshot").unwrap()));
        assert!(!listed(&rows, &Combo::typed(&defaults(combos::SCREENSHOT_KEYS)).unwrap(), words_of("screenshot").unwrap()));
        assert!(!listed(&rows, &Combo::typed(&defaults(combos::LIST_KEYS)).unwrap(), words_of("shortcuts").unwrap()));
    }

    #[test]
    fn plugin_shortcuts_are_on_the_list_with_the_keys_in_force() {
        let plugins = [("Flow: Switch computer".to_string(), Combo::typed("Ctrl+Alt+S").unwrap())];
        let rows = list("", &plugins, &defaults);
        assert!(rows.iter().any(|r| r.group == Group::Plugin && r.words == "Flow: Switch computer" && r.keys == "Ctrl+Alt+S"));
    }

    #[test]
    fn no_default_takes_the_keys_of_another() {
        let mut seen: Vec<(Combo, String)> = Vec::new();
        let mut add = |c: Combo, who: String| {
            if let Some((_, other)) = seen.iter().find(|(s, _)| *s == c) {
                panic!("{who} and {other} have the same keys, {}", c.shown());
            }
            seen.push((c, who));
        };
        let file: Vec<Bind> = binds(&own());
        for b in &file {
            add(Combo::from_rc(&b.key).unwrap(), b.key.clone());
        }
        for rc in combos::BUILT_IN.iter().filter(|rc| !file.iter().any(|b| b.key == **rc)) {
            add(Combo::from_rc(rc).unwrap(), (*rc).to_string());
        }
        for s in combos::SETTING_KEYS {
            add(Combo::typed(&defaults(s)).unwrap(), (*s).to_string());
        }
    }

    #[test]
    fn the_list_s_key_is_known_when_pressed_with_the_keys_held_and_no_others() {
        use crate::toolkit::input::{Keysym, Mods};
        let keys = Combo::typed(&defaults(combos::LIST_KEYS)).unwrap();
        let held = Mods { logo: true, ..Mods::default() };
        assert!(pressed(&keys, Keysym::k, held));
        assert!(pressed(&keys, Keysym::K, held), "caps lock must not matter");
        assert!(!pressed(&keys, Keysym::k, Mods::default()));
        assert!(!pressed(&keys, Keysym::k, Mods { shift: true, ..held }));
        assert!(!pressed(&keys, Keysym::j, held));
    }

    #[test]
    fn a_keybind_is_read_with_its_action_and_command_and_the_mouse_is_left_out() {
        let text = r#"<k><keyboard><keybind key="A-x" list="ring"><action name="Execute" command="a b" /></keybind>
            <keybind key="C-q"><action name="Close" /></keybind></keyboard>
            <mouse><context name="Root"><mousebind button="Left" action="DoubleClick"><action name="Execute" command="z" /></mousebind></context></mouse></k>"#;
        assert_eq!(binds(text), vec![
            Bind { key: "A-x".into(), list: Some("ring".into()), action: "Execute".into(), command: "a b".into() },
            Bind { key: "C-q".into(), list: None, action: "Close".into(), command: String::new() },
        ]);
    }
}
