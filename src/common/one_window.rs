//! Checks that every program meant to have one window uses the one-window rule.

use super::no_writing_in_place::all_code;

const ONE: &[(&[&str], &str)] = &[
    (&["trindle-settings"], "settings/window.rs"),
    (&["trindle-apps", "trindle-store"], "store/launch.rs"),
    (&["trindle-activity"], "activity/mod.rs"),
    (&["trindle-drives", "trindle-disks"], "disks/mod.rs"),
    (&["trindle-text"], "text/mod.rs"),
    (&["trindle-welcome"], "welcome/mod.rs"),
    (&["trindle-report"], "report/window.rs"),
    (&["trindle-eye-dropper"], "screenshot/colours.rs"),
];

const EXEMPT: &[(&str, &str)] = &[
    ("trindle-ring", "the desktop shell; it owns its own bus name"),
    ("trindle-keyring", "a service; its prompts come one at a time"),
    ("trindle-mcp", "speaks on standard input and output"),
    ("trindle-boxed", "starts other programs; its windows are one question each"),
    ("trindle-plugin", "command line"),
    ("trindle-terminal", "starts the person's own terminal"),
    ("trindle-greeter", "the log in screen, started by greetd"),
    ("trindle-greeter-session", "the log in screen's session"),
    ("trindle-wallpaper", "a background, one per screen"),
    ("trindle-idle", "no window"),
    ("trindle-wsdd", "no window"),
    ("trindle-nightlight", "no window"),
    ("trindle-brightness", "no window"),
    ("trindle-icons", "command line"),
    ("trindle-scale", "no window"),
    ("trindle-session", "the session itself"),
    ("trindle-starting", "shown once while the session starts"),
    ("trindle-screensaver", "one for each screen, over everything"),
    ("trindle-autostart", "no window"),
    ("trindle-admin", "runs as root, no window"),
    ("trindle-dns", "no window"),
    ("trindle-thunder", "no window; its questions are asked one at a time"),
    ("trindle-controller", "runs as root, no window"),
    ("trindle-usb-retry", "no window"),
    ("trindle-voice-setup", "command line"),
    ("trindle-lock", "the lock screen, one for each screen"),
    ("trindle-clipboard", "no window"),
    ("trindle-screenshot", "a tool laid over the whole screen, started by its key"),
    ("trindle-portal", "no window"),
    ("trindle-installer", "runs from the live image, once"),
    ("trindle-live-welcome", "runs from the live image, once"),
    ("42_trindle_snapshots", "run by update-grub"),
    ("trindle-snapshots-grub", "run by update-grub"),
    ("41_trindle_windows", "run by update-grub"),
    ("trindle-windows-grub", "run by update-grub"),
];

fn parts() -> Vec<String> {
    let text = crate::common::source::code(include_str!("../main.rs"));
    let table = text.split("fn run_part(").nth(1).and_then(|t| t.split("_ =>").next()).unwrap_or("");
    let mut out = Vec::new();
    for line in table.lines() {
        let Some((names, _)) = line.split_once("=>") else { continue };
        out.extend(names.split('"').skip(1).step_by(2).map(String::from));
    }
    out
}

#[test]
fn every_program_has_one_window_or_says_why_not() {
    let all = parts();
    assert!(all.len() > 30, "main.rs's table of parts was not read: {all:?}");
    for part in &all {
        let one = ONE.iter().any(|(names, _)| names.contains(&part.as_str()));
        let exempt = EXEMPT.iter().any(|(name, _)| name == part);
        assert!(one || exempt, "{part} is in neither ONE nor EXEMPT (common/one_window.rs): give it the one-window rule or say why it has none");
        assert!(!(one && exempt), "{part} is both ONE and EXEMPT");
    }
    for (names, _) in ONE {
        for name in *names {
            assert!(all.iter().any(|p| p == name), "{name} is in ONE but main.rs does not start it");
        }
    }
    for (name, why) in EXEMPT {
        assert!(!why.is_empty() && all.iter().any(|p| p == name), "{name} is exempt but main.rs does not start it");
    }
}

fn live(text: &str) -> String {

    let tests = text.match_indices("#[cfg(test)]").map(|(at, m)| at + m.len()).find(|&at| {
        let rest = text[at..].trim_start();
        rest.strip_prefix("mod ").is_some_and(|r| r.trim_start_matches(|c: char| c.is_alphanumeric() || c == '_').trim_start().starts_with('{'))
    });
    let code = &text[..tests.unwrap_or(text.len())];
    code.lines()
        .map(|l| l.match_indices("//").map(|(at, _)| at).find(|at| *at == 0 || l[..*at].ends_with([' ', '\t'])).map_or(l, |at| &l[..at]))
        .flat_map(|l| l.split_whitespace())
        .collect()
}

fn rule_faults(text: &str) -> Vec<&'static str> {
    let flat = live(text);
    let mut bad = Vec::new();
    if !flat.contains("once::claim(") {
        bad.push("the second copy must raise the first with once::claim");
    }
    if !flat.contains("once::serve_window(") {
        bad.push("the first copy must answer with once::serve_window");
    }

    let name = flat.split("once::claim(").nth(1).and_then(|r| r.split(',').nth(2)).unwrap_or("");
    if !name.is_empty() && (flat.contains(&format!(".own({name},")) || flat.contains(&format!(".own_firmly({name},")) || flat.contains(&format!(".take_firmly({name},"))) {
        bad.push("it claims its name in the old private way");
    }
    bad
}

fn raises_by_hiding(text: &str) -> bool {
    let flat = live(text);
    flat.match_indices("set_visible(false);").any(|(at, m)| flat[at + m.len()..].chars().take(60).collect::<String>().contains("set_visible(true)"))
}

#[test]
fn each_program_with_one_window_uses_the_shared_rule() {
    let code = all_code();
    let find = |file: &str| code.iter().find(|(name, _)| name == file).map(|(_, text)| text.as_str()).unwrap_or_else(|| panic!("{file} was not found"));
    for (names, file) in ONE {
        assert_eq!(rule_faults(find(file)), Vec::<&str>::new(), "{}", names[0]);
    }
}

#[test]
fn the_one_window_checks_read_code_and_not_comments_and_catch_other_spellings() {
    let good = "fn main() { crate::toolkit::once::claim(&bus, &events, NAME, PATH, a, b); once::serve_window(&bus, PATH, &w); }";
    assert!(rule_faults(good).is_empty());

    let comment = "// once::claim( and once::serve_window(\nfn main() {}";
    assert_eq!(rule_faults(comment).len(), 2);
    let tests_only = "fn main() {}\n#[cfg(test)]\nmod t { fn f() { once::claim(&b, &e, N, P, a, b); once::serve_window(); } }";
    assert_eq!(rule_faults(tests_only).len(), 2);

    for other in ["bus.own(NAME, false, &e, f);", "bus.own( NAME , true, &e, f);", "bus\n    .own(NAME,\n false, &e, f);", "bus.own_firmly(NAME, &e, f);"] {
        let text = format!("{good}\nfn x() {{ {other} }}");
        assert!(rule_faults(&text).contains(&"it claims its name in the old private way"), "{other}");
    }

    assert!(rule_faults(&format!("{good}\nfn x() {{ bus.own(MANAGER, false, e, f); }}")).is_empty());
    for hide in ["w.set_visible(false); w.set_visible(true);", "shown.window.set_visible(false);\n    shown.window.set_visible(true);", "set_visible(false); set_visible(true);"] {
        assert!(raises_by_hiding(hide), "{hide}");
    }
    assert!(!raises_by_hiding("w.set_visible(false);\n// w.set_visible(true);"));
    assert!(!raises_by_hiding("w.set_visible(true);"));
}

#[test]
fn only_the_programs_listed_use_the_rule_and_no_window_is_raised_by_hiding_it() {
    let listed: Vec<&str> = ONE.iter().map(|(_, f)| *f).collect();
    for (file, text) in all_code() {
        if live(&text).contains("once::claim(") {
            assert!(listed.contains(&file.as_str()), "{file} uses once::claim but is not in ONE (common/one_window.rs)");
        }

        if file == "toolkit/window/handle.rs" {
            continue;
        }
        assert!(!raises_by_hiding(&text), "{file} raises a window by hiding and showing it; use Window::raise");
    }
}
