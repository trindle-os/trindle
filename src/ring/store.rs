//! Persisted favorites order and per-app launch counts.

use std::collections::HashMap;
use std::time::SystemTime;

use crate::common::favourites::{from_text, renamed, HEADER};
use crate::common::files::{config_path, read_text, write_text};

pub use crate::common::favourites::MAX as MAX_FAVOURITES;
#[cfg(test)]
use crate::common::favourites::STARTING;

pub fn favourites() -> Vec<String> {
    let text = read_text(config_path("favourites"));
    let ids = from_text(&text);
    let old = text.lines().any(|l| renamed(l.trim()) != l.trim());
    if old && text.lines().any(|l| l == HEADER) {
        save_favourites(&ids);
    }
    ids
}

pub fn save_favourites(ids: &[String]) {
    if let Err(e) = crate::common::favourites::save(&config_path("favourites"), ids) {
        eprintln!("trindle-ring: could not save favorites: {e}");
        crate::common::crash::error("E_SAVE_FAILED");
    }
}

pub fn move_favourite(ids: &mut Vec<String>, from: &str, to: &str) -> bool {
    let (Some(i), Some(j)) = (ids.iter().position(|x| x == from), ids.iter().position(|x| x == to)) else { return false };
    if i == j {
        return false;
    }
    let id = ids.remove(i);
    ids.insert(j, id);
    true
}

pub fn favourites_changed_at() -> Option<SystemTime> {
    std::fs::metadata(config_path("favourites")).and_then(|m| m.modified()).ok()
}

pub const USAGE_DAMAGED: &str = "Launch counts";

pub fn usage_in(file: &std::path::Path) -> HashMap<String, u32> {
    crate::common::damaged::read(file, USAGE_DAMAGED, |l| l.split_once(' ').is_some_and(|(n, _)| n.parse::<u32>().is_ok()))
        .lines()
        .filter_map(|l| {
            let (n, id) = l.split_once(' ')?;
            Some((renamed(id.trim()).to_string(), n.parse::<u32>().ok()?))
        })
        .fold(HashMap::new(), |mut counts, (id, n)| {
            *counts.entry(id).or_insert(0) += n;
            counts
        })
}

pub fn record_use(app_id: &str) {
    record_use_in(&config_path("usage"), app_id);
}

pub(crate) fn record_use_in(file: &std::path::Path, app_id: &str) {
    let mut counts = usage_in(file);
    if crate::common::damaged::may_write(file, USAGE_DAMAGED).is_err() {
        return;
    }
    *counts.entry(app_id.to_string()).or_insert(0) += 1;
    let mut lines: Vec<_> = counts.into_iter().collect();
    lines.sort_by_key(|k| std::cmp::Reverse(k.1));
    lines.truncate(500);
    let text = lines.iter().fold(String::new(), |mut s, (id, n)| {
        let _ = std::fmt::Write::write_fmt(&mut s, format_args!("{n} {id}\n"));
        s
    });
    let _ = write_text(file, &text);
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_renamed_favourite_keeps_its_place() {
        assert_eq!(super::renamed("trindle-disks.desktop"), "trindle-drives.desktop");
        assert_eq!(super::renamed("trindle-store.desktop"), "trindle-apps.desktop");
        assert_eq!(super::renamed("trindle-files.desktop"), "trindle-folders.desktop");
        assert_eq!(super::renamed("firefox.desktop"), "firefox.desktop");
        let ids = super::from_text(&format!("{}\ntrindle-store.desktop\ntrindle-files.desktop\ntrindle-apps.desktop\n", super::HEADER));
        assert_eq!(ids, ["trindle-apps.desktop", "trindle-folders.desktop"]);
        let ids = super::from_text(&format!("{}\ntrindle-settings.desktop\ntrindle-files.desktop\ntrindle-drives.desktop\ntrindle-folders.desktop\n", super::HEADER));
        assert_eq!(ids, ["trindle-settings.desktop", "trindle-folders.desktop", "trindle-drives.desktop"]);
    }

    #[test]
    fn dragging_a_favourite_takes_the_place_of_another() {
        let mut ids: Vec<String> = ["a", "b", "c", "d"].iter().map(|s| s.to_string()).collect();
        assert!(super::move_favourite(&mut ids, "d", "b"));
        assert_eq!(ids, ["a", "d", "b", "c"]);
        assert!(super::move_favourite(&mut ids, "a", "c"));
        assert_eq!(ids, ["d", "b", "c", "a"]);
        assert!(!super::move_favourite(&mut ids, "x", "a"));
    }

    #[test]
    fn a_new_person_starts_with_the_starting_favourites() {
        use super::{from_text, STARTING};
        assert_eq!(from_text(""), STARTING);
        let stick = from_text("trindle-installer.desktop\n");
        assert_eq!(&stick[..STARTING.len()], STARTING);
        assert_eq!(stick.last().map(String::as_str), Some("trindle-installer.desktop"));
    }

    #[test]
    fn a_favourite_taken_off_stays_off() {
        use super::{from_text, HEADER};
        let text = format!("{HEADER}\nfirefox-esr.desktop\ntrindle-drives.desktop\n");
        assert_eq!(from_text(&text), ["firefox-esr.desktop", "trindle-drives.desktop"]);
    }
}
