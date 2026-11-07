//! How loud the sound is and how bright the screen is, read the one way everywhere.

use std::process::{Command, Stdio};

pub const SPEAKERS: &str = "@DEFAULT_AUDIO_SINK@";
pub const MICROPHONE: &str = "@DEFAULT_AUDIO_SOURCE@";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sound {
    pub share: f32,
    pub muted: bool,
}

pub fn parse_volume(text: &str) -> Option<Sound> {
    let rest = text.trim().strip_prefix("Volume:")?;
    let share: f32 = rest.split_whitespace().next()?.parse().ok().filter(|n: &f32| n.is_finite())?;
    Some(Sound { share, muted: rest.contains("[MUTED]") })
}

pub fn volume(target: &str) -> Option<Sound> {
    let out = Command::new("wpctl").args(["get-volume", target]).stdin(Stdio::null()).stderr(Stdio::null()).output().ok()?;
    if !out.status.success() {
        return None;
    }
    parse_volume(&String::from_utf8_lossy(&out.stdout))
}

pub fn bar_share(share: f32) -> f32 {
    if share.is_finite() { share.clamp(0.0, 1.0) } else { 0.0 }
}

pub fn backlight() -> Option<(String, u32, u32)> {
    let name = std::fs::read_dir("/sys/class/backlight").ok()?.flatten().next()?.file_name().to_string_lossy().to_string();
    let read = |f: &str| std::fs::read_to_string(format!("/sys/class/backlight/{name}/{f}")).ok()?.trim().parse::<u32>().ok();
    Some((name.clone(), read("brightness")?, read("max_brightness")?))
}

pub fn brightness() -> Option<f32> {
    let (_, now, max) = backlight()?;
    (max > 0).then(|| now as f32 / max as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_volume_is_read_from_what_wpctl_says() {
        assert_eq!(parse_volume("Volume: 0.70\n"), Some(Sound { share: 0.70, muted: false }));
        assert_eq!(parse_volume("Volume: 0.35 [MUTED]\n"), Some(Sound { share: 0.35, muted: true }));
        assert_eq!(parse_volume("Volume: 1.20"), Some(Sound { share: 1.2, muted: false }));
        assert_eq!(parse_volume("Could not find node"), None);
        assert_eq!(parse_volume("Volume: loud"), None);
        assert_eq!(parse_volume(""), None);
    }

    #[test]
    fn a_bar_never_fills_past_its_ends() {
        assert_eq!(bar_share(-0.2), 0.0);
        assert_eq!(bar_share(0.0), 0.0);
        assert_eq!(bar_share(0.4), 0.4);
        assert_eq!(bar_share(1.0), 1.0);
        assert_eq!(bar_share(1.5), 1.0);
        assert_eq!(bar_share(f32::NAN), 0.0);
        assert_eq!(bar_share(f32::INFINITY), 0.0);
    }
}
