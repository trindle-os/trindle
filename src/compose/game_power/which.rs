//! Whether a window filling the screen is a game's: started by Steam, Lutris or Heroic, running under Wine or Proton, or an installed app whose launcher file calls it a game.

use std::io::Read;
use std::path::{Path, PathBuf};

pub(super) const DEEPEST: usize = 64;

const MOST_BYTES: u64 = 256 * 1024;

const MOST_LAUNCHERS: usize = 4096;

const LAUNCHED_BY: &[&[u8]] = &[b"LUTRIS_GAME_UUID=", b"HEROIC_APP_NAME="];

const WINE: &[&str] = &["wine-preloader", "wine64-preloader", "wine", "wine64"];

const LAUNCHERS: &[&str] = &["steam", "com.valvesoftware.steam", "net.lutris.lutris", "lutris", "com.heroicgameslauncher.hgl",
                             "heroic", "com.usebottles.bottles"];

pub(super) struct Places {
    pub proc: PathBuf,
    pub apps: Vec<PathBuf>,
}

impl Places {
    pub fn computer() -> Places {
        let home = crate::files::home();
        Places {
            proc: PathBuf::from("/proc"),
            apps: vec![
                home.join(".local/share/applications"),
                home.join(".local/share/flatpak/exports/share/applications"),
                PathBuf::from("/var/lib/flatpak/exports/share/applications"),
                PathBuf::from("/usr/local/share/applications"),
                PathBuf::from("/usr/share/applications"),
            ],
        }
    }
}

pub(super) fn is_game(places: &Places, pid: Option<u32>, app_id: &str) -> bool {
    pid.is_some_and(|p| steam_game(&places.proc, p) || under_wine(&places.proc, p) || launched_game(&places.proc, p))
        || game_app(&places.apps, app_id)
}

pub(super) fn steam_game(proc: &Path, mut pid: u32) -> bool {
    for _ in 0..DEEPEST {
        if pid <= 1 {
            return false;
        }
        let dir = proc.join(pid.to_string());
        if read_capped(&dir.join("cmdline")).is_some_and(|line| crate::pads::steam_launch(&line).is_some()) {
            return true;
        }
        match read_capped(&dir.join("stat")).and_then(|stat| parent_of(&stat)) {
            Some(parent) => pid = parent,
            None => return false,
        }
    }
    false
}

fn under_wine(proc: &Path, pid: u32) -> bool {
    std::fs::read_link(proc.join(pid.to_string()).join("exe")).ok()
        .and_then(|exe| exe.file_name().map(|n| WINE.contains(&n.to_string_lossy().as_ref())))
        .unwrap_or(false)
}

fn launched_game(proc: &Path, pid: u32) -> bool {
    read_capped(&proc.join(pid.to_string()).join("environ"))
        .is_some_and(|env| env.split(|&b| b == 0).any(|v| LAUNCHED_BY.iter().any(|m| v.starts_with(m))))
}

fn game_app(dirs: &[PathBuf], app_id: &str) -> bool {
    let wanted = app_id.trim().to_lowercase();
    let wanted = wanted.strip_suffix(".desktop").unwrap_or(&wanted);
    if wanted.is_empty() {
        return false;
    }
    let files = dirs.iter().filter_map(|d| std::fs::read_dir(d).ok()).flat_map(|l| l.flatten()).take(MOST_LAUNCHERS);
    for file in files {
        let path = file.path();
        let Some(id) = path.file_name().and_then(|n| n.to_str()?.strip_suffix(".desktop")).map(str::to_lowercase) else { continue };
        let Some(text) = read_capped(&path) else { continue };
        let launcher = Launcher::read(&String::from_utf8_lossy(&text));
        let named = id == wanted || id.rsplit('.').next() == Some(wanted) || launcher.wm_class == wanted || launcher.flatpak == wanted;
        if named {
            return launcher.game && ![&id, &launcher.flatpak].iter().any(|n| LAUNCHERS.contains(&n.as_str()));
        }
    }
    false
}

#[derive(Default)]
struct Launcher {
    game: bool,
    wm_class: String,
    flatpak: String,
}

impl Launcher {
    fn read(text: &str) -> Launcher {
        let mut launcher = Launcher::default();
        let mut in_entry = false;
        for line in text.lines().map(str::trim) {
            if line.starts_with('[') {
                in_entry = line == "[Desktop Entry]";
                continue;
            }
            let Some((key, value)) = line.split_once('=').filter(|_| in_entry) else { continue };
            match key.trim() {
                "Categories" => launcher.game = value.split(';').any(|c| c.trim() == "Game"),
                "StartupWMClass" => launcher.wm_class = value.trim().to_lowercase(),
                "X-Flatpak" => launcher.flatpak = value.trim().to_lowercase(),
                _ => {}
            }
        }
        launcher
    }
}

fn read_capped(path: &Path) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path).ok()?.take(MOST_BYTES).read_to_end(&mut bytes).ok()?;
    Some(bytes)
}

pub(super) fn parent_of(stat: &[u8]) -> Option<u32> {
    let after = &stat[stat.iter().rposition(|&b| b == b')')? + 1..];
    std::str::from_utf8(after).ok()?.split_ascii_whitespace().nth(1)?.parse().ok()
}
