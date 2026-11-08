//! Steam, restarted after an update to it or what it runs on, once no game runs.

use std::path::Path;
use std::time::Duration;

use crate::common::run;
use crate::common::updating::{Lock, Who};

const STEAM: &str = "com.valvesoftware.Steam";
const LOOK_AGAIN: Duration = Duration::from_secs(60);
const CLOSING: Duration = Duration::from_secs(60);

pub fn touched(ids: &[String]) -> bool {
    ids.iter().any(|id| id == STEAM || id.starts_with("com.valvesoftware.Steam.") || id.starts_with("org.freedesktop.Platform"))
}

pub fn running() -> bool {
    run::text(&["flatpak", "ps", "--columns=application"]).lines().any(|l| l.trim() == STEAM)
}

fn game_running(proc: &Path) -> bool {
    let Ok(all) = std::fs::read_dir(proc) else { return false };
    all.flatten().filter(|e| e.file_name().to_string_lossy().bytes().all(|b| b.is_ascii_digit()))
        .any(|e| std::fs::read(e.path().join("cmdline")).is_ok_and(|c| c.split(|&b| b == 0).any(|a| a == b"SteamLaunch")))
}

pub fn renew_when_free() -> i32 {
    let Ok(_one) = Lock::take(&crate::common::files::scratch().join("trindle-steam-renew"), Who::Apps) else { return 0 };
    renew(&Real)
}

trait Machine {
    fn running(&self) -> bool;
    fn game_running(&self) -> bool;
    fn pause(&self, how_long: Duration);
    fn ask_to_close(&self);
    fn start(&self) -> bool;
}

struct Real;

impl Machine for Real {
    fn running(&self) -> bool {
        running()
    }
    fn game_running(&self) -> bool {
        game_running(Path::new("/proc"))
    }
    fn pause(&self, how_long: Duration) {
        std::thread::sleep(how_long);
    }
    fn ask_to_close(&self) {
        run::text(&["flatpak", "run", STEAM, "-shutdown"]);
    }
    fn start(&self) -> bool {
        let argv = crate::common::apps::with_own_x11(["flatpak", "run", STEAM, "-silent"].map(String::from).to_vec());
        run::spawn(&argv.iter().map(String::as_str).collect::<Vec<_>>())
    }
}

fn renew(m: &impl Machine) -> i32 {
    while m.running() && m.game_running() {
        m.pause(LOOK_AGAIN);
    }

    if !m.running() {
        return 0;
    }

    m.ask_to_close();
    let mut waited = Duration::ZERO;
    while m.running() {
        if waited > CLOSING {
            return 1;
        }
        m.pause(Duration::from_secs(1));
        waited += Duration::from_secs(1);
    }
    i32::from(!m.start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_and_what_it_runs_on_touch_it_and_nothing_else_does() {
        let ids = |l: &[&str]| l.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(touched(&ids(&["org.gnome.Calculator", STEAM])));
        assert!(touched(&ids(&["org.freedesktop.Platform.GL.default"])));
        assert!(touched(&ids(&["com.valvesoftware.Steam.CompatibilityTool.Proton-GE"])));
        assert!(!touched(&ids(&["org.gnome.Calculator", "org.gnome.Platform", "com.valvesoftware.SteamLink"])));
    }

    use std::cell::{Cell, RefCell};

    struct Fake {
        up: Cell<bool>,
        game_looks: Cell<u32>,
        close_after: Option<u32>,
        asked_at: Cell<Option<u32>>,
        pauses: Cell<u32>,
        log: RefCell<Vec<&'static str>>,
        person_quits: bool,
    }

    fn fake(game_looks: u32, close_after: Option<u32>) -> Fake {
        Fake { up: Cell::new(true), game_looks: Cell::new(game_looks), close_after, asked_at: Cell::new(None), pauses: Cell::new(0), log: RefCell::new(vec![]), person_quits: false }
    }

    impl Machine for Fake {
        fn running(&self) -> bool {
            if let (Some(at), Some(n)) = (self.asked_at.get(), self.close_after) {
                if self.pauses.get() >= at + n {
                    self.up.set(false);
                }
            }
            self.up.get()
        }
        fn game_running(&self) -> bool {
            let left = self.game_looks.get();
            self.game_looks.set(left.saturating_sub(1));
            left > 0
        }
        fn pause(&self, _: Duration) {
            self.pauses.set(self.pauses.get() + 1);
            if self.person_quits {
                self.up.set(false);
            }
        }
        fn ask_to_close(&self) {
            assert_eq!(self.game_looks.get(), 0, "asked Steam to close with a game still running");
            self.asked_at.set(Some(self.pauses.get()));
            self.log.borrow_mut().push("close");
        }
        fn start(&self) -> bool {
            self.log.borrow_mut().push("start");
            true
        }
    }

    #[test]
    fn steam_waits_for_the_game_then_closes_and_starts_again() {
        let m = fake(3, Some(2));
        assert_eq!(renew(&m), 0);
        assert_eq!(*m.log.borrow(), ["close", "start"]);
        assert_eq!(m.pauses.get(), 3 + 2);
    }

    #[test]
    fn steam_closed_by_the_person_while_waiting_is_left_closed() {
        let mut m = fake(2, Some(1));
        m.person_quits = true;
        assert_eq!(renew(&m), 0);
        assert!(m.log.borrow().is_empty());
    }

    #[test]
    fn steam_that_will_not_close_is_asked_once_never_forced_and_not_started_again() {
        let m = fake(0, None);
        assert_eq!(renew(&m), 1);
        assert_eq!(*m.log.borrow(), ["close"]);
    }

    #[test]
    fn a_game_is_seen_only_by_steams_reaper() {
        let proc = std::env::temp_dir().join(format!("trindle-steam-proc-{}", std::process::id()));
        let put = |pid: &str, cmd: &[u8]| {
            std::fs::create_dir_all(proc.join(pid)).unwrap();
            std::fs::write(proc.join(pid).join("cmdline"), cmd).unwrap();
        };
        put("12", b"/app/bin/steam\0-silent\0");
        put("self", b"SteamLaunch\0");
        assert!(!game_running(&proc));
        put("40", b"/home/john/.steam/ubuntu12_32/reaper\0SteamLaunch\0AppId=289650\0--\0game.exe\0");
        assert!(game_running(&proc));
        let _ = std::fs::remove_dir_all(&proc);
    }
}
