//! The EULA: its version, its text, the installer's record of it, and the tick that asks.

use crate::common::clock::civil;
use crate::toolkit::t;
use crate::toolkit::widget::{form_label, heading, label, rich, row, scroll, tick, column, Length, Node};

pub const VERSION: u32 = 1;

pub const SYSTEM_RECORD: &str = "/var/lib/trindle/terms-accepted";
const TEXT: &str = include_str!("../../share/terms.md");

pub fn record_text(version: u32, date: &str) -> String {
    format!("version={version}\ndate={date}\n")
}

pub fn today() -> String {
    let days = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() / 86_400);
    let (y, m, d) = civil(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

fn paragraphs<M: 'static>() -> Vec<Node<M>> {
    TEXT.split("\n\n").map(str::trim).filter(|p| !p.is_empty()).map(|p| {
        match p.strip_prefix("### ").map(|h| (false, h)).or_else(|| p.strip_prefix("# ").map(|h| (true, h))) {
            Some((true, h)) => heading(h),
            Some((false, h)) => form_label(h),
            None => label(p).width(Length::Fill),
        }
    }).collect()
}

pub fn reading<M: 'static>() -> Node<M> {
    scroll("terms", column(paragraphs()).gap(12.0).padding(4.0)).height(Length::Fill)
}

pub fn tick_row<M: Clone + 'static>(on: bool, change: impl Fn(bool) -> M + 'static, open: M) -> Node<M> {
    let words = t!("terms-accept");
    let spoken = words.replace('[', "").replace("](eula)", "");
    row(vec![
        tick(on, spoken, change),
        rich(crate::toolkit::rich::parse(words)).on_link(move |_| open.clone()).width(Length::Fill),
    ]).gap(12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_is_the_technical_one_and_the_box_says_eula() {
        assert!(TEXT.contains("1. Parties") && TEXT.contains("the Trindle project") && !TEXT.contains("short version"));
        assert_eq!(t!("terms-accept"), "I agree to the [EULA](eula)");
    }

    #[test]
    fn every_language_links_the_word_to_the_text() {
        for dir in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/share/lang")).unwrap() {
            let file = dir.unwrap().path().join("terms.ftl");
            let text = std::fs::read_to_string(&file).unwrap();
            assert!(text.lines().any(|l| l.starts_with("terms-accept = ") && l.contains("](eula)")), "{file:?}");
        }
    }

    #[test]
    fn the_record_and_today_are_as_written() {
        assert_eq!(record_text(1, "2026-11-02"), "version=1\ndate=2026-11-02\n");
        let d = today();
        assert!(d.len() == 10 && d.starts_with("20") && d.as_bytes()[4] == b'-', "{d}");
    }
}
