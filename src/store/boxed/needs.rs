//! Working out what a package needs, and finding the parts that answer it.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::store::parts;
use super::categories_in;

pub fn wants_from(categories: &str, direct: &HashSet<String>, all: &HashSet<String>, programs: bool, depends: &str) -> Vec<String> {
    let cats: Vec<&str> = categories.split(';').map(str::trim).filter(|c| !c.is_empty()).collect();
    let cat = |c: &[&str]| c.iter().any(|w| cats.contains(w));
    let links = |set: &HashSet<String>, names: &[&str]| set.iter().any(|l| names.iter().any(|n| l.starts_with(n)));
    let wayland = links(all, &["libwayland-client.so"]);
    let x = links(all, &["libX11.so", "libxcb.so"]);
    let tk = parts::groups(depends).into_iter().flatten().any(|d| d.starts_with("tk") || d.starts_with("python3-tk"));
    let mut out = Vec::new();
    let mut want = |key: &str, yes: bool| if yes { out.push(key.to_string()) };
    want("network", links(direct, &["libcurl", "libssl", "libgnutls", "libsoup", "libnss3", "libwebkit"])
        || cat(&["Network", "WebBrowser", "Email", "Chat"]));
    want("sound", links(direct, &["libpulse", "libpipewire", "libasound", "libSDL2", "libopenal", "libgstreamer"])
        || cat(&["AudioVideo", "Audio", "Video"]));
    want("microphone", cat(&["Recorder", "Chat"]));
    want("files", cat(&["Office", "Graphics", "TextEditor", "AudioVideo", "Development", "FileManager", "Photography"])
        || (links(direct, &["libgtk-", "libQt"]) && !cat(&["Calculator", "Game"])));
    want("screen", cat(&["Accessibility"]) || (cat(&["Recorder"]) && cat(&["Video"])));
    want("x11", if programs { !wayland && x } else { tk });
    want("computer", cat(&["TerminalEmulator"]));
    out
}

pub(super) fn work_out_wants(main: &Path, files: &Path, control: &str) -> Vec<String> {
    let programs = parts::programs(main);
    let own: Vec<PathBuf> = programs.iter()
        .filter(|p| !p.file_name().is_some_and(|n| n.to_string_lossy().contains(".so"))).cloned().collect();
    let triplet = parts::triplet();
    let mut dirs = vec![files.join(format!("usr/lib/{triplet}")), files.join("usr/lib")];
    dirs.extend(parts::system_libraries());
    let direct: HashSet<String> = programs.iter().flat_map(|p| parts::needed(p)).collect();
    let all = parts::links(&own, &dirs);
    wants_from(&categories_in(main), &direct, &all, !own.is_empty(), &parts::field_of(control, "Depends"))
}

pub(super) fn debs_in(dir: &Path) -> Vec<PathBuf> {
    let mut all: Vec<PathBuf> = std::fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "deb")).collect();
    all.sort();
    all
}

pub(super) fn candidate(name: &str) -> Option<String> {
    let out = Command::new("apt-cache").args(["show", "--no-all-versions", "--", name])
        .stderr(Stdio::null()).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let first = text.split("\n\n").next().unwrap_or("").to_string();
    (out.status.success() && !parts::field_of(&first, "Version").is_empty()).then_some(first)
}

pub(super) fn download(name: &str, into: &Path) -> Option<PathBuf> {
    let before: HashSet<PathBuf> = debs_in(into).into_iter().collect();
    let ok = Command::new("apt-get").args(["download", "-q", "--", name]).current_dir(into)
        .stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success());
    ok.then(|| debs_in(into).into_iter().find(|p| !before.contains(p))).flatten()
}

pub(super) fn providers(name: &str) -> Vec<String> {
    Command::new("apt-cache").args(["showpkg", "--", name]).stderr(Stdio::null()).output()
        .map(|o| providers_in(&String::from_utf8_lossy(&o.stdout))).unwrap_or_default()
}

pub(super) fn providers_in(showpkg: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in showpkg.lines().skip_while(|l| !l.starts_with("Reverse Provides:")).skip(1) {
        match line.split_whitespace().next() {
            Some(p) if !out.iter().any(|o| o == p) => out.push(p.to_string()),
            _ => {}
        }
    }
    out
}

pub(super) fn answered(group: &[String], have: &HashSet<String>) -> bool {
    group.iter().any(|p| have.contains(p))
        || group.iter().any(|p| providers(p).iter().any(|q| have.contains(q)))
}

fn answers_to(control: &str, name: &str) -> bool {
    parts::field_of(control, "Package") == name
        || parts::groups(&parts::field_of(control, "Provides")).into_iter().flatten().any(|p| p == name)
}

pub(super) fn find_part(name: &str, local: &[(PathBuf, String)], downloads: &Path) -> Option<String> {
    find_named(name, local, downloads)
        .or_else(|| providers(name).iter().find_map(|p| find_named(p, local, downloads)))
}

fn find_named(name: &str, local: &[(PathBuf, String)], downloads: &Path) -> Option<String> {
    if let Some((deb, _)) = local.iter().find(|(_, c)| answers_to(c, name)) {
        if let Ok(key) = parts::make(deb) {
            return Some(key);
        }
    }
    if let Some(c) = candidate(name) {
        let key = parts::key(&parts::field_of(&c, "Package"), &parts::field_of(&c, "Version"), &parts::field_of(&c, "Architecture"));
        if parts::has(&key) {
            return Some(key);
        }
        if std::fs::create_dir_all(downloads).is_ok() {
            if let Some(deb) = download(name, downloads) {
                if let Ok(key) = parts::make(&deb) {
                    return Some(key);
                }
            }
        }
    }
    std::fs::read_dir(parts::store()).into_iter().flatten().flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|k| !k.contains(".new"))
        .find(|k| answers_to(&parts::control(k), name))
}
