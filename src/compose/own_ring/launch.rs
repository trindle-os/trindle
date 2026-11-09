//! Starting the ring or the screenshot tool on a connection of its own, and the door the start-up says "now" through.

use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::{net::UnixStream, process::CommandExt};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use smithay::reexports::calloop::channel::Sender;

use super::{Told, DOOR, NAME, PROGRAM, PULSE};

pub(super) fn start_on(
    program: &std::path::Path,
    name: &str,
    pair: io::Result<(UnixStream, UnixStream)>,
    pulse: Option<&OwnedFd>,
    hand: Option<&OwnedFd>,
    words: &[String],
    taken: impl FnOnce(UnixStream) -> io::Result<()>,
) -> io::Result<Child> {
    let (ours, theirs) = pair?;
    taken(ours)?;
    let mut cmd = Command::new(program);
    cmd.arg0(name).args(words).env("WAYLAND_SOCKET", theirs.as_raw_fd().to_string());
    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(own_words());
    hand_over(&mut cmd, theirs.as_raw_fd());
    if let Some(pulse) = pulse {
        cmd.env(PULSE, pulse.as_raw_fd().to_string());
        hand_over(&mut cmd, pulse.as_raw_fd());
    }
    match hand {
        Some(hand) => {
            cmd.env(crate::own_press::NAME, hand.as_raw_fd().to_string());
            hand_over(&mut cmd, hand.as_raw_fd());
        }

        None if name == NAME => {
            cmd.env(crate::own_press::NAME, crate::own_press::NONE);
        }
        None => {}
    }
    crate::session::usual_files(&mut cmd);

    cmd.spawn()
}

fn own_words() -> Stdio {
    use std::os::fd::AsFd;
    std::io::stdout().as_fd().try_clone_to_owned().map_or_else(|_| Stdio::null(), Stdio::from)
}

pub fn program() -> Option<PathBuf> {
    let beside = std::env::current_exe().ok()?.parent()?.join(PROGRAM);
    beside.is_file().then_some(beside)
}

fn hand_over(cmd: &mut Command, fd: RawFd) {
    // SAFETY: between fork and exec the hook only calls fcntl, which is safe

    unsafe {
        cmd.pre_exec(move || {
            if libc::fcntl(fd, libc::F_SETFD, 0) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

pub(super) fn nonblocking(end: OwnedFd) -> Option<std::fs::File> {
    // SAFETY: fcntl only sets a flag on a file this owns.
    (unsafe { libc::fcntl(end.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) } == 0).then(|| std::fs::File::from(end))
}

pub(super) fn pipe() -> Option<(OwnedFd, OwnedFd)> {
    let mut ends = [0 as RawFd; 2];
    // SAFETY: pipe2 writes two file numbers into the array it is handed.
    if unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return None;
    }
    // SAFETY: both numbers were just made by pipe2 and belong to nothing else.
    Some(unsafe { (OwnedFd::from_raw_fd(ends[0]), OwnedFd::from_raw_fd(ends[1])) })
}

#[must_use]
pub fn door(cmd: &mut Command, told: Sender<Told>) -> Option<Door> {
    let (read, write) = pipe()?;
    cmd.env(DOOR, write.as_raw_fd().to_string());
    hand_over(cmd, write.as_raw_fd());
    Some(Door { read, write, told })
}

pub struct Door {
    read: OwnedFd,
    write: OwnedFd,
    told: Sender<Told>,
}

impl Door {

    pub fn opened(self) {
        let Door { read, write, told } = self;
        drop(write);
        std::thread::spawn(move || {
            use std::io::Read;
            let mut one = [0u8; 1];

            let _ = std::fs::File::from(read).read(&mut one);
            let _ = told.send(Told::Now);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::super::SHOT;
    use super::*;

    #[test]
    fn with_no_connection_to_hand_over_no_ring_is_started_by_name_or_at_all() {
        let dir = std::env::temp_dir().join(format!("trindle-own-ring-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (mark, program) = (dir.join("started"), dir.join("trindle"));
        std::fs::write(&program, format!("#!/bin/sh\necho started > {}\n", mark.display())).unwrap();
        std::fs::set_permissions(&program, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();

        let none = start_on(&program, NAME, Err(io::Error::from_raw_os_error(libc::EMFILE)), None, None, &[], |_| Ok(()));
        assert_eq!(none.unwrap_err().raw_os_error(), Some(libc::EMFILE));

        let refused = start_on(&program, NAME, UnixStream::pair(), None, None, &[], |_| Err(io::Error::from_raw_os_error(libc::ENOMEM)));
        assert_eq!(refused.unwrap_err().raw_os_error(), Some(libc::ENOMEM));
        std::thread::sleep(Duration::from_millis(300));
        assert!(!mark.exists(), "a program was started with no connection handed to it");

        let mut child = start_on(&program, NAME, UnixStream::pair(), None, None, &[], |_| Ok(())).unwrap();
        child.wait().unwrap();
        assert!(mark.exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_ring_is_handed_its_connection_its_pulse_and_its_words() {
        let dir = std::env::temp_dir().join(format!("trindle-own-ring-words-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (out, program) = (dir.join("out"), dir.join("trindle"));
        let script = format!("#!/bin/sh\n{{ echo \"$*\"; test -e /proc/self/fd/$WAYLAND_SOCKET && echo connection; test -e /proc/self/fd/${PULSE} && echo pulse; test -e /proc/self/fd/${} && echo hand; }} > {}\n", crate::own_press::NAME, out.display());
        std::fs::write(&program, script).unwrap();
        std::fs::set_permissions(&program, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
        let (_read, write) = pipe().unwrap();
        let words = ["--notes".to_string()];
        let (hand, _hand_write) = pipe().unwrap();
        start_on(&program, SHOT, UnixStream::pair(), Some(&write), Some(&hand), &words, |_| Ok(())).unwrap().wait().unwrap();
        assert_eq!(std::fs::read_to_string(&out).unwrap(), "--notes\nconnection\npulse\nhand\n");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_ring_with_no_pipe_for_the_persons_presses_is_told_there_is_none() {
        let dir = std::env::temp_dir().join(format!("trindle-own-ring-hand-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (out, program) = (dir.join("out"), dir.join("trindle"));
        std::fs::write(&program, format!("#!/bin/sh\necho \"hand=${{{}-unsaid}}\" > {}\n", crate::own_press::NAME, out.display())).unwrap();
        std::fs::set_permissions(&program, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
        let (_read, write) = pipe().unwrap();

        for pulse in [Some(&write), None] {
            start_on(&program, NAME, UnixStream::pair(), pulse, None, &[], |_| Ok(())).unwrap().wait().unwrap();
            assert_eq!(std::fs::read_to_string(&out).unwrap(), format!("hand={}\n", crate::own_press::NONE), "the ring was not told it has no pipe to read the person's presses from");
        }

        let (hand, _hand_write) = pipe().unwrap();
        start_on(&program, NAME, UnixStream::pair(), Some(&write), Some(&hand), &[], |_| Ok(())).unwrap().wait().unwrap();
        assert_eq!(std::fs::read_to_string(&out).unwrap(), format!("hand={}\n", hand.as_raw_fd()));

        start_on(&program, SHOT, UnixStream::pair(), None, None, &[], |_| Ok(())).unwrap().wait().unwrap();
        assert_eq!(std::fs::read_to_string(&out).unwrap(), "hand=unsaid\n");

        assert!(crate::source::code(include_str!("../../ring/kept.rs")).contains("crate::common::own_press::NONE"));
        let _ = std::fs::remove_dir_all(dir);
    }

    fn said(cmd: &mut Command) -> String {
        String::from_utf8(cmd.stdin(Stdio::null()).output().unwrap().stdout).unwrap().trim().to_string()
    }

    #[test]
    fn only_the_file_handed_over_is_open_in_the_program_started() {
        let (_read, write) = pipe().unwrap();
        let (_other_read, other) = pipe().unwrap();
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg("test -e /proc/self/fd/$HANDED && echo handed; test -e /proc/self/fd/$KEPT || echo kept back");
        cmd.env("HANDED", write.as_raw_fd().to_string()).env("KEPT", other.as_raw_fd().to_string());
        hand_over(&mut cmd, write.as_raw_fd());
        assert_eq!(said(&mut cmd), "handed\nkept back");
    }

    #[test]
    fn the_door_is_heard_when_written_to_and_when_only_closed() {
        for script in ["printf 1 > /proc/self/fd/$TRINDLE_RING_DOOR", "true"] {
            let (told, heard) = std::sync::mpsc::channel::<()>();
            let (read, write) = pipe().unwrap();
            let mut cmd = Command::new("/bin/sh");
            cmd.arg("-c").arg(script).env(DOOR, write.as_raw_fd().to_string());
            hand_over(&mut cmd, write.as_raw_fd());
            let mut child = cmd.stdin(Stdio::null()).spawn().unwrap();
            drop(write);
            std::thread::spawn(move || {
                use std::io::Read;
                let _ = std::fs::File::from(read).read(&mut [0u8; 1]);
                let _ = told.send(());
            });
            assert!(heard.recv_timeout(Duration::from_secs(10)).is_ok(), "{script}: the door was never heard");
            child.wait().unwrap();
        }
    }
}
