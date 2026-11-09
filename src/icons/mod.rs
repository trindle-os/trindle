//! trindle-icons: fills the gaps in the chosen Trindle icon set.

pub mod appimages;

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use crate::common::apps::app_dirs;
use crate::common::files::{home, own_place, read_text, write_text};

const ELSEWHERE_WAIT: std::time::Duration = std::time::Duration::from_secs(1);

static ELSEWHERE_READS: crate::common::run::Slots = crate::common::run::Slots::new(2);

static GAVE_UP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

enum Seen {
    Is(Vec<u8>),
    Missing,

    Unknown,
}

fn alone_elsewhere() -> bool {
    use std::os::fd::AsRawFd;
    static HELD: std::sync::OnceLock<Option<std::fs::File>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| {
        let dir = crate::common::files::cache_home().join("trindle");
        std::fs::create_dir_all(&dir).ok()?;
        let lock = std::fs::OpenOptions::new().write(true).create(true).truncate(false).open(dir.join("icons-elsewhere.lock")).ok()?;
        // SAFETY: lock is an open file this function owns; flock only

        (unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0).then_some(lock)
    }).is_some()
}

fn elsewhere(file: &Path, read: fn(&Path) -> Option<Vec<u8>>, wait: std::time::Duration, may: fn() -> bool) -> Seen {
    use std::sync::atomic::Ordering::SeqCst;
    if GAVE_UP.load(SeqCst) || !may() {
        return Seen::Unknown;
    }
    let Some(place) = ELSEWHERE_READS.take() else { return Seen::Unknown };
    let (back, came) = std::sync::mpsc::channel();
    let file = file.to_path_buf();

    std::thread::spawn(move || {
        let _place = place;

        let _ = back.send(read(&file));
    });
    match came.recv_timeout(wait) {
        Ok(Some(data)) => Seen::Is(data),
        Ok(None) => Seen::Missing,
        Err(_) => {
            GAVE_UP.store(true, SeqCst);
            Seen::Unknown
        }
    }
}

fn whole_file(file: &Path) -> Option<Vec<u8>> {
    file.is_file().then(|| std::fs::read(file).ok()).flatten()
}

fn picture(file: &Path) -> Seen {
    if own_place(file) {
        return whole_file(file).map_or(Seen::Missing, Seen::Is);
    }
    elsewhere(file, whole_file, ELSEWHERE_WAIT, alone_elsewhere)
}

fn stays_here(entry: &std::fs::DirEntry, folder: bool) -> bool {
    let Ok(kind) = entry.file_type() else { return false };
    if kind.is_symlink() {
        let path = entry.path();
        return own_place(&path) && if folder { path.is_dir() } else { path.is_file() };
    }
    if folder { kind.is_dir() } else { kind.is_file() }
}

const MARK: &str = "<!-- made by trindle-icons -->";

pub const THEME: &str = "Trindle";

pub const CIRCLES: &str = "Trindle-Circle";
const SIZES: &[&str] = &["scalable", "512x512", "256x256", "192x192", "128x128", "96x96", "64x64", "48x48", "32x32"];

const PAPIRUS_SIZES: &[&str] = &["64x64", "128x128", "48x48", "32x32"];

const COLOURS: &[&str] = &["#e74c3c", "#e67e22", "#f1c40f", "#2ecc71", "#1abc9c", "#3498db", "#9b59b6", "#e84393",
    "#16a085", "#2980b9", "#8e44ad", "#d35400", "#c0392b", "#27ae60", "#0097a7", "#5c6bc0"];

fn theme_dir() -> PathBuf {
    home().join(".local/share/icons").join(THEME)
}

pub fn theme_made() -> bool {
    theme_dir().join("index.theme").is_file()
}

pub fn is_a_set(name: &str) -> bool {
    name == CIRCLES
}

pub fn chosen_set() -> &'static str {
    CIRCLES
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Shape {
    Disc,
}

fn shape_of(set: &str) -> Shape {
    let _ = set;
    Shape::Disc
}

fn data_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![home().join(".local/share")];
    let sys = std::env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/local/share:/usr/share".into());
    dirs.extend(sys.split(':').filter(|s| !s.is_empty()).map(PathBuf::from));
    dirs.push(home().join(".local/share/flatpak/exports/share"));
    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share"));
    dirs
}

fn color_for(key: &str) -> &'static str {

    let mut h: u64 = 0xcbf29ce484222325;
    for b in key.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    COLOURS[(h % COLOURS.len() as u64) as usize]
}

fn names_in_set(set: &str, dirs: &[PathBuf]) -> HashSet<String> {
    let mut names = HashSet::new();
    for base in dirs {
        names.extend(app_names(&base.join("icons").join(set)));
    }
    names
}

fn app_names(root: &Path) -> HashSet<String> {
    let mut names = HashSet::new();
    if !own_place(&root.join("index.theme")) {
        return names;
    }
    let index = read_text(root.join("index.theme"));
    let Some(line) = index.lines().find_map(|l| l.strip_prefix("Directories=")) else { return names };
    for part in line.split(',').map(str::trim).filter(|p| p.ends_with("apps") && own_place(&root.join(p))) {
        for e in std::fs::read_dir(root.join(part)).into_iter().flatten().flatten() {
            let path = e.path();
            if !path.extension().is_some_and(|x| x == "svg" || x == "png") {
                continue;
            }
            let Some(name) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else { continue };
            if !names.contains(&name) && stays_here(&e, false) {
                names.insert(name);
            }
        }
    }
    names
}

fn find_source(icon: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    let p = Path::new(icon);
    if p.is_absolute() {
        return Some(p.to_path_buf());
    }
    let papirus = dirs.iter().flat_map(|b| PAPIRUS_SIZES.iter().map(move |s| b.join("icons/Papirus").join(s).join("apps")));
    let hicolor = dirs.iter().flat_map(|b| SIZES.iter().map(move |s| b.join("icons/hicolor").join(s).join("apps")));
    let pixmaps = dirs.iter().map(|b| b.join("pixmaps"));
    papirus.chain(hicolor).chain(pixmaps)
        .flat_map(|d| ["svg", "png"].map(|e| d.join(format!("{icon}.{e}"))))
        .find(|f| own_place(f) && f.is_file())
}

fn base64(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len() * 4 / 3 + 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn nested(text: &str) -> Option<String> {
    let open = text.find("<svg")?;
    let close = text[open..].find('>')? + open;
    let head = &text[open..close];
    let body = text[close + 1..].rfind("</svg>").map(|e| &text[close + 1..close + 1 + e])?;
    let attr = |name: &str| {
        let at = head.find(&format!("{name}=\""))? + name.len() + 2;
        head[at..].find('"').map(|end| head[at..at + end].to_string())
    };
    let view = attr("viewBox").or_else(|| {
        let (w, h) = (attr("width")?, attr("height")?);
        let number = |v: String| v.trim_end_matches(|c: char| c.is_alphabetic()).parse::<f64>().ok();
        Some(format!("0 0 {} {}", number(w)?, number(h)?))
    })?;
    Some(format!("<svg x=\"10\" y=\"9\" width=\"28\" height=\"28\" viewBox=\"{view}\" \
                  preserveAspectRatio=\"xMidYMid meet\" xmlns=\"http://www.w3.org/2000/svg\" \
                  xmlns:xlink=\"http://www.w3.org/1999/xlink\">{body}</svg>"))
}

fn face(shape: Shape) -> &'static str {
    match shape {
        Shape::Disc => "<circle cx=\"24\" cy=\"23\" r=\"19.5\" fill=\"url(#face)\"/>\n\
                        <circle cx=\"24\" cy=\"23\" r=\"18.7\" fill=\"none\" stroke=\"#09090b\" stroke-opacity=\"0.32\" stroke-width=\"1.6\"/>",
    }
}

fn badge(shape: Shape, color: &str, source: Option<(&Path, Vec<u8>)>, letter: &str) -> String {
    let middle = match source {
        Some((s, data)) if s.extension().is_some_and(|e| e == "svg") => {

            match nested(&String::from_utf8_lossy(&data)) {
                Some(svg) => svg,
                None => format!("<image x=\"10\" y=\"9\" width=\"28\" height=\"28\" \
                                 preserveAspectRatio=\"xMidYMid meet\" \
                                 href=\"data:image/svg+xml;base64,{}\"/>", base64(&data)),
            }
        }
        Some((_, data)) => {
            format!("<image x=\"10\" y=\"9\" width=\"28\" height=\"28\" preserveAspectRatio=\"xMidYMid meet\" \
                     href=\"data:image/png;base64,{}\"/>", base64(&data))
        }
        None => format!("<text x=\"24\" y=\"30\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"20\" \
                         font-weight=\"bold\" fill=\"{color}\">{}</text>", escape(letter)),
    };
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"48\" height=\"48\" viewBox=\"0 0 48 48\">\n{MARK}\n\
             <defs>\n\
             <filter id=\"lift\" x=\"-30%\" y=\"-30%\" width=\"160%\" height=\"170%\">\n\
             <feDropShadow dx=\"0\" dy=\"1.6\" stdDeviation=\"1.5\" flood-color=\"#000000\" flood-opacity=\"0.45\"/>\n\
             </filter>\n\
             <linearGradient id=\"face\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\n\
             <stop offset=\"0\" stop-color=\"#ffffff\"/><stop offset=\"1\" stop-color=\"#e9e9ee\"/>\n\
             </linearGradient>\n\
             </defs>\n\
             <g filter=\"url(#lift)\">\n{}\n</g>\n{middle}\n</svg>\n", face(shape))
}

pub fn read_entry(path: &Path) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    let mut in_entry = false;
    for line in std::fs::read_to_string(path).unwrap_or_default().lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_entry = line == "[Desktop Entry]";
        } else if in_entry {
            if let Some((k, v)) = line.split_once('=') {
                values.entry(k.trim().to_string()).or_insert_with(|| v.trim().to_string());
            }
        }
    }
    values
}

fn desktop_files() -> Vec<(String, PathBuf)> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for base in app_dirs() {
        let mut stack = vec![base.clone()];
        while let Some(dir) = stack.pop() {

            if !own_place(&dir) {
                continue;
            }
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for e in entries.flatten() {
                let p = e.path();
                if stays_here(&e, true) {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "desktop") && stays_here(&e, false) {
                    let id = p.strip_prefix(&base).map(|r| r.to_string_lossy().replace('/', "-")).unwrap_or_default();
                    if seen.insert(id.clone()) {
                        out.push((id, p));
                    }
                }
            }
        }
    }
    out
}

struct Launcher {
    key: String,
    icon: String,
    letter: String,
}

fn launchers() -> Vec<Launcher> {
    let mut out = Vec::new();
    for (id, path) in desktop_files() {
        let e = read_entry(&path);
        if e.get("NoDisplay").map(String::as_str) == Some("true") || e.get("Hidden").map(String::as_str) == Some("true")
            || e.get("Type").map(String::as_str).unwrap_or("Application") != "Application"
        {
            continue;
        }
        let mut icon = e.get("Icon").cloned().unwrap_or_default();
        if !Path::new(&icon).is_absolute() {
            for ending in [".png", ".svg", ".xpm"] {
                if let Some(bare) = icon.strip_suffix(ending) {
                    icon = bare.to_string();
                }
            }
        }
        let name = e.get("Name").cloned().unwrap_or_else(|| id.clone());
        let letter = name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or("?".into());
        let key = id.strip_suffix(".desktop").unwrap_or(&id).to_string();
        out.push(Launcher { key, icon, letter });
    }
    out
}

fn plan<'a>(apps: &'a [Launcher], covered: &HashSet<String>) -> BTreeMap<String, &'a Launcher> {
    let mut todo = BTreeMap::new();
    for app in apps {
        if app.icon.is_empty() || Path::new(&app.icon).is_absolute() {
            todo.insert(format!("trindle-app-{}.svg", app.key), app);
        } else if !covered.contains(&app.icon) {
            todo.insert(format!("{}.svg", app.icon), app);
        }
    }
    todo
}

fn use_our_theme() {
    for dir in ["gtk-3.0", "gtk-4.0"] {
        let file = crate::common::files::config_home().join(dir).join("settings.ini");
        let text = read_text(&file);
        let ours = format!("gtk-icon-theme-name={THEME}");
        if text.is_empty() || text.lines().any(|l| l.trim() == ours) {
            continue;
        }
        let mut lines: Vec<String> =
            text.lines().filter(|l| !l.starts_with("gtk-icon-theme-name")).map(String::from).collect();
        let at = lines.iter().position(|l| l.trim() == "[Settings]").map(|i| i + 1).unwrap_or(0);
        lines.insert(at, ours);
        let _ = write_text(&file, &(lines.join("\n") + "\n"));
    }
    crate::common::dconf::copy_look();
}

fn index_for(set: &str) -> String {
    format!("[Icon Theme]\nName={THEME}\nComment=Icons Trindle makes for apps the Trindle sets do not draw\n\
             Inherits={set},Papirus,hicolor\nDirectories=scalable/apps\n\n\
             [scalable/apps]\nSize=48\nMinSize=8\nMaxSize=512\nType=Scalable\nContext=Applications\n")
}

fn write_index(set: &str) {
    let text = index_for(set);
    let file = theme_dir().join("index.theme");
    if read_text(&file) != text {
        let _ = write_text(&file, &text);
    }
}

pub fn main(args: Vec<String>) -> i32 {
    appimages::sync();
    let dirs = data_dirs();
    let set = chosen_set();
    let covered = names_in_set(set, &dirs);
    let out = theme_dir().join("scalable/apps");
    let _ = std::fs::create_dir_all(&out);
    write_index(set);
    let apps = launchers();
    let todo = plan(&apps, &covered);
    let made = draw_missing(&todo, &out, &dirs, shape_of(set));
    let removed = remove_gone(&todo, &out);
    use_our_theme();

    let _ = std::fs::File::open(theme_dir()).and_then(|f| f.set_modified(std::time::SystemTime::now()));
    if !args.iter().any(|a| a == "--quiet") {
        let missing = if covered.is_empty() { format!("{set} is not installed") } else { format!("{set} has {}", covered.len()) };
        println!("trindle-icons: {} applications, {missing}; {} made here, {made} drawn, {removed} removed",
                 apps.len(), todo.len());
    }
    0
}

fn draw_missing(todo: &BTreeMap<String, &Launcher>, out: &Path, dirs: &[PathBuf], shape: Shape) -> usize {
    let mut made = 0;
    for (file, app) in todo {
        let source = if app.icon.is_empty() { None } else { find_source(&app.icon, dirs) };
        let path = out.join(file);
        let seen = source.as_deref().map(|s| (s, picture(s)));
        let source = match seen {
            Some((s, Seen::Is(data))) => Some((s, data)),

            Some((_, Seen::Unknown)) if path.is_file() => continue,
            _ => None,
        };
        let svg = badge(shape, color_for(&app.key), source, &app.letter);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(svg.as_str()) {
            continue;
        }
        if std::fs::write(&path, &svg).is_ok() {
            made += 1;
        }
    }
    made
}

fn remove_gone(todo: &BTreeMap<String, &Launcher>, out: &Path) -> usize {
    let mut removed = 0;
    if let Ok(entries) = std::fs::read_dir(out) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".svg") && !todo.contains_key(&name)
                && std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains(MARK))
            {
                let _ = std::fs::remove_file(e.path());
                removed += 1;
            }
        }
    }
    let old = home().join(".local/share/icons/hicolor/scalable/apps");
    if let Ok(entries) = std::fs::read_dir(&old) {
        for e in entries.flatten() {
            if std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains(MARK)) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_on_a_share_that_has_stopped_answering_is_not_waited_for() {
        use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
        static READS: AtomicUsize = AtomicUsize::new(0);
        fn slow(_: &Path) -> Option<Vec<u8>> {
            READS.fetch_add(1, SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(3));
            None
        }
        fn quick(file: &Path) -> Option<Vec<u8>> {
            READS.fetch_add(1, SeqCst);
            file.ends_with("there.svg").then(|| b"<svg/>".to_vec())
        }
        let (yes, no): (fn() -> bool, fn() -> bool) = (|| true, || false);
        let wait = std::time::Duration::from_millis(80);
        GAVE_UP.store(false, SeqCst);

        assert!(matches!(elsewhere(Path::new("/far/there.svg"), quick, wait, yes), Seen::Is(d) if d == b"<svg/>"));
        assert!(matches!(elsewhere(Path::new("/far/gone.svg"), quick, wait, yes), Seen::Missing));

        let before = READS.load(SeqCst);
        assert!(matches!(elsewhere(Path::new("/far/there.svg"), quick, wait, no), Seen::Unknown));
        assert_eq!(READS.load(SeqCst), before);

        let began = std::time::Instant::now();
        assert!(matches!(elsewhere(Path::new("/far/dead.svg"), slow, wait, yes), Seen::Unknown));
        assert!(began.elapsed() < std::time::Duration::from_secs(2), "the read was waited for");
        let before = READS.load(SeqCst);
        assert!(matches!(elsewhere(Path::new("/far/there.svg"), quick, wait, yes), Seen::Unknown));
        assert_eq!(READS.load(SeqCst), before, "another read went out after one was late");
        GAVE_UP.store(false, SeqCst);

        assert_eq!(find_source("/far/anywhere/icon.png", &[]), Some(PathBuf::from("/far/anywhere/icon.png")));
        let code = crate::common::source::code(include_str!("mod.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        let body = |from: &str| {
            let rest = &code[code.find(from).unwrap_or_else(|| panic!("{from} is gone"))..];
            &rest[..rest.find("\n}\n").expect("the end of it")]
        };
        assert!(!body("\nfn badge(").contains("std::fs::"), "badge reads a picture for itself");
        let find = body("\nfn find_source(");
        assert_eq!((find.matches(".is_file()").count(), find.matches("own_place(f) && f.is_file()").count()), (1, 1));
        assert!(!body("\nfn desktop_files(").contains(".is_dir()") && !body("\nfn app_names(").contains(".exists()"));
        assert_eq!(code.matches("whole_file(").count(), 2, "a picture is read from somewhere but `picture`");

        let beside = std::env::current_exe().unwrap().ancestors().nth(2).unwrap().join("share").join(format!("trindle-icons-links-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&beside);
        std::fs::create_dir_all(&beside).unwrap();
        std::fs::write(beside.join("real.desktop"), "[Desktop Entry]\n").unwrap();
        std::os::unix::fs::symlink("real.desktop", beside.join("stays.desktop")).unwrap();
        std::os::unix::fs::symlink("/proc/version", beside.join("leaves.desktop")).unwrap();
        std::os::unix::fs::symlink("/proc/self", beside.join("leaves")).unwrap();
        let mut kept: Vec<String> = std::fs::read_dir(&beside).unwrap().flatten()
            .filter(|e| stays_here(e, false) || stays_here(e, true)).map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        kept.sort();
        assert_eq!(kept, ["real.desktop", "stays.desktop"]);
        std::fs::remove_dir_all(&beside).unwrap();
    }

    fn app(key: &str, icon: &str) -> Launcher {
        Launcher { key: key.into(), icon: icon.into(), letter: "A".into() }
    }

    #[test]
    fn base64_matches_the_standard() {
        assert_eq!(base64(b"Man"), "TWFu");
        assert_eq!(base64(b"Ma"), "TWE=");
        assert_eq!(base64(b"M"), "TQ==");
    }

    #[test]
    fn only_the_apps_the_set_lacks_are_made() {
        let covered: HashSet<String> = ["firefox", "org.gnome.DiskUtility"].map(String::from).into();
        let apps = [app("firefox", "firefox"), app("org.example.Odd", "odd-app"),
                    app("appimage-thing", "/home/x/.appimage/thing.png"), app("bare", ""),
                    app("trindle-drives", "org.gnome.DiskUtility")];
        let todo: Vec<String> = plan(&apps, &covered).into_keys().collect();
        assert_eq!(todo, ["odd-app.svg", "trindle-app-appimage-thing.svg", "trindle-app-bare.svg"]);
    }

    #[test]
    fn a_set_is_read_from_its_app_folders_links_and_all() {

        let root = std::env::current_exe().unwrap().ancestors().nth(2).unwrap().join("share").join(format!("trindle-icons-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for d in ["48x48/apps", "48x48@2x/apps", "48x48/places"] {
            std::fs::create_dir_all(root.join(d)).unwrap();
        }
        std::fs::write(root.join("index.theme"),
                       "[Icon Theme]\nName=T\nDirectories=48x48/apps,48x48@2x/apps,48x48/places\n").unwrap();
        std::fs::write(root.join("48x48/apps/firefox.svg"), "<svg/>").unwrap();
        std::fs::write(root.join("48x48@2x/apps/gimp.png"), "").unwrap();
        std::fs::write(root.join("48x48/places/folder.svg"), "<svg/>").unwrap();
        std::fs::write(root.join("48x48/apps/notes.txt"), "").unwrap();
        std::os::unix::fs::symlink("firefox.svg", root.join("48x48/apps/org.mozilla.firefox.svg")).unwrap();
        std::os::unix::fs::symlink("gone.svg", root.join("48x48/apps/broken.svg")).unwrap();
        let mut names: Vec<String> = app_names(&root).into_iter().collect();
        names.sort();
        assert_eq!(names, ["firefox", "gimp", "org.mozilla.firefox"]);
        std::fs::remove_dir_all(&root).unwrap();
        assert!(app_names(&root).is_empty());
    }

    #[test]
    fn a_saved_squares_value_loads_as_circles() {
        assert!(!is_a_set("Trindle-Square"));
        assert_eq!(chosen_set(), CIRCLES);
    }

    #[test]
    fn circles_are_round() {
        assert!(badge(Shape::Disc, "#000000", None, "A").contains("<circle"));
        assert_eq!(shape_of(CIRCLES), Shape::Disc);
    }

    #[test]
    fn the_theme_falls_through_to_the_chosen_set_then_papirus() {
        assert!(index_for(CIRCLES).contains("\nInherits=Trindle-Circle,Papirus,hicolor\n"));
        assert!(is_a_set(CIRCLES) && !is_a_set("Trindle-Square") && !is_a_set("Papirus") && !is_a_set("Numix-Circle"));
    }

    #[test]
    fn circles_are_the_default_everywhere() {
        assert_eq!(crate::common::files::default_setting("icon_source"), CIRCLES);
        let here = env!("CARGO_MANIFEST_DIR");
        for f in ["config/gtk-3.0/settings.ini", "config/gtk-4.0/settings.ini"] {
            let text = std::fs::read_to_string(Path::new(here).join(f)).unwrap();
            assert!(text.contains(&format!("\ngtk-icon-theme-name={CIRCLES}\n")), "{f}");
        }
        let schema = std::fs::read_to_string(Path::new(here).join("system/glib/90_trindle.gschema.override")).unwrap();
        assert!(schema.contains(&format!("icon-theme='{CIRCLES}'")));
    }
}
