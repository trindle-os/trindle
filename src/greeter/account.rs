//! What the login screen keeps about the people who use it.

use std::ffi::{c_char, CStr};
use std::path::PathBuf;

use crate::common::files::cache_home;

pub(super) fn session_cmd() -> String {
    std::env::var("TRINDLE_SESSION_CMD").unwrap_or_else(|_| "/usr/bin/trindle-session".into())
}

fn last_user_file() -> PathBuf {
    cache_home().join("trindle-greeter-last-user")
}

pub(super) fn drawing_file() -> PathBuf {
    cache_home().join("trindle-greeter-drawing")
}

pub(super) fn reader_file() -> PathBuf {
    cache_home().join("trindle-greeter-reader")
}

fn accounts() -> Vec<(String, u32, String)> {
    fn text(p: *const c_char) -> String {
        if p.is_null() {
            return String::new();
        }
        // SAFETY: not null, and libc owns a NUL-terminated string at it until
        // the next call into the passwd database, which is after this returns.
        unsafe { CStr::from_ptr(p).to_string_lossy().to_string() }
    }
    let mut out = Vec::new();
    // SAFETY: the walk between setpwent and endpwent is the documented way to
    // read the database. Each entry libc hands back is read before the next
    // call, which is what invalidates it, and the loop ends at the first null.
    unsafe {
        libc::setpwent();
        loop {
            let p = libc::getpwent();
            if p.is_null() {
                break;
            }
            let name = text((*p).pw_name);
            if name.is_empty() {
                continue;
            }
            out.push((name, (*p).pw_uid, text((*p).pw_shell)));
        }
        libc::endpwent();
    }
    out
}

pub(super) fn remembered_user() -> String {
    let people = accounts();
    if let Ok(name) = std::fs::read_to_string(last_user_file()) {
        let name = name.trim();
        if people.iter().any(|(n, _, _)| n == name) {
            return name.to_string();
        }
    }
    let ordinary: Vec<&String> = people.iter()
        .filter(|(_, uid, shell)| crate::common::ORDINARY_UIDS.contains(uid) && !shell.ends_with("nologin") && !shell.ends_with("false"))
        .map(|(n, _, _)| n).collect();
    if ordinary.len() == 1 { ordinary[0].clone() } else { String::new() }
}

pub(super) fn remember(name: &str) {
    let f = last_user_file();
    if let Some(d) = f.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let _ = crate::common::files::write_text(&f, name);
}
