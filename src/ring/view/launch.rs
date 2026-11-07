//! Starting programs and tools from the ring, and the favourites it keeps.

use std::time::Duration;

use crate::common::{apps, favourites};
use crate::common::run::spawn;
use crate::ring::{magnify, store};
use crate::ring::ctx::Program;
use crate::toolkit::t;

use super::{Face, REFUSED_SECONDS};

impl Face {
    pub(super) fn run(&mut self, argv: &[&str]) {
        if spawn(argv) {
            self.close();
        } else {
            self.say(t!("ring-could-not-open"), REFUSED_SECONDS);
        }
    }

    pub(super) fn toggle_magnifier(&mut self, from: Option<(i32, i32, i32, i32)>) {
        if magnify::on() {
            return magnify::stop();
        }
        self.close();
        magnify::start(&self.ctx, from, || {});
    }

    pub(super) fn run_after_closing(&mut self, argv: &'static [&'static str]) {
        if crate::common::run::find(argv[0]).is_none() {
            return self.say(t!("ring-could-not-open"), REFUSED_SECONDS);
        }
        self.close();
        self.ctx.lp.after(Duration::from_millis(400), move || {
            spawn(argv);
        });
    }

    pub(in crate::ring) fn open_own(&mut self, key: &str, args: &[&str]) {
        let Some(own) = apps::own_key(key) else { return };
        let raised = self.raise_own(own);
        if raised && args.is_empty() {
            return self.close();
        }
        let mut argv = vec![own.start_with()];
        argv.extend_from_slice(args);
        self.run(&argv);
    }

    fn raise_own(&self, own: &apps::Own) -> bool {
        let Some(ws) = &self.windows else { return false };
        let all = ws.windows();
        let found = apps::newest_of(&all, |w| w.app_id.as_str(), own);
        if let Some(w) = found {
            ws.activate(w);
        }
        found.is_some()
    }

    pub(super) fn activate(&mut self, i: usize) {
        let all = self.all();
        let Some(app) = all.get(i) else { return };
        self.launch(app);
    }

    pub(super) fn launch(&mut self, app: &Program) {
        if favourites::is_path(&app.id) {
            return self.open_path(&app.id);
        }
        if let (Some(ws), Some(w)) = (self.windows.clone(), self.windows_of(app).last().cloned()) {
            ws.activate(&w);
            store::record_use(&app.id);
            self.close();
            return;
        }
        self.launch_new(app);
    }

    pub(super) fn launch_new(&mut self, app: &Program) {
        if favourites::is_path(&app.id) {
            return self.open_path(&app.id);
        }
        if apps::launch(app, &[]) {
            store::record_use(&app.id);
            self.close();
        } else {
            self.say(t!("ring-could-not-open"), REFUSED_SECONDS);
        }
    }

    pub(super) fn set_favourite(&mut self, id: &str, on: bool) {
        self.favourites.retain(|f| f != id);
        if on {
            self.favourites.push(id.to_string());
        }
        store::save_favourites(&self.favourites);
        self.fav_stamp = store::favourites_changed_at();
        self.relayout();
    }

    pub(super) fn use_tool(&mut self, key: &str) {
        match key {
            "screenshot" => self.run_after_closing(&["trindle-screenshot"]),
            "magnifier" => {
                let r = self.pressed_at;
                self.toggle_magnifier(Some((r[0] as i32, r[1] as i32, r[2] as i32, r[3] as i32)));
            }
            "eye-dropper" => self.run(&["trindle-eye-dropper", crate::screenshot::colours::TAKE]),
            "settings" => self.settings(None),
            "store" => self.open_own("store", &[]),
            "activity" => self.open_own("activity", &[]),
            "drives" => self.open_own("drives", &[]),
            "report" => self.open_own("report", &[]),
            other => self.quick.show(other),
        }
    }

    pub(super) fn toggle_shortcuts(&mut self, was_open: bool) {
        if was_open && self.quick.shows("shortcuts") {
            return self.close();
        }
        self.show_power(false);
        self.close_menu();
        self.close_beside();
        self.win_menu = None;
        self.quick.show("shortcuts");
    }

    pub fn quick_done(&mut self) {
        if self.quick.shows("shortcuts") {
            self.close();
        } else {
            self.quick.hide();
        }
    }
}
