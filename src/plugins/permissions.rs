//! What a plugin is allowed: the permissions it can ask for, the person's answers and the firewall they open.

use std::fmt::Write as _;
use std::path::Path;

use crate::common::firewall;
use crate::common::files::{read_text, write_text};
use crate::common::firewall::Door;
use crate::store::boxed;
use crate::toolkit::t;

use super::install::same_ports;
use super::manifest::{check, folder, folders, good_id, Plugin};

pub const PERMISSIONS: &[&str] = &[
    "network", "sound", "microphone", "files", "notifications", "background", "control", "capture", "listen", "copied",
    crate::common::summon::WORD, HOME_NETWORK, USB_DEVICES, crate::common::expand::WORD, ACCESSIBILITY,
];

pub const HOME_NETWORK: &str = "home-network";
pub const USB_DEVICES: &str = "usb-devices";
pub const ACCESSIBILITY: &str = "accessibility";

const PRESET: &str = "preset";

pub fn preset_reader(folders: &Path, id: &str) {
    let at = folders.join(id);
    if !crate::common::own_hands::is_screen_reader_in(folders, id) || at.join(PRESET).exists() {
        return;
    }

    if write_text(&at.join(PRESET), "").is_err() {
        return;
    }
    if !["sound", ACCESSIBILITY].iter().all(|w| set_allow(&at, &[], w, true).is_ok()) {
        let _ = std::fs::remove_file(at.join(PRESET));
    }
}

pub fn installed() -> Vec<Plugin> {
    let mut all: Vec<Plugin> = std::fs::read_dir(folders()).into_iter().flatten().flatten()
        .filter_map(|e| check(&e.path().join("app")).ok())
        .filter(|p| folder(&p.id).join("app").is_dir())
        .collect();
    all.sort_by_key(|p| p.name.to_lowercase());
    all
}

pub fn find(id: &str) -> Option<Plugin> {
    good_id(id).then(|| check(&folder(id).join("app")).ok()).flatten()
}

pub fn allowed(p: &Plugin) -> Vec<String> {
    boxed::allowed_at(&folder(&p.id), PERMISSIONS).into_iter().filter(|a| p.asks.contains(a)).collect()
}

pub fn standing(p: &Plugin) -> (usize, usize) {
    let allowed = allowed(p);
    let asks: Vec<&String> = p.asks.iter().filter(|a| PERMISSIONS.contains(&a.as_str())).collect();
    (asks.len(), asks.iter().filter(|a| allowed.contains(a)).count())
}

pub fn needs_words(name: &str, asks: usize, allowed: usize) -> Option<String> {
    match (asks, allowed) {
        (0, _) => None,
        (n, a) if a >= n => None,
        (n, 0) => Some(t!("plugins-needs-none", name = name, n = n)),
        (n, a) => Some(t!("plugins-needs-some", name = name, a = a, n = n)),
    }
}

pub fn settled_words(asks: usize) -> String {
    t!("plugins-settled", n = asks)
}

pub fn allow_word() -> &'static str {
    t!("plugins-allow")
}
pub const ALLOW_ICON: &str = "changes-allow-symbolic";

pub fn ask_again(id: &str) -> bool {
    crate::common::run::spawn(&["trindle-plugin", "ask", id, "again"])
}

pub fn allow(id: &str, what: &str, on: bool) -> std::io::Result<()> {
    let ports = find(id).map(|p| p.listen).unwrap_or_default();
    set_allow(&folder(id), &ports, what, on)
}

pub(super) const GRANTED: &str = "listening";

pub(super) fn set_allow(at: &Path, ports: &[Door], what: &str, on: bool) -> std::io::Result<()> {
    boxed::allow_at(at, PERMISSIONS, what, on)?;
    if what == "listen" {
        if on {
            write_text(&at.join(GRANTED), &granted_text(ports))?;
        } else {
            let _ = std::fs::remove_file(at.join(GRANTED));
        }
    }
    Ok(())
}

pub(super) fn granted_text(ports: &[Door]) -> String {
    let mut all = ports.to_vec();
    all.sort();
    all.iter().fold(String::new(), |mut s, d| { let _ = writeln!(s, "{d}"); s })
}

pub fn open_ports(p: &Plugin, allowed: &[String]) -> Vec<Door> {
    let has = |k: &str| allowed.iter().any(|a| a == k);
    if has("listen") && has("network") { p.listen.clone() } else { Vec::new() }
}

pub(super) fn my_uid() -> u32 {
    // SAFETY: getuid only reads this process's own user number and cannot fail.
    unsafe { libc::getuid() }
}

pub(super) fn entitled(id: &str) -> bool {
    let me = my_uid();
    firewall::entitled().iter().any(|e| e.uid == me && e.id == id)
}

pub fn listen_refusal(p: &Plugin) -> Option<String> {
    if p.listen.is_empty() {
        return None;
    }
    let me = my_uid();
    let all = firewall::with(&firewall::without(&firewall::entitled(), me, &p.id), me, &p.id);
    let read = |e: &firewall::Entitled| -> Result<Vec<Door>, String> {
        if e.uid != me {
            return Ok(Vec::new());
        }
        if e.id == p.id { Ok(p.listen.clone()) } else { find(&e.id).map(|o| o.listen).ok_or_else(|| t!("plugins-not-installed").to_string()) }
    };
    let name = |id: &str| find(id).map_or_else(|| t!("plugins-another-plugin").to_string(), |o| o.name);
    firewall::derive(&all, &read).into_iter().find(|(e, _)| e.uid == me && e.id == p.id)
        .and_then(|(_, r)| r.err()).map(|r| r.words(&name))
}

pub fn firewall_apply(id: &str) -> Result<(), String> {
    let p = find(id).ok_or(t!("plugins-that-not-installed"))?;
    let wanted = !open_ports(&p, &allowed(&p)).is_empty();
    match (wanted, entitled(id)) {
        (true, false) => admin(&["firewall-plugin", id]),
        (false, true) => admin(&["firewall-plugin-close", id]),
        _ => Ok(()),
    }
}

pub(super) fn firewall_said(opening: bool, why: &str, put_off: Result<(), String>) -> String {
    let why = why.trim().trim_end_matches('.');
    match (opening, put_off) {
        (false, _) => format!("its ports could not be closed: {why}."),
        (true, Ok(())) => format!("its ports were not opened, so listening is off again: {why}."),
        (true, Err(kept)) => format!("its ports were not opened ({why}), and listening could not be switched off again either ({}). \
                                      Nothing is open; switch it off in Settings, Permissions.", kept.trim().trim_end_matches('.')),
    }
}

pub(super) fn admin(args: &[&str]) -> Result<(), String> {
    let mut argv: Vec<String> = vec!["pkexec".into(), crate::common::run::admin_helper()];
    argv.extend(args.iter().map(|a| a.to_string()));
    let (ok, _, err) = crate::common::run::captured(&argv, None);
    if ok { Ok(()) } else { Err(crate::common::run::admin_error(&err)) }
}

pub fn set_allowed(id: &str, what: &str, on: bool) -> Result<(), String> {
    let was = find(id).is_some_and(|p| allowed(&p).iter().any(|a| a == what));
    allow(id, what, on).map_err(|e| t!("plugins-not-saved", why = e.to_string()))?;

    let Some(p) = find(id).filter(|p| matches!(what, "listen" | "network") && !p.listen.is_empty()) else { return Ok(()) };
    let back = |e: String| {
        let _ = allow(id, what, was);
        e
    };
    if !open_ports(&p, &allowed(&p)).is_empty() {
        if let Some(why) = listen_refusal(&p) {
            return Err(back(why));
        }
    }
    firewall_apply(id).map_err(back)
}

pub fn asked(id: &str) -> Vec<String> {
    read_text(folder(id).join("asked")).lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

pub fn mark_asked(p: &Plugin) -> std::io::Result<()> {
    mark_asked_in(&folder(&p.id).join("asked"), &p.asks)
}

pub(crate) fn mark_asked_in(file: &Path, asks: &[String]) -> std::io::Result<()> {
    let mut all: Vec<String> = crate::common::files::read_small(file)?
        .lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect();
    for a in asks {
        if !all.contains(a) {
            all.push(a.clone());
        }
    }
    write_text(file, &all.iter().fold(String::new(), |mut s, a| { let _ = writeln!(s, "{a}"); s }))
}

pub fn asks_more(old: &Plugin, asks: &[String], listen: Option<&[Door]>) -> Vec<String> {
    let mut more: Vec<String> = asks.iter().filter(|a| !old.asks.contains(a))
        .map(|a| crate::common::access::ask(a).name.to_string()).collect();
    if listen.is_some_and(|l| !same_ports(&old.listen, l)) {
        more.push(other_ports().into());
    }
    more
}

pub(super) fn other_ports() -> &'static str {
    t!("plugins-other-ports")
}

#[cfg(test)]
mod tests;
