use super::*;
use crate::store::flatpak_peek::Line;
use std::cell::RefCell;
use std::rc::Rc;

fn store() -> Store {
    crate::store::flatpak_peek::test_place(std::env::temp_dir().join(format!("trindle-fp-page-copies-{}", std::process::id())));
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    Store::new(&events, asks, None)
}

fn file(name: &str, bytes: &[u8]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-fp-page-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

const REF: &[u8] = b"[Flatpak Ref]\nName=org.example.App\nBranch=stable\nUrl=https://dl.example.com/repo/\n";

fn one() -> Vec<Line> {
    vec![Line { icon: "folder-symbolic", words: "Look at the folder ~/Documents".into() }]
}

fn install_press(s: &Store) -> Option<bool> {
    laid(s.flatpaks.first()?).presses().into_iter().find(|(tip, _, _)| tip == t!("store-fp-install")).map(|(_, off, _)| !off)
}

#[test]
fn a_dropped_reference_only_puts_up_its_page_and_installs_nothing() {
    let mut s = store();
    let (id, about) = queue_ref(&mut s, file("a.flatpakref", REF)).unwrap();
    assert_eq!((about.name.as_str(), about.host.as_str()), ("org.example.App", "dl.example.com"));
    assert_eq!(s.flatpaks.len(), 1);
    assert_eq!(s.working, 0, "no install has started");
    assert!(s.top.is_none() && s.dials.is_empty());
    assert!(showing(&s) && view(&s).is_some());
    assert_ne!(install_press(&s), Some(true), "Install waits for what it reaches to be read");
    assert!(begin(&mut s, id).is_none(), "and cannot be started before");
    assert_eq!(s.working, 0);
}

#[test]
fn what_is_installed_is_the_reference_that_was_shown_whatever_happens_to_the_file() {
    let mut s = store();
    let given = file("swap.flatpakref", b"[Flatpak Ref]\nName=org.example.App\nUrl=https://dl.example.com/repo/\nGPGKey=abc=\nTitle=x\n");
    let (id, _) = queue_ref(&mut s, given.clone()).unwrap();
    file("swap.flatpakref", b"[Flatpak Ref]\nName=org.evil.App\nUrl=https://evil.example.com/\n");
    reached(&mut s, id, None, Some(one()), Some(COMMIT.into()));
    let plan = begin(&mut s, id).unwrap();
    let from = plan.steps[0].iter().find_map(|a| a.strip_prefix("--from=")).map(PathBuf::from).unwrap();
    assert_ne!(from, given, "never the person's file");
    let copy = std::fs::read_to_string(&from).unwrap();
    assert!(copy.contains("Name=org.example.App") && copy.contains("Url=https://dl.example.com/repo/") && copy.contains("GPGKey=abc="), "{copy}");
    assert!(!copy.contains("evil") && !copy.contains("Title"), "{copy}");
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(std::fs::metadata(from.parent().unwrap()).unwrap().permissions().mode() & 0o777, 0o700);
    }
    assert!(installed(&mut s, id, &plan.name, &plan.source, true, ""));
    assert!(!from.exists() && given.exists(), "the copy goes, the person's file stays");
    let (id, _) = queue_ref(&mut s, file("swap2.flatpakref", REF)).unwrap();
    let Source::Ref(copy) = s.flatpaks[0].source.clone() else { panic!() };
    cancel(&mut s, id);
    assert!(!copy.exists(), "a cancelled page's copy goes too");
}

#[test]
fn a_second_source_a_reference_adds_is_named_on_its_page() {
    let mut s = store();
    queue_ref(&mut s, file("rr.flatpakref", b"[Flatpak Ref]\nName=org.example.App\nUrl=https://dl.example.com/repo/\nRuntimeRepo=https://other.example.org/x.flatpakrepo\n")).unwrap();
    let words: Vec<String> = laid(&s.flatpaks[0]).said();
    assert!(words.iter().any(|w| *w == t!("store-fp-adds-source", host = "other.example.org")), "{words:?}");
    let mut plain = store();
    queue_ref(&mut plain, file("rr2.flatpakref", REF)).unwrap();
    assert!(!laid(&plain.flatpaks[0]).said().iter().any(|w| *w == t!("store-fp-adds-source", host = "dl.example.com")));
}

#[test]
fn install_starts_once_after_the_list_is_read_and_with_the_right_command() {
    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("b.flatpakref", REF)).unwrap();
    reached(&mut s, id, None, Some(one()), None);
    assert_eq!(install_press(&s), Some(true));
    let plan = begin(&mut s, id).unwrap();
    let argv = &plan.steps[0];
    assert_eq!(plan.name, "org.example.App");
    assert!(argv.iter().any(|a| a.starts_with("--from=") && a.ends_with("copy.flatpakref")) && argv.iter().any(|a| a == "install"));
    assert_eq!(plan.steps.len(), 1, "no commit was shown, so none is held to");
    assert_eq!(s.working, 1);
    assert!(begin(&mut s, id).is_none(), "a second press starts nothing");
    assert_eq!(s.working, 1);
}

#[test]
fn a_list_that_could_not_be_read_still_waits_for_install() {
    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("c.flatpakref", REF)).unwrap();
    reached(&mut s, id, None, None, None);
    assert!(matches!(s.flatpaks[0].reach, Reach::Unread));
    assert_eq!(s.working, 0);
    assert_eq!(install_press(&s), Some(true));
    let words: Vec<String> = laid(&s.flatpaks[0]).said();
    assert!(words.iter().any(|w| w == t!("store-fp-unread")), "{words:?}");
    assert!(!words.iter().any(|w| w == t!("store-fp-unread-again")));
}

#[test]
fn an_unread_list_needs_install_pressed_a_second_time() {
    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("c2.flatpakref", REF)).unwrap();
    reached(&mut s, id, None, None, None);
    assert!(begin(&mut s, id).is_none(), "the first press only warns");
    assert_eq!(s.working, 0);
    assert!(matches!(s.flatpaks[0].step, Step::Waiting));
    let words: Vec<String> = laid(&s.flatpaks[0]).said();
    assert!(words.iter().any(|w| w == t!("store-fp-unread-again")), "{words:?}");
    assert!(begin(&mut s, id).is_some(), "the second goes ahead");
    assert_eq!(s.working, 1);
    let mut read = store();
    let (id, _) = queue_ref(&mut read, file("c3.flatpakref", REF)).unwrap();
    reached(&mut read, id, None, Some(one()), None);
    assert!(begin(&mut read, id).is_some(), "a list that was read needs one press");
}

#[test]
fn a_page_for_something_that_is_not_a_reference_is_never_put_up() {
    let mut s = store();
    assert!(queue_ref(&mut s, file("d.flatpakref", b"echo hi")).is_none());
    assert!(queue_ref(&mut s, PathBuf::from("/nonexistent/x.flatpakref")).is_none());
    assert!(s.flatpaks.is_empty());
    assert!(s.top.as_ref().is_some_and(|(w, tone)| w == t!("store-add-bad-ref") && *tone == Tone::Problem));
}

#[test]
fn cancelling_or_leaving_removes_a_fetched_bundle_but_never_a_running_install() {
    let mut s = store();
    let (a, b) = (file("e.flatpak", b"x"), file("f.flatpak", b"x"));
    let first = queue_bundle(&mut s, Source::Bundle(a.clone()), "Thing.flatpak").unwrap();
    let second = queue_bundle(&mut s, Source::Bundle(b.clone()), "Other.flatpak").unwrap();
    assert_eq!(s.flatpaks[0].name, "Thing");
    cancel(&mut s, first);
    assert!(!a.exists() && s.flatpaks.len() == 1);
    reached(&mut s, second, Some("org.example.Other".into()), Some(Vec::new()), None);
    assert_eq!(s.flatpaks[0].name, "org.example.Other");
    let plan = begin(&mut s, second).unwrap();
    let argv = &plan.steps[0];
    assert_eq!(&argv[argv.len() - 2..], ["--bundle".to_string(), b.display().to_string()]);
    cancel(&mut s, second);
    forget(&mut s);
    assert!(b.exists(), "an install under way keeps its file");
    assert!(s.flatpaks.len() == 1 || s.flatpaks.is_empty());
    let source = Source::Bundle(b.clone());
    assert!(installed(&mut s, second, "Other", &source, true, ""));
    assert!(!b.exists() && s.flatpaks.is_empty());
    assert!(s.top.as_ref().is_some_and(|(w, _)| *w == t!("store-add-installed-ref", name = "Other")));
}

#[test]
fn a_failed_install_says_why_in_red_on_the_page_and_keeps_it() {
    let mut s = store();
    let b = file("g.flatpak", b"x");
    let id = queue_bundle(&mut s, Source::Bundle(b.clone()), "g.flatpak").unwrap();
    reached(&mut s, id, None, Some(one()), None);
    let plan = begin(&mut s, id).unwrap();
    assert!(!installed(&mut s, id, &plan.name, &plan.source, false, "warning\nerror: No space left\n"));
    assert!(matches!(&s.flatpaks[0].step, Step::Failed(w) if w.contains("No space left")));
    assert!(b.exists(), "kept so Install can be pressed again");
    assert_eq!(s.working, 0);
    assert!(begin(&mut s, id).is_some(), "and tried again");
    s.flatpaks.clear();
    let (name, source) = ("g".to_string(), Source::Bundle(b.clone()));
    assert!(!installed(&mut s, id, &name, &source, false, ""));
    assert!(!b.exists() && s.top.as_ref().is_some_and(|(_, tone)| *tone == Tone::Problem), "the person went elsewhere: said under the search box");
}

#[test]
fn a_bundles_shown_name_is_cut_and_cleaned() {
    let mut s = store();
    queue_bundle(&mut s, Source::Bundle(PathBuf::from("/c/x.flatpak")), &format!("a\u{202e}\nb{}.flatpak", "z".repeat(500))).unwrap();
    let name = &s.flatpaks[0].name;
    assert!(name.chars().count() <= 80 && !name.contains('\n'));
}

#[test]
fn an_open_app_page_is_not_covered() {
    let mut s = store();
    queue_ref(&mut s, file("h.flatpakref", REF)).unwrap();
    s.image = Some(crate::store::github::Found::default());
    assert!(!showing(&s) && view(&s).is_none());
}

const COMMIT: &str = "00516a24d410ad62ed3e955fa5deb5af221ab6b181ed5549103f3670d10d2fca";

#[test]
fn a_reference_is_held_to_the_commit_that_was_shown_and_taken_out_if_that_fails() {
    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("i.flatpakref", REF)).unwrap();
    reached(&mut s, id, None, Some(one()), Some(COMMIT.into()));
    let plan = begin(&mut s, id).unwrap();
    assert_eq!(plan.steps.len(), 2);
    assert!(plan.steps[0].iter().any(|a| a == "install"));
    let pin = &plan.steps[1];
    assert!(pin.iter().any(|a| a == "update") && pin.iter().any(|a| *a == format!("--commit={COMMIT}")));
    assert_eq!(pin.last().unwrap(), "org.example.App//stable");
    assert!(plan.undo.iter().any(|a| a == "uninstall") && plan.undo.contains(&"--user".to_string()));
    assert_eq!(plan.undo.last().unwrap(), "org.example.App//stable", "that branch and no other");
}

#[test]
fn a_bundle_needs_no_pin_and_an_opened_one_is_never_deleted() {
    let mut s = store();
    let mine = file("j.flatpak", b"x");
    let id = queue_bundle(&mut s, Source::Mine(mine.clone()), "j.flatpak").unwrap();
    reached(&mut s, id, None, Some(one()), None);
    assert!(begin(&mut s, id).is_none(), "the original is never installed from");
    s.flatpaks[0].source = Source::Copy(file("j-copy.flatpak", b"x"));
    let plan = begin(&mut s, id).unwrap();
    assert_eq!((plan.steps.len(), plan.undo.len()), (1, 0));
    assert!(installed(&mut s, id, &plan.name, &plan.source, true, ""));
    assert!(mine.exists(), "the person's own file stays");
    let again = queue_bundle(&mut s, Source::Mine(mine.clone()), "j.flatpak").unwrap();
    cancel(&mut s, again);
    assert!(mine.exists());
}

#[test]
fn a_program_already_installed_gets_no_second_install_but_its_page() {
    let mut s = store();
    s.installed.insert("org.example.App".into(), ("system".into(), "1.0".into()));
    offer_ref(&mut s, file("k.flatpakref", REF));
    assert!(s.flatpaks.is_empty(), "no page with Install is put up");
    assert_eq!(s.working, 0);
    assert!(s.top.as_ref().is_some_and(|(w, tone)| *w == t!("store-fp-have", name = "org.example.App") && *tone == Tone::Plain));
    assert!(s.view == crate::store::View::Installed, "Flathub does not know it, so Installed shows it, with its Update if one waits");

    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("k2.flatpakref", REF)).unwrap();
    Rc::make_mut(&mut s.apps).insert("org.example.App".into(), crate::store::catalogue::App { id: "org.example.App".into(), ..Default::default() });
    have(&mut s, id, "org.example.App");
    assert!(s.flatpaks.is_empty());
    assert_eq!(s.detail.as_ref().map(|a| a.id.as_str()), Some("org.example.App"), "Apps' usual page for it");
}

#[test]
fn the_page_that_found_it_installed_goes_and_the_others_wait() {
    let mut s = store();
    let (a, _) = queue_ref(&mut s, file("k3.flatpakref", REF)).unwrap();
    let (b, _) = queue_ref(&mut s, file("k4.flatpakref", b"[Flatpak Ref]\nName=org.example.Other\nUrl=https://dl.example.com/repo/\n")).unwrap();
    have(&mut s, a, "org.example.App");
    assert_eq!(s.flatpaks.iter().map(|p| p.id).collect::<Vec<_>>(), [b]);
    have(&mut s, 9999, "org.example.App");
    assert_eq!(s.flatpaks.len(), 1, "a page that is gone is not touched");
}

#[test]
fn a_bundle_found_installed_gets_the_same_and_its_copy_goes() {
    let mut s = store();
    let copy = file("l.flatpak", b"x");
    let id = queue_bundle(&mut s, Source::Mine(PathBuf::from("/c/mine.flatpak")), "mine.flatpak").unwrap();
    heard_bundle(&mut s, id, Seen::Bundle { copy: Some(Ok(copy.clone())), found: Some(("org.example.Bundled".into(), Some(Vec::new()))), have: true });
    assert!(s.flatpaks.is_empty() && !copy.exists());
    assert!(s.top.as_ref().is_some_and(|(w, _)| *w == t!("store-fp-have", name = "org.example.Bundled")));
}

fn ran<'a>(log: &'a RefCell<Vec<String>>, fail: &'static [&'static str]) -> impl FnMut(&[String]) -> (bool, String) + 'a {
    move |argv| {
        let verb = argv.iter().find(|a| ["install", "update", "uninstall"].contains(&a.as_str())).cloned().unwrap_or_default();
        log.borrow_mut().push(verb.clone());
        match fail.contains(&verb.as_str()) {
            true => (false, format!("first\nerror: {verb} failed")),
            false => (true, String::new()),
        }
    }
}

fn steps3() -> (Vec<Vec<String>>, Vec<String>) {
    let v = |w: &[&str]| w.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    (vec![v(&["flatpak", "install"]), v(&["flatpak", "update"])], crate::common::removal::flatpak_uninstall(None, "org.example.App", false))
}

#[test]
fn a_failing_second_step_takes_the_program_out_and_says_so_if_that_fails_too() {
    let (steps, undo) = steps3();
    let log = RefCell::new(Vec::new());
    assert_eq!(steps_with(&steps, &undo, &mut ran(&log, &["update"])), (false, "first\nerror: update failed".to_string()));
    assert_eq!(*log.borrow(), ["install", "update", "uninstall"]);
    let log = RefCell::new(Vec::new());
    let (ok, why) = steps_with(&steps, &undo, &mut ran(&log, &["update", "uninstall"]));
    assert!(!ok && why.starts_with("error: update failed") && why.ends_with(t!("store-fp-undo-failed")), "{why}");
    assert_eq!(*log.borrow(), ["install", "update", "uninstall"]);
}

#[test]
fn a_failing_first_step_runs_nothing_else_and_takes_nothing_out() {
    let (steps, undo) = steps3();
    let log = RefCell::new(Vec::new());
    assert!(!steps_with(&steps, &undo, &mut ran(&log, &["install"])).0);
    assert_eq!(*log.borrow(), ["install"], "no pin, and no undo of a program this run never put in");
    let log = RefCell::new(Vec::new());
    assert_eq!(steps_with(&steps, &undo, &mut ran(&log, &[])), (true, String::new()));
    assert_eq!(*log.borrow(), ["install", "update"]);
}

#[test]
fn a_step_that_hangs_is_ended_at_its_time() {
    let argv = vec!["sleep".to_string(), "30".to_string()];
    assert_eq!(&timed(&argv, STEP_SECONDS)[..4], ["timeout", "-k", "3", STEP_SECONDS]);
    let started = std::time::Instant::now();
    let (ok, _) = run_steps(&[argv], &[], "1");
    assert!(!ok && started.elapsed().as_secs() < 15);
}

#[test]
fn flatpaks_last_words_are_cut_and_cleaned() {
    assert_eq!(why_of("warning\nerror: No space left\n"), "error: No space left");
    assert_eq!(why_of("x\n\u{1b}[31mred\u{202e}\u{7}\rok"), "[31mredok");
    let long = why_of(&"z".repeat(5000));
    assert_eq!(long.chars().count(), WHY_MOST + 1);
    assert!(long.ends_with('…'));
    assert_eq!(why_of(""), "");
    let mut s = store();
    let id = queue_bundle(&mut s, Source::Bundle(file("m.flatpak", b"x")), "m.flatpak").unwrap();
    reached(&mut s, id, None, Some(one()), None);
    installed(&mut s, id, "m", &Source::Ref(PathBuf::new()), false, &format!("{}\u{1b}[2J", "q".repeat(900)));
    let Step::Failed(w) = &s.flatpaks[0].step else { panic!() };
    assert!(w.chars().count() < 300 && !w.chars().any(char::is_control), "{w}");
}

#[test]
fn after_going_elsewhere_an_installing_page_does_not_cover_the_grid() {
    let mut s = store();
    let (id, _) = queue_ref(&mut s, file("n.flatpakref", REF)).unwrap();
    reached(&mut s, id, None, Some(one()), None);
    begin(&mut s, id).unwrap();
    assert!(showing(&s) && view(&s).is_some());
    forget(&mut s);
    assert_eq!(s.flatpaks.len(), 1, "the install goes on");
    assert!(!showing(&s) && view(&s).is_none(), "the grid shows");
    let (next, _) = queue_ref(&mut s, file("n2.flatpakref", b"[Flatpak Ref]\nName=org.example.Next\nUrl=https://dl.example.com/repo/\n")).unwrap();
    assert!(showing(&s) && front(&s).map(|p| p.id) == Some(next), "a page put up later shows");
    cancel_first(&mut s);
    assert_eq!(s.flatpaks.len(), 1);
    assert!(!installed(&mut s, id, "org.example.App", &Source::Ref(PathBuf::new()), false, "error: nope"));
    assert!(s.flatpaks.is_empty(), "its end is said under the search box");
    assert!(s.top.as_ref().is_some_and(|(w, tone)| w.contains("nope") && *tone == Tone::Problem));
}

#[test]
fn escape_cannot_cancel_an_installing_page_and_two_pages_queue_in_turn() {
    let mut s = store();
    let (a, _) = queue_ref(&mut s, file("o.flatpakref", REF)).unwrap();
    let (b, _) = queue_ref(&mut s, file("o2.flatpakref", b"[Flatpak Ref]\nName=org.example.Two\nUrl=https://dl.example.com/repo/\n")).unwrap();
    assert_eq!(front(&s).map(|p| p.id), Some(a));
    reached(&mut s, a, None, Some(one()), None);
    begin(&mut s, a).unwrap();
    cancel_first(&mut s);
    assert_eq!(s.flatpaks.len(), 2, "an install under way stays");
    assert_eq!(front(&s).map(|p| p.id), Some(a));
    assert!(laid(&s.flatpaks[0]).presses().iter().all(|(tip, off, _)| tip != t!("store-fp-cancel") || *off), "Cancel is greyed while it installs");
    s.flatpaks[0].step = Step::Waiting;
    cancel_first(&mut s);
    assert_eq!(front(&s).map(|p| p.id), Some(b), "the next one is shown");
    cancel_first(&mut s);
    assert!(s.flatpaks.is_empty() && !showing(&s));
}

#[test]
fn the_same_file_or_program_is_not_put_up_twice() {
    let mut s = store();
    let f = file("p.flatpakref", REF);
    queue_ref(&mut s, f.clone()).unwrap();
    assert!(queue_ref(&mut s, f.clone()).is_none(), "the same file");
    assert!(queue_ref(&mut s, file("p2.flatpakref", REF)).is_none(), "the same program from another file");
    assert!(s.top.is_none() && s.flatpaks.len() == 1);
    let b = file("p.flatpak", b"x");
    assert!(queue_bundle(&mut s, Source::Mine(b.clone()), "p.flatpak").is_some());
    assert!(queue_bundle(&mut s, Source::Mine(b.clone()), "p.flatpak").is_none());
    let link = b.with_file_name("p-link.flatpak");
    let _ = std::fs::remove_file(&link);
    std::os::unix::fs::symlink(&b, &link).unwrap();
    assert!(queue_bundle(&mut s, Source::Mine(link), "p-link.flatpak").is_none(), "the same file by another way in");
    let twin = queue_bundle(&mut s, Source::Bundle(file("p3.flatpak", b"x")), "p3.flatpak").unwrap();
    reached(&mut s, twin, Some("org.example.App".into()), Some(Vec::new()), None);
    assert!(s.flatpaks.iter().all(|p| p.id != twin), "a bundle of a program already waiting is let go");
}

#[test]
fn an_opened_bundle_is_installed_from_its_private_copy_and_only_the_copy_goes() {
    let mut s = store();
    let mine = file("q.flatpak", b"x");
    let copy = file("q-copy.flatpak", b"x");
    let id = queue_bundle(&mut s, Source::Mine(mine.clone()), "q.flatpak").unwrap();
    heard_bundle(&mut s, id, Seen::Bundle { copy: Some(Ok(copy.clone())), found: Some(("org.example.Q".into(), Some(one()))), have: false });
    assert!(matches!(&s.flatpaks[0].source, Source::Copy(c) if *c == copy));
    let plan = begin(&mut s, id).unwrap();
    assert_eq!(plan.steps[0].last().unwrap(), &copy.display().to_string(), "installed from the copy");
    assert!(installed(&mut s, id, "q", &plan.source, true, ""));
    assert!(!copy.exists() && mine.exists());

    let id = queue_bundle(&mut s, Source::Mine(mine.clone()), "q.flatpak").unwrap();
    heard_bundle(&mut s, id, Seen::Bundle { copy: Some(Err(())), found: None, have: false });
    assert!(begin(&mut s, id).is_none() && begin(&mut s, id).is_none(), "never from the original");
    assert!(matches!(&s.flatpaks[0].step, Step::Failed(w) if w.contains(t!("store-fp-copy-failed"))));
    cancel(&mut s, id);
    assert!(mine.exists());

    let gone = file("q-gone.flatpak", b"x");
    heard_bundle(&mut s, 12345, Seen::Bundle { copy: Some(Ok(gone.clone())), found: None, have: false });
    assert!(!gone.exists(), "a copy for a page that was cancelled goes at once");
}
