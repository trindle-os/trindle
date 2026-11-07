//! Network.

use std::collections::HashMap;
use std::time::Duration;

use super::{Ap, Asking, CONNECTION, DEVICE, NM, NM_PATH, Network, ap_key, op, refused_words};
use crate::settings::ui::{line, no_more, on, Kit, Msg};
use crate::toolkit::bus::{get_u32, Failed, Props};
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, row, Length, Node};
use zbus::Message;
use zbus::zvariant::{OwnedObjectPath, Value};

impl Network {
    pub(super) fn connect(&mut self, kit: &mut Kit, name: &str, ap: &Ap) {
        let device = self.wifi_device.clone().unwrap_or_default();
        if let Some(conn) = self.shown.saved.get(name).cloned() {
            let key = ap_key(name);
            let Some(bus) = kit.system_for(&key) else { return };
            kit.say(&key, t!("settings-network-joining"), true, true);
            let kept = if ap.secure { Password::Kept } else { Password::None };
            let back = joining(kit, key, device.clone(), false, kept);
            bus.call(NM, NM_PATH, NM).method("ActivateConnection", &(op(&conn), op(&device), op(&ap.path)), false,
                                             Some(Duration::from_secs(30)), &kit.events, back);
            return;
        }
        if !ap.secure {
            self.add(kit, name, ap, None);
            return;
        }
        self.asking = Some(Asking::Password(name.to_string()));
        self.pass_typed.clear();
    }

    pub(super) fn close_entry(&mut self) {
        self.asking = None;
        self.name_typed.clear();
        self.pass_typed.clear();
    }

    pub(super) fn add(&mut self, kit: &mut Kit, name: &str, ap: &Ap, password: Option<String>) {
        let key = ap_key(name);
        if password.as_ref().is_some_and(|p| p.chars().count() < 8) {
            kit.say(&key, t!("settings-network-password-short"), false, false);
            return;
        }
        let Some(bus) = kit.system_for(&key) else { return };
        let device = self.wifi_device.clone().unwrap_or_default();
        let mut settings: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        settings.insert("connection", HashMap::from([("id", Value::from(name)), ("type", Value::from("802-11-wireless"))]));
        settings.insert("802-11-wireless", HashMap::from([("ssid", Value::from(name.as_bytes().to_vec()))]));
        if let Some(p) = &password {
            settings.insert("802-11-wireless-security",
                            HashMap::from([("key-mgmt", Value::from("wpa-psk")), ("psk", Value::from(p.as_str()))]));
        }
        kit.say(&key, t!("settings-network-joining"), true, true);
        let typed = if password.is_some() { Password::Typed } else { Password::None };
        let back = joining(kit, key, device.clone(), true, typed);
        bus.call(NM, NM_PATH, NM).method("AddAndActivateConnection", &(settings, op(&device), op(&ap.path)), false,
                                         Some(Duration::from_secs(45)), &kit.events, back);
    }

    pub(super) fn hidden_line(&self, kit: &Kit) -> Node<Msg> {
        let mut l = line(t!("settings-network-hidden"))
            .sub(t!("settings-network-hidden-sub"))
            .icon("network-wireless-encrypted-symbolic")
            .key(HIDDEN)
            .control(button("list-add-symbolic", t!("settings-network-hidden")).on_press(on::<Network>(|me, k| {
                me.asking = Some(Asking::Hidden);
                me.name_typed.clear();
                me.pass_typed.clear();
                k.hush(HIDDEN);
            })));
        if self.asking == Some(Asking::Hidden) {
            let join = on::<Network>(|me, k| me.join_hidden(k));
            let name = entry(self.name_typed.clone(), t!("settings-network-hidden-name"), |t| on::<Network>(move |me, _| me.name_typed = t.clone()))
                .width(Length::Fill)
                .first();
            let pass = entry(self.pass_typed.clone(), t!("settings-network-hidden-password"), |t| {
                on::<Network>(move |me, _| me.pass_typed = t.clone())
            })
            .key("network:hidden-password")
            .secret()
            .on_submit(join.clone())
            .width(Length::Fill);
            l = l.below(row(vec![
                name,
                pass,
                crate::toolkit::widget::word_button(t!("settings-network-join")).on_press(join),
                button("window-close-symbolic", t!("settings-network-cancel")).on_press(on::<Network>(|me, _| me.close_entry())),
            ]).gap(8.0).width(Length::Fill));
        }
        l.node(kit)
    }

    fn join_hidden(&mut self, kit: &mut Kit) {
        let typed = self.name_typed.trim().to_string();
        if typed.is_empty() {
            kit.say(HIDDEN, t!("settings-network-name-needed"), false, false);
            return;
        }
        let password = (!self.pass_typed.is_empty()).then(|| self.pass_typed.clone());
        self.add_hidden(kit, &typed, password);
    }

    fn add_hidden(&mut self, kit: &mut Kit, name: &str, password: Option<String>) {
        if password.as_ref().is_some_and(|p| p.chars().count() < 8) {
            kit.say(HIDDEN, t!("settings-network-password-short"), false, false);
            return;
        }
        let Some(bus) = kit.system_for(HIDDEN) else { return };
        let device = self.wifi_device.clone().unwrap_or_default();
        let mut settings: HashMap<&str, HashMap<&str, Value>> = HashMap::new();
        settings.insert("connection", HashMap::from([("id", Value::from(name)), ("type", Value::from("802-11-wireless"))]));
        settings.insert("802-11-wireless", HashMap::from([
            ("ssid", Value::from(name.as_bytes().to_vec())),
            ("hidden", Value::from(true)),
            ("mode", Value::from("infrastructure")),
        ]));
        if let Some(p) = &password {
            settings.insert("802-11-wireless-security",
                            HashMap::from([("key-mgmt", Value::from("wpa-psk")), ("psk", Value::from(p.as_str()))]));
        }
        kit.say(HIDDEN, t!("settings-network-joining"), true, true);
        let typed = if password.is_some() { Password::Typed } else { Password::None };
        let back = joining(kit, HIDDEN.to_string(), device.clone(), true, typed);
        bus.call(NM, NM_PATH, NM).method("AddAndActivateConnection", &(settings, op(&device), op("/")), false,
                                         Some(Duration::from_secs(45)), &kit.events, back);
    }

    fn watch_join(&mut self, kit: &mut Kit, j: Joining) {
        if j.active.is_empty() || j.active == "/" {
            self.join_ended(kit, j, Join::Failed(0));
            return;
        }
        kit.after::<Network>(JOIN_LOOK, move |me, k| {
            let Some(bus) = k.system.clone() else {
                me.join_ended(k, j, Join::Failed(0));
                return;
            };
            let (device, active) = (j.device.clone(), j.active.clone());
            let back = k.then::<Network, (Props, Option<u32>)>(move |me, k, (device, active)| {
                let mut j = j;
                let state = get_u32(&device, "State");
                j.looks += 1;
                j.auth = if state == STATE_NEED_AUTH { j.auth + 1 } else { 0 };
                match join_step(active, state, state_reason(&device), j.auth) {
                    Join::Waiting if j.looks < JOIN_LOOKS => me.watch_join(k, j),
                    Join::Waiting => me.join_ended(k, j, Join::Failed(0)),
                    ended => me.join_ended(k, j, ended),
                }
            });
            let (again, events) = (bus.clone(), k.events.clone());
            bus.call(NM, &device, DEVICE).get_all(&k.events, move |found| {
                let found = found.unwrap_or_default();
                again.call(NM, &active, ACTIVE).get_all(&events, move |r| back((found, r.ok().map(|p| get_u32(&p, "State")))));
            });
        });
    }

    pub(super) fn join_ended(&mut self, kit: &mut Kit, j: Joining, how: Join) {
        let reason = match how {
            Join::Joined => {
                self.close_entry();
                kit.say(&j.key, t!("settings-network-joined"), true, false);
                self.refresh(kit);
                return;
            }
            Join::Failed(reason) => reason,
            Join::Waiting => return,
        };
        let words = join_failed_words(reason, j.password);
        kit.say(&j.key, words, false, false);
        let (Some(added), Some(bus)) = (j.added, kit.system.clone()) else {
            self.refresh(kit);
            return;
        };
        let key = j.key;
        let back = kit.then::<Network, Result<Message, Failed>>(move |me, k, forgotten| {
            if forgotten.is_err() {
                k.say(&key, &t!("settings-network-kept-anyway", words = words), false, false);
            }
            me.refresh(k);
        });
        bus.call(NM, &added, CONNECTION).method("Delete", &(), false, None, &kit.events, back);
    }

}

const ACTIVE: &str = "org.freedesktop.NetworkManager.Connection.Active";

const HIDDEN: &str = "network:hidden";

pub(super) const STATE_NEED_AUTH: u32 = 60;
pub(super) const ACTIVE_JOINED: u32 = 2;
pub(super) const ACTIVE_LEAVING: u32 = 3;
pub(super) const ACTIVE_LEFT: u32 = 4;
const JOIN_LOOK: Duration = Duration::from_millis(1000);
const JOIN_LOOKS: u32 = 40;
pub(super) const AUTH_LOOKS: u32 = 3;

pub(super) struct Joining {
    pub(super) key: String,
    pub(super) device: String,
    pub(super) active: String,
    pub(super) added: Option<String>,
    pub(super) password: Password,
    pub(super) looks: u32,
    pub(super) auth: u32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Password {
    None,
    Typed,
    Kept,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Join {
    Joined,
    Waiting,
    Failed(u32),
}

pub(super) const REASON_NO_PASSWORD: u32 = 7;

pub(super) fn join_step(active: Option<u32>, device: u32, reason: u32, auth: u32) -> Join {
    match active {
        Some(ACTIVE_JOINED) => Join::Joined,
        Some(ACTIVE_LEAVING) | Some(ACTIVE_LEFT) | None => Join::Failed(reason),
        _ if device == STATE_NEED_AUTH && auth >= AUTH_LOOKS => Join::Failed(REASON_NO_PASSWORD),
        _ => Join::Waiting,
    }
}

pub(super) fn join_failed_words(reason: u32, password: Password) -> &'static str {
    match (reason, password) {
        (7..=11, Password::Typed) => t!("settings-network-join-password-typed"),
        (7..=11, Password::Kept) => t!("settings-network-join-password-kept"),
        (53, _) => t!("settings-network-join-not-found"),
        (5 | 15..=17, _) => t!("settings-network-join-no-address"),
        _ => t!("settings-network-join-failed"),
    }
}

fn state_reason(device: &Props) -> u32 {
    device.get("StateReason").and_then(|v| v.try_clone().ok()).and_then(|v| <(u32, u32)>::try_from(v).ok()).map_or(0, |(_, why)| why)
}

pub(super) fn joining(kit: &Kit, key: String, device: String, added: bool, password: Password) -> impl FnOnce(Result<Message, Failed>) + 'static {
    kit.then::<Network, Result<Message, Failed>>(move |me, k, answer| match answer {
        Ok(m) => {
            let body = m.body();
            let (settings, active) = if added {
                body.deserialize::<(OwnedObjectPath, OwnedObjectPath)>().map(|(s, a)| (Some(s.to_string()), a.to_string())).unwrap_or_default()
            } else {
                let attempt = body.deserialize::<OwnedObjectPath>().ok()
                    .or_else(|| body.deserialize::<(OwnedObjectPath,)>().ok().map(|(a,)| a));
                (None, attempt.map(|a| a.to_string()).unwrap_or_default())
            };
            me.watch_join(k, Joining { key, device, active, added: settings, password, looks: 0, auth: 0 });
        }
        Err(e) => {
            k.say(&key, &refused_words(&e), false, false);
            me.refresh(k);
        }
    })
}

pub(super) fn disconnect(kit: &mut Kit, device: &str, key: &str) {
    let Some(bus) = kit.system_for(key) else { return };
    let back = kit.settles::<Network, Message, Failed>(key, t!("settings-network-disconnect-failed"), no_more);
    bus.call(NM, device, DEVICE).method("Disconnect", &(), false, None, &kit.events, back);
}
