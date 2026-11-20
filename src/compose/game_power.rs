//! While a game fills a screen, power-profiles-daemon holds the power profile at performance; the person's own profile comes back once none does.
// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use smithay::desktop::WindowSurface;
use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::reexports::wayland_server::Resource;
use tracing::{info, warn};

use crate::shell::FullscreenSurface;
use crate::state::{Backend, Compose};

mod which;

const LOOK_EVERY: Duration = Duration::from_secs(2);

const ASK_AGAIN: Duration = Duration::from_secs(60);

const X_WAIT: Duration = Duration::from_secs(2);

const PROFILES: &str = "org.freedesktop.UPower.PowerProfiles";
const PROFILES_PATH: &str = "/org/freedesktop/UPower/PowerProfiles";
const PERFORMANCE: &str = "performance";
const REASON: &str = "game on screen";
const APP: &str = "trindle";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Filling {
    program: Program,
    app_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Program {
    Pid(u32),
    Older { display: u32, window: u32 },
}

impl<BackendData: Backend> Compose<BackendData> {

    pub fn watch_game_power(&mut self) {
        if crate::session::launch().greeter {
            return;
        }
        let (tx, rx) = mpsc::channel();
        let started = std::thread::Builder::new().name("game-power".into())
            .spawn(move || hold_while_playing(&rx, &which::Places::computer(), ASK_AGAIN, zbus::blocking::Connection::system));
        if let Err(e) = started {
            warn!("the power profile is not raised for games: {e}");
            return;
        }
        let mut last = Vec::new();
        crate::session::watch(&self.handle, Timer::from_duration(LOOK_EVERY), move |_, _, state| {
            let now = state.filling_screens();
            if now != last {
                if tx.send(now.clone()).is_err() {
                    return TimeoutAction::Drop;
                }
                last = now;
            }
            TimeoutAction::ToDuration(LOOK_EVERY)
        });
    }

    pub(crate) fn filling_screens(&self) -> Vec<Filling> {
        if self.is_locked() {
            return Vec::new();
        }
        let mut filling: Vec<Filling> = self.space.outputs()
            .filter_map(|o| o.user_data().get::<FullscreenSurface>()?.get())
            .filter(|w| !self.space.outputs_for_element(w).is_empty())
            .filter_map(|w| {
                let program = match w.0.underlying_surface() {
                    WindowSurface::Wayland(t) => {
                        let pid = t.wl_surface().client()?.get_credentials(&self.display_handle).ok()?.pid;
                        Program::Pid(u32::try_from(pid).ok()?)
                    }
                    WindowSurface::X11(x) => Program::Older { display: self.rooms.display_of(x.xwm_id()?)?, window: x.window_id() },
                };
                Some(Filling { program, app_id: w.app_id() })
            })
            .collect();
        filling.sort_unstable();
        filling.dedup();
        filling
    }
}

fn hold_while_playing(rx: &Receiver<Vec<Filling>>, places: &which::Places, ask_again: Duration,
                      connect: impl Fn() -> zbus::Result<zbus::blocking::Connection>) {
    let mut hold = Hold::default();
    loop {
        match rx.recv_timeout(ask_again) {
            Ok(mut filling) => {
                while let Ok(newer) = rx.try_recv() {
                    filling = newer;
                }
                hold.wanted = filling.iter().any(|f| which::is_game(places, program_of(&f.program), &f.app_id));
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                hold.wanted = false;
                hold.settle(&connect);
                return;
            }
        }
        hold.settle(&connect);
    }
}

#[derive(Default)]
struct Hold {
    wanted: bool,
    bus: Option<zbus::blocking::Connection>,
    cookie: Option<u32>,
    warned: bool,
}

impl Hold {
    fn settle(&mut self, connect: &impl Fn() -> zbus::Result<zbus::blocking::Connection>) {
        match (self.wanted, self.cookie) {
            (true, None) => self.take(connect),
            (false, Some(cookie)) => self.give_back(cookie),
            _ => {}
        }
    }

    fn take(&mut self, connect: &impl Fn() -> zbus::Result<zbus::blocking::Connection>) {
        let bus = match self.bus.take() {
            Some(bus) => Ok(bus),
            None => connect(),
        };
        let held = bus.and_then(|bus| {
            let reply = bus.call_method(Some(PROFILES), PROFILES_PATH, Some(PROFILES), "HoldProfile", &(PERFORMANCE, REASON, APP))?;
            let cookie: u32 = reply.body().deserialize()?;
            Ok((bus, cookie))
        });
        match held {
            Ok((bus, cookie)) => {
                info!("the power profile is held at performance while a game is on screen");
                (self.bus, self.cookie, self.warned) = (Some(bus), Some(cookie), false);
            }
            Err(e) if !self.warned => {
                warn!("a game is on screen and the power profile could not be raised: {e}");
                self.warned = true;
            }
            Err(_) => {}
        }
    }

    fn give_back(&mut self, cookie: u32) {
        if let Some(bus) = self.bus.take() {
            let _ = bus.call_method(Some(PROFILES), PROFILES_PATH, Some(PROFILES), "ReleaseProfile", &(cookie,));
        }
        self.cookie = None;
        info!("the power profile is given back: no game is on screen");
    }
}

fn program_of(program: &Program) -> Option<u32> {
    match *program {
        Program::Pid(pid) => Some(pid),
        Program::Older { display, window } => older_program(display, window),
    }
}

fn older_program(display: u32, window: u32) -> Option<u32> {
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new().name("game-power-x".into()).spawn(move || {
        let pid = x11rb::connect(Some(&format!(":{display}"))).ok().and_then(|(conn, _)| crate::panel::x11_program(&conn, window));
        let _ = tx.send(pid);
    }).ok()?;
    rx.recv_timeout(X_WAIT).ok().flatten()
}

#[cfg(test)]
mod tests;
