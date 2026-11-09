//! A box's own Xwayland, held for as long as the box runs.

use crate::toolkit::t;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::common::x11_rooms::{self, Answer};

pub(super) fn x11_socket(n: u32) -> String {
    format!("/tmp/.X11-unix/X{n}")
}

pub(super) struct XHold {
    stream: std::os::unix::net::UnixStream,
    told: std::sync::Arc<std::sync::Mutex<Option<x11_rooms::Told>>>,
    reader: Option<std::thread::JoinHandle<()>>,
}

impl XHold {
    pub(super) fn new(stream: std::os::unix::net::UnixStream) -> XHold {
        use std::io::BufRead;
        let told = std::sync::Arc::new(std::sync::Mutex::new(None));
        let reader = stream.try_clone().ok().map(|s| {
            let told = told.clone();
            std::thread::spawn(move || {
                for line in std::io::BufReader::new(s).lines() {
                    let Ok(line) = line else { break };
                    let Some(now) = x11_rooms::Told::parse(&line) else { continue };
                    let mut was = told.lock().unwrap_or_else(|e| e.into_inner());

                    if *was != Some(x11_rooms::Told::Failed) {
                        *was = Some(now);
                    }
                }
            })
        });
        XHold { stream, told, reader }
    }

    pub(super) fn told(&self, code: i32) -> Option<x11_rooms::Told> {
        let end = std::time::Instant::now() + X_WORD_WAIT;
        loop {
            let now = *self.told.lock().unwrap_or_else(|e| e.into_inner());
            if now.is_some() || code == 0 || std::time::Instant::now() >= end {
                return now;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

impl Drop for XHold {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
        if let Some(r) = self.reader.take() {
            let _ = r.join();
        }
    }
}

pub(super) const X_WORD_WAIT: std::time::Duration = std::time::Duration::from_secs(1);

#[derive(Debug, PartialEq, Eq)]
pub(super) enum XNext {
    Done,
    Again,
    Say,
}

pub(super) fn x_next(told: Option<x11_rooms::Told>, tries: u32) -> XNext {
    match told {
        None => XNext::Done,
        Some(x11_rooms::Told::Lost) if tries < 2 => XNext::Again,
        Some(_) => XNext::Say,
    }
}

pub(super) fn tell_would_not_start(app: &str) {
    tell(x11_rooms::would_not_start(), &t!("store-boxed-x-could-not-open", app = app));
}

pub(super) fn own_x11(name: &str) -> (Option<u32>, Option<XHold>) {
    match x11_rooms::ask(name) {
        Ok((held, Answer::Display(n))) => (Some(n), Some(XHold::new(held))),
        Ok((_, Answer::Off)) => {
            tell(t!("store-boxed-x-off"), t!("store-boxed-x-off-where"));
            (None, None)
        }
        Ok((_, answer)) => {
            eprintln!("trindle-boxed: {name}: no Xwayland of its own ({answer:?})");
            tell(t!("store-boxed-x-many"), t!("store-boxed-x-many-close"));
            (None, None)
        }
        Err(e) => {
            let desktop = std::env::var("DISPLAY").ok().and_then(|d| d.strip_prefix(':')?.split('.').next()?.parse().ok());
            eprintln!("trindle-boxed: {name}: {e}; the desktop's own display is used until the next login");
            (desktop.filter(|n| Path::new(&x11_socket(*n)).exists()), None)
        }
    }
}

pub(crate) fn tell(summary: &str, body: &str) {
    let _ = Command::new("busctl").args(["--user", "call", "org.freedesktop.Notifications", "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications", "Notify", "susssasa{sv}i", "Apps", "0", "dialog-warning-symbolic", summary, body, "0", "0", "-1"])
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
}

pub(super) fn run_own_x11(app: &str, command: &[String]) -> i32 {
    use std::os::unix::process::{CommandExt, ExitStatusExt};
    let made = || {
        let mut c = Command::new(&command[0]);
        c.args(&command[1..]);
        c
    };
    let permissions = crate::common::run::text(&["flatpak", "info", "--show-permissions", app]);
    if !crate::common::access::flatpak_uses_x11(&permissions) {
        let e = made().exec();
        eprintln!("trindle-boxed: could not start {}: {e}", command[0]);
        return 1;
    }
    let mut tries = 0;
    loop {
        tries += 1;
        let mut c = made();
        let (display, held) = own_x11(app);
        match display {
            Some(n) => c.env("DISPLAY", format!(":{n}")),
            None => c.env_remove("DISPLAY"),
        };
        let Some(held) = held else {
            let e = c.exec();
            eprintln!("trindle-boxed: could not start {}: {e}", command[0]);
            return 1;
        };
        let code = match c.spawn().and_then(|mut child| child.wait()) {
            Ok(s) => s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0)),
            Err(e) => {
                eprintln!("trindle-boxed: could not start {}: {e}", command[0]);
                1
            }
        };
        let next = x_next(held.told(code), tries);
        drop(held);
        match next {
            XNext::Done => return code,
            XNext::Again => eprintln!("trindle-boxed: {app}: its Xwayland ended under it; starting it again"),
            XNext::Say => {
                tell_would_not_start(app);
                return code;
            }
        }
    }
}
