// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;
use std::time::Duration;

use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::reexports::calloop::{Interest, Mode, PostAction};
use smithay::input::keyboard::XkbConfig;

use crate::settings_watch::SettingsWatch;
use crate::state::{Compose, Backend};

const SETTINGS_LOOK: Duration = Duration::from_secs(3);

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

pub fn watch<B: Backend + 'static, F>(handle: &smithay::reexports::calloop::LoopHandle<'static, Compose<B>>, timer: Timer, callback: F)
where
    F: FnMut(std::time::Instant, &mut (), &mut Compose<B>) -> TimeoutAction + 'static,
{
    if let Err(e) = handle.insert_source(timer, callback) {
        tracing::warn!("a timer could not be put on the loop: {e}");
    }
}

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

static SESSION_ENV: std::sync::Mutex<Vec<(String, String)>> = std::sync::Mutex::new(Vec::new());

pub fn set_env(name: &str, value: impl Into<String>) {
    let mut env = SESSION_ENV.lock().unwrap_or_else(|e| e.into_inner());
    env.retain(|(n, _)| n != name);
    env.push((name.to_string(), value.into()));
}

pub fn env_var(name: &str) -> Option<String> {
    let set = SESSION_ENV.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|(n, _)| n == name).map(|(_, v)| v.clone());
    set.or_else(|| std::env::var(name).ok())
}

pub fn usual_files(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    for (name, value) in SESSION_ENV.lock().unwrap_or_else(|e| e.into_inner()).iter() {
        cmd.env(name, value);
    }
    let Some(&limit) = FILES.get() else { return };
    // SAFETY: between fork and exec the hook only calls setrlimit, which is

    unsafe {
        cmd.pre_exec(move || {
            libc::setrlimit(libc::RLIMIT_NOFILE, &limit);
            Ok(())
        });
    }
}

impl<BackendData: Backend + 'static> Compose<BackendData> {

    pub fn session_ready(&mut self) {
        if let Some(name) = &self.socket_name {
            set_env("WAYLAND_DISPLAY", name.as_str());
        }
        if let Some(x) = self.xdisplay {
            set_env("DISPLAY", format!(":{x}"));
        }

        set_env("TRINDLE_COMPOSE_PID", std::process::id().to_string());
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
        watch(&self.handle, Timer::from_duration(Duration::from_millis(250)), move |_, _, state| {
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
        watch(&self.handle, Timer::from_duration(Duration::from_millis(100)), |_, _, state| {
            state.reader_listen();
            if state.reader_keys.keep_watching() {
                TimeoutAction::ToDuration(Duration::from_millis(100))
            } else {
                TimeoutAction::Drop
            }
        });
    }

    pub fn watch_settings(&mut self) {
        let heard = SettingsWatch::new(crate::config::sources()).and_then(|watch| {
            let source = Generic::new(watch.fd()?, Interest::READ, Mode::Level);
            Ok((watch, source))
        });
        let (mut watch, source) = match heard {
            Ok(both) => both,
            Err(e) => return self.look_at_settings(&e),
        };
        let added = self.handle.insert_source(source, move |_, _, state| match watch.take_news() {
            Ok(true) => {
                state.reconfigure();
                Ok(PostAction::Continue)
            }
            Ok(false) => Ok(PostAction::Continue),
            Err(e) => {
                state.reconfigure();
                state.look_at_settings(&e);
                Ok(PostAction::Remove)
            }
        });
        if let Err(e) = added {
            self.look_at_settings(&e.error);
        }
    }

    fn look_at_settings(&mut self, why: &dyn std::fmt::Display) {
        tracing::warn!("settings changes are looked for every {} seconds: inotify could not watch them: {why}", SETTINGS_LOOK.as_secs());
        let mut last = settings_stamp();
        watch(&self.handle, Timer::from_duration(SETTINGS_LOOK), move |_, _, state| {
            let now = settings_stamp();
            if now != last {
                last = now;
                state.reconfigure();
            }
            TimeoutAction::ToDuration(SETTINGS_LOOK)
        });
    }

    pub fn reconfigure(&mut self) {
        crate::config::reapply_environment();
        crate::sheet::reread();
        crate::config::note_reread();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_session_environment_goes_to_programs_started_and_not_into_this_one() {
        set_env("TRINDLE_TEST_SESSION_ENV", "one");
        set_env("TRINDLE_TEST_SESSION_ENV", "two");
        let mut cmd = Command::new("true");
        usual_files(&mut cmd);
        let given: Vec<_> = cmd.get_envs().filter(|(n, _)| *n == "TRINDLE_TEST_SESSION_ENV").collect();
        assert_eq!(given.len(), 1);
        assert_eq!(given[0].1, Some(std::ffi::OsStr::new("two")));
        assert!(std::env::var_os("TRINDLE_TEST_SESSION_ENV").is_none());
    }
}
