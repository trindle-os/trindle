//! Network: Wi-Fi and wired connections, through NetworkManager over D-Bus.

mod connection_sharing;
mod join;
mod refresh;

use std::collections::HashMap;
use std::rc::Rc;

use zbus::zvariant::{ObjectPath, Value};
use zbus::Message;

use crate::common::kept_order::KeptOrder;
use crate::common::sharing::{Problem, Shared};
use crate::settings::ui::{confirm, group, headline, line, loading, missing, on, page, Cause, Kit, Msg, Page};
use crate::toolkit::bus::{Failed, Listen};
use crate::toolkit::t;
use crate::toolkit::tiles::Sketch;
use crate::toolkit::widget::{button, drawing, entry, layers, note, row, switch, Length, Node, Tone};
use join::{disconnect, joining, Password};

const NM: &str = "org.freedesktop.NetworkManager";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";
const DEVICE: &str = "org.freedesktop.NetworkManager.Device";
const CONNECTION: &str = "org.freedesktop.NetworkManager.Settings.Connection";
const PROPERTIES: &str = "org.freedesktop.DBus.Properties";

const STATE_ACTIVATED: u32 = 100;
const STATE_DISCONNECTED: u32 = 30;

const WIFI: &str = "network:wifi";

fn refused_words(e: &Failed) -> String {
    let said = format!("{} {}", e.name, e.text).to_lowercase();
    if said.contains("psk") || said.contains("802-11-wireless-security") {
        t!("settings-network-bad-password").to_string()
    } else {
        e.words(t!("settings-network-join-refused"))
    }
}

pub(crate) fn banded(rows: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    rows.into_iter().enumerate().map(|(i, r)| {
        let padded = row(vec![r]).padding(BAND_PAD);
        if i % 2 == 0 {
            return padded;
        }
        let band = drawing((1.0, 1.0), "", |canvas, pen| {
            let (w, h) = (canvas.width() as f32, canvas.height() as f32);
            let radius = crate::toolkit::theme::sheet().corner_of(crate::toolkit::theme::Part::Row);
            let mut s = Sketch::fit(canvas, pen, (w / pen.scale, h / pen.scale));
            let c = s.theme().band;
            s.rect((0.0, 0.0, w / pen.scale, h / pen.scale), radius, c);
        }).width(Length::Fill).height(Length::Fill);
        layers(vec![band, padded]).height(Length::Shrink)
    }).collect()
}

const BAND_PAD: f32 = 8.0;

fn device_state(state: u32) -> &'static str {
    match state {
        10 => t!("settings-network-state-unmanaged"),
        20 => t!("settings-network-state-unavailable"),
        30 => t!("settings-network-state-disconnected"),
        40 => t!("settings-network-state-preparing"),
        50 => t!("settings-network-state-configuring"),
        60 => t!("settings-network-state-needs-password"),
        70 => t!("settings-network-state-address"),
        80 => t!("settings-network-state-checking"),
        90 => t!("settings-network-state-waiting"),
        100 => t!("settings-network-state-connected"),
        110 => t!("settings-network-state-disconnecting"),
        120 => t!("settings-network-state-failed"),
        _ => "",
    }
}

fn op(path: &str) -> ObjectPath<'_> {
    ObjectPath::try_from(path).unwrap_or_else(|_| ObjectPath::try_from("/").expect("the root path"))
}

fn signal_level(strength: u32) -> &'static str {
    match strength {
        s if s > 75 => "strong",
        s if s > 50 => "good",
        s if s > 25 => "fair",
        _ => "weak",
    }
}

fn headline_words(wifi_on: bool, link: Option<&(String, u32)>, wired: bool) -> (String, Tone) {
    match (link, wired) {
        (Some((name, s)), true) => (t!("settings-network-top-both", name = name), if *s > 25 { Tone::Plain } else { Tone::Warn }),
        (Some((name, s)), false) => (t!("settings-network-top-wifi", level = signal_level(*s), name = name),
                                     if *s > 25 { Tone::Plain } else { Tone::Warn }),
        (None, true) => (t!("settings-network-top-cable").into(), Tone::Plain),
        (None, false) if !wifi_on => (t!("settings-network-top-wifi-off").into(), Tone::Calm),
        (None, false) => (t!("settings-network-top-none").into(), Tone::Warn),
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
enum State {
    #[default]
    Looking,
    NoBus,
    Gone,
    Ready,
}

#[derive(Clone)]
struct Ap {
    path: String,
    strength: u32,
    secure: bool,
}

#[derive(Clone)]
struct Wired {
    path: String,
    name: String,
    state: u32,
}

#[derive(Default)]
struct Found {
    waiting: usize,
    wired: Vec<Wired>,
    wifi: bool,

    device_state: u32,
    active_ap: String,
    bitrate: u32,
    saved: HashMap<String, String>,
    saved_waiting: usize,
    aps: HashMap<String, Ap>,
    aps_waiting: usize,

    listed: bool,
}

#[derive(Clone, PartialEq)]
enum Asking {
    Hidden,
    Password(String),
}

#[derive(Default)]
pub struct Network {
    state: State,
    hearing: Vec<Listen>,

    wait: u64,

    round: u64,
    next: Found,
    shown: Found,

    ready: bool,
    enabled: bool,
    problem: String,
    wifi_device: Option<String>,
    no_driver: bool,
    asking: Option<Asking>,
    name_typed: String,
    pass_typed: String,

    sharing: bool,

    shared: Option<Result<Vec<Shared>, Problem>>,

    ap_names: Vec<String>,
    order: KeptOrder,
}

impl Page for Network {
    fn id(&self) -> &'static str {
        "network"
    }
    fn title(&self) -> &'static str {
        t!("settings-network-title")
    }
    fn icon(&self) -> &'static str {
        "network-wireless-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-network-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-network-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.clone() else {
            self.state = State::NoBus;
            return;
        };
        if self.hearing.is_empty() {
            let tell: Rc<dyn Fn(&str)> = Rc::new(kit.heard::<Network>(|me, k, what| match what {
                "up" => me.refresh(k),
                "gone" => me.gone(),
                _ => me.soon(k),
            }));
            let (up, down, changed) = (tell.clone(), tell.clone(), tell);
            if let Ok(l) = bus.watch(NM, &kit.events, move |_| up("up"), move || down("gone")) {
                self.hearing.push(l);
            }
            if let Ok(l) = bus.listen(Some(NM), PROPERTIES, Some("PropertiesChanged"), None, move |_| changed("changed")) {
                self.hearing.push(l);
            }
        }
        self.sharing = crate::common::firewall::sharing();
        self.read_shared(kit);
        self.refresh(kit);
        self.scan(kit);
    }
    fn hidden(&mut self, _kit: &mut Kit) {
        self.hearing.clear();

        self.order.forget();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = Vec::new();
        match self.state {
            State::NoBus => {
                items.push(headline(kit, t!("settings-network-top-looking")));
                items.push(missing(kit, t!("settings-network-the-bus"), "dbus"));
                crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Wifi);
                return page(kit, self.title(), self.help(), items);
            }
            State::Gone => {
                items.push(headline(kit, t!("settings-network-top-gone")));
                items.push(missing(kit, "NetworkManager", "network-manager"));

                items.push(self.sharing_card(kit));
                crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Wifi);
                return page(kit, self.title(), self.help(), items);
            }
            _ if !self.ready => {
                items.push(headline(kit, t!("settings-network-top-looking")));
                items.push(loading());
                crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Wifi);
                return page(kit, self.title(), self.help(), items);
            }
            _ => {
                let (words, _) = headline_words(self.enabled, self.link().as_ref(), self.cabled());
                items.push(headline(kit, words));
            }
        }

        items.push(self.wifi_card(kit));
        if self.shown.wifi || self.shown.listed {
            items.push(self.networks(kit));
        }
        items.push(self.sharing_card(kit));
        crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Wifi);
        page(kit, self.title(), self.help(), items)
    }
}

impl Network {

    fn link(&self) -> Option<(String, u32)> {
        let f = &self.shown;
        f.aps.iter().find(|(_, ap)| ap.path == f.active_ap).map(|(name, ap)| (name.clone(), ap.strength))
    }

    fn cabled(&self) -> bool {
        self.shown.wired.iter().any(|w| w.state == STATE_ACTIVATED)
    }

    fn wifi_card(&self, kit: &Kit) -> Node<Msg> {
        let link = self.link();
        let sub = match &link {
            Some((name, s)) => {
                t!("settings-network-signal-from", level = signal_level(*s), name = name)
            }
            None if !self.shown.wifi && self.no_driver => t!("settings-network-chip").into(),
            None if !self.shown.wifi => t!("settings-network-no-wifi").into(),
            None if !self.enabled => t!("settings-network-wifi-off").into(),
            None => t!("settings-network-wifi-idle").into(),
        };
        let mut wifi = line(t!("settings-network-use-wifi")).sub(sub).key(WIFI);

        if self.shown.wifi {
            wifi = wifi.control(switch(self.enabled, t!("settings-network-use-wifi"), |v| on::<Network>(move |me, k| me.switch_wifi(k, v))));
        }
        if !self.problem.is_empty() {
            wifi = wifi.below(note(self.problem.clone()).tone(Tone::Problem));
        }
        let mut rows = vec![wifi.node(kit)];
        if self.wifi_device.is_some() {
            rows.push(self.hidden_line(kit));
        }

        rows.extend(self.shown.wired.iter().map(|w| self.wired_line(kit, w)));
        group(if self.shown.wired.is_empty() { t!("settings-network-wifi-card") } else { t!("settings-network-wifi-cable-card") })
            .icon("network-wireless-symbolic")
            .about(t!("settings-network-wifi-about"))
            .rows(rows)
            .node(kit)
    }

    fn switch_wifi(&mut self, kit: &mut Kit, on_now: bool) {
        let Some(bus) = kit.system_for(WIFI) else { return };
        self.enabled = on_now;
        let could_not = if on_now { t!("settings-network-on-failed") } else { t!("settings-network-off-failed") };

        let back = kit.settles_quietly::<Network, (), Failed>(WIFI, could_not, move |me, _, done| {
            if done.is_none() {
                me.enabled = !on_now;
            }
        });
        bus.call(NM, NM_PATH, NM).set("WirelessEnabled", Value::from(on_now), false, &kit.events, back);
    }

    fn gone(&mut self) {
        self.state = State::Gone;
        self.round += 1;
        self.shown.wired.clear();
        self.shown.listed = false;
        self.wifi_device = None;
    }

    fn wired_line(&self, kit: &Kit, w: &Wired) -> Node<Msg> {
        let key = format!("network:wired:{}", w.path);
        let mut l = line(if w.name.is_empty() { t!("settings-network-ethernet") } else { &w.name }).icon("network-wired-symbolic")
            .sub(device_state(w.state)).key(key.clone());
        let p = w.path.clone();
        if w.state == STATE_ACTIVATED {
            l = l.control(button("network-offline-symbolic", t!("settings-network-disconnect")).on_press(on::<Network>(move |_, k| {
                disconnect(k, &p, &key);
            })));
        } else if w.state == STATE_DISCONNECTED {
            l = l.control(button("network-transmit-receive-symbolic", t!("settings-network-connect")).on_press(on::<Network>(move |_, k| {
                let Some(bus) = k.system_for(&key) else { return };
                let key = key.clone();
                k.say(&key, t!("settings-network-connecting"), true, true);
                let back = joining(k, key, p.clone(), false, Password::None);
                bus.call(NM, NM_PATH, NM)
                    .method("ActivateConnection", &(op("/"), op(&p), op("/")), false, None, &k.events, back);
            })));
        }
        l.node(kit)
    }

    fn networks(&self, kit: &Kit) -> Node<Msg> {
        let f = &self.shown;
        let lines: Vec<Node<Msg>> = self.ap_names.iter().filter_map(|name| Some(self.ap_line(kit, name, f.aps.get(name)?))).collect();
        let rows = if lines.is_empty() {
            vec![line(t!("settings-network-none-in-range")).sub(t!("settings-network-none-in-range-sub")).node(kit)]
        } else {
            banded(lines)
        };
        group(t!("settings-network-in-range")).icon("network-wireless-signal-excellent-symbolic")
            .about(t!("settings-network-in-range-about"))
            .span(5, 8)
            .waits_for((!self.enabled).then_some(t!("settings-network-switch-on")))
            .rows(rows)
            .node(kit)
    }

    fn ap_line(&self, kit: &Kit, name: &str, ap: &Ap) -> Node<Msg> {
        let f = &self.shown;
        let active = ap.path == f.active_ap;
        let level = match ap.strength {
            s if s > 75 => "excellent",
            s if s > 50 => "good",
            s if s > 25 => "ok",
            _ => "weak",
        };
        let saved = f.saved.get(name).cloned();
        let key = ap_key(name);
        let mut l = line(name).sub(self.ap_words(ap, active, saved.is_some())).icon(format!("network-wireless-signal-{level}-symbolic")).key(key.clone());
        if active {
            let (d, k2) = (self.wifi_device.clone().unwrap_or_default(), key.clone());
            l = l.control(button("network-offline-symbolic", t!("settings-network-disconnect")).on_press(on::<Network>(move |_, k| disconnect(k, &d, &k2))));
        } else {
            let (n, ap) = (name.to_string(), ap.clone());
            l = l.control(button("network-transmit-receive-symbolic", t!("settings-network-connect"))
                .on_press(on::<Network>(move |me, k| me.connect(k, &n, &ap))));
        }
        if let Some(conn) = saved {
            l = l.control(forget_button(kit, &key, conn));
        }
        if self.asking.as_ref() == Some(&Asking::Password(name.to_string())) {
            l = l.below(self.password_row(name, ap));
        }
        l.node(kit)
    }

    fn ap_words(&self, ap: &Ap, active: bool, saved: bool) -> String {
        let f = &self.shown;

        let speed = match f.bitrate {
            kb if active && kb > 0 => t!("settings-network-speed", n = kb / 1000),
            _ => String::new(),
        };
        let mut bits: Vec<&str> = Vec::new();
        if active {
            let s = device_state(f.device_state);
            bits.push(if s.is_empty() { t!("settings-network-state-connected") } else { s });
        } else if saved {
            bits.push(t!("settings-network-saved"));
        }
        bits.push(if ap.secure { t!("settings-network-secured") } else { t!("settings-network-open") });
        if !speed.is_empty() {
            bits.push(&speed);
        }
        bits.join(", ")
    }

    fn password_row(&self, name: &str, ap: &Ap) -> Node<Msg> {
        let (n, ap) = (name.to_string(), ap.clone());
        let join = on::<Network>(move |me, k| {
            let word = me.pass_typed.clone();
            me.add(k, &n, &ap, Some(word));
        });
        let pass = entry(self.pass_typed.clone(), t!("settings-network-password"), |t| on::<Network>(move |me, _| me.pass_typed = t.clone()))
            .key("network:password")
            .secret()
            .on_submit(join.clone())
            .width(Length::Fill)
            .first();
        row(vec![
            pass,
            crate::toolkit::widget::word_button(t!("settings-network-join")).on_press(join),
            button("window-close-symbolic", t!("settings-network-cancel")).on_press(on::<Network>(|me, _| me.close_entry())),
        ]).gap(8.0).width(Length::Fill)
    }

}

fn forget_button(kit: &Kit, key: &str, conn: String) -> Node<Msg> {
    let k2 = key.to_string();
    confirm::<Network>(kit, key, "user-trash-symbolic", t!("settings-network-forget"), move |_, k| {
        let Some(bus) = k.system_for(&k2) else { return };

        let back = k.settles::<Network, Message, Failed>(&k2, t!("settings-network-forget-failed"), |me, k, _| me.refresh(k));
        bus.call(NM, &conn, CONNECTION).method("Delete", &(), false, None, &k.events, back);
    })
}

fn ap_key(name: &str) -> String {
    format!("network:ap:{name}")
}

#[cfg(test)]
impl Network {

    pub(crate) fn flipped() -> Vec<Network> {
        let mut off = Network::made_up();
        off.enabled = false;
        off.shown.listed = false;
        off.shown.aps.clear();
        off.shown.active_ap.clear();
        off.arrange();
        vec![Network::made_up(), off]
    }

    pub(crate) fn made_up() -> Network {
        let mut shown = Found { wifi: true, device_state: STATE_ACTIVATED, active_ap: "/ap/1".into(), bitrate: 433_000,
                                listed: true, ..Found::default() };
        for (i, (name, strength, secure)) in [("Home", 82, true), ("Home guests", 78, true), ("Cafe on the corner", 55, false),
                                              ("Number 14", 40, true), ("Printer setup", 22, false)].into_iter().enumerate() {
            shown.aps.insert(name.to_string(), Ap { path: format!("/ap/{}", i + 1), strength, secure });
        }
        shown.saved.insert("Home".into(), "/saved/1".into());
        shown.saved.insert("Number 14".into(), "/saved/2".into());
        shown.wired.push(Wired { path: "/wired/1".into(), name: "Wired".into(), state: STATE_DISCONNECTED });
        let mut me = Network { state: State::Ready, shown, ready: true, enabled: true, wifi_device: Some("/wifi".into()),
                               shared: Some(Ok(vec![Shared { name: "photos".into(), path: "/home/john/Photos".into(),
                                                             write: false, guests: false }])),
                               ..Network::default() };
        me.arrange();
        me
    }

}

#[cfg(test)]
mod tests;
