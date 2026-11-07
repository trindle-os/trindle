//! AppImages and the programs Folders starts, run in the same bubblewrap box as a boxed .deb (store::boxed), though they are not installed in a folder of their own.

use crate::toolkit::t;
use std::path::{Path, PathBuf};

use super::{boxed, downloaded};
use crate::common::files::{data_home, home};

pub const APPIMAGE_STARTS: &[&str] = &["network", "sound", "x11"];

pub const PROGRAM_STARTS: &[&str] = &["network", "sound"];

const MOST_FILES: usize = 64;
const NAME_MOST: usize = 64;

const IMAGE_AT: &str = "/app/image.AppImage";

const NOT_SETTINGS: &[&str] = &[
    "autostart", "autostart-scripts", "systemd", "environment.d", "dconf", "trindle", "ssh", "gnupg", "gnupg2", "pki", "fontconfig",
    "gtk-2.0", "gtk-3.0", "gtk-4.0", "mimeapps", "applications", "icons", "themes", "fonts", "bash", "bashrc", "profile",
    "zsh", "zshrc", "zprofile", "local", "config", "cache", "share", "dbus-1", "pulse", "pipewire", "wireplumber", "keyrings",
    "password-store", "kwalletrc", "xdg", "user-dirs", "xinitrc", "xprofile", "xsession", "session", "login", "history",
    "bash_history", "bash_profile", "bash_login", "bash_logout", "netrc", "pam_environment", "inputrc", "mozilla-sync",
];

const SYSTEM: &[&str] = &["/usr", "/etc", "/proc", "/dev", "/sys", "/run", "/bin", "/sbin", "/lib", "/lib64", "/opt", "/var", "/app"];

pub fn root() -> PathBuf {
    data_home().join("trindle/loose")
}

pub fn id_for(name: &str) -> String {
    let s: String = name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let s = s.split('-').filter(|p| !p.is_empty()).collect::<Vec<_>>().join("-");
    let s: String = s.chars().take(64).collect();
    if s.is_empty() { "app".into() } else { s }
}

pub fn folder(kind: &str, id: &str) -> PathBuf {
    root().join(format!("{kind}-{}", id_for(id)))
}

pub const KINDS: &[&str] = &["appimage", "program"];

#[derive(Debug, Clone, PartialEq)]
pub struct Boxed {
    pub kind: &'static str,
    pub id: String,
    pub name: String,
    pub allowed: Vec<String>,
}

pub fn listed_in(top: &Path) -> Vec<Boxed> {
    let mut out: Vec<Boxed> = Vec::new();
    for entry in std::fs::read_dir(top).into_iter().flatten().flatten() {
        let file = entry.file_name().to_string_lossy().into_owned();
        let Some(kind) = KINDS.iter().copied().find(|k| file.strip_prefix(*k).is_some_and(|r| r.starts_with('-'))) else { continue };
        let id = file[kind.len() + 1..].to_string();
        let at = entry.path();
        if id.is_empty() || id_for(&id) != id || !real_folder(&at) || !real_folder(&at.join("home")) {
            continue;
        }
        let name = std::fs::read_to_string(at.join("name")).ok().map(|n| n.trim().chars().take(NAME_MOST).collect::<String>())
            .filter(|n| !n.is_empty() && !n.chars().any(char::is_control)).unwrap_or_else(|| id.clone());
        out.push(Boxed { kind, id, name, allowed: boxed::allowed_at(&at, boxed::ALLOWED) });
    }
    out.sort_by(|a, b| (a.kind, a.name.to_lowercase(), &a.id).cmp(&(b.kind, b.name.to_lowercase(), &b.id)));
    out
}

pub fn allow_in(top: &Path, kind: &str, id: &str, what: &str, on: bool) -> std::io::Result<()> {
    let known = KINDS.contains(&kind) && id_for(id) == id;
    let at = top.join(format!("{kind}-{id}"));
    if !known || !real_folder(&at) || !boxed::ALLOWED.contains(&what) {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, t!("store-loose-not-boxed")));
    }
    boxed::allow_at(&at, boxed::ALLOWED, what, on)
}

pub fn remember_name(at: &Path, name: &str) {
    let path = at.join("name");
    if !path.exists() {
        let _ = crate::common::files::write_text(&path, &format!("{}\n", name.chars().filter(|c| !c.is_control()).take(NAME_MOST).collect::<String>()));
    }
}

pub fn prepare(at: &Path, starts: &[&str]) -> std::io::Result<()> {
    std::fs::create_dir_all(at.join("home"))?;
    let allowed = at.join("allowed");
    if !allowed.exists() {
        let lines: Vec<String> = starts.iter().map(|s| s.to_string()).collect();
        crate::common::files::write_text(&allowed, &(lines.join("\n") + "\n"))?;
    }
    Ok(())
}

fn path_of(word: &str) -> Option<PathBuf> {
    let raw = match word.strip_prefix("file://") {
        Some(rest) => {
            let rest = rest.strip_prefix("localhost").unwrap_or(rest);
            let bytes = rest.as_bytes();
            let mut out = Vec::with_capacity(bytes.len());
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'%' && i + 3 <= bytes.len() {
                    let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
                    out.push(u8::from_str_radix(hex, 16).ok()?);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            String::from_utf8(out).ok()?
        }
        None => word.to_string(),
    };
    (raw.starts_with('/') && !raw.contains('\0')).then(|| PathBuf::from(raw))
}

pub fn handed(words: &[String], home: &Path, exists: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for word in words {
        let Some(p) = path_of(word) else { continue };
        if p.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::CurDir)) {
            continue;
        }
        if p == home || p == Path::new("/") || SYSTEM.iter().any(|s| p.starts_with(s)) || !exists(&p) || out.contains(&p) {
            continue;
        }
        out.push(p);
        if out.len() == MOST_FILES {
            break;
        }
    }
    out
}

pub fn own_settings(program: &str, home: &Path, is_folder: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let name = program.to_lowercase();
    let plain = name.len() >= 2
        && name.len() <= 64
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        && !name.starts_with(['-', '_'])
        && !NOT_SETTINGS.contains(&name.as_str());
    if !plain {
        return Vec::new();
    }
    [format!(".config/{name}"), format!(".local/share/{name}"), format!(".cache/{name}"), format!(".{name}")]
        .iter()
        .map(|rel| home.join(rel))
        .filter(|p| is_folder(p))
        .collect()
}

fn real_folder(p: &Path) -> bool {
    std::fs::symlink_metadata(p).is_ok_and(|m| m.is_dir())
}

pub fn binds(settings: &[PathBuf], files: &[PathBuf], program: Option<&Path>) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();
    for p in settings.iter().chain(files) {
        let s = p.to_string_lossy().into_owned();
        a.extend(["--bind".into(), s.clone(), s]);
    }
    if let Some(p) = program {
        let s = p.to_string_lossy().into_owned();
        a.extend(["--ro-bind".into(), s.clone(), s]);
    }
    a
}

fn plan(files: &[PathBuf], settings: &[PathBuf], program: Option<&Path>, allowed: Vec<String>) -> (Vec<String>, Vec<String>) {
    let kinds = if downloaded::on() {
        let from = downloaded::folder();
        downloaded::boxed_kinds(files, |p| downloaded::arrived(p, &from), &downloaded::as_is_in(&downloaded::as_is_path()))
    } else {
        Vec::new()
    };
    if !kinds.is_empty() {
        let _ = downloaded::remember_in(&downloaded::seen_path(), &kinds);
        boxed::x11::tell(t!("store-downloaded-opened"), t!("store-downloaded-why"));
    }
    shut(files, settings, program, allowed, !kinds.is_empty())
}

fn shut(files: &[PathBuf], settings: &[PathBuf], program: Option<&Path>, allowed: Vec<String>, downloaded: bool) -> (Vec<String>, Vec<String>) {
    if downloaded {
        (binds_closed(settings, files, program), downloaded::kept(&allowed))
    } else {
        (binds(settings, files, program), allowed)
    }
}

pub fn binds_closed(settings: &[PathBuf], files: &[PathBuf], program: Option<&Path>) -> Vec<String> {
    let mut a = binds(settings, &[], program);
    for p in files {
        let s = p.to_string_lossy().into_owned();
        a.extend(["--ro-bind".into(), s.clone(), s]);
    }
    a
}

pub fn appimage(file: &Path, args: &[String]) -> i32 {
    let Ok(file) = std::fs::canonicalize(file) else {
        eprintln!("trindle-boxed: {} is not there", file.display());
        return 1;
    };
    if !file.is_file() {
        eprintln!("trindle-boxed: {} is not a file", file.display());
        return 1;
    }
    let name = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let id = id_for(crate::common::apps::appimage_stem(&name));
    let at = folder("appimage", &id);
    if let Err(e) = prepare(&at, APPIMAGE_STARTS) {
        eprintln!("trindle-boxed: {}: {e}", at.display());
        return 1;
    }
    remember_name(&at, crate::common::apps::appimage_stem(&name));
    let home = home();
    let mut extra: Vec<String> = Vec::new();
    let image = file.to_string_lossy().into_owned();
    extra.extend(["--ro-bind".to_string(), image, IMAGE_AT.to_string()]);
    extra.extend(["--setenv".to_string(), "APPIMAGE_EXTRACT_AND_RUN".to_string(), "1".to_string()]);
    let mut command = vec![IMAGE_AT.to_string()];
    command.extend(args.iter().cloned());
    let (more, allowed) = plan(&handed(args, &home, |p| p.exists()), &[], None, boxed::allowed_at(&at, boxed::ALLOWED));
    extra.extend(more);
    boxed::run::run_boxed_extra(&at, &format!("appimage-{id}"), &allowed, &extra, &command)
}

pub fn program(id: &str, command: &[String]) -> i32 {
    let Some(first) = command.first() else { return 2 };
    let at = folder("program", id);
    if let Err(e) = prepare(&at, PROGRAM_STARTS) {
        eprintln!("trindle-boxed: {}: {e}", at.display());
        return 1;
    }
    let home = home();
    let named = command.iter().find(|a| *a != "env" && !a.ends_with("/env") && !a.contains('=') && !a.starts_with('-')).unwrap_or(first);
    let base = Path::new(named).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    remember_name(&at, if base.is_empty() { id } else { &base });
    let settings = own_settings(&base, &home, real_folder);
    let files = handed(&command[1..], &home, |p| p.exists());
    let found = crate::common::run::find(first);
    let outside = found.filter(|p| !SYSTEM.iter().any(|s| p.starts_with(s)) && p.is_file());
    let (extra, allowed) = plan(&files, &settings, outside.as_deref(), boxed::allowed_at(&at, boxed::ALLOWED));
    boxed::run::run_boxed_extra(&at, &format!("program-{}", id_for(id)), &allowed, &extra, command)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(w: &[&str]) -> Vec<String> {
        w.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_boxs_folder_name_is_plain_and_cannot_climb() {
        assert_eq!(id_for("Obsidian"), "obsidian");
        assert_eq!(id_for("../../.ssh"), "ssh");
        assert_eq!(id_for("a/b c"), "a-b-c");
        assert_eq!(id_for("///"), "app");
        assert!(id_for(&"x".repeat(500)).len() <= 64);
        assert!(folder("appimage", "../../x").starts_with(root()));
    }

    #[test]
    fn only_the_files_a_program_was_handed_come_into_its_box() {
        let home = PathBuf::from("/home/john");
        let all = |_: &Path| true;
        let got = handed(
            &words(&["--flag", "/home/john/Pictures/a b.png", "file:///home/john/Documents/c%20d.txt", "relative.txt", "/etc/shadow", "/home/john",
                     "/home/john/../root/x", "/usr/bin/x", "https://example.com/x", "/home/john/Pictures/a b.png"]),
            &home,
            all,
        );
        assert_eq!(got, [PathBuf::from("/home/john/Pictures/a b.png"), PathBuf::from("/home/john/Documents/c d.txt")]);
        assert!(handed(&words(&["/home/john/missing"]), &home, |_| false).is_empty(), "a file that is not there is not bound");
        let many: Vec<String> = (0..200).map(|i| format!("/home/john/Documents/{i}.txt")).collect();
        assert_eq!(handed(&many, &home, all).len(), MOST_FILES);
        assert!(path_of("file:///home/john/%zz").is_none());
    }

    #[test]
    fn a_program_keeps_its_own_settings_folders_and_no_one_elses() {
        let home = PathBuf::from("/home/john");
        let there = |p: &Path| p.starts_with("/home/john");
        let got = own_settings("Gimp", &home, there);
        assert_eq!(got.len(), 4);
        assert!(got.contains(&PathBuf::from("/home/john/.config/gimp")) && got.contains(&PathBuf::from("/home/john/.gimp")));
        for bad in ["autostart", "ssh", "gnupg", "systemd", "bashrc", "..", ".", "a/b", "x", "config", "-rf", "dbus-1"] {
            assert!(own_settings(bad, &home, there).is_empty(), "{bad} was given a settings folder");
        }
        assert!(own_settings("gimp", &home, |_| false).is_empty(), "only folders that are there");
        let room = std::env::temp_dir().join(format!("trindle-loose-links-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&room);
        std::fs::create_dir_all(room.join("real")).unwrap();
        std::os::unix::fs::symlink(room.join("real"), room.join("link")).unwrap();
        assert!(real_folder(&room.join("real")) && !real_folder(&room.join("link")));
        std::fs::remove_dir_all(&room).unwrap();
    }

    #[test]
    fn a_loose_programs_box_hides_the_persons_other_files() {
        let home = home();
        let handed_file = home.join("Pictures/a.png");
        let extra = binds(&[home.join(".config/gimp")], std::slice::from_ref(&handed_file), Some(Path::new("/home/john/bin/tool")));
        let gates = boxed::walls::Gates { extra, ..boxed::walls::Gates::default() };
        let at = PathBuf::from("/data/loose/program-gimp");
        let line = boxed::walls::walls(&at, &[], &gates, &words(&["gimp"])).join(" ");
        let binds_of_home: Vec<&str> = line.split(" --").filter(|p| p.contains(&*home.to_string_lossy())).collect();
        assert!(binds_of_home.iter().any(|b| b.starts_with("bind /data/loose/program-gimp/home")), "{line}");
        assert!(line.contains(&format!("--bind {} {}", handed_file.display(), handed_file.display())));
        assert!(line.contains("--ro-bind /home/john/bin/tool /home/john/bin/tool"));
        assert!(!line.contains("--share-net") && !line.contains("--bind-try"), "nothing allowed, nothing shared: {line}");
        assert!(!line.contains(&format!("--bind {} {}", home.display(), home.display())), "the whole home was bound: {line}");
        let open = boxed::walls::walls(&at, &words(&["network", "files"]), &gates, &words(&["gimp"])).join(" ");
        assert!(open.contains("--share-net") && open.contains("--bind-try"));
    }

    #[test]
    fn a_downloaded_files_run_has_the_file_read_only_and_no_network_or_folders() {
        let home = home();
        let file = home.join("Downloads/a.pdf");
        let all: Vec<String> = ["network", "sound", "microphone", "files", "screen", "x11", "computer"].iter().map(|s| s.to_string()).collect();
        let at = PathBuf::from("/data/loose/program-evince");
        let line = |downloaded: bool| {
            let (extra, allowed) = shut(std::slice::from_ref(&file), &[], None, all.clone(), downloaded);
            let gates = boxed::walls::Gates { extra, ..boxed::walls::Gates::default() };
            boxed::walls::walls(&at, &allowed, &gates, &words(&["evince"])).join(" ")
        };
        let shut_line = line(true);
        assert!(shut_line.contains(&format!("--ro-bind {} {}", file.display(), file.display())), "{shut_line}");
        assert!(!shut_line.contains(&format!("--bind {} {}", file.display(), file.display())), "writable: {shut_line}");
        assert!(!shut_line.contains("--share-net") && !shut_line.contains("--bind-try"), "{shut_line}");
        let open = line(false);
        assert!(open.contains(&format!("--bind {} {}", file.display(), file.display())) && open.contains("--share-net") && open.contains("--bind-try"));
    }

    #[test]
    fn a_new_box_starts_with_its_grants_and_keeps_the_persons_choices() {
        let room = std::env::temp_dir().join(format!("trindle-loose-box-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&room);
        let at = room.join("appimage-krita");
        prepare(&at, APPIMAGE_STARTS).unwrap();
        assert!(at.join("home").is_dir());
        assert_eq!(boxed::allowed_at(&at, boxed::ALLOWED), ["network", "sound", "x11"]);
        boxed::allow_at(&at, boxed::ALLOWED, "network", false).unwrap();
        boxed::allow_at(&at, boxed::ALLOWED, "files", true).unwrap();
        prepare(&at, APPIMAGE_STARTS).unwrap();
        assert_eq!(boxed::allowed_at(&at, boxed::ALLOWED), ["sound", "x11", "files"]);
        std::fs::remove_dir_all(&room).unwrap();
    }
}
