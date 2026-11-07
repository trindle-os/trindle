//! Event sounds, synthesised at runtime and played with pw-play.

use std::f64::consts::PI;
use std::path::PathBuf;

use super::files::setting;
use crate::toolkit::t;
use super::run::{capture, on_thread};

pub const SOUNDS: &[&str] = &["open", "close", "sunrise", "sunset", "rising", "falling", "alert", "bell", "glass", "pop"];

pub fn sound_name(id: &str) -> &'static str {
    match id {
        "open" => t!("settings-sounds-name-open"),
        "close" => t!("settings-sounds-name-close"),
        "sunrise" => t!("settings-sounds-name-sunrise"),
        "sunset" => t!("settings-sounds-name-sunset"),
        "rising" => t!("settings-sounds-name-rising"),
        "falling" => t!("settings-sounds-name-falling"),
        "alert" => t!("settings-sounds-name-alert"),
        "bell" => t!("settings-sounds-name-bell"),
        "glass" => t!("settings-sounds-name-glass"),
        "pop" => t!("settings-sounds-name-pop"),
        _ => "",
    }
}

pub const OFF: &str = "off";

pub const MOMENTS: &[(&str, &str)] = &[
    ("login", "open"),
    ("logout", "close"),
    ("plug", "rising"),
    ("unplug", "falling"),
    ("battery", "alert"),
    ("notification", "pop"),
];

pub fn moment_label(moment: &str) -> &'static str {
    match moment {
        "login" => t!("settings-sounds-moment-login"),
        "logout" => t!("settings-sounds-moment-logout"),
        "plug" => t!("settings-sounds-moment-plug"),
        "unplug" => t!("settings-sounds-moment-unplug"),
        "battery" => t!("settings-sounds-moment-battery"),
        "notification" => t!("settings-sounds-moment-notification"),
        _ => "",
    }
}

pub const VOLUMES: &[(&str, &[&str])] = &[
    ("coming", &["login", "logout"]),
    ("power", &["plug", "unplug", "battery"]),
    ("notification", &["notification"]),
];

pub const VOLUME_STEP: u32 = 10;
pub const VOLUME_FULL: u32 = 100;

pub fn clamp_volume(v: i64) -> u32 {
    let step = VOLUME_STEP as i64;
    ((v.clamp(0, VOLUME_FULL as i64) + step / 2) / step * step) as u32
}

pub fn volume_key(group: &str) -> String {
    format!("sound_volume_{group}")
}

pub fn volume(group: &str) -> u32 {
    setting(&volume_key(group)).trim().parse::<i64>().map(clamp_volume).unwrap_or(VOLUME_FULL)
}

pub fn volume_of(moment: &str) -> u32 {
    VOLUMES.iter().find(|(_, m)| m.contains(&moment)).map(|(g, _)| volume(g)).unwrap_or(VOLUME_FULL)
}

pub const ROLE: &str = "Notification";

pub fn command(path: &std::path::Path, percent: u32) -> Vec<String> {
    vec!["pw-play".into(), "--media-role".into(), ROLE.into(), "--volume".into(), format!("{:.1}", clamp_volume(percent as i64) as f64 / 100.0),
         path.to_string_lossy().to_string()]
}

pub fn own_dir() -> PathBuf {
    super::files::data_home().join("trindle/sounds")
}

pub fn own() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = std::fs::read_dir(own_dir()).into_iter().flatten().flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| playable(n))
        .map(|n| {
            let shown = n.rsplit_once('.').map(|(stem, _)| stem.to_string()).unwrap_or_else(|| n.clone());
            (format!("own:{n}"), shown)
        })
        .collect();
    out.sort_by_key(|(_, name)| name.to_lowercase());
    out
}

pub fn playable(name: &str) -> bool {
    let lower = name.to_lowercase();
    [".wav", ".ogg", ".oga", ".opus", ".flac", ".mp3"].iter().any(|e| lower.ends_with(e))
}

pub fn chosen(moment: &str) -> Option<String> {
    let saved = setting(&format!("sound_{moment}"));
    if saved == OFF {
        return None;
    }
    let default = MOMENTS.iter().find(|m| m.0 == moment).map(|m| m.1)?;
    if saved.is_empty() {
        return Some(default.to_string());
    }
    if let Some(name) = saved.strip_prefix("own:") {
        return own_dir().join(name).is_file().then(|| saved.clone()).or(Some(default.to_string()));
    }
    Some(SOUNDS.iter().copied().find(|s| *s == saved).unwrap_or(default).to_string())
}

pub fn play(moment: &str) {
    let Some(path) = chosen(moment).and_then(|id| file(&id)) else { return };
    let args = command(&path, volume_of(moment));
    if let Ok(mut child) = std::process::Command::new(&args[0]).args(&args[1..])
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn()
    {
        std::thread::spawn(move || child.wait());
    }
}

pub fn play_then(moment: &str, then: impl FnOnce() + 'static) {
    match chosen(moment) {
        Some(sound) => play_sound(&sound, volume_of(moment), then),
        None => then(),
    }
}

const LONGEST_WAIT: std::time::Duration = std::time::Duration::from_secs(5);

pub fn play_sound(id: &str, percent: u32, then: impl FnOnce() + 'static) {
    let Some(path) = file(id) else {
        then();
        return;
    };
    let once: std::rc::Rc<std::cell::RefCell<Option<Box<dyn FnOnce()>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(then))));
    let late = once.clone();
    on_thread(|| std::thread::sleep(LONGEST_WAIT), move |()| {
        if let Some(go) = late.borrow_mut().take() {
            go();
        }
    });
    let args = command(&path, percent);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    capture(&args, None, move |_, _, _| {
        if let Some(go) = once.borrow_mut().take() {
            go();
        }
    });
}

pub fn file(id: &str) -> Option<PathBuf> {
    if let Some(name) = id.strip_prefix("own:") {
        let path = own_dir().join(name);
        return path.is_file().then_some(path);
    }
    let sounds = super::files::share("sounds");
    for ext in ["flac", "wav"] {
        let recorded = sounds.join(format!("{id}.{ext}"));
        if recorded.is_file() {
            return Some(recorded);
        }
    }
    let samples = make(id)?;
    let dir = super::files::scratch().join("trindle-sounds");
    let path = dir.join(format!("{id}-{}.wav", env!("CARGO_PKG_VERSION")));
    if path.exists() {
        return Some(path);
    }
    let _ = std::fs::create_dir_all(&dir);
    match std::fs::write(&path, wav(&samples)) {
        Ok(()) => Some(path),
        Err(e) => {
            eprintln!("trindle: could not write the sound {id}: {e}");
            None
        }
    }
}

pub const RATE: u32 = 44100;

fn strike(out: &mut [f64], freq: f64, at: f64, amp: f64, decay: f64, partials: &[(f64, f64)]) {
    let start = (at * RATE as f64) as usize;
    for (i, v) in out.iter_mut().enumerate().skip(start) {
        let t = (i - start) as f64 / RATE as f64;
        let attack = (t / 0.004).min(1.0);
        let mut s = 0.0;
        for (k, &(ratio, weight)) in partials.iter().enumerate() {
            s += weight * (-decay * (1.0 + 0.7 * k as f64) * t).exp() * (2.0 * PI * freq * ratio * t).sin();
        }
        *v += amp * attack * s;
    }
}

const BELL: &[(f64, f64)] = &[(1.0, 1.0), (2.0, 0.42), (3.0, 0.2), (4.16, 0.1), (5.43, 0.05)];
const SOFT: &[(f64, f64)] = &[(1.0, 1.0), (2.0, 0.18), (3.0, 0.05)];
const GLASS: &[(f64, f64)] = &[(1.0, 1.0), (2.76, 0.32), (5.4, 0.12)];

fn make(id: &str) -> Option<Vec<f64>> {
    let (seconds, notes): (f64, Vec<(f64, f64, f64, f64, &[(f64, f64)])>) = match id {
        "sunrise" => (2.2, vec![(523.25, 0.0, 0.5, 2.4, BELL), (659.25, 0.14, 0.45, 2.4, BELL),
                                (783.99, 0.28, 0.42, 2.2, BELL), (1046.5, 0.42, 0.4, 2.0, BELL)]),
        "sunset" => (2.2, vec![(1046.5, 0.0, 0.36, 2.6, BELL), (783.99, 0.16, 0.42, 2.4, BELL),
                               (659.25, 0.32, 0.45, 2.4, BELL), (523.25, 0.48, 0.5, 2.2, BELL)]),
        "rising" => (1.0, vec![(783.99, 0.0, 0.5, 4.5, SOFT), (1046.5, 0.12, 0.5, 4.0, SOFT)]),
        "falling" => (1.0, vec![(1046.5, 0.0, 0.5, 4.5, SOFT), (783.99, 0.12, 0.5, 4.0, SOFT)]),
        "alert" => (1.2, vec![(880.0, 0.0, 0.45, 9.0, SOFT), (880.0, 0.22, 0.45, 9.0, SOFT),
                              (880.0, 0.44, 0.45, 7.0, SOFT)]),
        "bell" => (2.0, vec![(659.25, 0.0, 0.6, 2.0, BELL)]),
        "glass" => (1.4, vec![(1318.5, 0.0, 0.45, 3.5, GLASS), (1975.5, 0.09, 0.3, 3.5, GLASS)]),
        "pop" => (0.35, vec![(620.0, 0.0, 0.7, 26.0, SOFT)]),
        _ => return None,
    };
    let mut out = vec![0.0; (seconds * RATE as f64) as usize];
    for (freq, at, amp, decay, partials) in notes {
        strike(&mut out, freq, at, amp, decay, partials);
    }
    let dry = out.clone();
    for (delay, gain) in [(0.037, 0.22), (0.061, 0.14)] {
        let d = (delay * RATE as f64) as usize;
        for i in d..out.len() {
            out[i] += gain * dry[i - d];
        }
    }
    let peak = out.iter().fold(0.0f64, |m, v| m.max(v.abs())).max(1e-9);
    let fade = (0.03 * RATE as f64) as usize;
    let n = out.len();
    for (i, v) in out.iter_mut().enumerate() {
        *v *= 0.55 / peak * ((n - i) as f64 / fade as f64).min(1.0);
    }
    Some(out)
}

fn wav(samples: &[f64]) -> Vec<u8> {
    let mut frames = Vec::with_capacity(samples.len() * 4);
    for v in samples {
        let s = (v.clamp(-1.0, 1.0) * 32000.0) as i16;
        frames.extend_from_slice(&s.to_le_bytes());
        frames.extend_from_slice(&s.to_le_bytes());
    }
    let mut w = Vec::with_capacity(44 + frames.len());
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + frames.len() as u32).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&2u16.to_le_bytes());
    w.extend_from_slice(&RATE.to_le_bytes());
    w.extend_from_slice(&(RATE * 4).to_le_bytes());
    w.extend_from_slice(&4u16.to_le_bytes());
    w.extend_from_slice(&16u16.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    w.extend_from_slice(&frames);
    w
}

#[cfg(test)]
mod tests {
    #[test]
    fn volume_is_whole_tens_from_0_to_100_and_full_by_default() {
        for (given, want) in [(-5, 0), (0, 0), (4, 0), (5, 10), (44, 40), (95, 100), (100, 100), (250, 100)] {
            assert_eq!(super::clamp_volume(given), want, "{given}");
        }
        for (group, _) in super::VOLUMES {
            assert!(super::volume_key(group).starts_with("sound_volume_"));
        }
        for (m, _) in super::MOMENTS {
            assert_eq!(super::VOLUMES.iter().filter(|(_, ms)| ms.contains(m)).count(), 1, "{m}");
        }
        assert_eq!(super::command(std::path::Path::new("/a.wav"), 70), ["pw-play", "--media-role", "Notification", "--volume", "0.7", "/a.wav"]);
        assert_eq!(super::command(std::path::Path::new("/a.wav"), 1000)[4], "1.0");
    }

    #[test]
    fn every_sound_can_be_played_and_every_default_exists() {
        for id in super::SOUNDS {
            match super::make(id) {
                Some(s) => assert!(s.iter().all(|v| v.abs() <= 0.56), "{id} too loud"),
                None => assert!(super::file(id).is_some() || ["open", "close"].contains(id),
                                "{id} is neither made nor recorded"),
            }
        }
        for (_, default) in super::MOMENTS {
            assert!(super::SOUNDS.iter().any(|s| s == default));
        }
    }
}
