//! The words the login screen says.

use std::time::Duration;

use crate::toolkit::t;

pub(super) const HINT: Duration = Duration::from_secs(3);

pub(super) fn pam_words(description: &str) -> Option<&'static str> {
    if description.contains("MAXTRIES") {
        Some(t!("greeter-pam-max-tries"))
    } else if description.contains("ACCT_EXPIRED") {
        Some(t!("greeter-pam-account-expired"))
    } else if description.contains("NEW_AUTHTOK_REQD") {
        Some(t!("greeter-pam-password-expired"))
    } else if description.contains("AUTH_ERR") || description.contains("USER_UNKNOWN") {
        Some(t!("greeter-no-match"))
    } else {
        None
    }
}

pub(super) fn no_match() -> &'static str {
    t!("greeter-no-match")
}

pub(super) fn welcome_back() -> &'static str {
    t!("greeter-welcome-back")
}

pub(super) fn welcome_new() -> &'static str {
    t!("greeter-welcome")
}

pub(super) fn ready_plain() -> &'static str {
    t!("greeter-ready")
}

pub(super) fn ready_phone() -> &'static str {
    t!("greeter-ready-phone")
}

pub(super) fn ready_name() -> &'static str {
    t!("greeter-ready-name")
}

pub(super) fn phone_no() -> &'static str {
    t!("greeter-phone-failed")
}

pub(super) fn phone_hint() -> &'static str {
    t!("greeter-phone-hint")
}

pub(super) fn trouble() -> &'static str {
    t!("greeter-trouble")
}

pub(super) fn no_chip() -> &'static str {
    t!("greeter-no-chip")
}
