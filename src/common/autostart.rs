//! Login autostart entries in ~/.config/autostart with a per-entry delay (X-Trindle-Delay, falling back to X-GNOME-Autostart-Delay).

use std::path::{Path, PathBuf};

use super::files::home;
use crate::icons::read_entry;

pub const DELAYS: &[u32] = &[0, 5, 10, 20, 30, 60, 120];

pub fn folder() -> PathBuf {
    home().join(".config/autostart")
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub icon: String,
    pub exec: String,
    pub delay: u32,
    pub enabled: bool,
}

fn delay_of(values: &std::collections::BTreeMap<String, String>) -> u32 {
    values.get("X-Trindle-Delay").or_else(|| values.get("X-GNOME-Autostart-Delay"))
        .and_then(|v| v.trim().parse().ok()).unwrap_or(0)
}

pub fn list() -> Vec<Entry> {
    let mut out: Vec<Entry> = std::fs::read_dir(folder()).map(|d| d.flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "desktop"))
        .map(|path| {
            let v = read_entry(&path);
            let enabled = v.get("Hidden").map(String::as_str) != Some("true")
                && v.get("X-GNOME-Autostart-enabled").map(String::as_str) != Some("false");
            Entry {
                name: v.get("Name").cloned().unwrap_or_else(|| path.file_stem().unwrap_or_default().to_string_lossy().to_string()),
                icon: v.get("Icon").cloned().unwrap_or_default(),
                exec: v.get("Exec").cloned().unwrap_or_default(),
                delay: delay_of(&v),
                enabled,
                path,
            }
        }).collect()).unwrap_or_default();
    out.sort_by_key(|e| e.name.to_lowercase());
    out
}

fn free_name(stem: &str) -> PathBuf {
    let clean: String = stem.chars().map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' }).collect();
    let clean = if clean.is_empty() { "start".to_string() } else { clean };
    let mut path = folder().join(format!("{clean}.desktop"));
    let mut n = 2;
    while path.exists() {
        path = folder().join(format!("{clean}-{n}.desktop"));
        n += 1;
    }
    path
}

pub fn add_app(launcher: &Path, delay: u32) -> std::io::Result<()> {
    std::fs::create_dir_all(folder())?;
    let text = std::fs::read_to_string(launcher)?;
    let mut out = String::new();
    let mut in_entry = false;
    for line in text.lines() {
        if line.starts_with('[') {
            in_entry = line.trim() == "[Desktop Entry]";
        }
        if in_entry && (line.starts_with("Hidden=") || line.starts_with("NoDisplay=") || line.starts_with("X-Trindle-Delay=")) {
            continue;
        }
        out.push_str(line);
        out.push('\n');
        if line.trim() == "[Desktop Entry]" {
            out.push_str(&format!("X-Trindle-Delay={delay}\n"));
        }
    }
    let stem = launcher.file_stem().unwrap_or_default().to_string_lossy().to_string();
    super::files::write_text(&free_name(&stem), &out)
}

pub fn of_launcher(launcher: &Path) -> Option<Entry> {
    let exec = read_entry(launcher).get("Exec").cloned()?;
    list().into_iter().find(|e| e.exec == exec)
}

pub fn toggle_app(launcher: &Path) -> std::io::Result<bool> {
    match of_launcher(launcher) {
        Some(e) => remove(&e.path).map(|()| false),
        None => add_app(launcher, 0).map(|()| true),
    }
}

pub fn add_command(command: &str, delay: u32) -> std::io::Result<()> {
    std::fs::create_dir_all(folder())?;
    let first = command.split_whitespace().next().unwrap_or("command");
    let name = Path::new(first).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| first.to_string());
    let text = format!("[Desktop Entry]\nType=Application\nName={name}\nExec={}\n\
                        X-Trindle-Delay={delay}\nX-Trindle-Command=true\n", command.replace('\n', " "));
    super::files::write_text(&free_name(&name), &text)
}

fn set_key(path: &Path, key: &str, value: &str) -> std::io::Result<()> {
    let text = std::fs::read_to_string(path)?;
    let mut out = String::new();
    let (mut in_entry, mut done) = (false, false);
    for line in text.lines() {
        if line.starts_with('[') {
            if in_entry && !done {
                out.push_str(&format!("{key}={value}\n"));
                done = true;
            }
            in_entry = line.trim() == "[Desktop Entry]";
        }
        if in_entry && line.split_once('=').is_some_and(|(k, _)| k.trim() == key) {
            out.push_str(&format!("{key}={value}\n"));
            done = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    if !done {
        out.push_str(&format!("{key}={value}\n"));
    }
    super::files::write_text(path, &out)
}

pub fn set_delay(path: &Path, delay: u32) -> std::io::Result<()> {
    set_key(path, "X-Trindle-Delay", &delay.to_string())
}

pub fn set_enabled(path: &Path, on: bool) -> std::io::Result<()> {
    set_key(path, "Hidden", if on { "false" } else { "true" })
}

pub fn remove(path: &Path) -> std::io::Result<()> {
    if path.parent() != Some(folder().as_path()) {
        return Err(std::io::Error::other("Only launchers in ~/.config/autostart are removed here."));
    }
    std::fs::remove_file(path)
}

pub fn start_all() -> Vec<std::thread::JoinHandle<()>> {
    list().into_iter().filter(|e| e.enabled).map(|e| {
        std::thread::spawn(move || {
            if e.delay > 0 {
                std::thread::sleep(std::time::Duration::from_secs(e.delay as u64));
            }
            if let Err(err) = super::apps::start_file_saying(&e.path, std::process::Stdio::inherit()) {
                eprintln!("trindle-autostart: could not start {}: {err}", e.name);
            }
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn keys_are_changed_in_place() {
        let dir = std::env::temp_dir().join(format!("trindle-autostart-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.desktop");
        std::fs::write(&f, "[Desktop Entry]\nName=A\nExec=a\n[Desktop Action x]\nName=X\n").unwrap();
        super::set_delay(&f, 20).unwrap();
        super::set_delay(&f, 30).unwrap();
        let text = std::fs::read_to_string(&f).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(text.matches("X-Trindle-Delay").count(), 1);
        assert!(text.contains("X-Trindle-Delay=30\n[Desktop Action x]"));
    }

    #[test]
    fn a_launcher_is_whole_as_it_was_when_a_change_cannot_be_written() {
        use crate::common::files::FULL;
        let dir = std::env::temp_dir().join(format!("trindle-autostart-whole-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.desktop");
        let was = "[Desktop Entry]\nName=A\nExec=a\nX-Trindle-Delay=20\n";
        std::fs::write(&f, was).unwrap();
        FULL.set(true);
        let (delay, enabled) = (super::set_delay(&f, 30), super::set_enabled(&f, false));
        FULL.set(false);
        let (text, left) = (std::fs::read_to_string(&f).unwrap(), std::fs::read_dir(&dir).unwrap().count());
        let _ = std::fs::remove_dir_all(&dir);
        assert!(delay.is_err() && enabled.is_err());
        assert_eq!((text.as_str(), left), (was, 1), "the launcher as it was, and nothing left beside it");
    }
}
