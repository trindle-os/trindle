//! Tests' way to run with the words of a language Trindle ships.

use std::path::Path;

pub fn tags() -> Vec<String> {
    let mut tags: Vec<String> = std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang"))
        .unwrap().flatten().filter(|e| e.path().is_dir()).filter_map(|e| e.file_name().into_string().ok()).collect();
    tags.sort();
    tags
}

pub fn in_language<R>(tag: &str, during: impl FnOnce() -> R) -> R {
    use crate::toolkit::lang::{english, folder_files, with_catalogue, Bundle, Catalogue};
    if let Some(script) = font_missing(tag) {
        panic!("{tag}: no {} font is installed, so its words cannot be measured: install {}", script.name, script.package);
    }
    let files = folder_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang").join(tag));
    let files: Vec<(&str, &str)> = files.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    with_catalogue(Catalogue::new(Bundle::from_files(tag, &files), english()), during)
}

fn font_missing(tag: &str) -> Option<&'static crate::toolkit::lang::script::Script> {
    use crate::toolkit::lang::{script, tags_of};
    static FAMILIES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    let tags = tags_of(tag);
    script::script_of(&tags)?;
    script::missing_in(script::SCRIPTS, &tags, FAMILIES.get_or_init(script::system_families))
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_shipped_language_has_its_fonts() {
        for tag in super::tags() {
            if let Some(script) = super::font_missing(&tag) {
                panic!("{tag}: install {} for its {} writing", script.package, script.name);
            }
        }
    }
}
