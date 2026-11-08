//! The polkit authentication agent for the session, run inside the ring.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::ffi::CStr;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::rc::{Rc, Weak};
use std::time::Duration;

use zbus::zvariant::{OwnedValue, Value};
use zeroize::Zeroizing;

use crate::toolkit::t;
use crate::common::drive_leave::AUTHORITY;
use crate::common::pocketkey;
use crate::common::refused::ADMIN_ACTION;
use crate::common::run::{self, Proc};
use crate::ring::ctx::Ctx;
use crate::toolkit::bus::{Asked, Failed, Offer, Served};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::theme;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::tiles::tile_head;
use crate::toolkit::widget::{button, card, column, drawing, entry, label, layers, main_action, note, placed, row, space, spinner, Length, Node, Tone};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

const AGENT_PATH: &str = "/org/trindle/PolicyKit1/AuthenticationAgent";
const AGENT: &str = "org.freedesktop.PolicyKit1.AuthenticationAgent";
const AGENT_XML: &str = r#"<interface name="org.freedesktop.PolicyKit1.AuthenticationAgent"><method name="BeginAuthentication"><arg type="s" name="action_id" direction="in"/><arg type="s" name="message" direction="in"/><arg type="s" name="icon_name" direction="in"/><arg type="a{ss}" name="details" direction="in"/><arg type="s" name="cookie" direction="in"/><arg type="a(sa{sv})" name="identities" direction="in"/></method><method name="CancelAuthentication"><arg type="s" name="cookie" direction="in"/></method></interface>"#;
const TRIES: u32 = 3;
const REGISTER_WAIT: Duration = Duration::from_secs(10);

const HELPERS: [&str; 3] = ["/usr/lib/polkit-1/polkit-agent-helper-1", "/usr/libexec/polkit-agent-helper-1",
                            "/usr/lib/policykit-1/polkit-agent-helper-1"];

const LOGIN_WIDTH: f32 = 460.0;
const LOGIN_PADDING: f32 = 24.0;
const BACKDROP: f32 = 0.55;

const PROBLEM_ROOM: f32 = 24.0;

const CANCELLED: &str = "Cancelled";

const CALLER: &str = "polkit.caller-pid";

fn look() -> theme::Theme {
    theme::held()
}

fn said_for(action: &str, message: &str, details: &HashMap<String, String>) -> String {
    let step = || {
        let pid: u32 = details.get(CALLER)?.parse().ok()?;
        crate::common::run::admin_step(&std::fs::read(format!("/proc/{pid}/cmdline")).ok()?)
    };
    let reason = if action == ADMIN_ACTION { step().and_then(|s| crate::common::updating::admin_reason(&s)) } else { None };

    if let Some(drive) = crate::common::drive_leave::said(action, message, details) {
        return drive;
    }
    reason.unwrap_or(message).to_string()
}

type Begin = (String, String, String, HashMap<String, String>, String, Vec<(String, HashMap<String, OwnedValue>)>);

fn name_of(uid: u32) -> String {
    // SAFETY: pw_name is set for every entry getpwuid returns, and the string

    unsafe {
        let p = libc::getpwuid(uid);
        if p.is_null() {
            return uid.to_string();
        }
        CStr::from_ptr((*p).pw_name).to_string_lossy().to_string()
    }
}

fn group_members(gid: u32) -> Vec<String> {
    let mut out = Vec::new();
    // SAFETY: gr_mem is an array of string pointers ending in a null one, which

    unsafe {
        let g = libc::getgrgid(gid);
        if g.is_null() {
            return out;
        }
        let mut m = (*g).gr_mem;
        while !m.is_null() && !(*m).is_null() {
            out.push(CStr::from_ptr(*m).to_string_lossy().to_string());
            m = m.add(1);
        }
    }
    out
}

fn pick_user(identities: &[(String, HashMap<String, OwnedValue>)]) -> Option<u32> {
    // SAFETY: asks the kernel for this process's own user id.
    let me = unsafe { libc::getuid() };
    let number = |d: &HashMap<String, OwnedValue>, k: &str| d.get(k).and_then(|v| u32::try_from(v).ok());
    let mut users = Vec::new();
    for (kind, details) in identities {
        match kind.as_str() {
            "unix-user" => users.extend(number(details, "uid")),
            "unix-group" => {
                if let Some(gid) = number(details, "gid") {
                    users.extend(group_members(gid).iter().filter_map(|n| crate::common::files::uid_of(n)));
                }
            }
            _ => {}
        }
    }
    if users.contains(&me) { Some(me) } else { users.first().copied() }
}

fn unescape(text: &str) -> String {
    let b = text.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' || i + 1 == b.len() {
            out.push(b[i]);
            i += 1;
            continue;
        }
        i += 1;
        let c = b[i];
        i += 1;
        match c {
            b'0'..=b'7' => {
                let mut v = (c - b'0') as u32;
                let mut n = 1;
                while n < 3 && i < b.len() && (b'0'..=b'7').contains(&b[i]) {
                    v = v * 8 + (b[i] - b'0') as u32;
                    i += 1;
                    n += 1;
                }
                out.push(v as u8);
            }
            b'b' => out.push(8),
            b'f' => out.push(12),
            b'n' => out.push(b'\n'),
            b'r' => out.push(b'\r'),
            b't' => out.push(b'\t'),
            b'v' => out.push(11),
            other => out.push(other),
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Clone)]
enum Msg {
    Pass(String),
    Allow,
    Cancel,
}

enum Out {
    Answer(Zeroizing<String>),
    Cancel,
}

struct Prompt {
    message: String,

    note: String,
    password: Zeroizing<String>,
    problem: Option<String>,

    waiting: bool,

    turn: u32,

    behind: usize,
    out: Post<Out>,
}

impl App for Prompt {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Pass(t) => {
                if !self.waiting {
                    self.password = Zeroizing::new(t);
                }
            }
            Msg::Allow => {
                if self.waiting {
                    return;
                }
                self.waiting = true;
                let password = std::mem::replace(&mut self.password, Zeroizing::new(String::new()));
                self.out.send(Out::Answer(password));
            }
            Msg::Cancel => {
                self.out.send(Out::Cancel);
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let password = entry(self.password.to_string(), "", Msg::Pass).key("pass").spoken(self.note.clone())
            .secret().first().grab(self.turn).off(self.waiting).width(Length::Fill);
        let password = if self.waiting { password } else { password.on_submit(Msg::Allow) };

        let problem = match &self.problem {
            Some(p) => label(p.clone()).tone(Tone::Problem).width(Length::Fill),
            None => space(),
        };
        let problem = column(vec![problem]).height(Length::Fixed(PROBLEM_ROOM));

        let waiting = match self.behind {
            0 => space(),
            n => note(t!("polkit-waiting-behind", n = n)).width(Length::Fill),
        };
        let waiting = column(vec![waiting]).height(Length::Fixed(PROBLEM_ROOM));
        let allow_button = main_action("changes-allow-symbolic", t!("polkit-allow")).off(self.waiting);
        let allow_button = if self.waiting { allow_button } else { allow_button.on_press(Msg::Allow) };

        let mut buttons = vec![space()];
        if self.waiting {
            buttons.push(spinner());
        }
        buttons.push(button("window-close-symbolic", t!("polkit-cancel")).on_press(Msg::Cancel));
        buttons.push(allow_button);
        let login = card(column(vec![
            tile_head("dialog-password-symbolic", t!("polkit-title"), vec![]),
            label(self.message.clone()).dialog_message().width(Length::Fill),
            note(self.note.clone()).width(Length::Fill),
            password,
            problem,
            waiting,
            row(buttons).gap(8.0).width(Length::Fill),
        ]).gap(12.0).padding(LOGIN_PADDING - theme::sheet().card_padding)).width(Length::Fixed(LOGIN_WIDTH)).floats();
        let backdrop = drawing((0.0, 0.0), "", |canvas, pen| {
            canvas.fill(theme::alpha(pen.theme.shadow, BACKDROP).to_skia());
        }).width(Length::Fill).height(Length::Fill);
        layers(vec![backdrop, placed(login, |(w, h), (aw, ah)| (((aw - w) / 2.0).max(0.0), ((ah - h) / 2.0).max(0.0)))])
            .on_escape(Msg::Cancel)
    }
}

struct Request {
    agent: Weak<Agent>,
    me: Weak<Request>,
    lp: Loop,
    display: Rc<Display>,
    asked: RefCell<Option<Asked>>,
    cookie: String,
    user: String,
    message: String,

    said: RefCell<String>,
    tries: Cell<u32>,

    run: Cell<u64>,
    pending_problem: RefCell<Option<String>>,
    finished: Cell<bool>,
    helper: RefCell<Option<Proc>>,

    input: RefCell<Option<File>>,
    shown: RefCell<Option<Rc<Shown<Prompt>>>>,
    out: Post<Out>,

    note: Option<String>,

    caller: Option<u32>,

    behind: Cell<usize>,

    typed: RefCell<Option<Zeroizing<String>>>,

    reuse: RefCell<Option<Zeroizing<String>>>,
}

impl Request {
    fn new(agent: &Rc<Agent>, asked: Asked, action: &str, message: &str, cookie: String, uid: u32, caller: Option<u32>) -> Rc<Request> {
        let user = name_of(uid);
        let r = Rc::new_cyclic(|me: &Weak<Request>| {
            let w = me.clone();
            let out = agent.ctx.lp.post(move |o: Out| {
                let Some(r) = w.upgrade() else { return };
                match o {
                    Out::Answer(password) => {
                        r.respond(&password);
                        *r.typed.borrow_mut() = Some(password);
                    }
                    Out::Cancel => r.cancel(),
                }
            });
            Request {
                agent: Rc::downgrade(agent),
                me: me.clone(),
                lp: agent.ctx.lp.clone(),
                display: agent.ctx.display.clone(),
                asked: RefCell::new(Some(asked)),
                said: RefCell::new(t!("polkit-password-for", user = &user)),
                cookie, user,
                message: message.to_string(),
                tries: Cell::new(0),
                run: Cell::new(0),
                pending_problem: RefCell::new(None),
                finished: Cell::new(false),
                helper: RefCell::new(None),
                input: RefCell::new(None),
                shown: RefCell::new(None),
                out,

                note: (pocketkey::on() && !message.contains(char::is_control)).then(|| format!("polkit1 {action} {message}\n")),
                caller,
                behind: Cell::new(0),
                typed: RefCell::new(None),
                reuse: RefCell::new(None),
            }
        });
        r
    }

    fn begin(self: &Rc<Self>, reuse: Option<Zeroizing<String>>) {
        *self.reuse.borrow_mut() = reuse;
        self.start(None);
    }

    fn start(self: &Rc<Self>, problem: Option<&str>) {
        self.tries.set(self.tries.get() + 1);
        *self.pending_problem.borrow_mut() = problem.map(String::from);
        let run = self.run.get() + 1;
        self.run.set(run);

        if let Some(note) = &self.note {
            let _ = pocketkey::ask(note);
        }
        if self.launch(run).is_none() {
            self.ended(run, false);
        }
    }

    fn launch(&self, run: u64) -> Option<()> {
        let helper = HELPERS.iter().find(|p| std::path::Path::new(p).exists())?;
        let (in_read, in_write) = crate::common::run::pipe()?;
        let (out_read, out_write) = crate::common::run::pipe()?;
        // SAFETY: a new descriptor of our own, given the flag that keeps a

        unsafe {
            let flags = libc::fcntl(out_read.as_raw_fd(), libc::F_GETFL);
            libc::fcntl(out_read.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK);
        }
        let mut cmd = Command::new(helper);
        cmd.arg(&self.user).stdin(Stdio::from(in_read)).stdout(Stdio::from(out_write)).stderr(Stdio::inherit());
        let proc = run::child(cmd, |_| {})?;
        let mut input = File::from(in_write);
        if input.write_all(format!("{}\n", self.cookie).as_bytes()).is_err() {
            proc.signal(libc::SIGTERM);
            return None;
        }
        *self.helper.borrow_mut() = Some(proc);
        *self.input.borrow_mut() = Some(input);

        self.hear_helper(File::from(out_read), run);
        Some(())
    }

    fn hear_helper(&self, mut from: File, run: u64) {
        let fd = from.as_raw_fd();
        let w = self.me.clone();
        let mut heard: Vec<u8> = Vec::new();
        self.lp.watch(fd, move |_| {
            let Some(r) = w.upgrade() else { return Flow::Stop };
            if r.run.get() != run || r.finished.get() {
                return Flow::Stop;
            }
            let mut chunk = [0u8; 1024];
            let mut over = false;
            loop {
                match from.read(&mut chunk) {
                    Ok(0) => {
                        over = true;
                        break;
                    }
                    Ok(n) => heard.extend_from_slice(&chunk[..n]),
                    Err(e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                    Err(_) => {
                        over = true;
                        break;
                    }
                }
            }
            while let Some(at) = heard.iter().position(|b| *b == b'\n') {
                let line: Vec<u8> = heard.drain(..=at).collect();
                let line = String::from_utf8_lossy(&line[..line.len() - 1]).into_owned();
                if !r.heard(&line, run) {
                    return Flow::Stop;
                }
            }
            if over {
                r.ended(run, false);
                return Flow::Stop;
            }
            Flow::Continue
        });
    }

    fn heard(self: &Rc<Self>, line: &str, run: u64) -> bool {
        let after = |p: &str| line.strip_prefix(p).map(unescape);
        if let Some(text) = after("PAM_PROMPT_ECHO_OFF ").or_else(|| after("PAM_PROMPT_ECHO_ON ")) {
            self.ask(&text);
        } else if let Some(text) = after("PAM_ERROR_MSG ") {
            *self.pending_problem.borrow_mut() = Some(text);
        } else if let Some(text) = after("PAM_TEXT_INFO ") {

            if text.starts_with(pocketkey::PREFIX) {
                self.shown.borrow_mut().take();
            }
        } else {
            self.ended(run, line == "SUCCESS");
            return false;
        }
        true
    }

    fn ended(self: &Rc<Self>, run: u64, gained: bool) {
        if self.run.get() != run {
            return;
        }
        self.run.set(run + 1);
        self.input.borrow_mut().take();
        if let Some(h) = self.helper.borrow_mut().take() {
            h.signal(libc::SIGTERM);
        }

        let r = self.clone();
        self.lp.after(Duration::ZERO, move || r.completed(gained));
    }

    fn ask(self: &Rc<Self>, prompt: &str) {

        if let Some(password) = self.reuse.borrow_mut().take() {
            self.respond(&password);
            *self.typed.borrow_mut() = Some(password);
            return;
        }

        let p = prompt.trim();
        if !p.is_empty() && !p.to_lowercase().starts_with("password") {
            *self.said.borrow_mut() = p.trim_end_matches(':').to_string();
        }
        let problem = self.pending_problem.borrow().clone().filter(|p| !p.is_empty());
        let said = self.said.borrow().clone();
        let shown = self.shown.borrow().clone();
        if let Some(shown) = shown {
            shown.change(|a| {
                a.note = said;
                a.problem = problem;
                a.password = Zeroizing::new(String::new());
                a.waiting = false;
                a.turn += 1;
            });
            return;
        }
        let app = Prompt {
            message: self.message.clone(), note: said, password: Zeroizing::new(String::new()), problem,
            waiting: false, turn: 0, behind: self.behind.get(), out: self.out.clone(),
        };
        let kind = Kind::Panel(Panel {
            layer: Layer::Overlay, edges: Edges::ALL, keep: -1, margins: [0; 4], keys: Keys::Always,
            name: "trindle-polkit".into(), screen: None,
        });
        let Some(shown) = Shown::open(&self.lp, &self.display, kind, (0, 0), app) else {
            eprintln!("trindle-polkit: could not open the password window");
            self.cancel_with(Some("Authentication failed"), false);
            return;
        };

        shown.set_look(look);
        shown.see_through();
        shown.spoken(t!("polkit-spoken"));
        let out = self.out.clone();
        shown.window.on_close(move || {
            out.send(Out::Cancel);
        });
        *self.shown.borrow_mut() = Some(Rc::new(shown));
    }

    fn show_behind(&self, n: usize) {
        self.behind.set(n);
        let shown = self.shown.borrow().clone();
        if let Some(shown) = shown {
            shown.change(|a| a.behind = n);
        }
    }

    fn requester_gone(&self) -> bool {
        self.caller.is_some_and(process_gone)
    }

    fn respond(&self, text: &str) {

        let text = text.split('\n').next().unwrap_or("");
        let mut line = Zeroizing::new(Vec::with_capacity(text.len() + 1));
        line.extend_from_slice(text.as_bytes());
        line.push(b'\n');
        if let Some(input) = self.input.borrow_mut().as_mut() {
            let _ = input.write_all(&line);
        }
    }

    fn completed(self: &Rc<Self>, gained: bool) {
        if self.finished.get() {
            return;
        }
        if gained {
            self.finish(None, false);
        } else if self.tries.get() < TRIES {
            self.start(Some(t!("polkit-not-accepted")));
        } else {
            self.finish(Some("Authentication failed"), false);
        }
    }

    fn cancel(self: &Rc<Self>) {
        self.cancel_with(Some("Cancelled"), true);
    }

    fn cancel_with(self: &Rc<Self>, error: Option<&str>, cancelled: bool) {
        self.run.set(self.run.get() + 1);
        self.input.borrow_mut().take();
        if let Some(h) = self.helper.borrow_mut().take() {
            h.signal(libc::SIGTERM);
        }
        self.finish(error, cancelled);
    }

    fn finish(self: &Rc<Self>, error: Option<&str>, cancelled: bool) {
        if self.finished.replace(true) {
            return;
        }
        self.shown.borrow_mut().take();
        if let Some(asked) = self.asked.borrow_mut().take() {
            match error {
                None => asked.reply(&()),
                Some(e) => {
                    let name = if cancelled { CANCELLED } else { "Failed" };
                    asked.fail(&format!("org.freedesktop.PolicyKit1.Error.{name}"), e);
                }
            }
        }
        let typed = self.typed.borrow_mut().take().filter(|_| error.is_none());
        if let Some(a) = self.agent.upgrade() {
            a.requests.borrow_mut().remove(&self.cookie);
            a.left(&self.cookie, typed.map(|t| (self.user.clone(), t)));
        }
    }
}

struct Agent {
    ctx: Rc<Ctx>,
    requests: RefCell<HashMap<String, Rc<Request>>>,

    turns: RefCell<Turns<String>>,

    sweeping: Cell<bool>,

    served: RefCell<Option<Served>>,
}

#[derive(Default)]
struct Turns<K> {
    now: Option<K>,
    waiting: VecDeque<K>,
}

impl<K: PartialEq + Clone> Turns<K> {

    fn enter(&mut self, k: K) -> bool {
        if self.now.is_none() {
            self.now = Some(k);
            true
        } else {
            self.waiting.push_back(k);
            false
        }
    }

    fn leave(&mut self, k: &K) -> Option<K> {
        if self.now.as_ref() == Some(k) {
            self.now = self.waiting.pop_front();
            return self.now.clone();
        }
        self.waiting.retain(|w| w != k);
        None
    }

    fn behind(&self) -> usize {
        self.waiting.len()
    }

    fn all(&self) -> Vec<K> {
        self.now.iter().chain(self.waiting.iter()).cloned().collect()
    }
}

fn process_gone(pid: u32) -> bool {
    match std::fs::read_to_string(format!("/proc/{pid}/stat")) {
        Err(e) => e.kind() == ErrorKind::NotFound,

        Ok(stat) => stat.rsplit(')').next().and_then(|r| r.trim().chars().next()).is_none_or(|c| matches!(c, 'Z' | 'X')),
    }
}

const SWEEP: Duration = Duration::from_secs(3);

impl Agent {

    fn enter(self: &Rc<Self>, r: Rc<Request>) {
        let first = self.turns.borrow_mut().enter(r.cookie.clone());
        if first {
            r.begin(None);
        }
        self.tell_behind();
        self.sweep_soon();
    }

    fn left(self: &Rc<Self>, cookie: &str, accepted: Option<(String, Zeroizing<String>)>) {
        let next = self.turns.borrow_mut().leave(&cookie.to_string());
        if let Some(c) = next {
            let found = self.requests.borrow().get(&c).cloned();
            match found {

                Some(next) if next.requester_gone() => {
                    next.cancel();
                    return;
                }
                Some(next) => {
                    let reuse = accepted.as_ref().filter(|(user, _)| *user == next.user).map(|(_, p)| p.clone());
                    next.begin(reuse);
                }
                None => {
                    self.left(&c, accepted);
                    return;
                }
            }
        }
        self.tell_behind();
    }

    fn tell_behind(&self) {
        let (now, n) = {
            let t = self.turns.borrow();
            (t.now.clone(), t.behind())
        };
        if let Some(r) = now.and_then(|c| self.requests.borrow().get(&c).cloned()) {
            r.show_behind(n);
        }
    }

    fn drop_dead(self: &Rc<Self>) {
        let all = self.turns.borrow().all();
        for c in all {
            let r = self.requests.borrow().get(&c).cloned();
            if let Some(r) = r.filter(|r| r.requester_gone()) {
                r.cancel();
            }
        }
    }

    fn sweep_soon(self: &Rc<Self>) {
        if self.sweeping.replace(true) {
            return;
        }
        let a = self.clone();
        self.ctx.lp.after(SWEEP, move || {
            a.sweeping.set(false);
            a.drop_dead();
            if !a.requests.borrow().is_empty() {
                a.sweep_soon();
            }
        });
    }

    fn call(self: &Rc<Self>, asked: Asked) {
        match asked.member().as_str() {
            "BeginAuthentication" => {
                let body: Result<Begin, Failed> = asked.message().body().deserialize().map_err(Failed::from);
                let (action, message, _icon, details, cookie, identities) = match body {
                    Ok(b) => b,
                    Err(e) => {
                        asked.fail("org.freedesktop.DBus.Error.InvalidArgs", &e.to_string());
                        return;
                    }
                };
                let Some(uid) = pick_user(&identities) else {
                    asked.fail("org.freedesktop.PolicyKit1.Error.Failed", "No account this agent can ask for");
                    return;
                };
                let message = said_for(&action, &message, &details);
                let caller = details.get(CALLER).and_then(|p| p.parse().ok());
                let r = Request::new(self, asked, &action, &message, cookie.clone(), uid, caller);
                self.requests.borrow_mut().insert(cookie, r.clone());
                self.enter(r);
            }
            "CancelAuthentication" => {
                let cookie = asked.message().body().deserialize::<(String,)>().map(|c| c.0).unwrap_or_default();
                let r = self.requests.borrow().get(&cookie).cloned();
                if let Some(r) = r {
                    r.cancel();
                }
                asked.reply(&());
            }
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "No such method"),
        }
    }
}

fn register(agent: &Rc<Agent>) -> Result<(), String> {
    let bus = agent.ctx.system.clone().ok_or("the system bus is not available")?;
    let w = Rc::downgrade(agent);
    let served = bus.serve(AGENT_PATH, Offer {
        name: AGENT.into(),
        xml: AGENT_XML.into(),
        call: Box::new(move |asked| match w.upgrade() {
            Some(a) => a.call(asked),
            None => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "No such method"),
        }),
        props: Box::new(HashMap::new),
        set: None,
    }).map_err(|e| e.to_string())?;
    *agent.served.borrow_mut() = Some(served);
    let (b, lp) = (bus.clone(), agent.ctx.lp.clone());
    crate::common::drive_leave::session_id(&bus, &agent.ctx.lp, move |sid| {
        let sid = match sid {
            Ok(s) => s,
            Err(e) => {
                eprintln!("trindle-polkit: not in a logind session, so there is nothing to answer for: {e}");
                return;
            }
        };
        let mut details: HashMap<String, Value> = HashMap::new();
        details.insert("session-id".to_string(), Value::from(sid));
        let lang = std::env::var("LANG").unwrap_or_else(|_| "C".into());
        let (name, path, iface) = AUTHORITY;
        b.call(name, path, iface).method("RegisterAuthenticationAgent",
            &(("unix-session".to_string(), details), lang, AGENT_PATH.to_string()), false, Some(REGISTER_WAIT), &lp, |r| {
                if let Err(e) = r {
                    eprintln!("trindle-polkit: polkit would not take this agent: {e}");
                }
            });
    });
    Ok(())
}

pub fn start(ctx: &Rc<Ctx>) {
    let agent = Rc::new(Agent { ctx: ctx.clone(), requests: RefCell::new(HashMap::new()), turns: RefCell::default(), sweeping: Cell::new(false), served: RefCell::new(None) });
    if let Err(e) = register(&agent) {
        eprintln!("trindle-polkit: {e}");
    }

    std::mem::forget(agent);
}

#[cfg(test)]
mod tests {
    use super::{process_gone, unescape, Turns};

    #[test]
    fn the_first_request_asks_and_the_rest_wait_in_order() {
        let mut t = Turns::default();
        assert!(t.enter("a"));
        assert!(!t.enter("b"));
        assert!(!t.enter("c"));
        assert_eq!(t.behind(), 2);
        assert_eq!(t.all(), ["a", "b", "c"]);
    }

    #[test]
    fn when_the_asking_request_is_cancelled_the_next_asks_at_once() {
        let mut t = Turns::default();
        t.enter("stale");
        t.enter("update");
        assert_eq!(t.leave(&"stale"), Some("update"));
        assert_eq!(t.now, Some("update"));
        assert_eq!(t.behind(), 0);
        assert_eq!(t.leave(&"update"), None);
        assert_eq!(t.now, None);

        assert!(t.enter("later"));
    }

    #[test]
    fn a_waiting_request_cancelled_leaves_the_asking_one_alone() {
        let mut t = Turns::default();
        t.enter("a");
        t.enter("b");
        t.enter("c");
        assert_eq!(t.leave(&"b"), None);
        assert_eq!(t.now, Some("a"));
        assert_eq!(t.all(), ["a", "c"]);
        assert_eq!(t.leave(&"a"), Some("c"));

        assert_eq!(t.leave(&"b"), None);
        assert_eq!(t.leave(&"zzz"), None);
        assert_eq!(t.all(), ["c"]);
    }

    #[test]
    fn two_waiting_come_up_one_after_the_other_and_are_counted() {
        let mut t = Turns::default();
        for k in ["a", "b", "c"] {
            t.enter(k);
        }
        assert_eq!(t.behind(), 2);
        assert_eq!(t.leave(&"a"), Some("b"));
        assert_eq!(t.behind(), 1);
        assert_eq!(t.leave(&"b"), Some("c"));
        assert_eq!(t.behind(), 0);
        assert_eq!(t.leave(&"c"), None);
    }

    #[test]
    fn a_requester_that_has_died_is_found_and_a_live_one_is_not() {
        assert!(!process_gone(std::process::id()));
        let mut child = std::process::Command::new("true").spawn().expect("true runs");
        let pid = child.id();
        child.wait().expect("true ends");
        assert!(process_gone(pid), "a program that has ended is gone");

        let mut zombie = std::process::Command::new("true").spawn().expect("true runs");
        let zpid = zombie.id();
        for _ in 0..200 {
            if process_gone(zpid) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(process_gone(zpid));
        zombie.wait().ok();
    }

    #[test]
    fn the_waiting_words_are_in_the_language_file_for_one_and_for_many() {
        use crate::toolkit::t;
        let one = t!("polkit-waiting-behind", n = 1);
        let many = t!("polkit-waiting-behind", n = 3);
        assert!(one.contains("Another request"), "{one}");
        assert!(many.contains('3') && many.contains("requests"), "{many}");
        assert!(!one.contains('\u{2014}') && !many.contains('\u{2014}'));
    }

    #[test]
    fn the_question_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        use crate::toolkit::ui::{App, Ui};
        let events = crate::toolkit::event::Loop::new();
        drawn::none(drawn::in_every_look("the password question", 12, || {
            for (problem, waiting) in [(None, false), (Some("That password is not right. Try again."), false), (None, true)] {
                let prompt = super::Prompt {
                    message: "Authentication is needed to change the time of this computer.".into(),
                    note: "Password for the administrator".into(),
                    password: zeroize::Zeroizing::new("hunter2".into()),
                    problem: problem.map(String::from), waiting, turn: 0, behind: 2,
                    out: events.post(|_: super::Out| {}),
                };
                let mut ui = Ui::new(prompt.view(), super::look());
                let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
            }
        }));
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_password_window() {
        use crate::toolkit::ui::{App, Ui};
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let events = crate::toolkit::event::Loop::new();
        let time = "Authentication is needed to change the time of this computer.";
        let updates = crate::common::updating::admin_reason("apt-upgrade").unwrap_or_default();
        for (name, message, problem, waiting) in [("polkit", time, None, false), ("polkit-wrong", time, Some("That password is not right. Try again."), false),
                                                  ("polkit-waiting", time, None, true), ("polkit-updates", updates, None, false)] {
            let prompt = super::Prompt {
                message: message.into(),
                note: "Password for the administrator".into(),
                password: zeroize::Zeroizing::new("hunter2".into()),
                problem: problem.map(String::from), waiting, turn: 0, behind: 1,
                out: events.post(|_: super::Out| {}),
            };
            for (w, h) in [(1280u32, 800u32), (1920, 1080)] {
                let mut ui = Ui::new(prompt.view(), super::look());
                let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                pix.save_png(std::path::Path::new(&dir).join(format!("{name}-{w}.png"))).unwrap();
            }
        }
    }

    #[test]
    fn the_question_is_painted_in_trindles_own_look() {
        use crate::common::style::{Layers, CSS, DEFAULT_ACCENT};
        use crate::toolkit::theme::{contrast, painted, Ink, WORDS};
        use crate::toolkit::ui::{App, Ui};
        let look = super::look();
        let own = Layers::new(CSS, "");

        assert!(painted(&own, look.light, DEFAULT_ACCENT).iter().all(|p| !p.short()));
        assert!(painted(&own, look.light, DEFAULT_ACCENT).iter().any(|p| p.ink == Ink::Red && p.colour == look.danger_text));
        let events = crate::toolkit::event::Loop::new();
        let prompt = super::Prompt {
            message: "Authentication is needed to change the time of this computer.".into(),
            note: "Password for the administrator".into(),
            password: zeroize::Zeroizing::new(String::new()),
            problem: Some("That password is not right. Try again.".into()), waiting: false, turn: 0, behind: 2,
            out: events.post(|_: super::Out| {}),
        };
        let mut ui = Ui::new(prompt.view(), look.clone());
        let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);

        let is = |x: u32, y: u32, c: crate::toolkit::colour::Colour| {
            let p = pix.pixel(x, y).unwrap();
            (p.red(), p.green(), p.blue()) == ((c.r * 255.0).round() as u8, (c.g * 255.0).round() as u8, (c.b * 255.0).round() as u8)
        };

        let count = |c: crate::toolkit::colour::Colour| (0..1280u32).flat_map(|x| (0..800u32).map(move |y| (x, y))).filter(|(x, y)| is(*x, *y, c)).count();
        assert!(count(look.raised) > 20_000, "the card is the built-in sheet's: {}", count(look.raised));
        assert!(count(look.field) > 2_000, "the password box is the built-in sheet's: {}", count(look.field));
        assert!(contrast(look.danger_text, look.raised) >= WORDS && contrast(look.fg, look.raised) >= WORDS && contrast(look.fg, look.field) >= WORDS);
        let red = count(look.danger_text);
        assert!(red > 20, "the red line is drawn in the built-in red: {red}");
    }

    #[test]
    fn a_closed_box_is_cancelled_in_polkit_s_own_word_and_an_update_says_why() {
        use crate::common::run::{admin_error, admin_step, NO_PASSWORD};
        use crate::common::updating::admin_reason;
        use std::collections::HashMap;
        assert_eq!(format!("org.freedesktop.PolicyKit1.Error.{}", super::CANCELLED), "org.freedesktop.PolicyKit1.Error.Cancelled");
        let source = crate::common::source::code(include_str!("mod.rs"));
        let one_l = ["Cancel", "ed\""].concat();
        assert!(!source.contains(&one_l), "polkit does not know the name with one l");

        assert_eq!(admin_error("Error executing command as another user: Request dismissed"), NO_PASSWORD);
        assert_ne!(admin_error("Error executing command as another user: Not authorized"), NO_PASSWORD);

        assert_eq!(admin_step(b"pkexec\0/usr/lib/trindle/libexec/trindle-admin\0apt-upgrade\0").as_deref(), Some("apt-upgrade"));
        assert_eq!(admin_step(b"/usr/bin/pkexec\0/usr/lib/trindle/libexec/trindle-admin\0firmware-update\0abc\0").as_deref(), Some("firmware-update"));
        assert_eq!(admin_step(b"pkexec\0/usr/bin/env\0apt-upgrade\0"), None);
        assert_eq!(admin_step(b"sh\0/usr/lib/trindle/libexec/trindle-admin\0apt-upgrade\0"), None);
        assert_eq!(admin_step(b"pkexec\0/usr/lib/trindle/libexec/trindle-admin\0"), None);

        for step in ["apt-upgrade", "apt-upgrade-debian", "apt-upgrade-trindle", "firmware-update", "apt-refresh"] {
            let why = admin_reason(step).unwrap_or_else(|| panic!("{step} says why"));
            assert!(why.contains("administrator's password") && why.ends_with('.') && !why.contains('\u{2014}'), "{why}");
        }
        assert_eq!(admin_reason("hostname"), None);

        let none = HashMap::new();
        assert_eq!(super::said_for("org.example.other", "Example words.", &none), "Example words.");
        assert_eq!(super::said_for(super::ADMIN_ACTION, "Example words.", &none), "Example words.");

        let drive = HashMap::from([("device".to_string(), "/dev/sdb".to_string())]);
        let words = super::said_for("org.freedesktop.udisks2.modify-device", "Authentication is required to format Example Disk (/dev/sdb)", &drive);
        assert_eq!(words, format!("An administrator's password is needed to format Example Disk (/dev/sdb). {}", crate::common::drive_leave::EVERY_TIME));
        let me = HashMap::from([(super::CALLER.to_string(), std::process::id().to_string())]);
        assert_eq!(super::said_for(super::ADMIN_ACTION, "Example words.", &me), "Example words.", "this test is not pkexec");
    }

    #[test]
    fn helper_lines_are_unescaped() {
        assert_eq!(unescape("Password: "), "Password: ");
        assert_eq!(unescape(r#"a\tb\nc\\d\"e"#), "a\tb\nc\\d\"e");
        assert_eq!(unescape(r"caf\303\251"), "café");
        assert_eq!(unescape(r#"\101\0end\"#), "A\0end\\");
    }
}
