//! Which page of Help explains which failure in Drives. Red words in Drives
//! get a quiet More beside them (`common::more`); the messages are grouped
//! by cause, so one page covers several of them. The pages are the sections
//! of site/content/drives.md (in the trindle-help repository) named below, and the test in `common::more`
//! fails for an address with no section there.

use crate::common::more::fixed_part as fixed;
use crate::toolkit::t;

pub const SERVICE: &str = "drives#the-drive-service-did-not-answer";

pub const PASSWORD: &str = "drives#a-change-was-not-made-without-a-password";

pub const STOPPED: &str = "drives#a-change-did-not-finish";

pub const ROOM: &str = "drives#room-could-not-be-made";

pub const NAMES: &str = "drives#a-name-or-size-was-not-accepted";

pub const HEALTH: &str = "drives#a-drive-needs-attention";

pub const FULL: &str = "drives#the-computer-is-nearly-full";

pub const HIDE: &str = "drives#hiding-a-drive-was-not-kept";

pub fn of_outcome(row: &str, words: &str) -> &'static str {
    let has = |needle: String| !needle.is_empty() && words.contains(needle.as_str());
    if has(t!("disks-no-service").into()) || has(t!("disks-service-not-asked").into()) || has(t!("disks-service-unreachable").into()) {
        return SERVICE;
    }
    if [t!("disks-no-password"), t!("disks-no-password-part-way"), t!("disks-one-at-a-time"), t!("disks-not-sure"), t!("disks-standing")]
        .iter().any(|w| words.contains(*w))
        || has(fixed("disks-nothing-changed", &["reason"]))
        || has(fixed("disks-part-way", &["reason"]))
        || has(fixed("disks-put-back-password", &["dev"]))
    {
        return PASSWORD;
    }
    match row.rsplit_once(':').map_or("", |(_, kind)| kind) {
        "room" => ROOM,
        "faults" => HEALTH,
        _ => STOPPED,
    }
}

pub fn open(page: &'static str) {
    #[cfg(test)]
    OPENED.with(|o| o.borrow_mut().push(page));
    #[cfg(not(test))]
    crate::common::more::open(page);
}

#[cfg(test)]
thread_local! {
    static OPENED: std::cell::RefCell<Vec<&'static str>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub const PAGES: &[&str] = &[SERVICE, PASSWORD, STOPPED, ROOM, NAMES, HEALTH, FULL, HIDE];

#[cfg(test)]
pub const MESSAGES: &[crate::common::more::Entry] = &[

    ("disks-cannot-read", SERVICE),
    ("disks-missing", SERVICE),
    ("disks-no-service", SERVICE),
    ("disks-service-unreachable", SERVICE),
    ("disks-service-not-asked", SERVICE),

    ("disks-no-password", PASSWORD),
    ("disks-no-password-part-way", PASSWORD),
    ("disks-one-at-a-time", PASSWORD),
    ("disks-not-sure", PASSWORD),
    ("disks-standing", PASSWORD),
    ("disks-nothing-changed", PASSWORD),
    ("disks-part-way", PASSWORD),
    ("disks-put-back-password", PASSWORD),

    ("disks-failed-part-way", STOPPED),
    ("disks-closed", STOPPED),
    ("disks-left-unmounted", STOPPED),
    ("disks-still-unmounted", STOPPED),
    ("disks-may-be-unmounted", STOPPED),
    ("disks-put-back-failed", STOPPED),
    ("disks-folder-not-kept", STOPPED),
    ("disks-busy", STOPPED),
    ("disks-no-room", STOPPED),
    ("disks-new-part-missing", STOPPED),
    ("disks-uuid-unread", STOPPED),

    ("disks-room-files-missing", ROOM),
    ("disks-snapshot-missing", ROOM),
    ("disks-snapshot-failed", ROOM),
    ("disks-snapshot-changed", ROOM),
    ("disks-room-ext-offline", ROOM),
    ("disks-room-xfs-fixed", ROOM),
    ("disks-room-other-kind", ROOM),
    ("disks-room-no-snapshot", ROOM),
    ("disks-room-not-in-table", ROOM),
    ("disks-room-no-online-shrink", ROOM),
    ("disks-room-holds-parts", ROOM),
    ("disks-room-not-gpt", ROOM),
    ("disks-room-mount-first", ROOM),
    ("disks-room-mounts-not-one", ROOM),
    ("disks-room-read-only", ROOM),
    ("disks-room-read-only-unknown", ROOM),
    ("disks-room-many-drives", ROOM),
    ("disks-room-drives-unknown", ROOM),
    ("disks-room-swap-file", ROOM),
    ("disks-room-swap-unknown", ROOM),
    ("disks-room-used-unknown", ROOM),
    ("disks-room-area-unknown", ROOM),
    ("disks-room-open-missing", ROOM),
    ("disks-room-encryption-sizes", ROOM),
    ("disks-room-sizes", ROOM),
    ("disks-room-too-little", ROOM),
    ("disks-room-sizes-unread", ROOM),
    ("disks-room-range", ROOM),
    ("disks-room-stopped", ROOM),
    ("disks-room-gone", ROOM),

    ("disks-name-empty", NAMES),
    ("disks-name-too-long", NAMES),
    ("disks-name-letters", NAMES),
    ("disks-name-start", NAMES),
    ("disks-name-taken", NAMES),
    ("disks-label-too-long", NAMES),
    ("disks-label-fat-marks", NAMES),
    ("disks-resize-mount-first", NAMES),
    ("disks-resize-cannot", NAMES),

    ("disks-headline-failing", HEALTH),
    ("disks-headline-look", HEALTH),
    ("disks-health-failing", HEALTH),
    ("disks-health-attention", HEALTH),
    ("disks-trouble-spare", HEALTH),
    ("disks-trouble-temperature", HEALTH),
    ("disks-trouble-wearing-out", HEALTH),
    ("disks-trouble-readonly", HEALTH),
    ("disks-trouble-volatile", HEALTH),
    ("disks-trouble-pmr", HEALTH),
    ("disks-repair-failed", HEALTH),
    ("disks-faults-found", HEALTH),
    ("disks-no-answer", HEALTH),
    ("disks-no-answer-help", HEALTH),

    ("disks-headline-full", FULL),

    ("disks-hide-failed", HIDE),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::refused;
    use crate::disks::plan;

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
    fn words_about_a_change_land_on_the_page_for_their_cause() {
        let row = "/b/Data:size";
        assert_eq!(of_outcome(row, plan::no_service()), SERVICE);
        assert_eq!(of_outcome(row, &t!("disks-nothing-changed", reason = t!("disks-service-not-asked"))), SERVICE);
        for r in refused::ALL {
            let words = plan::refusal_words(r);
            assert_eq!(of_outcome(row, &words), PASSWORD, "{words}");
            assert_eq!(of_outcome("/b/Data:room", &words), PASSWORD, "a refusal is a refusal on any row");
        }
        for words in [plan::no_password().to_string(), plan::one_at_a_time().into(), plan::not_sure().into(), plan::standing_words("Resized."),
                      t!("disks-part-way", reason = "The password was not accepted"), t!("disks-put-back-password", dev = "sdb1")] {
            assert_eq!(of_outcome(row, &words), PASSWORD, "{words}");
        }
        for words in [t!("disks-failed-part-way", why = "It broke."), plan::closed().to_string(), t!("disks-busy").to_string()] {
            assert_eq!(of_outcome(row, &words), STOPPED, "{words}");
        }
        assert_eq!(of_outcome("/b/Data:room", t!("disks-snapshot-changed")), ROOM);
        assert_eq!(of_outcome("/b/Data:faults", t!("disks-repair-failed")), HEALTH);
    }

    #[test]
    fn a_message_that_is_only_values_has_no_fixed_part() {
        assert_eq!(fixed("disks-sentence", &["words"]), "");
        assert!(fixed("disks-nothing-changed", &["reason"]).contains("nothing changed"));
    }
}
