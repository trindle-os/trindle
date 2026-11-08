//! Low-battery warning card, also shown briefly when the charger is plugged or unplugged.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use super::ctx::Ctx;
use crate::common::files::setting;
use crate::settings::ui::icon_in;
use crate::toolkit::bus::{self, Bus, Listen};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::text::Style;
use crate::toolkit::theme::{self, Theme};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::tiles::standard_ring_with;
use crate::toolkit::widget::{button, card, column, drawing, layers, placed, row, Length, Node};
use crate::toolkit::window::{Edges, Keys, Kind, Layer, Panel};
use crate::toolkit::t;

const UPOWER: &str = "org.freedesktop.UPower";
const DEVICE: &str = "/org/freedesktop/UPower/devices/DisplayDevice";
const PROPS: &str = "org.freedesktop.DBus.Properties";

const ICON_SIZE: f32 = 64.0;

const MIDDLE_ICON: f32 = 24.0;
const CARD_GAP: f32 = 24.0;
const WORDS_GAP: f32 = 4.0;
const TITLE_SIZE: f32 = 30.0;
const BODY_SIZE: f32 = 18.0;

const INSET: f32 = 8.0;

const WORDS_WIDTH: f32 = 300.0;

const MARGIN: f32 = 32.0;

const LINE: f32 = 1.3;
const FADE: Duration = Duration::from_millis(200);
const FRAME: Duration = Duration::from_millis(16);
const CHARGER_SECONDS: u64 = 4;
const POLL: Duration = Duration::from_secs(60);

pub fn level() -> u32 {
    setting("battery_warn").parse().unwrap_or(10)
}

struct Card {
    lp: Loop,
    alert: Weak<BatteryAlert>,
    icon: String,
    title: String,
    sub: String,

    share: f32,
}

#[derive(Clone)]
enum Msg {
    Close,
}

impl App for Card {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Close => {
                let alert = self.alert.clone();
                self.lp.after(Duration::ZERO, move || {
                    if let Some(a) = alert.upgrade() {
                        a.hide();
                    }
                });
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let words = column(vec![
            line(&self.title, TITLE_SIZE, 700, |t| t.fg),
            line(&self.sub, BODY_SIZE, 400, |t| t.muted),
        ]).gap(WORDS_GAP);
        let inside = row(vec![
            layers(vec![
                standard_ring_with(Some(self.share), None, ICON_SIZE),
                placed(icon_in(self.icon.clone(), MIDDLE_ICON, |t| t.fg), |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0)),
            ]).width(Length::Fixed(ICON_SIZE)).height(Length::Fixed(ICON_SIZE)),
            words,
            button("window-close-symbolic", t!("ring-close")).on_press(Msg::Close),
        ]).gap(CARD_GAP).padding(INSET).centred();
        column(vec![card(inside).floats()]).padding(MARGIN).centred()
    }
}

fn line<M>(text: &str, size: f32, weight: u16, colour: fn(&Theme) -> Colour) -> Node<M> {
    let height = (size * LINE).ceil();
    let text = text.to_string();
    let copied = text.clone();
    drawing((WORDS_WIDTH, height), text.clone(), move |canvas, pen| {
        let s = pen.scale;
        let style = Style { weight, ..Style::new((size * s).round()) };
        let full = pen.fonts.layout(&text, &style, None);
        let dots = pen.fonts.layout("\u{2026}", &style, None);
        let shown = full.cut(WORDS_WIDTH * s, &dots);
        let y = ((height * s - shown.height) / 2.0).max(0.0).round();
        pen.fonts.draw(canvas, &shown, 0.0, y, colour(pen.theme));
    }).copies(copied).width(Length::Fixed(WORDS_WIDTH))
}

struct BatteryAlert {
    ctx: Rc<Ctx>,
    me: Weak<BatteryAlert>,
    shown: RefCell<Option<Shown<Card>>>,
    warned: Cell<bool>,

    warning: Cell<bool>,

    plugged: Cell<Option<bool>>,

    hide_timer: Cell<u32>,
    seen: Cell<f32>,
    fading: Cell<u32>,
    _listens: RefCell<Vec<Listen>>,
}

pub fn start(ctx: &Rc<Ctx>) {
    let me = Rc::new_cyclic(|me| BatteryAlert {
        ctx: ctx.clone(), me: me.clone(), shown: RefCell::new(None),
        warned: Cell::new(false), warning: Cell::new(false), plugged: Cell::new(None),
        hide_timer: Cell::new(0), seen: Cell::new(0.0), fading: Cell::new(0), _listens: RefCell::new(Vec::new()),
    });
    if let Some(bus) = &ctx.system {
        let mut listens = Vec::new();

        for path in [DEVICE, "/org/freedesktop/UPower"] {
            let w = Rc::downgrade(&me);
            if let Ok(l) = bus.listen(Some(UPOWER), PROPS, Some("PropertiesChanged"), Some(path), move |_| {
                if let Some(a) = w.upgrade() {
                    a.check();
                }
            }) {
                listens.push(l);
            }
        }
        *me._listens.borrow_mut() = listens;
    }

    let a = me.clone();
    ctx.lp.every(POLL, move || {
        a.check();
        Flow::Continue
    });
    me.check();
}

impl BatteryAlert {
    fn bus(&self) -> Option<Bus> {
        self.ctx.system.clone()
    }

    fn check(&self) {
        let Some(bus) = self.bus() else { return };
        let w = self.me.clone();
        bus.call(UPOWER, DEVICE, "org.freedesktop.UPower.Device").get_all(&self.ctx.lp, move |r| {
            let (Some(a), Ok(p)) = (w.upgrade(), r) else { return };
            if !bus::get_bool(&p, "IsPresent") {
                return;
            }
            let pct = bus::get_f64(&p, "Percentage").round() as u32;

            let state = bus::get_u32(&p, "State");
            let on_charger = !crate::common::charge::on_battery();
            let running_down = matches!(state, 2 | 6) && !on_charger;
            let warned = a.decide(pct, running_down, level());
            let plugged = (state != 0 || on_charger).then_some(on_charger);
            if let Some(now) = plugged {
                let before = a.plugged.replace(Some(now));

                if before.is_some_and(|b| b != now) {
                    crate::session::idle::restart();
                }

                if before.is_some_and(|b| b != now) && !warned {
                    a.charger(now, pct);
                }
            }
        });
    }

    fn decide(&self, pct: u32, running_down: bool, level: u32) -> bool {
        match what_to_do(pct, running_down, level, self.warned.get()) {
            Show::Warn => {
                self.warned.set(true);
                self.show("battery-caution-symbolic", &t!("ring-battery-at", pct = pct), t!("ring-battery-plug-in"), true, pct);
                crate::common::sounds::play("battery");
                return true;
            }
            Show::Clear => {
                self.warned.set(false);
                if self.warning.get() {
                    self.hide();
                }
            }
            Show::Leave => {}
        }
        false
    }

    fn charger(&self, plugged: bool, pct: u32) {
        crate::common::sounds::play(if plugged { "plug" } else { "unplug" });
        if plugged {
            self.show("battery-full-charging-symbolic", t!("ring-battery-charging"), &t!("ring-battery-at", pct = pct), false, pct);
        } else {
            self.show("battery-good-symbolic", t!("ring-battery-unplugged"), &t!("ring-battery-running", pct = pct), false, pct);
        }
        let this = self.hide_timer.get() + 1;
        self.hide_timer.set(this);
        let w = self.me.clone();
        self.ctx.lp.after(Duration::from_secs(CHARGER_SECONDS), move || {
            let Some(a) = w.upgrade() else { return };
            if a.hide_timer.get() == this && !a.warning.get() {
                a.hide();
            }
        });
    }

    fn show(&self, icon: &str, title: &str, sub: &str, warning: bool, pct: u32) {
        if warning {
            self.hide_timer.set(self.hide_timer.get() + 1);
        }
        self.warning.set(warning);
        let card = Card {
            lp: self.ctx.lp.clone(), alert: self.me.clone(),
            icon: icon.to_string(), title: title.to_string(), sub: sub.to_string(), share: pct as f32 / 100.0,
        };
        let opened = self.shown.borrow().is_some();
        if opened {
            if let Some(shown) = &*self.shown.borrow() {
                shown.change(|c| *c = card);
                shown.window.set_visible(true);
                shown.fit();
            }
        } else {
            let kind = Kind::Panel(Panel {

                layer: Layer::Overlay, edges: Edges::NONE, keep: -1, margins: [0; 4], keys: Keys::Never,
                name: "trindle-battery".into(), screen: None,
            });
            let width = (ICON_SIZE + WORDS_WIDTH + theme::icon_button() + 2.0 * CARD_GAP
                + 2.0 * (INSET + theme::sheet().card_padding + MARGIN)).ceil() as u32;
            let Some(shown) = Shown::open(&self.ctx.lp, &self.ctx.display, kind, (width, 1), card) else { return };
            shown.see_through();
            shown.set_shown(0.0);
            shown.fit();
            self.seen.set(0.0);
            *self.shown.borrow_mut() = Some(shown);
        }
        self.fade(true);
    }

    fn hide(&self) {
        self.warning.set(false);
        if self.shown.borrow().is_some() {
            self.fade(false);
        }
    }

    fn fade(&self, inwards: bool) {
        let this = self.fading.get() + 1;
        self.fading.set(this);
        let (from, began, w) = (self.seen.get(), Instant::now(), self.me.clone());
        self.ctx.lp.every(FRAME, move || {
            let Some(a) = w.upgrade() else { return Flow::Stop };
            if a.fading.get() != this {
                return Flow::Stop;
            }
            let t = (began.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.0);
            let seen = if inwards { from + (1.0 - from) * t } else { from * (1.0 - t) };
            a.seen.set(seen);
            if let Some(shown) = &*a.shown.borrow() {
                shown.set_shown(seen);
                if t >= 1.0 && !inwards {
                    shown.window.set_visible(false);
                }
            }
            if t < 1.0 { Flow::Continue } else { Flow::Stop }
        });
    }
}

#[derive(Debug, PartialEq)]
enum Show {
    Warn,
    Clear,
    Leave,
}

fn what_to_do(pct: u32, running_down: bool, level: u32, warned: bool) -> Show {
    if level == 0 || !running_down || pct > level + 5 {
        return Show::Clear;
    }
    if pct <= level && !warned { Show::Warn } else { Show::Leave }
}

#[cfg(test)]
mod tests {
    use super::{what_to_do, Show};

    #[test]
    fn warns_once_and_resets_on_charging() {
        assert_eq!(what_to_do(30, true, 10, false), Show::Clear);
        assert_eq!(what_to_do(10, true, 10, false), Show::Warn);
        assert_eq!(what_to_do(8, true, 10, true), Show::Leave);
        assert_eq!(what_to_do(8, false, 10, true), Show::Clear);
        assert_eq!(what_to_do(9, true, 0, false), Show::Clear);
    }

    #[test]
    fn the_batterys_card_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("the battery's card", 4, || {
            for (icon, title, sub, share) in [("battery-caution-symbolic", "Battery low", "10% left. Plug the charger in soon.", 0.1),
                                              ("battery-good-symbolic", "Charger unplugged", "Running on the battery, at 100%", 1.0)] {
                let card = super::Card {
                    lp: crate::toolkit::event::Loop::new(), alert: std::rc::Weak::new(), icon: icon.into(),
                    title: title.into(), sub: sub.into(), share,
                };
                drawn::window(title, crate::toolkit::ui::App::view(&card), Theme::chosen(), (640, 220), true);
            }
        }));
    }

    #[test]
    #[ignore]
    fn shoot_ring_battery() {
        let card = super::Card {
            lp: crate::toolkit::event::Loop::new(), alert: std::rc::Weak::new(), icon: "battery-caution-symbolic".into(),
            title: "Charger unplugged".into(), sub: "Running on the battery, at 100%".into(), share: 0.1,
        };
        crate::ring::shots::save("battery-alert", crate::toolkit::ui::App::view(&card), (560, 200));
    }
}
