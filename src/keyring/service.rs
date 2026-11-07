//! The Secret Service on the session bus, as programs expect to find it at org.freedesktop.secrets.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap};
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use serde::Serialize;
use zbus::zvariant::{DynamicType, OwnedObjectPath, OwnedValue, Value};
use zbus::Message;

use super::store::{self, Error, Store};
use super::pam;
use crate::toolkit::bus::{Asked, Bus, Listen, Props, Served};
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::window::Display;

pub const NAME: &str = "org.freedesktop.secrets";
pub const ROOT: &str = "/org/freedesktop/secrets";
mod methods;
mod password;
mod prompts;
mod props;
mod tree;
mod xml;

const COLLECTIONS: &str = "/org/freedesktop/secrets/collection/";
const ALIASES: &str = "/org/freedesktop/secrets/aliases/";
const SESSIONS: &str = "/org/freedesktop/secrets/session/";
const PROMPTS: &str = "/org/freedesktop/secrets/prompt/";

pub const SERVICE: &str = "org.freedesktop.Secret.Service";
pub const COLLECTION: &str = "org.freedesktop.Secret.Collection";
pub const ITEM: &str = "org.freedesktop.Secret.Item";
const SESSION: &str = "org.freedesktop.Secret.Session";
const PROMPT: &str = "org.freedesktop.Secret.Prompt";
pub const GNOME: &str = "org.gnome.keyring.InternalUnsupportedGuiltRiddenInterface";
pub const PORTAL: &str = "org.freedesktop.impl.portal.Secret";
const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
const PORTAL_SECRET: usize = 64;
const PORTAL_SCHEMA: &str = "org.freedesktop.portal.Secret";

const IS_LOCKED: &str = "org.freedesktop.Secret.Error.IsLocked";
const NO_SESSION: &str = "org.freedesktop.Secret.Error.NoSession";
const NO_SUCH: &str = "org.freedesktop.Secret.Error.NoSuchObject";
const NOT_SUPPORTED: &str = "org.freedesktop.DBus.Error.NotSupported";
const INVALID: &str = "org.freedesktop.DBus.Error.InvalidArgs";
const FAILED: &str = "org.freedesktop.DBus.Error.Failed";
const MOST_HELD: usize = 256;

pub const LABEL_COLLECTION: &str = "org.freedesktop.Secret.Collection.Label";
pub const LABEL_ITEM: &str = "org.freedesktop.Secret.Item.Label";
pub const ATTRIBUTES: &str = "org.freedesktop.Secret.Item.Attributes";

pub type Secret = (OwnedObjectPath, Vec<u8>, Vec<u8>, String);

enum Work {
    Unlock(Vec<OwnedObjectPath>),
    Create { label: String, alias: String },
    Portal { app_id: String, fd: OwnedFd, asked: Option<Asked> },
}

enum Target {
    Service,
    Collection(String),
    Item(String, u64),
    Session(String),
    Prompt(String),
    Portal,
}

struct Fault(&'static str, String);

impl From<Error> for Fault {
    fn from(e: Error) -> Fault {
        let name = match e {
            Error::Locked => IS_LOCKED,
            Error::NoSuch => NO_SUCH,
            Error::WrongPassword => INVALID,
            Error::Damaged | Error::Io(_) => FAILED,
        };
        Fault(name, e.to_string())
    }
}

fn fault(name: &'static str, words: &str) -> Fault {
    Fault(name, words.to_string())
}

pub fn op(path: &str) -> OwnedObjectPath {
    OwnedObjectPath::try_from(path).unwrap_or_else(|_| none())
}

pub fn none() -> OwnedObjectPath {
    OwnedObjectPath::try_from("/").expect("\"/\" is an object path")
}

fn collection_path(name: &str) -> String {
    format!("{COLLECTIONS}{name}")
}

fn item_path(collection: &str, id: u64) -> String {
    format!("{COLLECTIONS}{collection}/{id}")
}

fn path_safe(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn name_for(label: &str, taken: impl Fn(&str) -> bool) -> String {
    let base: String = label.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect();
    let base = if base.chars().all(|c| c == '_') { "collection".to_string() } else { base };
    let mut name = base.clone();
    let mut n = 2;
    while taken(&name) {
        name = format!("{base}_{n}");
        n += 1;
    }
    name
}

fn login_name() -> String {
    // SAFETY: getuid cannot fail. getpwuid returns null or a pointer to a
    // static record, read at once before any other call could replace it.
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        if pw.is_null() || (*pw).pw_name.is_null() {
            return crate::common::files::user();
        }
        std::ffi::CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned()
    }
}

pub(crate) fn reseal(store: &mut Store, key: store::Key) -> bool {
    if store.is_locked() {
        return false;
    }
    if store.key().as_ref() == Some(&key) {
        return true;
    }
    let salt = store.salt();
    store.rekey_with(salt, key).is_ok()
}

pub(crate) fn why(no_keyring: bool, key_did_not_fit: bool, key_left: bool) -> &'static str {
    if no_keyring {
        t!("keyring-why-none")
    } else if key_did_not_fit {
        t!("keyring-why-old")
    } else if !key_left {
        t!("keyring-why-no-login")
    } else {
        t!("keyring-why-wanted")
    }
}

fn login_key() -> Option<store::Key> {
    if cfg!(test) { None } else { pam::take_key() }
}

fn args<T: serde::de::DeserializeOwned + zbus::zvariant::Type>(msg: &Message, wanted: &str) -> Result<T, Fault> {
    let body = msg.body();
    body.deserialize::<T>().map_err(|_| fault(INVALID, wanted))
}

fn answer<B: Serialize + DynamicType>(asked: &mut Option<Asked>, body: &B) {
    if let Some(a) = asked.take() {
        a.reply(body);
    }
}

fn props(pairs: Vec<(&str, Value<'_>)>) -> Props {
    pairs.into_iter().filter_map(|(k, v)| Some((k.to_string(), OwnedValue::try_from(v).ok()?))).collect()
}

fn portal_answer(ok: bool) -> (u32, HashMap<String, OwnedValue>) {
    (if ok { 0 } else { 1 }, HashMap::new())
}

pub struct Keyring {
    me: Weak<Keyring>,
    bus: Bus,
    events: Loop,
    display: RefCell<Option<Rc<Display>>>,
    path: PathBuf,
    store: RefCell<Option<Store>>,
    damaged: Cell<bool>,
    sessions: RefCell<HashMap<String, String>>,
    prompts: RefCell<HashMap<String, Work>>,
    asking: RefCell<Vec<String>>,
    served: RefCell<BTreeMap<String, Vec<Served>>>,
    counter: Cell<u64>,
    watching: RefCell<Option<Listen>>,
    working: Cell<u32>,
    told: RefCell<Option<Box<dyn Fn()>>>,
    pending: RefCell<Option<store::Key>>,
    key_left: Cell<bool>,
    held_back: RefCell<Option<Vec<(String, String, Asked)>>>,
    window: RefCell<Option<Box<dyn Fn()>>>,
}

enum Asker {
    Bus(String),
    App(String),
}

pub struct Held {
    pub key: Option<store::Key>,
    pub counter: u64,
    pub sessions: Vec<(String, String)>,
}

impl Keyring {
    pub fn start(bus: &Bus, events: &Loop, path: PathBuf) -> Result<Rc<Keyring>, String> {
        Self::start_with(bus, events, path, login_key)
    }

    pub fn start_with(bus: &Bus, events: &Loop, path: PathBuf, key: impl FnOnce() -> Option<store::Key>) -> Result<Rc<Keyring>, String> {
        let (mut found, damaged) = match Store::load(&path) {
            Ok(s) => (Some(s), false),
            Err(_) if !path.exists() && !store::backup(&path).exists() => (None, false),
            Err(e) => {
                eprintln!("trindle-keyring: {}: {e}", path.display());
                (None, true)
            }
        };
        let given = key();
        let key_left = Cell::new(given.is_some());
        let pending = RefCell::new(None);
        if let (Some(s), Some(key)) = (found.as_mut(), given) {
            match s.unlock_with_key(key.clone()) {
                Ok(()) => {
                    if !cfg!(test) {
                        pam::clear_key();
                    }
                }
                Err(e) => {
                    eprintln!("trindle-keyring: the key from login did not open the keyring: {e}");
                    if !cfg!(test) {
                        pam::put_back(&key);
                    }
                    *pending.borrow_mut() = Some(key);
                }
            }
        }
        let keyring = Rc::new_cyclic(|me| Keyring {
            me: me.clone(),
            bus: bus.clone(),
            events: events.clone(),
            display: RefCell::default(),
            path,
            store: RefCell::new(found),
            damaged: Cell::new(damaged),
            sessions: RefCell::default(),
            prompts: RefCell::default(),
            asking: RefCell::default(),
            served: RefCell::default(),
            counter: Cell::new(0),
            watching: RefCell::default(),
            working: Cell::new(0),
            told: RefCell::default(),
            pending,
            key_left,
            held_back: RefCell::default(),
            window: RefCell::default(),
        });
        keyring.serve(ROOT, &[SERVICE, GNOME])?;
        keyring.serve(PORTAL_PATH, &[PORTAL])?;
        keyring.sync();
        keyring.watch_programs();
        Ok(keyring)
    }

    pub fn resting(&self) -> bool {
        self.prompts.borrow().is_empty() && self.asking.borrow().is_empty() && !self.working()
    }

    pub fn working(&self) -> bool {
        self.working.get() > 0
    }

    pub fn when_changed(&self, told: Option<Box<dyn Fn()>>) {
        *self.told.borrow_mut() = told;
    }

    pub fn held(&self) -> Held {
        self.finish_all(false);
        let waiting: Vec<String> = self.prompts.borrow().keys().cloned().collect();
        for path in waiting {
            self.complete(&path, false);
        }
        self.holding()
    }

    pub fn holding(&self) -> Held {
        Held {
            key: self.store.borrow().as_ref().and_then(Store::key),
            counter: self.counter.get(),
            sessions: self.sessions.borrow().iter().map(|(p, o)| (p.clone(), o.clone())).collect(),
        }
    }

    pub fn adopt(&self, held: Held) -> Result<(), String> {
        self.key_left.set(true);
        if let Some(key) = held.key {
            let mut store = Store::load(&self.path).map_err(|e| e.to_string())?;
            store.unlock_with_key(key).map_err(|e| e.to_string())?;
            *self.store.borrow_mut() = Some(store);
            self.damaged.set(false);
        }
        self.counter.set(self.counter.get().max(held.counter));
        for (path, owner) in held.sessions {
            if !path.starts_with(SESSIONS) || self.sessions.borrow().contains_key(&path) {
                continue;
            }
            self.serve(&path, &[SESSION])?;
            self.sessions.borrow_mut().insert(path, owner);
        }
        self.sync();
        Ok(())
    }

    pub fn hold_calls(&self) {
        *self.held_back.borrow_mut() = Some(Vec::new());
    }

    pub fn answer_held(&self) {
        let Some(held) = self.held_back.borrow_mut().take() else { return };
        for (path, iface, asked) in held {
            self.call(&path, &iface, asked);
        }
    }

    fn next(&self) -> u64 {
        self.counter.set(self.counter.get() + 1);
        self.counter.get()
    }

    fn locked(&self) -> bool {
        self.store.borrow().as_ref().is_none_or(Store::is_locked)
    }

    fn change<T>(&self, work: impl FnOnce(&mut Store) -> Result<T, Error>) -> Result<T, Fault> {
        let mut guard = self.store.borrow_mut();
        let s = guard.as_mut().ok_or_else(|| fault(IS_LOCKED, "The keyring is locked"))?;
        let out = work(s)?;
        s.save()?;
        Ok(out)
    }
}

pub fn run() -> i32 {
    if std::env::args().nth(1).as_deref() == Some(crate::common::renewed::RENEWED) {
        return super::renew::take_over();
    }
    let events = Loop::new();
    let Ok(bus) = Bus::session(&events) else { return 1 };
    super::migrate::clear_gnome();
    let keyring = match Keyring::start(&bus, &events, store::default_path()) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("trindle-keyring: {e}");
            return 1;
        }
    };
    let q = events.clone();
    bus.own_firmly(NAME, &events, move |ok| {
        if ok {
            super::migrate::end_spare_gnome();
        } else {
            eprintln!("trindle-keyring: another keyring has {NAME}");
            q.quit();
        }
    });
    serve_until_over(&keyring, &bus, &events);
    0
}

pub(super) fn serve_until_over(keyring: &Rc<Keyring>, bus: &Bus, events: &Loop) {
    let handing = super::renew::watch(keyring, bus, events);
    let q = events.clone();
    let _lost = bus.listen(Some("org.freedesktop.DBus"), "org.freedesktop.DBus", Some("NameLost"), None, move |m| {
        if m.body().deserialize::<String>().is_ok_and(|n| n == NAME) && !handing.get() {
            eprintln!("trindle-keyring: another keyring has {NAME}");
            q.quit();
        }
    });
    events.run();
}

#[cfg(test)]
mod tests {
    use super::xml::xml_for;
    use super::*;
    use crate::toolkit::bus::testing::{own_daemon, until};
    use crate::toolkit::bus::Offer;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn the_interfaces_are_there() {
        for name in [SERVICE, COLLECTION, ITEM, SESSION, PROMPT, GNOME, PORTAL] {
            let xml = xml_for(name).unwrap_or_default();
            assert!(xml.contains(&format!("name=\"{name}\"")), "{name}");
        }
    }

    #[test]
    fn collection_names_suit_paths() {
        assert_eq!(name_for("My Passwords!", |_| false), "my_passwords_");
        assert_eq!(name_for("", |_| false), "collection");
        assert_eq!(name_for("login", |n| n == "login"), "login_2");
        assert!(path_safe(&name_for("Ünïcode ✓", |_| false)));
    }

    #[test]
    fn a_secret_travels_as_oayays() {
        assert_eq!(<Secret as zbus::zvariant::Type>::SIGNATURE.to_string(), "(oayays)");
    }

    struct Done(Arc<AtomicBool>);

    impl Drop for Done {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn a_running_keyring_answers() {
        let d = own_daemon();
        assert!(!d.1.is_empty());
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        Store::create(&dir.join("keyring"), b"test").unwrap();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start(&bus, &lp, dir.join("keyring")).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get().is_some()));
        assert_eq!(owned.get(), Some(true));

        let done = Arc::new(AtomicBool::new(false));
        let (flag, addr) = (done.clone(), d.1.clone());
        let client = std::thread::spawn(move || {
            let _done = Done(flag);
            client(&addr);
        });
        for _ in 0..24 {
            let d = done.clone();
            if until(&lp, move || d.load(Ordering::SeqCst)) {
                break;
            }
        }
        let joined = client.join();
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
        if let Err(e) = joined {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn the_ring_is_asked_to_give_way_as_the_password_box_is_about_to_open() {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-gives-way-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        Store::create(&dir.join("keyring"), b"test").unwrap();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start(&bus, &lp, dir.join("keyring")).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get() == Some(true)));
        let ring = Bus::at(&lp, &d.1).unwrap();
        let asked_of_ring = Rc::new(RefCell::new(Vec::<String>::new()));
        let a = asked_of_ring.clone();
        let _served = ring.serve(crate::ring::BUS_PATH, Offer {
            name: crate::ring::BUS_NAME.into(),
            xml: crate::ring::XML.into(),
            call: Box::new(move |asked: Asked| {
                a.borrow_mut().push(asked.member().as_str().to_string());
                asked.reply(&());
            }),
            props: Box::new(Props::new),
            set: None,
        }).unwrap();
        let named = Rc::new(Cell::new(false));
        let n = named.clone();
        ring.own(crate::ring::BUS_NAME, false, &lp, move |ok| n.set(ok));
        assert!(until(&lp, move || named.get()), "the stand-in ring has its name");
        let done = Arc::new(AtomicBool::new(false));
        let (flag, addr) = (done.clone(), d.1.clone());
        let program = std::thread::spawn(move || {
            let _done = Done(flag);
            let conn = zbus::blocking::connection::Builder::address(addr.as_str()).unwrap()
                .method_timeout(std::time::Duration::from_secs(5)).build().unwrap();
            let (_, prompt): (Vec<OwnedObjectPath>, OwnedObjectPath) = conn.call_method(Some(NAME), ROOT, Some(SERVICE), "Unlock",
                &(vec![op("/org/freedesktop/secrets/collection/login")],)).unwrap().body().deserialize().unwrap();
            assert_ne!(prompt.as_str(), "/", "a locked keyring gave no question to ask");
            conn.call_method(Some(NAME), prompt.as_str(), Some(PROMPT), "Prompt", &("",)).unwrap();
        });
        let d2 = done.clone();
        assert!(until(&lp, move || d2.load(Ordering::SeqCst)), "the program was not answered");
        let a = asked_of_ring.clone();
        until(&lp, move || !a.borrow().is_empty());
        let joined = program.join();
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
        if let Err(e) = joined {
            std::panic::resume_unwind(e);
        }
        assert_eq!(*asked_of_ring.borrow(), ["CloseRing"], "the ring was not asked to give way, or was asked for more");
    }

    fn a_locked_keyring(tag: &str) -> (crate::toolkit::bus::testing::Daemon, PathBuf, Loop, Rc<Keyring>) {
        let d = own_daemon();
        assert_ne!(Some(d.1.clone()), std::env::var("DBUS_SESSION_BUS_ADDRESS").ok(), "never the real bus");
        let dir = std::env::temp_dir().join(format!("trindle-keyring-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        Store::create(&dir.join("keyring"), b"test").unwrap();
        let lp = Loop::new();
        let bus = Bus::at(&lp, &d.1).unwrap();
        let keyring = Keyring::start(&bus, &lp, dir.join("keyring")).unwrap();
        let owned = Rc::new(Cell::new(None));
        let o = owned.clone();
        bus.own_firmly(NAME, &lp, move |ok| o.set(Some(ok)));
        let o = owned.clone();
        assert!(until(&lp, move || o.get() == Some(true)));
        (d, dir, lp, keyring)
    }

    fn as_a_program(lp: &Loop, addr: &str, program: impl FnOnce(&zbus::blocking::Connection) + Send + 'static) {
        let done = Arc::new(AtomicBool::new(false));
        let (flag, addr) = (done.clone(), addr.to_string());
        let t = std::thread::spawn(move || {
            let _done = Done(flag);
            let conn = zbus::blocking::connection::Builder::address(addr.as_str()).unwrap()
                .method_timeout(Duration::from_secs(5)).build().unwrap();
            program(&conn);
        });
        assert!(until(lp, move || done.load(Ordering::SeqCst)), "the program was not answered");
        if let Err(e) = t.join() {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn a_password_box_nothing_needs_closes() {
        let (d, dir, lp, keyring) = a_locked_keyring("box-closes");
        let shut = Rc::new(Cell::new(0u32));
        let up = |work: Work| {
            let path = keyring.new_prompt(work).ok().unwrap().as_str().to_string();
            keyring.asking.borrow_mut().push(path.clone());
            let s = shut.clone();
            *keyring.window.borrow_mut() = Some(Box::new(move || s.set(s.get() + 1)));
            path
        };
        let path = up(Work::Unlock(Vec::new()));
        as_a_program(&lp, &d.1, move |c| {
            c.call_method(Some(NAME), path.as_str(), Some(PROMPT), "Dismiss", &()).unwrap();
        });
        assert_eq!(shut.get(), 1, "the box stayed up once the program that asked had withdrawn");
        up(Work::Unlock(vec![op("/org/freedesktop/secrets/collection/login")]));
        as_a_program(&lp, &d.1, |c| {
            c.call_method(Some(NAME), ROOT, Some(GNOME), "UnlockWithMasterPassword",
                &(op("/org/freedesktop/secrets/collection/login"), (none(), Vec::<u8>::new(), b"test".to_vec(), "text/plain"))).unwrap();
        });
        assert_eq!(shut.get(), 2, "the box stayed up over a keyring opened another way");
        assert!(keyring.resting(), "a question was left waiting");
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_keyring_taken_over_after_an_update_never_says_no_password_was_typed() {
        let (_d, dir, _lp, keyring) = a_locked_keyring("taken-why");
        keyring.key_left.set(false);
        keyring.adopt(Held { key: None, counter: 0, sessions: Vec::new() }).unwrap();
        assert_eq!(keyring.why_asked(), t!("keyring-why-wanted"));
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_password_box_names_the_program_that_asked() {
        let (d, dir, lp, keyring) = a_locked_keyring("names");
        let program = zbus::blocking::connection::Builder::address(d.1.as_str()).unwrap().build().unwrap();
        let named = Rc::new(RefCell::new(None));
        let n = named.clone();
        keyring.program_name(Asker::Bus(program.unique_name().unwrap().to_string()), move |p| *n.borrow_mut() = Some(p));
        let n = named.clone();
        assert!(until(&lp, move || n.borrow().is_some()));
        let comm = std::fs::read_to_string("/proc/self/comm").unwrap().trim().to_string();
        let got = named.borrow_mut().take().flatten().expect("no name was found for the program");
        assert_eq!(got, format!("a program that calls itself {comm}"), "a name a program gives itself is shown as its claim");
        let n = named.clone();
        keyring.program_name(Asker::App("org.example.NotInstalled".into()), move |p| *n.borrow_mut() = Some(p));
        assert_eq!(named.borrow_mut().take().flatten().as_deref(), Some("org.example.NotInstalled"));
        drop(keyring);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn client(addr: &str) {
        let c = Client(zbus::blocking::connection::Builder::address(addr).unwrap()
            .method_timeout(Duration::from_secs(5)).build().unwrap());
        let login = "/org/freedesktop/secrets/collection/login";
        let (_, s): (OwnedValue, OwnedObjectPath) =
            c.call(ROOT, SERVICE, "OpenSession", &("plain", Value::from(""))).unwrap().body().deserialize().unwrap();
        unlocks_with_the_master_password(&c, &s, login);
        let item = items_are_made_found_and_read(&c, &s, login);
        a_locked_collection_keeps_its_items(&c, &s, login, &item);
        collections_come_and_go(&c);
        the_portal_gives_each_program_its_secret(&c);
        deleting_an_item_and_closing_the_session(&c, &s, &item);
    }

    struct Client(zbus::blocking::Connection);

    impl Client {
        fn call(&self, path: &str, iface: &str, method: &str, body: &dyn Body) -> zbus::Result<Message> {
            body.send(&self.0, path, iface, method)
        }

        fn get(&self, path: &str, iface: &str, prop: &str) -> OwnedValue {
            self.call(path, "org.freedesktop.DBus.Properties", "Get", &(iface, prop)).unwrap()
                .body().deserialize::<OwnedValue>().unwrap()
        }

        fn locked(&self, path: &str) -> bool {
            bool::try_from(self.get(path, COLLECTION, "Locked")).unwrap()
        }

        fn label(&self, path: &str) -> String {
            String::try_from(self.get(path, ITEM, "Label")).unwrap()
        }
    }

    fn secret(s: &OwnedObjectPath, v: &[u8]) -> Secret {
        (s.clone(), vec![], v.to_vec(), "text/plain".into())
    }

    fn unlocks_with_the_master_password(c: &Client, s: &OwnedObjectPath, login: &str) {

        assert!(c.locked(login));
        assert!(c.call(ROOT, GNOME, "UnlockWithMasterPassword", &(op(login), secret(s, b"no"))).is_err());
        c.call(ROOT, GNOME, "UnlockWithMasterPassword", &(op(login), secret(s, b"test"))).unwrap();
        assert!(!c.locked(login));
    }

    fn items_are_made_found_and_read(c: &Client, s: &OwnedObjectPath, login: &str) -> OwnedObjectPath {

        let attrs = HashMap::from([("service".to_string(), "demo".to_string()), ("user".to_string(), "john".to_string())]);
        let props = |label: &str| HashMap::from([
            ("org.freedesktop.Secret.Item.Label".to_string(), Value::from(label.to_string())),
            ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(attrs.clone())),
        ]);
        let (item, _): (OwnedObjectPath, OwnedObjectPath) = c.call("/org/freedesktop/secrets/aliases/default", COLLECTION,
            "CreateItem", &(props("Test"), secret(s, b"hi"), true)).unwrap().body().deserialize().unwrap();
        let (again, _): (OwnedObjectPath, OwnedObjectPath) = c.call(login, COLLECTION, "CreateItem",
            &(props("Test 2"), secret(s, b"ho"), true)).unwrap().body().deserialize().unwrap();
        assert_eq!(item, again, "replace keeps the one item");
        assert_eq!(c.label(item.as_str()), "Test 2");
        let found: Vec<OwnedObjectPath> = c.call(login, COLLECTION, "SearchItems",
            &(HashMap::from([("user", "john")]),)).unwrap().body().deserialize().unwrap();
        assert_eq!(found, vec![item.clone()]);
        let reply = c.call(item.as_str(), ITEM, "GetSecret", &(s.clone(),)).unwrap();
        assert_eq!(reply.body().signature().to_string(), "(oayays)");
        assert!(reply.data().windows(10).any(|w| w == b"\x08(oayays)\0"), "one struct, as programs expect");
        let got: Secret = reply.body().deserialize().unwrap();
        assert_eq!(got.2, b"ho");
        c.call(item.as_str(), ITEM, "SetSecret", &(secret(s, b"hey"),)).unwrap();
        c.call(item.as_str(), "org.freedesktop.DBus.Properties", "Set", &(ITEM, "Label", Value::from("Renamed"))).unwrap();
        assert_eq!(c.label(item.as_str()), "Renamed");
        let all: HashMap<OwnedObjectPath, Secret> = c.call(ROOT, SERVICE, "GetSecrets",
            &(vec![item.clone()], s.clone())).unwrap().body().deserialize().unwrap();
        assert_eq!(all[&item].2, b"hey");
        item
    }

    fn a_locked_collection_keeps_its_items(c: &Client, s: &OwnedObjectPath, login: &str, item: &OwnedObjectPath) {

        c.call(ROOT, SERVICE, "Lock", &(vec![op(login)],)).unwrap();
        let (open, shut): (Vec<OwnedObjectPath>, Vec<OwnedObjectPath>) = c.call(ROOT, SERVICE, "SearchItems",
            &(HashMap::from([("service", "demo")]),)).unwrap().body().deserialize().unwrap();
        assert!(open.is_empty());
        assert_eq!(shut, vec![item.clone()], "a locked item is still found");
        assert!(c.call(item.as_str(), ITEM, "GetSecret", &(s.clone(),)).is_err());
        let change = |old: &[u8], new: &[u8]| c.call(ROOT, GNOME, "ChangeWithMasterPassword",
            &(op(login), secret(s, old), secret(s, new)));
        assert!(change(b"wrong", b"new").is_err());
        change(b"test", b"new").unwrap();
        c.call(ROOT, SERVICE, "Lock", &(vec![op(login)],)).unwrap();
        assert!(c.call(ROOT, GNOME, "UnlockWithMasterPassword", &(op(login), secret(s, b"test"))).is_err());
        c.call(ROOT, GNOME, "UnlockWithMasterPassword", &(op(login), secret(s, b"new"))).unwrap();
        let got: Secret = c.call(item.as_str(), ITEM, "GetSecret", &(s.clone(),)).unwrap().body().deserialize().unwrap();
        assert_eq!(got.2, b"hey", "the secret survives a new password");
        change(b"new", b"test").unwrap();
    }

    fn collections_come_and_go(c: &Client) {
        let (made, _): (OwnedObjectPath, OwnedObjectPath) = c.call(ROOT, SERVICE, "CreateCollection",
            &(HashMap::from([("org.freedesktop.Secret.Collection.Label".to_string(), Value::from("My Stuff"))]), "work"))
            .unwrap().body().deserialize().unwrap();
        let alias: OwnedObjectPath = c.call(ROOT, SERVICE, "ReadAlias", &("work",)).unwrap().body().deserialize().unwrap();
        assert_eq!(alias, made);
        c.call(made.as_str(), COLLECTION, "Delete", &()).unwrap();
        let alias: OwnedObjectPath = c.call(ROOT, SERVICE, "ReadAlias", &("work",)).unwrap().body().deserialize().unwrap();
        assert_eq!(alias.as_str(), "/");
    }

    fn the_portal_gives_each_program_its_secret(c: &Client) {
        use std::io::Read;
        use std::os::fd::AsFd;
        let portal = |app: &str| -> Vec<u8> {
            let (mut ours, theirs) = std::os::unix::net::UnixStream::pair().unwrap();
            ours.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let reply = c.call(PORTAL_PATH, PORTAL, "RetrieveSecret", &(op("/org/freedesktop/portal/desktop/request/1_1/t"),
                app, zbus::zvariant::Fd::from(theirs.as_fd()), HashMap::<String, Value>::new())).unwrap();
            let (code, _): (u32, HashMap<String, OwnedValue>) = reply.body().deserialize().unwrap();
            assert_eq!(code, 0);
            drop(theirs);
            let mut got = Vec::new();
            ours.read_to_end(&mut got).unwrap();
            got
        };
        let first = portal("org.example.App");
        assert_eq!(first.len(), PORTAL_SECRET);
        assert_eq!(portal("org.example.App"), first, "the same program gets the same secret");
        assert_ne!(portal("org.example.Other"), first);
    }

    fn deleting_an_item_and_closing_the_session(c: &Client, s: &OwnedObjectPath, item: &OwnedObjectPath) {
        c.call(item.as_str(), ITEM, "Delete", &()).unwrap();
        assert!(c.call(item.as_str(), ITEM, "GetSecret", &(s.clone(),)).is_err());
        c.call(s.as_str(), SESSION, "Close", &()).unwrap();
        assert!(c.call(ROOT, SERVICE, "GetSecrets", &(Vec::<OwnedObjectPath>::new(), s.clone())).is_err());
    }

    trait Body {
        fn send(&self, conn: &zbus::blocking::Connection, path: &str, iface: &str, method: &str) -> zbus::Result<Message>;
    }

    impl<B: Serialize + DynamicType> Body for B {
        fn send(&self, conn: &zbus::blocking::Connection, path: &str, iface: &str, method: &str) -> zbus::Result<Message> {
            conn.call_method(Some(NAME), path, Some(iface), method, self)
        }
    }
}
