//! Screen-lock PIN, accepted only for a limited time after locking.

use std::ffi::{c_char, c_int, c_ulong, CStr, CString};
use std::path::PathBuf;

use super::files::{config_path, setting};
use crate::toolkit::t;

pub const LENGTH: usize = 4;

pub const WINDOWS: &[u32] = &[15, 30, 45, 60, 75, 90, 105, 120];
pub const TRIES: u32 = 5;

#[link(name = "libcrypt.so.1", kind = "dylib", modifiers = "+verbatim")]
extern "C" {
    fn crypt(key: *const c_char, setting: *const c_char) -> *mut c_char;
    fn crypt_gensalt(prefix: *const c_char, count: c_ulong, rbytes: *const c_char, nrbytes: c_int) -> *mut c_char;
}

fn file() -> PathBuf {
    config_path("pin")
}

fn hash_with(pin: &str, setting: &str) -> Option<String> {
    let (k, s) = (CString::new(pin).ok()?, CString::new(setting).ok()?);
    // SAFETY: both arguments are NUL-terminated and outlive the call. crypt

    let out = unsafe { crypt(k.as_ptr(), s.as_ptr()) };
    if out.is_null() {
        return None;
    }
    // SAFETY: not null, and crypt's answer is NUL-terminated.
    let h = unsafe { CStr::from_ptr(out) }.to_string_lossy().to_string();

    (!h.starts_with('*')).then_some(h)
}

pub fn valid(pin: &str) -> bool {
    pin.len() == LENGTH && pin.chars().all(|c| c.is_ascii_digit())
}

pub fn is_set() -> bool {
    std::fs::read_to_string(file()).is_ok_and(|t| t.trim().starts_with('$'))
}

pub fn set(pin: &str) -> Result<(), String> {
    if !valid(pin) {
        return Err(format!("A PIN is {LENGTH} digits."));
    }
    let prefix = CString::new("$y$").map_err(|e| e.to_string())?;
    // SAFETY: a NUL-terminated prefix that outlives the call, and a null

    let salt = unsafe { crypt_gensalt(prefix.as_ptr(), 0, std::ptr::null(), 0) };
    if salt.is_null() {
        return Err("The PIN could not be hashed.".into());
    }
    // SAFETY: not null, and NUL-terminated; copied before any further call.
    let salt = unsafe { CStr::from_ptr(salt) }.to_string_lossy().to_string();
    let hash = hash_with(pin, &salt).ok_or("The PIN could not be hashed.")?;
    let path = file();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }

    super::files::write_whole_private(&path, format!("{hash}\n").as_bytes()).map_err(|e| e.to_string())
}

pub fn remove() {
    let _ = std::fs::remove_file(file());
}

pub fn check(pin: &str) -> bool {
    let Ok(stored) = std::fs::read_to_string(file()) else { return false };
    let stored = stored.trim();
    valid(pin) && hash_with(pin, stored).is_some_and(|h| h == stored)
}

pub fn minutes() -> u32 {
    let m: u32 = setting("pin_minutes").parse().unwrap_or(60);
    if WINDOWS.contains(&m) { m } else { 60 }
}

pub fn window_label(m: u32) -> String {
    let hours = |h| t!("settings-power-pin-hours", n = h);
    let minutes = |m| t!("settings-power-pin-minutes", n = m);
    match (m / 60, m % 60) {
        (0, m) => minutes(m),
        (h, 0) => hours(h),
        (h, m) => t!("settings-power-pin-hours-minutes", hours = hours(h), minutes = minutes(m)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pin_is_four_digits() {
        assert!(valid("0123"));
        assert!(!valid("123"));
        assert!(!valid("12a4"));
        assert!(!valid("12345"));
    }

    #[test]
    fn hashes_check_and_differ() {
        // SAFETY: a static NUL-terminated prefix, and libcrypt's own

        let salt = unsafe { CStr::from_ptr(crypt_gensalt(c"$y$".as_ptr(), 0, std::ptr::null(), 0)) }
            .to_string_lossy().to_string();
        let h = hash_with("4821", &salt).unwrap();
        assert!(h.starts_with("$y$"));
        assert_eq!(hash_with("4821", &h).unwrap(), h);
        assert_ne!(hash_with("4822", &h).unwrap(), h);
    }

    #[test]
    fn labels() {
        assert_eq!(window_label(45), "45 minutes");
        assert_eq!(window_label(60), "1 hour");
        assert_eq!(window_label(75), "1 hour 15 minutes");
        assert_eq!(window_label(120), "2 hours");
    }
}
