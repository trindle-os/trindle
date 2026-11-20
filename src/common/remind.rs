//! "Remind me later" for a restart that waits: when the reminder is due and which start it was set in, kept in trindle.conf as "due:boot id". A restart ends it, as the boot id changes.

use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use super::files::{set_setting, setting};

pub const KEY: &str = "restart_remind";

pub const HOURS: [u64; 5] = [1, 3, 6, 12, 24];

const BOOT_ID: &str = "/proc/sys/kernel/random/boot_id";
const LONGEST_ID: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reminder {

    None,
    Waiting,
    Due,
}

pub fn reminder(saved: &str, now: u64, boot: &str) -> Reminder {
    let Some((due, id)) = saved.trim().split_once(':') else { return Reminder::None };
    let Ok(due) = due.parse::<u64>() else { return Reminder::None };
    let furthest = now.saturating_add(HOURS[HOURS.len() - 1] * 3600);
    if id.is_empty() || id.len() > LONGEST_ID || id != boot || due > furthest {
        Reminder::None
    } else if now >= due {
        Reminder::Due
    } else {
        Reminder::Waiting
    }
}

pub fn value(hours: u64, now: u64, boot: &str) -> Option<String> {
    (HOURS.contains(&hours) && !boot.is_empty() && boot.len() <= LONGEST_ID).then(|| format!("{}:{boot}", now.saturating_add(hours * 3600)))
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn boot() -> &'static str {
    static ID: OnceLock<String> = OnceLock::new();
    ID.get_or_init(|| std::fs::read_to_string(BOOT_ID).map(|s| s.trim().to_string()).unwrap_or_default())
}

pub fn state() -> Reminder {
    if boot().is_empty() { Reminder::None } else { reminder(&setting(KEY), now(), boot()) }
}

pub fn due() -> bool {
    state() == Reminder::Due
}

pub fn set(at: usize) -> std::io::Result<()> {
    let hours = HOURS.get(at).copied().unwrap_or(0);
    let value = value(hours, now(), boot()).ok_or_else(|| std::io::Error::other("no such reminder"))?;
    set_setting(KEY, &value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOT: &str = "3f2a1c9e-0b7d-4d51-9c1e-5a6b7c8d9e0f";

    #[test]
    fn a_reminder_is_waiting_until_its_time_and_then_due() {
        let at = |due: u64, now: u64| reminder(&format!("{due}:{BOOT}"), now, BOOT);
        assert_eq!(at(5000, 4999), Reminder::Waiting);
        assert_eq!(at(5000, 5000), Reminder::Due);
        assert_eq!(at(5000, 9000), Reminder::Due);
    }

    #[test]
    fn another_start_a_damaged_or_far_off_value_is_no_reminder() {
        let now = 1_000_000;
        let other = "00000000-0000-0000-0000-000000000000";
        let day = HOURS[4] * 3600;
        for saved in [String::new(), "x".into(), ":".into(), format!(":{BOOT}"), format!("abc:{BOOT}"), format!("-5:{BOOT}"), format!("{now}"),
                      format!("{now}:{other}"), format!("{}:{BOOT}", now + day + 1), format!("{}:{BOOT}", u64::MAX), format!("99999999999999999999999:{BOOT}"),
                      format!("{now}:{}", "a".repeat(LONGEST_ID + 1))] {
            assert_eq!(reminder(&saved, now, BOOT), Reminder::None, "{saved}");
        }
        assert_eq!(reminder(&format!("{}:{BOOT}", now + day), now, BOOT), Reminder::Waiting);
        assert_eq!(reminder(&format!("{now}:{BOOT}"), u64::MAX, BOOT), Reminder::Due);
    }

    #[test]
    fn only_the_five_times_can_be_saved_and_each_reads_back_as_waiting() {
        assert_eq!(HOURS, [1, 3, 6, 12, 24]);
        for h in HOURS {
            let v = value(h, 1000, BOOT).expect("offered");
            assert_eq!(reminder(&v, 1000, BOOT), Reminder::Waiting);
            assert_eq!(reminder(&v, 1000 + h * 3600, BOOT), Reminder::Due);
        }
        for h in [0, 2, 25, u64::MAX] {
            assert_eq!(value(h, 1000, BOOT), None);
        }
        assert_eq!(value(1, 1000, ""), None);
    }

    #[test]
    fn the_saved_value_survives_the_settings_file() {
        let dir = std::env::temp_dir().join(format!("trindle-remind-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let conf = dir.join("trindle.conf");
        let v = value(6, 4242, BOOT).unwrap();
        super::super::files::update_env(&conf, &[(KEY, Some(&v))]).unwrap();
        assert_eq!(super::super::files::read_env(&conf).get(KEY).map(String::as_str), Some(v.as_str()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
