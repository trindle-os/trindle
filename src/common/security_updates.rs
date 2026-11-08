//! Security updates that install by themselves, through Debian's
//! unattended-upgrades. On unless the person switches the separate Security updates
//! switch off; the Check for updates switch never touches them. The file
//! below is shipped by tools/system-files.sh: it turns the security fixes
//! on over Trindle's own 20auto-upgrades, which keeps every other update
//! from going in unasked, and narrows Debian's list to its security
//! archive alone.

use std::path::Path;

use super::files::{default_setting, read_env, trindle_conf, update_env};
use super::update_checks::enabled;

pub const FILE: &str = "/etc/apt/apt.conf.d/52trindle-security-updates";
const PROGRAM: &str = "/usr/bin/unattended-upgrade";

pub const KEY: &str = "security_updates";

pub const APT_OFF: &str = "/etc/apt/apt.conf.d/54trindle-no-security-updates";

pub const APT_OFF_TEXT: &str = "APT::Periodic::Unattended-Upgrade \"0\";\n";

pub fn on() -> bool {
    on_with(Path::new(FILE).exists(), Path::new(PROGRAM).exists(), switch_on())
}

pub fn on_with(file: bool, program: bool, switch: bool) -> bool {
    file && program && switch
}

pub fn switch_on() -> bool {
    switch_on_in(&trindle_conf())
}

pub fn switch_on_in(conf: &Path) -> bool {
    enabled(read_env(conf).get(KEY).map_or(default_setting(KEY), String::as_str))
}

pub fn set(on: bool) -> std::io::Result<()> {
    set_in(&trindle_conf(), on)
}

pub fn set_in(conf: &Path, on: bool) -> std::io::Result<()> {
    update_env(conf, &[(KEY, Some(if on { "yes" } else { "no" }))])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_by_default_and_off_with_their_own_switch() {
        let dir = std::env::temp_dir().join(format!("trindle-security-updates-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let conf = dir.join("trindle.conf");
        let _ = std::fs::remove_file(&conf);
        assert!(switch_on_in(&conf), "on by default");
        set_in(&conf, false).unwrap();
        assert!(!switch_on_in(&conf));
        assert_eq!(read_env(&conf).get(KEY).map(String::as_str), Some("no"));
        set_in(&conf, true).unwrap();
        assert!(switch_on_in(&conf));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn on_only_when_set_up_and_switched_on() {
        assert!(on_with(true, true, true));
        assert!(!on_with(true, true, false));
        assert!(!on_with(false, true, true) && !on_with(true, false, true));
    }

    #[test]
    fn check_for_updates_off_leaves_them_on() {
        let dir = std::env::temp_dir().join(format!("trindle-security-checks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let conf = dir.join("trindle.conf");
        let _ = std::fs::remove_file(&conf);
        super::super::update_checks::set_in(&conf, false).unwrap();
        assert!(!super::super::update_checks::on_in(&conf));
        assert!(switch_on_in(&conf));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_marker_changes_only_the_install_by_itself() {
        assert_eq!(APT_OFF_TEXT.lines().count(), 1);
        assert_eq!(APT_OFF_TEXT, "APT::Periodic::Unattended-Upgrade \"0\";\n");
        assert!(APT_OFF > FILE, "it is read after the file that turns them on");
        assert_ne!(APT_OFF, super::super::update_checks::APT_OFF);
        let shipped = include_str!("../../tools/system-files.sh");
        assert!(!shipped.contains("54trindle-no-security-updates"), "nothing ships the marker");
    }
}
