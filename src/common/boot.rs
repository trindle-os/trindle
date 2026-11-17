//! How long this computer took to start, for anyone curious how quick it is,
//! kept for Settings to show.

pub const WAITED: &str = "/run/trindle-unlock-wait";

pub const BEFORE_LINUX: &str = "/run/trindle-before-linux";

fn read_seconds(path: &str) -> Option<f64> {
    std::fs::read_to_string(path).ok()?.split_whitespace().next()?.parse().ok()
}

pub fn from_stick() -> bool {
    std::fs::read_to_string("/proc/cmdline").is_ok_and(|c| c.split_whitespace().any(|w| w == "boot=live"))
}

pub fn seconds() -> Option<f64> {
    if from_stick() {
        return None;
    }
    let up = read_seconds("/proc/uptime")?;
    Some((before_linux() + up - read_seconds(WAITED).unwrap_or(0.0)).max(0.0))
}

fn before_linux() -> f64 {
    if let Some(s) = read_seconds(BEFORE_LINUX) {
        return s;
    }
    let s = from_firmware().unwrap_or(0.0);

    let _ = std::fs::write(BEFORE_LINUX, format!("{s:.3}\n"));
    s
}

const FIRMWARE_HANDED_OVER: &str = "/sys/firmware/acpi/fpdt/boot/exitbootservices_end_ns";

fn from_firmware() -> Option<f64> {
    nanoseconds_file(std::path::Path::new(FIRMWARE_HANDED_OVER))
}

fn nanoseconds_file(path: &std::path::Path) -> Option<f64> {
    let s = std::fs::read_to_string(path).ok()?.trim().parse::<u64>().ok()? as f64 / 1e9;
    (s > 0.0 && s < 600.0).then_some(s)
}

pub const HISTORY: &str = "/var/lib/trindle/start-times";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Start {

    pub at: i64,
    pub took: f64,
}

pub fn record(took: f64) {
    let Ok(id) = std::fs::read_to_string("/proc/sys/kernel/random/boot_id") else { return };
    let id = id.trim();
    if std::fs::read_to_string(HISTORY).is_ok_and(|h| h.lines().any(|l| l.ends_with(id))) {
        return;
    }
    let (Some(up), Ok(now)) = (read_seconds("/proc/uptime"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH))
    else { return };

    let at = now.as_secs_f64() - before_linux() - up;
    if let Some(dir) = std::path::Path::new(HISTORY).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(HISTORY) {
        let _ = writeln!(f, "{at:.0} {took:.2} {id}");
    }
}

pub fn history() -> Vec<Start> {
    parse_history(&std::fs::read_to_string(HISTORY).unwrap_or_default())
}

fn parse_history(text: &str) -> Vec<Start> {
    text.lines().filter_map(|l| {
        let mut w = l.split_whitespace();
        Some(Start { at: w.next()?.parse().ok()?, took: w.next()?.parse().ok()? })
    }).collect()
}

pub fn words(seconds: f64) -> String {
    use trindle_toolkit::lang::format::decimal;
    if seconds < 60.0 {
        trindle_toolkit::t!("settings-about-boot-seconds", n = decimal(seconds, 1))
    } else {
        let whole = (seconds * 10.0).round() as u64;
        let tenths = decimal((whole % 10) as f64 / 10.0, 1);
        let n = format!("{}:{:02}{}", whole / 600, whole / 10 % 60, tenths.trim_start_matches('0'));
        trindle_toolkit::t!("settings-about-boot-minutes", n = n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_firmware_time_is_read_from_the_kernels_file() {
        let dir = std::env::temp_dir().join(format!("trindle-boot-fpdt-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for (text, want) in [("4320000000\n", Some(4.32)), ("0\n", None), ("99999999999999\n", None), ("junk", None), ("-5", None)] {
            std::fs::write(dir.join("ns"), text).unwrap();
            assert_eq!(super::nanoseconds_file(&dir.join("ns")), want, "{text:?}");
        }
        assert_eq!(super::nanoseconds_file(&dir.join("none")), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_history_reads_back() {
        let h = super::parse_history("1790000000 19.90 abc\nbroken\n1790003600 18.25 def\n");
        assert_eq!(h, vec![super::Start { at: 1790000000, took: 19.9 }, super::Start { at: 1790003600, took: 18.25 }]);
    }

    const ENGLISH: &str = "settings-about-boot-seconds = { $n } seconds\nsettings-about-boot-minutes = { $n } minutes\n";

    #[test]
    fn a_time_reads_plainly() {
        use trindle_toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        with_catalogue(Catalogue::new(Bundle::from_text("en", ENGLISH), english()), || {
            assert_eq!(super::words(9.84), "9.8 seconds");
            assert_eq!(super::words(12.0), "12.0 seconds");
            assert_eq!(super::words(65.24), "1:05.2 minutes");
            assert_eq!(super::words(299.96), "5:00.0 minutes");
            assert_eq!(super::words(0.04), "0.0 seconds");
        });
    }

    #[test]
    fn a_time_follows_the_languages_decimal_mark_and_words() {
        use trindle_toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let made_up = "kit-number-decimal = ,\nsettings-about-boot-seconds = { $n } zekunda\nsettings-about-boot-minutes = { $n } minutes\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(super::words(9.84), "9,8 zekunda");
            assert_eq!(super::words(65.24), "1:05,2 minutes");
        });
    }
}
