//! The folder each program last chose in, kept in Trindle's own settings folder.

use std::path::{Path, PathBuf};

use crate::common::files::config_path;
use crate::toolkit::t;

pub(super) fn folders_file() -> PathBuf {
    config_path("file-chooser-folders")
}

pub fn folders_damaged() -> &'static str {
    t!("screenshot-chooser-damaged")
}

pub(crate) fn folders_read(file: &Path) -> String {
    crate::common::damaged::read(file, folders_damaged(), |l| l.split_once('\t').is_some())
}

pub(crate) fn keep_folder(file: &Path, app_id: &str, folder: &Path) {
    let text = folders_read(file);
    if crate::common::damaged::may_write(file, folders_damaged()).is_err() {
        return;
    }
    let mut lines: Vec<String> = text.lines().filter(|l| l.split_once('\t').is_none_or(|(a, _)| a != app_id)).map(String::from).collect();
    lines.insert(0, format!("{app_id}\t{}", folder.display()));
    lines.truncate(100);
    let _ = crate::common::files::write_text(file, &(lines.join("\n") + "\n"));
}
