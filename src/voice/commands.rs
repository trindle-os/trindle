//! Voice command parsing: pure text in, one Action out. Matching is loose,
//! since transcriptions vary ("Thunder bird").

use crate::toolkit::t;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Nothing,
    Unknown(String),
    Open(usize),
    Switch(usize),
    Close(Option<usize>),
    Volume(&'static str),
    Mute(bool),
    Wallpaper,
    Screenshot,
    Time,
    Level(u8),
    Settings(Option<&'static str>),
    Store(Option<String>),
    Confirm(&'static str),
    Tool(&'static str),
    KeepAwake(bool),
    Radio(&'static str, bool),
    Brightness(&'static str),
    Keyboard,
    Lock,
    Updates,
}

const FILLER: &[&str] = &["please", "can", "could", "you", "would", "the", "a", "an", "my", "me", "for", "i",
    "want", "to", "hey", "trindle", "ok", "okay", "just", "now", "up"];
const OPEN: &[&str] = &["open", "start", "launch", "run", "load"];
const SWITCH: &[&str] = &["switch to", "go to", "show me", "show", "bring up", "bring"];
const CLOSE: &[&str] = &["close", "quit", "exit", "kill"];
const INSTALL: &[&str] = &["install", "get", "download", "find in apps", "search apps for", "find in the store", "search the store for", "look for"];
const PANELS: &[(&str, &[&str])] = &[
    ("ring", &["favourites", "favorites", "ring"]),
    ("display", &["display", "screen", "monitor", "resolution", "brightness"]),

    ("sounds", &["sounds", "sound effects", "chime"]),
    ("audio", &["sound", "audio", "speakers", "microphone", "headphones"]),
    ("network", &["network", "wifi", "wi-fi", "wireless", "internet", "ethernet"]),
    ("bluetooth", &["bluetooth"]),
    ("wallpaper", &["wallpaper", "background"]),
    ("appearance", &["appearance", "theme", "icons", "fonts", "dark mode"]),
    ("keyboard", &["keyboard", "layout"]),
    ("power", &["power", "battery", "suspend settings"]),
    ("default-apps", &["default apps", "default applications"]),
    ("printers", &["printer", "printers", "printing"]),

    ("disks", &["drive", "drives", "disk", "disks", "usb"]),
    ("region", &["language", "region"]),
    ("datetime", &["date", "time zone", "timezone", "clock"]),
    ("users", &["users", "accounts", "password"]),
    ("help", &["help"]),
];

pub fn clean(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0;
    for c in text.to_lowercase().chars() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' => depth = (depth - 1).max(0),
            _ if depth > 0 => {}
            c if c.is_alphanumeric() || c == '-' => out.push(c),
            _ => out.push(' '),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_filler(text: &str) -> String {
    text.split_whitespace().filter(|w| !FILLER.contains(w)).collect::<Vec<_>>().join(" ")
}

fn squash(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}

fn has(text: &str, phrase: &str) -> bool {
    let t = format!(" {text} ");
    t.contains(&format!(" {phrase} "))
}

fn has_any(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|p| has(text, p))
}

fn starts<'a>(text: &'a str, phrases: &[&str]) -> Option<&'a str> {
    for p in phrases {
        if text == *p {
            return Some("");
        }
        if let Some(rest) = text.strip_prefix(&format!("{p} ")) {
            return Some(rest.trim());
        }
    }
    None
}

fn similarity(a: &str, b: &str) -> f64 {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i; b.len() + 1];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
        }
        prev = cur;
    }
    1.0 - prev[b.len()] as f64 / a.len().max(b.len()) as f64
}

pub struct Named<'a> {
    pub name: &'a str,
    pub id: &'a str,
}

const FILE_MANAGER: &[&str] = &["files", "filemanager", "filebrowser", "filesapp", "fileapp"];

fn best_app(words: &str, apps: &[Named]) -> Option<usize> {
    let target = squash(words);
    if target.is_empty() {
        return None;
    }

    if FILE_MANAGER.contains(&target.as_str()) {
        let folders = crate::common::apps::own_key("files");
        if let Some(i) = apps.iter().position(|a| folders.is_some_and(|o| o.knows(a.id))) {
            return Some(i);
        }
    }
    let mut best = (None, 0.0);
    for (i, app) in apps.iter().enumerate() {
        let key = app.id.strip_suffix(".desktop").unwrap_or(app.id);
        for name in [app.name, key.rsplit('.').next().unwrap_or(key)] {
            let n = squash(name);
            if n.is_empty() {
                continue;
            }
            if n == target {
                return Some(i);
            }
            let mut s = similarity(&target, &n);
            if (target.contains(&n) || n.contains(&target)) && n.len().min(target.len()) >= 4 {
                s = s.max(0.8);
            }
            if s > best.1 {
                best = (Some(i), s);
            }
        }
    }
    if best.1 >= 0.72 { best.0 } else { None }
}

fn ring_tool(text: &str) -> Option<(Action, String)> {
    let say = |a: Action, s: &str| Some((a, s.to_string()));
    if has_any(text, &["settings", "setting"]) {
        return None;
    }
    let off = has_any(text, &["off", "disable", "stop", "no longer"]);
    let on = has_any(text, &["on", "enable", "start"]);
    for (radio, names, name) in [("wifi", &["wifi", "wi-fi", "wireless"][..], "Wi-Fi"), ("bluetooth", &["bluetooth"][..], "Bluetooth")] {
        if has_any(text, names) && (on || off) {
            return say(Action::Radio(radio, !off), &if off { t!("voice-radio-off", name = name) } else { t!("voice-radio-on", name = name) });
        }
    }
    if has_any(text, &["brighter", "brightness up", "more brightness"]) || (has(text, "brightness") && has_any(text, &["up", "increase", "raise"])) {
        return say(Action::Brightness("up"), t!("voice-brighter"));
    }
    if has_any(text, &["dimmer", "darker", "dim the screen", "brightness down", "less brightness"])
        || (has(text, "brightness") && has_any(text, &["down", "decrease", "lower"])) {
        return say(Action::Brightness("down"), t!("voice-dimmer"));
    }
    if has_any(text, &["keep awake", "stay awake", "keep the screen on", "caffeine"]) {
        return if off {
            say(Action::KeepAwake(false), t!("voice-keep-awake-off"))
        } else {
            say(Action::KeepAwake(true), t!("voice-keep-awake-on"))
        };
    }
    if has_any(text, &["lock", "lock the screen", "lock screen", "lock the computer"]) {
        return say(Action::Lock, "");
    }
    if has_any(text, &["clipboard", "copy history", "paste history", "copied"]) {
        return say(Action::Tool("clipboard"), "");
    }
    if has_any(text, &["notifications", "notification"]) {
        return say(Action::Tool("notes"), "");
    }
    if has_any(text, &["magnifier", "magnify", "zoom in", "make it bigger"]) {
        return say(Action::Tool("magnifier"), "");
    }
    if has_any(text, &["activity", "activities", "task manager", "what is running"]) {
        return say(Action::Tool("activity"), "");
    }
    if has_any(text, &["on-screen keyboard", "on screen keyboard", "onscreen keyboard", "show the keyboard",
                       "show keyboard", "hide the keyboard", "hide keyboard"]) {
        return say(Action::Keyboard, "");
    }
    if has_any(text, &["updates", "update"]) {
        return say(Action::Updates, t!("voice-updates"));
    }
    None
}

pub fn interpret(heard: &str, apps: &[Named]) -> (Action, String) {
    let text = clean(heard);
    if text.is_empty() {
        return (Action::Nothing, t!("voice-nothing-heard").into());
    }
    if let Some(done) = ring_tool(&text).or_else(|| computer_words(&text)) {
        return done;
    }
    let words = strip_filler(&text);
    if let Some(done) = store_words(&text, &words).or_else(|| settings_words(&text)) {
        return done;
    }
    if let Some(done) = close_or_switch(&text, &words, apps) {
        return done;
    }
    let target = starts(&words, OPEN).unwrap_or(&words);
    match best_app(target, apps) {
        Some(i) => (Action::Open(i), t!("voice-opening-app", name = apps[i].name)),
        None => (Action::Unknown(text.clone()), t!("voice-heard-unknown", heard = heard.trim())),
    }
}

fn say(a: Action, s: &str) -> (Action, String) {
    (a, s.to_string())
}

fn computer_words(text: &str) -> Option<(Action, String)> {
    if has_any(text, &["louder", "volume up", "turn up", "turn it up", "increase volume", "increase the volume"]) {
        return Some(say(Action::Volume("5%+"), t!("voice-louder")));
    }
    if has_any(text, &["quieter", "softer", "volume down", "turn down", "turn it down", "decrease volume",
                        "decrease the volume"]) {
        return Some(say(Action::Volume("5%-"), t!("voice-quieter")));
    }
    if has(text, "unmute") {
        return Some(say(Action::Mute(false), t!("voice-sound-on")));
    }
    if has_any(text, &["mute", "silence"]) {
        return Some(say(Action::Mute(true), t!("voice-muted")));
    }
    if ["next", "change", "new"].iter().any(|a| ["wallpaper", "background", "picture"].iter().any(|b| has(text, &format!("{a} {b}")))) {
        return Some(say(Action::Wallpaper, t!("voice-next-wallpaper")));
    }
    if has_any(text, &["screenshot", "screen shot", "take a picture of the screen"]) {
        return Some(say(Action::Screenshot, t!("voice-screenshot-saved")));
    }
    if has_any(text, &["what time", "the time"]) {
        return Some(say(Action::Time, ""));
    }
    if has_any(text, &["all apps", "every app", "everything", "show all", "more apps"]) {
        return Some(say(Action::Level(1), t!("voice-all-apps")));
    }
    if has_any(text, &["shut down", "shutdown", "power off", "turn off", "turn the computer off"]) {
        return Some(say(Action::Confirm("PowerOff"), t!("voice-confirm-power-off")));
    }
    if has_any(text, &["restart", "reboot"]) {
        return Some(say(Action::Confirm("Reboot"), t!("voice-confirm-reboot")));
    }
    if has_any(text, &["log out", "logout", "sign out"]) {
        return Some(say(Action::Confirm("logout"), t!("voice-confirm-logout")));
    }
    None
}

fn store_words(text: &str, words: &str) -> Option<(Action, String)> {
    if let Some(rest) = starts(text, INSTALL).or_else(|| starts(words, INSTALL)) {
        if !rest.is_empty() {
            let rest = rest.replace(" from apps", "").replace(" in apps", "").replace(" from the store", "").replace(" in the store", "").replace(" from store", "");
            let wanted = strip_filler(&rest);
            let wanted = if wanted.is_empty() { rest.clone() } else { wanted };
            return Some((Action::Store(Some(wanted)), t!("voice-looking-for-app", what = rest.as_str())));
        }
    }
    if has_any(words, &["apps", "store", "app store", "shop"]) && words.split_whitespace().count() <= 3 {
        return Some(say(Action::Store(None), t!("voice-apps")));
    }
    None
}

fn settings_words(text: &str) -> Option<(Action, String)> {
    if has(text, "settings") || has(text, "setting") {
        for (panel, keys) in PANELS {
            if keys.iter().any(|k| text.contains(k)) {

                return Some(say(Action::Settings(Some(panel)), if *panel == "disks" { t!("voice-drives") } else { t!("voice-settings") }));
            }
        }
        return Some(say(Action::Settings(None), t!("voice-settings")));
    }
    None
}

fn close_or_switch(text: &str, words: &str, apps: &[Named]) -> Option<(Action, String)> {
    if let Some(rest) = starts(words, CLOSE) {
        if rest.is_empty() || ["window", "this", "this window", "it"].contains(&rest) {
            return Some(say(Action::Close(None), t!("voice-closing-window")));
        }
        return Some(match best_app(rest, apps) {
            Some(i) => (Action::Close(Some(i)), t!("voice-closing-app", name = apps[i].name)),
            None => (Action::Unknown(rest.to_string()), t!("voice-no-app", name = rest)),
        });
    }
    let switched = starts(text, SWITCH).map(str::to_string).or_else(|| starts(words, &["switch"]).map(str::to_string));
    let rest = strip_filler(&switched?);
    Some(match best_app(&rest, apps) {
        Some(i) => (Action::Switch(i), t!("voice-showing-app", name = apps[i].name)),
        None => (Action::Unknown(rest.clone()), t!("voice-no-app", name = rest.as_str())),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apps() -> Vec<Named<'static>> {
        vec![
            Named { name: "Firefox", id: "org.mozilla.firefox.desktop" },
            Named { name: "Folders", id: "trindle-folders.desktop" },
            Named { name: "Terminal", id: "Alacritty.desktop" },
            Named { name: "Thunderbird", id: "org.mozilla.Thunderbird.desktop" },
            Named { name: "LibreOffice Writer", id: "libreoffice-writer.desktop" },
            Named { name: "Calculator", id: "org.gnome.Calculator.desktop" },
        ]
    }

    fn act(t: &str) -> Action {
        interpret(t, &apps()).0
    }

    #[test]
    fn opening() {
        assert_eq!(act("Open Firefox."), Action::Open(0));
        assert_eq!(act("firefox"), Action::Open(0));
        assert_eq!(act("Can you start the calculator please"), Action::Open(5));
        assert_eq!(act("Thunder bird"), Action::Open(3));
        assert_eq!(act("open libre office writer"), Action::Open(4));

        assert_eq!(act("open folders"), Action::Open(1));
        assert_eq!(act("open files"), Action::Open(1));
        assert_eq!(act("files"), Action::Open(1));
        assert_eq!(act("open the file manager"), Action::Open(1));
        assert_eq!(act("close files"), Action::Close(Some(1)));
    }

    #[test]
    fn other_things() {
        assert_eq!(act("Turn it up"), Action::Volume("5%+"));
        assert_eq!(act("mute"), Action::Mute(true));
        assert_eq!(act("Wi-Fi settings"), Action::Settings(Some("network")));
        assert_eq!(act("open settings"), Action::Settings(None));
        assert_eq!(act("install spotify"), Action::Store(Some("spotify".into())));
        assert_eq!(act("close firefox"), Action::Close(Some(0)));
        assert_eq!(act("close window"), Action::Close(None));
        assert_eq!(act("switch to thunderbird"), Action::Switch(3));
        assert_eq!(act("next wallpaper"), Action::Wallpaper);
    }

    #[test]
    fn the_ring_tools() {
        assert_eq!(act("turn off the wifi"), Action::Radio("wifi", false));
        assert_eq!(act("Wi-Fi on"), Action::Radio("wifi", true));
        assert_eq!(act("switch bluetooth off"), Action::Radio("bluetooth", false));
        assert_eq!(act("turn up the brightness"), Action::Brightness("up"));
        assert_eq!(act("make it dimmer"), Action::Brightness("down"));
        assert_eq!(act("keep awake"), Action::KeepAwake(true));
        assert_eq!(act("stop keep awake"), Action::KeepAwake(false));
        assert_eq!(act("lock the screen"), Action::Lock);
        assert_eq!(act("show my clipboard"), Action::Tool("clipboard"));
        assert_eq!(act("notifications"), Action::Tool("notes"));
        assert_eq!(act("magnifier"), Action::Tool("magnifier"));
        assert_eq!(act("open activities"), Action::Tool("activity"));
        assert_eq!(act("show the keyboard"), Action::Keyboard);
        assert_eq!(act("any updates"), Action::Updates);

        assert_eq!(act("bluetooth settings"), Action::Settings(Some("bluetooth")));
        assert_eq!(act("keyboard settings"), Action::Settings(Some("keyboard")));
        assert!(matches!(act("turn off the computer"), Action::Confirm(_)));
        assert_eq!(act("turn it up"), Action::Volume("5%+"));
    }

    #[test]
    fn ending_the_session_always_asks() {
        assert!(matches!(act("shut down"), Action::Confirm(_)));
        assert!(matches!(act("restart the computer"), Action::Confirm(_)));
    }

    #[test]
    fn everything_the_feature_lists_can_be_said() {
        assert_eq!(act("switch to firefox"), Action::Switch(0));
        assert_eq!(act("go to the calculator"), Action::Switch(5));
        assert_eq!(act("take a screenshot"), Action::Screenshot);
        assert_eq!(act("turn the wifi off"), Action::Radio("wifi", false));
        assert_eq!(act("bluetooth on"), Action::Radio("bluetooth", true));
        assert_eq!(act("brighter"), Action::Brightness("up"));
        assert_eq!(act("dimmer"), Action::Brightness("down"));
        assert_eq!(act("lock"), Action::Lock);
        assert_eq!(act("open the clipboard"), Action::Tool("clipboard"));
        assert_eq!(act("show notifications"), Action::Tool("notes"));
        assert_eq!(act("how many updates are waiting"), Action::Updates);
    }

    #[test]
    fn what_ends_the_session_or_changes_the_system_only_ever_asks() {
        assert_eq!(act("log out"), Action::Confirm("logout"));
        assert_eq!(act("sign out"), Action::Confirm("logout"));
        assert_eq!(act("shut down the computer"), Action::Confirm("PowerOff"));
        assert_eq!(act("reboot"), Action::Confirm("Reboot"));
        for said in ["install the updates", "put the updates in", "update everything", "install all updates"] {
            assert!(matches!(act(said), Action::Updates | Action::Store(_) | Action::Level(_)), "{said}: {:?}", act(said));
        }
    }

    #[test]
    fn nonsense_and_silence() {
        assert_eq!(act("[BLANK_AUDIO]"), Action::Nothing);
        assert!(matches!(act("the weather in paris is lovely"), Action::Unknown(_)));
    }
}
