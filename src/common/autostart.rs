//! Login autostart entries in ~/.config/autostart with a per-entry delay
//! (X-Trindle-Delay, falling back to X-GNOME-Autostart-Delay), and the
//! system's own list in /etc/xdg/autostart, each of which can be switched
//! off by a note of the person's own beside the person's entries.

use std::path::{Path, PathBuf};

use super::files::home;
use super::keyfile::KeyFile;

const GROUP: &str = "Desktop Entry";
const GNOME_ENABLED: &str = "X-GNOME-Autostart-enabled";

pub const DELAYS: &[u32] = &[0, 5, 10, 20, 30, 60, 120];

#[cfg(not(test))]
const SYSTEM: &str = "/etc/xdg/autostart";

const MOST_SYSTEM: usize = 400;

const SWITCHED_OFF: &str = "X-Trindle-Switched-Off";

pub fn folder() -> PathBuf {
    home().join(".config/autostart")
}

fn system_folder() -> PathBuf {
    #[cfg(test)]
    return home().join("system-autostart");
    #[cfg(not(test))]
    PathBuf::from(SYSTEM)
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub icon: String,
    pub exec: String,
    pub delay: u32,
    pub enabled: bool,

    pub system: bool,
}

fn delay_of(entry: &KeyFile) -> u32 {
    entry.string(GROUP, "X-Trindle-Delay").or_else(|| entry.string(GROUP, "X-GNOME-Autostart-Delay"))
        .and_then(|v| v.trim().parse().ok()).unwrap_or(0)
}

fn entry_of(path: PathBuf, v: &KeyFile, enabled: bool, system: bool) -> Entry {
    Entry {
        name: v.string(GROUP, "Name").unwrap_or_else(|| path.file_stem().unwrap_or_default().to_string_lossy().to_string()),
        icon: v.string(GROUP, "Icon").unwrap_or_default(),
        exec: v.string(GROUP, "Exec").unwrap_or_default(),
        delay: delay_of(v),
        enabled,
        system,
        path,
    }
}

fn desktop_files(dir: &Path, most: usize) -> Vec<PathBuf> {
    std::fs::read_dir(dir).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "desktop")).take(most).collect())
        .unwrap_or_default()
}

fn meant_for(v: &KeyFile, desktops: &[String]) -> bool {
    let yes = |key: &str| v.string(GROUP, key).is_some_and(|x| x == "true");
    let in_list = |key: &str| v.list(GROUP, key).map(|l| l.iter().any(|d| desktops.iter().any(|x| x.eq_ignore_ascii_case(d))));
    v.string(GROUP, "Type").as_deref() == Some("Application") && !yes("Hidden") && v.string(GROUP, GNOME_ENABLED).as_deref() != Some("false")
        && in_list("OnlyShowIn") != Some(false) && in_list("NotShowIn") != Some(true)
        && v.string(GROUP, "TryExec").filter(|t| !t.is_empty()).is_none_or(|t| super::apps::on_path(&t))
        && v.string(GROUP, "Exec").is_some_and(|e| !e.is_empty())
}

fn list_in(own: &Path, system: &Path, desktops: &[String]) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut hiding = Vec::new();
    for path in desktop_files(own, 4 * MOST_SYSTEM) {
        let v = KeyFile::load_or_empty(&path);
        if v.string(GROUP, SWITCHED_OFF).as_deref() == Some("true") && system.join(path.file_name().unwrap_or_default()).is_file() {
            hiding.push(path.file_name().unwrap_or_default().to_owned());
            continue;
        }
        let enabled = v.string(GROUP, "Hidden").as_deref() != Some("true") && v.string(GROUP, GNOME_ENABLED).as_deref() != Some("false");
        out.push(entry_of(path, &v, enabled, false));
    }
    let own_names: Vec<_> = out.iter().filter_map(|e| e.path.file_name().map(|n| n.to_owned())).collect();
    for path in desktop_files(system, MOST_SYSTEM) {
        let name = path.file_name().unwrap_or_default().to_owned();
        let Ok(v) = KeyFile::load(&path) else { continue };
        if own_names.contains(&name) || !path.is_file() || !meant_for(&v, desktops) {
            continue;
        }
        out.push(entry_of(path, &v, !hiding.contains(&name), true));
    }
    out.sort_by_key(|e| e.name.to_lowercase());
    out
}

pub fn list() -> Vec<Entry> {
    list_in(&folder(), &system_folder(), &super::apps::current_desktops())
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
    let mut entry = KeyFile::load(launcher)?;

    entry.set(GROUP, "Hidden", "false");
    entry.set(GROUP, "NoDisplay", "false");
    entry.set(GROUP, "X-Trindle-Delay", &delay.to_string());
    let stem = launcher.file_stem().unwrap_or_default().to_string_lossy().to_string();
    super::files::write_text(&free_name(&stem), &entry.text())
}

pub fn of_launcher(launcher: &Path) -> Option<Entry> {
    let exec = KeyFile::load_or_empty(launcher).string(GROUP, "Exec")?;
    list().into_iter().find(|e| !e.system && e.exec == exec)
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
    let mut entry = KeyFile::default();
    for (key, value) in [("Type", "Application"), ("Name", name.as_str()), ("Exec", &command.replace('\n', " ")),
                         ("X-Trindle-Delay", &delay.to_string()), ("X-Trindle-Command", "true")] {
        entry.set(GROUP, key, value);
    }
    super::files::write_text(&free_name(&name), &entry.text())
}

fn change(path: &Path, edit: impl FnOnce(&mut KeyFile)) -> std::io::Result<()> {
    let mut entry = KeyFile::load_kept(path)?;
    edit(&mut entry);
    super::files::write_text(path, &entry.text())
}

pub fn set_delay(path: &Path, delay: u32) -> std::io::Result<()> {
    change(path, |e| e.set(GROUP, "X-Trindle-Delay", &delay.to_string()))
}

pub fn set_enabled(path: &Path, on: bool) -> std::io::Result<()> {
    if path.parent() == Some(system_folder().as_path()) {
        return switch_system(path, on);
    }
    change(path, |e| {
        e.set(GROUP, "Hidden", if on { "false" } else { "true" });
        if on && e.string(GROUP, GNOME_ENABLED).is_some() {
            e.set(GROUP, GNOME_ENABLED, "true");
        }
    })
}

fn switch_system(path: &Path, on: bool) -> std::io::Result<()> {
    let note = folder().join(path.file_name().ok_or_else(|| std::io::Error::other("It has no name."))?);
    if on {
        return match KeyFile::load_or_empty(&note).string(GROUP, SWITCHED_OFF).as_deref() {
            Some("true") => std::fs::remove_file(&note),
            _ => Ok(()),
        };
    }
    if note.exists() {
        return Err(std::io::Error::other("A launcher of the same name is already there."));
    }
    std::fs::create_dir_all(folder())?;
    let mut entry = KeyFile::default();
    for (key, value) in [("Type", "Application"), ("Hidden", "true"), (SWITCHED_OFF, "true")] {
        entry.set(GROUP, key, value);
    }
    super::files::write_text(&note, &entry.text())
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

    #[test]
    fn a_launcher_another_desktop_switched_off_can_be_switched_on() {
        let dir = std::env::temp_dir().join(format!("trindle-autostart-gnome-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.desktop");
        std::fs::write(&f, "[Desktop Entry]\nName=A\nExec=a\nX-GNOME-Autostart-enabled=false\n").unwrap();
        let off = super::KeyFile::load_or_empty(&f);
        assert_eq!(off.string("Desktop Entry", "X-GNOME-Autostart-enabled").as_deref(), Some("false"));
        super::set_enabled(&f, true).unwrap();
        let on = super::KeyFile::load_or_empty(&f);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(on.string("Desktop Entry", "X-GNOME-Autostart-enabled").as_deref(), Some("true"));
        assert_eq!(on.string("Desktop Entry", "Hidden").as_deref(), Some("false"));
    }

    #[test]
    fn a_command_with_a_backslash_runs_as_typed() {
        let home = std::env::temp_dir().join(format!("trindle-autostart-command-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let mut entry = super::KeyFile::default();
        let typed = "sh -c 'echo a\\nb \\ c'";
        entry.set("Desktop Entry", "Exec", typed);
        let back = super::KeyFile::parse(&entry.text());
        assert_eq!(back.string("Desktop Entry", "Exec").as_deref(), Some(typed));
    }

    #[test]
    fn the_systems_launchers_are_listed_when_meant_for_this_desktop_and_switched_by_a_note() {
        let Some(home) = crate::common::scratch_home::inside() else {
            return crate::common::scratch_home::rerun(module_path!(), "the_systems_launchers_are_listed_when_meant_for_this_desktop_and_switched_by_a_note");
        };
        let system = super::system_folder();
        std::fs::create_dir_all(&system).unwrap();
        let make = |name: &str, extra: &str| std::fs::write(system.join(name), format!("[Desktop Entry]\nType=Application\nName={name}\nExec=/bin/true\n{extra}")).unwrap();
        make("plain.desktop", "");
        make("gnome-only.desktop", "OnlyShowIn=GNOME;\n");
        make("not-here.desktop", "NotShowIn=Trindle;\n");
        make("here.desktop", "OnlyShowIn=KDE;Trindle;\n");
        make("hidden.desktop", "Hidden=true\n");
        make("no-program.desktop", "TryExec=/nowhere/at/all\n");
        make("shadowed.desktop", "");
        std::fs::create_dir_all(super::folder()).unwrap();
        std::fs::write(super::folder().join("shadowed.desktop"), "[Desktop Entry]\nType=Application\nName=Mine\nExec=mine\n").unwrap();
        let names = |all: &[super::Entry]| all.iter().map(|e| e.path.file_name().unwrap().to_string_lossy().to_string()).collect::<Vec<_>>();
        let desktops = ["Trindle".to_string()];
        let all = super::list_in(&super::folder(), &system, &desktops);
        let mut got = names(&all);
        got.sort();
        assert_eq!(got, ["here.desktop", "plain.desktop", "shadowed.desktop"], "only what is meant for this desktop, and the person's own in place of the system's");
        let (plain, mine) = (all.iter().find(|e| e.name == "plain.desktop").unwrap(), all.iter().find(|e| e.name == "Mine").unwrap());
        assert!(plain.system && plain.enabled && !mine.system);

        let before = std::fs::read_to_string(&plain.path).unwrap();
        super::set_enabled(&plain.path, false).unwrap();
        assert_eq!(std::fs::read_to_string(&plain.path).unwrap(), before);
        let all = super::list_in(&super::folder(), &system, &desktops);
        assert_eq!(all.len(), 3, "the note is not a second line");
        assert!(!all.iter().find(|e| e.name == "plain.desktop").unwrap().enabled);
        super::set_enabled(&plain.path, true).unwrap();
        assert!(super::list_in(&super::folder(), &system, &desktops).iter().find(|e| e.name == "plain.desktop").unwrap().enabled);
        assert!(!super::folder().join("plain.desktop").exists(), "on again takes the note away");

        super::set_enabled(&system.join("shadowed.desktop"), true).unwrap();
        assert!(super::folder().join("shadowed.desktop").exists());
        assert!(super::remove(&plain.path).is_err(), "a system launcher is never removed");
        assert!(home.is_dir());
    }
}
