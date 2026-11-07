//! The permission window.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::boxed;
use crate::common::access;
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, column, form_label, headline, icon_circle, main_action, note, row, scroll, section, space, switch, Length, Node};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub key: String,
    pub name: String,
    pub without: String,
    pub warning: Option<String>,
    pub detail: Option<String>,
    pub refused: Option<String>,
    pub on: bool,
}

pub fn lines(wants: &[String], asked: &[String], allowed: &[String], categories: &str) -> Vec<Line> {
    lines_of(boxed::ALLOWED, wants, asked, allowed, categories)
}

pub fn lines_of(known: &[&str], wants: &[String], asked: &[String], allowed: &[String], categories: &str) -> Vec<Line> {
    let new: Vec<&String> = wants.iter().filter(|w| !asked.contains(w)).collect();
    let shown: Vec<&String> = if new.is_empty() { wants.iter().collect() } else { new };
    shown.into_iter().filter(|k| known.contains(&k.as_str())).map(|k| {
        let a = access::ask(k);
        Line {
            key: k.clone(),
            name: access::title(k),
            without: a.without.to_string(),
            warning: access::warning(k).map(String::from).or_else(|| access::unusual(categories, k)),
            detail: None,
            refused: None,
            on: allowed.contains(k),
        }
    }).collect()
}

pub fn plugin_lines(p: &crate::plugins::Plugin, asked: &[String], allowed: &[String], refusal: Option<String>) -> Vec<Line> {
    let mut lines = lines_of(crate::plugins::PERMISSIONS, &p.asks, asked, allowed, "");
    for l in lines.iter_mut().filter(|l| l.key == "listen") {
        l.detail = Some(access::listen_detail(&p.listen));
        if refusal.is_some() {
            l.refused = refusal.clone();
            l.on = false;
        }
    }
    lines
}

pub fn worked_out_words() -> &'static str {
    t!("store-ask-worked-out")
}

#[derive(Clone)]
pub enum Msg {
    Set(usize, bool),
    AllowAll,
    Done,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Whose {
    Deb,
    Plugin,
}

pub struct Asking {
    pub app: String,
    pub name: String,
    pub whose: Whose,
    pub lines: Vec<Line>,
    events: Loop,
}

impl App for Asking {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Set(i, on) => {
                if let Some(l) = self.lines.get_mut(i) {
                    l.on = on;
                }
            }
            Msg::AllowAll => {
                for l in self.lines.iter_mut().filter(|l| l.refused.is_none()) {
                    l.on = true;
                }
                self.update(Msg::Done);
            }
            Msg::Done => {
                for l in &self.lines {
                    let on = l.on && l.refused.is_none();
                    let kept = match self.whose {
                        Whose::Deb => boxed::allow(&self.app, &l.key, on),
                        Whose::Plugin => crate::plugins::allow(&self.app, &l.key, on),
                    };
                    if let Err(e) = kept {
                        eprintln!("trindle-boxed: {}: {} could not be saved: {e}", self.app, l.key);
                    }
                }
                let _ = match self.whose {
                    Whose::Deb => boxed::mark_asked(&self.app),
                    Whose::Plugin => crate::plugins::find(&self.app).map_or(Ok(()), |p| crate::plugins::mark_asked(&p)),
                };
                if self.whose == Whose::Plugin {
                    crate::common::run::spawn(&["trindle-plugin", "restart", &self.app]);
                    if crate::plugins::find(&self.app).is_some_and(|p| !p.listen.is_empty()) {
                        crate::common::run::spawn(&["trindle-plugin", "firewall", &self.app]);
                    }
                }
                self.events.quit();
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        view_of(&self.name, &self.lines, self.whose == Whose::Deb)
    }
}

pub fn view_of(app: &str, lines: &[Line], worked_out: bool) -> Node<Msg> {
    let mut rows = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let mut words = vec![form_label(l.name.clone()), note(access::off_line(&l.without))];
        if let Some(d) = &l.detail {
            words.push(note(d.clone()));
        }
        if let Some(w) = &l.warning {
            words.push(note(w.clone()));
        }
        if let Some(w) = &l.refused {
            words.push(note(w.clone()));
        }
        let mut cells = vec![icon_circle(access::icon(&l.key).to_string()), column(words).gap(4.0).width(Length::Fill)];
        if l.refused.is_none() {
            cells.push(switch(l.on, access::title(&l.key), move |on| Msg::Set(i, on)));
        }
        rows.push(row(cells).gap(12.0).centred());
    }
    let rows = super::listing::rows(rows);
    let mut items = vec![
        section(t!("store-ask-title", app = app)),
        headline(said_above(lines)).dialog_message(),
        scroll("asked", card(column(rows).gap(4.0))),
    ];
    if worked_out {
        items.push(note(worked_out_words()));
    }
    let mut actions = vec![space()];
    if lines.iter().filter(|l| l.refused.is_none()).count() > 1 {
        actions.push(button("edit-select-all-symbolic", t!("store-ask-allow-all")).on_press(Msg::AllowAll));
    }
    actions.push(main_action(crate::common::no_tick::GO_ICON, t!("store-ask-keep")).on_press(Msg::Done));
    items.push(row(actions).gap(12.0).centred());
    column(items).gap(16.0).padding(24.0).on_escape(Msg::Done)
}

fn said_above(lines: &[Line]) -> &'static str {
    if lines.iter().any(|l| l.on) {
        t!("store-ask-above-on")
    } else {
        t!("store-ask-above-off")
    }
}

const WIDTH: u32 = 560;

pub fn main(name: &str) -> i32 {
    if boxed::version(name).is_none() {
        eprintln!("trindle-boxed: {name} is not installed in its own folder");
        return 1;
    }
    let lines = lines(&boxed::wants(name), &boxed::asked(name), &boxed::allowed(name), &boxed::categories(name));
    if lines.is_empty() {
        let _ = boxed::mark_asked(name);
        return 0;
    }
    show(name, name, Whose::Deb, lines)
}

pub fn plugin_main(id: &str, again: bool) -> i32 {
    let Some(p) = crate::plugins::find(id) else {
        eprintln!("trindle-plugin: {id} is not an installed plugin that conforms");
        return 1;
    };
    let lock = std::fs::File::create(crate::plugins::folder(id).join("asking.lock")).ok();
    if let Some(f) = &lock {
        use std::os::fd::AsRawFd;
        // SAFETY: the descriptor is that of `f`, which is open for as long as `lock` lives.
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return 0;
        }
    }
    let asked = if again { Vec::new() } else { crate::plugins::asked(id) };
    if !again && p.asks.iter().all(|a| asked.contains(a)) {
        return 0;
    }
    let allowed = crate::plugins::allowed(&p);
    let lines = plugin_lines(&p, &asked, &allowed, crate::plugins::listen_refusal(&p));
    if lines.is_empty() {
        let _ = crate::plugins::mark_asked(&p);
        return 0;
    }
    show(id, &p.name, Whose::Plugin, lines)
}

fn show(app: &str, name: &str, whose: Whose, lines: Vec<Line>) -> i32 {
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else {
        eprintln!("trindle-boxed: no screen to ask on");
        return 1;
    };
    let kind = Kind::Panel(Panel {
        layer: Layer::Overlay, edges: Edges::NONE, keep: -1, margins: [0; 4], keys: Keys::Always,
        name: "trindle-permissions".into(), screen: None,
    });
    let me: Rc<RefCell<Weak<Shown<Asking>>>> = Rc::new(RefCell::new(Weak::new()));
    let asking = Asking { app: app.to_string(), name: name.to_string(), whose, lines, events: events.clone() };
    let Some(shown) = Shown::open(&events, &display, kind, (WIDTH, 1), asking) else { return 1 };
    let shown = Rc::new(shown);
    *me.borrow_mut() = Rc::downgrade(&shown);
    shown.fit();
    let quit = events.clone();
    shown.window.on_close(move || quit.quit());
    events.run();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn what_a_program_may_use_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let events = Loop::new();
        drawn::none(drawn::in_every_look("what a program may use", 12, || {
            let first = lines(&v(&["network", "microphone", "files"]), &[], &[], "Graphics;");
            let again = lines(&v(&["network", "sound", "computer"]), &v(&["network", "sound", "computer"]), &v(&["network"]), "System;");
            let mut plugin = lines_of(crate::plugins::PERMISSIONS, &v(&["network", "listen", "control"]), &[], &v(&["control"]), "");
            for l in plugin.iter_mut().filter(|l| l.key == "listen") {
                l.detail = Some("Ports 8080 and 8443".into());
                l.refused = Some("Another program is already listening there.".into());
                l.on = false;
            }
            for (state, whose, lines) in [("a package, first asked", Whose::Deb, first), ("a package, opened again", Whose::Deb, again), ("a plugin", Whose::Plugin, plugin)] {
                assert!(!lines.is_empty(), "{state}");
                let asking = Asking { app: "org.example.paint".into(), name: "Paint".into(), whose, lines, events: events.clone() };
                drawn::window(state, asking.view(), Theme::chosen(), (WIDTH, 640), false);
            }
        }));
    }

    #[test]
    fn a_screen_reader_reaches_every_switch() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let l = lines_of(crate::plugins::PERMISSIONS, &v(&["network", "notifications", "background", "control", "files", "sound", "microphone"]), &[], &[], "");
        assert!(l.len() >= 6);
        let fitted = Ui::new(view_of("Sync", &l, false), Theme::new(true, "#fbc711")).natural_height(WIDTH as f32).ceil() as u32;
        for h in [1, fitted, 300] {
            let mut ui = Ui::new(view_of("Sync", &l, false), Theme::new(true, "#fbc711"));
            ui.lay_at(WIDTH, h);
            let kept = ui.kept_from_a_reader();
            assert!(kept.is_empty(), "at a height of {h} a screen reader could not reach {kept:?}");
        }
    }

    #[test]
    fn the_window_takes_the_keyboard_as_it_opens() {
        let code = crate::common::source::code(include_str!("asking.rs"));
        let made = code.split("fn show(").nth(1).expect("the window is made in show");
        assert!(made.contains(&["keys: Keys::", "Always,"].concat()), "the permission window no longer takes the keyboard as it opens");
        let l = lines_of(crate::plugins::PERMISSIONS, &v(&["network", "control"]), &[], &[], "");
        let mut ui = crate::toolkit::ui::Ui::new(view_of("Sync", &l, false), crate::toolkit::theme::Theme::new(true, "#fbc711"));
        ui.lay_at(WIDTH, 400);
        assert_eq!(ui.access("Sync").focus, accesskit::NodeId(0), "something in the window takes the keyboard by itself, so a stray Enter could press it");
    }

    #[test]
    fn allow_all_switches_on_only_what_can_be_allowed() {
        let mut l = lines_of(crate::plugins::PERMISSIONS, &v(&["network", "sound"]), &[], &[], "");
        l[1].refused = Some("Not on this computer.".into());
        for l in l.iter_mut().filter(|l| l.refused.is_none()) {
            l.on = true;
        }
        assert!(l[0].on && !l[1].on);
        let hears = |lines: &[Line]| {
            let mut ui = crate::toolkit::ui::Ui::new(view_of("Sync", lines, false), crate::toolkit::theme::Theme::new(true, "#fbc711"));
            ui.lay_at(WIDTH, 400);
            ui.access("Sync").nodes.iter().any(|(_, part)| part.label() == Some("Allow all"))
        };
        assert!(hears(&lines_of(crate::plugins::PERMISSIONS, &v(&["network", "sound"]), &[], &[], "")), "two switches and no Allow all");
        let code = crate::common::source::code(include_str!("asking.rs"));
        assert!(code.contains("Msg::AllowAll => {") && code.contains("filter(|l| l.refused.is_none())"));
        assert!(!hears(&lines_of(crate::plugins::PERMISSIONS, &v(&["network"]), &[], &[], "")), "one switch needs no Allow all");
    }

    #[test]
    fn every_switch_is_off_until_pressed() {
        let l = lines(&v(&["network", "sound", "computer"]), &[], &[], "System;TerminalEmulator;");
        assert_eq!(l.iter().map(|l| l.key.as_str()).collect::<Vec<_>>(), ["network", "sound", "computer"]);
        assert!(l.iter().all(|l| !l.on));
        assert_eq!(l[0].name, "The internet");
        assert_eq!(l[1].without, "It will make no sound.");
        assert!(l.iter().all(|l| l.warning.is_none()));
    }

    #[test]
    fn the_line_under_a_switch_says_it_is_for_off() {
        assert_eq!(access::off_line("It will not be able to hear you."), "Off: it will not be able to hear you.");
        assert_eq!(access::off_line("Its windows will not open."), "Off: its windows will not open.");
        assert_eq!(access::off_line(""), "");
        for on in [false, true] {
            let allowed = if on { v(&["control", "network"]) } else { vec![] };
            let l = lines_of(&["control", "network"], &v(&["control", "network"]), &[], &allowed, "");
            for l in &l {
                assert_eq!(l.on, on);
                assert!(access::off_line(&l.without).starts_with("Off: "), "{}", l.key);
            }
        }
        let code = crate::common::source::code(include_str!("asking.rs"));
        let code = &code[..code.find("#[cfg(test)]").unwrap()];
        assert!(code.contains("note(access::off_line(&l.without))") && !code.contains("note(l.without"), "the window shows the line some other way");
    }

    #[test]
    fn an_update_asks_only_for_what_is_new() {
        let l = lines(&v(&["network", "microphone"]), &v(&["network"]), &v(&["network"]), "Graphics;");
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].key, "microphone");
        assert_eq!(l[0].name, "Your microphone");
        assert_eq!(l[0].without, "It will not be able to hear you.");
        assert!(l[0].warning.as_deref().is_some_and(|w| w.contains("a graphics app")));
        let again = lines(&v(&["network", "microphone"]), &v(&["network", "microphone"]), &v(&["network"]), "Graphics;");
        assert_eq!(again.iter().map(|l| l.on).collect::<Vec<_>>(), [true, false]);
    }

    #[test]
    fn unusual_wants_are_warned_of_plainly() {
        let l = lines(&v(&["computer", "network"]), &[], &[], "Utility;Calculator;");
        assert!(l[0].warning.as_deref().is_some_and(|w| w.contains("a calculator")));
        assert!(l[1].warning.is_some(), "a calculator wanting the internet is unusual");
        for l in &l {
            assert!(!l.without.contains('\u{2014}') && !l.name.contains('\u{2014}'));
        }
    }

    #[test]
    fn a_plugin_is_asked_only_for_what_a_plugin_can_have() {
        let l = lines_of(crate::plugins::PERMISSIONS, &v(&["network", "notifications", "background", "computer"]), &[], &[], "");
        assert_eq!(l.iter().map(|l| l.key.as_str()).collect::<Vec<_>>(), ["network", "notifications", "background"]);
        assert!(l.iter().all(|l| !l.on && !l.without.is_empty()));
        let c = lines_of(crate::plugins::PERMISSIONS, &v(&["control"]), &[], &[], "");
        assert_eq!(c[0].name, "Control your keyboard, mouse and clipboard");
        assert!(!c[0].on && c[0].warning.as_deref().is_some_and(|w| w.contains("plugin you trust")));
    }

    #[test]
    fn every_permission_a_box_has_is_put_in_plain_words() {
        for k in boxed::ALLOWED {
            let a = access::ask(k);
            assert_eq!(a.key, *k, "{k} has no words in common::access");
            assert!(!a.without.is_empty() && a.without.ends_with('.'), "{k}");
        }
    }

    #[test]
    #[ignore]
    fn shoot_the_permission_window() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let dir = std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into());
        std::fs::create_dir_all(&dir).unwrap();
        let p = syncing();
        let l = plugin_lines(&p, &[], &[], None);
        for scale in [1.0f32, 1.75] {
            let mut ui = Ui::new(view_of("Sync", &l, false), Theme::new(false, "#fbc711"));
            let h = ui.natural_height(WIDTH as f32).ceil() as u32;
            let mut pix = tiny_skia::Pixmap::new((WIDTH as f32 * scale) as u32, (h as f32 * scale) as u32).unwrap();
            ui.paint(&mut pix.as_mut(), scale);
            pix.save_png(std::path::Path::new(&dir).join(format!("permission-window-{scale}.png"))).unwrap();
        }
    }

    #[test]
    fn opened_again_the_window_shows_everything_as_it_stands() {
        let p = syncing();
        let asked = v(&["network", "listen"]);
        let again = plugin_lines(&p, &[], &v(&["network"]), None);
        assert_eq!(again.iter().map(|l| (l.key.as_str(), l.on)).collect::<Vec<_>>(), [("network", true), ("listen", false)]);
        assert_eq!(plugin_lines(&p, &asked, &v(&["network"]), None), again);
        assert_eq!(plugin_lines(&p, &v(&["network"]), &[], None).len(), 1);
        assert_eq!(plugin_lines(&p, &[], &[], None).len(), 2);
        assert!(said_above(&again).starts_with("Each switch shows what is allowed now"));
        assert!(said_above(&plugin_lines(&p, &[], &[], None)).starts_with("Each switch stays off"));
    }

    #[test]
    fn the_window_builds_with_a_switch_for_each() {
        let l = lines(&v(&["network", "x11"]), &[], &[], "");
        let mut app = Asking { app: "galculator".into(), name: "galculator".into(), whose: Whose::Deb, lines: l.clone(), events: Loop::new() };
        let _ = view_of("galculator", &l, true);
        app.update(Msg::Set(1, true));
        assert!(!app.lines[0].on && app.lines[1].on);
    }

    fn syncing() -> crate::plugins::Plugin {
        use crate::common::firewall::Door;
        crate::plugins::Plugin {
            id: "org.example.Sync".into(), name: "Sync".into(), version: "1".into(), summary: String::new(),
            asks: v(&["network", "listen"]), ring: None, fields: Vec::new(), background: None, shortcuts: Vec::new(),
            listen: vec![Door::parse("tcp:24810").unwrap(), Door::parse("udp:24811").unwrap()],
        }
    }

    #[test]
    fn the_window_names_the_ports_a_plugin_listens_on() {
        let p = syncing();
        let l = plugin_lines(&p, &[], &[], None);
        assert_eq!(l.iter().map(|l| l.key.as_str()).collect::<Vec<_>>(), ["network", "listen"]);
        let listen = &l[1];
        assert_eq!(listen.name, "Let other computers on your home network reach it");
        assert!(listen.detail.as_deref().is_some_and(|d| d.starts_with("Ports TCP 24810 and UDP 24811")));
        assert!(!listen.on && listen.refused.is_none());
        assert!(l[0].detail.is_none());
        let _ = view_of("Sync", &l, false);
    }

    #[test]
    fn a_port_that_cannot_be_had_is_said_and_cannot_be_switched_on() {
        let p = syncing();
        let why = "TCP 24810 is already used by Mirror, so it cannot be opened for this plugin too.".to_string();
        let l = plugin_lines(&p, &[], &v(&["network", "listen"]), Some(why.clone()));
        assert_eq!(l[1].refused.as_deref(), Some(why.as_str()));
        assert!(!l[1].on, "shown off even if it had been on");
        assert!(l[0].on && l[0].refused.is_none());
        let _ = view_of("Sync", &l, false);
    }
}
