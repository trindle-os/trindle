//! Which icon stands for a notification: its own, its program's, or the fallback.

use crate::common::apps;
use crate::ring::ctx::Program;
use crate::ring::notify::Note;

pub(super) const FALLBACK_ICON: &str = "preferences-system-notifications-symbolic";

pub(super) const ICON_PX: u32 = 16;

pub fn note_icon(all: &[Program], note: &Note) -> String {
    match note_app(all, note) {
        Some(app) => app.icon.clone().unwrap_or_else(|| apps::generic_icon_for(app.key(), &app.name)),
        None => named_icon(note),
    }
}

fn note_app<'a>(all: &'a [Program], note: &Note) -> Option<&'a Program> {
    if note.desktop.is_empty() { None } else { apps::find_for_window(all, &note.desktop) }
}

pub(super) fn note_icons(all: &[Program], note: &Note) -> Vec<String> {
    match note_app(all, note) {
        Some(app) => {
            let mut icons = app.icons();
            icons.retain(|i| !i.is_empty());
            icons
        }
        None => vec![named_icon(note)],
    }
}

pub(super) fn named_icon(note: &Note) -> String {
    let named = note.icon.as_str();
    if named.starts_with("file://") {
        match crate::common::copied::uri_path(named) {
            Ok(path) if image_file(&path) => path.to_string_lossy().into_owned(),
            _ => FALLBACK_ICON.into(),
        }
    } else if named.starts_with('/') {
        if image_file(std::path::Path::new(named)) { named.to_string() } else { FALLBACK_ICON.into() }
    } else if !named.is_empty() {
        named.to_string()
    } else {
        FALLBACK_ICON.into()
    }
}

fn image_file(path: &std::path::Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| ["png", "svg", "svgz", "jpg", "jpeg", "webp", "gif", "xpm", "bmp"].contains(&e.to_ascii_lowercase().as_str()))
}
