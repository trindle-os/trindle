//! Starting the lock screen and keeping it going until it is unlocked.

use std::ffi::{c_int, c_void};
use std::os::fd::AsRawFd;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant, SystemTime};

use wayland_client::{Connection, EventQueue, QueueHandle};
use wayland_client::globals::{registry_queue_init, GlobalList};
use wayland_client::protocol::{wl_compositor, wl_output, wl_seat, wl_shm};
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
use zeroize::Zeroizing;

use crate::common::pocketkey;
use crate::toolkit::t;
use crate::toolkit::text::Fonts;

use super::{add_screen, give_surface, keyboard, unlock, CHECKED, HINT, Mode, State};
use super::input::{answer_counts, CHECK_LIMIT, CHECK_OK, CHECK_PHONE_NO, CHECK_WRONG};
use super::picture::{make_cursor, redraw};
use super::reveal::Eye;

const LONGEST_WAIT_MS: c_int = 1000;

const FLASH: Duration = Duration::from_millis(180);

pub fn main(args: Vec<String>) -> i32 {

    if crate::common::files::setting("lock") != "on" && !args.iter().any(|a| a == "--now") {
        return 0;
    }
    let Ok(conn) = Connection::connect_to_env() else {
        eprintln!("trindle-lock: no Wayland display to lock");
        return 1;
    };
    let Ok((globals, mut queue)) = registry_queue_init::<State>(&conn) else { return 1 };
    let qh = queue.handle();
    let (Ok(compositor), Ok(shm), Ok(manager)) = (
        globals.bind::<wl_compositor::WlCompositor, _, _>(&qh, 4..=6, ()),
        globals.bind::<wl_shm::WlShm, _, _>(&qh, 1..=1, ()),
        globals.bind::<ExtSessionLockManagerV1, _, _>(&qh, 1..=1, ()),
    ) else {
        eprintln!("trindle-lock: this compositor cannot lock the screen");
        return 1;
    };
    let _seat: Option<wl_seat::WlSeat> = globals.bind(&qh, 1..=7, ()).ok();
    let mut state = first_state(compositor, shm, args.iter().any(|a| a == "--resting"));
    add_outputs(&mut state, &globals, &qh);
    let _ = queue.roundtrip(&mut state);
    if let Some(code) = take_lock(&mut state, &mut queue, &qh, &manager) {
        return code;
    }
    go_to_background();
    let Some(pipe) = make_pipe() else { return 1 };
    let failed = serve(&mut state, &mut queue, &qh, pipe[0]);

    let _ = queue.roundtrip(&mut state);
    std::thread::sleep(Duration::from_millis(50));
    i32::from(failed)
}

fn first_state(compositor: wl_compositor::WlCompositor, shm: wl_shm::WlShm, resting: bool) -> State {
    let mut state = State {
        compositor, shm, lock: None, screens: Vec::new(), locked: false, refused: false,
        mode: Mode::Password, entered: Zeroizing::new(String::new()), reader: super::reader_events::Reader::new(), eye: Eye::default(), message: String::new(), wrong: false, tries: 0,
        locked_at: SystemTime::now(), checking: false, check_since: None, check_turn: 0, done: false, pointer_on: None, pointer_at: (0.0, 0.0), flash: None,
        cursor: None, xkb_ctx: keyboard::new_context(), xkb_state: std::ptr::null_mut(), minute: -1,

        resting: resting.then(|| {
            let moving = crate::session::screensaver::moving_for().unwrap_or(0.0);
            Instant::now().checked_sub(Duration::from_secs_f64(moving)).unwrap_or_else(Instant::now)
        }),
        rest_pointer: None,
        phone: false,
        due: None,
        phone_hint: None,
        fonts: Fonts::new(),
        moved: Instant::now(),
        faces: Vec::new(),
        old: Vec::new(),
    };
    (state.phone, state.due) = pocketkey::screen();
    state.mode = if state.pin_allowed() { Mode::Pin } else { Mode::Password };
    if state.resting.is_none() {
        state.phone_hint = Some(Instant::now());
    }
    state
}

fn add_outputs(state: &mut State, globals: &GlobalList, qh: &QueueHandle<State>) {
    let registry = globals.registry().clone();
    let outputs: Vec<(u32, u32)> = globals.contents().with_list(|l| {
        l.iter().filter(|g| g.interface == "wl_output").map(|g| (g.name, g.version)).collect()
    });
    for (name, version) in outputs {
        let o = registry.bind::<wl_output::WlOutput, _, _>(name, version.min(4), qh, ());
        add_screen(state, qh, o);
    }
}

fn take_lock(state: &mut State, queue: &mut EventQueue<State>, qh: &QueueHandle<State>, manager: &ExtSessionLockManagerV1) -> Option<i32> {
    state.lock = Some(manager.lock(qh, ()));
    for i in 0..state.screens.len() {
        give_surface(state, qh, i);
    }
    state.cursor = make_cursor(state, qh);

    while !state.locked && !state.refused {
        if queue.blocking_dispatch(state).is_err() {
            return Some(1);
        }
    }

    state.refused.then_some(0)
}

fn go_to_background() {
    // SAFETY: the child calls setsid and then carries on as this program, which has only one thread

    match unsafe { libc::fork() } {
        -1 => eprintln!("trindle-lock: could not go to the background; the caller waits until the screen is unlocked"),
        // SAFETY: setsid in the child, async-signal-safe, touching no memory.
        0 => unsafe {
            libc::setsid();
        },
        // SAFETY: the parent leaves at once, so no destructor runs twice.
        _ => unsafe { libc::_exit(0) },
    }
}

fn make_pipe() -> Option<[c_int; 2]> {
    let mut pipe = [0 as c_int; 2];
    // SAFETY: fills the two-element array with the pipe's descriptors.
    if unsafe { libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        eprintln!("trindle-lock: cannot make a pipe for the checks");
        return None;
    }
    CHECKED.with(|c| *c.borrow_mut() = Some((pipe[0], pipe[1])));
    Some(pipe)
}

fn serve(state: &mut State, queue: &mut EventQueue<State>, qh: &QueueHandle<State>, answers: c_int) -> bool {
    while !state.done {
        let _ = queue.flush();
        let Some(guard) = queue.prepare_read() else {
            let _ = queue.dispatch_pending(state);
            continue;
        };
        let mut fds = [
            libc::pollfd { fd: guard.connection_fd().as_raw_fd(), events: libc::POLLIN, revents: 0 },
            libc::pollfd { fd: answers, events: libc::POLLIN, revents: 0 },
        ];
        let wait = wait_millis(state);
        // SAFETY: two descriptors in a local array of exactly the length given.
        let n = unsafe { libc::poll(fds.as_mut_ptr(), 2, wait) };
        if n > 0 && fds[0].revents & libc::POLLIN != 0 {
            match guard.read() {
                Ok(_) => {}

                Err(wayland_client::backend::WaylandError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    eprintln!("trindle-lock: the connection to the compositor failed: {e}");
                    return true;
                }
            }
        } else {
            drop(guard);
        }
        if let Err(e) = queue.dispatch_pending(state) {
            eprintln!("trindle-lock: the compositor sent something that could not be handled: {e}");
            return true;
        }
        timers(state, qh);
        if n > 0 && fds[1].revents & libc::POLLIN != 0 {
            check_answered(state, qh, answers);
        }
        housekeeping(state, qh);
    }
    false
}

fn wait_millis(state: &State) -> c_int {

    let mut wait = match state.moves_every() {
        Some(every) => every.saturating_sub(state.moved.elapsed()).as_micros().div_ceil(1000) as c_int,
        None => LONGEST_WAIT_MS,
    };
    if state.flash.is_some() {
        wait = wait.min(30);
    }
    if let Some(since) = state.check_since {
        wait = wait.min(CHECK_LIMIT.saturating_sub(since.elapsed()).as_millis() as c_int + 10);
    }
    if let Some(at) = state.phone_hint {
        wait = wait.min(HINT.saturating_sub(at.elapsed()).as_millis() as c_int + 10);
    }
    if let Some(left) = state.eye.wake(Instant::now()) {
        wait = wait.min(left.as_millis().min(LONGEST_WAIT_MS as u128) as c_int + 10);
    }
    wait
}

fn timers(state: &mut State, qh: &QueueHandle<State>) {
    if state.check_since.is_some_and(|at| at.elapsed() >= CHECK_LIMIT) {
        state.abandon_check();
        redraw(state, qh);
    }
    if state.flash.is_some_and(|(_, at)| at.elapsed() >= FLASH) {
        state.flash = None;
        redraw(state, qh);
    }
    if state.phone_hint.is_some_and(|at| at.elapsed() >= HINT) {
        state.phone_hint = None;
        redraw(state, qh);
    }

    let now = Instant::now();
    let named = state.eye.named.is_some() && !state.eye.name_showing(now);
    if named {
        state.eye.named = None;
    }
    if state.eye.rest(now) || named {
        redraw(state, qh);
    }
}

fn check_answered(state: &mut State, qh: &QueueHandle<State>, answers: c_int) {
    let mut byte = [CHECK_WRONG, 0];
    // SAFETY: two bytes into a local array, from the reading end of the

    let got = unsafe { libc::read(answers, byte.as_mut_ptr() as *mut c_void, 2) };
    if !answer_counts(got, byte[1], state.check_turn, state.checking) {

    } else if byte[0] == CHECK_OK {
        state.checking = false;
        state.check_since = None;
        unlock(state);
    } else {
        state.checking = false;
        state.check_since = None;
        state.message = match (byte[0], state.mode) {
            (CHECK_PHONE_NO, Mode::Pin) => t!("session-lock-phone-no-pin"),
            (CHECK_PHONE_NO, Mode::Password) => t!("session-lock-phone-no-password"),
            _ => t!("session-lock-wrong-password"),
        }.into();
        state.wrong = true;
        if byte[0] == CHECK_WRONG {
            state.reader.wrong();
        }
        redraw(state, qh);
    }
}

fn housekeeping(state: &mut State, qh: &QueueHandle<State>) {

    if state.moves_every().is_some_and(|every| state.moved.elapsed() >= every) {
        redraw(state, qh);
    }
    state.old.retain(|k| {
        let busy = k.busy.load(Ordering::Relaxed);
        if !busy {
            k.buffer.destroy();
        }
        busy
    });

    let now = crate::common::clock::Local::now();
    let minute = now.minute();
    if minute != state.minute {
        state.minute = minute;
        state.reader.time(now.hour(), minute);
        if state.mode == Mode::Pin && !state.pin_allowed() {
            state.ask_for_password(t!("session-lock-pin-timed-out"));
        }
        redraw(state, qh);
    }
}
