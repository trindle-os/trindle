//! Not a back door.

use std::ffi::CStr;

use crate::toolkit::t;

pub const QUESTION: &CStr = c"Trindle Pocketkey: use your phone? (yes/no)";

pub const PREFIX: &str = "Trindle Pocketkey:";

#[cfg(not(test))]
const SOCKET: &str = "/run/trindle-pocketkey/user.sock";

fn socket() -> std::path::PathBuf {
    #[cfg(test)]
    return test_socket::AT.with(|a| a.borrow().clone()).unwrap_or_else(|| "/nonexistent/trindle-pocketkey.sock".into());
    #[cfg(not(test))]
    SOCKET.into()
}

#[cfg(test)]
mod test_socket {
    thread_local! {
        pub static AT: std::cell::RefCell<Option<std::path::PathBuf>> = const { std::cell::RefCell::new(None) };
    }
}

pub const REAUTH: [u32; 5] = [0, 1800, 3600, 21600, 86400];

const OLD_REAUTH: [u32; 3] = [14400, 28800, 604800];

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize)]
pub struct Status {
    pub paired: bool,
    pub phone_name: String,
    pub paired_at: u64,
    pub lock_delay_secs: u32,
    pub near: bool,
    #[serde(default)]
    pub on: bool,
    #[serde(default)]
    pub reauth_secs: u32,
    #[serde(default)]
    pub password_due: bool,
}

pub fn ask(line: &str) -> Option<String> {
    let at = socket();
    #[cfg(not(test))]
    {
        use std::os::unix::fs::MetadataExt;
        if std::fs::symlink_metadata(&at).ok()?.uid() != 0 {
            return None;
        }
    }
    crate::plugins::rows::ask_at(&at, line)
}

pub fn status() -> Option<Status> {
    serde_json::from_str(&ask("status\n")?).ok()
}

pub fn on() -> bool {
    status().is_some_and(|s| s.on)
}

pub fn on_for(name: &str) -> Option<bool> {
    let uid = super::files::uid_of(name.trim())?;
    ask(&format!("on? {uid}\n")).map(|a| a == "yes")
}

pub fn reauth_words(secs: u32) -> String {
    const HOUR: u32 = 3600;
    const DAY: u32 = 24 * HOUR;
    const WEEK: u32 = 7 * DAY;
    match secs {
        0 => t!("common-pocketkey-never").to_string(),
        s if s % WEEK == 0 => t!("common-pocketkey-weeks", n = s / WEEK),
        s if s % DAY == 0 => t!("common-pocketkey-days", n = s / DAY),
        s if s % HOUR == 0 => t!("common-pocketkey-hours", n = s / HOUR),
        s => t!("common-pocketkey-minutes", n = s / 60),
    }
}

pub fn due_words(secs: u32) -> String {
    t!("common-pocketkey-due", time = reauth_words(secs))
}

pub fn parse_due(answer: &str) -> Option<u32> {
    let secs: u32 = answer.strip_prefix("yes ")?.parse().ok()?;
    (REAUTH.contains(&secs) || OLD_REAUTH.contains(&secs)).then_some(secs).filter(|s| *s != 0)
}

pub fn due_for(name: &str) -> Option<u32> {
    super::files::uid_of(name.trim()).and_then(due_uid)
}

fn due_uid(uid: u32) -> Option<u32> {
    parse_due(&ask(&format!("due? {uid}\n"))?)
}

pub fn screen() -> (bool, Option<u32>) {
    match status() {
        Some(s) if s.on => gate(true, (s.paired && s.password_due && s.reauth_secs != 0).then_some(s.reauth_secs)),
        _ => (false, None),
    }
}

pub fn gate(on: bool, due: Option<u32>) -> (bool, Option<u32>) {
    (on && due.is_none(), due.filter(|_| on))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_daemons_status() {
        let s: Status = serde_json::from_str(r#"{"paired":true,"phone_name":"Pixel 8","paired_at":5,"lock_delay_secs":30,"near":false}"#).unwrap();
        assert!(s.paired && s.phone_name == "Pixel 8" && s.lock_delay_secs == 30);
        assert!(!s.on, "off unless the daemon says on");
        let s: Status = serde_json::from_str(r#"{"paired":false,"phone_name":"","paired_at":0,"lock_delay_secs":30,"near":false,"on":true}"#).unwrap();
        assert!(s.on);
        assert!(!s.password_due && s.reauth_secs == 0, "never, and not due, unless the daemon says");
        let s: Status = serde_json::from_str(r#"{"paired":true,"phone_name":"P","paired_at":5,"lock_delay_secs":30,"near":true,"on":true,"reauth_secs":3600,"password_due":true}"#).unwrap();
        assert!(s.password_due && s.reauth_secs == 3600);
        assert!(!on() && on_for("root").is_none());
    }

    #[test]
    fn every_choice_has_its_words_and_its_label() {
        let words = ["Never", "30 minutes", "1 hour", "6 hours", "1 day"];
        for (secs, said) in REAUTH.iter().zip(words) {
            assert_eq!(reauth_words(*secs), said);
        }
        assert_eq!(REAUTH[0], 0, "Never is first");
        assert_eq!(due_words(3600), "Password needed again: it has been more than 1 hour.");
        assert_eq!(due_words(1800), "Password needed again: it has been more than 30 minutes.");
        assert_eq!(due_words(21600), "Password needed again: it has been more than 6 hours.");
        assert_eq!(due_words(604800), "Password needed again: it has been more than 1 week.");
        assert_eq!(parse_due("yes 14400"), Some(14400), "an older stored choice is still read");
    }

    #[test]
    fn the_due_answer_is_read_for_every_choice_and_nothing_else() {
        assert_eq!(parse_due("no"), None);
        for secs in REAUTH.into_iter().filter(|s| *s != 0) {
            assert_eq!(parse_due(&format!("yes {secs}")), Some(secs));
        }
        for bad in ["yes 0", "yes 7", "yes", "yes -1", "yes 3600 x", "yes  3600", "YES 3600", "", "maybe 3600", "yes 99999999999"] {
            assert_eq!(parse_due(bad), None, "{bad}");
        }
    }

    #[test]
    fn the_phone_shows_only_while_on_and_not_due() {
        assert_eq!(gate(true, None), (true, None));
        assert_eq!(gate(true, Some(3600)), (false, Some(3600)));
        assert_eq!(gate(false, Some(3600)), (false, None), "off says nothing");
        assert_eq!(gate(false, None), (false, None));
    }

    fn daemon(replies: &'static [&'static str]) -> std::thread::JoinHandle<Vec<String>> {
        use std::io::{BufRead, BufReader, Write};
        let path = std::env::temp_dir().join(format!("trindle-pk-{}-{:?}.sock", std::process::id(), std::thread::current().id()));
        let _ = std::fs::remove_file(&path);
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        test_socket::AT.with(|a| *a.borrow_mut() = Some(path.clone()));
        std::thread::spawn(move || {
            let mut heard = Vec::new();
            for reply in replies {
                let (mut s, _) = listener.accept().unwrap();
                let mut line = String::new();
                BufReader::new(&s).read_line(&mut line).unwrap();
                heard.push(line);
                s.write_all(reply.as_bytes()).unwrap();
            }
            let _ = std::fs::remove_file(&path);
            heard
        })
    }

    #[test]
    fn the_screen_follows_the_daemons_status_in_one_question() {
        let on = |extra: &str| format!("{{\"paired\":true,\"phone_name\":\"P\",\"paired_at\":5,\"lock_delay_secs\":30,\"near\":true,\"on\":true{extra}}}\n");
        for (reply, want) in [
            (on(""), (true, None)),
            (on(",\"reauth_secs\":3600,\"password_due\":false"), (true, None)),
            (on(",\"reauth_secs\":3600,\"password_due\":true"), (false, Some(3600))),
            (on(",\"reauth_secs\":0,\"password_due\":true"), (true, None)),
            ("{\"paired\":true,\"phone_name\":\"P\",\"paired_at\":5,\"lock_delay_secs\":30,\"near\":true,\"on\":false,\"reauth_secs\":3600,\"password_due\":true}\n".to_string(), (false, None)),
            ("junk\n".to_string(), (false, None)),
        ] {
            let reply: &'static str = Box::leak(reply.into_boxed_str());
            let d = daemon(std::slice::from_ref(Box::leak(Box::new(reply))));
            assert_eq!(screen(), want, "{reply}");
            assert_eq!(d.join().unwrap(), ["status\n"], "one question only");
        }
    }

    #[test]
    fn off_shows_nothing_and_tells_the_phone_nothing() {
        const OFF: &str = "{\"paired\":true,\"phone_name\":\"P\",\"paired_at\":5,\"lock_delay_secs\":30,\"near\":true,\"on\":false}\n";
        let d = daemon(&[OFF, OFF]);
        assert!(!on());
        assert_eq!(screen(), (false, None));
        assert_eq!(d.join().unwrap(), ["status\n", "status\n"]);
        let polkit = include_str!("../polkit/mod.rs");
        assert!(polkit.contains("note: (pocketkey::on() &&"), "polkit sends the daemon a note only while it is on");
        assert!(polkit.contains("let _ = pocketkey::ask(note)"));
    }

    #[test]
    fn the_login_screen_asks_the_second_question_only_when_the_first_was_answered() {
        assert_eq!(on_for("root"), None);
        let d = daemon(&["no\n", "yes 3600\n"]);
        assert_eq!(on_for("root"), Some(false));
        assert_eq!(due_for("root"), Some(3600));
        assert_eq!(d.join().unwrap(), ["on? 0\n", "due? 0\n"]);
    }
}
