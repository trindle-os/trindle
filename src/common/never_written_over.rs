//! A settings file that is there and cannot be read is never written over.

use std::path::{Path, PathBuf};

const NOT_TEXT: &[u8] = b"first\n\xff\xfe second\nthird\n";

type Change = (&'static str, &'static str, fn(&Path) -> bool);

const CHANGES: &[Change] = &[
    ("the ring's favourites, kept whole by the ring", "favourites",
     |f| super::favourites::save(f, &["org.example.One.desktop".to_string()]).is_err()),
    ("Text's own spelling words", "text-words",
     |f| crate::text::kept_files::add_word(f, "colour").is_err()),
    ("Text's open tabs, and through them the words not yet saved", "session",
     |f| crate::text::kept_files::write_no_tabs(f.parent().unwrap()).is_err()),
    ("the apps that open each kind of file", "mimeapps.list",
     |f| super::apps::choose_opener_in(f, "org.example.One.desktop", &["text/plain"]).is_err()),
    ("GTK's settings, which other programs write too", "settings.ini",
     |f| crate::settings::panels::appearance::set_ini_in(f, "gtk-font-name", "Sans 11").is_err()),
    ("the person's own gtk.css", "gtk.css",
     |f| super::app_theme::import_into(f).is_err()),
    ("GTK's bookmarks, which other file managers write too", "bookmarks",
     |f| crate::session::folders::side_bar_in(f, "").is_err()),
    ("the keys chosen for plugins' shortcuts", "plugin-keys.ini",
     |f| crate::plugins::shortcuts::keep_choice(f, "org.example.One", "open", "Super+O").is_err()),
    ("what a plugin has already asked the person", "asked",
     |f| crate::plugins::mark_asked_in(f, &["network".to_string()]).is_err()),
    ("the crash-report switches", "crash-reports.json",
     |f| crate::settings::panels::crash_reports::save_in(f, "send", true).is_err()),
];

fn scratch(n: usize) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-never-written-over-{}-{n}", std::process::id()));

    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_settings_file_that_cannot_be_read_is_never_written_over() {

    let mut wrong = Vec::new();
    for (n, (what, name, change)) in CHANGES.iter().enumerate() {
        let dir = scratch(n);
        let file = dir.join(name);
        std::fs::write(&file, NOT_TEXT).unwrap();
        let was = std::fs::read(&file).unwrap();
        if !change(&file) {
            wrong.push(format!("{what}: the change went ahead over a file that could not be read"));
        }
        if std::fs::read(&file).ok() != Some(was) {
            wrong.push(format!("{what}: a file that could not be read was written over"));
        }

        let _ = std::fs::remove_file(&file);
        if change(&file) {
            wrong.push(format!("{what}: a first change was refused"));
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
