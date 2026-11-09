//! The one switch "Check for updates", kept in trindle.conf as "yes" or "no".
//! Off, nothing contacts an update or catalogue server unless the person asks: the ring, Apps, the plugin
//! and app catalogues, and the DNS lists. Debian's daily list fetch and security fixes never stop
//! (common::security_updates); the system-wide services learn of the switch from APT_OFF, which
//! `trindle-admin update-checks off` writes.

use std::path::Path;

use super::files::{default_setting, read_env, trindle_conf, update_env};

pub const KEY: &str = "check_updates_by_itself";

pub fn enabled(value: &str) -> bool {
    value.trim() != "no"
}

pub fn on() -> bool {
    on_in(&trindle_conf())
}

pub fn on_in(conf: &Path) -> bool {
    enabled(read_env(conf).get(KEY).map_or(default_setting(KEY), String::as_str))
}

pub fn set(on: bool) -> std::io::Result<()> {
    set_in(&trindle_conf(), on)
}

pub fn set_in(conf: &Path, on: bool) -> std::io::Result<()> {
    update_env(conf, &[(KEY, Some(if on { "yes" } else { "no" }))])
}

pub const APT_OFF: &str = "/etc/apt/apt.conf.d/53trindle-no-update-checks";

pub const APT_OFF_TEXT: &str = "// Check for updates is off in Trindle. Debian's security updates are not affected.\n";

pub fn on_system_wide() -> bool {
    !Path::new(APT_OFF).exists()
}

pub fn may_fetch_catalogue(have_copy: bool, fresh: bool, switch_on: bool) -> bool {
    !have_copy || (!fresh && switch_on)
}

pub fn may_ask_network(person_asked: bool, switch_on: bool) -> bool {
    person_asked || switch_on
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_unless_saved_as_no() {
        assert!(enabled("yes") && enabled("") && enabled("anything"));
        assert!(!enabled("no") && !enabled(" no "));
    }

    #[test]
    fn the_switch_decides_only_what_is_not_asked_for() {
        assert!(may_ask_network(false, true));
        assert!(!may_ask_network(false, false));
        assert!(may_ask_network(true, false), "a press always works");
        assert!(may_ask_network(true, true));
    }

    #[test]
    fn a_catalogue_copy_of_any_age_is_read_with_the_switch_off() {
        assert!(!may_fetch_catalogue(true, false, false));
        assert!(may_fetch_catalogue(true, false, true));
        assert!(!may_fetch_catalogue(true, true, true));
        assert!(may_fetch_catalogue(false, false, false), "no copy at all: the shelves need one");
    }

    #[test]
    fn off_never_touches_debians_daily_jobs_or_the_security_fixes() {
        assert!(APT_OFF_TEXT.lines().all(|l| l.starts_with("//")), "the marker sets no apt option");
        assert!(!APT_OFF_TEXT.contains("Periodic") && !APT_OFF_TEXT.contains("Unattended"));
        assert!(super::super::security_updates::on_with(true, true, true));
        assert!(!super::super::security_updates::on_with(false, true, true));
        let shipped = include_str!("../../tools/system-files.sh");
        assert!(shipped.contains("APT::Periodic::Unattended-Upgrade \"1\""), "the security fixes are shipped on");
        assert!(!shipped.contains("53trindle-no-update-checks"), "nothing ships or sets the marker over them");
    }

    #[test]
    fn with_the_switch_off_nothing_automatic_asks() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("system/systemd");
        let mut timers: Vec<String> = std::fs::read_dir(dir).unwrap().flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| n.ends_with(".timer")).collect();
        timers.sort();

        assert_eq!(timers, ["trindle-boxed-patch.timer", "trindle-home-snapshots.timer", "trindle-read-only.timer"]);
        let patch = include_str!("../store/patch.rs").split_once("pub fn main()").unwrap().1;
        assert!(!patch[..300].contains("update_checks::on()"), "the box security fixes do not follow the switch");
        let cat = include_str!("../store/catalogue.rs").split_once("pub fn load(").unwrap().1;
        assert!(cat[..900].contains("may_fetch_catalogue"));
        let hub = include_str!("../store/hub.rs").split_once("pub fn fetch(").unwrap().1;
        assert!(hub[..600].contains("may_fetch_catalogue"));

        assert!(!may_ask_network(false, false));
        let ring = include_str!("../ring/updates.rs");
        assert!(ring.contains("may_ask_network(false, !self.off)"));
        let store = include_str!("../store/reading.rs");
        assert!(store.contains("!self.lists_fetched && self.online && !self.upd.off"));
        assert!(store.contains("!self.firmware_fetched && self.online && !self.upd.off"));
        let dns = include_str!("../dns/mod.rs").split_once("fn keep_lists(").unwrap().1;
        assert!(dns[..600].contains("may_fetch_catalogue(") && dns[..600].contains("on_system_wide()"), "the DNS lists follow the switch");
    }

    #[test]
    fn saved_and_read_back_from_the_file_and_on_when_nothing_is_saved() {
        let dir = std::env::temp_dir().join(format!("trindle-update-checks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let conf = dir.join("trindle.conf");
        let _ = std::fs::remove_file(&conf);
        assert!(on_in(&conf), "on by default");
        set_in(&conf, false).unwrap();
        assert!(!on_in(&conf));
        assert_eq!(read_env(&conf).get(KEY).map(String::as_str), Some("no"));
        set_in(&conf, true).unwrap();
        assert!(on_in(&conf));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
