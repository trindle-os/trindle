//! How the ring's program ended, and whether it left for an update.

use smithay::reexports::calloop::channel::Sender;

use super::Told;

pub(super) fn watch(pid: u32, number: u64, told: Sender<Told>) {
    std::thread::spawn(move || {
        // SAFETY: waitid only writes into the siginfo it is handed, which is
        // zeroed and the right size; WNOWAIT leaves the child uncollected.
        unsafe {
            let mut info: libc::siginfo_t = std::mem::zeroed();
            while libc::waitid(libc::P_PID, pid, &mut info, libc::WEXITED | libc::WNOWAIT) == -1
                && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
            {}
        }
        let _ = told.send(Told::Ended(number));
    });
}

pub(super) fn ending(status: std::process::ExitStatus) -> String {
    use std::os::unix::process::ExitStatusExt;
    match (status.signal(), status.code()) {
        (Some(signal), _) => format!("the ring was ended by signal {signal}"),
        (None, Some(0)) => "the ring left of its own accord".to_string(),
        (None, Some(code)) => format!("the ring ended with a fault (code {code})"),
        (None, None) => "the ring ended".to_string(),
    }
}

pub(super) type Stamp = (u64, u64, i64, i64);

pub(super) fn stamp(program: &std::path::Path) -> Option<Stamp> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::metadata(program).ok()?;
    Some((m.dev(), m.ino(), m.ctime(), m.ctime_nsec()))
}

pub(super) fn left_for_update(status: std::process::ExitStatus, was: Option<Stamp>, is: Option<Stamp>) -> bool {
    status.code() == Some(0) && was.is_some() && is.is_some() && was != is
}

pub(super) fn still_running(pid: u32) -> bool {
    // SAFETY: waitid only writes into the siginfo it is handed, which is
    // zeroed and the right size; WNOHANG returns at once and WNOWAIT leaves
    // the child uncollected. It fails for a number that is no child of ours.
    unsafe {
        let mut info: libc::siginfo_t = std::mem::zeroed();
        libc::waitid(libc::P_PID, pid, &mut info, libc::WEXITED | libc::WNOHANG | libc::WNOWAIT) == 0 && info.si_pid() == 0
    }
}

pub(super) fn ended(pid: u32) -> bool {
    // SAFETY: waitid only writes into the siginfo it is handed, which is
    // zeroed and the right size; WNOHANG returns at once and WNOWAIT leaves
    // the child uncollected. si_pid stays 0 when nothing has ended.
    unsafe {
        let mut info: libc::siginfo_t = std::mem::zeroed();
        libc::waitid(libc::P_PID, pid, &mut info, libc::WEXITED | libc::WNOHANG | libc::WNOWAIT) == 0 && info.si_pid() != 0
    }
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    use smithay::reexports::calloop::channel::{channel, Event};

    use super::*;

    const N: u64 = 3;

    #[test]
    fn each_ending_is_worded_as_what_it_is() {
        let ends = |script: &str| ending(Command::new("/bin/sh").arg("-c").arg(script).stdin(Stdio::null()).status().unwrap());
        assert_eq!(ends("kill -KILL $$"), "the ring was ended by signal 9");
        assert_eq!(ends("kill -TERM $$"), "the ring was ended by signal 15");
        assert_eq!(ends("exit 0"), "the ring left of its own accord");
        assert_eq!(ends("exit 3"), "the ring ended with a fault (code 3)");
    }

    #[test]
    fn only_a_ring_that_left_of_its_own_accord_with_a_new_file_in_its_place_left_for_an_update() {
        let ends = |script: &str| Command::new("/bin/sh").arg("-c").arg(script).stdin(Stdio::null()).status().unwrap();
        let (old, new): (Stamp, Stamp) = ((5, 100, 1000, 0), (5, 101, 1090, 0));
        assert!(left_for_update(ends("exit 0"), Some(old), Some(new)));
        assert!(left_for_update(ends("exit 0"), Some(old), Some((5, 100, 1090, 0))));
        assert!(!left_for_update(ends("exit 0"), Some(old), Some(old)));
        assert!(!left_for_update(ends("exit 3"), Some(old), Some(new)));
        assert!(!left_for_update(ends("kill -TERM $$"), Some(old), Some(new)));
        assert!(!left_for_update(ends("exit 0"), None, Some(new)));
        assert!(!left_for_update(ends("exit 0"), Some(old), None));
        let dir = std::env::temp_dir().join(format!("trindle-own-ring-stamp-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (program, beside) = (dir.join("trindle"), dir.join("trindle.dpkg-new"));
        std::fs::write(&program, "one").unwrap();
        let was = stamp(&program);
        assert!(was.is_some() && was == stamp(&program));
        std::fs::write(&beside, "two").unwrap();
        std::fs::rename(&beside, &program).unwrap();
        assert_ne!(stamp(&program), was);
        assert_eq!(stamp(&beside), None);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_program_is_watched_until_it_ends_and_left_to_be_collected() {
        let mut child = Command::new("/bin/sh").arg("-c").arg("exit 7").stdin(Stdio::null()).spawn().unwrap();
        let (told, heard) = channel::<Told>();
        watch(child.id(), N, told);
        let mut events = smithay::reexports::calloop::EventLoop::<Option<Told>>::try_new().unwrap();
        events.handle().insert_source(heard, |e, _, got| if let Event::Msg(t) = e { *got = Some(t) }).unwrap();
        let mut got = None;
        let since = Instant::now();
        while got.is_none() && since.elapsed() < Duration::from_secs(10) {
            events.dispatch(Duration::from_millis(100), &mut got).unwrap();
        }
        assert_eq!(got, Some(Told::Ended(N)));
        assert!(ended(child.id()));
        let mut running = Command::new("/bin/sh").arg("-c").arg("read line").stdin(Stdio::piped()).spawn().unwrap();
        assert!(!ended(running.id()), "one still running has not ended");
        drop(running.stdin.take());
        running.wait().unwrap();
        assert_eq!(child.wait().unwrap().code(), Some(7));
    }
}
