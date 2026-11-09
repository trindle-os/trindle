//! AppImages in ~/.appimage, ~/Applications and ~/AppImages, each given a
//! launcher named and iconed from its embedded .desktop and .DirIcon.

use crate::toolkit::t;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::common::copies::{Put, Taken};
use crate::common::files::home;

const PREFIX: &str = "trindle-appimage-";
pub const MARK: &str = "X-Trindle-AppImage=";

const LAUNCHER_FORM: u32 = 4;

pub fn store() -> PathBuf {
    home().join(".appimage")
}

fn folders() -> [PathBuf; 3] {
    [store(), home().join("Applications"), home().join("AppImages")]
}

fn launchers() -> PathBuf {
    home().join(".local/share/applications")
}

fn icons() -> PathBuf {
    home().join(".local/share/trindle/appimage-icons")
}

pub fn nice_name(filename: &str) -> String {
    let name = crate::common::apps::appimage_stem(filename).replace(['_', '-'], " ").trim().to_string();
    let mut chars = name.chars();
    match chars.next() {
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        None => filename.to_string(),
    }
}

pub fn slug(name: &str) -> String {
    let s: String = name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let s = s.split('-').filter(|p| !p.is_empty()).collect::<Vec<_>>().join("-");
    if s.is_empty() { "app".into() } else { s }
}

pub fn found() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for folder in folders() {
        let Ok(entries) = std::fs::read_dir(&folder) else { continue };
        let mut v: Vec<PathBuf> = entries.flatten().map(|e| e.path())
            .filter(|p| p.is_file() && p.to_string_lossy().to_lowercase().ends_with(".appimage")).collect();
        v.sort_by_key(|p| p.to_string_lossy().to_lowercase());
        out.extend(v);
    }
    out
}

fn extract(path: &Path, pattern: &str, into: &Path) {
    let child = Command::new(path).args(["--appimage-extract", pattern]).current_dir(into)
        .stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    if let Ok(mut c) = child {

        for _ in 0..200 {
            if let Ok(Some(_)) = c.try_wait() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let _ = c.kill();
    }
}

fn real(path: &Path, entry: &Path, tmp: &Path) -> Option<PathBuf> {
    let inside = std::fs::canonicalize(tmp).ok()?;
    if let Ok(target) = std::fs::read_link(entry) {
        if target.is_relative() {
            extract(path, &target.to_string_lossy(), tmp);
        }
        let r = std::fs::canonicalize(entry).ok()?;
        return (r.starts_with(&inside) && r.is_file()).then_some(r);
    }
    let r = std::fs::canonicalize(entry).ok()?;
    (r.starts_with(&inside) && r.is_file()).then_some(r)
}

pub struct Inside {
    pub name: String,
    pub icon: Option<PathBuf>,

    pub terminal: bool,
    pub categories: String,

    pub window: Option<String>,
}

fn window_name(file: &str, wm_class: Option<&String>) -> Option<String> {
    let fits = |c: &&str| !c.is_empty() && !c.contains(['\n', '\r']);
    let own = wm_class.map(|c| c.trim()).filter(fits);
    own.or_else(|| file.strip_suffix(".desktop").filter(fits)).map(str::to_string)
}

pub fn read_inside(path: &Path) -> Inside {
    let mut inside = Inside {
        name: nice_name(&path.file_name().unwrap_or_default().to_string_lossy()),
        icon: None,
        terminal: false,
        categories: "Utility;".to_string(),
        window: None,
    };
    let name = &mut inside.name;
    let icon = &mut inside.icon;
    let Ok(tmp) = crate::common::files::private_temp("trindle-appimage") else { return inside };
    extract(path, "*.desktop", &tmp);
    extract(path, ".DirIcon", &tmp);
    let root = tmp.join("squashfs-root");
    let mut entries: Vec<String> = std::fs::read_dir(&root).map(|r| r.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default();
    entries.sort();
    if let Some(d) = entries.iter().find(|f| f.ends_with(".desktop")) {
        if let Some(r) = real(path, &root.join(d), &tmp) {
            let e = super::read_entry(&r);
            if let Some(n) = e.get("Name").filter(|n| !n.is_empty()) {
                *name = n.clone();
            }
            inside.terminal = e.get("Terminal").map(String::as_str) == Some("true");
            inside.window = window_name(d, e.get("StartupWMClass"));
            if let Some(c) = e.get("Categories").filter(|c| !c.is_empty()) {
                inside.categories = c.clone();
            }
        }
    }
    if entries.iter().any(|f| f == ".DirIcon") {
        if let Some(r) = real(path, &root.join(".DirIcon"), &tmp) {
            if let Ok(data) = std::fs::read(&r) {
                let kind = if data.starts_with(b"\x89PNG") { Some("png") }
                           else if data.windows(4).take(512).any(|w| w == b"<svg") { Some("svg") } else { None };
                if let (Some(kind), true) = (kind, data.len() < 5_000_000) {
                    let _ = std::fs::create_dir_all(icons());
                    let p = icons().join(format!("{}.{kind}", slug(name)));
                    if std::fs::write(&p, &data).is_ok() {
                        *icon = Some(p);
                    }
                }
            }
        }
    }
    let _ = std::fs::remove_dir_all(&tmp);
    inside
}

pub fn exec_line(path: &Path) -> String {
    format!("trindle-boxed --appimage \"{}\" %U", path.display())
}

pub fn launcher_for(path: &Path) -> PathBuf {
    launchers().join(format!("{PREFIX}{}.desktop", slug(&path.file_name().unwrap_or_default().to_string_lossy())))
}

pub fn sync() -> usize {
    let _ = std::fs::create_dir_all(launchers());
    let mut wanted = std::collections::HashSet::new();
    for path in found() {
        if let Ok(meta) = std::fs::metadata(&path) {
            let mut perms = meta.permissions();
            if perms.mode() & 0o100 == 0 {
                perms.set_mode(perms.mode() | 0o755);
                let _ = std::fs::set_permissions(&path, perms);
            }
        }
        let target = launcher_for(&path);
        wanted.insert(target.file_name().unwrap_or_default().to_string_lossy().to_string());
        let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);

        let stamp = format!("{MARK}{}|{mtime}|{LAUNCHER_FORM}", path.display());
        if std::fs::read_to_string(&target).is_ok_and(|t| t.contains(&stamp)) {
            continue;
        }
        let inside = read_inside(&path);

        let icon = inside.icon.map(|p| format!("Icon={}\n", p.display())).unwrap_or_default();

        let class = inside.window.map(|w| format!("StartupWMClass={w}\n")).unwrap_or_default();
        let entry = format!("[Desktop Entry]\nType=Application\nName={}\nExec={}\n{icon}\
                             Categories={}\nTerminal={}\n{class}{stamp}\n",
                            inside.name, exec_line(&path), inside.categories, inside.terminal);
        let _ = std::fs::write(&target, entry);
    }
    if let Ok(entries) = std::fs::read_dir(launchers()) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with(PREFIX) && !wanted.contains(&name)
                && std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains(MARK))
            {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    wanted.len()
}

pub fn add(source: &Path) -> std::io::Result<Put> {
    std::fs::create_dir_all(store())?;
    let name = source.file_name().unwrap_or_default();
    let dest = store().join(name);
    let put = if std::fs::canonicalize(source).ok() == std::fs::canonicalize(&dest).ok() {
        Put::Already(dest)
    } else {
        crate::common::copies::copy_in(source, &store(), name)?
    };
    if let Put::Made(at) | Put::Already(at) = &put {
        ready(at)?;
    }
    Ok(put)
}

pub fn replace(taken: &Taken) -> std::io::Result<PathBuf> {
    crate::common::copies::replace(taken)?;
    ready(&taken.there)?;
    Ok(taken.there.clone())
}

fn ready(dest: &Path) -> std::io::Result<()> {
    let mut perms = std::fs::metadata(dest)?.permissions();
    perms.set_mode(perms.mode() | 0o755);
    std::fs::set_permissions(dest, perms)?;
    sync();
    Ok(())
}

pub fn remove(path: &Path) -> std::io::Result<()> {
    if path.parent().and_then(|p| std::fs::canonicalize(p).ok()) != std::fs::canonicalize(store()).ok() {
        return Err(std::io::Error::other(t!("icons-only-appimages")));
    }
    crate::common::removal::stop_boxed("--appimage", Some(path));

    match std::fs::remove_file(path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
        _ => {}
    }
    sync();
    let id = launcher_for(path).file_name().unwrap_or_default().to_string_lossy().into_owned();
    let _ = crate::common::favourites::unpin_app(&crate::common::favourites::file(), &id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{exec_line, nice_name, window_name};

    #[test]
    fn an_appimages_launcher_starts_it_in_its_box() {
        let line = exec_line(std::path::Path::new("/home/john/.appimage/Obsidian-1.6.7-x86_64.AppImage"));
        assert_eq!(line, "trindle-boxed --appimage \"/home/john/.appimage/Obsidian-1.6.7-x86_64.AppImage\" %U");
        let words = crate::common::keyfile::command_words(&line).unwrap();
        assert_eq!(words[..2], ["trindle-boxed", "--appimage"]);
        assert_eq!(words[2], "/home/john/.appimage/Obsidian-1.6.7-x86_64.AppImage");
    }

    #[test]
    fn names() {
        assert_eq!(nice_name("Obsidian-1.6.7-x86_64.AppImage"), "Obsidian");
        assert_eq!(nice_name("Waveframe_x86_64.AppImage"), "Waveframe");
        assert_eq!(nice_name("krita-5.2.3-x86_64.appimage"), "Krita");
    }

    #[test]
    fn its_windows_name_comes_from_the_launcher_inside() {
        assert_eq!(window_name("obsidian.desktop", Some(&"obsidian".to_string())).as_deref(), Some("obsidian"));
        assert_eq!(window_name("org.kde.krita.desktop", None).as_deref(), Some("org.kde.krita"));
        assert_eq!(window_name("x.desktop", Some(&"  ".to_string())).as_deref(), Some("x"));

        assert_eq!(window_name("y.desktop", Some(&"a\nExec=evil".to_string())).as_deref(), Some("y"));
    }
}

#[cfg(test)]
mod language_tests {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    use crate::toolkit::t;

    #[test]
    fn the_appimage_words_follow_the_language() {
        let french = "icons-only-appimages = Seulement les AppImages.\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(t!("icons-only-appimages"), "Seulement les AppImages.");
        });
    }
}
