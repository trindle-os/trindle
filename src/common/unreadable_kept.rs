//! Every file of the person's own that is changed from what was read of it
//! is kept when it cannot be read. Two faults are tried on each:
//! a file of garbage (not text) and a file this account is not allowed to
//! read. Neither may be taken for an empty file and written over: the
//! change is refused and the file stays exactly as it was. The files that
//! are set aside when damaged (`damaged::read`) may instead be moved to a
//! name beside themselves with the garbage in it, and a fresh one started.
//! Each change works in a folder of its own under the temporary folder,
//! never the person's own.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const NOT_TEXT: &[u8] = b"first\n\xff\xfe second\nthird\n";

const LOCKED: &[u8] = b"a\tb\nc\td\n";

struct Place {
    what: &'static str,
    name: &'static str,
    set_aside: bool,
    change: fn(&Path),
}

fn home_of(f: &Path) -> PathBuf {
    f.parent().unwrap().to_path_buf()
}

const PLACES: &[Place] = &[
    Place { what: "the ring's launch counts", name: "usage", set_aside: true,
            change: |f| crate::ring::store::record_use_in(f, "org.example.One.desktop") },
    Place { what: "the apps used last for each kind of file", name: "app-choices", set_aside: true,
            change: |f| crate::common::app_choice::remember_in(f, "text/plain", "org.example.One.desktop") },
    Place { what: "the clipboard history", name: "index.json", set_aside: true,
            change: |f| {
                crate::common::clipboard::TEST_DIR.with(|d| *d.borrow_mut() = Some(home_of(f)));
                crate::common::clipboard::keep(b"hello".to_vec());
                crate::common::clipboard::TEST_DIR.with(|d| *d.borrow_mut() = None);
            } },
    Place { what: "the folder each program last chose in", name: "file-chooser-folders", set_aside: true,
            change: |f| crate::screenshot::file_chooser::kept::keep_folder(f, "org.example.One", Path::new("/home/john/Music")) },
    Place { what: "the eye dropper's colours", name: "colours", set_aside: true,
            change: |f| crate::screenshot::colours::save_in(f, &["#112233".to_string()]) },
    Place { what: "Text's recent files", name: "recent", set_aside: true,
            change: |f| { let _ = crate::text::store::add_recent(&home_of(f), Path::new("/home/john/a.txt")); } },
    Place { what: "the list of missing icons", name: "missing-icons.txt", set_aside: false,
            change: |f| { let _ = crate::common::missing_icons::add_to(f, &[("org.example.One".to_string(), "One".to_string())]); } },
    Place { what: "the screen arrangement an older version kept", name: "display.sh", set_aside: false,
            change: |f| { let _ = crate::common::outputs::saved_in(&home_of(f).join("displays.json"), f); } },
    Place { what: "what a box is allowed", name: "allowed", set_aside: false,
            change: |f| { let _ = crate::store::boxed::allow_at(&home_of(f), crate::store::boxed::ALLOWED, "network", true); } },
    Place { what: "what has been put to the person about a box", name: "asked", set_aside: false,
            change: |f| { let _ = crate::store::boxed::mark_asked_at(&home_of(f)); } },
    Place { what: "the apps offered at first login", name: "offered", set_aside: false,
            change: |f| { let _ = crate::store::boxed::write_kept_lines(f, "Offered", &["org.example.One".to_string()]); } },
    Place { what: "a fenced program's list of programs", name: "programs", set_aside: false,
            change: |f| { let _ = crate::store::fence::set_programs(&home_of(f), &[("claude".to_string(), true)]); } },
    Place { what: "a fenced program's sites", name: "net-hosts", set_aside: false,
            change: |f| { let _ = crate::store::fence::set_net_hosts(&home_of(f), &["example.com".to_string()]); } },
    Place { what: "a fenced program's computers to reach by SSH", name: "ssh-hosts", set_aside: false,
            change: |f| { let _ = crate::store::fence::set_ssh_hosts(&home_of(f), &["example.com".to_string()]); } },
    Place { what: "a fenced program's shared folders", name: "shared", set_aside: false,
            change: |f| { let _ = crate::store::fence::unshare(&home_of(f), Path::new("/home/john/Music")); } },
];

fn scratch(tag: &str, n: usize) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-unreadable-kept-{tag}-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn all_files(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(dir).unwrap().flatten()
        .filter(|e| e.path().is_file())
        .filter_map(|e| Some((e.file_name().to_string_lossy().into_owned(), std::fs::read(e.path()).ok()?)))
        .collect();
    out.sort();
    out
}

#[test]
fn a_file_of_garbage_is_never_written_over() {
    let mut wrong = Vec::new();
    for (n, p) in PLACES.iter().enumerate() {
        let dir = scratch("garbage", n);
        let file = dir.join(p.name);
        std::fs::write(&file, NOT_TEXT).unwrap();
        (p.change)(&file);
        let kept_here = std::fs::read(&file).ok().as_deref() == Some(NOT_TEXT);
        let kept_aside = p.set_aside && all_files(&dir).iter().any(|(name, bytes)| name.contains(".damaged") && bytes == NOT_TEXT);
        if !(kept_here || kept_aside) {
            wrong.push(format!("{}: the garbage in {} was lost", p.what, p.name));
        }
        if !p.set_aside && !kept_here {
            wrong.push(format!("{}: {} was changed though it could not be read", p.what, p.name));
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn a_file_that_may_not_be_read_is_never_written_over() {
    // SAFETY: geteuid takes nothing and cannot fail.
    assert_ne!(unsafe { libc::geteuid() }, 0, "root reads every file, so this test would prove nothing");
    let mut wrong = Vec::new();
    for (n, p) in PLACES.iter().enumerate() {
        let dir = scratch("locked", n);
        let file = dir.join(p.name);
        std::fs::write(&file, LOCKED).unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o000)).unwrap();
        (p.change)(&file);
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).ok();
        if std::fs::read(&file).ok().as_deref() != Some(LOCKED) {
            wrong.push(format!("{}: {} that this account may not read was written over", p.what, p.name));
        }
        if all_files(&dir).len() != 1 {
            wrong.push(format!("{}: something else was made beside {}", p.what, p.name));
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn with_no_file_yet_the_same_change_goes_through() {
    let mut wrong = Vec::new();
    for (n, p) in PLACES.iter().enumerate() {

        if p.name == "display.sh" || p.name == "shared" {
            continue;
        }
        let dir = scratch("first", n);
        (p.change)(&dir.join(p.name));
        if all_files(&dir).is_empty() {
            wrong.push(format!("{}: a first change was refused", p.what));
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn a_picture_copied_while_the_history_cannot_be_read_takes_no_old_picture_name() {
    // SAFETY: geteuid takes nothing and cannot fail.
    assert_ne!(unsafe { libc::geteuid() }, 0, "root reads every file, so this test would prove nothing");
    let dir = scratch("clipboard", 0);
    let index = dir.join("index.json");
    std::fs::write(&index, LOCKED).unwrap();
    std::fs::set_permissions(&index, std::fs::Permissions::from_mode(0o000)).unwrap();
    std::fs::write(dir.join("1.png"), b"an old picture").unwrap();
    crate::common::clipboard::TEST_DIR.with(|d| *d.borrow_mut() = Some(dir.clone()));
    let mut picture = b"\x89PNG\r\n\x1a\n".to_vec();
    picture.extend_from_slice(b"a new picture");
    crate::common::clipboard::keep(picture);
    crate::common::clipboard::TEST_DIR.with(|d| *d.borrow_mut() = None);
    std::fs::set_permissions(&index, std::fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(std::fs::read(dir.join("1.png")).unwrap(), b"an old picture");
    assert_eq!(std::fs::read(&index).unwrap(), LOCKED);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_words_for_a_file_kept_are_in_the_word_files_with_a_help_address() {
    use crate::common::damaged::{may_write, said, unreadable_words, words, HELP};
    let dir = scratch("words", 0);
    let file = dir.join("usage");
    std::fs::write(&file, NOT_TEXT).unwrap();
    let e = may_write(&file, "Launch counts").unwrap_err();
    assert_eq!(said(&e).as_deref(), Some(unreadable_words("Launch counts").as_str()));
    assert!(unreadable_words("Launch counts").contains("Launch counts") && unreadable_words("x") != words("x"));
    assert!(may_write(&dir.join("none"), "Launch counts").is_ok());
    assert_eq!(HELP, "settings-unreadable");
    let ftl = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/share/lang/en/common.ftl")).unwrap();
    assert!(ftl.contains("settings-unreadable") && ftl.contains("common-unreadable-kept"));
    assert!(!unreadable_words("Launch counts").contains('\u{2014}'));
    std::fs::remove_dir_all(&dir).unwrap();
}
