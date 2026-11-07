//! Non-blocking process spawning.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use zeroize::Zeroizing;

pub use crate::toolkit::common::run::*;

pub const MAX_ANSWER: &str = "16m";
pub const MAX_CATALOGUE: &str = "128m";
pub const MAX_DOWNLOAD: &str = "4g";

pub fn spawn(argv: &[&str]) -> bool {
    let program = argv.first().and_then(|p| match p.contains('/') {
        true => Some(std::path::PathBuf::from(p)).filter(|p| p.is_file()),
        false => find(p),
    });
    let Some(program) = program else {
        eprintln!("trindle: could not run {}: not found", argv.first().unwrap_or(&""));
        return false;
    };
    let args: Vec<String> = argv[1..].iter().map(|a| a.to_string()).collect();
    let start = move |token: Option<String>| {
        let mut command = std::process::Command::new(&program);
        command.args(&args).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
        if let Some(token) = token {
            command.env("XDG_ACTIVATION_TOKEN", token);
        }
        match command.spawn() {
            Ok(mut child) => drop(std::thread::spawn(move || child.wait())),
            Err(e) => eprintln!("trindle: could not run {}: {e}", program.display()),
        }
    };
    vouched(start);
    true
}

pub fn line_by_line<'a>(argv: &[&'a str]) -> Vec<&'a str> {
    let mut out = vec!["stdbuf", "-oL"];
    out.extend_from_slice(argv);
    out
}

#[derive(Clone)]
pub struct Proc {
    pid: libc::pid_t,
    group: bool,
    gone: Arc<Mutex<bool>>,
}

impl Proc {
    pub fn force_exit(&self) {
        if let Ok(gone) = self.gone.lock() {
            if !*gone {
                // SAFETY: the program is not yet collected, so this number
                // is still its own.
                unsafe { libc::kill(self.pid, libc::SIGKILL) };
            }
        }
    }
}

impl Proc {
    pub fn signal(&self, signal: libc::c_int) {
        if let Ok(gone) = self.gone.lock() {
            if !*gone {
                let target = if self.group { -self.pid } else { self.pid };
                // SAFETY: as in force_exit; a group's number is its head's.
                unsafe { libc::kill(target, signal) };
            }
        }
    }

    pub fn running(&self) -> bool {
        self.gone.lock().is_ok_and(|gone| !*gone)
    }

    pub fn end(&self, grace: Duration) {
        self.signal(libc::SIGTERM);
        let me = self.clone();
        std::thread::spawn(move || {
            std::thread::sleep(grace);
            me.signal(libc::SIGKILL);
        });
    }
}

pub fn start_group(command: &mut Command) -> std::io::Result<(std::process::Child, Proc)> {
    let child = command.process_group(0).spawn()?;
    let proc = Proc { pid: child.id() as libc::pid_t, group: true, gone: Arc::new(Mutex::new(false)) };
    Ok((child, proc))
}

pub fn collect(child: &mut std::process::Child, proc: &Proc) -> Option<std::process::ExitStatus> {
    settled(child, &proc.gone)
}

pub fn child(mut command: Command, ended: impl FnOnce(i32) + 'static) -> Option<Proc> {
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("trindle: could not run {}: {e}", command.get_program().to_string_lossy());
            return None;
        }
    };
    let proc = Proc { pid: child.id() as libc::pid_t, group: false, gone: Arc::new(Mutex::new(false)) };
    let gone = proc.gone.clone();
    on_thread(move || {
        use std::os::unix::process::ExitStatusExt;
        settled(&mut child, &gone).map(|s| s.into_raw()).unwrap_or(0)
    }, ended);
    Some(proc)
}

enum Heard {
    Line(String),
    Ended(bool),
}

pub fn stream(argv: &[&str], on_line: impl Fn(&str) + 'static, done: impl FnOnce(bool) + 'static) {
    stream_stoppable(argv, on_line, done);
}

pub fn stream_stoppable(argv: &[&str], on_line: impl Fn(&str) + 'static,
                        done: impl FnOnce(bool) + 'static) -> Option<Proc> {
    streamed(argv, false, on_line, done)
}

pub fn stream_group(argv: &[&str], on_line: impl Fn(&str) + 'static, done: impl FnOnce(bool) + 'static) -> Option<Proc> {
    streamed(argv, true, on_line, done)
}

fn streamed(argv: &[&str], group: bool, on_line: impl Fn(&str) + 'static, done: impl FnOnce(bool) + 'static) -> Option<Proc> {
    let mut done = Some(done);
    let id = wait_for(Box::new(move |v| match v.downcast::<Heard>().map(|h| *h) {
        Ok(Heard::Line(line)) => {
            on_line(&line);
            true
        }
        Ok(Heard::Ended(ok)) => {
            if let Some(done) = done.take() {
                done(ok);
            }
            false
        }
        Err(_) => false,
    }));
    let Some((mut child, out)) = merged(argv, group) else {
        answer(id, Box::new(Heard::Ended(false)));
        return None;
    };
    let proc = Proc { pid: child.id() as libc::pid_t, group, gone: Arc::new(Mutex::new(false)) };
    let gone = proc.gone.clone();
    std::thread::spawn(move || {
        let mut lines = BufReader::new(std::fs::File::from(out));
        let mut line = Vec::new();
        while matches!(lines.read_until(b'\n', &mut line), Ok(n) if n > 0) {
            let text = String::from_utf8_lossy(&line).trim_end_matches(['\n', '\r']).to_string();
            if !text.is_empty() {
                answer(id, Box::new(Heard::Line(text)));
            }
            line.clear();
        }
        let ok = settle(&mut child, &gone);
        answer(id, Box::new(Heard::Ended(ok)));
    });
    Some(proc)
}

fn merged(argv: &[&str], group: bool) -> Option<(std::process::Child, OwnedFd)> {
    asks_polkit(argv.first()?);
    let (read, write) = pipe()?;
    let also = write.try_clone().ok()?;
    let mut command = Command::new(argv.first()?);
    command.args(&argv[1..]).stdin(Stdio::null()).stdout(Stdio::from(write)).stderr(Stdio::from(also));
    if group {
        command.process_group(0);
    }
    let child = command.spawn().ok()?;
    Some((child, read))
}

fn settle(child: &mut std::process::Child, gone: &Mutex<bool>) -> bool {
    settled(child, gone).is_some_and(|s| s.success())
}

fn settled(child: &mut std::process::Child, gone: &Mutex<bool>) -> Option<std::process::ExitStatus> {
    // SAFETY: an all-zero siginfo_t is a valid one for waitid to fill in.
    let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
    // SAFETY: waits on this child alone, writing only into `info`.
    unsafe { libc::waitid(libc::P_PID, child.id(), &mut info, libc::WEXITED | libc::WNOWAIT) };
    let mut gone = gone.lock().unwrap_or_else(|e| e.into_inner());
    *gone = true;
    child.wait().ok()
}

pub fn capture<F>(argv: &[&str], stdin: Option<&str>, done: F)
where
    F: FnOnce(bool, String, String) + 'static,
{
    let argv: Vec<String> = argv.iter().map(|a| a.to_string()).collect();
    let input = stdin.map(|s| s.to_string());
    on_thread(move || captured(&argv, input), move |(ok, out, err)| done(ok, out, err));
}

pub fn captured(argv: &[String], stdin: Option<String>) -> (bool, String, String) {
    run_fed(argv, stdin.map(|s| Zeroizing::new(s.into_bytes())))
}

pub fn captured_secret(argv: &[&str], secret: Zeroizing<String>) -> (bool, String, String) {
    let argv: Vec<String> = argv.iter().map(|a| a.to_string()).collect();
    let mut line = Zeroizing::new(Vec::with_capacity(secret.len() + 1));
    line.extend_from_slice(secret.as_bytes());
    line.push(b'\n');
    run_fed(&argv, Some(line))
}

fn run_fed(argv: &[String], stdin: Option<Zeroizing<Vec<u8>>>) -> (bool, String, String) {
    let Some(program) = argv.first() else { return (false, String::new(), String::new()) };
    asks_polkit(program);
    let child = Command::new(program).args(&argv[1..])
        .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => return (false, String::new(), e.to_string()),
    };
    if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
        std::thread::spawn(move || {
            let _ = pipe.write_all(&text);
        });
    }
    let mut err = String::new();
    let complaints = child.stderr.take().map(|mut e| std::thread::spawn(move || {
        let mut got = Vec::new();
        let _ = e.read_to_end(&mut got);
        String::from_utf8_lossy(&got).to_string()
    }));
    let mut got = Vec::new();
    if let Some(mut o) = child.stdout.take() {
        let _ = o.read_to_end(&mut got);
    }
    if let Some(Ok(e)) = complaints.map(|t| t.join()) {
        err = e;
    }
    let ok = child.wait().map(|s| s.success()).unwrap_or(false);
    (ok, String::from_utf8_lossy(&got).to_string(), err)
}

pub fn text(argv: &[&str]) -> String {
    std::process::Command::new(argv[0]).args(&argv[1..]).output().ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
}

pub fn have(program: &str) -> bool {
    find(program).is_some()
}

pub fn find(program: &str) -> Option<std::path::PathBuf> {
    std::env::var_os("PATH")
        .and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(program)).find(|p| p.is_file()))
}

pub fn admin_helper() -> String {
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(3) {
            let p = dir.join("libexec").join("trindle-admin");
            if p.is_file() {
                return p.to_string_lossy().to_string();
            }
        }
    }
    "/usr/lib/trindle/libexec/trindle-admin".to_string()
}

#[cfg(test)]
pub const NO_PASSWORD: &str = "Not done: no administrator's password was given.";

fn asks_polkit(program: &str) {
    if program == "pkexec" || program.ends_with("/pkexec") {
        super::refused::prime(&[super::refused::ADMIN_ACTION]);
    }
}

pub fn admin_error(err: &str) -> String {
    use super::refused;
    let e = err.trim();
    if e.is_empty() {
        return crate::toolkit::t!("common-refused-no-reason").into();
    }
    if let Some(heard) = refused::heard_pkexec(e) {
        return refused::not_done(heard.settled(|| refused::would_ask(&[refused::ADMIN_ACTION])));
    }
    e.lines().last().unwrap_or(e).to_string()
}

pub fn admin_step(cmdline: &[u8]) -> Option<String> {
    let mut words = cmdline.split(|b| *b == 0).map(|w| String::from_utf8_lossy(w).into_owned());
    let (program, helper, step) = (words.next()?, words.next()?, words.next()?);
    let pkexec = program.rsplit('/').next() == Some("pkexec");
    (pkexec && helper.ends_with("/libexec/trindle-admin") && !step.is_empty()).then_some(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn captured_keeps_output_and_complaints_apart() {
        let (ok, out, err) = captured(&argv(&["sh", "-c", "echo hi; echo no >&2; exit 3"]), None);
        assert!(!ok);
        assert_eq!((out.as_str(), err.as_str()), ("hi\n", "no\n"));
        let (ok, out, _) = captured(&argv(&["cat"]), Some("fed in".into()));
        assert!(ok);
        assert_eq!(out, "fed in");
        assert!(!captured(&argv(&["no-such-program-here"]), None).0);
    }

    #[test]
    fn a_step_that_failed_without_a_word_is_not_a_closed_password_box() {
        for silent in ["", "  \n"] {
            let said = admin_error(silent);
            assert_ne!(said, NO_PASSWORD, "{silent:?}");
            assert_eq!(crate::common::refused::of_words(&said), None, "no refusal is read into it");
            assert!(said.starts_with("Not done") && said.ends_with('.'), "{said}");
        }
        assert_eq!(admin_error("Error executing command as another user: Request dismissed"), NO_PASSWORD);
    }

    #[test]
    fn merged_mixes_both_and_settles() {
        let (mut child, out) = merged(&["sh", "-c", "echo one; echo two >&2"], false).unwrap();
        let mut text = String::new();
        std::fs::File::from(out).read_to_string(&mut text).unwrap();
        assert_eq!(text, "one\ntwo\n");
        let gone = Mutex::new(false);
        assert!(settle(&mut child, &gone));
        assert!(*gone.lock().unwrap());
    }

    #[test]
    fn a_child_is_watched_to_its_end() {
        use std::os::unix::process::ExitStatusExt;
        for (script, crashed) in [("kill -SEGV $$", true), ("true", false)] {
            let mut child = Command::new("sh").args(["-c", script]).spawn().unwrap();
            let status = settled(&mut child, &Mutex::new(false)).map(|s| s.into_raw()).unwrap_or(0);
            assert_eq!(crate::common::crash::is_fault(status), crashed, "{script}");
        }
    }
}
