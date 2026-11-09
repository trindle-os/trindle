//! The helper's steps for the whole computer: language, name, update checks, the charge limit and the radios.

use super::{Outcome, and_undone, at, gone, kept, replace, run};
use crate::toolkit::t;

pub(super) fn locale(args: &[String]) -> Outcome {
    let [code] = args else { return Err("usage: trindle-admin locale ll_CC.UTF-8".into()) };
    if code.is_empty() || !code.chars().all(|c| c.is_ascii_alphanumeric() || "_@.-".contains(c)) {
        return Err(t!("admin-language-invalid").into());
    }
    let wanted = format!("{code} UTF-8");
    let supported = std::fs::read_to_string(at("/usr/share/i18n/SUPPORTED"))
        .map_err(|e| t!("admin-language-list", why = e.to_string()))?;
    if !supported.lines().any(|l| l == wanted) {
        return Err(t!("admin-language-unknown", code = code));
    }
    let gen = &at("/etc/locale.gen");
    let text = kept(gen)?;
    let mut found = false;
    let mut lines: Vec<String> = text.lines().map(|l| {
        let bare = l.trim_start_matches('#').trim_start();
        let matches = bare.strip_prefix(&wanted).is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace));
        if matches {
            found = true;
            bare.to_string()
        } else {
            l.to_string()
        }
    }).collect();
    if !found {
        lines.push(wanted);
    }
    replace(gen, (lines.join("\n") + "\n").as_bytes(), 0o644)?;
    run(&["locale-gen"], None)
}

pub(super) fn locale_remove(args: &[String]) -> Outcome {
    let [code] = args else { return Err("usage: trindle-admin locale-remove ll_CC.UTF-8".into()) };
    if code.is_empty() || !code.chars().all(|c| c.is_ascii_alphanumeric() || "_@.-".contains(c)) {
        return Err(t!("admin-language-invalid").into());
    }
    let wanted = format!("{code} UTF-8");
    for file in ["/etc/default/locale", "/etc/locale.conf"] {
        let path = at(file);
        if !path.exists() { continue }
        let text = kept(&path)?;
        if text.lines().any(|l| l.split_once('=').is_some_and(|(k, v)| k.trim() == "LANG" && v.trim().trim_matches('"') == code)) {
            return Err(t!("admin-language-in-use", code = code));
        }
    }
    let gen = &at("/etc/locale.gen");
    let text = kept(gen)?;
    let lines: Vec<String> = text.lines().map(|l| {
        let on = !l.trim_start().starts_with('#') && l.trim_start().strip_prefix(&wanted).is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace));
        if on { format!("# {}", l.trim_start()) } else { l.to_string() }
    }).collect();
    replace(gen, (lines.join("\n") + "\n").as_bytes(), 0o644)?;
    run(&["locale-gen"], None)
}

pub fn valid_hostname(name: &str) -> bool {
    !name.is_empty() && name.len() <= 63
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !name.starts_with('-') && !name.ends_with('-')
}

pub(super) fn hostname(args: &[String]) -> Outcome {
    let [name] = args else { return Err("usage: trindle-admin hostname NAME".into()) };
    let name = name.to_lowercase();
    if !valid_hostname(&name) {
        return Err(t!("admin-hostname-invalid").into());
    }

    let hosts = &at("/etc/hosts");
    let was_there = hosts.exists();
    let text = kept(hosts)?;
    let mut found = false;
    let mut lines: Vec<String> = text.lines().map(|l| {
        if l.split_whitespace().next() == Some("127.0.1.1") {
            found = true;
            format!("127.0.1.1\t{name}")
        } else {
            l.to_string()
        }
    }).collect();
    if !found {
        lines.insert(1.min(lines.len()), format!("127.0.1.1\t{name}"));
    }

    replace(hosts, (lines.join("\n") + "\n").as_bytes(), 0o644)?;
    run(&["hostnamectl", "set-hostname", &name], None).map_err(|e| {
        let back = if was_there { replace(hosts, text.as_bytes(), 0o644) } else { gone(hosts) };
        and_undone(e, back)
    })
}

pub(super) fn update_checks(args: &[String]) -> Outcome {
    use crate::common::update_checks::{APT_OFF, APT_OFF_TEXT};
    match args {
        [w] if w == "off" => replace(&at(APT_OFF), APT_OFF_TEXT.as_bytes(), 0o644),
        [w] if w == "on" => gone(&at(APT_OFF)),
        _ => Err("usage: trindle-admin update-checks on|off".into()),
    }
}

pub(super) fn security_updates(args: &[String]) -> Outcome {
    use crate::common::security_updates::{APT_OFF, APT_OFF_TEXT};
    match args {
        [w] if w == "off" => replace(&at(APT_OFF), APT_OFF_TEXT.as_bytes(), 0o644),
        [w] if w == "on" => gone(&at(APT_OFF)),
        _ => Err("usage: trindle-admin security-updates on|off".into()),
    }
}

pub(super) fn charge_limit(args: &[String]) -> Outcome {
    use crate::common::charge;
    let on = match args {
        [w] if w == "on" => true,
        [w] if w == "off" => false,
        _ => return Err("usage: trindle-admin charge-limit on|off".into()),
    };
    charge_limit_with(on, &charge::apply)
}

fn charge_limit_with(on: bool, apply: &dyn Fn(bool) -> Outcome) -> Outcome {
    use crate::common::charge;
    apply(on)?;
    if on {
        std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
        replace(&at(charge::STATE), format!("{}\n", charge::LIMIT).as_bytes(), 0o644)?;
        run(&["systemctl", "enable", charge::SERVICE], None)
            .map_err(|e| t!("admin-charge-limit-lost", why = e))
    } else {
        gone(&at(charge::STATE))
            .map_err(|e| t!("admin-charge-limit-returns", why = e))?;

        // charging to full at a start, which is what was asked.
        let _ = run(&["systemctl", "disable", charge::SERVICE], None);
        Ok(())
    }
}

pub(super) fn radio_unblock(args: &[String]) -> Outcome {
    use crate::common::rfkill::{self, Radio};
    let radio = match args {
        [w] => Radio::from_word(w),
        _ => None,
    }.ok_or("usage: trindle-admin radio-unblock wifi|bluetooth")?;

    rfkill::unblock(radio).map_err(|why| radio_refusal(radio, why))
}

fn radio_refusal(radio: crate::common::rfkill::Radio, why: crate::common::rfkill::Refused) -> String {
    use crate::common::rfkill::{refused_words, Refused};
    match why {

        Refused::NotAllowed => refused_words(radio),
        Refused::Other(said) => t!("admin-radio-refused", words = refused_words(radio), said = said),
    }
}

pub(super) fn charge_apply() -> Outcome {
    crate::common::charge::apply(crate::common::charge::on())
}

#[cfg(test)]
mod tests {
    use super::valid_hostname;
    use crate::admin::stand_in::StandIn;
    use crate::common::charge;

    #[test]
    fn hostnames() {
        assert!(valid_hostname("laptop2"));
        assert!(valid_hostname("back-room-laptop"));
        assert!(!valid_hostname("-bad"));
        assert!(!valid_hostname("bad-"));
        assert!(!valid_hostname("has space"));
        assert!(!valid_hostname("dot.ted"));
        assert!(!valid_hostname(""));
    }

    #[test]
    fn a_charge_limit_that_cannot_be_kept_for_the_next_start_is_not_called_on() {
        let s = StandIn::failing("charge-on", "systemctl enable", "Failed to enable unit.");
        let said = super::charge_limit_with(true, &|_| Ok(())).unwrap_err();
        assert!(said.starts_with("The limit is on for now, but will be lost when the computer is switched off"), "{said}");
        assert!(said.ends_with("Failed to enable unit."), "{said}");
        drop(s);
        let s = StandIn::all_well("charge-on");
        assert!(super::charge_limit_with(true, &|_| Ok(())).is_ok());
        assert!(s.path(charge::STATE).is_file());
        assert_eq!(s.ran(), [format!("systemctl enable {}", charge::SERVICE)]);
    }

    #[test]
    fn a_charge_limit_that_cannot_be_forgotten_is_not_called_off() {
        let s = StandIn::all_well("charge-off");
        s.stuck(charge::STATE);
        let said = super::charge_limit_with(false, &|_| Ok(())).unwrap_err();
        assert!(said.starts_with("Charging is full for now, but the limit will come back at the next start"), "{said}");

        assert_eq!(super::charge_limit_with(false, &|_| Err("no".into())), Err("no".into()));
    }

    #[test]
    fn a_list_of_hosts_that_cannot_be_read_is_not_written_over_and_the_name_stays() {
        let s = StandIn::all_well("hosts");
        let was: &[u8] = b"127.0.0.1\tlocalhost\n10.0.0.9\tprinter\xff\n";
        s.write("/etc/hosts", was);
        let said = super::hostname(&["newname".to_string()]).unwrap_err();
        assert!(said.starts_with("Nothing was changed, because the file that keeps it is not text"), "{said}");
        assert_eq!(std::fs::read(s.path("/etc/hosts")).unwrap(), was);
        assert!(s.ran().is_empty(), "the name was changed though its list could not be");

        s.write("/etc/hosts", b"127.0.0.1\tlocalhost\n127.0.1.1\told\n10.0.0.9\tprinter\n");
        assert!(super::hostname(&["newname".to_string()]).is_ok());
        assert_eq!(std::fs::read_to_string(s.path("/etc/hosts")).unwrap(), "127.0.0.1\tlocalhost\n127.0.1.1\tnewname\n10.0.0.9\tprinter\n");
    }

    #[test]
    fn a_name_and_its_line_in_the_list_of_hosts_change_together_or_not_at_all() {
        const WAS: &str = "127.0.0.1\tlocalhost\n127.0.1.1\told\n";

        let s = StandIn::all_well("hosts-stuck");
        s.write("/etc/hosts", WAS.as_bytes());
        s.stuck("/etc/hosts.trindle-new");
        assert!(super::hostname(&["newname".to_string()]).is_err());
        assert!(s.ran().is_empty(), "the name was changed though its line could not be: {:?}", s.ran());
        assert_eq!(std::fs::read_to_string(s.path("/etc/hosts")).unwrap(), WAS);
        drop(s);

        let s = StandIn::failing("hosts-back", "hostnamectl", "Could not set the name.");
        s.write("/etc/hosts", WAS.as_bytes());
        assert_eq!(super::hostname(&["newname".to_string()]), Err("Could not set the name.".to_string()));
        assert_eq!(std::fs::read_to_string(s.path("/etc/hosts")).unwrap(), WAS);
        drop(s);

        let s = StandIn::failing("hosts-none", "hostnamectl", "Could not set the name.");
        std::fs::create_dir_all(s.path("/etc")).unwrap();
        assert!(super::hostname(&["newname".to_string()]).is_err());
        assert!(!s.path("/etc/hosts").exists());
    }

    #[test]
    fn a_language_is_taken_off_the_list_unless_the_system_starts_in_it() {
        let s = StandIn::all_well("locale");
        s.write("/etc/locale.gen", b"# comment\nen_GB.UTF-8 UTF-8\naa_DJ.UTF-8 UTF-8\n");
        s.write("/etc/default/locale", b"LANG=en_GB.UTF-8\n");
        let said = super::locale_remove(&["en_GB.UTF-8".to_string()]).unwrap_err();
        assert!(said.contains("en_GB.UTF-8"), "{said}");
        super::locale_remove(&["aa_DJ.UTF-8".to_string()]).unwrap();
        assert_eq!(std::fs::read_to_string(s.path("/etc/locale.gen")).unwrap(), "# comment\nen_GB.UTF-8 UTF-8\n# aa_DJ.UTF-8 UTF-8\n");
        assert_eq!(s.ran().len(), 1);
    }

    #[test]
    fn a_list_of_languages_that_cannot_be_read_is_not_written_over() {
        let s = StandIn::all_well("locale");
        s.write("/usr/share/i18n/SUPPORTED", b"en_GB.UTF-8 UTF-8\n");
        let was: &[u8] = b"# caf\xe9\nen_US.UTF-8 UTF-8\n";
        s.write("/etc/locale.gen", was);
        let said = super::locale(&["en_GB.UTF-8".to_string()]).unwrap_err();
        assert!(said.starts_with("Nothing was changed, because"), "{said}");
        assert_eq!(std::fs::read(s.path("/etc/locale.gen")).unwrap(), was);
        assert!(s.ran().is_empty());
    }

    #[test]
    fn a_block_that_would_not_come_off_is_said_with_what_the_system_said() {
        use crate::common::rfkill::{refused_words, Radio, Refused};
        let radio = Radio::from_word("wifi").unwrap();
        assert_eq!(super::radio_refusal(radio, Refused::NotAllowed), refused_words(radio));
        assert_eq!(super::radio_refusal(radio, Refused::Other("No such device (os error 19)".into())),
                   format!("{} No such device (os error 19)", refused_words(radio)));
    }
}
