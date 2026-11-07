use super::*;
use crate::plugins::fixtures::*;
use crate::plugins::fields::Kind;
use crate::plugins::fields::choices_of_in;
use crate::plugins::fields::field_shown;
use crate::plugins::permissions::asks_more;
use crate::plugins::permissions::other_ports;
use crate::plugins::ports::declared_ports;

#[test]
fn a_plugin_that_conforms_is_read() {
    let dir = folder_with("good", GOOD);
    let p = check(&dir).unwrap();
    assert_eq!((p.id.as_str(), p.name.as_str()), ("org.example.Weather", "Weather"));
    assert_eq!(p.ring.unwrap().2, vec!["bin/go".to_string(), "--show".into()]);
    assert_eq!(p.fields[1].kind, Kind::Choice(vec!["Celsius".into(), "Fahrenheit".into()]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_validator_says_what_is_wrong() {
    let bad = GOOD.replace("Interface=1", "Interface=2").replace("Run=bin/go --show", "Run=/usr/bin/sh")
        .replace("network;", "network;camera;").replace("Kind=text", "Kind=slider");
    let dir = folder_with("bad", &bad);
    std::os::unix::fs::symlink("/etc/passwd", dir.join("peek")).unwrap();
    let problems = check(&dir).unwrap_err().join("\n");
    for said in ["Interface=2", "inside the plugin's folder, not /usr/bin/sh", "camera", "slider", "peek links outside"] {
        assert!(problems.contains(said), "{said} not in: {problems}");
    }
    let _ = std::fs::remove_dir_all(&dir);
    let none = folder_with("none", "[Plugin]\nInterface=1\nId=a.b\nName=A\nVersion=1\n");
    assert!(check(&none).unwrap_err()[0].contains("hooks into nothing"));
    let _ = std::fs::remove_dir_all(&none);
}

#[test]
fn the_screen_reader_needs_no_hook_but_no_other_plugin_may_ask_for_accessibility() {
    let reader = crate::common::own_hands::SCREEN_READER;
    let body = |id: &str| format!("[Plugin]\nInterface=1\nId={id}\nName=R\nVersion=1\nPermissions=sound;accessibility\n");
    let ok = folder_with("reader", &body(reader));
    assert!(check(&ok).is_ok(), "{:?}", check(&ok).err());
    let _ = std::fs::remove_dir_all(&ok);
    let other = folder_with("not-reader", &body("a.b"));
    assert!(check(&other).is_err());
    let _ = std::fs::remove_dir_all(&other);
}

#[test]
fn a_plugin_may_ask_to_control_the_keyboard_mouse_and_clipboard() {
    let dir = folder_with("control", &GOOD.replace("network;notifications", "network;control"));
    assert_eq!(check(&dir).unwrap().asks, ["network", "control"]);
    let _ = std::fs::remove_dir_all(&dir);
    let a = crate::common::access::ask("control");
    assert_eq!(a.name, "control your keyboard, mouse and clipboard");
    assert!(crate::common::access::warning("control").unwrap().contains("trust"));
}

#[test]
fn a_plugin_may_ask_to_share_the_mouse_and_keyboard_apart_from_control() {
    let dir = folder_with("capture", &GOOD.replace("network;notifications", "network;control;capture"));
    assert_eq!(check(&dir).unwrap().asks, ["network", "control", "capture"]);
    let a = crate::common::access::ask("capture");
    assert_eq!(a.name, "share your mouse and keyboard with another computer");
    assert!(crate::common::access::warning("capture").unwrap().contains("trust"));
}

#[test]
fn the_validator_checks_the_ring_menu() {
    let missing = says(&MENU.replace("[Menu disconnect]\nLabel=Disconnect\nRun=bin/go disconnect\n", ""), "m-missing");
    assert!(missing.contains("disconnect, which has no [Menu disconnect] part"), "{missing}");
    let no_label = says(&MENU.replace("Label=Connect", "Label="), "m-label");
    assert!(no_label.contains("[Menu connect] needs a Label"), "{no_label}");
    let long = says(&MENU.replace("Label=Connect", &format!("Label={}", "x".repeat(33))), "m-long");
    assert!(long.contains("at most 32 characters"), "{long}");
    let out = says(&MENU.replace("Run=bin/go connect", "Run=../go"), "m-out");
    assert!(out.contains("[Menu connect] Run must name a program inside the plugin's folder"), "{out}");
    let abs = says(&MENU.replace("Run=bin/go disconnect", "Run=/usr/bin/true"), "m-abs");
    assert!(abs.contains("[Menu disconnect] Run must name a program inside"), "{abs}");
    let none = says(&MENU.replace("Run=bin/go connect", "Run=bin/nothing"), "m-none");
    assert!(none.contains("[Menu connect] Run must name a program inside the plugin's folder, not bin/nothing"), "{none}");
    let twice = says(&MENU.replace("connect;disconnect", "connect;connect"), "m-twice");
    assert!(twice.contains("names connect twice"), "{twice}");
    let odd = says(&MENU.replace("connect;disconnect", "connect;dis connect"), "m-odd");
    assert!(odd.contains("Menu id dis connect may only have"), "{odd}");
    let no_run = says(&MENU.replace("Run=bin/go connect\n", ""), "m-run");
    assert!(no_run.contains("[Menu connect] needs Run"), "{no_run}");
}

#[test]
fn what_a_plugin_calls_things_is_plain_words_of_a_sensible_length() {
    let base = |from: &str, to: &str| says(&GOOD.replace(from, to), "plain");
    for (from, to, said) in [
        ("Name=Weather", "Name=Wea\u{202e}ther", "Name is plain words"),
        ("Name=Weather", &format!("Name={}", "W".repeat(49)), "Name is plain words"),
        ("Name=Weather", "Name=Wea\u{7}ther", "Name is plain words"),
        ("Version=1.0", &format!("Version={}", "1".repeat(33)), "Version is plain words"),
        ("Permissions=", "Summary=Two  spaces\nPermissions=", "Summary is plain words"),
        ("Permissions=", &format!("Summary={}\nPermissions=", "s".repeat(201)), "Summary is plain words"),
        ("Label=Weather", &format!("Label={}", "L".repeat(49)), "[Ring] Label is plain words"),
        ("Label=Town", "Label=To\u{200b}wn", "Label is plain words"),
        ("Choices=Celsius;Fahrenheit", &format!("Choices=Celsius;{}", "F".repeat(49)), "Choices are at most"),
        ("Choices=Celsius;Fahrenheit", "Choices=Celsius;Fah\u{202e}renheit", "Choices are at most"),
    ] {
        let got = base(from, to);
        assert!(got.contains(said), "{said:?} not in: {got}");
    }
    assert_eq!(says(GOOD, "plain-ok"), "");
}

#[test]
fn a_ring_menu_has_at_most_eight_items() {
    let ids: Vec<String> = (0..9).map(|n| format!("i{n}")).collect();
    let mut ini = format!("[Plugin]\nInterface=1\nId=a.b\nName=A\nVersion=1\n\n[Ring]\nIcon=icon.svg\nLabel=A\nRun=bin/go\nMenu={}\n", ids.join(";"));
    for i in &ids {
        ini += &format!("\n[Menu {i}]\nLabel=Item {i}\nRun=bin/go {i}\n");
    }
    assert!(says(&ini, "m-nine").contains("at most 8 are allowed"));
    let eight = ini.replace(";i8", "").replace("\n[Menu i8]\nLabel=Item i8\nRun=bin/go i8\n", "");
    assert_eq!(says(&eight, "m-eight"), "");
}

#[test]
fn a_choice_may_take_its_choices_from_a_file_of_the_plugin() {
    let p = check(&folder_with("from", FOUND)).unwrap();
    assert_eq!(p.fields[0].choices_from.as_deref(), Some("found.txt"));
    let none = says(&FOUND.replace("ChoicesFrom=found.txt\n", ""), "f-none");
    assert!(none.contains("needs Choices with two or more, or ChoicesFrom a file"), "{none}");
    let both = FOUND.replace("ChoicesFrom=found.txt", "Choices=a;b\nChoicesFrom=found.txt\nDefault=a");
    assert_eq!(says(&both, "f-both"), "");
    for bad in ["../found.txt", "/etc/passwd", "a/b", ".hidden", "found txt", "f\u{1b}x", &"x".repeat(65), "..", "a;b"] {
        let got = says(&FOUND.replace("found.txt", bad), "f-bad");
        assert!(got.contains("ChoicesFrom is a file name in the plugin's home"), "{bad:?}: {got}");
    }
}

#[test]
fn what_a_newer_version_asks_for_beyond_the_one_installed() {
    let ini = |asks: &str, listen: &str| GOOD.replace("network;notifications", asks) + &format!("\n[Network]\nListen={listen}\n");
    let old_dir = folder_with("more-old", &ini("network;listen", "tcp:24810;udp:24811"));
    let old = check(&old_dir).unwrap();
    let v = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let d = |list: &[&str]| list.iter().map(|s| Door::parse(s).unwrap()).collect::<Vec<Door>>();
    assert!(asks_more(&old, &v(&["listen", "network"]), Some(&d(&["udp:24811", "tcp:24810"]))).is_empty());
    assert!(asks_more(&old, &v(&["network"]), None).is_empty());
    assert!(asks_more(&old, &v(&["network", "listen"]), None).is_empty());
    assert_eq!(asks_more(&old, &v(&["network", "listen", "files", "control"]), None),
               ["your Documents, Downloads, Pictures, Music, Videos and Desktop folders", "control your keyboard, mouse and clipboard"]);
    for ports in [&["tcp:24810", "udp:24811", "tcp:24812"][..], &["tcp:24810"], &["tcp:24810", "udp:24899"]] {
        assert_eq!(asks_more(&old, &old.asks, Some(&d(ports))), [other_ports().to_string()], "{ports:?}");
    }
    let _ = std::fs::remove_dir_all(&old_dir);
}

#[test]
fn ids_cannot_climb_out() {
    assert!(good_id("org.example.Weather") && good_id("a-b"));
    assert!(!good_id("..") && !good_id("a/b") && !good_id(".hidden") && !good_id(""));
}

#[test]
fn root_reads_the_ports_from_the_manifest_text_alone_and_judges_them_again() {
    let ports = |l: &str| declared_ports(&manifest(l)).map(|d| d.iter().map(|d| d.to_string()).collect::<Vec<_>>());
    assert_eq!(ports("tcp:24810;udp:24811").unwrap(), ["tcp:24810", "udp:24811"]);
    assert!(declared_ports(&manifest("tcp:24810").replace("Interface=1", "Interface=2")).is_err());
    assert!(declared_ports(&manifest("tcp:24810").replace("network;listen", "listen")).is_err());
    assert!(declared_ports(&manifest("tcp:24810").replace("network;listen", "network")).is_err());
    assert!(declared_ports("[Plugin]\nInterface=1\nPermissions=network;listen\n").is_err(), "no ports named");
}

#[test]
fn a_choice_from_a_file_with_fewer_than_two_is_not_shown() {
    let dir = folder_with("roles-few", ROLES);
    let p = check(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    let desk = p.fields.iter().find(|f| f.name == "desk").unwrap();
    let side = p.fields.iter().find(|f| f.name == "side").unwrap();
    let home = home_with("few", None);
    let see = |home: &Path| field_shown(&p, desk, &vals(&[]), &choices_of_in(home, desk));
    assert!(!see(&home), "no file");
    std::fs::write(home.join("found.txt"), "").unwrap();
    assert!(!see(&home), "an empty file");
    std::fs::write(home.join("found.txt"), "\n\n  \n").unwrap();
    assert!(!see(&home), "only empty lines");
    std::fs::write(home.join("found.txt"), "one|One\n").unwrap();
    assert!(!see(&home), "one choice is no choice");
    std::fs::write(home.join("found.txt"), "one|One\none|Again\n").unwrap();
    assert!(!see(&home), "the same value twice is one");
    std::fs::write(home.join("found.txt"), "one|One\ntwo\n").unwrap();
    assert!(see(&home), "two or more");
    assert!(!field_shown(&p, desk, &vals(&[("role", "controlled")]), &choices_of_in(&home, desk)));
    std::fs::remove_file(home.join("found.txt")).unwrap();
    std::fs::write(home.join("real.txt"), "one\ntwo\n").unwrap();
    std::os::unix::fs::symlink(home.join("real.txt"), home.join("found.txt")).unwrap();
    assert!(!see(&home), "a link is not followed");
    std::fs::remove_file(home.join("found.txt")).unwrap();
    std::fs::create_dir(home.join("found.txt")).unwrap();
    assert!(!see(&home), "a folder");
    std::fs::remove_dir(home.join("found.txt")).unwrap();
    let pipe = std::ffi::CString::new(home.join("found.txt").to_string_lossy().as_bytes()).unwrap();
    // SAFETY: `pipe` is a valid C string naming a path in this test's own folder.
    assert_eq!(unsafe { libc::mkfifo(pipe.as_ptr(), 0o600) }, 0);
    assert!(!see(&home), "a pipe is not waited on");
    std::fs::remove_file(home.join("found.txt")).unwrap();
    std::fs::write(home.join("found.txt"), "x\ny\n".repeat(5000)).unwrap();
    assert!(!see(&home), "over 8 KB is not read");
    assert!(field_shown(&p, side, &vals(&[]), &[]));
    let _ = std::fs::remove_dir_all(&home);
    let both = ROLES.replace("ChoicesFrom=found.txt", "Choices=a;b\nChoicesFrom=found.txt");
    let dir = folder_with("roles-both", &both);
    let p = check(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    let desk = p.fields.iter().find(|f| f.name == "desk").unwrap();
    let home = home_with("both", None);
    assert!(field_shown(&p, desk, &vals(&[]), &choices_of_in(&home, desk)));
    std::fs::write(home.join("found.txt"), "only\n").unwrap();
    assert!(!field_shown(&p, desk, &vals(&[]), &choices_of_in(&home, desk)), "the file's one line is what is offered");
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn the_validator_refuses_a_bad_show_when_in_plain_words() {
    let with = |field: &str, when: &str| {
        let at = format!("[Field {field}]\n");
        assert!(ROLES.contains(&at));
        ROLES.replace("ShowWhen=role=controls\n\n[Field speed]", "\n[Field speed]").replace("ShowWhen=on=true\n", "")
            .replace("ShowWhen=role=controls\n", "").replace(&at, &format!("{at}ShowWhen={when}\n"))
    };
    assert_eq!(says(&with("side", "role=controls"), "sw-ok"), "");
    let long = format!("role={}", "x".repeat(49));
    let huge = format!("role={}", "x".repeat(200));
    for (field, when, said) in [
        ("side", "role", "[Field side] ShowWhen is written field=value"),
        ("side", "=controls", "the field's name may only have letters"),
        ("side", "ro le=controls", "the field's name may only have letters"),
        ("side", "../role=controls", "the field's name may only have letters"),
        ("side", "role=", "needs a value of plain words"),
        ("side", long.as_str(), "needs a value of plain words"),
        ("side", huge.as_str(), "is at most 128 characters"),
        ("side", "town=a\u{1b}[31mb", "needs a value of plain words"),
        ("side", "town=a\u{202e}b", "needs a value of plain words"),
        ("side", "town=a  b", "needs a value of plain words"),
        ("side", "side=left", "names its own field"),
        ("side", "nowhere=1", "names the field nowhere, which is not in this plugin's Fields"),
        ("side", "speed=true", "names the field speed, which comes after it in Fields"),
        ("role", "side=left", "names the field side, which comes after it in Fields"),
        ("side", "on=yes", "names the switch on, so its value is true or false"),
        ("side", "role=Controls", "is not one of the Choices of role"),
        ("side", "role=controls;controlled", "is not one of the Choices of role"),
    ] {
        let got = says(&with(field, when), "sw-bad");
        assert!(got.contains(said), "{field} ShowWhen={when:?}: {got}");
        assert!(got.contains(&format!("[Field {field}] ShowWhen")), "the field is named: {got}");
    }
    let chain = ROLES.replace("ShowWhen=on=true", "ShowWhen=side=left");
    let got = says(&chain, "sw-chain");
    assert!(got.contains("[Field speed] ShowWhen names the field side, which has a ShowWhen of its own"), "{got}");
    assert_eq!(says(&with("side", "town=Mildenhall"), "sw-text"), "");
    let filled = ROLES.replace("Fields=role;on;town;side;speed;desk", "Fields=desk;role;on;town;side;speed")
        .replace("ChoicesFrom=found.txt\nShowWhen=role=controls", "ChoicesFrom=found.txt")
        .replace("ShowWhen=on=true", "ShowWhen=desk=anything at all");
    assert_eq!(says(&filled, "sw-filled"), "");
}
