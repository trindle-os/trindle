//! The tray row in the ring's centre.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use zbus::zvariant::OwnedObjectPath;

use crate::common::bt_audio;
use crate::common::files::setting;
use crate::common::run::{capture, find, spawn};
use crate::toolkit::bus::{get_bool, get_f64, get_path, get_paths, get_str, get_u32, Bus, Listen, Props};
use crate::toolkit::event::Flow;
use crate::toolkit::widget::{button, Node};

use super::ctx::{msg, Ctx, Msg};
use crate::toolkit::t;

const VOLUME_EVERY: Duration = Duration::from_secs(2);
const NETWORK_EVERY: Duration = Duration::from_secs(15);

const UPOWER: &str = "org.freedesktop.UPower";
const BATTERY: &str = "/org/freedesktop/UPower/devices/DisplayDevice";
const NM: &str = "org.freedesktop.NetworkManager";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";
const BLUEZ: &str = "org.bluez";
const OBJECT_MANAGER: &str = "org.freedesktop.DBus.ObjectManager";
const PROPS: &str = "org.freedesktop.DBus.Properties";

#[derive(Clone, PartialEq)]
struct Shown {
    icon: String,
    text: Option<String>,
    tip: String,
}

fn shown(icon: &str, tip: &str) -> Option<Shown> {
    Some(Shown { icon: icon.to_string(), text: None, tip: tip.to_string() })
}

type Slot = Rc<RefCell<Option<Shown>>>;

fn put(ctx: &Weak<Ctx>, slot: &Slot, v: Option<Shown>) {
    if *slot.borrow() == v {
        return;
    }
    *slot.borrow_mut() = v;
    if let Some(ctx) = ctx.upgrade() {
        ctx.face(|_| {});
    }
}

pub struct TrayRow {
    ctx: Rc<Ctx>,
    volume: Slot,
    bluetooth: Slot,
    network: Slot,
    battery: Slot,
    active: Rc<Cell<bool>>,
    busy: Rc<Cell<bool>>,
    keyboard: bool,
    brightness: bool,
    cup: String,
    _listens: Vec<Listen>,
}

impl TrayRow {
    pub fn new(ctx: &Rc<Ctx>) -> TrayRow {
        let slot = || Rc::new(RefCell::new(None));
        let mut me = TrayRow {
            ctx: ctx.clone(),
            volume: slot(),
            bluetooth: slot(),
            network: slot(),
            battery: slot(),
            active: Rc::new(Cell::new(false)),
            busy: Rc::new(Cell::new(false)),
            keyboard: keyboard_button_on() && find(KEYBOARD).is_some(),
            brightness: crate::session::brightness::backlight().is_some(),
            cup: install_cup_icon().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|| CUP_ICON.to_string()),
            _listens: Vec::new(),
        };

        let weak = Rc::downgrade(ctx);
        let (active, busy, slot) = (me.active.clone(), me.busy.clone(), me.volume.clone());
        ctx.lp.every(VOLUME_EVERY, move || {
            if weak.strong_count() == 0 {
                return Flow::Stop;
            }
            if active.get() {
                poll_volume(&weak, &busy, &slot);
            }
            Flow::Continue
        });
        poll_volume(&Rc::downgrade(ctx), &me.busy, &me.volume);

        if let Some(bus) = ctx.system.clone() {
            me.bluetooth_watch(&bus);
            me.network_watch(&bus);
            me.battery_watch(&bus);
        }
        me
    }

    pub fn set_open(&mut self, open: bool) {
        self.active.set(open);
        if open {
            poll_volume(&Rc::downgrade(&self.ctx), &self.busy, &self.volume);
        }
    }

    fn battery_watch(&mut self, bus: &Bus) {
        let refresh = {
            let (weak, bus, slot) = (Rc::downgrade(&self.ctx), bus.clone(), self.battery.clone());
            Rc::new(move || battery(&weak, &bus, &slot))
        };
        let r = refresh.clone();
        if let Ok(l) = bus.listen(Some(UPOWER), PROPS, Some("PropertiesChanged"), Some(BATTERY), move |_| r()) {
            self._listens.push(l);
        }
        refresh();
    }

    fn network_watch(&mut self, bus: &Bus) {
        let refresh: Rc<dyn Fn()> = {
            let (weak, bus, slot) = (Rc::downgrade(&self.ctx), bus.clone(), self.network.clone());
            Rc::new(move || network(&weak, &bus, &slot))
        };
        let r = refresh.clone();
        if let Ok(l) = bus.listen(Some(NM), PROPS, Some("PropertiesChanged"), None, move |_| r()) {
            self._listens.push(l);
        }
        let r = refresh.clone();
        let (weak, slot) = (Rc::downgrade(&self.ctx), self.network.clone());
        if let Ok(l) = bus.watch(NM, &self.ctx.lp, move |_| r(), move || put(&weak, &slot, None)) {
            self._listens.push(l);
        }
        let weak = Rc::downgrade(&self.ctx);
        self.ctx.lp.every(NETWORK_EVERY, move || {
            if weak.strong_count() == 0 {
                return Flow::Stop;
            }
            refresh();
            Flow::Continue
        });
    }

    fn bluetooth_watch(&mut self, bus: &Bus) {
        let refresh: Rc<dyn Fn()> = {
            let (weak, bus, slot) = (Rc::downgrade(&self.ctx), bus.clone(), self.bluetooth.clone());
            Rc::new(move || bluetooth(&weak, &bus, &slot))
        };
        for member in ["InterfacesAdded", "InterfacesRemoved"] {
            let r = refresh.clone();
            if let Ok(l) = bus.listen(Some(BLUEZ), OBJECT_MANAGER, Some(member), None, move |_| r()) {
                self._listens.push(l);
            }
        }
        let r = refresh.clone();
        if let Ok(l) = bus.listen(Some(BLUEZ), PROPS, Some("PropertiesChanged"), None, move |_| r()) {
            self._listens.push(l);
        }
        self.sound_watch(bus);
        let (weak, slot) = (Rc::downgrade(&self.ctx), self.bluetooth.clone());
        if let Ok(l) = bus.watch(BLUEZ, &self.ctx.lp, move |_| refresh(), move || put(&weak, &slot, None)) {
            self._listens.push(l);
        }
    }

    fn sound_watch(&mut self, bus: &Bus) {
        let watch = Rc::new(RefCell::new(bt_audio::Watch::default()));
        let lp = self.ctx.lp.clone();
        let listen = bus.listen(Some(BLUEZ), PROPS, Some("PropertiesChanged"), None, move |m| {
            let Ok((iface, changed, _)) = m.body().deserialize::<(String, Props, Vec<String>)>() else { return };
            if iface != "org.bluez.Adapter1" || !changed.contains_key("Powered") {
                return;
            }
            let adapter = m.header().path().map(|p| p.to_string()).unwrap_or_default();
            if !watch.borrow_mut().powered(&adapter, get_bool(&changed, "Powered"), Instant::now()) {
                return;
            }
            let watch = watch.clone();
            lp.after(bt_audio::SETTLE, move || {
                if !watch.borrow_mut().due(Instant::now()) {
                    return;
                }
                std::thread::spawn(|| bt_audio::keep_note(&bt_audio::revive(&bt_audio::UserSystemd)));
            });
        });
        if let Ok(l) = listen {
            self._listens.push(l);
        }
    }

    pub fn battery(&self) -> Option<(String, String, Option<String>)> {
        self.battery.borrow().as_ref().map(|s| (s.icon.clone(), s.tip.clone(), s.text.clone()))
    }

    pub fn buttons(&self, size: f32) -> Vec<Node<Msg>> {
        let mut items: Vec<Node<Msg>> = Vec::new();
        let mut add = |slot: &Slot, panel: &'static str| {
            if let Some(s) = slot.borrow().as_ref() {
                items.push(button(s.icon.clone(), s.tip.clone()).size(size).on_press(msg(move |f| f.open_quick(panel))));
            }
        };
        add(&self.volume, "audio");
        add(&self.bluetooth, "bluetooth");
        add(&self.network, "network");
        if self.brightness {
            items.push(
                button("display-brightness-symbolic", t!("ring-tray-brightness")).size(size).on_press(msg(|f| f.open_quick("brightness"))),
            );
        }
        items.insert(self.volume.borrow().is_some() as usize, super::mic::switch(size));
        items.push(self.keep_awake(size));
        items.into_iter().map(|n| n.tinted(super::Job::Computer.share())).collect()
    }

    fn keep_awake(&self, size: f32) -> Node<Msg> {
        button(self.cup.clone(), t!("ring-tray-keep-awake"))
            .size(size)
            .on(crate::session::idle::keep_awake())
            .on_press(msg(|_| crate::session::idle::set_keep_awake(!crate::session::idle::keep_awake())))
    }

    pub fn keyboard_button(&self, size: f32) -> Option<Node<Msg>> {
        self.keyboard.then(|| {
            button("input-keyboard-symbolic", t!("ring-tray-keyboard")).size(size).tinted(super::Job::Tools.share()).on_press(msg(|_| keyboard()))
        })
    }
}

fn poll_volume(ctx: &Weak<Ctx>, busy: &Rc<Cell<bool>>, slot: &Slot) {
    if busy.replace(true) {
        return;
    }
    let (ctx, busy, slot) = (ctx.clone(), busy.clone(), slot.clone());
    capture(&["wpctl", "get-volume", "@DEFAULT_AUDIO_SINK@"], None, move |ok, out, _| {
        busy.set(false);
        put(&ctx, &slot, volume_shown(ok, &out));
    });
}

fn volume_shown(ok: bool, out: &str) -> Option<Shown> {
    let level = out.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok());
    let (true, Some(level)) = (ok, level) else { return None };
    let pct = (level * 100.0).round() as i32;
    let muted = out.contains("MUTED");
    let icon = if muted || pct == 0 {
        "audio-volume-muted-symbolic"
    } else if pct < 34 {
        "audio-volume-low-symbolic"
    } else if pct < 67 {
        "audio-volume-medium-symbolic"
    } else {
        "audio-volume-high-symbolic"
    };
    let tip = if muted { t!("ring-tray-muted").to_string() } else { t!("ring-tray-volume", pct = pct) };
    shown(icon, &tip)
}

fn battery(ctx: &Weak<Ctx>, bus: &Bus, slot: &Slot) {
    let Some(c) = ctx.upgrade() else { return };
    let (ctx, slot) = (ctx.clone(), slot.clone());
    bus.call(UPOWER, BATTERY, "org.freedesktop.UPower.Device").get_all(&c.lp, move |r| {
        let Ok(p) = r else { return put(&ctx, &slot, None) };
        if !get_bool(&p, "IsPresent") {
            return put(&ctx, &slot, None);
        }
        let pct = get_f64(&p, "Percentage").round() as i32;
        let state = match crate::common::charge::true_state(get_u32(&p, "State")) {
            1 => "charging",
            2 => "onbattery",
            4 => "full",
            5 => "pluggedin",
            _ => "other",
        };
        let icon = get_str(&p, "IconName");
        let icon = if icon.is_empty() { "battery-symbolic".to_string() } else { icon };
        put(&ctx, &slot, Some(Shown { icon, text: Some(format!("{pct}%")), tip: t!("ring-tray-battery", pct = pct, state = state) }));
    });
}

fn network(ctx: &Weak<Ctx>, bus: &Bus, slot: &Slot) {
    let Some(c) = ctx.upgrade() else { return };
    let (ctx, bus2, slot) = (ctx.clone(), bus.clone(), slot.clone());
    bus.call(NM, NM_PATH, NM).get_all(&c.lp, move |r| {
        let Ok(p) = r else { return put(&ctx, &slot, None) };
        let set = |icon: &str, tip: &str| put(&ctx, &slot, shown(icon, tip));
        if p.contains_key("NetworkingEnabled") && !get_bool(&p, "NetworkingEnabled") {
            return set("network-offline-symbolic", t!("ring-tray-net-off"));
        }
        let primary = get_path(&p, "PrimaryConnection");
        let kind = get_str(&p, "PrimaryConnectionType");
        if primary.is_empty() || primary == "/" || get_u32(&p, "State") < 50 {
            return if get_bool(&p, "WirelessHardwareEnabled") {
                set("network-wireless-offline-symbolic", t!("ring-tray-not-connected"))
            } else {
                set("network-wired-disconnected-symbolic", t!("ring-tray-not-connected"))
            };
        }
        let limited = matches!(get_u32(&p, "Connectivity"), 2 | 3);
        match kind.as_str() {
            "802-11-wireless" => wifi(&ctx, &bus2, &primary, limited, &slot),
            "vpn" | "wireguard" => set("network-vpn-symbolic", t!("ring-tray-vpn")),
            _ => set("network-wired-symbolic", if limited { t!("ring-tray-wired-limited") } else { t!("ring-tray-wired") }),
        }
    });
}

fn wifi(ctx: &Weak<Ctx>, bus: &Bus, active: &str, limited: bool, slot: &Slot) {
    let Some(c) = ctx.upgrade() else { return };
    let (ctx, bus2, slot) = (ctx.clone(), bus.clone(), slot.clone());
    bus.call(NM, active, "org.freedesktop.NetworkManager.Connection.Active").get_all(&c.lp, move |r| {
        let p: Props = r.unwrap_or_default();
        let name = get_str(&p, "Id");
        let devices = get_paths(&p, "Devices");
        let Some(dev) = devices.first() else {
            return put(&ctx, &slot, shown("network-wireless-symbolic", &name));
        };
        let Some(c) = ctx.upgrade() else { return };
        let bus3 = bus2.clone();
        bus2.call(NM, dev, "org.freedesktop.NetworkManager.Device.Wireless").get_all(&c.lp, move |r| {
            let ap = r.map(|p| get_path(&p, "ActiveAccessPoint")).unwrap_or_default();
            if ap.is_empty() || ap == "/" {
                return put(&ctx, &slot, shown("network-wireless-symbolic", &name));
            }
            let Some(c) = ctx.upgrade() else { return };
            bus3.call(NM, &ap, "org.freedesktop.NetworkManager.AccessPoint").get_all(&c.lp, move |r| {
                let strength =
                    r.ok().and_then(|p| p.get("Strength").and_then(|v| u8::try_from(v).ok())).unwrap_or(0);
                let level = match strength {
                    76.. => "excellent",
                    51..=75 => "good",
                    26..=50 => "ok",
                    _ => "weak",
                };
                let tip = if limited { t!("ring-tray-wifi-limited", name = name, strength = strength) } else { t!("ring-tray-wifi", name = name, strength = strength) };
                put(&ctx, &slot, shown(&format!("network-wireless-signal-{level}-symbolic"), &tip));
            });
        });
    });
}

fn bluetooth(ctx: &Weak<Ctx>, bus: &Bus, slot: &Slot) {
    let Some(c) = ctx.upgrade() else { return };
    let (ctx, slot) = (ctx.clone(), slot.clone());
    bus.call(BLUEZ, "/", OBJECT_MANAGER).method("GetManagedObjects", &(), false, None, &c.lp, move |r| {
        type Objects = HashMap<OwnedObjectPath, HashMap<String, Props>>;
        let Some((objects,)) = r.ok().and_then(|m| m.body().deserialize::<(Objects,)>().ok()) else {
            return put(&ctx, &slot, None);
        };
        let mut adapters_on = None;
        let mut connected = Vec::new();
        for ifaces in objects.values() {
            for (name, props) in ifaces {
                if name == "org.bluez.Adapter1" {
                    adapters_on = Some(adapters_on.unwrap_or(false) || get_bool(props, "Powered"));
                } else if name == "org.bluez.Device1" && get_bool(props, "Connected") {
                    connected.push(get_str(props, "Alias"));
                }
            }
        }
        let v = match adapters_on {
            None => None,
            Some(false) => shown("bluetooth-disabled-symbolic", t!("ring-tray-bt-off")),
            Some(true) if !connected.is_empty() => {
                shown("bluetooth-active-symbolic", &t!("ring-tray-bt-connected", names = connected.join(", ")))
            }
            Some(true) => shown("bluetooth-active-symbolic", t!("ring-tray-bt-on")),
        };
        put(&ctx, &slot, v);
    });
}

const CUP: &str = include_str!("../../share/icons/trindle-keep-awake-symbolic.svg");
pub const CUP_ICON: &str = "trindle-keep-awake-symbolic";

pub fn install_cup_icon() -> Option<PathBuf> {
    let runtime = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from)?;
    let root = runtime.join("trindle-icons");
    let dir = root.join("hicolor/scalable/status");
    let file = dir.join(format!("{CUP_ICON}.svg"));
    if std::fs::read_to_string(&file).ok().as_deref() != Some(CUP) {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(&file, CUP);
        let _ = std::fs::write(root.join("hicolor/index.theme"),
                               "[Icon Theme]\nName=Trindle\nDirectories=scalable/status\n\n                                [scalable/status]\nSize=16\nMinSize=8\nMaxSize=512\n                                Type=Scalable\nContext=Status\n");
    }
    file.is_file().then_some(file)
}

pub fn keyboard_button_on() -> bool {
    setting("keyboard_button") != "off"
}

fn running(exe: &std::path::Path) -> Vec<std::os::fd::OwnedFd> {
    use std::os::fd::FromRawFd;
    use std::os::unix::fs::MetadataExt;
    // SAFETY: getuid cannot fail and touches no memory.
    let me = unsafe { libc::getuid() };
    let Ok(dir) = std::fs::read_dir("/proc") else { return Vec::new() };
    dir.flatten()
        .filter_map(|e| e.file_name().to_str().and_then(|n| n.parse::<libc::pid_t>().ok()))
        .filter_map(|pid| {
            // SAFETY: pidfd_open takes two numbers and gives back a descriptor or -1, checked here.
            let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0) };
            // SAFETY: a descriptor just made, owned by nothing else.
            let fd = (fd >= 0).then(|| unsafe { std::os::fd::OwnedFd::from_raw_fd(fd as libc::c_int) })?;
            let mine = std::fs::metadata(format!("/proc/{pid}")).is_ok_and(|m| m.uid() == me);
            let same = |p: std::path::PathBuf| p == exe || p.as_os_str().to_str().and_then(|p| p.strip_suffix(" (deleted)")) == exe.to_str();
            (mine && std::fs::read_link(format!("/proc/{pid}/exe")).is_ok_and(same)).then_some(fd)
        })
        .collect()
}

pub const KEYBOARD: &str = "trindle-keyboard";

pub fn keyboard() {
    use std::os::fd::AsRawFd;
    let Some(exe) = find(KEYBOARD).and_then(|p| p.canonicalize().ok()) else { return };
    let held = running(&exe);
    if held.is_empty() {
        spawn(&[KEYBOARD]);
        return;
    }
    for fd in held {
        // SAFETY: the descriptor is a process checked above; no signal info is given.
        unsafe { libc::syscall(libc::SYS_pidfd_send_signal, fd.as_raw_fd(), libc::SIGRTMIN(), std::ptr::null::<libc::siginfo_t>(), 0) };
    }
}
