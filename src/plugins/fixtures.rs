
use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::process::Command;
use std::time::{Duration, Instant};
use super::background::is_background_of;
use super::background::lock_free;
use super::background::take_lock;
use super::fields::choices_in;
use super::install::put_in;
use super::manifest::MANIFEST;
use super::manifest::Plugin;
use super::manifest::check;
use super::notes::DynItem;
use super::notes::RING_NOTE;
use super::notes::parse_ring_note;
use super::permissions::GRANTED;
use super::permissions::PERMISSIONS;
use super::*;

pub(super) fn install_at(root: &Path, from: &Path) -> Result<Plugin, String> {
    put_in(root, from).map(|i| i.plugin)
}

pub(super) fn folder_with(name: &str, ini: &str) -> PathBuf {

    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("trindle-plugin-{name}-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    std::fs::write(dir.join(MANIFEST), ini).unwrap();
    std::fs::write(dir.join("icon.svg"), "<svg/>").unwrap();
    std::fs::write(dir.join("bin/go"), "#!/bin/sh\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(dir.join("bin/go"), std::fs::Permissions::from_mode(0o755)).unwrap();
    dir
}

pub(super) const GOOD: &str = "[Plugin]\nInterface=1\nId=org.example.Weather\nName=Weather\nVersion=1.0\n\
                    Permissions=network;notifications\n\n[Ring]\nIcon=icon.svg\nLabel=Weather\nRun=bin/go --show\n\n\
                    [Settings]\nFields=town;units\n\n[Field town]\nKind=text\nLabel=Town\n\n\
                    [Field units]\nKind=choice\nLabel=Units\nChoices=Celsius;Fahrenheit\nDefault=Celsius\n";

pub(super) const MENU: &str = "[Plugin]\nInterface=1\nId=org.example.Weather\nName=Weather\nVersion=1.0\n\n\
                    [Ring]\nIcon=icon.svg\nLabel=Weather\nRun=bin/go --show\nMenu=connect;disconnect\n\n\
                    [Menu connect]\nLabel=Connect\nRun=bin/go connect\n\n\
                    [Menu disconnect]\nLabel=Disconnect\nRun=bin/go disconnect\n";

pub(super) fn says(ini: &str, name: &str) -> String {
    let dir = folder_with(name, ini);
    let said = check(&dir).err().map(|p| p.join("\n")).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);
    said
}

pub(super) fn home_with(name: &str, ring: Option<&[u8]>) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-ringnote-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    if let Some(b) = ring {
        std::fs::write(dir.join(RING_NOTE), b).unwrap();
    }
    dir
}

pub(super) fn items_of(json: &str) -> Vec<DynItem> {
    parse_ring_note(json.as_bytes()).items
}

pub(super) const FOUND: &str = "[Plugin]\nInterface=1\nId=a.b\nName=A\nVersion=1\n\n[Ring]\nIcon=icon.svg\nLabel=A\nRun=bin/go\n\n\
                     [Settings]\nFields=desk\n\n[Field desk]\nKind=choice\nLabel=Desktop\nChoicesFrom=found.txt\n";

pub(super) fn home_choices(name: &str, bytes: &[u8]) -> (PathBuf, Vec<(String, String)>) {
    let dir = home_with(name, None);
    std::fs::write(dir.join("found.txt"), bytes).unwrap();
    let got = choices_in(&dir, "found.txt");
    (dir, got)
}

pub(super) fn precious(dir: &Path, bytes: &[u8]) -> PathBuf {
    let p = dir.join("precious.txt");
    std::fs::write(&p, bytes).unwrap();
    p
}

pub(super) fn kept_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-kept-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub(super) fn made_up_keeper(dir: &Path, id: &str) -> std::process::Child {
    use std::os::unix::process::CommandExt;
    let child = Command::new("cat").arg0("trindle-plugin").args(["run", id, "background", "-"]).current_dir(dir)
        .stdin(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().unwrap();
    for _ in 0..250 {
        if is_background_of(child.id() as i32, id) {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    child
}

pub(super) const LET_GO_WAIT: Duration = Duration::from_secs(5);

pub(super) fn comes_free(dir: &Path) -> bool {
    let began = Instant::now();
    while !lock_free(dir) {
        if began.elapsed() >= LET_GO_WAIT {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    true
}

pub(super) fn lock_taken(dir: &Path) -> OwnedFd {
    let began = Instant::now();
    loop {
        if let Some(fd) = take_lock(dir) {
            return fd;
        }
        assert!(began.elapsed() < LET_GO_WAIT, "the lock never came free");
        std::thread::sleep(Duration::from_millis(10));
    }
}

pub(super) const LISTENING: &str = "\n[Network]\nListen=tcp:24810;udp:24811\n";

pub(super) fn listening(name: &str, ini: &str) -> Result<Plugin, Vec<String>> {
    let dir = folder_with(name, ini);
    let r = check(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    r
}

pub(super) fn with_listen(permissions: &str, network: &str) -> String {
    GOOD.replace("network;notifications", permissions) + network
}

pub(super) fn manifest(listen: &str) -> String {
    format!("[Plugin]\nInterface=1\nId=org.example.Sync\nName=Sync\nVersion=1\nPermissions=network;listen\n\n[Network]\nListen={listen}\n")
}

pub(super) fn tmp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-open-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("app")).unwrap();
    dir
}

pub(super) fn keep(at: &Path, ini: &str, allowed: &str, granted: Option<&str>) {
    std::fs::write(at.join("app").join(MANIFEST), ini).unwrap();
    std::fs::write(at.join("allowed"), allowed).unwrap();
    match granted {
        Some(g) => std::fs::write(at.join(GRANTED), g).unwrap(),
        None => {
            let _ = std::fs::remove_file(at.join(GRANTED));
        }
    }
}

pub(super) const ROLES: &str = "[Plugin]\nInterface=1\nId=org.example.Roles\nName=Roles\nVersion=1.0\n\n\
                     [Settings]\nFields=role;on;town;side;speed;desk\n\n\
                     [Field role]\nKind=choice\nLabel=This computer\nChoices=controls;controlled\nDefault=controls\n\n\
                     [Field on]\nKind=switch\nLabel=Switched on\n\n\
                     [Field town]\nKind=text\nLabel=Town\n\n\
                     [Field side]\nKind=choice\nLabel=Side\nChoices=left;right\nShowWhen=role=controls\n\n\
                     [Field speed]\nKind=switch\nLabel=Faster\nShowWhen=on=true\n\n\
                     [Field desk]\nKind=choice\nLabel=Desktop\nChoicesFrom=found.txt\nShowWhen=role=controls\n";

pub(super) fn vals(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

pub(super) fn two() -> Vec<(String, String)> {
    vec![("a".into(), "A".into()), ("b".into(), "B".into())]
}

pub(super) fn asking(permissions: &str) -> Result<Plugin, Vec<String>> {
    listening("scan-asks", &with_listen(permissions, ""))
}

pub(super) fn allowed_in_folder(at: &Path, p: &Plugin) -> Vec<String> {
    boxed::allowed_at(at, PERMISSIONS).into_iter().filter(|a| p.asks.contains(a)).collect()
}

pub(super) fn plugin_code() -> String {
    let files = [
        include_str!("mod.rs"),
        include_str!("background.rs"),
        include_str!("fields.rs"),
        include_str!("install.rs"),
        include_str!("manifest.rs"),
        include_str!("menu.rs"),
        include_str!("notes.rs"),
        include_str!("permissions.rs"),
        include_str!("ports.rs"),
    ];
    files
        .iter()
        .map(|f| crate::common::source::code(f).replace("#[cfg(test)]\nmod tests;\n", "").replace("#[cfg(test)]\nmod fixtures;\n", ""))
        .collect::<Vec<_>>()
        .join("\n")
}
