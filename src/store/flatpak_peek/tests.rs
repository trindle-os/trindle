use super::*;
use crate::store::dropped::Reference;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

const SAMPLE: &[u8] = include_bytes!("../testdata/bundle-sample.bin");
const METADATA: &str = "[Application]\nname=org.example.Test\n\n[Context]\nshared=network;ipc;\nsockets=x11;wayland;pulseaudio;session-bus;\ndevices=all;dri;\n\
filesystems=home;/opt/x:ro;xdg-documents;!xdg-music;host-os;\n\n[Session Bus Policy]\norg.freedesktop.Notifications=talk\norg.example.Test=own\norg.example.Seen=see\n\n\
[System Bus Policy]\norg.freedesktop.NetworkManager=talk\n";

fn has(said: &[String], w: impl AsRef<str>) -> bool {
    said.iter().any(|s| s == w.as_ref())
}

fn words(lines: &[Line]) -> Vec<String> {
    lines.iter().map(|l| l.words.clone()).collect()
}

fn scratch(name: &str, bytes: &[u8]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-peek-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

#[test]
fn the_permissions_are_listed_from_a_programs_metadata() {
    let lines = lines_of(METADATA).unwrap();
    let said = words(&lines);
    for expect in [t!("store-fp-session-bus"), t!("store-fp-devices"), t!("store-fp-x11"), t!("store-fp-network"), t!("store-fp-sound")] {
        assert!(has(&said, expect), "{expect}");
    }
    assert!(has(&said, t!("store-fp-files", what = t!("store-fp-where-home"))));
    assert!(has(&said, t!("store-fp-files-read", what = t!("store-fp-where-path", path = "/opt/x"))));
    assert!(has(&said, t!("store-fp-files", what = t!("store-fp-where-path", path = "~/Documents"))));
    assert!(has(&said, t!("store-fp-files-read", what = t!("store-fp-where-system"))) || has(&said, t!("store-fp-files", what = t!("store-fp-where-system"))));
    assert!(has(&said, t!("store-fp-talk", name = "org.freedesktop.Notifications")));
    assert!(has(&said, t!("store-fp-own", name = "org.example.Test")));
    assert!(has(&said, t!("store-fp-talk-system", name = "org.freedesktop.NetworkManager")));
    assert!(!said.iter().any(|w| w.contains("Seen") || w.contains("Music")), "seeing a name and a taken-away folder are not reach");
    let first_talk = said.iter().position(|w| w.contains("Notifications")).unwrap();
    assert!(said.iter().position(|w| *w == t!("store-fp-session-bus")).unwrap() < first_talk, "the far-reaching come first");
}

#[test]
fn a_program_that_asks_for_nothing_has_an_empty_list() {
    assert_eq!(lines_of("[Application]\nname=org.example.Quiet\n[Context]\nsockets=wayland;\ndevices=dri;\n"), Some(Vec::new()));
}

#[test]
fn odd_metadata_is_turned_away_or_read_safely() {
    assert!(lines_of("").is_none());
    assert!(lines_of("just words\nno groups=here\n").is_none());
    assert!(lines_of(&format!("[Application]\n{}", "x".repeat(METADATA_MOST))).is_none(), "too big");
    let junk = "[Application]\n=\n[Context]\nfilesystems=;;;:ro;!;home:bogus;\nsockets\n[[\n]\n";
    assert!(lines_of(junk).is_some_and(|l| l.len() <= LINES_MOST));
    let evil = "[Application]\n[Context]\nfilesystems=/tmp/a\u{202e}b;/tmp/\nx;..;xdg-nonsense;\n[Session Bus Policy]\nbad name=talk\n=own\nok.name=talk\n";
    let said = words(&lines_of(evil).unwrap());
    assert!(said.iter().all(|w| !w.contains('\u{202e}') && !w.contains("bad name")));
    assert!(has(&said, t!("store-fp-talk", name = "ok.name")));
}

#[test]
fn a_flood_of_entries_is_never_shown_in_part() {
    let flood = |n: usize| {
        let mut text = String::from("[Application]\n[Session Bus Policy]\n");
        for i in 0..n {
            text.push_str(&format!("org.example.N{i}=talk\n"));
        }
        text.push_str("[Context]\nfilesystems=home;home;home;\n");
        text
    };
    let text = flood(100);
    let lines = lines_of(&text).unwrap();
    assert_eq!(lines.len(), 102, "the names, the home folder and, since home can be changed, that it can run any program");
    assert_eq!(lines.iter().filter(|l| l.words == t!("store-fp-escape")).count(), 1);
    assert_eq!(lines.iter().filter(|l| l.words == t!("store-fp-files", what = t!("store-fp-where-home"))).count(), 1);
    assert!(lines_of(&flood(LINES_MOST + 1)).is_none(), "more lines than are shown: not read, so none is left out of sight");
    let big = flood(TEXT_LINES_MOST);
    assert!(big.len() < METADATA_MOST && lines_of(&big).is_none(), "more lines of text than are looked at");
}

#[test]
fn a_bundles_header_is_read_without_the_rest() {
    let file = scratch("sample.flatpak", SAMPLE);
    let got = read_bundle(&file).unwrap();
    assert_eq!(got.id, "org.example.Test");
    assert!(got.metadata.starts_with("[Application]\nname=org.example.Test\n"));
    let said = words(&lines_of(&got.metadata).unwrap());
    assert!(has(&said, t!("store-fp-network")) && has(&said, t!("store-fp-devices")));
}

#[test]
fn a_hostile_or_broken_bundle_is_never_read_past_its_bytes() {
    assert!(read_bundle(Path::new("/nonexistent/x.flatpak")).is_none());
    assert!(read_bundle(Path::new("/")).is_none(), "a folder");
    for len in [0, 1, 4, 20, 100, 1000, SAMPLE.len() - 1] {
        assert!(read_bundle(&scratch("cut.flatpak", &SAMPLE[..len])).is_none(), "cut at {len}");
    }
    for at in [0, 3, 8, 16, 60, 500, 1100, 1244] {
        let mut bad = SAMPLE.to_vec();
        for b in &mut bad[at..(at + 8).min(SAMPLE.len() - 4)] {
            *b = 0xff;
        }
        let _ = read_bundle(&scratch("bad.flatpak", &bad));
    }
    let mut far = SAMPLE.to_vec();
    let n = far.len();
    far[n - 2..].copy_from_slice(&[0xff, 0xff]);
    assert!(read_bundle(&scratch("far.flatpak", &far)).is_none(), "an end past the file");
    let mut none = SAMPLE.to_vec();
    none[n - 2..].copy_from_slice(&[0, 0]);
    assert!(read_bundle(&scratch("zero.flatpak", &none)).is_none(), "an empty header");
    assert!(read_bundle(&scratch("text.flatpak", &vec![b'a'; 5000])).is_none(), "plain text");
    assert!(read_bundle(&scratch("zeros.flatpak", &vec![0u8; 5000])).is_none(), "zeros");
}

#[test]
fn a_bundle_of_another_kind_is_not_taken_for_a_program() {
    assert_eq!(app_id("app/org.example.Test/x86_64/stable").as_deref(), Some("org.example.Test"));
    assert!(app_id("runtime/org.example.Platform/x86_64/1").is_none());
    assert!(app_id("app/-x/x86_64/1").is_none());
    assert!(app_id("app/a b/x86_64/1").is_none());
    assert!(app_id("").is_none());
}

#[test]
fn the_bundle_path_is_never_read_as_an_option() {
    let argv = bundle_install_argv(Path::new("/c/--user.flatpak"));
    assert_eq!(&argv[argv.len() - 2..], ["--bundle", "/c/--user.flatpak"]);
}

#[test]
fn the_pin_command_names_the_shown_commit_and_a_commit_is_only_hex() {
    let about = Reference { name: "org.example.App".into(), host: "x.example.com".into(), url: "https://x.example.com/".into(), branch: Some("stable".into()), gpg_key: None, runtime_repo: None };
    let commit = "ab".repeat(32);
    let argv = pin_argv(&about, &commit);
    assert_eq!(&argv[argv.len() - 2..], [format!("--commit={commit}"), "org.example.App//stable".to_string()]);
    assert!(commit_ok(&commit));
    for bad in ["", "ab", &"AB".repeat(32), &"ab".repeat(33), &format!("--user{}", "a".repeat(58)), &format!("{}\n", "a".repeat(63))] {
        assert!(!commit_ok(bad), "{bad:?}");
    }
    let bare = Reference { branch: None, ..about };
    assert_eq!(pin_argv(&bare, &commit).last().unwrap(), "org.example.App");
}

fn said_of(context: &str) -> Vec<String> {
    words(&lines_of(&format!("[Application]\nname=org.example.T\n{context}")).unwrap())
}

#[test]
fn fallback_x11_and_the_other_devices_and_features_are_listed() {
    assert!(has(&said_of("[Context]\nsockets=fallback-x11;\n"), t!("store-fp-x11")));
    assert_eq!(said_of("[Context]\nsockets=x11;fallback-x11;\n").len(), 1, "said once");
    assert!(has(&said_of("[Context]\ndevices=kvm;\n"), t!("store-fp-kvm")));
    assert!(has(&said_of("[Context]\ndevices=shm;\n"), t!("store-fp-shm")));
    assert!(has(&said_of("[Context]\nfeatures=multiarch;\n"), t!("store-fp-multiarch")));
    assert!(has(&said_of("[Context]\nfeatures=devel;\n"), t!("store-fp-devel")));
    assert!(said_of("[Context]\nfeatures=per-app-dev-shm;\nsockets=!x11;\n").is_empty(), "known and harmless, or taken away");
}

#[test]
fn what_it_cannot_say_is_one_last_line_of_other_settings() {
    let other = t!("store-fp-other");
    for context in ["[Context]\nsockets=wayland;brand-new;\n", "[Context]\nnew-key=1\n", "[Environment]\nFOO=bar\n", "[Policy Tracker3]\ndbus:org.freedesktop.Tracker3.Miner.Files=tracker:Documents\n"] {
        let said = said_of(context);
        assert_eq!(said.last().map(String::as_str), Some(other), "{context}");
    }
    assert!(!has(&said_of("[Context]\nsockets=wayland;x11;\nshared=network;ipc;\npersistent=.app;\n"), other));
    let mixed = said_of("[Context]\nnew-key=1\nsockets=session-bus;\n");
    assert_eq!(mixed.last().map(String::as_str), Some(other), "after the rest");
    assert_eq!(mixed.len(), 2);
}

#[test]
fn host_escape_bus_names_come_first_in_words_that_say_so() {
    let escape = t!("store-fp-escape");
    for names in ["[Session Bus Policy]\norg.freedesktop.Flatpak=talk\n", "[Session Bus Policy]\norg.freedesktop.systemd1=own\n", "[System Bus Policy]\norg.freedesktop.systemd1=talk\n",
        "[Session Bus Policy]\norg.freedesktop.Flatpak.*=talk\n", "[Session Bus Policy]\norg.freedesktop.*=talk\n"] {
        let said = said_of(&format!("[Context]\nsockets=wayland;\n{names}"));
        assert!(has(&said, escape), "{names}");
        assert!(!said.iter().any(|w| w.contains("systemd1") || w.contains("Flatpak")), "{said:?}");
    }
    let said = said_of("[Context]\nsockets=x11;\n[Session Bus Policy]\norg.freedesktop.Flatpak=talk\n");
    assert_eq!(said[0], escape, "before even the windows of other apps");
    assert!(!has(&said_of("[Session Bus Policy]\norg.freedesktop.Flatpak=see\n"), escape));
    assert!(!has(&said_of("[Session Bus Policy]\norg.freedesktop.FlatpakNot=talk\n"), escape));
}

#[test]
fn the_root_and_system_folders_are_the_system_files_and_come_first() {
    let system = t!("store-fp-files", what = t!("store-fp-where-system"));
    let read = t!("store-fp-files-read", what = t!("store-fp-where-system"));
    for place in ["/", "/usr", "/etc", "/var", "/opt", "/boot", "/srv", "/root", "/home", "/usr/", "/etc:ro"] {
        let said = said_of(&format!("[Context]\nfilesystems=~/Documents;{place};\n"));
        let first = said.iter().find(|w| **w != t!("store-fp-escape")).unwrap();
        assert!(*first == system || *first == read, "{place}: {said:?}");
    }
    let said = said_of("[Context]\nfilesystems=/usr/share/fonts;\n");
    assert!(!has(&said, &system), "a folder inside one is a folder");
}

#[test]
fn a_relative_bundle_path_is_never_read_as_an_option() {
    for rel in ["-x.flatpak", "--user.flatpak", "sub/-x.flatpak", "x.flatpak"] {
        let argv = bundle_install_argv(Path::new(rel));
        let last = argv.last().unwrap();
        assert!(last.starts_with("./") && !last.starts_with('-'), "{rel}: {last}");
        assert_eq!(argv[argv.len() - 2], "--bundle");
    }
    assert_eq!(bundle_install_argv(Path::new("../a.flatpak")).last().unwrap(), "../a.flatpak");
}

#[test]
fn only_a_name_in_the_list_is_installed() {
    let list = "org.example.One\n org.example.Two \n";
    assert!(listed(list, "org.example.One") && listed(list, "org.example.Two"));
    assert!(!listed(list, "org.example") && !listed(list, "example.One") && !listed("", "org.example.One"));
}

fn fake(script: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    static N: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!("trindle-peek-fake-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("flatpak");
    std::fs::write(&file, format!("#!/bin/sh\n{script}\n")).unwrap();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).unwrap();
    TEST.with(|t| *t.borrow_mut() = (Some(dir.join("cache")), Some(dir.clone())));
    dir
}

fn about() -> Reference {
    Reference { name: "org.example.App".into(), host: "x.example.com".into(), url: "https://x.example.com/".into(), branch: Some("stable".into()), gpg_key: None, runtime_repo: None }
}

const PRINT: &str = r#"case "$*" in *--show-commit*) echo "COMMIT";; *--show-metadata*) printf '[Application]\nname=org.example.App\n';; esac"#;

#[test]
fn a_commit_that_is_not_64_lowercase_hex_is_refused_whole() {
    let good = "ab".repeat(32);
    let dir = fake(&PRINT.replace("COMMIT", &good));
    assert_eq!(read_ref(&about()).map(|p| p.commit), Some(good));
    for bad in ["AB".repeat(32), "ab".repeat(31), "--user".to_string(), format!("{}; echo ab", "ab".repeat(32)), String::new()] {
        fake(&PRINT.replace("COMMIT", &bad));
        assert!(read_ref(&about()).is_none(), "{bad:?}");
    }
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn too_much_output_ends_flatpak_and_everything_it_started() {
    let dir = fake("sleep 60 & echo $! > \"$0.child\"; head -c 100000 /dev/zero; wait");
    let started = std::time::Instant::now();
    assert!(flatpak(Some(&dir), "50", &["list"], 1000).is_none());
    assert!(started.elapsed().as_secs() < 20, "it did not wait for the end");
    let child = std::fs::read_to_string(dir.join("flatpak.child")).unwrap().trim().to_string();
    let gone = (0..60).any(|_| {
        std::thread::sleep(std::time::Duration::from_millis(100));
        std::fs::read_to_string(format!("/proc/{child}/stat")).map_or(true, |s| s.contains(") Z"))
    });
    assert!(gone, "the program it started is still running");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn old_throwaway_folders_are_swept_and_nothing_else() {
    let parent = std::env::temp_dir().join(format!("trindle-peek-sweep-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&parent);
    let live = format!("peek-bundle-{}-0", std::process::id());
    for d in ["peek-999999999-0", "peek-bundle-999999999-0", "peek-ref-999999999-1", "keep-me", live.as_str()] {
        std::fs::create_dir_all(parent.join(d)).unwrap();
    }
    std::fs::write(parent.join("peek-file"), b"x").unwrap();
    sweep(&parent, std::time::Duration::from_secs(3600));
    assert!(parent.join("peek-999999999-0").exists(), "a fresh one is left, it may be in use");
    sweep(&parent, std::time::Duration::ZERO);
    assert!(!parent.join("peek-999999999-0").exists() && !parent.join("peek-bundle-999999999-0").exists() && !parent.join("peek-ref-999999999-1").exists());
    assert!(parent.join(&live).exists(), "a running process's copy is never taken, however old");
    assert!(parent.join("keep-me").exists() && parent.join("peek-file").exists());
    let _ = std::fs::remove_dir_all(&parent);
}

#[test]
fn a_bundle_is_copied_whole_into_a_folder_only_the_person_opens() {
    use std::os::unix::fs::PermissionsExt;
    let from = scratch("copy-me.flatpak", b"the bytes");
    let cache = std::env::temp_dir().join(format!("trindle-peek-copy-{}", std::process::id()));
    TEST.with(|t| *t.borrow_mut() = (Some(cache.clone()), None));
    let copy = private_copy(&from).unwrap();
    assert_eq!(std::fs::read(&copy).unwrap(), b"the bytes");
    assert_ne!(copy, from);
    assert_eq!(std::fs::metadata(copy.parent().unwrap()).unwrap().permissions().mode() & 0o777, 0o700);
    std::fs::write(&from, b"changed").unwrap();
    assert_eq!(std::fs::read(&copy).unwrap(), b"the bytes", "what was read is what is installed");
    assert!(private_copy(Path::new("/nonexistent/x.flatpak")).is_none());
    assert!(private_copy(&std::env::temp_dir()).is_none(), "a folder is no bundle");
    let _ = std::fs::remove_dir_all(&cache);
}

fn folder(path: &str) -> String {
    t!("store-fp-files", what = t!("store-fp-where-path", path = path)).to_string()
}

#[test]
fn a_place_with_odd_characters_is_shown_quoted_and_never_left_off() {
    let said = said_of("[Context]\nfilesystems=~/My Files (old);/tmp/a\u{202e}b\"c;xdg-documents/x*y:ro;\n");
    assert!(has(&said, folder("\"~/My Files (old)\"")), "{said:?}");
    assert!(has(&said, folder("\"/tmp/a\\u{202e}b\\\"c\"")), "{said:?}");
    assert!(has(&said, t!("store-fp-files-read", what = t!("store-fp-where-path", path = "\"~/Documents/x*y\""))), "{said:?}");
    let long = said_of(&format!("[Context]\nfilesystems=/tmp/{}(;\n", "a".repeat(300)));
    assert!(long.iter().any(|w| w.contains('…') && w.chars().count() < 200), "{long:?}");
}

#[test]
fn xdg_sub_folders_read_as_folders_and_places_that_let_it_out_say_so_first() {
    let escape = t!("store-fp-escape");
    assert!(has(&said_of("[Context]\nfilesystems=xdg-download/Films;\n"), folder("~/Downloads/Films")));
    let home = std::env::var("HOME").unwrap_or_default();
    for place in ["xdg-config/autostart", "xdg-config/autostart/x.desktop", "~/.bashrc", "home/.profile", "xdg-data/applications", "xdg-config/systemd/user",
        "xdg-run/bus", "xdg-config", "~/.local", "home", "host", "/", "~/Documents/../.bashrc", "~/./.bashrc:create", &format!("{home}/.zshrc")] {
        let said = said_of(&format!("[Context]\nfilesystems={place};\n"));
        assert_eq!(said.first().map(String::as_str), Some(escape), "{place}: {said:?}");
        assert!(!has(&said_of(&format!("[Context]\nfilesystems={place}:ro;\n")), escape), "{place} only looked at");
    }
    assert!(has(&said_of("[Context]\nfilesystems=xdg-config/autostart;\n"), folder("~/.config/autostart")), "and the folder is named too");
    for place in ["xdg-documents", "~/.config/gtk-3.0", "~/.bashrc-old", "xdg-data/fonts", "xdg-run/pipewire-0", "/usr/share/fonts"] {
        assert!(!has(&said_of(&format!("[Context]\nfilesystems={place};\n")), escape), "{place}");
    }
}
