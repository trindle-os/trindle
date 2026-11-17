//! Clipboard history, fed by `trindle-clipboard watch`, which Trindle starts
//! at login and which hears of every copy through `selection`. Stored in
//! ~/.local/share/trindle/clipboard (mode 0700).

use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use super::files::{home, setting};
use super::selection;
use crate::toolkit::t;

const KEEP: usize = 200;
const MAX_TEXT: usize = 200_000;
const MAX_PICTURE: usize = 20_000_000;

pub fn forget_choices() -> [(&'static str, &'static str); 5] {
    [
        ("reboot", t!("settings-ring-forget-reboot")),
        ("1", t!("settings-ring-forget-day")),
        ("7", t!("settings-ring-forget-week")),
        ("14", t!("settings-ring-forget-fortnight")),
        ("never", t!("settings-ring-forget-never")),
    ]
}

pub fn forget_setting() -> String {
    let v = setting("clipboard_forget");
    if forget_choices().iter().any(|(k, _)| *k == v) { v } else { "7".into() }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: u64,
    pub time: u64,
    pub text: String,

    pub picture: Option<String>,
    pub pinned: bool,
}

#[cfg(test)]
thread_local! {

    pub(crate) static TEST_DIR: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

fn dir() -> PathBuf {
    #[cfg(test)]
    if let Some(d) = TEST_DIR.with(|d| d.borrow().clone()) {
        return d;
    }
    home().join(".local/share/trindle/clipboard")
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn damaged() -> &'static str { t!("common-damaged-clipboard") }

pub fn load() -> Vec<Item> {
    load_from(&dir().join("index.json"))
}

pub fn load_from(file: &std::path::Path) -> Vec<Item> {
    let text = super::damaged::read_whole(file, damaged(), |t| serde_json::from_str::<Value>(t).is_ok_and(|v| v.is_array()));
    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    v.as_array().map(|a| a.iter().filter_map(|e| Some(Item {
        id: e["id"].as_u64()?,
        time: e["time"].as_u64().unwrap_or(0),
        text: e["text"].as_str().unwrap_or("").to_string(),
        picture: e["picture"].as_str().filter(|p| !p.contains('/')).map(String::from),
        pinned: e["pinned"].as_bool() == Some(true),
    })).collect()).unwrap_or_default()
}

fn save(items: &[Item]) {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    let d = dir();

    if super::damaged::may_write(&d.join("index.json"), damaged()).is_err() {
        return;
    }

    if std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&d).is_err() {
        return;
    }
    if std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o700)).is_err() {
        eprintln!("trindle-clipboard: {} could not be made private; nothing was saved.", d.display());
        return;
    }
    let v: Vec<Value> = items.iter().map(|i| json!({
        "id": i.id, "time": i.time, "text": i.text, "picture": i.picture, "pinned": i.pinned,
    })).collect();

    let _ = super::files::write_whole_private(&d.join("index.json"), serde_json::to_string(&v).unwrap_or_default().as_bytes());

    if let Ok(entries) = std::fs::read_dir(&d) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".png") && !items.iter().any(|i| i.picture.as_deref() == Some(name.as_str())) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
}

fn tidy(items: &mut Vec<Item>) {
    tidy_with(items, &forget_setting());
}

fn tidy_with(items: &mut Vec<Item>, forget: &str) {
    if let Ok(days) = forget.parse::<u64>() {
        let cut = now().saturating_sub(days * 86_400);
        items.retain(|i| i.pinned || i.time >= cut);
    }
    let mut unpinned = 0;
    items.retain(|i| {
        if i.pinned {
            return true;
        }
        unpinned += 1;
        unpinned <= KEEP
    });
}

fn turn() -> Option<std::fs::File> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
    let d = dir();
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&d).ok()?;
    let held = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC).open(&d).ok()?;
    loop {
        // SAFETY: the descriptor is that of `held`, which is open throughout the call.
        if unsafe { libc::flock(held.as_raw_fd(), libc::LOCK_EX) } == 0 {
            return Some(held);
        }
        if std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
            return None;
        }
    }
}

fn stdin() -> Vec<u8> {
    let mut data = Vec::new();
    let _ = std::io::stdin().take(MAX_PICTURE as u64 + 1).read_to_end(&mut data);
    data
}

pub(crate) fn keep(data: Vec<u8>) {
    let Some(_turn) = (!data.is_empty()).then(turn).flatten() else { return };
    let mut items = load();

    if super::damaged::may_write(&dir().join("index.json"), damaged()).is_err() {
        return;
    }
    let id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    let item = if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        if data.len() > MAX_PICTURE {
            return;
        }

        let same = |i: &Item| i.picture.as_ref().is_some_and(|p| {
            let path = dir().join(p);
            std::fs::metadata(&path).is_ok_and(|m| m.len() == data.len() as u64)
                && std::fs::read(&path).is_ok_and(|d| d == data)
        });
        if let Some(pos) = items.iter().position(same) {
            return raise(items, pos);
        }
        let name = format!("{id}.png");

        if std::os::unix::fs::DirBuilderExt::mode(std::fs::DirBuilder::new().recursive(true), 0o700).create(dir()).is_err() || std::fs::write(dir().join(&name), &data).is_err() {
            return;
        }
        Item { id, time: now(), text: String::new(), picture: Some(name), pinned: false }
    } else {
        let Ok(text) = String::from_utf8(data) else { return };
        if text.trim().is_empty() || text.len() > MAX_TEXT {
            return;
        }
        if let Some(pos) = items.iter().position(|i| i.picture.is_none() && i.text == text) {
            return raise(items, pos);
        }
        Item { id, time: now(), text, picture: None, pinned: false }
    };
    items.insert(0, item);
    tidy(&mut items);
    save(&items);
}

fn raise(mut items: Vec<Item>, pos: usize) {
    let mut old = items.remove(pos);
    old.time = now();
    items.insert(0, old);
    tidy(&mut items);
    save(&items);
}

pub fn at_login() {
    let Some(_turn) = turn() else { return };
    let mut items = load();
    if forget_setting() == "reboot" {
        items.retain(|i| i.pinned);
    }
    tidy(&mut items);
    save(&items);
}

pub fn list() -> Vec<Item> {
    let mut items = load();
    tidy(&mut items);
    items.sort_by_key(|i| (!i.pinned, std::cmp::Reverse(i.time)));
    items
}

pub fn set_pinned(id: u64, pinned: bool) {
    let Some(_turn) = turn() else { return };
    let mut items = load();
    if let Some(i) = items.iter_mut().find(|i| i.id == id) {
        i.pinned = pinned;
    }
    save(&items);
}

pub fn remove(id: u64) {
    let Some(_turn) = turn() else { return };
    let mut items = load();
    items.retain(|i| i.id != id);
    save(&items);
}

pub fn clear() {
    let Some(_turn) = turn() else { return };
    let mut items = load();
    items.retain(|i| i.pinned);
    save(&items);
}

pub fn copy(item: &Item) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut cmd = Command::new("trindle-clipboard");
    cmd.arg("copy");
    if item.picture.is_some() {
        cmd.args(["--type", selection::PNG]);
    }
    let Ok(mut child) = cmd.stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() else { return };
    let data = match &item.picture {
        Some(p) => std::fs::read(dir().join(p)).unwrap_or_default(),
        None => item.text.clone().into_bytes(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&data);
    }
    std::thread::spawn(move || child.wait());
}

fn copy_stdin(args: &[String]) -> i32 {

    let mut types: Vec<&str> = args.windows(2).filter(|w| w[0] == "--type").map(|w| w[1].as_str()).collect();
    if types.is_empty() {
        types = selection::TEXT.to_vec();
    }
    let data = stdin();
    if data.is_empty() || data.len() > MAX_PICTURE {
        return 1;
    }
    selection::offer(data, &types, || {
        use std::io::Write;

        let mut out = std::io::stdout();
        let _ = writeln!(out, "copied");
        let _ = out.flush();
    })
}

pub fn main(args: Vec<String>) -> i32 {
    match args.get(1).map(String::as_str) {
        Some("watch") => return selection::watch(MAX_PICTURE as u64 + 1, keep, |resting| super::renewed::watch_apart("trindle-clipboard", resting)),
        Some("copy") => return copy_stdin(&args[2..]),
        Some("paste") => {
            use std::io::Write;

            let kind = args.windows(2).find(|w| w[0] == "--type").map(|w| w[1].as_str());
            let data = match kind {
                Some(k) => selection::paste_kinds(MAX_PICTURE as u64 + 1, &[k]).into_iter().find(|(name, _)| name == k).map(|(_, d)| d),
                None => selection::paste(MAX_PICTURE as u64 + 1),
            };
            let Some(data) = data else { return 1 };
            let _ = std::io::stdout().write_all(&data);
        }
        Some("clear") => clear(),
        Some("login") => at_login(),
        _ => {
            eprintln!("usage: trindle-clipboard watch|copy [--type TYPE]...|paste [--type TYPE]|clear|login");
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_made_at_the_same_time_are_all_kept() {
        let dir = std::env::temp_dir().join(format!("trindle-clipboard-turns-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let workers: Vec<_> = (0..6).map(|w| {
            let dir = dir.clone();
            std::thread::spawn(move || {
                TEST_DIR.with(|d| *d.borrow_mut() = Some(dir));
                for n in 0..12 {
                    keep(format!("copy {w}-{n}").into_bytes());
                }
            })
        }).collect();
        workers.into_iter().for_each(|w| w.join().unwrap());
        TEST_DIR.with(|d| *d.borrow_mut() = Some(dir.clone()));
        let kept = load().len();
        TEST_DIR.with(|d| *d.borrow_mut() = None);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(kept, 72);
    }

    #[test]
    fn old_unpinned_go_and_pinned_stay() {
        let t = now();
        let mut items = vec![
            Item { id: 1, time: t, text: "new".into(), picture: None, pinned: false },
            Item { id: 2, time: t - 30 * 86_400, text: "old".into(), picture: None, pinned: false },
            Item { id: 3, time: t - 30 * 86_400, text: "old but pinned".into(), picture: None, pinned: true },
        ];
        tidy_with(&mut items, "7");
        let left: Vec<u64> = items.iter().map(|i| i.id).collect();
        assert_eq!(left, vec![1, 3]);
    }
}
