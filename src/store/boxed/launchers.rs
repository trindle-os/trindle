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
        let Some(text) = own_file(files, &path).and_then(|p| read_at_most(&p, LAUNCHER_MOST)) else { continue };
        let terminal = our_terminal(name, &text);

        let Some(entry) = launcher(name, &text, terminal, |icon| icon_in(files, icon)) else {
            eprintln!("trindle-boxed: {name}: {} has a command that could not be boxed, so it is left out", e.file_name().to_string_lossy());
            continue;
        };
        let target = out.join(e.file_name());
        if !may_write(&target, name) {
            eprintln!("trindle-boxed: {name}: {} is somebody else's launcher, so it is left alone", e.file_name().to_string_lossy());
            continue;
        }
        std::fs::write(&target, entry).map_err(|e| e.to_string())?;
    }
    Ok(())
}

const LAUNCHER_MOST: u64 = 64 * 1024;

pub(super) fn own_file(files: &Path, path: &Path) -> Option<PathBuf> {
    let (real, top) = (std::fs::canonicalize(path).ok()?, std::fs::canonicalize(files).ok()?);
    (real.starts_with(&top) && std::fs::metadata(&real).is_ok_and(|m| m.is_file())).then_some(real)
}

pub(super) fn read_at_most(path: &Path, most: u64) -> Option<String> {
    let mut text = String::new();
    std::io::Read::read_to_string(&mut std::io::Read::take(std::fs::File::open(path).ok()?, most + 1), &mut text).ok()?;
    (text.len() as u64 <= most).then_some(text)
}

pub(super) fn may_write(target: &Path, name: &str) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(target) else { return true };
    let mark = format!("{MARK}{name}");
    meta.is_file() && std::fs::read_to_string(target).is_ok_and(|t| t.lines().any(|l| l.trim() == mark))
}

fn our_terminal(name: &str, text: &str) -> bool {
    text.lines().filter_map(|l| l.strip_prefix("Categories=")).any(is_terminal) && debs_in(Path::new(FIRST_APPS)).iter()
        .any(|d| parts::field_of(&parts::control_of(d), "Package") == name)
}

pub(super) fn is_terminal(categories: &str) -> bool {
    categories.split(';').any(|c| c == "TerminalEmulator")
}

pub(super) fn key_value(line: &str) -> Option<(&str, &str)> {
    let t = line.trim_start();
    if t.starts_with('[') || t.starts_with('#') {
        return None;
    }
    let (k, v) = t.split_once('=')?;
    Some((k.trim(), v.trim_start()))
}

pub(super) fn bare(key: &str) -> &str {
    key.split('[').next().unwrap_or(key).trim()
}

pub(super) fn launcher(name: &str, text: &str, terminal: bool, find_icon: impl Fn(&str) -> Option<PathBuf>) -> Option<String> {
    let mut out = String::new();
    let mut first = true;
    let mut main = false;
    for line in text.lines() {
        let section = line.trim_start().starts_with('[');
        if section {
            main = line.trim() == "[Desktop Entry]";
        }
        let kv = key_value(line);
        let key = kv.map(|(k, _)| k);
        let base = key.map(bare);
        if terminal && main && matches!(key, Some("Name" | "Icon")) {
            out.push_str(&if key == Some("Name") { format!("Name={}\n", t!("store-boxed-terminal")) } else { "Icon=utilities-terminal\n".to_string() });
        } else if terminal && main && base == Some("Name") {
            continue;
        } else if let (Some("Exec"), Some((_, cmd))) = (key, kv) {
            if cmd.trim().is_empty() {
                return None;
            }
            out.push_str(&format!("Exec=trindle-boxed {name} {cmd}\n"));
        } else if matches!(base, Some("Exec" | "TryExec" | "DBusActivatable")) {
            continue;
        } else if let Some(icon) = (key == Some("Icon")).then(|| kv.map(|(_, v)| v)).flatten().filter(|i| !i.starts_with('/')) {
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
    let unboxed = out.lines().any(|l| key_value(l).is_some_and(|(k, _)| matches!(bare(k), "Exec" | "TryExec" | "DBusActivatable"))
        && !l.starts_with(&format!("Exec=trindle-boxed {name} ")));
    (!unboxed).then_some(out)
}

pub(super) fn icon_in(files: &Path, icon: &str) -> Option<PathBuf> {
    let theme = files.join("usr/share/icons/hicolor");
    if let Some(scalable) = own_file(files, &theme.join(format!("scalable/apps/{icon}.svg"))) {
        return Some(scalable);
    }
    let size = |d: &std::fs::DirEntry| d.file_name().to_string_lossy().split('x').next()?.parse::<u32>().ok();
    let biggest = std::fs::read_dir(&theme).into_iter().flatten().flatten()
        .filter_map(|d| Some((size(&d)?, own_file(files, &d.path().join(format!("apps/{icon}.png")))?)))
        .max_by_key(|(s, _)| *s)
        .map(|(_, p)| p);
    biggest.or_else(|| {
        ["png", "svg", "xpm"].iter().find_map(|x| own_file(files, &files.join(format!("usr/share/pixmaps/{icon}.{x}"))))
    })
}
