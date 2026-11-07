//! Every panel that takes the keyboard is under the one rule.

use super::no_writing_in_place::all_code;

const RING: &str = "the ring's program";
const OWN: &str = "a program of its own";

const PANELS: [(&str, usize, &str, &str); 12] = [
    ("polkit/mod.rs", 1, RING, "the question for the administrator's password, which holds the whole keyboard"),
    ("ring/view/panel.rs", 1, RING, "the ring's own panel, which holds the whole keyboard"),
    ("ring/notes/keyboard.rs", 2, RING, "a notification circle, which holds the whole keyboard only while it was reached from the keyboard (Super and N), and lets go when Escape is pressed or it goes"),
    ("ring/notes/moving.rs", 2, RING, "a notification circle, which holds the whole keyboard only while a pinned one is being moved, and lets go when it is put down or it goes"),
    ("thunder/ask.rs", 1, RING, "the question about a Thunderbolt or USB4 device, which takes the keyboard when clicked"),
    ("screenshot/mod.rs", 1, OWN, "the still picture a part is dragged out of"),
    ("screenshot/eye_dropper.rs", 1, OWN, "the colour picker"),
    ("screenshot/pin.rs", 1, OWN, "a pinned picture"),
    ("session/screensaver.rs", 1, OWN, "the screensaver"),
    ("store/fence_window.rs", 1, OWN, "the request an AI program makes to share a folder or run a command as administrator, which holds the whole keyboard from the moment it opens"),
    ("session/restart_ask.rs", 1, OWN, "the question before everything is restarted, which holds the whole keyboard from the moment it opens"),
    ("store/asking.rs", 1, OWN, "the permission window for an app or a plugin, which holds the whole keyboard from the moment it opens"),
];

fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.trim_start().starts_with("//")).collect()
}

fn takes(text: &str) -> usize {
    let ways = [["Keys::", "Always"].concat(), ["Keys::", "OnClick"].concat(), ["set_", "keys("].concat()];
    lines(text).iter().filter(|l| ways.iter().any(|w| l.contains(w.as_str()))).count()
}

const TOOLKIT: &str = "toolkit/window";

fn in_toolkit_windows(name: &str) -> bool {
    name == "toolkit/window.rs" || name.starts_with("toolkit/window/")
}
const COMPOSITOR: &str = "compose/";

#[test]
fn a_line_that_takes_the_keyboard_is_counted_and_one_that_never_does_is_not() {
    let always = ["keys: Keys::", "Always,"].concat();
    let clicked = ["keys: Keys::", "OnClick,"].concat();
    let changed = ["window.set_", "keys(Keys::Never);"].concat();
    assert_eq!(takes(&format!("{always}\n{clicked}\n{changed}\nkeys: Keys::Never,\n// {always}\n")), 3);
    assert_eq!(takes("fn a() {}\nlet keys = Keys::Never;\n"), 0);
}

#[test]
fn every_panel_that_takes_the_keyboard_is_on_the_list_with_the_program_that_draws_it() {
    let mut wrong = Vec::new();
    let all = all_code();
    for (name, text) in &all {
        if name.starts_with("toolkit/") || name.starts_with(COMPOSITOR) || name == "common/keys_panels.rs" {
            continue;
        }
        let (found, listed) = (takes(text), PANELS.iter().find(|p| p.0 == name).map_or(0, |p| p.1));
        if found != listed {
            wrong.push(format!("{name}: {found} lines have a panel take the keyboard, and the list says {listed}"));
        }
    }
    for (name, ..) in PANELS {
        if !all.iter().any(|(n, _)| n == name) {
            wrong.push(format!("{name} is on the list and is no file"));
        }
    }
    assert!(
        wrong.is_empty(),
        "a panel that takes the keyboard is not on the list in common/keys_panels.rs. Add it there with the program that draws it. \
         It needs nothing more for the pulse: the toolkit counts it, and the kept ring beats for it. Never have it ask for the pulse itself.\n{}",
        wrong.join("\n")
    );
}

#[test]
fn only_the_toolkit_makes_a_panel_or_says_whether_it_takes_the_keyboard() {
    let ways = [["get_layer_", "surface("].concat(), ["set_keyboard_", "interactivity("].concat()];
    let wallpaper = ("session/backdrop.rs", 1);
    for (name, text) in all_code() {
        if in_toolkit_windows(&name) || name.starts_with(COMPOSITOR) {
            continue;
        }
        let found = lines(&text).iter().filter(|l| ways.iter().any(|w| l.contains(w.as_str()))).count();
        let allowed = if name == wallpaper.0 { wallpaper.1 } else { 0 };
        assert_eq!(found, allowed, "{name} makes a panel, or says whether one takes the keyboard, by itself: the toolkit would not count it, and the ring would write no pulse for it");
        if name == wallpaper.0 {
            assert!(!text.contains(&ways[1]), "the wallpaper asks for the keyboard");
        }
    }
}

#[test]
fn the_toolkit_keeps_what_it_tells_the_compositor_and_says_as_a_panel_comes_out() {
    let all = all_code();
    let mut files: Vec<&(String, String)> = all.iter().filter(|(n, _)| in_toolkit_windows(n)).collect();
    assert!(!files.is_empty(), "the toolkit's windows ({TOOLKIT}) are gone");
    files.sort();
    let text: String = files.iter().map(|(_, t)| t.as_str()).collect::<Vec<_>>().join("\n");
    let code = lines(&text);
    let told = ["set_keyboard_", "interactivity("].concat();
    let at: Vec<usize> = (0..code.len()).filter(|&i| code[i].contains(&told)).collect();
    assert_eq!(at.len(), 2, "the toolkit tells the compositor whether a panel takes the keyboard in two places: as it opens, and as that is changed");
    for i in at {
        let kept = code[i + 1].trim();
        assert!(kept == "Some(layer)" || kept == "w.keys = keys;", "what the compositor was told is not kept for the count on the next line: {kept}");
    }
    assert!(code.iter().any(|l| l.trim() == "keys = p.keys;"), "what the compositor was told as a panel opens is not kept for the count");
    let said = code.iter().filter(|l| l.trim().ends_with(".keys_held();")).count();
    assert_eq!(said, 3, "the toolkit says a panel that takes the keyboard has come out as it is opened, as its keys are changed and as it is shown again");
    for inside in ["pub fn open(", "pub fn set_keys(", "pub fn set_visible("] {
        let from = code.iter().position(|l| l.contains(inside)).unwrap_or_else(|| panic!("{inside} is gone"));
        let to = code[from + 1..].iter().position(|l| l.trim_start().starts_with("pub fn ")).map_or(code.len(), |n| from + 1 + n);
        assert!(code[from..to].iter().any(|l| l.trim().ends_with(".keys_held();")), "{inside} no longer says a panel may have come to take the keyboard");
    }
}

#[test]
fn no_panel_asks_for_the_pulse_and_the_ring_asks_once_for_them_all() {
    let all = all_code();
    let asked = ["kept::", "pulse_while_keys_held("].concat();
    let beat = ["beat_", "while("].concat();
    let pulse = ["kept::", "pulse("].concat();
    let mut asks = Vec::new();
    for (name, text) in &all {
        if name.starts_with(COMPOSITOR) || name == "ring/kept.rs" {
            continue;
        }
        let code = lines(text);
        asks.extend(code.iter().filter(|l| l.contains(&asked)).map(|_| name.as_str()));
        assert!(!code.iter().any(|l| l.contains(&beat)), "{name} starts the ring's beat itself");
        let allowed = if name == "ring/view/panel.rs" { 1 } else { 0 };
        assert_eq!(code.iter().filter(|l| l.contains(&pulse)).count(), allowed, "{name} writes the ring's pulse itself");
    }
    assert_eq!(asks, ["ring/mod.rs", "screenshot/mod.rs"], "the ring asks for its pulse once, as it starts, for every panel it will ever have, and the screenshot tool once, as it starts");
    assert!(PANELS.iter().any(|p| p.2 == RING && p.0 == "polkit/mod.rs"));
}
