// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;
use std::time::Duration;

use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::input::keyboard::XkbConfig;

use crate::state::{AnvilState, Backend};

fn settings_stamp() -> Vec<Option<std::time::SystemTime>> {
    crate::config::sources().iter().map(|p| std::fs::metadata(p).and_then(|m| m.modified()).ok()).collect()
}

#[derive(Clone, Debug, Default)]
pub struct Launch {

    pub startup: Option<String>,

    pub greeter: bool,

    pub ring: Option<std::path::PathBuf>,
}

pub const AUTOSTART: &str = "trindle-autostart";

pub static LAUNCH: OnceLock<Launch> = OnceLock::new();

pub fn launch() -> Launch {
    LAUNCH.get().cloned().unwrap_or_default()
}

static FILES: OnceLock<libc::rlimit> = OnceLock::new();

pub fn more_files() {
    let mut limit = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    // SAFETY: getrlimit only writes into the rlimit it is handed.
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return;
    }
    let _ = FILES.set(limit);
    let more = libc::rlimit { rlim_cur: limit.rlim_max, rlim_max: limit.rlim_max };
    // SAFETY: setrlimit only reads the rlimit it is handed.
    unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &more) };
}

pub fn usual_files(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    let Some(&limit) = FILES.get() else { return };
    // SAFETY: between fork and exec the hook only calls setrlimit, which is

    unsafe {
        cmd.pre_exec(move || {
            libc::setrlimit(libc::RLIMIT_NOFILE, &limit);
            Ok(())
        });
    }
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn session_ready(&mut self) {
        if let Some(name) = &self.socket_name {
            std::env::set_var("WAYLAND_DISPLAY", name);
        }
        if let Some(x) = self.xdisplay {
            std::env::set_var("DISPLAY", format!(":{x}"));
        }

        std::env::set_var("TRINDLE_COMPOSE_PID", std::process::id().to_string());
        let Some(cmd) = self.startup.take() else { return };
        let mut child = Command::new("/bin/sh");
        child.arg("-c").arg(&cmd).stdin(Stdio::null());
        usual_files(&mut child);

        let door = self.keep_ring().and_then(|told| crate::own_ring::door(&mut child, told));

        if door.is_none() && cmd == AUTOSTART {
            tracing::error!("the ring cannot be started and kept here: the start-up starts one by name, which says so");
            child.env(crate::own_ring::DOOR, crate::own_ring::NO_DOOR);
        }
        let child = child.spawn();
        if let Some(door) = door {
            door.opened();
        }
        match child {
            Ok(child) if launch().greeter => self.end_with(child),
            Ok(child) => let_go(child),
            Err(e) => tracing::error!(cmd, "could not start: {e}"),
        }
    }

    fn end_with(&mut self, child: Child) {
        let mut child = Some(child);
        let _ = self.handle.insert_source(Timer::from_duration(Duration::from_millis(250)), move |_, _, state| {
            let gone = child.as_mut().map(|c| !matches!(c.try_wait(), Ok(None))).unwrap_or(true);
            if gone {
                child = None;
                state.running.store(false, std::sync::atomic::Ordering::SeqCst);
                return TimeoutAction::Drop;
            }
            TimeoutAction::ToDuration(Duration::from_millis(250))
        });
    }

    pub fn watch_reader(&mut self) {
        if !self.reader_keys.start_watching() {
            return;
        }
        let _ = self.handle.insert_source(Timer::from_duration(Duration::from_millis(100)), |_, _, state| {
            state.reader_listen();
            if state.reader_keys.keep_watching() {
                TimeoutAction::ToDuration(Duration::from_millis(100))
            } else {
                TimeoutAction::Drop
            }
        });
    }

    pub fn watch_settings(&mut self) {
        let mut last = settings_stamp();
        let _ = self.handle.insert_source(Timer::from_duration(Duration::from_secs(1)), move |_, _, state| {
            let now = settings_stamp();
            if now != last {
                last = now;
                state.reconfigure();
            }
            TimeoutAction::ToDuration(Duration::from_secs(1))
        });
    }

    pub fn reconfigure(&mut self) {
        crate::config::apply_environment();
        crate::sheet::reread();
        let older_before = self.config.older_programs;
        self.config = crate::config::Config::load();
        if !launch().greeter {
            self.older_programs_changed(older_before);
        }
        crate::shell::ssd::set_theme(self.config.theme, self.config.corner_radius);
        let pointer = self.config.pointer.clone();
        self.backend_data.apply_input_settings(&pointer);
        self.backend_data.reload_pointer();
        if let Some(keyboard) = self.seat.get_keyboard() {
            keyboard.change_repeat_info(self.config.repeat_rate, self.config.repeat_delay);
            let names = crate::config::xkb_names();

            if names != self.xkb_names {
                let before = keyboard.modifier_state();
                let (layout, variant, model, options) = names.clone();
                let xkb = XkbConfig { rules: "", model: &model, layout: &layout, variant: &variant, options };
                match keyboard.set_xkb_config(self, xkb) {
                    Ok(()) => {
                        let mut after = keyboard.modifier_state();
                        after.num_lock = before.num_lock;
                        after.caps_lock = before.caps_lock;
                        keyboard.set_modifier_state(after);
                        self.backend_data.update_led_state(keyboard.led_state());
                        self.xkb_names = names;
                    }
                    Err(e) => tracing::warn!("keyboard layout not changed: {e:?}"),
                }
            }
        }
        self.set_workspace_count(self.config.workspaces);
        self.keys_reread();
        tracing::info!("settings read again");
    }
}

pub fn let_go(mut child: Child) {
    std::thread::spawn(move || {
        let _ = child.wait();
    });
}
