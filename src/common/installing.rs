//! Whether Trindle is being installed on this computer right now, from the installer's own lock.

use std::os::fd::AsRawFd;
use std::path::Path;

pub const INSTALL_LOCK: &str = "/run/trindle-install.lock";

pub fn installing() -> bool {
    installing_at(Path::new(INSTALL_LOCK))
}

fn installing_at(path: &Path) -> bool {
    let Ok(file) = std::fs::File::open(path) else { return false };
    // SAFETY: file owns a valid descriptor for both calls.
    let taken = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } != 0;
    if !taken {
        // SAFETY: as above; the shared lock just taken is let go.
        unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
    }
    taken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_installers_lock_says_whether_it_is_installing() {
        let dir = std::env::temp_dir().join(format!("trindle-installing-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("lock");
        assert!(!installing_at(&path), "no lock file");
        let held = std::fs::File::create(&path).unwrap();
        assert!(!installing_at(&path), "a file nobody holds");
        // SAFETY: held owns a valid descriptor.
        assert_eq!(unsafe { libc::flock(held.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) }, 0);
        assert!(installing_at(&path), "held by the installer");
        drop(held);
        assert!(!installing_at(&path), "let go when the installer ends");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
