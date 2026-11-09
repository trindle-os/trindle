//! Typing, read from the console the kernel has already set up, so the
//! keyboard layout chosen when the system was installed is the one that works.

use std::io::{BufRead, Read, Write};
use std::os::unix::io::AsRawFd;

pub enum Key {
    Char(char),
    Backspace,
    Enter,

    Clear,

    Show,
    Restart,
    Off,
}

pub struct Tty {
    file: std::fs::File,
    was: libc::termios,
}

impl Tty {
    pub fn raw() -> Option<Tty> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
            .or_else(|_| std::fs::OpenOptions::new().read(true).write(true).open("/dev/console"))
            .ok()?;
        let fd = file.as_raw_fd();
        // SAFETY: `file` keeps the descriptor open; isatty reads nothing of ours.
        if unsafe { libc::isatty(fd) } != 1 {
            return None;
        }
        // SAFETY: termios is plain numbers, so all-zeroes is a valid starting value.
        let mut was: libc::termios = unsafe { std::mem::zeroed() };
        // SAFETY: the kernel fills in that local struct, through the same descriptor.
        if unsafe { libc::tcgetattr(fd, &mut was) } != 0 {
            return None;
        }
        let mut now = was;
        now.c_lflag &= !(libc::ICANON | libc::ECHO);
        now.c_cc[libc::VMIN] = 1;
        now.c_cc[libc::VTIME] = 0;
        // SAFETY: the settings just read back, with two flags cleared.
        if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &now) } != 0 {
            return None;
        }

        let _ = (&file).write_all(b"\x1b[?25l");
        Some(Tty { file, was })
    }

    pub fn fd(&self) -> std::os::unix::io::RawFd {
        self.file.as_raw_fd()
    }

    pub fn next(&mut self) -> Option<Key> {
        let mut bytes = Vec::new();
        loop {
            let mut one = [0u8; 1];
            if self.file.read(&mut one).ok()? == 0 {
                return None;
            }
            match one[0] {
                b'\r' | b'\n' => return Some(Key::Enter),
                0x7f | 0x08 => return Some(Key::Backspace),
                0x15 => return Some(Key::Clear),
                b'\t' => return Some(Key::Show),

                0x1b => {
                    let mut rest = [0u8; 8];
                    let n = self.file.read(&mut rest).unwrap_or(0);
                    let seq = &rest[..n];
                    return Some(match seq {
                        b"OP" | b"[11~" | b"[[A" => Key::Restart,
                        b"OQ" | b"[12~" | b"[[B" => Key::Off,
                        _ => Key::Clear,
                    });
                }
                b => bytes.push(b),
            }

            if let Ok(text) = std::str::from_utf8(&bytes) {
                if let Some(c) = text.chars().next() {
                    return Some(Key::Char(c));
                }
            }
            if bytes.len() > 4 {
                bytes.clear();
            }
        }
    }
}

impl Drop for Tty {
    fn drop(&mut self) {
        // SAFETY: the settings saved when this Tty was made, put back through

        unsafe { libc::tcsetattr(self.file.as_raw_fd(), libc::TCSANOW, &self.was) };
        let _ = self.file.write_all(b"\x1b[?25h");
    }
}

pub fn ask_plainly(prompt: &str, pipe: &mut Option<crate::fifo::PassFifo>) -> Option<Vec<u8>> {
    let mut err = std::io::stderr();
    let _ = write!(err, "{prompt}: ");
    let _ = err.flush();
    let quiet = Tty::raw();
    let mut line = String::new();
    let got = if quiet.is_some() {
        let mut typed = String::new();
        let mut tty = quiet?;
        loop {
            if let Some(key) = wait(&tty, pipe) {
                let _ = writeln!(err);
                return Some(key);
            }
            match tty.next()? {
                Key::Enter => break,
                Key::Backspace => {
                    typed.pop();
                }
                Key::Clear => typed.clear(),
                Key::Show | Key::Restart | Key::Off => {}
                Key::Char(c) => typed.push(c),
            }
        }
        typed
    } else {
        std::io::stdin().lock().read_line(&mut line).ok()?;
        line.trim_end_matches('\n').to_string()
    };
    let _ = writeln!(err);
    Some(got.into_bytes())
}

fn wait(tty: &Tty, pipe: &mut Option<crate::fifo::PassFifo>) -> Option<Vec<u8>> {
    let p = pipe.as_mut()?;
    let mut waiting = [
        libc::pollfd { fd: tty.fd(), events: libc::POLLIN, revents: 0 },
        libc::pollfd { fd: p.fd(), events: libc::POLLIN, revents: 0 },
    ];
    loop {
        // SAFETY: two live pollfd, both descriptors held open by `tty` and `p`.
        let ready = unsafe { libc::poll(waiting.as_mut_ptr(), 2, -1) };
        if ready < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
            continue;
        }
        if ready < 0 || waiting[0].revents != 0 {
            return None;
        }
        if let Some(key) = p.take() {
            return Some(key);
        }
        waiting[1].revents = 0;
    }
}
