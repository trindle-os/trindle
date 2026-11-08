//! Tests for crash and bug reports: what is kept, sent and left out.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use super::*;

fn who() -> Who {
    Who { home: "/home/jsmith".into(), names: vec!["jsmith".into(), "John".into(), "Smith".into(), "laptop-7".into(), "ab".into(), "mary".into()],
          ssids: vec!["Home Net 5G".into(), "BT-Hub6".into(), "ab".into()] }
}

#[test]
fn wifi_names_addresses_hostnames_and_usernames_are_each_blanked() {
    let cases = [

        ("joined Home Net 5G at 10:15:30", "joined [wifi] at 10:15:30"),
        ("connected to BT-Hub6.", "connected to [wifi]."),
        ("wlan0: SSID=Cafe_Guest rssi -40", "wlan0: SSID=[wifi] rssi -40"),
        ("ssid: \"Some Place\" ok", "ssid: \"[wifi]\" ok"),
        ("SSID 'Neighbour wifi' seen", "SSID '[wifi]' seen"),
        ("the ssid list is empty", "the ssid list is empty"),

        ("lease 192.168.1.20 from 10.0.0.1", "lease [ip] from [ip]"),
        ("peer 203.0.113.7:443 closed", "peer [ip]:443 closed"),
        ("version 1.2.3 and 1.2.3.4.5 stay", "version 1.2.3 and 1.2.3.4.5 stay"),
        ("not 999.1.1.1", "not 999.1.1.1"),

        ("addr fe80::1c2b:3dff:fe4a:9b10%wlan0 up", "addr [ip] up"),
        ("[2001:db8::1]:8080", "[[ip]]:8080"),
        ("loopback ::1.", "loopback [ip]."),
        ("core::fmt::Display and std::io", "core::fmt::Display and std::io"),

        ("hw 3c:97:0e:aa:bb:cc up", "hw [mac] up"),
        ("hw 3C-97-0E-AA-BB-CC up", "hw [mac] up"),
        ("time 10:15:30 and 12:00", "time 10:15:30 and 12:00"),
        ("addr:192.168.1.5 up", "addr:[ip] up"),
        ("mac=aa:bb:cc:dd:ee:ff up", "mac=[mac] up"),

        ("host laptop-7 up, laptop-7.local too", "host [name] up, [name].local too"),

        ("user jsmith and Mary ran it", "user [name] and [name] ran it"),
        ("owner mary:users", "owner [name]:users"),
    ];
    for (line, want) in cases {
        assert_eq!(scrub::scrub(line, &who()), want, "{line}");
    }
}

#[test]
fn paths_names_and_email_addresses_are_blanked_over_many_lines() {
    let cases = [
        ("opened /home/jsmith/Documents/tax.pdf", "opened ~/Documents/tax.pdf"),
        ("cwd=/home/jsmith", "cwd=~"),
        ("path /home/mary/.config/trindle failed", "path /home/[name]/.config/trindle failed"),
        ("(/home/bob)", "(/home/[name])"),
        ("mail from john.smith@example.com sent", "mail from [email] sent"),
        ("to=<jsmith@mail.example.co.uk>", "to=<[email]>"),
        ("a@b.com, c@d.org", "[email], [email]"),
        ("user John logged in", "user [name] logged in"),
        ("john SMITH jsmith", "[name] [name] [name]"),
        ("Johnson stays, John goes", "Johnson stays, [name] goes"),
        ("jsmith_backup stays", "[name]_backup stays"),
        ("user ab stays", "user ab stays"),
        ("no personal words here", "no personal words here"),
        ("at sign @ alone and x@y", "at sign @ alone and x@y"),
        ("end of sentence a@b.com.", "end of sentence [email]."),
        ("/home/jsmith/John/Smith.txt", "~/[name]/[name].txt"),
        ("ring: /home/jsmith/.cache/x john@example.org", "ring: ~/.cache/x [email]"),
    ];
    for (line, want) in cases {
        let got = scrub::scrub(line, &who());
        assert_eq!(got, want, "{line}");
        for secret in ["jsmith", "john.smith", "mary", "bob", "@example", "@mail", "@b.com"] {
            assert!(!got.to_lowercase().contains(secret), "{secret} left in {got}");
        }
    }
}

#[test]
fn only_the_last_lines_are_kept_and_each_is_blanked() {
    let raw: String = (0..500).fold(String::new(), |mut s, i| { let _ = writeln!(s, "line {i} /home/jsmith/f{i} a{i}@example.com"); s });
    let lines = log_lines(&raw, &who());
    assert_eq!(lines.len(), LOG_LINES);
    assert!(lines[0].starts_with("line 300 ~/f300 [email]"), "{}", lines[0]);
    assert!(lines.iter().all(|l| !l.contains("jsmith") && !l.contains('@')));
}

fn parts<'a>(mode: Mode, words: &'a str, hardware: &'a str, crash: Option<&'a Crash>, log: &'a [String]) -> Parts<'a> {
    Parts { mode, version: "1.2", words, hardware, crash, log, username: " jsmith2 ", github: " jsmith-gh " }
}

#[test]
fn a_report_holds_only_the_fixed_list_and_each_part_is_blanked() {
    let log = vec!["one 10.0.0.9".to_string(), "two".to_string()];
    let crash = Crash { program: "trindle-ring".into(), signal: "11 (SEGV)".into(), trace: "at /home/jsmith/x aa:bb:cc:dd:ee:ff".into() };
    let text = document(&parts(Mode::Crash, "It froze for John at jsmith@example.com", "Computer: Acme Box on laptop-7", Some(&crash), &log), &who());
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let mut got: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
    let mut want = FIELDS.to_vec();
    want.sort();
    got.sort();
    assert_eq!(got, want, "nothing beyond the fixed list, and every field of it filled here");
    assert_eq!(want, ["crash", "github", "hardware", "kind", "log", "project", "username", "version", "words"],
               "the server refuses the whole report for any other field: adding one here is a mistake");
    let mut c: Vec<&str> = v["crash"].as_object().unwrap().keys().map(String::as_str).collect();
    c.sort();
    assert_eq!(c, CRASH_FIELDS);
    assert_eq!((v["project"].as_str(), v["kind"].as_str(), v["version"].as_str()), (Some("trindle"), Some("crash"), Some("1.2")));
    assert_eq!(v["words"], "It froze for [name] at [email]");
    assert_eq!(v["hardware"], "Computer: Acme Box on [name]");
    assert_eq!(v["crash"]["trace"], "at ~/x [mac]");
    assert_eq!(v["log"], serde_json::json!(["one [ip]", "two"]));
    assert_eq!(v["username"], "jsmith2", "the username is the person's own, trimmed");
    assert_eq!(v["github"], "jsmith-gh");

    let only = Crash { program: "trindle-ring".into(), signal: String::new(), trace: "  ".into() };
    let o: serde_json::Value = serde_json::from_str(&document(&parts(Mode::Crash, "", "", Some(&only), &[]), &who())).unwrap();
    assert_eq!(o["crash"], serde_json::json!({ "program": "trindle-ring" }));

    let bare = Parts { mode: Mode::Note, version: "1.2", words: "  ", hardware: "Computer: x", crash: None, log: &[], username: "", github: "" };
    let b: serde_json::Value = serde_json::from_str(&document(&bare, &who())).unwrap();
    let mut keys: Vec<&str> = b.as_object().unwrap().keys().map(String::as_str).collect();
    keys.sort();
    assert_eq!(keys, ["hardware", "kind", "project", "version"]);
    let d: serde_json::Value = serde_json::from_str(&document(&parts(Mode::Note, "x", "h", None, &[]), &who())).unwrap();
    assert_eq!((d["kind"].as_str(), d["crash"].is_null()), (Some("note"), true));
}

#[test]
fn the_crash_and_its_log_are_read_from_systemd_and_nothing_else() {
    let list = r#"[{"time":1791000000000000,"pid":10,"exe":"/usr/bin/other"},
        {"time":1791000100000000,"pid":11,"exe":"/usr/bin/trindle-ring"},
        {"time":1791000200000000,"pid":12,"exe":"/usr/bin/trindle-settings"},
        {"time":1791000300000000,"pid":13,"exe":"/usr/bin/firefox"}]"#;
    assert_eq!(probe::newest(list), Some((12, "trindle-settings".to_string(), 1_791_000_200)));
    assert_eq!(probe::newest("[]"), None);
    assert_eq!(probe::newest("not json"), None);
    let info = "           PID: 12\n        Signal: 11 (SEGV)\n       Message: Process 12 crashed\n\nStack trace of thread 12:\n#0  0x1 main\n#1  0x2 start\n";
    let (signal, trace) = probe::parse_info(info);
    assert_eq!(signal, "11 (SEGV)");
    assert!(trace.starts_with("Stack trace of thread:") && trace.contains("#1  start") && !trace.contains("PID"));
    let again = "        Signal: 11 (SEGV)\n\nStack trace of thread 9031:\n#0  0x00007f11aa00 main (trindle + 0x1a2b)\n#1  0x00007f11bb00 start (libc.so.6 + 0x2c3)\n";
    let first_run = "        Signal: 11 (SEGV)\n\nStack trace of thread 77:\n#0  0x000055cc01aa main (trindle + 0x1a2b)\n#1  0x000055cc02bb start (libc.so.6 + 0x2c3)\n";
    assert_eq!(probe::parse_info(again), probe::parse_info(first_run), "the same crash must read the same each time");
    assert!(probe::parse_info(again).1.contains("#0  main (trindle + 0x1a2b)"));
    assert!(probe::hardware_text("Acme Box", None).starts_with("Acme Box") && !probe::hardware_text("Acme Box", None).contains('\n'));
    let whole = probe::hardware_text("Acme Box", Some("Processor: Acme 9\nMemory: 16000 MB"));
    let first = whole.lines().next().unwrap();
    assert!(first.starts_with("Acme Box, Acme 9, 16 GB"), "{first}");
    assert!(whole.ends_with("Memory: 16000 MB"));

    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
    assert!(probe::before_this_boot(now - 10_000, 600));
    assert!(!probe::before_this_boot(now - 100, 600));
}

#[test]
fn usernames_and_github_names_are_checked_with_their_reasons() {
    assert_eq!(name_fault("john.smith@example.com"), Some(no_at()));
    assert_eq!(name_fault("a@b"), Some(no_at()));
    assert_eq!(name_fault("has space"), Some(name_chars()));
    assert_eq!(name_fault(""), None);
    assert_eq!(name_fault("john-smith-2"), None);
    assert_eq!(name_fault("john_smith"), Some(name_chars()), "Buildmorphic names have no underscore");
    assert_eq!(name_fault("a"), Some(name_chars()), "at least two letters");
    assert_eq!(name_fault(&"a".repeat(32)), Some(name_chars()), "at most 31");
    assert_eq!(name_fault("John"), Some(name_chars()), "lower case only");
    assert_eq!(github_fault(""), None);
    assert_eq!(github_fault("john-smith2"), None);
    assert!(github_fault("@john").is_some() && github_fault("has space").is_some() && github_fault("a.b").is_some());
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("trindle-report-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn files(dir: &Path) -> usize {
    std::fs::read_dir(dir).map(|d| d.filter(|f| f.as_ref().is_ok_and(|f| f.path().is_file())).count()).unwrap_or(0)
}

fn ready(name: &str, mode: Mode) -> Reporter {
    let mut r = Reporter::new(who(), "Acme Box".into(), temp(name));
    r.mode = mode;
    r.program = "trindle-ring".into();
    r.log = Some(vec!["a line".into()]);
    r.typed = "The ring did not open".into();
    r.switch = true;
    r.gate = Gate::Open;
    r
}

#[test]
fn nothing_is_sent_before_send_and_send_goes_once_with_the_exact_text_shown() {
    let mut r = ready("send", Mode::Crash);
    r.update(Msg::Typed("It crashed".into()));
    r.update(Msg::Details);
    let _ = r.view();
    r.update(Msg::Copy);
    assert!(r.outbox.is_empty(), "opening, typing and looking sent something");
    r.update(Msg::Send);
    assert_eq!(r.outbox.len(), 1);
    assert_eq!(r.outbox[0], r.text(), "what is sent is byte for byte what is shown");
    r.update(Msg::Send);
    assert_eq!(r.outbox.len(), 1, "only once per window");
    assert_eq!(r.refusal, already_sent());

    let mut q = ready("dont", Mode::Crash);
    q.update(Msg::Cancel);
    assert!(q.outbox.is_empty() && !q.dir.exists() && !q.waiting_dir.exists());
}

#[test]
fn each_window_says_what_it_is_and_the_box_is_optional() {
    for (mode, title) in [(Mode::Crash, title_crash()), (Mode::Note, title_note())] {
        let r = ready("modes", mode);
        assert!(r.view().said().iter().any(|s| s == title), "{title}");
        assert_eq!(r.refused(), None, "nothing has to be written");
    }
    let c = ready("crashdoc", Mode::Crash);
    let t: serde_json::Value = serde_json::from_str(&c.text()).unwrap();
    assert_eq!(t["crash"]["program"], "trindle-ring");
    assert!(summary().contains("File names and web addresses can still be in the log lines. It does not send your files or passwords"));
}

#[test]
fn the_mode_comes_from_the_arguments_and_a_stranger_is_not_a_program() {
    let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(mode_of(&a(&["trindle-report"])).0, Mode::Note);

    assert_eq!(mode_of(&a(&["trindle-report", "--hardware"])).0, Mode::Note);
    assert_eq!(mode_of(&a(&["trindle-report", "--crash", "trindle-ring"])), (Mode::Crash, "trindle-ring".to_string()));
    assert_eq!(mode_of(&a(&["trindle-report", "--crash", "/bin/evil"])), (Mode::Crash, "trindle".to_string()));
}

#[test]
fn the_window_follows_the_persons_language_and_falls_back_to_english_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

    let chosen = Bundle::from_text("xx", "report-send = Envoyer\nreport-title-note = Signaler un souci\nreport-keep-failed = Pas gardé: { $reason }.\n");
    let (said, title, failed) = with_catalogue(Catalogue::new(chosen, english()), || {
        let r = ready("language", Mode::Note);
        let failed = t!("report-keep-failed", reason = reason(&std::io::Error::from(std::io::ErrorKind::NotFound)));
        (r.view().said(), title_note(), failed)
    });
    assert!(said.iter().any(|s| s.contains("Envoyer")), "{said:?}");
    assert_eq!(title, "Signaler un souci");
    assert_eq!(failed, "Pas gardé: the folder is missing.", "a reason with no translation stays English");
    assert!(said.iter().any(|s| s.contains("Cancel")), "a word not translated shows in English: {said:?}");
}

#[test]
fn save_works_once_and_holds_the_same_text() {
    let mut r = ready("save", Mode::Note);
    let dir = r.dir.clone();
    r.update(Msg::Typed("It crashed".into()));
    let _ = r.view();
    let _ = r.text();
    assert!(!dir.exists(), "something was written before Save");
    r.update(Msg::Save);
    assert_eq!(files(&dir), 1);
    assert!(r.saved.is_some() && r.refusal.is_empty());
    let kept = std::fs::read_to_string(std::fs::read_dir(&dir).unwrap().flatten().find(|f| f.path().is_file()).unwrap().path()).unwrap();
    assert_eq!(kept, r.text());
    r.update(Msg::Save);
    assert_eq!(files(&dir), 1);
    assert_eq!(r.refusal, already());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_that_cannot_be_written_says_why_and_can_be_tried_again() {
    let dir = temp("blocked");
    std::fs::write(&dir, "a file where the folder should be").unwrap();
    let mut r = Reporter::new(who(), "Acme Box".into(), dir.clone());
    r.update(Msg::Save);
    assert!(r.refusal.starts_with("Could not save it: "), "{}", r.refusal);
    assert!(r.saved.is_none());
    let _ = std::fs::remove_file(&dir);
}

#[test]
fn the_hardware_summary_is_own_and_holds_nothing_personal() {
    assert_eq!(probe::graphics_line("0x8086", "0x9a49", "i915"), "Graphics: Intel 0x9a49 (driver i915)");
    assert_eq!(probe::graphics_line("", "", ""), "Graphics: unknown maker");
    let s = probe::hardware_summary();
    assert!(s.chars().count() <= 20_000);
    assert!(!s.contains("/home"));
    assert!(!include_str!("../../packages.txt").lines().any(|l| l.starts_with("hw-probe")));
}

#[test]
fn in_the_beta_reports_are_asked_about_until_the_person_says_no() {
    assert_eq!(crate::common::files::default_setting(OFFER_KEY), "on", "the beta's one place to flip at the full release");
    assert!(offered() || crate::common::files::setting(OFFER_KEY) == "off", "on unless the person chose off");
}

#[test]
fn the_shortcut_for_reports_is_kept_in_settings_and_starts_the_report_window() {
    use crate::common::combos;
    assert!(combos::SETTING_KEYS.contains(&combos::REPORT_KEYS));
    assert_eq!(combos::REPORT_COMMAND, "trindle-report");
    assert_eq!(crate::common::files::default_setting(combos::REPORT_KEYS), "Ctrl+Alt+R");
    assert!(crate::NAMES.contains(&combos::REPORT_COMMAND));
}

#[test]
fn a_log_with_no_lines_of_its_own_is_read_from_trindles_units() {
    assert_eq!(super::first_with_lines("one\n".into(), || panic!("not read")), "one\n");
    assert_eq!(super::first_with_lines(String::new(), || "unit line".into()), "unit line");
    assert_eq!(super::first_with_lines(" \n".into(), || "unit line".into()), "unit line");
}
