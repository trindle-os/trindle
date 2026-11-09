//! The computer right now as four cards of an amount each: memory, the
//! processor, the disk, and the swap file (or, on a computer with no swap
//! file, how warm the processor is). Each is the standard ring of Trindle's
//! desktop style, an amount of a limit: the percentage on its left, the
//! limit in words beside it, and one sentence above saying what it shows.
//! The ring is in the highlight until the amount is a real worry, when it
//! takes the busy or bad colour, so colour is only ever a status.
//! The memory card says Trindle's own share, which is the point: people
//! are told it uses very little, and here they can see it for themselves.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

use tiny_skia::{FillRule, LineCap, LineJoin, PathBuilder, Stroke, Transform};

use crate::toolkit::{t, theme};
use crate::toolkit::colour::{Colour, paint as brush};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::tiles::{empty_state, loading_state, Sketch, tile_head};
use crate::toolkit::widget::{card, column, drawing, Length, Node, note, Pen, row, title, Tone};

use super::procs::{field_kb, read};
use super::reading::{FLAT_OUT, MEMORY_FULL};

const HISTORY: usize = 60;

const RING: f32 = 60.0;

const CARD_WORDS: f32 = 240.0;

const EASE: f64 = 0.12;

const THERE: f64 = 0.0005;

const FRAME: Duration = Duration::from_millis(16);

const MEMORY_WORRY: (f64, f64) = (0.75, MEMORY_FULL);
const PROCESSOR_WORRY: (f64, f64) = (0.6, FLAT_OUT);
const DISK_WORRY: (f64, f64) = (0.85, 0.95);
const SWAP_WORRY: (f64, f64) = (0.75, 0.9);
const HEAT_WORRY: (f64, f64) = (0.8, 0.9);

type Drawn = Rc<RefCell<Vec<Vec<f64>>>>;

pub fn percent(share: f64) -> String {
    format!("{:.0}%", share.clamp(0.0, 1.0) * 100.0)
}

pub fn dot<M>(size: f32, colour: impl Fn(&Pen) -> Colour + 'static) -> Node<M> {
    drawing((size, size), "", move |canvas, pen| {
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        if let Some(path) = PathBuilder::from_circle(w / 2.0, h / 2.0, w.min(h) / 2.0) {
            canvas.fill_path(&path, &brush(colour(pen)), FillRule::Winding, Transform::identity(), None);
        }
    })
}

fn worry(share: f64, (busy, bad): (f64, f64)) -> Option<Colour> {
    if share >= bad {
        Some(theme::problem())
    } else if share >= busy {
        Some(theme::warn())
    } else {
        None
    }
}

fn ring<M>(status: Option<Colour>, tip: String, drawn: Drawn, at: usize) -> Node<M> {
    drawing((RING, RING), tip, move |canvas, pen| {
        let share = drawn.borrow().get(at).and_then(|d| d.first().copied()).unwrap_or(0.0) as f32;
        let mut s = Sketch::fit(canvas, pen, (84.0, 84.0));
        let track = s.track();
        s.ring((42.0, 42.0), 34.0, 15.0, track);
        let colour = status.unwrap_or_else(|| s.highlight());
        s.arc((42.0, 42.0), 34.0, 15.0, 0.0, share, colour);
    })
}

fn history_line<M>(history: Vec<f64>) -> Node<M> {
    drawing((0.0, 30.0), "", move |canvas, pen| {

        if history.len() < 5 {
            return;
        }
        let colour = theme::pair()[1];
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let step = w / (HISTORY - 1) as f32;
        let x0 = w - step * (history.len() - 1) as f32;
        let y = |v: f64| h - 2.0 * pen.scale - v.clamp(0.0, 1.0) as f32 * (h - 4.0 * pen.scale);
        let mut area = PathBuilder::new();
        area.move_to(x0, h);
        for (i, v) in history.iter().enumerate() {
            area.line_to(x0 + i as f32 * step, y(*v));
        }
        area.line_to(w, h);
        area.close();
        if let Some(path) = area.finish() {
            canvas.fill_path(&path, &brush(theme::alpha(colour, 0.25)), FillRule::Winding, Transform::identity(), None);
        }
        let mut edge = PathBuilder::new();
        for (i, v) in history.iter().enumerate() {
            let x = x0 + i as f32 * step;
            if i == 0 { edge.move_to(x, y(*v)) } else { edge.line_to(x, y(*v)) }
        }
        if let Some(path) = edge.finish() {
            let stroke = Stroke { width: 3.0 * pen.scale, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Stroke::default() };
            canvas.stroke_path(&path, &brush(colour), &stroke, Transform::identity(), None);
        }
    }).width(Length::Fill)
}

struct Tile<M> {
    icon: &'static str,
    name: &'static str,

    sentence: String,

    share: Option<f64>,

    figure: String,
    limit: String,
    status: Option<Colour>,

    extra: Option<Node<M>>,
}

impl<M: 'static> Tile<M> {

    fn view(self, drawn: Drawn, at: usize, known: bool, failed: bool) -> Node<M> {
        let body = match (known, self.share) {

            (false, _) if failed => note(t!("activity-ring-not-read")).tone(Tone::Problem),
            (false, _) => loading_state(RING),
            (true, None) => empty_state(self.sentence, None),
            (true, Some(_)) => {
                let tip = format!("{}: {} {}", self.name, self.figure, self.limit);

                let mut all = vec![note(self.sentence)];
                all.extend(self.extra);
                all.push(row(vec![title(self.figure), ring(self.status, tip, drawn, at), note(self.limit).width(Length::Fill)])
                    .gap(12.0).centred());
                column(all).gap(12.0)
            }
        };
        let b = theme::bento();
        card(column(vec![tile_head(self.icon, self.name, Vec::new()), body]).gap(16.0))
            .height(Length::Fill).beside().span_grows(b.figure.0, b.figure.1, CARD_WORDS)
    }
}

pub struct Rings {
    history: VecDeque<f64>,
    has_swap: bool,
    heat_known: bool,

    known: bool,

    unread: bool,
    trindle_kb: u64,
    busy: f64,
    threads: usize,

    mem: (u64, u64),
    swap: (u64, u64),
    disk: (u64, u64),
    heat: Option<f64>,

    drawn: Drawn,
    target: Rc<RefCell<Vec<Vec<f64>>>>,
    easing: Rc<Cell<bool>>,
}

impl Rings {
    pub fn new() -> Rings {
        Rings {
            history: VecDeque::new(),

            has_swap: meminfo()("SwapTotal:") > 0,
            heat_known: temperature().is_some(),
            known: false,
            unread: false,
            trindle_kb: 0,
            busy: 0.0,
            threads: 0,
            mem: (0, 0),
            swap: (0, 0),
            disk: (0, 0),
            heat: None,
            drawn: Rc::default(),
            target: Rc::default(),
            easing: Rc::default(),
        }
    }

    pub fn ease(&self, events: &Loop, redraw: impl Fn() + 'static) {
        let targets: Vec<Vec<f64>> = self.tiles::<()>().iter().map(|t| vec![t.share.unwrap_or(0.0)]).collect();
        *self.target.borrow_mut() = targets;
        if !crate::common::graphics::moving() {
            *self.drawn.borrow_mut() = self.target.borrow().clone();
            redraw();
            return;
        }
        if self.easing.replace(true) {
            return;
        }
        let (drawn, target, easing) = (self.drawn.clone(), self.target.clone(), self.easing.clone());
        events.every(FRAME, move || {
            let moving = step(&mut drawn.borrow_mut(), &target.borrow());
            redraw();
            if moving {
                return Flow::Continue;
            }
            easing.set(false);
            Flow::Stop
        });
    }

    #[cfg(test)]
    pub fn rest(&self) {
        let targets: Vec<Vec<f64>> = self.tiles::<()>().iter().map(|t| vec![t.share.unwrap_or(0.0)]).collect();
        *self.target.borrow_mut() = targets.clone();
        *self.drawn.borrow_mut() = targets;
    }

    pub fn show(&mut self, trindle_kb: u64, busy: f64, threads: usize) {
        self.known = true;
        self.trindle_kb = trindle_kb;
        self.busy = busy;
        self.threads = threads;
        self.history.push_back(busy);
        while self.history.len() > HISTORY {
            self.history.pop_front();
        }
        let m = meminfo();
        self.mem = (m("MemTotal:"), m("MemAvailable:"));
        self.swap = (m("SwapTotal:"), m("SwapFree:"));
        self.disk = disk_space();
        if !self.has_swap && self.heat_known {
            self.heat = temperature();
        }
    }

    pub fn failed(&mut self, yes: bool) {
        self.unread = yes;
    }

    pub fn view<M: 'static>(&self) -> Vec<Node<M>> {
        let (known, unread) = (self.known, self.unread);
        self.tiles().into_iter().enumerate().map(|(i, t)| t.view(self.drawn.clone(), i, known, unread)).collect()
    }

    fn tiles<M: 'static>(&self) -> Vec<Tile<M>> {
        vec![self.memory_tile(), self.processor_tile(), self.disk_tile(), self.swap_tile()]
    }

    fn memory_tile<M: 'static>(&self) -> Tile<M> {
        let (total, avail) = self.mem;
        let used = total.saturating_sub(avail);
        let ours = self.trindle_kb.min(used);
        let of = |v: u64| if total > 0 { v as f64 / total as f64 } else { 0.0 };
        Tile {
            icon: "memory-symbolic",
            name: t!("activity-memory"),
            sentence: t!("activity-memory-sentence", used = size(used * 1024), ours = size(ours * 1024)),
            share: (total > 0).then(|| of(used)),
            figure: percent(of(used)),
            limit: t!("activity-of-size", amount = size(total * 1024)),
            status: worry(of(used), MEMORY_WORRY),
            extra: None,
        }
    }

    fn processor_tile<M: 'static>(&self) -> Tile<M> {
        let busy = self.busy;
        Tile {
            icon: "cpu-symbolic",
            name: t!("activity-processor"),
            sentence: t!("activity-processor-sentence", threads = super::reading::threads_words(1.0, self.threads)),
            share: Some(busy),
            figure: percent(busy),
            limit: t!("activity-of-threads", n = self.threads),
            status: worry(busy, PROCESSOR_WORRY),
            extra: Some(history_line(self.history.iter().copied().collect())),
        }
    }

    fn disk_tile<M: 'static>(&self) -> Tile<M> {
        let (free, whole) = self.disk;
        let dused = whole.saturating_sub(free);
        let dshare = if whole > 0 { dused as f64 / whole as f64 } else { 0.0 };
        Tile {
            icon: "drive-harddisk-symbolic",
            name: t!("activity-disk"),
            sentence: t!("activity-disk-sentence", used = size(dused), free = size(free)),
            share: (whole > 0).then_some(dshare),
            figure: percent(dshare),
            limit: t!("activity-of-size", amount = size(whole)),
            status: worry(dshare, DISK_WORRY),
            extra: None,
        }
    }

    fn swap_tile<M: 'static>(&self) -> Tile<M> {
        if self.has_swap {
            let (st, sf) = self.swap;
            let su = st.saturating_sub(sf);
            let sshare = if st > 0 { su as f64 / st as f64 } else { 0.0 };
            Tile {
                icon: "system-suspend-hibernate-symbolic",
                name: t!("activity-swap-file"),
                sentence: t!("activity-swap-sentence", used = size(su * 1024)),
                share: (st > 0).then_some(sshare),
                figure: percent(sshare),
                limit: t!("activity-of-size", amount = size(st * 1024)),
                status: worry(sshare, SWAP_WORRY),
                extra: None,
            }
        } else if let Some(t) = self.heat.filter(|_| self.heat_known) {

            let share = (t / 100.0).clamp(0.0, 1.0);
            Tile {
                icon: "sensors-temperature-symbolic",
                name: t!("activity-temperature"),
                sentence: t!("activity-temperature-sentence").into(),
                share: Some(share),
                figure: format!("{t:.0}°"),
                limit: t!("activity-of-hundred-degrees").into(),
                status: worry(share, HEAT_WORRY),
                extra: None,
            }
        } else {
            Tile {
                icon: "system-suspend-hibernate-symbolic",
                name: t!("activity-swap-file"),
                sentence: t!("activity-no-swap").into(),
                share: None,
                figure: String::new(),
                limit: String::new(),
                status: None,
                extra: None,
            }
        }
    }
}

fn step(drawn: &mut Vec<Vec<f64>>, target: &[Vec<f64>]) -> bool {
    drawn.resize(target.len(), Vec::new());
    let mut moving = false;
    for (d, t) in drawn.iter_mut().zip(target) {
        d.resize(t.len(), 0.0);
        for (s, t) in d.iter_mut().zip(t) {
            let by = (t - *s) * EASE;
            if by.abs() > THERE {
                *s += by;
                moving = true;
            } else {
                *s = *t;
            }
        }
    }
    moving
}

fn temperature() -> Option<f64> {
    let dir = std::fs::read_dir("/sys/class/thermal").ok()?;
    let mut zones: Vec<_> = dir.flatten().map(|e| e.path()).collect();
    zones.sort();
    let pick = zones.iter().find(|z| {
        let kind = read(z.join("type")).to_lowercase();
        kind.contains("x86_pkg") || kind.contains("cpu") || kind.contains("k10temp") || kind.contains("tctl")
    }).or_else(|| zones.first())?;
    let milli: f64 = read(pick.join("temp")).trim().parse().ok()?;
    (milli > 0.0).then_some(milli / 1000.0)
}

pub fn meminfo() -> impl Fn(&str) -> u64 {
    let text = read("/proc/meminfo");
    move |key: &str| field_kb(&text, key)
}

fn disk_space() -> (u64, u64) {
    // SAFETY: statvfs is plain numbers, so all-zeroes is a valid starting

    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: a static NUL-terminated path, and a local struct the kernel fills.
    if unsafe { libc::statvfs(c"/".as_ptr(), &mut st) } != 0 {
        return (0, 0);
    }
    (st.f_bavail * st.f_frsize, st.f_blocks * st.f_frsize)
}

pub fn size(bytes: u64) -> String {
    let mb = bytes as f64 / 1_000_000.0;
    if mb < 0.5 {
        t!("activity-size-under-mb").into()
    } else {
        trindle_toolkit::lang::format::size_from_megabytes(mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_read_the_way_people_say_them() {
        assert_eq!(size(142_000_000), "142 MB");
        assert_eq!(size(3_400_000_000), "3.4 GB");
        assert_eq!(size(300_000), "under 1 MB");
    }

    #[test]
    fn sizes_follow_the_languages_marks_and_units() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let made_up = "kit-number-decimal = ,\nkit-size-gb = GO { $size }\nactivity-size-under-mb = sous 1 Mo\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(size(3_400_000_000), "GO 3,4");
            assert_eq!(size(142_000_000), "142 MB");
            assert_eq!(size(300_000), "sous 1 Mo");
        });
    }

    #[test]
    fn rings_ease_in_from_empty_and_settle() {
        let target = vec![vec![0.6]];
        let mut drawn = Vec::new();
        assert!(step(&mut drawn, &target), "a new ring starts moving");
        assert!(drawn[0][0] > 0.0 && drawn[0][0] < 0.6, "part of the way, not all at once");
        let mut frames = 1;
        while step(&mut drawn, &target) {
            frames += 1;
            assert!(frames < 200, "settles");
        }
        assert!(frames > 10, "takes a little while");
        assert_eq!(drawn, target, "ends exactly on the figures");
    }

    #[test]
    fn an_amount_takes_a_status_colour_only_when_it_is_a_worry() {
        assert_eq!(worry(0.3, MEMORY_WORRY), None);
        assert_eq!(worry(0.8, MEMORY_WORRY), Some(theme::warn()));
        assert_eq!(worry(0.95, MEMORY_WORRY), Some(theme::problem()));
        assert_eq!(worry(1.0, PROCESSOR_WORRY), Some(theme::problem()));
    }
}
