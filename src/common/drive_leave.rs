//! The password before a drive is changed: nothing is done to a drive
//! until a password has been entered.

use crate::toolkit::t;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use zbus::Message;
use zbus::zvariant::{Endian, OwnedObjectPath, OwnedValue, Value};

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
    super::refused::would_ask(&drive_actions().iter().map(String::as_str).collect::<Vec<_>>())
}

fn drive_actions() -> Vec<String> {
    DRIVES_ASKS.iter().map(|a| format!("{UDISKS}{a}")).collect()
}

pub fn prime() {
    super::refused::prime(&drive_actions().iter().map(String::as_str).collect::<Vec<_>>());
}

pub fn every_time() -> &'static str { t!("common-drive-every-time") }

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
        Some(rest) => t!("common-drive-password-needed-to", rest = rest).to_string(),
        None if what.is_empty() => t!("common-drive-password-needed").to_string(),
        None => what.to_string(),
    };
    let device = details.get("device").map(|d| d.trim()).unwrap_or_default();
    if !device.is_empty() && !what.contains(device) {
        what.push_str(&format!(" ({device})"));
    }
    Some(format!("{what}. {}", every_time()))
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

const KEPT: &str = "a(ss(sa{sv})tt)";

const KEPT_MOST: usize = 4096;

fn unread() -> String {
    t!("common-leave-list-unread").to_string()
}

fn drive_leave_ids(m: &Message) -> Result<Vec<String>, String> {
    let body = m.body();
    let sig = body.signature().to_string();
    if sig != KEPT {
        return Err(unread());
    }
    let data = body.data();
    let ctxt = data.context();
    let mut r = Reader { b: data.bytes(), at: 0, base: ctxt.position(), big: matches!(ctxt.endian(), Endian::Big) };
    let len = r.u32()? as usize;
    r.align(8)?;
    let end = r.at.checked_add(len).filter(|&e| e <= r.b.len()).ok_or_else(unread)?;
    let (mut ids, mut seen) = (Vec::new(), 0usize);
    while r.at < end {
        seen += 1;
        if seen > KEPT_MOST {
            return Err(unread());
        }
        r.align(8)?;
        let id = r.string()?;
        let action = r.string()?;
        r.align(8)?;
        r.string()?;
        let subject = r.u32()? as usize;
        r.align(8)?;
        r.skip(subject)?;
        r.align(8)?;
        r.skip(16)?;
        if changes(&action) {
            ids.push(id);
        }
    }
    if r.at != end {
        return Err(unread());
    }
    Ok(ids)
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,

    base: usize,
    big: bool,
}

impl Reader<'_> {
    fn skip(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.b.len()).ok_or_else(unread)?;
        let got = &self.b[self.at..end];
        self.at = end;
        Ok(got)
    }

    fn align(&mut self, n: usize) -> Result<(), String> {
        let pad = (n - (self.base + self.at) % n) % n;
        self.skip(pad).map(|_| ())
    }

    fn u32(&mut self) -> Result<u32, String> {
        self.align(4)?;
        let big = self.big;
        let w: [u8; 4] = self.skip(4)?.try_into().map_err(|_| unread())?;
        Ok(if big { u32::from_be_bytes(w) } else { u32::from_le_bytes(w) })
    }

    fn string(&mut self) -> Result<String, String> {
        let n = self.u32()? as usize;
        let s = String::from_utf8(self.skip(n)?.to_vec()).map_err(|_| unread())?;
        match self.skip(1)? {
            [0] => Ok(s),
            _ => Err(unread()),
        }
    }
}

pub fn take_back(bus: &Bus, lp: &Loop, done: impl FnOnce(Result<(), String>) + 'static) {
    let (b, l) = (bus.clone(), lp.clone());
    session_id(bus, lp, move |sid| match sid {
        Ok(sid) if !sid.is_empty() => take_back_for(&b, &l, &sid, done),
        Ok(_) => done(Err(t!("common-leave-no-session").to_string())),
        Err(e) => done(Err(e.to_string())),
    });
}

pub fn take_back_for(bus: &Bus, lp: &Loop, sid: &str, done: impl FnOnce(Result<(), String>) + 'static) {
    let mut details: HashMap<String, Value> = HashMap::new();
    details.insert("session-id".to_string(), Value::from(sid.to_string()));
    let (name, path, iface) = AUTHORITY;
    let (b, l) = (bus.clone(), lp.clone());
    bus.call(name, path, iface).method("EnumerateTemporaryAuthorizations", &(("unix-session".to_string(), details),), false, Some(WAIT), lp, move |r| {
        match r.map_err(|e| e.to_string()).and_then(|m| drive_leave_ids(&m)) {
            Err(e) => done(Err(e)),
            Ok(ids) => revoke(b, l, ids.into(), Box::new(done)),
        }
    });
}

fn revoke(bus: Bus, lp: Loop, mut ids: VecDeque<String>, done: Box<dyn FnOnce(Result<(), String>)>) {
    let Some(id) = ids.pop_front() else { return done(Ok(())) };
    let (name, path, iface) = AUTHORITY;
    let (b, l) = (bus.clone(), lp.clone());
    bus.call(name, path, iface).method("RevokeTemporaryAuthorizationById", &(id.clone(),), false, Some(WAIT), &lp, move |r| match r {
        Err(e) => done(Err(format!("{id}: {e}"))),
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

    type Kept = (String, String, (String, HashMap<String, OwnedValue>), u64, u64);

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
        assert_eq!(words, format!("An administrator's password is needed to format Example Disk (/dev/sdb1). {}", every_time()));

        let words = said("org.freedesktop.udisks2.modify-device", "Authentication is required to delete the partition.", &details).unwrap();
        assert!(words.starts_with("An administrator's password is needed to delete the partition (/dev/sdb1). "), "{words}");

        let words = said("org.freedesktop.udisks2.filesystem-fstab", "Exemple de phrase", &HashMap::new()).unwrap();
        assert_eq!(words, format!("Exemple de phrase. {}", every_time()));
        assert!(said("org.freedesktop.udisks2.modify-device", "", &HashMap::new()).unwrap().starts_with("An administrator's password is needed to change this drive. "));

        assert_eq!(said("org.freedesktop.udisks2.filesystem-mount-system", "Example", &details), None);
        assert_eq!(said("org.example.other", "Example", &details), None);
        assert!(!every_time().contains('\u{2014}'));
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
    fn a_leave_polkit_will_not_take_back_is_said_with_polkits_reason() {
        let d = own_daemon();
        let lp = Loop::new();
        let fake = Bus::at(&lp, &d.1).expect("the fake's bus");
        let named = Rc::new(std::cell::Cell::new(false));
        let n = named.clone();
        fake.own(AUTHORITY.0, false, &lp, move |ok| n.set(ok));
        assert!(until(&lp, move || named.get()));
        let calls: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let c = calls.clone();
        let _served = fake.serve(AUTHORITY.1, Offer {
            name: AUTHORITY.2.into(),
            xml: format!("<interface name=\"{}\"/>", AUTHORITY.2),
            call: Box::new(move |a: Asked| {
                c.borrow_mut().push(a.member());
                match a.member().as_str() {
                    "EnumerateTemporaryAuthorizations" => {
                        let subject = ("unix-session".to_string(), HashMap::<String, OwnedValue>::new());
                        let kept: Vec<Kept> = vec![("tmpauthz1".into(), "org.freedesktop.udisks2.modify-device-system".into(), subject, 10, 310)];
                        a.reply(&(kept,));
                    }
                    "RevokeTemporaryAuthorizationById" => a.fail("org.freedesktop.PolicyKit1.Error.Failed", "Example refusal"),
                    other => a.fail("org.freedesktop.DBus.Error.UnknownMethod", other),
                }
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
        let why = got.borrow().clone().expect("an answer").expect_err("not called safe");
        assert!(why.contains("tmpauthz1") && why.contains("Example refusal"), "{why}");
        assert_eq!(calls.borrow().clone(), vec!["EnumerateTemporaryAuthorizations", "RevokeTemporaryAuthorizationById"]);
    }

    fn real_answer() -> Vec<u8> {
        use zbus::zvariant::{Fd, serialized::Context, to_bytes};
        let null = std::fs::File::open("/dev/null").expect("/dev/null");
        let subject = || {
            let map: HashMap<String, Value> = HashMap::from([
                ("pid".to_string(), Value::from(2245u32)),
                ("start-time".to_string(), Value::from(13678u64)),
                ("uid".to_string(), Value::from(1000i32)),
                ("pidfd".to_string(), Value::from(Fd::from(std::os::fd::AsFd::as_fd(&null)))),
            ]);
            ("unix-process".to_string(), map)
        };
        let kept: Vec<(String, String, (String, HashMap<String, Value>), u64, u64)> = vec![
            ("tmpauthz0".into(), "org.trindle.admin".into(), subject(), 1_700_000_000, 1_700_000_300),
            ("tmpauthz1".into(), "org.freedesktop.udisks2.modify-device-system".into(), subject(), 1_700_000_000, 1_700_000_300),
        ];
        to_bytes(Context::new_dbus(Endian::Little, 0), &(kept,)).expect("the answer").bytes().to_vec()
    }

    fn real_polkit(address: &str, calls: std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
        let conn = zbus::blocking::connection::Builder::address(address).expect("an address")
            .name(AUTHORITY.0).expect("the name").build().expect("the fake's bus");
        let body = real_answer();
        std::thread::spawn(move || {
            for m in zbus::blocking::MessageIterator::from(&conn).flatten() {
                let h = m.header();
                if h.message_type() != zbus::message::Type::MethodCall {
                    continue;
                }
                let member = h.member().map(|m| m.to_string()).unwrap_or_default();
                let reply = Message::method_return(&h).expect("a reply");
                let reply = match member.as_str() {
                    "EnumerateTemporaryAuthorizations" => {
                        // SAFETY: the bytes are a whole a(ss(sa{sv})tt) from zvariant; the handle in them

                        unsafe { reply.build_raw_body(&body, KEPT, Vec::new()) }
                    }
                    "RevokeTemporaryAuthorizationById" => {
                        let (id,): (String,) = m.body().deserialize().expect("a name");
                        calls.lock().expect("calls").push(format!("take back {id}"));
                        reply.build(&())
                    }
                    _ => continue,
                };
                let _ = conn.send(&reply.expect("built"));
            }
        });
    }

    #[test]
    fn leave_polkit_lists_as_a_real_install_does_is_taken_back() {
        let d = own_daemon();
        assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
        let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        real_polkit(&d.1, calls.clone());
        let lp = Loop::new();
        let client = Bus::at(&lp, &d.1).expect("the client's bus");
        let got: Rc<RefCell<Option<Result<(), String>>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        take_back_for(&client, &lp, "3", move |r| *g.borrow_mut() = Some(r));
        let g = got.clone();
        assert!(until(&lp, move || g.borrow().is_some()));
        assert_eq!(got.borrow().clone(), Some(Ok(())));
        assert_eq!(calls.lock().expect("calls").clone(), vec!["take back tmpauthz1"]);
    }

    #[test]
    fn a_broken_list_from_polkit_is_a_reason() {
        let whole = real_answer();
        for cut in [0, 3, 4, 9, whole.len() / 2, whole.len() - 1] {
            let m = Message::method_call("/", "Example").expect("a call");
            // SAFETY: a cut answer is the invalid message this test is for; the reader holds every step inside it.
            let m = unsafe { m.build_raw_body(&whole[..cut], KEPT, Vec::new()) }.expect("built");
            assert!(drive_leave_ids(&m).is_err(), "cut at {cut}");
        }
        let m = Message::method_call("/", "Example").expect("a call");
        // SAFETY: the whole answer, as real_answer made it.
        let m = unsafe { m.build_raw_body(&whole, KEPT, Vec::new()) }.expect("built");
        assert_eq!(drive_leave_ids(&m), Ok(vec!["tmpauthz1".to_string()]));

        assert!(m.body().deserialize::<(Vec<Kept>,)>().is_err());
        let wrong = Message::method_call("/", "Example").expect("a call").build(&("x",)).expect("built");
        assert!(drive_leave_ids(&wrong).is_err());
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
