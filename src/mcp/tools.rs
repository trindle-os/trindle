//! What each tool in the fence does. Reading tools answer from the machine
//! as it is; changing tools are checked into a Plan first, so the person is
//! asked about exactly what will happen, and only then carried out.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use serde_json::{json, Value};
use wayland_client::Proxy;

use crate::activity::procs::Reader;
use crate::common::files::{data_home, home, read_text, set_setting, setting, state_home, trindle_conf};
use crate::common::{access, audio, pointer, run, style};
use crate::ring::windows::{Window, Windows};

use crate::toolkit::t;
use super::mend;
use super::outside::quoted;
use crate::report::scrub::unescape;

fn listed(words: &[String]) -> String {
    words.iter().map(|w| quoted(w)).collect::<Vec<_>>().join(", ")
}

fn settable(key: &str) -> bool {
    KEYS.contains(&key)
}

const KEYS: &[&str] = &[
    "clock_format", "wallpaper_color", "dim_after", "blank_after",
    "suspend_after", "dim_after_battery", "screensaver_after",
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
    Focus(Target),
    Close(Target),
    Set(String, String),
    Theme(String, Option<String>),
    Install(String),
    Remove(String),
    Permission(String, &'static str, bool),
    Network(String),
    Pair(String),
    Undo(PathBuf),
    Restart(String),
    AppSetting(AppLine),
    ClearCache(String),
}

struct AppLine {
    id: String,
    file: String,
    section: String,
    key: String,
    value: String,
}

const APP: &str = "app";

const SINCE: &str = "cache-since";

const TRINDLE: &str = "That is part of Trindle, which an assistant never changes. Pass the fault on as a bug report \
instead; list_crashes gives what is known.";

impl Plan {

    fn keeps(&self, home: &Path, part: &Path) -> std::io::Result<()> {
        let id = match &self.action {
            Action::AppSetting(l) => &l.id,
            Action::ClearCache(id) => id,
            _ => return Ok(()),
        };
        let app = part.join(APP);
        std::fs::create_dir(&app)?;
        write_synced(&app.join("id"), id)?;
        if let Action::AppSetting(l) = &self.action {
            let (text, mode) = mend::read(home, &l.id, &l.file).map_err(std::io::Error::other)?;
            write_synced(&app.join("file"), &l.file)?;
            write_synced(&app.join("mode"), &mode.to_string())?;
            write_synced(&app.join("settings"), &text)?;
        }
        Ok(())
    }
}

struct Target {
    id: u32,
    title: String,
    app_id: String,
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
            "list_app_settings" => mend::names(&home(), &mendable(args["app"].as_str().unwrap_or("").trim())?)?,
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
            "restart_app" => plan_restart(&text("app"))?,
            "set_app_setting" => plan_app_setting(AppLine {
                id: text("app"), file: text("file"), section: text("section"), key: text("key"), value: text("value"),
            })?,
            "clear_app_cache" => plan_cache(&text("app"))?,
            _ => return Err("No such changing tool.".into()),
        };
        Ok(Plan { question, action })
    }

    fn plan_window(&mut self, tool: &str, args: &Value) -> Result<(String, Action), String> {
        let id = args["window"].as_u64().and_then(|n| u32::try_from(n).ok()).ok_or("window must be a number from list_windows.")?;
        let win = self.window(id)?;
        let name = quoted(if win.title.is_empty() { &win.app_id } else { &win.title });
        let target = Target { id, title: win.title, app_id: win.app_id };
        Ok(if tool == "focus_window" {
            (t!("mcp-ask-focus", name = name), Action::Focus(target))
        } else {
            (t!("mcp-ask-close", name = name), Action::Close(target))
        })
    }

    pub fn change(&mut self, plan: Plan, place: &Path) -> Result<String, String> {
        match plan.action {
            Action::Open(id) => {
                let app = find_app(&id).ok_or("The app has gone.")?;
                crate::common::apps::start_file(&app.path)?;
                Ok(format!("Opened {}.", quoted(&app.name)))
            }
            Action::Focus(target) => {
                let win = self.same_window(&target)?;
                self.windows.clone().flatten().ok_or("No window list.")?.activate(&win);
                Ok("Brought to the front.".into())
            }
            Action::Close(target) => {
                let win = self.same_window(&target)?;
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
            Action::Restart(id) => {
                stop(&id)?;
                if !run::spawn(&["flatpak", "run", &id]) {
                    return Err(format!("{id} was closed and could not be started again."));
                }
                Ok(format!("{id} was closed and started again."))
            }
            action => {
                if let Action::AppSetting(AppLine { id, .. }) | Action::ClearCache(id) = &action {
                    stop(id)?;
                }
                mend_now(action, &home(), place)
            }
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

    fn same_window(&mut self, target: &Target) -> Result<Window, String> {
        let win = self.window(target.id)?;
        if win.title == target.title && win.app_id == target.app_id {
            Ok(win)
        } else {
            Err("That window has gone or changed since the question, so nothing was done.".into())
        }
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
    Ok((t!("mcp-ask-open", name = quoted(&app.name)), Action::Open(app.id)))
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
    let q = if allow { t!("mcp-ask-allow", id = id.as_str(), what = what) } else { t!("mcp-ask-deny", id = id.as_str(), what = what) };
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
    let question = match bluetooth_name(&address) {
        Some(name) => t!("mcp-ask-pair-named", name = quoted(&name), address = address.as_str()),
        None => t!("mcp-ask-pair", address = address.as_str()),
    };
    Ok((question, Action::Pair(address)))
}

fn plan_undo(snapshot: String) -> Result<(String, Action), String> {
    let place = pick(&snapshots_dir(), snapshot)?;
    let note = quoted(read_text(place.join("note")).trim());
    let app = read_text(place.join(APP).join("id")).trim().to_string();
    let question = if app.is_empty() {
        t!("mcp-ask-undo", note = note)
    } else {
        t!("mcp-ask-undo-app", id = kept_id(&app)?.as_str(), note = note)
    };
    Ok((question, Action::Undo(place)))
}

fn mendable(app: &str) -> Result<String, String> {
    let id = flatpak_id(app)?;
    if mend::trindle_own(&id) {
        return Err(TRINDLE.into());
    }
    if !user_apps().contains(&id) {
        return Err(format!("{} is not an app this person installed from Flathub, so it is left alone.", quoted(&id)));
    }
    Ok(id)
}

fn kept_id(id: &str) -> Result<String, String> {
    let id = flatpak_id(id)?;
    if mend::trindle_own(&id) { Err(TRINDLE.into()) } else { Ok(id) }
}

fn plan_restart(app: &str) -> Result<(String, Action), String> {
    let id = mendable(app)?;
    Ok((t!("mcp-ask-restart", id = id.as_str()), Action::Restart(id)))
}

fn plan_app_setting(mut line: AppLine) -> Result<(String, Action), String> {
    line.id = mendable(&line.id)?;
    mend::check_line(&line.section, &line.key, &line.value)?;
    let (text, _) = mend::read(&home(), &line.id, &line.file)?;
    let (id, file, key, to) = (line.id.as_str(), quoted(&line.file), quoted(&line.key), quoted(&line.value));
    let question = match mend::value_of(&text, &line.section, &line.key) {
        Some(from) => t!("mcp-ask-app-setting", id = id, key = key, file = file, from = quoted(&from), to = to),
        None => t!("mcp-ask-app-setting-new", id = id, key = key, file = file, to = to),
    };
    Ok((question, Action::AppSetting(line)))
}

fn plan_cache(app: &str) -> Result<(String, Action), String> {
    let id = mendable(app)?;
    if !mend::has_cache(&home(), &id)? {
        return Err(format!("{id} has no cache to clear."));
    }
    Ok((t!("mcp-ask-cache", id = id.as_str()), Action::ClearCache(id)))
}

fn stop(id: &str) -> Result<(), String> {
    use crate::common::removal::flatpak_running;
    if !flatpak_running(id) {
        return Ok(());
    }
    quiet(&["flatpak", "kill", id])?;
    for _ in 0..20 {
        if !flatpak_running(id) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    Err(format!("{id} is still running, so nothing was changed."))
}

fn mend_now(action: Action, home: &Path, place: &Path) -> Result<String, String> {
    match action {
        Action::AppSetting(l) => {
            let (text, mode) = mend::read(home, &l.id, &l.file)?;
            let new = mend::edited(&text, &l.section, &l.key, &l.value);
            if new.len() as u64 > mend::MOST_FILE {
                return Err("The file would be too big, so it is left alone.".into());
            }
            mend::write(home, &l.id, &l.file, &new, mode)?;
            Ok(format!("{} in {} is now {} for {}.", quoted(&l.key), quoted(&l.file), quoted(&l.value), l.id))
        }
        Action::ClearCache(id) => {
            mend::move_cache(home, &id, &place.join(APP), mend::CACHE)?;
            Ok(format!("The cache of {id} is cleared. It is kept with the snapshot, so undo_change can put it back."))
        }
        _ => Err("No such repair.".into()),
    }
}

fn put_app_back(home: &Path, place: &Path, stop: impl Fn(&str) -> Result<(), String>) -> Result<String, String> {
    let app = place.join(APP);
    let id = read_text(app.join("id")).trim().to_string();
    if id.is_empty() {
        return Ok(String::new());
    }
    let id = kept_id(&id)?;
    stop(&id)?;
    let mut said = String::new();
    if app.join("settings").is_file() {
        let file = read_text(app.join("file")).trim().to_string();
        let text = std::fs::read_to_string(app.join("settings")).map_err(|e| e.to_string())?;
        let mode = read_text(app.join("mode")).trim().parse::<u32>().unwrap_or(0o600);
        mend::write(home, &id, &file, &text, mode)?;
        let _ = write!(said, " {} of {id} is back as it was.", quoted(&file));
    }
    if std::fs::symlink_metadata(app.join(mend::CACHE)).is_ok_and(|m| m.is_dir()) {
        mend::put_cache_back(home, &id, &app, mend::CACHE, SINCE)?;
        let _ = write!(said, " The cache of {id} is back.");
    }
    Ok(said)
}

fn pick(dir: &Path, name: String) -> Result<PathBuf, String> {
    match name {
        n if n.is_empty() => snapshots(dir).pop().ok_or_else(|| "No snapshots have been taken yet.".into()),
        n if numbered(&n) && whole(&dir.join(&n)) => Ok(dir.join(n)),
        n => Err(format!("No snapshot called {}.", quoted(&n))),
    }
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
    if !value_fits(key, value) {
        return Err(format!("{} is not a value {key} takes.", quoted(value)));
    }
    Ok(())
}

fn value_fits(key: &str, value: &str) -> bool {
    let whole = |most: u32| value.parse::<u32>().is_ok_and(|n| n <= most);
    let within = |r: &pointer::Range| value.parse::<f64>().is_ok_and(|n| n.is_finite() && (r.least..=r.most).contains(&n));
    match key {
        "wallpaper_every" => whole(7 * 24 * 3600),
        "wallpaper_order" => ["shuffle", "name"].contains(&value),
        "wallpaper_fit" => ["fill", "fit", "center", "stretch", "tile"].contains(&value),
        "wallpaper_color" => hex(value),
        pointer::SPEED => within(&pointer::SPEEDS),
        pointer::SCROLL => within(&pointer::SCROLLS),
        pointer::MOUSE_NATURAL | pointer::LEFT_HANDED | pointer::TAP | pointer::TOUCHPAD_NATURAL | pointer::QUIET_TYPING => ["yes", "no"].contains(&value),
        k if k.contains("_after") => whole(24 * 60),
        _ => true,
    }
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

fn bluetooth_name(address: &str) -> Option<String> {
    name_in(&run::text(&["bluetoothctl", "devices"]), address)
}

fn name_in(devices: &str, address: &str) -> Option<String> {
    devices.lines().find_map(|l| {
        let mut f = l.splitn(3, ' ');
        (f.next() == Some("Device") && f.next().is_some_and(|a| a.eq_ignore_ascii_case(address))).then(|| f.next().map(str::to_string)).flatten()
    }).filter(|n| !n.trim().is_empty())
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
        .any(|l| l.rsplit_once(':').is_some_and(|(n, t)| unescape(n) == name && t.contains("wireless")))
}

fn open_network(name: &str) -> bool {
    run::text(&["nmcli", "-t", "-f", "SSID,SECURITY", "device", "wifi", "list"]).lines()
        .any(|l| l.rsplit_once(':').is_some_and(|(n, s)| unescape(n) == name && (s.is_empty() || s == "--")))
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
    state_home().join("trindle/assistant")
}

fn overrides_dir() -> PathBuf {
    data_home().join("flatpak/overrides")
}

const WHOLE: &str = "whole";

const STALE: std::time::Duration = std::time::Duration::from_secs(3600);

fn numbered(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_digit())
}

fn whole(place: &Path) -> bool {
    place.join(WHOLE).is_file()
}

fn snapshots(dir: &Path) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut all: Vec<PathBuf> = read.flatten().map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(numbered) && p.is_dir() && whole(p))
        .collect();
    all.sort();
    all
}

pub fn snapshot(plan: &Plan) -> Result<(String, PathBuf), String> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    let name = now.to_string();
    let dir = snapshots_dir();
    let extra = |part: &Path| plan.keeps(&home(), part);
    let place = keep(&dir, &name, &trindle_conf(), &overrides_dir(), &user_apps(), &plan.question, &extra)
        .map_err(|e| format!("No snapshot could be taken, so nothing was changed: {e}"))?;
    tidy(&dir, STALE);
    Ok((name, place))
}

fn tidy(dir: &Path, stale: std::time::Duration) {
    let all = snapshots(dir);
    for old in all.iter().take(all.len().saturating_sub(KEEP)) {

        if std::fs::remove_file(old.join(WHOLE)).is_ok() {
            let _ = std::fs::remove_dir_all(old);
        }
    }
    let Ok(read) = std::fs::read_dir(dir) else { return };
    for e in read.flatten() {
        let p = e.path();
        let old = e.metadata().and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|age| age >= stale);
        let kept = e.file_name().to_str().is_some_and(numbered) && whole(&p);
        if old && !kept {
            let _ = std::fs::remove_dir_all(&p);
        }
    }
}

fn keep(dir: &Path, name: &str, conf: &Path, overrides: &Path, apps: &[String], note: &str,
        extra: &dyn Fn(&Path) -> std::io::Result<()>) -> std::io::Result<PathBuf> {
    let part = dir.join(format!("{name}.part"));
    std::fs::create_dir_all(dir)?;
    std::fs::create_dir(&part)?;
    let place = dir.join(name);
    let built = (|| {
        std::fs::create_dir(part.join("overrides"))?;
        match std::fs::metadata(conf) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            m => if m?.is_file() {
                copy_synced(conf, &part.join("trindle.conf"))?;
            },
        }
        copy_files(overrides, &part.join("overrides"))?;
        write_synced(&part.join("apps"), &apps.join("\n"))?;
        write_synced(&part.join("note"), note)?;
        extra(&part)?;
        write_synced(&part.join(WHOLE), "")?;
        std::fs::rename(&part, &place)
    })();
    if built.is_err() {
        let _ = std::fs::remove_dir_all(&part);
    }
    built.map(|()| place)
}

fn put_back(place: &Path, conf: &Path, overrides: &Path) -> Result<(), String> {
    let s = |e: std::io::Error| e.to_string();
    let kept = place.join("trindle.conf");
    if kept.is_file() {

        let was = std::fs::read(&kept).map_err(s)?;
        crate::common::files::write_whole(conf, &was).map_err(s)?;
    } else {
        absent(std::fs::remove_file(conf)).map_err(s)?;
    }

    for e in listing(overrides).map_err(s)?.into_iter().flatten() {
        let e = e.map_err(s)?;
        if !place.join("overrides").join(e.file_name()).try_exists().map_err(s)? {
            absent(std::fs::remove_file(e.path())).map_err(s)?;
        }
    }
    std::fs::create_dir_all(overrides).map_err(s)?;
    copy_files(&place.join("overrides"), overrides).map_err(s)
}

fn absent(r: std::io::Result<()>) -> std::io::Result<()> {
    match r {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        r => r,
    }
}

fn listing(dir: &Path) -> std::io::Result<Option<std::fs::ReadDir>> {
    match std::fs::read_dir(dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        r => r.map(Some),
    }
}

fn copy_files(from: &Path, to: &Path) -> std::io::Result<()> {
    for e in listing(from)?.into_iter().flatten() {
        let e = e?;
        if e.file_type()?.is_file() {
            copy_synced(&e.path(), &to.join(e.file_name()))?;
        }
    }
    Ok(())
}

fn copy_synced(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::copy(from, to)?;
    std::fs::File::open(to)?.sync_all()
}

fn write_synced(to: &Path, text: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(to)?;
    f.write_all(text.as_bytes())?;
    f.sync_all()
}

fn undo(place: &Path) -> Result<String, String> {
    let app = put_app_back(&home(), place, stop)?;
    put_back(place, &trindle_conf(), &overrides_dir())?;
    restyle();
    let then: Vec<String> = read_text(place.join("apps")).lines().map(str::to_string).collect();
    let apps = user_apps();
    let mut said = format!("Settings and app permissions are back as they were.{app}");
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
    fn a_snapshot_then_an_undo_puts_settings_and_permissions_back() {
        let dir = std::env::temp_dir().join(format!("trindle-assistant-undo-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (conf, overrides, snaps) = (dir.join("trindle.conf"), dir.join("overrides"), dir.join("snaps"));
        std::fs::create_dir_all(&overrides).unwrap();
        std::fs::write(&conf, "lock_after=15\n").unwrap();
        std::fs::write(overrides.join("org.example.Kept"), "[Context]\nfilesystems=home\n").unwrap();
        let place = keep(&snaps, "1", &conf, &overrides, &["org.example.Kept".into()], "before a change", &|_| Ok(())).unwrap();

        std::fs::write(&conf, "lock_after=1\n").unwrap();
        std::fs::write(overrides.join("org.example.Kept"), "[Context]\nfilesystems=host\n").unwrap();
        std::fs::write(overrides.join("org.example.New"), "[Context]\nshared=network\n").unwrap();
        put_back(&place, &conf, &overrides).unwrap();
        assert_eq!(std::fs::read_to_string(&conf).unwrap(), "lock_after=15\n");
        assert_eq!(std::fs::read_to_string(overrides.join("org.example.Kept")).unwrap(), "[Context]\nfilesystems=home\n");
        assert!(!overrides.join("org.example.New").exists(), "an app that had no permissions has none again");
        assert_eq!(std::fs::read_to_string(place.join("note")).unwrap(), "before a change");

        std::fs::remove_file(&conf).unwrap();
        let bare = keep(&snaps, "2", &conf, &overrides, &[], "", &|_| Ok(())).unwrap();
        std::fs::write(&conf, "lock_after=1\n").unwrap();
        put_back(&bare, &conf, &overrides).unwrap();
        assert!(!conf.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_half_made_snapshot_is_never_taken_as_whole() {
        let dir = std::env::temp_dir().join(format!("trindle-assistant-half-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (conf, snaps) = (dir.join("trindle.conf"), dir.join("snaps"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&conf, "dim_after=5\n").unwrap();
        let kept = keep(&snaps, "1", &conf, &dir.join("none"), &[], "", &|_| Ok(())).unwrap();

        let bad = dir.join("not-a-folder");
        std::fs::write(&bad, "").unwrap();
        assert!(keep(&snaps, "3", &conf, &bad, &[], "", &|_| Ok(())).is_err());
        assert!(!snaps.join("3").exists() && !snaps.join("3.part").exists(), "nothing is left behind");

        std::fs::create_dir_all(snaps.join("2/overrides")).unwrap();
        assert_eq!(snapshots(&snaps), vec![kept.clone()], "undo with no name never picks a half-made folder");
        assert!(pick(&snaps, "2".into()).is_err());
        tidy(&snaps, std::time::Duration::ZERO);
        assert!(!snaps.join("2").exists() && kept.join(WHOLE).is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn security_and_privacy_settings_are_never_settable() {
        for key in ["security_updates", "downloads_boxed", "outside_network", "outside_files", "check_updates_by_itself",
            "report_offer", "older_programs", "lock", "lock_after", "lock_on_suspend", "lid_close", "lid_close_battery",
            "restart-parts-keys", "restart-all-keys", "screenshot-keys", "shortcut-list-keys", "report-keys"] {
            assert!(check_setting(key, "no").is_err(), "{key}");
        }
    }

    #[test]
    fn only_known_settings_with_plain_values() {
        assert!(check_setting("dim_after", "15").is_ok());
        assert!(check_setting(pointer::TAP, "no").is_ok());
        for key in ["theme", "accent", "PATH", "", "../x", "nonsense"] {
            assert!(check_setting(key, "1").is_err(), "{key}");
        }
        for value in ["", "a\nb=c", "x\r", &"y".repeat(201)] {
            assert!(check_setting("dim_after", value).is_err(), "{value:?}");
        }
        assert!(KEYS.iter().all(|k| !crate::common::files::default_setting(k).is_empty()), "each has a Settings page");
    }

    #[test]
    fn each_setting_takes_only_values_its_page_can_make() {
        for key in KEYS {
            let default = crate::common::files::default_setting(key);
            assert!(check_setting(key, default).is_ok(), "{key} must take its own default {default:?}");
        }
        for (key, value) in [("dim_after", "soon"), ("dim_after", "-5"), ("dim_after", "99999"), ("suspend_after_battery", "1.5"),
            ("wallpaper_every", "-1"), ("wallpaper_every", "9999999999"), ("wallpaper_fit", "bogus"), ("wallpaper_order", "random"),
            ("wallpaper_color", "red"), ("wallpaper_color", "#12345g"), (pointer::SPEED, "9"), (pointer::SPEED, "NaN"),
            (pointer::SPEED, "inf"), (pointer::SCROLL, "0"), (pointer::TAP, "maybe")] {
            assert!(check_setting(key, value).is_err(), "{key} = {value}");
        }
        for (key, value) in [("dim_after", "0"), ("wallpaper_fit", "tile"), (pointer::SPEED, "-0.5"), (pointer::SCROLL, "6")] {
            assert!(check_setting(key, value).is_ok(), "{key} = {value}");
        }
    }

    #[test]
    fn a_window_number_too_big_is_refused_not_wrapped_and_a_device_is_named() {
        let mut desk = Desk::default();
        for n in [4_294_967_296u64, 4_294_967_297, u64::MAX] {
            let said = desk.plan("close_window", &json!({ "window": n })).err().unwrap();
            assert!(said.contains("must be a number"), "{n}: {said}");
        }
        let list = "Device 00:11:22:33:44:55 Blue Speaker\nDevice AA:BB:CC:DD:EE:FF \n";
        assert_eq!(name_in(list, "00:11:22:33:44:55").as_deref(), Some("Blue Speaker"));
        assert_eq!(name_in(list, "AA:BB:CC:DD:EE:FF"), None);
        assert_eq!(name_in(list, "00:00:00:00:00:00"), None);
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

    fn app_home(tag: &str) -> (PathBuf, PathBuf) {
        let home = std::env::temp_dir().join(format!("trindle-assistant-mend-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let app = home.join(".var/app/org.example.App");
        std::fs::create_dir_all(app.join("config")).unwrap();
        std::fs::create_dir_all(app.join("cache")).unwrap();
        std::fs::write(app.join("config/app.conf"), "[General]\nzoom=2\n").unwrap();
        std::fs::write(app.join("cache/thumb"), "x").unwrap();
        (home, app)
    }

    fn mended(home: &Path, action: Action) -> (PathBuf, String) {
        let plan = Plan { question: "a repair".into(), action };
        let extra = |part: &Path| plan.keeps(home, part);
        let place = keep(&home.join("snaps"), "1", &home.join("trindle.conf"), &home.join("overrides"), &[], "a repair", &extra).unwrap();
        let said = mend_now(plan.action, home, &place).unwrap();
        (place, said)
    }

    #[test]
    fn an_apps_setting_is_changed_after_a_snapshot_and_undone() {
        let (home, app) = app_home("setting");
        let line = AppLine { id: "org.example.App".into(), file: "app.conf".into(), section: "General".into(), key: "zoom".into(), value: "3".into() };
        let (place, said) = mended(&home, Action::AppSetting(line));
        assert_eq!(std::fs::read_to_string(app.join("config/app.conf")).unwrap(), "[General]\nzoom=3\n", "{said}");
        assert_eq!(std::fs::read_to_string(place.join("app/settings")).unwrap(), "[General]\nzoom=2\n");
        std::fs::write(app.join("config/app.conf"), "[General]\nzoom=7\nnew=1\n").unwrap();
        let back = put_app_back(&home, &place, |_| Ok(())).unwrap();
        assert!(back.contains("back as it was"), "{back}");
        assert_eq!(std::fs::read_to_string(app.join("config/app.conf")).unwrap(), "[General]\nzoom=2\n");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn an_apps_cache_is_cleared_after_a_snapshot_and_undone() {
        let (home, app) = app_home("cache");
        let (place, said) = mended(&home, Action::ClearCache("org.example.App".into()));
        assert!(!app.join("cache/thumb").exists() && app.join("cache").is_dir(), "{said}");
        assert!(place.join("app/cache/thumb").is_file());
        std::fs::write(app.join("cache/since"), "y").unwrap();
        let back = put_app_back(&home, &place, |_| Ok(())).unwrap();
        assert!(back.contains("cache"), "{back}");
        assert!(app.join("cache/thumb").is_file() && !app.join("cache/since").exists());

        let (place, _) = { let _ = std::fs::remove_dir_all(home.join("snaps")); mended(&home, Action::ClearCache("org.example.App".into())) };
        assert!(put_app_back(&home, &place, |_| Err("still running".into())).is_err());
        assert!(place.join("app/cache/thumb").is_file());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_snapshot_whose_app_is_not_an_app_or_is_trindle_puts_nothing_back() {
        let (home, _) = app_home("forged");
        for id in ["../../..", "org.trindle.Store", "--system"] {
            let place = home.join(format!("forged-{}", id.len()));
            std::fs::create_dir_all(place.join("app/cache")).unwrap();
            std::fs::write(place.join("app/id"), id).unwrap();
            assert!(put_app_back(&home, &place, |_| Ok(())).is_err(), "{id}");
        }

        assert_eq!(put_app_back(&home, &home.join("none"), |_| panic!("nothing to close")).unwrap(), "");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn repairs_refuse_trindle_and_anything_not_a_flathub_app_before_asking() {
        let mut d = Desk::default();
        for tool in ["restart_app", "clear_app_cache", "set_app_setting"] {
            for app in ["org.trindle.Settings", "io.github.trindle_os.Voice"] {
                let said = d.plan(tool, &json!({ "app": app, "file": "a.conf", "key": "k", "value": "v" })).err().unwrap();
                assert_eq!(said, TRINDLE, "{tool} {app}");
            }
            for app in ["--system", "../x", "org.example"] {
                assert!(d.plan(tool, &json!({ "app": app, "file": "a.conf", "key": "k", "value": "v" })).is_err(), "{tool} {app}");
            }
        }
        assert_eq!(d.read("list_app_settings", &json!({ "app": "org.trindle.Store" })).err().unwrap(), TRINDLE);
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
        assert_eq!(p.question, "use the light theme with #ffc107 as the accent colour");
    }
}
