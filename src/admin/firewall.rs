//! The helper's steps for the firewall: sharing and the ports plugins are let through.

use std::ffi::CStr;

use super::accounts::{name_of, regular_user};
use super::{and_undone, at, gone, kept, replace, run, shares, sharing, Outcome};
use crate::toolkit::t;

pub(super) fn firewall_switch(args: &[String]) -> Outcome {
    use crate::common::firewall;
    let on = match args {
        [w] if w == "on" => true,
        [w] if w == "off" => false,
        _ => return Err("usage: trindle-admin firewall on|off".into()),
    };
    std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
    let was = at(firewall::ON).exists();
    let flag = |on: bool| if on {
        replace(&at(firewall::ON), b"on\n", 0o644)
    } else {
        gone(&at(firewall::ON)).map_err(|e| t!("admin-firewall-still-on", why = e))
    };
    flag(on)?;

    if let Err(e) = firewall_apply() {
        return Err(and_undone(e, flag(was).and_then(|()| firewall_apply())));
    }
    replace(&at(firewall::CHOSEN), b"chosen\n", 0o644)
}

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

type Doors = Vec<crate::common::firewall::Door>;

const APPROVED: &str = "/etc/trindle/firewall-plugin-ports";

type Approved = Vec<(crate::common::firewall::Entitled, Doors)>;

fn parse_approved(text: &str) -> Approved {
    use crate::common::firewall::{Door, Entitled};
    text.lines().filter_map(|l| {
        let mut f = l.split('\t');
        let (Some(uid), Some(id), Some(doors), None) = (f.next(), f.next(), f.next(), f.next()) else { return None };
        let doors: Result<Doors, _> = doors.split(';').map(Door::parse).collect();
        Some((Entitled { uid: uid.parse().ok()?, id: id.to_string() }, doors.ok()?))
    }).collect()
}

fn approved_text(all: &Approved) -> String {
    all.iter().fold(String::new(), |mut s, (e, doors)| {
        let doors: Vec<String> = doors.iter().map(ToString::to_string).collect();
        s.extend([e.uid.to_string(), "\t".into(), e.id.clone(), "\t".into(), doors.join(";"), "\n".into()]);
        s
    })
}

fn approved_now() -> Approved {
    parse_approved(&std::fs::read_to_string(at(APPROVED)).unwrap_or_default())
}

fn approved_kept() -> Result<Approved, String> {
    Ok(parse_approved(&kept(&at(APPROVED))?))
}

fn approved_only(e: &crate::common::firewall::Entitled, found: Doors, approved: &Approved) -> Result<Doors, String> {
    let sorted = |d: &Doors| { let mut d = d.clone(); d.sort(); d };
    match approved.iter().find(|(a, _)| a == e) {
        Some((_, doors)) if sorted(doors) == sorted(&found) => Ok(found),
        _ => Err(t!("admin-plugin-ports-changed").into()),
    }
}

fn plugin_ports(e: &crate::common::firewall::Entitled, approved: &Approved) -> Result<Doors, String> {
    approved_only(e, ports_in_files(e)?, approved)
}

fn ports_in_files(e: &crate::common::firewall::Entitled) -> Result<Doors, String> {
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

fn firewall_rules(entries: &[crate::common::firewall::Entitled], approved: &Approved) -> (String, Derived) {
    use crate::common::firewall;
    let derived = firewall::derive(entries, &|e| plugin_ports(e, approved));
    let own = firewall::parse_own_list(&std::fs::read_to_string(at(firewall::PORTS)).unwrap_or_default());
    (table(&derived, &own), derived)
}

fn table(derived: &Derived, own: &[crate::common::firewall::Door]) -> String {
    use crate::common::firewall;
    if !at(firewall::ON).exists() {
        return firewall::OFF.into();
    }
    firewall::rules_with(firewall::sharing(), crate::common::sharing::shared_now(), firewall::ssh_wanted(), &firewall::held_of(derived), own)
}

pub(super) fn firewall_apply() -> Outcome {
    use crate::common::firewall;

    let let_through = firewall::parse_entitled(&std::fs::read_to_string(at(firewall::PLUGINS)).unwrap_or_default());
    let (rules, _) = firewall_rules(&let_through, &approved_now());
    run(&["nft", "-f", "-"], Some(&rules))
}

fn firewall_commit(entries: &[crate::common::firewall::Entitled], approved: &Approved) -> Outcome {
    use crate::common::firewall;
    let (rules, _) = firewall_rules(entries, approved);
    run(&["nft", "-f", "-"], Some(&rules))?;
    let written = std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())
        .and_then(|()| replace(&at(APPROVED), approved_text(approved).as_bytes(), 0o644))
        .and_then(|()| replace(&at(firewall::PLUGINS), firewall::entitled_text(entries).as_bytes(), 0o644))
        .map_err(|e| t!("admin-plugins-list-not-kept", why = e));
    if let Err(e) = written {
        return Err(and_undone(e, firewall_apply()));
    }
    Ok(())
}

pub(super) fn firewall_port(args: &[String]) -> Outcome {
    use crate::common::firewall;
    let [verb, door] = args else { return Err("usage: trindle-admin firewall-port open|close tcp:PORT|udp:PORT".into()) };
    let door = firewall::parse_own(door)?;

    std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
    let _held = held(&at(format!("{}.lock", firewall::PORTS))).map_err(|e| e.to_string())?;
    let mut list = firewall::parse_own_list(&kept(&at(firewall::PORTS))?);
    match verb.as_str() {
        "open" if list.contains(&door) => return Ok(()),
        "open" if list.len() >= firewall::MAX_OWN => return Err(t!("admin-ports-too-many", most = &firewall::MAX_OWN.to_string())),
        "open" => list.push(door),
        "close" => list.retain(|d| *d != door),
        _ => return Err("usage: trindle-admin firewall-port open|close tcp:PORT|udp:PORT".into()),
    }
    let list = firewall::parse_own_list(&firewall::own_list_text(&list));
    let (_, derived) = firewall_rules(&entitled_kept()?, &approved_now());
    run(&["nft", "-f", "-"], Some(&table(&derived, &list)))?;
    let written = std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())
        .and_then(|()| replace(&at(firewall::PORTS), firewall::own_list_text(&list).as_bytes(), 0o644))
        .map_err(|e| t!("admin-ports-not-kept", why = e));
    if let Err(e) = written {
        return Err(and_undone(e, firewall_apply()));
    }
    Ok(())
}

fn held(path: &std::path::Path) -> std::io::Result<std::fs::File> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().write(true).create(true).truncate(false).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW).open(path)?;
    loop {
        // SAFETY: flock on a file this function has just opened and owns.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } == 0 {
            return Ok(file);
        }
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::Interrupted {
            return Err(e);
        }
    }
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
    let mine = Entitled { uid: me.uid, id: id.clone() };

    let mut approved = approved_kept()?;
    approved.retain(|(e, _)| *e != mine);
    approved.push((mine.clone(), ports_in_files(&mine)?));
    let (_, derived) = firewall_rules(&now, &approved);
    match derived.into_iter().find(|(e, _)| *e == mine).map(|(_, r)| r) {
        Some(Ok(_)) => firewall_commit(&now, &approved),
        Some(Err(r)) => Err(r.words(&|i| i.to_string())),
        None => Err(t!("admin-plugin-not-found").into()),
    }
}

pub(super) fn firewall_plugin_close(args: &[String]) -> Outcome {
    use crate::common::firewall;
    let [id] = args else { return Err("usage: trindle-admin firewall-plugin-close ID".into()) };
    let me = shares::caller()?;
    let mut approved = approved_kept()?;
    approved.retain(|(e, _)| !(e.uid == me.uid && e.id == *id));
    firewall_commit(&firewall::without(&entitled_kept()?, me.uid, id), &approved)
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;
    use crate::common::firewall;

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn the_firewall_switch_goes_back_when_its_rules_will_not_load() {
        let s = StandIn::failing("fw-on", "nft", "nft: it would not load");
        let said = super::firewall_switch(&words(&["on"])).unwrap_err();
        assert!(said.contains("it would not load"), "{said}");
        assert!(!s.path(firewall::ON).exists(), "still off, as it was");
        assert!(!s.path(firewall::CHOSEN).exists());

        let mut loads = 0;
        let s = StandIn::new("fw-off", move |_| { loads += 1; if loads == 1 { Err("nft: no".into()) } else { Ok(()) } });
        s.write(firewall::ON, b"on\n");
        let said = super::firewall_switch(&words(&["off"])).unwrap_err();
        assert_eq!(said, "nft: no", "put back as it was, and nothing more to say");
        assert!(s.path(firewall::ON).exists(), "still on, as it was");
        assert_eq!(s.ran().len(), 2, "the rules were loaded again as they were");

        let s = StandIn::all_well("fw-on-well");
        super::firewall_switch(&words(&["on"])).unwrap();
        assert!(s.path(firewall::ON).exists() && s.path(firewall::CHOSEN).exists());
        super::firewall_switch(&words(&["off"])).unwrap();
        assert!(!s.path(firewall::ON).exists());
        assert!(super::firewall_switch(&words(&["maybe"])).is_err());
    }

    #[test]
    fn ports_open_and_close_one_at_a_time_up_to_the_most() {
        let s = StandIn::all_well("ports");
        let list = || std::fs::read_to_string(s.path(firewall::PORTS)).unwrap_or_default();
        super::firewall_port(&words(&["open", "tcp:37666"])).unwrap();
        super::firewall_port(&words(&["open", "tcp:37666"])).unwrap();
        super::firewall_port(&words(&["open", "udp:37666"])).unwrap();
        assert_eq!(list(), "tcp:37666\nudp:37666\n");
        super::firewall_port(&words(&["close", "tcp:37666"])).unwrap();
        assert_eq!(list(), "udp:37666\n");
        for bad in [["open", "tcp:0"], ["open", "sctp:5"], ["open", "tcp:65536"], ["shut", "tcp:5"]] {
            assert!(super::firewall_port(&words(&bad)).is_err(), "{bad:?}");
        }
        use std::fmt::Write as _;
        let full: String = (2000..2000 + firewall::MAX_OWN as u16).fold(String::new(), |mut t, p| {
            let _ = writeln!(t, "tcp:{p}");
            t
        });
        s.write(firewall::PORTS, full.as_bytes());
        let said = super::firewall_port(&words(&["open", "tcp:9"])).unwrap_err();
        assert!(said.contains(&firewall::MAX_OWN.to_string()), "{said}");
        assert_eq!(list(), full);
    }

    #[test]
    fn a_port_change_that_will_not_load_keeps_the_list_and_one_unreadable_is_not_written_over() {
        let s = StandIn::failing("ports-fail", "nft", "nft: it would not load");
        s.write(firewall::PORTS, b"tcp:8000\n");
        assert!(super::firewall_port(&words(&["open", "tcp:37666"])).is_err());
        assert_eq!(std::fs::read_to_string(s.path(firewall::PORTS)).unwrap(), "tcp:8000\n");
        let s = StandIn::all_well("ports-unreadable");
        let was: &[u8] = b"tcp:8000\n\xff\xfe\n";
        s.write(firewall::PORTS, was);
        assert!(super::firewall_port(&words(&["open", "tcp:37666"])).is_err());
        assert_eq!(std::fs::read(s.path(firewall::PORTS)).unwrap(), was);
        assert!(s.ran().is_empty(), "nothing loaded from a list that could not be read");
    }

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
        let said = super::firewall_commit(&[], &Vec::new()).unwrap_err();
        assert!(said.starts_with("Trindle couldn't save the list of allowed plugins"), "{said}");
        assert!(said.ends_with("Trindle also couldn't put things back as they were: nft: it would not load"), "{said}");
        assert_eq!(s.ran().len(), 2);
    }

    fn who(id: &str) -> firewall::Entitled {
        firewall::Entitled { uid: 1000, id: id.into() }
    }

    fn doors(list: &[&str]) -> Vec<firewall::Door> {
        list.iter().map(|d| firewall::Door::parse(d).unwrap()).collect()
    }

    #[test]
    fn a_plugin_whose_files_now_ask_for_other_ports_than_were_approved_gets_none() {
        let approved = vec![(who("org.example.One"), doors(&["tcp:24810", "udp:24811"]))];
        let ok = super::approved_only(&who("org.example.One"), doors(&["udp:24811", "tcp:24810"]), &approved);
        assert_eq!(ok.unwrap().len(), 2);
        let more = super::approved_only(&who("org.example.One"), doors(&["tcp:24810", "udp:24811", "tcp:30000"]), &approved);
        assert!(more.is_err());
        let other = super::approved_only(&who("org.example.One"), doors(&["tcp:30000"]), &approved);
        assert!(other.is_err());
        let unrecorded = super::approved_only(&who("org.example.Two"), doors(&["tcp:24810"]), &approved);
        assert!(unrecorded.is_err(), "a plugin with no record of an approval opens nothing");
    }

    #[test]
    fn the_approved_ports_are_kept_and_read_back() {
        let approved = vec![(who("org.example.One"), doors(&["tcp:24810", "udp:24811"]))];
        let text = super::approved_text(&approved);
        assert_eq!(text, "1000\torg.example.One\ttcp:24810;udp:24811\n");
        assert_eq!(super::parse_approved(&text), approved);
        assert!(super::parse_approved("1000\tx\ttcp:80\nnonsense\n1000\ty\n").is_empty());
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
