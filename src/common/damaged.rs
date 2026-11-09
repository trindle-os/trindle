//! A lesser file of the person's own (clipboard history, launch counts,
//! last-used apps, the folders the chooser remembers, the eye dropper's
//! colours, Text's recent files) that has been damaged. It is set aside
//! under another name beside itself and a fresh one is started, so what was
//! left of it can still be looked at and the next write does not lose it.
//! The program says so in a few words where the person is working
//! (`take_notes`).

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use trindle_toolkit::t;

use super::files::read_small;

static NOTES: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn words(what: &str) -> String {
    t!("common-damaged-reset", what = what)
}

pub fn unreadable_words(what: &str) -> String {
    t!("common-unreadable-kept", what = what)
}

pub fn may_write(path: &Path, what: &str) -> std::io::Result<()> {
    super::files::read_small(path).map(drop).map_err(|e| {
        let said = unreadable_words(what);
        note(said.clone());
        std::io::Error::new(e.kind(), Unreadable(said))
    })
}

#[derive(Debug)]
struct Unreadable(String);

impl std::fmt::Display for Unreadable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Unreadable {}

pub fn said(e: &std::io::Error) -> Option<String> {
    e.get_ref().and_then(|r| r.downcast_ref::<Unreadable>()).map(|u| u.0.clone())
}

pub fn take_notes() -> Vec<String> {
    std::mem::take(&mut *NOTES.lock().unwrap_or_else(|e| e.into_inner()))
}

fn note(words: String) {
    let mut all = NOTES.lock().unwrap_or_else(|e| e.into_inner());
    if !all.contains(&words) {
        all.push(words);
    }
}

fn aside_name(path: &Path) -> PathBuf {
    let base = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    (0..).map(|n| path.with_file_name(if n == 0 { format!("{base}.damaged") } else { format!("{base}.damaged{n}") }))
        .find(|p| std::fs::symlink_metadata(p).is_err())
        .unwrap_or_else(|| path.with_file_name(format!("{base}.damaged")))
}

pub fn read(path: &Path, what: &str, sound: impl Fn(&str) -> bool) -> String {
    let text = match read_small(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::InvalidData => return aside(path, what),
        Err(_) => {
            note(unreadable_words(what));
            return String::new();
        }
    };
    if text.lines().filter(|l| !l.trim().is_empty()).all(&sound) {
        text
    } else {
        aside(path, what)
    }
}

pub fn read_whole(path: &Path, what: &str, sound: impl Fn(&str) -> bool) -> String {
    let text = match read_small(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::InvalidData => return aside(path, what),
        Err(_) => {
            note(unreadable_words(what));
            return String::new();
        }
    };
    if text.trim().is_empty() || sound(&text) {
        text
    } else {
        aside(path, what)
    }
}

fn aside(path: &Path, what: &str) -> String {
    let to = aside_name(path);

    match super::files::rename_new(path, &to) {
        Ok(()) => note(words(what)),
        Err(e) => eprintln!("trindle: {} is damaged and could not be set aside: {e}", path.display()),
    }
    String::new()
}

#[cfg(test)]
pub const HELP: &str = "settings-unreadable";

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = crate::common::files::scratch().join(format!("damaged-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_damaged_file_is_set_aside_and_a_fresh_one_started() {
        let d = dir("one");
        let f = d.join("usage");
        std::fs::write(&f, "3 org.example.Notes\nnot a count\n").unwrap();
        assert_eq!(read(&f, "Launch counts", |l| l.split_once(' ').is_some_and(|(n, _)| n.parse::<u32>().is_ok())), "");
        assert!(!f.exists());
        assert!(std::fs::read_to_string(d.join("usage.damaged")).unwrap().contains("not a count"));
        assert!(take_notes().contains(&words("Launch counts")));

        std::fs::write(&f, [0xff, 0xfe, 0x00]).unwrap();
        assert_eq!(read(&f, "Launch counts", |_| true), "");
        assert!(d.join("usage.damaged1").exists() && d.join("usage.damaged").exists());
    }

    #[test]
    fn each_of_the_six_lesser_files_is_set_aside_when_damaged() {
        use crate::{common::{app_choice, clipboard}, ring::store, screenshot::{colours, file_chooser::kept}, text};
        let d = dir("six");
        let _ = take_notes();
        let junk: &[u8] = &[0xff, 0xfe, b'{', 0x00];
        let names = ["index.json", "usage", "app-choices", "file-chooser-folders", "colours", "recent"];
        for name in names {
            std::fs::write(d.join(name), junk).unwrap();
        }
        assert!(clipboard::load_from(&d.join("index.json")).is_empty());
        assert!(store::usage_in(&d.join("usage")).is_empty());
        assert!(app_choice::read(&d.join("app-choices")).is_empty());
        assert!(kept::folders_read(&d.join("file-chooser-folders")).is_empty());
        assert!(colours::load_from(&d.join("colours")).is_empty());
        assert!(text::store::recent(&d).is_empty());
        for name in names {
            assert!(!d.join(name).exists() && d.join(format!("{name}.damaged")).exists(), "{name}");
        }
        let said = take_notes();
        for what in [clipboard::DAMAGED, store::USAGE_DAMAGED, app_choice::DAMAGED, kept::folders_damaged(), "Eye dropper colours", text::store::recent_damaged()] {
            assert!(said.contains(&words(what)), "{what}");
        }
        assert_eq!(words("Clipboard history"), "Clipboard history reset: file damaged, kept beside it.");
    }

    #[test]
    fn a_sound_file_and_a_missing_one_are_left_alone() {
        let d = dir("two");
        let f = d.join("usage");
        assert_eq!(read(&f, "x", |_| false), "");
        std::fs::write(&f, "3 a\n\n").unwrap();
        assert_eq!(read(&f, "x", |l| l.starts_with('3')), "3 a\n\n");
        assert!(f.exists() && !d.join("usage.damaged").exists());
    }
}
