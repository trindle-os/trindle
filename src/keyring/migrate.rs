//! Moving a person from GNOME's keyring to Trindle's. Trindle keeps saved
//! logins in its own keyring, and everyone moves across by themselves the
//! first time they log in: the login hands over the password, which opens
//! GNOME's keyring files straight off the disk. Should the password not open
//! one of them, that person stays on GNOME's until they move from Settings,
//! Users, which asks for that keyring's own password.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Serialize;
use zbus::blocking::Connection;
use zbus::zvariant::{DynamicType, OwnedObjectPath, OwnedValue, Value};
use zbus::Message;
use zeroize::Zeroizing;

use super::gnome::{self, Keyring};
use super::service::{op, Secret, GNOME, NAME, ROOT, SERVICE};
use super::store::{self, Store, LOGIN};
use crate::common::files::{config_home, data_home};
use crate::toolkit::t;

pub const DBUS_FILE: &str = "dbus-1/services/org.freedesktop.secrets.service";
const DBUS_TEXT: &str = "[D-BUS Service]\nName=org.freedesktop.secrets\nExec=/usr/bin/trindle-keyring\n";

const PORTALS: &str = "xdg-desktop-portal/trindle-portals.conf";
const SYSTEM_PORTALS: &str = "/usr/share/xdg-desktop-portal/trindle-portals.conf";
const SECRET_PORTAL: &str = "org.freedesktop.impl.portal.Secret=trindle-keyring";
const GNOME_UNITS: &[&str] = &["gnome-keyring-daemon.service", "gnome-keyring-daemon.socket"];

const WAIT: Duration = Duration::from_secs(8);
const CALL: Duration = Duration::from_secs(10);

const MOST: u64 = 4096;

fn dbus_file() -> PathBuf {
    data_home().join(DBUS_FILE)
}

fn portals_file() -> PathBuf {
    config_home().join(PORTALS)
}

fn gnome_files() -> PathBuf {
    data_home().join("keyrings")
}

pub fn switched() -> bool {
    dbus_file().exists()
}

pub enum Stopped {

    Shut(Vec<String>),
    Failed(String),
}

impl From<String> for Stopped {
    fn from(e: String) -> Self {
        Stopped::Failed(e)
    }
}

pub fn switch(user: &str, password: &str, others: &[String]) -> Result<usize, Stopped> {
    if !crate::session::lock::password_ok(user, password) {
        return Err(Stopped::Failed(t!("keyring-not-your-password").into()));
    }
    let tries: Vec<&[u8]> = std::iter::once(password).chain(others.iter().map(String::as_str))
        .map(str::as_bytes).collect();
    let found = gnome::read_all(&gnome_files(), &tries);
    if !found.left.is_empty() {
        return Err(Stopped::Shut(found.left));
    }
    let copied = fill_store(password.as_bytes(), &found.opened, found.default.as_deref())?;
    point_at_trindle()?;

    let bus = bus()?;
    reload(&bus);
    // systemd reads the masks, and GNOME's keyring stops if it is running.
    let _ = std::process::Command::new("systemctl").args(["--user", "daemon-reload"]).status();
    let _ = systemctl(&["stop"]);
    let _ = stop_owner(&bus, &["gnome-keyring-d"]);
    wait_free(&bus)?;
    start(&bus)?;

    let s = open_session(&bus)?;
    let _ = call(&bus, ROOT, GNOME, "UnlockWithMasterPassword",
        &(op(&format!("{}{LOGIN}", collections())), secret(&s, password)));
    restart_portal();
    Ok(copied)
}

pub fn at_login() -> i32 {
    // SAFETY: umask only sets this process's file mode mask.
    unsafe { libc::umask(0o077) };

    let there = store::default_path();
    if switched() && (there.exists() || store::backup(&there).exists()) {
        return 0;
    }
    let mut password = Zeroizing::new(Vec::with_capacity(MOST as usize));
    if std::io::stdin().take(MOST).read_to_end(&mut password).is_err() {
        return 1;
    }
    if let Some(end) = password.iter().position(|b| *b == 0) {
        password.truncate(end);
    }

    if password.is_empty() {
        return 0;
    }
    let found = gnome::read_all(&gnome_files(), &[&password]);
    if !found.left.is_empty() {
        eprintln!("trindle-keyring: not moving; the password does not open {}", found.left.join(", "));
        return 0;
    }
    match fill_store(&password, &found.opened, found.default.as_deref()).and_then(|_| point_at_trindle()) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-keyring: {e}");
            1
        }
    }
}

pub fn refresh_portals() {
    if !switched() {
        return;
    }
    let (file, now) = (portals_file(), portals_with_secret());
    if fs::read_to_string(&file).is_ok_and(|had| had != now) {
        let _ = write(&file, &now);
    }
}

fn point_at_trindle() -> Result<(), String> {
    write(&portals_file(), &portals_with_secret())?;
    mask_gnome(&config_home().join("systemd/user"))?;
    write(&dbus_file(), DBUS_TEXT)
}

fn mask_gnome(dir: &Path) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for unit in GNOME_UNITS {
        let path = dir.join(unit);
        if fs::symlink_metadata(&path).is_ok() {
            continue;
        }
        std::os::unix::fs::symlink("/dev/null", &path).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(())
}

fn bus() -> Result<Connection, String> {
    zbus::blocking::connection::Builder::session().and_then(|b| b.method_timeout(CALL).build())
        .map_err(|e| e.to_string())
}

fn said(e: zbus::Error) -> String {
    match e {
        zbus::Error::MethodError(_, Some(text), _) => text,
        e => e.to_string(),
    }
}

fn call<B: Serialize + DynamicType>(bus: &Connection, path: &str, iface: &str, method: &str, args: &B) -> Result<Message, String> {
    bus.call_method(Some(NAME), path, Some(iface), method, args).map_err(said)
}

fn bus_call<B: Serialize + DynamicType>(bus: &Connection, method: &str, args: &B) -> Result<Message, String> {
    bus.call_method(Some("org.freedesktop.DBus"), "/org/freedesktop/DBus", Some("org.freedesktop.DBus"), method, args)
        .map_err(said)
}

fn owner(bus: &Connection) -> Option<String> {
    bus_call(bus, "GetNameOwner", &(NAME,)).ok()?.body().deserialize().ok()
}

fn reload(bus: &Connection) {
    let _ = bus_call(bus, "ReloadConfig", &());
}

fn start(bus: &Connection) -> Result<(), String> {
    bus_call(bus, "StartServiceByName", &(NAME, 0u32)).map(|_| ())
}

fn stop_owner(bus: &Connection, expected: &[&str]) -> Result<(), String> {
    let Some(who) = owner(bus) else { return Ok(()) };
    let pid: u32 = bus_call(bus, "GetConnectionUnixProcessID", &(who,))?
        .body().deserialize().map_err(|_| "no process")?;
    let comm = fs::read_to_string(format!("/proc/{pid}/comm")).unwrap_or_default();
    if !expected.contains(&comm.trim()) {
        return Err(format!("{} holds the keyring's name", comm.trim()));
    }
    // SAFETY: kill only sends a signal; pid came from the bus a moment ago.
    unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    Ok(())
}

pub fn clear_gnome() {
    if !switched() {
        return;
    }
    let Ok(bus) = bus() else { return };
    if owner(&bus).is_none() {
        return;
    }
    let _ = std::process::Command::new("systemctl").args(["--user", "daemon-reload"]).status();
    let _ = systemctl(&["stop"]);
    if stop_owner(&bus, &["gnome-keyring-d"]).is_ok() {
        let _ = wait_free(&bus);
    }
}

pub fn end_spare_gnome() {
    use std::os::unix::fs::MetadataExt;
    // SAFETY: getuid cannot fail.
    let me = unsafe { libc::getuid() };
    let Ok(procs) = fs::read_dir("/proc") else { return };
    for p in procs.flatten() {
        let Ok(pid) = p.file_name().to_string_lossy().parse::<libc::pid_t>() else { continue };
        if p.metadata().is_ok_and(|m| m.uid() == me)
            && fs::read_to_string(p.path().join("comm")).is_ok_and(|c| c.trim() == "gnome-keyring-d") {
            // SAFETY: kill only sends a signal, to this user's own process.
            unsafe { libc::kill(pid, libc::SIGTERM) };
        }
    }
}

fn wait_free(bus: &Connection) -> Result<(), String> {
    let until = Instant::now() + WAIT;
    while owner(bus).is_some() {
        if Instant::now() > until {
            return Err(t!("keyring-would-not-stop").into());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn open_session(bus: &Connection) -> Result<OwnedObjectPath, String> {
    let reply = call(bus, ROOT, SERVICE, "OpenSession", &("plain", Value::from("")))?;
    reply.body().deserialize::<(OwnedValue, OwnedObjectPath)>().map(|(_, path)| path)
        .map_err(|_| "no session".into())
}

fn secret(session: &OwnedObjectPath, password: &str) -> Secret {
    (session.clone(), Vec::new(), password.as_bytes().to_vec(), "text/plain".into())
}

fn collections() -> String {
    format!("{ROOT}/collection/")
}

fn fill_store(password: &[u8], keyrings: &[Keyring], default: Option<&str>) -> Result<usize, String> {
    let path = store::default_path();
    let mut st = match Store::load(&path) {
        Ok(mut st) => {
            if st.unlock(password).is_err() {
                set_aside(&path)?;
                st = Store::create(&path, password).map_err(|e| e.to_string())?;
            }
            st
        }
        Err(_) => {

            set_aside(&path)?;
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            }
            Store::create(&path, password).map_err(|e| e.to_string())?
        }
    };
    let mut copied = 0;
    for k in keyrings {
        if !st.index.collections.contains_key(&k.name) {
            st.add_collection(&k.name, &k.label).map_err(|e| e.to_string())?;
        }
        for item in &k.items {
            st.put(&k.name, item.clone(), true).map_err(|e| e.to_string())?;
            copied += 1;
        }
    }
    if let Some(d) = default.filter(|d| st.index.collections.contains_key(*d)) {
        st.index.aliases.insert(store::DEFAULT_ALIAS.into(), d.into());
    }
    st.save().map_err(|e| e.to_string())?;
    Ok(copied)
}

fn set_aside(path: &Path) -> Result<(), String> {
    let stamp = store::now();
    for p in [path.to_path_buf(), store::backup(path)] {
        if p.exists() {
            let to = PathBuf::from(format!("{}.before-{stamp}", p.display()));
            fs::rename(&p, to).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    crate::common::files::write_text(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn portals_with_secret() -> String {
    let base = fs::read_to_string(SYSTEM_PORTALS)
        .unwrap_or_else(|_| "[preferred]\ndefault=trindle\norg.freedesktop.impl.portal.Screenshot=trindle\n".into());
    with_secret(&base)
}

fn with_secret(base: &str) -> String {
    let mut text: String = base.lines().filter(|l| !l.starts_with("org.freedesktop.impl.portal.Secret="))
        .flat_map(|l| [l, "\n"]).collect();
    text.push_str(SECRET_PORTAL);
    text.push('\n');
    text
}

fn systemctl(verb: &[&str]) -> Result<(), String> {
    let out = std::process::Command::new("systemctl").arg("--user").args(verb).args(GNOME_UNITS)
        .output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn restart_portal() {
    let _ = std::process::Command::new("systemctl")
        .args(["--user", "try-restart", "xdg-desktop-portal.service"]).status();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::service::{self, COLLECTION};
    use std::collections::HashMap;

    #[test]
    fn the_portal_settings_gain_one_secret_line() {
        let base = "[preferred]\ndefault=gtk\norg.freedesktop.impl.portal.Secret=gnome-keyring\n";
        let text = with_secret(base);
        assert_eq!(text, "[preferred]\ndefault=gtk\norg.freedesktop.impl.portal.Secret=trindle-keyring\n");
        assert_eq!(with_secret(&text), text);
    }

    #[test]
    fn masking_leaves_a_real_unit_file_alone() {
        let dir = std::env::temp_dir().join(format!("trindle-mask-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(GNOME_UNITS[1]), "mine").unwrap();
        mask_gnome(&dir).unwrap();
        mask_gnome(&dir).unwrap();
        assert_eq!(fs::read_link(dir.join(GNOME_UNITS[0])).unwrap(), Path::new("/dev/null"));
        assert_eq!(fs::read_to_string(dir.join(GNOME_UNITS[1])).unwrap(), "mine");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    #[ignore = "needs a real login and its password in TRINDLE_TEST_PW: run by hand"]
    fn a_real_login_switches_over() {
        let pw = std::env::var("TRINDLE_TEST_PW").unwrap();
        let user = crate::common::files::user();
        let bus = bus().unwrap();
        let who = owner(&bus).and_then(|o| bus_call(&bus, "GetConnectionUnixProcessID", &(o,)).ok())
            .and_then(|m| m.body().deserialize::<u32>().ok())
            .map(|pid| fs::read_to_string(format!("/proc/{pid}/comm")).unwrap_or_default());
        println!("keyring now: {who:?}, switched: {}", switched());
        match std::env::var("TRINDLE_TEST_STEP").unwrap().as_str() {
            "seed" => {
                let s = open_session(&bus).unwrap();
                let login = op(&format!("{}{LOGIN}", collections()));
                let props = HashMap::from([
                    (service::LABEL_ITEM, Value::from("Mail")),
                    (service::ATTRIBUTES, Value::from(HashMap::from([
                        ("app".to_string(), "mail".to_string())]))),
                ]);
                let v: Secret = (s, Vec::new(), b"one".to_vec(), "text/plain".into());
                call(&bus, login.as_str(), COLLECTION, "CreateItem", &(props, v, true)).unwrap();
            }
            "switch" => match switch(&user, &pw, &[]) {
                Ok(n) => println!("moved {n}"),
                Err(Stopped::Shut(left)) => println!("shut: {left:?}"),
                Err(Stopped::Failed(e)) => println!("failed: {e}"),
            },
            "look" => {
                let s = open_session(&bus).unwrap();
                let attrs = HashMap::from([("app".to_string(), "mail".to_string())]);
                let found = call(&bus, ROOT, SERVICE, "SearchItems", &(attrs,)).unwrap();
                let (open, shut): (Vec<OwnedObjectPath>, Vec<OwnedObjectPath>) = found.body().deserialize().unwrap();
                println!("open {}, locked {}", open.len(), shut.len());
                assert!(shut.is_empty() && open.len() == 1);
                let got = call(&bus, ROOT, SERVICE, "GetSecrets", &(open, s)).unwrap();
                let got: HashMap<OwnedObjectPath, Secret> = got.body().deserialize().unwrap();
                assert_eq!(got.values().next().unwrap().2, b"one");
                println!("secret read back");
            }
            other => panic!("no step {other}"),
        }
    }
}
