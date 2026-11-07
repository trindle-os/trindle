//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn the_right_click_menu_has_folders_lines_on_a_thing_and_two_on_empty_room() {
    for mode in ALL_MODES {
        let (dir, mut c) = menu_chooser("lines", mode());
        let what = format!("{:?}", mode());
        let i = at(&c, "Letters");
        c.update(Msg::Menu(i, 300.0, 200.0));
        assert_eq!(lines(&c), ["Rename", "Copy", "Copy location", "Cut", "Paste", "New folder", "Pin to the sidebar", "Move to Trash", "Delete permanently"], "{what}");
        c.update(Msg::Pop(PopupMsg::Close));
        if let Some(f) = c.entries.iter().position(|e| e.name == "list.txt") {
            c.update(Msg::Menu(f, 300.0, 200.0));
            assert_eq!(lines(&c), ["Rename", "Copy", "Copy location", "Cut", "Paste", "New folder", "Move to Trash", "Delete permanently"], "{what}: a file is not pinned");
            c.update(Msg::Pop(PopupMsg::Close));
        }
        c.update(Msg::Menu(i, 300.0, 200.0));
        let key = |sym| Input::Key { sym, text: None, mods: Default::default(), repeat: false };
        let nowhere = Keyboard { keys: true, text: false, arrows: true };
        assert!(matches!(c.intercept_with(&key(Keysym::Down), nowhere), Some(Msg::Pop(PopupMsg::Move(1)))), "{what}");
        assert!(matches!(c.intercept_with(&key(Keysym::Return), nowhere), Some(Msg::Pop(PopupMsg::PickHovered))), "{what}");
        c.update(Msg::Pop(popup::key(Keysym::Escape)));
        assert!(c.menu.is_none(), "{what}: Escape closes it");
        c.update(Msg::RoomMenu(false));
        assert_eq!(lines(&c), ["Paste", "New folder"], "{what}");
        assert!(c.chosen.is_none() && c.picked.is_empty(), "{what}: nothing is picked on the empty room");
        c.update(Msg::Pop(PopupMsg::Close));
        assert!(matches!(c.intercept_with(&key(Keysym::Menu), nowhere), Some(Msg::RoomMenu(true))), "{what}");
        let shift = crate::toolkit::input::Mods { shift: true, ..Default::default() };
        assert!(matches!(c.intercept_with(&Input::Key { sym: Keysym::F10, text: None, mods: shift, repeat: false }, nowhere), Some(Msg::RoomMenu(true))), "{what}");
        c.update(Msg::RoomMenu(true));
        assert_eq!(c.menu.as_ref().and_then(|(menu, _)| menu.hover), Some(0), "{what}: opened by a key, its first line is lit");
        c.update(Msg::Pop(PopupMsg::Close));
        c.update(Msg::Choose(i));
        assert!(c.intercept_with(&key(Keysym::Menu), nowhere).is_none(), "{what}");
        assert!(matches!(c.intercept_with(&key(Keysym::F2), nowhere), Some(Msg::Rename)), "{what}");
        assert!(c.intercept_with(&key(Keysym::F2), Keyboard { keys: true, text: true, arrows: true }).is_none(), "{what}");
        fn asks<M>(n: &Node<M>) -> bool {
            matches!(&n.what, crate::toolkit::widget::What::List { menu: Some(_), .. }) || n.items().iter().any(|i| asks(i))
        }
        assert!(asks(&c.view()), "{what}");
        c.at = Where::Recent;
        c.entries = vec![Entry { name: "notes.txt".into(), path: dir.join("in/notes.txt"), folder: false, size: 9, when: 0 }];
        c.update(Msg::Menu(0, 300.0, 200.0));
        assert_eq!(lines(&c), ["Copy"], "{what}");
        c.update(Msg::Pop(PopupMsg::Close));
        c.update(Msg::RoomMenu(false));
        assert!(c.menu.is_none(), "{what}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
fn rename_changes_a_name_in_the_list_and_keeps_the_thing_picked() {
    let (dir, mut c) = menu_chooser("rename", Mode::Open { many: false, folder: true });
    let i = at(&c, "Letters");
    c.update(Msg::Menu(i, 300.0, 200.0));
    c.update(Msg::Choose(i));
    pick(&mut c, "Rename");
    assert!(c.view().said().contains(&"entry: New name for Letters".to_string()), "{:?}", c.view().said());
    for (typed, why) in [("Albums", "A folder called Albums is already here."), ("a/b", "A name cannot hold a slash."), ("", "A name cannot be empty."),
                         ("..", "That name is kept for the system.")] {
        c.update(Msg::RenameText(typed.into()));
        c.update(Msg::RenameDone);
        assert!(c.renaming.is_some() && c.view().said().contains(&why.to_string()), "{typed:?}: {:?}", c.view().said());
        assert!(dir.join("in/Letters/one.txt").is_file(), "{typed:?}: nothing is changed");
    }
    c.update(Msg::RenameText("Post".into()));
    assert!(!c.view().said().iter().any(|w| w.starts_with("That name") || w.starts_with("A name")));
    c.update(Msg::RenameDone);
    assert!(c.renaming.is_none() && dir.join("in/Post/one.txt").is_file() && !dir.join("in/Letters").exists());
    let now = at(&c, "Post");
    assert_eq!((c.chosen, c.picked.clone()), (Some(now), vec![now]), "it stays picked under its new name");
    assert!(c.ready());
    c.update(Msg::Rename);
    c.update(Msg::RenameText("Gone".into()));
    c.update(Msg::RenameStop);
    assert!(c.renaming.is_none() && dir.join("in/Post").is_dir());
    c.update(Msg::Rename);
    c.update(Msg::RenameText("Albums".into()));
    c.update(Msg::RenameLeft);
    assert!(c.renaming.is_none() && dir.join("in/Post").is_dir() && dir.join("in/Albums").is_dir());
    c.update(Msg::Rename);
    c.update(Msg::RenameText("Mail".into()));
    c.update(Msg::RenameLeft);
    assert!(dir.join("in/Mail/one.txt").is_file());
    std::fs::remove_dir_all(&dir).unwrap();
    let (dir, mut c) = menu_chooser("rename-save", Mode::Save);
    let i = at(&c, "notes.txt");
    c.update(Msg::Choose(i));
    assert_eq!(c.name, "notes.txt");
    c.update(Msg::Rename);
    c.update(Msg::RenameText("minutes.txt".into()));
    c.update(Msg::RenameDone);
    assert_eq!((c.name.as_str(), std::fs::read(dir.join("in/minutes.txt")).unwrap()), ("minutes.txt", b"the notes".to_vec()));
    assert_eq!(c.chosen, Some(at(&c, "minutes.txt")));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn rename_in_the_menu_is_of_the_row_the_menu_was_opened_on() {
    let (dir, mut c) = menu_chooser("rename-row", Mode::Open { many: true, folder: false });
    let (first, last) = (at(&c, "list.txt"), at(&c, "notes.txt"));
    c.update(Msg::Choose(first));
    c.update(Msg::Pick(vec![first]));
    c.update(Msg::Choose(last));
    c.update(Msg::Pick(vec![first, last]));
    c.update(Msg::Menu(first, 300.0, 200.0));
    pick(&mut c, "Rename");
    assert_eq!(c.renaming.as_ref().map(|r| r.path.clone()), Some(dir.join("in/list.txt")), "the name box opened on another row");
    c.update(Msg::RenameText("shopping.txt".into()));
    c.update(Msg::RenameDone);
    assert!(dir.join("in/shopping.txt").is_file() && dir.join("in/notes.txt").is_file(), "the wrong thing was renamed");
    let chosen = at(&c, "notes.txt");
    c.update(Msg::Choose(chosen));
    c.update(Msg::Rename);
    assert_eq!(c.renaming.as_ref().map(|r| r.path.clone()), Some(dir.join("in/notes.txt")));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn rename_in_the_menu_is_of_the_same_thing_after_the_list_is_read_again() {
    let (dir, mut c) = menu_chooser("rename-resorted", Mode::Open { many: false, folder: false });
    let row = at(&c, "notes.txt");
    c.update(Msg::Choose(row));
    c.update(Msg::Menu(row, 300.0, 200.0));
    std::fs::write(dir.join("in/a first.txt"), b"new").unwrap();
    std::fs::write(dir.join("in/mid.txt"), b"new").unwrap();
    c.update(Msg::Changed);
    assert_ne!(at(&c, "notes.txt"), row, "the test is of a row that moved");
    pick(&mut c, "Rename");
    assert_eq!(c.renaming.as_ref().map(|r| r.path.clone()), Some(dir.join("in/notes.txt")), "the name box opened on whatever sorted into the row");
    c.update(Msg::RenameText("minutes.txt".into()));
    c.update(Msg::RenameDone);
    assert!(dir.join("in/minutes.txt").is_file() && dir.join("in/list.txt").is_file() && dir.join("in/mid.txt").is_file(), "the wrong thing was renamed");
    let row = at(&c, "list.txt");
    c.update(Msg::Choose(row));
    c.update(Msg::Menu(row, 300.0, 200.0));
    std::fs::remove_file(dir.join("in/list.txt")).unwrap();
    c.update(Msg::Changed);
    pick(&mut c, "Rename");
    assert!(c.renaming.is_none(), "the name box opened on {:?}", c.renaming.as_ref().map(|r| r.path.clone()));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn what_is_pasted_goes_into_the_folder_paste_was_pressed_in() {
    let (dir, mut c) = menu_chooser("paste-where", Mode::Open { many: false, folder: false });
    let notes = dir.join("in/notes.txt");
    c.update(Msg::Open(at(&c, "Albums")));
    let pressed_in = c.folder().map(Path::to_path_buf).unwrap();
    c.go(Where::Folder(dir.join("in/Letters")));
    c.update(Msg::Pasted(Some((vec![notes.clone()], false)), pressed_in));
    assert!(dir.join("in/Albums/notes.txt").is_file(), "it did not go where Paste was pressed");
    assert!(!dir.join("in/Letters/notes.txt").exists(), "it went into the folder opened since");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_cut_by_another_program_is_pasted_as_a_copy() {
    let (dir, mut c) = menu_chooser("paste-foreign", Mode::Open { many: false, folder: false });
    let (notes, list) = (dir.join("in/notes.txt"), dir.join("in/list.txt"));
    let albums = dir.join("in/Albums");
    c.update(Msg::Pasted(Some((vec![notes.clone()], true)), albums.clone()));
    assert!(dir.join("in/Albums/notes.txt").is_file(), "it is pasted");
    assert!(notes.is_file(), "a file this chooser never cut was moved away");
    let i = at(&c, "list.txt");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Cut");
    c.update(Msg::Pasted(Some((vec![notes.clone()], true)), dir.join("in/Letters")));
    assert!(notes.is_file() && dir.join("in/Letters/notes.txt").is_file());
    c.update(Msg::Pasted(Some((vec![list.clone()], true)), albums));
    assert!(dir.join("in/Albums/list.txt").is_file() && !list.exists(), "what was cut here is moved");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_paste_onto_a_taken_name_asks_and_does_what_was_answered() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let (dir, mut c) = menu_chooser("paste-asks", Mode::Open { many: false, folder: false });
    let (notes, list) = (dir.join("in/notes.txt"), dir.join("in/list.txt"));
    std::fs::write(dir.join("in/Albums/notes.txt"), b"old notes").unwrap();
    std::fs::write(dir.join("in/Albums/list.txt"), b"old list").unwrap();
    let albums = dir.join("in/Albums");
    let events = crate::toolkit::event::Loop::new();
    let heard: Rc<RefCell<Vec<Msg>>> = Rc::default();
    let hears = heard.clone();
    c.set_post(events.post(move |m: Msg| hears.borrow_mut().push(m)));
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
    assert!(turn(&mut c, &|c| c.listed));
    c.update(Msg::Pasted(Some((vec![notes.clone(), list.clone()], false)), albums.clone()));
    assert!(turn(&mut c, &|c| c.asking.is_some()), "nothing was asked");
    assert_eq!(c.asking.as_ref().map(|a| a.name.as_str()), Some("notes.txt"));
    assert_eq!(std::fs::read(albums.join("notes.txt")).unwrap(), b"old notes", "nothing is replaced before the answer");
    let view = c.view();
    let said = view.worded();
    assert!(said.iter().any(|(tip, words)| tip == "Replace" && words.as_deref() == Some("Replace")), "{said:?}");
    assert!(view.iconed().iter().all(|(tip, _)| tip != "Replace"));
    c.update(Msg::Answer(Answer::Replace));
    assert!(turn(&mut c, &|c| c.asking.as_ref().is_some_and(|a| a.name == "list.txt")), "the second name was not asked");
    c.update(Msg::Answer(Answer::Skip));
    assert!(turn(&mut c, &|c| c.reply.is_none()), "the paste did not end");
    assert_eq!(std::fs::read(albums.join("notes.txt")).unwrap(), b"the notes");
    assert_eq!(std::fs::read(albums.join("list.txt")).unwrap(), b"old list");
    assert_eq!(std::fs::read_dir(&albums).unwrap().count(), 2, "no copy under another name");
    c.update(Msg::Pasted(Some((vec![notes.clone(), list.clone()], false)), albums.clone()));
    assert!(turn(&mut c, &|c| c.asking.is_some()));
    c.update(Msg::SameForRest(true));
    c.update(Msg::Answer(Answer::Both));
    assert!(turn(&mut c, &|c| c.reply.is_none()));
    assert!(c.asking.is_none());
    assert_eq!(std::fs::read_dir(&albums).unwrap().count(), 4, "both kept, each under a new name");
    std::fs::write(dir.join("in/Letters/notes.txt"), b"letters notes").unwrap();
    c.update(Msg::Pasted(Some((vec![notes.clone()], false)), dir.join("in/Letters")));
    assert!(turn(&mut c, &|c| c.asking.is_some()));
    c.update(Msg::AskStop);
    assert!(turn(&mut c, &|c| c.reply.is_none()));
    assert_eq!(std::fs::read(dir.join("in/Letters/notes.txt")).unwrap(), b"letters notes");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn ctrl_c_x_v_and_delete_do_the_menus_acts() {
    let (dir, mut c) = menu_chooser("keys", Mode::Open { many: false, folder: false });
    let nowhere = Keyboard { keys: true, text: false, arrows: true };
    let typing = Keyboard { keys: true, text: true, arrows: true };
    let ctrl = crate::toolkit::input::Mods { ctrl: true, ..Default::default() };
    let key = |sym, mods| Input::Key { sym, text: None, mods, repeat: false };
    let none = crate::toolkit::input::Mods::default();
    for sym in [Keysym::c, Keysym::x, Keysym::v] {
        assert!(c.intercept_with(&key(sym, ctrl), typing).is_none());
    }
    assert!(c.intercept_with(&key(Keysym::Delete, none), typing).is_none());
    let shift = crate::toolkit::input::Mods { shift: true, ..Default::default() };
    assert!(c.intercept_with(&key(Keysym::Delete, shift), typing).is_none());
    assert!(matches!(c.intercept_with(&key(Keysym::Delete, shift), nowhere), Some(Msg::Key(Act::Delete))));
    let press = |c: &mut Chooser, sym, mods| {
        let m = c.intercept_with(&key(sym, mods), nowhere).unwrap_or_else(|| panic!("{sym:?} is not taken"));
        c.update(m);
    };
    let i = at(&c, "list.txt");
    c.update(Msg::Choose(i));
    press(&mut c, Keysym::c, ctrl);
    assert!(c.said.starts_with("Copied list.txt"), "{}", c.said);
    c.update(Msg::Open(at(&c, "Albums")));
    press(&mut c, Keysym::v, ctrl);
    assert!(dir.join("in/Albums/list.txt").is_file() && dir.join("in/list.txt").is_file());
    c.go(Where::Folder(dir.join("in")));
    let i = at(&c, "notes.txt");
    c.update(Msg::Choose(i));
    press(&mut c, Keysym::x, ctrl);
    c.go(Where::Folder(dir.join("in/Letters")));
    press(&mut c, Keysym::v, ctrl);
    assert!(dir.join("in/Letters/notes.txt").is_file() && !dir.join("in/notes.txt").exists());
    let i = at(&c, "notes.txt");
    c.update(Msg::Choose(i));
    press(&mut c, Keysym::Delete, none);
    assert!(!dir.join("in/Letters/notes.txt").exists());
    assert!(std::fs::read_dir(dir.join("Trash")).unwrap().count() > 0, "it went to the Trash");
    c.chosen = None;
    press(&mut c, Keysym::Delete, none);
    assert_eq!(c.said, "Nothing is picked.");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn delete_permanently_asks_first_and_then_deletes_for_good() {
    let (dir, mut c) = menu_chooser("delete", Mode::Open { many: false, folder: false });
    let i = at(&c, "list.txt");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Delete permanently");
    assert!(c.deleting.is_some() && dir.join("in/list.txt").exists(), "it asks and does nothing yet");
    c.update(Msg::DeleteNo);
    assert!(c.deleting.is_none() && dir.join("in/list.txt").exists());
    let shift = crate::toolkit::input::Mods { shift: true, ..Default::default() };
    let nowhere = Keyboard { keys: true, text: false, arrows: true };
    let m = c.intercept_with(&Input::Key { sym: Keysym::Delete, text: None, mods: shift, repeat: false }, nowhere).unwrap();
    c.update(m);
    assert!(c.deleting.is_some());
    c.update(Msg::DeleteSure);
    assert!(!dir.join("in/list.txt").exists() && c.failed.is_empty(), "{}", c.failed);
    assert_eq!(std::fs::read_dir(dir.join("Trash")).map(|d| d.flatten().filter(|e| e.file_name() == "files").flat_map(|e| std::fs::read_dir(e.path()).unwrap().flatten()).count()).unwrap_or(0), 0, "nothing in the Trash");
    assert!(!c.entries.iter().any(|e| e.name == "list.txt") && c.deleting.is_none());
    let i = at(&c, "Letters");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Delete permanently");
    c.update(Msg::DeleteSure);
    assert!(!dir.join("in/Letters").exists() && dir.join("in/notes.txt").exists());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn pin_and_unpin_in_the_menu_keep_folders_pins() {
    let (dir, mut c) = menu_chooser("pin", Mode::Open { many: false, folder: false });
    let i = at(&c, "Albums");
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Pin to the sidebar");
    assert!(lent::is_pinned(&c.pins, &dir.join("in/Albums")));
    c.update(Msg::Menu(i, 300.0, 200.0));
    assert!(lines(&c).contains(&"Unpin".to_string()));
    pick(&mut c, "Unpin");
    assert!(!lent::is_pinned(&c.pins, &dir.join("in/Albums")));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn copy_cut_paste_and_the_trash_are_folders_own() {
    let (dir, mut c) = menu_chooser("acts", Mode::Open { many: false, folder: false });
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "Paste");
    assert!(c.view().said().contains(&"Nothing has been cut or copied.".to_string()));
    let i = at(&c, "notes.txt");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Copy");
    assert!(c.view().said().contains(&"Copied notes.txt. Paste it into a folder.".to_string()), "{:?}", c.view().said());
    c.update(Msg::Open(at(&c, "Albums")));
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "Paste");
    assert!(dir.join("in/Albums/notes.txt").is_file() && dir.join("in/notes.txt").is_file());
    assert_eq!(c.entries.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["notes.txt"], "the list shows it");
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "Paste");
    assert_eq!(std::fs::read_dir(dir.join("in/Albums")).unwrap().count(), 2);
    assert_eq!(std::fs::read(dir.join("in/Albums/notes.txt")).unwrap(), b"the notes");
    c.go(Where::Folder(dir.join("in")));
    let i = at(&c, "Letters");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Cut");
    assert!(c.view().said().contains(&"Cut Letters. Paste it into a folder.".to_string()));
    c.update(Msg::Open(at(&c, "Albums")));
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "Paste");
    assert!(dir.join("in/Albums/Letters/one.txt").is_file() && !dir.join("in/Letters").exists());
    c.go(Where::Folder(dir.join("in")));
    let i = at(&c, "Albums");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Copy");
    c.update(Msg::Open(at(&c, "Albums")));
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "Paste");
    assert!(c.failed.starts_with("One item could not be copied: ") && c.failed.contains("Albums"), "{}", c.failed);
    assert!(!dir.join("in/Albums/Albums").exists());
    c.go(Where::Folder(dir.join("in")));
    let i = at(&c, "list.txt");
    c.update(Msg::Choose(i));
    c.update(Msg::Menu(i, 300.0, 200.0));
    pick(&mut c, "Move to Trash");
    assert!(!dir.join("in/list.txt").exists() && c.failed.is_empty(), "{}", c.failed);
    assert_eq!(std::fs::read(dir.join("Trash/files/list.txt")).unwrap(), b"the list");
    assert!(!c.entries.iter().any(|e| e.name == "list.txt") && c.chosen.is_none());
    c.update(Msg::RoomMenu(false));
    pick(&mut c, "New folder");
    assert!(dir.join("in/New folder").is_dir());
    assert!(c.renaming.as_ref().is_some_and(|r| r.fresh && r.path == dir.join("in/New folder")));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_the_chooser_menu() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let Some(out) = std::env::var_os("TRINDLE_SHOTS").map(PathBuf::from) else { return };
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let home = std::env::temp_dir().join(format!("trindle-chooser-shoot-menu-{}", std::process::id()));
    std::env::set_var("HOME", &home);
    std::fs::create_dir_all(&out).unwrap();
    let shoot = |c: &Chooser, what: &str, light: bool| {
        let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
        for _ in 0..2 {
            let mut ui = Ui::new(c.view(), Theme::new(light, &crate::common::style::accent()));
            ui.paint(&mut pix.as_mut(), 1.0);
        }
        pix.save_png(out.join(format!("chooser-menu-{name}-{what}-{}.png", if light { "light" } else { "dark" }))).unwrap();
    };
    for light in [false, true] {
        let (dir, mut c) = menu_chooser("shoot", Mode::Open { many: false, folder: true });
        let i = at(&c, "Letters");
        c.update(Msg::Choose(i));
        c.update(Msg::Menu(i, 420.0, 130.0));
        shoot(&c, "item", light);
        c.update(Msg::Pop(PopupMsg::Close));
        c.pointer.set((520.0, 300.0));
        c.update(Msg::RoomMenu(false));
        shoot(&c, "room", light);
        c.update(Msg::Pop(PopupMsg::Close));
        c.update(Msg::Choose(i));
        c.update(Msg::Rename);
        shoot(&c, "rename", light);
        c.update(Msg::RenameText("Albums".into()));
        c.update(Msg::RenameDone);
        shoot(&c, "rename-taken", light);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_the_chooser() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let Some(out) = std::env::var_os("TRINDLE_SHOTS").map(PathBuf::from) else { return };
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let dir = std::env::temp_dir().join(format!("trindle-chooser-shoot-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Receipts 2026", "Recipes", "Work"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for n in 1..=30 {
        let kind = ["txt", "pdf", "ods", "png"][n % 4];
        std::fs::write(papers.join(format!("made up {n:02}.{kind}")), vec![b'x'; 900 * n]).unwrap();
    }
    let ask = Ask { title: "Save the picture".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                    folder: Some(papers), name: "holiday.png".into(), app_id: "org.example.Paint".into() };
    let chooser = Chooser::looked(ask, |_| {});
    std::fs::create_dir_all(&out).unwrap();
    let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
    for _ in 0..2 {
        let mut ui = Ui::new(chooser.view(), Theme::new(false, &crate::common::style::accent()));
        ui.paint(&mut pix.as_mut(), 1.0);
    }
    pix.save_png(out.join(format!("chooser-{name}.png"))).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_the_chooser_saving_over_a_name() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let Some(out) = std::env::var_os("TRINDLE_SHOTS").map(PathBuf::from) else { return };
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let dir = std::env::temp_dir().join(format!("trindle-chooser-shoot-taken-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for (file, bytes) in [("holiday.png", 48_000), ("notes.md", 640), ("letter to John Smith.txt", 1_800), ("budget.ods", 12_000), ("report.pdf", 220_000)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for e in std::fs::read_dir(&papers).unwrap().flatten() {
        std::fs::File::open(e.path()).unwrap().set_modified(then).unwrap();
    }
    std::fs::create_dir_all(&out).unwrap();
    for (light, look) in [(false, "dark"), (true, "light")] {
        let ask = Ask { title: "Save the picture".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                        folder: Some(papers.clone()), name: "holiday.png".into(), app_id: "org.example.Paint".into() };
        let chooser = Chooser::looked(ask, |_| {});
        let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
        for _ in 0..2 {
            let mut ui = Ui::new(chooser.view(), Theme::new(light, &crate::common::style::accent()));
            ui.paint(&mut pix.as_mut(), 1.0);
        }
        pix.save_png(out.join(format!("chooser-taken-{name}-{look}.png"))).unwrap();
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_file_chooser_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    let shoot = |c: &Chooser, state: &str| {
        for _ in 0..2 {
            drawn::window(state, c.view(), Theme::for_page("filechooser"), SIZE, false);
        }
    };
    drawn::none(drawn::in_every_look("the file chooser", 20, || {
        let (dir, mut c) = menu_chooser("pairs", Mode::Open { many: true, folder: false });
        shoot(&c, "opening");
        let i = at(&c, "Letters");
        c.update(Msg::Choose(i));
        shoot(&c, "one chosen");
        c.update(Msg::Menu(i, 420.0, 130.0));
        shoot(&c, "the menu on a thing");
        c.update(Msg::Pop(PopupMsg::Close));
        c.pointer.set((520.0, 300.0));
        c.update(Msg::RoomMenu(false));
        shoot(&c, "the menu on the room");
        c.update(Msg::Pop(PopupMsg::Close));
        c.update(Msg::Choose(i));
        c.update(Msg::Rename);
        shoot(&c, "a name being changed");
        c.update(Msg::RenameText("Albums".into()));
        c.update(Msg::RenameDone);
        shoot(&c, "a name that is taken");
        c.update(Msg::RenameStop);
        c.said = "Copied notes.txt. Paste it into a folder.".into();
        shoot(&c, "after a copy");
        c.said.clear();
        c.failed = "One item could not be copied: Albums cannot be put inside itself.".into();
        shoot(&c, "a failure");
        c.failed.clear();
        c.update(Msg::Hidden);
        c.update(Msg::NewFolder);
        shoot(&c, "a new folder named in the list");
        c.update(Msg::RenameText("a/b".into()));
        c.update(Msg::RenameDone);
        shoot(&c, "a new folder's name that will not do");
        let ask = Ask { title: "Save the picture".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                        folder: Some(dir.join("in")), name: "notes.txt".into(), app_id: "org.example.Paint".into() };
        let mut saving = Chooser::looked(ask, |_| {});
        saving.kept = dir.join("folders");
        shoot(&saving, "saving");
        saving.update(Msg::Accept);
        shoot(&saving, "saving over a name that is there");
        std::fs::remove_dir_all(&dir).unwrap();
    }));
}
