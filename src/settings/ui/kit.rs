//! What every Settings page shares.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use super::{Cause, Do, Page, is_just_saved, no_session_bus, no_system_bus};
use crate::common::filling::Dial;
use crate::settings::Settings;
use crate::toolkit::bus::Bus;
use crate::toolkit::t;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::widget::Rect;

const SAID_FOR: u64 = 3;

const RING_STAYS: Duration = Duration::from_millis(1500);

struct Said {
    words: String,
    ok: bool,
    glow: bool,
    n: u64,
}

pub struct Kit {
    pub events: Loop,
    pub(crate) asks: Post<Do>,
    pub session: Option<Bus>,
    pub system: Option<Bus>,
    said: HashMap<String, Said>,
    saying: u64,
    pub dials: HashMap<String, Dial>,
    pub(crate) info: Option<(String, String, Rect)>,
    pub(crate) pressed_at: Option<Rect>,
    pub(crate) hosted: bool,
    pub(crate) want: Option<String>,
    pub(crate) back: bool,
    pub(crate) from: Option<String>,
    pub(crate) current: String,
    pub(crate) icon: &'static str,
    pub(crate) pulse: std::cell::Cell<bool>,
}

impl Kit {
    pub(crate) fn new(events: Loop, asks: Post<Do>, session: Option<Bus>, system: Option<Bus>) -> Kit {
        Kit { events, asks, session, system, said: HashMap::new(), saying: 0, dials: HashMap::new(),
              info: None, pressed_at: None, hosted: false, want: None, back: false, from: None, current: String::new(),
              icon: "", pulse: std::cell::Cell::new(false) }
    }

    pub fn then<P: Page + 'static, T: 'static>(&self, f: impl FnOnce(&mut P, &mut Kit, T) + 'static) -> impl FnOnce(T) + 'static {
        let asks = self.asks.clone();
        move |t| {
            asks.send(Do::Then(Box::new(move |s: &mut Settings| s.with::<P>(move |p, k| f(p, k, t)))));
        }
    }

    pub fn heard<P: Page + 'static>(&self, f: impl Fn(&mut P, &mut Kit, &str) + 'static) -> impl Fn(&str) + 'static {
        let asks = self.asks.clone();
        let f = Rc::new(f);
        move |line: &str| {
            let (f, line) = (f.clone(), line.to_string());
            asks.send(Do::Then(Box::new(move |s: &mut Settings| s.with::<P>(move |p, k| f(p, k, &line)))));
        }
    }

    pub fn after<P: Page + 'static>(&self, wait: Duration, f: impl FnOnce(&mut P, &mut Kit) + 'static) {
        let back = self.then::<P, ()>(move |p, k, ()| f(p, k));
        self.events.after(wait, move || back(()));
    }

    fn after_kit(&self, wait: Duration, f: impl FnOnce(&mut Kit) + 'static) {
        let asks = self.asks.clone();
        self.events.after(wait, move || {
            asks.send(Do::Then(Box::new(move |s: &mut Settings| f(&mut s.kit))));
        });
    }

    pub fn aloud(&self, words: &str) {
        self.asks.send(Do::Say(words.to_string()));
    }

    pub fn go_to(&mut self, id: &str) {
        self.want = Some(id.to_string());
    }

    pub fn go_to_help(&mut self, topic: &str) {
        self.from = Some(self.current.clone());
        self.want = Some(format!("help:{topic}"));
    }

    pub fn go_back(&mut self) {
        self.back = true;
    }

    pub fn came_from(&self) -> Option<&str> {
        self.from.as_deref()
    }

    pub fn say(&mut self, key: &str, words: &str, ok: bool, stay: bool) {
        self.show(key, words, ok, stay, ok && !stay);
    }

    pub fn tell(&mut self, key: &str, words: &str) {
        self.show(key, words, true, false, false);
    }

    pub fn done(&mut self, key: &str, error: Option<&str>, success: &str) {
        match error {
            Some(e) if !e.is_empty() => self.say(key, e, false, false),
            _ => self.say(key, success, true, false),
        }
    }

    pub fn settle<T, E: Cause>(&mut self, key: &str, result: Result<T, E>, could_not: &str) -> Option<T> {
        match result {
            Ok(got) => {
                self.say(key, t!("settings-common-saved"), true, false);
                Some(got)
            }
            Err(e) => {
                self.say(key, &e.words(could_not), false, false);
                None
            }
        }
    }

    pub fn settle_quietly<T, E: Cause>(&mut self, key: &str, result: Result<T, E>, could_not: &str) -> Option<T> {
        match result {
            Ok(got) => {
                if self.said.get(key).is_some_and(|s| !s.ok) {
                    self.said.remove(key);
                }
                Some(got)
            }
            Err(e) => {
                self.say(key, &e.words(could_not), false, false);
                None
            }
        }
    }

    pub fn settles<P: Page + 'static, T: 'static, E: Cause + 'static>(
        &self, key: &str, could_not: impl Into<String>, then: impl FnOnce(&mut P, &mut Kit, Option<T>) + 'static,
    ) -> impl FnOnce(Result<T, E>) + 'static {
        let (key, could_not) = (key.to_string(), could_not.into());
        self.then::<P, Result<T, E>>(move |p, k, result| {
            let got = k.settle(&key, result, &could_not);
            then(p, k, got);
        })
    }

    pub fn settles_quietly<P: Page + 'static, T: 'static, E: Cause + 'static>(
        &self, key: &str, could_not: impl Into<String>, then: impl FnOnce(&mut P, &mut Kit, Option<T>) + 'static,
    ) -> impl FnOnce(Result<T, E>) + 'static {
        let (key, could_not) = (key.to_string(), could_not.into());
        self.then::<P, Result<T, E>>(move |p, k, result| {
            let got = k.settle_quietly(&key, result, &could_not);
            then(p, k, got);
        })
    }

    pub fn started(&mut self, key: &str, started: bool, could_not: &str) -> bool {
        if !started {
            self.say(key, could_not, false, false);
        }
        started
    }

    pub fn unanswered(&mut self, key: &str) {
        if crate::toolkit::chooser::failed() {
            self.say(key, crate::toolkit::chooser::did_not_answer(), false, false);
        }
    }

    pub fn system_for(&mut self, key: &str) -> Option<Bus> {
        if self.system.is_none() {
            self.say(key, no_system_bus(), false, false);
        }
        self.system.clone()
    }

    pub fn session_for(&mut self, key: &str) -> Option<Bus> {
        if self.session.is_none() {
            self.say(key, no_session_bus(), false, false);
        }
        self.session.clone()
    }

    pub fn hush(&mut self, key: &str) {
        self.said.remove(key);
    }

    fn show(&mut self, key: &str, words: &str, ok: bool, stay: bool, glow: bool) {
        if words.is_empty() && !glow {
            self.said.remove(key);
            return;
        }
        let words = if ok && !stay && is_just_saved(words) { "" } else { words };
        self.saying += 1;
        let n = self.saying;
        self.said.insert(key.to_string(), Said { words: words.to_string(), ok, glow, n });
        if ok && !stay {
            let key = key.to_string();
            self.after_kit(Duration::from_secs(SAID_FOR), move |k| {
                if k.said.get(&key).is_some_and(|s| s.n == n) {
                    k.said.remove(&key);
                }
            });
        }
    }

    pub fn said(&self, key: &str) -> Option<(&str, bool)> {
        self.said.get(key).filter(|s| !s.words.is_empty()).map(|s| (s.words.as_str(), !s.ok))
    }

    pub(super) fn glow(&self, key: &str) -> u32 {
        self.said.get(key).filter(|s| s.glow).map_or(0, |s| s.n as u32)
    }

    pub fn start(&mut self, key: &str, doing: &str) -> &mut Dial {
        let mut d = Dial::default();
        d.say(doing);
        self.dials.insert(key.to_string(), d);
        self.dials.get_mut(key).expect("just put in")
    }

    pub fn dial(&mut self, key: &str) -> Option<&mut Dial> {
        self.dials.get_mut(key)
    }

    pub fn working(&self, key: &str) -> bool {
        self.dials.get(key).is_some_and(Dial::running)
    }

    pub fn finish(&mut self, key: &str, ok: bool, words: &str) {
        let said = match (words.is_empty(), ok) {
            (false, _) => words,
            (true, true) => t!("settings-common-done"),
            (true, false) => t!("settings-common-not-done"),
        };
        if let Some(d) = self.dials.get_mut(key) {
            d.finish(ok, said);
        }
        if !ok && !words.is_empty() {
            self.say(key, words, false, false);
        }
        let key = key.to_string();
        self.after_kit(RING_STAYS, move |k| {
            if k.dials.get(&key).is_some_and(|d| !d.running()) {
                k.dials.remove(&key);
            }
        });
    }

    pub(crate) fn moving(&self) -> bool {
        self.pulse.get() || self.dials.values().any(Dial::running)
    }
}
