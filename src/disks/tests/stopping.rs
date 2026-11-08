//! Tests: stopping.

#[cfg(test)]
use super::*;

#[test]
fn a_password_is_not_taken_back_in_the_middle_of_a_change() {
    let mut rig = Rig::new();
    rig.grants.set(true);
    let row = format_key("/d/usb", "/b/Photos", false);

    rig.down_at.set(Some("Format"));
    let format = Act::Format { drive: "/d/usb".into(), target: "/b/Photos".into(), whole: false };
    rig.d.act(format.clone());
    rig.d.act(format);
    let (words, _) = rig.said("/d/usb", &row).expect("words");
    assert_eq!(words, plan::standing_words("Formatted"));

    rig.down_at.set(None);
    rig.polkit_down.set(false);
    rig.hold_at.set(Some("Format"));
    rig.log.borrow_mut().clear();
    let erase = Act::Format { drive: "/d/usb".into(), target: "/b/sdb".into(), whole: true };
    rig.d.act(erase.clone());
    rig.d.act(erase);
    let held = rig.held.clone();
    assert!(rig.until(move |_| held.borrow().is_some()), "the first step is being done");
    assert!(rig.d.changing.get());
    let before = rig.log.borrow().clone();
    assert_eq!(before.iter().filter(|l| *l == "password box").count(), 1, "{before:?}");

    rig.wait(STANDING_GAP * 6);
    assert_eq!(rig.log.borrow().clone(), before, "polkit is asked nothing while a change is running");
    rig.hold_at.set(None);
    rig.let_go();
    assert_eq!(rig.said("/d/usb", ""), Some(("Formatted".to_string(), false)));
    let log = rig.log.borrow().clone();
    assert_eq!(log.iter().filter(|l| *l == "password box").count(), 1, "one change is one question: {log:?}");
    assert!(log.iter().any(|l| l == "CreatePartitionAndFormat"), "{log:?}");

    let done = plan::taken_back_words("Formatted");
    assert!(rig.until(move |d| d.told("/d/usb", &row).is_some_and(|(w, _)| w == done)), "taken back once the change ended, and said");
}

#[test]
fn a_change_stopped_part_way_says_what_was_done_and_what_was_not() {
    let erase = || Act::Format { drive: "/d/usb".into(), target: "/b/sdb".into(), whole: true };
    let delete = || Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() };
    let rename = || Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() };

    type Run<'a> = (&'a dyn Fn() -> Act, bool, String, Vec<(&'static str, &'static str, bool)>);
    let runs: Vec<Run> = vec![
        (&erase, true, String::new(), vec![("Format", "erasing the whole drive", true), ("CreatePartitionAndFormat", "making the new part", true)]),
        (&delete, false, String::new(), vec![("Unmount", "unmounting Photos", false), ("Delete", "deleting Photos", true)]),
        (&rename, false, "/b/Photos:name".to_string(),
         vec![("Unmount", "unmounting Photos", false), ("SetLabel", "renaming Photos", true), ("Mount", "mounting Photos again", false)]),
    ];
    for (start, armed, row, steps) in &runs {
        for at in 0..steps.len() {
            for error in [DISMISSED, ("org.freedesktop.UDisks2.Error.Failed", "Example reason")] {
                let mut rig = Rig::new();
                rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
                rig.fails.borrow_mut().insert(steps[at].0, error);
                if *armed {
                    rig.d.act(start());
                }
                rig.d.act(start());
                let (words, red) = rig.said("/d/usb", row).expect("words");
                let case = format!("{} stopped at {} by {}: {words}", steps.last().expect("steps").0, steps[at].0, error.1);
                assert!(red, "{case}");
                let asked: Vec<String> = rig.log.borrow().iter().filter(|l| !l.starts_with("polkit")).cloned().collect();
                let changed = steps[..at].iter().any(|s| s.2);
                let refused = error == DISMISSED;
                assert_eq!(words.contains("nothing changed"), refused && !changed, "{case}");

                let mut want: Vec<String> = steps[..=at].iter().map(|s| s.0.to_string()).collect();
                if refused && !changed && steps[..at].iter().any(|s| s.0 == "Unmount") {
                    want.push("Mount".into());
                }
                assert_eq!(asked, want, "{case}");
                if changed {
                    for (n, step) in steps.iter().enumerate() {
                        let list = if n < at { "Done: " } else if n == at && !refused { "Failed: " } else { "Not done: " };
                        let listed = words.split(". ").find(|part| part.starts_with(list)).unwrap_or("");
                        assert!(listed.contains(step.1), "{case}: {} is not under {list}", step.1);
                        assert_eq!(words.matches(step.1).count(), 1, "{case}: {} is said once", step.1);
                    }
                }
            }
        }
    }
}

#[test]
fn a_part_unmounted_for_a_change_that_stopped_is_mounted_again_or_said_to_be_unmounted() {
    let rename = || Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() };
    let delete = || Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() };
    const NEVER_BEGUN: (&str, &str) = ("org.freedesktop.DBus.Error.UnknownMethod", "No such method");
    const FAILED: (&str, &str) = ("org.freedesktop.UDisks2.Error.Failed", "Example reason");
    let not_now = "Photos was unmounted for this and is not mounted now.";

    type Case<'a> = (&'a dyn Fn() -> Act, &'a str, &'static str, (&'static str, &'static str), bool, &'a str);
    let cases: [Case; 6] = [
        (&rename, "/b/Photos:name", "SetLabel", FAILED, false, not_now),
        (&rename, "/b/Photos:name", "Mount", FAILED, false, not_now),
        (&delete, "", "Delete", FAILED, false, not_now),
        (&rename, "/b/Photos:name", "SetLabel", NEVER_BEGUN, true, "nothing changed"),
        (&delete, "", "Delete", NEVER_BEGUN, true, "nothing changed"),
        (&rename, "/b/Photos:name", "SetLabel", DISMISSED, true, "nothing changed"),
    ];
    for (start, row, at, error, again, said) in cases {
        let mut rig = Rig::new();
        rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
        rig.fails.borrow_mut().insert(at, error);
        rig.d.act(start());
        let (words, red) = rig.said("/d/usb", row).expect("words");
        let case = format!("stopped at {at} by {}: {words}", error.1);
        assert!(red && words.contains(said), "{case}");
        assert_eq!(words.matches("was unmounted for this").count(), usize::from(!again), "{case}");
        assert_eq!(words.contains("Failed: "), at == "Mount", "{case}: a list only once a step has changed the drive");
        let mounts = rig.log.borrow().iter().filter(|l| *l == "Mount").count();
        assert_eq!(mounts, usize::from(again || at == "Mount"), "{case}: {:?}", rig.log.borrow());
    }
}

#[test]
fn drives_stays_open_while_a_change_is_being_made_and_then_says_how_it_ended() {
    let mut rig = Rig::new();
    rig.grants.set(true);
    rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
    rig.hold_at.set(Some("SetLabel"));
    rig.d.act(Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let held = rig.held.clone();
    assert!(rig.until(move |_| held.borrow().is_some()), "the renaming is being done");
    assert!(!rig.d.may_close(), "not closed in the middle of a change");
    rig.d.handle(Do::Close);
    assert!(rig.d.stays);
    let (words, tone) = rig.d.told("/d/usb", "/b/Photos:name").expect("the row is being worked on");
    assert!(words.ends_with(plan::stays_open()) && matches!(tone, Tone::Plain), "{words}");

    let noted = notes(&rig.d);
    assert_eq!(noted.len(), 1, "the run is noted while it is made");
    let (drive, row, steps, done) = plan::read_noted(&noted[0]).expect("a note that reads");
    assert_eq!((drive.as_str(), row.as_str(), done), ("/d/usb", "/b/Photos:name", 1));
    assert_eq!(steps.iter().map(|s| s.what.as_str()).collect::<Vec<_>>(), ["unmounting Photos", "renaming Photos", "mounting Photos again"]);
    rig.hold_at.set(None);
    rig.let_go();
    let (words, red) = rig.said("/d/usb", "/b/Photos:name").expect("how it ended");
    assert!(!red && !words.contains("stays open"), "{words}");
    assert!(notes(&rig.d).is_empty(), "a run that ended and was told leaves no note");
    assert!(!rig.d.stays);
    assert!(rig.d.may_close(), "with nothing being done it closes");
}

#[test]
fn a_change_drives_was_ended_in_the_middle_of_is_told_the_next_time() {
    let mut rig = Rig::new();
    rig.grants.set(true);
    rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
    rig.hold_at.set(Some("SetLabel"));
    rig.d.act(Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let held = rig.held.clone();
    assert!(rig.until(move |_| held.borrow().is_some()), "the renaming is being done");

    let events = Loop::new();
    let mut next = Drives::new(&events, events.post(|_: Do| {}));
    next.noted = rig.d.noted.clone();
    next.tell_what_was_left();
    let (words, tone) = next.told("/d/usb", "/b/Photos:name").expect("told on the row it was made from");
    assert!(matches!(tone, Tone::Problem));
    assert_eq!(words, "Drives was closed while this change was being made. Done: unmounting Photos. Not known: renaming Photos. \
                       Not done: mounting Photos again. Photos was unmounted for this and is not mounted now.");

    assert!(notes(&next).is_empty());
    let mut third = Drives::new(&events, events.post(|_: Do| {}));
    third.noted = rig.d.noted.clone();
    third.tell_what_was_left();
    assert!(third.told("/d/usb", "/b/Photos:name").is_none());

    std::fs::write(third.noted.join("1-2-3"), "not a note").unwrap();
    third.tell_what_was_left();
    assert!(third.said.is_empty() && notes(&third).is_empty());
}

#[test]
fn a_note_of_a_run_still_being_made_elsewhere_is_left_alone() {
    let events = Loop::new();
    let mut d = Drives::new(&events, events.post(|_: Do| {}));
    std::fs::create_dir_all(&d.noted).unwrap();
    let steps = vec![plan::Did { what: "formatting Photos".into(), changes: true, mounts: None }];
    let text = plan::noted("/d/usb", "/b/Photos:format", &steps, 0);

    let other = std::os::unix::process::parent_id();
    let live = d.noted.join(format!("{other}-{}-0", steps::began(other).expect("its start is read")));

    let gone = d.noted.join(format!("{other}-{}-0", steps::began(other).unwrap() + 1));
    std::fs::write(&live, &text).unwrap();
    d.tell_what_was_left();
    assert!(d.said.is_empty() && live.exists(), "left for the Drives that is making it");
    std::fs::write(&gone, &text).unwrap();
    d.tell_what_was_left();
    assert!(d.told("/d/usb", "/b/Photos:format").is_some() && !gone.exists() && live.exists());

    std::os::unix::fs::symlink("/dev/zero", d.noted.join("7-7-7")).unwrap();
    d.tell_what_was_left();
    assert!(std::fs::symlink_metadata(d.noted.join("7-7-7")).is_err());
    let _ = std::fs::remove_dir_all(&d.noted);
}

#[test]
fn a_run_s_note_is_not_trampled_by_another_run() {
    let mut rig = Rig::new();
    rig.grants.set(true);
    rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
    rig.hold_at.set(Some("SetLabel"));
    rig.d.act(Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let held = rig.held.clone();
    assert!(rig.until(move |_| held.borrow().is_some()), "the renaming is being done");

    rig.d.act(Act::Mount { drive: "/d/usb".into(), vol: "/b/Photos".into(), on: true });
    let (words, red) = rig.said("/d/usb", "/b/Photos").expect("the mount ended");
    assert!(!red && words == "Mounted", "{words}");

    let events = Loop::new();
    let mut next = Drives::new(&events, events.post(|_: Do| {}));
    next.noted = rig.d.noted.clone();
    next.tell_what_was_left();
    let (words, tone) = next.told("/d/usb", "/b/Photos:name").expect("the renaming is told on its own row");
    assert!(matches!(tone, Tone::Problem) && words.contains("Not known: renaming Photos"), "{words}");
    assert!(next.told("/d/usb", "/b/Photos").is_none(), "the mount ended and left nothing to tell");
}

#[test]
fn a_change_that_cannot_be_started_says_so() {
    let mut rig = Rig::new();
    rig.d.texts.insert("/b/Photos:name".into(), "Holiday".into());
    rig.d.bus = None;
    rig.d.act(Act::Rename { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let (words, red) = rig.d.told("/d/usb", "/b/Photos:name").map(|(words, tone)| (words, matches!(tone, Tone::Problem))).expect("the row says why");
    assert!(red && words.contains("could not be reached") && words.contains("nothing changed"), "{words}");
    assert!(rig.d.working.is_empty() && rig.d.busy == 0, "nothing is left turning");
}

#[test]
fn a_slow_polkit_never_holds_the_window_while_a_refusal_is_put_into_words() {
    use crate::common::refused;
    let mut rig = Rig::new();
    refused::drawing_thread();
    refused::pretend(Some(true));
    refused::pretend_slow(Duration::from_millis(1500));

    let worst = Rc::new(Cell::new(Duration::ZERO));
    fn tick(lp: Loop, last: Instant, worst: Rc<Cell<Duration>>, on: Rc<Cell<bool>>) {
        let again = lp.clone();
        lp.after(Duration::from_millis(20), move || {
            worst.set(worst.get().max(last.elapsed()));
            if on.get() {
                tick(again, Instant::now(), worst, on);
            }
        });
    }
    let on = Rc::new(Cell::new(true));
    tick(rig.lp.clone(), Instant::now(), worst.clone(), on.clone());
    rig.fails.borrow_mut().insert("Delete", NOT_AUTHORIZED);
    rig.d.act(Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let (words, red) = rig.said("/d/usb", "").expect("words");
    rig.wait(Duration::from_millis(100));
    on.set(false);
    refused::pretend_slow(Duration::ZERO);
    refused::pretend(None);
    assert!(red && words.contains("nothing changed"), "{words}");
    assert!(worst.get() < Duration::from_millis(1000), "the window's thread was held for {:?}", worst.get());
}

#[test]
fn a_password_that_cannot_be_taken_back_is_said_in_red_and_tried_again() {
    let mut rig = Rig::new();
    let row = format_key("/d/usb", "/b/Photos", false);
    rig.down_at.set(Some("Format"));
    let format = Act::Format { drive: "/d/usb".into(), target: "/b/Photos".into(), whole: false };
    rig.d.act(format.clone());
    rig.d.act(format);
    let (words, red) = rig.said("/d/usb", &row).expect("words");
    assert_eq!(words, plan::standing_words("Formatted"));
    assert!(red, "never good news alone while the password still stands");
    assert!(words.starts_with("Formatted. The password just given could not be taken back"), "{words}");
    let mut asked = vec!["polkit: what leave is kept?".to_string(), "Format".to_string()];
    asked.extend(std::iter::repeat_n("polkit: no answer".to_string(), TAKE_BACK_TRIES as usize));
    assert_eq!(rig.log.borrow()[..asked.len()], asked[..], "taken back before, then tried {TAKE_BACK_TRIES} times after");
    assert!(!rig.d.changing.get(), "another change may be tried");

    rig.d.act(Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let (words, red) = rig.said("/d/usb", "").expect("words");
    assert!(red && words.starts_with(plan::not_sure()), "{words}");
    assert!(!rig.log.borrow().iter().any(|l| l == "Delete" || l == "Unmount"), "nothing was asked of UDisks");

    let (log, tries) = (rig.log.clone(), rig.log.borrow().len());
    assert!(rig.until(move |_| log.borrow().len() >= tries + 2), "it is tried again");
    let (words, tone) = rig.d.told("/d/usb", &row).expect("still said");
    assert!(matches!(tone, Tone::Problem) && words == plan::standing_words("Formatted"), "{words}");

    rig.polkit_down.set(false);
    let at = row.clone();
    let done = plan::taken_back_words("Formatted");
    assert!(rig.until(move |d| d.told("/d/usb", &at).is_some_and(|(w, _)| w == done)), "taken back in the end, and said");
    assert!(matches!(rig.d.told("/d/usb", &row), Some((_, Tone::Plain))));
    assert_eq!(rig.log.borrow().last().map(String::as_str), Some("polkit: what leave is kept?"));

    rig.down_at.set(Some("Delete"));
    rig.fails.borrow_mut().insert("Delete", ("org.freedesktop.UDisks2.Error.Failed", "Example reason"));
    rig.d.act(Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    let (words, red) = rig.said("/d/usb", "").expect("words");
    assert!(red);

    assert_eq!(words, plan::standing_words("Example reason. Photos was unmounted for this and is not mounted now."));
}

#[test]
fn a_wrong_password_is_not_a_closed_box_and_a_part_left_unmounted_is_named() {
    use crate::common::refused::{self, Refused};
    let mut rig = Rig::new();
    let delete = || Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() };
    let mut heard = Vec::new();
    for (error, polkit, kind) in [(DISMISSED, None, Refused::Cancelled), (NOT_AUTHORIZED, Some(true), Refused::Wrong),
                                  (NOT_AUTHORIZED, Some(false), Refused::NotAllowed), (NOT_AUTHORIZED, None, Refused::Unclear)] {
        refused::pretend(polkit);
        rig.fails.borrow_mut().insert("Delete", error);
        rig.log.borrow_mut().clear();
        rig.d.act(delete());
        let (words, red) = rig.said("/d/usb", "").expect("words");
        assert!(red);
        assert_eq!(words, plan::refusal_words(kind));
        assert_eq!(rig.log.borrow().clone(), vec!["polkit: what leave is kept?", "Unmount", "Delete", "Mount", "polkit: what leave is kept?"],
                   "{kind:?}: nothing changed, so it is mounted again");
        heard.push(words);
    }
    refused::pretend(None);
    assert_eq!(heard[0], plan::no_password());
    assert!(heard[1..].iter().all(|w| !w.contains("No password was given")), "{heard:?}");
    heard.sort();
    heard.dedup();
    assert_eq!(heard.len(), 4, "each in words of its own");

    rig.fails.borrow_mut().insert("Delete", DISMISSED);
    rig.fails.borrow_mut().insert("Mount", ("org.freedesktop.UDisks2.Error.Failed", "Example reason"));
    rig.d.act(delete());
    let (words, red) = rig.said("/d/usb", "").expect("words");
    assert!(red);
    assert_eq!(words, format!("{} Photos was unmounted for this and could not be mounted again (Example reason): mount it from its row.", plan::no_password()));

    rig.fails.borrow_mut().insert("Mount", ("org.freedesktop.UDisks2.Error.NotAuthorizedCanObtain", "Not authorized to perform operation"));
    rig.d.act(delete());
    let (words, _) = rig.said("/d/usb", "").expect("words");
    assert_eq!(words, format!("{} Photos was unmounted for this, and mounting it again needs a password of its own: mount it from its row.", plan::no_password()));

    rig.fails.borrow_mut().clear();
    rig.fails.borrow_mut().insert("Delete", ("org.freedesktop.UDisks2.Error.Failed", "Example reason"));
    rig.log.borrow_mut().clear();
    rig.d.act(delete());

    assert_eq!(rig.said("/d/usb", ""), Some(("Example reason. Photos was unmounted for this and is not mounted now.".to_string(), true)));
    assert!(!rig.log.borrow().iter().any(|l| l == "Mount"));
}

#[test]
fn a_folder_that_could_not_be_kept_is_said_under_the_formatted_row() {

    let mut d = sample();
    d.changing.set(true);
    d.busy = 1;
    d.carry_known(carried("/mnt/example"), "uuid-new");
    let (words, tone) = d.told("/d/usb", "/b/Photos").expect("said under the row");
    assert_eq!(words, plan::folder_not_kept("/mnt/example", plan::one_at_a_time()));
    assert!(matches!(tone, Tone::Problem) && d.busy == 0);

    let mut d = sample();
    d.busy = 1;
    d.carry_known(carried("/mnt/example"), "");
    let (words, tone) = d.told("/d/usb", "/b/Photos").expect("said under the row");
    assert_eq!(words, "Formatted, but it was not given its folder /mnt/example again. The new part's identifier could not be read. \
                       Choose the folder again from its row.");
    assert!(matches!(tone, Tone::Problem) && d.busy == 0 && d.ran.is_empty());

    let mut rig = Rig::new();
    rig.fails.borrow_mut().insert("AddConfigurationItem", DISMISSED);
    rig.d.busy = 1;
    rig.d.carry_known(carried("/mnt/example"), "uuid-new");
    assert_eq!(rig.said("/d/usb", "/b/Photos"), Some((plan::folder_not_kept("/mnt/example", plan::no_password()), true)));
    assert_eq!(rig.log.borrow().clone(), vec!["polkit: what leave is kept?", "AddConfigurationItem", "polkit: what leave is kept?"]);

    rig.fails.borrow_mut().clear();
    rig.d.said.clear();
    rig.d.busy = 1;
    rig.d.carry_known(carried("/mnt/example"), "uuid-new");
    assert_eq!(rig.said("/d/usb", "/b/Photos"), Some(("Formatted".to_string(), false)));
}

#[test]
fn no_answer_about_a_drive_is_thrown_away_unread() {
    let own = drives_code();
    let dropped = [concat!("|", "_| {}"), concat!("move |", "_| {")];
    let wrong: Vec<&str> = own.lines()
        .filter(|l| dropped.iter().any(|d| l.contains(d)))
        .filter(|l| [".method(", "take_back", "self.run(", "chain(", "put_back("].iter().any(|call| l.contains(call)))
        .collect();
    assert!(wrong.is_empty(), "{wrong:?}");
}

#[test]
fn drives_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("Drives", 300, || every_state(1280, 800, None)));
}

#[test]
#[ignore]
fn shoot_the_window() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-a".into()));
    let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1280x800".into());
    let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))).unwrap_or((1280, 800));
    std::fs::create_dir_all(&out).unwrap();
    every_state(w, h, Some(&out));
}
