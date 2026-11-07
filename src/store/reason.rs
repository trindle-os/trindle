//! The reason a command or Flatpak gave for a failure.

use crate::toolkit::t;

pub(super) fn last_line(err: &str, fallback: &str) -> String {
    err.trim().lines().last().unwrap_or(fallback).to_string()
}

pub(crate) fn flatpak_reason(said: &[String]) -> Option<String> {
    said.iter().rev().map(|l| l.trim()).find(|l| {
        let lower = l.to_lowercase();
        ["error", "warning", "info:", "end-of-life", "skipping", "not ", "nothing to do", "requires"]
            .iter().any(|w| lower.contains(w))
    }).map(String::from)
}

pub(super) fn why_not_updated(said: &[String]) -> String {
    match flatpak_reason(said) {
        Some(m) => t!("store-not-updated-said", why = m),
        None => t!("store-not-updated-none").to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_reason_is_the_last_message_not_the_progress() {
        let said: Vec<String> = ["Looking for updates…", "Updating 1/1… 100%",
            "Info: org.example.App is end-of-life, with reason: moved", "Updating 1/1… 100%"]
            .iter().map(|s| s.to_string()).collect();
        assert!(super::why_not_updated(&said).contains("end-of-life"));
        assert!(super::why_not_updated(&[]).contains("no reason"));
    }
}
