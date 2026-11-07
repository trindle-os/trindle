// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Asking for the ring: a key, a click or a command that brings it out, and the wait for it to answer.

use std::time::Instant;

use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::{Logical, Point};

use super::steps::{step, Step};
use super::{opens, ring_words, start_of, Told, SHOT, UNANSWERED};
use crate::state::{AnvilState, Backend};

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn try_again_at(&mut self, at: Point<f64, Logical>) -> bool {
        let on = self.space.outputs().any(|o| self.space.output_geometry(o).is_some_and(|place| crate::stopped::on_button(place, at)));
        if !on {
            return false;
        }
        match crate::stopped::now() {
            Some(crate::stopped::Stopped::Ring) => {
                tracing::info!("Try again was pressed on what is said of the ring");
                self.ring_asked_for("trindle-ring");
            }
            Some(crate::stopped::Stopped::Shot) => {
                tracing::info!("Try again was pressed on what is said of the screenshot tool");
                self.screenshot_asked_for(SHOT);
            }
            None => return false,
        }
        true
    }

    pub fn ring_asked_for(&mut self, command: &str) -> bool {
        let Some(words) = ring_words(command) else { return false };
        if self.ring.told.is_none() {
            return false;
        }
        if opens(&words) && (self.ring.by_hand || self.summoned()) {
            let running = self.ring.running.as_ref().map(|r| r.0);
            let kept = |surface: &WlSurface| running.is_some() && start_of(surface) == running;
            let mut brought = false;
            for o in self.space.outputs() {
                brought |= crate::trusted::bring_out_unseen(o, kept);
            }
            if brought {
                tracing::info!("the ring was out but hidden by a full-screen window when the person asked for it: brought out over the window");
                return true;
            }
            self.ring.wished = Some(Instant::now());
            self.expect_ring(&words);
        }
        let running = self.ring.running.as_ref().map(|r| r.0);
        let now = Instant::now();
        if step(Told::Wished, running, self.ring.asked, &self.ring.starts, now) != Step::Fresh {
            return false;
        }
        if self.ring.by_hand {
            tracing::info!("the ring was asked for by the person while it was held back: started now, the count begun again");
            self.ring.starts.clear();
            self.ring.lent = false;
        } else if std::mem::replace(&mut self.ring.lent, true) {
            return true;
        } else {
            tracing::info!("the ring was asked for by a made-up key or click while it was held back: started this once, counted with the rest; no other made-up one starts it until the person's own has, or the wait is over");
        }
        self.start_ring(now, Some(&words));
        true
    }

    fn expect_ring(&mut self, words: &[String]) {
        if self.ring.expecting.is_some() || self.ring.running.is_none() {
            return;
        }
        let asked = (Instant::now(), crate::ring_pulse::now());
        self.ring.expecting = Some(asked);
        let words = words.to_vec();
        let _ = self.handle.insert_source(Timer::from_duration(UNANSWERED), move |_, _, state| {
            state.ring_unanswered(asked, &words);
            TimeoutAction::Drop
        });
    }

    fn ring_unanswered(&mut self, asked: (Instant, u64), words: &[String]) {
        if self.ring.expecting != Some(asked) {
            return;
        }
        self.ring.expecting = None;
        let Some(number) = self.ring.running.as_ref().map(|r| r.0) else { return };
        self.ring.hear(crate::ring_pulse::now());
        if self.ring.heard.is_some_and(|heard| heard >= asked.1) || self.ring.given_up == Some(number) {
            return;
        }
        tracing::warn!("the person asked for the ring and in {} seconds it neither came out nor gave a sign of life: it is ended, and the one started in its place is opened as they asked", UNANSWERED.as_secs());
        self.ring.reopen = Some(words.to_vec());
        self.give_up_on_ring(number);
    }

    pub fn spawn_for_click(&mut self, command: &str) {
        let (by_hand, at) = (self.ring.by_hand, Some(self.pointer.current_location()));
        self.ring.by_hand = by_hand && self.ring.hand_at == at;
        let maker = self.ring.maker.take();
        let put_there = |plugin: &String| self.ring.hand_at == at || self.ring.made_at.as_ref().is_some_and(|(by, to)| by == plugin && Some(*to) == at);
        self.ring.maker = maker.clone().filter(put_there);
        self.spawn(command);
        self.ring.by_hand = by_hand;
        self.ring.maker = maker;
    }

    fn summoned(&self) -> bool {
        self.ring.maker.as_deref().is_some_and(crate::summon::allowed)
    }
}
