//! What Text keeps between runs: its settings beside Trindle's own, and in
//! Trindle's state folder the recent files, the open tabs, and the text of
//! any tab not yet saved to a file. Everything takes its folder, so tests
//! never touch the real ones.

use crate::toolkit::t;
use std::fs;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

use crate::common::files;

pub const RECENT_MAX: usize = 12;
pub const SIZE_MIN: f32 = 8.0;
pub const SIZE_MAX: f32 = 48.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {

    pub reopen: bool,
    pub wrap: bool,
    pub numbers: bool,

    pub font: String,

    pub size: f32,
    pub spell: bool,

    pub lang: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            reopen: true,
            wrap: true,
            numbers: true,
            font: String::new(),
            size: 15.0,
            spell: true,
            lang: String::new(),
        }
    }
}

pub fn settings_file() -> PathBuf {
    files::config_path("text")
}

pub fn words_file() -> PathBuf {
    files::config_path("text-words")
}

pub fn state_dir() -> PathBuf {
    let base = match std::env::var_os("XDG_STATE_HOME") {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => files::home().join(".local/state"),
    };
    base.join("trindle/text")
}

fn yes(v: Option<&String>, default: bool) -> bool {
    v.map(|v| v == "true").unwrap_or(default)
}

impl Settings {
    pub fn load(path: &Path) -> Settings {
        let keys = files::read_env(path);
        let d = Settings::default();
        Settings {
            reopen: keys.get("start").map(|v| v != "blank").unwrap_or(d.reopen),
            wrap: yes(keys.get("wrap"), d.wrap),
            numbers: yes(keys.get("numbers"), d.numbers),
            font: keys.get("font").cloned().unwrap_or(d.font),
            size: keys.get("size").and_then(|v| v.parse().ok()).map(|s: f32| s.clamp(SIZE_MIN, SIZE_MAX)).unwrap_or(d.size),
            spell: yes(keys.get("spell"), d.spell),
            lang: keys.get("language").filter(|v| !v.is_empty()).cloned().unwrap_or(d.lang),
        }
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let size = format!("{}", self.size);
        let flag = |b: bool| if b { "true" } else { "false" };
        files::update_env(
            path,
            &[
                ("start", Some(if self.reopen { "last" } else { "blank" })),
                ("wrap", Some(flag(self.wrap))),
                ("numbers", Some(flag(self.numbers))),
                ("font", Some(&self.font)),
                ("size", Some(&size)),
                ("spell", Some(flag(self.spell))),
                ("language", Some(&self.lang)),
            ],
        )
    }
}

fn ensure(dir: &Path) -> std::io::Result<()> {
    fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)
}

const ROOT: char = '/';
const ESCAPED: char = '\\';

fn line_of(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    let bytes = path.as_os_str().as_bytes();
    if let Ok(plain) = std::str::from_utf8(bytes) {
        if plain.starts_with(ROOT) && !plain.chars().any(char::is_control) {
            return plain.to_string();
        }
    }
    let mut line = String::from(ESCAPED);
    for chunk in bytes.utf8_chunks() {
        for c in chunk.valid().chars() {
            match c {
                ESCAPED => line.push_str("\\\\"),
                c if c.is_control() => line.extend(c.to_string().bytes().map(|b| format!("\\x{b:02x}"))),
                c => line.push(c),
            }
        }
        line.extend(chunk.invalid().iter().map(|b| format!("\\x{b:02x}")));
    }
    line
}

fn path_of(line: &str) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    let Some(written) = line.strip_prefix(ESCAPED) else { return PathBuf::from(line) };
    let (from, mut bytes, mut at) = (written.as_bytes(), Vec::with_capacity(written.len()), 0);
    while at < from.len() {
        let hex = |i: usize| from.get(i).and_then(|b| (*b as char).to_digit(16));
        match (from[at], from.get(at + 1)) {
            (b'\\', Some(b'\\')) => {
                bytes.push(b'\\');
                at += 2;
            }
            (b'\\', Some(b'x')) if hex(at + 2).is_some() && hex(at + 3).is_some() => {
                bytes.push((hex(at + 2).unwrap_or(0) * 16 + hex(at + 3).unwrap_or(0)) as u8);
                at += 4;
            }
            (b, _) => {
                bytes.push(b);
                at += 1;
            }
        }
    }
    PathBuf::from(std::ffi::OsString::from_vec(bytes))
}

pub fn recent_damaged() -> &'static str {
    t!("text-recent-files")
}

pub fn recent(dir: &Path) -> Vec<PathBuf> {
    crate::common::damaged::read(&dir.join("recent"), recent_damaged(), |l| !l.contains('\0')).lines().filter(|l| !l.is_empty()).map(path_of).collect()
}

pub fn add_recent(dir: &Path, path: &Path) -> std::io::Result<Vec<PathBuf>> {

    let mut list = recent(dir);
    crate::common::damaged::may_write(&dir.join("recent"), recent_damaged())?;
    list.retain(|p| p != path);
    list.insert(0, path.to_path_buf());
    list.truncate(RECENT_MAX);
    write_recent(dir, &list)?;
    Ok(list)
}

pub fn write_recent(dir: &Path, list: &[PathBuf]) -> std::io::Result<()> {
    ensure(dir)?;
    let mut text = String::new();
    for p in list {
        text.push_str(&line_of(p));
        text.push('\n');
    }
    files::write_text(&dir.join("recent"), &text)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: u64,
    pub path: Option<PathBuf>,

    pub enc: String,
    pub bom: bool,
    pub crlf: bool,

    pub unsaved: bool,
    pub caret: usize,
    pub markdown: bool,
}

pub fn write_list(dir: &Path, tabs: &[Entry], active: usize) -> std::io::Result<()> {
    ensure(dir)?;

    files::read_small(&dir.join("session"))?;
    let mut out = format!("active\t{active}\n");
    for t in tabs {
        let path = t.path.as_deref().map(line_of).unwrap_or_default();
        out.push_str(&format!(
            "tab\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            t.id, t.enc, t.bom as u8, t.crlf as u8, t.unsaved as u8, t.caret, t.markdown as u8, path
        ));
    }
    files::write_text(&dir.join("session"), &out)?;
    tidy(dir, tabs)
}

pub fn read_list(dir: &Path) -> (Vec<Entry>, usize) {
    let mut tabs = Vec::new();
    let mut active = 0;
    for line in files::read_text(dir.join("session")).lines() {
        let f: Vec<&str> = line.splitn(9, '\t').collect();
        match f.as_slice() {
            ["active", n] => active = n.parse().unwrap_or(0),
            ["tab", id, enc, bom, crlf, unsaved, caret, md, path] => {
                let Ok(id) = id.parse() else { continue };
                tabs.push(Entry {
                    id,
                    path: (!path.is_empty()).then(|| path_of(path)),
                    enc: enc.to_string(),
                    bom: *bom == "1",
                    crlf: *crlf == "1",
                    unsaved: *unsaved == "1",
                    caret: caret.parse().unwrap_or(0),
                    markdown: *md == "1",
                })
            }
            _ => {}
        }
    }
    (tabs, active)
}

fn text_file(dir: &Path, id: u64) -> PathBuf {
    dir.join(format!("kept-{id}.txt"))
}

pub fn keep_text(dir: &Path, id: u64, text: &str) -> std::io::Result<()> {
    ensure(dir)?;
    files::write_text(&text_file(dir, id), text)
}

pub fn kept_text(dir: &Path, id: u64) -> Option<String> {
    crate::common::files::read_string(text_file(dir, id)).ok()
}

fn tidy(dir: &Path, tabs: &[Entry]) -> std::io::Result<()> {
    let wanted: Vec<PathBuf> = tabs.iter().filter(|t| t.unsaved).map(|t| text_file(dir, t.id)).collect();
    for e in fs::read_dir(dir)?.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with("kept-") && name.ends_with(".txt") && !wanted.contains(&e.path()) {
            let _ = fs::remove_file(e.path());
        }
    }
    Ok(())
}

pub fn restore(dir: &Path, reopen: bool) -> (Vec<Entry>, usize) {
    let (tabs, active) = read_list(dir);
    let all = tabs.len();
    let kept: Vec<Entry> = tabs
        .into_iter()
        .filter(|t| reopen || t.unsaved)
        .filter(|t| if t.unsaved { text_file(dir, t.id).is_file() } else { t.path.as_ref().is_some_and(|p| p.is_file()) })
        .collect();
    let active = if kept.len() == all { active.min(kept.len().saturating_sub(1)) } else { 0 };
    (kept, active)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stamp {
    modified: Option<std::time::SystemTime>,
    len: u64,
    inode: u64,
}

pub fn stamp(path: &Path) -> Option<Stamp> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::metadata(path).ok()?;
    Some(Stamp { modified: m.modified().ok(), len: m.len(), inode: m.ino() })
}

pub fn read_document(path: &Path) -> std::io::Result<Vec<u8>> {
    if std::fs::metadata(path).is_ok_and(|m| m.len() > LARGEST) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "the file is too big to open"));
    }
    files::read_file(path)
}

pub const LARGEST: u64 = 50 * 1024 * 1024;

#[cfg(test)]
thread_local! {

    static BREAKS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };

    pub(super) static CUT: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

pub fn write_document(path: &Path, bytes: &[u8], state: &Path) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    let real = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let old = match fs::metadata(&real) {
        Ok(old) if old.is_file() => old,

        Ok(_) => return fs::write(&real, bytes),

        Err(_) if fs::symlink_metadata(path).is_ok() => {
            let mut file = fs::File::create(path)?;
            file.write_all(bytes)?;
            return file.sync_all();
        }
        Err(_) => return files::write_whole(&real, bytes),
    };
    if old.nlink() == 1 {
        match files::write_whole_owned(&real, bytes) {
            Err(e) if no_new_file(&e) => {}
            done => return done,
        }
    }

    let mut file = fs::OpenOptions::new().write(true).open(&real)?;
    let before = read_document(&real)?;
    let kept = Kept::keep(state, &real, &before, bytes)?;
    #[cfg(test)]
    let (breaks, cut) = (BREAKS.get(), CUT.get());
    #[cfg(not(test))]
    let (breaks, cut) = (false, 0);
    if cut == 1 {

        file.write_all(&bytes[..bytes.len() / 2])?;
        return Err(std::io::Error::other("stopped half way"));
    }
    let done = put(&mut file, bytes, breaks);
    if cut == 2 {

        return Err(std::io::Error::other("stopped before tidying"));
    }
    match &done {
        Ok(()) => kept.forget(),
        Err(e) => match put(&mut file, &before, false) {
            Ok(()) => kept.forget(),

            Err(_) => return Err(std::io::Error::new(e.kind(), t!("text-kept-before", reason = e.to_string(), path = kept.old().display().to_string()))),
        },
    }
    done
}

fn put(file: &mut fs::File, words: &[u8], breaks: bool) -> std::io::Result<()> {
    use std::io::{Seek, Write};
    file.rewind()?;
    if breaks {
        file.write_all(&words[..words.len() / 2])?;
        return Err(std::io::Error::from_raw_os_error(libc::ENOSPC));
    }
    file.write_all(words)?;
    file.set_len(words.len() as u64)?;
    file.sync_all()
}

const KEPT: &str = "before-save-";

struct Kept {
    stem: PathBuf,
}

impl Kept {
    fn part(stem: &Path, ending: &str) -> PathBuf {
        let mut name = stem.as_os_str().to_os_string();
        name.push(ending);
        PathBuf::from(name)
    }

    fn keep(state: &Path, document: &Path, before: &[u8], new: &[u8]) -> std::io::Result<Kept> {
        use std::os::unix::ffi::OsStrExt;
        use std::sync::atomic::{AtomicU64, Ordering};
        static SAVES: AtomicU64 = AtomicU64::new(0);
        ensure(state)?;
        let kept = Kept { stem: state.join(format!("{KEPT}{}-{}", std::process::id(), SAVES.fetch_add(1, Ordering::Relaxed))) };

        let written = files::write_whole(&Kept::part(&kept.stem, WHERE), document.as_os_str().as_bytes())
            .and_then(|()| files::write_whole(&Kept::part(&kept.stem, NEW), new))
            .and_then(|()| files::write_whole(&kept.old(), before));
        if written.is_err() {
            kept.forget();
        }
        written.map(|()| kept)
    }

    fn old(&self) -> PathBuf {
        Kept::part(&self.stem, OLD)
    }

    fn forget(&self) {
        for ending in [OLD, NEW, WHERE] {
            let _ = fs::remove_file(Kept::part(&self.stem, ending));
        }
    }
}

const WHERE: &str = ".where";
const NEW: &str = ".new";
const OLD: &str = ".old";

#[derive(Debug, PartialEq)]
pub struct Mended {

    pub document: PathBuf,

    pub words: Option<Vec<u8>>,

    pub problem: Option<String>,
}

pub fn mend(state: &Path) -> Vec<Mended> {
    mend_as(state, |pid| {
        let ours = pid == std::process::id();
        ours || fs::read(format!("/proc/{pid}/cmdline")).is_ok_and(|c| String::from_utf8_lossy(&c).contains("trindle-text"))
    })
}

pub fn mend_as(state: &Path, running: impl Fn(u32) -> bool) -> Vec<Mended> {
    let Ok(entries) = fs::read_dir(state) else { return Vec::new() };

    let mut saves = std::collections::BTreeSet::new();
    for e in entries.flatten() {
        let name = e.file_name();
        let Some(rest) = name.to_str().and_then(|n| n.strip_prefix(KEPT)) else { continue };
        let Some(stem) = [WHERE, NEW, OLD].iter().find_map(|ending| rest.strip_suffix(ending)) else { continue };
        let Some((Ok(pid), Ok(n))) = stem.split_once('-').map(|(pid, n)| (pid.parse::<u32>(), n.parse::<u64>())) else { continue };
        saves.insert((pid, n));
    }
    saves.into_iter().filter(|(pid, _)| !running(*pid))
        .filter_map(|(pid, n)| Kept { stem: state.join(format!("{KEPT}{pid}-{n}")) }.mend()).collect()
}

impl Kept {

    fn mend(&self) -> Option<Mended> {
        use std::os::unix::ffi::OsStringExt;
        let (at, new, old) = (Kept::part(&self.stem, WHERE), Kept::part(&self.stem, NEW), self.old());

        let place = crate::common::files::read_file(&at).ok()?;
        if !old.exists() {
            let _ = fs::remove_file(&new);
            let _ = fs::remove_file(&at);
            return None;
        }
        let document = PathBuf::from(std::ffi::OsString::from_vec(place));
        let shown = document.display().to_string();
        let words = crate::common::files::read_file(&new).ok();
        let (Ok(before), Some(words)) = (crate::common::files::read_file(&old), words.clone()) else {
            return self.leave(document, words, t!("text-saved-unreadable", name = &shown));
        };
        let most = before.len().max(words.len()) as u64;
        let Some((named, now)) = document_now(&document, most) else {
            return self.leave(document, Some(words), t!("text-saved-changed", name = &shown));
        };
        if now == words {
            self.forget();
            return Some(Mended { document, words: None, problem: None });
        }
        if now == before {
            self.forget();
            return Some(Mended { document, words: Some(words), problem: None });
        }
        let least = before.len().min(words.len());
        let cut_short = now.len() >= least && now.iter().enumerate().all(|(i, b)| before.get(i) == Some(b) || words.get(i) == Some(b));
        if !cut_short {
            return self.leave(document, Some(words), t!("text-saved-changed", name = &shown));
        }
        let back = fs::OpenOptions::new().write(true).open(files::through(&named)).and_then(|mut file| put(&mut file, &before, false));
        match back {
            Ok(()) => {
                self.forget();
                Some(Mended { document, words: Some(words), problem: None })
            }

            Err(e) => Some(Mended {
                document, words: Some(words),
                problem: Some(t!("text-saved-not-put-back", name = &shown, reason = e.to_string(), path = self.old().display().to_string())),
            }),
        }
    }

    fn leave(&self, document: PathBuf, words: Option<Vec<u8>>, why: String) -> Option<Mended> {
        let _ = fs::remove_file(Kept::part(&self.stem, NEW));
        let _ = fs::remove_file(Kept::part(&self.stem, WHERE));
        Some(Mended { document, words, problem: Some(t!("text-kept-before-that", why = why, path = self.old().display().to_string())) })
    }
}

fn document_now(document: &Path, most: u64) -> Option<(fs::File, Vec<u8>)> {
    use std::io::Read;
    let named = match files::open_named(document, None, None) {
        Ok(files::Named::File(named, meta)) if meta.len() <= most => named,
        _ => return None,
    };
    let mut now = Vec::new();
    files::readable(&named).ok()?.take(most + 1).read_to_end(&mut now).ok()?;
    Some((named, now))
}

fn no_new_file(e: &std::io::Error) -> bool {
    matches!(e.raw_os_error(), Some(libc::EACCES | libc::EPERM | libc::EROFS | libc::EBUSY | libc::EXDEV | libc::EOPNOTSUPP))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-text-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn names(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(dir).map(|d| d.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect()).unwrap_or_default();
        names.sort();
        names
    }

    #[test]
    fn a_save_that_stops_half_way_leaves_the_old_words_whole() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let dir = scratch("save-whole");
        let state = dir.join("state");
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("letter.txt");
        fs::write(&file, "Dear John Smith,\nthe old words.\n").unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o640)).unwrap();
        let first = fs::metadata(&file).unwrap().ino();
        write_document(&file, b"Dear John Smith,\nthe new words.\n", &state).unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "Dear John Smith,\nthe new words.\n");
        let meta = fs::metadata(&file).unwrap();
        assert_ne!(meta.ino(), first, "the document was written over where it lay");
        assert_eq!(meta.mode() & 0o777, 0o640, "it keeps who may read it");
        let before = names(&dir);
        files::FULL.set(true);
        let got = write_document(&file, b"words that are never to be seen, not even the first half of them\n", &state);
        files::FULL.set(false);
        assert_eq!(got.map_err(|e| e.raw_os_error()), Err(Some(libc::ENOSPC)));
        assert_eq!(fs::read_to_string(&file).unwrap(), "Dear John Smith,\nthe new words.\n", "the old words were lost");
        assert_eq!(names(&dir), before, "a half-written file was left");

        let groups = std::process::Command::new("id").arg("-G").output().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
        if let Some(other) = groups.split_whitespace().filter_map(|g| g.parse::<u32>().ok()).find(|g| *g != meta.gid()) {
            std::os::unix::fs::chown(&file, None, Some(other)).unwrap();
            write_document(&file, b"Dear John Smith,\nthe new words.\n", &state).unwrap();
            assert_eq!(fs::metadata(&file).unwrap().gid(), other, "it was taken from its group");
        }

        write_document(&dir.join("new.txt"), b"first words\n", &state).unwrap();
        assert_eq!(fs::read_to_string(dir.join("new.txt")).unwrap(), "first words\n");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_save_through_a_link_keeps_the_link() {
        use std::os::unix::fs::MetadataExt;
        let dir = scratch("save-link");
        let state = dir.join("state");
        fs::create_dir_all(dir.join("elsewhere")).unwrap();
        let real = dir.join("elsewhere/notes.txt");
        let link = dir.join("notes.txt");
        fs::write(&real, "old\n").unwrap();
        std::os::unix::fs::symlink(&real, &link).unwrap();
        let first = fs::metadata(&real).unwrap().ino();
        write_document(&link, b"new\n", &state).unwrap();
        assert!(fs::symlink_metadata(&link).unwrap().file_type().is_symlink(), "the link was replaced by a file");
        assert_eq!(fs::read_to_string(&real).unwrap(), "new\n");
        assert_ne!(fs::metadata(&real).unwrap().ino(), first, "what the link leads to was written over where it lay");
        assert_eq!(names(&dir), ["elsewhere", "notes.txt"], "something was left beside the link");
        assert_eq!(names(&dir.join("elsewhere")), ["notes.txt"]);
        let nowhere = dir.join("later.txt");
        std::os::unix::fs::symlink(dir.join("elsewhere/later.txt"), &nowhere).unwrap();
        write_document(&nowhere, b"made\n", &state).unwrap();
        assert!(fs::symlink_metadata(&nowhere).unwrap().file_type().is_symlink());
        assert_eq!(fs::read_to_string(dir.join("elsewhere/later.txt")).unwrap(), "made\n");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_save_in_a_folder_that_takes_no_new_file_never_loses_the_old_words() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let dir = scratch("save-fixed");
        let state = dir.join("state");
        let folder = dir.join("fixed");
        fs::create_dir_all(&folder).unwrap();
        let file = folder.join("list.txt");
        fs::write(&file, "milk\neggs\nbread\n").unwrap();
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o500)).unwrap();
        assert!(fs::write(folder.join("probe"), "").is_err(), "{}", files::tests::NOT_AS_ROOT);
        let first = fs::metadata(&file).unwrap().ino();
        let saved = write_document(&file, b"tea\n", &state);
        let after = fs::read_to_string(&file).unwrap();
        BREAKS.set(true);
        let broken = write_document(&file, b"a much longer list that stops half way through\n", &state);
        BREAKS.set(false);
        let kept = fs::read_to_string(&file).unwrap();
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
        saved.unwrap();
        assert_eq!(after, "tea\n", "the longer old words were left after the new");
        assert_eq!(fs::metadata(&file).unwrap().ino(), first);
        assert!(broken.is_err());
        assert_eq!(kept, "tea\n", "a save that stopped half way lost the old words");
        assert_eq!(names(&folder), ["list.txt"]);
        assert!(names(&state).is_empty(), "the copy kept for safety was left: {:?}", names(&state));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_save_keeps_a_document_s_other_names() {
        use std::os::unix::fs::MetadataExt;
        let dir = scratch("save-names");
        let state = dir.join("state");
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("report.txt");
        fs::write(&file, "draft one\n").unwrap();
        fs::hard_link(&file, dir.join("same-report.txt")).unwrap();
        write_document(&file, b"draft two\n", &state).unwrap();
        assert_eq!(fs::read_to_string(dir.join("same-report.txt")).unwrap(), "draft two\n", "the other name was left with the old words");
        assert_eq!(fs::metadata(&file).unwrap().nlink(), 2);
        BREAKS.set(true);
        let broken = write_document(&file, b"draft three, which is never finished\n", &state);
        BREAKS.set(false);
        assert!(broken.is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), "draft two\n", "a save that stopped half way lost the old words");
        assert_eq!(fs::read_to_string(dir.join("same-report.txt")).unwrap(), "draft two\n");
        assert!(names(&state).is_empty(), "{:?}", names(&state));
        fs::remove_dir_all(&dir).ok();
    }

    fn two_names(dir: &Path, words: &str) -> PathBuf {
        fs::create_dir_all(dir).unwrap();
        let file = dir.join("report.txt");
        fs::write(&file, words).unwrap();
        fs::hard_link(&file, dir.join("same-report.txt")).unwrap();
        file
    }

    #[test]
    fn a_save_cut_short_where_the_document_lies_is_put_right_at_the_next_start() {
        let dir = scratch("save-cut");
        let state = dir.join("state");
        let old = "draft one, which is the longer of the two by some way\n";
        let new = b"draft two\nand a second line\n";
        let file = two_names(&dir, old);
        CUT.set(1);
        let cut = write_document(&file, new, &state);
        CUT.set(0);
        assert!(cut.is_err());
        let torn = fs::read(&file).unwrap();
        assert!(torn != old.as_bytes() && torn != new, "the save was not cut short part way");

        assert_eq!(mend_as(&state, |_| true), []);
        assert_eq!(fs::read(&file).unwrap(), torn);

        let real = fs::canonicalize(&file).unwrap();
        assert_eq!(mend_as(&state, |_| false), [Mended { document: real.clone(), words: Some(new.to_vec()), problem: None }]);
        assert_eq!(fs::read_to_string(&file).unwrap(), old, "the document was left half written");
        assert_eq!(fs::read_to_string(dir.join("same-report.txt")).unwrap(), old);
        assert!(names(&state).is_empty(), "{:?}", names(&state));
        assert_eq!(mend_as(&state, |_| false), [], "it is put right once");

        CUT.set(2);
        let _ = write_document(&file, new, &state);
        CUT.set(0);
        assert_eq!(names(&state).len(), 3, "{:?}", names(&state));
        assert_eq!(mend_as(&state, |_| false), [Mended { document: real.clone(), words: None, problem: None }]);
        assert_eq!(fs::read(&file).unwrap(), new);
        assert!(names(&state).is_empty(), "{:?}", names(&state));

        fs::write(&file, old).unwrap();
        CUT.set(1);
        let _ = write_document(&file, new, &state);
        CUT.set(0);
        fs::write(&file, "something else altogether, written by another program\n").unwrap();
        let found = mend_as(&state, |_| false);
        assert_eq!(found.len(), 1);
        assert_eq!((found[0].document.clone(), found[0].words.clone()), (real.clone(), Some(new.to_vec())));
        let left = names(&state);
        assert_eq!(left.len(), 1, "{left:?}");
        let said = found[0].problem.clone().expect("it is said");
        assert!(said.contains(&state.join(&left[0]).display().to_string()), "{said}");
        assert_eq!(fs::read_to_string(state.join(&left[0])).unwrap(), old, "the words it held were not kept");
        assert_eq!(fs::read_to_string(&file).unwrap(), "something else altogether, written by another program\n");
        assert_eq!(mend_as(&state, |_| false), [], "it is said once");
        assert_eq!(names(&state), left, "the copy was taken away");

        fs::write(&file, old).unwrap();
        fs::write(state.join("before-save-7-0.where"), real.as_os_str().as_encoded_bytes()).unwrap();
        fs::write(state.join("before-save-7-0.new"), new).unwrap();
        assert_eq!(mend_as(&state, |_| false), []);
        assert_eq!(names(&state), left);
        assert_eq!(fs::read_to_string(&file).unwrap(), old);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_path_with_a_line_break_or_bytes_that_are_not_text_is_kept_as_itself() {
        use std::os::unix::ffi::OsStrExt;
        let dir = scratch("paths");
        let odd = [
            PathBuf::from("/home/john/draft\nv2.txt"),
            PathBuf::from("/home/john/draft\r\nv3.txt"),
            PathBuf::from("/home/john/a file\twith a tab.txt"),
            PathBuf::from("/home/john/back\\nslash.txt"),
            PathBuf::from("\\home\\john\\notes.txt"),
            PathBuf::from("notes beside.txt"),
            PathBuf::from(std::ffi::OsStr::from_bytes(b"/home/john/caf\xe9 \\x41 \xff.txt")),
            PathBuf::from("/home/john/plain.txt"),
        ];
        let tabs: Vec<Entry> = odd.iter().enumerate().map(|(i, p)| Entry {
            id: i as u64 + 1, path: Some(p.clone()), enc: "UTF-8".into(), bom: false, crlf: false, unsaved: true, caret: 0, markdown: false,
        }).collect();
        write_list(&dir, &tabs, 0).unwrap();
        assert_eq!(read_list(&dir).0, tabs);
        write_recent(&dir, &odd).unwrap();
        assert_eq!(recent(&dir), odd);

        fs::write(dir.join("recent"), "/home/john/one.txt\n/home/john/a\\nb.txt\n").unwrap();
        assert_eq!(recent(&dir), [PathBuf::from("/home/john/one.txt"), PathBuf::from("/home/john/a\\nb.txt")]);
        fs::write(dir.join("session"), "active\t0\ntab\t4\tUTF-8\t0\t0\t0\t0\t0\t/home/john/a\\nb.txt\n").unwrap();
        assert_eq!(read_list(&dir).0[0].path, Some(PathBuf::from("/home/john/a\\nb.txt")));

        write_recent(&dir, &[PathBuf::from("/home/john/plain.txt")]).unwrap();
        assert_eq!(fs::read_to_string(dir.join("recent")).unwrap(), "/home/john/plain.txt\n");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn settings_round_trip() {
        let dir = scratch("settings");
        let file = dir.join("text");
        assert_eq!(Settings::load(&file), Settings::default());
        let s = Settings { reopen: false, wrap: false, font: "DejaVu Sans Mono".into(), size: 18.0, ..Settings::default() };
        s.save(&file).unwrap();
        assert_eq!(Settings::load(&file), s);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn recent_keeps_the_newest_first_without_repeats() {
        let dir = scratch("recent");
        for i in 0..15 {
            add_recent(&dir, Path::new(&format!("/tmp/f{i}"))).unwrap();
        }
        let list = add_recent(&dir, Path::new("/tmp/f5")).unwrap();
        assert_eq!(list.len(), RECENT_MAX);
        assert_eq!(list[0], Path::new("/tmp/f5"));
        assert_eq!(list[1], Path::new("/tmp/f14"));
        assert_eq!(list.iter().filter(|p| *p == Path::new("/tmp/f5")).count(), 1);
        assert_eq!(recent(&dir), list);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn session_restore() {
        let dir = scratch("session");
        let file = dir.join("a file\twith a tab.txt");
        ensure(&dir).unwrap();
        fs::write(&file, "saved").unwrap();
        let saved = Entry {
            id: 3,
            path: Some(file.clone()),
            enc: "windows-1252".into(),
            bom: false,
            crlf: true,
            unsaved: false,
            caret: 2,
            markdown: true,
        };
        let draft = Entry { id: 7, path: None, enc: "UTF-8".into(), bom: false, crlf: false, unsaved: true, caret: 5, markdown: false };
        keep_text(&dir, 7, "draft text").unwrap();
        keep_text(&dir, 9, "left over").unwrap();
        write_list(&dir, &[saved.clone(), draft.clone()], 1).unwrap();
        assert_eq!(read_list(&dir), (vec![saved.clone(), draft.clone()], 1));
        assert!(kept_text(&dir, 9).is_none(), "text no tab needs is removed");
        assert_eq!(kept_text(&dir, 7).as_deref(), Some("draft text"));
        assert_eq!(restore(&dir, true), (vec![saved, draft.clone()], 1));
        assert_eq!(restore(&dir, false), (vec![draft], 0));
        fs::remove_file(&file).unwrap();
        assert_eq!(restore(&dir, true).0.len(), 1, "a file gone since is not reopened");
        let _ = fs::remove_dir_all(&dir);
    }
}
