//! What an app may be allowed, in the plain words the permission window and the Permissions page both use, and what a Flathub app has, read from flatpak and changed with the person's own overrides.

use std::collections::BTreeSet;

use super::run;

pub struct Ask {
    pub key: &'static str,
    pub name: &'static str,
    pub without: &'static str,
}

pub const ASKS: &[Ask] = &[
    Ask { key: "network", name: "the internet", without: "It will not be able to reach the internet or other computers nearby." },
    Ask { key: "sound", name: "play sound", without: "It will make no sound." },
    Ask { key: "microphone", name: "your microphone", without: "It will not be able to hear you." },
    Ask { key: "sound-and-microphone", name: "play sound and use your microphone", without: "It will make no sound and will not be able to hear you." },
    Ask { key: "files", name: "your Documents, Downloads, Pictures, Music, Videos and Desktop folders",
          without: "It will not be able to open your files or save into your folders." },
    Ask { key: "home", name: "everything in your home folder, hidden settings files included",
          without: "It will not be able to open your files or save into your folders." },
    Ask { key: "devices", name: "your camera and other plugged-in devices", without: "It will not be able to see you or use what you plug in." },
    Ask { key: "bluetooth", name: "Bluetooth, for headphones and other things nearby", without: "It will not be able to talk to anything over Bluetooth." },
    Ask { key: "notifications", name: "show notifications", without: "It will not be able to tell you when something happens." },
    Ask { key: "background", name: "start by itself when you log in", without: "It will only run while you have it open." },
    Ask { key: "screen", name: "take pictures of your screen, read what you copy and type as if it were your keyboard",
          without: "It will not be able to see other windows or read what you copy at any time." },
    Ask { key: "x11", name: "draw its windows the older way, which lets it watch other older apps and what is typed in them",
          without: "Its windows will not open." },
    Ask { key: "computer", name: "work outside its box, with everything you can reach yourself",
          without: "What you type into it stays shut in its box, so sudo and your own files will not work there." },
    Ask { key: "control", name: "control your keyboard, mouse and clipboard",
          without: "It will not be able to move the pointer, type or share what you copy." },
    Ask { key: "capture", name: "share your mouse and keyboard with another computer",
          without: "The pointer will not be able to cross to another computer." },
    Ask { key: "copied", name: "read the files you copy",
          without: "It will not be able to read the files you copy, so it cannot carry them to another computer." },
    Ask { key: "summon", name: "open the ring over a full-screen window, as if you pressed the key",
          without: "Its keys and clicks will open the ring on the desktop only, never over a full-screen window." },
    Ask { key: "listen", name: "let other computers on your home network reach it",
          without: "Other computers on your home network will not be able to find it or connect to it." },
    Ask { key: "home-network", name: "reach devices on your home network only",
          without: "It will not be able to find or talk to printers, scanners or other devices at home." },
    Ask { key: "usb-devices", name: "use scanners and USB devices",
          without: "It will not be able to use a scanner that is plugged into this computer." },
    Ask { key: "expand", name: "expand your short codes as you type",
          without: "Its short codes will not be expanded as you type." },
    Ask { key: "accessibility", name: "read what every window shows and say it aloud",
          without: "It will not be able to read your windows aloud." },
];

pub fn warning(key: &str) -> Option<&'static str> {
    match key {
        "control" => Some("It can type, click and read what you copy as if it were you. Only allow this for a plugin you trust."),
        "capture" => Some("While the pointer is on the other computer, everything you type and click goes to this plugin. Only allow this for a plugin you trust."),
        "copied" => Some("It can read exactly the files you copy while it is allowed, and nothing else. Only allow this for a plugin you trust."),
        "summon" => Some("It can put the ring, which takes everything you type, over a game or a film without you touching a key. Only allow this for a plugin you trust."),
        "home-network" => Some("It can talk to every device on your home network, your router included, but not to the internet. Only allow this for a plugin you trust."),
        "usb-devices" => Some("It can use the scanners and image devices that are plugged in when it starts, and no other USB device. Only allow this for a plugin you trust."),
        "expand" => Some("Trindle watches what you type for its codes and types its words for you. The plugin never sees what you type. Only allow this for a plugin you trust."),
        "accessibility" => Some("It can read everything on your screen. Like any screen reader, it can also press buttons and type text in other windows, though it has no keyboard or mouse of its own. Only allow this for a plugin you trust."),
        _ => None,
    }
}

pub fn row_note(key: &str) -> Option<&'static str> {
    match key {
        "control" | "capture" | "copied" | "summon" => Some("it can act as you"),
        "home-network" => Some("it can reach every device on your home network"),
        "usb-devices" => Some("it can use scanners and image devices"),
        "expand" => Some("it can type into your windows"),
        "accessibility" => Some("it can read everything on your screen"),
        _ => None,
    }
}

pub fn listen_detail(doors: &[super::firewall::Door]) -> String {
    format!("{}, from computers on your home network only. It needs the internet allowed too.", super::firewall::ports_words(doors))
}

pub fn off_line(without: &str) -> String {
    let mut c = without.chars();
    match c.next() {
        None => String::new(),
        Some(f) => format!("Off: {}{}", f.to_lowercase(), c.as_str()),
    }
}

pub fn ask(key: &str) -> Ask {
    ASKS.iter().find(|a| a.key == key).map(|a| Ask { key: a.key, name: a.name, without: a.without })
        .unwrap_or(Ask { key: "", name: "something it did not name", without: "" })
}

pub fn title(key: &str) -> String {
    let name = ask(key).name;
    let mut c = name.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

const KINDS: &[(&str, &str, &[&str])] = &[
    ("TerminalEmulator", "a terminal", &["microphone", "devices", "screen"]),
    ("Calculator", "a calculator", &["network", "microphone", "files", "devices", "screen", "background"]),
    ("Photography", "a photo app", &["microphone", "screen", "background"]),
    ("TextEditor", "a text editor", &["microphone", "devices", "screen"]),
    ("WebBrowser", "a web browser", &["screen"]),
    ("Office", "an office app", &["microphone", "devices", "screen"]),
    ("Graphics", "a graphics app", &["microphone", "background"]),
    ("Game", "a game", &["screen", "files", "background"]),
    ("AudioVideo", "a sound and video app", &["screen"]),
    ("Development", "a programming tool", &["microphone", "devices"]),
    ("Utility", "a utility", &["microphone", "devices", "screen"]),
];

pub fn unusual(categories: &str, key: &str) -> Option<String> {
    let has: Vec<&str> = categories.split(';').map(str::trim).filter(|c| !c.is_empty()).collect();
    let kind = KINDS.iter().find(|(c, _, _)| has.contains(c));
    let odd = match kind {
        Some((c, _, keys)) => keys.contains(&key) || (key == "computer" && *c != "TerminalEmulator"),
        None => key == "computer",
    };
    let name = kind.map_or("an app like this", |(_, n, _)| *n);
    odd.then(|| format!("This is unusual for {name}. Only allow it if you trust where this app came from."))
}

pub struct Permission {
    pub key: &'static str,
    pub words: &'static str,
    pub yes: &'static [&'static str],
    pub no: &'static [&'static str],
}

pub const FLATPAK: &[Permission] = &[
    Permission { key: "network", words: "network", yes: &["--share=network"], no: &["--unshare=network"] },
    Permission { key: "home", words: "home", yes: &["--filesystem=home"], no: &["--nofilesystem=home", "--nofilesystem=host"] },
    Permission { key: "devices", words: "devices", yes: &["--device=all"], no: &["--nodevice=all"] },
    Permission { key: "sound", words: "sound-and-microphone", yes: &["--socket=pulseaudio"], no: &["--nosocket=pulseaudio"] },
    Permission { key: "bluetooth", words: "bluetooth", yes: &["--allow=bluetooth"], no: &["--disallow=bluetooth"] },
];

pub fn icon(key: &str) -> &'static str {
    match key {
        "network" => "network-wireless-symbolic",
        "home" | "files" => "folder-symbolic",
        "devices" => "camera-web-symbolic",
        "sound" | "sound-and-microphone" => "audio-volume-high-symbolic",
        "microphone" => "audio-input-microphone-symbolic",
        "screen" => "video-display-symbolic",
        "x11" => "preferences-desktop-display-symbolic",
        "computer" => "utilities-terminal-symbolic",
        "bluetooth" => "bluetooth-active-symbolic",
        "notifications" => "preferences-system-notifications-symbolic",
        "background" => "system-run-symbolic",
        "control" => "input-mouse-symbolic",
        "capture" => "input-keyboard-symbolic",
        "listen" => "network-server-symbolic",
        "copied" => "edit-copy-symbolic",
        "summon" => "view-fullscreen-symbolic",
        "home-network" => crate::common::in_use::HOME_NETWORK_ICON,
        "usb-devices" => crate::common::in_use::USB_ICON,
        "expand" => "input-keyboard-symbolic",
        "accessibility" => "preferences-desktop-accessibility-symbolic",
        _ => "security-high-symbolic",
    }
}

pub fn flatpak_places() -> [std::path::PathBuf; 2] {
    let own = std::env::var_os("FLATPAK_USER_DIR").map(std::path::PathBuf::from)
        .unwrap_or_else(|| crate::common::files::home().join(".local/share/flatpak"));
    [own, std::path::PathBuf::from("/var/lib/flatpak")]
}

pub fn flatpak_apps_named(places: &[std::path::PathBuf]) -> Vec<String> {
    let all: BTreeSet<String> = places.iter().flat_map(|p| std::fs::read_dir(p.join("app")).into_iter().flatten().flatten())
        .filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.contains('.') && !name.starts_with('.')).collect();
    all.into_iter().collect()
}

pub fn flatpak_apps() -> Vec<String> {
    let all: BTreeSet<String> = run::text(&["flatpak", "list", "--app", "--columns=application"]).lines()
        .map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect();
    all.into_iter().collect()
}

pub fn flatpak_allowed(id: &str) -> Vec<&'static str> {
    allowed_in(&run::text(&["flatpak", "info", "--show-permissions", id]))
}

fn allowed_in(text: &str) -> Vec<&'static str> {
    let has = |key: &str, names: &[&str]| text.lines()
        .filter_map(|l| l.trim().strip_prefix(key).and_then(|r| r.strip_prefix('=')))
        .flat_map(|v| v.split(';'))
        .any(|v| names.contains(&v.split(':').next().unwrap_or("")));
    let mut on = Vec::new();
    for (key, field, names) in [
        ("network", "shared", &["network"][..]),
        ("home", "filesystems", &["home", "host"][..]),
        ("devices", "devices", &["all"][..]),
        ("sound", "sockets", &["pulseaudio"][..]),
        ("bluetooth", "features", &["bluetooth"][..]),
    ] {
        if has(field, names) {
            on.push(key);
        }
    }
    on
}

pub fn flatpak_uses_x11(text: &str) -> bool {
    text.lines()
        .filter_map(|l| l.trim().strip_prefix("sockets=").map(str::to_string))
        .any(|v| v.split(';').any(|s| s.trim() == "x11"))
}

pub fn flatpak_set(id: &str, key: &str, on: bool) -> Result<(), String> {
    let p = FLATPAK.iter().find(|p| p.key == key).ok_or_else(|| format!("{key} is not a permission"))?;
    for flag in if on { p.yes } else { p.no } {
        let out = std::process::Command::new("flatpak").args(["override", "--user", flag, id]).output()
            .map_err(|e| format!("flatpak: {e}"))?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_permission_has_plain_words() {
        for key in ["network", "sound", "microphone", "files", "devices", "bluetooth", "notifications", "background",
                    "screen", "x11", "computer", "control", "capture", "copied", "listen", "summon", "home-network", "usb-devices", "expand", "accessibility"] {
            let a = ask(key);
            assert_eq!(a.key, key);
            assert!(!a.name.is_empty() && a.without.ends_with('.'), "{key}");
            for word in ["/", "Wayland", "X11", "pipewire", "pulse", ".so", "--"] {
                assert!(!a.name.contains(word) && !a.without.contains(word), "{key} says {word}");
            }
        }
        assert_eq!(ask("microphone").name, "your microphone");
        assert_eq!(ask("microphone").without, "It will not be able to hear you.");
        assert_eq!(title("network"), "The internet");
        for p in FLATPAK {
            assert_ne!(ask(p.words).key, "", "{} has words", p.key);
        }
    }

    #[test]
    fn control_always_carries_its_warning() {
        let w = warning("control").unwrap();
        assert!(w.contains("trust") && w.ends_with('.'));
        assert_eq!(icon("control"), "input-mouse-symbolic");
        assert!(warning("capture").is_some_and(|w| w.contains("trust") && w.ends_with('.')));
        let w = warning("copied").unwrap();
        assert_eq!(w, "It can read exactly the files you copy while it is allowed, and nothing else. Only allow this for a plugin you trust.");
        assert_eq!(ask("copied").name, "read the files you copy");
        assert_eq!(title("copied"), "Read the files you copy");
        assert_eq!(icon("copied"), "edit-copy-symbolic");
        assert!(warning("network").is_none());
    }

    #[test]
    fn the_home_network_and_usb_devices_say_their_risk_calmly_and_exactly() {
        assert_eq!(ask("home-network").name, "reach devices on your home network only");
        assert_eq!(title("home-network"), "Reach devices on your home network only");
        assert_eq!(ask("usb-devices").name, "use scanners and USB devices");
        assert_eq!(title("usb-devices"), "Use scanners and USB devices");
        assert_eq!(warning("home-network"), Some("It can talk to every device on your home network, your router included, but not to the internet. Only allow this for a plugin you trust."));
        assert_eq!(warning("usb-devices"), Some("It can use the scanners and image devices that are plugged in when it starts, and no other USB device. Only allow this for a plugin you trust."));
        assert_eq!(icon("home-network"), "network-workgroup-symbolic");
        assert_eq!(icon("usb-devices"), "scanner-symbolic");
        for key in ["home-network", "usb-devices"] {
            let w = warning(key).unwrap();
            assert!(!w.contains('!') && !w.contains("DANGER") && !w.contains(" — ") && w.ends_with('.'), "{key}");
        }
    }

    #[test]
    fn accessibility_says_it_reads_and_can_press_and_type_through_the_bus() {
        assert_eq!(ask("accessibility").name, "read what every window shows and say it aloud");
        assert_eq!(title("accessibility"), "Read what every window shows and say it aloud");
        assert_eq!(ask("accessibility").without, "It will not be able to read your windows aloud.");
        let w = warning("accessibility").unwrap();
        assert!(w.contains("read everything on your screen") && w.contains("press buttons and type text in other windows") && w.ends_with("Only allow this for a plugin you trust."));
        assert_eq!(row_note("accessibility"), Some("it can read everything on your screen"));
        assert_ne!(row_note("accessibility"), row_note("control"), "reading is not acting as the person");
    }

    #[test]
    fn every_warning_has_a_note_for_the_permissions_row_and_the_rest_have_none() {
        for a in ASKS {
            assert_eq!(warning(a.key).is_some(), row_note(a.key).is_some(), "{}", a.key);
        }
        assert_eq!(row_note("control"), row_note("copied"));
        assert_ne!(row_note("home-network"), row_note("control"), "the home network is not acting as the person");
    }

    #[test]
    fn says_when_a_request_is_unusual() {
        let warn = unusual("Utility;Calculator;", "network").unwrap();
        assert_eq!(warn, "This is unusual for a calculator. Only allow it if you trust where this app came from.");
        assert!(unusual("Graphics;Photography;", "screen").unwrap().contains("a photo app"));
        assert!(unusual("Graphics;Photography;", "files").is_none());
        assert!(unusual("Office;WordProcessor;", "microphone").is_some());
        assert!(unusual("Network;Chat;", "microphone").is_none());
        assert!(unusual("System;TerminalEmulator;", "computer").is_none());
        assert!(unusual("Graphics;", "computer").is_some());
        assert!(unusual("", "computer").unwrap().contains("an app like this"));
        assert!(unusual("", "network").is_none());
    }

    #[test]
    fn reads_what_an_app_has() {
        let text = "[Context]\nshared=network;ipc;\nsockets=x11;wayland;pulseaudio;\ndevices=dri;\n\
                    features=!bluetooth;\nfilesystems=xdg-download;host:ro;\n\n[Session Bus Policy]\norg.x=talk\n";
        assert_eq!(allowed_in(text), ["network", "home", "sound"]);
        assert_eq!(allowed_in("[Context]\nshared=!network;ipc;\nfilesystems=!home;xdg-music;\n"), Vec::<&str>::new());
        assert_eq!(allowed_in(""), Vec::<&str>::new());
    }

    #[test]
    fn tells_which_flathub_apps_draw_through_x11() {
        assert!(flatpak_uses_x11("[Context]\nshared=network;ipc;\nsockets=x11;pulseaudio;\n"));
        assert!(flatpak_uses_x11("[Context]\nsockets=wayland;x11;\n"));
        assert!(!flatpak_uses_x11("[Context]\nsockets=wayland;fallback-x11;pulseaudio;\n"));
        assert!(!flatpak_uses_x11("[Context]\nsockets=!x11;wayland;\n"));
        assert!(!flatpak_uses_x11("[Context]\nshared=x11;\n"));
        assert!(!flatpak_uses_x11(""));
    }
}
