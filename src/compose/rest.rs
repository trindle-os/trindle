//! A screen at rest. A screen whose picture has not changed for a moment is
//! no longer looked at every frame: it rests, and is looked at again the
//! moment anything at all happens in the compositor (a program draws, the
//! mouse moves, a key is pressed, a timer goes off), and once a second
//! whatever happens. Looking at it every frame, sixty or more times a second
//! for every screen, kept a processor awake all day to find nothing new
//! (about a twentieth of a processor on a laptop left alone, screen on or
//! off).

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
        let said = std::env::var("TRINDLE_REST")
            .ok()
            .or_else(|| std::fs::read_to_string(SWITCH).ok())
            .unwrap_or_default();
        said.split_whitespace().next().unwrap_or_default().to_lowercase()
    })
}

pub const LOG: &str = "trindle_rest";

pub const SWITCH: &str = "/etc/trindle/rest";

pub fn allowed() -> bool {
    allowed_with(word())
}

fn allowed_with(said: &str) -> bool {
    !is_off(said)
}

pub fn told() -> bool {
    word() == "tell"
}

fn is_off(word: &str) -> bool {
    matches!(word, "off" | "no" | "0" | "false")
}

pub const A_FRAME: Duration = Duration::from_millis(16);

pub fn sleep() -> Duration {
    if allowed() { LOOK_AT_LEAST } else { A_FRAME }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn screens_rest_unless_told_not_to() {
        for w in ["", "on", "yes", "1", "true", "tell", "maybe"] {
            assert!(allowed_with(w), "{w}");
        }
        for w in ["off", "no", "0", "false"] {
            assert!(!allowed_with(w), "{w}");
        }
    }
}
