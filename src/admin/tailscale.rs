//! The helper's steps for Tailscale.

use std::path::Path;

use super::accounts::name_of;
use super::{sources, Outcome, replace, run};
use crate::toolkit::t;

pub(super) fn tailscale_on() -> Outcome {
    use crate::common::tailscale;
    let user = std::env::var("PKEXEC_UID").ok().and_then(|u| u.parse().ok()).and_then(name_of)
        .ok_or(t!("admin-tailscale-who"))?;
    if !tailscale::installed() {
        sources::add_unstarted("tailscale", &user)?;
    }
    run(&["systemctl", "enable", "--now", tailscale::SERVICE], None)?;
    let operator = format!("--operator={user}");
    let blocking = crate::common::dns::holds_resolv();
    let accept = format!("--accept-dns={}", !blocking);
    tailscale_set(&[operator.as_str(), accept.as_str()])?;
    if blocking {
        mark_tailscale()?;
    }
    Ok(())
}

pub(super) fn tailscale_set(what: &[&str]) -> Outcome {
    let mut argv = vec!["tailscale", "set"];
    argv.extend(what);
    let mut last = Ok(());
    for _ in 0..20 {
        last = run(&argv, None);
        if last.is_ok() {
            break;
        }
        #[cfg(not(test))]
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    last
}

pub(super) fn tailscale_off() -> Outcome {
    run(&["systemctl", "disable", "--now", crate::common::tailscale::SERVICE], None)
}

pub(super) fn mark_tailscale() -> Outcome {
    use crate::common::dns;
    std::fs::create_dir_all("/etc/trindle").map_err(|e| e.to_string())?;
    replace(Path::new(dns::TAILSCALE_MARK), b"", 0o644)
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;
    use crate::common::tailscale::SERVICE;

    #[test]
    fn switching_off_stops_the_service_and_its_start_and_nothing_else() {
        let s = StandIn::all_well("tailscale-off");
        super::tailscale_off().unwrap();
        assert_eq!(s.ran(), [format!("systemctl disable --now {SERVICE}")]);
    }

    #[test]
    fn a_service_still_opening_its_socket_is_asked_again_and_a_dead_one_is_given_up_on() {
        let mut fails = 3;
        let s = StandIn::new("tailscale-set", move |argv| {
            if argv[0] == "tailscale" && fails > 0 { fails -= 1; Err("not ready".into()) } else { Ok(()) }
        });
        super::tailscale_set(&["--operator=john"]).unwrap();
        assert_eq!(s.ran().len(), 4);
        drop(s);
        let s = StandIn::failing("tailscale-dead", "tailscale set", "no socket");
        assert_eq!(super::tailscale_set(&["--accept-dns=true"]), Err("no socket".into()));
        assert_eq!(s.ran().len(), 20);
    }

    #[test]
    fn switching_on_without_a_person_to_make_the_operator_changes_nothing() {
        if std::env::var("PKEXEC_UID").is_ok() {
            return;
        }
        let s = StandIn::all_well("tailscale-on-nobody");
        assert!(super::tailscale_on().is_err());
        assert!(s.ran().is_empty());
    }
}
