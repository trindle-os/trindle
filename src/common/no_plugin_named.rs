//! The desktop never adapts to one plugin.

use std::path::Path;

const OWN: [&str; 27] = [
    "activity", "apps", "boxed", "disks", "drives", "eyedropperlist", "filechooser", "files", "folders", "greeter", "help",
    "installer", "portal", "report", "ring", "screenshot", "settings", "store", "text", "welcome",
    "test",
    "onewindow", "oncetest", "oncehung", "oncerefuse",
    "toolkit",
    "screenreader",
];

fn start() -> String {
    ["org.", "trindle", "."].concat()
}

fn names(text: &str, start: &str) -> Vec<String> {
    text.match_indices(start)
        .map(|(at, _)| text[at + start.len()..].chars().take_while(char::is_ascii_alphanumeric).collect::<String>().to_ascii_lowercase())
        .filter(|name| !name.is_empty())
        .collect()
}

fn scan(dir: &Path, start: &str, found: &mut Vec<String>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(&path, start, found);
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for name in names(&text, start) {
            if !OWN.contains(&name.as_str()) {
                found.push(format!("{start}{name} in {}", path.display()));
            }
        }
    }
}

#[test]
fn a_name_is_read_whatever_its_case_and_whatever_follows() {
    let start = start();
    let text = format!("a \"{start}Ring\" b {start}weather.Extra, {start}");
    assert_eq!(names(&text, &start), ["ring", "weather"]);
}

#[test]
fn no_plugin_is_named_in_the_source() {
    let mut found = Vec::new();
    for (src, _) in crate::common::source::trees() {
        scan(&src, &start(), &mut found);
    }
    let flow = format!("{}flow in {}", start(), crate::common::source::path_of("common/own_hands.rs").display());
    assert_eq!(found.iter().filter(|f| **f == flow).count(), 1, "Flow is not named once, in common/own_hands.rs");
    found.retain(|f| *f != flow);
    assert!(found.is_empty(), "the desktop names a plugin, or a part of Trindle not yet in OWN: {found:?}");
}
