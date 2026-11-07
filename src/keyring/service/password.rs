//! Calls that carry a password or a secret to a program.

use std::io::Write;
use std::collections::{BTreeMap, HashMap};
use std::os::fd::OwnedFd;
use zbus::Message;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zeroize::Zeroizing;
use crate::keyring::store::{DEFAULT_ALIAS, Error, Item, LOGIN, Store};
use crate::keyring::store;
use crate::toolkit::bus::Asked;
use super::{Asker, FAILED, Fault, INVALID, IS_LOCKED, Keyring, NO_SUCH, PORTAL_SCHEMA, PORTAL_SECRET, Work, answer, args, fault, portal_answer};

impl Keyring {
    pub(super) fn retrieve_secret(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (_, app_id, fd, _): (OwnedObjectPath, String, zbus::zvariant::OwnedFd, HashMap<String, OwnedValue>) =
            args(msg, "h")?;
        let fd = OwnedFd::from(fd);
        if self.locked() && self.try_login_key() {
            self.lock_changed();
        }
        if !self.locked() {
            answer(asked, &portal_answer(self.hand_over(&app_id, fd)));
            return Ok(());
        }
        if self.damaged.get() {
            return Err(fault(FAILED, "The keyring file is damaged"));
        }
        let Some(asked) = asked.take() else { return Ok(()) };
        let path = self.new_prompt(Work::Portal { app_id: app_id.clone(), fd, asked: Some(asked) })?;
        self.show_prompt(path.as_str().to_string(), Asker::App(app_id));
        Ok(())
    }

    pub(super) fn hand_over(&self, app_id: &str, fd: OwnedFd) -> bool {
        let secret = self.portal_secret(app_id);
        match secret {
            Ok(secret) => std::fs::File::from(fd).write_all(&secret).is_ok(),
            Err(Fault(_, e)) => {
                eprintln!("trindle-keyring: the secret for {app_id}: {e}");
                false
            }
        }
    }

    fn portal_secret(&self, app_id: &str) -> Result<Zeroizing<Vec<u8>>, Fault> {
        let collection = self.aliases().get(DEFAULT_ALIAS).cloned().unwrap_or_else(|| LOGIN.to_string());
        let attributes = BTreeMap::from([
            ("app_id".to_string(), app_id.to_string()),
            ("xdg:schema".to_string(), PORTAL_SCHEMA.to_string()),
        ]);
        {
            let guard = self.store.borrow();
            let s = guard.as_ref().ok_or_else(|| fault(IS_LOCKED, "The keyring is locked"))?;
            if let Some((c, id)) = s.search(Some(&collection), &attributes).into_iter().next() {
                return Ok(Zeroizing::new(s.item(&c, id)?.secret.clone()));
            }
        }
        let mut secret = Zeroizing::new(vec![0u8; PORTAL_SECRET]);
        getrandom::fill(&mut secret).map_err(|e| fault(FAILED, &e.to_string()))?;
        let item = Item {
            label: format!("Secret for {app_id}"),
            attributes,
            secret: secret.to_vec(),
            content_type: "application/octet-stream".into(),
            created: 0,
            modified: 0,
        };
        self.change(|s| s.put(&collection, item, true))?;
        self.sync();
        Ok(secret)
    }

    pub(super) fn change_password(&self, old: Zeroizing<Vec<u8>>, new: Zeroizing<Vec<u8>>, asked: Asked) {
        let Some(salt) = self.store.borrow().as_ref().map(Store::salt) else {
            asked.fail(NO_SUCH, "There is no keyring yet");
            return;
        };
        self.off_loop(move || {
            let fresh = store::new_salt();
            (store::derive(&old, &salt), fresh, store::derive(&new, &fresh))
        }, move |k, (old_key, fresh, new_key)| match k.rekey(old_key, fresh, new_key) {
            Ok(()) => asked.reply(&()),
            Err(Fault(name, words)) => asked.fail(name, &words),
        });
    }

    fn rekey(&self, old_key: store::Key, fresh: [u8; store::SALT_LEN], new_key: store::Key) -> Result<(), Fault> {
        let was_locked = self.locked();
        {
            let mut guard = self.store.borrow_mut();
            let s = guard.as_mut().ok_or_else(|| fault(NO_SUCH, "There is no keyring yet"))?;
            let opened = if s.is_locked() {
                s.unlock_with_key(old_key)
            } else {
                Store::load(&self.path).and_then(|mut check| check.unlock_with_key(old_key))
            };
            match opened {
                Ok(()) => {}
                Err(Error::WrongPassword) => return Err(fault(INVALID, "The old password does not open the keyring")),
                Err(e) => return Err(e.into()),
            }
            s.rekey_with(fresh, new_key)?;
        }
        if was_locked {
            self.lock_changed();
        } else {
            self.tell();
        }
        Ok(())
    }

    pub(super) fn unlock_with_password(&self, password: Zeroizing<Vec<u8>>, asked: Asked) {
        if !self.locked() {
            asked.reply(&());
            return;
        }
        let Some(salt) = self.store.borrow().as_ref().map(Store::salt) else {
            let typed = Zeroizing::new(String::from_utf8_lossy(&password).into_owned());
            self.try_password(typed, Box::new(move |r| match r {
                Ok(()) => asked.reply(&()),
                Err(e) => asked.fail(INVALID, &e),
            }));
            return;
        };
        self.off_loop(move || store::derive(&password, &salt), move |k, key| {
            let opened = match k.store.borrow_mut().as_mut() {
                Some(s) if s.is_locked() => s.unlock_with_key(key),
                _ => Ok(()),
            };
            match opened {
                Ok(()) => {
                    k.use_login_key();
                    k.lock_changed();
                    asked.reply(&());
                }
                Err(Error::WrongPassword) => asked.fail(INVALID, "That password does not open the keyring"),
                Err(e) => asked.fail(FAILED, &e.to_string()),
            }
        });
    }

    pub(super) fn off_loop<T: Send + 'static>(&self, work: impl FnOnce() -> T + Send + 'static,
                                   then: impl FnOnce(&Keyring, T) + 'static) {
        let me = self.me.clone();
        let mut then = Some(then);
        self.working.set(self.working.get() + 1);
        let post = self.events.post(move |t: T| {
            if let (Some(f), Some(k)) = (then.take(), me.upgrade()) {
                k.working.set(k.working.get().saturating_sub(1));
                f(&k, t);
            }
        });
        std::thread::spawn(move || {
            post.send(work());
        });
    }
}
