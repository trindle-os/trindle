//! The PIN card.

use std::cell::RefCell;
use std::io::{BufRead, Read, Write};
use std::process::{Command, Stdio};
use std::rc::{Rc, Weak};
use std::time::Duration;

use zeroize::Zeroize;

use crate::common::person_only::PIN_CARD;
use crate::toolkit::t;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::tiles::tile_head;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, column, drawing, label, Length, main_word, Node, note, row, space, Tone, word_button};
use crate::toolkit::window::{Display, Kind, persons_own};

pub const LENGTH: usize = 4;
pub fn four() -> &'static str {
    t!("plugins-pin-four")
}
pub fn not_saved() -> &'static str {
    t!("plugins-pin-not-saved")
}
pub fn no_answer() -> &'static str {
    t!("plugins-pin-no-answer")
}
pub fn not_unlocked() -> &'static str {
    t!("plugins-pin-not-unlocked")
}
const LINE_MAX: u64 = 512;
const LINE_SHOWN: usize = 80;
const ANSWER_WAIT: Duration = Duration::from_secs(30);
const SIZE: (u32, u32) = (340, 520);
const SAID_ROOM: f32 = 22.0;

pub struct Digits {
    d: [u8; LENGTH],
    n: usize,
}

impl Digits {
    pub const fn new() -> Digits {
        Digits { d: [0; LENGTH], n: 0 }
    }

    pub fn push(&mut self, c: u8) -> bool {
        if self.n < LENGTH && c.is_ascii_digit() {
            self.d[self.n] = c;
            self.n += 1;
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) {
        if self.n > 0 {
            self.n -= 1;
            self.d[self.n] = 0;
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn full(&self) -> bool {
        self.n == LENGTH
    }

    pub fn wipe(&mut self) {
        self.d.zeroize();
        self.n = 0;
    }

    pub fn hand_over(&mut self, out: &mut impl Write) -> Result<(), &'static str> {
        if !self.full() {
            self.wipe();
            return Err(four());
        }
        let sent = out.write_all(&self.d).and_then(|()| out.write_all(b"\n")).and_then(|()| out.flush());
        self.wipe();
        sent.map_err(|_| no_answer())
    }

    #[cfg(test)]
    pub fn raw(&self) -> [u8; LENGTH] {
        self.d
    }
}

impl Drop for Digits {
    fn drop(&mut self) {
        self.wipe();
    }
}

impl std::fmt::Debug for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Digits({} of {LENGTH})", self.n)
    }
}

pub type Answer = Result<(), String>;

pub fn answer_of(code: Option<i32>, line: &str) -> Answer {
    let said = crate::plugins::notes::plain_words(line.trim(), LINE_SHOWN);
    match code {
        Some(0) => Ok(()),
        Some(1) if !said.is_empty() => Err(said),
        Some(1) => Err(not_unlocked().into()),
        _ if !said.is_empty() => Err(said),
        _ => Err(no_answer().into()),
    }
}

pub fn start(mut command: Command, digits: &mut Digits, done: impl FnOnce(Answer) + Send + 'static) {
    use std::os::unix::process::CommandExt;
    command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).process_group(0);
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => {
            digits.wipe();
            return done(Err(no_answer().into()));
        }
    };
    let pid = child.id() as libc::pid_t;
    let mut input = child.stdin.take();
    let sent = match input.as_mut() {
        Some(pipe) => digits.hand_over(pipe),
        None => {
            digits.wipe();
            Err(no_answer())
        }
    };
    drop(input);
    if let Err(why) = sent {
        end(pid);
        let _ = child.wait();
        return done(Err(why.into()));
    }
    let output = child.stdout.take();
    let over = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let watch = over.clone();
    std::thread::spawn(move || {
        std::thread::sleep(ANSWER_WAIT);
        if !watch.load(std::sync::atomic::Ordering::SeqCst) {
            end(pid);
        }
    });
    std::thread::spawn(move || {
        let mut line = String::new();
        if let Some(output) = output {
            let _ = std::io::BufReader::new(output.take(LINE_MAX)).read_line(&mut line);
        }
        let code = child.wait().ok().and_then(|s| s.code());
        over.store(true, std::sync::atomic::Ordering::SeqCst);
        done(answer_of(code, &line));
    });
}

fn end(pid: libc::pid_t) {
    // SAFETY: a signal to the group the command was started as the leader of.
    unsafe {
        libc::kill(-pid, libc::SIGKILL);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Digit(u8),
    Other,
    Back,
    Send,
    Cancel,
    Answer(Answer),
}

enum Out {
    Close,
}

pub struct Card {
    digits: Digits,
    said: Option<String>,
    busy: bool,
    turn: u32,
    launch: Box<dyn Fn(&mut Digits)>,
    out: Post<Out>,
}

impl Card {
    fn new(launch: Box<dyn Fn(&mut Digits)>, out: Post<Out>) -> Card {
        Card { digits: Digits::new(), said: None, busy: false, turn: 0, launch, out }
    }

    #[cfg(test)]
    pub fn held(&self) -> usize {
        self.digits.len()
    }
}

pub fn key_of(input: &Input) -> Option<Msg> {
    let Input::Key { sym, text, mods, .. } = input else { return None };
    if mods.ctrl || mods.alt || mods.logo {
        return None;
    }
    match *sym {
        Keysym::BackSpace => return Some(Msg::Back),
        Keysym::Return | Keysym::KP_Enter => return Some(Msg::Send),
        _ => {}
    }
    let text = text.as_deref().filter(|t| !t.is_empty() && !t.chars().any(char::is_control))?;
    match text.as_bytes() {
        [c] if c.is_ascii_digit() => Some(Msg::Digit(*c)),
        _ => Some(Msg::Other),
    }
}

impl App for Card {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Digit(_) | Msg::Other | Msg::Back | Msg::Send if !persons_own(PIN_CARD) => {}
            Msg::Digit(c) => {
                if !self.busy && self.digits.push(c) {
                    self.said = None;
                }
            }
            Msg::Other => {
                if !self.busy {
                    self.said = Some(four().into());
                }
            }
            Msg::Back => {
                if !self.busy {
                    self.digits.pop();
                    self.said = None;
                }
            }
            Msg::Send => {
                if self.busy {
                    return;
                }
                if !self.digits.full() {
                    self.said = Some(four().into());
                    return;
                }
                self.busy = true;
                self.said = None;
                (self.launch)(&mut self.digits);
            }
            Msg::Cancel => {
                self.digits.wipe();
                self.out.send(Out::Close);
            }
            Msg::Answer(Ok(())) => {
                self.digits.wipe();
                self.out.send(Out::Close);
            }
            Msg::Answer(Err(why)) => {
                self.digits.wipe();
                self.busy = false;
                self.turn += 1;
                self.said = Some(why);
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let held = self.digits.len();
        let dots = drawing((4.0 * 40.0, 28.0), t!("plugins-pin-dots"), move |canvas, pen| {
            let s = pen.scale;
            let mut fill = tiny_skia::Paint::default();
            fill.set_color(pen.theme.fg.to_skia());
            fill.anti_alias = true;
            let mut edge = tiny_skia::Paint::default();
            edge.set_color(pen.theme.muted.to_skia());
            edge.anti_alias = true;
            let line = tiny_skia::Stroke { width: 2.0 * s, ..tiny_skia::Stroke::default() };
            for i in 0..LENGTH {
                let (cx, cy) = ((20.0 + 40.0 * i as f32) * s, 14.0 * s);
                let Some(circle) = tiny_skia::PathBuilder::from_circle(cx, cy, 8.0 * s) else { continue };
                if i < held {
                    canvas.fill_path(&circle, &fill, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
                } else {
                    canvas.stroke_path(&circle, &edge, &line, tiny_skia::Transform::identity(), None);
                }
            }
        }).on_keys(key_of).key("pin").first().grab(self.turn);
        let said = match &self.said {
            Some(words) => label(words.clone()).tone(Tone::Problem).width(Length::Fill),
            None => space(),
        };
        let said = column(vec![said]).height(Length::Fixed(SAID_ROOM));
        let digit = |c: u8| word_button((c as char).to_string()).on_press(Msg::Digit(c));
        let pad = |a: u8| row(vec![digit(a), digit(a + 1), digit(a + 2)]).gap(8.0);
        let last = row(vec![button("edit-clear-symbolic", t!("plugins-pin-delete")).on_press(Msg::Back), digit(b'0')]).gap(8.0);
        let send = if self.busy { main_word(t!("plugins-pin-send")).off(true) } else { main_word(t!("plugins-pin-send")).on_press(Msg::Send) };
        let buttons = row(vec![space(), button("window-close-symbolic", t!("plugins-pin-cancel")).on_press(Msg::Cancel), send]).gap(8.0).width(Length::Fill);
        column(vec![card(column(vec![
            tile_head("dialog-password-symbolic", t!("plugins-pin-title"), vec![]),
            note(not_saved()).width(Length::Fill),
            dots,
            said,
            column(vec![pad(b'1'), pad(b'4'), pad(b'7'), last]).gap(8.0),
            buttons,
        ]).gap(12.0))]).padding(24.0).on_escape(Msg::Cancel)
    }
}

pub fn ask(events: &Loop, display: &Rc<Display>, make: impl Fn() -> Command + 'static) -> Option<Rc<Shown<Card>>> {
    let slot: Rc<RefCell<Weak<Shown<Card>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let answers = events.post(move |m: Msg| {
        if let Some(shown) = s.borrow().upgrade() {
            shown.change(|c| c.update(m));
        }
    });
    let quit = events.clone();
    let out = events.post(move |o: Out| match o {
        Out::Close => quit.quit(),
    });
    let launch: Box<dyn Fn(&mut Digits)> = Box::new(move |digits| {
        let back = answers.clone();
        start(make(), digits, move |answer| {
            back.send(Msg::Answer(answer));
        });
    });
    let kind = Kind::Window { title: t!("plugins-pin-title").into(), app_id: "trindle-pin".into() };
    let shown = Rc::new(Shown::open(events, display, kind, SIZE, Card::new(launch, out.clone()))?);
    shown.set_look(crate::toolkit::theme::held);
    shown.fit();
    shown.window.on_close(move || {
        out.send(Out::Close);
    });
    *slot.borrow_mut() = Rc::downgrade(&shown);
    Some(shown)
}

pub fn main_for(id: &str, computer: &str) -> i32 {
    let _ = crate::common::renewed::not_traceable();
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };
    let (id, computer) = (id.to_string(), computer.to_string());
    let Some(shown) = ask(&events, &display, move || unlock_command(&id, &computer)) else { return 1 };
    events.run();
    drop(shown);
    0
}

fn unlock_command(id: &str, computer: &str) -> Command {
    use std::os::unix::process::CommandExt;
    let program = crate::common::run::find("trindle-plugin").or_else(|| std::env::current_exe().ok()).unwrap_or_else(|| "trindle-plugin".into());
    let mut command = Command::new(program);
    command.arg0("trindle-plugin").args(["run", id, "unlock", computer]);
    command
}

#[cfg(test)]
mod tests;
