//! Why a step behind the administrator's password was refused.

use std::collections::HashMap;
use std::time::Duration;

use zbus::zvariant::Value;

use super::drive_leave::AUTHORITY;
use crate::toolkit::t;

pub const ADMIN_ACTION: &str = concat!("org.", "trindle", ".admin");

const WAIT: Duration = Duration::from_millis(1500);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    Dismissed,
    NoAsker,
    NotAuthorized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Cancelled,
    Wrong,
    NotAllowed,
    Unasked,
    Unclear,
}

pub const ALL: [Refused; 5] = [Refused::Cancelled, Refused::Wrong, Refused::NotAllowed, Refused::Unasked, Refused::Unclear];

pub fn heard_udisks(name: &str, text: &str) -> Option<Heard> {
    let l = format!("{name} {text}").to_lowercase();
    if l.contains("dismissed") || l.contains("cancelled") {
        Some(Heard::Dismissed)
    } else if l.contains("notauthorizedcanobtain") {
        Some(Heard::NoAsker)
    } else if l.contains("notauthorized") || l.contains("not authorized") {
        Some(Heard::NotAuthorized)
    } else {
        None
    }
}

pub fn heard_pkexec(text: &str) -> Option<Heard> {
    let l = text.to_lowercase();
    if l.contains("not authorized") {
        Some(Heard::NotAuthorized)
    } else if l.contains("dismissed") {
        Some(Heard::Dismissed)
    } else if l.contains("no authentication agent") {
        Some(Heard::NoAsker)
    } else {
        None
    }
}

impl Heard {
    pub fn settled(self, asks: impl FnOnce() -> Option<bool>) -> Refused {
        match self {
            Heard::Dismissed => Refused::Cancelled,
            Heard::NoAsker => Refused::Unasked,
            Heard::NotAuthorized => match asks() {
                Some(true) => Refused::Wrong,
                Some(false) => Refused::NotAllowed,
                None => Refused::Unclear,
            },
        }
    }
}

impl Refused {
    pub fn why(self) -> &'static str {
        match self {
            Refused::Cancelled => t!("common-refused-why-cancelled"),
            Refused::Wrong => t!("common-refused-why-wrong"),
            Refused::NotAllowed => t!("common-refused-why-not-allowed"),
            Refused::Unasked => t!("common-refused-why-unasked"),
            Refused::Unclear => t!("common-refused-why-unclear"),
        }
    }

    pub fn sentence(self) -> String {
        match self {
            Refused::Cancelled => t!("common-refused-sentence-cancelled"),
            Refused::Wrong => t!("common-refused-sentence-wrong"),
            Refused::NotAllowed => t!("common-refused-sentence-not-allowed"),
            Refused::Unasked => t!("common-refused-sentence-unasked"),
            Refused::Unclear => t!("common-refused-sentence-unclear"),
        }
        .to_string()
    }
}

pub fn not_done(r: Refused) -> String {
    match r {
        Refused::Cancelled => t!("common-refused-not-done-cancelled"),
        Refused::Wrong => t!("common-refused-not-done-wrong"),
        Refused::NotAllowed => t!("common-refused-not-done-not-allowed"),
        Refused::Unasked => t!("common-refused-not-done-unasked"),
        Refused::Unclear => t!("common-refused-not-done-unclear"),
    }
    .to_string()
}

pub fn of_words(words: &str) -> Option<Refused> {
    ALL.into_iter().find(|r| not_done(*r) == words)
}

fn started() -> Option<u64> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    stat.rsplit_once(") ")?.1.split(' ').nth(19)?.parse().ok()
}

fn answer_about(bus: &zbus::blocking::Connection, action: &str, pid: u32, start: u64) -> Option<(bool, bool)> {
    answer_in_full(bus, action, pid, start).map(|(allowed, would_ask, _)| (allowed, would_ask))
}

fn answer_in_full(bus: &zbus::blocking::Connection, action: &str, pid: u32, start: u64) -> Option<(bool, bool, HashMap<String, String>)> {
    let mut me: HashMap<&str, Value> = HashMap::new();
    me.insert("pid", Value::U32(pid));
    me.insert("start-time", Value::U64(start));
    let (name, path, iface) = AUTHORITY;
    let none: HashMap<&str, &str> = HashMap::new();
    let answer = bus.call_method(Some(name), path, Some(iface), "CheckAuthorization",
                                 &(("unix-process", me), action, none, 0u32, "")).ok()?;
    let (all,): ((bool, bool, HashMap<String, String>),) = answer.body().deserialize().ok()?;
    Some(all)
}

fn asks_for(bus: &zbus::blocking::Connection, action: &str) -> Option<bool> {
    let (allowed, would_ask) = answer_about(bus, action, std::process::id(), started()?)?;
    Some(allowed || would_ask)
}

fn all_ask(bus: &zbus::blocking::Connection, actions: &[&str]) -> Option<bool> {
    let mut answers = actions.iter().map(|a| asks_for(bus, a));
    let first = answers.next()??;
    answers.all(|a| a == Some(first)).then_some(first)
}

fn ask_polkit(actions: &[String]) -> Option<bool> {
    all_ask(&system()?, &actions.iter().map(String::as_str).collect::<Vec<_>>())
}

fn system() -> Option<zbus::blocking::Connection> {
    zbus::blocking::connection::Builder::system().and_then(|b| b.method_timeout(WAIT).build()).ok()
}

const ASK_WITHIN: Duration = Duration::from_millis(if cfg!(test) { 600 } else { 3000 });
const STANDS: Duration = Duration::from_secs(600);

thread_local! {
    static DRAWS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn drawing_thread() {
    DRAWS.with(|d| d.set(true));
}

static ANSWERS: std::sync::Mutex<Vec<(u64, String, std::time::Instant, Option<bool>)>> = std::sync::Mutex::new(Vec::new());

fn whose() -> u64 {
    if !cfg!(test) {
        return 0;
    }
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish() | 1
}

fn remember(whose: u64, actions: &str, answer: Option<bool>) {
    let mut all = ANSWERS.lock().unwrap_or_else(|e| e.into_inner());
    all.retain(|(w, a, at, _)| !(*w == whose && a == actions) && at.elapsed() < STANDS);
    all.push((whose, actions.to_string(), std::time::Instant::now(), answer));
}

fn remembered(whose: u64, actions: &str) -> Option<bool> {
    let all = ANSWERS.lock().unwrap_or_else(|e| e.into_inner());
    all.iter().find(|(w, a, at, _)| *w == whose && a == actions && at.elapsed() < STANDS).and_then(|(_, _, _, answer)| *answer)
}

#[derive(Clone, Copy)]
struct Asker {
    #[cfg(test)]
    answer: Option<bool>,
    #[cfg(test)]
    slow: Duration,
}

impl Asker {
    fn here() -> Asker {
        Asker {
            #[cfg(test)]
            answer: PRETEND.with(std::cell::Cell::get),
            #[cfg(test)]
            slow: SLOW.with(std::cell::Cell::get),
        }
    }

    #[cfg(not(test))]
    fn ask(self, actions: &[String]) -> Option<bool> {
        ask_polkit(actions)
    }

    #[cfg(test)]
    fn ask(self, _actions: &[String]) -> Option<bool> {
        std::thread::sleep(self.slow);
        self.answer
    }

    fn ask_within(self, actions: Vec<String>) -> Option<bool> {
        let (tell, heard) = std::sync::mpsc::channel();
        std::thread::Builder::new().name("polkit".into()).spawn(move || {
            let _ = tell.send(self.ask(&actions));
        }).ok()?;
        heard.recv_timeout(ASK_WITHIN).ok().flatten()
    }
}

pub fn would_ask(actions: &[&str]) -> Option<bool> {
    let asked = actions.join(" ");
    if DRAWS.with(std::cell::Cell::get) {
        return remembered(whose(), &asked);
    }
    let answer = Asker::here().ask_within(actions.iter().map(|a| a.to_string()).collect());
    remember(whose(), &asked, answer);
    answer
}

pub fn prime(actions: &[&str]) {
    let (asker, whose, asked) = (Asker::here(), whose(), actions.join(" "));
    let actions: Vec<String> = actions.iter().map(|a| a.to_string()).collect();
    let _ = std::thread::Builder::new().name("polkit".into()).spawn(move || {
        let answer = asker.ask_within(actions);
        remember(whose, &asked, answer);
    });
}

#[cfg(test)]
thread_local! {
    static PRETEND: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static SLOW: std::cell::Cell<Duration> = const { std::cell::Cell::new(Duration::ZERO) };
}

#[cfg(test)]
pub fn pretend(answer: Option<bool>) {
    PRETEND.with(|p| p.set(answer));
}

#[cfg(test)]
pub fn pretend_slow(long: Duration) {
    SLOW.with(|s| s.set(long));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc};

    use zbus::zvariant::OwnedValue;

    use crate::toolkit::bus::testing::{own_daemon, until};
    use crate::toolkit::bus::{Asked, Bus, Offer, Props};
    use crate::toolkit::event::Loop;

    #[test]
    fn the_three_refusals_are_told_apart_and_said_differently() {
        let udisks = "org.freedesktop.UDisks2.Error.";
        assert_eq!(heard_udisks(&format!("{udisks}NotAuthorizedDismissed"), "The authentication dialog was dismissed"), Some(Heard::Dismissed));
        assert_eq!(heard_udisks(&format!("{udisks}NotAuthorizedDismissed"), "Not authorized to perform operation"), Some(Heard::Dismissed));
        assert_eq!(heard_udisks(&format!("{udisks}NotAuthorized"), "Not authorized to perform operation"), Some(Heard::NotAuthorized));
        assert_eq!(heard_udisks(&format!("{udisks}NotAuthorizedCanObtain"), "Not authorized to perform operation"), Some(Heard::NoAsker));
        assert_eq!(heard_udisks("", "Not authorized to perform operation"), Some(Heard::NotAuthorized));
        assert_eq!(heard_udisks(&format!("{udisks}DeviceBusy"), "Error unmounting /dev/sdb1: target is busy"), None);
        assert_eq!(heard_pkexec("Error executing command as another user: Request dismissed"), Some(Heard::Dismissed));
        assert_eq!(heard_pkexec("Error executing command as another user: Not authorized\n\nThis incident has been reported."), Some(Heard::NotAuthorized));
        assert_eq!(heard_pkexec("Error executing command as another user: No authentication agent found."), Some(Heard::NoAsker));
        assert_eq!(heard_pkexec("E: Unable to fetch some archives"), None);
        let never = || -> Option<bool> { panic!("polkit is asked only about \"Not authorized\"") };
        assert_eq!(Heard::Dismissed.settled(never), Refused::Cancelled);
        assert_eq!(Heard::NoAsker.settled(never), Refused::Unasked);
        assert_eq!(Heard::NotAuthorized.settled(|| Some(true)), Refused::Wrong);
        assert_eq!(Heard::NotAuthorized.settled(|| Some(false)), Refused::NotAllowed);
        assert_eq!(Heard::NotAuthorized.settled(|| None), Refused::Unclear);
        let mut said: Vec<String> = ALL.iter().map(|r| not_done(*r)).collect();
        for (r, words) in ALL.iter().zip(&said) {
            assert_eq!(of_words(words), Some(*r));
            assert!(words.starts_with("Not done: ") && words.ends_with('.') && !words.contains('\u{2014}'), "{words}");
            assert!(r.sentence().starts_with(char::is_uppercase) && r.sentence()[1..] == r.why()[1..]);
        }
        said.sort();
        said.dedup();
        assert_eq!(said.len(), ALL.len(), "no two refusals share their words");
        assert_eq!(not_done(Refused::Cancelled), crate::common::run::NO_PASSWORD);
        assert!(Refused::Wrong.why().contains("not accepted") && !Refused::Wrong.why().contains("allowed"));
        assert!(Refused::NotAllowed.why().contains("not allowed") && !Refused::NotAllowed.why().contains("accepted"));
        assert_eq!(of_words("apt-get failed."), None);
        assert_eq!(would_ask(&[ADMIN_ACTION]), None);
        pretend(Some(true));
        assert_eq!(would_ask(&[ADMIN_ACTION]), Some(true));
        pretend(None);
    }

    #[test]
    fn polkit_is_never_waited_for_on_a_thread_that_draws() {
        use std::time::Instant;
        std::thread::spawn(|| {
            drawing_thread();
            pretend(Some(true));
            pretend_slow(Duration::from_millis(300));
            let (a, b, c) = (["org.example.one"], ["org.example.two"], ["org.example.three"]);
            let began = Instant::now();
            assert_eq!(would_ask(&a), None, "nothing is known yet, and nothing is asked here");
            assert!(began.elapsed() < Duration::from_millis(150), "{:?}", began.elapsed());
            prime(&a);
            assert_eq!(would_ask(&a), None);
            assert!(began.elapsed() < Duration::from_millis(250), "{:?}", began.elapsed());
            std::thread::sleep(Duration::from_millis(900));
            let again = Instant::now();
            assert_eq!(would_ask(&a), Some(true));
            assert_eq!(would_ask(&b), None, "another question has no answer yet");
            assert!(again.elapsed() < Duration::from_millis(150), "{:?}", again.elapsed());
            pretend_slow(ASK_WITHIN + Duration::from_secs(3));
            prime(&c);
            std::thread::sleep(ASK_WITHIN + Duration::from_millis(500));
            assert_eq!(would_ask(&c), None);
        }).join().expect("the checks held");
        std::thread::spawn(|| {
            pretend(Some(false));
            assert_eq!(would_ask(&["org.example.four"]), Some(false));
            pretend_slow(ASK_WITHIN + Duration::from_secs(3));
            let began = Instant::now();
            assert_eq!(would_ask(&["org.example.five"]), None);
            assert!(began.elapsed() < ASK_WITHIN + Duration::from_secs(2), "{:?}", began.elapsed());
        }).join().expect("the checks held");
    }

    fn stand_in_for_polkit(address: String, stopped: Arc<AtomicBool>, ready: mpsc::Sender<bool>, asked: mpsc::Sender<String>) {
        let lp = Loop::new();
        let bus = Bus::at(&lp, &address).expect("the fake's bus");
        let named = std::rc::Rc::new(std::cell::Cell::new(false));
        let n = named.clone();
        bus.own(AUTHORITY.0, false, &lp, move |ok| n.set(ok));
        let n = named.clone();
        let owned = until(&lp, move || n.get());
        let _served = bus.serve(AUTHORITY.1, Offer {
            name: AUTHORITY.2.into(),
            xml: format!("<interface name=\"{}\"/>", AUTHORITY.2),
            call: Box::new(move |a: Asked| {
                type Check = ((String, HashMap<String, OwnedValue>), String, HashMap<String, String>, u32, String);
                let Ok(((kind, who), action, _, flags, _)) = a.message().body().deserialize::<Check>() else {
                    return a.fail("org.freedesktop.DBus.Error.InvalidArgs", "not a question this fake knows");
                };
                let pid = who.get("pid").and_then(|v| u32::try_from(v).ok()).unwrap_or(0);
                let start = who.get("start-time").and_then(|v| u64::try_from(v).ok()).unwrap_or(0);
                let _ = asked.send(format!("{} {kind} {} {} flags {flags}", a.member(), pid == std::process::id(), start > 0));
                let found = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()
                    .and_then(|stat| stat.rsplit_once(") ")?.1.split(' ').nth(19)?.parse::<u64>().ok());
                if kind != "unix-process" || found != Some(start) {
                    return a.fail("org.freedesktop.PolicyKit1.Error.Failed", "Unix process subject does not have uid set");
                }
                let none: HashMap<String, String> = HashMap::new();
                let kept: HashMap<String, String> = HashMap::from([("polkit.retains_authorization_after_challenge".to_string(), "1".to_string())]);
                let still: HashMap<String, String> = HashMap::from([(KEPT_LEAVE.to_string(), "tmpauthz7".to_string())]);
                match action.as_str() {
                    "org.example.still-kept" => a.reply(&((true, false, still),)),
                    "org.example.asks-and-keeps" => a.reply(&((false, true, kept),)),
                    "org.example.asks" => a.reply(&((false, true, none),)),
                    "org.example.refuses" => a.reply(&((false, false, none),)),
                    "org.example.allowed" => a.reply(&((true, false, none),)),
                    other => a.fail("org.freedesktop.PolicyKit1.Error.Failed", &format!("Action {other} is not registered")),
                }
            }),
            props: Box::new(Props::new),
            set: None,
        });
        let _ = ready.send(owned);
        until(&lp, move || stopped.load(Ordering::Relaxed));
    }

    fn check_the_answers(bus: &zbus::blocking::Connection) {
        assert_eq!(asks_for(bus, "org.example.asks"), Some(true));
        assert_eq!(asks_for(bus, "org.example.refuses"), Some(false));
        assert_eq!(asks_for(bus, "org.example.allowed"), Some(true));
        assert_eq!(asks_for(bus, "org.example.asks-and-keeps"), Some(true));
        assert_eq!(asks_for(bus, "org.example.unknown"), None);
        let (me, start) = (std::process::id(), started().expect("this program's start"));
        assert_eq!(answer_about(bus, "org.example.asks", me, start), Some((false, true)));
        assert_eq!(answer_about(bus, "org.example.refuses", me, start), Some((false, false)));
        assert_eq!(answer_about(bus, "org.example.allowed", me, start), Some((true, false)));
        let asked = |action: &str| answer_in_full(bus, action, me, start);
        assert!(asked("org.example.still-kept").is_some_and(|(allowed, _, said)| allowed && said.contains_key(KEPT_LEAVE)));
        assert_eq!(needs_a_password("org.example.asks", asked("org.example.asks")), Ok(()));
        assert_eq!(needs_a_password("org.example.asks-and-keeps", asked("org.example.asks-and-keeps")), Ok(()));
        for (action, why) in [("org.example.allowed", "with no password at all"), ("org.example.still-kept", "still keeping a password"),
                              ("org.example.refuses", "refuses it outright"), ("org.example.unknown", "gave no answer")] {
            let said = needs_a_password(action, asked(action)).expect_err(action);
            assert!(said.starts_with(&format!("{action}: ")) && said.contains(why), "{said}");
        }
        assert_eq!(answer_about(bus, "org.example.asks", me, start + 1), None);
        assert_eq!(answer_about(bus, "org.example.refuses", me, start + 1), None);
        assert_eq!(all_ask(bus, &["org.example.asks", "org.example.allowed"]), Some(true));
        assert_eq!(all_ask(bus, &["org.example.refuses", "org.example.refuses"]), Some(false));
        assert_eq!(all_ask(bus, &["org.example.asks", "org.example.refuses"]), None);
        assert_eq!(all_ask(bus, &["org.example.refuses", "org.example.unknown"]), None);
        assert_eq!(all_ask(bus, &[]), None);
    }

    #[test]
    fn polkit_is_asked_whether_it_would_ask_and_never_to_show_a_box() {
        let d = own_daemon();
        assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
        assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()));
        let (address, stop) = (d.1.clone(), Arc::new(AtomicBool::new(false)));
        let (ready, is_ready) = mpsc::channel::<bool>();
        let (asked, was_asked) = mpsc::channel::<String>();
        let stopped = stop.clone();
        let fake = std::thread::spawn(move || stand_in_for_polkit(address, stopped, ready, asked));
        assert_eq!(is_ready.recv_timeout(Duration::from_secs(10)), Ok(true), "the fake owns polkit's name");
        let bus = zbus::blocking::connection::Builder::address(d.1.as_str()).and_then(|b| b.method_timeout(WAIT).build()).expect("the asker's bus");
        check_the_answers(&bus);
        stop.store(true, Ordering::Relaxed);
        fake.join().expect("the fake ended");
        let questions: Vec<String> = was_asked.try_iter().collect();
        assert!(questions.len() >= 4, "{questions:?}");
        for q in &questions {
            assert_eq!(q, "CheckAuthorization unix-process true true flags 0");
        }
    }

    fn listed_actions() -> Vec<(String, [String; 3])> {
        let listed = std::process::Command::new("pkaction").arg("--verbose").output()
            .expect("pkaction could not be run, and this test checks nothing without polkit: install polkit (polkitd and pkexec)");
        assert!(listed.status.success(), "pkaction failed: {}", String::from_utf8_lossy(&listed.stderr));
        read_listed(&String::from_utf8_lossy(&listed.stdout))
    }

    fn read_listed(words: &str) -> Vec<(String, [String; 3])> {
        let (mut all, mut name) = (Vec::new(), String::new());
        let mut three = [String::new(), String::new(), String::new()];
        for line in words.lines() {
            let said = |what: &str| line.trim().strip_prefix(what).map(|rest| rest.trim().to_string());
            if !line.starts_with(' ') && line.ends_with(':') {
                name = line.trim_end_matches(':').to_string();
            } else if let Some(any) = said("implicit any:") {
                three[0] = any;
            } else if let Some(inactive) = said("implicit inactive:") {
                three[1] = inactive;
            } else if let Some(active) = said("implicit active:") {
                three[2] = active;
                all.push((name.clone(), three.clone()));
            }
        }
        all
    }

    const SHIPPED: &str = include_str!(concat!("../../system/polkit/org.", "trindle", ".admin.policy"));

    fn must_be(word: &str) -> Option<bool> {
        match word {
            "no" => Some(false),
            "yes" | "auth_self" | "auth_self_keep" | "auth_admin" | "auth_admin_keep" => Some(true),
            _ => None,
        }
    }

    fn policy_says(xml: &str, id: &str) -> Option<[String; 3]> {
        let action = xml.split("<action id=\"").find_map(|part| part.strip_prefix(id)?.strip_prefix('"'))?;
        let action = action.split("</action>").next()?;
        let of = |tag: &str| Some(action.split_once(&format!("<{tag}>"))?.1.split_once(&format!("</{tag}>"))?.0.trim().to_string());
        Some([of("allow_any")?, of("allow_inactive")?, of("allow_active")?])
    }

    const KEPT_LEAVE: &str = "polkit.temporary_authorization_id";

    type Full = (bool, bool, HashMap<String, String>);

    fn needs_a_password(action: &str, got: Option<Full>) -> Result<(), String> {
        match got {
            Some((false, true, _)) => Ok(()),
            None => Err(format!("{action}: polkit gave no answer about it. It is not registered on this computer, or polkit could not be asked.")),
            Some((true, _, said)) if said.contains_key(KEPT_LEAVE) => Err(format!(
                "{action}: polkit is still keeping a password that was given a few minutes ago, so nothing was proved. \
                 Wait until it has let that go, five minutes at the most, and run this again.")),
            Some((true, _, _)) => Err(format!(
                "{action}: polkit allows it with no password at all. Trindle's rule file (system/polkit/60-trindle.rules) is not installed, \
                 or no longer answers for this action.")),
            Some((false, false, _)) => Err(format!(
                "{action}: polkit refuses it outright, whatever password is given. A rule refuses it, or this program is not in the login in use at the computer.")),
        }
    }

    fn own_actions() -> Vec<String> {
        use crate::common::drive_leave::{CHANGING, UDISKS};
        std::iter::once(ADMIN_ACTION.to_string()).chain(CHANGING.iter().map(|a| format!("{UDISKS}{a}"))).collect()
    }

    fn own_actions_need_a_password(ask: impl Fn(&str) -> Option<Full>) -> Result<Vec<String>, String> {
        let all = own_actions();
        for action in &all {
            needs_a_password(action, ask(action))?;
        }
        Ok(all)
    }

    fn in_use_at_the_computer(said: &str) -> Result<(), String> {
        for want in ["Active=yes", "Remote=no", "Class=user"] {
            if !said.lines().any(|l| l.trim() == want) {
                return Err(format!("This login is not the one in use at the computer (wanted {want}; the login service says: {}).",
                                   said.split_whitespace().collect::<Vec<_>>().join(" ")));
            }
        }
        Ok(())
    }

    #[test]
    fn the_release_check_fails_unless_every_action_of_trindles_needs_a_password() {
        use crate::common::drive_leave::{CHANGING, DRIVES_ASKS, UDISKS};
        let none = HashMap::<String, String>::new;
        let asks = || Some((false, true, none()));
        let all = own_actions();
        assert_eq!(all.len(), 1 + CHANGING.len());
        assert!(all.contains(&ADMIN_ACTION.to_string()));
        for own in DRIVES_ASKS {
            assert!(all.contains(&format!("{UDISKS}{own}")), "{own}");
        }
        assert_eq!(own_actions_need_a_password(|_| asks()), Ok(all.clone()));
        let kept = || HashMap::from([(KEPT_LEAVE.to_string(), "tmpauthz1".to_string())]);
        let wrong: [(Option<Full>, &str); 5] = [
            (Some((true, false, none())), "with no password at all"),
            (Some((true, true, none())), "with no password at all"),
            (Some((true, false, kept())), "still keeping a password"),
            (Some((false, false, none())), "refuses it outright"),
            (None, "gave no answer"),
        ];
        for action in &all {
            for (answer, why) in &wrong {
                let got = own_actions_need_a_password(|a| if a == action { answer.clone() } else { asks() });
                let said = got.expect_err("an action that did not need a password passed the check");
                assert!(said.starts_with(&format!("{action}: ")) && said.contains(why), "{said}");
                assert!(!said.contains('\u{2014}'), "{said}");
            }
        }
        assert!(needs_a_password("org.example.a", wrong[0].0.clone()).unwrap_err().contains("system/polkit/60-trindle.rules"));
        assert_eq!(in_use_at_the_computer("Active=yes\nRemote=no\nClass=user\n"), Ok(()));
        assert_eq!(in_use_at_the_computer("Class=user\nRemote=no\nActive=yes"), Ok(()));
        for (said, want) in [("Active=no\nRemote=no\nClass=user\n", "Active=yes"), ("Active=yes\nRemote=yes\nClass=user\n", "Remote=no"),
                             ("Active=yes\nRemote=no\nClass=greeter\n", "Class=user"), ("Active=yes\nRemote=no\nClass=user-early\n", "Class=user"),
                             ("", "Active=yes"), ("Active=yes Remote=no Class=user", "Active=yes")] {
            let why = in_use_at_the_computer(said).expect_err(said);
            assert!(why.contains(&format!("wanted {want};")), "{why}");
        }
    }

    #[test]
    fn the_release_check_reads_polkits_list_and_trindles_policy_as_they_are_written() {
        let listed = "org.example.one:\n  description:       One\n  implicit any:      no\n  implicit inactive: no\n  implicit active:   auth_admin_keep\n\n\
                      org.example.two:\n  implicit any:      auth_admin\n  implicit inactive: auth_admin\n  implicit active:   yes\n";
        let no = |s: &str| s.to_string();
        assert_eq!(read_listed(listed), vec![(no("org.example.one"), [no("no"), no("no"), no("auth_admin_keep")]),
                                              (no("org.example.two"), [no("auth_admin"), no("auth_admin"), no("yes")])]);
        assert_eq!((must_be("no"), must_be("yes"), must_be("auth_admin"), must_be("auth_admin_keep"), must_be("porridge")),
                   (Some(false), Some(true), Some(true), Some(true), None));
        let shipped = policy_says(SHIPPED, ADMIN_ACTION).expect("the action is in the policy");
        assert_eq!(shipped, [no("no"), no("no"), no("auth_admin_keep")]);
        assert_eq!(policy_says("<action id=\"org.example.other\"></action>", ADMIN_ACTION), None);
    }

    #[test]
    #[ignore = "asks the real polkit on the system bus: run by hand"]
    fn the_real_polkit_answers_as_the_stand_in_does() {
        // SAFETY: getuid only reads this program's own user number.
        assert_ne!(unsafe { libc::getuid() }, 0, "root is allowed everything, so nothing is told apart: run this as an ordinary login");
        let actions = listed_actions();
        assert!(!actions.is_empty(), "polkit lists no actions at all");
        let bus = system().expect("the system bus could not be reached, and this test checks nothing without the real polkit");
        let (me, start) = (std::process::id(), started().expect("this program's start"));
        let password = |a: &&(String, [String; 3])| a.1.iter().all(|w| w == "auth_admin" || w == "auth_admin_keep");
        let asks = actions.iter().find(|a| a.0 == "org.freedesktop.policykit.exec" && password(a)).or_else(|| actions.iter().find(password))
            .expect("polkit lists no action that needs an administrator's password, so this was not checked");
        let refuses = actions.iter().find(|a| a.1.iter().all(|w| w == "no"))
            .expect("polkit lists no action it refuses outright, so half of this was not checked");
        assert_eq!(answer_about(&bus, &asks.0, me, start), Some((false, true)), "{}: an administrator's password would do", asks.0);
        assert_eq!(asks_for(&bus, &asks.0), Some(true), "{}", asks.0);
        assert_eq!(answer_about(&bus, &refuses.0, me, start), Some((false, false)), "{}: refused outright", refuses.0);
        assert_eq!(asks_for(&bus, &refuses.0), Some(false), "{}", refuses.0);
        assert_eq!(all_ask(&bus, &[asks.0.as_str(), refuses.0.as_str()]), None, "they do not agree");
        assert_eq!(asks_for(&bus, "org.example.trindle-test.not-registered"), None);
        assert_eq!(answer_about(&bus, &asks.0, me, start + 1), None);
        let login = std::process::Command::new("loginctl").args(["show-session", "self", "-p", "Active", "-p", "Remote", "-p", "Class"]).output()
            .expect("loginctl could not be run, so whether this login is the one in use could not be found out and nothing of Trindle's own was checked");
        assert!(login.status.success(), "this was not started from inside a login (the login service knows no session for it), so polkit would answer it \
                 as it answers a stranger and nothing of Trindle's own was checked: run tools/real-polkit-check.sh from a terminal at the computer itself");
        if let Err(why) = in_use_at_the_computer(&String::from_utf8_lossy(&login.stdout)) {
            panic!("{why} polkit would answer it differently, so nothing of Trindle's own was checked: run tools/real-polkit-check.sh from a terminal at the computer itself");
        }
        let listed = |action: &str| actions.iter().find(|a| a.0 == action).map(|a| a.1.clone()).unwrap_or_else(|| {
            panic!("{action} is not registered with polkit on this computer, so it was not checked: run this where Trindle and UDisks are installed")
        });
        let shipped = policy_says(SHIPPED, ADMIN_ACTION).expect("the action is in the policy");
        assert_eq!(listed(ADMIN_ACTION), shipped, "{ADMIN_ACTION}: the policy polkit has is not the one that goes out with this build");
        assert_eq!(must_be(&shipped[2]), Some(true), "{ADMIN_ACTION}: an administrator's password would do, at the computer");
        for action in own_actions() {
            listed(&action);
        }
        let asked = match own_actions_need_a_password(|action| answer_in_full(&bus, action, me, start)) {
            Ok(asked) => asked,
            Err(why) => panic!("{why}"),
        };
        for action in &asked {
            assert_eq!(ask_polkit(std::slice::from_ref(action)), Some(true), "{action}");
        }
        let drives: Vec<String> = crate::common::drive_leave::DRIVES_ASKS.iter().map(|a| format!("{}{a}", crate::common::drive_leave::UDISKS)).collect();
        assert_eq!(ask_polkit(&drives), Some(true), "the drives' actions asked together, as Drives asks them");
        println!("Asked the real polkit, with flags 0, about {}: each is not allowed already and would be with an administrator's password.", asked.join(", "));
    }
}
