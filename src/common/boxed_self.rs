//! Trindle's own program started again in a bubblewrap box, with nothing of the person's to read by name.

use std::ffi::OsString;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

const TMP: u64 = 16 << 20;

fn program(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into());
    std::env::split_paths(&path).map(|d| d.join(name)).find(|p| p.is_file())
        .and_then(|p| p.canonicalize().ok())
        .filter(|p| p.starts_with("/usr"))
}

fn bwrap() -> Option<&'static PathBuf> {
    static B: OnceLock<Option<PathBuf>> = OnceLock::new();
    B.get_or_init(|| program("bwrap")).as_ref()
}

fn walls_of(more: &[OsString], command: &[OsString]) -> Vec<OsString> {
    let mut a: Vec<OsString> = Vec::new();
    let mut add = |args: &[&str]| a.extend(args.iter().map(OsString::from));
    add(&["--unshare-all", "--new-session", "--die-with-parent", "--clearenv",
          "--setenv", "PATH", "/usr/bin", "--setenv", "LANG", "C.UTF-8",

          "--setenv", "MALLOC_ARENA_MAX", "2",
          "--ro-bind", "/usr", "/usr", "--ro-bind-try", "/etc/ld.so.cache", "/etc/ld.so.cache",

          "--ro-bind-try", "/etc/alternatives", "/etc/alternatives",

          "--ro-bind-try", "/etc/fonts", "/etc/fonts"]);
    for (link, to) in [("/bin", "usr/bin"), ("/sbin", "usr/sbin"), ("/lib", "usr/lib"), ("/lib64", "usr/lib64")] {
        add(&["--symlink", to, link]);
    }
    let tmp = TMP.to_string();
    add(&["--proc", "/proc", "--dev", "/dev", "--size", &tmp, "--tmpfs", "/tmp", "--dir", "/thumb"]);
    a.extend(more.iter().cloned());
    a.extend(["--chdir", "/tmp", "--"].map(OsString::from));
    a.extend(command.iter().cloned());
    a
}

pub(crate) const SELF: &str = "/thumb/trindle";

pub(crate) fn this_program() -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    match exe.to_str().and_then(|e| e.strip_suffix(" (deleted)")) {
        Some(now) if !exe.exists() => Ok(PathBuf::from(now)),
        _ => Ok(exe),
    }
}

pub(crate) fn boxed_self(more: &[OsString], run: &[OsString], memory: u64, cpu: u64) -> Option<Command> {
    let mut all: Vec<OsString> = vec!["--ro-bind".into(), this_program().ok()?.into_os_string(), SELF.into()];
    all.extend(more.iter().cloned());
    let command: Vec<OsString> = std::iter::once(OsString::from(SELF)).chain(run.iter().cloned()).collect();
    held(walls_of(&all, &command), memory, cpu)
}

fn held(walls: Vec<OsString>, memory: u64, cpu: u64) -> Option<Command> {
    let mut cmd = Command::new(bwrap()?);

    let filter = seccomp_fd()?;
    let number = filter.as_raw_fd();
    cmd.args(["--unshare-user", "--disable-userns", "--seccomp"]).arg(number.to_string());
    cmd.args(walls);
    let memory = libc::rlimit { rlim_cur: memory, rlim_max: memory };
    let cpu = libc::rlimit { rlim_cur: cpu, rlim_max: cpu };
    // SAFETY: between fork and exec the hook only calls setrlimit and

    unsafe {
        cmd.pre_exec(move || {
            let _keep = &filter;
            libc::setrlimit(libc::RLIMIT_AS, &memory);
            libc::setrlimit(libc::RLIMIT_CPU, &cpu);

            if libc::fcntl(number, libc::F_SETFD, 0) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    Some(cmd)
}

fn seccomp_fd() -> Option<OwnedFd> {
    let made = crate::common::seccomp::memfd()?;
    // SAFETY: F_DUPFD_CLOEXEC only returns a new descriptor numbered at

    let high = unsafe { libc::fcntl(made.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 10) };
    if high < 0 {
        return None;
    }
    // SAFETY: high is a new descriptor nothing else owns.
    Some(unsafe { OwnedFd::from_raw_fd(high) })
}
