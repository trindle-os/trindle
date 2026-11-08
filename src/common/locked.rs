//! Whether this login's screen is locked, as a small file the compositor
//! keeps in the login's runtime folder: there while locked, gone once
//! unlocked. The Thunderbolt guard reads it, as root, before letting a new
//! device in, and its question waits while it is there.

use std::path::{Path, PathBuf};

pub const NAME: &str = "trindle-locked";

pub fn path() -> PathBuf {
    crate::common::files::scratch().join(NAME)
}

pub fn mark(locked: bool) {
    let file = path();
    let done = if locked { std::fs::write(&file, b"") } else { std::fs::remove_file(&file) };
    if let Err(e) = done {
        if locked || e.kind() != std::io::ErrorKind::NotFound {
            eprintln!("trindle: {} not kept: {e}", file.display());
        }
    }
}

pub fn of(run_user: &Path, uid: u32) -> bool {
    run_user.join(uid.to_string()).join(NAME).exists()
}
