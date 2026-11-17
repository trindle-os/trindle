//! What a Flatpak program asks to reach, read before it is installed: from a bundle's own header, or from the remote a reference names. Nothing from the file is ever run.

use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use super::dropped::Reference;
use crate::toolkit::t;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub icon: &'static str,
    pub words: String,
}

const METADATA_MOST: usize = 256 * 1024;
const HEADER_MOST: u64 = 4 * 1024 * 1024;

const ENTRIES_MOST: usize = 256;
const LINES_MOST: usize = 200;
const TEXT_LINES_MOST: usize = 4000;

const ADD_SECONDS: &str = "30";
const INFO_SECONDS: &str = "90";

fn offset_size(total: u64) -> usize {
    match total {
        0..=0xff => 1,
        0x100..=0xffff => 2,
        0x1_0000..=0xffff_ffff => 4,
        _ => 8,
    }
}

fn offset_at(bytes: &[u8], at: usize, width: usize) -> Option<usize> {
    let raw = bytes.get(at..at.checked_add(width)?)?;
    let value = raw.iter().enumerate().fold(0u64, |v, (i, b)| v | (u64::from(*b) << (8 * i)));
    usize::try_from(value).ok()
}

fn align8(n: usize) -> Option<usize> {
    n.checked_add(7).map(|n| n & !7)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bundle {
    pub id: String,
    pub metadata: String,
}

pub fn read_bundle(file: &Path) -> Option<Bundle> {
    let mut f = std::fs::File::open(file).ok()?;
    let meta = f.metadata().ok().filter(|m| m.is_file())?;
    let size = meta.len();
    let w = offset_size(size);
    let tail = (2 * w) as u64;
    if size < tail + 16 {
        return None;
    }
    f.seek(SeekFrom::End(-(w as i64))).ok()?;
    let mut last = [0u8; 8];
    f.read_exact(&mut last[..w]).ok()?;
    let end = offset_at(&last, 0, w)? as u64;
    if end == 0 || end > HEADER_MOST || end > size - tail {
        return None;
    }
    f.rewind().ok()?;
    let mut head = vec![0u8; end as usize];
    f.read_exact(&mut head).ok()?;
    header(&head)
}

fn header(dict: &[u8]) -> Option<Bundle> {
    let m = dict.len();
    let w = offset_size(m as u64);
    let table = offset_at(dict, m.checked_sub(w)?, w)?;
    if table > m - w || (m - table) % w != 0 || (m - table) / w > ENTRIES_MOST {
        return None;
    }
    let (mut prev, mut reference, mut metadata) = (0usize, None, None);
    for i in 0..(m - table) / w {
        let start = align8(prev)?;
        let end = offset_at(dict, table + i * w, w)?;
        if start > end || end > table {
            return None;
        }
        match entry(&dict[start..end]) {
            Some((b"ref", text)) => reference = Some(text),
            Some((b"metadata", text)) => metadata = Some(text),
            _ => {}
        }
        prev = end;
    }
    Some(Bundle { id: app_id(&reference?)?, metadata: metadata? })
}

fn entry(e: &[u8]) -> Option<(&[u8], String)> {
    let w = offset_size(e.len() as u64);
    let stop = e.len().checked_sub(w)?;
    let key_end = offset_at(e, stop, w)?;
    if key_end == 0 || key_end > stop || e[key_end - 1] != 0 {
        return None;
    }
    let value = e.get(align8(key_end)?..stop)?;
    let p = value.iter().rposition(|b| *b == 0)?;
    if &value[p + 1..] != b"s" {
        return None;
    }
    let (nul, text) = value[..p].split_last()?;
    let text = String::from_utf8(text.to_vec()).ok().filter(|t| *nul == 0 && !t.contains('\0') && t.len() <= METADATA_MOST)?;
    Some((&e[..key_end - 1], text))
}

fn app_id(reference: &str) -> Option<String> {
    let mut parts = reference.split('/');
    let (kind, id) = (parts.next()?, parts.next()?);
    (kind == "app" && plain_name(id)).then(|| id.to_string())
}

fn plain_name(id: &str) -> bool {
    id.len() <= 255 && id.starts_with(|c: char| c.is_ascii_alphanumeric()) && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Peeked {
    pub metadata: String,
    pub commit: String,
}

pub fn read_ref(about: &Reference) -> Option<Peeked> {
    static COUNT: AtomicU32 = AtomicU32::new(0);
    let parent = private_parent();
    let _ = std::fs::create_dir_all(&parent);
    sweep_once(&parent);
    let dir = parent.join(format!("{PEEK}{}-{}", std::process::id(), COUNT.fetch_add(1, Ordering::Relaxed)));
    std::fs::DirBuilder::new().mode(0o700).create(&dir).ok()?;
    let found = remote_metadata(&dir, about);
    let _ = std::fs::remove_dir_all(&dir);
    found
}

fn private_parent() -> PathBuf {
    #[cfg(test)]
    if let Some(p) = TEST.with(|t| t.borrow().0.clone()) {
        return p;
    }
    super::github::cache()
}

#[cfg(test)]
thread_local! {
    static TEST: std::cell::RefCell<(Option<PathBuf>, Option<PathBuf>)> = const { std::cell::RefCell::new((None, None)) };
}

#[cfg(test)]
pub(crate) fn test_place(dir: PathBuf) {
    TEST.with(|t| t.borrow_mut().0 = Some(dir));
}

const PEEK: &str = "peek-";
const STALE_SECONDS: u64 = 3600;

fn sweep_once(parent: &Path) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| sweep(parent, std::time::Duration::from_secs(STALE_SECONDS)));
}

pub fn sweep(parent: &Path, age: std::time::Duration) {
    let Ok(entries) = std::fs::read_dir(parent) else { return };
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(rest) = name.strip_prefix(PEEK) else { continue };
        let old = e.metadata().ok().filter(|m| m.is_dir()).and_then(|m| m.modified().ok()).and_then(|t| t.elapsed().ok()).is_some_and(|d| d >= age);
        if old && !held(rest) {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
}

fn held(rest: &str) -> bool {
    let mut parts: Vec<&str> = rest.split('-').collect();
    parts.pop();
    parts.pop().and_then(|pid| pid.parse::<u32>().ok()).filter(|pid| *pid > 0).is_some_and(|pid| Path::new(&format!("/proc/{pid}")).exists())
}

pub fn target(about: &Reference) -> String {
    match &about.branch {
        Some(b) => format!("{}//{b}", about.name),
        None => about.name.clone(),
    }
}

pub fn commit_ok(commit: &str) -> bool {
    commit.len() == 64 && commit.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

fn remote_metadata(dir: &Path, about: &Reference) -> Option<Peeked> {
    let target = target(about);
    let dir = Some(dir);
    flatpak(dir, ADD_SECONDS, &["remote-add", "--user", "--no-gpg-verify", "--no-enumerate", REMOTE, &about.url], 4096)?;
    let commit = flatpak(dir, INFO_SECONDS, &["remote-info", "--user", "--show-commit", REMOTE, &target], 256)?.trim().to_string();
    if !commit_ok(&commit) {
        return None;
    }
    let at = format!("--commit={commit}");
    let metadata = flatpak(dir, INFO_SECONDS, &["remote-info", "--user", "--show-metadata", &at, REMOTE, &target], METADATA_MOST)?;
    Some(Peeked { metadata, commit })
}

pub fn pin_argv(about: &Reference, commit: &str) -> Vec<String> {
    let at = format!("--commit={commit}");
    crate::common::gentle::argv(&["flatpak", "update", "--user", "--noninteractive", "-y", &at, &target(about)]).into_iter().map(String::from).collect()
}

const REMOTE: &str = "peek";

fn flatpak(dir: Option<&Path>, seconds: &str, rest: &[&str], most: usize) -> Option<String> {
    let mut words = vec!["timeout", "-k", "3", seconds, "flatpak"];
    words.extend_from_slice(rest);
    let argv = crate::common::gentle::argv(&words);
    let mut command = std::process::Command::new(argv[0]);
    let command = crate::common::run::parsed(command.args(&argv[1..]));
    if let Some(dir) = dir {
        command.env("FLATPAK_USER_DIR", dir);
    }
    #[cfg(test)]
    if let Some(fake) = TEST.with(|t| t.borrow().1.clone()) {
        command.env("PATH", format!("{}:{}", fake.display(), std::env::var("PATH").unwrap_or_default()));
    }
    let mut child = command.process_group(0)
        .stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null()).stdout(std::process::Stdio::piped())
        .spawn().ok()?;
    let mut got = Vec::new();
    let read = child.stdout.take().map(|o| o.take(most as u64 + 1).read_to_end(&mut got));
    if got.len() > most {
        end_group(&mut child);
    }
    let ok = child.wait().map(|s| s.success()).unwrap_or(false);
    (ok && read.is_some_and(|r| r.is_ok()) && got.len() <= most).then(|| String::from_utf8(got).ok()).flatten()
}

pub fn private_copy(file: &Path) -> Option<PathBuf> {
    use std::os::unix::fs::OpenOptionsExt;
    let from = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open(file).ok()?;
    from.metadata().ok().filter(|m| m.is_file())?;
    private_file("bundle", "copy.flatpak", |out| crate::common::gentle::copy_file_whole(from, out).map(drop))
}

pub fn private_ref(about: &Reference) -> Option<PathBuf> {
    use std::io::Write;
    let text = about.written();
    private_file("ref", "copy.flatpakref", |mut out| out.write_all(text.as_bytes()))
}

fn private_file(kind: &str, name: &str, fill: impl FnOnce(std::fs::File) -> std::io::Result<()>) -> Option<PathBuf> {
    use std::os::unix::fs::OpenOptionsExt;
    static COUNT: AtomicU32 = AtomicU32::new(0);
    let parent = private_parent();
    std::fs::create_dir_all(&parent).ok()?;
    sweep_once(&parent);
    let dir = parent.join(format!("{PEEK}{kind}-{}-{}", std::process::id(), COUNT.fetch_add(1, Ordering::Relaxed)));
    std::fs::DirBuilder::new().mode(0o700).create(&dir).ok()?;
    let to = dir.join(name);
    let made = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&to).and_then(fill);
    if made.is_err() {
        let _ = std::fs::remove_dir_all(&dir);
        return None;
    }
    Some(to)
}

fn end_group(child: &mut std::process::Child) {
    if let Ok(pid) = i32::try_from(child.id()) {
        // SAFETY: kill(2) with a process group this program started and still holds.
        unsafe { libc::kill(-pid, libc::SIGKILL) };
    }
    let _ = child.kill();
}

pub fn is_installed(name: &str) -> bool {
    flatpak(None, ADD_SECONDS, &["list", "--app", "--columns=application"], 1 << 20).is_some_and(|out| listed(&out, name))
}

pub fn listed(list: &str, name: &str) -> bool {
    list.lines().any(|l| l.trim() == name)
}

pub fn bundle_install_argv(file: &Path) -> Vec<String> {
    let path = if file.is_absolute() || file.starts_with(".") || file.starts_with("..") { file.display().to_string() } else { format!("./{}", file.display()) };
    crate::common::gentle::argv(&["flatpak", "install", "--user", "--noninteractive", "-y", "--bundle", &path]).into_iter().map(String::from).collect()
}

struct Keyfile {
    groups: Vec<String>,
    items: Vec<(String, String, String)>,
}

impl Keyfile {
    fn read(text: &str) -> Option<Keyfile> {
        if text.lines().nth(TEXT_LINES_MOST).is_some() {
            return None;
        }
        let (mut group, mut groups, mut items) = (String::new(), Vec::new(), Vec::new());
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
            if let Some(g) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                group = g.to_string();
                groups.push(group.clone());
            } else if let Some((k, v)) = line.split_once('=') {
                items.push((group.clone(), k.trim().to_string(), v.trim().to_string()));
            }
        }
        Some(Keyfile { groups, items })
    }

    fn has_group(&self, group: &str) -> bool {
        self.groups.iter().any(|g| g == group)
    }

    fn values(&self, group: &str, key: &str) -> Vec<&str> {
        self.items.iter().filter(|(g, k, _)| g == group && k == key)
            .flat_map(|(_, _, v)| v.split(';')).map(str::trim).filter(|v| !v.is_empty()).collect()
    }

    fn pairs<'a>(&'a self, group: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.items.iter().filter(move |(g, _, _)| g == group).map(|(_, k, v)| (k.as_str(), v.as_str()))
    }
}

pub fn lines_of(text: &str) -> Option<Vec<Line>> {
    if text.len() > METADATA_MOST {
        return None;
    }
    let k = Keyfile::read(text)?;
    if !k.has_group("Application") && !k.has_group("Runtime") {
        return None;
    }
    let mut found: Vec<(u8, Line)> = Vec::new();
    sockets_and_devices(&k, &mut found);
    if other_settings(&k) {
        found.push(line(4, "dialog-information-symbolic", t!("store-fp-other")));
    }
    for item in k.values("Context", "filesystems") {
        files_lines(item, &mut found);
    }
    for (group, system) in [("Session Bus Policy", false), ("System Bus Policy", true)] {
        found.extend(k.pairs(group).filter_map(|(name, how)| bus_line(name, how, system)));
    }
    found.sort_by_key(|(rank, _)| *rank);
    let mut out: Vec<Line> = Vec::new();
    for (_, line) in found {
        if !out.contains(&line) {
            out.push(line);
        }
    }
    (out.len() <= LINES_MOST).then_some(out)
}

const HARMLESS: [(&str, &[&str]); 4] = [
    ("sockets", &["wayland", "pulseaudio", "x11", "fallback-x11", "session-bus", "system-bus", "ssh-auth", "gpg-agent", "pcsc", "cups"]),
    ("devices", &["all", "dri", "input", "kvm", "shm", "usb"]),
    ("shared", &["network", "ipc"]),
    ("features", &["devel", "bluetooth", "multiarch", "canbus", "per-app-dev-shm"]),
];
const GROUPS: [&str; 5] = ["Application", "Runtime", "Context", "Session Bus Policy", "System Bus Policy"];

fn other_settings(k: &Keyfile) -> bool {
    if k.groups.iter().any(|g| !GROUPS.contains(&g.as_str())) {
        return true;
    }
    k.pairs("Context").any(|(key, _)| !matches!(key, "filesystems" | "persistent" | "unset-environment") && !HARMLESS.iter().any(|(h, _)| *h == key))
        || HARMLESS.iter().any(|(key, known)| k.values("Context", key).iter().any(|v| !known.contains(&v.trim_start_matches('!'))))
}

fn line(rank: u8, icon: &'static str, words: impl Into<String>) -> (u8, Line) {
    (rank, Line { icon, words: words.into() })
}

fn sockets_and_devices(k: &Keyfile, found: &mut Vec<(u8, Line)>) {
    use crate::common::access::icon;
    let has = |group: &str, key: &str, v: &str| k.values(group, key).contains(&v);
    let table: [(&str, &str, &str, u8, &str, &str); 19] = [
        ("Context", "sockets", "session-bus", 0, "network-server-symbolic", t!("store-fp-session-bus")),
        ("Context", "sockets", "system-bus", 0, "network-server-symbolic", t!("store-fp-system-bus")),
        ("Context", "sockets", "ssh-auth", 0, "security-high-symbolic", t!("store-fp-ssh")),
        ("Context", "sockets", "gpg-agent", 0, "security-high-symbolic", t!("store-fp-gpg")),
        ("Context", "features", "devel", 0, "utilities-terminal-symbolic", t!("store-fp-devel")),
        ("Context", "devices", "all", 0, icon("devices"), t!("store-fp-devices")),
        ("Context", "sockets", "x11", 1, icon("x11"), t!("store-fp-x11")),
        ("Context", "sockets", "fallback-x11", 1, icon("x11"), t!("store-fp-x11")),
        ("Context", "devices", "kvm", 1, icon("devices"), t!("store-fp-kvm")),
        ("Context", "devices", "shm", 1, icon("devices"), t!("store-fp-shm")),
        ("Context", "devices", "usb", 1, icon("usb-devices"), t!("store-fp-usb")),
        ("Context", "features", "canbus", 1, icon("devices"), t!("store-fp-canbus")),
        ("Context", "sockets", "cups", 2, icon("devices"), t!("store-fp-cups")),
        ("Context", "features", "multiarch", 2, icon("devices"), t!("store-fp-multiarch")),
        ("Context", "shared", "network", 1, icon("network"), t!("store-fp-network")),
        ("Context", "features", "bluetooth", 1, icon("bluetooth"), t!("store-fp-bluetooth")),
        ("Context", "devices", "input", 1, icon("devices"), t!("store-fp-input")),
        ("Context", "sockets", "pcsc", 1, "security-high-symbolic", t!("store-fp-smartcard")),
        ("Context", "sockets", "pulseaudio", 2, icon("sound"), t!("store-fp-sound")),
    ];
    for (group, key, v, rank, picture, words) in table {
        if has(group, key, v) {
            found.push(line(rank, picture, words));
        }
    }
}

fn files_lines(item: &str, found: &mut Vec<(u8, Line)>) {
    if item.starts_with('!') {
        return;
    }
    let (place, mode) = match item.rsplit_once(':') {
        Some((p, m @ ("ro" | "rw" | "create"))) => (p, m),
        _ => (item, "rw"),
    };
    let read = mode == "ro";
    let (rank, what) = match place {
        "host" => (0, t!("store-fp-where-host").to_string()),
        "host-os" | "host-etc" => (0, t!("store-fp-where-system").to_string()),
        "home" | "~" => (0, t!("store-fp-where-home").to_string()),
        p if system_folder(p) => (0, t!("store-fp-where-system").to_string()),
        other => (1, t!("store-fp-where-path", path = place_words(other))),
    };
    let words = if read { t!("store-fp-files-read", what = what) } else { t!("store-fp-files", what = what) };
    if !read && lets_out(place) {
        found.push(line(0, crate::common::access::icon("computer"), t!("store-fp-escape")));
    }
    found.push(line(rank, "folder-symbolic", words));
}

fn system_folder(place: &str) -> bool {
    let p = if place.len() > 1 { place.trim_end_matches('/') } else { place };
    p == "/" || ["/usr", "/etc", "/var", "/opt", "/boot", "/srv", "/root", "/home"].contains(&p)
}

const XDG: [(&str, &str); 11] = [
    ("xdg-documents", "~/Documents"), ("xdg-download", "~/Downloads"), ("xdg-music", "~/Music"), ("xdg-pictures", "~/Pictures"),
    ("xdg-videos", "~/Videos"), ("xdg-desktop", "~/Desktop"), ("xdg-templates", "~/Templates"), ("xdg-public-share", "~/Public"),
    ("xdg-config", "~/.config"), ("xdg-cache", "~/.cache"), ("xdg-data", "~/.local/share"),
];

const PLACE_MOST: usize = 120;

fn place_words(place: &str) -> String {
    let shown = on_screen(place);
    let plain = !shown.is_empty() && shown.chars().count() <= PLACE_MOST
        && shown.chars().all(|c| c.is_alphanumeric() || matches!(c, ' ' | '/' | '.' | '_' | '-' | '~' | '+' | '@' | '=' | ','));
    if plain {
        return shown;
    }
    let mut out = String::from("\"");
    for c in shown.chars().take(PLACE_MOST) {
        match c {
            '"' | '\\' => out.extend(['\\', c]),
            c if c == ' ' || c.is_ascii_graphic() || (c.is_alphanumeric() && !c.is_ascii()) => out.push(c),
            c => out.push_str(&format!("\\u{{{:x}}}", u32::from(c))),
        }
    }
    if shown.chars().count() > PLACE_MOST {
        out.push('…');
    }
    out.push('"');
    out
}

fn on_screen(place: &str) -> String {
    let (first, rest) = place.split_once('/').map_or((place, None), |(f, r)| (f, Some(r)));
    let base = match first {
        "~" | "home" => "~".to_string(),
        "xdg-run" => run_folder(),
        x => match XDG.iter().find(|(name, _)| *name == x) {
            Some((_, folder)) => folder.to_string(),
            None => return place.to_string(),
        },
    };
    match rest.map(|r| r.trim_matches('/')) {
        Some(r) if !r.is_empty() => format!("{base}/{r}"),
        _ => base,
    }
}

fn run_folder() -> String {
    use std::os::unix::fs::MetadataExt;
    match std::env::var("XDG_RUNTIME_DIR") {
        Ok(d) if d.starts_with('/') => d.trim_end_matches('/').to_string(),
        _ => format!("/run/user/{}", std::fs::metadata("/proc/self").map_or(0, |m| m.uid())),
    }
}

const RUNS_CODE: [&str; 26] = [
    ".bashrc", ".bash_profile", ".bash_login", ".bash_logout", ".profile", ".zshrc", ".zprofile", ".zshenv", ".zlogin",
    ".xprofile", ".xsession", ".xsessionrc", ".xinitrc", ".pam_environment", ".ssh", "bin", ".local/bin",
    ".config/autostart", ".config/systemd", ".config/environment.d", ".config/fish", ".config/plasma-workspace",
    ".local/share/applications", ".local/share/systemd", ".local/share/dbus-1", ".local/share/flatpak",
];

fn lets_out(place: &str) -> bool {
    let shown = if place == "host" { "/".to_string() } else { on_screen(place) };
    let home = std::env::var("HOME").ok().filter(|h| h.starts_with('/')).map_or("/~".to_string(), |h| h.trim_end_matches('/').to_string());
    let whole = match shown.strip_prefix('~') {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => format!("{home}{rest}"),
        _ if shown.starts_with('/') => shown,
        _ => return false,
    };
    let steps: Vec<&str> = whole.split('/').filter(|s| !s.is_empty() && *s != ".").collect();
    if steps.contains(&"..") {
        return true;
    }
    let whole = format!("/{}", steps.join("/"));
    let within = |inner: &str, outer: &str| outer == "/" || inner == outer || inner.strip_prefix(outer).is_some_and(|r| r.starts_with('/'));
    RUNS_CODE.iter().map(|p| format!("{home}/{p}")).chain([format!("{}/bus", run_folder())])
        .any(|p| within(&p, &whole) || within(&whole, &p))
}

fn bus_name(name: &str) -> bool {
    let base = name.strip_suffix(".*").unwrap_or(name);
    !base.is_empty() && name.len() <= 255 && base.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

const ESCAPES: [&str; 2] = ["org.freedesktop.Flatpak", "org.freedesktop.systemd1"];

fn escapes(name: &str) -> bool {
    ESCAPES.iter().any(|e| {
        name == *e || name.strip_prefix(e).is_some_and(|rest| rest.starts_with('.'))
            || name.strip_suffix(".*").is_some_and(|base| *e == base || e.starts_with(&format!("{base}.")))
    })
}

fn bus_line(name: &str, how: &str, system: bool) -> Option<(u8, Line)> {
    if !bus_name(name) {
        return None;
    }
    if matches!(how, "talk" | "own") && escapes(name) {
        return Some(line(0, crate::common::access::icon("computer"), t!("store-fp-escape")));
    }
    let words = match (how, system) {
        ("talk", false) => t!("store-fp-talk", name = name),
        ("own", false) => t!("store-fp-own", name = name),
        ("talk", true) => t!("store-fp-talk-system", name = name),
        ("own", true) => t!("store-fp-own-system", name = name),
        _ => return None,
    };
    Some(line(3, "network-server-symbolic", words))
}

#[cfg(test)]
mod tests;
