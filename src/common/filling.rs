//! The one way Trindle shows a job under way: a thin line filling round the
//! edge of the round button that started it, clockwise from the top, with
//! the button's own icon still in the middle. No number, nothing turning, no
//! bar. Joined up at the top in the words' plain colour, it is done; red
//! where it stopped, it was not.

use std::cell::Cell;
use std::time::{Duration, Instant};

use crate::toolkit::tiles::{RingShare, Sketch};
use crate::toolkit::widget::{drawing, Node, Tone};

pub const SIZE: i32 = 36;

pub const STEP_SECS: f64 = 20.0;

const CREEP: f64 = 0.85;

const MOST: f64 = 0.98;

const START: f64 = 0.03;

pub const JOINED: Duration = Duration::from_millis(1500);

pub const TURN: Duration = Duration::from_millis(250);
const TURNS_A_SECOND: u32 = (1000 / TURN.as_millis()) as u32;

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
    since: Instant,
    step_secs: f64,
    shown: Cell<f64>,
    ended: Option<bool>,
    words: String,
    ring: RingShare,
    whole: Cell<u8>,
}

impl Default for Dial {
    fn default() -> Self {
        Dial { stepped: false, step: None, within: None, since: Instant::now(),
               step_secs: STEP_SECS, shown: Cell::new(0.0), ended: None, words: String::new(),
               ring: RingShare::new(Some(START as f32)), whole: Cell::new(percent(START as f32)) }
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

    pub fn hear(&mut self, line: &str) -> bool {
        self.heard(line) && parse_within(line).is_none()
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

    pub fn round<M>(&self, b: Node<M>) -> Node<M> {
        b.working_following(&self.ring(), self.tone())
    }

    pub fn node<M>(&self) -> Node<M> {
        if self.running() {
            return following_ring(&self.ring(), SIZE as f32, Tone::Calm);
        }
        crate::toolkit::widget::ring(self.share(), SIZE as f32).tone(self.tone())
    }

    pub fn ring(&self) -> RingShare {
        let share = self.share();
        self.ring.set(Some(share));
        self.whole.set(percent(share));
        self.ring.clone()
    }

    pub fn turn(&self) -> bool {
        let share = self.share();
        self.ring.set(Some(share));
        let whole = percent(share);
        whole != self.whole.replace(whole)
    }
}

pub fn percent(share: f32) -> u8 {
    (share.clamp(0.0, 1.0) * 100.0).floor() as u8
}

pub fn following_ring<M>(share: &RingShare, size: f32, tone: Tone) -> Node<M> {
    use crate::toolkit::theme::{alpha, calm};
    let (now, kept, read) = (share.get(), share.clone(), share.clone());
    let line = (size / 8.4).round().max(6.0);
    let (middle, radius) = (size / 2.0, size / 2.0 - line / 2.0 - 1.0);
    drawing((size, size), "", move |canvas, pen| {
        let v = read.get().unwrap_or(0.0);
        let ink = if v >= 1.0 && tone == Tone::Plain { pen.theme.fg } else { crate::toolkit::ui::state(pen.theme, tone, calm()) };
        let mut s = Sketch::fit(canvas, pen, (size, size));
        s.ring((middle, middle), radius, line, alpha(pen.theme.fg, 0.10));
        s.arc((middle, middle), radius, line, 0.0, v, ink);
    }).alive().progress(now).following(&kept).called(crate::toolkit::t!("kit-progress"))
}

#[derive(Default)]
pub struct Turning {
    turns: Cell<u32>,
    moved: Cell<bool>,
}

impl Turning {
    pub fn turned(&self, moved: bool) -> bool {
        let turns = self.turns.get().wrapping_add(1);
        self.turns.set(turns);
        let moved = self.moved.get() | moved;
        let due = moved && turns % TURNS_A_SECOND == 0;
        self.moved.set(moved && !due);
        due
    }

    pub fn turned_alone(&self, moved: bool) -> bool {
        self.turned(moved && !crate::common::graphics::moving())
    }

    pub fn moved(&self) {
        self.moved.set(true);
    }
}

#[cfg(test)]
pub fn following_buttons<M>(n: &Node<M>) -> Vec<(String, RingShare)> {
    let mut out = Vec::new();
    if let (crate::toolkit::widget::What::Button { tip, working: Some(_), .. }, Some(says)) = (&n.what, &n.says) {
        out.extend(says.following.clone().map(|r| (tip.clone(), r)));
    }
    for i in n.items() {
        out.extend(following_buttons(i));
    }
    out
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

    struct Counted<F: Fn() -> Node<()>> {
        draw: F,
        built: Cell<u32>,
    }

    impl<F: Fn() -> Node<()>> crate::toolkit::ui::App for Counted<F> {
        type Msg = ();
        fn update(&mut self, _: ()) {}
        fn view(&self) -> Node<()> {
            self.built.set(self.built.get() + 1);
            (self.draw)()
        }
    }

    fn counted<F: Fn() -> Node<()>>(draw: F) -> crate::toolkit::testing::Harness<Counted<F>> {
        let side = SIZE as u32 + 8;
        crate::toolkit::testing::Harness::new(Counted { draw, built: Cell::new(0) }, (side, side))
    }

    #[test]
    fn a_running_dial_moves_its_ring_with_no_new_view() {
        let dial = std::rc::Rc::new({
            let mut d = Dial::default();
            d.expect(1.0);
            d
        });
        let d = dial.clone();
        let mut h = counted(move || d.node());
        let first = h.picture(1.0);
        if crate::common::graphics::moving() && !crate::toolkit::motion::reduced() {
            assert!(h.ui().only_turns(), "the window paints the ring alone");
        }
        let mut moved = false;
        for _ in 0..20 {
            std::thread::sleep(Duration::from_millis(50));
            moved |= dial.turn();
        }
        assert!(moved, "the guess moved on by whole percentages");
        assert_ne!(h.picture(1.0).data(), first.data(), "the ring shows the new figure");
        assert_eq!(h.app().built.get(), 1, "the view was built once");
    }

    #[test]
    fn a_running_job_moves_its_button_edge_with_no_new_view() {
        let dial = std::rc::Rc::new({
            let mut d = Dial::default();
            d.expect(1.0);
            d
        });
        let d = dial.clone();
        let mut h = counted(move || d.round(crate::toolkit::widget::button("list-add-symbolic", "Install")));
        let first = h.picture(1.0);
        let mut moved = false;
        for _ in 0..20 {
            std::thread::sleep(Duration::from_millis(50));
            moved |= dial.turn();
        }
        assert!(moved, "the guess moved on by whole percentages");
        assert_ne!(h.picture(1.0).data(), first.data(), "the button's edge shows the new figure");
        assert_eq!(h.app().built.get(), 1, "the view was built once");
        let shown = following_buttons(&dial.round(crate::toolkit::widget::button::<()>("list-add-symbolic", "Install")));
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].1, dial.ring, "the edge follows the job's own figure");
    }

    #[test]
    fn words_follow_a_moving_figure_at_most_once_a_second() {
        let t = Turning::default();
        let told: Vec<bool> = (0..12).map(|i| t.turned(i == 1 || i == 2)).collect();
        assert_eq!(told.iter().filter(|b| **b).count(), 1, "{told:?}");
        assert!(told[TURNS_A_SECOND as usize - 1], "told at the second");
        assert_eq!((0..40).filter(|_| t.turned(true)).count(), 40 / TURNS_A_SECOND as usize);
        assert_eq!((0..40).filter(|_| t.turned(false)).count(), 0);
        t.moved();
        assert_eq!((0..TURNS_A_SECOND).filter(|_| t.turned(false)).count(), 1, "a figure moved elsewhere is shown too");
        let alone = Turning::default();
        let shown = (0..40).filter(|_| alone.turned_alone(true)).count();
        assert_eq!(shown, if crate::common::graphics::moving() { 0 } else { 40 / TURNS_A_SECOND as usize });
    }

    #[test]
    fn only_a_new_step_asks_for_a_new_view() {
        let mut d = Dial::default();
        assert!(!d.hear(&within_line(0.5)));
        assert!(d.hear(&step_line(2, 3, "Fetching")) && d.words() == "Fetching");
        assert!(!d.hear("Get:1 https://deb.debian.org trixie InRelease"));
    }

    #[test]
    fn with_less_movement_the_rings_stand_still() {
        use crate::common::scratch_home;
        if scratch_home::inside().is_none() {
            return scratch_home::rerun(module_path!(), "with_less_movement_the_rings_stand_still");
        }
        crate::toolkit::motion::reduce(true);
        let mut loading = counted(|| crate::toolkit::tiles::loading_state(SIZE as f32));
        let first = loading.picture(1.0);
        assert!(loading.ui().next_wake(Instant::now()).is_none(), "nothing wakes the window");
        let dial = std::rc::Rc::new({
            let mut d = Dial::default();
            d.by_steps();
            d
        });
        let d = dial.clone();
        let mut held = counted(move || d.node());
        let still = held.picture(1.0);
        std::thread::sleep(Duration::from_millis(300));
        assert!(!dial.turn(), "a dial waiting for its first step does not move");
        assert_eq!(loading.picture(1.0).data(), first.data(), "the loading ring stands still");
        assert_eq!(held.picture(1.0).data(), still.data(), "the dial's ring stands still");
    }

    #[test]
    fn a_sliver_is_always_shown() {
        assert!(near(how_far(None, None, 0.0, STEP_SECS), START));
        assert!(near(how_far(Some((1, 3)), Some(0.0), 0.0, STEP_SECS), START));
    }
}
