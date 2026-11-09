// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::os::unix::net::UnixStream;
use std::sync::Arc;

use smithay::reexports::calloop::{generic::Generic, Interest, Mode};
use smithay::reexports::wayland_server::backend::DisconnectReason;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Resource;

use super::ended::still_running;
use super::launch::{nonblocking, pipe, start_on};
use super::{Shot, SHOT};
use crate::state::{AnvilState, Backend, ClientState};

impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn screenshot_asked_for(&mut self, command: &str) -> bool {
        if command.trim() != SHOT || !self.ring.by_hand {
            return false;
        }
        let Some(program) = crate::session::launch().ring else { return false };
        let state = ClientState { asked: true, ..ClientState::default() };
        let mut display = self.display_handle.clone();

        let pulse = pipe();
        let mut handed = None;
        let started = start_on(&program, SHOT, UnixStream::pair(), pulse.as_ref().map(|p| &p.1), None, &[], |ours| {
            handed = Some(display.insert_client(ours, Arc::new(state))?.id());
            Ok(())
        });

        self.ring.shots.retain(|s| s.pulse.is_some() && s.client.is_some());
        match started {
            Ok(child) => {
                self.ring.last_shot += 1;
                let number = self.ring.last_shot;
                let pid = child.id();
                let (read, _write) = pulse.unzip();
                let pulse = read.and_then(nonblocking);

                let hearing = pulse.as_ref().and_then(|pipe| pipe.try_clone().ok()).and_then(|pipe| {
                    let source = Generic::new(pipe, Interest::READ, Mode::Level);
                    self.handle.insert_source(source, move |_, _, data| Ok(data.ring.hear_shot(number))).ok()
                });
                self.ring.shots.push(Shot { number, pid, pulse, heard: None, hearing, client: handed, given_up: false });

                crate::stopped::clear(crate::stopped::Stopped::Shot);
                crate::session::let_go(child);
                true
            }
            Err(e) => {
                tracing::warn!("the screenshot tool could not be started on a connection of its own, so it is run as a command and a full-screen window will hide it: {e}");
                false
            }
        }
    }

    pub fn shot_silent(&mut self, surface: &WlSurface, judged: bool) -> bool {
        let Some(client) = surface.client().map(|c| c.id()) else { return false };
        let Some(at) = self.ring.shots.iter().position(|s| s.client.as_ref() == Some(&client)) else { return false };
        if self.ring.shots[at].given_up {
            return true;
        }
        if !judged {
            return false;
        }
        let now = crate::ring_pulse::now();
        let shot = &mut self.ring.shots[at];
        if let Some(pipe) = shot.pulse.as_mut() {
            let looked = crate::ring_pulse::read(pipe, shot.heard, now);
            shot.heard = looked.heard;
            if looked.closed {
                shot.pulse = None;
            }
        }
        if !crate::ring_pulse::silent(shot.heard, now) {
            return false;
        }
        self.give_up_on_shot(at);
        true
    }

    fn give_up_on_shot(&mut self, at: usize) {
        let shot = &mut self.ring.shots[at];
        shot.given_up = true;
        shot.hearing = None;
        tracing::warn!("the screenshot tool holds the keyboard and has stopped answering: the keyboard is taken back, its connection closed, and the tool ended");
        let (pid, client) = (shot.pid, shot.client.clone());
        if still_running(pid) {
            // SAFETY: kill only sends a signal. The tool was handed to

            unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
        }
        if let Some(id) = client {
            self.display_handle.backend_handle().kill_client(id, DisconnectReason::ConnectionClosed);
        }
        crate::stopped::say(crate::stopped::Stopped::Shot);
    }
}
