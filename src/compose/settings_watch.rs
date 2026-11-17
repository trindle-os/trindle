//! Hears the settings files change through inotify: each file's folder is watched, or the nearest folder there is while it is missing.

use std::collections::HashMap;
use std::ffi::CString;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

const MASK: u32 = libc::IN_CLOSE_WRITE
    | libc::IN_MOVED_TO
    | libc::IN_MOVED_FROM
    | libc::IN_CREATE
    | libc::IN_DELETE
    | libc::IN_ATTRIB
    | libc::IN_DELETE_SELF
    | libc::IN_MOVE_SELF;

const READS_MOST: usize = 64;

const PATH_MOST: usize = 4096;

pub struct SettingsWatch {
    fd: OwnedFd,
    files: Vec<PathBuf>,
    watched: HashMap<i32, PathBuf>,
}

impl SettingsWatch {

    pub fn new(files: Vec<PathBuf>) -> io::Result<SettingsWatch> {
        if files.iter().any(|f| !f.is_absolute()) {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }
        // SAFETY: no pointers; the result is checked before it is owned.
        let fd = unsafe { libc::inotify_init1(libc::IN_CLOEXEC | libc::IN_NONBLOCK) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: fd is a fresh descriptor nothing else owns.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        let mut watch = SettingsWatch { fd, files, watched: HashMap::new() };
        watch.watch_all()?;
        Ok(watch)
    }

    pub fn fd(&self) -> io::Result<OwnedFd> {
        self.fd.try_clone()
    }

    fn wanted(&self) -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = Vec::new();
        for file in &self.files {
            let Some(mut dir) = file.parent() else { continue };
            while !dir.is_dir() {
                match dir.parent() {
                    Some(up) => dir = up,
                    None => break,
                }
            }
            if !dirs.iter().any(|d| d == dir) {
                dirs.push(dir.to_path_buf());
            }
        }
        dirs
    }

    fn watch_all(&mut self) -> io::Result<()> {

        match self.try_watch_all() {
            Err(e) if matches!(e.raw_os_error(), Some(libc::ENOENT | libc::ENOTDIR)) => self.try_watch_all(),
            other => other,
        }
    }

    fn try_watch_all(&mut self) -> io::Result<()> {
        let wanted = self.wanted();
        let unwanted: Vec<i32> = self.watched.iter().filter(|(_, d)| !wanted.contains(d)).map(|(wd, _)| *wd).collect();
        for wd in unwanted {
            self.watched.remove(&wd);
            // SAFETY: no pointers; a watch the kernel already dropped only gives EINVAL.
            unsafe { libc::inotify_rm_watch(self.fd.as_raw_fd(), wd) };
        }
        for dir in wanted {
            if dir.as_os_str().len() > PATH_MOST {
                return Err(io::Error::from_raw_os_error(libc::ENAMETOOLONG));
            }
            let name = CString::new(dir.as_os_str().as_bytes()).map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
            // SAFETY: name is a NUL-terminated path that lives through the call.
            let wd = unsafe { libc::inotify_add_watch(self.fd.as_raw_fd(), name.as_ptr(), MASK | libc::IN_ONLYDIR) };
            if wd < 0 {
                return Err(io::Error::last_os_error());
            }
            self.watched.retain(|_, d| *d != dir);
            self.watched.insert(wd, dir);
        }
        Ok(())
    }

    pub fn take_news(&mut self) -> io::Result<bool> {
        let (mut changed, mut rewatch) = (false, false);
        let mut buf = [0u8; 4096];
        for _ in 0..READS_MOST {
            // SAFETY: buf is writable for its whole length.
            let n = unsafe { libc::read(self.fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
            if n < 0 {
                let e = io::Error::last_os_error();
                match e.kind() {
                    io::ErrorKind::WouldBlock => break,
                    io::ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                }
            }
            if n == 0 {
                break;
            }
            let (c, r) = self.judge(&buf[..n as usize]);
            changed |= c;
            rewatch |= r;
        }
        if rewatch {
            self.watch_all()?;
        }
        Ok(changed)
    }

    fn judge(&mut self, mut events: &[u8]) -> (bool, bool) {
        const HEAD: usize = std::mem::size_of::<libc::inotify_event>();
        let (mut changed, mut rewatch) = (false, false);
        while events.len() >= HEAD {
            let word = |at: usize| u32::from_ne_bytes([events[at], events[at + 1], events[at + 2], events[at + 3]]);
            let (wd, mask, len) = (word(0) as i32, word(4), word(12) as usize);
            let Some(name) = events.get(HEAD..HEAD + len) else { break };
            events = &events[HEAD + len..];
            if mask & (libc::IN_Q_OVERFLOW | libc::IN_IGNORED | libc::IN_DELETE_SELF | libc::IN_MOVE_SELF) != 0 {
                if mask & libc::IN_IGNORED != 0 {
                    self.watched.remove(&wd);
                }
                changed = true;
                rewatch = true;
                continue;
            }
            let Some(dir) = self.watched.get(&wd) else { continue };
            let name = &name[..name.iter().position(|b| *b == 0).unwrap_or(name.len())];
            let path = dir.join(Path::new(std::ffi::OsStr::from_bytes(name)));
            if self.files.contains(&path) {
                changed = true;
            } else if self.files.iter().any(|f| f.starts_with(&path)) {

                changed = true;
                rewatch = true;
            }
        }
        (changed, rewatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-settings-watch-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn saved_beside(path: &Path, text: &str) {
        let beside = path.with_extension("new");
        std::fs::write(&beside, text).unwrap();
        std::fs::rename(&beside, path).unwrap();
    }

    #[test]
    fn a_settings_file_saved_beside_itself_and_moved_in_is_heard() {
        let dir = folder("moved");
        let file = dir.join("trindle.conf");
        std::fs::write(&file, "accent=#112233\n").unwrap();
        let mut watch = SettingsWatch::new(vec![file.clone()]).unwrap();
        assert!(!watch.take_news().unwrap(), "news with nothing changed");
        saved_beside(&file, "accent=#445566\n");
        assert!(watch.take_news().unwrap(), "a file moved into place was not heard");
        std::fs::write(dir.join("other.conf"), "x").unwrap();
        assert!(!watch.take_news().unwrap(), "another file in the folder was taken for a setting");
        std::fs::write(&file, "accent=#778899\n").unwrap();
        assert!(watch.take_news().unwrap(), "a file written in place was not heard");
        std::fs::remove_file(&file).unwrap();
        assert!(watch.take_news().unwrap(), "a file removed was not heard");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_settings_file_in_a_folder_not_yet_made_is_heard_when_it_comes() {
        let dir = folder("missing");
        let file = dir.join("a/b/shortcuts.xml");
        let mut watch = SettingsWatch::new(vec![file.clone()]).unwrap();
        assert!(!watch.take_news().unwrap());
        std::fs::create_dir(dir.join("a")).unwrap();
        watch.take_news().unwrap();
        std::fs::create_dir(dir.join("a/b")).unwrap();
        watch.take_news().unwrap();
        saved_beside(&file, "<keyboard/>");
        assert!(watch.take_news().unwrap(), "a file in a folder made later was not heard");
        assert!(watch.watched.values().any(|d| *d == dir.join("a/b")), "the file's own folder is not watched");
        assert_eq!(watch.watched.len(), 1, "folders above were still watched");

        std::fs::remove_dir_all(dir.join("a")).unwrap();
        assert!(watch.take_news().unwrap(), "the folder going was not heard");
        std::fs::create_dir_all(dir.join("a/b")).unwrap();
        watch.take_news().unwrap();
        saved_beside(&file, "<keyboard/>");
        assert!(watch.take_news().unwrap(), "a file in a folder made again was not heard");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_path_that_is_not_absolute_is_refused() {
        assert!(SettingsWatch::new(vec![PathBuf::from("trindle.conf")]).is_err());
    }

    #[test]
    fn events_cut_short_are_never_read_past_their_end() {
        let dir = folder("short");
        let mut watch = SettingsWatch::new(vec![dir.join("trindle.conf")]).unwrap();
        let wd = *watch.watched.keys().next().unwrap();
        let mut event = Vec::new();
        for word in [wd as u32, libc::IN_CLOSE_WRITE, 0, 64] {
            event.extend(word.to_ne_bytes());
        }
        event.extend(b"trindle.conf");
        assert_eq!(watch.judge(&event), (false, false), "a name longer than what was read was read");
        assert_eq!(watch.judge(&event[..7]), (false, false));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
