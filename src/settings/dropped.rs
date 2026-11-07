//! No Settings page drops an action's failure.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const THROWN_AWAY: &str = "result thrown away";
const NOT_LOOKED_AT: &str = "answer not looked at";
const STARTED_UNSEEN: &str = "started without looking";
const NO_BUS: &str = "silent without its bus";
const RAW_WORDS: &str = "the error's own wording";

const BEST_EFFORT: &[(&str, &str, &str, &str)] = &[
    ("steam.rs", "fresh_start", NOT_LOOKED_AT, "says its own trouble and success through Kit::done; the tuple it gets back is read there"),
    ("fonts_icons.rs", "load_families", NOT_LOOKED_AT, "reads the list of fonts; an empty list shows the font still in use"),
    ("bluetooth.rs", "refresh", NO_BUS, "a refresh of the list; the page already says the bus is missing"),
    ("datetime.rs", "list_zones", NO_BUS, "reads the list of time zones"),
    ("datetime.rs", "moved_zone", NO_BUS, "reads the zone back after it was changed"),
    ("refresh.rs", "refresh", NO_BUS, "a refresh of the page; the page already says the bus is missing"),
    ("refresh.rs", "got_nm", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "got_devices", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "got_device", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "got_wifi", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "got_saved", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "collect_aps", NO_BUS, "part of the same refresh"),
    ("refresh.rs", "scan", NOT_LOOKED_AT, "asks for a fresh look for networks as the page opens; the list in hand is shown either way"),
    ("power.rs", "shown", THROWN_AWAY, "listens for the battery changing; without it the battery is read as the page opens"),
    ("audio.rs", "test_sound", THROWN_AWAY, "makes the scratch folder; the test sound says so if it then cannot play"),
    ("audio.rs", "stop", THROWN_AWAY, "ends the microphone meter as the page closes"),
    ("bluetooth.rs", "hidden", NOT_LOOKED_AT, "hands the pairing agent back as the page closes"),
    ("bluetooth.rs", "ask_bluez", NOT_LOOKED_AT, "offers BlueZ this page's pairing agent; refused, pairing is asked in the ring instead"),
    ("bluetooth.rs", "ask_bluez", NO_BUS, "the same offer, made as the page opens"),
    ("users.rs", "pipe_face", THROWN_AWAY, "hands the picture to the helper; the helper's own answer says whether it arrived"),
    ("colours.rs", "set_look", STARTED_UNSEEN, "asks the ring to redraw in the new look"),
    ("reading.rs", "keep_labels", STARTED_UNSEEN, "asks the ring to redraw"),
    ("fonts_icons.rs", "icon_set", STARTED_UNSEEN, "asks the ring to redraw with the new icons"),
    ("ring.rs", "voice_change", STARTED_UNSEEN, "asks the ring to take up voice"),
    ("ring.rs", "save_and_restart", STARTED_UNSEEN, "asks the ring to take up the change"),
];

fn pages() -> Vec<PathBuf> {
    let settings = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/settings");
    let mut files = vec![settings.join("ui.rs"), settings.join("session_env.rs")];
    let mut folders = vec![settings.join("panels"), settings.join("ui")];
    while let Some(folder) = folders.pop() {
        for path in std::fs::read_dir(folder).unwrap().flatten().map(|e| e.path()) {
            if path.is_dir() {
                folders.push(path);
            } else if path.extension().is_some_and(|x| x == "rs") && !path.ends_with("tests.rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn defines(line: &str) -> Option<&str> {
    let line = line.trim_start();
    let rest = ["pub(crate) fn ", "pub(super) fn ", "pub fn ", "fn "].iter().find_map(|start| line.strip_prefix(start))?;
    let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
    Some(&rest[..end])
}

fn lost(line: &str) -> Vec<&'static str> {
    let code = line.trim();
    let mut found = Vec::new();
    if code.starts_with("//") {
        return found;
    }
    let underscore = ["let ", "_ ="].concat();
    if code.contains(&underscore) || code.ends_with(&[".ok", "();"].concat()) {
        found.push(THROWN_AWAY);
    }
    let ignored = [["|_|", " {}"].concat(), ["|_, _,", " _|"].concat(), ["|_, out,", " _|"].concat()];
    if ignored.iter().any(|i| code.contains(i.as_str())) {
        found.push(NOT_LOOKED_AT);
    }
    let start = ["spawn", "(&["].concat();
    if let Some(at) = code.find(&start) {
        let before = code[..at].trim_end_matches("crate::common::run::").trim_end_matches("run::").trim();
        if before.is_empty() && code.ends_with(';') {
            found.push(STARTED_UNSEEN);
        }
    }
    let bus_gone = [[".system.clone()", " else { return }"].concat(), [".session.clone()", " else { return }"].concat(),
                    [".system.as_ref()", " else { return }"].concat(), [".session.as_ref()", " else { return }"].concat()];
    if bus_gone.iter().any(|b| code.contains(b.as_str())) {
        found.push(NO_BUS);
    }
    let red = ["false,", " false)"].concat();
    let raw = [[".to_", "string()"].concat(), ["{", "e}"].concat(), ["{", "err}"].concat(), ["{", "e:"].concat()];
    let old_way = [["Could not", " save: {"].concat(), [".err().map(|e|", " e.to_string())"].concat()];
    if (code.contains(&red) && raw.iter().any(|r| code.contains(r.as_str()))) || old_way.iter().any(|o| code.contains(o.as_str())) {
        found.push(RAW_WORDS);
    }
    found
}

fn findings() -> BTreeSet<(String, String, &'static str)> {
    let mut all = BTreeSet::new();
    for file in pages() {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&file).unwrap();
        let end = text.find("\nmod tests {").unwrap_or(text.len());
        let mut function = String::new();
        for line in text[..end].lines() {
            if let Some(f) = defines(line) {
                function = f.to_string();
            }
            for way in lost(line) {
                all.insert((name.clone(), function.clone(), way));
            }
        }
    }
    all
}

#[test]
fn each_way_of_losing_a_failure_is_seen() {
    let line = |parts: &[&str]| parts.concat();
    assert_eq!(lost(&line(&["        let ", "_ = std::fs::remove_file(path);"])), [THROWN_AWAY]);
    assert_eq!(lost(&line(&["            .listen(x)", ".ok", "();"])), [THROWN_AWAY]);
    assert_eq!(lost(&line(&["bus.call(a, b, c).set(\"Discoverable\", v, false, &k.events, ", "|_|", " {});"])), [NOT_LOOKED_AT]);
    assert_eq!(lost(&line(&["capture(&[\"wpctl\", \"set-mute\", t, \"1\"], None, ", "|_, _,", " _| {});"])), [NOT_LOOKED_AT]);
    assert_eq!(lost(&line(&["    run::", "spawn", "(&[", "\"trindle-lock\"]);"])), [STARTED_UNSEEN]);
    assert_eq!(lost(&line(&["let Some(bus) = kit", ".system.clone()", " else { return };"])), [NO_BUS]);
    assert_eq!(lost(&line(&["k.say(SEARCH, &e", ".to_", "string(), ", "false,", " false);"])), [RAW_WORDS]);
    assert_eq!(lost(&line(&["Err(e) => k.say(key, &format!(\"", "Could not", " save: {", "e}\"), ", "false,", " false),"])), [RAW_WORDS]);
    assert!(lost(&line(&["let started = run::", "spawn", "(&[", "\"trindle-lock\"]);"])).is_empty());
    assert!(lost(&line(&["if ", "spawn", "(&[", "\"tailscale\", \"up\"]) {"])).is_empty());
    assert!(lost("let Some(bus) = kit.system_for(POWER) else { return };").is_empty());
    assert!(lost("kit.settle(key, set_setting(key, value), NOT_SAVED);").is_empty());
    assert!(lost(&line(&["// ", "let ", "_ = old();"])).is_empty());
    assert_eq!(defines("    pub(crate) fn made_up() -> Audio {"), Some("made_up"));
    assert_eq!(defines("fn joining(kit: &Kit) -> impl FnOnce() {"), Some("joining"));
    assert_eq!(defines("        let back = kit.then(|_| {});"), None);
}

#[test]
fn no_settings_page_drops_an_actions_failure_without_saying_it() {
    let found = findings();
    let allowed: BTreeSet<(String, String, &'static str)> =
        BEST_EFFORT.iter().map(|(file, function, way, _)| (file.to_string(), function.to_string(), *way)).collect();
    let dropped: Vec<_> = found.difference(&allowed).collect();
    assert!(dropped.is_empty(),
            "a Settings page loses a failure. Say it through Kit::settle, settles, started, system_for or session_for \
             (src/settings/ui.rs), or list it in BEST_EFFORT with its reason: {dropped:#?}");
    let stale: Vec<_> = allowed.difference(&found).collect();
    assert!(stale.is_empty(), "listed as best effort but no longer in the source: {stale:#?}");
    for (file, function, _, why) in BEST_EFFORT {
        assert!(why.len() > 20, "{file} {function} needs a real reason");
    }
}
