//! Starting the Store.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::time::Duration;

use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::t;
use crate::toolkit::ui::Shown;
use crate::toolkit::window::{Display, Kind};
use super::opts::Opts;
use super::spot::bring_into_view;
use super::{boxed, cli, steam, Do, Store, NAME, PATH, SIZE, TICK, XML};

const SETTLED: Duration = Duration::from_millis(1500);

pub(super) fn folders_that_change() -> Vec<PathBuf> {
    let mut all = Vec::new();
    for place in crate::common::access::flatpak_places() {
        all.extend([place.join("app"), place.join("overrides"), place]);
    }
    all.extend([boxed::folders(), crate::plugins::folders()]);
    all
}

fn watch_what_is_installed(events: &Loop, asks: Post<Do>) {
    let Ok(mut watch) = crate::toolkit::watch::Watch::new() else { return };
    let watched = folders_that_change().iter().filter(|f| watch.folder(f).is_ok()).count();
    if watched == 0 {
        return;
    }
    let last = Rc::new(Cell::new(0u64));
    let lp = events.clone();
    events.watch(watch.fd(), move |hung_up| {
        if hung_up {
            return Flow::Stop;
        }
        if watch.changed().is_empty() {
            return Flow::Continue;
        }
        let mine = last.get() + 1;
        last.set(mine);
        let (last, asks) = (last.clone(), asks.clone());
        lp.after(SETTLED, move || {
            if last.get() == mine {
                asks.send(Do::Then(Box::new(|s: &mut Store| s.apps_changed())));
            }
        });
        Flow::Continue
    });
}

struct Running {
    _shown: Rc<Shown<Store>>,
    _served: Option<Served>,
    _raise: Option<Served>,
}

pub fn main(args: Vec<String>) -> i32 {
    let opts = Opts::read(&args);
    if opts.renew_steam {
        return steam::renew_when_free();
    }
    let events = Loop::new();
    if let Some(code) = cli::command_line_work(&opts, &events) {
        return code;
    }
    let Some(bus) = Bus::session(&events).ok() else { return open(&events, opts) };
    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let opts = RefCell::new(Some(opts));
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        match start(&ev, &b, opts.borrow_mut().take().unwrap_or_default()) {
            Some(running) => *k.borrow_mut() = Some(running),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        b2.call(NAME, PATH, NAME).method("Open", &(args.clone(),), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    code.get()
}

fn start(events: &Loop, bus: &Bus, opts: Opts) -> Option<Running> {
    let (shown, asks) = window(events, opts, Some(bus.clone()))?;
    let served = bus.serve(PATH, Offer {
        name: NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            if asked.member() == "Open" {
                let args = asked.message().body().deserialize::<(Vec<String>,)>().map(|a| a.0).unwrap_or_default();
                asks.send(Do::Open(args));
            }
            asks.send(Do::Show);
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window).ok();
    Some(Running { _shown: shown, _served: served, _raise: raise })
}

fn open(events: &Loop, opts: Opts) -> i32 {
    let Some(_running) = window(events, opts, None) else { return 1 };
    events.run();
    0
}

fn window(events: &Loop, opts: Opts, bus: Option<Bus>) -> Option<(Rc<Shown<Store>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Store>>>> = Rc::new(RefCell::new(Weak::new()));
    let asks = answer_asks(events, slot.clone());
    let deb = opts.deb.clone();
    let mut app = Store::new(events, asks.clone(), Some(display.clone()));
    let back = asks.clone();
    let settings_asks = events.post(move |d: crate::settings::ui::Do| {
        back.send(Do::Settings(d));
    });
    app.hosted = Some(crate::settings::Hosted::new(events, settings_asks, bus.clone(), Bus::system(events).ok()));
    app.bus = bus;
    let kind = Kind::Window { title: t!("store-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    if deb.is_some() {
        shown.window.set_visible(false);
    }
    *slot.borrow_mut() = Rc::downgrade(&shown);
    hide_while_working_on_close(events, &shown);
    tick_while_moving(events, &asks, &shown);
    watch_what_is_installed(events, asks.clone());
    shown.change(|s| {
        s.load_catalogue();
        s.want_listed();
        s.read_what_is_known();
        s.apply(Opts { deb: None, ..opts });
    });
    if deb.is_none() {
        bring_into_view(&shown);
    }
    if deb.is_some() {
        shown.change(|s| {
            s.hidden = true;
            s.apply(Opts { deb, ..Opts::default() });
        });
    }
    Some((shown, asks))
}

fn answer_asks(events: &Loop, slot: Rc<RefCell<Weak<Shown<Store>>>>) -> Post<Do> {
    events.post(move |ask: Do| {
        let Some(shown) = slot.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                shown.change(Store::back_in_sight);
                shown.window.raise();
            }
            Do::Tick if !shown.with(Store::moving) => {}
            Do::Pulse => {
                if shown.with(|s| s.pulsing.get() && !s.hidden) && shown.window.seen() {
                    shown.window.patch();
                }
            }
            ask => {
                shown.change(|s| {
                    let was = s.page_key();
                    s.handle(ask);
                    s.turned(&was);
                });
                bring_into_view(&shown);
            }
        }
    })
}

fn hide_while_working_on_close(events: &Loop, shown: &Rc<Shown<Store>>) {
    let (ev, w) = (events.clone(), Rc::downgrade(shown));
    shown.window.on_close(move || {
        if let Some(shown) = w.upgrade() {
            if shown.with(|s| s.working > 0) {
                shown.window.set_visible(false);
                shown.change(|s| s.hidden = true);
                return;
            }
        }
        ev.quit();
    });
}

fn tick_while_moving(events: &Loop, asks: &Post<Do>, shown: &Rc<Shown<Store>>) {
    let (t, w) = (asks.clone(), Rc::downgrade(shown));
    let mut ticks = 0u32;
    events.every(TICK, move || {
        let Some(shown) = w.upgrade() else { return Flow::Stop };
        ticks = ticks.wrapping_add(1);
        if ticks % 4 == 0 {
            if shown.window.seen() {
                shown.change_if(Store::preload);
            }
            shown.change_if(Store::second);
        }
        if ticks % 20 == 0 {
            shown.change_if(Store::net);
        }
        if shown.with(Store::moving) && !t.send(Do::Tick) {
            return Flow::Stop;
        }
        Flow::Continue
    });
}
