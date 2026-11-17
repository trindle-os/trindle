//! What an app may be allowed, in the plain words the permission window and
//! the Permissions page both use, and what a Flathub app has, read from
//! flatpak and changed with the person's own overrides. The permissions
//! page and the assistants' set_app_permission both use the one list.
//! Every word is in share/lang/en/access.ftl.

use std::collections::BTreeSet;

use super::run;
use crate::toolkit::t;

pub struct Ask {
    pub name: &'static str,
    pub without: &'static str,

    pub title: &'static str,

    pub off: &'static str,
}

#[cfg(test)]
pub const ASKS: &[&str] = &[
    "network", "sound", "microphone", "sound-and-microphone", "files", "home", "devices", "bluetooth", "notifications",
    "background", "screen", "x11", "computer", "control", "capture", "copied", "summon", "listen", "home-network",
    "usb-devices", "expand", "accessibility",
];

pub fn warning(key: &str) -> Option<&'static str> {
    match key {
        "control" => Some(t!("access-control-warning")),
        "capture" => Some(t!("access-capture-warning")),
        "copied" => Some(t!("access-copied-warning")),
        "summon" => Some(t!("access-summon-warning")),
        "home-network" => Some(t!("access-home-network-warning")),
        "usb-devices" => Some(t!("access-usb-devices-warning")),
        "expand" => Some(t!("access-expand-warning")),
        "accessibility" => Some(t!("access-accessibility-warning")),
        _ => None,
    }
}

pub fn row_note(key: &str) -> Option<&'static str> {
    match key {
        "control" | "capture" | "copied" | "summon" => Some(t!("access-note-acts-as-you")),
        "home-network" => Some(t!("access-note-home-network")),
        "usb-devices" => Some(t!("access-note-usb-devices")),
        "expand" => Some(t!("access-note-expand")),
        "accessibility" => Some(t!("access-note-accessibility")),
        _ => None,
    }
}

pub fn listen_detail(doors: &[super::firewall::Door]) -> String {
    t!("access-listen-detail", ports = super::firewall::ports_words(doors))
}

pub fn off_line(key: &str) -> &'static str {
    ask(key).off
}

pub fn ask(key: &str) -> Ask {
    match key {
        "network" => Ask { name: t!("access-network-name"), without: t!("access-network-without"), title: t!("access-network-title"), off: t!("access-network-off") },
        "sound" => Ask { name: t!("access-sound-name"), without: t!("access-sound-without"), title: t!("access-sound-title"), off: t!("access-sound-off") },
        "microphone" => Ask { name: t!("access-microphone-name"), without: t!("access-microphone-without"), title: t!("access-microphone-title"), off: t!("access-microphone-off") },
        "sound-and-microphone" => Ask { name: t!("access-sound-and-microphone-name"), without: t!("access-sound-and-microphone-without"), title: t!("access-sound-and-microphone-title"), off: t!("access-sound-and-microphone-off") },
        "files" => Ask { name: t!("access-files-name"), without: t!("access-files-without"), title: t!("access-files-title"), off: t!("access-files-off") },
        "home" => Ask { name: t!("access-home-name"), without: t!("access-home-without"), title: t!("access-home-title"), off: t!("access-home-off") },
        "devices" => Ask { name: t!("access-devices-name"), without: t!("access-devices-without"), title: t!("access-devices-title"), off: t!("access-devices-off") },
        "bluetooth" => Ask { name: t!("access-bluetooth-name"), without: t!("access-bluetooth-without"), title: t!("access-bluetooth-title"), off: t!("access-bluetooth-off") },
        "notifications" => Ask { name: t!("access-notifications-name"), without: t!("access-notifications-without"), title: t!("access-notifications-title"), off: t!("access-notifications-off") },
        "background" => Ask { name: t!("access-background-name"), without: t!("access-background-without"), title: t!("access-background-title"), off: t!("access-background-off") },
        "screen" => Ask { name: t!("access-screen-name"), without: t!("access-screen-without"), title: t!("access-screen-title"), off: t!("access-screen-off") },
        "x11" => Ask { name: t!("access-x11-name"), without: t!("access-x11-without"), title: t!("access-x11-title"), off: t!("access-x11-off") },
        "computer" => Ask { name: t!("access-computer-name"), without: t!("access-computer-without"), title: t!("access-computer-title"), off: t!("access-computer-off") },
        "control" => Ask { name: t!("access-control-name"), without: t!("access-control-without"), title: t!("access-control-title"), off: t!("access-control-off") },
        "capture" => Ask { name: t!("access-capture-name"), without: t!("access-capture-without"), title: t!("access-capture-title"), off: t!("access-capture-off") },
        "copied" => Ask { name: t!("access-copied-name"), without: t!("access-copied-without"), title: t!("access-copied-title"), off: t!("access-copied-off") },
        "summon" => Ask { name: t!("access-summon-name"), without: t!("access-summon-without"), title: t!("access-summon-title"), off: t!("access-summon-off") },
        "listen" => Ask { name: t!("access-listen-name"), without: t!("access-listen-without"), title: t!("access-listen-title"), off: t!("access-listen-off") },
        "home-network" => Ask { name: t!("access-home-network-name"), without: t!("access-home-network-without"), title: t!("access-home-network-title"), off: t!("access-home-network-off") },
        "usb-devices" => Ask { name: t!("access-usb-devices-name"), without: t!("access-usb-devices-without"), title: t!("access-usb-devices-title"), off: t!("access-usb-devices-off") },
        "expand" => Ask { name: t!("access-expand-name"), without: t!("access-expand-without"), title: t!("access-expand-title"), off: t!("access-expand-off") },
        "accessibility" => Ask { name: t!("access-accessibility-name"), without: t!("access-accessibility-without"), title: t!("access-accessibility-title"), off: t!("access-accessibility-off") },
        _ => Ask { name: t!("access-unnamed-name"), without: "", title: t!("access-unnamed-title"), off: "" },
    }
}

pub fn title(key: &str) -> &'static str {
    ask(key).title
}

fn kinds() -> [(&'static str, &'static str, &'static [&'static str]); 11] {
    [
        ("TerminalEmulator", t!("access-kind-terminal"), &["microphone", "devices", "screen"]),
        ("Calculator", t!("access-kind-calculator"), &["network", "microphone", "files", "devices", "screen", "background"]),
        ("Photography", t!("access-kind-photo"), &["microphone", "screen", "background"]),
        ("TextEditor", t!("access-kind-text-editor"), &["microphone", "devices", "screen"]),
        ("WebBrowser", t!("access-kind-browser"), &["screen"]),
        ("Office", t!("access-kind-office"), &["microphone", "devices", "screen"]),
        ("Graphics", t!("access-kind-graphics"), &["microphone", "background"]),
        ("Game", t!("access-kind-game"), &["screen", "files", "background"]),
        ("AudioVideo", t!("access-kind-sound-video"), &["screen"]),
        ("Development", t!("access-kind-programming"), &["microphone", "devices"]),
        ("Utility", t!("access-kind-utility"), &["microphone", "devices", "screen"]),
    ]
}

pub fn unusual(categories: &str, key: &str) -> Option<String> {
    let has: Vec<&str> = categories.split(';').map(str::trim).filter(|c| !c.is_empty()).collect();
    let kinds = kinds();
    let kind = kinds.iter().find(|(c, _, _)| has.contains(c));
    let odd = match kind {
        Some((c, _, keys)) => keys.contains(&key) || (key == "computer" && *c != "TerminalEmulator"),
        None => key == "computer",
    };
    let name = kind.map_or_else(|| t!("access-kind-other"), |(_, n, _)| *n);
    odd.then(|| t!("access-unusual", kind = name))
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

fn plain_app_id(id: &str) -> bool {
    id.len() <= 255 && id.contains('.') && !id.starts_with(['-', '.']) && !id.ends_with('.') && !id.contains("..")
        && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

pub fn flatpak_set(id: &str, key: &str, on: bool) -> Result<(), String> {
    if !plain_app_id(id) {
        return Err(t!("common-flatpak-not-an-app").to_string());
    }
    let p = FLATPAK.iter().find(|p| p.key == key).ok_or_else(|| t!("common-flatpak-not-a-permission").to_string())?;
    for flag in if on { p.yes } else { p.no } {
        let out = std::process::Command::new("flatpak").args(["override", "--user", flag, id]).output()
            .map_err(|e| e.to_string())?;
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
            assert!(!a.name.is_empty() && a.without.ends_with('.'), "{key}");

            for word in ["/", "Wayland", "X11", "pipewire", "pulse", ".so", "--"] {
                assert!(!a.name.contains(word) && !a.without.contains(word), "{key} says {word}");
            }
        }
        assert_eq!(ask("microphone").name, "your microphone");
        assert_eq!(ask("microphone").without, "It will not be able to hear you.");
        assert_eq!(title("network"), "The internet");
        for p in FLATPAK {
            assert_ne!(ask(p.words).title, ask("nothing").title, "{} has words", p.key);
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
        assert_eq!(ask("copied").name, "the files you copy");
        assert_eq!(title("copied"), "Read the files you copy");
        assert_eq!(icon("copied"), "edit-copy-symbolic");
        assert!(warning("network").is_none());
    }

    #[test]
    fn the_home_network_and_usb_devices_say_their_risk_calmly_and_exactly() {
        assert_eq!(ask("home-network").name, "devices on your local network only");
        assert_eq!(title("home-network"), "Reach devices on your local network only");
        assert_eq!(ask("usb-devices").name, "scanners and USB devices");
        assert_eq!(title("usb-devices"), "Use scanners and USB devices");
        assert_eq!(warning("home-network"), Some("It can talk to every device on your local network, your router included, but not to the internet. Only allow this for a plugin you trust."));
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
        assert_eq!(ask("accessibility").name, "reading every window aloud");
        assert_eq!(title("accessibility"), "Read what every window shows and say it aloud");
        assert_eq!(ask("accessibility").without, "It will not be able to read your windows aloud.");
        let w = warning("accessibility").unwrap();
        assert!(w.contains("read everything on your screen") && w.contains("press buttons and type text in other windows") && w.ends_with("Only allow this for a plugin you trust."));
        assert_eq!(row_note("accessibility"), Some("it can read everything on your screen"));
        assert_ne!(row_note("accessibility"), row_note("control"), "reading is not acting as the person");
    }

    #[test]
    fn every_warning_has_a_note_for_the_permissions_row_and_the_rest_have_none() {
        for key in ASKS {
            assert_eq!(warning(key).is_some(), row_note(key).is_some(), "{key}");
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

    #[test]
    fn no_permission_word_is_a_literal_english_string_in_the_code() {
        let text = include_str!("access.rs");
        let live = text.split("#[cfg(test)]").next().unwrap();
        let code: String = live.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
        let mut rest = code.as_str();
        while let Some(a) = rest.find('"') {
            let Some(len) = rest[a + 1..].find('"') else { break };
            let lit = &rest[a + 1..a + 1 + len];
            rest = &rest[a + len + 2..];
            let english = lit.contains(' ') && lit.chars().any(|c| c.is_ascii_lowercase());
            assert!(!english, "access.rs has English written in the code: {lit}");
        }
        let ftl = include_str!("../../share/lang/en/access.ftl");
        for key in ASKS {
            for part in ["name", "without", "title", "off"] {
                assert!(ftl.contains(&format!("\naccess-{key}-{part} = ")), "access.ftl lacks access-{key}-{part}");
            }
        }
        assert_eq!(title("usb-devices"), "Use scanners and USB devices");
    }

    #[test]
    fn no_code_in_access_rs_changes_the_case_of_a_translated_string() {
        let live = include_str!("access.rs").split("#[cfg(test)]").next().unwrap();
        for call in ["to_lowercase", "to_uppercase", "to_ascii_lowercase", "to_ascii_uppercase", "make_ascii"] {
            assert!(!live.contains(call), "access.rs changes case with {call}; each language writes its own");
        }
    }

    #[test]
    fn an_id_that_flatpak_would_read_as_an_option_or_as_every_app_is_refused() {
        for bad in ["", "--reset", "-x.y", "org..x", ".org.x", "org.x;rm", "org.x y", "noname"] {
            assert!(!plain_app_id(bad), "{bad:?}");
            assert!(flatpak_set(bad, "network", true).is_err(), "{bad:?}");
        }
        assert!(plain_app_id("org.mozilla.firefox") && plain_app_id("com.valvesoftware.Steam") && plain_app_id("io.github.some-app_2.X"));
    }
}
