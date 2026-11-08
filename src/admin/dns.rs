//! The helper's steps for private DNS: the name servers Tailscale and blocking set, and the sites never blocked.

use std::path::Path;

use super::{Outcome, and_undone, at, gone, kept, replace, run};
use super::tailscale::mark_tailscale;
use crate::toolkit::t;

pub(super) fn dns_on() -> Outcome {
    use crate::common::{dns, tailscale};
    if dns::on() {
        return Ok(());
    }
    std::fs::create_dir_all("/etc/trindle").map_err(|e| e.to_string())?;

    let check = !dns::nm_check_chosen(Path::new("/"));
    if dns::behind_resolved() {
        replace(Path::new(dns::RESOLVED_MARK), b"", 0o644)?;
        if Path::new("/etc/NetworkManager").is_dir() {
            if let Some(text) = dns::nm_drop_in(false, check) {
                std::fs::create_dir_all("/etc/NetworkManager/conf.d").map_err(|e| e.to_string())?;
                replace(Path::new(dns::NM_DROP_IN), text.as_bytes(), 0o644)?;

                // reads the file when it starts.
                let _ = run(&["nmcli", "general", "reload", "conf"], None);
            }
        }
        if let Err(e) = start_dns() {
            return Err(and_undone(e, dns_off()));
        }
        return Ok(());
    }

    if tailscale::installed() && run(&["systemctl", "is-active", "--quiet", tailscale::SERVICE], None).is_ok() {
        run(&["tailscale", "set", "--accept-dns=false"], None)?;
        mark_tailscale()?;
    }
    std::fs::create_dir_all("/etc/NetworkManager/conf.d").map_err(|e| e.to_string())?;
    let text = dns::nm_drop_in(true, check).unwrap_or_default();
    replace(Path::new(dns::NM_DROP_IN), text.as_bytes(), 0o644)?;
    // As above.
    let _ = run(&["nmcli", "general", "reload", "conf"], None);
    if let Err(e) = start_dns() {
        return Err(and_undone(e, dns_off()));
    }
    let before = Path::new(dns::RESOLV_BEFORE);
    let etc = Path::new(dns::ETC_RESOLV);

    // Were it still there, the link made next fails.
    let _ = std::fs::remove_file(before);
    if etc.symlink_metadata().is_ok() {
        std::fs::hard_link(etc, before).map_err(|e| e.to_string())?;
    }
    let tmp = etc.with_extension("trindle-new");
    // Were it still there, the link made next fails.
    let _ = std::fs::remove_file(&tmp);
    std::os::unix::fs::symlink(dns::RESOLV, &tmp)
        .and_then(|()| std::fs::rename(&tmp, etc))
        .map_err(|e| e.to_string())
}

fn start_dns() -> Outcome {
    use crate::common::dns;
    run(&["systemctl", "enable", "--now", dns::SERVICE], None)?;
    for _ in 0..50 {
        if Path::new(dns::RESOLV).exists() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Err(t!("admin-blocking-late").into())
}

pub(super) fn dns_off() -> Outcome {
    use crate::common::{dns, tailscale};
    let before = &at(dns::RESOLV_BEFORE);
    let etc = &at(dns::ETC_RESOLV);

    let mut left: Vec<String> = Vec::new();
    let mut tried = |done: Outcome| left.extend(done.err());
    let put_back = |e: String| t!("admin-file-not-put-back", path = etc.display().to_string(), why = e);
    if before.symlink_metadata().is_ok() {
        tried(std::fs::rename(before, etc).map_err(|e| put_back(e.to_string())));
    } else if dns::holds_resolv() {

        let text = dns::NETWORK_FILES.iter().filter_map(|f| std::fs::read_to_string(f).ok())
            .find(|t| t.contains("nameserver")).unwrap_or_default();
        tried(replace(etc, text.as_bytes(), 0o644).map_err(put_back));
    }
    tried(gone(&at(dns::RESOLVED_MARK)));
    tried(gone(&at(dns::NM_DROP_IN)));

    // files when it starts.
    let _ = run(&["nmcli", "general", "reload"], None);

    tried(run(&["systemctl", "disable", "--now", dns::SERVICE], None));
    let passed_on = &at(dns::RESOLVED_DROP_IN);
    if passed_on.symlink_metadata().is_ok() {
        tried(gone(passed_on).and_then(|()| run(&["systemctl", "try-reload-or-restart", "systemd-resolved.service"], None)));
    }
    if at(dns::TAILSCALE_MARK).exists() {

        if tailscale::installed() && run(&["systemctl", "is-active", "--quiet", tailscale::SERVICE], None).is_ok() {
            tried(run(&["tailscale", "set", "--accept-dns=true"], None));
        }
        tried(gone(&at(dns::TAILSCALE_MARK)));
    }
    if left.is_empty() {
        Ok(())
    } else {
        Err(t!("admin-blocking-not-off", why = left.join("; ")))
    }
}

pub(super) fn dns_allow(args: &[String]) -> Outcome {
    use crate::common::dns;
    let [how, typed] = args else { return Err("usage: trindle-admin dns-allow add|remove SITE".into()) };
    let site = dns::site(typed).ok_or(t!("admin-site-invalid"))?;

    let mut list = dns::allowed_in(&kept(&at(dns::ALLOW))?);
    match how.as_str() {
        "add" if !list.contains(&site) => list.push(site),
        "add" => {}
        "remove" => list.retain(|s| *s != site),
        _ => return Err("usage: trindle-admin dns-allow add|remove SITE".into()),
    }
    list.sort();
    let text: String = list.iter().fold(String::new(), |mut out, s| { out.push_str(s); out.push('\n'); out });
    std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
    replace(&at(dns::ALLOW), text.as_bytes(), 0o644)?;
    run(&["systemctl", "try-reload-or-restart", dns::SERVICE], None)?;

    if dns::resolved_running() {

        // resolved holds runs out, as it would with no resolved.
        let _ = run(&["resolvectl", "flush-caches"], None);
    }
    Ok(())
}

pub(super) fn dns_provider(args: &[String]) -> Outcome {
    use crate::common::dns;
    let [key] = args else { return Err("usage: trindle-admin dns-provider quad9|mullvad|cloudflare|network".into()) };
    let p = dns::Provider::from_key(key).ok_or(t!("admin-server-unknown"))?;
    std::fs::create_dir_all("/etc/trindle").map_err(|e| e.to_string())?;
    replace(Path::new(dns::PROVIDER), format!("{}\n", p.key()).as_bytes(), 0o644)?;
    run(&["systemctl", "try-reload-or-restart", dns::SERVICE], None)
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;
    use crate::common::dns;

    #[test]
    fn blocking_that_is_not_fully_off_says_each_part_left() {
        let s = StandIn::failing("dns-off", "systemctl disable", "Failed to disable unit.");
        s.stuck(dns::NM_DROP_IN);
        let said = super::dns_off().unwrap_err();
        assert!(said.starts_with("Blocking is not fully off: "), "{said}");
        assert!(said.contains("could not be removed") && said.contains("Failed to disable unit."), "{said}");

        assert!(s.ran().iter().any(|c| c.starts_with("systemctl disable --now")), "{:?}", s.ran());
    }

    #[test]
    fn blocking_whose_first_part_cannot_be_undone_still_undoes_the_rest() {
        let s = StandIn::all_well("dns-off-first");
        s.write(dns::RESOLV_BEFORE, b"nameserver 192.0.2.1\n");
        s.write(dns::RESOLVED_MARK, b"");
        s.stuck(dns::ETC_RESOLV);
        let said = super::dns_off().unwrap_err();
        assert!(said.starts_with("Blocking is not fully off: "), "{said}");
        assert!(s.ran().iter().any(|c| c.starts_with("systemctl disable --now")), "{:?}", s.ran());
        assert!(!s.path(dns::RESOLVED_MARK).exists(), "the parts after the first were undone");
    }

    #[test]
    fn a_list_of_sites_never_blocked_that_cannot_be_read_is_not_written_over() {
        let s = StandIn::all_well("dns-allow");
        let was: &[u8] = b"example.com\n\xff\xfe\n";
        s.write(dns::ALLOW, was);
        let said = super::dns_allow(&["add".to_string(), "example.org".to_string()]).unwrap_err();
        assert!(said.starts_with("Nothing was changed, because"), "{said}");
        assert_eq!(std::fs::read(s.path(dns::ALLOW)).unwrap(), was);
        assert!(s.ran().is_empty());
    }

    #[test]
    fn a_failure_is_said_with_how_the_putting_back_went() {
        assert_eq!(super::and_undone("It failed.".into(), Ok(())), "It failed.");
        assert_eq!(super::and_undone("It failed.".into(), Err("no".into())),
                   "It failed. Putting things back as they were did not work either: no");
    }
}
