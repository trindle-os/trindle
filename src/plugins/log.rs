//! What a plugin's background part prints, kept where its author and the
//! person can read it: `~/.local/state/trindle/plugins/ID.log`, with the
//! one before it beside it as `ID.log.1`. A plugin is not trusted, so the
//! log is capped: it can never fill the disk, and it is written by
//! Trindle, outside the plugin's box, never by the plugin itself.

use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub const MOST: u64 = 256 * 1024;

pub fn path(id: &str) -> PathBuf {
    crate::common::files::home().join(".local/state/trindle/plugins").join(format!("{id}.log"))
}

pub fn forget(id: &str) {
    let at = path(id);
    let mut old = at.as_os_str().to_owned();
    old.push(".1");
    let _ = std::fs::remove_file(&at);
    let _ = std::fs::remove_file(old);
}

pub struct Kept {
    pump: std::thread::JoinHandle<()>,
}

fn open(at: &Path) -> Option<File> {
    std::fs::OpenOptions::new().append(true).create(true).mode(0o600).custom_flags(libc::O_NOFOLLOW).open(at).ok()
}

fn add(at: &Path, file: Option<File>, bytes: &[u8], most: u64) -> Option<File> {
    let mut file = file.or_else(|| open(at))?;
    let size = file.metadata().map_or(0, |m| m.len());
    if size + bytes.len() as u64 > most {
        drop(file);
        let mut old = at.as_os_str().to_owned();
        old.push(".1");
        let _ = std::fs::rename(at, &old);
        file = open(at)?;
    }

    let cut = bytes.len().saturating_sub(most as usize);
    let _ = file.write_all(&bytes[cut..]);
    Some(file)
}

pub fn keep(at: &Path) -> Option<Kept> {
    std::fs::create_dir_all(at.parent()?).ok()?;
    let first = open(at)?;
    let mut ends = [0; 2];
    // SAFETY: pipe2 fills the two-element array it is given and nothing else.
    if unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return None;
    }
    // SAFETY: both descriptors were just made by pipe2 and are owned by

    let (mut read, write) = unsafe { (File::from_raw_fd(ends[0]), File::from_raw_fd(ends[1])) };
    // SAFETY: dup2 only makes descriptors 1 and 2 name the pipe's writing

    let joined = unsafe { libc::dup2(write.as_raw_fd(), 1) >= 0 && libc::dup2(write.as_raw_fd(), 2) >= 0 };
    drop(write);
    if !joined {
        return None;
    }
    let at = at.to_path_buf();
    let pump = std::thread::spawn(move || {
        let (mut file, mut buf) = (Some(first), [0u8; 8192]);
        loop {
            match read.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => file = add(&at, file, &buf[..n], MOST),
            }
        }
    });
    Some(Kept { pump })
}

impl Kept {

    pub fn finish(self) {
        // SAFETY: descriptors 1 and 2 are this program's own, pointed at

        unsafe {
            libc::close(1);
            libc::close(2);
        }
        let _ = self.pump.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_log_is_capped_keeps_one_before_it_and_follows_no_link() {
        let dir = std::env::temp_dir().join(format!("trindle-plugin-log-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let at = dir.join("org.example.One.log");
        let mut file = None;
        for _ in 0..10 {
            file = add(&at, file, &[b'a'; 40], 100);
        }
        drop(file);
        let old = dir.join("org.example.One.log.1");
        assert!(std::fs::metadata(&at).unwrap().len() <= 100 && std::fs::metadata(&old).unwrap().len() <= 100);
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2, "never more than the log and the one before it");

        let big: Vec<u8> = (0..250u32).map(|i| (i % 250) as u8).collect();
        drop(add(&at, None, &big, 100));
        assert_eq!(std::fs::read(&at).unwrap(), &big[150..]);

        let other = dir.join("elsewhere");
        std::fs::write(&other, "mine").unwrap();
        std::fs::remove_file(&at).unwrap();
        std::os::unix::fs::symlink(&other, &at).unwrap();
        assert!(add(&at, None, b"x", 100).is_none());
        assert_eq!(std::fs::read(&other).unwrap(), b"mine");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
