//! Why an action failed, said in words a person can use, and the stock lines for what is missing.

use super::Kit;
use crate::toolkit::t;

pub fn not_saved() -> &'static str {
    t!("settings-common-not-saved")
}

pub fn no_system_bus() -> &'static str {
    t!("settings-common-no-system-bus")
}

pub fn no_session_bus() -> &'static str {
    t!("settings-common-no-session-bus")
}

pub fn no_chooser() -> &'static str {
    t!("settings-common-no-chooser")
}

pub trait Cause {
    fn words(&self, could_not: &str) -> String;
}

pub struct Raw(pub String);

impl Cause for Raw {
    fn words(&self, could_not: &str) -> String {
        why(&self.0, could_not)
    }
}

impl Cause for crate::toolkit::bus::Failed {
    fn words(&self, could_not: &str) -> String {
        why(&format!("{} {}", self.name, self.text), could_not)
    }
}

impl Cause for std::io::Error {
    fn words(&self, could_not: &str) -> String {
        why(&self.to_string(), could_not)
    }
}

impl Cause for String {
    fn words(&self, could_not: &str) -> String {
        let said = self.trim();
        if said.is_empty() { could_not.to_string() } else { said.to_string() }
    }
}

impl Cause for &str {
    fn words(&self, could_not: &str) -> String {
        self.to_string().words(could_not)
    }
}

impl Cause for () {
    fn words(&self, could_not: &str) -> String {
        could_not.to_string()
    }
}

pub fn ran(ok: bool, complaint: &str) -> Result<(), Raw> {
    if ok { Ok(()) } else { Err(Raw(complaint.to_string())) }
}

pub fn no_more<P, T>(_: &mut P, _: &mut Kit, _: Option<T>) {}

pub fn kept(took: bool) -> Result<(), ()> {
    if took { Ok(()) } else { Err(()) }
}

pub fn asked(ok: bool, complaint: &str) -> Result<(), String> {
    if ok { Ok(()) } else { Err(crate::common::run::admin_error(complaint)) }
}

pub fn why(raw: &str, could_not: &str) -> String {
    let low = raw.to_lowercase();
    let has = |any: &[&str]| any.iter().any(|w| low.contains(w));
    let cause = if has(&["no space left", "quota exceeded", "os error 28", "os error 122"]) {
        t!("settings-common-why-full")
    } else if has(&["read-only file system", "os error 30"]) {
        t!("settings-common-why-read-only")
    } else if has(&["serviceunknown", "namehasnoowner", "name has no owner", "not provided by any .service", "servicenotfound",
                    "unknownobject", "unknownmethod", "unknowninterface"]) {
        t!("settings-common-why-not-running")
    } else if has(&["noreply", "timed out", "timeout", "did not receive a reply"]) {
        t!("settings-common-why-slow")
    } else if has(&["interactiveauthorizationrequired", "notauthorized", "not authorized", "authfailed", "authentication"]) {
        t!("settings-common-why-password")
    } else if has(&["accessdenied", "access denied", "permission denied", "not permitted", "os error 13"]) {
        t!("settings-common-why-not-allowed")
    } else if has(&["no such file", "os error 2)"]) {
        t!("settings-common-why-missing")
    } else {
        return could_not.to_string();
    };
    t!("settings-common-why-then", could_not = could_not, cause = cause)
}

pub(super) fn is_just_saved(text: &str) -> bool {
    let low = text.trim().trim_end_matches('.').to_lowercase();
    if [t!("settings-common-saved"), t!("settings-common-done")].iter().any(|w| w.to_lowercase() == low) {
        return true;
    }
    matches!(low.as_str(),
             "saved" | "done" | "in use now" | "changed" | "set" | "taken" | "added" | "freed" | "removed"
             | "stored" | "installed" | "updated" | "joined" | "copied" | "default now" | "played" | "sent"
             | "connected" | "connecting" | "on" | "off" | "forgotten" | "stops at 80%" | "charges to full"
             | "starts at login" | "kept, but does not start" | "up to date")
}
