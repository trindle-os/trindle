//! The turning mark shown while the computer starts: the screen near-black
//! and the Trindle mark in the middle with its dots going round, in place of
//! a black screen, a blinking cursor and lines of start-up text.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use super::boot;
use super::fb::Screen;
use super::paint;

pub const PID_FILE: &str = "/run/trindle-spin.pid";

const LOG: &str = "/run/trindle-spin.log";

fn note(what: &str) {
    use std::io::Write;
    let up = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
    let up = up.split_whitespace().next().unwrap_or("?");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(LOG) {
        let _ = writeln!(f, "{up} {} {what}", std::process::id());
    }
}

const LONGEST: Duration = Duration::from_secs(300);

const AFTER_HANDOVER: Duration = Duration::from_secs(6);
const FRAME: Duration = Duration::from_millis(40);

const STEP_SECS: f32 = 0.9;

const REPAINT_EVERY: Duration = Duration::from_secs(2);

static STOP: AtomicBool = AtomicBool::new(false);
static REPAINT: AtomicBool = AtomicBool::new(true);

extern "C" fn on_stop(_: libc::c_int) {
    STOP.store(true, Ordering::SeqCst);
}

extern "C" fn on_carry_on(_: libc::c_int) {
    REPAINT.store(true, Ordering::SeqCst);
}

struct Console {
    vt: String,
}

impl Console {
    fn take() -> Console {
        Console { vt: active_vt() }
    }

    fn moved(&self) -> bool {
        let now = active_vt();
        !now.is_empty() && !self.vt.is_empty() && now != self.vt
    }
}

fn active_vt() -> String {
    std::fs::read_to_string("/sys/class/tty/tty0/active").map(|s| s.trim().to_string()).unwrap_or_default()
}

fn which_screen() -> String {
    std::fs::read_to_string("/proc/fb").unwrap_or_default()
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {

    Starting,

    Closing,

    Between,
}

const BETWEEN: Duration = Duration::from_secs(5);

pub fn run(kind: Kind) -> std::process::ExitCode {
    let closing = kind != Kind::Starting;
    take_over(kind);

    let logo = super::logo::Logo::firmware();
    let console = Console::take();
    let began = Instant::now();
    let mut mark = Turning { screen: None, known: String::new(), looked: Instant::now(), painted: Instant::now(), began };

    let mut handed: Option<Instant> = None;
    while !STOP.load(Ordering::SeqCst) && (kind == Kind::Closing || began.elapsed() < LONGEST)
        && (kind != Kind::Between || began.elapsed() < BETWEEN)
        && handed.is_none_or(|at| at.elapsed() < AFTER_HANDOVER) {
        if !closing && handed.is_none() && console.moved() {

            if let Some(at) = boot::seconds() {
                boot::record(at);
            }
            handed = Some(Instant::now());
            note("handed over");
        }
        mark.look_for_screen();
        mark.paint(logo.as_ref());
        std::thread::sleep(FRAME);
    }
    note("stopped");
    drop(mark);
    drop(console);
    if std::fs::read_to_string(PID_FILE).is_ok_and(|p| p.trim() == std::process::id().to_string()) {
        let _ = std::fs::remove_file(PID_FILE);
    }
    std::process::ExitCode::SUCCESS
}

fn take_over(kind: Kind) {
    // SAFETY: plain signal handlers that only set atomics.
    unsafe {
        libc::signal(libc::SIGTERM, on_stop as *const () as libc::sighandler_t);
        libc::signal(libc::SIGINT, on_stop as *const () as libc::sighandler_t);
        libc::signal(libc::SIGCONT, on_carry_on as *const () as libc::sighandler_t);

        libc::signal(libc::SIGHUP, libc::SIG_IGN);
    }

    if kind == Kind::Between {

    } else if let Some(before) = running().filter(|p| *p != std::process::id() as libc::pid_t) {
        // SAFETY: signals to a process number; nothing of ours is touched.
        unsafe {
            libc::kill(before, libc::SIGCONT);
            libc::kill(before, libc::SIGTERM);
        }
    }
    if kind != Kind::Between {
        let _ = std::fs::write(PID_FILE, format!("{}\n", std::process::id()));
    }
    note(match kind { Kind::Starting => "starting", Kind::Closing => "closing", Kind::Between => "between" });
}

struct Turning {
    screen: Option<Screen>,

    known: String,
    looked: Instant,
    painted: Instant,
    began: Instant,
}

impl Turning {

    fn look_for_screen(&mut self) {
        if self.looked.elapsed() >= Duration::from_millis(100) || self.screen.is_none() {
            let now = which_screen();
            if now != self.known || self.screen.is_none() {
                self.screen = Screen::open();
                note(&format!("screen {} {}", if self.screen.is_some() { "opened" } else { "missing" },
                              now.split_whitespace().skip(1).collect::<Vec<_>>().join(" ")));
                self.known = now;
                REPAINT.store(true, Ordering::SeqCst);
            }
            self.looked = Instant::now();
        }
    }

    fn paint(&mut self, logo: Option<&super::logo::Logo>) {
        if self.painted.elapsed() >= REPAINT_EVERY {
            REPAINT.store(true, Ordering::SeqCst);
        }
        if let Some(s) = self.screen.as_mut() {
            let turn = self.began.elapsed().as_secs_f32() / STEP_SECS * std::f32::consts::PI / 4.0;
            let whole = REPAINT.swap(false, Ordering::SeqCst);
            paint::turning(s, turn, whole, logo);
            if whole {
                self.painted = Instant::now();
            }
        }
    }
}

fn running() -> Option<libc::pid_t> {
    std::fs::read_to_string(PID_FILE).ok()?.trim().parse().ok().filter(|p: &libc::pid_t| *p > 1)
}

pub fn stop() -> std::process::ExitCode {
    if let Some(pid) = running() {
        // SAFETY: signals to a process number; nothing of ours is touched.
        unsafe {
            libc::kill(pid, libc::SIGCONT);
            libc::kill(pid, libc::SIGTERM);
        }
        for _ in 0..20 {
            // SAFETY: signal 0 only asks whether the process is still there.
            if unsafe { libc::kill(pid, 0) } != 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    std::process::ExitCode::SUCCESS
}

pub struct Paused(Option<libc::pid_t>);

pub fn pause() -> Paused {
    let pid = running();
    if let Some(pid) = pid {
        // SAFETY: as in stop().
        unsafe { libc::kill(pid, libc::SIGSTOP) };
    }
    Paused(pid)
}

impl Drop for Paused {
    fn drop(&mut self) {
        if let Some(pid) = self.0 {
            // SAFETY: as in stop().
            unsafe { libc::kill(pid, libc::SIGCONT) };
        }
    }
}
