//! Speaking to the ring: listening to the microphone and doing what was heard.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::common::run::{capture, spawn};
use crate::ring::{tray, updates};
use crate::ring::ctx::Program;
use crate::ring::windows::Window;
use crate::toolkit::t;
use crate::toolkit::event::Flow;
use crate::voice::commands;

use super::{ALL, Face, FAVOURITES};

const PULSE_MS: u64 = 40;

const ANSWER_SECONDS: u32 = 6;

impl Face {
    pub(super) fn listen(&mut self, seconds: u32) {
        if self.listening {
            return;
        }
        let listener = Rc::new(crate::voice::Listener::new(&self.ctx.lp));
        if let Err(e) = listener.start() {
            return self.say(&e, ANSWER_SECONDS);
        }
        let ctx = self.ctx.clone();
        listener.when_finished(move || ctx.face(|f| f.finish_listening()));
        self.listener = Some(listener);
        self.listening = true;
        self.listen_turn += 1;
        self.say(t!("ring-listening"), 0);
        let (ctx, turn) = (self.ctx.clone(), self.listen_turn);
        self.ctx.lp.after(Duration::from_secs(seconds as u64), {
            let ctx = ctx.clone();
            move || ctx.face(move |f| if f.listen_turn == turn && f.listening { f.finish_listening() })
        });
        let alive = Rc::new(Cell::new(true));
        self.ctx.lp.every(Duration::from_millis(PULSE_MS), move || {
            if !alive.get() {
                return Flow::Stop;
            }
            let a = alive.clone();
            ctx.face(move |f| {
                if !f.listening || f.listen_turn != turn {
                    a.set(false);
                }
                f.pulse = (f.pulse + 0.04) % 1.0;
            });
            Flow::Continue
        });
    }

    pub(super) fn stop_listening(&mut self) {
        self.listening = false;
        self.words = None;
        if let Some(l) = self.listener.take() {
            l.stop_now();
        }
    }

    fn finish_listening(&mut self) {
        self.listening = false;
        let Some(l) = self.listener.take() else { return };
        self.say(t!("ring-working-out"), 0);
        let hints: Vec<String> = self.ctx.apps().iter().take(80).map(|a| a.name.clone()).collect();
        let ctx = self.ctx.clone();
        l.finish(hints, move |r| {
            ctx.face(move |f| match r {
                Err(e) => f.say(&e, ANSWER_SECONDS),
                Ok(text) => f.heard(&text),
            })
        });
    }

    pub(super) fn heard(&mut self, text: &str) {
        let all = self.ctx.apps();
        let named: Vec<commands::Named> = all.iter().map(|a| commands::Named { name: &a.name, id: &a.id }).collect();
        let (action, mut words) = commands::interpret(text, &named);
        use commands::Action as A;
        let Some(action) = self.heard_and_done(action, &all, &words) else { return };
        match action {
            A::Close(target) => self.heard_close(target, &all, &mut words),
            A::Level(l) => self.set_level(if l > 0 { ALL } else { FAVOURITES }),
            A::Updates => self.heard_updates(),
            other => heard_of_the_computer(other, &mut words),
        }
        self.say(&words, 5);
    }

    fn heard_and_done(&mut self, action: commands::Action, all: &[Program], words: &str) -> Option<commands::Action> {
        use commands::Action as A;
        match action {
            A::Open(i) | A::Switch(i) => self.launch(&all[i]),
            A::Screenshot => self.run_after_closing(&["trindle-screenshot", "--whole"]),
            A::Settings(p) => self.settings(p),
            A::Store(q) => match q {
                Some(q) => self.open_own("store", &["--search", &q]),
                None => self.open_own("store", &[]),
            },
            A::Confirm(what) => self.confirm(what, words),
            A::Tool("magnifier") => self.toggle_magnifier(None),
            A::Tool("activity") => self.open_own("activity", &[]),
            A::Tool(panel) => self.quick.show(panel),
            A::Lock => self.run(&["trindle-lock", "--now"]),
            other => return Some(other),
        }
        None
    }

    fn heard_close(&mut self, target: Option<usize>, all: &[Program], words: &mut String) {
        let wins: Vec<Window> = match target {
            Some(i) => self.windows_of(&all[i]),
            None => self.windows.as_ref().map(|w| w.windows().into_iter().filter(|x| x.active).take(1).collect()).unwrap_or_default(),
        };
        if let Some(ws) = &self.windows {
            for w in &wins {
                ws.close(w);
            }
        }
        if wins.is_empty() {
            *words = t!("ring-nothing-to-close").into();
        }
    }

    fn heard_updates(&mut self) {
        self.updates.notice();
        if let Some(other) = self.updates.other() {

            self.ctx.face(move |f| f.say(other.sentence(), ANSWER_SECONDS));
        } else {
            let ctx = self.ctx.clone();

            updates::waiting(&self.ctx, None, crate::common::update_checks::may_ask_network(true, false), move |w, _| {
                let words = w.spoken();
                ctx.face(move |f| f.say(&words, ANSWER_SECONDS));
            });
        }
    }
}

fn heard_of_the_computer(action: commands::Action, words: &mut String) {
    use commands::Action as A;
    match action {
        A::Volume(step) => capture(&["wpctl", "set-volume", "-l", "1.0", "@DEFAULT_AUDIO_SINK@", step], None, |_, _, _| {}),
        A::Mute(on) => capture(&["wpctl", "set-mute", "@DEFAULT_AUDIO_SINK@", if on { "1" } else { "0" }], None, |_, _, _| {}),
        A::Wallpaper => {
            if !spawn(&["trindle-wallpaper", "--next"]) {
                *words = t!("ring-could-not-open").into();
            }
        }
        A::Time => {
            let now = crate::common::clock::Local::now();
            *words = t!("ring-voice-time", time = now.line(crate::toolkit::lang::format::Pattern::Clock), date = now.line(crate::toolkit::lang::format::Pattern::DayMonthPadded));
        }
        A::KeepAwake(on) => crate::session::idle::set_keep_awake(on),
        A::Radio("wifi", on) => capture(&["nmcli", "radio", "wifi", if on { "on" } else { "off" }], None, |_, _, _| {}),
        A::Radio(_, on) => capture(&["bluetoothctl", "power", if on { "on" } else { "off" }], None, |_, _, _| {}),
        A::Brightness(way) => {
            if crate::session::brightness::backlight().is_none() {
                *words = t!("ring-brightness-fixed").into();
            } else if !spawn(&["trindle-brightness", way]) {
                *words = t!("ring-could-not-open").into();
            }
        }
        A::Keyboard => {
            if crate::common::run::find(tray::KEYBOARD).is_none() {
                *words = t!("ring-keyboard-missing").into();
            } else {
                tray::keyboard();
            }
        }

        _ => {}
    }
}
