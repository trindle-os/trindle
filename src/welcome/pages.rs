//! The pages of the tour and what each offers.

use crate::toolkit::t;

use super::Offered;

pub(super) type Kinds = Vec<(&'static str, &'static str, &'static [(&'static str, &'static str)])>;

pub(super) fn first_apps() -> Kinds {
    vec![
        (t!("welcome-kind-browser"), "web-browser-symbolic",
         &[("com.brave.Browser", "Brave"), ("org.chromium.Chromium", "Chromium"), ("org.mozilla.firefox", "Firefox"),
           ("io.gitlab.librewolf-community", "LibreWolf"), ("app.zen_browser.zen", "Zen")]),
        (t!("welcome-kind-email"), "mail-send-symbolic",
         &[("eu.betterbird.Betterbird", "Betterbird"), ("org.gnome.Geary", "Geary"), ("org.mozilla.thunderbird", "Thunderbird")]),
        (t!("welcome-kind-chat"), "system-users-symbolic",
         &[("com.discordapp.Discord", "Discord"), ("org.signal.Signal", "Signal"), ("org.telegram.desktop", "Telegram")]),
        (t!("welcome-kind-office"), "x-office-document-symbolic",
         &[("org.libreoffice.LibreOffice", "LibreOffice"), ("org.onlyoffice.desktopeditors", "ONLYOFFICE")]),
    ]
}

pub(super) fn more_apps() -> Kinds {
    vec![
        (t!("welcome-kind-media"), "audio-x-generic-symbolic",
         &[("io.github.celluloid_player.Celluloid", "Celluloid"), ("com.spotify.Client", "Spotify"), ("org.videolan.VLC", "VLC")]),
        (t!("welcome-kind-pictures"), "image-x-generic-symbolic",
         &[("org.gimp.GIMP", "GIMP"), ("org.inkscape.Inkscape", "Inkscape"), ("org.kde.krita", "Krita"), ("org.gnome.Loupe", "Loupe")]),
        (t!("welcome-kind-notes"), "document-edit-symbolic",
         &[("net.cozic.joplin_desktop", "Joplin"), ("md.obsidian.Obsidian", "Obsidian"), ("org.gnome.TextEditor", "Text Editor")]),
        (t!("welcome-kind-pdf"), "document-open-symbolic",
         &[("org.gnome.Papers", "Papers"), ("com.github.xournalpp.xournalpp", "Xournal++")]),
    ]
}

const PLUGINS_SHOWN: usize = 3;

pub(super) fn offered(offers: &[crate::plugins::catalogue::Offer]) -> Vec<Offered> {
    offers.iter().take(PLUGINS_SHOWN)
        .map(|o| Offered { icon: o.icon.clone(), name: o.entry.name.clone(), line: o.entry.summary.clone() })
        .collect()
}

pub(super) fn in_the_ring() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("view-app-grid-symbolic", t!("welcome-held-programs"), t!("welcome-held-programs-line")),
        ("preferences-system-time-symbolic", t!("welcome-held-time"), t!("welcome-held-time-line")),
        ("audio-volume-high-symbolic", t!("welcome-held-switches"), t!("welcome-held-switches-line")),
    ]
}

pub(super) fn know_cards() -> [(&'static str, &'static str, &'static str); 4] {
    [
        ("package-x-generic-symbolic", t!("welcome-know-boxes"), t!("welcome-know-boxes-line")),
        ("software-update-available-symbolic", t!("welcome-know-updates"), t!("welcome-know-updates-line")),
        ("emblem-system-symbolic", t!("welcome-know-ring"), t!("welcome-know-ring-line")),
        ("help-about-symbolic", t!("welcome-know-help"), t!("welcome-know-help-line")),
    ]
}

pub(super) fn files_cards() -> [(&'static str, &'static str, &'static str); 4] {
    [
        ("go-home-symbolic", t!("welcome-files-home"), t!("welcome-files-home-line")),
        ("drive-removable-media-symbolic", t!("welcome-files-drives"), t!("welcome-files-drives-line")),
        ("folder-symbolic", t!("welcome-files-hidden"), t!("welcome-files-hidden-line")),
        ("user-trash-symbolic", t!("welcome-files-bin"), t!("welcome-files-bin-line")),
    ]
}

pub(super) fn software_cards() -> [(&'static str, &'static str, &'static str); 4] {
    [
        ("view-app-grid-symbolic", t!("welcome-software-apps"), t!("welcome-software-apps-line")),
        ("package-x-generic-symbolic", t!("welcome-software-exe"), t!("welcome-software-exe-line")),
        ("software-update-available-symbolic", t!("welcome-software-updates"), t!("welcome-software-updates-line")),
        ("edit-delete-symbolic", t!("welcome-software-remove"), t!("welcome-software-remove-line")),
    ]
}

pub(super) fn admin_cards() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("dialog-password-symbolic", t!("welcome-admin-why"), t!("welcome-admin-why-line")),
        ("system-lock-screen-symbolic", t!("welcome-admin-who"), t!("welcome-admin-who-line")),
        ("dialog-information-symbolic", t!("welcome-admin-not"), t!("welcome-admin-not-line")),
    ]
}

pub(super) fn snapshot_cards() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("document-revert-symbolic", t!("welcome-snap-taken"), t!("welcome-snap-taken-line")),
        ("preferences-system-time-symbolic", t!("welcome-snap-list"), t!("welcome-snap-list-line")),
        ("system-reboot-symbolic", t!("welcome-snap-boot"), t!("welcome-snap-boot-line")),
    ]
}

pub(super) fn words_cards() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("document-edit-symbolic", t!("welcome-words-change"), t!("welcome-words-change-line")),
        ("document-send-symbolic", t!("welcome-words-share"), t!("welcome-words-share-line")),
        ("preferences-desktop-locale-symbolic", t!("welcome-words-all"), t!("welcome-words-all-line")),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    New,
    Linux,
    Trindle,
}

impl Level {
    pub(super) const ALL: [Level; 3] = [Level::New, Level::Linux, Level::Trindle];

    pub(super) fn card(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Level::New => ("face-smile-symbolic", t!("welcome-level-new"), t!("welcome-level-new-line")),
            Level::Linux => ("utilities-terminal-symbolic", t!("welcome-level-linux"), t!("welcome-level-linux-line")),
            Level::Trindle => ("trindle-ring-symbolic", t!("welcome-level-trindle"), t!("welcome-level-trindle-line")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    LinuxBasics,
    Trindle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Level,
    Hello,
    Ring,
    Files,
    Software,
    Admin,
    Apps,
    MoreApps,
    Plugins,
    Reports,
    Dots,
    Snapshots,
    Words,
    Know,
}

pub(super) const PAGES: [Page; 14] = [
    Page::Level, Page::Hello, Page::Ring, Page::Files, Page::Software, Page::Admin, Page::Apps, Page::MoreApps, Page::Plugins,
    Page::Reports, Page::Dots, Page::Snapshots, Page::Words, Page::Know,
];

pub fn reports_question() -> &'static str {
    t!("welcome-reports-question")
}

pub(super) fn title() -> &'static str {
    t!("welcome-title")
}

impl Page {
    pub fn mark(self) -> Mark {
        match self {
            Page::Files | Page::Software | Page::Admin => Mark::LinuxBasics,
            Page::Level | Page::Hello | Page::Ring | Page::Apps | Page::MoreApps | Page::Plugins | Page::Reports | Page::Dots
            | Page::Snapshots | Page::Words | Page::Know => Mark::Trindle,
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            Page::Level => t!("welcome-page-level"),
            Page::Files => t!("welcome-page-files"),
            Page::Software => t!("welcome-page-software"),
            Page::Admin => t!("welcome-page-admin"),
            Page::Snapshots => t!("welcome-page-snapshots"),
            Page::Words => t!("welcome-page-words"),
            Page::Hello => title(),
            Page::Ring => t!("welcome-page-ring"),
            Page::Apps => t!("welcome-page-apps"),
            Page::MoreApps => t!("welcome-page-more-apps"),
            Page::Plugins => t!("welcome-page-plugins"),
            Page::Reports => t!("welcome-page-reports"),
            Page::Dots => t!("welcome-page-dots"),
            Page::Know => t!("welcome-page-know"),
        }
    }

    pub(super) fn sentence(self) -> &'static str {
        match self {
            Page::Level => t!("welcome-sentence-level"),
            Page::Files => t!("welcome-sentence-files"),
            Page::Software => t!("welcome-sentence-software"),
            Page::Admin => t!("welcome-sentence-admin"),
            Page::Snapshots => t!("welcome-sentence-snapshots"),
            Page::Words => t!("welcome-sentence-words"),
            Page::Hello => t!("welcome-sentence-hello"),
            Page::Ring => t!("welcome-sentence-ring"),
            Page::Apps => t!("welcome-sentence-apps"),
            Page::MoreApps => t!("welcome-sentence-more-apps"),
            Page::Plugins => t!("welcome-sentence-plugins"),
            Page::Reports => t!("welcome-sentence-reports"),
            Page::Dots => t!("welcome-sentence-dots"),
            Page::Know => t!("welcome-sentence-know"),
        }
    }

    pub(super) fn spoken(self, n: usize, total: usize) -> String {
        t!("welcome-page-spoken", title = self.title(), n = n, total = total, sentence = self.sentence())
    }

    pub(super) fn kinds(self) -> Option<Kinds> {
        match self {
            Page::Apps => Some(first_apps()),
            Page::MoreApps => Some(more_apps()),
            _ => None,
        }
    }

    pub(super) fn moves(self) -> bool {
        matches!(self, Page::Ring)
    }
}

pub(super) fn every_app() -> impl Iterator<Item = (&'static str, &'static str)> {
    first_apps().into_iter().chain(more_apps()).flat_map(|(_, _, apps)| apps.iter().copied())
}

pub(super) fn name_of(id: &str) -> String {
    every_app().find(|(i, _)| *i == id).map(|(_, n)| n.to_string()).unwrap_or_else(|| id.to_string())
}

pub(super) fn named(names: &[String]) -> String {
    if names.len() <= 3 {
        return crate::settings::ui::in_words(names);
    }
    t!("welcome-named-more", first = &names[0], second = &names[1], more = names.len() - 2)
}
