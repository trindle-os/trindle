use std::fmt::Write as _;
use super::*;
use crate::plugins::fixtures::*;
use crate::plugins::manifest::check;

#[test]
fn choices_come_one_a_line_with_an_optional_label() {
    let (dir, got) = home_choices("lines", b"abc123|Desktop one\n\nxyz\r\n  spaced  |  Padded  label \nabc123|Again\n");
    assert_eq!(got, [("abc123".into(), "Desktop one".into()), ("xyz".into(), "xyz".into()), ("spaced".into(), "Padded label".into())]);
    let _ = std::fs::remove_dir_all(&dir);

    let (dir, got) = home_choices("bar", b"a|b|c\nd|\n");
    assert_eq!(got, [("a".into(), "b|c".into()), ("d".into(), "d".into())]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn hostile_choices_are_cut_cleaned_and_counted() {

    let many: String = (0..500).fold(String::new(), |mut s, n| { let _ = writeln!(s, "v{n}|Label {n}"); s });
    let (dir, got) = home_choices("many", many.as_bytes());
    assert_eq!(got.len(), CHOICES_MAX);
    assert_eq!(got[31].0, "v31");
    let _ = std::fs::remove_dir_all(&dir);
    let long = format!("{}|{}\n", "v".repeat(200), "L".repeat(200));
    let (dir, got) = home_choices("long", long.as_bytes());
    assert_eq!(got[0].0.chars().count(), CHOICE_TEXT_MAX);
    assert_eq!(got[0].1.chars().count(), CHOICE_TEXT_MAX);
    let _ = std::fs::remove_dir_all(&dir);

    let (dir, got) = home_choices("odd", "a\u{1b}[31mb|<b>x</b>\u{202e}y\tz\u{7}\n\u{0}\n\u{200b}\n".as_bytes());
    assert_eq!(got, [("a[31mb".into(), "<b>x</b>y z".into())]);
    assert!(!got[0].0.chars().any(char::is_control));
    let _ = std::fs::remove_dir_all(&dir);

    let (dir, got) = home_choices("binary", &[0xff, 0xfe, 0, 1, 2, b'\n', 0x80, b'\n']);
    assert!(got.iter().all(|(v, l)| !v.chars().any(char::is_control) && !l.chars().any(char::is_control)));
    let _ = std::fs::remove_dir_all(&dir);

    let (dir, got) = home_choices("big", "x\n".repeat(5000).as_bytes());
    assert!(got.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_choices_file_that_is_a_link_a_folder_or_missing_gives_none() {
    let dir = home_with("links", None);
    assert!(choices_in(&dir, "found.txt").is_empty());
    std::os::unix::fs::symlink("/etc/hostname", dir.join("found.txt")).unwrap();
    assert!(choices_in(&dir, "found.txt").is_empty(), "a link is not followed");
    std::fs::create_dir(dir.join("dir.txt")).unwrap();
    assert!(choices_in(&dir, "dir.txt").is_empty());

    std::fs::create_dir(dir.join("sub")).unwrap();
    std::fs::write(dir.join("sub/found.txt"), "a\n").unwrap();
    assert!(choices_in(&dir, "sub/found.txt").is_empty() && choices_in(&dir, "../found.txt").is_empty());
    std::fs::write(dir.join("ok.txt"), "a|A\n").unwrap();
    assert_eq!(choices_in(&dir, "ok.txt"), [("a".to_string(), "A".to_string())]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_note_shows_one_plain_line_from_the_plugins_file_and_holds_no_setting() {
    let noted = ROLES.replace("Fields=role;on;town;side;speed;desk", "Fields=role;on;town;side;speed;desk;state")
        + "\n[Field state]\nKind=note\nTextFrom=status.txt\nShowWhen=role=controls\n";
    let dir = folder_with("noted", &noted);
    let p = check(&dir).unwrap();
    let f = p.fields.last().unwrap();
    assert_eq!((f.kind.clone(), f.label.as_str()), (Kind::Note("status.txt".into()), ""));
    let home = dir.join("home");
    std::fs::create_dir_all(&home).unwrap();

    assert_eq!(note_in(&home, f), "");
    std::fs::write(home.join("status.txt"), "\n  \n").unwrap();
    assert_eq!(note_in(&home, f), "");

    let long = "x".repeat(300);
    std::fs::write(home.join("status.txt"), format!("\n Found\u{202E} the\tdesk. \nsecond line\n{long}")).unwrap();
    assert_eq!(note_in(&home, f), "Found the desk.");
    std::fs::write(home.join("status.txt"), &long).unwrap();
    assert_eq!(note_in(&home, f).chars().count(), NOTE_MAX);

    std::fs::remove_file(home.join("status.txt")).unwrap();
    std::fs::write(dir.join("elsewhere"), "secret").unwrap();
    std::os::unix::fs::symlink(dir.join("elsewhere"), home.join("status.txt")).unwrap();
    assert_eq!(note_in(&home, f), "");
    let _ = std::fs::remove_dir_all(&dir);

    for (bad, why) in [
        (noted.replace("TextFrom=status.txt", ""), "[Field state] is a note, so needs TextFrom"),
        (noted.replace("TextFrom=status.txt", "TextFrom=../status.txt"), "[Field state] is a note, so needs TextFrom"),
        (noted.replace("TextFrom=status.txt", "TextFrom=status.txt\nDefault=hello"), "has no Default"),
        (noted.replace("Fields=role;on;town;side;speed;desk;state", "Fields=state;role;on;town;side;speed;desk")
            .replace("TextFrom=status.txt\nShowWhen=role=controls", "TextFrom=status.txt")
            .replace("Label=Faster\nShowWhen=on=true", "Label=Faster\nShowWhen=state=x"), "names the note state"),
    ] {
        let dir = folder_with("noted-bad", &bad);
        let said = check(&dir).unwrap_err().join("\n");
        assert!(said.contains(why), "{why}: {said}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

#[test]
fn show_when_is_read_as_one_other_field_and_its_value() {
    let dir = folder_with("roles", ROLES);
    let p = check(&dir).unwrap();
    let when: Vec<Option<(&str, &str)>> = p.fields.iter()
        .map(|f| f.show_when.as_ref().map(|(o, v)| (o.as_str(), v.as_str()))).collect();
    assert_eq!(when, [None, None, None, Some(("role", "controls")), Some(("on", "true")), Some(("role", "controls"))]);
    let _ = std::fs::remove_dir_all(&dir);

    let odd = ROLES.replace("Choices=left;right\nShowWhen=role=controls", "Choices=left;right\nShowWhen= town = a=b c ");
    let dir = folder_with("roles-odd", &odd);
    let p = check(&dir).unwrap();
    assert_eq!(p.fields[3].show_when, Some(("town".to_string(), "a=b c".to_string())));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_field_shows_only_while_the_other_holds_that_value() {
    let dir = folder_with("roles-shown", ROLES);
    let p = check(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    let shown = |v: &BTreeMap<String, String>, offered: &[(String, String)]| -> Vec<String> {
        p.fields.iter().filter(|f| field_shown(&p, f, v, offered)).map(|f| f.name.clone()).collect()
    };

    assert_eq!(shown(&vals(&[]), &two()), ["role", "on", "town", "side", "desk"]);

    let v = vals(&[("role", "controlled"), ("on", "true"), ("side", "right")]);
    assert_eq!(shown(&v, &two()), ["role", "on", "town", "speed"]);

    assert_eq!(v.get("side").map(String::as_str), Some("right"));

    for near in ["Controls", "control", "controls ", " controls", "controls;controlled", "*", ""] {
        assert!(!shown(&vals(&[("role", near)]), &two()).contains(&"side".to_string()), "{near:?}");
    }

    for (kept, on) in [("true", true), ("false", false), ("", false), ("TRUE", false), ("1", false), ("yes", false)] {
        assert_eq!(shown(&vals(&[("on", kept)]), &two()).contains(&"speed".to_string()), on, "{kept:?}");
    }

    let mut lost = p.clone();
    lost.fields.retain(|f| f.name != "role");
    assert!(!field_shown(&lost, &lost.fields[2], &vals(&[("role", "controls")]), &two()));
}
