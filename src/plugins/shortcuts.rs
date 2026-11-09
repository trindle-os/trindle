//! A plugin's keyboard shortcuts.
//! plugin.ini names them, `[Plugin] Shortcuts=switch;other`, each with a
//! part `[Shortcut switch]` of `Keys=Ctrl+Alt+S` and `Run=bin/one switch`.
//! The compositor knows nothing of plugins: Trindle writes the installed
//! plugins' shortcuts, with what a person has typed over them, into an
//! ordinary keybinds file the compositor reads beside its own, and the
//! key runs `trindle-plugin run ID shortcut ID`, in the plugin's box.

use std::path::Path;

use crate::common::combos::{self, Combo};
use crate::common::files::{self, write_text};
use crate::common::keyfile::KeyFile;
use crate::toolkit::t;

use super::fields::is_plain;
use super::manifest::{command, good_name, list, MANIFEST, Plugin, text};
use super::menu::MENU_LABEL_MAX;

pub const MAX: usize = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct Shortcut {
    pub id: String,

    pub label: String,

    pub keys: Combo,

    pub run: Vec<String>,
}

pub fn kept_by_trindle() -> Vec<Combo> {
    combos::kept(&[files::system_desktop("shortcuts.xml")])
}

fn kept_by_person() -> Vec<Combo> {
    combos::rc_combos(&files::read_text(files::trindle_config().join("shortcuts.xml")))
}

pub(super) fn parse(at: &Path, kf: &KeyFile, kept: &dyn Fn() -> Vec<Combo>, problems: &mut Vec<String>) -> Vec<Shortcut> {
    let ids = list(kf, "Plugin", "Shortcuts");
    if ids.len() > MAX {
        problems.push(t!("plugins-shortcut-too-many", n = ids.len(), max = MAX));
    }
    let kept = if ids.is_empty() { Vec::new() } else { kept() };
    let mut all: Vec<Shortcut> = Vec::new();
    for id in ids.into_iter().take(MAX) {
        let group = format!("Shortcut {id}");
        if !good_name(&id) {
            problems.push(t!("plugins-shortcut-bad-id", id = &id));
            continue;
        }
        if all.iter().any(|s| s.id == id) {
            problems.push(t!("plugins-shortcut-twice", id = &id));
            continue;
        }
        if !kf.has_group(&group) {
            problems.push(t!("plugins-shortcut-no-part", id = &id, group = &group));
            continue;
        }
        let label = text(kf, &group, "Label");
        if label.chars().count() > MENU_LABEL_MAX || !is_plain(&label, MENU_LABEL_MAX) {
            problems.push(t!("plugins-label-short-line", group = &group, max = MENU_LABEL_MAX));
        }
        let keys = match Combo::typed(&text(kf, &group, "Keys")) {
            Ok(keys) if kept.contains(&keys) => {
                problems.push(t!("plugins-shortcut-keys-trindle", group = &group, keys = keys.shown()));
                None
            }
            Ok(keys) if all.iter().any(|s| s.keys == keys) => {
                problems.push(t!("plugins-shortcut-keys-twice", group = &group, keys = keys.shown()));
                None
            }
            Ok(keys) => Some(keys),
            Err(why) => {
                problems.push(t!("plugins-shortcut-keys-needed", group = &group, why = &why));
                None
            }
        };
        let run = text(kf, &group, "Run");
        let argv = command(at, &group, &run, problems);
        if argv.is_none() && run.is_empty() {
            problems.push(t!("plugins-shortcut-needs-run", group = &group));
        }
        if let (Some(keys), Some(run)) = (keys, argv) {
            let label = if label.is_empty() { id.clone() } else { label };
            all.push(Shortcut { id, label, keys, run });
        }
    }
    all
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub plugin: String,
    pub plugin_name: String,
    pub id: String,
    pub label: String,

    pub default: Combo,

    pub typed: Option<String>,

    pub keys: Option<Combo>,

    pub refused: Option<String>,
}

impl Binding {

    pub fn shown(&self) -> String {
        match &self.typed {
            Some(t) => t.clone(),
            None => self.default.shown(),
        }
    }
}

fn holder(b: &Binding) -> String {
    format!("{} ({})", b.plugin_name, b.label)
}

pub fn bindings_of(plugins: &[Plugin], chosen: &KeyFile, trindle: &[Combo], person: &[Combo]) -> Vec<Binding> {
    let mut sorted: Vec<&Plugin> = plugins.iter().filter(|p| !p.shortcuts.is_empty()).collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    let mut all: Vec<Binding> = Vec::new();
    for p in sorted {
        for s in &p.shortcuts {
            let typed = chosen.string(&p.id, &s.id).map(|t| t.trim().to_string());
            let mut b = Binding {
                plugin: p.id.clone(), plugin_name: p.name.clone(), id: s.id.clone(), label: s.label.clone(),
                default: s.keys.clone(), typed: typed.clone(), keys: None, refused: None,
            };
            let want = match &typed {
                None => Ok(Some(s.keys.clone())),
                Some(t) if t.is_empty() => Ok(None),
                Some(t) => Combo::typed(t).map(Some),
            };
            match want {
                Ok(None) => {}
                Err(why) => b.refused = Some(t!("plugins-shortcut-typed-bad", why = &why)),
                Ok(Some(keys)) => {
                    if trindle.contains(&keys) {
                        b.refused = Some(t!("plugins-shortcut-trindle", keys = keys.shown()));
                    } else if person.contains(&keys) {
                        b.refused = Some(t!("plugins-shortcut-yours", keys = keys.shown()));
                    } else if let Some(other) = all.iter().find(|o| o.keys.as_ref() == Some(&keys)) {
                        b.refused = Some(t!("plugins-shortcut-used", keys = keys.shown(), holder = holder(other)));
                    } else {
                        b.keys = Some(keys);
                    }
                }
            }
            all.push(b);
        }
    }
    all
}

pub fn bindings() -> Vec<Binding> {
    bindings_of(&super::installed(), &KeyFile::load_or_empty(&combos::plugin_keys_chosen()),
                &kept_by_trindle(), &kept_by_person())
}

pub fn rc_text(all: &[Binding]) -> String {
    let mut out = String::from("<?xml version=\"1.0\"?>\n<!-- Written by Trindle from the installed plugins and what \
                                you typed in Settings, Keyboard and mouse. Not for editing. -->\n\
                                <trindle_desktop>\n  <keyboard>\n");
    for b in all {
        if let Some(keys) = &b.keys {

            out.push_str(&format!("    <keybind key=\"{}\"><action name=\"Execute\" command=\"trindle-plugin run {} shortcut {}\" /></keybind>\n",
                                  keys.rc(), b.plugin, b.id));
        }
    }
    out.push_str("  </keyboard>\n</trindle_desktop>\n");
    out
}

pub fn refresh() -> std::io::Result<()> {
    let text = rc_text(&bindings());
    let path = combos::plugin_keys_rc();
    if files::read_text(&path) == text {
        return Ok(());
    }
    write_text(&path, &text)
}

pub fn check_typed(plugin: &str, shortcut: &str, typed: &str, all: &[Binding], trindle: &[Combo], person: &[Combo])
    -> Result<Option<Combo>, String> {
    let typed = typed.trim();
    if typed.is_empty() {
        return Ok(None);
    }
    let keys = Combo::typed(typed)?;
    if trindle.contains(&keys) {
        return Err(t!("plugins-shortcut-trindle", keys = keys.shown()));
    }
    if person.contains(&keys) {
        return Err(t!("plugins-shortcut-yours", keys = keys.shown()));
    }
    if let Some(other) = all.iter().find(|b| !(b.plugin == plugin && b.id == shortcut) && b.keys.as_ref() == Some(&keys)) {
        return Err(t!("plugins-shortcut-used", keys = keys.shown(), holder = holder(other)));
    }
    Ok(Some(keys))
}

pub fn choose(plugin: &str, shortcut: &str, typed: &str) -> Result<(), String> {
    let all = bindings();
    if !all.iter().any(|b| b.plugin == plugin && b.id == shortcut) {
        return Err(t!("plugins-shortcut-unknown").into());
    }
    let keys = check_typed(plugin, shortcut, typed, &all, &kept_by_trindle(), &kept_by_person())?;
    keep_choice(&combos::plugin_keys_chosen(), plugin, shortcut, &keys.map(|k| k.shown()).unwrap_or_default())?;
    refresh().map_err(|e| t!("plugins-shortcut-not-in-use", why = e.to_string()))
}

pub(crate) fn keep_choice(path: &std::path::Path, plugin: &str, shortcut: &str, shown: &str) -> Result<(), String> {
    let mut kf = KeyFile::load_kept(path).map_err(|e| t!("plugins-shortcut-not-saved-unchanged", why = e.to_string()))?;
    kf.set(plugin, shortcut, shown);
    write_text(path, &kf.text()).map_err(|e| t!("plugins-shortcut-not-saved", why = e.to_string()))
}

pub fn run_of(at: &Path, id: &str) -> Option<Vec<String>> {
    match KeyFile::load(&at.join(MANIFEST)) {
        Ok(kf) => parse(at, &kf, &|| Vec::new(), &mut Vec::new()).into_iter().find(|s| s.id == id).map(|s| s.run),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;
    use super::*;
    use std::path::PathBuf;

    fn folder_with(name: &str, ini: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-keys-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("bin")).unwrap();
        std::fs::write(dir.join("plugin.ini"), ini).unwrap();
        std::fs::write(dir.join("bin/one"), "#!/bin/sh\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.join("bin/one"), std::fs::Permissions::from_mode(0o755)).unwrap();
        dir
    }

    fn ini(extra: &str) -> String {
        format!("[Plugin]\nInterface=1\nId=org.example.One\nName=One\nVersion=1.0\n{extra}")
    }

    const ONE: &str = "Shortcuts=switch\n\n[Shortcut switch]\nLabel=Switch computer\nKeys=Ctrl+Alt+S\nRun=bin/one switch\n";

    fn conforming(name: &str, extra: &str) -> Result<Plugin, Vec<String>> {
        let dir = folder_with(name, &ini(extra));
        let r = super::super::manifest::check(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        r
    }

    fn says(extra: &str) -> String {
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        conforming(&format!("says{n}"), extra).unwrap_err().join("\n")
    }

    #[test]
    fn a_plugin_may_have_shortcuts_and_nothing_else() {
        let p = conforming("one", ONE).unwrap();
        assert_eq!(p.shortcuts.len(), 1);
        let s = &p.shortcuts[0];
        assert_eq!((s.id.as_str(), s.label.as_str(), s.keys.shown().as_str()), ("switch", "Switch computer", "Ctrl+Alt+S"));
        assert_eq!(s.run, ["bin/one", "switch"]);

        let p = conforming("nolabel", &ONE.replace("Label=Switch computer\n", "")).unwrap();
        assert_eq!(p.shortcuts[0].label, "switch");
    }

    #[test]
    fn a_plugin_s_shortcut_the_person_retyped_is_on_the_shortcut_list_as_typed() {
        use crate::common::key_list::{rows_of, Group, Sources};
        let p = conforming("listed", ONE).unwrap();
        let chosen = KeyFile::parse("[org.example.One]\nswitch=Ctrl+Alt+Y\n");
        let plugins: Vec<(String, Combo)> = bindings_of(&[p], &chosen, &[], &[]).into_iter().filter_map(|b| Some((format!("{}: {}", b.plugin_name, b.label), b.keys?))).collect();
        let rows = rows_of(&Sources { system_rc: "", user_rc: "", plugins: &plugins, setting: &|_| String::new() });
        assert!(rows.iter().any(|r| r.group == Group::Plugin && r.words == "One: Switch computer" && r.keys == "Ctrl+Alt+Y"), "{rows:?}");
        assert!(!rows.iter().any(|r| r.keys == "Ctrl+Alt+S"));
    }

    #[test]
    fn the_validator_refuses_what_is_wrong_with_shortcuts() {

        let five = "Shortcuts=a;b;c;d;e\n\n".to_string() + &["a", "b", "c", "d", "e"].iter().enumerate()
            .fold(String::new(), |mut s, (i, n)| { let _ = write!(s, "[Shortcut {n}]\nKeys=Ctrl+Alt+F{}\nRun=bin/one {n}\n", i + 5); s });
        assert!(says(&five).contains("at most 4"), "{}", says(&five));
        let bad = [
            ("Shortcuts=a b\n\n[Shortcut a b]\nKeys=Ctrl+Alt+S\nRun=bin/one\n", "Shortcut id a b"),
            ("Shortcuts=a;a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=bin/one\n", "names a twice"),
            ("Shortcuts=a\n", "no [Shortcut a] part"),
            ("Shortcuts=a\n\n[Shortcut a]\nRun=bin/one\n", "needs Keys"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=S\nRun=bin/one\n", "needs Ctrl, Alt or Super"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Shift+S\nRun=bin/one\n", "needs Ctrl, Alt or Super"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+Fish\nRun=bin/one\n", "not a key"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\n", "needs Run"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=/usr/bin/sh\n", "inside the plugin's folder"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=../x\n", "inside the plugin's folder"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=bin/nothing\n", "inside the plugin's folder"),
            ("Shortcuts=a\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=bin/one\nLabel=0123456789012345678901234567890123\n", "at most 32"),
            ("Shortcuts=a;b\n\n[Shortcut a]\nKeys=Ctrl+Alt+S\nRun=bin/one\n[Shortcut b]\nKeys=ctrl+alt+s\nRun=bin/one\n", "another shortcut of the plugin's"),
        ];
        for (extra, said) in bad {
            let got = says(extra);
            assert!(got.contains(said), "{said:?} not in: {got}");
        }
    }

    #[test]
    fn a_combination_trindle_keeps_is_refused() {

        for keys in ["Ctrl+Alt+T", "Super+L", "Alt+Tab", "Super+Left", "Super+Space"] {
            let got = says(&ONE.replace("Ctrl+Alt+S", keys));
            assert!(got.contains("already a shortcut of Trindle's"), "{keys}: {got}");
        }
    }

    fn plugin(id: &str, name: &str, shortcuts: &[(&str, &str)]) -> Plugin {
        Plugin {
            id: id.into(), name: name.into(), version: "1".into(), summary: String::new(), asks: Vec::new(), ring: None,
            fields: Vec::new(), background: None, listen: Vec::new(),
            shortcuts: shortcuts.iter().map(|(i, k)| Shortcut {
                id: (*i).into(), label: format!("{i} it"), keys: Combo::typed(k).unwrap(), run: vec!["bin/x".into()],
            }).collect(),
        }
    }

    fn kept() -> Vec<Combo> {
        combos::kept(&[PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/desktop/shortcuts.xml")])
    }

    #[test]
    fn two_plugins_cannot_have_the_same_keys_the_first_by_id_keeps_them() {
        let a = plugin("org.a.Alpha", "Alpha", &[("go", "Ctrl+Alt+G")]);
        let b = plugin("org.b.Beta", "Beta", &[("go", "ctrl+alt+g"), ("stop", "Ctrl+Alt+H")]);
        let all = bindings_of(&[b.clone(), a.clone()], &KeyFile::default(), &kept(), &[]);
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].plugin, "org.a.Alpha");
        assert!(all[0].keys.is_some() && all[0].refused.is_none());
        assert!(all[1].keys.is_none());
        assert_eq!(all[1].refused.as_deref(), Some("Ctrl+Alt+G is already used by Alpha (go it)."));
        assert!(all[2].keys.is_some(), "the other shortcut of the one refused is its own");

        let rc = rc_text(&all);
        assert_eq!(rc.matches("<keybind").count(), 2);
        assert!(rc.contains("command=\"trindle-plugin run org.a.Alpha shortcut go\"") && rc.contains("key=\"A-C-h\""), "{rc}");
        assert!(!rc.contains("org.b.Beta shortcut go"));
    }

    #[test]
    fn what_a_person_types_wins_and_clearing_turns_it_off() {
        let a = plugin("org.a.Alpha", "Alpha", &[("go", "Ctrl+Alt+G"), ("stop", "Ctrl+Alt+H")]);
        let chosen = KeyFile::parse("[org.a.Alpha]\ngo=Ctrl+Alt+J\nstop=\n");
        let all = bindings_of(std::slice::from_ref(&a), &chosen, &kept(), &[]);
        assert_eq!(all[0].keys.as_ref().map(Combo::shown).as_deref(), Some("Ctrl+Alt+J"));
        assert_eq!(all[0].shown(), "Ctrl+Alt+J");
        assert!(all[1].keys.is_none() && all[1].refused.is_none(), "off, not refused");
        assert_eq!(all[1].shown(), "");
        assert_eq!(rc_text(&all).matches("<keybind").count(), 1);

        let weird = KeyFile::parse("[org.a.Alpha]\ngo=rm -rf\n");
        let all = bindings_of(std::slice::from_ref(&a), &weird, &kept(), &[]);
        assert!(all[0].keys.is_none() && all[0].refused.as_deref().is_some_and(|r| r.contains("not a shortcut")));

        let kf = KeyFile::parse("[org.a.Alpha]\ngo=Ctrl+Alt+H\n");
        let all = bindings_of(&[a], &kf, &kept(), &[]);
        assert!(all[0].keys.is_some() && all[1].keys.is_none(), "stop keeps nothing: go took Ctrl+Alt+H first");
    }

    #[test]
    fn typing_is_checked_against_trindle_the_person_and_the_other_plugins() {
        let a = plugin("org.a.Alpha", "Alpha", &[("go", "Ctrl+Alt+G")]);
        let b = plugin("org.b.Beta", "Beta", &[("go", "Ctrl+Alt+H")]);
        let all = bindings_of(&[a, b], &KeyFile::default(), &kept(), &[]);
        let person = [Combo::typed("Ctrl+Alt+Y").unwrap()];
        let ask = |text: &str| check_typed("org.b.Beta", "go", text, &all, &kept(), &person);
        assert_eq!(ask("").unwrap(), None);
        assert_eq!(ask("  ").unwrap(), None);
        assert_eq!(ask("Ctrl+Alt+K").unwrap().map(|c| c.shown()).as_deref(), Some("Ctrl+Alt+K"));

        assert!(ask("ctrl+alt+h").unwrap().is_some());
        assert!(ask("Ctrl+Alt+G").unwrap_err().contains("already used by Alpha"));
        assert!(ask("Ctrl+Alt+T").unwrap_err().contains("Trindle's"));
        assert!(ask("Ctrl+Alt+Y").unwrap_err().contains("your own"));
        for nonsense in ["S", "Shift+K", "Ctrl+Alt+Nope", "Ctrl+Alt+K; rm -rf ~", "Ctrl+Alt+K\nCtrl+Alt+L", "$(touch x)"] {
            assert!(ask(nonsense).is_err(), "{nonsense:?}");
        }
    }

    #[test]
    fn a_shortcut_runs_the_programs_the_plugin_names() {
        let dir = folder_with("run", &ini(ONE));
        assert_eq!(run_of(&dir, "switch"), Some(vec!["bin/one".into(), "switch".into()]));
        assert_eq!(run_of(&dir, "other"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
