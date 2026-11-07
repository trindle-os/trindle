//! Putting a plugin in its folder, swapping in an update and taking it away.

use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;

use crate::common::files::write_text;
use crate::common::firewall::Door;
use crate::store::boxed;
use crate::toolkit::t;

use super::{catalogue, log, shortcuts};
use super::background::{back_on, begin_background, stop_background};
use super::manifest::{check, folders, good_id, Plugin};
use super::permissions::{allowed, entitled, PERMISSIONS, set_allow};

#[derive(Clone, Debug, PartialEq)]
pub enum Background {
    None,
    Running,
    NotRunning(String),
}

#[derive(Clone, Debug)]
pub struct Installed {
    pub plugin: Plugin,
    pub background: Background,
}

pub(super) fn put_in(root: &Path, from: &Path) -> Result<Installed, String> {
    let p = check(from).map_err(|problems| problems.join("\n"))?;
    let at = root.join(&p.id);
    let fresh = at.join("app.new");
    std::fs::create_dir_all(at.join("home")).map_err(|e| e.to_string())?;
    let _alone = install_lock(&at).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&fresh);
    copy_aside(from, &fresh)?;
    let old = check(&at.join("app")).ok();
    let changed = old.is_some_and(|old| !same_ports(&old.listen, &p.listen));
    if changed {
        revoke_listen(&at);
    }
    if let Err(e) = drop_unasked(&at, &p.asks) {
        let _ = std::fs::remove_dir_all(&fresh);
        return Err(e.to_string());
    }
    let live = root == folders() && at.join("app").exists();
    if live {
        stop_background(&p.id);
    }
    let not_swapped = |why: String| -> String {
        match if live { back_on(&p.id) } else { None } {
            Some(not) => t!("plugins-swap-not-running", why = &why, not = &not),
            None => why,
        }
    };
    if let Err(e) = swap_in(&at, &fresh) {
        return Err(not_swapped(e));
    }
    let background = if live && p.background.is_some() && allowed(&p).iter().any(|a| a == "background") {
        match begin_background(&p.id) {
            Ok(()) => Background::Running,
            Err(why) => Background::NotRunning(why),
        }
    } else {
        Background::None
    };
    settle(root, &p, changed);
    Ok(Installed { plugin: p, background })
}

fn install_lock(at: &Path) -> std::io::Result<std::fs::File> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_DIRECTORY | libc::O_NONBLOCK).open(at)?;
    // SAFETY: flock only takes a lock on a descriptor this function owns.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(file)
}

fn copy_aside(from: &Path, fresh: &Path) -> Result<(), String> {
    let copied = crate::common::gentle::program(&mut Command::new("cp")).args(["-a", "--no-preserve=ownership", "--"])
        .arg(from).arg(fresh).status().is_ok_and(|s| s.success());
    if !copied {
        let _ = std::fs::remove_dir_all(fresh);
        return Err(t!("plugins-copy-failed").into());
    }
    Ok(())
}

fn settle(root: &Path, p: &Plugin, changed: bool) {
    if root != folders() {
        return;
    }
    if changed && entitled(&p.id) {
        crate::common::run::spawn(&["trindle-plugin", "close", &p.id]);
    }
    let _ = shortcuts::refresh();
}

fn swap_in(at: &Path, fresh: &Path) -> Result<(), String> {
    let (app, old_app) = (at.join("app"), at.join("app.old"));
    let _ = std::fs::remove_dir_all(&old_app);
    let aside = std::fs::rename(&app, &old_app).is_ok();
    if let Err(e) = std::fs::rename(fresh, &app) {
        if aside {
            let _ = std::fs::rename(&old_app, &app);
        }
        let _ = std::fs::remove_dir_all(fresh);
        return Err(e.to_string());
    }
    let _ = std::fs::remove_dir_all(&old_app);
    Ok(())
}

pub(super) fn same_ports(a: &[Door], b: &[Door]) -> bool {
    let sorted = |v: &[Door]| {
        let mut v = v.to_vec();
        v.sort();
        v
    };
    sorted(a) == sorted(b)
}

fn revoke_listen(at: &Path) {
    let _ = set_allow(at, &[], "listen", false);
    let Ok(asked) = crate::common::files::read_small(&at.join("asked")) else { return };
    let kept: String = asked.lines().map(str::trim).filter(|l| !l.is_empty() && *l != "listen").fold(String::new(), |mut s, l| { let _ = writeln!(s, "{l}"); s });
    let _ = write_text(&at.join("asked"), &kept);
}

fn drop_unasked(at: &Path, asks: &[String]) -> std::io::Result<()> {
    let gone: Vec<String> = boxed::allowed_at(at, PERMISSIONS).into_iter().filter(|a| !asks.contains(a)).collect();
    for what in &gone {
        set_allow(at, &[], what, false)?;
    }
    let Ok(asked) = crate::common::files::read_small(&at.join("asked")) else { return Ok(()) };
    let kept: String = asked.lines().map(str::trim).filter(|l| !l.is_empty() && asks.iter().any(|a| a == l)).fold(String::new(), |mut s, l| { let _ = writeln!(s, "{l}"); s });
    if kept != asked {
        write_text(&at.join("asked"), &kept)?;
    }
    Ok(())
}

pub fn remove(id: &str) -> std::io::Result<()> {
    if good_id(id) {
        stop_background(id);
        if entitled(id) {
            crate::common::run::spawn(&["trindle-plugin", "close", id]);
        }
    }
    let removed = remove_at(&folders(), id);
    catalogue::forget_icon(id);
    if good_id(id) {
        log::forget(id);
    }
    let _ = shortcuts::refresh();
    removed
}

pub fn remove_at(root: &Path, id: &str) -> std::io::Result<()> {
    if !good_id(id) {
        return Err(std::io::Error::other(t!("plugins-not-an-id")));
    }
    std::fs::remove_dir_all(root.join(id))
}

#[cfg(test)]
mod tests;
