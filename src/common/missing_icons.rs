//! The list of programs shown with a generic icon because they have none of their own.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use trindle_toolkit::t;
use std::sync::atomic::{AtomicBool, Ordering};

const LINE_MOST: usize = 200;
const LINES_MOST: usize = 2000;

static SEEN: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static WAITING: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
static WRITING: AtomicBool = AtomicBool::new(false);

pub fn file() -> PathBuf {
    super::files::home().join(".local/share/trindle/missing-icons.txt")
}

pub fn note(id: &str, name: &str) {
    let id = tidy(id);
    if id.is_empty() {
        return;
    }
    {
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if !seen.get_or_insert_with(HashSet::new).insert(id.clone()) {
            return;
        }
    }
    WAITING.lock().unwrap_or_else(|e| e.into_inner()).push((id, tidy(name)));
    #[cfg(test)]
    if std::env::var_os("TRINDLE_TEST_MISSING_ICONS").is_none() {
        WAITING.lock().unwrap_or_else(|e| e.into_inner()).clear();
        return;
    }
    if WRITING.swap(true, Ordering::AcqRel) {
        return;
    }
    let spawned = std::thread::Builder::new().name("missing-icons".into()).spawn(|| {
        loop {
            let batch: Vec<_> = std::mem::take(&mut *WAITING.lock().unwrap_or_else(|e| e.into_inner()));
            if !batch.is_empty() {
                let _ = add_to(&file(), &batch);
            }
            WRITING.store(false, Ordering::Release);
            if WAITING.lock().unwrap_or_else(|e| e.into_inner()).is_empty() || WRITING.swap(true, Ordering::AcqRel) {
                return;
            }
        }
    });
    if spawned.is_err() {
        WRITING.store(false, Ordering::Release);
    }
}

fn tidy(s: &str) -> String {
    let s: String = s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    s.trim().chars().take(LINE_MOST).collect()
}

pub fn add_to(path: &Path, items: &[(String, String)]) -> std::io::Result<()> {
    let old = super::damaged::may_write(path, t!("common-unreadable-name-missing-icons"))
        .map(|()| std::fs::read_to_string(path).unwrap_or_default())?;
    let Some(text) = with(&old, items) else { return Ok(()) };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    super::files::write_whole(path, text.as_bytes())
}

fn with(old: &str, items: &[(String, String)]) -> Option<String> {
    let mut have: HashSet<&str> = old.lines().filter_map(|l| l.split('\t').next()).collect();
    let mut text = old.to_string();
    let mut lines = old.lines().count();
    let mut added = false;
    for (id, name) in items {
        if lines >= LINES_MOST || !have.insert(id.as_str()) {
            continue;
        }
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&format!("{id}\t{name}\n"));
        lines += 1;
        added = true;
    }
    added.then_some(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, name: &str) -> (String, String) {
        (id.to_string(), name.to_string())
    }

    #[test]
    fn a_program_is_listed_once_with_its_id_and_name_and_the_list_is_kept_whole() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("missing-icons-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let file = dir.join("share/trindle/missing-icons.txt");
        add_to(&file, &[item("org.x.A", "Alpha"), item("org.x.A", "Alpha again")]).unwrap();
        add_to(&file, &[item("org.x.B", "Beta"), item("org.x.A", "Alpha")]).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "org.x.A\tAlpha\norg.x.B\tBeta\n");
        let before = std::fs::metadata(&file).unwrap().modified().unwrap();
        add_to(&file, &[item("org.x.B", "Beta")]).unwrap();
        assert_eq!(std::fs::metadata(&file).unwrap().modified().unwrap(), before, "nothing new, nothing written");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_program_that_names_itself_oddly_cannot_break_or_flood_the_list() {
        assert_eq!(tidy("a\tb\nc"), "a b c");
        assert_eq!(tidy(&"x".repeat(500)).chars().count(), LINE_MOST);
        let many: Vec<_> = (0..LINES_MOST + 50).map(|n| item(&format!("id{n}"), "n")).collect();
        let text = with("", &many).unwrap();
        assert_eq!(text.lines().count(), LINES_MOST);
        assert!(with(&text, &[item("another", "x")]).is_none(), "a full list takes no more");
        assert_eq!(with("org.x.A\tA", &[item("org.x.B", "B")]).unwrap(), "org.x.A\tA\norg.x.B\tB\n");
    }
}
