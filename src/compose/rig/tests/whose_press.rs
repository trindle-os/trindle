// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Whose presses count as the person's own, and who may unlock.
// The source guards read what comes before this attribute as the live code.

#[cfg(test)]
use crate::rig::kept_ring::{beat, KeptRing};
use crate::rig::*;

const ANYONE: [(&str, &str); 21] = [
    ("org.trindle.Activity", "an ordinary window; its one harmful press is a single act on the list (activity-stop)"),
    ("org.trindle.Settings", "an ordinary window; its ways out of the session and its Permissions switches are single acts on the list"),
    ("org.trindle.Store", "an ordinary window; Install and its restart after an update are single acts on the list, and whatever needs the administrator asks for the password, which is on the list"),
    ("org.trindle.Drives", "every change asks for the administrator's password, which is on the list"),
    ("org.trindle.Text", "an ordinary window"),
    ("org.trindle.EyeDropperList", "an ordinary window"),
    ("org.trindle.Welcome", "an ordinary window"),
    ("org.trindle.report", "an ordinary window: it only saves the person's own words and blanked log lines as a private file in Downloads, or copies them; it sends nothing"),
    ("org.trindle.Greeter", "the login screen, in a session of its own where no plugin runs"),
    ("org.trindle.Installer", "the installer, on the stick, where no plugin runs"),
    ("trindle-battery", "says the battery is low; asks nothing"),
    ("trindle-eye-dropper", "picks a colour; gives nothing away"),
    ("trindle-label", "a label beside a notification; takes no press"),
    ("trindle-magnify", "the magnifier; takes no press"),
    ("trindle-notice", "a notice; asks nothing"),
    ("trindle-notes", "notifications; their buttons are the sending program's own"),
    ("trindle-corner", "the corner that opens the ring"),
    ("trindle-screensaver", "any press ends it, and it gives nothing away"),
    ("trindle-screenshot", "takes a picture of the screen for the person; nothing leaves the computer"),
    ("trindle-screenshot-pin", "a pinned picture"),
    ("trindle-starting", "shown while the desktop starts; takes no press"),
];

fn names_given(src: &Path) -> std::collections::BTreeMap<String, String> {
    let mut all = Vec::new();
    rust_files_below(src, &mut all);
    let mut given = std::collections::BTreeMap::new();
    let skipped = ["compose/", "toolkit/", "common/person_only.rs", "common/own_tests.rs"];
    for file in all {
        let short = file.strip_prefix(src).unwrap().display().to_string();
        if skipped.iter().any(|s| short.starts_with(s)) {
            continue;
        }
        let text = crate::files::read_string(&file).unwrap();
        for (at, _) in text.match_indices("app_id: ") {
            let said = &text[at + "app_id: ".len()..];
            if !(said.starts_with('"') || said.starts_with("NAME") || said.starts_with("super::NAME") || said.starts_with("APP_ID") || said.starts_with("format!(")) {
                continue;
            }
            for name in names_said(said, &file, &text).into_iter().filter(|n| n.starts_with("org.trindle.") || n.starts_with("trindle-")) {
                given.insert(name, short.clone());
            }
        }
        for (at, _) in text.match_indices("screen: None") {
            let before = &text[at.saturating_sub(400)..at];
            let Some(name) = before.rfind("name: ") else { continue };
            let names = names_said(&before[name + "name: ".len()..], &file, &text);
            assert!(!names.is_empty(), "{short}: a panel is given a name this test cannot read: {}", &before[name..]);
            for name in names {
                given.insert(name, short.clone());
            }
        }
    }
    given
}

fn rust_files_below(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files_below(&path, out);
        } else if path.extension().is_some_and(|x| x == "rs") {
            out.push(path);
        }
    }
}

fn quoted(text: &str) -> Option<&str> {
    text.strip_prefix('"').and_then(|rest| rest.split('"').next())
}

fn constant(text: &str, name: &str) -> Option<String> {
    let wanted = format!("const {name}: &str = \"");
    text.find(&wanted).and_then(|at| text[at + wanted.len()..].split('"').next().map(str::to_string))
}

fn names_said(said: &str, file: &Path, text: &str) -> Vec<String> {
    let folders = |name: &str| file.parent().and_then(|d| crate::files::read_string(d.join("mod.rs")).ok()).and_then(|t| constant(&t, name));
    let above = said.starts_with("super::");
    let own = |name: &str| if above { folders(name) } else { constant(text, name).or_else(|| folders(name)) };
    let said = said.trim_start_matches("super::");
    if let Some(word) = quoted(said) {
        return vec![word.to_string()];
    }
    if let Some(rest) = said.strip_prefix("format!(") {
        return quoted(rest).map(str::to_string).into_iter().collect();
    }
    for name in ["NAME", "APP_ID"] {
        if said.starts_with(name) {
            return own(name).into_iter().collect();
        }
    }
    let word: String = said.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    let mut found = Vec::new();
    for (at, _) in text.match_indices(&format!("{word}: &str")) {
        let Some(start) = text[..at].rfind("fn ") else { continue };
        let function: String = text[start + 3..].chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        for (call, _) in text.match_indices(&format!("{function}(")) {
            let line = text[call..].lines().next().unwrap_or("");
            found.extend(line.split('"').skip(1).step_by(2).filter(|w| w.starts_with("trindle-")).map(str::to_string));
        }
    }
    found
}

#[test]
fn everything_only_the_person_may_press_is_on_the_one_list() {
    alone("rig::tests::whose_press::everything_only_the_person_may_press_is_on_the_one_list", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        questions_trindle_asks(&mut rig, &mut ring, &plugin);
        panels_started_by_name(&mut rig, &mut ring, &plugin);
        questions_in_windows(&mut rig, &plugin);
        let plain = rig.join(ClientState::default());
        rig.window_called(plain, "org.example.Notes");
        rig.onto_the_window_in_front();
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.pressed(plain), [KEY_A]);
        assert!(!rig.state.only_the_person_answers());
        questions_by_name_alone(&mut rig, &plugin);

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let listed = names_on_the_list_are_given();
        every_source_name_is_listed(&src, &listed);
    });
}

fn questions_trindle_asks(rig: &mut Rig, ring: &mut KeptRing, plugin: &Maker) {
    use crate::person_only as only;
    for name in only::ASKED_BY_TRINDLE {
        beat(&mut ring.pulse);
        let before = rig.pressed(ring.who).len();
        let question = rig.panel(ring.who, name);
        assert!(rig.state.only_the_person_answers(), "{name}: the mouse and keyboard would be lent while it is out");
        rig.made_key(plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.pressed(ring.who).len(), before, "{name}: a made-up key was typed into it");
        rig.move_by_hand(0.02, 0.98);
        let (key, click) = (rig.press(Some(KEY_A)), rig.press(None));
        assert!(rig.state.not_for_a_made_up_press(&key) && rig.state.not_for_a_made_up_press(&click), "{name}: a made-up press passed elsewhere while it was out");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who).len(), before + 1, "{name}: the person's own key did not reach it");
        rig.put_away(&question);
        assert!(!rig.state.only_the_person_answers(), "{name} was still asking once put away");
    }
}

fn panels_started_by_name(rig: &mut Rig, ring: &mut KeptRing, plugin: &Maker) {
    use crate::person_only as only;
    let none: [u32; 0] = [];
    for name in only::PANELS_BY_NAME {
        let by_name = rig.join(ClientState::default());
        let panel = rig.panel(by_name, name);
        rig.made_key(plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.pressed(by_name), none, "{name}: a made-up key reached a ring started by name");
        rig.move_by_hand(0.5, 0.5);
        let click = rig.press(None);
        assert!(rig.state.not_for_a_made_up_press(&click), "{name}: a made-up click reached a ring started by name");
        rig.numbered(by_name);
        rig.made_click(plugin);
        assert_eq!(rig.numbered(by_name).len(), 1, "{name}: only the letting go of a made-up click passes");
        rig.move_by_hand(0.02, 0.98);
        let click = rig.press(None);
        assert!(!rig.state.not_for_a_made_up_press(&click), "{name}: a ring started by name kept a made-up click from the rest of the screen");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(by_name), [KEY_A], "{name}: the person's own key did not reach the ring started by name");
        rig.put_away(&panel);
        beat(&mut ring.pulse);
        let before = rig.pressed(ring.who).len();
        let kept = rig.panel(ring.who, name);
        rig.made_key(plugin, KEY_A);
        assert_eq!(rig.pressed(ring.who).len(), before + 1, "{name}: a made-up key was kept from the kept ring");
        rig.put_away(&kept);
    }
}

fn questions_in_windows(rig: &mut Rig, plugin: &Maker) {
    use crate::person_only as only;
    let none: [u32; 0] = [];
    for name in only::WINDOWS {
        let boxed = rig.boxed();
        rig.window_called(boxed, name);
        rig.onto_the_window_in_front();
        assert!(rig.has_keyboard(boxed));
        rig.made_key(plugin, KEY_A);
        assert_eq!(rig.pressed(boxed), [KEY_A], "{name}: a program in a box had made-up keys kept from it by taking the name");
        assert!(!rig.state.only_the_person_answers(), "{name}: a program in a box held the lending back by taking the name");
        let asker = rig.join(ClientState::default());
        rig.window_called(asker, name);
        rig.onto_the_window_in_front();
        assert!(rig.has_keyboard(asker), "{name}: the question was not given the keyboard");
        rig.made_key(plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.pressed(asker), none, "{name}: a made-up key was typed into it");
        rig.numbered(asker);
        rig.made_click(plugin);
        rig.made_up_click();
        assert_eq!(rig.numbered(asker).len(), 2, "{name}: a made-up click was passed to it");
        assert!(rig.state.only_the_person_answers(), "{name}: the mouse and keyboard would be lent while it has the keyboard");
        rig.move_by_hand(0.02, 0.98);
        let (key, click) = (rig.press(Some(KEY_A)), rig.press(None));
        assert!(rig.state.not_for_a_made_up_press(&key), "{name}: a made-up key passed with the pointer elsewhere");
        assert!(!rig.state.not_for_a_made_up_press(&click), "{name}: a made-up click elsewhere was kept back");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(asker), [KEY_A], "{name}: the person's own key did not reach it");
        rig.numbered(asker);
        rig.onto_the_window_in_front();
        assert_eq!(rig.numbered(asker).len(), 2, "{name}: the person's own click did not reach it");
    }
}

fn questions_by_name_alone(rig: &mut Rig, plugin: &Maker) {
    use crate::person_only as only;
    let none: [u32; 0] = [];
    for name in only::ASKED_BY_NAME {
        let asker = rig.join(ClientState::default());
        rig.panel(asker, name);
        assert!(rig.state.only_the_person_answers(), "{name}: the mouse and keyboard would be lent while it is out");
        rig.made_key(plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.pressed(asker), none, "{name}: a made-up key was typed into it");
        rig.move_by_hand(0.02, 0.98);
        let (key, click) = (rig.press(Some(KEY_A)), rig.press(None));
        assert!(rig.state.not_for_a_made_up_press(&key) && rig.state.not_for_a_made_up_press(&click), "{name}: a made-up press passed elsewhere while it was out");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(asker), [KEY_A], "{name}: the person's own key did not reach it");
    }
}

fn names_on_the_list_are_given() -> std::collections::BTreeSet<&'static str> {
    use crate::person_only as only;
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let read = |file: &Path| crate::files::read_string(file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    let listed: std::collections::BTreeSet<&str> = only::ASKED_BY_TRINDLE.iter().chain(&only::ASKED_BY_NAME).chain(&only::PANELS_BY_NAME).chain(&only::WINDOWS).copied().collect();
    assert_eq!(listed, only::GIVEN_IN.iter().map(|g| g.0).collect::<std::collections::BTreeSet<_>>(), "every name on the list is held against the line that gives it");
    for (name, file, words) in only::GIVEN_IN {
        assert!(words.contains(name) && read(&src.join(file)).contains(words), "{file} no longer gives its window or panel the name {name} as {words}");
    }
    listed
}

fn every_source_name_is_listed(src: &Path, listed: &std::collections::BTreeSet<&str>) {
    let given = names_given(src);
    for must in ["org.trindle.Settings", "org.trindle.Activity", "trindle-ring", "trindle-notes", "trindle-keyring", "org.trindle.Portal"] {
        assert!(given.contains_key(must), "the reading of the source no longer finds {must}: {given:?}");
    }
    for (name, file) in &given {
        let anyone = ANYONE.iter().any(|a| a.0 == name);
        assert!(listed.contains(name.as_str()) != anyone, "{file} gives a window or panel the name {name}. Say which it is: a question or a press only the person may make goes on the list in common/person_only.rs, and anything else among those anyone may press, in this test, with why. Never both");
    }
    for (name, why) in ANYONE {
        assert!(given.contains_key(name), "{name} is among those anyone may press and no window or panel is given that name any more");
        assert!(!why.is_empty());
    }
}

#[test]
fn the_person_s_presses_are_named_to_any_window_outside_a_box_and_no_made_up_one() {
    alone("rig::tests::whose_press::the_person_s_presses_are_named_to_any_window_outside_a_box_and_no_made_up_one", |dir| {
        let mut rig = Rig::new(dir);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        let app = rig.join(ClientState::default());
        assert!(rig.offered(app), "a program outside every box was not offered the naming of the person's presses");
        rig.window_called(app, "org.trindle.Activity");
        rig.onto_the_window_in_front();
        assert_eq!(rig.named_first(app), [true, true], "the person's own click was not named before it arrived");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.named_first(app), [true, true], "the person's own key was not named before it arrived");
        rig.made_key(&plugin, KEY_A);
        rig.made_up_key(KEY_A);
        assert_eq!(rig.named_first(app), [false; 4], "a made-up key was named as the person's own");
        rig.made_click(&plugin);
        rig.made_up_click();
        assert_eq!(rig.named_first(app), [false; 4], "a made-up click was named as the person's own");
        let (x, y) = rig.middle;
        rig.made_up_move(x + 0.02, y + 0.02);
        rig.click_by_hand();
        assert_eq!(rig.named_first(app), [false, false], "a real click after a made-up move was named as the person's own");
        rig.move_by_hand(x, y);
        rig.click_by_hand();
        assert_eq!(rig.named_first(app), [true, true]);
        let on_screen = rig.join(ClientState::default());
        let typed = rig.pressed(app).len();
        rig.on_screen_key(on_screen, KEY_A);
        assert_eq!(rig.named_first(app), [false, false], "a made-up keyboard's key was named as the person's own, with the on-screen keyboard not switched on by the person");
        assert_eq!(rig.pressed(app).len(), typed + 1, "the key was not passed on");
        assert_eq!(rig.pressed(app).last(), Some(&KEY_A));
        crate::own_helpers::kept_at(&dir.join("home/.local/state/trindle/own-helpers"), || on_screen_keyboard_is_the_persons_only_while_they_switch_it_on(&mut rig, &plugin, app, on_screen));
        let other = rig.join(ClientState::default());
        rig.window_called(other, "org.example.Notes");
        rig.onto_the_window_in_front();
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.named_first(other), [true; 4]);
        assert_eq!(rig.named_first(app), [] as [bool; 0]);
        let boxed = rig.boxed();
        assert!(!rig.offered(boxed), "a program in a box was offered the naming of the person's presses");
        rig.window_called(boxed, "org.example.Boxed");
        rig.onto_the_window_in_front();
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.named_first(boxed), [false; 4]);
    });
}

fn on_screen_keyboard_is_the_persons_only_while_they_switch_it_on(rig: &mut Rig, plugin: &Maker, app: usize, on_screen: usize) {
    use crate::own_helpers::Helper;
    use crate::own_helpers_set::switch;
    switch(Helper::Keyboard, true, false).unwrap();
    switch(Helper::Reader, true, true).unwrap();
    rig.on_screen_key(on_screen, KEY_A);
    assert_eq!(rig.named_first(app), [false, false], "a made-up keyboard's key was named as the person's own, with the on-screen keyboard switched on by a made-up press");
    switch(Helper::Keyboard, true, true).unwrap();
    rig.on_screen_key(on_screen, KEY_A);
    assert_eq!(rig.named_first(app), [true, true], "an on-screen keyboard's key was not named as the person's own, with the keyboard switched on by the person");
    assert_eq!(rig.pressed(app).last(), Some(&KEY_A));
    rig.made_key(plugin, KEY_A);
    rig.made_click(plugin);
    assert_eq!(rig.named_first(app), [false; 4], "a made-up press after an on-screen key was named as the person's own");
    switch(Helper::Keyboard, false, false).unwrap();
    rig.on_screen_key(on_screen, KEY_A);
    assert_eq!(rig.named_first(app), [false, false], "a made-up keyboard's key was named as the person's own after the on-screen keyboard was switched off");
}

#[test]
fn only_flow_from_the_catalogue_presses_as_the_person() {
    alone("rig::tests::whose_press::only_flow_from_the_catalogue_presses_as_the_person", |dir| {
        use crate::own_hands::FLOW;
        let mut rig = Rig::new(dir);
        let app = rig.join(ClientState::default());
        rig.window_called(app, "org.trindle.Activity");
        rig.onto_the_window_in_front();
        rig.named_first(app);
        let (x, y) = rig.middle;
        let flow = rig.maker(Some(crate::sandbox::CONTROL), Some(FLOW));
        let plugins = dir.join("home/.local/share/trindle/plugins");
        let named_flow = "org.example.Flow";
        flow_is_the_person_only_with_the_mark(&mut rig, app, &flow, x, y);
        let other = pretenders_press_as_no_one(&mut rig, &plugins, named_flow, app, &flow, x, y);
        let chooser = the_questions_take_only_flows_presses(&mut rig, &flow, &other);
        let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
        let asked = rig.panel(root, "anything");
        rig.made_key(&flow, KEY_A);
        assert_eq!(rig.pressed(root), [] as [u32; 0], "Flow typed into root's own question");
        rig.put_away(&asked);
        a_changed_mark_is_no_mark(&mut rig, &flow, chooser, &plugins, named_flow);
    });
}

fn flow_is_the_person_only_with_the_mark(rig: &mut Rig, app: usize, flow: &Maker, x: f64, y: f64) {
    use crate::own_hands::FLOW;
    rig.allow(FLOW, &["control"]);
    rig.made_key(flow, KEY_A);
    assert_eq!(rig.named_first(app), [false, false], "Flow's id with no mark from the catalogue pressed as the person");
    rig.flow_from_the_catalogue();
    rig.a_moment();
    rig.made_key(flow, KEY_A);
    assert_eq!(rig.named_first(app), [true, true], "Flow's key was not named as the person's own");
    rig.made_move(flow, x + 0.01, y);
    rig.made_click(flow);
    assert_eq!(rig.named_first(app), [true, true], "Flow's click was not named as the person's own");
}

fn pretenders_press_as_no_one(rig: &mut Rig, plugins: &Path, named_flow: &str, app: usize, flow: &Maker, x: f64, y: f64) -> Maker {
    use crate::own_hands::{FLOW, MARK};
    rig.allow(named_flow, &["control"]);
    std::fs::copy(plugins.join(FLOW).join(MARK), plugins.join(named_flow).join(MARK)).unwrap();
    let other = rig.maker(Some(crate::sandbox::CONTROL), Some(named_flow));
    let plain = rig.maker(Some(crate::sandbox::BOXED), Some(FLOW));
    for (who, maker) in [("another id", &other), ("Flow's id on a plain box", &plain), ("a program outside every box", &Maker::default())] {
        rig.made_key(maker, KEY_A);
        rig.made_move(maker, x, y + 0.01);
        rig.made_click(maker);
        assert_eq!(rig.named_first(app), [false; 4], "{who} pressed as the person");
    }
    rig.made_move(&other, x, y);
    rig.made_click(flow);
    assert_eq!(rig.named_first(app), [false, false], "Flow's click where another plugin put the pointer was named as the person's");
    other
}

fn the_questions_take_only_flows_presses(rig: &mut Rig, flow: &Maker, other: &Maker) -> usize {
    let mut ring = rig.kept_ring();
    beat(&mut ring.pulse);
    let question = rig.panel(ring.who, crate::trusted::QUESTION);
    rig.made_key(other, KEY_A);
    assert_eq!(rig.pressed(ring.who), [] as [u32; 0], "a pretender typed into the administrator's question");
    rig.made_key(flow, KEY_A);
    assert_eq!(rig.pressed(ring.who), [KEY_A], "Flow could not type into the administrator's question");
    rig.made_move(flow, 0.5, 0.5);
    rig.numbered(ring.who);
    rig.made_click(flow);
    assert_eq!(rig.numbered(ring.who).len(), 2, "Flow's click did not reach the administrator's question");
    rig.put_away(&question);
    let chooser = rig.join(ClientState::default());
    rig.window_called(chooser, crate::person_only::CHOOSER);
    rig.onto_the_window_in_front();
    rig.numbered(chooser);
    rig.named_first(chooser);
    let (cx, cy) = rig.middle;
    rig.made_move(other, cx, cy);
    rig.made_click(other);
    assert_eq!(rig.numbered(chooser).len(), 1, "a pretender's click went down in the file chooser");
    rig.named_first(chooser);
    rig.made_move(flow, cx, cy);
    rig.made_click(flow);
    assert_eq!(rig.named_first(chooser), [true, true], "Flow's click in the file chooser was not the person's own");
    chooser
}

fn a_changed_mark_is_no_mark(rig: &mut Rig, flow: &Maker, chooser: usize, plugins: &Path, named_flow: &str) {
    use crate::own_hands::{FLOW, MARK};
    let mark = plugins.join(FLOW).join(MARK);
    std::fs::set_permissions(&mark, std::os::unix::fs::PermissionsExt::from_mode(0o666)).unwrap();
    rig.a_moment();
    rig.onto_the_window_in_front();
    rig.named_first(chooser);
    rig.made_key(flow, KEY_A);
    assert_eq!(rig.named_first(chooser), [false], "a mark anyone may write was taken for the catalogue's");
    std::fs::remove_file(&mark).unwrap();
    std::os::unix::fs::symlink(plugins.join(named_flow).join(MARK), &mark).unwrap();
    rig.a_moment();
    rig.made_key(flow, KEY_A);
    assert_eq!(rig.named_first(chooser), [false], "a link was taken for the catalogue's mark");
}

#[test]
fn a_clicking_helper_presses_as_the_person_only_once_they_switched_it_on() {
    alone("rig::tests::whose_press::a_clicking_helper_presses_as_the_person_only_once_they_switched_it_on", |dir| {
        use crate::own_helpers::{kept_at, Helper};
        use crate::own_helpers_set::switch;
        let mut rig = Rig::new(dir);
        let app = rig.join(ClientState::default());
        rig.window_called(app, "org.trindle.Activity");
        rig.onto_the_window_in_front();
        rig.named_first(app);
        let (x, y) = rig.middle;
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        kept_at(&dir.join("home/.local/state/trindle/own-helpers"), || {
            let click = |rig: &mut Rig| {
                rig.made_up_move(x, y);
                rig.made_up_click();
                rig.named_first(app)
            };
            assert_eq!(click(&mut rig), [false, false], "a made-up mouse pressed as the person with no helper switched on");
            switch(Helper::Clicker, true, false).unwrap();
            rig.a_moment();
            assert_eq!(click(&mut rig), [false, false], "the clicking helper counted, switched on by a press that was not the person's");
            switch(Helper::Keyboard, true, true).unwrap();
            rig.a_moment();
            assert_eq!(click(&mut rig), [false, false], "the on-screen keyboard's switch counted for a made-up mouse");
            switch(Helper::Clicker, true, true).unwrap();
            rig.a_moment();
            assert_eq!(click(&mut rig), [true, true], "the clicking helper's click was not the person's own once they switched it on");
            rig.made_move(&plugin, x, y);
            rig.made_click(&plugin);
            assert_eq!(rig.named_first(app), [false, false], "a plugin's made-up mouse counted as the clicking helper");
            switch(Helper::Clicker, false, false).unwrap();
            rig.a_moment();
            assert_eq!(click(&mut rig), [false, false], "the clicking helper still counted once switched off");
        });
    });
}

#[test]
fn only_flow_may_unlock_and_only_with_the_pin() {
    alone("rig::tests::whose_press::only_flow_may_unlock_and_only_with_the_pin", |dir| {
        use crate::own_hands::FLOW;
        let mut rig = Rig::new(dir);
        rig.flow_from_the_catalogue();
        let locker = rig.join(ClientState::default());
        let flow = rig.boxed_as(Some(crate::sandbox::CONTROL), Some(FLOW));
        let note = crate::flow_unlock::notes();
        assert!(rig.sees_unlock(flow), "Flow was not offered the way to unlock");
        let other = rig.boxed_as(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        let named_flow = rig.boxed_as(Some(crate::sandbox::CONTROL), Some("org.example.Flow"));
        let plain_box = rig.boxed_as(Some(crate::sandbox::BOXED), Some(FLOW));
        let outside = rig.join(ClientState::default());
        let nameless = rig.boxed_as(Some(crate::sandbox::CONTROL), None);
        let mark = rig.dir.join("home/.local/share/trindle/plugins").join(FLOW).join(crate::own_hands::MARK);
        std::fs::remove_file(&mark).unwrap();
        let unmarked = rig.boxed_as(Some(crate::sandbox::CONTROL), Some(FLOW));
        rig.flow_from_the_catalogue();
        crate::pin::set("4821").unwrap();
        rig.lock_by(locker);
        for (who, name) in [(other, "another plugin"), (named_flow, "a plugin called Flow"), (plain_box, "a box without control"),
                            (outside, "a program outside every box"), (nameless, "a box with no id"), (unmarked, "Flow with no mark")] {
            assert!(!rig.sees_unlock(who), "{name} was offered the way to unlock");
        }
        std::fs::remove_file(&mark).unwrap();
        for (who, name) in [(other, "another plugin"), (named_flow, "a plugin called Flow"), (plain_box, "a box without control"),
                            (outside, "a program outside every box"), (nameless, "a box with no id"), (flow, "Flow with no mark")] {
            assert!(rig.unlock_straight(who, "4821").is_err(), "{name} unlocked the screen");
            assert!(rig.state.is_locked(), "{name} unlocked the screen");
        }
        rig.flow_from_the_catalogue();
        flow_gets_the_password_and_wrong_pins_refused(&mut rig, flow, locker, &note);
        flow_unlocks_with_the_right_pin_in_time(&mut rig, flow, locker, &note);
    });
}

fn flow_gets_the_password_and_wrong_pins_refused(rig: &mut Rig, flow: usize, locker: usize, note: &Path) {
    let said = rig.asks_to_unlock(flow, "correct horse battery").unwrap();
    assert_eq!(said, Err(crate::flow_unlock::NOT_A_PIN.to_string()));
    assert_eq!(rig.asks_to_unlock(flow, "1111").unwrap(), Err(crate::flow_unlock::wrong(4)));
    assert!(rig.state.is_locked() && !note.exists(), "a wrong PIN unlocked, or was noted as an unlock");
    let maker = crate::virtual_input::maker(&rig.programs[flow].client);
    let before = rig.pressed(locker).len();
    rig.made_key(&maker, KEY_A);
    assert_eq!(rig.pressed(locker).len(), before, "Flow typed at the locked screen");
}

fn flow_unlocks_with_the_right_pin_in_time(rig: &mut Rig, flow: usize, locker: usize, note: &Path) {
    for left in (0..4).rev() {
        assert_eq!(rig.asks_to_unlock(flow, "2222").unwrap(), Err(crate::flow_unlock::wrong(left)));
    }
    assert_eq!(rig.asks_to_unlock(flow, "4821").unwrap(), Err(crate::flow_unlock::TOO_MANY.to_string()));
    assert!(rig.state.is_locked());
    rig.state.session_lock.flow_tries = 0;
    rig.state.session_lock.locked_at = Some(std::time::Instant::now() - std::time::Duration::from_secs(60 * 60 * 3));
    assert_eq!(rig.asks_to_unlock(flow, "4821").unwrap(), Err(crate::flow_unlock::LATE.to_string()));
    rig.state.session_lock.locked_at = Some(std::time::Instant::now());
    assert_eq!(rig.asks_to_unlock(flow, "4821").unwrap(), Ok(()));
    assert!(rig.lock_ended(locker), "Flow's right PIN did not unlock");
    let kept = std::fs::read_to_string(note).unwrap();
    assert_eq!(kept.lines().count(), 1);
    assert!(kept.trim_end().ends_with("Z Flow unlocked with the PIN"), "{kept}");
    assert_eq!(rig.asks_to_unlock(flow, "4821").unwrap(), Err(crate::flow_unlock::NOT_LOCKED.to_string()));
    crate::pin::remove();
    let again = rig.join(ClientState::default());
    rig.lock_by(again);
    assert_eq!(rig.asks_to_unlock(flow, "4821").unwrap(), Err(crate::flow_unlock::NO_PIN.to_string()));
    assert!(rig.state.is_locked());
}

#[test]
fn a_refused_press_says_why_where_it_was_kept_from() {
    alone("rig::tests::whose_press::a_refused_press_says_why_where_it_was_kept_from", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        beat(&mut ring.pulse);
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        rig.flow_from_the_catalogue();
        let flow = rig.maker(Some(crate::sandbox::CONTROL), Some(crate::own_hands::FLOW));
        let question = rig.panel(ring.who, crate::trusted::QUESTION);
        rig.refused(ring.who);
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.refused(ring.who), 1, "the question was not told a press was kept from it");
        rig.made_key(&plugin, KEY_A);
        assert_eq!(rig.refused(ring.who), 0, "the question was told twice in a second");
        rig.a_moment();
        rig.a_moment();
        rig.key_by_hand(KEY_A);
        rig.made_key(&flow, KEY_A);
        assert_eq!(rig.refused(ring.who), 0, "the question was told of the person's own key, or Flow's");
        rig.put_away(&question);
        let chooser = rig.join(ClientState::default());
        rig.window_called(chooser, crate::person_only::CHOOSER);
        rig.onto_the_window_in_front();
        let (x, y) = rig.middle;
        rig.made_move(&plugin, x, y);
        rig.made_click(&plugin);
        assert_eq!(rig.refused(chooser), 1, "the file chooser was not told a click was kept from it");
        assert_eq!(rig.refused(window), 0);
        rig.a_moment();
        rig.a_moment();
        rig.made_key(&flow, KEY_A);
        assert_eq!(rig.refused(chooser), 0, "Flow's key into the file chooser was refused");
    });
}

#[test]
fn the_ring_is_told_which_presses_were_the_persons_own_and_no_made_up_one() {
    alone("rig::tests::whose_press::the_ring_is_told_which_presses_were_the_persons_own_and_no_made_up_one", |dir| {
        let mark = dir.join("home/.local/state/trindle/own-helpers");
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        // SAFETY: fcntl only sets a flag on a file the test owns.
        assert_eq!(unsafe { libc::fcntl(ring.hand.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) }, 0);
        let mut seen = crate::own_press::Seen::default();
        let mut own = |rig: &mut Rig, ring: &mut KeptRing| -> Vec<bool> {
            seen.read(&mut ring.hand);
            rig.numbered(ring.who).into_iter().map(|n| seen.has(n)).collect()
        };
        beat(&mut ring.pulse);
        let panel = rig.panel(ring.who, "trindle-ring");
        assert!(rig.has_keyboard(ring.who));
        rig.key_by_hand(KEY_A);
        assert_eq!(own(&mut rig, &mut ring), [true, true], "the person's own key was not said to be theirs");
        let plugin = rig.maker(Some(crate::sandbox::CONTROL), Some("org.example.plugin"));
        rig.allow("org.example.plugin", &["control", "summon"]);
        rig.made_key(&plugin, KEY_A);
        assert_eq!(own(&mut rig, &mut ring), [false, false], "a made-up key was said to be the person's own");
        rig.made_up_key(KEY_A);
        assert_eq!(own(&mut rig, &mut ring), [false, false], "a made-up key from outside a box was said to be the person's own");
        let on_screen = rig.join(ClientState::default());
        crate::own_helpers::kept_at(&mark, || {
            use crate::own_helpers::Helper;
            use crate::own_helpers_set::switch;
            rig.on_screen_key(on_screen, KEY_A);
            let off = own(&mut rig, &mut ring);
            assert!(off.iter().all(|own| !own), "an on-screen keyboard switched off was said to be the person's own");
            switch(Helper::Keyboard, true, true).unwrap();
            rig.on_screen_key(on_screen, KEY_A);
            let on = own(&mut rig, &mut ring);
            assert!(!on.is_empty() && on.iter().all(|own| *own), "the on-screen keyboard the person switched on was not heard on the ring's power choices: {on:?}");
            switch(Helper::Keyboard, false, false).unwrap();
        });
        rig.move_by_hand(0.5, 0.5);
        rig.click_by_hand();
        assert_eq!(own(&mut rig, &mut ring), [true, true], "the person's own click was not said to be theirs");
        rig.made_click(&plugin);
        assert_eq!(own(&mut rig, &mut ring), [false, false], "a made-up click was said to be the person's own");
        rig.made_move(&plugin, 0.51, 0.5);
        rig.click_by_hand();
        assert_eq!(own(&mut rig, &mut ring), [false, false], "a click where a made-up mouse put the pointer was said to be the person's own");
        rig.move_by_hand(0.5, 0.5);
        rig.click_by_hand();
        assert_eq!(own(&mut rig, &mut ring), [true, true]);
        rig.put_away(&panel);
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A]);
        let mut waiting = [0u8; 4];
        assert!(std::io::Read::read(&mut ring.hand, &mut waiting).is_err(), "a key for a window was written to the ring");
    });
}

#[test]
fn the_screen_readers_command_is_told_only_of_the_persons_own_key() {
    alone("rig::tests::whose_press::the_screen_readers_command_is_told_only_of_the_persons_own_key", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_screen_reader();
        rig.made_up_key(KEY_F7);
        assert_eq!(rig.settings_ran(), ["settings --screen-reader own=unsaid"], "a made-up key had the screen reader's command told it was the person's own");
        rig.key_by_hand(KEY_F7);
        assert_eq!(rig.settings_ran()[1..], ["settings --screen-reader own=1"], "the person's own key was not said to the screen reader's command");
        rig.key_by_hand(KEY_F6);
        assert_eq!(rig.settings_ran()[2..], ["settings --panel power own=unsaid"], "another command was told its key was the person's own");
    });
}

#[test]
fn a_made_up_key_starts_the_screenshot_tool_as_any_command_and_the_persons_own_key_as_theirs() {
    alone("rig::tests::whose_press::a_made_up_key_starts_the_screenshot_tool_as_any_command_and_the_persons_own_key_as_theirs", |dir| {
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        rig.kept_ring();
        rig.made_up_key(KEY_F6);
        assert_eq!(rig.shots_by_command(), 1, "a made-up key had the compositor start the screenshot tool as the person's own");
        rig.key_by_hand(KEY_F6);
        assert_eq!(rig.shots_by_command(), 1, "the person's own key ran the screenshot tool as a command");
    });
}
