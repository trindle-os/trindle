//! The network ports a plugin listens on, read from its manifest and judged again where they are opened.

use std::path::Path;

use crate::common::firewall;
use crate::common::firewall::Door;
use crate::common::keyfile::KeyFile;
use crate::toolkit::t;

use super::manifest::{INTERFACE, list, MANIFEST};
use super::permissions::{GRANTED, granted_text};

pub(super) fn listen_ports(kf: &KeyFile, asks: &[String], problems: &mut Vec<String>) -> Vec<Door> {
    let words = list(kf, "Network", "Listen");
    let has = |k: &str| asks.iter().any(|a| a == k);
    let mut doors = Vec::new();
    for w in &words {
        match Door::parse(w) {
            Ok(d) => doors.push(d),
            Err(e) => problems.push(t!("plugins-listen-fault", why = &e)),
        }
    }
    if doors.len() == words.len() {
        if let Err(e) = firewall::vet(&doors) {
            problems.push(t!("plugins-listen-fault", why = &e));
        }
    }
    if !words.is_empty() && !has("network") {
        problems.push(t!("plugins-listen-needs-network").into());
    }
    if !words.is_empty() && !has("listen") {
        problems.push(t!("plugins-listen-needs-listen").into());
    }
    if words.is_empty() && has("listen") {
        problems.push(t!("plugins-listen-needs-ports").into());
    }
    doors
}

pub fn declared_ports(ini: &str) -> Result<Vec<Door>, String> {
    let kf = KeyFile::parse(ini);
    if kf.integer("Plugin", "Interface") != Some(i64::from(INTERFACE)) {
        return Err(t!("plugins-ports-interface").into());
    }
    let asks = list(&kf, "Plugin", "Permissions");
    let mut problems = Vec::new();
    let doors = listen_ports(&kf, &asks, &mut problems);
    match problems.into_iter().next() {
        Some(p) => Err(p),
        None if doors.is_empty() => Err(t!("plugins-ports-none").into()),
        None => Ok(doors),
    }
}

fn read_theirs(path: &Path, uid: u32, most: u64) -> Option<String> {
    use std::io::Read;
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

    let f = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    let m = f.metadata().ok()?;
    if !m.is_file() || m.uid() != uid || m.len() > most {
        return None;
    }
    let mut text = String::new();
    f.take(most).read_to_string(&mut text).ok()?;
    Some(text)
}

pub fn ports_to_open(at: &Path, uid: u32) -> Result<Vec<Door>, String> {
    let doors = declared_ports(&read_theirs(&at.join("app").join(MANIFEST), uid, 65536).ok_or(t!("plugins-ports-unreadable"))?)?;
    let allowed = read_theirs(&at.join("allowed"), uid, 4096).unwrap_or_default();
    let has = |k: &str| allowed.lines().any(|l| l.trim() == k);
    if !has("listen") || !has("network") {
        return Err(t!("plugins-ports-not-allowed").into());
    }
    let shown = read_theirs(&at.join(GRANTED), uid, 4096).unwrap_or_default();
    if shown != granted_text(&doors) {
        return Err(t!("plugins-ports-changed").into());
    }
    Ok(doors)
}

#[cfg(test)]
mod tests;
