//! Tests of the activity window.

use crate::common::info_popup_tests::{shuts_like_every_other, THE_I, UNDER_THE_I};

use super::*;
use super::procs::{Group, Kind};
use super::headline::{Moment, Mood, sentence};
use super::reading::{busy_words, effort, heat, MOST_BUBBLES, own_bubble, percent, RESTING_BELOW, running_words, SYSTEM, those_left, threads_words, tidy};
use super::stop::{stop_step, Stop};

#[test]
fn the_i_shuts_on_a_press_anywhere_outside_its_words() {
    let mut a = looked_at();
    shuts_like_every_other("Activity", UNDER_THE_I, &mut a,
        |a| { a.help = true; a.help_at = THE_I; },
        |a| a.help, |a| a.view(), |a, m| a.update(m), |a, i| a.intercept(i, false));
}

#[test]
fn the_i_leaves_the_page_where_it_was() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let mut a = looked_at();
    let mut ui = Ui::new(a.view(), Theme::new(true, "#fbc711"));
    ui.lay_at(900, 420);
    ui.to_the_ends();
    assert!(ui.places().iter().any(|p| p.0 == "activity" && p.1 > 0.0), "{:?}", ui.places());
    for on in [true, false, true, false] {
        a.help = on;
        assert_eq!(ui.moved_by(a.view()), Vec::<String>::new(), "the i {on}");
    }
}

#[test]
fn stop_is_armed_and_sent_only_by_the_persons_own_press() {
    use crate::toolkit::window::{with_named, with_press};
    assert_eq!(stop_step(false, true), Stop::Arm);
    assert_eq!(stop_step(true, true), Stop::Send);
    assert_eq!(stop_step(false, false), Stop::Refuse, "a made-up press armed Stop");
    assert_eq!(stop_step(true, false), Stop::Refuse, "a made-up press sent an armed Stop");

    let mut a = looked_at();
    with_named(true, Some(7), || with_press(Some(8), || a.stop()));
    assert!(!a.armed, "a press the compositor never named armed Stop");
    with_named(true, None, || with_press(Some(7), || a.stop()));
    assert!(!a.armed, "a press the compositor offered to name, and did not, armed Stop");

    with_named(false, None, || with_press(Some(7), || a.stop()));
    assert!(a.armed, "under a compositor that names no presses, the person's own press did not arm Stop");
    with_named(false, None, || with_press(Some(7), || a.stop()));
    assert!(!a.armed);
    with_named(true, Some(7), || with_press(Some(7), || a.stop()));
    assert!(a.armed, "the person's own press did not arm Stop");

    with_named(true, Some(7), || with_press(Some(9), || a.stop()));
    assert!(!a.armed, "a made-up press left Stop armed, or sent it");

    with_named(true, Some(7), || with_press(Some(7), || a.stop()));
    with_named(true, Some(7), || with_press(Some(7), || a.stop()));
    assert!(!a.armed);
}

#[test]
fn a_program_that_did_not_answer_is_ended_and_the_line_says_so() {
    let mut child = std::process::Command::new("sh").args(["-c", "trap '' TERM; while :; do sleep 0.2; done"]).spawn().unwrap();
    let pid = child.id();
    std::thread::sleep(Duration::from_millis(300));
    let start = procs::parse_stat(&std::fs::read_to_string(format!("/proc/{pid}/stat")).unwrap()).unwrap().start;
    let mut a = looked_at();
    a.handle(Do::Force(vec![Member { pid, start: start + 1, mine: true }], "Notes".into()));
    assert!(a.forced.is_empty() && child.try_wait().unwrap().is_none(), "a later process with the number was touched");
    a.handle(Do::Force(vec![Member { pid, start, mine: true }], "Notes".into()));
    assert!(a.forced.contains("Notes"), "{}", a.forced);
    child.wait().unwrap();
    let n = a.forces;
    a.handle(Do::Unforce(n - 1));
    assert!(!a.forced.is_empty(), "an old timer cleared a newer line");
    a.handle(Do::Unforce(n));
    assert!(a.forced.is_empty());
    a.handle(Do::Force(vec![Member { pid, start, mine: true }], "Notes".into()));
    assert!(a.forced.is_empty(), "something already gone was said to be forced");
}

fn looked_at() -> Activity {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut a = Activity::new(&events, asks);
    for busy in [0.1, 0.2, 0.15, 0.3, 0.5, 0.45, 0.6, 0.4, 0.35, 0.5, 0.7, 0.4] {
        a.rings.show(400_000, busy, 8);
    }
    a.rings.rest();
    a.looked = true;
    a.said = "Firefox is doing most of the work right now.".into();
    a.orbit.set_threads(&[0.9, 0.5, 0.2, 0.1, 0.0, 0.3, 0.6, 0.1]);
    a.orbit.set_notes(Vec::new(), "9.3 GB of memory free".into());
    let programs = [("firefox", "Firefox", 0.18, 3.0, "Working hard"), ("code", "Code", 0.09, 0.2, "Working a little"),
                    ("steam", "Steam", 0.05, 0.0, "Resting"), ("trindle-desktop", "Trindle desktop", 0.02, 0.1, "Resting")];
    let mut blobs = Vec::new();
    for (key, title, memory, threads, effort) in programs {
        blobs.push(orbit::Blob {
            key: key.into(), title: title.into(), icon: vec![crate::common::apps::generic_icon_for(key, title)], memory,
            figure: rings::size((memory * 16e9) as u64), threads, heat: heat(threads, threads / 8.0), quiet: key == "trindle-desktop",
        });
        a.order.push(key.into());
        a.infos.insert(key.into(), Info {
            title: title.into(), icon: vec![crate::common::apps::generic_icon_for(key, title)], heat: heat(threads, threads / 8.0), effort, working: threads >= RESTING_BELOW,
            work: (percent(threads / 8.0), threads_words(threads / 8.0, 8)),
            holding: (rings::size((memory * 16e9) as u64), format!("{} of the memory", percent(memory))),
            running: "2 hours".into(), parts: 3, can_stop: key != "trindle-desktop",
        });
    }
    a.orbit.set_bubbles(blobs);
    a
}

#[test]
fn the_room_and_rings_leave_no_loose_tile() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut waiting = Activity::new(&events, asks);
    waiting.rings.show(400_000, 0.4, 8);
    let fonts = crate::toolkit::text::Fonts::new();
    for a in [&waiting, &looked_at()] {
        for w in [1280.0, 900.0, 400.0] {
            let loose = crate::toolkit::widget::loose_tiles(&fonts, &a.view(), w, 820.0);
            assert!(loose.is_empty(), "at {w}: {loose:?}");
        }
    }
}

#[test]
fn the_window_has_the_house_shape() {
    let fonts = crate::toolkit::text::Fonts::new();
    let a = looked_at();
    for w in [1280.0, 1920.0] {
        let found = crate::toolkit::widget::shape::check(&fonts, &a.view(), w, 800.0);
        assert!(found.wide <= 1, "at {w}: {} cards wider than three quarters", found.wide);
        assert!(found.bars.is_empty(), "at {w}: {:?}", found.bars);
        assert!(found.long.is_empty(), "at {w}: {:?}", found.long);
        assert!(found.puzzle, "at {w}: not a puzzle");
    }
}

#[test]
fn a_program_in_the_list_picks_its_bubble() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut a = Activity::new(&events, asks);
    a.looked = true;
    a.orbit.set_bubbles(vec![orbit::Blob {
        key: "firefox".into(), title: "Firefox".into(), icon: Vec::new(), memory: 0.2, figure: "3 GB".into(),
        threads: 1.0, heat: 0.5, quiet: false,
    }]);
    assert_eq!(a.orbit.picked(), None);
    a.update(Msg::Pick("firefox".into()));
    assert_eq!(a.orbit.picked().as_deref(), Some("firefox"));
    a.update(Msg::Pick("firefox".into()));
    assert_eq!(a.orbit.picked(), None);
}

#[test]
#[ignore]
fn shoot_the_window() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-a".into()));
    let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1280x800".into());
    let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))).unwrap_or((1280, 800));
    std::fs::create_dir_all(&out).unwrap();
    let shoot = |name: &str, a: &Activity| {
        let mut ui = Ui::new(a.view(), Theme::chosen());
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);

        a.orbit.finish();
        ui = Ui::new(a.view(), Theme::chosen());
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("activity-{name}-{w}.png"))).unwrap();
    };
    let mut a = looked_at();
    shoot("full", &a);
    a.orbit.pick(Some("firefox"));
    shoot("picked", &a);
    a.orbit.pick(None);
    a.help = true;
    a.help_at = Rect { x: 170.0, y: 60.0, w: 28.0, h: 28.0 };
    shoot("help", &a);
    let events = Loop::new();
    let waiting = Activity::new(&events, events.post(|_: Do| {}));
    shoot("loading", &waiting);
    let mut failed = Activity::new(&events, events.post(|_: Do| {}));
    failed.failed = true;
    failed.rings.failed(true);
    shoot("failed", &failed);
}

#[test]
fn activity_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    let events = Loop::new();
    drawn::none(drawn::in_every_look("Activity", 20, || {
        let shoot = |state: &str, a: &Activity| {

            drawn::window(state, a.view(), Theme::for_page("activity"), (1280, 800), false);
            a.orbit.finish();
            drawn::window(state, a.view(), Theme::for_page("activity"), (1280, 800), false);
        };
        let mut a = looked_at();
        shoot("full", &a);
        a.orbit.pick(Some("firefox"));
        shoot("a program picked", &a);
        a.orbit.pick(None);
        a.help = true;
        a.help_at = Rect { x: 170.0, y: 60.0, w: 28.0, h: 28.0 };
        shoot("the i's words", &a);
        shoot("still looking", &Activity::new(&events, events.post(|_: Do| {})));
        let mut failed = Activity::new(&events, events.post(|_: Do| {}));
        failed.failed = true;
        failed.rings.failed(true);
        shoot("failed", &failed);
    }));
}

#[test]
fn threads_in_words() {
    assert_eq!(threads_words(0.19, 16), "about 3 of 16 threads");
    assert_eq!(threads_words(0.01, 16), "less than 1 of 16 threads");
    assert_eq!(threads_words(1.0, 16), "all 16 threads");
    assert_eq!(threads_words(0.06, 16), "about 1 of 16 threads");
}

#[test]
fn shares_in_words() {
    assert_eq!(percent(0.45), "45%");
    assert_eq!(percent(0.005), "1%");
    assert_eq!(percent(0.001), "under 1%");
    assert_eq!(tidy("rustc"), "Rustc");
    assert_eq!(tidy("NetworkManager"), "NetworkManager");
    assert_eq!(busy_words(0.01), "resting");
    assert_eq!(busy_words(0.8), "80% busy");
}

#[test]
fn work_and_time_in_words() {
    assert_eq!(effort(0.01, 0.0), "Resting");
    assert_eq!(effort(0.3, 0.02), "Working a little");
    assert_eq!(effort(3.0, 0.19), "Working hard");
    assert_eq!(effort(15.0, 0.95), "Working flat out");
    assert_eq!(running_words(30), "Under a minute");
    assert_eq!(running_words(60), "1 minute");
    assert_eq!(running_words(7300), "2 hours");
    assert_eq!(running_words(3 * 86_400 + 5), "3 days");
}

#[test]
fn heat_warms_with_work() {

    assert!(heat(1.0, 0.06) < 0.5);
    assert!((heat(2.0, 0.12) - 0.75).abs() < 1e-9);
    assert_eq!(heat(16.0, 1.0), 1.0);
}

fn say(busy: f64, memory: f64, busiest: &[(&str, f64)], biggest: Option<&str>) -> (Mood, String) {
    let busiest: Vec<(String, f64)> = busiest.iter().map(|(n, s)| (n.to_string(), *s)).collect();
    sentence(&Moment { busy, memory, threads: 16, busiest: &busiest, biggest })
}

#[test]
fn the_moment_in_one_sentence() {
    assert_eq!(say(0.03, 0.4, &[("Firefox", 0.01)], Some("Firefox")).1, "All calm: nothing is working hard.");
    assert_eq!(say(0.6, 0.4, &[("Firefox", 0.45), ("Steam", 0.1)], None).1, "Firefox is doing most of the work right now.");
    assert_eq!(say(0.6, 0.4, &[("Firefox", 0.25), ("Steam", 0.2)], None).1,
               "Firefox and Steam are keeping the computer busy.");
    assert_eq!(say(0.97, 0.4, &[("Steam", 0.5), ("Firefox", 0.4)], None),
               (Mood::FlatOut, "Your computer is flat out: Steam and Firefox are using all 16 threads.".into()));
    assert_eq!(say(0.99, 0.4, &[("Rustc", 0.95)], None).1, "Your computer is flat out: Rustc is using all 16 threads.");
    assert_eq!(say(0.2, 0.95, &[("Firefox", 0.1)], Some("Chrome")),
               (Mood::MemoryFull, "Memory is nearly full; Chrome is holding the most.".into()));

    assert_eq!(say(0.07, 0.4, &[("Rustc", 0.0625)], None).1,
               "Rustc is working hard, but the computer has plenty to spare.");
}

#[test]
fn the_sentence_holds_still() {
    let mut h = Headline::default();
    assert!(h.offer(Mood::Calm, "All calm.".into()));

    assert!(!h.offer(Mood::TickingOver, "Ticking over.".into()));
    assert!(!h.offer(Mood::TickingOver, "Ticking over.".into()));
    assert!(h.offer(Mood::TickingOver, "Ticking over.".into()));

    assert!(h.offer(Mood::FlatOut, "Flat out.".into()));
    assert!(h.offer(Mood::MemoryFull, "Memory is nearly full.".into()));
}

fn group(key: &str, kind: Kind, kb: u64, mine: bool) -> Group {
    Group {
        owner: procs::Owner { key: key.into(), kind, app: None, name: key.into() }, ticks: 10, kb,
        members: vec![Member { pid: 1000 + kb as u32, start: 1, mine }],
    }
}

#[test]
fn programs_past_the_last_bubble_are_counted_once_in_the_quiet_one() {
    let groups: Vec<Group> = (0..20).map(|n| group(&format!("app{n}"), Kind::App, 1000 + n, true)).collect();
    let own: Vec<&Group> = groups.iter().take(MOST_BUBBLES).collect();
    let rest = those_left(&groups, &own);
    assert_eq!(rest.len(), 20 - MOST_BUBBLES);
    let mut a = looked_at();
    a.show(procs::Look { busy: 10, all: 100, threads: 8, each: vec![0.0; 8], groups }, (16_000_000, 8_000_000));
    assert_eq!(a.infos[SYSTEM].parts, 20 - MOST_BUBBLES, "a program was counted twice");
    assert_eq!(a.infos.len(), MOST_BUBBLES + 1);
    assert!(a.members.contains_key("app19"));
}

#[test]
fn only_a_program_that_can_be_stopped_keeps_its_parts() {
    let groups = vec![group("app", Kind::App, 5000, true), group("other", Kind::Other, 5000, false), group("os", Kind::Linux, 5000, true)];
    let mut a = looked_at();
    a.show(procs::Look { busy: 10, all: 100, threads: 8, each: vec![0.0; 8], groups }, (16_000_000, 8_000_000));
    assert!(a.members.contains_key("app"));
    assert!(!a.members.contains_key("other") && !a.members.contains_key("os"));
    assert!(!a.infos["app"].working || a.infos["app"].effort != "Resting");
}

#[test]
fn a_name_from_proc_is_one_plain_line() {
    assert_eq!(tidy("evil\n\u{202e}name"), "Evil  name");
}

#[test]
fn who_has_a_bubble() {
    use super::procs::Owner;
    let group = |kind, mine| Group {
        owner: Owner { key: "k".into(), kind, app: None, name: "x".into() }, ticks: 0, kb: 0,
        members: vec![Member { pid: 1, start: 1, mine }],
    };
    assert!(own_bubble(&group(Kind::App, true), 0.0, 0.0));
    assert!(!own_bubble(&group(Kind::Trindle, true), 0.3, 4.0));
    assert!(!own_bubble(&group(Kind::Linux, false), 0.0, 0.1));
    assert!(own_bubble(&group(Kind::Linux, false), 0.0, 2.0));

    assert!(!own_bubble(&group(Kind::Other, false), 0.02, 0.0));
    assert!(own_bubble(&group(Kind::Other, false), 0.0, 0.9));
    assert!(own_bubble(&group(Kind::Other, true), 0.02, 0.0));
}
