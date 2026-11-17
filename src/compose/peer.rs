//! Who is at the other end of a socket, and process handles that stay one process.

use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd};

const SO_PEERPIDFD: libc::c_int = 77;

pub fn credentials(socket: &impl AsFd) -> Option<libc::ucred> {
    let mut cred = libc::ucred { pid: 0, uid: u32::MAX, gid: u32::MAX };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: the fd is an open socket for the call, and cred and len are valid, with len holding cred's size.
    let rc = unsafe {
        libc::getsockopt(socket.as_fd().as_raw_fd(), libc::SOL_SOCKET, libc::SO_PEERCRED, (&mut cred as *mut libc::ucred).cast(), &mut len)
    };
    (rc == 0 && len as usize == std::mem::size_of::<libc::ucred>()).then_some(cred)
}

pub fn pidfd_of_peer(socket: &impl AsFd) -> Option<OwnedFd> {
    let mut pidfd: libc::c_int = -1;
    let mut len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: `pidfd` and `len` are live and sized for an int.
    let rc = unsafe { libc::getsockopt(socket.as_fd().as_raw_fd(), libc::SOL_SOCKET, SO_PEERPIDFD, (&mut pidfd as *mut libc::c_int).cast(), &mut len) };
    // SAFETY: a pidfd the kernel just gave us, owned by nothing else.
    (rc == 0 && pidfd >= 0).then(|| unsafe { OwnedFd::from_raw_fd(pidfd) })
}

pub fn pidfd_open(pid: u32) -> Option<OwnedFd> {
    // SAFETY: pidfd_open takes a number and flags and returns a new descriptor or -1.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::pid_t, 0) };
    // SAFETY: a descriptor the kernel just gave us, owned by nothing else.
    (fd >= 0).then(|| unsafe { OwnedFd::from_raw_fd(fd as libc::c_int) })
}

pub fn signal(pidfd: &OwnedFd, signal: libc::c_int) -> bool {
    // SAFETY: no pointers are read; the descriptor is open for the call.
    unsafe { libc::syscall(libc::SYS_pidfd_send_signal, pidfd.as_raw_fd(), signal, std::ptr::null::<libc::siginfo_t>(), 0) == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixStream;
    use std::process::Command;

    #[test]
    fn the_far_end_of_a_socket_is_named_by_the_kernel() {
        let (a, _b) = UnixStream::pair().unwrap();
        let cred = credentials(&a).unwrap();
        assert_eq!(cred.pid, std::process::id() as i32);
        let pidfd = pidfd_of_peer(&a).unwrap();
        assert!(signal(&pidfd, 0));
    }

    #[test]
    fn a_handle_ends_the_process_it_was_opened_on_and_no_other() {
        let mut child = Command::new("sleep").arg("30").spawn().unwrap();
        let handle = pidfd_open(child.id()).unwrap();
        assert!(signal(&handle, libc::SIGKILL));
        child.wait().unwrap();

        assert!(!signal(&handle, libc::SIGKILL));
    }
}
