//! trindle-report: "Report a problem", opened from a ring button;
//! "Something went wrong. Send a report?", opened by the ring after one of
//! Trindle's programs crashes (`trindle-report --crash <program>`). The person
//! sees the exact text a report is before anything is done with it; Send
//! posts that very text, once, to Buildmorphic's tester report API (the
//! one address in `send`), and only after the server said it is taking
//! reports.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::common::files::{home, setting};
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{button, card, column, entry, label, main_word, page_title, row, scroll, space, Length, Node, Tone};

pub mod probe;
pub mod scrub;
pub mod send;
mod window;

use probe::Crash;
use scrub::{scrub, Who};
use send::{Answer, Backoff, Server};

pub const LOG_LINES: usize = 200;

const WORDS_MOST: usize = 2000;

pub const HARDWARE_MOST: usize = 20000;
const LINE_MOST: usize = 1000;
pub const TRACE_MOST: usize = 30000;
const VERSION_MOST: usize = 40;
const PROGRAM_MOST: usize = 120;
const SIGNAL_MOST: usize = 40;

const NOTE_LEAST: usize = 10;

const NAME_MOST: usize = 39;

const BM_NAME: std::ops::RangeInclusive<usize> = 2..=31;
const SIZE: (u32, u32) = (520, 800);
const NAME: &str = "org.trindle.report";
const PATH: &str = "/org/trindle/report";
const XML: &str = r#"<interface name="org.trindle.report"><method name="Show"><arg type="b" name="crash" direction="in"/><arg type="s" name="program" direction="in"/><arg type="b" name="taken" direction="out"/></method></interface>"#;
const CANCEL_ICON: &str = "window-close-symbolic";
const COPY_ICON: &str = "edit-copy-symbolic";
const SEND_ICON: &str = "mail-send-symbolic";
const DETAILS_ICON: &str = "view-reveal-symbolic";

pub const OFFER_KEY: &str = "report_offer";
pub const USERNAME_KEY: &str = "report_username";
pub const GITHUB_KEY: &str = "report_github";

pub fn offered() -> bool {
    setting(OFFER_KEY) == "on"
}

pub fn username() -> String {
    setting(USERNAME_KEY).trim().to_string()
}

pub fn server() -> Server {
    Server::live()
}

fn live_for(server: &Server, slug: &str) -> Server {
    if *server == Server::live() { Server::of_slug(slug) } else { server.clone() }
}

pub fn github() -> String {
    setting(GITHUB_KEY).trim().to_string()
}

pub fn already() -> &'static str {
    t!("report-already-saved")
}
pub fn already_sent() -> &'static str {
    t!("report-already-sent")
}

pub fn not_taken() -> &'static str {
    t!("report-not-taken")
}
pub fn waiting() -> &'static str {
    t!("report-waiting")
}
pub fn no_at() -> &'static str {
    t!("report-no-at")
}
pub fn name_chars() -> &'static str {
    t!("report-name-chars")
}
pub fn title_crash() -> &'static str {
    t!("report-title-crash")
}
pub fn title_note() -> &'static str {
    t!("report-title-note")
}

pub fn summary() -> &'static str {
    t!("report-summary")
}

pub fn name_fault(n: &str) -> Option<&'static str> {
    let n = n.trim();
    if n.contains('@') {
        return Some(no_at());
    }
    if n.is_empty() {
        return None;
    }
    if !BM_NAME.contains(&n.chars().count()) || !n.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Some(name_chars());
    }
    None
}

pub fn github_fault(n: &str) -> Option<&'static str> {
    let n = n.trim();
    if n.chars().count() > NAME_MOST || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Some(t!("report-github-chars"));
    }
    None
}

pub fn pending_dir() -> PathBuf {
    home().join(".local/state/trindle/reports")
}

pub fn forget_pending(dir: &Path) -> usize {
    let mut n = 0;
    for f in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        if f.path().extension().is_some_and(|e| e == "json") && std::fs::remove_file(f.path()).is_ok() {
            n += 1;
        }
    }
    n
}

fn keep_for_later(dir: &Path, body: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let name = format!("{}-{}.json", crate::common::clock::Local::now().format("%Y%m%d-%H%M%S"), std::process::id());
    crate::common::files::write_whole_private(&dir.join(name), body.as_bytes())
}

pub fn flush_in(dir: &Path, on: bool, backoff: &mut Backoff, now: u64, accepting: &dyn Fn(&str) -> bool, post: &dyn Fn(&str) -> Answer) -> usize {
    if !on {
        return 0;
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).into_iter().flatten().flatten().map(|f| f.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json")).collect();
    if files.is_empty() {
        return 0;
    }
    files.sort();
    let mut sent = 0;

    let mut asked: Vec<(String, bool)> = Vec::new();
    for f in files {
        let Ok(body) = std::fs::read_to_string(&f) else { continue };
        let slug = send::slug_of(&body);
        let yes = match asked.iter().find(|(s, _)| *s == slug) {
            Some((_, yes)) => *yes,
            None => {
                let yes = backoff.due(&slug, now) && accepting(&slug);
                if !yes && backoff.due(&slug, now) {
                    backoff.failed(&slug, now);
                }
                asked.push((slug.clone(), yes));
                yes
            }
        };
        if !yes {
            continue;
        }
        let taken = match post(&body) {
            Answer::Taken(_) => {
                backoff.worked(&slug);
                true
            }

            Answer::Refused(_) => false,

            Answer::Later(_) => {
                backoff.failed(&slug, now);
                if let Some(entry) = asked.iter_mut().find(|(s, _)| *s == slug) {
                    entry.1 = false;
                }
                continue;
            }
        };
        if std::fs::remove_file(&f).is_ok() && taken {
            sent += 1;
        }
    }
    sent
}

pub fn flush() -> usize {
    let server = server();
    let on = offered();
    send::with_backoff(|b| flush_in(&pending_dir(), on, b, send::now_secs(), &|slug| send::accepting(&live_for(&server, slug)), &|body| send::post(&live_for(&server, &send::slug_of(body)), body)))
}

fn programs() -> Vec<&'static str> {
    crate::NAMES.iter().copied().filter(|n| !matches!(*n, "trindle-disks" | "trindle-store" | "trindle-files")).collect()
}

fn read_log() -> String {
    let n = LOG_LINES.to_string();
    let mut argv = vec!["journalctl", "--user", "--no-pager", "-q", "-o", "short-iso", "-n", n.as_str()];
    for p in programs() {
        argv.push("-t");
        argv.push(p);
    }
    let own = crate::common::run::text(&argv);
    first_with_lines(own, || crate::common::run::text(&["journalctl", "--user", "--no-pager", "-q", "-o", "short-iso", "-n", n.as_str(), "-u", "trindle-*"]))
}

fn first_with_lines(first: String, second: impl FnOnce() -> String) -> String {
    if first.lines().any(|l| !l.trim().is_empty()) { first } else { second() }
}

fn model() -> String {
    let get = |f: &str| std::fs::read_to_string(format!("/sys/class/dmi/id/{f}")).unwrap_or_default().trim().to_string();
    let (vendor, name, version) = (get("sys_vendor"), get("product_name"), get("product_version"));
    let model = if vendor.eq_ignore_ascii_case("lenovo") && !version.is_empty() { version } else { name };
    let text = format!("{vendor} {model}").trim().to_string();
    if text.is_empty() { "unknown".into() } else { text }
}

pub fn log_lines(raw: &str, who: &Who) -> Vec<String> {
    let all: Vec<&str> = raw.lines().collect();
    all[all.len().saturating_sub(LOG_LINES)..].iter().map(|l| scrub(l, who)).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {

    Note,

    Crash,
}

impl Mode {
    fn word(self) -> &'static str {
        match self {
            Mode::Note => "note",
            Mode::Crash => "crash",
        }
    }
}

pub struct Parts<'a> {
    pub mode: Mode,
    pub version: &'a str,
    pub words: &'a str,
    pub hardware: &'a str,
    pub crash: Option<&'a Crash>,
    pub log: &'a [String],
    pub username: &'a str,
    pub github: &'a str,
}

fn cut(text: &str, n: usize) -> String {
    text.chars().take(n).collect()
}

fn plain(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}' | '\u{feff}'))
        .map(|c| if c.is_control() && c != '\n' { ' ' } else { c })
        .collect()
}

fn untagged(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        match after.find('>').filter(|&end| is_tag(&after[..end])) {
            Some(end) => rest = &after[end + 1..],
            None => {
                out.push('<');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn is_tag(inner: &str) -> bool {
    if inner.starts_with(['!', '?']) {
        return true;
    }
    let body = inner.strip_prefix('/').unwrap_or(inner);
    let name = body.chars().take_while(|c| c.is_ascii_alphanumeric()).count();
    if name == 0 || !body.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return false;
    }
    let tail = body[name..].trim();
    tail.is_empty() || tail == "/" || tail.contains('=')
}

pub fn document(p: &Parts, who: &Who) -> String {
    let mut doc = serde_json::Map::new();
    doc.insert("kind".into(), json!(p.mode.word()));
    doc.insert("project".into(), json!(p.crash.map_or("trindle".to_string(), |c| send::slug_for(&c.program))));
    let mut put = |key: &str, value: String| {
        if !value.trim().is_empty() {
            doc.insert(key.into(), json!(value));
        }
    };
    put("version", cut(p.version, VERSION_MOST));
    put("words", cut(&scrub(untagged(&plain(p.words).replace('\n', " ")).trim(), who), WORDS_MOST));
    put("hardware", cut(&scrub(&plain(p.hardware), who), HARDWARE_MOST));
    put("username", p.username.trim().to_string());
    put("github", p.github.trim().to_string());
    let mut log: Vec<String> = p.log.iter().map(|l| cut(&scrub(&plain(l), who), LINE_MOST)).collect();
    log.drain(..log.len().saturating_sub(LOG_LINES));
    let mut crash = p.crash.map(|c| (cut(&scrub(&plain(&c.program), who), PROGRAM_MOST), cut(&scrub(&plain(&c.signal), who), SIGNAL_MOST), cut(&scrub(&plain(&c.trace), who), TRACE_MOST)));
    loop {
        let mut whole = doc.clone();
        if let Some((program, signal, trace)) = &crash {

            let parts: serde_json::Map<String, Value> = [("program", program), ("signal", signal), ("trace", trace)].into_iter()
                .filter(|(_, v)| !v.trim().is_empty()).map(|(k, v)| (k.to_string(), json!(v))).collect();
            whole.insert("crash".into(), Value::Object(parts));
        }
        if !log.is_empty() {
            whole.insert("log".into(), json!(log));
        }
        let text = serde_json::to_string_pretty(&Value::Object(whole)).unwrap_or_default();
        if text.len() <= send::MOST_BYTES {
            return text;
        }
        if log.len() > 1 {
            log.drain(..log.len().div_ceil(4));
        } else if let Some((_, _, trace)) = crash.as_mut().filter(|c| !c.2.is_empty()) {
            *trace = trace.chars().take(trace.chars().count() / 2).collect();
        } else if !log.is_empty() {
            log.clear();
        } else {
            return text;
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Typed(String),
    Username(String),
    Details,
    Send,
    Save,
    Copy,
    Cancel,
}

enum Do {
    Log(String),
    Hardware(Option<String>),
    Crash(Crash, String),
    Copy(String),
    Post(String),

    Status(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Gate {

    Asking,
    Open,

    Shut,
}

struct Reporter {
    mode: Mode,

    program: String,
    typed: String,
    username: String,
    log: Option<Vec<String>>,
    hardware: Option<String>,
    crash: Option<Crash>,
    who: Who,
    model: String,
    version: &'static str,

    dir: PathBuf,

    waiting_dir: PathBuf,

    switch: bool,

    keeps_username: bool,
    gate: Gate,
    github: String,

    rejected: bool,
    details: bool,
    saved: Option<String>,

    sent: bool,

    posted: String,

    sending: bool,

    delivered: bool,
    bug: Option<u64>,

    waiting: Option<String>,
    refusal: String,
    events: Option<Loop>,
    asks: Option<Post<Do>>,

    outbox: Vec<String>,
}

impl Reporter {
    fn new(who: Who, model: String, dir: PathBuf) -> Reporter {
        Reporter {
            mode: Mode::Note, program: String::new(), typed: String::new(), username: String::new(), log: None, hardware: None,
            crash: None, who, model, version: env!("CARGO_PKG_VERSION"), waiting_dir: dir.join("waiting"), dir, switch: false, keeps_username: false,
            gate: Gate::Asking, github: String::new(), rejected: false, details: false, saved: None, sent: false, posted: String::new(), sending: false,
            delivered: false, bug: None, waiting: None, refusal: String::new(), events: None, asks: None, outbox: Vec::new(),
        }
    }

    fn hardware_text(&self) -> String {
        probe::hardware_text(&self.model, self.hardware.as_deref())
    }

    fn text(&self) -> String {
        let crash = match self.mode {
            Mode::Crash => Some(self.crash.clone().unwrap_or(Crash { program: self.program.clone(), signal: String::new(), trace: String::new() })),
            _ => None,
        };
        let log: &[String] = self.log.as_deref().unwrap_or(&[]);
        document(&Parts {
            mode: self.mode, version: self.version, words: &self.typed, hardware: &self.hardware_text(),
            crash: crash.as_ref(), log, username: &self.username, github: &self.github,
        }, &self.who)
    }

    fn refused(&self) -> Option<&'static str> {
        name_fault(&self.username).or_else(|| github_fault(&self.github))
    }

    fn send_off(&self) -> Option<&'static str> {
        if self.rejected {
            Some("")
        } else if self.sent {
            Some(already_sent())
        } else if self.mode == Mode::Note && self.typed.trim().chars().count() < NOTE_LEAST {
            Some(t!("report-say-more"))
        } else if self.mode == Mode::Note && self.log.is_none() {
            Some(t!("report-reading-log"))
        } else if self.mode == Mode::Note && self.log.as_ref().is_some_and(|l| l.is_empty()) {
            Some(t!("report-no-log"))
        } else {
            match self.gate {
                Gate::Asking => Some(t!("report-checking")),
                Gate::Shut => Some(not_taken()),
                Gate::Open => None,
            }
        }
    }

    fn send(&mut self) {
        if self.rejected {
            return;
        }
        if let Some(why) = self.send_off() {
            self.refusal = why.into();
            return;
        }
        if let Some(why) = self.refused() {
            self.refusal = why.into();
            return;
        }
        self.refusal.clear();
        self.sent = true;
        self.sending = true;
        self.posted = self.text();
        match &self.asks {
            Some(asks) => {
                asks.send(Do::Post(self.posted.clone()));
            }
            None => self.outbox.push(self.posted.clone()),
        }
    }

    fn username_is(&mut self, t: String) {
        self.username = t;
        match name_fault(&self.username) {
            Some(why) => self.refusal = why.into(),
            None => {
                self.refusal.clear();
                if self.keeps_username {
                    let _ = crate::common::files::set_setting(USERNAME_KEY, self.username.trim());
                }
            }
        }
    }

    fn gate_is(&mut self, open: bool) {
        self.gate = if open { Gate::Open } else { Gate::Shut };
    }

    fn answered(&mut self, result: Answer) {
        self.sending = false;
        let why = match result {
            Answer::Taken(bug) => {
                self.delivered = true;
                self.bug = bug;
                return;
            }
            Answer::Refused(why) => {
                self.rejected = true;
                self.refusal = why;
                return;
            }
            Answer::Later(why) => why,
        };
        if !self.switch {
            self.sent = false;
            self.refusal = t!("report-try-again", why = why);
            return;
        }
        match keep_for_later(&self.waiting_dir, &self.posted) {
            Ok(()) => self.waiting = Some(why),
            Err(e) => {
                self.sent = false;
                self.refusal = t!("report-keep-failed", reason = reason(&e));
            }
        }
    }

    fn save(&mut self) {
        if self.saved.is_some() {
            self.refusal = already().into();
            return;
        }
        if let Some(why) = self.refused() {
            self.refusal = why.into();
            return;
        }
        let name = format!("trindle-report-{}.json", crate::common::clock::Local::now().format("%Y%m%d-%H%M%S"));
        let made = std::fs::create_dir_all(&self.dir)
            .and_then(|()| crate::common::files::write_whole_private(&self.dir.join(&name), self.text().as_bytes()));
        match made {
            Ok(()) => {
                self.refusal.clear();
                self.saved = Some(name);
            }
            Err(e) => self.refusal = t!("report-save-failed", reason = reason(&e)),
        }
    }

    fn quit(&self) {
        if let Some(events) = &self.events {
            events.quit();
        }
    }
}

fn reason(e: &std::io::Error) -> &'static str {
    match e.kind() {
        std::io::ErrorKind::PermissionDenied => t!("report-reason-permission"),
        std::io::ErrorKind::StorageFull => t!("report-reason-full"),
        std::io::ErrorKind::NotFound => t!("report-reason-missing"),
        _ => t!("report-reason-refused"),
    }
}

impl App for Reporter {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Typed(t) => {
                self.typed = t;
                self.refusal.clear();
            }
            Msg::Username(t) => self.username_is(t),
            Msg::Details => self.details = !self.details,
            Msg::Send => self.send(),
            Msg::Save => self.save(),
            Msg::Copy => match self.refused() {
                Some(why) => self.refusal = why.into(),
                None => {
                    self.refusal.clear();
                    if let Some(asks) = &self.asks {
                        asks.send(Do::Copy(self.text()));
                    }
                }
            },
            Msg::Cancel => self.quit(),
        }
    }

    fn view(&self) -> Node<Msg> {
        let title = match self.mode {
            Mode::Crash => title_crash(),
            Mode::Note => title_note(),
        };
        let below = if !self.refusal.is_empty() {
            label(self.refusal.clone()).tone(Tone::Problem)
        } else if let Some(name) = &self.saved {
            label(t!("report-saved-as", name = name))
        } else if self.delivered {
            label(match self.bug { Some(n) => t!("report-sent-as-bug", number = n).to_string(), None => t!("report-sent").to_string() })
        } else if let Some(why) = &self.waiting {
            label(t!("report-waiting-why", why = why))
        } else if self.sending {
            label(t!("report-sending"))
        } else {
            label(" ")
        };
        let off = self.send_off();
        let mut bottom = vec![];
        match off {
            Some("") => {}
            Some(why) => bottom.push(label(why)),
            None if self.waiting.is_some() => bottom.push(label(waiting())),
            None => {}
        }
        bottom.push(space());
        bottom.push(button(CANCEL_ICON, if self.mode == Mode::Note { t!("report-cancel") } else { t!("report-dont-send") }).on_press(Msg::Cancel));

        if off.is_none() {
            bottom.push(button(SEND_ICON, t!("report-send")).on_press(Msg::Send));
        }
        let mut inner = vec![
            label(t!("report-beta")),
            entry(self.typed.clone(), t!("report-what-doing"), Msg::Typed).first(),
            entry(self.username.clone(), t!("report-username-box"), Msg::Username),
            below,
            label(summary()),
            row(vec![space(), button(DETAILS_ICON, if self.details { t!("report-hide-details") } else { t!("report-show-details") }).on_press(Msg::Details)]),
        ];
        if self.details {
            inner.push(scroll("report-details", label(self.text())).height(Length::Fixed(160.0)));
            inner.push(row(vec![space(), button(COPY_ICON, t!("report-copy")).on_press(Msg::Copy),
                                main_word(t!("report-save")).on_press(Msg::Save).off(self.saved.is_some())]).gap(12.0));
        }
        inner.push(row(bottom).gap(12.0));
        column(vec![page_title(title), card(column(inner).gap(12.0))]).gap(16.0).padding(theme::sheet().page_padding)
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::Key { sym: Keysym::Escape, .. } => Some(Msg::Cancel),
            _ => None,
        }
    }
}

fn mode_of(args: &[String]) -> (Mode, String) {
    if let Some(at) = args.iter().position(|a| a == "--crash") {
        let name = args.get(at + 1).map(String::as_str).unwrap_or("trindle");
        let program = if crate::NAMES.contains(&name) { name } else { "trindle" };
        return (Mode::Crash, program.to_string());
    }
    (Mode::Note, String::new())
}

pub fn opens_by_itself(mode: Mode) -> bool {
    mode == Mode::Crash
}

fn same_window(mode: Mode, program: &str, crash: bool, wanted: &str) -> bool {
    match (mode, crash) {
        (Mode::Note, false) => true,
        (Mode::Crash, true) => program == wanted,
        _ => false,
    }
}

pub fn main(args: Vec<String>) -> i32 {
    let (mode, program) = mode_of(&args);

    if opens_by_itself(mode) && !offered() {
        return 0;
    }
    let events = Loop::new();

    let Ok(bus) = Bus::session(&events) else {
        let Some(_shown) = window::open(&events, mode, program) else { return 1 };
        events.run();
        return 0;
    };
    window::share(&events, &bus, mode, program)
}

#[cfg(test)]
pub const FIELDS: [&str; 9] = ["kind", "project", "version", "words", "hardware", "crash", "log", "username", "github"];

#[cfg(test)]
pub const CRASH_FIELDS: [&str; 3] = ["program", "signal", "trace"];

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_send;
#[cfg(test)]
mod tests_window;
