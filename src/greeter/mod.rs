//! trindle-greeter: the greetd login screen, shown by trindle-compose --greeter.

mod words;
mod account;
mod face;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::Duration;

use account::{drawing_file, reader_file, remember, remembered_user, session_cmd};
use greetd::{Backend, Greetd};
use image::DynamicImage;
use serde_json::{json, Value};
use words::{HINT, no_chip, no_match, pam_words, phone_hint, phone_no, ready_name, ready_phone, ready_plain, trouble, welcome_back, welcome_new};

use crate::common::{faces, pocketkey};
use crate::common::files::cache_home;
use crate::common::ui::DISARM_SECONDS;
use crate::toolkit::t;
use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::icon::Icons;
use crate::toolkit::text::Style;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, column, drawing, entry, heading, label, lead, Length, Node, note, row, space, switch, Tone};
use crate::toolkit::window::{Display, Kind};

pub mod greetd;

const PANEL: f32 = 400.0;

const CLOCK: f32 = 64.0;

const PROBLEM_ROOM: f32 = 22.0;

const FOOT_BUTTON: f32 = 112.0;

const KEYBOARD: &str = "wvkbd-mobintl";

#[derive(Debug, PartialEq)]
enum Answer {

    Send(Option<String>),

    Ask,

    PhoneFailed,
}

fn answer(what: &str, text: &str, by_phone: bool, secret: &mut Option<String>) -> Answer {
    match what {
        "visible" | "secret" if text.as_bytes() == pocketkey::QUESTION.to_bytes() => {
            Answer::Send(Some(if by_phone { "yes" } else { "no" }.into()))
        }
        "secret" if by_phone => Answer::PhoneFailed,
        "secret" => match secret.take() {
            Some(s) => Answer::Send(Some(s)),
            None => Answer::Ask,
        },
        "info" | "error" => Answer::Send(None),
        _ => Answer::Ask,
    }
}

fn phone_for(name: &str) -> (bool, Option<u32>) {
    match pocketkey::on_for(name) {
        Some(true) => (true, None),

        Some(false) => (false, pocketkey::due_for(name)),
        None => (false, None),
    }
}

fn plain_error(reply: &Value) -> String {
    let description = reply["description"].as_str().unwrap_or("");
    if let Some(words) = pam_words(description) {
        return words.to_string();
    }
    if reply["error_type"] == "auth_error" {
        return no_match().into();
    }
    if description.is_empty() { t!("greeter-login-failed").into() } else { description.into() }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Step {

    Reply,

    Retry,

    Started,

    Cancelled,
}

#[derive(Clone)]
enum Msg {
    Name(String),
    Pass(String),

    NameDone,

    Submit,
    Phone,
    Back(Step, Option<Value>, Option<String>),
    Tick,
    HintGone(u64),
    Light(bool),
    Power(&'static str),
    Disarm,
    Said(String),

    Keyboard,

    Reader,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Under {
    Name,
    Password,
}

#[derive(Default)]
struct Helpers {
    keyboard: Option<std::process::Child>,
}

impl Helpers {
    fn running(child: &mut Option<std::process::Child>) -> bool {
        child.as_mut().is_some_and(|c| matches!(c.try_wait(), Ok(None)))
    }

    fn start(program: &str, args: &[&str]) -> Option<std::process::Child> {
        use std::process::{Command, Stdio};
        Command::new(program).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().ok()
    }

    fn end(child: &mut Option<std::process::Child>) {
        if let Some(mut c) = child.take() {
            if let Ok(pid) = libc::pid_t::try_from(c.id()) {
                // SAFETY: kill only sends a signal, to a child this program

                unsafe { libc::kill(pid, libc::SIGTERM) };
            }
            std::thread::spawn(move || c.wait());
        }
    }
}

impl Drop for Helpers {
    fn drop(&mut self) {
        Helpers::end(&mut self.keyboard);
    }
}

impl Drop for Greeter {
    fn drop(&mut self) {
        self.end_reader();
    }
}

struct Greeter {
    backend: Box<dyn Backend>,
    events: Loop,
    me: Post<Msg>,

    bus: Option<Bus>,
    icons: Rc<Icons>,
    user: String,

    known: bool,
    password: String,

    placeholder: String,

    headline: String,

    problem: Option<(String, Under)>,
    busy: bool,

    phone: bool,

    due: Option<u32>,

    phone_off: bool,

    hint: bool,

    hint_shown: u64,

    by_phone: bool,
    pending_question: bool,
    retried: bool,
    started: bool,
    secret: Option<String>,

    light: Option<bool>,
    face: Option<Rc<DynamicImage>>,

    now: (String, String),

    turn: u32,
    grab_name: u32,
    grab_pass: u32,

    armed: Option<&'static str>,

    keyboard: bool,
    keyboard_out: bool,

    reader_on: bool,
    helpers: Helpers,
}

impl Greeter {
    fn new(backend: Box<dyn Backend>, events: Loop, me: Post<Msg>, user: String) -> Greeter {
        let (phone, due) = phone_for(&user);
        let light = (!crate::common::graphics::has_graphics_chip())
            .then(|| std::fs::read_to_string(drawing_file()).map(|t| t.trim() != "normal").unwrap_or(true));
        let known = !user.trim().is_empty();
        let mut g = Greeter {
            backend, events, me, bus: None, icons: Rc::new(Icons::new()),
            user, known, password: String::new(), placeholder: t!("greeter-password-box").into(),
            headline: String::new(), problem: None, busy: false,
            phone, due, phone_off: false, hint: false, hint_shown: 0,
            by_phone: false, pending_question: false, retried: false, started: false, secret: None,
            light, face: None, now: (String::new(), String::new()),
            turn: 0, grab_name: 0, grab_pass: 0, armed: None,
            keyboard: crate::common::run::have(KEYBOARD), keyboard_out: false,
            reader_on: false,
            helpers: Helpers::default(),
        };
        g.say("");
        g.show_face();
        g.tick();
        if g.user.trim().is_empty() { g.focus_name() } else { g.focus_pass() }
        if g.phone {
            g.show_hint();
        }
        if crate::common::screen_reader::installed() && std::fs::read_to_string(reader_file()).is_ok_and(|t| t.trim() == "on") {
            g.set_reader(true);
        }
        g
    }

    fn focus_name(&mut self) {
        self.turn += 1;
        self.grab_name = self.turn;
    }

    fn focus_pass(&mut self) {
        self.turn += 1;
        self.grab_pass = self.turn;
    }

    fn show_face(&mut self) {
        self.face = faces::path_for(self.user.trim())
            .and_then(|p| crate::toolkit::picture::load(&p)).map(Rc::new);
    }

    fn tick(&mut self) {
        let now = crate::common::clock::Local::now();
        self.now = (now.line(crate::toolkit::lang::format::Pattern::Clock), now.line(crate::toolkit::lang::format::Pattern::DayMonth));
    }

    fn show_hint(&mut self) {
        self.hint = true;
        self.hint_shown += 1;
        let (me, n) = (self.me.clone(), self.hint_shown);
        self.events.after(HINT, move || {
            me.send(Msg::HintGone(n));
        });
    }

    fn welcome(&self) -> &'static str {
        if self.known { welcome_back() } else { welcome_new() }
    }

    fn say(&mut self, text: &str) {
        self.headline = self.welcome().into();
        self.problem = (!text.is_empty()).then(|| (text.to_string(), Under::Password));
    }

    fn news(&mut self, text: &str) {
        self.headline = text.into();
        self.problem = None;
    }

    fn ready(&self) -> &'static str {
        if self.user.trim().is_empty() {
            ready_name()
        } else if self.phone {
            ready_phone()
        } else {
            ready_plain()
        }
    }

    fn send(&self, request: Value, step: Step) {
        let me = self.me.clone();
        self.backend.send(request, Box::new(move |reply, err| {
            me.send(Msg::Back(step, reply, err));
        }));
    }

    fn refresh_phone(&mut self) {
        if self.busy {
            return;
        }
        let was = self.phone;
        (self.phone, self.due) = phone_for(&self.user);
        if self.phone && !was {
            self.show_hint();
        }
    }

    fn submit(&mut self) {
        self.refresh_phone();
        let by_phone = self.phone && !self.pending_question && self.password.is_empty();
        self.start(by_phone);
    }

    fn start(&mut self, by_phone: bool) {
        if self.busy {
            return;
        }
        if self.pending_question {
            self.pending_question = false;
            self.busy = true;
            self.send(json!({"type": "post_auth_message_response", "response": self.password}), Step::Reply);
            self.password.clear();
            return;
        }
        let name = self.user.trim().to_string();
        if name.is_empty() {
            self.headline = self.welcome().into();
            self.problem = Some((t!("greeter-name-first").into(), Under::Name));
            self.focus_name();
            return;
        }
        self.news(if by_phone { t!("greeter-asking-phone") } else { t!("greeter-logging-in") });
        self.busy = true;
        self.retried = false;
        self.by_phone = by_phone;
        self.secret = if by_phone { None } else { Some(self.password.clone()) };
        if by_phone {
            self.password.clear();
        }
        self.send(json!({"type": "create_session", "username": name}), Step::Reply);
    }

    fn reply(&mut self, reply: Option<Value>, err: Option<String>) {
        let reply = match (reply, err) {
            (_, Some(e)) => return self.fail(&e),
            (Some(r), None) => r,
            (None, None) => return self.fail(t!("greeter-login-failed")),
        };
        match reply["type"].as_str().unwrap_or("") {
            "auth_message" => self.auth_message(&reply),
            "success" => self.session_made(),
            "error" => self.session_refused(&reply),
            _ => {}
        }
    }

    fn auth_message(&mut self, reply: &Value) {
        let what = reply["auth_message_type"].as_str().unwrap_or("");
        let text = reply["auth_message"].as_str().unwrap_or("").to_string();
        match answer(what, &text, self.by_phone, &mut self.secret) {
            Answer::Send(Some(response)) => {
                self.send(json!({"type": "post_auth_message_response", "response": response}), Step::Reply);
            }
            Answer::Send(None) => {
                if what == "error" {
                    self.problem = Some((text, Under::Password));
                } else {
                    self.news(&text);
                }
                self.send(json!({"type": "post_auth_message_response"}), Step::Reply);
            }
            Answer::PhoneFailed => self.fail(phone_no()),
            Answer::Ask => {

                self.busy = false;
                self.password.clear();
                let hint = text.trim();
                self.placeholder = if hint.is_empty() { t!("greeter-answer-box").into() } else { hint.into() };
                self.news(&text);
                self.focus_pass();
                self.pending_question = true;
                self.phone_off = true;
            }
        }
    }

    fn session_made(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        remember(self.user.trim());

        let env: Vec<String> = std::iter::once("XDG_SESSION_TYPE=wayland".to_owned())
            .chain(self.light.iter()
                .map(|on| format!("{}={}", crate::common::graphics::DRAWING,
                                  if *on { "light" } else { "normal" })))
            .collect();
        self.send(json!({"type": "start_session", "cmd": [session_cmd()], "env": env}), Step::Started);
    }

    fn session_refused(&mut self, reply: &Value) {
        if reply["description"].as_str().unwrap_or("").contains("already") && !self.retried {

            self.retried = true;
            self.send(json!({"type": "cancel_session"}), Step::Retry);
            return;
        }
        self.retried = false;
        self.fail(&plain_error(reply));
    }

    fn started(&mut self, reply: Option<Value>, err: Option<String>) {
        let problem = match (&reply, err) {
            (_, Some(e)) => Some(e),
            (Some(r), None) if r["type"] == "error" => Some(plain_error(r)),
            (None, None) => Some(t!("greeter-login-failed").into()),
            _ => None,
        };
        match problem {
            Some(p) => {
                self.started = false;
                self.fail(&p);
            }
            None => {

                Helpers::end(&mut self.helpers.keyboard);
                self.end_reader();
                self.events.quit();
            }
        }
    }

    fn fail(&mut self, text: &str) {
        self.pending_question = false;
        self.by_phone = false;
        self.secret = None;

        self.send(json!({"type": "cancel_session"}), Step::Cancelled);
        self.busy = false;
        self.phone_off = false;
        self.password.clear();
        self.placeholder = t!("greeter-password-box").into();
        self.say(text);
        self.focus_pass();
    }

    fn power(&mut self, method: &'static str) {
        if self.armed != Some(method) {
            self.armed = Some(method);
            let me = self.me.clone();
            self.events.after(Duration::from_secs(DISARM_SECONDS.into()), move || {
                me.send(Msg::Disarm);
            });
            return;
        }
        self.armed = None;
        if self.bus.is_none() {
            self.bus = Bus::system(&self.events).ok();
        }
        let Some(bus) = &self.bus else {
            return self.say(t!("greeter-no-bus"));
        };
        let me = self.me.clone();
        bus.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
            .method(method, &(false,), false, Some(Duration::from_secs(10)), &self.events, move |res| {
                if let Err(e) = res {
                    let why = e.to_string();
                    let said = if method == "Reboot" { t!("greeter-restart-failed", why = why) } else { t!("greeter-shut-down-failed", why = why) };
                    me.send(Msg::Said(said));
                }
            });
    }

    fn keyboard(&mut self) {
        if !Helpers::running(&mut self.helpers.keyboard) {
            let font = format!("{} 14", crate::toolkit::text::FAMILY);
            self.helpers.keyboard = Helpers::start(KEYBOARD, &["-L", "260", "--fn", &font]);
            self.keyboard_out = self.helpers.keyboard.is_some();
            return;
        }
        if let Some(pid) = self.helpers.keyboard.as_ref().and_then(|c| libc::pid_t::try_from(c.id()).ok()) {
            // SAFETY: kill only sends a signal, to a child this program

            unsafe { libc::kill(pid, libc::SIGRTMIN()) };
            self.keyboard_out = !self.keyboard_out;
        }
    }

    fn set_reader(&mut self, on: bool) {
        use crate::common::screen_reader as reader;
        if !reader::installed() {
            self.say(reader::NONE_YET);
            return;
        }
        if on && !reader::running() {
            reader::start_for_login();
        } else if !on {
            reader::stop();
        }
        self.reader_on = on && reader::running();
        let _ = std::fs::create_dir_all(cache_home());
        let _ = crate::common::files::write_text(&reader_file(), if self.reader_on { "on" } else { "off" });
    }

    fn end_reader(&mut self) {
        if std::mem::take(&mut self.reader_on) {
            crate::common::screen_reader::stop();
        }
    }
}

impl App for Greeter {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Name(t) => {
                if !self.busy {
                    self.user = t;
                    if self.problem.as_ref().is_some_and(|(_, at)| *at == Under::Name) {
                        self.problem = None;
                    }
                    self.show_face();
                }
            }
            Msg::Pass(t) => {
                if !self.busy {
                    self.password = t;
                }
            }
            Msg::NameDone => {
                self.refresh_phone();
                self.focus_pass();
            }
            Msg::Submit => self.submit(),
            Msg::Phone => {
                if !self.phone_off {
                    self.start(true);
                }
            }
            Msg::Back(Step::Reply, r, e) => self.reply(r, e),
            Msg::Back(Step::Retry, _, _) => {
                let name = self.user.trim().to_string();
                self.send(json!({"type": "create_session", "username": name}), Step::Reply);
            }
            Msg::Back(Step::Started, r, e) => self.started(r, e),
            Msg::Back(Step::Cancelled, _, _) => self.backend.reset(),
            Msg::Tick => self.tick(),
            Msg::HintGone(n) => {
                if n == self.hint_shown {
                    self.hint = false;
                }
            }
            Msg::Light(on) => {
                self.light = Some(on);
                let _ = std::fs::create_dir_all(cache_home());
                let _ = crate::common::files::write_text(&drawing_file(), if on { "light" } else { "normal" });
            }
            Msg::Power(method) => self.power(method),
            Msg::Disarm => self.armed = None,
            Msg::Said(t) => self.say(&t),
            Msg::Keyboard => self.keyboard(),
            Msg::Reader => {
                let on = !self.reader_on;
                self.set_reader(on);
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let (time, date) = self.now.clone();
        column(vec![
            space().height(Length::Fill),
            self.clock(time),
            middle(heading(date)),
            space().height(Length::Fixed(8.0)),
            self.panel(),
            space().height(Length::Fill),
            self.foot(),
        ]).centred().gap(8.0).padding(crate::toolkit::theme::sheet().page_padding)
    }
}

impl Greeter {
    fn clock(&self, time: String) -> Node<Msg> {
        drawing((PANEL, CLOCK * 1.15), time.clone(), move |canvas, pen| {
            let style = Style {
                family: Some(crate::toolkit::text::FAMILY.into()), weight: 700,
                spacing: -crate::toolkit::theme::sheet().tighten * CLOCK * pen.scale, ..Style::new(CLOCK * pen.scale)
            };
            let layout = pen.fonts.layout(&time, &style, None);
            let x = ((canvas.width() as f32 - layout.width) / 2.0).round();
            let y = ((canvas.height() as f32 - layout.height) / 2.0).round();
            pen.fonts.draw(canvas, &layout, x, y, pen.theme.fg);
        })
    }

    fn panel(&self) -> Node<Msg> {
        let mut items = vec![self.picture()];
        items.push(middle(lead(self.headline.clone())));
        items.push(entry(self.user.clone(), t!("greeter-name-box"), Msg::Name).key("name")
            .grab(self.grab_name).on_submit(Msg::NameDone).width(Length::Fill));
        if let Some((text, Under::Name)) = &self.problem {
            items.push(label(text.clone()).tone(Tone::Problem).width(Length::Fill));
        }

        let mut pass_row = vec![entry(self.password.clone(), self.placeholder.clone(), Msg::Pass).key("pass")
            .secret().guarded().grab(self.grab_pass).on_submit(Msg::Submit).width(Length::Fill)];
        if self.phone {
            let phone = button("phone-symbolic", t!("greeter-use-phone"));
            pass_row.push(if self.busy || self.phone_off { phone } else { phone.on_press(Msg::Phone) });
        }
        pass_row.push(self.sign_in());
        items.push(row(pass_row).gap(12.0));

        let under = match (&self.problem, self.due) {
            (Some((text, Under::Password)), _) => label(text.clone()).tone(Tone::Problem).width(Length::Fill),
            (_, Some(secs)) => note(pocketkey::due_words(secs)).width(Length::Fill),
            _ if self.hint => note(phone_hint()).width(Length::Fill),
            _ => note(self.ready()).width(Length::Fill),
        };
        items.push(column(vec![under]).height(Length::Fixed(PROBLEM_ROOM)));

        card(column(items).gap(12.0)).width(Length::Fixed(PANEL))
    }

    fn foot(&self) -> Node<Msg> {
        let mut buttons = Vec::new();

        if let Some(on) = self.light {
            buttons.push(labelled(switch(on, t!("greeter-light-drawing"), Msg::Light), t!("greeter-light-drawing")));
        }
        if self.keyboard {
            buttons.push(labelled(button("input-keyboard-symbolic", t!("greeter-keyboard-tip")).on(self.keyboard_out)
                .on_press(Msg::Keyboard), t!("greeter-keyboard")));
        }
        buttons.push(labelled(button("preferences-desktop-accessibility-symbolic", t!("greeter-screen-reader")).on(self.reader_on)
            .on_press(Msg::Reader), t!("greeter-screen-reader")));

        let power = |icon: &str, tip: &str, sure: &str, method: &'static str| {
            let armed = self.armed == Some(method);
            let b = button(icon, if armed { sure } else { tip }).on_press(Msg::Power(method));
            labelled(if armed { b.filled() } else { b }, if armed { t!("greeter-press-again") } else { tip })
        };
        buttons.push(power("system-reboot-symbolic", t!("greeter-restart"), t!("greeter-restart-sure"), "Reboot"));
        buttons.push(power("system-shutdown-symbolic", t!("greeter-shut-down"), t!("greeter-shut-down-sure"), "PowerOff"));
        let mut said = vec![note(trouble())];
        if self.light.is_some() {
            said.push(note(no_chip()));
        }
        let mut foot = vec![column(said).gap(4.0).width(Length::Fill)];
        foot.extend(buttons);
        row(foot).gap(4.0)
    }

    fn sign_in(&self) -> Node<Msg> {

        crate::toolkit::widget::main_action("go-next-symbolic", t!("greeter-sign-in")).on_press(Msg::Submit).off(self.busy)
    }
}

fn middle(item: Node<Msg>) -> Node<Msg> {
    row(vec![space(), item, space()])
}

fn labelled(button: Node<Msg>, words: &str) -> Node<Msg> {

    let held = row(vec![space(), button, space()]).height(Length::Fixed(crate::toolkit::theme::icon_button()));
    column(vec![held, note(words)]).centred().gap(4.0).width(Length::Fixed(FOOT_BUTTON))
}

pub fn main(_args: Vec<String>) -> i32 {
    if std::env::var_os("GREETD_SOCK").is_none() {
        eprintln!("trindle-greeter: GREETD_SOCK is not set. It is started by greetd.");
        return 1;
    }

    crate::common::sheet::no_themes();
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };

    let slot: Rc<RefCell<Weak<Shown<Greeter>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let me = events.post(move |m: Msg| {
        let Some(shown) = s.borrow().upgrade() else { return };

        if matches!(m, Msg::Tick) {
            return shown.change_if(|g| {
                let was = g.now.clone();
                g.tick();
                g.now != was
            });
        }
        shown.change(|g| g.update(m));
    });
    let app = Greeter::new(Box::new(Greetd::new(&events)), events.clone(), me.clone(), remembered_user());

    let kind = Kind::Window { title: t!("greeter-window-title").into(), app_id: "org.trindle.Greeter".into() };
    let Some(shown) = Shown::open(&events, &display, kind, (1024, 700), app) else { return 1 };
    let shown = Rc::new(shown);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    events.every(Duration::from_secs(1), move || {
        if me.send(Msg::Tick) { Flow::Continue } else { Flow::Stop }
    });
    events.run();
    0
}
