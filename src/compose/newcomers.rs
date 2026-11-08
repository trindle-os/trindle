//! Keyboards and mice plugged in while the screen is locked do nothing until
//! it is unlocked. A gadget that pretends to be a keyboard cannot start
//! typing at an empty desk.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arrival {

    pub id: String,

    pub key: String,

    pub types: bool,
}

#[derive(Debug, Default)]
pub struct Newcomers {
    known: HashSet<String>,

    present: HashMap<String, Arrival>,

    held: HashSet<String>,
    file: Option<PathBuf>,
}

fn known_file() -> PathBuf {
    crate::files::home().join(".local/state/trindle/known-input")
}

pub fn key_of(usb: Option<(u32, u32)>, name: &str) -> String {
    match usb {
        Some((maker, model)) => format!("{maker:04x}:{model:04x} {name}"),
        None => name.to_string(),
    }
}

impl Newcomers {
    pub fn load() -> Self {
        let file = known_file();
        let known = std::fs::read_to_string(&file)
            .map(|t| t.lines().filter(|l| !l.is_empty()).map(String::from).collect())
            .unwrap_or_default();
        Newcomers { known, file: Some(file), ..Default::default() }
    }

    pub fn added(&mut self, arrival: Arrival, locked: bool) -> bool {
        let known = self.known.contains(&arrival.key);
        let last_keyboard = arrival.types && !self.present.values().any(|a| a.types && self.known.contains(&a.key));
        let hold = locked && !known && !last_keyboard;
        if hold {
            info!(device = %arrival.key, "Held back until the screen is unlocked");
            self.held.insert(arrival.id.clone());
        } else if !known {
            self.remember(&arrival.key);
        }
        self.present.insert(arrival.id.clone(), arrival);
        hold
    }

    pub fn removed(&mut self, id: &str) {
        self.present.remove(id);
        self.held.remove(id);
    }

    pub fn is_held(&self, id: &str) -> bool {
        self.held.contains(id)
    }

    pub fn unlocked(&mut self) {
        let keys: Vec<String> = self
            .held
            .drain()
            .filter_map(|id| self.present.get(&id).map(|a| a.key.clone()))
            .collect();
        for key in keys {
            self.remember(&key);
        }
    }

    fn remember(&mut self, key: &str) {
        if !self.known.insert(key.to_string()) {
            return;
        }
        let Some(file) = &self.file else { return };
        if let Some(dir) = file.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut all: Vec<&String> = self.known.iter().collect();
        all.sort();
        let text = all.into_iter().fold(String::new(), |t, k| t + k + "\n");

        // known before.
        let _ = crate::files::write_text(file, &text);
    }
}

#[cfg(test)]
mod tests {
    use super::{Arrival, Newcomers};

    fn dev(id: &str, key: &str, types: bool) -> Arrival {
        Arrival { id: id.into(), key: key.into(), types }
    }

    #[test]
    fn a_new_keyboard_waits_for_the_unlock() {
        let mut n = Newcomers::default();
        assert!(!n.added(dev("event0", "laptop keys", true), false));
        assert!(n.added(dev("event9", "046d:c31c gadget", true), true));
        assert!(n.is_held("event9"));
        n.unlocked();
        assert!(!n.is_held("event9"));
        n.removed("event9");

        assert!(!n.added(dev("event10", "046d:c31c gadget", true), true));
    }

    #[test]
    fn a_known_keyboard_coming_back_is_let_in() {
        let mut n = Newcomers::default();
        n.added(dev("event3", "bt keyboard", true), false);
        n.removed("event3");
        assert!(!n.added(dev("event4", "bt keyboard", true), true));
    }

    #[test]
    fn the_only_keyboard_is_never_held() {
        let mut n = Newcomers::default();

        n.added(dev("event1", "Power Button", false), false);
        assert!(!n.added(dev("event5", "new keyboard", true), true));

        assert!(n.added(dev("event6", "new mouse", false), true));
    }

    #[test]
    fn the_list_kept_is_whole_as_it_was_when_it_cannot_be_written() {
        let dir = std::env::temp_dir().join(format!("trindle-newcomers-whole-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("known-input");
        std::fs::write(&file, "bt keyboard\nlaptop keys\n").unwrap();
        let mut n = Newcomers { known: ["bt keyboard", "laptop keys"].map(String::from).into(), file: Some(file.clone()), ..Default::default() };
        crate::files::FULL.set(true);
        n.added(dev("event7", "new mouse", false), false);
        crate::files::FULL.set(false);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "bt keyboard\nlaptop keys\n");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1, "nothing is left beside it");

        n.added(dev("event8", "another mouse", false), false);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "another mouse\nbt keyboard\nlaptop keys\nnew mouse\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
