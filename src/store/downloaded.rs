//! Files that came from the internet, opened in a box.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::common::files::{self, config_path};

pub const SETTING: &str = "downloads_boxed";

const MOST_SEEN: usize = 40;

const KEPT: &[&str] = &["sound", "x11"];

pub fn on() -> bool {
    files::setting(SETTING) != "no"
}

pub fn seen_path() -> PathBuf {
    config_path("downloads-seen")
}

pub fn as_is_path() -> PathBuf {
    config_path("downloads-as-is")
}

pub fn folder() -> PathBuf {
    let home = files::home();
    let listed = std::fs::read_to_string(files::config_home().join("user-dirs.dirs")).unwrap_or_default();
    files::user_folder_for(&listed, "DOWNLOAD", &home).unwrap_or_else(|| home.join("Downloads"))
}

pub fn kind_of(path: &Path) -> String {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .filter(|e| !e.is_empty() && e.len() <= 12 && e.chars().all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or_else(|| "file".into())
}

pub fn arrived(path: &Path, downloads: &Path) -> bool {
    path.starts_with(downloads)
        || ["user.xdg.origin.url", "user.xdg.referrer.url"].iter().any(|a| super::origin::xattr(path, a).is_some())
}

pub fn boxed_kinds(files: &[PathBuf], arrived: impl Fn(&Path) -> bool, as_is: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for f in files.iter().filter(|f| arrived(f)) {
        let kind = kind_of(f);
        if !as_is.contains(&kind) && !out.contains(&kind) {
            out.push(kind);
        }
    }
    out
}

pub fn kept(allowed: &[String]) -> Vec<String> {
    allowed.iter().filter(|a| KEPT.contains(&a.as_str())).cloned().collect()
}

fn lines(at: &Path) -> Vec<String> {
    files::read_small(at).map(|t| t.lines().map(str::trim).filter(|l| kind_of(Path::new(&format!("x.{l}"))) == *l).map(String::from).collect()).unwrap_or_default()
}

fn write_lines(at: &Path, list: &[String]) -> std::io::Result<()> {
    if let Some(up) = at.parent() {
        std::fs::create_dir_all(up)?;
    }
    files::write_text(at, &list.iter().fold(String::new(), |mut s, l| { let _ = writeln!(s, "{l}"); s }))
}

pub fn seen_in(at: &Path) -> Vec<String> {
    lines(at)
}

pub fn as_is_in(at: &Path) -> Vec<String> {
    lines(at)
}

pub fn remember_in(at: &Path, kinds: &[String]) -> std::io::Result<()> {
    let mut list = seen_in(at);
    let before = list.len();
    for k in kinds {
        if !list.contains(k) {
            list.push(k.clone());
        }
    }
    let skip = list.len().saturating_sub(MOST_SEEN);
    list.drain(..skip);
    if list.len() == before && skip == 0 {
        return Ok(());
    }
    write_lines(at, &list)
}

pub fn set_as_is_in(at: &Path, kind: &str, as_is: bool) -> std::io::Result<()> {
    if let Err(e) = files::read_small(at) {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(e);
        }
    }
    let mut list = as_is_in(at);
    list.retain(|k| k != kind);
    if as_is && kind_of(Path::new(&format!("x.{kind}"))) == kind {
        list.push(kind.to_string());
    }
    write_lines(at, &list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(p: &[&str]) -> Vec<PathBuf> {
        p.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn a_file_in_downloads_or_marked_by_the_browser_arrived() {
        let top = std::env::temp_dir().join(format!("trindle-downloaded-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("Downloads")).unwrap();
        std::fs::create_dir_all(top.join("Documents")).unwrap();
        let (inside, outside, marked) = (top.join("Downloads/a.pdf"), top.join("Documents/b.pdf"), top.join("Documents/c.pdf"));
        for f in [&inside, &outside, &marked] {
            std::fs::write(f, "x").unwrap();
        }
        let downloads = top.join("Downloads");
        assert!(arrived(&inside, &downloads));
        assert!(!arrived(&outside, &downloads), "an ordinary file is not");
        let name = std::ffi::CString::new("user.xdg.origin.url").unwrap();
        let file = std::ffi::CString::new(marked.to_str().unwrap()).unwrap();
        let value = b"https://example.com/c.pdf";
        // SAFETY: both names are NUL-terminated CStrings and the value is a buffer of exactly the length passed.
        let set = unsafe { libc::setxattr(file.as_ptr(), name.as_ptr(), value.as_ptr().cast(), value.len(), 0) };
        if set == 0 {
            assert!(arrived(&marked, &downloads), "marked by the browser");
        }
        assert!(!arrived(&top.join("Downloads2/x.pdf"), &downloads), "a folder that only starts the same way");
        std::fs::remove_dir_all(&top).unwrap();
    }

    #[test]
    fn only_arrived_files_of_types_not_opened_as_normal_make_a_box() {
        let all = paths(&["/home/john/Downloads/a.PDF", "/home/john/Downloads/b.pdf", "/home/john/Downloads/c.png", "/home/john/Documents/d.txt"]);
        let came = |p: &Path| p.starts_with("/home/john/Downloads");
        assert_eq!(boxed_kinds(&all, came, &[]), ["pdf", "png"]);
        assert_eq!(boxed_kinds(&all, came, &["pdf".to_string()]), ["png"]);
        assert!(boxed_kinds(&all, came, &["pdf".to_string(), "png".to_string()]).is_empty(), "every type opened as normal");
        assert!(boxed_kinds(&all[3..], came, &[]).is_empty(), "an ordinary file is no box");
        assert_eq!(kind_of(Path::new("/x/readme")), "file");
        assert_eq!(kind_of(Path::new("/x/a.tar.gz")), "gz");
        assert_eq!(kind_of(Path::new("/x/a.we ird")), "file");
    }

    #[test]
    fn a_box_for_a_downloaded_file_keeps_only_sound_and_its_window() {
        let allowed: Vec<String> = ["network", "sound", "microphone", "files", "screen", "x11", "computer"].iter().map(|s| s.to_string()).collect();
        assert_eq!(kept(&allowed), ["sound", "x11"]);
        assert!(kept(&[]).is_empty());
    }

    #[test]
    fn the_lists_are_kept_plainly_and_a_broken_one_is_not_written_over() {
        let top = std::env::temp_dir().join(format!("trindle-downloaded-lists-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let (seen, as_is) = (top.join("seen"), top.join("as-is"));
        remember_in(&seen, &["pdf".to_string(), "png".to_string()]).unwrap();
        remember_in(&seen, &["pdf".to_string()]).unwrap();
        assert_eq!(seen_in(&seen), ["pdf", "png"]);
        let many: Vec<String> = (0..60).map(|i| format!("t{i}")).collect();
        remember_in(&seen, &many).unwrap();
        assert_eq!(seen_in(&seen).len(), MOST_SEEN);
        assert_eq!(seen_in(&seen).last().map(String::as_str), Some("t59"), "the newest stay");
        set_as_is_in(&as_is, "pdf", true).unwrap();
        set_as_is_in(&as_is, "pdf", true).unwrap();
        set_as_is_in(&as_is, "../x", true).unwrap();
        assert_eq!(as_is_in(&as_is), ["pdf"]);
        set_as_is_in(&as_is, "pdf", false).unwrap();
        assert!(as_is_in(&as_is).is_empty());
        std::fs::write(&as_is, [0xff, 0xfe]).unwrap();
        assert!(set_as_is_in(&as_is, "pdf", true).is_err());
        assert_eq!(std::fs::read(&as_is).unwrap(), [0xff, 0xfe]);
        std::fs::remove_dir_all(&top).unwrap();
    }
}
