//! The question put to the person when a Thunderbolt or USB4 device waits to connect.

use std::cell::RefCell;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::time::Duration;

use super::{Choice, Question};
use crate::toolkit::t;
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::watch::Watch;
use crate::toolkit::widget::{button, column, label, note, row, space, title, Length, Node, Tone};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

const WIDTH: u32 = 500;
const AGAIN: Duration = Duration::from_secs(30);

pub fn why() -> &'static str {
    t!("thunder-why")
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Queue {
    pub questions: Vec<Question>,
    pub locked: bool,
}

impl Queue {
    pub fn heard(&mut self, line: &str) -> bool {
        if let Some(q) = Question::parse(line) {
            match self.questions.iter_mut().find(|o| o.id == q.id) {
                Some(o) if *o == q => false,
                Some(o) => {
                    *o = q;
                    true
                }
                None => {
                    self.questions.push(q);
                    true
                }
            }
        } else if let Some(id) = line.strip_prefix("gone\t") {
            let before = self.questions.len();
            self.questions.retain(|q| q.id != id);
            before != self.questions.len()
        } else {
            false
        }
    }

    pub fn showing(&self) -> Option<&Question> {
        if self.locked { None } else { self.questions.first() }
    }
}

pub fn named(q: &Question) -> (String, String) {
    let name = if q.name.is_empty() { t!("thunder-no-name").to_string() } else { q.name.clone() };
    let maker = if q.vendor.is_empty() {
        t!("thunder-no-maker").to_string()
    } else {
        t!("thunder-made-by", maker = &q.vendor)
    };
    (name, maker)
}

#[derive(Clone)]
pub struct Said(pub String, pub Choice);

pub struct Window {
    pub q: Question,
    pub busy: bool,
    pub problem: Option<String>,
    out: Option<Post<Said>>,
}

impl Window {
    pub fn new(q: Question, out: Option<Post<Said>>) -> Window {
        Window { q, busy: false, problem: None, out }
    }
}

impl App for Window {
    type Msg = Choice;

    fn update(&mut self, choice: Choice) {
        if self.busy {
            return;
        }
        self.busy = true;
        self.problem = None;
        if let Some(out) = &self.out {
            out.send(Said(self.q.id.clone(), choice));
        }
    }

    fn view(&self) -> Node<Choice> {
        let (name, maker) = named(&self.q);
        let mut items = vec![
            row(vec![
                crate::settings::ui::icon("thunderbolt-symbolic", 32.0),
                column(vec![title(t!("thunder-let-connect", name = &name)), note(maker)]).gap(2.0).width(Length::Fill),
            ]).gap(14.0).centred(),
            label(why()),
        ];
        if self.q.doubt {
            items.push(note(t!("thunder-doubt")).tone(Tone::Warn));
        }
        if let Some(p) = &self.problem {
            items.push(note(p.clone()).tone(Tone::Problem));
        }
        let press = |icon: &str, tip: &str, c: Choice| {
            let b = if icon.is_empty() { crate::toolkit::widget::word_button(tip) } else { button(icon, tip) };
            if self.busy { b } else { b.on_press(c) }
        };
        items.push(row(vec![
            space(),
            press("window-close-symbolic", t!("thunder-never"), Choice::Never),
            press("document-open-recent-symbolic", t!("thunder-once"), Choice::Once),
            press("", t!("thunder-always"), Choice::Always),
        ]).gap(12.0));
        column(items).gap(16.0).padding(24.0).on_escape(Choice::Never)
    }
}

fn kind() -> Kind {
    Kind::Panel(Panel {
        layer: Layer::Overlay, edges: Edges::NONE, keep: -1, margins: [0; 4], keys: Keys::OnClick,
        name: "trindle-thunder".into(), screen: None,
    })
}

struct Asker {
    lp: Loop,
    display: Rc<Display>,
    queue: RefCell<Queue>,
    link: RefCell<Option<UnixStream>>,
    got: RefCell<Vec<u8>>,
    shown: RefCell<Option<Rc<Shown<Window>>>>,
    out: Post<Said>,
    _watch: RefCell<Option<Rc<Watch>>>,
}

impl Asker {
    fn connect(self: &Rc<Self>) {
        let stream = UnixStream::connect(super::socket_path()).and_then(|s| {
            s.set_nonblocking(true)?;
            (&s).write_all(b"watch\n")?;
            Ok(s)
        });
        let s = match stream {
            Ok(s) => s,
            Err(_) => {
                let me = Rc::downgrade(self);
                self.lp.after(AGAIN, move || if let Some(a) = me.upgrade() { a.connect() });
                return;
            }
        };
        let fd = s.as_raw_fd();
        *self.link.borrow_mut() = Some(s);
        self.got.borrow_mut().clear();
        let me = Rc::downgrade(self);
        self.lp.watch(fd, move |hung_up| {
            let Some(a) = me.upgrade() else { return Flow::Stop };
            if a.read() && !hung_up {
                return Flow::Continue;
            }
            a.link.borrow_mut().take();
            a.queue.borrow_mut().questions.clear();
            a.show();
            let again = Rc::downgrade(&a);
            a.lp.after(Duration::from_secs(2), move || if let Some(a) = again.upgrade() { a.connect() });
            Flow::Stop
        });
    }

    fn read(self: &Rc<Self>) -> bool {
        let mut open = true;
        {
            let link = self.link.borrow();
            let Some(mut s) = link.as_ref() else { return false };
            let mut buf = [0u8; 2048];
            loop {
                match s.read(&mut buf) {
                    Ok(0) => {
                        open = false;
                        break;
                    }
                    Ok(n) => self.got.borrow_mut().extend_from_slice(&buf[..n]),
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(_) => {
                        open = false;
                        break;
                    }
                }
            }
        }
        let lines: Vec<String> = {
            let mut got = self.got.borrow_mut();
            let mut out = Vec::new();
            while let Some(end) = got.iter().position(|&b| b == b'\n') {
                let line: Vec<u8> = got.drain(..=end).collect();
                out.push(String::from_utf8_lossy(&line[..end]).to_string());
            }
            out
        };
        for line in lines {
            if let Some(rest) = line.strip_prefix("answered\t") {
                self.answered(rest);
            } else if self.queue.borrow_mut().heard(&line) {
                self.show();
            }
        }
        open
    }

    fn answered(self: &Rc<Self>, rest: &str) {
        let mut f = rest.splitn(3, '\t');
        let (id, how, words) = (f.next().unwrap_or(""), f.next().unwrap_or(""), f.next().unwrap_or(""));
        let Some(shown) = self.shown.borrow().clone() else { return };
        if shown.with(|w| w.q.id != id) {
            return;
        }
        match how {
            "ok" | "unknown" => {
                self.queue.borrow_mut().questions.retain(|q| q.id != id);
                self.show();
            }
            "locked" => shown.change(|w| w.busy = false),
            _ => {
                let words = if words.is_empty() { t!("thunder-not-connected") } else { words };
                shown.change(|w| {
                    w.busy = false;
                    w.problem = Some(words.to_string());
                });
                shown.fit();
            }
        }
    }

    fn show(self: &Rc<Self>) {
        let want = self.queue.borrow().showing().cloned();
        let current = self.shown.borrow().as_ref().map(|s| s.with(|w| w.q.id.clone()));
        match (want, current) {
            (None, None) => {}
            (None, Some(_)) => {
                self.shown.borrow_mut().take();
            }
            (Some(q), Some(id)) if q.id == id => {
                if let Some(s) = self.shown.borrow().clone() {
                    s.change(|w| w.q = q);
                    s.fit();
                }
            }
            (Some(q), _) => {
                self.shown.borrow_mut().take();
                let (name, _) = named(&q);
                let win = Window::new(q.clone(), Some(self.out.clone()));
                let Some(shown) = Shown::open(&self.lp, &self.display, kind(), (WIDTH, 1), win) else {
                    eprintln!("trindle-thunder: the question could not be shown");
                    return;
                };
                shown.fit();
                shown.spoken(&format!("Let {name} connect?"));
                let (out, id) = (self.out.clone(), q.id.clone());
                shown.window.on_close(move || {
                    out.send(Said(id.clone(), Choice::Never));
                });
                *self.shown.borrow_mut() = Some(Rc::new(shown));
            }
        }
    }

    fn send(&self, said: Said) {
        let line = format!("answer\t{}\t{}\n", said.0, said.1.word());
        let ok = self.link.borrow().as_ref().is_some_and(|mut s| s.write_all(line.as_bytes()).is_ok());
        if !ok {
            if let Some(s) = self.shown.borrow().clone() {
                s.change(|w| {
                    w.busy = false;
                    w.problem = Some(t!("thunder-guard-silent").into());
                });
                s.fit();
            }
        }
    }

    fn watch_lock(self: &Rc<Self>) {
        let mark = crate::common::locked::path();
        self.queue.borrow_mut().locked = mark.exists();
        let Some(folder) = mark.parent().map(Path::to_path_buf) else { return };
        let Ok(mut w) = Watch::new() else { return };
        if w.folder(&folder).is_err() {
            return;
        }
        let w = Rc::new(w);
        let (me, ww) = (Rc::downgrade(self), w.clone());
        self.lp.watch(w.fd(), move |_| {
            let Some(a) = me.upgrade() else { return Flow::Stop };
            if ww.changed().contains(&mark) {
                let locked = mark.exists();
                let was = std::mem::replace(&mut a.queue.borrow_mut().locked, locked);
                if was != locked {
                    a.show();
                }
            }
            Flow::Continue
        });
        *self._watch.borrow_mut() = Some(w);
    }
}

pub fn start(lp: &Loop, display: &Rc<Display>) {
    if !Path::new("/sys/bus/thunderbolt").exists() && std::env::var_os("TRINDLE_THUNDER_SOCKET").is_none() {
        return;
    }
    let me: Rc<RefCell<Weak<Asker>>> = Rc::new(RefCell::new(Weak::new()));
    let m = me.clone();
    let out = lp.post(move |said: Said| {
        if let Some(a) = m.borrow().upgrade() {
            a.send(said);
        }
    });
    let asker = Rc::new(Asker {
        lp: lp.clone(), display: display.clone(), queue: RefCell::new(Queue::default()), link: RefCell::new(None),
        got: RefCell::new(Vec::new()), shown: RefCell::new(None), out, _watch: RefCell::new(None),
    });
    *me.borrow_mut() = Rc::downgrade(&asker);
    asker.watch_lock();
    asker.connect();
    std::mem::forget(asker);
}

pub fn main() -> i32 {
    let lp = Loop::new();
    let gone = lp.clone();
    let Some(display) = Display::connect(&lp, move || gone.quit()) else { return 1 };
    start(&lp, &display);
    lp.run();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thunder::sysfs::Level;

    fn q(id: &str) -> Question {
        Question { id: id.into(), vendor: "Dock Makers".into(), name: "Big Dock".into(), level: Level::User, iommu: false,
                   doubt: false }
    }

    #[test]
    fn questions_wait_while_the_screen_is_locked() {
        let mut queue = Queue { locked: true, ..Queue::default() };
        assert!(queue.heard(&q("aaaa-1").line()));
        assert!(!queue.heard(&q("aaaa-1").line()), "the same question twice is one question");
        assert!(queue.showing().is_none(), "nothing shows over a locked screen");
        queue.locked = false;
        assert_eq!(queue.showing().map(|q| q.id.as_str()), Some("aaaa-1"));
        assert!(queue.heard(&q("bbbb-2").line()));
        assert!(queue.heard("gone\taaaa-1"));
        assert_eq!(queue.showing().map(|q| q.id.as_str()), Some("bbbb-2"));
        assert!(!queue.heard("gone\tcccc-3") && !queue.heard("nonsense"));
    }

    #[test]
    fn the_thunderbolt_question_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("the Thunderbolt question", 8, || {
            for (state, doubt, problem, busy) in [("plain", false, None, false), ("in doubt", true, None, false),
                                                  ("gone wrong", true, Some("The device could not be allowed. Unplug it and plug it in again."), false),
                                                  ("waiting", false, None, true)] {
                let mut w = Window::new(Question { doubt, ..q("aaaa-1") }, None);
                w.problem = problem.map(String::from);
                w.busy = busy;
                drawn::window(state, w.view(), Theme::chosen(), (520, 360), false);
            }
        }));
    }

    #[test]
    fn the_window_names_the_device_and_answers_once() {
        let (name, maker) = named(&q("aaaa-1"));
        assert_eq!((name.as_str(), maker.as_str()), ("Big Dock", "Made by Dock Makers."));
        let nameless = Question { name: String::new(), vendor: String::new(), ..q("aaaa-1") };
        assert_eq!(named(&nameless).0, "A Thunderbolt device");
        let mut w = Window::new(q("aaaa-1"), None);
        let _ = w.view();
        w.update(Choice::Always);
        assert!(w.busy);
        let _ = w.view();
        assert_eq!(why().matches(". ").count(), 0);
        assert!(!why().contains('\u{2014}'));
    }
}
