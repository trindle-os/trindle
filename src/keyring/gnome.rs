//! Reading GNOME's keyring files straight off the disk, so a person's saved logins move to Trindle's keyring without GNOME's program running.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use md5::{Digest, Md5};
use sha2::Sha256;
use zeroize::Zeroizing;

use super::store::Item;

const SEALED: &[u8] = b"GnomeKeyring\n\r\0\n";
const PLAIN: &str = "[keyring]";
const CONTENT_TYPE: &str = "text/plain";
const SCHEMA: &str = "xdg:schema";

pub struct Keyring {
    pub name: String,
    pub label: String,
    pub items: Vec<Item>,
}

#[derive(Default)]
pub struct Found {
    pub opened: Vec<Keyring>,
    pub left: Vec<String>,
    pub default: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Refused {
    WrongPassword,
    Damaged,
}

pub fn read_all(dir: &Path, passwords: &[&[u8]]) -> Found {
    let mut found = Found::default();
    let Ok(entries) = fs::read_dir(dir) else { return found };
    let mut files: Vec<_> = entries.flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "keyring") && p.is_file()).collect();
    files.sort();
    for path in files {
        let Some(name) = path.file_stem().map(|s| s.to_string_lossy().into_owned()) else { continue };
        let Ok(bytes) = fs::read(&path) else {
            found.left.push(name);
            continue;
        };
        let mut tried = passwords.iter().map(|pw| open(&name, &bytes, pw));
        match tried.find(|r| !matches!(r, Err(Refused::WrongPassword))) {
            Some(Ok(k)) => found.opened.push(k),
            _ => found.left.push(label_of(&bytes).unwrap_or(name)),
        }
    }
    found.default = fs::read_to_string(dir.join("default")).ok()
        .map(|d| d.trim().to_string()).filter(|d| found.opened.iter().any(|k| &k.name == d));
    found
}

pub fn open(name: &str, bytes: &[u8], password: &[u8]) -> Result<Keyring, Refused> {
    if bytes.starts_with(SEALED) {
        sealed(name, bytes, password)
    } else if std::str::from_utf8(bytes).is_ok_and(|t| t.trim_start().starts_with(PLAIN)) {
        Ok(plain(name, &String::from_utf8_lossy(bytes)))
    } else {
        Err(Refused::Damaged)
    }
}

fn label_of(bytes: &[u8]) -> Option<String> {
    let mut r = Reader { bytes: bytes.get(SEALED.len() + 4..)?, at: 0 };
    r.text().ok().flatten()
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], Refused> {
        let end = self.at.checked_add(n).filter(|e| *e <= self.bytes.len()).ok_or(Refused::Damaged)?;
        let got = &self.bytes[self.at..end];
        self.at = end;
        Ok(got)
    }

    fn u32(&mut self) -> Result<u32, Refused> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn time(&mut self) -> Result<u64, Refused> {
        Ok((u64::from(self.u32()?) << 32) | u64::from(self.u32()?))
    }

    fn bytes(&mut self) -> Result<Option<Vec<u8>>, Refused> {
        match self.u32()? {
            u32::MAX => Ok(None),
            n => Ok(Some(self.take(n as usize)?.to_vec())),
        }
    }

    fn text(&mut self) -> Result<Option<String>, Refused> {
        match self.bytes()? {
            None => Ok(None),
            Some(b) => String::from_utf8(b).map(Some).map_err(|_| Refused::Damaged),
        }
    }

    fn attributes(&mut self) -> Result<BTreeMap<String, String>, Refused> {
        let mut out = BTreeMap::new();
        for _ in 0..self.u32()? {
            let name = self.text()?.unwrap_or_default();
            let value = match self.u32()? {
                0 => self.text()?.unwrap_or_default(),
                1 => self.u32()?.to_string(),
                _ => return Err(Refused::Damaged),
            };
            out.insert(name, value);
        }
        Ok(out)
    }
}

fn sealed(name: &str, bytes: &[u8], password: &[u8]) -> Result<Keyring, Refused> {
    let mut r = Reader { bytes, at: SEALED.len() };
    if r.take(4)? != [0, 0, 0, 0] {
        return Err(Refused::Damaged);
    }
    let label = r.text()?.unwrap_or_else(|| name.to_string());
    r.time()?;
    r.time()?;
    let _flags = r.u32()?;
    let _lock_timeout = r.u32()?;
    let rounds = r.u32()?;
    let salt = r.take(8)?.to_vec();
    r.take(16)?;
    let count = r.u32()?;
    let mut kinds = Vec::new();
    for _ in 0..count {
        let _id = r.u32()?;
        kinds.push(r.u32()?);
        r.attributes()?;
    }
    let size = r.u32()? as usize;
    if size % 16 != 0 || size < 16 {
        return Err(Refused::Damaged);
    }
    let mut body = Zeroizing::new(r.take(size)?.to_vec());
    unseal(&mut body, password, &salt, rounds)?;

    let mut r = Reader { bytes: &body[16..], at: 0 };
    let mut items = Vec::new();
    for kind in kinds {
        let label = r.text()?.unwrap_or_default();
        let secret = Zeroizing::new(r.bytes()?.unwrap_or_default());
        let created = r.time()?;
        let modified = r.time()?;
        r.text()?;
        r.take(16)?;
        let mut attributes = r.attributes()?;
        for _ in 0..r.u32()? {
            r.u32()?;
            r.text()?;
            r.text()?;
            r.text()?;
            r.u32()?;
        }
        add_schema(&mut attributes, kind);
        items.push(Item { label, attributes, secret: secret.to_vec(), content_type: CONTENT_TYPE.into(), created, modified });
    }
    Ok(Keyring { name: name.to_string(), label, items })
}

fn unseal(body: &mut [u8], password: &[u8], salt: &[u8], rounds: u32) -> Result<(), Refused> {
    let mut digest = Zeroizing::new(<[u8; 32]>::from(Sha256::new().chain_update(password).chain_update(salt).finalize()));
    for _ in 1..rounds {
        *digest = Sha256::digest(*digest).into();
    }
    let key: [u8; 16] = digest[..16].try_into().map_err(|_| Refused::Damaged)?;
    let iv: [u8; 16] = digest[16..].try_into().map_err(|_| Refused::Damaged)?;
    cbc::Decryptor::<aes::Aes128>::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<NoPadding>(body).map_err(|_| Refused::Damaged)?;
    if Md5::digest(&body[16..])[..] != body[..16] {
        return Err(Refused::WrongPassword);
    }
    Ok(())
}

fn add_schema(attributes: &mut BTreeMap<String, String>, kind: u32) {
    let schema = match kind {
        0 => "org.freedesktop.Secret.Generic",
        1 => "org.gnome.keyring.NetworkPassword",
        2 => "org.gnome.keyring.Note",
        3 => "org.gnome.keyring.ChainedKeyring",
        4 => "org.gnome.keyring.EncryptionKey",
        0x100 => "org.gnome.keyring.PkStorage",
        _ => return,
    };
    attributes.entry(SCHEMA.into()).or_insert_with(|| schema.into());
}

fn plain(name: &str, text: &str) -> Keyring {
    let mut parts: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut order = Vec::new();
    let mut at = String::new();
    for line in text.lines() {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(part) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            at = part.to_string();
            if !parts.contains_key(&at) {
                order.push(at.clone());
            }
            parts.entry(at.clone()).or_default();
        } else if let Some((k, v)) = line.split_once('=') {
            parts.entry(at.clone()).or_default().entry(k.trim_end().to_string()).or_insert_with(|| v.to_string());
        }
    }
    let label = parts.get("keyring").and_then(|p| p.get("display-name")).map(|v| unescape(v))
        .unwrap_or_else(|| name.to_string());
    let mut items = Vec::new();
    for id in order.iter().filter(|p| !p.contains(':') && *p != "keyring") {
        let part = &parts[id];
        let secret = match (part.get("secret"), part.get("binary-secret")) {
            (Some(s), _) => s.as_bytes().to_vec(),
            (None, Some(hex)) => unhex(hex.trim()).unwrap_or_default(),
            (None, None) => Vec::new(),
        };
        let mut attributes = BTreeMap::new();
        for n in 0.. {
            let Some(a) = parts.get(&format!("{id}:attribute{n}")) else { break };
            let (Some(k), Some(v)) = (a.get("name"), a.get("value")) else { continue };
            let v = if a.get("type").is_some_and(|t| t.trim() == "uint32") { v.trim().to_string() } else { unescape(v) };
            attributes.insert(unescape(k), v);
        }
        let number = |k: &str| part.get(k).and_then(|v| v.trim().parse().ok());
        add_schema(&mut attributes, number("item-type").unwrap_or(0) as u32);
        items.push(Item {
            label: part.get("display-name").map(|v| unescape(v)).unwrap_or_default(),
            attributes,
            secret,
            content_type: CONTENT_TYPE.into(),
            created: number("ctime").unwrap_or(0),
            modified: number("mtime").unwrap_or(0),
        });
    }
    Keyring { name: name.to_string(), label, items }
}

fn unescape(v: &str) -> String {
    let mut out = String::new();
    let mut chars = v.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('s') => out.push(' '),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

fn unhex(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOGIN: &[u8] = include_bytes!("testdata/login.keyring");
    const OPEN: &[u8] = include_bytes!("testdata/Open.keyring");

    fn find<'a>(k: &'a Keyring, label: &str) -> &'a Item {
        k.items.iter().find(|i| i.label == label).unwrap()
    }

    #[test]
    fn a_sealed_keyring_opens_with_its_password() {
        let k = open("login", LOGIN, b"pw").unwrap();
        assert_eq!((k.name.as_str(), k.label.as_str(), k.items.len()), ("login", "Login", 3));
        let mail = find(&k, "Mail");
        assert_eq!(mail.secret, b"one");
        assert_eq!(mail.attributes["user"], "john@example.com");
        assert_eq!(mail.attributes[SCHEMA], "org.freedesktop.Secret.Generic");
        assert!(mail.created > 0);
        assert_eq!(find(&k, "Chat login").secret, b"two\nlines");
        let wifi = find(&k, "Wifi");
        assert_eq!(wifi.secret, br"caf\xc3\xa9");
        assert_eq!(wifi.attributes["port"], "443");
        assert_eq!(wifi.attributes[SCHEMA], "org.gnome.keyring.NetworkPassword");
    }

    #[test]
    fn a_wrong_password_is_refused() {
        assert_eq!(open("login", LOGIN, b"pW").err(), Some(Refused::WrongPassword));
        assert_eq!(open("login", LOGIN, b"").err(), Some(Refused::WrongPassword));
        assert_eq!(label_of(LOGIN).as_deref(), Some("Login"));
    }

    #[test]
    fn a_damaged_keyring_is_refused_not_panicked_over() {
        for cut in [0, 10, 30, 60, 200, LOGIN.len() - 1] {
            assert!(open("login", &LOGIN[..cut], b"pw").is_err());
        }
        let mut bad = LOGIN.to_vec();
        bad[SEALED.len()] = 1;
        assert_eq!(open("login", &bad, b"pw").err(), Some(Refused::Damaged));
    }

    #[test]
    fn a_plain_keyring_needs_no_password() {
        let k = open("Open", OPEN, b"anything").unwrap();
        assert_eq!((k.label.as_str(), k.items.len()), ("Open", 3));
        assert_eq!(find(&k, "Mail").secret, b"one");
        assert_eq!(find(&k, "Mail").attributes["user"], "john@example.com");
        assert_eq!(find(&k, "Chat login").secret, b"two");
        assert_eq!(find(&k, "Wifi").secret, "café".as_bytes());
        assert_eq!(find(&k, "Wifi").attributes[SCHEMA], "org.gnome.keyring.NetworkPassword");
    }

    #[test]
    fn plain_keyrings_read_escapes_numbers_and_hex() {
        let k = plain("x", "[keyring]\ndisplay-name=A\\sB\n\n[7]\nitem-type=1\ndisplay-name=Line\\none\n\
            binary-secret=00ff41\nctime=5\n\n[7:attribute0]\nname=port\ntype=uint32\nvalue=443\n");
        assert_eq!(k.label, "A B");
        let i = &k.items[0];
        assert_eq!((i.label.as_str(), i.secret.as_slice(), i.created), ("Line\none", &[0, 255, 65][..], 5));
        assert_eq!(i.attributes["port"], "443");
        assert_eq!(i.attributes[SCHEMA], "org.gnome.keyring.NetworkPassword");
    }

    #[test]
    fn a_folder_reads_what_opens_and_names_what_does_not() {
        let dir = std::env::temp_dir().join(format!("trindle-gnome-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("login.keyring"), LOGIN).unwrap();
        fs::write(dir.join("Open.keyring"), OPEN).unwrap();
        fs::write(dir.join("user.keystore"), b"not a keyring").unwrap();
        fs::write(dir.join("default"), b"login").unwrap();
        let found = read_all(&dir, &[b"pw"]);
        assert_eq!(found.opened.len(), 2);
        assert_eq!(found.default.as_deref(), Some("login"));
        let found = read_all(&dir, &[b"other"]);
        assert_eq!((found.opened.len(), found.left.as_slice(), found.default), (1, &["Login".to_string()][..], None));
        let found = read_all(&dir, &[b"other", b"pw"]);
        assert_eq!((found.opened.len(), found.left.len()), (2, 0));
        fs::remove_dir_all(&dir).unwrap();
        assert!(read_all(&dir, &[b"pw"]).opened.is_empty());
    }

    #[test]
    #[ignore = "needs real GNOME keyring files named by TRINDLE_TEST_KEYRINGS: run by hand"]
    fn prints_like_gnome() {
        let found = read_all(Path::new(&std::env::var("TRINDLE_TEST_KEYRINGS").unwrap()), &[b"pw"]);
        let mut out = serde_json::Map::new();
        for k in &found.opened {
            let mut items: Vec<_> = k.items.iter().map(|i| serde_json::json!({"label": i.label,
                "attributes": i.attributes, "secret": i.secret, "content_type": i.content_type,
                "created": i.created, "modified": i.modified})).collect();
            items.sort_by_key(|i| i["label"].as_str().unwrap().to_string());
            out.insert(k.name.clone(), serde_json::json!({"label": k.label, "items": items}));
        }
        out.insert("default".into(), found.default.into());
        println!("{}", serde_json::Value::Object(out));
    }

}
