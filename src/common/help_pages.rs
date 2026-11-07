//! Test only.

use std::path::Path;
use trindle_help::CONTENT;

pub fn page(id: &str) -> String {
    let path = Path::new(CONTENT).join(format!("{id}.md"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Help has no page {id}: {e}"))
}

fn slug(heading: &str) -> String {
    let mut out = String::new();
    let mut gap = false;
    for c in heading.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('-');
            }
            gap = false;
            out.push(c);
        } else {
            gap = true;
        }
    }
    out
}

pub fn section(address: &str) -> Option<String> {
    let (id, part) = address.split_once('#')?;
    let raw = std::fs::read_to_string(Path::new(CONTENT).join(format!("{id}.md"))).ok()?;
    let mut found: Option<Vec<&str>> = None;
    for line in raw.lines() {
        let heading = line.strip_prefix("### ").or_else(|| line.strip_prefix("## "));
        match (heading, &mut found) {
            (Some(h), None) if slug(h) == part => found = Some(vec![line]),
            (Some(_), Some(_)) => break,
            (_, Some(lines)) => lines.push(line),
            _ => {}
        }
    }
    found.map(|l| l.join("\n"))
}

pub fn classify(text: &str) -> Option<String> {
    trindle_help::classify(text).map(str::to_string)
}

#[test]
fn a_section_is_found_by_its_address_and_a_missing_one_is_not() {
    assert!(section("errors#the-package-database-is-locked").is_some_and(|s| s.contains("Questions:")));
    assert!(section("errors#no-such-section").is_none());
    assert_eq!(classify("E: Could not get lock /var/lib/dpkg/lock-frontend").as_deref(), Some("errors#the-package-database-is-locked"));
    assert_eq!(classify("nothing in particular"), None);
}
