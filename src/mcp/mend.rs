//! Small repairs to one Flathub app, inside its own folder `~/.var/app/ID`
//! and nowhere else: one line of a settings file under `config`, and the
//! whole `cache` moved aside into a snapshot and back.

use std::ffi::{CString, OsStr};
use std::io::Read as _;
use std::io::Write as _;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path};

use serde_json::{json, Value};

pub const MOST_FILE: u64 = 256 * 1024;

const MOST_FILES: usize = 200;
const MOST_DEPTH: usize = 4;

pub const MOST_WORD: usize = 200;

const PART: &str = ".trindle-assistant-part";
pub const CACHE: &str = "cache";
const CONFIG: &str = "config";

pub fn trindle_own(id: &str) -> bool {
    id.to_ascii_lowercase().split('.').any(|p| p.contains("trindle"))
}

fn cname(name: &OsStr) -> std::io::Result<CString> {
    CString::new(name.as_bytes()).map_err(|_| std::io::ErrorKind::InvalidInput.into())
}

fn owned(fd: RawFd) -> std::io::Result<OwnedFd> {
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: a descriptor just returned by the kernel, owned by nothing else.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

fn open_at(dir: RawFd, name: &OsStr, flags: i32, mode: libc::c_uint) -> std::io::Result<OwnedFd> {
    let c = cname(name)?;
    // SAFETY: `c` is NUL-terminated and outlives the call; `dir` is held open by the caller.
    owned(unsafe { libc::openat(dir, c.as_ptr(), flags | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NOCTTY, mode) })
}

fn folder_at(dir: RawFd, name: &OsStr) -> std::io::Result<OwnedFd> {
    open_at(dir, name, libc::O_RDONLY | libc::O_DIRECTORY, 0)
}

fn stat(fd: RawFd) -> std::io::Result<libc::stat> {
    let mut st = std::mem::MaybeUninit::<libc::stat>::zeroed();
    // SAFETY: `st` has room for one stat and `fd` is held open by the caller.
    if unsafe { libc::fstat(fd, st.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: fstat succeeded, so it filled the struct in.
    Ok(unsafe { st.assume_init() })
}

fn rename_at(from_dir: RawFd, from: &OsStr, to_dir: RawFd, to: &OsStr) -> std::io::Result<()> {
    let (a, b) = (cname(from)?, cname(to)?);
    // SAFETY: both names are NUL-terminated and outlive the call; both folders are held open.
    if unsafe { libc::renameat(from_dir, a.as_ptr(), to_dir, b.as_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

fn unlink_at(dir: RawFd, name: &OsStr) {
    if let Ok(c) = cname(name) {
        // SAFETY: `c` is NUL-terminated and outlives the call; `dir` is held open.
        unsafe { libc::unlinkat(dir, c.as_ptr(), 0) };
    }
}

fn app_folder(home: &Path, id: &str) -> Result<OwnedFd, String> {
    let dir = crate::common::removal::data_dir(home, id).ok_or("That is not an app id.")?;
    let start = std::fs::File::open(home).map_err(|e| format!("The home folder cannot be read: {e}"))?;
    let mut fd = OwnedFd::from(start);
    for name in [".var", "app", id] {
        fd = folder_at(fd.as_raw_fd(), OsStr::new(name)).map_err(|e| missing(e, &dir))?;
    }
    Ok(fd)
}

fn missing(e: std::io::Error, at: &Path) -> String {
    match e.raw_os_error() {
        Some(libc::ELOOP) | Some(libc::ENOTDIR) => "A folder on the way is a link or not a folder, so it is left alone.".into(),
        Some(libc::ENOENT) => format!("{} is not there.", at.file_name().map(|n| n.to_string_lossy()).unwrap_or_default()),
        _ => format!("It cannot be reached: {e}"),
    }
}

pub fn parts(rel: &str) -> Result<Vec<&OsStr>, String> {
    let bad = || "file must be a settings file's place inside the app's config folder, as list_app_settings gives it.".to_string();
    if rel.is_empty() || rel.len() > MOST_WORD || rel.chars().any(char::is_control) || rel.contains("//") {
        return Err(bad());
    }
    let mut out = Vec::new();
    for c in Path::new(rel).components() {
        match c {
            Component::Normal(n) if n.as_bytes() != PART.as_bytes() => out.push(n),
            _ => return Err(bad()),
        }
    }
    if out.is_empty() || out.len() > MOST_DEPTH || rel.ends_with('/') {
        return Err(bad());
    }
    Ok(out)
}

fn holder<'a>(home: &Path, id: &str, rel: &'a str) -> Result<(OwnedFd, &'a OsStr), String> {
    let names = parts(rel)?;
    let (file, folders) = names.split_last().ok_or("No file.")?;
    let mut fd = app_folder(home, id)?;
    for name in std::iter::once(OsStr::new(CONFIG)).chain(folders.iter().copied()) {
        fd = folder_at(fd.as_raw_fd(), name).map_err(|e| missing(e, Path::new(name)))?;
    }
    Ok((fd, file))
}

pub fn read(home: &Path, id: &str, rel: &str) -> Result<(String, u32), String> {
    let (dir, name) = holder(home, id, rel)?;
    let fd = open_at(dir.as_raw_fd(), name, libc::O_RDONLY | libc::O_NONBLOCK, 0).map_err(|e| missing(e, Path::new(rel)))?;
    read_open(fd)
}

fn read_open(fd: OwnedFd) -> Result<(String, u32), String> {
    let st = stat(fd.as_raw_fd()).map_err(|e| e.to_string())?;
    if st.st_mode & libc::S_IFMT != libc::S_IFREG || st.st_nlink != 1 {
        return Err("That is not a plain settings file of the app's own, so it is left alone.".into());
    }
    if st.st_size as u64 > MOST_FILE {
        return Err(format!("That file is bigger than {} KB, so it is left alone.", MOST_FILE / 1024));
    }
    let mut bytes = Vec::new();
    std::fs::File::from(fd).take(MOST_FILE + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MOST_FILE {
        return Err(format!("That file is bigger than {} KB, so it is left alone.", MOST_FILE / 1024));
    }
    let text = String::from_utf8(bytes).map_err(|_| "That file is not text, so it is left alone.".to_string())?;
    Ok((text, st.st_mode & 0o777))
}

pub fn write(home: &Path, id: &str, rel: &str, text: &str, mode: u32) -> Result<(), String> {
    let (dir, name) = holder(home, id, rel)?;
    let d = dir.as_raw_fd();
    let s = |e: std::io::Error| format!("The file could not be written: {e}");
    let flags = libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL;
    let part = OsStr::new(PART);

    let fd = open_at(d, part, flags, mode & 0o777).or_else(|_| {
        unlink_at(d, part);
        open_at(d, part, flags, mode & 0o777)
    }).map_err(s)?;
    let mut f = std::fs::File::from(fd);
    let done = f.write_all(text.as_bytes()).and_then(|()| f.sync_all()).and_then(|()| rename_at(d, part, d, name));
    if done.is_err() {
        unlink_at(d, part);
    }
    done.map_err(s)
}

fn heading(line: &str) -> Option<&str> {
    let t = line.trim();
    t.strip_prefix('[')?.strip_suffix(']').map(str::trim)
}

fn key_of(line: &str) -> Option<&str> {
    let t = line.trim_start();
    if t.starts_with('#') || t.starts_with(';') || heading(line).is_some() {
        return None;
    }
    t.split_once('=').map(|(k, _)| k.trim())
}

pub fn value_of(text: &str, section: &str, key: &str) -> Option<String> {
    let mut here = "";
    for line in text.lines() {
        if let Some(h) = heading(line) {
            here = h;
        } else if here == section && key_of(line) == Some(key) {
            return line.split_once('=').map(|(_, v)| v.trim().to_string());
        }
    }
    None
}

pub fn edited(text: &str, section: &str, key: &str, value: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut here = "".to_string();
    let mut last_in = (section.is_empty()).then_some(0usize);
    for (i, line) in lines.iter().enumerate() {
        if let Some(h) = heading(line) {
            here = h.to_string();
            if here == section {
                last_in = Some(i + 1);
            }
            continue;
        }
        if here == section {
            if key_of(line) == Some(key) {
                let (lead, rest) = line.split_once('=').unwrap_or((line, ""));
                let gap = if rest.starts_with(' ') { " " } else { "" };
                lines[i] = format!("{lead}={gap}{value}");
                return joined(lines, text);
            }
            if !line.trim().is_empty() {
                last_in = Some(i + 1);
            }
        }
    }
    match last_in {
        Some(at) => lines.insert(at, format!("{key}={value}")),
        None => {
            if lines.last().is_some_and(|l| !l.trim().is_empty()) {
                lines.push(String::new());
            }
            lines.push(format!("[{section}]"));
            lines.push(format!("{key}={value}"));
        }
    }
    joined(lines, text)
}

fn joined(lines: Vec<String>, was: &str) -> String {
    let mut out = lines.join("\n");
    if was.is_empty() || was.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn check_line(section: &str, key: &str, value: &str) -> Result<(), String> {
    let plain = |w: &str| w.len() <= MOST_WORD && !w.chars().any(char::is_control);
    if !plain(section) || section.contains(['[', ']']) {
        return Err("section must be a section's name, without brackets.".into());
    }
    if key.is_empty() || !plain(key) || key.trim() != key || key.contains(['=', '[', ']']) || key.starts_with(['#', ';']) {
        return Err("key must be one setting's name.".into());
    }
    if !plain(value) {
        return Err("The value must be one short line of text.".into());
    }
    Ok(())
}

pub fn names(home: &Path, id: &str) -> Result<Value, String> {
    let root = folder_at(app_folder(home, id)?.as_raw_fd(), OsStr::new(CONFIG)).map_err(|e| missing(e, Path::new(CONFIG)))?;
    let mut found = Vec::new();
    walk(root, "", 1, &mut found);
    Ok(json!(found))
}

fn walk(dir: OwnedFd, at: &str, depth: usize, found: &mut Vec<Value>) {
    // SAFETY: a folder descriptor of our own, dup'd so ReadDir can own its copy.
    let Ok(copy) = owned(unsafe { libc::dup(dir.as_raw_fd()) }) else { return };
    let Ok(list) = std::fs::read_dir(format!("/proc/self/fd/{}", copy.as_raw_fd())) else { return };
    let mut entries: Vec<_> = list.flatten().map(|e| e.file_name()).collect();
    entries.sort();
    for name in entries {
        if found.len() >= MOST_FILES {
            return;
        }
        let rel = if at.is_empty() { name.to_string_lossy().into_owned() } else { format!("{at}/{}", name.to_string_lossy()) };
        if parts(&rel).is_err() {
            continue;
        }
        if let Ok(sub) = folder_at(dir.as_raw_fd(), &name) {
            if depth < MOST_DEPTH {
                walk(sub, &rel, depth + 1, found);
            }
        } else if let Ok(fd) = open_at(dir.as_raw_fd(), &name, libc::O_RDONLY | libc::O_NONBLOCK, 0) {
            if let Ok((text, _)) = read_open(fd) {
                found.push(json!({ "file": rel, "sections": sections(&text) }));
            }
        }
    }
}

fn sections(text: &str) -> Value {
    let mut out = serde_json::Map::new();
    let mut here = String::new();
    for line in text.lines() {
        if let Some(h) = heading(line) {
            here = h.to_string();
        } else if let Some(k) = key_of(line).filter(|k| !k.is_empty()) {
            let keys = out.entry(here.clone()).or_insert_with(|| json!([]));
            if let Some(a) = keys.as_array_mut().filter(|a| a.len() < MOST_FILES) {
                a.push(json!(k));
            }
        }
    }
    Value::Object(out)
}

pub fn has_cache(home: &Path, id: &str) -> Result<bool, String> {
    let app = app_folder(home, id)?;
    match folder_at(app.as_raw_fd(), OsStr::new(CACHE)) {
        Ok(_) => Ok(true),
        Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(false),
        Err(e) => Err(missing(e, Path::new(CACHE))),
    }
}

pub fn move_cache(home: &Path, id: &str, to: &Path, name: &str) -> Result<(), String> {
    let app = app_folder(home, id)?;
    let held = std::fs::File::open(to).map_err(|e| e.to_string())?;
    let cache = OsStr::new(CACHE);
    rename_at(app.as_raw_fd(), cache, held.as_raw_fd(), OsStr::new(name)).map_err(|e| match e.raw_os_error() {
        Some(libc::EXDEV) => "The cache is on another disk from the snapshots, so it is left alone.".to_string(),
        _ => missing(e, Path::new(CACHE)),
    })?;
    fresh(&app)
}

pub fn put_cache_back(home: &Path, id: &str, from: &Path, name: &str, since: &str) -> Result<(), String> {
    let app = app_folder(home, id)?;
    let held = std::fs::File::open(from).map_err(|e| e.to_string())?;
    let kept = folder_at(held.as_raw_fd(), OsStr::new(name)).map_err(|e| missing(e, Path::new(name)))?;
    drop(kept);
    let cache = OsStr::new(CACHE);
    match rename_at(app.as_raw_fd(), cache, held.as_raw_fd(), OsStr::new(since)) {
        Err(e) if e.raw_os_error() != Some(libc::ENOENT) => return Err(format!("The cache made since could not be set aside: {e}")),
        _ => {}
    }
    rename_at(held.as_raw_fd(), OsStr::new(name), app.as_raw_fd(), cache).map_err(|e| e.to_string())
}

fn fresh(app: &OwnedFd) -> Result<(), String> {
    let c = cname(OsStr::new(CACHE)).map_err(|e| e.to_string())?;
    // SAFETY: `c` is NUL-terminated and outlives the call; the folder is held open.
    if unsafe { libc::mkdirat(app.as_raw_fd(), c.as_ptr(), 0o700) } != 0 {
        let e = std::io::Error::last_os_error();
        if e.raw_os_error() != Some(libc::EEXIST) {
            return Err(format!("The cache was cleared, and a new empty one could not be made: {e}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
