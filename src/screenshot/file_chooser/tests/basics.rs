//! Tests: basics.

#[cfg(test)]
use super::*;

#[test]
fn a_files_size_and_day_follow_the_language() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

    let long_ago = 1_405_000_000;
    assert_eq!(crate::common::kinds::when_text(long_ago), "10 Jul 2014");
    let made_up = "kit-date-month-short-jul = Jlo\nkit-date-pattern-day-short-year = %Y/%b/%-d\nkit-number-decimal = ,\nkit-size-kb = { $size } Ko\n";
    with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
        assert_eq!(crate::common::kinds::when_text(long_ago), "2014/Jlo/10");
        assert_eq!(crate::disks::size_text(4_500.0), "4,5 Ko");
    });
}

#[test]
fn a_chooser_with_a_window_looks_at_the_disk_only_on_threads() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let dir = std::env::temp_dir().join(format!("trindle-chooser-threads-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Albums")).unwrap();
    std::fs::write(dir.join("notes.txt"), b"made up").unwrap();
    let ask = Ask { title: "Made up".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                    folder: Some(dir.clone()), name: "notes.txt".into(), app_id: "org.example.Paint".into() };
    let chosen: Rc<RefCell<Option<Outcome>>> = Rc::default();
    let told = chosen.clone();
    let mut c = Chooser::new(ask, move |outcome| *told.borrow_mut() = Some(outcome));

    c.kept = std::env::temp_dir().join(format!("trindle-chooser-threads-kept-{}", std::process::id()));

    assert!(!c.listed && c.entries.is_empty() && c.looks.is_empty());
    assert_eq!((c.at.clone(), c.places.len()), (Where::Folder(dir.clone()), 3));

    let events = crate::toolkit::event::Loop::new();
    let heard: Rc<RefCell<Vec<Msg>>> = Rc::default();
    let hears = heard.clone();
    c.set_post(events.post(move |m: Msg| hears.borrow_mut().push(m)));
    assert!(!c.listed && c.entries.is_empty(), "the folder was read on the window's own loop");

    let turn = |c: &mut Chooser, done: &dyn Fn(&Chooser) -> bool| {
        for _ in 0..50 {
            if done(c) {
                return true;
            }
            let waiting = heard.clone();
            crate::toolkit::bus::testing::until(&events, move || !waiting.borrow().is_empty());
            for msg in heard.take() {
                c.update(msg);
            }
        }
        done(c)
    };
    assert!(turn(&mut c, &|c| c.listed), "the folder was never listed");
    assert_eq!(c.entries.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["Albums", "notes.txt"]);
    assert_eq!((c.places[0].label.as_str(), c.places[1].label.as_str(), c.places.last().map(|p| p.label.as_str())), ("Recent", "Home", Some("System disk")));
    assert!(c.watched.contains(&dir), "the folder shown is not watched");

    assert!(turn(&mut c, &|c| c.taken().is_some()), "the name typed was never looked at");
    assert_eq!(c.accept_words(), "Replace");

    c.update(Msg::Changed);
    c.update(Msg::Changed);
    assert_eq!((c.looks.len(), c.again), (1, true), "a place out being read was sent for again");
    assert!(turn(&mut c, &|c| c.looks.is_empty() && !c.again));
    into_a_folder(&mut c, &dir, &turn);
    a_folder_made_and_a_save_held_back_and_made(&mut c, &dir, &chosen, &turn);
    let _ = std::fs::remove_file(&c.kept);
    std::fs::remove_dir_all(&dir).unwrap();
}

type Turn<'a> = &'a dyn Fn(&mut Chooser, &dyn Fn(&Chooser) -> bool) -> bool;

fn into_a_folder(c: &mut Chooser, dir: &std::path::Path, turn: Turn) {

    let albums = c.entries.iter().position(|e| e.folder).unwrap();
    c.update(Msg::Open(albums));
    assert_eq!(c.at, Where::Folder(dir.join("Albums")));
    assert!(!c.listed && c.entries.is_empty());
    assert!(turn(c, &|c| c.listed) && c.entries.is_empty());
}

fn a_folder_made_and_a_save_held_back_and_made(c: &mut Chooser, dir: &std::path::Path, chosen: &std::rc::Rc<std::cell::RefCell<Option<Outcome>>>, turn: Turn) {

    c.update(Msg::NewFolder);
    assert!(c.renaming.is_none() && !dir.join("Albums/New folder").exists(), "the folder was made on the window's own loop");
    assert!(turn(c, &|c| c.renaming.is_some() && c.listed && c.entries.iter().any(|e| e.name == "New folder")));
    c.update(Msg::RenameText("Summer".into()));
    c.update(Msg::RenameDone);
    assert_eq!(c.at, Where::Folder(dir.join("Albums")), "the folder was renamed on the window's own loop");
    assert!(turn(c, &|c| c.at == Where::Folder(dir.join("Albums/Summer")) && c.listed));
    assert!(dir.join("Albums/Summer").is_dir());
    c.update(Msg::Name(dir.join("notes.txt").to_string_lossy().into_owned()));
    c.update(Msg::Submit);
    assert!(c.said.is_empty() && chosen.borrow().is_none());
    assert!(turn(c, &|c| !c.said.is_empty()), "a save over a file was not held back");
    assert!(c.said.contains("already there") && chosen.borrow().is_none());
    c.update(Msg::Submit);
    assert!(turn(c, &|_| chosen.borrow().is_some()), "the second Enter did not save");
    assert_eq!(*chosen.borrow(), Some(Outcome::Chosen { uris: vec![file_uri(&dir.join("notes.txt"))], choices: Vec::new(), filter: None }));
}

#[test]
fn every_mode_has_one_line_rows_at_the_compact_level_and_no_folder_is_called_one() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-compact-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Albums")).unwrap();
    std::fs::write(dir.join("notes.txt"), b"made up").unwrap();
    let room = theme::compact_room();
    for mode in [Mode::Save, Mode::Open { many: false, folder: false }, Mode::Open { many: true, folder: false },
                 Mode::Open { many: false, folder: true }, Mode::SaveMany(vec!["a.png".into()])] {
        let ask = Ask { title: "Made up".into(), accept: None, mode: mode.clone(), filters: Vec::new(), filter: None, choices: Vec::new(),
                        folder: Some(dir.clone()), name: String::new(), app_id: "org.example.Paint".into() };
        let c = Chooser::looked(ask, |_| {});
        let folder = c.entries.iter().find(|e| e.folder).expect("the folder is listed");
        let row = c.row(folder);
        assert!(row.under.is_none(), "{mode:?}: one line");

        assert!(row.folder && c.entries.iter().filter(|e| !e.folder).all(|e| !c.row(e).folder), "{mode:?}");
        for p in &c.places {
            let is = p.icon.starts_with("folder-") || p.icon == "user-home-symbolic";
            assert_eq!(p.folder, is, "{mode:?}: {}", p.label);
        }
        assert!(c.places.iter().any(|p| p.folder) && c.places.iter().any(|p| !p.folder), "{mode:?}");
        assert!(!format!("{} {}", row.text, row.detail.clone().unwrap_or_default()).to_lowercase().contains("folder"), "{mode:?}");
        if let Some(file) = c.entries.iter().find(|e| !e.folder) {
            let row = c.row(file);
            assert!(row.under.is_none() && row.detail.is_some_and(|d| d.starts_with("7 bytes")), "{mode:?}: a file's size is at the far end");
        }
        fn heights<M>(n: &Node<M>, out: &mut Vec<f32>) {
            if let crate::toolkit::widget::What::List { row, .. } = &n.what {
                out.push(*row);
            }
            for i in n.items() {
                heights(i, out);
            }
        }
        let mut tall = Vec::new();
        heights(&c.view(), &mut tall);
        assert_eq!(tall, [room.side_row, room.row], "{mode:?}: the places, then the folder's contents");
    }

    let before = theme::sheet().rich_row;
    assert!((8.0 * before / room.row).floor() >= 16.0, "{} against {before}", room.row);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_main_button_says_replace_while_the_name_typed_is_taken() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-replace-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Albums")).unwrap();
    std::fs::write(dir.join("notes.txt"), b"the old words").unwrap();
    std::fs::write(dir.join("list.txt"), b"the old list").unwrap();
    for accept in [None, Some("_Export".to_string())] {
        let save = accept.as_deref().map_or("Save".to_string(), |a| a.replace('_', ""));
        let ask = Ask { title: "Made up".into(), accept: accept.clone(), mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                        folder: Some(dir.clone()), name: String::new(), app_id: "org.example.Paint".into() };
        let mut c = Chooser::looked(ask, |_| {});
        let already = |c: &Chooser| c.view().said().into_iter().filter(|w| w.contains("already there")).collect::<Vec<_>>();
        assert_eq!(main_button(&c), (save.clone(), false), "no name yet");
        for (typed, words) in [("n", &save), ("notes.tx", &save), ("notes.txt", &"Replace".to_string()), ("notes.txt2", &save), ("Albums", &save),
                               ("list.txt", &"Replace".to_string()), (" list.txt ", &"Replace".to_string())] {
            c.update(Msg::Name(typed.into()));
            assert_eq!(main_button(&c), (words.clone(), true), "{accept:?}: {typed:?} typed");
            let taken = *words == "Replace";
            assert_eq!(already(&c), if taken { vec![format!("{} is already there.", typed.trim())] } else { Vec::new() }, "{accept:?}: {typed:?} typed");
        }

        c.update(Msg::Name("new.txt".into()));
        let at = c.entries.iter().position(|e| e.name == "notes.txt").unwrap();
        c.update(Msg::Choose(at));
        assert_eq!((c.name.as_str(), main_button(&c)), ("notes.txt", ("Replace".to_string(), true)), "{accept:?}");
    }

    let ask = Ask { title: "Made up".into(), accept: None, mode: Mode::Open { many: false, folder: false }, filters: Vec::new(), filter: None,
                    choices: Vec::new(), folder: Some(dir.clone()), name: "notes.txt".into(), app_id: "org.example.Paint".into() };
    let mut c = Chooser::looked(ask, |_| {});
    let at = c.entries.iter().position(|e| e.name == "notes.txt").unwrap();
    c.update(Msg::Choose(at));
    assert_eq!(main_button(&c), ("Open".to_string(), true));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn replace_is_one_press_on_its_button_and_two_by_enter_or_a_double_click() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let dir = std::env::temp_dir().join(format!("trindle-chooser-replacing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("in")).unwrap();
    std::fs::write(dir.join("in/notes.txt"), b"the old words").unwrap();
    let start = |name: &str| {
        let chosen: Rc<RefCell<Option<Outcome>>> = Rc::default();
        let ask = Ask { title: "Made up".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                        folder: Some(dir.join("in")), name: name.into(), app_id: "org.example.Paint".into() };
        let heard = chosen.clone();
        let mut c = Chooser::looked(ask, move |o| *heard.borrow_mut() = Some(o));

        c.kept = dir.join("folders");
        (c, chosen)
    };
    let saved = |chosen: &Rc<RefCell<Option<Outcome>>>| match &*chosen.borrow() {
        Some(Outcome::Chosen { uris, .. }) => uris.iter().filter_map(|u| uri_path(u)).collect(),
        _ => Vec::new(),
    };
    let there = vec![dir.join("in/notes.txt")];

    let (mut c, chosen) = start("notes.txt");
    assert_eq!(main_button(&c).0, "Replace");
    c.update(Msg::Accept);
    assert_eq!(saved(&chosen), there);

    let (mut c, chosen) = start("notes.txt");
    let _ = c.view();
    c.update(Msg::Submit);
    assert!(chosen.borrow().is_none() && c.view().said().iter().any(|w| w.starts_with("notes.txt is already there. Press Replace")), "{:?}", c.view().said());
    assert_eq!(main_button(&c).0, "Replace");
    c.update(Msg::Submit);
    assert_eq!(saved(&chosen), there);

    let (mut c, chosen) = start("notes.txt");
    c.update(Msg::Submit);
    c.update(Msg::Accept);
    assert_eq!(saved(&chosen), there);

    let (mut c, chosen) = start("new.txt");
    let at = c.entries.iter().position(|e| e.name == "notes.txt").unwrap();
    c.update(Msg::Open(at));
    assert!(chosen.borrow().is_none() && c.name == "notes.txt");

    c.update(Msg::Name("notes.txt".into()));
    c.update(Msg::Submit);
    assert!(chosen.borrow().is_none(), "held back afresh after the name was typed again");

    let (mut c, chosen) = start("later.txt");
    assert_eq!(main_button(&c).0, "Save");
    std::fs::write(dir.join("in/later.txt"), b"came since").unwrap();
    c.update(Msg::Accept);
    assert!(chosen.borrow().is_none());

    let (mut c, chosen) = start("fresh.txt");
    c.update(Msg::Submit);
    assert_eq!(saved(&chosen), [dir.join("in/fresh.txt")]);
    assert_eq!(folders_read(&dir.join("folders")), format!("org.example.Paint\t{}\n", dir.join("in").display()));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_folders_last_chosen_in_are_whole_as_they_were_when_they_cannot_be_written() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-kept-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("folders");
    keep_folder(&file, "org.example.Paint", Path::new("/made/up"));
    crate::common::files::FULL.set(true);
    keep_folder(&file, "org.example.Write", Path::new("/made/up/too"));
    crate::common::files::FULL.set(false);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "org.example.Paint\t/made/up\n");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1, "nothing is left beside it");
    keep_folder(&file, "org.example.Write", Path::new("/made/up/too"));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "org.example.Write\t/made/up/too\norg.example.Paint\t/made/up\n");
    std::fs::remove_dir_all(&dir).unwrap();
}
