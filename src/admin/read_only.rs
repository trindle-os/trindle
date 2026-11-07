//! A read-only system: /usr and /opt are each a
//! read-only bind mount of themselves, opened by this helper alone for the
//! length of one software change and closed again however the change ends.

use std::cell::Cell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;

use super::{at, gone, replace, run, Outcome};
use crate::toolkit::t;

/// Put there when the lock is switched on (`system-read-only on`). Without
/// it nothing here runs a command, so this code alone changes no computer.
pub const MARKER: &str = crate::common::snapshots::READ_ONLY_MARKER;
/// Root's own file in the runtime folder: held locked by the one helper that
/// has the system open, with its process number in it. The kernel lets go of
/// the lock when that helper ends, however it ends.
pub const STATE: &str = "/run/trindle-system-open";
/// What is kept read-only; /opt only where it is a folder.
pub const PLACES: [&str; 2] = ["/usr", "/opt"];
/// Kept writable on top of a locked /usr: the owner's own programs.
const LOCAL: &str = "/usr/local";
/// The watchdog and the lock at each start, shipped switched off.
const SERVICE: &str = "trindle-read-only.service";
const TIMER: &str = "trindle-read-only.timer";
const MOUNTS: &str = "/proc/self/mountinfo";
/// dpkg's own lock: held while packages are going in, by whatever started it.
const DPKG_LOCK: &str = "/var/lib/dpkg/lock";

/// The helper's commands that change the software, each run with the system
/// open from start to end, so a change is one opening however many steps it
/// takes. `apt-get` and `dpkg` open it for themselves as well (`writes_system`).
pub const CHANGES: [&str; 14] = ["apt-upgrade", "apt-upgrade-debian", "apt-upgrade-trindle", "source-add", "source-custom",
                                 "source-remove", "app-remove", "tailscale-on", "sharing-setup", "voice-install",
                                 "voice-remove", "voice-tools-remove", "locale", "locale-remove"];

thread_local! {
    /// How many openings are alive on this thread; only the first opens.
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// The system held open. Dropping it closes the system again, on an error
/// and in a panic as well.
pub struct Open {
    /// The locked state file, for the opening that opened; nothing for one
    /// inside another, or with the read-only system not set up.
    held: Option<File>,
    counted: bool,
}

/// Whether a command writes under /usr, so needs the system open.
pub fn writes_system(argv: &[&str]) -> bool {
    match argv.first() {
        Some(&"apt-get") => !argv.iter().any(|a| matches!(*a, "update" | "-s" | "--simulate")),
        Some(&"dpkg") | Some(&"locale-gen") => true,
        _ => false,
    }
}

/// Runs `change` with the system open, and closes it after; a failure to
/// close is said with the change's own news.
pub fn around(change: impl FnOnce() -> Outcome) -> Outcome {
    let open = open()?;
    let done = change();
    match (done, open.close()) {
        (done, Ok(())) => done,
        (Ok(()), Err(left)) => Err(left),
        (Err(e), Err(left)) => Err(format!("{e} {left}")),
    }
}

/// Opens the system for one change.
pub fn open() -> Result<Open, String> {
    if DEPTH.get() > 0 {
        DEPTH.set(DEPTH.get() + 1);
        return Ok(Open { held: None, counted: true });
    }
    if !set_up() {
        return Ok(Open { held: None, counted: false });
    }
    let mut state = match take() {
        Ok(f) => f,
        Err(Taken::Busy) => return Err(t!("admin-system-busy").into()),
        Err(Taken::Unusable(why)) => return Err(t!("admin-system-not-opened", why = why)),
    };
    // Written before anything is opened, so an ending part-way is on record.
    note(&mut state, &std::process::id().to_string()).map_err(|e| t!("admin-system-not-opened", why = e))?;
    DEPTH.set(1);
    let open = Open { held: Some(state), counted: true };
    unlock()?;
    Ok(open)
}

/// Every place made writable.
fn unlock() -> Outcome {
    let mounts = mounts().map_err(|e| t!("admin-system-not-opened", why = e))?;
    for place in places() {
        // Not yet a mount of itself: the system has not been closed since it
        // started, so it is open already.
        if bound(&mounts, place).is_some() {
            run(&["mount", "-o", "remount,bind,rw", place], None).map_err(|e| t!("admin-system-not-opened", why = e))?;
        }
    }
    Ok(())
}

impl Open {
    /// Closes the system now, saying when it could not be.
    pub fn close(mut self) -> Outcome {
        self.end()
    }

    fn end(&mut self) -> Outcome {
        if std::mem::take(&mut self.counted) {
            DEPTH.set(DEPTH.get().saturating_sub(1));
        }
        let Some(mut state) = self.held.take() else { return Ok(()) };
        // Left with the number in it, the next look closes it again.
        let local = relock()?;
        note(&mut state, "").map_err(|e| t!("admin-system-left-open", why = e))?;
        local
    }

    /// Lets go without closing: switching the lock off leaves it open.
    fn leave_open(mut self) {
        if std::mem::take(&mut self.counted) {
            DEPTH.set(DEPTH.get().saturating_sub(1));
        }
        if let Some(mut state) = self.held.take() {
            // A number left there is read only by this process's fault hook.
            let _ = note(&mut state, "");
        }
    }
}

impl Drop for Open {
    fn drop(&mut self) {
        if let Err(e) = self.end() {
            // Only an error or a panic comes here; the next look closes it.
            eprintln!("{e}");
        }
    }
}

/// The watchdog's and the start-up's step: closes the system unless a change
/// is going on. Never opens anything.
pub fn lock(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin system-lock".into());
    }
    close_unless_held(true)
}

/// dpkg's post-invoke hook: closes what its pre-invoke opened. dpkg still
/// holds its own lock while the hook runs, so that is not waited for.
pub fn dpkg_post_invoke(args: &[String]) -> Outcome {
    by_root()?;
    if !args.is_empty() {
        return Err("usage: trindle-admin dpkg-post-invoke".into());
    }
    close_unless_held(false)
}

/// dpkg's pre-invoke hook, for dpkg typed by hand and Debian's daily
/// security updates: opens the system for dpkg's run without holding the
/// state file, so post-invoke, or else the watchdog, closes it.
pub fn dpkg_pre_invoke(args: &[String]) -> Outcome {
    by_root()?;
    if !args.is_empty() {
        return Err("usage: trindle-admin dpkg-pre-invoke".into());
    }
    before_dpkg()
}

fn before_dpkg() -> Outcome {
    if !set_up() {
        return Ok(());
    }
    // Held only while opening, so nothing closes it part-way.
    let _held = match take() {
        // The helper that has it open runs this dpkg, and closes it after.
        Err(Taken::Busy) => return Ok(()),
        held => held.ok(),
    };
    unlock()
}

/// `system-run -- PROGRAM ARGS...`: runs one program with the system open,
/// and closes it again however the program ends.
pub fn system_run(args: &[String]) -> Outcome {
    by_root()?;
    run_open(args)
}

fn run_open(args: &[String]) -> Outcome {
    let usage = || "usage: trindle-admin system-run -- PROGRAM ARGS...".to_string();
    let Some((dashes, argv)) = args.split_first() else { return Err(usage()) };
    if dashes != "--" || argv.is_empty() {
        return Err(usage());
    }
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    around(|| run_shown(&argv))
}

/// Runs a program as it is, its words going where the caller's go.
fn run_shown(argv: &[&str]) -> Outcome {
    #[cfg(test)]
    if let Some(answer) = super::stand_in::run(argv) {
        return answer;
    }
    let ended = std::process::Command::new(argv[0]).args(&argv[1..]).status()
        .map_err(|e| t!("admin-tool-error", tool = argv[0], why = e.to_string()))?;
    if ended.success() { Ok(()) } else { Err(t!("admin-tool-failed", tool = argv[0])) }
}

/// The commands that open the system outside a change are root's alone,
/// never through pkexec, so nothing Settings or Apps runs can reach them.
fn root_only(uid: u32, pkexec: Option<std::ffi::OsString>) -> Outcome {
    if uid == 0 && pkexec.is_none() { Ok(()) } else { Err(t!("admin-system-root-only").into()) }
}

fn by_root() -> Outcome {
    // SAFETY: asks the kernel for this process's real user id; it cannot fail.
    root_only(unsafe { libc::getuid() }, std::env::var_os("PKEXEC_UID"))
}

/// `system-read-only on|off`, the switch in Settings.
pub fn switch(args: &[String]) -> Outcome {
    match args {
        [w] if w == "on" => switch_on(),
        [w] if w == "off" => switch_off(),
        _ => Err("usage: trindle-admin system-read-only on|off".into()),
    }
}

/// Only on Btrfs, where a change gone wrong has a snapshot to go back to.
fn switch_on() -> Outcome {
    let mounts = mounts().map_err(|e| t!("admin-system-not-opened", why = e))?;
    if !root_is_btrfs(&mounts) {
        return Err(t!("admin-system-needs-btrfs").into());
    }
    // Both wait for the mark, so they do nothing until it is written.
    run(&["systemctl", "enable", SERVICE], None)?;
    run(&["systemctl", "enable", "--now", TIMER], None)?;
    std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
    replace(&at(MARKER), b"", 0o644)?;
    close_unless_held(true)
}

/// Leaves the system open, held for the while so no change is under way.
fn switch_off() -> Outcome {
    let open = open()?;
    // Should either fail, the opening's drop closes the system again.
    run(&["systemctl", "disable", "--now", TIMER, SERVICE], None)?;
    gone(&at(MARKER))?;
    open.leave_open();
    Ok(())
}

fn root_is_btrfs(mounts: &str) -> bool {
    mounts.lines().rev().find(|l| l.split(' ').nth(4) == Some("/"))
        .and_then(|l| l.split(" - ").nth(1))
        .is_some_and(|after| after.split(' ').next() == Some("btrfs"))
}

/// Closes the system unless a helper holds it open, and, when asked, unless
/// dpkg is at work.
fn close_unless_held(wait_for_dpkg: bool) -> Outcome {
    if !set_up() {
        return Ok(());
    }
    let state = match take() {
        Ok(f) => Some(f),
        // A helper at work closes it itself when it is done.
        Err(Taken::Busy) => return Ok(()),
        // Closing matters more than the record of who opened it.
        Err(Taken::Unusable(_)) => None,
    };
    if wait_for_dpkg && dpkg_at_work() {
        return Ok(());
    }
    let local = relock()?;
    if let Some(mut f) = state {
        note(&mut f, "").map_err(|e| t!("admin-system-left-open", why = e))?;
    }
    local
}

/// For a panic, which in the helper ends it at once with no drop run:
/// closes the system when this process was the one that opened it.
pub fn after_panic() {
    let mut said = String::new();
    let ours = File::open(at(STATE)).and_then(|mut f| f.read_to_string(&mut said)).is_ok()
        && said.trim() == std::process::id().to_string();
    if ours {
        if let Err(e) = relock().and_then(|local| local) {
            eprintln!("{e}");
        }
    }
}

fn set_up() -> bool {
    at(MARKER).is_file()
}

/// The places there are on this computer: folders, never a link.
fn places() -> impl Iterator<Item = &'static str> {
    PLACES.into_iter().filter(|p| folder(p))
}

fn folder(path: &str) -> bool {
    std::fs::symlink_metadata(at(path)).is_ok_and(|m| m.is_dir())
}

fn mounts() -> Result<String, String> {
    std::fs::read_to_string(at(MOUNTS)).map_err(|e| e.to_string())
}

/// Whether `place` is mounted on itself, and if so whether read-only: the
/// last mount there is the one in use.
fn bound(mounts: &str, place: &str) -> Option<bool> {
    mounts.lines().rev().find_map(|l| {
        let f: Vec<&str> = l.split(' ').collect();
        (f.get(4) == Some(&place)).then(|| f.get(5).is_some_and(|o| o.split(',').any(|w| w == "ro")))
    })
}

/// Every place made a read-only mount of itself. Each is tried whatever
/// became of the others, and one still in use is tried again for a moment.
/// An error when one is left open; once all are closed, how /usr/local fared.
fn relock() -> Result<Outcome, String> {
    // The mark gone mid-change (the package removed, or the lock switched off
    // by hand): nothing is left to open it again, so nothing is locked.
    if !set_up() {
        return Ok(Ok(()));
    }
    let mounts = mounts();
    let mut left = Vec::new();
    let mut local = Ok(());
    for place in places() {
        // With the table of mounts unread a second bind is never stacked: a
        // place not yet bound then fails to close, and that is said.
        let unbound = mounts.as_ref().is_ok_and(|m| bound(m, place).is_none());
        let bind = if unbound { run(&["mount", "--bind", place, place], None) } else { Ok(()) };
        let mut closed = bind.and_then(|()| run(&["mount", "-o", "remount,bind,ro", place], None));
        for _ in 0..4 {
            if closed.is_ok() {
                break;
            }
            // A file still being written there; the test's stand-in answers the same each time.
            #[cfg(not(test))]
            std::thread::sleep(std::time::Duration::from_millis(250));
            closed = run(&["mount", "-o", "remount,bind,ro", place], None);
        }
        match closed {
            Err(e) => left.push(format!("{place}: {e}")),
            Ok(()) if place == "/usr" => local = local_writable(unbound, &mounts),
            Ok(()) => {}
        }
    }
    if !left.is_empty() {
        return Err(t!("admin-system-left-open", why = left.join("; ")));
    }
    Ok(local.map_err(|e| t!("admin-system-local-locked", why = e)))
}

/// /usr/local made a writable mount of itself over the locked /usr. A /usr
/// bound afresh hides the one there was, so it is made again then.
fn local_writable(usr_bound_afresh: bool, mounts: &Result<String, String>) -> Outcome {
    // Unread, /usr was not bound afresh either, so what is there stays.
    let Ok(mounts) = mounts else { return Ok(()) };
    if !folder(LOCAL) {
        return Ok(());
    }
    let now = if usr_bound_afresh { None } else { bound(mounts, LOCAL) };
    match now {
        Some(false) => Ok(()),
        Some(true) => run(&["mount", "-o", "remount,bind,rw", LOCAL], None),
        None => run(&["mount", "--bind", LOCAL, LOCAL], None)
            .and_then(|()| run(&["mount", "-o", "remount,bind,rw", LOCAL], None)),
    }
}

enum Taken {
    Busy,
    Unusable(String),
}

/// The state file, locked for this process alone.
fn take() -> Result<File, Taken> {
    let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false)
        .mode(0o600).custom_flags(libc::O_NOFOLLOW).open(at(STATE))
        .map_err(|e| Taken::Unusable(e.to_string()))?;
    // SAFETY: the descriptor is open for as long as `file` lives.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let e = std::io::Error::last_os_error();
        return Err(if e.raw_os_error() == Some(libc::EWOULDBLOCK) { Taken::Busy } else { Taken::Unusable(e.to_string()) });
    }
    Ok(file)
}

fn note(file: &mut File, text: &str) -> Result<(), String> {
    file.set_len(0).and_then(|()| file.seek(SeekFrom::Start(0))).and_then(|_| file.write_all(text.as_bytes()))
        .map_err(|e| e.to_string())
}

/// Whether something holds dpkg's lock: packages going in that a closed
/// system would break half-way.
fn dpkg_at_work() -> bool {
    // Asking about a lock needs only the file open to read.
    let Ok(file) = File::open(at(DPKG_LOCK)) else { return false };
    // SAFETY: flock is plain numbers, for which all zeros is a valid value.
    let mut ask: libc::flock = unsafe { std::mem::zeroed() };
    ask.l_type = libc::F_WRLCK as libc::c_short;
    ask.l_whence = libc::SEEK_SET as libc::c_short;
    // SAFETY: `ask` outlives the call, which only writes the answer into it.
    let asked = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETLK, &mut ask) };
    asked == 0 && ask.l_type != libc::F_UNLCK as libc::c_short
}

#[cfg(test)]
mod tests {
    use super::super::stand_in::StandIn;
    use super::*;

    const RW_USR: &str = "mount -o remount,bind,rw /usr";
    const RW_OPT: &str = "mount -o remount,bind,rw /opt";
    const RO_USR: &str = "mount -o remount,bind,ro /usr";
    const RO_OPT: &str = "mount -o remount,bind,ro /opt";
    const BIND_LOCAL: &str = "mount --bind /usr/local /usr/local";
    const RW_LOCAL: &str = "mount -o remount,bind,rw /usr/local";
    const ROOT_BTRFS: &str = "30 1 0:26 /@rootfs / rw,noatime shared:1 - btrfs /dev/vda3 rw\n";

    /// A computer with the read-only system set up and closed.
    fn set_up(s: &StandIn) {
        s.write(MARKER, b"");
        std::fs::create_dir_all(s.path("/run")).unwrap();
        std::fs::create_dir_all(s.path("/usr/bin")).unwrap();
        std::fs::create_dir_all(s.path("/opt")).unwrap();
        s.write(MOUNTS, b"30 1 0:26 /@rootfs / rw,noatime shared:1 - btrfs /dev/vda3 rw\n\
                          60 30 0:26 /@rootfs/usr /usr ro,noatime shared:1 - btrfs /dev/vda3 rw\n\
                          61 30 0:26 /@rootfs/opt /opt ro,noatime shared:1 - btrfs /dev/vda3 rw\n");
    }

    fn state(s: &StandIn) -> String {
        std::fs::read_to_string(s.path(STATE)).unwrap_or_default()
    }

    #[test]
    fn a_change_opens_the_system_first_and_closes_it_after() {
        let s = StandIn::all_well("ro-order");
        set_up(&s);
        let mut seen = String::new();
        let done = around(|| {
            seen = state(&s);
            run(&["apt-get", "-q", "-y", "install", "thing"], None)
        });
        assert_eq!(done, Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "apt-get -q -y install thing", RO_USR, RO_OPT]);
        assert_eq!(seen, std::process::id().to_string(), "who opened it is on record while it is open");
        assert_eq!(state(&s), "", "and cleared once it is closed");
        assert_eq!(DEPTH.get(), 0);
    }

    #[test]
    fn a_change_that_fails_still_closes_the_system_and_says_its_own_failure() {
        let s = StandIn::failing("ro-error", "apt-get", "E: Broken packages");
        set_up(&s);
        let done = around(|| run(&["apt-get", "-q", "-y", "install", "thing"], None));
        assert_eq!(done, Err("E: Broken packages".to_string()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "apt-get -q -y install thing", RO_USR, RO_OPT]);
    }

    #[test]
    fn a_change_that_panics_still_closes_the_system() {
        let s = StandIn::all_well("ro-panic");
        set_up(&s);
        let ended = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| around(|| panic!("made-up fault"))));
        assert!(ended.is_err());
        assert_eq!(s.ran(), [RW_USR, RW_OPT, RO_USR, RO_OPT]);
        assert_eq!(DEPTH.get(), 0);
        assert_eq!(state(&s), "");
    }

    /// The helper itself ends at once on a panic, with no drop: its hook
    /// closes the system when the record says this process opened it.
    #[test]
    fn the_panic_hook_closes_the_system_only_for_the_process_that_opened_it() {
        let s = StandIn::all_well("ro-hook");
        set_up(&s);
        s.write(STATE, b"1");
        after_panic();
        assert!(s.ran().is_empty(), "another process's opening is not this one's to close");
        s.write(STATE, std::process::id().to_string().as_bytes());
        after_panic();
        assert_eq!(s.ran(), [RO_USR, RO_OPT]);
    }

    #[test]
    fn a_close_that_fails_is_tried_again_and_then_said() {
        let s = StandIn::failing("ro-busy", "mount -o remount,bind,ro /usr", "mount point is busy");
        set_up(&s);
        let done = around(|| Ok(()));
        let Err(said) = done else { panic!("a system left open is never called done") };
        assert!(said.contains("/usr: mount point is busy"), "{said}");
        assert_eq!(s.ran().iter().filter(|c| *c == RO_USR).count(), 5);
        assert!(s.ran().contains(&RO_OPT.to_string()), "the other place is closed whatever became of the first");
        assert_eq!(state(&s), std::process::id().to_string(), "left on record for the next look");
    }

    #[test]
    fn a_change_that_removes_the_mark_leaves_the_system_open_for_good() {
        let s = StandIn::all_well("ro-mark-gone");
        set_up(&s);
        let done = around(|| std::fs::remove_file(s.path(MARKER)).map_err(|e| e.to_string()));
        assert_eq!(done, Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT], "no program is left to unlock it again");
        assert_eq!(state(&s), "");
    }

    #[test]
    fn nothing_is_opened_where_the_read_only_system_is_not_set_up() {
        let s = StandIn::all_well("ro-absent");
        std::fs::create_dir_all(s.path("/usr")).unwrap();
        assert_eq!(around(|| run(&["apt-get", "-q", "-y", "install", "thing"], None)), Ok(()));
        assert_eq!(lock(&[]), Ok(()));
        assert_eq!(s.ran(), ["apt-get -q -y install thing"]);
        assert!(!s.path(STATE).exists());
    }

    #[test]
    fn an_opening_inside_another_opens_and_closes_nothing() {
        let s = StandIn::all_well("ro-nested");
        set_up(&s);
        let outer = open().unwrap();
        let inner = open().unwrap();
        assert_eq!(run(&["dpkg", "--configure", "-a"], None), Ok(()));
        assert_eq!(inner.close(), Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "dpkg --configure -a"], "the inner close leaves it open");
        drop(outer);
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "dpkg --configure -a", RO_USR, RO_OPT]);
    }

    #[test]
    fn a_second_helper_is_refused_while_one_has_the_system_open() {
        let s = StandIn::all_well("ro-second");
        set_up(&s);
        // Another helper's hold, by its own opening of the file.
        let other = take().ok().expect("the first hold");
        assert_eq!(around(|| Ok(())).err().as_deref(), Some(t!("admin-system-busy")));
        assert_eq!(lock(&[]), Ok(()), "the watchdog leaves a change at work alone");
        assert!(s.ran().is_empty(), "{:?}", s.ran());
        drop(other);
        assert_eq!(lock(&[]), Ok(()));
        assert_eq!(s.ran(), [RO_USR, RO_OPT]);
    }

    #[test]
    fn a_place_not_yet_mounted_on_itself_is_bound_when_closed_and_left_when_opened() {
        let s = StandIn::all_well("ro-unbound");
        set_up(&s);
        s.write(MOUNTS, b"30 1 0:26 /@rootfs / rw,noatime shared:1 - btrfs /dev/vda3 rw\n");
        std::fs::remove_dir(s.path("/opt")).unwrap();
        assert_eq!(around(|| Ok(())), Ok(()));
        assert_eq!(s.ran(), ["mount --bind /usr /usr", RO_USR], "no /opt here, and a link is never one");
    }

    #[test]
    fn usr_local_stays_writable_over_a_locked_usr() {
        let s = StandIn::all_well("ro-local");
        set_up(&s);
        std::fs::create_dir_all(s.path(LOCAL)).unwrap();
        // Not yet a mount of its own: made one, writable, once /usr is locked.
        assert_eq!(around(|| Ok(())), Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, RO_USR, BIND_LOCAL, RW_LOCAL, RO_OPT]);
        // Already one, writable: left as it is.
        let mut table = std::fs::read_to_string(s.path(MOUNTS)).unwrap();
        table.push_str("62 60 0:26 /@rootfs/usr/local /usr/local rw,noatime shared:1 - btrfs /dev/vda3 rw\n");
        s.write(MOUNTS, table.as_bytes());
        assert_eq!(lock(&[]), Ok(()));
        assert_eq!(s.ran()[6..], [RO_USR, RO_OPT]);
        // One left read-only is made writable again.
        s.write(MOUNTS, table.replace("/usr/local rw,", "/usr/local ro,").as_bytes());
        assert_eq!(lock(&[]), Ok(()));
        assert_eq!(s.ran()[8..], [RO_USR, RW_LOCAL, RO_OPT]);
    }

    #[test]
    fn a_usr_bound_afresh_has_usr_local_bound_again_on_top() {
        let s = StandIn::all_well("ro-local-fresh");
        set_up(&s);
        std::fs::create_dir_all(s.path(LOCAL)).unwrap();
        // At a start: nothing bound yet but the old /usr/local, which a new
        // /usr would hide.
        s.write(MOUNTS, format!("{ROOT_BTRFS}62 30 0:26 /@rootfs/usr/local /usr/local rw - btrfs /dev/vda3 rw\n").as_bytes());
        assert_eq!(lock(&[]), Ok(()));
        assert_eq!(s.ran(), ["mount --bind /usr /usr", RO_USR, BIND_LOCAL, RW_LOCAL, "mount --bind /opt /opt", RO_OPT]);
    }

    #[test]
    fn usr_local_that_cannot_be_kept_writable_is_said_once_usr_is_locked() {
        let s = StandIn::failing("ro-local-fail", "mount --bind /usr/local", "no room");
        set_up(&s);
        std::fs::create_dir_all(s.path(LOCAL)).unwrap();
        let said = around(|| Ok(())).unwrap_err();
        assert!(said.contains("no room") && said.contains("/usr/local"), "{said}");
        assert!(s.ran().contains(&RO_OPT.to_string()));
        assert_eq!(state(&s), "", "the system itself is locked");
    }

    #[test]
    fn dpkg_by_hand_opens_without_holding_and_its_hook_after_closes() {
        let s = StandIn::all_well("ro-dpkg");
        set_up(&s);
        assert_eq!(before_dpkg(), Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT]);
        assert!(take().is_ok(), "nothing is held once dpkg's hook has ended");
        assert_eq!(close_unless_held(false), Ok(()));
        assert_eq!(s.ran()[2..], [RO_USR, RO_OPT]);
    }

    #[test]
    fn dpkgs_hooks_leave_a_helpers_opening_to_the_helper() {
        let s = StandIn::all_well("ro-dpkg-helper");
        set_up(&s);
        // The helper's hold, by its own opening of the file.
        let helper = take().ok().expect("the helper's hold");
        assert_eq!(before_dpkg(), Ok(()));
        assert_eq!(close_unless_held(false), Ok(()));
        assert!(s.ran().is_empty(), "{:?}", s.ran());
        drop(helper);
    }

    #[test]
    fn dpkgs_hooks_do_nothing_where_the_lock_is_off() {
        let s = StandIn::all_well("ro-dpkg-off");
        std::fs::create_dir_all(s.path("/usr")).unwrap();
        assert_eq!(before_dpkg(), Ok(()));
        assert_eq!(close_unless_held(false), Ok(()));
        assert!(s.ran().is_empty());
    }

    #[test]
    fn system_run_opens_runs_and_closes_whatever_the_program_does() {
        let s = StandIn::all_well("ro-run");
        set_up(&s);
        let args = |a: &[&str]| a.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(run_open(&args(&["--", "cp", "new", "/usr/bin/trindle"])), Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "cp new /usr/bin/trindle", RO_USR, RO_OPT]);
        drop(s);
        let f = StandIn::failing("ro-run-fail", "cp", "cp: no room");
        set_up(&f);
        assert_eq!(run_open(&args(&["--", "cp", "a", "b"])), Err("cp: no room".into()));
        assert_eq!(f.ran(), [RW_USR, RW_OPT, "cp a b", RO_USR, RO_OPT]);
        for wrong in [&[][..], &["cp", "a", "b"], &["--"]] {
            assert!(run_open(&args(wrong)).unwrap_err().starts_with("usage:"));
        }
        assert_eq!(f.ran().len(), 5, "nothing opened for a command not understood");
    }

    #[test]
    fn only_root_itself_may_open_outside_a_change_never_through_pkexec() {
        assert_eq!(root_only(0, None), Ok(()));
        assert!(root_only(0, Some("1000".into())).is_err(), "pkexec runs as root but says who asked");
        assert!(root_only(1000, None).is_err());
    }

    #[test]
    fn the_lock_switches_on_only_on_btrfs() {
        let s = StandIn::all_well("ro-on-ext4");
        std::fs::create_dir_all(s.path("/usr")).unwrap();
        s.write(MOUNTS, b"30 1 8:2 / / rw,relatime shared:1 - ext4 /dev/sda2 rw\n");
        assert_eq!(switch(&["on".into()]).err().as_deref(), Some(t!("admin-system-needs-btrfs")));
        assert!(s.ran().is_empty());
        assert!(!s.path(MARKER).exists());
        assert!(root_is_btrfs(ROOT_BTRFS));
        assert!(!root_is_btrfs(""));
    }

    #[test]
    fn switching_on_marks_starts_the_watchdog_and_locks_now() {
        let s = StandIn::all_well("ro-on");
        set_up(&s);
        std::fs::remove_file(s.path(MARKER)).unwrap();
        assert_eq!(switch(&["on".into()]), Ok(()));
        assert!(s.path(MARKER).is_file());
        assert_eq!(s.ran(), ["systemctl enable trindle-read-only.service", "systemctl enable --now trindle-read-only.timer",
                             RO_USR, RO_OPT]);
        assert!(switch(&["sideways".into()]).is_err());
    }

    #[test]
    fn switching_off_leaves_the_system_open_and_takes_the_mark_away() {
        let s = StandIn::all_well("ro-off");
        set_up(&s);
        assert_eq!(switch(&["off".into()]), Ok(()));
        assert_eq!(s.ran(), [RW_USR, RW_OPT, "systemctl disable --now trindle-read-only.timer trindle-read-only.service"]);
        assert!(!s.path(MARKER).exists());
        assert_eq!(state(&s), "");
        assert_eq!(DEPTH.get(), 0);
        assert!(take().is_ok(), "nothing held after");
        drop(s);
        // Should the units not go, it stays locked as it was.
        let f = StandIn::failing("ro-off-fail", "systemctl", "Failed to disable");
        set_up(&f);
        assert!(switch(&["off".into()]).is_err());
        assert_eq!(f.ran(), [RW_USR, RW_OPT, "systemctl disable --now trindle-read-only.timer trindle-read-only.service", RO_USR, RO_OPT]);
        assert!(f.path(MARKER).exists());
    }

    #[test]
    fn only_commands_that_write_under_usr_open_it_themselves() {
        assert!(writes_system(&["apt-get", "-q", "-y", "install", "x"]));
        assert!(writes_system(&["dpkg", "-i", "x.deb"]));
        assert!(writes_system(&["locale-gen"]));
        assert!(!writes_system(&["apt-get", "-q", "update"]));
        assert!(!writes_system(&["apt-get", "-s", "-q", "upgrade"]));
        assert!(!writes_system(&["snapper", "-c", "root", "create"]));
    }

    /// Nothing outside the helper can open the system: the words that open
    /// it are in this file alone, the module is the helper's own, it is
    /// reached only from the helper, and the helper has no command that
    /// opens it and does nothing else.
    #[test]
    fn nothing_outside_the_helper_can_open_the_system() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let opening = ["remount,bind", ",rw"].concat();
        let mut files = Vec::new();
        walk(&src, &mut files);
        let mut found = Vec::new();
        for f in &files {
            let text = std::fs::read_to_string(f).unwrap_or_default();
            let name = f.strip_prefix(&src).unwrap().to_string_lossy().into_owned();
            if name == "admin/read_only.rs" {
                continue;
            }
            if text.contains(&opening) {
                found.push(name.clone());
            }
            if (text.contains("read_only::open") || text.contains("read_only::around")) && name != "admin/mod.rs" {
                found.push(name);
            }
        }
        assert!(found.is_empty(), "the system is opened from {found:?}");
        let admin = std::fs::read_to_string(src.join("admin/mod.rs")).unwrap();
        assert!(admin.contains("\nmod read_only;"), "the module is the helper's alone");
        assert!(!admin.contains("\"system-unlock\"") && !admin.contains("\"system-open\""));
        for verb in CHANGES {
            assert!(admin.contains(&format!("\"{verb}\" =>")), "{verb} is one of the helper's commands");
        }
        // The two that open outside a change are root's alone.
        let here = std::fs::read_to_string(src.join("admin/read_only.rs")).unwrap();
        for opener in ["pub fn dpkg_pre_invoke", "pub fn system_run"] {
            let body = here.split(opener).nth(1).unwrap_or_else(|| panic!("{opener} is gone"));
            assert!(body.split("\n}").next().unwrap().contains("by_root()?;"), "{opener} is for root alone");
        }
    }

    fn walk(dir: &std::path::Path, into: &mut Vec<std::path::PathBuf>) {
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() { walk(&p, into) } else if p.extension().is_some_and(|x| x == "rs") { into.push(p) }
        }
    }

    /// The start-up lock runs before sysinit.target, and a timer comes before
    /// the service it starts. A timer with the default ordering after
    /// sysinit.target closes a loop, and systemd breaks it by dropping the
    /// start-up lock.
    #[test]
    fn the_start_up_lock_and_its_timer_make_no_loop() {
        let unit = include_str!("../../system/systemd/trindle-read-only.service");
        let timer = include_str!("../../system/systemd/trindle-read-only.timer");
        let set = |file: &str, key: &str| -> Vec<String> {
            file.lines().filter_map(|l| l.strip_prefix(key)?.strip_prefix('='))
                .flat_map(|v| v.split_whitespace().map(str::to_string)).collect()
        };
        assert!(set(unit, "Before").contains(&"sysinit.target".to_string()));
        assert_eq!(set(unit, "DefaultDependencies"), ["no"]);
        assert_eq!(set(timer, "DefaultDependencies"), ["no"], "the timer would be ordered after sysinit.target");
        assert!(!set(timer, "After").iter().any(|u| u == "sysinit.target" || u == "basic.target"));
        assert!(set(timer, "Conflicts").contains(&"shutdown.target".to_string()));
    }
}
