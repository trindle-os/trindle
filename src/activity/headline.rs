//! The sentence at the top: how the computer is, in words that change only when it has changed.

use crate::toolkit::t;

use super::HOLD_LOOKS;
use super::reading::{FLAT_OUT, MEMORY_FULL, threads_words};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Mood {
    #[default]
    Calm,
    TickingOver,
    Busy,
    FlatOut,
    MemoryFull,
}

pub(super) struct Moment<'a> {

    pub(super) busy: f64,

    pub(super) memory: f64,
    pub(super) threads: usize,

    pub(super) busiest: &'a [(String, f64)],

    pub(super) biggest: Option<&'a str>,
}

pub(super) fn sentence(m: &Moment) -> (Mood, String) {
    let n = m.threads;
    let on = threads_words(1.0, n);
    let (first, second) = (m.busiest.first(), m.busiest.get(1));
    if m.busy >= FLAT_OUT {
        return (Mood::FlatOut, match (first, second) {
            (Some((a, s)), _) if *s >= 0.85 * m.busy => t!("activity-flat-out-one", a = a, threads = on),
            (Some((a, s1)), Some((b, s2))) if s1 + s2 >= 0.7 * m.busy => t!("activity-flat-out-two", a = a, b = b, threads = on),
            (Some((a, _)), _) => t!("activity-flat-out-busiest", threads = on, a = a),
            _ => t!("activity-flat-out", threads = on),
        });
    }
    if m.memory >= MEMORY_FULL {
        return (Mood::MemoryFull, match m.biggest {
            Some(b) => t!("activity-memory-full-named", b = b),
            None => t!("activity-memory-full").into(),
        });
    }
    let most = |mood: Mood, fallback: String| match (first, second) {
        (Some((a, s)), _) if *s >= 0.5 * m.busy => (mood, t!("activity-doing-most", a = a)),
        (Some((a, _)), Some((b, _))) if mood != Mood::TickingOver => (mood, t!("activity-two-keeping-busy", a = a, b = b)),
        (Some((a, _)), _) => (mood, t!("activity-ticking-busiest", a = a)),
        _ => (mood, fallback),
    };
    if m.busy >= 0.5 {
        return most(Mood::Busy, t!("activity-busy").into());
    }
    if m.busy >= 0.15 {
        return most(Mood::TickingOver, t!("activity-ticking-over").into());
    }
    match (first, m.biggest) {
        (Some((a, s)), _) if s * n as f64 >= 0.8 => (Mood::TickingOver, t!("activity-plenty-to-spare", a = a)),
        (_, Some(b)) if m.memory >= 0.75 => (Mood::Calm, t!("activity-calm-memory-filling", b = b)),
        _ => (Mood::Calm, t!("activity-all-calm").into()),
    }
}

#[derive(Default)]
pub(super) struct Headline {
    words: String,
    mood: Mood,
    held: u32,
}

impl Headline {

    pub(super) fn offer(&mut self, mood: Mood, words: String) -> bool {
        if words == self.words {
            self.held += 1;
            return false;
        }
        let memory_turned = (mood == Mood::MemoryFull) != (self.mood == Mood::MemoryFull);
        let big = self.words.is_empty() || memory_turned || (mood as u8).abs_diff(self.mood as u8) >= 2;
        if self.held + 1 < HOLD_LOOKS && !big {
            self.held += 1;
            return false;
        }
        *self = Headline { words, mood, held: 0 };
        true
    }
}
