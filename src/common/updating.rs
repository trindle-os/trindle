//! One update at a time. The ring, Apps and the box patcher are separate
//! programs, and each can put updates in. Before any of them starts it
//! takes the one lock here, and holds it until it has finished; the others
//! see who holds it and keep their own buttons off, or out of sight, until
//! it is free.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Weak};

use super::refused::{self, Refused};
use crate::toolkit::t;

const LOOK_AGAIN: std::time::Duration = std::time::Duration::from_secs(15);

const NAME: &str = "trindle-updating";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Who {

    Ring,

    Apps,

    Boxes,

    System,

    Unknown,
}

impl Who {

    fn word(self) -> &'static str {
        match self {
            Who::Ring => "ring",
            Who::Apps => "apps",
            Who::Boxes => "boxes",
            Who::System => "system",
            Who::Unknown => "",
        }
    }

    fn from_word(word: &str) -> Who {
        [Who::Ring, Who::Apps, Who::Boxes].into_iter().find(|w| w.word() == word.trim()).unwrap_or(Who::Unknown)
    }

    pub fn short(self) -> &'static str {
        match self {
            Who::Ring => t!("common-updating-short-ring"),
            Who::Apps => t!("common-updating-short-apps"),
            Who::Boxes => t!("common-updating-short-boxes"),
            Who::System => t!("common-updating-short-system"),
            Who::Unknown => t!("common-updating-short-unknown"),
        }
    }

    pub fn sentence(self) -> &'static str {
        match self {
            Who::Ring => t!("common-updating-sentence-ring"),
            Who::Apps => t!("common-updating-sentence-apps"),
            Who::Boxes => t!("common-updating-sentence-boxes"),
            Who::System => t!("common-updating-sentence-system"),
            Who::Unknown => t!("common-updating-sentence-unknown"),
        }
    }
}

fn whole(kind: libc::c_int) -> libc::flock {
    // SAFETY: flock is plain numbers, for which all zeros is a valid value;

    let mut l: libc::flock = unsafe { std::mem::zeroed() };
    l.l_type = kind as libc::c_short;
    l.l_whence = libc::SEEK_SET as libc::c_short;
    l
}

pub struct Lock {

    _file: Option<File>,
}

impl Drop for Lock {

    fn drop(&mut self) {
        let Some(file) = &self._file else { return };
        let ask = whole(libc::F_UNLCK);
        // SAFETY: fcntl reads `ask`, which lives until the call returns, and

        unsafe { libc::fcntl(file.as_raw_fd(), libc::F_OFD_SETLK, &ask) };
    }
}

impl Lock {

    pub fn take(path: &Path, who: Who) -> Result<Lock, Who> {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let opened = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false)
            .mode(0o600).custom_flags(libc::O_NOFOLLOW).open(path);
        let Ok(mut file) = opened else { return Ok(Lock { _file: None }) };
        let ask = whole(libc::F_WRLCK);
        // SAFETY: the descriptor is open for as long as `file` lives, and

        if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_OFD_SETLK, &ask) } != 0 {
            let e = std::io::Error::last_os_error().raw_os_error();
            if e == Some(libc::EAGAIN) || e == Some(libc::EACCES) {
                return Err(said(&mut file));
            }

            return Ok(Lock { _file: None });
        }
        let _ = file.set_len(0);
        let _ = file.seek(SeekFrom::Start(0));
        let _ = file.write_all(who.word().as_bytes());
        Ok(Lock { _file: Some(file) })
    }
}

fn said(file: &mut File) -> Who {
    let mut word = String::new();
    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.take(32).read_to_string(&mut word);
    Who::from_word(&word)
}

pub fn holder_at(path: &Path) -> Option<Who> {
    let mut file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW).open(path).ok()?;
    let mut ask = whole(libc::F_WRLCK);
    // SAFETY: the descriptor is open for as long as `file` lives, and `ask`

    if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_OFD_GETLK, &mut ask) } != 0 {
        return None;
    }
    (libc::c_int::from(ask.l_type) != libc::F_UNLCK).then(|| said(&mut file))
}

#[derive(Clone)]
pub struct Hold {

    _lock: Arc<Lock>,
}

static OURS: Mutex<Vec<(PathBuf, Weak<Lock>)>> = Mutex::new(Vec::new());

fn ours(path: &Path) -> Option<Arc<Lock>> {
    let mut all = OURS.lock().unwrap_or_else(|e| e.into_inner());
    all.retain(|(_, w)| w.strong_count() > 0);
    all.iter().find(|(p, _)| p == path).and_then(|(_, w)| w.upgrade())
}

#[cfg(not(test))]
pub fn place() -> PathBuf {
    super::files::scratch().join(NAME)
}

#[cfg(test)]
thread_local! {
    static PLACE: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
    static SYSTEM: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub fn place() -> PathBuf {
    PLACE.with(|p| p.borrow().clone()).unwrap_or_else(|| {
        let thread: String = format!("{:?}", std::thread::current().id()).chars().filter(char::is_ascii_digit).collect();
        std::env::temp_dir().join(format!("{NAME}-test-{}-{thread}", std::process::id()))
    })
}

#[cfg(test)]
pub fn pretend(path: Option<PathBuf>, system: bool) {
    PLACE.with(|p| *p.borrow_mut() = path);
    SYSTEM.with(|s| s.set(system));
}

pub fn begin(who: Who) -> Result<Hold, Who> {
    begin_at(&place(), who)
}

fn begin_at(path: &Path, who: Who) -> Result<Hold, Who> {
    if let Some(lock) = ours(path) {
        return Ok(Hold { _lock: lock });
    }
    if system_busy() {
        return Err(Who::System);
    }
    let lock = Arc::new(Lock::take(path, who)?);
    OURS.lock().unwrap_or_else(|e| e.into_inner()).push((path.to_path_buf(), Arc::downgrade(&lock)));
    Ok(Hold { _lock: lock })
}

pub fn begin_waiting(who: Who, most: std::time::Duration) -> Option<Hold> {
    let started = std::time::Instant::now();
    loop {
        match begin(who) {
            Ok(hold) => return Some(hold),
            Err(_) if started.elapsed() >= most => return None,
            Err(_) => std::thread::sleep(LOOK_AGAIN.min(most)),
        }
    }
}

pub fn other() -> Option<Who> {
    other_at(&place())
}

fn other_at(path: &Path) -> Option<Who> {
    if ours(path).is_some() {
        return None;
    }
    holder_at(path).or_else(|| system_busy().then_some(Who::System))
}

#[cfg(not(test))]
const DPKG_LOCKS: [&str; 2] = ["/var/lib/dpkg/lock-frontend", "/var/lib/dpkg/lock"];

#[cfg(not(test))]
const KERNEL_LOCKS: &str = "/proc/locks";
#[cfg(not(test))]
const MOUNTS: &str = "/proc/self/mountinfo";

#[cfg(not(test))]
const KEYS_FOR: std::time::Duration = std::time::Duration::from_secs(600);

type Key = (u64, u64, u64);

#[cfg(test)]
fn system_busy() -> bool {
    SYSTEM.with(std::cell::Cell::get)
}

#[cfg(not(test))]
fn system_busy() -> bool {
    static KEYS: Mutex<Option<(std::time::Instant, Vec<Key>)>> = Mutex::new(None);
    let mut kept = KEYS.lock().unwrap_or_else(|e| e.into_inner());
    if kept.as_ref().is_none_or(|(at, keys)| keys.is_empty() || at.elapsed() > KEYS_FOR) {
        let mounts = std::fs::read_to_string(MOUNTS).unwrap_or_default();
        let keys = DPKG_LOCKS.iter().flat_map(|f| keys_of(Path::new(f), &mounts)).collect();
        *kept = Some((std::time::Instant::now(), keys));
    }
    let keys = kept.as_ref().map(|(_, k)| k.clone()).unwrap_or_default();
    drop(kept);
    !keys.is_empty() && held_in(&std::fs::read_to_string(KERNEL_LOCKS).unwrap_or_default(), &keys)
}

#[cfg(not(test))]
fn keys_of(file: &Path, mounts: &str) -> Vec<Key> {
    use std::os::unix::fs::MetadataExt;
    let Ok(m) = std::fs::metadata(file) else { return Vec::new() };
    let mut keys = vec![(u64::from(libc::major(m.dev())), u64::from(libc::minor(m.dev())), m.ino())];
    if let Some((major, minor)) = mount_of(mounts, file) {
        if !keys.contains(&(major, minor, m.ino())) {
            keys.push((major, minor, m.ino()));
        }
    }
    keys
}

fn mount_of(mounts: &str, file: &Path) -> Option<(u64, u64)> {
    let mut best: Option<(usize, (u64, u64))> = None;
    for line in mounts.lines() {
        let mut fields = line.split(' ');
        let (Some(numbers), Some(point)) = (fields.nth(2), fields.nth(1)) else { continue };
        let Some((major, minor)) = numbers.split_once(':') else { continue };
        let (Ok(major), Ok(minor)) = (major.parse(), minor.parse()) else { continue };
        if file.starts_with(point) && best.is_none_or(|(deep, _)| point.len() >= deep) {
            best = Some((point.len(), (major, minor)));
        }
    }
    best.map(|(_, numbers)| numbers)
}

fn held_in(locks: &str, keys: &[Key]) -> bool {
    locks.lines().filter(|l| !l.contains("->")).any(|line| {
        line.split_whitespace().any(|word| {
            let mut parts = word.split(':');
            let (Some(major), Some(minor), Some(inode), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else { return false };
            let (Ok(major), Ok(minor), Ok(inode)) = (u64::from_str_radix(major, 16), u64::from_str_radix(minor, 16), inode.parse::<u64>()) else {
                return false;
            };
            keys.contains(&(major, minor, inode))
        })
    })
}

fn known(version: Option<&str>) -> Option<&str> {
    version.map(str::trim).filter(|v| !v.is_empty())
}

fn same_version(have: Option<&str>, to: Option<&str>) -> bool {
    matches!((known(have), known(to)), (Some(a), Some(b)) if a == b)
}

pub fn change_words(have: Option<&str>, to: Option<&str>) -> Option<String> {
    if same_version(have, to) {
        return known(have).map(|v| t!("common-updating-newer-build-of", version = v));
    }
    match (known(have), known(to)) {
        (Some(v), Some(to)) => Some(t!("common-updating-version-to", from = v, to = to)),
        (Some(v), None) => Some(t!("common-updating-now-version", version = v)),
        (None, Some(to)) => Some(t!("common-updating-to-version", version = to)),
        (None, None) => None,
    }
}

pub fn waiting_words(have: Option<&str>, to: Option<&str>) -> String {
    if same_version(have, to) {
        return t!("common-updating-newer-waiting").into();
    }
    match known(to) {
        Some(to) => t!("common-updating-version-waiting", version = to),
        None => t!("common-updating-update-waiting").into(),
    }
}

pub fn needs_password(system_waiting: usize) -> bool {
    system_waiting > 0
}

pub fn admin_reason(step: &str) -> Option<&'static str> {
    match step {
        "apt-upgrade" => Some(t!("common-updating-reason-all")),
        "apt-upgrade-debian" => Some(t!("common-updating-reason-debian")),
        "apt-upgrade-trindle" => Some(t!("common-updating-reason-trindle")),
        "firmware-update" => Some(t!("common-updating-reason-firmware")),
        "apt-refresh" => Some(t!("common-updating-reason-refresh")),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Part {
    Done,

    NoPassword,

    Refused(Refused),

    Failed(String),
}

impl Part {

    pub fn of_admin(res: Result<(), String>) -> Part {
        match res {
            Ok(()) => Part::Done,
            Err(e) => match refused::of_words(&e) {
                Some(Refused::Cancelled) => Part::NoPassword,
                Some(other) => Part::Refused(other),
                None => Part::Failed(e),
            },
        }
    }

    fn not_started(&self) -> Option<String> {
        match self {
            Part::NoPassword => Some(t!("common-updating-because", why = Refused::Cancelled.why())),
            Part::Refused(r) => Some(t!("common-updating-because", why = r.why())),
            Part::Done | Part::Failed(_) => None,
        }
    }

    pub fn why(&self) -> Option<String> {
        match self {
            Part::Done => None,
            Part::Failed(e) if e.trim().is_empty() => Some(t!("common-updating-no-reason").into()),
            Part::Failed(e) => Some(t!("common-updating-failed", why = e.trim())),
            other => other.not_started().map(|why| t!("common-updating-not-installed", why = &why)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::fmt::Write as _;
    use super::*;

    #[test]
    fn a_password_is_asked_for_only_when_the_system_has_updates() {
        assert!(!needs_password(0), "apps alone never bring the password box");
        assert!(needs_password(1) && needs_password(40));

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let ring = std::fs::read_to_string(src.join("ring/updates.rs")).unwrap();
        assert!(!ring.contains("pkexec") && !ring.contains("debian_watching(") && !ring.contains("admin_watching("), "the ring puts nothing in itself");
        let page = std::fs::read_to_string(src.join("store/update_page/install.rs")).unwrap();
        let install = page.split_once("fn install_all_in(").unwrap().1.split_once("\n}\n").unwrap().0;
        let asks = install.find("updating::needs_password(").expect("Apps asks whether a password is needed");
        assert!(install.find("upgrade_system(").is_some_and(|at| at > asks));
    }

    #[test]
    fn every_step_behind_the_password_ends_through_the_one_rule() {
        let steps = [["\"apt-", "upgrade"].concat(), ["\"firmware-", "update\""].concat()];
        let mut wrong = Vec::new();
        for (name, file) in named_sources() {
            if name.starts_with("admin/") || name == "common/updating.rs" || name == "common/run.rs" || name == "polkit/mod.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap();
            let lines = functions(&text);
            for (function, line) in &lines {
                if line.trim_start().starts_with("//") {
                    continue;
                }

                if line.contains("_watching(") && line.trim_end().ends_with(DROPPED) {
                    wrong.push(format!("{name} {function}: its ending is thrown away: {}", line.trim()));
                }
                if !steps.iter().any(|s| line.contains(s.as_str())) {
                    continue;
                }
                let body: String = lines.iter().filter(|(f, _)| f == function).fold(String::new(), |mut s, (_, l)| { let _ = writeln!(s, "{l}"); s });
                let runs = line.contains("_watching(") || line.contains("upgrade_system(");
                let ended = body.contains("Part::of_admin(") || line.contains("upgrade_system(");
                if !runs || !ended {
                    wrong.push(format!("{name} {function}: {}", line.trim()));
                }
            }
        }
        assert!(wrong.is_empty(), "a step behind the password is run or ended some other way:\n{}", wrong.join("\n"));
    }

    const DROPPED: &str = concat!("move |", "_| {");

    #[test]
    fn a_newer_build_of_the_same_version_is_said_as_one() {
        assert_eq!(change_words(Some("3.2"), Some("3.3")).as_deref(), Some("version 3.2 to 3.3"));
        assert_eq!(change_words(Some("1.3.8"), Some("1.3.8")).as_deref(), Some("a newer build of version 1.3.8"));
        assert_eq!(change_words(Some(" 1.3.8"), Some("1.3.8 ")).as_deref(), Some("a newer build of version 1.3.8"));
        assert_eq!(change_words(Some("3.2"), None).as_deref(), Some("now version 3.2"));
        assert_eq!(change_words(Some("3.2"), Some(" ")).as_deref(), Some("now version 3.2"));
        assert_eq!(change_words(None, Some("3.3")).as_deref(), Some("to version 3.3"));
        assert_eq!(change_words(None, None), None);
        assert_eq!(change_words(Some(""), Some("")), None);
        assert_eq!(waiting_words(Some("3.2"), Some("3.3")), "version 3.3 is waiting");
        assert_eq!(waiting_words(Some("1.3.8"), Some("1.3.8")), "a newer build is waiting");
        assert_eq!(waiting_words(None, Some("3.3")), "version 3.3 is waiting");
        assert_eq!(waiting_words(Some("3.2"), None), "an update is waiting");
        assert_eq!(waiting_words(Some("3.2"), Some("")), "an update is waiting");

        for v in ["1", "1.3.8", "v0.41.0", "51.0", "2026.10.1-beta.3+build.20261001"] {
            for words in [change_words(Some(v), Some(v)).unwrap(), waiting_words(Some(v), Some(v))] {
                assert!(!words.contains(&format!("{v} to {v}")) && !words.contains(&format!("version {v} is waiting")), "{words}");
            }
        }
    }

    #[test]
    fn only_this_file_words_a_change_of_version() {
        let files: Vec<PathBuf> = named_sources().into_iter().map(|f| f.1).collect();

        let own = [["} is ", "waiting\""].concat(), ["update is ", "waiting"].concat(), ["build is ", "waiting"].concat()];
        let mut wrong = Vec::new();
        for file in files {

            if file.ends_with("common/updating.rs") || file.ends_with("store/update_page/tests.rs") {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap_or_default();
            for (n, (_, line)) in functions(&text).iter().enumerate() {
                let l = line.trim_start();
                let pair = l.contains("format!(") && l.contains("version {") && l.contains("} to {");
                if !l.starts_with("//") && (pair || own.iter().any(|w| l.contains(w.as_str()))) {
                    wrong.push(format!("{}:{}: {l}", file.display(), n + 1));
                }
            }
        }
        assert!(wrong.is_empty(), "these word an update's versions themselves, not through change_words or waiting_words:\n{}", wrong.join("\n"));
    }

    pub fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-updating-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir.join(NAME)
    }

    #[test]
    fn the_lock_is_taken_refused_to_a_second_and_free_again() {
        let path = temp("taken");
        assert_eq!(holder_at(&path), None, "nothing holds a file that is not there");
        let first = Lock::take(&path, Who::Ring).expect("a free lock is taken");
        assert_eq!(holder_at(&path), Some(Who::Ring));
        assert_eq!(Lock::take(&path, Who::Apps).err(), Some(Who::Ring), "a second holder is refused and told who has it");
        assert_eq!(holder_at(&path), Some(Who::Ring), "being refused changes nothing");
        drop(first);
        assert_eq!(holder_at(&path), None, "letting go frees it, though the file and its word stay");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "ring");
        let second = Lock::take(&path, Who::Apps).expect("free again");
        assert_eq!(holder_at(&path), Some(Who::Apps));
        drop(second);
    }

    #[test]
    fn one_program_shares_its_lock_between_its_jobs() {
        let path = temp("shared");
        pretend(Some(path.clone()), false);
        let apps = begin(Who::Apps).expect("free");
        let debian = begin(Who::Apps).expect("the same program's second job shares it");
        assert_eq!(other(), None, "its own update is not somebody else's");
        assert_eq!(holder_at(&path), Some(Who::Apps));
        drop(apps);
        assert_eq!(holder_at(&path), Some(Who::Apps), "held until the last job is over");
        drop(debian);
        assert_eq!(holder_at(&path), None);

        let ring = Lock::take(&path, Who::Ring).ok().unwrap();
        assert_eq!(begin(Who::Apps).err(), Some(Who::Ring));
        assert_eq!(other(), Some(Who::Ring));
        drop(ring);
        assert_eq!(other(), None);
        assert!(begin(Who::Apps).is_ok());
        pretend(None, false);
    }

    #[test]
    fn debian_s_installer_at_work_counts_as_a_holder() {
        let path = temp("system");
        pretend(Some(path.clone()), true);
        assert_eq!(other(), Some(Who::System));
        assert_eq!(begin(Who::Ring).err(), Some(Who::System));
        assert!(!path.exists(), "nothing was taken");
        pretend(Some(path.clone()), false);
        assert_eq!(other(), None);

        let ring = Lock::take(&path, Who::Ring).ok().unwrap();
        assert!(begin_waiting(Who::Boxes, std::time::Duration::ZERO).is_none());
        drop(ring);
        assert!(begin_waiting(Who::Boxes, std::time::Duration::ZERO).is_some());
        pretend(None, false);
    }

    const HOLD_FOR_TEST: &str = "TRINDLE_TEST_UPDATING_HOLD";

    #[test]
    #[ignore = "not a test of its own: the test below starts it as a second process"]
    fn a_holder_for_the_test_below() {
        let Some(path) = std::env::var_os(HOLD_FOR_TEST) else { return };
        let _lock = Lock::take(Path::new(&path), Who::Apps).expect("free");
        std::thread::sleep(std::time::Duration::from_secs(60));
    }

    #[test]
    fn the_kernel_frees_the_lock_when_its_holder_is_killed() {
        let path = temp("killed");
        let mut holder = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--ignored", "--exact", "common::updating::tests::a_holder_for_the_test_below", "--test-threads=1"])
            .env(HOLD_FOR_TEST, &path)
            .env_remove("DISPLAY").env_remove("WAYLAND_DISPLAY").env_remove("DBUS_SESSION_BUS_ADDRESS")
            .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
            .spawn().unwrap();
        let waited = std::time::Instant::now();
        while holder_at(&path).is_none() && waited.elapsed().as_secs() < 30 {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert_eq!(holder_at(&path), Some(Who::Apps), "the other program holds it");
        assert_eq!(Lock::take(&path, Who::Ring).err(), Some(Who::Apps));

        holder.kill().unwrap();
        holder.wait().unwrap();
        assert_eq!(holder_at(&path), None, "nothing is left behind by a program that died");
        assert!(Lock::take(&path, Who::Ring).is_ok());
    }

    #[test]
    fn dpkg_s_lock_is_found_in_the_kernel_s_list() {
        let locks = "1: POSIX  ADVISORY  WRITE 4242 fd:01:1311234 0 EOF\n\
                     2: FLOCK  ADVISORY  WRITE 900 00:1b:77 0 EOF\n\
                     3: -> POSIX  ADVISORY  WRITE 4300 08:02:555 0 EOF\n";
        assert!(held_in(locks, &[(0xfd, 1, 1_311_234)]));
        assert!(held_in(locks, &[(8, 9, 1), (0, 0x1b, 77)]), "any of the files' numbers will do");
        assert!(!held_in(locks, &[(0xfd, 2, 1_311_234)]), "the same number on another filesystem is another file");
        assert!(!held_in(locks, &[(8, 2, 555)]), "waiting for a lock is not holding it");
        assert!(!held_in("", &[(8, 2, 555)]));
    }

    #[test]
    fn the_filesystem_s_numbers_come_from_the_deepest_mount() {
        let mounts = "23 1 0:21 / / rw,relatime shared:1 - btrfs /dev/sda2 rw\n\
                      40 23 0:38 / /var rw shared:9 - btrfs /dev/sda3 rw\n\
                      41 23 8:1 / /boot rw shared:10 - ext4 /dev/sda1 rw\n";
        assert_eq!(mount_of(mounts, Path::new("/var/lib/dpkg/lock")), Some((0, 38)));
        assert_eq!(mount_of(mounts, Path::new("/usr/bin/apt")), Some((0, 21)));
        assert_eq!(mount_of("", Path::new("/var/lib/dpkg/lock")), None);
    }

    fn functions(text: &str) -> Vec<(String, &str)> {
        let text = text.split("#[cfg(test)]\nmod tests").next().unwrap_or(text);
        let text = text.split("#[cfg(test)]\npub(super) mod tests").next().unwrap_or(text);
        let text = text.split("#[cfg(test)]\nuse ").next().unwrap_or(text);
        let mut inside = String::new();
        let mut out = Vec::new();
        for line in text.lines() {
            let t = line.trim_start();
            let t = t.strip_prefix("pub(super) ").or_else(|| t.strip_prefix("pub(crate) ")).or_else(|| t.strip_prefix("pub ")).unwrap_or(t);
            if let Some(rest) = t.strip_prefix("fn ") {
                inside = rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
            }
            out.push((inside.clone(), line));
        }
        out
    }

    fn sources(dir: &Path, found: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                sources(&path, found);
            } else if path.extension().is_some_and(|e| e == "rs") {
                found.push(path);
            }
        }
    }

    fn named_sources() -> Vec<(String, PathBuf)> {
        let mut all = Vec::new();
        for (tree, prefix) in crate::common::source::trees() {
            let mut files = Vec::new();
            sources(&tree, &mut files);
            all.extend(files.into_iter().map(|f| (format!("{prefix}{}", f.strip_prefix(&tree).unwrap().to_string_lossy()), f)));
        }
        all
    }

    fn scan_for_starts(starts: &[String]) -> (Vec<String>, Vec<String>) {
        let mut found: Vec<String> = Vec::new();
        let mut loose: Vec<String> = Vec::new();
        for (name, file) in named_sources() {

            if name.starts_with("admin/") || name == "common/updating.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap();
            let lines = functions(&text);
            for (i, (function, line)) in lines.iter().enumerate() {
                let defines = line.trim_start().starts_with("pub fn ") || line.trim_start().starts_with("fn ");
                if line.trim_start().starts_with("//") || defines || !starts.iter().any(|s| line.contains(s.as_str())) {
                    continue;
                }
                let entry = format!("{name} {function}");

                let before: String = lines[..=i].iter().filter(|(f, _)| f == function).fold(String::new(), |mut s, (_, l)| { let _ = writeln!(s, "{l}"); s });
                let holds = before.contains("hold: Hold") || before.contains("updating::begin(");
                if !holds {
                    loose.push(entry.clone());
                }
                if !found.contains(&entry) {
                    found.push(entry);
                }
            }
        }
        found.sort();
        (found, loose)
    }

    fn apps_page_takes_the_turn_first(src: &Path) {

        let page = std::fs::read_to_string(src.join("store/update_page/install.rs")).unwrap();
        let lines = functions(&page);
        let mut takers: Vec<&str> = lines.iter().filter(|(_, l)| l.contains("updating::begin(")).map(|(f, _)| f.as_str()).collect();
        takers.dedup();
        assert_eq!(takers, ["install_all_in", "update_app", "update_part", "update_trindle", "update_debian", "update_device"]);
        for taker in takers {
            let body: String = lines.iter().filter(|(f, _)| f == taker).fold(String::new(), |mut s, (_, l)| { let _ = writeln!(s, "{l}"); s });
            assert!(body.contains("updating::begin(Who::Apps)"), "{taker} takes Apps' turn");
            let starts_at = ["update_flatpak(", "upgrade_system(", "update_firmware(", "update_apps("].iter().filter_map(|w| body.find(w)).min();
            assert!(starts_at.is_some_and(|at| body.find("updating::begin(").is_some_and(|took| took < at)), "{taker} takes the turn before it starts anything");
        }

        let plugin: String = lines.iter().filter(|(f, _)| f == "update_plugin").fold(String::new(), |mut s, (_, l)| { let _ = writeln!(s, "{l}"); s });
        assert!(plugin.contains("plugin_page::install("));
    }

    #[test]
    fn every_way_an_update_starts_goes_through_the_lock() {

        let quoted = |w: &str| format!("\"{w}\"");
        let starts = [

            ["\"apt-", "upgrade"].concat(),
            quoted("firmware-update"),
            ["\"flatpak\"", " \"update\"", " \"--user\"", " \"-y\""].join(","),
            ["fetch_", "install("].concat(),
            ["fetch_update_", "unseen("].concat(),
        ];
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let (found, loose) = scan_for_starts(&starts);
        assert!(loose.is_empty(), "an update is started without the lock: {loose:?}");
        assert_eq!(found, [
            "store/plugin_page.rs install_as",
            "store/update_page/install.rs install_all_in",
            "store/update_page/install.rs update_apps",
            "store/update_page/install.rs update_debian",
            "store/update_page/install.rs update_firmware",
            "store/update_page/install.rs update_flatpak",
            "store/update_page/install.rs update_trindle",
        ], "the ways an update starts have changed: each new one must hold the lock, then be listed here");

        let patch = std::fs::read_to_string(src.join("store/patch.rs")).unwrap();
        assert!(patch.contains("updating::begin_waiting("));
        apps_page_takes_the_turn_first(&src);
    }

    #[test]
    fn each_holder_says_plainly_what_is_going_on() {
        for who in [Who::Ring, Who::Apps, Who::Boxes, Who::System, Who::Unknown] {
            let words = who.sentence();
            assert!(words.ends_with('.') && !words.contains('\u{2014}'), "{words}");
            if !who.word().is_empty() && who != Who::System {
                assert_eq!(Who::from_word(who.word()), who);
            }
        }
        assert_eq!(Who::from_word("something else"), Who::Unknown);
        assert_eq!(Who::Ring.sentence(), "Updates are being installed from the ring.");
    }
}
