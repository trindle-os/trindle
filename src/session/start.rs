//! trindle-session (becomes Trindle's compositor, which starts the desktop),
//! trindle-greeter-session (the compositor holding the login screen, with a
//! text-login fallback) and trindle-autostart.

use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use crate::common::files::{config_home, home, read_env, user_environment};

pub const COMPOSE: &str = "/usr/lib/trindle/bin/trindle-compose";

pub const SYSTEM_SHARE: &str = "/usr/share/trindle";

fn prepend_path(dir: &str) {
    let path = std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".into());
    if !path.split(':').any(|p| p == dir) {
        std::env::set_var("PATH", format!("{dir}:{path}"));
    }
}

fn append_path(dir: &str) {
    let path = std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".into());
    if !path.split(':').any(|p| p == dir) && std::path::Path::new(dir).is_dir() {
        std::env::set_var("PATH", format!("{path}:{dir}"));
    }
}

fn installed_keyboard() {
    let kb = read_env(Path::new("/etc/default/keyboard"));
    let get = |k: &str| kb.get(k).map(|v| v.trim_matches('"').to_string()).filter(|v| !v.is_empty());
    std::env::set_var("XKB_DEFAULT_LAYOUT", get("XKBLAYOUT").unwrap_or_else(|| "us".into()));
    for (from, to) in [("XKBVARIANT", "XKB_DEFAULT_VARIANT"), ("XKBOPTIONS", "XKB_DEFAULT_OPTIONS")] {
        if let Some(v) = get(from) {
            std::env::set_var(to, v);
        }
    }
}

fn exit_code(status: std::process::ExitStatus) -> i32 {
    if asked_to_end(status) {
        return 0;
    }
    status.code().unwrap_or(1)
}

fn asked_to_end(status: std::process::ExitStatus) -> bool {
    use std::os::unix::process::ExitStatusExt;
    status.success() || matches!(status.signal(), Some(libc::SIGTERM | libc::SIGHUP | libc::SIGINT))
}

fn open_log(path: &Path) -> Option<std::fs::File> {
    std::fs::OpenOptions::new().create(true).append(true).open(path).ok()
}

pub fn session(_args: Vec<String>) -> i32 {
    set_environment();
    set_share_dirs();
    qt_look();
    gtk_portal();
    crate::keyring::migrate::refresh_portals();
    move_off_labwc();
    crate::common::compositor::end_programs();

    let state = home().join(".local/state/trindle");
    let _ = std::fs::create_dir_all(&state);
    let log = start_log(&state);
    choose_drawing(&log);

    between_screens();

    end_services();

    let plain = state.join("plain-drawing");
    if plain.exists() {
        std::env::set_var(PLAIN_DRAWING, "1");
    }
    if std::env::var_os(PLAIN_DRAWING).is_none() {
        if let Some(code) = first_try(&log, &plain) {
            return code;
        }
    }
    keep_running(&log)
}

fn set_environment() {
    prepend_path("/usr/local/bin");

    let ai = crate::store::fence::root();
    crate::store::fence::sync_wrappers(&ai);
    prepend_path(&crate::store::fence::wrappers(&ai).to_string_lossy());

    append_path("/usr/sbin");
    append_path("/sbin");
    std::env::set_var("XDG_SESSION_TYPE", "wayland");
    std::env::set_var("XDG_CURRENT_DESKTOP", "Trindle");
    std::env::set_var("XDG_SESSION_DESKTOP", "trindle");

    std::env::set_var("GNOME_DESKTOP_SESSION_ID", "this-is-deprecated");
    installed_keyboard();
}

fn set_share_dirs() {
    let data = std::env::var("XDG_DATA_DIRS").ok().filter(|d| !d.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    let h = home();
    let mut dirs = vec![SYSTEM_SHARE.to_string()];
    if !data.contains("flatpak") {
        dirs.push(h.join(".local/share/flatpak/exports/share").display().to_string());
        dirs.push("/var/lib/flatpak/exports/share".to_string());
    }
    dirs.push(data);
    std::env::set_var("XDG_DATA_DIRS", dirs.join(":"));
}

fn start_log(state: &Path) -> PathBuf {
    let log = state.join("session.log");
    if log.exists() {
        let _ = std::fs::rename(&log, state.join("session.log.old"));
    }
    if let Some(mut f) = open_log(&log) {
        use std::io::Write;
        let now = crate::common::clock::Local::now().format("%c");
        let _ = writeln!(f, "Trindle session starting {now}");
        let _ = writeln!(f, "user={} XDG_RUNTIME_DIR={} XDG_SESSION_ID={}", std::env::var("USER").unwrap_or_default(),
                         std::env::var("XDG_RUNTIME_DIR").unwrap_or("unset".into()),
                         std::env::var("XDG_SESSION_ID").unwrap_or("unset".into()));
        let _ = writeln!(f, "keyboard={}", std::env::var("XKB_DEFAULT_LAYOUT").unwrap_or_default());
    }
    log
}

fn choose_drawing(log: &Path) {
    let light = crate::common::graphics::light_drawing(
        std::env::var(crate::common::graphics::DRAWING).ok().as_deref());
    if light {
        std::env::set_var(crate::common::graphics::DRAWING, "light");
        std::env::set_var("GSK_RENDERER", "cairo");
    }
    if let Some(mut f) = open_log(log) {
        use std::io::Write;
        let _ = writeln!(f, "drawing={} (graphics chip: {})", if light { "light" } else { "normal" },
                         if crate::common::graphics::has_graphics_chip() { "yes" } else { "none found" });
    }
}

fn desktop_command(log: &Path) -> Command {
    let mut cmd = Command::new(COMPOSE);
    if let (Some(out), Some(err)) = (open_log(log), open_log(log)) {
        cmd.stdout(out).stderr(err);
    }

    // SAFETY: between fork and exec the hook only calls prctl, which is

    unsafe {
        cmd.pre_exec(|| {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
            Ok(())
        });
    }
    cmd
}

fn run_desktop(log: &Path) -> Option<ExitStatus> {
    match desktop_command(log).status() {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("trindle-session: could not start the desktop: {e}");
            None
        }
    }
}

fn first_try(log: &Path, plain: &Path) -> Option<i32> {
    let start = Instant::now();
    let Some(mut status) = run_desktop(log) else { return Some(1) };

    while crate::common::restart::all_asked() && asked_to_end(status) {
        end_services();
        let Some(next) = run_desktop(log) else { return Some(1) };
        status = next;
    }
    if asked_to_end(status) || start.elapsed() > FIRST_TRY {
        end_services();
        return Some(exit_code(status));
    }
    if let Some(mut f) = open_log(log) {
        use std::io::Write;
        let _ = writeln!(f, "The desktop stopped at once ({status}); starting it again without extra planes");
    }
    let _ = std::fs::write(plain, "");
    std::env::set_var(PLAIN_DRAWING, "1");
    None
}

fn keep_running(log: &Path) -> i32 {
    let result = |status: Option<ExitStatus>| status.map_or((false, 1), |s| (asked_to_end(s), exit_code(s)));
    let mut code = result(run_desktop(log));
    while code.0 && crate::common::restart::all_asked() {
        end_services();
        code = result(run_desktop(log));
    }
    end_services();
    code.1
}

pub const SESSION_TARGET: &str = "trindle-session.target";

pub fn end_services() {
    let _ = Command::new("systemctl").args(["--user", "stop", SESSION_TARGET])
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();

    // and leave programs waiting at start. Started afresh when next asked.
    let _ = Command::new("systemctl").args(["--user", "stop", "xdg-desktop-portal*.service"])
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
}

const PLAIN_DRAWING: &str = "ANVIL_DISABLE_DIRECT_SCANOUT";

const FIRST_TRY: Duration = Duration::from_secs(20);

fn gtk_portal() {
    if std::env::var_os(GTK_PORTAL).is_none() {
        std::env::set_var(GTK_PORTAL, "1");
    }
}

const GTK_PORTAL: &str = "GTK_USE_PORTAL";

const ACTIVATION: [&str; 6] = ["WAYLAND_DISPLAY", "DISPLAY", "XDG_CURRENT_DESKTOP", "XDG_SESSION_TYPE", "GNOME_DESKTOP_SESSION_ID", GTK_PORTAL];

fn qt_look() {
    if std::env::var_os("QT_QPA_PLATFORMTHEME").is_some() {
        return;
    }
    std::env::set_var("QT_QPA_PLATFORMTHEME", "kde");
    std::env::set_var("KDE_SESSION_VERSION", "6");
    let system = std::env::var("XDG_CONFIG_DIRS").ok().filter(|d| !d.is_empty())
        .unwrap_or_else(|| "/etc/xdg".into());
    let folder = crate::common::look::qt_folder();
    std::env::set_var("XDG_CONFIG_DIRS", format!("{}:{system}", folder.display()));
    crate::common::look::write_qt();
}

fn move_off_labwc() {
    let mine = user_environment();
    if mine.exists() {
        return;
    }
    let old = config_home().join("labwc/environment.d");
    let mut lines = String::new();
    let Ok(dir) = std::fs::read_dir(&old) else { return };
    let mut files: Vec<PathBuf> = dir.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "env")).collect();
    files.sort();
    for f in files {
        for (k, v) in read_env(&f) {
            lines.push_str(&format!("{k}={v}\n"));
        }
    }
    if !lines.is_empty() {
        let _ = crate::common::files::write_text(&mine, &lines);
    }
}

fn between_screens() {

    // keep it as a dead entry in the process list all session).
    let _ = Command::new("/bin/sh")
        .args(["-c", "/usr/lib/trindle/bin/trindle-unlock --spin-between </dev/null >/dev/null 2>&1 &"])
        .status();
}

pub fn greeter_session(_args: Vec<String>) -> i32 {
    std::env::set_var("PATH", "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin");
    installed_keyboard();

    std::env::set_var("GIO_USE_VFS", "local");
    let start = Instant::now();

    let cache = home().join(".cache");
    let _ = std::fs::create_dir_all(&cache);
    let log = cache.join("trindle-greeter.log");
    let _ = std::fs::rename(&log, cache.join("trindle-greeter.log.old"));

    between_screens();
    let mut login = Command::new(COMPOSE);
    login.args(["--greeter", "trindle-greeter"]).stdin(Stdio::null());
    if let (Some(out), Some(err)) = (open_log(&log), open_log(&log)) {
        login.stdout(out).stderr(err);
    }
    let status = login.status();
    let code = match &status {
        Ok(s) => exit_code(*s),
        Err(_) => 127,
    };
    if code != 0 && start.elapsed() < Duration::from_secs(10) {
        println!("The graphical login did not start (exit {code}). Using the text login instead.");
        let e = Command::new("/usr/sbin/agreety").args(["--cmd", "/usr/bin/trindle-session"]).exec();
        eprintln!("trindle-greeter-session: {e}");
        return 1;
    }
    code
}

const TRAY_WAIT: Duration = Duration::from_secs(10);

const KEPT_RING_WAIT: Duration = Duration::from_secs(60);

const RING_DOOR: &str = "TRINDLE_RING_DOOR";

fn ring_door() -> Door {
    let named = std::env::var(RING_DOOR).ok();
    std::env::remove_var(RING_DOOR);
    door_named(named.as_deref())
}

enum Door {

    Handed(std::fs::File),

    Missing,

    NotOffered,
}

fn door_named(named: Option<&str>) -> Door {
    match named {
        None => Door::NotOffered,
        Some(named) => door_numbered(named).map_or(Door::Missing, Door::Handed),
    }
}

fn door_numbered(named: &str) -> Option<std::fs::File> {
    use std::os::fd::FromRawFd;
    let fd: i32 = named.parse().ok()?;
    if fd < 3 {
        return None;
    }
    // SAFETY: fstat only writes into the stat it is handed, and fcntl only

    let pipe = unsafe {
        let mut st: libc::stat = std::mem::zeroed();
        libc::fstat(fd, &mut st) == 0 && (st.st_mode & libc::S_IFMT) == libc::S_IFIFO
            && libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == 0
    };
    // SAFETY: the number is an open pipe handed to this program alone, and

    pipe.then(|| unsafe { std::fs::File::from_raw_fd(fd) })
}

const RING_CHECK: Duration = Duration::from_secs(2);

const RING_TRIES: usize = 5;
const RING_WINDOW: Duration = Duration::from_secs(600);

fn tray_owned(bus: &zbus::blocking::Connection) -> bool {
    bus.call_method(Some("org.freedesktop.DBus"), "/org/freedesktop/DBus", Some("org.freedesktop.DBus"),
        "NameHasOwner", &(crate::ring::sni::WATCHER,))
        .ok().and_then(|m| m.body().deserialize::<bool>().ok()).unwrap_or(false)
}

fn session_bus() -> Option<zbus::blocking::Connection> {
    zbus::blocking::connection::Builder::session().ok()?.method_timeout(Duration::from_secs(1)).build().ok()
}

fn wait_for_tray() {
    wait_for_tray_for(TRAY_WAIT);
}

fn wait_for_tray_for(long: Duration) {
    let Some(bus) = session_bus() else { return };
    let since = Instant::now();
    while since.elapsed() < long {
        if tray_owned(&bus) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn keep_ring() {
    use crate::toolkit::event::{Flow, Loop};
    use std::cell::Cell;
    use std::rc::Rc;
    let events = Loop::new();
    let Ok(bus) = crate::toolkit::bus::Bus::session(&events) else { return };
    let owned = Rc::new(Cell::new(false));
    let (taken, let_go) = (owned.clone(), owned.clone());
    let Ok(_hearing) = bus.watch(crate::ring::sni::WATCHER, &events, move |_| taken.set(true), move || let_go.set(false)) else { return };
    // SAFETY: getppid and kill with signal 0 only ask; neither can fail harmfully.
    let compositor = unsafe { libc::getppid() };
    // SAFETY: as above, both calls only ask.
    let alive = move || unsafe { libc::getppid() == compositor && libc::kill(compositor, 0) == 0 };
    let mut starts: Vec<Instant> = Vec::new();
    let mut missing = false;

    let mut waiting: Option<Instant> = None;
    let quit = events.clone();
    events.every(RING_CHECK, move || {
        if !alive() {
            quit.quit();
            return Flow::Stop;
        }

        // SAFETY: waitpid with WNOHANG only collects ended children.
        while unsafe { libc::waitpid(-1, std::ptr::null_mut(), libc::WNOHANG) } > 0 {}
        if waiting.is_some_and(|since| since.elapsed() < TRAY_WAIT) {
            return Flow::Continue;
        }
        waiting = None;
        if owned.get() {
            missing = false;
            return Flow::Continue;
        }
        if !missing {
            missing = true;
            return Flow::Continue;
        }
        starts.retain(|t| t.elapsed() < RING_WINDOW);
        if starts.len() >= RING_TRIES {
            return Flow::Continue;
        }
        starts.push(Instant::now());
        missing = false;
        let _ = own_part(&["trindle-ring", "--background"]).spawn();
        waiting = Some(Instant::now());
        Flow::Continue
    });
    events.run();
}

fn greeting(stick: bool, first: bool) -> Option<&'static [&'static str]> {
    match (stick, first) {
        (true, _) => None,
        (false, true) => Some(&["trindle-welcome", "--first"]),
        (false, false) => Some(&["trindle-ring"]),
    }
}

fn own_part(argv: &[&str]) -> Command {
    let mut part = Command::new(argv[0]);
    part.args(&argv[1..]).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::inherit());
    part
}

pub fn autostart(args: Vec<String>) -> i32 {

    if args.get(1).map(String::as_str) == Some("--folders") {
        super::folders::ensure();
        return 0;
    }

    let door = ring_door();
    set_up_display_and_keys();
    start_parts();

    let kept = matches!(door, Door::Handed(_));
    open_ring(door);
    if let Some(greet) = greeting(crate::common::boot::from_stick(), crate::welcome::first_time()) {
        own(greet);
    }
    start_persons_programs();
    if !kept {
        keep_ring();
    }
    0
}

fn quiet(argv: &[&str]) {
    let _ = Command::new(argv[0]).args(&argv[1..]).stdin(Stdio::null()).stdout(Stdio::null())
        .stderr(Stdio::null()).spawn();
}

fn own(argv: &[&str]) {
    let _ = own_part(argv).spawn();
}

fn set_up_display_and_keys() {

    match crate::common::outputs::saved() {
        Some(settings) => crate::common::outputs::apply_saved(&settings),
        None => {
            super::scale::main(vec!["trindle-scale".into()]);
        }
    }

    if let Ok(heads) = crate::common::outputs::heads() {
        crate::common::outputs::remember_for_login(&heads);
    }

    match crate::common::remap::load() {
        Ok(table) => crate::common::remap::remember_for_login(&table.text()),
        Err(e) => eprintln!("trindle: the changed keys were not kept for the login screen: {e}"),
    }
}

fn start_parts() {
    super::folders::ensure();
    super::folders::terminal();
    crate::common::style::apply_system_theme();

    own(&["trindle-boxed", "--first"]);
    crate::common::sounds::play("login");
    own(&["trindle-wallpaper"]);

    own(&["trindle-starting"]);
    crate::common::clipboard::at_login();
    own(&["trindle-clipboard", "watch"]);

    if crate::keyring::migrate::switched() {
        own(&["trindle-keyring"]);
    }
    own(&["trindle-idle"]);

    own(&["trindle-wsdd"]);

    own(&["trindle-nightlight"]);

    // Waited for: the session's services, started below, need it in place.
    let _ = Command::new("dbus-update-activation-environment")
        .arg("--systemd").args(ACTIVATION)
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();

    if crate::common::screen_reader::on() && crate::common::screen_reader::installed() {
        crate::common::screen_reader::start();
    }

    crate::icons::main(vec!["trindle-icons".into(), "--quiet".into()]);
}

fn open_ring(door: Door) {
    match door {
        Door::Handed(mut door) => {
            use std::io::Write;
            // The door closing says it too, should the word not be written.
            let _ = door.write_all(b"1");
            drop(door);
            wait_for_tray_for(KEPT_RING_WAIT);
        }
        Door::Missing => {

            eprintln!("trindle-autostart: the compositor could not start the ring itself; starting one by name, which says so");
            crate::common::ring_said::leave(crate::common::ring_said::Said::ByName);
            own(&["trindle-ring", "--background"]);
        }
        Door::NotOffered => own(&["trindle-ring", "--background"]),
    }
}

fn start_persons_programs() {

    let flathub = crate::store::catalogue::add_flathub(crate::store::catalogue::REMOTE);
    quiet(&flathub.iter().map(String::as_str).collect::<Vec<_>>());
    wait_for_tray();

    quiet(&["systemctl", "--user", "start", "--no-block", SESSION_TARGET]);
    let waiting = crate::common::autostart::start_all();

    crate::plugins::start_background();
    for t in waiting {
        let _ = t.join();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn a_compositor_asked_to_end_by_a_signal_is_a_clean_end() {
        use std::os::unix::process::ExitStatusExt;
        use std::process::ExitStatus;
        for signal in [libc::SIGTERM, libc::SIGHUP, libc::SIGINT] {
            let status = ExitStatus::from_raw(signal);
            assert!(super::asked_to_end(status), "signal {signal}");
            assert_eq!(super::exit_code(status), 0, "signal {signal}");
        }
        assert_eq!(super::exit_code(ExitStatus::from_raw(0)), 0);

        for signal in [libc::SIGSEGV, libc::SIGKILL, libc::SIGABRT] {
            assert!(!super::asked_to_end(ExitStatus::from_raw(signal)), "signal {signal}");
            assert_eq!(super::exit_code(ExitStatus::from_raw(signal)), 1);
        }
        assert_eq!(super::exit_code(ExitStatus::from_raw(3 << 8)), 3);
        assert!(!super::asked_to_end(ExitStatus::from_raw(1 << 8)));

        let code = crate::common::source::code(include_str!("start.rs")).split_once("#[cfg(test)]").unwrap().0.to_string();
        assert_eq!(code.matches(".code().unwrap_or(1)").count(), 1, "a place reads the end by hand");
    }

    use super::{door_named, door_numbered, greeting, Door};

    #[test]
    fn gtk_programs_are_told_to_use_trindles_file_chooser() {
        let code = crate::common::source::code(include_str!("start.rs"));
        let session = code.split_once("pub fn session(").unwrap().1;
        let told = session.find("gtk_portal();").expect("the session never tells GTK programs to use the portal");
        let compositor = session.find("Command::new(COMPOSE)").unwrap();
        assert!(told < compositor, "GTK programs are told only after the compositor has started");
        assert_eq!(super::GTK_PORTAL, "GTK_USE_PORTAL");
        assert!(super::ACTIVATION.contains(&super::GTK_PORTAL), "D-Bus-started programs are not told");
        let f = code.split_once("fn gtk_portal() {").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(f.contains("var_os(GTK_PORTAL).is_none()"), "a choice of the person's own is overwritten");
    }

    #[test]
    fn the_session_refreshes_the_portal_settings_copy_before_the_compositor() {
        let code = crate::common::source::code(include_str!("start.rs"));
        let session = code.split_once("pub fn session(").unwrap().1;
        let refreshed = session.find("refresh_portals();").expect("the stale portal settings copy is never refreshed");
        assert!(refreshed < session.find("Command::new(COMPOSE)").unwrap());
    }

    #[test]
    fn a_parts_errors_go_to_the_sessions_log() {
        use std::io::{Read, Seek};
        use std::os::fd::AsRawFd;
        use std::os::unix::process::CommandExt;

        let sh = crate::common::needs::program("sh");
        let mut log = tempfile_here("trindle-own-part");
        let fd = log.as_raw_fd();
        let mut part = super::own_part(&[&sh.to_string_lossy(), "-c", "echo to-out; echo the library is not installed >&2"]);
        // SAFETY: between fork and exec the hook only calls dup2, which is

        unsafe {
            part.pre_exec(move || if libc::dup2(fd, 2) < 0 { Err(std::io::Error::last_os_error()) } else { Ok(()) });
        }
        assert!(part.status().unwrap().success());
        let mut said = String::new();
        log.rewind().unwrap();
        log.read_to_string(&mut said).unwrap();
        assert_eq!(said, "the library is not installed\n", "a part's errors, and only its errors, are kept");

        let code = crate::common::source::code(include_str!("start.rs")).split_once("#[cfg(test)]").unwrap().0;
        let own = ["trindle", "-"].concat();
        for line in code.lines().map(str::trim).filter(|l| !l.starts_with("//")) {
            let quietly = line.contains("quiet(&[") || line.contains(&["Command::new(\"", &own].concat());
            assert!(!(quietly && line.contains(&["\"", &own].concat())), "a part of Trindle's is started with its errors thrown away: {line}");
        }
        assert!(code.contains("own(&[\"trindle-wallpaper\"]);"));
        let part = code.split_once("fn own_part(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(part.contains(".stderr(Stdio::inherit())") && !part.contains(".stderr(Stdio::null())"));
    }

    fn tempfile_here(tag: &str) -> std::fs::File {
        let path = std::env::temp_dir().join(format!("{tag}-{}", std::process::id()));
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        file
    }

    #[test]
    fn a_door_that_was_named_and_is_not_there_is_never_taken_for_another_compositor() {

        assert!(matches!(door_named(None), Door::NotOffered));

        for named in ["none", "", "7000", "door"] {
            assert!(matches!(door_named(Some(named)), Door::Missing), "{named:?}");
        }
        let mut ends = [0i32; 2];
        // SAFETY: pipe writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
        assert!(matches!(door_named(Some(&ends[1].to_string())), Door::Handed(_)));
        // SAFETY: the read end was just made by pipe and belongs to nothing else.
        unsafe { libc::close(ends[0]) };
    }

    #[test]
    fn the_door_is_a_pipe_and_nothing_else_and_is_closed_in_what_is_started() {
        use std::os::fd::{AsRawFd, IntoRawFd};
        let mut ends = [0i32; 2];
        // SAFETY: pipe writes two file numbers into the array it is handed.
        assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
        let door = door_numbered(&ends[1].to_string()).expect("a pipe is a door");
        // SAFETY: fcntl only reads the flags of an open file.
        let flags = unsafe { libc::fcntl(door.as_raw_fd(), libc::F_GETFD) };
        assert_eq!(flags & libc::FD_CLOEXEC, libc::FD_CLOEXEC);

        use std::io::{Read, Write};
        (&door).write_all(b"1").unwrap();
        drop(door);
        // SAFETY: the read end was just made by pipe and belongs to nothing else.
        let mut read = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(ends[0]) };
        let mut got = Vec::new();
        read.read_to_end(&mut got).unwrap();
        assert_eq!(got, b"1");

        let file = std::fs::File::open("/proc/self/status").unwrap().into_raw_fd();
        assert!(door_numbered(&file.to_string()).is_none());
        // SAFETY: the number came from into_raw_fd just above and is closed once.
        unsafe { libc::close(file) };
        for bad in ["", "door", "-1", "0", "2", "1048575"] {
            assert!(door_numbered(bad).is_none(), "{bad:?}");
        }
    }

    #[test]
    fn the_first_login_gets_the_welcome_and_every_other_the_ring() {
        assert_eq!(greeting(false, true), Some(&["trindle-welcome", "--first"][..]));
        assert_eq!(greeting(false, false), Some(&["trindle-ring"][..]));
        assert_eq!(greeting(true, true), None);
    }

    #[test]
    fn qt_look_sets_the_theme_only_when_the_person_has_none() {
        let Some(home) = crate::common::scratch_home::inside() else { return crate::common::scratch_home::rerun(module_path!(), "qt_look_sets_the_theme_only_when_the_person_has_none") };
        std::env::remove_var("QT_QPA_PLATFORMTHEME");
        std::env::remove_var("KDE_SESSION_VERSION");
        std::env::set_var("XDG_CONFIG_DIRS", "/etc/xdg");
        super::qt_look();
        assert_eq!(std::env::var("QT_QPA_PLATFORMTHEME").unwrap(), "kde");
        assert_eq!(std::env::var("KDE_SESSION_VERSION").unwrap(), "6");
        let dirs = std::env::var("XDG_CONFIG_DIRS").unwrap();
        assert!(dirs.starts_with(&home.display().to_string()) && dirs.ends_with(":/etc/xdg"), "{dirs}");

        std::env::set_var("QT_QPA_PLATFORMTHEME", "qt6ct");
        std::env::remove_var("KDE_SESSION_VERSION");
        std::env::set_var("XDG_CONFIG_DIRS", "/etc/xdg");
        super::qt_look();
        assert_eq!(std::env::var("QT_QPA_PLATFORMTHEME").unwrap(), "qt6ct");
        assert!(std::env::var_os("KDE_SESSION_VERSION").is_none());
        assert_eq!(std::env::var("XDG_CONFIG_DIRS").unwrap(), "/etc/xdg");
    }
}
