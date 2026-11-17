//! Tests for the installer's window.

use super::pages::{keep_words, matching, ready_sentence};
use super::*;
use crate::common::shipped_languages::in_language;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
use crate::toolkit::widget::Tone;

#[test]
fn warns_before_a_disk_is_replaced() {
    let erase = Some(Where::Erase { disk: "/dev/sda".into() });
    assert_eq!(ready_sentence(&erase, None), ("Everything on /dev/sda will be replaced.".to_string(), Tone::Warn));
    assert_eq!(ready_sentence(&Some(Where::Beside { part: "/dev/sda3".into(), leave: 1 }), None).1, Tone::Warn);
    assert_eq!(ready_sentence(&None, None).1, Tone::Problem);
}

#[test]
fn the_last_page_names_the_disk_by_model_size_and_what_is_on_it() {
    let part = plan::Part { dev: "/dev/sda1".into(), size: 100 * plan::GIB, fstype: "ntfs".into(), ..plan::Part::default() };
    let disk = Disk { dev: "/dev/sda".into(), model: "Example disk".into(), size: 500 * plan::GIB, parts: vec![part], ..Disk::default() };
    let (words, tone) = ready_sentence(&Some(Where::Erase { disk: "/dev/sda".into() }), Some(&disk));
    assert_eq!(tone, Tone::Warn);
    assert!(words.contains("Example disk") && words.contains("/dev/sda") && words.contains("Windows"), "{words}");
}

#[test]
fn reminds_about_the_disk_password_only_when_there_is_one() {
    assert_eq!(keep_words(Lock::None), None);
    assert!(keep_words(Lock::Password).is_some_and(|w| w.contains("write the disk password down")));
    assert!(keep_words(Lock::Chip).is_some_and(|w| w.contains("chip")));
}

fn installer(events: &Loop) -> Installer {
    let part = plan::Part { dev: "/dev/sda2".into(), size: 400 * plan::GIB, used: 60 * plan::GIB,
                            fstype: "ntfs".into(), ..plan::Part::default() };
    let disk = Disk { dev: "/dev/sda".into(), model: "Example disk".into(), size: 500 * plan::GIB,
                      parts: vec![part], ..Disk::default() };
    let machine = Machine {
        locales: vec![("en_GB.UTF-8".into(), "English (United Kingdom)".into()),
                      ("fr_FR.UTF-8".into(), "French (France)".into())],
        tongues: vec![("en".into(), "English".into()), ("fr".into(), "Français".into())],
        keyboards: vec![("gb\t".into(), "English (UK)".into())],
        zones: vec![("Europe/London".into(), "Europe/London".into())],
        disks: vec![disk],
        chip: true,
        secure_boot: false,
        memory: 16,
        short_of_memory: false,
        uefi: true,
    };
    Installer::new("Trindle".into(), Theme::chosen().accent, machine, events.clone(),
                   events.post(|_: Msg| {}), events.post(|()| {}))
}

#[test]
fn install_over_a_whole_disk_is_the_red_button() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = READY;
    app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
    for armed in [false, true] {
        app.armed = armed;
        assert_eq!(app.foot().for_good(), [if armed { "Sure?" } else { "Install" }], "armed {armed}");
    }
    app.plan.target = None;
    app.armed = false;
    assert!(app.foot().for_good().is_empty(), "Install is red with no disk to be replaced");
    app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
    for page in (0..PAGE_COUNT).filter(|p| *p != READY) {
        app.page = page;
        assert!(app.foot().for_good().is_empty(), "the button on page {page} is red");
    }
}

#[test]
fn every_page_fits_the_window() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
    app.plan.lock = Lock::Chip;
    app.disk_one = Zeroizing::new("one".into());
    app.disk_two = Zeroizing::new("two".into());
    app.one = Zeroizing::new("correct horse".into());
    app.two = Zeroizing::new("correct horse battery".into());
    app.user = "Bad Name".into();
    for (page, name) in pages().iter().enumerate() {
        app.page = page;
        app.armed = page == READY;
        let ui = Ui::new(app.view(), Theme::chosen());
        let h = ui.natural_height(SIZE.0 as f32);
        assert!(h <= SIZE.1 as f32, "{name} is {h} tall");

    }
}

#[test]
fn every_page_has_a_name_and_one_sentence() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
    app.plan.lock = Lock::Password;
    for (page, name) in pages().iter().enumerate() {
        app.page = page;
        let said = app.body().said();
        assert!(said.len() > 1, "{name} says something");
        let sentence = &said[1];
        assert!(!sentence.trim_end_matches('.').contains(". "), "{name}: one sentence, not two: {sentence}");
    }
}

#[test]
fn install_progress_is_the_standard_ring_in_the_foot() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = RUNNING;
    app.share = 0.4;
    app.doing = "Copying the system".into();
    let said = app.foot().said();
    assert!(said.contains(&"40%".to_string()) && said.contains(&"Copying the system".to_string()), "{said:?}");
    app.fail("The installation stopped.");
    let said = app.foot().said();
    assert!(!said.iter().any(|s| s.ends_with('%')), "{said:?}");
    assert!(said.contains(&"The installation stopped.".to_string()));
}

#[test]
fn the_installer_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    let events = Loop::new();
    drawn::none(drawn::in_every_look("the installer", 40, || {
        for right in [true, false] {
            let mut app = installer(&events);
            app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
            app.plan.lock = if right { Lock::Password } else { Lock::Chip };
            app.plan.locale = "en_GB.UTF-8".into();
            app.disk_one = Zeroizing::new("river stone".into());
            app.disk_two = Zeroizing::new(if right { "river stone" } else { "river" }.into());
            app.full = "John Smith".into();
            app.user = if right { "john" } else { "Bad Name" }.into();
            app.host = "laptop".into();
            app.one = Zeroizing::new("correct horse".into());
            app.two = Zeroizing::new(if right { "correct horse" } else { "correct horse battery" }.into());
            app.share = 0.4;
            app.doing = "Copying the system".into();
            for (page, name) in pages().iter().enumerate() {
                app.page = page;
                app.roll_at.set(page as f32);
                app.armed = page == READY && !right;
                for slide in [0, 3] {
                    app.slide = slide;
                    drawn::window(&format!("{name}, {}, slide {slide}", if right { "typed right" } else { "typed wrong" }), app.view(), Theme::for_page("installer"), SIZE, false);
                    if page != RUNNING {
                        break;
                    }
                }
            }
            app.page = RUNNING;
            app.fail("The installation stopped.");
            drawn::window("failed", app.view(), Theme::for_page("installer"), SIZE, false);
        }
    }));
}

#[test]
#[ignore]
fn pictures_of_every_page() {
    let Some(dir) = std::env::var_os("INSTALLER_SHOTS").map(std::path::PathBuf::from) else { return };
    let _ = std::fs::create_dir_all(&dir);
    let events = Loop::new();
    let mut app = installer(&events);
    app.plan.target = Some(Where::Erase { disk: "/dev/sda".into() });
    app.plan.lock = Lock::Password;
    app.plan.locale = "en_GB.UTF-8".into();
    app.disk_one = Zeroizing::new("river stone".into());
    app.disk_two = Zeroizing::new("river stone".into());
    app.full = "John Smith".into();
    app.user = "john".into();
    app.host = "laptop".into();
    app.one = Zeroizing::new("correct horse".into());
    app.two = Zeroizing::new("correct horse".into());
    app.share = 0.4;
    app.doing = "Copying the system".into();
    for (page, name) in pages().iter().enumerate() {
        app.page = page;
        app.roll_at.set(page as f32);
        for slide in [0, 3] {
            app.slide = slide;
            let mut ui = Ui::new(app.view(), Theme::chosen());
            let (w, h) = (SIZE.0, SIZE.1);
            let mut px = tiny_skia::Pixmap::new(w, h).expect("pixmap");
            ui.paint(&mut px.as_mut(), 1.0);
            let rgba: Vec<u8> = px.pixels().iter().flat_map(|p| {
                let c = p.demultiply();
                [c.red(), c.green(), c.blue(), c.alpha()]
            }).collect();
            let file = dir.join(format!("installer-{page}-{}-{slide}.png", name.to_lowercase()));
            image::save_buffer_with_format(file, &rgba, w, h, image::ExtendedColorType::Rgba8, image::ImageFormat::Png)
                .expect("saved");
            if page != RUNNING {
                break;
            }
        }
    }
}

#[test]
fn a_list_search_finds_words_inside_names() {
    let items = vec![("en_GB".to_string(), "English (United Kingdom)".to_string()),
                     ("fr_FR".to_string(), "French (France)".to_string())];
    assert_eq!(matching(&items, "kingdom").len(), 1);
    assert_eq!(matching(&items, "").len(), 2);
}

#[test]
fn the_disk_password_counts_only_when_typed_twice_the_same() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Lock(Lock::Password));
    app.page = DISK_PASSWORD;
    app.update(Msg::DiskOne("river stone".to_string().into()));
    assert!(app.missing().is_some());
    app.update(Msg::DiskTwo("river stone".to_string().into()));
    assert_eq!(app.plan.disk_password, "river stone");
    assert!(app.missing().is_none());
}

#[test]
fn the_sign_in_name_follows_the_name_until_edited() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Full("John Smith".into()));
    assert_eq!(app.plan.username, "john");
    app.update(Msg::User("js".into()));
    app.update(Msg::Full("Jane Smith".into()));
    assert_eq!(app.plan.username, "js");
}

#[test]
fn install_asks_twice_and_sharing_a_disk_picks_its_option() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Leave("/dev/sda2".into(), 100.0 * plan::GIB as f32));
    assert_eq!(app.plan.target, Some(Where::Beside { part: "/dev/sda2".into(), leave: 100 * plan::GIB }));
    app.page = READY;
    app.update(Msg::Terms(true));
    app.update(Msg::Install);
    assert!(app.armed && app.page == READY);
    app.update(Msg::Disarm(app.arms));
    assert!(!app.armed);

    app.page = 2;
    app.update(Msg::Next);
    assert_eq!(app.page, 4);
    app.update(Msg::Back);
    assert_eq!(app.page, 2);
}

#[test]
fn the_wipe_comes_first_and_past_it_there_is_stop_and_no_back() {
    let events = Loop::new();
    let mut app = installer(&events);
    const { assert!(READY < ABOUT_YOU && DISK_PASSWORD < READY) };
    let (helper, _lines) = std::sync::mpsc::channel();
    app.helper = Some(helper);
    app.share = 0.4;
    app.doing = "Copying the system".into();
    app.page = ABOUT_YOU;
    let said = app.foot().said();
    for word in ["Installing", "40%", "Copying the system", "button: Stop"] {
        assert!(said.contains(&word.to_string()), "{word}: {said:?}");
    }
    assert!(!said.contains(&"button: Back".to_string()), "{said:?}");
    app.page = READY;
    app.helper = None;
    assert!(app.foot().said().contains(&"button: Back".to_string()));
}

#[test]
fn stop_asks_once_then_forgets_what_was_typed() {
    let events = Loop::new();
    let mut app = installer(&events);
    let (helper, lines) = std::sync::mpsc::channel::<zeroize::Zeroizing<String>>();
    app.helper = Some(helper);
    app.page = ABOUT_YOU;
    app.update(Msg::One("river stone".to_string().into()));
    app.update(Msg::DiskOne("river stone".to_string().into()));
    app.update(Msg::Stop);
    assert!(app.armed && app.helper.is_some());
    app.update(Msg::Stop);
    assert!(app.helper.is_none() && app.stopping && !app.armed);
    assert!(lines.recv().is_err(), "the helper's input is still open");
    app.update(Msg::Stopped(true));
    assert_eq!(app.page, 1);
    assert!(app.plan.password.is_empty() && app.plan.disk_password.is_empty() && app.one.is_empty() && app.disk_one.is_empty());
    assert!(app.plan.target.is_none() && !app.stuck);
    app.page = ABOUT_YOU;
    app.update(Msg::Stopped(false));
    assert!(app.stuck && app.failed);
    let said = app.foot().said();
    assert!(said.iter().any(|s| s.contains("Restart")), "{said:?}");
    assert!(!said.contains(&"button: Back".to_string()), "{said:?}");
}

#[test]
fn a_failed_copy_stays_on_the_page_and_goes_back_to_the_wipe() {
    let events = Loop::new();
    let mut app = installer(&events);
    let (helper, _lines) = std::sync::mpsc::channel();
    app.helper = Some(helper);
    app.page = ABOUT_YOU;
    app.update(Msg::Finished(Err("no room left".into())));
    assert_eq!(app.page, ABOUT_YOU);
    assert!(app.foot().said().contains(&"no room left".to_string()));
    app.update(Msg::Back);
    assert_eq!(app.page, READY);
}

#[test]
fn install_stays_off_until_the_terms_are_ticked_and_stop_forgets_the_tick() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Erase("/dev/sda".into()));
    app.page = READY;
    let install = |app: &Installer| app.foot().buttons().into_iter().find(|(tip, _)| tip == "Install").map(|(_, on)| on);
    assert_eq!(install(&app), Some(false));
    assert!(app.foot().said().contains(&"Agreement required".to_string()));
    app.update(Msg::Install);
    assert!(!app.armed && !app.plan.terms);
    app.update(Msg::Terms(true));
    assert_eq!(install(&app), Some(true));
    assert!(app.plan.terms && !app.foot().said().contains(&"Agreement required".to_string()));
    app.update(Msg::Install);
    assert!(app.armed);
    app.update(Msg::Terms(false));
    assert!(!app.armed && install(&app) == Some(false));
    app.update(Msg::Terms(true));
    let (helper, _lines) = std::sync::mpsc::channel();
    app.helper = Some(helper);
    app.page = ABOUT_YOU;
    app.update(Msg::Stopped(true));
    assert!(!app.terms && !app.plan.terms && app.page == 1);
    app.page = READY;
    assert_eq!(install(&app), Some(false));
}

#[test]
fn the_word_eula_opens_the_text_and_back_shuts_it() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = READY;
    assert!(app.body().said().contains(&"tick: I agree to the EULA".to_string()));
    let reads = |app: &Installer| app.body().said().iter().any(|s| s.contains("1. Parties"));
    assert!(!reads(&app));
    app.update(Msg::ReadTerms(true));
    assert!(reads(&app));
    assert!(app.body().said().contains(&"tick: I agree to the EULA".to_string()));
    app.update(Msg::ReadTerms(false));
    assert!(!reads(&app));
}

#[test]
fn what_is_kept_never_leaves_trindle_less_than_its_room() {
    let part = plan::Part { dev: "/dev/sda2".into(), size: 100 * plan::GIB, used: 70 * plan::GIB, fstype: "ext4".into(), ..plan::Part::default() };
    assert!(part.shrinkable());
    assert_eq!(part.most_kept(), 100 * plan::GIB - plan::LEAST);
    assert!(super::pages::half_way(&part) <= part.most_kept(), "even the first offer leaves the room");
}

#[test]
fn enter_twice_on_install_confirms_without_a_shift_tab() {
    use crate::toolkit::input::{Input, Keysym, Mods};
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = READY;
    app.update(Msg::Terms(true));
    let mut ui = Ui::new(app.view(), Theme::chosen());
    ui.lay_at(SIZE.0, SIZE.1);
    ui.keyboard_to("installer:install");
    for armed in [false, true] {
        assert_eq!(app.armed, armed);
        let reply = ui.input(&Input::Key { sym: Keysym::Return, text: None, mods: Mods::default(), repeat: false }, std::time::Instant::now());
        assert!(matches!(reply.messages.as_slice(), [Msg::Install]), "Enter presses Install (armed {armed})");
        if armed {
            break;
        }
        app.update(Msg::Install);
        ui.set_view(app.view());
        ui.lay_at(SIZE.0, SIZE.1);
        assert!(app.armed);
        assert_eq!(ui.keyboard_on().as_deref(), Some("installer:install"), "the keyboard stays on the pill once armed");
    }
    app.update(Msg::Disarm(app.arms));
    ui.set_view(app.view());
    ui.lay_at(SIZE.0, SIZE.1);
    assert_eq!(ui.keyboard_on().as_deref(), Some("installer:install"), "and when it disarms again");
}

#[test]
fn every_page_fits_the_window_and_reads_from_the_right_in_arabic_and_urdu() {
    for tag in ["ar", "ur"] {
        in_language(tag, || {
            let events = Loop::new();
            let mut app = installer(&events);
            app.plan.lock = Lock::Password;
            app.problems = vec![t!("installer-snapshots-failed", why = "snapper").to_string()];
            let mut tall = Vec::new();
            for (page, name) in pages().iter().enumerate() {
                app.page = page;
                for beside in [false, true] {
                    app.plan.target = Some(if beside { Where::Beside { part: "/dev/sda2".into(), leave: 100 * plan::GIB } }
                                           else { Where::Erase { disk: "/dev/sda".into() } });
                    let mut ui = Ui::new(app.view(), Theme::chosen());
                    ui.lay_at(SIZE.0, SIZE.1);
                    let boxes = ui.boxes();
                    let room = boxes.iter().find(|b| ui.places().iter().any(|p| p.0 == b.0 && b.0.ends_with(&format!("page-{page}"))));
                    match room {
                        Some((_, r, _)) if r.y + r.h <= SIZE.1 as f32 - 40.0 => {}
                        other => tall.push(format!("{tag} {name} beside {beside}: page area {other:?}")),
                    }
                }
                let said = app.body().said();
                assert!(said.iter().any(|w| crate::toolkit::edit::right_to_left(w)), "{tag} {name} says nothing in {tag}: {said:?}");
            }
            assert!(tall.is_empty(), "{tall:?}");
        });
    }
}

#[test]
fn a_computer_with_too_little_memory_cannot_go_past_the_first_page_and_says_why() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.plan.locale = "en_GB.UTF-8".into();
    app.plan.trindle_language = "en".into();
    app.plan.layout = "gb".into();
    app.plan.timezone = "Europe/London".into();
    assert_eq!(app.missing(), None);
    app.machine.short_of_memory = true;
    assert_eq!(app.missing(), Some("Trindle needs at least 1 GB of memory, and this computer has less."));
}

#[test]
fn a_disk_is_not_encrypted_on_a_computer_that_started_the_old_bios_way() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = 2;
    app.machine.uefi = false;
    assert_eq!(app.missing(), None, "no encryption is fine");
    app.plan.lock = Lock::Password;
    assert!(app.missing().is_some_and(|w| w.contains("UEFI")));
    app.machine.uefi = true;
    assert_eq!(app.missing(), None);
}

#[test]
fn what_the_install_went_on_without_is_said_on_the_last_page() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = DONE;
    assert!(!app.body().said().iter().any(|w| w.contains("snapshots")));
    app.update(Msg::Line("! Trindle is installed, but snapshots could not be set up: snapper failed".into()));
    app.update(Msg::Line("70 Settings".into()));
    assert!(app.body().said().iter().any(|w| w == "Trindle is installed, but snapshots could not be set up: snapper failed"), "{:?}", app.body().said());
    app.stopped(true);
    assert!(app.problems.is_empty());
}

#[test]
fn sharing_a_disk_says_to_back_up_first_before_it_is_confirmed() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.page = READY;
    app.plan.target = Some(Where::Beside { part: "/dev/sda2".into(), leave: 100 * plan::GIB });
    let (words, tone) = ready_sentence(&app.plan.target, None);
    assert!(words.contains("back it up first") && tone == Tone::Warn, "{words}");
    assert!(app.body().said().iter().any(|w| w == &words), "{:?}", app.body().said());
}

#[test]
fn a_changed_disk_reads_the_disks_again_and_only_then() {
    use std::os::unix::process::ExitStatusExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static READS: AtomicUsize = AtomicUsize::new(0);
    let exit = |code: i32| Ok(std::process::ExitStatus::from_raw(code << 8));
    assert!(matches!(actions::ended(exit(steps::CHANGED), "changed".into()), Msg::DiskChanged(w) if w == "changed"));
    assert!(matches!(actions::ended(exit(1), "no room left".into()), Msg::Finished(Err(w)) if w == "no room left"));
    assert!(matches!(actions::ended(exit(steps::STOPPED), String::new()), Msg::Stopped(true)));

    let events = Loop::new();
    let mut app = installer(&events);
    let got: Rc<RefCell<Vec<Msg>>> = Rc::default();
    let (kept, ev) = (got.clone(), events.clone());
    app.asks = events.post(move |m: Msg| {
        kept.borrow_mut().push(m);
        ev.quit();
    });
    app.read_disks = || {
        READS.fetch_add(1, Ordering::SeqCst);
        let part = plan::Part { dev: "/dev/sdb2".into(), size: 400 * plan::GIB, used: 60 * plan::GIB, fstype: "ntfs".into(), ..plan::Part::default() };
        vec![Disk { dev: "/dev/sdb".into(), model: "Example disk two".into(), size: 500 * plan::GIB, parts: vec![part], ..Disk::default() }]
    };
    app.update(Msg::Erase("/dev/sda".into()));
    app.page = ABOUT_YOU;
    app.update(Msg::Finished(Err("no room left".into())));
    assert!(!app.rereading && READS.load(Ordering::SeqCst) == 0, "no other failure reads the disks again");
    let words = t!("installer-disk-changed", dev = "/dev/sda");
    app.update(Msg::DiskChanged(words.clone()));
    assert!(app.failed && app.rereading && app.plan.target.is_none() && app.plan.disk_id.is_empty());
    assert!(app.foot().said().contains(&words), "{:?}", app.foot().said());
    app.page = 1;
    assert_eq!(app.missing(), Some(t!("installer-disks-rereading")));
    app.update(Msg::DiskChanged(words));
    let ev = events.clone();
    events.after(Duration::from_secs(30), move || ev.quit());
    events.run();
    let read = got.borrow_mut().pop().expect("the disks were read again");
    app.update(read);
    assert_eq!(READS.load(Ordering::SeqCst), 1, "read once, however often it was refused meanwhile");
    assert!(!app.rereading && app.machine.disks.len() == 1 && app.machine.disks[0].dev == "/dev/sdb");
    assert!(app.leave.contains_key("/dev/sdb2") && !app.leave.contains_key("/dev/sda2"));
    assert_eq!(app.missing(), Some(t!("installer-missing-disk")));
    app.update(Msg::Erase("/dev/sdb".into()));
    assert!(app.plan.target.is_some() && app.missing().is_none());
}

fn names_shown(app: &Installer) -> Vec<String> {
    let mut ui = Ui::new(app.view(), Theme::chosen());
    ui.lay_at(SIZE.0, SIZE.1);
    ui.access("Installer").nodes.iter().filter_map(|(_, n)| n.label().map(str::to_string)).collect()
}

#[test]
fn picking_a_language_shows_the_installer_in_it_at_once() {
    use crate::common::live_language::switched;
    let events = Loop::new();
    let mut app = installer(&events);
    app.machine.tongues = plan::trindle_languages();
    app.machine.locales.extend([("ar_EG.UTF-8".into(), "Arabic (Egypt)".into()), ("tlh.UTF-8".into(), "Klingon".into())]);
    let at = |app: &Installer, code: &str| matching(&app.machine.locales, "").iter().position(|(k, _)| k == code).unwrap();
    assert_eq!(app.window_title(), None, "nothing to tell before a language is picked");
    for (code, tag, next) in [("fr_FR.UTF-8", "fr", "Suivant"), ("ar_EG.UTF-8", "ar", "التالي")] {
        app.update(Msg::Pick(0, at(&app, code)));
        assert_eq!(app.plan.locale, code);
        let tags = switched().unwrap_or_else(|| panic!("{code}: the words were not switched"));
        assert_eq!(tags, [tag], "{code}");
        in_language(tag, || {
            assert_eq!(app.window_title(), Some(t!("installer-window-title", name = "Trindle")), "{code}");
            assert_eq!(app.window_title(), None, "the title is told once");
            assert!(names_shown(&app).iter().any(|n| n == next), "{code}: {:?}", names_shown(&app));
        });
        if tag == "ar" {
            assert!(crate::toolkit::edit::right_to_left(next), "Arabic is laid right to left");
        }
    }
    app.update(Msg::Pick(0, at(&app, "en_GB.UTF-8")));
    assert_eq!(switched(), None, "built-in English is in use in tests, so nothing changes");
    app.update(Msg::Pick(0, at(&app, "tlh.UTF-8")));
    assert_eq!(switched(), None, "no words for it: English stays");
}

#[test]
fn trindle_s_language_is_its_own_choice_from_the_languages_it_has() {
    use crate::common::live_language::switched;
    use super::{LANGUAGE, TONGUE};
    let events = Loop::new();
    let mut app = installer(&events);
    app.machine.tongues = plan::trindle_languages();
    let codes: Vec<&str> = app.machine.tongues.iter().map(|(c, _)| c.as_str()).collect();
    for has in ["en", "en-US", "fr", "de", "ar", "zh-Hans"] {
        assert!(codes.contains(&has), "{has}: {codes:?}");
    }
    assert!(!codes.contains(&"af"), "no words for Afrikaans, so it is not offered");
    app.machine.locales.push(("af_ZA.UTF-8".into(), "Afrikaans (South Africa)".into()));
    let at = |items: &[(String, String)], code: &str| matching(items, "").iter().position(|(k, _)| k == code).unwrap();
    assert_eq!(app.missing(), Some(t!("installer-missing-language")));
    app.update(Msg::Pick(LANGUAGE, at(&app.machine.locales, "af_ZA.UTF-8")));
    assert_eq!((app.plan.locale.as_str(), app.plan.trindle_language.as_str()), ("af_ZA.UTF-8", "en"));
    assert_eq!(app.plan.language_setting(), None, "English is what Afrikaans gives anyway");
    app.update(Msg::Pick(TONGUE, at(&app.machine.tongues, "fr")));
    assert_eq!(app.plan.locale, "af_ZA.UTF-8", "the computer's language stays");
    assert_eq!(switched().as_deref(), Some(&["fr".to_string()][..]), "the installer is shown in French");
    assert_eq!(app.plan.language_setting().as_deref(), Some("language=fr"));
    assert_eq!(app.plan.words_in(), "fr", "the helper speaks French too");
    app.update(Msg::Pick(LANGUAGE, at(&app.machine.locales, "fr_FR.UTF-8")));
    assert_eq!(app.plan.trindle_language, "fr");
    assert_eq!(app.plan.language_setting(), None, "French follows from French, so Trindle keeps following the system");
    app.plan.trindle_language.clear();
    assert_eq!(app.missing(), Some(t!("installer-missing-trindle-language")));
}

#[test]
fn a_short_window_keeps_room_for_the_lists() {
    let events = Loop::new();
    let mut app = installer(&events);
    let lists = |app: &Installer| Ui::new(app.view(), Theme::chosen()).natural_height(SIZE.0 as f32);
    let tall = lists(&app);
    assert!(app.resized(SIZE.0 as f32, 600.0), "going short lays the page out again");
    assert!(lists(&app) > tall, "the lists keep their room, so the page grows and scrolls");
    assert!(!app.resized(SIZE.0 as f32, 620.0), "still short: nothing to lay out again");
    assert!(app.resized(SIZE.0 as f32, 1000.0));
}

#[test]
fn the_armed_pill_reads_sure_in_the_laid_out_window() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Erase("/dev/sda".into()));
    app.page = READY;
    app.update(Msg::Terms(true));
    let mut ui = Ui::new(app.view(), Theme::chosen());
    ui.lay_at(SIZE.0, SIZE.1);
    app.update(Msg::Install);
    ui.set_view(app.view());
    ui.lay_at(SIZE.0, SIZE.1);
    let words: Vec<String> = ui.worded_buttons().into_iter().map(|b| b.0).collect();
    assert!(words.contains(&"Sure?".to_string()) && !words.contains(&"Install".to_string()), "{words:?}");
}

#[test]
fn a_disk_with_no_model_name_is_named_once() {
    let disk = Disk { dev: "/dev/vda".into(), size: 40 * plan::GIB, ..Disk::default() };
    assert_eq!(disk.named().matches("/dev/vda").count(), 1, "{}", disk.named());
    let (words, _) = ready_sentence(&Some(Where::Erase { disk: "/dev/vda".into() }), Some(&disk));
    assert_eq!(words.matches("/dev/vda").count(), 1, "{words}");
}

#[test]
fn the_ready_page_lists_the_swap_file_only_when_it_is_on() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.update(Msg::Erase("/dev/sda".into()));
    app.page = READY;
    for on in [false, true] {
        app.update(Msg::Swap(on));
        assert_eq!(app.view().said().iter().any(|w| w.contains("swap file")), on, "swap {on}");
    }
}

#[test]
fn without_a_tpm_chip_the_chip_choice_is_off_with_its_reason() {
    let events = Loop::new();
    let mut app = installer(&events);
    app.machine.chip = false;
    app.page = 2;
    app.update(Msg::Lock(Lock::None));
    let view = app.privacy();
    assert!(view.said().iter().any(|w| w.starts_with("Needs a TPM chip")), "{:?}", view.said());
    let on = view.buttons().into_iter().find(|(tip, _)| tip.contains("TPM chip unlock")).map(|(_, on)| on);
    assert_eq!(on, Some(false), "the chip choice must be off");
    assert!(!view.said().iter().any(|w| w.starts_with("This computer has") && w.contains("password")), "{:?}", view.said());
    app.update(Msg::Lock(Lock::Password));
    assert!(app.privacy().said().iter().any(|w| w.contains("ask for the password every time")), "{:?}", app.privacy().said());
}
