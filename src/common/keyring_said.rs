//! The one rule for asking the person's keyring, the Secret Service, for
//! anything. Apps keeps its GitHub key by it (store/key.rs) and Folders
//! its logins for shares (files/shares.rs).

use std::time::Duration;

use crate::toolkit::t;

pub fn locked_words() -> &'static str {
    t!("common-keyring-locked")
}

pub fn missing_words() -> &'static str {
    t!("common-keyring-missing")
}

pub fn silent_words() -> &'static str {
    t!("common-keyring-silent")
}

pub fn never_words() -> &'static str {
    t!("common-keyring-never")
}

pub const WAIT: Duration = Duration::from_secs(3);

pub const DEADLINE: Duration = Duration::from_secs(if cfg!(test) { 2 } else { 8 });

const LATE_MOST: Duration = Duration::from_secs(if cfg!(test) { 30 } else { 600 });

pub struct Waited<T> {
    pub now: Result<T, String>,
    pub late: Option<Late<T>>,
}

pub struct Late<T>(std::sync::mpsc::Receiver<Result<T, String>>);

impl<T> Late<T> {

    pub fn wait(self) -> Result<T, String> {
        self.0.recv_timeout(LATE_MOST).unwrap_or_else(|_| Err(never_words().into()))
    }

    #[cfg(test)]
    pub fn come(how: Result<T, String>) -> Late<T> {
        let (done, answer) = std::sync::mpsc::channel();
        let _ = done.send(how);
        Late(answer)
    }
}

#[derive(Clone)]
struct Turn {
    here: std::sync::Arc<std::sync::Mutex<()>>,
    shared: Option<std::path::PathBuf>,
}

#[cfg(not(test))]
const TURN_FILE: &str = "trindle-keyring.turn";

#[cfg(not(test))]
fn turn() -> Turn {
    static HERE: std::sync::OnceLock<std::sync::Arc<std::sync::Mutex<()>>> = std::sync::OnceLock::new();
    Turn { here: HERE.get_or_init(Default::default).clone(), shared: Some(crate::common::files::scratch().join(TURN_FILE)) }
}

#[cfg(test)]
thread_local! {

    static KEYRING: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };

    static SHARED: std::cell::RefCell<Option<std::path::PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub fn pretend(keyring: Option<String>) {
    KEYRING.with(|k| *k.borrow_mut() = keyring);
}

#[cfg(test)]
pub fn pretend_shared(file: Option<std::path::PathBuf>) {
    SHARED.with(|s| *s.borrow_mut() = file);
}

#[cfg(test)]
fn turn() -> Turn {
    static TURNS: std::sync::Mutex<Vec<(String, std::sync::Arc<std::sync::Mutex<()>>)>> = std::sync::Mutex::new(Vec::new());
    let name = KEYRING.with(|k| k.borrow().clone()).unwrap_or_else(|| format!("thread {:?}", std::thread::current().id()));
    let shared = SHARED.with(|s| s.borrow().clone());
    let mut turns = TURNS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, here)) = turns.iter().find(|(n, _)| *n == name) {
        return Turn { here: here.clone(), shared };
    }
    turns.push((name, Default::default()));
    Turn { here: turns[turns.len() - 1].1.clone(), shared }
}

fn taken(file: &std::path::Path) -> Result<std::fs::File, String> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    let why = |e: std::io::Error| t!("common-keyring-turn-failed", why = e.to_string());
    if let Some(folder) = file.parent() {
        let _ = std::fs::create_dir_all(folder);
    }
    let held = std::fs::OpenOptions::new().write(true).create(true).truncate(false).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC).open(file).map_err(why)?;
    loop {
        // SAFETY: the descriptor is that of `held`, which is open throughout the call.
        if unsafe { libc::flock(held.as_raw_fd(), libc::LOCK_EX) } == 0 {
            return Ok(held);
        }
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::Interrupted {
            return Err(why(e));
        }
    }
}

pub fn within_late<T: Send + 'static>(work: impl FnOnce() -> Result<T, String> + Send + 'static) -> Waited<T> {
    let turn = turn();
    let (done, answer) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _mine = turn.here.lock().unwrap_or_else(|e| e.into_inner());
        let answer = match turn.shared.as_deref().map(taken) {
            Some(Err(why)) => Err(why),

            Some(Ok(_among_programs)) => work(),
            None => work(),
        };

        let _ = done.send(answer);
    });
    match answer.recv_timeout(DEADLINE) {
        Ok(now) => Waited { now, late: None },
        Err(_) => Waited { now: Err(silent_words().into()), late: Some(Late(answer)) },
    }
}

pub fn within<T: Send + 'static>(work: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    within_late(work).now
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Said {
    pub words: String,
    pub failed: bool,
}

fn sentence(words: &str) -> String {
    let words = words.trim();
    if words.ends_with(['.', '!', '?']) { words.to_string() } else { format!("{words}.") }
}

pub fn waiting(what: &str, removing: bool) -> Said {
    Said { words: t!("common-keyring-waiting", what = what, action = if removing { "removing" } else { "keeping" }), failed: false }
}

pub fn late(what: &str, removing: bool, how: &Result<(), String>) -> Said {
    let action = if removing { "removing" } else { "keeping" };
    match how {
        Ok(()) => Said { words: t!("common-keyring-late-done", what = what, action = action), failed: false },
        Err(why) if why == never_words() => Said { words: t!("common-keyring-late-never", what = what, action = action), failed: true },
        Err(why) => Said { words: t!("common-keyring-late-refused", what = what, action = action, why = sentence(why)), failed: true },
    }
}

pub fn why(e: zbus::Error) -> String {
    match e {

        zbus::Error::MethodError(name, _, _)
            if name.as_str().ends_with("ServiceUnknown") || name.as_str().ends_with("NameHasNoOwner") || name.as_str().contains(".Spawn.") => {
            missing_words().to_string()
        }
        zbus::Error::MethodError(name, _, _) if name.as_str().ends_with(".IsLocked") => locked_words().to_string(),
        zbus::Error::MethodError(name, _, _) if name.as_str().ends_with(".NoReply") || name.as_str().ends_with(".Timeout") || name.as_str().ends_with(".TimedOut") => {
            silent_words().to_string()
        }
        zbus::Error::InputOutput(io) if matches!(io.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) => silent_words().to_string(),
        e => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_that_never_ends_is_said_as_a_keyring_that_did_not_answer_and_nobody_is_held() {

        pretend(Some("test:never-ends".into()));
        let since = std::time::Instant::now();
        let got: Result<(), String> = within(|| {
            std::thread::sleep(DEADLINE * 20);
            Ok(())
        });
        assert_eq!(got, Err(silent_words().to_string()));
        assert!(since.elapsed() < DEADLINE * 2, "the one who asked was held for {:?}", since.elapsed());

        let since = std::time::Instant::now();
        assert_eq!(within(|| Ok(7)), Err(silent_words().to_string()));
        assert!(since.elapsed() < DEADLINE * 2);

        pretend(Some("test:answers".into()));
        assert_eq!(within(|| Ok(7)), Ok(7));
        assert_eq!(within(|| Err::<(), _>(locked_words().to_string())), Err(locked_words().to_string()));
    }

    #[test]
    fn work_with_the_keyring_is_done_one_piece_at_a_time_whoever_asks() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static INSIDE: AtomicUsize = AtomicUsize::new(0);
        static MOST: AtomicUsize = AtomicUsize::new(0);
        let threads: Vec<_> = (0..3).map(|_| std::thread::spawn(|| {

            pretend(Some("test:one-rule".into()));
            within(|| {
                let now = INSIDE.fetch_add(1, Ordering::SeqCst) + 1;
                MOST.fetch_max(now, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(250));
                INSIDE.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            })
        })).collect();
        for t in threads {
            assert_eq!(t.join().unwrap(), Ok(()));
        }
        assert_eq!(MOST.load(Ordering::SeqCst), 1, "two pieces of work were with the keyring at once");
    }

    fn turn_file(test: &str) -> std::path::PathBuf {
        let folder = std::env::temp_dir().join(format!("trindle-keyring-turn-{}-{test}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        folder.join("turn")
    }

    #[test]
    fn two_programs_of_one_person_take_turns_with_the_keyring() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static INSIDE: AtomicUsize = AtomicUsize::new(0);
        static MOST: AtomicUsize = AtomicUsize::new(0);
        let file = turn_file("two-programs");
        let programs: Vec<_> = ["test:window-one", "test:window-two", "test:apps"].into_iter().map(|name| {
            let file = file.clone();
            std::thread::spawn(move || {
                pretend(Some(name.into()));
                pretend_shared(Some(file));
                within(|| {
                    let now = INSIDE.fetch_add(1, Ordering::SeqCst) + 1;
                    MOST.fetch_max(now, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(250));
                    INSIDE.fetch_sub(1, Ordering::SeqCst);
                    Ok(())
                })
            })
        }).collect();
        for p in programs {
            assert_eq!(p.join().unwrap(), Ok(()));
        }
        assert_eq!(MOST.load(Ordering::SeqCst), 1, "two programs were with the keyring at once");
        let _ = std::fs::remove_dir_all(file.parent().unwrap());
    }

    #[test]
    fn a_turn_another_program_holds_is_waited_for() {
        use std::os::fd::AsRawFd;
        use std::process::{Command, Stdio};
        let file = turn_file("another-program");
        pretend(Some("test:beside-another-program".into()));
        pretend_shared(Some(file.clone()));

        let held = |file: &std::path::Path| {
            let Ok(look) = std::fs::File::open(file) else { return false };
            // SAFETY: the descriptor is that of `look`, open throughout; a lock had here goes as it closes.
            unsafe { libc::flock(look.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) != 0 }
        };
        let hold = |secs: &str| {
            let other = Command::new(crate::common::needs::program("flock")).arg("--exclusive").arg(&file).args(["sleep", secs])
                .stdin(Stdio::null()).stdout(Stdio::null()).spawn().expect("flock could not be started");
            for _ in 0..300 {
                if held(&file) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            assert!(held(&file), "the other program never took the file");
            other
        };
        let mut other = hold("0.8");
        let since = std::time::Instant::now();
        assert_eq!(within(|| Ok(5)), Ok(5));
        assert!(since.elapsed() >= Duration::from_millis(300), "the work did not wait for the other program: {:?}", since.elapsed());
        assert!(other.wait().unwrap().success());

        let mut other = hold("30");
        let since = std::time::Instant::now();
        assert_eq!(within(|| Ok(6)), Err(silent_words().to_string()));
        assert!(since.elapsed() < DEADLINE * 2, "{:?}", since.elapsed());

        other.kill().unwrap();
        let _ = other.wait();

        pretend(Some("test:no-turn-file".into()));
        pretend_shared(Some(file.join("under-a-file")));
        let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let d = done.clone();
        let got = within(move || {
            d.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        });
        assert!(got.as_ref().is_err_and(|why| why.starts_with("The turn with the keyring could not be taken (")), "{got:?}");
        assert!(!done.load(std::sync::atomic::Ordering::SeqCst), "the work was done with no turn taken");
        let _ = std::fs::remove_dir_all(file.parent().unwrap());
    }

    #[test]
    fn an_answer_that_misses_its_deadline_is_still_heard() {
        pretend(Some("test:late".into()));
        let began = std::time::Instant::now();
        let waited = within_late(|| {
            std::thread::sleep(DEADLINE + Duration::from_millis(700));
            Ok(7)
        });
        assert_eq!(waited.now, Err(silent_words().to_string()));
        assert!(began.elapsed() < DEADLINE + Duration::from_millis(600), "{:?}", began.elapsed());

        let second = within(|| Ok(8));
        assert_eq!(second, Ok(8));
        assert_eq!(waited.late.expect("the way to hear it").wait(), Ok(7));

        let prompt = within_late(|| Ok(9));
        assert!(prompt.now == Ok(9) && prompt.late.is_none());
        assert_eq!(Late::come(Ok(3)).wait(), Ok(3));
    }

    #[test]
    fn not_answered_yet_is_plain_and_never_answered_is_not_answered_late() {
        for (what, removing) in [("key", false), ("key", true), ("login", false)] {
            let meanwhile = waiting(what, removing);
            assert!(!meanwhile.failed, "not answered yet is not a failure");
            assert!(meanwhile.words.contains("has not answered yet") && meanwhile.words.contains("this line will say so") && meanwhile.words.contains(what), "{meanwhile:?}");
            let good = late(what, removing, &Ok(()));
            assert!(!good.failed && good.words == format!("The keyring answered late: the {what} is {}.", if removing { "removed" } else { "kept" }), "{good:?}");
            let bad = late(what, removing, &Err(locked_words().into()));
            assert!(bad.failed && bad.words.contains("answered late and did not") && bad.words.ends_with(locked_words()), "{bad:?}");
            let never = late(what, removing, &Err(never_words().into()));
            assert!(never.failed && !never.words.contains("answered late"), "{never:?}");
            assert_eq!(never.words, format!("The keyring never answered, so the {what} is not {}.", if removing { "removed" } else { "kept" }));
        }

        for removing in [false, true] {
            let (words, good) = crate::store::github::late_words(removing, &Err(never_words().to_string()));
            assert!(!good && !words.contains("answered late"), "{words}");
            assert!(words.starts_with("The keyring never answered, so the key is not"), "{words}");
            assert_eq!(crate::store::github::waiting_words(removing), waiting("key", removing).words);
        }
    }

    #[test]
    fn each_failure_is_said_as_what_it_is() {
        let named = |name: &str| {
            let call = zbus::Message::method_call("/a", "Ask").unwrap().destination("org.example.Asked").unwrap().build(&()).unwrap();
            let reply = zbus::Message::error(&call.header(), name).unwrap().build(&("why",)).unwrap();
            zbus::Error::from(reply)
        };
        assert_eq!(why(named("org.freedesktop.DBus.Error.ServiceUnknown")), missing_words());
        assert_eq!(why(named("org.freedesktop.DBus.Error.NameHasNoOwner")), missing_words());
        assert_eq!(why(named("org.freedesktop.DBus.Error.Spawn.ChildExited")), missing_words());
        assert_eq!(why(named("org.freedesktop.Secret.Error.IsLocked")), locked_words());
        assert_eq!(why(named("org.freedesktop.DBus.Error.NoReply")), silent_words());
        assert_eq!(why(zbus::Error::InputOutput(std::sync::Arc::new(std::io::Error::from(std::io::ErrorKind::TimedOut)))), silent_words());
    }
}
