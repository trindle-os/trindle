//! The window an AI program's request opens: `trindle-boxed --ai-approve
//! KIND TEXT`. It is a program of its own, started by the broker outside the
//! fence, with no way for anything inside to press its buttons: the fence
//! has no desktop socket, no display and no way to reach the keyboard. KIND
//! is `share` (TEXT is the full path of a folder) or `sudo` (TEXT is the
//! exact command). The exit code is the answer: 1 no; for a folder 10 and 11
//! share for the next start only and always, 12 and 13 the same read only;
//! for a command 0 allows it.

use crate::toolkit::t;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use crate::toolkit::event::Loop;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, column, form_label, headline, main_action, note, row, section, space, switch, Node};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

#[derive(Clone)]
pub enum Msg {
    ReadOnly(bool),
    Answer(i32),
}

pub struct Ask {
    kind: String,
    text: String,
    read_only: bool,
    code: Rc<Cell<i32>>,
    events: Loop,
}

pub fn question(kind: &str) -> &'static str {
    if kind == "share" {
        t!("store-fence-ask-share")
    } else {
        t!("store-fence-ask-admin")
    }
}

impl App for Ask {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ReadOnly(on) => self.read_only = on,
            Msg::Answer(code) => {

                let code = if self.kind == "share" && code >= 10 && self.read_only { code + 2 } else { code };
                self.code.set(code);
                self.events.quit();
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        view_of(&self.kind, &self.text, self.read_only)
    }
}

pub fn view_of(kind: &str, text: &str, read_only: bool) -> Node<Msg> {
    let mut items = vec![section(t!("store-fence-ai-program")), headline(question(kind)), card(column(vec![form_label(text.to_string())]).padding(12.0))];
    let mut actions = vec![space()];
    if kind == "share" {
        items.push(row(vec![note(t!("store-fence-read-only")), switch(read_only, t!("store-fence-read-only"), Msg::ReadOnly)]).gap(12.0).centred());
        items.push(note(t!("store-fence-sees-next")));
        actions.push(button("window-close-symbolic", t!("store-fence-no")).on_press(Msg::Answer(1)));
        actions.push(button("folder-symbolic", t!("store-fence-share-session")).on_press(Msg::Answer(10)));
        actions.push(main_action(crate::common::no_tick::GO_ICON, t!("store-fence-share-always")).on_press(Msg::Answer(11)));
    } else {
        actions.push(button("window-close-symbolic", t!("store-fence-dont-allow")).on_press(Msg::Answer(1)));
        actions.push(main_action(crate::common::no_tick::GO_ICON, t!("store-fence-allow")).on_press(Msg::Answer(0)));
    }
    items.push(row(actions).gap(12.0).centred());

    column(items).gap(16.0).padding(24.0).on_escape(Msg::Answer(1))
}

const WIDTH: u32 = 560;

pub fn main(kind: &str, text: &str) -> i32 {
    if kind != "share" && kind != "sudo" {
        return 1;
    }
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else {
        eprintln!("trindle-boxed: no screen to ask on");
        return 1;
    };
    let shape = Kind::Panel(Panel {
        layer: Layer::Overlay, edges: Edges::NONE, keep: -1, margins: [0; 4], keys: Keys::Always,
        name: "trindle-permissions".into(), screen: None,
    });
    let code = Rc::new(Cell::new(1));
    let ask = Ask { kind: kind.to_string(), text: text.to_string(), read_only: false, code: code.clone(), events: events.clone() };
    let me: Rc<RefCell<Weak<Shown<Ask>>>> = Rc::new(RefCell::new(Weak::new()));
    let Some(shown) = Shown::open(&events, &display, shape, (WIDTH, 1), ask) else { return 1 };
    let shown = Rc::new(shown);
    *me.borrow_mut() = Rc::downgrade(&shown);
    shown.fit();
    let quit = events.clone();
    shown.window.on_close(move || quit.quit());
    events.run();
    code.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_requests_draw_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("an AI program's request", 12, || {
            for (state, kind, text) in [("a folder", "share", "/home/john/Projects/notes"), ("a command", "sudo", "apt install -y gimp")] {
                drawn::window(state, view_of(kind, text, false), Theme::chosen(), (WIDTH, 360), false);
            }
        }));
    }

    #[test]
    fn a_read_only_choice_changes_the_code_for_a_folder_only() {
        let events = Loop::new();
        for (kind, ro, press, want) in [("share", true, 11, 13), ("share", true, 10, 12), ("share", false, 11, 11), ("share", true, 1, 1), ("sudo", false, 0, 0)] {
            let code = Rc::new(Cell::new(-1));
            let mut a = Ask { kind: kind.into(), text: "x".into(), read_only: ro, code: code.clone(), events: events.clone() };
            a.update(Msg::Answer(press));
            assert_eq!(code.get(), want, "{kind} {ro} {press}");
        }
    }
}
