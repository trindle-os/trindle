//! How old an app is, as its page and each result say it.

use crate::toolkit::lang::format::{date, Pattern};
use crate::toolkit::t;
use crate::toolkit::widget::{note, Node, Tone};

pub const STALE: u64 = 183 * 24 * 3600;

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

pub fn day(unix: u64) -> String {
    date(&crate::common::clock::moment_at(unix as i64, 0), Pattern::DayLongYear)
}

pub fn stale(updated: Option<u64>, now: u64) -> bool {
    updated.is_some_and(|u| now.saturating_sub(u) > STALE)
}

pub fn updated_words(updated: Option<u64>, now: u64) -> (String, bool) {
    match updated {
        None => (t!("store-age-updated-unknown").into(), false),
        Some(u) if stale(Some(u), now) => (t!("store-age-updated-old", date = day(u)), true),
        Some(u) => (t!("store-age-updated", date = day(u)), false),
    }
}

pub fn updated<M>(updated: Option<u64>, now: u64) -> Node<M> {
    let (words, red) = updated_words(updated, now);
    let line = note(words).cut();
    if red { line.tone(Tone::Problem) } else { line }
}

pub fn created_words(created: Option<u64>) -> String {
    created.map_or_else(|| t!("store-age-created-unknown").into(), |c| t!("store-age-created", date = day(c)))
}

pub fn day_or_not_known(when: Option<u64>) -> String {
    when.map_or_else(|| t!("store-age-not-known").into(), day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_months_without_an_update_is_said_in_red() {
        let now = 1_800_000_000;
        assert!(!updated_words(Some(now - STALE), now).1);
        let (words, red) = updated_words(Some(now - STALE - 1), now);
        assert!(red && words.contains("security"), "{words}");
        assert_eq!(updated_words(None, now), ("Last updated: not known".to_string(), false));
        assert_eq!(updated_words(Some(86_400 * 59), now).0.split('.').next(), Some("Last updated 1 March 1970"));
        assert_eq!(created_words(Some(0)), "Created 1 January 1970");
        assert_eq!((created_words(None).as_str(), day_or_not_known(None).as_str()), ("Created: not known", "Not known"));
    }
}
