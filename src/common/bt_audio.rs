//! Bluetooth sound after the adapter has been switched off and on.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub const SETTLE: Duration = Duration::from_secs(3);

pub const HELP: &str = "bluetooth-codec";

const SERVICE: &str = "wireplumber.service";
const NOTE: &str = "trindle-bluetooth-sound-note";

pub trait Services {
    fn running(&self) -> bool;
    fn restart(&self) -> Result<(), String>;
}

pub struct UserSystemd;

impl Services for UserSystemd {
    fn running(&self) -> bool {
        std::process::Command::new("systemctl").args(["--user", "--quiet", "is-active", SERVICE])
            .stdin(std::process::Stdio::null()).status().is_ok_and(|s| s.success())
    }

    fn restart(&self) -> Result<(), String> {
        let out = std::process::Command::new("systemctl").args(["--user", "restart", SERVICE])
            .stdin(std::process::Stdio::null()).output().map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok(());
        }
        let said = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if said.is_empty() { format!("exit {}", out.status) } else { said })
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    NotRunning,
    Restarted,
    Failed(String),
}

pub fn revive(services: &dyn Services) -> Outcome {
    if !services.running() {
        return Outcome::NotRunning;
    }
    match services.restart() {
        Ok(()) => Outcome::Restarted,
        Err(why) => Outcome::Failed(why),
    }
}

#[derive(Default)]
pub struct Watch {
    on: HashMap<String, bool>,
    due: Option<Instant>,
}

impl Watch {
    pub fn powered(&mut self, adapter: &str, on: bool, now: Instant) -> bool {
        let before = self.on.insert(adapter.to_string(), on);
        if !on {
            if !self.on.values().any(|v| *v) {
                self.due = None;
            }
            return false;
        }
        if before == Some(true) {
            return false;
        }
        self.due = Some(now + SETTLE);
        true
    }

    pub fn due(&mut self, now: Instant) -> bool {
        match self.due {
            Some(at) if now >= at => {
                self.due = None;
                true
            }
            _ => false,
        }
    }
}

fn note_file() -> Option<PathBuf> {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").filter(|d| !d.is_empty())?;
    Some(PathBuf::from(dir).join(NOTE))
}

pub fn keep_note(outcome: &Outcome) {
    if let Some(file) = note_file() {
        keep_in(&file, outcome);
    }
}

fn keep_in(file: &std::path::Path, outcome: &Outcome) {
    match outcome {
        Outcome::Failed(why) => {
            let _ = std::fs::write(file, why);
        }
        Outcome::Restarted => {
            let _ = std::fs::remove_file(file);
        }
        Outcome::NotRunning => {}
    }
}

pub fn kept_note() -> Option<String> {
    std::fs::read_to_string(note_file()?).ok()
}

#[cfg(test)]
fn kept_in(file: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(file).ok()
}

pub fn words(why: &str) -> String {
    crate::toolkit::t!("settings-bluetooth-sound-failed", why = why).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct Made {
        running: bool,
        fails: Option<&'static str>,
        restarts: RefCell<u32>,
    }

    impl Services for Made {
        fn running(&self) -> bool {
            self.running
        }
        fn restart(&self) -> Result<(), String> {
            *self.restarts.borrow_mut() += 1;
            self.fails.map_or(Ok(()), |w| Err(w.to_string()))
        }
    }

    fn made(running: bool, fails: Option<&'static str>) -> Made {
        Made { running, fails, restarts: RefCell::new(0) }
    }

    fn play(stream: &[(u64, &str, bool)], end: u64, services: &Made) -> Vec<Outcome> {
        let start = Instant::now();
        let at = |s: u64| start + Duration::from_secs(s);
        let mut watch = Watch::default();
        let mut outcomes = Vec::new();
        let mut now = 0;
        for &(s, adapter, on) in stream {
            while now < s {
                now += 1;
                if watch.due(at(now)) {
                    outcomes.push(revive(services));
                }
            }
            watch.powered(adapter, on, at(s));
        }
        while now < end {
            now += 1;
            if watch.due(at(now)) {
                outcomes.push(revive(services));
            }
        }
        outcomes
    }

    #[test]
    fn restarts_once_after_power_up() {
        let m = made(true, None);
        let got = play(&[(1, "hci0", false), (5, "hci0", true)], 20, &m);
        assert_eq!(got, [Outcome::Restarted]);
        assert_eq!(*m.restarts.borrow(), 1);
    }

    #[test]
    fn power_down_alone_does_nothing() {
        let m = made(true, None);
        assert!(play(&[(1, "hci0", false)], 20, &m).is_empty());
        assert_eq!(*m.restarts.borrow(), 0);
    }

    #[test]
    fn several_power_events_in_a_row_make_one_restart() {
        let m = made(true, None);
        let stream = [(1, "hci0", false), (2, "hci0", true), (3, "hci0", false), (4, "hci0", true), (5, "hci0", false), (6, "hci0", true)];
        assert_eq!(play(&stream, 30, &m), [Outcome::Restarted]);
        assert_eq!(*m.restarts.borrow(), 1);
    }

    #[test]
    fn off_again_before_the_wait_ends_restarts_nothing() {
        let m = made(true, None);
        assert!(play(&[(1, "hci0", false), (2, "hci0", true), (3, "hci0", false)], 30, &m).is_empty());
    }

    #[test]
    fn two_cycles_far_apart_make_two_restarts() {
        let m = made(true, None);
        let stream = [(1, "hci0", false), (2, "hci0", true), (30, "hci0", false), (31, "hci0", true)];
        assert_eq!(play(&stream, 60, &m).len(), 2);
    }

    #[test]
    fn a_repeat_of_on_is_not_a_power_up() {
        let mut w = Watch::default();
        let now = Instant::now();
        assert!(w.powered("hci0", true, now));
        assert!(!w.powered("hci0", true, now));
    }

    #[test]
    fn nothing_when_wireplumber_is_not_running() {
        let m = made(false, None);
        assert_eq!(play(&[(1, "hci0", false), (2, "hci0", true)], 20, &m), [Outcome::NotRunning]);
        assert_eq!(*m.restarts.borrow(), 0);
    }

    #[test]
    fn a_failed_restart_keeps_its_reason() {
        let m = made(true, Some("Failed to restart wireplumber.service: Unit not found."));
        let got = play(&[(1, "hci0", false), (2, "hci0", true)], 20, &m);
        assert_eq!(got, [Outcome::Failed("Failed to restart wireplumber.service: Unit not found.".into())]);
    }

    #[test]
    fn the_note_is_kept_and_cleared() {
        let dir = std::env::temp_dir().join(format!("trindle-btnote-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join(NOTE);
        assert_eq!(kept_in(&file), None);
        keep_in(&file, &Outcome::NotRunning);
        assert_eq!(kept_in(&file), None);
        keep_in(&file, &Outcome::Failed("no unit".into()));
        assert_eq!(kept_in(&file).as_deref(), Some("no unit"));
        keep_in(&file, &Outcome::NotRunning);
        assert_eq!(kept_in(&file).as_deref(), Some("no unit"));
        keep_in(&file, &Outcome::Restarted);
        assert_eq!(kept_in(&file), None);
        let _ = std::fs::remove_dir_all(dir);
    }
}
