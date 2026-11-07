//! The helper's steps for the firewall.

use std::ffi::CStr;

use super::accounts::{name_of, regular_user};
use super::{and_undone, at, gone, kept, replace, run, shares, sharing, Outcome};
use crate::toolkit::t;

pub(super) fn firewall_sharing(args: &[String]) -> Outcome {
    use crate::common::firewall;
    match args {
        [w] if w == "on" => {
            std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
            replace(&at(firewall::SHARING), b"on\n", 0o644)?;
        }
        [w] if w == "off" => {
            gone(&at(firewall::SHARING))
                .map_err(|e| t!("admin-sharing-still-on", why = e))?;
        }
        _ => return Err("usage: trindle-admin firewall-sharing on|off".into()),
    }
    sharing::apply(&[])
}

fn plugin_ports(e: &crate::common::firewall::Entitled) -> Result<Vec<crate::common::firewall::Door>, String> {
    if !crate::plugins::good_id(&e.id) {
        return Err(t!("admin-plugin-id-invalid").into());
    }
    let name = name_of(e.uid).ok_or(t!("admin-person-unknown"))?;
    regular_user(&name)?;
    let home = home_of(e.uid).ok_or(t!("admin-person-folder-unknown"))?;
    crate::plugins::ports_to_open(&home.join(".local/share/trindle/plugins").join(&e.id), e.uid)
}

pub(super) fn plugin_catalogue_restart() -> Outcome {
    let me = shares::caller()?;
    let home = home_of(me.uid).ok_or(t!("admin-person-folder-unknown"))?;
    crate::plugins::catalogue::start_again_in(&home)
}

fn home_of(uid: u32) -> Option<std::path::PathBuf> {
    use std::os::unix::ffi::OsStrExt;
    // SAFETY: getpwuid's entry is read at once, before anything else touches
    // the passwd database, and only after it is known not to be null; pw_dir
    // is a nul-ended string it keeps there.
    unsafe {
        let p = libc::getpwuid(uid);
        if p.is_null() || (*p).pw_dir.is_null() {
            return None;
        }
        let dir = std::path::PathBuf::from(std::ffi::OsStr::from_bytes(CStr::from_ptr((*p).pw_dir).to_bytes()));
        dir.is_absolute().then_some(dir)
    }
}

type Derived = Vec<(crate::common::firewall::Entitled, Result<Vec<crate::common::firewall::Door>, crate::common::firewall::Refusal>)>;

fn firewall_rules(entries: &[crate::common::firewall::Entitled]) -> (String, Derived) {
    use crate::common::firewall;
    let derived = firewall::derive(entries, &plugin_ports);
    let rules = firewall::rules(firewall::sharing(), crate::common::sharing::shared_now(), firewall::ssh_wanted(), &firewall::held_of(&derived));
    (rules, derived)
}

pub(super) fn firewall_apply() -> Outcome {
    use crate::common::firewall;
    let let_through = firewall::parse_entitled(&std::fs::read_to_string(at(firewall::PLUGINS)).unwrap_or_default());
    let (rules, _) = firewall_rules(&let_through);
    run(&["nft", "-f", "-"], Some(&rules))
}

fn firewall_commit(entries: &[crate::common::firewall::Entitled]) -> Outcome {
    use crate::common::firewall;
    let (rules, _) = firewall_rules(entries);
    run(&["nft", "-f", "-"], Some(&rules))?;
    let written = std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())
        .and_then(|()| replace(&at(firewall::PLUGINS), firewall::entitled_text(entries).as_bytes(), 0o644))
        .map_err(|e| t!("admin-plugins-list-not-kept", why = e));
    if let Err(e) = written {
        return Err(and_undone(e, firewall_apply()));
    }
    Ok(())
}

fn entitled_kept() -> Result<Vec<crate::common::firewall::Entitled>, String> {
    use crate::common::firewall;
    Ok(firewall::parse_entitled(&kept(&at(firewall::PLUGINS))?))
}

pub(super) fn firewall_plugin(args: &[String]) -> Outcome {
    use crate::common::firewall::{self, Entitled};
    let [id] = args else { return Err("usage: trindle-admin firewall-plugin ID".into()) };
    let me = shares::caller()?;
    let now = firewall::with(&entitled_kept()?, me.uid, id);
    let (_, derived) = firewall_rules(&now);
    let mine = Entitled { uid: me.uid, id: id.clone() };
    match derived.into_iter().find(|(e, _)| *e == mine).map(|(_, r)| r) {
        Some(Ok(_)) => firewall_commit(&now),
        Some(Err(r)) => Err(r.words(&|i| i.to_string())),
        None => Err(t!("admin-plugin-not-found").into()),
    }
}

pub(super) fn firewall_plugin_close(args: &[String]) -> Outcome {
    use crate::common::firewall;
    let [id] = args else { return Err("usage: trindle-admin firewall-plugin-close ID".into()) };
    let me = shares::caller()?;
    firewall_commit(&firewall::without(&entitled_kept()?, me.uid, id))
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;
    use crate::common::firewall;

    #[test]
    fn sharing_that_cannot_be_switched_off_is_not_called_off() {
        let s = StandIn::all_well("sharing-off");
        s.stuck(firewall::SHARING);
        let said = super::firewall_sharing(&["off".to_string()]).unwrap_err();
        assert!(said.starts_with("Other computers can still connect"), "{said}");
        assert!(s.ran().is_empty(), "the rules are not put in again as though the switch were off");
    }

    #[test]
    fn rules_that_could_not_be_put_back_are_said_with_what_failed() {
        let mut loads = 0;
        let s = StandIn::new("firewall", move |_| {
            loads += 1;
            if loads == 1 { Ok(()) } else { Err("nft: it would not load".into()) }
        });
        s.stuck(firewall::PLUGINS);
        let said = super::firewall_commit(&[]).unwrap_err();
        assert!(said.starts_with("The list of plugins let through could not be kept"), "{said}");
        assert!(said.ends_with("Putting things back as they were did not work either: nft: it would not load"), "{said}");
        assert_eq!(s.ran().len(), 2);
    }

    #[test]
    fn a_list_of_plugins_let_through_that_cannot_be_read_is_not_taken_for_empty() {
        let s = StandIn::all_well("firewall-list");
        s.write(firewall::PLUGINS, b"1000\torg.example.One\n\xff\n");
        assert!(super::entitled_kept().unwrap_err().starts_with("Nothing was changed, because"));
        s.write(firewall::PLUGINS, b"1000\torg.example.One\n");
        assert_eq!(super::entitled_kept().unwrap().len(), 1);
    }
}
