//! Stopping an update that is running. The program doing the work is kept
//! here, at the head of a process group of its own (`run::start_group`),
//! and one press of Stop ends it, whole group included: SIGTERM first, so
//! it can put down what it holds, then SIGKILL after a few seconds if it
//! is still there. A second press, or a press after it has ended, does
//! nothing. What was fetched so far is left to the program that fetched
//! it, as it would be after a Ctrl-C. Only jobs that run as the person can
//! be stopped this way: a program started through pkexec runs as root, and
//! the person's own account may not signal it.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::run::Proc;

pub const GRACE: Duration = Duration::from_secs(5);

#[derive(Default)]
struct Inner {
    proc: Option<Proc>,
    pressed: bool,
}

#[derive(Clone, Default)]
pub struct Stop(Arc<Mutex<Inner>>);

impl Stop {
    fn inner(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn hold(&self, proc: Proc) {
        self.inner().proc = Some(proc);
    }

    pub fn release(&self) {
        self.inner().proc = None;
    }

    pub fn offered(&self) -> bool {
        let i = self.inner();
        i.proc.as_ref().is_some_and(Proc::running) && !i.pressed
    }

    pub fn pressed(&self) -> bool {
        self.inner().pressed
    }

    pub fn press(&self) -> bool {
        self.press_after(GRACE)
    }

    fn press_after(&self, grace: Duration) -> bool {
        let mut i = self.inner();
        let Some(proc) = i.proc.clone().filter(|p| p.running() && !i.pressed) else { return false };
        i.pressed = true;
        drop(i);
        proc.end(grace);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};

    fn job(script: &str) -> (std::process::Child, Proc, i32) {
        let mut command = Command::new("sh");
        command.args(["-c", script]).stdout(Stdio::piped());
        let (mut child, proc) = super::super::run::start_group(&mut command).unwrap();
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap()).read_line(&mut line).unwrap();
        (child, proc, line.trim().parse().unwrap())
    }

    fn ended(pid: i32) -> bool {
        match std::fs::read_to_string(format!("/proc/{pid}/stat")) {
            Ok(stat) => stat.rsplit_once(')').is_some_and(|(_, rest)| rest.trim_start().starts_with('Z')),
            Err(_) => true,
        }
    }

    fn gone_soon(pid: i32) -> bool {
        (0..100).any(|_| {
            std::thread::sleep(Duration::from_millis(50));
            ended(pid)
        })
    }

    #[test]
    fn one_press_ends_the_job_and_what_it_started_and_a_second_does_nothing() {
        use std::os::unix::process::ExitStatusExt;
        let (mut child, proc, helper) = job("sleep 300 & echo $!; wait");
        let stop = Stop::default();
        assert!(!stop.offered() && !stop.press(), "nothing to stop before a job is held");
        stop.hold(proc.clone());
        assert!(stop.offered() && !stop.pressed());
        assert!(stop.press(), "the first press ends it");
        assert!(stop.pressed() && !stop.offered(), "and Stop is no longer offered");
        assert!(!stop.press(), "a second press does nothing");
        let status = super::super::run::collect(&mut child, &proc).unwrap();
        assert_eq!(status.signal(), Some(libc::SIGTERM), "asked, not killed");
        assert!(gone_soon(helper), "what the job started goes with it");
        stop.release();
        assert!(!stop.offered() && !stop.press(), "and nothing is signalled once it has been collected");
        assert!(stop.pressed(), "how it ended is still known to be a stop");
    }

    #[test]
    fn a_job_that_ignores_sigterm_is_killed_after_the_grace() {
        use std::os::unix::process::ExitStatusExt;
        let (mut child, proc, helper) = job("trap '' TERM; sleep 300 & echo $!; wait");
        let stop = Stop::default();
        stop.hold(proc.clone());
        assert!(stop.press_after(Duration::from_millis(300)));
        let status = super::super::run::collect(&mut child, &proc).unwrap();
        assert_eq!(status.signal(), Some(libc::SIGKILL));
        assert!(gone_soon(helper));
    }

    #[test]
    fn a_job_that_ended_by_itself_is_never_signalled() {
        let (mut child, proc, _) = job("echo 1");
        super::super::run::collect(&mut child, &proc).unwrap();
        let stop = Stop::default();
        stop.hold(proc);
        assert!(!stop.offered() && !stop.press_after(Duration::from_millis(10)) && !stop.pressed());
    }
}
