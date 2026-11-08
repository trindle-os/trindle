//! What is using the microphone, the camera or the screen right now, for
//! the dots in the middle of the top edge and the small icons in the open windows
//! list. The compositor works it out: the microphone and camera from
//! PipeWire's streams and from which programs hold a camera device open,
//! the screen from its own screen copying. It writes what it found to one
//! small file the ring reads, so both say the same thing.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::common::kit::{self, Rgb};
use trindle_toolkit::t;

pub const HOME_NETWORK_ICON: &str = "network-workgroup-symbolic";
pub const USB_ICON: &str = "scanner-symbolic";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Microphone,
    Camera,
    Screen,
    Input,

    Files,

    HomeNetwork,

    Usb,
}

impl Kind {

    pub const ALL: [Kind; 7] = [Kind::Microphone, Kind::Camera, Kind::Screen, Kind::Input, Kind::Files, Kind::HomeNetwork, Kind::Usb];

    pub fn word(self) -> &'static str {
        match self {
            Kind::Microphone => "microphone",
            Kind::Camera => "camera",
            Kind::Screen => "screen",
            Kind::Input => "input",
            Kind::Files => "files",
            Kind::HomeNetwork => "home-network",
            Kind::Usb => "usb",
        }
    }

    pub fn from_word(w: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.word() == w)
    }

    pub fn icon(self) -> &'static str {
        match self {
            Kind::Microphone => "audio-input-microphone-symbolic",
            Kind::Camera => "camera-web-symbolic",
            Kind::Screen => "video-display-symbolic",
            Kind::Input => "input-keyboard-symbolic",
            Kind::Files => "folder-symbolic",
            Kind::HomeNetwork => HOME_NETWORK_ICON,
            Kind::Usb => USB_ICON,
        }
    }

    pub fn colour(self) -> Rgb {
        match self {
            Kind::Microphone => kit::microphone(),
            Kind::Camera => kit::camera(),
            Kind::Screen => kit::screen(),
            Kind::Input => kit::input(),
            Kind::Files => kit::files(),
            Kind::HomeNetwork => kit::home_network(),
            Kind::Usb => kit::usb(),
        }
    }

    pub fn colour_name(self) -> &'static str {
        match self {
            Kind::Microphone => kit::MICROPHONE_NAME,
            Kind::Camera => kit::CAMERA_NAME,
            Kind::Screen => kit::SCREEN_NAME,
            Kind::Input => kit::INPUT_NAME,
            Kind::Files => kit::FILES_NAME,
            Kind::HomeNetwork => kit::NETWORK_NAME,
            Kind::Usb => kit::USB_NAME,
        }
    }

    pub fn colour_in(self, light: bool) -> Rgb {
        kit::named_in(self.colour_name(), light)
    }

    pub fn bit(self) -> u8 {
        match self {
            Kind::Microphone => 1,
            Kind::Camera => 2,
            Kind::Screen => 4,
            Kind::Input => 8,
            Kind::Files => 16,
            Kind::HomeNetwork => 32,
            Kind::Usb => 64,
        }
    }

    pub fn spoken(self, who: &[&str]) -> String {
        let names = who.join(", ");
        match (self, who.is_empty()) {
            (Kind::Microphone, true) => t!("common-in-use-microphone").into(),
            (Kind::Microphone, false) => t!("common-in-use-microphone-by", who = &names),
            (Kind::Camera, true) => t!("common-in-use-camera").into(),
            (Kind::Camera, false) => t!("common-in-use-camera-by", who = &names),
            (Kind::Screen, true) => t!("common-in-use-screen").into(),
            (Kind::Screen, false) => t!("common-in-use-screen-by", who = &names),
            (Kind::Input, true) => t!("common-in-use-input").into(),
            (Kind::Input, false) => t!("common-in-use-input-by", who = &names),
            (Kind::Files, true) => t!("common-in-use-files").into(),
            (Kind::Files, false) => t!("common-in-use-files-by", who = &names),
            (Kind::HomeNetwork, true) => t!("common-in-use-home-network").into(),
            (Kind::HomeNetwork, false) => t!("common-in-use-home-network-by", who = &names),
            (Kind::Usb, true) => t!("common-in-use-usb").into(),
            (Kind::Usb, false) => t!("common-in-use-usb-by", who = &names),
        }
    }
}

pub fn lines(users: &[User], microphone_off: bool) -> Vec<String> {
    let mut out = Vec::new();
    for kind in Kind::ALL {
        let mut who: Vec<&str> = Vec::new();
        for u in users.iter().filter(|u| u.kind == kind) {
            let name = if u.name.trim().is_empty() { u.app_ids.first().map_or("", String::as_str) } else { u.name.trim() };
            if !name.is_empty() && !who.contains(&name) {
                who.push(name);
            }
        }
        if !users.iter().any(|u| u.kind == kind) {
            continue;
        }
        out.push(match (kind, microphone_off, who.is_empty()) {
            (Kind::Microphone, true, true) => t!("common-in-use-microphone-off").to_string(),
            (Kind::Microphone, true, false) => t!("common-in-use-microphone-off-by", who = who.join(", ")),
            _ => kind.spoken(&who),
        });
    }
    out
}

pub fn mask(users: &[User]) -> u8 {
    users.iter().fold(0, |m, u| m | u.kind.bit())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub kind: Kind,
    pub pid: Option<u32>,
    pub name: String,

    pub app_ids: Vec<String>,
}

#[derive(Default)]
pub struct Pw {
    clients: HashMap<u64, Client>,
    nodes: HashMap<u64, Node>,
}

#[derive(Default)]
struct Client {

    sec_pid: Option<u32>,

    said_pid: Option<u32>,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub id: u64,

    pub name: String,

    pub muted: Option<bool>,
}

impl Source {

    fn key(&self) -> String {
        if self.name.is_empty() { format!("#{}", self.id) } else { self.name.clone() }
    }
}

#[derive(Default)]
struct Node {

    kind: Option<Kind>,

    camera_source: bool,

    sound_source: bool,
    muted: Option<bool>,
    node_name: String,
    running: bool,
    client: Option<u64>,
    pid: Option<u32>,
    name: String,
}

fn prop_str(props: &Value, key: &str) -> Option<String> {
    match props.get(key)? {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn prop_num(props: &Value, key: &str) -> Option<u64> {
    match props.get(key)? {
        Value::Number(n) => n.as_u64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

fn prop_true(props: &Value, key: &str) -> bool {
    match props.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => s == "true",
        _ => false,
    }
}

fn pid_of(n: Option<u64>) -> Option<u32> {
    n.filter(|&p| p > 1).and_then(|p| u32::try_from(p).ok())
}

impl Pw {

    pub fn feed(&mut self, v: &Value) {
        match v {
            Value::Array(items) => items.iter().for_each(|o| self.feed_one(o)),
            Value::Object(_) => self.feed_one(v),
            _ => {}
        }
    }

    fn feed_one(&mut self, o: &Value) {
        let Some(id) = o.get("id").and_then(Value::as_u64) else { return };
        let info = match o.get("info") {
            Some(i) if !i.is_null() => i,
            _ => {

                self.clients.remove(&id);
                self.nodes.remove(&id);
                return;
            }
        };
        let kind = o.get("type").and_then(Value::as_str).unwrap_or("");
        let props = info.get("props").unwrap_or(&Value::Null);
        if kind.ends_with(":Client") {
            let c = self.clients.entry(id).or_default();
            if !props.is_null() {
                c.sec_pid = pid_of(prop_num(props, "pipewire.sec.pid"));
                c.said_pid = pid_of(prop_num(props, "application.process.id"));
                c.name = prop_str(props, "application.name").unwrap_or_default();
            }
        } else if kind.ends_with(":Node") {
            let n = self.nodes.entry(id).or_default();
            if let Some(state) = info.get("state").and_then(Value::as_str) {
                n.running = state == "running";
            }
            let said: Vec<bool> = info.pointer("/params/Props").and_then(Value::as_array).into_iter().flatten()
                .filter_map(|p| p.get("mute").and_then(Value::as_bool))
                .collect();
            if !said.is_empty() {
                n.muted = Some(said.iter().any(|&m| m));
            }
            if !props.is_null() {
                let class = prop_str(props, "media.class").unwrap_or_default();
                let monitor = prop_true(props, "stream.monitor");
                n.kind = match class.as_str() {
                    "Stream/Input/Audio" if !monitor => Some(Kind::Microphone),
                    "Stream/Input/Video" => Some(Kind::Camera),
                    _ => None,
                };
                let api = prop_str(props, "device.api").unwrap_or_default();
                n.camera_source = class == "Video/Source" && (api == "v4l2" || api == "libcamera");
                n.sound_source = class.starts_with("Audio/Source") || class == "Audio/Duplex";
                n.node_name = prop_str(props, "node.name").unwrap_or_default();
                n.client = prop_num(props, "client.id");
                n.pid = pid_of(prop_num(props, "application.process.id"));
                n.name = prop_str(props, "application.name")
                    .or_else(|| prop_str(props, "node.name"))
                    .unwrap_or_default();
            }
        }
    }

    pub fn users(&self, middleman: impl Fn(u32) -> bool) -> Vec<User> {
        let camera_on = self.nodes.values().any(|n| n.camera_source && n.running);
        let mut out: Vec<User> = Vec::new();
        for n in self.nodes.values() {
            let Some(kind) = n.kind else { continue };
            if !n.running || (kind == Kind::Camera && !camera_on) {
                continue;
            }
            let client = n.client.and_then(|c| self.clients.get(&c));
            let pid = match client {
                Some(c) => match c.sec_pid {
                    Some(p) if !middleman(p) => Some(p),

                    _ => c.said_pid,
                },
                None => n.pid,
            };
            let name = if !n.name.is_empty() {
                n.name.clone()
            } else {
                client.map(|c| c.name.clone()).unwrap_or_default()
            };
            if !out.iter().any(|u| u.kind == kind && u.pid == pid && u.name == name) {
                out.push(User { kind, pid, name, app_ids: Vec::new() });
            }
        }
        out.sort_by(|a, b| (a.kind, a.pid, &a.name).cmp(&(b.kind, b.pid, &b.name)));
        out
    }

    pub fn sources(&self) -> Vec<Source> {
        let mut out: Vec<Source> = self.nodes.iter()
            .filter(|(_, n)| n.sound_source)
            .map(|(&id, n)| Source { id, name: n.node_name.clone(), muted: n.muted })
            .collect();
        out.sort_by_key(|s| s.id);
        out
    }
}

pub fn microphone_off_mark() -> PathBuf {
    crate::common::files::home().join(".local/state/trindle/microphone-off")
}

pub fn microphone_off_note() -> &'static str {
    t!("common-in-use-microphone-off-note")
}

pub fn microphone_off() -> bool {
    microphone_off_mark().exists()
}

pub fn set_microphone_off(off: bool) -> std::io::Result<()> {
    let mark = microphone_off_mark();
    if off {
        if let Some(dir) = mark.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&mark, "Trindle keeps every microphone muted while this file is here.\n")
    } else {
        match std::fs::remove_file(&mark) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }
}

pub fn microphone_muted_list() -> PathBuf {
    crate::common::files::home().join(".local/state/trindle/microphone-muted")
}

pub const ASK_AGAIN: Duration = Duration::from_secs(2);

#[derive(Default)]
pub struct MicSwitch {

    pub ours: HashSet<String>,

    asked: HashMap<u64, Instant>,
}

impl MicSwitch {

    pub fn plan(&mut self, off: bool, sources: &[Source], now: Instant) -> Vec<(u64, bool)> {
        self.asked.retain(|id, _| sources.iter().any(|s| s.id == *id && s.muted != Some(true)));
        let mut out = Vec::new();
        if off {
            for s in sources {
                if s.muted == Some(true) {
                    continue;
                }
                let waiting = self.asked.get(&s.id).is_some_and(|&t| now.saturating_duration_since(t) < ASK_AGAIN);
                if !waiting {
                    out.push((s.id, true));
                    self.asked.insert(s.id, now);
                }
                self.ours.insert(s.key());
            }
        } else {

            for s in sources {
                if self.ours.remove(&s.key()) && s.muted != Some(false) {
                    out.push((s.id, false));
                }
            }
            self.asked.clear();
        }
        out
    }
}

pub fn is_video_link(target: &Path) -> bool {
    target.to_str().is_some_and(|t| {
        t.strip_prefix("/dev/video").is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    })
}

const MIDDLEMEN: [&str; 4] = ["pipewire", "pipewire-pulse", "wireplumber", "pipewire-media-session"];

pub fn is_middleman(proc_root: &Path, pid: u32) -> bool {
    let Ok(exe) = std::fs::read_link(proc_root.join(pid.to_string()).join("exe")) else { return false };
    exe.starts_with("/usr/")
        && exe.file_name().and_then(|n| n.to_str()).is_some_and(|n| MIDDLEMEN.contains(&n))
}

pub fn video_holders(proc_root: &Path) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    let Ok(dir) = std::fs::read_dir(proc_root) else { return out };
    for e in dir.flatten() {
        let Some(pid) = e.file_name().to_str().and_then(|s| s.parse::<u32>().ok()) else { continue };
        let Ok(fds) = std::fs::read_dir(e.path().join("fd")) else { continue };
        let holds = fds.flatten().any(|fd| std::fs::read_link(fd.path()).is_ok_and(|t| is_video_link(&t)));
        if !holds {
            continue;
        }
        if !is_middleman(proc_root, pid) {
            let comm = std::fs::read_to_string(e.path().join("comm")).unwrap_or_default().trim().to_string();
            out.push((pid, comm));
        }
    }
    out.sort();
    out
}

pub const GAP: Duration = Duration::from_secs(2);

pub const SPAN: Duration = Duration::from_millis(500);
pub const FRAMES: u32 = 3;

struct Run {
    first: Instant,
    last: Instant,
    frames: u32,
}

#[derive(Default)]
pub struct Recorders {
    runs: HashMap<u32, Run>,
}

impl Recorders {

    pub fn note(&mut self, pid: u32, now: Instant) {
        let run = self.runs.entry(pid).or_insert(Run { first: now, last: now, frames: 0 });
        if now.saturating_duration_since(run.last) >= GAP {
            *run = Run { first: now, last: now, frames: 0 };
        }
        run.last = run.last.max(now);
        run.frames = run.frames.saturating_add(1);
    }

    pub fn recording(&mut self, now: Instant) -> Vec<u32> {
        self.runs.retain(|_, r| now.saturating_duration_since(r.last) < GAP);
        let mut out: Vec<u32> = self.runs.iter()
            .filter(|(_, r)| r.frames >= FRAMES && r.last.saturating_duration_since(r.first) >= SPAN)
            .map(|(&pid, _)| pid)
            .collect();
        out.sort_unstable();
        out
    }
}

pub const CASTS: &str = "trindle-screencast";

pub fn casts_path() -> PathBuf {
    crate::common::files::scratch().join(CASTS)
}

pub const PORTAL: &str = "trindle-portal";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cast {
    pub pid: Option<u32>,
    pub name: String,
}

pub fn encode_casts(portal: u32, casts: &[Cast]) -> String {
    let mut out = format!("{portal}\n");
    for c in casts {
        let _ = writeln!(out, "{}\t{}", c.pid.unwrap_or(0), c.name.replace(['\t', '\n'], " "));
    }
    out
}

pub fn decode_casts(text: &str) -> Option<(u32, Vec<Cast>)> {
    let mut lines = text.lines();
    let portal = lines.next()?.trim().parse().ok()?;
    let casts = lines
        .filter_map(|line| {
            let (pid, name) = line.split_once('\t')?;
            Some(Cast { pid: pid.parse::<u32>().ok().filter(|&p| p != 0), name: name.to_string() })
        })
        .collect();
    Some((portal, casts))
}

pub fn screen_users(recording: &[u32], casts: Option<(u32, Vec<Cast>)>, comm: impl Fn(u32) -> Option<String>) -> Vec<User> {
    let portal = casts.filter(|(pid, _)| comm(*pid).as_deref() == Some(PORTAL));
    let mut out: Vec<User> = Vec::new();
    if let Some((_, casts)) = &portal {
        for c in casts {
            out.push(User { kind: Kind::Screen, pid: c.pid, name: c.name.clone(), app_ids: Vec::new() });
        }
    }
    for &pid in recording {
        if portal.as_ref().is_some_and(|(p, _)| *p == pid) {
            continue;
        }
        let name = comm(pid).unwrap_or_default();
        out.push(User { kind: Kind::Screen, pid: Some(pid), name, app_ids: Vec::new() });
    }
    out
}

pub fn parent_from_stat(stat: &str) -> Option<u32> {
    let rest = &stat[stat.rfind(')')? + 1..];
    rest.split_whitespace().nth(1)?.parse().ok()
}

pub fn parent_of(pid: u32) -> Option<u32> {
    parent_from_stat(&std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?)
}

pub fn match_windows(pid: Option<u32>, name: &str, owners: &[(u32, String)], parent: impl Fn(u32) -> Option<u32>) -> Vec<String> {
    let of = |p: u32| -> Vec<String> {
        let mut ids: Vec<String> = owners.iter().filter(|(o, _)| *o == p).map(|(_, a)| a.clone()).collect();
        ids.dedup();
        ids
    };
    if let Some(p) = pid {
        let ids = of(p);
        if !ids.is_empty() {
            return ids;
        }
    }
    let key = name.trim().to_lowercase();
    if !key.is_empty() {
        let mut ids: Vec<String> = owners.iter()
            .filter(|(_, a)| {
                let a = a.to_lowercase();
                a == key || a.rsplit('.').next() == Some(key.as_str())
            })
            .map(|(_, a)| a.clone())
            .collect();
        ids.dedup();
        if !ids.is_empty() {
            return ids;
        }
    }
    let mut at = pid;
    for _ in 0..32 {
        let Some(p) = at.and_then(&parent) else { break };
        if p <= 1 {
            break;
        }
        let ids = of(p);
        if !ids.is_empty() {
            return ids;
        }
        at = Some(p);
    }
    Vec::new()
}

pub fn path() -> PathBuf {
    crate::common::files::scratch().join(FILE)
}

pub const FILE: &str = "trindle-in-use";

pub fn encode(users: &[User]) -> String {
    let clean = |s: &str| s.replace(['\t', '\n', ','], " ");
    let mut out = String::new();
    for u in users {
        let ids: Vec<String> = u.app_ids.iter().map(|a| clean(a)).collect();
        let _ = writeln!(out, "{}\t{}\t{}\t{}", u.kind.word(), u.pid.unwrap_or(0), clean(&u.name), ids.join(","));
    }
    out
}

pub fn decode(text: &str) -> Vec<User> {
    text.lines().filter_map(|line| {
        let mut parts = line.split('\t');
        let kind = Kind::from_word(parts.next()?)?;
        let pid = parts.next()?.parse::<u32>().ok().filter(|&p| p != 0);
        let name = parts.next().unwrap_or("").to_string();
        let app_ids = parts.next().unwrap_or("").split(',').filter(|a| !a.is_empty()).map(String::from).collect();
        Some(User { kind, pid, name, app_ids })
    }).collect()
}

pub fn write(at: &Path, users: &[User]) -> std::io::Result<()> {
    let tmp = at.with_extension("new");
    std::fs::write(&tmp, encode(users))?;
    std::fs::rename(&tmp, at)
}

pub fn read(at: &Path) -> Vec<User> {
    std::fs::read_to_string(at).map(|t| decode(&t)).unwrap_or_default()
}

pub const FILES_PREFIX: &str = "trindle-in-use-files-";

pub const FILES_SHOWN: Duration = Duration::from_secs(10);

pub fn files_path(pid: u32) -> PathBuf {
    crate::common::files::scratch().join(format!("{FILES_PREFIX}{pid}"))
}

pub fn encode_files(until_ms: u64, who: &str) -> String {
    format!("{until_ms}\t{}\n", who.replace(['\t', '\n', '\r'], " "))
}

pub fn decode_files(text: &str, now_ms: u64) -> Vec<User> {
    text.lines().filter_map(|line| {
        let (until, who) = line.split_once('\t')?;
        let until = until.parse::<u64>().ok().filter(|u| *u > now_ms)?;

        (until <= now_ms + FILES_SHOWN.as_millis() as u64 && !who.is_empty())
            .then(|| User { kind: Kind::Files, pid: None, name: who.to_string(), app_ids: Vec::new() })
    }).collect()
}

pub const BOX_PREFIX: &str = "trindle-in-use-box-";

const BOX_KINDS: [Kind; 2] = [Kind::HomeNetwork, Kind::Usb];

pub fn box_path(pid: u32) -> PathBuf {
    crate::common::files::scratch().join(format!("{BOX_PREFIX}{pid}"))
}

pub fn decode_box(text: &str) -> Vec<User> {
    decode(text).into_iter().filter(|u| BOX_KINDS.contains(&u.kind)).take(BOX_KINDS.len()).collect()
}

pub struct BoxRows(PathBuf);

impl BoxRows {

    pub fn start(pid: u32, rows: &[(Kind, &str, &str)]) -> Option<BoxRows> {
        let users: Vec<User> = rows.iter().filter(|(k, _, _)| BOX_KINDS.contains(k))
            .map(|(k, name, id)| User { kind: *k, pid: None, name: name.to_string(), app_ids: vec![id.to_string()] }).collect();
        if users.is_empty() {
            return None;
        }
        let at = box_path(pid);
        write(&at, &users).ok()?;
        Some(BoxRows(at))
    }
}

impl Drop for BoxRows {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

pub fn boxes_in(dir: &Path, alive: impl Fn(u32) -> bool) -> Vec<User> {
    let mut users = Vec::new();
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(pid) = name.strip_prefix(BOX_PREFIX).and_then(|p| p.parse::<u32>().ok()) else { continue };
        if !alive(pid) {
            let _ = std::fs::remove_file(e.path());
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(e.path()) {
            for u in decode_box(&text) {
                if !users.contains(&u) {
                    users.push(u);
                }
            }
        }
    }
    users
}

pub fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

pub fn read_all() -> Vec<User> {
    let mut users = read(&path());
    let now = now_ms();
    for e in std::fs::read_dir(crate::common::files::scratch()).into_iter().flatten().flatten() {
        if !e.file_name().to_string_lossy().starts_with(FILES_PREFIX) {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(e.path()) {
            for u in decode_files(&text, now) {
                if !users.contains(&u) {
                    users.push(u);
                }
            }
        }
    }
    for u in boxes_in(&crate::common::files::scratch(), |pid| Path::new(&format!("/proc/{pid}")).exists()) {
        if !users.contains(&u) {
            users.push(u);
        }
    }
    users
}

pub fn fake(spec: &str) -> Vec<User> {
    spec.split(',').filter_map(|part| {
        let (word, app) = part.trim().split_once(':').unwrap_or((part.trim(), ""));
        let kind = Kind::from_word(word)?;
        let app_ids = if app.is_empty() { Vec::new() } else { vec![app.to_string()] };
        let name = if app.is_empty() { format!("Test {}", kind.word()) } else { app.to_string() };
        Some(User { kind, pid: None, name, app_ids })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_two_kinds_in_use_share_a_colour_in_either_look() {
        let q = |c: Rgb| ((c.0 * 255.0).round() as i32, (c.1 * 255.0).round() as i32, (c.2 * 255.0).round() as i32);
        for light in [false, true] {
            for a in Kind::ALL {
                for b in Kind::ALL {
                    if a != b {
                        assert_ne!(q(a.colour_in(light)), q(b.colour_in(light)), "{a:?} and {b:?}, light {light}");
                    }
                }
            }
        }
        for k in Kind::ALL {
            assert_eq!(q(k.colour()), q(k.colour_in(false)), "{k:?}");
        }
    }

    #[test]
    fn the_portal_is_named_by_the_programs_it_shares_with() {
        let file = encode_casts(700, &[Cast { pid: Some(4242), name: "Brave".into() }, Cast { pid: None, name: "Call".into() }]);
        let casts = decode_casts(&file);
        assert_eq!(casts, Some((700, vec![Cast { pid: Some(4242), name: "Brave".into() }, Cast { pid: None, name: "Call".into() }])));
        let comm = |pid: u32| match pid {
            700 => Some(PORTAL.to_string()),
            900 => Some("wf-recorder".to_string()),
            _ => None,
        };
        let users = screen_users(&[700, 900], casts.clone(), comm);
        let named: Vec<(Option<u32>, &str)> = users.iter().map(|u| (u.pid, u.name.as_str())).collect();
        assert_eq!(named, vec![(Some(4242), "Brave"), (None, "Call"), (Some(900), "wf-recorder")]);

        let users = screen_users(&[700], casts, |pid| (pid == 700).then(|| "other".to_string()));
        assert_eq!(users.iter().map(|u| u.name.as_str()).collect::<Vec<_>>(), vec!["other"]);
        assert_eq!(decode_casts(""), None);
    }

    fn stream(json: &str) -> Pw {
        let mut pw = Pw::default();
        for v in serde_json::Deserializer::from_str(json).into_iter::<Value>() {
            pw.feed(&v.unwrap());
        }
        pw
    }

    const CLIENT: &str = r#"[{"id":18,"type":"PipeWire:Interface:Client","info":{"props":
        {"application.name":"Voice Notes","application.process.id":4242,"pipewire.sec.pid":4242}}}]"#;

    fn mic(state: &str) -> String {
        format!(r#"[{{"id":19,"type":"PipeWire:Interface:Node","info":{{"state":"{state}","props":
            {{"media.class":"Stream/Input/Audio","client.id":18,"application.name":"Voice Notes","node.name":"notes"}}}}}}]"#)
    }

    #[test]
    fn microphone_needs_a_running_stream() {
        let pw = stream(&format!("{CLIENT}\n{}", mic("suspended")));
        assert!(pw.users(|_| false).is_empty());
        let pw = stream(&format!("{CLIENT}\n{}", mic("running")));
        assert_eq!(pw.users(|_| false), vec![User { kind: Kind::Microphone, pid: Some(4242), name: "Voice Notes".into(), app_ids: vec![] }]);
    }

    #[test]
    fn microphone_goes_when_the_stream_does() {
        let pw = stream(&format!("{CLIENT}\n{}\n[{{\"id\":19,\"info\":null}}]", mic("running")));
        assert!(pw.users(|_| false).is_empty());
    }

    #[test]
    fn pulse_clients_give_the_pid_as_words_and_meters_do_not_count() {
        let json = r#"[{"id":30,"type":"PipeWire:Interface:Client","info":{"props":{"application.process.id":"777","application.name":"Voice Notes"}}},
            {"id":31,"type":"PipeWire:Interface:Node","info":{"state":"running","props":{"media.class":"Stream/Input/Audio","client.id":30}}},
            {"id":32,"type":"PipeWire:Interface:Node","info":{"state":"running","props":{"media.class":"Stream/Input/Audio","client.id":30,"stream.monitor":"true"}}}]"#;
        let users = stream(json).users(|_| false);
        assert_eq!(users.len(), 1);
        assert_eq!((users[0].pid, users[0].name.as_str()), (Some(777), "Voice Notes"));
    }

    #[test]
    fn a_video_stream_is_the_camera_only_while_a_camera_runs() {
        let reader = r#"[{"id":40,"type":"PipeWire:Interface:Node","info":{"state":"running","props":{"media.class":"Stream/Input/Video","application.name":"Meet","application.process.id":555}}}]"#;
        assert!(stream(reader).users(|_| false).is_empty());
        let cam = r#"[{"id":41,"type":"PipeWire:Interface:Node","info":{"state":"running","props":{"media.class":"Video/Source","device.api":"v4l2"}}}]"#;
        let users = stream(&format!("{reader}\n{cam}")).users(|_| false);
        assert_eq!(users.len(), 1);
        assert_eq!((users[0].kind, users[0].pid), (Kind::Camera, Some(555)));
    }

    #[test]
    fn a_program_is_known_by_the_process_pipewire_saw() {

        let liar = r#"[{"id":50,"type":"PipeWire:Interface:Client","info":{"props":{"pipewire.sec.pid":900,"application.process.id":4242,"application.name":"Voice Notes"}}},
            {"id":51,"type":"PipeWire:Interface:Node","info":{"state":"running","props":{"media.class":"Stream/Input/Audio","client.id":50,"application.process.id":4242}}}]"#;
        assert_eq!(stream(liar).users(|_| false)[0].pid, Some(900));

        assert_eq!(stream(liar).users(|p| p == 900)[0].pid, Some(4242));
    }

    fn source(id: u64, muted: &str) -> String {
        format!(r#"[{{"id":{id},"type":"PipeWire:Interface:Node","info":{{"state":"suspended","props":{{"media.class":"Audio/Source","node.name":"mic{id}"}},
            "params":{{"Props":[{{"mute":{muted},"channelVolumes":[0.5]}},{{}}]}}}}}}]"#)
    }

    #[test]
    fn microphones_and_their_mute() {
        let pw = stream(&format!("{}\n{}\n{}", source(60, "false"), source(61, "true"), mic("running")));
        assert_eq!(pw.sources(), vec![
            Source { id: 60, name: "mic60".into(), muted: Some(false) },
            Source { id: 61, name: "mic61".into(), muted: Some(true) },
        ]);

        let later = r#"[{"id":60,"type":"PipeWire:Interface:Node","info":{"state":"running"}}]"#;
        let pw = stream(&format!("{}\n{later}", source(60, "true")));
        assert_eq!(pw.sources()[0].muted, Some(true));
    }

    #[test]
    fn the_switch_mutes_every_microphone_and_puts_back_only_its_own() {
        let t = Instant::now();
        let s = |id: u64, muted: bool| Source { id, name: format!("mic{id}"), muted: Some(muted) };
        let mut sw = MicSwitch::default();

        assert!(sw.plan(false, &[s(1, false), s(2, true)], t).is_empty());

        assert_eq!(sw.plan(true, &[s(1, false), s(2, true)], t), vec![(1, true)]);

        assert!(sw.plan(true, &[s(1, false), s(2, true)], t + Duration::from_millis(100)).is_empty());
        assert!(sw.plan(true, &[s(1, true), s(2, true)], t + Duration::from_millis(200)).is_empty());

        assert_eq!(sw.plan(true, &[s(1, false), s(2, true), s(3, false)], t + Duration::from_millis(300)), vec![(1, true), (3, true)]);

        assert!(sw.plan(true, &[s(1, true), s(2, true), s(3, false)], t + Duration::from_millis(400)).is_empty());
        assert_eq!(sw.plan(true, &[s(1, true), s(2, true), s(3, false)], t + ASK_AGAIN + Duration::from_millis(400)), vec![(3, true)]);

        assert_eq!(sw.plan(false, &[s(1, true), s(2, true)], t + Duration::from_secs(5)), vec![(1, false)]);
        assert!(sw.plan(false, &[s(1, false), s(2, true)], t + Duration::from_secs(6)).is_empty());
        assert_eq!(sw.plan(false, &[s(1, false), s(2, true), s(3, true)], t + Duration::from_secs(7)), vec![(3, false)]);
        assert!(sw.ours.is_empty());
    }

    #[test]
    fn camera_links() {
        assert!(is_video_link(Path::new("/dev/video0")));
        assert!(is_video_link(Path::new("/dev/video12")));
        assert!(!is_video_link(Path::new("/dev/video")));
        assert!(!is_video_link(Path::new("/dev/videox")));
        assert!(!is_video_link(Path::new("/dev/snd/pcmC0D0c")));
        assert!(!is_video_link(Path::new("socket:[1234]")));
    }

    #[test]
    fn camera_holders_from_a_made_up_proc() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("trindle-in-use-proc-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);

        for (pid, comm, exe, target) in [(100, "cheese", "/usr/bin/cheese", "/dev/video0"),
                                         (200, "pipewire", "/usr/bin/pipewire", "/dev/video0"),
                                         (300, "bash", "/usr/bin/bash", "/dev/null"),
                                         (400, "pipewire", "/var/tmp/pipewire", "/dev/video1")] {
            let at = root.join(pid.to_string());
            let fd = at.join("fd");
            std::fs::create_dir_all(&fd).unwrap();
            symlink(target, fd.join("3")).unwrap();
            symlink(exe, at.join("exe")).unwrap();
            std::fs::write(at.join("comm"), format!("{comm}\n")).unwrap();
        }
        std::fs::create_dir_all(root.join("self")).unwrap();
        assert_eq!(video_holders(&root), vec![(100, "cheese".to_string()), (400, "pipewire".to_string())]);
        assert!(is_middleman(&root, 200) && !is_middleman(&root, 400) && !is_middleman(&root, 999));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_screenshot_is_not_recording() {
        let t = Instant::now();
        let mut r = Recorders::default();
        r.note(9, t);
        assert!(r.recording(t).is_empty());
        assert!(r.recording(t + Duration::from_secs(1)).is_empty());

        r.note(9, t + Duration::from_secs(3));
        assert!(r.recording(t + Duration::from_secs(3)).is_empty());
    }

    #[test]
    fn steady_frames_are_recording_until_they_stop() {
        let t = Instant::now();
        let mut r = Recorders::default();
        for i in 0..30 {
            r.note(9, t + Duration::from_millis(33 * i));
        }
        let end = t + Duration::from_millis(33 * 29);
        assert_eq!(r.recording(end), vec![9]);
        assert_eq!(r.recording(end + Duration::from_millis(1900)), vec![9]);
        assert!(r.recording(end + GAP).is_empty());
    }

    #[test]
    fn a_still_screen_keeps_a_waiting_recorder_counted() {
        let t = Instant::now();
        let mut r = Recorders::default();
        for i in 0..10 {
            r.note(5, t + Duration::from_millis(500 * i));
        }
        assert_eq!(r.recording(t + Duration::from_millis(4500)), vec![5]);
    }

    #[test]
    fn stat_parent() {
        assert_eq!(parent_from_stat("1234 (firefox) S 1000 1234 1234 0 -1"), Some(1000));
        assert_eq!(parent_from_stat("77 (Web (Content) x) R 66 77 77"), Some(66));
        assert_eq!(parent_from_stat("garbage"), None);
    }

    #[test]
    fn processes_find_their_window_rows() {
        let owners = vec![(1000, "firefox".to_string()), (2000, "org.example.Recorder".to_string()), (3000, "Alacritty".to_string())];
        let parents: HashMap<u32, u32> = [(1500, 1000), (1600, 1500), (4000, 3000), (5000, 1)].into_iter().collect();
        let up = |p: u32| parents.get(&p).copied();

        assert_eq!(match_windows(Some(2000), "", &owners, up), vec!["org.example.Recorder"]);

        assert_eq!(match_windows(Some(1600), "Web Content", &owners, up), vec!["firefox"]);

        assert_eq!(match_windows(None, "Recorder", &owners, up), vec!["org.example.Recorder"]);
        assert_eq!(match_windows(None, "FIREFOX", &owners, up), vec!["firefox"]);

        assert_eq!(match_windows(Some(4000), "arecord", &owners, up), vec!["Alacritty"]);

        assert!(match_windows(Some(5000), "daemon", &owners, up).is_empty());
        assert!(match_windows(None, "", &owners, up).is_empty());
    }

    #[test]
    fn the_file_round_trips() {
        let users = vec![
            User { kind: Kind::Microphone, pid: Some(12), name: "Voice\tNotes".into(), app_ids: vec!["a.b".into(), "c".into()] },
            User { kind: Kind::Screen, pid: None, name: "obs".into(), app_ids: vec![] },
        ];
        let back = decode(&encode(&users));
        assert_eq!(back[0].name, "Voice Notes");
        assert_eq!(back[0].app_ids, vec!["a.b", "c"]);
        assert_eq!(back[1], users[1]);
        assert_eq!(mask(&back), Kind::Microphone.bit() | Kind::Screen.bit());
        assert!(decode("nonsense\n\tx\n").is_empty());
    }

    #[test]
    fn the_test_hook() {
        let users = fake("microphone, camera:foot ,screen,bogus");
        assert_eq!(users.len(), 3);
        assert_eq!(users[1].app_ids, vec!["foot"]);
        assert_eq!(mask(&users), 7);
    }

    #[test]
    fn pointing_names_the_programs() {
        let u = |kind, name: &str, ids: &[&str]| User { kind, pid: None, name: name.into(), app_ids: ids.iter().map(|s| s.to_string()).collect() };
        let users = vec![u(Kind::Screen, "obs", &[]), u(Kind::Microphone, "Firefox", &[]), u(Kind::Microphone, "Firefox", &[]),
                         u(Kind::Microphone, "", &["org.example.Talk"]), u(Kind::Camera, "", &[])];
        assert_eq!(lines(&users, false), vec!["Microphone in use by Firefox, org.example.Talk", "Camera in use", "Screen being recorded by obs"]);
        assert_eq!(lines(&users, true)[0], "Microphone switched off, wanted by Firefox, org.example.Talk");
        assert!(lines(&[], true).is_empty());
    }

    #[test]
    fn spoken_names() {
        assert_eq!(Kind::Microphone.spoken(&["Voice Notes"]), "Microphone in use by Voice Notes");
        assert_eq!(Kind::Screen.spoken(&[]), "Screen being recorded");
        assert_eq!(Kind::Input.spoken(&["One"]), "One is using your keyboard and mouse");
        assert_eq!(Kind::from_word("input"), Some(Kind::Input));
        assert_eq!(mask(&[User { kind: Kind::Input, pid: None, name: "a.b".into(), app_ids: vec![] }]), 8);
    }

    #[test]
    fn a_box_allowed_the_home_network_or_usb_devices_is_listed_while_it_runs() {
        let dir = std::env::temp_dir().join(format!("trindle-in-use-box-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let user = |kind: Kind, name: &str| User { kind, pid: None, name: name.into(), app_ids: vec!["org.example.Scan".into()] };
        let both = [user(Kind::HomeNetwork, "Scan"), user(Kind::Usb, "Scan")];
        let users = decode_box(&encode(&both));
        assert_eq!(users, both);
        assert_eq!(lines(&users, false), ["Home network in use by Scan", "Scanners and USB devices in use by Scan"]);
        assert_eq!(Kind::from_word("home-network"), Some(Kind::HomeNetwork));
        assert_eq!(Kind::from_word("usb"), Some(Kind::Usb));
        assert_eq!(mask(&users), 32 | 64);
        assert_eq!(Kind::HomeNetwork.icon(), "network-workgroup-symbolic");

        let note = |pid: u32, users: &[User]| write(&dir.join(format!("{BOX_PREFIX}{pid}")), users).unwrap();
        note(101, &both);
        note(202, &[user(Kind::Usb, "Other")]);
        let got = boxes_in(&dir, |pid| pid == 101);
        assert_eq!(got, both);
        assert!(!dir.join(format!("{BOX_PREFIX}202")).exists(), "a box that has gone is not listed and its note is removed");
        assert!(dir.join(format!("{BOX_PREFIX}101")).exists());

        let lying: Vec<User> = ["microphone", "camera", "screen", "input", "files"].iter().map(|k| user(Kind::from_word(k).unwrap(), "Spy")).collect();
        assert!(decode_box(&encode(&lying)).is_empty());

        assert_eq!(decode_box(&encode(&[both.clone(), both.clone()].concat())).len(), 2);
        assert_eq!(decode_box(&encode(&[user(Kind::Usb, "A\nmicrophone\tB")])).len(), 1);
        let _ = std::fs::remove_dir_all(&dir);

        let pid = std::process::id() + 1_000_000_000;
        let started = BoxRows::start(pid, &[(Kind::HomeNetwork, "Scan", "org.example.Scan"), (Kind::Usb, "Scan", "org.example.Scan")]).expect("written");
        let path = box_path(pid);
        assert_eq!(decode_box(&std::fs::read_to_string(&path).unwrap()), both);
        drop(started);
        assert!(!path.exists());
        assert!(BoxRows::start(1, &[]).is_none(), "nothing to say, no note");
        assert!(BoxRows::start(1, &[(Kind::Microphone, "Spy", "spy")]).is_none(), "a kind that is not a box's is not noted");
    }
}
