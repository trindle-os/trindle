//! A screen at rest.

use std::time::Duration;

pub const REST_AFTER: Duration = Duration::from_millis(500);

pub const LOOK_AT_LEAST: Duration = Duration::from_secs(1);

pub fn rests(still_for: Duration) -> bool {
    still_for >= REST_AFTER
}

pub fn wait(frame: Duration, since_looked: Duration) -> Duration {
    frame.saturating_sub(since_looked)
}

fn word() -> &'static str {
    static WORD: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    WORD.get_or_init(|| {
        if once(&crate::files::config_path(ONCE)) {
            return "on".to_string();
        }
        let said = std::env::var("TRINDLE_REST")
            .ok()
            .or_else(|| std::fs::read_to_string(SWITCH).ok())
            .unwrap_or_default();
        said.split_whitespace().next().unwrap_or_default().to_lowercase()
    })
}

pub const ONCE: &str = "rest-once";

fn once(file: &std::path::Path) -> bool {
    file.is_file() && std::fs::remove_file(file).is_ok()
}

pub const LOG: &str = "trindle_rest";

pub const SWITCH: &str = "/etc/trindle/rest";

pub fn allowed() -> bool {
    allowed_with(word(), DRAWN_BY_PROCESSOR.load(std::sync::atomic::Ordering::Relaxed))
}

fn allowed_with(said: &str, by_processor: bool) -> bool {
    (said.is_empty() && by_processor) || !is_off(said)
}

static DRAWN_BY_PROCESSOR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn drawn_by_processor() {
    DRAWN_BY_PROCESSOR.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn told() -> bool {
    word() == "tell"
}

fn is_off(word: &str) -> bool {
    !matches!(word, "on" | "yes" | "1" | "true" | "tell")
}

pub const A_FRAME: Duration = Duration::from_millis(16);

pub fn sleep() -> Duration {
    if allowed() { LOOK_AT_LEAST } else { A_FRAME }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_one_session_file_counts_once_and_is_gone() {
        let dir = std::env::temp_dir().join(format!("trindle-rest-once-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join(ONCE);
        assert!(!once(&file), "no file, no trial");
        std::fs::write(&file, "").unwrap();
        assert!(once(&file), "there: resting for this session");
        assert!(!file.exists() && !once(&file), "and gone, so the next start is as before");
        std::fs::create_dir(&file).unwrap();
        assert!(!once(&file));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_screen_rests_only_after_staying_the_same_a_while() {
        assert!(!rests(Duration::from_millis(16)));
        assert!(!rests(REST_AFTER - Duration::from_millis(1)));
        assert!(rests(REST_AFTER));
    }

    #[test]
    fn a_resting_screen_is_looked_at_once_a_frame_at_most() {
        let frame = Duration::from_micros(16_667);
        assert_eq!(wait(frame, Duration::from_millis(40)), Duration::ZERO);
        assert_eq!(wait(frame, Duration::from_micros(6_667)), Duration::from_millis(10));
        assert_eq!(wait(frame, Duration::ZERO), frame);
    }

    #[test]
    fn where_the_processor_draws_screens_rest_unless_told_not_to() {
        assert!(allowed_with("", true));
        assert!(!allowed_with("", false));
        assert!(!allowed_with("off", true));
        assert!(!allowed_with("maybe", true));
        assert!(allowed_with("on", false));
    }

    #[test]
    fn resting_is_off_until_it_is_switched_on_in_plain_words() {
        for w in ["", "off", "no", "0", "false", "maybe"] {
            assert!(is_off(w), "{w}");
        }
        for w in ["tell", "on", "yes", "1", "true"] {
            assert!(!is_off(w), "{w}");
        }
    }
}
