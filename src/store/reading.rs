//! What the Store reads and when.

use std::rc::Rc;

use super::catalogue::{self, Catalogue};
use super::{firmware, plugin_page, update_page, updates, Store, View};

impl Store {
    pub(super) fn second(&mut self) -> bool {
        let mut found = self.notice_updating();
        if self.view == View::Plugins {
            found |= plugin_page::second(self);
        }
        if let (View::Hosted(_), Some(h)) = (&self.view, self.hosted.as_mut()) {
            found |= h.second();
        }
        found
    }

    pub(super) fn net(&mut self) -> bool {
        let online = crate::common::online();
        let changed = online != self.online;
        let back = online && !self.online;
        self.online = online;
        if back {
            self.back_online();
        }
        changed
    }

    fn back_online(&mut self) {
        update_page::refresh_off(self);
        if self.failed.is_some() && !self.loading {
            self.load_catalogue();
        } else if !update_page::own_busy(self) {
            update_page::read::read_flatpaks(self, self.upd.off, |_| {});
        }
        (self.lists_fetched, self.firmware_fetched) = (false, false);
        self.check_system();
    }

    pub(super) fn read_what_is_known(&mut self) {
        update_page::read::read_flatpaks(self, true, |_| {});
        self.check_system();
    }

    pub(super) fn load_catalogue(&mut self) {
        self.loading = true;
        self.failed = None;
        let f = self.then(|s: &mut Store, (apps, err): (Option<Catalogue>, String)| {
            s.loading = false;
            s.loaded(apps, &err);
        });
        catalogue::load(move |a, e| f((a, e)));
    }

    fn loaded(&mut self, apps: Option<Catalogue>, err: &str) {
        let Some(apps) = apps else {
            self.online = crate::common::online();
            self.failed = Some(err.to_string());
            return;
        };
        self.arrived(apps);
        update_page::refresh_off(self);
        if !update_page::own_busy(self) {
            update_page::read::read_flatpaks(self, self.upd.off, |_| {});
        }
    }

    pub(super) fn arrived(&mut self, apps: Catalogue) {
        self.failed = None;
        self.apps = Rc::new(apps);
        self.show_pending_app();
    }

    pub(super) fn refresh_installed(&mut self, then: impl FnOnce(&mut Store) + 'static) {
        catalogue::installed(self.then(move |s: &mut Store, result| {
            let changed = s.installed != result;
            s.installed = result;
            then(s);
            if changed {
                s.apps_changed();
            }
        }));
    }

    pub(super) fn preload(&mut self) -> bool {
        if self.preloaded || self.hidden {
            return false;
        }
        self.preloaded = true;
        self.read_ahead(true);
        false
    }

    fn read_ahead(&mut self, first: bool) {
        self.stale = false;
        #[cfg(test)]
        {
            let _ = first;
            self.read_aheads += 1;
        }
        #[cfg(not(test))]
        {
            super::pages::installed::installed_opened(self);
            if first {
                plugin_page::opened_by_itself(self);
                super::pages::sources_page::sources_opened(self);
            }
            if let Some(h) = self.hosted.as_mut() {
                h.preload();
            }
        }
    }

    pub(super) fn apps_changed(&mut self) {
        if !self.preloaded {
            return;
        }
        if self.hidden {
            self.stale = true;
            return;
        }
        self.read_ahead(false);
    }

    pub(super) fn back_in_sight(&mut self) {
        self.hidden = false;
        if self.stale {
            self.read_ahead(false);
        }
    }

    pub(super) fn check_system(&mut self) {
        update_page::refresh_off(self);
        updates::trindle_waiting(self.then(|s: &mut Store, v| s.trindle_waiting = v));
        updates::debian_upgradable(self.then(|s: &mut Store, list| s.debian = list));
        if !self.lists_fetched && self.online && !self.upd.off {
            self.lists_fetched = true;
            let f = self.then(|s: &mut Store, res: Result<(), String>| {
                if update_page::check::lists_fetched(s, res, false) {
                    s.check_system();
                }
            });
            updates::refresh_lists_if_stale(f);
        }
        self.check_firmware();
        if !self.firmware_fetched && self.online && !self.upd.off {
            self.firmware_fetched = true;
            let f = self.then(|s: &mut Store, ()| s.check_firmware());
            firmware::refresh_if_stale(move || f(()));
        }
    }

    fn check_firmware(&mut self) {
        firmware::waiting(self.then(|s: &mut Store, list| s.firmware = list));
    }

    pub(super) fn waiting_updates(&self) -> usize {
        update_page::counts(self).0
    }

    pub(super) fn notice_updating(&mut self) -> bool {
        let other = crate::common::updating::other();
        if other == self.other {
            return false;
        }
        self.other = other;
        if other.is_none() && self.view == View::Updates {
            update_page::check::looked_again(self);
        }
        true
    }
}
