//! The keyring's file.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use argon2::{Algorithm, Argon2, Params, Version};
use blake2::digest::{KeyInit, Mac};
use blake2::Blake2bMac512;
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::toolkit::t;

const MAGIC: &[u8; 4] = b"RKR1";
const VERSION: u8 = 1;
pub const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
const HEAD_LEN: usize = MAGIC.len() + 1 + SALT_LEN + NONCE_LEN;

pub const ARGON: (u32, u32, u32) = (64 * 1024, 3, 1);

pub const LOGIN: &str = "login";
pub const DEFAULT_ALIAS: &str = "default";

pub type Key = Zeroizing<[u8; 32]>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Locked,
    WrongPassword,
    NoSuch,
    Damaged,
    Io(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Locked => write!(f, "{}", t!("keyring-error-locked")),
            Error::WrongPassword => write!(f, "{}", t!("keyring-error-wrong-password")),
            Error::NoSuch => write!(f, "{}", t!("keyring-error-no-such")),
            Error::Damaged => write!(f, "{}", t!("keyring-error-damaged")),
            Error::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Item {
    pub label: String,
    pub attributes: BTreeMap<String, String>,
    pub secret: Vec<u8>,
    pub content_type: String,
    pub created: u64,
    pub modified: u64,
}

impl Drop for Item {
    fn drop(&mut self) {
        zeroize::Zeroize::zeroize(&mut self.secret);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Body {
    items: BTreeMap<String, BTreeMap<u64, Item>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Index {
    pub collections: BTreeMap<String, Shelf>,
    pub aliases: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Shelf {
    pub label: String,
    pub created: u64,
    pub modified: u64,
    pub items: BTreeMap<u64, BTreeSet<String>>,
}

pub struct Store {
    path: PathBuf,
    salt: [u8; SALT_LEN],
    pub index: Index,
    open: Option<(Key, Body)>,
}

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn derive(password: &[u8], salt: &[u8]) -> Key {
    let (m, t, p) = ARGON;
    let params = Params::new(m, t, p, Some(32)).expect("the argon2 constant is valid");
    let mut key = Zeroizing::new([0u8; 32]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(password, salt, key.as_mut())
        .expect("a 16-byte salt and a 32-byte key are within argon2's limits");
    key
}

fn hash(salt: &[u8], name: &str, value: &str) -> String {
    let mut mac = <Blake2bMac512 as KeyInit>::new_from_slice(salt).expect("a 16-byte key suits blake2b");
    mac.update(name.as_bytes());
    mac.update(&[0]);
    mac.update(value.as_bytes());
    mac.finalize().into_bytes().iter().fold(String::new(), |mut s, b| {
        let _ = std::fmt::Write::write_fmt(&mut s, format_args!("{b:02x}"));
        s
    })
}

fn random<const N: usize>() -> [u8; N] {
    let mut b = [0u8; N];
    getrandom::fill(&mut b).expect("the kernel's random numbers are there");
    b
}

pub fn default_path() -> PathBuf {
    crate::common::files::data_home().join("trindle/keyring")
}

pub fn backup(path: &Path) -> PathBuf {
    path.with_file_name(format!("{}.bak", path.file_name().and_then(|n| n.to_str()).unwrap_or("keyring")))
}

pub fn salt_in(bytes: &[u8]) -> Option<[u8; SALT_LEN]> {
    if bytes.len() < HEAD_LEN || &bytes[..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    bytes[5..5 + SALT_LEN].try_into().ok()
}

pub fn new_salt() -> [u8; SALT_LEN] {
    random()
}

type Parsed<'a> = ([u8; SALT_LEN], [u8; NONCE_LEN], Index, &'a [u8], &'a [u8]);

fn parse(bytes: &[u8]) -> Result<Parsed<'_>, Error> {
    if bytes.len() < HEAD_LEN + 4 || &bytes[..4] != MAGIC || bytes[4] != VERSION {
        return Err(Error::Damaged);
    }
    let salt: [u8; SALT_LEN] = bytes[5..5 + SALT_LEN].try_into().map_err(|_| Error::Damaged)?;
    let nonce: [u8; NONCE_LEN] = bytes[5 + SALT_LEN..HEAD_LEN].try_into().map_err(|_| Error::Damaged)?;
    let len = u32::from_le_bytes(bytes[HEAD_LEN..HEAD_LEN + 4].try_into().map_err(|_| Error::Damaged)?) as usize;
    let end = HEAD_LEN + 4 + len;
    if bytes.len() < end {
        return Err(Error::Damaged);
    }
    let index = serde_json::from_slice(&bytes[HEAD_LEN + 4..end]).map_err(|_| Error::Damaged)?;
    Ok((salt, nonce, index, &bytes[..end], &bytes[end..]))
}

impl Store {
    pub fn create(path: &Path, password: &[u8]) -> Result<Store, Error> {
        let salt = random::<SALT_LEN>();
        let key = derive(password, &salt);
        let mut store = Store { path: path.to_path_buf(), salt, index: Index::default(), open: Some((key, Body::default())) };
        store.add_collection(LOGIN, "Login")?;
        store.index.aliases.insert(DEFAULT_ALIAS.into(), LOGIN.into());
        store.save()?;
        Ok(store)
    }

    pub fn load(path: &Path) -> Result<Store, Error> {
        let bytes = fs::read(path).map_err(Error::from).and_then(|b| { parse(&b)?; Ok(b) });
        let bytes = match bytes {
            Ok(b) => b,
            Err(e) => match fs::read(backup(path)) {
                Ok(b) if parse(&b).is_ok() => b,
                _ => return Err(e),
            },
        };
        let (salt, _, index, ..) = parse(&bytes)?;
        Ok(Store { path: path.to_path_buf(), salt, index, open: None })
    }

    pub fn salt(&self) -> [u8; SALT_LEN] {
        self.salt
    }

    pub fn key(&self) -> Option<Key> {
        self.open.as_ref().map(|(key, _)| key.clone())
    }

    pub fn is_locked(&self) -> bool {
        self.open.is_none()
    }

    pub fn lock(&mut self) {
        self.open = None;
    }

    pub fn unlock(&mut self, password: &[u8]) -> Result<(), Error> {
        let key = derive(password, &self.salt);
        self.unlock_with_key(key)
    }

    pub fn unlock_with_key(&mut self, key: Key) -> Result<(), Error> {
        let mut bytes = fs::read(&self.path)?;
        if parse(&bytes).map(|p| p.0) != Ok(self.salt) {
            bytes = fs::read(backup(&self.path))?;
        }
        let (salt, nonce, index, aad, sealed) = parse(&bytes)?;
        if salt != self.salt {
            return Err(Error::Damaged);
        }
        let cipher = XChaCha20Poly1305::new(key.as_ref().into());
        let plain = Zeroizing::new(cipher.decrypt(XNonce::from_slice(&nonce), Payload { msg: sealed, aad })
            .map_err(|_| Error::WrongPassword)?);
        let body = serde_json::from_slice(&plain).map_err(|_| Error::Damaged)?;
        self.index = index;
        self.open = Some((key, body));
        Ok(())
    }

    pub fn rekey_with(&mut self, salt: [u8; SALT_LEN], key: Key) -> Result<(), Error> {
        if self.open.is_none() {
            return Err(Error::Locked);
        }
        let old = (self.salt, self.open.as_ref().map(|o| o.0.clone()));
        self.salt = salt;
        if let Some(o) = self.open.as_mut() {
            o.0 = key;
        }
        self.reindex();
        if let Err(e) = self.save() {
            self.salt = old.0;
            if let (Some(o), Some(k)) = (self.open.as_mut(), old.1) {
                o.0 = k;
            }
            self.reindex();
            return Err(e);
        }
        Ok(())
    }

    fn body(&self) -> Result<&Body, Error> {
        self.open.as_ref().map(|o| &o.1).ok_or(Error::Locked)
    }

    fn body_mut(&mut self) -> Result<&mut Body, Error> {
        self.open.as_mut().map(|o| &mut o.1).ok_or(Error::Locked)
    }

    pub fn search(&self, collection: Option<&str>, attributes: &BTreeMap<String, String>) -> Vec<(String, u64)> {
        let wanted: Vec<String> = attributes.iter().map(|(n, v)| hash(&self.salt, n, v)).collect();
        let mut found = Vec::new();
        for (name, shelf) in &self.index.collections {
            if collection.is_some_and(|c| c != name) {
                continue;
            }
            for (id, hashes) in &shelf.items {
                if wanted.iter().all(|w| hashes.contains(w)) {
                    found.push((name.clone(), *id));
                }
            }
        }
        found
    }

    pub fn item(&self, collection: &str, id: u64) -> Result<&Item, Error> {
        if !self.index.collections.get(collection).is_some_and(|s| s.items.contains_key(&id)) {
            return Err(Error::NoSuch);
        }
        self.body()?.items.get(collection).and_then(|c| c.get(&id)).ok_or(Error::NoSuch)
    }

    pub fn add_collection(&mut self, name: &str, label: &str) -> Result<(), Error> {
        self.body_mut()?.items.entry(name.into()).or_default();
        let t = now();
        self.index.collections.entry(name.into())
            .or_insert_with(|| Shelf { label: label.into(), created: t, modified: t, items: BTreeMap::new() });
        Ok(())
    }

    pub fn set_collection_label(&mut self, name: &str, label: &str) -> Result<(), Error> {
        self.body()?;
        let shelf = self.index.collections.get_mut(name).ok_or(Error::NoSuch)?;
        shelf.label = label.into();
        shelf.modified = now();
        Ok(())
    }

    pub fn remove_collection(&mut self, name: &str) -> Result<(), Error> {
        self.body_mut()?.items.remove(name);
        self.index.collections.remove(name).ok_or(Error::NoSuch)?;
        self.index.aliases.retain(|_, c| c != name);
        Ok(())
    }

    pub fn put(&mut self, collection: &str, mut item: Item, replace: bool) -> Result<u64, Error> {
        self.body()?;
        if !self.index.collections.contains_key(collection) {
            return Err(Error::NoSuch);
        }
        let t = now();
        let same = if replace {
            self.search(Some(collection), &item.attributes).into_iter()
                .find(|(c, id)| self.item(c, *id).is_ok_and(|i| i.attributes == item.attributes))
                .map(|(_, id)| id)
        } else {
            None
        };
        let body = self.body_mut()?;
        let shelf = body.items.entry(collection.into()).or_default();
        let id = same.unwrap_or_else(|| shelf.keys().next_back().map_or(1, |n| n + 1));
        item.created = shelf.get(&id).map_or(t, |old| old.created);
        item.modified = t;
        shelf.insert(id, item);
        self.reindex();
        Ok(id)
    }

    pub fn update(&mut self, collection: &str, id: u64, change: impl FnOnce(&mut Item)) -> Result<(), Error> {
        let item = self.body_mut()?.items.get_mut(collection).and_then(|c| c.get_mut(&id)).ok_or(Error::NoSuch)?;
        change(item);
        item.modified = now();
        self.reindex();
        Ok(())
    }

    pub fn remove(&mut self, collection: &str, id: u64) -> Result<(), Error> {
        self.body_mut()?.items.get_mut(collection).and_then(|c| c.remove(&id)).ok_or(Error::NoSuch)?;
        self.reindex();
        Ok(())
    }

    fn reindex(&mut self) {
        let Some((_, body)) = &self.open else { return };
        for (name, shelf) in self.index.collections.iter_mut() {
            shelf.items = body.items.get(name).into_iter().flatten()
                .map(|(id, item)| (*id, item.attributes.iter().map(|(n, v)| hash(&self.salt, n, v)).collect()))
                .collect();
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.reindex();
        let (key, body) = self.open.as_ref().ok_or(Error::Locked)?;
        let nonce = random::<NONCE_LEN>();
        let index = serde_json::to_vec(&self.index).map_err(|e| Error::Io(e.to_string()))?;
        let mut out = Vec::with_capacity(HEAD_LEN + 4 + index.len());
        out.extend_from_slice(MAGIC);
        out.push(VERSION);
        out.extend_from_slice(&self.salt);
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&(index.len() as u32).to_le_bytes());
        out.extend_from_slice(&index);
        let plain = Zeroizing::new(serde_json::to_vec(body).map_err(|e| Error::Io(e.to_string()))?);
        let cipher = XChaCha20Poly1305::new(key.as_ref().into());
        let sealed = cipher.encrypt(XNonce::from_slice(&nonce), Payload { msg: &plain, aad: &out })
            .map_err(|_| Error::Io("could not seal the keyring".into()))?;
        out.extend_from_slice(&sealed);

        let dir = self.path.parent().ok_or_else(|| Error::Io("the keyring has no folder".into()))?;
        fs::create_dir_all(dir)?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;
        let temp = self.path.with_extension("new");
        let mut f = fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(&temp)?;
        f.write_all(&out)?;
        f.sync_all()?;
        drop(f);
        if self.path.exists() {
            let bak = backup(&self.path);
            let _ = fs::remove_file(&bak);
            fs::hard_link(&self.path, &bak)?;
        }
        fs::rename(&temp, &self.path)?;
        fs::File::open(dir)?.sync_all()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-keyring-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir.join("keyring")
    }

    fn attrs(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
    }

    fn secret(label: &str, pairs: &[(&str, &str)], s: &str) -> Item {
        Item { label: label.into(), attributes: attrs(pairs), secret: s.as_bytes().to_vec(),
               content_type: "text/plain".into(), created: 0, modified: 0 }
    }

    #[test]
    fn a_large_keyring_still_gives_its_salt_from_the_head() {
        let path = place("salt");
        let mut s = Store::create(&path, b"correct horse").unwrap();
        for n in 0..20 {
            s.put(LOGIN, secret(&format!("Login {n}"), &[("service", "mail"), ("n", &n.to_string())], "hunter2"), true).unwrap();
        }
        s.save().unwrap();
        let bytes = fs::read(&path).unwrap();
        assert!(bytes.len() > 256);
        assert_eq!(salt_in(&bytes[..256]), Some(s.salt()));
        assert_eq!(salt_in(b"RKR1"), None);
    }

    #[test]
    fn a_secret_comes_back_after_a_reload() {
        let path = place("round");
        let mut s = Store::create(&path, b"correct horse").unwrap();
        let id = s.put(LOGIN, secret("Mail", &[("service", "mail"), ("user", "john")], "hunter2"), true).unwrap();
        s.save().unwrap();
        assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);

        let mut again = Store::load(&path).unwrap();
        assert!(again.is_locked());
        assert_eq!(again.index.aliases.get(DEFAULT_ALIAS).map(String::as_str), Some(LOGIN));
        again.unlock(b"correct horse").unwrap();
        let item = again.item(LOGIN, id).unwrap();
        assert_eq!(item.secret, b"hunter2");
        assert_eq!(item.label, "Mail");

        let same = again.put(LOGIN, secret("Mail", &[("service", "mail"), ("user", "john")], "new"), true).unwrap();
        assert_eq!(same, id);
        assert_eq!(again.index.collections[LOGIN].items.len(), 1);
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_wrong_password_is_refused() {
        let path = place("wrong");
        Store::create(&path, b"right").unwrap();
        let mut s = Store::load(&path).unwrap();
        assert_eq!(s.unlock(b"wrong"), Err(Error::WrongPassword));
        assert!(s.is_locked());
        assert_eq!(s.put(LOGIN, Item::default(), false), Err(Error::Locked));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_search_finds_items_while_locked() {
        let path = place("search");
        let mut s = Store::create(&path, b"pw").unwrap();
        let a = s.put(LOGIN, secret("A", &[("app", "one"), ("user", "john")], "1"), false).unwrap();
        s.put(LOGIN, secret("B", &[("app", "two"), ("user", "john")], "2"), false).unwrap();
        s.save().unwrap();

        let locked = Store::load(&path).unwrap();
        assert_eq!(locked.search(None, &attrs(&[("app", "one")])), vec![(LOGIN.to_string(), a)]);
        assert_eq!(locked.search(None, &attrs(&[("user", "john")])).len(), 2);
        assert!(locked.search(None, &attrs(&[("app", "three")])).is_empty());
        assert_eq!(locked.item(LOGIN, a), Err(Error::Locked));
        let bytes = fs::read(&path).unwrap();
        assert!(!bytes.windows(4).any(|w| w == b"john"));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_new_password_opens_it_and_the_old_one_does_not() {
        let path = place("rekey");
        let mut s = Store::create(&path, b"old").unwrap();
        s.put(LOGIN, secret("A", &[("k", "v")], "x"), false).unwrap();
        let salt = new_salt();
        s.rekey_with(salt, derive(b"new", &salt)).unwrap();
        let mut again = Store::load(&path).unwrap();
        assert_eq!(again.unlock(b"old"), Err(Error::WrongPassword));
        again.unlock(b"new").unwrap();
        assert_eq!(again.search(None, &attrs(&[("k", "v")])).len(), 1);
        assert!(backup(&path).exists());
        fs::write(&path, b"nonsense").unwrap();
        assert!(Store::load(&path).is_ok());
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    #[ignore = "makes a keyring file for the tests below; run by hand with TRINDLE_KEYRING_TEST"]
    fn make_a_test_keyring() {
        let path = PathBuf::from(std::env::var("TRINDLE_KEYRING_TEST").unwrap());
        Store::create(&path, b"test").unwrap();
    }
}
