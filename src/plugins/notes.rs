//! What a plugin writes in its home for the ring and the settings page: notes, values and the ring menu file.

use std::collections::BTreeMap;
use std::path::Path;

use crate::common::keyfile::KeyFile;
use crate::toolkit::t;

use super::manifest::{good_computer, home_of, Plugin};
use super::menu::MENU_LABEL_MAX;

pub const NOTES: &str = "notify.jsonl";

const VALUES: &str = "settings.ini";

pub const RING_NOTE: &str = "ring.json";

const RING_NOTE_MAX: u64 = 4096;

pub const HEADLINE_MAX: usize = 60;

const HIDDEN_MAX: usize = 32;

pub const ITEMS_MAX: usize = 8;
pub const ARGS_MAX: usize = 4;
pub const ARG_MAX: usize = 64;

const VALUES_MAX: u64 = 64 * 1024;

fn keyfile_in_home(path: &Path) -> KeyFile {
    read_plain(path, VALUES_MAX).map(|b| KeyFile::parse(&String::from_utf8_lossy(&b))).unwrap_or_default()
}

pub fn write_in_home(path: &Path, text: &str) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => return Err(std::io::Error::other(t!("plugins-folder-in-the-way"))),
        Ok(m) if !m.is_file() => std::fs::remove_file(path)?,
        _ => {}
    }
    let tmp = path.with_extension("trindle-tmp");
    let _ = std::fs::remove_file(&tmp);
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC).open(&tmp)?;
    f.write_all(text.as_bytes())?;
    drop(f);
    std::fs::rename(&tmp, path)
}

pub fn unread_notes(path: &Path, seen: &Path, trim_at: u64, drop_at: u64, keep: usize) -> Vec<String> {
    let bytes = match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_file() && m.len() > drop_at => {
            let _ = std::fs::remove_file(path);
            return Vec::new();
        }
        _ => read_plain(path, drop_at).unwrap_or_default(),
    };
    let text = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = text.lines().collect();
    let was = read_plain(seen, 32).map(|b| String::from_utf8_lossy(&b).trim().parse::<usize>().ok()).unwrap_or(None);
    let from = match was {
        Some(n) if n <= lines.len() => n,
        Some(_) => 0,
        None => lines.len(),
    };
    let new = lines[from..].iter().map(|l| l.to_string()).collect();
    let mut kept = lines.len();
    if bytes.len() as u64 > trim_at {
        let last = &lines[lines.len().saturating_sub(keep)..];
        if write_in_home(path, &(last.join("\n") + "\n")).is_ok() {
            kept = last.len();
        }
    }
    let _ = write_in_home(seen, &kept.to_string());
    new
}

pub fn values(p: &Plugin) -> BTreeMap<String, String> {
    let kf = keyfile_in_home(&home_of(&p.id).join(VALUES));
    p.fields.iter().map(|f| {
        let v = kf.string("Settings", &f.name).unwrap_or_else(|| f.default.clone());
        (f.name.clone(), v)
    }).collect()
}

pub fn set_value(id: &str, name: &str, value: &str) -> std::io::Result<()> {
    let path = home_of(id).join(VALUES);
    let mut kf = keyfile_in_home(&path);
    kf.set("Settings", name, value);
    write_in_home(&path, &kf.text())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RingNote {
    pub headline: String,
    pub hidden: Vec<String>,

    pub items: Vec<DynItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynItem {
    pub label: String,
    pub args: Vec<String>,

    pub unlock: Option<String>,
}

pub fn good_args(args: &[String]) -> bool {
    args.len() <= ARGS_MAX
        && args.iter().all(|a| !a.is_empty() && a.chars().count() <= ARG_MAX && !a.chars().any(char::is_control))
}

pub fn read_plain(path: &Path, most: u64) -> Option<Vec<u8>> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(most + 1).read_to_end(&mut bytes).ok()?;
    (bytes.len() as u64 <= most).then_some(bytes)
}

pub fn ring_note(id: &str) -> RingNote {
    ring_note_at(&home_of(id))
}

pub fn ring_note_at(home: &Path) -> RingNote {

    match read_plain(&home.join(RING_NOTE), RING_NOTE_MAX) {
        Some(bytes) => parse_ring_note(&bytes),
        None => RingNote::default(),
    }
}

pub fn parse_ring_note(bytes: &[u8]) -> RingNote {
    let Ok(serde_json::Value::Object(o)) = serde_json::from_slice::<serde_json::Value>(bytes) else {
        return RingNote::default();
    };
    let headline = o.get("headline").and_then(|v| v.as_str()).map(|s| plain_words(s, HEADLINE_MAX)).unwrap_or_default();
    let hidden = match o.get("hidden") {
        Some(serde_json::Value::Array(a)) => a.iter().filter_map(|v| v.as_str()).take(HIDDEN_MAX).map(String::from).collect(),
        _ => Vec::new(),
    };

    let items = match o.get("items") {
        Some(serde_json::Value::Array(a)) => a.iter().take(ITEMS_MAX).filter_map(|v| {
            let label = plain_words(v.get("label")?.as_str()?, MENU_LABEL_MAX);

            if let Some(computer) = v.get("unlock") {
                let computer = computer.as_str().filter(|c| good_computer(c) && v.get("args").is_none())?;
                return (!label.is_empty()).then(|| DynItem { label, args: Vec::new(), unlock: Some(computer.to_string()) });
            }
            let args: Option<Vec<String>> = v.get("args")?.as_array()?.iter().map(|a| a.as_str().map(String::from)).collect();
            let args = args.filter(|a| good_args(a))?;
            (!label.is_empty()).then_some(DynItem { label, args, unlock: None })
        }).collect(),
        _ => Vec::new(),
    };
    RingNote { headline, hidden, items }
}

pub fn plain_words(text: &str, max: usize) -> String {
    let hidden = |c: char| c.is_control() || matches!(c, '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2060}'..='\u{2069}' | '\u{FEFF}');
    let spaced: String = text.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).filter(|c| !hidden(*c)).collect();
    let one = spaced.split(' ').filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" ");
    if one.chars().count() <= max {
        one
    } else {
        one.chars().take(max.saturating_sub(1)).collect::<String>().trim_end().to_string() + "\u{2026}"
    }
}

#[cfg(test)]
mod tests;
