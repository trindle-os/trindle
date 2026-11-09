//! Tests of the Updates page.

use super::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::store::catalogue::REMOTE;
use crate::store::look::Dot;
use crate::store::pages::job_rows::begin;
use crate::store::plugin_page;
use crate::store::reason::why_not_updated;
use crate::store::{firmware, updates, Msg, Store};
use crate::common::held::{self, Thing};
use crate::common::run;
use crate::common::stopping::Stop;
use crate::common::updating::{self, Part, Who};
use crate::toolkit::widget::{Node, Tone};
use super::check::{answered, ask_start, check_all, gave_up, lists_fetched, looked_again, start_asked, start_is_waiting};
use super::hold::{keep_held, set_held};
use super::install::{alone, fetching, flatpaks_for_all, forget_failures, install_all, job_done, table_row, update_app, update_debian, update_device, update_part, update_plugin, update_trindle, went_in};
use super::read::{bytes_from, flatpaks_heard, parse_flat_updates, parse_package_sizes};
use super::rows::{app_name, held_row_key};
use super::stopping::{stop_update, stopped};
use super::view::view;
use super::words::{debian_why, found_sentence, from_words, headline, held_not_told, held_words, hold_why, never_held, no_flathub, no_lists, no_more, not_checked, not_fetched, not_held, not_told, nothing_held, nothing_known, release_why, released, released_not_told, status, too_long};

#[test]
fn why_flatpak_could_not_be_told_is_said_with_what_it_means() {
    let said = not_told(true, "Looking for matches\u{2026}\nerror: Unable to load metadata: example reason\n\n");
    assert_eq!(said, format!("{} Flatpak said: Unable to load metadata: example reason", held_not_told()));
    let said = not_told(false, "error: example reason");
    assert_eq!(said, format!("{} Flatpak said: example reason", released_not_told()));

    assert_eq!(not_told(true, " \n"), held_not_told());
    assert_eq!(not_told(false, ""), released_not_told());
}

#[test]
fn lists_refused_as_apps_opens_are_said_and_a_closed_box_is_not() {
    use crate::common::refused::{not_done, Refused};
    let mut s = store_with_updates();
    s.upd.failed.clear();

    assert!(!lists_fetched(&mut s, Err(not_done(Refused::Cancelled)), false));
    assert!(!s.upd.failed.contains_key(SYSTEM), "a closed box nobody asked for is no failure");

    for why in [not_done(Refused::Wrong), not_done(Refused::NotAllowed), "apt-get could not reach the servers.".to_string()] {
        s.upd.failed.clear();
        assert!(!lists_fetched(&mut s, Err(why.clone()), false));
        let opened = s.upd.failed.get(SYSTEM).cloned().expect("said on the system's card");
        assert!(opened.starts_with(no_lists()) && opened.len() > no_lists().len(), "{opened}");
        s.upd.failed.clear();
        lists_fetched(&mut s, Err(why), true);
        assert_eq!(s.upd.failed.get(SYSTEM), Some(&opened), "the same words either way");
    }

    s.upd.failed.clear();
    lists_fetched(&mut s, Err(not_done(Refused::Cancelled)), true);
    assert!(s.upd.failed.contains_key(SYSTEM));

    assert!(lists_fetched(&mut s, Ok(()), false));
    assert!(!s.upd.failed.contains_key(SYSTEM));

    let fetch = crate::common::source::code(include_str!("../updates.rs")).split_once("pub fn refresh_lists_if_stale(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(fetch.contains("admin_watching(&[\"apt-refresh\"]") && !fetch.contains("pkexec"), "through the runner that says why");
    let opens = crate::common::source::code(include_str!("../reading.rs")).split_once("fn check_system(").unwrap().1.split_once("\n    }\n").unwrap().0;
    assert!(opens.contains("update_page::check::lists_fetched(s, res, false)"));
}

fn store_with_updates() -> Store {
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let mut s = Store::new(&events, asks, None);

    s.online = true;
    s.updatable = vec!["org.example.Calculator".into()];
    s.debian = vec!["bash".into(), "curl".into()];
    s.trindle_waiting = Some("9.9.9".into());
    s.firmware = vec![firmware::Firmware { id: "abc".into(), name: "Example keyboard".into(), current: "1".into(), newest: "2".into(), size: 0 }];
    s.plugins = plugin_page::tests::page_with_an_update();
    s
}

fn update_buttons(page: &Node<Msg>) -> Vec<(String, bool)> {
    page.buttons().into_iter()
        .filter(|(tip, _)| ["Install", "Update", "Check", "Restart"].iter().any(|w| tip.starts_with(w)))
        .collect()
}

fn headline_of(page: &Node<Msg>) -> String {

    page.said().into_iter().find(|w| w.ends_with('.')).unwrap_or_default()
}

#[test]
fn every_install_button_is_off_while_the_ring_installs_and_back_after() {
    let path = updating::tests::temp("apps-page");
    updating::pretend(Some(path.clone()), false);
    let mut s = store_with_updates();
    s.notice_updating();
    let free = view(&s);
    let names: Vec<String> = update_buttons(&free).into_iter().map(|(tip, _)| tip).collect();
    assert_eq!(names, ["Check again", "Install 6 updates", "Update org.example.Calculator", "Update Trindle", "Update Debian's packages",
                       "Update Example keyboard", "Update org.example.One"],
               "the page's one main action, then each thing's own button; Debian's packages have one between them");
    assert!(update_buttons(&free).iter().all(|(_, can)| *can), "all can be pressed while nothing else updates");
    assert!(headline_of(&free).starts_with("6 updates are ready."), "{}", headline_of(&free));

    let ring = updating::Lock::take(&path, Who::Ring).expect("free");
    s.notice_updating();
    assert_eq!(s.other, Some(Who::Ring), "noticed on the next tick");
    let held = view(&s);
    assert_eq!(headline_of(&held), "Updates are being installed from the ring. The buttons here come back when it has finished.");
    let buttons = update_buttons(&held);
    assert_eq!(buttons.iter().map(|(tip, _)| tip.clone()).collect::<Vec<_>>(), names, "each stays in its place with its words");
    assert!(buttons.iter().all(|(_, can)| !*can), "every one is off: {buttons:?}");

    let mut none = store_with_updates();
    (none.updatable, none.debian, none.trindle_waiting, none.firmware, none.plugins) = (Vec::new(), Vec::new(), None, Vec::new(), Default::default());
    none.notice_updating();
    assert_eq!(update_buttons(&view(&none)), [("Check for updates".to_string(), false)]);
    none.upd.restart = true;
    assert_eq!(update_buttons(&view(&none)), [("Check again".to_string(), false), ("Restart now".to_string(), false)]);

    let plugins = plugin_page::view(&s);
    let buttons = update_buttons(&plugins);
    assert_eq!(buttons.len(), 2, "{buttons:?}");
    assert!(buttons.iter().all(|(_, can)| !*can), "{buttons:?}");
    assert!(plugins.said().iter().any(|w| w.starts_with("Updates are being installed from the ring.")));

    drop(ring);
    s.notice_updating();
    assert_eq!(s.other, None);
    let after = view(&s);
    assert!(update_buttons(&after).iter().all(|(_, can)| *can));
    assert!(headline_of(&after).starts_with("6 updates are ready."));
    assert!(update_buttons(&plugin_page::view(&s)).iter().all(|(_, can)| *can));

    updating::pretend(Some(path), true);
    s.notice_updating();
    assert!(headline_of(&view(&s)).starts_with("Debian's installer is at work"));
    assert!(update_buttons(&view(&s)).iter().all(|(_, can)| !*can));
    updating::pretend(None, false);
}

#[test]
fn nothing_starts_from_any_entry_while_another_updates() {
    let path = updating::tests::temp("apps-entries");
    updating::pretend(Some(path.clone()), false);
    let ring = updating::Lock::take(&path, Who::Ring).expect("free");
    assert_eq!(updating::other(), Some(Who::Ring), "held, or the lines below would start real updates");
    let mut s = store_with_updates();

    assert_eq!(s.other, None);
    install_all(&mut s);
    assert_eq!((s.upd.jobs, s.working, s.dials.len()), (0, 0, 0), "Install all started nothing");
    assert_eq!(s.other, Some(Who::Ring), "and the page now says who has it");
    s.other = None;
    let offer = plugin_page::tests::first_offer(&s.plugins);
    plugin_page::install(&mut s, &offer);
    assert_eq!((s.working, s.dials.len()), (0, 0), "a plugin's Update started nothing");
    assert_eq!(s.other, Some(Who::Ring));
    check_all(&mut s);
    assert!(s.upd.checking.is_empty(), "nor does a check, which would fetch Debian's lists under the install");

    s.upd = std::mem::take(&mut s.upd).with_flats(&[("org.example.Calculator", true, "2.0", 0), ("org.example.Platform", false, "", 0)]);
    let part = s.upd.flats[1].key();
    let alone: [(&str, Box<dyn Fn(&mut Store)>); 6] = [
        ("an app", Box::new(|s| update_app(s, "org.example.Calculator"))),
        ("a shared part", Box::new(move |s| update_part(s, &part))),
        ("Trindle", Box::new(update_trindle)),
        ("Debian's packages", Box::new(update_debian)),
        ("a device's firmware", Box::new(|s| update_device(s, "abc"))),
        ("a plugin", Box::new(move |s| update_plugin(s, &offer))),
    ];
    for (what, press) in alone {
        s.other = None;
        press(&mut s);
        assert_eq!((s.upd.jobs, s.working, s.dials.len(), s.upd.one.clone()), (0, 0, 0, None), "{what} on its own started nothing");
        assert_eq!(s.other, Some(Who::Ring), "{what}: the page now says who has the turn");
    }

    assert!(updating::begin_waiting(Who::Boxes, std::time::Duration::ZERO).is_none());

    assert_eq!(updating::begin(Who::Ring).err(), Some(Who::Ring));
    drop(ring);
    updating::pretend(None, false);
}

#[test]
fn one_update_alone_holds_the_turn_and_the_counts_drop_by_one() {
    let path = updating::tests::temp("apps-alone");
    updating::pretend(Some(path.clone()), false);
    let mut s = store_with_updates();
    s.updatable.push("org.example.Paint".into());
    s.upd = std::mem::take(&mut s.upd).with_flats(&[("org.example.Calculator", true, "2.0", 0), ("org.example.Paint", true, "", 0),
                                                    ("org.example.Platform", false, "", 0)]);
    assert_eq!(s.waiting_updates(), 8, "the shared part is counted with the apps");
    assert!(headline_of(&view(&s)).starts_with("8 updates are ready."));
    assert!(view(&s).said().iter().any(|w| w == "From Flathub, to version 2.0"), "the version it would go to is on its row");

    let hold = alone(&mut s, "Calculator", || updating::begin(Who::Apps)).expect("free, so it starts");
    begin(&mut s, &app_key("org.example.Calculator"), |_| {});
    assert_eq!(updating::holder_at(&path), Some(Who::Apps), "the same lock Install all takes");
    assert_eq!(headline_of(&view(&s)), "Updating Calculator. You can keep working while it goes in.");

    let during = update_buttons(&view(&s));
    assert_eq!(during.len(), 8, "the greyed pill and the seven rows: {during:?}");
    assert!(during.iter().all(|(_, can)| !*can), "nothing else can be pressed while it goes in: {during:?}");
    assert!(during.iter().any(|(tip, _)| tip == "Update org.example.Calculator"), "{during:?}");

    assert!(alone(&mut s, "Paint", || updating::begin(Who::Apps)).is_none());
    install_all(&mut s);
    check_all(&mut s);
    assert_eq!((s.upd.jobs, s.upd.one.as_deref(), s.upd.checking.len()), (1, Some("Calculator"), 0));
    assert!(app_share(&s, "org.example.Calculator").is_some(), "Installed and the app's page show it going in");
    assert_eq!(app_share(&s, "org.example.Paint"), None);

    s.dials.clear();
    went_in(&mut s, &app_key("org.example.Calculator"));
    (s.upd.jobs, s.upd.one) = (0, None);
    drop(hold);
    assert_eq!(updating::holder_at(&path), None, "the turn is given up");
    assert_eq!(s.waiting_updates(), 7);
    assert_eq!(s.updatable, ["org.example.Paint"], "the others wait as they did, in their order");
    let after = view(&s);
    assert!(headline_of(&after).starts_with("7 updates are ready."), "{}", headline_of(&after));
    let buttons = update_buttons(&after);
    assert!(buttons.iter().all(|(_, can)| *can) && buttons.iter().any(|(tip, _)| tip == "Install 7 updates"), "{buttons:?}");
    assert!(!buttons.iter().any(|(tip, _)| tip == "Update org.example.Calculator"));

    let part = s.upd.flats.iter().find(|f| !f.app).unwrap().key();
    went_in(&mut s, &part);
    assert_eq!((s.waiting_updates(), s.upd.parts()), (6, 0));

    s.say(&app_key("org.example.Paint"), "Not updated. Flatpak said: error: the remote could not be reached", Tone::Problem);
    s.say("fw:abc", "The device was unplugged.", Tone::Problem);
    let failed = view(&s);
    assert!(failed.said().iter().any(|w| w.starts_with("Not updated. Flatpak said:")) && failed.said().iter().any(|w| w == "The device was unplugged."));
    assert!(update_buttons(&failed).iter().all(|(_, can)| *can));

    s.upd.restart = true;
    let wants = view(&s);
    assert!(headline_of(&wants).ends_with("What has gone in needs a restart to finish."), "{}", headline_of(&wants));
    assert!(update_buttons(&wants).iter().any(|(tip, _)| tip == "Install 6 updates"), "Install all is still the main action");

    forget_failures(&mut s);
    assert!(!view(&s).said().iter().any(|w| w == "The device was unplugged."));
    updating::pretend(None, false);
}

#[test]
fn debian_s_packages_have_one_button_and_the_card_says_why() {
    let s = store_with_updates();
    let page = view(&s);
    let said = page.said();
    assert!(said.iter().any(|w| w == debian_why()), "{said:?}");
    assert!(said.iter().any(|w| w == "2 updates, which go in together"));
    let tips: Vec<String> = page.buttons().into_iter().map(|(tip, _)| tip).collect();
    assert!(tips.contains(&"Update Debian's packages".to_string()));
    assert!(!tips.iter().any(|t| t == "Update bash" || t == "Update curl"), "no package of Debian's has a button of its own: {tips:?}");
    for words in [debian_why(), "An app brings the shared parts its new version needs."] {
        assert!(!words.contains('\u{2014}') && words.ends_with('.'));
    }
}

#[test]
fn sizes_are_read_from_flatpak_and_apt() {
    assert_eq!(bytes_from("45.2\u{a0}MB"), 45_200_000);
    assert_eq!(bytes_from("800 kB"), 800_000);
    assert_eq!(bytes_from("1.5 GB"), 1_500_000_000);
    assert_eq!(bytes_from(""), 0);
    let found = parse_flat_updates("app/org.example.Paint/x86_64/stable\tflathub\t3.3\t120.0\u{a0}MB\n\
                                    runtime/org.example.Platform/x86_64/25.08\tflathub\t\t30 MB\n\
                                    app/org.example.Notes/x86_64/beta\texample-origin\n\n\
                                    org.example.NoKind/x86_64/stable\tflathub\n\
                                    app/--system/x86_64/stable\tflathub\n");
    let told: Vec<(String, bool, &str, &str, u64, String)> = found.iter()
        .map(|f| (f.named(), f.app, f.origin.as_str(), f.version.as_str(), f.size, f.key())).collect();
    assert_eq!(told, vec![
        ("org.example.Paint/x86_64/stable".to_string(), true, "flathub", "3.3", 120_000_000, "up:app:org.example.Paint".to_string()),
        ("org.example.Platform/x86_64/25.08".to_string(), false, "flathub", "", 30_000_000, "up:part:org.example.Platform:25.08".to_string()),
        ("org.example.Notes/x86_64/beta".to_string(), true, "example-origin", "", 0, "up:app:org.example.Notes".to_string()),
    ], "apps and shared parts, each named whole; a line that is neither, or could be read as an option, is left");
    assert_eq!((from_words("flathub"), from_words(""), from_words("example-origin")), ("Flathub".into(), "Flathub".into(), "example-origin".to_string()));
    let debs = parse_package_sizes("Package: bash\nVersion: 5.2\nSize: 1500000\n\nPackage: curl\nSize: 300000\n");
    assert_eq!(debs, vec![("bash".to_string(), 1_500_000), ("curl".to_string(), 300_000)]);
}

#[test]
fn no_card_repeats_what_the_commands_say_and_each_failure_says_why_where_it_is() {
    let mut s = store_with_updates();
    s.upd.flats = parse_flat_updates("app/org.example.Calculator/x86_64/stable\tflathub\t2.0\t1.0 MB\tCalculator\n");
    let cards = |s: &Store| -> Vec<String> {
        view(s).said().into_iter().filter(|w| ["Apps", "Trindle and Debian", "Firmware", "Plugins", "Held back", "Security updates"].contains(&w.as_str())).collect()
    };

    let key = app_key("org.example.Calculator");
    let going = || {
        let mut d = crate::common::filling::Dial::default();
        d.within(0.42);
        d
    };
    for state in 0..4 {
        match state {
            1 => { s.dials.insert(key.clone(), going()); }
            2 => {
                s.dials.clear();
                s.say(&key, why_not_updated(&["Updating 1/1\u{2026} 100%".to_string(), "error: The disk is full".to_string()]), Tone::Problem);
            }
            3 => s.say(SYSTEM_ROW, Part::NoPassword.why().unwrap(), Tone::Problem),
            _ => {}
        }
        let said = view(&s).said();
        assert_eq!(cards(&s).last().map(String::as_str), Some("Security updates"), "state {state}: {said:?}");

        let card = ["What it is ", "doing"].concat();
        for gone in [card.as_str(), "Nothing is going in now.", "Checking for updates.", "Checked.", "Done."] {
            assert!(!said.iter().any(|w| w == gone), "state {state} still says {gone:?}");
        }
        assert!(!said.iter().any(|w| w.starts_with("Updating 1/1")), "state {state}: a command's own line is on the page");
    }

    s.said.clear();
    s.dials.insert(key.clone(), going());
    assert!(view(&s).said().iter().any(|w| w == "42%"), "{:?}", view(&s).said());

    s.dials.clear();
    s.say(&key, why_not_updated(&["error: The disk is full".to_string()]), Tone::Problem);
    s.say(SYSTEM_ROW, Part::NoPassword.why().unwrap(), Tone::Problem);
    let said = view(&s).said();
    assert!(said.iter().any(|w| w == "Not updated. Flatpak said: error: The disk is full"), "{said:?}");
    assert!(said.iter().any(|w| w == "Not installed, because no administrator's password was given."), "{said:?}");

    assert_eq!(not_fetched(""), no_lists());
    assert_eq!(not_fetched("Not done: no administrator's password was given."), "Debian's lists could not be fetched. No administrator's password was given.");
    assert_eq!(not_fetched("Temporary failure resolving 'deb.example.org'"), "Debian's lists could not be fetched. Temporary failure resolving 'deb.example.org'.");

    assert_eq!(crate::store::pages::job_rows::no_reason(), "It did not finish, and gave no reason.");
    assert_eq!(why_not_updated(&[]), "Not updated, and Flatpak gave no reason.");
    no_words_point_elsewhere_for_the_reason();
}

fn no_words_point_elsewhere_for_the_reason() {

    let trees = crate::common::source::trees();
    let pointers = [["What it is ", "doing"].concat(), ["page, says ", "why"].concat(), ["Tick the ", "box"].concat(), ["to see all it ", "said"].concat()];
    let mut wrong = Vec::new();

    for folder in ["store", "ring", "common", "toolkit/common"] {
        let (tree, name) = match folder.strip_prefix("toolkit/") {
            Some(rest) => (&trees[1].0, rest),
            None => (&trees[0].0, folder),
        };
        for entry in std::fs::read_dir(tree.join(name)).unwrap().flatten() {
            let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
            for (n, line) in text.lines().enumerate() {
                if !line.trim_start().starts_with("//") && pointers.iter().any(|p| line.contains(p.as_str())) {
                    wrong.push(format!("{}:{}: {}", entry.path().display(), n + 1, line.trim()));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "these point somewhere else for the reason, or at the card that is gone:\n{}", wrong.join("\n"));
}

#[test]
fn without_the_internet_what_is_known_is_shown_and_the_rest_said_plainly() {
    let path = updating::tests::temp("apps-page-offline");
    updating::pretend(Some(path.clone()), false);
    let mut s = store_with_updates();
    s.notice_updating();
    let with = update_buttons(&view(&s));
    assert!(with.iter().all(|(_, can)| *can));

    s.upd.failed.insert(APPS, no_flathub().into());
    s.online = false;
    let page = view(&s);
    assert_eq!(update_buttons(&page), with, "every button is where it was and can be pressed");
    assert_eq!(headline_of(&page), format!("6 updates are ready. {}", no_more()));
    assert!(page.said().iter().any(|w| w == "button: Update org.example.Calculator"));
    assert!(!page.said().iter().any(|w| w.starts_with("Could not check")), "not being connected is no failure");
    assert_eq!(status(&s, APPS, None, 1, 0, 0).0, Dot::Waiting);

    updating::pretend(Some(path.clone()), true);
    s.notice_updating();
    assert!(update_buttons(&view(&s)).iter().all(|(_, can)| !*can));
    assert_eq!(update_buttons(&view(&s)).len(), with.len());
    updating::pretend(Some(path.clone()), false);
    s.notice_updating();

    let mut s = store_with_updates();
    (s.updatable, s.debian, s.trindle_waiting, s.firmware, s.plugins) = (Vec::new(), Vec::new(), None, Vec::new(), Default::default());
    s.online = false;
    let page = view(&s);
    assert_eq!(headline_of(&page), nothing_known());
    assert_eq!(headline(&s).0, Dot::Quiet);
    assert!(page.said().iter().filter(|w| *w == not_checked()).count() >= 2, "the apps' card and Trindle and Debian's");
    assert!(!page.said().iter().any(|w| w == "Up to date" || w.starts_with("Everything is up to date")));
    assert_eq!(status(&s, APPS, None, 0, 0, 0), (Dot::Quiet, not_checked().to_string()));
    assert_eq!(update_buttons(&page), [("Check for updates".to_string(), true)], "looking can still be asked for");

    s.online = true;
    assert_eq!(status(&s, APPS, None, 0, 0, 0).0, Dot::UpToDate);
    updating::pretend(None, false);
    let _ = std::fs::remove_file(path);
}

#[test]
fn with_looking_by_itself_off_nothing_claims_a_look_that_was_not_made() {
    let mut s = store_with_updates();
    (s.updatable, s.debian, s.trindle_waiting, s.firmware, s.plugins) = (Vec::new(), Vec::new(), None, Vec::new(), Default::default());
    s.online = true;
    s.upd.off = true;
    s.upd.checked_at = None;
    let page = view(&s);
    assert_eq!(headline_of(&page), "Checking for updates by itself is off. Press Check for updates to look.");
    assert_eq!((headline(&s).0, status(&s, APPS, None, 0, 0, 0)), (Dot::Quiet, (Dot::Quiet, "Not checked".to_string())));
    assert!(!page.said().iter().any(|w| w == "Up to date" || w.starts_with("Everything is up to date")));
    assert_eq!(update_buttons(&page), [("Check for updates".to_string(), true)], "the press still works");

    s.upd.checked_at = Some("14:32".into());
    assert_eq!(headline(&s).0, Dot::UpToDate);
    s.upd.off = false;
    s.upd.checked_at = None;
    assert_eq!(status(&s, APPS, None, 0, 0, 0).0, Dot::UpToDate);

    let mut waits = store_with_updates();
    waits.upd.off = true;
    assert!(waits.waiting_updates() > 0 && headline(&waits).1.contains("ready"), "{}", headline(&waits).1);
}

#[test]
fn updates_already_known_stay_when_flathub_does_not_answer() {
    const ONE: &str = "app/org.example.Paint/x86_64/stable\tflathub\t3.3\t45.2 MB\n";
    let mut s = store_with_updates();
    assert!(!flatpaks_heard(&mut s, true, true, ONE), "read from what Flatpak last fetched");
    assert_eq!(s.updatable, ["org.example.Paint"]);
    assert_eq!(newer_version(&s, "org.example.Paint"), Some("3.3"));

    assert!(flatpaks_heard(&mut s, false, false, ""));
    assert_eq!(s.updatable, ["org.example.Paint"]);
    assert_eq!(s.upd.failed.get(APPS).map(String::as_str), Some(no_flathub()));

    assert!(!flatpaks_heard(&mut s, true, false, ""));
    assert_eq!(s.updatable, ["org.example.Paint"]);

    assert!(!flatpaks_heard(&mut s, false, true, ""));
    assert!(s.updatable.is_empty() && s.upd.flats.is_empty() && s.upd.failed.is_empty());

    s.online = false;
    assert!(flatpaks_heard(&mut s, false, false, ""));
    assert!(s.upd.failed.is_empty());
}

#[test]
fn a_check_that_never_answers_is_given_up_on() {
    let mut s = store_with_updates();
    s.upd.round = 2;
    s.upd.checking.extend([APPS, SYSTEM]);
    gave_up(&mut s, 1);
    assert_eq!(phase(&s), Phase::Checking, "an earlier check's limit ends nothing");
    gave_up(&mut s, 2);
    assert!(s.upd.checking.is_empty());
    assert_eq!(phase(&s), Phase::Found);
    assert_eq!(status(&s, APPS, None, 0, 0, 0), (Dot::Failed, format!("Could not check. {}", too_long())));
    assert!(s.upd.checked_at.is_some());

    s.upd.checked_at = None;
    assert!(!flatpaks_heard(&mut s, false, true, ""));
    answered(&mut s, APPS);
    assert!(!s.upd.failed.contains_key(APPS) && s.upd.checked_at.is_none());

    let mut s = store_with_updates();
    s.online = false;
    s.upd.round = 1;
    s.upd.checking.extend([APPS, SYSTEM, FIRMWARE]);
    gave_up(&mut s, 1);
    assert!(s.upd.checking.is_empty() && s.upd.failed.is_empty());
}

#[test]
fn an_app_is_named_from_this_computer_without_the_catalogue() {
    let mut s = store_with_updates();
    assert_eq!(app_name(&s, "org.example.Calculator"), "org.example.Calculator", "nothing read yet");
    s.owned = crate::store::pages::made_up::installed(Some(vec![updates::Installed {
        id: "org.example.Calculator".into(), name: "Sums".into(), version: "1.0".into(), size: String::new(),
        source: updates::Source::Flatpak { origin: REMOTE.into(), installation: "user".into() },
    }]), 0, "");
    assert_eq!(app_name(&s, "org.example.Calculator"), "Sums");
    assert!(view(&s).said().iter().any(|w| w == "Sums"));
}

fn hold_buttons(s: &Store) -> Vec<(String, bool)> {
    view(s).buttons().into_iter().filter(|(tip, _)| tip.starts_with("Hold back") || tip.starts_with("Release")).collect()
}

fn row_order(s: &Store) -> Vec<String> {
    update_buttons(&view(s)).into_iter().map(|(tip, _)| tip).filter(|t| t.starts_with("Update ")).collect()
}

fn held_card_shown(s: &Store) -> bool {
    view(s).said().iter().any(|w| w == "Held back")
}

#[test]
fn an_update_held_back_stays_in_its_place_and_out_of_every_count() {
    let path = held::temp("apps-held");
    held::pretend(Some(path.clone()));
    updating::pretend(Some(updating::tests::temp("apps-held")), false);
    let mut s = store_with_updates();
    s.updatable.push("org.example.Paint".into());
    s.installed.insert("org.example.Calculator".into(), ("user".into(), "1.0".into()));
    s.upd = std::mem::take(&mut s.upd).with_flats(&[("org.example.Calculator", true, "2.0", 40_000_000), ("org.example.Paint", true, "", 10_000_000),
                                                    ("org.example.Platform", false, "25.08.3", 300_000_000)]);
    s.upd.sizes.extend([("org.example.Calculator".to_string(), 40_000_000), ("org.example.Paint".to_string(), 10_000_000)]);
    the_holding_buttons_are_there_and_nothing_is_held(&s);
    let rows = row_order(&s);
    one_of_each_kind_is_held_back(&mut s, &rows, &path);
    everything_of_flatpaks_is_held(&mut s);
    released_it_waits_again(&mut s);
    updating::pretend(None, false);
    held::pretend(None);
}

fn the_holding_buttons_are_there_and_nothing_is_held(s: &Store) {
    let before = hold_buttons(s);
    assert_eq!(before.iter().map(|(tip, _)| tip.as_str()).collect::<Vec<_>>(),
               ["Hold back org.example.Calculator", "Hold back org.example.Paint", "Hold back Example Platform 25.08 (org.example.Platform)", "Hold back Example keyboard",
                "Hold back org.example.One"],
               "an app, a shared part, a device's firmware and a plugin; not Trindle, and not Debian's packages");
    assert!(before.iter().all(|(_, can)| *can));
    assert!(view(s).said().iter().any(|w| w == not_held()) && view(s).said().iter().any(|w| w == never_held()), "why Trindle's and Debian's have none");
    assert_eq!(counts(s), (8, 0));
    assert_eq!(headline_of(&view(s)), "8 updates are ready, 350 MB in all. The firmware needs a restart to finish.");
    assert_eq!(flatpaks_for_all(s), Some(Vec::new()), "nothing held: Flatpak updates everything, as ever");
}

fn one_of_each_kind_is_held_back(s: &mut Store, rows: &[String], path: &std::path::Path) {

    let part = s.upd.flats[2].clone();
    for (thing, key) in [(Thing::app("org.example.Calculator"), app_key("org.example.Calculator")), (part.thing(), part.key()),
                         (Thing::firmware("abc"), "fw:abc".to_string()), (Thing::plugin("org.example.One"), "pl:org.example.One".to_string())] {
        assert!(keep_held(s, &thing.expect("a plain name"), true, &key));
    }
    assert_eq!(counts(s), (4, 4), "Paint, Trindle and Debian's two can go in");
    assert_eq!(s.waiting_updates(), 4, "the side bar's count");
    let page = view(s);
    assert_eq!(headline_of(&page), "4 updates are ready, 10 MB in all; 4 are held back.", "nor is the held firmware's restart spoken of");
    assert_eq!(row_order(s), rows, "every row is still there, in its place");
    let buttons = update_buttons(&page);
    assert!(buttons.contains(&("Install 4 updates".to_string(), true)), "{buttons:?}");
    for held in ["Update org.example.Calculator", "Update Example Platform 25.08 (org.example.Platform)", "Update Example keyboard", "Update org.example.One"] {
        assert!(buttons.contains(&(held.to_string(), false)), "{held} is off: {buttons:?}");
    }
    for free in ["Update org.example.Paint", "Update Trindle", "Update Debian's packages"] {
        assert!(buttons.contains(&(free.to_string(), true)), "{free} can be pressed: {buttons:?}");
    }
    assert_eq!(hold_buttons(s).iter().map(|(tip, can)| (tip.as_str(), *can)).collect::<Vec<_>>(),
               [("Release org.example.Calculator", true), ("Hold back org.example.Paint", true), ("Release Example Platform 25.08 (org.example.Platform)", true),
                ("Release Example keyboard", true), ("Release org.example.One", true)], "each hold button is where it was, now Release");
    let said = page.said();
    for words in ["Held back at version 1.0; version 2.0 is waiting", "Held back; version 25.08.3 is waiting", "Held back at version 1; version 2 is waiting",
                  "1 update, 10 MB; 2 held back", "1 held back", "1 plugin's update is held back.", hold_why()] {
        assert!(said.iter().any(|w| w == words), "{words}: {said:?}");
    }
    assert!(!said.iter().any(|w| w.contains('\u{2014}')));

    assert_eq!(flatpaks_for_all(s), Some(vec!["org.example.Paint/x86_64/stable".to_string()]));

    assert_eq!(held::Held::read(), s.held);
    assert!(std::fs::read_to_string(path).unwrap().contains("part org.example.Platform 25.08\n"));
}

fn everything_of_flatpaks_is_held(s: &mut Store) {

    assert!(keep_held(s, &Thing::app("org.example.Paint").unwrap(), true, &app_key("org.example.Paint")));
    assert_eq!(flatpaks_for_all(s), None);
    assert_eq!(status(s, APPS, None, 0, 0, 3), (Dot::Quiet, "3 held back".to_string()), "a card with only held updates is not up to date");

    (s.debian, s.trindle_waiting) = (Vec::new(), None);
    assert_eq!(counts(s), (0, 5));
    assert_eq!(headline(s), (Dot::Quiet, "No updates are ready; 5 are held back.".to_string()));
    assert_eq!(update_buttons(&view(s)).first(), Some(&("Check for updates".to_string(), true)), "Install all is not offered");
    install_all(s);
    assert_eq!((s.upd.jobs, s.working, s.dials.len()), (0, 0, 0), "and pressed all the same, it starts nothing");
    s.online = false;
    assert_eq!(headline(s).1, format!("No updates are ready; 5 are held back. {}", no_more()));
    s.online = true;
}

fn released_it_waits_again(s: &mut Store) {

    assert!(keep_held(s, &Thing::app("org.example.Calculator").unwrap(), false, &app_key("org.example.Calculator")));
    assert_eq!(counts(s), (1, 4));
    assert_eq!(headline_of(&view(s)), "1 update is ready, 40 MB in all; 4 are held back.");
    assert!(update_buttons(&view(s)).contains(&("Update org.example.Calculator".to_string(), true)));
    assert!(!held::Held::read().app("org.example.Calculator"));

    s.other = Some(Who::Ring);
    assert!(hold_buttons(s).iter().all(|(_, can)| !*can));
    set_held(s, &Thing::app("org.example.Calculator").unwrap(), true, "up:app:org.example.Calculator");
    assert!(!s.held.app("org.example.Calculator"), "a press that slipped through changes nothing");
}

#[test]
fn everything_held_back_is_listed_in_one_card_with_its_release() {
    let path = held::temp("apps-held-card");
    held::pretend(Some(path));
    updating::pretend(Some(updating::tests::temp("apps-held-card")), false);
    let mut s = store_with_updates();
    s.installed.insert("org.example.Calculator".into(), ("user".into(), "1.0".into()));
    s.installed.insert("org.example.Old".into(), ("user".into(), "7.1".into()));
    s.upd = std::mem::take(&mut s.upd).with_flats(&[("org.example.Calculator", true, "2.0", 40_000_000)]);
    let calculator = a_press_never_adds_the_card(&mut s);

    let old = Thing::app("org.example.Old").unwrap();
    let part = Thing::part("org.example.Platform", "25.08").unwrap();
    let device = Thing::firmware("zzz").unwrap();
    let keys = the_card_lists_what_is_held(&mut s, &calculator, &old, &part, &device);
    let paint = one_released_from_the_card(&mut s, &old, &keys);
    the_last_one_released(&mut s, [&calculator, &old, &part, &device, &paint]);
    updating::pretend(None, false);
    held::pretend(None);
}

fn a_press_never_adds_the_card(s: &mut Store) -> Thing {

    assert!(!held_card_shown(s));
    let calculator = Thing::app("org.example.Calculator").unwrap();
    assert!(keep_held(s, &calculator, true, &app_key("org.example.Calculator")));
    assert!(!held_card_shown(s), "a press added a card");
    assert!(keep_held(s, &calculator, false, &app_key("org.example.Calculator")));
    calculator
}

fn the_card_lists_what_is_held(s: &mut Store, calculator: &Thing, old: &Thing, part: &Thing, device: &Thing) -> Vec<String> {
    for thing in [&calculator, &old, &part, &device] {
        assert!(keep_held(s, thing, true, &held_row_key(thing)));
    }

    s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
    assert!(held_card_shown(s));
    let said = view(s).said();
    for words in ["4 held back", "Held back at version 1.0; version 2.0 is waiting", "Held back at version 7.1; nothing newer yet", "Held back; nothing newer yet",
                  "Example Platform 25.08", "A device's firmware", release_why()] {
        assert!(said.iter().any(|w| w == words), "{words}: {said:?}");
    }
    let in_card = ["Release org.example.Calculator", "Release org.example.Old", "Release Example Platform 25.08 (org.example.Platform)",
                   "Release the firmware of device zzz"];
    let all = hold_buttons(s);
    for tip in in_card {
        assert!(all.contains(&(tip.to_string(), true)), "{tip}: {all:?}");
    }
    assert_eq!(all.iter().filter(|(tip, _)| tip == "Release org.example.Calculator").count(), 2, "on its own row and in the card");
    let keys: Vec<String> = s.upd.held_rows.iter().map(held_row_key).collect();
    assert_eq!(keys, ["held:up:app:org.example.Calculator", "held:up:app:org.example.Old", "held:up:part:org.example.Platform:25.08", "held:fw:zzz"]);
    keys
}

fn one_released_from_the_card(s: &mut Store, old: &Thing, keys: &[String]) -> Thing {

    assert!(keep_held(s, old, false, &held_row_key(old)));
    assert!(!s.held.has(old) && !held::Held::read().has(old));
    let said = view(s).said();
    assert!(said.iter().any(|w| w == "3 held back") && said.iter().any(|w| w == released()), "{said:?}");
    assert!(hold_buttons(s).contains(&("Hold back org.example.Old".to_string(), true)));
    assert_eq!(s.upd.held_rows.iter().map(held_row_key).collect::<Vec<_>>(), keys, "no row has gone or moved");

    let paint = Thing::app("org.example.Paint").unwrap();
    assert!(keep_held(s, &paint, true, &app_key("org.example.Paint")));
    assert_eq!(s.upd.held_rows.len(), 5);
    assert_eq!(s.upd.held_rows.last(), Some(&paint));
    assert_eq!(s.upd.held_rows[..4].iter().map(held_row_key).collect::<Vec<_>>(), keys);

    assert!(keep_held(s, old, true, &held_row_key(old)));
    assert_eq!(s.upd.held_rows.len(), 5);
    paint
}

fn the_last_one_released(s: &mut Store, things: [&Thing; 5]) {

    for thing in things {
        assert!(keep_held(s, thing, false, &held_row_key(thing)));
    }
    assert!(s.held.is_empty() && held_card_shown(s), "the card went with the last release");
    let said = view(s).said();
    assert!(said.iter().any(|w| w == nothing_held()), "{said:?}");
    assert_eq!(said.iter().filter(|w| *w == released()).count(), 5);
    assert!(!said.iter().any(|w| w.contains('\u{2014}')));

    s.other = Some(Who::Ring);
    assert!(hold_buttons(s).iter().all(|(_, can)| !*can));
    s.other = None;

    s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
    assert!(!held_card_shown(s));
}

#[test]
fn a_hold_that_cannot_be_kept_is_said_in_red_and_changes_nothing() {

    let dir = held::temp("apps-held-refused");
    std::fs::write(&dir, "in the way").unwrap();
    held::pretend(Some(dir.join("held-updates")));
    std::fs::write(&dir, "in the way").unwrap();
    let mut s = store_with_updates();
    assert!(!keep_held(&mut s, &Thing::app("org.example.Calculator").unwrap(), true, &app_key("org.example.Calculator")));
    assert!(!s.held.app("org.example.Calculator"));
    assert_eq!(counts(&s), (6, 0));
    assert!(view(&s).said().iter().any(|w| w.starts_with("Not held back. The list of what is held back could not be kept")));
    assert!(!keep_held(&mut s, &Thing::plugin("org.example.One").unwrap(), true, "pl:org.example.One"));
    assert!(plugin_page::problem(&s, "org.example.One").is_some_and(|w| w.starts_with("Not held back.")));
    held::pretend(None);
}

#[test]
fn a_shared_part_goes_by_a_plain_name() {
    let found = parse_flat_updates("runtime/org.example.Platform/x86_64/25.08\tflathub\t25.08.3\t259.3\u{a0}MB\tExample Platform\n\
                                    runtime/org.example.Other.Platform/x86_64/49\tflathub\t\t418 MB\tOther Application Platform version 49\n\
                                    runtime/org.example.Platform.GL.default/x86_64/25.08-extra\tflathub\t26.2\t148 MB\tMesa (Extra)\n\
                                    runtime/org.example.Platform.GL.default/x86_64/25.08\tflathub\n\
                                    runtime/com.example.Sdk/x86_64/1\tflathub\t\t\t   \n\
                                    app/org.example.Paint/x86_64/stable\tflathub\t3.3\t120 MB\tPaint\n");
    let names: Vec<String> = found.iter().filter(|f| !f.app).map(Flat::plain_name).collect();
    assert_eq!(names, ["Example Platform 25.08", "Other Application Platform version 49", "Mesa (Extra) 25.08-extra",
                       "Example Platform GL default 25.08", "Example Sdk 1"]);
    assert_eq!(found[0].size, 259_300_000, "the columns before the name are read as they were");
    assert_eq!((tidy_id("org.example.Platform"), tidy_id("example.Platform"), tidy_id("Platform"), tidy_id("")),
               ("Example Platform".to_string(), "Example Platform".to_string(), "Platform".to_string(), String::new()));

    held::pretend(None);
    let mut s = store_with_updates();
    s.upd = std::mem::take(&mut s.upd).with_flats(&[("org.example.Platform", false, "", 0)]).with_part_name("org.example.Platform", "Example Platform");
    let page = view(&s);
    let said = page.said();
    assert!(said.iter().any(|w| w == "Example Platform 25.08"), "{said:?}");
    assert!(!said.iter().any(|w| w == "org.example.Platform"), "its id is not what the row is called");
    assert!(said.iter().any(|w| w == "A shared part, from Flathub"));
    let buttons = page.buttons();
    assert!(buttons.contains(&("Update Example Platform 25.08 (org.example.Platform)".to_string(), true)), "its id is in its tips: {buttons:?}");
    assert!(buttons.contains(&("Hold back Example Platform 25.08 (org.example.Platform)".to_string(), true)), "{buttons:?}");
}

#[test]
fn the_headline_counts_and_sizes_what_is_ready() {
    assert_eq!(found_sentence(3, 0, false, 1), "3 updates are ready; 1 is held back.");
    assert_eq!(found_sentence(3, 120_000_000, false, 2), "3 updates are ready, 120 MB in all; 2 are held back.");
    assert_eq!(held_words(Some("1.0"), None), "Held back at version 1.0; an update is waiting");
    assert_eq!(held_words(Some(" "), Some("")), "Held back; an update is waiting");
    assert_eq!(found_sentence(4, 210_000_000, false, 0), "4 updates are ready, 210 MB in all.");
    assert_eq!(found_sentence(1, 0, true, 0), "1 update is ready. The firmware needs a restart to finish.");
}

#[test]
fn flatpak_s_table_and_progress_name_each_row() {
    assert_eq!(table_row(" 1.     org.gnome.Platform   51  u  flathub  < 432.2 MB"), Some("org.gnome.Platform".into()));
    assert_eq!(table_row(" 2. [\u{2713}] org.gimp.GIMP stable u flathub < 120 MB"), Some("org.gimp.GIMP".into()));
    assert_eq!(table_row("Updating 1/2\u{2026}"), None);
    assert_eq!(fetching("Updating 2/5\u{2026} \u{2588}\u{2588}   44%  9.8 MB/s"), Some((2, 0.44)));
    assert_eq!(fetching("Updating 1/5\u{2026}"), Some((1, 0.0)));
    assert_eq!(fetching("Looking for updates\u{2026}"), None);
}

fn args(rest: &[&str]) -> Vec<String> {
    std::iter::once("trindle-store").chain(rest.iter().copied()).map(String::from).collect()
}

#[test]
fn the_rings_ways_of_opening_the_page_are_understood() {
    use crate::store::opts::Opts;
    let o = Opts::read(&args(&["--updates"]));
    assert_eq!((o.updates.as_deref(), o.update_now), (Some(""), false), "the page alone, which used to be dropped for want of a word");
    let o = Opts::read(&args(&["--updates=firmware"]));
    assert_eq!((o.updates.as_deref(), o.update_now), (Some("firmware"), false));
    let o = Opts::read(&args(&["--updates", "--update-now"]));
    assert_eq!((o.updates.as_deref(), o.update_now), (Some(""), true));
    let o = Opts::read(&args(&["--update-now"]));
    assert!(o.updates.is_none() && o.update_now, "alone it opens no page, and apply starts nothing without one");
}

#[test]
fn a_start_is_asked_only_after_the_rings_note() {
    let mut s = store_with_updates();
    ask_start(&mut s);
    assert!(!start_is_waiting(&s), "no note: the option alone starts nothing");
    assert!(crate::common::update_asked::leave());
    ask_start(&mut s);
    assert!(start_is_waiting(&s));
    ask_start(&mut s);
    assert!(!start_is_waiting(&s), "the note was used up, so a second try asks for nothing");
}

#[test]
fn a_start_asked_for_is_dropped_when_it_may_not_go_ahead() {
    let path = updating::tests::temp("apps-asked");
    updating::pretend(Some(path.clone()), false);
    let quiet = |s: &Store| (s.upd.jobs, s.working, s.dials.len());
    let asked = |s: &mut Store| { s.upd.start_asked = true; };

    let mut s = store_with_updates();
    s.view = crate::store::View::Updates;
    asked(&mut s);
    start_asked(&mut s);
    assert_eq!((quiet(&s), start_is_waiting(&s)), ((0, 0, 0), false));

    let mut none = store_with_updates();
    (none.updatable, none.debian, none.trindle_waiting, none.firmware, none.plugins) = (Vec::new(), Vec::new(), None, Vec::new(), Default::default());
    none.view = crate::store::View::Updates;
    asked(&mut none);
    start_asked(&mut none);
    assert_eq!((quiet(&none), start_is_waiting(&none)), ((0, 0, 0), false));

    let mut elsewhere = store_with_updates();
    elsewhere.firmware.clear();
    elsewhere.view = crate::store::View::Home;
    asked(&mut elsewhere);
    start_asked(&mut elsewhere);
    assert_eq!((quiet(&elsewhere), start_is_waiting(&elsewhere)), ((0, 0, 0), false));

    let apps_turn = updating::Lock::take(&path, Who::Boxes).expect("free");
    let mut busy = store_with_updates();
    busy.firmware.clear();
    busy.view = crate::store::View::Updates;
    asked(&mut busy);
    start_asked(&mut busy);
    assert_eq!((quiet(&busy), start_is_waiting(&busy)), ((0, 0, 0), false));
    assert_eq!(busy.other, Some(Who::Boxes), "and the page says who has it");

    let mut going = store_with_updates();
    going.upd.jobs = 1;
    asked(&mut going);
    looked_again(&mut going);
    assert!(!start_is_waiting(&going));
    drop(apps_turn);
    updating::pretend(None, false);
}

#[test]
fn the_rings_start_is_the_pages_install_all() {
    let source = crate::common::source::code(include_str!("install.rs"));
    let all = source.split_once("pub fn install_all(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(all.trim().ends_with("install_all_in(s);") && all.matches(";").count() == 1);
    let start = crate::common::source::code(include_str!("check.rs")).split_once("fn start_asked(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(start.contains("install_all_in(s)"));
    assert!(start.find("take(&mut s.upd.start_asked)").unwrap() < start.find("install_all_in").unwrap());
    let inner = source.split_once("fn install_all_in(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(inner.contains("updating::begin(Who::Apps)") && inner.contains("!asks_more") && !inner.contains("left_to_look"));

    assert!(crate::common::source::code(include_str!("view.rs")).contains(".on_press(go(install_all))"));
}

thread_local! {
    static STAND_IN: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(super) fn stand_in() -> Option<String> {
    STAND_IN.with(|s| s.borrow().clone())
}

fn press(s: &mut Store, tip: &str) -> bool {
    let page = view(s);
    match page.presses().into_iter().find(|(t, off, _)| t == tip && !off) {
        Some((_, _, msg)) => {
            let msg = msg.clone();
            (msg.0)(s);
            true
        }
        None => false,
    }
}

fn running_job() -> (Stop, std::process::Child, run::Proc) {
    let mut command = std::process::Command::new("sleep");
    command.arg("300");
    let (child, proc) = run::start_group(&mut command).unwrap();
    let stop = Stop::default();
    stop.hold(proc.clone());
    (stop, child, proc)
}

fn helper_pid(file: &std::path::Path) -> Option<i32> {
    std::fs::read_to_string(file).ok().and_then(|t| t.trim().parse().ok())
}

fn alive(pid: i32) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .is_ok_and(|stat| !stat.rsplit_once(')').is_some_and(|(_, rest)| rest.trim_start().starts_with('Z')))
}

fn texts(page: &Node<Msg>) -> Vec<String> {
    page.said()
}

#[test]
fn a_running_update_shows_stop_and_a_stopped_one_says_what_to_press() {
    let path = updating::tests::temp("apps-stop-page");
    updating::pretend(Some(path.clone()), false);
    let mut s = store_with_updates();
    let key = app_key("org.example.Calculator");
    let before = update_buttons(&view(&s));
    let holds = |page: &Node<Msg>| page.buttons().iter().filter(|(tip, _)| tip.starts_with("Hold back")).count();
    let held_before = holds(&view(&s));
    assert!(held_before > 1 && !view(&s).buttons().iter().any(|(tip, _)| tip == "Stop"), "nothing runs, so nothing to stop");

    let hold = alone(&mut s, "Calculator", || updating::begin(Who::Apps)).expect("free");
    begin(&mut s, &key, |_| {});
    let (stop, mut child, proc) = running_job();
    s.upd.stops.insert(key.clone(), stop.clone());
    let during = view(&s);
    let stops: Vec<_> = during.buttons().into_iter().filter(|(tip, _)| tip == "Stop").collect();
    assert_eq!(stops, [("Stop".to_string(), true)], "one Stop, on the row that runs");
    assert_eq!(holds(&during), held_before - 1, "the other rows keep their own, and the running one's room holds Stop");
    let tips = |page: &Node<Msg>| update_buttons(page).into_iter().map(|(tip, _)| tip).filter(|t| t.starts_with("Update")).collect::<Vec<_>>();
    assert_eq!(tips(&during), tips(&view(&store_with_updates())), "no Update button comes or goes");
    let _ = before;
    assert!(!texts(&during).iter().any(|w| w.starts_with("Stopped")));

    assert!(press(&mut s, "Stop"));
    assert!(stop.pressed());
    assert!(view(&s).buttons().contains(&("Stop".to_string(), false)), "greyed, not gone");
    assert!(!press(&mut s, "Stop"), "and a second press is not offered");
    assert!(!stop.press(), "nor would it do anything");
    let status = run::collect(&mut child, &proc).unwrap();
    assert!(!status.success());

    s.upd.jobs = 1;
    assert!(stopped(&mut s, &key, &stop, false));
    s.upd.one = None;
    job_done(&mut s);
    drop(hold);
    assert_eq!(updating::holder_at(&path), None, "the turn is given back");
    let after = view(&s);
    assert!(texts(&after).iter().any(|w| w == "Stopped. Press Update to carry on."), "{:?}", texts(&after));
    assert!(!after.buttons().iter().any(|(tip, _)| tip == "Stop"));
    assert!(!s.said.contains_key(&key) && !s.dials.contains_key(&key), "no red words and no ring");
    assert!(update_buttons(&after).iter().any(|(tip, can)| tip == "Update org.example.Calculator" && *can), "Update is back");
    assert_eq!(s.waiting_updates(), 6, "and it still waits, counted");

    forget_failures(&mut s);
    assert!(!texts(&view(&s)).iter().any(|w| w.starts_with("Stopped")));

    let (late, mut done, proc) = running_job();
    assert!(late.press());
    run::collect(&mut done, &proc);
    assert!(!stopped(&mut s, &key, &late, true), "it ended well");

    let (other, mut done, proc) = running_job();
    proc.end(std::time::Duration::from_millis(10));
    run::collect(&mut done, &proc);
    assert!(!stopped(&mut s, &key, &other, false));

    install_all_has_its_own_stop(&mut s);
    updating::pretend(None, false);
}

fn install_all_has_its_own_stop(s: &mut Store) {

    let (all, mut child, proc) = running_job();
    s.upd.stops.insert(APPS_ROW.into(), all.clone());
    assert_eq!(view(s).buttons().iter().filter(|(tip, _)| tip == "Stop").count(), 1);
    assert!(press(s, "Stop"));
    run::collect(&mut child, &proc);
    s.upd.jobs = 0;
    assert!(stopped(s, APPS_ROW, &all, false));
    let page = view(s);
    assert!(texts(&page).iter().any(|w| w == "Stopped. Press Install to carry on."), "{:?}", texts(&page));
    assert!(!s.said.contains_key(APPS_ROW));
}

#[test]
fn stop_ends_a_running_update_for_good_and_gives_the_turn_back() {
    use crate::toolkit::event::{Flow, Loop};
    use std::time::Duration;

    const INNER: &str = "TRINDLE_STOP_TEST_IN_ITS_OWN_PROCESS";
    if std::env::var_os(INNER).is_none() {
        run_again_in_its_own_process(INNER);
        return;
    }
    let path = updating::tests::temp("apps-stop-run");
    updating::pretend(Some(path.clone()), false);
    let helper = std::env::temp_dir().join(format!("trindle-stop-helper-{}", std::process::id()));
    let _ = std::fs::remove_file(&helper);
    let script = format!("echo ' 1.  org.example.Calculator  stable  u  flathub  < 1.9 MB'; sleep 300 & echo $! > {}; wait", helper.display());
    STAND_IN.with(|s| *s.borrow_mut() = Some(script));

    let events = Loop::new();
    crate::toolkit::common::run::hand_back_to(&events);
    let slot: Rc<RefCell<Option<Rc<RefCell<Store>>>>> = Rc::default();
    let seen = slot.clone();
    let asks = events.post(move |d: super::super::Do| {
        let store = seen.borrow().clone();
        if let Some(store) = store {
            store.borrow_mut().handle(d);
        }
    });
    let mut store = Store::new(&events, asks, None);
    store.online = true;
    store.updatable = vec!["org.example.Calculator".into()];
    let store = Rc::new(RefCell::new(store));
    *slot.borrow_mut() = Some(store.clone());

    assert!(press(&mut store.borrow_mut(), "Update org.example.Calculator"), "started from its own button");
    let key = app_key("org.example.Calculator");
    assert_eq!(updating::holder_at(&path), Some(Who::Apps));
    assert!(store.borrow().upd.stops.contains_key(&key));

    let ticks = Rc::new(std::cell::Cell::new(0));
    let (watch, quit, count) = (store.clone(), events.clone(), ticks.clone());
    let (named, turn, file) = (key.clone(), path.clone(), helper.clone());
    let mut pressed = false;
    events.every(Duration::from_millis(50), move || {
        count.set(count.get() + 1);
        let mut s = watch.borrow_mut();

        if !pressed && helper_pid(&file).is_some() {
            assert!(press(&mut s, "Stop"));
            assert!(!press(&mut s, "Stop"), "greyed from then on");
            stop_update(&mut s, &named);
            pressed = true;
        }
        if (pressed && s.upd.jobs == 0 && updating::holder_at(&turn).is_none()) || count.get() > 200 {
            quit.quit();
            return Flow::Stop;
        }
        Flow::Continue
    });
    events.run();

    the_stopped_update_has_ended_for_good(&store, key, &path, &helper, ticks.get());
    updating::pretend(None, false);
}

fn run_again_in_its_own_process(inner: &str) {
    let name = concat!(module_path!(), "::stop_ends_a_running_update_for_good_and_gives_the_turn_back");
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name.split_once("::").unwrap().1, "--test-threads=1", "--nocapture"]).env(inner, "1").output().unwrap();
    assert!(out.status.success() && String::from_utf8_lossy(&out.stdout).contains("1 passed"),
            "{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
}

fn the_stopped_update_has_ended_for_good(store: &Rc<RefCell<Store>>, key: String, path: &std::path::Path, helper: &std::path::Path, ticks: i32) {
    use std::time::Duration;
    STAND_IN.with(|s| *s.borrow_mut() = None);
    let s = store.borrow();
    assert!(ticks <= 200, "it ended by itself, not by running out of time: jobs {}, turn {:?}, stops {:?}, pid {:?}",
            s.upd.jobs, updating::holder_at(path), s.upd.stops.keys().collect::<Vec<_>>(), helper_pid(helper));
    assert_eq!(s.upd.jobs, 0);
    assert_eq!(updating::holder_at(path), None, "the turn is given back");
    assert!(s.upd.stopped.contains(&key) && s.upd.stops.is_empty());
    assert!(!s.said.contains_key(&key) && !s.dials.contains_key(&key), "no red words and no ring");
    drop(s);

    store.borrow_mut().updatable = vec!["org.example.Calculator".into()];
    assert!(texts(&view(&store.borrow())).iter().any(|w| w == "Stopped. Press Update to carry on."));
    let pid = helper_pid(helper).expect("the helper ran");
    assert!((0..100).any(|_| { std::thread::sleep(Duration::from_millis(50)); !alive(pid) }), "what the update started has ended too");
    let _ = std::fs::remove_file(helper);
}
