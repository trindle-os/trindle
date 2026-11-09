//! Stopping a program from its bubble: a first press to arm, a second to end it, and force quit when it will not go.

use std::time::Duration;

use crate::common::ui::DISARM_SECONDS;

use super::{Activity, Do, procs};

const FORCE_AFTER_SECONDS: u64 = 5;

pub(super) const FORCED_SHOWN_SECONDS: u64 = 10;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Stop {

    Refuse,
    Arm,
    Send,
}

pub(super) fn stop_step(armed: bool, own: bool) -> Stop {
    match (own, armed) {
        (false, _) => Stop::Refuse,
        (true, false) => Stop::Arm,
        (true, true) => Stop::Send,
    }
}

impl Activity {

    pub(super) fn stop(&mut self) {
        match stop_step(self.armed, crate::toolkit::window::persons_own(crate::common::person_only::STOP)) {
            Stop::Refuse => self.armed = false,
            Stop::Arm => {
                self.armed = true;
                self.arms += 1;
                let (n, asks) = (self.arms, self.asks.clone());
                self.events.after(Duration::from_secs(DISARM_SECONDS.into()), move || {
                    asks.send(Do::Disarm(n));
                });
            }
            Stop::Send => {
                self.armed = false;
                if let Some(key) = self.orbit.picked() {
                    if let Some(m) = self.members.get(&key) {
                        procs::stop(m);

                        let title = self.infos.get(&key).map(|i| i.title.clone()).unwrap_or_default();
                        let (m, asks) = (m.clone(), self.asks.clone());
                        self.events.after(Duration::from_secs(FORCE_AFTER_SECONDS), move || { asks.send(Do::Force(m, title)); });
                    }
                }
            }
        }
    }
}
