//! Drives as a Toolkit app: its messages, its drawing and its keys.

use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::ui::App;
use crate::toolkit::widget::{Node, Rect, layers};
use crate::toolkit::{popup, t};
use super::{Do, Drives, Msg, help, plan, spot};
use super::steps::Said;
use super::steps::STANDING_TRIES;

impl Drives {
    pub(super) fn handle(&mut self, d: Do) {
        match d {
            Do::Objects(o) => self.got(o),
            Do::Cap { kind, method, yes } => self.cap(&kind, method, yes),
            Do::Done { drive, row, res, standing } => {
                self.working.remove(&spot(&drive, &row));
                match (res, standing) {
                    (Ok(w), false) => self.say(&drive, &row, w, true),
                    (Err(e), false) => self.say(&drive, &row, e, false),

                    (Ok(said), true) | (Err(said), true) => {
                        self.say(&drive, &row, plan::standing_words(&said), false);
                        self.keep_taking_back(drive, row, said, STANDING_TRIES);
                    }
                }
                self.finish();
            }
            Do::TookBack { drive, row, said, left, done } => self.took_back(drive, row, said, left, done),
            Do::Carry(carried) => self.carry_folder(carried),
            Do::Snapped { drive, vol, row, want, desc, res } => self.snapped(&drive, &vol, &row, want, &desc, res),
            Do::CarryKnown { carried, uuid } => self.carry_known(carried, &uuid),
            Do::Soon(n) => {
                if n == self.soon && self.busy == 0 {
                    self.refresh();
                }
            }
            Do::Changed => self.soon(),
            Do::Refresh => self.refresh(),
            Do::Gone => self.missing = Some((t!("disks-missing-service"), "udisks2")),
            Do::Disarm(n) => {
                if n == self.arms {
                    self.armed = None;
                }
            }
            Do::Redraw => self.said.retain(Said::fresh),
            Do::Show => {}
            Do::Close => self.close(),
        }
    }
}

impl App for Drives {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Toggle(k) => {
                self.help = false;
                if self.open.as_deref() == Some(k.as_str()) {
                    self.open = None;
                } else {
                    self.open = Some(k);
                    self.sheet_at = self.pressed_at;
                }
            }
            Msg::Help => {
                self.help = !self.help;
                self.help_at = self.pressed_at;
                if self.help {
                    self.open = None;
                }
            }
            Msg::More(page) => help::open(page),
            Msg::Idle => {}
            Msg::Retry => self.refresh(),
            Msg::Text(k, t) => {
                self.texts.insert(k, t);
            }
            Msg::Pick(k, i) => {
                self.picks.insert(k, i);
            }
            Msg::Gb(k, v) => {
                self.gbs.insert(k, v);
            }
            Msg::Startup { drive, vol, on } => self.startup(&drive, &vol, on),
            Msg::Hide { drive, row, key, hide } => self.hide(&drive, &row, &key, hide),
            Msg::Act(a) => self.act(a),
            Msg::Weigh => {
                if self.loaded {
                    self.weigh();
                }
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let mut page = self.page();
        if self.help {
            page = popup::dismissing(page, self.help_panel(), Msg::Help, Msg::Idle);
        }
        match self.sheet() {
            Some(sheet) => layers(vec![page, sheet]),
            None => page,
        }
    }

    fn focused(&self) -> Option<Msg> {
        Some(Msg::Weigh)
    }

    fn pressed(&mut self, place: Rect) {
        self.pressed_at = place;
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::Key { sym: Keysym::Escape, .. } if self.help => Some(Msg::Help),
            Input::Key { sym: Keysym::Escape, .. } => self.open.clone().map(Msg::Toggle),
            _ => None,
        }
    }
}
