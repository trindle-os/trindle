//! The one way any Trindle program offers a "More" beside a failure message.

use crate::toolkit::t;
use crate::toolkit::widget::{button as round, Node};

const PROGRAM: &str = "trindle-help";

pub fn valid(address: &str) -> bool {
    let word = |w: &str| !w.is_empty() && !w.starts_with('-') && w.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    match address.split_once('#') {
        Some((page, section)) => word(page) && word(section),
        None => word(address),
    }
}

pub fn open(address: &str) -> bool {
    valid(address) && super::run::spawn(&[PROGRAM, address])
}

pub const ERROR_LIMIT: usize = 4000;

pub fn clean_error(text: &str) -> String {
    let mut out: String = text.chars().map(|c| if c == '\n' || !super::expand::hides_in_words(c) { c } else { ' ' }).collect();
    if out.len() > ERROR_LIMIT {
        let mut at = ERROR_LIMIT;
        while !out.is_char_boundary(at) {
            at -= 1;
        }
        out.truncate(at);
    }
    out.trim().to_string()
}

pub fn open_with(address: &str, error: &str) -> bool {
    match help_argv(address, error) {
        Some(argv) => super::run::spawn(&argv.iter().map(String::as_str).collect::<Vec<_>>()),
        None => false,
    }
}

fn help_argv(address: &str, error: &str) -> Option<Vec<String>> {
    if !valid(address) {
        return None;
    }
    let error = clean_error(error);
    let mut argv = vec![PROGRAM.to_string(), address.to_string()];
    if !error.is_empty() {
        argv.push(error);
    }
    Some(argv)
}

pub fn button<M>(press: M) -> Node<M> {
    round("help-browser-symbolic", t!("common-more")).spoken(t!("common-more-spoken")).on_press(press)
}

pub fn fixed_part(key: &str, args: &[&str]) -> String {
    use crate::toolkit::lang::{t_with, Arg};
    const MARK: &str = "\u{1}";
    let given: Vec<(&str, Arg)> = args.iter().map(|a| (*a, Arg::from(MARK))).collect();
    let words = t_with(key, &given);
    words.split(MARK).map(|p| p.trim_matches(|c: char| c.is_whitespace() || ".,:;".contains(c))).find(|p| !p.is_empty()).unwrap_or("").to_string()
}

#[cfg(test)]
pub type Entry = (&'static str, &'static str);

#[cfg(test)]
pub const TABLES: &[(&str, &[Entry])] = &[
    ("Apps", crate::store::help::MESSAGES),
    ("Drives", crate::disks::help::MESSAGES),
    ("Plugins", crate::plugins::help::MESSAGES),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::help_pages;

    fn keys() -> std::collections::HashSet<String> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang/en");
        let mut out = std::collections::HashSet::new();
        for f in std::fs::read_dir(dir).unwrap().flatten() {
            let text = std::fs::read_to_string(f.path()).unwrap_or_default();
            for line in text.lines() {
                if let Some((k, _)) = line.split_once(" =") {
                    if !k.starts_with(|c: char| c == '#' || c.is_whitespace()) {
                        out.insert(k.to_string());
                    }
                }
            }
        }
        out
    }

    #[test]
    fn only_a_page_and_a_section_are_handed_to_help() {
        for good in ["folders", "folders#archive-password", "a-b#c-1"] {
            assert!(valid(good), "{good}");
        }
        for bad in ["", "#x", "x#", "-x", "x#-y", "--help", "Folders", "a b", "a/b", "a#b#c", "a;b", "../x", "x\n"] {
            assert!(!valid(bad), "{bad:?}");
        }
        assert!(!open("--version"));
    }

    #[test]
    fn every_message_with_a_more_names_an_address_that_has_a_page() {
        let keys = keys();
        for (program, table) in TABLES {
            assert!(!table.is_empty(), "{program} has an empty table");
            let mut seen = std::collections::HashSet::new();
            for (key, address) in *table {
                assert!(seen.insert(key), "{program}: {key} is listed twice");
                assert!(keys.contains(*key), "{program}: {key} is not in the word files");
                assert!(valid(address), "{program}: {key} names {address:?}, which Help cannot be given");
                assert!(help_pages::section(address).is_some(), "{program}: {key} names {address}, which has no section in the pages");
            }
        }
    }

    #[test]
    fn the_sections_failures_point_to_are_thorough() {
        for (program, table) in TABLES {
            for (key, address) in *table {
                let section = help_pages::section(address).unwrap_or_else(|| panic!("{program}: {address} ({key}) has no section"));
                let questions: usize = section.lines().filter_map(|l| l.strip_prefix("Questions:")).map(|q| q.split(';').filter(|x| !x.trim().is_empty()).count()).sum();
                assert!(questions >= 3, "{program}: {address} ({key}) has {questions} questions");
                assert!(!section.contains('\u{2014}') && !section.contains('\u{2013}'), "{address} has a dash");
                assert!(section.contains("<!-- screenshot: ") || section.contains("!["), "{address} has no picture and names none to take");
            }
        }
    }

    #[test]
    fn an_error_text_is_cleaned_and_cut_before_help_is_given_it() {
        assert_eq!(clean_error("a\u{7}b\nc\u{1b}[0m "), "a b\nc [0m");
        assert_eq!(clean_error("  \n "), "");
        assert_eq!(clean_error("rm\u{202e}fi\u{200b}le"), "rm fi le");
        assert_eq!(help_argv("folders", "bad\u{202e}\u{7}words").unwrap(), [PROGRAM, "folders", "bad  words"]);
        assert_eq!(help_argv("folders", " \u{200b} ").unwrap(), [PROGRAM, "folders"]);
        assert_eq!(help_argv("--version", "words"), None);
        let cut = clean_error(&"\u{e9}".repeat(ERROR_LIMIT));
        assert!(cut.len() <= ERROR_LIMIT && cut.chars().all(|c| c == '\u{e9}'));
        assert!(!open_with("--version", "words"));
        assert!(!open_with("--version", ""));
    }

    #[test]
    fn the_button_says_more_and_is_pressable() {
        let b: Node<u8> = button(1);
        let said = b.said();
        assert_eq!(said, vec![format!("button: {}", t!("common-more"))]);
    }
}
