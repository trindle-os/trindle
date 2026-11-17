//! Installed apps and updates: Flatpaks, AppImages, Trindle and Debian.
//! Trindle is a package like any other, so it and Debian's updates both go
//! through trindle-admin, which asks for the admin password.
//! Firmware has its own file, firmware.rs.

use crate::toolkit::t;
use std::path::PathBuf;

use crate::common::run::{self, capture};

#[derive(Clone, Debug)]
pub enum Source {

    Flatpak { origin: String, installation: String },
    AppImage(PathBuf),

    Deb { launcher: Option<PathBuf>, icon: Option<String> },

    Plugin { icon: Option<String> },
}

pub const KINDS: usize = 5;

#[derive(Clone, Debug)]
pub struct Installed {
    pub id: String,
    pub name: String,
    pub version: String,
    pub size: String,
    pub source: Source,
}

impl Installed {

    pub fn kind(&self) -> usize {
        match &self.source {
            Source::Flatpak { origin, .. } if origin == "flathub" => 0,
            Source::Flatpak { .. } => 1,
            Source::AppImage(_) => 2,
            Source::Deb { .. } => 3,
            Source::Plugin { .. } => 4,
        }
    }

    pub fn key(&self) -> String {
        match &self.source {
            Source::Deb { .. } => format!("deb:{}", self.id),
            Source::Plugin { .. } => format!("plugin:{}", self.id),
            _ => self.id.clone(),
        }
    }

    pub fn of_deb(p: &super::boxed::added::Program) -> Installed {
        Installed {
            id: p.package.clone(), name: p.name.clone(), version: p.version.clone(),
            size: if p.size == 0 { String::new() } else { super::github::size_text(p.size) },
            source: Source::Deb { launcher: p.launcher.clone(), icon: p.icon.clone() },
        }
    }

    pub fn of_plugin(p: &crate::plugins::Plugin, icon: Option<String>, size: u64) -> Installed {
        Installed {
            id: p.id.clone(), name: p.name.clone(), version: p.version.clone(),
            size: if size == 0 { String::new() } else { super::github::size_text(size) },
            source: Source::Plugin { icon },
        }
    }

    pub fn came_from(&self) -> String {
        match &self.source {
            Source::Flatpak { origin, .. } if origin == "flathub" => "Flathub".into(),
            Source::Flatpak { .. } => t!("store-from-flatpak-file").into(),
            Source::AppImage(_) => "AppImage".into(),
            Source::Deb { .. } => t!("store-from-deb").into(),
            Source::Plugin { .. } => t!("store-from-plugin").into(),
        }
    }
}

pub fn installed(done: impl FnOnce(Vec<Installed>) + 'static) {
    capture(&super::progress::english(&["flatpak", "list", "--app", "--columns=application,name,origin,installation,size,version"]), None,
            move |_, out, _| {
        let mut list: Vec<Installed> = out.lines().filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            let id = c.first().filter(|s| !s.is_empty())?.to_string();
            Some(Installed {
                name: c.get(1).filter(|s| !s.is_empty()).unwrap_or(&c[0]).to_string(),
                source: Source::Flatpak {
                    origin: c.get(2).unwrap_or(&"").to_string(),
                    installation: c.get(3).unwrap_or(&"user").to_string(),
                },
                size: c.get(4).unwrap_or(&"").to_string(),
                version: c.get(5).unwrap_or(&"").to_string(),
                id,
            })
        }).collect();
        let store = crate::icons::appimages::store();
        for path in crate::icons::appimages::found().into_iter().filter(|p| p.parent() == Some(store.as_path())) {
            let file = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let size = std::fs::metadata(&path).map(|m| super::github::size_text(m.len())).unwrap_or_default();
            list.push(Installed {
                id: crate::icons::appimages::launcher_for(&path).file_stem().unwrap_or_default().to_string_lossy().to_string(),
                name: crate::icons::appimages::nice_name(&file),
                version: String::new(),
                size,
                source: Source::AppImage(path),
            });
        }
        list.sort_by_key(|i| i.name.to_lowercase());
        done(list);
    });
}

#[derive(Clone, Debug, PartialEq)]
pub enum Removal {

    Flatpak { system: bool },

    AppImage(PathBuf),

    Deb,

    Plugin,
}

impl Installed {

    pub fn removal(&self) -> Removal {
        match &self.source {
            Source::Flatpak { installation, .. } => Removal::Flatpak { system: installation == "system" },
            Source::AppImage(path) => Removal::AppImage(path.clone()),
            Source::Deb { .. } => Removal::Deb,
            Source::Plugin { .. } => Removal::Plugin,
        }
    }
}

pub fn remove(item: &Installed, delete_data: bool, done: impl FnOnce(Result<(), String>) + 'static) {
    let said = |r: std::io::Result<()>| r.map_err(|e| e.to_string());
    let id = item.id.clone();
    let work: Box<dyn FnOnce() -> Result<(), String> + Send> = match item.removal() {
        Removal::Flatpak { system } => {
            let argv = crate::common::removal::flatpak_uninstall(Some(if system { "--system" } else { "--user" }), &item.id, delete_data);
            let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
            capture(&argv, None, move |ok, _, err| {
                done(if ok { Ok(()) } else { Err(err.trim().lines().last().unwrap_or(t!("store-flatpak-refused")).to_string()) });
            });
            return;
        }
        Removal::AppImage(path) => Box::new(move || said(crate::icons::appimages::remove(&path))),
        Removal::Deb => Box::new(move || said(super::boxed::install::remove(&id))),
        Removal::Plugin => Box::new(move || said(crate::plugins::remove(&id))),
    };

    crate::common::run::on_thread(work, done);
}

pub fn trindle_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn shown(v: &str) -> String {
    let v = v.trim_start_matches('v');
    let v = v.split_once(':').map_or(v, |(_, rest)| rest);
    let v = v.split('+').next().unwrap_or(v);
    let cut = v.find(['~', '-']);
    let (main, pre) = cut.map_or((v, ""), |i| (&v[..i], &v[i + 1..]));
    if pre.is_empty() {
        return main.to_string();
    }
    let main = main.strip_suffix(".0").unwrap_or(main);
    let word: String = pre.chars().take_while(char::is_ascii_alphabetic).collect();
    let number: String = pre.chars().filter(char::is_ascii_digit).collect();
    let kind = if word == "rc" { t!("store-version-rc").to_string() } else { word };
    t!("store-version-pre", main = main, kind = kind, number = number).trim_end().to_string()
}

fn debian_form(v: &str) -> String {
    let v = v.trim().trim_start_matches('v');
    match ["~", "-rc", "-beta", "-alpha"].iter().filter_map(|m| v.find(m)).min() {
        Some(i) => {
            let pre: String = v[i..].trim_start_matches(['~', '-']).chars().filter(|c| *c != '.').collect();
            format!("{}~{pre}", &v[..i])
        }
        None => v.to_string(),
    }
}

pub fn newer(latest: &str, current: &str) -> bool {
    super::patch::compare(&debian_form(latest), &debian_form(current)) == std::cmp::Ordering::Greater
}

pub fn trindle_waiting(done: impl FnOnce(Option<String>) + 'static) {
    capture(&super::progress::english(&["apt-cache", "policy", "trindle"]), None, move |ok, out, _| {
        if !ok {
            done(None);
            return;
        }
        let field = |name: &str| out.lines().find_map(|l| l.trim().strip_prefix(name).map(|v| v.trim().to_string()));
        let (Some(installed), Some(candidate)) = (field("Installed:"), field("Candidate:")) else {
            done(None);
            return;
        };
        done(newer(&candidate, &installed).then_some(candidate));
    });
}

pub fn refresh_lists_if_stale(then: impl FnOnce(Result<(), String>) + 'static) {
    let age = std::fs::metadata(LISTS_STAMP).and_then(|m| m.modified()).ok()
        .and_then(|t| t.elapsed().ok());
    if age.is_some_and(|a| a < LISTS_FRESH) {
        return;
    }
    admin_watching(&["apt-refresh"], |_| {}, then);
}

const LISTS_STAMP: &str = "/var/cache/apt/pkgcache.bin";
const LISTS_FRESH: std::time::Duration = std::time::Duration::from_secs(3600);

pub fn debian_upgradable(done: impl FnOnce(Vec<String>) + 'static) {
    capture(&super::progress::english(&["apt", "list", "--upgradable"]), None, move |_, out, _| {

        done(out.lines().filter(|l| l.contains("[upgradable"))
            .filter_map(|l| l.split('/').next().map(String::from))
            .filter(|name| name != "trindle").collect());
    });
}

pub fn debian_watching(step: &'static str, on_line: impl Fn(&str) + 'static,
                       done: impl FnOnce(Result<(), String>) + 'static) {
    admin_watching(&[step], on_line, done);
}

pub fn admin_watching(args: &[&str], on_line: impl Fn(&str) + 'static,
                      done: impl FnOnce(Result<(), String>) + 'static) {
    let helper = run::admin_helper();
    let mut argv = vec!["pkexec", helper.as_str()];
    argv.extend_from_slice(args);

    let tail = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let keep = tail.clone();
    run::stream(&argv, move |line| {
        let mut t = keep.borrow_mut();
        t.push_str(line);
        if !line.ends_with('\n') {
            t.push('\n');
        }

        if t.len() > 4000 {
            let mut cut = t.len() - 2000;
            while !t.is_char_boundary(cut) {
                cut += 1;
            }
            *t = t[cut..].to_string();
        }
        drop(t);
        on_line(line);
    }, move |ok| {
        done(if ok { Ok(()) } else { Err(run::admin_error(&tail.borrow())) });
    });
}

pub fn installed_counts(list: &[Installed]) -> [usize; KINDS] {
    let mut counts = [0; KINDS];
    for item in list {
        counts[item.kind()] += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    #[test]
    fn versions_compare_as_numbers() {
        assert!(super::newer("0.10.0", "0.9.9"));
        assert!(super::newer("v0.5.1", "0.5.0"));
        assert!(!super::newer("0.5.0", "0.5.0"));
        assert!(!super::newer("0.4.9", "0.5.0"));
    }

    #[test]
    fn a_candidate_comes_before_the_release_it_is_a_candidate_for() {
        assert!(super::newer("1.0.0~rc1", "1.0.0~beta2"), "rc 1 follows beta 2");
        assert!(!super::newer("1.0.0~beta2", "1.0.0~rc1"));
        assert!(super::newer("1.0.0-rc.1", "1.0.0-beta.2"));
        assert!(super::newer("1.0.0", "1.0.0~rc1"));
        assert!(super::newer("1.0.0~rc2", "1.0.0~rc1"));
        assert!(super::newer("1.0.0~rc1", "0.15.0"));
        assert!(!super::newer("1.0.0~rc1", "1.0.0"));
        assert!(!super::newer("1.0.0~rc1", "1.0.0~rc1"));

        assert!(super::newer("1.0.0", "1.0.0-rc.1"));
        assert!(super::newer("1.0.0-rc.1", "0.15.0"));

        assert!(super::newer("1.1.0~beta1", "1.0.0~rc2"));
        assert!(super::newer("1.1.0~beta1", "1.0.0~rc2+local4"));
    }

    #[test]
    fn the_beta_after_one_point_oh_still_reaches_those_on_it() {
        assert!(super::newer("1:0.9.0~beta1", "1.0.16"));
        assert!(super::newer("1:0.9.0~beta1", "1.0.16+local13"));
        assert!(super::newer("1:0.9.0~beta2", "1:0.9.0~beta1+local3"));
        assert!(!super::newer("1:0.9.0~beta1", "1:0.9.0~beta1"));
        assert!(!super::newer("1.0.16", "1:0.9.0~beta1"));
    }

    #[test]
    fn this_version_reaches_those_on_the_first_three_betas() {

        let this = format!("1:{}", super::debian_form(env!("CARGO_PKG_VERSION")));
        for old in ["1:0.9.0~beta11319", "1:0.9.0~beta11320+local11"] {
            assert!(super::newer(&this, old), "{this} must come after {old}");
        }
    }

    #[test]
    fn versions_are_shown_plainly() {
        assert_eq!(super::shown("1:0.9.0~beta1+local2"), "0.9 beta 1");
        assert_eq!(super::shown("0.9.0-beta.1"), "0.9 beta 1");
        assert_eq!(super::shown("1.0.16"), "1.0.16");
        assert_eq!(super::shown("1.0.0~rc2"), "1.0 release candidate 2");
    }
}
