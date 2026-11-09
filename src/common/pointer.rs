//! Mouse and touchpad choices, kept in trindle.conf, where Settings writes
//! them and the compositor reads them.

use std::collections::BTreeMap;

use super::files::{default_setting, read_env, read_text, trindle_conf};

pub const DOUBLE_CLICK_MS: u32 = 400;

pub const DOUBLE_CLICK_SLOP: f64 = 8.0;

pub const DOUBLE_TAP_SLOP: f64 = 32.0;

pub const SPEED: &str = "pointer_speed";

pub const SCROLL: &str = "scroll_speed";

pub const TOUCHPAD_SCROLL: &str = "touchpad_scroll_speed";
pub const MOUSE_NATURAL: &str = "mouse_natural_scroll";
pub const LEFT_HANDED: &str = "left_handed";
pub const TAP: &str = "touchpad_tap";
pub const TOUCHPAD_NATURAL: &str = "touchpad_natural_scroll";
pub const QUIET_TYPING: &str = "touchpad_off_while_typing";

pub struct Range {
    pub least: f64,
    pub most: f64,
    pub step: f64,
}

pub const SPEEDS: Range = Range { least: -1.0, most: 1.0, step: 0.1 };

pub const SCROLLS: Range = Range { least: 0.25, most: 6.0, step: 0.25 };

pub const TOUCHPAD_SCROLLS: Range = Range { least: 0.25, most: 4.0, step: 0.25 };

pub const WHEEL_PACES: [f64; 3] = [2.0, 4.0, 6.0];

pub const TOUCHPAD_PACES: [f64; 3] = [1.0, 1.5, 2.5];

#[derive(Clone, Debug)]
pub struct Pointer {
    pub speed: f64,
    pub scroll: f64,
    pub touchpad_scroll: f64,
    pub mouse_natural: bool,
    pub left_handed: bool,
    pub tap: bool,
    pub touchpad_natural: bool,
    pub quiet_typing: bool,
}

impl Pointer {
    pub fn load() -> Pointer {
        Pointer::from_conf(&read_env(&trindle_conf()))
    }

    fn from_conf(conf: &BTreeMap<String, String>) -> Pointer {
        let get = |k: &str| conf.get(k).map(String::as_str).unwrap_or_else(|| default_setting(k));
        let flag = |k: &str| get(k) == "yes";
        let read = |k: &str| conf.get(k).and_then(|v| v.parse::<f64>().ok()).filter(|v| v.is_finite());
        let number = |k: &str, r: &Range| {
            read(k).or_else(|| default_setting(k).parse().ok()).unwrap_or(0.0).clamp(r.least, r.most)
        };

        let touchpad_scroll = read(TOUCHPAD_SCROLL).or_else(|| read(SCROLL))
            .or_else(|| default_setting(TOUCHPAD_SCROLL).parse().ok())
            .unwrap_or(1.0)
            .clamp(TOUCHPAD_SCROLLS.least, TOUCHPAD_SCROLLS.most);
        Pointer {
            speed: number(SPEED, &SPEEDS),
            scroll: number(SCROLL, &SCROLLS),
            touchpad_scroll,
            mouse_natural: flag(MOUSE_NATURAL),
            left_handed: flag(LEFT_HANDED),
            tap: flag(TAP),
            touchpad_natural: flag(TOUCHPAD_NATURAL),
            quiet_typing: flag(QUIET_TYPING),
        }
    }

}

pub fn has_touchpad() -> bool {
    touchpad_in(&read_text(std::path::Path::new("/proc/bus/input/devices")))
}

const BTN_TOOL_FINGER: usize = 0x145;

const PROP_POINTER: usize = 0;
const PROP_DIRECT: usize = 1;

fn touchpad_in(devices: &str) -> bool {
    devices.split("\n\n").any(|dev| {
        let (mut name, mut keys, mut props) = ("", "", "");
        for line in dev.lines() {
            if let Some(v) = line.strip_prefix("N: Name=") {
                name = v;
            } else if let Some(v) = line.strip_prefix("B: KEY=") {
                keys = v;
            } else if let Some(v) = line.strip_prefix("B: PROP=") {
                props = v;
            }
        }
        name.to_lowercase().contains("touchpad")
            || (bit(keys, BTN_TOOL_FINGER) && bit(props, PROP_POINTER) && !bit(props, PROP_DIRECT))
    })
}

fn bit(words: &str, n: usize) -> bool {
    let words: Vec<&str> = words.split_whitespace().collect();
    let per = usize::BITS as usize;
    let i = n / per;
    i < words.len()
        && u64::from_str_radix(words[words.len() - 1 - i], 16).is_ok_and(|w| (w >> (n % per)) & 1 == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_leave_tapping_and_quiet_typing_on() {
        let p = Pointer::from_conf(&BTreeMap::new());
        assert!(p.tap && p.quiet_typing && !p.left_handed);
        assert_eq!((p.speed, p.scroll, p.touchpad_scroll), (0.0, WHEEL_PACES[1], TOUCHPAD_PACES[1]));
    }

    #[test]
    fn normal_is_quicker_than_the_device_itself() {
        const _: () = assert!(WHEEL_PACES[1] > 1.0 && TOUCHPAD_PACES[1] > 1.0);
        for (paces, r) in [(WHEEL_PACES, &SCROLLS), (TOUCHPAD_PACES, &TOUCHPAD_SCROLLS)] {
            assert!(paces[0] < paces[1] && paces[1] < paces[2]);

            assert!(paces.iter().all(|p| (r.least..=r.most).contains(p) && ((p - r.least) / r.step).fract() == 0.0));
        }

        assert!(WHEEL_PACES.iter().all(|p| p.fract() == 0.0));
    }

    #[test]
    fn an_earlier_scroll_speed_carries_over_to_the_touchpad() {
        let conf = |pairs: &[(&str, &str)]| -> BTreeMap<String, String> {
            pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        };
        let p = Pointer::from_conf(&conf(&[(SCROLL, "3.00")]));
        assert_eq!((p.scroll, p.touchpad_scroll), (3.0, 3.0));
        let p = Pointer::from_conf(&conf(&[(SCROLL, "5"), (TOUCHPAD_SCROLL, "1.25")]));
        assert_eq!((p.scroll, p.touchpad_scroll), (5.0, 1.25));
        let p = Pointer::from_conf(&conf(&[(SCROLL, "9"), (TOUCHPAD_SCROLL, "nonsense")]));
        assert_eq!((p.scroll, p.touchpad_scroll), (SCROLLS.most, TOUCHPAD_SCROLLS.most));
    }

    #[test]
    fn choices_are_written_and_kept_in_range() {
        let conf: BTreeMap<String, String> = [(SPEED, "5"), (SCROLL, "nonsense"), (LEFT_HANDED, "yes"), (TAP, "no")]
            .iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        let p = Pointer::from_conf(&conf);
        assert_eq!(p.speed, 1.0);
        assert_eq!(p.scroll, WHEEL_PACES[1]);
        assert!(p.left_handed && !p.tap);
    }

    #[test]
    fn finds_a_touchpad_but_not_a_keyboard_claiming_fingers() {
        let touchpad = "I: Bus=0018\nN: Name=\"PIXA3854:00 093A:0274 Pad\"\nB: PROP=5\nB: EV=1b\n\
                        B: KEY=e520 30000 0 0 0 0\nB: ABS=2e0800000000003\n";
        let keyboard = "I: Bus=0003\nN: Name=\"Logitech K780\"\nB: PROP=0\nB: EV=12001f\n\
                        B: KEY=3f00033fff 0 0 483ffff17aff32d bfd4444600000000 ffff0001 130ff38b17d007 \
                        ffff7bfad941dfff ffbeffdfffefffff fffffffffffffffe\n";
        assert!(touchpad_in(touchpad));
        assert!(!touchpad_in(keyboard));
        assert!(touchpad_in(&format!("{keyboard}\nN: Name=\"SynPS/2 Synaptics TouchPad\"\n")));
        assert!(!touchpad_in(""));
    }
}
