//! The Bluetooth panel.

use std::collections::HashSet;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::Duration;

use zbus::zvariant::{ObjectPath, OwnedObjectPath, Value};

use crate::common::rfkill;
use crate::common::run::capture;
use crate::ring::ctx::Msg;
use crate::toolkit::t;
use crate::toolkit::bus::{Asked, Failed, Offer, Props};
use crate::toolkit::widget::{button, card, label, Node, row, space, switch};

use super::{banded, Body, Dev, line_of, m, Quick, ROW_GAP, Well};

const BLUEZ: &str = "org.bluez";

const DEVICE: &str = "org.bluez.Device1";

const AGENT_MANAGER: &str = "org.bluez.AgentManager1";

const AGENT_IFACE: &str = "org.bluez.Agent1";

const AGENT_PATH: &str = "/org/trindle/ring_bluetooth_agent";

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

const BT_EVERY: Duration = Duration::from_secs(3);

const PAIR_WAIT: Duration = Duration::from_secs(60);

const CONNECT_WAIT: Duration = Duration::from_secs(30);

const BT_VERB_TIMEOUT: &str = "25";

const SCAN_TIMEOUT: &str = "120";

pub(super) fn parse_bt(out: &str) -> (bool, Vec<Dev>, Vec<(String, String)>) {
    let on = out.lines().next() == Some("on");
    let (mut all, mut paired, mut connected) = (Vec::new(), HashSet::new(), HashSet::new());
    let mut section = "";
    for l in out.lines() {
        if let Some(s) = l.strip_prefix('#') {
            section = match s { "all" => "all", "paired" => "paired", "connected" => "connected", _ => "" };
            continue;
        }
        let mut parts = l.splitn(3, ' ');
        let (Some("Device"), Some(mac)) = (parts.next(), parts.next()) else { continue };
        let name = parts.next().unwrap_or("").trim().to_string();
        match section {
            "all" => all.push((mac.to_string(), name)),
            "paired" => { paired.insert(mac.to_string()); }
            "connected" => { connected.insert(mac.to_string()); }
            _ => {}
        }
    }
    let mut devices: Vec<Dev> = all.iter()
        .filter(|(mac, name)| !name.is_empty() && name.replace('-', ":") != *mac)
        .map(|(mac, name)| Dev { mac: mac.clone(), name: name.clone(), paired: paired.contains(mac), connected: connected.contains(mac) })
        .collect();
    devices.sort_by_key(|d| (!d.connected, !d.paired, d.name.to_lowercase()));
    (on, devices, all)
}

fn device_path(mac: &str) -> String {
    let adapter = std::fs::read_dir("/sys/class/bluetooth").ok()
        .and_then(|d| d.flatten().map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.starts_with("hci") && !n.contains(':')).min())
        .unwrap_or_else(|| "hci0".into());
    format!("/org/bluez/{adapter}/dev_{}", mac.replace(':', "_"))
}

pub(super) fn mac_of(path: &str) -> String {
    path.split("dev_").nth(1).unwrap_or("").replace('_', ":")
}

impl Quick {
    pub(super) fn bluetooth(&mut self) {
        self.body = Body::Bt(None, vec![]);
        self.start_agent();
        self.bt_draw();
        self.repeat(BT_EVERY, |q| q.bt_draw());
    }

    pub(super) fn bt_view(&self, on: Option<bool>, devices: &[Dev]) -> Vec<Node<Msg>> {
        let top = card(row(vec![
            label(t!("ring-quick-bt-switch")),
            space(),
            switch(on == Some(true), t!("ring-quick-bt-switch"), |on| m(move |q| q.bt_power(on))),
        ]).gap(ROW_GAP)).snug().flat();
        let rows: Vec<Node<Msg>> = match on {
            None => vec![],
            Some(false) => vec![label(t!("ring-quick-bt-off"))],
            Some(true) if devices.is_empty() => vec![label(t!("ring-quick-bt-searching"))],
            Some(true) => devices.iter().map(|d| {
                let mut line = vec![line_of(d.name.clone(), false, 400)];
                let (icon_name, tip, verb) = if d.connected {
                    ("network-offline-symbolic", t!("ring-quick-bt-disconnect"), "disconnect")
                } else if d.paired {
                    ("network-transmit-receive-symbolic", t!("ring-quick-bt-connect"), "connect")
                } else {
                    ("list-add-symbolic", t!("ring-quick-bt-pair"), "pair")
                };
                let mac = d.mac.clone();
                line.push(button(icon_name, tip).on(d.connected).off(self.busy.contains(&d.mac))
                    .on_press(m(move |q| q.bt_do(&mac, verb))).well());
                row(line).gap(ROW_GAP)
            }).collect(),
        };
        vec![top, self.scrolling(banded(rows))]
    }

    fn bt_draw(&mut self) {
        let b = self.back();
        capture(&["sh", "-c", "bluetoothctl show | grep -q 'Powered: yes' && echo on; echo '#all'; bluetoothctl devices; \
                               echo '#paired'; bluetoothctl devices Paired; echo '#connected'; bluetoothctl devices Connected"],
                None, move |_, out, _| b.then(move |q| q.bt_drawn(&out)));
    }

    fn bt_drawn(&mut self, out: &str) {
        let (on, mut devices, names) = parse_bt(out);
        self.order.keep(&mut devices, |d| d.mac.clone());
        self.bt_names.extend(names);
        if on {
            self.start_scan();
        }
        self.body = Body::Bt(Some(on), devices);
    }

    fn bt_power(&mut self, on: bool) {
        if let Body::Bt(state, _) = &mut self.body {
            *state = Some(on);
        }
        if on && rfkill::state(rfkill::Radio::Bluetooth) == Some(rfkill::Block::Soft) {
            if let Err(e) = rfkill::unblock(rfkill::Radio::Bluetooth) {
                eprintln!("trindle-ring: could not unblock Bluetooth: {e:?}");
            }
        }
        let b = self.back();
        capture(&["bluetoothctl", "power", if on { "on" } else { "off" }], None, move |_, _, _| b.then(|q| q.bt_draw()));
    }

    fn start_scan(&mut self) {
        if self.scan.is_none() {
            self.scan = Command::new("bluetoothctl").args(["--timeout", SCAN_TIMEOUT, "scan", "on"])
                .stdout(Stdio::null()).stderr(Stdio::null()).spawn().ok();
        }
    }

    pub(super) fn stop_scan(&mut self) {
        if let Some(mut child) = self.scan.take() {
            std::thread::spawn(move || {
                let _ = child.kill();
                let _ = child.wait();
                let _ = Command::new("bluetoothctl").args(["scan", "off"]).stdout(Stdio::null()).stderr(Stdio::null()).status();
            });
        }
    }

    fn bt_name(&self, mac: &str) -> String {
        self.bt_names.get(mac).filter(|n| !n.is_empty()).cloned().unwrap_or_else(|| t!("ring-quick-bt-device").into())
    }

    fn bt_do(&mut self, mac: &str, verb: &'static str) {
        let name = self.bt_name(mac);
        self.busy.insert(mac.to_string());
        if verb == "pair" {
            self.say(&t!("ring-quick-bt-pairing", name = name));
            self.pair(mac.to_string());
            return;
        }
        self.say(&if verb == "connect" { t!("ring-quick-bt-connecting", name = name) } else { t!("ring-quick-bt-disconnecting", name = name) });
        let (b, mine) = (self.back(), mac.to_string());
        capture(&["bluetoothctl", "--timeout", BT_VERB_TIMEOUT, verb, mac], None, move |ok, out, _| b.then(move |q| {
            let mac = mine;
            q.busy.remove(&mac);
            let failed = !ok || out.contains("Failed") || out.contains("not available");
            q.say(if failed { t!("ring-quick-bt-failed") } else { "" });
        }));
    }

    fn pair(&mut self, mac: String) {
        let Some(bus) = self.ctx().system.clone() else { return };
        let path = device_path(&mac);
        let b = self.back();
        let call_path = path.clone();
        bus.call(BLUEZ, &call_path, DEVICE).method("Pair", &(), false, Some(PAIR_WAIT), &self.ctx().lp, move |r| {
            let e = r.err().map(|e: Failed| format!("{} {}", e.name, e.text));
            b.then(move |q| q.paired(mac, path, e));
        });
    }

    fn paired(&mut self, mac: String, path: String, failed: Option<String>) {
        self.busy.remove(&mac);
        self.cancel_ask();
        if let Some(e) = failed {
            let again = e.contains("AlreadyExists");
            self.say(if e.contains("AuthenticationFailed") || e.contains("Rejected") || e.contains("Canceled") {
                t!("ring-quick-bt-codes")
            } else if again { "" } else { t!("ring-quick-bt-not-paired") });
            if !again {
                return;
            }
        }
        let Some(bus) = self.ctx().system.clone() else { return };
        let lp = &self.ctx().lp;
        bus.call(BLUEZ, &path, DEVICE).set("Trusted", Value::from(true), false, lp, |_| {});
        let b = self.back();
        bus.call(BLUEZ, &path, DEVICE).method("Connect", &(), false, Some(CONNECT_WAIT), lp, move |r| {
            let ok = r.is_ok();
            b.then(move |q| q.say(if ok { "" } else { t!("ring-quick-bt-paired-not-connected") }));
        });
    }

    fn start_agent(&mut self) {
        if self.agent.is_some() {
            return;
        }
        let Some(bus) = self.ctx().system.clone() else { return };
        let w = Rc::downgrade(self.ctx());
        let offer = Offer {
            name: AGENT_IFACE.into(),
            xml: AGENT_XML.into(),
            call: Box::new(move |asked: Asked| match w.upgrade() {
                Some(ctx) => ctx.face(move |face| face.quick.agent_asked(asked)),
                None => asked.fail(REJECTED, "Closed"),
            }),
            props: Box::new(Props::new),
            set: None,
        };
        let Ok(served) = bus.serve(AGENT_PATH, offer) else { return };
        self.agent = Some(served);
        let Ok(path) = ObjectPath::try_from(AGENT_PATH) else { return };
        let lp = self.ctx().lp.clone();
        let (bus2, lp2) = (bus.clone(), lp.clone());
        bus.call(BLUEZ, "/org/bluez", AGENT_MANAGER).method("RegisterAgent", &(path.clone(), "KeyboardDisplay"), false, None, &lp,
            move |r| {
                if r.is_ok() {
                    bus2.call(BLUEZ, "/org/bluez", AGENT_MANAGER).method("RequestDefaultAgent", &(path,), false, None, &lp2, |_| {});
                }
            });
    }

    pub(super) fn stop_agent(&mut self) {
        let Some(served) = self.agent.take() else { return };
        if let (Some(bus), Ok(path)) = (&self.ctx().system, ObjectPath::try_from(AGENT_PATH)) {
            bus.call(BLUEZ, "/org/bluez", AGENT_MANAGER).method("UnregisterAgent", &(path,), false, None, &self.ctx().lp, |_| {});
        }
        drop(served);
    }

    fn agent_asked(&mut self, asked: Asked) {
        if self.agent.is_none() {
            asked.fail(REJECTED, "Closed");
            return;
        }
        let body = asked.message().body();
        let member = asked.member();
        match member.as_str() {
            "RequestPinCode" | "RequestPasskey" => self.ask_for_pin(asked, &body, member == "RequestPasskey"),
            "DisplayPasskey" | "DisplayPinCode" => self.show_pin(asked, &body, member == "DisplayPasskey"),
            "RequestConfirmation" => self.ask_same_number(asked, &body),
            "RequestAuthorization" => self.ask_to_pair(asked, &body),
            "Cancel" => {
                asked.reply(&());
                self.cancel_ask();
            }
            _ => asked.reply(&()),
        }
    }

    fn ask_for_pin(&mut self, asked: Asked, body: &zbus::message::Body, passkey: bool) {
        let Ok((path,)) = body.deserialize::<(OwnedObjectPath,)>() else { return asked.fail(REJECTED, "Refused") };
        let name = self.bt_name(&mac_of(path.as_str()));
        self.ask("bluetooth-active-symbolic", name, t!("ring-quick-bt-ask-pin"),
                 None, Some((t!("ring-quick-bt-pin-box"), false)), t!("ring-quick-bt-ask-pair"), t!("ring-quick-cancel"), move |_, typed| {
            let typed = typed.map(|t| t.trim().to_string()).unwrap_or_default();
            if passkey {
                match typed.parse::<u32>() {
                    Ok(n) => asked.reply(&(n,)),
                    Err(_) => asked.fail(REJECTED, "Refused"),
                }
            } else if !typed.is_empty() {
                asked.reply(&(typed,));
            } else {
                asked.fail(REJECTED, "Refused");
            }
        });
    }

    fn show_pin(&mut self, asked: Asked, body: &zbus::message::Body, passkey: bool) {
        let shown = if passkey {
            body.deserialize::<(OwnedObjectPath, u32, u16)>().ok().map(|(p, n, _)| (p, format!("{n:06}")))
        } else {
            body.deserialize::<(OwnedObjectPath, String)>().ok()
        };
        asked.reply(&());
        let Some((path, code)) = shown else { return };
        let path = path.as_str().to_string();
        let name = self.bt_name(&mac_of(&path));
        self.ask("bluetooth-active-symbolic", name, t!("ring-quick-bt-type-on-device"), Some(code),
                 None, "", t!("ring-quick-cancel"), move |q, typed| {
            if typed.is_none() {
                if let Some(bus) = &q.ctx().system {
                    bus.call(BLUEZ, &path, DEVICE).method("CancelPairing", &(), false, None, &q.ctx().lp, |_| {});
                }
            }
        });
    }

    fn ask_same_number(&mut self, asked: Asked, body: &zbus::message::Body) {
        let Ok((path, n)) = body.deserialize::<(OwnedObjectPath, u32)>() else { return asked.fail(REJECTED, "Refused") };
        let name = self.bt_name(&mac_of(path.as_str()));
        self.ask("bluetooth-active-symbolic", name, t!("ring-quick-bt-same-number"), Some(format!("{n:06}")),
                 None, t!("ring-quick-bt-matches"), t!("ring-quick-no"), move |_, typed| match typed {
            Some(_) => asked.reply(&()),
            None => asked.fail(REJECTED, "Refused"),
        });
    }

    fn ask_to_pair(&mut self, asked: Asked, body: &zbus::message::Body) {
        let Ok((path,)) = body.deserialize::<(OwnedObjectPath,)>() else { return asked.fail(REJECTED, "Refused") };
        let name = self.bt_name(&mac_of(path.as_str()));
        self.ask("bluetooth-active-symbolic", name, t!("ring-quick-bt-pair-this"), None, None, t!("ring-quick-bt-ask-pair"), t!("ring-quick-no"),
                 move |_, typed| match typed {
            Some(_) => asked.reply(&()),
            None => asked.fail(REJECTED, "Refused"),
        });
    }
}
