//! The pipe Debian's own askpass offers, so something other than a person can answer.

use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;

const PATH: &str = "/lib/cryptsetup/passfifo";
const CLEVIS: &str = "/usr/bin/clevis";
const TRIED: &str = "/run/trindle-unlock-chip-tried";
const CHIP_WAIT: std::time::Duration = std::time::Duration::from_secs(10);

pub struct PassFifo {
    file: std::fs::File,
}

impl PassFifo {
    pub fn open() -> Option<PassFifo> {
        let path = std::ffi::CString::new(PATH).ok()?;
        // SAFETY: a valid C string; mkfifo reads nothing else of ours.
        if unsafe { libc::mkfifo(path.as_ptr(), 0o600) } != 0
            && std::io::Error::last_os_error().kind() != std::io::ErrorKind::AlreadyExists
        {
            return None;
        }
        let file = std::fs::OpenOptions::new().read(true).write(true)
            .custom_flags(libc::O_NONBLOCK).open(PATH).ok()?;
        Some(PassFifo { file })
    }

    pub fn fd(&self) -> std::os::unix::io::RawFd {
        self.file.as_raw_fd()
    }

    pub fn wait_for_chip(&mut self) -> Option<Vec<u8>> {
        if !std::path::Path::new(CLEVIS).exists() || std::path::Path::new(TRIED).exists() {
            return None;
        }
        let _ = std::fs::write(TRIED, b"");
        let until = std::time::Instant::now() + CHIP_WAIT;
        loop {
            let left = until.saturating_duration_since(std::time::Instant::now());
            if left.is_zero() {
                return None;
            }
            let mut waiting = libc::pollfd { fd: self.fd(), events: libc::POLLIN, revents: 0 };
            let ms = i32::try_from(left.as_millis()).unwrap_or(i32::MAX);
            // SAFETY: one pollfd that lives across the call.
            if unsafe { libc::poll(&mut waiting, 1, ms) } > 0 {
                if let Some(key) = self.take() {
                    return Some(key);
                }
            }
        }
    }

    pub fn take(&mut self) -> Option<Vec<u8>> {
        let mut key = Vec::new();
        let mut chunk = [0u8; 512];
        loop {
            match self.file.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => key.extend_from_slice(&chunk[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => break,
            }
        }
        (!key.is_empty()).then_some(key)
    }
}

impl Drop for PassFifo {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(PATH);
    }
}
