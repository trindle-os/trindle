//! The word sheet: a good sheet comes back whole, and every hostile sheet is refused with nothing installed.

use super::*;

const ENGLISH: &str = "greet = Hello, { $name }\nopen = Open the file\ncount =\n    { $n ->\n        [one] One file\n       *[other] { $n } files\n    }\nbrand = { -brand } is here\n";

fn rows() -> Vec<Row> {
    rows_of(&[("t.ftl", ENGLISH)])
}

fn words(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

fn good() -> String {
    export("fr", &rows(), &words(&[("greet", "Bonjour, { $name }"), ("open", "Ouvrir le fichier")]))
}

fn temp(name: &str) -> PathBuf {
    static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("trindle-wordsheet-{}-{name}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn all_refused(hostile: Vec<(&str, Vec<u8>)>) {
    let rows = rows();
    let dir = temp("refused");
    let base = dir.join("lang");
    for (name, bytes) in hostile {
        assert!(import(&bytes, &rows).is_err(), "accepted: {name}");
        let file = dir.join("sheet.txt");
        std::fs::write(&file, &bytes).unwrap();
        assert!(take(&file, &rows, &base).is_err(), "installed: {name}");
        assert!(!base.exists(), "something was written for: {name}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

fn changed(from: &str, to: &str) -> Vec<u8> {
    let sheet = good();
    assert!(sheet.contains(from), "test sheet has no {from:?}");
    sheet.replacen(from, to, 1).into_bytes()
}

fn greeting(words: &str) -> Vec<u8> {
    changed("Bonjour, { $name }", words)
}

#[test]
fn a_good_sheet_comes_back_whole() {
    let got = import(good().as_bytes(), &rows()).unwrap();
    assert_eq!(got.lang, "fr");
    assert_eq!(got.words, words(&[("greet", "Bonjour, { $name }"), ("open", "Ouvrir le fichier")]));
}

#[test]
fn a_sheet_installs_and_reads_back_as_language_files() {
    let dir = temp("install");
    let file = dir.join("sheet.txt");
    let plural = "{ $n ->\n[one] Un fichier\n*[other] { $n } fichiers\n}";
    let sheet = export("fr", &rows(), &words(&[("greet", "Bonjour, { $name }"), ("count", plural)]));
    std::fs::write(&file, sheet).unwrap();
    let base = dir.join("lang");
    assert_eq!(take(&file, &rows(), &base).unwrap(), "fr");
    let back = installed(&base, "fr");
    assert_eq!(back.len(), 2);
    let bundle = lang::Bundle::from_files("fr", &lang::folder_files(&base.join("fr")).iter().map(|(a, b)| (a.as_str(), b.as_str())).collect::<Vec<_>>());
    assert!(bundle.get("greet").is_some() && bundle.get("count").is_some());
    install(&base, "fr", &BTreeMap::new()).unwrap();
    assert!(installed(&base, "fr").is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_translation_that_fails_its_check_is_never_written() {
    let sheet = export("fr", &rows(), &words(&[("greet", "<b>Bonjour</b>")]));
    assert!(import(sheet.as_bytes(), &rows()).unwrap().words.is_empty());
}

#[test]
fn keys_that_are_extra_missing_renamed_or_repeated_are_refused() {
    let sheet = good();
    let open_row = sheet.lines().find(|l| l.starts_with("open\t")).unwrap().to_string();
    let last = sheet.lines().last().unwrap().to_string();
    all_refused(vec![
        ("renamed", changed("\nopen\t", "\nopen2\t")),
        ("renamed to another real key", changed("\nopen\t", "\ngreet\t")),
        ("missing", sheet.replacen(&format!("{open_row}\n"), "", 1).into_bytes()),
        ("extra", format!("{sheet}extra\tMore\t\n").into_bytes()),
        ("extra empty line", format!("{sheet}\n\n").into_bytes()),
        ("repeated", sheet.replacen(&format!("{open_row}\n"), &format!("{open_row}\n{open_row}\n"), 1).into_bytes()),
        ("truncated", sheet.as_bytes()[..sheet.len() - last.len() - 1].to_vec()),
        ("reordered", sheet.replacen(&format!("{open_row}\n"), "", 1).replace("greet\t", &format!("{open_row}\ngreet\t")).into_bytes()),
    ]);
}

#[test]
fn a_changed_english_line_is_refused() {
    all_refused(vec![
        ("english changed", changed("Open the file", "Open the files")),
        ("english emptied", changed("\tOpen the file\t", "\t\t")),
        ("english copied into the box", changed("Ouvrir le fichier", "Open the file\tOuvrir")),
        ("english swapped", changed("Hello, { $name }\t", "Hello, { $nom }\t")),
    ]);
}

#[test]
fn a_changed_header_version_language_or_notes_is_refused() {
    all_refused(vec![
        ("kind", changed("Trindle word sheet", "Trindle word sheets")),
        ("kind removed", changed("Trindle word sheet\n", "")),
        ("version", changed("version: 1", "version: 2")),
        ("language", changed("language: fr", "language: de")),
        ("language path", changed("language: fr", "language: ../fr")),
        ("language empty", changed("language: fr", "language: ")),
        ("language long", changed("language: fr", "language: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")),
        ("rows", changed("rows: 4", "rows: 5")),
        ("note changed", changed("Change only", "Change anything but")),
        ("note removed", changed(&format!("{}\n", NOTES[0]), "")),
        ("blank line removed", changed("\n\nbrand", "\nbrand")),
        ("byte order mark", [&b"\xef\xbb\xbf"[..], good().as_bytes()].concat()),
        ("line ends", good().replace('\n', "\r\n").into_bytes()),
    ]);
}

#[test]
fn a_broken_copied_or_another_sheets_check_value_is_refused() {
    let own = good();
    let sig = own.lines().find_map(|l| l.strip_prefix("check value: ")).unwrap().to_string();
    let mut broken = sig.clone();
    broken.replace_range(0..1, if sig.starts_with('0') { "1" } else { "0" });
    let other_language = export("de", &rows(), &BTreeMap::new());
    let de_sig = other_language.lines().find_map(|l| l.strip_prefix("check value: ")).unwrap().to_string();
    let other_rows = rows_of(&[("o.ftl", "greet = Hello, { $name }\nopen = Open the file\ncount = Files\nbrand = { -brand } is here\n")]);
    let other_sheet = check_value("fr", &other_rows);
    all_refused(vec![
        ("broken", changed(&sig, &broken)),
        ("copied from another language", changed(&sig, &de_sig)),
        ("from a sheet of other English", changed(&sig, &other_sheet)),
        ("empty", changed(&sig, "")),
        ("short", changed(&sig, &sig[..10])),
        ("removed", changed(&format!("check value: {sig}\n"), "")),
        ("upper case", changed(&sig, &sig.to_uppercase())),
    ]);
}

#[test]
fn markup_scripts_and_commands_in_a_translation_are_refused() {
    all_refused(vec![
        ("markup", greeting("<b>Bonjour</b>, { $name }")),
        ("script", greeting("<script>alert(1)</script> { $name }")),
        ("image", greeting("<img src=x onerror=1> { $name }")),
        ("entity", greeting("&#60;b&#62; { $name }")),
        ("backtick", greeting("`id` { $name }")),
        ("command", greeting("$(rm -rf ~) { $name }")),
        ("expansion", greeting("${HOME} { $name }")),
        ("javascript", greeting("javascript:alert(1) { $name }")),
    ]);
}

#[test]
fn control_invisible_and_override_characters_are_refused() {
    let bad = |c: &str| greeting(&format!("Bon{c}jour, {{ $name }}"));
    all_refused(vec![
        ("escape code", bad("\u{1b}[31m")),
        ("null byte", bad("\u{0}")),
        ("bell", bad("\u{7}")),
        ("carriage return", bad("\r")),
        ("right to left override", bad("\u{202e}")),
        ("left to right override", bad("\u{202d}")),
        ("isolate", bad("\u{2066}")),
        ("zero width space", bad("\u{200b}")),
        ("zero width joiner", bad("\u{200d}")),
        ("word joiner", bad("\u{2060}")),
        ("soft hyphen", bad("\u{ad}")),
        ("byte order mark inside", bad("\u{feff}")),
        ("tag character", bad("\u{e0041}")),
        ("private use", bad("\u{e000}")),
        ("variation selector", bad("\u{fe0f}")),
        ("replacement character", bad("\u{fffd}")),
        ("tab", bad("\t")),
        ("whole-file escape", [&b"\x1b[2J"[..], good().as_bytes()].concat()),
        ("null at the end", [good().as_bytes(), b"\0"].concat()),
    ]);
}

#[test]
fn added_dropped_or_changed_placeholders_are_refused() {
    all_refused(vec![
        ("dropped", greeting("Bonjour")),
        ("added", greeting("Bonjour, { $name } { $age }")),
        ("renamed", greeting("Bonjour, { $nom }")),
        ("term added", greeting("Bonjour, { $name } { -brand }")),
        ("message named", greeting("Bonjour, { $name } { open }")),
        ("function", greeting("Bonjour, { NUMBER($name) }")),
        ("unclosed", greeting("Bonjour, { $name")),
        ("selector not closed", greeting("{ $name ->\n[one] x\n*[other] y")),
        ("selector on another variable", greeting("{ $age ->\n*[other] { $name }\n}")),
    ]);
}

#[test]
fn very_long_lines_and_huge_files_are_refused() {
    let long = "a".repeat(20_000);
    let mut huge = good().into_bytes();
    huge.extend(std::iter::repeat_n(b'x', (FILE_MOST as usize) + 1));
    all_refused(vec![
        ("long translation", greeting(&format!("Bonjour {long} {{ $name }}"))),
        ("long within the line limit", greeting(&format!("Bonjour {} {{ $name }}", "a".repeat(2500)))),
        ("long line outside any row", changed("\n\nbrand", &format!("\n{long}\nbrand"))),
        ("huge file", huge),
        ("empty file", Vec::new()),
        ("only a newline", b"\n".to_vec()),
    ]);
}

#[test]
fn bad_encoding_is_refused() {
    let mut cut = good().into_bytes();
    let at = cut.iter().position(|b| *b == b'B').unwrap();
    cut.splice(at..at + 1, [0xc3]);
    let mut latin = good().into_bytes();
    let o = latin.iter().position(|b| *b == b'O').unwrap();
    latin.splice(o..o + 1, [0xff, 0xfe]);
    all_refused(vec![
        ("cut character", cut),
        ("not utf-8", latin),
        ("utf-16", good().encode_utf16().flat_map(|u| u.to_le_bytes()).collect()),
        ("binary", (0u8..=255).collect()),
    ]);
}

#[test]
fn paths_and_links_are_refused() {
    all_refused(vec![
        ("absolute path", greeting("/etc/passwd { $name }")),
        ("home path", greeting("~/.ssh/id_ed25519 { $name }")),
        ("up a folder", greeting("../../secret { $name }")),
        ("web link", greeting("https://evil.example/x { $name }")),
        ("file link", greeting("file:///etc/passwd { $name }")),
        ("windows path", greeting("C:\\\\Windows { $name }")),
        ("link as the language", changed("language: fr", "language: https://x")),
    ]);
}

#[test]
fn badly_formed_rows_are_refused() {
    all_refused(vec![
        ("extra column", changed("Ouvrir le fichier", "Ouvrir\textra")),
        ("no translation column", changed("Open the file\tOuvrir le fichier", "Open the file")),
        ("unknown escape", changed("Ouvrir le fichier", "Ouvrir\\x41")),
        ("escape at the end", changed("Ouvrir le fichier", "Ouvrir\\")),
        ("spaces around", changed("\tOuvrir le fichier", "\t  Ouvrir le fichier ")),
        ("space before the tab", changed("greet\t", "greet \t")),
    ]);
}

#[test]
fn a_link_to_a_sheet_and_anything_but_a_plain_file_is_refused() {
    let dir = temp("links");
    let real = dir.join("real.txt");
    std::fs::write(&real, good()).unwrap();
    let link = dir.join("link.txt");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let base = dir.join("lang");
    assert!(take(&real, &rows(), &base).is_ok());
    let _ = std::fs::remove_dir_all(&base);
    assert!(take(&link, &rows(), &base).is_err());
    assert!(take(&dir, &rows(), &base).is_err());
    assert!(take(&dir.join("missing"), &rows(), &base).is_err());
    assert!(!base.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_sheet_for_another_language_cannot_borrow_this_ones_check_value() {
    let fr = good();
    let de = export("de", &rows(), &words(&[("greet", "Hallo, { $name }")]));
    let swapped = {
        let sig = |s: &str| s.lines().find_map(|l| l.strip_prefix("check value: ")).unwrap().to_string();
        de.replace(&sig(&de), &sig(&fr))
    };
    all_refused(vec![("german with the french check value", swapped.into_bytes())]);
    assert!(import(de.as_bytes(), &rows()).is_ok());
}

#[test]
fn trindles_own_english_makes_a_sheet_that_comes_back_and_a_changed_one_does_not() {
    let real = english_rows();
    assert!(real.len() > 1000, "only {} messages", real.len());
    let pick = |i: usize| real[i * real.len() / 7..].iter().find(|r| !r.english.contains('\n')).unwrap();
    let mine = words(&[(pick(1).key.as_str(), pick(1).english.as_str()), (pick(3).key.as_str(), pick(3).english.as_str())]);
    let sheet = export("sw", &real, &mine);
    assert!(sheet.len() as u64 <= FILE_MOST, "a sheet is {} bytes", sheet.len());
    assert_eq!(import(sheet.as_bytes(), &real).unwrap().words, mine);
    let third = pick(5);
    let bad_english = sheet.replacen(&format!("{}\t{}\t", third.key, escape(&third.english)), &format!("{}\t{}x\t", third.key, escape(&third.english)), 1);
    assert_eq!(import(bad_english.as_bytes(), &real), Err(Why::English(third.key.clone())));
    let dropped = sheet.replacen(&format!("{}\t", third.key), "zzz-not-a-key\t", 1);
    assert!(import(dropped.as_bytes(), &real).is_err());
}

const SHIPPED_ALLOWED: &[&str] = &[];

#[test]
fn what_the_shipped_languages_say_passes_the_checks() {
    let real = english_rows();
    let by_key: std::collections::HashMap<&str, &Row> = real.iter().map(|r| (r.key.as_str(), r)).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failed = Vec::new();
    let mut seen = 0;
    for tag in ["es", "fr", "zh-Hans", "hi", "ar", "pt-BR", "bn", "de", "id", "ru", "sw", "tr", "ur", "vi"] {
        for share in [root.join("share/lang"), crate::common::source::toolkit_root().join("share/lang")] {
            for (_, text) in lang::folder_files(&share.join(tag)) {
                for (key, body) in lang::raw_messages(&text) {
                    let Some(row) = by_key.get(key.as_str()) else { continue };
                    seen += 1;
                    if let Err(why) = check_words(row, &body) {
                        let line = format!("{tag} {key}: {why:?}");
                        if !SHIPPED_ALLOWED.contains(&line.as_str()) {
                            failed.push(line);
                        }
                    }
                }
            }
        }
    }
    assert!(seen > 0);
    assert!(failed.is_empty(), "{} of {seen} shipped lines fail, for example {:?}", failed.len(), &failed[..failed.len().min(40)]);
}

#[test]
fn what_other_languages_write_is_not_mistaken_for_an_attack() {
    let one = |english: &str, words: &str| {
        let row = Row { key: "k".into(), english: english.into(), marks: lang::placeholders(english).unwrap() };
        check_words(&row, words)
    };

    assert!(one("{ $n } could not be taken out", "{ $n ->\n    [one] 1 \u{e9}l\u{e9}ment\n   *[other] { $n } \u{e9}l\u{e9}ments\n}").is_ok());
    assert!(one("{ $n ->\n    [one] 1 found\n   *[other] { $n } found\n}", "\u{627e}\u{5230} { $n } \u{9879}").is_ok());
    assert!(one("{ $n } found", "{ $n ->\n   *[other] { $n } found\n}").is_ok());

    assert!(one("{ $n } found", "{ $m ->\n   *[other] { $n } found\n}").is_err());
    assert!(one("{ $n } found", "{ $n ->\n   *[other] { $m } found\n}").is_err());
    assert!(one("{ $n } found", "{ $m } found").is_err());

    assert!(one("of { $amount }", "/ { $amount }").is_ok());
    assert!(one("Port { $port } worked", "\u{644}\u{645} USB \u{200f}{ $port } \u{628}").is_ok());
    assert!(one("Open { $path }", "/etc/passwd { $path }").is_err(), "a path is still refused");
    assert!(one("Open { $path }", "~/x { $path }").is_err());
    assert!(one("Open { $path }", "\u{6253}\u{5f00}/etc/fstab { $path }").is_err(), "a path set in Chinese text is found too");
    assert!(one("Read it in /usr/share/doc/.", "\u{5728} /usr/share/doc/ \u{4e2d}\u{9605}\u{8bfb}\u{3002}").is_ok());
    assert!(one("Read it in /usr/share/doc/.", "\u{5728}/usr/share/doc/\u{4e2d}\u{9605}\u{8bfb}").is_ok());
    assert!(one("Open { $path }", "<b>{ $path }</b>").is_err());

    let english = "word ".repeat(300);
    let hindi = "\u{936}\u{92c}\u{94d}\u{926} ".repeat(300);
    assert!(hindi.len() > 2000 && one(&english, hindi.trim()).is_ok());
    assert!(one(&english, &"x".repeat(2001)).is_err(), "the most is still the most");
}

#[test]
fn a_joiner_is_kept_between_letters_of_the_scripts_that_need_it_and_nowhere_else() {
    let greet = rows().into_iter().find(|r| r.key == "greet").unwrap();
    let ok = |w: &str| check_words(&greet, &format!("{w} {{ $name }}")).is_ok();

    assert!(ok("\u{645}\u{6cc}\u{200c}\u{62e}\u{648}\u{627}\u{647}\u{645}"));

    assert!(ok("\u{d28}\u{d4d}\u{200d}"));
    assert!(ok("\u{d28}\u{d4d}\u{200d}\u{d28}"));

    for bad in ["Bon\u{200c}jour", "Bon\u{200d}jour", "\u{200c}\u{645}\u{6cc}", "\u{200d}\u{d28}", "\u{645}\u{200c}\u{200c}\u{6cc}",
                "\u{645}\u{200c}\u{200d}\u{6cc}", "\u{645}\u{200c} \u{6cc}", "\u{645}\u{200c}", "\u{645}\u{200b}\u{6cc}"] {
        assert!(!ok(bad), "kept {bad:?}");
    }

    let persian = "\u{645}\u{6cc}\u{200c}\u{62e}\u{648}\u{627}\u{647}\u{645} { $name }";
    let sheet = export("fa", &rows(), &words(&[("greet", persian)]));
    assert_eq!(import(sheet.as_bytes(), &rows()).unwrap().words, words(&[("greet", persian)]));
}

#[test]
fn many_words_are_split_into_parts_and_a_shrunk_set_leaves_no_stale_part() {
    let dir = temp("parts");
    let big: BTreeMap<String, String> = (0..3000).map(|i| (format!("k{i:05}"), "x".repeat(300))).collect();
    let parts = as_ftl(&big);
    assert!(parts.len() >= 3 && parts.iter().all(|p| p.len() <= PART_MOST), "{} parts", parts.len());
    install(&dir, "fr", &big).unwrap();
    let on_disk = |n: usize| (0..n + 2).filter(|i| dir.join("fr").join(format!("{PART}{i:03}.ftl")).exists()).count();
    assert_eq!(on_disk(parts.len()), parts.len());
    assert_eq!(installed(&dir, "fr"), big, "everything written comes back");
    let small = words(&[("a", "one")]);
    install(&dir, "fr", &small).unwrap();
    assert_eq!(on_disk(parts.len()), 1, "the old parts are gone");
    assert_eq!(installed(&dir, "fr"), small);
    install(&dir, "fr", &BTreeMap::new()).unwrap();
    assert_eq!(on_disk(parts.len()), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_that_cannot_be_written_keeps_the_old_words_and_an_import_says_not_saved() {
    use std::os::unix::fs::PermissionsExt;
    let dir = temp("readonly");
    let base = dir.join("lang");
    install(&base, "fr", &words(&[("greet", "Bonjour, { $name }")])).unwrap();
    let file = dir.join("sheet.txt");
    std::fs::write(&file, good()).unwrap();
    std::fs::set_permissions(base.join("fr"), std::fs::Permissions::from_mode(0o500)).unwrap();
    if std::fs::write(base.join("fr").join("probe"), b"x").is_ok() {
        // Running as a user the permissions do not stop (root): nothing to prove.
        let _ = std::fs::remove_dir_all(&dir);
        return;
    }
    assert!(install(&base, "fr", &words(&[("open", "Ouvrir le fichier")])).is_err());
    assert_eq!(installed(&base, "fr"), words(&[("greet", "Bonjour, { $name }")]), "the old words are as they were");
    assert_eq!(take(&file, &rows(), &base), Err(Why::NotSaved), "the disk failed, not the sheet");
    assert_eq!(installed(&base, "fr"), words(&[("greet", "Bonjour, { $name }")]));
    std::fs::set_permissions(base.join("fr"), std::fs::Permissions::from_mode(0o700)).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_import_keeps_the_words_it_replaces_as_a_backup_the_reader_never_takes() {
    let dir = temp("backup");
    let base = dir.join("lang");
    let mine = words(&[("greet", "Salut, { $name }"), ("open", "Ouvre le fichier")]);
    install(&base, "fr", &mine).unwrap();
    let file = dir.join("sheet.txt");
    std::fs::write(&file, good()).unwrap();
    assert_eq!(take(&file, &rows(), &base), Ok("fr".to_string()));
    assert_eq!(installed(&base, "fr"), words(&[("greet", "Bonjour, { $name }"), ("open", "Ouvrir le fichier")]));
    let bak = std::fs::read_to_string(base.join("fr").join(format!("{PART}000.ftl.bak"))).unwrap();
    assert_eq!(lang::raw_messages(&bak).into_iter().collect::<BTreeMap<_, _>>(), mine, "the earlier words are kept");
    assert!(lang::folder_files(&base.join("fr")).iter().all(|(n, _)| !n.ends_with(".bak")), "the reader never takes it");

    let blank = export("fr", &rows(), &BTreeMap::new());
    std::fs::write(&file, blank).unwrap();
    assert_eq!(take(&file, &rows(), &base), Ok("fr".to_string()));
    assert!(installed(&base, "fr").is_empty());
    let bak = std::fs::read_to_string(base.join("fr").join(format!("{PART}000.ftl.bak"))).unwrap();
    assert!(bak.contains("Bonjour"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_linked_or_oversize_file_is_not_read() {
    let dir = temp("files");
    let real = dir.join("real.txt");
    std::fs::write(&real, good()).unwrap();
    let link = dir.join("link.txt");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    assert!(import_file(&real, &rows()).is_ok());
    assert_eq!(import_file(&link, &rows()), Err(Why::NotSheet));
    assert_eq!(import_file(&dir, &rows()), Err(Why::NotSheet));
    let big = dir.join("big.txt");
    std::fs::write(&big, vec![b'x'; FILE_MOST as usize + 1]).unwrap();
    assert_eq!(import_file(&big, &rows()), Err(Why::TooBig));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_way_back_to_trindles_own_words_is_never_changed_by_a_sheet() {
    let rows = rows_of(&[("t.ftl", "settings-words-reset = Back\nopen = Open the file\n")]);
    let dir = temp("way-back");
    let file = dir.join("sheet.txt");
    std::fs::write(&file, export("fr", &rows, &words(&[("settings-words-reset", "Qapla"), ("open", "Ouvrir")]))).unwrap();
    let base = dir.join("lang");
    take(&file, &rows, &base).unwrap();
    assert_eq!(installed(&base, "fr"), words(&[("open", "Ouvrir")]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_changed_word_longer_than_its_place_allows_is_refused_but_trindles_own_is_not_checked() {
    let rows = rows_of(&[("t.ftl", "go = Go\nline = Open the file now please\n")]);
    let dir = temp("room");
    let base = dir.join("lang");
    let file = dir.join("sheet.txt");
    std::fs::write(&file, export("fr", &rows, &words(&[("go", "Maintenant tout de suite")]))).unwrap();
    assert!(matches!(take(&file, &rows, &base), Err(Why::Long(k)) if k == "go"));
    assert!(!base.exists(), "nothing written");
    std::fs::write(&file, export("fr", &rows, &words(&[("go", "Allez"), ("line", "Ouvrez le fichier maintenant, je vous prie")]))).unwrap();
    take(&file, &rows, &base).unwrap();
    assert_eq!(room(2), 8);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn hostile_names_for_a_language_of_your_own_are_refused_and_nothing_is_written() {
    let dir = temp("own-names");
    let base = dir.join("lang");
    let long = "k".repeat(31);
    let huge = "k".repeat(1 << 20);
    for (name, why) in [("", Why::NameLength), ("   ", Why::NameLength), (long.as_str(), Why::NameLength), (huge.as_str(), Why::NameLength),
                        ("<b>Klingon</b>", Why::NamePlain), ("Kl&amp;", Why::NamePlain), ("{ $x }", Why::NamePlain), ("$(rm)", Why::NamePlain),
                        ("../../etc", Why::NamePlain), ("a/b", Why::NamePlain), ("a\\b", Why::NamePlain), ("~root", Why::NamePlain),
                        ("file:x", Why::NamePlain), ("..", Why::NamePlain), ("\u{202e}Klingon", Why::NamePlain), ("Kling\u{200b}on", Why::NamePlain),
                        ("Kl\ningon", Why::NamePlain), ("Kl\u{0}ingon", Why::NamePlain), ("Kl\u{1b}[31m", Why::NamePlain), ("\u{feff}Klingon", Why::NamePlain),
                        ("Kl\u{e000}", Why::NamePlain), ("Kl\u{fffd}", Why::NamePlain)] {
        assert_eq!(own::start(&base, name, std::slice::from_ref(&base)), Err(why), "accepted: {:?}", name.chars().take(40).collect::<String>());
        assert!(!base.exists(), "something was written for: {:?}", name.chars().take(40).collect::<String>());
    }
    for good in ["Klingon", "tlhIngan Hol", "Na'vi", "Эльфийский", "نیم\u{200c}فاصله", &"k".repeat(30)] {
        assert_eq!(own::name_ok(good).as_deref(), Ok(good), "{good}");
    }
    let tag = own::start(&base, "  Klingon ", std::slice::from_ref(&base)).unwrap();
    assert_eq!(tag, "x-klingon");
    assert_eq!(own::start(&base, "KLINGON", std::slice::from_ref(&base)), Err(Why::NameTaken), "a name used already, ignoring capitals");
    assert_eq!(own::languages(&base), vec![("x-klingon".to_string(), "Klingon".to_string())]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_name_becomes_a_private_tag_that_is_safe_reads_back_as_itself_and_never_collides() {
    let dir = temp("own-tags");
    let (base, system) = (dir.join("lang"), dir.join("share"));
    std::fs::create_dir_all(system.join("X-Klingon")).unwrap();
    std::fs::create_dir_all(system.join("fr")).unwrap();
    let dirs = vec![base.clone(), system.clone()];
    assert_eq!(own::tag_for("Klingon", &dirs).as_deref(), Some("x-klingon2"), "a folder of that name anywhere, ignoring capitals");
    assert_eq!(own::tag_for("Ab", &dirs).as_deref(), Some("x-ab2"), "never read as a country");
    assert_eq!(own::tag_for("Elfs", &dirs).as_deref(), Some("x-elfs2"), "never read as a script");
    assert_eq!(own::tag_for("123", &dirs).as_deref(), Some("x-123"), "three digits read back as themselves");
    assert_eq!(own::tag_for("Эльфийский", &dirs).as_deref(), Some("x-own"));
    assert_eq!(own::tag_for("tlhIngan Hol", &dirs).as_deref(), Some("x-tlhingan"));
    assert_eq!(own::start(&base, "tlhIngan Hol", &dirs).as_deref(), Ok("x-tlhingan"));
    assert_eq!(own::start(&base, "tlhIngan Holmes", &dirs).as_deref(), Ok("x-tlhinga2"), "cut to leave room for the number");
    for tag in ["x-klingon2", "x-ab2", "x-own", "x-tlhinga2"] {
        assert!(tag_ok(tag) && own::is_own(tag) && lang::tags_of(tag)[0] == tag, "{tag}");
    }
    assert!(!tag_ok("x") && !tag_ok("X"), "a lone x is never a language");
    assert!(!own::is_own("fr") && !own::is_own("xh"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_name_is_read_as_plain_text_from_its_own_file_never_from_a_link_or_a_bad_file() {
    let dir = temp("own-file");
    let base = dir.join("lang");
    let tag = own::start(&base, "Klingon", std::slice::from_ref(&base)).unwrap();
    assert_eq!(own::name_of(&base, &tag).as_deref(), Some("Klingon"));
    let file = base.join(&tag).join("name.txt");
    for bad in ["<script>x</script>".as_bytes(), b"Kl\x1b[2Jon", &[0xff, 0xfe], &[b'k'; 300], b""] {
        std::fs::write(&file, bad).unwrap();
        assert_eq!(own::name_of(&base, &tag), None);
        assert_eq!(own::languages(&base), vec![(tag.clone(), tag.clone())], "the tag stands in for a bad name");
    }
    std::fs::remove_file(&file).unwrap();
    let real = dir.join("real.txt");
    std::fs::write(&real, "Elvish").unwrap();
    std::os::unix::fs::symlink(&real, &file).unwrap();
    assert_eq!(own::name_of(&base, &tag), None, "never through a link");
    assert_eq!(own::name_of(&base, "../real"), None);
    assert!(lang::folder_files(&base.join(&tag)).is_empty(), "the reader never takes the name as words");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_language_of_your_own_starts_as_english_and_installs_only_what_changed() {
    let dir = temp("own-sheet");
    let base = dir.join("lang");
    let tag = own::start(&base, "Klingon", std::slice::from_ref(&base)).unwrap();
    let english: BTreeMap<String, String> = rows().into_iter().map(|r| (r.key, r.english)).collect();
    assert_eq!(in_use(&tag, &rows(), &base), english, "a copy of English, never another language");
    let sheet = export(&tag, &rows(), &in_use(&tag, &rows(), &base));
    let trimmed: BTreeMap<String, String> = english.iter().map(|(k, w)| (k.clone(), w.trim().to_string())).collect();
    assert_eq!(import(sheet.as_bytes(), &rows()).unwrap().words, trimmed);
    let file = dir.join("sheet.txt");
    std::fs::write(&file, sheet.replacen("Open the file\tOpen the file", "Open the file\tpoSmoH", 1)).unwrap();
    assert_eq!(take(&file, &rows(), &base), Ok(tag.clone()));
    assert_eq!(installed(&base, &tag), words(&[("open", "poSmoH")]));
    assert_eq!(own::name_of(&base, &tag).as_deref(), Some("Klingon"), "the name stays beside the words");
    assert!(!sheet.contains("Klingon"), "the name never travels in the sheet");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn back_to_trindles_words_removes_a_language_of_your_own_and_keeps_it_as_a_backup() {
    let dir = temp("own-remove");
    let base = dir.join("lang");
    let tag = own::start(&base, "Klingon", std::slice::from_ref(&base)).unwrap();
    install(&base, &tag, &words(&[("open", "poSmoH")])).unwrap();
    own::remove(&base, &tag).unwrap();
    assert!(!base.join(&tag).exists() && own::languages(&base).is_empty());
    let bak = base.join(format!("{tag}.bak"));
    assert!(std::fs::read_to_string(bak.join("name.txt")).unwrap() == "Klingon" && bak.join(format!("{PART}000.ftl")).exists());
    assert!(crate::common::language_names::codes_in(std::slice::from_ref(&base)).iter().all(|c| !own::is_own(c)), "no reader takes the backup");
    let again = own::start(&base, "Klingon", std::slice::from_ref(&base)).unwrap();
    assert_eq!(again, tag);
    own::remove(&base, &again).unwrap();
    assert!(!bak.join(format!("{PART}000.ftl")).exists(), "the newer backup replaces the older");
    assert!(own::remove(&base, "fr").is_err() && own::remove(&base, "x-../a").is_err(), "only a language of your own");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn languages_of_your_own_are_listed_after_the_real_ones_by_their_name_and_never_as_trindles() {
    use crate::common::language_names::Tongue;
    let dir = temp("own-list");
    let base = dir.join("lang");
    own::start(&base, "Klingon", std::slice::from_ref(&base)).unwrap();
    let found = vec![Tongue { code: "x-klingon".into(), name: "x-klingon".into(), done: 1, all: 9, machine: true },
                     Tongue { code: "fr".into(), name: "Français".into(), done: 9, all: 9, machine: false }];
    let listed = own::with_own(found, &base);
    assert_eq!(listed.iter().map(|t| t.code.as_str()).collect::<Vec<_>>(), ["fr", "x-klingon"]);
    assert!(listed[1].name.contains("Klingon") && listed[1].name.contains("(your own)") && !listed[1].machine && listed[1].done == 1);
    own::start(&base, "Elvish", std::slice::from_ref(&base)).unwrap();
    let listed = own::with_own(Vec::new(), &base);
    assert_eq!(listed.iter().map(|t| t.code.as_str()).collect::<Vec<_>>(), ["x-elvish", "x-klingon"], "one with no words yet is listed too");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_sheet_never_changes_the_way_out_of_a_language_through_the_language_list() {
    let rows = rows_of(&[("t.ftl", "settings-language-pick = Trindle's language\nsettings-language-auto = Follow the system\nsettings-language-heading = Language of Trindle\nsettings-words-reset-own-sub = Removes it\nopen = Open the file\n")]);
    let dir = temp("way-out");
    let file = dir.join("sheet.txt");
    let all: Vec<(&str, &str)> = rows.iter().map(|r| (r.key.as_str(), "Qapla")).collect();
    std::fs::write(&file, export("x-klingon", &rows, &words(&all))).unwrap();
    let base = dir.join("lang");
    take(&file, &rows, &base).unwrap();
    assert_eq!(installed(&base, "x-klingon"), words(&[("open", "Qapla")]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn words_that_begin_on_the_next_line_keep_that_break() {
    let rows = rows_of(&[("t.ftl", "note =\n    First line\n    Second line\n")]);
    let ftl = as_ftl(&words(&[("note", &rows[0].english)]));
    assert_eq!(rows_of(&[("t.ftl", &ftl[0])]), rows);
}
