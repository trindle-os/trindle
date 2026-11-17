//! What a PAM code from greetd means to the person, and how long the phone hint stays.

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
