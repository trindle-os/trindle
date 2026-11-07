// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Trindle's own panel program, the ring, which also draws the notifications: the
//! compositor starts it itself and hands it
//! its connection, one end of a pair of sockets made for it alone. That
//! connection is the whole proof of which program it is (trusted.rs): a
//! name, an app id or a path can be taken by anything, and anything running
//! as the person can connect to the compositor's socket, but only the
//! program the compositor started holds the other end of the pair.
//!
//! The program is the one beside the compositor's own, found once as the
//! compositor starts and never looked for along PATH, where a folder the
//! person can write to could hold another of the same name.
//!
//! The session's start-up says when the ring may start, through a pipe the
//! compositor hands to it and to nothing else (the door): the desktop's
//! settings and icons are put in place first. It says only "now"; whoever
//! says it, what starts is Trindle's own program.
//!
//! The ring is kept: one that ends is started again, and so is one that
//! lets go of the connection it was handed while still running, as an older
//! ring does when it starts itself afresh after an update and joins like
//! any other program. It is ended first, so the one that follows holds a
//! handed connection again. A ring that is ending closes its connection a
//! moment before its ending is heard, so a closed connection is looked at
//! again after LINGER, and the log says each case as what it is: ended by a
//! signal, left of its own accord, ended with a fault, or let go.
//!
//! Nothing but the compositor starts the ring it keeps. While it is held
//! back after falling over too often, the person asking for it (a click at
//! the screen's edge, Super, any command of the compositor's own that is
//! the ring's) starts it here and now, with the count begun again, where it
//! once started a ring by name that then stood in the kept one's way for
//! the rest of the session. A ring started by name some other way, by
//! another program or before the start-up has said now, gives way: the ring
//! started here takes the ring's name on the session bus from it, and it
//! leaves (ring/kept.rs).
//!
//! A ring held back, or one that could not be started, is left as a word
//! for the next ring to say to the person (common/ring_said.rs), since the
//! log is nowhere they look.
//!
//! What this cannot stop: a program running as the person can end the
//! ring, or take the ring's name on the session bus while the ring is away
//! and refuse to give it up, so that the ring started here finds one
//! already answering and steps aside. Such a program is then asked for the
//! ring and the notifications,
//! but it holds no handed connection, so its panels are ordinary: under
//! Trindle's notifications, had there been any, and never over a
//! full-screen window. And where the system lets one program look inside
//! another of the same person's, a program can change the running ring
//! itself; no connection can say so.

mod asking;
mod ended;
mod keeping;
mod launch;
mod presses;
mod pulse;
mod shot;
mod steps;

pub use launch::{door, program};

use std::time::{Duration, Instant};

use std::process::Child;

use smithay::reexports::calloop::{channel::Sender, RegistrationToken};
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::{Logical, Point};

use crate::state::ClientState;
use ended::Stamp;

pub const DOOR: &str = "TRINDLE_RING_DOOR";
pub const NO_DOOR: &str = "none";
pub const PULSE: &str = "TRINDLE_RING_KEPT";
const SHOT: &str = "trindle-screenshot";
const NAME: &str = "trindle-ring";
const PROGRAM: &str = "trindle";
const TRIES: usize = 5;
const WINDOW: Duration = Duration::from_secs(600);
const RETRY: Duration = Duration::from_secs(10);
const LINGER: Duration = Duration::from_millis(500);
const ENDS_WITHIN: Duration = Duration::from_secs(5);

pub(crate) const UNANSWERED: Duration = Duration::from_millis(if cfg!(test) { 3000 } else { 8000 });
pub const LOG: &str = module_path!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Told {
    Now,
    LetGo(u64),
    Lingers(u64),
    Ended(u64),
    Again,
    Wished,
    Unended(u64),
}

fn ring_words(command: &str) -> Option<Vec<String>> {
    let mut words = command.split_whitespace();
    if words.next()? != NAME {
        return None;
    }
    let option = |w: &&str| {
        w.strip_prefix("--").is_some_and(|o| !o.is_empty() && o.chars().all(|c| c.is_ascii_lowercase() || c == '-'))
    };
    let words: Vec<&str> = words.collect();
    words.iter().all(option).then(|| words.into_iter().map(str::to_string).collect())
}

pub fn opens_ring(command: &str) -> bool {
    ring_words(command).is_some_and(|w| opens(&w))
}

fn opens(words: &[String]) -> bool {
    words.iter().all(|w| matches!(w.as_str(), "--listen" | "--all" | "--power" | "--shortcuts"))
}

#[derive(Default)]
pub struct Kept {
    told: Option<Sender<Told>>,
    running: Option<(u64, Child)>,
    asked: bool,
    last: u64,
    starts: Vec<Instant>,
    from: Option<Stamp>,
    pub(crate) pulse: Option<std::fs::File>,
    heard: Option<u64>,
    hearing: Option<RegistrationToken>,
    client: Option<ClientId>,
    given_up: Option<u64>,
    pub(crate) wished: Option<Instant>,
    pub(crate) by_hand: bool,
    pub(crate) hand_at: Option<Point<f64, Logical>>,
    pub(crate) maker: Option<String>,
    pub(crate) made_at: Option<(String, Point<f64, Logical>)>,
    pub(crate) hands_seen: Option<((bool, Option<String>), bool, Instant)>,
    pub(crate) expecting: Option<(Instant, u64)>,
    hand: Option<std::fs::File>,
    reopen: Option<Vec<String>>,
    lent: bool,
    shots: Vec<Shot>,
    last_shot: u64,
    #[cfg(test)]
    pub(crate) spared: bool,
}

struct Shot {
    number: u64,
    pid: u32,
    pulse: Option<std::fs::File>,
    heard: Option<u64>,
    hearing: Option<RegistrationToken>,
    client: Option<ClientId>,
    given_up: bool,
}

impl std::fmt::Debug for Kept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Kept").field("running", &self.running.as_ref().map(|r| r.0)).field("asked", &self.asked).finish()
    }
}

fn start_of(surface: &WlSurface) -> Option<u64> {
    let client = surface.client()?;
    let state = client.get_data::<ClientState>()?;
    state.left.as_ref().filter(|_| state.ours).map(|l| l.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_real_sessions_log_keeps_the_keepers_own_lines() {
        for rest in [false, true] {
            let kept = crate::kept_in_log(rest);
            assert!(kept.split(',').any(|part| part == format!("{LOG}=info")), "{kept}");
            assert!(kept.starts_with("warn,"), "{kept}");
            assert!(kept.contains(&format!("{}=info", crate::capture::LOG)), "{kept}");
            assert_eq!(kept.contains(&format!("{}=info", crate::rest::LOG)), rest, "{kept}");
            assert!(kept.parse::<tracing_subscriber::EnvFilter>().is_ok(), "{kept}");
        }
        assert_eq!(LOG, "trindle_compose::own_ring");
    }

    #[test]
    fn only_a_command_that_brings_the_ring_out_counts_as_asking_for_it_to_show() {
        let opens = |c: &str| opens(&ring_words(c).unwrap());
        for yes in ["trindle-ring", "trindle-ring --power", "trindle-ring --all", "trindle-ring --listen", "trindle-ring --shortcuts"] {
            assert!(opens(yes), "{yes}");
        }
        for no in ["trindle-ring --corners", "trindle-ring --show-desktop", "trindle-ring --background",
                   "trindle-ring --restyle", "trindle-ring --power --notes", "trindle-ring --notes"] {
            assert!(!opens(no), "{no}");
        }
    }

    #[test]
    fn only_the_rings_own_command_counts_as_asking_for_it() {
        let words = |c: &str| ring_words(c);
        assert_eq!(words("trindle-ring"), Some(vec![]));
        assert_eq!(words("trindle-ring --corners"), Some(vec!["--corners".to_string()]));
        assert_eq!(words("  trindle-ring   --power  --all "), Some(vec!["--power".to_string(), "--all".to_string()]));
        for other in ["", "foot", "trindle-ringer", "/tmp/trindle-ring", "trindle-ring; foot", "trindle-ring --say hello",
                      "trindle-ring && foot", "trindle-ring --", "trindle-ring -x", "trindle-ring $(foot)", "env trindle-ring"] {
            assert_eq!(words(other), None, "{other}");
        }
    }
}
