//! The switch Linux keeps for every radio (rfkill).

use std::io::Write;
use std::path::Path;

pub const DEVICE: &str = "/dev/rfkill";
pub const STATES: &str = "/sys/class/rfkill";

const CHANGE_ALL: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Radio {
    Wifi = 1,
    Bluetooth = 2,
}

impl Radio {
    fn sys_name(self) -> &'static str {
        match self {
            Radio::Wifi => "wlan",
            Radio::Bluetooth => "bluetooth",
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Radio::Wifi => "wifi",
            Radio::Bluetooth => "bluetooth",
        }
    }

    pub fn from_word(word: &str) -> Option<Radio> {
        [Radio::Wifi, Radio::Bluetooth].into_iter().find(|r| r.word() == word)
    }

    pub fn name(self) -> &'static str {
        match self {
            Radio::Wifi => "Wi-Fi",
            Radio::Bluetooth => "Bluetooth",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Block {
    Clear,
    Soft,
    Hard,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Refused {
    NotAllowed,
    Other(String),
}

pub fn unblock_event(radio: Radio) -> [u8; 8] {
    let mut event = [0u8; 8];
    event[..4].copy_from_slice(&0u32.to_ne_bytes());
    event[4] = radio as u8;
    event[5] = CHANGE_ALL;
    event[6] = 0;
    event[7] = 0;
    event
}

pub fn state_in(states: &Path, radio: Radio) -> Option<Block> {
    let is = |dir: &Path, file: &str, want: &str| std::fs::read_to_string(dir.join(file)).is_ok_and(|t| t.trim() == want);
    let mut found = None;
    for entry in std::fs::read_dir(states).into_iter().flatten().flatten() {
        let dir = entry.path();
        if !is(&dir, "type", radio.sys_name()) {
            continue;
        }
        let now = if is(&dir, "hard", "1") {
            Block::Hard
        } else if is(&dir, "soft", "1") {
            Block::Soft
        } else {
            Block::Clear
        };
        found = Some(match (found, now) {
            (Some(Block::Hard), _) | (_, Block::Hard) => Block::Hard,
            (Some(Block::Soft), _) | (_, Block::Soft) => Block::Soft,
            _ => Block::Clear,
        });
    }
    found
}

pub fn state(radio: Radio) -> Option<Block> {
    state_in(Path::new(STATES), radio)
}

pub fn unblock_at(device: &Path, radio: Radio) -> Result<(), Refused> {
    let refused = |e: std::io::Error| match e.kind() {
        std::io::ErrorKind::PermissionDenied => Refused::NotAllowed,
        _ => Refused::Other(e.to_string()),
    };
    let mut file = std::fs::OpenOptions::new().write(true).open(device).map_err(refused)?;
    file.write_all(&unblock_event(radio)).map_err(refused)
}

pub fn unblock(radio: Radio) -> Result<(), Refused> {
    unblock_at(Path::new(DEVICE), radio)
}

pub fn hard_words(radio: Radio) -> String {
    crate::toolkit::t!("settings-network-rfkill-hard", radio = radio.name())
}

pub fn refused_words(radio: Radio) -> String {
    crate::toolkit::t!("settings-network-rfkill-refused", radio = radio.name())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn made_up(radios: &[(&str, &str, &str)]) -> std::path::PathBuf {
        static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let at = std::env::temp_dir().join(format!("trindle-rfkill-{}-{}", std::process::id(),
                                                   N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        for (i, (kind, soft, hard)) in radios.iter().enumerate() {
            let dir = at.join(format!("rfkill{i}"));
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("type"), format!("{kind}\n")).unwrap();
            std::fs::write(dir.join("soft"), format!("{soft}\n")).unwrap();
            std::fs::write(dir.join("hard"), format!("{hard}\n")).unwrap();
        }
        std::fs::create_dir_all(&at).unwrap();
        at
    }

    #[test]
    fn the_unblock_is_the_kernels_eight_bytes() {
        assert_eq!(unblock_event(Radio::Bluetooth), [0, 0, 0, 0, 2, 3, 0, 0]);
        assert_eq!(unblock_event(Radio::Wifi), [0, 0, 0, 0, 1, 3, 0, 0]);
        assert_eq!(std::mem::size_of_val(&unblock_event(Radio::Wifi)), 8);
    }

    #[test]
    fn the_unblock_is_written_whole_to_the_device_given() {
        let at = made_up(&[]);
        let device = at.join("device");
        std::fs::write(&device, b"").unwrap();
        assert_eq!(unblock_at(&device, Radio::Bluetooth), Ok(()));
        assert_eq!(std::fs::read(&device).unwrap(), [0, 0, 0, 0, 2, 3, 0, 0]);
        assert!(matches!(unblock_at(&at.join("none"), Radio::Bluetooth), Err(Refused::Other(_))));
        std::fs::remove_dir_all(&at).unwrap();
    }

    #[test]
    fn a_radio_is_read_as_clear_soft_or_hard_and_hard_wins() {
        let at = made_up(&[("wlan", "0", "0"), ("bluetooth", "1", "0")]);
        assert_eq!(state_in(&at, Radio::Bluetooth), Some(Block::Soft));
        assert_eq!(state_in(&at, Radio::Wifi), Some(Block::Clear));
        std::fs::remove_dir_all(&at).unwrap();
        let at = made_up(&[("bluetooth", "1", "0"), ("bluetooth", "0", "1")]);
        assert_eq!(state_in(&at, Radio::Bluetooth), Some(Block::Hard));
        assert_eq!(state_in(&at, Radio::Wifi), None, "no Wi-Fi at all is not the same as Wi-Fi unblocked");
        std::fs::remove_dir_all(&at).unwrap();
        assert_eq!(state_in(Path::new("/no/such/folder/here"), Radio::Bluetooth), None);
    }

    #[test]
    fn the_words_say_what_the_person_can_do() {
        assert_eq!(hard_words(Radio::Bluetooth),
                   "Bluetooth is switched off by a switch or a key on the computer itself. Turn that on, then try again.");
        assert!(hard_words(Radio::Wifi).starts_with("Wi-Fi is switched off by a switch"));
        for words in [hard_words(Radio::Bluetooth), refused_words(Radio::Bluetooth)] {
            assert!(!words.to_lowercase().contains("rfkill"), "{words}");
        }
        assert_eq!(Radio::from_word("bluetooth"), Some(Radio::Bluetooth));
        assert_eq!(Radio::from_word("wifi"), Some(Radio::Wifi));
        assert_eq!(Radio::from_word("all"), None);
    }
}
