//! Plugins in the ring: each one's icon sits with other programs' tray
//! icons, and what an allowed plugin writes to notify.jsonl in its home is
//! shown as a notification under its name.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use super::ctx::{msg, Ctx, Do, Msg};
use super::notify;
use super::sni::look::Look;
use super::sni::menu::{menu_frame, menu_gap, menu_line, menu_words};
use super::view::Face;
use crate::plugins::{self, DynItem, MenuItem, Plugin, RingNote};
use crate::toolkit::event::Flow;
use crate::toolkit::icon::Icons;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::watch::Watch;
use crate::toolkit::widget::{self, Length, Node, Pointer};
use crate::toolkit::t;

const TRIM_AT: u64 = 64 * 1024;
const KEEP: usize = 20;

const DROP_AT: u64 = 1024 * 1024;

const SETTLE: Duration = Duration::from_millis(300);

struct Button {
    id: String,
    label: String,
    look: Rc<Look>,

    menu: Vec<MenuItem>,
}

#[derive(Default)]
struct State {
    ctx: Option<Rc<Ctx>>,
    watch: Option<Rc<RefCell<Watch>>>,
    buttons: Vec<Button>,

    notes: Vec<(Plugin, PathBuf)>,
    pending: bool,

    said: std::collections::HashMap<String, u32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());

    static OPENED: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

pub fn start(ctx: &Rc<Ctx>) {
    let _ = std::fs::create_dir_all(plugins::folders());
    let watch = Watch::new().ok().map(|w| Rc::new(RefCell::new(w)));
    if let Some(w) = &watch {
        let fd = w.borrow().fd();
        let w = w.clone();
        ctx.lp.watch(fd, move |_| {
            let changed = w.borrow().changed();
            for path in changed {
                heard(&path);
            }
            Flow::Continue
        });
    }
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.ctx = Some(ctx.clone());
        s.watch = watch;
    });
    refresh();
}

fn heard(path: &Path) {
    let note = STATE.with(|s| s.borrow().notes.iter().find(|(_, n)| n == path).map(|(p, n)| (p.clone(), n.clone())));
    if let Some((p, notes)) = note {
        if notes.is_file() {
            look(&p, &notes);
        }
        return;
    }
    let all = plugins::folders();
    let parent = path.parent().unwrap_or(Path::new(""));

    if parent == all || parent.parent() == Some(all.as_path()) {
        soon();
    }
}

fn soon() {
    let (already, ctx) = STATE.with(|s| {
        let mut s = s.borrow_mut();
        (std::mem::replace(&mut s.pending, true), s.ctx.clone())
    });
    let Some(ctx) = ctx else { return };
    if !already {
        ctx.lp.after(SETTLE, || {
            STATE.with(|s| s.borrow_mut().pending = false);
            refresh();
        });
    }
}

fn refresh() {
    let all = plugins::installed();

    plugins::launcher::sync();
    if super::sni::tray().is_none() {
        return;
    }
    let (ctx, watch) = STATE.with(|s| {
        let s = s.borrow();
        (s.ctx.clone(), s.watch.clone())
    });
    let Some(ctx) = ctx else { return };

    let watching = |path: &Path| {
        let _ = std::fs::create_dir_all(path);
        if let Some(w) = &watch {
            let _ = w.borrow_mut().folder(path);
        }
    };
    watching(&plugins::folders());
    let mut buttons = Vec::new();
    let mut notes = Vec::new();
    for p in &all {
        watching(&plugins::folder(&p.id));
        if let Some((icon, label, _)) = p.ring.as_ref().filter(|_| plugins::launcher::status_icon(p)) {
            let file = icon.to_string_lossy().into_owned();
            let look = Rc::new(Look::shape(Rc::new(Icons::new()), file));
            let menu = plugins::menu_of(&plugins::folder(&p.id).join("app"));
            buttons.push(Button { id: p.id.clone(), label: label.clone(), look, menu });
        }
        if plugins::allowed(p).iter().any(|a| a == "notifications") {
            let home = plugins::home_of(&p.id);
            let file = home.join(plugins::NOTES);
            look(p, &file);

            watching(&home);
            notes.push((p.clone(), file));
        }
    }
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.buttons = buttons;
        s.notes = notes;
    });
    super::chosen::settle();
    ctx.tell(Do::Redraw);
}

pub(super) fn ids() -> Vec<String> {
    STATE.with(|s| s.borrow().buttons.iter().rev().map(|b| super::chosen::plugin_id(&b.id)).collect())
}

pub(super) fn icons(in_panel: bool) -> Vec<(String, Node<Msg>)> {
    let size = crate::toolkit::theme::sheet().ring_button;
    STATE.with(|s| s.borrow().buttons.iter().rev().map(|b| {
        let (id, look) = (b.id.clone(), b.look.clone());
        let button = widget::button("", b.label.clone()).size(size).tinted(super::Job::Tray.share())
            .face(move |c, r, ink, pen| look.paint(c, r, ink, pen))
            .on_press(msg(move |f| {
                if in_panel {
                    f.quick.hide();
                }
                pressed(&id)
            }));
        let key = if in_panel { format!("panel:{}", anchor(&b.id)) } else { anchor(&b.id) };
        let node = widget::layers(vec![button, over(&b.id, &b.label, size)]).width(Length::Fixed(size)).height(Length::Fixed(size)).key(key);
        (super::chosen::plugin_id(&b.id), node)
    }).collect())
}

fn over(id: &str, label: &str, size: f32) -> Node<Msg> {
    let (by_pointer, by_key, by_click, by_enter) = (id.to_string(), id.to_string(), id.to_string(), id.to_string());
    widget::drawing((size, size), label.to_string(), |_, _| {})
        .pressable(msg(move |_| pressed(&by_enter)))
        .on_pointer(move |p| match p {
            Pointer::Menu(..) => open_by(&by_pointer, false),
            p if let_go_over(&p, size) => {
                let id = by_click.clone();
                msg(move |_| pressed(&id))
            }
            _ => Msg::idle(),
        })
        .on_keys(move |input| match input {
            Input::Key { sym: Keysym::Menu, .. } => Some(open_by(&by_key, true)),
            Input::Key { sym: Keysym::F10, mods, .. } if mods.shift => Some(open_by(&by_key, true)),
            _ => None,
        })
}

fn let_go_over(p: &Pointer, size: f32) -> bool {
    matches!(p, Pointer::Up(x, y) if (0.0..=size).contains(x) && (0.0..=size).contains(y))
}

fn pressed(id: &str) {
    crate::common::run::spawn(&["trindle-plugin", "run", id, "ring"]);
    let Some(p) = plugins::find(id) else { return };
    let (asks, allowed) = plugins::standing(&p);
    let Some((title, body)) = nothing_allowed(&p.name, asks, allowed) else { return };
    let icon = p.ring.as_ref().map(|r| r.0.to_string_lossy().into_owned()).unwrap_or_default();
    let before = STATE.with(|s| s.borrow().said.get(id).copied().unwrap_or(0));
    let now = notify::say_allow(&p.name, &icon, &title, &body, id, before);
    STATE.with(|s| s.borrow_mut().said.insert(id.to_string(), now));
}

fn nothing_allowed(name: &str, asks: usize, allowed: usize) -> Option<(String, String)> {
    if asks == 0 || allowed > 0 {
        return None;
    }
    Some((t!("ring-plugin-needs", name = name), t!("ring-plugin-cannot-work", asks = asks)))
}

fn anchor(id: &str) -> String {
    format!("plugin:{id}")
}

fn open_by(id: &str, by_key: bool) -> Msg {
    let id = id.to_string();
    msg(move |f| open_menu(f, &id, by_key))
}

#[derive(Clone, Debug, PartialEq)]
enum Line {

    Headline(String),

    Item(MenuItem),

    Now(DynItem),

    Gap,

    Ring,

    Close,

    Settings,
}

fn lines(items: &[MenuItem], note: &RingNote) -> Vec<Line> {
    let mut all = Vec::new();
    if !note.headline.is_empty() {
        all.push(Line::Headline(note.headline.clone()));
    }
    all.extend(items.iter().filter(|i| !note.hidden.contains(&i.id)).cloned().map(Line::Item));
    all.extend(note.items.iter().cloned().map(Line::Now));
    if !all.is_empty() {
        all.push(Line::Gap);
    }
    all.push(Line::Close);
    all.push(Line::Settings);
    all
}

fn open_settings(f: &mut Face, _id: &str) {
    f.settings(Some("plugins"));
}

fn open_menu(f: &mut Face, id: &str, by_key: bool) {
    let Some(items) = STATE.with(|s| s.borrow().buttons.iter().find(|b| b.id == id).map(|b| b.menu.clone())) else { return };
    let mut note = plugins::ring_note(id);

    if plugins::pin_command(id).is_none() {
        note.items.retain(|i| i.unlock.is_none());
    }
    let mut all = lines(&items, &note);
    if let Some(at) = all.iter().position(|l| *l == Line::Close) {
        all.insert(at, Line::Ring);
    }
    f.close_menu();
    let id = id.to_string();
    let turn = if by_key { OPENED.with(|n| { n.set(n.get() + 1); n.get() }) } else { 0 };
    f.show_beside(&anchor(&id), Rc::new(move || menu_card(&id, &all, turn)));
}

fn menu_card(id: &str, all: &[Line], turn: u32) -> Node<Msg> {
    let first = all.iter().position(|l| matches!(l, Line::Item(_) | Line::Now(_) | Line::Ring | Line::Close | Line::Settings));
    menu_frame(all.iter().enumerate().map(|(n, line)| match line {
        Line::Headline(words) => menu_words(words.clone()),
        Line::Gap => menu_gap(),
        Line::Item(item) => {
            let (id, item_id) = (id.to_string(), item.id.clone());
            let line = menu_line(item.label.clone(), msg(move |f| {
                crate::common::run::spawn(&["trindle-plugin", "run", &id, "menu", &item_id]);
                f.close_beside();
            }));
            if Some(n) == first { line.grab(turn) } else { line }
        }
        Line::Now(item) => {
            let (id, args, unlock) = (id.to_string(), item.args.clone(), item.unlock.clone());

            let line = menu_line(item.label.clone(), msg(move |f| {

                let mut words: Vec<&str> = match &unlock {
                    Some(computer) => vec!["trindle-plugin", "run", &id, "pin", computer],
                    None => vec!["trindle-plugin", "run", &id, "args"],
                };
                words.extend(args.iter().map(String::as_str));
                crate::common::run::spawn(&words);
                f.close_beside();
            }));
            if Some(n) == first { line.grab(turn) } else { line }
        }
        Line::Ring => {
            let line = super::chosen::menu_part(&super::chosen::plugin_id(id), false);
            if Some(n) == first { line.grab(turn) } else { line }
        }
        Line::Close => {
            let id = id.to_string();
            let line = menu_line(t!("ring-close").into(), msg(move |f| {
                if !f.close_windows_named(&id) {
                    f.say(t!("ring-plugin-not-open"), 3);
                }
                f.close_beside();
            }));
            if Some(n) == first { line.grab(turn) } else { line }
        }
        Line::Settings => {
            let id = id.to_string();
            let line = menu_line(t!("ring-settings").into(), msg(move |f| open_settings(f, &id)));
            if Some(n) == first { line.grab(turn) } else { line }
        }
    }).collect())
}

fn look(p: &Plugin, notes: &Path) {
    let seen = notes.with_file_name("notify.seen");
    let icon = p.ring.as_ref().map(|r| r.0.to_string_lossy().into_owned()).unwrap_or_default();
    for line in plugins::unread_notes(notes, &seen, TRIM_AT, DROP_AT, KEEP) {
        if let Some((summary, body)) = note(&line) {
            notify::say_as(&p.name, &icon, &summary, &body);
        }
    }
}

fn note(line: &str) -> Option<(String, String)> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    let text = |k: &str| v.get(k).and_then(|s| s.as_str()).unwrap_or("").chars().take(500).collect::<String>();
    let summary = text("summary");
    (!summary.trim().is_empty()).then(|| (summary, text("body")))
}

#[cfg(test)]
mod tests {
    use super::{lines, note, Line};
    use crate::plugins::{DynItem, MenuItem, RingNote};

    #[test]
    fn a_note_needs_a_summary() {
        assert_eq!(note(r#"{"summary":"Rain at 3","body":"Take a coat"}"#), Some(("Rain at 3".into(), "Take a coat".into())));
        assert_eq!(note(r#"{"summary":"Sun"}"#), Some(("Sun".into(), String::new())));
        assert_eq!(note(r#"{"body":"no summary"}"#), None);
        assert_eq!(note("not json"), None);
    }

    fn item(id: &str) -> MenuItem {
        MenuItem { id: id.into(), label: format!("Item {id}"), run: vec![format!("bin/{id}")] }
    }

    #[test]
    fn a_menu_has_its_items_then_a_gap_then_settings() {
        let all = lines(&[item("connect"), item("disconnect")], &RingNote::default());
        assert_eq!(all, [Line::Item(item("connect")), Line::Item(item("disconnect")), Line::Gap, Line::Close, Line::Settings]);

        assert_eq!(lines(&[], &RingNote::default()), [Line::Close, Line::Settings]);
    }

    #[test]
    fn a_headline_comes_first_and_hidden_items_are_left_out() {
        let note = RingNote { headline: "Connected to desktop.".into(), hidden: vec!["connect".into(), "nothing".into()], ..Default::default() };
        let all = lines(&[item("connect"), item("disconnect")], &note);
        assert_eq!(all, [Line::Headline("Connected to desktop.".into()), Line::Item(item("disconnect")), Line::Gap, Line::Close, Line::Settings]);

        let bare = RingNote { headline: "Off.".into(), hidden: vec!["connect".into()], ..Default::default() };
        assert_eq!(lines(&[item("connect")], &bare), [Line::Headline("Off.".into()), Line::Gap, Line::Close, Line::Settings]);

        let hide = RingNote { headline: String::new(), hidden: vec!["settings".into()], ..Default::default() };
        assert_eq!(lines(&[], &hide), [Line::Close, Line::Settings]);
    }

    #[test]
    fn a_click_let_go_over_the_icon_presses_it_and_one_dragged_off_does_not() {
        use super::{let_go_over, Pointer};
        assert!(let_go_over(&Pointer::Up(14.0, 14.0), 28.0));
        assert!(!let_go_over(&Pointer::Up(40.0, 14.0), 28.0));
        assert!(!let_go_over(&Pointer::Up(14.0, -3.0), 28.0));

        assert!(!let_go_over(&Pointer::Down(14.0, 14.0), 28.0));
        assert!(!let_go_over(&Pointer::Move(14.0, 14.0), 28.0));
        assert!(!let_go_over(&Pointer::Menu(14.0, 14.0), 28.0));
    }

    #[test]
    fn a_plugin_pressed_while_allowed_nothing_is_said_with_allow() {
        assert_eq!(super::nothing_allowed("Weather", 2, 0),
                   Some(("Weather needs your permission".to_string(), "It cannot work yet: 0 of 2 allowed.".to_string())));

        assert_eq!(super::nothing_allowed("Weather", 2, 1), None);
        assert_eq!(super::nothing_allowed("Weather", 2, 2), None);
        assert_eq!(super::nothing_allowed("Weather", 0, 0), None);
    }

    #[test]
    fn items_ring_json_adds_come_after_the_plugins_own_and_before_the_gap() {
        let now = |label: &str, args: &[&str]| DynItem { label: label.into(), args: args.iter().map(|s| s.to_string()).collect(), unlock: None };
        let note = RingNote { items: vec![now("Go to desktop", &["goto", "abc123"]), now("Back", &[])], ..Default::default() };
        let all = lines(&[item("connect")], &note);
        assert_eq!(all, [Line::Item(item("connect")), Line::Now(now("Go to desktop", &["goto", "abc123"])), Line::Now(now("Back", &[])),
                         Line::Gap, Line::Close, Line::Settings]);

        assert_eq!(lines(&[], &note).len(), 5);
        let card = super::menu_card("org.example.One", &all, 0);
        assert_eq!(card.said(), ["Item connect", "Go to desktop", "Back", "Close", "Settings"]);

        let odd = RingNote { items: vec![now("<b>bold</b>", &["x"])], ..Default::default() };
        assert_eq!(super::menu_card("a.b", &lines(&[], &odd), 0).said(), ["<b>bold</b>", "Close", "Settings"]);
    }

    #[test]
    fn every_plugin_button_shows_its_name_on_pointing() {
        for label in ["Flow", "Connect to desktop", "A"] {
            assert_eq!(super::over("org.example.One", label, 40.0).tip(), Some(label));
        }
    }

    #[test]
    fn the_menu_card_shows_its_lines_in_order() {
        let note = RingNote { headline: "Connected to desktop.".into(), hidden: vec!["connect".into()], ..Default::default() };
        let card = super::menu_card("org.example.One", &lines(&[item("connect"), item("disconnect")], &note), 0);
        assert_eq!(card.said(), ["Connected to desktop.", "Item disconnect", "Close", "Settings"]);
        let plain = super::menu_card("org.example.One", &lines(&[], &RingNote::default()), 0);
        assert_eq!(plain.said(), ["Close", "Settings"]);
    }

    #[test]
    #[ignore]
    fn shoot_ring_plugin_menu() {
        let now = |label: &str| DynItem { label: label.into(), args: vec!["goto".into()], unlock: None };
        let note = RingNote { headline: "Connected to the desktop.".into(), items: vec![now("Go to desktop"), now("Go to the second screen")],
                              ..Default::default() };
        let card = super::menu_card("org.example.One", &lines(&[item("connect"), item("disconnect"), item("sync")], &note), 0);
        crate::ring::shots::save("plugin-menu", crate::toolkit::widget::card(card).floats(), (300, 300));
    }
}
