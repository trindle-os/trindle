//! Installed applications from .desktop files, searched in XDG order so user entries (including Hidden=true) override system ones.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::files::home;

pub const CATEGORIES: [&str; 9] =
    ["internet", "office", "graphics", "media", "games", "development", "learning", "utilities", "system"];

pub fn category_title(c: &str) -> &'static str {
    match c {
        "internet" => "Internet",
        "office" => "Office",
        "graphics" => "Graphics",
        "media" => "Sound and video",
        "games" => "Games",
        "development" => "Development",
        "learning" => "Learning",
        "utilities" => "Utilities",
        _ => "System",
    }
}

pub fn app_dirs() -> Vec<PathBuf> {
    let h = home();
    vec![
        h.join(".local/share/applications"),
        h.join(".local/share/flatpak/exports/share/applications"),
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        PathBuf::from("/usr/share/trindle/applications"),
        PathBuf::from("/usr/share/applications"),
    ]
}

fn category_of(cats: &str) -> &'static str {
    let has = |c: &str| cats.split(';').any(|x| x == c);
    for (piece, names) in [
        ("games", &["Game"][..]),
        ("development", &["Development", "IDE"][..]),
        ("internet", &["Network", "WebBrowser", "Email", "Chat", "InstantMessaging"][..]),
        ("office", &["Office"][..]),
        ("graphics", &["Graphics"][..]),
        ("media", &["AudioVideo", "Audio", "Video"][..]),
        ("learning", &["Education", "Science"][..]),
        ("utilities", &["Utility", "Accessories", "TextEditor", "Archiving", "Calculator"][..]),
    ] {
        if names.iter().any(|n| has(n)) {
            return piece;
        }
    }
    "system"
}

fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        if p.is_dir() {
            walk(base, &p, out);
        } else if p.extension().is_some_and(|x| x == "desktop") {
            if let Ok(rel) = p.strip_prefix(base) {
                out.push((rel.to_string_lossy().replace('/', "-"), p.clone()));
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub exec: String,
    pub try_exec: Option<String>,
    pub wm_class: String,
    pub program: String,
    pub kinds: Vec<String>,
    pub path: PathBuf,
    pub category: &'static str,
    pub keywords: Vec<String>,
    pub flatpak: Option<String>,
    pub appimage: Option<String>,
    pub terminal: bool,
}

impl Entry {
    pub fn key(&self) -> &str {
        self.id.strip_suffix(".desktop").unwrap_or(&self.id)
    }

    pub fn icons(&self) -> Vec<String> {
        let mut all = vec![format!("trindle-app-{}", self.key())];
        all.extend(self.icon.clone());
        all.push(generic_icon_for(self.key(), &self.name));
        all
    }
}

pub const GENERIC_ICONS: [&str; 20] = crate::toolkit::generic::NAMES;

const OLD_STAND_INS: [&str; 10] = [
    "application-x-executable-symbolic", "package-x-generic-symbolic", "application-x-addon-symbolic",
    "text-x-generic-symbolic", "image-x-generic-symbolic", "computer-symbolic", "video-display-symbolic",
    "audio-speakers-symbolic", "starred-symbolic", "preferences-system-symbolic",
];

fn is_old_stand_in(icon: &str) -> bool {
    OLD_STAND_INS.contains(&icon) || icon.starts_with("trindle-generic-")
}

pub fn generic_icon_for(id: &str, name: &str) -> String {
    crate::toolkit::generic::name_with_who(id, name)
}

fn languages() -> Vec<String> {
    let lang = ["LC_ALL", "LC_MESSAGES", "LANG"].iter()
        .filter_map(|v| std::env::var(v).ok()).find(|v| !v.is_empty()).unwrap_or_default();
    locale_names(&lang)
}

fn locale_names(lang: &str) -> Vec<String> {
    let (rest, modifier) = match lang.split_once('@') {
        Some((r, m)) => (r, Some(m)),
        None => (lang, None),
    };
    let rest = rest.split('.').next().unwrap_or("");
    if rest.is_empty() || rest == "C" || rest == "POSIX" {
        return Vec::new();
    }
    let (ll, cc) = match rest.split_once('_') {
        Some((l, c)) => (l, Some(c)),
        None => (rest, None),
    };
    let mut names = Vec::new();
    if let (Some(c), Some(m)) = (cc, modifier) {
        names.push(format!("{ll}_{c}@{m}"));
    }
    if let Some(c) = cc {
        names.push(format!("{ll}_{c}"));
    }
    if let Some(m) = modifier {
        names.push(format!("{ll}@{m}"));
    }
    names.push(ll.to_string());
    names
}

fn on_path(program: &str) -> bool {
    let p = Path::new(program);
    if p.is_absolute() {
        return if super::files::own_place(p) { p.is_file() } else { far_there(p, FAR_GRACE, |p| p.is_file()) };
    }
    std::env::var_os("PATH").is_some_and(|path| std::env::split_paths(&path).any(|d| d.join(program).is_file()))
}

const FAR_GRACE: std::time::Duration = std::time::Duration::from_millis(40);
const FAR_FRESH: std::time::Duration = std::time::Duration::from_secs(2);
const FAR_SHY: std::time::Duration = std::time::Duration::from_secs(10);
static FAR_LOOKS: super::run::Slots = super::run::Slots::new(8);

#[derive(Clone, Copy)]
enum Far {
    Out,
    Seen(bool, std::time::Instant),
}

struct Fars {
    known: HashMap<PathBuf, Far>,
    late: Option<std::time::Instant>,
    tell: Vec<crate::toolkit::event::Post<()>>,
}

fn fars() -> &'static std::sync::Mutex<Fars> {
    static FARS: std::sync::OnceLock<std::sync::Mutex<Fars>> = std::sync::OnceLock::new();
    FARS.get_or_init(|| std::sync::Mutex::new(Fars { known: HashMap::new(), late: None, tell: Vec::new() }))
}

fn far_there(path: &Path, grace: std::time::Duration, look: fn(&Path) -> bool) -> bool {
    use std::time::Instant;
    let Ok(mut fars) = fars().lock() else { return false };
    match fars.known.get(path) {
        Some(Far::Seen(there, when)) if when.elapsed() < FAR_FRESH => return *there,
        Some(Far::Out) => return false,
        _ => {}
    }
    let Some(slot) = FAR_LOOKS.take() else { return false };
    let shy = fars.late.is_some_and(|late| late.elapsed() < FAR_SHY);
    fars.known.insert(path.to_path_buf(), Far::Out);
    drop(fars);
    let (back, answer) = std::sync::mpsc::channel();
    let at = path.to_path_buf();
    let started = std::thread::Builder::new().name("app-there".into()).stack_size(128 << 10).spawn(move || {
        let _slot = slot;
        let there = look(&at);
        let Ok(mut fars) = self::fars().lock() else { return };
        fars.known.insert(at, Far::Seen(there, Instant::now()));
        if back.send(there).is_err() && there {
            fars.tell.retain(|tell| tell.send(()));
        }
    });
    if started.is_err() {
        if let Ok(mut fars) = self::fars().lock() {
            fars.known.remove(path);
        }
        return false;
    }
    let heard = if shy { answer.try_recv().ok() } else { answer.recv_timeout(grace).ok() };
    drop(answer);
    if let Some(there) = heard {
        return there;
    }
    let Ok(mut fars) = self::fars().lock() else { return false };
    if let Some(Far::Seen(there, _)) = fars.known.get(path) {
        return *there;
    }
    fars.late = Some(Instant::now());
    false
}

fn entry_from(id: &str, file_path: &Path, file: &super::keyfile::KeyFile, langs: &[String], desktops: &[String],
              found: impl Fn(&str) -> bool) -> Option<Entry> {
    const GROUP: &str = "Desktop Entry";
    let get = |key: &str| file.string(GROUP, key);
    let yes = |key: &str| get(key).is_some_and(|v| v == "true");
    if get("Type").as_deref() != Some("Application") || yes("Hidden") || yes("NoDisplay") {
        return None;
    }
    let listed = |key: &str| file.list(GROUP, key).map(|l| l.iter().any(|d| desktops.iter().any(|x| x.eq_ignore_ascii_case(d))));
    if listed("OnlyShowIn") == Some(false) || listed("NotShowIn") == Some(true) {
        return None;
    }
    let try_exec = get("TryExec").filter(|t| !t.is_empty());
    if try_exec.as_deref().is_some_and(|t| !found(t)) {
        return None;
    }
    let exec = get("Exec").unwrap_or_default();
    if exec.is_empty() && !yes("DBusActivatable") {
        return None;
    }
    let local = |key: &str| langs.iter().find_map(|l| get(&format!("{key}[{l}]"))).or_else(|| get(key));
    let name = local("Name")?;
    let mut keywords = langs.iter().find_map(|l| file.list(GROUP, &format!("Keywords[{l}]")))
        .or_else(|| file.list(GROUP, "Keywords")).unwrap_or_default();
    keywords.retain(|k| !k.is_empty());
    keywords.extend(local("GenericName").filter(|g| !g.is_empty()));
    Some(Entry {
        id: id.to_string(),
        name,
        icon: get("Icon").filter(|i| !i.is_empty() && !is_old_stand_in(i)),
        program: program_from(get("X-Flatpak").as_deref(), &exec),
        exec,
        try_exec,
        wm_class: get("StartupWMClass").map(|s| s.to_lowercase()).unwrap_or_default(),
        kinds: file.list(GROUP, "MimeType").unwrap_or_default(),
        path: file_path.to_path_buf(),
        category: category_of(&get("Categories").unwrap_or_default()),
        keywords,
        flatpak: get("X-Flatpak").filter(|f| !f.is_empty()),
        appimage: get("X-Trindle-AppImage").filter(|f| !f.is_empty()),
        terminal: yes("Terminal"),
    })
}

pub fn entries() -> Vec<Entry> {
    let langs = languages();
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_string).collect();
    let mut seen = HashSet::new();
    let mut all = Vec::new();
    for base in app_dirs() {
        let mut found = Vec::new();
        walk(&base, &base, &mut found);
        for (id, path) in found {
            if !seen.insert(id.clone()) {
                continue;
            }
            let Ok(file) = super::keyfile::KeyFile::load(&path) else { continue };
            all.extend(entry_from(&id, &path, &file, &langs, &desktops, on_path));
        }
    }
    all.sort_by_key(|a| a.name.to_lowercase());
    all
}

fn program_from(flatpak: Option<&str>, line: &str) -> String {
    if let Some(id) = flatpak {
        return id.to_lowercase();
    }
    let Some(mut argv) = super::keyfile::command_words(line) else { return String::new() };
    if let Some(at) = argv.iter().position(|a| a == "trindle-boxed" || a.ends_with("/trindle-boxed")) {
        let skip = if argv.get(at + 1).is_some_and(|a| a == "--loose") { 3 } else { 2 };
        argv.drain(..(at + skip).min(argv.len()));
    }
    argv.into_iter()
        .find(|a| a != "env" && !a.ends_with("/env") && !a.contains('=') && !a.starts_with('-'))
        .map(|a| Path::new(&a).file_name().map(|f| f.to_string_lossy().to_lowercase()).unwrap_or_default())
        .unwrap_or_default()
}

pub fn command_of(file: &super::keyfile::KeyFile, path: &Path) -> Option<(Vec<String>, Option<PathBuf>)> {
    command_with(file, path, &[])
}

fn command_with(file: &super::keyfile::KeyFile, path: &Path, files: &[String]) -> Option<(Vec<String>, Option<PathBuf>)> {
    const ENTRY: &str = "Desktop Entry";
    let line = file.string(ENTRY, "Exec")?;
    let link = |f: &String| match is_link(f) {
        true => f.clone(),
        false => crate::session::folders::file_uri(Path::new(f)),
    };
    let mut argv = Vec::new();
    for word in super::keyfile::command_words(&line)? {
        match word.as_str() {
            "%f" => argv.extend(files.first().cloned()),
            "%F" => argv.extend(files.iter().cloned()),
            "%u" => argv.extend(files.first().map(link)),
            "%U" => argv.extend(files.iter().map(link)),
            "%d" | "%D" | "%n" | "%N" | "%v" | "%m" => {}
            "%i" => {
                if let Some(icon) = file.string(ENTRY, "Icon") {
                    argv.extend(["--icon".to_string(), icon]);
                }
            }
            _ => argv.push(
                word.replace("%%", "\u{0}")
                    .replace("%c", &file.string(ENTRY, "Name").unwrap_or_default())
                    .replace("%k", &path.to_string_lossy())
                    .replace('\u{0}', "%"),
            ),
        }
    }
    let folder = file.string(ENTRY, "Path").filter(|p| !p.is_empty()).map(PathBuf::from);
    (!argv.is_empty()).then_some((argv, folder))
}

fn is_link(f: &str) -> bool {
    let Some((scheme, _)) = f.split_once(':') else { return false };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
        && !f.starts_with('/')
}

pub fn with_own_x11(argv: Vec<String>) -> Vec<String> {
    let flatpak = argv.first().is_some_and(|p| Path::new(p).file_name().is_some_and(|n| n == "flatpak"))
        && argv.get(1).is_some_and(|a| a == "run");
    let Some(id) = argv.iter().skip(2).find(|a| !a.starts_with('-')).filter(|_| flatpak).cloned() else { return argv };
    ["trindle-boxed".to_string(), "--own-x11".to_string(), id].into_iter().chain(argv).collect()
}

pub fn launch(app: &Entry, files: &[String]) -> bool {
    launch_with(app, files, None)
}

pub fn launch_with(app: &Entry, files: &[String], token: Option<String>) -> bool {
    launch_inner(app, files, token, false)
}

pub fn loose_argv(app: &Entry, argv: &[String]) -> Option<Vec<String>> {
    let first = Path::new(argv.first()?).file_name()?.to_string_lossy().to_lowercase();
    let kept = app.terminal
        || app.flatpak.is_some()
        || app.appimage.is_some()
        || own(app.key()).is_some()
        || first == "trindle"
        || first.starts_with("trindle-")
        || ["flatpak", "bwrap", "snap"].contains(&first.as_str());
    if kept {
        return None;
    }
    let id = crate::store::loose::id_for(app.key());
    Some(["trindle-boxed".to_string(), "--loose".to_string(), id].into_iter().chain(argv.iter().cloned()).collect())
}

fn any_downloaded(files: &[String]) -> bool {
    use crate::store::downloaded;
    if !downloaded::on() || files.is_empty() {
        return false;
    }
    let from = downloaded::folder();
    files.iter().filter(|f| !is_link(f)).any(|f| downloaded::arrived(Path::new(f), &from))
}

fn launch_inner(app: &Entry, files: &[String], token: Option<String>, boxed: bool) -> bool {
    let Ok(file) = super::keyfile::KeyFile::load(&app.path) else { return false };
    if app.exec.is_empty() {
        let path = format!("/{}", app.key().replace(['.', '-'], "/"));
        return super::run::spawn(&["busctl", "--user", "call", app.key(), &path, "org.freedesktop.Application", "Activate", "a{sv}", "0"]);
    }
    let one_each = ["%f", "%u"].iter().any(|c| app.exec.contains(c)) && files.len() > 1;
    let rounds: Vec<&[String]> = match one_each {
        true => files.chunks(1).collect(),
        false => vec![files],
    };
    let mut started = false;
    for some in rounds {
        let Some((mut argv, folder)) = command_with(&file, &app.path, some) else { return false };
        if app.terminal {
            if let Some((_, inside)) = terminal_argv(&argv) {
                argv = inside;
            }
        }
        argv = with_own_x11(argv);
        if boxed || any_downloaded(some) {
            argv = loose_argv(app, &argv).unwrap_or(argv);
        }
        let (id, name, desktop) = (app.id.clone(), app.name.clone(), app.path.clone());
        let go = move |token: Option<String>| {
            let mut command = std::process::Command::new(&argv[0]);
            command.args(&argv[1..]).stdin(std::process::Stdio::null()).env("GIO_LAUNCHED_DESKTOP_FILE", &desktop);
            if let Some(folder) = folder {
                command.current_dir(folder);
            }
            if let Some(token) = token {
                command.env("XDG_ACTIVATION_TOKEN", token);
            }
            super::run::child(command, move |status| crate::common::crash::app_ended(&id, &name, status));
        };
        match token.clone() {
            Some(token) => go(Some(token)),
            None => super::run::vouched(go),
        }
        started = true;
    }
    started
}

pub fn watch(events: &crate::toolkit::event::Loop, changed: impl Fn() + 'static) {
    let changed = std::rc::Rc::new(changed);
    let found = changed.clone();
    let tell = events.post(move |()| found());
    if let Ok(mut fars) = fars().lock() {
        fars.tell.push(tell);
    }
    let Ok(watch) = crate::toolkit::watch::Watch::new() else { return };
    let watch = std::rc::Rc::new(std::cell::RefCell::new(watch));
    let cover = {
        let watch = watch.clone();
        move || for d in app_dirs() {
            let mut at = d.clone();
            while !at.is_dir() {
                match at.parent() {
                    Some(up) if up.as_os_str().len() > 1 => at = up.to_path_buf(),
                    _ => break,
                }
            }
            if at.is_dir() {
                let _ = watch.borrow_mut().folder(&at);
            }
        }
    };
    cover();
    let pending = std::rc::Rc::new(std::cell::Cell::new(false));
    let lp = events.clone();
    let fd = watch.borrow().fd();
    events.watch(fd, move |hung_up| {
        if hung_up {
            return crate::toolkit::event::Flow::Stop;
        }
        if watch.borrow().changed().is_empty() || pending.replace(true) {
            return crate::toolkit::event::Flow::Continue;
        }
        let (pending, changed, cover) = (pending.clone(), changed.clone(), cover.clone());
        lp.after(std::time::Duration::from_millis(500), move || {
            pending.set(false);
            cover();
            changed();
        });
        crate::toolkit::event::Flow::Continue
    });
}

pub struct Own {
    pub key: &'static str,
    pub desktop: &'static [&'static str],
    pub window: &'static [&'static str],
    pub command: &'static [&'static str],
}

impl Own {
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.desktop.iter().chain(self.window).chain(self.command).copied()
    }

    pub fn knows(&self, name: &str) -> bool {
        let name = name.to_lowercase();
        let name = name.strip_suffix(".desktop").unwrap_or(&name);
        self.names().any(|n| n == name)
    }

    pub fn start_with(&self) -> &'static str {
        self.command.iter().find(|c| super::run::find(c).is_some()).copied().unwrap_or(self.command[0])
    }
}

pub const OWN: &[Own] = &[
    Own { key: "settings", desktop: &["trindle-settings"], window: &["org.trindle.settings", "trindle-settings"], command: &["trindle-settings"] },
    Own {
        key: "store",
        desktop: &["trindle-apps", "trindle-store", "trindle-deb", "trindle-appimage"],
        window: &["org.trindle.apps", "org.trindle.store", "trindle-apps", "trindle-store"],
        command: &["trindle-apps", "trindle-store"],
    },
    Own { key: "activity", desktop: &["trindle-activity"], window: &["org.trindle.activity", "trindle-activity"], command: &["trindle-activity"] },
    Own {
        key: "files",
        desktop: &["trindle-folders", "trindle-files"],
        window: &["org.trindle.folders", "org.trindle.files", "trindle-folders", "trindle-files"],
        command: &["trindle-folders", "trindle-files"],
    },
    Own {
        key: "drives",
        desktop: &["trindle-drives", "trindle-disks"],
        window: &["org.trindle.drives", "org.trindle.disks", "trindle-drives", "trindle-disks"],
        command: &["trindle-drives", "trindle-disks"],
    },
    Own { key: "text", desktop: &["trindle-text"], window: &["org.trindle.text", "trindle-text"], command: &["trindle-text"] },
    Own { key: "welcome", desktop: &["trindle-welcome"], window: &["org.trindle.welcome", "trindle-welcome"], command: &["trindle-welcome"] },
    Own {
        key: "eye-dropper",
        desktop: &["trindle-eye-dropper"],
        window: &["org.trindle.eyedropperlist", "trindle-eye-dropper"],
        command: &["trindle-eye-dropper"],
    },
    Own { key: "help", desktop: &["trindle-help"], window: &["org.trindle.help", "trindle-help"], command: &["trindle-help"] },
    Own { key: "report", desktop: &["trindle-report"], window: &["org.trindle.report", "trindle-report"], command: &["trindle-report"] },
    Own { key: "screenshot", desktop: &["trindle-screenshot"], window: &["org.trindle.screenshot", "trindle-screenshot"], command: &["trindle-screenshot"] },
];

pub fn own(name: &str) -> Option<&'static Own> {
    OWN.iter().find(|o| o.knows(name))
}

pub fn own_key(key: &str) -> Option<&'static Own> {
    OWN.iter().find(|o| o.key == key)
}

pub fn newest_of<'a, T>(windows: &'a [T], app_id: impl Fn(&T) -> &str, program: &Own) -> Option<&'a T> {
    windows.iter().rev().find(|w| program.knows(app_id(w)))
}

pub fn same_name(a: &str, b: &str) -> bool {
    let (a, b) = (a.to_lowercase(), b.to_lowercase());
    let (a, b) = (a.strip_suffix(".desktop").unwrap_or(&a), b.strip_suffix(".desktop").unwrap_or(&b));
    a == b || own(a).is_some_and(|o| o.knows(b))
}

pub fn appimage_stem(filename: &str) -> &str {
    let lower = filename.to_lowercase();
    let stem = if lower.ends_with(".appimage") { &filename[..filename.len() - 9] } else { filename };
    for (i, c) in stem.char_indices() {
        if matches!(c, '-' | '_' | ' ') {
            let rest = &stem[i + 1..];
            let r = rest.to_lowercase();
            if rest.starts_with(|c: char| c.is_ascii_digit())
                || (r.starts_with('v') && rest[1..].starts_with(|c: char| c.is_ascii_digit()))
                || ["x86_64", "amd64", "aarch64", "linux", "appimage"].iter().any(|w| r.starts_with(w))
            {
                return &stem[..i];
            }
        }
    }
    stem
}

fn tier(app: &Entry, wanted: &str) -> Option<usize> {
    let last = |s: &str| s.rsplit('.').next().unwrap_or(s).to_string();
    let key = app.key().to_lowercase();
    let program = {
        let p = app.program.as_str();
        let p = if p.ends_with(".appimage") { appimage_stem(p) } else { p };
        p.strip_suffix(".sh").or_else(|| p.strip_suffix(".py")).unwrap_or(p).to_lowercase()
    };
    let flatpak = app.flatpak.as_deref().map(str::to_lowercase).unwrap_or_default();
    let tiers: [bool; 6] = [
        same_name(&key, wanted),
        !app.wm_class.is_empty() && same_name(&app.wm_class, wanted),
        !flatpak.is_empty() && flatpak == wanted,
        last(&key) == wanted,
        !program.is_empty() && (program == wanted || last(&program) == wanted || same_name(&program, wanted)),
        wanted.contains('.') && last(&key) == last(wanted),
    ];
    tiers.iter().position(|&t| t)
}

fn window_name(app_id: &str) -> String {
    let wanted = app_id.trim().to_lowercase();
    wanted.strip_suffix(".desktop").map(str::to_string).unwrap_or(wanted)
}

fn best_tier(apps: &[Entry], wanted: &str) -> Option<usize> {
    if wanted.is_empty() {
        return None;
    }
    apps.iter().filter_map(|a| tier(a, wanted)).min()
}

pub fn find_for_window<'a>(apps: &'a [Entry], app_id: &str) -> Option<&'a Entry> {
    let wanted = window_name(app_id);
    let best = best_tier(apps, &wanted)?;
    apps.iter().find(|a| tier(a, &wanted) == Some(best))
}

pub fn owns(apps: &[Entry], app: &Entry, app_id: &str) -> bool {
    let wanted = window_name(app_id);
    best_tier(apps, &wanted).is_some_and(|best| tier(app, &wanted) == Some(best))
}

pub fn opener_for(kind: &str) -> Option<Entry> {
    let all = entries();
    chosen_of(&all, kind).or_else(|| openers_of(&all, kind).into_iter().next())
}

pub fn start_file(path: &Path) -> Result<(), String> {
    start_file_saying(path, std::process::Stdio::null())
}

pub fn start_file_saying(path: &Path, stderr: std::process::Stdio) -> Result<(), String> {
    let file = super::keyfile::KeyFile::load(path).map_err(|e| e.to_string())?;
    let (argv, folder) = command_of(&file, path).ok_or("It has no command to run.")?;
    let argv = with_own_x11(argv);
    let mut command = std::process::Command::new(&argv[0]);
    command.args(&argv[1..]).stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null()).stderr(stderr);
    if let Some(folder) = folder {
        command.current_dir(folder);
    }
    let mut child = command.spawn().map_err(|e| format!("could not start {}: {e}", argv[0]))?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

pub fn opens(app: &Entry, kind: &str) -> bool {
    let family = kind.split('/').next().map(|f| format!("{f}/*"));
    app.kinds.iter().any(|t| t == kind || Some(t.as_str()) == family.as_deref())
}

pub fn openers_of(all: &[Entry], kind: &str) -> Vec<Entry> {
    all.iter().filter(|a| opens(a, kind)).cloned().collect()
}

fn mimeapps() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()).map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config")).join("mimeapps.list")
}

fn mimeapps_lists() -> Vec<PathBuf> {
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_lowercase).collect();
    let config = mimeapps();
    let mut dirs = vec![config.parent().map(Path::to_path_buf).unwrap_or_default()];
    let system = std::env::var("XDG_CONFIG_DIRS").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| "/etc/xdg".into());
    dirs.extend(system.split(':').filter(|d| !d.is_empty()).map(PathBuf::from));
    dirs.extend(app_dirs());
    lists_in(&dirs, &desktops)
}

fn lists_in(dirs: &[PathBuf], desktops: &[String]) -> Vec<PathBuf> {
    dirs.iter().flat_map(|d| {
        desktops.iter().map(|n| d.join(format!("{n}-mimeapps.list"))).chain(std::iter::once(d.join("mimeapps.list")))
    }).collect()
}

pub fn chosen_of(all: &[Entry], kind: &str) -> Option<Entry> {
    let found = |id: &str| all.iter().find(|a| a.id == id && opens(a, kind)).cloned();
    for list in mimeapps_lists() {
        let file = super::keyfile::KeyFile::load_or_empty(&list);
        for group in ["Default Applications", "Added Associations"] {
            if let Some(app) = file.list(group, kind).unwrap_or_default().iter().find_map(|id| found(id)) {
                return Some(app);
            }
        }
    }
    openers_of(all, kind).into_iter().next()
}

pub fn choose_opener(id: &str, kinds: &[&str]) -> Result<(), String> {
    choose_opener_in(&mimeapps(), id, kinds)
}

pub(crate) fn choose_opener_in(path: &std::path::Path, id: &str, kinds: &[&str]) -> Result<(), String> {
    let mut file = super::keyfile::KeyFile::load_kept(path).map_err(|e| format!("Nothing was changed, because {e}."))?;
    for kind in kinds {
        file.set("Default Applications", kind, id);
        let mut added = vec![id.to_string()];
        added.extend(file.list("Added Associations", kind).unwrap_or_default().into_iter().filter(|a| a != id));
        file.set("Added Associations", kind, &format!("{};", added.join(";")));
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    super::files::write_text(path, &file.text()).map_err(|e| e.to_string())
}

pub fn terminals(all: &[Entry]) -> Vec<Entry> {
    all.iter().filter(|e| is_terminal(&super::keyfile::KeyFile::load_or_empty(&e.path))).cloned().collect()
}

fn is_terminal(file: &super::keyfile::KeyFile) -> bool {
    file.list("Desktop Entry", "Categories").unwrap_or_default().iter().any(|c| c == "TerminalEmulator")
}

pub fn default_terminal(all: &[Entry]) -> Option<Entry> {
    let found = terminals(all);
    let lists: Vec<String> = terminal_lists().iter().filter_map(|l| std::fs::read_to_string(l).ok()).collect();
    chosen_terminal(&found, &lists).or_else(|| found.into_iter().next())
}

fn terminal_lists() -> Vec<PathBuf> {
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_lowercase).collect();
    let config = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).unwrap_or_else(|| home().join(".config"));
    let dirs = std::env::var("XDG_CONFIG_DIRS").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/etc/xdg".into());
    let mut all = vec![config];
    all.extend(dirs.split(':').filter(|d| !d.is_empty()).map(PathBuf::from));
    let mut lists = Vec::new();
    for dir in all {
        lists.extend(desktops.iter().map(|d| dir.join(format!("{d}-xdg-terminals.list"))));
        lists.push(dir.join("xdg-terminals.list"));
    }
    lists
}

fn chosen_terminal(found: &[Entry], lists: &[String]) -> Option<Entry> {
    lists.iter().flat_map(|t| t.lines()).map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('-'))
        .map(|l| l.trim_start_matches('+'))
        .map(|l| l.split_once(':').map_or(l, |(id, _)| id))
        .find_map(|id| found.iter().find(|e| e.id == id || e.key() == id).cloned())
}

pub fn terminal_argv(command: &[String]) -> Option<(Entry, Vec<String>)> {
    let term = default_terminal(&entries())?;
    let file = super::keyfile::KeyFile::load(&term.path).ok()?;
    let (argv, _) = command_with(&file, &term.path, &[])?;
    Some((term, with_command(argv, &file, command)))
}

fn with_command(mut argv: Vec<String>, file: &super::keyfile::KeyFile, command: &[String]) -> Vec<String> {
    if !command.is_empty() {
        let flag = file.string("Desktop Entry", "X-TerminalArgExec").unwrap_or_else(|| "-e".into());
        argv.extend(flag.split_whitespace().map(str::to_string));
        argv.extend(command.iter().cloned());
    }
    argv
}

pub fn start_terminal(folder: Option<&Path>, command: &[String]) -> bool {
    let Some((term, argv)) = terminal_argv(command) else { return false };
    let (id, name, desktop, folder) = (term.id.clone(), term.name.clone(), term.path.clone(), folder.map(Path::to_path_buf));
    super::run::vouched(move |token| {
        let mut command = std::process::Command::new(&argv[0]);
        command.args(&argv[1..]).stdin(std::process::Stdio::null()).env("GIO_LAUNCHED_DESKTOP_FILE", &desktop);
        if let Some(folder) = folder {
            command.current_dir(folder);
        }
        if let Some(token) = token {
            command.env("XDG_ACTIVATION_TOKEN", token);
        }
        super::run::child(command, move |status| crate::common::crash::app_ended(&id, &name, status));
    });
    true
}

#[cfg(test)]
pub fn generic_icon(key: &str) -> &'static str {
    crate::toolkit::generic::name_for(key)
}

#[cfg(test)]
mod tests;
