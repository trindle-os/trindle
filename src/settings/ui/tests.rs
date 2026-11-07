//! Tests for the parts Settings pages are built from.

use std::rc::Rc;

use super::stand::{made_up_clashes, made_up_taken};
use super::*;
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card as boxed, column, entry, main_action, note, row, switch, Node};

#[test]
fn an_icon_fills_its_room_at_any_scale() {
    for scale in [1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0] {
        for size in [16.0, 24.0, 56.0] {
            let (side, whole) = icon_px(size, scale);
            let room = (size * scale).round() as u32;
            assert!(side * whole <= room && room - side * whole < whole, "{size} at {scale}: {side} x {whole} in {room}");
        }
    }
}

#[test]
fn clashes_with_the_switch_off_are_answered_one_at_a_time() {
    let mut c = made_up_clashes(&["a.wav", "b.wav", "c.wav"]);
    assert_eq!(c.more(), 2);
    let first = c.answer();
    assert_eq!(first.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(), ["a.wav"]);
    assert_eq!(c.first().unwrap().name, "b.wav");
    assert_eq!(c.more(), 1);
}

#[test]
fn clashes_with_the_switch_on_answer_the_rest_of_the_batch_and_it_goes_off_again() {
    let mut c = made_up_clashes(&["a.wav", "b.wav", "c.wav"]);
    c.add_batch([made_up_taken("later.wav")]);
    c.set_same(true);
    assert!(c.same());
    let covered = c.answer();
    assert_eq!(covered.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(), ["a.wav", "b.wav", "c.wav"]);
    assert_eq!(c.first().unwrap().name, "later.wav");
    assert!(!c.same());
    c.set_same(true);
    assert!(!c.same(), "the switch came on with nothing more to cover");
    assert_eq!(c.answer().len(), 1);
    assert!(c.is_empty());
}

#[test]
fn the_question_is_one_height_with_or_without_the_switch() {
    let k = kit();
    let height = |names: &[&str]| {
        let c = made_up_clashes(names);
        let rows = replace_rows(&k, "k", &c, Vec::new(), Vec::new(), nothing(), nothing(), |_| nothing());
        let ui = crate::toolkit::ui::Ui::new(boxed(column(rows).gap(ROW_GAP)), crate::toolkit::theme::Theme::new(false, "#fbc711"));
        ui.natural_height(700.0)
    };
    let (alone, with_more, with_many) = (height(&["a.wav"]), height(&["a.wav", "b.wav"]), height(&["a.wav", "b.wav", "c.wav", "d.wav"]));
    assert!(alone > 0.0);
    assert_eq!(alone, with_more, "the question changed height when the switch came");
    assert_eq!(alone, with_many);
}

#[test]
fn every_page_that_asks_about_a_name_taken_uses_the_one_question() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/settings");
    let mut pages: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut stack = vec![dir.clone()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            if p.extension().is_none_or(|x| x != "rs") || p.ends_with("ui.rs") || p.starts_with(dir.join("ui")) || p.ends_with("tests.rs") {
                continue;
            }
            let rel = p.strip_prefix(&dir).unwrap();
            let name = match rel.components().count() {
                3 => format!("{}.rs", rel.parent().unwrap().display()),
                _ => rel.to_string_lossy().to_string(),
            };
            let text: &'static str = Box::leak(std::fs::read_to_string(&p).unwrap().into_boxed_str());
            let code = crate::common::source::code(text);
            let code = code.split("\n#[cfg(test)]").next().unwrap();
            pages.entry(name).or_default().push_str(code);
        }
    }
    let mut asking = Vec::new();
    let mut every = Vec::new();
    for (name, code) in pages {
        assert!(!code.contains("Vec<Taken>") && !code.contains("Vec<crate::common::copies::Taken>"), "{name} keeps clashes in a list of its own");
        assert!(!code.contains("is already there"), "{name} asks in words of its own");
        if code.contains("Put::Taken") || code.contains("packs::add_fonts") {
            every.push(name.clone());
        }
        if code.contains("replace_rows(") {
            assert!(code.contains("Clashes"), "{name} asks without the one list");
            assert!(code.contains("add_batch(") && code.contains("set_same("), "{name} does not give the switch");
            asking.push(name);
        }
    }
    asking.sort();
    every.sort();
    assert_eq!(asking, ["panels/appearance.rs", "panels/sounds.rs"]);
    assert_eq!(every, asking, "a page adds things that may meet a name taken and does not ask through the one question");
}

fn kit() -> Kit {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    Kit::new(events, asks, None, None)
}

fn nothing() -> Msg {
    Msg(Rc::new(|_| {}))
}

fn glowing(n: &Node<Msg>, out: &mut Vec<&'static str>) {
    if n.glow > 0 {
        out.push(match () {
            () if n.holds_box() => "box",
            () if n.only_buttons() => "button",
            () => "other",
        });
    }
    for item in n.items() {
        glowing(item, out);
    }
}

fn after(kit: &mut Kit, words: &str, ok: bool, make: impl Fn() -> Line) -> (Vec<&'static str>, Option<(String, bool)>) {
    let mut before = Vec::new();
    glowing(&make().key("line").node(kit), &mut before);
    assert!(before.is_empty(), "nothing glows before anything is done");
    kit.say("line", words, ok, false);
    let mut out = Vec::new();
    glowing(&make().key("line").node(kit), &mut out);
    (out, kit.said("line").map(|(w, red)| (w.to_string(), red)))
}

#[test]
fn a_press_that_worked_on_a_line_of_buttons_glows_them_and_says_nothing() {
    let press = || button("list-add-symbolic", "Do it").on_press(nothing());
    assert_eq!(after(&mut kit(), "Saved", true, || line("One").control(press())), (vec!["button"], None));
    assert_eq!(after(&mut kit(), "Sent", true, || line("Three").controls(vec![press(), press(), press()])).0.len(), 3);
    assert_eq!(after(&mut kit(), "Done", true, || line("Under").below(row(vec![note("words"), press()]))), (vec!["button"], None));
    assert_eq!(after(&mut kit(), "Saved", true, || line("Pill").controls(vec![press(), main_action("go-next-symbolic", "Keep")])).0,
               vec!["button"]);
}

#[test]
fn a_line_with_a_typing_box_glows_the_box_alone_wherever_it_sits() {
    let typing = || entry("John Smith", "Name", |_| nothing());
    let press = || button("document-save-symbolic", "Keep").on_press(nothing());
    assert_eq!(after(&mut kit(), "Saved", true, || line("Beside").controls(vec![typing(), press()])), (vec!["box"], None));
    assert_eq!(after(&mut kit(), "Set", true, || line("Under").below(row(vec![typing(), typing(), press()]))).0, vec!["box", "box"]);
}

#[test]
fn a_switch_or_a_choice_shows_its_own_answer_so_nothing_beside_it_glows() {
    let press = || button("help-about-symbolic", "More").on_press(nothing());
    let flip = || switch(true, "On", |_| nothing());
    assert_eq!(after(&mut kit(), "Saved", true, || line("Switch").controls(vec![flip(), press()])), (vec![], None));
    let pick = || crate::toolkit::widget::one_of(vec!["One".into(), "Two".into()], Some(0), "Pick", |_| nothing());
    assert_eq!(after(&mut kit(), "Saved", true, || line("Choice").controls(vec![pick(), press()])), (vec![], None));
}

#[test]
fn a_failure_is_red_words_under_the_line_and_never_a_glow() {
    let press = || button("list-add-symbolic", "Do it").on_press(nothing());
    let typing = || entry("John Smith", "Name", |_| nothing());
    let why = Some(("It could not be done.".to_string(), true));
    assert_eq!(after(&mut kit(), "It could not be done.", false, || line("Button").control(press())), (vec![], why.clone()));
    assert_eq!(after(&mut kit(), "It could not be done.", false, || line("Box").control(typing())), (vec![], why));
    let mut k = kit();
    k.done("line", Some("Refused."), "Saved");
    assert_eq!(k.said("line"), Some(("Refused.", true)));
    assert_eq!(k.glow("line"), 0);
}

struct Lamp {
    on: bool,
}

impl Page for Lamp {
    fn id(&self) -> &'static str {
        "lamp"
    }
    fn title(&self) -> &'static str {
        "Lamp"
    }
    fn icon(&self) -> &'static str {
        "dialog-information-symbolic"
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        page(kit, "Lamp", "", vec![line("Lamp").key("lamp").control(switch(self.on, "Lamp", |_| nothing())).node(kit)])
    }
}

impl Lamp {
    fn switch(&mut self, kit: &Kit, on: bool) -> impl FnOnce(Result<(), Failed>) + 'static {
        self.on = on;
        kit.settles::<Lamp, (), Failed>("lamp", "The lamp could not be switched.", move |me, _, done| {
            if done.is_none() {
                me.on = !on;
            }
        })
    }
}

use crate::toolkit::bus::Failed;

#[test]
fn an_action_that_worked_glows_and_one_that_failed_says_why_in_red_and_puts_its_control_back() {
    let mut s = stand::Stand::new(Box::new(Lamp { on: false }));
    let answer = s.page::<Lamp, _>(|lamp, k| lamp.switch(k, true));
    answer(Ok(()));
    s.turn();
    assert!(s.glows("lamp"));
    assert_eq!(s.red("lamp"), None);
    assert!(s.page::<Lamp, _>(|lamp, _| lamp.on));
    let answer = s.page::<Lamp, _>(|lamp, k| lamp.switch(k, false));
    assert!(!s.page::<Lamp, _>(|lamp, _| lamp.on), "it looks switched while the answer is awaited");
    answer(Err(Failed::new("org.example.Error.Odd", "0x2f: something went wrong")));
    s.turn();
    assert_eq!(s.red("lamp").as_deref(), Some("The lamp could not be switched."));
    assert!(!s.glows("lamp"));
    assert!(s.page::<Lamp, _>(|lamp, _| lamp.on), "a switch that failed does not stay looking switched");
    let answer = s.page::<Lamp, _>(|lamp, k| lamp.switch(k, false));
    answer(Err(Failed::new("org.freedesktop.DBus.Error.ServiceUnknown", "The name is not activatable")));
    s.turn();
    assert_eq!(s.red("lamp").as_deref(),
               Some("The lamp could not be switched. The part of the system that does this is not running."));
}

#[test]
fn a_failure_is_never_an_error_code_alone() {
    let could_not = "It could not be saved.";
    for (raw, cause) in [
        ("No space left on device (os error 28)", t!("settings-common-why-full")),
        ("Read-only file system (os error 30)", t!("settings-common-why-read-only")),
        ("org.freedesktop.DBus.Error.NameHasNoOwner", t!("settings-common-why-not-running")),
        ("org.freedesktop.DBus.Error.NoReply Did not receive a reply", t!("settings-common-why-slow")),
        ("org.freedesktop.DBus.Error.InteractiveAuthorizationRequired", t!("settings-common-why-password")),
        ("Permission denied (os error 13)", t!("settings-common-why-not-allowed")),
        ("No such file or directory (os error 2)", t!("settings-common-why-missing")),
    ] {
        let words = why(raw, could_not);
        assert_eq!(words, format!("{could_not} {cause}"));
        assert!(!words.contains("os error") && !words.contains("org."), "{words}");
    }
    assert_eq!(why("GDBus.Error:org.example.Odd: code 17", could_not), could_not);
    assert_eq!(why("", could_not), could_not);
    assert_eq!(std::io::Error::from_raw_os_error(28).words(could_not), "It could not be saved. The disk is full.");
    assert_eq!(Raw("exit status 1".into()).words(could_not), could_not);
    assert_eq!("Pair a phone first.".to_string().words(could_not), "Pair a phone first.", "Trindle's own words are said as they are");
    assert_eq!(String::new().words(could_not), could_not);
    assert_eq!(().words(could_not), could_not);
    assert!(ran(true, "noise").is_ok());
    assert_eq!(ran(false, "Permission denied").err().map(|e| e.words(could_not)).as_deref(),
               Some("It could not be saved. This account is not allowed to."));
    assert_eq!(asked(false, "Request dismissed").err().as_deref(), Some("Not done: no administrator's password was given."));
    assert!(kept(true).is_ok() && kept(false).is_err());
}

#[test]
fn something_done_over_and_over_says_only_its_failures_and_clears_them_when_it_works() {
    let mut k = kit();
    assert_eq!(k.settle_quietly("volume", ran(true, ""), "The volume could not be changed."), Some(()));
    assert_eq!((k.said("volume"), k.glow("volume")), (None, 0), "nothing glows for each step of a drag");
    assert_eq!(k.settle_quietly("volume", ran(false, "broken pipe"), "The volume could not be changed."), None);
    assert_eq!(k.said("volume"), Some(("The volume could not be changed.", true)));
    k.settle_quietly("volume", ran(true, ""), "The volume could not be changed.");
    assert_eq!(k.said("volume"), None, "the next one that works clears the red words");
}

#[test]
fn a_press_never_does_nothing_when_what_it_needs_is_not_there() {
    let mut k = kit();
    assert!(k.system_for("line").is_none());
    assert_eq!(k.said("line"), Some((no_system_bus(), true)));
    assert!(k.session_for("other").is_none());
    assert_eq!(k.said("other"), Some((no_session_bus(), true)));
    assert!(!k.started("open", false, no_browser()));
    assert_eq!(k.said("open"), Some((no_browser(), true)));
    assert!(k.started("fine", true, no_browser()));
    assert_eq!(k.said("fine"), None);
    for words in [no_system_bus(), no_session_bus(), no_browser(), no_chooser(), not_saved()] {
        assert!(!words.contains('\u{2014}') && words.ends_with('.'), "{words}");
    }
}
