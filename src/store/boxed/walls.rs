//! The walls of a box: bubblewrap's arguments for what it may see and reach.

use std::path::{Path, PathBuf};
use crate::common::files::home;
use super::x11::{x11_socket};

pub const OWN_FOLDERS: &[&str] = &["Documents", "Pictures", "Music", "Videos", "Downloads", "Desktop"];

const ALSA_CONF: &str = "trindle-asound.conf";

const PRELOAD: &str = "etc/ld.so.preload";

pub(super) const ALSA_PULSE: &str = "</usr/share/alsa/alsa.conf>\npcm.!default { type pulse }\nctl.!default { type pulse }\n";

#[derive(Default)]
pub struct Gates {

    pub wayland: Option<String>,

    pub pulse: Option<String>,

    pub alsa: Option<String>,

    pub seccomp: Option<i32>,

    pub sync: Option<i32>,

    pub sys: Vec<String>,

    pub copied: Option<String>,

    pub x11: Option<u32>,

    pub extra: Vec<String>,

    pub usb: Vec<String>,

    pub a11y: Option<(PathBuf, PathBuf, PathBuf)>,

    pub home_room: Option<(u32, u32)>,

    pub bus: Option<String>,

    pub starter: Option<(super::start::Host, u32)>,
}

pub const QT_PLATFORM: &str = "wayland;xcb";

pub const COPIED_AT: &str = "trindle-copied";

pub const COPIED_ENV: &str = "TRINDLE_COPIED_SOCKET";

pub const BUS_AT: &str = "bus";

fn runtime() -> String {
    crate::store::pulse_guard::runtime_dir().to_string_lossy().into_owned()
}

pub fn walls(at: &Path, allowed: &[String], gates: &Gates, command: &[String]) -> Vec<String> {
    let mut walls = Walls { at, allowed, gates, files: at.join("files"), home: home(), runtime: runtime(),
                            args: ["--unshare-all", "--new-session", "--die-with-parent"].map(String::from).into() };
    walls.args.extend(clean_env_args(&std::env::vars().collect::<Vec<_>>()));
    walls.seccomp();
    walls.network();
    walls.system();
    walls.home_and_lent();
    walls.display();
    walls.sound();
    walls.starter();
    walls.environment();
    match &gates.starter {
        Some((host, first)) => walls.args.extend(super::start::command(*first, host, command)),
        None => walls.args.extend(command.iter().cloned()),
    }
    walls.args
}

pub fn clean_env_args(host: &[(String, String)]) -> Vec<String> {
    let mut a = vec!["--clearenv".to_string()];
    for (k, v) in crate::store::fence::clean_env(host) {
        a.extend(["--setenv".to_string(), k, v]);
    }
    a
}

struct Walls<'a> {
    at: &'a Path,
    allowed: &'a [String],
    gates: &'a Gates,
    files: PathBuf,
    home: PathBuf,
    runtime: String,
    args: Vec<String>,
}

pub(super) fn real_dir(p: &std::path::Path) -> bool {
    std::fs::symlink_metadata(p).is_ok_and(|m| m.is_dir())
}

impl Walls<'_> {
    fn has(&self, k: &str) -> bool {
        self.allowed.iter().any(|w| w == k)
    }

    fn add(&mut self, args: &[&str]) {
        self.args.extend(args.iter().map(|s| s.to_string()));
    }

    fn file(&self, sub: &str) -> String {
        self.files.join(sub).to_string_lossy().into_owned()
    }

    fn seccomp(&mut self) {
        if let Some(fd) = self.gates.seccomp {
            self.add(&["--unshare-user", "--disable-userns", "--seccomp", &fd.to_string()]);
        }
    }

    fn network(&mut self) {
        if self.has("network") {
            self.add(&["--share-net"]);
        } else if let Some((uid, gid)) = self.gates.home_room.filter(|_| self.has(crate::plugins::HOME_NETWORK)) {

            self.add(&["--share-net", "--unshare-user", "--disable-userns", "--uid", &uid.to_string(), "--gid", &gid.to_string(), "--cap-drop", "ALL"]);
        }
    }

    fn system(&mut self) {
        let preload = std::fs::symlink_metadata(self.files.join(PRELOAD)).ok().map(|m| m.file_type());
        for top in ["usr", "etc"] {

            let linked = top == "etc" && preload.is_some_and(|t| t.is_symlink());
            if linked {
                eprintln!("trindle-boxed: its {PRELOAD} is a link, so its box has the computer's /etc");
            }
            let own = real_dir(&self.files.join(top)) && !linked;
            let system = format!("/{top}");
            if own {

                let own = self.file(top);
                self.add(&["--overlay-src", &system, "--overlay-src", &own, "--ro-overlay", &system]);
            } else {
                self.add(&["--ro-bind", &system, &system]);
            }
        }
        if real_dir(&self.files.join("etc")) && preload.is_some_and(|t| !t.is_dir() && !t.is_symlink()) {
            self.add(&["--ro-bind", "/dev/null", &format!("/{PRELOAD}")]);
        }
        if real_dir(&self.files.join("opt")) {
            let opt = self.file("opt");
            self.add(&["--ro-bind", &opt, "/opt"]);
        }
        let app = self.at.join("app");
        if real_dir(&app) {
            self.add(&["--ro-bind", &app.to_string_lossy(), "/app"]);
        }
        for (link, to) in [("/bin", "usr/bin"), ("/sbin", "usr/sbin"), ("/lib", "usr/lib"), ("/lib64", "usr/lib64")] {
            self.add(&["--symlink", to, link]);
        }
        self.add(&["--proc", "/proc", "--dev", "/dev", "--dev-bind-try", "/dev/dri", "/dev/dri"]);
        let sys: Vec<&str> = self.gates.sys.iter().map(String::as_str).collect();
        self.add(&sys);
        let runtime = self.runtime.clone();
        self.add(&["--tmpfs", "/tmp", "--tmpfs", "/run", "--dir", &runtime]);
    }

    fn home_and_lent(&mut self) {
        let (at, home) = (self.at, self.home.clone());
        let h = home.to_string_lossy();
        self.add(&["--bind", &at.join("home").to_string_lossy(), &h]);
        let fonts = home.join(".local/share/fonts");
        if fonts.is_dir() {
            self.add(&["--ro-bind", &fonts.to_string_lossy(), &fonts.to_string_lossy()]);
        }
        let gates = self.gates;
        let (extra, _) = split_extra(&gates.extra);
        self.add(&extra);

        if self.has(crate::plugins::USB_DEVICES) {
            let usb: Vec<&str> = self.gates.usb.iter().map(String::as_str).collect();
            self.add(&usb);
        }
        if self.has("files") {

            for f in OWN_FOLDERS.iter().map(|f| home.join(f)) {
                self.add(&["--bind-try", &f.to_string_lossy(), &f.to_string_lossy()]);
            }
        }
    }

    fn display(&mut self) {
        let runtime = self.runtime.clone();
        match &self.gates.wayland {
            Some(socket) => self.add(&["--ro-bind", socket, &format!("{runtime}/wayland-0"), "--setenv", "WAYLAND_DISPLAY", "wayland-0",
                                      "--setenv", "QT_QPA_PLATFORM", QT_PLATFORM]),
            None => self.add(&["--unsetenv", "WAYLAND_DISPLAY"]),
        }

        match &self.gates.bus {
            Some(socket) => {
                let inside = format!("{runtime}/{BUS_AT}");
                self.add(&["--ro-bind", socket, &inside, "--setenv", "DBUS_SESSION_BUS_ADDRESS", &format!("unix:path={inside}")]);
            }
            None => self.add(&["--unsetenv", "DBUS_SESSION_BUS_ADDRESS"]),
        }

        match self.gates.a11y.as_ref().filter(|_| self.has(crate::plugins::ACCESSIBILITY)) {
            Some((bus, keys, voice)) => {
                let (bus, keys, voice) = (bus.to_string_lossy(), keys.to_string_lossy(), voice.to_string_lossy());
                self.add(&["--ro-bind", &bus, &bus, "--setenv", crate::store::accessibility::BUS_ENV, &format!("unix:path={bus}"), "--bind", &keys, &keys,
                           "--ro-bind", &voice, &voice]);
            }
            None => self.add(&["--unsetenv", crate::store::accessibility::BUS_ENV]),
        }

        match self.gates.copied.as_ref().filter(|_| self.has("copied")) {
            Some(socket) => {
                let inside = format!("{runtime}/{COPIED_AT}");
                self.add(&["--ro-bind", socket, &inside, "--setenv", COPIED_ENV, &inside]);
            }
            None => self.add(&["--unsetenv", COPIED_ENV]),
        }

        match self.gates.x11.filter(|_| self.has("x11")) {
            Some(n) => {
                let socket = x11_socket(n);
                self.add(&["--ro-bind", &socket, &socket, "--setenv", "DISPLAY", &format!(":{n}"), "--unsetenv", "XAUTHORITY"]);
            }
            None => self.add(&["--unsetenv", "DISPLAY", "--unsetenv", "XAUTHORITY"]),
        }
    }

    fn sound(&mut self) {
        let runtime = self.runtime.clone();
        if self.has("microphone") {
            for s in ["pipewire-0", "pulse/native"] {
                let p = format!("{runtime}/{s}");
                self.add(&["--ro-bind-try", &p, &p]);
            }
        } else if self.has("sound") {
            if let Some(guard) = &self.gates.pulse {
                self.add(&["--ro-bind", guard, &format!("{runtime}/pulse/native")]);
            }
            if let Some(conf) = &self.gates.alsa {
                let inside = format!("{runtime}/{ALSA_CONF}");
                self.add(&["--ro-bind", conf, &inside, "--setenv", "ALSA_CONFIG_PATH", &inside]);
            }
        }
    }

    fn starter(&mut self) {
        if let Some((host, _)) = &self.gates.starter {
            self.args.extend(host.binds());
        }
    }

    fn environment(&mut self) {
        if let Some(fd) = self.gates.sync {
            self.add(&["--sync-fd", &fd.to_string()]);
        }
        for v in ["SSH_AUTH_SOCK", "GNOME_KEYRING_CONTROL", "GPG_AGENT_INFO", "PULSE_SERVER"] {
            self.add(&["--unsetenv", v]);
        }
        let (runtime, h) = (self.runtime.clone(), self.home.to_string_lossy().into_owned());
        let start = split_extra(&self.gates.extra).1.unwrap_or(h);
        self.add(&["--setenv", "XDG_RUNTIME_DIR", &runtime, "--chdir", &start, "--"]);
    }
}

fn values_of(option: &str) -> usize {
    match option {
        "--overlay" => 3,
        "--bind" | "--bind-try" | "--ro-bind" | "--ro-bind-try" | "--dev-bind" | "--dev-bind-try" | "--setenv"
        | "--symlink" | "--bind-data" | "--ro-bind-data" | "--chmod" | "--file" | "--bind-fd" | "--ro-bind-fd" => 2,
        "--chdir" | "--tmpfs" | "--dir" | "--proc" | "--dev" | "--mqueue" | "--unsetenv" | "--uid" | "--gid"
        | "--hostname" | "--sync-fd" | "--seccomp" | "--cap-add" | "--cap-drop" | "--perms" | "--size" | "--args" | "--argv0"
        | "--remount-ro" | "--overlay-src" | "--ro-overlay" | "--tmp-overlay" | "--lock-file" | "--block-fd"
        | "--userns-block-fd" | "--info-fd" | "--json-status-fd" | "--exec-label" | "--file-label"
        | "--userns" | "--userns2" | "--pidns" | "--add-seccomp-fd" => 1,
        _ => 0,
    }
}

fn split_extra(extra: &[String]) -> (Vec<&str>, Option<String>) {
    let (mut rest, mut start, mut i) = (Vec::new(), None, 0);
    while i < extra.len() {
        let n = values_of(&extra[i]);
        let end = (i + 1 + n).min(extra.len());
        if extra[i] == "--chdir" {
            start = extra.get(i + 1).cloned();
        } else {
            rest.extend(extra[i..end].iter().map(String::as_str));
        }
        i = end;
    }
    (rest, start)
}

pub fn sys_args_for(devices: &[(String, String, PathBuf)]) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();
    let mut cards: Vec<PathBuf> = Vec::new();
    for (name, number, real) in devices {

        let card = real.parent().and_then(Path::parent).map(Path::to_path_buf);
        let Some(card) = card.filter(|c| c.starts_with("/sys/devices")) else { continue };
        if !cards.contains(&card) {
            cards.push(card.clone());
            let c = card.to_string_lossy().into_owned();
            a.extend(["--ro-bind".into(), c.clone(), c.clone()]);
            if let Some(id) = card.file_name() {
                a.extend(["--symlink".into(), c, format!("/sys/bus/pci/devices/{}", id.to_string_lossy())]);
            }
        }
        let r = real.to_string_lossy().into_owned();
        a.extend(["--symlink".into(), r.clone(), format!("/sys/dev/char/{number}")]);
        a.extend(["--symlink".into(), r, format!("/sys/class/drm/{name}")]);
    }
    a.extend(["--ro-bind-try", "/sys/devices/system/cpu", "/sys/devices/system/cpu"].map(String::from));
    a
}

pub fn sys_args() -> Vec<String> {
    use std::os::unix::fs::{FileTypeExt, MetadataExt};
    let mut devices = Vec::new();
    for e in std::fs::read_dir("/dev/dri").into_iter().flatten().flatten() {
        let Ok(m) = e.metadata() else { continue };
        if !m.file_type().is_char_device() {
            continue;
        }
        let number = format!("{}:{}", libc::major(m.rdev()), libc::minor(m.rdev()));
        let Ok(real) = std::fs::canonicalize(format!("/sys/dev/char/{number}")) else { continue };
        devices.push((e.file_name().to_string_lossy().into_owned(), number, real));
    }
    devices.sort();
    sys_args_for(&devices)
}

pub fn engine_for(allowed: &[String]) -> &'static str {
    let has = |k: &str| allowed.iter().any(|a| a == k);
    crate::common::sandbox::engine(has("control"), has("capture"))
}
