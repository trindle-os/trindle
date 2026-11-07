//! The password prompt.

use std::rc::Rc;
use zbus::zvariant::{OwnedObjectPath, Value};
use zeroize::Zeroizing;
use crate::keyring::store::{Error, Store};
use crate::keyring::{pam, prompt, store};
use crate::toolkit::t;
use crate::toolkit::window::Display;
use super::{Asker, FAILED, Fault, Keyring, PROMPT, PROMPTS, Work, login_key, login_name, none, op, portal_answer, reseal, why};

impl Keyring {
    pub(super) fn try_login_key(&self) -> bool {
        if self.pending.borrow().is_some() {
            return false;
        }
        let mut guard = self.store.borrow_mut();
        let (Some(s), Some(key)) = (guard.as_mut(), login_key()) else { return false };
        self.key_left.set(true);
        if s.unlock_with_key(key.clone()).is_ok() {
            return true;
        }
        if !cfg!(test) {
            pam::put_back(&key);
        }
        *self.pending.borrow_mut() = Some(key);
        false
    }

    pub(super) fn use_login_key(&self) {
        let Some(key) = self.pending.borrow_mut().take() else { return };
        if let Some(s) = self.store.borrow_mut().as_mut() {
            if !reseal(s, key) {
                return;
            }
        }
        if !cfg!(test) {
            pam::clear_key();
        }
    }

    pub(super) fn why_asked(&self) -> &'static str {
        why(self.store.borrow().is_none(), self.pending.borrow().is_some(), self.key_left.get())
    }

    pub(super) fn new_prompt(&self, work: Work) -> Result<OwnedObjectPath, Fault> {
        let path = format!("{PROMPTS}p{}", self.next());
        self.serve(&path, &[PROMPT]).map_err(|e| Fault(FAILED, e))?;
        self.prompts.borrow_mut().insert(path.clone(), work);
        Ok(op(&path))
    }

    pub(super) fn show_prompt(&self, path: String, asker: Asker) {
        let first = {
            let mut asking = self.asking.borrow_mut();
            if asking.contains(&path) {
                return;
            }
            asking.push(path);
            asking.len() == 1
        };
        if !first {
            return;
        }
        if !self.locked() {
            self.finish_all(true);
            return;
        }
        let me = self.me.clone();
        self.program_name(asker, move |program| {
            if let Some(k) = me.upgrade() {
                k.open_window(program);
            }
        });
    }

    pub(super) fn program_name(&self, asker: Asker, done: impl FnOnce(Option<String>) + 'static) {
        let named = |id: &str| {
            let apps = crate::common::apps::entries();
            crate::common::apps::find_for_window(&apps, id).map_or_else(|| id.to_string(), |e| e.name.clone())
        };
        match asker {
            Asker::App(id) if !id.is_empty() => done(Some(named(&id))),
            Asker::App(_) => done(None),
            Asker::Bus(sender) => {
                self.bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus")
                    .method("GetConnectionUnixProcessID", &(sender,), false, Some(std::time::Duration::from_secs(2)), &self.events, move |r| {
                        let pid = r.ok().and_then(|m| m.body().deserialize::<u32>().ok());
                        let comm = pid.and_then(|p| std::fs::read_to_string(format!("/proc/{p}/comm")).ok());
                        done(comm.map(|c| c.trim().to_string()).filter(|c| !c.is_empty()).map(|c| t!("keyring-named-itself", name = c)));
                    });
            }
        }
    }

    fn open_window(&self, program: Option<String>) {
        if self.asking.borrow().is_empty() {
            return;
        }
        if !self.locked() {
            self.finish_all(true);
            return;
        }
        crate::ring::give_way(&self.bus, &self.events);
        let Some(display) = self.display() else {
            self.finish_all(false);
            return;
        };
        let title = if self.store.borrow().is_none() { t!("keyring-title-start") } else { t!("keyring-title-unlock") };
        let why = self.why_asked();
        let (me, again) = (self.me.clone(), self.me.clone());
        let shown = prompt::ask(&self.events, &display, title, why, program,
            move |password, answer| match me.upgrade() {
                Some(k) => k.try_password(password, answer),
                None => answer(Err(t!("keyring-stopped").into())),
            },
            move |ok| {
                if let Some(k) = again.upgrade() {
                    k.finish_all(ok);
                }
            });
        match shown {
            Some(close) => *self.window.borrow_mut() = Some(close),
            None => self.finish_all(false),
        }
    }

    fn display(&self) -> Option<Rc<Display>> {
        if let Some(d) = self.display.borrow().as_ref() {
            return Some(d.clone());
        }
        let me = self.me.clone();
        let display = Display::connect(&self.events, move || {
            if let Some(k) = me.upgrade() {
                let gone = k.display.borrow_mut().take();
                drop(gone);
                k.finish_all(false);
            }
        })?;
        *self.display.borrow_mut() = Some(display.clone());
        Some(display)
    }

    pub(super) fn finish_all(&self, ok: bool) {
        let waiting: Vec<String> = self.asking.borrow_mut().drain(..).collect();
        for path in waiting {
            self.complete(&path, ok);
        }
        self.close_window();
    }

    pub(super) fn close_window(&self) {
        let close = self.window.borrow_mut().take();
        if let Some(close) = close {
            close();
        }
    }

    pub(super) fn complete(&self, path: &str, ok: bool) {
        let Some(work) = self.prompts.borrow_mut().remove(path) else { return };
        let (dismissed, result) = match work {
            Work::Unlock(objects) if ok && !self.locked() => (false, Value::from(objects)),
            Work::Unlock(_) => (true, Value::from(Vec::<OwnedObjectPath>::new())),
            Work::Create { label, alias } if ok => match self.create_collection(&label, &alias) {
                Ok(p) => (false, Value::from(p)),
                Err(Fault(_, e)) => {
                    eprintln!("trindle-keyring: {e}");
                    (true, Value::from(none()))
                }
            },
            Work::Create { .. } => (true, Value::from(none())),
            Work::Portal { app_id, fd, asked } => {
                let ok = ok && self.hand_over(&app_id, fd);
                if let Some(a) = asked {
                    a.reply(&portal_answer(ok));
                }
                (!ok, Value::from(none()))
            }
        };
        self.emit(path, PROMPT, "Completed", &(dismissed, result));
        self.unserve(path);
    }

    pub(super) fn try_password(&self, password: Zeroizing<String>, answer: prompt::Answer) {
        let salt = self.store.borrow().as_ref().map(Store::salt);
        if let Some(salt) = salt {
            self.off_loop(move || store::derive(password.as_bytes(), &salt), move |k, key| {
                let opened = match k.store.borrow_mut().as_mut() {
                    Some(s) if s.is_locked() => s.unlock_with_key(key),
                    _ => Ok(()),
                };
                answer(match opened {
                    Ok(()) => {
                        k.use_login_key();
                        k.lock_changed();
                        Ok(())
                    }
                    Err(Error::WrongPassword) => Err(t!("keyring-not-the-keyring-password").into()),
                    Err(e) => Err(e.to_string()),
                });
            });
            return;
        }
        if self.damaged.get() {
            answer(Err(t!("keyring-damaged-left").into()));
            return;
        }
        let (user, path) = (login_name(), self.path.clone());
        self.off_loop(move || {
            if !crate::session::lock::password_ok(&user, &password) {
                return Err(t!("keyring-not-login-password").to_string());
            }
            if path.exists() {
                let mut s = Store::load(&path).map_err(|e| e.to_string())?;
                s.unlock(password.as_bytes()).map_err(|e| e.to_string())?;
                return Ok(s);
            }
            Store::create(&path, password.as_bytes()).map_err(|e| e.to_string())
        }, move |k, made| match made {
            Ok(s) => {
                *k.store.borrow_mut() = Some(s);
                k.lock_changed();
                answer(Ok(()));
            }
            Err(e) => answer(Err(e)),
        });
    }
}
