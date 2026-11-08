//! Every Trindle program's window id is the id the ring looks for. The ring
//! raises a program's window by matching the window's id against its
//! launcher (`StartupWMClass`), so the launcher files, the ids the programs
//! give their windows and the table in `apps::OWN` must all say one thing.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::apps::{self, Entry, OWN};
use super::keyfile::KeyFile;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn launchers() -> Vec<(String, KeyFile)> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(root().join("share/applications")).unwrap().flatten() {
        let p = e.path();
        let stem = p.file_stem().unwrap().to_string_lossy().into_owned();
        if stem.starts_with("trindle-") && p.extension().is_some_and(|x| x == "desktop") {
            out.push((stem, KeyFile::load(&p).unwrap()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

const NO_WINDOW: [&str; 1] = ["trindle-screenshot"];

const NOT_FROM_THE_RING: [&str; 9] = [
    "org.trindle.greeter", "org.trindle.installer", "org.trindle.filechooser", "org.trindle.portal", "org.trindle.test",
    "trindle-keyring", "trindle-mcp", "trindle-locked", "trindle-pin",
];

fn window_ids() -> BTreeSet<String> {
    let mut files = Vec::new();
    for (src, _) in crate::common::source::trees() {
        sources(&src, &mut files);
    }
    let mut ids = BTreeSet::new();

    for f in files.into_iter().filter(|f| !f.ends_with("own_tests.rs")) {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        for (at, _) in text.match_indices("app_id: ") {
            let rest = &text[at + 8..];
            if let Some(lit) = rest.strip_prefix('"') {
                if let Some(end) = lit.find('"') {
                    ids.insert(lit[..end].to_lowercase());
                }
            } else if rest.starts_with("NAME") || rest.starts_with("super::NAME") || rest.starts_with("APP_ID") {
                let wanted = if rest.starts_with("APP_ID") { "const APP_ID: &str = \"" } else { "const NAME: &str = \"" };
                let pick = |t: &str| t.find(wanted).and_then(|i| t[i + wanted.len()..].split('"').next().map(str::to_lowercase));

                let found = pick(&text).or_else(|| f.parent().and_then(|d| std::fs::read_to_string(d.join("mod.rs")).ok()).and_then(|t| pick(&t)));
                ids.extend(found);
            }
        }
    }
    ids
}

fn fake(id: &str, wm: &str, exec: &str) -> Entry {
    Entry {
        id: format!("{id}.desktop"), name: id.into(), icon: None, exec: exec.into(), try_exec: None, wm_class: wm.to_lowercase(),
        program: exec.split(' ').next().unwrap_or("").into(), kinds: Vec::new(), path: PathBuf::from(id), category: "system",
        keywords: Vec::new(), flatpak: None, appimage: None, terminal: false,
    }
}

#[test]
fn every_launcher_names_the_window_its_program_opens_and_the_table_knows_it() {
    let ids = window_ids();
    assert!(ids.contains("org.trindle.settings") && ids.contains("org.trindle.store"), "the scan finds windows: {ids:?}");
    for (stem, file) in launchers() {
        if NO_WINDOW.contains(&stem.as_str()) {
            continue;
        }
        let own = apps::own(&stem).unwrap_or_else(|| panic!("{stem}.desktop is not in apps::OWN, so the ring cannot raise its window"));
        let class = file.string("Desktop Entry", "StartupWMClass").unwrap_or_else(|| panic!("{stem}.desktop has no StartupWMClass"));
        assert!(own.knows(&class), "{stem}: the launcher's window class {class} is not one of {}'s window ids", own.key);
        assert!(ids.contains(&class.to_lowercase()), "{stem}: no program gives its window the id {class}; the ring would never find it");
        let command = file.string("Desktop Entry", "Exec").unwrap().split(' ').next().unwrap().to_string();
        assert!(own.knows(&command), "{stem}: the command {command} is not one of {}'s commands", own.key);
    }
}

#[test]
fn every_window_id_in_the_source_is_a_program_the_ring_knows_or_one_it_never_starts() {
    for id in window_ids() {

        if !(id.starts_with("org.trindle.") || id.starts_with("trindle-")) || NOT_FROM_THE_RING.contains(&id.as_str()) {
            continue;
        }
        assert!(apps::own(&id).is_some(), "a window is given the id {id}, which is in neither apps::OWN nor the list of those the ring never starts");
    }
}

const ENDS: [(&str, Option<&str>, &str); 60] = [
    ("toolkit/window/stock_tests.rs", None, "a test stopping its own drawing service to see the window carry on"),
    ("toolkit/paint/service/tests.rs", None, "a test stopping its own drawing service to see what programs do then"),
    ("toolkit/paint/service.rs", None, "a test's own drawing service, stopped when the test is over"),
    ("common/restart.rs", None, "the keys that restart the desktop end the ring and the helper parts, found by their own names among the person's own programs, and start them again; no window's program is among them, and only the compositor's own key or the person's press in Settings reaches it: not an act on the list"),
    ("session/restart_ask.rs", None, "pressing Restart in the question ends the parts and the compositor, which the session starts again; the window is the permissions panel, which no made-up press reaches, and nothing happens without the press: not an act on the list"),
    ("ring/sni/menu.rs", None, "Close on a tray icon's menu asks the program that owns the icon to end with a polite stop, at the person's own press; the process is the one the bus names for that icon, never init or Trindle, and the kernel refuses any not the person's: not an act on the list"),
    ("plugins/background/tests.rs", None, "a test ends the made-up keeper it started, and the sleeping program of its own that stands for a plugin's"),
    ("plugins/pin.rs", None, "the PIN box ends its own command, started a moment before, when it has not answered in 30 seconds: not an act on the list"),
    ("store/fence_broker/tests.rs", None, "a test stops the sshd of its own that it started on the loopback to try the real ssh"),
    ("store/fence/tests.rs", None, "a test ends the sleeping program of its own that it made to stand outside the fence"),
    ("common/cups.rs", None, "its own lp, started to send one document, cut off when CUPS takes too long"),
    ("activity/procs.rs", Some(crate::common::person_only::STOP), "Stop in Activity ends the person's own program"),
    ("installer/steps.rs", None, "Stop in the installer ends its own copy of the system"),
    ("compose/panel.rs", Some(crate::common::person_only::FORCE_CLOSE), "the compositor ends a program when the ring's windows ask twice: Force close"),
    ("common/compositor.rs", Some(crate::common::person_only::RING_POWER), "ending the session: from the ring by its power choices, and from Settings' Power page by its own act (settings-power)"),
    ("store/flat.rs", None, "putting a backup back ends a running app to restore its settings: not an act on the list, and said to the coordinator"),
    ("ring/meter.rs", None, "its own pw-record, started to read the microphone level while the Sound page is open, ended when the page closes: not an act on the list"),
    ("common/removal.rs", None, "removing or reinstalling a boxed program or AppImage first ends its own running copies, found by the trindle-boxed name in their command line, so two are never open: the program is the one the person has just asked to remove or reinstall, in a press already guarded as the removal; not a separate act on the list"),
    ("plugins/background.rs", None, "turning a plugin off or removing it ends its own background part: not an act on the list, and said to the coordinator"),
    ("voice/commands.rs", None, "the spoken word kill closes a window as its close button does; no press, and nothing is ended"),
    ("ring/tray.rs", None, "a signal that shows or hides the on-screen keyboard; nothing is ended"),
    ("controller/tests.rs", None, "a test starts a child program and ends it (kill) to check a program that has gone is found gone; nothing of the person's is ended"),
    ("common/crash.rs", None, "reads how a program ended; ends none"),
    ("common/dconf.rs", None, "its own helper"),
    ("common/keyring_said.rs", None, "a test's own helper"),
    ("common/run.rs", None, "a program this one started, when it outstays its time"),
    ("common/stopping.rs", None, "Stop on an update that is running ends the update Apps itself started a moment before, in a process group of its own, and what it started; never another of the person's programs, and the kernel refuses anything not the person's. A stop is undone by pressing Update again: not an act on the list"),
    ("common/updating.rs", None, "a test's own helper"),
    ("compose/indicators.rs", None, "the compositor's own part"),
    ("compose/own_ring/keeping.rs", None, "the ring the compositor keeps, when it stops answering"),
    ("compose/own_ring/shot.rs", None, "the screenshot tool the compositor starts for the person's key, when it stops answering"),
    ("compose/rig/kept_ring.rs", None, "the compositor's tests"),
    ("compose/rig/mod.rs", None, "the compositor's tests"),
    ("compose/rig/tests/ring_life.rs", None, "the compositor's tests"),
    ("compose/reader_keys.rs", None, "a test ends the sleeping program of its own that it made"),
    ("compose/xrooms.rs", None, "only its tests end the stand-in program they started"),
    ("store/home_net.rs", None, "the network room of one plugin box: its own keeper and pasta, started for that box and ended with it"),
    ("store/home_net/tests.rs", None, "a test's own made-up house"),
    ("greeter/mod.rs", None, "the login screen's own children, before any session"),
    ("icons/appimages.rs", None, "its own helper"),
    ("keyring/migrate.rs", None, "the old keyring's daemon, as Trindle's takes over at login"),
    ("keyring/pam.rs", None, "its own helper"),
    ("polkit/mod.rs", None, "the helper that checks the administrator's password; the question is on the list"),
    ("ring/quick/bluetooth.rs", None, "its own helper"),
    ("screenshot/colours.rs", None, "its own helper, stopped when it takes too long"),
    ("screenshot/portal.rs", None, "its own helper"),
    ("screenshot/portal_files.rs", None, "its own Folders chooser, stopped when the program takes its request back"),
    ("screenshot/pw/tests.rs", None, "a hand-run test's own pw-link"),
    ("session/idle.rs", None, "Trindle's own background part, started afresh"),
    ("session/moving.rs", None, "the boxed decoder the wallpaper started"),
    ("session/moving/tests.rs", None, "the same decoder, in the tests"),
    ("session/nightlight.rs", None, "Trindle's own background part, started afresh"),
    ("session/screensaver.rs", None, "Trindle's own screensaver"),
    ("session/start.rs", None, "only asks whether the compositor is there"),
    ("session/wallpaper.rs", None, "Trindle's own background part"),
    ("settings/panels/audio.rs", None, "the sound test Settings started"),
    ("plugins/rows.rs", None, "ends the pkexec it started when it has not finished in two minutes"),
    ("text/mod.rs", None, "hears its own signals"),
    ("toolkit/bus.rs", None, "a test's own bus"),
    ("toolkit/sharp.rs", None, "its own fontconfig helper, stopped when it takes too long"),
];

#[test]
fn every_act_only_the_person_may_do_is_on_the_one_list_and_guarded() {
    use crate::common::person_only::{ACTS, WAITING};
    let src = root().join("src");
    let read = |file: &str| std::fs::read_to_string(crate::common::source::path_of(file)).unwrap_or_else(|e| panic!("{file}: {e}"));
    let mut names = BTreeSet::new();
    for (act, file, guard) in ACTS {
        assert!(names.insert(act), "{act} is on the list twice");
        assert!(read(file).contains(guard), "{act} is on the list of what only the person may do, and {file} no longer guards it with {guard}");
    }

    for (file, pressed) in crate::common::person_only::INSTALL_PRESSED {
        assert!(read(file).contains(pressed), "{file} no longer has Install pressed through the guard: {pressed}");
    }
    let mut store = Vec::new();
    sources(&src.join("store"), &mut store);
    for file in store {
        let text = std::fs::read_to_string(&file).unwrap();
        for bare in ["go(move |s| pages::install::install(", "go(install_boxed)", "\"Install\", move |s| install_", "move |s| install(s, &o)"] {
            assert!(!text.contains(bare), "{}: Install is pressed past the guard, as {bare}", file.display());
        }
    }
    for (act, file, line) in WAITING {
        assert!(names.insert(act), "{act} is on the list twice, or on both");
        assert!(read(file).contains(line), "{act} waits for the owner, and the line its guard would go before is no longer in {file}: {line}");
    }

    let words = [["libc::", "kill("].concat(), ["SIG", "TERM"].concat(), ["SIG", "KILL"].concat(), [".", "kill()"].concat(),
                 ["\"", "kill\""].concat(), ["\"p", "kill\""].concat(), ["\"ki", "llall\""].concat(), ["pidfd_send", "_signal"].concat()];
    let mut found = BTreeSet::new();
    for (tree, prefix) in crate::common::source::trees() {
        let mut files = Vec::new();
        sources(&tree, &mut files);
        for file in files {
            let text = std::fs::read_to_string(&file).unwrap();
            if words.iter().any(|w| text.contains(w.as_str())) {
                found.insert(format!("{prefix}{}", file.strip_prefix(&tree).unwrap().display()));
            }
        }
    }
    assert!(found.contains("activity/procs.rs") && found.len() > 20, "the reading of the source finds them: {found:?}");
    for file in &found {

        if file.starts_with("unlock/") {
            continue;
        }
        assert!(ENDS.iter().any(|e| e.0 == file), "{file} ends or signals a program. Say what it is in ENDS: an act only the person may do goes on the list in common/person_only.rs and is guarded; anything else says why it is not one");
    }
    for (file, act, why) in ENDS {
        assert!(found.contains(file), "{file} is said to end or signal a program and no longer does");
        assert!(!why.is_empty());
        if let Some(act) = act {
            assert!(ACTS.iter().any(|a| a.0 == act), "{file} is put down to {act}, which is not on the list");
        }
    }

    for act in [crate::common::person_only::STOP, crate::common::person_only::FORCE_CLOSE] {
        assert!(ENDS.iter().any(|e| e.1 == Some(act)), "{act} is on the list and no file is put down to it");
    }
}

#[test]
fn the_table_is_whole_and_each_program_is_in_it_once() {
    let mut seen = BTreeSet::new();
    for o in OWN {
        assert!(!o.desktop.is_empty() && !o.window.is_empty() && !o.command.is_empty(), "{} has every kind of name", o.key);
        let mine: BTreeSet<&str> = o.names().collect();
        for n in mine {
            assert_eq!(n, n.to_lowercase(), "names are kept in lower case: {n}");
            assert!(seen.insert(n), "{n} belongs to two programs in apps::OWN");
        }
    }
}

#[test]
fn a_window_is_found_under_the_old_name_or_the_new() {

    let launchers = vec![fake("trindle-files", "org.trindle.Files", "trindle-files %U"), fake("trindle-apps", "org.trindle.Apps", "trindle-apps")];
    let key = |id: &str| apps::find_for_window(&launchers, id).map(|e| e.key().to_string());
    assert_eq!(key("org.trindle.Files").as_deref(), Some("trindle-files"));
    assert_eq!(key("org.trindle.Folders").as_deref(), Some("trindle-files"), "a Folders window finds the Files launcher");
    assert_eq!(key("trindle-folders").as_deref(), Some("trindle-files"));
    assert_eq!(key("org.trindle.Store").as_deref(), Some("trindle-apps"), "a Store window finds the Apps launcher");
    assert_eq!(key("org.trindle.Apps").as_deref(), Some("trindle-apps"));
    let launchers = vec![fake("trindle-folders", "org.trindle.Folders", "trindle-folders %U"), fake("trindle-store", "org.trindle.Store", "trindle-store")];
    let key = |id: &str| apps::find_for_window(&launchers, id).map(|e| e.key().to_string());
    assert_eq!(key("org.trindle.Files").as_deref(), Some("trindle-folders"));
    assert_eq!(key("org.trindle.Apps").as_deref(), Some("trindle-store"));

    assert_eq!(key("org.mozilla.firefox"), None);
    assert!(apps::same_name("TRINDLE-FILES.desktop", "org.trindle.folders"));
    assert!(!apps::same_name("org.trindle.files", "org.trindle.text"));
}

const RING_ITEMS: &[(&str, Option<&str>)] = &[

    ("middle: Settings", Some("settings")),
    ("middle: Store (Apps)", Some("store")),
    ("middle: Activity", Some("activity")),
    ("lower tools: Report a problem", Some("report")),

    ("the time and date opens Settings at Date and time", Some("settings")),
    ("\"More in Settings\" in a panel", Some("settings")),
    ("the updates button opens the Store (Apps) at Updates", Some("store")),
    ("a voice command for the Store (Apps)", Some("store")),
    ("a voice command for Activity", Some("activity")),
    ("a mounted drive opens Folders on it", Some("files")),
    ("Settings at Drives opens Drives", Some("drives")),

    ("a favourite", None),
    ("an app in All apps", None),
    ("an app found by typing", None),
    ("an app named by voice", None),

    ("Wi-Fi, Bluetooth, sound and brightness panels", None),
    ("clipboard history and notifications panels", None),
    ("Power", None),
    ("the battery band opens the power panel in the ring", None),
    ("the magnifier", None),
    ("the screenshot button (the program opens its own window, and the ring goes)", None),
    ("keep awake", None),
    ("the on-screen keyboard", None),

    ("a tray icon (handed a pass to come forward with, then told to Activate)", None),
    ("a plugin's icon", None),
];

#[test]
fn every_ring_button_that_owns_a_window_raises_it_through_the_one_table() {
    let mut ring = String::new();
    let mut files = Vec::new();
    sources(&root().join("src/ring"), &mut files);
    for f in files {
        ring.push_str(&std::fs::read_to_string(f).unwrap());
    }
    let mut raised = BTreeSet::new();
    for (at, _) in ring.match_indices("open_own(\"") {
        if let Some(key) = ring[at + 10..].split('"').next() {
            raised.insert(key.to_string());
        }
    }

    for (at, _) in ring.match_indices("raise_key(\"") {
        if let Some(key) = ring[at + 11..].split('"').next() {
            raised.insert(key.to_string());
        }
    }
    for (item, owner) in RING_ITEMS {
        if let Some(key) = owner {
            assert!(apps::own_key(key).is_some(), "{item}: {key} is not a program in apps::OWN");
            assert!(raised.contains(*key), "{item}: the ring never calls open_own(\"{key}\"), so its window is not raised");
        }
    }

    for key in &raised {
        assert!(RING_ITEMS.iter().any(|(_, o)| o == &Some(key.as_str())), "the ring raises {key} but no item of the table says so");
    }

    for command in ["\"trindle-store\"", "\"trindle-activity\"", "\"trindle-files\"", "\"trindle-folders\""] {
        let wrote = ring.match_indices(command).filter(|(at, _)| ring[..*at].lines().last().is_some_and(|l| l.contains("spawn(") || l.contains("run(&["))).count();
        assert_eq!(wrote, 0, "the ring runs {command} directly, which would start a second window instead of raising the first");
    }
}

#[test]
fn the_ring_raises_the_newest_window_of_a_program_that_has_several() {
    struct W(&'static str, &'static str);
    let wins = [W("org.trindle.Files", "Home"), W("org.trindle.Text", "notes"), W("org.trindle.Files", "Copying"), W("org.trindle.Store", "Store")];
    let own = apps::own_key("files").unwrap();
    let newest = apps::newest_of(&wins, |w| w.0, own).map(|w| w.1);
    assert_eq!(newest, Some("Copying"), "the newest window of the program, not the first");
    let own = apps::own_key("store").unwrap();
    let renamed = [W("org.trindle.Apps", "Apps")];
    assert_eq!(apps::newest_of(&renamed, |w| w.0, own).map(|w| w.1), Some("Apps"), "a window under the new name is found as the program");
    assert_eq!(apps::newest_of(&wins, |w| w.0, apps::own_key("activity").unwrap()).map(|w| w.1), None);
}

fn names_the_app_files(words: &str) -> bool {
    words.split(|c: char| !c.is_alphanumeric()).any(|w| w == "Files")
}

const ARCHIVE_KINDS: [&str; 34] = [
    "application/zip", "application/x-7z-compressed", "application/x-tar", "application/x-compressed-tar",
    "application/x-bzip2-compressed-tar", "application/x-bzip-compressed-tar", "application/x-xz-compressed-tar",
    "application/x-zstd-compressed-tar", "application/x-lz4-compressed-tar", "application/x-lzma-compressed-tar",
    "application/x-lzip-compressed-tar", "application/x-tarz", "application/vnd.rar", "application/x-rar",
    "application/x-rar-compressed", "application/vnd.efi.iso", "application/x-cd-image", "application/x-iso9660-image",
    "application/vnd.ms-cab-compressed", "application/x-cpio", "application/x-rpm", "application/x-archive",
    "application/x-lha", "application/x-xar", "application/gzip", "application/x-gzip", "application/x-bzip2",
    "application/x-bzip", "application/x-xz", "application/zstd", "application/x-lz4", "application/x-lzma",
    "application/x-lzip", "application/x-compress",
];

#[test]
fn the_file_manager_is_folders_and_its_old_names_are_aliases() {
    let found = launchers();
    for (stem, file) in &found {
        for key in ["Name", "GenericName", "Comment"] {
            if let Some(words) = file.string("Desktop Entry", key) {
                assert!(!names_the_app_files(&words), "{stem}.desktop {key} calls a program Files: {words}");
            }
        }
    }

    assert!(!found.iter().any(|(s, _)| s == "trindle-files" || s == "trindle-store"), "no hidden old launcher is installed");

    let kinds: Vec<&str> = std::iter::once("inode/directory").chain(ARCHIVE_KINDS).collect();
    let defaults = std::fs::read_to_string(root().join("system/xdg/trindle-mimeapps.list")).unwrap();
    let mine: Vec<&str> = defaults.lines().filter_map(|l| l.strip_suffix("=trindle-folders.desktop")).collect();
    assert_eq!(mine, kinds, "Folders is the default for exactly what it opens");
    assert!(defaults.lines().any(|l| l == "application/vnd.debian.binary-package=trindle-deb.desktop"), "a .deb stays with the installer");
    assert!(!defaults.contains("FileRoller") && !defaults.contains("file-roller"), "no other archive program is named");

    let own = apps::own("trindle-files").expect("the old command is known");
    assert_eq!(apps::own("trindle-folders").map(|o| o.key), Some(own.key));
    assert_eq!(own.command[0], "trindle-folders", "the new command is the current one");
    assert_eq!(own.desktop[0], "trindle-folders");
    assert!(own.knows("trindle-files.desktop") && own.knows("org.trindle.Files") && own.knows("org.trindle.folders"));
}

#[test]
fn no_settings_page_or_help_topic_calls_the_file_manager_files() {
    use crate::settings::help_topics::{topics, Block};
    for page in crate::settings::panels::all() {
        assert!(!names_the_app_files(page.title()), "the Settings page {} is called Files", page.title());
        assert!(!names_the_app_files(page.help()), "the help of {}: {}", page.title(), page.help());
    }

    assert!(crate::settings::panels::all().iter().all(|p| p.id() != "files" && p.title() != "Folders"));
    for t in topics() {
        assert!(!names_the_app_files(t.title), "the Help topic {} is called Files", t.title);
        for b in t.blocks {
            let said: Vec<&str> = match b {
                Block::P(s) | Block::Tip(s) => vec![s],
                Block::Steps(v) => v.to_vec(),
                Block::Img(_, alt) => vec![alt],
            };
            for s in said {
                assert!(!names_the_app_files(s), "Help topic {}: {s}", t.title);
            }
        }
    }
    let topic = topics().iter().find(|t| t.id == "files").expect("the topic is still there by its id");
    assert!(topic.title.starts_with("Folders"));
    assert!(topic.keywords.split_whitespace().any(|k| k == "files"), "the topic is found by files");
}

#[test]
fn names_that_must_not_be_in_trindle_are_not_in_it() {
    let banned = [["cos", "mic"].concat(), ["ne", "mo"].concat(), ["cinn", "amon"].concat(), ["muf", "fin"].concat()];
    let community = ["build", "morphic"].concat();
    const COMMUNITY_FILES: [&str; 13] = ["system/identity/os-release", "README.md", "share/terms.md", "src/report/tests.rs", "share/lang/en/settings-words.ftl", "src/settings/panels/crash_reports.rs", "share/lang/en/settings-crash.ftl", "src/report/mod.rs", "src/report/send.rs", "src/report/tests_send.rs", "share/lang/en/report.ftl", "spec/trindle.md", "spec/features.md"];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut stack: Vec<std::path::PathBuf> = ["src", "crates", "spec", "docs", "tools", "share", "config", "packaging", "system", "tests", "README.md", "packages.txt", "build.rs", "Cargo.toml", "../trindle-site"]
        .iter().map(|d| root.join(d)).filter(|d| d.exists()).collect();
    let mut hits = Vec::new();
    crate::common::needs::program("git");
    let site = root.join("../trindle-site");
    let listed = std::process::Command::new("git").arg("-C").arg(&site).args(["ls-files", "-z"]).output();
    let tracked: std::collections::HashSet<std::path::PathBuf> = match listed {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).split('\0').filter(|f| !f.is_empty()).map(|f| site.join(f)).collect(),
        _ => Default::default(),
    };
    while let Some(p) = stack.pop() {
        if p.file_name().is_some_and(|n| n == ".git") { continue; }

        if p.starts_with(&site) && !tracked.is_empty() && !p.is_dir() && !tracked.contains(&p) { continue; }
        if p.is_dir() {
            stack.extend(std::fs::read_dir(&p).into_iter().flatten().flatten().map(|e| e.path()));
        } else if let Ok(text) = std::fs::read_to_string(&p) {
            let low = text.to_lowercase();
            for b in &banned {

                if low.replace("mnemonic", "").contains(b.as_str()) {
                    hits.push(format!("{} has {b}", p.display()));
                }
            }
            let whole = [["m", "int"].concat(), ["cos", "mo"].concat()];
            for w in low.split(|c: char| !c.is_alphanumeric()) {
                if whole.iter().any(|b| w == b) || w.starts_with(&["m", "intupdate"].concat()) || w.starts_with(&["m", "intinstall"].concat()) {
                    hits.push(format!("{} has the word {w}", p.display()));
                }
            }
            let rel = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
            let website = rel.starts_with("../trindle-site/") && rel.ends_with(".html");
            let marker = format!("data-{community}");
            let translated = rel.starts_with("share/lang/") && (rel.ends_with("/report.ftl") || rel.ends_with("/settings-crash.ftl") || rel.ends_with("/settings-words.ftl"));
            let stray = if COMMUNITY_FILES.contains(&rel.as_str()) || translated { false }
                else if website { low.lines().any(|l| l.contains(&community) && !l.contains(&marker)) }
                else { low.contains(&community) };
            if stray { hits.push(format!("{rel} has {community} outside its allowed places")); }
        }
    }
    assert!(hits.is_empty(), "{hits:?}");
}

#[test]
fn files_that_began_elsewhere_say_where_from() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<PathBuf> = Vec::new();
    let mut stack = vec![root.join("src/compose")];
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            stack.extend(std::fs::read_dir(&p).into_iter().flatten().flatten().map(|e| e.path()));
        } else if p.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&p).unwrap_or_default();
            let in_extras = p.starts_with(root.join("src/compose/drm_extras")) && !p.ends_with("leftovers.rs");
            if text.contains("AnvilState") || in_extras {
                files.push(p);
            }
        }
    }
    files.push(root.join("src/compose/main.rs"));
    assert!(files.len() >= 30, "found only {} files", files.len());
    let missing: Vec<String> = files.iter().filter(|f| {
        let text = std::fs::read_to_string(f).unwrap_or_default();
        !text.lines().take(12).any(|l| l.starts_with("// Origin:") && l.contains("Copyright") && l.contains("THIRD-PARTY.txt"))
    }).map(|f| f.display().to_string()).collect();
    assert!(missing.is_empty(), "no origin header: {missing:?}");
    let smithay = std::fs::read_to_string(root.join("vendor/smithay/LICENSE.txt")).unwrap_or_default();
    assert!(smithay.contains("Copyright (c) 2017 Victor Berger and Victoria Brekenfeld"));
    assert!(root.join("vendor/smithay/TRINDLE.md").is_file());
}

#[test]
fn the_notices_list_every_package_in_cargo_lock() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lock = std::fs::read_to_string(root.join("Cargo.lock")).unwrap();
    let notices = std::fs::read_to_string(root.join("share/licences/THIRD-PARTY.txt"))
        .expect("share/licences/THIRD-PARTY.txt is made by tools/licences.sh");
    let (mut name, mut missing, mut count) = (String::new(), Vec::new(), 0);
    for line in lock.lines() {
        if let Some(n) = line.strip_prefix("name = \"") {
            name = n.trim_end_matches('"').to_string();
        } else if let Some(v) = line.strip_prefix("version = \"") {
            count += 1;
            let entry = format!("\n{name} {}\n", v.trim_end_matches('"'));
            if !notices.contains(&entry) {
                missing.push(entry.trim().to_string());
            }
        }
    }
    assert!(count > 300, "read only {count} packages from Cargo.lock");
    assert!(missing.is_empty(), "not in THIRD-PARTY.txt (run sh tools/licences.sh): {missing:?}");
    let build = std::fs::read_to_string(root.join("tools/deb/build.sh")).unwrap();
    assert!(build.contains("licences.sh") && build.contains("usr/share/doc/trindle"));
    let copyright = std::fs::read_to_string(root.join("tools/deb/copyright")).unwrap();
    assert!(copyright.starts_with("Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/"));
    for needed in ["GPL-3+", "Smithay", "smithay-drm-extras", "spellbook", "OFL-1.1"] {
        assert!(copyright.contains(needed), "the copyright file does not name {needed}");
    }
}
