//! Bluetooth via BlueZ.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Duration;

use zbus::zvariant::{ObjectPath, OwnedObjectPath, Value};
use zbus::Message;

use crate::common::kept_order::KeptOrder;
use crate::settings::panels::network::banded;
use crate::common::bt_audio;
use crate::common::rfkill::{self, Block, Radio, Refused};
use crate::common::run::{admin_helper, capture};
use crate::settings::ui::{asked, confirm, group, headline, help_link, in_words, line, missing, on, page, Cause, Kit, Msg, Page};
use crate::toolkit::bus::{get_bool, get_str, Asked, Failed, Listen, Offer, Props, Served};
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, entry, label, row, switch, Length, Node, Tone};

const BLUEZ: &str = "org.bluez";
const ADAPTER: &str = "org.bluez.Adapter1";
const DEVICE: &str = "org.bluez.Device1";
const OBJECT_MANAGER: &str = "org.freedesktop.DBus.ObjectManager";
const PROPERTIES: &str = "org.freedesktop.DBus.Properties";
const AGENT_MANAGER: &str = "org.bluez.AgentManager1";
const AGENT_IFACE: &str = "org.bluez.Agent1";
const AGENT_PATH: &str = "/org/trindle/bluetooth_agent";
const REJECTED: &str = "org.bluez.Error.Rejected";

const AGENT_XML: &str = r#"<interface name="org.bluez.Agent1">
  <method name="Release"/>
  <method name="RequestPinCode"><arg type="o" direction="in"/><arg type="s" direction="out"/></method>
  <method name="DisplayPinCode"><arg type="o" direction="in"/><arg type="s" direction="in"/></method>
  <method name="RequestPasskey"><arg type="o" direction="in"/><arg type="u" direction="out"/></method>
  <method name="DisplayPasskey"><arg type="o" direction="in"/><arg type="u" direction="in"/><arg type="q" direction="in"/></method>
  <method name="RequestConfirmation"><arg type="o" direction="in"/><arg type="u" direction="in"/></method>
  <method name="RequestAuthorization"><arg type="o" direction="in"/></method>
  <method name="AuthorizeService"><arg type="o" direction="in"/><arg type="s" direction="in"/></method>
  <method name="Cancel"/>
</interface>"#;

const SEARCH: &str = "bluetooth:search";
const POWER: &str = "bluetooth:power";
const HELP_CONNECT: &str = "bluetooth-connect";
const DROP_CHECK: Duration = Duration::from_secs(3);
const VISIBLE: &str = "bluetooth:visible";

const UNBLOCK_SETTLES: Duration = Duration::from_millis(500);

fn not_trusted() -> &'static str {
    t!("settings-bluetooth-not-trusted")
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum First {
    Nothing,
    Unblock,
    Impossible,
}

fn before_on(block: Option<Block>) -> First {
    match block {
        Some(Block::Hard) => First::Impossible,
        Some(Block::Soft) => First::Unblock,
        Some(Block::Clear) | None => First::Nothing,
    }
}

fn power_words(e: &Failed, on: bool, block: Option<Block>) -> String {
    let said = format!("{} {}", e.name, e.text).to_lowercase();
    if on && block == Some(Block::Hard) {
        rfkill::hard_words(Radio::Bluetooth)
    } else if on && (block == Some(Block::Soft) || said.contains("blocked") || said.contains("rfkill")) {
        rfkill::refused_words(Radio::Bluetooth)
    } else {
        e.words(if on { t!("settings-bluetooth-on-failed") } else { t!("settings-bluetooth-off-failed") })
    }
}

fn bluez_words(e: &Failed, could_not: &str) -> String {
    let said = format!("{} {}", e.name, e.text).to_lowercase();
    let has = |any: &[&str]| any.iter().any(|w| said.contains(w));
    if has(&["doesnotexist", "does not exist", "doesn't exist", "unknownobject"]) {
        t!("settings-bluetooth-cause-gone", could_not = could_not)
    } else if has(&["notready"]) {
        t!("settings-bluetooth-cause-not-ready", could_not = could_not)
    } else if has(&["authenticationfailed"]) {
        t!("settings-bluetooth-cause-auth-failed", could_not = could_not)
    } else if has(&["authenticationcanceled", "authenticationrejected"]) {
        t!("settings-bluetooth-cause-turned-down", could_not = could_not)
    } else if has(&["authenticationtimeout"]) {
        t!("settings-bluetooth-cause-timeout", could_not = could_not)
    } else if has(&["inprogress", "in progress"]) {
        t!("settings-bluetooth-cause-in-progress", could_not = could_not)
    } else if has(&["connectionattemptfailed", "page-timeout", "host is down", "connection refused"]) {
        t!("settings-bluetooth-cause-no-answer", could_not = could_not)
    } else if has(&["notavailable", "profile-unavailable"]) {
        t!("settings-bluetooth-cause-not-available", could_not = could_not)
    } else {
        e.words(could_not)
    }
}

#[derive(Debug, PartialEq)]
struct Refusal {
    words: String,
    old_pairing: bool,
}

fn key_refused(e: &Failed) -> bool {
    let said = format!("{} {}", e.name, e.text).to_lowercase();
    ["authenticationfailed", "authenticationrejected", "authenticationcanceled", "key-missing", "key missing", "keymissing"]
        .iter()
        .any(|w| said.contains(w))
}

fn connect_refusal(e: &Failed) -> Refusal {
    let could_not = t!("settings-bluetooth-connect-failed");
    if key_refused(e) {
        Refusal { words: t!("settings-bluetooth-cause-old-pairing", could_not = could_not), old_pairing: true }
    } else {
        Refusal { words: bluez_words(e, could_not), old_pairing: false }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
enum State {
    #[default]
    Looking,
    NoBus,
    Gone,
    NoAdapter,
    Off,
    On,
}

struct Adapter {
    path: String,
    name: String,
    powered: bool,
    discoverable: bool,
    discovering: bool,
}

struct Device {
    path: String,
    name: String,
    named: bool,
    icon: &'static str,
    paired: bool,
    trusted: bool,
    connected: bool,
    battery: Option<u32>,
    rssi: i32,
}

impl Device {
    fn known(&self) -> bool {
        self.paired || self.trusted
    }

    fn listed(&self) -> bool {
        self.paired || self.named
    }
}

enum Ask {
    Confirm(u32, Asked),
    Authorise(Asked),
    Pin { number: bool, asked: Asked },
}

#[derive(Default)]
pub struct Bluetooth {
    state: State,
    adapter: Option<Adapter>,
    devices: Vec<Device>,
    hold: u32,
    wait: u64,
    stale: HashSet<String>,
    asks: HashMap<String, Ask>,
    pins: HashMap<String, String>,
    inbox: Rc<RefCell<Vec<Asked>>>,
    agent: Option<Served>,
    hearing: Vec<Listen>,
    order: KeptOrder,
    block: Option<Block>,
    turn_on: bool,
}

impl Page for Bluetooth {
    fn id(&self) -> &'static str {
        "bluetooth"
    }
    fn title(&self) -> &'static str {
        t!("settings-bluetooth-title")
    }
    fn icon(&self) -> &'static str {
        "bluetooth-active-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-bluetooth-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-bluetooth-help")
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (words, _) = self.sentence();
        let mut items = vec![headline(kit, words)];
        match self.state {
            State::Looking => {}
            State::NoBus => items.push(missing(kit, t!("settings-bluetooth-the-bus"), "dbus")),
            State::Gone => items.push(missing(kit, t!("settings-bluetooth-title"), "bluez")),
            State::NoAdapter if self.block != Some(Block::Soft) => items.push(self.no_adapter_card(kit)),
            State::NoAdapter | State::Off | State::On => {
                items.push(self.adapter_card(kit));
                items.push(self.mine(kit));
                items.push(self.nearby(kit));
            }
        }
        crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Bluetooth);
        page(kit, self.title(), self.help(), items)
    }

    fn shown(&mut self, kit: &mut Kit) {
        if let Some(why) = bt_audio::kept_note() {
            kit.say(POWER, &bt_audio::words(&why), false, false);
        }
        let Some(bus) = kit.system.clone() else {
            self.state = State::NoBus;
            return;
        };
        if !self.hearing.is_empty() {
            return;
        }
        let tell: Rc<dyn Fn(&str)> = Rc::new(kit.heard::<Bluetooth>(|me, k, what| match what {
            "up" => {
                me.ask_bluez(k);
                me.refresh(k);
            }
            "gone" => me.gone(),
            "changed" => me.soon(k),
            _ => me.take_asks(k),
        }));
        let (up, down) = (tell.clone(), tell.clone());
        if let Ok(l) = bus.watch(BLUEZ, &kit.events, move |_| up("up"), move || down("gone")) {
            self.hearing.push(l);
        }
        for (iface, member) in [(OBJECT_MANAGER, "InterfacesAdded"), (OBJECT_MANAGER, "InterfacesRemoved"), (PROPERTIES, "PropertiesChanged")] {
            let t = tell.clone();
            if let Ok(l) = bus.listen(Some(BLUEZ), iface, Some(member), None, move |_| t("changed")) {
                self.hearing.push(l);
            }
        }
        let inbox = self.inbox.clone();
        let offer = Offer {
            name: AGENT_IFACE.into(),
            xml: AGENT_XML.into(),
            call: Box::new(move |asked: Asked| {
                inbox.borrow_mut().push(asked);
                tell("asked");
            }),
            props: Box::new(Props::new),
            set: None,
        };
        if let Ok(served) = bus.serve(AGENT_PATH, offer) {
            self.agent = Some(served);
            self.ask_bluez(kit);
        }
    }

    fn hidden(&mut self, kit: &mut Kit) {
        self.order.forget();
        if self.agent.take().is_some() {

            if let Some(bus) = kit.system.clone() {
                bus.call(BLUEZ, "/org/bluez", AGENT_MANAGER)
                    .method("UnregisterAgent", &(agent_path(),), false, None, &kit.events, |_| {});
            }
        }
        self.hearing.clear();
        self.asks.clear();
        self.pins.clear();
        self.inbox.borrow_mut().clear();
    }
}

impl Bluetooth {
    fn ask_bluez(&self, kit: &Kit) {
        let Some(bus) = kit.system.clone() else { return };
        if self.agent.is_none() {
            return;
        }
        let manager = bus.call(BLUEZ, "/org/bluez", AGENT_MANAGER);
        manager.method("RegisterAgent", &(agent_path(), "DisplayYesNo"), false, None, &kit.events, |_| {});
        manager.method("RequestDefaultAgent", &(agent_path(),), false, None, &kit.events, |_| {});
    }

    fn gone(&mut self) {
        self.state = State::Gone;
        self.adapter = None;
        self.devices.clear();
    }

    fn soon(&mut self, kit: &mut Kit) {
        self.wait += 1;
        let n = self.wait;
        kit.after::<Bluetooth>(Duration::from_millis(400), move |me, k| {
            if me.wait == n {
                me.settle(k);
            }
        });
    }

    fn settle(&mut self, kit: &mut Kit) {
        if self.hold == 0 {
            self.refresh(kit);
            return;
        }
        self.wait += 1;
        let n = self.wait;
        kit.after::<Bluetooth>(Duration::from_millis(1000), move |me, k| {
            if me.wait == n {
                me.settle(k);
            }
        });
    }

    fn refresh(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.clone() else { return };
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(|me, k, r| {
            me.got(r);
            me.block = if me.state == State::NoAdapter { rfkill::state(Radio::Bluetooth) } else { None };
            me.found(k);
        });
        bus.call(BLUEZ, "/", OBJECT_MANAGER).method("GetManagedObjects", &(), false, None, &kit.events, back);
    }

    fn got(&mut self, reply: Result<Message, Failed>) {
        type Objects = HashMap<OwnedObjectPath, HashMap<String, Props>>;
        let Some((objects,)) = reply.ok().and_then(|m| m.body().deserialize::<(Objects,)>().ok()) else {
            self.gone();
            return;
        };
        let mut adapters: Vec<(String, &Props)> =
            objects.iter().filter_map(|(p, o)| o.get(ADAPTER).map(|a| (p.to_string(), a))).collect();
        adapters.sort_by(|a, b| a.0.cmp(&b.0));
        let Some((path, a)) = adapters.into_iter().next() else {
            self.state = State::NoAdapter;
            self.adapter = None;
            self.devices.clear();
            return;
        };
        let adapter = Adapter {
            name: first_of(a, &["Alias", "Name"]),
            powered: get_bool(a, "Powered"),
            discoverable: get_bool(a, "Discoverable"),
            discovering: get_bool(a, "Discovering"),
            path,
        };
        self.devices = objects
            .iter()
            .filter_map(|(p, o)| o.get(DEVICE).map(|d| (p, d)))
            .filter(|(_, d)| path_of(d, "Adapter") == adapter.path)
            .map(|(p, d)| Device {
                path: p.to_string(),
                name: first_of(d, &["Alias", "Name", "Address"]),
                named: !first_of(d, &["Name", "Alias"]).is_empty(),
                icon: device_icon(&get_str(d, "Icon")),
                paired: get_bool(d, "Paired"),
                trusted: get_bool(d, "Trusted"),
                connected: get_bool(d, "Connected"),
                battery: d.get("Battery").and_then(|v| u8::try_from(v).ok()).map(u32::from),
                rssi: rssi(d),
            })
            .collect();
        self.arrange();
        self.state = if adapter.powered { State::On } else { State::Off };
        self.adapter = Some(adapter);
    }

    fn sentence(&self) -> (String, Tone) {
        match self.state {
            State::Looking | State::NoBus => (t!("settings-bluetooth-top-looking").into(), Tone::Plain),
            State::Gone => (t!("settings-bluetooth-top-gone").into(), Tone::Plain),
            State::NoAdapter => (match self.block {
                Some(Block::Soft) => t!("settings-bluetooth-top-off"),
                Some(Block::Hard) => t!("settings-bluetooth-top-hard"),
                Some(Block::Clear) | None => t!("settings-bluetooth-top-none"),
            }.into(), Tone::Plain),
            State::Off => (t!("settings-bluetooth-top-off").into(), Tone::Plain),
            State::On => {
                let on_now: Vec<String> =
                    self.devices.iter().filter(|d| d.paired && d.connected).map(|d| d.name.clone()).collect();
                if self.adapter.as_ref().is_some_and(|a| a.discovering) {
                    (t!("settings-bluetooth-top-searching").into(), Tone::Calm)
                } else if on_now.is_empty() {
                    (t!("settings-bluetooth-top-idle").into(), Tone::Calm)
                } else {
                    (t!("settings-bluetooth-top-connected", n = on_now.len(), names = in_words(&on_now)), Tone::Plain)
                }
            }
        }
    }

    fn adapter_card(&self, kit: &Kit) -> Node<Msg> {
        let about = t!("settings-bluetooth-card-about");
        let Some(a) = &self.adapter else {
            let power = line(t!("settings-bluetooth-title")).sub(t!("settings-bluetooth-switched-off")).key(POWER)
                .control(switch(self.turn_on, t!("settings-bluetooth-title"), |v| on::<Bluetooth>(move |me, k| {
                    if v {
                        me.unblock(k, None);
                    }
                })));
            let visible = line(t!("settings-bluetooth-visible")).sub(t!("settings-bluetooth-visible-sub")).key(VISIBLE)
                .control(switch(false, t!("settings-bluetooth-visible"), |_| on::<Bluetooth>(|_, _| {})))
                .node(kit).off(true);
            return group(t!("settings-bluetooth-title")).icon("bluetooth-active-symbolic").about(about).rows(vec![power.node(kit), visible]).node(kit);
        };
        let path = a.path.clone();
        let power = line(t!("settings-bluetooth-title")).sub(a.name.clone()).key(POWER).controls(vec![
            help_link(t!("settings-bluetooth-sound-help"), bt_audio::HELP),
            switch(a.powered, t!("settings-bluetooth-title"), move |v| {
                let path = path.clone();
                on::<Bluetooth>(move |me, k| me.set_power(k, &path, v))
            }),
        ]);
        let mut rows = vec![power.node(kit)];
        let path = a.path.clone();
        rows.push(line(t!("settings-bluetooth-visible")).sub(t!("settings-bluetooth-visible-sub")).key(VISIBLE)
            .control(switch(a.powered && a.discoverable, t!("settings-bluetooth-visible"), move |v| {
                let path = path.clone();
                on::<Bluetooth>(move |me, k| me.set_visible(k, &path, v))
            }))
            .node(kit).off(!a.powered));
        group(t!("settings-bluetooth-title")).icon("bluetooth-active-symbolic")
            .about(about)
            .rows(rows)
            .node(kit)
    }

    fn no_adapter_card(&self, kit: &Kit) -> Node<Msg> {
        match self.block {
            Some(Block::Soft) => self.adapter_card(kit),
            Some(Block::Hard) => group(t!("settings-bluetooth-title")).icon("bluetooth-disabled-symbolic")
                .about(rfkill::hard_words(Radio::Bluetooth))
                .rows(vec![line(t!("settings-bluetooth-hard-line")).sub(t!("settings-bluetooth-hard-line-sub")).node(kit)])
                .node(kit),
            Some(Block::Clear) | None => group(t!("settings-bluetooth-no-adapter")).icon("bluetooth-disabled-symbolic")
                .about(t!("settings-bluetooth-no-adapter-about"))
                .rows(vec![line(t!("settings-bluetooth-none-found")).sub(t!("settings-bluetooth-none-found-sub")).node(kit)])
                .node(kit),
        }
    }

    fn set_power(&mut self, kit: &mut Kit, path: &str, v: bool) {
        if v {
            match before_on(rfkill::state(Radio::Bluetooth)) {
                First::Impossible => {
                    kit.say(POWER, &rfkill::hard_words(Radio::Bluetooth), false, false);
                    return;
                }
                First::Unblock => {
                    self.unblock(kit, Some(path.to_string()));
                    return;
                }
                First::Nothing => {}
            }
        }
        self.power(kit, path, v);
    }

    fn unblock(&mut self, kit: &mut Kit, path: Option<String>) {
        match rfkill::unblock(Radio::Bluetooth) {
            Ok(()) => self.unblocked(kit, path),
            Err(Refused::NotAllowed) => {
                kit.say(POWER, t!("settings-bluetooth-waiting"), true, true);
                let back = kit.then::<Bluetooth, Result<(), String>>(move |me, k, done| match done {
                    Ok(()) => {
                        k.hush(POWER);
                        me.unblocked(k, path);
                    }
                    Err(e) => k.say(POWER, &e, false, false),
                });
                let helper = admin_helper();
                capture(&["pkexec", &helper, "radio-unblock", Radio::Bluetooth.word()], None, move |ok, _, err| back(asked(ok, &err)));
            }
            Err(Refused::Other(_)) => kit.say(POWER, &rfkill::refused_words(Radio::Bluetooth), false, false),
        }
    }

    fn unblocked(&mut self, kit: &mut Kit, path: Option<String>) {
        match path {
            Some(path) => {
                if let Some(a) = self.adapter.as_mut() {
                    a.powered = true;
                }
                kit.after::<Bluetooth>(UNBLOCK_SETTLES, move |me, k| me.power(k, &path, true));
            }
            None => {
                self.turn_on = true;
                kit.after::<Bluetooth>(UNBLOCK_SETTLES, |me, k| me.refresh(k));
            }
        }
    }

    fn found(&mut self, kit: &mut Kit) {
        if !self.turn_on {
            return;
        }
        let Some((path, powered)) = self.adapter.as_ref().map(|a| (a.path.clone(), a.powered)) else { return };
        self.turn_on = false;
        if !powered {
            self.power(kit, &path, true);
        }
    }

    fn power(&mut self, kit: &mut Kit, path: &str, v: bool) {
        let Some(bus) = kit.system_for(POWER) else { return };
        if let Some(a) = self.adapter.as_mut() {
            a.powered = v;
        }
        let back = kit.then::<Bluetooth, Result<(), Failed>>(move |me, k, r| {
            let block = if r.is_err() && v { rfkill::state(Radio::Bluetooth) } else { None };
            me.powered(k, v, r, block);
        });
        bus.call(BLUEZ, path, ADAPTER).set("Powered", Value::from(v), false, &kit.events, back);
    }

    fn powered(&mut self, kit: &mut Kit, v: bool, r: Result<(), Failed>, block: Option<Block>) {
        let r = r.map_err(|e| power_words(&e, v, block));
        if kit.settle_quietly(POWER, r, "").is_none() {
            if let Some(a) = self.adapter.as_mut() {
                a.powered = !v;
            }
        }
    }

    fn set_visible(&mut self, kit: &mut Kit, path: &str, v: bool) {
        let Some(bus) = kit.system_for(VISIBLE) else { return };
        if let Some(a) = self.adapter.as_mut() {
            a.discoverable = v;
        }
        let back = kit.then::<Bluetooth, Result<(), Failed>>(move |me, k, r| me.visible_set(k, v, r));
        bus.call(BLUEZ, path, ADAPTER).set("Discoverable", Value::from(v), false, &kit.events, back);
    }

    fn visible_set(&mut self, kit: &mut Kit, v: bool, r: Result<(), Failed>) {
        let could_not = if v { t!("settings-bluetooth-visible-failed") } else { t!("settings-bluetooth-hidden-failed") };
        if kit.settle_quietly(VISIBLE, r.map_err(|e| bluez_words(&e, could_not)), could_not).is_none() {
            if let Some(a) = self.adapter.as_mut() {
                a.discoverable = !v;
            }
        }
    }

    fn arrange(&mut self) {
        self.devices.sort_by(|a, b| (!a.paired, !a.connected).cmp(&(!b.paired, !b.connected)).then_with(|| match a.paired {
            true => a.name.cmp(&b.name),
            false => b.rssi.cmp(&a.rssi),
        }));
        self.order.keep(&mut self.devices, |d| d.path.clone());
    }

    fn off_words(&self) -> Option<&'static str> {
        (self.state != State::On).then_some(t!("settings-bluetooth-switch-on-first"))
    }

    fn mine(&self, kit: &Kit) -> Node<Msg> {
        let paired: Vec<&Device> = self.devices.iter().filter(|d| d.paired).collect();
        let mut rows: Vec<Node<Msg>> = banded(paired.iter().map(|d| self.device_line(kit, d)).collect());
        if rows.is_empty() {
            rows.push(line(t!("settings-bluetooth-none-paired")).sub(t!("settings-bluetooth-none-paired-sub")).node(kit));
        }
        group(t!("settings-bluetooth-mine")).icon("bluetooth-active-symbolic")
            .about(t!("settings-bluetooth-mine-about"))
            .span(5, 8)
            .waits_for(self.off_words())
            .rows(rows)
            .node(kit)
    }

    fn nearby(&self, kit: &Kit) -> Node<Msg> {
        let looking = self.adapter.as_ref().is_some_and(|a| a.discovering);
        let press = on::<Bluetooth>(|me, k| me.toggle_looking(k));
        let search = if looking {
            line(t!("settings-bluetooth-looking"))
                .control(button("media-playback-stop-symbolic", t!("settings-bluetooth-stop-looking")).on_press(press))
        } else {
            line(t!("settings-bluetooth-look")).sub(t!("settings-bluetooth-look-sub"))
                .control(button("edit-find-symbolic", t!("settings-bluetooth-look")).on_press(press))
        }
        .key(SEARCH);
        let near: Vec<&Device> = self.devices.iter().filter(|d| !d.paired && d.named).collect();
        let mut rows = vec![search.node(kit)];
        rows.extend(banded(near.iter().map(|d| self.device_line(kit, d)).collect()));
        group(t!("settings-bluetooth-nearby")).icon("edit-find-symbolic")
            .about(t!("settings-bluetooth-nearby-about"))
            .span(4, 8)
            .waits_for(self.off_words())
            .rows(rows)
            .node(kit)
    }

    fn toggle_looking(&mut self, kit: &mut Kit) {
        let Some((path, looking)) = self.adapter.as_ref().map(|a| (a.path.clone(), a.discovering)) else { return };
        let Some(bus) = kit.system_for(SEARCH) else { return };
        let (method, could_not) = if looking {
            ("StopDiscovery", t!("settings-bluetooth-stop-failed"))
        } else {
            ("StartDiscovery", t!("settings-bluetooth-start-failed"))
        };
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(move |_, k, r| {
            k.settle_quietly(SEARCH, r.map_err(|e| bluez_words(&e, could_not)), could_not);
        });
        bus.call(BLUEZ, &path, ADAPTER).method(method, &(), false, None, &kit.events, back);
    }

    fn device_line(&self, kit: &Kit, d: &Device) -> Node<Msg> {
        let mut bits: Vec<String> = Vec::new();
        if d.connected {
            bits.push(t!("settings-bluetooth-connected").into());
        } else if d.paired {
            bits.push(t!("settings-bluetooth-not-connected").into());
        }
        if let Some(b) = d.battery {
            bits.push(t!("settings-bluetooth-battery", n = b));
        }
        let mut l = line(d.name.clone()).sub(bits.join(", ")).icon(d.icon).key(d.path.clone());
        let p = d.path.clone();
        l = l.control(if !d.paired {
            button("insert-link-symbolic", t!("settings-bluetooth-pair")).on_press(on::<Bluetooth>(move |me, k| me.pair(k, p.clone())))
        } else if d.connected {
            button("network-offline-symbolic", t!("settings-bluetooth-disconnect")).on_press(on::<Bluetooth>(move |_, k| {
                let Some(bus) = k.system_for(&p) else { return };
                let key = p.clone();
                let back = k.then::<Bluetooth, Result<Message, Failed>>(move |_, k, r| {
                    k.settle(&key, r.map_err(|e| bluez_words(&e, t!("settings-bluetooth-disconnect-failed"))), "");
                });
                bus.call(BLUEZ, &p, DEVICE).method("Disconnect", &(), false, None, &k.events, back);
            }))
        } else {
            button("network-transmit-receive-symbolic", t!("settings-bluetooth-connect")).on_press(on::<Bluetooth>(move |me, k| me.connect(k, p.clone(), true)))
        });
        if d.paired && self.stale.contains(&d.path) {
            let p = d.path.clone();
            l = l.control(button("insert-link-symbolic", t!("settings-bluetooth-pair-again")).on_press(on::<Bluetooth>(move |me, k| me.pair_again(k, p.clone()))));
        }
        if d.known() {
            let p = d.path.clone();
            l = l.control(confirm::<Bluetooth>(kit, &d.path, "user-trash-symbolic", t!("settings-bluetooth-forget"), move |me, k| {
                me.forget(k, &p);
            }));
        }
        if kit.said(&d.path).is_some_and(|(_, problem)| problem) {
            l = l.control(help_link(t!("settings-bluetooth-more"), HELP_CONNECT));
        }
        if let Some(ask) = self.asks.get(&d.path) {
            l = l.below(self.ask_node(&d.path, ask));
        }
        l.node(kit)
    }

    fn forget(&mut self, kit: &mut Kit, path: &str) {
        let Some(adapter) = self.adapter.as_ref().map(|a| a.path.clone()) else { return };
        let Some(bus) = kit.system_for(path) else { return };
        let Ok(device) = ObjectPath::try_from(path) else { return };
        let key = path.to_string();
        self.stale.remove(path);
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(move |_, k, r| {
            k.settle(&key, r.map_err(|e| bluez_words(&e, t!("settings-bluetooth-forget-failed"))), "");
        });
        bus.call(BLUEZ, &adapter, ADAPTER).method("RemoveDevice", &(device,), false, None, &kit.events, back);
    }

    fn pair_again(&mut self, kit: &mut Kit, path: String) {
        let Some(adapter) = self.adapter.as_ref().map(|a| a.path.clone()) else { return };
        let Some(bus) = kit.system_for(&path) else { return };
        let Ok(device) = ObjectPath::try_from(path.as_str()) else { return };
        let looking = self.adapter.as_ref().is_some_and(|a| a.discovering);
        self.stale.remove(&path);
        let (again, key, remover) = (bus.clone(), path.clone(), adapter.clone());
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(move |_, k, r| {
            if let Err(e) = r {
                k.say(&key, &bluez_words(&e, t!("settings-bluetooth-forget-failed")), false, false);
                return;
            }
            k.say(SEARCH, t!("settings-bluetooth-forgotten-pair"), true, true);
            if !looking {
                let go = k.then::<Bluetooth, Result<Message, Failed>>(|_, k, r| {
                    k.settle_quietly(SEARCH, r.map_err(|e| bluez_words(&e, t!("settings-bluetooth-start-failed"))), "");
                });
                again.call(BLUEZ, &adapter, ADAPTER).method("StartDiscovery", &(), false, None, &k.events, go);
            }
        });
        bus.call(BLUEZ, &remover, ADAPTER).method("RemoveDevice", &(device,), false, None, &kit.events, back);
    }

    fn pair(&mut self, kit: &mut Kit, path: String) {
        let Some(bus) = kit.system_for(&path) else { return };
        kit.say(&path, t!("settings-bluetooth-pairing"), true, true);
        self.hold += 1;
        let p = path.clone();
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(move |me, k, r| {
            me.hold = me.hold.saturating_sub(1);
            if let Err(e) = r {
                k.say(&p, &bluez_words(&e, t!("settings-bluetooth-pair-failed")), false, false);
                return;
            }
            let Some(bus) = k.system.clone() else {
                me.connect(k, p, false);
                return;
            };
            let device = p.clone();
            let go = k.then::<Bluetooth, Result<(), Failed>>(move |me, k, trusted| me.connect(k, p, trusted.is_ok()));
            bus.call(BLUEZ, &device, DEVICE).set("Trusted", Value::from(true), false, &k.events, go);
        });
        bus.call(BLUEZ, &path, DEVICE).method("Pair", &(), false, Some(Duration::from_secs(60)), &kit.events, back);
    }

    fn connect(&mut self, kit: &mut Kit, path: String, trusted: bool) {
        if !self.devices.iter().any(|d| d.path == path) {
            let words = t!("settings-bluetooth-cause-gone", could_not = t!("settings-bluetooth-connect-failed"));
            kit.say(&path, &words, false, false);
            return;
        }
        let Some(bus) = kit.system_for(&path) else { return };
        kit.say(&path, t!("settings-bluetooth-connecting"), true, true);
        let key = path.clone();
        let back = kit.then::<Bluetooth, Result<Message, Failed>>(move |me, k, r| match r {
            Err(e) => {
                let refusal = connect_refusal(&e);
                if refusal.old_pairing {
                    me.stale.insert(key.clone());
                }
                k.say(&key, &refusal.words, false, false);
            }
            Ok(_) => {
                k.settle(&key, Ok::<_, String>(()), "");
                if !trusted {
                    k.say(&key, not_trusted(), false, false);
                }
                me.stale.remove(&key);
                me.check_held(k, key);
            }
        });
        bus.call(BLUEZ, &path, DEVICE).method("Connect", &(), false, Some(Duration::from_secs(30)), &kit.events, back);
    }

    fn check_held(&mut self, kit: &mut Kit, path: String) {
        kit.after::<Bluetooth>(DROP_CHECK, move |me, k| {
            if me.dropped(&path) {
                me.stale.insert(path.clone());
                k.say(&path, t!("settings-bluetooth-dropped"), false, false);
            }
        });
    }

    fn dropped(&self, path: &str) -> bool {
        self.state == State::On && self.devices.iter().any(|d| d.path == path && !d.connected)
    }

    fn take_asks(&mut self, kit: &mut Kit) {
        let waiting: Vec<Asked> = self.inbox.borrow_mut().drain(..).collect();
        for asked in waiting {
            self.agent_call(kit, asked);
        }
    }

    fn agent_call(&mut self, kit: &mut Kit, asked: Asked) {
        let method = asked.member();
        let device = device_of(&asked);
        let listed = self.state == State::On && self.devices.iter().any(|d| d.path == device && d.listed());
        match method.as_str() {
            "DisplayPasskey" | "DisplayPinCode" => {
                if listed {
                    let body = asked.message().body();
                    let code = if method == "DisplayPasskey" {
                        format!("{:06}", body.deserialize::<(OwnedObjectPath, u32, u16)>().map(|t| t.1).unwrap_or(0))
                    } else {
                        body.deserialize::<(OwnedObjectPath, String)>().map(|t| t.1).unwrap_or_default()
                    };
                    kit.say(&device, &t!("settings-bluetooth-type-code", code = &code), true, true);
                }
                asked.reply(&());
            }
            "RequestConfirmation" | "RequestAuthorization" | "AuthorizeService" | "RequestPinCode" | "RequestPasskey" => {
                if !listed {
                    asked.fail(REJECTED, "No row for this device");
                    return;
                }
                let ask = match method.as_str() {
                    "RequestConfirmation" => {
                        let passkey = asked.message().body().deserialize::<(OwnedObjectPath, u32)>().map(|t| t.1).unwrap_or(0);
                        Ask::Confirm(passkey, asked)
                    }
                    "RequestPinCode" | "RequestPasskey" => {
                        self.pins.insert(device.clone(), String::new());
                        Ask::Pin { number: method == "RequestPasskey", asked }
                    }
                    _ => Ask::Authorise(asked),
                };
                self.asks.insert(device, ask);
            }
            "Release" | "Cancel" => {
                if method == "Cancel" {
                    self.asks.clear();
                    self.pins.clear();
                }
                asked.reply(&());
            }
            _ => asked.fail(REJECTED, "Not supported"),
        }
    }

    fn answer(&mut self, path: &str, yes: bool, why: &'static str) {
        let Some(ask) = self.asks.remove(path) else { return };
        let typed = self.pins.remove(path).unwrap_or_default().trim().to_string();
        match ask {
            Ask::Confirm(_, asked) | Ask::Authorise(asked) => {
                if yes {
                    asked.reply(&());
                } else {
                    asked.fail(REJECTED, why);
                }
            }
            Ask::Pin { number, asked } => match (yes, number, typed.parse::<u32>()) {
                (false, _, _) => asked.fail(REJECTED, why),
                (true, true, Ok(n)) => asked.reply(&n),
                (true, true, Err(_)) => asked.fail(REJECTED, "That is not a number"),
                (true, false, _) if !typed.is_empty() => asked.reply(&typed),
                _ => asked.fail(REJECTED, "No PIN was given"),
            },
        }
    }

    fn ask_node(&self, path: &str, ask: &Ask) -> Node<Msg> {
        let reply = |yes: bool, why: &'static str| {
            let p = path.to_string();
            on::<Bluetooth>(move |me, _| me.answer(&p, yes, why))
        };
        match ask {
            Ask::Confirm(passkey, _) => row(vec![
                label(t!("settings-bluetooth-compare", code = format!("{passkey:06}"))).width(Length::Fill),
                button("window-close-symbolic", t!("settings-bluetooth-no")).on_press(reply(false, "Numbers did not match")),
                crate::toolkit::widget::word_button(t!("settings-bluetooth-yes-matches")).on_press(reply(true, "")),
            ]).gap(8.0),
            Ask::Authorise(_) => row(vec![
                label(t!("settings-bluetooth-pair-question")).width(Length::Fill),
                button("window-close-symbolic", t!("settings-bluetooth-no")).on_press(reply(false, "Refused")),
                button("insert-link-symbolic", t!("settings-bluetooth-pair")).on_press(reply(true, "")),
            ]).gap(8.0),
            Ask::Pin { .. } => {
                let typed = self.pins.get(path).cloned().unwrap_or_default();
                let p = path.to_string();
                let pin = entry(typed, t!("settings-bluetooth-pin"), move |t| {
                    let p = p.clone();
                    on::<Bluetooth>(move |me, _| {
                        me.pins.insert(p.clone(), t.clone());
                    })
                })
                .on_submit(reply(true, ""))
                .width(Length::Fill)
                .first();
                column(vec![
                    label(t!("settings-bluetooth-pin-help")),
                    row(vec![
                        pin,
                        button("window-close-symbolic", t!("settings-bluetooth-cancel")).on_press(reply(false, "No PIN was given")),
                        button("insert-link-symbolic", t!("settings-bluetooth-pair")).on_press(reply(true, "")),
                    ]).gap(8.0).width(Length::Fill),
                ]).gap(8.0)
            }
        }
    }
}

fn agent_path() -> ObjectPath<'static> {
    ObjectPath::from_static_str_unchecked(AGENT_PATH)
}

fn device_of(asked: &Asked) -> String {
    let body = asked.message().body();
    body.deserialize::<(OwnedObjectPath,)>()
        .map(|t| t.0)
        .or_else(|_| body.deserialize::<(OwnedObjectPath, u32)>().map(|t| t.0))
        .or_else(|_| body.deserialize::<(OwnedObjectPath, String)>().map(|t| t.0))
        .or_else(|_| body.deserialize::<(OwnedObjectPath, u32, u16)>().map(|t| t.0))
        .map(|p| p.to_string())
        .unwrap_or_default()
}

fn device_icon(icon: &str) -> &'static str {
    match icon {
        "audio-headset" => "audio-headset-symbolic",
        "audio-headphones" => "audio-headphones-symbolic",
        "audio-card" => "audio-speakers-symbolic",
        "input-mouse" => "input-mouse-symbolic",
        "input-keyboard" => "input-keyboard-symbolic",
        "input-gaming" => "input-gaming-symbolic",
        "phone" => "phone-symbolic",
        "computer" => "computer-symbolic",
        _ => "bluetooth-active-symbolic",
    }
}

fn first_of(p: &Props, keys: &[&str]) -> String {
    keys.iter().map(|k| get_str(p, k)).find(|s| !s.is_empty()).unwrap_or_default()
}

fn path_of(p: &Props, key: &str) -> String {
    p.get(key).and_then(|v| <&ObjectPath>::try_from(v).ok()).map(|o| o.to_string()).unwrap_or_default()
}

fn rssi(d: &Props) -> i32 {
    match d.get("RSSI").and_then(|v| i16::try_from(v).ok()) {
        None | Some(0) => -200,
        Some(n) => i32::from(n),
    }
}

#[cfg(test)]
impl Bluetooth {
    pub(crate) fn flipped() -> Vec<Bluetooth> {
        let mut off = Bluetooth::made_up();
        off.state = State::Off;
        if let Some(a) = off.adapter.as_mut() {
            a.powered = false;
        }
        let mut other = Bluetooth::made_up();
        other.devices[0].connected = false;
        other.devices[1].connected = true;
        let blocked = Bluetooth { state: State::NoAdapter, block: Some(Block::Soft), ..Bluetooth::default() };
        vec![Bluetooth::made_up(), off, other, blocked]
    }

    pub(crate) fn made_up() -> Bluetooth {
        let device = |n: usize, name: &str, icon: &'static str, paired: bool, connected: bool, battery: Option<u32>| Device {
            path: format!("/dev/{n}"), name: name.to_string(), named: true, icon, paired, trusted: paired, connected, battery, rssi: -40 - n as i32,
        };
        Bluetooth {
            state: State::On,
            adapter: Some(Adapter { path: "/hci0".into(), name: "johns-laptop".into(), powered: true, discoverable: false,
                                    discovering: false }),
            devices: vec![
                device(1, "Headphones", "audio-headphones-symbolic", true, true, Some(72)),
                device(2, "Travel mouse", "input-mouse-symbolic", true, false, Some(40)),
                device(3, "Keyboard", "input-keyboard-symbolic", true, false, None),
                device(4, "Living room speaker", "audio-speakers-symbolic", false, false, None),
                device(5, "John's phone", "phone-symbolic", false, false, None),
            ],
            ..Bluetooth::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::ui::Do;

    fn lines(card: &Node<Msg>, of: &[&str]) -> Vec<String> {
        card.said().into_iter().filter(|w| of.contains(&w.as_str())).collect()
    }

    #[test]
    fn a_device_keeps_its_place_when_it_connects() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let mine = ["Headphones", "Travel mouse", "Keyboard"];
        let near = ["Living room speaker", "John's phone"];
        let mut b = Bluetooth::made_up();
        b.devices.reverse();
        b.arrange();
        assert_eq!(lines(&b.mine(&kit), &mine), ["Headphones", "Keyboard", "Travel mouse"]);
        assert_eq!(lines(&b.nearby(&kit), &near), ["Living room speaker", "John's phone"]);
        for d in &mut b.devices {
            d.connected = d.name == "Travel mouse";
        }
        b.devices.reverse();
        b.arrange();
        assert_eq!(lines(&b.mine(&kit), &mine), ["Headphones", "Keyboard", "Travel mouse"], "no line moved");
        assert!(b.mine(&kit).said().contains(&"Connected, battery 40%".to_string()), "and the mouse says it is connected");
        for d in &mut b.devices {
            if d.name == "John's phone" {
                d.rssi = -10;
            }
        }
        b.devices.push(Device { path: "/dev/9".into(), name: "A new lamp".into(), named: true, icon: "phone-symbolic",
                                paired: false, trusted: false, connected: false, battery: None, rssi: -5 });
        b.arrange();
        assert_eq!(lines(&b.nearby(&kit), &["Living room speaker", "John's phone", "A new lamp"]),
                   ["Living room speaker", "John's phone", "A new lamp"]);
        b.order.forget();
        b.arrange();
        assert_eq!(lines(&b.mine(&kit), &mine), ["Travel mouse", "Headphones", "Keyboard"]);
        assert_eq!(lines(&b.nearby(&kit), &["Living room speaker", "John's phone", "A new lamp"]),
                   ["A new lamp", "John's phone", "Living room speaker"]);
    }

    #[test]
    fn the_lists_stay_greyed_with_bluetooth_off() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let off = Bluetooth::flipped().remove(1);
        for card in [off.mine(&kit), off.nearby(&kit)] {
            assert!(card.said().contains(&"Switch on Bluetooth to use these.".to_string()));
            assert!(card.buttons().iter().all(|(tip, usable)| !usable || tip.starts_with("About")),
                    "nothing in it can be pressed: {:?}", card.buttons());
        }
        let on = Bluetooth::made_up();
        assert!(!on.mine(&kit).said().contains(&"Switch on Bluetooth to use these.".to_string()));
        assert!(on.mine(&kit).buttons().iter().any(|(_, usable)| *usable));
    }

    #[test]
    fn switching_on_takes_the_systems_block_off_first_and_a_switch_on_the_computer_is_said() {
        assert_eq!(before_on(Some(Block::Soft)), First::Unblock);
        assert_eq!(before_on(Some(Block::Hard)), First::Impossible, "nothing in software undoes a switch on the computer");
        assert_eq!(before_on(Some(Block::Clear)), First::Nothing);
        assert_eq!(before_on(None), First::Nothing);
        let blocked = Failed::new("org.bluez.Error.Blocked", "Blocked through rfkill");
        let odd = Failed::new("org.bluez.Error.Failed", "0x03");
        assert_eq!(power_words(&blocked, true, Some(Block::Hard)),
                   "Bluetooth is switched off by a switch or a key on the computer itself. Turn that on, then try again.");
        assert_eq!(power_words(&odd, true, Some(Block::Hard)), power_words(&blocked, true, Some(Block::Hard)));
        assert_eq!(power_words(&blocked, true, None), "Bluetooth is blocked by the system, and the block could not be taken off.");
        assert_eq!(power_words(&odd, true, Some(Block::Soft)), power_words(&blocked, true, None));
        assert_eq!(power_words(&odd, true, Some(Block::Clear)), "Bluetooth could not be switched on.");
        assert_eq!(power_words(&odd, false, Some(Block::Hard)), "Bluetooth could not be switched off.");
        for words in [power_words(&blocked, true, Some(Block::Hard)), power_words(&blocked, true, None), power_words(&odd, true, None)] {
            let low = words.to_lowercase();
            assert!(!low.contains("rfkill") && !low.contains("org.bluez") && !low.contains("0x"), "{words}");
        }
    }

    #[test]
    fn a_switch_that_was_refused_goes_back_and_says_why() {
        use crate::settings::ui::stand::Stand;
        let mut s = Stand::new(Box::new(Bluetooth::made_up()));
        let switches = |s: &mut Stand| s.page::<Bluetooth, _>(|b, _| {
            let a = b.adapter.as_ref().unwrap();
            (a.powered, a.discoverable)
        });
        let (powered, visible) = (switches, switches);
        s.page::<Bluetooth, _>(|b, k| {
            b.adapter.as_mut().unwrap().powered = false;
            b.powered(k, false, Err(Failed::new("org.bluez.Error.Failed", "0x03")), None);
        });
        assert!(powered(&mut s).0);
        assert_eq!(s.red(POWER).as_deref(), Some("Bluetooth could not be switched off."));
        s.page::<Bluetooth, _>(|b, k| {
            b.adapter.as_mut().unwrap().powered = true;
            b.powered(k, true, Err(Failed::new("org.bluez.Error.Blocked", "")), Some(Block::Hard));
        });
        assert!(!powered(&mut s).0, "a switch that failed does not stay looking switched");
        assert!(s.red(POWER).is_some_and(|w| w.starts_with("Bluetooth is switched off by a switch")));
        s.page::<Bluetooth, _>(|b, k| b.powered(k, true, Ok(()), None));
        assert_eq!(s.red(POWER), None);
        s.page::<Bluetooth, _>(|b, k| {
            b.adapter.as_mut().unwrap().discoverable = true;
            b.visible_set(k, true, Err(Failed::new("org.bluez.Error.NotReady", "Resource Not Ready")));
        });
        assert!(!visible(&mut s).1, "the switch does not stay looking switched");
        assert_eq!(s.red(VISIBLE).as_deref(),
                   Some("This computer could not be made visible to other devices. Bluetooth is not switched on."));
        s.page::<Bluetooth, _>(|b, k| {
            b.adapter.as_mut().unwrap().discoverable = true;
            b.visible_set(k, true, Ok(()));
        });
        assert!(visible(&mut s).1);
        assert_eq!(s.red(VISIBLE), None);
        s.page::<Bluetooth, _>(|b, k| b.set_visible(k, "/hci0", false));
        assert_eq!(s.red(VISIBLE).as_deref(), Some(crate::settings::ui::no_system_bus()));
        assert!(visible(&mut s).1, "and the switch is left as it was");
    }

    #[test]
    fn with_no_adapter_the_page_says_whether_bluetooth_is_blocked_or_not_there() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let page = |block| Bluetooth { state: State::NoAdapter, block, ..Bluetooth::default() };
        let soft = page(Some(Block::Soft));
        assert_eq!(soft.sentence().0, "Bluetooth is off.");
        let card = soft.no_adapter_card(&kit);
        assert!(card.said().contains(&"Switched off.".to_string()));
        assert!(soft.mine(&kit).said().contains(&"Switch on Bluetooth to use these.".to_string()));
        assert!(card.said().contains(&"switch: Bluetooth".to_string()), "{:?}", card.said());
        let hard = page(Some(Block::Hard));
        assert_eq!(hard.sentence().0, "Bluetooth is switched off on the computer itself.");
        let card = hard.no_adapter_card(&kit);
        assert!(card.said().contains(&"Switched off on the computer itself".to_string()));
        assert!(!card.said().iter().any(|w| w.starts_with("switch:")), "{:?}", card.said());
        let none = page(None);
        assert_eq!(none.sentence().0, "This computer has no Bluetooth.");
        assert!(none.no_adapter_card(&kit).said().contains(&"None was found".to_string()));
        for p in [soft, hard, none] {
            for words in p.no_adapter_card(&kit).said() {
                assert!(!words.to_lowercase().contains("rfkill"), "the page names the kernel's switch: {words}");
            }
        }
    }

    #[test]
    fn what_bluez_refuses_is_said_in_plain_words() {
        let could_not = "It could not be paired.";
        let say = |name: &str, text: &str| bluez_words(&Failed::new(name, text), could_not);
        assert_eq!(say("org.bluez.Error.AuthenticationFailed", "Authentication Failed"),
                   "It could not be paired. The PIN or the number was not accepted. Try again.");
        assert_eq!(say("org.bluez.Error.AuthenticationCanceled", ""), "It could not be paired. It was turned down, here or on the other device.");
        assert_eq!(say("org.bluez.Error.Failed", "br-connection-page-timeout"),
                   "It could not be paired. The device did not answer. Check it is switched on and near, then try again.");
        assert_eq!(say("org.bluez.Error.InProgress", "Operation already in progress"), "It could not be paired. It is already being done. Wait a moment.");
        assert_eq!(say("org.bluez.Error.Failed", "le-connection-abort-by-local"), could_not);
        assert_eq!(say("org.freedesktop.DBus.Error.NoReply", ""), "It could not be paired. It took too long to answer. Try again.");
    }

    #[test]
    fn a_refused_connection_is_always_said_and_an_old_pairing_is_named() {
        let said = |name: &str, text: &str| connect_refusal(&Failed::new(name, text));
        let sorry = "It could not be connected.";
        for (name, text) in [("org.bluez.Error.AuthenticationFailed", "Authentication Failed"),
                             ("org.bluez.Error.AuthenticationRejected", ""),
                             ("org.bluez.Error.AuthenticationCanceled", ""),
                             ("org.bluez.Error.Failed", "br-connection-key-missing")] {
            let r = said(name, text);
            assert!(r.old_pairing, "{name} {text}");
            assert_eq!(r.words, "It could not be connected. The saved pairing is not accepted any more. Forget the device and pair it again.");
        }
        for (name, text, words) in [
            ("org.bluez.Error.NotReady", "Resource Not Ready", "It could not be connected. Bluetooth is not switched on."),
            ("org.bluez.Error.Failed", "br-connection-page-timeout", "It could not be connected. The device did not answer. Check it is switched on and near, then try again."),
            ("org.bluez.Error.ConnectionAttemptFailed", "", "It could not be connected. The device did not answer. Check it is switched on and near, then try again."),
            ("org.bluez.Error.InProgress", "Operation already in progress", "It could not be connected. It is already being done. Wait a moment."),
            ("org.bluez.Error.NotAvailable", "", "It could not be connected. The device is not ready to be used. Put it in pairing mode, then try again."),
            ("org.bluez.Error.DoesNotExist", "Does Not Exist", "It could not be connected. It is no longer known. Look for it again."),
            ("org.freedesktop.DBus.Error.UnknownObject", "no such object", "It could not be connected. It is no longer known. Look for it again."),
            ("org.freedesktop.DBus.Error.NoReply", "", "It could not be connected. It took too long to answer. Try again."),
            ("org.bluez.Error.Failed", "le-connection-abort-by-local", sorry),
            ("", "", sorry),
        ] {
            let r = said(name, text);
            assert_eq!(r.words, words, "{name} {text}");
            assert!(!r.old_pairing, "{name} {text}");
        }
    }

    #[test]
    fn a_row_offers_forget_and_pair_again_only_where_it_helps() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let (forget, again, more) = ("Forget this device", "Forget and pair again", "More about this");
        let mut b = Bluetooth::made_up();
        let has = |b: &Bluetooth, kit: &Kit, word: &str| b.mine(kit).buttons().iter().chain(b.nearby(kit).buttons().iter()).any(|(tip, _)| tip == word);
        assert!(has(&b, &kit, forget), "a paired device can be forgotten");
        assert!(!has(&b, &kit, again) && !has(&b, &kit, more), "nothing wrong, nothing offered");
        let nearby_forgets = |b: &Bluetooth, kit: &Kit| b.nearby(kit).buttons().iter().any(|(tip, _)| tip == forget);
        assert!(!nearby_forgets(&b, &kit), "a device only seen nearby has nothing to forget");
        b.devices[3].trusted = true;
        assert!(nearby_forgets(&b, &kit), "a device BlueZ still trusts can be forgotten");
        b.stale.insert("/dev/2".into());
        kit.say("/dev/2", "It could not be connected.", false, false);
        assert!(has(&b, &kit, again), "a refused pairing offers a fresh start");
        assert!(has(&b, &kit, more), "and a failure links to Help");
        b.stale.clear();
        assert!(!has(&b, &kit, again));
    }

    #[test]
    fn a_connection_that_drops_at_once_is_noticed() {
        let mut b = Bluetooth::made_up();
        assert!(!b.dropped("/dev/1"), "the headphones stay connected");
        assert!(b.dropped("/dev/2"), "the mouse is not connected");
        assert!(!b.dropped("/dev/99"), "a device the page does not list is not said to drop");
        b.state = State::Off;
        assert!(!b.dropped("/dev/2"));
    }

    #[test]
    fn the_headline_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-bluetooth-top-gone = Bluetooth ne tourne pas sur cet ordinateur.\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            let gone = Bluetooth { state: State::Gone, ..Bluetooth::default() };
            assert_eq!(gone.sentence().0, "Bluetooth ne tourne pas sur cet ordinateur.");
            let off = Bluetooth { state: State::Off, ..Bluetooth::default() };
            assert_eq!(off.sentence().0, "Bluetooth is off.");
        });
    }
}
