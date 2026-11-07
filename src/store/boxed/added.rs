//! The programs the person added.

use std::path::{Path, PathBuf};
use super::first_apps::{offered_file};
use super::launchers::launchers_of;
use super::{folder, installed, lines_at, version};

pub(super) const ADDED: &str = "added";

pub fn came_with_trindle(name: &str, marked: bool, offered: &[String]) -> bool {
    !marked && offered.iter().any(|n| n == name)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub package: String,
    pub name: String,
    pub version: String,
    pub size: u64,
    pub launcher: Option<PathBuf>,
    pub icon: Option<String>,
}

pub fn added() -> Vec<Program> {
    let offered = lines_at(&offered_file());
    let mut all: Vec<Program> = installed().into_iter()
        .filter(|name| !came_with_trindle(name, folder(name).join(ADDED).is_file(), &offered))
        .map(|package| {
            let entries: Vec<(PathBuf, std::collections::BTreeMap<String, String>)> = launchers_of(&package).into_iter()
                .map(|p| { let e = crate::icons::read_entry(&p); (p, e) }).collect();
            let shown = entries.iter().find(|(_, e)| e.get("NoDisplay").map(String::as_str) != Some("true")).or(entries.first());
            Program {
                name: shown.and_then(|(_, e)| e.get("Name").cloned()).filter(|n| !n.is_empty()).unwrap_or_else(|| package.clone()),
                version: version(&package).unwrap_or_default(),
                size: size_of_tree(&folder(&package)),
                launcher: shown.map(|(p, _)| p.clone()),
                icon: shown.and_then(|(_, e)| e.get("Icon").cloned()).filter(|i| !i.is_empty()),
                package,
            }
        }).collect();
    all.sort_by_key(|p| p.name.to_lowercase());
    all
}

pub fn size_of_tree(top: &Path) -> u64 {
    std::fs::read_dir(top).into_iter().flatten().flatten().map(|e| match e.file_type() {
        Ok(t) if t.is_dir() => size_of_tree(&e.path()),
        Ok(t) if t.is_file() => e.metadata().map_or(0, |m| m.len()),
        _ => 0,
    }).sum()
}
