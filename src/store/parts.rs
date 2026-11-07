//! What a package in its own folder is built from.

use crate::toolkit::t;
use std::collections::{HashMap, HashSet};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const GRAPHICS: &[&str] = &[
    "libegl1", "libgl1", "libgles2", "libglx0", "libopengl0", "libglvnd0", "libgbm1", "libegl-mesa0",
    "libglx-mesa0", "libgl1-mesa-dri", "mesa-libgallium", "mesa-vulkan-drivers", "libvulkan1", "libdrm2",
];

const SERVICE_PLACES: &[&str] = &[
    "lib/modules/", "usr/lib/modules/", "lib/firmware/", "usr/lib/firmware/",
    "lib/systemd/system/", "usr/lib/systemd/system/", "lib/systemd/user/", "usr/lib/systemd/user/", "etc/systemd/",
    "lib/udev/", "usr/lib/udev/", "etc/udev/", "etc/init.d/",
    "etc/modprobe.d/", "lib/modprobe.d/", "usr/lib/modprobe.d/", "etc/modules-load.d/", "usr/lib/modules-load.d/",
    "usr/share/dbus-1/system-services/", "usr/share/dbus-1/system.d/", "etc/dbus-1/system.d/",
    "usr/lib/cups/", "usr/share/ppd/", "usr/share/cups/model/", "etc/sane.d/",
];

const MERGED: &[&str] = &["bin", "sbin", "lib", "lib64", "lib32", "libx32"];

pub fn groups(depends: &str) -> Vec<Vec<String>> {
    depends.split(',')
        .map(|g| g.split('|')
            .filter_map(|a| a.split_whitespace().next())
            .map(|a| a.split(':').next().unwrap_or(a).to_string())
            .filter(|a| !a.is_empty())
            .collect::<Vec<_>>())
        .filter(|g| !g.is_empty())
        .collect()
}

pub fn field_of(control: &str, name: &str) -> String {
    super::deb::fields(control).remove(name).unwrap_or_default()
}

pub fn needs(control: &str) -> Vec<Vec<String>> {
    let mut all = groups(&field_of(control, "Depends"));
    all.extend(groups(&field_of(control, "Pre-Depends")));
    all
}

fn qt_of(name: &str) -> Option<u8> {
    let n = name.to_ascii_lowercase();
    if n.starts_with("libqt6") || n.starts_with("qt6-") {
        Some(6)
    } else if n.starts_with("libqt5") || n.starts_with("qt5-") || n.starts_with("qtbase5") {
        Some(5)
    } else {
        None
    }
}

pub fn recommended_extras(control: &str, have: &HashSet<String>) -> Vec<Vec<String>> {
    let mut out = groups(&field_of(control, "Recommends"));
    for (qt, add_on) in [(6u8, "qt6-wayland"), (5, "qtwayland5")] {
        let listed = out.iter().any(|g| g.iter().any(|n| n == add_on));
        if have.iter().any(|n| qt_of(n) == Some(qt)) && !have.contains(add_on) && !listed {
            out.push(vec![add_on.to_string()]);
        }
    }
    out
}

pub fn base(status: &str) -> HashSet<String> {
    let mut have: HashMap<String, (String, String, bool)> = HashMap::new();
    for stanza in status.split("\n\n") {
        let f = super::deb::fields(stanza);
        let get = |k: &str| f.get(k).map(String::as_str).unwrap_or("");
        let name = get("Package");
        if name.is_empty() || !get("Status").ends_with(" installed") {
            continue;
        }
        let root = get("Essential") == "yes" || get("Priority") == "required" || GRAPHICS.contains(&name);
        let depends = format!("{}, {}", get("Depends"), get("Pre-Depends"));
        have.insert(name.to_string(), (depends, get("Provides").to_string(), root));
    }
    let mut answers: HashMap<String, Vec<String>> = HashMap::new();
    for (name, (_, provides, _)) in &have {
        for v in groups(provides).into_iter().flatten() {
            answers.entry(v).or_default().push(name.clone());
        }
    }
    let mut out: HashSet<String> = HashSet::new();
    let mut todo: Vec<String> = have.iter().filter(|(_, p)| p.2).map(|(n, _)| n.clone()).collect();
    while let Some(name) = todo.pop() {
        let Some((depends, _, _)) = have.get(&name) else { continue };
        if !out.insert(name) {
            continue;
        }
        for a in groups(depends).into_iter().flatten() {
            if have.contains_key(&a) {
                todo.push(a.clone());
            }
            todo.extend(answers.get(&a).cloned().unwrap_or_default());
        }
    }
    let provided: Vec<String> = out.iter().flat_map(|n| groups(&have[n].1).into_iter().flatten()).collect();
    out.extend(provided);
    out
}

pub fn base_here() -> HashSet<String> {
    base(&std::fs::read_to_string("/var/lib/dpkg/status").unwrap_or_default())
}

pub fn cannot_box(paths: &[String]) -> bool {
    paths.iter().map(|p| p.trim_start_matches("./")).any(|p| {
        SERVICE_PLACES.iter().any(|s| p.starts_with(s))
            || (p.starts_with("usr/src/") && p.ends_with("/dkms.conf"))
            || (p.starts_with("usr/lib/") && p.contains("/sane/"))
    })
}

pub fn walk(top: &Path) -> Vec<String> {
    fn go(dir: &Path, top: &Path, out: &mut Vec<String>) {
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = e.path();
            if e.file_type().is_ok_and(|t| t.is_dir()) {
                go(&path, top, out);
            } else if let Ok(rel) = path.strip_prefix(top) {
                out.push(rel.to_string_lossy().into_owned());
            }
        }
    }
    let mut out = Vec::new();
    go(top, top, &mut out);
    out.sort();
    out
}

pub fn programs(top: &Path) -> Vec<PathBuf> {
    use std::os::unix::fs::PermissionsExt;
    walk(top).into_iter().map(|r| top.join(r))
        .filter(|p| std::fs::symlink_metadata(p).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0))
        .filter(|p| is_elf(p))
        .collect()
}

pub fn is_elf(path: &Path) -> bool {
    let mut magic = [0u8; 4];
    std::fs::File::open(path).and_then(|f| f.read_exact_at(&mut magic, 0)).is_ok() && magic == *b"\x7fELF"
}

pub fn needed(path: &Path) -> Vec<String> {
    let Ok(file) = std::fs::File::open(path) else { return Vec::new() };
    let at = |offset: u64, len: usize| -> Option<Vec<u8>> {
        let mut buf = vec![0u8; len];
        file.read_exact_at(&mut buf, offset).ok().map(|()| buf)
    };
    let u16_at = |b: &[u8], i: usize| u16::from_le_bytes([b[i], b[i + 1]]) as u64;
    let u32_at = |b: &[u8], i: usize| u32::from_le_bytes(b[i..i + 4].try_into().unwrap_or([0; 4])) as u64;
    let u64_at = |b: &[u8], i: usize| u64::from_le_bytes(b[i..i + 8].try_into().unwrap_or([0; 8]));
    let Some(head) = at(0, 64) else { return Vec::new() };
    if head[..4] != *b"\x7fELF" || head[4] != 2 || head[5] != 1 {
        return Vec::new();
    }
    let (shoff, shentsize, shnum) = (u64_at(&head, 0x28), u16_at(&head, 0x3a), u16_at(&head, 0x3c));
    if shentsize < 64 || shnum == 0 || shnum > 4096 {
        return Vec::new();
    }
    let Some(table) = at(shoff, (shentsize * shnum) as usize) else { return Vec::new() };
    let section = |i: u64| -> &[u8] { &table[(i * shentsize) as usize..(i * shentsize + 64) as usize] };
    let mut out = Vec::new();
    for i in 0..shnum {
        let s = section(i);
        if u32_at(s, 4) != 6 {
            continue;
        }
        let (offset, size, link) = (u64_at(s, 24), u64_at(s, 32), u32_at(s, 40));
        if link >= shnum || size > 1 << 20 {
            continue;
        }
        let strings = section(link);
        let (str_off, str_size) = (u64_at(strings, 24), u64_at(strings, 32));
        let (Some(dynamic), Some(names)) = (at(offset, size as usize), at(str_off, str_size.min(1 << 24) as usize)) else { continue };
        for entry in dynamic.chunks_exact(16) {
            let (tag, value) = (u64_at(entry, 0), u64_at(entry, 8) as usize);
            if tag == 0 {
                break;
            }
            if tag == 1 && value < names.len() {
                let end = names[value..].iter().position(|&b| b == 0).map_or(names.len(), |e| value + e);
                out.push(String::from_utf8_lossy(&names[value..end]).into_owned());
            }
        }
    }
    out
}

pub fn system_libraries() -> Vec<PathBuf> {
    let triplet = triplet();
    [format!("/usr/lib/{triplet}"), format!("/lib/{triplet}"), "/usr/lib".into(), "/lib".into()]
        .into_iter().map(PathBuf::from).collect()
}

pub fn triplet() -> &'static str {
    if cfg!(target_arch = "aarch64") { "aarch64-linux-gnu" } else { "x86_64-linux-gnu" }
}

pub fn links(programs: &[PathBuf], dirs: &[PathBuf]) -> HashSet<String> {
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut names: HashSet<String> = HashSet::new();
    let mut todo: Vec<PathBuf> = programs.to_vec();
    while let Some(file) = todo.pop() {
        if !seen.insert(file.clone()) {
            continue;
        }
        for lib in needed(&file) {
            let beside = file.parent().map(Path::to_path_buf);
            if let Some(p) = beside.iter().chain(dirs).map(|d| d.join(&lib)).find(|p| p.is_file()) {
                todo.push(p);
            }
            names.insert(lib);
        }
    }
    names
}

pub fn store() -> PathBuf {
    super::boxed::folders().join(".parts")
}

pub fn key(package: &str, version: &str, arch: &str) -> String {
    format!("{package}_{version}_{arch}").replace(':', "%3a").replace('/', "_")
}

pub fn tree(key: &str) -> PathBuf {
    store().join(key).join("tree")
}

pub fn has(key: &str) -> bool {
    store().join(key).join("control").is_file()
}

pub fn control(key: &str) -> String {
    std::fs::read_to_string(store().join(key).join("control")).unwrap_or_default()
}

pub fn control_of(deb: &Path) -> String {
    Command::new("dpkg-deb").arg("-f").arg(deb)
        .args(["Package", "Version", "Architecture", "Depends", "Pre-Depends", "Provides"])
        .stderr(Stdio::null()).output().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default()
}

pub fn make(deb: &Path) -> Result<String, String> {
    let control = control_of(deb);
    let (p, v, a) = (field_of(&control, "Package"), field_of(&control, "Version"), field_of(&control, "Architecture"));
    if p.is_empty() || v.is_empty() || p.starts_with('.') {
        return Err(t!("store-parts-not-deb", path = deb.display().to_string()));
    }
    let key = key(&p, &v, &a);
    if has(&key) {
        return Ok(key);
    }
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNT: AtomicU32 = AtomicU32::new(0);
    let fresh = store().join(format!("{key}.new{}-{}", std::process::id(), COUNT.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_dir_all(&fresh);
    std::fs::create_dir_all(fresh.join("tree")).map_err(|e| e.to_string())?;
    let ok = crate::common::gentle::program(&mut Command::new("dpkg-deb")).arg("-x").arg(deb).arg(fresh.join("tree")).stderr(Stdio::null()).status()
        .is_ok_and(|s| s.success());
    let written = ok && std::fs::write(fresh.join("control"), &control).is_ok();
    if !written || (std::fs::rename(&fresh, store().join(&key)).is_err() && !has(&key)) {
        let _ = std::fs::remove_dir_all(&fresh);
        return Err(t!("store-parts-not-unpacked", path = deb.display().to_string()));
    }
    let _ = std::fs::remove_dir_all(&fresh);
    Ok(key)
}

pub fn link_tree(from: &Path, into: &Path) -> std::io::Result<()> {
    for e in std::fs::read_dir(from)?.flatten() {
        let name = e.file_name();
        let merged = MERGED.iter().any(|m| name == *m);
        if merged && e.file_type()?.is_symlink() {
            continue;
        }
        let to = if merged { into.join("usr").join(&name) } else { into.join(&name) };
        link_one(&e.path(), &to)?;
    }
    Ok(())
}

fn link_one(from: &Path, to: &Path) -> std::io::Result<()> {
    let kind = std::fs::symlink_metadata(from)?.file_type();
    let there = std::fs::symlink_metadata(to).ok().map(|m| m.file_type());
    if kind.is_dir() {
        if there.is_some_and(|t| !t.is_dir()) {
            std::fs::remove_file(to)?;
        }
        std::fs::create_dir_all(to)?;
        for e in std::fs::read_dir(from)?.flatten() {
            link_one(&e.path(), &to.join(e.file_name()))?;
        }
        return Ok(());
    }
    match there {
        Some(t) if t.is_dir() => return Ok(()),
        Some(_) => std::fs::remove_file(to)?,
        None => {}
    }
    if kind.is_symlink() {
        std::os::unix::fs::symlink(std::fs::read_link(from)?, to)
    } else {
        std::fs::hard_link(from, to).or_else(|_| crate::common::gentle::copy_named(from, to).map(|_| ()))
    }
}

fn in_use() -> HashSet<String> {
    let mut used = HashSet::new();
    for e in std::fs::read_dir(super::boxed::folders()).into_iter().flatten().flatten() {
        for list in ["parts", "parts.new"] {
            let text = std::fs::read_to_string(e.path().join(list)).unwrap_or_default();
            used.extend(text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from));
        }
    }
    used
}

pub fn sweep() {
    let used = in_use();
    for e in std::fs::read_dir(store()).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.contains(".new") && !used.contains(&name) {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_what_a_package_needs() {
        assert_eq!(groups("libc6 (>= 2.38), libfcft4t64 (>= 3.3.1) | libfcft4, python3:any"),
                   vec![vec!["libc6".to_string()], vec!["libfcft4t64".into(), "libfcft4".into()], vec!["python3".into()]]);
        assert!(groups("").is_empty());
        let control = "Package: a\nVersion: 1\nDepends: b, c | d\nPre-Depends: e\n";
        assert_eq!(needs(control), vec![vec!["b".to_string()], vec!["c".into(), "d".into()], vec!["e".into()]]);
    }

    #[test]
    fn the_base_is_what_the_system_cannot_do_without() {
        let status = "Package: libc6\nStatus: install ok installed\nPriority: optional\nDepends: libgcc-s1\n\n\
                      Package: libgcc-s1\nStatus: install ok installed\nPriority: optional\n\n\
                      Package: coreutils\nStatus: install ok installed\nEssential: yes\nPriority: required\n\
                      Pre-Depends: libc6 (>= 2.38), libselinux1\n\n\
                      Package: libselinux1\nStatus: install ok installed\nPriority: optional\nProvides: selinux-lib\n\n\
                      Package: mawk\nStatus: install ok installed\nPriority: required\nDepends: libc6\nProvides: awk\n\n\
                      Package: libwayland-client0\nStatus: install ok installed\nPriority: optional\nDepends: libc6\n\n\
                      Package: libgl1\nStatus: install ok installed\nPriority: optional\nDepends: libglx0\n\n\
                      Package: libglx0\nStatus: install ok installed\nPriority: optional\n\n\
                      Package: old\nStatus: deinstall ok config-files\nPriority: required\n";
        let base = base(status);
        for n in ["libc6", "libgcc-s1", "coreutils", "libselinux1", "selinux-lib", "mawk", "awk", "libgl1", "libglx0"] {
            assert!(base.contains(n), "{n} should be in the base");
        }
        assert!(!base.contains("libwayland-client0") && !base.contains("old"));
    }

    #[test]
    fn drivers_and_services_cannot_live_in_a_folder() {
        let p = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(!cannot_box(&p(&["./usr/bin/alacritty", "./usr/share/applications/Alacritty.desktop", "./etc/cron.daily/google-chrome"])));
        assert!(cannot_box(&p(&["usr/lib/systemd/system/thing.service"])));
        assert!(cannot_box(&p(&["./lib/modules/6.12.0/extra/wifi.ko"])));
        assert!(cannot_box(&p(&["./etc/udev/rules.d/60-dongle.rules"])));
        assert!(cannot_box(&p(&["./etc/init.d/printerd"])));
        assert!(cannot_box(&p(&["./usr/src/wifi-1.0/dkms.conf"])));
        assert!(cannot_box(&p(&["./usr/lib/cups/filter/brother"])));
        assert!(cannot_box(&p(&["./usr/lib/x86_64-linux-gnu/sane/libsane-epson.so"])));
    }

    fn tiny_elf(needs: &[&str]) -> Vec<u8> {
        let mut strings = vec![0u8];
        let mut at = Vec::new();
        for n in needs {
            at.push(strings.len() as u64);
            strings.extend(n.as_bytes());
            strings.push(0);
        }
        let str_off = 64u64;
        let dyn_off = (str_off + strings.len() as u64).next_multiple_of(8);
        let dyn_size = (needs.len() as u64 + 1) * 16;
        let sh_off = dyn_off + dyn_size;
        let mut b = vec![0u8; 64];
        b[..4].copy_from_slice(b"\x7fELF");
        b[4] = 2;
        b[5] = 1;
        b[0x28..0x30].copy_from_slice(&sh_off.to_le_bytes());
        b[0x3a..0x3c].copy_from_slice(&64u16.to_le_bytes());
        b[0x3c..0x3e].copy_from_slice(&3u16.to_le_bytes());
        b.extend(&strings);
        b.resize(dyn_off as usize, 0);
        for a in &at {
            b.extend(1u64.to_le_bytes());
            b.extend(a.to_le_bytes());
        }
        b.extend([0u8; 16]);
        let section = |kind: u32, off: u64, size: u64, link: u32| {
            let mut s = vec![0u8; 64];
            s[4..8].copy_from_slice(&kind.to_le_bytes());
            s[24..32].copy_from_slice(&off.to_le_bytes());
            s[32..40].copy_from_slice(&size.to_le_bytes());
            s[40..44].copy_from_slice(&link.to_le_bytes());
            s
        };
        b.extend(vec![0u8; 64]);
        b.extend(section(3, str_off, strings.len() as u64, 0));
        b.extend(section(6, dyn_off, dyn_size, 1));
        b
    }

    #[test]
    fn finds_wayland_through_a_programs_own_libraries() {
        let dir = std::env::temp_dir().join(format!("trindle-elf-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (bin, lib) = (dir.join("usr/bin"), dir.join("usr/lib"));
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::create_dir_all(&lib).unwrap();
        std::fs::write(bin.join("old"), tiny_elf(&["libX11.so.6", "libc.so.6"])).unwrap();
        std::fs::write(bin.join("new"), tiny_elf(&["libtoolkit.so.1", "libc.so.6"])).unwrap();
        std::fs::write(lib.join("libtoolkit.so.1"), tiny_elf(&["libwayland-client.so.0"])).unwrap();
        std::fs::write(bin.join("script"), "#!/bin/sh\n").unwrap();
        assert_eq!(needed(&bin.join("old")), ["libX11.so.6", "libc.so.6"]);
        assert!(needed(&bin.join("script")).is_empty());
        let dirs = [lib.clone()];
        let wayland = |p: &[PathBuf]| links(p, &dirs).iter().any(|l| l.starts_with("libwayland-client.so"));
        assert!(!wayland(&[bin.join("old")]), "an X11-only program");
        assert!(wayland(&[bin.join("new")]), "Wayland through its own library");
        assert!(wayland(&[bin.join("old"), bin.join("new")]));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parts_are_shared_by_hard_links() {
        use std::os::unix::fs::MetadataExt;
        let dir = std::env::temp_dir().join(format!("trindle-parts-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let part = dir.join("part");
        std::fs::create_dir_all(part.join("lib/x86_64-linux-gnu")).unwrap();
        std::fs::create_dir_all(part.join("usr/bin")).unwrap();
        std::fs::write(part.join("lib/x86_64-linux-gnu/libold.so.1"), "old").unwrap();
        std::fs::write(part.join("usr/bin/tool"), "tool").unwrap();
        std::os::unix::fs::symlink("tool", part.join("usr/bin/alias")).unwrap();
        for into in ["one", "two"] {
            link_tree(&part, &dir.join(into)).unwrap();
        }
        let one = dir.join("one/usr/bin/tool");
        assert_eq!(std::fs::read_to_string(&one).unwrap(), "tool");
        assert_eq!(std::fs::metadata(&one).unwrap().ino(), std::fs::metadata(dir.join("two/usr/bin/tool")).unwrap().ino());
        assert_eq!(std::fs::read_link(dir.join("one/usr/bin/alias")).unwrap(), PathBuf::from("tool"));
        assert!(dir.join("one/usr/lib/x86_64-linux-gnu/libold.so.1").is_file(), "/lib goes inside /usr");
        assert!(!dir.join("one/lib").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
