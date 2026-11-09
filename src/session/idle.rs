//! trindle-idle: notices the computer being left alone and does what
//! Settings, Power says, in turn: dim the screen, show the moving clock,
//! lock, turn the screens off, sleep. Any movement undoes the first ones.
//! It also locks before the computer sleeps, and when logind asks.

use std::cell::RefCell;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::Duration;

use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::{wl_output, wl_registry, wl_seat};
use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{self, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};
use wayland_protocols_wlr::output_power_management::v1::client::{
    zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1,
    zwlr_output_power_v1::{self, ZwlrOutputPowerV1},
};

use crate::common::clock::Local;
use crate::common::files::setting;
use crate::toolkit::bus::{Bus, Listen};
use crate::toolkit::event::{Flow, Loop};

const LOGIN: &str = "org.freedesktop.login1";
const LOGIN_PATH: &str = "/org/freedesktop/login1";
const MANAGER: &str = "org.freedesktop.login1.Manager";
const SESSION: &str = "org.freedesktop.login1.Session";

const DIM_TO: f64 = 0.4;

const DIM_LEAST: f64 = 0.02;

const ANSWER: Duration = Duration::from_secs(10);

fn minutes(key: &str) -> u32 {
    let key = if crate::common::charge::on_battery() { format!("{key}_battery") } else { key.to_string() };
    setting(&key).trim().parse().unwrap_or(0)
}

fn caffeine_mark() -> PathBuf {
    crate::common::files::scratch().join("trindle-keep-awake")
}

fn pidfile() -> PathBuf {
    crate::common::files::scratch().join("trindle-idle.pid")
}

fn record_file() -> PathBuf {
    crate::common::files::scratch().join("trindle-idle.log")
}

fn bright_file() -> PathBuf {
    crate::common::files::scratch().join("trindle-idle.bright")
}

fn dimmed(now: u32, max: u32) -> Option<u32> {
    let dim = ((now as f64) * DIM_TO).round().max((max as f64) * DIM_LEAST).max(1.0) as u32;
    (now > dim).then_some(dim)
}

fn record(what: &str) {
    use std::io::Write;
    let path = record_file();
    if std::fs::metadata(&path).is_ok_and(|m| m.len() > 32_768) {
        let _ = std::fs::rename(&path, path.with_extension("log.old"));
    }
    let when = Local::now().format("%F %T");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{when} [{}] {what}", std::process::id());
    }
}

pub fn keep_awake() -> bool {
    caffeine_mark().exists()
}

fn stop_running() {
    let Some(pid) = std::fs::read_to_string(pidfile()).ok().and_then(|p| p.trim().parse::<i32>().ok()) else { return };
    let ours = std::fs::read_to_string(format!("/proc/{pid}/comm")).is_ok_and(|c| c.trim() == "trindle-idle");
    if ours && pid != std::process::id() as i32 {
        // SAFETY: a signal to the pid in our own pidfile, just checked to be a

        unsafe { libc::kill(pid, libc::SIGTERM) };
    }
}

fn start_again() {
    crate::common::run::spawn(&["trindle-idle"]);
}

pub fn restart() {
    stop_running();
    if !keep_awake() && !held() {
        start_again();
    }
}

pub fn set_keep_awake(on: bool) {
    if on {
        let _ = std::fs::write(caffeine_mark(), "Trindle keeps the screen awake while this file is here.\n");
        stop_running();
    } else {
        let _ = std::fs::remove_file(caffeine_mark());
        restart();
    }
}

fn holds() -> PathBuf {
    crate::common::files::scratch().join("trindle-awake")
}

fn held() -> bool {
    let Ok(dir) = std::fs::read_dir(holds()) else { return false };
    dir.flatten().any(|f| {
        let name = f.file_name().to_string_lossy().into_owned();
        let pid = name.split('-').next().and_then(|p| p.parse::<u32>().ok());
        let alive = pid.is_some_and(|p| std::path::Path::new(&format!("/proc/{p}")).exists());
        if !alive {
            let _ = std::fs::remove_file(f.path());
        }
        alive
    })
}

pub fn hold_awake(tag: &str) {
    let dir = holds();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join(format!("{}-{tag}", std::process::id())), "Kept awake while this file is here.\n");
    stop_running();
}

pub fn release_awake(tag: &str) {
    let _ = std::fs::remove_file(holds().join(format!("{}-{tag}", std::process::id())));
    restart();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Dim,
    Screensaver,
    Lock,
    Blank,
    Sleep,
}

fn plan() -> Vec<(Step, u32)> {
    let mut steps = Vec::new();
    let mut add = |step, after: u32| {
        if after > 0 {
            steps.push((step, after));
        }
    };
    if has_backlight().is_some() {
        add(Step::Dim, minutes("dim_after"));
    }
    add(Step::Screensaver, minutes("screensaver_after"));

    if setting("lock") == "on" {
        add(Step::Lock, minutes("lock_after"));
    }
    add(Step::Blank, minutes("blank_after"));
    add(Step::Sleep, minutes("suspend_after"));
    steps
}

fn has_backlight() -> Option<String> {
    std::fs::read_dir("/sys/class/backlight").ok()?.flatten().next().map(|e| e.file_name().to_string_lossy().to_string())
}

fn spawn(argv: &[&str]) {
    if let Ok(mut child) = Command::new(argv[0]).args(&argv[1..]).stdin(Stdio::null()).stdout(Stdio::null())
        .stderr(Stdio::null()).spawn() {
        std::thread::spawn(move || {
            let _ = child.wait();
        });
    }
}

struct State {
    seat: Option<wl_seat::WlSeat>,
    outputs: Vec<wl_output::WlOutput>,
    power: Option<ZwlrOutputPowerManagerV1>,

    off: Vec<ZwlrOutputPowerV1>,

    steps: Vec<Step>,

    fired: Vec<Step>,

    was_bright: Option<(String, u32)>,

    system: Option<(Bus, Loop)>,
}

impl State {
    fn idled(&mut self, step: Step, qh: &QueueHandle<State>) {
        record(&format!("left alone: {step:?}"));
        self.fired.push(step);
        match step {
            Step::Dim => {
                if let Some(name) = has_backlight() {
                    let read = |f: &str| std::fs::read_to_string(format!("/sys/class/backlight/{name}/{f}"))
                        .ok().and_then(|v| v.trim().parse::<u32>().ok());
                    if let (Some(now), Some(max)) = (read("brightness"), read("max_brightness")) {
                        match dimmed(now, max) {
                            Some(dim) => {
                                let _ = std::fs::write(bright_file(), format!("{name} {now}"));
                                self.was_bright = Some((name.clone(), now));
                                set_brightness(&name, dim);
                            }
                            None => record(&format!("already dim at {now} of {max}")),
                        }
                    }
                }
            }
            Step::Screensaver => spawn(&["trindle-screensaver"]),
            Step::Lock => spawn(&["trindle-lock", "--resting"]),
            Step::Blank => self.screens(false, qh),
            Step::Sleep => {
                if let Some((bus, events)) = &self.system {
                    bus.call(LOGIN, LOGIN_PATH, MANAGER).method("Suspend", &(false,), false, Some(ANSWER), events, |_| {});
                }
            }
        }
    }

    fn resumed(&mut self, step: Step, qh: &QueueHandle<State>) {
        if !self.fired.contains(&step) {
            return;
        }
        self.fired.retain(|s| *s != step);
        record(&format!("back: undo {step:?}"));
        match step {
            Step::Dim => {
                if let Some((name, value)) = self.was_bright.take() {
                    set_brightness(&name, value);
                }
                let _ = std::fs::remove_file(bright_file());
            }
            Step::Screensaver => spawn(&["trindle-screensaver", "--stop"]),
            Step::Blank => self.screens(true, qh),

            Step::Lock | Step::Sleep => {}
        }
    }

    fn screens(&mut self, on: bool, qh: &QueueHandle<State>) {
        let Some(manager) = &self.power else {
            record("this desktop cannot turn screens off");
            return;
        };
        if on {
            for p in self.off.drain(..) {
                p.set_mode(zwlr_output_power_v1::Mode::On);
                p.destroy();
            }
        } else {
            for o in &self.outputs {
                let p = manager.get_output_power(o, qh, ());
                p.set_mode(zwlr_output_power_v1::Mode::Off);
                self.off.push(p);
            }
        }
    }
}

fn set_brightness(name: &str, value: u32) {
    if let Err(e) = crate::session::brightness::set(name, value) {
        record(&format!("brightness not set: {e}"));
    }
}

thread_local! {

    static DELAY: RefCell<Option<zbus::zvariant::OwnedFd>> = const { RefCell::new(None) };
}

fn hold_off_sleep(bus: &Bus, events: &Loop) {
    let args = ("sleep", "Trindle", "Locking the screen first", "delay");
    bus.call(LOGIN, LOGIN_PATH, MANAGER).method("Inhibit", &args, false, Some(ANSWER), events, |r| {
        match r.and_then(|m| m.body().deserialize::<zbus::zvariant::OwnedFd>().map_err(Into::into)) {
            Ok(fd) => DELAY.with(|d| *d.borrow_mut() = Some(fd)),
            Err(e) => record(&format!("sleep not held off: {}", e.text)),
        }
    });
}

fn before_sleep() {
    if setting("lock") == "on" && setting("lock_on_suspend") != "no" {

        let _ = Command::new("trindle-lock").stdin(Stdio::null()).status();
    }
    DELAY.with(|d| *d.borrow_mut() = None);
}

fn watch_logind(bus: &Bus, events: &Loop, screens_on: impl Fn() + 'static) -> Rc<RefCell<Vec<Listen>>> {
    let hearing = Rc::new(RefCell::new(Vec::new()));
    hold_off_sleep(bus, events);
    let (b, e) = (bus.clone(), events.clone());
    let sleep = bus.listen(Some(LOGIN), MANAGER, Some("PrepareForSleep"), Some(LOGIN_PATH), move |msg| {
        let going = msg.body().deserialize::<bool>().unwrap_or(false);
        if going {
            before_sleep();
        } else {

            screens_on();
            hold_off_sleep(&b, &e);
        }
    });
    hearing.borrow_mut().extend(sleep.ok());

    let (b, h) = (bus.clone(), hearing.clone());
    bus.call(LOGIN, LOGIN_PATH, MANAGER).method("GetSessionByPID", &(std::process::id(),), false, Some(ANSWER), events,
                                                move |r| {
        let Some(path) = r.ok().and_then(|m| m.body().deserialize::<zbus::zvariant::OwnedObjectPath>().ok()) else { return };
        let lock = b.listen(Some(LOGIN), SESSION, Some("Lock"), Some(path.as_str()), |_| {
            spawn(&["trindle-lock", "--now"]);
        });
        h.borrow_mut().extend(lock.ok());
    });
    hearing
}

pub fn main(args: Vec<String>) -> i32 {

    if let Some(i) = args.iter().position(|a| a == "--keep-awake") {
        let on = match args.get(i + 1).map(String::as_str) {
            Some("on") => true,
            Some("off") => false,
            _ => !keep_awake(),
        };
        set_keep_awake(on);
        return 0;
    }
    if keep_awake() || held() {
        return 0;
    }

    stop_running();
    let _ = std::fs::write(pidfile(), std::process::id().to_string());
    put_brightness_back();

    let Ok(conn) = Connection::connect_to_env() else {
        eprintln!("trindle-idle: no desktop to watch");
        return 1;
    };
    let Ok((globals, mut queue)) = registry_queue_init::<State>(&conn) else { return 1 };
    let qh = queue.handle();
    let Some(mut state) = watching_state(&globals, &mut queue, &qh) else { return 1 };

    let events = Loop::new();
    let system = Bus::system(&events).ok();
    state.system = system.clone().map(|bus| (bus, events.clone()));
    let queue = Rc::new(RefCell::new(queue));
    let state = Rc::new(RefCell::new(state));
    let _hearing = system.as_ref().map(|bus| {
        let (st, qh, c) = (state.clone(), qh.clone(), conn.clone());
        watch_logind(bus, &events, move || {
            st.borrow_mut().screens(true, &qh);
            let _ = c.flush();
        })
    });
    super::wayland_loop::dispatch_on(&events, &queue, &state, &conn);
    let ev = events.clone();
    events.on_signal(libc::SIGTERM, move || {
        ev.quit();
        Flow::Stop
    });

    let st = state.clone();
    crate::common::renewed::watch(&events, "trindle-idle", move || {
        st.try_borrow().is_ok_and(|s| s.fired.is_empty()).then_some(crate::common::renewed::Renew::InPlace)
    });
    let _ = conn.flush();
    events.run();
    leave(&state, &qh, &conn);
    let _ = queue;
    0
}

fn put_brightness_back() {
    if let Some((name, value)) = std::fs::read_to_string(bright_file()).ok()
        .and_then(|t| t.split_once(' ').and_then(|(n, v)| Some((n.to_string(), v.trim().parse::<u32>().ok()?)))) {
        record("left dimmed: brightness put back");
        set_brightness(&name, value);
        let _ = std::fs::remove_file(bright_file());
    }
}

fn watching_state(globals: &GlobalList, queue: &mut EventQueue<State>, qh: &QueueHandle<State>) -> Option<State> {
    let Ok(notifier) = globals.bind::<ExtIdleNotifierV1, _, _>(qh, 1..=1, ()) else {
        eprintln!("trindle-idle: this desktop cannot say when it is left alone");
        return None;
    };
    let mut state = State {
        seat: globals.bind::<wl_seat::WlSeat, _, _>(qh, 1..=1, ()).ok(),
        outputs: Vec::new(),
        power: globals.bind(qh, 1..=1, ()).ok(),
        off: Vec::new(),
        steps: Vec::new(),
        fired: Vec::new(),
        was_bright: None,
        system: None,
    };
    for g in globals.contents().clone_list() {
        if g.interface == "wl_output" {
            state.outputs.push(globals.registry().bind(g.name, g.version.min(4), qh, ()));
        }
    }
    let seat = state.seat.clone()?;
    let steps = plan();
    record(&format!("watching: {}", steps.iter().map(|(s, m)| format!("{s:?} {m} min")).collect::<Vec<_>>().join(", ")));
    for (i, (step, after)) in steps.into_iter().enumerate() {
        state.steps.push(step);
        notifier.get_idle_notification(after.saturating_mul(60_000), &seat, qh, i);
    }
    let _ = queue.roundtrip(&mut state);
    Some(state)
}

fn leave(state: &Rc<RefCell<State>>, qh: &QueueHandle<State>, conn: &Connection) {
    let mut st = state.borrow_mut();
    let steps: Vec<Step> = st.fired.clone();
    for step in steps {
        st.resumed(step, qh);
    }
    let _ = conn.flush();

    if std::fs::read_to_string(pidfile()).is_ok_and(|p| p.trim() == std::process::id().to_string()) {
        let _ = std::fs::remove_file(pidfile());
    }
    record("stopped");
}

pub fn lock(args: Vec<String>) -> i32 {
    super::lock::run::main(args)
}

impl Dispatch<ExtIdleNotificationV1, usize> for State {
    fn event(state: &mut Self, _: &ExtIdleNotificationV1, event: ext_idle_notification_v1::Event, index: &usize,
             _: &Connection, qh: &QueueHandle<Self>) {
        let Some(step) = state.steps.get(*index).copied() else { return };
        match event {
            ext_idle_notification_v1::Event::Idled => state.idled(step, qh),
            ext_idle_notification_v1::Event::Resumed => state.resumed(step, qh),
            _ => {}
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, qh: &QueueHandle<Self>) {

        if let wl_registry::Event::Global { name, interface, version } = event {
            if interface == "wl_output" {
                state.outputs.push(registry.bind(name, version.min(4), qh, ()));
            }
        }
    }
}

impl Dispatch<ZwlrOutputPowerV1, ()> for State {
    fn event(_: &mut Self, power: &ZwlrOutputPowerV1, event: zwlr_output_power_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        if let zwlr_output_power_v1::Event::Failed = event {
            record("a screen refused to turn off");
            power.destroy();
        }
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for State {
            fn event(_: &mut Self, _: &$t, _: <$t as wayland_client::Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
        }
    )*};
}
ignore!(ExtIdleNotifierV1, wl_seat::WlSeat, wl_output::WlOutput, ZwlrOutputPowerManagerV1);

#[cfg(test)]
mod tests {
    #[test]
    fn dimming_always_shows_but_never_goes_black() {
        assert_eq!(super::dimmed(1000, 1000), Some(400));

        assert_eq!(super::dimmed(300, 1000), Some(120));
        assert_eq!(super::dimmed(30, 1000), Some(20));
        assert_eq!(super::dimmed(20, 1000), None);
        assert_eq!(super::dimmed(1, 1), None);
    }

    #[test]
    fn a_step_set_to_never_is_left_out() {

        assert!(super::plan().iter().all(|(_, m)| *m > 0));
    }
}
