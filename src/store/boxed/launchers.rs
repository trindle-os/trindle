//! The launchers a boxed package brings, made to start it boxed.

use crate::toolkit::t;
use std::path::{Path, PathBuf};
use crate::store::parts;
use super::needs::{debs_in};
use super::{applications, FIRST_APPS, MARK};

pub fn launchers_of(name: &str) -> Vec<PathBuf> {
    let mark = format!("{MARK}{name}");
    std::fs::read_dir(applications()).into_iter().flatten().flatten().map(|e| e.path())
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| t.lines().any(|l| l.trim() == mark)))
        .collect()
}

pub(super) fn take_launchers(name: &str) {
    for path in launchers_of(name) {
        let _ = std::fs::remove_file(path);
    }
}

pub(super) fn put_launchers(name: &str, files: &Path) -> Result<(), String> {
    let out = applications();
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let from = files.join("usr/share/applications");
    for e in std::fs::read_dir(&from).into_iter().flatten().flatten() {
        let path = e.path();
        if path.extension().is_none_or(|x| x != "desktop") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let terminal = our_terminal(name, &text);
        let entry = launcher(name, &text, terminal, |icon| icon_in(files, icon));
        std::fs::write(out.join(e.file_name()), entry).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn our_terminal(name: &str, text: &str) -> bool {
    text.lines().filter_map(|l| l.strip_prefix("Categories=")).any(is_terminal) && debs_in(Path::new(FIRST_APPS)).iter()
        .any(|d| parts::field_of(&parts::control_of(d), "Package") == name)
}

pub(super) fn is_terminal(categories: &str) -> bool {
    categories.split(';').any(|c| c == "TerminalEmulator")
}

pub(super) fn launcher(name: &str, text: &str, terminal: bool, find_icon: impl Fn(&str) -> Option<PathBuf>) -> String {
    let mut out = String::new();
    let mut first = true;
    let mut main = false;
    for line in text.lines() {
        let section = line.starts_with('[');
        if section {
            main = line.trim() == "[Desktop Entry]";
        }
        if terminal && main && (line.starts_with("Name=") || line.starts_with("Icon=")) {
            out.push_str(&if line.starts_with("Name=") { format!("Name={}\n", t!("store-boxed-terminal")) } else { "Icon=utilities-terminal\n".to_string() });
        } else if terminal && main && line.starts_with("Name[") {
            continue;
        } else if let Some(cmd) = line.strip_prefix("Exec=") {
            out.push_str(&format!("Exec=trindle-boxed {name} {cmd}\n"));
        } else if line.starts_with("TryExec=") || line.starts_with("DBusActivatable=") {
            continue;
        } else if let Some(icon) = line.strip_prefix("Icon=").filter(|i| !i.starts_with('/')) {
            match find_icon(icon) {
                Some(p) => out.push_str(&format!("Icon={}\n", p.display())),
                None => out.push_str(&format!("Icon={icon}\n")),
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
        if section && first {
            out.push_str(&format!("{MARK}{name}\n"));
            first = false;
        }
    }
    out
}

fn icon_in(files: &Path, icon: &str) -> Option<PathBuf> {
    let theme = files.join("usr/share/icons/hicolor");
    let scalable = theme.join(format!("scalable/apps/{icon}.svg"));
    if scalable.is_file() {
        return Some(scalable);
    }
    let size = |d: &std::fs::DirEntry| d.file_name().to_string_lossy().split('x').next()?.parse::<u32>().ok();
    let biggest = std::fs::read_dir(&theme).into_iter().flatten().flatten()
        .filter_map(|d| Some((size(&d)?, d.path().join(format!("apps/{icon}.png")))))
        .filter(|(_, p)| p.is_file())
        .max_by_key(|(s, _)| *s)
        .map(|(_, p)| p);
    biggest.or_else(|| {
        ["png", "svg", "xpm"].iter().map(|x| files.join(format!("usr/share/pixmaps/{icon}.{x}"))).find(|p| p.is_file())
    })
}
