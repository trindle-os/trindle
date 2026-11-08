//! The one way Trindle shows a job under way: a thin line filling round the
//! edge of the round button that started it, clockwise from the top, with
//! the button's own icon still in the middle. No number, nothing turning, no
//! bar. Joined up at the top in the words' plain colour, it is done; red
//! where it stopped, it was not.

use std::cell::Cell;
use std::time::{Duration, Instant};

pub const SIZE: i32 = 36;

pub const STEP_SECS: f64 = 20.0;

const CREEP: f64 = 0.85;

const MOST: f64 = 0.98;

const START: f64 = 0.03;

pub const JOINED: Duration = Duration::from_millis(1500);

pub fn step_line(n: usize, of: usize, words: &str) -> String {
    format!("step {n} of {of}: {words}")
}

pub fn parse_step(line: &str) -> Option<(usize, usize, String)> {
    let rest = line.trim().strip_prefix("step ")?;
    let (figures, words) = rest.split_once(':').unwrap_or((rest, ""));
    let (n, of) = figures.split_once(" of ")?;
    let (n, of): (usize, usize) = (n.trim().parse().ok()?, of.trim().parse().ok()?);
    (1..=of).contains(&n).then(|| (n, of, words.trim().to_string()))
}

pub fn within_line(share: f64) -> String {
    format!("within {:.3}", share.clamp(0.0, 1.0))
}

pub fn parse_within(line: &str) -> Option<f64> {
    let share: f64 = line.trim().strip_prefix("within ")?.trim().parse().ok()?;
    (0.0..=1.0).contains(&share).then_some(share)
}

pub fn creep(secs: f64, step_secs: f64) -> f64 {
    CREEP * (1.0 - (-secs.max(0.0) / step_secs.max(1.0)).exp())
}

pub fn how_far(step: Option<(usize, usize)>, within: Option<f64>, secs: f64, step_secs: f64) -> f64 {
    let (n, of) = step.filter(|(n, of)| *n >= 1 && n <= of).unwrap_or((1, 1));
    let each = 1.0 / of as f64;
    let part = within.map_or_else(|| creep(secs, step_secs), |w| w.clamp(0.0, 1.0));
    ((n - 1) as f64 * each + part * each).clamp(START, MOST)
}

pub struct Dial {
    stepped: bool,
    step: Option<(usize, usize)>,
    within: Option<f64>,
    inner: (usize, usize),
    since: Instant,
    step_secs: f64,
    shown: Cell<f64>,
    ended: Option<bool>,
    words: String,
}

impl Default for Dial {
    fn default() -> Self {
        Dial { stepped: false, step: None, within: None, inner: (1, 1), since: Instant::now(),
               step_secs: STEP_SECS, shown: Cell::new(0.0), ended: None, words: String::new() }
    }
}

impl Dial {

    pub fn expect(&mut self, secs: f64) {
        self.step_secs = secs;
    }

    pub fn by_steps(&mut self) {
        self.stepped = true;
    }

    pub fn step(&mut self, n: usize, of: usize, words: &str) {
        self.step = Some((n, of));
        self.within = None;
        self.inner = (1, 1);
        self.since = Instant::now();
        if !words.is_empty() {
            self.words = words.to_string();
        }
    }

    pub fn within(&mut self, share: f64) {
        self.within = Some(share);
    }

    pub fn heard(&mut self, line: &str) -> bool {
        if let Some((n, of, words)) = parse_step(line) {
            self.step(n, of, &words);
            return true;
        }
        if let Some(share) = parse_within(line) {
            self.within(share);
            return true;
        }
        false
    }

    pub fn say(&mut self, words: &str) {
        self.words = words.to_string();
    }

    pub fn finish(&mut self, ok: bool, words: &str) {
        self.ended = Some(ok);
        self.words = words.to_string();
    }

    pub fn running(&self) -> bool {
        self.ended.is_none()
    }

    pub fn words(&self) -> &str {
        &self.words
    }

    pub fn share(&self) -> f32 {
        match self.ended {
            Some(true) => 1.0,
            Some(false) => self.shown.get() as f32,
            None if self.stepped && self.step.is_none() => START as f32,
            None => {
                let secs = self.since.elapsed().as_secs_f64();
                let share = how_far(self.step, self.within, secs, self.step_secs).max(self.shown.get());
                self.shown.set(share);
                share as f32
            }
        }
    }

    pub fn tone(&self) -> crate::toolkit::widget::Tone {
        use crate::toolkit::widget::Tone;
        match self.ended {
            None => Tone::Calm,
            Some(true) => Tone::Plain,
            Some(false) => Tone::Problem,
        }
    }

    pub fn round<M>(&self, b: crate::toolkit::widget::Node<M>) -> crate::toolkit::widget::Node<M> {
        b.working(self.share(), self.tone())
    }

    pub fn node<M>(&self) -> crate::toolkit::widget::Node<M> {
        crate::toolkit::widget::ring(self.share(), SIZE as f32).tone(self.tone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn steps_read_back_as_written() {
        assert_eq!(parse_step(&step_line(2, 4, "Reading its list")), Some((2, 4, "Reading its list".into())));
        assert_eq!(parse_step("step 1 of 1\n"), Some((1, 1, String::new())));
        assert_eq!(parse_step("step 5 of 4: too far"), None);
        assert_eq!(parse_step("step 0 of 4: none"), None);
        assert_eq!(parse_step("Get:1 https://deb.debian.org trixie InRelease"), None);
        assert_eq!(parse_step("step two of four"), None);
    }

    #[test]
    fn shares_within_a_step_read_back() {
        assert_eq!(parse_within(&within_line(0.42)), Some(0.42));
        assert_eq!(parse_within(&within_line(3.0)), Some(1.0));
        assert_eq!(parse_within("within 1.5"), None);
        assert_eq!(parse_within("within half"), None);
        assert_eq!(parse_within("Unpacking within 0.5"), None);
    }

    #[test]
    fn each_step_has_an_equal_share() {

        assert!(near(how_far(Some((2, 4)), Some(0.5), 0.0, STEP_SECS), 0.375));

        assert!(near(how_far(Some((3, 4)), Some(0.0), 0.0, STEP_SECS), 0.5));

        assert!(near(how_far(None, Some(0.6), 0.0, STEP_SECS), 0.6));

        assert!(near(how_far(Some((5, 4)), Some(0.6), 0.0, STEP_SECS), 0.6));
    }

    #[test]
    fn the_guess_slows_and_never_ends_a_step() {
        let s = STEP_SECS;
        assert!(near(creep(0.0, s), 0.0));
        let (a, b, c) = (creep(s * 0.5, s), creep(s, s), creep(s * 1.5, s));

        assert!(b - a < a);
        assert!(c - b < b - a);

        assert!(creep(s * 1000.0, s) <= CREEP);
        assert!(how_far(Some((1, 2)), None, s * 1000.0, s) < 0.5);

        assert!(near(creep(-5.0, s), 0.0));
        assert!(creep(10.0, 0.0) <= CREEP);
    }

    #[test]
    fn it_joins_up_only_when_finished() {
        assert!(how_far(None, Some(1.0), 0.0, STEP_SECS) < 1.0);
        assert!(how_far(Some((4, 4)), Some(1.0), 0.0, STEP_SECS) < 1.0);
        assert!(how_far(None, None, 1e9, STEP_SECS) < 1.0);
    }

    #[test]
    fn each_step_starts_at_its_share_and_never_reaches_the_next() {
        assert!(near(how_far(Some((2, 4)), None, 0.0, STEP_SECS), 0.25));
        assert!(near(how_far(Some((3, 3)), None, 0.0, STEP_SECS), 2.0 / 3.0));
        let early = how_far(Some((2, 4)), None, 5.0, STEP_SECS);
        let later = how_far(Some((2, 4)), None, 60.0, STEP_SECS);
        let ages = how_far(Some((2, 4)), None, 100_000.0, STEP_SECS);
        assert!(0.25 < early && early < later && later < ages && ages < 0.5);
    }

    #[test]
    fn a_dial_never_goes_back_and_ends_joined_up() {
        let mut d = Dial::default();
        d.by_steps();
        assert!(near(d.share() as f64, START));
        assert!(d.heard("step 2 of 4: Fetching"));
        let half = d.share();
        assert!(half >= 0.25 && d.words() == "Fetching");
        d.within(0.0);
        assert!(d.share() >= half);
        d.finish(true, "Installed");
        assert!(!d.running() && d.share() == 1.0);
    }

    #[test]
    fn a_sliver_is_always_shown() {
        assert!(near(how_far(None, None, 0.0, STEP_SECS), START));
        assert!(near(how_far(Some((1, 3)), Some(0.0), 0.0, STEP_SECS), START));
    }
}
