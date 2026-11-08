//! A Debian package installed in its own folder, never for the whole
//! computer, and run inside bubblewrap. Nothing here needs a password.

pub mod added;
pub mod first_apps;
pub mod install;
pub mod launchers;
pub mod needs;
pub mod run;
pub mod walls;
pub mod x11;
#[cfg(test)]
mod e2e_tests;
#[cfg(test)]
mod tests;

use crate::toolkit::t;
use std::path::{Path, PathBuf};
use crate::common::files::data_home;

pub const MARK: &str = "X-Trindle-Boxed=";

pub const ALLOWED: &[&str] = &["network", "sound", "microphone", "files", "screen", "x11", "computer"];

const MOST_PARTS: usize = 300;

pub const FIRST_APPS: &str = "/usr/share/trindle/first-apps";

pub fn folders() -> PathBuf {
    data_home().join("trindle/debs")
}

pub fn folder(name: &str) -> PathBuf {
    folders().join(name)
}

fn applications() -> PathBuf {
    data_home().join("applications")
}

pub fn version(name: &str) -> Option<String> {
    std::fs::read_to_string(folder(name).join("version")).ok().map(|v| v.trim().to_string())
}

pub fn installed() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(folders()).into_iter().flatten().flatten()
        .filter(|e| e.path().join("version").is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn lines_at(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path).unwrap_or_default()
        .lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

pub(crate) fn write_kept_lines(path: &Path, what: &str, lines: &[String]) -> std::io::Result<()> {
    crate::common::damaged::may_write(path, what)?;
    write_lines(path, lines)
}

fn write_lines(path: &Path, lines: &[String]) -> std::io::Result<()> {
    let text = if lines.is_empty() { String::new() } else { lines.join("\n") + "\n" };
    crate::common::files::write_text(path, &text)
}

pub fn allowed(name: &str) -> Vec<String> {
    allowed_at(&folder(name), ALLOWED)
}

pub fn allow(name: &str, what: &str, on: bool) -> std::io::Result<()> {
    allow_at(&folder(name), ALLOWED, what, on)
}

pub fn allowed_at(at: &Path, known: &[&str]) -> Vec<String> {
    lines_at(&at.join("allowed")).into_iter().filter(|l| known.contains(&l.as_str())).collect()
}

pub fn allow_at(at: &Path, known: &[&str], what: &str, on: bool) -> std::io::Result<()> {
    let mut now = allowed_at(at, known);
    now.retain(|w| w != what);
    if on {
        now.push(what.to_string());
    }
    write_kept_lines(&at.join("allowed"), t!("common-unreadable-name-box-permissions"), &now)
}

pub fn wants(name: &str) -> Vec<String> {
    lines_at(&folder(name).join("wants"))
}

pub fn asked(name: &str) -> Vec<String> {
    lines_at(&folder(name).join("asked"))
}

pub fn mark_asked(name: &str) -> std::io::Result<()> {
    mark_asked_at(&folder(name))
}

pub(crate) fn mark_asked_at(at: &Path) -> std::io::Result<()> {
    let mut all = lines_at(&at.join("asked"));
    for w in lines_at(&at.join("wants")) {
        if !all.contains(&w) {
            all.push(w);
        }
    }
    write_kept_lines(&at.join("asked"), t!("common-unreadable-name-box-asked"), &all)
}

pub fn categories(name: &str) -> String {
    categories_in(&folder(name).join("files"))
}

fn categories_in(files: &Path) -> String {
    std::fs::read_dir(files.join("usr/share/applications")).into_iter().flatten().flatten()
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .flat_map(|t| t.lines().filter_map(|l| l.strip_prefix("Categories=").map(String::from)).collect::<Vec<_>>())
        .collect::<Vec<_>>().join(";")
}

pub fn carried(name: &str) -> Vec<(String, String, String)> {
    lines_at(&folder(name).join("packages")).iter().filter_map(|l| {
        let mut w = l.split_whitespace();
        Some((w.next()?.to_string(), w.next()?.to_string(), w.next().unwrap_or("").to_string()))
    }).collect()
}
