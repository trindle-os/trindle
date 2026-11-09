//! What Uninstall does for an app in the ring, worked out from its
//! launcher, as the ring's Uninstall shows it. Shared with
//! trindle-admin, which checks a Debian package again as root.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::apps::Entry;
use crate::toolkit::t;

pub fn trindle() -> &'static str {
    t!("common-removal-trindle")
}
pub fn needed() -> &'static str {
    t!("common-removal-needed")
}
pub fn trindle_needs() -> &'static str {
    t!("common-removal-trindle-needs")
}

const OWN_LAUNCHERS: &str = ".local/share/applications";

const RENAMED: &str = "/usr/share/trindle/applications";
const LAUNCHERS: &str = "/usr/share/applications";

pub fn flatpak_uninstall(place: Option<&str>, id: &str, delete_data: bool) -> Vec<String> {
    let mut argv: Vec<String> = ["flatpak", "uninstall"].iter().map(|w| w.to_string()).collect();
    argv.extend(place.map(str::to_string));
    argv.extend(["-y", "--noninteractive"].map(String::from));
    if delete_data {
        argv.push("--delete-data".into());
    }
    argv.extend(["--".to_string(), id.to_string()]);
    argv
}

pub fn data_dir(home: &Path, id: &str) -> Option<PathBuf> {
    let plain = !id.is_empty() && !id.starts_with('.') && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    plain.then(|| home.join(".var/app").join(id))
}

pub fn data_size(dir: &Path) -> std::io::Result<u64> {
    let meta = match std::fs::symlink_metadata(dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        other => other?,
    };
    if !meta.is_dir() {
        return Ok(meta.len());
    }
    let mut sum = 0u64;
    for e in std::fs::read_dir(dir)? {
        sum = sum.saturating_add(data_size(&e?.path())?);
    }
    Ok(sum)
}

pub fn flatpak_running(id: &str) -> bool {
    Command::new("flatpak").args(["ps", "--columns=application"]).output().ok().filter(|o| o.status.success())
        .is_some_and(|o| String::from_utf8_lossy(&o.stdout).lines().any(|l| l.trim() == id))
}

pub fn delete_data_only(home: &Path, id: &str) -> Result<(), &'static str> {
    delete_data_only_unless(home, id, flatpak_running)
}

fn delete_data_only_unless(home: &Path, id: &str, running: impl FnOnce(&str) -> bool) -> Result<(), &'static str> {
    let dir = data_dir(home, id).ok_or(t!("common-removal-data-name"))?;
    match std::fs::symlink_metadata(&dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(t!("common-removal-data-failed")),
        Ok(m) if m.file_type().is_symlink() => Err(t!("common-removal-data-link")),
        Ok(m) if !m.is_dir() => Err(t!("common-removal-data-not-folder")),
        Ok(_) if running(id) => Err(t!("common-removal-data-running")),
        Ok(_) => std::fs::remove_dir_all(&dir).map_err(|_| t!("common-removal-data-failed")),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum How {

    Flatpak { place: Option<&'static str>, id: String, launcher: Option<PathBuf> },

    Image(PathBuf),

    Boxed(String),

    Package(String),

    Hide { launcher: PathBuf, id: String },

    Refused(&'static str),
}

impl How {

    pub fn only_hides(&self) -> bool {
        matches!(self, How::Hide { .. })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Facts {
    pub essential: bool,
    pub protected: bool,
    pub priority: String,
}

pub trait Look {
    fn home(&self) -> PathBuf;
    fn is_file(&self, p: &Path) -> bool;
    fn is_dir(&self, p: &Path) -> bool;

    fn owner(&self, p: &Path) -> Option<String>;
    fn facts(&self, package: &str) -> Facts;

    fn boxed(&self, name: &str) -> bool;
}

pub struct Real;

impl Look for Real {
    fn home(&self) -> PathBuf {
        super::files::home()
    }
    fn is_file(&self, p: &Path) -> bool {
        p.is_file()
    }
    fn is_dir(&self, p: &Path) -> bool {
        p.is_dir()
    }
    fn owner(&self, p: &Path) -> Option<String> {
        let out = Command::new("dpkg-query").arg("-S").arg(p).output().ok().filter(|o| o.status.success())?;
        owner_from(&String::from_utf8_lossy(&out.stdout))
    }
    fn facts(&self, package: &str) -> Facts {
        facts(package)
    }
    fn boxed(&self, name: &str) -> bool {
        crate::store::boxed::version(name).is_some()
    }
}

pub fn facts(package: &str) -> Facts {
    let out = Command::new("dpkg-query").args(["-W", "-f=${Essential}\\n${Protected}\\n${Priority}\\n", "--", package]).output();
    let text = out.ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
    let mut lines = text.lines().map(str::trim);
    Facts {
        essential: lines.next() == Some("yes"),
        protected: lines.next() == Some("yes"),
        priority: lines.next().unwrap_or("").to_string(),
    }
}

fn owner_from(said: &str) -> Option<String> {
    said.lines().filter(|l| !l.starts_with("diversion by")).find_map(|l| {
        let (who, _) = l.split_once(": ")?;
        let first = who.split(',').next()?.trim();
        let name = first.split(':').next()?.trim();
        (!name.is_empty()).then(|| name.to_string())
    })
}

pub fn trindle_own(package: &str) -> bool {
    let p = package.split(':').next().unwrap_or(package);
    p == "trindle" || p.starts_with("trindle-")
        || include_str!("../../packages.txt").lines().any(|l| l.split('#').next().unwrap_or("").trim() == p)
}

pub fn refusal(package: &str, f: &Facts) -> Option<&'static str> {
    if trindle_own(package) {
        Some(trindle())
    } else if f.essential || f.protected || matches!(f.priority.as_str(), "required" | "important") {
        Some(needed())
    } else {
        None
    }
}

pub fn taken_by(simulated: &str) -> Vec<String> {
    simulated.lines()
        .filter_map(|l| l.strip_prefix("Purg ").or_else(|| l.strip_prefix("Remv ")))
        .filter_map(|r| r.split_whitespace().next())
        .map(|p| p.split(':').next().unwrap_or(p).to_string())
        .collect()
}

pub fn refusal_of_all(package: &str, simulated: &str, facts: impl Fn(&str) -> Facts) -> Option<&'static str> {
    if let Some(why) = refusal(package, &facts(package)) {
        return Some(why);
    }
    taken_by(simulated).iter().filter(|p| *p != package).find_map(|p| refusal(p, &facts(p)))
        .map(|why| if why == trindle() { trindle_needs() } else { why })
}

fn flatpak_id_ok(id: &str) -> bool {
    id.len() <= 255 && id.split('.').count() >= 2 && !id.starts_with('-')
        && id.split('.').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'))
}

const RUN_VALUES: &[&str] = &["--arch", "--branch", "--command", "--cwd", "--runtime", "--runtime-version", "--commit",
                               "--runtime-commit", "--env", "--filesystem", "--nofilesystem", "--socket", "--device",
                               "--share", "--unshare", "--talk-name", "--own-name", "--system-talk-name", "--system-own-name",
                               "--persist", "--allow", "--disallow", "--nosocket", "--nodevice", "--unset-env", "--app-path",
                               "--usr-path", "--instance-id-fd", "--parent-pid"];

pub fn flatpak_id(app: &Entry) -> Option<String> {
    if let Some(id) = app.flatpak.as_deref().filter(|i| flatpak_id_ok(i)) {
        return Some(id.to_string());
    }
    let words = super::keyfile::command_words(&app.exec).unwrap_or_default();
    if let Some(at) = words.iter().position(|w| w == "flatpak" || w.ends_with("/flatpak")) {
        if words.get(at + 1).map(String::as_str) == Some("run") {
            let mut rest = words[at + 2..].iter();
            while let Some(w) = rest.next() {
                if RUN_VALUES.contains(&w.as_str()) {
                    rest.next();
                } else if !w.starts_with('-') && !w.starts_with("@@") {
                    let id = w.split('/').next().unwrap_or(w);
                    if flatpak_id_ok(id) {
                        return Some(id.to_string());
                    }
                    break;
                }
            }
        }
    }
    if app.path.to_string_lossy().contains("/flatpak/exports/share/applications/") {
        if let Some(id) = Some(app.key()).filter(|k| flatpak_id_ok(k)) {
            return Some(id.to_string());
        }
    }
    words.iter().chain(app.try_exec.iter()).find_map(|w| {
        ["/.var/app/", "/flatpak/app/"].iter().find_map(|mark| {
            let id = w.split_once(mark)?.1.split('/').next()?;
            flatpak_id_ok(id).then(|| id.to_string())
        })
    })
}

fn boxed_of(app: &Entry) -> Option<String> {
    let words = super::keyfile::command_words(&app.exec)?;
    let at = words.iter().position(|w| w == "trindle-boxed" || w.ends_with("/trindle-boxed"))?;
    words.get(at + 1).filter(|n| crate::store::deb::valid_name(n)).cloned()
}

pub fn how(app: &Entry, look: &dyn Look, images_there: &[PathBuf]) -> Option<How> {
    if super::favourites::is_path(&app.id) {
        return None;
    }
    let home = look.home();
    let own = home.join(OWN_LAUNCHERS);
    let mine = |p: &Path| p.starts_with(&own);
    if let Some(name) = boxed_of(app).filter(|n| look.boxed(n)) {
        return Some(How::Boxed(name));
    }
    if let Some(id) = flatpak_id(app) {
        let user = look.is_dir(&home.join(".local/share/flatpak/app").join(&id));
        let system = look.is_dir(&Path::new("/var/lib/flatpak/app").join(&id));
        let place = match (user, system) {
            (true, true) => Some(if app.path.starts_with(&home) { "--user" } else { "--system" }),
            (true, false) => Some("--user"),
            (false, true) => Some("--system"),
            (false, false) => None,
        };
        return Some(How::Flatpak { place, id, launcher: Some(app.path.clone()).filter(|p| mine(p)) });
    }
    let first = super::keyfile::command_words(&app.exec).and_then(|w| w.into_iter().next()).unwrap_or_default();
    let image = app.appimage.as_deref().and_then(|a| a.split('|').next()).map(PathBuf::from)
        .or_else(|| first.to_lowercase().ends_with(".appimage").then(|| PathBuf::from(&first)));

    if let Some(image) = image.filter(|i| i.parent() == Some(home.join(".appimage").as_path())) {
        return images_there.contains(&image).then_some(How::Image(image));
    }
    let hide = How::Hide { launcher: app.path.clone(), id: app.id.clone() };
    if app.path.starts_with(&home) {
        return Some(hide);
    }
    let mut asks = Vec::new();
    if app.path.starts_with(RENAMED) {
        asks.extend(app.path.file_name().map(|f| Path::new(LAUNCHERS).join(f)).filter(|p| look.is_file(p)));
    }
    asks.push(app.path.clone());
    Some(match asks.iter().find_map(|p| look.owner(p)).map(|o| o.split(':').next().unwrap_or(&o).to_string()) {
        Some(package) => match refusal(&package, &look.facts(&package)) {
            Some(why) => How::Refused(why),
            None => How::Package(package),
        },
        None => hide,
    })
}

fn hidden(text: &str) -> String {
    let mut out = String::new();
    let mut first = true;
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with('[') {
            inside = first && line.trim() == "[Desktop Entry]";
            out.push_str(line);
            out.push('\n');
            if inside {
                out.push_str("Hidden=true\n");
            }
            first = false;
            continue;
        }
        if inside && line.split('=').next().is_some_and(|k| k.trim() == "Hidden") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

pub fn hide(launcher: &Path, id: &str, home: &Path) -> std::io::Result<()> {
    let own = home.join(OWN_LAUNCHERS);
    let text = std::fs::read_to_string(launcher)?;
    if id.contains('/') || id.starts_with('.') {
        return Err(std::io::Error::other("not a launcher's name"));
    }
    let to = if launcher.starts_with(&own) { launcher.to_path_buf() } else { own.join(id) };
    super::files::write_text(&to, &hidden(&text))
}

pub fn stop_boxed(first: &str, path: Option<&Path>) {
    for pid in boxed_copies(first, path, std::fs::read_dir("/proc").into_iter().flatten().flatten().filter_map(|e| {
        let pid: i32 = e.file_name().to_str()?.parse().ok()?;
        Some((pid, std::fs::read(e.path().join("cmdline")).ok()?))
    })) {
        // SAFETY: kill with a process number and a signal; it touches no memory.
        unsafe { libc::kill(pid, libc::SIGTERM) };
    }
}

fn boxed_copies(first: &str, path: Option<&Path>, all: impl Iterator<Item = (i32, Vec<u8>)>) -> Vec<i32> {
    all.filter(|(_, raw)| {
        let words: Vec<&[u8]> = raw.split(|b| *b == 0).collect();
        let boxed = words.first().is_some_and(|w| w.ends_with(b"trindle-boxed"));
        let same = words.get(1).is_some_and(|w| *w == first.as_bytes());
        boxed && same && path.is_none_or(|p| words.get(2).is_some_and(|w| *w == p.as_os_str().as_encoded_bytes()))
    })
    .map(|(pid, _)| pid)
    .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn running_copies_of_a_boxed_program_and_an_appimage_are_found() {
        let cmd = |w: &[&str]| w.join("\0").into_bytes();
        let all = vec![
            (1, cmd(&["/usr/bin/trindle-boxed", "--appimage", "/h/.appimage/A.AppImage", "x"])),
            (2, cmd(&["/usr/bin/trindle-boxed", "--appimage", "/h/.appimage/B.AppImage"])),
            (3, cmd(&["trindle-boxed", "gimp", "gimp"])),
            (4, cmd(&["vim", "gimp"])),
        ];
        let a = Path::new("/h/.appimage/A.AppImage");
        assert_eq!(super::boxed_copies("--appimage", Some(a), all.clone().into_iter()), vec![1]);
        assert_eq!(super::boxed_copies("gimp", None, all.into_iter()), vec![3]);
    }

    use super::*;
    use std::collections::{HashMap, HashSet};

    #[derive(Default)]
    struct Fake {
        files: HashSet<PathBuf>,
        dirs: HashSet<PathBuf>,
        owners: HashMap<PathBuf, String>,
        facts: HashMap<String, Facts>,
        boxed: HashSet<String>,
    }

    impl Look for Fake {
        fn home(&self) -> PathBuf {
            PathBuf::from("/home/john")
        }
        fn is_file(&self, p: &Path) -> bool {
            self.files.contains(p)
        }
        fn is_dir(&self, p: &Path) -> bool {
            self.dirs.contains(p)
        }
        fn owner(&self, p: &Path) -> Option<String> {
            self.owners.get(p).cloned()
        }
        fn facts(&self, package: &str) -> Facts {
            self.facts.get(package).cloned().unwrap_or_default()
        }
        fn boxed(&self, name: &str) -> bool {
            self.boxed.contains(name)
        }
    }

    fn app(path: &str, exec: &str) -> Entry {
        let id = Path::new(path).file_name().unwrap().to_string_lossy().to_string();
        Entry { id: id.clone(), name: id, icon: None, exec: exec.into(), try_exec: None, wm_class: String::new(),
                program: String::new(), kinds: Vec::new(), path: path.into(), category: "system", keywords: Vec::new(),
                flatpak: None, appimage: None, terminal: false }
    }

    fn at_all(app: &Entry, look: &Fake) -> How {
        how(app, look, &[]).expect("something to do")
    }

    fn flat(place: Option<&'static str>, id: &str, launcher: Option<&str>) -> How {
        How::Flatpak { place, id: id.into(), launcher: launcher.map(PathBuf::from) }
    }

    const JD: &str = "org.jdownloader.JDownloader";

    #[test]
    fn a_flatpak_is_removed_with_its_data_only_when_asked() {
        for place in [None, Some("--user"), Some("--system")] {
            let keep = flatpak_uninstall(place, "org.example.App", false);
            let wipe = flatpak_uninstall(place, "org.example.App", true);
            assert!(!keep.iter().any(|w| w == "--delete-data"), "{keep:?}");
            assert_eq!(wipe.iter().filter(|w| *w == "--delete-data").count(), 1, "{wipe:?}");
            let without: Vec<&String> = wipe.iter().filter(|w| *w != "--delete-data").collect();
            assert_eq!(without, keep.iter().collect::<Vec<_>>());
            assert_eq!(keep.last().unwrap(), "org.example.App");
            assert_eq!(keep[keep.len() - 2], "--");
            assert_eq!(place.is_some(), keep.iter().any(|w| Some(w.as_str()) == place));
        }
    }

    #[test]
    fn a_flatpak_is_found_however_its_launcher_came() {
        let mut look = Fake::default();
        look.dirs.insert(format!("/home/john/.local/share/flatpak/app/{JD}").into());
        look.dirs.insert("/var/lib/flatpak/app/org.example.Sys".into());
        let run = format!("/usr/bin/flatpak run --branch=stable --arch x86_64 --command=JDownloader {JD} @@u %U @@");

        let mut a = app(&format!("/home/john/.local/share/flatpak/exports/share/applications/{JD}.desktop"), &run);
        a.flatpak = Some(JD.into());
        assert_eq!(at_all(&a, &look), flat(Some("--user"), JD, None));

        let mut s = app("/var/lib/flatpak/exports/share/applications/org.example.Sys.desktop", "flatpak run org.example.Sys");
        s.flatpak = Some("org.example.Sys".into());
        assert_eq!(at_all(&s, &look), flat(Some("--system"), "org.example.Sys", None));

        let copy = app("/home/john/.local/share/applications/jdownloader.desktop", &run);
        assert_eq!(at_all(&copy, &look), flat(Some("--user"), JD, Some("/home/john/.local/share/applications/jdownloader.desktop")));

        let made = app("/home/john/.local/share/applications/JDownloader 2.desktop",
                       &format!("/home/john/.var/app/{JD}/data/jd2/JDownloader2 %U"));
        assert_eq!(at_all(&made, &look), flat(Some("--user"), JD, Some("/home/john/.local/share/applications/JDownloader 2.desktop")));

        let bare = app("/var/lib/flatpak/exports/share/applications/org.example.Sys.desktop", "org.example.Sys");
        assert_eq!(at_all(&bare, &look), flat(Some("--system"), "org.example.Sys", None));

        let gone = app("/usr/share/applications/x.desktop", "flatpak run org.example.Gone");
        assert_eq!(at_all(&gone, &look), flat(None, "org.example.Gone", None));
    }

    #[test]
    fn appimages_in_trindles_folder_go_and_others_leave_the_ring() {
        let look = Fake::default();
        let file = PathBuf::from("/home/john/.appimage/Obsidian-1.6.7.AppImage");
        let mut a = app("/home/john/.local/share/applications/trindle-appimage-obsidian.desktop", &format!("\"{}\" %U", file.display()));
        a.appimage = Some(format!("{}|1", file.display()));
        assert_eq!(how(&a, &look, std::slice::from_ref(&file)), Some(How::Image(file.clone())));

        assert_eq!(how(&a, &look, &[]), None);
        a.appimage = None;
        assert_eq!(how(&a, &look, std::slice::from_ref(&file)), Some(How::Image(file)), "by its command");

        for dir in ["Applications", "AppImages", "Downloads"] {
            let other = app("/home/john/.local/share/applications/x.desktop", &format!("/home/john/{dir}/x.AppImage"));
            assert!(at_all(&other, &look).only_hides(), "{dir}");
        }

        let pinned = crate::common::favourites::path_entry(Path::new("/home/john/Documents"), None);
        assert_eq!(how(&pinned, &look, &[]), None);
    }

    #[test]
    fn a_deb_in_its_own_folder_is_removed_like_the_store_does() {
        let mut look = Fake::default();
        look.boxed.insert("foot".into());
        let a = app("/home/john/.local/share/applications/foot.desktop", "trindle-boxed foot foot");
        assert_eq!(at_all(&a, &look), How::Boxed("foot".into()));

        look.boxed.clear();
        assert!(at_all(&a, &look).only_hides());
    }

    #[test]
    fn system_packages_are_removed_unless_trindle_or_the_computer_needs_them() {
        let mut look = Fake::default();
        let own = |look: &mut Fake, path: &str, pkg: &str| { look.owners.insert(path.into(), pkg.into()); };
        own(&mut look, "/usr/share/applications/gimp.desktop", "gimp");
        own(&mut look, "/usr/share/applications/trindle-settings.desktop", "trindle");
        own(&mut look, "/usr/share/applications/trindle-icons.desktop", "trindle-icons");
        own(&mut look, "/usr/share/applications/xwayland.desktop", "xwayland");
        own(&mut look, "/usr/share/applications/htop.desktop", "htop");
        own(&mut look, "/usr/share/applications/debian-xterm.desktop", "xterm:amd64");
        look.facts.insert("htop".into(), Facts { priority: "important".into(), ..Facts::default() });
        look.facts.insert("xterm".into(), Facts { essential: true, ..Facts::default() });
        let at = |p: &str| at_all(&app(p, "x"), &look);
        assert_eq!(at("/usr/share/applications/gimp.desktop"), How::Package("gimp".into()));
        assert_eq!(at("/usr/share/applications/trindle-settings.desktop"), How::Refused(trindle()));
        assert_eq!(at("/usr/share/applications/trindle-icons.desktop"), How::Refused(trindle()));
        assert_eq!(at("/usr/share/applications/xwayland.desktop"), How::Refused(trindle()), "in packages.txt");
        assert_eq!(at("/usr/share/applications/htop.desktop"), How::Refused(needed()));
        assert_eq!(at("/usr/share/applications/debian-xterm.desktop"), How::Refused(needed()));

        look.files.insert("/usr/share/applications/gimp.desktop".into());
        look.owners.insert("/usr/share/trindle/applications/gimp.desktop".into(), "trindle".into());
        assert_eq!(at_all(&app("/usr/share/trindle/applications/gimp.desktop", "gimp"), &look), How::Package("gimp".into()));

        assert!(at_all(&app("/usr/local/share/applications/thing.desktop", "x"), &look).only_hides());
        assert!(at_all(&app("/usr/share/applications/made-by-hand.desktop", "x"), &look).only_hides());
    }

    #[test]
    fn dpkg_and_apt_are_read() {
        assert_eq!(owner_from("gimp: /usr/share/applications/gimp.desktop\n"), Some("gimp".into()));
        assert_eq!(owner_from("xterm:amd64, other: /x\n"), Some("xterm".into()));
        assert_eq!(owner_from("diversion by foo from: /x\ndiversion by foo to: /y\n"), None);
        let sim = "NOTE: simulating\nPurg gimp [3.0.4-1]\nPurg trindle [1.0]\nRemv libgimp3.0-0t64:amd64 [3.0.4-1]\nConf x\n";
        assert_eq!(taken_by(sim), ["gimp", "trindle", "libgimp3.0-0t64"]);
        let none = |_: &str| Facts::default();
        assert_eq!(refusal_of_all("gimp", sim, none), Some(trindle_needs()));
        assert_eq!(refusal_of_all("gimp", "Purg gimp [3]\n", none), None);
        assert_eq!(refusal_of_all("trindle", "Purg trindle [3]\n", none), Some(trindle()));
        let needs = |p: &str| Facts { protected: p == "libc6", ..Facts::default() };
        assert_eq!(refusal_of_all("gimp", "Purg gimp [3]\nPurg libc6 [2]\n", needs), Some(needed()));
    }

    #[test]
    fn hiding_keeps_the_launcher_and_masks_it() {
        let home = std::env::temp_dir().join(format!("trindle-removal-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let sys = home.join("sys/thing.desktop");
        std::fs::create_dir_all(sys.parent().unwrap()).unwrap();
        std::fs::write(&sys, "[Desktop Entry]\nType=Application\nName=Thing\nHidden=false\nExec=thing\n[Desktop Action a]\nHidden=x\n").unwrap();
        hide(&sys, "thing.desktop", &home).unwrap();
        let copy = std::fs::read_to_string(home.join(".local/share/applications/thing.desktop")).unwrap();
        assert_eq!(copy, "[Desktop Entry]\nHidden=true\nType=Application\nName=Thing\nExec=thing\n[Desktop Action a]\nHidden=x\n");
        assert!(std::fs::read_to_string(&sys).unwrap().contains("Hidden=false"), "the original is untouched");
        let mine = home.join(".local/share/applications/sub/mine.desktop");
        std::fs::create_dir_all(mine.parent().unwrap()).unwrap();
        std::fs::write(&mine, "[Desktop Entry]\nName=Mine\n").unwrap();
        hide(&mine, "sub-mine.desktop", &home).unwrap();
        assert_eq!(std::fs::read_to_string(&mine).unwrap(), "[Desktop Entry]\nHidden=true\nName=Mine\n");
        assert!(hide(&sys, "../x.desktop", &home).is_err());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn only_the_data_folder_of_a_plain_name_is_deleted_and_never_through_a_link() {
        let home = Path::new("/home/john");
        for bad in ["", ".", "..", "../x", "a/b", ".hidden", "a b", "a\u{0}b"] {
            assert!(data_dir(home, bad).is_none(), "{bad:?}");
        }
        assert_eq!(data_dir(home, "org.example.Thing"), Some(PathBuf::from("/home/john/.var/app/org.example.Thing")));
        let root = std::env::temp_dir().join(format!("trindle-removal-data-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let app = root.join(".var/app/org.example.Thing");
        std::fs::create_dir_all(app.join("data/deeper")).unwrap();
        std::fs::write(app.join("data/deeper/a"), [0u8; 1000]).unwrap();
        std::fs::write(app.join("b"), [0u8; 24]).unwrap();
        let outside = root.join("outside");
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("keep"), [0u8; 4000]).unwrap();
        std::os::unix::fs::symlink(&outside, app.join("link")).unwrap();
        let size = data_size(&app).unwrap();
        assert!((1024..4000).contains(&size), "a link is not followed: {size}");
        assert_eq!(data_size(&root.join("nothing")).unwrap(), 0);
        let (idle, busy) = (|_: &str| false, |_: &str| true);

        assert_eq!(delete_data_only_unless(&root, "org.example.Thing", busy), Err(t!("common-removal-data-running")));
        assert!(app.exists());
        assert_eq!(delete_data_only_unless(&root, "org.example.Thing", idle), Ok(()));
        assert!(!app.exists() && outside.join("keep").exists(), "its data went, and what the link led to did not");
        assert_eq!(delete_data_only_unless(&root, "org.example.Thing", idle), Ok(()), "no data is no fault");

        std::fs::write(root.join(".var/app/org.example.File"), "x").unwrap();
        assert_eq!(delete_data_only_unless(&root, "org.example.File", idle), Err(t!("common-removal-data-not-folder")));

        let shut = app.join("shut");
        std::fs::create_dir_all(&shut).unwrap();
        std::fs::set_permissions(&shut, std::os::unix::fs::PermissionsExt::from_mode(0o000)).unwrap();
        if std::fs::read_dir(&shut).is_err() {
            assert!(data_size(&app).is_err(), "an unreadable folder is not counted as empty");
        }
        std::fs::set_permissions(&shut, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
        let linked = root.join(".var/app/org.example.Other");
        std::os::unix::fs::symlink(&outside, &linked).unwrap();
        assert!(delete_data_only_unless(&root, "org.example.Other", idle).is_err() && outside.join("keep").exists(), "a link is refused");
        assert!(delete_data_only_unless(&root, "../outside", idle).is_err() && outside.join("keep").exists());
        let _ = std::fs::remove_dir_all(&root);
    }
}
