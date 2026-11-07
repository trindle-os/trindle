//! User pictures, readable by both Settings and the greeter.

use std::path::PathBuf;

pub const FACES: &str = "/var/lib/trindle/faces";

pub fn path_for(name: &str) -> Option<PathBuf> {
    if name.is_empty() || name.contains('/') {
        return None;
    }
    let p = PathBuf::from(FACES).join(format!("{name}.png"));
    p.is_file().then_some(p)
}
