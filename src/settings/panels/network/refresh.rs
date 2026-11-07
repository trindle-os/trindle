//! Network.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use super::{Ap, CONNECTION, DEVICE, Found, NM, NM_PATH, Network, State, Wired};
use crate::settings::ui::Kit;
use crate::toolkit::bus::{get_bool, get_bytes, get_paths, get_str, get_u32, Failed, Props};
use zbus::Message;
use zbus::zvariant::{OwnedObjectPath, Value};

impl Network {
    pub(super) fn soon(&mut self, kit: &mut Kit) {
        self.wait += 1;
        let n = self.wait;
        kit.after::<Network>(Duration::from_millis(600), move |me, k| {
            if me.wait == n && me.asking.is_none() {
                me.refresh(k);
            }
        });
    }

    pub(super) fn refresh(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.clone() else { return };
        self.round += 1;
        let round = self.round;
        let back = kit.then::<Network, Result<Props, Failed>>(move |me, k, r| {
            if me.round == round {
                me.got_nm(k, r);
            }
        });
        bus.call(NM, NM_PATH, NM).get_all(&kit.events, back);
    }

    fn got_nm(&mut self, kit: &mut Kit, res: Result<Props, Failed>) {
        let Ok(props) = res else {
            self.gone();
            return;
        };
        self.state = State::Ready;
        self.enabled = get_bool(&props, "WirelessEnabled");
        let hardware = props.get("WirelessHardwareEnabled").and_then(|v| bool::try_from(v).ok()).unwrap_or(true);
        self.problem = if hardware { String::new() } else { crate::common::rfkill::hard_words(crate::common::rfkill::Radio::Wifi) };
        let Some(bus) = kit.system.clone() else { return };
        let round = self.round;
        let back = kit.then::<Network, Result<Message, Failed>>(move |me, k, r| {
            if me.round == round {
                let paths = r.ok()
                    .and_then(|m| m.body().deserialize::<(Vec<OwnedObjectPath>,)>().ok())
                    .map(|(p,)| p.into_iter().map(|p| p.to_string()).collect())
                    .unwrap_or_default();
                me.got_devices(k, paths);
            }
        });
        bus.call(NM, NM_PATH, NM).method("GetDevices", &(), false, None, &kit.events, back);
    }

    fn got_devices(&mut self, kit: &mut Kit, paths: Vec<String>) {
        self.next = Found { waiting: paths.len(), ..Found::default() };
        if paths.is_empty() {
            self.finish();
            return;
        }
        let Some(bus) = kit.system.clone() else { return };
        for path in paths {
            let round = self.round;
            let p = path.clone();
            let back = kit.then::<Network, Result<Props, Failed>>(move |me, k, r| {
                if me.round == round {
                    me.got_device(k, &p, r.unwrap_or_default());
                }
            });
            bus.call(NM, &path, DEVICE).get_all(&kit.events, back);
        }
    }

    fn got_device(&mut self, kit: &mut Kit, path: &str, dev: Props) {
        let kind = get_u32(&dev, "DeviceType");
        if kind == DEVICE_WIFI {
            self.next.wifi = true;
        }
        if kind == DEVICE_ETHERNET {
            self.next.wired.push(Wired { path: path.to_string(), name: get_str(&dev, "Interface"), state: get_u32(&dev, "State") });
            self.one_done();
        } else if kind == DEVICE_WIFI && self.enabled {
            let first = self.wifi_device.replace(path.to_string()).is_none();
            if first {
                self.scan(kit);
            }
            self.next.device_state = get_u32(&dev, "State");
            let Some(bus) = kit.system.clone() else { return };
            let round = self.round;
            let back = kit.then::<Network, Result<Props, Failed>>(move |me, k, r| {
                if me.round == round {
                    me.got_wifi(k, r.unwrap_or_default());
                }
            });
            bus.call(NM, path, WIRELESS).get_all(&kit.events, back);
        } else {
            self.one_done();
        }
    }

    fn one_done(&mut self) {
        self.next.waiting = self.next.waiting.saturating_sub(1);
        if self.next.waiting == 0 {
            self.finish();
        }
    }

    fn finish(&mut self) {
        self.shown = std::mem::take(&mut self.next);
        self.arrange();
        self.no_driver = !self.shown.wifi && chip_without_driver(std::path::Path::new("/sys/bus/pci/devices"));
        self.ready = true;
    }

    pub(super) fn scan(&self, kit: &Kit) {
        let (Some(bus), Some(dev)) = (kit.system.clone(), self.wifi_device.clone()) else { return };
        let options: HashMap<&str, Value> = HashMap::new();
        bus.call(NM, &dev, WIRELESS).method("RequestScan", &(options,), false, None, &kit.events, |_| {});
    }

    fn got_wifi(&mut self, kit: &mut Kit, wireless: Props) {
        self.next.active_ap = get_path(&wireless, "ActiveAccessPoint");
        self.next.bitrate = get_u32(&wireless, "Bitrate");
        let aps = get_paths(&wireless, "AccessPoints");
        let Some(bus) = kit.system.clone() else { return };
        let round = self.round;
        let back = kit.then::<Network, Result<Message, Failed>>(move |me, k, r| {
            if me.round == round {
                let saved = r.ok()
                    .and_then(|m| m.body().deserialize::<(Vec<OwnedObjectPath>,)>().ok())
                    .map(|(p,)| p.into_iter().map(|p| p.to_string()).collect())
                    .unwrap_or_default();
                me.got_saved(k, aps, saved);
            }
        });
        bus.call(NM, SETTINGS_PATH, SETTINGS).method("ListConnections", &(), false, None, &kit.events, back);
    }

    fn got_saved(&mut self, kit: &mut Kit, aps: Vec<String>, saved: Vec<String>) {
        if saved.is_empty() {
            self.collect_aps(kit, aps);
            return;
        }
        let Some(bus) = kit.system.clone() else { return };
        self.next.saved_waiting = saved.len();
        let aps = Rc::new(aps);
        for conn in saved {
            let (round, c, aps) = (self.round, conn.clone(), aps.clone());
            let back = kit.then::<Network, Result<Message, Failed>>(move |me, k, r| {
                if me.round != round {
                    return;
                }
                type Groups = HashMap<String, Props>;
                let groups = r.ok().and_then(|m| m.body().deserialize::<(Groups,)>().ok()).map(|(g,)| g).unwrap_or_default();
                if let Some(wifi) = groups.get("802-11-wireless") {
                    if wifi.contains_key("ssid") {
                        me.next.saved.insert(ssid_text(&get_bytes(wifi, "ssid")), c.clone());
                    }
                }
                me.next.saved_waiting = me.next.saved_waiting.saturating_sub(1);
                if me.next.saved_waiting == 0 {
                    me.collect_aps(k, aps.to_vec());
                }
            });
            bus.call(NM, &conn, CONNECTION).method("GetSettings", &(), false, None, &kit.events, back);
        }
    }

    fn collect_aps(&mut self, kit: &mut Kit, aps: Vec<String>) {
        if aps.is_empty() {
            self.listed();
            return;
        }
        let Some(bus) = kit.system.clone() else { return };
        self.next.aps_waiting = aps.len();
        for path in aps {
            let (round, p) = (self.round, path.clone());
            let back = kit.then::<Network, Result<Props, Failed>>(move |me, _, r| {
                if me.round != round {
                    return;
                }
                me.got_ap(p, r.unwrap_or_default());
                me.next.aps_waiting = me.next.aps_waiting.saturating_sub(1);
                if me.next.aps_waiting == 0 {
                    me.listed();
                }
            });
            bus.call(NM, &path, ACCESS_POINT).get_all(&kit.events, back);
        }
    }

    fn got_ap(&mut self, path: String, props: Props) {
        let ssid = get_bytes(&props, "Ssid");
        if ssid.is_empty() {
            return;
        }
        let name = ssid_text(&ssid);
        let strength = props.get("Strength").and_then(|v| u8::try_from(v).ok()).map(u32::from).unwrap_or(0);
        let ap = Ap {
            path: path.clone(),
            strength,
            secure: get_u32(&props, "Flags") & 1 != 0 || get_u32(&props, "WpaFlags") != 0 || get_u32(&props, "RsnFlags") != 0,
        };
        let f = &mut self.next;
        let holding_active = f.aps.get(&name).is_some_and(|old| old.path == f.active_ap);
        if path == f.active_ap || (!holding_active && f.aps.get(&name).is_none_or(|old| ap.strength > old.strength)) {
            f.aps.insert(name, ap);
        }
    }

    pub(super) fn listed(&mut self) {
        self.next.listed = true;
        self.one_done();
    }

    pub(super) fn arrange(&mut self) {
        let f = &self.shown;
        let mut names: Vec<(&String, &Ap)> = f.aps.iter().collect();
        names.sort_by_key(|(name, ap)| (ap.path != f.active_ap, !f.saved.contains_key(*name), std::cmp::Reverse(ap.strength), (*name).clone()));
        let mut names: Vec<String> = names.into_iter().map(|(name, _)| name.clone()).collect();
        self.order.keep(&mut names, String::clone);
        self.ap_names = names;
    }

}

const SETTINGS_PATH: &str = "/org/freedesktop/NetworkManager/Settings";

const WIRELESS: &str = "org.freedesktop.NetworkManager.Device.Wireless";
const ACCESS_POINT: &str = "org.freedesktop.NetworkManager.AccessPoint";
const SETTINGS: &str = "org.freedesktop.NetworkManager.Settings";

const DEVICE_WIFI: u32 = 2;
const DEVICE_ETHERNET: u32 = 1;

fn ssid_text(raw: &[u8]) -> String {
    match std::str::from_utf8(raw) {
        Ok(s) => s.to_string(),
        Err(_) => raw.iter().map(|&b| b as char).collect(),
    }
}

fn get_path(p: &Props, k: &str) -> String {
    p.get(k)
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| OwnedObjectPath::try_from(v).ok())
        .map(|p| p.to_string())
        .unwrap_or_else(|| "/".into())
}

pub(super) fn chip_without_driver(devices: &std::path::Path) -> bool {
    std::fs::read_dir(devices).into_iter().flatten().flatten().any(|d| {
        std::fs::read_to_string(d.path().join("class")).is_ok_and(|c| c.starts_with("0x0280"))
            && !d.path().join("driver").exists()
    })
}
