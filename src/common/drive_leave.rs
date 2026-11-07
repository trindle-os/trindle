//! The password before a drive is changed.

use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use crate::toolkit::bus::{Bus, Failed};
use crate::toolkit::event::Loop;

pub const AUTHORITY: (&str, &str, &str) = ("org.freedesktop.PolicyKit1", "/org/freedesktop/PolicyKit1/Authority",
                                           "org.freedesktop.PolicyKit1.Authority");
pub const UDISKS: &str = "org.freedesktop.udisks2.";

pub const CHANGING: [&str; 16] = [
    "modify-device", "modify-device-system", "modify-device-other-seat", "modify-drive-settings",
    "modify-system-configuration", "filesystem-fstab", "filesystem-take-ownership",
    "ata-secure-erase", "nvme-format-namespace", "nvme-sanitize",
    "manage-swapspace", "manage-md-raid",
    "encrypted-change-passphrase", "encrypted-change-passphrase-system",
    "open-device", "open-device-system",
];

pub const DRIVES_ASKS: [&str; 4] = ["modify-device", "modify-device-system", "modify-device-other-seat", "modify-system-configuration"];

pub fn would_ask() -> Option<bool> {
    let actions: Vec<String> = DRIVES_ASKS.iter().map(|a| format!("{UDISKS}{a}")).collect();
    super::refused::would_ask(&actions.iter().map(String::as_str).collect::<Vec<_>>())
}

pub fn prime() {
    let actions: Vec<String> = DRIVES_ASKS.iter().map(|a| format!("{UDISKS}{a}")).collect();
    super::refused::prime(&actions.iter().map(String::as_str).collect::<Vec<_>>());
}

pub const EVERY_TIME: &str = "It is asked for every time a drive is changed. Closing this box changes nothing.";

const WAIT: Duration = Duration::from_secs(5);

pub fn changes(action: &str) -> bool {
    action.strip_prefix(UDISKS).is_some_and(|a| CHANGING.contains(&a))
}

pub fn said(action: &str, message: &str, details: &HashMap<String, String>) -> Option<String> {
    if !changes(action) {
        return None;
    }
    let what = message.trim().trim_end_matches('.');
    let mut what = match what.strip_prefix("Authentication is required to ") {
        Some(rest) => format!("An administrator's password is needed to {rest}"),
        None if what.is_empty() => "An administrator's password is needed to change this drive".to_string(),
        None => what.to_string(),
    };
    let device = details.get("device").map(|d| d.trim()).unwrap_or_default();
    if !device.is_empty() && !what.contains(device) {
        what.push_str(&format!(" ({device})"));
    }
    Some(format!("{what}. {EVERY_TIME}"))
}

pub fn session_id(bus: &Bus, lp: &Loop, done: impl FnOnce(Result<String, Failed>) + 'static) {
    if let Ok(sid) = std::env::var("XDG_SESSION_ID") {
        if !sid.is_empty() {
            done(Ok(sid));
            return;
        }
    }
    let (b, l) = (bus.clone(), lp.clone());
    bus.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
        .method("GetSessionByPID", &(std::process::id(),), false, Some(WAIT), lp, move |r| {
            let path = r.and_then(|m| m.body().deserialize::<(OwnedObjectPath,)>().map_err(Failed::from));
            let path = match path {
                Ok(p) => p.0.to_string(),
                Err(e) => return done(Err(e)),
            };
            b.call("org.freedesktop.login1", &path, "org.freedesktop.DBus.Properties")
                .method("Get", &("org.freedesktop.login1.Session", "Id"), false, Some(WAIT), &l, move |r| {
                    let id = r.and_then(|m| m.body().deserialize::<(OwnedValue,)>().map_err(Failed::from));
                    done(id.map(|v| <&str>::try_from(&v.0).map(String::from).unwrap_or_default()))
                });
        });
}

type Kept = (String, String, (String, HashMap<String, OwnedValue>), u64, u64);

pub fn take_back(bus: &Bus, lp: &Loop, done: impl FnOnce(Result<(), String>) + 'static) {
    let (b, l) = (bus.clone(), lp.clone());
    session_id(bus, lp, move |sid| match sid {
        Ok(sid) if !sid.is_empty() => take_back_for(&b, &l, &sid, done),
        Ok(_) => done(Err("this login has no session of its own".into())),
        Err(e) => done(Err(e.to_string())),
    });
}

pub fn take_back_for(bus: &Bus, lp: &Loop, sid: &str, done: impl FnOnce(Result<(), String>) + 'static) {
    let mut details: HashMap<String, Value> = HashMap::new();
    details.insert("session-id".to_string(), Value::from(sid.to_string()));
    let (name, path, iface) = AUTHORITY;
    let (b, l) = (bus.clone(), lp.clone());
    bus.call(name, path, iface).method("EnumerateTemporaryAuthorizations", &(("unix-session".to_string(), details),), false, Some(WAIT), lp, move |r| {
        let kept = r.and_then(|m| m.body().deserialize::<(Vec<Kept>,)>().map_err(Failed::from));
        match kept {
            Err(e) => done(Err(e.to_string())),
            Ok((kept,)) => {
                let ids: VecDeque<String> = kept.into_iter().filter(|k| changes(&k.1)).map(|k| k.0).collect();
                revoke(b, l, ids, Box::new(done));
            }
        }
    });
}

fn revoke(bus: Bus, lp: Loop, mut ids: VecDeque<String>, done: Box<dyn FnOnce(Result<(), String>)>) {
    let Some(id) = ids.pop_front() else { return done(Ok(())) };
    let (name, path, iface) = AUTHORITY;
    let (b, l) = (bus.clone(), lp.clone());
    bus.call(name, path, iface).method("RevokeTemporaryAuthorizationById", &(id,), false, Some(WAIT), &lp, move |r| match r {
        Err(e) => done(Err(e.to_string())),
        Ok(_) => revoke(b, l, ids, done),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::toolkit::bus::testing::{own_daemon, until};
    use crate::toolkit::bus::{Asked, Offer, Props};

    const RULES: &str = include_str!("../../system/polkit/60-trindle.rules");

    #[test]
    fn the_rule_file_asks_for_every_action_that_changes_a_drive() {
        let start = RULES.find("var changing = [").expect("the rule for drives");
        let rule = &RULES[start..];
        let rule = &rule[..rule.find("});").expect("the rule ends")];
        let listed: Vec<&str> = rule.split('"').filter(|w| w.starts_with(UDISKS)).collect();
        let want: Vec<String> = CHANGING.iter().map(|a| format!("{UDISKS}{a}")).collect();
        assert_eq!(listed, want.iter().map(String::as_str).collect::<Vec<_>>(), "the rule file and the list say the same, in the same order");
        assert!(rule.contains("return polkit.Result.AUTH_ADMIN_KEEP;"), "{rule}");
        assert!(!rule.contains("Result.YES"), "nothing that changes a drive is waved through");
        assert!(!RULES[..start].contains("udisks2"), "an earlier rule would answer first");
        for free in ["filesystem-mount", "filesystem-mount-system", "eject-media", "power-off-drive", "encrypted-unlock", "rescan"] {
            assert!(!changes(&format!("{UDISKS}{free}")), "{free}");
        }
        assert!(changes("org.freedesktop.udisks2.modify-device") && changes("org.freedesktop.udisks2.modify-device-system"));
        for own in DRIVES_ASKS {
            assert!(CHANGING.contains(&own), "{own}");
        }
        assert!(!changes("modify-device") && !changes("org.example.modify-device"));
    }

    #[test]
    fn every_rule_that_reads_a_command_line_copes_with_none() {
        for (n, l) in RULES.lines().enumerate() {
            if l.contains("lookup(") {
                assert!(l.trim_end().ends_with("|| \"\";"), "line {}: {l}", n + 1);
            }
        }
        assert!(RULES.contains("lookup(\"command_line\") || \"\""));
    }

    #[test]
    fn the_box_says_which_drive_and_what_and_that_it_is_asked_every_time() {
        let details = HashMap::from([("device".to_string(), "/dev/sdb1".to_string())]);
        let words = said("org.freedesktop.udisks2.modify-device-system", "Authentication is required to format Example Disk (/dev/sdb1)", &details).unwrap();
        assert_eq!(words, format!("An administrator's password is needed to format Example Disk (/dev/sdb1). {EVERY_TIME}"));
        let words = said("org.freedesktop.udisks2.modify-device", "Authentication is required to delete the partition.", &details).unwrap();
        assert!(words.starts_with("An administrator's password is needed to delete the partition (/dev/sdb1). "), "{words}");
        let words = said("org.freedesktop.udisks2.filesystem-fstab", "Exemple de phrase", &HashMap::new()).unwrap();
        assert_eq!(words, format!("Exemple de phrase. {EVERY_TIME}"));
        assert!(said("org.freedesktop.udisks2.modify-device", "", &HashMap::new()).unwrap().starts_with("An administrator's password is needed to change this drive. "));
        assert_eq!(said("org.freedesktop.udisks2.filesystem-mount-system", "Example", &details), None);
        assert_eq!(said("org.example.other", "Example", &details), None);
        assert!(!EVERY_TIME.contains('\u{2014}'));
    }

    #[test]
    fn leave_kept_for_a_drive_is_taken_back_and_other_leave_is_left() {
        let d = own_daemon();
        assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
        let lp = Loop::new();
        let fake = Bus::at(&lp, &d.1).expect("the fake's bus");
        let named = Rc::new(std::cell::Cell::new(false));
        let n = named.clone();
        fake.own(AUTHORITY.0, false, &lp, move |ok| n.set(ok));
        assert!(until(&lp, move || named.get()), "the fake owns the name");
        let calls: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let c = calls.clone();
        let _served = fake.serve(AUTHORITY.1, Offer {
            name: AUTHORITY.2.into(),
            xml: format!("<interface name=\"{}\"/>", AUTHORITY.2),
            call: Box::new(move |a: Asked| match a.member().as_str() {
                "EnumerateTemporaryAuthorizations" => {
                    let (who,): ((String, HashMap<String, OwnedValue>),) = a.message().body().deserialize().expect("a subject");
                    let sid = who.1.get("session-id").and_then(|v| <&str>::try_from(v).ok()).unwrap_or_default().to_string();
                    c.borrow_mut().push(format!("list {} {sid}", who.0));
                    let subject = || ("unix-process".to_string(), HashMap::<String, OwnedValue>::new());
                    let kept: Vec<Kept> = vec![
                        ("tmpauthz1".into(), "org.freedesktop.udisks2.modify-device-system".into(), subject(), 10, 310),
                        ("tmpauthz2".into(), "org.freedesktop.udisks2.filesystem-mount-system".into(), subject(), 10, 310),
                        ("tmpauthz3".into(), "org.example.settings".into(), subject(), 10, 310),
                        ("tmpauthz4".into(), "org.freedesktop.udisks2.modify-device".into(), subject(), 10, 310),
                    ];
                    a.reply(&(kept,));
                }
                "RevokeTemporaryAuthorizationById" => {
                    let (id,): (String,) = a.message().body().deserialize().expect("a name");
                    c.borrow_mut().push(format!("take back {id}"));
                    a.reply(&());
                }
                other => a.fail("org.freedesktop.DBus.Error.UnknownMethod", other),
            }),
            props: Box::new(Props::new),
            set: None,
        }).expect("served");
        let client = Bus::at(&lp, &d.1).expect("the client's bus");
        let got: Rc<RefCell<Option<Result<(), String>>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        take_back_for(&client, &lp, "c7", move |r| *g.borrow_mut() = Some(r));
        let g = got.clone();
        assert!(until(&lp, move || g.borrow().is_some()));
        assert_eq!(got.borrow().clone(), Some(Ok(())));
        assert_eq!(calls.borrow().clone(), vec!["list unix-session c7", "take back tmpauthz1", "take back tmpauthz4"]);
    }

    #[test]
    fn with_nobody_to_ask_it_says_it_could_not_make_sure() {
        let d = own_daemon();
        let lp = Loop::new();
        let client = Bus::at(&lp, &d.1).expect("the client's bus");
        let got: Rc<RefCell<Option<Result<(), String>>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        take_back_for(&client, &lp, "c7", move |r| *g.borrow_mut() = Some(r));
        let g = got.clone();
        assert!(until(&lp, move || g.borrow().is_some()));
        assert!(got.borrow().as_ref().is_some_and(|r| r.is_err()));
    }
}
