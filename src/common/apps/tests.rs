//! Tests for apps.rs.

#[cfg(test)]
use super::*;

#[test]
fn flathub_apps_start_through_their_own_xwayland() {
    let v = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let run = v(&["/usr/bin/flatpak", "run", "--branch=stable", "--command=jd", "org.jdownloader.JDownloader", "@@u", "x", "@@"]);
    assert_eq!(with_own_x11(run.clone()), [v(&["trindle-boxed", "--own-x11", "org.jdownloader.JDownloader"]), run].concat());
    for other in [v(&["flatpak", "list"]), v(&["alacritty"]), v(&["/opt/flatpak-run", "run", "x"]), v(&["flatpak", "run"]), v(&[])] {
        assert_eq!(with_own_x11(other.clone()), other);
    }
}

#[test]
fn the_chosen_terminal_wins_else_the_first_found() {
    let dir = std::env::temp_dir().join(format!("trindle-terminals-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let entry = |id: &str, cats: &str| {
        let path = dir.join(id);
        std::fs::write(&path, format!("[Desktop Entry]\nType=Application\nName={id}\nExec={id}\nCategories={cats}\n")).unwrap();
        Entry { id: id.into(), name: id.into(), icon: None, exec: id.into(), try_exec: None, wm_class: String::new(),
                program: id.into(), kinds: Vec::new(), path, category: "system", keywords: Vec::new(),
                flatpak: None, appimage: None, terminal: false }
    };
    let all = vec![entry("aterm.desktop", "System;TerminalEmulator;"), entry("editor.desktop", "Utility;TextEditor;"),
                   entry("org.example.Zterm.desktop", "System;TerminalEmulator;")];
    let ids = |v: Vec<Entry>| v.into_iter().map(|e| e.id).collect::<Vec<_>>();
    assert_eq!(ids(terminals(&all)), ["aterm.desktop", "org.example.Zterm.desktop"]);
    let pick = |lists: &[&str]| chosen_terminal(&terminals(&all), &lists.iter().map(|s| s.to_string()).collect::<Vec<_>>()).map(|e| e.id);
    assert_eq!(pick(&["# mine\n-aterm.desktop\ngone.desktop\n+org.example.Zterm.desktop:new-window\n"]).as_deref(), Some("org.example.Zterm.desktop"));
    assert_eq!(pick(&["", "aterm"]).as_deref(), Some("aterm.desktop"));
    assert_eq!(pick(&["editor.desktop"]), None);
    assert_eq!(pick(&[]), None);
    assert!(terminals(&all[1..2]).is_empty());
    let own = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nExec=zterm\nX-TerminalArgExec=--\n");
    let plain = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nExec=aterm\n");
    let cmd = ["htop".to_string()];
    assert_eq!(with_command(vec!["zterm".into()], &own, &cmd), ["zterm", "--", "htop"]);
    assert_eq!(with_command(vec!["aterm".into()], &plain, &cmd), ["aterm", "-e", "htop"]);
    assert_eq!(with_command(vec!["aterm".into()], &plain, &[]), ["aterm"]);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_desktops_own_list_of_defaults_comes_first() {
    let lists = lists_in(&[PathBuf::from("/home/john/.config"), PathBuf::from("/usr/share/applications")], &["trindle".into()]);
    assert_eq!(lists, [
        PathBuf::from("/home/john/.config/trindle-mimeapps.list"), PathBuf::from("/home/john/.config/mimeapps.list"),
        PathBuf::from("/usr/share/applications/trindle-mimeapps.list"), PathBuf::from("/usr/share/applications/mimeapps.list"),
    ]);
}

#[test]
fn links_are_told_from_paths() {
    assert!(is_link("mailto:john@example.com?subject=Hi"));
    assert!(is_link("https://example.com/a"));
    assert!(!is_link("/home/john/notes:1.txt"));
    assert!(!is_link("notes.txt"));
    assert!(!is_link("1x:y"));
    let file = super::super::keyfile::KeyFile::parse("[Desktop Entry]\nExec=mail %u\n");
    let (argv, _) = command_with(&file, Path::new("/x.desktop"), &["mailto:john@example.com".into()]).expect("command");
    assert_eq!(argv, vec!["mail".to_string(), "mailto:john@example.com".into()]);
}

#[test]
fn an_app_with_no_icon_gets_the_same_generic_one_every_time() {
    assert_eq!(GENERIC_ICONS.len(), 20);
    let distinct: std::collections::HashSet<_> = GENERIC_ICONS.iter().collect();
    assert_eq!(distinct.len(), 20, "the twenty generic icons must differ");
    for (n, name) in GENERIC_ICONS.iter().enumerate() {
        assert_eq!(*name, format!("trindle-generic-{:02}", n + 1));
    }
    assert_eq!(generic_icon("org.x.Y"), generic_icon("org.x.Y.desktop"));
    assert_eq!(generic_icon("Firefox"), generic_icon("firefox"));
    assert_eq!(generic_icon(""), GENERIC_ICONS[(0x811c_9dc5u32 % 20) as usize]);
    assert_eq!(generic_icon("a"), GENERIC_ICONS[((0x811c_9dc5u32 ^ 0x61).wrapping_mul(0x0100_0193) % 20) as usize]);
    let used: std::collections::HashSet<_> = (0..400).map(|n| generic_icon(&format!("org.example.App{n}"))).collect();
    assert_eq!(used.len(), 20, "a spread of names uses all twenty");
}

#[test]
fn every_place_that_shows_an_app_goes_through_the_one_function() {
    fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() { rust_files(&p, out) } else if p.extension().is_some_and(|x| x == "rs") { out.push(p) }
        }
    }
    let mut files = Vec::new();
    for (src, _) in crate::common::source::trees() {
        rust_files(&src, &mut files);
    }
    let entry = Entry { id: "org.x.Y.desktop".into(), name: "Y".into(), icon: Some("y".into()), exec: "y".into(), try_exec: None,
        wm_class: String::new(), program: "y".into(), kinds: Vec::new(), path: PathBuf::new(), category: "", keywords: Vec::new(),
        flatpak: None, appimage: None, terminal: false };
    assert_eq!(entry.icons().last().map(String::as_str), Some(generic_icon_for("org.x.Y", "Y").as_str()));
    assert_eq!(generic_icon_for("org.x.Y", "Why\tnot"), format!("{}?org.x.Y\tWhy not", generic_icon("org.x.Y")));
    for p in files {
        let text = std::fs::read_to_string(&p).unwrap();
        let code = text.split("#[cfg(test)]").next().unwrap();
        for (n, line) in code.lines().enumerate() {
            let t = line.trim_start();
            if t.starts_with("//") || p.ends_with("common/apps.rs") { continue; }
            assert!(!line.contains("\"application-x-executable\"") || line.contains("defaults_to"),
                "{}:{}: the old stand-in icon must not be shown for a program", p.display(), n + 1);
            assert!(!line.contains("generic_icon("),
                "{}:{}: an app's stand-in icon must come from common::apps::generic_icon_for, which puts the program on the list of those that need an icon", p.display(), n + 1);
        }
    }
}

#[test]
fn launcher_files_are_read_without_gio() {
    use crate::common::keyfile::KeyFile;
    let file = |text: &str| KeyFile::parse(&format!("[Desktop Entry]\nType=Application\n{text}"));
    let langs = locale_names("en_GB.UTF-8");
    assert_eq!(langs, ["en_GB", "en"]);
    assert_eq!(locale_names("sr_RS@latin"), ["sr_RS@latin", "sr_RS", "sr@latin", "sr"]);
    assert!(locale_names("C.UTF-8").is_empty());
    let desk = vec!["Trindle".to_string()];
    let read = |text: &str| entry_from("org.x.Y.desktop", Path::new("/x"), &file(text), &langs, &desk, |p| p == "there");
    let e = read("Name=Color\nName[en_GB]=Colour\nExec=env A=1 /usr/bin/Ycolour %U\nIcon=y\nStartupWMClass=YC").unwrap();
    assert_eq!((e.name.as_str(), e.icon.as_deref(), e.program.as_str(), e.wm_class.as_str(), e.key()),
               ("Colour", Some("y"), "ycolour", "yc", "org.x.Y"));
    assert_eq!(e.icons(), ["trindle-app-org.x.Y".to_string(), "y".to_string(), generic_icon_for("org.x.Y", "Colour")]);
    let old = read("Name=Old\nExec=old\nIcon=application-x-executable-symbolic").unwrap();
    assert_eq!(old.icon, None, "a launcher that names one of the old stand-ins has no icon of its own");
    assert_eq!(read("Name=Old\nExec=old\nIcon=trindle-generic-03").unwrap().icon, None);
    assert!(read("Name=A\nExec=a\nNoDisplay=true").is_none());
    assert!(read("Name=A\nExec=a\nHidden=true").is_none());
    assert!(read("Name=A\nExec=a\nOnlyShowIn=KDE;").is_none());
    assert!(read("Name=A\nExec=a\nOnlyShowIn=KDE;trindle;").is_some());
    assert!(read("Name=A\nExec=a\nNotShowIn=Trindle;").is_none());
    assert!(read("Name=A\nExec=a\nTryExec=gone").is_none());
    assert!(on_path("/usr/bin/env") && !on_path("/usr/bin/trindle-test-not-there"));
    assert!(read("Name=A\nExec=a\nTryExec=there").is_some());
    assert!(read("Name=A").is_none());
    assert!(entry_from("l.desktop", Path::new("/x"), &KeyFile::parse("[Desktop Entry]\nType=Link\nName=L\nExec=a"), &langs, &desk, |_| true).is_none());
}

#[test]
fn a_program_that_may_be_on_a_share_is_looked_for_on_a_thread_and_is_missing_until_seen() {
    use std::time::{Duration, Instant};
    assert!(!on_path("/mnt/share/trindle-test-not-there") && !on_path("/home/sam/Applications/trindle-test-not-there"));
    let dir = std::env::temp_dir().join(format!("trindle-apps-far-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let program = dir.join("program");
    std::fs::write(&program, "").unwrap();
    let here = program.to_str().unwrap();
    assert!(!crate::common::files::own_place(Path::new(here)));
    assert!(on_path(here), "a program that is there was not found");
    let _ = std::fs::remove_dir_all(&dir);
    fn slow(_: &Path) -> bool {
        std::thread::sleep(Duration::from_millis(400));
        true
    }
    let share = Path::new("/mnt/share/trindle-test-slow");
    let since = Instant::now();
    assert!(!far_there(share, Duration::from_millis(20), slow));
    assert!(since.elapsed() < Duration::from_millis(300), "the list of apps waited on a share");
    assert!(!far_there(share, Duration::from_secs(5), |_| panic!("looked twice")));
    assert!(since.elapsed() < Duration::from_millis(300));
    let back = || matches!(fars().lock().unwrap().known.get(share), Some(Far::Seen(true, _)));
    while !back() && since.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(far_there(share, Duration::from_millis(20), |_| panic!("looked again while the answer was fresh")));
    assert!(fars().lock().unwrap().late.take().is_some());
    let held: Vec<_> = std::iter::from_fn(|| FAR_LOOKS.take()).collect();
    assert!((1..=8).contains(&held.len()), "{} looks may be out at once", held.len());
    assert!(!far_there(Path::new("/mnt/share/trindle-test-one-more"), Duration::from_secs(5), |_| true));
    drop(held);
    assert!(far_there(Path::new("/mnt/share/trindle-test-one-more"), Duration::from_secs(5), |_| true));
}

#[test]
fn a_boxed_package_is_known_by_its_own_program() {
    assert_eq!(program_from(None, "trindle-boxed alacritty alacritty --working-directory /tmp"), "alacritty");
    assert_eq!(program_from(None, "env A=1 /usr/bin/gimp %U"), "gimp");
}

#[test]
fn a_program_folders_starts_is_boxed_unless_it_already_is() {
    let app = |id: &str, exec: &str, flatpak: Option<&str>, appimage: bool, terminal: bool| Entry {
        id: format!("{id}.desktop"), name: id.into(), icon: None, exec: exec.into(), try_exec: None,
        wm_class: String::new(), program: program_from(flatpak, exec), kinds: Vec::new(),
        path: PathBuf::new(), category: "utilities", keywords: Vec::new(), flatpak: flatpak.map(str::to_string),
        appimage: appimage.then(|| "/home/john/.appimage/x.AppImage|1".to_string()), terminal,
    };
    let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let gimp = app("gimp", "gimp %U", None, false, false);
    assert_eq!(loose_argv(&gimp, &words(&["gimp", "/home/john/a.png"])).unwrap(), words(&["trindle-boxed", "--loose", "gimp", "gimp", "/home/john/a.png"]));
    assert!(loose_argv(&gimp, &words(&["env", "A=1", "gimp"])).is_some(), "a wrapper is boxed with the program");
    for (entry, argv) in [
        (app("org.gimp.GIMP", "flatpak run org.gimp.GIMP", Some("org.gimp.GIMP"), false, false), words(&["flatpak", "run", "org.gimp.GIMP"])),
        (app("x", "trindle-boxed --appimage /a %U", None, true, false), words(&["trindle-boxed", "--appimage", "/a"])),
        (app("alacritty", "trindle-boxed alacritty alacritty", None, false, false), words(&["trindle-boxed", "alacritty", "alacritty"])),
        (app("trindle-text", "trindle-text %F", None, false, false), words(&["trindle-text"])),
        (app("htop", "htop", None, false, true), words(&["htop"])),
        (app("other", "/usr/bin/trindle-thing", None, false, false), words(&["/usr/bin/trindle-thing"])),
    ] {
        assert!(loose_argv(&entry, &argv).is_none(), "{} was boxed twice or boxed when it is Trindle's", entry.id);
    }
    assert_eq!(program_from(None, "trindle-boxed --appimage \"/home/john/.appimage/Krita-5-x86_64.AppImage\" %U"), "krita-5-x86_64.appimage");
    assert_eq!(program_from(None, "trindle-boxed --loose gimp env A=1 /usr/bin/gimp %U"), "gimp");
}

#[test]
fn windows_find_their_app() {
    let app = |id: &str, exec: &str, flatpak: Option<&str>, class: &str| Entry {
        id: format!("{id}.desktop"), name: id.into(), icon: None, exec: exec.into(), try_exec: None,
        wm_class: class.to_lowercase(), program: program_from(flatpak, exec), kinds: Vec::new(),
        path: PathBuf::new(), category: "utilities", keywords: Vec::new(), flatpak: flatpak.map(str::to_string),
        appimage: None, terminal: false,
    };
    let apps = vec![
        app("firefox-esr", "/usr/lib/firefox-esr/firefox-esr %u", None, ""),
        app("com.synology.SynologyDrive", "/usr/bin/flatpak run com.synology.SynologyDrive", Some("com.synology.SynologyDrive"), ""),
        app("claude-desktop", "env FOO=1 /opt/Claude/claude %U", None, ""),
        app("google-chrome", "/usr/bin/google-chrome-stable %U", None, "Google-chrome"),
    ];
    let found = |w: &str| find_for_window(&apps, w).map(|a| a.key().to_string());
    assert_eq!(found("firefox-esr").as_deref(), Some("firefox-esr"));
    assert_eq!(found("Google-chrome").as_deref(), Some("google-chrome"));
    assert_eq!(found("google-chrome-stable").as_deref(), Some("google-chrome"));
    assert_eq!(found("Claude").as_deref(), Some("claude-desktop"));
    assert_eq!(found("SynologyDrive").as_deref(), Some("com.synology.SynologyDrive"));
    assert_eq!(found("com.synology.SynologyDrive.desktop").as_deref(), Some("com.synology.SynologyDrive"));
    assert_eq!(found(""), None);
    assert_eq!(found("gimp"), None);
}

fn made_up_apps() -> Vec<Entry> {
    let app = |id: &str, exec: &str, flatpak: Option<&str>, class: &str| Entry {
        id: format!("{id}.desktop"), name: id.into(), icon: None, exec: exec.into(), try_exec: None,
        wm_class: class.to_lowercase(), program: program_from(flatpak, exec), kinds: Vec::new(),
        path: PathBuf::new(), category: "utilities", keywords: Vec::new(), flatpak: flatpak.map(str::to_string),
        appimage: None, terminal: false,
    };
    vec![
        app("trindle-settings", "trindle-settings", None, "org.trindle.Settings"),
        app("trindle-apps", "trindle-apps", None, "org.trindle.Store"),
        app("trindle-deb", "trindle-apps --deb %f", None, "org.trindle.Store"),
        app("trindle-appimage", "trindle-apps --appimage %f", None, "org.trindle.Store"),
        app("trindle-folders", "trindle-folders %U", None, "org.trindle.Files"),
        app("org.mozilla.firefox", "/usr/bin/flatpak run --branch=stable org.mozilla.firefox @@u %u @@",
            Some("org.mozilla.firefox"), ""),
        app("com.discordapp.Discord", "/usr/bin/flatpak run com.discordapp.Discord", Some("com.discordapp.Discord"), ""),
        app("firefox-esr", "/usr/lib/firefox-esr/firefox-esr %u", None, ""),
        app("gimp", "gimp-2.10 %U", None, ""),
        app("trindle-appimage-obsidian-1-6-7-x86-64-appimage",
            "\"/home/john/.appimage/Obsidian-1.6.7-x86_64.AppImage\" %U", None, ""),
        app("trindle-appimage-krita-5-2-3-x86-64-appimage",
            "\"/home/john/Applications/krita-5.2.3-x86_64.appimage\" %U", None, "org.kde.krita"),
        app("google-chrome", "/usr/bin/google-chrome-stable %U", None, "Google-chrome"),
        app("steam", "/usr/games/steam %U", None, "steam"),
        app("code", "/usr/share/code/code %F", None, "Code"),
        app("brave-browser", "/usr/bin/brave-browser-stable %U", None, "brave-browser"),
        app("vivaldi-stable", "/usr/bin/vivaldi-stable %U", None, "vivaldi-stable"),
        app("microsoft-edge", "/usr/bin/microsoft-edge-stable %U", None, "microsoft-edge"),
        app("chrome-abcdefghijklmnop-Default", "/opt/google/chrome/google-chrome --app-id=abcdefghijklmnop", None,
            "crx_abcdefghijklmnop"),
        app("foo-launcher", "/opt/foo/foo.sh %U", None, ""),
    ]
}

fn window_cases() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("org.trindle.Settings", &["trindle-settings"]),
        ("org.trindle.Store", &["trindle-apps", "trindle-deb", "trindle-appimage"]),
        ("org.trindle.Apps", &["trindle-apps", "trindle-deb", "trindle-appimage"]),
        ("org.trindle.Files", &["trindle-folders"]),
        ("org.trindle.Folders", &["trindle-folders"]),
        ("org.mozilla.firefox", &["org.mozilla.firefox"]),
        ("discord", &["com.discordapp.Discord"]),
        ("firefox-esr", &["firefox-esr"]),
        ("gimp-2.10", &["gimp"]),
        ("Gimp", &["gimp"]),
        ("obsidian", &["trindle-appimage-obsidian-1-6-7-x86-64-appimage"]),
        ("org.kde.krita", &["trindle-appimage-krita-5-2-3-x86-64-appimage"]),
        ("Google-chrome", &["google-chrome"]),
        ("google-chrome", &["google-chrome"]),
        ("steam", &["steam"]),
        ("Code", &["code"]),
        ("code", &["code"]),
        ("brave-browser", &["brave-browser"]),
        ("vivaldi-stable", &["vivaldi-stable"]),
        ("microsoft-edge", &["microsoft-edge"]),
        ("chrome-abcdefghijklmnop-Default", &["chrome-abcdefghijklmnop-Default"]),
        ("crx_abcdefghijklmnop", &["chrome-abcdefghijklmnop-Default"]),
        ("foo", &["foo-launcher"]),
        ("", &[]),
        ("some-other-program", &[]),
    ]
}

#[test]
fn every_kind_of_app_owns_its_windows() {
    let apps = made_up_apps();
    let owners = |window: &str| -> Vec<String> {
        apps.iter().filter(|a| owns(&apps, a, window)).map(|a| a.key().to_string()).collect()
    };
    for (window, want) in window_cases() {
        assert_eq!(owners(window), want.iter().map(|s| s.to_string()).collect::<Vec<_>>(), "window {window:?}");
        assert_eq!(find_for_window(&apps, window).map(|a| a.key()), want.first().copied(), "window {window:?}");
    }
}

#[test]
fn trindle_launchers_own_their_windows() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("share/applications");
    let desk = vec!["Trindle".to_string()];
    let ids = ["trindle-activity", "trindle-apps", "trindle-drives", "trindle-settings",
               "trindle-text", "trindle-welcome"];
    let all: Vec<Entry> = ids.iter().map(|id| {
        let path = dir.join(format!("{id}.desktop"));
        let file = crate::common::keyfile::KeyFile::load(&path).unwrap();
        entry_from(&format!("{id}.desktop"), &path, &file, &[], &desk, |_| true).unwrap()
    }).collect();
    for (id, window) in [("trindle-activity", "org.trindle.Activity"), ("trindle-apps", "org.trindle.Apps"),
                         ("trindle-apps", "org.trindle.Store"), ("trindle-drives", "org.trindle.Drives"),
                         ("trindle-settings", "org.trindle.Settings"), ("trindle-text", "org.trindle.Text"),
                         ("trindle-welcome", "org.trindle.Welcome")] {
        let app = all.iter().find(|a| a.key() == id).unwrap();
        assert!(owns(&all, app, window), "{id} owns {window}");
    }
}

#[test]
fn a_launcher_command_drops_its_file_codes() {
    let file = crate::common::keyfile::KeyFile::parse(
        "[Desktop Entry]\nName=Paint\nIcon=paint\nPath=/tmp\nExec=env A=1 \"/opt/my paint/paint\" %U --title=%c %i 100%%\n");
    let (argv, folder) = command_of(&file, Path::new("/x/paint.desktop")).unwrap();
    assert_eq!(argv, ["env", "A=1", "/opt/my paint/paint", "--title=Paint", "--icon", "paint", "100%"]);
    assert_eq!(folder.as_deref(), Some(Path::new("/tmp")));
    let none = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nName=Nothing\nExec=%f\n");
    assert_eq!(command_of(&none, Path::new("/x")), None);
}

#[test]
fn a_launcher_file_starts_its_program_in_its_folder() {
    let dir = std::env::temp_dir().join(format!("trindle-start-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let launcher = dir.join("touch.desktop");
    std::fs::write(&launcher, format!("[Desktop Entry]\nName=Touch\nPath={}\nExec=touch started %F\n", dir.display())).unwrap();
    start_file(&launcher).unwrap();
    let started = dir.join("started");
    for _ in 0..50 {
        if started.exists() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
    assert!(started.exists());
    std::fs::remove_dir_all(&dir).unwrap();
}
