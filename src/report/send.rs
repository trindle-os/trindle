//! How a report reaches Buildmorphic's tester report API.

use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Mutex;

use serde_json::Value;

use crate::toolkit::t;

const REPORTS_BASE: &str = "https://buildmorphic.com/api/reports/";
pub const REPORTS_URL: &str = "https://buildmorphic.com/api/reports/trindle";
const OWNERS: [(&str, &str); 1] = [("trindle-pocketkey", "trindle-pocketkey")];
pub const COMMUNITY_URL: &str = "https://trindle.buildmorphic.com";
pub const SHEETS_URL: &str = "https://buildmorphic.com/communities/trindle/sheets";
const AGENT: &str = "Trindle";
pub const MOST_BYTES: usize = 255 * 1024;

#[derive(Clone, Debug, PartialEq)]
pub struct Server {
    pub post: String,
    pub status: String,
}

pub fn slug_for(program: &str) -> String {
    match OWNERS.iter().find(|(start, _)| program.starts_with(start)) {
        Some((_, slug)) => (*slug).to_string(),
        None => plugin_slug(program).unwrap_or_else(|| "trindle".to_string()),
    }
}

const PLUGIN_SLUGS: [(&str, &str); 9] = [
    ("Flow", "trindle-flow"),
    ("Scan", "trindle-scan"),
    ("Talk", "trindle-talk"),
    ("Voice", "trindle-voice"),
    ("Snippets", "trindle-snippets"),
    ("Downloader", "trindle-downloader"),
    ("CrashReports", "trindle-crash-reports"),
    ("ScreenReader", "trindle-screen-reader"),
    ("PocketKey", "trindle-pocketkey"),
];

fn plugin_slug(program: &str) -> Option<String> {
    let id = program.strip_prefix("trindle-plugin ")?;
    let name = id.strip_prefix(["org.", "trindle", "."].concat().as_str())?;
    PLUGIN_SLUGS.iter().find(|(n, _)| *n == name).map(|(_, slug)| (*slug).to_string())
}

pub fn slug_of(body: &str) -> String {
    let named = serde_json::from_str::<Value>(body).ok().and_then(|v| v.get("project").and_then(Value::as_str).map(str::to_string));
    named.filter(|n| OWNERS.iter().map(|o| o.1).chain(PLUGIN_SLUGS.iter().map(|p| p.1)).any(|known| known == n)).unwrap_or_else(|| "trindle".to_string())
}

impl Server {
    pub fn live() -> Server {
        Server::at(REPORTS_URL)
    }

    pub fn of_slug(slug: &str) -> Server {
        Server::at(&format!("{REPORTS_BASE}{slug}"))
    }

    pub fn at(address: &str) -> Server {
        let post = address.trim().trim_end_matches('/').to_string();
        Server { status: format!("{post}/status"), post }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    Taken(Option<u64>),
    Refused(String),
    Later(String),
}

fn request(url: &str, body: Option<&str>) -> Result<(u16, String), String> {
    #[cfg(test)]
    ASKED.with(|n| n.set(n.get() + 1));
    let mut command = Command::new("curl");
    command.args(["-sS", "--max-time", "20", "--proto", if cfg!(test) { "=http,https" } else { "=https" }, "--noproxy", "127.0.0.1,localhost",
                  "-A", AGENT, "-H", "Accept: application/json", "-H", "Expect:", "-w", "\n%{http_code}"]);
    if body.is_some() {
        command.args(["-X", "POST", "-H", "Content-Type: application/json", "--data-binary", "@-"]);
    }
    command.arg(url).stdin(if body.is_some() { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = command.spawn().map_err(|_| t!("report-curl-missing").to_string())?;
    if let (Some(mut input), Some(body)) = (child.stdin.take(), body) {
        let _ = input.write_all(body.as_bytes());
    }
    let out = child.wait_with_output().map_err(|_| t!("report-not-posted").to_string())?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let (said, code) = text.rsplit_once('\n').unwrap_or(("", &text));
    match code.trim().parse::<u16>().unwrap_or(0) {
        0 => Err(t!("report-no-connection").to_string()),
        n => Ok((n, said.to_string())),
    }
}

#[cfg(test)]
thread_local! {
    pub(super) static ASKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn error_of(body: &str, otherwise: &str) -> String {
    let said = serde_json::from_str::<Value>(body).ok().and_then(|v| v.get("error").and_then(Value::as_str).map(str::to_string));
    let clean: String = said.unwrap_or_default().chars().filter(|c| !c.is_control()).take(120).collect();
    if clean.trim().is_empty() { otherwise.to_string() } else { clean.trim().to_string() }
}

pub fn accepting(server: &Server) -> bool {
    if cfg!(test) && server.post.starts_with(REPORTS_BASE) {
        return false;
    }
    let Ok((200, body)) = request(&server.status, None) else { return false };
    serde_json::from_str::<Value>(&body).ok().and_then(|v| v.get("accepting").and_then(Value::as_bool)) == Some(true)
}

pub fn post(server: &Server, body: &str) -> Answer {
    if cfg!(test) && server.post.starts_with(REPORTS_BASE) {
        return Answer::Later(t!("report-not-taken").to_string());
    }
    match request(&server.post, Some(body)) {
        Ok((201, said)) => Answer::Taken(bug_number(&said)),
        Ok((429, said)) => Answer::Later(error_of(&said, t!("report-too-many"))),
        Ok((n, said)) if (500..600).contains(&n) => Answer::Later(error_of(&said, &t!("report-answered", code = n))),
        Ok((n, said)) => Answer::Refused(error_of(&said, &t!("report-answered", code = n))),
        Err(why) => Answer::Later(why),
    }
}

fn bug_number(said: &str) -> Option<u64> {
    serde_json::from_str::<Value>(said).ok()?.get("issue")?.get("number")?.as_u64()
}

const FIRST_DELAY: u64 = 600;
const LONGEST_DELAY: u64 = 6 * 3600;

struct Pause {
    fails: u32,
    next: u64,
}

pub struct Backoff(Vec<(String, Pause)>);

impl Backoff {
    pub const fn new() -> Backoff {
        Backoff(Vec::new())
    }

    fn pause(&mut self, slug: &str) -> &mut Pause {
        let at = match self.0.iter().position(|(s, _)| s == slug) {
            Some(at) => at,
            None => {
                self.0.push((slug.to_string(), Pause { fails: 0, next: 0 }));
                self.0.len() - 1
            }
        };
        &mut self.0[at].1
    }

    pub fn due(&self, slug: &str, now: u64) -> bool {
        self.0.iter().find(|(s, _)| s == slug).is_none_or(|(_, p)| now >= p.next)
    }

    pub fn failed(&mut self, slug: &str, now: u64) {
        let pause = self.pause(slug);
        pause.fails = pause.fails.saturating_add(1);
        let delay = FIRST_DELAY.saturating_mul(1u64 << (pause.fails - 1).min(20)).min(LONGEST_DELAY);
        pause.next = now + delay;
    }

    pub fn worked(&mut self, slug: &str) {
        self.0.retain(|(s, _)| s != slug);
    }

    #[cfg(test)]
    pub fn fails(&self, slug: &str) -> u32 {
        self.0.iter().find(|(s, _)| s == slug).map_or(0, |(_, p)| p.fails)
    }
}

static BACKOFF: Mutex<Backoff> = Mutex::new(Backoff::new());

pub fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn with_backoff<T>(f: impl FnOnce(&mut Backoff) -> T) -> T {
    f(&mut BACKOFF.lock().unwrap_or_else(|e| e.into_inner()))
}
