//! How the bubbles move: pushing each other apart, drifting to the middle, settling and the frame loop that runs while anything moves.

use std::f64::consts::{FRAC_PI_2, TAU};
use std::time::{Duration, Instant};

use crate::toolkit::event::Flow;

use super::{BEAT_MS, FREE_ROW, Orbit, RIM_GAP, Room};
use super::paint::{BAR_FULL, BEAT, beat};

const FRAME_MS: u64 = 33;

pub(super) const FILL: f64 = 0.75;

pub(super) const SMALLEST_PX: f64 = 22.0;

pub(super) const GAP: f64 = 0.014;

const PULL: f64 = 0.018;

pub(super) const REST: f64 = 0.0005;

pub(super) fn room(w: f64, h: f64) -> Room {
    let tall = (h - FREE_ROW).max(80.0);
    let rim = (w.min(tall) / 2.0 - BAR_FULL - 4.0).max(40.0);
    Room { cx: w / 2.0, cy: tall / 2.0, radius: rim - RIM_GAP, rim }
}

pub(super) fn thread_at(g: &Room, x: f64, y: f64, threads: usize) -> Option<usize> {
    let (dx, dy) = (x - g.cx, y - g.cy);
    let d = dx.hypot(dy);
    if threads == 0 || d < g.rim - 4.0 || d > g.rim + BAR_FULL + 4.0 {
        return None;
    }
    let angle = (dy.atan2(dx) + FRAC_PI_2).rem_euclid(TAU);
    Some(((angle / TAU * threads as f64) as usize).min(threads - 1))
}

pub fn settle(items: &mut [(f64, f64, f64)]) -> f64 {
    let before: Vec<(f64, f64)> = items.iter().map(|(x, y, _)| (*x, *y)).collect();
    for (x, y, _) in items.iter_mut() {
        *x -= *x * PULL;
        *y -= *y * PULL;
    }
    let n = items.len();

    for _ in 0..2 {
        for (i, j) in (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))) {
            let (xi, yi, ri) = items[i];
            let (xj, yj, rj) = items[j];
            let (dx, dy) = (xj - xi, yj - yi);
            let d = dx.hypot(dy);
            let need = ri + rj + GAP;
            if d >= need {
                continue;
            }

            let (ux, uy) = if d > 1e-9 { (dx / d, dy / d) } else { ((i + j) as f64).sin_cos() };
            let over = need - d;
            let (ai, aj) = (ri * ri, rj * rj);
            let whole = (ai + aj).max(1e-12);
            let (wi, wj) = (aj / whole, ai / whole);
            items[i].0 -= ux * over * wi;
            items[i].1 -= uy * over * wi;
            items[j].0 += ux * over * wj;
            items[j].1 += uy * over * wj;
        }
    }
    for (x, y, r) in items.iter_mut() {
        let d = x.hypot(*y);
        let limit = (1.0 - GAP - *r).max(0.0);
        if d > limit && d > 1e-9 {
            *x *= limit / d;
            *y *= limit / d;
        }
    }
    items.iter().zip(before).map(|((x, y, _), (bx, by))| (x - bx).hypot(y - by)).sum()
}

pub(super) fn arrival(key: &str, r: f64) -> (f64, f64) {
    let h = key.bytes().fold(2166136261u32, |h, b| (h ^ b as u32).wrapping_mul(16777619));
    let angle = h as f64 / u32::MAX as f64 * TAU;
    let d = (1.0 - GAP - r).max(0.0);
    (angle.cos() * d, angle.sin() * d)
}

fn toward(now: f64, to: f64, ease: f64, least: f64) -> (f64, f64) {
    if (to - now).abs() < least {
        return (0.0, to);
    }
    let step = (to - now) * ease;
    (step.abs(), now + step)
}

impl Orbit {

    pub(super) fn settle_now(&self) {
        let least = self.smallest();
        let mut bubbles = self.bubbles.borrow_mut();
        bubbles.retain(|b| !b.leaving);
        let mut items: Vec<(f64, f64, f64)> = bubbles.iter().map(|b| (b.x, b.y, b.target.max(least))).collect();
        for _ in 0..600 {
            if settle(&mut items) < REST / 10.0 {
                break;
            }
        }
        for (b, (x, y, r)) in bubbles.iter_mut().zip(items) {
            (b.x, b.y) = (x, y);
            if !self.moving {
                b.r = r;
                b.heat = b.heat_to;
            }
        }
        drop(bubbles);
        if !self.moving {
            for s in self.threads.borrow_mut().iter_mut() {
                s.0 = s.1;
            }
        }
    }

    #[cfg(test)]
    pub fn finish(&self) {
        let least = self.smallest();
        for b in self.bubbles.borrow_mut().iter_mut() {
            b.r = b.target.max(least);
            b.heat = b.heat_to;
        }
        for s in self.threads.borrow_mut().iter_mut() {
            s.0 = s.1;
        }
    }

    pub fn wake(&self) {
        if !self.moving {
            self.redraw();
            return;
        }
        self.run(FRAME_MS);
    }

    fn run(&self, every: u64) {
        let pace = self.pace.get();
        if pace != 0 && pace <= every {
            return;
        }
        if pace == 0 {
            self.last.set(None);
        }
        self.pace.set(every);
        let no = self.run_no.get() + 1;
        self.run_no.set(no);
        let weak = self.me.borrow().clone();
        self.events.every(Duration::from_millis(every), move || {
            let Some(o) = weak.upgrade() else { return Flow::Stop };
            if o.run_no.get() != no {
                return Flow::Stop;
            }
            let next = o.frame();
            o.redraw();
            match next {
                Some(n) if n == every => Flow::Continue,
                next => {
                    o.pace.set(0);
                    if let Some(n) = next {
                        o.run(n);
                    }
                    Flow::Stop
                }
            }
        });
    }

    fn frame(&self) -> Option<u64> {
        let now = Instant::now();
        let dt = self.last.replace(Some(now)).map_or(FRAME_MS as f64 / 1000.0, |t| (now - t).as_secs_f64()).min(0.1);
        let ease = 1.0 - (-dt * 6.0).exp();
        let least = self.smallest();
        let mut busy = false;
        let mut moved = 0.0;
        for s in self.threads.borrow_mut().iter_mut() {
            let (step, now) = toward(s.0, s.1, ease, 0.02);
            s.0 = now;
            moved += step;
        }
        let mut bubbles = self.bubbles.borrow_mut();
        for b in bubbles.iter_mut() {
            let to = if b.leaving { 0.0 } else { b.target.max(least) };

            let (step, r) = toward(b.r, to, ease, 0.003);
            b.r = r;
            moved += step;
            let (step, heat) = toward(b.heat, b.heat_to, ease, 0.02);
            b.heat = heat;
            moved += step;
            if b.threads >= BEAT && !b.leaving {
                b.phase = (b.phase + dt * TAU * beat(b.heat)) % TAU;
                busy = true;
            }
        }
        bubbles.retain(|b| !(b.leaving && b.r <= 0.0));
        let mut items: Vec<(f64, f64, f64)> = bubbles.iter().map(|b| (b.x, b.y, b.r)).collect();
        moved += settle(&mut items);
        for (b, (x, y, _)) in bubbles.iter_mut().zip(items) {
            (b.x, b.y) = (x, y);
        }
        if moved > REST {
            Some(FRAME_MS)
        } else if busy {
            Some(BEAT_MS)
        } else {
            None
        }
    }
}
