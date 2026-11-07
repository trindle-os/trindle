//! The kept ring's pulse, written by the ring and read by the compositor.

use std::io::{ErrorKind, Write as _};
use std::os::fd::AsRawFd;

pub const SILENT_AFTER: u64 = 4000;
pub const PULSE_EVERY: u64 = 1000;

pub fn now() -> u64 {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: clock_gettime only writes into the timespec it is handed.
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) };
    (t.tv_sec as u64) * 1000 + (t.tv_nsec as u64) / 1_000_000
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrote {
    Sent,
    Full,
    Gone,
}

pub fn write(pipe: &mut std::fs::File, now: u64) -> Wrote {
    loop {
        match pipe.write(&now.to_le_bytes()) {
            Ok(_) => return Wrote::Sent,
            Err(e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) if e.kind() == ErrorKind::WouldBlock => return Wrote::Full,
            Err(_) => return Wrote::Gone,
        }
    }
}

fn brim(pipe: &std::fs::File) -> bool {
    let mut waiting: libc::c_int = 0;
    // SAFETY: FIONREAD only writes a count into the int it is handed, and
    // F_GETPIPE_SZ only reads the size of a pipe this holds open.
    let (asked, size) = unsafe {
        (libc::ioctl(pipe.as_raw_fd(), libc::FIONREAD, &mut waiting), libc::fcntl(pipe.as_raw_fd(), libc::F_GETPIPE_SZ))
    };
    asked == 0 && size > 0 && waiting + 8 > size
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Looked {
    pub heard: Option<u64>,
    pub closed: bool,
    pub full: bool,
}

pub fn read(pipe: &mut std::fs::File, heard: Option<u64>, now: u64) -> Looked {
    let full = brim(pipe);
    let (heard, closed) = drain(pipe, heard);
    Looked { heard: if full { Some(now) } else { heard }, closed, full }
}

const MOST_READ: usize = 128 * 1024;

fn drain(pipe: &mut impl std::io::Read, heard: Option<u64>) -> (Option<u64>, bool) {
    let (mut heard, mut closed, mut read) = (heard, false, 0);
    let mut bytes = [0u8; 4096];
    while read < MOST_READ {
        match pipe.read(&mut bytes) {
            Ok(0) => {
                closed = true;
                break;
            }
            Ok(n) => {
                heard = newest(&bytes[..n]).or(heard);
                read += n;
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => {}
            Err(_) => break,
        }
    }
    (heard, closed)
}

pub fn newest(bytes: &[u8]) -> Option<u64> {
    let whole = bytes.len() / 8 * 8;
    let last: [u8; 8] = bytes.get(whole.checked_sub(8)?..whole)?.try_into().ok()?;
    Some(u64::from_le_bytes(last))
}

pub fn silent(last: Option<u64>, now: u64) -> bool {
    last.is_some_and(|last| now.saturating_sub(last) > SILENT_AFTER)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipe() -> (std::fs::File, std::fs::File) {
        use std::os::fd::FromRawFd;
        let mut ends = [0i32; 2];
        // SAFETY: pipe2 writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC | libc::O_NONBLOCK) }, 0);
        // SAFETY: both numbers were just made by pipe2 and belong to nothing else.
        unsafe { (std::fs::File::from_raw_fd(ends[0]), std::fs::File::from_raw_fd(ends[1])) }
    }

    #[test]
    fn a_ring_is_silent_only_once_heard_from_and_then_not_for_too_long() {
        assert!(!silent(None, 1_000_000));
        assert!(!silent(Some(10_000), 10_000 + SILENT_AFTER));
        assert!(silent(Some(10_000), 10_001 + SILENT_AFTER));
        assert!(!silent(Some(10_050), 10_000));
    }

    #[test]
    fn the_newest_whole_pulse_is_read_and_a_part_of_one_is_left() {
        assert_eq!(newest(&[]), None);
        assert_eq!(newest(&[1, 2, 3]), None);
        let mut bytes = 5u64.to_le_bytes().to_vec();
        bytes.extend(9u64.to_le_bytes());
        assert_eq!(newest(&bytes), Some(9));
        bytes.extend([1, 2, 3]);
        assert_eq!(newest(&bytes), Some(9));
    }

    #[test]
    fn a_pipe_filled_to_the_brim_is_never_taken_for_silence_and_the_ring_never_waits_on_it() {
        let (mut read_end, mut write_end) = pipe();
        let (mut clock, mut sent, mut last_sent) = (1_000_000u64, 0, 0);
        let since = std::time::Instant::now();
        for _ in 0..3 * 3600 {
            clock += PULSE_EVERY;
            match write(&mut write_end, clock) {
                Wrote::Sent => {
                    sent += 1;
                    last_sent = clock;
                }
                Wrote::Full => {}
                Wrote::Gone => panic!("a full pipe was taken for one nobody reads"),
            }
        }
        assert!(since.elapsed() < std::time::Duration::from_secs(5), "the ring waited on a full pipe");
        assert!(sent > 0 && sent < 3 * 3600, "the pipe never filled: {sent} pulses went in");
        assert!(clock - last_sent > SILENT_AFTER, "the newest pulse in the pipe is not an old one, so this shows nothing");
        let now = clock + 500;
        let looked = read(&mut read_end, None, now);
        assert!(looked.full && !looked.closed);
        let heard = looked.heard;
        assert_eq!(heard, Some(now), "the newest pulse in a full pipe was taken as the ring's last word");
        assert!(!silent(heard, now));
        assert_eq!(write(&mut write_end, now + 500), Wrote::Sent);
        let looked = read(&mut read_end, heard, now + 600);
        assert_eq!(looked, Looked { heard: Some(now + 500), closed: false, full: false });
        let looked = read(&mut read_end, looked.heard, now + 500 + SILENT_AFTER + 1);
        assert!(silent(looked.heard, now + 500 + SILENT_AFTER + 1));
    }

    #[test]
    fn a_ring_that_writes_without_end_does_not_keep_the_compositor_reading() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let (mut read_end, write_end) = pipe();
        let (stop, writing) = (std::sync::Arc::new(AtomicBool::new(false)), std::sync::Arc::new(AtomicBool::new(true)));
        let flood: Vec<_> = (0..4).map(|_| {
            let (s, w, mut end) = (stop.clone(), writing.clone(), write_end.try_clone().unwrap());
            std::thread::spawn(move || {
                let pulses: Vec<u8> = (0..8192u64).flat_map(|n| n.to_le_bytes()).collect();
                let since = std::time::Instant::now();
                while !s.load(Ordering::SeqCst) && since.elapsed() < std::time::Duration::from_secs(3) {
                    let _ = end.write(&pulses);
                }
                w.store(false, Ordering::SeqCst);
            })
        }).collect();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let looked = read(&mut read_end, None, 5);
        let still_writing = writing.load(Ordering::SeqCst);
        stop.store(true, Ordering::SeqCst);
        for thread in flood {
            thread.join().unwrap();
        }
        assert!(still_writing, "the look at the pipe ended only when the ring stopped writing");
        assert!(looked.heard.is_some() && !looked.closed);
        struct Endless(u64);
        impl std::io::Read for Endless {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                let whole = bytes.len() / 8 * 8;
                for pulse in bytes[..whole].chunks_mut(8) {
                    self.0 += 1;
                    pulse.copy_from_slice(&self.0.to_le_bytes());
                }
                Ok(whole)
            }
        }
        let mut endless = Endless(0);
        let (heard, closed) = drain(&mut endless, None);
        assert_eq!((heard, closed), (Some(endless.0), false));
        assert_eq!(endless.0 as usize * 8, MOST_READ);
    }

    #[test]
    fn a_pipe_nobody_reads_ends_the_pulsing_and_one_the_ring_closed_is_known() {
        let (read_end, mut write_end) = pipe();
        assert_eq!(write(&mut write_end, 5), Wrote::Sent);
        drop(read_end);
        let mut wrote = write(&mut write_end, 6);
        for _ in 0..2000 {
            if wrote == Wrote::Gone {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            wrote = write(&mut write_end, 6);
        }
        assert_eq!(wrote, Wrote::Gone);
        let (mut read_end, mut write_end) = pipe();
        assert_eq!(read(&mut read_end, None, 100), Looked { heard: None, closed: false, full: false });
        assert_eq!(write(&mut write_end, 7), Wrote::Sent);
        drop(write_end);
        let mut looked = read(&mut read_end, None, 100);
        for _ in 0..2000 {
            if looked.closed {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            looked = read(&mut read_end, looked.heard, 100);
        }
        assert_eq!(looked, Looked { heard: Some(7), closed: true, full: false });
        assert_eq!(read(&mut read_end, Some(7), 200), Looked { heard: Some(7), closed: true, full: false });
    }

    #[test]
    fn the_clock_only_runs_forward() {
        let first = now();
        assert!(first > 0 && now() >= first);
    }
}
