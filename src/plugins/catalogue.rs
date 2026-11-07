//! Where plugins come from.

use std::fmt::Write as _;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use ::ring::signature::{UnparsedPublicKey, ED25519};
use sha2::{Digest, Sha256};

use super::install::{put_in, Installed};
use super::manifest::{check, good_id, Plugin, INTERFACE, MANIFEST};
use super::notes::read_plain;
use super::permissions::{asks_more, PERMISSIONS};
use crate::common::files::{cache_home, home, open_file, private_temp, write_whole};
use crate::common::keyfile::KeyFile;
use crate::common::run;
use crate::common::stopping::Stop;
use crate::toolkit::t;

#[cfg(any(test, feature = "publisher"))]
pub mod publisher;

pub const INDEX: &str = "catalogue.ini";

#[cfg(not(feature = "test-catalogue"))]
pub const PUBLISHED: &str = "https://github.com/Trindle-OS/trindle-releases/releases/latest/download/catalogue.ini";
#[cfg(feature = "test-catalogue")]
pub const PUBLISHED: &str = "https://localhost:8443/catalogue.ini";

#[cfg(not(feature = "test-catalogue"))]
pub const KEYS: &[&str] = &["0586e5476f6b93a84f1cf01d58774e70ecb81719c74d6abe0bd184e95f11ee1b"];
#[cfg(feature = "test-catalogue")]
pub const KEYS: &[&str] = &["838042661858272bf75de1596dc74f61d6b2223825ccf95d52318d1dd9992589"];

const NUMBER_FLOOR: u64 = 1_791_763_200;

pub const SIGNATURE_END: &str = ".sig";
const SIGNATURE_BYTES: usize = 64;
const KEY_BYTES: usize = 32;
const SIGNATURE_MOST: &str = "64";

const NUMBER_NOTE: &str = ".local/state/trindle/plugin-catalogue-number";

const FORMAT: i64 = 1;

const ARCHIVES: &[&str] = &[".tar.zst", ".tar.gz", ".tgz", ".tar.xz", ".tar"];

const KEPT: &str = "trindle/plugin-catalogue.ini";
pub const INDEX_FRESH: Duration = Duration::from_secs(60 * 60);
const KEPT_MOST: u64 = 1 << 20;

const INDEX_MOST: &str = "1M";
const ICON_MOST: &str = "1M";
const ARCHIVE_MOST: &str = "500M";
const UNPACKED_MOST: u64 = 1 << 30;
const FILES_MOST: usize = 20_000;

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    pub permissions: Vec<String>,
    pub icon: Option<String>,
    pub icon_sha256: Option<String>,
    pub archive: String,
    pub sha256: String,
}

#[derive(Clone, Debug)]
pub struct Offer {
    pub entry: Entry,
    pub icon: Option<PathBuf>,
}

fn plain_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 200 && !name.starts_with('.') && !name.contains(['/', '\\', '\0'])
}

fn is_archive(name: &str) -> bool {
    ARCHIVES.iter().any(|a| name.ends_with(a))
}

fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

pub fn parse(text: &str) -> Result<(Vec<Entry>, Vec<String>), String> {
    let parts = KeyFile::parts(text);
    let format = parts.iter().filter(|(group, _)| group == "Catalogue").filter_map(|(group, kf)| kf.integer(group, "Format")).next_back();
    if format.is_none() {
        return Err(t!("plugins-catalogue-not-one", file = INDEX));
    }
    if format != Some(FORMAT) {
        return Err(t!("plugins-catalogue-newer", format = FORMAT));
    }
    let (mut found, mut problems) = (Vec::new(), Vec::new());
    for (group, kf) in &parts {
        let Some(id) = group.strip_prefix("Plugin").and_then(|rest| rest.strip_prefix(' ')) else { continue };
        let get = |key: &str| kf.string(group, key).map(|v| v.trim().to_string()).unwrap_or_default();
        if kf.integer(group, "Interface") != Some(i64::from(INTERFACE)) {
            continue;
        }
        let permissions: Vec<String> = kf.list(group, "Permissions").unwrap_or_default()
            .into_iter().map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect();
        let icon = Some(get("Icon")).filter(|i| !i.is_empty());
        let icon_sha256 = Some(get("IconSHA256").to_lowercase()).filter(|h| is_sha256(h));
        let icon = icon.filter(|_| icon_sha256.is_some());
        let icon_sha256 = icon_sha256.filter(|_| icon.is_some());
        let e = Entry {
            id: id.trim().to_string(), name: get("Name"), version: get("Version"), summary: get("Summary"),
            permissions, icon, icon_sha256, archive: get("Archive"), sha256: get("SHA256").to_lowercase(),
        };
        let wrong = if !good_id(&e.id) {
            Some("id")
        } else if e.name.is_empty() || e.version.is_empty() {
            Some("needs")
        } else if !plain_name(&e.archive) || !is_archive(&e.archive) {
            Some("archive")
        } else if !is_sha256(&e.sha256) {
            Some("hash")
        } else if e.permissions.iter().any(|p| !PERMISSIONS.contains(&p.as_str())) {
            Some("permission")
        } else if e.icon.as_deref().is_some_and(|i| !plain_name(i) || !(i.ends_with(".svg") || i.ends_with(".png"))) {
            Some("icon")
        } else {
            None
        };
        match wrong {
            Some(why) => problems.push(t!("plugins-catalogue-left-out", id = id, why = why)),
            None => found.push(e),
        }
    }
    Ok((found, problems))
}

pub fn number_of(text: &str) -> Option<u64> {
    let said = KeyFile::parse(text).string("Catalogue", "Serial")?;
    let said = said.trim();
    (!said.is_empty() && said.len() <= 19 && said.bytes().all(|b| b.is_ascii_digit())).then(|| said.parse().ok()).flatten()
}

pub fn newer(a: &str, b: &str) -> bool {
    fn parts(v: &str) -> Vec<(u64, String)> {
        v.split(['.', '-', '+', '~', '_']).map(|p| {
            let digits: String = p.chars().take_while(char::is_ascii_digit).collect();
            (digits.parse().unwrap_or(0), p[digits.len()..].to_string())
        }).collect()
    }
    parts(a) > parts(b)
}

pub fn sha256_of(path: &Path) -> std::io::Result<String> {
    let mut f = open_file(path)?;
    let mut hash = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hash.update(&buf[..n]);
    }
    Ok(hash.finalize().iter().fold(String::new(), |mut s, b| { let _ = write!(s, "{b:02x}"); s }))
}

pub fn unpack(archive: &Path, into: &Path) -> Result<PathBuf, String> {
    within(archive, UNPACKED_MOST, FILES_MOST)?;
    let out = crate::common::gentle::program(&mut Command::new("tar")).args(["-x", "--no-same-owner", "--no-same-permissions", "-f"]).arg(archive)
        .arg("-C").arg(into).stdin(Stdio::null()).output()
        .map_err(|e| t!("plugins-unpack-tar", why = e.to_string()))?;
    if !out.status.success() {
        let said = String::from_utf8_lossy(&out.stderr);
        return Err(t!("plugins-unpack-failed", why = said.lines().next().unwrap_or(t!("plugins-unpack-tar-refused")).trim()));
    }
    if into.join(MANIFEST).is_file() {
        return Ok(into.to_path_buf());
    }
    let tops: Vec<PathBuf> = std::fs::read_dir(into).into_iter().flatten().flatten().map(|e| e.path()).collect();
    match tops.as_slice() {
        [one] if one.is_dir() && !one.is_symlink() && one.join(MANIFEST).is_file() => Ok(one.clone()),
        _ => Err(t!("plugins-unpack-no-manifest", file = MANIFEST)),
    }
}

fn within(archive: &Path, bytes: u64, files: usize) -> Result<(), String> {
    use std::io::BufRead;
    let refused = || t!("plugins-unpack-failed", why = t!("plugins-unpack-tar-refused"));
    let mut list = crate::common::gentle::program(&mut Command::new("tar")).arg("-tvf").arg(archive)
        .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn()
        .map_err(|e| t!("plugins-unpack-tar", why = e.to_string()))?;
    let (mut total, mut seen) = (0u64, 0usize);
    let mut too_much = false;
    for line in std::io::BufReader::new(list.stdout.take().expect("piped")).lines().map_while(Result::ok) {
        seen += 1;
        total = total.saturating_add(line.split_whitespace().nth(2).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0));
        if seen > files || total > bytes {
            too_much = true;
            break;
        }
    }
    let ended = list.wait().map_err(|e| e.to_string())?;
    if too_much || !ended.success() { Err(refused()) } else { Ok(()) }
}

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scratch() -> Result<Scratch, String> {
    private_temp("trindle-plugin").map(Scratch).map_err(|e| t!("plugins-scratch", why = e.to_string()))
}

fn key_of(hex: &str) -> Option<[u8; KEY_BYTES]> {
    let hex = hex.trim();
    if hex.len() != KEY_BYTES * 2 || !hex.is_ascii() {
        return None;
    }
    let mut key = [0u8; KEY_BYTES];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(key)
}

fn built_in_keys() -> Vec<[u8; KEY_BYTES]> {
    KEYS.iter().filter_map(|k| key_of(k)).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Refused {
    NoKey,
    NoSignature,
    WrongSize,
    NotSigned,
    NoNumber,
    Older { is: u64, after: u64 },
    NotRemembered,
    NotACatalogue(String),
}

pub fn paused() -> &'static str {
    t!("plugins-catalogue-paused")
}

pub fn start_again() -> &'static str {
    t!("plugins-catalogue-start-again")
}

pub fn is_paused(words: &str) -> bool {
    words == paused()
}

pub fn start_again_in(home: &Path) -> Result<(), String> {
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    let flags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC;
    let cpath = |s: &std::ffi::OsStr| std::ffi::CString::new(std::os::unix::ffi::OsStrExt::as_bytes(s)).map_err(|_| t!("plugins-start-nul").to_string());
    // SAFETY: opens a folder by a nul-ended name; the descriptor is owned at once.
    let top = unsafe { libc::open(cpath(home.as_os_str())?.as_ptr(), flags) };
    if top < 0 {
        return Err(format!("{}: {}", home.display(), std::io::Error::last_os_error()));
    }
    // SAFETY: `top` is a descriptor just opened and not owned by anything else.
    let mut dir = unsafe { OwnedFd::from_raw_fd(top) };
    let parts: Vec<&str> = NUMBER_NOTE.split('/').collect();
    let (name, folders) = parts.split_last().ok_or(t!("plugins-start-no-note"))?;
    for part in folders {
        let c = cpath(std::ffi::OsStr::new(part))?;
        // SAFETY: opens a folder inside the one held, never through a link.
        let fd = unsafe { libc::openat(dir.as_raw_fd(), c.as_ptr(), flags | libc::O_NOFOLLOW) };
        if fd < 0 {
            let e = std::io::Error::last_os_error();
            return if e.kind() == std::io::ErrorKind::NotFound { Ok(()) } else { Err(format!("{part}: {e}")) };
        }
        // SAFETY: as above.
        dir = unsafe { OwnedFd::from_raw_fd(fd) };
    }
    let c = cpath(std::ffi::OsStr::new(name))?;
    // SAFETY: takes away one name inside the folder held; a link is taken away itself and not followed.
    let gone = unsafe { libc::unlinkat(dir.as_raw_fd(), c.as_ptr(), 0) };
    if gone < 0 {
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(t!("plugins-start-not-taken", why = e.to_string()));
        }
    }
    Ok(())
}

impl Refused {
    pub fn words(&self) -> String {
        match self {
            Refused::NoKey => t!("plugins-refused-no-key").into(),
            Refused::NoSignature => t!("plugins-refused-no-signature").into(),
            Refused::WrongSize => t!("plugins-refused-wrong-size").into(),
            Refused::NotSigned => t!("plugins-refused-not-signed").into(),
            Refused::NoNumber => t!("plugins-refused-no-number").into(),
            Refused::Older { is, after } => t!("plugins-refused-older", is = *is, after = *after),
            Refused::NotRemembered => paused().into(),
            Refused::NotACatalogue(why) => t!("plugins-refused-unreadable", why = why),
        }
    }
}

fn signed_by(bytes: &[u8], signature: &[u8], keys: &[[u8; KEY_BYTES]]) -> Result<(), Refused> {
    if keys.is_empty() {
        return Err(Refused::NoKey);
    }
    if signature.len() != SIGNATURE_BYTES {
        return Err(Refused::WrongSize);
    }
    if keys.iter().any(|key| UnparsedPublicKey::new(&ED25519, key).verify(bytes, signature).is_ok()) {
        Ok(())
    } else {
        Err(Refused::NotSigned)
    }
}

fn accepted(bytes: &[u8], signature: &[u8], keys: &[[u8; KEY_BYTES]], least: u64) -> Result<(String, u64), Refused> {
    signed_by(bytes, signature, keys)?;
    let text = std::str::from_utf8(bytes).map_err(|_| Refused::NotACatalogue(t!("plugins-catalogue-not-text").into()))?.to_string();
    parse(&text).map_err(Refused::NotACatalogue)?;
    let number = number_of(&text).ok_or(Refused::NoNumber)?;
    if number < least {
        return Err(Refused::Older { is: number, after: least });
    }
    Ok((text, number))
}

struct Trust {
    keys: Vec<[u8; KEY_BYTES]>,
    kept: PathBuf,
    note: PathBuf,
    floor: u64,
}

impl Trust {
    fn own() -> Trust {
        Trust { keys: built_in_keys(), kept: cache_home().join(KEPT), note: home().join(NUMBER_NOTE), floor: NUMBER_FLOOR }
    }

    fn kept_signature(&self) -> PathBuf {
        let mut name = self.kept.as_os_str().to_owned();
        name.push(SIGNATURE_END);
        PathBuf::from(name)
    }

    fn kept_good(&self) -> Option<(String, Duration, u64)> {
        let bytes = small_file(&self.kept, KEPT_MOST)?;
        let signature = small_file(&self.kept_signature(), SIGNATURE_BYTES as u64)?;
        let (text, number) = accepted(&bytes, &signature, &self.keys, 0).ok()?;
        let age = std::fs::metadata(&self.kept).ok()?.modified().ok().and_then(|m| m.elapsed().ok()).unwrap_or(Duration::MAX);
        Some((text, age, number))
    }

    fn least(&self, kept: Option<u64>) -> Result<u64, Refused> {
        let noted = match std::fs::symlink_metadata(&self.note) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Some(0),
            Ok(m) if m.is_file() && m.len() <= 32 => read_plain(&self.note, 32).and_then(|n| String::from_utf8(n).ok()).and_then(|n| {
                let n = n.trim();
                (!n.is_empty() && n.bytes().all(|b| b.is_ascii_digit())).then(|| n.parse::<u64>().ok()).flatten()
            }),
            _ => None,
        };
        match (noted, kept) {
            (Some(noted), kept) => Ok(noted.max(kept.unwrap_or(0)).max(self.floor)),
            (None, Some(kept)) => Ok(kept.max(self.floor)),
            (None, None) => Err(Refused::NotRemembered),
        }
    }

    fn keep(&self, bytes: &[u8], signature: &[u8], number: u64) {
        for (at, what) in [(&self.kept, bytes), (&self.kept_signature(), signature), (&self.note, number.to_string().as_bytes())] {
            let _ = at.parent().map(std::fs::create_dir_all);
            if std::fs::symlink_metadata(at).is_ok_and(|m| !m.is_file()) {
                let _ = std::fs::remove_file(at);
            }
            let _ = write_whole(at, what);
        }
    }
}

fn small_file(path: &Path, most: u64) -> Option<Vec<u8>> {
    read_plain(path, most)
}

fn download(url: &str, to: &Path, most: &str, seconds: &str) -> Result<(), String> {
    download_with(url, to, most, seconds, None)
}

fn download_with(url: &str, to: &Path, most: &str, seconds: &str, stop: Option<&Stop>) -> Result<(), String> {
    let mut curl = Command::new("curl");
    curl.args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", seconds, "--max-filesize", most, "-o"])
        .arg(to).arg(url).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped());
    let (mut child, proc) = run::start_group(&mut curl).map_err(|e| t!("plugins-download-curl", why = e.to_string()))?;
    if let Some(stop) = stop {
        stop.hold(proc.clone());
    }
    let mut said = Vec::new();
    if let Some(mut err) = child.stderr.take() {
        let _ = err.read_to_end(&mut said);
    }
    let status = run::collect(&mut child, &proc);
    if let Some(stop) = stop {
        stop.release();
    }
    if status.is_some_and(|s| s.success()) {
        return Ok(());
    }
    let said = String::from_utf8_lossy(&said);
    Err(t!("plugins-download-failed", why = said.trim().trim_start_matches("curl: ").lines().next().unwrap_or("")))
}

fn beside(index_url: &str, name: &str) -> String {
    match index_url.rsplit_once('/') {
        Some((base, _)) => format!("{base}/{name}"),
        None => name.to_string(),
    }
}

fn icons_home() -> PathBuf {
    cache_home().join("trindle/plugin-icons")
}

pub fn forget_icon(id: &str) {
    forget_icon_at(&icons_home(), id);
}

pub fn forget_icon_at(icons: &Path, id: &str) {
    if !good_id(id) {
        return;
    }
    for kind in ["svg", "png"] {
        let _ = std::fs::remove_file(icons.join(format!("{id}.{kind}")));
    }
}

fn fetch_file(url: &str, to: &Path, most: &str) -> Result<(), String> {
    download(url, to, most, "15")
}

type Fetch<'a> = &'a dyn Fn(&str, &Path, &str) -> Result<(), String>;

fn index_text(fresh: Duration, fetch: Fetch, trust: &Trust) -> (Option<String>, Option<String>) {
    let kept = trust.kept_good();
    if let Some((text, _, _)) = kept.as_ref().filter(|(_, age, _)| *age < fresh) {
        return (Some(text.clone()), None);
    }
    let kept_number = kept.as_ref().map(|(_, _, number)| *number);
    let kept_text = kept.map(|(text, _, _)| text);
    let fetched = scratch().and_then(|tmp| {
        let file = tmp.0.join(INDEX);
        fetch(PUBLISHED, &file, INDEX_MOST)?;
        let bytes = small_file(&file, KEPT_MOST).ok_or(t!("plugins-catalogue-too-big"))?;
        let beside = tmp.0.join(format!("{INDEX}{SIGNATURE_END}"));
        let signature = fetch(&format!("{PUBLISHED}{SIGNATURE_END}"), &beside, SIGNATURE_MOST).ok()
            .map(|()| open_file(&beside).ok().and_then(|f| {
                let mut read = Vec::new();
                f.take(SIGNATURE_BYTES as u64 + 1).read_to_end(&mut read).ok().map(|_| read)
            }).unwrap_or_default());
        Ok::<_, String>((bytes, signature))
    });
    let Ok((bytes, signature)) = fetched else { return (kept_text, None) };
    let judged = match signature {
        None => Err(Refused::NoSignature),
        Some(signature) => signed_by(&bytes, &signature, &trust.keys)
            .and_then(|()| trust.least(kept_number))
            .and_then(|least| accepted(&bytes, &signature, &trust.keys, least))
            .map(|(text, number)| (text, number, signature)),
    };
    match judged {
        Ok((text, number, signature)) => {
            trust.keep(&bytes, &signature, number);
            (Some(text), None)
        }
        Err(why) => {
            let said = if kept_text.is_some() { t!("plugins-catalogue-last-good", why = why.words()) } else { why.words() };
            (kept_text, Some(said))
        }
    }
}

fn entries_at(fresh: Duration, fetch: Fetch, trust: &Trust) -> (Vec<Entry>, Option<String>) {
    let (text, said) = index_text(fresh, fetch, trust);
    (text.and_then(|t| parse(&t).ok()).map(|(entries, _)| entries).unwrap_or_default(), said)
}

fn read_address() -> (Vec<Offer>, Option<String>) {
    let (entries, said) = entries_at(Duration::ZERO, &fetch_file, &Trust::own());
    let icons = icons_home();
    let _ = std::fs::create_dir_all(&icons);
    (entries.into_iter().map(|e| {
        let icon = e.icon.as_ref().and_then(|i| {
            let at = icons.join(format!("{}.{}", e.id, if i.ends_with(".png") { "png" } else { "svg" }));
            download(&beside(PUBLISHED, i), &at, ICON_MOST, "15").ok().and_then(|_| icon_checked(&e, &at))
        });
        Offer { entry: e, icon }
    }).collect(), said)
}

fn icon_checked(entry: &Entry, at: &Path) -> Option<PathBuf> {
    let right = entry.icon_sha256.as_deref().is_some_and(|want| sha256_of(at).is_ok_and(|got| got == want));
    if right {
        Some(at.to_path_buf())
    } else {
        let _ = std::fs::remove_file(at);
        None
    }
}

fn one_of_each(all: Vec<Offer>) -> Vec<Offer> {
    let mut one: Vec<Offer> = Vec::new();
    for o in all {
        match one.iter_mut().find(|k| k.entry.id == o.entry.id) {
            Some(k) if newer(&o.entry.version, &k.entry.version) => *k = o,
            Some(_) => {}
            None => one.push(o),
        }
    }
    one.sort_by_key(|o| o.entry.name.to_lowercase());
    one
}

pub fn gather() -> (Vec<Offer>, Option<String>) {
    let (offers, said) = read_address();
    (one_of_each(offers), said)
}

pub fn offers_to_count(fresh: Duration) -> Vec<Offer> {
    offers_to_count_from(fresh, &fetch_file, &Trust::own())
}

pub fn offers_kept() -> Vec<Offer> {
    let kept = Trust::own().kept_good().and_then(|(text, _, _)| parse(&text).ok()).map(|(entries, _)| entries).unwrap_or_default();
    one_of_each(kept.into_iter().map(|entry| Offer { entry, icon: None }).collect())
}

fn offers_to_count_from(fresh: Duration, fetch: Fetch, trust: &Trust) -> Vec<Offer> {
    one_of_each(entries_at(fresh, fetch, trust).0.into_iter().map(|entry| Offer { entry, icon: None }).collect())
}

pub fn update_of<'a>(offers: &'a [Offer], installed: &Plugin) -> Option<&'a Offer> {
    offers.iter().find(|o| o.entry.id == installed.id && newer(&o.entry.version, &installed.version))
}

pub fn updates_for(installed: &[Plugin], offers: &[Offer]) -> Vec<(Plugin, Offer)> {
    let mut out: Vec<(Plugin, Offer)> = installed.iter().filter_map(|i| update_of(offers, i).map(|o| (i.clone(), o.clone()))).collect();
    out.sort_by_key(|a| a.0.name.to_lowercase());
    out
}

pub fn said_to_ask_more(installed: &Plugin, offer: &Offer) -> Vec<String> {
    asks_more(installed, &offer.entry.permissions, None)
}

type Get<'a> = &'a dyn Fn(&str, &Path) -> Result<(), String>;

fn get_archive(url: &str, to: &Path, stop: Option<&Stop>) -> Result<(), String> {
    download_with(url, to, ARCHIVE_MOST, "1800", stop)
}

fn fetched(offer: &Offer, get: Get) -> Result<(Scratch, PathBuf, Plugin), String> {
    let e = &offer.entry;
    let tmp = scratch()?;
    let archive = tmp.0.join(&e.archive);
    get(&beside(PUBLISHED, &e.archive), &archive)?;
    let sum = sha256_of(&archive).map_err(|err| err.to_string())?;
    if sum != e.sha256 {
        return Err(t!("plugins-install-hash").into());
    }
    let into = tmp.0.join("unpacked");
    std::fs::create_dir(&into).map_err(|err| err.to_string())?;
    let at = unpack(&archive, &into)?;
    let p = check(&at).map_err(|problems| {
        t!("plugins-install-check", why = problems.first().map_or("", String::as_str))
    })?;
    if p.id != e.id {
        return Err(t!("plugins-install-other-id", found = &p.id, wanted = &e.id));
    }
    Ok((tmp, at, p))
}

pub fn fetch_install(offer: &Offer, root: &Path, stop: &Stop) -> Result<Installed, String> {
    install_offer(offer, root, &|url, to| get_archive(url, to, Some(stop)))
}

fn install_offer(offer: &Offer, root: &Path, get: Get) -> Result<Installed, String> {
    let (_tmp, at, _) = fetched(offer, get)?;
    put_in(root, &at).and_then(|done| marked(root, offer, done))
}

fn marked<T>(root: &Path, offer: &Offer, done: T) -> Result<T, String> {
    crate::common::own_hands::mark(root, &offer.entry.id, &offer.entry.sha256)
        .map(|()| { super::permissions::preset_reader(root, &offer.entry.id); done })
        .map_err(|e| t!("plugins-install-note", why = e.to_string()))
}

#[derive(Clone, Debug)]
pub enum Unseen {
    In(Box<Installed>),
    AsksMore(Vec<String>),
}

pub fn fetch_update_unseen(offer: &Offer, root: &Path, stop: &Stop) -> Result<Unseen, String> {
    update_unseen(offer, root, &|url, to| get_archive(url, to, Some(stop)))
}

fn update_unseen(offer: &Offer, root: &Path, get: Get) -> Result<Unseen, String> {
    let (_tmp, at, new) = fetched(offer, get)?;
    let old = check(&root.join(&new.id).join("app")).map_err(|_| t!("plugins-update-not-installed").to_string())?;
    let more = asks_more(&old, &new.asks, Some(&new.listen));
    if !more.is_empty() {
        return Ok(Unseen::AsksMore(more));
    }
    put_in(root, &at).and_then(|done| marked(root, offer, done)).map(|done| Unseen::In(Box::new(done)))
}

#[cfg(test)]
mod tests;
