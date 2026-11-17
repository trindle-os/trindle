//! Calls that carry a password or a secret to a program: the portal, a password change and an unlock.

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

fn typed_password(bytes: &[u8]) -> Option<Zeroizing<String>> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Some(Zeroizing::new(text)),
        Err(e) => {
            drop(Zeroizing::new(e.into_bytes()));
            None
        }
    }
}

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
            Ok(secret) => give_without_waiting(fd, &secret),
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
        if self.working() {
            asked.fail(FAILED, "The keyring is busy with another password");
            return;
        }
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

        if self.working() {
            asked.fail(FAILED, "The keyring is busy with another password");
            return;
        }
        let Some(salt) = self.store.borrow().as_ref().map(Store::salt) else {

            let Some(typed) = typed_password(&password) else {
                asked.fail(INVALID, crate::toolkit::t!("keyring-password-unreadable"));
                return;
            };
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

fn give_without_waiting(fd: OwnedFd, bytes: &[u8]) -> bool {
    use std::os::fd::AsRawFd;
    // SAFETY: the descriptor is open and owned here; the calls only read and set its flags.
    let set = unsafe {
        let flags = libc::fcntl(fd.as_raw_fd(), libc::F_GETFL);
        flags >= 0 && libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) >= 0
    };
    set && std::fs::File::from(fd).write_all(bytes).is_ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_full_pipe_is_not_waited_on() {
        use std::os::fd::FromRawFd;
        let mut ends = [0; 2];
        // SAFETY: pipe fills the two-element array it is given.
        assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
        // SAFETY: both descriptors are fresh and owned here.
        let (read, write) = unsafe { (std::os::fd::OwnedFd::from_raw_fd(ends[0]), std::os::fd::OwnedFd::from_raw_fd(ends[1])) };
        assert!(super::give_without_waiting(write.try_clone().unwrap(), b"secret"));
        let fill = vec![0u8; 1 << 20];
        let mut file = std::fs::File::from(write.try_clone().unwrap());
        // SAFETY: as above, on the descriptor just cloned.
        unsafe { libc::fcntl(std::os::fd::AsRawFd::as_raw_fd(&file), libc::F_SETFL, libc::O_NONBLOCK) };
        let _ = std::io::Write::write(&mut file, &fill);
        let started = std::time::Instant::now();
        assert!(!super::give_without_waiting(write, b"secret"));
        assert!(started.elapsed().as_secs() < 2);
        drop(read);
    }

    #[test]
    fn a_password_is_kept_exactly_or_refused() {
        assert_eq!(super::typed_password("pässword".as_bytes()).as_deref().map(String::as_str), Some("pässword"));
        assert!(super::typed_password(b"p\xe4ssword").is_none(), "changed rather than refused");
    }
}
