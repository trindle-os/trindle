//! Apps from GitHub releases: AppImages and Flatpak bundles.
//! Sources: GitHub search, and AppImageHub's list through `super::hub`.
//! The key a person gives for GitHub is kept in their keyring (`key`) and
//! handed to curl on its input, never on its command line and never in a
//! file.

use crate::toolkit::t;
use std::path::PathBuf;

use serde_json::Value;
use zeroize::Zeroizing;

use super::key;
use crate::common::files::home;
use crate::common::run::{self, capture};

pub fn unchecked() -> &'static str {
    t!("store-gh-unchecked")
}

pub fn says_unchecked(offered: usize) -> bool {
    offered > 0
}

#[cfg(not(test))]
const API: &str = "https://api.github.com";

pub(super) const SUMMARY_MOST: usize = 160;
const DAY: u64 = 24 * 3600;

const RELEASE_KEPT: u64 = 6 * 3600;

const SEARCH_KEPT: u64 = 3600;

pub(super) fn no_files() -> &'static str {
    t!("store-gh-no-files")
}

pub(super) fn no_releases() -> &'static str {
    t!("store-gh-no-releases")
}

fn limit_lead() -> String {
    super::refusal::lead_of(t!("store-gh-limit", which = "\u{0}", when = "")).unwrap_or_default()
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Found {

    pub repo: String,
    pub name: String,
    pub summary: String,
    pub stars: Option<u64>,

    pub categories: Vec<String>,

    pub icon: String,

    pub site: String,

    pub pushed: Option<u64>,
    pub created: Option<u64>,

    pub listed: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Project {
    pub stars: Option<u64>,
    pub pushed: Option<u64>,
    pub created: Option<u64>,
}

fn when(v: &Value) -> Option<u64> {
    v.as_str().and_then(crate::common::home_snapshots::unix_of).and_then(|n| u64::try_from(n).ok())
}

pub fn repo_of_url(url: &str) -> Option<String> {
    let rest = url.trim().strip_prefix("https://github.com/")?;
    let mut parts = rest.trim_end_matches('/').split('/');
    let (owner, name) = (parts.next()?, parts.next()?.trim_end_matches(".git"));
    let plain = |p: &str| !p.is_empty() && !p.starts_with('.') && p.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'));
    (plain(owner) && plain(name)).then(|| format!("{owner}/{name}"))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    AppImage,
    Flatpak,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Download {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub kind: Kind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub tag: String,
    pub files: Vec<Download>,

    pub published: Option<u64>,
}

pub(super) fn cache() -> PathBuf {
    home().join(".cache/trindle")
}

#[cfg(not(test))]
fn old_files() -> (PathBuf, Vec<PathBuf>) {
    const COPY: &str = "trindle-github-headers";
    let mut copies = vec![crate::common::files::trindle_config().join(COPY)];
    copies.extend(std::env::var_os("XDG_RUNTIME_DIR").map(|dir| PathBuf::from(dir).join(COPY)));
    (crate::common::files::config_path("github-key"), copies)
}

#[cfg(test)]
thread_local! {
    static OLD_FILE: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn old_files() -> (PathBuf, Vec<PathBuf>) {
    let none = std::env::temp_dir().join(format!("trindle-github-no-old-key-{}", std::process::id()));
    (OLD_FILE.with(|f| f.borrow().clone()).unwrap_or(none), Vec::new())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum KeyIs {

    #[default]
    Looking,

    Stored,

    None,

    Plain(String),

    Hidden(String),

    NoKeyring,

    Unsendable,
}

impl KeyIs {

    pub fn words(&self) -> String {
        match self {
            KeyIs::Looking => t!("store-gh-keyis-looking").into(),
            KeyIs::Stored => t!("store-gh-keyis-stored").into(),
            KeyIs::None => t!("store-gh-keyis-none").into(),
            KeyIs::NoKeyring => t!("store-gh-keyis-nokeyring").into(),
            KeyIs::Plain(_) | KeyIs::Hidden(_) | KeyIs::Unsendable => self.trouble().unwrap_or_default(),
        }
    }

    pub fn trouble(&self) -> Option<String> {
        match self {
            KeyIs::Plain(why) => Some(t!("store-gh-keyis-plain", why = sentence(why))),
            KeyIs::Hidden(why) => Some(t!("store-gh-keyis-hidden", why = sentence(why))),
            KeyIs::Unsendable => Some(t!("store-gh-keyis-unsendable").into()),
            _ => None,
        }
    }
}

fn sentence(words: &str) -> String {
    let words = words.trim();
    if words.ends_with(['.', '!', '?']) { words.to_string() } else { format!("{words}.") }
}

fn stored_key(ring: &key::Ring) -> (Option<Zeroizing<String>>, KeyIs) {
    let (old, copies) = old_files();
    let bus = ring.bus();
    let moved = key::move_old(bus.as_ref(), &old, &copies);
    let kept = bus.as_ref().map_err(String::clone).and_then(key::read);
    let plain = std::fs::read_to_string(&old).ok().map(|text| Zeroizing::new(text.trim().to_string())).filter(|k| !k.is_empty());

    let (kept, plain) = match (kept, plain) {
        (Ok(Some(key)), _) if checked(&key).is_err() => return (None, KeyIs::Unsendable),
        (Ok(None) | Err(_), Some(key)) if checked(&key).is_err() => return (None, KeyIs::Unsendable),
        other => other,
    };
    match (kept, plain) {
        (Ok(Some(key)), None) => (Some(key), KeyIs::Stored),

        (Ok(Some(key)), Some(_)) => (Some(key), KeyIs::Plain(moved.err().unwrap_or_else(|| t!("store-gh-keyis-file-stays").into()))),
        (Ok(None), None) => (None, KeyIs::None),
        (Ok(None), Some(plain)) => (Some(plain), KeyIs::Plain(moved.err().unwrap_or_else(|| t!("store-gh-keyis-not-taken").into()))),
        (Err(why), Some(plain)) => (Some(plain), KeyIs::Plain(why)),
        (Err(why), None) if why == key::missing_words() => (None, KeyIs::NoKeyring),
        (Err(why), None) => (None, KeyIs::Hidden(why)),
    }
}

fn with_key<T: Send + 'static>(work: impl FnOnce(Option<Zeroizing<String>>, KeyIs) -> T + Send + 'static, done: impl FnOnce(T) + 'static) {
    let from = Asking::here();
    run::on_thread(move || {
        let (key, is) = key_within(from);
        work(key, is)
    }, done);
}

struct Asking {
    ring: key::Ring,
    #[cfg(test)]
    old: Option<PathBuf>,
}

impl Asking {
    fn here() -> Asking {
        Asking {
            ring: key::ring(),
            #[cfg(test)]
            old: OLD_FILE.with(|f| f.borrow().clone()),
        }
    }

    fn arrive(&self) {
        #[cfg(test)]
        OLD_FILE.with(|f| f.borrow_mut().clone_from(&self.old));
    }
}

fn key_within(from: Asking) -> (Option<Zeroizing<String>>, KeyIs) {
    key::within(move || {
        from.arrive();
        Ok(stored_key(&from.ring))
    }).unwrap_or_else(|why| (None, KeyIs::Hidden(why)))
}

fn set_waited(from: Asking, new: Zeroizing<String>) -> key::Waited<()> {
    key::within_late(move || {
        from.arrive();
        set_stored(&from.ring, &new)
    })
}

#[cfg(test)]
fn set_within(from: Asking, new: Zeroizing<String>) -> Result<(), String> {
    set_waited(from, new).now
}

#[derive(Clone, Debug, PartialEq)]
pub enum Set {

    Made,

    Not(String),

    Waiting,
}

pub fn waiting_words(removing: bool) -> String {
    crate::common::keyring_said::waiting("key", removing).words
}

pub fn late_words(removing: bool, how: &Result<(), String>) -> (String, bool) {
    let said = crate::common::keyring_said::late("key", removing, how);
    (said.words, !said.failed)
}

pub fn key_is(done: impl FnOnce(KeyIs) + 'static) {
    with_key(|_, is| is, done);
}

fn checked(new: &str) -> Result<&str, String> {
    let new = new.trim();
    if new.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return Err(t!("store-gh-key-spaces").into());
    }
    Ok(new)
}

fn set_stored(ring: &key::Ring, new: &str) -> Result<(), String> {
    let new = checked(new)?;
    let (old, copies) = old_files();
    if new.is_empty() {

        for file in copies.iter().chain([&old]) {
            match std::fs::remove_file(file) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(t!("store-key-not-removed", path = file.display().to_string(), why = e.to_string())),
                _ => {}
            }
        }
        return match ring.bus().and_then(|bus| key::forget(&bus)) {

            Err(why) if why == key::missing_words() => Ok(()),
            other => other,
        };
    }
    let bus = ring.bus()?;
    key::keep(&bus, new)?;

    key::move_old(Ok(&bus), &old, &copies)
}

pub fn set_key(new: &str, done: impl FnOnce(Set, KeyIs) + 'static, late: impl FnOnce(Result<(), String>, KeyIs) + 'static) {
    if let Err(why) = checked(new) {
        return key_is(move |is| done(Set::Not(why), is));
    }
    let (from, new) = (Asking::here(), Zeroizing::new(new.to_string()));
    run::on_thread(move || set_waited(from, new), move |waited: key::Waited<()>| {
        let how = match (&waited.now, &waited.late) {
            (_, Some(_)) => Set::Waiting,
            (Ok(()), None) => Set::Made,
            (Err(why), None) => Set::Not(why.clone()),
        };
        key_is(move |is| done(how, is));
        if let Some(coming) = waited.late {
            run::on_thread(move || coming.wait(), move |how| key_is(move |is| late(how, is)));
        }
    });
}

fn headers(key: Option<&str>) -> Zeroizing<String> {
    let mut text = Zeroizing::new(String::from("Accept: application/vnd.github+json\nX-GitHub-Api-Version: 2022-11-28\n"));
    if let Some(key) = key.and_then(|k| checked(k).ok()).filter(|k| !k.is_empty()) {
        text.push_str(&format!("Authorization: Bearer {key}\n"));
    }
    text
}

fn sent_with_key(headers: &str) -> bool {
    headers.lines().any(|line| line.starts_with("Authorization: "))
}

fn question(url: &str) -> Vec<&str> {
    vec!["curl", "-sS", "-L", "--proto", "=https", "--proto-redir", "=https", "--max-time", "20",
         "--max-filesize", run::MAX_ANSWER, "-H", "@-", "-w", TRAILER, url]
}

pub fn is_limit(error: &str) -> bool {
    let lead = limit_lead();
    !lead.is_empty() && error.starts_with(&lead)
}

fn limit_words(searching: bool, wait: Option<u64>, key: bool) -> String {
    let when = match wait.map(|secs| secs.div_ceil(60)) {
        None => t!("store-gh-wait-later").to_string(),
        Some(mins) => t!("store-gh-wait-minutes", n = mins.max(1)),
    };
    let which = match (searching, key) {
        (true, true) => t!("store-gh-limit-search-key"),
        (true, false) => t!("store-gh-limit-search"),
        (false, true) => t!("store-gh-limit-key"),
        (false, false) => t!("store-gh-limit-plain"),
    };
    if key || searching {
        t!("store-gh-limit", which = which, when = when)
    } else {
        t!("store-gh-limit-hint", which = which, when = when, hint = t!("store-gh-limit-key-hint"))
    }
}

const TRAILER: &str = "\n%{http_code} %header{x-ratelimit-remaining} %header{x-ratelimit-reset} %header{x-ratelimit-resource} %header{retry-after}";

fn answered(code: &str, rest: &str, body: &str, now: u64, key: bool) -> Result<Value, String> {
    let value: Value = serde_json::from_str(body).unwrap_or(Value::Null);
    let mut said = rest.split(' ');
    let mut number = || said.next().and_then(|w| w.parse::<u64>().ok());
    let (left, reset) = (number(), number());
    let searching = rest.split(' ').nth(2) == Some("search");
    let retry = rest.split(' ').nth(3).and_then(|w| w.parse::<u64>().ok());
    let message = value["message"].as_str().unwrap_or("").trim();
    match code {
        "200" => Ok(value),
        "401" if key => Err(t!("store-gh-key-refused").into()),
        "403" | "429" => {

            let limited = code == "429" || left == Some(0) || retry.is_some() || message.to_lowercase().contains("rate limit");
            if !limited && (left.is_some() || !message.is_empty()) {
                return Err(refused_words(message));
            }

            let wait = if left == Some(0) { reset.map(|at| at.saturating_sub(now)) } else { retry };
            Err(limit_words(searching, wait, key))
        }
        "404" => Err(no_releases().into()),
        _ => Err(if message.is_empty() { t!("store-gh-answered", code = code) } else { message.to_string() }),
    }
}

fn refused_words(message: &str) -> String {
    match message.trim().trim_end_matches('.') {
        "" => t!("store-gh-refused-plain").into(),
        why => t!("store-gh-refused", why = why),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Failure {

    Limit,

    Nothing,

    Unread,
}

pub fn failure(error: &str) -> Failure {
    if is_limit(error) {
        Failure::Limit
    } else if error == no_files() || error == no_releases() {
        Failure::Nothing
    } else {
        Failure::Unread
    }
}

pub fn unread_words(n: usize, why: &str) -> String {
    let why = why.trim().trim_end_matches('.');
    let which = t!("store-gh-unread", n = n);
    if why.is_empty() { t!("store-gh-unread-sentence", which = which) } else { t!("store-gh-unread-why", which = which, why = why) }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn api(path: &str, done: impl FnOnce(Result<Value, String>) + 'static) {

    #[cfg(test)]
    ASKED.with(|a| a.borrow_mut().push(path.to_string()));
    #[cfg(test)]
    drop(done);
    #[cfg(not(test))]
    {
        let url = format!("{API}{path}");
        with_key(move |key, _| ask(&["curl"], &url, key.as_deref().map(String::as_str)), done);
    }
}

#[cfg(test)]
thread_local! {
    static ASKED: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(crate) fn asked_so_far() -> Vec<String> {
    ASKED.with(|a| std::mem::take(&mut *a.borrow_mut()))
}

fn ask(curl: &[&str], url: &str, key: Option<&str>) -> Result<Value, String> {
    let sent = headers(key);
    let with_key = sent_with_key(&sent);
    let argv: Vec<&str> = curl.iter().copied().chain(question(url).into_iter().skip(1)).collect();

    let (ok, out, err) = run::captured_secret(&argv, Zeroizing::new(sent.trim_end_matches('\n').to_string()));
    if !ok {
        return Err(if err.trim().is_empty() { t!("store-gh-unreached").into() } else { err.trim().to_string() });
    }
    let (body, trailer) = out.rsplit_once('\n').unwrap_or((&out, ""));
    let (code, rest) = trailer.split_once(' ').unwrap_or((trailer, ""));
    answered(code, rest, body, now(), with_key)
}

fn repo_name(repo: &str) -> String {
    let mut parts = repo.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(owner), Some(name), None) if !owner.is_empty() && !name.is_empty() && !repo.contains('+') => format!("{owner}+{name}"),
        _ => String::new(),
    }
}

struct Kept {
    dir: PathBuf,
}

impl Kept {
    fn here() -> Kept {
        Kept { dir: cache().join("github") }
    }

    fn file(&self, kind: &str, name: &str) -> Option<PathBuf> {
        let plain = !name.is_empty() && name.len() <= 200 && !name.starts_with('.')
            && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'));
        plain.then(|| self.dir.join(format!("{kind}-{name}.json")))
    }

    fn read(&self, kind: &str, name: &str, kept_for: u64, now: u64) -> Option<Value> {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(self.file(kind, name)?).ok()?).ok()?;
        let at = v["at"].as_u64()?;

        (at <= now && now - at < kept_for).then(|| v["answer"].clone())
    }

    fn write(&self, kind: &str, name: &str, answer: Value, now: u64) {
        let Some(file) = self.file(kind, name) else { return };
        let _ = std::fs::create_dir_all(&self.dir);
        let _ = crate::common::files::write_text(&file, &serde_json::json!({ "at": now, "answer": answer }).to_string());
    }

    fn release(&self, repo: &str, now: u64) -> Option<Result<Release, String>> {
        let v = self.read("release", &repo_name(repo), RELEASE_KEPT, now)?;
        let files: Vec<Download> = v["files"].as_array()?.iter().filter_map(|f| Some(Download {
            name: f["name"].as_str()?.to_string(),
            url: f["url"].as_str()?.to_string(),
            size: f["size"].as_u64().unwrap_or(0),
            kind: if f["kind"] == "flatpak" { Kind::Flatpak } else { Kind::AppImage },
        })).collect();
        Some(if files.is_empty() { Err(no_files().into()) } else {
            Ok(Release { tag: v["tag"].as_str().unwrap_or("").to_string(), files, published: v["published"].as_u64() })
        })
    }

    fn keep_release(&self, repo: &str, answer: &Result<Release, String>, now: u64) {
        let (tag, files, published) = match answer {
            Ok(rel) => (rel.tag.as_str(), rel.files.as_slice(), rel.published),
            Err(e) if e == no_files() => ("", &[][..], None),
            Err(_) => return,
        };
        let files: Vec<Value> = files.iter().map(|f| serde_json::json!({
            "name": f.name, "url": f.url, "size": f.size, "kind": if f.kind == Kind::Flatpak { "flatpak" } else { "appimage" },
        })).collect();
        self.write("release", &repo_name(repo), serde_json::json!({ "tag": tag, "files": files, "published": published }), now);
    }

    fn search(&self, q: &str, now: u64) -> Option<Vec<Found>> {
        let v = self.read("search", q, SEARCH_KEPT, now)?;
        Some(v.as_array()?.iter().filter_map(|f| Some(Found {
            repo: f["repo"].as_str()?.to_string(),
            name: f["name"].as_str()?.to_string(),
            summary: f["summary"].as_str().unwrap_or("").to_string(),
            stars: f["stars"].as_u64(),
            pushed: f["pushed"].as_u64(),
            created: f["created"].as_u64(),
            ..Default::default()
        })).collect())
    }

    fn keep_searched(&self, q: &str, searched: &Searched, now: u64) {
        if searched.missed.is_none() {
            self.keep_search(q, &searched.found, now);
        }
    }

    fn keep_search(&self, q: &str, found: &[Found], now: u64) {
        let all: Vec<Value> = found.iter().map(|f| serde_json::json!({
            "repo": f.repo, "name": f.name, "summary": f.summary, "stars": f.stars, "pushed": f.pushed, "created": f.created,
        })).collect();
        self.write("search", q, Value::Array(all), now);
    }

    fn tidy(&self, now: u64) {
        for entry in std::fs::read_dir(&self.dir).into_iter().flatten().flatten() {
            let age = entry.metadata().and_then(|m| m.modified()).ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|t| now.saturating_sub(t.as_secs()));
            if age.is_none_or(|a| a > DAY) {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}

fn repos(v: &Value) -> Vec<Found> {
    v["items"].as_array().map(|items| items.iter().map(|i| Found {
        repo: i["full_name"].as_str().unwrap_or("").to_string(),
        name: i["name"].as_str().unwrap_or("").to_string(),
        summary: i["description"].as_str().unwrap_or("").chars().take(SUMMARY_MOST).collect(),
        stars: i["stargazers_count"].as_u64(),
        pushed: when(&i["pushed_at"]),
        created: when(&i["created_at"]),
        ..Default::default()
    }).filter(|f| !f.repo.is_empty()).collect()).unwrap_or_default()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Searched {
    pub found: Vec<Found>,
    pub missed: Option<String>,
}

fn both(first: Vec<Found>, second: Result<Vec<Found>, String>) -> Searched {
    let mut found = first;
    let missed = match second {
        Ok(more) => {
            for f in more {
                if !found.iter().any(|a| a.repo == f.repo) {
                    found.push(f);
                }
            }
            None
        }
        Err(why) => Some(why),
    };
    found.sort_by_key(|k| std::cmp::Reverse(k.stars));
    Searched { found, missed }
}

pub fn half_words(why: &str) -> String {
    t!("store-gh-half", why = why.trim())
}

pub fn search_github(words: &[String], done: impl FnOnce(Result<Searched, String>) + 'static) {
    let q: String = words.iter().map(|w| w.chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>())
        .filter(|w| !w.is_empty()).collect::<Vec<_>>().join("+");
    if q.is_empty() {
        crate::common::run::soon(move || done(Ok(Searched { found: Vec::new(), missed: None })));
        return;
    }
    if let Some(found) = Kept::here().search(&q, now()) {
        crate::common::run::soon(move || done(Ok(Searched { found, missed: None })));
        return;
    }
    let asked = q.clone();
    let done = move |res: Result<Searched, String>| {
        if let Ok(searched) = &res {
            Kept::here().keep_searched(&asked, searched, now());
        }
        done(res)
    };
    let second = format!("/search/repositories?q={q}+flatpak+fork:false&sort=stars&per_page=15");
    api(&format!("/search/repositories?q={q}+appimage+fork:false&sort=stars&per_page=15"), move |first| {
        let first = match first {
            Ok(v) => repos(&v),
            Err(e) => return done(Err(e)),
        };
        api(&second, move |r| done(Ok(both(first, r.map(|v| repos(&v))))));
    });
}

fn wrong_machine(name: &str) -> bool {
    let n = name.to_lowercase();
    let arm = ["aarch64", "arm64", "armhf", "armv7"].iter().any(|a| n.contains(a));
    let intel = ["x86_64", "x86-64", "amd64", "x64"].iter().any(|a| n.contains(a));
    let i386 = ["i386", "i686"].iter().any(|a| n.contains(a));
    match std::env::consts::ARCH {
        "aarch64" => (intel || i386) && !arm,
        _ => (arm || i386) && !intel,
    }
}

pub fn pick_files(assets: &Value) -> Vec<Download> {
    assets.as_array().map(|a| a.iter().filter_map(|x| {
        let name = x["name"].as_str()?.to_string();
        let lower = name.to_lowercase();
        let kind = if lower.ends_with(".appimage") { Kind::AppImage } else if lower.ends_with(".flatpak") { Kind::Flatpak } else { return None };
        if wrong_machine(&name) {
            return None;
        }
        Some(Download { url: x["browser_download_url"].as_str()?.to_string(), size: x["size"].as_u64().unwrap_or(0), name, kind })
    }).collect()).unwrap_or_default()
}

pub fn release(repo: &str, done: impl FnOnce(Result<Release, String>) + 'static) {
    if let Some(kept) = Kept::here().release(repo, now()) {
        crate::common::run::soon(move || done(kept));
        return;
    }
    let asked = repo.to_string();
    api(&format!("/repos/{repo}/releases?per_page=5"), move |r| {
        let answer = r.and_then(|v| {
            let releases = v.as_array().cloned().unwrap_or_default();
            releases.iter().filter(|r| r["draft"] != true)
                .map(|r| Release { tag: r["tag_name"].as_str().unwrap_or("").to_string(), files: pick_files(&r["assets"]),
                                   published: when(&r["published_at"]) })
                .find(|r| !r.files.is_empty())
                .ok_or_else(|| no_files().to_string())
        });
        Kept::here().keep_release(&asked, &answer, now());
        done(answer)
    });
}

pub fn project(repo: &str, done: impl FnOnce(Result<Project, String>) + 'static) {
    let kept = Kept::here();
    if let Some(v) = kept.read("project", &repo_name(repo), RELEASE_KEPT, now()) {
        let p = Project { stars: v["stars"].as_u64(), pushed: v["pushed"].as_u64(), created: v["created"].as_u64() };
        crate::common::run::soon(move || done(Ok(p)));
        return;
    }
    let asked = repo.to_string();
    api(&format!("/repos/{repo}"), move |r| {
        let answer = r.map(|v| Project { stars: v["stargazers_count"].as_u64(), pushed: when(&v["pushed_at"]), created: when(&v["created_at"]) });
        if let Ok(p) = &answer {
            Kept::here().write("project", &repo_name(&asked), serde_json::json!({ "stars": p.stars, "pushed": p.pushed, "created": p.created }), now());
        }
        done(answer)
    });
}

pub fn tidy() {
    Kept::here().tidy(now());
}

pub fn downloads() -> PathBuf {
    cache().join("github")
}

pub fn install(file: &Download, heard: impl Fn(&str) + 'static,
               done: impl FnOnce(Result<Option<PathBuf>, String>) + 'static) {
    use crate::common::filling::{step_line, within_line};
    let dir = match file.kind {
        Kind::AppImage => downloads(),
        Kind::Flatpak => cache(),
    };
    let _ = std::fs::create_dir_all(&dir);

    let base: String = file.name.chars().filter(|c| !matches!(c, '/' | '\0')).collect();
    let dest = dir.join(base.trim_start_matches('.'));
    let part = dest.with_extension("part");
    let (p, d, kind) = (part.display().to_string(), dest.clone(), file.kind.clone());
    let heard = std::rc::Rc::new(heard);
    heard(&step_line(1, 2, t!("store-gh-step-downloading")));
    let arriving = std::rc::Rc::new(std::cell::Cell::new(true));
    if file.size > 0 {
        let (h, a, watched, size) = (heard.clone(), arriving.clone(), part.clone(), file.size as f64);
        run::every(super::TICK, move || {
            if !a.get() {
                return false;
            }
            if let Ok(m) = std::fs::metadata(&watched) {
                h(&within_line(m.len() as f64 / size));
            }
            true
        });
    }
    capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "1800",
              "--max-filesize", run::MAX_DOWNLOAD, "-o", &p, &file.url],
            None, move |ok, _, err| {
        arriving.set(false);
        heard(&step_line(2, 2, t!("store-gh-step-installing")));
        if !ok {
            let _ = std::fs::remove_file(&part);
            return done(Err(t!("store-gh-download-failed", why = err.trim())));
        }
        if let Err(e) = std::fs::rename(&part, &d) {
            return done(Err(e.to_string()));
        }
        match kind {
            Kind::AppImage => done(Ok(Some(d))),
            Kind::Flatpak => {
                let path = d.display().to_string();
                capture(&crate::common::gentle::argv(&["flatpak", "install", "--user", "-y", "--noninteractive", "--bundle", &path]), None,
                        move |ok, _, err| {
                    let _ = std::fs::remove_file(&d);
                    done(if ok { Ok(None) } else {
                        Err(err.trim().lines().last().unwrap_or(t!("store-add-ref-refused")).to_string())
                    })
                });
            }
        }
    });
}

pub fn size_text(bytes: u64) -> String {
    let mb = bytes as f64 / 1_000_000.0;
    if mb >= 1.0 { crate::toolkit::lang::format::size_from_megabytes(mb) } else { t!("store-size-under").into() }
}

#[cfg(test)]
mod tests;
