//! The ring's favourites: the apps round its middle, and the folders and
//! files pinned there beside them. They are kept in one text file, a line
//! each, in order round the ring from twelve o'clock: an app by its
//! launcher's name, a folder or a file by its whole path.

use std::path::{Path, PathBuf};

use super::apps::Entry;
use super::files::{config_path, write_text};

pub const MAX: usize = 12;

pub const STARTING: &[&str] = &["trindle-folders.desktop", "trindle-drives.desktop"];

pub const HEADER: &str = "# Trindle ring favourites, in order round the ring from twelve o'clock.";

const RENAMED: &[(&str, &str)] = &[
    ("trindle-disks.desktop", "trindle-drives.desktop"),
    ("trindle-store.desktop", "trindle-apps.desktop"),
    ("trindle-files.desktop", "trindle-folders.desktop"),
];

pub fn renamed(id: &str) -> &str {
    RENAMED.iter().find(|(old, _)| *old == id).map_or(id, |(_, new)| new)
}

pub fn file() -> PathBuf {
    config_path("favourites")
}

pub fn from_text(text: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for line in text.lines() {
        let line = renamed(line.trim());
        if !line.is_empty() && !line.starts_with('#') && !ids.iter().any(|i| i == line) {
            ids.push(line.to_string());
        }
    }

    if !text.lines().any(|l| l.starts_with("# Trindle ring favo")) {
        for (n, p) in STARTING.iter().enumerate() {
            if !ids.iter().any(|i| i == p) {
                ids.insert(n.min(ids.len()), p.to_string());
            }
        }
    }
    ids
}

fn read_kept(file: &Path) -> std::io::Result<Vec<String>> {
    Ok(from_text(&crate::common::files::read_small(file)?))
}

pub fn save(file: &Path, ids: &[String]) -> std::io::Result<()> {
    crate::common::files::read_small(file)?;
    write(file, ids)
}

pub fn write(file: &Path, ids: &[String]) -> std::io::Result<()> {
    let mut text = format!("{HEADER}\n");
    for id in ids {
        text.push_str(id);
        text.push('\n');
    }
    write_text(file, &text)
}

pub fn is_path(id: &str) -> bool {
    id.starts_with('/')
}

pub fn unpin_app(file: &Path, id: &str) -> std::io::Result<bool> {
    let mut ids = read_kept(file)?;
    let before = ids.len();
    ids.retain(|i| i != id);
    if ids.len() == before {
        return Ok(false);
    }
    write(file, &ids)?;
    Ok(true)
}

pub fn path_entry(path: &Path, icon: Option<String>) -> Entry {
    let name = path.file_name().map_or_else(|| path.to_string_lossy().into_owned(), |n| n.to_string_lossy().into_owned());
    Entry {
        id: path.to_string_lossy().into_owned(), name, icon, exec: String::new(), try_exec: None, wm_class: String::new(),
        program: String::new(), kinds: Vec::new(), path: path.to_path_buf(), category: "", keywords: Vec::new(),
        flatpak: None, appimage: None, terminal: false,
    }
}

#[cfg(test)]
pub fn read(file: &Path) -> Vec<String> {
    from_text(&super::files::read_text(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-favourites-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_app_taken_off_twice_is_not_a_fault() {
        let dir = scratch("twice");
        let file = dir.join("favourites");
        write(&file, &["krita.desktop".to_string(), "obsidian.desktop".to_string()]).unwrap();
        assert!(unpin_app(&file, "obsidian.desktop").unwrap());
        assert!(!unpin_app(&file, "obsidian.desktop").unwrap());
        assert_eq!(read(&file), ["krita.desktop".to_string()]);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_path_shows_in_the_ring_under_its_name_and_in_no_piece_of_all_apps() {
        let e = path_entry(Path::new("/home/john/Photos"), Some("folder-symbolic".into()));
        assert_eq!((e.name.as_str(), e.id.as_str(), e.category), ("Photos", "/home/john/Photos", ""));
        assert!(is_path(&e.id) && e.exec.is_empty());
        assert!(e.icons().contains(&"folder-symbolic".to_string()));
    }
}
