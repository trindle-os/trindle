//! Which page of Help explains which failure in Apps. Red words in Apps get
//! a quiet More beside them (`common::more`); the messages are grouped by
//! cause, so one page covers several of them. The pages are the sections of
//! site/content/apps.md (in the trindle-help repository) named below, and the test in `common::more` fails
//! for an address with no section there.

use crate::toolkit::widget::{note, row, Length, Node, Tone};

pub const OFFLINE: &str = "apps#apps-could-not-reach-the-internet";

pub const INSTALL: &str = "apps#an-app-would-not-install-or-remove";

pub const UPDATE: &str = "apps#an-update-did-not-finish";

pub const HELD: &str = "apps#holding-an-update-back-did-not-work";

pub const FIRMWARE: &str = "apps#firmware-did-not-update";

pub const GITHUB: &str = "apps#github-would-not-answer";

pub const GITHUB_KEY: &str = "apps#the-github-key-was-not-kept";

pub const BACKUP_MAKE: &str = "apps#a-backup-was-not-made";

pub const BACKUP_PUT: &str = "apps#a-backup-was-not-put-back";

pub const BACKUP_HTTP: &str = super::link::HELP;

pub const DEB: &str = "apps#a-deb-file-would-not-install";

pub const PLUGIN: &str = "apps#a-plugin-is-not-working";

pub const SOURCES: &str = "apps#a-software-source-was-refused";

pub fn of_key(key: &str) -> &'static str {
    if key == "gh:key" {
        return GITHUB_KEY;
    }
    if key == "bk:make" {
        return BACKUP_MAKE;
    }
    match key.split_once(':').map_or("", |(kind, _)| kind) {
        "gh" => GITHUB,
        "bk" => BACKUP_PUT,
        "src" => SOURCES,
        "fw" => FIRMWARE,
        "pl" | "plugin" => PLUGIN,
        "deb" => DEB,
        "up" => UPDATE,
        "hold" | "held" => HELD,
        _ => INSTALL,
    }
}

pub fn open(page: &'static str) {
    open_with(page, "");
}

pub fn open_with(page: &'static str, error: &str) {
    #[cfg(test)]
    {
        OPENED.with(|o| o.borrow_mut().push(page));
        TEXTS.with(|o| o.borrow_mut().push(error.to_string()));
    }
    #[cfg(not(test))]
    crate::common::more::open_with(page, error);
}

#[cfg(test)]
thread_local! {
    static OPENED: std::cell::RefCell<Vec<&'static str>> = const { std::cell::RefCell::new(Vec::new()) };
    static TEXTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub fn texts() -> Vec<String> {
    TEXTS.with(|o| o.borrow().clone())
}

#[cfg(test)]
pub fn opened() -> Vec<&'static str> {
    OPENED.with(|o| o.borrow().clone())
}

pub fn red<M: Clone + 'static>(words: impl Into<String>, press: M) -> Node<M> {
    row(vec![note(words.into()).tone(Tone::Problem).width(Length::Fill), crate::common::more::button(press)]).gap(8.0).centred()
}

pub fn red_cut<M: Clone + 'static>(words: impl Into<String>, press: M) -> Node<M> {
    row(vec![note(words.into()).tone(Tone::Problem).width(Length::Fill).cut(), crate::common::more::button(press)]).gap(8.0).centred()
}

pub fn red_blank<M: Clone + 'static>() -> Node<M> {
    let k = crate::toolkit::theme::icon_button();
    row(vec![note(super::listing::BLANK).tone(Tone::Problem).width(Length::Fill).cut(), crate::toolkit::widget::space().width(Length::Fixed(k)).height(Length::Fixed(k))]).gap(8.0).centred()
}

pub fn beside<M: Clone + 'static>(words: Node<M>, press: M) -> Node<M> {
    row(vec![words.width(Length::Fill), crate::common::more::button(press)]).gap(8.0).centred()
}

pub fn beside_room<M: Clone + 'static>(words: Node<M>) -> Node<M> {
    let k = crate::toolkit::theme::icon_button();
    row(vec![words.width(Length::Fill), crate::toolkit::widget::space().width(Length::Fixed(k)).height(Length::Fixed(k))]).gap(8.0).centred()
}

#[cfg(test)]
pub const PAGES: &[&str] = &[OFFLINE, INSTALL, UPDATE, HELD, FIRMWARE, GITHUB, GITHUB_KEY, BACKUP_MAKE, BACKUP_PUT, BACKUP_HTTP, DEB, PLUGIN, SOURCES];

#[cfg(test)]
pub const MESSAGES: &[crate::common::more::Entry] = &[

    ("store-gh-offline-note", OFFLINE),
    ("store-app-picture-missed", OFFLINE),
    ("store-app-picture-offline", OFFLINE),
    ("store-app-rating-offline", OFFLINE),
    ("store-app-people-offline", OFFLINE),
    ("store-wait-comes-from", OFFLINE),
    ("store-catalogue-unreadable", OFFLINE),
    ("store-catalogue-not-fetched", OFFLINE),

    ("store-flatpak-refused", INSTALL),
    ("store-installed-appimage-failed", INSTALL),
    ("store-add-no-bus", INSTALL),
    ("store-add-no-chooser", INSTALL),
    ("store-add-bad-ref", INSTALL),
    ("store-add-ref-refused", INSTALL),
    ("store-add-ref-failed", INSTALL),
    ("store-job-no-reason", INSTALL),
    ("store-not-in-catalogue", INSTALL),
    ("store-no-browser", INSTALL),

    ("store-upd-no-flathub", UPDATE),
    ("store-upd-no-lists", UPDATE),
    ("store-upd-no-lists-why", UPDATE),
    ("store-not-updated-said", UPDATE),
    ("store-not-updated-none", UPDATE),
    ("store-upd-status-failed", UPDATE),
    ("store-upd-flatpak-said", UPDATE),
    ("common-updating-no-reason", UPDATE),
    ("common-updating-failed", UPDATE),
    ("common-updating-not-installed", UPDATE),
    ("store-upd-too-long", UPDATE),

    ("store-upd-not-held-list", HELD),
    ("store-upd-not-released-list", HELD),
    ("store-upd-held-not-told", HELD),
    ("store-upd-released-not-told", HELD),

    ("store-upd-fw-no-list", FIRMWARE),

    ("store-gh-unreached", GITHUB),
    ("store-gh-no-files", GITHUB),
    ("store-gh-no-releases", GITHUB),
    ("store-gh-nothing-installs", GITHUB),
    ("store-gh-limit", GITHUB),
    ("store-gh-limit-hint", GITHUB),
    ("store-gh-refused-plain", GITHUB),
    ("store-gh-refused", GITHUB),
    ("store-gh-answered", GITHUB),
    ("store-gh-unread-sentence", GITHUB),
    ("store-gh-unread-why", GITHUB),
    ("store-gh-half", GITHUB),
    ("store-gh-download-failed", GITHUB),

    ("store-gh-key-refused", GITHUB_KEY),
    ("store-gh-key-spaces", GITHUB_KEY),
    ("store-gh-key-not-removed", GITHUB_KEY),
    ("store-gh-key-not-kept", GITHUB_KEY),
    ("store-gh-keyis-plain", GITHUB_KEY),
    ("store-gh-keyis-hidden", GITHUB_KEY),
    ("store-gh-keyis-unsendable", GITHUB_KEY),
    ("store-gh-keyis-file-stays", GITHUB_KEY),
    ("store-gh-keyis-not-taken", GITHUB_KEY),
    ("store-key-unreached", GITHUB_KEY),
    ("store-key-not-text", GITHUB_KEY),
    ("store-key-not-removed", GITHUB_KEY),
    ("store-key-not-read", GITHUB_KEY),

    ("store-backup-not-made", BACKUP_MAKE),

    ("store-bk-no-bus", BACKUP_PUT),
    ("store-bk-no-answer", BACKUP_PUT),
    ("store-refusal-nothing", BACKUP_PUT),
    ("store-refusal-part", BACKUP_PUT),
    ("store-refusal-picture-only", BACKUP_PUT),
    ("store-refusal-picture-too", BACKUP_PUT),
    ("store-refusal-pkexec", BACKUP_PUT),
    ("store-refusal-not-read-from", BACKUP_PUT),
    ("store-not-installed-failed", BACKUP_PUT),
    ("store-not-installed-both", BACKUP_PUT),
    ("store-not-installed-refused", BACKUP_PUT),
    ("store-flat-summary-failed", BACKUP_PUT),

    ("store-bk-http-line", BACKUP_HTTP),
    ("store-bk-http-card", BACKUP_HTTP),

    ("store-deb-not-finished", DEB),
    ("store-deb-elsewhere", DEB),
    ("store-deb-trindle", DEB),
    ("store-deb-service", DEB),
    ("store-deb-missing", DEB),
    ("store-boxed-not-deb", DEB),
    ("store-parts-not-deb", DEB),
    ("store-parts-not-unpacked", DEB),
    ("store-loose-not-boxed", DEB),

    ("store-plugins-not-removed", PLUGIN),
    ("plugins-not-saved", PLUGIN),

    ("store-src-bad-address", SOURCES),
    ("store-src-bad-words", SOURCES),
    ("store-src-bad-package", SOURCES),
];

#[cfg(test)]
mod tests {
    use super::*;

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
    fn red_words_land_on_the_page_for_where_they_are_said() {
        for (key, page) in [("gh:key", GITHUB_KEY), ("gh:owner/repo", GITHUB), ("gh:owner/repo/a.AppImage", GITHUB), ("bk:back", BACKUP_PUT), ("bk:make", BACKUP_MAKE),
                            ("src:custom", SOURCES), ("src:tailscale", SOURCES), ("fw:abc", FIRMWARE), ("pl:a.b", PLUGIN), ("plugin:a.b", PLUGIN), ("deb:x", DEB),
                            ("up:apps", UPDATE), ("up:system", UPDATE), ("up:app:org.example.Paint", UPDATE), ("hold:up:app:x", HELD),
                            ("org.example.Paint", INSTALL), ("app:org.example.Paint", INSTALL)] {
            assert_eq!(of_key(key), page, "{key}");
            assert!(crate::common::more::valid(page), "{page}");
        }
    }
}
