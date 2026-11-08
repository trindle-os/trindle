//! Tests: listing.

#[cfg(test)]
use super::*;

#[test]
fn a_name_gives_its_media_type() {
    let m = mime();
    assert_eq!(m.of("Photo.PNG").as_deref(), Some("image/png"));
    assert_eq!(m.of("a.tar.gz").as_deref(), Some("application/x-compressed-tar"));
    assert_eq!(m.of("Makefile").as_deref(), Some("text/x-makefile"));
    assert_eq!(m.of("README.md").as_deref(), Some("text/x-readme"));
    assert_eq!(m.of("readme.txt"), None);
    assert_eq!(m.of("nothing"), None);
}

#[test]
fn kinds_are_kinds_of_their_parents() {
    let m = mime();
    assert!(m.is_a("text/x-csrc", "text/plain"));
    assert!(m.is_a("text/x-makefile", "text/plain"));
    assert!(m.is_a("application/x-compressed-tar", "application/gzip"));
    assert!(m.is_a("image/png", "image/*"));
    assert!(m.is_a("image/png", "image/x-png"));
    assert!(!m.is_a("image/png", "text/plain"));
}

#[test]
fn filters_match_by_pattern_or_kind() {
    let m = mime();
    let pics = Filter { name: "Pictures".into(), rules: vec![(1, "image/*".into())] };
    let c = Filter { name: "C".into(), rules: vec![(0, "*.c".into()), (0, "*.H".into())] };
    assert!(passes(&pics, &m, "a.png", m.of("a.png").as_deref()));
    assert!(!passes(&pics, &m, "a.c", m.of("a.c").as_deref()));
    assert!(passes(&c, &m, "MAIN.C", None));
    assert!(passes(&c, &m, "x.h", None));
    assert!(!passes(&c, &m, "x.cc", None));
}

#[test]
fn folders_come_first_then_names_ignoring_case() {
    let e = |name: &str, folder| Entry { name: name.into(), path: PathBuf::from(name), folder, size: 0, when: 0 };
    let mut all = vec![e("b.txt", false), e("Zoo", true), e("A.txt", false), e("apple", true), e("c", false)];
    sort(&mut all);
    assert_eq!(all.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["apple", "Zoo", "A.txt", "b.txt", "c"]);
}

#[test]
fn recent_files_are_read_newest_first() {
    let xbel = r#"<?xml version="1.0"?><xbel>
<bookmark href="file:///home/john/a%20b.txt" added="x" modified="2026-10-01T10:00:00Z" visited="y">
<bookmark href="https://example.com/x" modified="2026-10-03T10:00:00Z">
<bookmark href="file:///home/john/new.png" modified="2026-10-02T10:00:00Z"></xbel>"#;
    assert_eq!(recent(xbel), [PathBuf::from("/home/john/new.png"), PathBuf::from("/home/john/a b.txt")]);
}

#[test]
fn no_folder_is_listed_twice_down_the_left() {
    let home = Path::new("/home/john");
    let place = |label: &str, icon: &'static str, at: &str| Place { label: label.into(), icon, folder: true, at: Where::Folder(PathBuf::from(at)) };
    let mut places = first_places(home);
    let computer = places.pop().unwrap();
    places.extend([
        place("Documents", "folder-documents-symbolic", "/home/john/Documents"),
        place("Downloads", "folder-download-symbolic", "/home/john/Downloads"),
        place("Music", "folder-music-symbolic", "/home/john/Music"),
        place("Videos", "folder-videos-symbolic", "/home/john/Videos"),

        place("Documents", "folder-symbolic", "/home/john/Documents"),
        place("Music", "folder-symbolic", "/home/john/Music/"),
        place("Work", "folder-symbolic", "/home/john/Work"),
        place("Downloads", "folder-symbolic", "/home/john/Downloads"),
        place("Films", "folder-symbolic", "/home/john/Videos"),
        place("Work again", "folder-symbolic", "/home/john/Work"),
        place("Me", "folder-symbolic", "/home/john"),
        place("Everything", "folder-symbolic", "/"),
        place("Stick", "drive-removable-media-symbolic", "/drives/Stick"),
        place("Stick", "folder-symbolic", "/drives/Stick/"),
    ]);
    places.push(computer);
    let listed: Vec<(String, &str)> = once_each(places).into_iter().map(|p| (p.label, p.icon)).collect();
    let want = [("Recent", "document-open-recent-symbolic"), ("Home", "user-home-symbolic"), ("Documents", "folder-documents-symbolic"),
        ("Downloads", "folder-download-symbolic"), ("Music", "folder-music-symbolic"), ("Videos", "folder-videos-symbolic"),
        ("Work", "folder-symbolic"), ("Stick", "drive-removable-media-symbolic"), ("System disk", "drive-harddisk-symbolic")];
    assert_eq!(listed, want.map(|(l, i)| (l.to_string(), i)));

    let code = crate::common::source::code(include_str!("../looks.rs"));
    assert!(code.contains("self.places = once_each(all);"), "the chooser no longer lists each folder once");
}

#[test]
fn a_place_that_does_not_answer_is_said_so_and_never_shown_as_empty() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-late-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Share")).unwrap();
    let ask = Ask { title: "Made up".into(), accept: None, mode: Mode::Save, filters: Vec::new(), filter: None, choices: Vec::new(),
                    folder: Some(dir.clone()), name: "notes.txt".into(), app_id: "org.example.Paint".into() };
    let mut c = Chooser::looked(ask, |_| {});
    let says = |c: &Chooser| c.view().said().iter().filter(|w| w.as_str() == not_answering()).count();
    assert_eq!(says(&c), 0);

    c.update(Msg::Late(Waited::Place(c.at.clone(), c.hidden)));
    assert_eq!(says(&c), 0, "a place that answered was said not to be answering");

    let share = Where::Folder(dir.join("Share"));
    c.at = share.clone();
    c.entries.clear();
    c.listed = false;
    c.looks.push((share.clone(), c.hidden));
    assert_eq!(says(&c), 0, "said at once, before the look had been out any time");
    assert!(!c.view().said().contains(&"Nothing here".to_string()), "a place still out being read was said to be empty");

    c.update(Msg::Late(Waited::Place(Where::Folder(dir.clone()), c.hidden)));
    assert_eq!(says(&c), 0);
    c.update(Msg::Late(Waited::Place(share.clone(), c.hidden)));
    assert_eq!(says(&c), 1, "a place that has not answered shows no words");
    assert!(!c.view().said().contains(&"Nothing here".to_string()));

    c.go(Where::Folder(dir.clone()));
    assert_eq!(says(&c), 0, "the words stayed after the person went elsewhere");

    c.at = share.clone();
    c.listed = false;
    c.update(Msg::Late(Waited::Place(share.clone(), c.hidden)));
    assert_eq!(says(&c), 1);
    c.update(Msg::Listed(Vec::new(), None, share.clone(), c.hidden));
    assert_eq!(says(&c), 0, "the words stayed after the place answered");
    assert!(c.view().said().contains(&"Nothing here".to_string()));

    let target = dir.join("Share/notes.txt");
    c.accepting.push(target.clone());
    c.update(Msg::Late(Waited::Save(dir.join("other.txt"))));
    assert_eq!(says(&c), 0);
    c.update(Msg::Late(Waited::Save(target.clone())));
    assert_eq!(says(&c), 1, "a save waiting on a place that does not answer says nothing");
    c.update(Msg::Target(Found { folder: false, parent: false, there: false }, target, false));
    assert_eq!(says(&c), 0);

    assert!(not_answering().ends_with('.'));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_that_does_not_answer_holds_back_only_itself() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-start-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let ask = Ask { title: "Choose a folder".into(), accept: None, mode: Mode::Open { many: false, folder: true }, filters: Vec::new(), filter: None, choices: Vec::new(),
                    folder: Some(dir.join("Pictures")), name: String::new(), app_id: "org.example.Paint".into() };
    let mut c = Chooser::new(ask, |_| {});
    let says = |c: &Chooser| c.view().said().iter().filter(|w| w.as_str() == not_answering()).count();

    c.began = true;
    c.start_out = true;
    assert_eq!(says(&c), 0, "said before it was late");
    c.update(Msg::Late(Waited::Start));
    assert_eq!(says(&c), 1, "a start folder that never answered left the window silent");

    let place = |label: &str, folder: bool, at: &Path| Place { label: label.into(), icon: "folder-symbolic", folder, at: Where::Folder(at.to_path_buf()) };
    c.update(Msg::Candidates(vec![place("Pictures", true, &dir.join("Pictures")), place("Work", true, &dir), place("Stick", false, Path::new("/media/stick"))]));
    let names = |c: &Chooser| c.places.iter().map(|p| p.label.clone()).collect::<Vec<_>>();
    assert_eq!(names(&c), ["Recent", "Home", "Work", "Stick", "System disk"], "a folder that is not there was shown");

    c.update(Msg::Checked(0, place("Pictures", true, &dir.join("Pictures"))));
    assert_eq!(names(&c), ["Recent", "Home", "Pictures", "Work", "Stick", "System disk"]);
    c.update(Msg::Checked(usize::MAX, place("Gone", true, &dir.join("Gone"))));
    assert_eq!(names(&c), ["Recent", "Home", "Pictures", "Work", "Stick", "System disk"], "a folder that is not there was shown");

    c.update(Msg::Began(dir.clone()));
    assert_eq!(c.at, Where::Folder(dir.clone()));
    assert!(!c.start_out);
    assert_eq!(says(&c), 0, "the words stayed after the start folder was found");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_first_looks_are_sent_apart() {
    let code = crate::common::source::code(include_str!("../looks.rs"));
    assert!(code.contains("self.aside(move || Msg::Began(begun(named, &kept, &app_id, &home)));"));
    assert!(code.contains("self.aside(move || Msg::Candidates(named_places(&home)));"));
    assert!(code.contains("self.aside(move || place_looked_at(rank, place));"), "each place is looked at apart");
}

#[test]
fn bookmarks_and_usual_folders_are_read() {
    let marks = bookmarks("file:///home/john/Work%20Stuff\nfile:///srv/share Shared\nsftp://example.com/x\n");
    assert_eq!(marks, [("Work Stuff".to_string(), PathBuf::from("/home/john/Work Stuff")), ("Shared".to_string(), PathBuf::from("/srv/share"))]);
    let home = Path::new("/home/john");
    let dirs = usual("XDG_DOCUMENTS_DIR=\"$HOME/Docs\"\nXDG_MUSIC_DIR=\"$HOME\"\nXDG_VIDEOS_DIR=\"/data/films\"\n", home);
    assert_eq!(dirs, [("folder-documents-symbolic", PathBuf::from("/home/john/Docs")), ("folder-videos-symbolic", PathBuf::from("/data/films"))]);
    let mounts = "/dev/sda1 / ext4 rw 0 0\n/dev/sdb1 /media/john/USB\\040Stick vfat rw 0 0\n/dev/sdc1 /run/media/john/Cam vfat rw 0 0\n";
    assert_eq!(drives(mounts), [PathBuf::from("/media/john/USB Stick"), PathBuf::from("/run/media/john/Cam")]);
}

#[test]
fn the_path_bar_starts_at_home_or_the_top() {
    let home = Path::new("/home/john");
    let names = |f: &str| parts(Path::new(f), home).into_iter().map(|(n, _)| n).collect::<Vec<_>>();
    assert_eq!(names("/home/john/Documents/Bills"), ["Home", "Documents", "Bills"]);
    assert_eq!(names("/home/john"), ["Home"]);
    assert_eq!(names("/usr/share"), ["System disk", "usr", "share"]);
    assert_eq!(parts(Path::new("/home/john/Documents"), home)[1].1, PathBuf::from("/home/john/Documents"));
}

#[test]
fn hidden_files_and_new_folders() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-new-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".secret")).unwrap();
    std::fs::write(dir.join("a.txt"), "a").unwrap();
    let ask = Ask { title: "Open".into(), accept: None, mode: Mode::Open { many: false, folder: false }, filters: Vec::new(),
        filter: None, choices: Vec::new(), folder: Some(dir.clone()), name: String::new(), app_id: String::new() };
    let mut c = Chooser::looked(ask, |_| {});
    assert_eq!(c.entries.len(), 1);
    c.update(Msg::Hidden);
    assert_eq!(c.entries.len(), 2);
    c.update(Msg::Hidden);
    new_folders_are_named_in_the_list(&mut c, &dir);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_new_folder_is_named_where_it_is_drawn_in_every_mode() {
    let modes = [Mode::Save, Mode::Open { many: false, folder: false }, Mode::Open { many: true, folder: false },
                 Mode::Open { many: false, folder: true }];
    for (i, mode) in modes.into_iter().enumerate() {
        let dir = std::env::temp_dir().join(format!("trindle-chooser-named-{}-{i}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), "a").unwrap();
        let ask = Ask { title: "Made up".into(), accept: None, mode, filters: Vec::new(), filter: None, choices: Vec::new(),
            folder: Some(dir.clone()), name: "b.txt".into(), app_id: String::new() };
        let mut c = Chooser::looked(ask, |_| {});
        new_folders_are_named_in_the_list(&mut c, &dir);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
fn long_folders_scroll_in_the_chooser() {
    let dir = std::env::temp_dir().join(format!("trindle-chooser-long-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for i in 0..60 {
        std::fs::create_dir_all(dir.join(format!("folder {i:02}"))).unwrap();
    }
    let ask = Ask { title: "Choose a folder".into(), accept: None, mode: Mode::Open { many: false, folder: true }, filters: Vec::new(),
        filter: None, choices: Vec::new(), folder: Some(dir.clone()), name: String::new(), app_id: String::new() };
    let c = Chooser::looked(ask, |_| {});
    assert_eq!(c.entries.len(), 60);
    assert!(c.view().lists_scroll());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn addresses_go_both_ways() {
    let p = PathBuf::from("/home/john smith/caf\u{e9}%.txt");
    assert_eq!(uri_path(&file_uri(&p)), Some(p));
    assert_eq!(uri_path("file://localhost/tmp/x"), Some(PathBuf::from("/tmp/x")));
    assert_eq!(uri_path("https://example.com/x"), None);
    assert_eq!(resolve("~/Notes", None, Path::new("/home/john")), Some(PathBuf::from("/home/john/Notes")));
    assert_eq!(resolve("sub/a.txt", Some(Path::new("/tmp")), Path::new("/home/john")), Some(PathBuf::from("/tmp/sub/a.txt")));
    assert_eq!(resolve("a.txt", None, Path::new("/home/john")), None);
}
