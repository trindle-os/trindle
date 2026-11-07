//! A plugin's short codes, read as hostile.

use std::path::Path;

pub const WORD: &str = "expand";
pub const FILE: &str = "expansions.json";
pub const MOST_BYTES: u64 = 9 * 1024 * 1024;
pub const MOST_ENTRIES: usize = 2000;
pub const SHORTEST_TRIGGER: usize = 3;
pub const LONGEST_TRIGGER: usize = 32;
pub const LONGEST_WORDS: usize = 4000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expansion {
    pub trigger: String,
    pub words: String,
    pub word: bool,
    pub case: bool,
}

pub fn hidden(c: char) -> bool {
    let n = c as u32;
    c.is_control()
        || matches!(n, 0xad | 0x34f | 0x61c | 0x115f | 0x1160 | 0x17b4 | 0x17b5 | 0x180b..=0x180f | 0x200b..=0x200f | 0x202a..=0x202e
            | 0x2028 | 0x2029 | 0x2060..=0x206f | 0x2800 | 0x3164 | 0xfe00..=0xfe0f | 0xfeff | 0xffa0 | 0xfff9..=0xfffb | 0xe000..=0xf8ff
            | 0xe0000..=0xe007f | 0xf0000..=0x10ffff | 0x1d173..=0x1d17a)
        || (n & 0xfffe) == 0xfffe
        || (0xfdd0..=0xfdef).contains(&n)
}

fn good_trigger(t: &str) -> bool {
    let n = t.chars().count();
    (SHORTEST_TRIGGER..=LONGEST_TRIGGER).contains(&n) && !t.chars().any(|c| c.is_whitespace() || hidden(c))
}

pub fn hides_in_words(c: char) -> bool {
    c.is_control() || (hidden(c) && !matches!(c, '\u{200C}' | '\u{200D}' | '\u{FE00}'..='\u{FE0F}'))
}

pub fn good_words(w: &str) -> bool {
    !w.is_empty() && w.len() <= LONGEST_WORDS && !w.chars().any(hides_in_words)
}

pub fn parse(text: &[u8]) -> Vec<Expansion> {
    let Ok(v) = serde_json::from_slice::<serde_json::Value>(text) else { return Vec::new() };
    if v.get("version").and_then(|n| n.as_u64()) != Some(1) {
        return Vec::new();
    }
    let Some(list) = v.get("snippets").and_then(|s| s.as_array()) else { return Vec::new() };
    let flag = |e: &serde_json::Value, k: &str| match e.get(k) {
        None => Some(false),
        Some(b) => b.as_bool(),
    };
    list.iter()
        .take(MOST_ENTRIES)
        .filter_map(|e| {
            let trigger = e.get("trigger")?.as_str()?;
            let words = e.get("replace")?.as_str()?;
            let (word, case) = (flag(e, "word")?, flag(e, "case")?);
            (good_trigger(trigger) && good_words(words))
                .then(|| Expansion { trigger: trigger.into(), words: words.into(), word, case })
        })
        .collect()
}

pub fn stamp(folder: &Path) -> Option<(u64, std::time::SystemTime)> {
    let home = folder.join("home");
    if !std::fs::symlink_metadata(&home).is_ok_and(|m| m.is_dir()) {
        return None;
    }
    let m = std::fs::symlink_metadata(home.join(FILE)).ok()?;
    (m.is_file() && m.len() <= MOST_BYTES).then(|| (m.len(), m.modified().unwrap_or(std::time::UNIX_EPOCH)))
}

pub fn read(folder: &Path) -> Vec<Expansion> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    if stamp(folder).is_none() {
        return Vec::new();
    }
    let opened = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(folder.join("home").join(FILE));
    let Ok(file) = opened else { return Vec::new() };
    if !file.metadata().is_ok_and(|m| m.is_file() && m.len() <= MOST_BYTES) {
        return Vec::new();
    }
    let mut text = Vec::new();
    if file.take(MOST_BYTES + 1).read_to_end(&mut text).is_err() || text.len() as u64 > MOST_BYTES {
        return Vec::new();
    }
    parse(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(entry: &str) -> Vec<Expansion> {
        parse(format!(r#"{{"version":1,"snippets":[{entry}]}}"#).as_bytes())
    }

    #[test]
    fn only_plain_entries_within_the_limits_count() {
        assert_eq!(one(r#"{"trigger":"tn#","replace":"Trindle","word":false,"case":true}"#),
                   [Expansion { trigger: "tn#".into(), words: "Trindle".into(), word: false, case: true }]);
        assert_eq!(one(r#"{"trigger":"tn#","replace":"Trindle"}"#).len(), 1, "word and case may be left out");
        for bad in [
            r#"{"trigger":"tn","replace":"x"}"#,
            r#"{"trigger":"a b#","replace":"x"}"#,
            r#"{"trigger":"ab\u0007","replace":"x"}"#,
            r#"{"trigger":"ab\u200bc","replace":"x"}"#,
            r#"{"trigger":"ab\u200dc","replace":"x"}"#,
            r#"{"trigger":"a\u00adbc","replace":"x"}"#,
            r#"{"trigger":"ab\ufeffc","replace":"x"}"#,
            r#"{"trigger":"tn#","replace":""}"#,
            r#"{"trigger":"tn#","replace":"rm -rf ~\n"}"#,
            r#"{"trigger":"tn#","replace":"a\tb"}"#,
            r#"{"trigger":"tn#","replace":"\u001b[2J"}"#,
            "{\"trigger\":\"tn#\",\"replace\":\"abc\\u202edcba\"}",
            r#"{"trigger":"tn#","replace":"x","word":"yes"}"#,
            r#"{"trigger":7,"replace":"x"}"#,
            "{\"trigger\":\"tn#\",\"replace\":\"a\\u200bb\"}",
            "{\"trigger\":\"tn#\",\"replace\":\"a\\u2028b\"}",
            "{\"trigger\":\"tn#\",\"replace\":\"a\\u2029b\"}",
            "{\"trigger\":\"tn#\",\"replace\":\"a\\u2060b\"}",
            "{\"trigger\":\"tn#\",\"replace\":\"a\\ufeffb\"}",
            "{\"trigger\":\"tn#\",\"replace\":\"a\\udb40\\udc41b\"}",
            "{\"trigger\":\"t\\u200bn#\",\"replace\":\"x\"}",
            "{\"trigger\":\"t\\u200dn#\",\"replace\":\"x\"}",
        ] {
            assert!(one(bad).is_empty(), "{bad}");
        }
        assert_eq!(one("{\"trigger\":\"tn#\",\"replace\":\"\u{1F468}\\u200d\u{1F469}\"}").len(), 1);
        assert!(one(&format!(r#"{{"trigger":"{}","replace":"x"}}"#, "a".repeat(33))).is_empty());
        assert!(one(&format!(r#"{{"trigger":"tn#","replace":"{}"}}"#, "a".repeat(4001))).is_empty());
        assert_eq!(one(&format!(r#"{{"trigger":"tn#","replace":"{}"}}"#, "é".repeat(2000))).len(), 1);
        assert!(parse(br#"{"version":2,"snippets":[{"trigger":"tn#","replace":"x"}]}"#).is_empty());
        assert!(parse(b"not json").is_empty());
        let many: Vec<String> = (0..2100).map(|i| format!(r#"{{"trigger":"t{i:04}","replace":"x"}}"#)).collect();
        assert_eq!(one(&many.join(",")).len(), MOST_ENTRIES);
    }

    #[test]
    fn a_link_or_a_huge_file_is_never_read() {
        let dir = std::env::temp_dir().join(format!("trindle-expand-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let good = br#"{"version":1,"snippets":[{"trigger":"tn#","replace":"x"}]}"#;
        let (plain, linked, home_linked, big) = (dir.join("plain"), dir.join("linked"), dir.join("home-linked"), dir.join("big"));
        for d in [&plain, &linked, &big] {
            std::fs::create_dir_all(d.join("home")).unwrap();
        }
        std::fs::create_dir_all(&home_linked).unwrap();
        std::fs::write(plain.join("home").join(FILE), good).unwrap();
        assert_eq!(read(&plain).len(), 1);
        std::os::unix::fs::symlink(plain.join("home").join(FILE), linked.join("home").join(FILE)).unwrap();
        assert!(read(&linked).is_empty() && stamp(&linked).is_none());
        std::os::unix::fs::symlink(plain.join("home"), home_linked.join("home")).unwrap();
        assert!(read(&home_linked).is_empty());
        let f = std::fs::File::create(big.join("home").join(FILE)).unwrap();
        f.set_len(MOST_BYTES + 1).unwrap();
        assert!(read(&big).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
