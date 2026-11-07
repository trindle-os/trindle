//! Neither the ring's own loop, nor the thread that draws Settings, nor the file chooser's window waits on a share.

use std::collections::BTreeMap;

use super::no_writing_in_place::{all_code, code};

const WHERE: [&str; 8] = ["ring/", "settings/", "screenshot/file_chooser", "toolkit/icon.rs",
                          "compose/cursor.rs", "session/folders.rs", "session/wallpaper.rs", "session/backdrop.rs"];

const LOOKS: &[&str] = &[
    ".exists()", ".is_dir()", ".is_file()", ".is_symlink()", "try_exists(", "metadata(", "read_dir(", "canonicalize(", "fs::read_link(",
    "fs::read(", "read_to_string(", "File::open(", "OpenOptions::", "open_file(", "read_file(", "read_string(", "read_small(",
    "read_within(", "read_bytes_within(", "read_text(", "fs::copy(", "copy_in(", "copy_new(", "copy_whole(", "fs::write(", "write_whole",
    "write_text(", "create_dir", "remove_file(", "remove_dir", "fs::rename(", "statvfs(", "room_left(", "how_full(",
    "picture::load(", "files::looks", "KeyFile::load", "moving::info(", "first_frame(", "Player::open(", "Clip::open(", "themes::add(",
    "add_fonts(", "from_xpm(", concat!("fetch_update", "_unseen("), "next_small(",
];

const HANDS: [&str; 5] = ["on_thread(", "thread::spawn(", "self.aside(", "gently(", "gently_over("];

const ASIDE: &[(&str, &[&str])] = &[
    ("compose/cursor.rs", &["first_of", "file_of"]),
    ("session/backdrop.rs", &["load_here"]),
    ("session/wallpaper.rs", &["named_picture"]),
    ("ring/crashes.rs", &["write_mark", "start_work", "made"]),
    ("ring/sni/outside.rs", &["seen_outside", "own_folder", "file_seen", "subfolders"]),
    ("ring/view/pinned.rs", &["found_outside", "path_kind"]),
    ("screenshot/file_chooser/looks.rs", &["begun", "named_places", "place_looked_at", "last_folder", "entry_of", "read_place", "watch_added", "listed_look", "is_taken", "target_found", "unique_all", "unique"]),
    ("settings/panels/sounds.rs", &["copied_in"]),
    ("settings/panels/thanks.rs", &["notices_path", "notices_counts", "open_notices"]),
    ("settings/panels/users.rs", &["fresh_png"]),
    ("toolkit/icon.rs", &["read_far", "seek_far"]),
    ("settings/panels/wallpaper/looking.rs", &["decode", "look", "showing_words", "said_words", "rotating_source", "open_film", "step_film", "next_frame", "vet"]),
];

const LEFT: &[(&str, usize, &str)] = &[
    ("ring/notes/keeps.rs", 2, "the pinned and snoozed notification circles, read at start and written whole in Trindle's own settings folder"),
    ("settings/panels/games/steam.rs", 1, "whether the game's setup folder is there, asked as the person presses Fresh start; Steam's library is in the person's own data folder"),
    ("settings/panels/keyboard/restart_keys.rs", 1, "the shortcut files in Trindle's own settings folder and in the system's, read when a typed key is checked; none can be on a share"),
    ("settings/wordsheet.rs", 10, "the sheet file the person chose and the words folder in Trindle's own settings folder; the reading, backing up and installing run only through take() and the card's save, which the Settings page calls on a thread"),
    ("settings/wordsheet/own.rs", 13, "the person's own language folders in Trindle's own settings folder; the Region page reads them with the other languages on a thread, and the start and removal are run from the card on one"),
    ("ring/drives.rs", 1, "the mark that the screen is locked, in the folder the session keeps while it runs"),
    ("ring/mod.rs", 2, "the note that the word about a virtual machine was given, in Trindle's own settings folder"),
    ("ring/notify.rs", 1, "the name of the sender's process, in /proc"),
    ("ring/plugins.rs", 3, "the plugins' own folders and their notes, in the person's data folder"),
    ("ring/quick/bluetooth.rs", 1, "the kernel's own list of Bluetooth adapters, in /sys"),
    ("ring/store.rs", 3, "the favourites and the count of what is used, in Trindle's own settings folder"),
    ("ring/tray.rs", 9, "the cup icon, written once to the person's data folder, and the kernel's own list of what runs, in /proc"),
    ("ring/view/mod.rs", 1, "the mark that the ring has closed, in the folder the session keeps while it runs"),
    ("session/folders.rs", 9, "user-dirs.dirs, the side bar's first bookmarks and the terminal's settings, all in the person's settings folder"),
    ("session/wallpaper.rs", 12, "the notes of what shows and who runs, in the folder the session keeps while it runs, and the kernel's own list of what runs; and the walk of a folder of pictures (`looked_in`), shared with Settings, which this program runs only by the rule (`gently`) and Settings only on a thread"),
    ("screenshot/file_chooser/lent.rs", 7, "a rename and a new folder, each called only on the thread `aside` starts (changes.rs), and the pins, in Trindle's own settings folder"),
    ("screenshot/file_chooser/kept.rs", 1, "the list of the folder each program last chose in, written whole in Trindle's own settings folder (it is read by `folders_read`, which sets a damaged list aside)"),
    ("settings/window.rs", 1, "Trindle's own settings folder, made so it can be watched"),
    ("settings/ui/icons.rs", 1, "a picture from Trindle's own share folder, or a plugin's own in the person's data folder"),
    ("settings/panels/about.rs", 2, "how full the computer's own disk is, asked of /, and the page's own note in Trindle's settings folder"),
    ("settings/panels/appearance.rs", 3, "the look's settings in the person's settings folder"),
    ("settings/panels/appearance/fonts_icons.rs", 1, "an icon set's folder, in the icon folders, the computer's and the person's own"),
    ("settings/panels/appearance/looks.rs", 1, "a theme file handed to the page's own `off`, which reads it on a thread"),
    ("settings/panels/appearance/pointers.rs", 3, "the pointers and their sets in the icon folders, the computer's and the person's own"),
    ("settings/panels/audio.rs", 3, "the test sound, written to the person's cache folder"),
    ("settings/panels/crash_reports.rs", 7, "the settings and the list of what was sent, all in Trindle's own settings and data folders, and the add-on's own file in /usr"),
    ("settings/panels/help.rs", 2, "the pictures for Help, in Trindle's own share folder"),
    ("settings/panels/keyboard/layout.rs", 1, "the computer's own list of keyboard layouts, in /usr"),
    ("settings/panels/network/refresh.rs", 3, "the kernel's own list of devices, in /sys"),
    ("settings/panels/permissions.rs", 3, "launcher files in the folders apps are installed to, the computer's and the person's data folder, and the folder of the plugins Trindle itself keeps in the person's data folder"),
    ("settings/panels/region.rs", 2, "the computer's own lists of languages, in /usr"),
    ("settings/panels/snapshots.rs", 1, "the mark that the system files are locked, in /etc/trindle"),
    ("settings/panels/sounds.rs", 3, "the folder of the person's own sounds, in their data folder"),
    ("settings/panels/startup.rs", 2, "launcher files in the folders apps are installed to, the computer's and the person's data folder"),
    ("settings/panels/users.rs", 3, "the computer's own login settings in /etc, each person's picture in /var/lib, and the person's own picture written to their home, where their settings are too"),
    ("toolkit/icon.rs", 8, "the chosen set's name in the person's settings folder; a folder of an icon set, its index and an icon in it, each looked at only once every step to it is known to be on this computer (`Icons::is_here`, and `own_place` for one that is a link), the name itself without following it; and an icon named by a whole path that is known the same way"),
];

fn begins(line: &str) -> Option<&str> {
    let mut rest = line.trim_start();
    if let Some(after) = rest.strip_prefix("pub") {
        rest = after.trim_start_matches(|c: char| c == '(' || c == ')' || c == ':' || c.is_ascii_lowercase()).trim_start();
    }
    for word in ["const ", "unsafe "] {
        rest = rest.strip_prefix(word).unwrap_or(rest);
    }
    let name = rest.strip_prefix("fn ")?;
    Some(&name[..name.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(name.len())])
}

fn said(line: &str) -> &str {
    let cut = line.match_indices("//").map(|(at, _)| at).find(|at| *at == 0 || line[..*at].ends_with([' ', '\t']));
    &line[..cut.unwrap_or(line.len())]
}

fn calls(line: &str, name: &str) -> bool {
    line.match_indices(name).any(|(at, _)| {
        let before = line[..at].chars().next_back();
        line[at + name.len()..].starts_with('(')
            && !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            && !line[..at].ends_with("fn ")
    })
}

fn judge(file: &str, text: &str, aside: &[&str]) -> (usize, Vec<String>) {
    let (mut loose, mut wrong, mut within, mut defined, mut closure) = (0, Vec::new(), "", Vec::new(), false);
    for line in code(text) {
        let line = said(line);
        if let Some(name) = begins(line) {
            within = name;
            if aside.contains(&name) {
                defined.push(name);
                if line.trim_start().starts_with("pub") {
                    wrong.push(format!("{file}: {name} is run on a thread and must be the file's own, not pub"));
                }
            }
        }
        let handed = closure || HANDS.iter().any(|h| line.contains(h));
        if HANDS.iter().any(|h| line.contains(h)) && line.trim_end().ends_with("|| {") {
            closure = true;
        } else if closure && line.trim_start().starts_with("}, ") {
            closure = false;
        }
        if aside.contains(&within) || handed {
            continue;
        }
        if LOOKS.iter().any(|l| line.contains(l)) {
            loose += 1;
        }
        for name in aside.iter().filter(|name| calls(line, name)) {
            wrong.push(format!("{file}: {name} looks at what may be on a share, and {within} calls it with no thread: {}", line.trim()));
        }
    }
    wrong.extend(aside.iter().filter(|name| !defined.contains(name)).map(|name| format!("{file}: {name} is listed in ASIDE and is not there")));
    (loose, wrong)
}

#[test]
fn a_look_is_counted_unless_it_is_run_on_a_thread() {
    let text = "fn view(&self) {\n    if path.is_dir() {\n        // p.exists()\n        let n = std::fs::read_dir(&path); // metadata(p)\n    }\n\
                \x20   on_thread(move || std::fs::read(&p), done);\n}\n\nfn seen(p: &Path) -> bool {\n    p.exists() && p.is_file()\n}\n\n\
                fn ask(&self) {\n    on_thread(move || seen(&p), done);\n}\n\n#[cfg(test)]\nmod tests {\n    fn t() { p.exists(); seen(p); }\n}\n";
    assert_eq!(judge("x.rs", text, &["seen"]), (2, Vec::new()));
    assert_eq!(judge("x.rs", text, &[]).0, 3);
    let called = "fn seen(p: &Path) -> bool {\n    p.exists()\n}\n\nfn view(&self) {\n    let there = seen(&p);\n    let unseen(p);\n}\n";
    let (loose, wrong) = judge("x.rs", called, &["seen"]);
    assert_eq!((loose, wrong.len()), (0, 1), "{wrong:?}");
    assert!(wrong[0].contains("view calls it with no thread"));
    assert_eq!(judge("x.rs", "pub(super) fn seen(p: &Path) -> bool {\n    p.exists()\n}\n", &["seen"]).1.len(), 1);
    assert_eq!(judge("x.rs", "fn view() {}\n", &["seen"]).1, ["x.rs: seen is listed in ASIDE and is not there"]);
    assert_eq!((begins("    pub(crate) fn look_at(p: &Path) {"), begins("    const unsafe fn f() {"), begins("    let f = fn_of(x);")), (Some("look_at"), Some("f"), None));
}

#[test]
fn neither_the_ring_nor_settings_looks_at_what_may_be_on_a_share_but_on_a_thread() {
    let aside: BTreeMap<&str, &[&str]> = ASIDE.iter().copied().collect();
    assert_eq!(aside.len(), ASIDE.len(), "a file is listed twice in ASIDE");
    let listed: BTreeMap<&str, usize> = LEFT.iter().map(|(file, n, _)| (*file, *n)).collect();
    assert_eq!(listed.len(), LEFT.len(), "a file is listed twice in LEFT");
    assert!(LEFT.iter().all(|(_, n, why)| *n > 0 && !why.is_empty()), "each one listed says how many and why");
    let (mut wrong, mut seen) = (Vec::new(), Vec::new());
    for (file, text) in all_code().into_iter().filter(|(file, _)| WHERE.iter().any(|w| file.starts_with(w))) {
        let (loose, faults) = judge(&file, &text, aside.get(file.as_str()).copied().unwrap_or_default());
        wrong.extend(faults);
        let most = listed.get(file.as_str()).copied().unwrap_or(0);
        if loose != most {
            wrong.push(format!("{file} looks at the disk on {loose} lines outside a thread, and {most} are listed"));
        }
        seen.push(file);
    }
    wrong.extend(aside.keys().chain(listed.keys()).filter(|file| !seen.iter().any(|s| s == *file)).map(|file| format!("{file} is listed and is not there")));
    assert!(seen.len() > 40, "the ring's and Settings' code was not found");
    assert!(seen.iter().any(|file| file == "screenshot/file_chooser.rs"), "the file chooser's code was not found");
    for start in ["compose/cursor.rs", "session/folders.rs", "session/wallpaper.rs", "session/backdrop.rs"] {
        assert!(seen.iter().any(|file| file == start), "{start}, which runs as a session starts, was not found");
    }
    assert!(wrong.is_empty(), "run the look on a thread and name its function in ASIDE, or list it in LEFT with why it cannot be on a share: {wrong:#?}");
}
