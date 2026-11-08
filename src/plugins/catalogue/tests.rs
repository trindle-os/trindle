//! Tests for catalogue.rs.

#[cfg(test)]
use super::publisher::*;
use super::*;
use ::ring::signature::{Ed25519KeyPair, KeyPair};
use crate::common::files::read_text;

#[test]
fn removing_a_plugin_forgets_its_fetched_icon_and_no_other() {
    let icons = temp("forget-icon");
    for name in ["org.example.One.svg", "org.example.One.png", "org.example.Two.svg"] {
        std::fs::write(icons.join(name), "x").unwrap();
    }
    forget_icon_at(&icons, "org.example.One");
    forget_icon_at(&icons, "../org.example.Two");
    let mut left: Vec<String> = std::fs::read_dir(&icons).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    left.sort();
    assert_eq!(left, ["org.example.Two.svg"]);
}

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-cat-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn plugin(at: &Path, id: &str, version: &str) -> PathBuf {
    let dir = at.join(id);
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    std::fs::write(dir.join("plugin.ini"), format!(
        "[Plugin]\nInterface=1\nId={id}\nName=Weather\nVersion={version}\nSummary=The weather, in the ring.\n\
         Permissions=network;notifications\n\n[Ring]\nIcon=icon.svg\nLabel=Weather\nRun=bin/go\n")).unwrap();
    std::fs::write(dir.join("icon.svg"), "<svg xmlns='http://www.w3.org/2000/svg'/>").unwrap();
    std::fs::write(dir.join("bin/go"), "#!/bin/sh\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(dir.join("bin/go"), std::fs::Permissions::from_mode(0o755)).unwrap();
    dir
}

fn pack(dir: &Path, to: &Path, name: &str) {
    crate::common::needs::programs(&["tar", "zstd"]);
    let ok = Command::new("tar").args(["--zstd", "-cf"]).arg(to.join(name)).arg("-C").arg(dir.parent().unwrap())
        .arg(dir.file_name().unwrap()).status().unwrap().success();
    assert!(ok, "tar --zstd could not pack the test plugin");
}

fn entry() -> Entry {
    Entry {
        id: "org.example.Weather".into(), name: "Weather".into(), version: "1.0".into(),
        summary: "The weather; in the ring.".into(), permissions: vec!["network".into(), "notifications".into()],
        icon: Some("org.example.Weather.svg".into()), icon_sha256: Some("cd".repeat(32)), archive: "org.example.Weather-1.0.tar.zst".into(),
        sha256: "ab".repeat(32),
    }
}

#[test]
fn the_index_reads_back_what_was_written() {
    let text = write(&[entry()], 7);
    assert!(text.contains("[Catalogue]\nFormat=1\n") && text.contains("[Plugin org.example.Weather]\nInterface=1\n"));
    assert!(text.contains("Permissions=network;notifications;\n"));
    let (back, problems) = parse(&text).unwrap();
    assert_eq!(back, vec![entry()]);
    assert!(problems.is_empty());
}

#[test]
fn a_picture_shows_only_if_it_matches_its_signed_hash() {
    let top = temp("icon-hash");
    let at = top.join("a.svg");
    std::fs::write(&at, "<svg/>").unwrap();
    let good = sha256_of(&at).unwrap();
    let with = |h: &str| Entry { icon_sha256: Some(h.into()), ..entry() };
    assert_eq!(icon_checked(&with(&good), &at), Some(at.clone()));

    for bad in [Some("ab".repeat(32)), None] {
        std::fs::write(&at, "<svg/>").unwrap();
        let e = Entry { icon_sha256: bad, ..entry() };
        assert_eq!(icon_checked(&e, &at), None);
        assert!(!at.exists());
    }

    let text = write(&[entry()], 7);
    let line = format!("IconSHA256={}", "cd".repeat(32));
    assert!(text.contains(&line));
    let without = text.replace(&format!("{line}\n"), "");
    let (back, _) = parse(&without).unwrap();
    assert_eq!((back[0].icon.clone(), back[0].icon_sha256.clone()), (None, None));
    let wrong = text.replace(&"cd".repeat(32), "zz");
    assert_eq!(parse(&wrong).unwrap().0[0].icon, None);
}

#[test]
fn the_index_refuses_what_it_should() {
    assert!(parse("[Plugin a.b]\nName=A\n").is_err(), "no [Catalogue] part");
    assert!(parse("[Catalogue]\nFormat=2\n").unwrap_err().contains("newer kind"));
    let good = write(&[entry()], 7);
    let sha = format!("SHA256={}", "ab".repeat(32));
    for (from, to, said) in [
        ("Archive=org.example.Weather-1.0.tar.zst", "Archive=../escape.tar.zst", "Archive"),
        ("Archive=org.example.Weather-1.0.tar.zst", "Archive=weather.exe", "Archive"),
        (sha.as_str(), "SHA256=1234", "SHA256"),
        ("Permissions=network;", "Permissions=camera;", "permission"),
        ("Icon=org.example.Weather.svg", "Icon=/etc/passwd", "Icon"),
        ("[Plugin org.example.Weather]", "[Plugin ../x]", "id"),
    ] {
        let (list, problems) = parse(&good.replace(from, to)).unwrap();
        assert!(list.is_empty(), "{to} was let through");
        assert!(problems[0].contains(said), "{to}: {problems:?}");
    }

    let (list, problems) = parse(&good.replace("Interface=1", "Interface=2")).unwrap();
    assert!(list.is_empty() && problems.is_empty());
}

#[test]
fn versions_compare_as_numbers() {
    assert!(newer("1.10", "1.9") && newer("2.0", "1.99.9") && newer("1.0.1", "1.0"));
    assert!(!newer("1.0", "1.0") && !newer("0.9", "1.0"));
}

#[test]
fn a_long_description_becomes_one_line() {
    assert_eq!(first_sentence("One turns speech into text. It runs on this computer.\n\nMore."), "One turns speech into text.");
    let long = first_sentence(&"word ".repeat(80));
    assert!(long.ends_with("...") && long.chars().count() <= SUMMARY_MOST);
}

fn from_shelf(shelf: &Path) -> impl Fn(&str, &Path) -> Result<(), String> + '_ {
    move |url, to| {
        assert_eq!(url.rsplit_once('/').unwrap().0, PUBLISHED.rsplit_once('/').unwrap().0, "fetched from beside Trindle's own catalogue");
        std::fs::copy(shelf.join(url.rsplit('/').next().unwrap()), to).map(|_| ()).map_err(|e| e.to_string())
    }
}

#[test]
fn a_shelf_of_archives_becomes_a_catalogue_and_installs() {
    let top = temp("build");
    let (work, shelf, root) = (top.join("work"), top.join("shelf"), top.join("plugins"));
    std::fs::create_dir_all(&shelf).unwrap();
    pack(&plugin(&work, "org.example.Weather", "1.0"), &shelf, "weather-1.0.tar.zst");
    pack(&plugin(&work.join("new"), "org.example.Weather", "1.2"), &shelf, "weather-1.2.tar.zst");

    let bad = plugin(&work, "org.example.Broken", "1.0");
    std::fs::write(bad.join("plugin.ini"), "[Plugin]\nInterface=1\nId=org.example.Broken\nName=Broken\nVersion=1\n").unwrap();
    pack(&bad, &shelf, "broken.tar.zst");

    let (entries, problems) = build(&shelf).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].version, "1.2", "the newer of two went in");
    assert_eq!(entries[0].icon.as_deref(), Some("org.example.Weather.svg"));
    assert!(shelf.join("org.example.Weather.svg").is_file(), "its ring icon was copied out beside");
    assert!(problems.len() == 1 && problems[0].contains("broken.tar.zst") && problems[0].contains("hooks into nothing"), "{problems:?}");
    assert_eq!(main(&shelf), 1, "a plugin was left out, so it says so");

    let (listed, left_out) = parse(&std::fs::read_to_string(shelf.join(INDEX)).unwrap()).unwrap();
    assert_eq!((listed.len(), left_out.len()), (1, 0));
    let offer = Offer { entry: listed[0].clone(), icon: None };
    let p = install_offer(&offer, &root, &from_shelf(&shelf)).unwrap().plugin;
    assert_eq!((p.id.as_str(), p.version.as_str()), ("org.example.Weather", "1.2"));
    assert!(root.join("org.example.Weather/app/plugin.ini").is_file());
    assert!(root.join("org.example.Weather/home").is_dir());
    super::super::install::remove_at(&root, "org.example.Weather").unwrap();
    assert!(!root.join("org.example.Weather").exists(), "removing takes all of it");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_file_that_is_not_the_one_listed_is_refused() {
    let top = temp("sha");
    let (shelf, root) = (top.join("shelf"), top.join("plugins"));
    std::fs::create_dir_all(&shelf).unwrap();
    pack(&plugin(&top.join("work"), "org.example.Weather", "1.0"), &shelf, "weather.tar.zst");
    let (entries, _) = build(&shelf).unwrap();
    let mut offer = Offer { entry: entries[0].clone(), icon: None };
    offer.entry.sha256 = "0".repeat(64);
    let e = install_offer(&offer, &root, &from_shelf(&shelf)).unwrap_err();
    assert!(e.contains("SHA-256"), "{e}");
    assert!(!root.join("org.example.Weather").exists(), "nothing was installed");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_plugin_the_validator_refuses_is_not_installed() {
    let top = temp("refused");
    let (shelf, root) = (top.join("shelf"), top.join("plugins"));
    std::fs::create_dir_all(&shelf).unwrap();
    let dir = plugin(&top.join("work"), "org.example.Weather", "1.0");
    std::fs::write(dir.join("plugin.ini"), std::fs::read_to_string(dir.join("plugin.ini")).unwrap()
        .replace("Run=bin/go", "Run=/usr/bin/sh")).unwrap();
    pack(&dir, &shelf, "weather.tar.zst");
    let sha256 = sha256_of(&shelf.join("weather.tar.zst")).unwrap();
    let offer = Offer { entry: Entry { archive: "weather.tar.zst".into(), sha256, icon: None, ..entry() }, icon: None };
    let e = install_offer(&offer, &root, &from_shelf(&shelf)).unwrap_err();
    assert!(e.contains("plugin check") && e.contains("/usr/bin/sh"), "{e}");
    assert!(!root.join("org.example.Weather").exists());
    let _ = std::fs::remove_dir_all(&top);
}

fn test_key() -> (Ed25519KeyPair, [u8; KEY_BYTES]) {
    let doc = Ed25519KeyPair::generate_pkcs8(&::ring::rand::SystemRandom::new()).unwrap();
    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let mut public = [0u8; KEY_BYTES];
    public.copy_from_slice(pair.public_key().as_ref());
    (pair, public)
}

struct Site {
    catalogue: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
    asked: std::cell::Cell<u32>,
}

impl Site {
    fn signed(text: &str, pair: &Ed25519KeyPair) -> Site {
        Site { catalogue: Some(text.as_bytes().to_vec()), signature: Some(pair.sign(text.as_bytes()).as_ref().to_vec()), asked: std::cell::Cell::new(0) }
    }

    fn down() -> Site {
        Site { catalogue: None, signature: None, asked: std::cell::Cell::new(0) }
    }

    fn fetch(&self) -> impl Fn(&str, &Path, &str) -> Result<(), String> + '_ {
        move |url, to, _| {
            self.asked.set(self.asked.get() + 1);
            let what = if url.ends_with(SIGNATURE_END) { &self.signature } else { &self.catalogue };
            let bytes = what.as_ref().ok_or("It could not be downloaded: not found")?;
            std::fs::write(to, bytes).map_err(|e| e.to_string())
        }
    }
}

fn trust_in(top: &Path, keys: &[[u8; KEY_BYTES]]) -> Trust {
    Trust { keys: keys.to_vec(), kept: top.join("cache/plugin-catalogue.ini"), note: top.join("state/plugin-catalogue-number"), floor: 0 }
}

fn read(site: &Site, fresh: Duration, trust: &Trust) -> (Vec<String>, Option<String>) {
    let (entries, said) = entries_at(fresh, &site.fetch(), trust);
    (entries.into_iter().map(|e| e.version).collect(), said)
}

fn numbered(version: &str, number: u64) -> String {
    write(&[Entry { version: version.into(), ..entry() }], number)
}

#[test]
fn a_catalogue_signed_with_a_built_in_key_is_accepted_kept_and_remembered() {
    let top = temp("signed");
    let (pair, public) = test_key();
    let trust = trust_in(&top, &[public]);
    let v1 = numbered("1.0", 5);
    assert_eq!(read(&Site::signed(&v1, &pair), Duration::ZERO, &trust), (vec!["1.0".to_string()], None));
    assert_eq!(std::fs::read(&trust.kept).unwrap(), v1.as_bytes(), "the bytes are kept as they came");
    assert_eq!(std::fs::read(trust.kept_signature()).unwrap().len(), SIGNATURE_BYTES);
    assert_eq!(std::fs::read_to_string(&trust.note).unwrap(), "5");

    assert_eq!(read(&Site::signed(&v1, &pair), Duration::ZERO, &trust).1, None);
    assert_eq!(read(&Site::signed(&numbered("1.1", 6), &pair), Duration::ZERO, &trust), (vec!["1.1".to_string()], None));
    assert_eq!(std::fs::read_to_string(&trust.note).unwrap(), "6");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_catalogue_with_no_signature_or_a_bad_one_is_refused_whole_and_the_last_good_one_kept() {
    let top = temp("refused-sig");
    let (pair, public) = test_key();
    let (other, _) = test_key();
    let trust = trust_in(&top, &[public]);
    let newer = numbered("6.6", 9);
    let good = Site::signed(&newer, &pair);
    let refusals = |trust: &Trust, kept: &[&str], tail: &str| {
        let said = |site: Site| {
            let (versions, said) = read(&site, Duration::ZERO, trust);
            assert_eq!(versions, kept, "what is refused offers nothing of its own");
            said.expect("a refusal is never silent")
        };

        assert_eq!(said(Site { signature: None, ..Site::signed(&newer, &pair) }),
                   format!("Trindle's plugin catalogue has no signature, so it was not used.{tail}"));

        assert_eq!(said(Site::signed(&newer, &other)), format!("Trindle's plugin catalogue is not signed with Trindle's key, so it was not used.{tail}"));

        let mut changed = good.catalogue.clone().unwrap();
        let at = changed.len() - 2;
        changed[at] ^= 1;
        assert_eq!(said(Site { catalogue: Some(changed), ..Site::signed(&newer, &pair) }),
                   format!("Trindle's plugin catalogue is not signed with Trindle's key, so it was not used.{tail}"));

        let whole = good.signature.clone().unwrap();
        for wrong in [whole[..63].to_vec(), [whole.clone(), vec![0]].concat(), Vec::new(), vec![0; 4096]] {
            assert_eq!(said(Site { signature: Some(wrong), ..Site::signed(&newer, &pair) }),
                       format!("The signature of Trindle's plugin catalogue is not the size a signature is, so the catalogue was not used.{tail}"));
        }

        assert!(said(Site { catalogue: Some(b"<html>Not found</html>".to_vec()), ..Site::signed(&newer, &pair) }).contains("not signed with Trindle's key"));
    };

    refusals(&trust, &[], "");
    assert!(!trust.kept.exists() && !trust.note.exists(), "nothing refused is kept or remembered");

    assert_eq!(read(&Site::signed(&numbered("1.0", 5), &pair), Duration::ZERO, &trust), (vec!["1.0".to_string()], None));
    refusals(&trust, &["1.0"], " The last good one is shown.");
    assert_eq!(std::fs::read_to_string(&trust.note).unwrap(), "5");

    let keyless = trust_in(&top.join("keyless"), &[]);
    assert_eq!(read(&good, Duration::ZERO, &keyless),
               (Vec::new(), Some("This Trindle has no key to check its plugin catalogue with, so it accepts none.".to_string())));

    assert_eq!(read(&Site::down(), Duration::ZERO, &trust), (vec!["1.0".to_string()], None));
    assert_eq!(read(&Site::down(), Duration::ZERO, &keyless), (Vec::new(), None));
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn an_older_catalogue_is_refused_however_good_its_signature() {
    let top = temp("older");
    let (pair, public) = test_key();
    let trust = trust_in(&top, &[public]);
    assert_eq!(read(&Site::signed(&numbered("2.0", 7), &pair), Duration::ZERO, &trust), (vec!["2.0".to_string()], None));

    let old = Site::signed(&numbered("1.0", 5), &pair);
    assert_eq!(read(&old, Duration::ZERO, &trust),
               (vec!["2.0".to_string()], Some("Trindle's plugin catalogue is older than the one accepted before (number 5, after 7), so it was not used. \
                                               The last good one is shown.".to_string())));
    assert_eq!(std::fs::read_to_string(&trust.note).unwrap(), "7");

    std::fs::remove_dir_all(top.join("cache")).unwrap();
    assert_eq!(read(&old, Duration::ZERO, &trust),
               (Vec::new(), Some("Trindle's plugin catalogue is older than the one accepted before (number 5, after 7), so it was not used.".to_string())));

    assert!(read(&Site::signed(&numbered("2.0", 7), &pair), Duration::ZERO, &trust).1.is_none());
    std::fs::remove_file(&trust.note).unwrap();
    assert!(read(&old, Duration::ZERO, &trust).1.is_some_and(|w| w.contains("number 5, after 7")));

    let unnumbered = write(&[entry()], 1).replace("Serial=1\n", "");
    assert_eq!(number_of(&unnumbered), None);
    assert!(read(&Site::signed(&unnumbered, &pair), Duration::ZERO, &trust).1.is_some_and(|w| w.starts_with("Trindle's plugin catalogue does not say which number it is")));
    let first = Trust { floor: 6, ..trust_in(&top.join("first"), &[public]) };
    assert!(read(&old, Duration::ZERO, &first).1.is_some_and(|w| w.contains("number 5, after 6")));
    assert_eq!(read(&Site::signed(&numbered("2.0", 7), &pair), Duration::ZERO, &first), (vec!["2.0".to_string()], None), "a first run takes any at or over it");

    for bad in ["-1", "+5", "5 6", "0x10", "", "99999999999999999999"] {
        assert_eq!(number_of(&format!("[Catalogue]\nFormat=1\nSerial={bad}\n")), None, "{bad:?}");
    }
    assert_eq!(number_of("[Catalogue]\nFormat=1\nSerial= 12 \n"), Some(12));

    assert!(next_number(None) > 1_700_000_000);
    assert_eq!(next_number(Some(u64::MAX - 1)), u64::MAX);
    assert!(next_number(Some(5)) > 5);
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn starting_again_takes_away_the_damaged_note_and_follows_no_link() {
    let top = temp("start-again");
    let home = top.join("john");
    let note = home.join(NUMBER_NOTE);
    std::fs::create_dir_all(note.parent().unwrap()).unwrap();
    std::fs::write(&note, "seven").unwrap();
    assert_eq!(start_again_in(&home), Ok(()));
    assert!(!note.exists());

    assert_eq!(start_again_in(&home), Ok(()));
    assert!(start_again_in(&top.join("nobody")).is_err());

    let elsewhere = top.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(elsewhere.join("plugin-catalogue-number"), "keep").unwrap();
    let other = top.join("mary");
    std::fs::create_dir_all(other.join(".local/state")).unwrap();
    std::os::unix::fs::symlink(&elsewhere, other.join(".local/state/trindle")).unwrap();
    assert!(start_again_in(&other).is_err());
    assert_eq!(std::fs::read_to_string(elsewhere.join("plugin-catalogue-number")).unwrap(), "keep");

    std::fs::write(top.join("target"), "1").unwrap();
    std::os::unix::fs::symlink(top.join("target"), &note).unwrap();
    assert_eq!(start_again_in(&home), Ok(()));
    assert!(std::fs::symlink_metadata(&note).is_err() && top.join("target").is_file());

    let (_, public) = test_key();
    let trust = Trust { floor: NUMBER_FLOOR, note: note.clone(), ..trust_in(&top, &[public]) };
    assert_eq!(trust.least(None), Ok(NUMBER_FLOOR));
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_remembered_number_that_cannot_be_read_fails_shut() {
    let top = temp("unreadable");
    let (pair, public) = test_key();
    let trust = trust_in(&top, &[public]);
    let site = Site::signed(&numbered("2.0", 7), &pair);
    std::fs::create_dir_all(trust.note.parent().unwrap()).unwrap();
    const WORDS: &str = "Plugin updates are paused: the plugin list is damaged.";
    assert_eq!(WORDS, paused());
    assert!(is_paused(WORDS) && !is_paused("Another sentence."));

    for bad in ["seven", "", "7 8", "-7", &"9".repeat(40)] {
        std::fs::write(&trust.note, bad).unwrap();
        assert_eq!(read(&site, Duration::ZERO, &trust), (Vec::new(), Some(WORDS.to_string())), "{bad:?}");
        assert!(!trust.kept.exists());
    }
    std::fs::remove_file(&trust.note).unwrap();
    std::fs::create_dir(&trust.note).unwrap();
    assert_eq!(read(&site, Duration::ZERO, &trust).1.as_deref(), Some(WORDS));
    std::fs::remove_dir(&trust.note).unwrap();
    std::fs::write(top.join("elsewhere"), "1").unwrap();
    std::os::unix::fs::symlink(top.join("elsewhere"), &trust.note).unwrap();
    assert_eq!(read(&site, Duration::ZERO, &trust).1.as_deref(), Some(WORDS));
    std::fs::remove_file(&trust.note).unwrap();

    assert_eq!(read(&site, Duration::ZERO, &trust), (vec!["2.0".to_string()], None));
    std::fs::write(&trust.note, "seven").unwrap();
    assert!(read(&Site::signed(&numbered("1.0", 5), &pair), Duration::ZERO, &trust).1.is_some_and(|w| w.contains("number 5, after 7")));
    assert_eq!(read(&Site::signed(&numbered("3.0", 8), &pair), Duration::ZERO, &trust), (vec!["3.0".to_string()], None));
    assert_eq!(std::fs::read_to_string(&trust.note).unwrap(), "8");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn an_old_key_and_a_new_one_are_both_accepted_while_both_are_built_in() {
    let top = temp("two-keys");
    let ((old, old_public), (new, new_public)) = (test_key(), test_key());
    let both = trust_in(&top, &[old_public, new_public]);
    assert_eq!(read(&Site::signed(&numbered("1.0", 5), &old), Duration::ZERO, &both), (vec!["1.0".to_string()], None));
    assert_eq!(read(&Site::signed(&numbered("1.1", 6), &new), Duration::ZERO, &both), (vec!["1.1".to_string()], None));

    let after = trust_in(&top.join("after"), &[new_public]);
    assert!(read(&Site::signed(&numbered("1.2", 7), &old), Duration::ZERO, &after).1.is_some_and(|w| w.contains("not signed with Trindle's key")));
    assert_eq!(read(&Site::signed(&numbered("1.2", 7), &new), Duration::ZERO, &after), (vec!["1.2".to_string()], None));
    let only_new = trust_in(&top, &[new_public]);
    assert!(only_new.kept_good().is_some(), "kept, signed with the new key");
    assert!(trust_in(&top, &[old_public]).kept_good().is_none(), "a kept copy is checked like one just fetched");

    assert_eq!(key_of(&hex_of(&new_public)), Some(new_public));
    for bad in ["", "abc", &"g".repeat(64), &"a".repeat(63), &"a".repeat(66), &"é".repeat(32)] {
        assert_eq!(key_of(bad), None, "{bad:?}");
    }
    let _ = std::fs::remove_dir_all(&top);
}

#[cfg(not(feature = "test-catalogue"))]
#[test]
fn the_catalogue_is_read_from_the_latest_github_release() {
    assert_eq!(PUBLISHED, "https://github.com/Trindle-OS/trindle-releases/releases/latest/download/catalogue.ini");
    assert_eq!(format!("{PUBLISHED}{SIGNATURE_END}"), "https://github.com/Trindle-OS/trindle-releases/releases/latest/download/catalogue.ini.sig");
    assert_eq!(beside(PUBLISHED, "org.example.Weather-1.2.tar.zst"),
               "https://github.com/Trindle-OS/trindle-releases/releases/latest/download/org.example.Weather-1.2.tar.zst");
}

#[test]
#[allow(clippy::const_is_empty)]
fn a_real_key_is_built_in_before_a_release() {
    assert!(!KEYS.is_empty(), "KEYS in src/plugins/catalogue.rs is empty: put the public key trindle-plugin new-key prints there before a release");
    assert!(KEYS.len() <= 3, "a short list: the key in use, and the one before or after it");
    assert_eq!(built_in_keys().len(), KEYS.len(), "each key is 64 hex digits");
}

#[test]
fn the_signature_is_checked_before_anything_is_read_as_a_catalogue() {
    let (pair, public) = test_key();

    assert_eq!(accepted(b"\xff\xfe not text", &[0; SIGNATURE_BYTES], &[public], 0), Err(Refused::NotSigned));

    let junk = b"[Nothing]\n";
    assert!(matches!(accepted(junk, pair.sign(junk).as_ref(), &[public], 0), Err(Refused::NotACatalogue(_))));
    let not_text = b"\xff\xfe";
    assert_eq!(accepted(not_text, pair.sign(not_text).as_ref(), &[public], 0), Err(Refused::NotACatalogue("It is not text.".into())));

    let source = crate::common::source::code(include_str!("../catalogue.rs")).split_once("#[cfg(test)]").unwrap().0;
    let judge = source.split_once("fn accepted(").unwrap().1.split_once("\n}\n").unwrap().0;
    let at = |what: &str| judge.find(what).unwrap_or_else(|| panic!("{what} is not in accepted"));
    assert!(at("signed_by(") < at("from_utf8(") && at("from_utf8(") < at("parse(") && at("parse(") < at("number_of("));
    let read = source.split_once("fn index_text(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(!read.contains("parse(") && !read.contains("KeyFile"), "what is fetched is read as a catalogue only by accepted");
    assert!(read.find("INDEX_MOST").unwrap() < read.find("SIGNATURE_MOST").unwrap() && read.find("SIGNATURE_MOST").unwrap() < read.find("accepted(").unwrap());

    assert_eq!(source.matches(".verify(").count(), 1);
    assert!(source.contains("UnparsedPublicKey::new(&ED25519, key).verify(bytes, signature)"));
}

#[test]
fn the_signing_commands_make_a_key_and_a_signature_that_checks() {

    let top = PathBuf::from("/tmp").join(format!("trindle-cat-signing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&top);
    std::fs::create_dir_all(&top).unwrap();
    let key = top.join("private.key");
    assert_eq!(new_key_main(&key), 0);
    use std::os::unix::fs::PermissionsExt;
    assert_eq!(std::fs::metadata(&key).unwrap().permissions().mode() & 0o777, 0o600, "the private key is for this account alone");
    assert_eq!(new_key_main(&key), 1, "a key that is there is never written over");

    std::fs::create_dir_all(top.join("repo/.git")).unwrap();
    std::fs::create_dir_all(top.join("repo/keys")).unwrap();
    assert_eq!(new_key_main(&top.join("repo/keys/private.key")), 1);
    assert!(!top.join("repo/keys/private.key").exists());

    let pair = Ed25519KeyPair::from_pkcs8(&std::fs::read(&key).unwrap()).unwrap();
    let mut public = [0u8; KEY_BYTES];
    public.copy_from_slice(pair.public_key().as_ref());
    let catalogue = top.join(INDEX);
    let text = numbered("1.0", 5);
    std::fs::write(&catalogue, &text).unwrap();
    assert_eq!(sign_main(&catalogue, &key), 0);
    let signature = std::fs::read(top.join(format!("{INDEX}{SIGNATURE_END}"))).unwrap();
    assert_eq!(signature.len(), SIGNATURE_BYTES);
    assert_eq!(accepted(text.as_bytes(), &signature, &[public], 0), Ok((text.clone(), 5)));

    std::fs::write(top.join("not.key"), "nothing").unwrap();
    assert_eq!(sign_main(&catalogue, &top.join("not.key")), 1);
    assert_eq!(sign_main(&catalogue, &top.join("missing.key")), 1);
    let plain = top.join("plain.ini");
    std::fs::write(&plain, text.replace("Serial=5\n", "")).unwrap();
    assert_eq!(sign_main(&plain, &key), 1);
    assert!(!top.join("plain.ini.sig").exists());
    std::fs::write(&plain, "not a catalogue").unwrap();
    assert_eq!(sign_main(&plain, &key), 1);

    let shelf = top.join("shelf");
    std::fs::create_dir_all(&shelf).unwrap();
    pack(&plugin(&top.join("work"), "org.example.Weather", "1.0"), &shelf, "weather.tar.zst");
    assert_eq!(main(&shelf), 0);
    let first = number_of(&read_text(shelf.join(INDEX))).expect("made with a number");
    assert_eq!(main(&shelf), 0);
    assert!(number_of(&read_text(shelf.join(INDEX))).unwrap() > first, "and each one made after is higher");
    assert_eq!(sign_main(&shelf.join(INDEX), &key), 0);

    let source = crate::common::source::code(include_str!("../catalogue.rs")).split_once("#[cfg(test)]").unwrap().0;
    assert!(!source.contains("PRIVATE") && !source.contains("from_seed"));
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn the_kept_catalogue_answers_while_it_is_fresh_and_when_the_address_cannot_be_reached() {
    let top = temp("kept");
    let (pair, public) = test_key();
    let trust = trust_in(&top, &[public]);
    let hour = Duration::from_secs(3600);
    let (v1, v2) = (Site::signed(&numbered("1.0", 5), &pair), Site::signed(&numbered("1.1", 6), &pair));

    assert_eq!((read(&v1, hour, &trust).0, v1.asked.get()), (vec!["1.0".to_string()], 2));

    assert_eq!((read(&v2, hour, &trust).0, v2.asked.get()), (vec!["1.0".to_string()], 0));

    assert_eq!((read(&v2, Duration::ZERO, &trust).0, v2.asked.get()), (vec!["1.1".to_string()], 2));

    assert_eq!(read(&Site::down(), Duration::ZERO, &trust), (vec!["1.1".to_string()], None));

    std::fs::write(&trust.kept, numbered("9.9", 99)).unwrap();
    assert!(trust.kept_good().is_none());
    assert_eq!((read(&v2, hour, &trust).0, v2.asked.get()), (vec!["1.1".to_string()], 4));

    let secret = top.join("secret");
    std::fs::write(&secret, "mine").unwrap();
    std::fs::remove_file(&trust.kept).unwrap();
    std::os::unix::fs::symlink(&secret, &trust.kept).unwrap();
    assert!(small_file(&trust.kept, KEPT_MOST).is_none() && trust.kept_good().is_none());
    assert_eq!(read(&v2, hour, &trust).0, ["1.1"]);
    assert!(std::fs::symlink_metadata(&trust.kept).unwrap().is_file());
    assert_eq!(std::fs::read_to_string(&secret).unwrap(), "mine");
    std::fs::write(top.join("big"), "x".repeat(KEPT_MOST as usize + 1)).unwrap();
    assert!(small_file(&top.join("big"), KEPT_MOST).is_none());
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn counting_reads_the_index_and_no_pictures() {
    let top = temp("count");
    let (pair, public) = test_key();
    let site = Site::signed(&numbered("1.1", 5), &pair);
    let offers = offers_to_count_from(INDEX_FRESH, &site.fetch(), &trust_in(&top, &[public]));
    let found: Vec<(String, String, bool)> = offers.iter().map(|o| (o.entry.id.clone(), o.entry.version.clone(), o.icon.is_some())).collect();
    assert_eq!(found, [("org.example.Weather".to_string(), "1.1".to_string(), false)]);
    assert_eq!(site.asked.get(), 2, "the index and its signature, once each, and nothing else of the address");

    let source = crate::common::source::code(include_str!("../catalogue.rs")).split_once("#[cfg(test)]").unwrap().0;
    let counting = source.split_once("fn offers_to_count_from(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(!counting.contains("download(") && !counting.contains("read_address("));
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn an_update_that_asks_for_more_is_not_put_in_unseen_whatever_the_catalogue_says() {
    let top = temp("unseen");
    let (work, shelf, root) = (top.join("work"), top.join("shelf"), top.join("plugins"));
    std::fs::create_dir_all(&shelf).unwrap();
    let offer_of = |dir: &Path, name: &str| {
        pack(dir, &shelf, name);
        let p = check(dir).unwrap();
        Offer { entry: Entry { version: p.version, permissions: p.asks, archive: name.into(), sha256: sha256_of(&shelf.join(name)).unwrap(), icon: None, ..entry() },
                icon: None }
    };

    let from_shelf = |url: &str, to: &Path| std::fs::copy(shelf.join(url.rsplit('/').next().unwrap()), to).map(|_| ()).map_err(|e| e.to_string());
    let fetch_update_unseen = |offer: &Offer, root: &Path| update_unseen(offer, root, &from_shelf);
    let fetch_install = |offer: &Offer, root: &Path| install_offer(offer, root, &from_shelf);
    let edit = |dir: &Path, from: &str, to: &str| {
        let ini = std::fs::read_to_string(dir.join("plugin.ini")).unwrap();
        assert!(ini.contains(from));
        std::fs::write(dir.join("plugin.ini"), ini.replace(from, to)).unwrap();
    };

    let v1 = offer_of(&plugin(&work.join("1"), "org.example.Weather", "1.0"), "w-1.0.tar.zst");
    assert!(fetch_update_unseen(&v1, &root).unwrap_err().contains("not installed"));
    assert!(!root.join("org.example.Weather").exists());
    let mine = fetch_install(&v1, &root).unwrap().plugin;
    std::fs::write(root.join("org.example.Weather/allowed"), "network\n").unwrap();

    let v2 = offer_of(&plugin(&work.join("2"), "org.example.Weather", "1.1"), "w-1.1.tar.zst");
    assert!(said_to_ask_more(&mine, &v2).is_empty());
    match fetch_update_unseen(&v2, &root).unwrap() {
        Unseen::In(done) => assert_eq!(done.plugin.version, "1.1"),
        other => panic!("{other:?}"),
    }
    assert_eq!(std::fs::read_to_string(root.join("org.example.Weather/allowed")).unwrap(), "network\n");

    let dir = plugin(&work.join("3"), "org.example.Weather", "1.2");
    edit(&dir, "network;notifications", "network;notifications;microphone");
    let v3 = offer_of(&dir, "w-1.2.tar.zst");
    assert_eq!(said_to_ask_more(&mine, &v3), ["your microphone"]);

    let lying = Offer { entry: Entry { permissions: mine.asks.clone(), ..v3.entry.clone() }, ..v3.clone() };
    assert!(said_to_ask_more(&mine, &lying).is_empty());
    match fetch_update_unseen(&lying, &root).unwrap() {
        Unseen::AsksMore(more) => assert_eq!(more, ["your microphone"]),
        other => panic!("{other:?}"),
    }
    assert_eq!(check(&root.join("org.example.Weather/app")).unwrap().version, "1.1", "nothing was installed");

    let dir = plugin(&work.join("4"), "org.example.Weather", "1.3");
    edit(&dir, "network;notifications", "network;notifications;listen");
    std::fs::write(dir.join("plugin.ini"), std::fs::read_to_string(dir.join("plugin.ini")).unwrap() + "\n[Network]\nListen=tcp:24810\n").unwrap();
    let v4 = offer_of(&dir, "w-1.3.tar.zst");
    let quiet = Offer { entry: Entry { permissions: mine.asks.clone(), ..v4.entry.clone() }, ..v4.clone() };
    match fetch_update_unseen(&quiet, &root).unwrap() {
        Unseen::AsksMore(more) => assert_eq!(more, ["let other computers on your home network reach it", "be reached on other ports than before"]),
        other => panic!("{other:?}"),
    }
    assert_eq!(check(&root.join("org.example.Weather/app")).unwrap().version, "1.1");

    let wrong = Offer { entry: Entry { sha256: "0".repeat(64), ..v2.entry.clone() }, ..v2.clone() };
    assert!(fetch_update_unseen(&wrong, &root).unwrap_err().contains("SHA-256"));

    assert_eq!(fetch_install(&v3, &root).unwrap().plugin.version, "1.2");
    assert_eq!(std::fs::read_to_string(root.join("org.example.Weather/allowed")).unwrap(), "network\n");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn which_installed_plugins_have_an_update_is_one_check() {
    let top = temp("which");
    let mine = |id: &str, version: &str| check(&plugin(&top.join(version), id, version)).unwrap();
    let offer = |id: &str, version: &str| Offer { entry: Entry { id: id.into(), version: version.into(), ..entry() }, icon: None };
    let installed = [mine("org.example.One", "1.0"), mine("org.example.Two", "2.0"), mine("org.example.Three", "1.9")];
    let offers = [offer("org.example.One", "1.1"), offer("org.example.Two", "2.0"), offer("org.example.Three", "1.10"), offer("org.example.Four", "9")];
    let found: Vec<(String, String)> = updates_for(&installed, &offers).into_iter().map(|(m, o)| (m.id, o.entry.version)).collect();
    assert_eq!(found, [("org.example.One".to_string(), "1.1".to_string()), ("org.example.Three".to_string(), "1.10".to_string())]);
    assert!(update_of(&offers, &installed[1]).is_none(), "the same version is no update");

    let mut places: Vec<(String, String)> = [("store/plugin_page.rs", crate::common::source::code(include_str!("../../store/plugin_page.rs"))), ("ring/updates.rs", crate::common::source::code(include_str!("../../ring/updates.rs")))].map(|(f, t)| (f.to_string(), t.to_string())).into();
    places.extend(crate::common::source::every_file().into_iter().filter(|(name, _)| name.starts_with("store/update_page/") || name.starts_with("store/pages/")));
    for (file, text) in places {
        assert!(!text.contains("catalogue::newer("), "{file} compares versions for itself");
    }
    assert!(crate::common::source::code(include_str!("../../store/plugin_page.rs")).contains("catalogue::update_of(") && crate::common::source::code(include_str!("../../ring/updates.rs")).contains("catalogue::updates_for("));
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_plugin_listed_twice_is_offered_once_at_its_newest() {
    let offer = |id: &str, name: &str, version: &str| Offer { entry: Entry { id: id.into(), name: name.into(), version: version.into(), ..entry() }, icon: None };
    let got: Vec<(String, String)> = one_of_each(vec![offer("org.example.Weather", "Weather", "1.2"), offer("org.example.Clock", "clock", "0.1"),
                                                      offer("org.example.Weather", "Weather", "1.10"), offer("org.example.Weather", "Weather", "1.9")])
        .into_iter().map(|o| (o.entry.id, o.entry.version)).collect();
    assert_eq!(got, [("org.example.Clock".to_string(), "0.1".to_string()), ("org.example.Weather".to_string(), "1.10".to_string())]);
}

#[test]
fn a_downloaded_catalogue_too_big_to_be_one_is_not_read() {
    let top = temp("big-catalogue");
    let (pair, public) = test_key();
    let mut text = numbered("1.0", 5);
    text.push_str(&"# padding\n".repeat(KEPT_MOST as usize / 10 + 1));
    assert!(parse(&text).is_ok_and(|(found, _)| found.len() == 1), "it would read as a catalogue, were it read");
    let trust = trust_in(&top, &[public]);
    assert_eq!(read(&Site::signed(&text, &pair), Duration::ZERO, &trust), (Vec::new(), None));
    assert!(!trust.kept.exists() && !trust.note.exists(), "nothing of it was kept");
    let _ = std::fs::remove_dir_all(&top);
}

#[test]
fn a_catalogue_of_very_many_plugins_is_read_in_good_time() {

    let mut text = write(&[], 1);
    for n in 0..12_000 {
        text.push_str(&format!("\n[Plugin org.example.P{n}]\nInterface={INTERFACE}\nName=Weather\nVersion=1.0\nSummary=\nPermissions=network;\n\
                                Archive=p.tar.zst\nSHA256={}\n", "ab".repeat(32)));
    }
    assert!(text.len() as u64 > KEPT_MOST, "as big as a catalogue may be, and more");
    let began = std::time::Instant::now();
    let (found, problems) = parse(&text).unwrap();
    let took = began.elapsed();
    assert_eq!((found.len(), problems.len()), (12_000, 0));
    assert_eq!(found[11_999].id, "org.example.P11999");
    assert!(took < Duration::from_secs(3), "reading it took {took:?}");

    let twice = format!("{}\n[Plugin org.example.Weather]\nInterface={INTERFACE}\nName=Other\nVersion=2.0\nArchive=other.tar.zst\nSHA256={}\n",
                        write(&[entry()], 1), "cd".repeat(32));
    let (found, _) = parse(&twice).unwrap();
    assert_eq!(found.iter().map(|e| (e.name.as_str(), e.version.as_str(), e.archive.as_str())).collect::<Vec<_>>(),
               [("Weather", "1.0", "org.example.Weather-1.0.tar.zst"), ("Other", "2.0", "other.tar.zst")]);
}

#[test]
fn a_first_run_takes_no_catalogue_numbered_before_the_first_key_was_made() {

    const _: () = assert!(NUMBER_FLOOR >= 1_791_763_200, "NUMBER_FLOOR is before 12 October 2026");
    let top = temp("floor");
    let (pair, public) = test_key();
    let trust = Trust { floor: NUMBER_FLOOR, ..trust_in(&top, &[public]) };
    let said = read(&Site::signed(&numbered("1.0", 5), &pair), Duration::ZERO, &trust);
    assert_eq!(said.0, Vec::<String>::new());
    assert!(said.1.is_some_and(|w| w.contains("is older than the one accepted before (number 5, after ")));
    assert_eq!(read(&Site::signed(&numbered("1.0", NUMBER_FLOOR), &pair), Duration::ZERO, &trust), (vec!["1.0".to_string()], None));
    let _ = std::fs::remove_dir_all(&top);
}

struct Shelf {
    top: PathBuf,
    shelf: PathBuf,
    root: PathBuf,
}

impl Shelf {
    fn new(name: &str) -> Shelf {
        let top = temp(name);
        let s = Shelf { shelf: top.join("shelf"), root: top.join("plugins"), top };
        std::fs::create_dir_all(&s.shelf).unwrap();
        s
    }

    fn offer(&self, id: &str, version: &str) -> Offer {
        self.offer_of(&plugin(&self.top.join("work").join(version), id, version), id)
    }

    fn offer_of(&self, dir: &Path, id: &str) -> Offer {
        let p = check(dir).unwrap();
        let name = format!("{id}-{}.tar.zst", p.version);
        pack(dir, &self.shelf, &name);
        Offer { entry: Entry { id: id.into(), version: p.version, permissions: p.asks, sha256: sha256_of(&self.shelf.join(&name)).unwrap(), archive: name, icon: None, ..entry() },
                icon: None }
    }

    fn get(&self) -> impl Fn(&str, &Path) -> Result<(), String> + '_ {
        move |url, to| std::fs::copy(self.shelf.join(url.rsplit('/').next().unwrap()), to).map(|_| ()).map_err(|e| e.to_string())
    }

    fn install(&self, offer: &Offer) -> Result<Installed, String> {
        install_offer(offer, &self.root, &self.get())
    }

    fn unseen(&self, offer: &Offer) -> Result<Unseen, String> {
        update_unseen(offer, &self.root, &self.get())
    }

    fn version(&self, id: &str) -> String {
        check(&self.root.join(id).join("app")).unwrap().version
    }
}

const WEATHER: &str = "org.example.Weather";

#[test]
fn only_an_install_from_the_signed_catalogue_leaves_its_mark() {
    use crate::common::own_hands::{is_flow_in, FLOW, MARK};
    let s = Shelf::new("mark");
    let mark = |id: &str| std::fs::read_to_string(s.root.join(id).join(MARK)).ok();

    let mut wrong = s.offer(FLOW, "1.0");
    wrong.entry.sha256 = "0".repeat(64);
    assert!(s.install(&wrong).is_err());
    assert_eq!(mark(FLOW), None);
    assert!(!is_flow_in(&s.root, FLOW));

    let first = s.offer(FLOW, "1.0");
    s.install(&first).unwrap();
    assert_eq!(mark(FLOW), Some(format!("{}\n", first.entry.sha256)));
    assert!(is_flow_in(&s.root, FLOW), "Flow from the catalogue was not taken for Flow");

    let newer = s.offer(FLOW, "1.1");
    assert!(matches!(s.unseen(&newer), Ok(Unseen::In(_))));
    assert_eq!(mark(FLOW), Some(format!("{}\n", newer.entry.sha256)));

    s.install(&s.offer(WEATHER, "1.0")).unwrap();
    assert!(mark(WEATHER).is_some() && !is_flow_in(&s.root, WEATHER));

    super::super::install::remove_at(&s.root, FLOW).unwrap();
    assert!(!is_flow_in(&s.root, FLOW));

    use crate::common::source::code;
    let mine = code(include_str!("../catalogue.rs")).split_once("#[cfg(test)]").unwrap().0;
    assert_eq!(mine.matches("own_hands::mark(").count(), 1);
    let mut places: Vec<(String, String)> = [("plugins/mod.rs", code(include_str!("../mod.rs"))), ("store/plugin_page.rs", code(include_str!("../../store/plugin_page.rs")))]
        .map(|(f, t)| (f.to_string(), t.to_string())).into();
    places.extend(crate::common::source::every_file().into_iter().filter(|(name, _)| name.starts_with("store/boxed/")));
    for (file, text) in places {
        assert!(!text.contains("own_hands::mark("), "{file} writes the catalogue's mark");
    }
    let _ = std::fs::remove_dir_all(&s.top);
}

#[test]
fn a_plugin_put_in_before_is_updated_only_from_the_catalogue() {
    let s = Shelf::new("put-in-before");
    for (n, note) in [Some("folder /shared/plugins\n"), Some("hand\n"), Some("catalogue\n"), Some("someone else\n"), None].into_iter().enumerate() {
        let id = format!("org.example.P{n}");
        let mine = s.install(&s.offer(&id, "1.0")).unwrap().plugin;
        let at = s.root.join(&id);
        if let Some(note) = note {
            std::fs::write(at.join("from"), note).unwrap();
        }
        std::fs::write(at.join("allowed"), "network\n").unwrap();

        assert_eq!(check(&at.join("app")).unwrap().version, "1.0", "{note:?}");

        assert!(update_of(&[], &mine).is_none(), "{note:?}");
        assert!(update_of(&[s.offer("org.example.Other", "9.9"), s.offer(&id, "1.0")], &mine).is_none(), "{note:?}");

        let newer = s.offer(&id, "1.1");
        assert!(update_of(std::slice::from_ref(&newer), &mine).is_some(), "{note:?}");
        assert!(matches!(s.unseen(&newer), Ok(Unseen::In(_))), "{note:?}");
        assert_eq!(s.version(&id), "1.1", "{note:?}");
        assert_eq!(std::fs::read_to_string(at.join("allowed")).unwrap(), "network\n", "{note:?}");

        let dir = plugin(&s.top.join("work/more").join(&id), &id, "1.2");
        std::fs::write(dir.join("plugin.ini"), std::fs::read_to_string(dir.join("plugin.ini")).unwrap()
            .replace("network;notifications", "network;notifications;microphone")).unwrap();
        let more = s.offer_of(&dir, &id);
        assert!(matches!(s.unseen(&more), Ok(Unseen::AsksMore(_))), "{note:?}");
        assert_eq!(s.version(&id), "1.1", "{note:?}");

        super::super::install::remove_at(&s.root, &id).unwrap();
        assert!(!at.exists(), "{note:?}");
    }
    let _ = std::fs::remove_dir_all(&s.top);
}

#[test]
fn a_plugins_box_is_given_its_program_and_its_home_and_nothing_else_of_its_folder() {
    let boxed: String = crate::common::source::every_file().into_iter().filter(|(name, _)| name.starts_with("store/boxed/") && !name.ends_with("/tests.rs"))
        .map(|(_, text)| crate::common::source::code(Box::leak(text.into_boxed_str())).to_string()).collect();
    assert!(boxed.contains(r#"add(&["--ro-bind", &app.to_string_lossy(), "/app"]);"#));
    assert!(boxed.contains(r#"add(&["--bind", &at.join("home").to_string_lossy(), &h]);"#));
    assert_eq!(boxed.matches("&at.to_string_lossy()").count(), 0, "a plugin's own folder, with what it is allowed, is never handed to its box");
}

#[test]
fn an_archive_that_holds_another_plugin_than_listed_is_refused() {
    let s = Shelf::new("other-id");
    let real = s.offer(WEATHER, "1.0");
    let lying = Offer { entry: Entry { id: "org.example.Trusted".into(), ..real.entry.clone() }, icon: None };
    let said = s.install(&lying).unwrap_err();
    assert!(said.contains("org.example.Weather, not org.example.Trusted"), "{said}");
    assert!(!s.root.join(WEATHER).exists() && !s.root.join("org.example.Trusted").exists());
    let _ = std::fs::remove_dir_all(&s.top);
}

#[test]
fn an_archive_that_unpacks_to_too_much_is_not_unpacked() {
    let top = temp("bomb");
    let dir = top.join("big");
    std::fs::create_dir_all(&dir).unwrap();
    for n in 0..3 {
        std::fs::write(dir.join(format!("f{n}")), vec![b'a'; 1000]).unwrap();
    }
    pack(&dir, &top, "big.tar.zst");
    let archive = top.join("big.tar.zst");
    assert_eq!(within(&archive, 3000, 10), Ok(()));
    assert!(within(&archive, 2999, 10).is_err(), "too many bytes");
    assert!(within(&archive, 3000, 3).is_err(), "too many files");
    let _ = std::fs::remove_dir_all(&top);
}
