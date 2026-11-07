//! The items a plugin adds to its ring icon's menu, read from its manifest.

use std::path::Path;

use crate::common::keyfile::KeyFile;
use crate::toolkit::t;

use super::fields::is_plain;
use super::manifest::{command, good_name, list, MANIFEST, text};

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub run: Vec<String>,
}

pub const MENU_MAX: usize = 8;
pub const MENU_LABEL_MAX: usize = 32;

pub(super) fn menu_items(at: &Path, kf: &KeyFile, problems: &mut Vec<String>) -> Vec<MenuItem> {
    let ids = list(kf, "Ring", "Menu");
    if ids.len() > MENU_MAX {
        problems.push(t!("plugins-menu-too-many", n = ids.len(), max = MENU_MAX));
    }
    let mut items: Vec<MenuItem> = Vec::new();
    for id in ids.into_iter().take(MENU_MAX) {
        let group = format!("Menu {id}");
        if !good_name(&id) {
            problems.push(t!("plugins-menu-bad-id", id = &id));
            continue;
        }
        if items.iter().any(|i| i.id == id) {
            problems.push(t!("plugins-menu-twice", id = &id));
            continue;
        }
        if !kf.has_group(&group) {
            problems.push(t!("plugins-menu-no-part", id = &id, group = &group));
            continue;
        }
        let label = text(kf, &group, "Label");
        if label.is_empty() {
            problems.push(t!("plugins-needs-label", group = &group));
        } else if label.chars().count() > MENU_LABEL_MAX || !is_plain(&label, MENU_LABEL_MAX) {
            problems.push(t!("plugins-label-short-line", group = &group, max = MENU_LABEL_MAX));
        }
        let run = text(kf, &group, "Run");
        match command(at, &group, &run, problems) {
            Some(argv) if !label.is_empty() => items.push(MenuItem { id, label, run: argv }),
            Some(_) => {}
            None if run.is_empty() => problems.push(t!("plugins-menu-needs-run", group = &group)),
            None => {}
        }
    }
    items
}

pub fn menu_of(at: &Path) -> Vec<MenuItem> {
    match KeyFile::load(&at.join(MANIFEST)) {
        Ok(kf) if kf.has_group("Ring") => menu_items(at, &kf, &mut Vec::new()),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests;
