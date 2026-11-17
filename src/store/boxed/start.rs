//! The first thing run in an app's box: it starts the app at the first process number of the box's band (bus.rs).

use std::sync::atomic::{AtomicI32, Ordering};

pub const STARTER: &str = "trindle-box-start";
pub const INSIDE_AT: &str = "/run/trindle-box-start";

const LIBS_AT: &str = "/run/trindle-box-libs";

const LIB_DIRS_MOST: usize = 8;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Host {
    pub exe: String,

    pub loader: Option<String>,
    pub libs: Vec<String>,
}

impl Host {

    pub fn now(exe: String) -> Host {
        // SAFETY: getauxval only reads this process's auxiliary vector.
        let base = unsafe { libc::getauxval(libc::AT_BASE) };
        let maps = std::fs::read_to_string("/proc/self/maps").unwrap_or_default();
        let (loader, libs) = loaded(&maps, base).map_or((None, Vec::new()), |(l, d)| (Some(l), d));
        Host { exe, loader, libs }
    }

    pub fn binds(&self) -> Vec<String> {
        let mut a = vec!["--ro-bind".to_string(), self.exe.clone(), INSIDE_AT.to_string()];
        if self.loader.is_some() {
            for (i, dir) in self.libs.iter().enumerate() {
                a.extend(["--ro-bind".to_string(), dir.clone(), format!("{LIBS_AT}/{i}")]);
            }
        }
        a
    }
}

pub fn loaded(maps: &str, base: u64) -> Option<(String, Vec<String>)> {
    let path = |line: &str| line.find('/').map(|i| line[i..].trim_end_matches(" (deleted)").to_string());
    let start = |line: &str| line.split('-').next().and_then(|a| u64::from_str_radix(a, 16).ok());
    if base == 0 {
        return None;
    }
    let loader = maps.lines().find(|l| start(l) == Some(base)).and_then(path)?;
    let (dir, name) = loader.rsplit_once('/')?;
    let mut dirs = vec![if dir.is_empty() { "/".to_string() } else { dir.to_string() }];
    for lib in maps.lines().filter_map(path) {
        let Some((d, file)) = lib.rsplit_once('/') else { continue };
        if !d.is_empty() && (file.ends_with(".so") || file.contains(".so.")) && !dirs.iter().any(|k| k == d) {
            dirs.push(d.to_string());
        }
    }
    (!name.is_empty() && dirs.len() <= LIB_DIRS_MOST).then(|| (name.to_string(), dirs))
}

pub fn command(first: u32, host: &Host, command: &[String]) -> Vec<String> {
    let mut c = Vec::new();
    if let Some(loader) = &host.loader {
        let path: Vec<String> = (0..host.libs.len()).map(|i| format!("{LIBS_AT}/{i}")).collect();
        c.extend([format!("{LIBS_AT}/0/{loader}"), "--library-path".to_string(), path.join(":")]);
    }
    c.extend([INSIDE_AT.to_string(), first.to_string(), "--".to_string()]);
    c.extend(command.iter().cloned());
    c
}

static APP: AtomicI32 = AtomicI32::new(0);

const PASSED: [i32; 6] = [libc::SIGTERM, libc::SIGINT, libc::SIGHUP, libc::SIGQUIT, libc::SIGUSR1, libc::SIGUSR2];

extern "C" fn pass_on(sig: libc::c_int) {
    let app = APP.load(Ordering::Relaxed);
    if app > 0 {
        // SAFETY: kill is async-signal-safe and sends to the one child only.
        unsafe { libc::kill(app, sig) };
    }
}

pub fn main(args: &[String]) -> i32 {
    let (Some(first), Some("--"), Some(program)) = (args.get(1).and_then(|f| f.parse::<u32>().ok()), args.get(2).map(String::as_str), args.get(3)) else {
        eprintln!("{STARTER}: FIRST -- COMMAND...");
        return 1;
    };
    use_up_to(first);
    let mut child = match std::process::Command::new(program).args(&args[4..]).spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{STARTER}: {program}: {e}");
            return 1;
        }
    };
    APP.store(child.id() as i32, Ordering::Relaxed);
    for sig in PASSED {
        // SAFETY: the handler only reads an atomic and calls kill.
        unsafe { libc::signal(sig, pass_on as *const () as libc::sighandler_t) };
    }
    use std::os::unix::process::ExitStatusExt;
    match child.wait() {
        Ok(s) => s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0)),
        Err(_) => 1,
    }
}

pub fn use_up_to(first: u32) {
    for _ in 0..first {
        // SAFETY: the child calls only _exit; the parent collects it.
        let pid = unsafe { libc::fork() };
        if pid == 0 {
            // SAFETY: _exit is async-signal-safe and ends the child at once.
            unsafe { libc::_exit(0) };
        }
        if pid < 0 {
            return;
        }
        // SAFETY: waits for the one child just made.
        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
        if pid as u32 + 1 >= first {
            return;
        }
    }
}

/// # Safety
/// `argv` holds `argc` nul-ended strings, as glibc gives an `.init_array` function.
#[cfg(test)]
pub unsafe extern "C" fn as_test_program(argc: libc::c_int, argv: *const *const libc::c_char, _env: *const *const libc::c_char) {
    let args: Vec<String> = (0..argc.max(0) as usize)
        // SAFETY: glibc passes argc and argv as main gets them: argc strings, each ended by a nul.
        .map(|i| unsafe { std::ffi::CStr::from_ptr(*argv.add(i)) }.to_string_lossy().into_owned()).collect();
    if args.first().is_some_and(|a| a.rsplit('/').next() == Some(STARTER)) {
        let code = main(&args);
        // SAFETY: ends the process before the test harness runs.
        unsafe { libc::_exit(code) };
    }
}
