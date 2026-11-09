//! `trindle-boxed --restart-ask`: the question before everything, the compositor too, is restarted. Nothing happens unless Restart is pressed.

use crate::toolkit::t;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use crate::toolkit::event::Loop;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, column, headline, main_action, note, row, space, Node};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

#[derive(Clone)]
pub enum Msg {
    Restart,
    Cancel,
}

pub struct Ask {
    restart: Rc<Cell<bool>>,
    events: Loop,
}

impl App for Ask {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        self.restart.set(matches!(msg, Msg::Restart));
        self.events.quit();
    }

    fn view(&self) -> Node<Msg> {
        view_of()
    }
}

pub fn view_of() -> Node<Msg> {
    let actions = vec![
        space(),
        button("window-close-symbolic", t!("session-restart-cancel")).on_press(Msg::Cancel),
        main_action("system-reboot-symbolic", t!("session-restart-go")).on_press(Msg::Restart),
    ];

    column(vec![headline(t!("session-restart-question")), note(t!("session-restart-windows")), row(actions).gap(12.0).centred()])
        .gap(16.0).padding(24.0).on_escape(Msg::Cancel)
}

const WIDTH: u32 = 520;

fn lock() -> Option<std::fs::File> {
    use std::os::fd::AsRawFd;
    let path = crate::common::restart::flag()?.with_extension("asking");
    let file = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(path).ok()?;
    // SAFETY: flock on a file this program holds open.
    (unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0).then_some(file)
}

fn everything() {
    let Some(flag) = crate::common::restart::flag() else { return };
    // SAFETY: kill only sends a signal.
    everything_with(&flag, || { crate::common::restart::end(crate::common::restart::PARTS); }, crate::common::compositor::desktop(),
                    |pid| {
                        // SAFETY: kill only sends a signal.
                        unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM); }
                    });
}

fn everything_with(flag: &std::path::Path, end_parts: impl FnOnce(), compositor: Option<u32>, end_compositor: impl FnOnce(u32)) {
    let _ = std::fs::write(flag, "");
    end_parts();
    match compositor {
        Some(pid) => end_compositor(pid),
        None => { let _ = std::fs::remove_file(flag); }
    }
}

pub fn after_update() -> i32 {
    use crate::common::updating::{begin_waiting, Who};

    let Some(_turn) = begin_waiting(Who::Boxes, std::time::Duration::from_secs(3600)) else { return 0 };
    crate::common::restart::parts_now();
    0
}

pub fn main() -> i32 {
    let Some(_one_at_a_time) = lock() else { return 1 };
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
    let restart = Rc::new(Cell::new(false));
    let ask = Ask { restart: restart.clone(), events: events.clone() };
    let me: Rc<RefCell<Weak<Shown<Ask>>>> = Rc::new(RefCell::new(Weak::new()));
    let Some(shown) = Shown::open(&events, &display, shape, (WIDTH, 1), ask) else { return 1 };
    let shown = Rc::new(shown);
    *me.borrow_mut() = Rc::downgrade(&shown);
    shown.fit();
    let quit = events.clone();
    shown.window.on_close(move || quit.quit());
    events.run();
    if restart.get() {
        everything();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_restart_question_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("the restart question", 5, || {
            drawn::window("the question", view_of(), Theme::chosen(), (WIDTH, 220), false);
        }));
    }

    #[test]
    fn only_restart_restarts_and_anything_else_does_nothing() {
        let events = Loop::new();
        for (msg, want) in [(Msg::Restart, true), (Msg::Cancel, false)] {
            let restart = Rc::new(Cell::new(!want));
            Ask { restart: restart.clone(), events: events.clone() }.update(msg);
            assert_eq!(restart.get(), want);
        }

        assert!(!Rc::new(Cell::new(false)).get());
    }
}

#[cfg(test)]
mod everything_tests {
    use super::*;

    #[test]
    fn everything_marks_then_ends_the_parts_then_the_compositor_and_never_leaves_a_mark_with_nothing_to_end() {
        let dir = std::env::temp_dir().join(format!("trindle-restart-all-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let flag = dir.join("trindle-restart-all");
        let order = RefCell::new(Vec::new());
        everything_with(&flag, || { assert!(flag.exists(), "the mark comes first"); order.borrow_mut().push("parts".to_string()) },
            Some(4242), |pid| order.borrow_mut().push(format!("compositor {pid}")));
        assert_eq!(*order.borrow(), ["parts", "compositor 4242"]);
        assert!(flag.exists(), "the session finds the mark after the compositor ends");
        let _ = std::fs::remove_file(&flag);
        everything_with(&flag, || {}, None, |_| panic!("nothing to end"));
        assert!(!flag.exists(), "no compositor found: no mark left to log anyone out later");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_package_restarts_the_parts_of_every_logged_in_person_after_an_update() {
        let postinst = include_str!("../../tools/deb/postinst");
        assert!(postinst.contains("--restart-parts-after-update"), "the package does not restart the parts");
        let main = include_str!("../store/boxed/first_apps.rs");
        assert!(main.contains("\"--restart-parts-after-update\" => return crate::session::restart_ask::after_update()"));
        assert!(include_str!("restart_ask.rs").contains("begin_waiting(Who::Boxes"), "it waits for the update to finish first");
    }
}

#[cfg(test)]
mod table_tests {

    #[test]
    fn the_parts_restarted_are_renewing_ones_and_no_window() {
        use crate::common::renewed::{part_named, Rule};
        for p in crate::common::restart::PARTS {
            let rule = part_named(p.name).map(|r| r.rule);
            assert!(matches!(rule, Some(Rule::InPlace | Rule::HandsOver)), "{} is not a renewing part", p.name);
        }
    }
}
