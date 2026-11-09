//! Plain event lines for the screen reader, if one is listening; lengths and times only, never what is typed.

use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::path::PathBuf;

#[derive(Default)]
pub(super) struct Reader {
    sock: Option<OwnedFd>,
    path: Option<PathBuf>,
    entry: usize,
}

impl Reader {
    pub(super) fn new() -> Self {
        Reader::at(std::env::var_os("XDG_RUNTIME_DIR").map(|d| PathBuf::from(d).join("trindle-screen-reader/events.sock")))
    }

    fn at(path: Option<PathBuf>) -> Self {
        Reader { sock: None, path, entry: 0 }
    }

    pub(super) fn locked(&mut self) {
        self.send("locked");
    }

    pub(super) fn time(&mut self, hour: i32, minute: i32) {
        self.send(&format!("time {hour:02}:{minute:02}"));
    }

    pub(super) fn entry(&mut self, length: usize) {
        if length != self.entry {
            self.entry = length;
            self.send(&format!("entry {length}"));
        }
    }

    pub(super) fn wrong(&mut self) {
        self.entry = 0;
        self.send("wrong");
    }

    pub(super) fn unlocked(&mut self) {
        self.send("unlocked");
    }

    fn send(&mut self, line: &str) {
        if self.sock.is_none() {
            self.sock = self.path.as_deref().and_then(connect);
        }
        let Some(fd) = &self.sock else { return };
        let text = format!("{line}\n");
        // SAFETY: the buffer is the whole of `text`; MSG_NOSIGNAL turns a gone reader into an error, not a signal.
        let sent = unsafe { libc::send(fd.as_raw_fd(), text.as_ptr().cast(), text.len(), libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT) };
        if sent < 0 {
            self.sock = None;
        }
    }
}

fn connect(path: &std::path::Path) -> Option<OwnedFd> {
    use std::os::unix::ffi::OsStrExt;
    let bytes = path.as_os_str().as_bytes();
    // SAFETY: sockaddr_un is plain numbers; all zero is a valid start.
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    if bytes.len() >= addr.sun_path.len() {
        return None;
    }
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
    for (to, from) in addr.sun_path.iter_mut().zip(bytes) {
        *to = *from as libc::c_char;
    }
    // SAFETY: a new descriptor, owned at once.
    let fd = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_STREAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return None;
    }
    // SAFETY: fd was just made and is ours alone.
    let fd = unsafe { OwnedFd::from_raw_fd(fd) };
    let len = (std::mem::size_of::<libc::sa_family_t>() + bytes.len() + 1) as libc::socklen_t;
    // SAFETY: addr is a whole sockaddr_un and len lies within it.
    let r = unsafe { libc::connect(fd.as_raw_fd(), (&addr as *const libc::sockaddr_un).cast(), len) };
    (r == 0).then_some(fd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::os::unix::net::UnixListener;

    fn lines(mut got: impl Read) -> Vec<String> {
        let mut text = String::new();
        let _ = got.read_to_string(&mut text);
        text.lines().map(String::from).collect()
    }

    #[test]
    fn a_failed_then_a_good_password_is_told_in_order_and_never_the_text() {
        let dir = crate::common::files::private_temp("lrd").unwrap();
        let path = dir.join("e.sock");
        let listener = UnixListener::bind(&path).unwrap();
        let mut reader = Reader::at(Some(path));
        let (wrong, right) = ("hunter2-secret", "correct horse");
        reader.locked();
        reader.time(7, 3);
        for pass in [wrong, right] {
            for n in 1..=pass.chars().count() {
                reader.entry(n);
            }
            reader.entry(pass.chars().count());
            if pass == wrong {
                reader.wrong();
            }
        }
        reader.unlocked();
        drop(reader);
        let (conn, _) = listener.accept().unwrap();
        let got = lines(conn);
        let mut want = vec!["locked".to_string(), "time 07:03".into()];
        want.extend((1..=14).map(|n| format!("entry {n}")));
        want.push("wrong".into());
        want.extend((1..=13).map(|n| format!("entry {n}")));
        want.push("unlocked".into());
        assert_eq!(got, want);
        let all = got.join("\n");
        for text in [wrong, right, "hunter", "horse"] {
            assert!(!all.contains(text));
        }
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_missing_socket_is_silent_and_never_created() {
        let dir = crate::common::files::private_temp("lrd").unwrap();
        let path = dir.join("none.sock");
        let mut reader = Reader::at(Some(path.clone()));
        reader.locked();
        reader.entry(3);
        reader.wrong();
        reader.unlocked();
        assert!(!path.exists());
        Reader::at(None).locked();
        std::fs::remove_dir_all(&dir).ok();
    }
}
