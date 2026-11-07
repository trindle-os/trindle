//! Tests of the Apps pages.

use super::github_page::github_page;
use super::install::pressed_install;
use super::installed::{installed_sentence, remove_installed, tabs, under_tab};
use super::backup_page::choose_backup;
use super::grid::{asks_for_chatgpt, grid};
use super::job_rows::pictured_row;
use crate::store::{github, plugin_page, updates, Store, View};
use super::GitHubPage;
use crate::toolkit::t;
use crate::toolkit::widget::Tone;

#[test]
fn install_runs_only_for_the_persons_own_press() {
    use crate::toolkit::window::{with_named, with_press};
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let mut s = Store::new(&events, asks, None);
    let mut started = 0;
    with_named(true, Some(7), || with_press(Some(8), || pressed_install(&mut s, |_| started += 1)));
    with_named(true, None, || with_press(Some(7), || pressed_install(&mut s, |_| started += 1)));
    with_named(true, Some(7), || pressed_install(&mut s, |_| started += 1));
    assert_eq!(started, 0, "Install ran for a press that was not the person's own");
    with_named(true, Some(7), || with_press(Some(7), || pressed_install(&mut s, |_| started += 1)));
    assert_eq!(started, 1, "Install was refused the person's own press");
}

#[test]
fn searching_for_chatgpt_shows_why_it_is_not_there() {
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let mut s = Store::new(&events, asks, None);
    let line = t!("store-chatgpt-returns").to_string();
    assert_eq!(line, "ChatGPT returns once OpenAI signs its download.");
    for text in ["chatgpt", "ChatGPT", "OPENAI", "codex", "open ai codex cli"] {
        assert!(asks_for_chatgpt(text), "{text}");
        s.view = View::Search(text.to_string());
        assert!(grid(&s, "Results").said().contains(&line), "{text}: no line");
    }
    for text in ["firefox", "chat", "gpt"] {
        assert!(!asks_for_chatgpt(text), "{text}");
        s.view = View::Search(text.to_string());
        assert!(!grid(&s, "Results").said().contains(&line), "{text}: a line for another search");
    }
}

fn more() -> String {
    format!("button: {}", t!("common-more"))
}

#[test]
fn red_words_under_a_row_have_a_more_that_opens_their_page() {
    use crate::store::help::{self, opened};
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let mut s = Store::new(&events, asks, None);
    let row = |s: &Store, key: &str| pictured_row(s, None, key, "Name".into(), "Detail".into(), Vec::new()).said();
    for (key, page) in [("org.example.Paint", help::INSTALL), ("up:app:org.example.Paint", help::UPDATE), ("up:system", help::UPDATE), ("fw:abc", help::FIRMWARE),
                        ("gh:owner/repo", help::GITHUB), ("bk:back", help::BACKUP_PUT), ("bk:make", help::BACKUP_MAKE), ("src:custom", help::SOURCES), ("pl:a.b", help::PLUGIN)] {
        assert!(!row(&s, key).contains(&more()), "{key}: a More with nothing to explain");
        s.say(key, "Not done.", Tone::Problem);
        assert!(row(&s, key).contains(&more()), "{key}: red words with no More");
        let before = opened().len();
        (s.more(key).0)(&mut s);
        assert_eq!(opened().get(before), Some(&page), "{key}");
        assert_eq!(crate::store::help::texts().last().map(String::as_str), Some("Not done."), "{key}: Help is given the real words");
        s.say(key, "Done.", Tone::Plain);
        assert!(!row(&s, key).contains(&more()), "{key}: a More beside words that are not red");
    }
    s.say_on("up:app:org.example.Paint", "Not held back.", help::HELD);
    assert_eq!(s.page_of("up:app:org.example.Paint"), help::HELD);
    s.say("up:app:org.example.Paint", "Not updated.", Tone::Problem);
    assert_eq!(s.page_of("up:app:org.example.Paint"), help::UPDATE);
}

#[test]
fn github_words_go_to_the_key_page_only_for_the_key() {
    assert_eq!(github_page(t!("store-gh-key-refused")), crate::store::help::GITHUB_KEY);
    assert_eq!(github_page(t!("store-gh-unreached")), crate::store::help::GITHUB);
}

#[test]
fn backup_put_back_plugin_off_and_plugin_removal_are_the_persons_own_press() {
    use crate::toolkit::window::{refused_act, with_named, with_press};
    use updates::{Installed, Removal, Source};
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let item = Installed { id: "org.example.Plug".into(), name: "Plug".into(), version: String::new(), size: String::new(),
                           source: Source::Plugin { icon: None } };
    let made_up = |act: &mut dyn FnMut()| {
        let _ = refused_act();
        with_named(true, Some(7), || with_press(Some(8), act));
        assert!(refused_act(), "a refused press asked for no words");
    };
    let mut s = Store::new(&events, asks, None);
    made_up(&mut || choose_backup(&mut s));
    assert!(!s.said.contains_key("bk:back"), "a made-up press started putting a backup back");
    with_named(true, Some(7), || with_press(Some(7), || choose_backup(&mut s)));
    assert!(s.said.contains_key("bk:back"), "the person's own press was refused");

    let noted = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    s.owned.removals = Some(noted.clone());
    made_up(&mut || remove_installed(&mut s, &item, false));
    assert!(noted.borrow().is_empty(), "a made-up press removed a plugin");
    with_named(true, Some(7), || with_press(Some(7), || remove_installed(&mut s, &item, false)));
    assert_eq!(*noted.borrow(), [(Removal::Plugin, item.id.clone())]);

    made_up(&mut || assert!(!plugin_page::may_remove()));
    with_named(true, Some(7), || with_press(Some(7), || {
        assert!(plugin_page::may_remove());
    }));
}

#[test]
fn no_project_drops_out_of_a_github_search_without_a_word() {
    let mut g = GitHubPage::default();
    assert!(g.problems().is_empty());
    g.release_failed(github::no_files().into());
    g.release_failed(github::no_releases().into());
    assert!(g.problems().is_empty() && g.unread == 0);
    g.release_failed("GitHub could not be reached.".into());
    assert_eq!(g.problems(), vec!["1 project found could not be looked at, so it is not listed: GitHub could not be reached.".to_string()]);
    g.release_failed("GitHub refused this: Repository access blocked.".into());
    assert_eq!(g.problems(), vec!["2 projects found could not be looked at, so they are not listed: GitHub refused this: Repository access blocked.".to_string()]);
    let limit = "GitHub has stopped answering for now: it answers 60 questions an hour from one address without a key. Try again later.";
    g.release_failed(limit.into());
    assert_eq!(g.problems().len(), 2);
    assert!(g.problems()[0].starts_with("GitHub has stopped answering") && g.unread == 2);
    g.half = Some(limit.into());
    let said = g.problems();
    assert_eq!(said.len(), 3);
    assert!(said[0].starts_with("Only projects with an AppImage were searched for: GitHub did not answer the search for Flatpaks."), "{}", said[0]);
    for words in &said {
        assert!(words.ends_with('.') && !words.contains('\u{2014}'), "{words}");
    }
}

#[test]
fn the_installed_sentence_counts_every_kind() {
    let says = |counts: [usize; updates::KINDS], waiting| installed_sentence(counts.iter().sum(), counts, waiting);
    assert_eq!(says([0, 0, 0, 0, 0], 0), "Nothing is installed yet. Discover is full of things to try.");
    assert_eq!(says([9, 0, 0, 0, 0], 0), "9 programs are installed, all from Flathub. All are up to date.");
    assert_eq!(says([1, 0, 0, 0, 0], 0), "1 program is installed, from Flathub. It is up to date.");
    assert_eq!(says([6, 2, 1, 0, 0], 2), "9 programs are installed, 6 from Flathub, 2 from Flatpak files and 1 AppImage. 2 have updates waiting.");
    assert_eq!(says([0, 0, 2, 0, 0], 0), "2 programs are installed, all AppImages. All are up to date.");
    assert_eq!(says([6, 2, 1, 2, 3], 1), "14 things are installed: 9 apps, 2 from .deb files and 3 plugins. 1 has an update waiting.");
    assert_eq!(says([6, 2, 1, 1, 0], 0), "10 programs are installed: 9 apps and 1 from a .deb file. All are up to date.");
    assert_eq!(says([1, 0, 0, 0, 1], 0), "2 things are installed: 1 app and 1 plugin. All are up to date.");
    assert_eq!(says([0, 0, 0, 2, 0], 0), "2 programs are installed, all from .deb files. All are up to date.");
    assert_eq!(says([0, 0, 0, 1, 0], 0), "1 program is installed, from a .deb file. It is up to date.");
    assert_eq!(says([0, 0, 0, 0, 3], 1), "3 plugins are installed. 1 has an update waiting.");
    assert_eq!(says([0, 0, 0, 0, 1], 0), "1 plugin is installed. It is up to date.");
    assert_eq!(says([0, 0, 0, 1, 2], 0), "3 things are installed: 1 from a .deb file and 2 plugins. All are up to date.");
}

#[test]
fn every_kind_has_its_tab_and_its_own_removal() {
    use updates::{Installed, Removal, Source};
    let thing = |source: Source| Installed { id: "org.example.Thing".into(), name: "Thing".into(), version: String::new(), size: String::new(), source };
    let kinds = [
        (thing(Source::Flatpak { origin: "flathub".into(), installation: "system".into() }), "Flathub", Removal::Flatpak { system: true, delete_data: false }, "org.example.Thing"),
        (thing(Source::Flatpak { origin: "example-origin".into(), installation: "user".into() }), "Flatpak files", Removal::Flatpak { system: false, delete_data: false }, "org.example.Thing"),
        (thing(Source::AppImage("/nonexistent/thing.AppImage".into())), "AppImages", Removal::AppImage("/nonexistent/thing.AppImage".into()), "org.example.Thing"),
        (thing(Source::Deb { launcher: None, icon: None }), "Deb files", Removal::Deb, "deb:org.example.Thing"),
        (thing(Source::Plugin { icon: None }), "Plugins", Removal::Plugin, "plugin:org.example.Thing"),
    ];
    assert_eq!(kinds.len(), updates::KINDS);
    for (i, (item, tab, way, key)) in kinds.iter().enumerate() {
        assert_eq!((item.kind(), tabs()[item.kind() + 1]), (i, *tab));
        assert_eq!((item.removal(), item.key()), (way.clone(), key.to_string()));
        assert!(under_tab(0, item) && under_tab(i + 1, item) && !under_tab((i + 1) % updates::KINDS + 1, item), "{tab}");
    }
}

#[test]
fn no_flatpak_is_uninstalled_except_through_the_one_command() {
    fn walk(dir: &std::path::Path, hits: &mut Vec<String>) {
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, hits);
            } else if p.extension().is_some_and(|x| x == "rs") && !p.ends_with("common/removal.rs") && !p.ends_with("store/pages/tests.rs") {
                let text = std::fs::read_to_string(&p).unwrap();
                if text.contains("\"flatpak\", \"uninstall\"") || text.contains("\"uninstall\", \"-y\"") {
                    hits.push(p.display().to_string());
                }
            }
        }
    }
    let mut hits = Vec::new();
    for (src, _) in crate::common::source::trees() {
        walk(&src, &mut hits);
    }
    assert!(hits.is_empty(), "these build their own flatpak uninstall: {hits:?}");
}

#[test]
fn every_way_to_install_an_appimage_shows_the_warning_first() {
    use super::appimage_warning::{cancel, showing};
    use super::github_search::{install_file, warns};
    use crate::store::came_from::CameFrom;
    use crate::toolkit::window::{with_named, with_press};
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: crate::store::Do| {});
    let mut s = Store::new(&events, asks, None);
    s.conf = std::env::temp_dir().join(format!("trindle-gate-{}", std::process::id())).join("trindle.conf");
    s.warning_off = false;
    let nothing_started = |s: &Store| s.dials.is_empty() && s.working == 0 && s.owned.adding.is_none();
    let found = github::Found { repo: "john-smith/thing".into(), name: "Thing".into(), listed: true, ..Default::default() };
    let file = |name: &str, kind| github::Download { name: name.into(), url: format!("https://example.org/{name}"), size: 1, kind };

    let tile = super::image_tile::image_tile(&s, &found);
    let press = tile.presses().into_iter().find(|(tip, _, _)| tip == "Install Thing").map(|(_, _, m)| m.clone()).expect("the tile has Install");
    with_named(true, Some(7), || with_press(Some(7), || (press.0)(&mut s)));
    assert!(showing(&s) && nothing_started(&s), "tile");
    cancel(&mut s);
    install_file(&mut s, "gh:john-smith/thing/Thing.AppImage", &file("Thing.AppImage", github::Kind::AppImage), CameFrom::GitHub("john-smith/thing".into()));
    assert!(showing(&s) && nothing_started(&s), "file");
    cancel(&mut s);
    super::github_search::install_repo(&mut s, &found);
    assert!(showing(&s) && nothing_started(&s), "project");
    cancel(&mut s);
    let release = |files: Vec<github::Download>| github::Release { tag: "v1".into(), files, published: None };
    let (app, flat) = (file("A.AppImage", github::Kind::AppImage), file("A.flatpak", github::Kind::Flatpak));
    assert!(warns(None, true) && warns(None, false), "not looked at yet");
    assert!(warns(Some(&release(vec![app.clone()])), false) && warns(Some(&release(vec![flat.clone(), app.clone()])), true));
    assert!(!warns(Some(&release(vec![app, flat.clone()])), false), "a Flatpak is installed first, so no warning");
    assert!(!warns(Some(&release(vec![flat])), true));
    let page = super::installed::installed(&s);
    let add = page.presses().into_iter().find(|(tip, _, _)| tip == t!("store-installed-add-appimage")).map(|(_, _, m)| m.clone()).expect("Installed has Add");
    (add.0)(&mut s);
    assert!(showing(&s) && nothing_started(&s), "Add");
    cancel(&mut s);
    let dir = std::env::temp_dir().join(format!("trindle-gate-files-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let elf = dir.join("Thing.AppImage");
    std::fs::write(&elf, b"\x7fELF").unwrap();
    s.dropped(crate::toolkit::dnd::Given::Files(vec![elf.clone()]));
    assert!(showing(&s) && nothing_started(&s), "dropped");
    cancel(&mut s);
    s.apply(crate::store::opts::Opts { appimage: Some(elf), ..Default::default() });
    assert!(showing(&s) && nothing_started(&s), "Folders");
    let _ = std::fs::remove_dir_all(&dir);
}
