//! How a snapshot is named for a person.

use crate::common::clock::{civil, Local};
use crate::common::home_snapshots::unix_of;

const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

pub fn what_it_is(kind: &str, description: &str) -> String {
    match (kind, description.trim()) {
        ("pre", "apt") => "before a software change".into(),
        ("post", "apt") => "after a software change".into(),
        (_, "boot") => "when the computer started".into(),
        (_, "timeline") => "a regular copy".into(),
        (_, "") => "a snapshot".into(),
        (_, d) => d.into(),
    }
}

pub fn what_alone(kind: &str, description: &str) -> String {
    let d = description.trim();
    let known = matches!((kind, d), ("pre" | "post", "apt") | (_, "boot" | "timeline" | ""));
    let text = what_it_is(kind, d);
    if !known {
        return text;
    }
    let mut letters = text.chars();
    letters.next().map(|f| f.to_uppercase().chain(letters).collect()).unwrap_or(text)
}

pub fn clock_label(unix: i64, offset: i64) -> String {
    let local = unix + offset;
    let days = local.div_euclid(86_400);
    let secs = local.rem_euclid(86_400);
    let (_, month, day) = civil(days);
    let weekday = (days + 4).rem_euclid(7) as usize;
    format!("{} {day} {}, {:02}:{:02}", DAYS[weekday], MONTHS[(month - 1) as usize], secs / 3600, secs / 60 % 60)
}

pub fn clock_words(unix: i64, offset: i64) -> String {
    use trindle_toolkit::lang::format::{date, Pattern};
    date(&crate::common::clock::moment_at(unix, offset), Pattern::ShortDayClock)
}

pub fn local_label(snapper_date: &str) -> Option<String> {
    let unix = unix_of(snapper_date)?;
    Some(clock_label(unix, Local::at(unix).utc_offset()))
}

pub fn local_words(snapper_date: &str) -> Option<String> {
    let unix = unix_of(snapper_date)?;
    Some(clock_words(unix, Local::at(unix).utc_offset()))
}

pub fn title(number: u32, snapper_date: &str, kind: &str, description: &str) -> String {
    let when = local_label(snapper_date).unwrap_or_else(|| format!("Snapshot {number}"));
    format!("{when}, {}", what_it_is(kind, description))
}

pub struct Moment {
    pub day_number: i64,
    pub date: String,
    pub time: String,
}

pub fn moment_at(unix: i64, offset: i64) -> Moment {
    use trindle_toolkit::lang::format::{date, Pattern};
    let m = crate::common::clock::moment_at(unix, offset);
    Moment {
        day_number: (unix + offset).div_euclid(86_400),
        date: date(&m, Pattern::DayShort),
        time: date(&m, Pattern::Clock),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZONES: [(&str, i64, i64); 4] = [
        ("Britain-like", 0, 3600),
        ("Nepal-like", 20_700, 20_700),
        ("Newfoundland-like", -12_600, -9_000),
        ("Lord-Howe-like", 37_800, 39_600),
    ];

    const SAMPLES: &str = include_str!("../../system/grub/snapshot-title-samples.txt");

    #[test]
    fn every_sample_reads_the_same() {
        let mut checked = 0;
        for line in SAMPLES.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
            let p: Vec<&str> = line.split('|').map(str::trim).collect();
            assert_eq!(p.len(), 4, "{line}");
            let zone = ZONES.iter().find(|z| z.0 == p[0]).unwrap_or_else(|| panic!("no zone {}", p[0]));
            let offset = if p[2] == "summer" { zone.2 } else { zone.1 };
            assert_eq!(clock_label(unix_of(p[1]).unwrap(), offset), p[3], "{line}");
            checked += 1;
        }
        assert!(checked >= 12);
    }

    #[test]
    fn a_title_names_the_time_and_what_it_is() {
        let t = title(7, "2026-10-04 09:34:12", "single", "boot");
        assert!(t.ends_with(", when the computer started"), "{t}");
        assert!(!t.contains("UTC"));
        assert!(title(7, "2026-10-04 09:34:12", "pre", "apt").ends_with(", before a software change"));
        assert!(title(7, "2026-10-04 09:34:12", "post", "apt").ends_with(", after a software change"));
        assert_eq!(title(9, "not a date", "pre", "apt"), "Snapshot 9, before a software change");
        assert_eq!(what_alone("pre", "apt"), "Before a software change");
        assert_eq!(what_alone("single", "my note"), "my note");
    }

    #[test]
    fn a_list_of_snapshots_across_the_clocks_change_is_named_in_order() {
        let list = [
            ("2026-10-24 22:30:00", "summer", "Sat 24 Oct, 23:30"),
            ("2026-10-25 00:30:00", "summer", "Sun 25 Oct, 01:30"),
            ("2026-10-25 02:30:00", "winter", "Sun 25 Oct, 02:30"),
            ("2026-12-31 23:59:00", "winter", "Thu 31 Dec, 23:59"),
        ];
        let zone = ZONES[0];
        for (date, season, want) in list {
            let offset = if season == "summer" { zone.2 } else { zone.1 };
            assert_eq!(clock_label(unix_of(date).unwrap(), offset), want);
        }
    }

    #[test]
    fn the_day_and_time_for_the_settings_page_follow_the_clock() {
        let m = moment_at(unix_of("2026-10-04 23:30:00").unwrap(), 3600);
        assert_eq!((m.date.as_str(), m.time.as_str()), ("5 Oct", "00:30"));
        let before = moment_at(unix_of("2026-10-04 23:30:00").unwrap(), 0);
        assert_eq!(m.day_number, before.day_number + 1);
    }

    #[test]
    fn a_person_reads_the_language_and_the_start_up_menu_stays_english() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let made_up = "kit-date-weekday-short-sun = Zun\nkit-date-month-short-oct = Blu\n\
                       kit-date-pattern-short-day-clock = %a %b %-d, %H.%M\nkit-date-pattern-day-short = %b %-d\n\
                       kit-date-pattern-clock = %H.%M\n";
        let unix = unix_of("2026-10-04 09:34:12").unwrap();
        assert_eq!(clock_words(unix, 3600), "Sun 4 Oct, 10:34");
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(clock_words(unix, 3600), "Zun Blu 4, 10.34");
            assert_eq!(clock_label(unix, 3600), "Sun 4 Oct, 10:34");
            let m = moment_at(unix, 3600);
            assert_eq!((m.date.as_str(), m.time.as_str()), ("Blu 4", "10.34"));
        });
    }
}
