//! What went wrong with starting the ring, kept where the ring that does
//! start can say it. The ring is also what shows notifications, so while it
//! is away nothing can tell the person why; the compositor and the session's
//! start-up leave one word here, and the next ring to open says it in red
//! under itself.

use crate::toolkit::t;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Said {

    HeldBack,

    NotStarted,

    ByName,

    NotEnded,

    NotDone,
}

const ALL: [Said; 5] = [Said::HeldBack, Said::NotStarted, Said::ByName, Said::NotEnded, Said::NotDone];

impl Said {
    fn word(self) -> &'static str {
        match self {
            Said::HeldBack => "held-back",
            Said::NotStarted => "not-started",
            Said::ByName => "by-name",
            Said::NotEnded => "not-ended",
            Said::NotDone => "not-done",
        }
    }

    pub fn words(self) -> &'static str {
        match self {
            Said::HeldBack => t!("common-ring-held-back"),
            Said::NotStarted => t!("common-ring-not-started"),
            Said::NotEnded => t!("common-ring-not-ended"),
            Said::NotDone => t!("common-ring-not-done"),
            Said::ByName => t!("common-ring-by-name"),
        }
    }

    pub fn passes(self) -> bool {
        self != Said::ByName
    }
}

fn file() -> Option<PathBuf> {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").filter(|d| !d.is_empty())?;
    Some(PathBuf::from(dir).join("trindle-ring-said"))
}

pub fn leave(said: Said) {
    if read() == Some(Said::ByName) {
        return;
    }
    if let Some(file) = file() {
        let _ = std::fs::write(file, said.word());
    }
}

pub fn read() -> Option<Said> {
    let text = std::fs::read_to_string(file()?).ok()?;
    ALL.into_iter().find(|s| s.word() == text.trim())
}

pub fn shown(said: Said) {
    if said.passes() {
        if let Some(file) = file() {
            let _ = std::fs::remove_file(file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_known_words_are_ever_said() {
        for said in ALL {
            assert_eq!(ALL.into_iter().find(|s| s.word() == said.word()), Some(said));
            assert!(!said.words().is_empty());
        }
        assert!(ALL.into_iter().all(|s| s.word() != "anything else"));
        assert!(Said::HeldBack.passes() && Said::NotStarted.passes() && Said::NotDone.passes() && !Said::ByName.passes());
    }
}
