// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

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
