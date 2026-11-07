//! The password window a program's prompt opens, in the look of Settings.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use zeroize::Zeroizing;

use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::tiles::tile_head;
use crate::toolkit::widget::{button, card, column, entry, label, main_action, note, row, space, spinner, Length, Node, Tone};
use crate::toolkit::window::{Display, Kind};

pub type Answer = Box<dyn FnOnce(Result<(), String>)>;

const SIZE: (u32, u32) = (440, 240);

#[derive(Clone)]
enum Msg {
    Pass(String),
    Submit,
    Cancel,
    Answer(Result<(), String>),
}

enum Out {
    Check(Zeroizing<String>),
    Done(bool),
}

struct Prompt {
    why: String,
    program: Option<String>,
    password: Zeroizing<String>,
    busy: bool,
    said: Option<(String, Tone)>,
    turn: u32,
    out: crate::toolkit::event::Post<Out>,
}

impl App for Prompt {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Pass(t) => {
                if !self.busy {
                    self.password = Zeroizing::new(t);
                }
            }
            Msg::Submit => {
                if self.busy {
                    return;
                }
                if self.password.is_empty() {
                    self.said = Some((t!("keyring-type-first").into(), Tone::Plain));
                    return;
                }
                self.busy = true;
                self.said = Some((t!("keyring-checking").into(), Tone::Plain));
                self.out.send(Out::Check(self.password.clone()));
            }
            Msg::Cancel => {
                self.out.send(Out::Done(false));
            }
            Msg::Answer(Ok(())) => {
                self.out.send(Out::Done(true));
            }
            Msg::Answer(Err(problem)) => {
                self.password = Zeroizing::new(String::new());
                self.busy = false;
                self.turn += 1;
                self.said = Some((problem, Tone::Problem));
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let password = entry(self.password.to_string(), t!("keyring-password"), Msg::Pass).key("pass")
            .secret().first().grab(self.turn).width(Length::Fill);
        let password = if self.busy { password } else { password.on_submit(Msg::Submit) };
        let said = match &self.said {
            Some((words, tone)) => label(words.clone()).tone(*tone),
            None => space(),
        };
        let unlock = main_action("changes-allow-symbolic", t!("keyring-unlock")).off(self.busy);
        let unlock = if self.busy { unlock } else { unlock.on_press(Msg::Submit) };
        let mut buttons = vec![said.width(Length::Fill)];
        if self.busy {
            buttons.push(spinner());
        }
        buttons.push(button("window-close-symbolic", t!("keyring-cancel")).on_press(Msg::Cancel));
        buttons.push(unlock);
        let mut parts = vec![
            tile_head("dialog-password-symbolic", t!("keyring-password"), vec![]),
            note(self.why.clone()).width(Length::Fill),
        ];
        if let Some(program) = &self.program {
            parts.push(note(t!("keyring-asked-by", program = program)).width(Length::Fill));
        }
        parts.extend([password, row(buttons).gap(8.0)]);
        column(vec![card(column(parts).gap(12.0))]).padding(24.0).on_escape(Msg::Cancel)
    }
}

pub fn ask(events: &Loop, display: &Rc<Display>, title: &str, why: &str, program: Option<String>,
           check: impl Fn(Zeroizing<String>, Answer) + 'static,
           done: impl FnOnce(bool) + 'static) -> Option<Box<dyn Fn()>> {
    let kept: Rc<RefCell<Option<Rc<Shown<Prompt>>>>> = Rc::new(RefCell::new(None));
    let slot: Rc<RefCell<Weak<Shown<Prompt>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let answers = events.post(move |m: Msg| {
        if let Some(shown) = s.borrow().upgrade() {
            shown.change(|p| p.update(m));
        }
    });
    let mut done = Some(Box::new(done) as Box<dyn FnOnce(bool)>);
    let k = kept.clone();
    let out = events.post(move |o: Out| match o {
        Out::Check(password) => {
            let a = answers.clone();
            check(password, Box::new(move |r| {
                a.send(Msg::Answer(r));
            }));
        }
        Out::Done(ok) => {
            if let Some(d) = done.take() {
                d(ok);
            }
            k.borrow_mut().take();
        }
    });
    let app = Prompt { why: why.into(), program, password: Zeroizing::new(String::new()), busy: false, said: None, turn: 0, out: out.clone() };
    let kind = Kind::Window { title: title.into(), app_id: "trindle-keyring".into() };
    let shown = Shown::open(events, display, kind, SIZE, app)?;
    let shown = Rc::new(shown);
    shown.set_look(crate::toolkit::theme::held);
    shown.fit();
    shown.window.on_close(move || {
        out.send(Out::Done(false));
    });
    *slot.borrow_mut() = Rc::downgrade(&shown);
    *kept.borrow_mut() = Some(shown);
    Some(Box::new(move || {
        let gone = kept.borrow_mut().take();
        drop(gone);
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_password_box_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        use crate::toolkit::ui::{App, Ui};
        let events = Loop::new();
        drawn::none(drawn::in_every_look("the keyring's password box", 12, || {
            for (said, busy) in [(None, false), (Some(("That password is not right.".to_string(), Tone::Problem)), false),
                                 (Some(("Checking\u{2026}".to_string(), Tone::Plain)), true)] {
                let prompt = Prompt {
                    why: "Unlock the login keyring so programs can use the passwords kept in it.".into(),
                    program: Some("Firefox".into()), password: Zeroizing::new("hunter2".into()), busy, said, turn: 0, out: events.post(|_: Out| {}),
                };
                let mut ui = Ui::new(prompt.view(), crate::toolkit::theme::held());
                let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
            }
        }));
    }

    #[test]
    fn the_password_box_takes_typing_as_it_opens_and_after_a_refusal() {
        use crate::toolkit::input::{Input, Mods};
        use crate::toolkit::ui::Ui;
        use crate::toolkit::theme::held;
        let events = Loop::new();
        let mut prompt = Prompt { why: "x".into(), program: None, password: Zeroizing::new(String::new()), busy: false, said: None, turn: 0, out: events.post(|_: Out| {}) };
        let mut ui = Ui::new(prompt.view(), held());
        let mut pix = tiny_skia::Pixmap::new(440, 280).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        let typed = |t: &str| Input::Key { sym: crate::toolkit::input::Keysym::NoSymbol, text: Some(t.into()), mods: Mods::default(), repeat: false };
        let now = std::time::Instant::now();
        let said = |ui: &mut Ui<Msg>, prompt: &mut Prompt, t: &str| {
            let reply = ui.input(&typed(t), now);
            assert!(reply.messages.iter().any(|m| matches!(m, Msg::Pass(_))), "{t} typed into a box just opened went nowhere");
            for m in reply.messages {
                prompt.update(m);
            }
            ui.set_view(prompt.view());
            ui.paint(&mut tiny_skia::Pixmap::new(440, 280).unwrap().as_mut(), 1.0);
        };
        said(&mut ui, &mut prompt, "a");
        assert_eq!(prompt.password.as_str(), "a");
        prompt.update(Msg::Answer(Err("That password is not right.".into())));
        ui.set_view(prompt.view());
        ui.paint(&mut tiny_skia::Pixmap::new(440, 280).unwrap().as_mut(), 1.0);
        said(&mut ui, &mut prompt, "b");
        assert_eq!(prompt.password.as_str(), "b");
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_password_window() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let events = Loop::new();
        for (name, said, busy) in [("keyring", None, false),
                                   ("keyring-wrong", Some(("That password is not right.".to_string(), Tone::Problem)), false),
                                   ("keyring-checking", Some(("Checking…".to_string(), Tone::Plain)), true)] {
            let prompt = Prompt {
                why: "Unlock the login keyring so programs can use the passwords kept in it.".into(),
                program: Some("Firefox".into()), password: Zeroizing::new("hunter2".into()), busy, said, turn: 0, out: events.post(|_: Out| {}),
            };
            for (w, h) in [(440u32, 280u32), (1280, 800), (1920, 1080)] {
                let mut ui = Ui::new(prompt.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
                let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                pix.save_png(std::path::Path::new(&dir).join(format!("{name}-{w}.png"))).unwrap();
            }
        }
    }

    #[test]
    fn the_password_box_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "keyring-unlock = Déverrouiller\nkeyring-cancel = Annuler\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(t!("keyring-unlock"), "Déverrouiller");
            assert_eq!(t!("keyring-cancel"), "Annuler");
            assert_eq!(t!("keyring-password"), "Password");
        });
    }
}
