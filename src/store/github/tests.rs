//! Tests for github.rs.

#[cfg(test)]
use super::*;

#[test]
fn keeps_only_files_for_this_computer() {
    let assets = serde_json::json!([
        {"name": "App-1.0-x86_64.AppImage", "browser_download_url": "u1", "size": 5},
        {"name": "App-1.0-aarch64.AppImage", "browser_download_url": "u2", "size": 5},
        {"name": "app.flatpak", "browser_download_url": "u3", "size": 5},
        {"name": "App-1.0.tar.gz", "browser_download_url": "u4", "size": 5},
    ]);
    let names: Vec<String> = pick_files(&assets).into_iter().map(|f| f.name).collect();
    if std::env::consts::ARCH == "aarch64" {
        assert_eq!(names, vec!["App-1.0-aarch64.AppImage", "app.flatpak"]);
    } else {
        assert_eq!(names, vec!["App-1.0-x86_64.AppImage", "app.flatpak"]);
    }
}

fn kept(name: &str) -> Kept {
    let dir = std::env::temp_dir().join(format!("trindle-github-kept-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    Kept { dir }
}

#[test]
fn what_github_answered_is_not_asked_again_while_it_is_fresh() {
    let k = kept("fresh");
    let file = Download { name: "Example-1.0-x86_64.AppImage".into(), url: "https://example.com/a".into(), size: 5_000_000, kind: Kind::AppImage };
    let bundle = Download { name: "example.flatpak".into(), url: "https://example.com/f".into(), size: 7, kind: Kind::Flatpak };
    let rel = Release { tag: "v1.0".into(), files: vec![file, bundle], published: Some(1_700_000_000) };
    let found: Vec<Found> = (1..=12).map(|n| Found { repo: format!("john-smith/example-{n}"), name: format!("Example {n}"),
                                                     summary: "Made up".into(), stars: Some(n), pushed: Some(n), created: Some(n),
                                                     ..Default::default() }).collect();
    let at = 1_000_000;

    assert!(k.search("example", at).is_none());
    let unasked = |k: &Kept, now: u64| found.iter().filter(|f| k.release(&f.repo, now).is_none()).count();
    assert_eq!(2 + unasked(&k, at), 14, "a first search is two searches and twelve looks");
    k.keep_search("example", &found, at);
    for (n, f) in found.iter().enumerate() {
        let none: Result<Release, String> = Err(no_files().into());
        let some: Result<Release, String> = Ok(rel.clone());
        k.keep_release(&f.repo, if n % 2 == 0 { &some } else { &none }, at);
    }

    let again = at + 600;
    assert_eq!(k.search("example", again).map(|f| f.len()), Some(12));
    assert_eq!(unasked(&k, again), 0);
    let back = k.release("john-smith/example-1", again).unwrap().unwrap();
    assert_eq!((back.tag.as_str(), back.files.len(), back.files[1].kind.clone(), back.files[0].size), ("v1.0", 2, Kind::Flatpak, 5_000_000));
    assert_eq!(k.release("john-smith/example-2", again), Some(Err(no_files().to_string())), "nothing to install is an answer too");

    assert_eq!(back.published, Some(1_700_000_000));
    assert_eq!(k.search("example", again).unwrap()[3].created, Some(4));

    assert!(k.search("examples", again).is_none());
    assert_eq!(2 + unasked(&k, again), 2);

    assert!(k.search("example", at + SEARCH_KEPT).is_none());
    assert!(k.release("john-smith/example-1", at + SEARCH_KEPT).is_some());
    assert!(k.release("john-smith/example-1", at + RELEASE_KEPT).is_none());
    assert!(k.release("john-smith/example-1", at - 1).is_none());

    let k = kept("half");
    let half = both(found[..3].to_vec(), Err(limit_words(true, Some(40), false)));
    assert_eq!((half.found.len(), half.missed.is_some()), (3, true));
    k.keep_searched("example", &half, at);
    assert!(k.search("example", at).is_none(), "an answer missing its Flatpak half is not given back as the whole");
    let whole = both(found[..3].to_vec(), Ok(found[2..5].to_vec()));
    assert_eq!(whole.missed, None);
    assert_eq!(whole.found.iter().map(|f| f.stars.unwrap()).collect::<Vec<_>>(), vec![5, 4, 3, 2, 1], "each project once, by stars");
    k.keep_searched("example", &whole, at);
    assert_eq!(k.search("example", at).map(|f| f.len()), Some(5));
    let _ = std::fs::remove_dir_all(&k.dir);
    let said = half_words(half.missed.as_deref().unwrap());
    assert!(said.starts_with("Only projects with an AppImage were searched for: GitHub did not answer the search for Flatpaks. GitHub has stopped answering"), "{said}");

    let k = kept("failures");
    for e in [limit_words(false, Some(600), false), "GitHub could not be reached.".to_string(), "GitHub has no releases for this project.".to_string()] {
        k.keep_release("john-smith/example-1", &Err(e), at);
        assert!(k.release("john-smith/example-1", at).is_none());
    }

    for odd in ["../../etc/passwd", "a/b/c", "", ".hidden", "a b"] {
        k.keep_release(odd, &Ok(rel.clone()), at);
        assert!(k.release(odd, at).is_none(), "{odd}");
    }
    assert!(std::fs::read_dir(&k.dir).map_or(true, |d| d.count() == 0));
    let _ = std::fs::remove_dir_all(&k.dir);
    let _ = std::fs::remove_dir_all(kept("fresh").dir);
}

#[test]
fn the_limit_is_said_in_plain_words_with_when_it_ends_and_where_the_key_goes() {
    let now = 1_000;

    let said = answered("403", "0 2380 core ", "{}", now, false).unwrap_err();
    assert_eq!(said, "GitHub has stopped answering for now: it answers 60 questions an hour from one address without a key. \
                      It will answer again in about 23 minutes. A GitHub key raises that to 5,000: press the key at the top right of Search GitHub.");
    assert!(is_limit(&said) && !said.contains("below") && !said.contains('\u{2014}'));

    let said = answered("403", "0 1030 core ", "{}", now, true).unwrap_err();
    assert!(said.contains("5,000 questions an hour with your key") && said.contains("in about a minute") && !said.contains("press the key"), "{said}");

    let said = answered("403", "0 1045 search ", "{}", now, false).unwrap_err();
    assert!(said.contains("10 searches a minute") && said.contains("in about a minute") && !said.contains("press the key"), "{said}");

    let said = answered("429", "40 5000 core 120", "{}", now, false).unwrap_err();
    assert!(said.contains("in about 2 minutes"), "{said}");

    let said = answered("403", "%header{x-ratelimit-remaining} %header{x-ratelimit-reset} %header{x-ratelimit-resource} %header{retry-after}", "{}", now, false).unwrap_err();
    assert!(is_limit(&said) && said.contains("Try again later."), "{said}");

    let blocked = answered("403", "41 2380 core ", r#"{"message":"Repository access blocked"}"#, now, false).unwrap_err();
    assert_eq!(blocked, "GitHub refused this: Repository access blocked.");
    assert!(!is_limit(&blocked) && !blocked.contains("key"));
    assert!(!is_limit(&answered("403", "41 2380 core ", "{}", now, true).unwrap_err()));
    let no_leave = answered("403", "%header{x-ratelimit-remaining} ", r#"{"message":"Resource not accessible by personal access token"}"#, now, true).unwrap_err();
    assert!(!is_limit(&no_leave) && no_leave.contains("Resource not accessible"), "{no_leave}");

    assert!(is_limit(&answered("403", "41 2380 core ", r#"{"message":"You have exceeded a secondary rate limit."}"#, now, false).unwrap_err()));
    assert!(is_limit(&answered("403", "41 2380 core 60", "{}", now, false).unwrap_err()));

    assert!(answered("401", "", r#"{"message":"Bad credentials"}"#, now, true).unwrap_err().starts_with("GitHub did not accept the key."));
    assert_eq!(answered("401", "", r#"{"message":"Bad credentials"}"#, now, false).unwrap_err(), "Bad credentials");

    assert_eq!(failure(&said), Failure::Limit);
    assert_eq!((failure(no_files()), failure(no_releases())), (Failure::Nothing, Failure::Nothing));
    assert_eq!(failure(&answered("404", "59 2380 core ", "{}", now, false).unwrap_err()), Failure::Nothing);
    assert_eq!((failure("GitHub could not be reached."), failure(&blocked)), (Failure::Unread, Failure::Unread));
    assert_eq!(unread_words(1, "GitHub could not be reached."), "1 project found could not be looked at, so it is not listed: GitHub could not be reached.");
    assert_eq!(unread_words(3, ""), "3 projects found could not be looked at, so they are not listed.");
    assert!(answered("200", "59 2380 core ", "[]", now, false).is_ok());
    assert!(!is_limit(&answered("404", "59 2380 core ", "{}", now, false).unwrap_err()));
    assert_eq!(answered("500", "", r#"{"message":"Made up"}"#, now, false).unwrap_err(), "Made up");
}

#[test]
fn a_key_kept_after_the_deadline_is_heard_of_and_told() {
    crate::store::key::tests::with_keyring("github-late", true, |address, _| {
        key::pretend(Some(address));
        let from = Asking::here();
        let waited = key::within_late(move || {
            std::thread::sleep(std::time::Duration::from_millis(2700));
            from.arrive();
            set_stored(&from.ring, "example-key")
        });
        assert_eq!(waited.now, Err(key::silent_words().to_string()), "not answered by its deadline");
        let coming = waited.late.expect("the way to hear how it ended");
        assert_eq!(coming.wait(), Ok(()), "it was kept after all, and that is heard");
        assert_eq!(key_within(Asking::here()).1, KeyIs::Stored);
    });

    for removing in [false, true] {
        let meanwhile = waiting_words(removing);
        assert!(meanwhile.contains("has not answered yet") && meanwhile.contains("this line will say so") && !meanwhile.contains("Could not"), "{meanwhile}");
        let (good, ok) = late_words(removing, &Ok(()));
        assert!(ok && good.starts_with("The keyring answered late: the key is") && good.ends_with(if removing { "removed." } else { "kept." }), "{good}");
        let (bad, ok) = late_words(removing, &Err(key::locked_words().into()));
        assert!(!ok && bad.contains("answered late and did not") && bad.ends_with(key::locked_words()), "{bad}");
    }
}

#[test]
fn a_key_kept_that_cannot_be_sent_is_said_and_never_passed_over() {
    crate::store::key::tests::with_keyring("github-unsendable", true, |address, _| {
        key::pretend(Some(address.clone()));
        let c = zbus::blocking::connection::Builder::address(address.as_str()).unwrap().build().unwrap();
        for kept in ["example\nkey", "example key", "exa\tmple-key"] {
            key::keep(&c, kept).unwrap();
            let (key, is) = key_within(Asking::here());
            assert!(key.is_none(), "{kept:?} is handed on to be dropped later");
            assert_ne!(is, KeyIs::Stored, "{kept:?}");
            let said = is.trouble().unwrap_or_else(|| panic!("{kept:?}: nothing is said of it"));
            assert!(said.contains("cannot be sent") && said.contains("Remove it"), "{said}");
            assert!(!said.contains(kept), "the key itself is never shown");
        }

        key::keep(&c, " example-key \n").unwrap();
        let (key, is) = key_within(Asking::here());
        assert_eq!((key.as_deref().map(String::as_str), is), (Some("example-key"), KeyIs::Stored));
    });
}

#[test]
fn the_key_is_in_the_keyring_and_reaches_curl_on_its_input_alone() {
    let sent = headers(Some(" example-key \n"));
    assert_eq!(sent.as_str(), "Accept: application/vnd.github+json\nX-GitHub-Api-Version: 2022-11-28\nAuthorization: Bearer example-key\n");
    assert!(!headers(None).contains("Authorization") && !headers(Some("  ")).contains("Authorization"));
    assert!(!headers(Some("a\nX-Other: b")).contains("X-Other"), "a key with a line break in it is sent nowhere");
    let argv = question("https://api.github.com/example");
    let at = argv.iter().position(|a| *a == "-H").expect("headers are given");
    assert_eq!(argv[at + 1], "@-", "read from curl's input");
    assert_eq!(argv.iter().filter(|a| **a == "-H").count(), 1);
    assert!(!argv.iter().any(|a| a.contains("example-key") || a.contains("Bearer") || a.contains("headers")), "{argv:?}");
    assert!(argv.windows(2).any(|w| w == ["--proto", "=https"]) && argv.windows(2).any(|w| w == ["--proto-redir", "=https"]));

    let (none, is) = key_within(Asking::here());
    assert!(none.is_none() && matches!(is, KeyIs::Hidden(_)), "{is:?}");
    assert!(set_within(Asking::here(), Zeroizing::new("example-key".into())).is_err());

    crate::store::key::tests::with_keyring("github", true, |address, dir| {
        key::pretend(Some(address));
        let set = |new: &str| set_within(Asking::here(), Zeroizing::new(new.to_string()));
        assert_eq!(key_within(Asking::here()).1, KeyIs::None);
        assert_eq!(set(" example-key \n"), Ok(()));
        let (key, is) = key_within(Asking::here());
        assert_eq!(is, KeyIs::Stored);
        assert_eq!(key.as_deref().map(String::as_str), Some("example-key"));
        let mut files = 0;
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            files += 1;
            let held = std::fs::read(entry.path()).unwrap_or_default();
            assert!(!held.windows(11).any(|w| w == b"example-key"), "{:?} holds the key in plain words", entry.file_name());
        }
        assert!(files > 0, "the keyring keeps a file of its own");
        assert_eq!(set(""), Ok(()));
        let (key, is) = key_within(Asking::here());
        assert!(key.is_none());
        assert_eq!(is, KeyIs::None);
        key::pretend(None);
    });
}

fn old_file(file: Option<PathBuf>) {
    OLD_FILE.with(|f| *f.borrow_mut() = file);
}

fn lock(address: &str) {
    let c = zbus::blocking::connection::Builder::address(address).unwrap().build().unwrap();
    c.call_method(Some("org.freedesktop.secrets"), "/org/freedesktop/secrets", Some("org.freedesktop.Secret.Service"), "Lock",
                  &(vec![zbus::zvariant::ObjectPath::try_from("/org/freedesktop/secrets/collection/login").unwrap()],)).unwrap();
}

fn no_keyring(address: &str, said: std::sync::Arc<std::sync::Mutex<&'static str>>)
    -> (std::sync::Arc<std::sync::atomic::AtomicBool>, std::thread::JoinHandle<()>) {
    use crate::toolkit::bus::testing::until;
    use crate::toolkit::bus::{Asked, Bus, Offer, Props};
    use crate::toolkit::event::Loop;
    use std::sync::atomic::{AtomicBool, Ordering};
    let stop = std::sync::Arc::new(AtomicBool::new(false));
    let (ready, is_ready) = std::sync::mpsc::channel::<bool>();
    let (address, stopped) = (address.to_string(), stop.clone());
    let thread = std::thread::spawn(move || {
        let lp = Loop::new();
        let bus = Bus::at(&lp, &address).expect("the stand-in's bus");
        let owned = std::rc::Rc::new(std::cell::Cell::new(false));
        let o = owned.clone();
        bus.own("org.freedesktop.secrets", false, &lp, move |ok| o.set(ok));
        let o = owned.clone();
        let named = until(&lp, move || o.get());
        let _served = bus.serve("/org/freedesktop/secrets", Offer {
            name: "org.freedesktop.Secret.Service".into(),
            xml: "<interface name=\"org.freedesktop.Secret.Service\"/>".into(),
            call: Box::new(move |a: Asked| a.fail(*said.lock().unwrap(), "made up for the test")),
            props: Box::new(Props::new),
            set: None,
        });
        let _ = ready.send(named);
        while !stopped.load(Ordering::Relaxed) {
            let s = stopped.clone();
            until(&lp, move || s.load(Ordering::Relaxed));
        }
    });
    assert_eq!(is_ready.recv_timeout(std::time::Duration::from_secs(10)), Ok(true), "the stand-in has the keyring's name");
    (stop, thread)
}

#[test]
fn a_keyring_that_cannot_give_the_key_is_said_as_what_it_is() {
    a_locked_keyring_is_said_as_locked();

    crate::store::key::tests::with_keyring("said-empty", false, |address, _| {
        key::pretend(Some(address));
        assert_eq!(key_within(Asking::here()).1, KeyIs::None);
        key::pretend(None);
    });
    no_keyring_at_all_is_said_as_that();

    let all = [KeyIs::Looking, KeyIs::Stored, KeyIs::None, KeyIs::NoKeyring, KeyIs::Plain(key::locked_words().into()),
               KeyIs::Hidden(key::locked_words().into()), KeyIs::Hidden(key::silent_words().into())];
    let mut said: Vec<String> = all.iter().map(KeyIs::words).collect();
    assert_eq!(all.iter().filter(|is| is.words() == "No key is stored").count(), 1);
    for words in &said {
        assert!(!words.contains('\u{2014}') && !words.is_empty(), "{words}");
    }
    said.sort();
    said.dedup();
    assert_eq!(said.len(), all.len());
}

fn a_locked_keyring_is_said_as_locked() {
    crate::store::key::tests::with_keyring("said", true, |address, dir| {
        key::pretend(Some(address.clone()));
        assert_eq!(set_within(Asking::here(), Zeroizing::new("example-key".into())), Ok(()));
        lock(&address);

        let (key, is) = key_within(Asking::here());
        assert!(key.is_none());
        assert_eq!(is, KeyIs::Hidden(key::locked_words().into()));
        assert!(!is.words().contains("No key is stored"), "{}", is.words());
        assert_eq!(is.trouble().as_deref(), Some("The keyring is locked. Log out and in again, or unlock it, then try again. \
                                                  So GitHub is asked without a key, whether or not one is stored."));
        assert_eq!(is.words(), is.trouble().unwrap(), "the key's box says the same");

        let old = dir.join("github-key");
        std::fs::write(&old, "example-key-old\n").unwrap();
        old_file(Some(old.clone()));
        let (key, is) = key_within(Asking::here());
        assert_eq!(key.as_deref().map(String::as_str), Some("example-key-old"));
        assert_eq!(is, KeyIs::Plain(key::locked_words().into()));
        assert!(old.exists());
        assert_eq!(is.trouble().as_deref(), Some("Your GitHub key is still in a plain file on this computer, and is used from there: \
                                                  it could not be moved into the keyring. The keyring is locked. Log out and in again, or unlock it, then try again."));
        old_file(None);
        key::pretend(None);
    });
}

fn no_keyring_at_all_is_said_as_that() {

    let d = crate::toolkit::bus::testing::own_daemon();
    assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()), "never the real bus");
    key::pretend(Some(d.1.clone()));
    let said = std::sync::Arc::new(std::sync::Mutex::new("org.freedesktop.DBus.Error.ServiceUnknown"));
    let absent = no_keyring(&d.1, said.clone());
    for name in ["org.freedesktop.DBus.Error.NameHasNoOwner", "org.freedesktop.DBus.Error.Spawn.ExecFailed", "org.freedesktop.DBus.Error.ServiceUnknown"] {
        *said.lock().unwrap() = name;
        assert_eq!(key_within(Asking::here()).1, KeyIs::NoKeyring, "{name}");
    }
    let (key, is) = key_within(Asking::here());
    assert!(key.is_none());
    assert_eq!(is, KeyIs::NoKeyring);
    assert_eq!(is.words(), "There is no keyring on this computer, so no key can be kept");
    assert_eq!(is.trouble(), None, "nothing could be stored, so nothing is missing from a search");
    assert_eq!(set_within(Asking::here(), Zeroizing::new("example-key".into())), Err(key::missing_words().to_string()));

    let dir = std::env::temp_dir().join(format!("trindle-github-plain-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let old = dir.join("github-key");
    std::fs::write(&old, "example-key-old\n").unwrap();
    old_file(Some(old.clone()));
    let (key, is) = key_within(Asking::here());
    assert_eq!(key.as_deref().map(String::as_str), Some("example-key-old"));
    assert_eq!(is, KeyIs::Plain(key::missing_words().into()));
    assert!(is.trouble().unwrap().starts_with("Your GitHub key is still in a plain file on this computer"));

    assert_eq!(set_within(Asking::here(), Zeroizing::new(String::new())), Ok(()));
    assert!(!old.exists());
    assert_eq!(key_within(Asking::here()).1, KeyIs::NoKeyring);
    old_file(None);
    key::pretend(None);
    absent.0.store(true, std::sync::atomic::Ordering::Relaxed);
    absent.1.join().expect("the stand-in ended");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_keyring_that_has_hung_holds_nobody_and_is_said() {
    use crate::toolkit::bus::testing::{own_daemon, until};
    use crate::toolkit::bus::Bus;
    use crate::toolkit::event::Loop;
    use std::time::{Duration, Instant};
    let d = own_daemon();
    assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()), "never the real bus");

    let lp = Loop::new();
    let bus = Bus::at(&lp, &d.1).unwrap();
    let owned = std::rc::Rc::new(std::cell::Cell::new(false));
    let o = owned.clone();
    bus.own("org.freedesktop.secrets", false, &lp, move |ok| o.set(ok));
    let o = owned.clone();
    assert!(until(&lp, move || o.get()), "the stand-in has the keyring's name");
    key::pretend(Some(d.1.clone()));

    let began = Instant::now();
    key_is(|_| {});
    set_key("example-key", |_, _| {}, |_, _| {});
    set_key("", |_, _| {}, |_, _| {});
    assert!(began.elapsed() < Duration::from_millis(500), "the thread that asked was held for {:?}", began.elapsed());

    let began = Instant::now();
    let (key, is) = key_within(Asking::here());
    let took = began.elapsed();
    assert!(key.is_none());
    assert_eq!(is, KeyIs::Hidden(key::silent_words().into()));
    assert!(took >= Duration::from_secs(1) && took < Duration::from_secs(4), "{took:?}");
    assert_eq!(is.trouble().as_deref(), Some("The keyring did not answer in time. So GitHub is asked without a key, whether or not one is stored."));
    assert_eq!(set_within(Asking::here(), Zeroizing::new("example-key".into())), Err(key::silent_words().to_string()));

    let began = Instant::now();
    assert_eq!(key::within(|| -> Result<(), String> { loop { std::thread::park() } }), Err(key::silent_words().to_string()));
    assert!(began.elapsed() < Duration::from_secs(4));
    key::pretend(None);
}

#[test]
fn a_key_that_is_not_one_line_is_refused_in_words() {
    crate::store::key::tests::with_keyring("one-line", true, |address, _| {
        key::pretend(Some(address));
        for bad in ["example\nkey", "example\r\nkey", "example key", "example\tkey", "example\u{0}key"] {
            let refused = set_within(Asking::here(), Zeroizing::new(bad.into())).unwrap_err();
            assert!(refused.starts_with("A key is one line with no spaces in it"), "{bad:?}: {refused}");
            assert_eq!(key_within(Asking::here()).1, KeyIs::None, "{bad:?}: nothing was kept");
        }

        assert_eq!(set_within(Asking::here(), Zeroizing::new(" example-key \n".into())), Ok(()));
        assert_eq!(key_within(Asking::here()).0.as_deref().map(String::as_str), Some("example-key"));
        key::pretend(None);
    });
    assert!(checked("example\nkey").is_err() && checked(" example-key\n") == Ok("example-key") && checked("") == Ok(""));
}

fn stand_in_curl(name: &str) -> (PathBuf, Vec<String>) {
    let dir = std::env::temp_dir().join(format!("trindle-github-curl-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let script = dir.join("curl.sh");
    std::fs::write(&script, "d=$(dirname \"$0\")\nprintf '%s\\n' \"$@\" > \"$d/words\"\ncat > \"$d/input\"\ncat \"$d/answer\"\n").unwrap();
    (dir, vec!["sh".to_string(), script.display().to_string()])
}

#[test]
fn a_question_carries_the_key_on_curls_input_and_nowhere_else() {
    let (dir, curl) = stand_in_curl("input");
    let curl: Vec<&str> = curl.iter().map(String::as_str).collect();
    let url = "https://api.github.com/example";
    std::fs::write(dir.join("answer"), "{\"ok\": true}\n200 59 2380 core ").unwrap();
    assert_eq!(ask(&curl, url, Some("example-key")), Ok(serde_json::json!({ "ok": true })));
    let read = |file: &str| std::fs::read_to_string(dir.join(file)).unwrap();
    assert_eq!(read("input"), "Accept: application/vnd.github+json\nX-GitHub-Api-Version: 2022-11-28\nAuthorization: Bearer example-key\n");
    let words = read("words");
    assert!(!words.contains("example-key") && !words.contains("Bearer"), "{words}");
    assert_eq!(words.lines().last(), Some(url));
    assert!(words.lines().collect::<Vec<_>>().windows(2).any(|w| w == ["-H", "@-"]), "its headers come on its input: {words}");
    for entry in std::fs::read_dir(&dir).unwrap().flatten() {
        let held = std::fs::read_to_string(entry.path()).unwrap_or_default();
        assert_eq!(held.contains("example-key"), entry.file_name() == "input", "{:?}", entry.file_name());
    }

    assert_eq!(ask(&curl, url, None), Ok(serde_json::json!({ "ok": true })));
    assert_eq!(read("input"), "Accept: application/vnd.github+json\nX-GitHub-Api-Version: 2022-11-28\n");

    std::fs::write(dir.join("answer"), "{}\n403 0 2380 core ").unwrap();
    let with = ask(&curl, url, Some("example-key")).unwrap_err();
    assert!(is_limit(&with) && with.contains("with your key"), "{with}");
    let without = ask(&curl, url, None).unwrap_err();
    assert!(is_limit(&without) && without.contains("without a key") && !without.contains("with your key"), "{without}");

    let broken = ask(&curl, url, Some("example\nkey")).unwrap_err();
    assert!(!read("input").contains("Authorization") && !read("input").contains("example"));
    assert_eq!(broken, without);

    std::fs::remove_file(dir.join("answer")).unwrap();
    assert!(ask(&curl, url, None).unwrap_err().contains("answer"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn reads_a_projects_name_and_dates() {
    assert_eq!(repo_of_url("https://github.com/john-smith/thing.git"), Some("john-smith/thing".into()));
    assert_eq!(repo_of_url("https://github.com/john-smith/thing/issues"), Some("john-smith/thing".into()));
    for not in ["https://gitlab.com/a/b", "https://github.com/a", "https://github.com/../b", "https://github.com/a/b c"] {
        assert_eq!(repo_of_url(not), None, "{not}");
    }
    assert_eq!(when(&Value::from("1970-01-02T00:00:00Z")), Some(86_400));
    assert_eq!((when(&Value::from("not a date")), when(&Value::Null)), (None, None));
    let found = repos(&serde_json::json!({"items":[{"full_name":"a/b","name":"b","created_at":"1970-01-02T00:00:00Z"}]}));
    assert_eq!((found[0].created, found[0].pushed), (Some(86_400), None));
}

#[test]
fn a_file_nobody_checks_is_said_so_wherever_one_is_offered() {

    assert!(super::says_unchecked(1) && super::says_unchecked(2));
    assert!(!super::says_unchecked(0));
    let said = super::unchecked();
    assert!(said.contains("nobody has checked") && said.contains("Downloaded securely"));
    assert_eq!(said.matches(". ").count(), 0, "one sentence");
    assert!(said.ends_with('.') && !said.contains("  "));

    for name in ["GitHub", "github"] {
        assert!(!said.contains(name), "{name}");
    }
}
