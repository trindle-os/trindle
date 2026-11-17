//! Trindle's own settings and desktop files. Everything else about files
//! and folders (the person's folders, the rules for writing and for opening
//! what was named, `KEY=value` files and where Trindle's settings folder
//! is) is the toolkit's, in its common/files.rs, and is used from here
//! under the same names. The compositor builds this file in too.

pub use trindle_toolkit::common::files::*;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use super::pointer;

pub const SYSTEM_DESKTOP: &str = "/usr/share/trindle/desktop";

pub fn system_desktop(name: &str) -> PathBuf {
    let installed = Path::new(SYSTEM_DESKTOP).join(name);

    #[cfg(any(test, debug_assertions))]
    if !installed.is_file() {
        return PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default()).join("config/desktop").join(name);
    }
    installed
}

pub fn user_environment() -> PathBuf {
    trindle_config().join("environment")
}

pub fn report_offer_for(version: &str) -> &'static str {
    if version.contains('-') { "on" } else { "off" }
}

pub fn default_setting(key: &str) -> &'static str {
    match key {

        "restart-parts-keys" => "Ctrl+Alt+Escape",
        "restart-all-keys" => "Ctrl+Alt+Shift+Escape",

        "screenshot-keys" => "Ctrl+Alt+Shift+S",
        "shortcut-list-keys" => "Super+K",
        "report-keys" => "Ctrl+Alt+R",
        "report_offer" => report_offer_for(env!("CARGO_PKG_VERSION")),
        "clock_format" => "%H:%M",
        "wallpaper_color" => "#09090b",
        "dim_after" => "5",
        "lock_after" => "10",
        "lock_on_suspend" => "yes",

        "ring_window_names" => "yes",

        "check_updates_by_itself" => "yes",

        "security_updates" => "yes",
        "lock" => "off",
        "blank_after" => "10",
        "suspend_after" => "30",

        "dim_after_battery" => "2",
        "screensaver_after" => "5",
        "lid_close" => "suspend",
        "lid_close_battery" => "suspend",
        "screensaver_after_battery" => "3",
        "blank_after_battery" => "5",
        "suspend_after_battery" => "15",
        "wallpaper_every" => "300",
        "wallpaper_order" => "shuffle",
        "wallpaper_fit" => "fill",

        "icon_source" => "Trindle-Circle",

        "older_programs" => "yes",

        "outside_network" | "outside_files" | "downloads_boxed" => "yes",

        pointer::SPEED => "0",

        pointer::SCROLL => "4",
        pointer::TOUCHPAD_SCROLL => "1.5",
        pointer::MOUSE_NATURAL => "no",
        pointer::LEFT_HANDED => "no",
        pointer::TAP => "yes",
        pointer::TOUCHPAD_NATURAL => "yes",
        pointer::QUIET_TYPING => "yes",
        _ => "",
    }
}

pub fn state_home() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME").map(PathBuf::from).filter(|p| p.is_absolute())
        .unwrap_or_else(|| home().join(".local/state"))
}

pub fn setting(key: &str) -> String {
    static KEPT: Mutex<Option<ReadConf>> = Mutex::new(None);
    let path = trindle_conf();
    let found = match KEPT.lock() {
        Ok(mut kept) => kept.get_or_insert_with(ReadConf::default).keys(&path, SystemTime::now()).get(key).cloned(),
        Err(_) => read_env(&path).get(key).cloned(),
    };
    found.unwrap_or_else(|| default_setting(key).to_string())
}

const SETTLED: Duration = Duration::from_secs(2);

#[derive(Default)]
struct ReadConf {
    path: PathBuf,
    stamp: Option<(u64, u64, u64, i64, i64, i64, i64)>,

    sure: bool,
    keys: BTreeMap<String, String>,
    #[cfg(test)]
    reads: u32,
}

impl ReadConf {
    fn keys(&mut self, path: &Path, now: SystemTime) -> &BTreeMap<String, String> {
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::metadata(path).ok();
        let stamp = meta.as_ref().map(|m| (m.dev(), m.ino(), m.size(), m.mtime(), m.mtime_nsec(), m.ctime(), m.ctime_nsec()));
        if !self.sure || self.path != path || self.stamp != stamp {
            self.keys = read_env(path);
            #[cfg(test)]
            {
                self.reads += 1;
            }
            self.path = path.to_path_buf();
            self.stamp = stamp;
            self.sure = meta.is_none_or(|m| {
                let changed = |s: i64, ns: i64| u64::try_from(s).ok().map(|s| SystemTime::UNIX_EPOCH + Duration::new(s, u32::try_from(ns).unwrap_or(0)));
                [changed(m.mtime(), m.mtime_nsec()), changed(m.ctime(), m.ctime_nsec())]
                    .iter()
                    .all(|at| at.and_then(|at| now.duration_since(at).ok()).is_some_and(|age| age >= SETTLED))
            });
        }
        &self.keys
    }
}

pub fn set_setting(key: &str, value: &str) -> std::io::Result<()> {
    update_env(&trindle_conf(), &[(key, Some(value))])
}

#[cfg(test)]
mod read_conf_tests {
    use super::*;

    fn conf(name: &str, text: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-read-conf-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("trindle.conf");
        std::fs::write(&path, text).unwrap();
        path
    }

    #[test]
    fn trindle_conf_is_read_again_only_when_it_has_changed() {
        let path = conf("settled", "accent=#112233\n");
        let later = SystemTime::now() + Duration::from_secs(10);
        let mut kept = ReadConf::default();
        for _ in 0..50 {
            assert_eq!(kept.keys(&path, later).get("accent").map(String::as_str), Some("#112233"));
        }
        assert_eq!(kept.reads, 1, "fifty looks at a settled file, one read");
        std::fs::write(&path, "accent=#445566 and more\n").unwrap();
        assert_eq!(kept.keys(&path, later + SETTLED).get("accent").map(String::as_str), Some("#445566 and more"));
        assert_eq!(kept.reads, 2);
        let other = conf("other", "accent=#778899\n");
        assert_eq!(kept.keys(&other, later).get("accent").map(String::as_str), Some("#778899"), "another file is read for itself");
        std::fs::remove_file(&path).unwrap();
        assert!(kept.keys(&path, later).is_empty());
        assert!(kept.keys(&path, later).is_empty());
        assert_eq!(kept.reads, 4, "a missing file is not looked for again until it is there");
        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
        std::fs::remove_dir_all(other.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_file_changed_just_now_is_read_every_time() {
        let path = conf("racy", "accent=#111111\n");
        let mut kept = ReadConf::default();
        assert_eq!(kept.keys(&path, SystemTime::now()).get("accent").map(String::as_str), Some("#111111"));
        std::fs::write(&path, "accent=#222222\n").unwrap();
        assert_eq!(kept.keys(&path, SystemTime::now()).get("accent").map(String::as_str), Some("#222222"));
        assert_eq!(kept.reads, 2);
        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
}
