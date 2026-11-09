//! What the fence lets an AI program ask Trindle to do outside it.

use std::io::Write;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};

use super::copied_guard::{recv_message, send_message};
use super::fence::{host_ok, ssh_hosts};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    No,
    Once(bool),
    Always(bool),
}

pub type Approver = Arc<dyn Fn(&str, &str) -> Answer + Send + Sync>;

const BODY_MAX: usize = 60_000;

static ASKING: Mutex<()> = Mutex::new(());

pub fn start(run_dir: &Path, root: &Path, home: &Path, approver: Option<Approver>) -> Option<PathBuf> {
    let socket = run_dir.join("broker.sock");
    let _ = std::fs::remove_file(&socket);
    let listener = UnixListener::bind(&socket).ok()?;
    let (root, home) = (root.to_path_buf(), home.to_path_buf());
    let approver: Approver = approver.unwrap_or_else(|| Arc::new(window));
    std::thread::spawn(move || {
        for conn in listener.incoming().flatten() {
            let (root, home, approver) = (root.clone(), home.clone(), approver.clone());
            std::thread::spawn(move || serve(conn, &root, &home, &approver));
        }
    });
    Some(socket)
}

fn admin_program() -> std::ffi::OsString {
    #[cfg(test)]
    if let Some(p) = tests::ADMIN.with(|a| a.borrow().clone()) {
        return p.into();
    }
    "pkexec".into()
}

fn admin_command(argv: Vec<String>) -> std::process::Command {
    let mut c = std::process::Command::new(admin_program());
    c.args(argv).current_dir("/");
    c
}

fn window(kind: &str, text: &str) -> Answer {
    use std::os::unix::process::CommandExt;
    let Ok(exe) = std::env::current_exe() else { return Answer::No };
    let code = std::process::Command::new(exe).arg0("trindle-boxed").args(["--ai-approve", kind, text]).status().ok().and_then(|s| s.code());
    match code {
        Some(0) | Some(11) => Answer::Always(false),
        Some(10) => Answer::Once(false),
        Some(12) => Answer::Once(true),
        Some(13) => Answer::Always(true),
        _ => Answer::No,
    }
}

fn serve(conn: UnixStream, root: &Path, home: &Path, approver: &Approver) {
    let Ok(Some((body, fds))) = recv_message(conn.as_raw_fd(), BODY_MAX) else { return };
    let mut fds = fds.into_iter();
    let (Some(i), Some(o), Some(e)) = (fds.next(), fds.next(), fds.next()) else { return };
    let words: Vec<String> = body.split(|b| *b == 0).map(|w| String::from_utf8_lossy(w).into_owned()).collect();
    let code = match words.split_first() {
        Some((kind, rest)) => handle(kind, rest, root, home, approver, (i, o, e.try_clone().ok())),
        None => 2,
    };
    let _ = send_message(conn.as_raw_fd(), code.to_string().as_bytes(), &[]);
}

fn tell(e: &Option<OwnedFd>, text: &str) {
    if let Some(fd) = e.as_ref().and_then(|f| f.try_clone().ok()) {
        let _ = std::fs::File::from(fd).write_all(format!("{text}\n").as_bytes());
    }
}

fn handle(kind: &str, args: &[String], root: &Path, home: &Path, approver: &Approver, io: (OwnedFd, OwnedFd, Option<OwnedFd>)) -> i32 {
    let (i, o, e) = io;
    let err = e.as_ref().and_then(|f| f.try_clone().ok());
    let cmd = match kind {
        "ssh" => match parse_ssh(args, &ssh_hosts(root)) {
            Ok(argv) => match ssh_command(root, &argv) {
                Ok(c) => c,
                Err(m) => {
                    tell(&err, &m);
                    return 255;
                }
            },
            Err(m) => {
                tell(&err, &m);
                return 255;
            }
        },
        "sudo" | "pkexec" => match parse_sudo(kind, args) {
            Ok(argv) => {
                let text = approval_text(&argv);
                let Ok(_one) = ASKING.try_lock() else {
                    tell(&err, "Another request is waiting for you to answer it.");
                    return 1;
                };
                if approver("sudo", &text) == Answer::No {
                    tell(&err, "You did not allow this.");
                    return 1;
                }
                admin_command(argv)
            }
            Err(m) => {
                tell(&err, &m);
                return 1;
            }
        },
        "share" => return share_request(args, root, home, approver, &err),
        other => {
            tell(&err, &format!("{other} is not available inside the fence."));
            return 1;
        }
    };
    let mut cmd = cmd;
    cmd.stdin(Stdio::from(i)).stdout(Stdio::from(o)).stderr(e.map_or_else(Stdio::null, Stdio::from));
    match cmd.status() {
        Ok(s) => {
            use std::os::unix::process::ExitStatusExt;
            s.code().unwrap_or_else(|| 128 + s.signal().unwrap_or(0))
        }
        Err(_) => {
            tell(&err, "It could not be started outside the fence.");
            127
        }
    }
}

fn share_request(args: &[String], root: &Path, home: &Path, approver: &Approver, err: &Option<OwnedFd>) -> i32 {
    let Some(asked) = args.first().filter(|a| a.starts_with('/') && !a.contains('\0')) else {
        tell(err, "Give the full path of the folder.");
        return 1;
    };
    let real = match super::fence::shareable(Path::new(asked), home) {
        Ok(r) => r,
        Err(why) => {
            tell(err, why);
            return 1;
        }
    };
    let Ok(_one) = ASKING.try_lock() else {
        tell(err, "Another request is waiting for you to answer it.");
        return 1;
    };
    let (read_only, always) = match approver("share", &real.to_string_lossy()) {
        Answer::No => {
            tell(err, "You did not share it.");
            return 1;
        }
        Answer::Once(r) => (r, false),
        Answer::Always(r) => (r, true),
    };
    match super::fence::share(root, home, &real, read_only, always) {
        Ok(_) => {
            tell(err, "Shared. It is there the next time this program starts.");
            0
        }
        Err(why) => {
            tell(err, why);
            1
        }
    }
}

fn plain_user(u: &str) -> bool {
    !u.is_empty() && u.len() <= 64 && !u.starts_with('-') && u.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

fn digits(v: &str) -> bool {
    !v.is_empty() && v.len() <= 6 && v.chars().all(|c| c.is_ascii_digit())
}

fn ssh_option(raw: &str) -> Result<String, String> {
    let (name, value) = raw.split_once(['=', ' ']).map(|(n, v)| (n.trim(), v.trim().trim_matches('"'))).unwrap_or((raw.trim(), ""));
    let lower = name.to_ascii_lowercase();
    let ok = match lower.as_str() {
        "connecttimeout" | "serveraliveinterval" | "serveralivecountmax" | "port" | "connectionattempts" => digits(value),
        "batchmode" | "tcpkeepalive" | "compression" => matches!(value, "yes" | "no"),
        "clearallforwardings" => value == "yes",
        "forwardagent" | "forwardx11" | "permitlocalcommand" | "forwardx11trusted" | "gssapiauthentication" => value == "no",
        "loglevel" => matches!(value, "QUIET" | "FATAL" | "ERROR" | "INFO" | "VERBOSE"),
        "requesttty" => matches!(value, "yes" | "no" | "force" | "auto"),
        "user" => plain_user(value),
        _ => false,
    };
    if ok { Ok(format!("{name}={value}")) } else { Err(format!("ssh option {name} is not allowed inside the fence.")) }
}

pub fn parse_ssh(args: &[String], hosts: &[String]) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    let mut target: Option<String> = None;
    while i < args.len() {
        let a = &args[i];
        i += 1;
        if a == "--" {
            target = args.get(i).cloned();
            i += 1;
            break;
        }
        let Some(flags) = a.strip_prefix('-').filter(|f| !f.is_empty()) else {
            target = Some(a.clone());
            break;
        };
        let chars: Vec<char> = flags.chars().collect();
        let mut c = 0;
        while c < chars.len() {
            let f = chars[c];
            c += 1;
            match f {
                'T' | 't' | 'n' | 'q' | 'v' | '4' | '6' | 'C' | 'x' | 's' => out.push(format!("-{f}")),
                'p' | 'l' | 'o' => {
                    let rest: String = chars[c..].iter().collect();
                    c = chars.len();
                    let value = if rest.is_empty() {
                        let v = args.get(i).cloned().ok_or_else(|| format!("ssh -{f} needs a value."))?;
                        i += 1;
                        v
                    } else {
                        rest
                    };
                    match f {
                        'p' if digits(&value) => out.extend(["-p".into(), value]),
                        'l' if plain_user(&value) => out.extend(["-l".into(), value]),
                        'o' => out.extend(["-o".into(), ssh_option(&value)?]),
                        _ => return Err(format!("ssh -{f} {value} is not allowed inside the fence.")),
                    }
                }
                other => return Err(format!("ssh -{other} is not allowed inside the fence.")),
            }
        }
    }
    let target = target.ok_or_else(|| "Name the computer to reach.".to_string())?;
    let (user, host) = match target.rsplit_once('@') {
        Some((u, h)) => (Some(u), h),
        None => (None, target.as_str()),
    };
    if user.is_some_and(|u| !plain_user(u)) || !host_ok(host) {
        return Err("That is not a computer name ssh may reach from the fence.".into());
    }
    if !hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
        return Err(format!("{host} is not on the list of computers AI programs may reach. Add it in Settings, AI programs."));
    }
    out.push("--".into());
    out.push(target);
    out.extend(args[i.min(args.len())..].iter().cloned());
    Ok(out)
}

fn ssh_command(root: &Path, argv: &[String]) -> Result<std::process::Command, String> {
    let dir = root.join("ssh");
    let key = dir.join("id_ed25519");
    if !key.exists() {
        make_key(&dir).map_err(|e| format!("The key for AI programs could not be made: {e}"))?;
    }
    let mut c = std::process::Command::new("ssh");
    c.args(["-F", "/dev/null", "-i"]).arg(&key)
        .args(["-o", "IdentitiesOnly=yes", "-o", "IdentityAgent=none", "-o", "BatchMode=yes", "-o", "StrictHostKeyChecking=accept-new"])
        .arg("-o").arg(format!("UserKnownHostsFile={}", dir.join("known_hosts").display()))
        .args(["-o", "ForwardAgent=no", "-o", "ForwardX11=no", "-o", "ClearAllForwardings=yes", "-o", "PermitLocalCommand=no",
               "-o", "ProxyCommand=none", "-o", "ProxyJump=none", "-o", "ControlMaster=no", "-o", "ControlPath=none"])
        .args(argv)
        .env_clear().env("PATH", "/usr/bin:/bin").current_dir("/");
    Ok(c)
}

pub fn make_key(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let key = dir.join("id_ed25519");
    let st = std::process::Command::new("ssh-keygen").args(["-q", "-t", "ed25519", "-N", "", "-C", "trindle-ai-programs", "-f"]).arg(&key)
        .stdin(Stdio::null()).status()?;
    if st.success() { Ok(()) } else { Err(std::io::Error::other("ssh-keygen failed")) }
}

pub fn public_key(root: &Path) -> Option<String> {
    std::fs::read_to_string(root.join("ssh/id_ed25519.pub")).ok().map(|s| s.trim().to_string())
}

pub fn parse_sudo(kind: &str, args: &[String]) -> Result<Vec<String>, String> {
    let mut i = 0;
    while let Some(a) = args.get(i) {
        match a.as_str() {
            "--" => {
                i += 1;
                break;
            }
            "-n" | "-H" | "-E" | "-S" | "--non-interactive" | "--disable-internal-agent" => i += 1,
            o if o.starts_with('-') => return Err(format!("{kind} {o} is not available inside the fence.")),
            _ => break,
        }
    }
    let cmd = &args[i.min(args.len())..];
    if cmd.is_empty() || cmd[0].is_empty() || cmd.iter().any(|w| w.contains('\0')) {
        return Err("Name a command to run.".into());
    }
    Ok(cmd.to_vec())
}

pub fn approval_text(argv: &[String]) -> String {
    let quote = |w: &String| if w.chars().all(|c| c.is_ascii_alphanumeric() || "-_./=:,+@%".contains(c)) { w.clone() } else { format!("'{}'", w.replace('\'', "'\\''")) };
    argv.iter().map(quote).collect::<Vec<_>>().join(" ")
}

pub fn stand_in(called: &str, args: &[String]) -> i32 {
    let Some(path) = std::env::var_os(super::fence::BROKER_ENV) else {
        eprintln!("{called} is not available here.");
        return 1;
    };
    ask(Path::new(&path), called, args)
}

pub fn ask(path: &Path, called: &str, args: &[String]) -> i32 {
    let kind = match called {
        "ssh" => "ssh",
        "sudo" | "pkexec" => called,
        super::fence::SHARE_ASK => "share",
        other => {
            eprintln!("{other} is not available inside the fence. Ask for it to be run outside.");
            return 1;
        }
    };
    let Ok(conn) = UnixStream::connect(path) else {
        eprintln!("{called}: the fence's request line is not there.");
        return 1;
    };
    let mut body: Vec<u8> = kind.as_bytes().to_vec();
    for a in args {
        body.push(0);
        body.extend_from_slice(a.as_bytes());
    }
    if body.len() > BODY_MAX || send_message(conn.as_raw_fd(), &body, &[0, 1, 2]).is_err() {
        eprintln!("{called}: the request was not sent.");
        return 1;
    }
    match recv_message(conn.as_raw_fd(), 32) {
        Ok(Some((code, _))) => String::from_utf8_lossy(&code).trim().parse().unwrap_or(1),
        _ => 1,
    }
}

#[cfg(test)]
mod tests;
