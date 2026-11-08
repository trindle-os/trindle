//! The question put to the person before an assistant changes anything: a
//! small window of its own, so the server's standard input and output stay
//! free for the assistant. A tick says yes; the cross, Escape, closing it or
//! two minutes without an answer say no.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::toolkit::t;
use crate::toolkit::event::Loop;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, column, label, row, space, title, Node};
use crate::toolkit::window::{Display, Kind};

const WAIT: Duration = Duration::from_secs(120);
const WIDTH: u32 = 480;
const MARGIN: f32 = 20.0;

pub fn person(question: &str) -> bool {
    use std::os::unix::process::CommandExt;
    let mut window = std::process::Command::new("/proc/self/exe");
    window.arg0("trindle-mcp").args(["--ask", question]).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null());

    if cfg!(test) {
        window.stderr(std::process::Stdio::null());
    }
    window.status().is_ok_and(|s| s.success())
}

pub fn main(question: String) -> i32 {
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };
    let yes = Rc::new(Cell::new(false));
    let app = Ask { question, yes: yes.clone(), events: events.clone() };
    let kind = Kind::Window { title: t!("mcp-title-ask").into(), app_id: "trindle-mcp".into() };
    let Some(shown) = Shown::open(&events, &display, kind, (WIDTH, 1), app) else { return 1 };
    shown.fit();
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    let ev = events.clone();
    events.after(WAIT, move || ev.quit());
    events.run();
    if yes.get() { 0 } else { 1 }
}

#[derive(Clone)]
enum Answer {
    Yes,
    No,
}

struct Ask {
    question: String,
    yes: Rc<Cell<bool>>,
    events: Loop,
}

impl App for Ask {
    type Msg = Answer;

    fn update(&mut self, answer: Answer) {
        self.yes.set(matches!(answer, Answer::Yes));
        self.events.quit();
    }

    fn view(&self) -> Node<Answer> {
        column(vec![
            title(t!("mcp-ask-heading")),
            label(self.question.clone()),
            row(vec![
                space(),
                button("window-close-symbolic", t!("mcp-no")).on_press(Answer::No).first(),
                crate::toolkit::widget::word_button(t!("mcp-yes")).on_press(Answer::Yes),
            ]),
        ]).gap(16.0).padding(MARGIN).on_escape(Answer::No)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_assistants_question_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let events = Loop::new();
        drawn::none(drawn::in_every_look("the assistant's question", 4, || {
            for question in ["Turn the night light on.", "Install three programs, remove one, change the wallpaper to the newest picture in Pictures and set the time zone to London."] {
                let ask = Ask { question: question.into(), yes: Rc::new(Cell::new(false)), events: events.clone() };
                drawn::window(question, ask.view(), Theme::for_page("mcp"), (WIDTH, 220), false);
            }
        }));
    }
}
