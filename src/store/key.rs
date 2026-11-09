//! Where Apps keeps the key a person gives it for GitHub: in their
//! keyring, the Secret Service, which is locked with their login password
//! and holds nothing in plain words on the disk. It is asked over D-Bus as
//! any program asks it, with a plain session, as the keyring lives on this
//! computer's own bus.

use crate::toolkit::t;
use std::collections::HashMap;
use std::path::Path;

use zbus::blocking::Connection;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use zeroize::Zeroizing;

const NAME: &str = "org.freedesktop.secrets";
const ROOT: &str = "/org/freedesktop/secrets";
const SERVICE: &str = "org.freedesktop.Secret.Service";
const COLLECTION: &str = "org.freedesktop.Secret.Collection";
const ITEM: &str = "org.freedesktop.Secret.Item";
const SESSION: &str = "org.freedesktop.Secret.Session";
const DEFAULT: &str = "/org/freedesktop/secrets/aliases/default";

const KIND: [(&str, &str); 2] = [("application", "trindle-apps"), ("service", "github")];
const LABEL: &str = "GitHub key for Apps";

pub use crate::common::keyring_said::{within, within_late, Waited, locked_words, missing_words};
#[cfg(test)]
pub use crate::common::keyring_said::silent_words;
use crate::common::keyring_said::WAIT;

type Secret = (OwnedObjectPath, Vec<u8>, Vec<u8>, String);

#[cfg(test)]
thread_local! {
    static ADDRESS: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

#[derive(Clone, Debug)]
pub struct Ring {

    #[cfg(test)]
    address: Option<String>,
}

pub fn ring() -> Ring {
    Ring {
        #[cfg(test)]
        address: ADDRESS.with(|a| a.borrow().clone()),
    }
}

impl Ring {

    #[cfg(not(test))]
    pub fn bus(&self) -> Result<Connection, String> {
        zbus::blocking::connection::Builder::session().and_then(|b| b.method_timeout(WAIT).build()).map_err(|e| unreached(&e))
    }

    #[cfg(test)]
    pub fn bus(&self) -> Result<Connection, String> {
        let address = self.address.as_deref().ok_or("no keyring in a test")?;
        zbus::blocking::connection::Builder::address(address).and_then(|b| b.method_timeout(WAIT).build()).map_err(|e| unreached(&e))
    }
}

fn unreached(e: &zbus::Error) -> String {
    t!("store-key-unreached", why = e.to_string())
}

#[cfg(test)]
pub fn pretend(address: Option<String>) {
    crate::common::keyring_said::pretend(address.clone());
    ADDRESS.with(|a| *a.borrow_mut() = address);
}

fn call<B: serde::Serialize + zbus::zvariant::DynamicType>(c: &Connection, path: &str, iface: &str, method: &str, body: &B)
    -> Result<zbus::Message, String> {
    c.call_method(Some(NAME), path, Some(iface), method, body).map_err(crate::common::keyring_said::why)
}

fn open(c: &Connection) -> Result<OwnedObjectPath, String> {
    let (_, s): (OwnedValue, OwnedObjectPath) = call(c, ROOT, SERVICE, "OpenSession", &("plain", Value::from("")))?
        .body().deserialize().map_err(|e| e.to_string())?;
    Ok(s)
}

fn close(c: &Connection, s: &OwnedObjectPath) {

    let _ = call(c, s.as_str(), SESSION, "Close", &());
}

fn items(c: &Connection) -> Result<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>), String> {
    let ask: HashMap<&str, &str> = HashMap::from(KIND);
    call(c, ROOT, SERVICE, "SearchItems", &(ask,))?.body().deserialize().map_err(|e| e.to_string())
}

pub fn read(c: &Connection) -> Result<Option<Zeroizing<String>>, String> {
    let (open_items, locked) = items(c)?;
    let Some(item) = open_items.first() else {
        return if locked.is_empty() { Ok(None) } else { Err(locked_words().into()) };
    };
    let s = open(c)?;
    let got = call(c, item.as_str(), ITEM, "GetSecret", &(&s,)).and_then(|m| m.body().deserialize::<Secret>().map_err(|e| e.to_string()));
    close(c, &s);
    let mut secret = got?;
    let key = String::from_utf8(std::mem::take(&mut secret.2)).map(Zeroizing::new).map_err(|_| t!("store-key-not-text").to_string())?;
    Ok(Some(key).filter(|k| !k.trim().is_empty()))
}

pub fn keep(c: &Connection, key: &str) -> Result<(), String> {
    let s = open(c)?;
    let attributes: HashMap<String, String> = KIND.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    let props: HashMap<String, Value> = HashMap::from([
        ("org.freedesktop.Secret.Item.Label".to_string(), Value::from(LABEL)),
        ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(attributes)),
    ]);
    let secret: Secret = (s.clone(), Vec::new(), key.trim().as_bytes().to_vec(), "text/plain".into());
    let made = call(c, DEFAULT, COLLECTION, "CreateItem", &(props, secret, true));
    close(c, &s);
    let (_, prompt): (OwnedObjectPath, OwnedObjectPath) = made?.body().deserialize().map_err(|e| e.to_string())?;

    if prompt.as_str() != "/" {
        return Err(locked_words().into());
    }
    Ok(())
}

pub fn forget(c: &Connection) -> Result<(), String> {
    let (open_items, locked) = items(c)?;
    if !locked.is_empty() {
        return Err(locked_words().into());
    }
    for item in open_items {
        let (prompt,): (OwnedObjectPath,) = call(c, item.as_str(), ITEM, "Delete", &())?.body().deserialize().map_err(|e| e.to_string())?;
        if prompt.as_str() != "/" {
            return Err(locked_words().into());
        }
    }
    Ok(())
}

pub fn move_old(c: Result<&Connection, &String>, old: &Path, copies: &[std::path::PathBuf]) -> Result<(), String> {

    for copy in copies {
        match std::fs::remove_file(copy) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(t!("store-key-not-removed", path = copy.display().to_string(), why = e.to_string())),
            _ => {}
        }
    }
    let text = match std::fs::read_to_string(old) {
        Ok(text) => Zeroizing::new(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(t!("store-key-not-read", path = old.display().to_string(), why = e.to_string())),
    };
    if !text.trim().is_empty() {
        let c = c.map_err(String::clone)?;

        if read(c)?.is_none() {
            keep(c, text.trim())?;
        }
    }
    std::fs::remove_file(old).map_err(|e| t!("store-key-not-removed", path = old.display().to_string(), why = e.to_string()))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use crate::keyring::service::{op, Keyring, GNOME};
    use crate::keyring::store::Store;
    use crate::toolkit::bus::testing::{own_daemon, until};
    use crate::toolkit::bus::Bus;
    use crate::toolkit::event::Loop;

    pub fn with_keyring(name: &str, unlocked: bool, client: impl FnOnce(String, std::path::PathBuf) + Send + 'static) {
        let d = own_daemon();

        assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-apps-key-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Store::create(&dir.join("keyring"), b"test").unwrap();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let _ring = Keyring::start(&bus, &lp, dir.join("keyring")).unwrap();
        let owned = std::rc::Rc::new(std::cell::Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get().is_some()));
        assert_eq!(owned.get(), Some(true), "the test's keyring has its name");
        let done = Arc::new(AtomicBool::new(false));
        let (flag, address, folder) = (done.clone(), d.1.clone(), dir.clone());
        let thread = std::thread::spawn(move || {
            struct Done(Arc<AtomicBool>);
            impl Drop for Done {
                fn drop(&mut self) {
                    self.0.store(true, Ordering::SeqCst);
                }
            }
            let _done = Done(flag);
            if unlocked {
                let c = zbus::blocking::connection::Builder::address(address.as_str()).unwrap().method_timeout(WAIT).build().unwrap();
                let s = open(&c).unwrap();
                let secret: Secret = (s, Vec::new(), b"test".to_vec(), "text/plain".to_string());
                c.call_method(Some(NAME), ROOT, Some(GNOME), "UnlockWithMasterPassword",
                              &(op("/org/freedesktop/secrets/collection/login"), secret)).unwrap();
            }
            client(address, folder);
        });
        for _ in 0..24 {
            let d = done.clone();
            if until(&lp, move || d.load(Ordering::SeqCst)) {
                break;
            }
        }
        assert!(done.load(Ordering::SeqCst), "the client finished");
        thread.join().expect("the client's checks held");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn connect(address: &str) -> Connection {
        zbus::blocking::connection::Builder::address(address).unwrap().method_timeout(WAIT).build().unwrap()
    }

    #[test]
    fn the_key_is_kept_in_the_keyring_and_nowhere_else() {
        with_keyring("kept", true, |address, _| {
            let c = connect(&address);
            assert_eq!(read(&c), Ok(None), "none kept yet");
            assert_eq!(forget(&c), Ok(()), "removing none is no failure");
            keep(&c, " example-key-one \n").unwrap();
            assert_eq!(read(&c).unwrap().as_deref().map(String::as_str), Some("example-key-one"));

            let s = open(&c).unwrap();
            let other: HashMap<String, Value> = HashMap::from([
                ("org.freedesktop.Secret.Item.Label".to_string(), Value::from("Example")),
                ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(HashMap::from([("service".to_string(), "example".to_string())]))),
            ]);
            let secret: Secret = (s.clone(), Vec::new(), b"not the key".to_vec(), "text/plain".into());
            call(&c, DEFAULT, COLLECTION, "CreateItem", &(other, secret, true)).unwrap();
            close(&c, &s);
            keep(&c, "example-key-two").unwrap();
            assert_eq!(read(&c).unwrap().as_deref().map(String::as_str), Some("example-key-two"), "the newer takes the older's place");
            assert_eq!(items(&c).unwrap().0.len(), 1, "one key, not two");
            assert_eq!(forget(&c), Ok(()));
            assert_eq!(read(&c), Ok(None));
            let ask: HashMap<&str, &str> = HashMap::from([("service", "example")]);
            let (left, _): (Vec<OwnedObjectPath>, Vec<OwnedObjectPath>) = call(&c, ROOT, SERVICE, "SearchItems", &(ask,)).unwrap().body().deserialize().unwrap();
            assert_eq!(left.len(), 1, "the other secret is still there");
        });
    }

    #[test]
    fn a_key_left_in_a_plain_file_is_moved_into_the_keyring_and_the_file_removed() {
        with_keyring("moved", true, |address, dir| {
            let c = connect(&address);
            let (old, copy) = (dir.join("github-key"), dir.join("headers"));
            std::fs::write(&old, "example-key-old\n").unwrap();
            std::fs::write(&copy, "Authorization: Bearer example-key-old\n").unwrap();
            assert_eq!(move_old(Ok(&c), &old, &[copy.clone(), dir.join("never-there")]), Ok(()));
            assert!(!old.exists() && !copy.exists(), "neither plain file is left");
            assert_eq!(read(&c).unwrap().as_deref().map(String::as_str), Some("example-key-old"));

            assert_eq!(move_old(Ok(&c), &old, std::slice::from_ref(&copy)), Ok(()));
            assert_eq!(read(&c).unwrap().as_deref().map(String::as_str), Some("example-key-old"));

            keep(&c, "example-key-new").unwrap();
            std::fs::write(&old, "example-key-old\n").unwrap();
            assert_eq!(move_old(Ok(&c), &old, &[]), Ok(()));
            assert!(!old.exists());
            assert_eq!(read(&c).unwrap().as_deref().map(String::as_str), Some("example-key-new"));

            std::fs::write(&old, "\n").unwrap();
            assert_eq!(move_old(Err(&"no keyring".to_string()), &old, &[]), Ok(()));
            assert!(!old.exists());

            std::fs::write(&old, "example-key-old\n").unwrap();
            std::fs::write(&copy, "Authorization: Bearer example-key-old\n").unwrap();
            assert_eq!(move_old(Err(&"no keyring".to_string()), &old, std::slice::from_ref(&copy)), Err("no keyring".to_string()));
            assert!(old.exists() && !copy.exists());
        });
    }

    #[test]
    fn a_locked_keyring_is_said_and_the_plain_file_is_left() {
        with_keyring("locked", false, |address, dir| {
            let c = connect(&address);
            assert_eq!(keep(&c, "example-key"), Err(locked_words().to_string()));
            let old = dir.join("github-key");
            std::fs::write(&old, "example-key-old\n").unwrap();
            assert_eq!(move_old(Ok(&c), &old, &[]), Err(locked_words().to_string()));
            assert_eq!(std::fs::read_to_string(&old).unwrap(), "example-key-old\n", "still there to move another time");
        });

        assert!(ring().bus().is_err());
    }
}
