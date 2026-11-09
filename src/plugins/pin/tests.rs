//! Tests for pinning a plugin to the version that was checked.

use std::fmt::Write as _;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use super::*;
use crate::common::own_hands::{self, FLOW};
use crate::plugins::manifest::{check, good_computer, pin_command_in};
use crate::plugins::PERMISSIONS;
use crate::toolkit::window::{with_named, with_press};

const SAMPLE: &str = "7391";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn digits(text: &str) -> Digits {
    let mut d = Digits::new();
    for c in text.bytes() {
        d.push(c);
    }
    d
}

fn recorder(dir: &Path, code: i32, line: &str) -> Command {
    let script = dir.join("flow");
    let rec = dir.join("record");
    std::fs::create_dir_all(&rec).unwrap();
    std::fs::write(&script, format!(
        "#!/bin/sh\necho \"$#:$*\" > '{r}/argv'\nenv > '{r}/env'\ncat > '{r}/stdin'\nls -l /proc/$$/fd > '{r}/fds'\necho '{line}'\nexit {code}\n",
        r = rec.display())).unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();

    let mut c = Command::new("/bin/sh");
    c.arg(&script).args(["unlock", "office-1"]);
    c
}

fn answered(command: Command, d: &mut Digits) -> Answer {
    let (tx, rx) = mpsc::channel();
    start(command, d, move |a| {
        let _ = tx.send(a);
    });
    rx.recv_timeout(Duration::from_secs(20)).expect("the command never answered")
}

fn everything_under(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(everything_under(&p));
        } else {
            out.push((p.clone(), std::fs::read(&p).unwrap_or_default()));
        }
    }
    out
}

#[test]
fn the_digits_reach_the_command_on_its_standard_input_only() {
    let dir = scratch("stdin");
    let mut d = digits(SAMPLE);
    assert!(d.full());
    let answer = answered(recorder(&dir, 0, "Unlocked."), &mut d);
    assert_eq!(answer, Ok(()));
    assert_eq!((d.len(), d.raw()), (0, [0; LENGTH]), "the buffer was not wiped once the digits were sent");
    let rec = dir.join("record");
    assert_eq!(std::fs::read_to_string(rec.join("stdin")).unwrap(), format!("{SAMPLE}\n"));
    assert_eq!(std::fs::read_to_string(rec.join("argv")).unwrap().trim(), "2:unlock office-1");

    let parent_env: String = std::env::vars().fold(String::new(), |mut s, (k, v)| { let _ = writeln!(s, "{k}={v}"); s });
    if !parent_env.contains(SAMPLE) {
        assert!(!std::fs::read_to_string(rec.join("env")).unwrap().contains(SAMPLE), "the digits are in the environment");
    }
    assert!(!std::fs::read_to_string(rec.join("argv")).unwrap().contains(SAMPLE), "the digits are in the arguments");
    let fds = std::fs::read_to_string(rec.join("fds")).unwrap();
    let held = fds.lines().filter(|l| l.contains("/tmp/") && !l.ends_with("/flow") && !l.ends_with("/record/fds")).count();
    assert!(!fds.contains(SAMPLE) && held == 0, "a file was held open for it: {fds}");
    for (path, bytes) in everything_under(&dir) {
        if path.file_name().is_some_and(|n| n == "stdin") {
            continue;
        }
        assert!(!String::from_utf8_lossy(&bytes).contains(SAMPLE), "the digits reached {}", path.display());
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn exit_zero_is_done_and_one_is_refused_and_the_digits_are_wiped_either_way() {
    let dir = scratch("exit");
    for (code, line, want) in [
        (0, "Unlocked.", Ok(())),
        (1, "Wrong PIN. 4 tries left.", Err("Wrong PIN. 4 tries left.".to_string())),
        (1, "", Err(not_unlocked().to_string())),
        (2, "", Err(no_answer().to_string())),
        (3, "It broke.", Err("It broke.".to_string())),
    ] {
        let mut d = digits(SAMPLE);
        assert_eq!(answered(recorder(&dir, code, line), &mut d), want, "exit {code}, {line:?}");
        assert_eq!((d.len(), d.raw()), (0, [0; LENGTH]));
    }

    let mut d = digits(SAMPLE);
    assert_eq!(answered(Command::new("/nonexistent/flow"), &mut d), Err(no_answer().to_string()));
    assert_eq!((d.len(), d.raw()), (0, [0; LENGTH]));

    let mut short = digits("73");
    let mut sink = Vec::new();
    assert_eq!(short.hand_over(&mut sink), Err(four()));
    assert!(sink.is_empty() && short.raw() == [0; LENGTH]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_line_it_prints_is_shown_as_plain_short_words() {
    let shown = answer_of(Some(1), "\u{1b}[31mRed\u{7}\u{202e}  words\n").unwrap_err();
    assert_eq!(shown, "[31mRed words");
    let long = answer_of(Some(1), &"x".repeat(500)).unwrap_err();
    assert!(long.chars().count() <= 80, "{}", long.len());
}

#[test]
fn the_buffer_prints_how_many_and_never_which() {
    let d = digits(SAMPLE);
    let said = format!("{d:?}");
    assert!(!said.contains(SAMPLE) && said.contains("4 of 4"), "{said}");
}

#[test]
fn the_cards_code_has_no_way_to_write_the_digits_anywhere() {
    let source = crate::common::source::code(include_str!("../pin.rs"));
    let code = source.split("#[cfg(test)]\nmod tests;").next().unwrap();
    for banned in ["println!", "eprintln!", "dbg!", "panic!", "format!(", "write!(", "log::", "fs::write", "File::create", "OpenOptions",
                   ".env(", ".envs(", ".arg(", "clipboard", ".copy(", "Command::new(\"sh"] {
        let hits: Vec<&str> = code.lines().filter(|l| l.contains(banned) && !l.trim_start().starts_with("//")).collect();

        let allowed = banned == "write!(" && hits.iter().all(|l| l.contains("Digits({} of {LENGTH})"));
        assert!(hits.is_empty() || allowed, "{banned} is used: {hits:?}");
    }
}

fn card_with(launch: impl Fn(&mut Digits) + 'static) -> Card {
    let events = Loop::new();
    Card::new(Box::new(launch), events.post(|_: Out| {}))
}

fn typed(card: &mut Card, text: &str, serial: u32) {
    for c in text.bytes() {
        with_press(Some(serial), || card.update(Msg::Digit(c)));
    }
}

#[test]
fn a_made_up_key_cannot_fill_the_card() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let sent: Rc<RefCell<Vec<[u8; LENGTH]>>> = Rc::default();
    let s = sent.clone();
    let mut card = card_with(move |d| {
        s.borrow_mut().push(d.raw());
        d.wipe();
    });
    with_named(true, Some(7), || {
        typed(&mut card, SAMPLE, 8);
        assert_eq!(card.held(), 0, "a made-up key filled the card");
        typed(&mut card, "73", 7);
        assert_eq!(card.held(), 2);

        with_press(Some(8), || card.update(Msg::Back));
        with_press(Some(8), || card.update(Msg::Other));
        assert_eq!((card.held(), card.said.clone()), (2, None));
        typed(&mut card, "91", 7);
        with_press(Some(8), || card.update(Msg::Send));
        assert!(sent.borrow().is_empty() && !card.busy, "a made-up press sent the PIN");
        with_press(Some(7), || card.update(Msg::Send));
        assert_eq!(*sent.borrow(), [*b"7391"]);
        assert!(card.busy);
        assert_eq!(card.held(), 0, "the card kept the digits after sending them");
    });
}

#[test]
fn a_made_up_key_through_the_window_is_refused_and_says_why() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let card = &mut card_with(|d| d.wipe());
    let mut ui = Ui::new(card.view(), Theme::chosen());
    let mut pix = tiny_skia::Pixmap::new(340, 520).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let key = |text: &str| Input::Key { sym: Keysym::NoSymbol, text: Some(text.into()), mods: Default::default(), repeat: false };
    with_named(true, Some(7), || {
        for (serial, want) in [(8, 0), (7, 1)] {
            ui.input(&Input::Focus(true), std::time::Instant::now());
            let messages = with_press(Some(serial), || ui.input(&key("5"), std::time::Instant::now()).messages);
            for m in messages {
                with_press(Some(serial), || card.update(m));
            }
            assert_eq!(card.held(), want, "press {serial}");
        }
    });
}

#[test]
fn the_keys_mean_digits_delete_and_send() {
    let key = |sym: Keysym, text: Option<&str>, ctrl: bool| {
        key_of(&Input::Key { sym, text: text.map(String::from), mods: crate::toolkit::input::Mods { ctrl, ..Default::default() }, repeat: false })
    };
    assert_eq!(key(Keysym::NoSymbol, Some("5"), false), Some(Msg::Digit(b'5')));
    assert_eq!(key(Keysym::KP_5, Some("5"), false), Some(Msg::Digit(b'5')));
    assert_eq!(key(Keysym::BackSpace, None, false), Some(Msg::Back));
    assert_eq!(key(Keysym::Return, Some("\r"), false), Some(Msg::Send));
    assert_eq!(key(Keysym::KP_Enter, None, false), Some(Msg::Send));
    assert_eq!(key(Keysym::NoSymbol, Some("a"), false), Some(Msg::Other));
    assert_eq!(key(Keysym::NoSymbol, Some("55"), false), Some(Msg::Other));
    assert_eq!(key(Keysym::NoSymbol, Some("5"), true), None);
    assert_eq!(key(Keysym::Escape, Some("\u{1b}"), false), None);
    assert_eq!(key(Keysym::Tab, Some("\t"), false), None);
}

#[test]
fn a_pin_is_four_digits() {
    use std::cell::Cell;
    use std::rc::Rc;
    let sent = Rc::new(Cell::new(0));
    let s = sent.clone();
    let mut card = card_with(move |d| {
        s.set(s.get() + 1);
        d.wipe();
    });
    typed(&mut card, "73", 1);
    card.update(Msg::Send);
    assert_eq!((card.said.as_deref(), sent.get(), card.held()), (Some(four()), 0, 2));
    card.said = None;
    card.update(Msg::Other);
    assert_eq!(card.said.as_deref(), Some(four()));
    typed(&mut card, "9155", 1);
    assert_eq!(card.held(), LENGTH);
    assert_eq!(card.said, None, "the next digit did not clear the old words");
}

#[test]
fn the_card_is_wiped_on_cancel_and_on_refusal() {
    let mut card = card_with(|d| d.wipe());
    typed(&mut card, SAMPLE, 1);
    card.update(Msg::Cancel);
    assert_eq!((card.held(), card.digits.raw()), (0, [0; LENGTH]));
    typed(&mut card, SAMPLE, 1);
    card.update(Msg::Send);
    assert!(card.busy);
    card.update(Msg::Answer(Err("Wrong PIN. 4 tries left.".into())));
    assert_eq!((card.held(), card.digits.raw()), (0, [0; LENGTH]));
    assert_eq!(card.said.as_deref(), Some("Wrong PIN. 4 tries left."));
    assert!(!card.busy, "the card could not be tried again");
    typed(&mut card, SAMPLE, 1);
    assert_eq!(card.held(), LENGTH);
    card.update(Msg::Answer(Ok(())));
    assert_eq!((card.held(), card.digits.raw()), (0, [0; LENGTH]));
}

#[test]
fn the_pin_card_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    use crate::toolkit::ui::Ui;
    let mut card = card_with(|d| d.wipe());
    typed(&mut card, "73", 1);
    card.said = Some("Wrong PIN. 4 tries left.".into());
    let ui = Ui::new(card.view(), Theme::chosen());
    assert!(ui.natural_height(SIZE.0 as f32) <= SIZE.1 as f32);
    drawn::none(drawn::in_every_look("the PIN card", 12, || {
        drawn::window("pin", card.view(), crate::toolkit::theme::held(), SIZE, false);
    }));
}

const SUM: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn plugin_ini(id: &str, permissions: &str, declares: bool) -> String {
    format!("[Plugin]\nInterface=1\nId={id}\nName=Sample\nVersion=1.0\nPermissions={permissions}\n\n\
             [Ring]\nIcon=icon.svg\nLabel=Sample\nRun=bin/flow\n{}",
            if declares { "\n[PinUnlock]\nRun=bin/flow unlock\n" } else { "" })
}

fn kept(all: &Path, id: &str, ini: &str) {
    let app = all.join(id).join("app");
    std::fs::create_dir_all(app.join("bin")).unwrap();
    std::fs::write(app.join("plugin.ini"), ini).unwrap();
    std::fs::write(app.join("icon.svg"), "<svg/>").unwrap();
    std::fs::write(app.join("bin/flow"), "#!/bin/sh\n").unwrap();
    std::fs::set_permissions(app.join("bin/flow"), std::fs::Permissions::from_mode(0o755)).unwrap();
}

fn folder_for(name: &str, ini: &str) -> PathBuf {
    let dir = scratch(name);
    let app = dir.join("app");
    std::fs::create_dir_all(app.join("bin")).unwrap();
    std::fs::write(app.join("plugin.ini"), ini).unwrap();
    std::fs::write(app.join("icon.svg"), "<svg/>").unwrap();
    std::fs::write(app.join("bin/flow"), "#!/bin/sh\n").unwrap();
    std::fs::set_permissions(app.join("bin/flow"), std::fs::Permissions::from_mode(0o755)).unwrap();
    app
}

#[test]
fn only_flow_from_the_catalogue_may_declare_or_have_the_pin_command() {
    let all = scratch("gate");

    kept(&all, FLOW, &plugin_ini(FLOW, "control", true));
    assert_eq!(pin_command_in(&all, FLOW), None, "Flow with no mark");
    own_hands::mark(&all, FLOW, SUM).unwrap();
    assert_eq!(pin_command_in(&all, FLOW), None, "Flow not yet allowed control");
    crate::store::boxed::allow_at(&all.join(FLOW), PERMISSIONS, "control", true).unwrap();
    assert_eq!(pin_command_in(&all, FLOW), Some(vec!["bin/flow".to_string(), "unlock".into()]));

    crate::store::boxed::allow_at(&all.join(FLOW), PERMISSIONS, "control", false).unwrap();
    assert_eq!(pin_command_in(&all, FLOW), None);
    crate::store::boxed::allow_at(&all.join(FLOW), PERMISSIONS, "control", true).unwrap();

    kept(&all, FLOW, &plugin_ini(FLOW, "control", false));
    assert_eq!(pin_command_in(&all, FLOW), None, "Flow that declares nothing");
    kept(&all, FLOW, &plugin_ini(FLOW, "network", true));
    assert_eq!(pin_command_in(&all, FLOW), None, "Flow that does not ask for control");

    for other in ["org.example.Other".to_string(), "org.example.Flow".into(), FLOW.to_lowercase(), format!("{FLOW}2")] {
        let other = other.as_str();
        kept(&all, other, &plugin_ini(other, "control", true));
        own_hands::mark(&all, other, SUM).unwrap();
        crate::store::boxed::allow_at(&all.join(other), PERMISSIONS, "control", true).unwrap();
        assert_eq!(pin_command_in(&all, other), None, "{other} was given the PIN command");
        let problems = check(&all.join(other).join("app")).unwrap_err().join("\n");
        assert!(problems.contains("[PinUnlock] is only for Flow"), "{other}: {problems}");
    }

    let plain = folder_for("plain", &plugin_ini("org.example.Other", "control", false));
    assert!(check(&plain).is_ok());

    let broken = folder_for("broken", &plugin_ini(FLOW, "control", true).replace("Run=bin/flow unlock", "Run=/bin/sh unlock"));
    assert!(check(&broken).unwrap_err().join("\n").contains("inside the plugin's folder"));
    let missing = folder_for("missing", &plugin_ini(FLOW, "control", true).replace("Run=bin/flow unlock\n", ""));
    assert!(check(&missing).unwrap_err().join("\n").contains("needs Run"));
    let _ = std::fs::remove_dir_all(&all);
}

#[test]
fn a_computer_id_is_one_plain_word() {
    for good in ["office-1", "abc123", "a.b_c", &"a".repeat(64)] {
        assert!(good_computer(good), "{good}");
    }
    for bad in ["", "-x", ".x", "a b", "a/b", "a;b", "\u{e9}", "a\n", &"a".repeat(65), "--help", "../x"] {
        assert!(!good_computer(bad), "{bad:?}");
    }
}

#[test]
fn ring_json_may_ask_for_the_card_by_a_computer_id_alone() {
    use crate::plugins::notes::parse_ring_note;
    let note = parse_ring_note(br#"{"items":[
        {"label":"Unlock Office","unlock":"office-1"},
        {"label":"Both","unlock":"office-1","args":["goto"]},
        {"label":"Bad","unlock":"--help"},
        {"label":"Number","unlock":7},
        {"label":"Go","args":["goto","x"]}]}"#);
    let items: Vec<(&str, Option<&str>, usize)> = note.items.iter().map(|i| (i.label.as_str(), i.unlock.as_deref(), i.args.len())).collect();
    assert_eq!(items, [("Unlock Office", Some("office-1"), 0), ("Go", None, 2)]);
}
