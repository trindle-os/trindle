//! What each tool in the fence does. Reading tools answer from the machine
//! as it is; changing tools are checked into a Plan first, so the person is
//! asked about exactly what will happen, and only then carried out.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use serde_json::{json, Value};
use wayland_client::Proxy;

use crate::activity::procs::Reader;
use crate::common::files::{data_home, default_setting, home, read_text, set_setting, setting, trindle_conf};
use crate::common::{access, audio, pointer, run, style};
use crate::ring::windows::{Window, Windows};

use crate::toolkit::t;
use super::outside::quoted;

fn listed(words: &[String]) -> String {
    words.iter().map(|w| quoted(w)).collect::<Vec<_>>().join(", ")
}

fn settable(key: &str) -> bool {
    !default_setting(key).is_empty()
}

const KEYS: &[&str] = &[
    "clock_format", "wallpaper_color", "dim_after", "lock_after", "lock_on_suspend", "lock", "blank_after",
    "suspend_after", "dim_after_battery", "screensaver_after", "lid_close", "lid_close_battery",
    "screensaver_after_battery", "blank_after_battery", "suspend_after_battery", "wallpaper_every",
    "wallpaper_order", "wallpaper_fit", "icon_source", pointer::SPEED, pointer::SCROLL, pointer::MOUSE_NATURAL,
    pointer::LEFT_HANDED, pointer::TAP, pointer::TOUCHPAD_NATURAL, pointer::QUIET_TYPING,
];

const KEEP: usize = 30;

const NAME_MOST: usize = 200;

pub struct Plan {
    pub question: String,
    action: Action,
}

enum Action {
    Open(String),
    Focus(u32),
    Close(u32),
    Set(String, String),
    Theme(String, Option<String>),
    Install(String),
    Remove(String),
    Permission(String, &'static str, bool),
    Network(String),
    Pair(String),
    Undo(PathBuf),
}

#[derive(Default)]
pub struct Desk {
    windows: Option<Option<Rc<Windows>>>,
    reader: Reader,
}

impl Desk {

    pub fn read(&mut self, tool: &str, args: &Value) -> Result<Value, String> {
        Ok(match tool {
            "list_windows" => json!(self.windows()?.iter().map(|w| json!({
                "id": w.handle.id().protocol_id(), "title": w.title, "app": w.app_id,
                "in_front": w.active, "minimised": w.minimized, "maximised": w.maximized,
            })).collect::<Vec<_>>()),
            "list_apps" => {
                let mut apps: Vec<Value> = crate::common::apps::entries().iter()
                    .map(|a| json!({ "id": a.key(), "name": a.name, "category": a.category }))
                    .collect();
                apps.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
                json!(apps)
            }
            "list_processes" => self.processes(),
            "system_status" => status(),
            "get_settings" => {
                let mut all = serde_json::Map::new();
                for k in KEYS {
                    all.insert(k.to_string(), json!(setting(k)));
                }
                all.insert("theme".into(), json!(if style::light() { "light" } else { "dark" }));
                all.insert("accent".into(), json!(style::accent()));
                Value::Object(all)
            }
            "read_logs" => logs(args)?,
            "list_crashes" => crashes(),
            "list_devices" => devices(),
            _ => return Err("No such reading tool.".into()),
        })
    }

    pub fn plan(&mut self, tool: &str, args: &Value) -> Result<Plan, String> {
        let text = |k: &str| args[k].as_str().map(str::trim).unwrap_or("").to_string();
        let (question, action) = match tool {
            "open_app" => plan_open(&text("app"))?,
            "focus_window" | "close_window" => self.plan_window(tool, args)?,
            "set_setting" => plan_setting(text("key"), text("value"))?,
            "set_theme" => plan_theme(text("look").to_lowercase(), text("accent").to_lowercase())?,
            "install_app" => plan_install(&text("app"))?,
            "remove_app" => plan_remove(&text("app"))?,
            "set_app_permission" => plan_permission(&text("app"), &text("permission"), args)?,
            "connect_network" => plan_network(text("network"))?,
            "pair_bluetooth" => plan_pair(text("address").to_uppercase())?,
            "undo_change" => plan_undo(text("snapshot"))?,
            _ => return Err("No such changing tool.".into()),
        };
        Ok(Plan { question, action })
    }

    fn plan_window(&mut self, tool: &str, args: &Value) -> Result<(String, Action), String> {
        let id = args["window"].as_u64().ok_or("window must be a number from list_windows.")? as u32;
        let win = self.window(id)?;
        let name = if win.title.is_empty() { win.app_id.clone() } else { win.title.clone() };
        Ok(if tool == "focus_window" {
            (t!("mcp-ask-focus", name = quoted(&name)), Action::Focus(id))
        } else {
            (format!("close {}", quoted(&name)), Action::Close(id))
        })
    }

    pub fn change(&mut self, plan: Plan) -> Result<String, String> {
        match plan.action {
            Action::Open(id) => {
                let app = find_app(&id).ok_or("The app has gone.")?;
                crate::common::apps::start_file(&app.path)?;
                Ok(format!("Opened {}.", quoted(&app.name)))
            }
            Action::Focus(id) => {
                let win = self.window(id)?;
                self.windows.clone().flatten().ok_or("No window list.")?.activate(&win);
                Ok("Brought to the front.".into())
            }
            Action::Close(id) => {
                let win = self.window(id)?;
                self.windows.clone().flatten().ok_or("No window list.")?.close(&win);
                Ok("Asked it to close.".into())
            }
            Action::Set(key, value) => {
                set_setting(&key, &value).map_err(|e| e.to_string())?;
                Ok(format!("{key} is now {}.", quoted(&value)))
            }
            Action::Theme(look, accent) => theme_now(&look, accent.as_deref()),
            Action::Install(id) => install_now(&id),
            Action::Remove(id) => {
                let argv = crate::common::removal::flatpak_uninstall(Some("--user"), &id, false);
                quiet(&argv.iter().map(String::as_str).collect::<Vec<_>>())?;
                Ok(format!("{id} is removed."))
            }
            Action::Permission(id, key, allow) => {
                access::flatpak_set(&id, key, allow)?;
                Ok(format!("Done: {key} {} for {id}. It takes effect the next time the app starts.",
                           if allow { "allowed" } else { "taken away" }))
            }
            Action::Network(name) => {
                quiet(&["nmcli", "--wait", "30", "device", "wifi", "connect", &name])?;
                Ok(format!("Joined {}.", quoted(&name)))
            }
            Action::Pair(address) => {
                for step in ["pair", "trust", "connect"] {
                    quiet(&["bluetoothctl", "--timeout", "20", step, &address])?;
                }
                Ok(format!("Paired and connected with {address}."))
            }
            Action::Undo(place) => undo(&place),
        }
    }

    fn windows(&mut self) -> Result<Vec<Window>, String> {

        #[cfg(test)]
        let connect = || None;
        #[cfg(not(test))]
        let connect = Windows::connect;
        let list = self.windows.get_or_insert_with(connect).clone().ok_or("No window list: not running on a Trindle screen.")?;
        list.settle();
        Ok(list.windows())
    }

    fn window(&mut self, id: u32) -> Result<Window, String> {
        self.windows()?.into_iter().find(|w| w.handle.id().protocol_id() == id).ok_or(format!("No window {id}; list_windows gives the ids."))
    }

    fn processes(&mut self) -> Value {
        let look = self.reader.look(&crate::activity::names(&crate::common::apps::entries()));
        let mut groups: Vec<_> = look.groups.iter().filter(|g| g.kb > 0).collect();
        groups.sort_by_key(|a| std::cmp::Reverse(a.kb));
        json!(groups.iter().take(40).map(|g| json!({
            "name": if g.owner.name.is_empty() { &g.owner.key } else { &g.owner.name },
            "memory_mb": g.kb / 1024,
            "processes": g.members.len(),
        })).collect::<Vec<_>>())
    }
}

fn plan_open(id: &str) -> Result<(String, Action), String> {
    let app = find_app(id).ok_or(format!("No app called {} is installed.", quoted(id)))?;
    Ok((format!("open {}", quoted(&app.name)), Action::Open(app.id)))
}

fn plan_setting(key: String, value: String) -> Result<(String, Action), String> {
    check_setting(&key, &value)?;
    Ok((t!("mcp-ask-setting", key = key.as_str(), from = quoted(&setting(&key)), to = quoted(&value)), Action::Set(key, value)))
}

fn plan_theme(look: String, accent: String) -> Result<(String, Action), String> {
    if look != "light" && look != "dark" {
        return Err("look must be light or dark.".into());
    }
    let accent = Some(accent).filter(|a| !a.is_empty());
    if let Some(a) = &accent {
        if !hex(a) {
            return Err("accent must be a colour written like #ffc107.".into());
        }
    }
    let q = match &accent {
        Some(a) => t!("mcp-ask-theme-accent", look = look.as_str(), accent = a),
        None => t!("mcp-ask-theme", look = look.as_str()),
    };
    Ok((q, Action::Theme(look, accent)))
}

fn plan_install(app: &str) -> Result<(String, Action), String> {
    let id = flatpak_id(app)?;
    if user_apps().contains(&id) {
        return Err(format!("{} is already installed.", quoted(&id)));
    }
    Ok((t!("mcp-ask-install", id = id.as_str()), Action::Install(id)))
}

fn plan_remove(app: &str) -> Result<(String, Action), String> {
    let id = flatpak_id(app)?;
    if !user_apps().contains(&id) {
        return Err(format!("{} was not installed from Flathub by this person, so it is left alone.", quoted(&id)));
    }
    Ok((t!("mcp-ask-remove", id = id.as_str()), Action::Remove(id)))
}

fn plan_permission(app: &str, what: &str, args: &Value) -> Result<(String, Action), String> {
    let id = flatpak_id(app)?;
    if !user_apps().contains(&id) {
        return Err(format!("{} is not an app this person installed from Flathub.", quoted(&id)));
    }
    let allow = args["allow"].as_bool().ok_or("allow must be true or false.")?;
    let p = access::FLATPAK.iter().find(|p| p.key == what)
        .ok_or("permission must be network, home, devices, sound or bluetooth.")?;
    let q = if allow { format!("let {id} use {what}") } else { format!("stop {id} using {what}") };
    Ok((q, Action::Permission(id, p.key, allow)))
}

fn plan_network(name: String) -> Result<(String, Action), String> {
    if name.is_empty() || name.chars().any(char::is_control) {
        return Err("That is not a network name.".into());
    }
    if !known_network(&name) && !open_network(&name) {
        return Err(format!("{} needs a password. Passwords are never passed through an assistant: \
            the person can join it from the Wi-Fi list in the ring.", quoted(&name)));
    }
    Ok((t!("mcp-ask-network", name = quoted(&name)), Action::Network(name)))
}

fn plan_pair(address: String) -> Result<(String, Action), String> {
    if !bluetooth_address(&address) {
        return Err("address must look like 00:11:22:33:44:55.".into());
    }
    Ok((t!("mcp-ask-pair", address = address.as_str()), Action::Pair(address)))
}

fn plan_undo(snapshot: String) -> Result<(String, Action), String> {
    let place = match snapshot {
        n if n.is_empty() => snapshots().pop().ok_or("No snapshots have been taken yet.")?,
        n if n.chars().all(|c| c.is_ascii_digit()) && snapshots_dir().join(&n).is_dir() => snapshots_dir().join(n),
        n => return Err(format!("No snapshot called {}.", quoted(&n))),
    };
    let note = read_text(place.join("note")).trim().to_string();
    Ok((t!("mcp-ask-undo", note = quoted(&note)), Action::Undo(place)))
}

fn theme_now(look: &str, accent: Option<&str>) -> Result<String, String> {
    set_setting("theme", look).map_err(|e| e.to_string())?;
    if let Some(a) = accent {
        set_setting("accent", a).map_err(|e| e.to_string())?;
    }
    restyle();

    if (look == "light") == style::kept_light() {
        Ok(format!("The {look} look is on."))
    } else {
        Ok(format!("The theme in use has no {look} look, so nothing changes now. {look} is saved for a theme that has both."))
    }
}

fn install_now(id: &str) -> Result<String, String> {
    let added = crate::store::catalogue::add_flathub(crate::store::catalogue::REMOTE);
    let added: Vec<&str> = added.iter().map(String::as_str).collect();
    quiet(&added)?;
    quiet(&crate::common::gentle::argv(&["flatpak", "install", "--user", "--noninteractive", "-y", crate::store::catalogue::REMOTE, &format!("app/{id}")]))?;
    Ok(format!("{id} is installed."))
}

fn find_app(id: &str) -> Option<crate::common::apps::Entry> {
    let id = id.trim_end_matches(".desktop");
    crate::common::apps::entries().into_iter().find(|a| a.key() == id)
}

pub fn check_setting(key: &str, value: &str) -> Result<(), String> {
    if !settable(key) {
        return Err(format!("{} is not a setting an assistant may change; get_settings lists those that are.", quoted(key)));
    }
    if value.is_empty() || value.len() > NAME_MOST || value.chars().any(char::is_control) {
        return Err("The value must be one short line of text.".into());
    }
    Ok(())
}

fn hex(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

pub fn flatpak_id(id: &str) -> Result<String, String> {
    let parts: Vec<&str> = id.split('.').collect();
    let good = id.len() <= 255
        && parts.len() >= 3
        && parts.iter().all(|p| {
            p.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        });
    if good { Ok(id.to_string()) } else { Err(format!("{} is not a Flathub app id, which looks like org.gnome.Calculator.", quoted(id))) }
}

pub fn bluetooth_address(a: &str) -> bool {
    a.len() == 17 && a.split(':').count() == 6 && a.split(':').all(|p| p.len() == 2 && p.chars().all(|c| c.is_ascii_hexdigit()))
}

pub fn unit_name(u: &str) -> bool {
    !u.is_empty() && u.len() <= NAME_MOST && !u.starts_with('-')
        && u.chars().all(|c| c.is_ascii_alphanumeric() || "@._-".contains(c))
}

fn user_apps() -> Vec<String> {
    run::text(&["flatpak", "list", "--user", "--app", "--columns=application"]).lines().map(str::to_string).collect()
}

fn known_network(name: &str) -> bool {
    run::text(&["nmcli", "-t", "-f", "NAME,TYPE", "connection", "show"]).lines()
        .any(|l| l.rsplit_once(':').is_some_and(|(n, t)| n.replace("\\:", ":") == name && t.contains("wireless")))
}

fn open_network(name: &str) -> bool {
    run::text(&["nmcli", "-t", "-f", "SSID,SECURITY", "device", "wifi", "list"]).lines()
        .any(|l| l.rsplit_once(':').is_some_and(|(n, s)| n.replace("\\:", ":") == name && (s.is_empty() || s == "--")))
}

fn quiet(argv: &[&str]) -> Result<(), String> {
    let out = Command::new(argv[0]).args(&argv[1..]).output().map_err(|e| format!("{}: {e}", argv[0]))?;
    if out.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        let out_text = String::from_utf8_lossy(&out.stdout);
        let why = err.lines().chain(out_text.lines()).rfind(|l| !l.trim().is_empty()).unwrap_or("it failed");
        Err(format!("{} said: {}", argv[0], quoted(why)))
    }
}

fn restyle() {
    style::apply_system_theme();
    run::spawn(&["trindle-ring", "--restyle"]);
}

fn status() -> Value {
    let meminfo = read_text("/proc/meminfo");
    let kb = |k| crate::activity::procs::field_kb(&meminfo, k);
    let uptime = read_text("/proc/uptime").split('.').next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    let mut batteries = Vec::new();
    if let Ok(dir) = std::fs::read_dir("/sys/class/power_supply") {
        for p in dir.flatten().map(|e| e.path()) {
            let get = |f: &str| read_text(p.join(f)).trim().to_string();
            if get("type") == "Battery" && get("scope") != "Device" {
                batteries.push(json!({ "percent": get("capacity").parse::<u32>().ok(), "state": get("status") }));
            }
        }
    }
    let disk = run::text(&["df", "-h", "--output=size,avail", "/"]).lines().nth(1).map(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        json!({ "size": f.first(), "free": f.get(1) })
    });
    json!({
        "trindle": env!("CARGO_PKG_VERSION"),
        "up_minutes": uptime / 60,
        "memory_mb": { "total": kb("MemTotal:") / 1024, "available": kb("MemAvailable:") / 1024 },
        "batteries": batteries,
        "network": run::text(&["nmcli", "-t", "-f", "DEVICE,TYPE,STATE,CONNECTION", "device"]).lines().collect::<Vec<_>>(),
        "volume": run::text(&["wpctl", "get-volume", "@DEFAULT_AUDIO_SINK@"]),
        "look": if style::light() { "light" } else { "dark" },
        "accent": style::accent(),
        "disk": disk,
    })
}

fn logs(args: &Value) -> Result<Value, String> {
    let lines = args["lines"].as_u64().unwrap_or(100).clamp(1, 500).to_string();
    let mut argv = vec!["journalctl", "--no-pager", "-q", "-o", "short-iso", "-n", &lines];
    let unit = args["unit"].as_str().unwrap_or("").trim().to_string();
    if !unit.is_empty() {
        if !unit_name(&unit) {
            return Err("That is not a service name.".into());
        }
        argv.extend(["-u", &unit]);
    }
    Ok(json!(run::text(&argv).lines().collect::<Vec<_>>()))
}

fn devices() -> Value {
    let sound = audio::parse_dump(&run::text(&["pw-dump"])).map(|d| {
        let d = audio::parse_nodes(&d);
        json!({
            "outputs": d.outputs.iter().map(|n| &n.label).collect::<Vec<_>>(),
            "inputs": d.inputs.iter().map(|n| &n.label).collect::<Vec<_>>(),
            "output_now": d.current_output().map(|n| &n.label),
            "input_now": d.current_input().map(|n| &n.label),
        })
    });
    let bluetooth: Vec<Value> = run::text(&["bluetoothctl", "devices"]).lines().filter_map(|l| {
        let mut f = l.splitn(3, ' ');
        (f.next() == Some("Device")).then(|| json!({ "address": f.next(), "name": f.next() }))
    }).collect();
    let wifi = run::text(&["nmcli", "-t", "-f", "IN-USE,SSID,SIGNAL,SECURITY", "device", "wifi", "list"]);
    let wifi: Vec<&str> = wifi.lines().collect();
    let drives: Value = serde_json::from_str(&run::text(&["lsblk", "-J", "-o", "NAME,SIZE,TYPE,MOUNTPOINT,LABEL"])).unwrap_or(Value::Null);
    json!({ "sound": sound, "bluetooth": bluetooth, "wifi": wifi, "drives": drives })
}

fn snapshots_dir() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME").map(PathBuf::from).filter(|p| p.is_absolute())
        .unwrap_or_else(|| home().join(".local/state"))
        .join("trindle/assistant")
}

fn overrides_dir() -> PathBuf {
    data_home().join("flatpak/overrides")
}

fn snapshots() -> Vec<PathBuf> {
    let Ok(dir) = std::fs::read_dir(snapshots_dir()) else { return Vec::new() };
    let mut all: Vec<PathBuf> = dir.flatten().map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.chars().all(|c| c.is_ascii_digit())))
        .collect();
    all.sort();
    all
}

pub fn snapshot(note: &str) -> Result<String, String> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    let name = now.to_string();
    let place = snapshots_dir().join(&name);
    let fail = |e: std::io::Error| format!("No snapshot could be taken, so nothing was changed: {e}");
    std::fs::create_dir_all(place.join("overrides")).map_err(fail)?;
    if trindle_conf().is_file() {
        std::fs::copy(trindle_conf(), place.join("trindle.conf")).map_err(fail)?;
    }
    copy_files(&overrides_dir(), &place.join("overrides")).map_err(fail)?;
    std::fs::write(place.join("apps"), user_apps().join("\n")).map_err(fail)?;
    std::fs::write(place.join("note"), note).map_err(fail)?;
    let all = snapshots();
    for old in all.iter().take(all.len().saturating_sub(KEEP)) {
        let _ = std::fs::remove_dir_all(old);
    }
    Ok(name)
}

fn copy_files(from: &Path, to: &Path) -> std::io::Result<()> {
    let Ok(dir) = std::fs::read_dir(from) else { return Ok(()) };
    for e in dir.flatten() {
        if e.file_type().is_ok_and(|t| t.is_file()) {
            std::fs::copy(e.path(), to.join(e.file_name()))?;
        }
    }
    Ok(())
}

fn undo(place: &Path) -> Result<String, String> {
    let kept = place.join("trindle.conf");
    if kept.is_file() {

        let was = std::fs::read(&kept).map_err(|e| e.to_string())?;
        crate::common::files::write_whole(&trindle_conf(), &was).map_err(|e| e.to_string())?;
    } else {
        let _ = std::fs::remove_file(trindle_conf());
    }

    let now = overrides_dir();
    if let Ok(dir) = std::fs::read_dir(&now) {
        for e in dir.flatten() {
            if !place.join("overrides").join(e.file_name()).exists() {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    std::fs::create_dir_all(&now).map_err(|e| e.to_string())?;
    copy_files(&place.join("overrides"), &now).map_err(|e| e.to_string())?;
    restyle();
    let then: Vec<String> = read_text(place.join("apps")).lines().map(str::to_string).collect();
    let apps = user_apps();
    let mut said = String::from("Settings and app permissions are back as they were.");
    let added: Vec<String> = apps.iter().filter(|a| !then.contains(a)).cloned().collect();
    let gone: Vec<String> = then.iter().filter(|a| !apps.contains(a)).cloned().collect();
    if !added.is_empty() {
        let _ = write!(said, " Installed since, and left in place: {}.", listed(&added));
    }
    if !gone.is_empty() {
        let _ = write!(said, " Removed since, and not put back: {}.", listed(&gone));
    }
    Ok(said)
}

fn crashes() -> Value {
    let text = crate::common::crash::history().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
    let list: Vec<Value> = text.lines().rev().filter_map(|l| serde_json::from_str(l).ok()).take(20).collect();
    json!(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_known_settings_with_plain_values() {
        assert!(check_setting("lock_after", "15").is_ok());
        assert!(check_setting(pointer::TAP, "no").is_ok());
        for key in ["theme", "accent", "PATH", "", "../x", "nonsense"] {
            assert!(check_setting(key, "1").is_err(), "{key}");
        }
        for value in ["", "a\nb=c", "x\r", &"y".repeat(201)] {
            assert!(check_setting("lock_after", value).is_err(), "{value:?}");
        }
        assert!(KEYS.iter().all(|k| settable(k)));
    }

    #[test]
    fn flathub_ids_cannot_be_options_or_paths() {
        assert!(flatpak_id("org.gnome.Calculator").is_ok());
        assert!(flatpak_id("com.github.tchx84.Flatseal").is_ok());
        assert!(flatpak_id("io.github.some_one.App-Name").is_ok());
        for bad in ["--system", "org.gnome", "-org.a.b", "org..b.c", "org/a.b.c", "org.a.b c", "../a.b.c", "", "org.1a.b"] {
            assert!(flatpak_id(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn addresses_and_units() {
        assert!(bluetooth_address("00:11:22:AA:bb:FF"));
        assert!(!bluetooth_address("00:11:22:33:44"));
        assert!(!bluetooth_address("--help:11:22:33:44"));
        assert!(unit_name("NetworkManager.service"));
        assert!(unit_name("user@1000.service"));
        assert!(!unit_name("-f"));
        assert!(!unit_name("a b"));
        assert!(!unit_name("a;b"));
    }

    #[test]
    fn plans_refuse_before_asking() {
        let mut d = Desk::default();
        let no = |d: &mut Desk, tool: &str, args: Value| d.plan(tool, &args).is_err();
        assert!(no(&mut d, "set_setting", json!({ "key": "theme", "value": "light" })));
        assert!(no(&mut d, "set_theme", json!({ "look": "purple" })));
        assert!(no(&mut d, "set_theme", json!({ "look": "dark", "accent": "red" })));
        assert!(no(&mut d, "install_app", json!({ "app": "--system" })));
        assert!(no(&mut d, "pair_bluetooth", json!({ "address": "nope" })));
        assert!(no(&mut d, "undo_change", json!({ "snapshot": "../../etc" })));
        let p = d.plan("set_theme", &json!({ "look": "Light", "accent": "#FFC107" })).unwrap();
        assert_eq!(p.question, "switch to the light look with the accent colour #ffc107");
    }
}
