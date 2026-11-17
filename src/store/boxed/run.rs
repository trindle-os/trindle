//! Running a command in its box, and what is made for it to start.

use crate::toolkit::t;
use std::os::fd::{AsRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::store::parts;
use crate::common::x11_rooms::{self};
use super::bus::{Band, Bus};
use super::walls::{ALSA_PULSE, Gates, engine_for, real_dir, sys_args, walls};
use super::x11::{XNext, own_x11, tell, tell_would_not_start, x_next};

pub fn run_boxed(at: &Path, app_id: &str, allowed: &[String], command: &[String]) -> i32 {
    run_boxed_extra(at, app_id, allowed, &[], command)
}

pub fn run_boxed_extra(at: &Path, app_id: &str, allowed: &[String], extra: &[String], command: &[String]) -> i32 {
    run_boxed_loop(at, app_id, allowed, extra, command, false)
}

pub fn run_boxed_bus(at: &Path, app_id: &str, allowed: &[String], extra: &[String], command: &[String]) -> i32 {
    run_boxed_loop(at, app_id, allowed, extra, command, true)
}

fn run_boxed_loop(at: &Path, app_id: &str, allowed: &[String], extra: &[String], command: &[String], bus: bool) -> i32 {
    let mut tries = 0;
    loop {
        tries += 1;
        let (code, told) = run_boxed_once(at, app_id, allowed, extra, command, bus);
        match x_next(told, tries) {
            XNext::Done => return code,
            XNext::Again => eprintln!("trindle-boxed: {app_id}: its Xwayland ended under it; starting it again"),
            XNext::Say => {
                tell_would_not_start(app_id);
                return code;
            }
        }
    }
}

fn run_boxed_once(at: &Path, app_id: &str, allowed: &[String], extra: &[String], command: &[String], bus: bool) -> (i32, Option<x11_rooms::Told>) {
    let has = |k: &str| allowed.iter().any(|w| w == k);

    let Some(folder) = RunFolder::make(app_id) else { return (1, None) };
    let run = folder.0.clone();
    let mut start = Start { gates: Gates { sys: sys_args(), extra: extra.to_vec(), ..Gates::default() }, keep: Vec::new() };

    let band = if bus { lend_band(app_id) } else { None };

    let _bus = if bus { Bus::start(&run, band.as_ref().map(|b| &b.0)).map_err(|why| eprintln!("trindle-boxed: {app_id}: no session bus: {why}")).ok() } else { None };
    start.gates.bus = _bus.as_ref().map(Bus::socket);
    start.gates.starter = band.as_ref().filter(|_| _bus.is_some()).map(|(b, exe)| (super::start::Host::now(exe.clone()), *b.pids().start()));
    if start.own_wayland(&run, app_id, allowed).is_none() {
        return (1, None);
    }

    let _copied = if has("copied") { crate::store::copied_guard::start(&run, app_id, at) } else { None };
    start.gates.copied = _copied.as_ref().map(|g| g.socket());
    if has("sound") && !has("microphone") {
        start.sound_guard(&run, app_id);
    }

    let (x11, x11_held) = if has("x11") { own_x11(app_id) } else { (None, None) };
    start.gates.x11 = x11;
    if start.call_filter(app_id).is_none() {
        return (1, None);
    }
    if has(crate::plugins::USB_DEVICES) {
        start.gates.usb = crate::store::usb_devices::args();
    }
    if has(crate::plugins::ACCESSIBILITY) && at.parent().is_some_and(|f| crate::common::own_hands::is_screen_reader_in(f, app_id)) {
        let runtime = crate::store::pulse_guard::runtime_dir();
        use crate::store::accessibility as a;
        start.gates.a11y = a::bus_socket(&runtime).zip(a::keys_folder(&runtime)).zip(a::voice_folder(&runtime)).map(|((b, k), v)| (b, k, v));
    }

    let room = home_room(app_id, allowed, &start.gates, &run);
    start.gates.home_room = room.as_ref().map(|r| r.ids());

    let _in_use = in_use_rows(app_id, allowed, room.is_some(), !start.gates.usb.is_empty());
    let code = start.launch(room, app_id, (at, allowed, command));
    (code, x11_held.as_ref().and_then(|h| h.told(code)))
}

fn lend_band(app_id: &str) -> Option<(Band, String)> {

    let exe = std::env::current_exe().ok().map(|e| e.to_string_lossy().trim_end_matches(" (deleted)").to_string());
    let Some(exe) = exe.filter(|e| Path::new(e).is_file()) else {
        eprintln!("trindle-boxed: {app_id}: Trindle's own program cannot be found, so it has no tray icon");
        return None;
    };
    let Some(band) = Band::take(&boxed_runs().join(BANDS_AT)) else {
        eprintln!("trindle-boxed: {app_id}: too many boxes are running for another tray icon");
        return None;
    };
    Some((band, exe))
}

const BANDS_AT: &str = "bands";

fn boxed_runs() -> PathBuf {
    crate::store::pulse_guard::runtime_dir().join("trindle-boxed")
}

static LIVE: std::sync::Mutex<Vec<PathBuf>> = std::sync::Mutex::new(Vec::new());

pub(super) struct RunFolder(pub(super) PathBuf);

impl RunFolder {

    pub(super) fn make(app_id: &str) -> Option<RunFolder> {
        let base = boxed_runs();
        sweep(&base, |pid| Path::new(&format!("/proc/{pid}")).exists());

        let short: String = app_id.chars().take(24).collect();
        let run = base.join(format!("{short}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&run);
        if let Err(e) = std::fs::create_dir_all(&run) {
            eprintln!("trindle-boxed: {}: {e}", run.display());
            return None;
        }
        clean_on_stop();
        LIVE.lock().unwrap_or_else(|e| e.into_inner()).push(run.clone());
        Some(RunFolder(run))
    }
}

impl Drop for RunFolder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
        LIVE.lock().unwrap_or_else(|e| e.into_inner()).retain(|p| *p != self.0);
    }
}

pub(super) fn sweep(base: &Path, alive: impl Fn(u32) -> bool) {
    for e in std::fs::read_dir(base).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(pid) = name.rsplit_once('-').and_then(|(_, p)| p.parse::<u32>().ok()) else { continue };
        if !alive(pid) && real_dir(&e.path()) {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
}

const STOPS: [i32; 3] = [libc::SIGTERM, libc::SIGINT, libc::SIGHUP];

static STOPPED: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

extern "C" fn on_stop(sig: libc::c_int) {
    let byte = sig as u8;
    // SAFETY: write is async-signal-safe and writes the one byte it is given.
    unsafe { libc::write(STOPPED.load(std::sync::atomic::Ordering::Relaxed), (&byte as *const u8).cast(), 1) };
}

fn clean_on_stop() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let Some((read, write)) = crate::store::secctx::close_pipe() else { return };
        STOPPED.store(std::os::fd::IntoRawFd::into_raw_fd(write), std::sync::atomic::Ordering::Relaxed);
        std::thread::spawn(move || {
            let mut byte = [0u8];
            if !matches!(std::io::Read::read(&mut std::fs::File::from(read), &mut byte), Ok(1)) {
                return;
            }
            for run in LIVE.lock().unwrap_or_else(|e| e.into_inner()).iter() {
                let _ = std::fs::remove_dir_all(run);
            }
            let sig = libc::c_int::from(byte[0]);
            // SAFETY: puts the signal's own ending back and sends it to this process.
            unsafe {
                libc::signal(sig, libc::SIG_DFL);
                libc::raise(sig);
            }
        });
        for sig in STOPS {
            // SAFETY: the handler only writes one byte to a pipe.
            unsafe { libc::signal(sig, on_stop as *const () as libc::sighandler_t) };
        }
    });
}

struct Start {
    gates: Gates,
    keep: Vec<OwnedFd>,
}

impl Start {

    fn own_wayland(&mut self, run: &Path, app_id: &str, allowed: &[String]) -> Option<()> {
        let base = crate::store::pulse_guard::runtime_dir();
        let plain = std::env::var("WAYLAND_DISPLAY").ok().filter(|w| !w.is_empty())
            .map(|w| if w.starts_with('/') { w } else { base.join(w).to_string_lossy().into_owned() });
        self.gates.wayland = match plain {
            Some(_) if !allowed.iter().any(|w| w == "screen") => match crate::store::secctx::close_pipe() {
                Some((read, write)) => {
                    let socket = crate::store::secctx::socket_in(run);
                    match crate::store::secctx::listen(&socket, app_id, engine_for(allowed), &read) {
                        Ok(()) => {
                            crate::store::secctx::keep_across_exec(&write);
                            self.gates.sync = Some(write.as_raw_fd());
                            self.keep.push(write);
                            Some(socket.to_string_lossy().into_owned())
                        }
                        Err(e) => {
                            eprintln!("trindle-boxed: {app_id}: {e}; it does not start without its own socket");
                            return None;
                        }
                    }
                }
                None => {
                    eprintln!("trindle-boxed: {app_id}: no pipe for its own socket; it does not start");
                    return None;
                }
            },
            other => other,
        };
        Some(())
    }

    fn sound_guard(&mut self, run: &Path, app_id: &str) {
        let guard = run.join("pulse-guard");
        match std::os::unix::net::UnixListener::bind(&guard) {
            Ok(listener) => {
                let real = crate::store::pulse_guard::real_socket();
                std::thread::spawn(move || crate::store::pulse_guard::serve(listener, real));
                self.gates.pulse = Some(guard.to_string_lossy().into_owned());
                let conf = run.join("asound.conf");
                if std::fs::write(&conf, ALSA_PULSE).is_ok() {
                    self.gates.alsa = Some(conf.to_string_lossy().into_owned());
                }
            }
            Err(e) => eprintln!("trindle-boxed: {app_id}: no sound: {e}"),
        }
    }

    fn call_filter(&mut self, app_id: &str) -> Option<()> {
        match crate::common::seccomp::memfd() {
            Some(fd) => {
                self.gates.seccomp = Some(fd.as_raw_fd());
                self.keep.push(fd);
                Some(())
            }
            None => {
                eprintln!("trindle-boxed: {app_id}: the system call filter could not be made; it does not start");
                None
            }
        }
    }

    fn launch(self, room: Option<crate::store::home_net::Room>, app_id: &str, what: (&Path, &[String], &[String])) -> i32 {
        let Start { mut gates, keep } = self;
        let (at, allowed, command) = what;
        match room {
            Some(room) => {
                let mut argv = vec!["bwrap".to_string()];
                argv.extend(walls(at, allowed, &gates, command));
                let out = room.run(&argv);
                if out.started {
                    drop(keep);
                    out.code
                } else {

                    tell_home_network_off(app_id, t!("store-boxed-home-why-rules"));
                    gates.home_room = None;
                    let status = Command::new("bwrap").args(walls(at, allowed, &gates, command)).spawn();
                    drop(keep);
                    code_of(status)
                }
            }
            None => {
                let status = Command::new("bwrap").args(walls(at, allowed, &gates, command)).spawn();

                drop(keep);
                code_of(status)
            }
        }
    }
}

fn code_of(status: std::io::Result<std::process::Child>) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    match status {
        Ok(mut child) => match child.wait() {
            Ok(s) => s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0)),
            Err(_) => 1,
        },
        Err(e) => {
            eprintln!("trindle-boxed: could not start bubblewrap: {e}");
            1
        }
    }
}

pub(super) fn home_room(app_id: &str, allowed: &[String], gates: &Gates, run: &Path) -> Option<crate::store::home_net::Room> {
    use crate::store::home_net::{Refused, Room, Tools};
    let has = |k: &str| allowed.iter().any(|w| w == k);
    if !has(crate::plugins::HOME_NETWORK) || has("network") {
        return None;
    }
    if gates.seccomp.is_none() {
        tell_home_network_off(app_id, t!("store-boxed-home-why-filter"));
        return None;
    }
    let made = Tools::installed().map_err(Refused::Because).and_then(|t| Room::prepare(&t, run));
    match made {
        Ok(room) => Some(room),
        Err(Refused::Offline) => {
            eprintln!("trindle-boxed: {app_id}: this computer has no network to reach, so it has none");
            None
        }
        Err(Refused::Because(why)) => {
            tell_home_network_off(app_id, &why);
            None
        }
    }
}

fn tell_home_network_off(app: &str, why: &str) {
    eprintln!("trindle-boxed: {app}: no home network: {why}");
    tell(t!("store-boxed-home-off"), &t!("store-boxed-home-off-why", app = crate::plugins::find(app).map_or_else(|| app.to_string(), |p| p.name), why = why));
}

fn in_use_rows(app_id: &str, allowed: &[String], room: bool, usb: bool) -> Option<crate::common::in_use::BoxRows> {
    use crate::common::in_use::{BoxRows, Kind};
    let has = |k: &str| allowed.iter().any(|w| w == k);
    let name = crate::plugins::find(app_id).map_or_else(|| app_id.to_string(), |p| p.name);
    let mut rows: Vec<(Kind, &str, &str)> = Vec::new();
    if room && has(crate::plugins::HOME_NETWORK) {
        rows.push((Kind::HomeNetwork, &name, app_id));
    }
    if usb && has(crate::plugins::USB_DEVICES) {
        rows.push((Kind::Usb, &name, app_id));
    }
    BoxRows::start(std::process::id(), &rows)
}

pub(super) fn loader() -> &'static str {
    if cfg!(target_arch = "aarch64") { "/lib/ld-linux-aarch64.so.1" } else { "/lib64/ld-linux-x86-64.so.2" }
}

pub fn plain_command(at: &Path, command: &[String]) -> Command {
    let files = at.join("files");
    let triplet = parts::triplet();
    let bins: Vec<PathBuf> = ["usr/bin", "usr/games", "usr/sbin"].iter().map(|b| files.join(b)).collect();
    let mut path = std::env::var("PATH").unwrap_or_else(|_| "/usr/local/bin:/usr/bin:/bin".into());
    for b in &bins {
        path.push(':');
        path.push_str(&b.to_string_lossy());
    }
    let first = command.first().map(String::as_str).unwrap_or("");
    let found = (!first.contains('/')).then(|| bins.iter().map(|b| b.join(first)).find(|p| p.is_file())).flatten();
    let mut c = match found {
        Some(p) if parts::is_elf(&p) => {
            let libs = format!("{}:{}", files.join(format!("usr/lib/{triplet}")).display(), files.join("usr/lib").display());
            let mut c = Command::new(loader());
            c.arg("--library-path").arg(libs).arg("--argv0").arg(first).arg(&p);
            c
        }
        Some(p) => Command::new(p),
        None => Command::new(first),
    };
    c.args(command.iter().skip(1));
    let data = std::env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/local/share:/usr/share".into());
    c.env("PATH", path)
        .env("TERMINFO_DIRS", format!("{}:", files.join("usr/share/terminfo").display()))
        .env("XDG_DATA_DIRS", format!("{data}:{}", files.join("usr/share").display()));
    c
}
