//! Installing the apps picked, from Flathub for this person alone, and telling the window how far it has got.

use std::cell::RefCell;
use std::rc::Rc;

use crate::common::run;
use crate::store::catalogue::REMOTE;
use crate::toolkit::t;
use crate::toolkit::event::Loop;
use crate::toolkit::ui::Shown;

use super::{Ask, Install, Welcome};

pub(super) fn install(shown: &Rc<Shown<Welcome>>, ids: Vec<String>, events: &Loop) {
    let mut argv = vec!["flatpak", "install", "--user", "-y", REMOTE];
    argv.extend(ids.iter().map(String::as_str));
    let job = RefCell::new(crate::store::progress::Job::default());
    let trouble = Rc::new(RefCell::new(String::new()));
    let (w, t) = (Rc::downgrade(shown), trouble.clone());
    let on_line = move |line: &str| {
        let heard = job.borrow_mut().heard(line);
        match heard {
            Some(at) => {
                let left = job.borrow().left(at).map(crate::store::progress::left_in_words);
                if let Some(shown) = w.upgrade() {
                    shown.change(|s| s.progress(at.share as f32, left));
                }
            }
            None if !line.trim().is_empty() => *t.borrow_mut() = line.trim().to_string(),
            None => {}
        }
    };
    let hold = events.hold();
    let w = Rc::downgrade(shown);
    run::stream_stoppable(&run::line_by_line(&crate::common::gentle::argv(&argv)), on_line, move |ok| {
        if let Some(shown) = w.upgrade() {
            let why = trouble.borrow().clone();
            shown.change(|s| s.installed_now(ok, if why.is_empty() { t!("welcome-flatpak-refused").into() } else { why }));
        }
        drop(hold);
    });
}

impl Welcome {
    pub(super) fn installing(&self) -> bool {
        matches!(self.install, Install::Running(..))
    }

    pub(super) fn begin(&mut self, ids: Vec<String>) {
        self.install = Install::Running(ids.clone(), 0.0, None);
        self.ask(Ask::Install(ids));
    }

    pub(super) fn retry(&mut self) {
        if !self.online {
            return;
        }
        if let Install::Failed(ids, _) = std::mem::replace(&mut self.install, Install::Idle) {
            self.begin(ids);
        }
    }

    pub(super) fn pick(&mut self, id: &str) {
        if self.installing() || self.installed.contains(id) {
            return;
        }
        if !self.picked.remove(id) {
            self.picked.insert(id.to_string());
        }
    }

    pub(super) fn progress(&mut self, share: f32, left: Option<String>) {
        if let Install::Running(_, at, words) = &mut self.install {
            *at = share.clamp(0.0, 1.0);
            *words = left;
        }
    }

    pub(super) fn installed_now(&mut self, ok: bool, trouble: String) {
        let Install::Running(ids, ..) = std::mem::replace(&mut self.install, Install::Idle) else { return };
        if ok {
            self.installed.extend(ids.iter().cloned());
            self.glow += 1;
            self.install = Install::Done(ids);
        } else {
            self.install = Install::Failed(ids, trouble);
        }
    }
}
