//! Which things the chooser lists, in what order, and where a typed name points.

use std::path::{Path, PathBuf};

use crate::common::kinds::{glob_matches, Mime};

use super::{Entry, Filter};

pub(super) fn passes(filter: &Filter, mime: &Mime, name: &str, kind: Option<&str>) -> bool {
    let lower = name.to_lowercase();
    filter.rules.iter().any(|(how, rule)| match how {
        0 => glob_matches(&rule.to_lowercase(), &lower),
        _ => rule == "application/octet-stream" || kind.is_some_and(|k| mime.is_a(k, rule)),
    })
}

pub(super) fn resolve(typed: &str, folder: Option<&Path>, home: &Path) -> Option<PathBuf> {
    if let Some(rest) = typed.strip_prefix("~/") {
        return Some(home.join(rest));
    }
    if typed == "~" {
        return Some(home.to_path_buf());
    }
    if typed.starts_with('/') {
        return Some(PathBuf::from(typed));
    }
    folder.map(|f| f.join(typed))
}

pub(super) fn sort(entries: &mut [Entry]) {
    entries.sort_by(|a, b| b.folder.cmp(&a.folder).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())).then_with(|| a.name.cmp(&b.name)));
}
