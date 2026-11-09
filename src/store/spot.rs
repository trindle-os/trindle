//! Bringing the page to the card of Updates that was asked for, and lighting its edge.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::toolkit::event::Flow;
use crate::toolkit::motion::{tween, Ease};
use crate::toolkit::ui::Shown;
use super::{Do, Store, View};

pub(super) const SPOT: &str = "firmware";
pub(super) const SPOT_PLUGINS: &str = "plugins";

const SPOT_LIT: Duration = Duration::from_millis(2500);
const SPOT_EASE_MS: u64 = 700;

const SPOT_KEPT: Duration = Duration::from_secs(5);

const SPOT_STEP: f32 = 0.2;

impl Store {

    fn spot_wanted(&self) -> Option<bool> {
        if self.view != View::Updates || self.detail.is_some() || (self.spot_target() == SPOT && self.firmware.is_empty()) {
            return None;
        }
        if matches!(self.pending_spot.as_deref(), Some(SPOT | SPOT_PLUGINS)) {
            return Some(true);
        }
        self.spot_kept.filter(|t| *t > Instant::now()).map(|_| false)
    }

    fn spot_target(&self) -> &'static str {
        match self.pending_spot.as_deref() {
            Some(SPOT_PLUGINS) => SPOT_PLUGINS,
            Some(SPOT) => SPOT,
            _ => self.spot_last,
        }
    }

    fn spot_lit(&mut self) {
        self.spot_last = self.spot_target();
        self.pending_spot = None;
        self.spot_kept = Some(Instant::now() + SPOT_KEPT);
        let (asks, back) = (self.asks.clone(), self.asks.clone());
        let lp = self.events.clone();
        if !crate::common::graphics::moving() {
            self.lit = 1.0;
            self.events.after(SPOT_LIT, move || { back.send(Do::Lit(0.0)); });
            return;
        }
        tween(&self.events, SPOT_EASE_MS, Ease::Out, move |t| { asks.send(Do::Lit(t)); }, || {});
        self.events.after(SPOT_LIT, move || {
            tween(&lp, SPOT_EASE_MS, Ease::Out, move |t| { back.send(Do::Lit(1.0 - t)); }, || {});
        });
    }
}

pub(super) fn bring_into_view(shown: &Rc<Shown<Store>>) {
    let Some(light) = shown.with(Store::spot_wanted) else { return };

    let key = shown.with(Store::spot_target);
    let Some((area, from, aim)) = shown.aim_at(key, 8.0) else { return };
    if light {
        shown.change(Store::spot_lit);
    }
    if !crate::common::graphics::moving() || (aim - from).abs() < 1.0 {
        shown.scroll_area(&area, aim);
        return;
    }
    let (w, now) = (Rc::downgrade(shown), Cell::new(from));
    let events = shown.with(|s| s.events.clone());
    events.every(Duration::from_millis(16), move || {
        let Some(shown) = w.upgrade() else { return Flow::Stop };
        let at = now.get();
        if (aim - at).abs() < 1.0 {
            shown.scroll_area(&area, aim);
            return Flow::Stop;
        }
        now.set(at + (aim - at) * SPOT_STEP);
        shown.scroll_area(&area, now.get());
        Flow::Continue
    });
}
