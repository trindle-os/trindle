//! The battery and brightness panels: charge, the power profile and the screen's light.

use std::time::Duration;

use zbus::zvariant::OwnedObjectPath;

use crate::common::run::{capture, spawn};
use crate::ring::ctx::Msg;
use crate::toolkit::t;
use crate::toolkit::bus::{get_f64, get_i64, get_u32, Props};
use crate::toolkit::widget::{button, label, Length, Node, row};

use super::{big_line, Body, m, Quick, ROW_GAP, Well};

const UPOWER: &str = "org.freedesktop.UPower";

const BATTERY_EVERY: Duration = Duration::from_secs(10);

const BRIGHT_REST: Duration = Duration::from_millis(80);

pub(super) const TOUCH_HOLD: Duration = Duration::from_secs(3);

fn profiles() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("power-saver", "power-profile-power-saver-symbolic", t!("ring-quick-profile-saver")),
        ("balanced", "power-profile-balanced-symbolic", t!("ring-quick-profile-balanced")),
        ("performance", "power-profile-performance-symbolic", t!("ring-quick-profile-performance")),
    ]
}

pub(super) fn battery_view(big: &str, state: &str, health: &str, mode: Option<&str>) -> Vec<Node<Msg>> {
    let mut out = vec![big_line(big.to_string())];
    if !state.is_empty() {
        out.push(label(state.to_string()));
    }
    if !health.is_empty() {
        out.push(label(health.to_string()));
    }
    if let Some(mode) = mode {
        out.push(row(profiles().iter().map(|&(key, icon_name, tip)| {
            button(icon_name, tip).on(mode == key).on_press(m(move |q| q.set_profile(key))).well()
        }).collect()).gap(ROW_GAP).width(Length::Shrink));
    }
    out
}

fn battery_words(p: &Props) -> (String, String, String) {
    let pct = get_f64(p, "Percentage").round() as i32;
    let (to_empty, to_full) = (get_i64(p, "TimeToEmpty"), get_i64(p, "TimeToFull"));
    let rate = get_f64(p, "EnergyRate");
    let state = match crate::common::charge::true_state(get_u32(p, "State")) {
        1 if to_full > 0 => t!("ring-quick-charging-full", time = hours(to_full)),
        1 => t!("ring-quick-charging").into(),
        2 | 6 if to_empty > 0 => t!("ring-quick-on-battery-left", time = hours(to_empty), rate = rate.round()),
        2 | 6 => t!("ring-quick-on-battery").into(),
        4 => t!("ring-quick-full").into(),
        5 => t!("ring-quick-plugged").into(),
        _ => String::new(),
    };
    let capacity = get_f64(p, "Capacity");
    let cycles = whole_number(p, "ChargeCycles");
    let mut words = Vec::new();
    if capacity > 0.0 {
        let how = if capacity >= 80.0 { "good" } else if capacity >= 60.0 { "worn" } else { "old" };
        words.push(t!("ring-quick-health", how = how, capacity = capacity.round()));
    }
    if cycles > 0 {
        words.push(t!("ring-quick-cycles", cycles = cycles));
    }
    (format!("{pct}%"), state, words.join(" "))
}

fn whole_number(p: &Props, k: &str) -> i64 {
    p.get(k).and_then(|v| i32::try_from(v).ok().map(i64::from)).unwrap_or_else(|| get_i64(p, k))
}

pub(super) fn hours(secs: i64) -> String {
    let m = (secs + 30) / 60;
    match (m / 60, m % 60) {
        (0, m) => t!("ring-quick-minutes", m = m),
        (h, 0) => t!("ring-quick-hours", h = h),
        (h, m) => t!("ring-quick-hours-minutes", h = h, m = m),
    }
}

impl Quick {
    pub(super) fn battery(&mut self) {
        self.body = Body::Battery { big: String::new(), state: String::new(), health: String::new(), mode: None };
        self.battery_read();
        self.repeat(BATTERY_EVERY, |q| q.battery_read());
        let b = self.back();
        capture(&["powerprofilesctl", "get"], None, move |ok, out, _| {
            if ok {
                let now = out.trim().to_string();
                b.then(move |q| q.set_mode(now));
            }
        });
    }

    fn battery_read(&mut self) {
        let Some(bus) = self.ctx().system.clone() else { return };
        let (b, lp) = (self.back(), self.ctx().lp.clone());
        bus.clone().call(UPOWER, "/org/freedesktop/UPower", UPOWER).method("EnumerateDevices", &(), false, None, &self.ctx().lp.clone(),
            move |r| {
                let found = r.ok().and_then(|m| m.body().deserialize::<(Vec<OwnedObjectPath>,)>().ok())
                    .and_then(|(paths,)| paths.into_iter().map(|p| p.as_str().to_string()).find(|p| p.contains("battery_")));
                let Some(path) = found else {
                    b.then(|q| q.set_battery(String::new(), t!("ring-quick-battery-none").into(), String::new()));
                    return;
                };
                bus.call(UPOWER, &path, "org.freedesktop.UPower.Device").get_all(&lp, move |r| {
                    let Ok(p) = r else { return };
                    let (big, state, health) = battery_words(&p);
                    b.then(move |q| q.set_battery(big, state, health));
                });
            });
    }

    fn set_battery(&mut self, b: String, s: String, h: String) {
        if let Body::Battery { big, state, health, .. } = &mut self.body {
            (*big, *state, *health) = (b, s, h);
        }
    }

    fn set_mode(&mut self, now: String) {
        if let Body::Battery { mode, .. } = &mut self.body {
            *mode = Some(now);
        }
    }

    fn set_profile(&mut self, key: &'static str) {
        let b = self.back();
        capture(&["powerprofilesctl", "set", key], None, move |ok, _, _| {
            if ok {
                b.then(move |q| q.set_mode(key.to_string()));
            }
        });
    }

    pub(super) fn brightness(&mut self) {
        match crate::session::brightness::backlight() {
            Some((_, now, max)) if max > 0 => {
                let v = (now as f64 * 100.0 / max as f64).round() as f32;
                self.body = Body::Bright(v, format!("{}%", v as i32));
            }
            _ => self.say(t!("ring-quick-brightness-fixed")),
        }
    }

    pub(super) fn set_brightness(&mut self, v: f32) {
        let v = v.round();
        if let Body::Bright(value, pct) = &mut self.body {
            *value = v;
            *pct = format!("{}%", v as i32);
        }
        self.bright_gen = self.bright_gen.wrapping_add(1);
        let (b, at) = (self.back(), self.bright_gen);
        self.ctx().lp.after(BRIGHT_REST, move || b.then(move |q| {
            if q.bright_gen == at {
                spawn(&["trindle-brightness", &(v as i32).to_string()]);
            }
        }));
    }
}
