//! "One moment, please": shown in the middle of the screen while the desktop
//! is still coming up, and gone the moment it has.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::toolkit::t;
use crate::common::filling;
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{card, column, label, ring, Length, Node};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

const RING: &str = "org.trindle.Ring";

const SETTLE: Duration = Duration::from_millis(900);
const LONGEST: Duration = Duration::from_secs(90);

const SHOW_AFTER: Duration = Duration::from_millis(250);

const RING_SIZE: f32 = 44.0;
const EXPECTED_SECS: f64 = 8.0;
const TICK: Duration = Duration::from_millis(250);
const FADE: Duration = Duration::from_millis(300);
const FRAME: Duration = Duration::from_millis(16);
const CARD_WIDTH: f32 = 220.0;

const MARGIN: f32 = 16.0;

struct Words {
    share: f32,
}

impl App for Words {
    type Msg = ();
    fn update(&mut self, _: ()) {}
    fn view(&self) -> Node<()> {
        column(vec![
            card(column(vec![ring(self.share, RING_SIZE), label(t!("session-one-moment"))]).centred().gap(14.0)).floats()
                .width(Length::Fixed(CARD_WIDTH)),
        ]).padding(MARGIN).centred()
    }
}

struct Starting {
    events: Loop,
    display: Rc<Display>,
    shown: RefCell<Option<Shown<Words>>>,
    began: Instant,

    over: Cell<bool>,

    seen: Cell<f32>,
    fading: Cell<u32>,
}

pub fn main(_args: Vec<String>) -> i32 {
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };
    let me = Rc::new(Starting {
        events: events.clone(), display, shown: RefCell::new(None), began: Instant::now(),
        over: Cell::new(false), seen: Cell::new(0.0), fading: Cell::new(0),
    });

    // never shown at all.
    let bus = Bus::session(&events).ok();
    let m = me.clone();
    let _listen = bus.as_ref().and_then(|bus| bus.watch(RING, &events, move |_| {
        if m.shown.borrow().is_none() && m.began.elapsed() < SHOW_AFTER {
            m.finish(false);
            return;
        }
        let m = m.clone();
        m.events.clone().after(SETTLE, move || m.finish(true));
    }, || {}).ok());
    let m = me.clone();
    events.after(LONGEST, move || m.finish(false));
    let m = me.clone();
    events.after(SHOW_AFTER, move || m.show());
    events.run();
    0
}

impl Starting {
    fn show(self: &Rc<Self>) {
        if self.over.get() {
            return;
        }
        let kind = Kind::Panel(Panel {
            layer: Layer::Overlay, edges: Edges::NONE, keep: -1, margins: [0; 4], keys: Keys::Never,
            name: "trindle-starting".into(), screen: None,
        });
        let width = (CARD_WIDTH + 2.0 * MARGIN) as u32;
        let Some(shown) = Shown::open(&self.events, &self.display, kind, (width, 1), Words { share: self.by_clock() }) else {
            self.events.quit();
            return;
        };
        shown.see_through();
        shown.set_shown(0.0);
        shown.fit();
        *self.shown.borrow_mut() = Some(shown);
        self.fade(true, || {});
        let me = self.clone();
        self.events.every(TICK, move || {
            if me.over.get() {
                return Flow::Stop;
            }
            let share = me.by_clock();
            me.set_share(share);
            Flow::Continue
        });
    }

    fn by_clock(&self) -> f32 {
        filling::how_far(None, None, self.began.elapsed().as_secs_f64(), EXPECTED_SECS) as f32
    }

    fn set_share(&self, share: f32) {
        if let Some(shown) = &*self.shown.borrow() {
            shown.change(|w| w.share = w.share.max(share));
        }
    }

    fn fade(self: &Rc<Self>, inwards: bool, done: impl FnOnce() + 'static) {
        let this = self.fading.get() + 1;
        self.fading.set(this);
        let (from, start, me) = (self.seen.get(), Instant::now(), self.clone());
        let mut done = Some(done);
        self.events.every(FRAME, move || {
            if me.fading.get() != this {
                return Flow::Stop;
            }
            let t = (start.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.0);
            let seen = if inwards { from + (1.0 - from) * t } else { from * (1.0 - t) };
            me.seen.set(seen);
            if let Some(shown) = &*me.shown.borrow() {
                shown.set_shown(seen);
            }
            if t < 1.0 {
                return Flow::Continue;
            }
            if let Some(done) = done.take() {
                done();
            }
            Flow::Stop
        });
    }

    fn finish(self: &Rc<Self>, up: bool) {
        if self.over.replace(true) {
            return;
        }
        if self.shown.borrow().is_none() {
            self.events.quit();
            return;
        }

        let me = self.clone();
        let fade_out = move || {
            let events = me.events.clone();
            me.fade(false, move || events.quit());
        };

        if up {
            self.set_share(1.0);
            self.events.after(filling::JOINED, fade_out);
        } else {
            fade_out();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;

    #[test]
    fn the_card_fits_its_window() {
        let ui = Ui::new(Words { share: 0.3 }.view(), Theme::chosen());
        let width = CARD_WIDTH + 2.0 * MARGIN;
        let h = ui.natural_height(width);

        assert!(h > 2.0 * MARGIN + RING_SIZE + 14.0 + 15.0, "{h}");
        assert!(h < 2.0 * MARGIN + RING_SIZE + 14.0 + 120.0, "{h}");
    }

    #[test]
    fn one_moment_please_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        drawn::none(drawn::in_every_look("One moment, please", 1, || {
            for share in [0.3, 1.0] {
                drawn::window(&format!("filled {share}"), Words { share }.view(), Theme::chosen(), ((CARD_WIDTH + 2.0 * MARGIN) as u32, 160), true);
            }
        }));
    }
}
