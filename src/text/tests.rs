use super::*;
use crate::toolkit::input::{Keysym, Mods};
use crate::toolkit::theme;
use crate::toolkit::ui::App;

#[test]
fn a_file_is_opened_and_text_put_in_and_a_file_is_never_taken_as_a_move() {
    use crate::toolkit::dnd::{Offered, Ways};
    let held = |files, text, ways| Held { offered: Offered { files, text, ways }, part: Part::Point(3.0, 4.0), mods: Mods::default() };
    assert_eq!(takes(&held(true, true, Ways::ALL), true), Some(Way::Copy));
    assert_eq!(takes(&held(true, false, Ways::MOVE), true), None);
    assert_eq!(takes(&held(false, true, Ways::COPY), true), Some(Way::Copy));
    assert_eq!(takes(&held(false, true, Ways::MOVE), true), Some(Way::Move));
    assert_eq!(takes(&held(false, true, Ways::COPY), false), None);
    assert_eq!(takes(&held(true, true, Ways::COPY), false), Some(Way::Copy));
    assert_eq!(takes(&held(false, false, Ways::ALL), true), None);
    assert!(matches!(dropped(&held(false, true, Ways::COPY), Given::Text("hi".into()), Way::Copy), Msg::Dropped(x, y, Given::Text(t)) if (x, y) == (3.0, 4.0) && t == "hi"));
}

fn text_in(name: &str) -> (Loop, Text, PathBuf) {
    use std::os::unix::fs::DirBuilderExt;
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
    let dir = std::env::temp_dir().join(format!("trindle-text-{name}-{}-{nanos}", std::process::id()));
    std::fs::DirBuilder::new().mode(0o700).create(&dir).unwrap();
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let settings = store::Settings { spell: false, ..store::Settings::default() };
    let text = Text::in_folder(&events, asks, None, settings, dir.clone());
    (events, text, dir)
}

#[test]
fn the_file_chooser_for_a_close_keeps_the_question_and_its_tab_until_it_answers() {
    let (_events, mut text, dir) = text_in("choosing");
    text.pane().borrow_mut().doc.put("Dear John Smith", doc::Kind::Type);
    text.update(Msg::Quit);
    assert!(text.view == View::Closing);
    let id = text.tabs[0].id;
    text.choosing = true;
    text.update(Msg::Answer(Answer::Discard));
    assert!(text.view == View::Closing && text.closing == vec![id] && text.tabs.len() == 1, "an answer while the chooser is open acted on a tab");
    text.chosen(Purpose::SaveAs { id, closing: true }, Vec::new());
    assert!(!text.choosing && text.closing.is_empty() && text.view == View::Edit, "no file chosen should put the question away");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_save_as_that_fails_leaves_the_tab_with_the_name_it_had() {
    let (_events, mut text, dir) = text_in("saveas-fails");
    text.pane().borrow_mut().doc.put("Dear John Smith", doc::Kind::Type);
    let id = text.tabs[0].id;
    let nowhere = dir.clone();
    text.chosen(Purpose::SaveAs { id, closing: false }, vec![nowhere]);
    assert!(text.tabs[0].path.is_none() && !text.tabs[0].markdown, "the tab kept a name that was never written");
    assert!(text.said.is_some());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_same_file_by_a_link_is_one_tab_and_a_huge_one_is_not_read() {
    let (_events, mut text, dir) = text_in("same-file");
    let real = dir.join("real.txt");
    std::fs::write(&real, "words\n").unwrap();
    let link = dir.join("link.txt");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    text.open_path(&real);
    text.open_path(&link);
    text.open_path(&dir.join(".").join("real.txt"));
    assert_eq!(text.tabs.iter().filter(|t| t.path.is_some()).count(), 1, "one file opened twice");
    let big = std::fs::File::create(dir.join("big.bin")).unwrap();
    big.set_len(store::LARGEST + 1).unwrap();
    assert!(store::read_document(&dir.join("big.bin")).is_err(), "a file past the limit was read");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_done_word_and_not_a_tick_follows_a_save() {
    let (_events, mut text, dir) = text_in("done-word");
    text.noted_to("status");
    assert!(text.noted_at::<Msg>("status").is_some() && text.noted_at::<Msg>("bar").is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_pipe_named_as_a_document_is_refused_and_text_does_not_wait() {
    use std::os::unix::fs::OpenOptionsExt;
    let (_events, mut text, dir) = text_in("pipe");
    let pipe = dir.join("notes.txt");
    let c = std::ffi::CString::new(pipe.to_str().unwrap()).unwrap();
    // SAFETY: a path that ends in a nought, for a pipe made in this test's own folder.
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
    let late = pipe.clone();
    let helper = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(2));
        let _ = std::fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(&late);
    });
    let began = std::time::Instant::now();
    let tabs = text.tabs.len();
    text.open_path(&pipe);
    let took = began.elapsed();
    helper.join().unwrap();
    assert!(took < Duration::from_secs(1), "the pipe was waited on for {took:?}");
    assert!(text.said.as_deref().is_some_and(|s| s.contains("could not be opened")), "{:?}", text.said);
    assert!(text.tabs.len() == tabs && !text.tabs.iter().any(|t| t.path.as_deref() == Some(pipe.as_path())), "a pipe was opened as a document");
    assert!(store::read_document(&pipe).is_err() && store::read_document(&dir).is_err());
    std::fs::write(dir.join("real.txt"), "words\n").unwrap();
    assert_eq!(store::read_document(&dir.join("real.txt")).unwrap(), b"words\n");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn text_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    use crate::toolkit::ui::Ui;
    let (_events, mut text, dir) = text_in("pairs");
    std::fs::write(dir.join("shopping.txt"), "milk\neggs\nmilk again, and bread\n").unwrap();
    let markdown = "# A heading\n\nSome *words*, **more** and `code`, and a [link](https://example.com).\n\n## Smaller\n\n- one\n- two\n\n\
                    1. first\n2. second\n\n> a quote from John Smith, with `code` in it\n\n```\na block of code\n```\n\n| a [link](https://example.com) | b |\n|---|---|\n| 1 | 2 |\n\n---\n\n- [ ] to do\n- [x] done\n";
    std::fs::write(dir.join("notes.md"), markdown).unwrap();
    text.open_path(&dir.join("shopping.txt"));
    text.open_path(&dir.join("notes.md"));
    assert_eq!(text.tabs.len(), 2);
    let (text, pix) = (RefCell::new(text), RefCell::new(tiny_skia::Pixmap::new(1100, 800).unwrap()));
    drawn::none(drawn::in_every_look("Text", 60, || {
        let mut t = text.borrow_mut();
        let shoot = |t: &Text, state: &str| {
            drawn::page(state);
            let mut ui = Ui::new(t.view(), theme::Theme::for_page("text"));
            ui.paint(&mut pix.borrow_mut().as_mut(), 1.0);
        };
        t.update(Msg::Tab(0));
        shoot(&t, "editing");
        t.update(Msg::Bar(Bar::Find));
        t.update(Msg::FindText("milk".into()));
        shoot(&t, "finding");
        t.update(Msg::Bar(Bar::Replace));
        t.update(Msg::WithText("oat milk".into()));
        shoot(&t, "replacing");
        t.update(Msg::Bar(Bar::Goto));
        t.update(Msg::GotoText("2".into()));
        shoot(&t, "going to a line");
        t.update(Msg::Bar(Bar::Goto));
        t.fail("shopping.txt could not be written: no room is left on the drive".into());
        shoot(&t, "a failure");
        t.said = None;
        t.update(Msg::Tab(1));
        shoot(&t, "Markdown, as written");
        t.update(Msg::Finished);
        shoot(&t, "Markdown, the finished look");
        t.update(Msg::Finished);
        for (view, name) in [(View::Recent, "recent files"), (View::Settings, "settings"), (View::Encodings, "encodings")] {
            t.update(Msg::Show(view));
            shoot(&t, name);
            t.update(Msg::Show(view));
        }
        t.pane().borrow_mut().doc.put("Dear John Smith, ", doc::Kind::Type);
        shoot(&t, "not yet saved");
        t.update(Msg::Quit);
        assert!(t.view == View::Closing);
        shoot(&t, "asking before it closes");
        t.update(Msg::Back);
        t.pane().borrow_mut().doc.undo();
    }));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_a_found_word_that_is_selected() {
    use crate::common::style::{ASKED_FOR_TEST, DEFAULT_ACCENT};
    use crate::toolkit::ui::Ui;
    let Some(out) = std::env::var_os("TRINDLE_SHOTS").map(PathBuf::from) else { return };
    std::fs::create_dir_all(&out).unwrap();
    let (_events, mut text, dir) = text_in("shoot-found");
    std::fs::write(dir.join("shopping.txt"), "milk\neggs\nmilk again, and bread\n").unwrap();
    text.open_path(&dir.join("shopping.txt"));
    text.update(Msg::Bar(Bar::Find));
    text.update(Msg::FindText("milk".into()));
    ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((false, DEFAULT_ACCENT.to_string())));
    for (when, as_was) in [("before", true), ("after", false)] {
        theme::WASH_AS_WAS.set(as_was);
        let mut ui = Ui::new(text.view(), theme::Theme::for_page("text"));
        let mut pix = tiny_skia::Pixmap::new(900, 360).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("text-found-and-selected-dark-{when}.png"))).unwrap();
    }
    theme::WASH_AS_WAS.set(false);
    ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn text_started_again_after_a_save_cut_short_has_lost_nothing() {
    let (_events, mut text, dir) = text_in("cut-short");
    std::fs::create_dir_all(dir.join("docs")).unwrap();
    let (file, other) = (dir.join("docs/report.txt"), dir.join("docs/same-report.txt"));
    let old = "draft one, which goes on for a good deal longer than this\n";
    std::fs::write(&file, old).unwrap();
    std::fs::hard_link(&file, &other).unwrap();
    text.open_path(&file);
    text.pane().borrow_mut().doc.put("An opening line. ", doc::Kind::Type);
    let words = text.pane().borrow().doc.text().to_string();
    store::CUT.set(1);
    text.update(Msg::Save);
    store::CUT.set(0);
    let torn = std::fs::read_to_string(&file).unwrap();
    assert!(torn != old && torn != words, "the save was not cut short part way: {torn:?}");
    drop(text);
    let kept: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("before-save-")).collect();
    assert_eq!(kept.len(), 3, "{kept:?}");
    for name in &kept {
        let rest = name.strip_prefix(&format!("before-save-{}-", std::process::id())).unwrap();
        std::fs::rename(dir.join(name), dir.join(format!("before-save-1-{rest}"))).unwrap();
    }
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let again = Text::in_folder(&events, asks, None, store::Settings { spell: false, ..store::Settings::default() }, dir.clone());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), old, "the document was left half written");
    assert_eq!(std::fs::read_to_string(&other).unwrap(), old);
    let front = &again.tabs[again.active];
    assert_eq!(front.path.as_deref().map(|p| std::fs::canonicalize(p).unwrap()), Some(std::fs::canonicalize(&file).unwrap()));
    assert!(front.unsaved(), "the words that were being saved count as saved");
    assert_eq!(front.pane.borrow().doc.text(), words);
    assert_eq!(again.said, None);
    let left: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("before-save-")).collect();
    assert!(left.is_empty(), "{left:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

fn ctrl(sym: Keysym) -> Input {
    Input::Key { sym, text: None, mods: Mods { ctrl: true, ..Mods::default() }, repeat: false }
}

fn left_whole(events: &Loop, text: &Text) {
    assert!(events.quitting());
    assert!(!text.tabs.is_empty() && text.active < text.tabs.len());
    let _ = text.view();
}

#[test]
fn closing_with_only_an_empty_document_leaves_text_whole() {
    let (events, mut text, dir) = text_in("quit-key");
    let quit = text.intercept(&ctrl(Keysym::q), true);
    assert!(matches!(quit, Some(Msg::Quit)));
    text.update(Msg::Quit);
    left_whole(&events, &text);
    assert!(store::read_list(&dir).0.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
    let (events, mut text, dir) = text_in("quit-bar");
    text.handle(Do::CloseWindow);
    left_whole(&events, &text);
    assert!(store::read_list(&dir).0.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn closing_without_saving_the_only_document_leaves_text_whole() {
    let (events, mut text, dir) = text_in("quit-discard");
    text.pane().borrow_mut().doc.put("Dear John Smith", doc::Kind::Type);
    text.update(Msg::Quit);
    assert!(text.view == View::Closing && !events.quitting());
    let buttons = text.view().worded();
    assert!(buttons.contains(&("Save".to_string(), Some("Save".to_string()))), "{buttons:?}");
    let icons: Vec<_> = text.view().iconed().into_iter().filter(|(tip, _)| tip.starts_with("Save")).collect();
    assert!(icons.is_empty(), "a Save button has an icon: {icons:?}");
    let saves: Vec<_> = buttons.iter().filter(|(tip, _)| tip.starts_with("Save")).collect();
    assert!(saves.iter().all(|(_, words)| words.is_some()), "a Save button is a bare icon: {saves:?}");
    text.update(Msg::Answer(Answer::Discard));
    left_whole(&events, &text);
    assert!(store::read_list(&dir).0.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn closing_the_last_tab_leaves_an_empty_document_and_text_open() {
    let (events, mut text, dir) = text_in("last-tab");
    let close = text.intercept(&ctrl(Keysym::w), true);
    assert!(matches!(close, Some(Msg::CloseTab(0))));
    text.update(Msg::CloseTab(0));
    assert!(!events.quitting());
    assert_eq!(text.tabs.len(), 1);
    let _ = text.view();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn work_is_kept_on_the_way_out_and_empty_documents_are_not() {
    let (events, mut text, dir) = text_in("quit-kept");
    let file = dir.join("shopping.txt");
    std::fs::write(&file, "milk\n").unwrap();
    text.open_path(&file);
    text.update(Msg::New);
    assert!(text.tabs.len() >= 2);
    text.handle(Do::CloseWindow);
    left_whole(&events, &text);
    let (kept, active) = store::read_list(&dir);
    assert_eq!(kept.iter().map(|e| e.path.clone()).collect::<Vec<_>>(), vec![Some(file)]);
    assert_eq!(active, 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_save_that_fails_keeps_the_file_and_the_unsaved_words() {
    let (_events, mut text, dir) = text_in("save-fails");
    let file = dir.join("shopping.txt");
    std::fs::write(&file, "milk\n").unwrap();
    text.open_path(&file);
    let at = text.active;
    text.tabs[at].pane.borrow_mut().doc.put("eggs and a great deal more besides\n", doc::Kind::Other);
    files::FULL.set(true);
    text.update(Msg::Save);
    files::FULL.set(false);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "milk\n", "the old words were lost");
    let said = text.said.clone().unwrap_or_default();
    assert!(said.contains("shopping.txt could not be saved"), "{said:?}");
    assert!(text.tabs[at].pane.borrow().doc.modified(), "the words were counted as saved");
    assert!(text.tabs[at].pane.borrow().doc.text().contains("eggs"));
    text.update(Msg::Save);
    assert!(text.said.is_none() && !text.tabs[at].pane.borrow().doc.modified());
    assert!(std::fs::read_to_string(&file).unwrap().contains("eggs"));
    let _ = std::fs::remove_dir_all(&dir);
}

fn overwrite(tab: &Tab, words: &str) {
    let mut pane = tab.pane.borrow_mut();
    let len = pane.doc.len();
    pane.doc.select(0, len);
    pane.doc.put(words, doc::Kind::Other);
}

fn changed_behind(name: &str) -> (Loop, Text, PathBuf, PathBuf) {
    let (events, mut text, dir) = text_in(name);
    let file = dir.join("shopping.txt");
    std::fs::write(&file, "milk\n").unwrap();
    text.open_path(&file);
    let at = text.active;
    overwrite(&text.tabs[at], "eggs\n");
    std::fs::write(&file, "milk and bread\n").unwrap();
    (events, text, dir, file)
}

#[test]
fn a_file_changed_elsewhere_is_not_overwritten_by_save() {
    let (_events, mut text, dir, file) = changed_behind("changed-blocks");
    text.update(Msg::Save);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "milk and bread\n");
    assert!(text.tabs[text.active].changed && text.tabs[text.active].pane.borrow().doc.modified());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_file_replaced_by_a_rename_of_the_same_size_is_noticed() {
    let (_events, mut text, dir) = text_in("changed-rename");
    let file = dir.join("a.txt");
    std::fs::write(&file, "milk\n").unwrap();
    text.open_path(&file);
    let other = dir.join("b.txt");
    std::fs::write(&other, "eggs\n").unwrap();
    std::fs::rename(&other, &file).unwrap();
    text.update(Msg::Save);
    assert!(text.tabs[text.active].changed);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_anyway_writes_over_the_changed_file() {
    let (_events, mut text, dir, file) = changed_behind("changed-anyway");
    text.update(Msg::Save);
    text.update(Msg::SaveAnyway);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "eggs\n");
    assert!(!text.tabs[text.active].changed && !text.tabs[text.active].pane.borrow().doc.modified());
    text.update(Msg::Save);
    assert!(!text.tabs[text.active].changed);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn reload_loads_the_file_and_drops_unsaved_edits() {
    let (_events, mut text, dir, file) = changed_behind("changed-reload");
    text.update(Msg::Save);
    text.update(Msg::Reload);
    let tab = &text.tabs[text.active];
    assert_eq!(tab.pane.borrow().doc.text(), "milk and bread\n");
    assert!(!tab.changed && !tab.pane.borrow().doc.modified());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "milk and bread\n");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_unchanged_file_saves_normally_and_so_does_a_deleted_one() {
    let (_events, mut text, dir) = text_in("changed-none");
    let file = dir.join("shopping.txt");
    std::fs::write(&file, "milk\n").unwrap();
    text.open_path(&file);
    overwrite(&text.tabs[text.active], "eggs\n");
    text.update(Msg::Save);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "eggs\n");
    assert!(!text.tabs[text.active].changed);
    overwrite(&text.tabs[text.active], "bread\n");
    std::fs::remove_file(&file).unwrap();
    text.update(Msg::Save);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "bread\n");
    assert!(!text.tabs[text.active].changed);
    let _ = std::fs::remove_dir_all(&dir);
}

fn greyed(text: &Text, name: &str) -> Option<bool> {
    let mut ui = crate::toolkit::ui::Ui::new(text.view(), theme::Theme::new(false, ""));
    let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let told = ui.access("Text");
    told.nodes.iter().map(|(_, n)| n).find(|n| n.label() == Some(name)).map(|n| n.is_disabled())
}

const UNDO: &str = "Undo (Ctrl+Z)";
const REDO: &str = "Redo (Ctrl+Shift+Z)";

#[test]
fn undo_and_redo_are_in_the_top_bar_and_greyed_while_there_is_nothing_for_them() {
    let (_events, mut text, dir) = text_in("undo-bar");
    assert_eq!((greyed(&text, UNDO), greyed(&text, REDO)), (Some(true), Some(true)));
    text.pane().borrow_mut().doc.put("Dear John Smith", doc::Kind::Type);
    assert_eq!((greyed(&text, UNDO), greyed(&text, REDO)), (Some(false), Some(true)));
    text.update(Msg::Undo);
    assert_eq!(text.pane().borrow().doc.text(), "");
    assert_eq!((greyed(&text, UNDO), greyed(&text, REDO)), (Some(true), Some(false)));
    text.update(Msg::Redo);
    assert_eq!(text.pane().borrow().doc.text(), "Dear John Smith");
    assert_eq!((greyed(&text, UNDO), greyed(&text, REDO)), (Some(false), Some(true)));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn undo_and_redo_follow_the_document_in_front() {
    let (_events, mut text, dir) = text_in("undo-tabs");
    text.pane().borrow_mut().doc.put("Shopping", doc::Kind::Type);
    assert_eq!(greyed(&text, UNDO), Some(false));
    text.update(Msg::New);
    assert_eq!((greyed(&text, UNDO), greyed(&text, REDO)), (Some(true), Some(true)));
    text.update(Msg::Tab(0));
    assert_eq!(greyed(&text, UNDO), Some(false));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_keys_for_undo_and_redo_are_the_editors_own() {
    let mut pane = Pane::new(Doc::new(String::new()), look(&store::Settings::default()));
    pane.doc.put("milk", doc::Kind::Type);
    let key = |sym, shift| Input::Key { sym, text: None, mods: Mods { ctrl: true, shift, ..Mods::default() }, repeat: false };
    assert!(pane.key(&key(Keysym::z, false)).is_some());
    assert_eq!(pane.doc.text(), "");
    assert!(pane.key(&key(Keysym::Z, true)).is_some());
    assert_eq!(pane.doc.text(), "milk");
    pane.key(&key(Keysym::z, false));
    assert!(pane.key(&key(Keysym::y, false)).is_some());
    assert_eq!(pane.doc.text(), "milk");
}

#[test]
fn names_are_cut_with_a_mark() {
    assert_eq!(cut("short.txt", 24), "short.txt");
    assert_eq!(cut("a-very-long-file-name-indeed.txt", 10).chars().count(), 10);
    assert!(cut("a-very-long-file-name-indeed.txt", 10).ends_with('…'));
}

#[test]
fn markdown_files_are_known_by_their_ending() {
    assert!(is_markdown(Path::new("/tmp/README.md")));
    assert!(is_markdown(Path::new("/tmp/notes.Markdown")));
    assert!(!is_markdown(Path::new("/tmp/notes.txt")));
}

#[test]
fn nothing_in_the_text_window_touches_its_neighbour() {
    use crate::toolkit::{theme::Theme, ui::Ui};
    let (_, text, dir) = text_in("touching");
    let mut wrong = Vec::new();
    for w in [760, 1278, 1920] {
        let mut ui = Ui::new(crate::toolkit::ui::App::view(&text), Theme::new(true, "#fbc711"));
        ui.lay_at(w, 692);
        wrong.extend(ui.squeezed_or_touching().into_iter().map(|found| format!("at {w}: {found}")));
    }
    let _ = std::fs::remove_dir_all(dir);
    assert!(wrong.is_empty(), "{} things touch or are squeezed:\n{}", wrong.len(), wrong.join("\n"));
}
