//! The power page.

use std::rc::Rc;
use std::time::Duration;

use crate::ring::{Job, kept, quick};
use crate::ring::ctx::{Ctx, msg, Msg};
use crate::ring::layout::CENTRE;
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, Node};

use super::{ALL, Face, REFUSED_SECONDS};
use super::middle::{band, core};
use super::panel::session_end;

pub(super) fn can_hibernate(ctx: &Rc<Ctx>, done: impl FnOnce(bool) + 'static) {
    let Some(bus) = &ctx.system else { return done(false) };
    bus.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
        .method("CanHibernate", &(), false, Some(Duration::from_secs(5)), &ctx.lp, move |r| {
            let said = r.and_then(|m| m.body().deserialize::<String>().map_err(Into::into));
            done(matches!(said.as_deref(), Ok("yes")));
        });
}

pub(super) fn only_a_click_confirms() -> &'static str {
    t!("ring-power-only-click")
}

pub(super) fn only_your_own_power() -> &'static str {
    t!("ring-power-only-own")
}

pub(super) fn power_closed() -> &'static str {
    t!("ring-power-closed")
}

const DISARM_SECONDS: u64 = crate::common::ui::DISARM_SECONDS as u64;

pub(super) const OFFER_SECONDS: u64 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Power {
    Refuse,
    Arm,
    NeedsClick,
    Run,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Ends {
    Power,
    WindowMenu,
    Menu,
    Quick,
    Typed,
    Ring,
}

pub(super) fn escape_ends(power: bool, window_menu: bool, menu: bool, quick: bool, typed: bool) -> Ends {
    if power {
        Ends::Power
    } else if window_menu {
        Ends::WindowMenu
    } else if menu {
        Ends::Menu
    } else if quick {
        Ends::Quick
    } else if typed {
        Ends::Typed
    } else {
        Ends::Ring
    }
}

pub(super) fn power_step(what: &str, armed: Option<&str>, own: bool) -> Power {
    if !ends_session(what) {
        Power::Run
    } else if !own {
        Power::Refuse
    } else if armed != Some(what) {
        Power::Arm
    } else {
        Power::Run
    }
}

pub(super) fn spoken_step(step: Power, offered: bool, clicked: bool) -> Power {
    if step == Power::Run && offered && !clicked { Power::NeedsClick } else { step }
}

pub(super) fn ends_session(what: &str) -> bool {
    !matches!(what, "lock" | "screensaver")
}

impl Face {
    pub(super) fn show_power(&mut self, on: bool) {
        self.power = on;
        self.armed = None;
        self.offered = false;
        if on {
            self.quick.hide();
            self.close_menu();
            self.close_beside();
            self.ask_hibernate();
        } else {
            self.hover_app = None;
        }
    }

    pub(super) fn ask_hibernate(&self) {
        let ctx = self.ctx.clone();
        can_hibernate(&self.ctx, move |yes| ctx.face(move |f| f.can_hibernate = yes));
    }

    fn power_choice(&mut self, what: &str) {
        let step = power_step(what, self.armed.as_deref(), kept::own_press());
        match spoken_step(step, self.offered, kept::clicked()) {
            Power::Refuse => {
                self.armed = None;
                self.say(only_your_own_power(), REFUSED_SECONDS);
            }
            Power::NeedsClick => self.say(only_a_click_confirms(), REFUSED_SECONDS),
            Power::Arm => self.arm(what, DISARM_SECONDS),
            Power::Run => {
                self.armed = None;
                match what {
                    "lock" => self.run(&["trindle-lock", "--now"]),
                    "screensaver" => self.run(&["trindle-screensaver"]),
                    _ => {
                        self.close();
                        session_end(&self.ctx, what);
                    }
                }
            }
        }
    }

    fn arm(&mut self, what: &str, seconds: u64) {
        self.armed = Some(what.to_string());
        self.offered = false;
        self.arm_turn += 1;
        let (ctx, turn) = (self.ctx.clone(), self.arm_turn);
        self.ctx.lp.after(Duration::from_secs(seconds), move || {
            ctx.face(move |f| if f.arm_turn == turn { f.armed = None; f.offered = false })
        });
    }

    pub(super) fn confirm(&mut self, what: &str, words: &str) {
        self.menu = None;
        self.show_power(true);
        self.arm(what, OFFER_SECONDS);
        self.offered = true;
        self.say(words, OFFER_SECONDS as u32);
    }

    pub(super) fn escape(&mut self) {
        match escape_ends(self.power, self.win_menu.is_some(), self.menu.is_some() || self.beside.is_some(), self.quick.is_open(), self.level == ALL && !self.typed.is_empty()) {
            Ends::Power => {
                self.win_menu = None;
                self.show_power(false);
            }
            Ends::WindowMenu => self.win_menu = None,
            Ends::Menu => {
                self.close_menu();
                self.close_beside();
            }
            Ends::Quick => self.quick_done(),
            Ends::Typed => self.set_typed(String::new()),
            Ends::Ring => self.close(),
        }
    }

    pub(super) fn power_cover(&self) -> Node<Msg> {
        let mut away = vec![("system-lock-screen-symbolic", t!("ring-power-lock"), "lock"),
                            ("preferences-desktop-screensaver-symbolic", t!("ring-power-screensaver"), "screensaver"),
                            ("system-suspend-symbolic", t!("ring-power-suspend"), "Suspend")];
        if self.can_hibernate {
            away.push(("system-hibernate-symbolic", t!("ring-power-hibernate"), "Hibernate"));
        }
        let leaving = [("system-log-out-symbolic", t!("ring-power-log-out"), "logout"),
                       ("system-reboot-symbolic", t!("ring-power-restart"), "Reboot"),
                       ("system-shutdown-symbolic", t!("ring-power-shut-down"), "PowerOff")];
        let make = |list: &[(&str, &str, &'static str)]| -> Node<Msg> {
            band(list.iter().map(|&(icon, words, what)| {
                let armed = self.armed.as_deref() == Some(what);
                let said = if armed { crate::common::ui::click_again(words) } else { words.to_string() };
                let b = button(if armed { crate::common::ui::TICK_ICON } else { icon }, said.clone())
                    .size(core())
                    .key(format!("power:{what}"))
                    .spoken(said)
                    .on_press(msg(move |f| f.power_choice(what)));
                if armed { b.filled() } else { b.tinted(Job::Computer.share()) }
            }).collect())
        };
        let close = button("window-close-symbolic", t!("ring-close")).size(core()).tinted(Job::Computer.share()).on_press(msg(|f| f.show_power(false)));
        let gap = crate::toolkit::theme::sheet().ring_band_gap;
        quick::on_disc(column(vec![make(&away), make(&leaving), band(vec![close])]).gap(gap).centred(), Some(CENTRE as f32))
    }
}
