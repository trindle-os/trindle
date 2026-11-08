//! The sentences the Updates page says: headline, card status, held back, and why a check or an update did not go through.

use crate::toolkit::t;
use crate::store::catalogue::REMOTE;
use crate::store::look::Dot;
use crate::store::plugin_page;
use crate::store::Store;
use crate::common::updating::{self};
use super::{APPS, FIRMWARE, Phase, SYSTEM, counts, flat_held, phase, unchecked};

pub(crate) fn held_words(have: Option<&str>, to: Option<&str>) -> String {
    let waits = updating::waiting_words(have, to);
    match held_version(have) {
        Some(v) => t!("store-upd-held-version-waits", have = v, waits = waits),
        None => t!("store-upd-held-waits", waits = waits),
    }
}

pub(crate) fn held_only(have: Option<&str>) -> String {
    match held_version(have) {
        Some(v) => t!("store-upd-held-version", have = v),
        None => t!("store-upd-held").to_string(),
    }
}

fn held_version(have: Option<&str>) -> Option<&str> {
    have.filter(|v| !v.trim().is_empty())
}

pub(super) fn held_with_nothing_newer(have: Option<&str>) -> String {
    match held_version(have) {
        Some(v) => t!("store-upd-held-version-nothing", have = v),
        None => t!("store-upd-held-nothing").to_string(),
    }
}

pub(super) fn from_words(origin: &str) -> String {
    if origin.is_empty() || origin == REMOTE { "Flathub".into() } else { origin.to_string() }
}

fn count_words(n: usize) -> String {
    t!("store-upd-count", n = n)
}

pub(super) fn with_held(text: String, held: usize) -> String {
    if held == 0 {
        return text;
    }
    t!("store-upd-with-held", text = text, held = t!("store-upd-held-clause", n = held))
}

pub(super) fn found_sentence(n: usize, bytes: u64, restart: bool, held: usize) -> String {
    let mut text = t!("store-upd-ready-count", n = n);
    if bytes > 0 {
        text = t!("store-upd-in-all", count = text, size = crate::store::github::size_text(bytes));
    }
    let mut words = t!("store-upd-sentence", text = with_held(text, held));
    if restart {
        words = then(words, t!("store-upd-firmware-restart"));
    }
    words
}

fn waiting_bytes(s: &Store) -> u64 {
    let mut names: Vec<&str> = s.updatable.iter().filter(|id| !s.held.app(id)).map(String::as_str).collect();
    names.extend(s.debian.iter().map(String::as_str));
    if s.trindle_waiting.is_some() {
        names.push("trindle");
    }
    let fw: u64 = s.firmware.iter().filter(|f| !s.held.firmware(&f.id)).map(|f| f.size).sum();
    let parts: u64 = s.upd.flats.iter().filter(|f| !f.app && !flat_held(s, f)).map(|f| f.size).sum();
    names.iter().filter_map(|n| s.upd.sizes.get(*n)).sum::<u64>() + fw + parts
}

pub(super) fn then(first: String, second: &str) -> String {
    t!("store-upd-and-then", first = first, second = second)
}

pub(super) fn size_words(bytes: Option<u64>) -> String {
    bytes.filter(|b| *b > 0).map(crate::store::github::size_text).unwrap_or_default()
}

pub(super) fn headline(s: &Store) -> (Dot, String) {

    if let Some(who) = s.other {
        return (Dot::Checking, t!("store-installed-other-busy", who = who.sentence(), back = plugin_page::come_back()));
    }
    let (n, kept) = counts(s);
    match phase(s) {
        Phase::Checking => (Dot::Checking, t!("store-upd-checking").into()),
        Phase::Installing => {
            let share = overall(s);
            let words = match &s.upd.one {
                Some(name) => t!("store-upd-updating-one", name = name),
                None => t!("store-upd-installing-all").into(),
            };
            (Dot::Filling(share), words)
        }
        Phase::Restart => (Dot::Ready, t!("store-upd-restart").into()),

        Phase::Found if !s.online => (Dot::Waiting, then(found_sentence(n, waiting_bytes(s), false, kept), no_more())),
        Phase::Found => {
            let restart = s.firmware.iter().any(|f| !s.held.firmware(&f.id));
            let mut words = found_sentence(n, waiting_bytes(s), restart, kept);
            if s.upd.restart {
                words = then(words, t!("store-upd-gone-in-restart"));
            }
            (Dot::Waiting, words)
        }

        Phase::Done if kept > 0 && (!s.online || s.upd.failed.is_empty()) => {
            let none = t!("store-upd-sentence", text = with_held(t!("store-upd-none-ready").to_string(), kept));
            (Dot::Quiet, if s.online { none } else { then(none, no_more()) })
        }
        Phase::Done if !s.online => (Dot::Quiet, nothing_known().into()),
        Phase::Done if !s.upd.failed.is_empty() => {
            let names: Vec<String> = [(APPS, t!("store-upd-name-apps")), (SYSTEM, t!("store-upd-name-system")), (FIRMWARE, t!("store-upd-name-firmware"))].iter()
                .filter(|(k, _)| s.upd.failed.contains_key(k)).map(|(_, w)| w.to_string()).collect();
            (Dot::Failed, t!("store-upd-check-failed", names = crate::settings::ui::in_words(&names)))
        }
        Phase::Done if unchecked(s) => (Dot::Quiet, t!("store-upd-off").into()),
        Phase::Done => {
            let mut words = t!("store-upd-current").to_string();
            if let Some(time) = &s.upd.checked_at {
                words = then(words, &t!("store-upd-last-checked", time = time));
            }

            if let Some(w) = &s.upd.waits {
                words = then(words, w);
            }
            (Dot::UpToDate, words)
        }
    }
}

pub(super) fn no_more() -> &'static str {
    t!("store-upd-no-more")
}

pub(super) fn nothing_known() -> &'static str {
    t!("store-upd-nothing-known")
}

pub(super) fn not_checked() -> &'static str {
    t!("store-upd-not-checked")
}

pub(super) const CHECK_LIMIT: std::time::Duration = std::time::Duration::from_secs(180);

pub(super) fn too_long() -> &'static str {
    t!("store-upd-too-long")
}

pub(super) fn overall(s: &Store) -> f32 {
    let running: Vec<f32> = s.dials.iter().filter(|(k, d)| (k.starts_with("up:") || k.starts_with("fw:")) && d.running())
        .map(|(_, d)| d.share()).collect();
    if running.is_empty() { 0.0 } else { running.iter().sum::<f32>() / running.len() as f32 }
}

pub(crate) fn stopped_row() -> &'static str {
    t!("store-upd-stopped")
}

pub(super) fn status(s: &Store, key: &'static str, going: Option<f32>, waiting: usize, bytes: u64, held: usize) -> (Dot, String) {
    if s.upd.checking.contains(key) {
        (Dot::Checking, t!("store-upd-status-checking").into())
    } else if let Some(share) = going {
        (Dot::Filling(share), t!("store-upd-status-installing").into())
    } else if let Some(why) = s.upd.failed.get(key).filter(|_| s.online) {
        (Dot::Failed, t!("store-upd-status-failed", why = why))
    } else if waiting > 0 {
        let mut text = count_words(waiting);
        if bytes > 0 {
            text = t!("store-upd-status-size", count = text, size = crate::store::github::size_text(bytes));
        }
        if held > 0 {
            text = t!("store-upd-status-held", text = text, held = held);
        }
        (Dot::Waiting, text)
    } else if held > 0 {

        (Dot::Quiet, t!("store-upd-held-back-n", n = held))
    } else if !s.online {

        (Dot::Quiet, not_checked().into())
    } else if unchecked(s) {
        (Dot::Quiet, t!("store-upd-not-looked").into())
    } else {
        (Dot::UpToDate, t!("store-upd-up-to-date").into())
    }
}

pub(super) fn hold_why() -> &'static str {
    t!("store-upd-hold-why")
}

pub(super) fn nothing_held() -> &'static str {
    t!("store-upd-nothing-held")
}

pub(super) fn released() -> &'static str {
    t!("store-upd-released")
}

pub(super) fn release_why() -> &'static str {
    t!("store-upd-release-why")
}

pub(super) fn debian_why() -> &'static str {
    t!("store-upd-debian-why")
}

pub(super) fn not_held() -> &'static str {
    t!("store-upd-not-held")
}

pub(super) fn never_held() -> &'static str {
    t!("store-upd-never-held")
}

pub(super) fn no_flathub() -> &'static str {
    t!("store-upd-no-flathub")
}

pub(super) fn no_lists() -> &'static str {
    t!("store-upd-no-lists")
}

pub(super) fn not_fetched(why: &str) -> String {
    let why = why.trim().trim_start_matches(t!("store-plugins-not-done", why = "").as_str());
    if why.is_empty() {
        return no_lists().into();
    }
    let mut first = why.chars();
    let why: String = first.next().map(|c| c.to_uppercase().chain(first).collect()).unwrap_or_default();
    let why = if why.ends_with('.') { why } else { format!("{why}.") };
    t!("store-upd-no-lists-why", why = why)
}

pub(super) fn held_not_told() -> &'static str {
    t!("store-upd-held-not-told")
}

pub(super) fn released_not_told() -> &'static str {
    t!("store-upd-released-not-told")
}

pub(super) fn not_told(hold: bool, err: &str) -> String {
    let words = if hold { held_not_told() } else { released_not_told() };
    match err.lines().map(str::trim).rfind(|l| !l.is_empty()) {
        Some(why) => t!("store-upd-flatpak-said", words = words, why = why.trim_start_matches("error:").trim()),
        None => words.to_string(),
    }
}
