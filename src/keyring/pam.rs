//! The keyring opens with the login, through PAM.

use std::ffi::{CStr, CString};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::store::{self, Key};

pub const KEYS: &str = "/run/trindle-keyring";

const SERVICES: &[&str] = &["greetd", "login"];

const MOST: u64 = 4096;

const MOVE_WAIT: Duration = Duration::from_secs(30);

fn uid() -> u32 {
    // SAFETY: getuid cannot fail and touches no memory of ours.
    unsafe { libc::getuid() }
}

pub fn take_key() -> Option<Key> {
    let path = Path::new(KEYS).join(uid().to_string());
    let mut file = OpenOptions::new().read(true).write(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    let meta = file.metadata().ok()?;
    if !meta.is_file() || meta.uid() != uid() {
        return None;
    }
    let mut key = Key::default();
    let got = file.read_exact(key.as_mut());
    let _ = file.set_len(0);
    got.ok().map(|()| key)
}

pub fn put_back(key: &Key) {
    let path = Path::new(KEYS).join(uid().to_string());
    let Ok(mut file) = OpenOptions::new().write(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path) else { return };
    if file.metadata().is_ok_and(|m| m.is_file() && m.uid() == uid()) {
        let _ = file.write_all(key.as_ref());
    }
}

pub fn clear_key() {
    let path = Path::new(KEYS).join(uid().to_string());
    let Ok(file) = OpenOptions::new().write(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path) else { return };
    if file.metadata().is_ok_and(|m| m.is_file() && m.uid() == uid()) {
        let _ = file.set_len(0);
    }
}

pub fn helper() -> i32 {
    if let Err(e) = leave_key() {
        eprintln!("trindle-keyring: {e}");
    }
    0
}

fn leave_key() -> Result<(), String> {
    let service = std::env::var("PAM_SERVICE").unwrap_or_default();
    let kind = std::env::var("PAM_TYPE").unwrap_or_default();
    if !SERVICES.contains(&service.as_str()) || kind != "auth" {
        return Ok(());
    }
    let user = std::env::var("PAM_USER").map_err(|_| "no PAM_USER")?;
    let password = read_password()?;
    let (uid, gid, home) = account(&user).ok_or("no such user")?;

    if password.is_empty() {
        let _ = fs::remove_file(Path::new(KEYS).join(uid.to_string()));
        return Ok(());
    }
    let moved = home.join(".local/share").join(super::migrate::DBUS_FILE);
    let path = home.join(".local/share/trindle/keyring");
    let made = salt_of(&path, uid).is_some() || salt_of(&store::backup(&path), uid).is_some();

    if !owned_by(&moved, uid) || !made {
        if let Err(e) = move_over(&user, uid, gid, &home, &password) {
            eprintln!("trindle-keyring: moving {user} over: {e}");
        }
    }

    if !owned_by(&moved, uid) {
        return Ok(());
    }

    let Some(salt) = salt_of(&path, uid).or_else(|| salt_of(&store::backup(&path), uid)) else { return Ok(()) };
    let key = store::derive(&password, &salt);
    write_key(uid, gid, &key)
}

fn move_over(user: &str, uid: u32, gid: u32, home: &Path, password: &[u8]) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut child = Command::new(exe).arg0("trindle-keyring").arg("--pam-move")
        .env_clear().env("HOME", home).env("USER", user).env("LOGNAME", user).env("PATH", "/usr/bin:/bin")
        .current_dir("/").uid(uid).gid(gid)
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::inherit())
        .spawn().map_err(|e| e.to_string())?;

    let sent = child.stdin.take().ok_or("no pipe")?.write_all(password);
    let until = Instant::now() + MOVE_WAIT;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if Instant::now() < until => std::thread::sleep(Duration::from_millis(50)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("took too long".into());
            }
        }
    }
    sent.map_err(|e| e.to_string())
}

fn read_password() -> Result<Zeroizing<Vec<u8>>, String> {
    let mut password = Zeroizing::new(Vec::with_capacity(MOST as usize));
    std::io::stdin().take(MOST).read_to_end(&mut password).map_err(|e| e.to_string())?;
    if let Some(end) = password.iter().position(|b| *b == 0) {
        password.truncate(end);
    }
    Ok(password)
}

fn owned_by(path: &Path, uid: u32) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.is_file() && m.uid() == uid)
}

fn account(user: &str) -> Option<(u32, u32, PathBuf)> {
    let name = CString::new(user).ok()?;
    // SAFETY: name is a valid C string for the call. getpwnam returns null

    unsafe {
        let pw = libc::getpwnam(name.as_ptr());
        if pw.is_null() || (*pw).pw_dir.is_null() {
            return None;
        }
        let home = CStr::from_ptr((*pw).pw_dir).to_string_lossy().into_owned();
        Some(((*pw).pw_uid, (*pw).pw_gid, PathBuf::from(home)))
    }
}

fn salt_of(path: &Path, owner: u32) -> Option<[u8; store::SALT_LEN]> {
    let file = OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    let meta = file.metadata().ok()?;
    if !meta.is_file() || meta.uid() != owner {
        return None;
    }
    let mut head = Vec::new();
    file.take(256).read_to_end(&mut head).ok()?;
    store::salt_in(&head)
}

fn write_key(uid: u32, gid: u32, key: &Key) -> Result<(), String> {
    let dir = Path::new(KEYS);
    match fs::DirBuilder::new().mode(0o755).create(dir) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(format!("{KEYS}: {e}")),
    }
    let meta = fs::symlink_metadata(dir).map_err(|e| e.to_string())?;
    if !meta.is_dir() || meta.uid() != 0 || meta.mode() & 0o022 != 0 {
        return Err(format!("{KEYS} is not root's own folder"));
    }
    let path = dir.join(uid.to_string());
    let mut file: File = OpenOptions::new().write(true).create(true).truncate(true).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW).open(&path).map_err(|e| e.to_string())?;
    file.set_permissions(fs::Permissions::from_mode(0o600)).map_err(|e| e.to_string())?;
    std::os::unix::fs::fchown(&file, Some(uid), Some(gid)).map_err(|e| e.to_string())?;
    file.write_all(key.as_ref()).map_err(|e| e.to_string())
}
