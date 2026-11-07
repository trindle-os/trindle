//! Which page of Help explains which failure of a plugin.

use crate::common::more::fixed_part as fixed;

pub const FILE: &str = "plugins#a-plugin-was-refused-by-the-check";
pub const BACKGROUND: &str = "plugins#a-plugin-s-background-part-is-not-running";
pub const GET: &str = "plugins#a-plugin-would-not-download-or-go-in";
pub const LIST: &str = "plugins#the-plugin-list-could-not-be-used";

pub fn of_words(words: &str) -> Option<&'static str> {
    let has = |needle: String| !needle.is_empty() && words.contains(needle.as_str());
    if has(fixed("store-plugins-not-running", &["why"])) || has(fixed("plugins-swap-not-running", &["why", "not"])) {
        return Some(BACKGROUND);
    }
    if has(fixed("plugins-install-check", &["why"])) {
        return Some(FILE);
    }
    let get = [("plugins-download-failed", "why"), ("plugins-unpack-failed", "why"), ("plugins-install-other-id", "found"), ("plugins-install-note", "why"),
               ("plugins-download-curl", "why"), ("plugins-unpack-tar", "why"), ("plugins-unpack-no-manifest", "file")];
    if get.iter().any(|(key, arg)| has(fixed(key, &[*arg, "wanted"]))) || [crate::toolkit::t!("plugins-install-hash"), crate::toolkit::t!("plugins-copy-failed"), crate::toolkit::t!("plugins-update-not-installed")].iter().any(|w| words.contains(*w)) {
        return Some(GET);
    }
    None
}

#[cfg(test)]
pub const PAGES: &[&str] = &[FILE, BACKGROUND, GET, LIST];

#[cfg(test)]
pub const MESSAGES: &[crate::common::more::Entry] = &[
    ("plugins-check-no-manifest", FILE),
    ("plugins-check-no-plugin-part", FILE),
    ("plugins-check-interface-other", FILE),
    ("plugins-check-interface-missing", FILE),
    ("plugins-check-id", FILE),
    ("plugins-check-needs-name", FILE),
    ("plugins-check-needs-version", FILE),
    ("plugins-check-summary-one-line", FILE),
    ("plugins-plain-line", FILE),
    ("plugins-check-permission-unknown", FILE),
    ("plugins-ring-needs-icon", FILE),
    ("plugins-ring-icon-missing", FILE),
    ("plugins-ring-needs-label", FILE),
    ("plugins-ring-needs-run", FILE),
    ("plugins-settings-needs-fields", FILE),
    ("plugins-field-bad-name", FILE),
    ("plugins-field-no-part", FILE),
    ("plugins-needs-label", FILE),
    ("plugins-choices-limits", FILE),
    ("plugins-choices-from-name", FILE),
    ("plugins-choice-needs-choices", FILE),
    ("plugins-choice-default", FILE),
    ("plugins-note-needs-textfrom", FILE),
    ("plugins-note-default", FILE),
    ("plugins-field-kind", FILE),
    ("plugins-switch-default", FILE),
    ("plugins-background-needs-permission", FILE),
    ("plugins-hooks-nothing", FILE),
    ("plugins-links-outside", FILE),
    ("plugins-run-unreadable", FILE),
    ("plugins-run-outside", FILE),
    ("plugins-run-missing", FILE),
    ("plugins-menu-too-many", FILE),
    ("plugins-menu-bad-id", FILE),
    ("plugins-menu-twice", FILE),
    ("plugins-menu-no-part", FILE),
    ("plugins-label-short-line", FILE),
    ("plugins-menu-needs-run", FILE),
    ("plugins-pin-group-only-flow", FILE),
    ("plugins-pin-group-needs-run", FILE),
    ("plugins-showwhen-long", FILE),
    ("plugins-showwhen-form", FILE),
    ("plugins-showwhen-field-name", FILE),
    ("plugins-showwhen-value", FILE),
    ("plugins-showwhen-own", FILE),
    ("plugins-showwhen-unknown", FILE),
    ("plugins-showwhen-later", FILE),
    ("plugins-showwhen-chained", FILE),
    ("plugins-showwhen-switch", FILE),
    ("plugins-showwhen-not-choice", FILE),
    ("plugins-showwhen-note", FILE),
    ("plugins-listen-fault", FILE),
    ("plugins-listen-needs-network", FILE),
    ("plugins-listen-needs-listen", FILE),
    ("plugins-listen-needs-ports", FILE),
    ("plugins-shortcut-too-many", FILE),
    ("plugins-shortcut-bad-id", FILE),
    ("plugins-shortcut-twice", FILE),
    ("plugins-shortcut-no-part", FILE),
    ("plugins-shortcut-keys-trindle", FILE),
    ("plugins-shortcut-keys-twice", FILE),
    ("plugins-shortcut-keys-needed", FILE),
    ("plugins-shortcut-needs-run", FILE),
    ("plugins-install-check", FILE),
    ("plugins-swap-not-running", BACKGROUND),
    ("plugins-bg-no-program", BACKGROUND),
    ("plugins-bg-slow", BACKGROUND),
    ("plugins-bg-no-session", BACKGROUND),
    ("plugins-bg-which-session", BACKGROUND),
    ("store-plugins-not-running", BACKGROUND),
    ("plugins-download-failed", GET),
    ("plugins-download-curl", GET),
    ("plugins-unpack-failed", GET),
    ("plugins-unpack-tar", GET),
    ("plugins-unpack-no-manifest", GET),
    ("plugins-scratch", GET),
    ("plugins-install-hash", GET),
    ("plugins-install-other-id", GET),
    ("plugins-install-note", GET),
    ("plugins-copy-failed", GET),
    ("plugins-update-not-installed", GET),
    ("plugins-refused-no-key", LIST),
    ("plugins-refused-no-signature", LIST),
    ("plugins-refused-wrong-size", LIST),
    ("plugins-refused-not-signed", LIST),
    ("plugins-refused-no-number", LIST),
    ("plugins-refused-older", LIST),
    ("plugins-refused-unreadable", LIST),
    ("plugins-catalogue-last-good", LIST),
    ("plugins-catalogue-paused", LIST),
    ("plugins-catalogue-not-text", LIST),
    ("plugins-catalogue-too-big", LIST),
    ("plugins-catalogue-not-one", LIST),
    ("plugins-catalogue-newer", LIST),
    ("plugins-catalogue-left-out", LIST),
    ("plugins-start-not-taken", LIST),
    ("plugins-start-nul", LIST),
    ("plugins-start-no-note", LIST),
    ("store-plugins-not-done", LIST),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::t;

    #[test]
    fn every_page_is_used_and_every_message_names_one() {
        for page in PAGES {
            assert!(MESSAGES.iter().any(|(_, a)| a == page), "{page} has no message");
        }
        for (key, address) in MESSAGES {
            assert!(PAGES.contains(address), "{key} names {address}");
        }
    }

    #[test]
    fn red_words_under_a_plugin_land_on_the_page_for_their_cause() {
        assert_eq!(of_words(&t!("store-plugins-not-running", why = "no desktop session is running")), Some(BACKGROUND));
        assert_eq!(of_words(&t!("plugins-swap-not-running", why = "The new files could not be put in.", not = "it did not start")), Some(BACKGROUND));
        assert_eq!(of_words(&t!("plugins-install-check", why = "[Plugin] needs a Name.")), Some(FILE));
        assert_eq!(of_words(&t!("plugins-download-failed", why = "Could not resolve host")), Some(GET));
        assert_eq!(of_words(&t!("plugins-unpack-failed", why = "bad archive")), Some(GET));
        assert_eq!(of_words(t!("plugins-install-hash")), Some(GET));
        assert_eq!(of_words(&t!("plugins-install-other-id", found = "a.b", wanted = "c.d")), Some(GET));
        assert_eq!(of_words(&t!("plugins-install-note", why = "disk full")), Some(GET));
        assert_eq!(of_words(&t!("store-plugins-not-removed", why = "busy")), None);
        assert_eq!(of_words("Refused."), None);
    }
}
