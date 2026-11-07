// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Keeping the ring: starting it, starting it again, ending it when it stops answering.

use std::os::fd::OwnedFd;
use std::process::Child;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::time::{Duration, Instant};

use smithay::reexports::calloop::{channel::{channel, Event, Sender}, timer::{TimeoutAction, Timer}};
use smithay::reexports::wayland_server::backend::{ClientId, DisconnectReason};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Resource;

use super::ended::{ended, ending, left_for_update, stamp, watch};
use super::launch::{nonblocking, pipe, start_on};
use super::pulse::hear_on;
use super::steps::{step, Step};
use super::{start_of, Told, ENDS_WITHIN, LINGER, NAME, RETRY, TRIES, WINDOW};
use crate::state::{AnvilState, Backend, ClientState};

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn keep_ring(&mut self) -> Option<Sender<Told>> {
        crate::session::launch().ring.as_ref()?;
        let (told, heard) = channel::<Told>();
        self.handle
            .insert_source(heard, |event, _, state| {
                if let Event::Msg(told) = event {
                    state.ring_told(told);
                }
            })
            .ok()?;
        self.ring.told = Some(told.clone());
        Some(told)
    }

    fn ring_told(&mut self, told: Told) {
        let running = self.ring.running.as_ref().map(|r| r.0);
        let now = Instant::now();
        if matches!(told, Told::Ended(n) if running == Some(n)) {
            self.collect_ended_ring();
        }
        let step = step(told, running, self.ring.asked, &self.ring.starts, now);
        if told == Told::Now {
            self.ring.asked = true;
        }
        match step {
            Step::Nothing => {}
            Step::Start | Step::Fresh => self.start_when_told(now),
            Step::Look => self.look_again_soon(running),
            Step::End if running.is_some() && self.ring.given_up == running => {}
            Step::End => self.end_ring_that_let_go(),
            Step::Unended => self.say_ring_unended(),
            Step::Wait(long) => self.hold_ring_back(long),
        }
    }

    fn collect_ended_ring(&mut self) {
        if let Some((_, mut child)) = self.ring.running.take() {
            self.ring.given_up = None;
            let from = self.ring.from.take();
            match child.wait() {
                Ok(status) if left_for_update(status, from, crate::session::launch().ring.as_deref().and_then(stamp)) => {
                    self.ring.starts.pop();
                    tracing::info!("the ring left of its own accord for an update: to be started again as the new program, and not counted as one that fell over");
                }
                Ok(status) => tracing::info!("{}: to be started again", ending(status)),
                Err(e) => tracing::info!("the ring ended, and how could not be read ({e}): to be started again"),
            }
        }
    }

    fn start_when_told(&mut self, now: Instant) {
        self.ring.lent = false;
        let words = self.ring.reopen.take();
        if words.is_some() {
            self.ring.wished = Some(now);
        }
        self.start_ring(now, words.as_deref());
    }

    fn look_again_soon(&mut self, running: Option<u64>) {
        let (told, number) = (self.ring.told.clone(), running);
        let _ = self.handle.insert_source(Timer::from_duration(LINGER), move |_, _, _| {
            if let (Some(told), Some(number)) = (&told, number) {
                let _ = told.send(Told::Lingers(number));
            }
            TimeoutAction::Drop
        });
    }

    fn end_ring_that_let_go(&mut self) {
        if let Some((_, child)) = self.ring.running.as_ref().filter(|r| !ended(r.1.id())) {
            tracing::info!("the ring let go of the connection it was handed and is still running: ended, to be started afresh");
            // SAFETY: kill only sends a signal. The child has not
            // been collected, so the number is still its own.
            unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) };
        }
    }

    fn say_ring_unended(&mut self) {
        if self.ring.running.as_ref().is_some_and(|r| !ended(r.1.id())) {
            tracing::error!("the ring had stopped answering and could not be ended: it is still running {} seconds after it was ended. The keyboard stays with the window, and no new ring is started until this one has gone", ENDS_WITHIN.as_secs());
            crate::ring_said::leave(crate::ring_said::Said::NotEnded);
            crate::stopped::say(crate::stopped::Stopped::Ring);
        }
    }

    fn hold_ring_back(&mut self, long: Duration) {
        self.ring.reopen = None;
        tracing::warn!("the ring ended {TRIES} times in a row: waiting before the next start, unless it is asked for");
        crate::ring_said::leave(crate::ring_said::Said::HeldBack);
        crate::stopped::say(crate::stopped::Stopped::Ring);
        let told = self.ring.told.clone();
        let _ = self.handle.insert_source(Timer::from_duration(long), move |_, _, _| {
            if let Some(told) = &told {
                let _ = told.send(Told::Again);
            }
            TimeoutAction::Drop
        });
    }

    pub fn ring_silent(&mut self, surface: &WlSurface, judged: bool) -> bool {
        let running = self.ring.running.as_ref().map(|r| r.0);
        let Some(number) = start_of(surface) else { return false };
        if Some(number) != running || self.ring.given_up == Some(number) {
            return true;
        }
        if !judged || !self.ring.quiet(crate::ring_pulse::now()) {
            return false;
        }
        self.give_up_on_ring(number);
        true
    }

    pub(super) fn give_up_on_ring(&mut self, number: u64) {
        self.ring.given_up = Some(number);
        if let Some((_, child)) = self.ring.running.as_ref().filter(|r| !ended(r.1.id())) {
            tracing::warn!("the ring holds the keyboard and has stopped answering: the keyboard is taken back, its connection closed, and the ring ended, to be started afresh once it has gone");
            #[cfg(test)]
            let spared = self.ring.spared;
            #[cfg(not(test))]
            let spared = false;
            if !spared {
                // SAFETY: kill only sends a signal. The child has not been
                // collected, so the number is still its own. A ring that has
                // stopped answering would not hear a request to leave.
                unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGKILL) };
            }
        }
        let mut clients: Vec<ClientId> = self.ring.client.take().into_iter().collect();
        for layer in self.layer_shell_state.layer_surfaces() {
            let on = layer.wl_surface().client().filter(|_| start_of(layer.wl_surface()) == Some(number)).map(|c| c.id());
            clients.extend(on.filter(|id| !clients.contains(id)));
        }
        for id in clients {
            self.display_handle.backend_handle().kill_client(id, DisconnectReason::ConnectionClosed);
        }
        if let Some(told) = self.ring.told.clone() {
            let _ = self.handle.insert_source(Timer::from_duration(ENDS_WITHIN), move |_, _, _| {
                let _ = told.send(Told::Unended(number));
                TimeoutAction::Drop
            });
        }
    }

    pub(super) fn start_ring(&mut self, now: Instant, words: Option<&[String]>) {
        drop(self.start_ring_holding(now, words));
    }

    fn start_ring_holding(&mut self, now: Instant, words: Option<&[String]>) -> (Option<OwnedFd>, Option<OwnedFd>) {
        let (Some(program), Some(told)) = (crate::session::launch().ring, self.ring.told.clone()) else { return (None, None) };
        self.ring.starts.retain(|t| now.duration_since(*t) < WINDOW);
        self.ring.starts.push(now);
        self.ring.last += 1;
        let number = self.ring.last;
        let background = [String::from("--background")];
        let pulse = pipe();
        let hand = pipe();
        let state = ClientState { ours: true, left: Some((told.clone(), number)), ..ClientState::default() };
        let mut display = self.display_handle.clone();
        let mut handed = None;
        self.ring.from = stamp(&program);
        let started = start_on(&program, NAME, UnixStream::pair(), pulse.as_ref().map(|p| &p.1), hand.as_ref().map(|h| &h.0), words.unwrap_or(&background), |ours| {
            handed = Some(display.insert_client(ours, Arc::new(state))?.id());
            Ok(())
        });
        self.ring.client = handed;
        if let Some(hearing) = self.ring.hearing.take() {
            self.handle.remove(hearing);
        }
        self.ring.pulse = None;
        self.ring.heard = None;
        self.ring.given_up = None;
        self.ring.hand = None;
        match started {
            Ok(child) => self.ring_started(child, number, told, pulse, hand),
            Err(e) => {
                self.ring_not_started(&program, e, told);
                (None, None)
            }
        }
    }

    fn ring_started(&mut self, child: Child, number: u64, told: Sender<Told>, pulse: Option<(OwnedFd, OwnedFd)>, hand: Option<(OwnedFd, OwnedFd)>) -> (Option<OwnedFd>, Option<OwnedFd>) {
        crate::stopped::clear(crate::stopped::Stopped::Ring);
        watch(child.id(), number, told);
        self.ring.running = Some((number, child));
        let (read, write) = pulse.unzip();
        self.ring.pulse = read.and_then(nonblocking);
        self.ring.hearing = self.ring.pulse.as_ref().and_then(|pipe| hear_on(&self.handle, pipe, |state| &mut state.ring));
        let (theirs, ours) = hand.unzip();
        self.ring.hand = ours.and_then(nonblocking);
        (write, theirs)
    }

    fn ring_not_started(&mut self, program: &std::path::Path, e: std::io::Error, told: Sender<Told>) {
        tracing::error!(program = %program.display(), "could not start the ring: {e}");
        crate::ring_said::leave(crate::ring_said::Said::NotStarted);
        crate::stopped::say(crate::stopped::Stopped::Ring);
        self.ring.running = None;
        let _ = self.handle.insert_source(Timer::from_duration(RETRY), move |_, _, _| {
            let _ = told.send(Told::Again);
            TimeoutAction::Drop
        });
    }
}

#[cfg(test)]
impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub(crate) fn start_ring_for_rig(&mut self) -> Option<(u64, std::fs::File, std::fs::File)> {
        self.ring.asked = true;
        let (write, hand) = self.start_ring_holding(Instant::now(), None);
        Some((self.ring.last, std::fs::File::from(write?), std::fs::File::from(hand?)))
    }
}
