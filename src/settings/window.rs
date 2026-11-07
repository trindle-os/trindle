//! Opening the Settings window, for this run alone or shared so that a second call reuses the open one.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

use super::ui::{Do, Kit};
use super::{own_pages, Settings, NAME, PATH, SIZE, XML};
use crate::common::files;
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::t;
use crate::toolkit::ui::Shown;
use crate::toolkit::watch::Watch;
use crate::toolkit::window::{Display, Kind};

type Slot = Rc<RefCell<Weak<Shown<Settings>>>>;

pub(super) fn share(events: &Loop, bus: &Bus, args: Vec<String>) -> i32 {
    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let first = RefCell::new(Some(args.clone()));
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(bus, events, NAME, PATH, move || {
        match start(&ev, &b, first.borrow_mut().take().unwrap_or_default()) {
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

struct Running {
    _shown: Rc<Shown<Settings>>,
    _served: Option<Served>,
    _raise: Option<Served>,
}

fn start(events: &Loop, bus: &Bus, args: Vec<String>) -> Option<Running> {
    let (shown, asks) = window(events, args, Some(bus.clone()))?;
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

pub(super) fn open(events: &Loop, args: Vec<String>) -> i32 {
    let Some(_running) = window(events, args, None) else { return 1 };
    events.run();
    0
}

fn window(events: &Loop, args: Vec<String>, bus: Option<Bus>) -> Option<(Rc<Shown<Settings>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Slot = Rc::new(RefCell::new(Weak::new()));
    let asks = answering(events, &slot);
    let system = Bus::system(events).ok();
    let mut kit = Kit::new(events.clone(), asks.clone(), bus, system);
    let pages = own_pages();
    kit.current = pages[0].id().to_string();
    kit.icon = pages[0].icon();
    let app = Settings { kit, pages, current: 0, search: String::new(), tall: 0.0 };
    let kind = Kind::Window { title: t!("settings-window-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    stop_pages_on_close(events, &shown);
    shown.change(|s| {
        let at = s.current;
        s.pages[at].shown(&mut s.kit);
        s.handle(Do::Open(args));
    });
    tick(events, &shown, &asks);
    watch_conf(events, asks.clone());
    Some((shown, asks))
}

fn answering(events: &Loop, slot: &Slot) -> Post<Do> {
    let s = slot.clone();
    events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                shown.window.raise();
            }
            Do::Tick if !shown.with(Settings::moving) => {}
            Do::Say(words) => shown.announce(&words),
            Do::Conf => {
                shown.restyle();
                shown.change(|s| s.handle(Do::Conf));
            }
            ask => shown.change(|s| s.handle(ask)),
        }
    })
}

fn stop_pages_on_close(events: &Loop, shown: &Rc<Shown<Settings>>) {
    let (ev, w) = (events.clone(), Rc::downgrade(shown));
    shown.window.on_close(move || {
        if let Some(shown) = w.upgrade() {
            shown.change(|s| {
                let at = s.current;
                s.pages[at].hidden(&mut s.kit);
            });
        }
        ev.quit();
    });
}

fn tick(events: &Loop, shown: &Rc<Shown<Settings>>, asks: &Post<Do>) {
    let (t, w) = (asks.clone(), Rc::downgrade(shown));
    let mut ticks = 0u32;
    events.every(Duration::from_millis(250), move || {
        let Some(shown) = w.upgrade() else { return Flow::Stop };
        ticks = ticks.wrapping_add(1);
        if ticks % 4 == 0 {
            shown.change_if(Settings::second);
        }
        if shown.with(Settings::moving) && !t.send(Do::Tick) {
            return Flow::Stop;
        }
        Flow::Continue
    });
}

fn watch_conf(events: &Loop, asks: Post<Do>) {
    let Ok(mut watch) = Watch::new() else { return };
    let folder = files::trindle_config();
    let _ = std::fs::create_dir_all(&folder);
    if watch.folder(&folder).is_err() {
        return;
    }
    let conf = files::trindle_conf();
    events.watch(watch.fd(), move |hung_up| {
        if hung_up {
            return Flow::Stop;
        }
        if watch.changed().contains(&conf) && !asks.send(Do::Conf) {
            return Flow::Stop;
        }
        Flow::Continue
    });
}
