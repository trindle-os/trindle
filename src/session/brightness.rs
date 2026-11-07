//! trindle-brightness up | down | PERCENT.

use std::sync::OnceLock;

use zbus::blocking::Connection;
use zbus::zvariant::OwnedObjectPath;

const LOGIN: &str = "org.freedesktop.login1";

const STEP: f64 = 0.05;
const LEAST: f64 = 0.01;

pub use crate::common::levels::backlight;

fn bus() -> Option<&'static Connection> {
    static BUS: OnceLock<Option<Connection>> = OnceLock::new();
    BUS.get_or_init(|| Connection::system().ok()).as_ref()
}

fn session(bus: &Connection) -> String {
    std::env::var("XDG_SESSION_ID").ok().filter(|id| !id.is_empty())
        .and_then(|id| bus.call_method(Some(LOGIN), "/org/freedesktop/login1", Some("org.freedesktop.login1.Manager"),
                                       "GetSession", &(id,)).ok())
        .and_then(|m| m.body().deserialize::<OwnedObjectPath>().ok())
        .map(|p| p.to_string())
        .unwrap_or_else(|| "/org/freedesktop/login1/session/auto".into())
}

pub fn set(name: &str, value: u32) -> Result<(), String> {
    let bus = bus().ok_or("no system bus")?;
    bus.call_method(Some(LOGIN), session(bus).as_str(), Some("org.freedesktop.login1.Session"), "SetBrightness",
                    &("backlight", name, value))
        .map(|_| ()).map_err(|e| e.to_string())
}

fn target(word: &str, now: u32, max: u32) -> Option<u32> {
    let max_f = max as f64;
    let share = match word {
        "up" => now as f64 / max_f + STEP,
        "down" => now as f64 / max_f - STEP,
        n => n.trim_end_matches('%').parse::<f64>().ok()? / 100.0,
    };
    Some((share.clamp(LEAST, 1.0) * max_f).round() as u32)
}

pub fn main(args: Vec<String>) -> i32 {
    let Some(word) = args.get(1) else {
        eprintln!("usage: trindle-brightness up | down | PERCENT");
        return 2;
    };
    let Some((name, now, max)) = backlight() else { return 0 };
    let Some(value) = target(word, now, max) else {
        eprintln!("trindle-brightness: {word} is not up, down or a percentage");
        return 2;
    };
    match set(&name, value) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-brightness: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::target;

    #[test]
    fn steps_stay_between_dim_and_full() {
        assert_eq!(target("up", 100, 100), Some(100));
        assert_eq!(target("down", 3, 100), Some(1));
        assert_eq!(target("up", 50, 100), Some(55));
        assert_eq!(target("40%", 0, 200), Some(80));
        assert_eq!(target("sideways", 0, 100), None);
    }
}
