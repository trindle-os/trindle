//! Key combinations, as the compositor's shortcuts and a plugin's shortcuts write them.

use std::path::PathBuf;

pub const BUILT_IN: &[&str] =
    &["A-Tab", "A-S-Tab", "A-F4", "W-a", "W-Left", "W-Right", "W-Up", "W-Down", "F11"];

pub const SETTING_KEYS: &[&str] = &["restart-parts-keys", "restart-all-keys", SCREENSHOT_KEYS, LIST_KEYS, REPORT_KEYS];
pub const SCREENSHOT_KEYS: &str = "screenshot-keys";
pub const SCREENSHOT_COMMAND: &str = "trindle-screenshot";
pub const LIST_KEYS: &str = "shortcut-list-keys";
pub const LIST_COMMAND: &str = "trindle-ring --shortcuts";
pub const REPORT_KEYS: &str = "report-keys";
pub const REPORT_COMMAND: &str = "trindle-report";

const NAMED: &[&str] = &[
    "Tab", "Return", "space", "Escape", "BackSpace", "Delete", "Insert", "Home", "End", "Page_Up", "Page_Down",
    "Left", "Right", "Up", "Down", "Print", "Pause", "minus", "equal", "comma", "period", "slash", "semicolon",
    "apostrophe", "grave", "bracketleft", "bracketright", "backslash",
];

const ALIASES: &[(&str, &str)] = &[
    ("enter", "Return"), ("esc", "Escape"), ("backspace", "BackSpace"), ("del", "Delete"), ("ins", "Insert"),
    ("pageup", "Page_Up"), ("pgup", "Page_Up"), ("pagedown", "Page_Down"), ("pgdn", "Page_Down"),
    ("spacebar", "space"), ("-", "minus"), ("=", "equal"), (",", "comma"), (".", "period"), ("/", "slash"),
    (";", "semicolon"), ("'", "apostrophe"), ("`", "grave"), ("[", "bracketleft"), ("]", "bracketright"),
    ("\\", "backslash"),
];

#[derive(Clone, Debug, Default)]
pub struct Combo {
    pub logo: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub key: String,
}

impl PartialEq for Combo {
    fn eq(&self, other: &Combo) -> bool {
        (self.logo, self.alt, self.ctrl, self.shift) == (other.logo, other.alt, other.ctrl, other.shift)
            && self.key.eq_ignore_ascii_case(&other.key)
    }
}

impl Eq for Combo {}

impl Combo {
    pub fn typed(text: &str) -> Result<Combo, String> {
        let mut c = Combo::default();
        if text.chars().any(char::is_control) {
            return Err("A shortcut is typed on one line, in plain letters.".into());
        }
        let words: Vec<&str> = text.split('+').map(str::trim).collect();
        let (last, held) = match words.split_last() {
            Some((last, held)) if !last.is_empty() => (*last, held),
            _ => return Err("A shortcut ends in a key, such as Ctrl+Alt+S.".into()),
        };
        for m in held {
            match m.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => c.ctrl = true,
                "alt" => c.alt = true,
                "shift" => c.shift = true,
                "super" | "logo" | "win" | "windows" => c.logo = true,
                other => return Err(format!("{other:?} is not Ctrl, Alt, Shift or Super.")),
            }
        }
        c.key = key_named(last).ok_or_else(|| format!("{last:?} is not a key a shortcut may use."))?;
        if !(c.ctrl || c.alt || c.logo) {
            return Err("A shortcut needs Ctrl, Alt or Super held with its key.".into());
        }
        Ok(c)
    }

    pub fn from_rc(text: &str) -> Option<Combo> {
        let mut c = Combo::default();
        let parts: Vec<&str> = text.split('-').collect();
        let (key, held) = parts.split_last()?;
        for m in held {
            match *m {
                "W" | "Mod4" => c.logo = true,
                "A" | "Mod1" => c.alt = true,
                "C" => c.ctrl = true,
                "S" => c.shift = true,
                _ => return None,
            }
        }
        if key.is_empty() {
            return None;
        }
        c.key = (*key).to_string();
        Some(c)
    }

    pub fn rc(&self) -> String {
        let mut s = String::new();
        for (on, m) in [(self.logo, "W"), (self.alt, "A"), (self.ctrl, "C"), (self.shift, "S")] {
            if on {
                s.push_str(m);
                s.push('-');
            }
        }
        s + &self.key
    }

    pub fn shown(&self) -> String {
        let mut words: Vec<String> = Vec::new();
        for (on, m) in [(self.ctrl, "Ctrl"), (self.alt, "Alt"), (self.shift, "Shift"), (self.logo, "Super")] {
            if on {
                words.push(m.into());
            }
        }
        words.push(if self.key.chars().count() == 1 { self.key.to_uppercase() } else { self.key.clone() });
        words.join("+")
    }
}

fn key_named(word: &str) -> Option<String> {
    let lower = word.to_ascii_lowercase();
    if lower.len() == 1 && lower.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        return Some(lower);
    }
    if let Some(n) = lower.strip_prefix('f').and_then(|n| n.parse::<u32>().ok()).filter(|n| (1..=24).contains(n)) {
        if lower == format!("f{n}") {
            return Some(format!("F{n}"));
        }
    }
    let name = ALIASES.iter().find(|(a, _)| *a == lower).map_or(lower.as_str(), |(_, n)| n);
    NAMED.iter().find(|n| n.eq_ignore_ascii_case(name)).map(|n| (*n).to_string())
}

pub fn rc_combos(text: &str) -> Vec<Combo> {
    let mut open = String::new();
    let mut rest = text;
    while let Some(i) = rest.find("<!--") {
        open.push_str(&rest[..i]);
        rest = match rest[i..].find("-->") {
            Some(j) => &rest[i + j + 3..],
            None => "",
        };
    }
    open.push_str(rest);
    let mut found = Vec::new();
    for tag in open.split("<keybind").skip(1) {
        let Some(tag) = tag.split('>').next() else { continue };
        if let Some(i) = tag.find("key=\"") {
            if let Some(key) = tag[i + 5..].split('"').next().and_then(Combo::from_rc) {
                found.push(key);
            }
        }
    }
    found
}

pub fn kept(files: &[PathBuf]) -> Vec<Combo> {
    let mut all: Vec<Combo> = BUILT_IN.iter().filter_map(|c| Combo::from_rc(c)).collect();
    for f in files {
        all.extend(rc_combos(&std::fs::read_to_string(f).unwrap_or_default()));
    }
    all.extend(setting_keys());
    all
}

pub fn setting_keys() -> Vec<Combo> {
    SETTING_KEYS.iter().filter_map(|k| Combo::typed(&super::files::setting(k)).ok()).collect()
}

pub fn plugin_keys_rc() -> PathBuf {
    super::files::trindle_config().join("plugin-keys.xml")
}

pub fn plugin_keys_chosen() -> PathBuf {
    super::files::trindle_config().join("plugin-keys.ini")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_combinations_are_read_in_any_case_and_need_a_real_modifier() {
        let c = Combo::typed("ctrl + ALT + s").unwrap();
        assert_eq!((c.ctrl, c.alt, c.shift, c.logo, c.key.as_str()), (true, true, false, false, "s"));
        assert_eq!(Combo::typed("Super+Left").unwrap().key, "Left");
        assert_eq!(Combo::typed("Ctrl+Alt+Enter").unwrap().key, "Return");
        assert_eq!(Combo::typed("Ctrl+F12").unwrap().key, "F12");
        assert_eq!(Combo::typed("Ctrl+Alt+PgDn").unwrap().key, "Page_Down");
        assert_eq!(Combo::typed("Ctrl+-").unwrap().key, "minus");
        for bad in ["", "S", "Ctrl", "Ctrl+", "Shift+S", "Ctrl+Alt+Fish", "Ctrl+Alt+F25", "Ctrl+Alt+F0", "Ctrl+Alt+F01",
                    "Hyper+S", "Ctrl+S+A", "Ctrl+Alt+ss", "Ctrl+Alt+\u{1b}", "Ctrl+Alt+S\n"] {
            assert!(Combo::typed(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn the_file_and_the_typed_way_name_the_same_combination() {
        assert_eq!(Combo::from_rc("C-A-t"), Combo::typed("Ctrl+Alt+T").ok());
        assert_eq!(Combo::from_rc("W-space"), Combo::typed("Super+Space").ok());
        assert_eq!(Combo::from_rc("A-S-Tab"), Combo::typed("alt+shift+tab").ok());
        assert_ne!(Combo::from_rc("C-A-t"), Combo::typed("Ctrl+Alt+Shift+T").ok());
        assert_eq!(Combo::typed("Ctrl+Alt+S").unwrap().rc(), "A-C-s");
        assert_eq!(Combo::typed("Super+Shift+Page_Up").unwrap().shown(), "Shift+Super+Page_Up");
        assert_eq!(Combo::typed("ctrl+alt+s").unwrap().shown(), "Ctrl+Alt+S");
        assert_eq!(Combo::from_rc("Print").map(|c| c.rc()), Some("Print".into()));
        assert_eq!(Combo::from_rc(""), None);
        assert_eq!(Combo::from_rc("X-a"), None);
    }

    #[test]
    fn the_keybinds_in_a_file_are_found_and_comments_are_not() {
        let text = "<keyboard><default />\n<!-- <keybind key=\"C-q\"> -->\n\
                    <keybind key=\"C-A-t\"><action name=\"Execute\" command=\"x\" /></keybind>\n\
                    <keybind key=\"Super_L\" onRelease=\"yes\"><action name=\"Execute\" command=\"y\" /></keybind>\
                    <keybind><action/></keybind></keyboard>";
        let found = rc_combos(text);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.contains(&Combo::from_rc("C-A-t").unwrap()));
        assert!(!found.contains(&Combo::from_rc("C-q").unwrap()));
        let own = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/desktop/shortcuts.xml");
        let all = kept(&[own]);
        assert!(all.contains(&Combo::typed("Ctrl+Alt+T").unwrap()) && all.contains(&Combo::typed("Super+L").unwrap()));
        assert!(all.contains(&Combo::typed("Alt+Tab").unwrap()), "the standard ones are in too");
    }
}
