//! Answers from work done elsewhere, handed back to the Store, and what it does with each.

use std::rc::Rc;

use super::opts::Opts;
use super::{Do, Store};

impl Store {

    pub(super) fn then<T: 'static>(&self, f: impl FnOnce(&mut Store, T) + 'static) -> impl FnOnce(T) + 'static {
        let asks = self.asks.clone();
        move |t| {
            asks.send(Do::Then(Box::new(move |s| f(s, t))));
        }
    }

    pub(super) fn heard_step(&self, f: impl Fn(&mut Store, String, Option<(usize, usize)>) + 'static) -> impl Fn((String, Option<(usize, usize)>)) + 'static {
        let asks = self.asks.clone();
        let f = Rc::new(f);
        move |(line, step): (String, Option<(usize, usize)>)| {
            let f = f.clone();
            asks.send(Do::Then(Box::new(move |s| f(s, line, step))));
        }
    }

    pub(super) fn heard(&self, f: impl Fn(&mut Store, &str) + 'static) -> impl Fn(&str) + 'static {
        let asks = self.asks.clone();
        let f = Rc::new(f);
        move |line: &str| {
            let (f, line) = (f.clone(), line.to_string());
            asks.send(Do::Then(Box::new(move |s| f(s, &line))));
        }
    }

    pub(super) fn handle(&mut self, d: Do) {
        match d {
            Do::Then(f) => f(self),
            Do::Open(args) => self.apply(Opts::read(&args)),
            Do::Fade(t) => self.fade = t,
            Do::Lit(t) => self.lit = t,
            Do::Settings(d) => {
                if let Some(h) = self.hosted.as_mut() {
                    h.handle(d);
                }
            }
            Do::Show | Do::Tick | Do::Pulse => {}
        }
    }

    pub(super) fn job_over(&mut self) {
        self.working = self.working.saturating_sub(1);
        if self.working == 0 && self.hidden && self.debs.is_empty() {
            self.events.quit();
        }
    }
}
