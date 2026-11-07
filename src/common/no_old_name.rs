//! Trindle's earlier name is gone from the whole project, and a scan of every text file and file name here keeps it so.

use std::path::Path;

fn old_name() -> Vec<u8> {
    ["round", "el"].concat().into_bytes()
}

fn text_holds(file: &Path, word: &[u8]) -> bool {
    std::fs::read(file).is_ok_and(|b| !b.contains(&0) && holds(&b, word))
}

fn holds(text: &[u8], word: &[u8]) -> bool {
    text.to_ascii_lowercase().windows(word.len()).any(|w| w == word)
}

fn scan(dir: &Path, word: &[u8], found: &mut Vec<String>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "target" || (name.starts_with('.') && name != ".gitignore") {
            continue;
        }
        let Ok(kind) = entry.file_type() else { continue };
        if holds(name.as_bytes(), word) || (kind.is_file() && text_holds(&path, word)) {
            found.push(path.display().to_string());
        }
        if kind.is_dir() {
            scan(&path, word, found);
        }
    }
}

#[test]
fn the_old_name_is_nowhere_in_the_project() {
    let mut found = Vec::new();
    scan(Path::new(env!("CARGO_MANIFEST_DIR")), &old_name(), &mut found);
    assert!(found.is_empty(), "Trindle's earlier name is back in: {found:?}");
}
