//! The fence for AI programs: they run in a room of their own without the person's files.

use crate::toolkit::t;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::common::files::{data_home, home};

pub const DEFAULT_PROGRAMS: &[&str] = &["claude", "codex", "cursor-agent", "gemini", "aider", "opencode", "goose", "amp"];

pub const INSIDE: &str = "trindle-fence-inside";
pub const STAND_INS: &[&str] = &["ssh", "sudo", "pkexec", "doas", "su", SHARE_ASK];

pub const SHARE_ASK: &str = "trindle-share";

pub const TRINDLE_AT: &str = "/tmp/.trindle";
pub const BROKER_ENV: &str = "TRINDLE_FENCE_BROKER";
pub const PROXY_ENV: &str = "TRINDLE_FENCE_PROXY";

const ALWAYS_PRIVATE: &[&str] = &["snap", "Mail", "mail", "Thunderbird", "Passwords", "Keys"];

const HIDDEN_DIRS: &[&str] = &["/run/user", "/run/dbus", "/run/media", "/media", "/mnt", "/var/mail", "/var/spool/mail",
                               "/run/libvirt", "/run/podman", "/run/containerd", "/run/lxd", "/var/lib/docker"];

const HIDDEN_FILES: &[&str] = &["/run/docker.sock", "/run/snapd.socket", "/run/snapd-snap.socket", "/run/containerd/containerd.sock"];

pub fn root() -> PathBuf {
    data_home().join("trindle/ai")
}

pub fn private_home(root: &Path) -> PathBuf {
    root.join("home")
}

pub fn wrappers(root: &Path) -> PathBuf {
    root.join("bin")
}

fn lines(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path).unwrap_or_default().lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

fn not_saved(e: std::io::Error) -> &'static str {

    crate::common::damaged::said(&e).map_or_else(|| t!("store-fence-not-saved"), |said| Box::leak(said.into_boxed_str()))
}

fn write(path: &Path, lines: &[String]) -> std::io::Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }

    crate::common::damaged::may_write(path, t!("common-unreadable-name-fence"))?;
    crate::common::files::write_text(path, &if lines.is_empty() { String::new() } else { lines.join("\n") + "\n" })
}

pub fn program_name_ok(name: &str) -> bool {
    !name.is_empty() && name.len() <= 64 && !name.starts_with(['-', '.'])
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && !STAND_INS.contains(&name) && !matches!(name, "bash" | "sh" | "zsh" | "env" | "trindle-boxed")
}

pub fn programs(root: &Path) -> Vec<(String, bool)> {
    let file = root.join("programs");
    if file.exists() {
        lines(&file).into_iter().filter_map(|l| {
            let (n, off) = match l.strip_suffix(" off") {
                Some(n) => (n.trim(), true),
                None => (l.as_str(), false),
            };
            program_name_ok(n).then(|| (n.to_string(), !off))
        }).collect()
    } else {
        DEFAULT_PROGRAMS.iter().map(|s| (s.to_string(), true)).collect()
    }
}

pub fn set_programs(root: &Path, list: &[(String, bool)]) -> Result<(), &'static str> {
    let mut clean: Vec<(String, bool)> = Vec::new();
    for (n, on) in list {
        let n = n.trim();
        if !program_name_ok(n) {
            return Err(t!("store-fence-bad-name"));
        }
        if !clean.iter().any(|c| c.0 == n) {
            clean.push((n.to_string(), *on));
        }
    }
    let text: Vec<String> = clean.iter().map(|(n, on)| if *on { n.clone() } else { format!("{n} off") }).collect();
    write(&root.join("programs"), &text).map_err(not_saved)?;
    sync_wrappers(root);
    Ok(())
}

pub fn enabled(root: &Path) -> bool {
    !root.join("fence-off").exists()
}

pub fn set_enabled(root: &Path, on: bool) -> std::io::Result<()> {
    std::fs::create_dir_all(root)?;
    if on {
        match std::fs::remove_file(root.join("fence-off")) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    } else {
        crate::common::files::write_whole(&root.join("fence-off"), b"")
    }
}

pub fn fenced(root: &Path, name: &str) -> bool {
    enabled(root) && programs(root).iter().any(|(n, on)| n == name && *on)
}

pub fn sync_wrappers(root: &Path) {
    let dir = wrappers(root);
    let want: Vec<String> = programs(root).into_iter().map(|(n, _)| n).collect();
    let _ = std::fs::create_dir_all(&dir);
    let Ok(exe) = std::env::current_exe() else { return };
    for e in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !want.contains(&name) {
            let _ = std::fs::remove_file(e.path());
        }
    }
    for name in want {
        let link = dir.join(&name);
        if std::fs::read_link(&link).ok().as_deref() != Some(exe.as_path()) {
            let _ = std::fs::remove_file(&link);
            let _ = std::os::unix::fs::symlink(&exe, &link);
        }
    }
}

pub fn judge(canonical: &Path, home: &Path, is_dir: bool) -> Result<(), &'static str> {
    if !is_dir {
        return Err(t!("store-fence-no-folder"));
    }
    if canonical == home {
        return Err(t!("store-fence-whole-home"));
    }
    if home.starts_with(canonical) {
        return Err(t!("store-fence-holds-home"));
    }
    let Ok(under) = canonical.strip_prefix(home) else {
        return Err(t!("store-fence-outside-home"));
    };
    let parts: Vec<String> = under.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    if parts.iter().any(|p| p.starts_with('.')) {
        return Err(t!("store-fence-hidden"));
    }
    if parts.first().is_some_and(|f| ALWAYS_PRIVATE.iter().any(|a| a.eq_ignore_ascii_case(f))) {
        return Err(t!("store-fence-passwords"));
    }
    Ok(())
}

pub fn shareable(path: &Path, home: &Path) -> Result<PathBuf, &'static str> {
    let real = std::fs::canonicalize(path).map_err(|_| t!("store-fence-no-folder"))?;
    let home = std::fs::canonicalize(home).unwrap_or_else(|_| home.to_path_buf());
    judge(&real, &home, real.is_dir())?;
    Ok(real)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Share {
    pub path: PathBuf,
    pub read_only: bool,
}

fn share_line(s: &Share) -> String {
    format!("{}{}", if s.read_only { "ro:" } else { "" }, s.path.display())
}

fn read_shares(file: &Path, home: &Path) -> Vec<Share> {
    let mut out: Vec<Share> = Vec::new();
    for l in lines(file) {
        let (read_only, p) = match l.strip_prefix("ro:") {
            Some(p) => (true, p),
            None => (false, l.as_str()),
        };

        if let Ok(real) = shareable(Path::new(p), home) {
            if real == Path::new(p) && !out.iter().any(|s| s.path == real) {
                out.push(Share { path: real, read_only });
            }
        }
    }
    out
}

pub fn shared(root: &Path, home: &Path) -> Vec<Share> {
    read_shares(&root.join("shared"), home)
}

pub fn take_once(root: &Path, home: &Path) -> Vec<Share> {
    let file = root.join("shared-once");
    let out = read_shares(&file, home);
    let _ = std::fs::remove_file(&file);
    out
}

pub fn share(root: &Path, home: &Path, path: &Path, read_only: bool, always: bool) -> Result<PathBuf, &'static str> {
    let real = shareable(path, home)?;
    let file = root.join(if always { "shared" } else { "shared-once" });
    let mut now: Vec<String> = lines(&file).into_iter().filter(|l| l.strip_prefix("ro:").unwrap_or(l) != real.to_string_lossy()).collect();
    now.push(share_line(&Share { path: real.clone(), read_only }));
    write(&file, &now).map_err(not_saved)?;
    Ok(real)
}

pub fn unshare(root: &Path, path: &Path) -> std::io::Result<()> {
    let s = path.to_string_lossy().into_owned();
    let now: Vec<String> = lines(&root.join("shared")).into_iter().filter(|l| l.strip_prefix("ro:").unwrap_or(l) != s).collect();
    write(&root.join("shared"), &now)
}

pub fn own_settings_of(name: &str) -> &'static [&'static str] {
    match name {
        "claude" => &[".claude", ".claude.json"],
        "codex" => &[".codex"],
        "gemini" => &[".gemini"],
        "cursor-agent" => &[".cursor"],
        "opencode" => &[".config/opencode", ".local/share/opencode"],
        "goose" => &[".config/goose", ".local/share/goose"],
        "amp" => &[".config/amp", ".local/share/amp"],
        "aider" => &[".aider"],
        _ => &[],
    }
}

pub fn uses_own_settings(root: &Path) -> Vec<String> {
    lines(&root.join("own-settings"))
}

pub fn set_uses_own_settings(root: &Path, name: &str, on: bool) -> Result<(), &'static str> {
    if own_settings_of(name).is_empty() {
        return Err(t!("store-fence-no-settings"));
    }
    let mut now = uses_own_settings(root);
    now.retain(|n| n != name);
    if on {
        now.push(name.to_string());
    }
    write(&root.join("own-settings"), &now).map_err(not_saved)
}

pub fn own_settings_binds(root: &Path, home: &Path, name: &str) -> Vec<PathBuf> {
    if !uses_own_settings(root).iter().any(|n| n == name) {
        return Vec::new();
    }
    own_settings_of(name).iter().map(|r| home.join(r)).filter(|p| {
        std::fs::symlink_metadata(p).is_ok_and(|m| !m.file_type().is_symlink()) && !never_offered(home, p)
    }).collect()
}

fn never_offered(home: &Path, p: &Path) -> bool {
    const NEVER: &[&str] = &[".ssh", ".gnupg", ".local/share/keyrings", ".mozilla", ".thunderbird", ".config/google-chrome",
                             ".config/chromium", ".config/BraveSoftware", ".var", ".password-store", ".aws", ".kube"];
    NEVER.iter().any(|n| p.starts_with(home.join(n)) || home.join(n).starts_with(p))
}

pub fn net_limit(root: &Path) -> Option<Vec<String>> {
    root.join("net-limit").exists().then(|| net_hosts(root))
}

pub fn set_net_limit(root: &Path, on: bool) -> std::io::Result<()> {
    std::fs::create_dir_all(root)?;
    if on {
        crate::common::files::write_whole(&root.join("net-limit"), b"")
    } else {
        match std::fs::remove_file(root.join("net-limit")) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }
}

pub fn net_hosts(root: &Path) -> Vec<String> {
    lines(&root.join("net-hosts")).into_iter().filter(|h| super::fence_net::host_pattern_ok(h)).map(|h| h.to_ascii_lowercase()).collect()
}

pub fn set_net_hosts(root: &Path, list: &[String]) -> Result<(), &'static str> {
    let mut clean: Vec<String> = Vec::new();
    for h in list {
        let h = h.trim().to_ascii_lowercase();
        if !super::fence_net::host_pattern_ok(&h) {
            return Err(t!("store-fence-bad-site"));
        }
        if !clean.contains(&h) {
            clean.push(h);
        }
    }
    write(&root.join("net-hosts"), &clean).map_err(not_saved)
}

pub fn host_ok(h: &str) -> bool {
    let h = h.to_ascii_lowercase();
    !h.is_empty() && h.len() <= crate::common::dns::NAME_MOST && !h.starts_with(['-', '.']) && !h.ends_with('.')
        && h.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_'))
        && h.parse::<std::net::IpAddr>().is_err()
}

pub fn ssh_hosts(root: &Path) -> Vec<String> {
    lines(&root.join("ssh-hosts")).into_iter().filter(|h| host_ok(h)).map(|h| h.to_ascii_lowercase()).collect()
}

pub fn set_ssh_hosts(root: &Path, list: &[String]) -> Result<(), &'static str> {
    let mut clean: Vec<String> = Vec::new();
    for h in list {
        let h = h.trim().to_ascii_lowercase();
        if !host_ok(&h) {
            return Err(t!("store-fence-bad-computer"));
        }
        if !clean.contains(&h) {
            clean.push(h);
        }
    }
    write(&root.join("ssh-hosts"), &clean).map_err(not_saved)
}

const SAFE_NAMES: &[&str] = &[
    "PATH", "HOME", "USER", "LOGNAME", "SHELL", "LANG", "LANGUAGE", "TZ", "TERM", "COLORTERM", "TERMINFO", "TERM_PROGRAM",
    "XDG_SESSION_TYPE", "XDG_CURRENT_DESKTOP", "XDG_DATA_DIRS", "XDG_CONFIG_DIRS", "NO_COLOR", "EDITOR", "VISUAL", "PAGER",
    "COLUMNS", "LINES", "TMPDIR",
];

pub fn secret_name(name: &str) -> bool {
    let n = name.to_ascii_uppercase();
    ["KEY", "TOKEN", "SECRET", "PASSWORD", "PASSWD", "PASS", "CREDENTIAL", "AUTH", "COOKIE", "SESSION", "SOCK", "PRIVATE"]
        .iter().any(|w| n.contains(w))
}

pub fn clean_env(host: &[(String, String)]) -> Vec<(String, String)> {
    host.iter()
        .filter(|(k, v)| (SAFE_NAMES.contains(&k.as_str()) || k.starts_with("LC_")) && !secret_name(k) && !v.contains('\0') && !k.contains('='))
        .cloned()
        .collect()
}

fn reserved(name: &str) -> bool {
    name.starts_with("LD_") || name.starts_with("TRINDLE_") || matches!(name, "PATH" | "HOME" | "BASH_ENV" | "ENV" | "IFS" | "PS4"
        | "PROMPT_COMMAND" | "SHELLOPTS" | "BASHOPTS" | "PYTHONSTARTUP" | "NODE_OPTIONS" | "XDG_RUNTIME_DIR" | "DISPLAY" | "WAYLAND_DISPLAY")
}

pub fn typed_env(root: &Path) -> Vec<(String, String)> {
    lines(&root.join("env")).into_iter().filter_map(|l| {
        let (k, v) = l.split_once('=')?;
        let ok = !k.is_empty() && k.len() <= 64 && k.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            && !k.starts_with(|c: char| c.is_ascii_digit()) && !reserved(k);
        ok.then(|| (k.to_string(), v.to_string()))
    }).collect()
}

pub fn set_typed_env(root: &Path, text: &str) -> Result<(), &'static str> {
    let mut keep = Vec::new();
    for l in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let Some((k, _)) = l.split_once('=') else { return Err(t!("store-fence-bad-env")) };
        if k.is_empty() || !k.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') || reserved(k) {
            return Err(t!("store-fence-bad-env-capitals"));
        }
        keep.push(l.to_string());
    }

    let text = if keep.is_empty() { String::new() } else { keep.join("\n") + "\n" };
    std::fs::create_dir_all(root).map_err(|_| t!("store-fence-not-saved"))?;
    crate::common::files::write_whole_private(&root.join("env"), text.as_bytes()).map_err(|_| t!("store-fence-not-saved"))?;
    Ok(())
}

pub struct Fence {

    pub home: PathBuf,
    pub private: PathBuf,
    pub shared: Vec<Share>,

    pub trindle: PathBuf,
    pub broker: Option<PathBuf>,

    pub program: Option<PathBuf>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub seccomp: i32,

    pub secrets: Option<i32>,

    pub hide_dirs: Vec<PathBuf>,
    pub hide_files: Vec<PathBuf>,

    pub no_network: bool,

    pub proxy: Option<PathBuf>,
    pub extra: Vec<String>,
}

pub fn hidden_here(exists: impl Fn(&Path) -> bool) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let dirs = HIDDEN_DIRS.iter().map(PathBuf::from).filter(|p| exists(p)).collect();
    let files = HIDDEN_FILES.iter().map(PathBuf::from).filter(|p| exists(p)).collect();
    (dirs, files)
}

pub fn walls(f: &Fence, command: &[String]) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();
    let mut add = |args: &[&str]| a.extend(args.iter().map(|s| s.to_string()));
    add(&["--unshare-user", "--unshare-pid", "--unshare-ipc", "--unshare-uts", "--unshare-cgroup-try", "--new-session", "--die-with-parent"]);
    if f.no_network {
        add(&["--unshare-net"]);
    }
    add(&["--disable-userns", "--seccomp", &f.seccomp.to_string()]);
    add(&["--ro-bind", "/", "/", "--dev", "/dev", "--proc", "/proc", "--tmpfs", "/tmp"]);
    for d in &f.hide_dirs {
        add(&["--tmpfs", &d.to_string_lossy()]);
    }
    for p in &f.hide_files {
        add(&["--ro-bind", "/dev/null", &p.to_string_lossy()]);
    }
    let home = f.home.to_string_lossy().into_owned();
    if f.home.starts_with("/home") {
        add(&["--tmpfs", "/home"]);
    }
    add(&["--bind", &f.private.to_string_lossy(), &home]);
    for s in &f.shared {
        let p = s.path.to_string_lossy();
        add(&[if s.read_only { "--ro-bind" } else { "--bind" }, &p, &p]);
    }
    if let Some(p) = &f.program {
        let p = p.to_string_lossy();
        add(&["--ro-bind", &p, &p]);
    }
    let t = TRINDLE_AT;
    add(&["--dir", t, "--dir", &format!("{t}/bin"), "--ro-bind", &f.trindle.to_string_lossy(), &format!("{t}/{INSIDE}")]);
    for name in STAND_INS {
        add(&["--ro-bind", &f.trindle.to_string_lossy(), &format!("{t}/bin/{name}")]);
    }
    if let Some(b) = &f.broker {
        add(&["--ro-bind", &b.to_string_lossy(), &format!("{t}/broker.sock")]);
    }
    if let Some(p) = &f.proxy {
        add(&["--ro-bind", &p.to_string_lossy(), &format!("{t}/proxy.sock")]);
    }
    let more: Vec<&str> = f.extra.iter().map(String::as_str).collect();
    add(&more);
    add(&["--clearenv"]);
    for (k, v) in &f.env {
        add(&["--setenv", k, v]);
    }
    if let Some(fd) = f.secrets {
        add(&["--args", &fd.to_string()]);
    }
    if f.proxy.is_some() {
        add(&["--setenv", PROXY_ENV, &format!("{t}/proxy.sock")]);
    }
    add(&["--setenv", BROKER_ENV, &format!("{t}/broker.sock"), "--chdir", &f.cwd.to_string_lossy(), "--"]);
    a.push(format!("{t}/{INSIDE}"));
    a.extend(command.iter().cloned());
    a
}

pub fn env_for(host: &[(String, String)], home: &Path, wrappers: &Path) -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = clean_env(host).into_iter().filter(|(k, _)| k != "PATH" && k != "HOME").collect();
    let path = host.iter().find(|(k, _)| k == "PATH").map(|(_, v)| v.as_str()).unwrap_or("/usr/local/bin:/usr/bin:/bin");
    let kept: Vec<&str> = path.split(':').filter(|p| !p.is_empty() && !Path::new(p).starts_with(home) && Path::new(p) != wrappers).collect();
    let p = format!("{TRINDLE_AT}/bin:{}/.local/bin:{}", home.display(), if kept.is_empty() { "/usr/local/bin:/usr/bin:/bin".to_string() } else { kept.join(":") });
    env.push(("PATH".into(), p));
    env.push(("HOME".into(), home.to_string_lossy().into_owned()));

    env.push(("npm_config_prefix".into(), format!("{}/.local", home.display())));
    env.push(("PYTHONUSERBASE".into(), format!("{}/.local", home.display())));
    env.push(("PIP_USER".into(), "1".into()));
    env.push(("CARGO_HOME".into(), format!("{}/.cargo", home.display())));
    env
}

fn landlock_version() -> i64 {
    // SAFETY: the call with a null attribute and the version flag only reads.
    unsafe { libc::syscall(444, std::ptr::null::<u8>(), 0usize, 1u32) }
}

pub fn scope_abstract_sockets() -> Result<(), String> {
    let v = landlock_version();
    if v < 6 {
        return Err("this computer's kernel is too old to keep programs off its abstract sockets (needs Linux 6.12)".into());
    }

    let attr: [u64; 3] = [0, 0, 1];
    // SAFETY: attr is a valid landlock_ruleset_attr of 24 bytes.
    let fd = unsafe { libc::syscall(444, attr.as_ptr(), std::mem::size_of_val(&attr), 0u32) };
    if fd < 0 {
        return Err(format!("Landlock refused: {}", std::io::Error::last_os_error()));
    }
    // SAFETY: plain prctl and landlock_restrict_self on the descriptor just made.
    unsafe {
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return Err("no_new_privs failed".into());
        }
        let r = libc::syscall(446, fd as i32, 0u32);
        libc::close(fd as i32);
        if r != 0 {
            return Err(format!("Landlock refused: {}", std::io::Error::last_os_error()));
        }
    }
    Ok(())
}

pub fn start_bridge(socket: &Path) {
    // SAFETY: fork in a process with one thread; the child only runs the bridge and never returns.
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        super::fence_net::bridge(socket);
    }
    for _ in 0..100 {
        if std::net::TcpStream::connect(("127.0.0.1", super::fence_net::PROXY_PORT)).is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

pub fn inside(args: &[String]) -> i32 {
    use std::os::unix::process::CommandExt;
    if let Err(e) = scope_abstract_sockets() {
        eprintln!("The fence did not start: {e}.");
        return 1;
    }
    if let Some(sock) = std::env::var_os(PROXY_ENV) {
        start_bridge(Path::new(&sock));
    }
    let Some(first) = args.get(1) else { return 2 };
    let e = std::process::Command::new(first).args(&args[2..]).exec();
    eprintln!("{first}: {e}");
    127
}

pub fn find_program(name: &str, wrappers: &Path, path: &str) -> Option<PathBuf> {
    let direct = name.contains('/');
    let cands: Vec<PathBuf> = if direct {
        vec![PathBuf::from(name)]
    } else {
        path.split(':').filter(|d| !d.is_empty() && Path::new(d) != wrappers).map(|d| Path::new(d).join(name)).collect()
    };
    cands.into_iter().filter_map(|c| std::fs::canonicalize(c).ok()).find(|c| c.is_file())
}

pub fn start_dir(cwd: &Path, shared: &[Share], home: &Path) -> (PathBuf, bool) {
    let real = std::fs::canonicalize(cwd).unwrap_or_else(|_| cwd.to_path_buf());
    if shared.iter().any(|s| real.starts_with(&s.path)) {
        (real, true)
    } else {
        (home.to_path_buf(), false)
    }
}

fn memfd_of(bytes: &[u8]) -> Option<std::os::fd::OwnedFd> {
    use std::io::{Seek, Write};
    use std::os::fd::FromRawFd;
    // SAFETY: memfd_create takes a NUL-terminated name and flags.
    let fd = unsafe { libc::memfd_create(c"trindle-fence".as_ptr(), 0) };
    if fd < 0 {
        return None;
    }
    // SAFETY: fd is new and owned by nothing else.
    let mut f = unsafe { std::fs::File::from_raw_fd(fd) };
    f.write_all(bytes).ok()?;
    f.rewind().ok()?;
    Some(f.into())
}

fn keep_open(fd: &impl std::os::fd::AsFd) {
    use std::os::fd::AsRawFd;
    let raw = fd.as_fd().as_raw_fd();
    // SAFETY: only the close-on-exec flag changes.
    unsafe {
        let flags = libc::fcntl(raw, libc::F_GETFD);
        if flags >= 0 {
            libc::fcntl(raw, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
        }
    }
}

pub fn secret_args(typed: &[(String, String)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (k, v) in typed {
        for part in ["--setenv", k, v] {
            out.extend_from_slice(part.as_bytes());
            out.push(0);
        }
    }
    out
}

pub struct Start<'a> {
    pub root: &'a Path,
    pub home: PathBuf,
    pub host_env: Vec<(String, String)>,
    pub cwd: PathBuf,
    pub trindle: PathBuf,

    pub command: Vec<String>,
    pub program: Option<PathBuf>,
    pub approver: Option<super::fence_broker::Approver>,
    pub extra_env: Vec<(String, String)>,

    pub name: Option<String>,

    pub run_dir: Option<PathBuf>,

    pub net_test: Option<(super::fence_net::Resolver, bool, u16)>,
}

pub fn run(s: &Start) -> i32 {
    let private = private_home(s.root);
    if let Err(e) = std::fs::create_dir_all(&private) {
        eprintln!("The fence did not start: {}: {e}", private.display());
        return 1;
    }
    let first = !s.root.join("seen").exists();
    let _ = crate::common::files::write_whole(&s.root.join("seen"), b"");
    if first {
        eprintln!("AI programs keep their own sign-in here, apart from yours: sign in inside when asked.");
    }
    let mut shared = shared(s.root, &s.home);
    for once in take_once(s.root, &s.home) {
        if !shared.iter().any(|x| x.path == once.path) {
            shared.push(once);
        }
    }
    let (cwd, in_shared) = start_dir(&s.cwd, &shared, &s.home);
    if !in_shared {
        eprintln!("Your folders are private from AI programs. Share one in Settings, AI programs, to work in it.");
    }
    let Some(seccomp) = crate::common::seccomp::memfd() else {
        eprintln!("The fence did not start: the system call filter could not be made.");
        return 1;
    };
    keep_open(&seccomp);
    let typed = typed_env(s.root);
    let secrets = if typed.is_empty() { None } else { memfd_of(&secret_args(&typed)) };
    if let Some(fd) = &secrets {
        keep_open(fd);
    }
    let run_dir = s.run_dir.clone().unwrap_or_else(|| crate::store::pulse_guard::runtime_dir().join("trindle-fence")).join(std::process::id().to_string());
    let _ = std::fs::remove_dir_all(&run_dir);
    let _ = std::fs::create_dir_all(&run_dir);
    let broker = super::fence_broker::start(&run_dir, s.root, &s.home, s.approver.clone());
    let mut env = env_for(&s.host_env, &s.home, &wrappers(s.root));

    let limit = net_limit(s.root);
    let proxy = match &limit {
        Some(hosts) => match start_proxy(s, hosts, &run_dir, &mut env) {
            Some(socket) => Some(socket),
            None => return 1,
        },
        None => None,
    };
    env.extend(s.extra_env.iter().cloned());
    let (hide_dirs, hide_files) = hidden_here(|p| std::fs::symlink_metadata(p).is_ok_and(|m| !m.file_type().is_symlink()));
    let program = s.program.as_ref().filter(|p| p.starts_with(&s.home)).cloned();
    let fence = Fence {
        home: s.home.clone(), private, shared, trindle: s.trindle.clone(), broker, program, cwd, env,
        seccomp: std::os::fd::AsRawFd::as_raw_fd(&seccomp), secrets: secrets.as_ref().map(std::os::fd::AsRawFd::as_raw_fd),
        hide_dirs, hide_files, no_network: limit.is_some(), proxy, extra: own_settings_args(s),
    };
    let args: Vec<OsString> = walls(&fence, &s.command).into_iter().map(OsString::from).collect();
    let status = std::process::Command::new("bwrap").args(args).status();
    let _ = std::fs::remove_dir_all(&run_dir);
    drop((seccomp, secrets));
    exit_code(status)
}

fn start_proxy(s: &Start, hosts: &[String], run_dir: &Path, env: &mut Vec<(String, String)>) -> Option<PathBuf> {
    let socket = run_dir.join("proxy.sock");
    let Ok(listener) = std::os::unix::net::UnixListener::bind(&socket) else {
        eprintln!("The fence did not start: the network limit could not be set up.");
        return None;
    };
    let (resolver, local, port) = match s.net_test.clone() {
        Some((r, l, p)) => (Some(r), l, p),
        None => (None, false, 443),
    };
    let policy = super::fence_net::Policy { hosts: hosts.to_vec(), ports: vec![port], allow_local: local };
    std::thread::spawn(move || super::fence_net::serve(listener, policy, resolver));
    env.extend(super::fence_net::proxy_env());
    Some(socket)
}

fn own_settings_args(s: &Start) -> Vec<String> {
    let name = s.program.as_ref().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let mut extra: Vec<String> = Vec::new();
    for p in own_settings_binds(s.root, &s.home, s.name.as_deref().unwrap_or(&name)) {
        extra.extend(["--bind".to_string(), p.to_string_lossy().into_owned(), p.to_string_lossy().into_owned()]);
    }
    extra
}

fn exit_code(status: std::io::Result<std::process::ExitStatus>) -> i32 {
    match status {
        Ok(st) => {
            use std::os::unix::process::ExitStatusExt;
            st.code().unwrap_or_else(|| 128 + st.signal().unwrap_or(0))
        }
        Err(e) => {
            eprintln!("The fence did not start: bubblewrap would not run: {e}");
            1
        }
    }
}

pub fn share_main(path: &str) -> i32 {
    match share(&root(), &home(), Path::new(path), false, true) {
        Ok(_) => 0,
        Err(why) => {
            eprintln!("{why}");
            1
        }
    }
}

pub fn main_for(name: &str, args: &[String]) -> i32 {
    use std::os::unix::process::CommandExt;
    let root = root();
    let path = std::env::var("PATH").unwrap_or_default();
    let Some(program) = find_program(name, &wrappers(&root), &path) else {
        eprintln!("{name}: not found outside the fence");
        return 127;
    };
    if !fenced(&root, name) {
        let e = std::process::Command::new(&program).args(args).exec();
        eprintln!("{name}: {e}");
        return 127;
    }
    let Ok(trindle) = std::env::current_exe() else { return 1 };
    let mut command = vec![program.to_string_lossy().into_owned()];
    command.extend(args.iter().cloned());
    run(&Start {
        root: &root, home: home(), host_env: std::env::vars().collect(),
        cwd: std::env::current_dir().unwrap_or_else(|_| home()), trindle, command, program: Some(program),
        approver: None, extra_env: Vec::new(), name: Some(name.to_string()), run_dir: None, net_test: None,
    })
}

#[cfg(test)]
mod tests;
