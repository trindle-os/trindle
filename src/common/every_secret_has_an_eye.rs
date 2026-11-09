//! Every box a password, a passphrase, a PIN or a key is typed in has the
//! eye that shows what is typed, and the toolkit draws it: a page makes a
//! secret box with `.secret()` and gets the eye with it. This walks the
//! source so a new password box cannot be added without one:

use std::path::{Path, PathBuf};

const READ_ON_PURPOSE: [(&str, &str); 3] = [

    ("src/common/words_tests.rs", "administrator's password"),

    ("src/toolkit/widget/tests.rs", "entry(\"hunter2\", \"Password\""),

    ("src/settings/panels/bluetooth.rs", "\"PIN\""),
];

const OWN_DRAWN: [&str; 2] = ["src/session/lock/reveal.rs", "src/unlock/main.rs"];

fn sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(list) = std::fs::read_dir(dir) else { return };
        for entry in list.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for (tree, prefix) in crate::common::source::trees() {
        let mut found = Vec::new();
        walk(&tree, &mut found);
        files.extend(found.into_iter().filter_map(|p| Some((format!("src/{prefix}{}", p.strip_prefix(&tree).ok()?.to_string_lossy()), p))));
    }
    files.sort();

    files.into_iter().filter_map(|(name, p)| {
        Some((name, std::fs::read_to_string(&p).ok()?))
    }).filter(|(name, text)| name != "src/common/every_secret_has_an_eye.rs" && (!boxes(text).is_empty() || OWN_DRAWN.contains(&name.as_str())))
        .collect()
}

fn closes(text: &str, open: usize) -> usize {
    let bytes = text.as_bytes();
    let (mut deep, mut i, mut quoted) = (0i32, open, false);
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if quoted => i += 1,
            b'"' => quoted = !quoted,
            b'(' | b'[' | b'{' if !quoted => deep += 1,
            b')' | b']' | b'}' if !quoted => {
                deep -= 1;
                if deep == 0 {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    bytes.len().saturating_sub(1)
}

fn boxes(text: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    for (at, _) in text.match_indices("entry(") {

        if text[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == ':') {
            continue;
        }
        let end = closes(text, at + "entry".len());
        let mut chain = end + 1;
        loop {
            let rest = &text[chain..];
            let skip = rest.len() - rest.trim_start().len();
            let Some(method) = rest[skip..].strip_prefix('.') else { break };
            let name = method.chars().take_while(|c| c.is_alphanumeric() || *c == '_').count();
            if !method[name..].starts_with('(') {
                break;
            }
            chain = closes(text, chain + skip + 1 + name) + 1;
        }
        out.push((&text[at..=end], &text[end + 1..chain]));
    }
    out
}

fn quoted(code: &str) -> Vec<&str> {
    code.split('"').skip(1).step_by(2).collect()
}

fn names_a_secret(hint: &str) -> bool {
    let low = hint.to_lowercase();
    let word = |w: &str| hint.split(|c: char| !c.is_alphanumeric()).any(|part| part == w);
    low.contains("password") || low.contains("passphrase") || low.contains("passcode") || word("PIN")
}

fn makes_secret(code: &str) -> bool {
    code.contains(".secret()") || code.contains(".secret_of(")
}

#[test]
fn a_box_and_its_chain_are_read_from_the_source() {
    let text = "let a = entry(self.pw.clone(), \"New password\", |t| go(t))\n    .key(\"pw\")\n    .secret()\n    .on_submit(go());\n\
                let b = plan::entry(x); m.entry(k).or_default(); entry(n, \"Name\", f).width(Length::Fill)";
    let found = boxes(text);
    assert_eq!(found.len(), 2);
    assert!(found[0].0.contains("New password") && found[0].1.contains(".secret()") && found[0].1.ends_with(".on_submit(go())"));
    assert!(found[1].0.contains("Name") && found[1].1 == ".width(Length::Fill)");
    assert!(names_a_secret("New password") && names_a_secret("Disk passphrase") && names_a_secret("New PIN") && names_a_secret("PIN"));
    assert!(!names_a_secret("Name") && !names_a_secret("Pinned notes") && !names_a_secret("Paste a checksum to check"));
}

#[test]
fn every_box_that_takes_a_secret_is_made_secret_and_so_has_the_eye() {
    let mut bare = Vec::new();
    for (name, text) in sources() {
        for (call, chain) in boxes(&text) {
            let hints: Vec<&str> = quoted(call).into_iter().filter(|h| names_a_secret(h)).collect();
            if hints.is_empty() || makes_secret(chain) {
                continue;
            }
            let meant = READ_ON_PURPOSE.iter().any(|(file, hint)| *file == name && call.contains(hint));
            if !meant {
                bare.push(format!("{name}: {}", hints.join(", ")));
            }
        }
    }
    assert!(bare.is_empty(), "a box that takes a secret is not made with .secret(), so has no eye: {bare:?}");
}

#[test]
fn no_page_keeps_a_show_button_of_its_own() {
    let mut found = Vec::new();
    for (name, text) in sources() {
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !makes_secret(line) {
                continue;
            }

            let code = line.trim_start();
            let before = lines[..i].iter().rev().map(|l| l.trim()).find(|l| !l.is_empty()).unwrap_or("");
            let chosen = code.contains("if ") || code.contains("else") || before.ends_with("else {")

                || (code.contains(" = ") && !code.starts_with("let ") && before.starts_with("if ") && before != "if secret {");
            if chosen {
                found.push(format!("{name}:{}: {code}", i + 1));
            }
        }

        if !name.starts_with("src/toolkit/") {
            for said in ["Show the password", "Hide the password", "Show the PIN", "Hide the PIN", "Show the key", "Hide the key",
                         "Show password", "Hide password"] {
                if text.contains(said) {
                    found.push(format!("{name}: writes \"{said}\" itself"));
                }
            }
        }
    }
    assert!(found.is_empty(), "the toolkit's eye is the only show button for a secret: {found:?}");
}

#[test]
fn every_secret_box_has_a_name_for_its_eye_to_be_kept_under() {
    let mut nameless = Vec::new();
    for (name, text) in sources() {
        if name.starts_with("src/toolkit/") {

            continue;
        }
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !makes_secret(line) {
                continue;
            }

            let from = i.saturating_sub(6);
            let to = (i + 3).min(lines.len());
            if !lines[from..to].iter().any(|l| l.contains(".key(")) {
                nameless.push(format!("{name}:{}", i + 1));
            }
        }
    }
    assert!(nameless.is_empty(), "a secret box needs a key, so the toolkit can keep its eye by name: {nameless:?}");
}

#[test]
fn the_screens_that_draw_their_own_password_box_keep_the_shared_rule() {
    let all = sources();
    for own in OWN_DRAWN {
        let text = &all.iter().find(|(name, _)| name == own).unwrap_or_else(|| panic!("{own} has gone")).1;
        assert!(text.contains("eye::rested("), "{own} does not hide a shown password after the shared half minute");
    }
}
