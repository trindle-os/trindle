use std::fmt::Write as _;
use super::*;
use crate::plugins::fixtures::*;
use crate::plugins::menu::MENU_LABEL_MAX;

#[test]
fn ring_json_gives_a_headline_and_hidden_ids() {
    let dir = home_with("good", Some(br#"{"headline":"Connected to desktop.","hidden":["connect"]}"#));
    assert_eq!(ring_note_at(&dir), RingNote { headline: "Connected to desktop.".into(), hidden: vec!["connect".into()], ..Default::default() });
    let _ = std::fs::remove_dir_all(&dir);
    let odd = parse_ring_note(br#"{"headline":3,"hidden":["a",7,null,"b"]}"#);
    assert_eq!(odd, RingNote { headline: String::new(), hidden: vec!["a".into(), "b".into()], ..Default::default() });
    assert_eq!(parse_ring_note(br#"{"headline":"Hi","hidden":"connect"}"#), RingNote { headline: "Hi".into(), hidden: vec![], ..Default::default() });
}

#[test]
fn a_missing_unreadable_big_or_bad_ring_json_means_everything_and_no_headline() {
    let none = home_with("none", None);
    assert_eq!(ring_note_at(&none), RingNote::default());
    let _ = std::fs::remove_dir_all(&none);
    for (name, bytes) in [("bad", &b"{not json"[..]), ("list", b"[1,2]"), ("text", b"\"headline\""), ("empty", b"")] {
        let dir = home_with(name, Some(bytes));
        assert_eq!(ring_note_at(&dir), RingNote::default(), "{name}");
        let _ = std::fs::remove_dir_all(&dir);
    }
    let big = format!(r#"{{"headline":"Hi","pad":"{}"}}"#, "x".repeat(5000));
    let dir = home_with("big", Some(big.as_bytes()));
    assert_eq!(ring_note_at(&dir), RingNote::default());
    let _ = std::fs::remove_dir_all(&dir);
    let pad = RING_NOTE_MAX as usize - r#"{"headline":"Hi","pad":""}"#.len();
    let edge = format!(r#"{{"headline":"Hi","pad":"{}"}}"#, "x".repeat(pad));
    assert_eq!(edge.len() as u64, RING_NOTE_MAX);
    let dir = home_with("edge", Some(edge.as_bytes()));
    assert_eq!(ring_note_at(&dir).headline, "Hi");
    let linked = home_with("link", None);
    std::os::unix::fs::symlink(dir.join(RING_NOTE), linked.join(RING_NOTE)).unwrap();
    assert_eq!(ring_note_at(&linked), RingNote::default());
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&linked);
    let folder = home_with("folder", None);
    std::fs::create_dir(folder.join(RING_NOTE)).unwrap();
    assert_eq!(ring_note_at(&folder), RingNote::default());
    let _ = std::fs::remove_dir_all(&folder);
}

#[test]
fn ring_json_may_add_items_that_hand_the_program_arguments() {
    let got = items_of(r#"{"items":[{"label":"Go to desktop","args":["goto","abc123"]},{"label":"Back","args":[]}]}"#);
    assert_eq!(got, [
        DynItem { label: "Go to desktop".into(), args: vec!["goto".into(), "abc123".into()], unlock: None },
        DynItem { label: "Back".into(), args: vec![], unlock: None },
    ]);
    for json in [r#"{}"#, r#"{"items":3}"#, r#"{"items":"a"}"#, r#"{"items":{"label":"x","args":[]}}"#, r#"{"items":[]}"#] {
        assert!(items_of(json).is_empty(), "{json}");
    }
}

#[test]
fn hostile_items_are_left_out_whole_or_cut_to_plain_words() {
    let many: Vec<String> = (0..50).map(|n| format!(r#"{{"label":"Item {n}","args":["x"]}}"#)).collect();
    let got = items_of(&format!(r#"{{"items":[{}]}}"#, many.join(",")));
    assert_eq!(got.len(), ITEMS_MAX);
    assert_eq!(got[7].label, "Item 7");
    let long = "y".repeat(ARG_MAX + 1);
    let fits = "y".repeat(ARG_MAX);
    for args in [
        r#"["a","b","c","d","e"]"#.to_string(), format!(r#"["{long}"]"#), r#"["a\u0000b"]"#.into(), r#"["a\nb"]"#.into(),
        r#"["a\u001b[31m"]"#.into(), r#"["a\u007f"]"#.into(), r#"[""]"#.into(), r#"[1]"#.into(), r#"[null]"#.into(),
        r#"[["a"]]"#.into(), r#""a b""#.into(), "null".into(), r#"{"0":"a"}"#.into(),
    ] {
        let got = items_of(&format!(r#"{{"items":[{{"label":"Bad","args":{args}}},{{"label":"Good","args":["{fits}"]}}]}}"#));
        assert_eq!(got.len(), 1, "{args}");
        assert_eq!(got[0].label, "Good");
        assert_eq!(got[0].args[0].chars().count(), ARG_MAX, "exactly as given");
    }
    for item in [r#"{"label":"No args"}"#, r#"{"args":["a"]}"#, r#"{"label":"","args":[]}"#, r#"{"label":" \n\t ","args":[]}"#,
                 r#"{"label":5,"args":[]}"#, r#""text""#, "7", "null", "[]"] {
        assert!(items_of(&format!(r#"{{"items":[{item}]}}"#)).is_empty(), "{item}");
    }
    let got = items_of(r#"{"items":[{"label":"<b>Go</b> &amp; <a href=x>now</a>\u202eevil\u0007\n\u200bdone","args":[]},
                                     {"label":"0123456789012345678901234567890123456789","args":[]}]}"#);
    assert_eq!(got[0].label, "<b>Go</b> &amp; <a href=x>now</a>evil done".chars().take(31).collect::<String>() + "\u{2026}");
    assert_eq!(got[1].label.chars().count(), MENU_LABEL_MAX);
    assert!(got[1].label.ends_with('\u{2026}'));
    let both = parse_ring_note(br#"{"headline":"On.","hidden":["a"],"items":[{"label":"X","args":["y"]}]}"#);
    assert_eq!((both.headline.as_str(), both.hidden.len(), both.items.len()), ("On.", 1, 1));
}

#[test]
fn what_a_menu_item_may_hand_over_is_checked_again_where_it_runs() {
    let ok = |a: &[&str]| good_args(&a.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    assert!(ok(&[]) && ok(&["goto", "abc123"]) && ok(&["a", "b", "c", "d"]));
    assert!(!ok(&["a", "b", "c", "d", "e"]) && !ok(&[""]) && !ok(&["a\nb"]) && !ok(&["\u{0}"]));
    assert!(!good_args(&["z".repeat(65)]) && good_args(&["z".repeat(64)]));
}

#[test]
fn a_plain_file_is_read_and_a_link_a_pipe_or_a_folder_is_not() {
    let dir = home_with("plain", Some(b"hello"));
    assert_eq!(read_plain(&dir.join(RING_NOTE), 10).as_deref(), Some(&b"hello"[..]));
    assert_eq!(read_plain(&dir.join(RING_NOTE), 4), None, "over the limit");
    assert_eq!(read_plain(&dir.join("nothing"), 10), None);
    std::os::unix::fs::symlink("/etc/hostname", dir.join("link")).unwrap();
    assert_eq!(read_plain(&dir.join("link"), 1000), None, "not followed, even to a file anyone may read");
    std::os::unix::fs::symlink(dir.join(RING_NOTE), dir.join("link2")).unwrap();
    assert_eq!(read_plain(&dir.join("link2"), 1000), None, "nor to a good one");
    std::fs::create_dir(dir.join("folder")).unwrap();
    assert_eq!(read_plain(&dir.join("folder"), 1000), None);
    let fifo = std::ffi::CString::new(dir.join("pipe").to_str().unwrap()).unwrap();
    // SAFETY: mkfifo is given a NUL-ended path that lives through the call.
    assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
    assert_eq!(read_plain(&dir.join("pipe"), 1000), None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_link_a_plugin_plants_for_trindles_own_files_is_replaced_never_written_through() {
    let outside = home_with("outside", None);
    let target = precious(&outside, b"line one\nline two\n");
    let home = home_with("settings", None);
    std::os::unix::fs::symlink(&target, home.join(VALUES)).unwrap();
    assert!(keyfile_in_home(&home.join(VALUES)).string("Settings", "x").is_none());
    write_in_home(&home.join(VALUES), "[Settings]\nx=1\n").unwrap();
    assert_eq!(std::fs::read(&target).unwrap(), b"line one\nline two\n");
    assert!(std::fs::symlink_metadata(home.join(VALUES)).unwrap().is_file());
    assert_eq!(keyfile_in_home(&home.join(VALUES)).string("Settings", "x").as_deref(), Some("1"));
    std::os::unix::fs::symlink(&target, home.join("settings.trindle-tmp")).unwrap();
    write_in_home(&home.join(VALUES), "[Settings]\nx=2\n").unwrap();
    assert_eq!(std::fs::read(&target).unwrap(), b"line one\nline two\n");
    assert_eq!(keyfile_in_home(&home.join(VALUES)).string("Settings", "x").as_deref(), Some("2"));
    std::fs::create_dir(home.join("dir.ini")).unwrap();
    assert!(write_in_home(&home.join("dir.ini"), "x").is_err());
    let _ = std::fs::remove_dir_all(&outside);
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn a_notes_file_that_is_a_link_is_not_read_cut_back_or_written_through() {
    let outside = home_with("notes-out", None);
    let big: Vec<u8> = (0..4000).flat_map(|n| format!("{{\"summary\":\"line {n}\"}}\n").into_bytes()).collect();
    let target = precious(&outside, &big);
    let home = home_with("notes", None);
    let (notes, seen) = (home.join(NOTES), home.join("notify.seen"));
    std::os::unix::fs::symlink(&target, &notes).unwrap();
    std::os::unix::fs::symlink(outside.join("seen-target"), &seen).unwrap();
    assert!(unread_notes(&notes, &seen, 64, 1 << 20, 5).is_empty(), "a link is no notes file");
    assert_eq!(std::fs::read(&target).unwrap(), big, "the file it led to is whole");
    assert!(!outside.join("seen-target").exists(), "nothing was written through the other link");
    let _ = std::fs::remove_dir_all(&outside);
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn real_notes_are_new_once_cut_back_when_long_and_taken_away_when_huge() {
    let home = home_with("realnotes", None);
    let (notes, seen) = (home.join(NOTES), home.join("notify.seen"));
    std::fs::write(&notes, "a\nb\n").unwrap();
    assert!(unread_notes(&notes, &seen, 1000, 5000, 5).is_empty());
    std::fs::write(&notes, "a\nb\nc\nd\n").unwrap();
    assert_eq!(unread_notes(&notes, &seen, 1000, 5000, 5), ["c", "d"]);
    assert!(unread_notes(&notes, &seen, 1000, 5000, 5).is_empty());
    let long: String = (0..300).fold(String::new(), |mut s, n| { let _ = writeln!(s, "line {n}"); s });
    std::fs::write(&notes, &long).unwrap();
    assert_eq!(unread_notes(&notes, &seen, 100, 5000, 3).len(), 296);
    assert_eq!(std::fs::read_to_string(&notes).unwrap(), "line 297\nline 298\nline 299\n");
    std::fs::write(&notes, "line 297\nline 298\nline 299\nnew\n").unwrap();
    assert_eq!(unread_notes(&notes, &seen, 1000, 5000, 3), ["new"]);
    std::fs::write(&notes, "x".repeat(6000)).unwrap();
    assert!(unread_notes(&notes, &seen, 100, 5000, 3).is_empty());
    assert!(!notes.exists());
    std::fs::create_dir(&notes).unwrap();
    assert!(unread_notes(&notes, &seen, 100, 5000, 3).is_empty());
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn a_headline_is_plain_words_cut_short() {
    assert_eq!(plain_words("  Connected \n\t to   desktop. ", 60), "Connected to desktop.");
    assert_eq!(plain_words("a\u{202E}b\u{0}c\u{200B}d", 60), "abcd");
    assert_eq!(plain_words("<b>Hi</b> &amp; bye", 60), "<b>Hi</b> &amp; bye");
    let long = plain_words(&"word ".repeat(40), HEADLINE_MAX);
    assert!(long.chars().count() <= HEADLINE_MAX && long.ends_with('\u{2026}'), "{long}");
    assert_eq!(plain_words(&"\u{e9}".repeat(100), 10).chars().count(), 10);
}
