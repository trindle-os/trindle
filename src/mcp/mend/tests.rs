use super::*;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

const ID: &str = "org.example.App";

fn home(tag: &str) -> PathBuf {
    let home = std::env::temp_dir().join(format!("trindle-mend-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(app(&home).join("config/sub")).unwrap();
    std::fs::create_dir_all(app(&home).join("cache")).unwrap();
    home
}

fn app(home: &Path) -> PathBuf {
    home.join(".var/app").join(ID)
}

#[test]
fn one_line_is_changed_and_nothing_else() {
    let text = "# kept\ntop=1\n[General]\nzoom = 2\nother=x\n\n[View]\nzoom=9\n";
    assert_eq!(edited(text, "General", "zoom", "3"), "# kept\ntop=1\n[General]\nzoom = 3\nother=x\n\n[View]\nzoom=9\n");
    assert_eq!(edited(text, "", "top", "0"), "# kept\ntop=0\n[General]\nzoom = 2\nother=x\n\n[View]\nzoom=9\n");
    assert_eq!(edited(text, "General", "new", "a"), "# kept\ntop=1\n[General]\nzoom = 2\nother=x\nnew=a\n\n[View]\nzoom=9\n");
    assert_eq!(edited(text, "Fresh", "k", "v"), format!("{text}\n[Fresh]\nk=v\n"));
    assert_eq!(edited("", "", "k", "v"), "k=v\n");
    assert_eq!(edited("a=1", "", "a", "2"), "a=2", "no newline is added that was not there");
    assert_eq!(value_of(text, "View", "zoom").as_deref(), Some("9"));
    assert_eq!(value_of(text, "General", "zoom").as_deref(), Some("2"));
    assert_eq!(value_of(text, "General", "kept"), None);
    assert_eq!(value_of("#zoom=1\n", "", "zoom"), None, "a comment sets nothing");
}

#[test]
fn a_line_can_only_be_the_one_line_asked_for() {
    assert!(check_line("General", "zoom", "2").is_ok());
    assert!(check_line("", "zoom", "").is_ok());
    for (section, key, value) in [
        ("General", "zoom", "2\n[Other]\nevil=1"), ("General", "zoom", "a\rb"), ("General", "zoom", &"x".repeat(MOST_WORD + 1)),
        ("General", "", "1"), ("General", "a=b", "1"), ("General", " zoom", "1"), ("General", "[x]", "1"), ("General", "#c", "1"),
        ("Gen]eral", "zoom", "1"), ("Gen\neral", "zoom", "1"),
    ] {
        assert!(check_line(section, key, value).is_err(), "{section:?} {key:?} {value:?}");
    }
}

#[test]
fn a_place_must_stay_inside_the_config_folder() {
    assert_eq!(parts("sub/a.conf").unwrap(), [OsStr::new("sub"), OsStr::new("a.conf")]);
    for bad in ["", "/etc/passwd", "../x", "sub/../../x", "./a", "a//b", "a/", "a\nb", "a/b/c/d/e", PART, &"a".repeat(MOST_WORD + 1)] {
        assert!(parts(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn a_settings_file_is_read_and_replaced_whole_with_its_permissions() {
    let home = home("write");
    let file = app(&home).join("config/sub/a.conf");
    std::fs::write(&file, "[General]\nzoom=2\n").unwrap();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o640)).unwrap();
    let (text, mode) = read(&home, ID, "sub/a.conf").unwrap();
    assert_eq!((text.as_str(), mode), ("[General]\nzoom=2\n", 0o640));

    let outside = home.join("private");
    std::fs::write(&outside, "secret").unwrap();
    symlink(&outside, app(&home).join("config/sub").join(PART)).unwrap();
    write(&home, ID, "sub/a.conf", "[General]\nzoom=3\n", mode).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "[General]\nzoom=3\n");
    assert_eq!(std::fs::metadata(&file).unwrap().permissions().mode() & 0o777, 0o640);
    assert_eq!(std::fs::read_to_string(&outside).unwrap(), "secret");
    assert!(!app(&home).join("config/sub").join(PART).exists());
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn links_second_names_big_files_and_other_kinds_are_refused() {
    let home = home("refuse");
    let config = app(&home).join("config");
    let outside = home.join("Documents");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("bank.conf"), "pin=1234\n").unwrap();
    symlink(outside.join("bank.conf"), config.join("link.conf")).unwrap();
    symlink(&outside, config.join("linked")).unwrap();
    std::fs::hard_link(outside.join("bank.conf"), config.join("hard.conf")).unwrap();
    std::fs::write(config.join("big.conf"), vec![b'a'; MOST_FILE as usize + 1]).unwrap();
    std::fs::write(config.join("bin.conf"), [0xff, 0xfe, 0]).unwrap();
    for rel in ["link.conf", "linked/bank.conf", "hard.conf", "big.conf", "bin.conf", "sub", "none.conf"] {
        assert!(read(&home, ID, rel).is_err(), "{rel} read");
    }
    assert!(write(&home, ID, "linked/bank.conf", "pin=0\n", 0o600).is_err(), "never through a linked folder");

    for rel in ["link.conf", "hard.conf"] {
        write(&home, ID, rel, "pin=0\n", 0o600).unwrap();
        assert!(std::fs::symlink_metadata(config.join(rel)).unwrap().is_file());
    }
    assert_eq!(std::fs::read_to_string(outside.join("bank.conf")).unwrap(), "pin=1234\n");

    std::fs::remove_dir_all(&config).unwrap();
    symlink(&outside, &config).unwrap();
    assert!(read(&home, ID, "bank.conf").is_err());
    assert!(names(&home, ID).is_err());
    std::fs::remove_dir_all(app(&home)).unwrap();
    symlink(&outside, app(&home)).unwrap();
    assert!(read(&home, ID, "config/bank.conf").is_err() && has_cache(&home, ID).is_err());
    assert!(read(&home, "../x.y.z", "a").is_err());
    assert_eq!(std::fs::read_to_string(outside.join("bank.conf")).unwrap(), "pin=1234\n");
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn the_list_gives_key_names_never_values_and_never_follows_a_link() {
    let home = home("names");
    let config = app(&home).join("config");
    std::fs::write(config.join("a.conf"), "[Account]\npassword=hunter2\nuser=john\n").unwrap();
    std::fs::write(config.join("sub/b.ini"), "top=1\n").unwrap();
    let outside = home.join("Documents");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("c.conf"), "secret=1\n").unwrap();
    symlink(&outside, config.join("out")).unwrap();
    symlink(outside.join("c.conf"), config.join("c.conf")).unwrap();
    let list = names(&home, ID).unwrap();
    assert_eq!(list, json!([
        { "file": "a.conf", "sections": { "Account": ["password", "user"] } },
        { "file": "sub/b.ini", "sections": { "": ["top"] } },
    ]));
    let text = list.to_string();
    assert!(!text.contains("hunter2") && !text.contains("john") && !text.contains("secret"));
    for i in 0..MOST_FILES + 5 {
        std::fs::write(config.join(format!("many{i:03}.conf")), "").unwrap();
    }
    assert_eq!(names(&home, ID).unwrap().as_array().unwrap().len(), MOST_FILES);
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn a_cache_is_moved_aside_and_put_back_and_a_linked_one_is_left_alone() {
    let home = home("cache");
    let snap = home.join("snap");
    std::fs::create_dir_all(&snap).unwrap();
    std::fs::write(app(&home).join("cache/old"), "x").unwrap();
    assert!(has_cache(&home, ID).unwrap());
    move_cache(&home, ID, &snap, CACHE).unwrap();
    assert!(snap.join("cache/old").is_file());
    assert_eq!(std::fs::read_dir(app(&home).join("cache")).unwrap().count(), 0, "an empty cache is left");
    std::fs::write(app(&home).join("cache/new"), "y").unwrap();
    put_cache_back(&home, ID, &snap, CACHE, "since").unwrap();
    assert!(app(&home).join("cache/old").is_file() && !app(&home).join("cache/new").exists());
    assert!(snap.join("since/new").is_file(), "the cache made since is set aside, not deleted");
    assert!(put_cache_back(&home, ID, &snap, CACHE, "since2").is_err(), "nothing left to put back");

    let outside = home.join("Pictures");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::remove_dir_all(app(&home).join("cache")).unwrap();
    symlink(&outside, app(&home).join("cache")).unwrap();
    assert!(has_cache(&home, ID).is_err());
    assert!(outside.is_dir());
    std::fs::remove_file(app(&home).join("cache")).unwrap();
    assert!(!has_cache(&home, ID).unwrap());
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn trindle_is_never_mended() {
    for id in ["org.trindle.Store", "io.github.Trindle_OS.Voice", "com.example.TrindleHelper"] {
        assert!(trindle_own(id), "{id}");
    }
    assert!(!trindle_own("org.gnome.Calculator"));
}
