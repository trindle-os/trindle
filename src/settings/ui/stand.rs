//! A Settings window with one made-up page and no buses, for tests.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::{Do, Kit, Page};
use crate::settings::Settings;
use crate::toolkit::event::Loop;

pub(crate) struct Stand {
    pub(crate) settings: Settings,
    back: Rc<RefCell<Vec<Do>>>,
    events: Loop,
}

pub(crate) fn shoot_both(page: Box<dyn Page>, name: &str, (w, h): (f32, f32)) {
    use crate::toolkit::ui::{App as _, Ui};
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let mut kit = Kit::new(events, asks, None, None);
    kit.current = page.id().to_string();
    let s = crate::settings::Settings { kit, pages: vec![page], current: 0, search: String::new(), tall: h };
    std::fs::create_dir_all(&out).unwrap();
    for (look, light) in [("dark", false), ("light", true)] {
        let mut ui = Ui::new(s.view(), crate::toolkit::theme::Theme::new(light, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{name}-{look}.png"))).unwrap();
    }
}

pub(crate) fn made_up_taken(name: &str) -> crate::common::copies::Taken {
    crate::common::copies::Taken { name: name.into(), from: format!("/nowhere/new/{name}").into(), there: format!("/nowhere/old/{name}").into(),
                                   new_size: 20_480, old_size: 18_000 }
}

pub(crate) fn made_up_clashes(names: &[&str]) -> super::Clashes {
    let mut c = super::Clashes::default();
    c.add_batch(names.iter().map(|n| made_up_taken(n)));
    c
}

impl Stand {
    pub(crate) fn new(page: Box<dyn Page>) -> Stand {
        let events = Loop::new();
        let back = Rc::new(RefCell::new(Vec::new()));
        let into = back.clone();
        let asks = events.post(move |ask: Do| into.borrow_mut().push(ask));
        let mut kit = Kit::new(events.clone(), asks, None, None);
        kit.current = page.id().to_string();
        Stand { settings: Settings { kit, pages: vec![page], current: 0, search: String::new(), tall: 800.0 }, back, events }
    }

    pub(crate) fn page<P: Page + 'static, T>(&mut self, f: impl FnOnce(&mut P, &mut Kit) -> T) -> T {
        let mut out = None;
        self.settings.with::<P>(|p, k| out = Some(f(p, k)));
        out.expect("the page is of this kind")
    }

    pub(crate) fn turn(&mut self) {
        let stop = self.events.clone();
        self.events.after(Duration::from_millis(20), move || stop.quit());
        self.events.run();
        let waiting: Vec<Do> = self.back.borrow_mut().drain(..).collect();
        for ask in waiting {
            if let Do::Then(f) = ask {
                f(&mut self.settings);
            }
        }
    }

    pub(crate) fn red(&self, key: &str) -> Option<String> {
        self.settings.kit.said(key).filter(|(_, problem)| *problem).map(|(words, _)| words.to_string())
    }

    pub(crate) fn glows(&self, key: &str) -> bool {
        self.settings.kit.glow(key) > 0
    }
}
